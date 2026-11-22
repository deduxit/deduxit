// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Unification of closed first-order terms with metavariables, as an
//! interactive session binds its open witnesses.
//!
//! A metavariable has a level, the eigenvariables made before it; a binding
//! may name only eigenvariables below the level, or constants. Every
//! binding and every lowering of a level is trailed with the number of the
//! step that made it, so that undoing a step takes them back. Every walk
//! over a term keeps a stack of its own, since a term may be deeper than
//! the call stack allows.

use crate::first_order::{Args, Eigen, FirstOrder, FoTerm, FoTermId};
use std::collections::HashSet;

/// A store of terms the unifier reads: each term by its id, and each
/// argument list by its id.
pub(crate) trait Terms {
    /// Returns the term `t`.
    fn term(&self, t: FoTermId) -> FoTerm;

    /// Returns the terms of the argument list `args`.
    fn list(&self, args: Args) -> &[FoTermId];
}

impl<K> Terms for FirstOrder<K> {
    fn term(&self, t: FoTermId) -> FoTerm {
        self.terms[t.index()]
    }

    fn list(&self, args: Args) -> &[FoTermId] {
        Self::list(self, args)
    }
}

impl Terms for crate::linear::occurrences::instances::Terms<'_> {
    fn term(&self, t: FoTermId) -> FoTerm {
        Self::term(self, t)
    }

    fn list(&self, args: Args) -> &[FoTermId] {
        Self::list(self, args)
    }
}

/// A metavariable: the witness of an `∃` step left open.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Meta {
    /// Its term in the store.
    pub(crate) term: FoTermId,
    /// How many eigenvariables were made before it. A binding may name only
    /// eigenvariables numbered below its level, or constants. Binding
    /// another metavariable to a term this one occurs in lowers this level
    /// to that one's, when it is higher.
    ///
    /// The reason: an eigenvariable made later belongs to a `∀` step above
    /// the `∃` step that made the metavariable, or on another branch. In
    /// the witness it would stand in that `∀` step's conclusion, or outside
    /// its subtree.
    pub(crate) level: u32,
    /// Its level when it was made, before any binding lowered it.
    pub(crate) made: u32,
    /// What a later axiom or the search bound it to.
    pub(crate) bound: Option<FoTermId>,
}

/// A change a step made that its undo takes back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Trailed {
    /// A metavariable was bound.
    Bound(u32),
    /// A metavariable's level was lowered from the level given.
    Lowered(u32, u32),
}

/// Why two terms do not unify.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Clash {
    /// The symbols differ, or a metavariable would occur in its own
    /// binding.
    Terms,
    /// A binding would name an eigenvariable made after the metavariable.
    Eigen(Eigen),
}

/// The metavariables of a session, what they are bound to, and the trail
/// of bindings and lowerings that undo takes back.
#[derive(Clone, Debug, Default)]
pub(crate) struct Unifier {
    /// The metavariables, by number.
    metas: Vec<Meta>,
    /// The bindings and lowerings made, in order, each with the number of
    /// the step that made it.
    trail: Vec<(u32, Trailed)>,
}

impl Unifier {
    /// Returns a unifier with no metavariable.
    pub(crate) const fn new() -> Self {
        Self {
            metas: Vec::new(),
            trail: Vec::new(),
        }
    }

    /// Returns the metavariables, by number.
    pub(crate) fn metas(&self) -> &[Meta] {
        &self.metas
    }

    /// Returns the number the next metavariable made gets.
    pub(crate) fn next_meta(&self) -> u32 {
        // The cast is exact: each metavariable is a term of its store,
        // which a `u32` numbers.
        self.metas.len() as u32
    }

    /// Makes an unbound metavariable whose term is `term`, at `level`, and
    /// returns its number.
    pub(crate) fn push_meta(&mut self, term: FoTermId, level: u32) -> u32 {
        let k = self.next_meta();
        self.metas.push(Meta {
            term,
            level,
            made: level,
            bound: None,
        });
        k
    }

    /// Drops the metavariables past the first `len`.
    #[cfg(feature = "interactive")]
    pub(crate) fn truncate_metas(&mut self, len: usize) {
        self.metas.truncate(len);
    }

    /// Renumbers every metavariable's term and binding by `term`, for a
    /// store renumbered.
    #[cfg(feature = "interactive")]
    pub(crate) fn renumber(&mut self, term: impl Fn(FoTermId) -> FoTermId) {
        for meta in &mut self.metas {
            meta.term = term(meta.term);
            meta.bound = meta.bound.map(&term);
        }
    }

    /// Returns the trail: the bindings and lowerings made, in order, each
    /// with the number of the step that made it.
    #[cfg(all(feature = "serialize", feature = "interactive"))]
    pub(crate) fn trail(&self) -> &[(u32, Trailed)] {
        &self.trail
    }

    /// Returns the trail's length, for [`undo_to`](Self::undo_to).
    #[cfg(feature = "interactive")]
    pub(crate) fn trail_len(&self) -> usize {
        self.trail.len()
    }

    /// Returns how many entries of the trail the steps before `step` made.
    #[cfg(feature = "interactive")]
    pub(crate) fn trail_before(&self, step: u32) -> usize {
        self.trail.partition_point(|&(s, _)| s < step)
    }

