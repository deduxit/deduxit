// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Proof search: decide whether a sequent is provable, and get its proof
//! or the reason it has none.
//!
//! [`prove`] decides a sequent, [`prove_within`] does so under
//! [`Limits`] and a stop condition, and [`prove_goal`] decides a
//! [`Goal`], any multiset of a forest's occurrences. Each returns an
//! [`Outcome`], whose [`Verdict`] is a checked proof, a [`Disproof`], or
//! the [`Reason`] the search gave up. [`Options`] picks the engine and
//! tunes the search.
//!
//! The library picks an engine by the sequent's fragment and the mode.
//! [`Engine`] lists the engines, and its documentation says which one
//! decides which sequents. [`batch`] decides many sequents in one call.

/// The additive fast path.
pub(crate) mod additive;
pub mod batch;
/// The atom bias: which literal of each atom a focused search takes as
/// positive.
pub(crate) mod bias;
/// The classical refuter: it looks for an assignment of truth values
/// that makes the goal's classical reading false.
mod classical;
/// The count invariants the engines prune and refute with.
pub(crate) mod counts;
/// Essential-net search for unit-free IMLL.
mod essential;
/// The focused sequent engine.
pub(crate) mod focus;
/// Random provable sequents for the tests.
#[cfg(all(test, feature = "parse"))]
pub(crate) mod generate;
/// The Horn engine: reachability of markings.
pub(crate) mod horn;
/// The focused inverse method, a search forward from the axioms.
mod inverse;
/// The proof-net engine.
pub(crate) mod net;
#[cfg(feature = "parallel")]
mod parallel;
/// The planar criterion of the linking searches, for the ordered modes.
mod planar;
/// A reference prover for the tests: the plain unfocused calculus.
#[cfg(all(test, feature = "parse"))]
pub(crate) mod reference;
/// A first-order reference prover for the tests: the plain unfocused
/// calculus with unification and the eigenvariable condition by
/// freshness.
#[cfg(all(test, feature = "parse"))]
pub(crate) mod reference_fo;
/// Why a sequent is unprovable, the disproof that says so, and the
/// checks of refutations, apart from the engines.
mod refutation;
/// A search's count of its work, and the values its threads share.
mod work;

/// What a search returns: its outcome, verdict, reason and statistics.
mod outcome;
pub use outcome::{Outcome, Reason, Statistics, Verdict};
/// How a search runs: its options, the focused engine's bias, the threads
/// and the inverse method's turns.
mod options;
pub use options::{Bias, Cadence, Jobs, Options, Schedule};
/// The engines, their counters, and why a forced one is not taken.
mod engine;
pub use engine::{Counter, Engine, NotTaken};
/// The dispatch: which engine decides a goal, and the inverse method's
/// turns beside it.
mod dispatch;
pub(crate) use dispatch::goal_fragment;
use dispatch::{Turns, dispatch, probes, turn_bounds};
/// What the front door and every engine share: the task, the answer and
/// the trait each engine implements.
mod interface;
pub(crate) use interface::{Answer, Decide, Finished, Task};

#[cfg(feature = "parallel")]
pub use parallel::Pool;
#[cfg(feature = "rocq")]
pub(crate) use refutation::weighted;
pub use refutation::{
    Basis, Classical, Disproof, Equation, Marking, OneSided, OneSidedSymbol, Refutation,
    RefutationFault, Saturated, Stable, StateEquation, Unbalanced, UnbalancedSymbol,
};
use refutation::{basis, saturated};

use crate::Error;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::fragment::{Fragment, Mode};
#[cfg(doc)]
use crate::linear::nets::ProofStructure;
use crate::linear::occurrences::{Forest, Member, OccId, Reading};
use crate::linear::proofs::{CheckError, Proof};
use crate::linear::sequents::Sequent;
use crate::memory;
use std::sync::OnceLock;

#[cfg(feature = "parallel")]
pub(crate) use work::taken;
pub(crate) use work::{Counted, Work, lock};

/// An engine's stop condition: the caller's in a sequential search, a
/// worker's chain of stop flags in a parallel one. Each poll is told the
/// units of work done since the last.
pub(crate) enum Stop<'a> {
    /// The caller's condition.
    Closure(&'a mut dyn FnMut(u64) -> bool),
    /// The caller's condition, and the work left in this turn, after
    /// which the search stops too. It is one turn of a search that takes
    /// turns with another.
    Turn(&'a mut dyn FnMut(u64) -> bool, u64),
    /// A condition told at every poll the work since the last, and
    /// whether a slice of work has passed since it was last told so. The
    /// fields are the condition, the work left in the slice, and the
    /// slice.
    ///
    /// It is one of two searches that alternate: it gives way to the
    /// other at the end of each slice.
    #[cfg_attr(
        not(feature = "parallel"),
        expect(
            dead_code,
            reason = "only threads alternate the two searches in slices"
        )
    )]
    Slice(&'a mut dyn FnMut(u64, bool) -> bool, u64, u64),
    /// A worker's flags, and its count of the search's work.
    #[cfg(feature = "parallel")]
    Flags(parallel::Flags<'a>, Counted<'a>),
}

impl<'a> Stop<'a> {
    /// Polls the condition after `work` more units of work, in the
    /// polling engine's unit. Returns whether the search must stop.
    pub(crate) fn fired(&mut self, work: u64) -> bool {
        match self {
            Self::Closure(stop) => stop(work),
            Self::Turn(stop, left) => {
                *left = left.saturating_sub(work);
                // The caller is told the units of the poll that ends the
                // turn too.
                let stopped = stop(work);
                *left == 0 || stopped
            }
            Self::Slice(stop, left, slice) => {
                *left = left.saturating_sub(work);
                let passed = *left == 0;
                if passed {
                    *left = *slice;
                }
                stop(work, passed)
            }
            #[cfg(feature = "parallel")]
            Self::Flags(flags, counted) => flags.fired(work, counted),
        }
    }

    /// A worker's stop: its flags, and its own count of the search's work.
    /// The count adds its units to the search's in batches, and the rest
    /// when the worker ends.
    #[cfg(feature = "parallel")]
    pub(crate) fn flags(flags: parallel::Flags<'a>) -> Self {
        Self::Flags(flags, flags.counted())
    }
}

/// The occurrences a forest must have for a search to poll the stop
/// condition between the passes that set it up.
///
/// A pass over fewer takes under a millisecond. Below it, a condition
/// that counts its polls sees the engine's own polls and no others.
const SET_UP_POLL: usize = 1 << 16;

/// Polls the caller's stop condition between two passes that set a
/// search up, when the forest has at least [`SET_UP_POLL`] occurrences.
/// Returns whether to stop.
///
/// On millions of occurrences the set-up takes a second, and the
/// engine's first poll comes after it.
pub(crate) fn set_up_stopped(forest: &Forest, stop: &mut dyn FnMut(u64) -> bool) -> bool {
    forest.len() >= SET_UP_POLL && stop(0)
}

/// Decides a sequent in a mode, and returns the outcome.
///
/// The outcome holds the verdict (with the proof, if there is one), the
/// fragment detected, the engine that ran and the statistics of the run.
/// The engine is the one the sequent's fragment and the mode call for,
/// unless the options force one. The search runs under the default
/// [`Limits`] and never stops early; [`prove_within`] takes limits and a
/// stop condition.
///
/// Every proof returned has passed the checker, unless [`Options::check`]
/// is off.
///
/// # Errors
///
/// - [`Error::FragmentMismatch`]: the sequent is outside the fragment the
///   options assert.
/// - [`Error::NotIntuitionistic`]: in intuitionistic mode, the sequent
///   has no intuitionistic reading.
/// - [`Error::IntuitionisticMix`]: intuitionistic mode with Mix.
/// - [`Error::EngineRefused`]: the options force an engine that does not
///   search this sequent, with the [`NotTaken`] that says why. The net
///   engine takes unit-free MLL only, and not in affine mode. The focused
///   engine is refused in intuitionistic mode, the two-sided engine in
///   classical mode. The additive engine takes two additive-only formulas
///   only, the Horn engine a Horn program only.
/// - [`Refusal::Occurrences`]: the sequent unfolds to more subformula
///   occurrences than `limits.occurrences` allows.
/// - [`Error::Rejected`]: the checker rejects a proof an engine found.
///   This is a defect of the library, not a verdict.
/// - [`Error::ThreadPool`]: on several threads, they cannot start.
///
/// A check of the proof that a bound or the stop gives up is no error.
/// The verdict is then [`Verdict::Unknown`]: with [`Reason::Unchecked`]
/// when the check would hold more than `limits.memory_bytes`, and with
/// [`Reason::WorkLimit`] or [`Reason::Stopped`] when `limits.work` or the
/// stop ends it.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::linear::search::{Engine, Options, Verdict, prove};
/// use deduxit::linear::{Fragment, Mode, Sequent};
///
/// let sequent: Sequent = "A, A -o B |- B".parse()?;
/// let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
/// assert_eq!(outcome.fragment, Fragment::MLL);
/// assert_eq!(outcome.engine, Engine::Focus);
/// let net = Options::default().with_engine(Some(Engine::Net));
/// let by_net = prove(&sequent, Mode::CLASSICAL, &net)?;
/// assert!(by_net.net.is_some(), "the net engine returns the net it found");
/// let Verdict::Proved(proof) = outcome.verdict else {
///     panic!("provable");
/// };
/// assert_eq!(
///     proof.derivation()?.to_string(),
///     "─────── ax   ─────── ax\n\
///      ⊢ ~A, A      ⊢ ~B, B\n\
///      ──────────────────── ⊗\n\
///     \x20 ⊢ ~A, A ⊗ ~B, B"
/// );
///
/// let sequent: Sequent = "|- A par B, ~A, ~B".parse()?;
/// let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
/// assert!(matches!(outcome.verdict, Verdict::Unprovable(_)));
/// let outcome = prove(&sequent, Mode::CLASSICAL.with_mix(), &Options::default())?;
/// assert!(matches!(outcome.verdict, Verdict::Proved(_)));
/// # Ok::<(), deduxit::Error>(())
/// ```
pub fn prove(sequent: &Sequent, mode: Mode, options: &Options) -> Result<Outcome, Error> {
    prove_within(sequent, mode, options, &Limits::default(), |_| false)
}

/// Decides a sequent as [`prove`] does, within `limits`, and gives up
/// with [`Reason::Stopped`] once `stop` returns true.
///
/// The limits bound:
/// - the sequent's forest, by `limits.occurrences`;
/// - the search's structures and the check of its proof, by
///   `limits.memory_bytes`;
/// - the search's recursion, by `limits.recursion_depth`;
/// - the search's work, and the check's, each by `limits.work`.
///
/// The stop condition is the caller's: a deadline on the caller's clock,
/// or a flag an interrupt handler sets. This crate has no clock of its
/// own. `|_| false` never stops. The condition is told the [`Progress`]
/// at every poll: [`Phase::Search`] during the
/// search, [`Phase::Refute`] while a refuter looks
/// for a disproof after it, and [`Phase::Check`]
/// while what it found is checked.
///
/// Where it is polled: the engines poll at every stable sequent or
/// literal chosen, and inside every loop that can run long between two of
/// them. On a forest of tens of thousands of occurrences, they also poll
/// between the passes that set the search up. The check of a proof found
/// polls every 4 096 nodes. The building of the forest before the search
/// is not polled, nor are single passes over the forest, each linear in
/// it.
///
/// Make the condition cheap to ask, and ask it at every poll. One that
/// reads a clock only every so many polls is late by that many polls, and
/// on a large sequent a poll can come many milliseconds after the last.
///
/// # Errors
///
/// Those of [`prove`], under `limits`.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::linear::search::{Options, Reason, Verdict, prove_within};
/// use deduxit::{Limits, Phase, Progress, linear::{Mode, Sequent}};
/// use std::time::{Duration, Instant};
///
/// let sequent: Sequent = "|- (a & b) + (a & c), ~a par (~b & ~c)".parse()?;
/// let (options, limits) = (Options::default(), Limits::default());
///
/// // `|_| false` never stops: the search ends at a verdict or a bound.
/// let outcome = prove_within(&sequent, Mode::CLASSICAL, &options, &limits, |_| false)?;
/// assert!(!matches!(outcome.verdict, Verdict::Unknown(_)));
///
/// // A deadline on the caller's clock.
/// let deadline = Instant::now() + Duration::from_secs(10);
/// let outcome = prove_within(&sequent, Mode::CLASSICAL, &options, &limits, |_| {
///     Instant::now() >= deadline
/// })?;
/// assert!(!matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)));
///
/// // A deadline for the search alone. What runs after the search (the
/// // refuters, the check) runs on within its own bounds.
/// let outcome = prove_within(&sequent, Mode::CLASSICAL, &options, &limits, |p: Progress| {
///     p.phase == Phase::Search && Instant::now() >= deadline
/// })?;
/// assert!(!matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)));
/// # Ok::<(), deduxit::Error>(())
/// ```
pub fn prove_within(
    sequent: &Sequent,
    mode: Mode,
    options: &Options,
    limits: &Limits,
    stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    let forest = Forest::within(sequent, limits)?;
    prove_goal(Goal::conclusion(&forest), mode, options, limits, stop)
}

