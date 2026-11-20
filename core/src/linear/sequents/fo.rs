// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The first-order tables of a sequent: its symbols, its terms, their
//! argument lists, each atom as a predicate applied to arguments, and the
//! names of its bound variables. A propositional sequent has empty tables
//! and pays nothing for them.

pub use crate::first_order::{Args, Eigen, FoTerm, FoTermId, Symbol, SymbolFault};

use super::TermId;

/// The first-order tables of a linear sequent, its binders named by term.
pub(crate) type FirstOrder = crate::first_order::FirstOrder<TermId>;

/// The builder of a linear sequent's first-order tables.
pub(crate) type Interner = crate::first_order::Interner<TermId>;
