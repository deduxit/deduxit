// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The focused sequent engine. It searches backward for a proof, over
//! dyadic sequents of occurrence ids, and keeps a memo of the stable
//! sequents it has decided. It follows the `MALL-Seq` and `MELL-Seq`
//! specifications.
//!
//! A sequent inside the search is `⊢ Θ ; Γ`. It is *dyadic*: it has two
//! zones. The *unrestricted zone* `Θ` is a set of the formulas that arrived
//! under a `?`, and each may be copied any number of times. The *linear
//! zone* `Γ` is a multiset of occurrences, each used exactly once.
//!
//! The *asynchronous phase* applies the rules that need no choice to the
//! negative formulas: `⅋` opens, `⊥` drops, `⊤` closes, `&` branches into
//! two premises with the same context, and `?` moves its subformula into
//! `Θ`. What remains is a *stable* sequent: its `Γ` holds positive formulas
//! and negative literals only.
//!
//! `prove` decides a stable sequent. It first looks the sequent up in the
//! memo, then runs the immediate tests: a `0`, an unbalanced atom, a dual
//! pair, a literal whose dual lies in `Θ`. Otherwise it chooses a positive
//! formula to *focus* on. That is a member of `Γ`, which the focus
//! consumes, or a member of `Θ`, which stays there. A focus on a member of
//! `Θ` is a *copy*. It is the one step that can repeat, so the copies on
//! each branch are bounded.
//!
//! `focus` applies the positive rules along the focused formula (the
//! *synchronous phase*): `⊕` picks a side, `⊗` splits `Γ`, `1` needs `Γ`
//! empty, `!` needs `Γ` empty and goes on with its subformula, and a
//! positive literal needs exactly its dual, in `Γ` or in `Θ`. When the
//! formula turns negative, it is *released* into the asynchronous phase.
//!
//! Every stable sequent proved or refuted goes into the memo. So a `&` that
//! duplicates the context, or a split that comes back to a part, never
//! pays twice. With Mix, a stable sequent that no focus proves is split
//! into two parts, each proved on its own. In affine mode a leaf weakens
//! whatever is left over.
//!
//! The *copy bound* limits the copies on each branch. The search deepens
//! it level by level, and answers `Unprovable` only after a level that
//! never hit its bound.
//!
//! Given the sequent's intuitionistic reading, the same engine searches
//! intuitionistic linear logic two-sided. Every rule of the two-sided
//! focused calculus is a rule of this engine on the one-sided sequent.
//! Every branch keeps exactly one output formula by itself, except at the
//! split of a hypothesis `A ⊸ B` (a `⊗` in input position). There the goal
//! must stay on the consequent's side. That is the one place the engine
//! reads the reading.
//!
//! The counts of `counts.rs` prune. A stable sequent, or a side of a split,
//! whose count intervals exclude zero for some atom is refuted without
//! search. In the multiplicative fragments the count equation prunes as
//! well.

/// The proof arena: pending and kept nodes.
mod arena;
/// The atom bias.
pub(crate) mod bias;
/// The choices a first-order search logs to resume a premise.
mod choices;
/// Interchangeable occurrences.
mod classes;
/// The linear zone as a multiset.
mod context;
/// The count invariants the engine prunes with.
pub(crate) mod counts;
/// The rules a first-order zone adds.
mod first_order;
/// The zone of a first-order search.
mod framed;
/// The memo and the loop check of a first-order search.
mod keyed;
/// The memo of stable sequents.
mod memo;
#[cfg(feature = "parallel")]
pub(crate) mod parallel;
/// The choice points of a first-order search that log their choices.
mod resumed;
/// The two searches of the default bias and how they share the cores.
mod schedule;
/// The pools of scratch buffers.
mod scratch;
/// The `⊗` rule and Mix: forced and searched splits.
mod split;
#[cfg(all(test, feature = "parse"))]
mod tests;
/// The failures that wait on an open ancestor.
mod waiting;
/// What the engine is generic over: its zones' members and how their
/// literals meet.
mod zone;

use self::arena::Arena;
use self::classes::Classes;
use self::counts::{Counts, Split, Tally, intervals_run};
use self::framed::Framed;
use self::memo::{Entry, Failure, Inserted, Key, Table, Zones};
use self::schedule::{Plan, plan, turns};
use self::scratch::{Pooled, Pools};
use self::split::Join;
use self::waiting::Waiting;
use self::zone::{Ground, Linear as _, Unrestricted as _, Zone};
use super::memory::{Account, Charged};
use super::{
    Answer, Decide, Finished, Options, Reason, Statistics, Stop, Task, Work, set_up_stopped,
};
use crate::Error;
use crate::limits::Limits;
use crate::linear::fragment::{Fragment, Mode};
use crate::linear::occurrences::{Forest, OccId, Reading};
use crate::linear::proofs::{Branch, Node, NodeId};
use crate::linear::sequents::Kind;

/// The stack depth that stands for "no pruned sequent depends on an
/// ancestor".
const NO_DEPENDENCY: u32 = u32::MAX;

/// How many steps of its searches for the splits of a `⊗` or a Mix the
/// engine takes between two polls of the stop condition.
const SPLITS_PER_POLL: u64 = 4096;

/// How many forced splits, literals of the tensors they close in place and
/// `&` rules the engine takes between two polls of the stop condition.
const FORCED_PER_POLL: u64 = 4096;

/// The work of a stable sequent before what depends on its size, in steps
/// of a split search (the unit a turn is counted in). A step moves one
/// member and tests its row; the bookkeeping of a visit costs some ten
/// times as much.
const NODE_WORK: u64 = 16;

/// How many occurrences of the forest make one unit of a stable sequent's
/// work. This charges a stable sequent what a key of both zones as bitsets
/// of the forest's width would cost. The key is sparse, but the charge
/// stays: the default bias's two searches take their slices by it, and
/// their counts of work depend on it.
const OCCURRENCES_PER_WORK: usize = 128;

/// How many comparisons of a member of `Γ` with a literal of a formula of
/// `Θ` make one unit of work. The copies are ranked by whether they meet a
/// member. On a Petri net, with its hundreds of transitions and tokens,
/// that ranking is most of what a stable sequent costs.
const MEETS_PER_WORK: usize = 16;

/// How many occurrences of the forest make one unit of the work of a split
/// whose premises are tried. Its two sides are built as zones of the
/// forest's width.
const OCCURRENCES_PER_LEAF: usize = 512;

/// The focused engine as the front door calls it, one-sided or two-sided.
pub(crate) struct Focused {
    /// Whether it is the two-sided engine of intuitionistic mode.
    two_sided: bool,
}

/// The focused engine of classical mode.
pub(crate) const ONE_SIDED: Focused = Focused { two_sided: false };

/// The focused engine of intuitionistic mode.
pub(crate) const TWO_SIDED: Focused = Focused { two_sided: true };

impl Decide for Focused {
    /// Refuses a goal beyond `LL1`. Also refuses the one-sided engine in
    /// intuitionistic mode, and the two-sided one in classical mode.
    fn admits(&self, task: &Task<'_>) -> Result<(), Error> {
        let decides = Fragment::LL1;
        let because = if !decides.contains(task.fragment) {
            super::NotTaken::Fragment {
                decides,
                goal: task.fragment,
            }
        } else if task.mode.intuitionistic != self.two_sided {
            super::NotTaken::Mode { mode: task.mode }
        } else {
            return Ok(());
        };
        let engine = if self.two_sided {
            super::Engine::TwoSided
        } else {
            super::Engine::Focus
        };
        Err(Error::EngineRefused { engine, because })
    }

    /// Searches the goal on one thread, or with the feature `parallel` on
    /// [`Options::jobs`] threads.
    fn decide(
        &self,
        task: &Task<'_>,
        options: &Options,
        limits: &Limits,
        account: &Account,
        work: &Work,
        stop: &mut dyn FnMut(u64) -> bool,
    ) -> Result<Answer, Error> {
        let first_order = task.fragment.has_quantifiers();
        #[cfg(feature = "parallel")]
        if options.threads() > 1 {
            let found = if first_order {
                parallel::search_goal::<Framed>(task, options, limits, account, work, stop)?
            } else {
                parallel::search_goal::<Ground>(task, options, limits, account, work, stop)?
            };
            return Ok(Answer::of_arena(task.forest, found, stop));
        }
        let found = if first_order {
            search_goal::<Framed>(task, options, limits, account, work, stop)
        } else {
            search_goal::<Ground>(task, options, limits, account, work, stop)
        };
        Ok(Answer::of_arena(task.forest, found, stop))
    }
}