/// Decides a goal of a forest with cut formulas by `run` on a forest of
/// its own, so that no engine sees a cut formula.
///
/// The own forest is that of a cut-free sequent over the same arena,
/// whose roots are the goal's formulas. Intuitionistically, the
/// hypotheses come first, left of `⊢`, then the goal's output. It
/// numbers each formula's occurrences as the goal's forest does, shifted.
///
/// A proof found is carried back by the shift, as a proof of the goal.
/// It is checked again there when it was checked. A disproof is the own
/// sequent's, whose formulas are the goal's.
fn on_own_forest(
    goal: Goal<'_>,
    mode: Mode,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
    run: impl FnOnce(&Forest, &mut dyn FnMut(Progress) -> bool) -> Result<Outcome, Error>,
) -> Result<Outcome, Error> {
    let forest = goal.forest;
    let mut members: Vec<OccId> = goal.occurrences().into_owned();
    let antecedents = if mode.is_intuitionistic() {
        let reading = Reading::of_mode(forest, mode).map_err(Error::NotIntuitionistic)?;
        let outputs = reading.outputs(members.iter().copied());
        if outputs != 1 {
            return Err(Error::GoalOutputs { count: outputs });
        }
        // Stable, so the hypotheses keep their order.
        members.sort_by_key(|&o| reading.position(o) == crate::linear::Side::Output);
        Some(members.len() as u32 - 1)
    } else {
        None
    };
    let sequent = forest.sequent();
    let own = Sequent {
        terms: sequent.terms().to_vec(),
        roots: members.iter().map(|&o| forest.term(o)).collect(),
        atoms: sequent.atom_names().to_vec(),
        antecedents,
        cuts: vec![],
        fo: sequent.fo.clone(),
    };
    let own = Forest::within(&own, limits)?;
    let mut outcome = run(&own, stop)?;
    let Verdict::Proved(proof) = &outcome.verdict else {
        return Ok(outcome);
    };
    // An occurrence of the own forest, in the goal's forest: its formula's
    // member, plus how far into the formula it lies.
    let lift = |m: Member| {
        let o = m.occ();
        let root = own.root(o);
        let i = own.roots().partition_point(|&r| r < root);
        Ok::<_, Error>(Member::from(OccId::new(
            members[i].get() + (o.get() - root.get()),
        )))
    };
    let nodes = proof
        .nodes()
        .iter()
        .map(|&node| node.renamed(lift))
        .collect::<Result<Vec<_>, _>>()?;
    let carried = if goal.conclusion {
        Proof::new(forest.clone(), nodes, proof.root())?
    } else {
        Proof::new_of_goal(forest.clone(), goal.members, nodes, proof.root())?
    };
    let carried = match proof.mode() {
        Some(mode) => carried.with_mode(mode),
        None => carried,
    };
    if outcome.checked
        && let Some(reason) = checked(&carried, mode, limits, limits, &mut *stop)?
    {
        // A check the bounds or the stop gave up is no fault of the input,
        // as `conclude` answers it.
        outcome.verdict = Verdict::Unknown(reason);
        outcome.checked = false;
        return Ok(outcome);
    }
    outcome.verdict = Verdict::Proved(Box::new(carried));
    Ok(outcome)
}

/// What a search decides: a multiset of a forest's occurrences, read as
/// the sequent of those subformulas.
///
/// It is the forest's conclusion (its roots, in any order) or any other
/// multiset of its occurrences, such as a goal an interactive proof left
/// open. [`prove_goal`] decides it.
#[derive(Clone, Copy, Debug)]
pub struct Goal<'a> {
    /// The forest.
    forest: &'a Forest,
    /// The members, as given; empty for the conclusion.
    members: &'a [Member],
    /// Whether the members are the roots.
    conclusion: bool,
}

impl<'a> Goal<'a> {
    /// Returns the forest's conclusion, the sequent itself, as a goal.
    pub fn conclusion(forest: &'a Forest) -> Self {
        Self {
            forest,
            members: &[],
            conclusion: true,
        }
    }

    /// Returns the goal of the members given, in any order and with
    /// repeats. The roots in any order are the conclusion.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for a member outside the forest.
    pub fn new(forest: &'a Forest, members: &'a [Member]) -> Result<Self, Error> {
        if let Some(m) = members.iter().find(|m| m.index() >= forest.len()) {
            return Err(Error::IndexOutOfBounds {
                space: crate::limits::Space::Occurrence,
                index: m.index(),
                len: forest.len(),
            });
        }
        let occurrences: Vec<OccId> = members.iter().map(|m| m.occ()).collect();
        Ok(Self {
            forest,
            members,
            conclusion: forest.is_roots(&occurrences),
        })
    }

    /// Returns whether the goal is the forest's conclusion.
    pub fn is_conclusion(&self) -> bool {
        self.conclusion
    }

    /// Returns the forest.
    pub fn forest(&self) -> &'a Forest {
        self.forest
    }

    /// Returns the goal's occurrences: the roots in the forest's order
    /// for the conclusion, the members as given otherwise.
    fn occurrences(&self) -> std::borrow::Cow<'a, [OccId]> {
        if self.conclusion {
            std::borrow::Cow::Borrowed(self.forest.roots())
        } else {
            std::borrow::Cow::Owned(self.members.iter().map(|m| m.occ()).collect())
        }
    }
}

/// Decides a [`Goal`] within `limits` and until `stop` returns true, as
/// [`prove_within`] does.
///
/// A goal is a multiset of a forest's occurrences, given in any order,
/// read as the sequent of those subformulas. The roots are the sequent
/// itself, and [`prove_within`] is this function on them. Any other goal
/// is one an interactive proof leaves open. The engine is the one the
/// goal's own fragment calls for, except that the net engine works on the
/// roots only.
///
/// The proof of a goal other than the roots is a [`Proof`] whose root
/// concludes the goal. It records the goal ([`Proof::goal`]), and
/// [`Proof::check`] checks it against that goal. It is meant to be
/// grafted onto the goal, as the interactive state does. Where a proof of
/// the sequent is needed, it is refused ([`Error::GoalProof`]). Every
/// proof records the mode it was found in ([`Proof::mode`]).
///
/// # Errors
///
/// Those of [`prove`], plus [`Error::GoalOutputs`] for an
/// intuitionistic goal without exactly one formula on the right of `⊢`,
/// and [`Error::EngineRefused`] with [`NotTaken::Goal`] for the net engine
/// forced on a goal other than the roots.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::linear::search::{Goal, Options, Verdict, prove_goal};
/// use deduxit::{Limits, linear::{Forest, Member, Mode, Sequent}};
///
/// // The sequent is ⊢ ~A, A ⊗ ~B, B, with the occurrences 0: ~A,
/// // 1: A ⊗ ~B, 2: A, 3: ~B, 4: B. The goal ⊢ ~A, A is the left premise
/// // of the ⊗.
/// let sequent: Sequent = "A, A -o B |- B".parse()?;
/// let forest = Forest::new(&sequent)?;
/// let members = [Member::new(0), Member::new(2)];
/// let goal = Goal::new(&forest, &members)?;
/// let limits = Limits::default();
/// let outcome = prove_goal(goal, Mode::CLASSICAL, &Options::default(), &limits, |_| false)?;
/// assert!(matches!(outcome.verdict, Verdict::Proved(_)));
/// # Ok::<(), deduxit::Error>(())
/// ```
pub fn prove_goal(
    goal: Goal<'_>,
    mode: Mode,
    options: &Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    if goal.forest.has_cuts() {
        return on_own_forest(goal, mode, limits, &mut stop, |own, stop| {
            prove_goal(Goal::conclusion(own), mode, options, limits, stop)
        });
    }
    // The search holds the input as long as it runs, so the bound counts
    // the input too.
    let account = memory::Account::holding(limits.memory_bytes, goal.forest.held_bytes() as u64);
    let work = Work::new(limits.work);
    decide_goal(goal, mode, options, limits, &account, &work, &mut stop)
}

/// Decides a goal as [`prove_goal`] does, on at most `threads` threads,
/// the calling one counted.
///
/// From three threads, one search starts on the calling thread. Once
/// `add_pool` returns true at one of its polls, a pool of the other
/// `threads − 1` threads starts the search afresh beside it. The first to
/// decide answers and stops the other. Both draw on one memory bound and
/// one bound on work. The single search goes on beside the pool, since a
/// pool may search worse than one thread.
///
/// Below three threads, in an ordered mode, or for an engine that
/// searches on one thread ([`Engine::parallel`]), it runs one search on
/// `threads` threads, and `add_pool` is never asked. With two threads,
/// that is a pool of two from the start.
///
/// The single search runs on the calling thread, whose stack must hold
/// the recursion ([`Limits::stack_bytes`]). `add_pool` is told the single
/// search's progress, so a front end decides with its own clock when to
/// add the pool. Both searches ask `stop`, from two threads. The outcome
/// is the deciding search's, with both searches' counters
/// ([`Statistics::add`]).
///
/// Needs the cargo feature `parallel` (off by default).
///
/// # Errors
///
/// Those of [`prove_goal`], and [`Error::ThreadPool`] when the pool's
/// threads cannot start.
#[cfg(feature = "parallel")]
pub fn race(
    goal: Goal<'_>,
    mode: Mode,
    options: &Options,
    limits: &Limits,
    threads: usize,
    mut add_pool: impl FnMut(Progress) -> bool,
    stop: impl Fn(Progress) -> bool + Sync,
) -> Result<Outcome, Error> {
    use std::sync::atomic::{AtomicBool, Ordering};
    if goal.forest.has_cuts() {
        // One instance of the function, whatever the stop's type.
        let stop: &(dyn Fn(Progress) -> bool + Sync) = &stop;
        return on_own_forest(goal, mode, limits, &mut |p| stop(p), |own, _| {
            race(
                Goal::conclusion(own),
                mode,
                options,
                limits,
                threads,
                add_pool,
                stop,
            )
        });
    }
    let threads = threads.clamp(1, Options::MAX_JOBS);
    let whole = std::sync::Arc::new(memory::Account::holding(
        limits.memory_bytes,
        goal.forest.held_bytes() as u64,
    ));
    let work = Work::new(limits.work);
    if threads < 3 || mode.is_ordered() || !engine_for(goal, mode, options)?.parallel() {
        let options = options.clone().with_jobs(threads);
        let account = memory::Account::part_of(&whole);
        return decide_goal(goal, mode, &options, limits, &account, &work, &mut |p| {
            stop(p)
        });
    }
    let decided = AtomicBool::new(false);
    let settled = |answered: &Result<Answered<'_>, Error>| matches!(answered, Ok(a) if a.answer.result.is_ok());
    let (single, pooled) = (
        options.clone().with_jobs(1),
        options.clone().with_jobs(threads - 1),
    );
    let answered = std::thread::scope(|scope| {
        let (whole, work, stop, decided) = (&whole, &work, &stop, &decided);
        let pooled = &pooled;
        let mut pool = None;
        let account = memory::Account::part_of(whole);
        // A panic on either side stops the other, which the scope waits for.
        let _raise = parallel::RaiseOnPanic(decided);
        let first = answer_goal(
            goal,
            mode,
            &single,
            limits,
            &account,
            work,
            &mut |progress| {
                if pool.is_none() && add_pool(progress) {
                    // A pool whose thread cannot start leaves the single
                    // search alone.
                    pool = Some(
                        std::thread::Builder::new()
                            .name("deduxit-race".into())
                            .stack_size(limits.stack_bytes())
                            .spawn_scoped(scope, move || {
                                let _raise = parallel::RaiseOnPanic(decided);
                                let account = memory::Account::part_of(whole);
                                let answered = answer_goal(
                                    goal,
                                    mode,
                                    pooled,
                                    limits,
                                    &account,
                                    work,
                                    &mut |p| stop(p) || decided.load(Ordering::Relaxed),
                                );
                                if settled(&answered) {
                                    decided.store(true, Ordering::Relaxed);
                                }
                                answered
                            })
                            .ok(),
                    );
                }
                stop(progress) || decided.load(Ordering::Relaxed)
            },
        );
        if settled(&first) {
            decided.store(true, Ordering::Relaxed);
        }
        let Some(Some(handle)) = pool else {
            return first;
        };
        let second = handle
            .join()
            .unwrap_or_else(|panic| std::panic::resume_unwind(panic));
        // A decision of either side answers, whatever the other ended
        // with (a pool that could not start its threads, say). Without
        // one, an error of either side answers.
        let (mut answered, other) = match (first, second) {
            (first, second) if settled(&first) => (first?, second.ok()),
            (first, second) if settled(&second) => (second?, first.ok()),
            (first, second) => (second?, Some(first?)),
        };
        if let Some(other) = other {
            answered.answer.statistics.add(&other.answer.statistics);
        }
        answered.answer.statistics.work = work.done();
        Ok(answered)
    })?;
    // Both sides have ended, so the verdict's check or refutation has
    // what the bound leaves beside the input.
    let account = memory::Account::part_of(&whole);
    answered.concluded(goal, options, limits, &account, &mut |p| stop(p))
}

/// Decides a goal as [`prove_goal`] does, charging what the search holds
/// to `account` and counting its work in `work`.
fn decide_goal(
    goal: Goal<'_>,
    mode: Mode,
    options: &Options,
    limits: &Limits,
    account: &memory::Account,
    work: &Work,
    stop: &mut impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    answer_goal(goal, mode, options, limits, account, work, stop)?
        .concluded(goal, options, limits, account, stop)
}

/// What a search of a goal answered, before the verdict is made of it:
/// the engine's answer, and the goal's occurrences, fragment, mode and
/// engine.
struct Answered<'a> {
    /// The engine's answer, with the counters.
    answer: Answer,
    /// The goal's occurrences, as the engine was handed them.
    occurrences: std::borrow::Cow<'a, [OccId]>,
    /// The fragment searched in.
    fragment: Fragment,
    /// The mode.
    mode: Mode,
    /// The engine that ran.
    engine: Engine,
}

impl Answered<'_> {
    /// Makes the verdict of the answer ([`conclude`]) and returns the
    /// outcome. The verdict's check or refutation draws on `account` too.
    fn concluded(
        self,
        goal: Goal<'_>,
        options: &Options,
        limits: &Limits,
        account: &memory::Account,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Outcome, Error> {
        let Self {
            answer,
            occurrences,
            fragment,
            mode,
            engine,
        } = self;
        let verdict = conclude(
            answer.result,
            answer.refutation,
            Concluding {
                forest: goal.forest,
                occurrences: &occurrences,
                fragment,
                mode,
                roots: goal.conclusion,
                members: goal.members,
            },
            options,
            limits,
            account,
            stop,
        )?;
        let checked = options.check && matches!(verdict, Verdict::Proved(_));
        Ok(Outcome {
            verdict,
            fragment,
            mode,
            engine,
            statistics: answer.statistics,
            net: answer.net.map(Box::new),
            checked,
            named: None,
        })
    }
}

