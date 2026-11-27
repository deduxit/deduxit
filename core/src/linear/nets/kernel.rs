// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The graph searches both correctness criteria run: the bridges of a
//! graph in compressed sparse row layout by one iterative Tarjan search,
//! the deletion rounds of Yeo's theorem, and the isolation of one
//! switching cycle among the vertices the rounds leave.
//!
//! A structure without boxes (`graph`) and one with boxes
//! (`mell`) lay their graphs out alike: per vertex its slots, the
//! first its parent edge. They differ in which slots decide whether a
//! vertex may be deleted (`Rounds::deciding`), so the functions here are
//! generic over that and made once per structure.

use super::NONE;
use crate::linear::occurrences::{OccId, OccSet};
use std::ops::Range;

/// A graph in compressed sparse row layout.
pub(super) trait Csr {
    /// Per vertex, where its slots start in [`to`](Csr::to); one more
    /// entry holds the total.
    fn start(&self) -> &[u32];
    /// Per slot, the other end of the edge, or `NONE`.
    fn to(&self) -> &[u32];
}

/// A coloured graph whose switching cycles the deletion rounds find.
pub(super) trait Rounds: Csr {
    /// Returns the slots of `v` whose edges decide whether it may be
    /// deleted: of a switched vertex its parent edge's slot, or none where
    /// it has no parent edge; of any other vertex every slot. The vertex
    /// may go when each of those edges is absent or a bridge.
    fn deciding(&self, v: u32) -> Range<usize>;
}

/// Working memory for the correctness tests, sized for one structure, so
/// that a test allocates nothing.
///
/// [`ProofStructure::scratch`] makes one, and
/// [`ProofStructure::is_acyclic`] takes it.
///
/// [`ProofStructure::scratch`]: super::ProofStructure::scratch
/// [`ProofStructure::is_acyclic`]: super::ProofStructure::is_acyclic
#[derive(Clone, Debug)]
pub struct Scratch {
    /// The vertices the deletion procedure has removed.
    pub(super) deleted: OccSet,
    /// The vertices deleted when the procedure got stuck, kept while a
    /// cycle is being isolated among the others.
    stuck: OccSet,
    /// Per slot, whether the edge is taken out of the graph, which the
    /// witness searches use.
    pub(super) removed: Box<[bool]>,
    /// Per vertex, its discovery time in the last search, or `NONE`.
    disc: Box<[u32]>,
    /// Per vertex, the least discovery time it reaches through its
    /// search-tree descendants and one back edge.
    low: Box<[u32]>,
    /// Per vertex, its parent in the search tree, or `NONE` for a root.
    dfs_parent: Box<[u32]>,
    /// Per vertex, whether the edge to its search-tree parent is a bridge.
    bridge: Box<[bool]>,
    /// Per vertex, the root of the search tree that reached it.
    pub(super) component: Box<[u32]>,
    /// The search stack: a vertex and its next slot to look at.
    stack: Vec<(u32, u32)>,
}

impl Scratch {
    /// Returns working memory for a graph.
    pub(super) fn of(graph: &impl Csr) -> Self {
        let n = graph.start().len() - 1;
        Self {
            deleted: OccSet::empty(n),
            stuck: OccSet::empty(n),
            removed: vec![false; graph.to().len()].into_boxed_slice(),
            disc: vec![NONE; n].into_boxed_slice(),
            low: vec![NONE; n].into_boxed_slice(),
            dfs_parent: vec![NONE; n].into_boxed_slice(),
            bridge: vec![false; n].into_boxed_slice(),
            component: vec![NONE; n].into_boxed_slice(),
            stack: Vec::with_capacity(n),
        }
    }

    /// Returns whether the scratch is sized for `graph`.
    pub(super) fn fits(&self, graph: &impl Csr) -> bool {
        self.removed.len() == graph.to().len() && self.disc.len() == graph.start().len() - 1
    }

    /// Returns whether a vertex is deleted.
    fn is_deleted(&self, v: u32) -> bool {
        self.deleted.contains(OccId::new(v))
    }

    /// Makes every vertex present again.
    pub(super) fn restore(&mut self) {
        self.deleted.clear();
    }

