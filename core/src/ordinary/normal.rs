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

use super::{Formulas, Logic, Node, NodeId, Outcome, Sequent, Verdict, decide_within};
use crate::Error;
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
}

impl Form {
    /// Every form, in the order of [`NAMES`](Self::NAMES).
    pub const ALL: &'static [Self] = &[
        Self::Nnf,
        Self::Cnf,
        Self::Dnf,
        Self::Tseitin,
        Self::PlaistedGreenbaum,
    ];

    /// The forms' names, which [`name`](Self::name) writes and `FromStr`
    /// reads.
    pub const NAMES: &'static [&'static str] =
        &["nnf", "cnf", "dnf", "tseitin", "plaisted-greenbaum"];

    /// Returns the form's name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Nnf => "nnf",
            Self::Cnf => "cnf",
            Self::Dnf => "dnf",
            Self::Tseitin => "tseitin",
            Self::PlaistedGreenbaum => "plaisted-greenbaum",
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
        }
    }

    /// Returns what the form keeps of its input.
    pub const fn promise(self) -> Promise {
        match self {
            Self::Nnf | Self::Cnf | Self::Dnf => Promise::Equivalent,
            Self::Tseitin | Self::PlaistedGreenbaum => Promise::Equisatisfiable,
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
        Self::ALL
            .iter()
            .copied()
            .find(|form| form.name() == name)
            .ok_or_else(|| Error::UnknownName {
                what: "normal form",
                name: name.into(),
                known: Self::NAMES,
            })
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
}

impl Promise {
    /// Returns the promise's name: `equivalent`, `equisatisfiable`.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Equivalent => "equivalent",
            Self::Equisatisfiable => "equisatisfiable",
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
        }
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
}

impl Claim {
    /// Returns the claim's name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Equivalent => "equivalent",
            Self::Extends => "extends",
            Self::Restricts => "restricts",
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
        }
    }
}

/// A claim decided.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Checked {
    /// The claim.
    pub claim: Claim,
    /// The classical sequent whose validity is the claim.
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
/// `{"version": 21, "deduxit": "0.6.0", "form": "cnf", "promise":
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
        let options = super::Options::default()
            .with_logic(Logic::Classical)
            .with_method(CLAIMS);
        // A value checked before and changed since keeps no old check.
        self.checks.clear();
        let mut checks = Vec::new();
        for (claim, sequent) in self.claims()? {
            let outcome = decide_within(&sequent, &options, search, limits, &mut stop)?;
            if matches!(outcome.verdict, Verdict::NotValid) {
                return Err(Error::ClaimFails {
                    claim: self.form.title().to_owned(),
                    reason: format!("it is not so that {}", claim.description()),
                });
            }
            checks.push(Checked {
                claim,
                sequent,
                outcome,
            });
        }
        self.checks = checks;
        Ok(())
    }
}

/// Returns the formulas of `sequent` added to `formulas`, built again by
/// their atoms' names, so that they are formulas of that arena whatever
/// arena the sequent has.
fn transfer(
    sequent: &Sequent,
    formulas: &mut Formulas,
) -> Result<(Vec<NodeId>, Vec<NodeId>), Error> {
    let from = sequent.formulas();
    let mut image: Vec<NodeId> = Vec::with_capacity(from.len());
    for k in 0..from.len() {
        let map = |x: NodeId| image[x.index()];
        let id = match from.node(NodeId(k as u32)) {
            Node::Atom(a) => formulas.atom(from.atom_name(a))?,
            Node::True => formulas.add(Node::True)?,
            Node::False => formulas.add(Node::False)?,
            Node::Not(a) => formulas.add(Node::Not(map(a)))?,
            Node::And(a, b) => formulas.add(Node::And(map(a), map(b)))?,
            Node::Or(a, b) => formulas.add(Node::Or(map(a), map(b)))?,
            Node::Implies(a, b) => formulas.add(Node::Implies(map(a), map(b)))?,
            Node::Iff(a, b) => formulas.add(Node::Iff(map(a), map(b)))?,
        };
        image.push(id);
    }
    let map = |ids: &[NodeId]| ids.iter().map(|x| image[x.index()]).collect();
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
        Form::Tseitin | Form::PlaistedGreenbaum => &[],
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
        },
        Stage::Negations => match node {
            Node::Not(a) => match formulas.node(a) {
                Node::Not(_) => Some(Rule::DoubleNegation),
                Node::And(..) => Some(Rule::DeMorganAnd),
                Node::Or(..) => Some(Rule::DeMorganOr),
                Node::True => Some(Rule::NotTrue),
                Node::False => Some(Rule::NotFalse),
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
        },
    }
}

