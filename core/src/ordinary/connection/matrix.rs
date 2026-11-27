// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The non-clausal matrix of a classical sequent (Otten, "A Non-clausal
//! Connection Calculus", TABLEAUX 2011, Table 1 and Definition 2).
//!
//! A sequent `Γ ⊢ Δ` is valid when every path through its matrix holds a
//! connection. Each hypothesis is read with polarity 1, each formula
//! right of `⊢` with polarity 0. Its formulas are unfolded into a tree of
//! signed occurrences ([`Occ`]), which the read-back walks too. From it
//! the matrix is built: a matrix is a list of clauses, a clause a list of
//! literals and matrices. A path takes one element of every clause of
//! every matrix it enters, so a matrix is a disjunction of its clauses and
//! a clause a conjunction of its elements.
//!
//! - An α-formula (`A ∧ B` with polarity 1, `A ∨ B` and `A → B` with
//!   polarity 0) puts its operands' clauses side by side.
//! - A β-formula (the others) makes one clause of its operands' elements.
//! - A γ-formula (`∀` with polarity 1, `∃` with polarity 0) gives its
//!   variable a slot, which every copy of a clause fills with a variable
//!   of its own.
//! - A δ-formula gives its variable a Skolem term over the slots of every
//!   γ-formula around it.
//!
//! Brackets of the form `{{X₁, …, Xₙ}}` are removed as the formula is
//! built, so a clause holds no matrix of one clause and a matrix no clause
//! of one matrix. Every walk is over the occurrences or the matrix in
//! index order or on a stack of its own: nothing here recurses over a
//! formula.

use super::super::{Formulas, Node, NodeId, Sequent};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal, Space};

/// How many occurrences or nodes the build lays out between two asks of
/// the stop and the memory bound.
const POLL: usize = 1 << 16;

/// The most entries of a table the matrix indexes with `u32`, below
/// [`NONE`].
const MOST: usize = (NONE - 1) as usize;

/// The build's account: the stop, the memory bound, and the work done.
/// The prefixes' build (`intuitionistic::prefix`) keeps one too.
pub(super) struct Budget<'a> {
    /// The caller's stop.
    pub(super) stop: &'a mut dyn FnMut(Progress) -> bool,
    /// The memory bound, `None` for none.
    pub(super) limit: Option<u64>,
    /// The work done, for the stop's progress.
    pub(super) done: u64,
}

impl Budget<'_> {
    /// Asks the stop and compares `bytes` with the bound.
    pub(super) fn poll(&mut self, work: u64, bytes: usize) -> Result<(), Error> {
        self.done = self.done.saturating_add(work);
        if (self.stop)(Progress::new(Phase::Search, work, self.done)) {
            return Err(Error::Refused(Refusal::stopped(Phase::Search)));
        }
        match self.limit {
            Some(limit) if bytes as u64 > limit => Err(Error::Refused(Refusal::Memory {
                phase: Phase::Search,
                limit_bytes: limit,
                needed_bytes: None,
            })),
            _ => Ok(()),
        }
    }
}

/// Fails with [`Refusal::Index`] where a table of `len` entries passes the
/// `u32` indices.
fn indexed(len: usize, what: Space) -> Result<(), Error> {
    if len > MOST {
        Err(Error::Refused(Refusal::Index {
            what,
            count: len as u64,
            most: MOST as u64,
        }))
    } else {
        Ok(())
    }
}

/// No index: the parent of a root, the slot around a literal under no
/// γ-formula.
pub(super) const NONE: u32 = u32::MAX;

/// What a signed occurrence is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    /// An atom, the literal of that index.
    Atom(u32),
    /// `⊤`.
    True,
    /// `⊥`.
    False,
    /// `¬A`: one child, of the other polarity.
    Not,
    /// `A ∧ B`.
    And,
    /// `A ∨ B`.
    Or,
    /// `A → B`.
    Implies,
    /// `A ↔ B`: two children, the implications `A → B` and `B → A`.
    Iff,
    /// One implication of an equivalence: `A → B` where `forward`, else
    /// `B → A`, its children the antecedent and the consequent.
    Part {
        /// Whether it is the implication from left to right.
        forward: bool,
    },
    /// A γ-formula, whose variable is the slot of that index.
    Gamma(u32),
    /// A δ-formula, whose variable is the Skolem term of that index.
    Delta(u32),
}

/// A signed occurrence of a subformula of the sequent.
#[derive(Clone, Copy, Debug)]
pub(super) struct Occ {
    /// The formula, its bound variables still de Bruijn indices of the
    /// binders above it.
    pub(super) node: NodeId,
    /// The occurrence it is an operand of, [`NONE`] for a formula of the
    /// sequent.
    pub(super) parent: u32,
    /// Its polarity: true for 1, a hypothesis's side.
    pub(super) left: bool,
    /// What it is.
    pub(super) kind: Kind,
    /// Its operands, in order; [`NONE`] past their number.
    pub(super) children: [u32; 2],
    /// The nearest binder around it, [`NONE`] for none.
    binder: u32,
    /// The innermost slot around it, [`NONE`] for none.
    pub(super) slot: u32,
}

