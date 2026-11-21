// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The proof search of the prefixed non-clausal connection calculus
//! (Otten, "Non-clausal connection calculi for non-classical logics",
//! TABLEAUX 2017, Figure 2, the calculus of nanoCoP-i): the classical
//! search, with prefixes beside it.
//!
//! - Each connection's two prefixes must be unifiable on their own
//!   (Otten and Kreitz's test), and their equation is kept.
//! - Once every path holds a connection, the equations of the whole proof
//!   are unified together, with one more per eigenvariable a term variable
//!   is bound to (the domain condition); where none unifies, the search
//!   backtracks into its last choice, as nanoCoP-i does.
//! - A proof that unifies is handed to the caller, which reads it back to
//!   LJ; where that fails, the search backtracks too.
//! - Regularity and lemmata compare prefixes as well as terms, and a
//!   literal solved is never settled: another of its solutions may be the
//!   one whose prefixes unify.
//! - A copy of a clause renames its prefix characters, so the bound on the
//!   active path holds for every copy that renames a variable or a
//!   character; the search is no decision procedure, even without
//!   quantifiers, and a failure is "unknown".

use super::super::matrix::{Matrix, NONE, Shape, Symbol};
use super::super::prove::{Config, Counts};
use super::super::terms::{Head, Mark, Terms};
use super::prefix::{AUXILIARY, NoUnifier, Prefixes, Sym, unifiable, unify};
use crate::Error;
use crate::hash::HashSet;
use crate::limits::{Limits, Phase, Progress};
use crate::linear::search::Reason;
use crate::ordinary::Derivation;

/// How many steps the search takes between two asks of its stop.
const POLL: u64 = 1 << 10;

/// The prefix characters read that count as one step between two asks
/// of the stop.
const PREFIX_CHARS: u64 = 1 << 6;

/// The steps the unification of a proof's prefixes may take before
/// the proof is given up.
const UNIFY_STEPS: u64 = 1 << 16;

/// How a search ended.
pub(super) enum End {
    /// A proof, read back.
    Proved(Box<Derivation>),
    /// No proof, for the reason given.
    Unknown(Reason),
}

/// What the caller makes of a proof whose prefixes unify: its derivation,
/// `None` where it does not read back (the search goes on), or an error
/// that ends the search.
pub(super) type Accept<'a> = dyn FnMut(&Terms) -> Result<Option<Derivation>, Error> + 'a;

/// An element of the active path: a literal, the copy it lies in, and
/// the copy its extension step made.
#[derive(Clone, Copy, Debug)]
struct PathNode {
    /// The literal.
    lit: u32,
    /// Its copy.
    copy: u32,
    /// The path below it, [`NONE`] for the empty one.
    parent: u32,
    /// The path's length with it.
    depth: u32,
    /// The copy of the extension clause its step made.
    ext: u32,
}

/// A goal.
#[derive(Clone, Copy, Debug)]
enum Item {
    /// An element of a subgoal clause to prove: a literal or a matrix of
    /// the matrix, read in a copy, under the active path.
    Elem {
        /// The node.
        node: u32,
        /// The copy.
        copy: u32,
        /// The active path.
        path: u32,
        /// Whether only positive clauses of it are taken (the start
        /// clause's, Lemma 4).
        positive: bool,
    },
    /// A literal whose proof is done: it becomes a lemma.
    Solved {
        /// The literal.
        lit: u32,
        /// Its copy.
        copy: u32,
        /// The active path it was solved under.
        path: u32,
        /// The choice points before its own.
        choices: u32,
        /// The terms when it was taken up.
        mark: Mark,
    },
}

/// A cell of a goal list.
#[derive(Clone, Copy, Debug)]
struct Goal {
    /// The goal.
    item: Item,
    /// The rest of the list, [`NONE`] for none.
    next: u32,
}

/// A literal solved under an active path, which closes an equal literal
/// that lies in its clause, in its copy, under a longer active path.
///
/// Why that is sound: a path through such a literal `L'` takes, at the
/// solved literal's clause, the element `L'` lies in. Taking the solved
/// literal there instead, and dropping what lay under that element, gives
/// a path through the solved literal and the active path it was solved
/// under, provided no literal of that active path lies in the clause; it
/// holds a connection, which is one of the first path's or the solved
/// literal's, and the solved literal is `L'` under the substitution. A
/// literal elsewhere has no such path, and a lemma reused there made a
/// proof of a sequent that is not valid.
#[derive(Clone, Copy, Debug)]
struct Lemma {
    /// The literal.
    lit: u32,
    /// Its copy.
    copy: u32,
    /// The active path it was solved under, [`NONE`] for the empty one.
    path: u32,
}

