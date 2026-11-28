// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Normal forms of classical formulas, every rewrite shown and the result
//! checked.
//!
//! [`rewrite`] turns each formula of a sequent into a normal form, one
//! step at a time:
//!
//! | form | what it is | what it keeps |
//! |---|---|---|
//! | [`Nnf`](Form::Nnf) | negation normal form: `∧`, `∨` and `¬` before atoms | each formula's meaning |
//! | [`Cnf`](Form::Cnf) | conjunctive normal form by distribution: a conjunction of clauses | each formula's meaning |
//! | [`Dnf`](Form::Dnf) | disjunctive normal form by distribution: a disjunction of conjunctions of literals | each formula's meaning |
//! | [`Tseitin`](Form::Tseitin) | the clauses of the sequent's negation, a new atom for every compound subformula | satisfiability |
//! | [`PlaistedGreenbaum`](Form::PlaistedGreenbaum) | Tseitin's clauses for the direction each subformula's polarity needs | satisfiability |
//!
//! A literal is an atom or a negated atom, a clause a disjunction of
//! literals. The first three forms rewrite by equivalence laws (`A → B` as
//! `¬A ∨ B`, De Morgan's laws, distribution), each [`Step`] one law applied
//! at the leftmost outermost place it fits, in phases: first `→` and `↔`,
//! then the negations, then the constants `⊤` and `⊥`, then distribution.
//! Distribution can make a formula exponentially longer, so every sequent
//! on the way is held to [`Limits::form_occurrences`].
//!
//! The encodings answer another question: a sequent `Γ ⊢ Δ` is valid
//! exactly when `Γ` with the negation of every formula of `Δ` cannot be
//! satisfied. They move each formula right of `⊢` to the left negated,
//! then name every compound subformula by a new atom, the innermost
//! first, and add the clauses that tie the atom to what it names. The
//! result `C₁, …, Cₙ ⊢` has the clauses as hypotheses. It is valid
//! exactly when the clauses cannot all be true, which is exactly when the
//! input is valid. The new atoms' names (`t1`, `t2`, …, with `_` after the
//! `t` where the input has such names) never meet the input's.
//!
//! [`normalize`] rewrites and checks what the form promises, deciding
//! classical sequents by the SAT solver, whose clausal proof the crate's
//! checker accepts (with the cargo feature `sat`, on by default; without
//! it by the logic's default method):
//! - an equivalent form: `⊢ (A₁ ↔ A₁') ∧ …` is valid, each formula with
//!   its normal form;
//! - an encoding: the input with the definitions `t ↔ G` concludes the
//!   clauses, so a model of the input extends to one of the clauses, and
//!   the clauses conclude the input, so a model of the clauses is one of
//!   the input. The definitions name new atoms, each by subformulas of the
//!   input and earlier atoms only, which the check confirms first.
//!
//! A promise found false is [`Error::ClaimFails`], a defect, never an
//! answer.
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::ordinary::normal::{self, Form, Options};
//! use deduxit::ordinary::Sequent;
//!
//! let sequent: Sequent = "a -> b /\\ c".parse()?;
//! let options = Options::default().with_form(Form::Cnf);
//! let cnf = normal::normalize_default(&sequent, &options)?;
//! assert_eq!(cnf.result.to_string(), "⊢ (¬a ∨ b) ∧ (¬a ∨ c)");
//! assert_eq!(cnf.steps.len(), 2);
//! assert!(cnf.is_checked());
//! # Ok::<(), deduxit::Error>(())
//! ```

use super::derivation::ArenaSize;
use super::transfer::{self, Parameters};
use super::{Formulas, Logic, Node, NodeId, Outcome, Sequent, Verdict, decide_within};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::search;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

mod encode;
mod first_order;
mod form;
mod laws;

use self::encode::encoded;
pub(super) use self::first_order::fresh_prefix;
#[cfg(feature = "sat")]
pub(super) use self::first_order::negation_normal;
use self::first_order::{skolem_term, skolemized};
use self::form::in_form;
use self::laws::{Budget, Passes, Stage, apply, equivalent, law};

/// A normal form, by its name.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "kebab-case"))]
pub enum Form {
    /// Negation normal form: no `→` or `↔`, and `¬` before atoms alone.
    Nnf,
    /// Conjunctive normal form by distribution: a conjunction of clauses,
    /// `⊤` or `⊥`.
    #[default]
    Cnf,
    /// Disjunctive normal form by distribution: a disjunction of
    /// conjunctions of literals, `⊤` or `⊥`.
    Dnf,
    /// Tseitin's encoding: the clauses of the sequent's negation, a new
    /// atom for every compound subformula with clauses for both
    /// directions of its definition.
    Tseitin,
    /// Plaisted and Greenbaum's encoding: Tseitin's, with only the
    /// direction of each definition that the subformula's polarity needs.
    PlaistedGreenbaum,
    /// Prenex normal form: every quantifier in front of a formula without
    /// one.
    Prenex,
    /// Miniscope form: negation normal form with every quantifier moved as
    /// far in as it goes.
    Miniscope,
    /// Skolem normal form: the sequent's negation as hypotheses in negation
    /// normal form, each `∃` replaced by a term of new function symbols
    /// over the variables free in it.
    Skolem,
    /// Herbrand normal form: the sequent with each strong quantifier (an
    /// `∃` left of `⊢`, a `∀` right of it) replaced by a term of new
    /// function symbols over the variables free in it.
    Herbrand,
    /// Clausal normal form: the Skolem normal form's hypotheses as clauses,
    /// each closed by `∀`s in front.
    Clausal,
}

impl Form {
    /// Every form, in the order of [`NAMES`](Self::NAMES).
    pub const ALL: &'static [Self] = &[
        Self::Nnf,
        Self::Cnf,
        Self::Dnf,
        Self::Tseitin,
        Self::PlaistedGreenbaum,
        Self::Prenex,
        Self::Miniscope,
        Self::Skolem,
        Self::Herbrand,
        Self::Clausal,
    ];

    /// The forms' names, which [`name`](Self::name) writes and `FromStr`
    /// reads.
    pub const NAMES: &'static [&'static str] = &[
        "nnf",
        "cnf",
        "dnf",
        "tseitin",
        "plaisted-greenbaum",
        "prenex",
        "miniscope",
        "skolem",
        "herbrand",
        "clausal",
    ];

    /// Returns the form's name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Nnf => "nnf",
            Self::Cnf => "cnf",
            Self::Dnf => "dnf",
            Self::Tseitin => "tseitin",
            Self::PlaistedGreenbaum => "plaisted-greenbaum",
            Self::Prenex => "prenex",
            Self::Miniscope => "miniscope",
            Self::Skolem => "skolem",
            Self::Herbrand => "herbrand",
            Self::Clausal => "clausal",
        }
    }

    /// Returns what a sentence calls it: "conjunctive normal form".
    pub const fn title(self) -> &'static str {
        match self {
            Self::Nnf => "negation normal form",
            Self::Cnf => "conjunctive normal form",
            Self::Dnf => "disjunctive normal form",
            Self::Tseitin => "Tseitin's encoding",
            Self::PlaistedGreenbaum => "Plaisted and Greenbaum's encoding",
            Self::Prenex => "prenex normal form",
            Self::Miniscope => "miniscope form",
            Self::Skolem => "Skolem normal form",
            Self::Herbrand => "Herbrand normal form",
            Self::Clausal => "clausal normal form",
        }
    }

    /// Returns what the form keeps of its input.
    pub const fn promise(self) -> Promise {
        match self {
            Self::Nnf | Self::Cnf | Self::Dnf | Self::Prenex | Self::Miniscope => {
                Promise::Equivalent
            }
            Self::Tseitin | Self::PlaistedGreenbaum | Self::Skolem | Self::Clausal => {
                Promise::Equisatisfiable
            }
            Self::Herbrand => Promise::Equivalid,
        }
    }

    /// Returns whether the form takes a first-order sequent: negation
    /// normal form, the forms that move quantifiers or remove them, and the
    /// clausal form. The others are propositional.
    pub const fn takes_first_order(self) -> bool {
        match self {
            Self::Nnf
            | Self::Prenex
            | Self::Miniscope
            | Self::Skolem
            | Self::Herbrand
            | Self::Clausal => true,
            Self::Cnf | Self::Dnf | Self::Tseitin | Self::PlaistedGreenbaum => false,
        }
    }
}

impl Display for Form {
    /// Writes the form's [`name`](Self::name).
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

impl std::str::FromStr for Form {
    type Err = Error;

    /// Reads a form's [`name`](Self::name), one of [`NAMES`](Self::NAMES).
    fn from_str(name: &str) -> Result<Self, Error> {
        crate::name::named(Self::ALL, Self::name, name, "normal form", Self::NAMES)
    }
}

/// What a normal form keeps of its input.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "snake_case"))]
pub enum Promise {
    /// Each formula is equivalent to its normal form.
    Equivalent,
    /// The clauses can be satisfied exactly when the sequent's negation
    /// can, so the result is valid exactly when the input is.
    Equisatisfiable,
    /// The result is valid exactly when the input is, its sides kept.
    Equivalid,
}

impl Promise {
    /// Returns the promise's name: `equivalent`, `equisatisfiable`.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Equivalent => "equivalent",
            Self::Equisatisfiable => "equisatisfiable",
            Self::Equivalid => "equivalid",
        }
    }
}

/// Which normal form to make.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Options {
    /// The form, conjunctive normal form by default.
    pub form: Form,
}

impl Options {
    /// Returns the options with the form given.
    #[must_use]
    pub const fn with_form(self, form: Form) -> Self {
        Self { form }
    }
}