/// A term of a literal, with the slots of γ-formulas for variables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Pat {
    /// The variable of a slot.
    Var(u32),
    /// A symbol applied to the patterns `args.0 .. args.0 + args.1` of
    /// [`Matrix::pat_args`].
    App(u32, (u32, u32)),
}

/// What a symbol of the patterns stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Symbol {
    /// A function symbol of the sequent's arena.
    Function(crate::first_order::Symbol),
    /// A term of the arena no quantifier binds: an eigenvariable or an
    /// open witness of the arena, a constant here.
    Parameter(FoTermId),
    /// The Skolem term of a δ-formula, by its index.
    Skolem(u32),
}

/// A literal of the matrix.
#[derive(Clone, Copy, Debug)]
pub(super) struct Literal {
    /// Its occurrence.
    pub(super) occ: u32,
    /// Its predicate: the symbol's index in a first-order arena, the
    /// atom's in a propositional one.
    pub(super) predicate: u32,
    /// Its polarity: true for 1.
    pub(super) left: bool,
    /// Its arguments, `args.0 .. args.0 + args.1` of
    /// [`Matrix::lit_args`].
    pub(super) args: (u32, u32),
    /// Its node in the matrix.
    pub(super) node: u32,
}

/// What a node of the matrix is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Shape {
    /// A literal, by its index.
    Literal(u32),
    /// A matrix: its children are clauses.
    Matrix,
    /// A clause: its children are literals and matrices.
    Clause,
}

/// A node of the matrix, numbered in preorder so that a subtree is a
/// range of indices.
#[derive(Clone, Copy, Debug)]
pub(super) struct MNode {
    /// What it is.
    pub(super) shape: Shape,
    /// The node it is a child of, [`NONE`] for the root matrix.
    pub(super) parent: u32,
    /// One past the last node of its subtree.
    pub(super) end: u32,
    /// Its children, `children.0 .. children.0 + children.1` of
    /// [`Matrix::children`].
    pub(super) children: (u32, u32),
    /// Whether it is positive (Definition 16): a literal of polarity 0, a
    /// clause all of whose elements are, a matrix with such a clause.
    pub(super) positive: bool,
    /// For a clause, the slots its literals have, sorted, each with
    /// whether its variable is free in the clause (Definition 5), `slots.0
    /// .. slots.0 + slots.1` of [`Matrix::slots`]; empty for the others.
    pub(super) slots: (u32, u32),
}

/// The occurrence tree and the matrix of a sequent.
#[derive(Debug)]
pub(super) struct Matrix {
    /// The signed occurrences, each after its parent.
    pub(super) occs: Vec<Occ>,
    /// The occurrences of the sequent's formulas: its hypotheses, then
    /// those right of `⊢`.
    pub(super) roots: Vec<u32>,
    /// The occurrence of each slot's γ-formula.
    pub(super) slot_occ: Vec<u32>,
    /// The slot around each slot's γ-formula, [`NONE`] for none.
    pub(super) slot_outer: Vec<u32>,
    /// The occurrence of each Skolem term's δ-formula.
    pub(super) skolem_occ: Vec<u32>,
    /// The symbols of the patterns.
    pub(super) symbols: Vec<Symbol>,
    /// Whether each symbol is an open witness of the arena, which no
    /// derivation names as a quantifier's witness, so that no variable is
    /// bound to a term that holds one.
    pub(super) opaque: Vec<bool>,
    /// The patterns.
    pub(super) pats: Vec<Pat>,
    /// The arguments of every application among the patterns.
    pub(super) pat_args: Vec<u32>,
    /// The literals.
    pub(super) literals: Vec<Literal>,
    /// The arguments of every literal.
    pub(super) lit_args: Vec<u32>,
    /// The nodes of the matrix in preorder; the root matrix is node 0.
    pub(super) nodes: Vec<MNode>,
    /// The children of every node.
    pub(super) children: Vec<u32>,
    /// The slots of every clause and whether each is free there.
    pub(super) slots: Vec<(u32, bool)>,
    /// The literals of each predicate and polarity, by `2 · predicate +
    /// polarity`.
    pub(super) by_predicate: HashMap<u64, Vec<u32>>,
}

/// The memories of the patterns made: of closed terms by term, of the
/// terms of the atom at hand by term, the symbol of each parameter, the
/// pattern of each Skolem term.
type Memo<'a> = (
    &'a mut [u32],
    &'a mut HashMap<FoTermId, u32>,
    &'a mut HashMap<FoTermId, u32>,
    &'a mut [u32],
);

