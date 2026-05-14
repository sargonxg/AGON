//! DARVO — Deny + Attack + Reverse Victim/Offender.
//!
//! Classical move: when accused, the actor (a) denies the accusation, (b) attacks
//! the original accuser's credibility/character, (c) reframes themselves as the
//! true victim and the accuser as the offender.
//!
//! Deterministic precondition (this v0.1 detector):
//! - turn N from actor A contains a **denial cue** (`I never`, `that's not true`,
//!   `I didn't`, `that's false`)
//! - turn N+k (k ≥ 1) from actor A contains an **attack/reverse cue**
//!   (`you're putting words`, `you're the one`, `you said you'd`,
//!   `you started`, `you're attacking me`, `stop accusing`)
//! - same speaker for both turns
//!
//! That precondition is intentionally tight — high precision, lower recall. LLM
//! adjudication and richer signal fusion lives in `aco-patterns/darvo_llm.rs`
//! (next PROMPT 09 iteration).
//!
//! Golden fixtures live under `crates/aco-patterns/fixtures/darvo/`.

use crate::{ConflictPattern, PatternContext, PatternMatch, RawSignal};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::json;

#[derive(Debug, Clone, Default)]
pub struct DarvoDetector;

static DENIAL_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(i never|i did(?:n['’]?t| not)|that['’]?s (?:not true|false|a lie)|i (?:didn['’]?t|never) (?:say|said|agree|agreed|do|did)|never said)\b")
        .expect("DENIAL_RX valid")
});

static REVERSE_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(you['’]?re putting words in my mouth|you['’]?re the one|you said you['’]?d|you started|stop accusing|you['’]?re attacking|you['’]?re twisting|you're the one being|you keep accusing|you have no right)\b")
        .expect("REVERSE_RX valid")
});

impl ConflictPattern for DarvoDetector {
    fn id(&self) -> &'static str { "darvo" }
    fn version(&self) -> &'static str { "0.1.0" }
    fn public_name(&self) -> &'static str { "possible role-reversal pattern" }

    fn detect(&self, ctx: &PatternContext) -> Vec<PatternMatch> {
        let mut matches = Vec::new();

        // First: find denials by speaker.
        let mut denials: Vec<(usize, &crate::context::Turn)> = Vec::new();
        for (i, t) in ctx.turns.iter().enumerate() {
            if DENIAL_RX.is_match(&t.body) {
                denials.push((i, t));
            }
        }

        for (deny_idx, deny_turn) in &denials {
            let speaker = match &deny_turn.speaker {
                Some(s) => s,
                None => continue,
            };
            // Look forward for an attack/reverse from the same speaker.
            for (fwd_idx, fwd_turn) in ctx.turns.iter().enumerate().skip(deny_idx + 1) {
                if fwd_turn.speaker.as_deref() != Some(speaker) {
                    continue;
                }
                if !REVERSE_RX.is_match(&fwd_turn.body) {
                    continue;
                }
                let gap = fwd_idx - deny_idx;
                // Closer turns → higher confidence.
                let confidence = match gap {
                    0..=1 => 0.85,
                    2..=3 => 0.70,
                    _ => 0.55,
                };

                let deny_quote = DENIAL_RX
                    .find(&deny_turn.body)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
                let reverse_quote = REVERSE_RX
                    .find(&fwd_turn.body)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();

                let actor_id = canonical_actor_id(speaker);
                matches.push(PatternMatch {
                    pattern_id: self.id().into(),
                    pattern_version: self.version().into(),
                    public_name: self.public_name().into(),
                    actors_involved: vec![actor_id.clone()],
                    raw_signals: vec![
                        RawSignal {
                            detector_id: "darvo.denial_cue".into(),
                            value: 1.0,
                            features: json!({ "turn": deny_idx, "match": deny_quote }),
                        },
                        RawSignal {
                            detector_id: "darvo.reverse_cue".into(),
                            value: 1.0,
                            features: json!({ "turn": fwd_idx, "match": reverse_quote, "gap": gap }),
                        },
                    ],
                    raw_confidence: confidence,
                    explanation: format!(
                        "Actor `{actor_id}` denied at turn {deny_idx} (\"{deny_quote}\") then \
                         attacked/reframed the accuser at turn {fwd_idx} (\"{reverse_quote}\"). \
                         Classical role-reversal sequence — confidence is raw, calibration pending."
                    ),
                    evidence_excerpts: vec![deny_quote, reverse_quote],
                });
            }
        }

        matches
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
    fn detects_textbook_darvo() {
        let turns = vec![
            Turn { idx: 0, speaker: Some("Sam".into()), body: "You promised Thursday.".into(), char_offset: 0 },
            Turn { idx: 1, speaker: Some("Alex".into()), body: "I never said I'd own it.".into(), char_offset: 0 },
            Turn { idx: 2, speaker: Some("Sam".into()), body: "That's not what we discussed.".into(), char_offset: 0 },
            Turn { idx: 3, speaker: Some("Alex".into()), body: "You're putting words in my mouth.".into(), char_offset: 0 },
        ];
        let lex = aco_lex::LexicalFeatures::default();
        let c = ctx(&turns, &lex);
        let detector = DarvoDetector;
        let matches = detector.detect(&c);
        assert_eq!(matches.len(), 1, "expected one DARVO match");
        let m = &matches[0];
        assert_eq!(m.pattern_id, "darvo");
        assert_eq!(m.actors_involved, vec!["actor_alex"]);
        assert!(m.raw_confidence >= 0.70);
        assert!(m.explanation.contains("turn 1"));
        assert!(m.explanation.contains("turn 3"));
    }

    #[test]
    fn no_match_when_no_denial() {
        let turns = vec![
            Turn { idx: 0, speaker: Some("Alex".into()), body: "You're putting words in my mouth.".into(), char_offset: 0 },
        ];
        let lex = aco_lex::LexicalFeatures::default();
        let c = ctx(&turns, &lex);
        let detector = DarvoDetector;
        assert!(detector.detect(&c).is_empty());
    }

    #[test]
    fn no_match_when_no_reverse() {
        let turns = vec![
            Turn { idx: 0, speaker: Some("Alex".into()), body: "I never said that.".into(), char_offset: 0 },
            Turn { idx: 1, speaker: Some("Sam".into()), body: "But you did.".into(), char_offset: 0 },
        ];
        let lex = aco_lex::LexicalFeatures::default();
        let c = ctx(&turns, &lex);
        let detector = DarvoDetector;
        assert!(detector.detect(&c).is_empty());
    }

    #[test]
    fn no_match_when_different_speakers_for_deny_and_reverse() {
        let turns = vec![
            Turn { idx: 0, speaker: Some("Alex".into()), body: "I never agreed.".into(), char_offset: 0 },
            Turn { idx: 1, speaker: Some("Sam".into()),  body: "You're putting words in my mouth.".into(), char_offset: 0 },
        ];
        let lex = aco_lex::LexicalFeatures::default();
        let c = ctx(&turns, &lex);
        let detector = DarvoDetector;
        assert!(detector.detect(&c).is_empty(), "different speakers should not match");
    }

    #[test]
    fn confidence_decays_with_gap() {
        let turns = vec![
            Turn { idx: 0, speaker: Some("Alex".into()), body: "I never said that.".into(), char_offset: 0 },
            Turn { idx: 1, speaker: Some("Sam".into()),  body: "okay".into(), char_offset: 0 },
            Turn { idx: 2, speaker: Some("Sam".into()),  body: "okay".into(), char_offset: 0 },
            Turn { idx: 3, speaker: Some("Sam".into()),  body: "okay".into(), char_offset: 0 },
            Turn { idx: 4, speaker: Some("Sam".into()),  body: "okay".into(), char_offset: 0 },
            Turn { idx: 5, speaker: Some("Alex".into()), body: "You're attacking me.".into(), char_offset: 0 },
        ];
        let lex = aco_lex::LexicalFeatures::default();
        let c = ctx(&turns, &lex);
        let m = DarvoDetector.detect(&c);
        assert_eq!(m.len(), 1);
        assert!(m[0].raw_confidence < 0.70, "wide gap should decay confidence");
    }
}
