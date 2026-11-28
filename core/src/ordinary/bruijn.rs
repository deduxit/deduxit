// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! What the transformations that move or remove a binder do to de Bruijn
//! indices: a formula lifted under another binder or lowered out of one,
//! an open term put in for a binder's variable, and a formula's free
//! variables.
//!
//! A variable is the number of binders between it and the one that binds
//! it. Inside a formula `A`, an index below the number of binders passed on
//! the way down is bound within `A`; one at or above it, less that number,
//! is a variable *free in `A`*, bound by a binder around `A`: its index
//! "outside" `A`. Every walk here maps those outside indices and keeps the
//! rest, on a stack of its own, each node and term made once per depth.

use super::instance::{Replace, Substitution};
use super::{Formulas, Node, NodeId};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
#[cfg(feature = "sat")]
use crate::hash::HashMap;
use crate::hash::HashSet;

/// The replacement of each closed term a table holds by its image there,
/// with what [`Formulas::replaced`] has made so far, kept across the
/// formulas it maps: each node and term by its depth.
#[cfg(feature = "sat")]
pub(super) struct Replaced<'m>(Substitution<'m>);

#[cfg(feature = "sat")]
impl<'m> Replaced<'m> {
    /// Returns the replacement by `image`, nothing made yet.
    pub(super) fn new(image: &'m HashMap<FoTermId, FoTermId>) -> Self {
        Self(Substitution::of(Replace::Table(image)))
    }
}

impl Formulas {
    /// Returns `node` with every variable free in it moved out past `by`
    /// more binders: what `node` reads as when a binder is put around it
    /// that it does not mention, `B` in `∀x. (A ∧ B)` made from
    /// `(∀x. A) ∧ B`.
    ///
    /// # Errors
    ///
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub(crate) fn lifted(&mut self, node: NodeId, by: u32) -> Result<NodeId, Error> {
        if by == 0 {
            return Ok(node);
        }
        self.mapped(node, Replace::Shift(i64::from(by)))
    }

    /// Returns `node`, which mentions no variable of outside index 0, with
    /// every variable free in it moved one binder in: what `B` reads as
    /// when the binder around it is taken away, `B` in `(∀x. A) ∧ B` made
    /// from `∀x. (A ∧ B)`. `None` where `node` mentions that variable.
    ///
    /// # Errors
    ///
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub(crate) fn lowered(&mut self, node: NodeId) -> Result<Option<NodeId>, Error> {
        if self.free_variables(node).first() == Some(&0) {
            return Ok(None);
        }
        self.mapped(node, Replace::Shift(-1)).map(Some)
    }

    /// Returns `body`, the operand of a binder, with the binder's variable
    /// replaced by `term`, an open term whose variables are read outside the
    /// binder, and every other variable free in `body` moved one binder in:
    /// `A[t/x]` for `∃x. A`, `t` a Skolem term over the variables free in
    /// `∃x. A`.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for a term of no table of the arena,
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub(crate) fn substituted(&mut self, body: NodeId, term: FoTermId) -> Result<NodeId, Error> {
        let len = self.tables().terms.len();
        if term.index() >= len {
            return Err(Error::IndexOutOfBounds {
                space: crate::limits::Space::Term,
                index: term.index(),
                len,
            });
        }
        self.mapped(body, Replace::Open(term))
    }

