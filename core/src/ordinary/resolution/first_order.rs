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

use super::super::derivation::ArenaSize;
use super::super::normal::{self, NormalForm};
use super::super::{Derivation, Formulas, Node, NodeId, Sequent};
use super::{Bound, Options};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::unify::Unifier;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
    /// why; [`Refusal::Index`] when the arena is full.
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

    /// Returns what `step` makes, or why it does not apply.
    fn made(&mut self, step: Step) -> Result<Result<Made, &'static str>, Error> {
        let out_of_range = || Error::InvalidOption {
            key: "step",
            message: "a clause or literal outside the clauses".to_owned(),
        };
        let (first, second, i, j) = match step {
            Step::Resolution {
                first,
                second,
                first_literal,
                second_literal,
            } => (first, Some(second), first_literal, second_literal),
            Step::Factor {
                clause,
                first_literal,
                second_literal,
            } => (clause, None, first_literal, second_literal),
            Step::Paramodulation {
                from,
                from_literal,
                into,
                into_literal,
                position,
                reverse,
            } => {
                return self.paramodulant(
                    (from, from_literal),
                    (into, into_literal),
                    position,
                    reverse,
                );
            }
        };
        let a = self.items.get(first).ok_or_else(out_of_range)?.clone();
        let b = match second {
            Some(k) => Some(self.items.get(k).ok_or_else(out_of_range)?.clone()),
            None => None,
        };
        let x = *a.literals.get(i).ok_or_else(out_of_range)?;
        let (y, shift) = match &b {
            Some(b) => (*b.literals.get(j).ok_or_else(out_of_range)?, a.variables),
            None => (*a.literals.get(j).ok_or_else(out_of_range)?, 0),
        };
        if b.is_none() && i == j {
            return Ok(Err("a factor takes two different literals"));
        }
        match (b.is_some(), x.positive == y.positive) {
            (true, true) => return Ok(Err("the two literals have the same sign")),
            (false, false) => return Ok(Err("the two literals have opposite signs")),
            _ => {}
        }
        // The second parent's variables renamed apart, after the first's.
        let variables = a.variables + b.as_ref().map_or(0, |b| b.variables);
        let renamed_b: Vec<Literal> = match &b {
            Some(b) => {
                let mut out = Vec::with_capacity(b.literals.len());
                for l in &b.literals {
                    out.push(Literal {
                        positive: l.positive,
                        atom: self.shifted(l.atom, shift)?,
                    });
                }
                out
            }
            None => Vec::new(),
        };
        let y_atom = if b.is_some() {
            renamed_b[j].atom
        } else {
            y.atom
        };
        let Some(pairs) = self.pairs(x.atom, y_atom) else {
            return Ok(Err("the two literals are of different predicates"));
        };
        let mut unifier = Unifier::new();
        for k in 0..variables {
            let t = self.formulas.interner().term(FoTerm::Meta(k));
            unifier.push_meta(t, u32::MAX);
        }
        if unifier
            .unify(self.formulas.first_order(), pairs, 0, &|_| true)
            .is_err()
        {
            return Ok(Err(
                "the atoms do not unify: two symbols clash, or a variable would occur in its own binding",
            ));
        }
        // The rest of both parents, before the unifier; a factor keeps its
        // first literal where it stood.
        let kept: Vec<Literal> = match &b {
            Some(_) => a
                .literals
                .iter()
                .enumerate()
                .filter(|&(k, _)| k != i)
                .map(|(_, l)| *l)
                .chain(
                    renamed_b
                        .iter()
                        .enumerate()
                        .filter(|&(k, _)| k != j)
                        .map(|(_, l)| *l),
                )
                .collect(),
            None => a
                .literals
                .iter()
                .enumerate()
                .filter(|&(k, _)| k != j)
                .map(|(_, l)| *l)
                .collect(),
        };
        self.concluded(&unifier, variables, kept, a.variables)
            .map(Ok)
    }

    /// Returns what an inference makes of the literals `kept` (its parents'
    /// rest, renamed apart) under `unifier`, over the step's `variables`, the
    /// first parent's `first` of them: each literal once, the variables
    /// left numbered in the order they first occur.
    fn concluded(
        &mut self,
        unifier: &Unifier,
        variables: u32,
        kept: Vec<Literal>,
        first: u32,
    ) -> Result<Made, Error> {
        // Every variable of the step under the unifier.
        let mut memo: HashMap<FoTermId, FoTermId> = HashMap::default();
        let mut terms = Vec::with_capacity(variables as usize);
        for k in 0..variables {
            let t = unifier.metas()[k as usize].term;
            terms.push(self.applied(unifier, t, &mut memo));
        }
        let bindings: Vec<(u32, FoTermId)> = (0..variables)
            .filter(|&k| unifier.metas()[k as usize].bound.is_some())
            .map(|k| (k, terms[k as usize]))
            .collect();
        let mut rest: Vec<Literal> = Vec::new();
        for l in kept {
            let atom = self.applied_atom(unifier, l.atom, &mut memo)?;
            let literal = Literal {
                positive: l.positive,
                atom,
            };
            if !rest.contains(&literal) {
                rest.push(literal);
            }
        }
        // The variables left, numbered in the order they first occur.
        let mut renamed: Vec<Option<u32>> = vec![None; variables as usize];
        let mut order: Vec<u32> = Vec::new();
        for l in &rest {
            for m in self.metas_of(l.atom) {
                if renamed[m as usize].is_none() {
                    renamed[m as usize] = Some(order.len() as u32);
                    order.push(m);
                }
            }
        }
        let mut literals = Vec::with_capacity(rest.len());
        for l in &rest {
            let atom = self.renumbered(l.atom, &renamed)?;
            literals.push(Literal {
                positive: l.positive,
                atom,
            });
        }
        Ok(Made {
            bindings,
            clause: Clause {
                literals,
                variables: order.len() as u32,
            },
            terms,
            renamed,
            first,
        })
    }

    /// Returns what the paramodulation of the literal `into` by the
    /// equation `from`, each a clause and a literal's place, makes at the
    /// subterm `position`, the equation read right to left where `reverse`
    /// is set; or why it does not apply.
    fn paramodulant(
        &mut self,
        (from, from_literal): (usize, usize),
        (into, into_literal): (usize, usize),
        position: usize,
        reverse: bool,
    ) -> Result<Result<Made, &'static str>, Error> {
        let out_of_range = || Error::InvalidOption {
            key: "step",
            message: "a clause or literal outside the clauses".to_owned(),
        };
        let a = self.items.get(from).ok_or_else(out_of_range)?.clone();
        let b = self.items.get(into).ok_or_else(out_of_range)?.clone();
        let x = *a.literals.get(from_literal).ok_or_else(out_of_range)?;
        b.literals.get(into_literal).ok_or_else(out_of_range)?;
        let Some((s, t)) = x.positive.then(|| self.equation(x.atom)).flatten() else {
            return Ok(Err("the first literal is no equation s = t"));
        };
        let (s, t) = if reverse { (t, s) } else { (s, t) };
        // The second clause's variables renamed apart, after the first's.
        let shift = a.variables;
        let variables = a.variables + b.variables;
        let mut renamed_b = Vec::with_capacity(b.literals.len());
        for l in &b.literals {
            renamed_b.push(Literal {
                positive: l.positive,
                atom: self.shifted(l.atom, shift)?,
            });
        }
        let target = renamed_b[into_literal].atom;
        // A literal has fewer places than `limits.terms`, its atom's bound,
        // so the walk to a place past it would find none, and is not made.
        if self.most_terms.is_some_and(|most| position as u64 >= most) {
            return Ok(Err("the literal has no subterm at that place"));
        }
        let places = self.places(target, position.saturating_add(1));
        let Some(&(u, _, _)) = places.get(position) else {
            return Ok(Err("the literal has no subterm at that place"));
        };
        // The path from the argument down, read off the parents.
        let mut path = Vec::new();
        let mut at = position;
        loop {
            let (_, parent, index) = places[at];
            path.push(index);
            if parent == usize::MAX {
                break;
            }
            at = parent;
        }
        path.reverse();
        if matches!(
            self.formulas.first_order().terms[u.index()],
            FoTerm::Meta(_)
        ) {
            return Ok(Err(
                "the subterm is a variable, which paramodulation does not rewrite",
            ));
        }
        let mut unifier = Unifier::new();
        for k in 0..variables {
            let m = self.formulas.interner().term(FoTerm::Meta(k));
            unifier.push_meta(m, u32::MAX);
        }
        if unifier
            .unify(self.formulas.first_order(), vec![(s, u)], 0, &|_| true)
            .is_err()
        {
            return Ok(Err(
                "the equation's side and the subterm do not unify: two symbols clash, or a variable would occur in its own binding",
            ));
        }
        renamed_b[into_literal].atom = self.replaced(target, &path, t)?;
        // The equation's clause's rest, then the rewritten clause.
        let kept: Vec<Literal> = a
            .literals
            .iter()
            .enumerate()
            .filter(|&(k, _)| k != from_literal)
            .map(|(_, l)| *l)
            .chain(renamed_b)
            .collect();
        self.concluded(&unifier, variables, kept, a.variables)
            .map(Ok)
    }

    /// Adds to `steps` every paramodulation by a positive equation of the
    /// clause `from` into a literal of the clause `into`, at each place
    /// that is no variable, each way round, but where the side and the
    /// subterm begin with two symbols or eigenvariables (no unifier), and by
    /// no equation of two equal sides (it rewrites nothing). Asks `stop`
    /// every [`PLACES_PER_POLL`] places walked, telling it the places, and
    /// returns `false` where it answered true.
    fn paramodulations(
        &self,
        from: usize,
        into: usize,
        steps: &mut Vec<Step>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> bool {
        let tables = self.formulas.first_order();
        // What a term begins with: a symbol or an eigenvariable, either a
        // constant of the clauses; nothing for a variable.
        let head = |t: FoTermId| match tables.terms[t.index()] {
            FoTerm::App(f, _) => Some((false, f.get())),
            FoTerm::Eigen(e) => Some((true, e.get())),
            FoTerm::Meta(_) | FoTerm::Bound(_) => None,
        };
        let mut walked = 0u64;
        for (i, x) in self.items[from].literals.iter().enumerate() {
            let Some((s, t)) = x.positive.then(|| self.equation(x.atom)).flatten() else {
                continue;
            };
            if s == t {
                continue;
            }
            for (j, y) in self.items[into].literals.iter().enumerate() {
                let places = self.places(y.atom, usize::MAX);
                for (position, &(u, _, _)) in places.iter().enumerate() {
                    walked += 1;
                    if walked.is_multiple_of(PLACES_PER_POLL)
                        && stop(Progress::new(Phase::Search, PLACES_PER_POLL, walked))
                    {
                        return false;
                    }
                    let Some(g) = head(u) else {
                        continue;
                    };
                    for (side, reverse) in [(s, false), (t, true)] {
                        if head(side).is_none_or(|f| f == g) {
                            steps.push(Step::Paramodulation {
                                from,
                                from_literal: i,
                                into,
                                into_literal: j,
                                position,
                                reverse,
                            });
                        }
                    }
                }
            }
        }
        true
    }

    /// Returns the two sides of `atom` where it is an equation `s = t`.
    fn equation(&self, atom: NodeId) -> Option<(FoTermId, FoTermId)> {
        let Node::Atom(a) = self.formulas.node(atom) else {
            return None;
        };
        let tables = self.formulas.first_order();
        let &(p, args) = tables.atoms.get(a as usize)?;
        match *tables.list(args) {
            [s, t] if tables.symbols[p.index()].0 == crate::first_order::EQUALS => Some((s, t)),
            _ => None,
        }
    }

    /// Returns the first `most` places of `atom`'s arguments as they stand
    /// in the text, left to right and each before its arguments: each its
    /// subterm, the index of its parent's place (`usize::MAX` for an
    /// argument) and its place among the parent's arguments. A shared part
    /// counts once per place it stands at; the walk stops at `most`, so a
    /// term that shares its parts costs no more than that.
    fn places(&self, atom: NodeId, most: usize) -> Vec<(FoTermId, usize, u32)> {
        let Node::Atom(a) = self.formulas.node(atom) else {
            return Vec::new();
        };
        let tables = self.formulas.first_order();
        let Some(&(_, args)) = tables.atoms.get(a as usize) else {
            return Vec::new();
        };
        let mut found = Vec::new();
        let mut stack: Vec<(FoTermId, usize, u32)> = tables
            .list(args)
            .iter()
            .enumerate()
            .rev()
            .map(|(i, &t)| (t, usize::MAX, i as u32))
            .collect();
        while let Some(place) = stack.pop() {
            if found.len() >= most {
                break;
            }
            let here = found.len();
            if let FoTerm::App(_, args) = tables.terms[place.0.index()] {
                for (i, &u) in tables.list(args).iter().enumerate().rev() {
                    stack.push((u, here, i as u32));
                }
            }
            found.push(place);
        }
        found
    }

    /// Returns `atom` with the subterm at `path` (the argument's place,
    /// then the place below at each level) put as `with`.
    fn replaced(&mut self, atom: NodeId, path: &[u32], with: FoTermId) -> Result<NodeId, Error> {
        let Node::Atom(a) = self.formulas.node(atom) else {
            return Ok(atom);
        };
        let Some(&(p, args)) = self.formulas.first_order().atoms.get(a as usize) else {
            return Ok(atom);
        };
        let mut items = self.formulas.first_order().list(args).to_vec();
        // The terms along the path, the argument first.
        let mut chain = Vec::with_capacity(path.len());
        let mut t = items[path[0] as usize];
        chain.push(t);
        for &i in &path[1..] {
            let FoTerm::App(_, below) = self.formulas.first_order().terms[t.index()] else {
                break;
            };
            t = self.formulas.first_order().list(below)[i as usize];
            chain.push(t);
        }
        // Built again from the bottom: each parent with its child put.
        let mut built = with;
        for level in (1..chain.len()).rev() {
            let FoTerm::App(f, below) = self.formulas.first_order().terms[chain[level - 1].index()]
            else {
                continue;
            };
            let mut list = self.formulas.first_order().list(below).to_vec();
            list[path[level] as usize] = built;
            let list = self.formulas.interner().list(&list);
            built = self.formulas.interner().term(FoTerm::App(f, list));
        }
        items[path[0] as usize] = built;
        let list = self.formulas.interner().list(&items);
        self.formulas.predicate_atom(p, list)
    }

    /// Returns the pairs of arguments of two atoms of one predicate, or
    /// `None` for atoms of two predicates.
    fn pairs(&self, x: NodeId, y: NodeId) -> Option<Vec<(FoTermId, FoTermId)>> {
        let tables = self.formulas.first_order();
        let (Node::Atom(a), Node::Atom(b)) = (self.formulas.node(x), self.formulas.node(y)) else {
            return None;
        };
        match (tables.atoms.get(a as usize), tables.atoms.get(b as usize)) {
            (Some(&(p, s)), Some(&(q, t))) => {
                let (s, t) = (tables.list(s), tables.list(t));
                (p == q && s.len() == t.len())
                    .then(|| s.iter().copied().zip(t.iter().copied()).collect())
            }
            (None, None) => (a == b).then(Vec::new),
            _ => None,
        }
    }

    /// Returns the atom `atom` with each metavariable `k` the term `f(k)`.
    fn mapped_atom(
        &mut self,
        atom: NodeId,
        f: &mut dyn FnMut(&mut Formulas, u32) -> FoTermId,
    ) -> Result<NodeId, Error> {
        self.mapped_atom_charged(atom, f, &mut |_| Ok(()))
    }

    /// [`mapped_atom`](Self::mapped_atom), handing `charge` the symbols the
    /// atom unfolds to before its key is written, so that a caller can
    /// refuse the key's memory before it is held.
    fn mapped_atom_charged(
        &mut self,
        atom: NodeId,
        f: &mut dyn FnMut(&mut Formulas, u32) -> FoTermId,
        charge: &mut dyn FnMut(u64) -> Result<(), Error>,
    ) -> Result<NodeId, Error> {
        let Node::Atom(a) = self.formulas.node(atom) else {
            return Ok(atom);
        };
        let Some(&(p, args)) = self.formulas.first_order().atoms.get(a as usize) else {
            return Ok(atom);
        };
        let args = self.formulas.first_order().list(args).to_vec();
        let mut memo: HashMap<FoTermId, FoTermId> = HashMap::default();
        let mut mapped = Vec::with_capacity(args.len());
        for t in args {
            mapped.push(map_term(&mut self.formulas, t, f, &mut memo));
        }
        let size = self.within_terms(&mapped)?;
        charge(size)?;
        let list = self.formulas.interner().list(&mapped);
        self.formulas.predicate_atom(p, list)
    }

    /// Returns the symbols an atom of these arguments unfolds to, counted
    /// over shared parts once each, refusing past the bound on terms: the
    /// atom's key is its text, which unfolds every term, and a term that
    /// shares its parts may unfold past any bound.
    fn within_terms(&mut self, items: &[FoTermId]) -> Result<u64, Error> {
        let mut size = 1u64;
        for &t in items {
            size = size.saturating_add(self.size(t));
        }
        match self.most_terms {
            Some(limit) if size > limit => {
                Err(Error::Refused(Refusal::Terms { terms: size, limit }))
            }
            _ => Ok(size),
        }
    }

    /// Returns `atom` with each metavariable `k` moved to `k + by`.
    fn shifted(&mut self, atom: NodeId, by: u32) -> Result<NodeId, Error> {
        if by == 0 {
            return Ok(atom);
        }
        self.mapped_atom(atom, &mut |formulas, k| {
            formulas.interner().term(FoTerm::Meta(k + by))
        })
    }

    /// Returns `atom` with each metavariable `k` renumbered `renamed[k]`.
    fn renumbered(&mut self, atom: NodeId, renamed: &[Option<u32>]) -> Result<NodeId, Error> {
        self.mapped_atom(atom, &mut |formulas, k| {
            let to = renamed.get(k as usize).copied().flatten().unwrap_or(k);
            formulas.interner().term(FoTerm::Meta(to))
        })
    }

    /// Returns the term `t` with every bound metavariable replaced by its
    /// binding, all the way down.
    fn applied(
        &mut self,
        unifier: &Unifier,
        t: FoTermId,
        memo: &mut HashMap<FoTermId, FoTermId>,
    ) -> FoTermId {
        let mut stack = vec![(t, false)];
        while let Some((u, ready)) = stack.pop() {
            if memo.contains_key(&u) {
                continue;
            }
            match self.formulas.first_order().terms[u.index()] {
                FoTerm::Meta(k) => match unifier.metas().get(k as usize).and_then(|m| m.bound) {
                    Some(b) if ready => {
                        let image = memo[&b];
                        memo.insert(u, image);
                    }
                    Some(b) => {
                        stack.push((u, true));
                        stack.push((b, false));
                    }
                    None => {
                        memo.insert(u, u);
                    }
                },
                FoTerm::App(f, args) => {
                    let list = self.formulas.first_order().list(args).to_vec();
                    if !ready {
                        stack.push((u, true));
                        stack.extend(list.into_iter().map(|v| (v, false)));
                        continue;
                    }
                    let items: Vec<FoTermId> = list.iter().map(|v| memo[v]).collect();
                    let args = self.formulas.interner().list(&items);
                    let image = self.formulas.interner().term(FoTerm::App(f, args));
                    memo.insert(u, image);
                }
                FoTerm::Bound(_) | FoTerm::Eigen(_) => {
                    memo.insert(u, u);
                }
            }
        }
        memo[&t]
    }

    /// Returns `atom` under the unifier, as [`applied`](Self::applied).
    fn applied_atom(
        &mut self,
        unifier: &Unifier,
        atom: NodeId,
        memo: &mut HashMap<FoTermId, FoTermId>,
    ) -> Result<NodeId, Error> {
        let Node::Atom(a) = self.formulas.node(atom) else {
            return Ok(atom);
        };
        let Some(&(p, args)) = self.formulas.first_order().atoms.get(a as usize) else {
            return Ok(atom);
        };
        let args = self.formulas.first_order().list(args).to_vec();
        let mut items = Vec::with_capacity(args.len());
        for t in args {
            items.push(self.applied(unifier, t, memo));
        }
        self.within_terms(&items)?;
        let list = self.formulas.interner().list(&items);
        self.formulas.predicate_atom(p, list)
    }

    /// Returns the symbols and variables the term `t` unfolds to, counted
    /// over its shared parts once each.
    fn size(&mut self, t: FoTermId) -> u64 {
        let mut stack = vec![(t, false)];
        while let Some((u, ready)) = stack.pop() {
            if self.sizes.contains_key(&u) {
                continue;
            }
            let tables = self.formulas.first_order();
            let FoTerm::App(_, args) = tables.terms[u.index()] else {
                self.sizes.insert(u, 1);
                continue;
            };
            let list = tables.list(args);
            if !ready {
                stack.push((u, true));
                stack.extend(list.iter().map(|&v| (v, false)));
                continue;
            }
            let total = list
                .iter()
                .fold(1u64, |sum, v| sum.saturating_add(self.sizes[v]));
            self.sizes.insert(u, total);
        }
        self.sizes[&t]
    }

    /// Returns the metavariables of `atom`, in the order they first occur,
    /// left to right.
    fn metas_of(&self, atom: NodeId) -> Vec<u32> {
        let tables = self.formulas.first_order();
        let Node::Atom(a) = self.formulas.node(atom) else {
            return Vec::new();
        };
        let Some(&(_, args)) = tables.atoms.get(a as usize) else {
            return Vec::new();
        };
        let mut found = Vec::new();
        let mut metas: HashSet<u32> = HashSet::default();
        let mut seen: HashSet<FoTermId> = HashSet::default();
        let mut stack: Vec<FoTermId> = tables.list(args).iter().rev().copied().collect();
        while let Some(t) = stack.pop() {
            // A shared part is walked once: its variables occur first where
            // it first does.
            if !seen.insert(t) {
                continue;
            }
            match tables.terms[t.index()] {
                // A set, since a clause may have as many variables as a
                // symbol's arity.
                FoTerm::Meta(k) => {
                    if metas.insert(k) {
                        found.push(k);
                    }
                }
                FoTerm::App(_, args) => stack.extend(tables.list(args).iter().rev().copied()),
                FoTerm::Bound(_) | FoTerm::Eigen(_) => {}
            }
        }
        found
    }

    /// Returns whether `clause` is a tautology: a literal and its negation.
    fn is_tautology(clause: &Clause) -> bool {
        clause.literals.iter().any(|l| {
            clause
                .literals
                .iter()
                .any(|m| m.atom == l.atom && m.positive != l.positive)
        })
    }

    /// Returns whether `general` subsumes `special`: a substitution of its
    /// variables makes each of its literals one of `special`'s. A search
    /// that passes `limit` matchings answers no.
    fn subsumes(&self, general: &Clause, special: &Clause, limit: u64) -> bool {
        if general.literals.len() > special.literals.len() {
            return false;
        }
        let tables = self.formulas.first_order();
        // Matches the literal at `k` of `general` onto some of `special`'s
        // under `values`, depth first.
        let mut budget = limit;
        let mut stack: Vec<(usize, Vec<Option<FoTermId>>)> =
            vec![(0, vec![None; general.variables as usize])];
        while let Some((k, values)) = stack.pop() {
            let Some(&l) = general.literals.get(k) else {
                return true;
            };
            for m in &special.literals {
                if budget == 0 {
                    return false;
                }
                budget -= 1;
                if m.positive != l.positive {
                    continue;
                }
                let mut values = values.clone();
                if match_atom(&self.formulas, tables, l.atom, m.atom, &mut values) {
                    stack.push((k + 1, values));
                }
            }
        }
        false
    }

    /// Returns the clause's weight: the symbols and variables of its atoms.
    fn weight(&mut self, clause: &Clause) -> u64 {
        let mut weight = 0u64;
        for l in &clause.literals {
            weight = weight.saturating_add(1);
            let Node::Atom(a) = self.formulas.node(l.atom) else {
                continue;
            };
            let Some(&(_, args)) = self.formulas.first_order().atoms.get(a as usize) else {
                continue;
            };
            let args = self.formulas.first_order().list(args).to_vec();
            for t in args {
                weight = weight.saturating_add(self.size(t));
            }
        }
        weight
    }
}