/// Runs the focused engine on a goal. Returns the node that proves it
/// (`None` when it is unprovable, or the reason the search gave up), the
/// arena the node lives in, and the statistics.
///
/// The goal is a multiset of occurrences of the forest, with an empty
/// unrestricted zone. It is the sequent's roots for a search of the
/// sequent itself, or an open goal that an interactive prover hands over.
/// When a reading is given, exactly one of its occurrences is in output
/// position.
///
/// What the search allocates is charged to `account`. Where two searches
/// decide the goal together, each has half its bound.
fn search_goal<Z: Zone>(
    task: &Task<'_>,
    options: &Options,
    limits: &Limits,
    account: &Account,
    work: &Work,
    stop: &mut dyn FnMut(u64) -> bool,
) -> Finished {
    let forest = task.forest;
    let gave_up = |r: Reason| Finished::gave_up(r.as_set(options, limits));
    let Some((classes, first, second)) = prepare(task, options, stop) else {
        return gave_up(Reason::Stopped);
    };
    let set_up = SetUp {
        task,
        classes: &classes,
        options,
        limits,
    };
    let Some(second) = second else {
        account.charge(classes.bytes());
        let counts = match Counts::new_until(forest, first.bias, task.mode, account, stop) {
            Ok(counts) => counts,
            Err(reason) => return gave_up(reason),
        };
        let (finished, _) = first.search::<Z>(set_up, &counts, account, Stop::Closure(stop));
        return Finished {
            result: finished.result.map_err(|r| r.as_set(options, limits)),
            ..finished
        };
    };
    // Each search is charged what it reads, the classes included.
    let accounts = [account.share(2), account.share(2)];
    for account in &accounts {
        account.charge(classes.bytes());
    }
    let first_counts = match Counts::new_until(forest, first.bias, task.mode, &accounts[0], stop) {
        Ok(counts) => counts,
        Err(reason) => return gave_up(reason),
    };
    let second_counts = match Counts::new_until(forest, second.bias, task.mode, &accounts[1], stop)
    {
        Ok(counts) => counts,
        Err(reason) => return gave_up(reason),
    };
    let searches = [
        (first, &first_counts, &accounts[0]),
        (second, &second_counts, &accounts[1]),
    ];
    // With threads, the two searches alternate in slices, and neither starts
    // over. Otherwise they take turns on this thread, each turn from its
    // start, and this thread's stop adds up the work. That is the case
    // without the feature, when a thread cannot start, or when the options
    // ask for it.
    #[cfg(not(feature = "parallel"))]
    let _ = work;
    #[cfg(feature = "parallel")]
    if options.schedule == super::Schedule::Auto
        && let Some(result) = schedule::alternate::<Z>(set_up, searches, work, stop)
    {
        return result;
    }
    turns::<Z>(set_up, searches, stop)
}

/// What every search of a goal is set up with: the task, the classes of
/// its forest, the options and the limits.
#[derive(Clone, Copy)]
struct SetUp<'a> {
    /// The goal, its forest, fragment, mode and reading.
    task: &'a Task<'a>,
    /// The forest's classes of interchangeable occurrences.
    classes: &'a Classes,
    /// The options.
    options: &'a Options,
    /// The limits.
    limits: &'a Limits,
}

/// Makes what every search of the goal needs first, on one thread or a
/// pool: the forest's classes and the plan of its searches. Polls after
/// each on a large forest. Returns `None` when the stop fired at a poll.
fn prepare(
    task: &Task<'_>,
    options: &Options,
    stop: &mut dyn FnMut(u64) -> bool,
) -> Option<(Classes, Plan, Option<Plan>)> {
    let classes = Classes::new(task.forest, task.reading);
    if set_up_stopped(task.forest, stop) {
        return None;
    }
    let (first, second) = plan(task.forest, task.goal, task.fragment, task.mode, options);
    if set_up_stopped(task.forest, stop) {
        return None;
    }
    Some((classes, first, second))
}

/// The rules in force beyond the core ones, switched by fragment and mode.
#[derive(Clone, Copy, Debug)]
struct Switches {
    /// The Mix rule, tried last on a stable sequent; off under weakening,
    /// which makes it admissible.
    mix: bool,
    /// The `MLL` count equation as a prune: only without additives and
    /// exponentials anywhere in the problem, and never with weakening.
    equation: bool,
    /// The interval check as a prune: never with weakening, which can
    /// discard any imbalance.
    intervals: bool,
    /// The exponential rules: `?` into the unrestricted zone, copies from it
    /// under the bound, `!` in focus.
    exponentials: bool,
    /// Weakening: a leaf discards what is left over.
    affine: bool,
    /// The stack of the branch's stable sequents, for the loop check when
    /// copies can repeat a sequent.
    stack: bool,
}

impl Switches {
    /// The rules for a fragment and a mode: the count equation only in the
    /// multiplicative fragments without weakening, the interval check
    /// unless weakening or a `⊤` under an exponential defeats it, and Mix
    /// only without weakening.
    fn new(fragment: Fragment, mode: Mode, counts: &Counts) -> Self {
        let exponentials = fragment.has_exponentials();
        Self {
            // With weakening Mix proves nothing new: the proof of one part
            // proves the whole with the other part weakened at its leaves,
            // with the same copies on every branch.
            mix: mode.mix && !mode.affine,
            equation: !mode.affine
                && !fragment.has_additives()
                && !fragment.has_additive_units()
                && !exponentials,
            intervals: intervals_run(mode, counts.absorbs_from_copies()),
            exponentials,
            affine: mode.affine,
            stack: exponentials,
        }
    }
}

/// The result of a search: the node proving the goal, `None` when it is
/// unprovable, or the reason the search stopped.
pub(crate) type Searched = Result<Option<NodeId>, Reason>;

/// What a failure below a step rests on, besides the sequents it searched:
/// a branch cut by the copy budget (the copies a branch may still make),
/// or a prune by the loop check (which prunes a sequent equal to one of
/// its ancestors). A prune is a fact about the branch, not about the
/// sequent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cuts {
    /// Whether some branch failed because its copy budget was spent. A level
    /// whose search failed with this set does not answer `Unprovable`.
    exhausted: bool,
    /// The shallowest stack depth of an ancestor that a pruned sequent
    /// repeated, or [`NO_DEPENDENCY`]. A failure that rests on such a prune
    /// is not memoized.
    dependency: u32,
}

impl Cuts {
    /// Nothing cut, nothing pruned.
    const NONE: Self = Self {
        exhausted: false,
        dependency: NO_DEPENDENCY,
    };

    /// A branch cut by the copy budget.
    const BUDGET: Self = Self {
        exhausted: true,
        dependency: NO_DEPENDENCY,
    };

    /// A sequent pruned as a repeat of the ancestor at `depth` on the
    /// branch stack.
    fn repeat(depth: u32) -> Self {
        Self {
            exhausted: false,
            dependency: depth,
        }
    }

    /// Both together: a cut if either was, the shallower dependency.
    fn and(self, other: Self) -> Self {
        Self {
            exhausted: self.exhausted || other.exhausted,
            dependency: self.dependency.min(other.dependency),
        }
    }
}

/// What a step of the search found: a proof of its sequent, or no proof, with
/// the cuts of every step it ran.
///
/// A failure rests on the failures below it and on nothing else. So a step that
/// runs others adds the cuts of those that failed to its own. A proof is a fact
/// that rests on nothing, so it carries no cuts. When a rule proves one premise
/// and fails on the other, the failure rests on the failed premise's cuts
/// alone. A stable sequent reads off its decision's result what its memo entry
/// may say.
#[derive(Clone, Copy, Debug)]
enum Found {
    /// A proof, by this node.
    Proved(NodeId),
    /// No proof, with the cuts of the steps run.
    Failed(Cuts),
}

impl Found {
    /// Nothing proved and nothing cut: a failure that is a fact.
    const NOTHING: Self = Self::Failed(Cuts::NONE);

    /// A proof.
    fn proved(node: NodeId) -> Self {
        Self::Proved(node)
    }

    /// A failure with these cuts.
    fn failed(cuts: Cuts) -> Self {
        Self::Failed(cuts)
    }

    /// The node of the proof, if there is one.
    fn node(self) -> Option<NodeId> {
        match self {
            Self::Proved(node) => Some(node),
            Self::Failed(_) => None,
        }
    }

    /// The same result after steps with the cuts given failed first: a
    /// failure rests on them too, a proof on nothing.
    fn after(self, cuts: Cuts) -> Self {
        match self {
            Self::Proved(node) => Self::Proved(node),
            Self::Failed(own) => Self::Failed(cuts.and(own)),
        }
    }

    /// The same result with the proof, if there is one, wrapped by `wrap`.
    fn map(self, wrap: impl FnOnce(NodeId) -> NodeId) -> Self {
        match self {
            Self::Proved(node) => Self::Proved(wrap(node)),
            failed => failed,
        }
    }
}

/// The result of a step of the search: what it found, or the reason the
/// whole search stops.
type Step = Result<Found, Reason>;

/// One alternative of a choice. One thread tries the alternatives in its
/// order; a pool's search runs each as a task of its own.
enum Alternative<'m, Z: Zone> {
    /// A focus on a member of `Γ`, which leaves the context.
    Focus(Z::Member),
    /// A copy of a member of `Θ` in focus, at one unit less of the budget.
    Copy(Z::Member),
    /// A side of a `⊕` in focus, with the subformula on that side.
    Branch(Z::Member, Branch, Z::Member),
    /// The free splits of a `⊗` (the splits of the context it searches) that
    /// assign the first `fixed` members as the bits of `pattern` say, a set
    /// bit for the left premise. They start from the sides and counts before
    /// any member moved.
    #[cfg_attr(
        not(feature = "parallel"),
        expect(dead_code, reason = "only a pool hands out the splits of a tensor")
    )]
    Splits {
        /// The `⊗` rule the sides are the premises of.
        join: Join<Z::Member>,
        /// The members of the context that the search assigns.
        members: &'m [Z::Member],
        /// How many of them the pattern assigns.
        fixed: usize,
        /// Their assignment.
        pattern: u64,
        /// The sides before any member moved.
        sides: (&'m Z::Linear, &'m Z::Linear),
        /// Their counts, with the members not yet assigned.
        split: &'m Split,
    },
}

impl<Z: Zone> Clone for Alternative<'_, Z> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Z: Zone> Copy for Alternative<'_, Z> {}

