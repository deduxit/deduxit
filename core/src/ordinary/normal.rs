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
use super::polarity::{FALSE, TRUE, polarities};
use super::transfer::{self, Parameters};
use super::{Formulas, Logic, Node, NodeId, Outcome, Sequent, Verdict, decide_within};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::search;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

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
/// symbols by name, a bound variable by its index, a binder by its name. A
/// step's formulas hold no eigenvariable or open witness (a result's arena
/// holds neither), so one is [`Error::ClaimFails`].
fn copy_formulas(
    from: &Formulas,
    roots: &[NodeId],
    to: &mut Formulas,
) -> Result<Vec<NodeId>, Error> {
    let copied = transfer::copy(to, from, Some(roots), &Parameters::Constants)?;
    if !copied.constants.is_empty() {
        return Err(Error::ClaimFails {
            claim: "a normal form".to_owned(),
            reason: "a step holds an eigenvariable or an open witness".to_owned(),
        });
    }
    Ok(roots.iter().map(|&r| copied.node(r)).collect())
}

/// Returns the formulas of `sequent` added to `formulas`, built again by
/// their atoms' names, so that they are formulas of that arena whatever
/// arena the sequent has. A sequent's arena holds no eigenvariable or open
/// witness, so one is [`Error::ClaimFails`].
fn transfer(
    sequent: &Sequent,
    formulas: &mut Formulas,
) -> Result<(Vec<NodeId>, Vec<NodeId>), Error> {
    let copied = transfer::copy(formulas, sequent.formulas(), None, &Parameters::Constants)?;
    if !copied.constants.is_empty() {
        return Err(Error::ClaimFails {
            claim: "a normal form".to_owned(),
            reason: "the input holds an eigenvariable or an open witness".to_owned(),
        });
    }
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

/// A phase of the rewriting into an equivalent form: the laws it applies.
#[derive(Clone, Copy, Debug)]
enum Stage {
    /// `→` and `↔` rewritten, `↔` by cases where `cases` is set.
    Arrows {
        /// Whether `↔` becomes a disjunction of cases, for the
        /// disjunctive form.
        cases: bool,
    },
    /// Negations moved inwards to the atoms.
    Negations,
    /// `⊤` and `⊥` dropped from conjunctions and disjunctions.
    Constants,
    /// `∨` distributed over `∧` (`conjunctive`) or `∧` over `∨`.
    Distribute {
        /// Whether the result is a conjunction of clauses.
        conjunctive: bool,
    },
    /// `↔` split into two implications, on the way to prenex form.
    Iffs,
    /// Quantifiers moved out to the front: of every kind, or `∀` alone
    /// over `∧` and `∨` where `universal` is set (the clausal form, which
    /// has no `∃` left).
    Outward {
        /// Whether only `∀` moves.
        universal: bool,
    },
    /// Quantifiers moved in as far as they go.
    Inward,
    /// `∀` distributed over `∧` and vacuous `∀`s left out, so that each
    /// clause has its own prefix.
    Split,
    /// The strong quantifiers to be replaced by their terms: `∃` where
    /// `exists` is set, else `∀`.
    Strong {
        /// Whether the quantifier is `∃`.
        exists: bool,
    },
}

/// Returns the phases of an equivalent form, in order.
const fn stages(form: Form) -> &'static [Stage] {
    match form {
        Form::Nnf => &[Stage::Arrows { cases: false }, Stage::Negations],
        Form::Cnf => &[
            Stage::Arrows { cases: false },
            Stage::Negations,
            Stage::Constants,
            Stage::Distribute { conjunctive: true },
        ],
        Form::Dnf => &[
            Stage::Arrows { cases: true },
            Stage::Negations,
            Stage::Constants,
            Stage::Distribute { conjunctive: false },
        ],
        Form::Prenex => &[Stage::Iffs, Stage::Outward { universal: false }],
        Form::Miniscope => &[
            Stage::Arrows { cases: false },
            Stage::Negations,
            Stage::Inward,
        ],
        Form::Tseitin | Form::PlaistedGreenbaum | Form::Skolem | Form::Herbrand | Form::Clausal => {
            &[]
        }
    }
}

/// Returns whether `id` mentions the variable of the binder right around
/// it, de Bruijn index 0 read outside it.
fn mentions_binder(formulas: &Formulas, id: NodeId) -> bool {
    formulas.free_variables(id).first() == Some(&0)
}

/// Returns the quantifier law of `stage` that applies at the root of `id`,
/// if one does: the first-order part of [`law`]'s table.
fn quantifier_law(stage: Stage, formulas: &Formulas, id: NodeId) -> Option<Rule> {
    let node = formulas.node(id);
    let kind = |k: NodeId| match formulas.node(k) {
        Node::Forall(_) => Some(true),
        Node::Exists(_) => Some(false),
        Node::Atom(_)
        | Node::True
        | Node::False
        | Node::Not(_)
        | Node::And(..)
        | Node::Or(..)
        | Node::Implies(..)
        | Node::Iff(..) => None,
    };
    match stage {
        Stage::Outward { universal } => match node {
            Node::Not(a) if !universal => match kind(a)? {
                true => Some(Rule::NotForall),
                false => Some(Rule::NotExists),
            },
            Node::And(a, b) | Node::Or(a, b) => {
                let and = matches!(node, Node::And(..));
                // The left operand first, a `∃` only where it may move.
                [a, b]
                    .into_iter()
                    .filter_map(kind)
                    .find(|&forall| forall || !universal)
                    .map(|forall| match (forall, and) {
                        (true, true) => Rule::ForallAnd,
                        (false, true) => Rule::ExistsAnd,
                        (true, false) => Rule::ForallOr,
                        (false, false) => Rule::ExistsOr,
                    })
            }
            Node::Implies(a, b) if !universal => match (kind(a), kind(b)) {
                (Some(true), _) => Some(Rule::ForallAntecedent),
                (Some(false), _) => Some(Rule::ExistsAntecedent),
                (None, Some(true)) => Some(Rule::ForallConsequent),
                (None, Some(false)) => Some(Rule::ExistsConsequent),
                (None, None) => None,
            },
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::Implies(..)
            | Node::Iff(..)
            | Node::Forall(_)
            | Node::Exists(_) => None,
        },
        Stage::Inward => match node {
            Node::Forall(a) | Node::Exists(a) => {
                let forall = matches!(node, Node::Forall(_));
                if !mentions_binder(formulas, a) {
                    return Some(Rule::Vacuous);
                }
                match (forall, formulas.node(a)) {
                    (true, Node::And(..)) => Some(Rule::ForallDistribute),
                    (false, Node::Or(..)) => Some(Rule::ExistsDistribute),
                    (true, Node::Or(c, d)) | (false, Node::And(c, d))
                        if !mentions_binder(formulas, c) || !mentions_binder(formulas, d) =>
                    {
                        Some(if forall {
                            Rule::ForallOr
                        } else {
                            Rule::ExistsAnd
                        })
                    }
                    _ => None,
                }
            }
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => None,
        },
        Stage::Split => match node {
            Node::Forall(a) if !mentions_binder(formulas, a) => Some(Rule::Vacuous),
            Node::Forall(a) if matches!(formulas.node(a), Node::And(..)) => {
                Some(Rule::ForallDistribute)
            }
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..)
            | Node::Forall(_)
            | Node::Exists(_) => None,
        },
        Stage::Strong { exists } => match node {
            Node::Exists(_) if exists => Some(Rule::Skolemize),
            Node::Forall(_) if !exists => Some(Rule::Herbrandize),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..)
            | Node::Forall(_)
            | Node::Exists(_) => None,
        },
        Stage::Iffs => match node {
            Node::Iff(..) => Some(Rule::EquivalenceAsImplications),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Forall(_)
            | Node::Exists(_) => None,
        },
        Stage::Arrows { .. } | Stage::Negations | Stage::Constants | Stage::Distribute { .. } => {
            None
        }
    }
}

