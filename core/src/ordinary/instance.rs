// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The instances of a quantified formula: `A[t/x]`, the body `A` of
//! `∀x. A` or `∃x. A` with its bound variable replaced by a closed term
//! `t`. An instance is nodes of the arena like any formula, so a checker
//! compares it with a premise's formulas by id.

#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the checker's quantifier rules are the first caller outside the tests"
    )
)]

use super::{Formulas, Node, NodeId};
use crate::Error;
use crate::first_order::{Args, FoTerm, FoTermId, Symbol};
use crate::hash::HashMap;

impl Formulas {
    /// Returns `body` with the variable of the binder around it replaced
    /// by the closed term `term`: the instance `A[t/x]` of `∀x. A` or
    /// `∃x. A`, built as nodes of the arena where new. A binder built here
    /// keeps the name of the binder it copies.
    ///
    /// `body` is the operand of a binder of a closed formula, so no de
    /// Bruijn index needs a shift beyond the one the removed binder makes.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for an id of another arena,
    /// [`Error::LooseVariable`] for a term with a bound variable, which
    /// the instance would capture, and
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub(crate) fn instantiate(&mut self, body: NodeId, term: FoTermId) -> Result<NodeId, Error> {
        self.holds(body)?;
        if term.index() >= self.tables().terms.len() {
            return Err(Error::IndexOutOfBounds {
                space: crate::limits::Space::Term,
                index: term.index(),
                len: self.tables().terms.len(),
            });
        }
        if !self.is_closed(term) {
            return Err(Error::LooseVariable { term: term.index() });
        }
        let made = Substitution::new(term).run(&mut Build(self), body)?;
        Ok(made.expect("an arena that adds what it lacks finds every node"))
    }

    /// Returns the instance `A[t/x]` that [`instantiate`](Self::instantiate)
    /// builds, if the arena holds it, and adds nothing. A checker, which
    /// builds nothing, finds the instance a right inference's premise
    /// holds.
    pub(crate) fn instance(&self, body: NodeId, term: FoTermId) -> Option<NodeId> {
        if body.index() >= self.nodes.len()
            || term.index() >= self.tables().terms.len()
            || !self.is_closed(term)
        {
            return None;
        }
        Substitution::new(term)
            .run(&mut Find(self), body)
            .ok()
            .flatten()
    }
}

impl Formulas {
    /// Returns whether the term `t` has no bound variable. It walks on a
    /// stack of its own, each shared term once.
    fn is_closed(&self, t: FoTermId) -> bool {
        let tables = self.tables();
        let mut seen = vec![false; tables.terms.len()];
        let mut stack = vec![t];
        while let Some(u) = stack.pop() {
            if std::mem::replace(&mut seen[u.index()], true) {
                continue;
            }
            match tables.terms[u.index()] {
                FoTerm::Bound(_) => return false,
                FoTerm::App(_, args) => stack.extend_from_slice(tables.list(args)),
                FoTerm::Eigen(_) | FoTerm::Meta(_) => {}
            }
        }
        true
    }
}

/// An arena as a substitution makes its results in it: one that adds what
/// it lacks, or one that only looks up and answers `None` for a miss.
trait Maker {
    /// Returns the arena.
    fn formulas(&self) -> &Formulas;
    /// Returns the id of `node`; a binder made new takes the name of the
    /// binder `like`.
    fn node(&mut self, node: Node, like: NodeId) -> Result<Option<NodeId>, Error>;
    /// Returns the id of `term`.
    fn term(&mut self, term: FoTerm) -> Option<FoTermId>;
    /// Returns the argument list of `items`.
    fn list(&mut self, items: &[FoTermId]) -> Option<Args>;
    /// Returns the atom of `symbol` applied to `args`.
    fn atom(&mut self, symbol: Symbol, args: Args) -> Result<Option<NodeId>, Error>;
}

/// An arena that adds what it lacks.
struct Build<'a>(&'a mut Formulas);

impl Maker for Build<'_> {
    fn formulas(&self) -> &Formulas {
        self.0
    }

    fn node(&mut self, node: Node, like: NodeId) -> Result<Option<NodeId>, Error> {
        let len = self.0.nodes.len();
        let id = self.0.add(node)?;
        if self.0.nodes.len() > len
            && matches!(node, Node::Forall(_) | Node::Exists(_))
            && let Some(name) = self.0.tables().binder_name(like)
        {
            let name = name.to_owned();
            if let Some(entry) = self.0.interner().tables.binders.last_mut() {
                entry.1 = name;
            }
        }
        Ok(Some(id))
    }

    fn term(&mut self, term: FoTerm) -> Option<FoTermId> {
        Some(self.0.interner().term(term))
    }

    fn list(&mut self, items: &[FoTermId]) -> Option<Args> {
        Some(self.0.interner().list(items))
    }

    fn atom(&mut self, symbol: Symbol, args: Args) -> Result<Option<NodeId>, Error> {
        self.0.predicate_atom(symbol, args).map(Some)
    }
}

/// An arena that only looks up.
struct Find<'a>(&'a Formulas);

