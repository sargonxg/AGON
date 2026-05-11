//! `aco-server` — Axum HTTP server + embedded dashboard for AGON.
#![forbid(unsafe_code)]

pub mod app;
pub mod prompts;

pub use app::{build_app, AppState};