/// A part of a matrix being built: a list of clauses or of elements, held
/// as a tree whose leaves are in order.
#[derive(Clone, Copy, Debug)]
enum Rope {
    /// No item.
    Empty,
    /// One item.
    Leaf(Item),
    /// The items of both.
    Concat(u32, u32),
}

/// An item of a [`Rope`].
#[derive(Clone, Copy, Debug)]
enum Item {
    /// A clause of the elements of a rope.
    Clause(u32),
    /// A literal.
    Literal(u32),
    /// A matrix of the clauses of a rope.
    Sub(u32),
}

/// The ropes being built and their numbers of items.
#[derive(Debug, Default)]
struct Ropes {
    /// The ropes.
    ropes: Vec<Rope>,
    /// How many items each holds.
    counts: Vec<u64>,
}

impl Ropes {
    /// Returns the bytes the ropes hold, by capacity.
    fn held_bytes(&self) -> usize {
        self.ropes.capacity() * size_of::<Rope>() + self.counts.capacity() * size_of::<u64>()
    }

    /// Adds a rope and returns its index.
    fn push(&mut self, rope: Rope, count: u64) -> u32 {
        self.ropes.push(rope);
        self.counts.push(count);
        (self.ropes.len() - 1) as u32
    }

    /// Returns the rope of no item.
    fn empty(&mut self) -> u32 {
        self.push(Rope::Empty, 0)
    }

    /// Returns the rope of one item.
    fn leaf(&mut self, item: Item) -> u32 {
        self.push(Rope::Leaf(item), 1)
    }

    /// Returns the rope of `a`'s items and then `b`'s, which is one of
    /// them where the other is empty, so that a rope of one item is a leaf.
    fn concat(&mut self, a: u32, b: u32) -> u32 {
        let (x, y) = (self.counts[a as usize], self.counts[b as usize]);
        if x == 0 {
            b
        } else if y == 0 {
            a
        } else {
            self.push(Rope::Concat(a, b), x.saturating_add(y))
        }
    }

    /// Returns the items of `rope`, in order.
    fn items(&self, rope: u32) -> Vec<Item> {
        let mut items = Vec::new();
        let mut stack = vec![rope];
        while let Some(r) = stack.pop() {
            match self.ropes[r as usize] {
                Rope::Empty => {}
                Rope::Leaf(item) => items.push(item),
                Rope::Concat(a, b) => {
                    stack.push(b);
                    stack.push(a);
                }
            }
        }
        items
    }

    /// Returns the one item of a rope of one item.
    fn single(&self, rope: u32) -> Option<Item> {
        match self.ropes[rope as usize] {
            Rope::Leaf(item) => Some(item),
            Rope::Empty | Rope::Concat(..) => None,
        }
    }
}

impl Matrix {
    /// Returns the matrix of `sequent`, its formulas unfolded within
    /// `limits.occurrences`, asking `stop` and the memory bound as it goes.
    ///
    /// # Errors
    ///
    /// [`Refusal::Occurrences`] for a sequent that unfolds past the bound
    /// (an equivalence's operands are unfolded twice), [`Refusal::Memory`]
    /// past `limits.memory_bytes`, [`Refusal::Stopped`] when the stop
    /// fires, and [`Refusal::Index`] for a matrix past its `u32` indices.
    pub(super) fn of(
        sequent: &Sequent,
        limits: &Limits,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
        let mut budget = Budget {
            stop,
            limit: limits.memory_bytes,
            done: 0,
        };
        let mut matrix = Self {
            occs: Vec::new(),
            roots: Vec::new(),
            slot_occ: Vec::new(),
            slot_outer: Vec::new(),
            skolem_occ: Vec::new(),
            symbols: Vec::new(),
            opaque: Vec::new(),
            pats: Vec::new(),
            pat_args: Vec::new(),
            literals: Vec::new(),
            lit_args: Vec::new(),
            nodes: Vec::new(),
            children: Vec::new(),
            slots: Vec::new(),
            by_predicate: HashMap::default(),
        };
        matrix.unfold(sequent, limits, &mut budget)?;
        matrix.literals_of(sequent.formulas(), &mut budget)?;
        let terms = &sequent.formulas().first_order().terms;
        matrix.opaque = matrix
            .symbols
            .iter()
            .map(|s| matches!(s, Symbol::Parameter(t) if matches!(terms[t.index()], FoTerm::Meta(_))))
            .collect();
        matrix.build(&mut budget)?;
        indexed(matrix.nodes.len(), Space::Node)?;
        indexed(matrix.pats.len(), Space::Term)?;
        matrix.free_slots(&mut budget)?;
        Ok(matrix)
    }

