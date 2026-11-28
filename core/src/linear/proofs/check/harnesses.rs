// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Kani's harnesses for the integers the checker's soundness rests on.
//!
//! The checker's argument that no integer wraps (see the module docs) is
//! made of small facts about its counters, sums, bounds and indices. Each
//! harness states one of them and checks it for every input within its
//! bound, bit by bit: Kani also fails a harness on any overflow, underflow
//! or panic along the way. A bound is the size of what a harness builds
//! (members, counts of distinct members, nodes); the values in it (counts,
//! weights, lengths, indices) range over their whole type unless the
//! harness says otherwise.
//!
//! The model checker runs them (`cargo kani -p deduxit`, the flake's
//! package `kani-harnesses`); no other build compiles this module.

use super::pass::room;
use super::zones::{Bag, Eigens, State, Zone, table_bytes};
use crate::hash::HashMap;
use crate::linear::occurrences::{Forest, Member};
use crate::linear::proofs::{Node, NodeId, Proof};
use crate::linear::sequents::{Eigen, Sequent};

/// The most distinct members any table holds: every member is a `u32`.
const MEMBERS: u64 = 1 << 32;

/// A table's bytes never overflow and stay below 2³⁸, for every count of
/// members a table can have held (up to 2³²) and every entry the checker
/// keeps: a member, a member with its count, an eigenvariable. So a
/// sequent's two tables with the value take fewer than 2³⁹ bytes, and
/// the pass's `u64` count of what it holds adds them without wrapping.
///
/// Bound: none; every `most` up to 2³².
#[kani::proof]
fn table_bytes_stay_below_two_to_the_38() {
    let most: usize = kani::any();
    kani::assume(most as u64 <= MEMBERS);
    for entry in [
        size_of::<Member>(),
        size_of::<(Member, u32)>(),
        size_of::<Eigen>(),
    ] {
        let bytes = table_bytes(most, entry);
        assert!(bytes < 1 << 38);
        // The table holds its entries, at least.
        assert!(bytes >= most as u64 * entry as u64);
    }
}

/// A sequent's bytes, as the pass counts them, stay below 2³⁹ whatever
/// its tables held, and an eigenvariable set's below 2³⁸ plus its box.
///
/// Bound: none; every `most` of either table up to 2³² (the tables
/// themselves empty, since only `most` enters the count).
#[kani::proof]
fn a_sequent_takes_fewer_than_two_to_the_39_bytes() {
    let theta: usize = kani::any();
    let gamma: usize = kani::any();
    let eigens: usize = kani::any();
    kani::assume(theta as u64 <= MEMBERS && gamma as u64 <= MEMBERS);
    kani::assume(eigens as u64 <= MEMBERS);
    let state = State {
        theta: Zone {
            most: theta,
            ..Zone::default()
        },
        gamma: Bag {
            most: gamma,
            ..Bag::default()
        },
        outputs: 0,
        linear: 0,
        unrestricted: 0,
        goal: 0,
        any: false,
    };
    assert!(state.bytes() < 1 << 39);
    let set = Eigens {
        most: eigens,
        ..Eigens::default()
    };
    assert!(set.bytes() < (1 << 38) + 1024);
}

/// The bound `within` holds a zone to is `goal + 2 · later` capped at
/// [`Bag::MOST`], and computing it never overflows. `Bag::MOST` is below
/// `u32::MAX`, the value a member's count saturates at, so a zone the
/// pass lets through has no saturated count.
///
/// Bound: none; every `goal` and `later`.
#[kani::proof]
fn the_zone_bound_is_capped_below_a_saturated_count() {
    let goal: usize = kani::any();
    let later: usize = kani::any();
    let bound = room(goal, later);
    let wanted = (goal as u128 + 2 * later as u128).min(Bag::MOST as u128);
    assert!(bound as u128 == wanted);
    assert!(Bag::MOST < u32::MAX as usize);
}

/// The members a harness's zones are made of.
const DOMAIN: usize = 2;

/// Which of the members a zone holds. A harness fixes it, so that the
/// tables' keys are constants and only the counts are free: with a
/// symbolic key every hash and probe of the table is symbolic, and one
/// harness took the model checker more than ten minutes.
type Shape = [bool; DOMAIN];

