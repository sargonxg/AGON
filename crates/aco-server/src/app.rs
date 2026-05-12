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
        .route("/api/sessions/{id}/report.md", get(get_session_report_md))
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

fn enrich_extraction(source_text: &str, mut x: serde_json::Value) -> serde_json::Value {
    add_evidence_audit(source_text, &mut x);
    add_deterministic_contradictions(&mut x);
    x
}

fn add_evidence_audit(source_text: &str, x: &mut serde_json::Value) {
    let mut rows = Vec::new();
    let primitive_sets = [
        ("actor", "actors", "label"),
        ("claim", "claims", "text"),
        ("event", "events", "label"),
        ("commitment", "commitments", "subject"),
        ("pattern", "patterns", "kind"),
        ("relationship", "relationships", "type"),
        ("power_dynamic", "power_dynamics", "basis"),
        ("escalation", "escalation_signals", "trigger"),
        ("resolution", "resolution_opportunities", "opening"),
    ];
    for (kind, key, label_key) in primitive_sets {
        if let Some(items) = x.get(key).and_then(|v| v.as_array()) {
            for item in items {
                let Some(quote) = item.get("evidence").and_then(|v| v.as_str()) else {
                    continue;
                };
                let label = item.get(label_key).and_then(|v| v.as_str()).unwrap_or(kind);
                rows.push(serde_json::json!({
                    "kind": kind,
                    "label": label,
                    "quote": quote,
                    "status": if quote_matches(source_text, quote) { "verified" } else { "unresolved" },
                }));
            }
        }
    }
    x["evidence_audit"] = serde_json::Value::Array(rows);
}

fn add_deterministic_contradictions(x: &mut serde_json::Value) {
    let Some(claims) = x.get("claims").and_then(|v| v.as_array()) else {
        return;
    };
    let mut additions = Vec::new();
    let mut known = std::collections::HashSet::new();
    if let Some(existing) = x.get("contradictions").and_then(|v| v.as_array()) {
        for c in existing {
            let a = c.get("claim_a").and_then(|v| v.as_str()).unwrap_or("");
            let b = c.get("claim_b").and_then(|v| v.as_str()).unwrap_or("");
            known.insert(pair_key(a, b));
        }
    }

    for (i, a) in claims.iter().enumerate() {
        for b in claims.iter().skip(i + 1) {
            let Some(a_id) = a.get("id").and_then(|v| v.as_str()) else {
                continue;
            };
            let Some(b_id) = b.get("id").and_then(|v| v.as_str()) else {
                continue;
            };
            if known.contains(&pair_key(a_id, b_id)) {
                continue;
            }
            let a_text = a.get("text").and_then(|v| v.as_str()).unwrap_or("");
            let b_text = b.get("text").and_then(|v| v.as_str()).unwrap_or("");
            let a_pol = a.get("polarity").and_then(|v| v.as_str()).unwrap_or("ambiguous");
            let b_pol = b.get("polarity").and_then(|v| v.as_str()).unwrap_or("ambiguous");
            let a_actor = a.get("actor_id").and_then(|v| v.as_str()).unwrap_or("");
            let b_actor = b.get("actor_id").and_then(|v| v.as_str()).unwrap_or("");
            if a_actor == b_actor || a_actor.is_empty() || b_actor.is_empty() {
                continue;
            }

            let denial_pair = (a_pol == "deny" && b_pol == "assert")
                || (b_pol == "deny" && a_pol == "assert")
                || (has_denial_language(a_text) != has_denial_language(b_text)
                    && token_overlap(a_text, b_text) >= 2);
            let timing_pair =
                token_overlap(a_text, b_text) >= 2 && has_temporal_conflict(a_text, b_text);
            let status_pair =
                token_overlap(a_text, b_text) >= 2 && has_status_conflict(a_text, b_text);

            if denial_pair || timing_pair || status_pair {
                known.insert(pair_key(a_id, b_id));
                let rationale = if timing_pair {
                    "Deterministic date/order language indicates incompatible timelines."
                } else if status_pair {
                    "Deterministic status language indicates incompatible account of the same obligation."
                } else {
                    "Deterministic denial/assertion pattern over overlapping terms."
                };
                additions.push(serde_json::json!({
                    "claim_a": a_id,
                    "claim_b": b_id,
                    "materiality": "material",
                    "source": "deterministic",
                    "confidence": 0.72,
                    "rationale": rationale,
                }));
            }
        }
    }

    if additions.is_empty() {
        return;
    }
    let contradictions = x
        .as_object_mut()
        .expect("extraction root is object")
        .entry("contradictions")
        .or_insert_with(|| serde_json::Value::Array(Vec::new()));
    if let Some(arr) = contradictions.as_array_mut() {
        arr.extend(additions);
    }
}

