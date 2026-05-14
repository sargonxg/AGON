//! Verify every shipped schema loads and accepts a known-good fixture,
//! and that Rust types in `primitives.rs` serialize into a shape that
//! validates against the schema.

use serde_json::json;
use tacitus_contracts::{primitives::*, schemas};

fn span() -> serde_json::Value {
    json!({
        "segment_id": "seg-abc",
        "char_start_canonical": 0,
        "char_end_canonical": 10,
        "char_start_raw": 0,
        "char_end_raw": 10,
        "verbatim_quote": "hello",
        "quote_hash": "xyz",
        "normalization_version": "0.1.0"
    })
}

#[test]
fn evidence_span_schema_accepts_good() {
    let v = schemas::evidence_span();
    assert!(v.is_valid(&span()));
}

#[test]
fn evidence_span_schema_rejects_bad_version() {
    let v = schemas::evidence_span();
    let mut bad = span();
    bad["normalization_version"] = json!("not-a-semver");
    assert!(!v.is_valid(&bad));
}

#[test]
fn actor_round_trips_through_schema() {
    let v = schemas::actor();
    let a = Actor {
        id: "actor:sam".into(),
        canonical_label: "Sam".into(),
        aliases: vec!["S.".into()],
        role: None,
        provenance: vec![EvidenceSpan {
            segment_id: "seg-abc".into(),
            char_start_canonical: 0,
            char_end_canonical: 10,
            char_start_raw: 0,
            char_end_raw: 10,
            verbatim_quote: "Sam".into(),
            quote_hash: "h".into(),
            normalization_version: "0.1.0".into(),
        }],
    };
    let json = serde_json::to_value(&a).unwrap();
    assert!(v.is_valid(&json), "Actor serialization did not pass schema validation: {json:#}");
}

#[test]
fn claim_round_trips_through_schema() {
    let v = schemas::claim();
    let c = Claim {
        id: "claim-1".into(),
        asserter: "actor:sam".into(),
        text: "Alex agreed to own the deck.".into(),
        polarity: Polarity::Affirm,
        modality: Modality::Factual,
        evidence_spans: vec![EvidenceSpan {
            segment_id: "seg-1".into(),
            char_start_canonical: 0,
            char_end_canonical: 30,
            char_start_raw: 0,
            char_end_raw: 30,
            verbatim_quote: "Alex agreed to own the deck.".into(),
            quote_hash: "h".into(),
            normalization_version: "0.1.0".into(),
        }],
        raw_confidence: 0.9,
    };
    let json = serde_json::to_value(&c).unwrap();
    assert!(v.is_valid(&json));
}

#[test]
fn commitment_state_uses_snake_case() {
    let v = schemas::commitment();
    let c = Commitment {
        id: "cm-1".into(),
        committer: "actor:alex".into(),
        beneficiary: Some("actor:sam".into()),
        subject: "Q4 launch deck".into(),
        deadline: Some("Thursday".into()),
        state: CommitmentState::Contested,
        evidence_spans: vec![EvidenceSpan {
            segment_id: "seg-1".into(),
            char_start_canonical: 0,
            char_end_canonical: 10,
            char_start_raw: 0,
            char_end_raw: 10,
            verbatim_quote: "I never said".into(),
            quote_hash: "h".into(),
            normalization_version: "0.1.0".into(),
        }],
    };
    let json = serde_json::to_value(&c).unwrap();
    assert!(v.is_valid(&json), "Commitment did not validate: {json:#}");
}
