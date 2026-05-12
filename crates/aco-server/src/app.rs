//! HTTP routes + state.

use crate::pretransform::{self, PreCanonical};
use crate::prompts::{PERCEIVE_SCHEMA, PERCEIVE_SYSTEM};
use aco_llm::{ExtractRequest, LlmBackend, MockLlmBackend, VertexAiBackend};
use aco_storage::{Session, Store};
use axum::{
    body::Body,
    extract::{Path as AxPath, State},
    http::{header, Request, StatusCode},
    middleware::{self, Next},
    response::sse::{Event, KeepAlive, Sse},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use futures::stream::Stream;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
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
    let public = Router::new().route("/healthz", get(healthz)).route("/readyz", get(readyz));

    let protected = Router::new()
        .route("/", get(index))
        .route("/assets/{*file}", get(asset))
        .route("/api/info", get(info))
        .route("/api/schema", get(schema_route))
        .route("/api/perceive", post(perceive))
        .route("/api/perceive/stream", post(perceive_stream))
        .route("/api/sessions", get(list_sessions))
        .route("/api/sessions/{id}", get(get_session_by_id))
        .layer(middleware::from_fn(require_demo_auth));

    public
        .merge(protected)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn require_demo_auth(req: Request<Body>, next: Next) -> Result<Response, Response> {
    const EXPECTED: &str = "Basic QUdPTjpBR09O";
    let allowed = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(|v| v == EXPECTED)
        .unwrap_or(false);

    if allowed {
        Ok(next.run(req).await)
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            [(header::WWW_AUTHENTICATE, "Basic realm=\"AGON\"")],
            "AGON access required",
        )
            .into_response())
    }
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
        Some("js") => "application/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

async fn healthz() -> &'static str {
    "ok"
}

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

fn resolve_model(m: Option<&str>) -> String {
    match m.unwrap_or("flash") {
        "flash-lite" | "lite" => "gemini-2.5-flash-lite".into(),
        "pro" | "smart" => "gemini-2.5-pro".into(),
        "flash" | "" => "gemini-2.5-flash".into(),
        other => other.into(),
    }
}