/// What every engine of one search reads and none changes: the problem,
/// the rules it is searched under, the account its memory is charged to,
/// and its limits. A pool's workers are built from the same one.
#[derive(Clone, Copy)]
struct Problem<'a> {
    /// The forest.
    forest: &'a Forest,
    /// Its intuitionistic reading, for a two-sided search.
    reading: Option<&'a Reading<'a>>,
    /// Its count invariants.
    counts: &'a Counts,
    /// Its classes of interchangeable occurrences.
    classes: &'a Classes,
    /// The rules in force.
    rules: Switches,
    /// The search's account, which the memo, the arena and the buffers are
    /// charged to.
    account: &'a Account,
    /// Whether the memo takes entries. A proof that an entry refers to is
    /// kept.
    memoizes: bool,
    /// The most failures that may wait on an open ancestor at once. It is the
    /// memo's own bound in entries.
    waits: usize,
    /// The deepest nesting allowed.
    recursion_limit: u32,
    /// The most copies a branch may take at the last deepening level.
    copies: u32,
}

impl<'a> Problem<'a> {
    /// The problem of a search of the set-up's goal with its counts and a
    /// copy bound of `copies`, charging `account`.
    fn new(set_up: SetUp<'a>, counts: &'a Counts, copies: u32, account: &'a Account) -> Self {
        let task = set_up.task;
        Self {
            forest: task.forest,
            reading: task.reading,
            counts,
            classes: set_up.classes,
            rules: Switches::new(task.fragment, task.mode, counts),
            account,
            memoizes: set_up.options.memo_limit != 0,
            waits: set_up.options.memo_entries(),
            recursion_limit: set_up.limits.recursion_depth,
            copies,
        }
    }
}

/// The unrestricted zone of a sequent: the subformulas of the `?`
/// formulas decomposed on the branch, and the id the memo interned it
/// under ([`Table::intern`]), by which keys name it.
struct Unrestricted<Z: Zone> {
    /// The zone.
    set: Z::Unrestricted,
    /// Its id. Two different zones never have the same id.
    id: u64,
}

/// The members of an unrestricted zone in the order of their classes.
struct ByClass<Z: Zone> {
    /// The id of the zone; `None` before the first.
    zone: Option<u64>,
    /// Its members with their classes, by class and then by id.
    members: Pooled<(Z::Member, Z::Member)>,
}

/// The state of one run: the problem, the memo, the proof arena, the
/// counters, the branch, and pools of scratch buffers so that no step
/// allocates once the pools are warm.
struct Run<'a, Z: Zone> {
    /// The problem: the forest and what was computed from it, the rules,
    /// the account and the limits.
    problem: Problem<'a>,
    /// The zones' members and how their literals meet.
    zone: Z,
    /// The memo of stable sequents.
    memo: Table<'a>,
    /// The proof arena.
    nodes: Arena<'a>,
    /// What the engine's own buffers were charged: the branch stack and
    /// the pools, which go with the engine.
    scratch: Charged<'a>,
    /// The counters.
    statistics: Statistics,
    /// The steps of split searches since the stop condition was last
    /// polled there.
    steps: u64,
    /// The forced splits, and the literals of the tensors they closed in
    /// place, since the stop condition was last polled in a chain of them.
    forced: u64,
    /// The work done since the stop condition was last polled: the part that
    /// grows with the size of the sequents and is not a step of a split
    /// search. It is counted in the unit of a turn.
    work: u64,
    /// The nesting of engine calls right now.
    depth: u32,
    /// The stop condition.
    stop: Stop<'a>,
    /// The runtime, when the search runs on several threads.
    #[cfg(feature = "parallel")]
    runtime: Option<&'a super::parallel::Runtime>,
    /// Whether this engine ran a choice's alternatives on several threads
    /// in the level being searched.
    #[cfg(feature = "parallel")]
    divided: bool,
    /// How many choices among alternatives on this branch ran on several
    /// threads: the levels of cube-and-conquer (a choice's alternatives
    /// divided among threads) above the current sequent.
    #[cfg_attr(
        not(feature = "parallel"),
        expect(dead_code, reason = "only a pool runs alternatives on several threads")
    )]
    or_depth: u32,
    /// The stable sequents of the current branch, the root end first; only
    /// the first `stack_len` are live, the rest are spare buffers.
    stack: Vec<Key>,
    /// The hash of every entry of the stack, live or spare: the loop check
    /// compares hashes before sequents.
    hashes: Vec<u64>,
    /// How many entries of the stack are live.
    stack_len: usize,
    /// Per entry of the stack, live or spare: how many failures waited
    /// when it was pushed, so that those decided below it are the ones
    /// after.
    marks: Vec<usize>,
    /// The failures that rest on a prune of the loop check and wait on an
    /// open ancestor of the branch.
    waiting: Waiting,
    /// The stable sequents of the branch above the stack, the root end first,
    /// with their hashes. They are the parts of the branch stacks of the
    /// engines this worker of a pool was spawned from, read in place. The
    /// depth of an entry of `stack` counts these first.
    ancestors: Vec<(&'a [Key], &'a [u64])>,
    /// The spare buffers.
    pools: Pools<Z>,
    /// The unrestricted zone of the stable sequent whose copies were
    /// listed last, with its members by class.
    by_class: ByClass<Z>,
}

impl<'a, Z: Zone> Run<'a, Z> {
    /// Prepares a run at the root of its branch, on the problem, with the
    /// stop condition, the memo, the arena and the zone to use. A pool's
    /// worker is such a run, then given the branch it continues.
    fn new(
        problem: Problem<'a>,
        stop: Stop<'a>,
        memo: Table<'a>,
        nodes: Arena<'a>,
        zone: Z,
    ) -> Self {
        Self {
            problem,
            zone,
            memo,
            scratch: Charged::new(problem.account),
            nodes,
            statistics: Statistics::default(),
            steps: 0,
            forced: 0,
            work: 0,
            depth: 0,
            stop,
            #[cfg(feature = "parallel")]
            runtime: None,
            #[cfg(feature = "parallel")]
            divided: false,
            or_depth: 0,
            stack: Vec::new(),
            hashes: Vec::new(),
            stack_len: 0,
            marks: Vec::new(),
            waiting: Waiting::default(),
            ancestors: Vec::new(),
            pools: Pools::default(),
            by_class: ByClass {
                zone: None,
                members: Pooled::default(),
            },
        }
    }

    /// Searches the goal ([`Run::run`]) and keeps the proof found. Returns
    /// the kept node, `None` when the goal is unprovable, or why the search
    /// stopped.
    fn run_kept(&mut self, goal: &[OccId]) -> Result<Option<NodeId>, Reason> {
        let Some(root) = self.run(goal)? else {
            return Ok(None);
        };
        let stop = &mut self.stop;
        self.nodes.keep_all(root, &mut || stop.fired(0)).map(Some)
    }

    /// The occurrence of a member.
    #[inline]
    fn occ(&self, m: Z::Member) -> OccId {
        self.zone.occurrence(m)
    }

    /// The kind of a member's occurrence.
    #[inline]
    fn kind(&self, m: Z::Member) -> Kind {
        self.problem.forest.kind(self.occ(m))
    }

    /// The member of the left subformula of a member's occurrence.
    #[inline]
    fn left(&self, m: Z::Member) -> Z::Member {
        let left = self.problem.forest.left(self.occ(m)).unwrap();
        self.zone.child(m, left)
    }

    /// The member of the right subformula of a member's occurrence.
    #[inline]
    fn right(&self, m: Z::Member) -> Z::Member {
        let right = self.problem.forest.right(self.occ(m)).unwrap();
        self.zone.child(m, right)
    }

    /// The class of a member.
    #[inline]
    fn class(&self, m: Z::Member) -> Z::Member {
        self.zone.class(self.problem.classes, m)
    }

    /// Returns the counters, with the memo's.
    fn statistics(&self) -> Statistics {
        let mut statistics = Statistics {
            memo_hits: self.memo.hits(),
            memo_entries: self.memo.peak() as u64,
            ..self.statistics
        };
        if Z::FIRST_ORDER {
            let (unifications, bindings) = self.zone.counts();
            statistics.unifications += unifications;
            statistics.bindings += bindings;
        }
        statistics
    }

    /// The counters a worker hands the engine that spawned it: its own and
    /// its zone's. The memo's are left out: a pool's engines share the memo,
    /// or each reads its own.
    #[cfg(feature = "parallel")]
    fn report(&self) -> Statistics {
        let mut statistics = self.statistics;
        if Z::FIRST_ORDER {
            let (unifications, bindings) = self.zone.counts();
            statistics.unifications += unifications;
            statistics.bindings += bindings;
        }
        statistics
    }

