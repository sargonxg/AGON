//! Supporting enums for the ACO ontology.
//!
//! All variants are closed-world. Add a variant only when the ontology requires
//! it — never widen on the LLM's behalf.

use serde::{Deserialize, Serialize};

/// Kind of actor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorKind {
    /// Individual person.
    Person,
    /// Organisation (corporate, non-profit, government).
    Organisation,
    /// Coalition or alliance of actors.
    Coalition,
    /// State / nation.
    State,
    /// Group, faction, informal collective.
    Group,
    /// Unknown / unresolved.
    Unknown,
}

/// Epistemic modality of a claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Modality {
    /// Asserted as fact.
    Asserted,
    /// Implied / inferred.
    Implied,
    /// Hedged ("might", "perhaps").
    Hedged,
    /// Denied.
    Denied,
    /// Questioned.
    Questioned,
    /// Counterfactual.
    Counterfactual,
}

/// Speech-act class (Searle).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeechAct {
    /// Assertive (states a fact).
    Assertive,
    /// Directive (requests an action).
    Directive,
    /// Commissive (promises an action).
    Commissive,
    /// Expressive (expresses a feeling).
    Expressive,
    /// Declaration (changes the world).
    Declaration,
}

/// Polarity (positive / negative / neutral).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Polarity {
    /// Positive valence.
    Positive,
    /// Negative valence.
    Negative,
    /// Neutral / mixed.
    Neutral,
}

/// Stance toward the proposition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stance {
    /// Supports / endorses.
    Endorses,
    /// Opposes / rejects.
    Opposes,
    /// Neutral.
    Neutral,
    /// Ambivalent / mixed.
    Ambivalent,
}

/// Kind of leverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeverageKind {
    /// Information asymmetry.
    Information,
    /// Resource (financial, material).
    Resource,
    /// Authority / formal role.
    Authority,
    /// Network / coalitional.
    Network,
    /// Reputational / normative.
    Reputational,
    /// Coercive (threat of harm).
    Coercive,
    /// Other.
    Other,
}

/// Deontic modality.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Deontic {
    /// Obligation (must).
    Obligation,
    /// Permission (may).
    Permission,
    /// Prohibition (must not).
    Prohibition,
    /// No deontic force.
    None,
}

/// Category for interests (Fisher/Ury framing).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterestCategory {
    /// Material / economic.
    Material,
    /// Security / safety.
    Security,
    /// Recognition / status.
    Recognition,
    /// Autonomy / control.
    Autonomy,
    /// Belonging / relational.
    Belonging,
    /// Identity / values.
    Identity,
    /// Other.
    Other,
}

/// Status of a commitment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitmentStatus {
    /// Pending — deadline not yet reached.
    Pending,
    /// Fulfilled in full.
    Fulfilled,
    /// Partially fulfilled.
    Partial,
    /// Breached / not fulfilled.
    Breached,
    /// Cancelled.
    Cancelled,
    /// Contested — parties disagree it was made.
    Contested,
}

/// Defeasibility of a primitive or derivation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefeasibilityClass {
    /// Strict — cannot be defeated.
    Strict,
    /// Defeasible — can be defeated by stronger rules.
    Defeasible,
    /// Presumption — defeasible default.
    Presumption,
}
