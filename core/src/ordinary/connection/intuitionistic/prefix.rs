// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Prefixes and their unification: what makes a classical connection
//! proof an intuitionistic one (Wallen, *Automated Deduction in
//! Nonclassical Logics*, 1990; Otten, "Non-clausal connection calculi for
//! non-classical logics", TABLEAUX 2017, §2 and Table 1).
//!
//! Each position whose rule moves to another world of a Kripke model gets
//! a character: a prefix *variable* where the rule may be taken in any
//! later world (`¬`, `→` and `∀` with polarity 1, and an atom with
//! polarity 1), a prefix *constant* where it makes a new world (the same
//! with polarity 0). A literal's prefix is the string of characters from
//! the root down to it, its own included. A connection is complementary
//! intuitionistically when its two prefixes are equal under one
//! substitution of strings for the prefix variables, the same for every
//! connection of the proof, and when every eigenvariable a term variable
//! is bound to belongs to a world below the term variable's (the domain
//! condition).
//!
//! **Copies**: a copy of a clause renames each prefix variable free in it
//! (Otten 2011, Definition 5: every literal whose prefix holds it lies in
//! the clause or in a clause α-related to it), as the classical prover's
//! copies rename term variables. A prefix constant is a Skolem term over
//! the variables above its position (Otten 2017, Definition 8), so a copy
//! renames it exactly where it renames one of those.
//!
//! **Unification** is Otten and Kreitz's T-string unification ("T-string
//! unification: unifying prefixes in non-classical proof methods",
//! TABLEAUX 1996, Definition 11 and Table 1): ten rules that rewrite one
//! equation at a time, whose successful runs give a minimal complete set
//! of most general unifiers. Prefixes of one formula tree are T-strings
//! (a character never repeats, and equal characters only begin both), and
//! applying a unifier keeps them so (their Lemma 12), so a system is solved
//! one equation at a time.

use super::super::matrix::{Kind, Matrix, NONE, Shape};
use super::super::terms::Terms;
use crate::hash::HashMap;
use crate::ordinary::{Formulas, Node};

/// What character a position adds to the prefixes below it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Char {
    /// None: `∧`, `∨`, `↔`, `∃`, `⊤`, `⊥`.
    No,
    /// A prefix variable: `¬`, `→`, `∀` and an atom with polarity 1.
    Var,
    /// A prefix constant: the same with polarity 0.
    Const,
}

/// A character of a prefix read in a copy: its number, and whether it is
/// a variable.
pub(super) type Sym = (u32, bool);

/// The characters of the matrix's positions, and where each position lies.
#[derive(Debug)]
pub(super) struct Prefixes {
    /// Per occurrence, its character.
    pub(super) chars: Vec<Char>,
    /// Per literal, the occurrences with a character from the root down
    /// to it, its own included, as `chains[start..start + len]`.
    lit_chain: Vec<(u32, u32)>,
    /// The chains.
    chains: Vec<u32>,
    /// Per node of the matrix, the positions with a prefix variable free in
    /// it, sorted; empty but for clauses.
    free: Vec<Vec<u32>>,
    /// The number of every prefix variable read in a copy: by position and
    /// the copy that renames it.
    vars: HashMap<(u32, u32), u32>,
    /// The number of every prefix constant: by position and the numbers of
    /// the variables above it.
    consts: HashMap<(u32, Vec<u32>), u32>,
    /// The numbers given so far.
    count: u32,
}

impl Prefixes {
    /// Returns the characters of `matrix`'s positions over `formulas`.
    pub(super) fn of(matrix: &Matrix, formulas: &Formulas) -> Self {
        let n = matrix.occs.len();
        let mut chars = vec![Char::No; n];
        for (o, occ) in matrix.occs.iter().enumerate() {
            let special = match occ.kind {
                Kind::Atom(_) | Kind::Not | Kind::Implies | Kind::Part { .. } => true,
                Kind::Gamma(_) | Kind::Delta(_) => {
                    matches!(formulas.node(occ.node), Node::Forall(_))
                }
                Kind::True | Kind::False | Kind::And | Kind::Or | Kind::Iff => false,
            };
            chars[o] = match (special, occ.left) {
                (false, _) => Char::No,
                (true, true) => Char::Var,
                (true, false) => Char::Const,
            };
        }
        let mut chains = Vec::new();
        let mut lit_chain = Vec::with_capacity(matrix.literals.len());
        for literal in &matrix.literals {
            let start = chains.len() as u32;
            let mut o = literal.occ;
            let mut chain = Vec::new();
            while o != NONE {
                if chars[o as usize] != Char::No {
                    chain.push(o);
                }
                o = matrix.occs[o as usize].parent;
            }
            chain.reverse();
            chains.extend_from_slice(&chain);
            lit_chain.push((start, chain.len() as u32));
        }
        let mut prefixes = Self {
            chars,
            lit_chain,
            chains,
            free: Vec::new(),
            vars: HashMap::default(),
            consts: HashMap::default(),
            count: 0,
        };
        prefixes.free = prefixes.free_vars(matrix);
        prefixes
    }

