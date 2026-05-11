//! Vertex AI Gemini backend — service-account auth via metadata server (Cloud Run)
//! or ADC (local). Schema-constrained JSON output.

use crate::{CostLedger, EmbedResponse, ExtractRequest, ExtractResponse, LlmBackend, LlmError};
use async_trait::async_trait;
use parking_lot::RwLock;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::{Duration, Instant};

const DEFAULT_FLASH: &str = "gemini-2.5-flash";

/// Vertex AI client.
pub struct VertexAiBackend {
    project_id: String,
    location: String,
    default_model: String,
    http: Client,
    token: Arc<RwLock<Option<CachedToken>>>,
    cost: CostLedger,
}

#[derive(Clone)]
struct CachedToken {
    value: String,
    expires_at: Instant,
}

#[derive(Serialize)]
struct GenerateRequest<'a> {
    contents: Vec<GenContent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "systemInstruction")]
    system_instruction: Option<GenContent<'a>>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
}

#[derive(Serialize)]
struct GenContent<'a> {
    role: &'a str,
    parts: Vec<GenPart<'a>>,
}

#[derive(Serialize)]
struct GenPart<'a> {
    text: &'a str,
}

#[derive(Serialize)]
struct GenerationConfig {
    temperature: f32,
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
    #[serde(rename = "responseMimeType")]
    response_mime_type: String,
    #[serde(rename = "responseSchema", skip_serializing_if = "Option::is_none")]
    response_schema: Option<Value>,
}

#[derive(Deserialize)]
struct GenerateResponse {
    candidates: Option<Vec<Candidate>>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<UsageMetadata>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Option<CandidateContent>,
    #[serde(rename = "finishReason", default)]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct CandidateContent {
    parts: Option<Vec<CandidatePart>>,
}

#[derive(Deserialize)]
struct CandidatePart {
    text: Option<String>,
}

#[derive(Deserialize, Default)]
struct UsageMetadata {
    #[serde(rename = "promptTokenCount", default)]
    prompt_token_count: u32,
    #[serde(rename = "candidatesTokenCount", default)]
    candidates_token_count: u32,
}

impl VertexAiBackend {
    pub fn new(project_id: String, location: String) -> Self {
        Self {
            project_id,
            location,
            default_model: DEFAULT_FLASH.into(),
            http: Client::builder()
                .timeout(Duration::from_secs(120))
                .build()
                .expect("reqwest client"),
            token: Arc::new(RwLock::new(None)),
            cost: CostLedger::new(),
        }
    }

    pub fn with_default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = model.into();
        self
    }

    pub fn cost_ledger(&self) -> CostLedger {
        self.cost.clone()
    }

    /// Fetch an access token via the metadata server (Cloud Run / GCE) or
    /// fall back to gcloud's ADC file when running locally.
    async fn access_token(&self) -> Result<String, LlmError> {
        if let Some(t) = self.token.read().clone() {
            if t.expires_at > Instant::now() + Duration::from_secs(60) {
                return Ok(t.value);
            }
        }
        // Try metadata server first.
        let md = self.http
            .get("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token")
            .header("Metadata-Flavor", "Google")
            .send().await;
        let token = match md {
            Ok(r) if r.status().is_success() => {
                let body: serde_json::Value =
                    r.json().await.map_err(|e| LlmError::Decode(e.to_string()))?;
                let tok = body["access_token"]
                    .as_str()
                    .ok_or_else(|| LlmError::Auth("no access_token in metadata response".into()))?
                    .to_string();
                let secs = body["expires_in"].as_u64().unwrap_or(3000);
                let cached = CachedToken {
                    value: tok.clone(),
                    expires_at: Instant::now() + Duration::from_secs(secs),
                };
                *self.token.write() = Some(cached);
                tok
            }
            _ => {
                // Local dev: shell out to gcloud for a token.
                let out = tokio::process::Command::new("gcloud")
                    .args(["auth", "application-default", "print-access-token"])
                    .output()
                    .await
                    .map_err(|e| LlmError::Auth(format!("gcloud not available: {e}")))?;
                if !out.status.success() {
                    return Err(LlmError::Auth(format!(
                        "gcloud auth print-access-token failed: {}",
                        String::from_utf8_lossy(&out.stderr)
                    )));
                }
                let tok = String::from_utf8_lossy(&out.stdout).trim().to_string();
                let cached = CachedToken {
                    value: tok.clone(),
                    expires_at: Instant::now() + Duration::from_secs(3000),
                };
                *self.token.write() = Some(cached);
                tok
            }
        };
        Ok(token)
    }
}

