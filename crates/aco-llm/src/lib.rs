//! `aco-llm` — typed LLM backends.
#![forbid(unsafe_code)]

use tracing::trace;

pub fn init() {
    trace!(crate_name = "aco-llm", "loaded");
}