    /// Returns the outside indices of the variables free in `node`, in
    /// ascending order: `[0]` for `p(#0)`, none for a closed formula.
    pub(crate) fn free_variables(&self, node: NodeId) -> Vec<u32> {
        let tables = self.tables();
        let mut free: Vec<u32> = Vec::new();
        let mut seen: HashSet<(NodeId, u32)> = HashSet::default();
        let mut seen_terms: HashSet<(FoTermId, u32)> = HashSet::default();
        let mut stack = vec![(node, 0u32)];
        let mut terms: Vec<(FoTermId, u32)> = Vec::new();
        while let Some((n, depth)) = stack.pop() {
            if !seen.insert((n, depth)) {
                continue;
            }
            match self.nodes()[n.index()] {
                Node::Atom(a) => {
                    if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                        terms.extend(tables.list(args).iter().map(|&t| (t, depth)));
                    }
                }
                Node::True | Node::False => {}
                Node::Not(a) => stack.push((a, depth)),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    stack.push((a, depth));
                    stack.push((b, depth));
                }
                Node::Forall(a) | Node::Exists(a) => stack.push((a, depth + 1)),
            }
        }
        while let Some((t, depth)) = terms.pop() {
            if !seen_terms.insert((t, depth)) {
                continue;
            }
            match tables.terms[t.index()] {
                FoTerm::Bound(i) if i >= depth => free.push(i - depth),
                FoTerm::Bound(_) | FoTerm::Eigen(_) | FoTerm::Meta(_) => {}
                FoTerm::App(_, args) => terms.extend(tables.list(args).iter().map(|&u| (u, depth))),
            }
        }
        free.sort_unstable();
        free.dedup();
        free
    }

    /// Returns `node` added to the arena, a binder made new taking the name
    /// of the binder `like`, so that a formula rebuilt prints with the
    /// names it was written with.
    ///
    /// # Errors
    ///
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub(crate) fn add_like(&mut self, node: Node, like: NodeId) -> Result<NodeId, Error> {
        let name = self.tables().binder_name(like).map(str::to_owned);
        self.add_named(node, name.as_deref())
    }

    /// Returns `node` added to the arena, a binder made new taking `name`
    /// where one is given.
    ///
    /// # Errors
    ///
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub(crate) fn add_named(&mut self, node: Node, name: Option<&str>) -> Result<NodeId, Error> {
        let len = self.len();
        let id = self.add(node)?;
        if self.len() > len
            && matches!(node, Node::Forall(_) | Node::Exists(_))
            && let Some(name) = name
            && let Some(entry) = self.interner().tables.binders.last_mut()
        {
            name.clone_into(&mut entry.1);
        }
        Ok(id)
    }

    /// Returns `root` with each closed term that `made`'s table holds
    /// replaced by its image there, `made` keeping what earlier calls made:
    /// a Skolem term turned back into its eigenvariable.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for a node of no arena,
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    #[cfg(feature = "sat")]
    pub(super) fn replaced(
        &mut self,
        root: NodeId,
        made: &mut Replaced<'_>,
    ) -> Result<NodeId, Error> {
        self.holds(root)?;
        made.0.build(self, root)
    }

    /// Returns the closed term `t` with each term that `made`'s table holds
    /// replaced by its image there, as [`replaced`](Self::replaced) does.
    #[cfg(feature = "sat")]
    pub(super) fn replaced_term(&mut self, t: FoTermId, made: &mut Replaced<'_>) -> FoTermId {
        made.0.build_term(self, t, 0)
    }

    /// Returns `root` as the operand of a new binder whose variable stands
    /// for each of `terms`, terms read outside `root`: `A[x/t]` for
    /// `∃x. A[x/t]` made from `A`, the variables free in `root` moved out
    /// past the binder.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for a node of no arena,
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub(crate) fn abstracted(&mut self, root: NodeId, terms: &[FoTermId]) -> Result<NodeId, Error> {
        self.mapped(root, Replace::Abstract(terms))
    }

    /// Returns `root` with every variable free in it mapped by `map`, by
    /// the one substitution walk of the crate (`instance.rs`).
    fn mapped(&mut self, root: NodeId, map: Replace<'_>) -> Result<NodeId, Error> {
        self.holds(root)?;
        Substitution::of(map).build(self, root)
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::super::{Formulas, Node, NodeId, Sequent};
    use crate::first_order::FoTerm;

    /// Returns the sequent of `text` and its first formula right of `⊢`.
    fn formula(text: &str) -> (Formulas, NodeId) {
        let sequent: Sequent = text.parse().unwrap();
        (sequent.formulas().clone(), sequent.right()[0])
    }

    /// Returns the operand of the binder `id`.
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "a test's every other node is a failure"
    )]
    fn body(formulas: &Formulas, id: NodeId) -> NodeId {
        match formulas.node(id) {
            Node::Forall(a) | Node::Exists(a) => a,
            node => panic!("no binder: {node:?}"),
        }
    }

    /// Lifting and lowering move only the variables free in a formula, and
    /// lowering refuses a formula that mentions the binder taken away.
    #[test]
    fn shifts_move_the_free_variables_alone() {
        // ∀x ∀y. (p(x) ∧ ∀z. q(y, z)): inside `∀y`, `p(x)` reads `x` as 1.
        let (mut formulas, root) = formula("|- forall x y. (p(x) /\\ forall z. q(y, z))");
        let inner = body(&formulas, body(&formulas, root));
        let Node::And(p, q) = formulas.node(inner) else {
            panic!("a conjunction")
        };
        assert_eq!(formulas.free_variables(p), [1]);
        assert_eq!(formulas.free_variables(q), [0]);
        assert_eq!(formulas.free_variables(inner), [0, 1]);
        assert_eq!(formulas.free_variables(root), Vec::<u32>::new());
        // `p(x)` mentions no variable 0, so it lowers to `p(#0)`, which
        // lifts back to itself.
        let low = formulas.lowered(p).unwrap().unwrap();
        assert_eq!(formulas.free_variables(low), [0]);
        assert_eq!(formulas.lifted(low, 1).unwrap(), p);
        // `∀z. q(y, z)` mentions variable 0 and does not lower.
        assert_eq!(formulas.lowered(q).unwrap(), None);
        // A variable bound inside stays: `∀z. q(#0, z)` lifted reads
        // `∀z. q(#1, z)`, the `z` still its own binder's.
        let lifted = formulas.lifted(q, 1).unwrap();
        assert_eq!(formulas.free_variables(lifted), [1]);
    }

    /// A Skolem term put in for an `∃`'s variable moves out past the
    /// binders inside, and the variables free in the body move one in.
    #[test]
    fn an_open_term_is_put_in_at_every_depth() {
        // ∀x. ∃y. (r(x, y) ∧ ∀z. s(y, z)), the `∃` replaced by `f(x)`.
        let (mut formulas, root) =
            formula("|- forall x. exists y. (r(x, y) /\\ forall z. s(y, z))");
        let exists = body(&formulas, root);
        let matrix = body(&formulas, exists);
        let x = formulas.variable(0);
        let f = formulas.function("f", &[x]).unwrap();
        let instance = formulas.substituted(matrix, f).unwrap();
        let wanted: Sequent = "|- forall x. (r(x, f(x)) /\\ forall z. s(f(x), z))"
            .parse()
            .unwrap();
        let mut copy = formulas.clone();
        let rebuilt = copy.add(Node::Forall(instance)).unwrap();
        assert_eq!(
            copy.formula(rebuilt).to_string(),
            wanted.formulas().formula(wanted.right()[0]).to_string()
        );
        // The term itself holds `#0`; under `∀z` it reads `#1`.
        assert!(matches!(
            formulas.first_order().terms[f.index()],
            FoTerm::App(..)
        ));
        assert_eq!(formulas.free_variables(instance), [0]);
    }
}
