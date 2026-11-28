// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Resolution at first order: binary resolution with the most general
//! unifier, and factoring, on the clausal form of a sequent's negation,
//! at teaching scale, every refutation checked.
//!
//! A clause is a disjunction of literals whose variables are the clause's
//! own, universally quantified. Two clauses resolve on a literal of each,
//! of one predicate and opposite signs, when a substitution (their most
//! general unifier) makes the two atoms equal: the resolvent is the rest
//! of both under that substitution. Factoring unifies two literals of one
//! clause of the same sign. Each inference renames the parents' variables
//! apart first; a clause's variables are numbered in the order they first
//! occur, so an inference made again makes the same clause.
//!
//! [`resolve`] makes the clausal form of `Γ ⊢ Δ`'s negation
//! ([`normal::Form::Clausal`]) and searches by the given clause: the
//! lightest clause not yet used is resolved with every clause used and
//! factored, its consequences kept unless a clause kept already subsumes
//! them. The clauses of the negated goals are the set of support: the
//! hypotheses' clauses are resolved with them, not with each other, until
//! the set of support runs out. A refutation is checked step by step
//! ([`Refutation::check`]: each unifier computed again, each clause made
//! again), then read back as a derivation of LK of the sequent as written
//! where it reaches: its ground instances, the refutation's substitutions
//! composed, are Herbrand's ([`herbrand`](super::super::herbrand)), and the
//! Skolem terms become eigenvariables again.
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::ordinary::resolution::first_order::{self, Answer};
//! use deduxit::ordinary::resolution::Options;
//! use deduxit::ordinary::Sequent;
//!
//! let sequent: Sequent = "forall x. (p(x) -> q(f(x))), p(a) |- exists y. q(y)".parse()?;
//! let found = first_order::resolve(&sequent, &Options::default(), &deduxit::Limits::default(), |_| false)?;
//! let Answer::Refuted { refutation, derivation, .. } = &found.answer else { panic!("refuted") };
//! assert_eq!(refutation.inferences().len(), 2);
//! assert!(derivation.is_some());
//! # Ok::<(), deduxit::Error>(())
//! ```

mod check;
mod clauses;
mod read_back;
mod search;
mod text;

use self::read_back::read_back;
use self::search::{Found, Search, given_clauses};
use self::text::{BINDING_TEXT, Capped, clause_text, write_term_to};
use super::super::normal::{self, NormalForm};
use super::super::{Derivation, Formulas, NodeId, Sequent};
use super::{Bound, Options};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::HashMap;
use crate::limits::{Limits, Progress};

/// A literal: an atom of the clauses' arena, whose metavariables are its
/// clause's variables, or its negation.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Literal {
    /// Whether it is the atom itself.
    pub positive: bool,
    /// The atom.
    pub atom: NodeId,
}

/// A clause: its literals, each once, and how many variables it has, the
/// metavariables `0` to `variables − 1` of its atoms.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Clause {
    /// The literals, in the order the inference that made it gives.
    literals: Vec<Literal>,
    /// The number of its variables.
    variables: u32,
}

impl Clause {
    /// Returns the literals.
    pub fn literals(&self) -> &[Literal] {
        &self.literals
    }

    /// Returns the number of its variables.
    pub const fn variables(&self) -> u32 {
        self.variables
    }

    /// Returns whether it is the empty clause `□`.
    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }
}

/// An inference: resolution of two clauses on a literal of each, or a
/// factor of one clause on two of its literals, each by its position.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(tag = "rule", rename_all = "snake_case"))]
pub enum Step {
    /// The resolvent of two clauses.
    Resolution {
        /// The first clause.
        first: usize,
        /// The second clause.
        second: usize,
        /// The literal of the first.
        first_literal: usize,
        /// The literal of the second, of opposite sign.
        second_literal: usize,
    },
    /// A factor: two literals of one sign of a clause unified, the second
    /// dropped.
    Factor {
        /// The clause, `parent` in JSON.
        #[cfg_attr(feature = "serialize", serde(rename = "parent"))]
        clause: usize,
        /// The literal kept.
        first_literal: usize,
        /// The literal dropped.
        second_literal: usize,
    },
    /// A paramodulant: a subterm of a literal of one clause rewritten by
    /// an equation of another, under their most general unifier.
    Paramodulation {
        /// The clause of the equation.
        from: usize,
        /// The equation `s = t`, a positive literal of `from`.
        from_literal: usize,
        /// The clause rewritten.
        into: usize,
        /// The literal rewritten.
        into_literal: usize,
        /// Where the subterm stands: its place among the subterms of the
        /// literal's arguments read left to right, each before its own
        /// arguments, from 0. A variable's place is refused.
        position: usize,
        /// Whether the equation is read right to left: `t` unified with the
        /// subterm and `s` put for it.
        reverse: bool,
    },
}

