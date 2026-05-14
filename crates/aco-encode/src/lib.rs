//! aco-encode — local neural encoders behind a typed Rust trait.
//!
//! Status (PROMPT 05 v0.1, 2026-05-14):
//! - `Encoder` trait + `EmbedResult` + `NliResult` types defined.
//! - `BgeM3Encoder` and `DebertaNliEncoder` stubs with download+cache plumbing.
//! - Real ONNX inference gated behind the `onnx` feature so the default
//!   workspace build does not require ONNX Runtime native libs.
//! - HuggingFace download uses the `HF_TOKEN` env var (Secret Manager).
//!
//! See `docs/HONEST_STATE.md` — neural inference is not yet running in production.
//! This crate is the path to flipping `local_encoders.status` from "scaffolded"
//! to "live".
#![forbid(unsafe_code)]

pub mod download;
pub mod traits;

// bge_m3 + deberta_nli are parked as `.rs.skel` files while the ort 2.x
// API is in flux (rc.10 vs rc.12 incompatibility). Next focused turn pins
// the version + ONNX Runtime native lib in the Dockerfile and re-enables
// these modules. See docs/HONEST_STATE.md.
#[cfg(all(feature = "onnx", any()))]
pub mod bge_m3;
#[cfg(all(feature = "onnx", any()))]
pub mod deberta_nli;

pub use traits::{EmbedResult, Encoder, EncodeError, NliLabel, NliResult};

// Parked until ort wiring is finalized — see comment above.
// pub use bge_m3::BgeM3Encoder;
// pub use deberta_nli::DebertaNliEncoder;

/// Build info — what's loaded at runtime.
#[must_use]
pub fn build_info() -> serde_json::Value {
    serde_json::json!({
        "crate": "aco-encode",
        "version": env!("CARGO_PKG_VERSION"),
        "onnx_feature": cfg!(feature = "onnx"),
        "models_planned": ["BGE-M3", "DeBERTa-v3-large-mnli", "fastcoref"],
    })
}
