// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The formulas a proof term names, each written once.
//!
//! A proof term of deduxit's library names formulas, not occurrences, so a
//! formula deep in a sequent is written again at every rule above it: the
//! text of `!A ⊢ A ⊗ … ⊗ A` grows with the square of the formula. Here the
//! formulas are hash-consed as they are written (equal text is one piece),
//! and the largest compound formulas that the term would write at two or
//! more places are bound once, `let f'1 := … in`, inside the term handed to
//! the soundness theorem. The statement stays written out, so that a
//! reader checks it against the calculus alone. `vm_compute` reduces the
//! `let`s with the rest of the term.
//!
//! A `let` is not free: Rocq's `apply` elaborates every constructor of the
//! term in a context that holds every `let` around it, so its time grows
//! with the constructors written times the `let`s in scope, and Rocq's
//! stack overflows between 3 000 and 5 000 nested `let`s. So as many of the
//! largest shared formulas are bound as make the estimate least (the
//! constructors written, times one more hundredth per `let`, the cost
//! measured on balanced trees of formulas), and never more than
//! [`MOST_LETS`].

use crate::hash::HashMap;
use std::fmt::Write;

/// The most `let`s a term binds, well below the nesting at which Rocq's
/// stack overflows.
const MOST_LETS: usize = 1024;

/// The `let`s whose cost in Rocq's elaboration equals that of writing every
/// constructor of the term once more: each `let` in scope adds about a
/// hundredth to the cost of each constructor (measured on balanced trees
/// of formulas, 256 to 2 048 leaves).
const LETS_PER_COPY: u64 = 100;

/// A formula as the term writes it: a leaf's whole text, or a constructor
/// and the pieces of its operands.
#[derive(Clone, PartialEq, Eq, Hash)]
enum Piece {
    /// A leaf's text (`one`, `var "A"`, `var "p" [eigen 1]`), and whether
    /// it is an application, which takes brackets as an argument.
    Leaf(String, bool),
    /// A constructor with one operand or two.
    Node(&'static str, u32, Option<u32>),
}

/// The pieces of every formula a term names, and which are bound.
#[derive(Default)]
pub(super) struct Shared {
    /// The pieces, every operand before the pieces built from it.
    pieces: Vec<Piece>,
    /// Each piece's index.
    index: HashMap<Piece, u32>,
    /// How many times the term names each piece directly.
    named: Vec<u32>,
    /// Per piece, the number of its `let` from 1, or 0 where it is
    /// written in place.
    names: Vec<u32>,
    /// How many pieces are bound.
    bound: u32,
}

impl Shared {
    /// Returns the piece of a leaf with this text.
    pub(super) fn leaf(&mut self, text: String, application: bool) -> u32 {
        self.intern(Piece::Leaf(text, application))
    }

    /// Returns the piece of the constructor `name` over one operand or two.
    pub(super) fn node(&mut self, name: &'static str, first: u32, second: Option<u32>) -> u32 {
        self.intern(Piece::Node(name, first, second))
    }

    /// Returns the index of `piece`, adding it where it is new.
    fn intern(&mut self, piece: Piece) -> u32 {
        if let Some(&id) = self.index.get(&piece) {
            return id;
        }
        let id = u32::try_from(self.pieces.len()).expect("fewer pieces than occurrences");
        self.pieces.push(piece.clone());
        self.index.insert(piece, id);
        self.named.push(0);
        id
    }

    /// Records that the term names the piece `id` once more.
    pub(super) fn name(&mut self, id: u32) {
        let named = &mut self.named[id as usize];
        *named = named.saturating_add(1);
    }

