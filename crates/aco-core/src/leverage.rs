//! `Leverage` — a resource or capability that shifts bargaining power.

use serde::{Deserialize, Serialize};

use crate::{common::Provenance, enums::LeverageKind, id::Id};

/// Leverage held by one actor over another (or many).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Leverage {
    /// Content-addressed id.
    pub id: Id,
    /// Who holds the leverage.
    pub holder: Id,
    /// Who the leverage is directed against.
    pub target: Id,
    /// Mechanism / kind of leverage.
    pub mechanism: LeverageKind,
    /// Magnitude in [0, 1] (strength of the leverage).
    pub magnitude: f32,
    /// Activation cost in [0, 1] — how expensive to deploy.
    pub activation_cost: f32,
    /// Credibility in [0, 1] — how believable the threat / promise is.
    pub credibility: f32,
    /// Free-text description.
    pub description: String,
    /// Provenance.
    pub prov: Provenance,
}