impl Maker for Find<'_> {
    fn formulas(&self) -> &Formulas {
        self.0
    }

    fn node(&mut self, node: Node, _like: NodeId) -> Result<Option<NodeId>, Error> {
        Ok(self.0.find(node))
    }

    fn term(&mut self, term: FoTerm) -> Option<FoTermId> {
        self.0.fo.as_deref()?.find_term(term)
    }

    fn list(&mut self, items: &[FoTermId]) -> Option<Args> {
        self.0.fo.as_deref()?.find_list(items)
    }

    fn atom(&mut self, symbol: Symbol, args: Args) -> Result<Option<NodeId>, Error> {
        let key = self.0.tables().atom_key(symbol, args);
        Ok(self
            .0
            .names
            .get(&key)
            .and_then(|&index| self.0.find(Node::Atom(index))))
    }
}

/// What the walk over a formula or a term has yet to do.
#[derive(Clone, Copy, Debug)]
enum Task<T> {
    /// Visit the operands of an entry under so many binders.
    Visit(T, u32),
    /// Make the entry's instance from its operands' instances.
    Make(T, u32),
}

/// The replacement of the variable of one binder by a closed term, at
/// every depth it is met, each node and term made once per depth.
struct Substitution {
    /// The closed term.
    term: FoTermId,
    /// Every node's instance under so many binders below the removed one.
    nodes: HashMap<(NodeId, u32), NodeId>,
    /// Every term's instance under so many binders below the removed one.
    terms: HashMap<(FoTermId, u32), FoTermId>,
}

impl Substitution {
    /// Returns the substitution of `term` for the removed binder's
    /// variable.
    fn new(term: FoTermId) -> Self {
        Self {
            term,
            nodes: HashMap::default(),
            terms: HashMap::default(),
        }
    }

    /// Returns the instance of `body`, or `None` where `maker` misses a
    /// node or a term on the way. It walks on a stack of its own.
    fn run(&mut self, maker: &mut impl Maker, body: NodeId) -> Result<Option<NodeId>, Error> {
        let mut stack = vec![Task::Visit(body, 0)];
        while let Some(task) = stack.pop() {
            match task {
                Task::Visit(n, depth) => {
                    if self.nodes.contains_key(&(n, depth)) {
                        continue;
                    }
                    stack.push(Task::Make(n, depth));
                    let node = maker.formulas().node(n);
                    let inner =
                        depth + u32::from(matches!(node, Node::Forall(_) | Node::Exists(_)));
                    let (a, b) = node.operands();
                    for k in [a, b].into_iter().flatten() {
                        stack.push(Task::Visit(k, inner));
                    }
                }
                Task::Make(n, depth) => {
                    if self.nodes.contains_key(&(n, depth)) {
                        continue;
                    }
                    let Some(made) = self.make(maker, n, depth)? else {
                        return Ok(None);
                    };
                    self.nodes.insert((n, depth), made);
                }
            }
        }
        Ok(self.nodes.get(&(body, 0)).copied())
    }

    /// Returns the instance of the node `n` under `depth` binders, its
    /// operands' instances made already.
    fn make(
        &mut self,
        maker: &mut impl Maker,
        n: NodeId,
        depth: u32,
    ) -> Result<Option<NodeId>, Error> {
        let at = |k: NodeId, d: u32| self.nodes[&(k, d)];
        let node = match maker.formulas().node(n) {
            Node::Atom(a) => {
                let Some(&(symbol, args)) = maker.formulas().tables().atoms.get(a as usize) else {
                    return Ok(Some(n));
                };
                let items: Vec<FoTermId> = maker.formulas().tables().list(args).to_vec();
                if items.is_empty() {
                    return Ok(Some(n));
                }
                let mut made = Vec::with_capacity(items.len());
                for t in items {
                    let Some(t) = self.substitute(maker, t, depth) else {
                        return Ok(None);
                    };
                    made.push(t);
                }
                let Some(args) = maker.list(&made) else {
                    return Ok(None);
                };
                return maker.atom(symbol, args);
            }
            Node::True => Node::True,
            Node::False => Node::False,
            Node::Not(a) => Node::Not(at(a, depth)),
            Node::And(a, b) => Node::And(at(a, depth), at(b, depth)),
            Node::Or(a, b) => Node::Or(at(a, depth), at(b, depth)),
            Node::Implies(a, b) => Node::Implies(at(a, depth), at(b, depth)),
            Node::Iff(a, b) => Node::Iff(at(a, depth), at(b, depth)),
            Node::Forall(a) => Node::Forall(at(a, depth + 1)),
            Node::Exists(a) => Node::Exists(at(a, depth + 1)),
        };
        maker.node(node, n)
    }

