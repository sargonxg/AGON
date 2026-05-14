//! Speaker-turn detection.
//!
//! Recognizes:
//! - Deposition-style:  `Q:`, `A:`, `MR. SMITH:`, `THE WITNESS:`
//! - Email thread headers: `From: ... \n Sent: ... \n To: ... \n Subject: ...`
//! - Slack-style: `[12:34 PM] Alex Chen:`

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerTurn {
    pub speaker_surface: String,
    pub kind: TurnKind,
    pub char_start: u32,
    pub char_end: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TurnKind {
    Deposition,
    Email,
    Chat,
}

static DEPOSITION_RX: Lazy<Regex> = Lazy::new(|| {
    // Matches Q:, A:, THE WITNESS:, THE COURT:, MR. SMITH:, MS. DOE: etc.
    Regex::new(r"(?m)^(?:Q|A|THE\s+[A-Z]+|(?:MR|MS|MRS|DR)\.\s*[A-Z]+):")
        .expect("DEPOSITION_RX valid")
});

static EMAIL_HEADER_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?im)^From:\s*([^\n<]+?)(?:\s*<[^>]+>)?\s*$")
        .expect("EMAIL_HEADER_RX valid")
});

static SLACK_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\[(?:\d{1,2}:\d{2}(?:\s*[AP]M)?)\]\s+([\w\s\.]+?):")
        .expect("SLACK_RX valid")
});

#[must_use]
pub fn detect_turns(canonical: &str) -> Vec<SpeakerTurn> {
    let mut out = Vec::new();

    for m in DEPOSITION_RX.find_iter(canonical) {
        let surface = m.as_str().trim_end_matches(':').trim().to_string();
        out.push(SpeakerTurn {
            speaker_surface: surface,
            kind: TurnKind::Deposition,
            char_start: u32::try_from(m.start()).unwrap_or(u32::MAX),
            char_end: u32::try_from(m.end()).unwrap_or(u32::MAX),
        });
    }

    for cap in EMAIL_HEADER_RX.captures_iter(canonical) {
        let m = cap.get(0).unwrap();
        let sender = cap.get(1).map(|x| x.as_str().trim().to_string()).unwrap_or_default();
        if !sender.is_empty() {
            out.push(SpeakerTurn {
                speaker_surface: sender,
                kind: TurnKind::Email,
                char_start: u32::try_from(m.start()).unwrap_or(u32::MAX),
                char_end: u32::try_from(m.end()).unwrap_or(u32::MAX),
            });
        }
    }

    for cap in SLACK_RX.captures_iter(canonical) {
        let m = cap.get(0).unwrap();
        let sender = cap.get(1).map(|x| x.as_str().trim().to_string()).unwrap_or_default();
        if !sender.is_empty() {
            out.push(SpeakerTurn {
                speaker_surface: sender,
                kind: TurnKind::Chat,
                char_start: u32::try_from(m.start()).unwrap_or(u32::MAX),
                char_end: u32::try_from(m.end()).unwrap_or(u32::MAX),
            });
        }
    }

    out.sort_by_key(|t| t.char_start);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposition_q_a() {
        let t = "Q: What did you see?\nA: A red car.\nQ: At what time?";
        let turns = detect_turns(t);
        assert!(turns.len() >= 3);
        assert!(turns.iter().all(|x| x.kind == TurnKind::Deposition));
    }

    #[test]
    fn deposition_named() {
        let t = "MR. SMITH: I object.\nTHE COURT: Overruled.";
        let turns = detect_turns(t);
        assert!(turns.iter().any(|t| t.speaker_surface.contains("SMITH")));
        assert!(turns.iter().any(|t| t.speaker_surface.contains("COURT")));
    }

    #[test]
    fn email_header() {
        let t = "From: Alex Chen <alex@example.com>\nSent: Monday\nSubject: re: deck";
        let turns = detect_turns(t);
        assert!(turns.iter().any(|t| t.speaker_surface == "Alex Chen" && t.kind == TurnKind::Email));
    }

    #[test]
    fn slack_style() {
        let t = "[12:34 PM] Alex Chen: I never agreed to that.";
        let turns = detect_turns(t);
        assert!(turns.iter().any(|t| t.speaker_surface == "Alex Chen" && t.kind == TurnKind::Chat));
    }
}
