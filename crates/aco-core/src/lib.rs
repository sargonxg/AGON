//! `aco-core` — pure types for the Agentic Conflict Ontology.
//!
//! Level 0 in the crate hierarchy. No I/O, no async, no tokio.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use tracing::trace;

/// Crate version for diagnostic emission.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Emit an init span; called by binaries that want to confirm the crate loaded.
pub fn init() {
    trace!(crate_name = "aco-core", version = VERSION, "loaded");
}
