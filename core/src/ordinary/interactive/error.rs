// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Why a step does not apply, and the text that says so.

use crate::WitnessFault;
use crate::limits::Refusal;
use crate::linear::proofs::GoalId;
use crate::ordinary::{Logic, Rule, Side};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Why a step does not apply to a goal as asked. Positions are the ones
/// the step gave, counted from 0 on the side of the goal it named.
///
/// Needs the cargo feature `interactive` (on by default).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize),
    serde(tag = "kind", rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum StepError {
    /// No open goal has this id.
    #[non_exhaustive]
    NoGoal {
        /// The id asked for.
        goal: GoalId,
    },
    /// The side has no formula at the position; it has `len` formulas.
    #[non_exhaustive]
    NoFormula {
        /// The side asked for.
        side: Side,
        /// The position asked for.
        position: usize,
        /// How many formulas the side has.
        len: usize,
    },
    /// The rule does not act on the formula: the formula has another
    /// connective, or the rule is for the other side of `⊢`. Cut and the
    /// mark of an open goal act on no formula.
    #[non_exhaustive]
    Rule {
        /// The rule asked for.
        rule: Rule,
        /// The side of the formula.
        side: Side,
        /// The position of the formula.
        position: usize,
    },
    /// The logic does not offer the rule: ex falso (`⊥L`) in minimal
    /// logic, and in intuitionistic and minimal logic a rule whose premise
    /// would have two formulas right of `⊢` or none, since every goal there
    /// has exactly one.
    #[non_exhaustive]
    Logic {
        /// The rule asked for.
        rule: Rule,
        /// The logic of the proof.
        logic: Logic,
    },
    /// The axiom needs the formula on the other side of `⊢` too.
    #[non_exhaustive]
    NoPartner {
        /// The side of the formula.
        side: Side,
        /// The position of the formula.
        position: usize,
    },
    /// In intuitionistic or minimal logic, a premise would have this many
    /// formulas right of `⊢` instead of one.
    #[non_exhaustive]
    Succedents {
        /// How many formulas would stand right of `⊢`.
        count: usize,
    },
    /// A position of the split is out of range, repeated, or the formula
    /// the rule acts on.
    #[non_exhaustive]
    Split {
        /// The side of the position.
        side: Side,
        /// The offending position.
        position: usize,
    },
    /// A split was given to a rule with fewer than two premises.
    #[non_exhaustive]
    NoSplit {
        /// The rule asked for.
        rule: Rule,
    },
    /// The weakenings below an axiom, or the contractions below a shared
    /// rule, are estimated past the bound on the derivation's bytes that
    /// the proof was started or read under.
    #[non_exhaustive]
    Size {
        /// The estimate of the run, in bytes.
        estimate_bytes: u64,
        /// The bound, `Limits::derivation_bytes`.
        limit_bytes: u64,
    },
    /// A witness was given to a rule that takes none: only `∀L` and `∃R`
    /// take one, and the session names the eigenvariable of `∀R` and `∃L`.
    #[non_exhaustive]
    NoWitness {
        /// The rule asked for.
        rule: Rule,
    },
    /// The witness of `∀L` or `∃R` is not a term of the goal: a term of
    /// the sequent's function symbols and of the eigenvariables in the
    /// goal's scope.
    #[non_exhaustive]
    Witness {
        /// What is wrong with it.
        #[cfg_attr(feature = "serialize", serde(flatten))]
        fault: Box<WitnessFault>,
    },
    /// The axiom's formula unifies with a formula on the other side of `⊢`
    /// only by giving an open witness a term it may not take: an
    /// eigenvariable that a `∀R` or `∃L` introduced above the step that
    /// left the witness open, or on another branch.
    #[non_exhaustive]
    Binding {
        /// What the filling would break.
        #[cfg_attr(feature = "serialize", serde(flatten))]
        fault: Box<BindingFault>,
    },
    /// The step would make the proof's terms unfold to more symbols
    /// written out than the bound on terms (`Limits::terms`) the proof
    /// was started or read under: an instance, or the terms an axiom
    /// fills open witnesses with. A document of such a proof would be
    /// refused when read.
    #[non_exhaustive]
    Terms {
        /// The terms written out, as a document's reader counts them, or
        /// a number past the bound.
        terms: u64,
        /// The bound.
        limit: u64,
    },
}

/// Returns `left of ⊢` or `right of ⊢`.
const fn of(side: Side) -> &'static str {
    match side {
        Side::Left => "left of ⊢",
        Side::Right => "right of ⊢",
    }
}

/// Returns what a rule acts on, as a phrase: `a conjunction right of ⊢`.
const fn target(rule: Rule) -> &'static str {
    use Rule::*;
    match rule {
        Axiom => "a formula that stands on both sides of ⊢",
        WeakenLeft | ContractLeft => "any formula left of ⊢",
        WeakenRight | ContractRight => "any formula right of ⊢",
        AndLeft | AndLeft1 | AndLeft2 => "a conjunction left of ⊢",
        AndRight => "a conjunction right of ⊢",
        OrLeft => "a disjunction left of ⊢",
        OrRight | OrRight1 | OrRight2 => "a disjunction right of ⊢",
        ImpliesLeft => "an implication left of ⊢",
        ImpliesRight => "an implication right of ⊢",
        NotLeft => "a negation left of ⊢",
        NotRight => "a negation right of ⊢",
        IffLeft | IffLeft1 | IffLeft2 => "an equivalence left of ⊢",
        IffRight => "an equivalence right of ⊢",
        FalseLeft => "⊥ left of ⊢",
        FalseRight => "⊥ right of ⊢",
        TrueLeft => "⊤ left of ⊢",
        TrueRight => "⊤ right of ⊢",
        Cut => "no formula of a goal, so no step takes it",
        Open => "no formula, since it only marks an open goal",
        ForallLeft => "a universal formula left of ⊢",
        ForallRight => "a universal formula right of ⊢",
        ExistsLeft => "an existential formula left of ⊢",
        ExistsRight => "an existential formula right of ⊢",
    }
}