    /// Returns the bytes the matrix holds, by capacity: what a search's
    /// memory bound counts for it.
    pub(super) fn held_bytes(&self) -> usize {
        self.occs.capacity() * size_of::<Occ>()
            + (self.slot_occ.capacity() + self.slot_outer.capacity() + self.skolem_occ.capacity())
                * size_of::<u32>()
            + self.symbols.capacity() * size_of::<Symbol>()
            + self.pats.capacity() * size_of::<Pat>()
            + (self.pat_args.capacity() + self.lit_args.capacity() + self.children.capacity())
                * size_of::<u32>()
            + self.literals.capacity() * size_of::<Literal>()
            + self.nodes.capacity() * size_of::<MNode>()
            + self.slots.capacity() * size_of::<(u32, bool)>()
    }

    /// Lays out the signed occurrences, each after its parent.
    fn unfold(
        &mut self,
        sequent: &Sequent,
        limits: &Limits,
        budget: &mut Budget<'_>,
    ) -> Result<(), Error> {
        let formulas = sequent.formulas();
        let bound = limits.occurrences.unwrap_or(u64::MAX);
        let most = bound.min(MOST as u64);
        let roots = sequent
            .left()
            .iter()
            .map(|&f| (f, true))
            .chain(sequent.right().iter().map(|&f| (f, false)));
        // Each entry: the formula, its polarity, its parent, the child slot,
        // and for an equivalence's part its direction.
        let mut stack: Vec<(NodeId, bool, u32, u8, Option<bool>)> = Vec::new();
        for (f, left) in roots.rev() {
            stack.push((f, left, NONE, 0, None));
        }
        while let Some((node, left, parent, at, part)) = stack.pop() {
            if self.occs.len() as u64 >= most {
                // Past the bound, or past the indices where the bound is
                // larger.
                return Err(Error::Refused(if bound <= MOST as u64 {
                    Refusal::Occurrences {
                        occurrences: most.saturating_add(1),
                        limit: bound,
                    }
                } else {
                    Refusal::Index {
                        what: Space::Occurrence,
                        count: most.saturating_add(1),
                        most,
                    }
                }));
            }
            if self.occs.len() % POLL == POLL - 1 {
                budget.poll(POLL as u64, self.occs.capacity() * size_of::<Occ>())?;
            }
            let index = self.occs.len() as u32;
            let (binder, slot) = if parent == NONE {
                (NONE, NONE)
            } else {
                let p = &self.occs[parent as usize];
                let binder = if matches!(p.kind, Kind::Gamma(_) | Kind::Delta(_)) {
                    parent
                } else {
                    p.binder
                };
                let slot = if let Kind::Gamma(s) = p.kind {
                    s
                } else {
                    p.slot
                };
                (binder, slot)
            };
            let kind = match part {
                Some(forward) => Kind::Part { forward },
                None => match formulas.node(node) {
                    Node::Atom(_) => Kind::Atom(NONE),
                    Node::True => Kind::True,
                    Node::False => Kind::False,
                    Node::Not(_) => Kind::Not,
                    Node::And(..) => Kind::And,
                    Node::Or(..) => Kind::Or,
                    Node::Implies(..) => Kind::Implies,
                    Node::Iff(..) => Kind::Iff,
                    // γ: `∀` with polarity 1, `∃` with polarity 0.
                    Node::Forall(_) if left => self.gamma(index, slot),
                    Node::Exists(_) if !left => self.gamma(index, slot),
                    Node::Forall(_) | Node::Exists(_) => {
                        self.skolem_occ.push(index);
                        Kind::Delta((self.skolem_occ.len() - 1) as u32)
                    }
                },
            };
            self.occs.push(Occ {
                node,
                parent,
                left,
                kind,
                children: [NONE; 2],
                binder,
                slot,
            });
            if parent != NONE {
                self.occs[parent as usize].children[at as usize] = index;
            } else {
                self.roots.push(index);
            }
            // The operands, pushed so that the first is laid out first.
            let mut operands: [Option<(NodeId, bool, Option<bool>)>; 2] = [None, None];
            match (kind, formulas.node(node)) {
                (Kind::Not, Node::Not(a)) => operands[0] = Some((a, !left, None)),
                (Kind::And | Kind::Or, Node::And(a, b) | Node::Or(a, b)) => {
                    operands = [Some((a, left, None)), Some((b, left, None))];
                }
                (Kind::Implies, Node::Implies(a, b)) => {
                    operands = [Some((a, !left, None)), Some((b, left, None))];
                }
                (Kind::Iff, Node::Iff(..)) => {
                    operands = [
                        Some((node, left, Some(true))),
                        Some((node, left, Some(false))),
                    ];
                }
                (Kind::Part { forward }, Node::Iff(a, b)) => {
                    let (from, to) = if forward { (a, b) } else { (b, a) };
                    operands = [Some((from, !left, None)), Some((to, left, None))];
                }
                (Kind::Gamma(_) | Kind::Delta(_), Node::Forall(a) | Node::Exists(a)) => {
                    operands[0] = Some((a, left, None));
                }
                _ => {}
            }
            for (k, operand) in operands.iter().enumerate().rev() {
                if let Some((f, l, p)) = *operand {
                    stack.push((f, l, index, k as u8, p));
                }
            }
        }
        Ok(())
    }