/// The law a step applies.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "kebab-case"))]
pub enum Rule {
    /// `A → B ≡ ¬A ∨ B`
    Implication,
    /// `A ↔ B ≡ (¬A ∨ B) ∧ (A ∨ ¬B)`, on the way to negation and
    /// conjunctive normal form.
    Equivalence,
    /// `A ↔ B ≡ (A ∧ B) ∨ (¬A ∧ ¬B)`, on the way to disjunctive normal
    /// form.
    EquivalenceAsCases,
    /// `¬¬A ≡ A`
    DoubleNegation,
    /// `¬(A ∧ B) ≡ ¬A ∨ ¬B`
    DeMorganAnd,
    /// `¬(A ∨ B) ≡ ¬A ∧ ¬B`
    DeMorganOr,
    /// `¬⊤ ≡ ⊥`
    NotTrue,
    /// `¬⊥ ≡ ⊤`
    NotFalse,
    /// `A ∧ ⊤ ≡ A`, either way round.
    AndTrue,
    /// `A ∧ ⊥ ≡ ⊥`, either way round.
    AndFalse,
    /// `A ∨ ⊤ ≡ ⊤`, either way round.
    OrTrue,
    /// `A ∨ ⊥ ≡ A`, either way round.
    OrFalse,
    /// `A ∨ (B ∧ C) ≡ (A ∨ B) ∧ (A ∨ C)`, either way round.
    DistributeOr,
    /// `A ∧ (B ∨ C) ≡ (A ∧ B) ∨ (A ∧ C)`, either way round.
    DistributeAnd,
    /// A formula right of `⊢` moved to the left, negated: `Γ ⊢ A, Δ` is
    /// valid exactly when `Γ, ¬A ⊢ Δ` is.
    Negate,
    /// A new atom `t` names a compound subformula `G`: `G` is replaced by
    /// `t` and the clauses of `t ↔ G` (or of one direction) are added.
    Define,
    /// `⊤` left of `⊢` left out: `Γ, ⊤ ⊢ Δ` is valid exactly when `Γ ⊢ Δ`
    /// is.
    TrueHypothesis,
    /// `¬∀x. A ≡ ∃x. ¬A`
    NotForall,
    /// `¬∃x. A ≡ ∀x. ¬A`
    NotExists,
    /// `(∀x. A) ∧ B ≡ ∀x. (A ∧ B)`, either way round, `x` not free in `B`.
    ForallAnd,
    /// `(∃x. A) ∧ B ≡ ∃x. (A ∧ B)`, either way round, `x` not free in `B`.
    ExistsAnd,
    /// `(∀x. A) ∨ B ≡ ∀x. (A ∨ B)`, either way round, `x` not free in `B`.
    ForallOr,
    /// `(∃x. A) ∨ B ≡ ∃x. (A ∨ B)`, either way round, `x` not free in `B`.
    ExistsOr,
    /// `(∀x. A) → B ≡ ∃x. (A → B)`, `x` not free in `B`.
    ForallAntecedent,
    /// `(∃x. A) → B ≡ ∀x. (A → B)`, `x` not free in `B`.
    ExistsAntecedent,
    /// `A → ∀x. B ≡ ∀x. (A → B)`, `x` not free in `A`.
    ForallConsequent,
    /// `A → ∃x. B ≡ ∃x. (A → B)`, `x` not free in `A`.
    ExistsConsequent,
    /// `A ↔ B ≡ (A → B) ∧ (B → A)`, on the way to prenex form.
    EquivalenceAsImplications,
    /// `∀x. (A ∧ B) ≡ (∀x. A) ∧ (∀x. B)`
    ForallDistribute,
    /// `∃x. (A ∨ B) ≡ (∃x. A) ∨ (∃x. B)`
    ExistsDistribute,
    /// `∀x. A ≡ A` and `∃x. A ≡ A`, `x` not free in `A`.
    Vacuous,
    /// An `∃` replaced by its Skolem term: `∃y. A` by `A[f(x̄)/y]`, `f` a new
    /// function symbol and `x̄` the variables free in `∃y. A`. The result is
    /// satisfiable exactly when the input is.
    Skolemize,
    /// A strong quantifier replaced by its Herbrand term: `∃y. A` left of
    /// `⊢` or `∀y. A` right of it by `A[f(x̄)/y]`, `f` a new function symbol
    /// and `x̄` the variables free in the quantified formula. The result is
    /// valid exactly when the input is.
    Herbrandize,
    /// A conjunction left of `⊢` split: `Γ, A ∧ B ⊢ Δ` is valid exactly
    /// when `Γ, A, B ⊢ Δ` is.
    SplitConjunction,
}

impl Rule {
    /// Returns the rule's name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Implication => "implication",
            Self::Equivalence => "equivalence",
            Self::EquivalenceAsCases => "equivalence-as-cases",
            Self::DoubleNegation => "double-negation",
            Self::DeMorganAnd => "de-morgan-and",
            Self::DeMorganOr => "de-morgan-or",
            Self::NotTrue => "not-true",
            Self::NotFalse => "not-false",
            Self::AndTrue => "and-true",
            Self::AndFalse => "and-false",
            Self::OrTrue => "or-true",
            Self::OrFalse => "or-false",
            Self::DistributeOr => "distribute-or",
            Self::DistributeAnd => "distribute-and",
            Self::Negate => "negate",
            Self::Define => "define",
            Self::TrueHypothesis => "true-hypothesis",
            Self::NotForall => "not-forall",
            Self::NotExists => "not-exists",
            Self::ForallAnd => "forall-and",
            Self::ExistsAnd => "exists-and",
            Self::ForallOr => "forall-or",
            Self::ExistsOr => "exists-or",
            Self::ForallAntecedent => "forall-antecedent",
            Self::ExistsAntecedent => "exists-antecedent",
            Self::ForallConsequent => "forall-consequent",
            Self::ExistsConsequent => "exists-consequent",
            Self::EquivalenceAsImplications => "equivalence-as-implications",
            Self::ForallDistribute => "forall-distribute",
            Self::ExistsDistribute => "exists-distribute",
            Self::Vacuous => "vacuous",
            Self::Skolemize => "skolemize",
            Self::Herbrandize => "herbrandize",
            Self::SplitConjunction => "split-conjunction",
        }
    }

    /// Returns the law the rule applies, as a reader sees it.
    pub const fn law(self) -> &'static str {
        match self {
            Self::Implication => "A → B ≡ ¬A ∨ B",
            Self::Equivalence => "A ↔ B ≡ (¬A ∨ B) ∧ (A ∨ ¬B)",
            Self::EquivalenceAsCases => "A ↔ B ≡ (A ∧ B) ∨ (¬A ∧ ¬B)",
            Self::DoubleNegation => "¬¬A ≡ A",
            Self::DeMorganAnd => "¬(A ∧ B) ≡ ¬A ∨ ¬B",
            Self::DeMorganOr => "¬(A ∨ B) ≡ ¬A ∧ ¬B",
            Self::NotTrue => "¬⊤ ≡ ⊥",
            Self::NotFalse => "¬⊥ ≡ ⊤",
            Self::AndTrue => "A ∧ ⊤ ≡ A",
            Self::AndFalse => "A ∧ ⊥ ≡ ⊥",
            Self::OrTrue => "A ∨ ⊤ ≡ ⊤",
            Self::OrFalse => "A ∨ ⊥ ≡ A",
            Self::DistributeOr => "A ∨ (B ∧ C) ≡ (A ∨ B) ∧ (A ∨ C)",
            Self::DistributeAnd => "A ∧ (B ∨ C) ≡ (A ∧ B) ∨ (A ∧ C)",
            Self::Negate => "Γ ⊢ A, Δ is valid exactly when Γ, ¬A ⊢ Δ is",
            Self::Define => {
                "a new atom t names A, with the clauses of t ↔ A or of its one direction the place of A needs"
            }
            Self::TrueHypothesis => "Γ, ⊤ ⊢ Δ is valid exactly when Γ ⊢ Δ is",
            Self::NotForall => "¬∀x. A ≡ ∃x. ¬A",
            Self::NotExists => "¬∃x. A ≡ ∀x. ¬A",
            Self::ForallAnd => "(∀x. A) ∧ B ≡ ∀x. (A ∧ B), x not free in B",
            Self::ExistsAnd => "(∃x. A) ∧ B ≡ ∃x. (A ∧ B), x not free in B",
            Self::ForallOr => "(∀x. A) ∨ B ≡ ∀x. (A ∨ B), x not free in B",
            Self::ExistsOr => "(∃x. A) ∨ B ≡ ∃x. (A ∨ B), x not free in B",
            Self::ForallAntecedent => "(∀x. A) → B ≡ ∃x. (A → B), x not free in B",
            Self::ExistsAntecedent => "(∃x. A) → B ≡ ∀x. (A → B), x not free in B",
            Self::ForallConsequent => "A → ∀x. B ≡ ∀x. (A → B), x not free in A",
            Self::ExistsConsequent => "A → ∃x. B ≡ ∃x. (A → B), x not free in A",
            Self::EquivalenceAsImplications => "A ↔ B ≡ (A → B) ∧ (B → A)",
            Self::ForallDistribute => "∀x. (A ∧ B) ≡ (∀x. A) ∧ (∀x. B)",
            Self::ExistsDistribute => "∃x. (A ∨ B) ≡ (∃x. A) ∨ (∃x. B)",
            Self::Vacuous => "∀x. A ≡ A and ∃x. A ≡ A, x not free in A",
            Self::Skolemize => {
                "∃y. A becomes A[f(x̄)/y], f a new function symbol over the variables x̄ free in ∃y. A: satisfiable exactly when the input is"
            }
            Self::Herbrandize => {
                "a strong quantifier Qy. A becomes A[f(x̄)/y], f a new function symbol over the variables x̄ free in Qy. A: valid exactly when the input is"
            }
            Self::SplitConjunction => "Γ, A ∧ B ⊢ Δ is valid exactly when Γ, A, B ⊢ Δ is",
        }
    }

    /// Returns whether an equivalence law holds in intuitionistic logic both
    /// ways, so that a derivation of LJ checks each step that applies it:
    /// false for the laws that only classical logic proves (one direction
    /// holds in LJ), and for the steps that are no equivalence.
    pub const fn intuitionistic(self) -> bool {
        match self {
            Self::DeMorganOr
            | Self::NotTrue
            | Self::NotFalse
            | Self::AndTrue
            | Self::AndFalse
            | Self::OrTrue
            | Self::OrFalse
            | Self::DistributeOr
            | Self::DistributeAnd
            | Self::NotExists
            | Self::ForallAnd
            | Self::ExistsAnd
            | Self::ExistsOr
            | Self::ExistsAntecedent
            | Self::ForallConsequent
            | Self::EquivalenceAsImplications
            | Self::ForallDistribute
            | Self::ExistsDistribute
            | Self::Vacuous => true,
            Self::Implication
            | Self::Equivalence
            | Self::EquivalenceAsCases
            | Self::DoubleNegation
            | Self::DeMorganAnd
            | Self::NotForall
            | Self::ForallOr
            | Self::ForallAntecedent
            | Self::ExistsConsequent
            | Self::Negate
            | Self::Define
            | Self::TrueHypothesis
            | Self::Skolemize
            | Self::Herbrandize
            | Self::SplitConjunction => false,
        }
    }

    /// Returns whether the rule is an equivalence law, whose step keeps each
    /// formula's meaning; false for the steps of an encoding, Skolem's and
    /// Herbrand's terms, and the moves of formulas between hypotheses.
    pub const fn is_equivalence(self) -> bool {
        !matches!(
            self,
            Self::Negate
                | Self::Define
                | Self::TrueHypothesis
                | Self::Skolemize
                | Self::Herbrandize
                | Self::SplitConjunction
        )
    }
}

