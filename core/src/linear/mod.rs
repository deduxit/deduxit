// linlog © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The linear suite: sequents of classical and intuitionistic linear
//! logic, their fragments and modes, the occurrence forest, proofs and
//! their checker, proof nets, and the search that decides a sequent and
//! returns a checked proof or a refutation. The items a caller starts
//! from are re-exported here.

#[cfg(feature = "parse")]
pub mod families;
/// Fragments of linear logic, modes of proof search, and fragment detection.
pub mod fragment;
#[cfg(feature = "parse")]
pub mod lltp;
#[cfg(feature = "parse")]
pub mod mist;
pub mod nets;
/// The occurrence forest of a sequent and sets over it.
pub mod occurrences;
/// Parsing sequents from text.
#[cfg(feature = "parse")]
mod parse;
pub mod proofs;
pub mod search;
/// Sequents and the terms they are built from.
pub mod sequents;

pub use fragment::{Fragment, Mode, Order};
pub use nets::{
    Criterion, MellStructure, NetError, Parts, Placed, Placement, ProofStructure, VertexId,
};
pub use occurrences::{
    ClassicalSequent, Forest, FrameId, IllSequent, IllSyntax, InputOptions, Instances, Member,
    OccId, Reading, ShapeError, Side, Sign, Written,
};
pub use proofs::{
    Branch, CheckError, ClassicalRule, ClassicalSides, Derivation, Division, InfId, Inference,
    Named, Node, NodeId, Proof, Rule, Size, TwoSided, ViewOptions,
};
#[cfg(feature = "interactive")]
pub use proofs::{CutFormula, GoalId, Interactive, Step, StepError};
pub use search::{
    Bias, Counter, Disproof, Engine, Goal, NotTaken, Options, Outcome, Reason, Refutation,
    Statistics, Verdict, prove, prove_goal, prove_within,
};
pub use sequents::{Atom, Eigen, Formula, Kind, Sequent, Term, TermId};
