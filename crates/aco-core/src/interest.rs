//! `Interest` — an underlying goal or need that motivates positions.

use serde::{Deserialize, Serialize};

use crate::{common::Provenance, enums::InterestCategory, id::Id};

/// An interest held by an actor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Interest {
    /// Content-addressed id.
    pub id: Id,
    /// Who holds the interest (canonical actor id).
    pub holder: Id,
    /// Natural-language description.
    pub description: String,
    /// Category.
    pub category: InterestCategory,
    /// Priority in [0, 1] inferred from text / position.
    pub priority: f32,
    /// Whether the interest was stated explicitly (true) or inferred (false).
    pub stated: bool,
    /// Optional numeric utility proxy (used by the BATNA/ZOPA layer).
    pub utility_proxy: Option<f32>,
    /// Provenance.
    pub prov: Provenance,
}
