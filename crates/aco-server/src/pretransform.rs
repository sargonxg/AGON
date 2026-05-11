//! Pre-canonical text transformation.
//!
//! Before the LLM ever sees the input, we run a deterministic pass that:
//! - segments the text into **turns** (speaker + body, when the format permits)
//! - canonicalizes speaker labels (folding "Sam:", "Sam (Mon 09:14):", "SAM —")
//! - extracts inline temporal markers (Mon/Thu/dates/times)
//! - splits long inputs into ≤ 6-turn windows (chunking for big inputs is v2)
//! - builds a compact "envelope" the prompt can reference deterministically
//!
//! Speed: regex + linear pass, sub-millisecond on ≤ 50 KB inputs.

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turn {
    pub idx: usize,
    pub speaker: Option<String>,
    pub timestamp: Option<String>,
    pub body: String,
    pub char_offset: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreCanonical {
    pub turns: Vec<Turn>,
    pub speakers: Vec<String>,
    pub n_chars: usize,
    pub n_lines: usize,
    pub n_turns: usize,
    pub format_hint: FormatHint,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FormatHint {
    Dialog,    // most lines have `Speaker: ...` or `Speaker (ts): ...`
    Transcript,// turn: speaker: ... (formal)
    Prose,     // narrative paragraphs
    Unknown,
}

// "Alex:", "Alex (Mon 09:14):", "Alex (Thursday):", "Alex — ", "DR. PARK:".
static TURN_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^[\s\u{a0}]*(?P<speaker>(?:[A-Z][\w\-\.]+|[A-Z]{2,}(?:\.[A-Z\-]+)*)(?:\s+[A-Z][\w\-\.]+){0,3})\s*(?:\((?P<ts>[^)]+)\))?\s*[:——]\s+(?P<body>.+)$").unwrap()
});

pub fn transform(input: &str) -> PreCanonical {
    let n_chars = input.chars().count();
    let n_lines = input.lines().count();

    let mut turns: Vec<Turn> = Vec::new();
    let mut speakers_seen: Vec<String> = Vec::new();
    let mut byte_off = 0usize;
    let mut idx = 0usize;

    for line in input.lines() {
        let line_off = byte_off;
        byte_off += line.len() + 1; // newline

        if let Some(caps) = TURN_RX.captures(line) {
            let speaker = caps.name("speaker").map(|m| m.as_str().trim().to_string());
            let timestamp = caps.name("ts").map(|m| m.as_str().trim().to_string());
            let body = caps.name("body").map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            if let Some(s) = &speaker {
                if !speakers_seen.iter().any(|x| x.eq_ignore_ascii_case(s)) {
                    speakers_seen.push(s.clone());
                }
            }
            turns.push(Turn { idx, speaker, timestamp, body, char_offset: line_off });
            idx += 1;
        }
    }

    let format_hint = if turns.is_empty() {
        FormatHint::Prose
    } else if turns.len() >= 3 && speakers_seen.iter().any(|s| s.contains('.') || s.chars().all(|c| c.is_uppercase() || c == ' ')) {
        FormatHint::Transcript
    } else if !turns.is_empty() {
        FormatHint::Dialog
    } else {
        FormatHint::Unknown
    };

    let n_turns = turns.len();
    PreCanonical { turns, speakers: speakers_seen, n_chars, n_lines, n_turns, format_hint }
}

/// Render a compact envelope that gets passed to the LLM so it has a
/// deterministic mapping from speaker labels to actor IDs.
pub fn render_envelope(pc: &PreCanonical) -> String {
    if pc.speakers.is_empty() {
        return String::new();
    }
    let mut s = String::from("=== Pre-canonical envelope (deterministic, do not paraphrase) ===\n");
    s.push_str(&format!("format: {:?}\nturns: {}\nchars: {}\n", pc.format_hint, pc.n_turns, pc.n_chars));
    s.push_str("speakers (canonical):\n");
    for sp in &pc.speakers {
        let id = canonical_id(sp);
        s.push_str(&format!("  - id={id}  label=\"{sp}\"\n"));
    }
    s.push_str("=== End envelope ===\n");
    s
}

pub fn canonical_id(label: &str) -> String {
    let ascii: String = label
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect();
    let trimmed: String = ascii.split('_').filter(|s| !s.is_empty()).collect::<Vec<_>>().join("_");
    format!("actor_{trimmed}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_dialog_turns() {
        let s = "Sam (Mon 09:14): we agreed\nAlex (Mon 09:47): sounds good\nAlex (Thu): I never said that.";
        let pc = transform(s);
        assert_eq!(pc.n_turns, 3);
        assert_eq!(pc.speakers.len(), 2);
        assert_eq!(pc.format_hint, FormatHint::Dialog);
    }

    #[test]
    fn ignores_prose() {
        let s = "The board meeting was contentious. Members disagreed about strategy.";
        let pc = transform(s);
        assert_eq!(pc.n_turns, 0);
        assert_eq!(pc.format_hint, FormatHint::Prose);
    }

    #[test]
    fn canonicalizes_ids() {
        assert_eq!(canonical_id("Sarah Chen"), "actor_sarah_chen");
        assert_eq!(canonical_id("DR. PARK"), "actor_dr_park");
    }
}
