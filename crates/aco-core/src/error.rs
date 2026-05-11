//! Error types for `aco-core`.

use thiserror::Error;

/// `aco-core` error type.
#[derive(Debug, Error)]
pub enum Error {
    /// Malformed [`crate::Id`] hex.
    #[error("bad id: {0}")]
    BadId(String),
    /// Validation failure on a primitive's invariants.
    #[error("invalid primitive: {0}")]
    Invalid(String),
    /// Serialization round-trip failure.
    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),
}

/// Convenience alias.
pub type Result<T> = std::result::Result<T, Error>;