/// Compute an actor-vs-actor friction matrix from claims + contradictions + patterns.
fn friction_matrix(x: &serde_json::Value) -> serde_json::Value {
    use std::collections::HashMap;
    let actors: Vec<String> = x
        .get("actors")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|o| o.get("id").and_then(|i| i.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let actor_label: HashMap<String, String> = x
        .get("actors")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|o| {
                    let id = o.get("id").and_then(|i| i.as_str())?;
                    let lab = o.get("label").and_then(|i| i.as_str()).unwrap_or(id);
                    Some((id.to_string(), lab.to_string()))
                })
                .collect()
        })
        .unwrap_or_default();
    let claim_owner: HashMap<String, String> = x
        .get("claims")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|c| {
                    let id = c.get("id").and_then(|i| i.as_str())?;
                    let actor = c.get("actor_id").and_then(|i| i.as_str())?;
                    Some((id.to_string(), actor.to_string()))
                })
                .collect()
        })
        .unwrap_or_default();

    // matrix[a][b] = float weight plus explainable factors for the UI.
    let mut m: HashMap<(String, String), f64> = HashMap::new();
    let mut reasons: HashMap<(String, String), Vec<String>> = HashMap::new();
    let mut bump = |a: &str, b: &str, w: f64, reason: String| {
        if a == b {
            return;
        }
        let (lo, hi) =
            if a < b { (a.to_string(), b.to_string()) } else { (b.to_string(), a.to_string()) };
        let key = (lo, hi);
        *m.entry(key.clone()).or_insert(0.0) += w;
        reasons.entry(key).or_default().push(reason);
    };

    // Contradictions: heavy
    if let Some(arr) = x.get("contradictions").and_then(|v| v.as_array()) {
        for c in arr {
            let a_claim = c.get("claim_a").and_then(|v| v.as_str()).unwrap_or("");
            let b_claim = c.get("claim_b").and_then(|v| v.as_str()).unwrap_or("");
            let w = if c.get("materiality").and_then(|v| v.as_str()) == Some("material") {
                2.0
            } else {
                0.6
            };
            if let (Some(a), Some(b)) = (claim_owner.get(a_claim), claim_owner.get(b_claim)) {
                bump(a, b, w, "contradictory claims".into());
            }
        }
    }
    // Negative-polarity claims pointed at others: light
    if let Some(arr) = x.get("claims").and_then(|v| v.as_array()) {
        for c in arr {
            if c.get("polarity").and_then(|v| v.as_str()) == Some("deny") {
                if let Some(a) = c.get("actor_id").and_then(|v| v.as_str()) {
                    // Spread denial over all other actors as small weight.
                    for other in &actors {
                        if other != a {
                            bump(a, other, 0.4, "denial pressure".into());
                        }
                    }
                }
            }
        }
    }
    // Relationship edges are direct actor-to-actor pressure signals.
    if let Some(arr) = x.get("relationships").and_then(|v| v.as_array()) {
        for r in arr {
            let Some(a) = r.get("from_actor").and_then(|v| v.as_str()) else {
                continue;
            };
            let Some(b) = r.get("to_actor").and_then(|v| v.as_str()) else {
                continue;
            };
            let kind = r.get("type").and_then(|v| v.as_str()).unwrap_or("relationship");
            let base = r.get("weight").and_then(|v| v.as_f64()).unwrap_or(1.0).clamp(0.2, 3.0);
            let w = match kind {
                "accuses" | "retaliation_risk" => 1.6 * base,
                "pressures" | "denies" | "bypasses" => 1.0 * base,
                "supervises" | "commits_to" => 0.5 * base,
                "supports" => -0.4 * base,
                _ => 0.4 * base,
            };
            if w > 0.0 {
                bump(a, b, w, format!("relationship: {kind}"));
            }
        }
    }
    // Broken or contested commitments create pairwise friction when a recipient is known.
    if let Some(arr) = x.get("commitments").and_then(|v| v.as_array()) {
        for c in arr {
            let Some(a) = c.get("by_actor").and_then(|v| v.as_str()) else {
                continue;
            };
            let Some(b) = c.get("to_actor").and_then(|v| v.as_str()) else {
                continue;
            };
            let status = c.get("status").and_then(|v| v.as_str()).unwrap_or("proposed");
            let w = match status {
                "broken" => 1.4,
                "contested" => 1.0,
                _ => 0.2,
            };
            if w > 0.2 {
                bump(a, b, w, format!("commitment {status}"));
            }
        }
    }
    // Escalation and power dynamics spread risk from the actor to their counterparties.
    if let Some(arr) = x.get("escalation_signals").and_then(|v| v.as_array()) {
        for e in arr {
            let Some(a) = e.get("actor_id").and_then(|v| v.as_str()) else {
                continue;
            };
            let intensity = e.get("intensity").and_then(|v| v.as_i64()).unwrap_or(1).clamp(1, 5);
            for other in &actors {
                if other != a {
                    bump(a, other, intensity as f64 * 0.25, "escalation signal".into());
                }
            }
        }
    }
    if let Some(arr) = x.get("power_dynamics").and_then(|v| v.as_array()) {
        for p in arr {
            let Some(a) = p.get("dominant_actor").and_then(|v| v.as_str()) else {
                continue;
            };
            let Some(b) = p.get("subordinate_actor").and_then(|v| v.as_str()) else {
                continue;
            };
            bump(a, b, 0.8, "power dynamic".into());
        }
    }
    // Patterns DARVO/contempt/criticism by actor: light spread.
    if let Some(arr) = x.get("patterns").and_then(|v| v.as_array()) {
        for p in arr {
            let kind = p.get("kind").and_then(|v| v.as_str()).unwrap_or("");
            let w = match kind {
                "DARVO" | "contempt" | "gaslighting" => 1.2,
                "criticism" | "defensiveness" | "stonewalling" => 0.6,
                _ => 0.2,
            };
            if let Some(a) = p.get("actor_id").and_then(|v| v.as_str()) {
                for other in &actors {
                    if other != a {
                        bump(a, other, w, format!("pattern: {kind}"));
                    }
                }
            }
        }
    }

    let pairs: Vec<serde_json::Value> = m
        .into_iter()
        .map(|((a, b), w)| {
            let key = (a.clone(), b.clone());
            serde_json::json!({
                "a": a,
                "b": b,
                "a_label": actor_label.get(&a).cloned().unwrap_or(a.clone()),
                "b_label": actor_label.get(&b).cloned().unwrap_or(b.clone()),
                "weight": (w * 100.0).round() / 100.0,
                "heat": ((w * 30.0).min(100.0)).round() as i64,
                "reasons": reasons.get(&key).cloned().unwrap_or_default(),
            })
        })
        .collect();

    serde_json::json!({ "actors": actors, "pairs": pairs })
}

