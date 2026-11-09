// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The log of the choices a first-order search makes while a rule of two
//! premises may ask its first premise for another answer. An answer is
//! the bindings the first premise's proof made to the variables the two
//! premises share. With the log, the next answer is found by resuming the
//! first premise's search where it stopped, without searching it again
//! from the start.
//!
//! **The log.** While such a rule is open, every choice point appends an
//! entry when it is entered. The choice points are the alternatives of a
//! stable sequent, the identities tried, the splits of a context, the
//! rule's own rounds and the parts a Mix tried. An entry holds one of:
//! - the alternative the choice point is trying, with the cuts of the
//!   alternatives it tried before;
//! - that it failed, with the cuts of all of them;
//! - the value it settled on, where it had no alternatives.
//!
//! A cut records that a search was cut short: by the copy budget (the
//! copies a branch may still make) or another bound, or by a loop-check
//! prune, which rests on an ancestor. Trying an alternative drops what
//! the one before it appended. So when the first premise is proved, the
//! log lists, in the order they were entered, the choice points of its
//! proof's search and the failed searches that search went on past.
//!
//! **Resuming.** The first premise's next answer is the one chronological
//! backtracking finds. Its last choice point that took an alternative
//! goes on with the next one, and every choice point before it keeps the
//! one it took. The search is replayed from the bindings it started from:
//! - each choice point takes its logged alternative at once (the ones
//!   before it failed under the same bindings, and their cuts are
//!   restored from the log);
//! - a failed one fails at once with its cuts;
//! - a settled one answers its value;
//! - the last open one goes on past its alternative.
//!
//! From there the search runs as usual, with the alternatives of every
//! choice point still open. The alternatives after the last one, and
//! every failure logged after it, are given up with the answer. Their
//! cuts are added to the last one's: an answer they hid, which more
//! budget would find, is one more answer the failure must not deny.
//!
//! **Divergence.** A replay is the same search only while what it reads
//! is what the first search read. The undo restores the bindings and the
//! zone, but a memo of failures may have learnt or forgotten entries
//! meanwhile. A replay has diverged when it meets another choice point
//! than the log's, fails before the last one, or ends before it. Every
//! choice point then fails at once, cut, and the rule searches its first
//! premise again with the failed answers excluded.

use super::Cuts;

/// The kind of choice point an entry is, which a replay checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Site {
    /// The alternatives of a stable sequent or of a `⊕` in focus.
    Choose,
    /// The identities tried on a stable sequent or a literal in focus.
    Initial,
    /// The splits of a context.
    Splits,
    /// The rounds of a rule that asks its first premise for answers.
    Answers,
    /// Whether every part of a Mix with a member less fails.
    Parts,
}

/// What a choice point did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Taken {
    /// It is trying this alternative.
    Open(u32),
    /// It failed: every alternative did.
    Failed,
    /// It answered this value, with no alternative to resume.
    Settled(u32),
}

/// An entry of the log.
#[derive(Clone, Copy, Debug)]
struct Entry {
    /// The choice point's kind.
    site: Site,
    /// What it did.
    taken: Taken,
    /// The cuts of the alternatives it tried before, or of all of them.
    cuts: Cuts,
}

/// A choice point entered: its entry in the log, if the log is kept, and
/// where it goes on.
#[derive(Clone, Copy, Debug)]
pub(super) struct Choice {
    /// Its entry's place in the log.
    slot: Option<usize>,
    /// The first alternative to try. It is past every alternative where
    /// the choice point fails at once.
    pub(super) start: u32,
    /// The cuts of the alternatives before it.
    pub(super) cuts: Cuts,
    /// The value a settled choice point answered in a replay.
    pub(super) settled: Option<u32>,
    /// Whether the replay resumes the search here.
    pub(super) resumed: bool,
}

impl Choice {
    /// A choice point that fails at once, cut, as every choice point of a
    /// diverged replay does.
    const FAILED: Self = Self {
        slot: None,
        start: u32::MAX,
        cuts: Cuts::BUDGET,
        settled: None,
        resumed: false,
    };
}

/// The log, the replay under way, and how many rules keep the log.
#[derive(Debug, Default)]
pub(super) struct Choices {
    /// The entries, in the order their choice points were entered.
    log: Vec<Entry>,
    /// The entries a replay follows.
    script: Vec<Entry>,
    /// The next entry of the script.
    cursor: usize,
    /// The entry of the script where the replay resumes the search.
    resume_at: usize,
    /// Whether a replay follows the script.
    replaying: bool,
    /// Whether the replay met what the script did not say.
    diverged: bool,
    /// How many rules that may ask for another answer are open.
    open: u32,
}

