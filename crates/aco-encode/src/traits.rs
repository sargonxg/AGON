//! Core traits for AGON encoders.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("download: {0}")]
    Download(String),
    #[error("tokenize: {0}")]
    Tokenize(String),
    #[error("inference: {0}")]
    Inference(String),
    #[error("not loaded: model weights have not been downloaded yet")]
    NotLoaded,
    #[error("feature `onnx` disabled — build with --features onnx")]
    FeatureDisabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedResult {
    pub vector: Vec<f32>,
    /// Sparse weights from BGE-M3's sparse head (term-id → weight). Empty if not produced.
    #[serde(default)]
    pub sparse: std::collections::BTreeMap<u32, f32>,
    pub model_id: String,
    pub dim: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NliLabel {
    Entailment,
    Neutral,
    Contradiction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NliResult {
    pub label: NliLabel,
    pub logits: [f32; 3],
    pub model_id: String,
}

#[async_trait]
pub trait Encoder: Send + Sync {
    fn model_id(&self) -> &str;
    fn model_version(&self) -> &str;
    /// True when weights are downloaded and the runtime is loaded.
    fn is_loaded(&self) -> bool;
}

#[async_trait]
pub trait EmbedEncoder: Encoder {
    async fn embed(&self, text: &str) -> Result<EmbedResult, EncodeError>;
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<EmbedResult>, EncodeError>;
}

#[async_trait]
pub trait NliEncoder: Encoder {
    async fn classify(&self, premise: &str, hypothesis: &str) -> Result<NliResult, EncodeError>;
}
