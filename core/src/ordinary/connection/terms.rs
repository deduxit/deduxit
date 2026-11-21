// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The terms of the search: a pattern of the matrix read in a copy of a
//! clause, whose variables the copy holds, and the rigid substitution
//! that unification builds.
//!
//! A term is shared, never copied: a variable is bound to a pattern and
//! the copy it is read in. Every binding is trailed, so that backtracking
//! takes it back. Every walk over a term keeps a stack of its own, since a
//! term may be deeper than the call stack allows, and visits each shared
//! part once ([`Seen`]): bindings share terms, so a term read as a tree can
//! be exponentially larger than the terms it is made of.

use super::matrix::{Matrix, NONE, Pat};
use crate::hash::HashSet;

/// The parts a walk has visited, kept once it has visited more than
/// [`Seen::SMALL`]: a small term costs no set, a shared one is walked as
/// the graph it is.
pub(super) struct Seen<K> {
    /// The parts visited so far.
    visits: usize,
    /// The parts visited, once there were more than [`Seen::SMALL`].
    set: Option<HashSet<K>>,
}

impl<K: std::hash::Hash + Eq> Seen<K> {
    /// The visits a walk makes before it keeps a set.
    const SMALL: usize = 64;

    /// A walk that has visited nothing.
    pub(super) const fn new() -> Self {
        Self {
            visits: 0,
            set: None,
        }
    }

    /// Returns whether `key` is visited for the first time. A walk that
    /// skips a part visited before sees the same constraint twice, which
    /// the first visit already handles.
    pub(super) fn first(&mut self, key: K) -> bool {
        self.visits += 1;
        if self.visits <= Self::SMALL {
            return true;
        }
        self.set.get_or_insert_with(HashSet::default).insert(key)
    }
}

/// A copy of a clause: where it lies and the variables of its slots.
#[derive(Clone, Copy, Debug)]
pub(super) struct Copy {
    /// The clause copied, [`NONE`] for the matrix itself, whose variables
    /// are every slot's, in slot order.
    pub(super) clause: u32,
    /// The copy whose region it lies in, [`NONE`] for the matrix itself.
    pub(super) parent: u32,
    /// The variables of the clause's slots, in the order of
    /// [`Matrix::clause_slots`]: `vars.0 .. vars.0 + vars.1` of
    /// [`Terms::vars`].
    pub(super) vars: (u32, u32),
}

/// A term: a pattern read in a copy.
pub(super) type Term = (u32, u32);

/// What a term is once its bound variables are followed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Head {
    /// An unbound variable.
    Var(u32),
    /// A symbol applied to the arguments of a pattern, read in a copy.
    App(u32, (u32, u32), u32),
}

/// The copies, the variables and their bindings.
#[derive(Debug, Default)]
pub(super) struct Terms {
    /// The copies; copy 0 is the matrix itself.
    pub(super) copies: Vec<Copy>,
    /// The variables of every copy.
    pub(super) vars: Vec<u32>,
    /// Each variable's binding, a term, or `(NONE, NONE)` where unbound.
    pub(super) bound: Vec<Term>,
    /// The variables bound, in order.
    pub(super) trail: Vec<u32>,
}

/// How far the terms had grown: what backtracking cuts them back to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Mark {
    /// The number of copies.
    copies: u32,
    /// The length of [`Terms::vars`].
    vars: u32,
    /// The number of variables.
    bound: u32,
    /// The length of the trail.
    trail: u32,
}

impl Mark {
    /// Returns the number of variables made by then: a variable numbered
    /// below it is older than the mark.
    pub(super) const fn var_count(self) -> u32 {
        self.bound
    }
}

impl Terms {
    /// Returns the terms with copy 0, the matrix itself, a fresh variable
    /// per slot.
    pub(super) fn new(matrix: &Matrix) -> Self {
        let mut terms = Self::default();
        let slots = matrix.slot_occ.len() as u32;
        for _ in 0..slots {
            let v = terms.fresh();
            terms.vars.push(v);
        }
        terms.copies.push(Copy {
            clause: NONE,
            parent: NONE,
            vars: (0, slots),
        });
        terms
    }

    /// Returns a new unbound variable.
    fn fresh(&mut self) -> u32 {
        self.bound.push((NONE, NONE));
        (self.bound.len() - 1) as u32
    }