impl Choices {
    /// Enters a choice point of the kind `site`.
    pub(super) fn enter(&mut self, site: Site) -> Choice {
        if self.diverged {
            return Choice::FAILED;
        }
        if self.replaying {
            let Some(&entry) = self.script.get(self.cursor).filter(|e| e.site == site) else {
                self.diverge();
                return Choice::FAILED;
            };
            let resumed = self.cursor == self.resume_at;
            self.cursor += 1;
            self.log.push(entry);
            let slot = Some(self.log.len() - 1);
            if resumed {
                self.replaying = false;
            }
            let (start, settled) = match entry.taken {
                Taken::Open(i) if resumed => (i + 1, None),
                Taken::Open(i) => (i, None),
                Taken::Failed => (u32::MAX, None),
                Taken::Settled(value) => (u32::MAX, Some(value)),
            };
            return Choice {
                slot,
                start,
                cuts: entry.cuts,
                settled,
                resumed,
            };
        }
        let slot = (self.open > 0).then(|| {
            self.log.push(Entry {
                site,
                taken: Taken::Open(0),
                cuts: Cuts::NONE,
            });
            self.log.len() - 1
        });
        Choice {
            slot,
            start: 0,
            cuts: Cuts::NONE,
            settled: None,
            resumed: false,
        }
    }

    /// Logs that the choice point is about to try alternative `i`, and
    /// that those before it cut `cuts`. In a replay it may try only the
    /// logged one.
    pub(super) fn take(&mut self, choice: &Choice, i: u32, cuts: Cuts) {
        let Some(slot) = choice.slot else {
            return;
        };
        if self.replaying && self.log[slot].taken != Taken::Open(i) {
            self.diverge();
            return;
        }
        self.log.truncate(slot + 1);
        self.log[slot] = Entry {
            taken: Taken::Open(i),
            cuts,
            ..self.log[slot]
        };
    }

    /// Logs that a rule that asks its first premise for answers begins
    /// round `i`, and that the rounds before it cut `cuts`. Its premises'
    /// choices stay logged, for the resumption that begins the round.
    pub(super) fn next_round(&mut self, choice: &Choice, i: u32, cuts: Cuts) {
        if let Some(slot) = choice.slot {
            self.log[slot] = Entry {
                taken: Taken::Open(i),
                cuts,
                ..self.log[slot]
            };
        }
    }

    /// Logs that the choice point failed, with the cuts of every
    /// alternative. In a replay only one that the log says failed may
    /// fail.
    pub(super) fn fail(&mut self, choice: &Choice, cuts: Cuts) {
        let Some(slot) = choice.slot else {
            return;
        };
        if self.replaying && self.log[slot].taken != Taken::Failed {
            self.diverge();
            return;
        }
        self.log.truncate(slot + 1);
        self.log[slot] = Entry {
            taken: Taken::Failed,
            cuts,
            ..self.log[slot]
        };
    }

    /// Logs that the choice point answered `value` and has nothing to
    /// resume.
    pub(super) fn settle(&mut self, choice: &Choice, value: u32) {
        let Some(slot) = choice.slot else {
            return;
        };
        self.log.truncate(slot + 1);
        self.log[slot] = Entry {
            taken: Taken::Settled(value),
            cuts: Cuts::NONE,
            ..self.log[slot]
        };
    }

    /// Opens a rule that may ask its first premise for another answer.
    /// The log is kept while one is open.
    pub(super) fn open(&mut self) {
        self.open += 1;
    }

    /// Closes the rule opened last. The log is cleared when none is open.
    pub(super) fn close(&mut self) {
        self.open -= 1;
        if self.open == 0 {
            self.log.clear();
        }
    }

    /// Where the log ends.
    pub(super) fn len(&self) -> usize {
        self.log.len()
    }

    /// Drops the log from `at` on.
    pub(super) fn truncate(&mut self, at: usize) {
        self.log.truncate(at);
    }

    /// Resumes the search whose choice points the log lists from `start`
    /// to `end`, as the module doc says. The log is cut back to `start`,
    /// and the replay is set up.
    ///
    /// Returns the cuts of every logged choice point as an error where
    /// none took an alternative, so that no other answer is left. During a
    /// replay, which never resumes another, it fails with a cut.
    pub(super) fn resume(&mut self, start: usize, end: usize) -> Result<(), Cuts> {
        if self.replaying || self.diverged {
            self.diverge();
            return Err(Cuts::BUDGET);
        }
        let entries = &self.log[start..end];
        let rest = |entries: &[Entry]| entries.iter().fold(Cuts::NONE, |c, e| c.and(e.cuts));
        let Some(at) = entries
            .iter()
            .rposition(|e| matches!(e.taken, Taken::Open(_)))
        else {
            let cuts = rest(entries);
            self.log.truncate(start);
            return Err(cuts);
        };
        let given_up = rest(&entries[at + 1..]);
        self.script.clear();
        self.script.extend_from_slice(entries);
        self.script[at].cuts = self.script[at].cuts.and(given_up);
        self.log.truncate(start);
        self.cursor = 0;
        self.resume_at = at;
        self.replaying = true;
        Ok(())
    }

