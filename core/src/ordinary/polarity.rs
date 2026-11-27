// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Every subformula's polarity where a sequent fails: Plaisted and
//! Greenbaum's rule, which the classical SAT encoder and the normal form
//! named after them both follow.

use super::{Formulas, Node, NodeId};

/// The polarity of a formula that must be true where the sequent fails,
/// as a hypothesis must.
pub(super) const TRUE: u8 = 1;
/// The polarity of a formula that must be false where the sequent fails,
/// as a formula right of `⊢` must.
pub(super) const FALSE: u8 = 2;

/// Returns the polarity of every formula of the arena where the formulas
/// `left` are true and those of `right` false. `TRUE` marks a formula that
/// must be true there, `FALSE` one that must be false. A part takes its
/// parent's polarity, flipped under a negation and left of an
/// implication; the parts of an equivalence get both. A formula neither
/// side reaches gets `0`.
pub(super) fn polarities(formulas: &Formulas, left: &[NodeId], right: &[NodeId]) -> Vec<u8> {
    let mut polarity = vec![0u8; formulas.len()];
    for h in left {
        polarity[h.index()] |= TRUE;
    }
    for c in right {
        polarity[c.index()] |= FALSE;
    }
    // Every formula comes after its parts, so a pass from the last formula
    // to the first meets each before its parts.
    for id in (0..formulas.len()).rev() {
        let p = polarity[id];
        if p == 0 {
            continue;
        }
        let flipped = ((p & TRUE) << 1) | ((p & FALSE) >> 1);
        match formulas.node(NodeId(id as u32)) {
            Node::Atom(_) | Node::True | Node::False => {}
            Node::Not(a) => polarity[a.index()] |= flipped,
            Node::And(a, b) | Node::Or(a, b) => {
                polarity[a.index()] |= p;
                polarity[b.index()] |= p;
            }
            Node::Implies(a, b) => {
                polarity[a.index()] |= flipped;
                polarity[b.index()] |= p;
            }
            Node::Iff(a, b) => {
                polarity[a.index()] |= TRUE | FALSE;
                polarity[b.index()] |= TRUE | FALSE;
            }
            Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
        }
    }
    polarity
}
