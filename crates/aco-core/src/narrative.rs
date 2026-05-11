//! `Narrative` — a coherent framing across multiple claims.

use serde::{Deserialize, Serialize};

use crate::{common::Provenance, id::Id};

/// A narrative (frame) attributable to an actor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Narrative {
    /// Content-addressed id.
    pub id: Id,
    /// Author of the narrative.
    pub author: Id,
    /// Short frame label (e.g., `"betrayed-employee"`).
    pub frame: String,
    /// Claim ids that compose the narrative.
    pub claims: Vec<Id>,
    /// Event ids the narrative references.
    pub events: Vec<Id>,
    /// Cast: villain (id).
    pub villain: Option<Id>,
    /// Cast: hero (id).
    pub hero: Option<Id>,
    /// Cast: victim (id).
    pub victim: Option<Id>,
    /// Internal coherence in [0, 1] — how consistently the claims fit the frame.
    pub coherence: f32,
    /// Provenance.
    pub prov: Provenance,
}
