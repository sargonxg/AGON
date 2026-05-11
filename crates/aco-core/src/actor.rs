//! `Actor` — any party capable of holding an interest or making a claim.

use serde::{Deserialize, Serialize};

use crate::{common::Provenance, enums::ActorKind, id::Id};

/// Canonical actor. Aliases are stored separately in the `actor_aliases` edge
/// table; this struct holds only the canonical form.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Actor {
    /// Content-addressed id (hash of canonical name + kind).
    pub id: Id,
    /// Canonical display name (post-normalisation).
    pub canonical_name: String,
    /// All known aliases seen across documents.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// What kind of party.
    pub kind: ActorKind,
    /// Roles this actor has held in this corpus (e.g., `"manager"`, `"plaintiff"`).
    #[serde(default)]
    pub roles: Vec<String>,
    /// Agency score in [0, 1] — how much agency / capability the actor exhibits.
    pub agency_score: f32,
    /// Provenance.
    pub prov: Provenance,
}
