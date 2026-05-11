//! `aco-fuse` — canonicalization layer.
#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use tracing::trace;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalActor {
    pub canonical_label: String,
    pub normalized_key: String,
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ActorCanonicalizer {
    aliases: BTreeMap<String, String>,
}

impl ActorCanonicalizer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_alias(&mut self, canonical_label: &str, alias: &str) {
        let canonical_key = normalize_actor_alias(canonical_label);
        for value in [canonical_label, alias] {
            let key = normalize_actor_alias(value);
            if !key.is_empty() {
                self.aliases.insert(key, canonical_key.clone());
            }
        }
    }

    pub fn canonicalize<I, S>(&self, labels: I) -> Vec<CanonicalActor>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut groups: BTreeMap<String, CanonicalActor> = BTreeMap::new();
        for label in labels {
            let label = label.as_ref().trim();
            if label.is_empty() {
                continue;
            }
            let normalized = normalize_actor_alias(label);
            if normalized.is_empty() {
                continue;
            }
            let key = self.aliases.get(&normalized).cloned().unwrap_or(normalized);
            let entry = groups.entry(key.clone()).or_insert_with(|| CanonicalActor {
                canonical_label: titleish(label),
                normalized_key: key,
                aliases: Vec::new(),
            });
            if !entry.aliases.iter().any(|a| a.eq_ignore_ascii_case(label)) {
                entry.aliases.push(label.to_string());
            }
        }
        groups.into_values().collect()
    }
}

pub fn normalize_actor_alias(alias: &str) -> String {
    let lower = alias
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
        .collect::<String>();
    lower
        .split_whitespace()
        .filter(|part| !matches!(*part, "mr" | "mrs" | "ms" | "miss" | "dr" | "prof" | "the"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn titleish(label: &str) -> String {
    label
        .split_whitespace()
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn init() {
    trace!(crate_name = "aco-fuse", "loaded");
}

#[cfg(test)]
mod tests {
    use super::{normalize_actor_alias, ActorCanonicalizer};

    #[test]
    fn normalizes_honorifics_case_and_punctuation() {
        assert_eq!(normalize_actor_alias("Mr. John Doe"), "john doe");
        assert_eq!(normalize_actor_alias("THE PLAINTIFF"), "plaintiff");
        assert_eq!(normalize_actor_alias("Doe, John"), "doe john");
    }

    #[test]
    fn merges_only_explicit_aliases_conservatively() {
        let mut c = ActorCanonicalizer::new();
        c.add_alias("John Doe", "Doe");
        c.add_alias("John Doe", "Mr. Doe");
        c.add_alias("John Doe", "the plaintiff");

        let actors = c.canonicalize(["John Doe", "Doe", "Mr. Doe", "the plaintiff", "Jane Doe"]);

        assert_eq!(actors.len(), 2);
        assert!(actors.iter().any(|a| a.aliases.len() == 4));
        assert!(actors.iter().any(|a| a.normalized_key == "jane doe"));
    }
}