    /// Returns, per node of the matrix, the prefix variables free in it,
    /// as the classical matrix finds its free slots: those of its literals
    /// that no other element of a clause above it holds.
    fn free_vars(&self, matrix: &Matrix) -> Vec<Vec<u32>> {
        let n = matrix.nodes.len();
        let mut below: Vec<Vec<u32>> = vec![Vec::new(); n];
        for i in (0..n).rev() {
            below[i] = match matrix.nodes[i].shape {
                Shape::Literal(l) => {
                    let (start, len) = self.lit_chain[l as usize];
                    let mut set: Vec<u32> = self.chains[start as usize..(start + len) as usize]
                        .iter()
                        .copied()
                        .filter(|&o| self.chars[o as usize] == Char::Var)
                        .collect();
                    set.sort_unstable();
                    set
                }
                Shape::Matrix | Shape::Clause => {
                    let mut set = Vec::new();
                    for &k in matrix.kids(i as u32) {
                        set = union(&set, &below[k as usize]);
                    }
                    set
                }
            };
        }
        let mut barred: Vec<Vec<u32>> = vec![Vec::new(); n];
        for i in 0..n {
            if matrix.nodes[i].shape != Shape::Clause {
                continue;
            }
            let kids = matrix.kids(i as u32).to_vec();
            let mut after: Vec<Vec<u32>> = vec![Vec::new(); kids.len() + 1];
            for j in (0..kids.len()).rev() {
                after[j] = union(&after[j + 1], &below[kids[j] as usize]);
            }
            let mut before = barred[i].clone();
            for (j, &k) in kids.iter().enumerate() {
                if matrix.nodes[k as usize].shape != Shape::Matrix {
                    before = union(&before, &below[k as usize]);
                    continue;
                }
                let set = union(&before, &after[j + 1]);
                before = union(&before, &below[k as usize]);
                for &c in matrix.kids(k) {
                    barred[c as usize].clone_from(&set);
                }
            }
        }
        (0..n)
            .map(|i| {
                if matrix.nodes[i].shape == Shape::Clause {
                    below[i]
                        .iter()
                        .copied()
                        .filter(|o| barred[i].binary_search(o).is_err())
                        .collect()
                } else {
                    Vec::new()
                }
            })
            .collect()
    }

    /// Returns whether a copy of clause `clause` renames a prefix variable.
    pub(super) fn renames(&self, clause: u32) -> bool {
        !self.free[clause as usize].is_empty()
    }

    /// Returns the copy that renames the prefix variable of position `o`
    /// read in copy `copy`: the innermost copy on its chain whose clause
    /// has it free, the matrix itself (copy 0) for the others.
    fn owner(&self, terms: &Terms, copy: u32, o: u32) -> u32 {
        let mut k = copy;
        loop {
            let c = terms.copies[k as usize];
            if c.clause == NONE || self.free[c.clause as usize].binary_search(&o).is_ok() {
                return k;
            }
            k = c.parent;
        }
    }

    /// Returns a new number.
    fn next(&mut self) -> u32 {
        self.count += 1;
        self.count - 1
    }

    /// Returns the character of position `o` read in copy `copy`, where
    /// `above` are the numbers of the prefix variables above it.
    fn sym(&mut self, terms: &Terms, copy: u32, o: u32, above: &[u32]) -> Sym {
        if self.chars[o as usize] == Char::Var {
            let owner = self.owner(terms, copy, o);
            let number = match self.vars.get(&(o, owner)) {
                Some(&n) => n,
                None => {
                    let n = self.next();
                    self.vars.insert((o, owner), n);
                    n
                }
            };
            (number, true)
        } else {
            let key = (o, above.to_vec());
            let number = match self.consts.get(&key) {
                Some(&n) => n,
                None => {
                    let n = self.next();
                    self.consts.insert(key, n);
                    n
                }
            };
            (number, false)
        }
    }