    /// Searches the goal with an empty unrestricted zone. Runs the
    /// asynchronous phase on its formulas once per copy bound, from zero up
    /// to the configured one, until a level proves the goal or fails without
    /// ever spending its budget. Without exponentials there is one level.
    fn run(&mut self, goal: &[OccId]) -> Searched {
        let levels = if self.problem.rules.exponentials {
            self.problem.copies
        } else {
            0
        };
        let set = self.take_set();
        let theta = Unrestricted {
            id: self.memo.intern(set.ground(), self.problem.account),
            set,
        };
        // The generation whose failures cut by the budget the levels read
        // ([`Table::begin`]). A generation is every level since the last one
        // that ended cut without a stable sequent new to the memo, or with a
        // part of the memo emptied.
        let mut generation = 0;
        self.memo.begin(generation);
        // Up to `u32::MAX` inclusive without a wrap; that bound stands for
        // none and is never reached (`Options::copy_bound`).
        for budget in 0..=levels {
            self.statistics.copies = budget;
            self.waiting.clear();
            if Z::FIRST_ORDER {
                // A level that a first-order bound cuts at once reaches no
                // stable sequent, which is where the stop is polled. So each
                // level counts as work and polls here.
                if let Err(reason) = self.poll_with() {
                    self.give_set(theta.set);
                    return Err(reason);
                }
                self.zone.begin_level();
                #[cfg(feature = "parallel")]
                {
                    self.divided = false;
                }
            }
            let (added, emptied) = (self.memo.added(), self.memo.emptied());
            let mut gamma = self.take_context();
            let mut list = self.take_list();
            // A stack: the first formula is decomposed first.
            list.extend(goal.iter().rev().map(|&o| self.zone.root(o)));
            let result = self.asynchronous(&theta, &mut gamma, &mut list, budget);
            #[cfg(feature = "parallel")]
            let result = if Z::FIRST_ORDER && budget == levels {
                self.again_alone(&theta, goal, budget, result, &mut generation)
            } else {
                result
            };
            self.give_context(gamma);
            self.give_list(list);
            match result {
                Ok(Found::Failed(cuts)) if cuts.exhausted => {
                    // Either the level met no stable sequent it had not met
                    // before, so the next one would read the same cuts again.
                    // Or a part of the memo was emptied, so the keys met again
                    // would count as new for ever. Either way a new generation
                    // starts: the next level reads only the facts and its own
                    // cuts.
                    if self.memo.added() == added || self.memo.emptied() != emptied {
                        generation += 1;
                        self.memo.begin(generation);
                    }
                }
                decided => {
                    self.give_set(theta.set);
                    return decided.map(Found::node);
                }
            }
        }
        self.give_set(theta.set);
        // When a first-order bound cut the last level, the goal is undecided
        // for that bound, which a larger copy bound alone may not lift.
        if Z::FIRST_ORDER
            && let Some(reason) = self.zone.cut()
        {
            return Err(reason);
        }
        Err(Reason::CopyBound { copies: levels })
    }

    /// Searches the last level of a first-order search on a pool once more,
    /// on this thread, where the level divided a choice and ended cut.
    /// Returns the level's result.
    ///
    /// A worker may have searched a sequent cut by its budget while a sibling
    /// was still finding it unprovable. One thread would have read the
    /// sibling's failure and decided. The level searched again reads every
    /// failure the workers recorded in the memo and none of their cuts (a new
    /// generation). So the pool answers "unknown" only where one thread with
    /// those facts does.
    #[cfg(feature = "parallel")]
    fn again_alone(
        &mut self,
        theta: &Unrestricted<Z>,
        goal: &[OccId],
        budget: u32,
        result: Step,
        generation: &mut u64,
    ) -> Step {
        let cut = matches!(result, Ok(Found::Failed(cuts)) if cuts.exhausted);
        if !(cut && std::mem::take(&mut self.divided)) {
            return result;
        }
        *generation += 1;
        self.memo.begin(*generation);
        let mut gamma = self.take_context();
        let mut list = self.take_list();
        list.extend(goal.iter().rev().map(|&o| self.zone.root(o)));
        let runtime = self.runtime.take();
        let again = self.asynchronous(theta, &mut gamma, &mut list, budget);
        self.runtime = runtime;
        self.give_context(gamma);
        self.give_list(list);
        again
    }

    // The phases.

    /// The asynchronous phase on `⊢ Θ ; Γ ⇑ L`, where `L` is the list of
    /// formulas still to decompose. Decomposes the negative formulas of the
    /// list until the sequent is stable, then proves it. The list is a stack;
    /// the order does not matter for completeness.
    fn asynchronous(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &mut Z::Linear,
        list: &mut Vec<Z::Member>,
        budget: u32,
    ) -> Step {
        self.enter()?;
        let result = self.decompose(theta, gamma, list, budget);
        self.leave();
        result
    }

    /// The body of the asynchronous phase. The `?` rules are applied in the
    /// loop like `⅋` and `⊥`, on a copy of the unrestricted zone that
    /// grows, so that a sequent of thousands of `?` formulas costs one
    /// level of recursion and not one each.
    fn decompose(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &mut Z::Linear,
        list: &mut Vec<Z::Member>,
        budget: u32,
    ) -> Step {
        // The `⅋`, `⊥` and `?` rules applied, to wrap around the proof of
        // what remains; the first applied is the lowest.
        let mut applied = self.take_list();
        // The unrestricted zone once a `?` added a formula to it, and,
        // once it is complete, the zone with the id it was interned under.
        let mut grown: Option<Z::Unrestricted> = None;
        let mut interned: Option<Unrestricted<Z>> = None;
        // Where the zone stood, for a first-order phase that fails.
        let mark = Z::FIRST_ORDER.then(|| self.zone.mark());
        let result = loop {
            let Some(o) = list.pop() else {
                interned = self.interned(grown.take());
                break self.prove(interned.as_ref().unwrap_or(theta), gamma, budget);
            };
            match self.kind(o) {
                Kind::Par => {
                    list.push(self.right(o));
                    list.push(self.left(o));
                    applied.push(o);
                }
                Kind::Bot => applied.push(o),
                Kind::Top => break Ok(Found::proved(self.push(Node::Top(o.into())))),
                Kind::With => {
                    interned = self.interned(grown.take());
                    break self.with(interned.as_ref().unwrap_or(theta), gamma, list, o, budget);
                }
                Kind::Quest => {
                    // The subformula joins the unrestricted zone, which is
                    // a set: a formula already there changes nothing.
                    let a = self.left(o);
                    if !grown.as_ref().unwrap_or(&theta.set).contains(a) {
                        if grown.is_none() {
                            let mut larger = self.take_set();
                            larger.copy_from(&theta.set);
                            grown = Some(larger);
                        }
                        grown.as_mut().unwrap().insert(a);
                    }
                    applied.push(o);
                }
                // A fresh eigenvariable for the body, which the `∀` node takes
                // back as it is built. The arm is the last one, so that for a
                // zone without binders the match compiles as if it were not
                // there.
                kind if Z::FIRST_ORDER && kind == Kind::Forall => {
                    match self.zone.forall(self.problem.forest, o) {
                        Ok(Some(body)) => {
                            list.push(body);
                            applied.push(o);
                        }
                        Ok(None) => break Ok(Found::failed(Cuts::BUDGET)),
                        Err(reason) => break Err(reason),
                    }
                }
                _ => {
                    // A positive formula or a literal: part of the stable
                    // sequent.
                    gamma.insert(o);
                }
            }
        };
        if let Some(larger) = grown.or(interned.map(|zone| zone.set)) {
            self.give_set(larger);
        }
        if let (Some(mark), Ok(Found::Failed(_))) = (mark, &result) {
            self.zone.undo(mark);
        }
        let result = result.map(|found| {
            found.map(|mut node| {
                for &o in applied.iter().rev() {
                    node = match self.kind(o) {
                        Kind::Par => self.push(Node::Par(o.into(), node)),
                        Kind::Quest => self.push(Node::Quest(o.into(), node)),
                        kind if Z::FIRST_ORDER && kind == Kind::Forall => {
                            let eigen = self.zone.introduced();
                            self.push(Node::Forall(o.into(), eigen, node))
                        }
                        _ => self.push(Node::Bot(o.into(), node)),
                    };
                }
                node
            })
        });
        self.give_list(applied);
        result
    }

    /// Returns a zone that grew with the id the memo interns it under.
    fn interned(&mut self, grown: Option<Z::Unrestricted>) -> Option<Unrestricted<Z>> {
        grown.map(|set| Unrestricted {
            id: self.memo.intern(set.ground(), self.problem.account),
            set,
        })
    }

    /// The `&` rule. Both premises have the same context, and each keeps the
    /// whole copy budget. The right one is searched only once the left one is
    /// proved. On several threads both are searched at once.
    fn with(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &mut Z::Linear,
        list: &mut Vec<Z::Member>,
        o: Z::Member,
        budget: u32,
    ) -> Step {
        // A proof of `&` rules closed by `⊤` reaches no stable sequent, which is
        // where the search polls and checks the memory bound. And such a proof
        // can be exponential in the sequent. So the rule polls and checks here.
        self.poll_with()?;
        if self.problem.account.over() {
            self.relieve()?;
        }
        if Z::FIRST_ORDER {
            return self.with_framed(theta, gamma, list, o, budget);
        }
        #[cfg(feature = "parallel")]
        if self.cubes() {
            return self.with_parallel(theta, gamma, list, o, budget);
        }
        let mark = self.nodes.mark();
        let left = self.premise(theta, gamma, list, self.left(o), budget)?;
        let Found::Proved(left_node) = left else {
            return Ok(left);
        };
        // The right premise on the state itself, which the rule leaves.
        list.push(self.right(o));
        let (left_node, right) = self.holding(left_node, |run| {
            run.asynchronous(theta, gamma, list, budget)
        });
        Ok(self.both(o, left_node, right?, mark))
    }

    /// Counts the work of an `&` rule: a step, and the copy of a zone of the
    /// forest's width. Polls the stop every [`FORCED_PER_POLL`] `&` rules and
    /// forced splits, and tells it the work done since its last poll. An `&`
    /// rule is work, which the default bias's two searches share and
    /// `Limits::work` bounds.
    fn poll_with(&mut self) -> Result<(), Reason> {
        self.work += 1 + (self.problem.forest.len() / OCCURRENCES_PER_WORK) as u64;
        self.forced += 1;
        if self.forced < FORCED_PER_POLL {
            return Ok(());
        }
        self.forced = 0;
        if self.stop.fired(std::mem::take(&mut self.work)) {
            Err(Reason::Stopped)
        } else {
            Ok(())
        }
    }