/// A zone of the first member.
const FIRST: Shape = [true, false];

/// A zone of the second member.
const SECOND: Shape = [false, true];

/// What an observer says of the members: each one's weight and whether it
/// is in output position, the same at every node.
struct Members {
    /// Each member's weight.
    weights: [u32; DOMAIN],
    /// Whether each member is in output position.
    outputs: [bool; DOMAIN],
}

/// The sums of a sequent's linear zone, without a bound: its outputs, its
/// weight and the weight of its members in output position.
type Sums = [u128; 3];

impl Members {
    /// Returns any weights and positions.
    fn any() -> Self {
        Self {
            weights: kani::any(),
            outputs: kani::any(),
        }
    }

    /// Returns any counts of the members `shape` holds, one copy at least
    /// each, and none of the others.
    fn counts(shape: Shape) -> [u32; DOMAIN] {
        let counts: [u32; DOMAIN] = kani::any();
        for k in 0..DOMAIN {
            kani::assume((counts[k] > 0) == shape[k]);
        }
        counts
    }

    /// Returns a sequent that passed `within`: its linear zone of `shape`
    /// holds `counts[k]` copies of member `k`, of a length within
    /// [`Bag::MOST`], the table's `most` is what it holds, the outputs are
    /// counted exactly, and the two weights are any that exact sums of
    /// these counts meet: each member held weighs in once at least, in
    /// the goal's weight too where it is an output; the members weigh
    /// less than 2³² each, so a weight is at most its members times 2³²
    /// (a shift, which the solver takes more easily than a product); and
    /// the goal's weight is part of the whole. Exact
    /// sums are products of counts and weights, which a solver handles
    /// badly; these facts are all the pass's sums rest on.
    fn state(&self, shape: Shape, counts: [u32; DOMAIN]) -> State {
        let mut table = HashMap::default();
        let mut len: u64 = 0;
        let mut outputs: u64 = 0;
        for k in 0..DOMAIN {
            if shape[k] {
                table.insert(Member::new(k as u32), counts[k]);
                len += u64::from(counts[k]);
                if self.outputs[k] {
                    outputs += u64::from(counts[k]);
                }
            }
        }
        kani::assume(len <= Bag::MOST as u64);
        let (linear, goal): (u64, u64) = (kani::any(), kani::any());
        kani::assume(u128::from(goal) <= u128::from(outputs) << 32);
        kani::assume(u128::from(linear - goal.min(linear)) <= u128::from(len - outputs) << 32);
        kani::assume(goal <= linear);
        for k in 0..DOMAIN {
            if shape[k] {
                let weight = u64::from(self.weights[k]);
                if self.outputs[k] {
                    kani::assume(goal >= weight);
                } else {
                    kani::assume(linear - goal >= weight);
                }
            }
        }
        State {
            theta: Zone::default(),
            gamma: Bag {
                most: table.len(),
                counts: table,
                len: len as usize,
            },
            outputs: outputs as usize,
            linear,
            unrestricted: 0,
            goal,
            any: kani::any(),
        }
    }

    /// Returns the sums `state` holds, widened.
    fn sums(state: &State) -> Sums {
        [
            state.outputs as u128,
            u128::from(state.linear),
            u128::from(state.goal),
        ]
    }

    /// Returns the sums of one copy of member `k`.
    fn one(&self, k: usize) -> Sums {
        let weight = u128::from(self.weights[k]);
        if self.outputs[k] {
            [1, weight, weight]
        } else {
            [0, weight, 0]
        }
    }

