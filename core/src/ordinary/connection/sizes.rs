// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! What the read-backs' instances add to the arena's terms, held to
//! `limits.terms` by the measure a document's reader takes: the terms the
//! atoms' arguments unfold to, summed over the atoms.
//!
//! An atom is keyed by its text, so building an instance writes every
//! argument out in full: a witness that shares its parts (`f(t, t)` bound
//! again and again) is exponentially longer written than stored. The
//! bound is asked before an instance is built, from an upper bound of what
//! it adds, and what it did add is counted after.

use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::{HashMap, HashSet};
use crate::limits::Refusal;
use crate::ordinary::{Formulas, Node, NodeId};

/// The arena's terms as the bound measures them.
#[derive(Debug)]
pub(super) struct TermBudget {
    /// Per term of the arena, how many terms it unfolds to, for the terms
    /// counted so far (a term comes after its arguments).
    sizes: Vec<u64>,
    /// The atoms' arguments unfolded, summed over the atoms counted.
    held: u64,
    /// The atoms counted.
    atoms: usize,
    /// The bound, `None` for none.
    limit: Option<u64>,
}

impl TermBudget {
    /// The measure of `formulas`' atoms, under `limit`.
    pub(super) fn new(formulas: &Formulas, limit: Option<u64>) -> Self {
        let mut budget = Self {
            sizes: Vec::new(),
            held: 0,
            atoms: 0,
            limit,
        };
        budget.count(formulas);
        budget
    }

    /// Counts the atoms the arena gained since the last count.
    pub(super) fn count(&mut self, formulas: &Formulas) {
        let tables = formulas.first_order();
        for t in &tables.terms[self.sizes.len()..] {
            let size = match *t {
                FoTerm::Bound(_) | FoTerm::Eigen(_) | FoTerm::Meta(_) => 1,
                FoTerm::App(_, args) => tables
                    .list(args)
                    .iter()
                    .fold(1u64, |sum, a| sum.saturating_add(self.sizes[a.index()])),
            };
            self.sizes.push(size);
        }
        for &(_, args) in &tables.atoms[self.atoms..] {
            for a in tables.list(args) {
                self.held = self.held.saturating_add(self.sizes[a.index()]);
            }
        }
        self.atoms = tables.atoms.len();
    }

    /// Refuses ([`Refusal::Terms`]) the instance of `body` by the terms
    /// `frame` where it could pass the bound: each atom of `body` with a
    /// bound variable may add one atom, its bound variables written as the
    /// largest term of `frame`. Nothing is built.
    pub(super) fn check(
        &mut self,
        formulas: &Formulas,
        body: NodeId,
        frame: &[FoTermId],
    ) -> Result<(), Error> {
        let Some(limit) = self.limit else {
            return Ok(());
        };
        self.count(formulas);
        let tables = formulas.first_order();
        let widest = frame
            .iter()
            .map(|t| self.sizes.get(t.index()).copied().unwrap_or(1))
            .max()
            .unwrap_or(1);
        // Per term: its size with every bound variable the widest term,
        // and whether it holds one.
        let mut written: HashMap<FoTermId, (u64, bool)> = HashMap::default();
        let mut added = 0u64;
        let mut seen: HashSet<NodeId> = HashSet::default();
        let mut stack = vec![body];
        while let Some(n) = stack.pop() {
            if !seen.insert(n) {
                continue;
            }
            match formulas.node(n) {
                Node::Atom(a) => {
                    let Some(&(_, args)) = tables.atoms.get(a as usize) else {
                        continue;
                    };
                    let parts: Vec<(u64, bool)> = tables
                        .list(args)
                        .iter()
                        .map(|&t| write(tables, t, widest, &mut written))
                        .collect();
                    if parts.iter().any(|&(_, loose)| loose) {
                        for (size, _) in parts {
                            added = added.saturating_add(size);
                        }
                    }
                }
                Node::True | Node::False => {}
                Node::Not(a) | Node::Forall(a) | Node::Exists(a) => stack.push(a),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    stack.push(a);
                    stack.push(b);
                }
            }
        }
        let terms = self.held.saturating_add(added);
        if terms > limit {
            return Err(Error::Refused(Refusal::Terms { terms, limit }));
        }
        Ok(())
    }
}

/// Returns the size of term `t` with every bound variable written as a
/// term of size `widest`, and whether it holds a bound variable; `written`
/// keeps what was counted, so each shared term is counted once.
fn write(
    tables: &crate::first_order::FirstOrder<NodeId>,
    t: FoTermId,
    widest: u64,
    written: &mut HashMap<FoTermId, (u64, bool)>,
) -> (u64, bool) {
    let mut stack = vec![(t, false)];
    while let Some((u, done)) = stack.pop() {
        if written.contains_key(&u) {
            continue;
        }
        match tables.terms[u.index()] {
            FoTerm::Bound(_) => {
                written.insert(u, (widest, true));
            }
            FoTerm::Eigen(_) | FoTerm::Meta(_) => {
                written.insert(u, (1, false));
            }
            FoTerm::App(_, args) => {
                let parts = tables.list(args);
                if done {
                    let value = parts.iter().fold((1u64, false), |(size, loose), p| {
                        let (s, l) = written.get(p).copied().unwrap_or((1, false));
                        (size.saturating_add(s), loose || l)
                    });
                    written.insert(u, value);
                } else {
                    stack.push((u, true));
                    stack.extend(parts.iter().map(|&p| (p, false)));
                }
            }
        }
    }
    written.get(&t).copied().unwrap_or((1, false))
}
