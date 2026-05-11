//! `agon-server` binary — Cloud Run entrypoint.

use aco_server::{build_app, AppState};
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=info".into()))
        .json()
        .init();

    let port: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
    let state = Arc::new(AppState::from_env().await);
    let app = build_app(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    tracing::info!("agon-server listening on {port}");
    axum::serve(listener, app).await?;
    Ok(())
}
