// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The two searches of the default bias, and how they share the cores.
//!
//! The bias says which literal of each atom the engine treats as
//! positive. It changes the speed of the search, never what it proves. On
//! a goal with exponentials the default bias runs two searches: the
//! *forward* one under the factor rule, and the *backward* one under the
//! rarer-literal rule ([`Bias`](crate::linear::search::Bias)). The first
//! to decide answers.
//!
//! This module has:
//! - which searches decide a goal ([`plan`]);
//! - one search on its own ([`Plan::search`](self::Plan::search));
//! - two ways for the searches to share one core: in turns, each from
//!   its start, without threads ([`turns`]), or alternating in slices
//!   on two threads
//!   ([`alternate`](crate::linear::search::focus::schedule::alternate)).

use super::arena::{Arena, Kept};
use super::counts::Counts;
use super::memo::{Interner, Memo, Table};
use super::zone::Zone;
use super::{Problem, Run, SetUp};
#[cfg(feature = "parallel")]
use crate::limits::Limits;
use crate::linear::Instances;
use crate::linear::fragment::{Fragment, Mode};
use crate::linear::occurrences::{Forest, OccId};
use crate::linear::proofs::{Node, NodeId};
use crate::linear::search::Bias;
use crate::linear::search::memory::Account;
use crate::linear::search::{Finished, Options, Reason, Statistics, Stop};
use crate::linear::sequents::Kind;
#[cfg(feature = "parallel")]
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
#[cfg(feature = "parallel")]
use std::sync::{Condvar, Mutex};

/// Decides a goal by two searches that take turns on one thread, until
/// one decides.
///
/// Each turn starts its search from the beginning, with a memo (the table
/// of sequents decided) and an arena of its own. The work a turn gets
/// grows from round to round.
/// A search that ended without deciding takes no further turn, and the
/// other then runs to its own end. The counters are those of every turn
/// together; the memo's entries are the most of one turn.
pub(super) fn turns<Z: Zone>(
    set_up: SetUp<'_>,
    searches: [(Plan, &Counts, &Account); 2],
    stop: &mut dyn FnMut(u64) -> bool,
) -> Finished {
    let (options, limits) = (set_up.options, set_up.limits);
    let mut ended: [Option<Reason>; 2] = [None, None];
    let mut statistics = Statistics::default();
    // The forward search's level and the backward one's, apart.
    let mut levels = [0, 0];
    // The level reported under the copy bound is the deciding search's,
    // else the backward one's.
    let leveled = |mut statistics: Statistics, levels: [u32; 2], decider: usize| {
        statistics.copies = levels[decider];
        statistics.forward_copies = levels[0];
        statistics
    };
    let mut turn = FIRST_TURN;
    loop {
        for (i, (plan, counts, account)) in searches.into_iter().enumerate() {
            if ended[i].is_some() {
                continue;
            }
            let stop = if ended[1 - i].is_some() {
                Stop::Closure(&mut *stop)
            } else if i == 0 {
                Stop::Turn(&mut *stop, turn)
            } else {
                Stop::Turn(&mut *stop, turn.saturating_mul(BACKWARD_SHARE))
            };
            let (
                Finished {
                    result,
                    nodes,
                    statistics: run,
                    instances,
                },
                over,
            ) = plan.search::<Z>(
                set_up,
                counts,
                // A turn's memo and arena go when it ends.
                &account.fork(),
                stop,
            );
            statistics.add(&run);
            levels[i] = levels[i].max(run.copies);
            match result {
                Err(Reason::Stopped) if over => {}
                Err(Reason::Stopped) => {
                    return Finished {
                        result: Err(Reason::Stopped),
                        nodes,
                        statistics: leveled(statistics, levels, 1),
                        instances: None,
                    };
                }
                Err(reason) => ended[i] = Some(reason),
                decided => {
                    return Finished {
                        result: decided,
                        nodes,
                        statistics: leveled(statistics, levels, i),
                        instances,
                    };
                }
            }
        }
        if let [Some(_), Some(backward)] = ended {
            return Finished {
                result: Err(backward.as_set(options, limits)),
                nodes: Vec::new(),
                statistics: leveled(statistics, levels, 1),
                instances: None,
            };
        }
        turn = turn.saturating_mul(TURN_GROWTH);
    }
}

