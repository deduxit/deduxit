// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! How much work a search has done, and the bound on it. Every thread
//! of a search adds to one count. Also the helpers that take what the
//! threads share.

#[cfg(doc)]
use crate::limits::Limits;

/// The work a search has done, in its engine's units, and the bound on
/// it ([`Limits::work`]). Every thread of the search shares it.
///
/// Each thread counts its own units through a [`Counted`]. With
/// `parallel`, that adds them in batches of `BATCH`; without, all at
/// once when the thread ends. An atomic addition at every poll cost the
/// additive path 5 % of its instructions, and threads on a pool would
/// contend for it.
#[derive(Debug)]
pub(crate) struct Work {
    /// The units done.
    done: std::sync::atomic::AtomicU64,
    /// The most units allowed. `u64::MAX` means no bound: no search
    /// reaches it, since a unit takes at least a nanosecond.
    limit: u64,
}

impl Work {
    /// How many units a thread counts before it adds them. So the total,
    /// and the check against the bound, lag by at most this much per
    /// thread.
    #[cfg(feature = "parallel")]
    pub(crate) const BATCH: u64 = 1 << 12;

    /// No work done yet, under the bound `limit`.
    pub(crate) const fn new(limit: Option<u64>) -> Self {
        Self {
            done: std::sync::atomic::AtomicU64::new(0),
            limit: match limit {
                Some(limit) => limit,
                None => u64::MAX,
            },
        }
    }

    /// Adds `units` and returns the units done since the search began.
    pub(crate) fn add(&self, units: u64) -> u64 {
        self.done
            .fetch_add(units, std::sync::atomic::Ordering::Relaxed)
            .saturating_add(units)
    }

    /// Returns the units done since the search began.
    pub(crate) fn done(&self) -> u64 {
        self.done.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Returns the most units allowed.
    pub(crate) const fn limit(&self) -> u64 {
        self.limit
    }

    /// Returns whether the work passed its bound.
    pub(crate) fn passed(&self) -> bool {
        self.done() > self.limit
    }
}

/// Locks what a search's threads share, and takes a poisoned lock as it
/// is. A panic in one thread cannot leave the value half changed, since
/// each change is one assignment or one push.
pub(crate) fn lock<T>(shared: &std::sync::Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    shared
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Returns what the threads shared, once they have ended, poisoned or
/// not, as [`lock`] does.
#[cfg(feature = "parallel")]
pub(crate) fn taken<T>(shared: std::sync::Mutex<T>) -> T {
    shared
        .into_inner()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// One thread's count of the units it has not yet added to the search's
/// [`Work`]. Every thread of a search counts through one.
///
/// With `parallel`, where other threads read the total, it adds its
/// units in batches of [`Work::BATCH`]. It adds the rest when it drops.
pub(crate) struct Counted<'a> {
    /// The search's work.
    work: &'a Work,
    /// The units not yet added to it.
    pending: u64,
}

impl<'a> Counted<'a> {
    /// No units counted yet towards `work`.
    pub(crate) const fn new(work: &'a Work) -> Self {
        Self { work, pending: 0 }
    }

    /// Counts `units` and returns the search's units in all, this
    /// thread's not yet added included.
    pub(crate) fn add(&mut self, units: u64) -> u64 {
        self.pending += units;
        #[cfg(feature = "parallel")]
        if self.pending >= Work::BATCH {
            self.work.add(self.pending);
            self.pending = 0;
        }
        self.done()
    }

    /// Returns the search's units in all, this thread's not yet added
    /// included.
    pub(crate) fn done(&self) -> u64 {
        self.work.done().saturating_add(self.pending)
    }

    /// Returns whether the search's work, this thread's included, passed
    /// its bound.
    pub(crate) fn passed(&self) -> bool {
        self.done() > self.work.limit
    }
}

impl Drop for Counted<'_> {
    /// Adds the units not yet added to the search's count.
    fn drop(&mut self) {
        self.work.add(self.pending);
    }
}
