//! Common types: provenance, evidence spans, temporal intervals, place.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{enums::DefeasibilityClass, id::Id};

/// Half-open character span `[char_start, char_end)` into a source chunk.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EvidenceSpan {
    /// Chunk id (FK into the `chunks` table).
    pub chunk_id: String,
    /// Inclusive start (UTF-8 char index).
    pub char_start: u32,
    /// Exclusive end (UTF-8 char index).
    pub char_end: u32,
    /// Verbatim quoted text. Must match `chunk.text[char_start..char_end]`.
    pub quote: String,
}

/// Where a primitive came from in the pipeline.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Derivation {
    /// Extracted directly from a source chunk by a named extractor.
    Extracted {
        /// Extractor module name (e.g., `"entity"`, `"event"`).
        extractor: String,
    },
    /// Result of fusing two or more raw primitives.
    Fused {
        /// Ids of the source primitives that were merged.
        sources: Vec<Id>,
    },
    /// Derived by a deductive rule.
    Inferred {
        /// Datalog / SMT / LP rule name.
        rule: String,
        /// Parent primitive ids the rule consumed.
        parents: Vec<Id>,
    },
    /// Abductive hypothesis from a gap-driven prompt.
    Abduced {
        /// The gap kind that motivated the hypothesis.
        gap: String,
        /// Parent primitive ids the abduction considered.
        parents: Vec<Id>,
    },
}

/// Defeasibility annotation attached to every provenance.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Defeasibility {
    /// Class: strict / defeasible / presumption.
    pub class: DefeasibilityClass,
    /// Free-form justification for the class assignment.
    pub justification: Option<String>,
}

/// Full provenance record. Attached to every primitive.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Provenance {
    /// Tagged source: extractor name, fusion engine, rule name, etc.
    pub extractor: String,
    /// Versioned prompt identifier (`"entity_v1"`).
    pub prompt_version: String,
    /// 16-byte fingerprint of the exact prompt template used.
    #[serde(with = "fingerprint_hex")]
    pub prompt_fingerprint: [u8; 16],
    /// Verbatim evidence spans grounding this primitive.
    pub spans: Vec<EvidenceSpan>,
    /// Extractor confidence in [0, 1].
    pub confidence: f32,
    /// When the primitive was created.
    pub created_at: DateTime<Utc>,
    /// Defeasibility annotation.
    pub defeasibility: Defeasibility,
    /// Derivation chain.
    pub derivation: Derivation,
}

/// A temporal interval. Either bounded, half-open, instantaneous, or unknown.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemporalInterval {
    /// Closed `[start, end]` interval.
    Closed {
        /// Start (inclusive).
        start: DateTime<Utc>,
        /// End (inclusive).
        end: DateTime<Utc>,
    },
    /// Open-ended after a point.
    After {
        /// Lower bound.
        start: DateTime<Utc>,
    },
    /// Open-ended before a point.
    Before {
        /// Upper bound.
        end: DateTime<Utc>,
    },
    /// Single instant.
    At {
        /// The instant.
        when: DateTime<Utc>,
    },
    /// No anchored time (relative reference that could not be resolved).
    Unknown,
}

/// Place identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Place {
    /// Canonical name.
    pub name: String,
    /// ISO 3166-1 alpha-2 country code if known.
    pub iso_country: Option<String>,
}

mod fingerprint_hex {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(b: &[u8; 16], ser: S) -> Result<S::Ok, S::Error> {
        let mut out = String::with_capacity(32);
        for x in b {
            out.push_str(&format!("{x:02x}"));
        }
        ser.serialize_str(&out)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<[u8; 16], D::Error> {
        use serde::de::Error as _;
        let s = <&str>::deserialize(de)?;
        if s.len() != 32 {
            return Err(D::Error::custom(format!("expected 32 hex chars, got {}", s.len())));
        }
        let mut out = [0u8; 16];
        for (i, chunk) in s.as_bytes().chunks(2).enumerate() {
            let nib = |b: u8| match b {
                b'0'..=b'9' => Ok(b - b'0'),
                b'a'..=b'f' => Ok(b - b'a' + 10),
                b'A'..=b'F' => Ok(b - b'A' + 10),
                _ => Err(D::Error::custom("non-hex")),
            };
            out[i] = (nib(chunk[0])? << 4) | nib(chunk[1])?;
        }
        Ok(out)
    }
}
