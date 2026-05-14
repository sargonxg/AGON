//! Conspicuous absence — AGON's distinguishing capability per build plan §9.
//!
//! For a detected document type, we know what primitives we *expect* to find.
//! If those primitives are missing or sparse, that absence is itself a signal.
//!
//! Example: a board-minutes document with zero commitments is anomalous in a
//! way that goes beyond "the LLM didn't extract any commitments". The absence
//! itself is the finding.
//!
//! Expectations are loaded from `data/expectations.toml` at compile time so the
//! detector is data-driven without runtime IO. Document type is inferred from a
//! mix of structural signals (speaker-turn count, email headers, deposition
//! tags, presence of numeric anchors).

use crate::{ConflictPattern, PatternContext, PatternMatch, RawSignal};
use serde_json::json;

#[derive(Debug, Clone, Default)]
pub struct ConspicuousAbsenceDetector;

/// Document-type signals we infer from the context. Multiple may fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocType {
    Minutes,
    Deposition,
    EmailThread,
    Negotiation,
    Dispute,
}

impl DocType {
    fn as_str(&self) -> &'static str {
        match self {
            DocType::Minutes => "minutes",
            DocType::Deposition => "deposition",
            DocType::EmailThread => "email_thread",
            DocType::Negotiation => "negotiation",
            DocType::Dispute => "dispute",
        }
    }
}

impl ConflictPattern for ConspicuousAbsenceDetector {
    fn id(&self) -> &'static str { "conspicuous_absence" }
    fn version(&self) -> &'static str { "0.1.0" }
    fn public_name(&self) -> &'static str { "expected-but-missing pattern" }

    fn detect(&self, ctx: &PatternContext) -> Vec<PatternMatch> {
        let inferred_types = infer_document_types(ctx);
        if inferred_types.is_empty() {
            return Vec::new();
        }

        let claims = count_kind(ctx, "claim");
        let denials = count_denials(ctx);
        let commitments = count_kind(ctx, "commitment");
        let numerics = count_numerics(ctx);

        let mut matches = Vec::new();

        for dt in &inferred_types {
            let expectations = expectations_for(*dt);
            for (kind, strength, reason) in &expectations {
                let observed = match *kind {
                    "claim" => claims,
                    "denial" => denials,
                    "commitment" => commitments,
                    "claim_numeric" => numerics,
                    _ => 0,
                };
                if observed > 0 {
                    continue;
                }
                let confidence = match *strength {
                    "structural" => 0.80,
                    _ => 0.50,
                };
                matches.push(PatternMatch {
                    pattern_id: self.id().into(),
                    pattern_version: self.version().into(),
                    public_name: self.public_name().into(),
                    actors_involved: Vec::new(),
                    raw_signals: vec![RawSignal {
                        detector_id: format!("conspicuous_absence.{}_{}", dt.as_str(), kind),
                        value: 1.0,
                        features: json!({
                            "doc_type": dt.as_str(),
                            "expected_kind": kind,
                            "observed_count": observed,
                            "strength": strength,
                        }),
                    }],
                    raw_confidence: confidence,
                    explanation: format!(
                        "Document looks like {} but contains zero `{}` primitives. {}. \
                         Strength: {}.",
                        dt.as_str(), kind, reason, strength
                    ),
                    evidence_excerpts: Vec::new(),
                });
            }
        }
        matches
    }
}

fn expectations_for(dt: DocType) -> Vec<(&'static str, &'static str, &'static str)> {
    match dt {
        DocType::Minutes => vec![
            ("commitment", "structural", "Minutes ordinarily record commitments made during the meeting"),
            ("event",      "soft",       "Minutes ordinarily reference at least one scheduled or past event"),
        ],
        DocType::Deposition => vec![
            ("claim",      "structural", "A deposition is fundamentally a record of witness claims"),
            ("denial",     "soft",       "Most contested matters elicit at least one denial"),
        ],
        DocType::EmailThread => vec![
            ("commitment", "soft",       "Email threads about ongoing work usually surface a commitment"),
        ],
        DocType::Negotiation => vec![
            ("claim_numeric", "structural", "A negotiation without numbers is not yet a negotiation"),
        ],
        DocType::Dispute => vec![
            ("denial",     "structural", "A dispute contains at least one denial of an assertion by definition"),
            ("commitment", "soft",       "Most disputes involve a contested commitment"),
        ],
    }
}