impl Display for Rule {
    /// Writes the rule's [`name`](Self::name).
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

/// One step of a rewriting. Its ids are formulas of the result's arena
/// ([`NormalForm::result`]), which holds every formula of every step.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    /// The law applied.
    pub rule: Rule,
    /// The subformula rewritten. Every occurrence of it is rewritten at
    /// once, as equal formulas are one node of the arena. For
    /// [`Rule::Negate`] the formula moved.
    pub before: NodeId,
    /// What replaces it: for [`Rule::Define`] the new atom, for
    /// [`Rule::Negate`] the negated formula, for [`Rule::TrueHypothesis`]
    /// the `⊤` left out, as `before`.
    pub after: NodeId,
    /// The formulas of the input as they stand after the step, left of
    /// `⊢`: the hypotheses, and for an encoding the negated formulas from
    /// the right.
    pub left: Vec<NodeId>,
    /// The formulas of the input as they stand after the step, right of
    /// `⊢`.
    pub right: Vec<NodeId>,
    /// The clauses the step adds as hypotheses (for [`Rule::Define`]). The
    /// sequent after a step is [`left`](Self::left) with the clauses of
    /// this step and every earlier one, then `⊢` and
    /// [`right`](Self::right).
    pub added: Vec<NodeId>,
    /// The term a [`Rule::Skolemize`] or [`Rule::Herbrandize`] step puts in
    /// for the quantifier's variable, a new function symbol applied to the
    /// variables free in the quantified formula (de Bruijn indices read
    /// outside it, the outermost first); `None` for every other step.
    pub term: Option<FoTermId>,
}

/// A claim whose validity checks a promise.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "snake_case"))]
pub enum Claim {
    /// `⊢ (A₁ ↔ A₁') ∧ …`: every formula is equivalent to its normal form.
    Equivalent,
    /// The input's negation and the definitions conclude the clauses: a
    /// model of the input's negation, extended by the definitions, is one
    /// of the clauses.
    Extends,
    /// The clauses conclude the input's negation: a model of the clauses,
    /// restricted to the input's atoms, is one of the input's negation.
    Restricts,
    /// The law of one step holds of the subformula it rewrote: `⊢ C ↔ C'`
    /// for the formula `C` replaced by `C'`, each of their free variables
    /// a new constant, decided in intuitionistic logic where the law holds
    /// there.
    Law,
    /// The input implies the result, side by side: `(∧Γ → ∨Δ) ⊢ (∧Γ' →
    /// ∨Δ')`, so a valid input gives a valid result.
    Implied,
}

impl Claim {
    /// Returns the claim's name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Equivalent => "equivalent",
            Self::Extends => "extends",
            Self::Restricts => "restricts",
            Self::Law => "law",
            Self::Implied => "implied",
        }
    }

    /// Returns what the claim says, as a reader sees it.
    pub const fn description(self) -> &'static str {
        match self {
            Self::Equivalent => "every formula is equivalent to its normal form",
            Self::Extends => {
                "wherever the sequent is false, the clauses hold with the new atoms as defined"
            }
            Self::Restricts => "wherever the clauses hold, the sequent is false",
            Self::Law => "the step's law holds of the formula it rewrote",
            Self::Implied => "the input implies the result",
        }
    }
}

/// A claim decided.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Checked {
    /// The claim.
    pub claim: Claim,
    /// The step whose law a [`Claim::Law`] checks, by its index in
    /// [`NormalForm::steps`]; `None` for the other claims.
    pub step: Option<usize>,
    /// The sequent whose validity is the claim: classical, but for a law
    /// that holds in intuitionistic logic.
    pub sequent: Sequent,
    /// The sequent decided.
    pub outcome: Outcome,
}

/// A sequent's normal form with every step that made it, and the checks
/// of its promise where it was checked.
///
/// # JSON
///
/// With the feature `serialize` a normal form is the document
/// `{"version": 21, "deduxit": "0.8.0", "form": "cnf", "promise":
/// "equivalent", "checked": true, "sequent": {…}, "result": {…},
/// "steps": [{"rule": "implication", "law": "A → B ≡ ¬A ∨ B", "before":
/// 2, "after": 4, "left": [], "right": [4], "added": []}, …],
/// "checks": [{"claim": "equivalent", "sequent": {…}, "outcome": {…}}]}`
/// ([`wire`](crate::wire)), written only: `sequent` the input and
/// `result` the normal form in the form of a [`Sequent`], the steps' ids
/// indices into the result's `formulas`, and each check's outcome in the
/// form of an [`Outcome`] without a `version`.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct NormalForm {
    /// The form.
    pub form: Form,
    /// The input.
    pub sequent: Sequent,
    /// The steps, in order.
    pub steps: Vec<Step>,
    /// The normal form. Its arena holds every formula of the steps.
    pub result: Sequent,
    /// The claims decided, empty where the promise was not checked.
    pub checks: Vec<Checked>,
}

impl NormalForm {
    /// Returns whether every claim of the promise was decided valid. False
    /// where a check stopped before it decided, or none was made.
    pub fn is_checked(&self) -> bool {
        !self.checks.is_empty()
            && self
                .checks
                .iter()
                .all(|c| matches!(c.outcome.verdict, Verdict::Valid(_)))
    }

