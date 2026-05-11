//! `aco-llm` — typed LLM backends for AGON.
//!
//! Provides the [`LlmBackend`] trait, a [`MockLlmBackend`] for reproducible tests,
//! and [`VertexAiBackend`] which calls Google Cloud Vertex AI Gemini using the
//! Cloud Run service-account credential (ADC). Schema-constrained extraction
//! ([`extract_json`]) returns parsed JSON the caller can deserialize into a typed
//! primitive.
#![forbid(unsafe_code)]

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod cost;
pub mod mock;
pub mod vertex;

pub use cost::{CostLedger, CostReport};
pub use mock::MockLlmBackend;
pub use vertex::VertexAiBackend;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("auth: {0}")]
    Auth(String),
    #[error("http: {0}")]
    Http(String),
    #[error("decode: {0}")]
    Decode(String),
    #[error("rate limited")]
    RateLimited,
    #[error("schema mismatch: {0}")]
    Schema(String),
    #[error("other: {0}")]
    Other(String),
}

/// A single LLM extraction request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractRequest {
    pub system: String,
    pub user: String,
    /// Optional JSON schema constraining the response.
    pub schema: Option<serde_json::Value>,
    /// Model name. If `None`, backend picks default (`gemini-2.5-flash`).
    pub model: Option<String>,
    /// Sampling temperature; default 0.0 for reproducibility.
    pub temperature: Option<f32>,
    pub max_output_tokens: Option<u32>,
}

/// The backend response — parsed JSON and a usage report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractResponse {
    pub value: serde_json::Value,
    pub model: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
}

/// Embedding response.
#[derive(Debug, Clone)]
pub struct EmbedResponse {
    pub vectors: Vec<Vec<f32>>,
    pub model: String,
}

#[async_trait]
pub trait LlmBackend: Send + Sync {
    async fn extract_json(&self, req: ExtractRequest) -> Result<ExtractResponse, LlmError>;
    async fn embed(&self, texts: &[String]) -> Result<EmbedResponse, LlmError>;
    fn name(&self) -> &'static str;
}