/// Returns the term `t` with each metavariable `k` the term `f(k)`.
fn map_term(
    formulas: &mut Formulas,
    t: FoTermId,
    f: &mut dyn FnMut(&mut Formulas, u32) -> FoTermId,
    memo: &mut HashMap<FoTermId, FoTermId>,
) -> FoTermId {
    let mut stack = vec![(t, false)];
    while let Some((u, ready)) = stack.pop() {
        if memo.contains_key(&u) {
            continue;
        }
        let image = match formulas.first_order().terms[u.index()] {
            FoTerm::Meta(k) => f(formulas, k),
            FoTerm::App(g, args) => {
                let list = formulas.first_order().list(args).to_vec();
                if !ready {
                    stack.push((u, true));
                    stack.extend(list.into_iter().map(|v| (v, false)));
                    continue;
                }
                let items: Vec<FoTermId> = list.iter().map(|v| memo[v]).collect();
                let args = formulas.interner().list(&items);
                formulas.interner().term(FoTerm::App(g, args))
            }
            FoTerm::Bound(_) | FoTerm::Eigen(_) => u,
        };
        memo.insert(u, image);
    }
    memo[&t]
}

/// Returns whether a substitution extending `values` makes the atom
/// `pattern` the atom `target`, extending it so.
fn match_atom(
    formulas: &Formulas,
    tables: &crate::first_order::FirstOrder<NodeId>,
    pattern: NodeId,
    target: NodeId,
    values: &mut [Option<FoTermId>],
) -> bool {
    let (Node::Atom(a), Node::Atom(b)) = (formulas.node(pattern), formulas.node(target)) else {
        return false;
    };
    let (Some(&(p, s)), Some(&(q, t))) =
        (tables.atoms.get(a as usize), tables.atoms.get(b as usize))
    else {
        return a == b;
    };
    if p != q {
        return false;
    }
    let (s, t) = (tables.list(s), tables.list(t));
    if s.len() != t.len() {
        return false;
    }
    let mut pairs: Vec<(FoTermId, FoTermId)> = s.iter().copied().zip(t.iter().copied()).collect();
    // A pair matched once matches again under the same values.
    let mut seen: HashSet<(FoTermId, FoTermId)> = HashSet::default();
    while let Some((u, v)) = pairs.pop() {
        if !seen.insert((u, v)) {
            continue;
        }
        match (tables.terms[u.index()], tables.terms[v.index()]) {
            (FoTerm::Meta(k), _) => match values.get_mut(k as usize) {
                Some(slot @ None) => *slot = Some(v),
                Some(Some(w)) if *w == v => {}
                _ => return false,
            },
            (FoTerm::App(f, x), FoTerm::App(g, y)) if f == g => {
                let (x, y) = (tables.list(x), tables.list(y));
                if x.len() != y.len() {
                    return false;
                }
                pairs.extend(x.iter().copied().zip(y.iter().copied()));
            }
            _ if u == v => {}
            _ => return false,
        }
    }
    true
}