    /// Takes a vertex out of the graph.
    pub(super) fn delete(&mut self, v: u32) {
        self.deleted.insert(OccId::new(v));
    }

    /// Returns whether the last search reached the vertex.
    pub(super) fn reached(&self, v: u32) -> bool {
        self.disc[v as usize] != NONE
    }

    /// Returns the root of the search tree that reached the vertex in the
    /// last search.
    pub(super) fn component(&self, v: u32) -> u32 {
        self.component[v as usize]
    }

    /// Returns whether the edge between two adjacent vertices is a bridge
    /// according to the last search, which must have reached them.
    pub(super) fn is_bridge(&self, v: u32, w: u32) -> bool {
        debug_assert!(
            self.reached(v) && self.reached(w),
            "the last search did not reach both ends"
        );
        (self.dfs_parent[w as usize] == v && self.bridge[w as usize])
            || (self.dfs_parent[v as usize] == w && self.bridge[v as usize])
    }
}

/// Returns the slots of a vertex.
pub(super) fn slots(graph: &impl Csr, v: u32) -> Range<usize> {
    let start = graph.start();
    start[v as usize] as usize..start[v as usize + 1] as usize
}

/// Returns the slot at `w` of the edge to `v`.
pub(super) fn twin(graph: &impl Csr, v: u32, w: u32) -> usize {
    slots(graph, w)
        .find(|&s| graph.to()[s] == v)
        .expect("the edge exists at both ends")
}

/// Searches depth-first from each of `roots` in turn, over the vertices
/// not deleted and the edges not removed. Returns the number of roots that
/// reached something new, which is the number of connected components
/// among the vertices reached.
///
/// For every vertex reached, the search records its discovery time, its
/// search-tree parent, the root that reached it, and whether its edge to
/// that parent is a bridge.
pub(super) fn search(
    graph: &impl Csr,
    scratch: &mut Scratch,
    roots: impl IntoIterator<Item = u32>,
) -> u32 {
    let (start, to) = (graph.start(), graph.to());
    debug_assert_eq!(
        scratch.removed.len(),
        to.len(),
        "a scratch of another graph"
    );
    scratch.disc.fill(NONE);
    let mut time = 0;
    let mut trees = 0;
    for r in roots {
        if scratch.is_deleted(r) || scratch.disc[r as usize] != NONE {
            continue;
        }
        trees += 1;
        discover(graph, scratch, r, NONE, r, &mut time);
        while let Some(&(v, next)) = scratch.stack.last() {
            if next < start[v as usize + 1] {
                scratch.stack.last_mut().unwrap().1 = next + 1;
                let w = to[next as usize];
                if w == NONE
                    || scratch.removed[next as usize]
                    || scratch.is_deleted(w)
                    || w == scratch.dfs_parent[v as usize]
                {
                    continue;
                }
                if scratch.disc[w as usize] == NONE {
                    discover(graph, scratch, w, v, r, &mut time);
                } else {
                    let (lv, dw) = (scratch.low[v as usize], scratch.disc[w as usize]);
                    scratch.low[v as usize] = lv.min(dw);
                }
            } else {
                scratch.stack.pop();
                let p = scratch.dfs_parent[v as usize];
                if p != NONE {
                    let (lp, lv) = (scratch.low[p as usize], scratch.low[v as usize]);
                    scratch.low[p as usize] = lp.min(lv);
                    scratch.bridge[v as usize] = lv > scratch.disc[p as usize];
                }
            }
        }
    }
    trees
}

/// Searches from every vertex in turn ([`search`]). Kept out of line: the
/// deletion rounds call it once a round, and inlined there it made the
/// net engine's exact test about 1 % dearer (`search-wide-m2-256`).
#[inline(never)]
fn search_every(graph: &impl Csr, scratch: &mut Scratch) -> u32 {
    let n = graph.start().len() - 1;
    search(graph, scratch, 0..n as u32)
}

/// Enters a vertex in the search: from `parent`, in the tree of `root`.
fn discover(
    graph: &impl Csr,
    scratch: &mut Scratch,
    v: u32,
    parent: u32,
    root: u32,
    time: &mut u32,
) {
    scratch.disc[v as usize] = *time;
    scratch.low[v as usize] = *time;
    *time += 1;
    scratch.dfs_parent[v as usize] = parent;
    scratch.component[v as usize] = root;
    scratch.bridge[v as usize] = false;
    scratch.stack.push((v, graph.start()[v as usize]));
}

