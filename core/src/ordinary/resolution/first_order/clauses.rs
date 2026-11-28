// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The inference function of the clauses: what a step of resolution,
//! factoring or paramodulation makes, and the matching, renaming and
//! subsumption it rests on.

use super::{Clause, Clauses, Literal, Made, PLACES_PER_POLL, Step};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::{HashMap, HashSet};
use crate::limits::{Phase, Progress, Refusal};
use crate::ordinary::{Formulas, Node, NodeId};
use crate::unify::Unifier;

impl Clauses {
    /// Returns what `step` makes, or why it does not apply.
    pub(super) fn made(&mut self, step: Step) -> Result<Result<Made, &'static str>, Error> {
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
    pub(super) fn paramodulations(
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
    pub(super) fn mapped_atom_charged(
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
    pub(super) fn renumbered(
        &mut self,
        atom: NodeId,
        renamed: &[Option<u32>],
    ) -> Result<NodeId, Error> {
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
    pub(super) fn metas_of(&self, atom: NodeId) -> Vec<u32> {
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
    pub(super) fn is_tautology(clause: &Clause) -> bool {
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
    pub(super) fn subsumes(&self, general: &Clause, special: &Clause, limit: u64) -> bool {
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
    pub(super) fn weight(&mut self, clause: &Clause) -> u64 {
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
pub(super) fn map_term(
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