    /// Gives the γ-formula at occurrence `index`, under the slot `outer`,
    /// a slot of its own.
    fn gamma(&mut self, index: u32, outer: u32) -> Kind {
        self.slot_occ.push(index);
        self.slot_outer.push(outer);
        Kind::Gamma((self.slot_occ.len() - 1) as u32)
    }

    /// Returns the slots around occurrence `occ`, outermost first.
    pub(super) fn env(&self, occ: u32) -> Vec<u32> {
        let mut env = Vec::new();
        let mut s = self.occs[occ as usize].slot;
        while s != NONE {
            env.push(s);
            s = self.slot_outer[s as usize];
        }
        env.reverse();
        env
    }

    /// Makes the literal of every atom occurrence, its terms as patterns,
    /// asking the stop and the memory bound every [`POLL`] units of work (a
    /// term walked, a binder passed).
    ///
    /// # Errors
    ///
    /// [`Refusal::Stopped`] and [`Refusal::Memory`] from `budget`.
    fn literals_of(&mut self, formulas: &Formulas, budget: &mut Budget<'_>) -> Result<(), Error> {
        let tables = formulas.first_order();
        let first_order = formulas.is_first_order();
        let loose = tables.loose();
        let functions = tables.symbols.len() as u32;
        // The symbols: the arena's, then its parameters as they are met,
        // then the Skolem terms.
        self.symbols = (0..functions)
            .map(|s| Symbol::Function(crate::first_order::Symbol::new(s)))
            .collect();
        let mut parameters: HashMap<FoTermId, u32> = HashMap::default();
        // The patterns of closed terms, which no binder changes.
        let mut closed: Vec<u32> = vec![NONE; tables.terms.len()];
        let mut skolems: Vec<u32> = vec![NONE; self.skolem_occ.len()];
        let mut local: HashMap<FoTermId, u32> = HashMap::default();
        // The binders around the literal, innermost first, as far as its
        // terms reach: each found once per literal, where a walk per bound
        // variable was quadratic in a nest of thousands of quantifiers.
        let mut chain: Vec<u32> = Vec::new();
        let mut work = 0u64;
        for o in 0..self.occs.len() {
            let Kind::Atom(_) = self.occs[o].kind else {
                continue;
            };
            let occ = self.occs[o];
            let Node::Atom(atom) = formulas.node(occ.node) else {
                unreachable!("an atom occurrence holds an atom");
            };
            let (predicate, args) = if first_order {
                let (symbol, args) = tables.atoms[atom as usize];
                (symbol.get(), tables.list(args).to_vec())
            } else {
                (atom, Vec::new())
            };
            local.clear();
            chain.clear();
            let start = self.lit_args.len() as u32;
            // Polled per argument: one argument can make thousands of
            // patterns (a Skolem term takes every slot around it).
            for &t in &args {
                let before = self.pats.len() + chain.len();
                let p = self.pattern(
                    formulas,
                    &loose,
                    o as u32,
                    t,
                    &mut chain,
                    (&mut closed, &mut local, &mut parameters, &mut skolems),
                );
                self.lit_args.push(p);
                let walked = (self.pats.len() + chain.len()).saturating_sub(before) + 1;
                let crossed = (work + walked as u64) / POLL as u64 != work / POLL as u64;
                work = work.saturating_add(walked as u64);
                if crossed {
                    budget.poll(POLL as u64, self.held_bytes())?;
                }
            }
            let index = self.literals.len() as u32;
            self.literals.push(Literal {
                occ: o as u32,
                predicate,
                left: occ.left,
                args: (start, args.len() as u32),
                node: NONE,
            });
            self.occs[o].kind = Kind::Atom(index);
            self.by_predicate
                .entry(2 * u64::from(predicate) + u64::from(occ.left))
                .or_default()
                .push(index);
        }
        Ok(())
    }

