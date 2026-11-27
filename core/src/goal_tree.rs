// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The walks over a proof in progress as a tree of inferences, which the
//! linear and the ordinary goal states share. A state keeps its inferences
//! top-down in one vector: the root first, every premise after its
//! conclusion, and each step's inferences appended at the end. These
//! functions read that tree through the premises of each inference alone,
//! so that each state keeps its own inference type. Every walk keeps a
//! stack of its own, since a proof may be higher than the call stack
//! allows.

use crate::linear::proofs::InfId;

/// Returns the inferences above `id`, excluding it, in no particular
/// order. `premises` gives each inference's premises.
pub(crate) fn subtree<'a>(premises: impl Fn(InfId) -> &'a [InfId], id: InfId) -> Vec<InfId> {
    let mut ids = premises(id).to_vec();
    let mut i = 0;
    while i < ids.len() {
        ids.extend_from_slice(premises(ids[i]));
        i += 1;
    }
    ids
}

/// Returns the inferences of the tree rooted at inference 0 in postorder,
/// premises before conclusions, of `len` inferences in all.
pub(crate) fn postorder<'a>(len: usize, premises: impl Fn(InfId) -> &'a [InfId]) -> Vec<InfId> {
    let mut order = Vec::with_capacity(len);
    // An entry is an inference and whether its premises were visited.
    let mut stack = vec![(InfId::new(0), false)];
    while let Some((id, visited)) = stack.pop() {
        if visited {
            order.push(id);
        } else {
            stack.push((id, true));
            stack.extend(premises(id).iter().rev().map(|&p| (p, false)));
        }
    }
    order
}

/// Returns, for every inference by its id, its place in `order`; `len`
/// is the number of inferences, and one `order` leaves out has none
/// (`u32::MAX`).
pub(crate) fn places(order: &[InfId], len: usize) -> Vec<InfId> {
    let mut place = vec![InfId::new(u32::MAX); len];
    for (i, id) in order.iter().enumerate() {
        // As many as the state's inferences, which a `u32` counts.
        place[id.index()] = InfId::new(i as u32);
    }
    place
}

/// Fails before a proof's derivation is built where its size, estimated
/// at `estimate` bytes, passes `limits.derivation_bytes`, or where `stop`
/// asks to stop.
///
/// # Errors
///
/// [`Refusal::Output`] past the bound, [`Refusal::Stopped`] when `stop`
/// asks.
///
/// [`Refusal::Output`]: crate::Refusal::Output
/// [`Refusal::Stopped`]: crate::Refusal::Stopped
pub(crate) fn may_build(
    estimate: u64,
    limits: &crate::Limits,
    stop: &mut impl FnMut(crate::Progress) -> bool,
) -> Result<(), crate::Error> {
    use crate::{Error, Phase, Progress, Refusal};
    if let Some(limit) = limits.derivation_bytes
        && estimate > limit
    {
        return Err(Error::Refused(Refusal::Output {
            what: "derivation",
            estimate_bytes: estimate,
            limit_bytes: limit,
            least_bytes: None,
        }));
    }
    if stop(Progress::new(Phase::View, 0, 0)) {
        return Err(Error::Refused(Refusal::Stopped { phase: Phase::View }));
    }
    Ok(())
}

/// What is wrong with a history of steps, as [`added`] finds it.
#[cfg(feature = "serialize")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Fault {
    /// A step names an inference that was not closed when it was taken, or
    /// one that another step names too.
    NotClosed,
    /// The inferences a step added are not the end of the vector it left.
    NotSuffix,
}