impl Display for StepError {
    /// Writes the refusal as a sentence that names the rule, side and
    /// position, and says what the step needs.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            StepError::NoGoal { goal } => write!(f, "there is no open goal {}", goal.get()),
            StepError::NoFormula {
                side,
                position,
                len,
            } => write!(
                f,
                "the goal has no formula {position} {}: it has {} there, and positions \
                 start at 0",
                of(*side),
                crate::errors::counted(*len, "formula", "formulas")
            ),
            StepError::Rule {
                rule,
                side,
                position,
            } => write!(
                f,
                "the rule {rule} does not act on formula {position} {}: it acts on {}",
                of(*side),
                target(*rule)
            ),
            StepError::Logic { rule, logic } => {
                let why = match (rule, logic) {
                    (Rule::FalseLeft, Logic::Minimal) => {
                        return write!(
                            f,
                            "the rule {rule} is not available in {logic} logic: minimal logic has \
                             no ex falso"
                        );
                    }
                    (Rule::OrRight, _) => {
                        "its premise would have both disjuncts right of ⊢, and every goal there \
                         has exactly one formula: use ∨R₁ or ∨R₂ to keep one disjunct"
                    }
                    (Rule::ContractRight, _) => {
                        "its premise would have two formulas right of ⊢, and every goal there \
                         has exactly one"
                    }
                    _ => {
                        "its premise would have no formula right of ⊢, and every goal there has \
                         exactly one"
                    }
                };
                write!(
                    f,
                    "the rule {rule} is not available in {logic} logic: {why}"
                )
            }
            StepError::NoPartner { side, position } => write!(
                f,
                "the axiom needs formula {position} {} to stand {} too",
                of(*side),
                of(side.flip())
            ),
            StepError::Succedents { count } => write!(
                f,
                "a premise would have {} right of ⊢, and in intuitionistic and minimal logic \
                 every goal has exactly one: share the context, or split it so that each \
                 premise keeps one formula right of ⊢",
                crate::errors::counted(*count, "formula", "formulas")
            ),
            StepError::Split { side, position } => write!(
                f,
                "the split's position {position} {} is out of range, named twice, or the \
                 formula the rule acts on: name each other formula of the goal at most once",
                of(*side)
            ),
            StepError::Size {
                estimate_bytes,
                limit_bytes,
            } => write!(
                f,
                "the weakenings or contractions this step makes are estimated at \
                 {estimate_bytes} bytes, over the derivation bound of {limit_bytes} bytes the \
                 proof was started or read with: start it with a higher bound"
            ),
            StepError::NoSplit { rule } => {
                write!(
                    f,
                    "the rule {rule} has fewer than two premises and takes no split"
                )
            }
            StepError::NoWitness { rule } => write!(
                f,
                "the rule {rule} takes no witness: only ∀L and ∃R take one, and the session \
                 names the eigenvariable of ∀R and ∃L"
            ),
            StepError::Witness { fault } => write!(f, "the witness is not a term here: {fault}"),
            StepError::Binding { fault } => {
                write!(f, "the axiom cannot fill the open witnesses: {fault}")
            }
            StepError::Terms { terms, limit } => write!(
                f,
                "the step would make the proof's terms unfold to {terms} symbols written out, \
                 more than the bound of {limit} on terms the proof was started or read with: \
                 start it with a higher bound"
            ),
        }
    }
}

impl std::error::Error for StepError {}

/// Why an axiom cannot fill the open witnesses its formulas unify by
/// ([`StepError::Binding`]).
///
/// # JSON
///
/// With the feature `serialize`, a step error's `details` hold it
/// flattened: `"fault"` names the variant in snake case, beside the
/// variant's fields.
///
/// Needs the cargo feature `interactive` (on by default).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize),
    serde(tag = "fault", rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum BindingFault {
    /// An open witness would take an eigenvariable that a `∀R` or `∃L`
    /// introduced after the witness was left open, or on another branch.
    #[non_exhaustive]
    Eigenvariable {
        /// The eigenvariable, as the proof writes it.
        name: String,
    },
    /// An inference that holds the witnesses would break its rule once
    /// they are filled: an eigenvariable would stand in the conclusion of
    /// the `∀R` or `∃L` that introduced it.
    Breaks,
    /// Building the filled formulas passed a bound of the proof's
    /// formulas.
    #[non_exhaustive]
    Refused {
        /// The bound.
        refusal: Refusal,
    },
}

impl Display for BindingFault {
    /// Writes what the filling would break, as a phrase.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Eigenvariable { name } => write!(
                f,
                "an open witness would take the eigenvariable {name}, which a ∀R or ∃L \
                 introduced after the witness was left open or on another branch"
            ),
            Self::Breaks => f.write_str(
                "an inference that holds the witnesses would break its rule once they are \
                 filled: an eigenvariable would stand in the conclusion of the ∀R or ∃L that \
                 introduced it",
            ),
            Self::Refused { refusal } => write!(f, "{refusal}"),
        }
    }
}