    /// Returns how far the terms have grown.
    pub(super) fn mark(&self) -> Mark {
        Mark {
            copies: self.copies.len() as u32,
            vars: self.vars.len() as u32,
            bound: self.bound.len() as u32,
            trail: self.trail.len() as u32,
        }
    }

    /// Cuts the terms back to `mark`, unbinding what was bound since.
    pub(super) fn undo(&mut self, mark: Mark) {
        for &v in &self.trail[mark.trail as usize..] {
            if (v as usize) < self.bound.len() {
                self.bound[v as usize] = (NONE, NONE);
            }
        }
        self.trail.truncate(mark.trail as usize);
        self.bound.truncate(mark.bound as usize);
        self.vars.truncate(mark.vars as usize);
        self.copies.truncate(mark.copies as usize);
    }

    /// Returns whether a variable older than `before` was bound since
    /// `mark`.
    pub(super) fn bound_older(&self, mark: Mark, before: u32) -> bool {
        self.trail[mark.trail as usize..]
            .iter()
            .any(|&v| v < before)
    }

    /// Makes a copy of clause `clause` in the region of `parent`: a fresh
    /// variable for every slot free in the clause, and for the others the
    /// variable `parent` has. Returns the copy and whether it made a
    /// variable.
    pub(super) fn copy(&mut self, matrix: &Matrix, clause: u32, parent: u32) -> (u32, bool) {
        let start = self.vars.len() as u32;
        let mut fresh = false;
        let slots = matrix.clause_slots(clause);
        for &(slot, free) in slots {
            let v = if free {
                fresh = true;
                self.fresh()
            } else {
                self.var(matrix, parent, slot)
            };
            self.vars.push(v);
        }
        self.copies.push(Copy {
            clause,
            parent,
            vars: (start, slots.len() as u32),
        });
        ((self.copies.len() - 1) as u32, fresh)
    }

    /// Returns the variable of `slot` in copy `copy`, whose clause has the
    /// slot.
    pub(super) fn var(&self, matrix: &Matrix, copy: u32, slot: u32) -> u32 {
        let c = self.copies[copy as usize];
        let at = if c.clause == NONE {
            slot
        } else {
            let slots = matrix.clause_slots(c.clause);
            match slots.binary_search_by_key(&slot, |&(s, _)| s) {
                Ok(at) => at as u32,
                Err(_) => unreachable!("a copy holds every slot of its literals"),
            }
        };
        self.vars[(c.vars.0 + at) as usize]
    }

    /// Returns what term `t` is once its bound variables are followed.
    pub(super) fn head(&self, matrix: &Matrix, (mut p, mut c): Term) -> Head {
        loop {
            match matrix.pats[p as usize] {
                Pat::Var(slot) => {
                    let v = self.var(matrix, c, slot);
                    let (q, d) = self.bound[v as usize];
                    if q == NONE {
                        return Head::Var(v);
                    }
                    (p, c) = (q, d);
                }
                Pat::App(f, args) => return Head::App(f, args, c),
            }
        }
    }

    /// Binds variable `v` to term `t`, trailed.
    fn bind(&mut self, v: u32, t: Term) {
        self.bound[v as usize] = t;
        self.trail.push(v);
    }

    /// Returns whether variable `v` may not be bound to term `t`: it occurs
    /// in `t`, or `t` holds an open witness of the arena
    /// ([`Matrix::opaque`]).
    fn occurs(&self, matrix: &Matrix, v: u32, t: Term) -> bool {
        let mut stack = vec![t];
        let mut seen = Seen::new();
        while let Some(u) = stack.pop() {
            match self.head(matrix, u) {
                Head::Var(w) => {
                    if w == v {
                        return true;
                    }
                }
                Head::App(f, (s, len), c) => {
                    if matrix.opaque[f as usize] {
                        return true;
                    }
                    if !seen.first((f, s, c)) {
                        continue;
                    }
                    for &a in &matrix.pat_args[s as usize..(s + len) as usize] {
                        stack.push((a, c));
                    }
                }
            }
        }
        false
    }

