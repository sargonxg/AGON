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
    pub document_profile: DocumentProfile,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FormatHint {
    Dialog,     // most lines have `Speaker: ...` or `Speaker (ts): ...`
    Transcript, // turn: speaker: ... (formal)
    Prose,      // narrative paragraphs
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSegment {
    pub id: String,
    pub kind: String,
    pub label: String,
    pub char_start: usize,
    pub char_end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentProfile {
    pub format: FormatHint,
    pub segments: Vec<DocumentSegment>,
    pub temporal_markers: Vec<String>,
    pub modality_markers: Vec<String>,
    pub conflict_density: f32,
    pub reading_notes: Vec<String>,
    pub candidate_questions: Vec<String>,
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
    } else if turns.len() >= 3
        && speakers_seen
            .iter()
            .any(|s| s.contains('.') || s.chars().all(|c| c.is_uppercase() || c == ' '))
    {
        FormatHint::Transcript
    } else if !turns.is_empty() {
        FormatHint::Dialog
    } else {
        FormatHint::Unknown
    };

    let n_turns = turns.len();
    let mut pc = PreCanonical {
        turns,
        speakers: speakers_seen,
        n_chars,
        n_lines,
        n_turns,
        format_hint,
        document_profile: DocumentProfile {
            format: format_hint,
            segments: Vec::new(),
            temporal_markers: Vec::new(),
            modality_markers: Vec::new(),
            conflict_density: 0.0,
            reading_notes: Vec::new(),
            candidate_questions: Vec::new(),
        },
    };
    pc.document_profile = profile(input, &pc);
    pc
}

/// Render a compact envelope that gets passed to the LLM so it has a
/// deterministic mapping from speaker labels to actor IDs.
pub fn render_envelope(pc: &PreCanonical) -> String {
    if pc.speakers.is_empty() {
        return String::new();
    }
    let mut s = String::from("=== Pre-canonical envelope (deterministic, do not paraphrase) ===\n");
    s.push_str(&format!(
        "format: {:?}\nturns: {}\nchars: {}\nconflict_density: {:.2}\n",
        pc.format_hint, pc.n_turns, pc.n_chars, pc.document_profile.conflict_density
    ));
    s.push_str("speakers (canonical):\n");
    for sp in &pc.speakers {
        let id = canonical_id(sp);
        s.push_str(&format!("  - id={id}  label=\"{sp}\"\n"));
    }
    if !pc.document_profile.temporal_markers.is_empty() {
        s.push_str("temporal markers:\n");
        for marker in pc.document_profile.temporal_markers.iter().take(12) {
            s.push_str(&format!("  - {marker}\n"));
        }
    }
    if !pc.document_profile.reading_notes.is_empty() {
        s.push_str("pre-reading notes:\n");
        for note in pc.document_profile.reading_notes.iter().take(8) {
            s.push_str(&format!("  - {note}\n"));
        }
    }
    s.push_str("=== End envelope ===\n");
    s
}

fn profile(input: &str, pc: &PreCanonical) -> DocumentProfile {
    let mut segments = Vec::new();
    let mut offset = 0usize;
    for (idx, block) in input.split("\n\n").enumerate() {
        let start = offset;
        let end = start + block.chars().count();
        let first = block.lines().next().unwrap_or("").trim();
        let kind = if first.to_ascii_uppercase().contains("DEPOSITION")
            || first.to_ascii_uppercase().contains("INTERVIEW")
            || first.to_ascii_uppercase().contains("NOTE")
            || first.to_ascii_uppercase().contains("EMAIL")
            || first.to_ascii_uppercase().contains("SLACK")
        {
            "source_block"
        } else if pc.format_hint == FormatHint::Dialog {
            "dialog_window"
        } else {
            "paragraph"
        };
        segments.push(DocumentSegment {
            id: format!("seg_{}", idx + 1),
            kind: kind.into(),
            label: if first.is_empty() {
                format!("segment {}", idx + 1)
            } else {
                first.chars().take(72).collect()
            },
            char_start: start,
            char_end: end,
        });
        offset += block.chars().count() + 2;
    }

    let lower = input.to_ascii_lowercase();
    let temporal_markers = collect_markers(
        &lower,
        &[
            "monday",
            "tuesday",
            "wednesday",
            "thursday",
            "friday",
            "saturday",
            "sunday",
            "today",
            "tomorrow",
            "yesterday",
            "before",
            "after",
            "march",
            "april",
            "june",
            "q1",
            "q2",
            "q3",
            "q4",
        ],
    );
    let modality_markers = collect_markers(
        &lower,
        &[
            "never",
            "not",
            "must",
            "should",
            "agreed",
            "approved",
            "denied",
            "claimed",
            "alleges",
            "promised",
            "committed",
            "postponed",
            "cancelled",
            "escalated",
        ],
    );
    let conflict_terms = [
        "never",
        "not",
        "deny",
        "denied",
        "blame",
        "fault",
        "failed",
        "complaint",
        "retaliation",
        "escalated",
        "objected",
        "contradict",
        "bypass",
        "threat",
    ];
    let conflict_hits = conflict_terms.iter().filter(|term| lower.contains(**term)).count();
    let conflict_density = ((conflict_hits as f32 / 10.0) + (pc.n_turns as f32 / 40.0)).min(1.0);

    let mut reading_notes = Vec::new();
    if pc.n_turns >= 3 {
        reading_notes
            .push(format!("{} speaker turns detected before model extraction", pc.n_turns));
    }
    if segments.len() > 1 {
        reading_notes.push(format!(
            "{} source/paragraph segments available for cross-checking",
            segments.len()
        ));
    }
    if !temporal_markers.is_empty() {
        reading_notes.push("temporal language is present; date/order conflicts should be checked deterministically".into());
    }
    if modality_markers.iter().any(|m| m == "never" || m == "not" || m == "denied") {
        reading_notes.push(
            "denial language is present; assertion/denial pairs should be prioritized".into(),
        );
    }

    let mut candidate_questions = Vec::new();
    if !temporal_markers.is_empty() {
        candidate_questions.push(
            "Which sequence of events is contested, and what evidence anchors each date?".into(),
        );
    }
    if modality_markers.iter().any(|m| m == "agreed" || m == "promised" || m == "committed") {
        candidate_questions
            .push("What exact text created or limited the alleged commitment?".into());
    }
    if conflict_density > 0.45 {
        candidate_questions
            .push("Which actor-pair has the strongest friction drivers and why?".into());
    }

    DocumentProfile {
        format: pc.format_hint,
        segments,
        temporal_markers,
        modality_markers,
        conflict_density,
        reading_notes,
        candidate_questions,
    }
}

fn collect_markers(text: &str, markers: &[&str]) -> Vec<String> {
    markers
        .iter()
        .filter(|marker| text.contains(**marker))
        .map(|marker| (*marker).to_string())
        .collect()
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
        assert!(pc.document_profile.conflict_density > 0.0);
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
