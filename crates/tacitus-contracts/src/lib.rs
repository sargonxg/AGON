#![forbid(unsafe_code)]
//! tacitus-contracts — typed primitives shared across AGON, DIALECTICA, KAIROS.
//!
//! JSON Schema is the source of truth. Rust types here are hand-written until
//! `typify` codegen is wired in (PROMPT 01 follow-up). They are kept aligned
//! with the schemas under `schemas/` by `tests/test_schemas.rs`.

pub mod primitives;
pub mod schemas;

pub use primitives::*;

pub const SCHEMA_VERSION: &str = "0.1.0";
