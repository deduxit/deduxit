// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The dispatch: which engine decides a goal, by the table
//! [`DISPATCH`](dispatch::DISPATCH), and the turns the inverse method
//! takes beside it.

use super::{Answer, Counted, Engine, Options, Reason, Task, Work, horn};
use crate::Error;
use crate::limits::{Limits, Phase, Progress};
use crate::linear::fragment::{Fragment, Mode, Order};
use crate::linear::occurrences::{Forest, OccId};
use crate::memory;

/// Returns the engine the dispatch picks for a goal: that of the first
/// row of [`DISPATCH`] that takes it.
///
/// # Errors
///
/// [`Error::NoEngine`] when no row takes the goal, which only a mode or a
/// fragment that no row names would meet.
pub(super) fn dispatch(task: &Task<'_>) -> Result<Engine, Error> {
    DISPATCH
        .iter()
        .find(|row| row.takes(task))
        .map(|row| row.engine)
        .ok_or(Error::NoEngine {
            fragment: task.fragment,
            mode: task.mode,
        })
}

/// Returns whether the inverse method takes turns beside the engine of a
/// search.
///
/// It does when the dispatch's row says so ([`Row::probe`]), no engine is
/// forced, the search runs on one thread, and the inverse method admits
/// the goal. So a race's pool searches without turns.
pub(super) fn probes(task: &Task<'_>, options: &Options) -> bool {
    #[cfg(feature = "parallel")]
    let alone = options.threads() == 1;
    #[cfg(not(feature = "parallel"))]
    let alone = true;
    alone
        && options.engine.is_none()
        && DISPATCH
            .iter()
            .find(|row| row.takes(task))
            .is_some_and(|row| row.probe)
        && Engine::Inverse.implementation().admits(task).is_ok()
}

/// The dispatch, read from the first row down. For each fragment, mode
/// and feature of a goal, it names the engine a measurement shows fastest
/// there. The documentation of [`Engine`] tabulates the rows with the
/// measurements. The last two rows take every goal, each in its own
/// modes.
pub(super) const DISPATCH: [Row; 8] = [
    Row {
        fragment: Fragment::MLL,
        modes: Modes::Lambek,
        feature: Feature::Any,
        engine: Engine::Essential,
        probe: false,
    },
    Row {
        fragment: Fragment::MLL,
        modes: Modes::Cyclic,
        feature: Feature::Any,
        engine: Engine::Net,
        probe: false,
    },
    Row {
        fragment: Fragment::ADDITIVE,
        modes: Modes::Any,
        feature: Feature::TwoFormulas,
        engine: Engine::Additive,
        probe: false,
    },
    Row {
        fragment: Fragment::MELL,
        modes: Modes::Any,
        feature: Feature::PetriNet,
        engine: Engine::Horn,
        probe: false,
    },
    Row {
        fragment: Fragment::LL1,
        modes: Modes::Intuitionistic,
        feature: Feature::Exponentials,
        engine: Engine::TwoSided,
        probe: true,
    },
    Row {
        fragment: Fragment::LL1,
        modes: Modes::Intuitionistic,
        feature: Feature::Any,
        engine: Engine::TwoSided,
        probe: false,
    },
    Row {
        fragment: Fragment::LL1,
        modes: Modes::Mix,
        feature: Feature::Any,
        engine: Engine::Focus,
        probe: true,
    },
    Row {
        fragment: Fragment::LL1,
        modes: Modes::Classical,
        feature: Feature::Any,
        engine: Engine::Focus,
        probe: false,
    },
];

/// The work a row's engine does alone before the inverse method takes its
/// first turn beside it, where the row gives turns ([`Row::probe`]).
///
/// It is in the dispatched engine's units, with every thread of the
/// search counted but the turns. On the intuitionistic rows measured, it
/// is some hundredths of a second. A goal decided within it is decided as
/// without the turns.
pub(super) const PROBE_AFTER: u64 = 1 << 24;

