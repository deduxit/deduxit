// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The proof search of the non-clausal connection calculus (Otten,
//! TABLEAUX 2011, Figure 6; nanoCoP, IJCAR 2016, Figure 2): the start,
//! reduction, extension and decomposition rules, with positive start
//! clauses, regularity, lemmata, and iterative deepening on the length of
//! the active path.
//!
//! The search is depth first, on stacks of its own: the goals are a list
//! shared between the choices that made them, and a choice point records
//! how far every structure had grown, so that backtracking cuts each back
//! to it.
//!
//! **Copies.** A clause is copied by giving its free slots fresh
//! variables (Definition 6). The copy is laid beside the clause in the
//! region of a copy of a clause around it, and takes that copy's
//! variables for its other slots. An extension clause is one every path
//! through the active path enters (Definition 8): its parent matrix in
//! that region holds a literal of the path, or it is a clause of the
//! sequent's matrix itself. In a matrix without slots every copy is the
//! clause, and the condition is read on the matrix's own positions, as the
//! calculus has it; the search there is the calculus's whole search, so an
//! exhausted one shows the sequent not valid.

use super::matrix::{Matrix, NONE, Shape};
use super::terms::{Mark, Terms};
use crate::Error;
use crate::limits::{Limits, Phase, Progress};
use crate::linear::search::Reason;

/// How many steps the search takes between two asks of its stop.
const POLL: u64 = 1 << 10;

/// How the search runs.
#[derive(Clone, Copy, Debug)]
pub(super) struct Config {
    /// The longest active path an extension that makes a variable may
    /// lengthen; `None` deepens without end.
    pub(super) path_length: Option<u32>,
    /// Whether a literal, once solved, is never solved again another way
    /// (restricted backtracking, Otten, AI Communications 23, 2010). It
    /// loses completeness.
    pub(super) restricted: bool,
}

/// What the search counted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Counts {
    /// Steps taken: goals taken up and alternatives tried, the unit of
    /// work.
    pub(super) steps: u64,
    /// Extension steps made.
    pub(super) extensions: u64,
    /// Reduction steps made.
    pub(super) reductions: u64,
    /// Literals closed by a lemma.
    pub(super) lemmata: u64,
    /// The bound on the active path of the last round of deepening.
    pub(super) path_length: u32,
}

/// How a search ended.
#[derive(Debug)]
pub(super) enum End<F> {
    /// A proof: what the calculus made of it, the literals it used, each
    /// in its copy, and the terms with the substitution.
    Proved {
        /// What [`Calculus::proved`] made of the proof.
        found: F,
        /// The literals connected, or closed by a lemma, each with the copy
        /// it lies in.
        used: Vec<(u32, u32)>,
        /// The copies and the bindings.
        terms: Terms,
    },
    /// The whole search of a matrix without slots failed: the sequent is
    /// not valid.
    Exhausted,
    /// No proof, for the reason given.
    Unknown(Reason),
}

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
struct Marks<M> {
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
    /// The extension clauses computed. A cursor's are computed before its
    /// choice point is made, so cutting back to that point keeps them.
    eclauses: u32,
    /// The start clause's copy.
    start: u32,
    /// The calculus's own state ([`Calculus::mark`]), nothing for classical
    /// logic, so that its choice points keep their size: the memory poll
    /// counts them by capacity.
    calculus: M,
}

// The memory poll counts the choice points by capacity, so a field the
// classical calculus does not need would move its refusals: its marks are
// the terms' and six counts.
const _: () = assert!(size_of::<Marks<()>>() == size_of::<Mark>() + 6 * size_of::<u32>());

