//! Anchoring — the first-number-wins negotiation move.
//!
//! Behavioral economics: in a negotiation, the first numeric offer creates a
//! cognitive anchor; subsequent offers cluster within ±X% of it even when the
//! anchor was arbitrary. AGON detects:
//!
//! - **anchor**: the first numeric claim in the thread (price, percent, units, date)
//! - **clustering**: later numeric claims that fall within ±30% of the anchor
//!
//! Output is a low-confidence raw signal — calibration in PROMPT 10 turns it
//! into a probability bin.

use crate::{ConflictPattern, PatternContext, PatternMatch, RawSignal};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::json;

/// `$1,234.56` `1,234.56` `1234` `12%` `12.5%` — captures the numeric magnitude.
static NUMERIC_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:\$|€|£)?\s?(\d{1,3}(?:,\d{3})*(?:\.\d+)?|\d+(?:\.\d+)?)\s?(?:%|k|K|M|million|billion)?")
        .expect("NUMERIC_RX valid")
});

#[derive(Debug, Clone, Default)]
pub struct AnchoringDetector;

impl ConflictPattern for AnchoringDetector {
    fn id(&self) -> &'static str { "anchoring" }
    fn version(&self) -> &'static str { "0.1.0" }
    fn public_name(&self) -> &'static str { "first-number-effect" }

    fn detect(&self, ctx: &PatternContext) -> Vec<PatternMatch> {
        let mut anchor: Option<(usize, String, f64)> = None;
        let mut nearby: Vec<(usize, String, f64)> = Vec::new();

        for (i, turn) in ctx.turns.iter().enumerate() {
            for cap in NUMERIC_RX.captures_iter(&turn.body) {
                let raw = cap.get(0).map_or("", |m| m.as_str()).to_string();
                let mag_str = cap.get(1).map_or("", |m| m.as_str()).replace(',', "");
                let Ok(mag) = mag_str.parse::<f64>() else { continue };
                if mag < 1.0 { continue; }  // skip "0.5" — too noisy

                match &anchor {
                    None => anchor = Some((i, raw, mag)),
                    Some((a_idx, _, a_mag)) if i > *a_idx => {
                        // Within ±30% of anchor → counts as anchored response.
                        let ratio = mag / a_mag;
                        if ratio >= 0.70 && ratio <= 1.30 {
                            nearby.push((i, raw, mag));
                        }
                    }
                    _ => {}
                }
            }
        }

        let Some((anchor_idx, anchor_raw, anchor_mag)) = anchor else { return Vec::new() };
        if nearby.is_empty() { return Vec::new() }

        // Confidence: anchor + ≥2 nearby = 0.75, +1 nearby = 0.55, else 0.40
        let confidence = match nearby.len() {
            0 => 0.0,
            1 => 0.55,
            _ => 0.75,
        };

        let speaker = ctx.turns
            .get(anchor_idx)
            .and_then(|t| t.speaker.as_ref())
            .cloned()
            .unwrap_or_else(|| "unknown".into());

        let actors: Vec<String> = nearby
            .iter()
            .filter_map(|(i, _, _)| ctx.turns.get(*i).and_then(|t| t.speaker.as_ref()).cloned())
            .map(|s| canonical_actor_id(&s))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .chain(std::iter::once(canonical_actor_id(&speaker)))
            .collect();

        let evidence: Vec<String> = std::iter::once(anchor_raw.clone())
            .chain(nearby.iter().map(|(_, raw, _)| raw.clone()))
            .collect();

        let mut raw_signals = vec![RawSignal {
            detector_id: "anchoring.anchor".into(),
            value: anchor_mag as f32,
            features: json!({ "turn": anchor_idx, "speaker": speaker, "raw": anchor_raw }),
        }];
        for (i, raw, mag) in &nearby {
            raw_signals.push(RawSignal {
                detector_id: "anchoring.cluster".into(),
                value: (mag / anchor_mag) as f32,
                features: json!({ "turn": i, "raw": raw, "ratio": mag / anchor_mag }),
            });
        }

        let explanation = format!(
            "First numeric claim was \"{anchor_raw}\" at turn {anchor_idx}. \
             {} subsequent numeric claim(s) cluster within ±30%: {}. \
             Possible anchoring effect — confidence raw, calibration pending.",
            nearby.len(),
            nearby.iter().map(|(_, r, _)| r.as_str()).collect::<Vec<_>>().join(", "),
        );

        vec![PatternMatch {
            pattern_id: self.id().into(),
            pattern_version: self.version().into(),
            public_name: self.public_name().into(),
            actors_involved: actors,
            raw_signals,
            raw_confidence: confidence,
            explanation,
            evidence_excerpts: evidence,
        }]
    }
}

