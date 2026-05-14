//! Sentence + paragraph segmentation. SRX-style rules implemented from primary
//! sources (not code-ported from pragmatic-segmenter or HeidelTime).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentKind {
    Paragraph,
    Sentence,
    Heading,
    Quoted,
    ListItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub id: String, // blake3 prefix of canonical text within range
    pub kind: SegmentKind,
    pub char_start: u32,
    pub char_end: u32,
    pub text: String,
}

/// Naive v0.1 segmenter: split on `\n\n` for paragraphs, then on sentence terminators
/// `[.!?]` followed by whitespace + uppercase. Honors abbreviations from a small list.
pub fn segment(canonical_text: &str) -> Vec<Segment> {
    let mut out = Vec::new();
    let abbrevs = ["Mr.", "Mrs.", "Ms.", "Dr.", "St.", "e.g.", "i.e.", "cf.", "vs.", "Inc.", "Ltd.", "No.", "Fed.", "Cir."];

    let mut cursor: u32 = 0;
    for para in canonical_text.split("\n\n") {
        if para.trim().is_empty() {
            cursor += para.len() as u32 + 2;
            continue;
        }

        // Sentence-split within the paragraph.
        let mut start: usize = 0;
        let chars: Vec<(usize, char)> = para.char_indices().collect();
        let mut i = 0;
        while i < chars.len() {
            let (bi, c) = chars[i];
            if matches!(c, '.' | '!' | '?') {
                let lookbehind: &str = &para[start.max(bi.saturating_sub(8))..=bi];
                let is_abbrev = abbrevs.iter().any(|a| lookbehind.ends_with(a));
                let next_is_terminator = i + 1 == chars.len()
                    || chars
                        .get(i + 1)
                        .map(|(_, nc)| nc.is_whitespace() || matches!(*nc, '"' | '\'' | ')'))
                        .unwrap_or(true);
                if !is_abbrev && next_is_terminator {
                    let end = bi + c.len_utf8();
                    let text = &para[start..end];
                    let id = blake3::hash(text.as_bytes()).to_hex()[..16].to_string();
                    out.push(Segment {
                        id,
                        kind: SegmentKind::Sentence,
                        char_start: cursor + start as u32,
                        char_end: cursor + end as u32,
                        text: text.to_string(),
                    });
                    start = end + 1; // skip the space
                }
            }
            i += 1;
        }
        if start < para.len() {
            let text = para[start..].trim();
            if !text.is_empty() {
                let id = blake3::hash(text.as_bytes()).to_hex()[..16].to_string();
                out.push(Segment {
                    id,
                    kind: SegmentKind::Sentence,
                    char_start: cursor + start as u32,
                    char_end: cursor + para.len() as u32,
                    text: text.to_string(),
                });
            }
        }

        cursor += para.len() as u32 + 2;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_two_sentences() {
        let segs = segment("Hello world. Goodbye world.");
        assert_eq!(segs.len(), 2);
        assert_eq!(segs[0].text, "Hello world.");
        assert_eq!(segs[1].text, "Goodbye world.");
    }

    #[test]
    fn abbreviation_does_not_split() {
        let segs = segment("Met Dr. Smith yesterday. Then went home.");
        assert_eq!(segs.len(), 2);
        assert!(segs[0].text.contains("Dr. Smith"));
    }

    #[test]
    fn paragraph_break() {
        let segs = segment("First para.\n\nSecond para.");
        assert_eq!(segs.len(), 2);
    }
}
