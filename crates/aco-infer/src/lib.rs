//! `aco-infer` — Datalog + SMT + LP reasoning.
#![forbid(unsafe_code)]

use tracing::trace;

pub fn init() {
    trace!(crate_name = "aco-infer", "loaded");
}