    /// Asserts that the sequent's linear zone holds exactly `counts[k]`
    /// copies of every member `k`, with the sums `wanted`, unless its
    /// length is over [`Bag::MOST`], where the pass refuses it before
    /// anything reads it; and in every case that no count in the table is
    /// zero or over the length, and that the table's `most` is no less
    /// than what it holds.
    fn assert_exact_or_refused(state: &State, counts: [u64; DOMAIN], wanted: Sums) {
        let bag = &state.gamma;
        assert!(bag.most >= bag.counts.len());
        for (_, n) in bag.counts() {
            assert!(n > 0);
            assert!(n as usize <= bag.len);
        }
        if bag.len <= Bag::MOST {
            for k in 0..DOMAIN {
                let held = bag.counts.get(&Member::new(k as u32)).copied();
                assert!(u64::from(held.unwrap_or(0)) == counts[k]);
            }
            assert!(bag.len as u64 == counts.iter().sum::<u64>());
            let sums = Self::sums(state);
            for i in 0..3 {
                assert!(sums[i] == wanted[i]);
            }
            let [outputs, linear, goal] = wanted;
            assert!(goal <= linear && outputs <= bag.len as u128);
        }
    }

    /// Puts member `k` into a zone of `shape` and asserts the result.
    fn put(shape: Shape, k: usize) {
        let members = Self::any();
        let counts = Self::counts(shape);
        let mut state = members.state(shape, counts);
        let before = (state.gamma.len, Self::sums(&state));
        state.put(
            Member::new(k as u32),
            u64::from(members.weights[k]),
            members.outputs[k],
        );
        let mut wanted = counts.map(u64::from);
        wanted[k] += 1;
        let one = members.one(k);
        assert!(state.gamma.len >= before.0);
        let sums = std::array::from_fn(|i| before.1[i] + one[i]);
        Self::assert_exact_or_refused(&state, wanted, sums);
    }

    /// Takes member `k` from a zone of `shape` and asserts the result.
    fn take(shape: Shape, k: usize) {
        let members = Self::any();
        let counts = Self::counts(shape);
        let mut state = members.state(shape, counts);
        let before = Self::sums(&state);
        let taken = state.remove(Member::new(k as u32), || {
            (u64::from(members.weights[k]), members.outputs[k])
        });
        assert!(taken == shape[k]);
        let mut wanted = counts.map(u64::from);
        let mut sums = before;
        if taken {
            wanted[k] -= 1;
            let one = members.one(k);
            sums = std::array::from_fn(|i| before[i] - one[i]);
        }
        Self::assert_exact_or_refused(&state, wanted, sums);
    }

    /// Joins zones of `left` and `right` as `⊗` does and asserts the
    /// result.
    fn join(left: Shape, right: Shape) {
        let members = Self::any();
        let (l, r) = (Self::counts(left), Self::counts(right));
        let mut state = members.state(left, l);
        let mut other = members.state(right, r);
        let lengths = (state.gamma.len, other.gamma.len);
        let (ours, theirs) = (Self::sums(&state), Self::sums(&other));
        let any = state.any || other.any;
        state.add_linear(&mut other);
        let wanted = std::array::from_fn(|k| u64::from(l[k]) + u64::from(r[k]));
        assert!(state.gamma.len >= lengths.0 && state.gamma.len >= lengths.1);
        assert!(state.any == any);
        let sums = std::array::from_fn(|i| ours[i] + theirs[i]);
        Self::assert_exact_or_refused(&state, wanted, sums);
    }

    /// Unites zones of `left` and `right` as `&` does when both premises
    /// absorb, counts the sums anew from the weights, and asserts the
    /// result: the products of counts and weights fit, and their sums are
    /// exact within the bound.
    fn unite(left: Shape, right: Shape) {
        let members = Self::any();
        let (l, r) = (Self::counts(left), Self::counts(right));
        let mut state = members.state(left, l);
        let other = members.state(right, r);
        let lengths = (state.gamma.len, other.gamma.len);
        state.gamma.unite(other.gamma);
        state.recount(|m| {
            let k = m.index();
            (members.weights[k], members.outputs[k])
        });
        let wanted: [u64; DOMAIN] = std::array::from_fn(|k| u64::from(l[k].max(r[k])));
        let mut sums = [0u128; 3];
        for k in 0..DOMAIN {
            // The same product the pass takes, so that the solver need not
            // multiply twice.
            let weight = u64::from(members.weights[k]) * u64::from(l[k].max(r[k]));
            sums[1] += u128::from(weight);
            if members.outputs[k] {
                sums[0] += u128::from(wanted[k]);
                sums[2] += u128::from(weight);
            }
        }
        assert!(state.gamma.len >= lengths.0 && state.gamma.len >= lengths.1);
        Self::assert_exact_or_refused(&state, wanted, sums);
    }

