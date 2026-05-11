//! Stable, content-addressed identifiers for every ACO primitive.
//!
//! `Id` is a thin wrapper around a 32-byte BLAKE3 hash. `Id::from_canonical`
//! must be stable across runs: the *same* canonical input always yields the
//! same id. This is the foundation of the audit trail — every conclusion the
//! system reaches can be traced back to its parents by id.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

/// 256-bit content-addressed identifier.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(#[serde(with = "hex")] pub [u8; 32]);

impl Id {
    /// Derive an id from any [`Canonical`] input.
    pub fn from_canonical<C: Canonical + ?Sized>(value: &C) -> Self {
        let mut hasher = blake3::Hasher::new();
        value.canonical_into(&mut hasher);
        Self(*hasher.finalize().as_bytes())
    }

    /// Derive an id from raw bytes (escape hatch for non-`Canonical` payloads).
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(*blake3::hash(bytes).as_bytes())
    }

    /// Hex string representation (used as the Postgres PK).
    pub fn to_hex(&self) -> String {
        let mut out = String::with_capacity(64);
        for b in self.0 {
            out.push_str(&format!("{b:02x}"));
        }
        out
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({}…)", &self.to_hex()[..12])
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl FromStr for Id {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 64 {
            return Err(crate::error::Error::BadId(format!("expected 64 hex chars, got {}", s.len())));
        }
        let mut out = [0u8; 32];
        for (i, chunk) in s.as_bytes().chunks(2).enumerate() {
            let hi = hex_nibble(chunk[0])?;
            let lo = hex_nibble(chunk[1])?;
            out[i] = (hi << 4) | lo;
        }
        Ok(Id(out))
    }
}

fn hex_nibble(b: u8) -> Result<u8, crate::error::Error> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err(crate::error::Error::BadId(format!("non-hex byte {b}"))),
    }
}

/// Anything that can produce a stable canonical byte stream for hashing.
///
/// Implementations must be deterministic — equal logical values produce equal
/// canonical streams across processes and Rust versions.
pub trait Canonical {
    /// Feed a stable, side-effect-free canonical representation into `hasher`.
    fn canonical_into(&self, hasher: &mut blake3::Hasher);
}

impl Canonical for str {
    fn canonical_into(&self, hasher: &mut blake3::Hasher) {
        hasher.update(self.as_bytes());
    }
}

impl Canonical for String {
    fn canonical_into(&self, hasher: &mut blake3::Hasher) {
        hasher.update(self.as_bytes());
    }
}

impl<T: Canonical + ?Sized> Canonical for &T {
    fn canonical_into(&self, hasher: &mut blake3::Hasher) {
        (*self).canonical_into(hasher)
    }
}

mod hex {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &[u8; 32], ser: S) -> Result<S::Ok, S::Error> {
        let mut out = String::with_capacity(64);
        for b in bytes {
            out.push_str(&format!("{b:02x}"));
        }
        ser.serialize_str(&out)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<[u8; 32], D::Error> {
        use serde::de::Error as _;
        let s = <&str>::deserialize(de)?;
        let id = super::Id::from_str(s).map_err(D::Error::custom)?;
        Ok(id.0)
    }
    use std::str::FromStr;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determinism() {
        let a = Id::from_canonical("Sarah Chen");
        let b = Id::from_canonical("Sarah Chen");
        assert_eq!(a, b);
    }

    #[test]
    fn different_inputs_differ() {
        let a = Id::from_canonical("Sarah Chen");
        let b = Id::from_canonical("sarah chen");
        assert_ne!(a, b);
    }

    #[test]
    fn hex_roundtrip() {
        let id = Id::from_canonical("test");
        let hex = id.to_hex();
        let back: Id = hex.parse().unwrap();
        assert_eq!(id, back);
    }
}
