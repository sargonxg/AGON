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

    if end > canonical_text.len() || start > end {
        return SpanVerification::Failed {
            reason: format!("offsets out of range: {}..{} on text len {}", start, end, canonical_text.len()),
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
    let dist = token_levenshtein(&a_tokens, &b_tokens);
    if dist <= 2 {
        return SpanVerification::Fuzzy { token_distance: dist };
    }

    SpanVerification::Failed {
        reason: format!("verbatim mismatch (token distance {})", dist),
    }
}

fn token_levenshtein(a: &[&str], b: &[&str]) -> usize {
    let m = a.len();
    let n = b.len();
    if m == 0 { return n; }
    if n == 0 { return m; }
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0usize; n + 1];
    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1)
                .min(curr[j - 1] + 1)
                .min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span_for(text: &str, start: u32, end: u32) -> EvidenceSpan {
        let quote: String = text.chars().skip(start as usize).take((end - start) as usize).collect();
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
}