/// Returns whether a node is `⊤`, `⊥`, `∧` or `∨`, by the kind asked.
const fn is(node: Node, kind: Kind) -> bool {
    matches!(
        (kind, node),
        (Kind::True, Node::True)
            | (Kind::False, Node::False)
            | (Kind::And, Node::And(..))
            | (Kind::Or, Node::Or(..))
    )
}

/// The kinds of node a law's table asks about.
#[derive(Clone, Copy)]
enum Kind {
    /// `⊤`
    True,
    /// `⊥`
    False,
    /// `∧`
    And,
    /// `∨`
    Or,
}

/// Returns the law of `stage` that applies at the root of `id`, if one
/// does. The table of the rewriting: [`apply`] makes what each law gives.
fn law(stage: Stage, formulas: &Formulas, id: NodeId) -> Option<Rule> {
    let node = formulas.node(id);
    let either =
        |a: NodeId, b: NodeId, kind| is(formulas.node(a), kind) || is(formulas.node(b), kind);
    match stage {
        Stage::Arrows { cases } => match node {
            Node::Implies(..) => Some(Rule::Implication),
            Node::Iff(..) if cases => Some(Rule::EquivalenceAsCases),
            Node::Iff(..) => Some(Rule::Equivalence),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..) => None,
            Node::Forall(_) | Node::Exists(_) => None,
        },
        Stage::Negations => match node {
            Node::Not(a) => match formulas.node(a) {
                Node::Not(_) => Some(Rule::DoubleNegation),
                Node::And(..) => Some(Rule::DeMorganAnd),
                Node::Or(..) => Some(Rule::DeMorganOr),
                Node::True => Some(Rule::NotTrue),
                Node::False => Some(Rule::NotFalse),
                Node::Forall(_) => Some(Rule::NotForall),
                Node::Exists(_) => Some(Rule::NotExists),
                // No arrow is left after the first phase.
                Node::Atom(_) | Node::Implies(..) | Node::Iff(..) => None,
            },
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => None,
            Node::Forall(_) | Node::Exists(_) => None,
        },
        Stage::Constants => match node {
            Node::And(a, b) if either(a, b, Kind::False) => Some(Rule::AndFalse),
            Node::And(a, b) if either(a, b, Kind::True) => Some(Rule::AndTrue),
            Node::Or(a, b) if either(a, b, Kind::True) => Some(Rule::OrTrue),
            Node::Or(a, b) if either(a, b, Kind::False) => Some(Rule::OrFalse),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => None,
            Node::Forall(_) | Node::Exists(_) => None,
        },
        Stage::Distribute { conjunctive } => match node {
            Node::Or(a, b) if conjunctive && either(a, b, Kind::And) => Some(Rule::DistributeOr),
            Node::And(a, b) if !conjunctive && either(a, b, Kind::Or) => Some(Rule::DistributeAnd),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => None,
            Node::Forall(_) | Node::Exists(_) => None,
        },
        Stage::Iffs
        | Stage::Outward { .. }
        | Stage::Inward
        | Stage::Split
        | Stage::Strong { .. } => quantifier_law(stage, formulas, id),
    }
}

/// Returns what `rule` makes of the formula `id`, at whose root [`law`]
/// found it, added to the arena.
fn apply(rule: Rule, formulas: &mut Formulas, id: NodeId) -> Result<NodeId, Error> {
    if !matches!(
        rule,
        Rule::Implication
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
    ) {
        return apply_quantifier(rule, formulas, id);
    }
    let node = formulas.node(id);
    let not = |f: &mut Formulas, a| f.add(Node::Not(a));
    let (a, b) = node.operands();
    let (a, b) = (
        a.unwrap_or_else(|| unreachable!("every law rewrites a compound formula")),
        b.unwrap_or(a.unwrap_or_else(|| unreachable!("checked above"))),
    );
    let inner = |f: &Formulas, k: NodeId| f.node(k).operands();
    match rule {
        Rule::Implication => {
            let na = not(formulas, a)?;
            formulas.add(Node::Or(na, b))
        }
        Rule::Equivalence => {
            let (na, nb) = (not(formulas, a)?, not(formulas, b)?);
            let one = formulas.add(Node::Or(na, b))?;
            let two = formulas.add(Node::Or(a, nb))?;
            formulas.add(Node::And(one, two))
        }
        Rule::EquivalenceAsCases => {
            let (na, nb) = (not(formulas, a)?, not(formulas, b)?);
            let one = formulas.add(Node::And(a, b))?;
            let two = formulas.add(Node::And(na, nb))?;
            formulas.add(Node::Or(one, two))
        }
        Rule::DoubleNegation => Ok(inner(formulas, a)
            .0
            .unwrap_or_else(|| unreachable!("a double negation"))),
        Rule::DeMorganAnd | Rule::DeMorganOr => {
            let (Some(c), Some(d)) = inner(formulas, a) else {
                unreachable!("a negated binary formula")
            };
            let (nc, nd) = (not(formulas, c)?, not(formulas, d)?);
            formulas.add(if rule == Rule::DeMorganAnd {
                Node::Or(nc, nd)
            } else {
                Node::And(nc, nd)
            })
        }
        Rule::NotTrue | Rule::AndFalse => formulas.add(Node::False),
        Rule::NotFalse | Rule::OrTrue => formulas.add(Node::True),
        // The operand that is not the constant.
        Rule::AndTrue => Ok(if is(formulas.node(a), Kind::True) {
            b
        } else {
            a
        }),
        Rule::OrFalse => Ok(if is(formulas.node(a), Kind::False) {
            b
        } else {
            a
        }),
        Rule::DistributeOr | Rule::DistributeAnd => {
            let (outer, kind): (fn(NodeId, NodeId) -> Node, Kind) = if rule == Rule::DistributeOr {
                (Node::And, Kind::And)
            } else {
                (Node::Or, Kind::Or)
            };
            let inner_of = if rule == Rule::DistributeOr {
                Node::Or
            } else {
                Node::And
            };
            // `A ∨ (B ∧ C)`, else `(B ∧ C) ∨ A`, which keeps the order.
            if is(formulas.node(b), kind) {
                let (Some(c), Some(d)) = inner(formulas, b) else {
                    unreachable!("a binary operand")
                };
                let one = formulas.add(inner_of(a, c))?;
                let two = formulas.add(inner_of(a, d))?;
                formulas.add(outer(one, two))
            } else {
                let (Some(c), Some(d)) = inner(formulas, a) else {
                    unreachable!("a binary operand")
                };
                let one = formulas.add(inner_of(c, b))?;
                let two = formulas.add(inner_of(d, b))?;
                formulas.add(outer(one, two))
            }
        }
        Rule::Negate
        | Rule::Define
        | Rule::TrueHypothesis
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
        | Rule::Vacuous
        | Rule::Skolemize
        | Rule::Herbrandize
        | Rule::SplitConjunction => unreachable!("taken above or no law of an equivalent form"),
    }
}

