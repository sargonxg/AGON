//! HTTP routes + state.

use crate::prompts::{PERCEIVE_SCHEMA, PERCEIVE_SYSTEM};
use aco_llm::{ExtractRequest, LlmBackend, MockLlmBackend, VertexAiBackend};
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

/// Application-wide shared state.
pub struct AppState {
    pub backend: Arc<dyn LlmBackend>,
    pub project_id: String,
    pub region: String,
    pub backend_name: String,
}

impl AppState {
    pub fn from_env() -> Self {
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

        Self { backend, project_id, region, backend_name: name }
    }
}

pub fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/assets/{*file}", get(asset))
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/info", get(info))
        .route("/api/perceive", post(perceive))
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

async fn asset(axum::extract::Path(p): axum::extract::Path<String>) -> Response {
    match Assets::get(&p) {
        Some(f) => {
            let mime = mime_for(&p);
            (
                [(header::CONTENT_TYPE, mime)],
                f.data.into_owned(),
            ).into_response()
        }
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
        "project": s.project_id,
        "region": s.region,
    }))
}

async fn info(State(s): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "AGON",
        "version": env!("CARGO_PKG_VERSION"),
        "backend": s.backend_name,
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
    elapsed_ms: u128,
    model: String,
    input_tokens: u32,
    output_tokens: u32,
    extraction: serde_json::Value,
}

async fn perceive(
    State(s): State<Arc<AppState>>,
    Json(body): Json<PerceiveBody>,
) -> Result<Json<PerceiveResponse>, (StatusCode, String)> {
    let started = Instant::now();
    let schema: serde_json::Value = serde_json::from_str(PERCEIVE_SCHEMA)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("schema parse: {e}")))?;

    let req = ExtractRequest {
        system: PERCEIVE_SYSTEM.into(),
        user: body.text,
        schema: Some(schema),
        model: body.model,
        temperature: Some(0.0),
        max_output_tokens: Some(4096),
    };

    let resp = s.backend.extract_json(req).await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("{e}")))?;

    Ok(Json(PerceiveResponse {
        elapsed_ms: started.elapsed().as_millis(),
        model: resp.model,
        input_tokens: resp.input_tokens,
        output_tokens: resp.output_tokens,
        extraction: resp.value,
    }))
}
