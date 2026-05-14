//! `PatternContext` — everything detectors need to make a decision.
//!
//! Wraps canonical text + speaker turns + lexical signals into one struct so
//! patterns can be written as pure functions of context → matches.

use aco_lex::LexicalFeatures;
use aco_text::{SpeakerTurn, SpeechSpan};

#[derive(Debug, Clone)]
pub struct PatternContext<'a> {
    pub canonical_text: &'a str,
    pub turns: &'a [Turn],
    pub speech_spans: &'a [SpeechSpan],
    pub speaker_turns: &'a [SpeakerTurn],
    pub lexical: &'a LexicalFeatures,
}

/// A speaker turn from the pretransform layer. Re-defined locally to avoid a
/// circular dep on aco-server. Construct from `aco-server`'s `Turn` via `.into()`.
#[derive(Debug, Clone)]
pub struct Turn {
    pub idx: usize,
    pub speaker: Option<String>,
    pub body: String,
    pub char_offset: usize,
}

impl<'a> PatternContext<'a> {
    pub fn new(
        canonical_text: &'a str,
        turns: &'a [Turn],
        speech_spans: &'a [SpeechSpan],
        speaker_turns: &'a [SpeakerTurn],
        lexical: &'a LexicalFeatures,
    ) -> Self {
        Self { canonical_text, turns, speech_spans, speaker_turns, lexical }
    }
}