/// Returns what the quantifier law `rule` makes of `id`, at whose root
/// [`law`] found it: a binder moved out of a binary formula (the other
/// operand lifted under it), moved in (the operand that does not mention
/// its variable lowered out of it), or a negation moved past it.
fn apply_quantifier(rule: Rule, formulas: &mut Formulas, id: NodeId) -> Result<NodeId, Error> {
    let node = formulas.node(id);
    let binder = |forall: bool, body: NodeId| {
        if forall {
            Node::Forall(body)
        } else {
            Node::Exists(body)
        }
    };
    let body_of = |f: &Formulas, q: NodeId| match f.node(q) {
        Node::Forall(a) => (true, a),
        Node::Exists(a) => (false, a),
        Node::Atom(_)
        | Node::True
        | Node::False
        | Node::Not(_)
        | Node::And(..)
        | Node::Or(..)
        | Node::Implies(..)
        | Node::Iff(..) => unreachable!("the law found a quantifier there"),
    };
    match (rule, node) {
        (Rule::EquivalenceAsImplications, Node::Iff(a, b)) => {
            let one = formulas.add(Node::Implies(a, b))?;
            let two = formulas.add(Node::Implies(b, a))?;
            formulas.add(Node::And(one, two))
        }
        (Rule::NotForall | Rule::NotExists, Node::Not(q)) => {
            let (forall, body) = body_of(formulas, q);
            let negated = formulas.add(Node::Not(body))?;
            formulas.add_like(binder(!forall, negated), q)
        }
        // Out of a junction: the quantified operand's binder goes around
        // both, the other operand lifted under it.
        (
            Rule::ForallAnd | Rule::ExistsAnd | Rule::ForallOr | Rule::ExistsOr,
            Node::And(a, b) | Node::Or(a, b),
        ) => {
            let forall = matches!(rule, Rule::ForallAnd | Rule::ForallOr);
            let first = matches!(formulas.node(a), Node::Forall(_) if forall)
                || matches!(formulas.node(a), Node::Exists(_) if !forall);
            let (q, other) = if first { (a, b) } else { (b, a) };
            let (_, body) = body_of(formulas, q);
            let lifted = formulas.lifted(other, 1)?;
            let (x, y) = if first {
                (body, lifted)
            } else {
                (lifted, body)
            };
            let joined = formulas.add(if matches!(node, Node::And(..)) {
                Node::And(x, y)
            } else {
                Node::Or(x, y)
            })?;
            formulas.add_like(binder(forall, joined), q)
        }
        // Out of an implication: out of its antecedent the quantifier
        // turns, out of its consequent it stays.
        (Rule::ForallAntecedent | Rule::ExistsAntecedent, Node::Implies(a, b)) => {
            let (forall, body) = body_of(formulas, a);
            let lifted = formulas.lifted(b, 1)?;
            let implies = formulas.add(Node::Implies(body, lifted))?;
            formulas.add_like(binder(!forall, implies), a)
        }
        (Rule::ForallConsequent | Rule::ExistsConsequent, Node::Implies(a, b)) => {
            let (forall, body) = body_of(formulas, b);
            let lifted = formulas.lifted(a, 1)?;
            let implies = formulas.add(Node::Implies(lifted, body))?;
            formulas.add_like(binder(forall, implies), b)
        }
        // In: a binder over a formula that does not mention its variable
        // goes, over a conjunction (`∀`) or disjunction (`∃`) it splits,
        // and past an operand that does not mention it the operand goes
        // out.
        (_, Node::Forall(body) | Node::Exists(body)) => {
            let forall = matches!(node, Node::Forall(_));
            let lowered = |f: &mut Formulas, k: NodeId| {
                f.lowered(k)?
                    .ok_or_else(|| unreachable!("the law checked the variable is not free"))
            };
            match (rule, formulas.node(body)) {
                (Rule::Vacuous, _) => lowered(formulas, body),
                (Rule::ForallDistribute, Node::And(c, d))
                | (Rule::ExistsDistribute, Node::Or(c, d)) => {
                    let one = formulas.add_like(binder(forall, c), id)?;
                    let two = formulas.add_like(binder(forall, d), id)?;
                    formulas.add(if forall {
                        Node::And(one, two)
                    } else {
                        Node::Or(one, two)
                    })
                }
                (Rule::ForallOr, Node::Or(c, d)) | (Rule::ExistsAnd, Node::And(c, d)) => {
                    let join = |x: NodeId, y: NodeId| {
                        if forall {
                            Node::Or(x, y)
                        } else {
                            Node::And(x, y)
                        }
                    };
                    if mentions_binder(formulas, d) {
                        let out = lowered(formulas, c)?;
                        let kept = formulas.add_like(binder(forall, d), id)?;
                        formulas.add(join(out, kept))
                    } else {
                        let kept = formulas.add_like(binder(forall, c), id)?;
                        let out = lowered(formulas, d)?;
                        formulas.add(join(kept, out))
                    }
                }
                _ => unreachable!("the law found this shape"),
            }
        }
        _ => unreachable!("the law found this shape"),
    }
}