    /// Returns `t` after following the bindings of the metavariables at its
    /// top.
    pub(crate) fn resolve(&self, terms: &impl Terms, mut t: FoTermId) -> FoTermId {
        while let FoTerm::Meta(k) = terms.term(t)
            && let Some(b) = self.metas[k as usize].bound
        {
            t = b;
        }
        t
    }

    /// Unifies the pairs of closed terms, binding metavariables under the
    /// occurs check and the levels; `constant` says which eigenvariables
    /// are constants. The bindings are trailed under `step`. On a clash
    /// nothing is left bound.
    pub(crate) fn unify(
        &mut self,
        terms: &impl Terms,
        pairs: Vec<(FoTermId, FoTermId)>,
        step: u32,
        constant: &impl Fn(Eigen) -> bool,
    ) -> Result<(), Clash> {
        let mark = self.trail.len();
        let result = self.unify_pairs(terms, pairs, step, constant);
        if result.is_err() {
            self.undo_to(mark);
        }
        result
    }

    /// Unifies the pairs, leaving what it bound before a clash.
    fn unify_pairs(
        &mut self,
        terms: &impl Terms,
        mut pairs: Vec<(FoTermId, FoTermId)>,
        step: u32,
        constant: &impl Fn(Eigen) -> bool,
    ) -> Result<(), Clash> {
        // A pair met before is unified already. So shared terms are walked
        // once, not once per path to them, which would take exponential
        // time on terms that share their halves.
        let mut met: HashSet<(FoTermId, FoTermId)> = HashSet::default();
        while let Some((x, y)) = pairs.pop() {
            let (x, y) = (self.resolve(terms, x), self.resolve(terms, y));
            if x == y || !met.insert((x, y)) {
                continue;
            }
            match (terms.term(x), terms.term(y)) {
                (FoTerm::Meta(k), _) => self.bind(terms, k, y, step, constant)?,
                (_, FoTerm::Meta(k)) => self.bind(terms, k, x, step, constant)?,
                (FoTerm::App(f, a), FoTerm::App(g, b)) if f == g => {
                    pairs.extend(
                        terms
                            .list(a)
                            .iter()
                            .copied()
                            .zip(terms.list(b).iter().copied()),
                    );
                }
                // Distinct eigenvariables, an eigenvariable against an
                // application, or two symbols.
                _ => return Err(Clash::Terms),
            }
        }
        Ok(())
    }

    /// Binds the unbound metavariable `k` to `t`, and lowers the levels of
    /// the metavariables `t` holds to `k`'s. The changes are trailed under
    /// `step`.
    ///
    /// It refuses a `t` that holds `k` itself (the occurs check), a bound
    /// variable, or an eigenvariable at or above `k`'s level that is not a
    /// constant, as `constant` says.
    pub(crate) fn bind(
        &mut self,
        terms: &impl Terms,
        k: u32,
        t: FoTermId,
        step: u32,
        constant: &impl Fn(Eigen) -> bool,
    ) -> Result<(), Clash> {
        let level = self.metas[k as usize].level;
        let mut lowered = Vec::new();
        let mut seen: HashSet<FoTermId> = HashSet::default();
        let mut stack = vec![t];
        while let Some(u) = stack.pop() {
            let u = self.resolve(terms, u);
            if !seen.insert(u) {
                continue;
            }
            match terms.term(u) {
                FoTerm::Meta(j) if j == k => return Err(Clash::Terms),
                FoTerm::Meta(j) => {
                    if self.metas[j as usize].level > level {
                        lowered.push(j);
                    }
                }
                FoTerm::Eigen(a) => {
                    if a.get() >= level && !constant(a) {
                        return Err(Clash::Eigen(a));
                    }
                }
                FoTerm::App(_, args) => stack.extend_from_slice(terms.list(args)),
                // A metavariable stands for a closed term. An ordinary
                // session unifies two formulas, whose atoms under a binder
                // hold its variable; the linear session's terms are closed.
                FoTerm::Bound(_) => return Err(Clash::Terms),
            }
        }
        for j in lowered {
            let meta = &mut self.metas[j as usize];
            self.trail.push((step, Trailed::Lowered(j, meta.level)));
            meta.level = level;
        }
        self.metas[k as usize].bound = Some(t);
        self.trail.push((step, Trailed::Bound(k)));
        Ok(())
    }

    /// Takes back what the steps numbered `step` and later bound and
    /// lowered.
    #[cfg(feature = "interactive")]
    pub(crate) fn undo(&mut self, step: u32) {
        self.undo_to(self.trail_before(step));
    }

    /// Takes back the trail past its first `len` entries, last first.
    pub(crate) fn undo_to(&mut self, len: usize) {
        while self.trail.len() > len {
            let (_, trailed) = self.trail.pop().expect("longer than len");
            match trailed {
                Trailed::Bound(k) => self.metas[k as usize].bound = None,
                Trailed::Lowered(k, level) => self.metas[k as usize].level = level,
            }
        }
    }

    /// Returns the bindings in force in the order made, each as the
    /// metavariable, its term and the step that made it.
    #[cfg(all(feature = "serialize", feature = "interactive"))]
    pub(crate) fn bindings(&self) -> impl Iterator<Item = (u32, FoTermId, u32)> + '_ {
        self.trail
            .iter()
            .filter_map(|&(step, trailed)| match trailed {
                Trailed::Bound(k) => Some((
                    k,
                    self.metas[k as usize]
                        .bound
                        .expect("a trailed binding holds"),
                    step,
                )),
                Trailed::Lowered(..) => None,
            })
    }
}
