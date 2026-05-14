//! aco-patterns — the moat. Named, versioned, golden-fixture-tested conflict patterns.
//!
//! Each pattern implements [`ConflictPattern`]. Detection runs against a
//! [`PatternContext`] holding the canonical text, speaker turns, lexical signals,
//! and (optionally) primitives already extracted by the LLM.
//!
//! v0.1 patterns:
//! - **DARVO** — Deny + Attack + Reverse Victim/Offender
//! - **anchoring** — first-number-wins negotiation move (stub)
//! - **scope creep** — topic drift across turns (stub)
//! - **conspicuous absence** — expected primitive missing for the context (stub)
//! - **coalition** — "we" outgrowth from "I" + new alliance members (stub)
//!
//! Public-facing names are neutral. `DARVO` surfaces as "possible role-reversal
//! pattern" in the UI; internal label stays for audit.
#![forbid(unsafe_code)]

pub mod anchoring;
pub mod context;
pub mod darvo;

pub use anchoring::AnchoringDetector;
pub use context::PatternContext;
pub use darvo::DarvoDetector;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMatch {
    pub pattern_id: String,
    pub pattern_version: String,
    /// Neutral, user-facing label.
    pub public_name: String,
    pub actors_involved: Vec<String>,
    pub raw_signals: Vec<RawSignal>,
    pub raw_confidence: f32,
    pub explanation: String,
    pub evidence_excerpts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawSignal {
    pub detector_id: String,
    pub value: f32,
    pub features: serde_json::Value,
}

pub trait ConflictPattern: Send + Sync {
    fn id(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn public_name(&self) -> &'static str;
    fn detect(&self, ctx: &PatternContext) -> Vec<PatternMatch>;
}

/// Run every registered pattern against a context.
#[must_use]
pub fn detect_all(ctx: &PatternContext) -> Vec<PatternMatch> {
    let detectors: Vec<Box<dyn ConflictPattern>> = vec![
        Box::new(DarvoDetector::default()),
        Box::new(AnchoringDetector::default()),
    ];
    detectors.iter().flat_map(|d| d.detect(ctx)).collect()
}