/// The work the forward search of the default bias gets in the first
/// round of turns; the backward one gets [`BACKWARD_SHARE`] times as
/// much. That is some thousand stable sequents of a small problem.
///
/// Work is counted in steps of a split search. A stable sequent counts
/// [`NODE_WORK`](super::NODE_WORK) of them, and more with its size.
const FIRST_TURN: u64 = 1 << 16;

/// How much work the backward search of the default bias gets for each
/// unit of the forward one.
///
/// So under a time limit the default decides everything the backward
/// search decides alone in two thirds of the limit, and what the forward
/// search decides in a third.
const BACKWARD_SHARE: u64 = 2;

/// The factor by which the turns of the two searches grow from round to
/// round. A search starts afresh in every turn, so with turns that grow
/// geometrically the turns that ended early cost a fraction of the one
/// that decides.
const TURN_GROWTH: u64 = 4;

/// One search of a goal: the bias it runs under and its copy bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Plan {
    /// The bias.
    pub(super) bias: Bias,
    /// The most copies a branch may take; `u32::MAX` is no bound.
    pub(super) copies: u32,
}

impl Plan {
    /// Runs the search of this plan alone, with a memo and an arena of its
    /// own, until it ends or the stop condition fires.
    ///
    /// Returns what [`search_goal`](super::search_goal) does, and whether
    /// the search was a turn that ran out of its work.
    pub(super) fn search<'a, Z: Zone>(
        self,
        set_up: SetUp<'a>,
        counts: &'a Counts,
        account: &'a Account,
        stop: Stop<'a>,
    ) -> (Finished, bool) {
        let problem = Problem::new(set_up, counts, self.copies, account);
        // A first-order search keys its sequents by instance. So its table
        // of zones keeps none, and gives every zone a new id without
        // reading it, which the copies' order by class needs.
        let memo = if Z::FIRST_ORDER {
            Table::Own(Memo::new(set_up.options.memo_entries()), Interner::new(0))
        } else {
            Table::own(set_up.options.memo_entries())
        };
        let mut engine = Run::new(
            problem,
            stop,
            memo,
            Arena::new(Kept::Own(Vec::new()), account),
            Z::new(set_up.task.forest, set_up.options),
        );
        let mut result = engine.run_kept(set_up.task.goal);
        let statistics = engine.statistics();
        let over = matches!(engine.stop, Stop::Turn(_, 0));
        let mut nodes = match engine.nodes.kept {
            Kept::Own(nodes) => nodes,
            Kept::Shared(_) => unreachable!("a sequential search owns its arena"),
        };
        let instances = closed(&engine.zone, set_up.task.forest, &mut result, &mut nodes);
        (
            Finished {
                result,
                nodes,
                statistics,
                instances,
            },
            over,
        )
    }
}

/// Returns the instance table of a first-order search's proof, written by
/// its zone over the arena the proof lies in. Returns `None` for a
/// propositional search or where there is no proof.
///
/// Where the table cannot be written, the reason becomes the search's
/// result.
pub(super) fn closed<Z: Zone>(
    zone: &Z,
    forest: &Forest,
    result: &mut Result<Option<NodeId>, Reason>,
    nodes: &mut [Node],
) -> Option<Box<Instances>> {
    if !Z::FIRST_ORDER || !matches!(result, Ok(Some(_))) {
        return None;
    }
    match zone.close(forest, nodes) {
        Ok(table) => Some(Box::new(table)),
        Err(reason) => {
            *result = Err(reason);
            None
        }
    }
}