fn canonical_actor_id(label: &str) -> String {
    let mut s = String::from("actor_");
    for c in label.chars() {
        if c.is_ascii_alphanumeric() { s.push(c.to_ascii_lowercase()); } else { s.push('_'); }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Turn;

    fn ctx<'a>(turns: &'a [Turn], lex: &'a aco_lex::LexicalFeatures) -> PatternContext<'a> {
        static EMPTY_SS: &[aco_text::SpeechSpan] = &[];
        static EMPTY_ST: &[aco_text::SpeakerTurn] = &[];
        PatternContext::new("", turns, EMPTY_SS, EMPTY_ST, lex)
    }

    #[test]
    fn detects_classic_anchoring() {
        let turns = vec![
            Turn { idx: 0, speaker: Some("Vendor".into()), body: "Our list price is $50,000.".into(), char_offset: 0 },
            Turn { idx: 1, speaker: Some("Buyer".into()), body: "We can do $48,000.".into(), char_offset: 0 },
            Turn { idx: 2, speaker: Some("Vendor".into()), body: "Let's settle at $52,000.".into(), char_offset: 0 },
        ];
        let lex = aco_lex::LexicalFeatures::default();
        let c = ctx(&turns, &lex);
        let m = AnchoringDetector.detect(&c);
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].pattern_id, "anchoring");
        assert!(m[0].raw_confidence >= 0.70);
        assert!(m[0].evidence_excerpts.len() >= 3);
    }

    #[test]
    fn no_match_when_no_anchor() {
        let turns = vec![
            Turn { idx: 0, speaker: Some("A".into()), body: "Let's talk.".into(), char_offset: 0 },
            Turn { idx: 1, speaker: Some("B".into()), body: "Sure.".into(), char_offset: 0 },
        ];
        let lex = aco_lex::LexicalFeatures::default();
        let c = ctx(&turns, &lex);
        assert!(AnchoringDetector.detect(&c).is_empty());
    }

    #[test]
    fn no_match_when_counter_far_from_anchor() {
        let turns = vec![
            Turn { idx: 0, speaker: Some("A".into()), body: "I want $1,000.".into(), char_offset: 0 },
            Turn { idx: 1, speaker: Some("B".into()), body: "How about $10?".into(), char_offset: 0 },
        ];
        let lex = aco_lex::LexicalFeatures::default();
        let c = ctx(&turns, &lex);
        // $10 is 1% of $1000 — far outside ±30% band.
        assert!(AnchoringDetector.detect(&c).is_empty());
    }

    #[test]
    fn detects_percentage_anchoring() {
        let turns = vec![
            Turn { idx: 0, speaker: Some("A".into()), body: "I think 20% is fair.".into(), char_offset: 0 },
            Turn { idx: 1, speaker: Some("B".into()), body: "Closer to 22%.".into(), char_offset: 0 },
            Turn { idx: 2, speaker: Some("A".into()), body: "Meet at 21%.".into(), char_offset: 0 },
        ];
        let lex = aco_lex::LexicalFeatures::default();
        let c = ctx(&turns, &lex);
        let m = AnchoringDetector.detect(&c);
        assert!(!m.is_empty());
    }
}