/// Searches a goal as [`decide_goal`] does, and returns what the engine
/// answered without making the verdict of it.
///
/// A race makes the verdict once both of its searches have ended.
fn answer_goal<'a>(
    goal: Goal<'a>,
    mode: Mode,
    options: &Options,
    limits: &Limits,
    account: &memory::Account,
    work: &Work,
    stop: &mut impl FnMut(Progress) -> bool,
) -> Result<Answered<'a>, Error> {
    let forest = goal.forest;
    let occurrences = goal.occurrences();
    let fragment = fragment_of(forest, &occurrences, options)?;
    let reading = read(forest, mode)?;
    let mut held = memory::Charged::new(account);
    held.charge(reading.as_ref().map_or(0, Reading::held_bytes));
    let (task, engine) = prepare(
        goal,
        &occurrences,
        mode,
        fragment,
        options,
        reading.as_ref(),
    )?;
    let implementation = engine.implementation();
    // The inverse method's turns beside the engine. `due` is the search's
    // work, the turns' included, at which the next turn is due.
    let (head_start, share) = turn_bounds();
    let mut due = head_start;
    let turns = std::cell::RefCell::new(Turns {
        spent: 0,
        share,
        answer: None,
    });
    // Every poll on this thread counts its units of work. It tells the
    // caller's stop those units and the units of every thread of the
    // search. Past the bound on work, the search ends. Where other threads
    // read the shared count, this one adds its units to it in batches.
    let mut mine = Counted::new(work);
    // The fragment, the reading and the dispatch were passes over the
    // forest, so the caller's condition is asked before the engine polls.
    if set_up_stopped(forest, &mut |units| poll(&mut mine, &mut *stop, units).0) {
        // This thread has done no work yet. So either another thread's
        // work passed the bound, or the caller stopped.
        let reason = if work.passed() {
            Reason::WorkLimit {
                limit: work.limit(),
            }
        } else {
            Reason::Stopped
        };
        return Ok(Answered {
            answer: Answer {
                result: Err(reason),
                statistics: Statistics::default(),
                net: None,
                refutation: None,
            },
            occurrences,
            fragment,
            mode,
            engine,
        });
    }
    // No more threads than the machine runs at once. `held` is what the
    // account holds before the engine runs, which a handover rewinds to.
    #[cfg(feature = "parallel")]
    let options = &options
        .clone()
        .with_jobs(parallel::threads(options.threads()));
    let held = account.used();
    let mut answer = if probes(&task, options) {
        // A turn comes only at a poll of the engine's own work on this
        // thread. While a second thread of the default bias has its turn,
        // this one polls without units and waits. A search whose row gives
        // no turn polls without the comparison: on every poll it costs some
        // instructions, a quarter of a percent of a focused search.
        implementation.decide(&task, options, limits, account, work, &mut |units| {
            let (stopped, done) = poll(&mut mine, &mut *stop, units);
            if stopped || done < due || units == 0 {
                return stopped;
            }
            let (ended, next) = turns.borrow_mut().take(
                done,
                &task,
                options,
                limits,
                account,
                work,
                &mut mine,
                &mut |progress| stop(progress),
            );
            due = next;
            ended
        })?
    } else {
        implementation.decide(&task, options, limits, account, work, &mut |units| {
            poll(&mut mine, &mut *stop, units).0
        })?
    };
    let probe = turns.borrow_mut().answer.take();
    let engine = if let Some(probe) = probe {
        // The engine stopped at the poll at which the inverse method
        // decided. The turn's answer and counters replace the engine's, as
        // at a handover.
        drop(answer);
        answer = probe?;
        Engine::Inverse
    } else if engine != Engine::Net && handed_over(options, &task, &answer) {
        let mut taker = if matches!(answer.result, Err(Reason::MemoryLimit { .. }))
            && !task.mode.is_intuitionistic()
            && !task.mode.has_mix()
            && Engine::Inverse.implementation().admits(&task).is_ok()
        {
            Engine::Inverse
        } else {
            Engine::Net
        };
        loop {
            account.rewind(held);
            answer = taker.implementation().decide(
                &task,
                options,
                limits,
                account,
                work,
                &mut |units| poll(&mut mine, &mut *stop, units).0,
            )?;
            // The inverse method's own bound leaves the goal to the net
            // engine, which holds less.
            if taker == Engine::Net
                || !matches!(
                    answer.result,
                    Err(Reason::MemoryLimit { .. } | Reason::IndexLimit)
                )
            {
                break taker;
            }
            taker = Engine::Net;
        }
    } else {
        engine
    };
    drop(mine);
    answer.statistics.work = work.done();
    if work.passed() && matches!(answer.result, Err(Reason::Stopped)) {
        answer.result = Err(Reason::WorkLimit {
            limit: work.limit(),
        });
    }
    drop(task);
    Ok(Answered {
        answer,
        occurrences,
        fragment,
        mode,
        engine,
    })
}

/// Counts a poll's units in `mine` and asks the caller's `stop`. Returns
/// whether the search ends, and the search's units in all.
///
/// The caller is told the poll that passes the bound on work too.
fn poll(
    mine: &mut Counted<'_>,
    stop: &mut impl FnMut(Progress) -> bool,
    units: u64,
) -> (bool, u64) {
    let done = mine.add(units);
    let stopped = stop(Progress::new(Phase::Search, units, done));
    (stopped || mine.passed(), done)
}

/// Returns whether another engine takes over a goal on which the
/// dispatch's engine ended at its recursion limit or memory bound.
///
/// It does when no engine is forced, and the goal is unit-free MLL, the
/// roots, in a mode without weakening, with a forced linking
/// ([`one_linking`]). The focused engines recurse once per stable
/// sequent and keep a memo. So a long chain of links reaches the
/// recursion limit, and a wide enough sequent the memory bound.
///
/// Which engine takes over:
/// - after the recursion limit, the net engine. It does not recurse, and
///   on a chain it is twice as fast as the inverse method;
/// - after the memory bound in classical mode without Mix, the inverse
///   method. It is three to four times as fast as the net engine on a
///   wide sequent there. Where it meets a bound of its own, the net
///   engine takes over after it;
/// - elsewhere, the net engine. Under Mix, the inverse method ran past a
///   minute where the net engine proved the same wide sequent.
///
/// What the searches before left charged is given back first, so the
/// proof is checked under the bound as if its engine had run alone. The
/// last answer replaces the ones before, and drops their counters (in a
/// race the other search's are added, as always). The work counted is
/// every search's, each in its engine's unit.
fn handed_over(options: &Options, task: &Task<'_>, answer: &Answer) -> bool {
    options.engine.is_none()
        && matches!(
            answer.result,
            Err(Reason::RecursionLimit { .. } | Reason::MemoryLimit { .. })
        )
        && Engine::Net.implementation().admits(task).is_ok()
        && one_linking(task.forest)
}

/// Returns whether every atom of the forest occurs at most once with each
/// sign, so that its axiom linking is the only one.
///
/// The net engine's search then makes one link per pair and backtracks
/// over none. Each link costs a scan of the atoms and a test of the
/// criterion, so the search is quadratic in all. On equal literals its
/// search is exponential: taking over a goal on which a focused engine
/// met its limit would trade a quick "unknown" for a search without end.
fn one_linking(forest: &Forest) -> bool {
    use crate::linear::occurrences::Sign;
    (0..forest.sequent().atom_names().len() as u32).all(|a| {
        let atom = crate::linear::sequents::Atom::new(a);
        forest.literals(atom, Sign::Atom).len() <= 1 && forest.literals(atom, Sign::Dual).len() <= 1
    })
}

/// What [`conclude`] reads of the goal an answer is of.
struct Concluding<'a> {
    /// The forest.
    forest: &'a Forest,
    /// The goal's occurrences, as the engine was handed them.
    occurrences: &'a [OccId],
    /// The fragment searched in.
    fragment: Fragment,
    /// The mode.
    mode: Mode,
    /// Whether the goal is the sequent's roots.
    roots: bool,
    /// The goal's members.
    members: &'a [Member],
}