    /// A premise of the `&` rule on copies of the state: the context and
    /// the list with the subformula on top.
    fn premise(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        list: &[Z::Member],
        sub: Z::Member,
        budget: u32,
    ) -> Step {
        let mut premise_gamma = self.take_context_from(gamma);
        let mut premise_list = self.take_list();
        premise_list.extend_from_slice(list);
        premise_list.push(sub);
        let result = self.asynchronous(theta, &mut premise_gamma, &mut premise_list, budget);
        self.give_context(premise_gamma);
        self.give_list(premise_list);
        result
    }

    /// Runs `second`, the search of a rule's second premise, with the proof
    /// of its first premise held ([`Arena::hold`]). Returns that proof where
    /// it is now, and what `second` returned.
    ///
    /// A collection of the arena in between may move the proof, hence the
    /// hold. Every rule that keeps a proof across a call that can reach
    /// [`prove_stable`](Self::prove_stable) or [`with`](Self::with) goes
    /// through it, since both may relieve the memory and collect the arena.
    #[inline(always)]
    fn holding<T>(&mut self, first: NodeId, second: impl FnOnce(&mut Self) -> T) -> (NodeId, T) {
        let held = self.nodes.hold(first);
        let found = second(self);
        (self.nodes.unhold(held), found)
    }

    /// A rule of two premises. `first` searches the first premise. Once it is
    /// proved, `second` searches the second, with the first's proof held
    /// across. `join` makes the rule's node from the two proofs. Otherwise
    /// returns the failure of either premise. When the second fails, the nodes
    /// pushed since the first began are released.
    #[inline(always)]
    fn two_premises(
        &mut self,
        first: impl FnOnce(&mut Self) -> Step,
        second: impl FnOnce(&mut Self) -> Step,
        join: impl FnOnce(NodeId, NodeId) -> Node,
    ) -> Step {
        let mark = self.nodes.mark();
        let l = first(self)?;
        let Found::Proved(l_node) = l else {
            return Ok(l);
        };
        let (l_node, r) = self.holding(l_node, second);
        let r = r?;
        let Found::Proved(r_node) = r else {
            self.nodes.release(mark);
            return Ok(r);
        };
        Ok(Found::proved(self.push(join(l_node, r_node))))
    }

    /// The `&` node on `o`, from the proof of its left premise and what the
    /// search of the right one found. When the right one failed, returns its
    /// failure, with the nodes pending since `mark` released.
    fn both(&mut self, o: Z::Member, left: NodeId, right: Found, mark: usize) -> Found {
        let Found::Proved(right) = right else {
            self.nodes.release(mark);
            return right;
        };
        Found::proved(self.push(Node::With(o.into(), left, right)))
    }

    /// Decides `⊢ Θ ; Γ` for a stable `Γ`. Tries the memo, the loop check
    /// and the immediate tests, then a focus on each positive formula in
    /// turn: first from `Γ`, then copied from `Θ`, then Mix.
    fn prove(&mut self, theta: &Unrestricted<Z>, gamma: &Z::Linear, budget: u32) -> Step {
        Ok(self.prove_part(theta, gamma, budget)?.0)
    }

    /// [`Self::prove`], and whether a failure is hereditary: no non-empty
    /// sub-multiset of `Γ` is provable either.
    fn prove_part(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        budget: u32,
    ) -> Result<(Found, bool), Reason> {
        self.enter()?;
        let result = self.prove_stable(theta, gamma, budget);
        self.leave();
        result
    }

    /// The body of `prove`: the poll, then the memo and the branch stack
    /// around the decision ([`Self::prove_listed`]), with the members of
    /// `Γ` listed once. Says whether a failure is hereditary.
    fn prove_stable(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        budget: u32,
    ) -> Result<(Found, bool), Reason> {
        self.statistics.nodes += 1;
        let work = NODE_WORK
            + (self.problem.forest.len() / OCCURRENCES_PER_WORK) as u64
            + std::mem::take(&mut self.work);
        if self.stop.fired(work) {
            return Err(Reason::Stopped);
        }
        if self.problem.account.over() {
            self.relieve()?;
        }
        if !self.zone.memoizes(&theta.set, gamma) {
            return self.prove_unkeyed(theta, gamma, budget);
        }
        // The key's words: the members, each once, where none repeats.
        let mut words = self.take_words();
        let ground = gamma.ground();
        let repeats = ground.repeats();
        if repeats {
            ground.key_into(&mut words);
        } else {
            ground.list_into(&mut words);
        }
        let mut canonical = self.take_words();
        let result = self.prove_listed(theta, gamma, (&words, repeats), &mut canonical, budget);
        self.give_words(words);
        self.give_words(canonical);
        result
    }

    /// Decides a stable sequent that the memo and the loop check may not key
    /// by its occurrences. Where the memo is on and the sequent has keys, a
    /// first-order sequent is keyed by its instance instead (`prove_keyed`).
    #[cold]
    fn prove_unkeyed(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        budget: u32,
    ) -> Result<(Found, bool), Reason> {
        if Z::FIRST_ORDER
            && self.problem.memoizes
            && let Some(decided) = self.prove_keyed(theta, gamma, budget)
        {
            return decided;
        }
        let mut members = self.take_list();
        gamma.list_into(&mut members);
        let mark = self.nodes.mark();
        let zone = Z::FIRST_ORDER.then(|| self.zone.mark());
        let result = self.decide(theta, gamma, &members, budget);
        self.give_list(members);
        if let Ok((Found::Failed(_), _)) = result {
            self.nodes.release(mark);
            if let Some(zone) = zone {
                self.zone.undo(zone);
            }
        }
        result
    }

    /// Decides a stable sequent through the memo and the branch stack. Its
    /// linear zone comes as the words of its key ([`Zones::members`]), which
    /// are its members where none repeats. `canonical` is a buffer for its
    /// canonical key.
    fn prove_listed(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        (words, repeats): (&[OccId], bool),
        canonical: &mut Vec<OccId>,
        budget: u32,
    ) -> Result<(Found, bool), Reason> {
        // The memo and the loop check read the zones where they are, under
        // one hash.
        let key = Zones {
            theta: theta.id,
            members: words,
        };
        let hash = if self.problem.memoizes || self.problem.rules.stack {
            key.hash()
        } else {
            0
        };
        // Complete failures are recorded up to interchangeable members, under
        // the canonical key: the key with every member of `Γ` replaced by its
        // class. It is built only where it differs from the sequent's own key.
        let renamed = self.problem.classes.renames(gamma.ground());
        if renamed {
            self.problem.classes.canonical(words, repeats, canonical);
            debug_assert!(
                *canonical != words,
                "a member's class is another occurrence"
            );
        }
        let canonical_key = Zones {
            theta: theta.id,
            members: canonical,
        };
        let canonical_hash = if renamed && self.problem.memoizes {
            canonical_key.hash()
        } else {
            0
        };
        // A proof or a complete failure from the memo settles the sequent. A
        // failure cut by the budget waits for the loop check, which may give the
        // stronger answer that the branch is redundant.
        let refuted = if renamed {
            self.memo.refuted(canonical_key, canonical_hash)
        } else {
            None
        };
        let entry = match refuted {
            Some(failure) => Some(Entry::Failed(failure)),
            None => self.memo.get(key, hash, budget),
        };
        match entry {
            Some(Entry::Proved(node)) => return Ok((Found::proved(node), false)),
            Some(Entry::Failed(failure @ (Failure::Complete | Failure::Hereditary))) => {
                return Ok((Found::NOTHING, failure == Failure::Hereditary));
            }
            Some(Entry::Failed(Failure::Exhausted(_))) | None => {}
        }
        // A sequent that repeats an ancestor on its branch is pruned, since a
        // smallest proof of the ancestor never passes through it. The failure
        // this causes above is a fact about the branch. So it carries a
        // dependency on the ancestor's depth, which keeps the sequents between
        // them out of the memo until the ancestor itself is decided.
        //
        // A sequent that merely contains an ancestor is not redundant, with or
        // without weakening. A proof of the larger sequent proves nothing about
        // the smaller one: ⊢ ?(a ⅋ ~a) is proved only through
        // ⊢ a ⅋ ~a ; a, ~a.
        if self.problem.rules.stack
            && let Some(depth) = self.repeated(key, hash)
        {
            // A depth of the branch, which the recursion limit bounds, so it fits a
            // `u32`.
            return Ok((Found::failed(Cuts::repeat(depth as u32)), false));
        }
        // A failure that waits on an open ancestor answers for the sequent while
        // that ancestor is open. One not cut by the budget answers before the
        // memo's entry cut by the budget; one cut by it answers after.
        if !self.waiting.is_empty()
            && let Some(reused) = self.waiting.find(key, hash, budget, entry.is_some())
        {
            if entry.is_none() {
                self.memo.count_hit(hash);
            }
            let cuts = Cuts {
                exhausted: reused.exhausted,
                dependency: reused.leader,
            };
            return Ok((Found::failed(cuts), reused.hereditary));
        }
        if entry.is_some() {
            // Cut by the budget at this or a larger remaining budget.
            return Ok((Found::failed(Cuts::BUDGET), false));
        }
        if self.problem.rules.stack {
            if self.stack_len == self.stack.len() {
                self.scratch
                    .charge(size_of::<Key>() + size_of::<u64>() + size_of::<usize>());
                self.stack.push(Key::default());
                self.hashes.push(0);
                self.marks.push(0);
            }
            // Into the entry's own buffer, which grows to the longest
            // linear zone it held.
            let pushed = &mut self.stack[self.stack_len];
            pushed.assign(key);
            pushed.members.settle(&mut self.scratch);
            self.hashes[self.stack_len] = hash;
            self.marks[self.stack_len] = self.waiting.len();
            self.stack_len += 1;
        }
        let mark = self.nodes.mark();
        // The members with their repeats, listed only to be decided: a
        // marking of a hundred tokens is a hundred members.
        let listed = if repeats { None } else { Z::listed(words) };
        let result = if let Some(members) = listed {
            self.decide(theta, gamma, members, budget)
        } else {
            let mut members = self.take_list();
            gamma.list_into(&mut members);
            let result = self.decide(theta, gamma, &members, budget);
            self.give_list(members);
            result
        };
        let own_depth = if self.problem.rules.stack {
            self.stack_len -= 1;
            (self.above() + self.stack_len) as u32
        } else {
            NO_DEPENDENCY
        };
        let (found, hereditary) = result?;
        // What a failure rests on, as the steps it ran returned it. A
        // dependency on this very sequent is settled with it: the pruned
        // descendants could only have proved what this sequent proves.
        let (node, cuts) = match found {
            Found::Proved(node) => (Some(node), Cuts::NONE),
            Found::Failed(cuts) => (
                None,
                Cuts {
                    exhausted: cuts.exhausted,
                    dependency: if cuts.dependency >= own_depth {
                        NO_DEPENDENCY
                    } else {
                        cuts.dependency
                    },
                },
            ),
        };
        if self.problem.rules.stack && self.problem.memoizes {
            let waited = self.marks[self.stack_len];
            debug_assert!(
                waited <= self.waiting.len(),
                "the failures decided below a stable sequent are gone unsettled"
            );
            let closed = self.close(
                key,
                hash,
                budget,
                waited,
                own_depth,
                (node, cuts, hereditary),
            );
            self.scratch.charge(self.waiting.grown());
            closed?;
        }
        let recorded = match node {
            Some(proof) if self.problem.memoizes => {
                // The entry outlives the branch, so the proof is kept.
                self.nodes.keep(mark, proof).and_then(|proof| {
                    match self.remember(key, hash, Entry::Proved(proof))? {
                        Entry::Proved(proof) => Ok(Some(proof)),
                        Entry::Failed(_) => unreachable!("the entry of a proof"),
                    }
                })
            }
            None => {
                self.nodes.release(mark);
                let mut recorded = Ok(None);
                if cuts.dependency == NO_DEPENDENCY {
                    // A complete failure answers for every relative (a sequent
                    // equal up to interchangeable members). One cut by the
                    // budget stays under the sequent's own key.
                    let complete = if hereditary {
                        Failure::Hereditary
                    } else {
                        Failure::Complete
                    };
                    recorded = if cuts.exhausted {
                        self.remember(key, hash, Entry::Failed(Failure::Exhausted(budget)))
                    } else if renamed {
                        self.remember(canonical_key, canonical_hash, Entry::Failed(complete))
                    } else {
                        self.remember(key, hash, Entry::Failed(complete))
                    }
                    .map(|_| None);
                }
                recorded
            }
            Some(proof) => Ok(Some(proof)),
        };
        Ok(match recorded? {
            Some(node) => (Found::proved(node), false),
            None => (Found::failed(cuts), hereditary),
        })
    }

