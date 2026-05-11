//! Interpersonal-conflict pattern types.
//!
//! Citations:
//! - Four Horsemen — Gottman & Levenson, *The timing of divorce*, JMF 2000
//! - Bids for connection — Gottman, *The Relationship Cure*, 2001
//! - Repair attempts — Gottman, *The Seven Principles for Making Marriage Work*, 1999
//! - DARVO — Freyd, *Violations of power*, 1996; Harsey & Freyd, 2020
//! - Gaslighting — Sweet, *The Sociology of Gaslighting*, 2019
//! - Triangulation — Bowen, *Family Therapy in Clinical Practice*, 1978
//! - Drama triangle — Karpman, TAB 1968

use serde::{Deserialize, Serialize};

use crate::{
    common::{Provenance, TemporalInterval},
    id::Id,
};

/// Bid-for-connection response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BidResponse {
    /// Turning toward — engagement, acknowledgement.
    TurningToward,
    /// Turning away — minimal/distracted response.
    TurningAway,
    /// Turning against — hostile or dismissive.
    TurningAgainst,
}

/// Kind of repair attempt.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairKind {
    /// Apology.
    Apology,
    /// Humor used to defuse.
    Humor,
    /// Explicit ownership of fault.
    TakingResponsibility,
    /// Acknowledgement of the other's feelings.
    Acknowledgment,
    /// Physical affection (where context implies it).
    PhysicalAffection,
    /// Offering a concrete resolution.
    OfferingResolution,
    /// Other.
    Other(String),
}

/// Pattern kind (closed except for `Other`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PatternKind {
    /// Gottman: criticism.
    Criticism,
    /// Gottman: contempt — the single strongest divorce predictor.
    Contempt,
    /// Gottman: defensiveness.
    Defensiveness,
    /// Gottman: stonewalling.
    Stonewalling,
    /// DARVO sequence.
    Darvo {
        /// Deny claim id.
        deny: Id,
        /// Attack claim id.
        attack: Id,
        /// Reverse-victim-and-offender claim id.
        reverse: Id,
    },
    /// Gaslighting (reality denial / memory undermining).
    Gaslighting,
    /// Triangulation (using a third party as leverage).
    Triangulation,
    /// Projection.
    Projection,
    /// Bid for connection + observed response.
    BidForConnection {
        /// Bid claim/event id.
        bid: Id,
        /// Observed response.
        response: BidResponse,
    },
    /// Repair attempt + whether it landed.
    RepairAttempt {
        /// Event that triggered the repair.
        trigger_event: Id,
        /// Kind of repair.
        repair_kind: RepairKind,
        /// True if the repair was accepted / de-escalated.
        landed: bool,
    },
    /// Escalation cycle with constituent events.
    EscalationCycle {
        /// Constituent event ids in order.
        events: Vec<Id>,
        /// Peak event id.
        peak: Id,
    },
    /// Withdrawal.
    Withdrawal,
    /// Capitulation.
    Capitulation,
    /// Other.
    Other(String),
}

/// A pattern finding detected over the canonical graph.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternFinding {
    /// Content-addressed id.
    pub id: Id,
    /// Pattern kind.
    pub kind: PatternKind,
    /// Actors involved.
    pub actors: Vec<Id>,
    /// Event ids referenced.
    pub events: Vec<Id>,
    /// Claim ids referenced.
    pub claims: Vec<Id>,
    /// Detector confidence in [0, 1].
    pub confidence: f32,
    /// Temporal interval the pattern spans.
    pub interval: TemporalInterval,
    /// Provenance.
    pub prov: Provenance,
}

/// Dominant emotion in an affect marker.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Emotion {
    /// Anger.
    Anger,
    /// Fear.
    Fear,
    /// Sadness.
    Sadness,
    /// Disgust.
    Disgust,
    /// Contempt.
    Contempt,
    /// Joy.
    Joy,
    /// Pride.
    Pride,
    /// Relief.
    Relief,
    /// Affection.
    Affection,
    /// Shame.
    Shame,
    /// Guilt.
    Guilt,
    /// Embarrassment.
    Embarrassment,
    /// Anxiety.
    Anxiety,
    /// Frustration.
    Frustration,
    /// Resentment.
    Resentment,
    /// Neutral.
    Neutral,
    /// Other.
    Other(String),
}

/// Per-turn affect marker.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AffectMarker {
    /// Content-addressed id.
    pub id: Id,
    /// Event id of the turn that bears this affect.
    pub turn_event: Id,
    /// Speaker.
    pub speaker: Id,
    /// Target (addressee).
    pub target: Option<Id>,
    /// Valence in [-1, +1].
    pub valence: f32,
    /// Arousal in [0, 1].
    pub arousal: f32,
    /// Dominant emotion.
    pub dominant_emotion: Emotion,
    /// Provenance.
    pub prov: Provenance,
}
