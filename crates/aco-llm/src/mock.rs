//! Mock backend — deterministic fixture-driven for tests.

use crate::{CostLedger, EmbedResponse, ExtractRequest, ExtractResponse, LlmBackend, LlmError};
use async_trait::async_trait;
use serde_json::json;

/// Mock backend. Returns a canned response based on a heuristic.
#[derive(Default, Debug)]
pub struct MockLlmBackend {
    pub cost: CostLedger,
}

impl MockLlmBackend {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl LlmBackend for MockLlmBackend {
    async fn extract_json(&self, req: ExtractRequest) -> Result<ExtractResponse, LlmError> {
        // Trivial entity extractor stub: pull capitalized tokens from `user`.
        let actors: Vec<String> = req
            .user
            .split_whitespace()
            .filter(|w| {
                let cleaned: String = w.chars().filter(|c| c.is_alphabetic()).collect();
                cleaned.len() > 2
                    && cleaned.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
            })
            .map(|w| w.chars().filter(|c| c.is_alphabetic()).collect())
            .take(8)
            .collect();

        let value = json!({
            "actors": actors,
            "claims": [],
            "events": [],
            "_mock": true
        });

        let in_t = (req.user.len() / 4) as u32;
        let out_t = 16u32;
        let model = req.model.unwrap_or_else(|| "mock".into());
        self.cost.record(&model, in_t, out_t);

        Ok(ExtractResponse { value, model, input_tokens: in_t, output_tokens: out_t })
    }

    async fn embed(&self, texts: &[String]) -> Result<EmbedResponse, LlmError> {
        let vectors = texts
            .iter()
            .map(|t| {
                let mut v = vec![0.0f32; 384];
                for (i, b) in t.bytes().enumerate() {
                    v[i % 384] += b as f32 / 255.0;
                }
                v
            })
            .collect();
        Ok(EmbedResponse { vectors, model: "mock-embed".into() })
    }

    fn name(&self) -> &'static str {
        "mock"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_extracts_capitalized_actors() {
        let m = MockLlmBackend::new();
        let resp = m
            .extract_json(ExtractRequest {
                system: "s".into(),
                user: "Sarah said Alex never agreed".into(),
                schema: None,
                model: None,
                temperature: None,
                max_output_tokens: None,
            })
            .await
            .unwrap();
        let actors = resp.value["actors"].as_array().unwrap();
        assert!(actors.len() >= 2);
    }
}
