//! Evidence-span verification. Three modes — Exact, Normalized, Fuzzy.

use tacitus_contracts::EvidenceSpan;

#[derive(Debug, Clone, PartialEq)]
pub enum SpanVerification {
    Exact,
    Normalized,
    Fuzzy { token_distance: usize },
    Failed { reason: String },
}

pub fn verify_span(canonical_text: &str, span: &EvidenceSpan) -> SpanVerification {
    let start = span.char_start_canonical as usize;
    let end = span.char_end_canonical as usize;

    let char_count = canonical_text.chars().count();
    if end > char_count || start >= end {
        return SpanVerification::Failed {
            reason: format!("offsets out of range: {}..{} on text len {}", start, end, char_count),
        };
    }

    let declared_hash = blake3::hash(span.verbatim_quote.as_bytes()).to_hex().to_string();
    if declared_hash != span.quote_hash {
        return SpanVerification::Failed {
            reason: "declared quote hash does not match the displayed quotation".into(),
        };
    }

    // Char-safe slice
    let actual: String = canonical_text.chars().skip(start).take(end - start).collect();
    let actual_hash = blake3::hash(actual.as_bytes()).to_hex().to_string();

    if actual_hash == span.quote_hash {
        return SpanVerification::Exact;
    }
    if actual.trim() == span.verbatim_quote.trim() {
        return SpanVerification::Normalized;
    }

    // Fuzzy: simple token-level Levenshtein.
    let a_tokens: Vec<&str> = actual.split_whitespace().collect();
    let b_tokens: Vec<&str> = span.verbatim_quote.split_whitespace().collect();
    if a_tokens.len() > 4096 || b_tokens.len() > 4096 {
        return SpanVerification::Failed {
            reason: "fuzzy comparison exceeds the bounded diagnostic limit".into(),
        };
    }
    let dist = token_levenshtein(&a_tokens, &b_tokens);
    if dist <= 2 {
        return SpanVerification::Fuzzy { token_distance: dist };
    }

    SpanVerification::Failed { reason: format!("verbatim mismatch (token distance {})", dist) }
}

fn token_levenshtein(a: &[&str], b: &[&str]) -> usize {
    let m = a.len();
    let n = b.len();
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0usize; n + 1];
    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span_for(text: &str, start: u32, end: u32) -> EvidenceSpan {
        let quote: String =
            text.chars().skip(start as usize).take((end - start) as usize).collect();
        EvidenceSpan {
            segment_id: "seg".into(),
            char_start_canonical: start,
            char_end_canonical: end,
            char_start_raw: start,
            char_end_raw: end,
            quote_hash: blake3::hash(quote.as_bytes()).to_hex().to_string(),
            verbatim_quote: quote,
            normalization_version: "0.1.0".into(),
        }
    }

    #[test]
    fn exact_round_trip() {
        let text = "Hello world.";
        let s = span_for(text, 0, 5);
        assert_eq!(verify_span(text, &s), SpanVerification::Exact);
    }

    #[test]
    fn failed_when_offsets_drift() {
        let text = "Hello world.";
        let mut s = span_for(text, 0, 5);
        s.char_end_canonical = 100;
        assert!(matches!(verify_span(text, &s), SpanVerification::Failed { .. }));
    }
    #[test]
    fn changed_display_quote_cannot_reuse_an_exact_hash() {
        let mut s = span_for("Hello world.", 0, 5);
        s.verbatim_quote = "Invented assertion".into();
        assert!(matches!(verify_span("Hello world.", &s), SpanVerification::Failed { .. }));
    }
    #[test]
    fn unicode_bounds_are_code_points_not_byte_length() {
        let text = "😀éA";
        let s = span_for(text, 1, 3);
        assert_eq!(verify_span(text, &s), SpanVerification::Exact);
        let mut out = span_for(text, 0, 3);
        out.char_end_canonical = 4;
        assert!(matches!(verify_span(text, &out), SpanVerification::Failed { .. }));
    }
    #[test]
    fn empty_spans_do_not_certify_evidence() {
        let s = span_for("word", 1, 1);
        assert!(matches!(verify_span("word", &s), SpanVerification::Failed { .. }));
    }
    #[test]
    fn fuzzy_diagnostic_is_not_an_exact_verification() {
        let mut s = span_for("a reported proposal", 0, 19);
        s.verbatim_quote = "a reported agreement".into();
        s.quote_hash = blake3::hash(s.verbatim_quote.as_bytes()).to_hex().to_string();
        assert!(matches!(verify_span("a reported proposal", &s), SpanVerification::Fuzzy { .. }));
    }
}
