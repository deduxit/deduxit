// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Calculi given as data: a rule file, a derivation whose nodes name its
//! rules, and a checker that says whether the derivation is one of that
//! calculus. Needs the cargo feature `serialize` (on by default), since
//! a calculus enters as a document.
//!
//! A [`Calculus`] is read from a rule file: its connectives, the zones
//! of its sequents, and its rules as premises and a conclusion over
//! schematic formulas and contexts. A [`Derivation`] holds terms and
//! nodes; [`Derivation::check`] matches every node against the rule it
//! names. The verdict is "valid in the calculus given": the rules are
//! the user's, and nothing here says they are sound.
//!
//! The checker never searches for a proof, and it stands beside the
//! checkers of the [`linear`](crate::linear) and
//! [`ordinary`](crate::ordinary) suites, which it does not replace.
//!
//! # Examples
//!
//! Checking a derivation of `a ∧ b ⊢ a` in the sequent calculus LK:
//!
//! ```
//! use deduxit::rules::{Calculus, Derivation, LK, Node, Step};
//! use deduxit::{Limits, wire};
//!
//! let mut file = serde_json::Deserializer::from_str(LK);
//! let lk: Calculus = wire::upgrade(&mut file, &Limits::default())?;
//!
//! let mut derivation = Derivation::new("LK", 2);
//! let a = derivation.terms_mut().atom("a", "formula")?;
//! let b = derivation.terms_mut().atom("b", "formula")?;
//! let both = derivation.terms_mut().apply("∧", &[a, b])?;
//! let axiom = [vec![a], vec![a]];
//! derivation.push(Node::new(&axiom, Step::Rule("ax")))?;
//! // `∧L₁` acts on the hypothesis at position 0.
//! let root = [vec![both], vec![a]];
//! derivation.push(Node::new(&root, Step::Rule("∧L₁")).premises(&[0]).positions(&[0]))?;
//!
//! derivation.check(&lk, &Limits::default(), |_| false)?;
//! # Ok::<(), deduxit::Error>(())
//! ```

/// The calculus: a rule file, compiled.
mod calculus;
/// The checker.
mod check;
/// The derivation.
mod derivation;
/// A derivation laid out as a tree for the exports.
mod draw;
/// A derivation laid out as a Fitch proof.
pub(crate) mod fitch;
/// Building a derivation step by step.
#[cfg(all(feature = "interactive", feature = "parse"))]
pub(crate) mod interactive;
/// The derivations of LK and LJ in this form.
mod ordinary;
/// Terms and sequents as text.
mod print;
/// The signed tableaux of classical logic in this form.
#[cfg(feature = "interactive")]
mod tableau;
/// The terms of a derivation.
mod terms;
/// Terms and sequents read from text.
#[cfg(feature = "parse")]
mod text;

pub use calculus::{Associativity, Calculus, Discipline, Fixity};
pub use derivation::{Derivation, Node, NodeId, NodeRef, ScopeId, Step};
pub use draw::Drawing;
pub use fitch::{Fitch, FitchOptions};
#[cfg(all(feature = "interactive", feature = "parse"))]
pub use interactive::{Applicable, Apply, Derive, GoalId, Interactive, LineId, Undone};
pub use print::Shown;
pub use terms::{Term, TermId, Terms};

/// The name of the sort every calculus has: that of formulas.
pub const FORMULA: &str = "formula";

/// The rule file of LK, the sequent calculus of classical propositional
/// logic, with the rules [`ordinary::Derivation::check`] checks:
/// Gentzen's, with explicit weakening and contraction, sequents of two
/// multisets, and every rule of two premises once with the context in
/// both and once split between them (`∧R` and `∧R split`). It has no
/// cut.
///
/// [`ordinary::Derivation::check`]: crate::ordinary::Derivation::check
pub const LK: &str = include_str!("calculi/lk.json");

/// The rule file of LJ, the sequent calculus of intuitionistic
/// propositional logic: LK's rules with at most one formula right of
/// `⊢`, `¬A` also read as `A → ⊥` (`¬R ⊥`, `¬L ⊥`), and Gentzen's cut.
pub const LJ: &str = include_str!("calculi/lj.json");

/// The rule file of LM, the sequent calculus of minimal propositional
/// logic: LJ with exactly one formula right of `⊢` and without `⊥L`.
pub const LM: &str = include_str!("calculi/lm.json");

/// The rule file of LL, Girard's one-sided sequent calculus of classical
/// linear logic, propositional and without cut, in which the
/// [`linear`](crate::linear) suite's derivations are written: a sequent
/// is a multiset of formulas in negation normal form, the dual of an
/// atom is the atom under `~`, and promotion `!` keeps `?` on its whole
/// context (`["?", "Γ"]`).
pub const LL: &str = include_str!("calculi/ll.json");

/// The rule file of Smullyan's signed analytic tableau of classical
/// propositional logic, written as a calculus: a node is the set of the
/// signed formulas (`T A`, `F A`) on its branch, an expansion keeps the
/// formula it expands, and a branch closes by `×` on a formula signed
/// both ways, by `T⊥` or by `F⊤`. `Derivation::from_tableau` writes the
/// ordinary suite's tableaux in it (feature `interactive`).
pub const TABLEAU: &str = include_str!("calculi/tableau.json");

/// The rule file of NJ, Gentzen's natural deduction for intuitionistic
/// propositional logic: a judgement is one formula, a hypothesis is a
/// node of its own, and `→I`, `∨E`, `¬I` and `↔I` discharge hypotheses
/// by scope. `¬I` and `¬E` are rules of their own, ex falso is `⊥E`, and
/// every elimination takes its major premise first.
pub const NJ: &str = include_str!("calculi/nj.json");

/// The rule file of NK, natural deduction for classical propositional
/// logic: [`NJ`] with the classical absurdity rule `RAA` (Prawitz's
/// `⊥c`), which concludes `A` from `⊥` and discharges `¬A`.
pub const NK: &str = include_str!("calculi/nk.json");

/// The rule file of NM, natural deduction for minimal propositional
/// logic: [`NJ`] without ex falso, so `⊥` has no rule of its own.
pub const NM: &str = include_str!("calculi/nm.json");

/// The rule file of NK with double negation elimination `¬¬E` in place
/// of `RAA`.
pub const NK_DNE: &str = include_str!("calculi/nk-dne.json");

/// The rule file of NK with the law of excluded middle `LEM`, which
/// concludes `A ∨ ¬A` from nothing, in place of `RAA`, as Gentzen gave
/// the calculus.
pub const NK_LEM: &str = include_str!("calculi/nk-lem.json");

/// The rule file of [`NJ`] with `¬A` written `A → ⊥`: it has no `¬`, and
/// `→I` and `→E` do the work of `¬I` and `¬E`.
pub const NJ_DEFINED: &str = include_str!("calculi/nj-defined.json");

/// The rule file of [`NK`] with `¬A` written `A → ⊥`: it has no `¬`,
/// and `RAA` discharges `A → ⊥`.
pub const NK_DEFINED: &str = include_str!("calculi/nk-defined.json");

/// The rule file of [`NM`] with `¬A` written `A → ⊥`: it has no `¬`.
pub const NM_DEFINED: &str = include_str!("calculi/nm-defined.json");

/// The JSON Schema (draft 2020-12) of a rule file: the shape of the
/// document, for an editor or a program that writes one. Reading a rule
/// file checks more than a schema can say: that every name is declared,
/// every pattern well sorted and every rule matched without a search.
pub const SCHEMA: &str = include_str!("calculus.schema.json");
