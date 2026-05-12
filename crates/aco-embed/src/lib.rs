//! `aco-embed` — local neural/sparse signals for conflict vision.
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use tracing::trace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralConfig {
    pub mode: String,
    pub max_pairs: usize,
    pub min_similarity: f32,
}

impl Default for NeuralConfig {
    fn default() -> Self {
        Self { mode: "local_sparse".into(), max_pairs: 250, min_similarity: 0.62 }
    }
}

impl NeuralConfig {
    pub fn from_env() -> Self {
        let mut cfg = Self::default();
        if let Ok(mode) = std::env::var("AGON_NEURAL_MODE") {
            cfg.mode = mode;
        }
        if let Ok(max_pairs) = std::env::var("AGON_NEURAL_MAX_PAIRS") {
            cfg.max_pairs = max_pairs.parse().unwrap_or(cfg.max_pairs);
        }
        if let Ok(min_similarity) = std::env::var("AGON_NEURAL_MIN_SIM") {
            cfg.min_similarity = min_similarity.parse().unwrap_or(cfg.min_similarity);
        }
        cfg
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimInput {
    pub id: String,
    pub actor_id: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralSignal {
    pub kind: String,
    pub a: String,
    pub b: String,
    pub score: f32,
    pub rationale: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralSummary {
    pub mode: String,
    pub model: String,
    pub supported_models: Vec<String>,
    pub signals: Vec<NeuralSignal>,
}

#[derive(Debug, Clone)]
pub struct LocalNeuralSensor {
    cfg: NeuralConfig,
}

impl LocalNeuralSensor {
    pub fn new(cfg: NeuralConfig) -> Self {
        Self { cfg }
    }

    pub fn from_env() -> Self {
        Self::new(NeuralConfig::from_env())
    }

    pub fn analyze_claims(&self, claims: &[ClaimInput]) -> NeuralSummary {
        let mut signals = Vec::new();
        let limit = self.cfg.max_pairs.min(claims.len().saturating_mul(claims.len()));
        let mut seen = 0usize;

        for (i, a) in claims.iter().enumerate() {
            for b in claims.iter().skip(i + 1) {
                if seen >= limit {
                    break;
                }
                seen += 1;
                if a.actor_id == b.actor_id && a.actor_id.is_some() {
                    continue;
                }
                let score = sparse_similarity(&a.text, &b.text);
                if score >= self.cfg.min_similarity {
                    signals.push(NeuralSignal {
                        kind: "claim_relatedness".into(),
                        a: a.id.clone(),
                        b: b.id.clone(),
                        score,
                        rationale: "local sparse semantic overlap marks these claims as same-subject candidates for deterministic contradiction checks".into(),
                        model: self.model_name(),
                    });
                }
            }
        }

        NeuralSummary {
            mode: self.cfg.mode.clone(),
            model: self.model_name(),
            supported_models: supported_fastembed_models(),
            signals,
        }
    }

    fn model_name(&self) -> String {
        if self.cfg.mode == "local" || self.cfg.mode == "fastembed" {
            "fastembed-ready+sparse-fallback".into()
        } else {
            "local-sparse-conflict-sensor".into()
        }
    }
}

fn supported_fastembed_models() -> Vec<String> {
    #[cfg(feature = "neural")]
    {
        let mut names = Vec::new();
        if let Some(info) = fastembed::TextEmbedding::list_supported_models().first() {
            names.push(format!("embedding:{}", info.model_code));
        }
        if let Some(info) = fastembed::TextRerank::list_supported_models().first() {
            names.push(format!("reranker:{}", info.model_code));
        }
        names
    }

    #[cfg(not(feature = "neural"))]
    {
        vec![
            "local_sparse".into(),
            "fastembed optional feature available for model-backed sensors".into(),
        ]
    }
}

fn sparse_similarity(a: &str, b: &str) -> f32 {
    let av = weighted_terms(a);
    let bv = weighted_terms(b);
    if av.is_empty() || bv.is_empty() {
        return 0.0;
    }
    let dot: f32 = av.iter().filter_map(|(term, aw)| bv.get(term).map(|bw| aw * bw)).sum();
    let an = av.values().map(|v| v * v).sum::<f32>().sqrt();
    let bn = bv.values().map(|v| v * v).sum::<f32>().sqrt();
    if an == 0.0 || bn == 0.0 {
        0.0
    } else {
        (dot / (an * bn)).clamp(0.0, 1.0)
    }
}

fn weighted_terms(text: &str) -> BTreeMap<String, f32> {
    let mut out = BTreeMap::new();
    let stop: HashSet<&str> = [
        "the", "and", "that", "this", "with", "from", "have", "has", "had", "was", "were", "said",
        "says", "will", "would", "could", "should", "they", "them", "their", "about", "into",
        "onto", "than",
    ]
    .into_iter()
    .collect();
    for token in text
        .split(|c: char| !c.is_ascii_alphanumeric())
        .map(str::to_ascii_lowercase)
        .filter(|t| t.len() >= 3 && !stop.contains(t.as_str()))
    {
        let weight = match token.as_str() {
            "never" | "not" | "denied" | "deny" | "agreed" | "approved" | "cancelled"
            | "canceled" | "postponed" | "removed" | "retaliation" | "bypass" | "escalated" => 1.8,
            _ => 1.0,
        };
        *out.entry(token).or_insert(0.0) += weight;
    }
    out
}

pub fn init() {
    trace!(crate_name = "aco-embed", "loaded");
}

#[cfg(test)]
mod tests {
    use super::{ClaimInput, LocalNeuralSensor, NeuralConfig};

    #[test]
    fn finds_related_claim_candidates() {
        let sensor =
            LocalNeuralSensor::new(NeuralConfig { min_similarity: 0.35, ..Default::default() });
        let claims = vec![
            ClaimInput {
                id: "c1".into(),
                actor_id: Some("sam".into()),
                text: "Alex agreed to own the launch deck content".into(),
            },
            ClaimInput {
                id: "c2".into(),
                actor_id: Some("alex".into()),
                text: "I never agreed to own the deck content".into(),
            },
        ];
        let summary = sensor.analyze_claims(&claims);
        assert_eq!(summary.signals.len(), 1);
        assert_eq!(summary.signals[0].kind, "claim_relatedness");
    }
}
