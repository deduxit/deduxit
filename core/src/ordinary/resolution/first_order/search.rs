// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The given-clause search for a refutation, from the clauses of the
//! clausal form.

use super::{Clause, Clauses, Inference, Literal, Refutation, SUBSUMPTION_LIMIT, Statistics, Step};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::ordinary::normal::NormalForm;
use crate::ordinary::resolution::{Bound, Options};
use crate::ordinary::{Node, Sequent, normal};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Returns the clauses of the clausal form, each hypothesis `∀x̄. C` its
/// literals with `x̄` the metavariables `0`, `1`, … outermost first, and
/// with `support`, which are the set of support: those that are no clause
/// of the hypotheses alone, where the sequent has formulas right of `⊢`;
/// all of them otherwise.
pub(super) fn given_clauses(
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
pub(super) enum Found {
    /// `□`, at its position.
    Refuted(usize),
    /// Every inference made, no `□`.
    Saturated,
    /// A bound.
    Unknown(Bound),
}

/// The search by the given clause.
pub(super) struct Search<'l> {
    /// Every clause kept and made.
    pub(super) set: Clauses,
    /// The inference that made each clause past the given ones.
    pub(super) made_by: Vec<Inference>,
    /// The clauses used: resolved with each given clause.
    pub(super) active: Vec<usize>,
    /// The clauses not used yet, the lightest first, then the oldest.
    pub(super) passive: BinaryHeap<Reverse<(u64, usize)>>,
    /// The given clauses outside the set of support, used once it runs out.
    pub(super) usable: Vec<usize>,
    /// The options' bounds.
    pub(super) options: &'l Options,
    /// The bounds.
    pub(super) limits: &'l Limits,
    /// What it cost so far.
    pub(super) statistics: Statistics,
    /// The bytes it holds, estimated.
    pub(super) bytes: u64,
}

impl<'l> Search<'l> {
    /// Returns a search over `set`, `support` saying which given clauses
    /// are the set of support.
    pub(super) fn new(
        set: Clauses,
        support: Vec<bool>,
        options: &'l Options,
        limits: &'l Limits,
    ) -> Self {
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
    pub(super) fn run(&mut self, stop: &mut dyn FnMut(Progress) -> bool) -> Result<Found, Error> {
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
            let paramodulating = self.options.equality
                == crate::ordinary::resolution::Equality::Paramodulation
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
    pub(super) fn refutation(&self, empty: usize) -> Refutation {
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