/// The scratch of the passes over the formulas a sequent reaches, kept
/// between steps: arrays indexed by node, valid where `seen` holds the
/// current pass's stamp, so that a pass costs what it reaches and not
/// the whole arena.
#[derive(Default)]
struct Passes {
    /// The pass that last reached each node.
    seen: Vec<u32>,
    /// The current pass.
    stamp: u32,
    /// The nodes reached, in the arena's order: every operand before the
    /// formulas built from it.
    order: Vec<NodeId>,
    /// The stack of the walk.
    stack: Vec<NodeId>,
    /// Per node: whether a law applies at or below it.
    found: Vec<bool>,
    /// Per node: the occurrences it unfolds to, saturating.
    size: Vec<u64>,
    /// Per node: what it becomes in a substitution.
    image: Vec<NodeId>,
}

impl Passes {
    /// Collects the nodes `roots` reach into `order`, in the arena's order,
    /// and their unfolded sizes, and returns the sequent's size.
    fn reach(&mut self, formulas: &Formulas, roots: &[NodeId]) -> u64 {
        let n = formulas.len();
        self.seen.resize(n, 0);
        self.found.resize(n, false);
        self.size.resize(n, 0);
        self.image.resize(n, NodeId(0));
        // A pass per step: `u32` passes would need more steps than the
        // occurrence bound and the work allow.
        self.stamp = self.stamp.wrapping_add(1);
        if self.stamp == 0 {
            self.seen.fill(0);
            self.stamp = 1;
        }
        self.order.clear();
        self.stack.clear();
        self.stack.extend_from_slice(roots);
        while let Some(id) = self.stack.pop() {
            if self.seen[id.index()] == self.stamp {
                continue;
            }
            self.seen[id.index()] = self.stamp;
            self.order.push(id);
            let (a, b) = formulas.node(id).operands();
            self.stack.extend([a, b].into_iter().flatten());
        }
        self.order.sort_unstable();
        for &id in &self.order {
            let (a, b) = formulas.node(id).operands();
            let of = |k: Option<NodeId>| k.map_or(0, |k| self.size[k.index()]);
            self.size[id.index()] = 1u64.saturating_add(of(a)).saturating_add(of(b));
        }
        roots
            .iter()
            .fold(0u64, |sum, r| sum.saturating_add(self.size[r.index()]))
    }

    /// Returns the leftmost outermost place among `roots`, whose nodes
    /// [`reach`](Self::reach) collected, where a law of `stage` applies,
    /// and the law.
    fn redex(
        &mut self,
        stage: Stage,
        formulas: &Formulas,
        roots: &[NodeId],
    ) -> Option<(NodeId, Rule)> {
        for &id in &self.order {
            let (a, b) = formulas.node(id).operands();
            let below = [a, b].into_iter().flatten().any(|k| self.found[k.index()]);
            self.found[id.index()] = below || law(stage, formulas, id).is_some();
        }
        let root = *roots.iter().find(|r| self.found[r.index()])?;
        let mut at = root;
        loop {
            if let Some(rule) = law(stage, formulas, at) {
                return Some((at, rule));
            }
            let (a, b) = formulas.node(at).operands();
            at = [a, b]
                .into_iter()
                .flatten()
                .find(|k| self.found[k.index()])
                .unwrap_or_else(|| unreachable!("a law applies below"));
        }
    }

    /// Returns `roots` with every occurrence of `before`, among the nodes
    /// [`reach`](Self::reach) collected, replaced by `after`.
    fn substitute(
        &mut self,
        formulas: &mut Formulas,
        roots: &[NodeId],
        before: NodeId,
        after: NodeId,
    ) -> Result<Vec<NodeId>, Error> {
        for k in 0..self.order.len() {
            let id = self.order[k];
            let new = if id == before {
                after
            } else {
                let node = formulas.node(id);
                let map = |x: NodeId| self.image[x.index()];
                let mapped = match node {
                    Node::Atom(_) | Node::True | Node::False => node,
                    Node::Not(a) => Node::Not(map(a)),
                    Node::And(a, b) => Node::And(map(a), map(b)),
                    Node::Or(a, b) => Node::Or(map(a), map(b)),
                    Node::Implies(a, b) => Node::Implies(map(a), map(b)),
                    Node::Iff(a, b) => Node::Iff(map(a), map(b)),
                    Node::Forall(a) => Node::Forall(map(a)),
                    Node::Exists(a) => Node::Exists(map(a)),
                };
                if mapped == node {
                    id
                } else {
                    formulas.add_like(mapped, id)?
                }
            };
            self.image[id.index()] = new;
        }
        Ok(roots.iter().map(|r| self.image[r.index()]).collect())
    }
}

/// What a rewriting has done and may still do: its bounds.
struct Budget<'a> {
    /// The bounds.
    limits: &'a Limits,
    /// The work done: the nodes every pass reached.
    work: u64,
    /// The bytes the steps and the arena hold, estimated.
    held: u64,
    /// The stop.
    stop: &'a mut dyn FnMut(Progress) -> bool,
}

/// The bytes a node of the arena is charged: the node, its id and its
/// entry in the arena's table of ids.
const NODE_BYTES: u64 = 48;

/// The bytes a step is charged beside its ids.
const STEP_BYTES: u64 = size_of::<Step>() as u64;

