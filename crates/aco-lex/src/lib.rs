//! aco-lex — deterministic lexical feature extractors.
//!
//! v0.1: hedging density + epistemic modality + passive-voice flag (EN only).
//! Full multilingual + face-work + register-shift in PROMPT 04 proper.
#![forbid(unsafe_code)]

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LexicalFeatures {
    pub hedge_count: u32,
    pub strong_hedges: u32,
    pub modal_count: u32,
    pub passive_voice_hits: u32,
    pub first_person_singular: u32,
    pub first_person_plural: u32,
    pub second_person: u32,
    pub third_person: u32,
}

const STRONG_HEDGES_EN: &[&str] = &["clearly", "obviously", "definitely", "certainly", "undoubtedly"];
const MODERATE_HEDGES_EN: &[&str] = &["probably", "likely", "presumably", "apparently"];
const WEAK_HEDGES_EN: &[&str] = &["perhaps", "possibly", "maybe", "arguably", "somewhat"];
const MODALS_EN: &[&str] = &["must", "should", "could", "might", "may", "would", "shall", "will"];

static PASSIVE_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(was|were|been|being|is|are|am)\s+(\w+ed|made|done|taken|given|seen|said|told)\b")
        .expect("PASSIVE_RX is a valid regex")
});

/// Extract English lexical features. Pure function; safe to call from many threads.
#[must_use]
pub fn extract_en(text: &str) -> LexicalFeatures {
    let lower = text.to_lowercase();
    let mut f = LexicalFeatures::default();

    for &w in STRONG_HEDGES_EN {
        f.strong_hedges += count_word(&lower, w);
    }
    for &w in MODERATE_HEDGES_EN.iter().chain(WEAK_HEDGES_EN.iter()) {
        f.hedge_count += count_word(&lower, w);
    }
    f.hedge_count += f.strong_hedges;

    for &w in MODALS_EN {
        f.modal_count += count_word(&lower, w);
    }

    f.passive_voice_hits = u32::try_from(PASSIVE_RX.find_iter(&lower).count()).unwrap_or(u32::MAX);

    f.first_person_singular = count_word(&lower, "i") + count_word(&lower, "me") + count_word(&lower, "my");
    f.first_person_plural = count_word(&lower, "we") + count_word(&lower, "us") + count_word(&lower, "our");
    f.second_person = count_word(&lower, "you") + count_word(&lower, "your");
    f.third_person = count_word(&lower, "he") + count_word(&lower, "she") + count_word(&lower, "they");

    f
}

fn count_word(haystack: &str, word: &str) -> u32 {
    use std::collections::HashMap;
    use std::sync::RwLock;

    static CACHE: Lazy<RwLock<HashMap<String, Regex>>> =
        Lazy::new(|| RwLock::new(HashMap::new()));

    // Fast path: cached compiled regex.
    if let Some(rx) = CACHE.read().expect("cache poisoned").get(word) {
        return u32::try_from(rx.find_iter(haystack).count()).unwrap_or(u32::MAX);
    }
    // Slow path: compile once + insert.
    let rx = Regex::new(&format!(r"\b{}\b", regex::escape(word)))
        .expect("count_word regex compile");
    let count = u32::try_from(rx.find_iter(haystack).count()).unwrap_or(u32::MAX);
    CACHE.write().expect("cache poisoned").insert(word.to_string(), rx);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_hedges() {
        let f = extract_en("This is clearly important, though perhaps not urgent.");
        assert!(f.strong_hedges >= 1);
        assert!(f.hedge_count >= 2);
    }

    #[test]
    fn detects_modals() {
        let f = extract_en("We should ship. They might delay. He must agree.");
        assert!(f.modal_count >= 3);
    }

    #[test]
    fn detects_passive() {
        let f = extract_en("Mistakes were made. The decision was taken.");
        assert!(f.passive_voice_hits >= 2);
    }
}