    /// Settles the failures that wait on an ancestor and were decided below
    /// the stable sequent `key` just closed, those from `waited` on.
    /// `waiting.rs` has the argument. They are settled by what `key` closed
    /// with. A proof drops them. A failure cut by the budget makes them cut
    /// by it too. A failure with no dependency records in the memo those it
    /// leads, or that a deeper ancestor led. A failure with a dependency hands
    /// them on to its leader, and waits there itself.
    fn close(
        &mut self,
        key: Zones<'_>,
        hash: u64,
        budget: u32,
        waited: usize,
        own_depth: u32,
        (node, cuts, hereditary): (Option<NodeId>, Cuts, bool),
    ) -> Result<(), Reason> {
        if node.is_some() {
            self.waiting.drop_from(waited);
            return Ok(());
        }
        if cuts.exhausted {
            self.waiting.exhaust_from(waited);
        }
        if cuts.dependency == NO_DEPENDENCY {
            let mut recorded = Ok(());
            while let Some(entry) = self.waiting.take_settled(waited, own_depth) {
                if recorded.is_ok() {
                    recorded = self.record_failure(
                        entry.key.zones(),
                        entry.hash,
                        entry.budget,
                        (entry.exhausted, entry.hereditary),
                    );
                }
                self.waiting.give(entry.key);
            }
            self.waiting.settled();
            return recorded;
        }
        if (cuts.dependency as usize) < self.above() {
            // A leader in the branch of the engine that spawned this one,
            // whose close this engine never sees.
            self.waiting.drop_from(waited);
            return Ok(());
        }
        self.waiting.lead_from(waited, cuts.dependency);
        if self.waiting.len() < self.problem.waits {
            let mut entry = self.waiting.spare().unwrap_or_default();
            entry.assign(key);
            entry.members.settle(&mut self.scratch);
            self.waiting.wait(
                entry,
                hash,
                budget,
                (cuts.exhausted, hereditary),
                cuts.dependency,
            );
        }
        Ok(())
    }

    /// Records a failure in the memo: one cut by the budget under the
    /// sequent's own key, a complete one under its canonical key, where
    /// every relative reads it.
    fn record_failure(
        &mut self,
        key: Zones<'_>,
        hash: u64,
        budget: u32,
        (exhausted, hereditary): (bool, bool),
    ) -> Result<(), Reason> {
        if exhausted {
            return self
                .remember(key, hash, Entry::Failed(Failure::Exhausted(budget)))
                .map(|_| ());
        }
        let complete = if hereditary {
            Failure::Hereditary
        } else {
            Failure::Complete
        };
        if !self.problem.classes.renames_any(key.members) {
            return self
                .remember(key, hash, Entry::Failed(complete))
                .map(|_| ());
        }
        let mut canonical = self.take_words();
        let repeats = key.members.iter().any(|o| o.get() & memo::REPEATED != 0);
        self.problem
            .classes
            .canonical(key.members, repeats, &mut canonical);
        let canonical_key = Zones {
            theta: key.theta,
            members: &canonical,
        };
        let recorded = self.remember(canonical_key, canonical_key.hash(), Entry::Failed(complete));
        self.give_words(canonical);
        recorded.map(|_| ())
    }

    /// Returns the depth of the stable sequent of the branch that `key` (of
    /// hash `hash`) repeats, if any. The depth counts first the ancestors that
    /// spawning engines hold, then the stack's live entries.
    fn repeated(&self, key: Zones<'_>, hash: u64) -> Option<usize> {
        let own = (
            &self.stack[..self.stack_len],
            &self.hashes[..self.stack_len],
        );
        self.ancestors
            .iter()
            .chain([&own])
            .flat_map(|&(keys, hashes)| keys.iter().zip(hashes))
            .position(|(entry, &h)| h == hash && key.of(entry))
    }

    /// How many stable sequents of the branch lie above this engine's
    /// stack.
    fn above(&self) -> usize {
        self.ancestors.iter().map(|(keys, _)| keys.len()).sum()
    }

    /// Records what the search found out about a stable sequent, and returns
    /// the entry as recorded.
    ///
    /// A memo of the engine's own that has no room is emptied first, and the
    /// kept proofs that only its entries referred to go with it. A proof's
    /// node then comes back under the id it has after that. Fails when even
    /// the emptied memo has no room: the search's memory is at its bound
    /// without it.
    fn remember(&mut self, key: Zones<'_>, hash: u64, entry: Entry) -> Result<Entry, Reason> {
        if self.memo.insert(key, hash, entry, self.problem.account) == Inserted::Done {
            return Ok(entry);
        }
        self.memo.clear();
        let entry = match entry {
            Entry::Proved(node) => Entry::Proved(
                self.nodes
                    .collect(Some(node), false)
                    .expect("the root given"),
            ),
            failed => {
                self.nodes.collect(None, false);
                failed
            }
        };
        if self.memo.insert(key, hash, entry, self.problem.account) == Inserted::Done {
            return Ok(entry);
        }
        // Not even the first entry fits. The memory the dropped proofs leave
        // free is the last there is, so all of it is given back.
        let entry = match entry {
            Entry::Proved(node) => Entry::Proved(
                self.nodes
                    .collect(Some(node), true)
                    .expect("the root given"),
            ),
            failed => {
                self.nodes.collect(None, true);
                failed
            }
        };
        match self.memo.insert(key, hash, entry, self.problem.account) {
            Inserted::Done => Ok(entry),
            _ => Err(Reason::MemoryLimit {
                limit_bytes: self.problem.account.limit(),
            }),
        }
    }

    /// Makes room when the search holds more than its bound. Empties the memo
    /// and drops the kept proofs that only its entries referred to. If that
    /// is not enough, gives the memo's own memory back; the memo takes it
    /// again as far as the rest leaves room.
    ///
    /// Fails when what is left is still over the bound: the branch's own
    /// buffers and proofs, as they are allocated. Squeezing those would be
    /// undone by the next node.
    fn relieve(&mut self) -> Result<(), Reason> {
        let charged = self.waiting.release();
        self.scratch.resize(charged, 0);
        // Every failure that waits from now on is decided below every
        // stable sequent of the branch, which must settle it when it
        // closes.
        self.marks[..self.stack_len].fill(0);
        self.memo.clear();
        self.nodes.collect(None, false);
        if !self.problem.account.over() {
            return Ok(());
        }
        self.memo.release(self.problem.account);
        if self.problem.account.over() {
            Err(Reason::MemoryLimit {
                limit_bytes: self.problem.account.limit(),
            })
        } else {
            Ok(())
        }
    }

