//! Schema loader. Each schema is bundled at compile time via `include_str!`.
//!
//! Cross-schema `$ref`s to `evidence_span.json` are rewritten to local
//! `#/$defs/evidence_span` and the EvidenceSpan schema is injected as a
//! `$defs` entry. This avoids HTTP fetches during validation.

use jsonschema::Validator;
use serde_json::{json, Value};
use std::sync::OnceLock;

const EVIDENCE_SPAN: &str = include_str!("../schemas/evidence_span.json");

fn build(file_content: &str) -> Validator {
    let mut root: Value = serde_json::from_str(file_content).expect("schema is not valid JSON");
    let es: Value = serde_json::from_str(EVIDENCE_SPAN).expect("evidence_span.json is not valid JSON");

    // Inject EvidenceSpan as a local $def.
    if let Some(obj) = root.as_object_mut() {
        let defs = obj
            .entry("$defs".to_string())
            .or_insert_with(|| json!({}));
        if let Some(d) = defs.as_object_mut() {
            d.insert("evidence_span".into(), es);
        }
    }

    // Rewrite external refs to local.
    rewrite_refs(&mut root);

    jsonschema::validator_for(&root).expect("not a valid JSON Schema")
}

fn rewrite_refs(v: &mut Value) {
    match v {
        Value::Object(map) => {
            if let Some(r) = map.get_mut("$ref") {
                if let Some(s) = r.as_str() {
                    if s == "https://tacitus.me/schemas/v0/evidence_span.json" {
                        *r = json!("#/$defs/evidence_span");
                    }
                }
            }
            for (_, child) in map.iter_mut() {
                rewrite_refs(child);
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                rewrite_refs(item);
            }
        }
        _ => {}
    }
}

macro_rules! schema {
    ($name:ident, $file:literal) => {
        pub fn $name() -> &'static Validator {
            static CELL: OnceLock<Validator> = OnceLock::new();
            CELL.get_or_init(|| build(include_str!(concat!("../schemas/", $file))))
        }
    };
}

schema!(evidence_span, "evidence_span.json");
schema!(actor, "actor.json");
schema!(claim, "claim.json");
schema!(event, "event.json");
schema!(commitment, "commitment.json");
schema!(contradiction, "contradiction.json");
schema!(pattern_match, "pattern_match.json");
schema!(provenance, "provenance.json");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_schemas_load() {
        let _ = evidence_span();
        let _ = actor();
        let _ = claim();
        let _ = event();
        let _ = commitment();
        let _ = contradiction();
        let _ = pattern_match();
        let _ = provenance();
    }
}