/// Returns the searches that decide a goal under the options.
///
/// One search, under the bias the options name, in most cases. There
/// `Auto` is the factor rule without an exponential in the sequent. With
/// one, or under weakening, it is the rarer-literal rule: under weakening
/// nothing forces a split, so the factors have nothing to say.
///
/// Two searches for `Auto` on a goal with exponentials in linear mode:
/// the forward search first, under the factor rule, and the backward
/// search, under the rarer-literal rule within [`Options::copies`], which
/// is `Bias::Rarer`'s search under the same options. They share nothing,
/// and the first to decide answers.
///
/// The forward search runs within `Options::copies` too. Where the goal
/// is a Horn program ([`chains`]), it runs within the larger of that and
/// [`Options::forward_copies`]: there a copy is a step of a forward
/// chain, and one branch takes as many copies as the chain is long. This
/// does not apply under Mix, where every sequent of a chain that grows is
/// tried in every partition.
///
/// When the two rules give every atom the same literal, the forward
/// search is the backward one continued, and runs alone.
pub(crate) fn plan(
    forest: &Forest,
    goal: &[OccId],
    fragment: Fragment,
    mode: Mode,
    options: &Options,
) -> (Plan, Option<Plan>) {
    let copies = options.copy_bound();
    let both = options.bias == Bias::Auto
        && !mode.affine
        && fragment.has_exponentials()
        && forest.sequent().fragment().has_exponentials();
    if !both {
        let bias = if options.bias == Bias::Auto && mode.affine {
            Bias::Rarer
        } else {
            options.bias
        };
        return (Plan { bias, copies }, None);
    }
    let forward = Plan {
        bias: Bias::Factors,
        copies: if !mode.mix && chains(forest, goal) {
            copies.max(options.forward_copies)
        } else {
            copies
        },
    };
    let backward = Plan {
        bias: Bias::Rarer,
        copies,
    };
    let same = super::bias::signs(forest, Bias::Factors) == super::bias::signs(forest, Bias::Rarer);
    (forward, (!same).then_some(backward))
}

/// Whether a goal is a Horn program.
///
/// In a Horn program every member under a `?` is a clause. Every other
/// member is a marking, a goal, or a clause used once, as a clause is in
/// the linear zone that an interactive dereliction leaves. With the atoms
/// of the bodies written `a` (or all of them `~a`):
/// - a clause is a tensor of body literals of which at most one factor is
///   a head instead: a literal of the other sign, or a `⅋` of such. That
///   is what `!(a ⊗ b ⊸ c ⊗ d)` is on the right of `⊢`;
/// - a marking is a `⅋` of head literals;
/// - a goal is a tensor of body literals;
/// - `1` stands for an empty body or goal, and `⊥` for an empty head.
///
/// A Petri net with a marking to reach is such a program. There a copy of
/// a clause rewrites the linear zone, so a search within `n` copies
/// visits the markings within `n` steps and nothing else. That is what
/// makes a copy bound of its own affordable.
pub(super) fn chains(forest: &Forest, goal: &[OccId]) -> bool {
    use crate::linear::occurrences::Sign;
    use crate::linear::search::horn::{clause_head, is_head};
    [Sign::Atom, Sign::Dual].into_iter().any(|body| {
        let clause = |o: OccId| clause_head(forest, body, o).is_some();
        goal.iter().all(|&member| {
            if forest.kind(member) == Kind::Quest {
                clause(forest.left(member).unwrap())
            } else {
                is_head(forest, body, member) || clause(member)
            }
        })
    })
}

#[cfg(feature = "parallel")]
/// The work a search does before it gives way to the other, when two
/// alternate on one core. It is about a thousand stable sequents of a
/// small problem, against which a change of threads costs little.
const SLICE: u64 = 1 << 16;

#[cfg(feature = "parallel")]
/// How long the calling thread waits for a search that runs on alone
/// before it polls the caller's stop condition again.
const POLL: std::time::Duration = std::time::Duration::from_millis(1);

#[cfg(feature = "parallel")]
/// Which of two alternating searches may run, and what both must know of
/// each other.
#[derive(Default)]
struct Baton {
    /// The state.
    state: Mutex<Turns>,
    /// Signalled whenever the state changes.
    changed: Condvar,
    /// The polls of the second search that the caller's stop condition
    /// has not been told of yet. The condition lives on the calling
    /// thread, which polls it once for each of them.
    polls: AtomicU64,
    /// Whether both must stop. The second search reads it at every poll,
    /// without the lock.
    halt: AtomicBool,
}

#[cfg(feature = "parallel")]
/// The state two alternating searches share.
#[derive(Default)]
struct Turns {
    /// The search that may run.
    holder: usize,
    /// Whether each search has ended.
    ended: [bool; 2],
    /// Whether both must stop, because one decided, the caller's stop
    /// condition fired, or a thread panicked.
    stop: bool,
}