#[derive(Serialize)]
struct PerceiveResponse {
    session_id: Option<Uuid>,
    elapsed_ms: u128,
    model: String,
    input_tokens: u32,
    output_tokens: u32,
    persisted: bool,
    pre_canonical: PreCanonical,
    friction_matrix: serde_json::Value,
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
    let pc = pretransform::transform(&source_text);
    let envelope = pretransform::render_envelope(&pc);
    let user_payload = if envelope.is_empty() {
        source_text.clone()
    } else {
        format!("{envelope}\n{source_text}")
    };

    let req = ExtractRequest {
        system: PERCEIVE_SYSTEM.into(),
        user: user_payload,
        schema: Some(schema),
        model: Some(resolve_model(body.model.as_deref())),
        temperature: Some(0.0),
        max_output_tokens: Some(16384),
    };

    let resp =
        s.backend.extract_json(req).await.map_err(|e| (StatusCode::BAD_GATEWAY, format!("{e}")))?;

    let elapsed = started.elapsed();
    let x = &resp.value;
    let fmatrix = friction_matrix(x);
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
        match store.insert_perception(&sess).await {
            Ok(_) => {
                session_id = Some(sess.id);
                persisted = true;
            }
            Err(e) => tracing::warn!("insert_perception: {e}"),
        }
    }

    Ok(Json(PerceiveResponse {
        session_id,
        elapsed_ms: elapsed.as_millis(),
        model: resp.model,
        input_tokens: resp.input_tokens,
        output_tokens: resp.output_tokens,
        persisted,
        pre_canonical: pc,
        friction_matrix: fmatrix,
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

    let emit = |tx: &mpsc::UnboundedSender<Result<Event, Infallible>>,
                name: &str,
                data: serde_json::Value| {
        let _ = tx.send(Ok(Event::default()
            .event(name)
            .json_data(data)
            .unwrap_or_else(|_| Event::default().event(name))));
    };

    tokio::spawn(async move {
        let overall = Instant::now();
        let mut t_stage = Instant::now();

        let source_text = body.text.clone();
        let n_chars = source_text.chars().count();
        let n_lines = source_text.lines().count();

        emit(
            &tx,
            "stage",
            serde_json::json!({
                "stage": "validating",
                "msg": "checking input",
                "chars": n_chars,
                "lines": n_lines,
                "elapsed_ms": t_stage.elapsed().as_millis(),
            }),
        );

        t_stage = Instant::now();
        let pc = pretransform::transform(&source_text);
        let envelope = pretransform::render_envelope(&pc);
        emit(
            &tx,
            "stage",
            serde_json::json!({
                "stage": "pre_canonical",
                "msg": "deterministic envelope built",
                "format": format!("{:?}", pc.format_hint),
                "turns": pc.n_turns,
                "speakers": pc.speakers,
                "elapsed_ms": t_stage.elapsed().as_millis(),
            }),
        );

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
        emit(
            &tx,
            "stage",
            serde_json::json!({
                "stage": "auth",
                "msg": "fetching service-account token",
                "backend": s.backend_name,
            }),
        );

        // Build request — prepend deterministic envelope.
        let user_payload = if envelope.is_empty() {
            source_text.clone()
        } else {
            format!("{envelope}\n{}", source_text)
        };
        let req = ExtractRequest {
            system: PERCEIVE_SYSTEM.into(),
            user: user_payload,
            schema: Some(schema),
            model: Some(resolve_model(body.model.as_deref())),
            temperature: Some(0.0),
            max_output_tokens: Some(16384),
        };
        let auth_ms = t_stage.elapsed().as_millis();

        t_stage = Instant::now();
        emit(
            &tx,
            "stage",
            serde_json::json!({
                "stage": "calling_vertex",
                "msg": "Vertex AI Gemini 2.5 Flash · schema-constrained generation",
                "auth_ms": auth_ms,
            }),
        );

        let resp = match s.backend.extract_json(req).await {
            Ok(r) => r,
            Err(e) => {
                emit(&tx, "error", serde_json::json!({"error": e.to_string()}));
                return;
            }
        };
        let vertex_ms = t_stage.elapsed().as_millis();

        emit(
            &tx,
            "stage",
            serde_json::json!({
                "stage": "vertex_done",
                "msg": "model response received",
                "model": resp.model,
                "input_tokens": resp.input_tokens,
                "output_tokens": resp.output_tokens,
                "elapsed_ms": vertex_ms,
                "tokens_per_sec": if vertex_ms > 0 { (resp.output_tokens as f64) / (vertex_ms as f64 / 1000.0) } else { 0.0 },
            }),
        );

        t_stage = Instant::now();
        let x = &resp.value;
        let count =
            |k: &str| x.get(k).and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0) as i32;
        let n_actors = count("actors");
        let n_claims = count("claims");
        let n_events = count("events");
        let n_patterns = count("patterns");
        let n_contradictions = count("contradictions");
        let friction = x.get("friction_score").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let summary = x.get("summary").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let parsing_ms = t_stage.elapsed().as_millis();

        emit(
            &tx,
            "stage",
            serde_json::json!({
                "stage": "parsing",
                "msg": "primitive counting & friction extracted",
                "elapsed_ms": parsing_ms,
                "n_actors": n_actors,
                "n_claims": n_claims,
                "n_events": n_events,
                "n_patterns": n_patterns,
                "n_contradictions": n_contradictions,
                "friction_score": friction,
            }),
        );

        let mut session_id: Option<Uuid> = None;
        let mut persisted = false;
        if let Some(store) = &s.store {
            t_stage = Instant::now();
            emit(
                &tx,
                "stage",
                serde_json::json!({
                    "stage": "persisting",
                    "msg": "writing session to Cloud SQL",
                }),
            );
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
            match store.insert_perception(&sess).await {
                Ok(_) => {
                    session_id = Some(sess.id);
                    persisted = true;
                }
                Err(e) => emit(&tx, "warn", serde_json::json!({"warn": format!("insert: {e}")})),
            }
            emit(
                &tx,
                "stage",
                serde_json::json!({
                    "stage": "persisted",
                    "msg": if persisted { "saved to Cloud SQL" } else { "skipped" },
                    "session_id": session_id,
                    "elapsed_ms": t_stage.elapsed().as_millis(),
                }),
            );
        }

        let fmatrix = friction_matrix(&resp.value);
        emit(
            &tx,
            "result",
            serde_json::json!({
                "session_id": session_id,
                "elapsed_ms": overall.elapsed().as_millis(),
                "model": resp.model,
                "input_tokens": resp.input_tokens,
                "output_tokens": resp.output_tokens,
                "persisted": persisted,
                "pre_canonical": pc,
                "friction_matrix": fmatrix,
                "extraction": resp.value,
            }),
        );
    });

    Sse::new(UnboundedReceiverStream::new(rx)).keep_alive(KeepAlive::default())
}

async fn list_sessions(
    State(s): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let Some(store) = &s.store else {
        return Ok(Json(serde_json::json!({"sessions": [], "db": false})));
    };
    let v = store
        .recent_sessions(50)
        .await
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