/// Returns what `rule` makes of the formula `id`, at whose root [`law`]
/// found it, added to the arena.
fn apply(rule: Rule, formulas: &mut Formulas, id: NodeId) -> Result<NodeId, Error> {
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
        Rule::Negate | Rule::Define | Rule::TrueHypothesis => {
            unreachable!("no law of an equivalent form")
        }
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
                };
                if mapped == node {
                    id
                } else {
                    formulas.add(mapped)?
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
            return Err(Error::Refused(Refusal::Stopped {
                phase: Phase::Rewrite,
            }));
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
    let mut budget = Budget {
        limits,
        work: 0,
        held: 0,
        stop,
    };
    let (formulas, left, right, steps) = match form.promise() {
        Promise::Equivalent => equivalent(sequent, form, &mut budget)?,
        Promise::Equisatisfiable => encoded(sequent, form, &mut budget)?,
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
    let (mut left, mut right) = (sequent.left().to_vec(), sequent.right().to_vec());
    let split = left.len();
    let mut roots: Vec<NodeId> = left.iter().chain(&right).copied().collect();
    let mut passes = Passes::default();
    let mut steps = Vec::new();
    for &stage in stages(form) {
        loop {
            let size = passes.reach(&formulas, &roots);
            budget.pass(passes.order.len(), size, steps.len())?;
            let Some((before, rule)) = passes.redex(stage, &formulas, &roots) else {
                break;
            };
            let nodes = formulas.len();
            let after = apply(rule, &mut formulas, before)?;
            roots = passes.substitute(&mut formulas, &roots, before, after)?;
            (left, right) = (roots[..split].to_vec(), roots[split..].to_vec());
            let step = Step {
                rule,
                before,
                after,
                left: left.clone(),
                right: right.clone(),
                added: Vec::new(),
            };
            budget.hold(&step, formulas.len() - nodes)?;
            steps.push(step);
        }
    }
    Ok((formulas, left, right, steps))
}

/// The polarity of a formula that must be true where the sequent fails.
const TRUE: u8 = 1;
/// The polarity of a formula that must be false where the sequent fails.
const FALSE: u8 = 2;

/// Returns whether `id` is a literal of a clause: an atom, a constant or
/// the negation of one.
fn is_literal(formulas: &Formulas, id: NodeId) -> bool {
    match formulas.node(id) {
        Node::Atom(_) | Node::True | Node::False => true,
        Node::Not(a) => matches!(formulas.node(a), Node::Atom(_) | Node::True | Node::False),
        Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => false,
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
    }
}

/// Returns the prefix of the new atoms' names: `t`, or `t` with as many
/// `_` after it as make no name of the input the prefix and digits.
fn prefix(formulas: &Formulas) -> String {
    let mut prefix = String::from("t");
    let taken = |prefix: &str| {
        formulas.atom_names().iter().any(|name| {
            name.strip_prefix(prefix)
                .is_some_and(|rest| !rest.is_empty() && rest.bytes().all(|c| c.is_ascii_digit()))
        })
    };
    while taken(&prefix) {
        prefix.push('_');
    }
    prefix
}

/// Encodes the sequent's negation as clauses: the formulas right of `⊢`
/// moved left negated, then every compound subformula named by a new atom,
/// the innermost first, with the clauses of its definition.
fn encoded(sequent: &Sequent, form: Form, budget: &mut Budget<'_>) -> Result<Made, Error> {
    let mut formulas = sequent.formulas().clone();
    let mut left = sequent.left().to_vec();
    let mut steps = Vec::new();
    let mut passes = Passes::default();
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
        };
        budget.hold(&step, formulas.len() - nodes)?;
        steps.push(step);
    }
    let size = passes.reach(&formulas, &left);
    budget.pass(passes.order.len(), size, steps.len())?;
    let order = passes.order.clone();
    let polarity = polarities(&formulas, &order, &left);
    let prefix = prefix(&formulas);
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
        })?;
        // A negation of a named subformula is a literal already.
        if is_literal(&formulas, before) {
            name.insert(x, before);
            continue;
        }
        count += 1;
        let t = formulas.atom(&format!("{prefix}{count}"))?;
        let p = polarity.get(&x).copied().unwrap_or(0);
        let (forward, backward) = match form {
            Form::PlaistedGreenbaum => (p & TRUE != 0, p & FALSE != 0),
            Form::Nnf | Form::Cnf | Form::Dnf | Form::Tseitin => (true, true),
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
        };
        budget.hold(&step, 0)?;
        steps.push(step);
    }
    left.extend_from_slice(&clauses);
    Ok((formulas, left, Vec::new(), steps))
}

/// Returns the polarity of every node of `order` (the nodes `roots`
/// reach, in the arena's order) where the roots are true: `TRUE` where it
/// must be true, `FALSE` where false, both under an equivalence.
fn polarities(formulas: &Formulas, order: &[NodeId], roots: &[NodeId]) -> HashMap<NodeId, u8> {
    let mut polarity: HashMap<NodeId, u8> = HashMap::with_capacity(order.len());
    for &r in roots {
        polarity.insert(r, TRUE);
    }
    // Every node comes after its operands, so the reverse order meets
    // each before its operands.
    for &id in order.iter().rev() {
        let p = polarity.get(&id).copied().unwrap_or(0);
        let flipped = ((p & TRUE) << 1) | ((p & FALSE) >> 1);
        let mut give = |k: NodeId, q: u8| *polarity.entry(k).or_insert(0) |= q;
        match formulas.node(id) {
            Node::Atom(_) | Node::True | Node::False => {}
            Node::Not(a) => give(a, flipped),
            Node::And(a, b) | Node::Or(a, b) => {
                give(a, p);
                give(b, p);
            }
            Node::Implies(a, b) => {
                give(a, flipped);
                give(b, p);
            }
            Node::Iff(a, b) => {
                give(a, TRUE | FALSE);
                give(b, TRUE | FALSE);
            }
        }
    }
    polarity
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
        },
        Node::Atom(_) | Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => None,
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
        | Node::Iff(..) => false,
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
    }
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
