//! `aco-server` — Axum dashboard backend.
#![forbid(unsafe_code)]

use tracing::trace;

pub fn init() {
    trace!(crate_name = "aco-server", "loaded");
}