fn quote_matches(source_text: &str, quote: &str) -> bool {
    source_text.contains(quote.trim())
        || normalized_text(source_text).contains(&normalized_text(quote))
}

fn normalized_text(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_alphanumeric()).flat_map(|c| c.to_lowercase()).collect()
}

fn pair_key(a: &str, b: &str) -> String {
    if a < b {
        format!("{a}|{b}")
    } else {
        format!("{b}|{a}")
    }
}

fn has_denial_language(text: &str) -> bool {
    let t = text.to_ascii_lowercase();
    [" not ", " never ", " no ", " didn't ", " did not ", " wasn't ", " was not ", " denies "]
        .iter()
        .any(|needle| format!(" {t} ").contains(needle))
}

fn has_temporal_conflict(a: &str, b: &str) -> bool {
    let a = a.to_ascii_lowercase();
    let b = b.to_ascii_lowercase();
    let terms = [
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
        "before",
        "after",
    ];
    terms.iter().any(|term| a.contains(term)) && terms.iter().any(|term| b.contains(term)) && a != b
}

fn has_status_conflict(a: &str, b: &str) -> bool {
    let a = a.to_ascii_lowercase();
    let b = b.to_ascii_lowercase();
    let pairs = [
        ("cancelled", "postponed"),
        ("canceled", "postponed"),
        ("approved", "not approved"),
        ("signed off", "no approval"),
        ("completed", "not completed"),
        ("agreed", "never agreed"),
    ];
    pairs.iter().any(|(x, y)| (a.contains(x) && b.contains(y)) || (a.contains(y) && b.contains(x)))
}

fn token_overlap(a: &str, b: &str) -> usize {
    let stop = [
        "the", "and", "for", "that", "this", "with", "from", "was", "were", "had", "has", "have",
        "not", "never", "said", "says", "claim", "claims",
    ];
    let toks = |s: &str| -> std::collections::HashSet<String> {
        s.split(|c: char| !c.is_ascii_alphanumeric())
            .map(str::to_ascii_lowercase)
            .filter(|t| t.len() >= 4 && !stop.contains(&t.as_str()))
            .collect()
    };
    toks(a).intersection(&toks(b)).count()
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
    let extraction = enrich_extraction(&source_text, resp.value.clone());
    let x = &extraction;
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
            extraction: extraction.clone(),
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
        extraction,
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
        let extraction = enrich_extraction(&source_text, resp.value.clone());
        let x = &extraction;
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
                extraction: extraction.clone(),
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

        let fmatrix = friction_matrix(&extraction);
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
                "extraction": extraction,
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

async fn get_session_report_md(
    State(s): State<Arc<AppState>>,
    AxPath(id): AxPath<Uuid>,
) -> Result<Response, (StatusCode, String)> {
    let Some(store) = &s.store else {
        return Err((StatusCode::SERVICE_UNAVAILABLE, "db not configured".into()));
    };
    let sess = store
        .get_session(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "no such session".into()))?;
    let markdown = render_markdown_report(&sess);
    Ok(([(header::CONTENT_TYPE, "text/markdown; charset=utf-8")], markdown).into_response())
}