/// Returns the verdict of an engine's answer. This is the one place a
/// verdict is made.
///
/// The verdict is one of:
/// - a proof that passed the checker, which records the goal it
///   concludes and the mode;
/// - an exhausted search, with the engine's refutation or the counts';
/// - why the search stopped.
///
/// No engine is trusted with its own proof. The checker has the last word
/// on every proof, of the sequent or of a goal, in every build. A check
/// that a bound or the stop gave up is no verdict on the proof, so no
/// proof is returned unchecked. The exception is [`Options::check`] off:
/// the proof is then the engine's word, and `Outcome::checked` says so.
///
/// The check of a proof and the refutation hold their memory on
/// `account`, beside what it holds, within the same bound as the search.
/// The check of a saturated set or a basis holds its own memory outside
/// it. That is of the order of the set, which the search held within the
/// bound.
///
/// # Errors
///
/// [`Error::Rejected`] for a proof the checker rejects, and
/// [`Error::Refutation`] for a saturated set or a basis its check
/// refuses: defects.
fn conclude(
    result: Result<Option<Proof>, Reason>,
    refutation: Option<Refutation>,
    goal: Concluding<'_>,
    options: &Options,
    limits: &Limits,
    account: &memory::Account,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Verdict, Error> {
    let (forest, mode, members) = (goal.forest, goal.mode, goal.members);
    let verdict = match result {
        Ok(Some(proof)) if goal.roots => Verdict::Proved(Box::new(proof.with_mode(mode))),
        Ok(Some(proof)) => {
            Verdict::Proved(Box::new(proof.concluding(goal.occurrences).with_mode(mode)))
        }
        Ok(None) => {
            // A saturated set and a basis are the refutations of last
            // resort: the counts' and the classical one are smaller and rest
            // on simpler invariants.
            let (refutation, last) = match refutation {
                Some(last @ (Refutation::Saturated(_) | Refutation::Basis(_))) => {
                    (None, Some(last))
                }
                refutation => (refutation, None),
            };
            // Where the engine has no refutation of its own: what the
            // counts of the goal rule out, under the same bound as the
            // search. Failing that, a classical assignment, where one is
            // found.
            let refutation = refutation.unwrap_or_else(|| {
                let mut polled = |_| stop(Progress::new(Phase::Refute, 0, 0));
                counts::refutation(
                    forest,
                    goal.occurrences,
                    goal.fragment,
                    mode,
                    account,
                    &mut polled,
                )
            });
            let refutation = match refutation {
                Refutation::Exhausted => classical(&goal, options, account, stop)
                    .map_or(Refutation::Exhausted, Refutation::Classical),
                refutation => refutation,
            };
            let refutation = match (refutation, last) {
                (Refutation::Exhausted, Some(last)) => {
                    // The set or the basis is checked as a proof is, by code
                    // apart from the engine that found it, unless the check
                    // is off, within a work bound of its own, as a proof's
                    // check is.
                    if options.check {
                        let mut done = 0u64;
                        let mut polled = |units: u64| {
                            done = done.saturating_add(units);
                            stop(Progress::new(Phase::Check, units, done))
                                || limits.work.is_some_and(|limit| done > limit)
                        };
                        let checked = match &last {
                            Refutation::Saturated(set) => {
                                saturated::check(forest, goal.occurrences, mode, set, &mut polled)
                                    .map_err(refutation::Failed::fault)
                            }
                            Refutation::Basis(basis) => {
                                basis::check(forest, goal.occurrences, basis, &mut polled)
                                    .map_err(refutation::Failed::fault)
                            }
                            _ => unreachable!("only a set or a basis is kept for last"),
                        };
                        match checked {
                            Ok(()) => {}
                            Err(None) => {
                                return Ok(Verdict::Unknown(match limits.work {
                                    Some(limit) if done > limit => Reason::WorkLimit { limit },
                                    _ => Reason::Stopped,
                                }));
                            }
                            Err(Some(fault)) => return Err(Error::Refutation(fault)),
                        }
                    }
                    last
                }
                (refutation, _) => refutation,
            };
            unprovable(forest, mode, refutation, &goal, members)
        }
        // A search that did not decide is unprovable where an assignment
        // makes the classical reading false. Otherwise it stays as it
        // ended.
        Err(reason) => match classical(&goal, options, account, stop) {
            Some(assignment) => unprovable(
                forest,
                mode,
                Refutation::Classical(assignment),
                &goal,
                members,
            ),
            // The option's bound, whatever share of it the engine's
            // account had (a race's part leaves out the input, and a
            // share of nothing is zero): every engine, the Horn engine's
            // too, which named its account's.
            None => Verdict::Unknown(reason.as_set(options, limits)),
        },
    };
    let Verdict::Proved(proof) = &verdict else {
        return Ok(verdict);
    };
    if !options.check {
        debug_assert_eq!(proof.check(mode), Ok(()), "an engine's proof");
        return Ok(verdict);
    }
    // The check gets what the bound leaves beside the input. A refusal
    // names the option's bound.
    let left = (*limits).with_memory_bytes(account.left());
    Ok(match checked(proof, mode, &left, limits, stop)? {
        None => verdict,
        Some(reason) => Verdict::Unknown(reason),
    })
}

/// Checks a proof a search found, within `limits` and until `stop`
/// fires: `None` where it passes, the reason of an unknown verdict where
/// a bound or the stop gave the check up, naming the bound of `named`
/// (the caller's, where `limits` is what the search left of it), as
/// [`unchecked`] does.
///
/// # Errors
///
/// [`Error::Rejected`] for a proof the checker refuses, a defect of the
/// engine; a refusal no check makes.
fn checked(
    proof: &Proof,
    mode: Mode,
    limits: &Limits,
    named: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Option<Reason>, Error> {
    match proof.check_within(mode, limits, stop) {
        Ok(()) => Ok(None),
        Err(e @ CheckError::Invalid(_)) => Err(Error::Rejected(Box::new(e))),
        Err(CheckError::Refused(refused)) => Ok(Some(unchecked(refused.refusal, named)?)),
    }
}

/// Returns the verdict that the goal is unprovable for `refutation`.
fn unprovable(
    forest: &Forest,
    mode: Mode,
    refutation: Refutation,
    goal: &Concluding<'_>,
    members: &[Member],
) -> Verdict {
    let disproof = Disproof::new(forest.sequent().clone(), mode, refutation);
    Verdict::Unprovable(Box::new(if goal.roots {
        disproof
    } else {
        disproof.of_goal(members.into())
    }))
}

/// Returns an assignment that makes the classical reading of the goal
/// false, or `None` where none is found.
///
/// The search for it runs within [`Options::classical_work`] and what the
/// memory bound leaves, and its polls ask `stop`.
fn classical(
    goal: &Concluding<'_>,
    options: &Options,
    account: &memory::Account,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Option<Classical> {
    classical::refute(
        goal.forest,
        goal.occurrences,
        options.classical_work,
        account,
        stop,
    )
}

/// Returns why a search whose proof's check was given up is unknown: the
/// stop, the work bound, or the memory bound of `limits` the check would
/// have passed.
///
/// # Errors
///
/// A refusal that no check makes stays the error it is.
fn unchecked(refusal: Refusal, limits: &Limits) -> Result<Reason, Error> {
    match refusal {
        Refusal::Stopped { .. } => Ok(Reason::Stopped),
        Refusal::Memory { limit_bytes, .. } => Ok(Reason::Unchecked {
            limit_bytes: limits.memory_bytes.unwrap_or(limit_bytes),
        }),
        Refusal::Work { limit } => Ok(Reason::WorkLimit { limit }),
        refusal => Err(Error::Refused(refusal)),
    }
}

/// Returns the engine [`prove_goal`] runs on a goal under the options:
/// the one they force, or the one the dispatch picks.
///
/// It costs what `prove_goal` does before it searches: a pass over the
/// goal for its fragment, in intuitionistic mode the reading of the
/// forest, and the dispatch's test of the goal's shape. A front end asks
/// it to say which engine will run. `race` (feature `parallel`) asks it
/// before it adds a pool beside a single thread, since an engine that
/// runs on one thread ([`Engine::parallel`]) would only search again.
///
/// # Errors
///
/// Those of [`prove_goal`] that come before the search.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::linear::search::{Engine, Goal, Options, engine_for};
/// use deduxit::linear::{Forest, Mode, Sequent};
///
/// let sequent: Sequent = "A, A -o B |- B".parse()?;
/// let forest = Forest::new(&sequent)?;
/// let goal = Goal::conclusion(&forest);
/// let options = Options::default();
/// assert_eq!(engine_for(goal, Mode::CLASSICAL, &options)?, Engine::Focus);
/// assert_eq!(engine_for(goal, Mode::INTUITIONISTIC, &options)?, Engine::TwoSided);
/// # Ok::<(), deduxit::Error>(())
/// ```
pub fn engine_for(goal: Goal<'_>, mode: Mode, options: &Options) -> Result<Engine, Error> {
    let occurrences = goal.occurrences();
    let fragment = fragment_of(goal.forest, &occurrences, options)?;
    let reading = read(goal.forest, mode)?;
    prepare(
        goal,
        &occurrences,
        mode,
        fragment,
        options,
        reading.as_ref(),
    )
    .map(|(_, engine)| engine)
}

/// Returns the fragment a goal is searched in: the one the options
/// assert, or else the goal's own.
///
/// # Errors
///
/// [`Error::FragmentMismatch`] when the goal is outside the asserted
/// fragment.
fn fragment_of(forest: &Forest, goal: &[OccId], options: &Options) -> Result<Fragment, Error> {
    let detected = goal_fragment(forest, goal);
    match options.fragment {
        Some(asserted) if !asserted.contains(detected) => {
            Err(Error::FragmentMismatch { asserted, detected })
        }
        Some(asserted) => Ok(asserted),
        None => Ok(detected),
    }
}

/// Returns the forest's intuitionistic reading in intuitionistic mode,
/// and `None` in classical mode.
///
/// # Errors
///
/// The mode's own error from [`Mode::check`], and
/// [`Error::NotIntuitionistic`] for a forest with no reading.
fn read(forest: &Forest, mode: Mode) -> Result<Option<Reading<'_>>, Error> {
    mode.check()?;
    if !mode.intuitionistic {
        return Ok(None);
    }
    Reading::of_mode(forest, mode)
        .map(Some)
        .map_err(Error::NotIntuitionistic)
}

/// Returns the task of a goal searched in `fragment`, and the engine that
/// decides it, which admits it.
///
/// # Errors
///
/// [`Error::GoalOutputs`] for an intuitionistic goal without exactly one
/// output under the reading given, [`Error::OrderedFragment`] for an
/// ordered mode beyond unit-free MLL, and [`Error::EngineRefused`] for an
/// engine that does not take the goal. The dispatch's own error where no
/// engine does.
fn prepare<'a>(
    goal: Goal<'a>,
    occurrences: &'a [OccId],
    mode: Mode,
    fragment: Fragment,
    options: &Options,
    reading: Option<&'a Reading<'a>>,
) -> Result<(Task<'a>, Engine), Error> {
    if let Some(reading) = reading {
        let outputs = reading.outputs(occurrences.iter().copied());
        if outputs != 1 {
            return Err(Error::GoalOutputs { count: outputs });
        }
    }
    // The roots in any order are the sequent itself, which the engines
    // are handed in the forest's order.
    let task = Task {
        forest: goal.forest,
        goal: occurrences,
        fragment,
        mode,
        reading,
        roots: goal.conclusion,
        program: OnceLock::new(),
    };
    if mode.is_ordered() && !Fragment::MLL.contains(fragment) {
        return Err(Error::OrderedFragment { fragment, mode });
    }
    let engine = match options.engine {
        Some(Engine::Unknown) => {
            return Err(Error::UnknownName {
                what: "engine",
                name: Engine::Unknown.name().into(),
                known: Engine::NAMES,
            });
        }
        Some(engine) => engine,
        None => dispatch(&task)?,
    };
    // Only the planar searches keep the order of the formulas: the net
    // engine's in the cyclic mode, the essential one's in the Lambek
    // calculus.
    let planar = if mode.intuitionistic {
        Engine::Essential
    } else {
        Engine::Net
    };
    if mode.is_ordered() && engine != planar {
        return Err(Error::EngineRefused {
            engine,
            because: NotTaken::Mode { mode },
        });
    }
    engine.implementation().admits(&task)?;
    Ok((task, engine))
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::dispatch::{PROBE_AFTER, TURN, TURN_SHARE};
    use super::*;
    use crate::linear::nets::ProofStructure;
    use crate::linear::proofs::{Node, NodeId};
    use crate::linear::sequents::Atom;

    /// The check of the proof carried back to a forest with cut formulas
    /// that the stop gives up makes the verdict unknown, as for any other
    /// proof, never an error. The proof is past the checker's first poll
    /// (4 096 nodes); the stop fires at the first poll of that second
    /// check, after the check of the proof on the cut-free forest.
    #[cfg(feature = "parse")]
    #[test]
    fn a_check_given_up_on_a_forest_with_cuts_is_unknown() {
        let atoms: Vec<String> = (0..1_500).map(|i| format!("a{i}")).collect();
        let product = atoms.join(" * ");
        let mut sequent: Sequent = format!("{product} |- {product}").parse().unwrap();
        sequent.add_cut("|- bot".parse().unwrap()).unwrap();
        let run = |fire_at: usize| {
            let mut polls = 0;
            let outcome = prove_within(
                &sequent,
                Mode::CLASSICAL,
                &Options::default(),
                &Limits::default(),
                |progress| {
                    if progress.phase != Phase::Check {
                        return false;
                    }
                    polls += 1;
                    polls == fire_at
                },
            )
            .unwrap();
            (outcome, polls)
        };
        // Both checks are of proofs of one shape, so each polls alike.
        let (outcome, polls) = run(0);
        assert!(outcome.checked && polls >= 2 && polls % 2 == 0, "{polls}");
        let (outcome, _) = run(polls / 2 + 1);
        assert!(
            matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)),
            "{:?}",
            outcome.verdict
        );
        assert!(!outcome.checked);
    }

    /// A forest with cut formulas is searched on a cut-free forest of the
    /// goal's formulas, by every engine that takes the goal. No engine
    /// sees a cut formula. Every proof found is cut-free, and checks
    /// against the goal, which may hold occurrences of the cut formulas.
    /// A sequent with cut formulas is proved as its conclusion.
    #[cfg(feature = "parse")]
    #[test]
    fn a_forest_with_cuts_is_searched_without_them() {
        let mut sequent: Sequent = "A, A -o B |- B".parse().unwrap();
        sequent.add_cut("|- A -o B".parse().unwrap()).unwrap();
        // 0 ~A, 1 A ⊗ ~B, 2 A, 3 ~B, 4 B, then 5 ~A ⅋ B and its dual 8.
        let forest = Forest::new(&sequent).unwrap();
        let cut_free = |proof: &Proof| !proof.nodes().iter().any(|n| matches!(n, Node::Cut(..)));
        let limits = Limits::default();
        for mode in [Mode::CLASSICAL, Mode::INTUITIONISTIC] {
            let focused = if mode.intuitionistic {
                Engine::TwoSided
            } else {
                Engine::Focus
            };
            for engine in [None, Some(focused), Some(Engine::Net)] {
                let options = Options::default().with_engine(engine);
                let outcome =
                    prove_goal(Goal::conclusion(&forest), mode, &options, &limits, |_| {
                        false
                    })
                    .unwrap();
                let proof = outcome.verdict.proof().expect("provable");
                assert!(cut_free(proof) && proof.goal().is_none());
                assert_eq!(proof.sequent(), &sequent);
                assert_eq!(proof.check(mode), Ok(()));
            }
            let members = [Member::new(1), Member::new(5)];
            let goal = Goal::new(&forest, &members).unwrap();
            let outcome = prove_goal(goal, mode, &Options::default(), &limits, |_| false).unwrap();
            let proof = outcome.verdict.proof().expect("A ⊸ B ⊢ A ⊸ B");
            assert!(cut_free(proof));
            assert_eq!(proof.goal(), Some(&members[..]));
            assert_eq!(proof.check(mode), Ok(()));
            // The dual and the goal: ⊢ A ⊗ ~B, B has no proof.
            let members = [Member::new(8), Member::new(4)];
            let goal = Goal::new(&forest, &members).unwrap();
            let outcome = prove_goal(goal, mode, &Options::default(), &limits, |_| false).unwrap();
            assert!(matches!(outcome.verdict, Verdict::Unprovable(_)), "{mode}");
        }
    }

    /// Parses `input`.
    fn sequent(input: &str) -> Sequent {
        input.parse().unwrap_or_else(|e| panic!("{input:?}: {e}"))
    }

    /// An outcome is checked exactly when it is a proof and the options
    /// ask for the check. A cadence of links converts from a number.
    #[test]
    fn checked_when_proved_and_asked() {
        let tautology = sequent("|- a, ~a");
        let proved = prove(&tautology, Mode::CLASSICAL, &Options::default()).unwrap();
        assert!(proved.checked);
        let unchecked = Options::default().with_check(false);
        assert!(
            !prove(&tautology, Mode::CLASSICAL, &unchecked)
                .unwrap()
                .checked
        );
        let refuted = prove(&sequent("|- a, a"), Mode::CLASSICAL, &Options::default()).unwrap();
        assert!(matches!(refuted.verdict, Verdict::Unprovable(_)) && !refuted.checked);
        assert_eq!(Cadence::from(7), Cadence::Every(7));
    }

    /// Every counter an engine lists is a field of the statistics' JSON
    /// form, once.
    #[cfg(feature = "serialize")]
    #[test]
    fn counters_are_fields() {
        // Every counter is set, since the first-order ones are written only
        // when they are not zero.
        let written = serde_json::to_value(Statistics {
            unifications: 1,
            bindings: 1,
            ..Statistics::default()
        })
        .unwrap();
        for &engine in Engine::ALL {
            let keys: Vec<&str> = engine.counters().iter().map(|c| c.key).collect();
            for key in &keys {
                assert!(written.get(key).is_some(), "{engine}: {key}");
            }
            let mut unique = keys.clone();
            unique.sort_unstable();
            unique.dedup();
            assert_eq!(unique.len(), keys.len(), "{engine}");
        }
    }

    /// Every reason has its word, listed in `NAMES` in its order, and in
    /// JSON its `kind` is that word. The same holds for the verdicts'
    /// words.
    #[test]
    fn reasons_and_verdicts_are_named() {
        let reasons = [
            Reason::Stopped,
            Reason::RecursionLimit { depth: 1 },
            Reason::CopyBound { copies: 1 },
            Reason::MemoryLimit { limit_bytes: 1 },
            Reason::IndexLimit,
            Reason::WorkLimit { limit: 1 },
            Reason::Unchecked { limit_bytes: 1 },
            Reason::WitnessDepth { depth: 1 },
            Reason::Instances { instances: 1 },
            Reason::Incomplete,
            Reason::PathLength { length: 1 },
            Reason::ModelSize { size: 1 },
            Reason::KripkeSize { worlds: 1, size: 1 },
        ];
        // No wildcard: a new reason fails to compile here until listed.
        let at = |reason: &Reason| match reason {
            Reason::Stopped => 0,
            Reason::RecursionLimit { .. } => 1,
            Reason::CopyBound { .. } => 2,
            Reason::MemoryLimit { .. } => 3,
            Reason::IndexLimit => 4,
            Reason::WorkLimit { .. } => 5,
            Reason::Unchecked { .. } => 6,
            Reason::WitnessDepth { .. } => 7,
            Reason::Instances { .. } => 8,
            Reason::Incomplete => 9,
            Reason::PathLength { .. } => 10,
            Reason::ModelSize { .. } => 11,
            Reason::KripkeSize { .. } => 12,
        };
        assert_eq!(reasons.len(), Reason::NAMES.len());
        for (i, reason) in reasons.iter().enumerate() {
            assert_eq!(at(reason), i);
            assert_eq!(reason.name(), Reason::NAMES[i]);
            #[cfg(feature = "serialize")]
            assert_eq!(serde_json::to_value(reason).unwrap()["kind"], reason.name());
        }
        // The verdicts likewise, each written as its word.
        let verdict_at = |verdict: &Verdict| match verdict {
            Verdict::Proved(_) => 0,
            Verdict::Unprovable(_) => 1,
            Verdict::Unknown(_) => 2,
        };
        let bounded = Options::default()
            .with_engine(Some(Engine::Focus))
            .with_bias(Bias::Rarer)
            .with_copies(Some(0));
        let mut seen = Vec::new();
        for (text, options) in [
            ("a |- a", Options::default()),
            ("a |- b", Options::default()),
            ("!a |- a * a", bounded),
        ] {
            let outcome = prove(&sequent(text), Mode::CLASSICAL, &options).unwrap();
            let i = verdict_at(&outcome.verdict);
            assert_eq!(outcome.verdict.name(), Verdict::NAMES[i]);
            #[cfg(feature = "serialize")]
            assert_eq!(
                serde_json::to_value(&outcome).unwrap()["verdict"],
                outcome.verdict.name()
            );
            seen.push(i);
        }
        assert_eq!((seen, Verdict::NAMES.len()), (vec![0, 1, 2], 3));
        // And the refutations, each its JSON `kind`.
        let refutations = [
            Refutation::Exhausted,
            Refutation::Unbalanced(Unbalanced {
                atom: Atom::new(0),
                least: 1,
                most: 1,
            }),
            Refutation::OneSided(OneSided {
                atom: Atom::new(0),
                least: 1,
                most: 1,
                copied: crate::linear::occurrences::Sign::Atom,
            }),
            Refutation::Equation(Equation {
                formulas: 1,
                needed: 2,
                tensors: 0,
                pars: 0,
                ones: 0,
                bottoms: 0,
                mix: false,
            }),
            Refutation::StateEquation(StateEquation {
                atoms: Vec::new(),
                clauses: Vec::new(),
                dropped: Vec::new(),
            }),
            Refutation::Classical(Classical {
                assignment: Vec::new(),
            }),
            Refutation::Saturated(Saturated::new(Vec::new(), Vec::new())),
            Refutation::UnbalancedSymbol(UnbalancedSymbol {
                symbol: crate::linear::sequents::Symbol::new(0),
                least: 1,
                most: 1,
            }),
            Refutation::OneSidedSymbol(OneSidedSymbol {
                symbol: crate::linear::sequents::Symbol::new(0),
                least: 1,
                most: 1,
                copied: crate::linear::occurrences::Sign::Atom,
            }),
            Refutation::Basis(Basis::new(
                crate::linear::occurrences::Sign::Atom,
                Vec::new(),
                Vec::new(),
            )),
        ];
        let refutation_at = |refutation: &Refutation| match refutation {
            Refutation::Exhausted => 0,
            Refutation::Unbalanced(_) => 1,
            Refutation::OneSided(_) => 2,
            Refutation::Equation(_) => 3,
            Refutation::StateEquation(_) => 4,
            Refutation::Classical(_) => 5,
            Refutation::Saturated(_) => 6,
            Refutation::UnbalancedSymbol(_) => 7,
            Refutation::OneSidedSymbol(_) => 8,
            Refutation::Basis(_) => 9,
        };
        assert_eq!(refutations.len(), Refutation::NAMES.len());
        for (i, refutation) in refutations.iter().enumerate() {
            assert_eq!(refutation_at(refutation), i);
            assert_eq!(refutation.name(), Refutation::NAMES[i]);
            #[cfg(feature = "serialize")]
            assert_eq!(
                serde_json::to_value(refutation).unwrap()["kind"],
                refutation.name()
            );
        }
    }

    /// The listed names are the names written, in order, and each reads
    /// back as its value.
    #[test]
    fn names_are_listed() {
        // Every variant has an arm, with no wildcard, and its place in the
        // list. A new variant fails to compile here until it has an arm,
        // and the lengths differ until it is listed.
        let engine_at = |engine| match engine {
            Engine::Focus => 0,
            Engine::Net => 1,
            Engine::TwoSided => 2,
            Engine::Additive => 3,
            Engine::Horn => 4,
            Engine::Essential => 5,
            Engine::Inverse => 6,
            // Not listed: no search runs it.
            Engine::Unknown => usize::MAX,
        };
        let bias_at = |bias| match bias {
            Bias::Auto => 0,
            Bias::Rarer => 1,
            Bias::Factors => 2,
        };
        assert_eq!(Engine::ALL.len(), 7);
        assert_eq!(Bias::ALL.len(), 3);
        for (i, &engine) in Engine::ALL.iter().enumerate() {
            assert_eq!(engine_at(engine), i);
        }
        for (i, &bias) in Bias::ALL.iter().enumerate() {
            assert_eq!(bias_at(bias), i);
        }
        let engines: Vec<_> = Engine::ALL.iter().map(|e| e.name()).collect();
        let biases: Vec<_> = Bias::ALL.iter().map(|b| b.name()).collect();
        assert_eq!(engines, Engine::NAMES);
        assert_eq!(biases, Bias::NAMES);
        for &engine in Engine::ALL {
            assert_eq!(engine.to_string().parse::<Engine>().unwrap(), engine);
        }
        for &bias in Bias::ALL {
            assert_eq!(bias.to_string().parse::<Bias>().unwrap(), bias);
        }
    }

    /// An engine's proof that the checker rejects is a defect, never a
    /// verdict. The proof here has a weakening in classical mode, as an
    /// affine search would leave it.
    #[test]
    fn a_rejected_proof_is_an_error() {
        let s = sequent("|- a, b, ~a");
        let forest = Forest::new(&s).unwrap();
        let (m, n) = (Member::new, NodeId::new);
        let nodes = vec![Node::Ax(m(0), m(2)), Node::Weaken(m(1), n(0))];
        let proof = Proof::new(forest.clone(), nodes, n(1)).unwrap();
        let goal = Concluding {
            forest: &forest,
            occurrences: forest.roots(),
            fragment: s.fragment(),
            mode: Mode::CLASSICAL,
            roots: true,
            members: &[],
        };
        let verdict = conclude(
            Ok(Some(proof)),
            None,
            goal,
            &Options::default(),
            &Limits::default(),
            &memory::Account::new(None),
            &mut |_| false,
        );
        assert!(matches!(verdict, Err(Error::Rejected(_))), "{verdict:?}");
    }

    /// A proof whose check a bound gives up is not returned. The verdict
    /// is unknown, by the bound that refused the check. For memory that is
    /// the bound as the limits set it, of which the check had what the
    /// input left.
    #[test]
    fn a_refused_check_is_unknown() {
        let refusals = [
            Refusal::stopped(Phase::Check),
            Refusal::Memory {
                phase: Phase::Check,
                limit_bytes: 48,
                needed_bytes: None,
            },
            Refusal::Work { limit: 1 },
        ];
        let limits = Limits::default().with_memory_bytes(Some(64));
        let reasons = refusals.map(|r| unchecked(r, &limits).unwrap());
        assert_eq!(
            reasons,
            [
                Reason::Stopped,
                Reason::Unchecked { limit_bytes: 64 },
                Reason::WorkLimit { limit: 1 }
            ]
        );
        // The check counts its work too, so a proof the search finds within
        // a bound of one unit is unknown.
        let work = Limits::default().with_work(Some(1));
        let outcome = prove_within(
            &sequent("a, b |- a * b"),
            Mode::CLASSICAL,
            &Options::default(),
            &work,
            |_| false,
        )
        .unwrap();
        assert!(
            matches!(
                outcome.verdict,
                Verdict::Unknown(Reason::WorkLimit { limit: 1 })
            ),
            "{:?}",
            outcome.verdict
        );
        assert!(!outcome.checked);
    }

    /// A proof the search finds within the memory bound, but whose check
    /// would need more, is unknown by that bound: neither a proof nor an
    /// error.
    ///
    /// The Horn engine holds a counter's firing sequence of 3 000 tokens in
    /// less memory than the check of its proof holds (a state per firing,
    /// each waiting for its reader). So the first bound, upwards from
    /// 512 KiB, at which the search no longer ends at the bound is one
    /// that the check passes.
    #[test]
    fn a_check_past_the_memory_bound_is_unknown() {
        let counter = sequent(&format!(
            "!(a -o a * b), a |- a * {}",
            vec!["b"; 3000].join(" * ")
        ));
        let first = (16..64u64).map(|k| k << 15).find_map(|bytes| {
            let limits = Limits::default().with_memory_bytes(Some(bytes));
            let outcome = prove_within(
                &counter,
                Mode::INTUITIONISTIC,
                &Options::default(),
                &limits,
                |_| false,
            )
            .unwrap();
            match outcome.verdict {
                Verdict::Unknown(Reason::MemoryLimit { .. }) => None,
                verdict => Some((bytes, verdict, outcome.checked)),
            }
        });
        assert!(
            matches!(
                first,
                Some((bytes, Verdict::Unknown(Reason::Unchecked { limit_bytes }), false))
                    if limit_bytes == bytes
            ),
            "{first:?}"
        );
    }

    /// The memory bound counts the input too: the forest with its
    /// sequent.
    ///
    /// `⊢ a, ~a` with an atom's name of a mebibyte, which the search itself
    /// never copies, ends at a bound one byte short of the input, at its
    /// first stable sequent. That holds through the front door and through
    /// the race of the command's default alike. With half the input more,
    /// it proves. So the input is counted once, and the proof's check has
    /// the room beside it.
    #[test]
    fn the_memory_bound_holds_the_input() {
        let name = "a".repeat(1 << 20);
        let s = sequent(&format!("|- {name}, ~{name}"));
        let forest = Forest::new(&s).unwrap();
        let goal = Goal::conclusion(&forest);
        let input = forest.held_bytes() as u64;
        assert!(input > 1 << 20);
        let options = Options::default().with_engine(Some(Engine::Focus));
        for (bytes, proved) in [(input - 1, false), (input + input / 2, true)] {
            let limits = Limits::default().with_memory_bytes(Some(bytes));
            let alone = prove_goal(goal, Mode::CLASSICAL, &options, &limits, |_| false);
            #[cfg(feature = "parallel")]
            let raced = race(
                goal,
                Mode::CLASSICAL,
                &options,
                &limits,
                3,
                |_| true,
                |_| false,
            );
            #[cfg(not(feature = "parallel"))]
            let raced = prove_goal(goal, Mode::CLASSICAL, &options, &limits, |_| false);
            for outcome in [alone, raced] {
                let verdict = outcome.unwrap().verdict;
                let expected = if proved {
                    matches!(verdict, Verdict::Proved(_))
                } else {
                    matches!(verdict, Verdict::Unknown(Reason::MemoryLimit { limit_bytes })
                        if limit_bytes == bytes)
                };
                let shown = match &verdict {
                    Verdict::Unknown(reason) => format!("{reason:?}"),
                    Verdict::Proved(_) => "proved".to_string(),
                    Verdict::Unprovable(_) => "unprovable".to_string(),
                };
                assert!(expected, "under {bytes} bytes: {shown}");
            }
        }
    }

    /// Every poll tells the stop the work done since the last and in all,
    /// in the engine's units. The bound on work ends a search there, with
    /// an unknown verdict that names the bound.
    #[test]
    fn the_search_counts_its_work() {
        let partition = crate::linear::families::FAMILIES
            .iter()
            .find(|f| f.name == "partition-no")
            .unwrap()
            .instance(3, 0)
            .unwrap();
        let atoms: Vec<String> = (0..8).map(|i| format!("a{i}")).collect();
        let tensor = sequent(&format!("{} |- {}", atoms.join(", "), atoms.join(" * ")));
        for (sequent, mode, engine) in [
            (&partition.sequent, partition.mode, Engine::Focus),
            (&tensor, Mode::CLASSICAL, Engine::Net),
        ] {
            // The search's work alone, without the classical refuter's.
            let options = Options::default()
                .with_engine(Some(engine))
                .with_classical_work(0);
            let (mut sum, mut last, mut polls) = (0, 0, 0);
            let outcome = prove_within(sequent, mode, &options, &Limits::default(), |progress| {
                assert_eq!(progress.phase, Phase::Search);
                sum += progress.work;
                last = progress.done;
                polls += 1;
                false
            })
            .unwrap();
            assert!(!matches!(outcome.verdict, Verdict::Unknown(_)), "{engine}");
            assert!(
                polls > 1 && last > 1 && sum == last,
                "{engine}: {sum} {last}"
            );
            assert_eq!(outcome.statistics.work, last, "{engine}");
            let bound = Limits::default().with_work(Some(last / 2));
            let mut told = 0;
            let outcome = prove_within(sequent, mode, &options, &bound, |progress| {
                told += progress.work;
                false
            })
            .unwrap();
            assert!(
                matches!(outcome.verdict, Verdict::Unknown(Reason::WorkLimit { limit }) if limit == last / 2),
                "{engine}: {:?}",
                outcome.verdict
            );
            // The poll that passes the bound is told too.
            assert_eq!(outcome.statistics.work, told, "{engine}");
        }
    }

    /// A turn tells the caller the units of every poll, including the one
    /// that ends it.
    #[test]
    fn a_turn_tells_the_caller_every_poll() {
        let mut told = 0;
        let mut caller = |units| {
            told += units;
            false
        };
        {
            let mut turn = Stop::Turn(&mut caller, 5);
            assert!(!turn.fired(3));
            assert!(turn.fired(3), "the turn is over");
        }
        assert_eq!(told, 6);
    }

    /// Beside the default bias's second search, which reads the shared
    /// count, the bound on work is late by a few batches at most. The
    /// calling thread adds its units in batches too. If it kept them to
    /// the end, the second search would run a whole slice past the bound.
    #[cfg(feature = "parallel")]
    #[test]
    fn the_work_bound_holds_beside_the_second_search() {
        let tokens = vec!["a"; 32].join(", ");
        let s = sequent(&format!(
            "!(a * a -o b), !(b * b -o c), !(c * c -o d), !(d * d -o e), !(e * e -o f), \
             !(a -o x), !(x -o y), !(y -o a), !(a -o u), !(u -o v), !(v -o a), !(b -o w), \
             !(w -o b), !(x -o u), {tokens} |- f * a"
        ));
        let options = Options::default()
            .with_engine(Some(Engine::Focus))
            .with_copies(None)
            .with_jobs(1);
        let limit = 300_000;
        let limits = Limits::default().with_work(Some(limit));
        let outcome = prove_within(&s, Mode::CLASSICAL, &options, &limits, |_| false).unwrap();
        assert!(
            matches!(outcome.verdict, Verdict::Unknown(Reason::WorkLimit { .. })),
            "{:?}",
            outcome.verdict
        );
        assert!(
            outcome.statistics.work < limit + 4 * Work::BATCH,
            "{} units under a bound of {limit}",
            outcome.statistics.work
        );
    }

    /// The race answers with the side that decided, whichever it is. The
    /// caller's stop ends one side at its first poll (the pool's thread is
    /// told by its name), and the other side proves the sequent.
    #[cfg(feature = "parallel")]
    #[test]
    fn the_race_answers_with_the_side_that_decided() {
        let s = sequent("!(a -o b), !(b -o c), a |- c * !(d -o d)");
        let forest = Forest::new(&s).unwrap();
        let goal = Goal::conclusion(&forest);
        for pool_stops in [false, true] {
            let outcome = race(
                goal,
                Mode::CLASSICAL,
                &Options::default(),
                &Limits::default(),
                3,
                |_| true,
                |_| (std::thread::current().name() == Some("deduxit-race")) == pool_stops,
            )
            .unwrap();
            assert!(
                matches!(outcome.verdict, Verdict::Proved(_)),
                "the pool stopped: {pool_stops}, {:?}",
                outcome.verdict
            );
        }
    }

    /// Where the engine of a row with turns runs past its head start, the
    /// inverse method takes its turn, and its decision answers. Where the
    /// turn does not decide, the engine answers as it does alone, with its
    /// own counters and the turn's work added.
    ///
    /// The head start and the turn are set low on this thread, so that
    /// small goals reach them. Under Mix, the focused engine searches the
    /// partitions of `mix(8)`'s context for a second, and the two-sided
    /// engine takes some hundred units on the intuitionistic goal. The
    /// inverse method refutes both at once. At the head start and share of
    /// every search, it refutes a goal that the two-sided engine, deepening
    /// its copy bound, would leave to the time limit.
    #[test]
    fn the_inverse_method_takes_its_turn() {
        let options = Options::default().with_jobs(1);
        let intuitionistic = sequent("!(a -o b & c), a |- b * c");
        let alone = prove(&intuitionistic, Mode::INTUITIONISTIC, &options).unwrap();
        assert_eq!(alone.engine, Engine::TwoSided);
        TURN.with(|t| t.set((16, 0)));
        for (s, mode) in [
            (crate::linear::families::mix(8), Mode::CLASSICAL.with_mix()),
            (intuitionistic.clone(), Mode::INTUITIONISTIC),
        ] {
            let outcome = prove(&s, mode, &options).unwrap();
            assert_eq!(outcome.engine, Engine::Inverse, "{mode}");
            assert!(
                matches!(outcome.verdict, Verdict::Unprovable(_)),
                "{:?}",
                outcome.verdict
            );
        }
        // Turns that end before they decide.
        TURN.with(|t| t.set((16, u64::MAX)));
        let turned = prove(&intuitionistic, Mode::INTUITIONISTIC, &options).unwrap();
        TURN.with(|t| t.set((PROBE_AFTER, TURN_SHARE)));
        assert_eq!(turned.engine, Engine::TwoSided);
        assert_eq!(
            format!("{:?}", turned.verdict),
            format!("{:?}", alone.verdict)
        );
        let counters = |s: &Statistics| (s.nodes, s.splits, s.memo_hits, s.memo_entries);
        assert_eq!(counters(&turned.statistics), counters(&alone.statistics));
        assert!(turned.statistics.work > alone.statistics.work);
        // With the head start and share of every search: the two-sided
        // engine, deepening without a copy bound as the command does,
        // leaves this goal to the time limit. A turn refutes it once the
        // engine's work passes the head start.
        let saturating = sequent("!(a -o a * a * a), !(a -o a), a |- a * (a + 0)");
        let deepening = options.clone().with_copies(None);
        let outcome = prove(&saturating, Mode::INTUITIONISTIC, &deepening).unwrap();
        assert_eq!(outcome.engine, Engine::Inverse);
        assert!(matches!(outcome.verdict, Verdict::Unprovable(_)));
    }

    /// The caller's stop ends both sides of the race, whether or not the
    /// other has decided. Asked from the pool too, it stops a search that
    /// would prove the sequent, and the race answers unknown.
    #[cfg(feature = "parallel")]
    #[test]
    fn the_caller_s_stop_ends_both_sides_of_the_race() {
        let s = crate::linear::families::mix(5);
        let forest = Forest::new(&s).unwrap();
        let outcome = race(
            Goal::conclusion(&forest),
            Mode::CLASSICAL.with_mix(),
            &Options::default(),
            &Limits::default(),
            3,
            |_| true,
            |_| true,
        )
        .unwrap();
        assert!(
            matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)),
            "{:?}",
            outcome.verdict
        );
    }

    /// A panic on one side of the race stops the other side, and then
    /// reaches the caller. Here the pool's stop panics, and the single
    /// search, which would take minutes, ends at its next poll.
    #[cfg(feature = "parallel")]
    #[test]
    fn a_panic_on_one_side_of_the_race_stops_the_other() {
        let (sender, ended) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let s = crate::linear::families::mix(11);
            let forest = Forest::new(&s).unwrap();
            let goal = Goal::conclusion(&forest);
            let panicked = std::panic::catch_unwind(|| {
                race(
                    goal,
                    Mode::CLASSICAL.with_mix(),
                    &Options::default(),
                    &Limits::default(),
                    3,
                    |_| true,
                    |_| {
                        let pool = std::thread::current().name() == Some("deduxit-race");
                        assert!(!pool, "the pool's stop panics");
                        false
                    },
                )
            });
            sender.send(panicked.is_err()).unwrap();
        });
        let panicked = ended.recv_timeout(std::time::Duration::from_secs(20));
        assert_eq!(panicked, Ok(true));
    }

    /// The race counts every thread. Below three, it asks nothing and
    /// searches as one call with that many threads would. From three, it
    /// starts the pool once, when asked to, and answers as one thread
    /// does.
    #[cfg(feature = "parallel")]
    #[test]
    fn the_race_counts_every_thread() {
        let s = sequent("!(a -o b), !(b -o c), a |- c * !(d -o d)");
        let forest = Forest::new(&s).unwrap();
        let goal = Goal::conclusion(&forest);
        let (options, limits) = (Options::default(), Limits::default());
        for threads in [1, 2] {
            let outcome = race(
                goal,
                Mode::CLASSICAL,
                &options,
                &limits,
                threads,
                |_| panic!("no pool is added below three threads"),
                |_| false,
            )
            .unwrap();
            assert!(outcome.verdict.proof().is_some(), "{threads}");
        }
        let alone = prove(&s, Mode::CLASSICAL, &options).unwrap();
        let mut asked = 0;
        let raced = race(
            goal,
            Mode::CLASSICAL,
            &options,
            &limits,
            3,
            |_| {
                asked += 1;
                true
            },
            |_| false,
        )
        .unwrap();
        assert_eq!(asked, 1);
        assert!(raced.verdict.proof().is_some());
        assert_eq!(raced.engine, alone.engine);
    }

    /// The front door's plain parts:
    /// - a sequent past the bound on occurrences is refused before
    ///   anything unfolds;
    /// - the engines that search on a pool are the focused engines and the
    ///   net engine;
    /// - the default options are their documented constants;
    /// - each refutation says its counts in words, with the singular, a
    ///   range, the minus sign and Mix's inequality where they apply.
    #[test]
    fn the_front_door_s_plain_parts() {
        let tight = Limits::default().with_occurrences(Some(2));
        let error = prove_within(
            &sequent("|- a, ~a, b"),
            Mode::CLASSICAL,
            &Options::default(),
            &tight,
            |_| false,
        )
        .unwrap_err();
        assert!(
            matches!(error, Error::Refused(Refusal::Occurrences { limit: 2, .. })),
            "{error}"
        );
        let parallel: Vec<bool> = Engine::ALL.iter().map(|e| e.parallel()).collect();
        assert_eq!(parallel, [true, true, true, false, false, false, false]);
        let defaults = Options::default();
        assert_eq!(
            (
                defaults.copies,
                defaults.forward_copies,
                defaults.memo_limit,
                defaults.check
            ),
            (
                Some(Options::DEFAULT_COPIES),
                Options::DEFAULT_FORWARD_COPIES,
                Options::DEFAULT_MEMO_LIMIT,
                Options::DEFAULT_CHECK
            )
        );
        assert_eq!(
            (Options::DEFAULT_COPIES, Options::DEFAULT_MEMO_LIMIT),
            (3, 1 << 20)
        );
        let atom = Atom::new(0);
        for (refutation, says) in [
            (
                Refutation::Unbalanced(Unbalanced {
                    atom,
                    least: 1,
                    most: 1,
                }),
                "#0 occurs 1 more time than ~#0 in the one-sided sequent, so",
            ),
            (
                Refutation::Unbalanced(Unbalanced {
                    atom,
                    least: -3,
                    most: -2,
                }),
                "~#0 occurs 2 to 3 more times than #0 in the one-sided sequent, whichever",
            ),
            (
                Refutation::OneSided(OneSided {
                    atom,
                    least: 1,
                    most: 1,
                    copied: crate::linear::occurrences::Sign::Atom,
                }),
                "#0 occurs 1 more time than ~#0 outside the exponentials of the one-sided \
                 sequent, and every literal of the atom below one, which copies add, is #0, so",
            ),
            (
                // The copies' literal is written as the fields name it, even
                // where a form read from JSON contradicts the counts.
                Refutation::OneSided(OneSided {
                    atom,
                    least: -2,
                    most: -1,
                    copied: crate::linear::occurrences::Sign::Atom,
                }),
                "~#0 occurs 1 to 2 more times than #0 outside the exponentials of the one-sided \
                 sequent, whichever additive alternatives a proof takes, and every literal of the \
                 atom below one, which copies add, is #0, so",
            ),
            (
                Refutation::Equation(Equation {
                    formulas: 3,
                    needed: -1,
                    tensors: 0,
                    pars: 3,
                    ones: 0,
                    bottoms: 0,
                    mix: true,
                }),
                "has at least #⊗ − #⅋ − #1 + #⊥ + 2 formulas, here 0 − 3 − 0 + 0 + 2 = −1, and this \
                 one has 3",
            ),
            (
                Refutation::StateEquation(StateEquation {
                    atoms: vec![(atom, -2), (Atom::new(1), 1)],
                    clauses: vec![(OccId::new(4), 1)],
                    dropped: Vec::new(),
                }),
                "weighting each #0 by −2, #1 by 1 and the clauses used once by weights of their \
                 own, no clause",
            ),
        ] {
            let text = refutation.to_string();
            assert!(text.contains(says), "{text}");
        }
    }

    /// A reason names the settings key a caller raises to search further,
    /// and says the bound it reached.
    #[test]
    fn reasons_name_their_setting() {
        for (reason, setting, text) in [
            (Reason::Stopped, None, "the search was stopped"),
            (
                Reason::RecursionLimit { depth: 7 },
                Some("limits.recursion_depth"),
                "the recursion limit of 7 was reached",
            ),
            (
                Reason::CopyBound { copies: 3 },
                Some("search.copies"),
                "the copy bound of 3 was reached",
            ),
            (
                Reason::MemoryLimit {
                    limit_bytes: 1 << 20,
                },
                Some("limits.memory_bytes"),
                "the memory limit of 1 MiB was reached",
            ),
            (
                Reason::MemoryLimit {
                    limit_bytes: u64::MAX,
                },
                None,
                "the memory the allocator would give was used up",
            ),
            (
                Reason::IndexLimit,
                None,
                "the search outgrew what its indices address",
            ),
            (
                Reason::WorkLimit { limit: 9 },
                Some("limits.work"),
                "the work limit of 9 was reached",
            ),
            (
                Reason::Unchecked { limit_bytes: 1024 },
                Some("limits.memory_bytes"),
                "a proof was found, but its check would hold more than the memory limit of 1 KiB",
            ),
        ] {
            assert_eq!(
                (reason.setting(), reason.to_string().as_str()),
                (setting, text)
            );
        }
    }

    /// Two searches' counters add up, except the memo's peak and the copy
    /// bounds reached, which are the larger of the two.
    #[test]
    fn statistics_add_up() {
        let mut first = Statistics {
            nodes: 3,
            memo_hits: 1,
            memo_entries: 7,
            splits: 2,
            links: 1,
            tests: 4,
            copies: 2,
            forward_copies: 9,
            work: 10,
            unifications: 6,
            bindings: 2,
        };
        let second = Statistics {
            nodes: 5,
            memo_hits: 2,
            memo_entries: 4,
            splits: 1,
            links: 3,
            tests: 1,
            copies: 5,
            forward_copies: 0,
            work: 20,
            unifications: 1,
            bindings: 3,
        };
        first.add(&second);
        assert_eq!(
            first,
            Statistics {
                nodes: 8,
                memo_hits: 3,
                memo_entries: 7,
                splits: 3,
                links: 4,
                tests: 5,
                copies: 5,
                forward_copies: 9,
                work: 30,
                unifications: 7,
                bindings: 5,
            }
        );
    }

    /// Classical inputs without exponentials go to the focused engine,
    /// unit-free MLL included, and two formulas with an additive
    /// connective or unit to the additive path. The outcome says which
    /// fragment it was searched in.
    #[test]
    fn dispatch() {
        for (input, fragment, engine) in [
            ("|- a, ~a", Fragment::EMPTY, Engine::Focus),
            ("a, a -o b |- b", Fragment::MLL, Engine::Focus),
            ("a * a |- a * a", Fragment::MLL, Engine::Focus),
            ("|- 1, bot", Fragment::MULTIPLICATIVE_UNITS, Engine::Focus),
            ("|- a & b, ~a + ~b", Fragment::ADDITIVES, Engine::Additive),
            ("|- top, 0", Fragment::ADDITIVE_UNITS, Engine::Additive),
            ("|- top, a, b", Fragment::ADDITIVE_UNITS, Engine::Focus),
            ("|- (a * top) + 1, ~a, bot", Fragment::MALL, Engine::Focus),
        ] {
            let outcome = prove(&sequent(input), Mode::CLASSICAL, &Options::default()).unwrap();
            assert_eq!(outcome.fragment, fragment, "{input:?}");
            assert_eq!(outcome.engine, engine, "{input:?}");
            assert_eq!(outcome.net.is_some(), engine == Engine::Net, "{input:?}");
            assert!(outcome.verdict.proof().is_some(), "{input:?}");
        }
    }

    /// Another engine takes over where the focused engine the dispatch
    /// picked ends at a limit on unit-free MLL whose linking is forced.
    /// After the recursion limit, the net engine decides the sequent, in
    /// classical and intuitionistic mode. After the memory bound in
    /// classical mode without Mix, the inverse method does, or the net
    /// engine after it. No engine takes over where an engine is forced, on
    /// equal literals, or outside unit-free MLL.
    #[test]
    fn another_engine_takes_over_at_a_limit() {
        let limits = Limits::default().with_recursion_depth(8);
        let decide = |input: &str, mode: Mode, options: &Options| {
            prove_within(&sequent(input), mode, options, &limits, |_| false).unwrap()
        };
        let chain = "a0, a0 -o a1, a1 -o a2, a2 -o a3, a3 -o a4, a4 -o a5, a5 -o a6, \
                     a6 -o a7, a7 -o a8, a8 -o a9, a9 -o a10 |- a10";
        for mode in [Mode::CLASSICAL, Mode::INTUITIONISTIC] {
            let outcome = decide(chain, mode, &Options::default());
            assert_eq!(outcome.engine, Engine::Net, "{mode}");
            assert!(outcome.verdict.proof().is_some(), "{mode}");
            assert!(outcome.net.is_some(), "{mode}");
        }
        let forced = Options::default().with_engine(Some(Engine::TwoSided));
        let outcome = decide(chain, Mode::INTUITIONISTIC, &forced);
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::RecursionLimit { .. })
        ));
        // The focused engine's set-up meets a bound of 1 MiB on `wide` with
        // 256 pairs, and the inverse method takes over. Under 64 KiB on 64
        // pairs, the inverse method meets the bound too, and the net engine
        // proves the sequent. What each search charged is given back before
        // the next one's proof is checked under the same bound. Under Mix
        // the net engine takes over at once.
        for (pairs, bytes, mode, engine) in [
            (256, 1 << 20, Mode::CLASSICAL, Engine::Inverse),
            (64, 1 << 16, Mode::CLASSICAL, Engine::Net),
            (256, 1 << 20, Mode::CLASSICAL.with_mix(), Engine::Net),
        ] {
            let tight = Limits::default().with_memory_bytes(Some(bytes));
            let wide = crate::linear::families::wide(pairs, 1);
            let outcome =
                prove_within(&wide, mode, &Options::default(), &tight, |_| false).unwrap();
            assert_eq!(outcome.engine, engine, "{pairs} {mode}");
            assert!(outcome.verdict.proof().is_some(), "{:?}", outcome.verdict);
        }
        // Equal literals: the net engine's search could run without end
        // where the focused engine gave up quickly, so the goal stays given
        // up.
        let equal = "a, a, a -o (a -o b0), b0 -o b1, b1 -o b2, b2 -o b3, b3 -o b4, \
                     b4 -o b5, b5 -o b6, b6 -o b7, b7 -o b8, b8 -o b9 |- b9";
        let outcome = decide(equal, Mode::INTUITIONISTIC, &Options::default());
        assert_eq!(outcome.engine, Engine::TwoSided);
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::RecursionLimit { .. })
        ));
        let with_unit = format!("{chain}, 1");
        let outcome = decide(&with_unit, Mode::CLASSICAL, &Options::default());
        assert_eq!(outcome.engine, Engine::Focus);
    }

    /// An unprovable sequent says why, where its counts tell: an atom
    /// whose literals cannot pair up, also across additive alternatives,
    /// or the count equation. Otherwise it says that the search was
    /// exhaustive.
    #[test]
    fn refutations() {
        let refuted = |input: &str, mode: Mode| {
            let outcome = prove(&sequent(input), mode, &Options::default()).unwrap();
            let Verdict::Unprovable(disproof) = outcome.verdict else {
                panic!("{input:?}: {:?}", outcome.verdict);
            };
            assert_eq!(
                (disproof.sequent(), disproof.mode()),
                (&sequent(input), mode)
            );
            *disproof
        };
        let unbalanced = |least, most| {
            Refutation::Unbalanced(Unbalanced {
                atom: Atom::new(0),
                least,
                most,
            })
        };
        let classical = Mode::CLASSICAL;
        assert_eq!(
            refuted("|- a, a", classical).refutation(),
            &unbalanced(2, 2)
        );
        assert_eq!(
            refuted("a |- b", Mode::INTUITIONISTIC).refutation(),
            &unbalanced(-1, -1)
        );
        let hull = refuted("|- (a * a) + (a * a * a), ~a", classical);
        assert_eq!(hull.refutation(), &unbalanced(1, 2));
        assert_eq!(
            hull.to_string(),
            "a occurs 1 to 2 more times than ~a in the one-sided sequent, whichever additive \
             alternatives a proof takes, so they cannot all meet in axioms"
        );
        assert!(
            hull.refutation()
                .to_string()
                .starts_with("#0 occurs 1 to 2 more times than ~#0")
        );
        let equation = refuted("|- a par b, ~a, ~b", classical);
        assert!(matches!(
            equation.refutation(),
            Refutation::Equation(Equation { needed: 1, .. })
        ));
        assert_eq!(
            equation.to_string(),
            "the count equation fails: a provable one-sided sequent of MLL has exactly #⊗ − #⅋ − \
             #1 + #⊥ + 2 formulas, here 0 − 1 − 0 + 0 + 2 = 1, and this one has 3"
        );
        // Where the counts rule out nothing, an assignment that makes the
        // classical reading false certifies the verdict. A tautology leaves
        // only the search's word.
        assert_eq!(
            refuted("|- a par ~a, b * ~b", classical).refutation(),
            &Refutation::Exhausted
        );
        let assigned = refuted("|- a & b, ~a", classical);
        assert_eq!(
            assigned.refutation(),
            &Refutation::Classical(Classical {
                assignment: vec![true, false]
            })
        );
        assert_eq!(
            assigned.to_string(),
            "the classical reading is false where a is true and every other atom false (! and ? \
             erased, ⊗ and & as and, ⅋ and ⊕ as or, 1 and ⊤ true, ⊥ and 0 false), and every \
             provable sequent reads as a tautology"
        );
    }

    /// A goal is a multiset. The roots in another order are the sequent
    /// itself, which the net engine takes, forced or by default.
    #[test]
    fn goal_in_any_order() {
        let s = sequent("a, a -o b |- b");
        let forest = Forest::new(&s).unwrap();
        let goal: Vec<Member> = forest
            .roots()
            .iter()
            .rev()
            .map(|&o| Member::from(o))
            .collect();
        assert_ne!(goal[0], Member::from(forest.roots()[0]));
        assert!(Goal::new(&forest, &goal).unwrap().is_conclusion());
        let classical = Mode::CLASSICAL;
        let outcome = prove_goal(
            Goal::new(&forest, &goal).unwrap(),
            classical,
            &Options::default(),
            &Limits::default(),
            |_| false,
        )
        .unwrap();
        assert_eq!(outcome.engine, Engine::Focus);
        assert!(outcome.verdict.proof().is_some());
        let net = Options::default().with_engine(Some(Engine::Net));
        let outcome = prove_goal(
            Goal::new(&forest, &goal).unwrap(),
            classical,
            &net,
            &Limits::default(),
            |_| false,
        )
        .unwrap();
        assert!(outcome.verdict.proof().is_some());
    }

    /// `Options::engine` forces an engine: the focused engine on MLL, and
    /// the net engine on MLL with Mix. The net engine forced outside MLL
    /// is an error.
    #[test]
    fn engine_override() {
        let s = sequent("|- a * b, ~a par ~b");
        let focus = Options::default().with_engine(Some(Engine::Focus));
        let outcome = prove(&s, Mode::CLASSICAL, &focus).unwrap();
        assert_eq!(outcome.engine, Engine::Focus);
        assert!(outcome.verdict.proof().is_some());
        assert!(outcome.net.is_none());
        let net = Options::default().with_engine(Some(Engine::Net));
        let outcome = prove(
            &sequent("|- a, ~a, b, ~b"),
            Mode::CLASSICAL.with_mix(),
            &net,
        )
        .unwrap();
        assert_eq!(outcome.engine, Engine::Net);
        assert!(outcome.verdict.proof().is_some());
        for (input, options) in [
            ("|- 1", net.clone()),
            ("|- a & b, ~a", net.clone()),
            ("|- a, ~a", net.clone().with_fragment(Some(Fragment::MALL))),
        ] {
            let error = prove(&sequent(input), Mode::CLASSICAL, &options).unwrap_err();
            assert!(
                matches!(
                    error,
                    Error::EngineRefused {
                        engine: Engine::Net,
                        because: NotTaken::Fragment { .. },
                        ..
                    }
                ),
                "{input:?}: {error}"
            );
        }
        assert_eq!(
            prove(&sequent("|- 1"), Mode::CLASSICAL, &net)
                .unwrap_err()
                .to_string(),
            "proof nets exist for MLL without units only, not for MLL with units"
        );
    }

    /// The asserted fragment must contain the sequent's. The search runs
    /// in it, and it picks the engine.
    #[test]
    fn fragment_override() {
        let s = sequent("|- a * b, ~a, ~b");
        let outcome = prove(
            &s,
            Mode::CLASSICAL,
            &Options::default().with_fragment(Some(Fragment::MALL)),
        )
        .unwrap();
        assert_eq!(outcome.fragment, Fragment::MALL);
        assert_eq!(outcome.engine, Engine::Focus);
        assert!(outcome.verdict.proof().is_some());
        let error = prove(
            &sequent("|- a & b, ~a"),
            Mode::CLASSICAL,
            &Options::default().with_fragment(Some(Fragment::MLL)),
        )
        .unwrap_err();
        assert!(matches!(
            error,
            Error::FragmentMismatch {
                asserted: Fragment::MLL,
                detected: Fragment::ADDITIVES,
            }
        ));
        assert_eq!(
            error.to_string(),
            "the sequent lies in ALL, outside MLL, the fragment asked for"
        );
    }

    /// A Horn program with a clause under `!` goes to the Horn engine in
    /// every mode, with Mix and in affine mode too. The other sequents with
    /// exponentials go to the focused engine. The net engine is refused in
    /// affine mode.
    #[test]
    fn dispatch_by_mode() {
        for (input, mode, fragment, engine) in [
            (
                "a, b |- a",
                Mode::CLASSICAL.with_affine(),
                Fragment::EMPTY,
                Engine::Focus,
            ),
            (
                "!a |- a",
                Mode::CLASSICAL,
                Fragment::EXPONENTIALS,
                Engine::Horn,
            ),
            (
                "!(a -o a * b), a |- a * b * b",
                Mode::CLASSICAL.with_mix(),
                Fragment::MLL | Fragment::EXPONENTIALS,
                Engine::Horn,
            ),
            (
                "a |- !a -o a",
                Mode::CLASSICAL,
                Fragment::MLL | Fragment::EXPONENTIALS,
                Engine::Focus,
            ),
            (
                "!a |- a & a",
                Mode::CLASSICAL.with_mix(),
                Fragment::ADDITIVES | Fragment::EXPONENTIALS,
                Engine::Focus,
            ),
            (
                "!a, b |- a",
                Mode::CLASSICAL.with_affine(),
                Fragment::EXPONENTIALS,
                Engine::Horn,
            ),
            (
                "!a, b |- a & a",
                Mode::CLASSICAL.with_affine(),
                Fragment::ADDITIVES | Fragment::EXPONENTIALS,
                Engine::Focus,
            ),
        ] {
            let outcome = prove(&sequent(input), mode, &Options::default()).unwrap();
            assert_eq!(outcome.engine, engine, "{input:?}");
            assert_eq!(outcome.fragment, fragment, "{input:?}");
            assert!(outcome.verdict.proof().is_some(), "{input:?}");
        }
        let net = Options::default().with_engine(Some(Engine::Net));
        let error = prove(&sequent("a, b |- a"), Mode::CLASSICAL.with_affine(), &net).unwrap_err();
        assert!(matches!(
            error,
            Error::EngineRefused {
                because: NotTaken::Mode { .. },
                ..
            }
        ));
        assert_eq!(
            error.to_string(),
            "proof nets exist in linear mode only, with or without Mix, not in classical affine mode"
        );
    }

    /// Intuitionistic mode: two formulas with an additive connective or
    /// unit go to the additive path, and a Horn program with a clause
    /// under `!` to the Horn engine. Everything else, unit-free IMLL
    /// included, goes to the two-sided engine. A sequent with no
    /// intuitionistic reading, Mix, and an engine forced for the other
    /// mode are errors.
    #[test]
    fn dispatch_intuitionistic() {
        let i = Mode::INTUITIONISTIC;
        for (input, fragment, engine) in [
            ("a, a -o b |- b", Fragment::MLL, Engine::TwoSided),
            ("1 |- 1", Fragment::MULTIPLICATIVE_UNITS, Engine::TwoSided),
            ("a & b |- a", Fragment::ADDITIVES, Engine::Additive),
            ("a & b, 0 |- a", Fragment::ADDITIVE, Engine::TwoSided),
            (
                "!a |- a * a",
                Fragment::MLL | Fragment::EXPONENTIALS,
                Engine::Horn,
            ),
            ("!a |- !a", Fragment::EXPONENTIALS, Engine::TwoSided),
        ] {
            let outcome = prove(&sequent(input), i, &Options::default()).unwrap();
            assert_eq!(outcome.fragment, fragment, "{input:?}");
            assert_eq!(outcome.engine, engine, "{input:?}");
            let proof = outcome
                .verdict
                .proof()
                .unwrap_or_else(|| panic!("{input:?}"));
            assert_eq!(proof.check(i), Ok(()), "{input:?}");
        }
        let outcome = prove(&sequent("a, b |- a"), i.with_affine(), &Options::default()).unwrap();
        assert_eq!(outcome.engine, Engine::TwoSided);
        assert!(outcome.verdict.proof().is_some());

        let error = prove(&sequent("|- a par b"), i, &Options::default()).unwrap_err();
        assert!(matches!(error, Error::NotIntuitionistic(_)));
        assert_eq!(
            error.to_string(),
            "not an intuitionistic sequent: the subformula a ⅋ b is neither an intuitionistic \
             formula nor the negation of one: intuitionistic linear logic has ⅋ only in A ⊸ B \
             (~A ⅋ B), and ? only under a negation"
        );
        let error = prove(&sequent("a |- a"), i.with_mix(), &Options::default()).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Mix has no intuitionistic form: one premise of a Mix would have nothing right of ⊢"
        );
        let focus = Options::default().with_engine(Some(Engine::Focus));
        let error = prove(&sequent("a |- a"), i, &focus).unwrap_err();
        assert_eq!(
            error.to_string(),
            "the focus engine does not search in intuitionistic mode"
        );
        let two_sided = Options::default().with_engine(Some(Engine::TwoSided));
        let error = prove(&sequent("a |- a"), Mode::CLASSICAL, &two_sided).unwrap_err();
        assert!(matches!(
            error,
            Error::EngineRefused {
                because: NotTaken::Mode { .. },
                ..
            }
        ));
        let outcome = prove(&sequent("a, a -o b |- b"), i, &two_sided).unwrap();
        assert_eq!(outcome.engine, Engine::TwoSided);
        assert!(outcome.verdict.proof().is_some());
        let additive = Options::default().with_engine(Some(Engine::Additive));
        let error = prove(&sequent("a & b, c |- a"), i, &additive).unwrap_err();
        assert!(matches!(
            error,
            Error::EngineRefused {
                engine: Engine::Additive,
                ..
            }
        ));
        assert_eq!(
            error.to_string(),
            "the additive engine decides a sequent of two additive-only formulas only"
        );
    }

    /// IMLL by embedding. On generated intuitionistic sequents over `⊗`
    /// and `⊸`, and their mutants, the net engine forced on the one-sided
    /// sequent gives the two-sided engine's verdict. Every net-engine proof
    /// passes the intuitionistic checker, since every sequent of a cut-free
    /// MLL proof of an intuitionistic sequent has one goal.
    #[test]
    fn embedding_agrees_with_the_two_sided_engine() {
        use crate::linear::search::generate::{self, IllRules, Rng};
        let i = Mode::INTUITIONISTIC;
        let rules = IllRules {
            units: false,
            additives: false,
            zero: false,
            exponentials: false,
        };
        let mut rng = Rng::new(7);
        let (mut compared, mut provable) = (0, 0);
        for _ in 0..200 {
            let budget = 2 + rng.below(8);
            let generate::Ill {
                mut hypotheses,
                mut goal,
                ..
            } = generate::ill(&mut rng, rules, 3, budget);
            for mutated in [false, true] {
                if mutated && !generate::mutate_two_sided(&mut rng, &mut hypotheses, &mut goal, 3) {
                    continue;
                }
                let text = generate::two_sided(&hypotheses, &goal);
                let s = sequent(&text);
                // No literal more than twice, so that a debug build decides
                // the sample quickly.
                let forest = Forest::new(&s).unwrap();
                if forest
                    .all_literals()
                    .chunk_by(|x, y| {
                        forest.atom(*x) == forest.atom(*y) && forest.sign(*x) == forest.sign(*y)
                    })
                    .any(|group| group.len() > 2)
                {
                    continue;
                }
                let net = Options::default().with_engine(Some(Engine::Net));
                let by_net = prove(&s, i, &net).unwrap();
                let two_sided = Options::default().with_engine(Some(Engine::TwoSided));
                let by_two_sided = prove(&s, i, &two_sided).unwrap();
                assert_eq!(by_two_sided.engine, Engine::TwoSided);
                let verdict = |outcome: Outcome| match outcome.verdict {
                    Verdict::Proved(proof) => {
                        assert_eq!(proof.check(i), Ok(()), "{text:?} by {}", outcome.engine);
                        assert_eq!(outcome.net.is_some(), outcome.engine == Engine::Net);
                        true
                    }
                    Verdict::Unprovable(_) => false,
                    Verdict::Unknown(reason) => panic!("{text:?} by {}: {reason}", outcome.engine),
                };
                let (net, focus) = (verdict(by_net), verdict(by_two_sided));
                assert_eq!(net, focus, "{text:?}: net {net}, two-sided {focus}");
                assert!(mutated || net, "{text:?} is provable");
                compared += 1;
                provable += usize::from(net);
            }
        }
        assert!(
            compared > 100 && provable > 50 && provable < compared,
            "{provable} of {compared}"
        );
    }

    /// A goal below the roots is decided in its own fragment: by the
    /// focused engine, two additive-only occurrences by the additive path,
    /// and in intuitionistic mode by the two-sided engine. The net engine
    /// is refused off the roots, and an intuitionistic goal must have one
    /// output.
    #[test]
    fn goals() {
        let o = |ids: &[u32]| ids.iter().map(|&i| Member::new(i)).collect::<Vec<_>>();
        // 0: ~a, 1: (a ⊗ ~b) ⊗ ?c, 2: a ⊗ ~b, 3: a, 4: ~b, 5: ?c, 6: c, 7: b.
        let s = sequent("|- ~a, (a * ~b) * ?c, b");
        let forest = Forest::new(&s).unwrap();
        let options = Options::default();
        let goal = o(&[0, 3]);
        let outcome = prove_goal(
            Goal::new(&forest, &goal).unwrap(),
            Mode::CLASSICAL,
            &options,
            &Limits::default(),
            |_| false,
        )
        .unwrap();
        assert_eq!(outcome.engine, Engine::Focus);
        assert_eq!(outcome.fragment, Fragment::EMPTY);
        let proof = outcome.verdict.proof().unwrap();
        // A goal proof is checked against the goal it records.
        assert!(proof.goal().is_some());
        assert_eq!(proof.check(Mode::CLASSICAL), Ok(()));
        let outcome = prove_goal(
            Goal::new(&forest, &o(&[0, 2, 7])).unwrap(),
            Mode::CLASSICAL,
            &options,
            &Limits::default(),
            |_| false,
        )
        .unwrap();
        assert!(outcome.verdict.proof().is_some());
        assert_eq!(outcome.fragment, Fragment::MLL);
        let outcome = prove_goal(
            Goal::new(&forest, &o(&[4, 5])).unwrap(),
            Mode::CLASSICAL,
            &options,
            &Limits::default(),
            |_| false,
        )
        .unwrap();
        assert!(
            matches!(outcome.verdict, Verdict::Unprovable(_)),
            "{:?}",
            outcome.verdict
        );
        assert_eq!(outcome.fragment, Fragment::EXPONENTIALS);
        assert!(!Goal::new(&forest, &goal).unwrap().is_conclusion());
        let net = Options::default().with_engine(Some(Engine::Net));
        let error = prove_goal(
            Goal::new(&forest, &goal).unwrap(),
            Mode::CLASSICAL,
            &net,
            &Limits::default(),
            |_| false,
        )
        .unwrap_err();
        assert!(
            matches!(
                error,
                Error::EngineRefused {
                    because: NotTaken::Goal,
                    ..
                }
            ),
            "{error}"
        );
        let error = Goal::new(&forest, &o(&[9])).unwrap_err();
        assert!(
            matches!(
                error,
                Error::IndexOutOfBounds {
                    space: crate::limits::Space::Occurrence,
                    index: 9,
                    len: 8
                }
            ),
            "{error}"
        );

        // 0: (~a & ~b) ⅋ (a & b), 1: ~a & ~b, 2: ~a, 3: ~b, 4: a & b, 5: a,
        // 6: b. The additive path decides the pair below the ⅋.
        let s = sequent("|- (~a & ~b) par (a & b)");
        let forest = Forest::new(&s).unwrap();
        let outcome = prove_goal(
            Goal::new(&forest, &o(&[1, 4])).unwrap(),
            Mode::CLASSICAL,
            &options,
            &Limits::default(),
            |_| false,
        )
        .unwrap();
        assert_eq!(outcome.engine, Engine::Additive);
        assert!(matches!(outcome.verdict, Verdict::Unprovable(_)));
        let outcome = prove_goal(
            Goal::new(&forest, &o(&[5, 2])).unwrap(),
            Mode::CLASSICAL,
            &options,
            &Limits::default(),
            |_| false,
        )
        .unwrap();
        assert_eq!(outcome.engine, Engine::Focus);
        assert!(outcome.verdict.proof().is_some());

        // 0: ~a, 1: a ⊗ ~b, 2: a, 3: ~b, 4: b, read as a, a ⊸ b ⊢ b. The
        // goal ⊢ ~a, a is a ⊢ a two-sided. ⊢ ~a alone has no output, and
        // ⊢ a, b has two.
        let s = sequent("a, a -o b |- b");
        let forest = Forest::new(&s).unwrap();
        let i = Mode::INTUITIONISTIC;
        let outcome = prove_goal(
            Goal::new(&forest, &o(&[0, 2])).unwrap(),
            i,
            &options,
            &Limits::default(),
            |_| false,
        )
        .unwrap();
        assert_eq!(outcome.engine, Engine::TwoSided);
        assert!(outcome.verdict.proof().is_some());
        for (goal, outputs) in [(o(&[0]), 0), (o(&[2, 4]), 2)] {
            let error = prove_goal(
                Goal::new(&forest, &goal).unwrap(),
                i,
                &options,
                &Limits::default(),
                |_| false,
            )
            .unwrap_err();
            assert!(
                matches!(error, Error::GoalOutputs { count: n } if n == outputs),
                "{error}"
            );
        }
    }

    /// A proof of a goal records the goal and the mode it was found in,
    /// passes the checker against the goal, and is refused where a proof
    /// of the sequent is needed.
    #[test]
    fn a_goal_proof_records_its_goal() {
        // 0: a ⊗ b, 1: a, 2: b, 3: ~a, 4: ~b, 5: c, 6: ~c
        let sequent: Sequent = "|- a * b, ~a, ~b, c, ~c".parse().unwrap();
        let forest = Forest::new(&sequent).unwrap();
        let goal = [Member::new(6), Member::new(5)];
        let mode = Mode::CLASSICAL;
        let outcome = prove_goal(
            Goal::new(&forest, &goal).unwrap(),
            mode,
            &Options::default(),
            &Limits::default(),
            |_| false,
        );
        let Verdict::Proved(proof) = outcome.unwrap().verdict else {
            panic!("⊢ ~c, c is provable");
        };
        let members = [crate::linear::Member::new(6), crate::linear::Member::new(5)];
        assert_eq!(
            (proof.goal(), proof.mode()),
            (Some(&members[..]), Some(mode))
        );
        assert_eq!(proof.check(mode), Ok(()));
        assert!(matches!(
            ProofStructure::from_proof(
                &proof,
                crate::linear::Criterion::MLL,
                &Limits::default(),
                |_| { false }
            ),
            Err(Error::GoalProof)
        ));
        #[cfg(feature = "rocq")]
        {
            let derivation = proof.derivation().unwrap();
            let options = crate::export::rocq::Options::default();
            let written =
                crate::export::rocq::write(&derivation, &options, &mut String::new(), |_| false);
            assert!(matches!(written, Err(Error::GoalProof)));
        }
    }

    /// A thread count is taken as at least one and at most
    /// [`Options::MAX_JOBS`], whatever is asked for.
    #[test]
    fn jobs_are_bounded() {
        assert_eq!(Jobs::Count(usize::MAX).count(), Options::MAX_JOBS);
        assert_eq!(Jobs::Count(0).count(), 1);
        assert!((1..=Options::MAX_JOBS).contains(&Jobs::Auto.count()));
        #[cfg(feature = "parallel")]
        assert_eq!(Options::default().with_jobs(2).threads(), 2);
    }
}