    /// Returns the pattern of the arena's term `t` in an atom at
    /// occurrence `occ`: a bound variable becomes its binder's slot, or
    /// for a δ-formula its Skolem term. Walks on a stack of its own;
    /// `chain` holds the binders around `occ` found so far, innermost
    /// first, and grows as a bound variable reaches further.
    fn pattern(
        &mut self,
        formulas: &Formulas,
        loose: &[u32],
        occ: u32,
        t: FoTermId,
        chain: &mut Vec<u32>,
        (closed, local, parameters, skolems): Memo<'_>,
    ) -> u32 {
        let tables = formulas.first_order();
        let known = |u: FoTermId, closed: &[u32], local: &HashMap<FoTermId, u32>| {
            if loose[u.index()] == 0 {
                (closed[u.index()] != NONE).then(|| closed[u.index()])
            } else {
                local.get(&u).copied()
            }
        };
        // Each entry is a term and whether its arguments are done.
        let mut stack = vec![(t, false)];
        while let Some((u, done)) = stack.pop() {
            if known(u, closed, local).is_some() {
                continue;
            }
            let made = match tables.terms[u.index()] {
                FoTerm::Bound(index) => {
                    // The binder `index` binders out from the atom.
                    while chain.len() <= index as usize {
                        let b = match chain.last() {
                            Some(&inner) => self.occs[inner as usize].binder,
                            None => self.occs[occ as usize].binder,
                        };
                        chain.push(b);
                    }
                    let b = chain[index as usize];
                    match self.occs[b as usize].kind {
                        Kind::Gamma(slot) => self.pat(Pat::Var(slot)),
                        Kind::Delta(k) => self.skolem(k, skolems),
                        Kind::Atom(_)
                        | Kind::True
                        | Kind::False
                        | Kind::Not
                        | Kind::And
                        | Kind::Or
                        | Kind::Implies
                        | Kind::Iff
                        | Kind::Part { .. } => unreachable!("a binder occurrence is γ or δ"),
                    }
                }
                FoTerm::Eigen(_) | FoTerm::Meta(_) => {
                    let symbol = *parameters.entry(u).or_insert_with(|| {
                        self.symbols.push(Symbol::Parameter(u));
                        (self.symbols.len() - 1) as u32
                    });
                    self.pat(Pat::App(symbol, (0, 0)))
                }
                FoTerm::App(f, args) => {
                    let items = tables.list(args);
                    if !done {
                        stack.push((u, true));
                        stack.extend(items.iter().rev().map(|&a| (a, false)));
                        continue;
                    }
                    let start = self.pat_args.len() as u32;
                    for &a in items {
                        let p = known(a, closed, local).expect("arguments first");
                        self.pat_args.push(p);
                    }
                    self.pat(Pat::App(f.get(), (start, items.len() as u32)))
                }
            };
            if loose[u.index()] == 0 {
                closed[u.index()] = made;
            } else {
                local.insert(u, made);
            }
        }
        known(t, closed, local).expect("the term is made")
    }

    /// Adds a pattern and returns its index.
    fn pat(&mut self, pat: Pat) -> u32 {
        self.pats.push(pat);
        (self.pats.len() - 1) as u32
    }

    /// Returns the pattern of the Skolem term `k`: its symbol applied to
    /// the slots around its δ-formula, outermost first.
    fn skolem(&mut self, k: u32, skolems: &mut [u32]) -> u32 {
        if skolems[k as usize] != NONE {
            return skolems[k as usize];
        }
        let env = self.env(self.skolem_occ[k as usize]);
        self.symbols.push(Symbol::Skolem(k));
        let symbol = (self.symbols.len() - 1) as u32;
        let vars: Vec<u32> = env.iter().map(|&s| self.pat(Pat::Var(s))).collect();
        let start = self.pat_args.len() as u32;
        self.pat_args.extend_from_slice(&vars);
        let p = self.pat(Pat::App(symbol, (start, vars.len() as u32)));
        skolems[k as usize] = p;
        p
    }

