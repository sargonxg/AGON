//! HTTP routes + state.

use crate::prompts::{PERCEIVE_SCHEMA, PERCEIVE_SYSTEM};
use aco_llm::{ExtractRequest, LlmBackend, MockLlmBackend, VertexAiBackend};
use aco_storage::{Session, Store};
use axum::{
    extract::{Path as AxPath, State},
    http::{header, StatusCode},
    response::sse::{Event, KeepAlive, Sse},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use futures::stream::Stream;
use std::convert::Infallible;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use chrono::Utc;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

/// Application-wide shared state.
pub struct AppState {
    pub backend: Arc<dyn LlmBackend>,
    pub store: Option<Store>,
    pub project_id: String,
    pub region: String,
    pub backend_name: String,
}

impl AppState {
    pub async fn from_env() -> Self {
        let project_id = std::env::var("AGON_GCP_PROJECT_ID").unwrap_or_else(|_| "unset".into());
        let region = std::env::var("AGON_GCP_REGION").unwrap_or_else(|_| "us-central1".into());
        let backend_kind = std::env::var("AGON_BACKEND").unwrap_or_else(|_| "vertex".into());

        let (backend, name): (Arc<dyn LlmBackend>, String) = match backend_kind.as_str() {
            "mock" => (Arc::new(MockLlmBackend::new()), "mock".into()),
            _ => {
                let v = VertexAiBackend::new(project_id.clone(), region.clone());
                (Arc::new(v), format!("vertex:{}", region))
            }
        };

        // Optional Cloud SQL — best-effort; server still runs without DB
        let store = match (
            std::env::var("AGON_DB_HOST"),
            std::env::var("AGON_DB_USER"),
            std::env::var("AGON_DB_PASSWORD"),
            std::env::var("AGON_DB_NAME"),
        ) {
            (Ok(h), Ok(u), Ok(p), Ok(d)) if !h.is_empty() => {
                let dsn = Store::dsn_from_parts(&h, &u, &p, &d);
                match Store::connect(&dsn).await {
                    Ok(s) => {
                        if let Err(e) = s.migrate().await {
                            tracing::warn!("migrate failed: {e}");
                        }
                        tracing::info!("connected to Cloud SQL at {h}");
                        Some(s)
                    }
                    Err(e) => {
                        tracing::warn!("Cloud SQL connect failed: {e}; running stateless");
                        None
                    }
                }
            }
            _ => {
                tracing::info!("no AGON_DB_* env -> running stateless");
                None
            }
        };

        Self { backend, store, project_id, region, backend_name: name }
    }
}

pub fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/assets/{*file}", get(asset))
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/info", get(info))
        .route("/api/schema", get(schema_route))
        .route("/api/perceive", post(perceive))
        .route("/api/perceive/stream", post(perceive_stream))
        .route("/api/sessions", get(list_sessions))
        .route("/api/sessions/{id}", get(get_session_by_id))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn index() -> impl IntoResponse {
    match Assets::get("index.html") {
        Some(f) => Html(String::from_utf8_lossy(&f.data).to_string()).into_response(),
        None => (StatusCode::NOT_FOUND, "missing").into_response(),
    }
}

async fn asset(AxPath(p): AxPath<String>) -> Response {
    match Assets::get(&p) {
        Some(f) => ([(header::CONTENT_TYPE, mime_for(&p))], f.data.into_owned()).into_response(),
        None => (StatusCode::NOT_FOUND, "missing").into_response(),
    }
}

fn mime_for(p: &str) -> &'static str {
    match p.rsplit_once('.').map(|(_, e)| e) {
        Some("html") => "text/html; charset=utf-8",
        Some("js")   => "application/javascript; charset=utf-8",
        Some("css")  => "text/css; charset=utf-8",
        Some("svg")  => "image/svg+xml",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

async fn healthz() -> &'static str { "ok" }

async fn readyz(State(s): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "ok": true,
        "backend": s.backend_name,
        "db": s.store.is_some(),
        "project": s.project_id,
        "region": s.region,
    }))
}

async fn schema_route() -> Json<serde_json::Value> {
    Json(serde_json::from_str(PERCEIVE_SCHEMA).unwrap_or_default())
}