    /// Returns the definitions of an encoding: every new atom with the
    /// formula it names, in the order they were made.
    pub fn definitions(&self) -> impl Iterator<Item = (NodeId, NodeId)> + '_ {
        self.steps
            .iter()
            .filter(|s| s.rule == Rule::Define)
            .map(|s| (s.after, s.before))
    }

    /// Returns the claims whose validity checks the promise, each as a
    /// classical sequent.
    ///
    /// # Errors
    ///
    /// [`Error::ClaimFails`] where an encoding's definitions do not name
    /// new atoms, each by the input's atoms and earlier ones, or its
    /// result has a formula right of `⊢`; [`Error::IndexOutOfBounds`] for
    /// a step's id outside the result's arena; the arena's errors.
    pub fn claims(&self) -> Result<Vec<(Claim, Sequent)>, Error> {
        // Its fields are public, so a first-order sequent can be set.
        if !self.form.takes_first_order() {
            self.sequent.formulas().propositional(self.form.title())?;
            self.result.formulas().propositional(self.form.title())?;
        }
        let mut formulas = self.result.formulas().clone();
        let (left, right) = transfer(&self.sequent, &mut formulas)?;
        match self.form.promise() {
            Promise::Equivalent => {
                if left.len() != self.result.left().len()
                    || right.len() != self.result.right().len()
                {
                    return Err(Error::ClaimFails {
                        claim: self.form.title().to_owned(),
                        reason: "the result has other formulas than the input".to_owned(),
                    });
                }
                let pairs = left
                    .iter()
                    .zip(self.result.left())
                    .chain(right.iter().zip(self.result.right()));
                let mut each = Vec::new();
                for (&a, &b) in pairs {
                    each.push(formulas.add(Node::Iff(a, b))?);
                }
                let all = conjunction(&mut formulas, &each)?;
                Ok(vec![(
                    Claim::Equivalent,
                    Sequent::new(formulas, Vec::new(), vec![all])?,
                )])
            }
            Promise::Equisatisfiable if matches!(self.form, Form::Skolem | Form::Clausal) => {
                if !self.result.right().is_empty() {
                    return Err(Error::ClaimFails {
                        claim: self.form.title().to_owned(),
                        reason: "the result has formulas right of ⊢".to_owned(),
                    });
                }
                self.skolem_fresh(&formulas)?;
                // A model of the result is one of the input's negation:
                // each Skolem term is a witness of its `∃`. The converse,
                // that a model of the input's negation expands to one of
                // the result, needs choice and has no derivation; it rests
                // on the construction the check above confirms.
                let mut negation = left;
                for &d in &right {
                    negation.push(formulas.add(Node::Not(d))?);
                }
                let input = conjunction(&mut formulas, &negation)?;
                Ok(vec![(
                    Claim::Restricts,
                    Sequent::new(formulas, self.result.left().to_vec(), vec![input])?,
                )])
            }
            Promise::Equivalid => {
                self.skolem_fresh(&formulas)?;
                // The input implies the result side by side, so a valid
                // input gives a valid result; the converse rests on the
                // construction the check above confirms.
                let (gamma, delta) = (
                    conjunction(&mut formulas, &left)?,
                    disjunction(&mut formulas, &right)?,
                );
                let input = formulas.add(Node::Implies(gamma, delta))?;
                let (gamma, delta) = (
                    conjunction(&mut formulas, self.result.left())?,
                    disjunction(&mut formulas, self.result.right())?,
                );
                let output = formulas.add(Node::Implies(gamma, delta))?;
                Ok(vec![(
                    Claim::Implied,
                    Sequent::new(formulas, vec![input], vec![output])?,
                )])
            }
            Promise::Equisatisfiable => {
                // Clauses stand left of `⊢`: a formula right of it would
                // change what the result's validity says.
                if !self.result.right().is_empty() {
                    return Err(Error::ClaimFails {
                        claim: self.form.title().to_owned(),
                        reason: "the result has formulas right of ⊢".to_owned(),
                    });
                }
                let roots: Vec<NodeId> = left.iter().chain(&right).copied().collect();
                self.fresh(&formulas, &roots)?;
                let mut negation = left.clone();
                for &d in &right {
                    negation.push(formulas.add(Node::Not(d))?);
                }
                let mut extended = negation.clone();
                for (atom, named) in self.definitions() {
                    extended.push(formulas.add(Node::Iff(atom, named))?);
                }
                let clauses = conjunction(&mut formulas, self.result.left())?;
                let input = conjunction(&mut formulas, &negation)?;
                Ok(vec![
                    (
                        Claim::Extends,
                        Sequent::new(formulas.clone(), extended, vec![clauses])?,
                    ),
                    (
                        Claim::Restricts,
                        Sequent::new(formulas, self.result.left().to_vec(), vec![input])?,
                    ),
                ])
            }
        }
    }

    /// Fails unless every Skolem or Herbrand step put in for its
    /// quantifier's variable a term of a new function symbol (in no formula
    /// of the input, made by no other step) applied to exactly the
    /// variables free in the quantified formula, the outermost first, and
    /// made of it what that substitution makes. Those are the hypotheses
    /// of Skolem's theorem: then a model of the input expands to one of the
    /// result.
    fn skolem_fresh(&self, formulas: &Formulas) -> Result<(), Error> {
        self.replay()?;
        let fails = |reason: String| Error::ClaimFails {
            claim: self.form.title().to_owned(),
            reason,
        };
        let input = self.sequent.formulas().first_order();
        let input_names: crate::hash::HashSet<&str> = input
            .symbols
            .iter()
            .map(|(name, _)| name.as_str())
            .chain(
                self.sequent
                    .formulas()
                    .atom_names()
                    .iter()
                    .map(String::as_str),
            )
            .collect();
        let tables = formulas.first_order();
        let mut made = crate::hash::HashSet::default();
        let mut arena = formulas.clone();
        for step in &self.steps {
            if !matches!(step.rule, Rule::Skolemize | Rule::Herbrandize) {
                continue;
            }
            formulas.holds(step.before)?;
            formulas.holds(step.after)?;
            let Some(term) = step.term else {
                return Err(fails("a step of a Skolem term without its term".to_owned()));
            };
            let Some(&FoTerm::App(symbol, args)) = tables.terms.get(term.index()) else {
                return Err(fails(
                    "a Skolem term that is no function symbol's".to_owned(),
                ));
            };
            let (name, arity) = &tables.symbols[symbol.index()];
            if input_names.contains(name.as_str()) || !made.insert(symbol) {
                return Err(fails(format!("the symbol {name} is not new")));
            }
            let (Node::Forall(body) | Node::Exists(body)) = formulas.node(step.before) else {
                return Err(fails("a Skolem term replaces no quantifier".to_owned()));
            };
            let free = formulas.free_variables(step.before);
            let wanted: Vec<FoTerm> = free.iter().rev().map(|&i| FoTerm::Bound(i)).collect();
            let given: Vec<FoTerm> = tables
                .list(args)
                .iter()
                .map(|t| tables.terms[t.index()])
                .collect();
            if given != wanted || *arity as usize != free.len() {
                return Err(fails(format!(
                    "the symbol {name} is not applied to the variables free in its quantifier"
                )));
            }
            if arena.substituted(body, term)? != step.after {
                return Err(fails(format!(
                    "the step of {name} does not put its term in for the variable"
                )));
            }
        }
        Ok(())
    }

    /// Fails unless the steps, made again one by one from the input in the
    /// result's arena, make the sequents they record and the result: each
    /// equivalence step its law where the law applies, each Skolem or
    /// Herbrand step its term (a symbol the sequent before it does not
    /// hold) put in for its quantifier on the side it is strong, each move
    /// of a formula the move. So a result set by hand, or a step forged
    /// between two others, is not judged as the engine's own.
    fn replay(&self) -> Result<(), Error> {
        let fails = |reason: String| Error::ClaimFails {
            claim: self.form.title().to_owned(),
            reason,
        };
        let input = self.sequent.formulas();
        let mut arena = self.result.formulas().clone();
        let extends = input.len() <= arena.len()
            && (0..input.len())
                .all(|k| input.node(NodeId(k as u32)) == arena.node(NodeId(k as u32)))
            && arena.atom_names().starts_with(input.atom_names());
        if !extends {
            return Err(fails("the result is not over the input's arena".to_owned()));
        }
        let (mut left, mut right) = (self.sequent.left().to_vec(), self.sequent.right().to_vec());
        let mut passes = Passes::default();
        for (k, step) in self.steps.iter().enumerate() {
            arena.holds(step.before)?;
            arena.holds(step.after)?;
            let wrong = || {
                fails(format!(
                    "step {} does not make the sequent it records",
                    k + 1
                ))
            };
            match step.rule {
                Rule::Negate => {
                    if right.first() != Some(&step.before)
                        || arena.find(Node::Not(step.before)) != Some(step.after)
                    {
                        return Err(wrong());
                    }
                    right.remove(0);
                    left.push(step.after);
                }
                Rule::SplitConjunction => {
                    let at = left
                        .iter()
                        .position(|&r| r == step.before)
                        .ok_or_else(wrong)?;
                    let Node::And(a, b) = arena.node(step.before) else {
                        return Err(wrong());
                    };
                    left.splice(at..=at, [a, b]);
                }
                Rule::TrueHypothesis => {
                    let at = left
                        .iter()
                        .position(|&r| r == step.before)
                        .ok_or_else(wrong)?;
                    if !matches!(arena.node(step.before), Node::True) {
                        return Err(wrong());
                    }
                    left.remove(at);
                }
                Rule::Skolemize | Rule::Herbrandize => {
                    let Some(&FoTerm::App(symbol, _)) = step
                        .term
                        .and_then(|t| arena.first_order().terms.get(t.index()))
                    else {
                        return Err(wrong());
                    };
                    let all: Vec<NodeId> = left.iter().chain(&right).copied().collect();
                    if holds_symbol(&arena, &all, symbol) {
                        return Err(fails(format!(
                            "step {} names a symbol the sequent before it holds",
                            k + 1
                        )));
                    }
                    // Strong where it stands: `∃` left of `⊢`, `∀` right.
                    let side = match arena.node(step.before) {
                        Node::Exists(_) => &mut left,
                        Node::Forall(_) => &mut right,
                        Node::Atom(_)
                        | Node::True
                        | Node::False
                        | Node::Not(_)
                        | Node::And(..)
                        | Node::Or(..)
                        | Node::Implies(..)
                        | Node::Iff(..) => return Err(wrong()),
                    };
                    passes.reach(&arena, side);
                    *side = passes.substitute(&mut arena, side, step.before, step.after)?;
                }
                Rule::Define => return Err(wrong()),
                rule @ (Rule::Implication
                | Rule::Equivalence
                | Rule::EquivalenceAsCases
                | Rule::DoubleNegation
                | Rule::DeMorganAnd
                | Rule::DeMorganOr
                | Rule::NotTrue
                | Rule::NotFalse
                | Rule::AndTrue
                | Rule::AndFalse
                | Rule::OrTrue
                | Rule::OrFalse
                | Rule::DistributeOr
                | Rule::DistributeAnd
                | Rule::NotForall
                | Rule::NotExists
                | Rule::ForallAnd
                | Rule::ExistsAnd
                | Rule::ForallOr
                | Rule::ExistsOr
                | Rule::ForallAntecedent
                | Rule::ExistsAntecedent
                | Rule::ForallConsequent
                | Rule::ExistsConsequent
                | Rule::EquivalenceAsImplications
                | Rule::ForallDistribute
                | Rule::ExistsDistribute
                | Rule::Vacuous) => {
                    let applies = REPLAYED
                        .iter()
                        .any(|&stage| law(stage, &arena, step.before) == Some(rule));
                    if !applies || apply(rule, &mut arena, step.before)? != step.after {
                        return Err(fails(format!(
                            "the law of step {} does not make what it records",
                            k + 1
                        )));
                    }
                    let mut roots: Vec<NodeId> = left.iter().chain(&right).copied().collect();
                    passes.reach(&arena, &roots);
                    roots = passes.substitute(&mut arena, &roots, step.before, step.after)?;
                    let split = left.len();
                    (left, right) = (roots[..split].to_vec(), roots[split..].to_vec());
                }
            }
            if left != step.left || right != step.right {
                return Err(wrong());
            }
        }
        if left != self.result.left() || right != self.result.right() {
            return Err(fails("the result is not what the steps make".to_owned()));
        }
        Ok(())
    }

    /// Fails unless every definition names a new atom, one the input does
    /// not hold and no earlier definition named, by a formula over the
    /// input's atoms and earlier new atoms: then every model of the
    /// input's atoms extends to one of the definitions.
    fn fresh(&self, formulas: &Formulas, roots: &[NodeId]) -> Result<(), Error> {
        let fails = |reason: String| Error::ClaimFails {
            claim: self.form.title().to_owned(),
            reason,
        };
        // The rank of each atom: 0 for the input's, k for the k-th
        // definition's, `NONE` for any other. A definition may use the
        // atoms of a lower rank alone. One pass of each kind, so the check
        // is linear in the arena and the definitions.
        const NONE: u64 = u64::MAX;
        let input = super::interpolant::atoms_of(formulas, roots);
        let mut rank: Vec<u64> = input.iter().map(|&i| if i { 0 } else { NONE }).collect();
        let definitions: Vec<(NodeId, NodeId)> = self.definitions().collect();
        for (k, &(atom, named)) in (1u64..).zip(&definitions) {
            formulas.holds(atom)?;
            formulas.holds(named)?;
            let Node::Atom(a) = formulas.node(atom) else {
                return Err(fails(format!("{} names no atom", formulas.formula(atom))));
            };
            if rank[a as usize] != NONE {
                return Err(fails(format!(
                    "the atom {} is not new",
                    formulas.atom_name(a)
                )));
            }
            rank[a as usize] = k;
        }
        // The highest rank of an atom each formula holds; its operands
        // come before it.
        let mut highest: Vec<u64> = Vec::with_capacity(formulas.len());
        for id in 0..formulas.len() {
            let node = formulas.node(NodeId(id as u32));
            let of = |k: Option<NodeId>| k.map_or(0, |k| highest[k.index()]);
            highest.push(match node {
                Node::Atom(a) => rank[a as usize],
                Node::True | Node::False => 0,
                Node::Not(_) | Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => {
                    let (a, b) = node.operands();
                    of(a).max(of(b))
                }
                Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
            });
        }
        for (k, &(atom, named)) in (1u64..).zip(&definitions) {
            if highest[named.index()] >= k {
                let used = super::interpolant::atoms_of(formulas, &[named]);
                let late = (0..used.len())
                    .find(|&j| used[j] && rank[j] >= k)
                    .unwrap_or(0);
                return Err(fails(format!(
                    "the definition of {} names {}, which is defined later or not at all",
                    formulas.formula(atom),
                    formulas.atom_name(late as u32)
                )));
            }
        }
        Ok(())
    }

    /// Decides every claim of the promise in classical logic by the SAT
    /// solver (with the cargo feature `sat`, else by the logic's default
    /// method), under `search`, `limits` and one stop, and keeps the
    /// outcomes.
    ///
    /// # Errors
    ///
    /// [`Error::ClaimFails`] for a claim that is not valid, or definitions
    /// that are not fresh; [`decide_within`]'s errors.
    pub fn check(
        &mut self,
        search: &search::Options,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        self.checked(search, limits, &mut stop)
    }

    /// [`check`](Self::check) with the stop as a trait object, so that it
    /// is compiled once, not once per caller's closure.
    fn checked(
        &mut self,
        search: &search::Options,
        limits: &Limits,
        mut stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        // The SAT solver decides propositional claims; at first order the
        // logic's default does.
        let first_order = self.sequent.formulas().is_first_order();
        let given = limits.memory_bytes;
        let method = if first_order { None } else { CLAIMS };
        let options = super::Options::default()
            .with_logic(Logic::Classical)
            .with_method(method);
        // A value checked before and changed since keeps no old check.
        self.checks.clear();
        let mut checks = Vec::new();
        // What the checks kept hold: each decision gets the memory left, and
        // a bound reached is named as the caller gave it.
        let mut held = 0u64;
        if first_order {
            for (k, step) in self.steps.iter().enumerate() {
                if !step.rule.is_equivalence() {
                    continue;
                }
                if stop(Progress::new(Phase::Check, 1, k as u64)) {
                    return Err(Error::Refused(Refusal::stopped(Phase::Check)));
                }
                let limits = &left_over(limits, held);
                let sequent = law_claim(&self.result, step)?;
                // The claim is valid in its logic by the law's table, so a
                // search that deepens without a copy bound ends; the
                // call's limits and stop bound it all the same.
                let lifted = search.clone().with_copies(None);
                let classical = super::Options::default().with_logic(Logic::Classical);
                let outcome = if step.rule.intuitionistic() {
                    // A derivation of LJ where one comes within the law's
                    // own work, else of LK, which checks the classical
                    // equivalence the form promises: an intuitionistic
                    // search that does not end would starve every claim
                    // after it.
                    let bounded =
                        limits.with_work(Some(limits.work.map_or(LAW_WORK, |w| w.min(LAW_WORK))));
                    let options = super::Options::default().with_logic(Logic::Intuitionistic);
                    // A bound that refuses LJ's derivation (its read-back of
                    // a deep nest of binders grows with the square of the
                    // nest) gives way to LK as an unknown does.
                    match decide_within(&sequent, &options, &lifted, &bounded, &mut stop) {
                        Ok(tried)
                            if !matches!(
                                tried.verdict,
                                Verdict::Unknown(ref reason) if *reason != search::Reason::Stopped
                            ) =>
                        {
                            tried
                        }
                        Ok(_)
                        | Err(Error::Refused(Refusal::Memory { .. } | Refusal::Work { .. })) => {
                            decide_within(&sequent, &classical, &lifted, limits, &mut stop)
                                .map_err(|e| given_refusal(e, given, held))?
                        }
                        Err(error) => return Err(given_refusal(error, given, held)),
                    }
                } else {
                    decide_within(&sequent, &classical, &lifted, limits, &mut stop)
                        .map_err(|e| given_refusal(e, given, held))?
                };
                if matches!(outcome.verdict, Verdict::NotValid) {
                    return Err(Error::ClaimFails {
                        claim: self.form.title().to_owned(),
                        reason: format!("the law {} fails at step {}", step.rule.law(), k + 1),
                    });
                }
                let checked = Checked {
                    claim: Claim::Law,
                    step: Some(k),
                    sequent,
                    outcome: given_bound(outcome, given),
                };
                held = held.saturating_add(checked_bytes(&checked));
                kept_within(held, given)?;
                checks.push(checked);
            }
        }
        for (claim, sequent) in self.claims()? {
            let limits = &left_over(limits, held);
            let outcome = decide_within(&sequent, &options, search, limits, &mut stop);
            let outcome = given_bound(outcome.map_err(|e| given_refusal(e, given, held))?, given);
            if matches!(outcome.verdict, Verdict::NotValid) {
                return Err(Error::ClaimFails {
                    claim: self.form.title().to_owned(),
                    reason: format!("it is not so that {}", claim.description()),
                });
            }
            let checked = Checked {
                claim,
                step: None,
                sequent,
                outcome,
            };
            held = held.saturating_add(checked_bytes(&checked));
            kept_within(held, given)?;
            checks.push(checked);
        }
        self.checks = checks;
        Ok(())
    }
}