    /// Returns the characters of the positions `chain`, outermost first,
    /// read in copy `copy`.
    fn read(&mut self, terms: &Terms, copy: u32, chain: &[u32]) -> Vec<Sym> {
        let mut out = Vec::with_capacity(chain.len());
        let mut above: Vec<u32> = Vec::new();
        for &o in chain {
            let sym = self.sym(terms, copy, o, &above);
            if sym.1 {
                above.push(sym.0);
            }
            out.push(sym);
        }
        out
    }

    /// Returns the prefix of literal `lit` read in copy `copy`.
    pub(super) fn of_literal(
        &mut self,
        matrix: &Matrix,
        terms: &Terms,
        lit: u32,
        copy: u32,
    ) -> Vec<Sym> {
        let _ = matrix;
        let (start, len) = self.lit_chain[lit as usize];
        let chain: Vec<u32> = self.chains[start as usize..(start + len) as usize].to_vec();
        self.read(terms, copy, &chain)
    }

    /// Returns the prefix of occurrence `o` read in copy `copy`: the
    /// characters from the root down to it, its own included.
    pub(super) fn of_occurrence(
        &mut self,
        matrix: &Matrix,
        terms: &Terms,
        o: u32,
        copy: u32,
    ) -> Vec<Sym> {
        let mut chain = Vec::new();
        let mut at = o;
        while at != NONE {
            if self.chars[at as usize] != Char::No {
                chain.push(at);
            }
            at = matrix.occs[at as usize].parent;
        }
        chain.reverse();
        self.read(terms, copy, &chain)
    }

    /// Returns a fresh variable, which no position has.
    pub(super) fn fresh(&mut self) -> Sym {
        (self.next(), true)
    }
}

