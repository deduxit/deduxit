// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The instances of a quantified formula: `A[t/x]`, the body `A` of
//! `∀x. A` or `∃x. A` with its bound variable replaced by a closed term
//! `t`. An instance is nodes of the arena like any formula, so a checker
//! compares it with a premise's formulas by id.

use super::{Formulas, Node, NodeId};
use crate::Error;
use crate::first_order::{Args, FirstOrder, FoTerm, FoTermId, Symbol};
use crate::hash::{HashMap, HashSet};

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
    #[cfg_attr(
        not(any(test, feature = "interactive")),
        expect(
            dead_code,
            reason = "the goal state's quantifier rules call it; the read-back puts in whole frames"
        )
    )]
    pub(crate) fn instantiate(&mut self, body: NodeId, term: FoTermId) -> Result<NodeId, Error> {
        self.instantiate_frame(body, &[term])
    }

    /// Returns `body` with the variables of the binders around it replaced
    /// by the closed `terms`, the innermost binder's first, in one pass:
    /// what [`instantiate`](Self::instantiate) builds term by term, but
    /// each node and atom made once, where term by term a frame of `n`
    /// terms over an atom of `n` arguments made `n` atoms of `n`
    /// arguments each.
    ///
    /// # Errors
    ///
    /// As [`instantiate`](Self::instantiate), for each term.
    pub(crate) fn instantiate_frame(
        &mut self,
        body: NodeId,
        terms: &[FoTermId],
    ) -> Result<NodeId, Error> {
        self.holds(body)?;
        for &term in terms {
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
        }
        if terms.is_empty() {
            return Ok(body);
        }
        let made = Substitution::of_frame(terms).run(&mut Build(self), body)?;
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
    /// Returns whether `term` is a term of the arena that is an
    /// eigenvariable.
    pub(crate) fn is_eigenvariable(&self, term: FoTermId) -> bool {
        matches!(
            self.tables().terms.get(term.index()),
            Some(FoTerm::Eigen(_))
        )
    }

    /// Returns whether the term `t` of the arena holds an open witness
    /// ([`FoTerm::Meta`]). It walks on a stack of its own, each shared
    /// term once.
    pub(crate) fn holds_meta(&self, t: FoTermId) -> bool {
        let tables = self.tables();
        if t.index() >= tables.terms.len() {
            return false;
        }
        let mut seen = HashSet::default();
        let mut stack = vec![t];
        while let Some(u) = stack.pop() {
            if !seen.insert(u) {
                continue;
            }
            match tables.terms[u.index()] {
                FoTerm::Meta(_) => return true,
                FoTerm::App(_, args) => stack.extend_from_slice(tables.list(args)),
                FoTerm::Bound(_) | FoTerm::Eigen(_) => {}
            }
        }
        false
    }

    /// Returns whether the term `term` occurs in one of the formulas
    /// `roots`, as an argument of an atom or inside one. It walks on
    /// stacks of its own, each shared node and term once.
    pub(crate) fn mentions(&self, roots: impl Iterator<Item = NodeId>, term: FoTermId) -> bool {
        let tables = self.tables();
        let mut nodes_seen = HashSet::default();
        let mut terms_seen = HashSet::default();
        let mut nodes: Vec<NodeId> = roots.collect();
        let mut terms: Vec<FoTermId> = Vec::new();
        while let Some(k) = nodes.pop() {
            if !nodes_seen.insert(k) {
                continue;
            }
            match self.nodes[k.index()] {
                Node::Atom(a) => {
                    if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                        terms.extend_from_slice(tables.list(args));
                    }
                }
                Node::True | Node::False => {}
                Node::Not(a) | Node::Forall(a) | Node::Exists(a) => nodes.push(a),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    nodes.push(a);
                    nodes.push(b);
                }
            }
        }
        while let Some(u) = terms.pop() {
            if u == term {
                return true;
            }
            if !terms_seen.insert(u) {
                continue;
            }
            if let FoTerm::App(_, args) = tables.terms[u.index()] {
                terms.extend_from_slice(tables.list(args));
            }
        }
        false
    }

    /// Returns the eigenvariables of the arena with the names they are
    /// written with, by ascending number: `a`, `b`, … `z`, then `a_1`,
    /// `b_1`, …, skipping the names of the arena's symbols, so that a
    /// sequent of a derivation reads as a sequent with its eigenvariables
    /// as new constants. A name goes by rank among the numbers the terms
    /// hold, not by the number, so a document's large numbers cost
    /// nothing; an eigenvariable made later takes a larger number and
    /// leaves the earlier names as they were.
    pub(crate) fn eigen_names(&self) -> Vec<(u32, String)> {
        let tables = self.tables();
        let mut numbers: Vec<u32> = tables
            .terms
            .iter()
            .filter_map(|term| match *term {
                FoTerm::Eigen(e) => Some(e.get()),
                FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => None,
            })
            .collect();
        numbers.sort_unstable();
        numbers.dedup();
        let names = eigen_names(tables, numbers.len());
        numbers.into_iter().zip(names).collect()
    }

    /// Returns the largest number of an eigenvariable that occurs in one
    /// of the formulas `roots` or in one of the terms `term`, or `None`
    /// where none does. It walks on stacks of its own, each shared node and term once.
    #[cfg(feature = "interactive")]
    pub(crate) fn largest_eigenvariable(
        &self,
        roots: impl Iterator<Item = NodeId>,
        term: impl Iterator<Item = FoTermId>,
    ) -> Option<u32> {
        let tables = self.tables();
        let mut nodes_seen = HashSet::default();
        let mut terms_seen = HashSet::default();
        let mut nodes: Vec<NodeId> = roots.collect();
        let mut terms: Vec<FoTermId> = term.collect();
        while let Some(k) = nodes.pop() {
            if !nodes_seen.insert(k) {
                continue;
            }
            match self.nodes[k.index()] {
                Node::Atom(a) => {
                    if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                        terms.extend_from_slice(tables.list(args));
                    }
                }
                Node::True | Node::False => {}
                Node::Not(a) | Node::Forall(a) | Node::Exists(a) => nodes.push(a),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    nodes.push(a);
                    nodes.push(b);
                }
            }
        }
        let mut largest = None;
        while let Some(u) = terms.pop() {
            if !terms_seen.insert(u) {
                continue;
            }
            match tables.terms[u.index()] {
                FoTerm::Eigen(e) => largest = largest.max(Some(e.get())),
                FoTerm::App(_, args) => terms.extend_from_slice(tables.list(args)),
                FoTerm::Bound(_) | FoTerm::Meta(_) => {}
            }
        }
        largest
    }

    /// Returns whether the term `t` has no bound variable. It walks on a
    /// stack of its own, each shared term once.
    pub(super) fn is_closed(&self, t: FoTermId) -> bool {
        let tables = self.tables();
        let mut seen = HashSet::default();
        let mut stack = vec![t];
        while let Some(u) = stack.pop() {
            if !seen.insert(u) {
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

#[cfg(feature = "interactive")]
impl Formulas {
    /// Returns the pairs of terms whose unification makes the formulas `x`
    /// and `y` one formula: the arguments of the atoms that stand at the
    /// same place in both, or `None` where the two differ in a connective
    /// or a predicate. Under a binder a pair may hold its bound variable,
    /// which only that variable unifies with. It walks on a stack of its
    /// own, each pair of shared nodes once.
    pub(crate) fn term_pairs(&self, x: NodeId, y: NodeId) -> Option<Vec<(FoTermId, FoTermId)>> {
        let tables = self.tables();
        let mut pairs = Vec::new();
        let mut seen = HashSet::default();
        let mut stack = vec![(x, y)];
        while let Some((a, b)) = stack.pop() {
            if a == b || !seen.insert((a, b)) {
                continue;
            }
            match (self.nodes[a.index()], self.nodes[b.index()]) {
                (Node::Atom(p), Node::Atom(q)) => {
                    let (&(f, s), &(g, t)) =
                        (tables.atoms.get(p as usize)?, tables.atoms.get(q as usize)?);
                    if f != g {
                        return None;
                    }
                    let (s, t) = (tables.list(s), tables.list(t));
                    pairs.extend(s.iter().copied().zip(t.iter().copied()));
                }
                (Node::Not(c), Node::Not(d))
                | (Node::Forall(c), Node::Forall(d))
                | (Node::Exists(c), Node::Exists(d)) => stack.push((c, d)),
                (Node::And(c, d), Node::And(e, f))
                | (Node::Or(c, d), Node::Or(e, f))
                | (Node::Implies(c, d), Node::Implies(e, f))
                | (Node::Iff(c, d), Node::Iff(e, f)) => {
                    stack.push((d, f));
                    stack.push((c, e));
                }
                _ => return None,
            }
        }
        Some(pairs)
    }

    /// Writes every atom's canonical text again from the tables, and the
    /// index of each, after terms were changed in place.
    pub(crate) fn rekey_atoms(&mut self) {
        let tables = self.tables();
        if tables.atoms.len() != self.atoms.len() {
            return;
        }
        let keys: Vec<String> = tables
            .atoms
            .iter()
            .map(|&(p, args)| tables.atom_key(p, args))
            .collect();
        self.names.clear();
        for (i, key) in keys.into_iter().enumerate() {
            // As many atoms as a `u32` numbers.
            self.names.insert(key.clone(), i as u32);
            self.atoms[i] = key;
        }
    }

    /// Returns the closed term `t` as text: `f(t, …)`, an eigenvariable by
    /// the name the proof prints it with, an open witness as `$1`, `$2`, ….
    pub(crate) fn term_text(&self, t: FoTermId) -> String {
        let eigens = self.eigen_names();
        let mut text = String::new();
        let written = self.tables().write_with(
            &mut text,
            t,
            &mut |o: &mut String, name: &str| {
                o.push_str(name);
                Ok(())
            },
            &mut |o: &mut String, index| {
                use std::fmt::Write as _;
                write!(o, "#{index}")
            },
            &mut |o: &mut String, other| {
                match other {
                    FoTerm::Eigen(e) => match eigens.binary_search_by_key(&e.get(), |&(k, _)| k) {
                        Ok(at) => o.push_str(&eigens[at].1),
                        Err(_) => o.push('?'),
                    },
                    FoTerm::Meta(k) => {
                        o.push_str(&crate::linear::occurrences::instances::meta_name(k));
                    }
                    FoTerm::Bound(_) | FoTerm::App(..) => {}
                }
                Ok(())
            },
        );
        debug_assert!(written.is_ok(), "a string takes every write");
        text
    }

    /// Returns the numbers of the open witnesses that occur in one of the
    /// formulas `roots` or in one of the terms `term`, ascending, each once.
    /// It walks on stacks of its own, each shared node and term once.
    pub(crate) fn metas_in(
        &self,
        roots: impl Iterator<Item = NodeId>,
        term: impl Iterator<Item = FoTermId>,
    ) -> Vec<u32> {
        let tables = self.tables();
        let mut nodes_seen = HashSet::default();
        let mut terms_seen = HashSet::default();
        let mut nodes: Vec<NodeId> = roots.collect();
        let mut terms: Vec<FoTermId> = term.collect();
        while let Some(k) = nodes.pop() {
            if !nodes_seen.insert(k) {
                continue;
            }
            match self.nodes[k.index()] {
                Node::Atom(a) => {
                    if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                        terms.extend_from_slice(tables.list(args));
                    }
                }
                Node::True | Node::False => {}
                Node::Not(a) | Node::Forall(a) | Node::Exists(a) => nodes.push(a),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    nodes.push(a);
                    nodes.push(b);
                }
            }
        }
        let mut metas = Vec::new();
        while let Some(u) = terms.pop() {
            if !terms_seen.insert(u) {
                continue;
            }
            match tables.terms[u.index()] {
                FoTerm::Meta(k) => metas.push(k),
                FoTerm::App(_, args) => terms.extend_from_slice(tables.list(args)),
                FoTerm::Bound(_) | FoTerm::Eigen(_) => {}
            }
        }
        metas.sort_unstable();
        metas
    }
}

/// The replacement of open witnesses by the terms they are bound to, over
/// any number of formulas and terms of one arena, each node and term made
/// once. The bindings hold no cycle: a binding never holds the open
/// witness it binds, nor one bound to a term that holds it.
#[cfg(feature = "interactive")]
pub(crate) struct Resolution(Substitution);

#[cfg(feature = "interactive")]
impl Resolution {
    /// Returns the replacement of each open witness `k` by `bindings[k]`,
    /// those with `None` left as they are.
    pub(crate) fn new(bindings: Vec<Option<FoTermId>>) -> Self {
        Self(Substitution::of_metas(bindings))
    }

    /// Returns the formula `node` of `formulas` with its open witnesses
    /// replaced, built in the arena where new.
    ///
    /// # Errors
    ///
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub(crate) fn node(&mut self, formulas: &mut Formulas, node: NodeId) -> Result<NodeId, Error> {
        let made = self.0.run(&mut Build(formulas), node)?;
        Ok(made.expect("an arena that adds what it lacks finds every node"))
    }

    /// Returns the term `t` of `formulas` with its open witnesses replaced,
    /// built in the arena where new.
    pub(crate) fn term(&mut self, formulas: &mut Formulas, t: FoTermId) -> FoTermId {
        self.0
            .substitute(&mut Build(formulas), t, 0)
            .expect("an arena that adds what it lacks finds every term")
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

/// What a substitution replaces.
enum Replace {
    /// The variables of the binders removed, by these closed terms, the
    /// innermost binder's first.
    Variables(Box<[FoTermId]>),
    /// Each open witness by the term it is bound to, by number, `None` for
    /// one left as it is. A bound term may hold open witnesses bound in
    /// turn, which are replaced too.
    #[cfg_attr(
        not(feature = "interactive"),
        expect(dead_code, reason = "only the goal state has open witnesses")
    )]
    Metas(Vec<Option<FoTermId>>),
}

/// The replacement of the variable of one binder by a closed term, at
/// every depth it is met, or of open witnesses by their bindings, each
/// node and term made once per depth.
struct Substitution {
    /// What is replaced.
    replace: Replace,
    /// Every node's instance under so many binders below the removed one.
    nodes: HashMap<(NodeId, u32), NodeId>,
    /// Every term's instance under so many binders below the removed one.
    terms: HashMap<(FoTermId, u32), FoTermId>,
}

impl Substitution {
    /// Returns the substitution of `term` for the removed binder's
    /// variable.
    fn new(term: FoTermId) -> Self {
        Self::of_frame(&[term])
    }

    /// Returns the substitution of `terms` for the variables of the
    /// binders removed, the innermost binder's first.
    fn of_frame(terms: &[FoTermId]) -> Self {
        Self {
            replace: Replace::Variables(terms.into()),
            nodes: HashMap::default(),
            terms: HashMap::default(),
        }
    }

    /// Returns the substitution of each open witness by its binding,
    /// `bindings[k]` for the open witness `k`.
    #[cfg(feature = "interactive")]
    fn of_metas(bindings: Vec<Option<FoTermId>>) -> Self {
        Self {
            replace: Replace::Metas(bindings),
            nodes: HashMap::default(),
            terms: HashMap::default(),
        }
    }

    /// Returns the depth under a binder at `depth`: one more where the
    /// substitution replaces a binder's variable, since a variable then
    /// reads by its depth, and the same where it replaces open witnesses,
    /// which are closed terms.
    const fn inner(&self, depth: u32) -> u32 {
        match self.replace {
            Replace::Variables(_) => depth + 1,
            Replace::Metas(_) => depth,
        }
    }

    /// Returns the binding of the open witness `k`, if it has one.
    fn binding(&self, k: u32) -> Option<FoTermId> {
        match &self.replace {
            Replace::Variables(_) => None,
            Replace::Metas(bindings) => bindings.get(k as usize).copied().flatten(),
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
                    let inner = if matches!(node, Node::Forall(_) | Node::Exists(_)) {
                        self.inner(depth)
                    } else {
                        depth
                    };
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
        let nodes = &self.nodes;
        let at = |k: NodeId, d: u32| nodes[&(k, d)];
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
            Node::Forall(a) => Node::Forall(at(a, self.inner(depth))),
            Node::Exists(a) => Node::Exists(at(a, self.inner(depth))),
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
                    match maker.formulas().tables().terms[u.index()] {
                        FoTerm::App(_, args) => {
                            for &v in maker.formulas().tables().list(args) {
                                stack.push(Task::Visit(v, d));
                            }
                        }
                        FoTerm::Meta(k) => {
                            if let Some(b) = self.binding(k) {
                                stack.push(Task::Visit(b, d));
                            }
                        }
                        FoTerm::Bound(_) | FoTerm::Eigen(_) => {}
                    }
                }
                Task::Make(u, d) => {
                    if self.terms.contains_key(&(u, d)) {
                        continue;
                    }
                    let made = match maker.formulas().tables().terms[u.index()] {
                        // A variable of a removed binder is its term; one
                        // of a binder further out moves in past them.
                        FoTerm::Bound(i) => match &self.replace {
                            Replace::Variables(terms) if i >= d => {
                                match terms.get((i - d) as usize) {
                                    Some(&term) => term,
                                    None => maker.term(FoTerm::Bound(i - terms.len() as u32))?,
                                }
                            }
                            Replace::Variables(_) | Replace::Metas(_) => u,
                        },
                        FoTerm::Meta(k) => match self.binding(k) {
                            Some(b) => self.terms[&(b, d)],
                            None => u,
                        },
                        FoTerm::Eigen(_) => u,
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

/// Returns the names of the first `count` eigenvariables over the symbols
/// of `tables`: `a`, `b`, … `z`, then `a_1`, `b_1`, …, each name of a
/// symbol skipped.
pub(crate) fn eigen_names(tables: &FirstOrder<NodeId>, count: usize) -> Vec<String> {
    let taken: std::collections::HashSet<&str> =
        tables.symbols.iter().map(|(s, _)| s.as_str()).collect();
    (0u64..)
        .map(|n| {
            let letter = char::from(b'a' + (n % 26) as u8);
            match n / 26 {
                0 => letter.to_string(),
                k => format!("{letter}_{k}"),
            }
        })
        .filter(|name| !taken.contains(name.as_str()))
        .take(count)
        .collect()
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