/// A choice point.
#[derive(Clone, Copy, Debug)]
struct Choice<M> {
    /// The goal list whose first goal it was made for; unused for a start.
    goal: u32,
    /// What it tries next.
    alt: Alt,
    /// What backtracking cuts back to.
    marks: Marks<M>,
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

/// What a calculus adds to the classical search: the hooks the
/// intuitionistic prover's prefixes take (Otten, TABLEAUX 2017). Each hook
/// is called at one fixed point of a step, so that the steps and the work
/// counted are a function of the input under either calculus. The search
/// is instantiated once per calculus, so no dynamic call enters its loop.
pub(super) trait Calculus: Sized {
    /// What a proof whose paths all hold a connection becomes.
    type Found;
    /// What a literal is compared by, beside its terms, for regularity and
    /// lemmata.
    type Key: PartialEq;
    /// How far the calculus's own state has grown, for backtracking.
    type Mark: Copy + core::fmt::Debug;
    /// Whether a matrix without slots is searched by position, its copies
    /// the clauses, so that an exhausted search shows the sequent not
    /// valid, and whether a literal solved without binding an older
    /// variable is settled.
    const CLASSICAL: bool;
    /// Starts a round.
    fn round(&mut self);
    /// Returns the work the step just taken did beyond the step itself.
    fn work(&mut self) -> u64;
    /// Returns the length of the calculus's longest table.
    fn longest(&self) -> usize;
    /// Returns the bytes the calculus holds, by capacity.
    fn held_bytes(&self) -> usize;
    /// Returns how far the calculus's own state has grown.
    fn mark(&self) -> Self::Mark;
    /// Cuts the calculus's own state back to `mark`.
    fn restore(&mut self, mark: Self::Mark);
    /// Returns the key of literal `lit` read in `copy`.
    fn key(&mut self, matrix: &Matrix, terms: &Terms, lit: u32, copy: u32) -> Self::Key;
    /// Returns whether literals `a` and `b`, each in its copy, whose terms
    /// unify, may be connected.
    fn connect(&mut self, matrix: &Matrix, terms: &Terms, a: (u32, u32), b: (u32, u32)) -> bool;
    /// Returns whether a copy of clause `clause` renames anything beyond
    /// its free slots.
    fn renames(&self, clause: u32) -> bool;
    /// Returns what a proof found by the round `search` becomes, `None`
    /// where the search goes on, or why the round ends.
    fn proved(
        search: &mut Search<'_, '_, Self>,
        counts: &Counts,
    ) -> Result<Option<Self::Found>, Ended>;
}

/// Classical logic: the calculus as it is.
pub(super) struct Classical;

impl Calculus for Classical {
    type Found = ();
    type Key = ();
    type Mark = ();
    const CLASSICAL: bool = true;
    fn round(&mut self) {}
    fn work(&mut self) -> u64 {
        0
    }
    fn longest(&self) -> usize {
        0
    }
    fn held_bytes(&self) -> usize {
        0
    }
    fn mark(&self) {}
    fn restore(&mut self, (): ()) {}
    fn key(&mut self, _: &Matrix, _: &Terms, _: u32, _: u32) {}
    fn connect(&mut self, _: &Matrix, _: &Terms, _: (u32, u32), _: (u32, u32)) -> bool {
        true
    }
    fn renames(&self, _: u32) -> bool {
        false
    }
    fn proved(_: &mut Search<'_, '_, Self>, _: &Counts) -> Result<Option<()>, Ended> {
        Ok(Some(()))
    }
}

/// Why a round ended without a proof or a failure.
pub(super) enum Ended {
    /// A bound or the stop.
    Reason(Reason),
    /// The caller's error, from [`Calculus::proved`].
    Error(Error),
}

/// The state of one round of the search.
pub(super) struct Search<'m, 's, C: Calculus> {
    /// The matrix.
    pub(super) matrix: &'m Matrix,
    /// The bytes the matrix holds, which the memory bound counts.
    matrix_bytes: usize,
    /// The caller's limits.
    limits: &'s Limits,
    /// The caller's stop.
    pub(super) stop: &'s mut dyn FnMut(Progress) -> bool,
    /// What the calculus adds.
    pub(super) calculus: &'s mut C,
    /// When the stop and the bounds are asked next, and were last.
    clock: Clock,
    /// Why a poll ended the round, once one did.
    halted: Option<Reason>,
    /// The copies and the substitution.
    pub(super) terms: Terms,
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
    choices: Vec<Choice<C::Mark>>,
    /// The copy of the start clause.
    start: u32,
    /// The bound on the active path for steps that make a variable.
    limit: u32,
    /// Whether that bound refused a step.
    limit_hit: bool,
    /// Whether restricted backtracking is on.
    restricted: bool,
}

/// When the search asks its stop and its bounds: at a count of work, and
/// the count at the last ask. Work is a step and what
/// [`Calculus::work`] adds to it; classically the steps.
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

/// Searches for a connection proof of `matrix`, deepening the bound on
/// the active path from 1 (one round at 0 for a bound of 0), and asks
/// `stop` and the bounds every [`POLL`] units of work.
///
/// # Errors
///
/// What [`Calculus::proved`] answers.
pub(super) fn search<C: Calculus>(
    matrix: &Matrix,
    calculus: &mut C,
    config: Config,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
    counts: &mut Counts,
) -> Result<End<C::Found>, Error> {
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
        calculus.round();
        let mut round = Search {
            matrix,
            matrix_bytes,
            limits,
            stop: &mut *stop,
            calculus: &mut *calculus,
            clock,
            halted: None,
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
        let ran = round.run(counts);
        clock = round.clock;
        match ran {
            Ok(Some(found)) => {
                return Ok(End::Proved {
                    found,
                    used: round.used,
                    terms: round.terms,
                });
            }
            Ok(None) => {}
            Err(Ended::Reason(reason)) => return Ok(End::Unknown(reason)),
            Err(Ended::Error(error)) => return Err(error),
        }
        if !round.limit_hit {
            // Without slots this was the classical calculus's whole search;
            // with them the copies' regions are a choice the search does not
            // make in every way, so a failure may miss a proof. Prefixes
            // need copies even without quantifiers, so no round of theirs
            // is the calculus's whole search.
            return Ok(if C::CLASSICAL && matrix.is_ground() {
                End::Exhausted
            } else {
                End::Unknown(Reason::Incomplete)
            });
        }
        if config.path_length.is_some_and(|bound| limit >= bound) {
            return Ok(End::Unknown(Reason::PathLength { length: limit }));
        }
        limit = limit.saturating_add(1);
    }
}

impl<C: Calculus> Search<'_, '_, C> {
    /// Runs one round: what the calculus made of a proof, `None` for a
    /// failure, and why a bound, the stop or the calculus ended it.
    fn run(&mut self, counts: &mut Counts) -> Result<Option<C::Found>, Ended> {
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
                // Every path holds a connection: the calculus decides.
                if let Some(found) = C::proved(self, counts)? {
                    return Ok(Some(found));
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
                    let args = (self.matrix.literals[lit as usize].args, copy);
                    // A literal solved without binding an older variable has
                    // no other solution worth trying, classically; with
                    // prefixes another solution may be the one whose
                    // prefixes unify, so only restricted backtracking cuts.
                    let settled = self.restricted
                        || (C::CLASSICAL
                            && self.terms.ground(self.matrix, args)
                            && !self.terms.bound_older(mark, mark.var_count()));
                    if settled {
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
        let more = self.calculus.work();
        self.clock.work = self.clock.work.saturating_add(1 + more);
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
            self.calculus.longest(),
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
            + self.choices.capacity() * size_of::<Choice<C::Mark>>()
            + self.calculus.held_bytes()
    }

    /// Returns how far every structure has grown.
    fn marks(&self) -> Marks<C::Mark> {
        Marks {
            terms: self.terms.mark(),
            goals: self.goals.len() as u32,
            paths: self.paths.len() as u32,
            lemmas: self.lemmas.len() as u32,
            used: self.used.len() as u32,
            eclauses: self.eclauses.len() as u32,
            start: self.start,
            calculus: self.calculus.mark(),
        }
    }

    /// Cuts every structure back to `marks`.
    fn restore(&mut self, marks: Marks<C::Mark>) {
        self.terms.undo(marks.terms);
        self.goals.truncate(marks.goals as usize);
        self.paths.truncate(marks.paths as usize);
        self.lemmas.truncate(marks.lemmas as usize);
        self.used.truncate(marks.used as usize);
        self.eclauses.truncate(marks.eclauses as usize);
        self.start = marks.start;
        self.calculus.restore(marks.calculus);
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
        // Equal terms and an equal key: equal under every substitution.
        let key = self.calculus.key(self.matrix, &self.terms, lit, copy);
        let same = |search: &mut Self, other: u32, other_copy: u32| {
            let o = search.matrix.literals[other as usize];
            o.predicate == literal.predicate
                && o.left == literal.left
                && search
                    .terms
                    .equal(search.matrix, (literal.args, copy), (o.args, other_copy))
                && search
                    .calculus
                    .key(search.matrix, &search.terms, other, other_copy)
                    == key
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
    /// of literal `lit`, in the region of copy `copy`: classically by
    /// position alone in a matrix without slots, whose copies are the
    /// clauses (copies that rename prefixes are copies of their own).
    fn in_clause_of(&self, lit: u32, copy: u32, (node, at): (u32, u32)) -> bool {
        let clause = self.matrix.nodes[self.matrix.literals[lit as usize].node as usize].parent;
        if C::CLASSICAL && self.matrix.is_ground() {
            self.matrix.within(node, clause)
        } else {
            self.inside(node, at, clause, copy)
        }
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
    /// only where it frees more slots than the last one kept (or renames
    /// what the last one did not), since otherwise its subgoals are the
    /// smaller one's and more; classically, in a matrix without slots only
    /// the smallest is kept.
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
                    + usize::from(self.calculus.renames(clause));
                if freed.is_none_or(|f| count > f) {
                    self.eclauses.push((clause, region));
                    freed = Some(count);
                    if C::CLASSICAL && matrix.is_ground() {
                        break;
                    }
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
                if C::CLASSICAL && matrix.is_ground() {
                    return matrix.within(node, outer);
                }
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
        if C::CLASSICAL && matrix.is_ground() {
            return holds(0).then_some(0);
        }
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

    /// Applies `way` to the literal of goal `g` and returns the goals
    /// after it, or `None` where it does not apply.
    fn apply(
        &mut self,
        g: u32,
        way: Way,
        marks: Marks<C::Mark>,
        counts: &mut Counts,
    ) -> Option<u32> {
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
                    || !self.calculus.connect(
                        self.matrix,
                        &self.terms,
                        (lit, copy),
                        (n.lit, n.copy),
                    )
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
                let fresh = self.calculus.renames(clause)
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
                    || !self
                        .calculus
                        .connect(self.matrix, &self.terms, (lit, copy), (target, ext))
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
