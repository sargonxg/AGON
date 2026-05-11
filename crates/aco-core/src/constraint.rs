//! `Constraint` — a rule, norm, or structural limit shaping feasible outcomes.

use serde::{Deserialize, Serialize};

use crate::{common::Provenance, enums::Deontic, id::Id};

/// A constraint binding one or more actors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Constraint {
    /// Content-addressed id.
    pub id: Id,
    /// Actors bound by the constraint.
    pub binds: Vec<Id>,
    /// Source of the constraint (e.g., `"contract"`, `"policy_xyz"`, `"statute_42"`).
    pub source: String,
    /// Deontic modality.
    pub modality: Deontic,
    /// Free-text description.
    pub content: String,
    /// Optional FOL representation id (populated when `--extract-fol` is on).
    pub formal: Option<Id>,
    /// Provenance.
    pub prov: Provenance,
}
