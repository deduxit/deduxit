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
//!   ([`alternate`](crate::linear::search::focus::schedule::threaded::alternate)).

use super::arena::{Arena, Kept};
use super::memo::{Interner, Memo, Table};
use super::zone::Zone;
use super::{Problem, Run, SetUp};
use crate::linear::Instances;
use crate::linear::fragment::{Fragment, Mode};
use crate::linear::occurrences::{Forest, OccId};
use crate::linear::proofs::{Node, NodeId};
use crate::linear::search::Bias;
use crate::linear::search::bias;
use crate::linear::search::counts::Counts;
use crate::linear::search::{Finished, Options, Reason, Statistics, Stop};
use crate::linear::sequents::Kind;
use crate::memory::Account;

/// The two searches alternating in slices on two threads, and the merge
/// of their answers that a pool shares.
#[cfg(feature = "parallel")]
pub(super) mod threaded;

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
                result: Err(backward),
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
    let same = bias::signs(forest, Bias::Factors) == bias::signs(forest, Bias::Rarer);
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