/// An inference made: its step, the unifier, and the clause it made.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Inference {
    /// The step.
    pub step: Step,
    /// The unifier: each variable it binds, numbered across the step's
    /// parents (the first's from 0, the second's after them), with its term
    /// over the variables left free.
    pub bindings: Vec<(u32, FoTermId)>,
    /// The clause made, by its position.
    pub clause: usize,
}

/// What an inference makes, with what the read-back needs of it.
#[derive(Clone, Debug)]
struct Made {
    /// The unifier's bindings.
    bindings: Vec<(u32, FoTermId)>,
    /// The clause.
    clause: Clause,
    /// Each variable of the step, its term under the unifier.
    terms: Vec<FoTermId>,
    /// Each variable of the step, the clause's variable it became, if any.
    renamed: Vec<Option<u32>>,
    /// How many variables the first parent has.
    first: u32,
}

/// Clauses over one arena: the clausal form's, and every clause an
/// inference made.
#[derive(Clone, Debug)]
pub struct Clauses {
    /// The arena of the atoms and terms.
    formulas: Formulas,
    /// The clauses, the given ones first.
    items: Vec<Clause>,
    /// How many are given.
    given: usize,
    /// The most symbols a clause's atoms may unfold to, `limits.terms`.
    most_terms: Option<u64>,
    /// The symbols each term unfolds to, as far as computed.
    sizes: HashMap<FoTermId, u64>,
}

impl Clauses {
    /// Returns the arena.
    pub const fn formulas(&self) -> &Formulas {
        &self.formulas
    }

    /// Returns the clauses, the given ones first.
    pub fn clauses(&self) -> &[Clause] {
        &self.items
    }

    /// Returns how many clauses are given.
    pub const fn given(&self) -> usize {
        self.given
    }

    /// Returns the clause at `k` as text, its variables `X1`, `X2`, …:
    /// `¬p(X1) ∨ q(f(X1))`, `□` for the empty clause.
    pub fn text(&self, k: usize) -> String {
        let Some(clause) = self.items.get(k) else {
            return String::new();
        };
        clause_text(&self.formulas, clause, "X")
    }

    /// Returns the unifier of an inference as text, the first parent's
    /// variables `X1`, … and the second's `Y1`, …: `X1 := f(Y1)`. A
    /// binding's term past 4 096 bytes is cut there and ends with `…`: a
    /// term that shares its parts may be exponentially longer written out
    /// than the arena that holds it.
    pub fn bindings_text(&self, inference: &Inference) -> Vec<String> {
        let first = match inference.step {
            Step::Resolution { first, .. } => self.items.get(first),
            Step::Factor { clause, .. } => self.items.get(clause),
            Step::Paramodulation { from, .. } => self.items.get(from),
        }
        .map_or(0, Clause::variables);
        let name = |k: u32| {
            if k < first {
                format!("X{}", k + 1)
            } else {
                format!("Y{}", k - first + 1)
            }
        };
        inference
            .bindings
            .iter()
            .map(|&(k, t)| {
                let mut text = Capped {
                    text: String::new(),
                    room: BINDING_TEXT,
                };
                if write_term_to(&self.formulas, t, &mut text, &|j| name(j)).is_err() {
                    text.text.push('…');
                }
                format!("{} := {}", name(k), text.text)
            })
            .collect()
    }

    /// Makes the clause `step` gives and adds it, unless the step does not
    /// apply: its literals are not of one predicate and opposite signs
    /// (the same sign for a factor), or their atoms do not unify (two
    /// symbols clash, or a variable would occur in its own binding).
    ///
    /// # Errors
    ///
    /// [`Error::InvalidOption`] (key `step`) for a position outside the
    /// clauses or their literals, and for a step that does not apply, with
    /// why; [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub fn infer(&mut self, step: Step) -> Result<Inference, Error> {
        let made = self.made(step)?.map_err(|why| Error::InvalidOption {
            key: "step",
            message: why.to_owned(),
        })?;
        self.items.push(made.clause);
        Ok(Inference {
            step,
            bindings: made.bindings,
            clause: self.items.len() - 1,
        })
    }
}

