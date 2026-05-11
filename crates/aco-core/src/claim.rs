//! `Claim` — an asserted fact, evaluation, or normative statement.

use serde::{Deserialize, Serialize};

use crate::{
    common::{Provenance, TemporalInterval},
    enums::{Modality, Polarity, SpeechAct, Stance},
    id::Id,
};

/// A claim made by an actor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    /// Content-addressed id.
    pub id: Id,
    /// Speaker (canonical actor id).
    pub speaker: Id,
    /// Optional addressee.
    pub addressed_to: Option<Id>,
    /// Verbatim or canonicalised proposition text.
    pub proposition: String,
    /// Epistemic modality.
    pub modality: Modality,
    /// Speech-act class (Searle).
    pub speech_act: SpeechAct,
    /// Polarity.
    pub polarity: Polarity,
    /// Stance toward the proposition.
    pub stance: Stance,
    /// Intensity in [0, 1] (rhetorical force / commitment strength).
    pub intensity: f32,
    /// When the claim was made.
    pub interval: TemporalInterval,
    /// Provenance.
    pub prov: Provenance,
}