/// Writes the term `t`, each metavariable `k` as `name(k)`.
fn write_term(formulas: &Formulas, t: FoTermId, out: &mut String, name: &dyn Fn(u32) -> String) {
    let written = write_term_to(formulas, t, out, name);
    debug_assert!(written.is_ok(), "a string takes every write");
}

/// Writes the term `t` into `out`, each metavariable `k` as `name(k)`,
/// ending at the first write `out` refuses.
fn write_term_to<W: std::fmt::Write>(
    formulas: &Formulas,
    t: FoTermId,
    out: &mut W,
    name: &dyn Fn(u32) -> String,
) -> std::fmt::Result {
    formulas.first_order().write_with(
        out,
        t,
        &mut |o: &mut W, symbol: &str| o.write_str(symbol),
        &mut |o: &mut W, index| write!(o, "#{index}"),
        &mut |o: &mut W, other| match other {
            FoTerm::Meta(k) => o.write_str(&name(k)),
            // A parameter of the arena, by the name a proof prints it
            // with.
            FoTerm::Eigen(e) => {
                let eigens = formulas.eigen_names();
                match eigens.binary_search_by_key(&e.get(), |&(k, _)| k) {
                    Ok(at) => o.write_str(&eigens[at].1),
                    Err(_) => o.write_char('?'),
                }
            }
            FoTerm::Bound(_) | FoTerm::App(..) => Ok(()),
        },
    )
}

