//! `Event` — a dated or orderable occurrence in the world.

use serde::{Deserialize, Serialize};

use crate::{
    common::{Place, Provenance, TemporalInterval},
    id::Id,
};

/// Thematic role of a participant in an event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// Agent (initiator).
    Agent,
    /// Patient (affected party).
    Patient,
    /// Beneficiary.
    Beneficiary,
    /// Witness / observer.
    Witness,
    /// Mediator / third party.
    Mediator,
    /// Instrument.
    Instrument,
    /// Other.
    Other,
}

/// A participant in an event with a thematic role.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Participant {
    /// Canonical actor id.
    pub actor: Id,
    /// Role.
    pub role: Role,
}

/// A discrete event.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// Content-addressed id.
    pub id: Id,
    /// Free-text event type ("meeting", "filing", "accusation", ...).
    pub event_type: String,
    /// Participants with roles.
    pub participants: Vec<Participant>,
    /// Temporal interval.
    pub interval: TemporalInterval,
    /// Optional place.
    pub place: Option<Place>,
    /// Cause ids (ids of other events believed to have caused this).
    #[serde(default)]
    pub causes: Vec<Id>,
    /// Effect ids (events caused by this).
    #[serde(default)]
    pub effects: Vec<Id>,
    /// Provenance.
    pub prov: Provenance,
}