#[cfg(feature = "parallel")]
impl Baton {
    /// Locks the state. It is plain data, so a panic of the other thread
    /// leaves it usable.
    fn lock(&self) -> std::sync::MutexGuard<'_, Turns> {
        crate::linear::search::lock(&self.state)
    }

    /// Waits until search `me` may run, and returns whether it must stop
    /// instead.
    fn wait(&self, me: usize) -> bool {
        let mut turns = self.lock();
        while turns.holder != me && !turns.stop {
            turns = self
                .changed
                .wait(turns)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
        turns.stop
    }

    /// Gives way to the other search, unless it has ended, and waits for
    /// the turn of search `me` to come again. Returns whether it must stop.
    fn pass(&self, me: usize) -> bool {
        {
            let mut turns = self.lock();
            if turns.stop || turns.ended[1 - me] {
                return turns.stop;
            }
            turns.holder = 1 - me;
        }
        self.changed.notify_all();
        self.wait(me)
    }

    /// Gives way to the second search from the first, which runs on the
    /// calling thread, and waits for the first's turn to come again,
    /// unless the second has ended. Returns whether both must stop.
    ///
    /// While it waits, it wakes once every [`POLL`] and polls the caller's
    /// stop condition for the polls the second search made meanwhile. A
    /// slice of the second search may take seconds, and the condition
    /// must not wait that long.
    fn pass_polling(&self, stop: &mut dyn FnMut(u64) -> bool) -> bool {
        {
            let mut turns = self.lock();
            if turns.stop || turns.ended[1] {
                return turns.stop;
            }
            turns.holder = 1;
        }
        self.changed.notify_all();
        loop {
            {
                let turns = self.lock();
                if turns.stop {
                    return true;
                }
                if turns.holder == 0 {
                    break;
                }
                let waited = self.changed.wait_timeout(turns, POLL);
                drop(waited.unwrap_or_else(|poisoned| poisoned.into_inner()));
            }
            // The caller's condition runs without the lock.
            if self.caught_up(stop) {
                self.stop();
                return true;
            }
        }
        self.caught_up(stop)
    }

    /// Tells both searches to stop.
    fn stop(&self) {
        self.halt.store(true, Ordering::Relaxed);
        self.lock().stop = true;
        self.changed.notify_all();
    }

    /// Polls the caller's stop condition once for every poll the second
    /// search made since this was last done. Returns whether it fired.
    ///
    /// So a condition that counts its polls, or reads a clock every so
    /// many, sees the two searches as it sees one.
    fn caught_up(&self, stop: &mut dyn FnMut(u64) -> bool) -> bool {
        // The other search added its work at its polls.
        (0..self.polls.swap(0, Ordering::Relaxed)).any(|_| stop(0))
    }
}

#[cfg(feature = "parallel")]
/// Stops both searches when the calling thread unwinds, so that a panic
/// of the caller's stop condition does not leave the second search
/// running.
struct StopOnPanic<'a>(&'a Baton);

#[cfg(feature = "parallel")]
impl Drop for StopOnPanic<'_> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            self.0.stop();
        }
    }
}

#[cfg(feature = "parallel")]
/// Marks a search as ended when its thread leaves it, by a return or by a
/// panic, so that the other never waits for it.
struct Ended<'a>(&'a Baton, usize);

#[cfg(feature = "parallel")]
impl Drop for Ended<'_> {
    fn drop(&mut self) {
        let Ended(baton, me) = *self;
        {
            let mut turns = baton.lock();
            turns.ended[me] = true;
            turns.holder = 1 - me;
            turns.stop |= std::thread::panicking();
            if turns.stop {
                baton.halt.store(true, Ordering::Relaxed);
            }
        }
        baton.changed.notify_all();
    }
}

