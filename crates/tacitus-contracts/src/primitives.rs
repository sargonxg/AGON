//! Typed primitives. Mirrors `schemas/*.json` exactly.
//!
//! When edited, also edit the schema. CI test enforces parity.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─────────────────────────────────────────────────────────────────────────────
// Evidence Span — the audit primitive. Every claim-bearing primitive carries
// at least one EvidenceSpan.
// ─────────────────────────────────────────────────────────────────────────────

/// Quad-form evidence span. Both canonical-NFC and raw-byte offsets travel
/// together so spans survive normalization and OCR repair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceSpan {
    /// Blake3 hash of the canonical text of the containing segment.
    pub segment_id: String,
    /// Canonical (NFC) character offset, inclusive start.
    pub char_start_canonical: u32,
    /// Canonical character offset, exclusive end.
    pub char_end_canonical: u32,
    /// Raw byte offset in the original document, inclusive start.
    pub char_start_raw: u32,
    /// Raw byte offset, exclusive end.
    pub char_end_raw: u32,
    /// The literal quoted text after normalization.
    pub verbatim_quote: String,
    /// Blake3 hash of `verbatim_quote`.
    pub quote_hash: String,
    /// Normalization pipeline semver at the time the span was captured.
    pub normalization_version: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Document & Segment
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub source_uri: String,
    pub ingested_at: DateTime<Utc>,
    pub raw_hash: String,
    pub canonical_hash: String,
    pub language: Option<String>,
    pub kind: DocumentKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentKind {
    Email,
    Message,
    Transcript,
    Minutes,
    Memo,
    Briefing,
    Cable,
    Pleading,
    Letter,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub id: String, // blake3 prefix
    pub doc_id: Uuid,
    pub char_start: u32,
    pub char_end: u32,
    pub kind: SegmentKind,
    pub speaker_hint: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentKind {
    Paragraph,
    Heading,
    Quoted,
    ReportedSpeech,
    ListItem,
    Code,
}

// ─────────────────────────────────────────────────────────────────────────────
// Actor
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub id: String, // canonical id (e.g., "actor:sam")
    pub canonical_label: String,
    pub aliases: Vec<String>,
    pub role: Option<String>,
    pub provenance: Vec<EvidenceSpan>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Claim
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub id: String,
    pub asserter: String, // actor id
    pub text: String,
    pub polarity: Polarity,
    pub modality: Modality,
    pub evidence_spans: Vec<EvidenceSpan>,
    pub raw_confidence: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Polarity {
    Affirm,
    Deny,
    Hedge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Modality {
    Factual,
    Normative,
    Evaluative,
    Predictive,
}

// ─────────────────────────────────────────────────────────────────────────────
// Event
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub label: String,
    pub participants: Vec<String>, // actor ids
    pub time: Option<TemporalInterval>,
    pub evidence_spans: Vec<EvidenceSpan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalInterval {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub approximate: bool,
    pub raw: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AllenRelation {
    Before,
    After,
    Meets,
    MetBy,
    Overlaps,
    OverlappedBy,
    Starts,
    StartedBy,
    During,
    Contains,
    Finishes,
    FinishedBy,
    Equals,
}

// ─────────────────────────────────────────────────────────────────────────────
// Commitment
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commitment {
    pub id: String,
    pub committer: String,    // actor id
    pub beneficiary: Option<String>,
    pub subject: String,
    pub deadline: Option<String>,
    pub state: CommitmentState,
    pub evidence_spans: Vec<EvidenceSpan>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitmentState {
    Made,
    Confirmed,
    Contested,
    Withdrawn,
    Fulfilled,
    Broken,
}

// ─────────────────────────────────────────────────────────────────────────────
// Contradiction
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contradiction {
    pub id: String,
    pub claim_a: String,
    pub claim_b: String,
    pub materiality: Materiality,
    pub rationale: String,
    pub evidence_spans: Vec<EvidenceSpan>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Materiality {
    Material,
    Cosmetic,
}

// ─────────────────────────────────────────────────────────────────────────────
// Pattern match (DARVO, anchoring, scope-creep, etc.)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMatch {
    pub pattern_id: String,
    pub pattern_version: String,
    pub public_name: String, // user-facing neutral label
    pub taxonomy: PatternTaxonomy,
    pub actors_involved: Vec<String>,
    pub evidence_spans: Vec<EvidenceSpan>,
    pub raw_signals: Vec<RawSignal>,
    pub raw_confidence: f32,
    pub calibrated_confidence_bin: Option<ConfidenceBin>,
    pub explanation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternTaxonomy {
    Escalation,
    Negotiation,
    Coalition,
    Institutional,
    Linguistic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawSignal {
    pub detector_id: String,
    pub detector_version: String,
    pub value: f32,
    pub features: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceBin {
    Low,
    Medium,
    High,
    VeryHigh,
}

// ─────────────────────────────────────────────────────────────────────────────
// Provenance
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    pub id: Uuid,
    pub parents: Vec<Uuid>,
    pub kind: ProvenanceKind,
    pub content_hash: String,
    pub method_tag: Option<MethodTag>,
    pub created_at: DateTime<Utc>,
    pub pipeline_version: String,
    pub model_versions: std::collections::BTreeMap<String, String>,
    pub prompt_versions: std::collections::BTreeMap<String, String>,
    pub schema_version: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceKind {
    RawDoc,
    Segment,
    SensorRun,
    Primitive,
    Pattern,
    Inference,
    Report,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MethodTag {
    Sensor { name: String, version: String },
    Encoder { name: String, version: String },
    Llm { model_id: String, prompt_hash: String, temperature: f32 },
    Rule { rule_id: String, version: String },
    Human { user_id: String },
}

// ─────────────────────────────────────────────────────────────────────────────
// Calibration
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Confidence {
    pub raw: f32,
    pub calibrated: f32,
    pub bin: ConfidenceBin,
    pub calibrator_id: String,
    pub calibrator_version: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Interest, Leverage, Narrative Frame
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestInference {
    pub id: String,
    pub holder: String, // actor id
    pub description: String,
    pub fisher_ury_kind: FisherUryKind,
    pub evidence_spans: Vec<EvidenceSpan>,
    pub raw_confidence: f32, // expected to be low — interests are inferred
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FisherUryKind {
    Substantive,
    Procedural,
    Psychological,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leverage {
    pub id: String,
    pub holder: String, // actor id
    pub power_kind: PowerVector,
    pub description: String,
    pub evidence_spans: Vec<EvidenceSpan>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PowerVector {
    Structural,
    Moral,
    Informational,
    Time,
    Audience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeFrame {
    pub id: String,
    pub framer: String, // actor id
    pub frame_kind: String, // controlled vocab loaded at runtime
    pub claims_in_frame: Vec<String>, // claim ids
    pub evidence_spans: Vec<EvidenceSpan>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_span_roundtrip() {
        let s = EvidenceSpan {
            segment_id: "abc".into(),
            char_start_canonical: 0,
            char_end_canonical: 10,
            char_start_raw: 0,
            char_end_raw: 10,
            verbatim_quote: "hello".into(),
            quote_hash: "xyz".into(),
            normalization_version: "0.1.0".into(),
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: EvidenceSpan = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }

    #[test]
    fn commitment_state_serializes_snake_case() {
        let s = serde_json::to_string(&CommitmentState::Contested).unwrap();
        assert_eq!(s, "\"contested\"");
    }
}