    /// Returns any weight of members of an unrestricted zone besides the
    /// harness's own: at most 2³² − [`DOMAIN`] of them, each weighing
    /// less than 2³², so at most their number times 2³² (a shift).
    fn rest() -> u64 {
        let others: u64 = kani::any();
        kani::assume(others <= MEMBERS - DOMAIN as u64);
        let rest: u64 = kani::any();
        kani::assume(u128::from(rest) <= u128::from(others) << 32);
        rest
    }

    /// Returns a sequent whose unrestricted zone holds the members of
    /// `shape` and others weighing `rest`, the sum being the sum over the
    /// set, as it is at every moment. The others are in the sum only, not
    /// in the table, whose `most` is what it holds; the linear zone is
    /// empty.
    fn unrestricted(&self, shape: Shape, rest: u64) -> State {
        let mut theta = Zone::default();
        let mut sum = rest;
        for k in 0..DOMAIN {
            if shape[k] {
                theta.insert(Member::new(k as u32));
                sum += u64::from(self.weights[k]);
            }
        }
        State {
            theta,
            gamma: Bag::default(),
            outputs: 0,
            linear: 0,
            unrestricted: sum,
            goal: 0,
            any: false,
        }
    }

    /// Asserts that the sequent's unrestricted zone holds exactly the
    /// members of `shape`, with the weight `rest` of the others besides,
    /// and that its table's `most` is no less than what it holds.
    fn assert_unrestricted(&self, state: &State, shape: Shape, rest: u64) {
        assert!(state.theta.most >= state.theta.len());
        let mut sum = u128::from(rest);
        for k in 0..DOMAIN {
            assert!(state.theta.members.contains(&Member::new(k as u32)) == shape[k]);
            if shape[k] {
                sum += u128::from(self.weights[k]);
            }
        }
        assert!(u128::from(state.unrestricted) == sum);
    }

    /// Copies member `k` into an unrestricted zone of `shape` and asserts
    /// the result.
    fn copy(shape: Shape, k: usize) {
        let members = Self::any();
        let rest = Self::rest();
        let mut state = members.unrestricted(shape, rest);
        let added = state.add_unrestricted(Member::new(k as u32), || u64::from(members.weights[k]));
        assert!(added != shape[k]);
        let mut wanted = shape;
        wanted[k] = true;
        members.assert_unrestricted(&state, wanted, rest);
    }

    /// Discharges member `k` from an unrestricted zone of `shape` as a `?`
    /// step does and asserts the result.
    fn quest(shape: Shape, k: usize) {
        let members = Self::any();
        let rest = Self::rest();
        let mut state = members.unrestricted(shape, rest);
        let removed =
            state.remove_unrestricted(Member::new(k as u32), || u64::from(members.weights[k]));
        assert!(removed == shape[k]);
        let mut wanted = shape;
        wanted[k] = false;
        members.assert_unrestricted(&state, wanted, rest);
    }

    /// Unites the unrestricted zones of `left` and `right` as `&` does and
    /// asserts the result: the union, its sum, the members both held noted
    /// once each, and `right`'s zone emptied. The larger zone (`left` when
    /// they are as large) has others besides: it is the one not poured,
    /// whose members the harness need not hold in its table.
    fn unite_unrestricted(left: Shape, right: Shape) {
        let members = Self::any();
        let rest = Self::rest();
        let size = |shape: Shape| shape.iter().filter(|&&held| held).count();
        let (ours, theirs) = if size(right) > size(left) {
            (0, rest)
        } else {
            (rest, 0)
        };
        let mut state = members.unrestricted(left, ours);
        let mut other = members.unrestricted(right, theirs);
        let mut shared = Vec::new();
        state.unite_unrestricted(
            &mut other,
            |m| u64::from(members.weights[m.index()]),
            &mut shared,
        );
        let wanted: Shape = std::array::from_fn(|k| left[k] || right[k]);
        members.assert_unrestricted(&state, wanted, rest);
        assert!(other.theta.is_empty());
        let both = (0..DOMAIN).filter(|&k| left[k] && right[k]).count();
        assert!(shared.len() == both);
        for m in shared {
            assert!(left[m.index()] && right[m.index()]);
        }
    }
}