    /// Returns the instance of the term `t` under `depth` binders below
    /// the removed one: its variable, `depth`, is the closed term, and a
    /// variable of a binder further out moves one in. It walks on a stack
    /// of its own.
    fn substitute(&mut self, maker: &mut impl Maker, t: FoTermId, depth: u32) -> Option<FoTermId> {
        let mut stack = vec![Task::Visit(t, depth)];
        while let Some(task) = stack.pop() {
            match task {
                Task::Visit(u, d) => {
                    if self.terms.contains_key(&(u, d)) {
                        continue;
                    }
                    stack.push(Task::Make(u, d));
                    if let FoTerm::App(_, args) = maker.formulas().tables().terms[u.index()] {
                        for &v in maker.formulas().tables().list(args) {
                            stack.push(Task::Visit(v, d));
                        }
                    }
                }
                Task::Make(u, d) => {
                    if self.terms.contains_key(&(u, d)) {
                        continue;
                    }
                    let made = match maker.formulas().tables().terms[u.index()] {
                        FoTerm::Bound(i) if i == d => self.term,
                        FoTerm::Bound(i) if i > d => maker.term(FoTerm::Bound(i - 1))?,
                        FoTerm::Bound(_) | FoTerm::Eigen(_) | FoTerm::Meta(_) => u,
                        FoTerm::App(f, args) => {
                            let items: Vec<FoTermId> = maker
                                .formulas()
                                .tables()
                                .list(args)
                                .iter()
                                .map(|&v| self.terms[&(v, d)])
                                .collect();
                            let args = maker.list(&items)?;
                            maker.term(FoTerm::App(f, args))?
                        }
                    };
                    self.terms.insert((u, d), made);
                }
            }
        }
        self.terms.get(&(t, depth)).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Formulas, Node, NodeId};
    use crate::first_order::FoTermId;

    /// Returns an arena holding `body`, made by `make` from the variables
    /// `#0` and `#1` and the constant `a`, and `a`.
    fn arena(
        make: impl FnOnce(&mut Formulas, [FoTermId; 3]) -> NodeId,
    ) -> (Formulas, NodeId, FoTermId) {
        let mut formulas = Formulas::default();
        let terms = [
            formulas.variable(0),
            formulas.variable(1),
            formulas.function("a", &[]).unwrap(),
        ];
        let body = make(&mut formulas, terms);
        (formulas, body, terms[2])
    }

    #[test]
    fn an_instance_replaces_the_binder_s_variable_at_every_depth() {
        // The body of `∀x. (p(x) ∧ ∀y. (p(y) ∧ r(x, y)))`: `p(#0)` stands
        // for `p(x)` at depth 0 and for `p(y)` under the inner binder.
        let (mut formulas, body, a) = arena(|f, [v0, v1, _]| {
            let p0 = f.predicate("p", &[v0]).unwrap();
            let r = f.predicate("r", &[v1, v0]).unwrap();
            let inner = f.add(Node::And(p0, r)).unwrap();
            let all = f.binder(true, inner, "y").unwrap();
            f.add(Node::And(p0, all)).unwrap()
        });
        let before = formulas.clone();
        let instance = formulas.instantiate(body, a).unwrap();
        let mut expected = formulas.clone();
        let p0 = expected.predicate("p", &[FoTermId::new(0)]).unwrap();
        let pa = expected.predicate("p", &[a]).unwrap();
        let r = expected.predicate("r", &[a, FoTermId::new(0)]).unwrap();
        let inner = expected.add(Node::And(p0, r)).unwrap();
        let all = expected.binder(true, inner, "z").unwrap();
        let whole = expected.add(Node::And(pa, all)).unwrap();
        assert_eq!(instance, whole);
        assert_eq!(
            expected.len(),
            formulas.len(),
            "the instance was built whole"
        );
        // The binder built keeps the name of the one it copies.
        assert_eq!(formulas.first_order().binder_name(all), Some("y"));
        // The checker's form finds it once built, and only then.
        assert_eq!(formulas.instance(body, a), Some(instance));
        assert_eq!(before.instance(body, a), None);
    }

    #[test]
    fn a_term_inside_a_function_is_replaced_and_deep_bodies_cost_no_stack() {
        let (mut formulas, body, a) = arena(|f, [v0, _, _]| {
            let mut t = v0;
            for _ in 0..50_000 {
                t = f.function("f", &[t]).unwrap();
            }
            let mut n = f.predicate("p", &[t]).unwrap();
            for _ in 0..50_000 {
                n = f.add(Node::Not(n)).unwrap();
            }
            n
        });
        // 50 000 levels: a walk that recursed would overflow the test's
        // stack, on every target (no thread of a small stack, which WASI
        // cannot start).
        let instance = formulas.instantiate(body, a).unwrap();
        assert_eq!(formulas.instance(body, a), Some(instance));
        assert_ne!(instance, body);
    }

    #[test]
    fn a_term_with_a_bound_variable_is_no_witness() {
        let (mut formulas, body, _) = arena(|f, [v0, v1, _]| {
            let r = f.predicate("r", &[v1, v0]).unwrap();
            f.binder(false, r, "y").unwrap()
        });
        // `#0` put for `x` in `∃y. r(x, y)` would be captured by `y`.
        let bound = FoTermId::new(0);
        assert!(matches!(
            formulas.instantiate(body, bound),
            Err(crate::Error::LooseVariable { .. })
        ));
        assert_eq!(formulas.instance(body, bound), None);
    }
}
