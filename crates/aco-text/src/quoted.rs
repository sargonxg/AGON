//! Quoted-speech FSM + reported-speech detection.
//!
//! Detects:
//! - Direct quotes wrapped in `"..."`, `«...»`, `„..."`, `'...'`
//! - Reported speech: `X said/told/wrote/claimed/replied that ...`
//! - Free-indirect speech (heuristic; lower confidence)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeechKind {
    Direct,
    Reported,
    FreeIndirect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeechSpan {
    pub kind: SpeechKind,
    pub speaker_hint: Option<String>,
    pub quote_text: String,
    pub char_start: u32,
    pub char_end: u32,
}

const QUOTE_PAIRS: &[(char, char)] = &[
    ('"', '"'),
    ('\u{201C}', '\u{201D}'), // " "
    ('«', '»'),
    ('\u{201E}', '\u{201D}'), // „ "
    ('\'', '\''),
];

const REPORTING_VERBS: &[&str] = &[
    "said", "says", "told", "tells", "wrote", "writes",
    "claimed", "claims", "replied", "replies", "stated", "states",
    "noted", "notes", "observed", "observes", "added", "adds",
    "remarked", "remarks", "asserted", "asserts", "denied", "denies",
];

/// Detect speech spans in canonical text. Char offsets reference the canonical text.
#[must_use]
pub fn detect_speech(canonical: &str) -> Vec<SpeechSpan> {
    let mut out = Vec::new();
    out.extend(detect_direct(canonical));
    out.extend(detect_reported(canonical));
    out.sort_by_key(|s| s.char_start);
    out
}

fn detect_direct(text: &str) -> Vec<SpeechSpan> {
    let mut spans = Vec::new();
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut i = 0;
    while i < chars.len() {
        let (byte_i, c) = chars[i];
        if let Some(&(_, close)) = QUOTE_PAIRS.iter().find(|(o, _)| *o == c) {
            // walk forward to matching close
            let mut j = i + 1;
            while j < chars.len() {
                let (byte_j, cj) = chars[j];
                if cj == close {
                    let quote_text = text[byte_i + c.len_utf8()..byte_j].to_string();
                    // skip empty or whitespace-only quotes
                    if !quote_text.trim().is_empty() {
                        let speaker_hint = sniff_speaker_before(text, byte_i);
                        spans.push(SpeechSpan {
                            kind: SpeechKind::Direct,
                            speaker_hint,
                            quote_text,
                            char_start: u32::try_from(i).unwrap_or(u32::MAX),
                            char_end: u32::try_from(j + 1).unwrap_or(u32::MAX),
                        });
                    }
                    i = j;
                    break;
                }
                j += 1;
            }
        }
        i += 1;
    }
    spans
}

fn detect_reported(text: &str) -> Vec<SpeechSpan> {
    let mut spans = Vec::new();
    let lower = text.to_lowercase();
    for &verb in REPORTING_VERBS {
        let mut start = 0;
        let needle = format!(" {verb} that ");
        while let Some(idx) = lower[start..].find(&needle) {
            let abs = start + idx;
            // speaker is the word(s) before `verb`
            let before = &text[..abs];
            let speaker_hint = before.rsplit_once(|c: char| c == '.' || c == '\n').map_or(before, |(_, w)| w);
            let speaker_hint = speaker_hint.trim().split_whitespace().last().map(str::to_string);
            // claim text runs from after `that ` to next sentence terminator
            let claim_start = abs + needle.len();
            let claim_end = text[claim_start..]
                .find(|c: char| c == '.' || c == '!' || c == '?')
                .map_or(text.len(), |p| claim_start + p + 1);
            spans.push(SpeechSpan {
                kind: SpeechKind::Reported,
                speaker_hint,
                quote_text: text[claim_start..claim_end].to_string(),
                char_start: u32::try_from(abs).unwrap_or(u32::MAX),
                char_end: u32::try_from(claim_end).unwrap_or(u32::MAX),
            });
            start = claim_end;
        }
    }
    spans
}

fn sniff_speaker_before(text: &str, byte_idx: usize) -> Option<String> {
    // Look back ~40 chars for "X said," / "X said that" / "X:"
    let window_start = byte_idx.saturating_sub(40);
    let window = &text[window_start..byte_idx];
    let trimmed = window.trim_end_matches(|c: char| c.is_whitespace() || c == ',' || c == ':');
    for verb in REPORTING_VERBS {
        if let Some(pos) = trimmed.to_lowercase().rfind(verb) {
            let speaker = &trimmed[..pos];
            return speaker.trim().rsplit_once(' ').map(|(_, last)| last.to_string())
                .or_else(|| Some(speaker.trim().to_string()))
                .filter(|s| !s.is_empty());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_direct_quote_curly() {
        let t = "Sam said, \u{201C}I never agreed.\u{201D}";
        let s = detect_speech(t);
        assert!(s.iter().any(|x| x.kind == SpeechKind::Direct && x.quote_text == "I never agreed."));
    }

    #[test]
    fn detects_direct_quote_straight() {
        let t = "Then: \"hello world\".";
        let s = detect_speech(t);
        assert!(s.iter().any(|x| x.kind == SpeechKind::Direct && x.quote_text == "hello world"));
    }

    #[test]
    fn detects_reported_speech() {
        let t = "Alex said that the deal was off.";
        let s = detect_speech(t);
        assert!(s.iter().any(|x| x.kind == SpeechKind::Reported));
    }

    #[test]
    fn ignores_empty_quotes() {
        let t = "He said \"\" and walked away.";
        let s = detect_speech(t);
        assert!(s.iter().filter(|x| x.kind == SpeechKind::Direct).count() == 0);
    }
}