    /// The bytes a set of the forest's width allocates.
    fn set_bytes(&self) -> usize {
        self.problem.forest.len().div_ceil(64) * size_of::<u64>()
    }

    /// Decides a stable sequent the memo does not know, and says whether a
    /// failure is hereditary. Under Mix, a failure is hereditary when no
    /// non-empty part of `Γ` is provable either: `Γ` has one member, or Mix
    /// found that each part with one member less failed hereditarily.
    fn decide(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        members: &[Z::Member],
        budget: u32,
    ) -> Result<(Found, bool), Reason> {
        let mut tally = self.take_tally();
        let mut candidates = self.take_list();
        let mut copies = self.take_list();
        let result = self
            .decide_with(
                theta,
                gamma,
                members,
                &mut tally,
                &mut candidates,
                &mut copies,
                budget,
            )
            .map(|(found, hereditary)| {
                // The one part of a single member is itself.
                let alone = self.problem.rules.mix
                    && members.len() == 1
                    && matches!(found, Found::Failed(_));
                (found, hereditary || alone)
            });
        self.give_tally(tally);
        self.give_list(candidates);
        self.give_list(copies);
        result
    }

    /// The body of `decide`, with its scratch buffers.
    #[allow(clippy::too_many_arguments)]
    fn decide_with(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        members: &[Z::Member],
        tally: &mut Tally,
        candidates: &mut Vec<Z::Member>,
        copies: &mut Vec<Z::Member>,
        budget: u32,
    ) -> Result<(Found, bool), Reason> {
        let affine = self.problem.rules.affine;
        // One pass over the members: the counts, a `0`, and the positive
        // formulas worth focusing on. `1` and `!` are worth it only when alone,
        // since they need an empty context (any context, with weakening). A
        // literal never is: a focus on it succeeds only in the initial cases
        // tested below.
        self.work += members.len() as u64;
        let mut zero = false;
        for &o in members {
            tally.add(self.problem.counts, self.occ(o));
            match self.kind(o) {
                Kind::Zero => zero = true,
                Kind::Tensor | Kind::Plus => candidates.push(o),
                Kind::One | Kind::Bang if members.len() == 1 || affine => candidates.push(o),
                Kind::One | Kind::Bang | Kind::Atom | Kind::DualAtom => {}
                kind if Z::FIRST_ORDER && kind == Kind::Exists => candidates.push(o),
                kind => unreachable!("{kind:?} in a stable sequent"),
            }
        }
        // A `0` has no rule. So only a `⊤` below some member of `Γ`, or below a
        // member of `Θ` that a copy can bring in, can prove the sequent:
        // ⊢ 0, ⊤ ⊕ b is provable. A `0` in a stable sequent is fatal only
        // without such a `⊤`. With weakening the `0` is discarded at a leaf like
        // anything else.
        if zero
            && !affine
            && !tally.absorbs()
            && !theta
                .set
                .iter()
                .any(|a| self.problem.counts.absorbs(self.occ(a)))
        {
            return Ok((Found::NOTHING, false));
        }
        let mut cuts = match self.initial(theta, gamma, members, budget)? {
            Found::Failed(cuts) => cuts,
            proved => return Ok((proved, false)),
        };
        if self.problem.rules.intervals && !tally.balanced() {
            return Ok((Found::failed(cuts), false));
        }
        if self.problem.rules.equation && !tally.equation(self.problem.rules.mix) {
            return Ok((Found::failed(cuts), false));
        }

        // Of interchangeable candidates one is enough: the focus on either
        // leaves the same sequent up to a renaming.
        self.one_of_each(candidates);
        // Forced splits first, then `⊕`, then the free splits; by id within
        // a class, so that the run is deterministic.
        candidates.sort_by_key(|&o| (self.focus_class(o), o));
        // After them come the copies from `Θ`. A formula with an unconsumed copy
        // in `Γ` is skipped: a second copy cannot help before the first is used.
        // Those that can meet a literal of `Γ` come first, then the rest by id.
        if self.problem.rules.exponentials {
            let unconsumed = self.one_of_each_copy(theta, gamma, copies);
            // Each is looked up in `Γ`, compared with its class and sorted.
            self.work += 2 * unconsumed as u64;
            if budget == 0 {
                // A branch cut by the bound: the level cannot claim
                // completeness, unless nothing was there to copy.
                if !copies.is_empty() {
                    cuts = cuts.and(Cuts::BUDGET);
                }
                copies.clear();
            } else {
                // By id, those that meet a member first: the heuristic is
                // asked once per formula, not once per comparison.
                copies.sort_unstable();
                self.work += (copies.len() * members.len() / MEETS_PER_WORK) as u64;
                self.zone
                    .mark_literals(self.problem.forest, members, &mut self.scratch);
                let mut others = self.take_list();
                let mut met = 0;
                for i in 0..copies.len() {
                    let a = copies[i];
                    if self.zone.meets(self.problem.forest, a) {
                        copies[met] = a;
                        met += 1;
                    } else {
                        others.push(a);
                    }
                }
                copies.truncate(met);
                copies.extend_from_slice(&others);
                self.give_list(others);
            }
        }
        let alternatives = candidates
            .iter()
            .map(|&f| Alternative::Focus(f))
            .chain(copies.iter().map(|&a| Alternative::Copy(a)));
        let cuts = match self.choose(theta, gamma, alternatives, budget)?.after(cuts) {
            Found::Failed(cuts) => cuts,
            proved => return Ok((proved, false)),
        };
        let (found, hereditary) = self.last_resort(theta, gamma, members, tally, budget)?;
        Ok((found.after(cuts), hereditary))
    }