#[async_trait]
impl LlmBackend for VertexAiBackend {
    async fn extract_json(&self, req: ExtractRequest) -> Result<ExtractResponse, LlmError> {
        let model = req.model.unwrap_or_else(|| self.default_model.clone());
        let token = self.access_token().await?;
        let url = format!(
            "https://{loc}-aiplatform.googleapis.com/v1/projects/{proj}/locations/{loc}/publishers/google/models/{model}:generateContent",
            loc = self.location, proj = self.project_id, model = model,
        );

        let body = GenerateRequest {
            contents: vec![GenContent { role: "user", parts: vec![GenPart { text: &req.user }] }],
            system_instruction: Some(GenContent {
                role: "system",
                parts: vec![GenPart { text: &req.system }],
            }),
            generation_config: GenerationConfig {
                temperature: req.temperature.unwrap_or(0.0),
                max_output_tokens: req.max_output_tokens.unwrap_or(2048),
                response_mime_type: "application/json".into(),
                response_schema: req.schema,
            },
        };

        let resp = self
            .http
            .post(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;

        let status = resp.status();
        if !status.is_success() {
            let txt = resp.text().await.unwrap_or_default();
            if status.as_u16() == 429 {
                return Err(LlmError::RateLimited);
            }
            return Err(LlmError::Http(format!("{status}: {txt}")));
        }

        let gen: GenerateResponse =
            resp.json().await.map_err(|e| LlmError::Decode(e.to_string()))?;
        let cand = gen
            .candidates
            .and_then(|c| c.into_iter().next())
            .ok_or_else(|| LlmError::Decode("no candidates".into()))?;
        let finish = cand.finish_reason.clone().unwrap_or_default();
        let text = cand
            .content
            .and_then(|c| c.parts)
            .and_then(|p| p.into_iter().next())
            .and_then(|p| p.text)
            .ok_or_else(|| LlmError::Decode("empty candidate".into()))?;

        let value: Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                if finish == "MAX_TOKENS" {
                    return Err(LlmError::Schema(format!(
                        "model hit output token cap ({}). Increase max_output_tokens or shorten input.",
                        req.max_output_tokens.unwrap_or(2048)
                    )));
                }
                // Last-resort: try to recover by salvaging up to last balanced brace.
                let salvaged = salvage_json(&text);
                match salvaged.and_then(|s| serde_json::from_str(&s).ok()) {
                    Some(v) => v,
                    None => {
                        return Err(LlmError::Schema(format!(
                            "not valid JSON: {e}; finish={finish}; preview: {}",
                            text.chars().take(300).collect::<String>()
                        )))
                    }
                }
            }
        };

        let usage = gen.usage_metadata.unwrap_or_default();
        self.cost.record(&model, usage.prompt_token_count, usage.candidates_token_count);

        Ok(ExtractResponse {
            value,
            model,
            input_tokens: usage.prompt_token_count,
            output_tokens: usage.candidates_token_count,
        })
    }

    async fn embed(&self, texts: &[String]) -> Result<EmbedResponse, LlmError> {
        let token = self.access_token().await?;
        let url = format!(
            "https://{loc}-aiplatform.googleapis.com/v1/projects/{proj}/locations/{loc}/publishers/google/models/text-embedding-005:predict",
            loc = self.location, proj = self.project_id,
        );
        let instances: Vec<Value> = texts
            .iter()
            .map(|t| json!({ "content": t, "task_type": "RETRIEVAL_DOCUMENT" }))
            .collect();
        let body = json!({ "instances": instances });

        let resp = self
            .http
            .post(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(LlmError::Http(format!(
                "{}: {}",
                resp.status(),
                resp.text().await.unwrap_or_default()
            )));
        }

        let body: Value = resp.json().await.map_err(|e| LlmError::Decode(e.to_string()))?;
        let preds = body["predictions"]
            .as_array()
            .ok_or_else(|| LlmError::Decode("no predictions".into()))?;
        let vectors: Vec<Vec<f32>> = preds
            .iter()
            .filter_map(|p| {
                p["embeddings"]["values"]
                    .as_array()
                    .map(|arr| arr.iter().filter_map(|v| v.as_f64().map(|f| f as f32)).collect())
            })
            .collect();

        Ok(EmbedResponse { vectors, model: "text-embedding-005".into() })
    }

    fn name(&self) -> &'static str {
        "vertex-ai"
    }
}

/// Best-effort recovery of truncated JSON by closing open structures
/// up to the last complete value boundary. Conservative — returns None
/// if it can't make sense of the input.
fn salvage_json(s: &str) -> Option<String> {
    // Trim trailing partial-token noise: walk back to last `,` or value end at depth.
    let bytes = s.as_bytes();
    let mut stack: Vec<u8> = Vec::new();
    let mut in_string = false;
    let mut escape = false;
    let mut last_complete = 0usize;
    let mut last_stack: Vec<u8> = Vec::new();
    for (i, &b) in bytes.iter().enumerate() {
        if escape {
            escape = false;
            continue;
        }
        if in_string {
            match b {
                b'\\' => escape = true,
                b'"' => in_string = false,
                _ => {}
            }
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'{' => stack.push(b'}'),
            b'[' => stack.push(b']'),
            b'}' | b']' => {
                stack.pop();
                last_complete = i + 1;
                last_stack = stack.clone();
            }
            b',' if stack.len() == 1 => {
                last_complete = i;
                last_stack = stack.clone();
            }
            _ => {}
        }
    }
    if last_complete == 0 {
        return None;
    }
    let trimmed = &s[..last_complete];
    // Strip trailing comma if present.
    let trimmed = trimmed.trim_end_matches(|c: char| c.is_whitespace() || c == ',');
    let closes: String = last_stack.iter().rev().map(|&b| b as char).collect();
    Some(format!("{trimmed}{closes}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn salvages_truncated_array() {
        let truncated = r#"{"actors":[{"id":"a1","label":"Sam"},{"id":"a2","label":"Al"#;
        let fixed = salvage_json(truncated).unwrap();
        let v: Value = serde_json::from_str(&fixed).unwrap();
        assert_eq!(v["actors"].as_array().unwrap().len(), 1);
    }
}