async fn info(State(s): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "AGON",
        "version": env!("CARGO_PKG_VERSION"),
        "backend": s.backend_name,
        "db": s.store.is_some(),
        "project": s.project_id,
        "region": s.region,
    }))
}

#[derive(Deserialize)]
struct PerceiveBody {
    text: String,
    #[serde(default)]
    model: Option<String>,
}

#[derive(Serialize)]
struct PerceiveResponse {
    session_id: Option<Uuid>,
    elapsed_ms: u128,
    model: String,
    input_tokens: u32,
    output_tokens: u32,
    persisted: bool,
    extraction: serde_json::Value,
}

async fn perceive(
    State(s): State<Arc<AppState>>,
    Json(body): Json<PerceiveBody>,
) -> Result<Json<PerceiveResponse>, (StatusCode, String)> {
    let started = Instant::now();
    let schema: serde_json::Value = serde_json::from_str(PERCEIVE_SCHEMA)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("schema parse: {e}")))?;

    let source_text = body.text.clone();
    let req = ExtractRequest {
        system: PERCEIVE_SYSTEM.into(),
        user: body.text,
        schema: Some(schema),
        model: body.model,
        temperature: Some(0.0),
        max_output_tokens: Some(16384),
    };

    let resp = s.backend.extract_json(req).await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("{e}")))?;

    let elapsed = started.elapsed();
    let x = &resp.value;
    let count = |k: &str| x.get(k).and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0) as i32;
    let friction = x.get("friction_score").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let summary = x.get("summary").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let mut session_id: Option<Uuid> = None;
    let mut persisted = false;
    if let Some(store) = &s.store {
        let sess = Session {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            model: resp.model.clone(),
            input_tokens: resp.input_tokens as i32,
            output_tokens: resp.output_tokens as i32,
            elapsed_ms: elapsed.as_millis() as i64,
            source_text,
            friction_score: friction,
            n_actors: count("actors"),
            n_claims: count("claims"),
            n_events: count("events"),
            n_patterns: count("patterns"),
            n_contradictions: count("contradictions"),
            extraction: resp.value.clone(),
            summary,
        };
        match store.insert_session(&sess).await {
            Ok(()) => { session_id = Some(sess.id); persisted = true; }
            Err(e) => tracing::warn!("insert_session: {e}"),
        }
    }

    Ok(Json(PerceiveResponse {
        session_id,
        elapsed_ms: elapsed.as_millis(),
        model: resp.model,
        input_tokens: resp.input_tokens,
        output_tokens: resp.output_tokens,
        persisted,
        extraction: resp.value,
    }))
}

