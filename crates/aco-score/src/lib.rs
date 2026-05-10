//! `aco-score` — calibrated typed scoring models.
#![forbid(unsafe_code)]

use tracing::trace;

pub fn init() {
    trace!(crate_name = "aco-score", "loaded");
}