/// Putting a copy of a member a zone holds keeps its counts and sums
/// exact, or makes the zone longer than [`Bag::MOST`], where the pass
/// refuses it: a count or a sum saturates only there. It never makes a
/// zone shorter.
///
/// Bound: a zone of one member; any count, weight and position.
#[kani::proof]
#[kani::unwind(5)]
fn a_copy_put_is_counted_or_refused() {
    Members::put(FIRST, 0);
}

/// As [`a_copy_put_is_counted_or_refused`], for a member the zone lacks.
///
/// Bound: a zone of one member and another put; any counts, weights and
/// positions.
#[kani::proof]
#[kani::unwind(5)]
fn a_new_member_put_is_counted_or_refused() {
    Members::put(FIRST, 1);
}

/// Taking a copy of a member from a zone the pass let through takes one,
/// subtracts its weight from exact sums without going below zero, and
/// leaves the counts and sums exact.
///
/// Bound: a zone of one member; any count, weight and position.
#[kani::proof]
#[kani::unwind(5)]
fn a_copy_taken_is_counted() {
    Members::take(FIRST, 0);
}

/// Taking a member a zone lacks takes nothing and changes nothing.
///
/// Bound: a zone of one member and another taken; any counts, weights and
/// positions.
#[kani::proof]
#[kani::unwind(5)]
fn a_member_missing_is_not_taken() {
    Members::take(FIRST, 1);
}

/// The sum of two zones the pass let through, as `⊗`, Mix and the cut
/// make it, has the true counts and sums, or is longer than [`Bag::MOST`]:
/// the sum of two counts or two weights saturates only in a zone the pass
/// refuses. It is no shorter than either, and absorbs when either does.
///
/// Bound: two zones of the same one member; any counts, weights and
/// positions. Tables of one member each are not swapped before the sum
/// (`Bag::add` pours the smaller into the larger), so that path is left
/// out: two-member zones took the model checker past ten minutes.
#[kani::proof]
#[kani::unwind(5)]
fn the_sum_of_two_zones_is_counted_or_refused() {
    Members::join(FIRST, FIRST);
}

/// As [`the_sum_of_two_zones_is_counted_or_refused`], for zones of two
/// different members, so that the sum adds a member to the table.
///
/// Bound: zones of one member each; any counts, weights and positions.
#[kani::proof]
#[kani::unwind(5)]
fn the_sum_of_two_zones_apart_is_counted_or_refused() {
    Members::join(FIRST, SECOND);
}

/// The union of two zones the pass let through, as `&` makes it when both
/// premises absorb, and its sums counted anew, are exact, or the zone is
/// longer than [`Bag::MOST`]. A count times a weight fits a `u64`.
///
/// Bound: two zones of the same one member; any counts, weights and
/// positions. As for the sum, the tables are not swapped first; zones of
/// two different members took the model checker past twenty minutes.
#[kani::proof]
#[kani::unwind(5)]
fn the_union_of_two_zones_is_counted_or_refused() {
    Members::unite(FIRST, FIRST);
}

/// Copying a member into the unrestricted zone adds its weight to the
/// sum once, when it is new, and never overflows: the zone holds at most
/// 2³² members of weights below 2³², so the sum stays below 2⁶⁴.
///
/// Bound: a zone of one member and others weighing any sum they can,
/// the new member copied; any weights.
#[kani::proof]
#[kani::unwind(5)]
fn a_new_copy_adds_its_weight() {
    Members::copy(FIRST, 1);
}

/// As [`a_new_copy_adds_its_weight`], for a member the zone holds: the
/// zone and its sum stay as they were.
///
/// Bound: a zone of one member and others, that member copied again; any
/// weights.
#[kani::proof]
#[kani::unwind(5)]
fn a_repeated_copy_adds_nothing() {
    Members::copy(FIRST, 0);
}

/// A `?` step that discharges a member of the unrestricted zone takes its
/// weight off the sum, which counted it, so the sum does not go below
/// zero.
///
/// Bound: a zone of one member and others, that member discharged; any
/// weights.
#[kani::proof]
#[kani::unwind(5)]
fn a_discharged_member_takes_its_weight() {
    Members::quest(FIRST, 0);
}