/// The work an intuitionistic decision of a law's claim may take before
/// the claim is decided in classical logic.
const LAW_WORK: u64 = 1 << 20;

/// Every phase whose laws a replayed step may apply.
const REPLAYED: [Stage; 11] = [
    Stage::Arrows { cases: false },
    Stage::Arrows { cases: true },
    Stage::Negations,
    Stage::Constants,
    Stage::Distribute { conjunctive: true },
    Stage::Distribute { conjunctive: false },
    Stage::Iffs,
    Stage::Outward { universal: false },
    Stage::Outward { universal: true },
    Stage::Inward,
    Stage::Split,
];

/// Returns whether a formula of `roots` holds a term of `symbol`.
fn holds_symbol(formulas: &Formulas, roots: &[NodeId], symbol: crate::first_order::Symbol) -> bool {
    let tables = formulas.first_order();
    let mut seen = crate::hash::HashSet::default();
    let mut terms: Vec<FoTermId> = Vec::new();
    let mut stack = roots.to_vec();
    while let Some(k) = stack.pop() {
        if !seen.insert(k) {
            continue;
        }
        let node = formulas.node(k);
        if let Node::Atom(a) = node
            && let Some(&(_, args)) = tables.atoms.get(a as usize)
        {
            terms.extend_from_slice(tables.list(args));
        }
        let (a, b) = node.operands();
        stack.extend([a, b].into_iter().flatten());
    }
    let mut seen_terms = crate::hash::HashSet::default();
    while let Some(t) = terms.pop() {
        if !seen_terms.insert(t) {
            continue;
        }
        if let FoTerm::App(f, args) = tables.terms[t.index()] {
            if f == symbol {
                return true;
            }
            terms.extend_from_slice(tables.list(args));
        }
    }
    false
}

/// Returns `limits` with the memory bound less what the checks kept hold.
fn left_over(limits: &Limits, held: u64) -> Limits {
    limits.with_memory_bytes(limits.memory_bytes.map(|m| m.saturating_sub(held)))
}

/// Refuses checks that together hold more than the memory bound `given`.
fn kept_within(held: u64, given: Option<u64>) -> Result<(), Error> {
    match given {
        Some(limit_bytes) if held > limit_bytes => Err(Error::Refused(Refusal::Memory {
            phase: Phase::Check,
            limit_bytes,
            needed_bytes: Some(held),
        })),
        _ => Ok(()),
    }
}

/// Returns the bytes a check holds, estimated: its claim's and its
/// derivation's arenas, and the derivation's inferences.
fn checked_bytes(checked: &Checked) -> u64 {
    let arena = |formulas| ArenaSize::of(formulas, &ArenaSize::default()).bytes;
    let mut bytes = arena(checked.sequent.formulas());
    if let Verdict::Valid(super::Evidence::Derivation(d)) = &checked.outcome.verdict {
        bytes = bytes.saturating_add(arena(d.formulas()));
        for inference in d.inferences() {
            let ids = inference.left.len() + inference.right.len() + inference.premises.len();
            bytes =
                bytes.saturating_add((size_of::<super::derivation::Inference>() + 4 * ids) as u64);
        }
    }
    bytes
}

/// Returns `outcome` with an unknown at the memory bound naming the bound
/// the caller gave, not the share a claim's decision had.
fn given_bound(mut outcome: Outcome, given: Option<u64>) -> Outcome {
    if let (Verdict::Unknown(search::Reason::MemoryLimit { limit_bytes }), Some(g)) =
        (&mut outcome.verdict, given)
    {
        *limit_bytes = g;
    }
    outcome
}

/// Returns `error` with a refusal at the memory bound naming the bound the
/// caller gave, and what the decision needed beside the `held` bytes the
/// checks before it kept.
fn given_refusal(error: Error, given: Option<u64>, held: u64) -> Error {
    match (error, given) {
        (
            Error::Refused(Refusal::Memory {
                phase,
                needed_bytes,
                ..
            }),
            Some(limit_bytes),
        ) => Error::Refused(Refusal::Memory {
            phase,
            limit_bytes,
            needed_bytes: needed_bytes.map(|n| n.saturating_add(held)),
        }),
        (error, _) => error,
    }
}

/// Returns the claim of a step's law: `⊢ C ↔ C'`, `C` the formula the
/// step replaced and `C'` what replaced it, each of their free variables a
/// new constant (`c1`, `c2`, …, the innermost binder's first). It is valid
/// exactly when `∀ȳ. (C ↔ C')` is, and the methods decide it with no
/// quantifier to instantiate for `ȳ`. The claim's arena holds the two
/// formulas alone: a copy of the result's, which grows with every step,
/// made the claims of a long rewriting take the square of its size.
fn law_claim(result: &Sequent, step: &Step) -> Result<Sequent, Error> {
    let source = result.formulas();
    source.holds(step.before)?;
    source.holds(step.after)?;
    let mut formulas = Formulas::default();
    let copied = copy_formulas(source, &[step.before, step.after], &mut formulas)?;
    let (before, after) = (copied[0], copied[1]);
    let needed = formulas
        .free_variables(before)
        .into_iter()
        .chain(formulas.free_variables(after))
        .max()
        .map_or(0, |i| i + 1);
    let mut claim = formulas.add(Node::Iff(before, after))?;
    if needed > 0 {
        let prefix = fresh_prefix(&formulas, "c");
        let mut constants = Vec::with_capacity(needed as usize);
        for k in 1..=needed {
            constants.push(skolem_term(&mut formulas, &format!("{prefix}{k}"), &[])?);
        }
        claim = formulas.instantiate_frame(claim, &constants)?;
    }
    Sequent::new(formulas, Vec::new(), vec![claim])
}