    /// Builds the matrix from the occurrences, asking the stop and the
    /// memory bound every [`POLL`] occurrences and nodes.
    ///
    /// # Errors
    ///
    /// [`Refusal::Stopped`] and [`Refusal::Memory`] from `budget`.
    fn build(&mut self, budget: &mut Budget<'_>) -> Result<(), Error> {
        let mut ropes = Ropes::default();
        // Per occurrence, its clauses and its elements.
        let mut clauses = vec![0u32; self.occs.len()];
        let mut elements = vec![0u32; self.occs.len()];
        for o in (0..self.occs.len()).rev() {
            if o.is_multiple_of(POLL) {
                budget.poll(POLL as u64, self.held_bytes() + ropes.held_bytes())?;
            }
            let occ = self.occs[o];
            let [a, b] = occ.children;
            let pass = |x: u32| (clauses[x as usize], elements[x as usize]);
            // `None` is a closed part (no path), `Some(false)` an open one
            // (one empty path).
            let (alpha, beta, unit) = match (occ.kind, occ.left) {
                (Kind::Atom(_), _) => (false, false, None),
                (Kind::True, false) | (Kind::False, true) => (false, false, Some(true)),
                (Kind::True, true) | (Kind::False, false) => (false, false, Some(false)),
                (Kind::Not | Kind::Gamma(_) | Kind::Delta(_), _) => {
                    (clauses[o], elements[o]) = pass(a);
                    continue;
                }
                (Kind::And, true) | (Kind::Or | Kind::Implies, false) => (true, false, None),
                (Kind::And, false) | (Kind::Or | Kind::Implies, true) => (false, true, None),
                // An equivalence is the conjunction of its implications.
                (Kind::Iff, true) => (true, false, None),
                (Kind::Iff, false) => (false, true, None),
                (Kind::Part { .. }, false) => (true, false, None),
                (Kind::Part { .. }, true) => (false, true, None),
            };
            let (c, e) = if alpha {
                let c = ropes.concat(clauses[a as usize], clauses[b as usize]);
                // A matrix of one clause is that clause's elements.
                let e = match ropes.single(c) {
                    Some(Item::Clause(elements)) => elements,
                    _ => ropes.leaf(Item::Sub(c)),
                };
                (c, e)
            } else if beta {
                let e = ropes.concat(elements[a as usize], elements[b as usize]);
                // A clause of one matrix is that matrix's clauses.
                let c = match ropes.single(e) {
                    Some(Item::Sub(clauses)) => clauses,
                    _ => ropes.leaf(Item::Clause(e)),
                };
                (c, e)
            } else {
                match unit {
                    // No path: a clause of no element.
                    Some(true) => {
                        let e = ropes.empty();
                        (ropes.leaf(Item::Clause(e)), e)
                    }
                    // One empty path: no clause, and as an element a matrix
                    // of none.
                    Some(false) => {
                        let c = ropes.empty();
                        (c, ropes.leaf(Item::Sub(c)))
                    }
                    None => {
                        let Kind::Atom(l) = occ.kind else {
                            unreachable!("the remaining occurrence is an atom");
                        };
                        let e = ropes.leaf(Item::Literal(l));
                        (ropes.leaf(Item::Clause(e)), e)
                    }
                }
            };
            clauses[o] = c;
            elements[o] = e;
        }
        // The sequent is the disjunction of its formulas' matrices.
        let mut root = ropes.empty();
        for &r in &self.roots {
            root = ropes.concat(root, clauses[r as usize]);
        }
        // Lay the nodes out in preorder: each entry an item and its parent.
        let mut stack: Vec<(Item, u32)> = vec![(Item::Sub(root), NONE)];
        let mut parents: Vec<u32> = Vec::new();
        while let Some((item, parent)) = stack.pop() {
            let index = self.nodes.len() as u32;
            if (index as usize).is_multiple_of(POLL) {
                budget.poll(POLL as u64, self.held_bytes() + ropes.held_bytes())?;
            }
            let (shape, below): (Shape, Vec<Item>) = match item {
                Item::Sub(r) => (Shape::Matrix, ropes.items(r)),
                Item::Clause(r) => (Shape::Clause, ropes.items(r)),
                Item::Literal(l) => {
                    self.literals[l as usize].node = index;
                    (Shape::Literal(l), Vec::new())
                }
            };
            self.nodes.push(MNode {
                shape,
                parent,
                end: index + 1,
                children: (0, 0),
                positive: false,
                slots: (0, 0),
            });
            parents.push(parent);
            for &child in below.iter().rev() {
                stack.push((child, index));
            }
        }
        // Subtree ends, children lists and positive flags, bottom-up.
        let n = self.nodes.len();
        let mut counts = vec![0u32; n];
        for i in (1..n).rev() {
            let p = parents[i] as usize;
            self.nodes[p].end = self.nodes[p].end.max(self.nodes[i].end);
            counts[p] += 1;
        }
        let mut start = 0u32;
        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.children = (start, 0);
            start += counts[i];
        }
        self.children = vec![0; start as usize];
        for (i, &parent) in parents.iter().enumerate().skip(1) {
            let p = parent as usize;
            let (s, len) = self.nodes[p].children;
            self.children[(s + len) as usize] = i as u32;
            self.nodes[p].children.1 += 1;
        }
        for i in (0..n).rev() {
            let node = self.nodes[i];
            let kids = self.kids(i as u32);
            self.nodes[i].positive = match node.shape {
                Shape::Literal(l) => !self.literals[l as usize].left,
                Shape::Clause => kids.iter().all(|&k| self.nodes[k as usize].positive),
                Shape::Matrix => kids.iter().any(|&k| self.nodes[k as usize].positive),
            };
        }
        Ok(())
    }

    /// Returns the children of node `n`.
    pub(super) fn kids(&self, n: u32) -> &[u32] {
        let (s, len) = self.nodes[n as usize].children;
        &self.children[s as usize..(s + len) as usize]
    }

    /// Returns whether node `inner` lies in the subtree of node `outer`.
    pub(super) fn within(&self, inner: u32, outer: u32) -> bool {
        outer <= inner && inner < self.nodes[outer as usize].end
    }

    /// Records, for every clause, the slots its literals have and whether
    /// each is free there: a slot is free in a clause where every literal
    /// with that slot is in the clause or in another clause of a matrix
    /// around it (Definition 5), so that no strict clause ancestor of the
    /// clause has another element with a literal of that slot.
    fn free_slots(&mut self, budget: &mut Budget<'_>) -> Result<(), Error> {
        let n = self.nodes.len();
        // The bytes the lists below hold, which deep nests of quantifiers
        // make quadratic: each literal has every slot around it.
        let mut held = self.held_bytes();
        // Each list made is work as long as it is: polled by the node, a few
        // thousand nodes each with a list of thousands of slots asked the
        // stop once. A list kept is charged; one that lives for a clause is
        // work alone.
        let mut work = 0usize;
        let mut account = |budget: &mut Budget<'_>, made: usize, kept: usize| {
            held = held.saturating_add(kept.saturating_mul(size_of::<u32>()));
            let before = work / POLL;
            work = work.saturating_add(made.max(1));
            if work / POLL == before {
                Ok(())
            } else {
                budget.poll(POLL as u64, held)
            }
        };
        // The slots below each node, sorted.
        let mut below: Vec<Vec<u32>> = vec![Vec::new(); n];
        for i in (0..n).rev() {
            let set = match self.nodes[i].shape {
                Shape::Literal(l) => {
                    let mut env = self.env(self.literals[l as usize].occ);
                    env.sort_unstable();
                    env
                }
                // The children's lists joined and sorted once: a union per
                // child copied the growing list, quadratic in a node of
                // thousands of elements.
                Shape::Matrix | Shape::Clause => {
                    let mut set = Vec::new();
                    for &k in self.kids(i as u32) {
                        set.extend_from_slice(&below[k as usize]);
                        account(budget, below[k as usize].len(), 0)?;
                    }
                    set.sort_unstable();
                    set.dedup();
                    set
                }
            };
            account(budget, set.len(), set.len())?;
            below[i] = set;
        }
        // The slots no clause of a matrix may rename, top-down: those of
        // the other elements of every clause above it. How many of a
        // clause's elements have each slot tells which another element
        // has, where the unions of the elements after each one held a
        // number of slots quadratic in the clause's width.
        let mut counts = vec![0u32; self.slot_occ.len()];
        account(budget, counts.len(), counts.len())?;
        let mut barred: Vec<Vec<u32>> = vec![Vec::new(); n];
        for i in 0..n {
            if self.nodes[i].shape != Shape::Clause {
                continue;
            }
            let kids = self.kids(i as u32).to_vec();
            if !kids
                .iter()
                .any(|&k| self.nodes[k as usize].shape == Shape::Matrix)
            {
                continue;
            }
            let mut here: Vec<u32> = Vec::new();
            for &k in &kids {
                for &s in &below[k as usize] {
                    if counts[s as usize] == 0 {
                        here.push(s);
                    }
                    counts[s as usize] += 1;
                }
                account(budget, below[k as usize].len(), 0)?;
            }
            here.sort_unstable();
            for &k in &kids {
                if self.nodes[k as usize].shape != Shape::Matrix {
                    continue;
                }
                let own = &below[k as usize];
                let others: Vec<u32> = here
                    .iter()
                    .copied()
                    .filter(|&s| counts[s as usize] > u32::from(own.binary_search(&s).is_ok()))
                    .collect();
                let set = union(&barred[i], &others);
                account(budget, here.len().saturating_add(set.len()), 0)?;
                // The matrix's clauses inherit what is barred for it.
                for &c in self.kids(k) {
                    account(budget, set.len(), set.len())?;
                    barred[c as usize].clone_from(&set);
                }
            }
            for &s in &here {
                counts[s as usize] = 0;
            }
        }
        for i in 0..n {
            if self.nodes[i].shape != Shape::Clause {
                continue;
            }
            indexed(self.slots.len() + below[i].len(), Space::Member)?;
            let start = self.slots.len() as u32;
            for &s in &below[i] {
                let free = barred[i].binary_search(&s).is_err();
                self.slots.push((s, free));
            }
            self.nodes[i].slots = (start, below[i].len() as u32);
        }
        Ok(())
    }

    /// Returns the slots of clause `c`, sorted, each with whether it is
    /// free there.
    pub(super) fn clause_slots(&self, c: u32) -> &[(u32, bool)] {
        let (s, len) = self.nodes[c as usize].slots;
        &self.slots[s as usize..(s + len) as usize]
    }

    /// Returns whether the matrix has no slot: every copy of a clause is
    /// the clause.
    pub(super) fn is_ground(&self) -> bool {
        self.slot_occ.is_empty()
    }
}

/// Returns the union of two sorted lists.
pub(super) fn union(a: &[u32], b: &[u32]) -> Vec<u32> {
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
