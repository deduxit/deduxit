// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The indices of the sequents kept, which answer the questions that
//! subsumption asks. Is there a kept sequent with the same linear zone
//! and an unrestricted zone within the one given? Or, among those that
//! absorb or weaken, one whose two zones both lie within the ones given?
//!
//! An index is a trie. A path spells the sorted members of the linear
//! zone, repeats spelled out, then a separator, then the sorted members
//! of the unrestricted zone. A query walks only the paths whose members
//! the zones given hold, instead of every sequent kept.

use super::super::memory::bytes_of;
use crate::hash::HashMap;
use crate::linear::occurrences::OccId;

/// What separates the linear zone from the unrestricted one on a path. It
/// is no occurrence, since a forest has fewer than `u32::MAX` of them.
const SEPARATOR: OccId = OccId::new(u32::MAX);

/// A trie of pairs of sorted zones, each path ending in the ids of the
/// sequents it spells.
#[derive(Debug, Default)]
pub(super) struct Trie {
    /// The children of a node by the member that extends its path. The
    /// root is node 0.
    children: HashMap<(u32, OccId), u32>,
    /// The ids at a node: the sequents whose zones end there.
    ends: HashMap<u32, Vec<u32>>,
    /// The nodes made, the root left out.
    nodes: u32,
    /// The bytes of the lists in `ends`. The table's own capacity does not
    /// count them.
    lists: usize,
    /// The children looked up by the queries since [`Trie::lookups`] was
    /// last called. It is the work they did, whatever their answer.
    looked: std::cell::Cell<u64>,
}

impl Trie {
    /// Returns the child of a node by a member, making it where it is
    /// missing. Returns `None` when every `u32` names a node already.
    fn child(&mut self, node: u32, o: OccId) -> Option<u32> {
        if let Some(&child) = self.children.get(&(node, o)) {
            return Some(child);
        }
        self.nodes = self.nodes.checked_add(1)?;
        self.children.insert((node, o), self.nodes);
        Some(self.nodes)
    }

    /// Adds a sequent's zones, sorted, under its id.
    ///
    /// Returns false, without adding the id, when the trie has no name
    /// left for a node it needs. The nodes made before that stay. Only a
    /// search without a memory bound reaches this.
    pub(super) fn insert(&mut self, gamma: &[OccId], theta: &[OccId], id: u32) -> bool {
        let mut node = 0;
        for &o in gamma.iter().chain([&SEPARATOR]).chain(theta) {
            match self.child(node, o) {
                Some(child) => node = child,
                None => return false,
            }
        }
        let list = self.ends.entry(node).or_default();
        let before = bytes_of(list);
        list.push(id);
        self.lists += bytes_of(list) - before;
        true
    }

    /// Calls `each` on the nodes reached from `start` along a path whose
    /// members lie within the sorted `zone`, as a multiset, until `each`
    /// returns true. Returns whether it did.
    fn walk(&self, start: u32, zone: &[OccId], mut each: impl FnMut(u32) -> bool) -> bool {
        let mut stack = vec![(start, 0usize)];
        while let Some((node, from)) = stack.pop() {
            if each(node) {
                return true;
            }
            for j in from..zone.len() {
                // A repeat of the member before it would extend the same
                // paths. The path through the first copy goes on from here,
                // and can take the repeat too.
                if j > from && zone[j] == zone[j - 1] {
                    continue;
                }
                self.looked.set(self.looked.get() + 1);
                if let Some(&child) = self.children.get(&(node, zone[j])) {
                    stack.push((child, j + 1));
                }
            }
        }
        false
    }

    /// Whether `hit` is true of some sequent at the end of a path from
    /// `node` whose members lie within `theta`.
    fn ends_within(&self, node: u32, theta: &[OccId], hit: &mut impl FnMut(u32) -> bool) -> bool {
        self.walk(node, theta, |n| {
            self.ends
                .get(&n)
                .is_some_and(|ids| ids.iter().any(|&id| hit(id)))
        })
    }

    /// Whether `hit` is true of some sequent with the linear zone `gamma`
    /// and an unrestricted zone within `theta`.
    pub(super) fn same_within(
        &self,
        gamma: &[OccId],
        theta: &[OccId],
        mut hit: impl FnMut(u32) -> bool,
    ) -> bool {
        let mut node = 0;
        for &o in gamma.iter().chain([&SEPARATOR]) {
            self.looked.set(self.looked.get() + 1);
            match self.children.get(&(node, o)) {
                Some(&child) => node = child,
                None => return false,
            }
        }
        self.ends_within(node, theta, &mut hit)
    }

    /// Whether `hit` is true of some sequent whose two zones lie within
    /// `gamma` and `theta`.
    pub(super) fn within(
        &self,
        gamma: &[OccId],
        theta: &[OccId],
        mut hit: impl FnMut(u32) -> bool,
    ) -> bool {
        self.walk(0, gamma, |node| {
            self.children
                .get(&(node, SEPARATOR))
                .is_some_and(|&split| self.ends_within(split, theta, &mut hit))
        })
    }

    /// Returns how many children the queries looked up since the last
    /// call, and starts the count again.
    pub(super) fn lookups(&self) -> u64 {
        self.looked.take()
    }

    /// Returns the bytes the trie holds.
    pub(super) fn bytes(&self) -> usize {
        self.children.capacity() * (size_of::<((u32, OccId), u32)>() + 1)
            + self.ends.capacity() * (size_of::<(u32, Vec<u32>)>() + 1)
            + self.lists
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Within a zone, the trie finds the zones inserted that lie within it,
    /// and only those. A node past the last `u32` is refused, never
    /// wrapped onto the root.
    #[test]
    fn finds_zones_within_and_refuses_past_its_names() {
        let o = OccId::new;
        let mut trie = Trie::default();
        assert!(trie.insert(&[o(1), o(1), o(3)], &[o(7)], 0));
        assert!(trie.insert(&[o(2)], &[], 1));
        let found = |gamma: &[OccId], theta: &[OccId]| {
            let mut ids = Vec::new();
            trie.within(gamma, theta, |id| {
                ids.push(id);
                false
            });
            ids.sort_unstable();
            ids
        };
        assert_eq!(found(&[o(1), o(1), o(2), o(3)], &[o(7)]), vec![0, 1]);
        assert_eq!(
            found(&[o(1), o(2), o(3)], &[o(7)]),
            vec![1],
            "one 1 too few"
        );
        assert_eq!(found(&[o(1), o(1), o(3)], &[]), Vec::<u32>::new());
        assert!(trie.same_within(&[o(2)], &[o(5)], |_| true));
        assert!(!trie.same_within(&[o(1), o(1), o(3)], &[], |_| true));
        let mut full = Trie {
            nodes: u32::MAX - 1,
            ..Trie::default()
        };
        assert!(!full.insert(&[o(1)], &[], 2), "two nodes past the names");
        assert!(!full.within(&[o(1)], &[], |_| true));
    }
}