/// Where the alternatives of a literal stand.
#[derive(Clone, Copy, Debug)]
struct Cursor {
    /// The next element of the active path to reduce with, counted from
    /// the top.
    reduction: u32,
    /// The next literal of the complementary predicate to extend to.
    target: u32,
    /// Its extension clauses, `clauses.0 .. clauses.0 + clauses.1` of
    /// `Search::eclauses`, once computed.
    clauses: (u32, u32),
    /// The next of them.
    clause: u32,
}

/// What a choice point tries next.
#[derive(Clone, Copy, Debug)]
enum Alt {
    /// The next start clause.
    Start(u32),
    /// The next clause of a matrix.
    Clause(u32),
    /// The next way to solve a literal.
    Literal(Cursor),
}

/// How far every structure had grown.
#[derive(Clone, Copy, Debug)]
struct Marks {
    /// The terms.
    terms: Mark,
    /// The goal cells.
    goals: u32,
    /// The path nodes.
    paths: u32,
    /// The lemmata.
    lemmas: u32,
    /// The literals used.
    used: u32,
    /// The start clause's copy.
    start: u32,
    /// The prefix equations.
    eqs: u32,
    /// The extension clauses computed.
    eclauses: u32,
}

/// A choice point.
#[derive(Clone, Copy, Debug)]
struct Choice {
    /// The goal list whose first goal it was made for; unused for a start.
    goal: u32,
    /// What it tries next.
    alt: Alt,
    /// What backtracking cuts back to.
    marks: Marks,
}

/// One way to solve a literal.
#[derive(Clone, Copy, Debug)]
enum Way {
    /// A reduction step with the path node given.
    Reduce(u32),
    /// An extension step to the literal given in a copy of the clause
    /// given, laid in the region of the copy given.
    Extend(u32, u32, u32),
}

/// When the search asks its stop and its bounds: at a count of work, and
/// the count at the last ask. Work is a step, and a step per
/// [`PREFIX_CHARS`] prefix characters read.
#[derive(Clone, Copy, Debug, Default)]
struct Clock {
    /// The work done.
    work: u64,
    /// The work count of the next ask.
    next: u64,
    /// The work count of the last.
    last: u64,
}

/// The most entries a table of a round holds: past it a round is
/// [`Reason::IndexLimit`], with room for the steps up to the next poll
/// before the `u32` indices run out.
const MOST_ENTRIES: usize = 1 << 31;

/// The state of one round of the search.
struct Search<'m, 'p, 's> {
    /// The matrix.
    matrix: &'m Matrix,
    /// The bytes the matrix holds, which the memory bound counts.
    matrix_bytes: usize,
    /// The caller's limits.
    limits: &'s Limits,
    /// The caller's stop.
    stop: &'s mut dyn FnMut(Progress) -> bool,
    /// When the stop and the bounds are asked next, and were last.
    clock: Clock,
    /// Why a poll ended the round, once one did.
    halted: Option<Reason>,
    /// The prefixes' characters.
    prefixes: &'p mut Prefixes,
    /// Per clause node, whether a copy of it renames a character.
    renames: &'m [bool],
    /// The prefix equation of every connection made.
    eqs: Vec<(Vec<Sym>, Vec<Sym>)>,
    /// Proofs whose prefixes did not unify, and proofs that did but did
    /// not read back.
    pub(super) rejected: (u64, u64),
    /// The copies and the substitution.
    terms: Terms,
    /// The goal cells.
    goals: Vec<Goal>,
    /// The path nodes.
    paths: Vec<PathNode>,
    /// The lemmata.
    lemmas: Vec<Lemma>,
    /// The literals used, each with its copy.
    used: Vec<(u32, u32)>,
    /// The extension clauses computed for cursors: a clause and the copy
    /// whose region its copy goes in.
    eclauses: Vec<(u32, u32)>,
    /// The choice points.
    choices: Vec<Choice>,
    /// The copy of the start clause.
    start: u32,
    /// The bound on the active path for steps that make a variable.
    limit: u32,
    /// Whether that bound refused a step.
    limit_hit: bool,
    /// Whether restricted backtracking is on.
    restricted: bool,
}

