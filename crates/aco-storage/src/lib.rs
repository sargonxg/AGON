//! `aco-storage` — Postgres source of truth + in-memory hot working set.
#![forbid(unsafe_code)]

use tracing::trace;

pub fn init() {
    trace!(crate_name = "aco-storage", "loaded");
}