#[cfg(feature = "parallel")]
/// Decides a goal by two searches that alternate on one core. Returns
/// what [`search_goal`](super::search_goal) does, with the counters of
/// both searches together, or `None` when the thread cannot start.
///
/// The first search runs on the calling thread and the second on a
/// thread of its own. One of them runs at a time, for a slice of work:
/// [`SLICE`] for the first, [`BACKWARD_SHARE`] slices for the second. So
/// each is the search it would be alone, none starts again, and the run
/// is a function of the input.
///
/// The first to decide stops the other at the end of its slice. A search
/// that ended without deciding leaves the other to run on.
///
/// The first search polls the caller's stop condition at its own polls.
/// The condition is also polled once for every poll of the second search,
/// within a millisecond of it: the calling thread wakes that often while
/// the second search has its turn or runs on alone.
pub(super) fn alternate<Z: Zone>(
    set_up: SetUp<'_>,
    [
        (first, first_counts, first_account),
        (second, second_counts, second_account),
    ]: [(Plan, &Counts, &Account); 2],
    work: &crate::linear::search::Work,
    stop: &mut dyn FnMut(u64) -> bool,
) -> Option<Finished> {
    let (options, limits) = (set_up.options, set_up.limits);
    let baton = Baton::default();
    let (forward, backward) = std::thread::scope(|scope| {
        let baton = &baton;
        let thread = std::thread::Builder::new()
            .name("deduxit-search".to_owned())
            .stack_size(limits.stack_bytes())
            .spawn_scoped(scope, move || {
                let _ended = Ended(baton, 1);
                if baton.wait(1) {
                    return Finished::gave_up(Reason::Stopped);
                }
                // The caller's stop lives on the calling thread, which asks
                // it for this search's polls. The work is counted here.
                let mut counted = crate::linear::search::Counted::new(work);
                let mut give_way = |units: u64, passed: bool| {
                    counted.add(units);
                    baton.polls.fetch_add(1, Ordering::Relaxed);
                    counted.passed()
                        || baton.halt.load(Ordering::Relaxed)
                        || (passed && baton.pass(1))
                };
                let slice = SLICE * BACKWARD_SHARE;
                let (finished, _) = second.search::<Z>(
                    set_up,
                    second_counts,
                    second_account,
                    Stop::Slice(&mut give_way, slice, slice),
                );
                drop(counted);
                if finished.result.is_ok() {
                    baton.stop();
                }
                finished
            })
            .ok()?;
        let _stop = StopOnPanic(baton);
        let forward = {
            let _ended = Ended(baton, 0);
            let mut give_way =
                |units: u64, passed: bool| stop(units) || (passed && baton.pass_polling(stop));
            let (finished, _) = first.search::<Z>(
                set_up,
                first_counts,
                first_account,
                Stop::Slice(&mut give_way, SLICE, SLICE),
            );
            if !matches!(finished.result, Err(reason) if reason != Reason::Stopped) {
                baton.stop();
            }
            finished
        };
        // The second search runs on alone, or is about to end.
        loop {
            let turns = baton.lock();
            if turns.ended[1] {
                break;
            }
            let waited = baton.changed.wait_timeout(turns, POLL);
            drop(waited.unwrap_or_else(|poisoned| poisoned.into_inner()));
            if baton.caught_up(stop) {
                baton.stop();
            }
        }
        let backward = thread
            .join()
            .unwrap_or_else(|panic| std::panic::resume_unwind(panic));
        Some((forward, backward))
    })?;
    Some(merged(forward, backward, options, limits))
}

#[cfg(feature = "parallel")]
/// Returns the answer of two searches of one goal that ran together, from
/// what each returned.
///
/// The answer is a verdict of either. Without one, it is the stop, which
/// is then the caller's; else the second search's reason. The counters
/// are both searches' together.
pub(super) fn merged(
    first: Finished,
    second: Finished,
    options: &Options,
    limits: &Limits,
) -> Finished {
    let mut statistics = first.statistics;
    // The two memos were held at once.
    statistics.add_run(&second.statistics);
    statistics.memo_hits += second.statistics.memo_hits;
    statistics.memo_entries += second.statistics.memo_entries;
    // The first is the forward search, the second the backward one. The
    // level under the copy bound is the deciding search's, else the
    // backward one's.
    statistics.forward_copies = first.statistics.copies;
    statistics.copies = if first.result.is_ok() {
        first.statistics.copies
    } else {
        second.statistics.copies
    };
    debug_assert!(
        !matches!((&first.result, &second.result), (Ok(a), Ok(b)) if a.is_some() != b.is_some()),
        "the two searches contradict each other"
    );
    let (result, nodes, instances) = match (first.result, second.result) {
        (Ok(root), _) => (Ok(root), first.nodes, first.instances),
        (_, Ok(root)) => (Ok(root), second.nodes, second.instances),
        (Err(Reason::Stopped), _) | (_, Err(Reason::Stopped)) => {
            (Err(Reason::Stopped), Vec::new(), None)
        }
        (Err(_), Err(reason)) => (Err(reason.as_set(options, limits)), Vec::new(), None),
    };
    Finished {
        result,
        nodes,
        statistics,
        instances,
    }
}