/// A refutation: the clauses given, and the inferences that make `□`
/// from them, each clause made after its parents.
#[derive(Clone, Debug)]
pub struct Refutation {
    /// The clauses: the given ones, then one per inference.
    clauses: Clauses,
    /// The inferences, in order.
    inferences: Vec<Inference>,
}

impl Refutation {
    /// Returns the clauses: the given ones, then one per inference.
    pub const fn clauses(&self) -> &Clauses {
        &self.clauses
    }

    /// Returns the inferences, in order.
    pub fn inferences(&self) -> &[Inference] {
        &self.inferences
    }
}

/// What [`resolve`] found.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Answer {
    /// The clauses are unsatisfiable, so the sequent is valid in classical
    /// logic: the refutation, checked, and the derivation of LK read back
    /// from it where the read-back reaches.
    Refuted {
        /// The refutation, checked.
        refutation: Box<Refutation>,
        /// The derivation of LK of the sequent, checked; `None` where the
        /// read-back did not reach, and the verdict rests on the
        /// refutation and Skolem's theorem.
        derivation: Option<Box<Derivation>>,
        /// Why the read-back did not reach, where it did not.
        not_read_back: Option<String>,
    },
    /// Every inference was made and none gave `□`: the clauses have a
    /// model, which resolution does not build, so the sequent is not
    /// valid; no checked countermodel stands for it.
    Saturated,
    /// A bound ended the search before it decided.
    Unknown(Bound),
}

/// What a first-order search cost.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Statistics {
    /// The clauses kept: the given ones and the inferences' that no clause
    /// subsumed.
    pub clauses: u64,
    /// The inferences made.
    pub steps: u64,
}

/// A sequent's clauses decided by first-order resolution.
///
/// # JSON
///
/// With the feature `serialize` it is a document at wire level 29,
/// written only; see [`wire`](crate::wire).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct FirstOrder {
    /// The sequent.
    pub sequent: Sequent,
    /// The clausal form of its negation, with its steps.
    pub clausal: NormalForm,
    /// The clauses the search started from, in the clausal form's order.
    pub clauses: Clauses,
    /// What the search found.
    pub answer: Answer,
    /// What it cost.
    pub statistics: Statistics,
}

/// The most matchings one subsumption test tries.
const SUBSUMPTION_LIMIT: u64 = 1 << 12;

/// How many places of the literals paramodulation walks between two asks
/// of the stop: a pair of clauses with wide literals walks millions.
const PLACES_PER_POLL: u64 = 1 << 12;

/// The most ground instances the read-back collects.
const MOST_INSTANCES: usize = 1 << 12;

/// Makes the clausal form of `sequent`'s negation and refutes its clauses
/// by first-order resolution, within the bounds of `options` (its method
/// is ignored: the search is saturation), `limits` and `stop`; checks a
/// refutation found and reads it back as a derivation of LK where it
/// reaches. A sequent with `=` is resolved by `options.equality`: with
/// reflexivity among its hypotheses and paramodulation, or with every
/// equality axiom among them; either way a saturated set has a model
/// where `=` is identity. A derivation read back derives the sequent with
/// those hypotheses.
///
/// # Errors
///
/// [`normal::rewrite`]'s errors for the clausal form; [`Error::ClaimFails`]
/// for a refutation that fails its check, a defect; [`Refusal::Memory`](crate::Refusal::Memory)
/// past `limits.memory_bytes`.
pub fn resolve(
    sequent: &Sequent,
    options: &Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<FirstOrder, Error> {
    let sequent = with_equality(sequent, limits, options.equality)?;
    resolve_dyn(&sequent, options, limits, &mut stop)
}

/// Returns `sequent` with what `equality` needs of a sequent that uses
/// `=`: every equality axiom
/// ([`with_axioms`](crate::ordinary::equality::with_axioms)), or
/// reflexivity alone for paramodulation
/// ([`with_reflexivity`](crate::ordinary::equality::with_reflexivity)).
/// Either way a saturated set has a model that reads `=` as identity once
/// divided by it.
///
/// # Errors
///
/// Their refusals.
pub(crate) fn with_equality<'a>(
    sequent: &'a Sequent,
    limits: &Limits,
    equality: super::Equality,
) -> Result<std::borrow::Cow<'a, Sequent>, Error> {
    let with = match equality {
        super::Equality::Axioms => crate::ordinary::equality::with_axioms(sequent, limits)?,
        super::Equality::Paramodulation => {
            crate::ordinary::equality::with_reflexivity(sequent, limits)?
        }
    };
    Ok(match with {
        Some(with) => std::borrow::Cow::Owned(with),
        None => std::borrow::Cow::Borrowed(sequent),
    })
}