/// Searches for a proof of `matrix` whose prefixes unify and which
/// `accept` reads back, deepening the bound on the active path from 1,
/// and asks `stop` every [`POLL`] steps.
///
/// # Errors
///
/// What `accept` answers.
#[expect(
    clippy::too_many_arguments,
    reason = "the matrix, its prefixes, the bounds and the caller's hooks"
)]
pub(super) fn search(
    matrix: &Matrix,
    prefixes: &mut Prefixes,
    config: Config,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
    accept: &mut Accept<'_>,
    counts: &mut Counts,
    rejected: &mut (u64, u64),
) -> Result<End, Error> {
    let renames: Vec<bool> = (0..matrix.nodes.len() as u32)
        .map(|n| matrix.nodes[n as usize].shape == Shape::Clause && prefixes.renames(n))
        .collect();
    // A bound of 0 is one round where no step may make a variable.
    let mut limit = config.path_length.map_or(1, |bound| bound.min(1));
    let matrix_bytes = matrix.held_bytes();
    let mut clock = Clock {
        work: 0,
        next: POLL,
        last: 0,
    };
    loop {
        counts.path_length = limit;
        let mut round = Search {
            matrix,
            matrix_bytes,
            limits,
            stop: &mut *stop,
            clock,
            halted: None,
            prefixes,
            renames: &renames,
            eqs: Vec::new(),
            rejected: (0, 0),
            terms: Terms::new(matrix),
            goals: Vec::new(),
            paths: Vec::new(),
            lemmas: Vec::new(),
            used: Vec::new(),
            eclauses: Vec::new(),
            choices: Vec::new(),
            start: 0,
            limit,
            limit_hit: false,
            restricted: config.restricted,
        };
        let ran = round.run(counts, accept);
        clock = round.clock;
        rejected.0 += round.rejected.0;
        rejected.1 += round.rejected.1;
        match ran {
            Ok(Some(derivation)) => return Ok(End::Proved(Box::new(derivation))),
            Ok(None) => {}
            Err(Ended::Reason(reason)) => return Ok(End::Unknown(reason)),
            Err(Ended::Error(error)) => return Err(error),
        }
        // Prefixes need copies even without quantifiers, so no round is
        // the calculus's whole search.
        if !round.limit_hit {
            return Ok(End::Unknown(Reason::Incomplete));
        }
        if config.path_length.is_some_and(|bound| limit >= bound) {
            return Ok(End::Unknown(Reason::PathLength { length: limit }));
        }
        limit = limit.saturating_add(1);
    }
}

/// Why a round ended without a proof or a failure.
enum Ended {
    /// A bound or the stop.
    Reason(Reason),
    /// The caller's error.
    Error(Error),
}