    /// Decides a choice among alternatives. On this thread they are tried in
    /// their order, and the first that proves the sequent ends the choice.
    /// Within the first levels of a pool's search they run on its threads. A
    /// failure carries the cuts of the alternatives searched.
    fn choose<'m>(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        alternatives: impl Iterator<Item = Alternative<'m, Z>>,
        budget: u32,
    ) -> Step {
        #[cfg(feature = "parallel")]
        if self.cubes() {
            let alternatives: Vec<Alternative<'m, Z>> = alternatives.collect();
            if alternatives.len() >= 2 {
                return self.choose_parallel(theta, gamma, &alternatives, budget);
            }
            return self.choose_here(theta, gamma, alternatives.into_iter(), budget);
        }
        self.choose_here(theta, gamma, alternatives, budget)
    }

    /// Decides a choice among alternatives on this engine, in their order.
    fn choose_here<'m>(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        alternatives: impl Iterator<Item = Alternative<'m, Z>>,
        budget: u32,
    ) -> Step {
        if Z::FIRST_ORDER {
            return self.choose_framed(theta, gamma, alternatives, budget);
        }
        let mut cuts = Cuts::NONE;
        let mut rest = None;
        for alternative in alternatives {
            match self.alternative(theta, gamma, alternative, &mut rest, budget)? {
                Found::Failed(failed) => cuts = cuts.and(failed),
                proved => {
                    if let Some(rest) = rest {
                        self.give_context(rest);
                    }
                    return Ok(proved);
                }
            }
        }
        if let Some(rest) = rest {
            self.give_context(rest);
        }
        Ok(Found::failed(cuts))
    }

    /// Searches one alternative of a choice on this engine. `rest` is the
    /// context a focus on a member of `Γ` leaves, taken from the pool the
    /// first time one needs it.
    fn alternative(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        alternative: Alternative<'_, Z>,
        rest: &mut Option<Z::Linear>,
        budget: u32,
    ) -> Step {
        match alternative {
            Alternative::Focus(f) => {
                if rest.is_none() {
                    *rest = Some(self.take_context_any());
                }
                let rest = rest.as_mut().expect("taken");
                rest.copy_from(gamma);
                rest.remove(f);
                self.focus(theta, rest, f, budget)
            }
            Alternative::Copy(a) => Ok(self
                .focus(theta, gamma, a, budget - 1)?
                .map(|node| self.push(Node::Copy(a.into(), node)))),
            Alternative::Branch(f, side, sub) => Ok(self
                .focus(theta, gamma, sub, budget)?
                .map(|node| self.push(Node::Plus(f.into(), side, node)))),
            Alternative::Splits {
                join,
                members,
                fixed,
                pattern,
                sides,
                split,
            } => {
                let mut left = self.take_context_from(sides.0);
                let mut right = self.take_context_from(sides.1);
                let mut counts = self.take_split();
                (*counts).clone_from(split);
                for (i, &m) in members.iter().enumerate().take(fixed) {
                    if pattern >> i & 1 == 1 {
                        right.remove(m);
                        left.insert(m);
                        counts.assign(self.problem.counts, self.occ(m), Branch::Left);
                    } else {
                        counts.assign(self.problem.counts, self.occ(m), Branch::Right);
                    }
                }
                let result = self.search_splits(
                    theta,
                    members,
                    (fixed, pattern),
                    (&mut left, &mut right),
                    &mut counts,
                    join,
                    budget,
                );
                self.give_context(left);
                self.give_context(right);
                self.give_split(counts);
                result
            }
        }
    }

    /// Lists in `copies` one member of each class among the members of `Θ`
    /// without a copy in `Γ`: the lowest id, in the order of the classes.
    /// That is [`Self::one_of_each`] on them. Returns how many members `Θ`
    /// has without a copy in `Γ`.
    ///
    /// `Θ`'s members by class are kept from the stable sequent before when it
    /// had the same `Θ`, as it mostly has along a branch. Sorting them anew
    /// for every stable sequent is most of what one costs on a net of
    /// thousands of transitions.
    fn one_of_each_copy(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        copies: &mut Vec<Z::Member>,
    ) -> usize {
        let by_class = &mut self.by_class;
        if by_class.zone != Some(theta.id) {
            by_class.zone = Some(theta.id);
            by_class.members.clear();
            by_class.members.extend(
                theta
                    .set
                    .iter()
                    .map(|a| (self.zone.class(self.problem.classes, a), a)),
            );
            by_class.members.sort_unstable();
            by_class.members.settle(&mut self.scratch);
        }
        let mut unconsumed = 0;
        let mut last = None;
        for &(class, a) in self.by_class.members.iter() {
            if gamma.contains(a) {
                continue;
            }
            unconsumed += 1;
            if last != Some(class) {
                copies.push(a);
                last = Some(class);
            }
        }
        unconsumed
    }

    /// Keeps, of the occurrences that are interchangeable, the one with
    /// the lowest id.
    fn one_of_each(&self, occurrences: &mut Vec<Z::Member>) {
        occurrences.sort_unstable_by_key(|&o| (self.class(o), o));
        occurrences.dedup_by_key(|o| self.class(*o));
    }

    /// Tries what is left on a stable sequent after every focus failed: Mix,
    /// when the rules have it. Says whether the failure is hereditary, which
    /// it is when no part of `Γ` is provable.
    fn last_resort(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        members: &[Z::Member],
        tally: &Tally,
        budget: u32,
    ) -> Result<(Found, bool), Reason> {
        if self.problem.rules.mix {
            return self.mix(theta, gamma, members, tally, budget);
        }
        Ok((Found::NOTHING, false))
    }

    /// The initial rules on a stable sequent: a dual pair in `Γ`, or a literal
    /// of `Γ` whose dual lies in `Θ`. That dual is copied, and the copy counts
    /// against the budget. In linear mode `Γ` must be exactly the pair or the
    /// literal; in affine mode anything else in `Γ` is weakened away.
    ///
    /// The first literal of the members, in their order, that has a dual among
    /// them pairs with the first such dual. Otherwise the first literal with a
    /// dual in `Θ` is copied. One pass marks the lists of the members'
    /// literals, so that the test is linear in the members. Comparing every
    /// pair would be quadratic under weakening, where any number of members
    /// may close a sequent.
    fn initial(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        members: &[Z::Member],
        budget: u32,
    ) -> Step {
        if Z::FIRST_ORDER {
            return self.initial_framed(theta, gamma, members, budget);
        }
        let affine = self.problem.rules.affine;
        if !affine && members.len() > 2 {
            return Ok(Found::NOTHING);
        }
        let f = self.problem.forest;
        self.zone.mark_literals(f, members, &mut self.scratch);
        let mut cuts = Cuts::NONE;
        for &p in members {
            let Some(dual) = self.zone.duals(f, p) else {
                continue;
            };
            // No member before `p` has a dual among the members, else the
            // pair would have closed the sequent there, so `p`'s first dual
            // comes after it.
            if (affine || members.len() == 2)
                && let Some(q) = self.zone.marked(dual)
            {
                let ax = self.push(Node::Ax(p.into(), q.into()));
                if !affine {
                    return Ok(Found::proved(ax));
                }
                let mut rest = self.take_context_from(gamma);
                rest.remove(p);
                rest.remove(q);
                let node = self.weakened(&rest, ax);
                self.give_context(rest);
                return Ok(Found::proved(node));
            }
            // Without exponentials `Θ` is empty; with them, a literal's
            // dual is looked up in `Θ` once, since another copy of it finds
            // what the first found.
            if !(affine || members.len() == 1)
                || !self.problem.rules.exponentials
                || !self.zone.first_lookup(dual)
            {
                continue;
            }
            if let Some(d) = self.zone.dual_in(f, p, &theta.set) {
                if budget == 0 {
                    // Another pair may still close the sequent without a
                    // copy.
                    cuts = Cuts::BUDGET;
                    continue;
                }
                let ax = self.push(Node::Ax(p.into(), d.into()));
                let copy = self.push(Node::Copy(d.into(), ax));
                if !affine {
                    return Ok(Found::proved(copy));
                }
                let mut rest = self.take_context_from(gamma);
                rest.remove(p);
                let node = self.weakened(&rest, copy);
                self.give_context(rest);
                return Ok(Found::proved(node));
            }
        }
        Ok(Found::failed(cuts))
    }

    /// Orders the focus candidates: `1` and `!`, then a `⊗` whose split is
    /// forced by a factor (a positive literal, `1`, `!`; or `0`, which
    /// fails at once), then `⊕`, then a `⊗` whose split must be searched.
    fn focus_class(&self, o: Z::Member) -> u8 {
        match self.kind(o) {
            Kind::One | Kind::Bang => 0,
            Kind::Tensor => {
                let forced = self
                    .problem
                    .forest
                    .children(self.occ(o))
                    .any(|c| self.forced_side(c).is_some());
                if forced { 1 } else { 3 }
            }
            _ => 2,
        }
    }

    /// The synchronous phase on `⊢ Θ ; Γ ⇓ F`, with `F` taken out of `Γ` or
    /// copied from `Θ`. Applies the positive rules along `F`, and releases
    /// `F` into the asynchronous phase once it is negative.
    fn focus(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        f: Z::Member,
        budget: u32,
    ) -> Step {
        self.enter()?;
        let result = self.focus_on(theta, gamma, f, budget);
        self.leave();
        result
    }

    /// The body of `focus`.
    fn focus_on(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        f: Z::Member,
        budget: u32,
    ) -> Step {
        match self.kind(f) {
            Kind::Plus => {
                let sides = [
                    Alternative::Branch(f, Branch::Left, self.left(f)),
                    Alternative::Branch(f, Branch::Right, self.right(f)),
                ];
                self.choose(theta, gamma, sides.into_iter(), budget)
            }
            Kind::Tensor => self.split(theta, gamma, f, budget),
            Kind::One if self.leftover(gamma) => {
                let one = self.push(Node::One(f.into()));
                Ok(Found::proved(self.weakened(gamma, one)))
            }
            Kind::One => Ok(Found::NOTHING),
            Kind::Bang => {
                // Promotion: the subformula is released into an empty
                // linear zone, and what was there is weakened below.
                if !self.leftover(gamma) {
                    return Ok(Found::NOTHING);
                }
                let mut released = self.take_context();
                let mut list = self.take_list();
                list.push(self.left(f));
                let result = self.asynchronous(theta, &mut released, &mut list, budget);
                self.give_context(released);
                self.give_list(list);
                Ok(result?.map(|node| {
                    let bang = self.push(Node::Bang(f.into(), node));
                    self.weakened(gamma, bang)
                }))
            }
            Kind::Zero => Ok(Found::NOTHING),
            Kind::Atom | Kind::DualAtom
                if self
                    .problem
                    .counts
                    .positive(self.problem.forest, self.occ(f)) =>
            {
                // The initial rules: the context is the dual literal, or
                // nothing and the dual lies in `Θ`.
                let mut members = self.take_list();
                members.push(f);
                gamma.list_into(&mut members);
                let mut whole = self.take_context_from(gamma);
                whole.insert(f);
                let result = self.initial(theta, &whole, &members, budget);
                self.give_list(members);
                self.give_context(whole);
                result
            }
            kind if Z::FIRST_ORDER && kind == Kind::Exists => self.exists(theta, gamma, f, budget),
            _ => {
                // Negative: release.
                let mut released = self.take_context_from(gamma);
                let mut list = self.take_list();
                list.push(f);
                let result = self.asynchronous(theta, &mut released, &mut list, budget);
                self.give_context(released);
                self.give_list(list);
                result
            }
        }
    }

    /// Whether a rule that needs an empty linear zone can be applied with
    /// this one: it is empty, or weakening will discard it.
    fn leftover(&self, gamma: &Z::Linear) -> bool {
        gamma.is_empty() || self.problem.rules.affine
    }

    /// Weakens every member of `gamma` below `node`, one `Weaken` per copy;
    /// nothing when it is empty.
    fn weakened(&mut self, gamma: &Z::Linear, mut node: NodeId) -> NodeId {
        debug_assert!(
            gamma.is_empty() || self.problem.rules.affine,
            "weakening needs affine mode"
        );
        debug_assert!(
            self.problem
                .reading
                .is_none_or(|r| r.outputs(gamma.iter().map(|m| self.occ(m))) == 0),
            "a leaf is the goal, so only hypotheses are left over"
        );
        for o in gamma.iter() {
            node = self.push(Node::Weaken(o.into(), node));
        }
        node
    }

    // Bookkeeping.

    /// Appends a node to the pending ones and returns its id.
    fn push(&mut self, node: Node) -> NodeId {
        self.nodes.push(node)
    }

    /// Enters a nested engine call, unless the nesting is at its limit.
    fn enter(&mut self) -> Result<(), Reason> {
        if self.depth >= self.problem.recursion_limit {
            return Err(Reason::RecursionLimit {
                depth: self.problem.recursion_limit,
            });
        }
        self.depth += 1;
        Ok(())
    }

    /// Leaves a nested engine call.
    fn leave(&mut self) {
        self.depth -= 1;
    }
}