/// Returns the union of two sorted lists.
fn union(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => {
                out.push(a[i]);
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                out.push(b[j]);
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                out.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

/// Returns whether the prefixes `s` and `t` can be unified at all: Otten
/// and Kreitz's test (their Lemma 8), linear in the strings, after the
/// common beginning is cut off. It binds nothing.
pub(super) fn unifiable(s: &[Sym], t: &[Sym]) -> bool {
    let common = s.iter().zip(t).take_while(|(a, b)| a == b).count();
    let (s, t) = (&s[common..], &t[common..]);
    let starts = |x: &[Sym]| x.first().is_some_and(|c| c.1);
    let ends = |x: &[Sym]| x.last().is_some_and(|c| c.1);
    let has_const = |x: &[Sym]| x.iter().any(|c| !c.1);
    let has_var = |x: &[Sym]| x.iter().any(|c| c.1);
    (s.is_empty() && t.is_empty())
        || (starts(s) && ends(t))
        || (ends(s) && starts(t))
        || (starts(s) && ends(s) && (!has_const(s) || has_var(t)))
        || (starts(t) && ends(t) && (!has_const(t) || has_var(s)))
}

/// An equation `s = z | t` of the transformation: the left string, the
/// part of the right string taken for the left string's first variable
/// so far, and the rest of the right string.
#[derive(Clone, Debug)]
struct Equation {
    /// The left string.
    s: Vec<Sym>,
    /// What the first variable of `s` has taken so far.
    z: Vec<Sym>,
    /// The rest of the right string.
    t: Vec<Sym>,
}

/// A state of the search for a unifier: the next equation of the system
/// to take up, the one being rewritten, the bindings so far, and the next
/// auxiliary variable's number. The equations not taken up yet are read
/// under the bindings when they are, so a state holds none of them.
#[derive(Clone, Debug)]
struct State {
    /// The next equation of the system to take up.
    next_equation: usize,
    /// The equation being rewritten, `None` between two.
    current: Option<Equation>,
    /// The bindings, each variable at most once, in the order made.
    bound: Vec<(u32, Vec<Sym>)>,
    /// The number of the next auxiliary variable.
    next: u32,
}

impl State {
    /// Returns `x` with every bound variable replaced by its image, the
    /// images read under the bindings too.
    fn resolve(&self, x: &[Sym]) -> Vec<Sym> {
        let mut out = Vec::with_capacity(x.len());
        let mut stack: Vec<Sym> = x.iter().rev().copied().collect();
        while let Some(c) = stack.pop() {
            match c
                .1
                .then(|| self.bound.iter().rev().find(|(v, _)| *v == c.0))
                .flatten()
            {
                Some((_, image)) => stack.extend(image.iter().rev().copied()),
                None => out.push(c),
            }
        }
        out
    }

    /// Binds `v` to `image` and reads the current equation under it.
    fn bind(&mut self, v: u32, image: Vec<Sym>) {
        self.bound.push((v, image));
        if let Some(e) = self.current.take() {
            self.current = Some(Equation {
                s: self.resolve(&e.s),
                z: self.resolve(&e.z),
                t: self.resolve(&e.t),
            });
        }
    }
}

/// Why the search for a unifier gave no answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum NoUnifier {
    /// It tried every alternative, or `budget` ran out.
    Failed,
    /// The stop fired.
    Stopped,
}

/// Searches a substitution of strings for prefix variables that makes
/// every pair of `equations` equal, by the ten rules of T-string
/// unification, trying their alternatives depth first. Fresh auxiliary
/// variables are numbered from `next`. It takes at most `budget` steps
/// and asks `stop` every 256.
pub(super) fn unify(
    equations: &[(Vec<Sym>, Vec<Sym>)],
    next: u32,
    budget: &mut u64,
    stop: &mut dyn FnMut() -> bool,
) -> Result<HashMap<u32, Vec<Sym>>, NoUnifier> {
    let start = State {
        next_equation: 0,
        current: None,
        bound: Vec::new(),
        next,
    };
    let mut stack = vec![start];
    let mut steps = 0u64;
    while let Some(mut state) = stack.pop() {
        if *budget == 0 {
            return Err(NoUnifier::Failed);
        }
        *budget -= 1;
        steps += 1;
        if steps.is_multiple_of(256) && stop() {
            return Err(NoUnifier::Stopped);
        }
        if state.current.is_none() {
            let Some((s, t)) = equations.get(state.next_equation) else {
                return Ok(state.bound.into_iter().collect());
            };
            state.current = Some(Equation {
                s: state.resolve(s),
                z: Vec::new(),
                t: state.resolve(t),
            });
            state.next_equation += 1;
        }
        // The successors, pushed so that the first rule is tried first.
        let mut next_states = successors(state);
        next_states.reverse();
        stack.extend(next_states);
    }
    Err(NoUnifier::Failed)
}

/// Returns the states the rules make of `state`'s current equation, one
/// per rule that applies (Otten and Kreitz 1996, Table 1).
fn successors(mut state: State) -> Vec<State> {
    let Some(e) = state.current.take() else {
        return vec![state];
    };
    let (s, z, t) = (&e.s, &e.z, &e.t);
    let mut out = Vec::new();
    // Makes the state with `equation` current after `binding`.
    let push = |out: &mut Vec<State>,
                state: &State,
                equation: Option<Equation>,
                binding: Option<(u32, Vec<Sym>)>,
                aux: bool| {
        let mut next = state.clone();
        if aux {
            next.next += 1;
        }
        next.current = equation;
        if let Some((v, image)) = binding {
            next.bind(v, image);
        }
        out.push(next);
    };
    let eq = |s: &[Sym], z: &[Sym], t: &[Sym]| Equation {
        s: s.to_vec(),
        z: z.to_vec(),
        t: t.to_vec(),
    };
    match (s.first(), t.first()) {
        // R1: ε = ε | ε.
        (None, None) if z.is_empty() => push(&mut out, &state, None, None, false),
        // R2: ε = ε | t⁺ becomes t⁺ = ε | ε.
        (None, Some(_)) if z.is_empty() => {
            push(&mut out, &state, Some(eq(t, &[], &[])), None, false);
        }
        // R3: X s = ε | X t.
        (Some(&x), Some(&y)) if z.is_empty() && x == y => {
            push(
                &mut out,
                &state,
                Some(eq(&s[1..], &[], &t[1..])),
                None,
                false,
            );
        }
        // R4: C s = ε | V t becomes V t = ε | C s.
        (Some(&c), Some(&v)) if z.is_empty() && !c.1 && v.1 => {
            push(&mut out, &state, Some(eq(t, &[], s)), None, false);
        }
        (Some(&v), _) if v.1 => {
            let rest = &s[1..];
            match t.first() {
                // R5: V s = z | ε.
                None => push(
                    &mut out,
                    &state,
                    Some(eq(rest, &[], &[])),
                    Some((v.0, z.clone())),
                    false,
                ),
                Some(&x) => {
                    // R6: V s = ε | C₁ t.
                    if z.is_empty() && !x.1 {
                        push(
                            &mut out,
                            &state,
                            Some(eq(rest, &[], t)),
                            Some((v.0, Vec::new())),
                            false,
                        );
                    }
                    // R7: V s = z | C₁ C₂ t.
                    if !x.1 && t.get(1).is_some_and(|c| !c.1) {
                        let mut image = z.clone();
                        image.push(x);
                        push(
                            &mut out,
                            &state,
                            Some(eq(rest, &[], &t[1..])),
                            Some((v.0, image)),
                            false,
                        );
                    }
                    if x.1 && x != v && !rest.is_empty() {
                        if z.is_empty() {
                            // R8: V s⁺ = ε | V₁ t becomes V₁ t = V | s⁺.
                            push(&mut out, &state, Some(eq(t, &[v], rest)), None, false);
                        } else {
                            // R9: V s⁺ = z⁺ | V₁ t becomes V₁ t = V' | s⁺,
                            // binding V to z⁺ V'.
                            let aux = (state.next, true);
                            let mut image = z.clone();
                            image.push(aux);
                            push(
                                &mut out,
                                &state,
                                Some(eq(t, &[aux], rest)),
                                Some((v.0, image)),
                                true,
                            );
                        }
                    }
                    // R10: V s = z | X t becomes V s = z X | t, where V ≠ X
                    // and s is empty, t is not, or X is a constant.
                    if x != v && (rest.is_empty() || t.len() > 1 || !x.1) {
                        let mut taken = z.clone();
                        taken.push(x);
                        push(&mut out, &state, Some(eq(s, &taken, &t[1..])), None, false);
                    }
                }
            }
        }
        _ => {}
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A variable and a constant.
    const fn v(n: u32) -> Sym {
        (n, true)
    }

    /// A constant.
    const fn c(n: u32) -> Sym {
        (n, false)
    }

    /// Applies `sigma` to `x`, its images followed until none is bound.
    fn apply(sigma: &HashMap<u32, Vec<Sym>>, x: &[Sym]) -> Vec<Sym> {
        let mut out = Vec::new();
        for &ch in x {
            match (ch.1, sigma.get(&ch.0)) {
                (true, Some(image)) => out.extend(apply(sigma, image)),
                _ => out.push(ch),
            }
        }
        out
    }

    /// The papers' examples unify, their unifiers make both sides equal,
    /// and the excluded middle's prefixes do not.
    #[test]
    fn t_string_unification_finds_a_unifier() {
        let cases: Vec<(Vec<Sym>, Vec<Sym>, bool)> = vec![
            // a₁V₂V₃ = a₁a₃ (Otten 2017, p. 7).
            (vec![c(1), v(2), v(3)], vec![c(1), c(3)], true),
            // aX = Yb (Otten and Kreitz 1996, p. 2).
            (vec![c(1), v(2)], vec![v(3), c(4)], true),
            // P ∨ ¬P: a₁ = a₂V₁ has none.
            (vec![c(1)], vec![c(2), v(3)], false),
            // aBcDEf = aBGhiJ (their Example 10).
            (
                vec![c(1), v(2), c(3), v(4), v(5), c(6)],
                vec![c(1), v(2), v(7), c(8), c(9), v(10)],
                true,
            ),
            (vec![c(1), c(2)], vec![c(1), c(3)], false),
            (vec![v(1)], vec![], true),
        ];
        for (s, t, expected) in cases {
            let mut budget = 10_000;
            let found = unify(&[(s.clone(), t.clone())], 100, &mut budget, &mut || false);
            assert_eq!(found.is_ok(), expected, "{s:?} = {t:?}");
            assert_eq!(unifiable(&s, &t), expected, "{s:?} = {t:?}");
            if let Ok(sigma) = found {
                assert_eq!(apply(&sigma, &s), apply(&sigma, &t), "{s:?} = {t:?}");
            }
        }
    }
}