/// [`resolve`], with the stop as a trait object.
fn resolve_dyn(
    sequent: &Sequent,
    options: &Options,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<FirstOrder, Error> {
    refuse_open_witnesses(sequent)?;
    let clausal = normal::rewrite(
        sequent,
        &normal::Options::default().with_form(normal::Form::Clausal),
        limits,
        &mut *stop,
    )?;
    let (clauses, support) = given_clauses(sequent, &clausal, true, limits, &mut *stop)?;
    let mut search = Search::new(clauses.clone(), support, options, limits);
    let found = search.run(&mut *stop)?;
    let statistics = search.statistics;
    let answer = match found {
        Found::Refuted(empty) => {
            let refutation = search.refutation(empty);
            refutation.check_within(limits, &mut *stop).map_err(|e| {
                if let Error::Refused(_) = e {
                    e
                } else {
                    Error::ClaimFails {
                        claim: "the first-order resolution search".to_owned(),
                        reason: format!("its refutation fails the check: {e}"),
                    }
                }
            })?;
            let (derivation, not_read_back) =
                match read_back(sequent, &clausal, &refutation, limits, &mut *stop) {
                    Ok(derivation) => (Some(Box::new(derivation)), None),
                    Err(error) => (None, Some(error.to_string())),
                };
            Answer::Refuted {
                refutation: Box::new(refutation),
                derivation,
                not_read_back,
            }
        }
        Found::Saturated => Answer::Saturated,
        Found::Unknown(bound) => Answer::Unknown(bound),
    };
    Ok(FirstOrder {
        sequent: sequent.clone(),
        clausal,
        clauses,
        answer,
        statistics,
    })
}

/// Returns the clausal form of `sequent`'s negation and its clauses, from
/// which [`Clauses::infer`] makes a refutation step by step.
///
/// # Errors
///
/// [`normal::rewrite`]'s errors; [`Error::ClaimFails`] for a hypothesis of
/// the clausal form that is no clause, a defect.
pub fn clauses_of(
    sequent: &Sequent,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<(NormalForm, Clauses), Error> {
    let sequent = &*with_equality(sequent, limits, super::Equality::Paramodulation)?;
    refuse_open_witnesses(sequent)?;
    let clausal = normal::rewrite(
        sequent,
        &normal::Options::default().with_form(normal::Form::Clausal),
        limits,
        &mut stop,
    )?;
    let (clauses, _) = given_clauses(sequent, &clausal, false, limits, &mut stop)?;
    Ok((clausal, clauses))
}

/// Refuses a sequent whose formulas hold an open witness of a session: a
/// clause's variables are metavariables too, and one would be taken for
/// the other.
///
/// # Errors
///
/// [`Error::InvalidOption`] (key `sequent`) for such a sequent.
fn refuse_open_witnesses(sequent: &Sequent) -> Result<(), Error> {
    let formulas = sequent.formulas();
    let tables = formulas.first_order();
    let roots = sequent.left().iter().chain(sequent.right()).copied();
    if formulas
        .terms_in(roots, std::iter::empty())
        .any(|t| matches!(tables.terms[t.index()], FoTerm::Meta(_)))
    {
        return Err(Error::InvalidOption {
            key: "sequent",
            message: "an open witness stands only in a proof in progress".to_owned(),
        });
    }
    Ok(())
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::limits::{Phase, Refusal};
    use crate::ordinary::Node;

    /// Returns what first-order resolution finds for `text` by default.
    fn found(text: &str) -> FirstOrder {
        let sequent: Sequent = text.parse().unwrap();
        resolve(&sequent, &Options::default(), &Limits::default(), |_| false).unwrap()
    }

    /// Valid sequents are refuted, every refutation checked and read back
    /// as a checked derivation of LK of the sequent as written.
    #[test]
    fn valid_sequents_are_refuted_and_read_back() {
        for text in [
            "forall x. (p(x) -> q(f(x))), p(a) |- exists y. q(y)",
            "|- exists x. (p(x) -> forall y. p(y))",
            "forall x. (p(x) -> p(f(x))), p(a) |- p(f(f(a)))",
            "exists x. forall y. r(x, y) |- forall y. exists x. r(x, y)",
            "forall x. (p(x) \\/ p(f(x))) |- exists x. p(x)",
            "|- (forall x. p(x)) <-> ~(exists x. ~p(x))",
            "~((exists x. p(x)) -> (r \\/ p(a))) |- forall y. ((exists x. p(x)) -> (exists x. p(x)))",
            // A closed instance of an open step's quantifier is another
            // node, which the clausal form left alone.
            "forall y. exists x. p(x, y), exists z. (q(z) /\\ exists x. p(x, a)) |- exists z. q(z)",
        ] {
            let found = found(text);
            let Answer::Refuted {
                refutation,
                derivation,
                not_read_back,
            } = &found.answer
            else {
                panic!("{text}: {:?}", found.answer);
            };
            refutation.check(|_| false).unwrap();
            let derivation = derivation
                .as_ref()
                .unwrap_or_else(|| panic!("{text}: {not_read_back:?}"));
            derivation.check(&Limits::default(), |_| false).unwrap();
            let (left, right) = derivation.sequent();
            assert_eq!(left, found.sequent.left(), "{text}");
            assert_eq!(right, found.sequent.right(), "{text}");
        }
    }

    /// A factor is needed where two literals of a clause must become one:
    /// `p(x) ∨ p(y)` and `¬p(u) ∨ ¬p(v)` have no refutation by binary
    /// resolution alone.
    #[test]
    fn factoring_refutes_what_binary_resolution_alone_cannot() {
        let found = found("forall x y. (p(x) \\/ p(y)), forall u v. (~p(u) \\/ ~p(v)) |-");
        let Answer::Refuted { refutation, .. } = &found.answer else {
            panic!("{:?}", found.answer);
        };
        assert!(
            refutation
                .inferences()
                .iter()
                .any(|i| matches!(i.step, Step::Factor { .. }))
        );
    }

    /// The unifier is shown with the parents' variables named apart, the
    /// given clause's `X1`, … and its partner's `Y1`, …; a clause with its
    /// own.
    #[test]
    fn steps_show_their_unifiers() {
        let found = found("forall x. (p(x) -> q(f(x))), p(a) |- exists y. q(y)");
        let Answer::Refuted { refutation, .. } = &found.answer else {
            panic!("{:?}", found.answer);
        };
        let set = refutation.clauses();
        let texts: Vec<String> = (0..set.clauses().len()).map(|k| set.text(k)).collect();
        assert_eq!(
            texts,
            ["¬p(X1) ∨ q(f(X1))", "p(a)", "¬q(X1)", "¬p(X1)", "□"]
        );
        let unifiers: Vec<Vec<String>> = refutation
            .inferences()
            .iter()
            .map(|i| set.bindings_text(i))
            .collect();
        assert_eq!(
            unifiers,
            [vec!["X1 := f(Y1)".to_owned()], vec!["X1 := a".to_owned()]]
        );
    }

    /// The occurs check refuses `p(x)` against `¬p(f(x))`-like pairs, so a
    /// sequent valid only without it is not refuted.
    #[test]
    fn the_occurs_check_holds() {
        let found = found("forall x. p(x, f(x)) |- exists y. p(y, y)");
        assert!(
            !matches!(found.answer, Answer::Refuted { .. }),
            "{:?}",
            found.answer
        );
    }

    /// With `=`, paramodulation refutes in steps the checker computes again,
    /// and its small set saturates where the axioms' would not; the axioms
    /// refute by resolution alone and read back. A paramodulation into a
    /// variable, or with a literal that is no equation, does not apply.
    #[test]
    fn equality_by_paramodulation_or_axioms() {
        for text in [
            "a = b, p(a) |- p(b)",
            "f(a) = b, p(f(a)) |- p(b)",
            "forall x. f(x) = x |- f(f(c)) = c",
            "a = b |- f(b) = f(a)",
        ] {
            let found = found(text);
            let Answer::Refuted { refutation, .. } = &found.answer else {
                panic!("{text}: {:?}", found.answer);
            };
            assert!(
                refutation
                    .inferences()
                    .iter()
                    .any(|i| matches!(i.step, Step::Paramodulation { .. })),
                "{text}"
            );
            refutation.check(|_| false).unwrap();
        }
        assert!(matches!(found("a = b |- a = c").answer, Answer::Saturated));
        let sequent: Sequent = "a = b, p(a) |- p(b)".parse().unwrap();
        let options = Options::default().with_equality(super::super::Equality::Axioms);
        let axioms = resolve(&sequent, &options, &Limits::default(), |_| false).unwrap();
        assert!(matches!(
            axioms.answer,
            Answer::Refuted {
                derivation: Some(_),
                ..
            }
        ));
        // `p(X)` has no place to rewrite but its variable, and `p(a)` is no
        // equation.
        let (_, mut clauses) = clauses_of(
            &"a = b, forall x. p(x) |- q".parse().unwrap(),
            &Limits::default(),
            |_| false,
        )
        .unwrap();
        let text: Vec<String> = (0..clauses.clauses().len())
            .map(|k| clauses.text(k))
            .collect();
        let at = |c: &str| text.iter().position(|t| t == c).unwrap();
        let (eq, p) = (at("a = b"), at("p(X1)"));
        for (from, into) in [(eq, p), (p, eq)] {
            let step = Step::Paramodulation {
                from,
                from_literal: 0,
                into,
                into_literal: 0,
                position: 0,
                reverse: false,
            };
            assert!(clauses.infer(step).is_err(), "{from} into {into}");
        }
    }

    /// Paramodulation's walk over a wide literal asks the stop as it goes,
    /// telling it the places walked, and counts no place as a step.
    #[test]
    fn the_walk_over_a_wide_literal_asks_the_stop() {
        let args = vec!["a"; 3 * PLACES_PER_POLL as usize].join(", ");
        let sequent: Sequent = format!("c = d, p({args}) |-").parse().unwrap();
        let options = Options::default().with_steps(None);
        let found = resolve(&sequent, &options, &Limits::default(), |progress| {
            progress.work == PLACES_PER_POLL
        })
        .unwrap();
        assert!(
            matches!(found.answer, Answer::Unknown(Bound::Stopped)),
            "{:?}",
            found.answer
        );
        let saturated = resolve(&sequent, &options, &Limits::default(), |_| false).unwrap();
        assert!(matches!(saturated.answer, Answer::Saturated));
        assert!(
            saturated.statistics.steps < 16,
            "{:?}",
            saturated.statistics
        );
    }

    /// A sequent that is not valid saturates where its clauses are finite,
    /// and ends at a bound where they grow.
    #[test]
    fn a_sequent_not_valid_saturates_or_ends_at_a_bound() {
        assert!(matches!(
            found("exists x. p(x) |- forall y. p(y)").answer,
            Answer::Saturated
        ));
        let sequent: Sequent = "forall x. (p(x) -> p(f(x))), p(a) |- q".parse().unwrap();
        let options = Options::default().with_steps(Some(200));
        let found = resolve(&sequent, &options, &Limits::default(), |_| false).unwrap();
        assert!(
            matches!(found.answer, Answer::Unknown(Bound::Steps { limit: 200 })),
            "{:?}",
            found.answer
        );
    }

    /// The checker refuses a refutation whose unifier or clause was changed
    /// after it was made.
    #[test]
    fn the_checker_refuses_a_changed_refutation() {
        let found = found("forall x. (p(x) -> q(f(x))), p(a) |- exists y. q(y)");
        let Answer::Refuted { refutation, .. } = &found.answer else {
            panic!("{:?}", found.answer);
        };
        let mut changed = (**refutation).clone();
        changed.inferences[0].bindings.clear();
        assert!(matches!(
            changed.check(|_| false),
            Err(Error::ClaimFails { .. })
        ));
        let mut changed = (**refutation).clone();
        let last = changed.clauses.items.len() - 1;
        changed.clauses.items[last] = changed.clauses.items[0].clone();
        assert!(matches!(
            changed.check(|_| false),
            Err(Error::ClaimFails { .. })
        ));
    }

    /// The bounded check keeps to the work and memory bounds its caller
    /// gives, and the short form checks as before.
    #[test]
    fn the_check_keeps_to_its_bounds() {
        let found = found("forall x. (p(x) -> q(f(x))), p(a) |- exists y. q(y)");
        let Answer::Refuted { refutation, .. } = &found.answer else {
            panic!("{:?}", found.answer);
        };
        refutation.check(|_| false).unwrap();
        refutation
            .check_within(&Limits::default(), |_| false)
            .unwrap();
        assert!(matches!(
            refutation.check_within(&Limits::default().with_work(Some(1)), |_| false),
            Err(Error::Refused(Refusal::Work { limit: 1 }))
        ));
        assert!(matches!(
            refutation.check_within(&Limits::default().with_memory_bytes(Some(64)), |_| false),
            Err(Error::Refused(Refusal::Memory { .. }))
        ));
    }

    /// A given `□` is a refutation of no step, read back too.
    #[test]
    fn a_given_empty_clause_is_a_refutation_of_no_step() {
        for text in [
            "false |- p(a)",
            "|- exists x. true",
            "forall x. false |- q(a)",
        ] {
            let found = found(text);
            let Answer::Refuted {
                refutation,
                derivation,
                ..
            } = &found.answer
            else {
                panic!("{text}: {:?}", found.answer);
            };
            assert!(refutation.inferences().is_empty(), "{text}");
            refutation.check(|_| false).unwrap();
            assert!(derivation.is_some(), "{text}");
        }
    }

    /// A unifier that doubles a term per variable makes a term that unfolds
    /// past `limits.terms` within one step: refused before its atom is
    /// made, at once.
    #[test]
    fn a_term_that_unfolds_past_the_bound_is_refused() {
        let n = 40;
        let xs: Vec<String> = (1..=n).map(|i| format!("x{i}")).collect();
        let mut doubled: Vec<String> = (2..=n).map(|i| format!("f(x{i}, x{i})")).collect();
        doubled.push("a".to_owned());
        let text = format!(
            "forall {}. (q({}) \\/ q({})) |-",
            xs.join(" "),
            xs.join(", "),
            doubled.join(", ")
        );
        let sequent: Sequent = text.parse().unwrap();
        let found = resolve(&sequent, &Options::default(), &Limits::default(), |_| false);
        assert!(
            matches!(found, Err(Error::Refused(Refusal::Terms { .. }))),
            "{found:?}"
        );
    }

    /// The read-back asks the stop while a branch grows, so a stop ends it
    /// and the refutation stands for the verdict.
    #[test]
    fn the_read_back_asks_the_stop() {
        let sequent: Sequent =
            "forall x0 x1 x2 x3 x4 x5. r(x0, x1, x2, x3, x4, x5) |- r(c0, c1, c2, c3, c4, c5)"
                .parse()
                .unwrap();
        // Its six instances take six steps before an inference closes.
        let found = resolve(&sequent, &Options::default(), &Limits::default(), |p| {
            p.phase == Phase::ReadBack && p.done > 2
        })
        .unwrap();
        let Answer::Refuted {
            derivation,
            not_read_back,
            ..
        } = &found.answer
        else {
            panic!("{:?}", found.answer);
        };
        assert!(derivation.is_none());
        assert!(not_read_back.is_some());
    }

    /// Returns a sequent whose one resolution step binds `y2 := f(y3, y3)`,
    /// …, `yn := a`, so that `y2` unfolds to `2^(n-1) - 1` symbols.
    fn doubling(n: usize) -> Sequent {
        let xs: Vec<String> = (1..=n).map(|i| format!("x{i}")).collect();
        let ys: Vec<String> = (2..=n).map(|i| format!("y{i}")).collect();
        let mut first = xs.clone();
        first.extend(xs[1..].iter().cloned());
        let mut second: Vec<String> = ys.iter().map(|y| format!("f({y}, {y})")).collect();
        second.push("a".to_owned());
        second.extend(ys.iter().cloned());
        format!(
            "forall {}. ~p({}), forall {}. p({}) |-",
            xs.join(" "),
            first.join(", "),
            ys.join(" "),
            second.join(", ")
        )
        .parse()
        .unwrap()
    }

    /// The read-back holds every instance to the bound on terms before its
    /// key is written, and to the memory bound; a binding's text is cut.
    /// Without them a unifier that doubles a term per variable made the
    /// read-back write keys of millions of symbols past every bound.
    #[test]
    fn the_read_back_keeps_to_the_bounds_on_shared_terms() {
        let sequent = doubling(14);
        let terms = Limits::default().with_terms(Some(1_000));
        let found = resolve(&sequent, &Options::default(), &terms, |_| false).unwrap();
        let Answer::Refuted {
            refutation,
            derivation,
            not_read_back,
        } = &found.answer
        else {
            panic!("{:?}", found.answer);
        };
        assert!(derivation.is_none());
        assert!(
            not_read_back
                .as_deref()
                .is_some_and(|why| why.contains("1000")),
            "{not_read_back:?}"
        );
        let texts = refutation.clauses.bindings_text(&refutation.inferences[0]);
        let longest = texts.iter().map(String::len).max().unwrap();
        assert!(texts.iter().any(|t| t.ends_with('…')), "{texts:?}");
        assert!(longest <= BINDING_TEXT + 16, "{longest}");
        // The memory bound: an instance's key is charged before it is
        // written (8 192 symbols, held twice and copied: 32 KiB), so the
        // read-back refuses itself, before Herbrand's construction would;
        // the refutation's check, whose copy of the arena is some 17 KB,
        // fits.
        let memory = Limits::default().with_memory_bytes(Some(24 << 10));
        let found = resolve(&sequent, &Options::default(), &memory, |_| false).unwrap();
        let Answer::Refuted { not_read_back, .. } = &found.answer else {
            panic!("{:?}", found.answer);
        };
        assert!(
            not_read_back
                .as_deref()
                .is_some_and(|why| why.contains("the read-back would take")),
            "{not_read_back:?}"
        );
    }

    /// A propositional sequent keeps an arena without first-order tables
    /// in its derivation, and a sequent that holds an open witness of a
    /// session is refused.
    #[test]
    fn arenas_stay_as_their_readers_want_them() {
        let found = found("p, p -> q |- q");
        let Answer::Refuted { derivation, .. } = &found.answer else {
            panic!("{:?}", found.answer);
        };
        let derivation = derivation.as_ref().unwrap();
        assert!(!derivation.formulas().is_first_order());
        let sequent: Sequent = "p(a) |- p(a)".parse().unwrap();
        let mut formulas = sequent.formulas().clone();
        let Node::Atom(a) = formulas.node(sequent.left()[0]) else {
            panic!("an atom")
        };
        let (symbol, _) = formulas.first_order().atoms[a as usize];
        let open = formulas.interner().term(FoTerm::Meta(0));
        let list = formulas.interner().list(&[open]);
        let atom = formulas.predicate_atom(symbol, list).unwrap();
        let witnessed = Sequent::new(formulas, vec![atom], sequent.right().to_vec()).unwrap();
        let refused = resolve(&witnessed, &Options::default(), &Limits::default(), |_| {
            false
        });
        assert!(
            matches!(refused, Err(Error::InvalidOption { key: "sequent", .. })),
            "{refused:?}"
        );
    }

    /// Generated first-order sequents: a refutation is checked and never
    /// of a sequent the default prover refutes, and most read back.
    #[test]
    fn generated_sequents_agree_with_the_default_prover() {
        use crate::linear::search::generate::Rng;
        use crate::ordinary::normal::first_order_tests::formula;
        let mut rng = Rng::new(78);
        let limits = Limits::default().with_work(Some(500_000));
        let options = Options::default().with_steps(Some(5_000));
        let (mut sequents, mut refuted, mut read) = (0, 0, 0);
        while sequents < SEQUENTS {
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
            let found = match resolve(&sequent, &options, &limits, |_| false) {
                Ok(found) => found,
                Err(Error::Refused(_)) => continue,
                Err(error) => panic!("{sequent}: {error}"),
            };
            if let Answer::Refuted { derivation, .. } = &found.answer {
                refuted += 1;
                let classical = crate::ordinary::decide_within(
                    &sequent,
                    &crate::ordinary::Options::default()
                        .with_logic(crate::ordinary::Logic::Classical),
                    &crate::linear::search::Options::default(),
                    &limits,
                    |_| false,
                )
                .map(|o| o.verdict);
                assert!(
                    !matches!(classical, Ok(crate::ordinary::Verdict::NotValid)),
                    "{sequent}"
                );
                read += usize::from(derivation.is_some());
            }
        }
        assert!(refuted > SEQUENTS / 10, "{refuted} refuted");
        assert!(read * 4 >= refuted * 3, "{read} of {refuted} read back");
    }

    /// How many sequents the generated test decides.
    const SEQUENTS: usize = 300;
}
