//! `Commitment` — a promised future action.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{common::Provenance, enums::CommitmentStatus, id::Id};

/// A commitment from one actor to another (or to a group).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Commitment {
    /// Content-addressed id.
    pub id: Id,
    /// Actor making the commitment.
    pub committed_by: Id,
    /// Actors the commitment is owed to.
    pub committed_to: Vec<Id>,
    /// What the commitment covers (the promised action / state).
    pub content: String,
    /// Deadline, if any.
    pub deadline: Option<DateTime<Utc>>,
    /// Conditional clauses ("if X then I will Y").
    #[serde(default)]
    pub conditionals: Vec<String>,
    /// Current status.
    pub status: CommitmentStatus,
    /// Verification regime ("inspection", "self-report", "audit", or `None`).
    pub verification: Option<String>,
    /// Provenance.
    pub prov: Provenance,
}