/// A `?` step whose member the zone lacks changes nothing.
///
/// Bound: a zone of one member and others, another member discharged;
/// any weights.
#[kani::proof]
#[kani::unwind(5)]
fn a_member_unused_is_not_discharged() {
    Members::quest(FIRST, 1);
}

/// The union of two unrestricted zones, as `&` makes it, holds both sets,
/// counts each member's weight once, notes each member both held, and
/// never overflows.
///
/// Bound: two zones of the same one member, the first with others
/// besides; any weights. Zones of equal size are not swapped first, so
/// that path is left out: a first zone of one member and a second of both
/// ran out of 8 GB, and an empty first zone makes the model checker drain
/// a table never allocated, whose layout it cannot compute.
#[kani::proof]
#[kani::unwind(5)]
fn the_union_of_two_unrestricted_zones_is_counted() {
    Members::unite_unrestricted(FIRST, FIRST);
}

/// As [`the_union_of_two_unrestricted_zones_is_counted`], for zones of
/// two different members, so that the union adds a member.
///
/// Bound: zones of one member each, the first with others besides; any
/// weights.
#[kani::proof]
#[kani::unwind(5)]
fn the_union_of_two_unrestricted_zones_apart_is_counted() {
    Members::unite_unrestricted(FIRST, SECOND);
}

/// The nodes a harness's arenas hold.
const NODES: u32 = 3;

/// Returns any index of a node up to one past the last.
fn any_index() -> NodeId {
    NodeId::new(kani::any_where(|&p: &u32| p <= NODES))
}

/// A proof that [`Proof::new`] keeps has between one node and as many as
/// it was given, every premise before its node and the root last, and
/// every node but the root read by a later one. So node `i` of `n` has at
/// most `n − i` readers, which is what lets the checker count readers in
/// a `u32` (a proof has fewer than 2³² nodes) and compute
/// `n − 1 − i` for `within` without going below zero.
///
/// It calls `Proof::reach`, the part of `Proof::new` that keeps and
/// orders the nodes, with an empty forest and a bound of one member:
/// laying out even a forest of one formula took the model checker past
/// ten minutes, and the forest only bounds the members, which the kept
/// order does not depend on.
///
/// Bound: the arena `1`, Mix, Mix with any premises up to one past the
/// end and the root last, which holds shared premises, premises out of
/// order or range, and nodes the root does not reach. A free root, nodes
/// of any kind or a fourth node ran out of 8 GB: a free root makes the
/// length of every table `Proof::reach` keeps free, and the kinds matter
/// only for the premises, which Mix has the most of.
#[kani::proof]
#[kani::unwind(5)]
fn a_kept_proof_is_ordered_and_reached() {
    // Neither `expect` nor a drop of the error: formatting or dropping the
    // crate's error takes the model checker through every variant.
    let forest = match Forest::new(&Sequent::new()) {
        Ok(forest) => forest,
        Err(error) => {
            std::mem::forget(error);
            unreachable!()
        }
    };
    let leaf = Node::One(Member::new(0));
    let nodes = vec![
        leaf,
        Node::Mix(any_index(), any_index()),
        Node::Mix(any_index(), any_index()),
    ];
    let len = nodes.len();
    let root = NodeId::new(NODES - 1);
    let proof = match Proof::reach(forest, nodes, root, 1, &mut || false) {
        Ok(proof) => proof,
        Err(error) => return std::mem::forget(error),
    };
    let kept = proof.nodes();
    assert!(!kept.is_empty() && kept.len() <= len);
    assert!(proof.root().index() == kept.len() - 1);
    let mut readers = [0u32; NODES as usize];
    for (i, node) in kept.iter().enumerate() {
        for p in node.premises() {
            assert!(p.index() < i);
            readers[p.index()] += 1;
        }
    }
    for (i, &count) in readers.iter().enumerate().take(kept.len()) {
        let later = kept.len() - 1 - i;
        assert!(count as usize <= later + 1);
        assert!(later == 0 || count >= 1);
    }
    std::mem::forget(proof);
}