fn render_markdown_report(sess: &Session) -> String {
    let x = &sess.extraction;
    let mut out = String::new();
    out.push_str("# AGON Conflict Intelligence Report\n\n");
    out.push_str(&format!("- Session: `{}`\n", sess.id));
    out.push_str(&format!("- Created: `{}`\n", sess.created_at));
    out.push_str(&format!("- Model: `{}`\n", sess.model));
    out.push_str(&format!("- Friction score: `{}`\n", sess.friction_score));
    out.push_str(&format!(
        "- Primitives: `{}` actors, `{}` claims, `{}` events, `{}` commitments, `{}` contradictions\n\n",
        count_array(x, "actors"),
        count_array(x, "claims"),
        count_array(x, "events"),
        count_array(x, "commitments"),
        count_array(x, "contradictions"),
    ));

    out.push_str("## Summary\n\n");
    out.push_str(x.get("summary").and_then(|v| v.as_str()).unwrap_or("(no summary)"));
    out.push_str("\n\n");

    out.push_str("## Actors\n\n");
    for actor in array(x, "actors").into_iter().flatten() {
        let label = str_field(actor, "label").unwrap_or("unknown");
        let kind = str_field(actor, "kind").unwrap_or("unknown");
        out.push_str(&format!("- **{}** `{}`", label, kind));
        if let Some(aliases) = actor.get("aliases").and_then(|v| v.as_array()) {
            let aliases = aliases.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>();
            if !aliases.is_empty() {
                out.push_str(&format!(" aliases: {}", aliases.join(", ")));
            }
        }
        out.push('\n');
    }
    out.push('\n');

    out.push_str("## Contradictions\n\n");
    let claims = claim_lookup(x);
    if let Some(contradictions) = array(x, "contradictions") {
        for c in contradictions {
            let a = str_field(c, "claim_a").unwrap_or("");
            let b = str_field(c, "claim_b").unwrap_or("");
            out.push_str(&format!(
                "- **{}** `{}` confidence `{}`\n",
                str_field(c, "materiality").unwrap_or("unknown"),
                str_field(c, "source").unwrap_or("model_suggested"),
                c.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.5)
            ));
            out.push_str(&format!("  - A: {}\n", claims.get(a).map(String::as_str).unwrap_or(a)));
            out.push_str(&format!("  - B: {}\n", claims.get(b).map(String::as_str).unwrap_or(b)));
            if let Some(rationale) = str_field(c, "rationale") {
                out.push_str(&format!("  - Rationale: {}\n", rationale));
            }
        }
    } else {
        out.push_str("No contradictions extracted.\n");
    }
    out.push('\n');

    out.push_str("## Evidence Ledger\n\n");
    if let Some(rows) = array(x, "evidence_audit") {
        for row in rows {
            out.push_str(&format!(
                "- `{}` `{}` {}\n  > {}\n",
                str_field(row, "status").unwrap_or("unknown"),
                str_field(row, "kind").unwrap_or("primitive"),
                str_field(row, "label").unwrap_or(""),
                str_field(row, "quote").unwrap_or("")
            ));
        }
    } else {
        out.push_str("No evidence audit metadata available.\n");
    }
    out.push('\n');

    out.push_str("## Source Text\n\n```text\n");
    out.push_str(&sess.source_text);
    out.push_str("\n```\n");
    out
}

fn count_array(x: &serde_json::Value, key: &str) -> usize {
    x.get(key).and_then(|v| v.as_array()).map(Vec::len).unwrap_or(0)
}

fn claim_lookup(x: &serde_json::Value) -> std::collections::HashMap<String, String> {
    array(x, "claims")
        .into_iter()
        .flatten()
        .filter_map(|claim| {
            let id = str_field(claim, "id")?;
            let text = str_field(claim, "text").unwrap_or(id);
            Some((id.to_string(), text.to_string()))
        })
        .collect()
}

fn array<'a>(value: &'a serde_json::Value, key: &str) -> Option<&'a Vec<serde_json::Value>> {
    value.get(key).and_then(serde_json::Value::as_array)
}

fn str_field<'a>(value: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(serde_json::Value::as_str).filter(|s| !s.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::enrich_extraction;

    #[test]
    fn enrichment_adds_evidence_audit() {
        let source = "Sam: Alex agreed to own the deck.";
        let x = serde_json::json!({
            "actors": [{"id":"sam","label":"Sam","evidence":"Sam"}],
            "claims": [{"id":"c1","actor_id":"sam","text":"Alex agreed to own the deck","polarity":"assert","evidence":"Alex agreed to own the deck"}],
            "events": [],
            "summary": "test",
            "friction_score": 10
        });
        let enriched = enrich_extraction(source, x);
        let audit = enriched.get("evidence_audit").unwrap().as_array().unwrap();
        assert!(audit
            .iter()
            .any(|row| row.get("status").and_then(|v| v.as_str()) == Some("verified")));
    }

    #[test]
    fn enrichment_adds_deterministic_denial_contradiction() {
        let x = serde_json::json!({
            "actors": [{"id":"sam","label":"Sam"},{"id":"alex","label":"Alex"}],
            "claims": [
                {"id":"c1","actor_id":"sam","text":"Alex agreed to own the deck","polarity":"assert","evidence":"Alex agreed to own the deck"},
                {"id":"c2","actor_id":"alex","text":"I never agreed to own the deck","polarity":"deny","evidence":"never agreed to own the deck"}
            ],
            "events": [],
            "contradictions": [],
            "summary": "test",
            "friction_score": 60
        });
        let enriched = enrich_extraction("x", x);
        let contradictions = enriched.get("contradictions").unwrap().as_array().unwrap();
        assert_eq!(contradictions.len(), 1);
        assert_eq!(contradictions[0].get("source").and_then(|v| v.as_str()), Some("deterministic"));
    }
}
