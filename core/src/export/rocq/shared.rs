// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The formulas a proof term names, each written once.
//!
//! A proof term of deduxit's library names formulas, not occurrences, so a
//! formula deep in a sequent is written again at every rule above it: the
//! text of `!A ⊢ A ⊗ … ⊗ A` grows with the square of the formula. Here the
//! formulas are hash-consed as they are written (equal text is one piece),
//! and every compound formula that the term would write at two or more
//! places is bound once, `let f'1 := … in`, inside the term handed to the
//! soundness theorem. The statement stays written out, so that a reader
//! checks it against the calculus alone. `vm_compute` reduces the `let`s
//! with the rest of the term.

use crate::hash::HashMap;
use std::fmt::Write;

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
        self.named[id as usize] += 1;
    }

    /// Decides which pieces are bound, once every name is recorded: each
    /// compound piece written at two or more places. A piece is written
    /// once per name the term gives it and once per operand place in a
    /// written piece built from it, whether that piece is bound (its `let`
    /// is written once) or written in place (at the one place that names
    /// it). The pieces are numbered in their order, operands first, so
    /// that every `let` names only earlier ones.
    pub(super) fn bind(&mut self) {
        let mut uses = self.named.clone();
        for id in (0..self.pieces.len()).rev() {
            if uses[id] == 0 {
                continue;
            }
            if let Piece::Node(_, first, second) = self.pieces[id] {
                uses[first as usize] += 1;
                if let Some(second) = second {
                    uses[second as usize] += 1;
                }
            }
        }
        self.names = vec![0; self.pieces.len()];
        for (id, piece) in self.pieces.iter().enumerate() {
            if uses[id] >= 2 && matches!(piece, Piece::Node(..)) {
                self.bound += 1;
                self.names[id] = self.bound;
            }
        }
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