/// Returns whether a vertex may be deleted according to the last search:
/// every edge that decides it is absent or a bridge.
fn deletable(graph: &impl Rounds, scratch: &Scratch, v: u32) -> bool {
    let to = graph.to();
    graph.deciding(v).all(|s| {
        let w = to[s];
        let present = w != NONE && !scratch.removed[s] && !scratch.is_deleted(w);
        !present || scratch.is_bridge(v, w)
    })
}

/// Runs the deletion procedure on the vertices not yet deleted: in rounds,
/// deletes every deletable vertex until none is. Returns whether every
/// vertex went, which means the graph has no switching cycle. Otherwise
/// the vertices left in place are those the procedure got stuck on, and a
/// switching cycle runs among them. `None` when `stop` fired.
///
/// It asks `stop` before each round. `stop` is a `dyn`, asked once a
/// round: a generic function would be made in its caller's codegen unit,
/// where the searches and the deletion test, which run per vertex, could
/// not be inlined into it.
pub(super) fn acyclic(
    graph: &impl Rounds,
    scratch: &mut Scratch,
    stop: &mut dyn FnMut(u64) -> bool,
) -> Option<bool> {
    let n = graph.start().len() - 1;
    let mut remaining = n - scratch.deleted.len();
    while remaining > 0 {
        if stop(1) {
            return None;
        }
        search_every(graph, scratch);
        let before = remaining;
        for v in 0..n as u32 {
            if !scratch.is_deleted(v) && deletable(graph, scratch, v) {
                scratch.delete(v);
                remaining -= 1;
            }
        }
        if remaining == before {
            return Some(false);
        }
    }
    Some(true)
}

/// Isolates one switching cycle among the vertices that [`acyclic`] got
/// stuck on, in the call just before, and returns its vertices in order
/// along the cycle. `None` when `stop`, which every deletion procedure
/// asks, fired. Either way no edge is left removed.
///
/// Every edge among those vertices is taken out in turn, and kept out when
/// a switching cycle survives without it. An edge found necessary stays
/// necessary as edges go, so one pass leaves exactly one cycle.
pub(super) fn cycle(
    graph: &impl Rounds,
    scratch: &mut Scratch,
    stop: &mut dyn FnMut(u64) -> bool,
) -> Option<Vec<u32>> {
    let n = graph.start().len() - 1;
    let to = graph.to();
    scratch.stuck.clone_from(&scratch.deleted);
    for v in 0..n as u32 {
        for s in slots(graph, v) {
            let w = to[s];
            if w == NONE
                || w < v
                || scratch.removed[s]
                || scratch.stuck.contains(OccId::new(v))
                || scratch.stuck.contains(OccId::new(w))
            {
                continue;
            }
            let t = twin(graph, v, w);
            scratch.removed[s] = true;
            scratch.removed[t] = true;
            scratch.deleted.clone_from(&scratch.stuck);
            let Some(acyclic) = acyclic(graph, scratch, stop) else {
                scratch.removed.fill(false);
                return None;
            };
            if acyclic {
                scratch.removed[s] = false;
                scratch.removed[t] = false;
            } else {
                scratch.stuck.clone_from(&scratch.deleted);
            }
        }
    }
    // What is left is one cycle: walk it from its first vertex.
    let first = (0..n as u32)
        .find(|&v| !scratch.stuck.contains(OccId::new(v)))
        .expect("a switching cycle is left");
    let mut cycle = Vec::new();
    let (mut previous, mut current) = (NONE, first);
    loop {
        cycle.push(current);
        let next = slots(graph, current)
            .map(|s| (s, to[s]))
            .find(|&(s, w)| {
                w != NONE
                    && w != previous
                    && !scratch.removed[s]
                    && !scratch.stuck.contains(OccId::new(w))
            })
            .map(|(_, w)| w)
            .expect("every vertex of the cycle has two edges on it");
        if next == first {
            break;
        }
        (previous, current) = (current, next);
    }
    scratch.removed.fill(false);
    Some(cycle)
}