/// The share of the engine's own work that a turn of the inverse method
/// may do: a turn does at most the engine's work over this, in the
/// inverse method's units.
///
/// A turn comes each time the engine's work has doubled, and starts
/// afresh. So the turns together do at most a sixteenth of the engine's
/// work. A share of 16 makes the turns an eighth, which takes a row the
/// engine proves in 54 s of a 60 s limit past the limit.
pub(super) const TURN_SHARE: u64 = 32;

#[cfg(test)]
thread_local! {
    /// The head start and the turns' share on a test's thread. A test may
    /// set them other than [`PROBE_AFTER`] and [`TURN_SHARE`], to see
    /// turns on small goals. A share of 0 lets a turn run to its end.
    pub(super) static TURN: std::cell::Cell<(u64, u64)> = const { std::cell::Cell::new((PROBE_AFTER, TURN_SHARE)) };
}

/// Returns the head start before the inverse method's first turn and the
/// share of the engine's work a turn may do: [`PROBE_AFTER`] and
/// [`TURN_SHARE`], or in a test what its thread set.
pub(super) fn turn_bounds() -> (u64, u64) {
    #[cfg(test)]
    {
        TURN.with(std::cell::Cell::get)
    }
    #[cfg(not(test))]
    {
        (PROBE_AFTER, TURN_SHARE)
    }
}

/// The inverse method's turns beside an engine's search ([`Row::probe`]).
///
/// A turn comes each time the engine's own work has doubled past the head
/// start. Each turn runs the inverse method from its start, and does at
/// most a share of that work. So the turns together do at most twice the
/// last one's share.
pub(super) struct Turns {
    /// The work the turns did.
    pub(super) spent: u64,
    /// The share of the engine's work a turn may do: [`TURN_SHARE`], or a
    /// test's (0 lets a turn run to its end).
    pub(super) share: u64,
    /// What the turn that decided answered, or `None` while none has.
    pub(super) answer: Option<Result<Answer, Error>>,
}

impl Turns {
    /// Takes the turn due when the search has done `done` units, the
    /// turns' included.
    ///
    /// It runs the inverse method on the task within its share; the caller
    /// asked [`probes`] first. It counts the turn's work in `mine`, and
    /// asks `stop` and the bound on work at the turn's polls.
    ///
    /// Returns whether the engine must stop, and the work at which the
    /// next turn is due. The engine must stop when the turn decided (the
    /// turn's answer is kept), or the caller's stop or the bound on work
    /// fired. No turn is due (`u64::MAX`) when the turn met the memory
    /// bound or an index limit, which a longer turn would meet sooner.
    ///
    /// It is kept apart from the poll, which it leaves one comparison.
    /// Inlined there, it makes every poll some seven instructions dearer.
    #[cold]
    #[inline(never)]
    #[expect(
        clippy::too_many_arguments,
        reason = "the search's task, settings, account, work and stop, which the turn shares"
    )]
    pub(super) fn take(
        &mut self,
        done: u64,
        task: &Task<'_>,
        options: &Options,
        limits: &Limits,
        account: &memory::Account,
        work: &Work,
        mine: &mut Counted<'_>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> (bool, u64) {
        let own = done.saturating_sub(self.spent);
        let budget = own.checked_div(self.share).unwrap_or(u64::MAX);
        let mut turn = 0u64;
        let mut ended = false;
        let answer = Engine::Inverse.implementation().decide(
            task,
            options,
            limits,
            account,
            work,
            &mut |units| {
                turn = turn.saturating_add(units);
                let done = mine.add(units);
                ended = stop(Progress::new(Phase::Search, units, done)) || mine.passed();
                ended || turn >= budget
            },
        );
        self.spent = self.spent.saturating_add(turn);
        // A decision answers, and so does an error. The caller's stop or
        // the bound on work ends the engine too. Otherwise what the turn
        // kept goes, and the engine goes on.
        let decided = !matches!(answer, Ok(Answer { result: Err(_), .. }));
        let over = matches!(
            answer,
            Ok(Answer {
                result: Err(Reason::MemoryLimit { .. } | Reason::IndexLimit),
                ..
            })
        );
        if decided {
            self.answer = Some(answer);
        }
        let next = if over {
            u64::MAX
        } else {
            own.saturating_mul(2).saturating_add(self.spent)
        };
        (decided || ended, next)
    }
}

