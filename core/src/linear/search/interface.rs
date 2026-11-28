// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! What the front door and every engine share: the [`Task`] an engine is
//! handed, the [`Answer`] it gives, and the trait [`Decide`] each engine
//! implements.

#[cfg(doc)]
use super::{Engine, NotTaken};
use super::{Options, Reason, Refutation, Statistics, Work, horn};
use crate::Error;
use crate::limits::{Limits, Refusal};
use crate::linear::fragment::{Fragment, Mode};
use crate::linear::nets::ProofStructure;
use crate::linear::occurrences::{Forest, OccId, Reading};
use crate::linear::proofs::{Node, NodeId, Proof};
use crate::memory;
use std::sync::OnceLock;

/// A goal as the engines are handed it: its occurrences of the forest,
/// the fragment and the mode it is searched in, and in intuitionistic
/// mode the forest's reading.
pub(crate) struct Task<'a> {
    /// The forest.
    pub(crate) forest: &'a Forest,
    /// The goal's occurrences. When the goal is the sequent itself, they
    /// are the roots in the forest's order.
    pub(crate) goal: &'a [OccId],
    /// The fragment searched in: the goal's own, or the one the options
    /// assert.
    pub(crate) fragment: Fragment,
    /// The mode.
    pub(crate) mode: Mode,
    /// The intuitionistic reading, in intuitionistic mode.
    pub(crate) reading: Option<&'a Reading<'a>>,
    /// Whether the goal is the sequent's roots.
    pub(crate) roots: bool,
    /// The Horn program the goal is, or `None` when it is none.
    ///
    /// The first to ask reads it: the dispatch's feature, the Horn
    /// engine's `admits` or its search. So a goal is read once.
    pub(super) program: OnceLock<Option<horn::Program>>,
}

/// What a search that keeps its proofs in an arena ended with: the
/// result, the arena, the counters and, for a first-order proof, its
/// instance table.
pub(crate) struct Finished {
    /// The node proving the goal, `None` for an unprovable goal, or why
    /// the search stopped.
    pub(crate) result: Result<Option<NodeId>, Reason>,
    /// The arena the node lives in.
    pub(crate) nodes: Vec<Node>,
    /// The counters.
    pub(crate) statistics: Statistics,
    /// The instance table of a first-order proof, whose members and
    /// witnesses the arena's nodes name; `None` for a propositional one.
    pub(crate) instances: Option<Box<crate::linear::Instances>>,
}

impl Finished {
    /// Returns the end of a search that gave up for `reason` before it
    /// found anything.
    pub(crate) fn gave_up(reason: Reason) -> Self {
        Self {
            result: Err(reason),
            nodes: Vec::new(),
            statistics: Statistics::default(),
            instances: None,
        }
    }
}

/// What an engine's search ended with: the result, its counters, the
/// proof net the net engine read its proof off, and the engine's own
/// refutation.
pub(crate) struct Answer {
    /// The proof, `None` for an unprovable goal, or why the search
    /// stopped.
    pub(crate) result: Result<Option<Proof>, Reason>,
    /// The counters.
    pub(crate) statistics: Statistics,
    /// The net found, from the net engine.
    pub(crate) net: Option<ProofStructure>,
    /// For an unprovable goal, why it is unprovable, where the engine
    /// knows more than that its search was exhaustive.
    pub(crate) refutation: Option<Refutation>,
}

impl Answer {
    /// Returns the answer of an engine that keeps its proofs as nodes of
    /// an arena.
    ///
    /// The proof is built in the arena's own vector. The build asks
    /// `stop` as it goes, and a stop there ends it as a stop of the
    /// search does.
    pub(crate) fn of_arena(
        forest: &Forest,
        Finished {
            result,
            nodes,
            statistics,
            instances,
        }: Finished,
        stop: &mut dyn FnMut(u64) -> bool,
    ) -> Self {
        let result = result.and_then(|root| {
            root.map(|root| match instances {
                // A table that the proof's ids no longer number has run
                // past its indices.
                Some(table) => Proof::with_instances(forest.clone(), *table, nodes, root)
                    .map_err(|_| Reason::IndexLimit),
                None => {
                    Proof::build(forest.clone(), nodes, root, &mut || stop(0)).map_err(
                        |e| match e {
                            Error::Refused(Refusal::Stopped { .. }) => Reason::Stopped,
                            e => panic!("the engine pushes premises before conclusions: {e}"),
                        },
                    )
                }
            })
            .transpose()
        });
        Self {
            result,
            statistics,
            net: None,
            refutation: None,
        }
    }
}

/// What every engine implements: which goals it takes, and how it decides
/// one.
///
/// An engine reads the options its variant of [`Engine`] names, and no
/// others.
pub(crate) trait Decide {
    /// Refuses a goal the engine does not decide.
    ///
    /// # Errors
    ///
    /// The error the options get when they force the engine there:
    /// [`Error::EngineRefused`] with the [`NotTaken`] that says why.
    fn admits(&self, task: &Task<'_>) -> Result<(), Error>;

    /// Decides the goal under the options, and returns the engine's
    /// answer.
    ///
    /// It polls `stop` with the units of work done since the last poll,
    /// which `work` adds up for every thread of the search. It charges
    /// what the search holds to `account`.
    ///
    /// # Errors
    ///
    /// [`Error::ThreadPool`] when the threads of a parallel search cannot
    /// start.
    fn decide(
        &self,
        task: &Task<'_>,
        options: &Options,
        limits: &Limits,
        account: &memory::Account,
        work: &Work,
        stop: &mut dyn FnMut(u64) -> bool,
    ) -> Result<Answer, Error>;
}
