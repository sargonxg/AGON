//! `aco-fuse` — canonicalization layer.
#![forbid(unsafe_code)]

use tracing::trace;

pub fn init() {
    trace!(crate_name = "aco-fuse", "loaded");
}
