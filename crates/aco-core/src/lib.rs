//! `aco-core` — pure types for the Agentic Conflict Ontology.
//!
//! Level 0 in the crate hierarchy. No I/O, no async, no tokio. Every type is
//! `Serialize + Deserialize` and round-trips through JSON. Every primitive
//! carries a [`Provenance`] tying it back to the source span(s) it came from.
//!
//! The eight primitives are: [`Actor`], [`Claim`], [`Interest`], [`Constraint`],
//! [`Leverage`], [`Commitment`], [`Event`], [`Narrative`]. Interpersonal
//! extensions ([`PatternFinding`], [`AffectMarker`]) live in [`patterns`].

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod actor;
pub mod claim;
pub mod commitment;
pub mod common;
pub mod constraint;
pub mod enums;
pub mod error;
pub mod event;
pub mod fol;
pub mod id;
pub mod interest;
pub mod leverage;
pub mod narrative;
pub mod patterns;

pub use actor::Actor;
pub use claim::Claim;
pub use commitment::Commitment;
pub use common::{Defeasibility, Derivation, EvidenceSpan, Place, Provenance, TemporalInterval};
pub use constraint::Constraint;
pub use enums::{
    ActorKind, CommitmentStatus, DefeasibilityClass, Deontic, InterestCategory, LeverageKind,
    Modality, Polarity, SpeechAct, Stance,
};
pub use error::{Error, Result};
pub use event::{Event, Participant, Role};
pub use fol::{Formula, Term};
pub use id::{Canonical, Id};
pub use interest::Interest;
pub use leverage::Leverage;
pub use narrative::Narrative;
pub use patterns::{AffectMarker, BidResponse, Emotion, PatternFinding, PatternKind, RepairKind};

/// Convenience: type alias for a 32-bit float confidence value in `[0, 1]`.
pub type Confidence = f32;

/// Emit an init span; called by binaries that want to confirm the crate loaded.
pub fn init() {
    tracing::trace!(crate_name = "aco-core", version = env!("CARGO_PKG_VERSION"), "loaded");
}
