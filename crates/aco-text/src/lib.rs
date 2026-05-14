#![forbid(unsafe_code)]
//! aco-text — canonical text foundation for AGON.
//!
//! Pipeline:
//!   raw bytes → NFC → strip bidi/zero-width → whitespace canonicalize
//!            → segments → quoted-speech FSM → speaker turns
//!
//! Every span downstream of this point references canonical-NFC offsets, NOT raw bytes.
//! The `raw_byte_map` lets us recover original bytes for audit / display.

pub mod normalize;
pub mod quoted;
pub mod segment;
pub mod turns;
pub mod verify;

pub use normalize::{normalize, NormalizedDocument, RawDocument};
pub use quoted::{detect_speech, SpeechKind, SpeechSpan};
pub use segment::{segment, Segment, SegmentKind};
pub use turns::{detect_turns, SpeakerTurn, TurnKind};
pub use verify::{verify_span, SpanVerification};

pub const NORMALIZATION_VERSION: &str = "0.1.0";