/// Returns `roots`, formulas of `from` (open ones among them), made again in
/// `to` with every node and term they reach and nothing else: atoms and
/// symbols by name, a bound variable by its index, a binder by its name.
/// It is not `transfer::copy`: that builds in index order, and a claim's
/// document numbers its formulas in the order they were made, which this
/// walk (depth first from the roots) fixes.
fn copy_formulas(
    from: &Formulas,
    roots: &[NodeId],
    to: &mut Formulas,
) -> Result<Vec<NodeId>, Error> {
    let tables = from.first_order();
    let mut symbols: HashMap<crate::first_order::Symbol, crate::first_order::Symbol> =
        HashMap::new();
    let mut terms: HashMap<FoTermId, FoTermId> = HashMap::new();
    let mut nodes: HashMap<NodeId, NodeId> = HashMap::new();
    let mut symbol_in = |to: &mut Formulas,
                         s: crate::first_order::Symbol,
                         predicate: bool|
     -> Result<crate::first_order::Symbol, Error> {
        if let Some(&t) = symbols.get(&s) {
            return Ok(t);
        }
        let (name, arity) = &tables.symbols[s.index()];
        let t = to
            .interner()
            .symbol(name, *arity, predicate)
            .map_err(|fault| Error::Symbol {
                name: name.clone(),
                fault,
            })?;
        symbols.insert(s, t);
        Ok(t)
    };
    let mut stack: Vec<(NodeId, bool)> = roots.iter().map(|&r| (r, false)).collect();
    while let Some((n, ready)) = stack.pop() {
        if nodes.contains_key(&n) {
            continue;
        }
        let node = from.node(n);
        if !ready {
            stack.push((n, true));
            let (a, b) = node.operands();
            stack.extend([a, b].into_iter().flatten().map(|k| (k, false)));
            continue;
        }
        let image = match node {
            Node::Atom(a) => match tables.atoms.get(a as usize) {
                Some(&(p, args)) => {
                    // The terms after the terms they name.
                    let mut pending: Vec<(FoTermId, bool)> =
                        tables.list(args).iter().map(|&t| (t, false)).collect();
                    while let Some((t, done)) = pending.pop() {
                        if terms.contains_key(&t) {
                            continue;
                        }
                        let made = match tables.terms[t.index()] {
                            FoTerm::Bound(i) => to.interner().term(FoTerm::Bound(i)),
                            FoTerm::App(f, list) => {
                                let items = tables.list(list);
                                if !done {
                                    pending.push((t, true));
                                    pending.extend(items.iter().map(|&u| (u, false)));
                                    continue;
                                }
                                let items: Vec<FoTermId> = items.iter().map(|u| terms[u]).collect();
                                let f = symbol_in(to, f, false)?;
                                let list = to.interner().list(&items);
                                to.interner().term(FoTerm::App(f, list))
                            }
                            // A result's arena holds neither.
                            FoTerm::Eigen(_) | FoTerm::Meta(_) => {
                                return Err(Error::ClaimFails {
                                    claim: "a normal form".to_owned(),
                                    reason: "a step holds an eigenvariable or an open witness"
                                        .to_owned(),
                                });
                            }
                        };
                        terms.insert(t, made);
                    }
                    let items: Vec<FoTermId> = tables.list(args).iter().map(|u| terms[u]).collect();
                    let p = symbol_in(to, p, true)?;
                    let list = to.interner().list(&items);
                    to.predicate_atom(p, list)?
                }
                None => to.atom(from.atom_name(a))?,
            },
            Node::True | Node::False => to.add(node)?,
            Node::Not(a) => to.add(Node::Not(nodes[&a]))?,
            Node::And(a, b) => to.add(Node::And(nodes[&a], nodes[&b]))?,
            Node::Or(a, b) => to.add(Node::Or(nodes[&a], nodes[&b]))?,
            Node::Implies(a, b) => to.add(Node::Implies(nodes[&a], nodes[&b]))?,
            Node::Iff(a, b) => to.add(Node::Iff(nodes[&a], nodes[&b]))?,
            Node::Forall(a) => to.add_named(Node::Forall(nodes[&a]), tables.binder_name(n))?,
            Node::Exists(a) => to.add_named(Node::Exists(nodes[&a]), tables.binder_name(n))?,
        };
        nodes.insert(n, image);
    }
    Ok(roots.iter().map(|r| nodes[r]).collect())
}

/// Returns the formulas of `sequent` added to `formulas`, built again by
/// their atoms' names, so that they are formulas of that arena whatever
/// arena the sequent has. A sequent's arena holds no eigenvariable or open
/// witness, so one is [`Error::ClaimFails`], before anything is copied.
fn transfer(
    sequent: &Sequent,
    formulas: &mut Formulas,
) -> Result<(Vec<NodeId>, Vec<NodeId>), Error> {
    let from = sequent.formulas();
    let parameter = |term: &FoTerm| matches!(term, FoTerm::Eigen(_) | FoTerm::Meta(_));
    if from.first_order().terms.iter().any(parameter) {
        return Err(Error::ClaimFails {
            claim: "a normal form".to_owned(),
            reason: "the input holds an eigenvariable or an open witness".to_owned(),
        });
    }
    let copied = transfer::copy_first_order(formulas, from, None, &Parameters::Constants)?;
    let map = |ids: &[NodeId]| ids.iter().map(|&x| copied.node(x)).collect();
    Ok((map(sequent.left()), map(sequent.right())))
}

/// The method that decides a promise's claims: the SAT solver, whose
/// clausal proof the crate's checker accepts before a claim is valid;
/// without it the logic's default.
#[cfg(feature = "sat")]
const CLAIMS: Option<super::Method> = Some(super::Method::Sat);
/// The method that decides a promise's claims: the SAT solver, whose
/// clausal proof the crate's checker accepts before a claim is valid;
/// without it the logic's default.
#[cfg(not(feature = "sat"))]
const CLAIMS: Option<super::Method> = None;

/// Returns the conjunction of `ids`, the first leftmost, or `⊤` for none.
fn conjunction(formulas: &mut Formulas, ids: &[NodeId]) -> Result<NodeId, Error> {
    let mut all: Option<NodeId> = None;
    for &id in ids {
        all = Some(match all {
            None => id,
            Some(left) => formulas.add(Node::And(left, id))?,
        });
    }
    match all {
        Some(id) => Ok(id),
        None => formulas.add(Node::True),
    }
}

/// Returns the disjunction of `ids`, the first leftmost, or `⊥` for none.
fn disjunction(formulas: &mut Formulas, ids: &[NodeId]) -> Result<NodeId, Error> {
    let mut any: Option<NodeId> = None;
    for &id in ids {
        any = Some(match any {
            None => id,
            Some(left) => formulas.add(Node::Or(left, id))?,
        });
    }
    match any {
        Some(id) => Ok(id),
        None => formulas.add(Node::False),
    }
}