/// Returns, for each step of `history` in order, the range of the
/// inferences it added, or what is wrong with the history.
///
/// A step is the inference it closed. Walking from the last step back,
/// what a step added is its subtree as far as it lies below the end the
/// later steps left, and it must be exactly the end of the vector then.
/// `len` is how many inferences there are, `premises` gives each one's
/// premises, and `closed` says whether an inference has a rule. Each
/// inference must be the premise of at most one other, as the caller
/// checks first, so that the walk counts each once.
#[cfg(feature = "serialize")]
pub(crate) fn added<'a>(
    history: &[InfId],
    len: usize,
    premises: impl Fn(InfId) -> &'a [InfId],
    closed: impl Fn(InfId) -> bool,
) -> Result<Vec<(usize, usize)>, Fault> {
    let mut end = len;
    let mut named = vec![false; len];
    let mut ranges = Vec::with_capacity(history.len());
    for &goal in history.iter().rev() {
        let g = goal.index();
        // The goal lies below the end this step left: an entry inside a
        // later step's subtree would make an undo index past the vector.
        if g >= end || named[g] || !closed(goal) {
            return Err(Fault::NotClosed);
        }
        named[g] = true;
        // A premise has a larger index than its conclusion, so the walk
        // enters no inference a later step added.
        let (mut count, mut lowest) = (0, end);
        let mut walk = premises(goal).to_vec();
        while let Some(id) = walk.pop() {
            if id.index() >= end {
                continue;
            }
            count += 1;
            lowest = lowest.min(id.index());
            walk.extend_from_slice(premises(id));
        }
        // `count` distinct inferences below `end`, none below `start`, are
        // exactly `start..end`.
        let start = end - count;
        if lowest < start || g >= start {
            return Err(Fault::NotSuffix);
        }
        ranges.push((start, end));
        end = start;
    }
    ranges.reverse();
    Ok(ranges)
}

/// Returns the map of a graft's inference ids to the state's: a derivation
/// of `root + 1` inferences, premises before conclusions and its root
/// last, goes in place of the open goal `goal`, its root as the goal and
/// the rest appended in reverse from `base`, so that every premise keeps a
/// larger index than its conclusion.
pub(crate) fn grafted(goal: InfId, base: usize, root: usize) -> impl Fn(InfId) -> InfId {
    move |id: InfId| {
        if id.index() == root {
            goal
        } else {
            // Fewer inferences than a `u32` counts: the state's and the
            // graft's together are bounded by the derivation's size.
            InfId::new((base + root - 1 - id.index()) as u32)
        }
    }
}

#[cfg(all(test, feature = "serialize"))]
mod tests {
    use super::*;

    /// A tree as each inference's premises: 0 has 1 and 2, 2 has 3.
    fn tree() -> Vec<Vec<InfId>> {
        let id = InfId::new;
        vec![vec![id(1), id(2)], vec![], vec![id(3)], vec![]]
    }

    /// The walks read the tree as its premises give it, and the history
    /// walk names each step's suffix or the fault.
    #[test]
    fn the_walks_follow_the_premises() {
        let t = tree();
        let premises = |i: InfId| &t[i.index()][..];
        let mut above = subtree(premises, InfId::new(0));
        above.sort_unstable();
        assert_eq!(above, [1, 2, 3].map(InfId::new));
        assert_eq!(postorder(4, premises), [1, 3, 2, 0].map(InfId::new));
        let closed = |i: InfId| !t[i.index()].is_empty();
        let history = [InfId::new(0), InfId::new(2)];
        assert_eq!(
            added(&history, 4, premises, closed),
            Ok(vec![(1, 3), (3, 4)])
        );
        // A step that added inferences below a later step's.
        let id = InfId::new;
        let t2 = [vec![id(1), id(3)], vec![id(2)], vec![], vec![]];
        assert_eq!(
            added(
                &[id(0), id(1)],
                4,
                |i: InfId| &t2[i.index()][..],
                |i: InfId| !t2[i.index()].is_empty()
            ),
            Err(Fault::NotSuffix)
        );
        assert_eq!(
            added(&[InfId::new(1)], 4, premises, closed),
            Err(Fault::NotClosed)
        );
        let map = grafted(InfId::new(5), 10, 2);
        assert_eq!(
            [2, 1, 0].map(|k| map(InfId::new(k))),
            [5, 10, 11].map(InfId::new)
        );
    }
}