/// The most bytes [`Clauses::bindings_text`] writes of a binding's term.
const BINDING_TEXT: usize = 1 << 12;

/// Text that takes at most `room` bytes and refuses the write that would
/// pass them, keeping what of it fits.
struct Capped {
    /// The text so far.
    text: String,
    /// The most bytes it takes.
    room: usize,
}

impl std::fmt::Write for Capped {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let left = self.room.saturating_sub(self.text.len());
        if s.len() <= left {
            self.text.push_str(s);
            return Ok(());
        }
        let mut end = left;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        self.text.push_str(&s[..end]);
        Err(std::fmt::Error)
    }
}

/// Returns `clause` as text, its variables `prefix` and their numbers.
fn clause_text(formulas: &Formulas, clause: &Clause, prefix: &str) -> String {
    if clause.literals.is_empty() {
        return "□".to_owned();
    }
    let name = |k: u32| format!("{prefix}{}", k + 1);
    let tables = formulas.first_order();
    let mut text = String::new();
    for (n, l) in clause.literals.iter().enumerate() {
        if n > 0 {
            text.push_str(" ∨ ");
        }
        if !l.positive {
            text.push('¬');
        }
        let Node::Atom(a) = formulas.node(l.atom) else {
            continue;
        };
        match tables.atoms.get(a as usize) {
            Some(&(p, args)) => {
                let name_of = &tables.symbols[p.index()].0;
                let args = tables.list(args);
                if args.len() == 2 && name_of == "=" {
                    write_term(formulas, args[0], &mut text, &name);
                    text.push_str(" = ");
                    write_term(formulas, args[1], &mut text, &name);
                    continue;
                }
                text.push_str(name_of);
                if !args.is_empty() {
                    text.push('(');
                    for (k, &t) in args.iter().enumerate() {
                        if k > 0 {
                            text.push_str(", ");
                        }
                        write_term(formulas, t, &mut text, &name);
                    }
                    text.push(')');
                }
            }
            None => text.push_str(formulas.atom_name(a)),
        }
    }
    text
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

    /// Checks the refutation step by step: from the given clauses alone,
    /// each inference's unifier is computed again and its clause made again,
    /// both equal to what it records, each parent made before it, and the
    /// last clause is `□`. An atom is held to the bound on terms the
    /// clauses were made under; there is no bound on work or memory, which
    /// [`check_within`](Self::check_within) takes.
    ///
    /// # Errors
    ///
    /// [`Error::ClaimFails`] for a step that fails, saying which;
    /// [`Refusal::Terms`] for an atom past the clauses' bound on terms;
    /// [`Refusal::Stopped`] when `stop` fires, asked once per step.
    pub fn check(&self, stop: impl FnMut(Progress) -> bool) -> Result<(), Error> {
        let limits = Limits::default()
            .with_terms(self.clauses.most_terms)
            .with_memory_bytes(None)
            .with_work(None);
        self.check_within(&limits, stop)
    }

    /// Checks the refutation as [`check`](Self::check) does, within
    /// `limits`: `limits.terms` bounds an atom, `limits.work` the steps
    /// and the literals they make, and `limits.memory_bytes` the copy of
    /// the clauses' arena the steps are made in again and what they add
    /// to it.
    ///
    /// # Errors
    ///
    /// [`Error::ClaimFails`] for a step that fails, saying which;
    /// [`Refusal::Terms`], [`Refusal::Work`] and [`Refusal::Memory`] past
    /// those bounds; [`Refusal::Stopped`] when `stop` fires, asked once
    /// per step.
    pub fn check_within(
        &self,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let fails = |reason: String| Error::ClaimFails {
            claim: "the first-order refutation".to_owned(),
            reason,
        };
        let memory = limits.memory_bytes.unwrap_or(u64::MAX);
        let most_work = limits.work.unwrap_or(u64::MAX);
        // The copy of the arena, charged before it is made.
        let mut held = ArenaSize::of(&self.clauses.formulas, &ArenaSize::default());
        let within_memory = |held: &ArenaSize| {
            if held.bytes > memory {
                return Err(Error::Refused(Refusal::Memory {
                    phase: Phase::Check,
                    limit_bytes: memory,
                    needed_bytes: Some(held.bytes),
                }));
            }
            Ok(())
        };
        within_memory(&held)?;
        let mut again = Clauses {
            formulas: self.clauses.formulas.clone(),
            items: self.clauses.items[..self.clauses.given].to_vec(),
            given: self.clauses.given,
            most_terms: limits.terms,
            sizes: HashMap::default(),
        };
        let mut work = 0u64;
        for (n, inference) in self.inferences.iter().enumerate() {
            if stop(Progress::new(Phase::Check, 1, n as u64 + 1)) {
                return Err(Error::Refused(Refusal::stopped(Phase::Check)));
            }
            let made = again
                .made(inference.step)
                .map_err(|e| {
                    if let Error::Refused(_) = e {
                        e
                    } else {
                        fails(format!("step {}: {e}", n + 1))
                    }
                })?
                .map_err(|why| fails(format!("step {}: {why}", n + 1)))?;
            // A step, and each literal it makes.
            work = work.saturating_add(1 + made.clause.literals.len() as u64);
            if work > most_work {
                return Err(Error::Refused(Refusal::Work { limit: most_work }));
            }
            held = ArenaSize::of(&again.formulas, &held);
            within_memory(&held)?;
            if made.bindings != inference.bindings {
                return Err(fails(format!(
                    "step {}: its unifier is not the most general one",
                    n + 1
                )));
            }
            let recorded = self
                .clauses
                .items
                .get(inference.clause)
                .ok_or_else(|| fails(format!("step {}: no clause made", n + 1)))?;
            if inference.clause != again.items.len() || made.clause != *recorded {
                return Err(fails(format!(
                    "step {}: its clause is not the one the step makes",
                    n + 1
                )));
            }
            again.items.push(made.clause);
        }
        // A given `□` is a refutation of no step.
        let ends = if self.inferences.is_empty() {
            again.items.iter().any(Clause::is_empty)
        } else {
            again.items.last().is_some_and(Clause::is_empty)
        };
        if !ends {
            return Err(fails("it does not end at □".to_owned()));
        }
        Ok(())
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
/// for a refutation that fails its check, a defect; [`Refusal::Memory`]
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
    let mut seen_nodes: HashSet<NodeId> = HashSet::default();
    let mut seen_terms: HashSet<FoTermId> = HashSet::default();
    let mut nodes: Vec<NodeId> = sequent
        .left()
        .iter()
        .chain(sequent.right())
        .copied()
        .collect();
    let mut terms: Vec<FoTermId> = Vec::new();
    while let Some(n) = nodes.pop() {
        if !seen_nodes.insert(n) {
            continue;
        }
        let node = formulas.node(n);
        if let Node::Atom(a) = node
            && let Some(&(_, args)) = tables.atoms.get(a as usize)
        {
            terms.extend_from_slice(tables.list(args));
        }
        let (a, b) = node.operands();
        nodes.extend([a, b].into_iter().flatten());
    }
    while let Some(t) = terms.pop() {
        if !seen_terms.insert(t) {
            continue;
        }
        match tables.terms[t.index()] {
            FoTerm::Meta(_) => {
                return Err(Error::InvalidOption {
                    key: "sequent",
                    message: "an open witness stands only in a proof in progress".to_owned(),
                });
            }
            FoTerm::App(_, args) => terms.extend_from_slice(tables.list(args)),
            FoTerm::Bound(_) | FoTerm::Eigen(_) => {}
        }
    }
    Ok(())
}

/// Returns the clauses of the clausal form, each hypothesis `∀x̄. C` its
/// literals with `x̄` the metavariables `0`, `1`, … outermost first, and
/// with `support`, which are the set of support: those that are no clause
/// of the hypotheses alone, where the sequent has formulas right of `⊢`;
/// all of them otherwise.
fn given_clauses(
    sequent: &Sequent,
    clausal: &NormalForm,
    support: bool,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<(Clauses, Vec<bool>), Error> {
    let mut formulas = clausal.result.formulas().clone();
    let mut clauses = Vec::new();
    for &root in clausal.result.left() {
        let mut matrix = root;
        let mut variables = 0u32;
        while let Node::Forall(body) = formulas.node(matrix) {
            variables += 1;
            matrix = body;
        }
        // The frame is the innermost binder's first.
        let frame: Vec<FoTermId> = (0..variables)
            .rev()
            .map(|k| formulas.interner().term(FoTerm::Meta(k)))
            .collect();
        let open = formulas.instantiate_frame(matrix, &frame)?;
        let mut literals = Vec::new();
        let mut tautology = false;
        let mut stack = vec![open];
        while let Some(n) = stack.pop() {
            match formulas.node(n) {
                Node::Or(a, b) => stack.extend([b, a]),
                Node::Not(a) => literals.push(Literal {
                    positive: false,
                    atom: a,
                }),
                Node::Atom(_) => literals.push(Literal {
                    positive: true,
                    atom: n,
                }),
                Node::False => {}
                Node::True => tautology = true,
                Node::And(..)
                | Node::Implies(..)
                | Node::Iff(..)
                | Node::Forall(_)
                | Node::Exists(_) => {
                    return Err(Error::ClaimFails {
                        claim: "the clausal form".to_owned(),
                        reason: "a hypothesis is no clause".to_owned(),
                    });
                }
            }
        }
        if tautology {
            continue;
        }
        let mut unique: Vec<Literal> = Vec::with_capacity(literals.len());
        for l in literals {
            if !unique.contains(&l) {
                unique.push(l);
            }
        }
        // The variables in the order they first occur, as every clause.
        let mut set = Clauses {
            formulas,
            items: Vec::new(),
            given: 0,
            most_terms: limits.terms,
            sizes: HashMap::default(),
        };
        let mut renamed: Vec<Option<u32>> = vec![None; variables as usize];
        let mut count = 0u32;
        for l in &unique {
            for m in set.metas_of(l.atom) {
                if renamed[m as usize].is_none() {
                    renamed[m as usize] = Some(count);
                    count += 1;
                }
            }
        }
        let mut literals = Vec::with_capacity(unique.len());
        for l in &unique {
            literals.push(Literal {
                positive: l.positive,
                atom: set.renumbered(l.atom, &renamed)?,
            });
        }
        formulas = set.formulas;
        clauses.push(Clause {
            literals,
            variables: count,
        });
    }
    let given = clauses.len();
    let set = Clauses {
        formulas,
        items: clauses,
        given,
        most_terms: limits.terms,
        sizes: HashMap::default(),
    };
    // The hypotheses' own clauses, by text: what is left is the goals'.
    let support = if !support || sequent.right().is_empty() {
        vec![true; given]
    } else {
        let hypotheses = Sequent::new(
            sequent.formulas().clone(),
            sequent.left().to_vec(),
            Vec::new(),
        )?;
        let own = normal::rewrite(
            &hypotheses,
            &normal::Options::default().with_form(normal::Form::Clausal),
            limits,
            &mut *stop,
        )?;
        let texts: HashSet<String> = own
            .result
            .left()
            .iter()
            .map(|&k| own.result.formulas().formula(k).to_string())
            .collect();
        clausal
            .result
            .left()
            .iter()
            .filter(|&&root| !matches!(clausal.result.formulas().node(root), Node::True))
            .map(|&root| !texts.contains(&clausal.result.formulas().formula(root).to_string()))
            .collect()
    };
    let support = if support.len() == given {
        support
    } else {
        vec![true; given]
    };
    Ok((set, support))
}

/// What the search ended with.
enum Found {
    /// `□`, at its position.
    Refuted(usize),
    /// Every inference made, no `□`.
    Saturated,
    /// A bound.
    Unknown(Bound),
}

/// The search by the given clause.
struct Search<'l> {
    /// Every clause kept and made.
    set: Clauses,
    /// The inference that made each clause past the given ones.
    made_by: Vec<Inference>,
    /// The clauses used: resolved with each given clause.
    active: Vec<usize>,
    /// The clauses not used yet, the lightest first, then the oldest.
    passive: BinaryHeap<Reverse<(u64, usize)>>,
    /// The given clauses outside the set of support, used once it runs out.
    usable: Vec<usize>,
    /// The options' bounds.
    options: &'l Options,
    /// The bounds.
    limits: &'l Limits,
    /// What it cost so far.
    statistics: Statistics,
    /// The bytes it holds, estimated.
    bytes: u64,
}

impl<'l> Search<'l> {
    /// Returns a search over `set`, `support` saying which given clauses
    /// are the set of support.
    fn new(set: Clauses, support: Vec<bool>, options: &'l Options, limits: &'l Limits) -> Self {
        let mut search = Self {
            made_by: Vec::new(),
            active: Vec::new(),
            passive: BinaryHeap::new(),
            usable: Vec::new(),
            options,
            limits,
            statistics: Statistics {
                clauses: set.items.len() as u64,
                steps: 0,
            },
            bytes: 0,
            set,
        };
        for (k, &supported) in support.iter().enumerate() {
            if supported {
                let clause = search.set.items[k].clone();
                let w = search.set.weight(&clause);
                search.passive.push(Reverse((w, k)));
            } else {
                // The hypotheses' clauses wait as partners of the given
                // clauses.
                search.active.push(k);
                search.usable.push(k);
            }
        }
        search
    }

    /// Runs the search until `□`, saturation or a bound.
    fn run(&mut self, stop: &mut dyn FnMut(Progress) -> bool) -> Result<Found, Error> {
        if let Some(k) = self.set.items.iter().position(Clause::is_empty) {
            return Ok(Found::Refuted(k));
        }
        loop {
            let Some(Reverse((_, given))) = self.passive.pop() else {
                // The set of support ran out: the hypotheses' clauses are
                // given clauses too, so nothing is left unresolved.
                if self.usable.is_empty() {
                    return Ok(Found::Saturated);
                }
                let usable = std::mem::take(&mut self.usable);
                self.active.retain(|k| !usable.contains(k));
                for k in usable {
                    let clause = self.set.items[k].clone();
                    let w = self.set.weight(&clause);
                    self.passive.push(Reverse((w, k)));
                }
                continue;
            };
            if stop(Progress::new(Phase::Search, 1, self.statistics.steps)) {
                return Ok(Found::Unknown(Bound::Stopped));
            }
            let clause = self.set.items[given].clone();
            if self.active.iter().any(|&a| {
                self.set
                    .subsumes(&self.set.items[a], &clause, SUBSUMPTION_LIMIT)
            }) {
                continue;
            }
            let mut steps = Vec::new();
            for i in 0..clause.literals.len() {
                for j in i + 1..clause.literals.len() {
                    if clause.literals[i].positive == clause.literals[j].positive {
                        steps.push(Step::Factor {
                            clause: given,
                            first_literal: i,
                            second_literal: j,
                        });
                    }
                }
            }
            self.active.push(given);
            let paramodulating = self.options.equality == super::Equality::Paramodulation
                && self.set.formulas.has_equality();
            for &other in &self.active {
                let partner = &self.set.items[other];
                for (i, x) in clause.literals.iter().enumerate() {
                    for (j, y) in partner.literals.iter().enumerate() {
                        if x.positive != y.positive {
                            steps.push(Step::Resolution {
                                first: given,
                                second: other,
                                first_literal: i,
                                second_literal: j,
                            });
                        }
                    }
                }
                if paramodulating {
                    // The walk over wide literals asks the stop as it goes,
                    // and once per pair of clauses.
                    let pairs = [(given, other), (other, given)];
                    for &(from, into) in &pairs[..if other == given { 1 } else { 2 }] {
                        if !self.set.paramodulations(from, into, &mut steps, &mut *stop)
                            || stop(Progress::new(Phase::Search, 1, self.statistics.steps))
                        {
                            return Ok(Found::Unknown(Bound::Stopped));
                        }
                    }
                    self.charge((steps.len() as u64).saturating_mul(size_of::<Step>() as u64))?;
                }
            }
            for step in steps {
                if stop(Progress::new(Phase::Search, 1, self.statistics.steps)) {
                    return Ok(Found::Unknown(Bound::Stopped));
                }
                if let Some(limit) = self.options.steps
                    && self.statistics.steps >= limit
                {
                    return Ok(Found::Unknown(Bound::Steps { limit }));
                }
                let Ok(made) = self.set.made(step)? else {
                    continue;
                };
                self.statistics.steps += 1;
                let new = made.clause;
                if Clauses::is_tautology(&new)
                    || self
                        .set
                        .items
                        .iter()
                        .any(|c| self.set.subsumes(c, &new, SUBSUMPTION_LIMIT))
                {
                    continue;
                }
                if let Some(limit) = self.options.clauses
                    && self.statistics.clauses >= limit
                {
                    return Ok(Found::Unknown(Bound::Clauses { limit }));
                }
                self.hold(&new)?;
                let empty = new.is_empty();
                let w = self.set.weight(&new);
                self.set.items.push(new);
                let k = self.set.items.len() - 1;
                self.made_by.push(Inference {
                    step,
                    bindings: made.bindings,
                    clause: k,
                });
                self.statistics.clauses += 1;
                if empty {
                    return Ok(Found::Refuted(k));
                }
                self.passive.push(Reverse((w, k)));
            }
        }
    }

    /// Charges a clause made, and refuses past the memory bound.
    fn hold(&mut self, clause: &Clause) -> Result<(), Error> {
        self.bytes = self
            .bytes
            .saturating_add(64 + 16 * clause.literals.len() as u64);
        self.charge(0)
    }

    /// Refuses where what the search holds, with `pending` bytes of steps
    /// to try, passes the memory bound.
    fn charge(&self, pending: u64) -> Result<(), Error> {
        match self.limits.memory_bytes {
            Some(limit_bytes) if self.bytes.saturating_add(pending) > limit_bytes => {
                Err(Error::Refused(Refusal::Memory {
                    phase: Phase::Search,
                    limit_bytes,
                    needed_bytes: None,
                }))
            }
            _ => Ok(()),
        }
    }

    /// Returns the refutation of the clause at `empty`: its ancestors'
    /// inferences, in order, the clauses renumbered.
    fn refutation(&self, empty: usize) -> Refutation {
        let given = self.set.given;
        let mut needed = vec![false; self.set.items.len()];
        needed[empty] = true;
        for k in (given..self.set.items.len()).rev() {
            if !needed[k] {
                continue;
            }
            match self.made_by[k - given].step {
                Step::Resolution { first, second, .. } => {
                    needed[first] = true;
                    needed[second] = true;
                }
                Step::Factor { clause, .. } => needed[clause] = true,
                Step::Paramodulation { from, into, .. } => {
                    needed[from] = true;
                    needed[into] = true;
                }
            }
        }
        let mut position: Vec<usize> = (0..given).collect();
        position.resize(self.set.items.len(), usize::MAX);
        let mut clauses = self.set.items[..given].to_vec();
        let mut inferences = Vec::new();
        for k in given..self.set.items.len() {
            if !needed[k] {
                continue;
            }
            let inference = &self.made_by[k - given];
            let step = match inference.step {
                Step::Resolution {
                    first,
                    second,
                    first_literal,
                    second_literal,
                } => Step::Resolution {
                    first: position[first],
                    second: position[second],
                    first_literal,
                    second_literal,
                },
                Step::Factor {
                    clause,
                    first_literal,
                    second_literal,
                } => Step::Factor {
                    clause: position[clause],
                    first_literal,
                    second_literal,
                },
                Step::Paramodulation {
                    from,
                    from_literal,
                    into,
                    into_literal,
                    position: place,
                    reverse,
                } => Step::Paramodulation {
                    from: position[from],
                    from_literal,
                    into: position[into],
                    into_literal,
                    position: place,
                    reverse,
                },
            };
            clauses.push(self.set.items[k].clone());
            position[k] = clauses.len() - 1;
            inferences.push(Inference {
                step,
                bindings: inference.bindings.clone(),
                clause: clauses.len() - 1,
            });
        }
        Refutation {
            clauses: Clauses {
                formulas: self.set.formulas.clone(),
                items: clauses,
                given,
                most_terms: self.set.most_terms,
                sizes: HashMap::default(),
            },
            inferences,
        }
    }
}

/// Reads a checked refutation back as a derivation of LK of `sequent`:
/// each clause the refutation uses gets the ground substitutions its
/// consequences need, `□` the empty one, so the given clauses' instances
/// are a ground refutation's; from them Herbrand's construction builds the
/// derivation, each Skolem term an eigenvariable, and checks it.
///
/// # Errors
///
/// [`Refusal::Occurrences`] past [`MOST_INSTANCES`] instances;
/// [`Refusal::Terms`] for an instance's atom past `limits.terms`;
/// [`Refusal::Memory`] where the instances pass `limits.memory_bytes`;
/// [`Refusal::Stopped`] when the stop fires; Herbrand's construction's
/// errors.
fn read_back(
    sequent: &Sequent,
    clausal: &NormalForm,
    refutation: &Refutation,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Derivation, Error> {
    // A ground instance of a paramodulant follows from its parents' only
    // with equality's axioms, which the refutation does not use: LK reads
    // no such step back.
    if refutation
        .inferences
        .iter()
        .any(|i| matches!(i.step, Step::Paramodulation { .. }))
    {
        return Err(Error::ReadBack {
            calculus: "LK",
            reason: "a refutation by paramodulation is checked step by step; LK has no rule for \
                     the step"
                .to_owned(),
        });
    }
    let mut set = Clauses {
        formulas: refutation.clauses.formulas.clone(),
        items: refutation.clauses.items[..refutation.clauses.given].to_vec(),
        given: refutation.clauses.given,
        most_terms: refutation.clauses.most_terms,
        sizes: HashMap::default(),
    };
    // The terms put for variables no consequence fixes: a constant of the
    // clauses, or a new one.
    // A propositional arena stays without tables: it has no variable to
    // put a constant for.
    let default = if set.formulas.is_first_order() {
        Some(default_constant(&mut set)?)
    } else {
        None
    };
    let mut made = Vec::with_capacity(refutation.inferences.len());
    for inference in &refutation.inferences {
        let Ok(m) = set.made(inference.step)? else {
            return Err(Error::ReadBack {
                calculus: "LK",
                reason: "a checked step does not apply".to_owned(),
            });
        };
        set.items.push(m.clause.clone());
        made.push(m);
    }
    let total = set.items.len();
    let mut needed: Vec<Vec<Vec<FoTermId>>> = vec![Vec::new(); total];
    let empty = if refutation.inferences.is_empty() {
        set.items
            .iter()
            .position(Clause::is_empty)
            .unwrap_or(total - 1)
    } else {
        total - 1
    };
    needed[empty].push(Vec::new());
    let mut count = 0usize;
    // The read-back's steps, each an instance of a clause, for the stop.
    let mut steps = 0u64;
    let mut poll = |steps: &mut u64| {
        *steps += 1;
        if stop(Progress::new(Phase::ReadBack, 1, *steps)) {
            return Err(Error::Refused(Refusal::stopped(Phase::ReadBack)));
        }
        Ok(())
    };
    for k in (set.given..total).rev() {
        let m = &made[k - set.given];
        let wants = std::mem::take(&mut needed[k]);
        for theta in wants {
            poll(&mut steps)?;
            // Each variable of the step ground: its term under the unifier
            // with the variables left free put as the clause's.
            let mut ground = Vec::with_capacity(m.terms.len());
            for &t in &m.terms {
                let mut memo = HashMap::default();
                let image = map_term(
                    &mut set.formulas,
                    t,
                    &mut |_, j| {
                        m.renamed
                            .get(j as usize)
                            .copied()
                            .flatten()
                            .and_then(|r| theta.get(r as usize).copied())
                            .or(default)
                            .unwrap_or(t)
                    },
                    &mut memo,
                );
                ground.push(image);
            }
            let split = m.first as usize;
            let (first, second) = match refutation.inferences[k - set.given].step {
                Step::Resolution { first, second, .. } => (first, Some(second)),
                Step::Factor { clause, .. } => (clause, None),
                Step::Paramodulation { from, into, .. } => (from, Some(into)),
            };
            let mut add = |c: usize, sub: Vec<FoTermId>| {
                if !needed[c].contains(&sub) {
                    needed[c].push(sub);
                    count += 1;
                }
            };
            add(first, ground[..split].to_vec());
            if let Some(second) = second {
                add(second, ground[split..].to_vec());
            }
            if count > MOST_INSTANCES {
                return Err(Error::Refused(Refusal::Occurrences {
                    occurrences: count as u64,
                    limit: MOST_INSTANCES as u64,
                }));
            }
        }
    }
    // The given clauses' ground instances, as formulas. Each atom is held
    // to the bound on terms before its key is written, and the arena they
    // grow to the memory bound: a unifier that doubles a term per variable
    // made keys of millions of symbols.
    let memory = limits.memory_bytes.unwrap_or(u64::MAX);
    let mut held = ArenaSize::of(&set.formulas, &ArenaSize::default());
    let start = held.bytes;
    // Refuses the instances past the bound where they have grown the
    // arena by `grown` bytes: the copy into the derivation's arena below
    // holds as much again.
    let within = |grown: u64| {
        let needed_bytes = grown.saturating_mul(2);
        if needed_bytes > memory {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::ReadBack,
                limit_bytes: memory,
                needed_bytes: Some(needed_bytes),
            }));
        }
        Ok(())
    };
    let mut roots = Vec::new();
    for c in 0..set.given {
        for theta in &needed[c] {
            poll(&mut steps)?;
            let clause = set.items[c].clone();
            let mut instance: Option<NodeId> = None;
            for l in &clause.literals {
                // What the instances hold once this atom's key is written
                // (its text is at least a byte a symbol, held twice),
                // charged before the key is written.
                let grown = held.bytes.saturating_sub(start);
                let mut charge = |size: u64| within(grown.saturating_add(size.saturating_mul(2)));
                let atom = set.mapped_atom_charged(
                    l.atom,
                    &mut |formulas, j| {
                        theta
                            .get(j as usize)
                            .copied()
                            .or(default)
                            .unwrap_or_else(|| formulas.interner().term(FoTerm::Meta(j)))
                    },
                    &mut charge,
                )?;
                held = ArenaSize::of(&set.formulas, &held);
                within(held.bytes.saturating_sub(start))?;
                let literal = if l.positive {
                    atom
                } else {
                    set.formulas.add(Node::Not(atom))?
                };
                instance = Some(match instance {
                    None => literal,
                    Some(before) => set.formulas.add(Node::Or(before, literal))?,
                });
            }
            let instance = match instance {
                Some(i) => i,
                None => set.formulas.add(Node::False)?,
            };
            roots.push(instance);
        }
    }
    // A derivation's arena holds no metavariable: the instances go into a
    // copy of the clausal form's, which the input's ids are ids of.
    let mut clean = clausal.result.formulas().clone();
    let mut copied = Copied::default();
    let mut ground_roots = Vec::with_capacity(roots.len());
    for root in roots {
        poll(&mut steps)?;
        ground_roots.push(copied.formula(&set.formulas, &mut clean, root)?);
    }
    let default = match default {
        Some(d) => vec![copied.term(&set.formulas, &mut clean, d)?],
        None => Vec::new(),
    };
    let ground = Sequent::new(clean, ground_roots, Vec::new())?;
    super::super::herbrand::deskolemized(sequent, clausal, &ground, &default, limits, stop)
}

