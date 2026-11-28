// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use super::super::SetUp;
use super::super::zone::Zone;
use super::{BACKWARD_SHARE, Plan};
use crate::linear::search::counts::Counts;
use crate::linear::search::{Finished, Reason, Stop};
use crate::memory::Account;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Condvar, Mutex};

/// The work a search does before it gives way to the other, when two
/// alternate on one core. It is about a thousand stable sequents of a
/// small problem, against which a change of threads costs little.
const SLICE: u64 = 1 << 16;

/// How long the calling thread waits for a search that runs on alone
/// before it polls the caller's stop condition again.
const POLL: std::time::Duration = std::time::Duration::from_millis(1);

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

/// Stops both searches when the calling thread unwinds, so that a panic
/// of the caller's stop condition does not leave the second search
/// running.
struct StopOnPanic<'a>(&'a Baton);

impl Drop for StopOnPanic<'_> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            self.0.stop();
        }
    }
}

/// Marks a search as ended when its thread leaves it, by a return or by a
/// panic, so that the other never waits for it.
struct Ended<'a>(&'a Baton, usize);

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

/// Decides a goal by two searches that alternate on one core. Returns
/// what [`search_goal`](super::super::search_goal) does, with the counters of
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
pub(in crate::linear::search::focus) fn alternate<Z: Zone>(
    set_up: SetUp<'_>,
    [
        (first, first_counts, first_account),
        (second, second_counts, second_account),
    ]: [(Plan, &Counts, &Account); 2],
    work: &crate::linear::search::Work,
    stop: &mut dyn FnMut(u64) -> bool,
) -> Option<Finished> {
    let limits = set_up.limits;
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
    Some(merged(forward, backward))
}

/// Returns the answer of two searches of one goal that ran together, from
/// what each returned.
///
/// The answer is a verdict of either. Without one, it is the stop, which
/// is then the caller's; else the second search's reason. The counters
/// are both searches' together.
pub(in crate::linear::search::focus) fn merged(first: Finished, second: Finished) -> Finished {
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
        (Err(_), Err(reason)) => (Err(reason), Vec::new(), None),
    };
    Finished {
        result,
        nodes,
        statistics,
        instances,
    }
}