impl Budget<'_> {
    /// Charges a pass over `reached` nodes and the sequent's `size`, asks
    /// the stop, and refuses past a bound.
    fn pass(&mut self, reached: usize, size: u64, steps: usize) -> Result<(), Error> {
        let reached = reached as u64;
        self.work = self.work.saturating_add(reached);
        if let Some(limit) = self.limits.form_occurrences
            && size > limit
        {
            return Err(Error::Refused(Refusal::Form {
                occurrences: size,
                limit,
            }));
        }
        if let Some(limit) = self.limits.work
            && self.work > limit
        {
            return Err(Error::Refused(Refusal::Work { limit }));
        }
        if (self.stop)(Progress::new(Phase::Rewrite, reached, steps as u64)) {
            return Err(Error::Refused(Refusal::stopped(Phase::Rewrite)));
        }
        Ok(())
    }

    /// Charges a step and the arena's growth, and refuses past the memory
    /// bound.
    fn hold(&mut self, step: &Step, nodes: usize) -> Result<(), Error> {
        let ids = step.left.len() + step.right.len() + step.added.len();
        self.held = self
            .held
            .saturating_add(STEP_BYTES + 4 * ids as u64)
            .saturating_add(NODE_BYTES * nodes as u64);
        match self.limits.memory_bytes {
            Some(limit_bytes) if self.held > limit_bytes => Err(Error::Refused(Refusal::Memory {
                phase: Phase::Rewrite,
                limit_bytes,
                needed_bytes: None,
            })),
            _ => Ok(()),
        }
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

/// Rewrites into an equivalent form, phase by phase, the leftmost
/// outermost place a law applies first.
fn equivalent(sequent: &Sequent, form: Form, budget: &mut Budget<'_>) -> Result<Made, Error> {
    let mut formulas = sequent.formulas().clone();
    let split = sequent.left().len();
    let mut roots: Vec<NodeId> = sequent
        .left()
        .iter()
        .chain(sequent.right())
        .copied()
        .collect();
    let mut passes = Passes::default();
    let mut steps = Vec::new();
    let mut run = Run {
        formulas: &mut formulas,
        roots: &mut roots,
        split,
        steps: &mut steps,
        passes: &mut passes,
        budget,
    };
    run.stages(stages(form))?;
    let (left, right) = (roots[..split].to_vec(), roots[split..].to_vec());
    Ok((formulas, left, right, steps))
}

/// A rewriting under way: the arena, the formulas of the input as they
/// stand (the first `split` left of `⊢`), the steps so far, the passes'
/// scratch and the bounds.
struct Run<'r, 'b> {
    /// The arena, which grows with every step.
    formulas: &'r mut Formulas,
    /// The input's formulas as they stand.
    roots: &'r mut Vec<NodeId>,
    /// How many of them stand left of `⊢`.
    split: usize,
    /// The steps so far.
    steps: &'r mut Vec<Step>,
    /// The passes' scratch.
    passes: &'r mut Passes,
    /// The bounds.
    budget: &'r mut Budget<'b>,
}

impl Run<'_, '_> {
    /// Runs the phases `stages`, each until no law of it applies, the
    /// leftmost outermost place first.
    fn stages(&mut self, stages: &[Stage]) -> Result<(), Error> {
        for &stage in stages {
            loop {
                let size = self.passes.reach(self.formulas, self.roots);
                self.budget
                    .pass(self.passes.order.len(), size, self.steps.len())?;
                let Some((before, rule)) = self.passes.redex(stage, self.formulas, self.roots)
                else {
                    break;
                };
                let nodes = self.formulas.len();
                let after = apply(rule, self.formulas, before)?;
                *self.roots = self
                    .passes
                    .substitute(self.formulas, self.roots, before, after)?;
                self.record(rule, before, after, None, nodes)?;
            }
        }
        Ok(())
    }

    /// Records a step that replaced `before` by `after`, the arena `nodes`
    /// long before it.
    fn record(
        &mut self,
        rule: Rule,
        before: NodeId,
        after: NodeId,
        term: Option<FoTermId>,
        nodes: usize,
    ) -> Result<(), Error> {
        let step = Step {
            rule,
            before,
            after,
            left: self.roots[..self.split].to_vec(),
            right: self.roots[self.split..].to_vec(),
            added: Vec::new(),
            term,
        };
        self.budget.hold(&step, self.formulas.len() - nodes)?;
        self.steps.push(step);
        Ok(())
    }

    /// Splits each conjunction left of `⊢` into its two parts and leaves out
    /// `⊤` there, each a step, so that the clauses stand as hypotheses one
    /// by one.
    fn hypotheses(&mut self) -> Result<(), Error> {
        let at = |run: &Self, wanted: &dyn Fn(Node) -> bool| {
            run.roots[..run.split]
                .iter()
                .position(|&r| wanted(run.formulas.node(r)))
        };
        while let Some(k) = at(self, &|node| matches!(node, Node::And(..))) {
            self.budget.pass(0, 0, self.steps.len())?;
            let conjunction = self.roots[k];
            let (Some(a), Some(b)) = self.formulas.node(conjunction).operands() else {
                unreachable!("a conjunction has two operands")
            };
            self.roots.splice(k..=k, [a, b]);
            self.split += 1;
            let nodes = self.formulas.len();
            self.record(
                Rule::SplitConjunction,
                conjunction,
                conjunction,
                None,
                nodes,
            )?;
        }
        while let Some(k) = at(self, &|node| matches!(node, Node::True)) {
            let truth = self.roots.remove(k);
            self.split -= 1;
            let nodes = self.formulas.len();
            self.record(Rule::TrueHypothesis, truth, truth, None, nodes)?;
        }
        Ok(())
    }

    /// Replaces each strong quantifier of the formulas in `range` (in
    /// negation normal form, where every quantifier stands positive: `∃`
    /// where `exists` is set, else `∀`), the outermost first, by its term:
    /// a new function symbol named by `prefix` and a number past `count`,
    /// applied to the variables free in the quantified formula. Equal
    /// quantified formulas are one node and share their symbol.
    fn strong(
        &mut self,
        range: std::ops::Range<usize>,
        exists: bool,
        rule: Rule,
        prefix: &str,
        count: &mut u64,
    ) -> Result<(), Error> {
        loop {
            let size = self.passes.reach(self.formulas, self.roots);
            self.budget
                .pass(self.passes.order.len(), size, self.steps.len())?;
            let side = self.roots[range.clone()].to_vec();
            self.passes.reach(self.formulas, &side);
            let Some((before, _)) =
                self.passes
                    .redex(Stage::Strong { exists }, self.formulas, &side)
            else {
                return Ok(());
            };
            let nodes = self.formulas.len();
            let (Node::Forall(body) | Node::Exists(body)) = self.formulas.node(before) else {
                unreachable!("the stage finds quantifiers")
            };
            *count += 1;
            let free = self.formulas.free_variables(before);
            let term = skolem_term(self.formulas, &format!("{prefix}{count}"), &free)?;
            if let Some(limit) = self.budget.limits.terms
                && free.len() as u64 + 1 > limit
            {
                return Err(Error::Refused(Refusal::Terms {
                    terms: free.len() as u64 + 1,
                    limit,
                }));
            }
            let after = self.formulas.substituted(body, term)?;
            let changed = self
                .passes
                .substitute(self.formulas, &side, before, after)?;
            self.roots.splice(range.clone(), changed);
            self.record(rule, before, after, Some(term), nodes)?;
        }
    }
}

/// Returns the term of the new function symbol `name` over the variables
/// `free` (outside indices, ascending), the outermost first: `name(x̄)`, a
/// constant where `free` is empty.
fn skolem_term(formulas: &mut Formulas, name: &str, free: &[u32]) -> Result<FoTermId, Error> {
    let symbol = formulas
        .interner()
        .symbol(name, free.len() as u32, false)
        .map_err(|fault| Error::Symbol {
            name: name.to_owned(),
            fault,
        })?;
    let args: Vec<FoTermId> = free
        .iter()
        .rev()
        .map(|&i| formulas.interner().term(FoTerm::Bound(i)))
        .collect();
    let list = formulas.interner().list(&args);
    Ok(formulas.interner().term(FoTerm::App(symbol, list)))
}

/// Returns `node`, a formula left of `⊢`, in negation normal form as
/// Skolem's and the clausal form make it before their strong quantifiers
/// go: the same laws in the same order, so that a closed instance of a
/// formula they rewrote meets their step's quantifier again.
///
/// # Errors
///
/// [`Refusal::Form`], [`Refusal::Work`] and [`Refusal::Memory`] past the
/// bounds of `limits`; [`Refusal::Index`] when the arena is full.
#[cfg(feature = "sat")]
pub(super) fn negation_normal(
    formulas: &mut Formulas,
    node: NodeId,
    limits: &Limits,
) -> Result<NodeId, Error> {
    let mut never = |_: Progress| false;
    let mut budget = Budget {
        limits,
        work: 0,
        held: 0,
        stop: &mut never,
    };
    let mut roots = vec![node];
    let (mut steps, mut passes) = (Vec::new(), Passes::default());
    let mut run = Run {
        formulas,
        roots: &mut roots,
        split: 1,
        steps: &mut steps,
        passes: &mut passes,
        budget: &mut budget,
    };
    run.stages(&[Stage::Arrows { cases: false }, Stage::Negations])?;
    Ok(roots[0])
}

/// Returns the prefix of the new function symbols' names: `base`, or
/// `base` with as many `_` after it as make no symbol or atom of the arena
/// the prefix and digits.
pub(super) fn fresh_prefix(formulas: &Formulas, base: &str) -> String {
    let mut prefix = String::from(base);
    let taken = |prefix: &str| {
        let numbered = |name: &str| {
            name.strip_prefix(prefix)
                .is_some_and(|rest| !rest.is_empty() && rest.bytes().all(|c| c.is_ascii_digit()))
        };
        formulas.atom_names().iter().any(|name| numbered(name))
            || formulas
                .first_order()
                .symbols
                .iter()
                .any(|(name, _)| numbered(name))
    };
    while taken(&prefix) {
        prefix.push('_');
    }
    prefix
}

/// Makes Skolem's, Herbrand's or the clausal normal form: Skolem's and the
/// clausal form move the formulas right of `⊢` left negated, then every
/// formula goes to negation normal form and each strong quantifier is
/// replaced by its term; the clausal form then moves the `∀`s left to the
/// front, distributes, gives each clause its own prefix and splits the
/// conjunctions into hypotheses.
fn skolemized(sequent: &Sequent, form: Form, budget: &mut Budget<'_>) -> Result<Made, Error> {
    let mut formulas = sequent.formulas().clone();
    let mut left = sequent.left().to_vec();
    let mut right = sequent.right().to_vec();
    let mut steps = Vec::new();
    let refutation = form != Form::Herbrand;
    if refutation {
        left = negated_right(sequent, &mut formulas, &mut steps, budget)?;
        right.clear();
    }
    let split = left.len();
    let mut roots: Vec<NodeId> = left.iter().chain(&right).copied().collect();
    let mut passes = Passes::default();
    let prefix = fresh_prefix(&formulas, "sk");
    let mut count = 0u64;
    let mut run = Run {
        formulas: &mut formulas,
        roots: &mut roots,
        split,
        steps: &mut steps,
        passes: &mut passes,
        budget,
    };
    run.stages(&[Stage::Arrows { cases: false }, Stage::Negations])?;
    let rule = if refutation {
        Rule::Skolemize
    } else {
        Rule::Herbrandize
    };
    let all = run.roots.len();
    run.strong(0..split, true, rule, &prefix, &mut count)?;
    run.strong(split..all, false, rule, &prefix, &mut count)?;
    if form == Form::Clausal {
        run.stages(&[
            Stage::Outward { universal: true },
            Stage::Constants,
            Stage::Distribute { conjunctive: true },
            Stage::Split,
        ])?;
        run.hypotheses()?;
    }
    let split = run.split;
    let (left, right) = (roots[..split].to_vec(), roots[split..].to_vec());
    Ok((formulas, left, right, steps))
}

/// Returns whether `id` is a literal of a clause: an atom, a constant or
/// the negation of one.
fn is_literal(formulas: &Formulas, id: NodeId) -> bool {
    match formulas.node(id) {
        Node::Atom(_) | Node::True | Node::False => true,
        Node::Not(a) => matches!(formulas.node(a), Node::Atom(_) | Node::True | Node::False),
        Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => false,
        Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
    }
}

/// Returns the negation of the literal `id`, without a double negation.
fn negated(formulas: &mut Formulas, id: NodeId) -> Result<NodeId, Error> {
    match formulas.node(id) {
        Node::Not(a) => Ok(a),
        Node::True => formulas.add(Node::False),
        Node::False => formulas.add(Node::True),
        Node::Atom(_) | Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => {
            formulas.add(Node::Not(id))
        }
        Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
    }
}

/// Moves the formulas right of `sequent`'s `⊢` left, each negated in
/// `formulas` by a `Negate` step of its own in `steps`, and returns the left
/// side with them.
fn negated_right(
    sequent: &Sequent,
    formulas: &mut Formulas,
    steps: &mut Vec<Step>,
    budget: &mut Budget<'_>,
) -> Result<Vec<NodeId>, Error> {
    let mut left = sequent.left().to_vec();
    for (k, &d) in sequent.right().iter().enumerate() {
        let nodes = formulas.len();
        let after = formulas.add(Node::Not(d))?;
        left.push(after);
        let step = Step {
            rule: Rule::Negate,
            before: d,
            after,
            left: left.clone(),
            right: sequent.right()[k + 1..].to_vec(),
            added: Vec::new(),
            term: None,
        };
        budget.hold(&step, formulas.len() - nodes)?;
        steps.push(step);
    }
    Ok(left)
}

/// Encodes the sequent's negation as clauses: the formulas right of `⊢`
/// moved left negated, then every compound subformula named by a new atom,
/// the innermost first, with the clauses of its definition.
fn encoded(sequent: &Sequent, form: Form, budget: &mut Budget<'_>) -> Result<Made, Error> {
    let mut formulas = sequent.formulas().clone();
    let mut steps = Vec::new();
    let mut passes = Passes::default();
    let mut left = negated_right(sequent, &mut formulas, &mut steps, budget)?;
    let size = passes.reach(&formulas, &left);
    budget.pass(passes.order.len(), size, steps.len())?;
    let order = passes.order.clone();
    // The classical encoder's rule: the left side true.
    let polarity = polarities(&formulas, &left, &[]);
    // A propositional arena has no symbols, so only the atoms are
    // avoided.
    let prefix = fresh_prefix(&formulas, "t");
    // The clauses so far, and their occurrences.
    let mut clauses: Vec<NodeId> = Vec::new();
    let mut clauses_size = 0u64;
    // What each compound node of the input is named.
    let mut name: HashMap<NodeId, NodeId> = HashMap::new();
    let mut count = 0u64;
    for &x in &order {
        if is_literal(&formulas, x) {
            continue;
        }
        let nodes = formulas.len();
        let current = |k: NodeId| name.get(&k).copied().unwrap_or(k);
        let node = formulas.node(x);
        let before = formulas.add(match node {
            Node::Atom(_) | Node::True | Node::False => node,
            Node::Not(a) => Node::Not(current(a)),
            Node::And(a, b) => Node::And(current(a), current(b)),
            Node::Or(a, b) => Node::Or(current(a), current(b)),
            Node::Implies(a, b) => Node::Implies(current(a), current(b)),
            Node::Iff(a, b) => Node::Iff(current(a), current(b)),
            Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
        })?;
        // A negation of a named subformula is a literal already.
        if is_literal(&formulas, before) {
            name.insert(x, before);
            continue;
        }
        count += 1;
        let t = formulas.atom(&format!("{prefix}{count}"))?;
        let p = polarity[x.index()];
        let (forward, backward) = match form {
            Form::PlaistedGreenbaum => (p & TRUE != 0, p & FALSE != 0),
            Form::Nnf
            | Form::Cnf
            | Form::Dnf
            | Form::Tseitin
            | Form::Prenex
            | Form::Miniscope
            | Form::Skolem
            | Form::Herbrand
            | Form::Clausal => (true, true),
        };
        let added = definition(&mut formulas, before, t, forward, backward)?;
        name.insert(x, t);
        passes.reach(&formulas, &left);
        let reached = passes.order.len();
        left = passes.substitute(&mut formulas, &left, before, t)?;
        for &c in &added {
            clauses_size = clauses_size.saturating_add(passes.reach(&formulas, &[c]));
        }
        let size = passes.reach(&formulas, &left);
        clauses.extend_from_slice(&added);
        let step = Step {
            rule: Rule::Define,
            before,
            after: t,
            left: left.clone(),
            right: Vec::new(),
            added,
            term: None,
        };
        budget.pass(reached, size.saturating_add(clauses_size), steps.len())?;
        budget.hold(&step, formulas.len() - nodes)?;
        steps.push(step);
    }
    // The formulas of the input are literals now, unit clauses. A negated
    // constant is rewritten and `⊤`, which holds anyway, is left out, each
    // a step, so that the last step's sequent is the result.
    for (rule, constant, value) in [
        (Rule::NotTrue, Node::True, Node::False),
        (Rule::NotFalse, Node::False, Node::True),
    ] {
        let Some(before) = formulas
            .find(constant)
            .and_then(|c| formulas.find(Node::Not(c)))
            .filter(|before| left.contains(before))
        else {
            continue;
        };
        let nodes = formulas.len();
        let after = formulas.add(value)?;
        for id in &mut left {
            if *id == before {
                *id = after;
            }
        }
        let step = Step {
            rule,
            before,
            after,
            left: left.clone(),
            right: Vec::new(),
            added: Vec::new(),
            term: None,
        };
        budget.hold(&step, formulas.len() - nodes)?;
        steps.push(step);
    }
    if let Some(truth) = formulas.find(Node::True).filter(|t| left.contains(t)) {
        left.retain(|&id| id != truth);
        let step = Step {
            rule: Rule::TrueHypothesis,
            before: truth,
            after: truth,
            left: left.clone(),
            right: Vec::new(),
            added: Vec::new(),
            term: None,
        };
        budget.hold(&step, 0)?;
        steps.push(step);
    }
    left.extend_from_slice(&clauses);
    Ok((formulas, left, Vec::new(), steps))
}

/// Returns the clauses that tie the new atom `t` to `before`, a compound
/// formula over literals: those of `t → before` where `forward` is set,
/// of `before → t` where `backward` is. A clause with `⊤` is left out and
/// `⊥` is dropped from a clause, the empty clause being `⊥`.
fn definition(
    formulas: &mut Formulas,
    before: NodeId,
    t: NodeId,
    forward: bool,
    backward: bool,
) -> Result<Vec<NodeId>, Error> {
    let nt = formulas.add(Node::Not(t))?;
    let lit = |f: &mut Formulas, k: NodeId, positive: bool| {
        if positive { Ok(k) } else { negated(f, k) }
    };
    // Each clause as literals: (operand, sign), with the atom's literal
    // first.
    let mut lists: Vec<Vec<NodeId>> = Vec::new();
    match formulas.node(before) {
        Node::Atom(_) | Node::True | Node::False => {
            unreachable!("a literal is named by no atom")
        }
        Node::Not(a) => {
            if forward {
                lists.push(vec![nt, lit(formulas, a, false)?]);
            }
            if backward {
                lists.push(vec![t, a]);
            }
        }
        Node::And(a, b) => {
            if forward {
                lists.push(vec![nt, a]);
                lists.push(vec![nt, b]);
            }
            if backward {
                lists.push(vec![t, lit(formulas, a, false)?, lit(formulas, b, false)?]);
            }
        }
        Node::Or(a, b) => {
            if forward {
                lists.push(vec![nt, a, b]);
            }
            if backward {
                lists.push(vec![t, lit(formulas, a, false)?]);
                lists.push(vec![t, lit(formulas, b, false)?]);
            }
        }
        Node::Implies(a, b) => {
            if forward {
                lists.push(vec![nt, lit(formulas, a, false)?, b]);
            }
            if backward {
                lists.push(vec![t, a]);
                lists.push(vec![t, lit(formulas, b, false)?]);
            }
        }
        Node::Iff(a, b) => {
            if forward {
                lists.push(vec![nt, lit(formulas, a, false)?, b]);
                lists.push(vec![nt, a, lit(formulas, b, false)?]);
            }
            if backward {
                lists.push(vec![t, a, b]);
                lists.push(vec![t, lit(formulas, a, false)?, lit(formulas, b, false)?]);
            }
        }
        Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
    }
    let mut clauses = Vec::with_capacity(lists.len());
    for list in lists {
        if list.iter().any(|&k| constant(formulas, k) == Some(true)) {
            continue;
        }
        let kept: Vec<NodeId> = list
            .into_iter()
            .filter(|&k| constant(formulas, k).is_none())
            .collect();
        clauses.push(disjunction(formulas, &kept)?);
    }
    Ok(clauses)
}

/// Returns the value of a literal that is a constant or a negated one:
/// `⊤` and `¬⊥` true, `⊥` and `¬⊤` false; `None` for any other formula.
fn constant(formulas: &Formulas, id: NodeId) -> Option<bool> {
    match formulas.node(id) {
        Node::True => Some(true),
        Node::False => Some(false),
        Node::Not(a) => match formulas.node(a) {
            Node::True => Some(false),
            Node::False => Some(true),
            Node::Atom(_)
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => None,
            Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
        },
        Node::Atom(_) | Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => None,
        Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
    }
}

/// Returns whether every formula of `result` is in `form`. A literal is
/// an atom or a negated atom; a constant stands only alone.
fn in_form(result: &Sequent, form: Form) -> bool {
    let formulas = result.formulas();
    let literal = |id: NodeId| match formulas.node(id) {
        Node::Atom(_) => true,
        Node::Not(a) => matches!(formulas.node(a), Node::Atom(_)),
        Node::True
        | Node::False
        | Node::And(..)
        | Node::Or(..)
        | Node::Implies(..)
        | Node::Iff(..)
        | Node::Forall(_)
        | Node::Exists(_) => false,
    };
    // Whether a formula is built by `join` over parts that `part` accepts,
    // walked with a stack of its own.
    let built = |root: NodeId, join: Kind, part: &dyn Fn(NodeId) -> bool| {
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            let node = formulas.node(id);
            if is(node, join) {
                let (a, b) = node.operands();
                stack.extend([a, b].into_iter().flatten());
            } else if !part(id) {
                return false;
            }
        }
        true
    };
    let clause = |id: NodeId| built(id, Kind::Or, &literal);
    let cube = |id: NodeId| built(id, Kind::And, &literal);
    let alone = |id: NodeId| matches!(formulas.node(id), Node::True | Node::False);
    let roots = result.left().iter().chain(result.right());
    match form {
        Form::Nnf => roots.copied().all(|id| nnf(formulas, id)),
        Form::Cnf => roots
            .copied()
            .all(|id| alone(id) || built(id, Kind::And, &clause)),
        Form::Dnf => roots
            .copied()
            .all(|id| alone(id) || built(id, Kind::Or, &cube)),
        Form::Tseitin | Form::PlaistedGreenbaum => {
            result.right().is_empty()
                && result
                    .left()
                    .iter()
                    .all(|&id| matches!(formulas.node(id), Node::False) || clause(id))
        }
        Form::Prenex => roots
            .copied()
            .all(|id| !has_binder(formulas, prefixed(formulas, id, true))),
        Form::Miniscope => roots.copied().all(|id| {
            nnf(formulas, id) && !reaches(formulas, id, &|k| law(Stage::Inward, formulas, k))
        }),
        Form::Skolem | Form::Herbrand => {
            let strong = |exists: bool| move |k: NodeId| law(Stage::Strong { exists }, formulas, k);
            (form == Form::Herbrand || result.right().is_empty())
                && roots.clone().copied().all(|id| nnf(formulas, id))
                && result
                    .left()
                    .iter()
                    .all(|&id| !reaches(formulas, id, &strong(true)))
                && result
                    .right()
                    .iter()
                    .all(|&id| !reaches(formulas, id, &strong(false)))
        }
        Form::Clausal => {
            result.right().is_empty()
                && result.left().iter().all(|&id| {
                    let matrix = prefixed(formulas, id, false);
                    matches!(formulas.node(matrix), Node::False) || clause(matrix)
                })
        }
    }
}

