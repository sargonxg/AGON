//! `aco-learn` — learning loop.
#![forbid(unsafe_code)]

use tracing::trace;

pub fn init() {
    trace!(crate_name = "aco-learn", "loaded");
}
