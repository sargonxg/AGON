//! First-order-logic representations for claims and constraints.
//!
//! Populated lazily via the optional `--extract-fol` extractor (Day 9). The
//! Z3 contradiction layer consumes these. Day 1 ships only the variant
//! constructors; the encoder/decoder lives in `aco-infer`.

use serde::{Deserialize, Serialize};

use crate::id::Id;

/// Variable / term in FOL.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Term {
    /// Free variable.
    Var(String),
    /// Constant referring to a canonical primitive id.
    Const(Id),
    /// Literal value (string).
    Literal(String),
    /// Function application.
    App {
        /// Function symbol.
        name: String,
        /// Arguments.
        args: Vec<Term>,
    },
}

/// FOL formula.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Formula {
    /// Atomic predicate `P(t1, …, tn)`.
    Predicate {
        /// Predicate symbol.
        name: String,
        /// Arguments.
        args: Vec<Term>,
    },
    /// `phi = psi`.
    Eq(Box<Term>, Box<Term>),
    /// `¬phi`.
    Not(Box<Formula>),
    /// `phi ∧ psi`.
    And(Vec<Formula>),
    /// `phi ∨ psi`.
    Or(Vec<Formula>),
    /// `phi → psi`.
    Implies(Box<Formula>, Box<Formula>),
    /// `∀ x . phi`.
    Forall {
        /// Bound variable.
        var: String,
        /// Body.
        body: Box<Formula>,
    },
    /// `∃ x . phi`.
    Exists {
        /// Bound variable.
        var: String,
        /// Body.
        body: Box<Formula>,
    },
    /// Boolean literal.
    Bool(bool),
}

impl Formula {
    /// Convenience constructor: tautological true.
    pub fn truth() -> Self {
        Formula::Bool(true)
    }

    /// Convenience constructor: tautological false.
    pub fn falsity() -> Self {
        Formula::Bool(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_compile() {
        let p = Formula::Predicate { name: "P".into(), args: vec![Term::Var("x".into())] };
        let not_p = Formula::Not(Box::new(p.clone()));
        let and = Formula::And(vec![p, not_p]);
        // round-trip
        let j = serde_json::to_string(&and).unwrap();
        let back: Formula = serde_json::from_str(&j).unwrap();
        assert_eq!(and, back);
    }
}