/// What a copy of ground formulas into another arena has made, by the
/// source's ids.
#[derive(Debug, Default)]
struct Copied {
    /// Each node copied.
    nodes: HashMap<NodeId, NodeId>,
    /// Each term copied.
    terms: HashMap<FoTermId, FoTermId>,
}

impl Copied {
    /// Returns the ground formula `root` of `from` made in `to`, its
    /// symbols by name.
    fn formula(
        &mut self,
        from: &Formulas,
        to: &mut Formulas,
        root: NodeId,
    ) -> Result<NodeId, Error> {
        let mut stack = vec![(root, false)];
        while let Some((n, ready)) = stack.pop() {
            if self.nodes.contains_key(&n) {
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
                Node::Atom(a) => match from.first_order().atoms.get(a as usize) {
                    Some(&(p, args)) => {
                        let args = from.first_order().list(args).to_vec();
                        let mut items = Vec::with_capacity(args.len());
                        for t in args {
                            items.push(self.term(from, to, t)?);
                        }
                        let symbol = symbol_in(from, to, p, true)?;
                        let list = to.interner().list(&items);
                        to.predicate_atom(symbol, list)?
                    }
                    None => to.atom(from.atom_name(a))?,
                },
                Node::True => to.add(Node::True)?,
                Node::False => to.add(Node::False)?,
                Node::Not(a) => to.add(Node::Not(self.nodes[&a]))?,
                Node::Or(a, b) => to.add(Node::Or(self.nodes[&a], self.nodes[&b]))?,
                Node::And(a, b) => to.add(Node::And(self.nodes[&a], self.nodes[&b]))?,
                Node::Implies(..) | Node::Iff(..) | Node::Forall(_) | Node::Exists(_) => {
                    return Err(Error::ReadBack {
                        calculus: "LK",
                        reason: "a ground instance is no clause".to_owned(),
                    });
                }
            };
            self.nodes.insert(n, image);
        }
        Ok(self.nodes[&root])
    }