/// Server-Sent Events stream of pipeline stages with live per-stage timing.
/// Stages: validating -> auth -> calling_vertex -> vertex_done -> parsing -> persisting -> result.
async fn perceive_stream(
    State(s): State<Arc<AppState>>,
    Json(body): Json<PerceiveBody>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<Event, Infallible>>();

    let emit = |tx: &mpsc::UnboundedSender<Result<Event, Infallible>>, name: &str, data: serde_json::Value| {
        let _ = tx.send(Ok(Event::default().event(name).json_data(data).unwrap_or_else(|_| Event::default().event(name))));
    };

    tokio::spawn(async move {
        let overall = Instant::now();
        let mut t_stage = Instant::now();

        let source_text = body.text.clone();
        let n_chars = source_text.chars().count();
        let n_lines = source_text.lines().count();

        emit(&tx, "stage", serde_json::json!({
            "stage": "validating",
            "msg": "checking input",
            "chars": n_chars,
            "lines": n_lines,
            "elapsed_ms": t_stage.elapsed().as_millis(),
        }));

        if source_text.trim().is_empty() {
            emit(&tx, "error", serde_json::json!({"error": "empty input"}));
            return;
        }

        let schema: serde_json::Value = match serde_json::from_str(PERCEIVE_SCHEMA) {
            Ok(v) => v,
            Err(e) => {
                emit(&tx, "error", serde_json::json!({"error": format!("schema parse: {e}")}));
                return;
            }
        };

        t_stage = Instant::now();
        emit(&tx, "stage", serde_json::json!({
            "stage": "auth",
            "msg": "fetching service-account token",
            "backend": s.backend_name,
        }));

        // Build request
        let req = ExtractRequest {
            system: PERCEIVE_SYSTEM.into(),
            user: source_text.clone(),
            schema: Some(schema),
            model: body.model.clone(),
            temperature: Some(0.0),
            max_output_tokens: Some(16384),
        };
        let auth_ms = t_stage.elapsed().as_millis();

        t_stage = Instant::now();
        emit(&tx, "stage", serde_json::json!({
            "stage": "calling_vertex",
            "msg": "Vertex AI Gemini 2.5 Flash · schema-constrained generation",
            "auth_ms": auth_ms,
        }));

        let resp = match s.backend.extract_json(req).await {
            Ok(r) => r,
            Err(e) => {
                emit(&tx, "error", serde_json::json!({"error": e.to_string()}));
                return;
            }
        };
        let vertex_ms = t_stage.elapsed().as_millis();

        emit(&tx, "stage", serde_json::json!({
            "stage": "vertex_done",
            "msg": "model response received",
            "model": resp.model,
            "input_tokens": resp.input_tokens,
            "output_tokens": resp.output_tokens,
            "elapsed_ms": vertex_ms,
            "tokens_per_sec": if vertex_ms > 0 { (resp.output_tokens as f64) / (vertex_ms as f64 / 1000.0) } else { 0.0 },
        }));

        t_stage = Instant::now();
        let x = &resp.value;
        let count = |k: &str| x.get(k).and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0) as i32;
        let n_actors = count("actors");
        let n_claims = count("claims");
        let n_events = count("events");
        let n_patterns = count("patterns");
        let n_contradictions = count("contradictions");
        let friction = x.get("friction_score").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let summary = x.get("summary").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let parsing_ms = t_stage.elapsed().as_millis();

        emit(&tx, "stage", serde_json::json!({
            "stage": "parsing",
            "msg": "primitive counting & friction extracted",
            "elapsed_ms": parsing_ms,
            "n_actors": n_actors,
            "n_claims": n_claims,
            "n_events": n_events,
            "n_patterns": n_patterns,
            "n_contradictions": n_contradictions,
            "friction_score": friction,
        }));

        let mut session_id: Option<Uuid> = None;
        let mut persisted = false;
        if let Some(store) = &s.store {
            t_stage = Instant::now();
            emit(&tx, "stage", serde_json::json!({
                "stage": "persisting",
                "msg": "writing session to Cloud SQL",
            }));
            let sess = Session {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                model: resp.model.clone(),
                input_tokens: resp.input_tokens as i32,
                output_tokens: resp.output_tokens as i32,
                elapsed_ms: overall.elapsed().as_millis() as i64,
                source_text,
                friction_score: friction,
                n_actors,
                n_claims,
                n_events,
                n_patterns,
                n_contradictions,
                extraction: resp.value.clone(),
                summary,
            };
            match store.insert_session(&sess).await {
                Ok(()) => { session_id = Some(sess.id); persisted = true; }
                Err(e) => emit(&tx, "warn", serde_json::json!({"warn": format!("insert: {e}")})),
            }
            emit(&tx, "stage", serde_json::json!({
                "stage": "persisted",
                "msg": if persisted { "saved to Cloud SQL" } else { "skipped" },
                "session_id": session_id,
                "elapsed_ms": t_stage.elapsed().as_millis(),
            }));
        }

        emit(&tx, "result", serde_json::json!({
            "session_id": session_id,
            "elapsed_ms": overall.elapsed().as_millis(),
            "model": resp.model,
            "input_tokens": resp.input_tokens,
            "output_tokens": resp.output_tokens,
            "persisted": persisted,
            "extraction": resp.value,
        }));
    });

    Sse::new(UnboundedReceiverStream::new(rx)).keep_alive(KeepAlive::default())
}

async fn list_sessions(State(s): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let Some(store) = &s.store else {
        return Ok(Json(serde_json::json!({"sessions": [], "db": false})));
    };
    let v = store.recent_sessions(50).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!({"sessions": v, "db": true})))
}

async fn get_session_by_id(
    State(s): State<Arc<AppState>>,
    AxPath(id): AxPath<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let Some(store) = &s.store else {
        return Err((StatusCode::SERVICE_UNAVAILABLE, "db not configured".into()));
    };
    match store.get_session(id).await {
        Ok(Some(sess)) => Ok(Json(serde_json::to_value(sess).unwrap())),
        Ok(None) => Err((StatusCode::NOT_FOUND, "no such session".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