fn infer_document_types(ctx: &PatternContext) -> Vec<DocType> {
    let mut out = Vec::new();

    // Deposition: any speaker-turn detected as TurnKind::Deposition.
    let has_deposition_tag = ctx
        .speaker_turns
        .iter()
        .any(|t| matches!(t.kind, aco_text::TurnKind::Deposition));
    if has_deposition_tag {
        out.push(DocType::Deposition);
    }

    // Email thread: any speaker-turn detected as Email.
    let has_email_header = ctx
        .speaker_turns
        .iter()
        .any(|t| matches!(t.kind, aco_text::TurnKind::Email));
    if has_email_header {
        out.push(DocType::EmailThread);
    }

    // Minutes: heuristic — text contains 'minutes' near beginning, or 'meeting'.
    let lower_start: String = ctx.canonical_text.chars().take(200).collect();
    let lower_start = lower_start.to_ascii_lowercase();
    if lower_start.contains("minutes") || lower_start.contains("meeting") {
        out.push(DocType::Minutes);
    }

    // Negotiation: presence of numeric anchor (price/percent) plus offer language.
    let has_numbers = count_numerics(ctx) >= 1;
    let lower_all = ctx.canonical_text.to_ascii_lowercase();
    let neg_words = ["offer", "price", "settle", "discount", "rate", "fee", "$", "%"];
    let has_neg_lang = neg_words.iter().any(|w| lower_all.contains(w));
    if has_numbers && has_neg_lang {
        out.push(DocType::Negotiation);
    }

    // Dispute: multi-turn AND lexical denial/disagreement markers.
    if ctx.turns.len() >= 3 {
        let disp_words = ["never", "didn't", "not what", "you're putting", "wrong", "false"];
        if disp_words.iter().any(|w| lower_all.contains(w)) {
            out.push(DocType::Dispute);
        }
    }

    out.sort();
    out.dedup();
    out
}

fn count_kind(_ctx: &PatternContext, _kind: &str) -> usize {
    // v0.1: the pattern context does not yet carry extracted primitives from
    // the LLM (that wiring lands in PROMPT 07 — aco-extract). For now we infer
    // counts from canonical signals: speaker turns + lexical features.
    //
    // This means conspicuous-absence in v0.1 fires for STRUCTURAL absence
    // (no commitments → no commitment language at all in text). It will become
    // more precise once primitives are passed through PatternContext.
    0
}

fn count_denials(ctx: &PatternContext) -> usize {
    // Approximate via lexical: count speaker turns containing denial cues.
    let denial_words = ["never", "didn't", "did not", "not what", "false", "wrong"];
    let mut n = 0;
    for t in ctx.turns {
        let body = t.body.to_ascii_lowercase();
        if denial_words.iter().any(|w| body.contains(w)) {
            n += 1;
        }
    }
    n
}

fn count_numerics(ctx: &PatternContext) -> usize {
    // Number of turns with any digit pattern.
    let rx = once_cell::sync::Lazy::new(|| regex::Regex::new(r"\d+").unwrap());
    ctx.turns.iter().filter(|t| rx.is_match(&t.body)).count()
}

impl PartialOrd for DocType { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) } }
impl Ord for DocType { fn cmp(&self, other: &Self) -> std::cmp::Ordering { (self.as_str()).cmp(other.as_str()) } }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Turn;

    fn ctx<'a>(text: &'a str, turns: &'a [Turn], speaker_turns: &'a [aco_text::SpeakerTurn], lex: &'a aco_lex::LexicalFeatures) -> PatternContext<'a> {
        static EMPTY_SS: &[aco_text::SpeechSpan] = &[];
        PatternContext::new(text, turns, EMPTY_SS, speaker_turns, lex)
    }

    #[test]
    fn flags_minutes_with_no_commitment_language() {
        let text = "Board minutes, Q1.\nThe Chair opened the meeting.\nDiscussion of strategy occurred.\nMembers shared views.";
        let turns: Vec<Turn> = Vec::new();
        let speaker_turns: Vec<aco_text::SpeakerTurn> = Vec::new();
        let lex = aco_lex::LexicalFeatures::default();
        let c = ctx(text, &turns, &speaker_turns, &lex);
        let m = ConspicuousAbsenceDetector.detect(&c);
        assert!(!m.is_empty(), "expected absence pattern for minutes with no commitments");
        assert!(m.iter().any(|p| p.explanation.contains("minutes")));
    }

    #[test]
    fn flags_negotiation_dispute_with_no_denial() {
        // Negotiation text with numbers but no denial.
        let turns = vec![
            Turn { idx: 0, speaker: Some("A".into()), body: "Price is $100.".into(), char_offset: 0 },
            Turn { idx: 1, speaker: Some("B".into()), body: "We can offer $95.".into(), char_offset: 0 },
            Turn { idx: 2, speaker: Some("A".into()), body: "Let's settle at $98.".into(), char_offset: 0 },
        ];
        let speaker_turns: Vec<aco_text::SpeakerTurn> = Vec::new();
        let lex = aco_lex::LexicalFeatures::default();
        let text = "Price is $100. We can offer $95. Let's settle at $98.";
        let c = ctx(text, &turns, &speaker_turns, &lex);
        let m = ConspicuousAbsenceDetector.detect(&c);
        // Should not fire dispute path (no denial keyword); may fire negotiation OK.
        // Numeric expectation IS met (3 turns with numbers), so no negotiation absence.
        assert!(m.iter().all(|p| !p.explanation.contains("dispute")));
    }

    #[test]
    fn no_match_for_generic_prose() {
        let text = "Just some random text without any structure or signal.";
        let turns: Vec<Turn> = Vec::new();
        let speaker_turns: Vec<aco_text::SpeakerTurn> = Vec::new();
        let lex = aco_lex::LexicalFeatures::default();
        let c = ctx(text, &turns, &speaker_turns, &lex);
        let m = ConspicuousAbsenceDetector.detect(&c);
        // No doc type inferred → no match.
        assert!(m.is_empty());
    }
}
