// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The terms that formulas reach, each once: the one walk that the
//! checker's eigenvariable condition, the goal state's counts, Herbrand's
//! universe and witnesses, resolution's door and the countermodels' checks
//! each read.

use super::{Formulas, Node, NodeId};
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::HashSet;

/// The terms some formulas hold as atoms' arguments, and every term inside
/// one, each once, as [`Formulas::terms_in`] returns them.
pub(crate) struct TermsIn<'a> {
    /// The arena the formulas and terms are of.
    formulas: &'a Formulas,
    /// The nodes the formulas reach, each once.
    nodes: HashSet<NodeId>,
    /// The terms still to visit, the last first.
    terms: Vec<FoTermId>,
    /// The terms visited.
    seen: HashSet<FoTermId>,
}

impl Formulas {
    /// Returns the terms the formulas `roots` hold as atoms' arguments,
    /// the terms `extra`, and every term inside one, each once. It walks on
    /// stacks of its own, each shared node and term once: the nodes when it
    /// is made, the terms as it is iterated, so a search that ends at the
    /// first hit visits no more terms. The last argument list met comes
    /// first: Herbrand's universe keeps that order for its constants and
    /// function symbols, which numbers its instances and atoms.
    pub(crate) fn terms_in(
        &self,
        roots: impl IntoIterator<Item = NodeId>,
        extra: impl IntoIterator<Item = FoTermId>,
    ) -> TermsIn<'_> {
        let tables = self.first_order();
        let mut nodes = HashSet::default();
        let mut stack: Vec<NodeId> = roots.into_iter().collect();
        let mut terms: Vec<FoTermId> = extra.into_iter().collect();
        while let Some(k) = stack.pop() {
            if !nodes.insert(k) {
                continue;
            }
            match self.node(k) {
                Node::Atom(a) => {
                    if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                        terms.extend_from_slice(tables.list(args));
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
        TermsIn {
            formulas: self,
            nodes,
            terms,
            seen: HashSet::default(),
        }
    }
}

impl TermsIn<'_> {
    /// Returns the nodes the formulas reach, each once, in no order.
    pub(crate) fn nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.nodes.iter().copied()
    }
}

impl Iterator for TermsIn<'_> {
    type Item = FoTermId;

    fn next(&mut self) -> Option<FoTermId> {
        let tables = self.formulas.first_order();
        while let Some(t) = self.terms.pop() {
            if !self.seen.insert(t) {
                continue;
            }
            if let FoTerm::App(_, args) = tables.terms[t.index()] {
                self.terms.extend_from_slice(tables.list(args));
            }
            return Some(t);
        }
        None
    }
}