/// Returns the formula under the quantifiers in front of `id`: every
/// quantifier where `both` is set, else `∀` alone.
fn prefixed(formulas: &Formulas, mut id: NodeId, both: bool) -> NodeId {
    loop {
        match formulas.node(id) {
            Node::Forall(a) => id = a,
            Node::Exists(a) if both => id = a,
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..)
            | Node::Exists(_) => return id,
        }
    }
}

/// Returns whether `id` holds a quantifier.
fn has_binder(formulas: &Formulas, id: NodeId) -> bool {
    reaches(formulas, id, &|k| {
        matches!(formulas.node(k), Node::Forall(_) | Node::Exists(_)).then_some(Rule::Vacuous)
    })
}

/// Returns whether some subformula of `id` is one `found` answers for,
/// walked with a stack of its own.
fn reaches(formulas: &Formulas, id: NodeId, found: &dyn Fn(NodeId) -> Option<Rule>) -> bool {
    let mut seen = crate::hash::HashSet::default();
    let mut stack = vec![id];
    while let Some(k) = stack.pop() {
        if !seen.insert(k) {
            continue;
        }
        if found(k).is_some() {
            return true;
        }
        let (a, b) = formulas.node(k).operands();
        stack.extend([a, b].into_iter().flatten());
    }
    false
}

/// Returns whether the formula `id` has no `→` or `↔` and a `¬` before
/// atoms alone, walked with a stack of its own.
fn nnf(formulas: &Formulas, id: NodeId) -> bool {
    let mut stack = vec![id];
    while let Some(id) = stack.pop() {
        match formulas.node(id) {
            Node::Atom(_) | Node::True | Node::False => {}
            Node::Not(a) => {
                if !matches!(formulas.node(a), Node::Atom(_)) {
                    return false;
                }
            }
            Node::And(a, b) | Node::Or(a, b) => stack.extend([a, b]),
            Node::Forall(a) | Node::Exists(a) => stack.push(a),
            Node::Implies(..) | Node::Iff(..) => return false,
        }
    }
    true
}

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