impl Search<'_, '_, '_> {
    /// Runs one round: the derivation of a proof, `None` for a failure,
    /// and why a bound, the stop or the caller ended it.
    fn run(
        &mut self,
        counts: &mut Counts,
        accept: &mut Accept<'_>,
    ) -> Result<Option<Derivation>, Ended> {
        let mut current = self.try_start(0, counts);
        loop {
            if let Some(reason) = self.halted.take() {
                return Err(Ended::Reason(reason));
            }
            if self.tick(counts) {
                continue;
            }
            let Some(g) = current else {
                // Backtrack to the latest choice point with an alternative.
                let Some(choice) = self.choices.pop() else {
                    return Ok(None);
                };
                self.restore(choice.marks);
                current = match choice.alt {
                    Alt::Start(next) => self.try_start(next, counts),
                    Alt::Clause(next) => self.try_clause(choice.goal, next),
                    Alt::Literal(cursor) => self.try_literal(choice.goal, cursor, counts),
                };
                continue;
            };
            if g == NONE {
                // Every path holds a connection: the prefixes decide, and
                // then the read-back.
                let unified = match self.prefixes_unify(counts.steps) {
                    Ok(unified) => unified,
                    Err(NoUnifier::Stopped) => return Err(Ended::Reason(Reason::Stopped)),
                    Err(NoUnifier::Failed) => false,
                };
                if !unified {
                    self.rejected.0 += 1;
                } else if let Some(derivation) = accept(&self.terms).map_err(|error| {
                    // The stop or the memory bound during the read-back ends
                    // the search, as during the search.
                    if let Error::Refused(crate::Refusal::Memory { limit_bytes, .. }) = error {
                        Ended::Reason(Reason::MemoryLimit { limit_bytes })
                    } else if matches!(error, Error::Refused(crate::Refusal::Stopped { .. })) {
                        Ended::Reason(Reason::Stopped)
                    } else {
                        Ended::Error(error)
                    }
                })? {
                    return Ok(Some(derivation));
                } else {
                    self.rejected.1 += 1;
                }
                current = None;
                continue;
            }
            let goal = self.goals[g as usize];
            current = match goal.item {
                Item::Elem { node, .. } => match self.matrix.nodes[node as usize].shape {
                    Shape::Matrix => self.try_clause(g, 0),
                    Shape::Literal(_) => self.enter_literal(g, counts),
                    Shape::Clause => unreachable!("a clause is never a goal"),
                },
                Item::Solved {
                    lit,
                    copy,
                    path,
                    choices,
                    mark,
                } => {
                    if self.may_be_lemma(lit, copy, path) {
                        self.lemmas.push(Lemma { lit, copy, path });
                    }
                    // Another solution of the literal may be the one whose
                    // prefixes unify, so only restricted backtracking cuts.
                    let _ = mark;
                    if self.restricted {
                        self.choices.truncate(choices as usize);
                    }
                    Some(goal.next)
                }
            };
        }
    }

    /// Counts a step, and asks the stop and the bounds every [`POLL`]
    /// units of work. Returns whether the round must end
    /// ([`halted`](Self::halted)).
    fn tick(&mut self, counts: &mut Counts) -> bool {
        counts.steps = counts.steps.saturating_add(1);
        // A prefix read costs its length, which a deep nest makes dear:
        // the stop is asked by that work too.
        let read = self.prefixes.take_read() / PREFIX_CHARS;
        self.clock.work = self.clock.work.saturating_add(1 + read);
        if self.halted.is_none() && self.clock.work >= self.clock.next {
            let work = self.clock.work - self.clock.last;
            self.clock.next = self.clock.work.saturating_add(POLL);
            self.clock.last = self.clock.work;
            self.halted = self.poll(counts, work).err();
        }
        self.halted.is_some()
    }

    /// Asks the stop and the bounds.
    fn poll(&mut self, counts: &Counts, work: u64) -> Result<(), Reason> {
        if (self.stop)(Progress::new(Phase::Search, work, counts.steps)) {
            return Err(Reason::Stopped);
        }
        if let Some(limit) = self.limits.work
            && counts.steps > limit
        {
            return Err(Reason::WorkLimit { limit });
        }
        if let Some(limit) = self.limits.memory_bytes
            && self.held_bytes() as u64 > limit
        {
            return Err(Reason::MemoryLimit { limit_bytes: limit });
        }
        let longest = [
            self.goals.len(),
            self.paths.len(),
            self.lemmas.len(),
            self.used.len(),
            self.eclauses.len(),
            self.choices.len(),
            self.eqs.len(),
            self.terms.longest(),
        ];
        if longest.into_iter().any(|len| len > MOST_ENTRIES) {
            return Err(Reason::IndexLimit);
        }
        Ok(())
    }

    /// Returns the bytes the round holds, by capacity.
    fn held_bytes(&self) -> usize {
        self.matrix_bytes
            + self.terms.held_bytes()
            + self.goals.capacity() * size_of::<Goal>()
            + self.paths.capacity() * size_of::<PathNode>()
            + self.lemmas.capacity() * size_of::<Lemma>()
            + self.used.capacity() * size_of::<(u32, u32)>()
            + self.eclauses.capacity() * size_of::<(u32, u32)>()
            + self.choices.capacity() * size_of::<Choice>()
            + self.prefixes.held_bytes()
            + self.eqs.capacity() * size_of::<(Vec<Sym>, Vec<Sym>)>()
            + self
                .eqs
                .iter()
                .map(|(a, b)| (a.capacity() + b.capacity()) * size_of::<Sym>())
                .sum::<usize>()
    }

    /// Returns how far every structure has grown.
    fn marks(&self) -> Marks {
        Marks {
            terms: self.terms.mark(),
            goals: self.goals.len() as u32,
            paths: self.paths.len() as u32,
            lemmas: self.lemmas.len() as u32,
            used: self.used.len() as u32,
            start: self.start,
            eqs: self.eqs.len() as u32,
            eclauses: self.eclauses.len() as u32,
        }
    }

    /// Cuts every structure back to `marks`.
    fn restore(&mut self, marks: Marks) {
        self.terms.undo(marks.terms);
        self.goals.truncate(marks.goals as usize);
        self.paths.truncate(marks.paths as usize);
        self.lemmas.truncate(marks.lemmas as usize);
        self.used.truncate(marks.used as usize);
        self.start = marks.start;
        self.eqs.truncate(marks.eqs as usize);
        self.eclauses.truncate(marks.eclauses as usize);
    }

    /// Puts `item` in front of the list `next` and returns the new list.
    fn cons(&mut self, item: Item, next: u32) -> u32 {
        self.goals.push(Goal { item, next });
        (self.goals.len() - 1) as u32
    }

    /// Pushes the elements `nodes` of a clause, read in `copy` under
    /// `path`, in front of `next`, the first element first.
    fn elements(&mut self, nodes: &[u32], copy: u32, path: u32, positive: bool, next: u32) -> u32 {
        let mut list = next;
        for &node in nodes.iter().rev() {
            list = self.cons(
                Item::Elem {
                    node,
                    copy,
                    path,
                    positive,
                },
                list,
            );
        }
        list
    }

    /// Tries the start clauses from the `next`-th clause of the matrix on,
    /// positive ones only (Lemma 4), and returns the goals of the first.
    fn try_start(&mut self, next: u32, counts: &mut Counts) -> Option<u32> {
        let kids = self.matrix.kids(0);
        for k in next..kids.len() as u32 {
            let clause = kids[k as usize];
            if !self.matrix.nodes[clause as usize].positive {
                continue;
            }
            if self.tick(counts) {
                return None;
            }
            let marks = self.marks();
            self.choices.push(Choice {
                goal: NONE,
                alt: Alt::Start(k + 1),
                marks,
            });
            let (copy, _) = self.terms.copy(self.matrix, clause, 0);
            self.start = copy;
            let nodes = self.matrix.kids(clause).to_vec();
            return Some(self.elements(&nodes, copy, NONE, true, NONE));
        }
        None
    }

    /// Decomposes the matrix of goal `g` by its clauses from the `next`-th
    /// on and returns the goals of the first that may be taken.
    fn try_clause(&mut self, g: u32, next: u32) -> Option<u32> {
        let goal = self.goals[g as usize];
        let Item::Elem {
            node,
            copy,
            path,
            positive,
        } = goal.item
        else {
            unreachable!("a matrix goal is an element");
        };
        let kids = self.matrix.kids(node);
        for k in next..kids.len() as u32 {
            let clause = kids[k as usize];
            if positive && !self.matrix.nodes[clause as usize].positive {
                continue;
            }
            let marks = self.marks();
            self.choices.push(Choice {
                goal: g,
                alt: Alt::Clause(k + 1),
                marks,
            });
            let nodes = self.matrix.kids(clause).to_vec();
            return Some(self.elements(&nodes, copy, path, positive, goal.next));
        }
        None
    }

    /// Takes up the literal of goal `g`: refuses it where the active path
    /// holds it (regularity), closes it where a lemma is it, and else tries
    /// its alternatives.
    fn enter_literal(&mut self, g: u32, counts: &mut Counts) -> Option<u32> {
        let goal = self.goals[g as usize];
        let Item::Elem {
            node, copy, path, ..
        } = goal.item
        else {
            unreachable!("a literal goal is an element");
        };
        let Shape::Literal(lit) = self.matrix.nodes[node as usize].shape else {
            unreachable!("the node is a literal");
        };
        let literal = self.matrix.literals[lit as usize];
        // Equal terms and the same prefix: equal under every substitution.
        let prefix = self
            .prefixes
            .of_literal(self.matrix, &self.terms, lit, copy);
        let same = |search: &mut Self, other: u32, other_copy: u32| {
            let o = search.matrix.literals[other as usize];
            o.predicate == literal.predicate
                && o.left == literal.left
                && search
                    .terms
                    .equal(search.matrix, (literal.args, copy), (o.args, other_copy))
                && search
                    .prefixes
                    .of_literal(search.matrix, &search.terms, other, other_copy)
                    == prefix
        };
        let mut p = path;
        while p != NONE {
            let n = self.paths[p as usize];
            if same(self, n.lit, n.copy) {
                return None;
            }
            p = n.parent;
        }
        for k in (0..self.lemmas.len()).rev() {
            let lemma = self.lemmas[k];
            if self.on_path(lemma.path, path)
                && lemma.lit != lit
                && self.in_clause_of(lemma.lit, lemma.copy, (node, copy))
                && same(self, lemma.lit, lemma.copy)
            {
                counts.lemmata += 1;
                self.used.push((lit, copy));
                return Some(goal.next);
            }
        }
        let cursor = Cursor {
            reduction: 0,
            target: 0,
            clauses: (0, 0),
            clause: 0,
        };
        self.try_literal(g, cursor, counts)
    }

    /// Returns whether the literal `lit` read in `copy`, just solved under
    /// `path`, may close others ([`Lemma`]): no literal of `path` lies in
    /// its clause, in its copy's region.
    fn may_be_lemma(&self, lit: u32, copy: u32, path: u32) -> bool {
        let mut p = path;
        while p != NONE {
            let n = self.paths[p as usize];
            if self.in_clause_of(
                lit,
                copy,
                (self.matrix.literals[n.lit as usize].node, n.copy),
            ) {
                return false;
            }
            p = n.parent;
        }
        true
    }

    /// Returns whether the node `node` read in copy `at` lies in the clause
    /// of literal `lit`, in the region of copy `copy`. Copies rename
    /// prefix characters, so even a matrix without slots has copies of its
    /// own.
    fn in_clause_of(&self, lit: u32, copy: u32, (node, at): (u32, u32)) -> bool {
        let clause = self.matrix.nodes[self.matrix.literals[lit as usize].node as usize].parent;
        self.inside(node, at, clause, copy)
    }

    /// Returns whether the path `prefix` is `path` or lies below it.
    fn on_path(&self, prefix: u32, path: u32) -> bool {
        if prefix == NONE {
            return true;
        }
        let depth = self.paths[prefix as usize].depth;
        let mut p = path;
        while p != NONE && self.paths[p as usize].depth > depth {
            p = self.paths[p as usize].parent;
        }
        p == prefix
    }

    /// Tries the ways to solve the literal of goal `g` from `cursor` on,
    /// and returns the goals of the first that applies.
    fn try_literal(&mut self, g: u32, mut cursor: Cursor, counts: &mut Counts) -> Option<u32> {
        loop {
            let way = self.next_way(g, &mut cursor)?;
            // A poll that ends the round ends it here: the caller reads
            // `halted` before it backtracks.
            if self.tick(counts) {
                return None;
            }
            let marks = self.marks();
            self.choices.push(Choice {
                goal: g,
                alt: Alt::Literal(cursor),
                marks,
            });
            if let Some(list) = self.apply(g, way, marks, counts) {
                return Some(list);
            }
            self.choices.pop();
            self.restore(marks);
        }
    }

    /// Returns the next way to solve the literal of goal `g`, moving the
    /// cursor past it: the reductions with the active path from its top,
    /// then the extensions to each literal of the complementary predicate
    /// by each of its extension clauses.
    fn next_way(&mut self, g: u32, cursor: &mut Cursor) -> Option<Way> {
        let Item::Elem {
            node, copy, path, ..
        } = self.goals[g as usize].item
        else {
            unreachable!("a literal goal is an element");
        };
        let Shape::Literal(lit) = self.matrix.nodes[node as usize].shape else {
            unreachable!("the node is a literal");
        };
        let literal = self.matrix.literals[lit as usize];
        // Reductions: the path from its top.
        let mut p = path;
        let mut k = 0;
        while p != NONE {
            let n = self.paths[p as usize];
            if k >= cursor.reduction {
                cursor.reduction = k + 1;
                let o = self.matrix.literals[n.lit as usize];
                if o.predicate == literal.predicate && o.left != literal.left {
                    return Some(Way::Reduce(p));
                }
            }
            k += 1;
            p = n.parent;
        }
        cursor.reduction = k;
        let key = 2 * u64::from(literal.predicate) + u64::from(!literal.left);
        let targets = self.matrix.by_predicate.get(&key)?;
        while (cursor.target as usize) < targets.len() {
            let target = targets[cursor.target as usize];
            if cursor.clause == 0 && cursor.clauses.1 == 0 {
                cursor.clauses = self.extension_clauses(target, (lit, copy), path);
            }
            if cursor.clause < cursor.clauses.1 {
                let (clause, region) = self.eclauses[(cursor.clauses.0 + cursor.clause) as usize];
                cursor.clause += 1;
                return Some(Way::Extend(target, clause, region));
            }
            cursor.target += 1;
            cursor.clause = 0;
            cursor.clauses = (0, 0);
        }
        None
    }

    /// Computes the extension clauses of the literal `target` for an
    /// extension from `from` under `path`: the clauses around it, from the
    /// innermost, that every path through the active path and `from`
    /// enters, each with the region its copy goes in. A larger one is kept
    /// only where it frees more slots than the last one kept, or renames
    /// characters the last one did not, since otherwise its subgoals are
    /// the smaller one's and more.
    fn extension_clauses(&mut self, target: u32, from: (u32, u32), path: u32) -> (u32, u32) {
        let start = self.eclauses.len() as u32;
        let matrix = self.matrix;
        let mut clause = matrix.nodes[matrix.literals[target as usize].node as usize].parent;
        let mut freed: Option<usize> = None;
        while clause != NONE {
            if let Some(region) = self.region_for(clause, from, path) {
                let count = matrix
                    .clause_slots(clause)
                    .iter()
                    .filter(|&&(_, free)| free)
                    .count()
                    + usize::from(self.renames[clause as usize]);
                if freed.is_none_or(|f| count > f) {
                    self.eclauses.push((clause, region));
                    freed = Some(count);
                }
            }
            // The clause around the clause's matrix.
            let parent = matrix.nodes[clause as usize].parent;
            clause = if parent == NONE {
                NONE
            } else {
                matrix.nodes[parent as usize].parent
            };
        }
        (start, self.eclauses.len() as u32 - start)
    }

    /// Returns the region a copy of `clause` goes in for an extension from
    /// `from` under `path`, where `clause` is an extension clause there:
    /// the latest copy on the path whose clause holds `clause` strictly,
    /// and whose region of the clause's matrix holds a literal of the path
    /// or `from`; the matrix itself for a clause of the sequent's matrix.
    fn region_for(&self, clause: u32, from: (u32, u32), path: u32) -> Option<u32> {
        let matrix = self.matrix;
        let outer = matrix.nodes[clause as usize].parent;
        if outer == 0 {
            return Some(0);
        }
        // The path's literals with their copies, `from` first.
        let holds = |region: u32| -> bool {
            let inside = |(lit, copy): (u32, u32)| {
                let node = matrix.literals[lit as usize].node;
                self.inside(node, copy, outer, region)
            };
            if inside(from) {
                return true;
            }
            let mut p = path;
            while p != NONE {
                let n = self.paths[p as usize];
                if inside((n.lit, n.copy)) {
                    return true;
                }
                p = n.parent;
            }
            false
        };
        // The copies on the path, the latest first, then the start's.
        let mut p = path;
        let mut copies = Vec::new();
        while p != NONE {
            copies.push(self.paths[p as usize].ext);
            p = self.paths[p as usize].parent;
        }
        copies.push(self.start);
        copies.into_iter().find(|&c| {
            let d = self.terms.copies[c as usize].clause;
            d != clause && matrix.within(clause, d) && holds(c)
        })
    }

    /// Returns whether the node `node` read in copy `copy` lies in the
    /// region of copy `region` inside its node `outer`.
    fn inside(&self, node: u32, copy: u32, outer: u32, region: u32) -> bool {
        if copy == region {
            return self.matrix.within(node, outer);
        }
        let mut k = copy;
        loop {
            let parent = self.terms.copies[k as usize].parent;
            if parent == region {
                return self
                    .matrix
                    .within(self.terms.copies[k as usize].clause, outer);
            }
            if parent == NONE {
                return false;
            }
            k = parent;
        }
    }

    /// Keeps the prefix equation of a connection of `a` and `b`, each a
    /// literal in its copy, where the two prefixes can be unified at all.
    fn connect(&mut self, (a, ca): (u32, u32), (b, cb): (u32, u32)) -> bool {
        let pa = self.prefixes.of_literal(self.matrix, &self.terms, a, ca);
        let pb = self.prefixes.of_literal(self.matrix, &self.terms, b, cb);
        if !unifiable(&pa, &pb) {
            return false;
        }
        self.eqs.push((pa, pb));
        true
    }

    /// Returns whether the prefix equations of the proof, with the domain
    /// condition's, have a unifier: for every term variable bound to a term
    /// with an eigenvariable (a Skolem term), the eigenvariable's prefix
    /// begins the variable's. `NoUnifier::Stopped` where the stop fired.
    fn prefixes_unify(&mut self, steps: u64) -> Result<bool, NoUnifier> {
        let matrix = self.matrix;
        let ask = &mut *self.stop;
        let mut stop = || ask(Progress::new(Phase::Search, 0, steps));
        let mut eqs = self.eqs.clone();
        for k in 0..self.terms.copies.len() as u32 {
            let c = self.terms.copies[k as usize];
            // The variables this copy made, with their slots.
            let made: Vec<(u32, u32)> = if c.clause == NONE {
                (0..c.vars.1)
                    .map(|s| (s, self.terms.vars[s as usize]))
                    .collect()
            } else {
                matrix
                    .clause_slots(c.clause)
                    .iter()
                    .enumerate()
                    .filter(|&(_, &(_, free))| free)
                    .map(|(at, &(slot, _))| {
                        (slot, self.terms.vars[(c.vars.0 + at as u32) as usize])
                    })
                    .collect()
            };
            for (slot, var) in made {
                let (pat, copy) = self.terms.bound[var as usize];
                if pat == NONE {
                    continue;
                }
                let mut skolems = Vec::new();
                let mut stack = vec![(pat, copy)];
                // Bindings share terms: each part is visited once, and each
                // δ-occurrence in a copy gives one equation.
                let mut seen: HashSet<(u32, u32)> = HashSet::default();
                let mut deltas: HashSet<(u32, u32)> = HashSet::default();
                while let Some(t) = stack.pop() {
                    if !seen.insert(t) {
                        continue;
                    }
                    if seen.len().is_multiple_of(POLL as usize) && stop() {
                        return Err(NoUnifier::Stopped);
                    }
                    if let Head::App(f, (s, len), d) = self.terms.head(matrix, t) {
                        if let Symbol::Skolem(sk) = matrix.symbols[f as usize]
                            && deltas.insert((matrix.skolem_occ[sk as usize], d))
                        {
                            skolems.push((matrix.skolem_occ[sk as usize], d));
                        }
                        for &a in &matrix.pat_args[s as usize..(s + len) as usize] {
                            stack.push((a, d));
                        }
                    }
                }
                if skolems.is_empty() {
                    continue;
                }
                let gamma = matrix.slot_occ[slot as usize];
                let of_var = self.prefixes.of_occurrence(matrix, &self.terms, gamma, k);
                for (delta, d) in skolems {
                    let mut of_eigen = self.prefixes.of_occurrence(matrix, &self.terms, delta, d);
                    of_eigen.push(self.prefixes.fresh());
                    eqs.push((of_var.clone(), of_eigen));
                }
            }
        }
        if self.prefixes.exhausted() {
            return Ok(false);
        }
        let mut budget = UNIFY_STEPS;
        match unify(&eqs, AUXILIARY, &mut budget, &mut stop) {
            Ok(_) => Ok(true),
            Err(NoUnifier::Failed) => Ok(false),
            Err(NoUnifier::Stopped) => Err(NoUnifier::Stopped),
        }
    }

    /// Applies `way` to the literal of goal `g` and returns the goals
    /// after it, or `None` where it does not apply.
    fn apply(&mut self, g: u32, way: Way, marks: Marks, counts: &mut Counts) -> Option<u32> {
        let goal = self.goals[g as usize];
        let Item::Elem {
            node, copy, path, ..
        } = goal.item
        else {
            unreachable!("a literal goal is an element");
        };
        let Shape::Literal(lit) = self.matrix.nodes[node as usize].shape else {
            unreachable!("the node is a literal");
        };
        let literal = self.matrix.literals[lit as usize];
        let solved = Item::Solved {
            lit,
            copy,
            path,
            choices: (self.choices.len() - 1) as u32,
            mark: marks.terms,
        };
        match way {
            Way::Reduce(p) => {
                let n = self.paths[p as usize];
                let other = self.matrix.literals[n.lit as usize];
                if !self
                    .terms
                    .unify(self.matrix, (literal.args, copy), (other.args, n.copy))
                    || !self.connect((lit, copy), (n.lit, n.copy))
                {
                    return None;
                }
                counts.reductions += 1;
                self.used.push((lit, copy));
                self.used.push((n.lit, n.copy));
                Some(self.cons(solved, goal.next))
            }
            Way::Extend(target, clause, region) => {
                let depth = if path == NONE {
                    0
                } else {
                    self.paths[path as usize].depth
                };
                let fresh = self.renames[clause as usize]
                    || self
                        .matrix
                        .clause_slots(clause)
                        .iter()
                        .any(|&(_, free)| free);
                if fresh && depth >= self.limit {
                    self.limit_hit = true;
                    return None;
                }
                let (ext, _) = self.terms.copy(self.matrix, clause, region);
                let other = self.matrix.literals[target as usize];
                if !self
                    .terms
                    .unify(self.matrix, (literal.args, copy), (other.args, ext))
                    || !self.connect((lit, copy), (target, ext))
                {
                    return None;
                }
                counts.extensions += 1;
                self.used.push((lit, copy));
                self.used.push((target, ext));
                self.paths.push(PathNode {
                    lit,
                    copy,
                    parent: path,
                    depth: depth + 1,
                    ext,
                });
                let at = (self.paths.len() - 1) as u32;
                let rest = self.cons(solved, goal.next);
                // The β-clause of the copy with respect to the target: the
                // other elements of every clause from the target's up to the
                // extension clause.
                let mut subgoals = Vec::new();
                let mut below = other.node;
                let mut c = self.matrix.nodes[below as usize].parent;
                loop {
                    subgoals.extend(self.matrix.kids(c).iter().filter(|&&k| k != below));
                    if c == clause {
                        break;
                    }
                    below = self.matrix.nodes[c as usize].parent;
                    c = self.matrix.nodes[below as usize].parent;
                }
                Some(self.elements(&subgoals, ext, at, false, rest))
            }
        }
    }
}
