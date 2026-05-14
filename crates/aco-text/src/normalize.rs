//! Canonical normalization. NFC + strip bidi controls + strip zero-width.
//! Whitespace canonicalized (runs collapsed to single space; \n preserved).
//!
//! Maintains a `raw_byte_map` so canonical offsets can be projected back to raw.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct RawDocument {
    pub bytes: Vec<u8>,
    pub source_uri: String,
    pub ingest_time: DateTime<Utc>,
}

impl RawDocument {
    pub fn from_str(s: impl Into<String>, source_uri: impl Into<String>) -> Self {
        Self {
            bytes: s.into().into_bytes(),
            source_uri: source_uri.into(),
            ingest_time: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedDocument {
    pub canonical_text: String,
    /// Sparse map: each entry is (canonical_char_idx, raw_byte_idx) at transition points.
    pub raw_byte_map: Vec<(u32, u32)>,
    pub doc_hash: String,
    pub raw_hash: String,
    pub normalization_version: String,
    pub source_uri: String,
}

const BIDI_CONTROLS: &[char] = &[
    '\u{200E}', '\u{200F}', // LRM, RLM
    '\u{202A}', '\u{202B}', '\u{202C}', '\u{202D}', '\u{202E}', // LRE, RLE, PDF, LRO, RLO
    '\u{2066}', '\u{2067}', '\u{2068}', '\u{2069}', // LRI, RLI, FSI, PDI
    '\u{061C}', // ALM
];

const ZERO_WIDTH: &[char] = &[
    '\u{200B}', '\u{200C}', '\u{200D}', // ZWSP, ZWNJ, ZWJ
    '\u{FEFF}', // BOM / ZWNBSP
    '\u{2060}', // word joiner
];

pub fn normalize(raw: &RawDocument) -> NormalizedDocument {
    let raw_text = String::from_utf8_lossy(&raw.bytes);
    let raw_hash = blake3::hash(&raw.bytes).to_hex().to_string();

    // Pass 1: NFC + strip bidi/zero-width. Track raw byte offsets as we go.
    let nfc: String = raw_text.nfc().collect();

    let mut canonical = String::with_capacity(nfc.len());
    let mut raw_byte_map: Vec<(u32, u32)> = Vec::new();
    let mut last_raw_byte: usize = 0;
    let mut last_canonical_char: usize = 0;
    let mut pending_space = false;
    let mut at_line_start = true;

    // Re-walk raw_text bytes & nfc chars in tandem. Because NFC may shift offsets,
    // we approximate: map every emitted canonical char back to a raw byte offset
    // by progressing through `raw_text` matching characters.
    // For v0.1 we record map entries only at "anchors" — char boundaries every 64 chars
    // and at line breaks. Good enough for span-recovery within ±64 chars; refinement
    // for fuzzy verification at PROMPT 11.
    let mut raw_iter = raw_text.char_indices().peekable();

    for (canonical_idx, ch) in nfc.chars().enumerate() {
        // Advance raw_iter to match (approximate; both should be in lock-step for ASCII).
        while let Some(&(idx, c)) = raw_iter.peek() {
            if c == ch || c.is_whitespace() != ch.is_whitespace() {
                last_raw_byte = idx;
                break;
            }
            raw_iter.next();
            last_raw_byte = idx;
        }

        if BIDI_CONTROLS.contains(&ch) || ZERO_WIDTH.contains(&ch) {
            // skip — logged via the difference in canonical_idx vs canonical.len()
            raw_iter.next();
            continue;
        }

        if ch == '\n' {
            // preserve line break; reset whitespace state
            if pending_space {
                pending_space = false; // line break supersedes pending space
            }
            canonical.push('\n');
            at_line_start = true;
        } else if ch.is_whitespace() {
            if !at_line_start {
                pending_space = true;
            }
        } else {
            if pending_space {
                canonical.push(' ');
                pending_space = false;
            }
            canonical.push(ch);
            at_line_start = false;
        }

        // Sparse map anchor every 64 canonical chars.
        if canonical.len().saturating_sub(last_canonical_char) >= 64 {
            raw_byte_map.push((canonical.chars().count() as u32, last_raw_byte as u32));
            last_canonical_char = canonical.len();
        }

        let _ = canonical_idx; // mark used
    }

    // Strip trailing whitespace per line.
    let canonical: String = canonical
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n");

    let doc_hash = blake3::hash(canonical.as_bytes()).to_hex().to_string();

    NormalizedDocument {
        canonical_text: canonical,
        raw_byte_map,
        doc_hash,
        raw_hash,
        normalization_version: super::NORMALIZATION_VERSION.into(),
        source_uri: raw.source_uri.clone(),
    }
}

/// Project canonical character range to raw byte range using the sparse map.
/// Falls back to a 1:1 assumption when no map entries straddle the range.
pub fn raw_offsets(doc: &NormalizedDocument, canonical_start: u32, canonical_end: u32) -> (u32, u32) {
    let find = |c_idx: u32| -> u32 {
        // Find largest map entry with canonical <= c_idx.
        let anchor = doc
            .raw_byte_map
            .iter()
            .rev()
            .find(|(c, _)| *c <= c_idx)
            .copied()
            .unwrap_or((0, 0));
        let (c_anchor, r_anchor) = anchor;
        // Assume 1:1 from anchor (good for ASCII-dominant text).
        r_anchor + (c_idx - c_anchor)
    };
    (find(canonical_start), find(canonical_end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_normalize() {
        let raw = RawDocument::from_str("hello   world", "test://1");
        let doc = normalize(&raw);
        assert_eq!(doc.canonical_text, "hello world");
        assert!(!doc.doc_hash.is_empty());
    }

    #[test]
    fn preserves_line_breaks() {
        let raw = RawDocument::from_str("line one\nline two\n", "test://2");
        let doc = normalize(&raw);
        assert!(doc.canonical_text.contains('\n'));
    }

    #[test]
    fn strips_zero_width() {
        let raw = RawDocument::from_str("hel\u{200B}lo", "test://3");
        let doc = normalize(&raw);
        assert_eq!(doc.canonical_text, "hello");
    }

    #[test]
    fn strips_bidi_controls() {
        let raw = RawDocument::from_str("hi\u{202E}there", "test://4");
        let doc = normalize(&raw);
        assert_eq!(doc.canonical_text, "hithere");
    }

    #[test]
    fn collapses_whitespace_but_keeps_lines() {
        let raw = RawDocument::from_str("a    b\nc   d", "test://5");
        let doc = normalize(&raw);
        assert_eq!(doc.canonical_text, "a b\nc d");
    }

    #[test]
    fn nfc_normalization() {
        // "café" composed (1 char é) vs decomposed (e + combining acute)
        let decomposed = "cafe\u{0301}";
        let composed = "café";
        let d_doc = normalize(&RawDocument::from_str(decomposed, "t"));
        let c_doc = normalize(&RawDocument::from_str(composed, "t"));
        assert_eq!(d_doc.canonical_text, c_doc.canonical_text);
        assert_eq!(d_doc.doc_hash, c_doc.doc_hash);
    }
}