/// Rewrites every formula of `sequent` into the normal form `options`
/// names, step by step, without checking the promise ([`normalize`]
/// does).
///
/// # Errors
///
/// [`Refusal::Form`] where the form or a sequent on the way unfolds past
/// `limits.form_occurrences`; [`Refusal::Work`] past `limits.work`, a unit
/// a formula a pass reaches; [`Refusal::Memory`] past
/// `limits.memory_bytes`; [`Refusal::Stopped`] where `stop` returns true,
/// asked once per step; [`Error::ClaimFails`] where the result is not in
/// the form, a defect; [`Refusal::Index`] when the arena is full.
pub fn rewrite(
    sequent: &Sequent,
    options: &Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<NormalForm, Error> {
    rewrite_dyn(sequent, options.form, limits, &mut stop)
}

/// [`rewrite`], with the stop as a trait object.
fn rewrite_dyn(
    sequent: &Sequent,
    form: Form,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<NormalForm, Error> {
    if !form.takes_first_order() {
        sequent.formulas().propositional(form.title())?;
    }
    let mut budget = Budget {
        limits,
        work: 0,
        held: 0,
        stop,
    };
    let (formulas, left, right, steps) = match form {
        Form::Skolem | Form::Herbrand | Form::Clausal => skolemized(sequent, form, &mut budget)?,
        Form::Nnf | Form::Cnf | Form::Dnf | Form::Prenex | Form::Miniscope => {
            equivalent(sequent, form, &mut budget)?
        }
        Form::Tseitin | Form::PlaistedGreenbaum => encoded(sequent, form, &mut budget)?,
    };
    let result = Sequent::new(formulas, left, right)?;
    if !in_form(&result, form) {
        return Err(Error::ClaimFails {
            claim: form.title().to_owned(),
            reason: format!("{result} is not in the form"),
        });
    }
    Ok(NormalForm {
        form,
        sequent: sequent.clone(),
        steps,
        result,
        checks: Vec::new(),
    })
}

/// What a rewriting makes: the arena, the two sides and the steps.
type Made = (Formulas, Vec<NodeId>, Vec<NodeId>, Vec<Step>);

/// Rewrites `sequent` into the normal form `options` names as [`rewrite`]
/// does, then checks the form's promise ([`NormalForm::check`]) under
/// `search`, `limits` and the same stop.
///
/// # Errors
///
/// [`rewrite`]'s errors and [`NormalForm::check`]'s.
pub fn normalize(
    sequent: &Sequent,
    options: &Options,
    search: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<NormalForm, Error> {
    let mut normal = rewrite_dyn(sequent, options.form, limits, &mut stop)?;
    normal.check(search, limits, &mut stop)?;
    Ok(normal)
}

/// Normalizes as [`normalize`] does within the default [`Limits`] and with
/// the linear search's default options.
///
/// # Errors
///
/// As [`normalize`].
pub fn normalize_default(sequent: &Sequent, options: &Options) -> Result<NormalForm, Error> {
    normalize(
        sequent,
        options,
        &search::Options::default(),
        &Limits::default(),
        |_| false,
    )
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;

    /// Returns the steps' rule names and the result's text.
    fn written(text: &str, form: Form) -> (Vec<&'static str>, String) {
        let sequent: Sequent = text.parse().unwrap();
        let options = Options::default().with_form(form);
        let normal = normalize_default(&sequent, &options).unwrap();
        assert!(normal.is_checked(), "{text} into {form}");
        let rules = normal.steps.iter().map(|s| s.rule.name()).collect();
        (rules, normal.result.to_string())
    }

    /// Each form applies its laws in their phases, the leftmost outermost
    /// place first, and its result is checked.
    #[test]
    fn each_form_writes_its_steps() {
        assert_eq!(
            written("a -> b /\\ c", Form::Cnf),
            (
                vec!["implication", "distribute-or"],
                "⊢ (¬a ∨ b) ∧ (¬a ∨ c)".to_owned()
            )
        );
        assert_eq!(
            written("~(a /\\ ~b) |- c <-> true", Form::Nnf),
            (
                vec![
                    "equivalence",
                    "de-morgan-and",
                    "double-negation",
                    "not-true"
                ],
                "¬a ∨ b ⊢ (¬c ∨ ⊤) ∧ (c ∨ ⊥)".to_owned()
            )
        );
        assert_eq!(
            written("a <-> b", Form::Dnf),
            (
                vec!["equivalence-as-cases"],
                "⊢ (a ∧ b) ∨ (¬a ∧ ¬b)".to_owned()
            )
        );
        assert_eq!(
            written("a /\\ b |- a", Form::Tseitin),
            (
                vec!["negate", "define"],
                "t1, ¬a, ¬t1 ∨ a, ¬t1 ∨ b, (t1 ∨ ¬a) ∨ ¬b ⊢".to_owned()
            )
        );
        // A constant unit becomes `⊥` or is left out, each a step.
        assert_eq!(
            written("|- true", Form::Tseitin),
            (vec!["negate", "not-true"], "⊥ ⊢".to_owned())
        );
        assert_eq!(
            written("|- false", Form::Tseitin),
            (
                vec!["negate", "not-false", "true-hypothesis"],
                "⊢".to_owned()
            )
        );
        assert_eq!(
            written("a /\\ b |- a", Form::PlaistedGreenbaum),
            (
                vec!["negate", "define"],
                "t1, ¬a, ¬t1 ∨ a, ¬t1 ∨ b ⊢".to_owned()
            )
        );
    }

    /// Every form of generated sequents keeps its promise, which its check
    /// confirms, and every step of an equivalent form is an equivalence.
    #[test]
    fn generated_sequents_keep_their_promises() {
        use crate::linear::search::generate::Rng;
        let mut rng = Rng::new(70);
        let mut checked = 0;
        for n in 0..60 {
            let sequent = super::super::tests::generated_sequent(&mut rng);
            for &form in Form::ALL {
                let options = Options::default().with_form(form);
                let normal = match normalize_default(&sequent, &options) {
                    Ok(normal) => normal,
                    Err(Error::Refused(Refusal::Form { .. })) => continue,
                    Err(error) => panic!("{sequent} into {form}: {error}"),
                };
                assert!(normal.is_checked(), "{sequent} into {form}");
                checked += 1;
                // The sequent after the last step is the result.
                if let Some(last) = normal.steps.last() {
                    let mut left = last.left.clone();
                    left.extend(normal.steps.iter().flat_map(|s| s.added.iter().copied()));
                    assert_eq!(left, normal.result.left(), "{sequent} into {form}");
                    assert_eq!(last.right, normal.result.right(), "{sequent} into {form}");
                }
                if n % 6 != 0 || form.promise() != Promise::Equivalent {
                    continue;
                }
                for step in &normal.steps {
                    let mut formulas = normal.result.formulas().clone();
                    let law = formulas.add(Node::Iff(step.before, step.after)).unwrap();
                    let claim = Sequent::new(formulas, Vec::new(), vec![law]).unwrap();
                    let outcome = super::super::decide(
                        &claim,
                        &super::super::Options::default().with_logic(Logic::Classical),
                    )
                    .unwrap();
                    assert!(
                        matches!(outcome.verdict, Verdict::Valid(_)),
                        "{sequent} into {form}: {}",
                        step.rule
                    );
                }
            }
        }
        assert!(checked > 200, "{checked} forms checked");
    }

    /// A form past `limits.form_occurrences` is refused; the bound counts
    /// the occurrences, and the rewriting stops at the first sequent past
    /// it.
    #[test]
    fn the_bound_refuses() {
        let sequent: Sequent = "a1 /\\ b1 \\/ a2 /\\ b2 \\/ a3 /\\ b3 \\/ a4 /\\ b4 \\/ a5 /\\ b5"
            .parse()
            .unwrap();
        let options = Options::default().with_form(Form::Cnf);
        let limits = Limits::default().with_form_occurrences(Some(100));
        assert!(matches!(
            rewrite(&sequent, &options, &limits, |_| false),
            Err(Error::Refused(Refusal::Form { limit: 100, occurrences })) if occurrences > 100
        ));
        let open = Limits::default().with_form_occurrences(Some(1_000));
        let cnf = rewrite(&sequent, &options, &open, |_| false).unwrap();
        // 32 clauses of 5 literals.
        assert_eq!(cnf.result.to_string().matches('∧').count(), 31);
    }

    /// The new atoms' names meet none of the input's.
    #[test]
    fn new_atoms_avoid_the_input() {
        let sequent: Sequent = "t1 /\\ t_1 |- t__2 \\/ t".parse().unwrap();
        let options = Options::default().with_form(Form::Tseitin);
        let normal = normalize_default(&sequent, &options).unwrap();
        let names = normal.result.formulas().atom_names();
        assert!(names.iter().any(|n| n == "t___1"), "{names:?}");
        assert!(normal.is_checked());
    }

    /// The check refuses a result that is not equivalent, and definitions
    /// that name no new atom.
    #[test]
    fn the_check_refuses_what_breaks_the_promise() {
        let search = search::Options::default();
        let limits = Limits::default();
        let sequent: Sequent = "a -> b".parse().unwrap();
        let mut cnf = rewrite(&sequent, &Options::default(), &limits, |_| false).unwrap();
        let wrong: Sequent = "a \\/ b".parse().unwrap();
        cnf.result = wrong;
        assert!(matches!(
            cnf.check(&search, &limits, |_| false),
            Err(Error::ClaimFails { .. })
        ));
        let options = Options::default().with_form(Form::Tseitin);
        let sequent: Sequent = "a /\\ b".parse().unwrap();
        let mut encoded = rewrite(&sequent, &options, &limits, |_| false).unwrap();
        // The definition `t1 ↔ a ∧ b` claimed to name the input's `a`.
        let a = encoded.result.formulas().find(Node::Atom(0)).unwrap();
        assert_eq!(encoded.steps[1].rule, Rule::Define);
        encoded.steps[1].after = a;
        assert!(matches!(
            encoded.check(&search, &limits, |_| false),
            Err(Error::ClaimFails { .. })
        ));
        // A step's id outside the arena is refused, not indexed.
        encoded.steps[1].after = NodeId(u32::MAX);
        assert!(matches!(
            encoded.check(&search, &limits, |_| false),
            Err(Error::IndexOutOfBounds { .. })
        ));
        // Clauses with a formula right of ⊢: `¬a ⊢ ¬a` is valid where
        // `⊢ a` is not, though both claims would hold.
        let sequent: Sequent = "a".parse().unwrap();
        let mut encoded = normalize(&sequent, &options, &search, &limits, |_| false).unwrap();
        assert!(encoded.is_checked());
        encoded.result = "~a |- ~a".parse().unwrap();
        assert!(matches!(
            encoded.check(&search, &limits, |_| false),
            Err(Error::ClaimFails { .. })
        ));
        // The failed check leaves no earlier check behind.
        assert!(!encoded.is_checked());
    }
}

#[cfg(all(test, feature = "parse"))]
pub(super) mod first_order_tests {
    use super::*;

    /// Returns the steps' rule names, the result's text and whether every
    /// claim was decided valid.
    fn made(text: &str, form: Form) -> (Vec<&'static str>, String, bool) {
        let sequent: Sequent = text.parse().unwrap();
        let options = Options::default().with_form(form);
        let normal = normalize_default(&sequent, &options).unwrap();
        let rules = normal.steps.iter().map(|s| s.rule.name()).collect();
        (rules, normal.result.to_string(), normal.is_checked())
    }

    /// Prenex form pulls every quantifier out, the leftmost outermost first,
    /// keeping `→`; each step's law is a checked derivation.
    #[test]
    fn prenex_form_pulls_the_quantifiers_out() {
        let (rules, result, checked) = made("|- (forall x. p(x)) -> exists y. q(y)", Form::Prenex);
        assert_eq!(rules, ["forall-antecedent", "exists-consequent"]);
        assert_eq!(result, "⊢ ∃x. ∃y. (p(x) → q(y))");
        assert!(checked);
        // A binder reaches as far right as it can: `∀x` holds the
        // conjunction.
        let (rules, result, checked) =
            made("(forall x. p(x)) /\\ ~exists y. q(y) |- r", Form::Prenex);
        assert_eq!(rules, ["forall-and", "not-exists", "forall-and"]);
        assert_eq!(result, "∀x. ∀y. (p(x) ∧ ¬q(y)) ⊢ r");
        assert!(checked);
    }

    /// Miniscoping moves each quantifier in as far as it goes.
    #[test]
    fn miniscoping_moves_the_quantifiers_in() {
        let (rules, result, checked) =
            made("|- forall x. (p(x) /\\ (q(x) \\/ r))", Form::Miniscope);
        assert_eq!(rules, ["forall-distribute", "forall-or"]);
        assert_eq!(result, "⊢ (∀x. p(x)) ∧ ((∀x. q(x)) ∨ r)");
        assert!(checked);
        let (_, result, _) = made("|- exists x. exists y. (p(x) /\\ q(y))", Form::Miniscope);
        assert_eq!(result, "⊢ (∃x. p(x)) ∧ (∃y. q(y))");
    }

    /// Skolemization replaces each `∃` of the sequent's negation by a new
    /// symbol over its free variables; equal formulas share it, two
    /// formulas of one variable do not.
    #[test]
    fn skolem_terms_take_the_free_variables() {
        let (rules, result, checked) = made("forall x. exists y. r(x, y) |- s", Form::Skolem);
        assert_eq!(rules, ["negate", "skolemize"]);
        assert_eq!(result, "∀x. r(x, sk1(x)), ¬s ⊢");
        assert!(checked);
        // Two `∃y` of different bodies get two symbols.
        let (_, result, _) = made("(exists y. p(y)) /\\ exists y. q(y) |-", Form::Skolem);
        assert_eq!(result, "p(sk1) ∧ q(sk2) ⊢");
        // The goal's `∀` is negated into an `∃` and gets its constant.
        let (_, result, _) = made("|- forall x. p(x)", Form::Skolem);
        assert_eq!(result, "¬p(sk1) ⊢");
    }

    /// Herbrandization keeps the sides and replaces the strong quantifiers.
    #[test]
    fn herbrand_terms_replace_the_strong_quantifiers() {
        let (rules, result, checked) = made(
            "exists x. p(x) |- exists y. forall z. q(y, z)",
            Form::Herbrand,
        );
        assert_eq!(rules, ["herbrandize", "herbrandize"]);
        assert_eq!(result, "p(sk1) ⊢ ∃y. q(y, sk2(y))");
        assert!(checked);
    }

    /// The clausal form gives each clause its own prefix as a hypothesis.
    #[test]
    fn the_clausal_form_writes_closed_clauses() {
        let (_, result, checked) = made(
            "forall x. (p(x) -> q(x) /\\ r(x)), p(a) |- exists y. r(y)",
            Form::Clausal,
        );
        assert_eq!(
            result,
            "∀x. (¬p(x) ∨ q(x)), ∀x. (¬p(x) ∨ r(x)), p(a), ∀y. ¬r(y) ⊢"
        );
        assert!(checked);
    }

    /// The symbols' prefix avoids the input's names.
    #[test]
    fn skolem_symbols_avoid_the_input() {
        let (_, result, _) = made("exists y. p(y, sk1) |-", Form::Skolem);
        assert_eq!(result, "p(sk_1, sk1) ⊢");
    }

    /// Returns a random first-order formula of at most `depth` levels over
    /// the variables of `scope`, a constant `a`, a unary `p`, a binary `q`
    /// and an atom `r`.
    pub(in crate::ordinary) fn formula(
        rng: &mut crate::linear::search::generate::Rng,
        depth: usize,
        scope: &mut Vec<String>,
    ) -> String {
        let term = |rng: &mut crate::linear::search::generate::Rng, scope: &[String]| {
            if scope.is_empty() || rng.one_in(4) {
                "a".to_owned()
            } else {
                scope[rng.below(scope.len())].clone()
            }
        };
        if depth == 0 || rng.one_in(4) {
            return match rng.below(3) {
                0 => format!("p({})", term(rng, scope)),
                1 => format!("q({}, {})", term(rng, scope), term(rng, scope)),
                _ => "r".to_owned(),
            };
        }
        match rng.below(7) {
            0 => format!("~({})", formula(rng, depth - 1, scope)),
            1..=4 => {
                let op = ["/\\", "\\/", "->", "<->"][rng.below(4)];
                let (x, y) = (
                    formula(rng, depth - 1, scope),
                    formula(rng, depth - 1, scope),
                );
                format!("({x} {op} {y})")
            }
            _ => {
                let quantifier = if rng.one_in(2) { "forall" } else { "exists" };
                let name = format!("x{}", scope.len());
                scope.push(name.clone());
                let body = formula(rng, depth - 1, scope);
                scope.pop();
                format!("({quantifier} {name}. {body})")
            }
        }
    }

    /// Every first-order form of generated sequents ends in its form with
    /// every claim valid, or unknown at a bound: never a claim decided not
    /// valid, which would be a defect.
    #[test]
    fn generated_sequents_keep_their_promises_at_first_order() {
        let mut rng = crate::linear::search::generate::Rng::new(77);
        let limits = Limits::default().with_work(Some(200_000));
        let search = search::Options::default();
        let (mut sequents, mut checked) = (0, 0);
        while sequents < 24 {
            let (left, right) = (
                formula(&mut rng, 3, &mut Vec::new()),
                formula(&mut rng, 3, &mut Vec::new()),
            );
            let Ok(sequent) = format!("{left} |- {right}").parse::<Sequent>() else {
                continue;
            };
            if !sequent.formulas().is_first_order() {
                continue;
            }
            sequents += 1;
            for &form in &[
                Form::Nnf,
                Form::Prenex,
                Form::Miniscope,
                Form::Skolem,
                Form::Herbrand,
                Form::Clausal,
            ] {
                let options = Options::default().with_form(form);
                match normalize(&sequent, &options, &search, &limits, |_| false) {
                    Ok(normal) => checked += usize::from(normal.is_checked()),
                    Err(Error::Refused(_)) => {}
                    Err(error) => panic!("{sequent} into {form}: {error}"),
                }
            }
        }
        // Most are checked within the bound.
        assert!(checked > 24 * 6 / 2, "{checked} of {} checked", 24 * 6);
    }

    /// A result set by hand is held to the steps: `⊥ ⊢` in place of the
    /// Skolem form of `p(a) ⊢` implies anything, but the steps do not make
    /// it.
    #[test]
    fn a_result_set_by_hand_is_held_to_the_steps() {
        let sequent: Sequent = "forall x. p(x) |- exists y. p(y)".parse().unwrap();
        let options = Options::default().with_form(Form::Skolem);
        let mut normal = normalize_default(&sequent, &options).unwrap();
        let mut formulas = normal.result.formulas().clone();
        let bottom = formulas.add(Node::False).unwrap();
        normal.result = Sequent::new(formulas, vec![bottom], Vec::new()).unwrap();
        let check = normal.check(&search::Options::default(), &Limits::default(), |_| false);
        assert!(matches!(check, Err(Error::ClaimFails { .. })), "{check:?}");
    }

    /// The clausal form of `n` hypotheses `∀x. pᵢ(x)` against the
    /// conjunction of every `pᵢ(a)`: a step per conjunct, each sequent on
    /// the way holding every hypothesis.
    fn many_hypotheses(n: usize) -> Sequent {
        let left: Vec<String> = (0..n).map(|i| format!("forall x. p{i}(x)")).collect();
        let right: Vec<String> = (0..n).map(|i| format!("p{i}(a)")).collect();
        format!("{} |- {}", left.join(", "), right.join(" /\\ "))
            .parse()
            .unwrap()
    }

    /// A law's claim holds its two formulas alone, however large the
    /// result's arena: a copy of that arena per step made the check of a
    /// long rewriting cubic in memory.
    #[test]
    fn a_law_s_claim_copies_its_formulas_alone() {
        let options = Options::default().with_form(Form::Clausal);
        let normal = normalize_default(&many_hypotheses(40), &options).unwrap();
        assert!(normal.is_checked());
        assert!(normal.result.formulas().len() > 200);
        // The nodes the step's two formulas reach in the result's arena.
        let formulas = normal.result.formulas();
        let reached = |roots: [NodeId; 2]| {
            let mut seen = crate::hash::HashSet::default();
            let mut stack = roots.to_vec();
            while let Some(k) = stack.pop() {
                if seen.insert(k) {
                    let (a, b) = formulas.node(k).operands();
                    stack.extend([a, b].into_iter().flatten());
                }
            }
            seen.len()
        };
        let mut laws = 0;
        for check in normal.checks.iter().filter(|c| c.claim == Claim::Law) {
            let step = &normal.steps[check.step.unwrap()];
            // Those nodes and the claim's `↔`.
            let most = reached([step.before, step.after]) + 1;
            assert!(
                check.sequent.formulas().len() <= most,
                "step {:?}",
                check.step
            );
            laws += 1;
        }
        assert!(laws >= 39, "{laws} laws");
    }

    /// The clausal form keeps to the work, memory and stop it is given,
    /// in the rewriting and in the check, and a bound reached names the
    /// bound the caller gave, not a claim's share of it.
    #[test]
    fn the_clausal_form_keeps_to_its_bounds() {
        let sequent = many_hypotheses(40);
        let options = Options::default().with_form(Form::Clausal);
        let search = search::Options::default();
        let work = Limits::default().with_work(Some(10));
        assert!(matches!(
            rewrite(&sequent, &options, &work, |_| false),
            Err(Error::Refused(Refusal::Work { .. }))
        ));
        let tight = Limits::default().with_memory_bytes(Some(4_096));
        assert!(matches!(
            rewrite(&sequent, &options, &tight, |_| false),
            Err(Error::Refused(Refusal::Memory {
                limit_bytes: 4_096,
                ..
            }))
        ));
        let made = rewrite(&sequent, &options, &Limits::default(), |_| false).unwrap();
        // The checks' derivations pass 64 KiB together.
        let mut normal = made.clone();
        let small = Limits::default().with_memory_bytes(Some(65_536));
        match normal.check(&search, &small, |_| false) {
            Err(Error::Refused(Refusal::Memory { limit_bytes, .. })) => {
                assert_eq!(limit_bytes, 65_536);
            }
            Ok(()) => assert!(normal.checks.iter().any(|c| matches!(
                c.outcome.verdict,
                Verdict::Unknown(search::Reason::MemoryLimit {
                    limit_bytes: 65_536
                })
            ))),
            other => panic!("{other:?}"),
        }
        // The stop is asked between the claims.
        let mut normal = made;
        let stopped = normal.check(&search, &Limits::default(), |p| p.phase == Phase::Check);
        assert!(
            matches!(
                stopped,
                Err(Error::Refused(Refusal::Stopped {
                    phase: Phase::Check
                }))
            ),
            "{stopped:?}"
        );
    }

    /// The propositional forms still refuse a first-order sequent.
    #[test]
    fn the_propositional_forms_refuse_quantifiers() {
        let sequent: Sequent = "|- forall x. p(x)".parse().unwrap();
        for form in [Form::Cnf, Form::Dnf, Form::Tseitin, Form::PlaistedGreenbaum] {
            let options = Options::default().with_form(form);
            assert!(matches!(
                normalize_default(&sequent, &options),
                Err(Error::FirstOrder { .. })
            ));
        }
    }
}