    /// Whether a replay follows the script. The excluded answers are not
    /// read until it resumes, since the search it replays met none.
    pub(super) fn replaying(&self) -> bool {
        self.replaying
    }

    /// Ends the replay of the first premise just searched. Returns true
    /// where it diverged, or ended before it resumed: its result then
    /// stands for nothing.
    pub(super) fn end_replay(&mut self) -> bool {
        let diverged = self.diverged || self.replaying;
        self.diverged = false;
        self.replaying = false;
        self.script.clear();
        diverged
    }

    /// The bytes the log and the script hold.
    pub(super) fn bytes(&self) -> usize {
        (self.log.capacity() + self.script.capacity()) * size_of::<Entry>()
    }

    /// Marks the replay as diverged: it met what the script did not say.
    fn diverge(&mut self) {
        self.diverged = true;
        self.replaying = false;
    }
}

#[cfg(test)]
mod tests {
    use super::{Choices, Cuts, Site};

    /// A resumption replays every choice point to its logged alternative,
    /// with the cuts of those tried before it, and goes on past the last
    /// one that took an alternative, with the cuts of every failure
    /// logged after it.
    #[test]
    fn a_resumption_goes_on_at_the_last_open_choice() {
        let mut choices = Choices::default();
        choices.open();
        let stable = choices.enter(Site::Choose);
        choices.take(&stable, 0, Cuts::NONE);
        choices.take(&stable, 1, Cuts::BUDGET);
        let pair = choices.enter(Site::Initial);
        choices.take(&pair, 2, Cuts::NONE);
        let absorbed = choices.enter(Site::Initial);
        choices.fail(&absorbed, Cuts::repeat(3));
        let end = choices.len();
        assert_eq!(choices.resume(0, end), Ok(()));
        let stable = choices.enter(Site::Choose);
        assert_eq!(
            (stable.start, stable.cuts, stable.resumed),
            (1, Cuts::BUDGET, false)
        );
        choices.take(&stable, 1, stable.cuts);
        let pair = choices.enter(Site::Initial);
        assert_eq!(
            (pair.start, pair.cuts, pair.resumed),
            (3, Cuts::repeat(3), true)
        );
        assert!(!choices.replaying());
        assert!(!choices.end_replay());
    }

    /// A replay that meets another choice point than its log's, or tries
    /// another alternative before its resume point, has diverged: every
    /// choice point fails at once, cut, and the rule is told.
    #[test]
    fn a_replay_off_its_log_diverges() {
        let mut choices = Choices::default();
        choices.open();
        let stable = choices.enter(Site::Choose);
        choices.take(&stable, 0, Cuts::NONE);
        let split = choices.enter(Site::Splits);
        choices.take(&split, 0, Cuts::NONE);
        let end = choices.len();
        choices.resume(0, end).unwrap();
        let other = choices.enter(Site::Initial);
        assert_eq!((other.start, other.cuts), (u32::MAX, Cuts::BUDGET));
        assert!(choices.end_replay());
        let mut choices = Choices::default();
        choices.open();
        let stable = choices.enter(Site::Choose);
        choices.take(&stable, 0, Cuts::NONE);
        let split = choices.enter(Site::Splits);
        choices.take(&split, 0, Cuts::NONE);
        choices.resume(0, choices.len()).unwrap();
        let stable = choices.enter(Site::Choose);
        choices.take(&stable, 1, Cuts::NONE);
        assert!(choices.end_replay());
    }

    /// Where no choice point took an alternative, the answer was the only
    /// one. There is no replay, and the result has the cuts of every
    /// failure logged.
    #[test]
    fn no_open_choice_leaves_no_answer() {
        let mut choices = Choices::default();
        choices.open();
        let pair = choices.enter(Site::Initial);
        choices.fail(&pair, Cuts::BUDGET);
        let parts = choices.enter(Site::Parts);
        choices.settle(&parts, 1);
        assert_eq!(choices.resume(0, choices.len()), Err(Cuts::BUDGET));
        assert!(!choices.replaying());
    }
}
