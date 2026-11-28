// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The clauses and their terms written as text.

use super::Clause;
use crate::first_order::{FoTerm, FoTermId};
use crate::ordinary::{Formulas, Node};

/// Writes the term `t`, each metavariable `k` as `name(k)`.
fn write_term(formulas: &Formulas, t: FoTermId, out: &mut String, name: &dyn Fn(u32) -> String) {
    let written = write_term_to(formulas, t, out, name);
    debug_assert!(written.is_ok(), "a string takes every write");
}

/// Writes the term `t` into `out`, each metavariable `k` as `name(k)`,
/// ending at the first write `out` refuses.
pub(super) fn write_term_to<W: std::fmt::Write>(
    formulas: &Formulas,
    t: FoTermId,
    out: &mut W,
    name: &dyn Fn(u32) -> String,
) -> std::fmt::Result {
    formulas.first_order().write_with(
        out,
        t,
        &mut |o: &mut W, symbol: &str| o.write_str(symbol),
        &mut |o: &mut W, index| write!(o, "#{index}"),
        &mut |o: &mut W, other| match other {
            FoTerm::Meta(k) => o.write_str(&name(k)),
            // A parameter of the arena, by the name a proof prints it
            // with.
            FoTerm::Eigen(e) => {
                let eigens = formulas.eigen_names();
                match eigens.binary_search_by_key(&e.get(), |&(k, _)| k) {
                    Ok(at) => o.write_str(&eigens[at].1),
                    Err(_) => o.write_char('?'),
                }
            }
            FoTerm::Bound(_) | FoTerm::App(..) => Ok(()),
        },
    )
}

/// The most bytes [`Clauses::bindings_text`](super::Clauses::bindings_text) writes of a binding's term.
pub(super) const BINDING_TEXT: usize = 1 << 12;

/// Text that takes at most `room` bytes and refuses the write that would
/// pass them, keeping what of it fits.
pub(super) struct Capped {
    /// The text so far.
    pub(super) text: String,
    /// The most bytes it takes.
    pub(super) room: usize,
}

impl std::fmt::Write for Capped {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let left = self.room.saturating_sub(self.text.len());
        if s.len() <= left {
            self.text.push_str(s);
            return Ok(());
        }
        let mut end = left;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        self.text.push_str(&s[..end]);
        Err(std::fmt::Error)
    }
}

/// Returns `clause` as text, its variables `prefix` and their numbers.
pub(super) fn clause_text(formulas: &Formulas, clause: &Clause, prefix: &str) -> String {
    if clause.literals.is_empty() {
        return "□".to_owned();
    }
    let name = |k: u32| format!("{prefix}{}", k + 1);
    let tables = formulas.first_order();
    let mut text = String::new();
    for (n, l) in clause.literals.iter().enumerate() {
        if n > 0 {
            text.push_str(" ∨ ");
        }
        if !l.positive {
            text.push('¬');
        }
        let Node::Atom(a) = formulas.node(l.atom) else {
            continue;
        };
        match tables.atoms.get(a as usize) {
            Some(&(p, args)) => {
                let name_of = &tables.symbols[p.index()].0;
                let args = tables.list(args);
                if args.len() == 2 && name_of == "=" {
                    write_term(formulas, args[0], &mut text, &name);
                    text.push_str(" = ");
                    write_term(formulas, args[1], &mut text, &name);
                    continue;
                }
                text.push_str(name_of);
                if !args.is_empty() {
                    text.push('(');
                    for (k, &t) in args.iter().enumerate() {
                        if k > 0 {
                            text.push_str(", ");
                        }
                        write_term(formulas, t, &mut text, &name);
                    }
                    text.push(')');
                }
            }
            None => text.push_str(formulas.atom_name(a)),
        }
    }
    text
}