    /// Returns the ground term `t` of `from` made in `to`.
    fn term(&mut self, from: &Formulas, to: &mut Formulas, t: FoTermId) -> Result<FoTermId, Error> {
        let mut stack = vec![(t, false)];
        while let Some((u, ready)) = stack.pop() {
            if self.terms.contains_key(&u) {
                continue;
            }
            let FoTerm::App(f, args) = from.first_order().terms[u.index()] else {
                return Err(Error::ReadBack {
                    calculus: "LK",
                    reason: "a ground instance holds a variable".to_owned(),
                });
            };
            let list = from.first_order().list(args).to_vec();
            if !ready {
                stack.push((u, true));
                stack.extend(list.into_iter().map(|v| (v, false)));
                continue;
            }
            let items: Vec<FoTermId> = list.iter().map(|v| self.terms[v]).collect();
            let symbol = symbol_in(from, to, f, false)?;
            let args = to.interner().list(&items);
            let image = to.interner().term(FoTerm::App(symbol, args));
            self.terms.insert(u, image);
        }
        Ok(self.terms[&t])
    }
}

/// Returns the symbol of `to` named and of the arity of `symbol` of `from`.
fn symbol_in(
    from: &Formulas,
    to: &mut Formulas,
    symbol: crate::first_order::Symbol,
    predicate: bool,
) -> Result<crate::first_order::Symbol, Error> {
    let (name, arity) = from.first_order().symbols[symbol.index()].clone();
    to.interner()
        .symbol(&name, arity, predicate)
        .map_err(|fault| Error::Symbol { name, fault })
}

/// Returns a constant of the given clauses, or a new one where they have
/// none.
fn default_constant(set: &mut Clauses) -> Result<FoTermId, Error> {
    let tables = set.formulas.first_order();
    for clause in &set.items {
        for l in &clause.literals {
            let Node::Atom(a) = set.formulas.node(l.atom) else {
                continue;
            };
            let Some(&(_, args)) = tables.atoms.get(a as usize) else {
                continue;
            };
            let mut stack: Vec<FoTermId> = tables.list(args).to_vec();
            while let Some(t) = stack.pop() {
                if let FoTerm::App(_, args) = tables.terms[t.index()] {
                    let list = tables.list(args);
                    if list.is_empty() {
                        return Ok(t);
                    }
                    stack.extend_from_slice(list);
                }
            }
        }
    }
    let name = format!("{}1", normal::fresh_prefix(&set.formulas, "c"));
    let symbol = set
        .formulas
        .interner()
        .symbol(&name, 0, false)
        .map_err(|fault| Error::Symbol { name, fault })?;
    let list = set.formulas.interner().list(&[]);
    Ok(set.formulas.interner().term(FoTerm::App(symbol, list)))
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;

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