    /// Unifies the literal argument lists `xs` read in copy `cx` and `ys`
    /// read in copy `cy` ([`Matrix::lit_args`]), with the occurs check. On failure the bindings it made
    /// stay: the caller undoes them to its mark.
    pub(super) fn unify(
        &mut self,
        matrix: &Matrix,
        (xs, cx): ((u32, u32), u32),
        (ys, cy): ((u32, u32), u32),
    ) -> bool {
        let pairs = &matrix.lit_args;
        let mut stack: Vec<(Term, Term)> = (0..xs.1)
            .map(|k| {
                (
                    (pairs[(xs.0 + k) as usize], cx),
                    (pairs[(ys.0 + k) as usize], cy),
                )
            })
            .collect();
        let mut seen = Seen::new();
        while let Some((a, b)) = stack.pop() {
            match (self.head(matrix, a), self.head(matrix, b)) {
                (Head::Var(v), Head::Var(w)) if v == w => {}
                (Head::Var(v), _) => {
                    if self.occurs(matrix, v, b) {
                        return false;
                    }
                    self.bind(v, b);
                }
                (_, Head::Var(w)) => {
                    if self.occurs(matrix, w, a) {
                        return false;
                    }
                    self.bind(w, a);
                }
                (Head::App(f, (s, len), c), Head::App(g, (t, len2), d)) => {
                    if f != g || len != len2 {
                        return false;
                    }
                    if !seen.first(((s, c), (t, d))) {
                        continue;
                    }
                    for k in 0..len {
                        stack.push((
                            (matrix.pat_args[(s + k) as usize], c),
                            (matrix.pat_args[(t + k) as usize], d),
                        ));
                    }
                }
            }
        }
        true
    }

    /// Returns whether the literal argument lists `xs` in copy `cx` and
    /// `ys` in copy `cy` are equal under the bindings, binding nothing.
    pub(super) fn equal(
        &self,
        matrix: &Matrix,
        (xs, cx): ((u32, u32), u32),
        (ys, cy): ((u32, u32), u32),
    ) -> bool {
        let pairs = &matrix.lit_args;
        let mut stack: Vec<(Term, Term)> = (0..xs.1)
            .map(|k| {
                (
                    (pairs[(xs.0 + k) as usize], cx),
                    (pairs[(ys.0 + k) as usize], cy),
                )
            })
            .collect();
        let mut seen = Seen::new();
        while let Some((a, b)) = stack.pop() {
            match (self.head(matrix, a), self.head(matrix, b)) {
                (Head::Var(v), Head::Var(w)) => {
                    if v != w {
                        return false;
                    }
                }
                (Head::App(f, (s, len), c), Head::App(g, (t, len2), d)) => {
                    if f != g || len != len2 {
                        return false;
                    }
                    if !seen.first(((s, c), (t, d))) {
                        continue;
                    }
                    for k in 0..len {
                        stack.push((
                            (matrix.pat_args[(s + k) as usize], c),
                            (matrix.pat_args[(t + k) as usize], d),
                        ));
                    }
                }
                _ => return false,
            }
        }
        true
    }

    /// Returns whether the argument list `xs` in copy `cx` holds no unbound
    /// variable.
    pub(super) fn ground(&self, matrix: &Matrix, (xs, cx): ((u32, u32), u32)) -> bool {
        let mut stack: Vec<Term> = matrix.lit_args[xs.0 as usize..(xs.0 + xs.1) as usize]
            .iter()
            .map(|&p| (p, cx))
            .collect();
        let mut seen = Seen::new();
        while let Some(t) = stack.pop() {
            match self.head(matrix, t) {
                Head::Var(_) => return false,
                Head::App(_, (s, len), c) => {
                    if !seen.first((s, len, c)) {
                        continue;
                    }
                    for &a in &matrix.pat_args[s as usize..(s + len) as usize] {
                        stack.push((a, c));
                    }
                }
            }
        }
        true
    }

    /// Returns the length of the longest table: the search refuses to go
    /// on past the `u32` indices.
    pub(super) fn longest(&self) -> usize {
        self.copies
            .len()
            .max(self.vars.len())
            .max(self.bound.len())
            .max(self.trail.len())
    }

    /// Returns the bytes the terms hold, by capacity.
    pub(super) fn held_bytes(&self) -> usize {
        self.copies.capacity() * size_of::<Copy>()
            + self.vars.capacity() * size_of::<u32>()
            + self.bound.capacity() * size_of::<Term>()
            + self.trail.capacity() * size_of::<u32>()
    }
}
