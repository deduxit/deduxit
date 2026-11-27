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

use super::{Formulas, Node, NodeId};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::{HashMap, HashSet};

/// What a walk does with a variable free in the formula it walks.
#[derive(Clone, Copy, Debug)]
enum Map<'m> {
    /// The variable's outside index moves `by`: out past more binders where
    /// positive, in where negative. The walks that lower a formula check
    /// first that no index would fall below zero.
    Shift(i64),
    /// The variable of outside index 0 (the binder removed) is the term, an
    /// open term whose variables are read outside that binder; every other
    /// moves one binder in.
    Substitute(FoTermId),
    /// Each closed term the table holds is its image there; variables stay.
    #[cfg(feature = "sat")]
    Replace(&'m HashMap<FoTermId, FoTermId>),
    /// Each term the set holds, read outside the formula, is the variable
    /// of a new binder around it; every other variable moves out past it.
    Abstract(&'m [FoTermId]),
}

/// What a walk of [`Map::Replace`] has made so far, kept across the
/// formulas it maps: each node and term by its depth.
#[cfg(feature = "sat")]
#[derive(Debug, Default)]
pub(super) struct Replaced {
    /// The image of each node walked, by the binders around it.
    nodes: HashMap<(NodeId, u32), NodeId>,
    /// The image of each term walked, by the binders around it.
    terms: HashMap<(FoTermId, u32), FoTermId>,
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
        self.mapped(node, Map::Shift(i64::from(by)))
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
        self.mapped(node, Map::Shift(-1)).map(Some)
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
        self.mapped(body, Map::Substitute(term))
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

    /// Returns `root` with each closed term that `image` holds replaced
    /// by its image there, `made` keeping what earlier calls with the same
    /// `image` made: a Skolem term turned back into its eigenvariable.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for a node of no arena,
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    #[cfg(feature = "sat")]
    pub(super) fn replaced(
        &mut self,
        root: NodeId,
        image: &HashMap<FoTermId, FoTermId>,
        made: &mut Replaced,
    ) -> Result<NodeId, Error> {
        self.mapped_in(root, Map::Replace(image), &mut made.nodes, &mut made.terms)
    }

    /// Returns the closed term `t` with each term that `image` holds
    /// replaced by its image there, as [`replaced`](Self::replaced) does.
    #[cfg(feature = "sat")]
    pub(super) fn replaced_term(
        &mut self,
        t: FoTermId,
        image: &HashMap<FoTermId, FoTermId>,
        made: &mut Replaced,
    ) -> FoTermId {
        self.mapped_term(t, 0, Map::Replace(image), &mut made.terms)
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
        self.mapped(root, Map::Abstract(terms))
    }

    /// Returns `root` with every variable free in it mapped by `map`.
    fn mapped(&mut self, root: NodeId, map: Map<'_>) -> Result<NodeId, Error> {
        let mut nodes: HashMap<(NodeId, u32), NodeId> = HashMap::default();
        let mut terms: HashMap<(FoTermId, u32), FoTermId> = HashMap::default();
        self.mapped_in(root, map, &mut nodes, &mut terms)
    }

    /// [`mapped`](Self::mapped), with what is made kept in `nodes` and
    /// `terms`.
    fn mapped_in(
        &mut self,
        root: NodeId,
        map: Map<'_>,
        nodes: &mut HashMap<(NodeId, u32), NodeId>,
        terms: &mut HashMap<(FoTermId, u32), FoTermId>,
    ) -> Result<NodeId, Error> {
        self.holds(root)?;
        // Each node is visited, then made once its operands are.
        let mut stack = vec![(root, 0u32, false)];
        while let Some((n, depth, ready)) = stack.pop() {
            if nodes.contains_key(&(n, depth)) {
                continue;
            }
            let node = self.nodes()[n.index()];
            if !ready {
                stack.push((n, depth, true));
                let inner = match node {
                    Node::Forall(_) | Node::Exists(_) => depth + 1,
                    Node::Atom(_)
                    | Node::True
                    | Node::False
                    | Node::Not(_)
                    | Node::And(..)
                    | Node::Or(..)
                    | Node::Implies(..)
                    | Node::Iff(..) => depth,
                };
                let (a, b) = node.operands();
                stack.extend([a, b].into_iter().flatten().map(|k| (k, inner, false)));
                continue;
            }
            let at = |k: NodeId, d: u32| nodes[&(k, d)];
            let made = match node {
                Node::Atom(a) => {
                    let Some(&(symbol, args)) = self.tables().atoms.get(a as usize) else {
                        nodes.insert((n, depth), n);
                        continue;
                    };
                    let items = self.tables().list(args).to_vec();
                    let mut changed = Vec::with_capacity(items.len());
                    for t in items {
                        changed.push(self.mapped_term(t, depth, map, terms));
                    }
                    let args = self.interner().list(&changed);
                    self.predicate_atom(symbol, args)?
                }
                Node::True | Node::False => n,
                Node::Not(a) => self.add_like(Node::Not(at(a, depth)), n)?,
                Node::And(a, b) => self.add_like(Node::And(at(a, depth), at(b, depth)), n)?,
                Node::Or(a, b) => self.add_like(Node::Or(at(a, depth), at(b, depth)), n)?,
                Node::Implies(a, b) => {
                    self.add_like(Node::Implies(at(a, depth), at(b, depth)), n)?
                }
                Node::Iff(a, b) => self.add_like(Node::Iff(at(a, depth), at(b, depth)), n)?,
                Node::Forall(a) => self.add_like(Node::Forall(at(a, depth + 1)), n)?,
                Node::Exists(a) => self.add_like(Node::Exists(at(a, depth + 1)), n)?,
            };
            nodes.insert((n, depth), made);
        }
        Ok(nodes[&(root, 0)])
    }

    /// Returns the term `t`, under `depth` binders inside the formula
    /// walked, with every variable free in that formula mapped by `map`.
    fn mapped_term(
        &mut self,
        t: FoTermId,
        depth: u32,
        map: Map<'_>,
        made: &mut HashMap<(FoTermId, u32), FoTermId>,
    ) -> FoTermId {
        let mut stack = vec![(t, false)];
        while let Some((u, ready)) = stack.pop() {
            if made.contains_key(&(u, depth)) {
                continue;
            }
            let term = self.tables().terms[u.index()];
            if !ready {
                stack.push((u, true));
                if let FoTerm::App(_, args) = term {
                    let items = self.tables().list(args).to_vec();
                    stack.extend(items.into_iter().map(|v| (v, false)));
                }
                continue;
            }
            #[cfg(feature = "sat")]
            if let Map::Replace(image) = map
                && let Some(&to) = image.get(&u)
            {
                made.insert((u, depth), to);
                continue;
            }
            if let Map::Abstract(terms) = map
                && terms.iter().any(|&t| self.lifted_term(t, depth) == u)
            {
                let variable = self.interner().term(FoTerm::Bound(depth));
                made.insert((u, depth), variable);
                continue;
            }
            let image = match term {
                FoTerm::Bound(i) if i >= depth => {
                    let outside = i - depth;
                    match map {
                        Map::Shift(by) => {
                            let moved = i64::from(outside) + by;
                            let moved = u32::try_from(moved)
                                .unwrap_or_else(|_| unreachable!("a lowered formula is checked"));
                            self.interner().term(FoTerm::Bound(moved + depth))
                        }
                        Map::Substitute(term) if outside == 0 => {
                            // The term's variables are read outside the
                            // removed binder: under `depth` binders they
                            // move out past them.
                            self.lifted_term(term, depth)
                        }
                        Map::Substitute(_) => self.interner().term(FoTerm::Bound(i - 1)),
                        #[cfg(feature = "sat")]
                        Map::Replace(_) => u,
                        Map::Abstract(_) => self.interner().term(FoTerm::Bound(i + 1)),
                    }
                }
                FoTerm::Bound(_) | FoTerm::Eigen(_) | FoTerm::Meta(_) => u,
                FoTerm::App(f, args) => {
                    let items: Vec<FoTermId> = self
                        .tables()
                        .list(args)
                        .iter()
                        .map(|&v| made[&(v, depth)])
                        .collect();
                    let args = self.interner().list(&items);
                    self.interner().term(FoTerm::App(f, args))
                }
            };
            made.insert((u, depth), image);
        }
        made[&(t, depth)]
    }

    /// Returns the term `t` with each of its variables moved out past `by`
    /// binders.
    fn lifted_term(&mut self, t: FoTermId, by: u32) -> FoTermId {
        if by == 0 {
            return t;
        }
        let mut made = HashMap::default();
        self.mapped_term(t, 0, Map::Shift(i64::from(by)), &mut made)
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