/// A row of the dispatch. A goal whose fragment lies within `fragment`,
/// searched in one of `modes`, with `feature`, goes to `engine`.
#[derive(Clone, Copy, Debug)]
pub(super) struct Row {
    /// The largest fragment the row takes.
    pub(super) fragment: Fragment,
    /// The modes it takes.
    pub(super) modes: Modes,
    /// What it asks of a goal besides.
    pub(super) feature: Feature,
    /// The engine its goals go to.
    pub(super) engine: Engine,
    /// Whether the inverse method takes turns beside the engine.
    ///
    /// The first turn comes once the engine has done [`PROBE_AFTER`] units
    /// of work without deciding, and another each time its work has
    /// doubled. Each turn runs the inverse method from its start, on the
    /// thread that polls. It does at most a [`TURN_SHARE`]th of that work,
    /// in the inverse method's units. Then the engine goes on where it
    /// stopped. Where a turn decides first, its answer is the search's.
    pub(super) probe: bool,
}

impl Row {
    /// Returns whether the row takes the goal.
    fn takes(&self, task: &Task<'_>) -> bool {
        self.fragment.contains(task.fragment) && self.modes.take(task.mode) && self.feature.of(task)
    }
}

/// The modes a row of the dispatch takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Modes {
    /// Every commutative mode.
    Any,
    /// Classical mode, with or without weakening and Mix.
    Classical,
    /// Intuitionistic mode, with or without weakening.
    Intuitionistic,
    /// Classical mode with Mix, without weakening.
    Mix,
    /// The classical mode that keeps the order of the formulas.
    Cyclic,
    /// The intuitionistic modes that keep it: the Lambek calculus, with
    /// or without empty antecedents.
    Lambek,
}

impl Modes {
    /// Returns whether a mode is one of these.
    fn take(self, mode: Mode) -> bool {
        #[expect(
            clippy::unneeded_field_pattern,
            reason = "every field named, so that a new one must be placed here"
        )]
        let Mode {
            intuitionistic,
            affine,
            mix,
            order,
            empty_antecedents: _,
        } = mode;
        let commutative = order == Order::Commutative;
        match self {
            Modes::Any => commutative,
            Modes::Classical => commutative && !intuitionistic,
            Modes::Mix => commutative && !intuitionistic && mix && !affine,
            Modes::Intuitionistic => commutative && intuitionistic,
            Modes::Cyclic => !commutative && !intuitionistic,
            Modes::Lambek => !commutative && intuitionistic,
        }
    }
}

/// What a row of the dispatch asks of a goal beyond its fragment and mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Feature {
    /// Nothing more.
    Any,
    /// Exactly two formulas, with an additive connective or unit among
    /// them (two literals are no additive problem).
    TwoFormulas,
    /// A Horn program with a clause under `!`: a Petri net with a marking
    /// to reach.
    PetriNet,
    /// An exponential in the goal.
    Exponentials,
}

impl Feature {
    /// Returns whether the goal has the feature.
    fn of(self, task: &Task<'_>) -> bool {
        match self {
            Feature::Any => true,
            Feature::TwoFormulas => task.goal.len() == 2 && !task.fragment.is_empty(),
            // The options may assert a fragment larger than the goal's, so
            // the fragment test only spares a goal that is no Horn program
            // the reading.
            Feature::PetriNet => task.fragment.has_exponentials() && horn::is_net(task),
            Feature::Exponentials => task.fragment.has_exponentials(),
        }
    }
}

/// Returns the smallest fragment the goal's subformulas lie in: the
/// sequent's own fragment for the roots, and possibly a smaller one for an
/// open goal deeper in the forest.
pub(crate) fn goal_fragment(forest: &Forest, goal: &[OccId]) -> Fragment {
    let mut fragment = Fragment::EMPTY;
    for &o in goal {
        for x in forest.subtree(o) {
            fragment |= forest.kind(x).fragment();
        }
    }
    fragment
}
