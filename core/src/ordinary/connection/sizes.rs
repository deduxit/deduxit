// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! What the read-backs' instances add to the arena's terms, held to
//! `limits.terms` by the measure a document's reader takes: the terms the
//! atoms' arguments unfold to, summed over the atoms.
//!
//! An atom is keyed by its text, so building an instance writes every
//! argument out in full: a witness that shares its parts (`f(t, t)` bound
//! again and again) is exponentially longer written than stored. The
//! bound is asked before an instance is built, from what its atoms would
//! add (each found or not by the terms' ids, never by its text), and what
//! it did add is counted after.

use crate::Error;
use crate::first_order::{Args, FoTerm, FoTermId, Symbol};
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
    /// The atoms counted, by symbol and arguments.
    known: HashSet<(Symbol, Args)>,
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
            known: HashSet::default(),
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
        for &(symbol, args) in &tables.atoms[self.atoms..] {
            for a in tables.list(args) {
                self.held = self.held.saturating_add(self.sizes[a.index()]);
            }
            self.known.insert((symbol, args));
        }
        self.atoms = tables.atoms.len();
    }

    /// Refuses ([`Refusal::Terms`]) the instance of `body` by the terms
    /// `frame` (innermost first) where it would pass the bound: each atom
    /// of `body` with a variable of `frame` makes the atom of its instance,
    /// which counts where the arena does not hold it yet. Nothing is
    /// built; two atoms of `body` with one instance count twice.
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
        let mut written: HashMap<(FoTermId, u32), Written> = HashMap::default();
        let mut added = 0u64;
        let mut seen: HashSet<(NodeId, u32)> = HashSet::default();
        let mut stack = vec![(body, 0u32)];
        while let Some((n, depth)) = stack.pop() {
            if !seen.insert((n, depth)) {
                continue;
            }
            match formulas.node(n) {
                Node::Atom(a) => {
                    let Some(&(symbol, args)) = tables.atoms.get(a as usize) else {
                        continue;
                    };
                    let parts: Vec<Written> = tables
                        .list(args)
                        .iter()
                        .map(|&t| self.write(formulas, t, depth, frame, &mut written))
                        .collect();
                    if !parts.iter().any(|p| p.substituted) {
                        continue;
                    }
                    let found = parts
                        .iter()
                        .map(|p| p.found)
                        .collect::<Option<Vec<FoTermId>>>()
                        .and_then(|ids| tables_list(formulas, &ids))
                        .is_some_and(|list| self.known.contains(&(symbol, list)));
                    if !found {
                        for p in parts {
                            added = added.saturating_add(p.size);
                        }
                    }
                }
                Node::True | Node::False => {}
                Node::Not(a) => stack.push((a, depth)),
                Node::Forall(a) | Node::Exists(a) => stack.push((a, depth + 1)),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    stack.push((a, depth));
                    stack.push((b, depth));
                }
            }
        }
        let terms = self.held.saturating_add(added);
        if terms > limit {
            return Err(Error::Refused(Refusal::Terms { terms, limit }));
        }
        Ok(())
    }

    /// Returns what term `t` under `depth` binders of the body becomes in
    /// the instance by `frame`: its size, the term if the arena holds it,
    /// and whether a variable of `frame` was put in. `written` keeps what
    /// was found, so each shared term is walked once.
    fn write(
        &self,
        formulas: &Formulas,
        t: FoTermId,
        depth: u32,
        frame: &[FoTermId],
        written: &mut HashMap<(FoTermId, u32), Written>,
    ) -> Written {
        let tables = formulas.first_order();
        let mut stack = vec![(t, false)];
        while let Some((u, done)) = stack.pop() {
            if written.contains_key(&(u, depth)) {
                continue;
            }
            let value = match tables.terms[u.index()] {
                FoTerm::Bound(i) if i >= depth => {
                    let put = frame.get((i - depth) as usize).copied();
                    Written {
                        size: put.map_or(1, |p| self.sizes.get(p.index()).copied().unwrap_or(1)),
                        found: put,
                        substituted: true,
                    }
                }
                FoTerm::Bound(_) | FoTerm::Eigen(_) | FoTerm::Meta(_) => Written {
                    size: 1,
                    found: Some(u),
                    substituted: false,
                },
                FoTerm::App(f, args) => {
                    let parts = tables.list(args);
                    if !done {
                        stack.push((u, true));
                        stack.extend(parts.iter().map(|&p| (p, false)));
                        continue;
                    }
                    let mut value = Written {
                        size: 1,
                        found: None,
                        substituted: false,
                    };
                    let mut ids = Vec::with_capacity(parts.len());
                    let mut all = true;
                    for p in parts {
                        let w = written.get(&(*p, depth)).copied().unwrap_or(Written {
                            size: 1,
                            found: None,
                            substituted: false,
                        });
                        value.size = value.size.saturating_add(w.size);
                        value.substituted |= w.substituted;
                        match w.found {
                            Some(id) => ids.push(id),
                            None => all = false,
                        }
                    }
                    value.found = if value.substituted {
                        all.then(|| tables_term(formulas, f, &ids)).flatten()
                    } else {
                        Some(u)
                    };
                    value
                }
            };
            written.insert((u, depth), value);
        }
        written.get(&(t, depth)).copied().unwrap_or(Written {
            size: 1,
            found: None,
            substituted: false,
        })
    }
}

/// What a term of the body becomes in an instance.
#[derive(Clone, Copy, Debug)]
struct Written {
    /// Its size written out.
    size: u64,
    /// The term, where the arena holds it.
    found: Option<FoTermId>,
    /// Whether a variable of the frame was put in.
    substituted: bool,
}

/// Returns the argument list of `ids` if the arena holds it.
fn tables_list(formulas: &Formulas, ids: &[FoTermId]) -> Option<Args> {
    formulas.fo.as_deref()?.find_list(ids)
}

/// Returns the term `f(ids)` if the arena holds it.
fn tables_term(formulas: &Formulas, f: Symbol, ids: &[FoTermId]) -> Option<FoTermId> {
    let list = tables_list(formulas, ids)?;
    formulas.fo.as_deref()?.find_term(FoTerm::App(f, list))
}