    /// Decides which pieces are bound, once every name is recorded.
    ///
    /// The candidates are the compound pieces written at two or more places
    /// even when every candidate is bound, largest first (by the
    /// constructors of their whole text). Of the first `0, 1, 2, 4, …`
    /// candidates and all of them, up to [`MOST_LETS`], it binds the number
    /// whose estimate is least: the constructors the term then writes, times
    /// one more [`LETS_PER_COPY`]th per `let`. The pieces are numbered in
    /// their order, operands first, so that every `let` names only earlier
    /// ones.
    pub(super) fn bind(&mut self) {
        let count = self.pieces.len();
        let none = vec![false; count];
        let all: Vec<bool> = self
            .pieces
            .iter()
            .map(|piece| matches!(piece, Piece::Node(..)))
            .collect();
        let uses = self.writes(&all);
        // The constructors of each piece's whole text, operands first.
        let mut size = vec![0u64; count];
        for (id, piece) in self.pieces.iter().enumerate() {
            size[id] = match *piece {
                Piece::Leaf(..) => 1,
                Piece::Node(_, first, second) => second
                    .map_or(0, |second| size[second as usize])
                    .saturating_add(size[first as usize])
                    .saturating_add(1),
            };
        }
        let mut candidates: Vec<usize> =
            (0..count).filter(|&id| all[id] && uses[id] >= 2).collect();
        candidates.sort_by_key(|&id| (std::cmp::Reverse(size[id]), id));
        candidates.truncate(MOST_LETS);
        let mut best = (self.cost(&none, 0), 0);
        let mut bound = none;
        let mut taken = 0;
        let mut next = 1;
        while taken < candidates.len() {
            let to = next.min(candidates.len());
            for &id in &candidates[taken..to] {
                bound[id] = true;
            }
            taken = to;
            let cost = self.cost(&bound, taken);
            if cost < best.0 {
                best = (cost, taken);
            }
            next *= 2;
        }
        let mut chosen = vec![false; count];
        for &id in &candidates[..best.1] {
            chosen[id] = true;
        }
        self.names = vec![0; count];
        for (id, &bound) in chosen.iter().enumerate() {
            if bound {
                self.bound += 1;
                self.names[id] = self.bound;
            }
        }
    }

    /// Returns how many times the term writes each piece when the pieces
    /// `bound` are bound: once per name the term gives it, and per operand
    /// place in a piece built from it, once if that piece is bound (its
    /// `let` is written once) and as often as it is written otherwise.
    fn writes(&self, bound: &[bool]) -> Vec<u64> {
        let mut writes: Vec<u64> = self.named.iter().map(|&n| u64::from(n)).collect();
        for id in (0..self.pieces.len()).rev() {
            if writes[id] == 0 {
                continue;
            }
            let each = if bound[id] { 1 } else { writes[id] };
            if let Piece::Node(_, first, second) = self.pieces[id] {
                for operand in std::iter::once(first).chain(second) {
                    let w = &mut writes[operand as usize];
                    *w = w.saturating_add(each);
                }
            }
        }
        writes
    }

    /// Returns the estimated cost in Rocq of the term with the pieces
    /// `bound` bound, `lets` of them: the constructors it writes (a bound
    /// piece's once, in its `let`), times one more [`LETS_PER_COPY`]th per
    /// `let`.
    fn cost(&self, bound: &[bool], lets: usize) -> u64 {
        let writes = self.writes(bound);
        let constructors = writes.iter().zip(bound).fold(0u64, |sum, (&w, &b)| {
            sum.saturating_add(if b { 1 } else { w })
        });
        constructors.saturating_mul(LETS_PER_COPY + lets as u64) / LETS_PER_COPY
    }

    /// Returns whether any piece is bound.
    pub(super) const fn binds(&self) -> bool {
        self.bound > 0
    }

    /// Returns the bound pieces in their order, each with its number.
    pub(super) fn lets(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.names
            .iter()
            .enumerate()
            .filter(|&(_, &k)| k > 0)
            .map(|(id, &k)| (k, id as u32))
    }

    /// Writes the piece `id`, a bound one as its name unless it is `whole`,
    /// in brackets if `argument` is set and it is an application; on a
    /// stack of its own, since a formula may nest as deep as it is long.
    pub(super) fn write(&self, out: &mut String, id: u32, argument: bool, whole: bool) {
        /// What the walk has yet to write.
        enum Item {
            /// A piece, and whether it is an argument of a constructor.
            Piece(u32, bool),
            /// Text closing or separating.
            Text(&'static str),
        }
        let mut stack = vec![Item::Piece(id, argument)];
        let mut first = whole;
        while let Some(item) = stack.pop() {
            let (id, argument) = match item {
                Item::Text(text) => {
                    out.push_str(text);
                    continue;
                }
                Item::Piece(id, argument) => (id, argument),
            };
            let top = std::mem::take(&mut first);
            let name = self.names.get(id as usize).copied().unwrap_or(0);
            if name > 0 && !top {
                write!(out, "f'{name}").expect("a string takes the text");
                continue;
            }
            match &self.pieces[id as usize] {
                Piece::Leaf(text, application) => {
                    let brackets = argument && *application;
                    if brackets {
                        out.push('(');
                    }
                    out.push_str(text);
                    if brackets {
                        out.push(')');
                    }
                }
                &Piece::Node(constructor, operand, second) => {
                    if argument {
                        out.push('(');
                        stack.push(Item::Text(")"));
                    }
                    out.push_str(constructor);
                    if let Some(second) = second {
                        stack.push(Item::Piece(second, true));
                        stack.push(Item::Text(" "));
                    }
                    stack.push(Item::Piece(operand, true));
                    out.push(' ');
                }
            }
        }
    }
}
