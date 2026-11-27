// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The coloured structure graph of a proof structure and the tests the
//! correctness criterion runs on it.
//!
//! The vertices are the occurrences. The edges are the premise edges of
//! every `⊗` and `⅋`, the cut links and the axiom links. A cut link takes
//! the place of the parent edge at both its ends, whose parents are no
//! vertices. An occurrence that is no vertex of the structure (one of a
//! cut formula's tree above no cut link's end) has no slot at all.
//!
//! The colouring: the two premise edges of a `⅋` share a colour of their
//! own, and every other edge has a colour of its own. So a cycle is
//! *properly coloured* (no two consecutive edges alike) exactly when it
//! never enters and leaves a `⅋` through its two premises, that is, when
//! some switching keeps it. A switching cycle is a properly coloured
//! cycle. Unlinked literals are leaves and lie on no cycle, so the test
//! means the same on a partial structure.
//!
//! By Yeo's theorem, a graph without a properly coloured cycle has a
//! vertex that every component of the graph without it meets in edges of
//! one colour only. Such a vertex lies on no properly coloured cycle, and
//! deleting it keeps the property. Repeating the deletion until it stops
//! decides the question exactly: everything goes, or a properly coloured
//! cycle is left.
//!
//! With this colouring the deletion condition needs no colours. A vertex
//! that is not a `⅋` has an edge of its own colour to every neighbour, so
//! it is deletable when every incident edge is a bridge, an edge whose
//! removal disconnects its two ends. A `⅋` is deletable when its parent
//! edge is a bridge or absent, since its premise edges share a colour.
//! One depth-first search per round finds the bridges.

use super::kernel::{self, Csr, Rounds, Scratch};
use super::{NONE, VertexId};
use crate::linear::occurrences::{Forest, OccId, OccSet};
use crate::linear::sequents::Kind;
use std::ops::Range;

/// The coloured structure graph in compressed sparse row layout: per
/// vertex, its edge to its parent or across its cut link first, then to
/// its children, and for a literal its axiom link last, with `NONE` while
/// it is unlinked.
#[derive(Clone, Debug)]
pub(super) struct Graph {
    /// Per vertex, where its slots start in `to`; one more entry holds the
    /// total. A vertex has three slots at most and a structure fewer than
    /// `u32::MAX / 3` vertices (`super::MOST`), so the total fits.
    start: Box<[u32]>,
    /// Per slot, the other end of the edge.
    to: Box<[u32]>,
    /// The edges every switching keeps: two per `⊗`, one per `⅋` and one
    /// per cut link.
    switched_tree_edges: usize,
    /// The vertices with slots: every occurrence but those of a cut
    /// formula's tree above no cut link's end.
    vertices: usize,
}

impl Csr for Graph {
    fn start(&self) -> &[u32] {
        &self.start
    }

    fn to(&self) -> &[u32] {
        &self.to
    }
}

/// The graph with the forest that says which vertices are `⅋`s, as the
/// deletion rounds read it.
struct Switched<'a> {
    /// The graph.
    graph: &'a Graph,
    /// The forest of its vertices.
    forest: &'a Forest,
}

impl Csr for Switched<'_> {
    fn start(&self) -> &[u32] {
        &self.graph.start
    }

    fn to(&self) -> &[u32] {
        &self.graph.to
    }
}

impl Rounds for Switched<'_> {
    fn deciding(&self, v: u32) -> Range<usize> {
        let slots = self.graph.slots(v);
        if self.forest.kind(OccId::new(v)) == Kind::Par {
            // A `⅋` has two premises and no axiom slot, so a third slot is
            // its parent edge, to its parent or across its cut link.
            if slots.len() < 3 {
                slots.start..slots.start
            } else {
                slots.start..slots.start + 1
            }
        } else {
            slots
        }
    }
}

impl Graph {
    /// Builds the graph of a forest, with every literal unlinked: over the
    /// occurrences `live` holds, or every one, and the cut links `cuts`
    /// between them.
    pub(super) fn new(
        forest: &Forest,
        live: Option<&OccSet>,
        cuts: &[(VertexId, VertexId)],
    ) -> Self {
        let n = forest.len();
        let mut start = Vec::with_capacity(n + 1);
        let mut to = Vec::with_capacity(3 * n);
        let mut switched_tree_edges = cuts.len();
        let mut across = Vec::new();
        if !cuts.is_empty() {
            across = vec![NONE; n];
            for &(x, y) in cuts {
                across[x.index()] = y.get();
                across[y.index()] = x.get();
            }
        }
        let mut vertices = 0;
        for o in forest.ids() {
            start.push(to.len() as u32);
            if live.is_some_and(|live| !live.contains(o)) {
                continue;
            }
            vertices += 1;
            match across.get(o.index()) {
                Some(&end) if end != NONE => to.push(end),
                _ => {
                    if let Some(p) = forest.parent(o) {
                        to.push(p.get());
                    }
                }
            }
            to.extend(forest.children(o).map(OccId::get));
            match forest.kind(o) {
                Kind::Tensor => switched_tree_edges += 2,
                Kind::Par => switched_tree_edges += 1,
                Kind::Atom
                | Kind::DualAtom
                | Kind::One
                | Kind::Bot
                | Kind::Top
                | Kind::Zero
                | Kind::With
                | Kind::Plus
                | Kind::Bang
                | Kind::Quest
                | Kind::Forall
                | Kind::Exists => to.push(NONE),
            }
        }
        start.push(to.len() as u32);
        Self {
            start: start.into_boxed_slice(),
            to: to.into_boxed_slice(),
            switched_tree_edges,
            vertices,
        }
    }

    /// Returns the number of vertices with slots.
    pub(super) fn vertices(&self) -> usize {
        self.vertices
    }

    /// Returns working memory for this graph.
    pub(super) fn scratch(&self) -> Scratch {
        Scratch::of(self)
    }

    /// Returns the slots of a vertex.
    fn slots(&self, v: u32) -> Range<usize> {
        kernel::slots(self, v)
    }

    /// Returns the slot of a literal's axiom link.
    fn axiom_slot(&self, l: u32) -> usize {
        self.start[l as usize + 1] as usize - 1
    }

    /// Adds the axiom link between two literals.
    pub(super) fn link(&mut self, x: u32, y: u32) {
        self.to[self.axiom_slot(x)] = y;
        self.to[self.axiom_slot(y)] = x;
    }

    /// Removes the axiom link between two literals.
    pub(super) fn unlink(&mut self, x: u32, y: u32) {
        self.to[self.axiom_slot(x)] = NONE;
        self.to[self.axiom_slot(y)] = NONE;
    }

    /// Takes the cut link at its end `x` out of the graph for the searches
    /// with `scratch` from now on.
    pub(super) fn remove_cut(&self, scratch: &mut Scratch, x: u32) {
        let slot = self.slots(x).start;
        let y = self.to[slot];
        let twin = kernel::twin(self, x, y);
        scratch.removed[slot] = true;
        scratch.removed[twin] = true;
    }

    /// Returns the number of edges every switching keeps when the
    /// structure has `links` axiom links.
    pub(super) fn switched_edges(&self, links: usize) -> usize {
        self.switched_tree_edges + links
    }

    /// Searches depth-first from each of `roots` in turn
    /// ([`kernel::search`]) and returns the number of search trees.
    pub(super) fn search(
        &self,
        scratch: &mut Scratch,
        roots: impl IntoIterator<Item = u32>,
    ) -> u32 {
        kernel::search(self, scratch, roots)
    }

    /// Runs the deletion procedure on the vertices not yet deleted
    /// ([`kernel::acyclic`]): whether no switching cycle is left, `None`
    /// when `stop` fired. Not generic, so that the rounds are made here
    /// with the searches inlined into them.
    pub(super) fn acyclic(
        &self,
        forest: &Forest,
        scratch: &mut Scratch,
        stop: &mut dyn FnMut(u64) -> bool,
    ) -> Option<bool> {
        kernel::acyclic(
            &Switched {
                graph: self,
                forest,
            },
            scratch,
            stop,
        )
    }

    /// Isolates one switching cycle among the vertices that
    /// [`acyclic`](Self::acyclic) got stuck on, in the call just before
    /// ([`kernel::cycle`]); `None` when `stop` fired.
    pub(super) fn cycle(
        &self,
        forest: &Forest,
        scratch: &mut Scratch,
        stop: &mut dyn FnMut(u64) -> bool,
    ) -> Option<Vec<VertexId>> {
        let cycle = kernel::cycle(
            &Switched {
                graph: self,
                forest,
            },
            scratch,
            stop,
        )?;
        Some(cycle.into_iter().map(VertexId).collect())
    }

    /// Returns the connected components of the switching that keeps the
    /// left premise of every `⅋`, in the order of their smallest vertex.
    ///
    /// Each component is given as the vertices in it that have no parent
    /// edge there: the roots, the right premises of `⅋` nodes, and of each
    /// cut link its end in a cut formula's tree.
    pub(super) fn parts(&self, forest: &Forest, scratch: &mut Scratch) -> Vec<Vec<VertexId>> {
        let n = self.start.len() - 1;
        scratch.restore();
        let vertex = |o: OccId| !self.slots(o.get()).is_empty();
        for p in forest
            .ids()
            .filter(|&p| forest.kind(p) == Kind::Par && vertex(p))
        {
            let r = forest.right(p).unwrap();
            scratch.removed[kernel::twin(self, r.get(), p.get())] = true;
            scratch.removed[self.slots(r.get()).start] = true;
        }
        self.search(scratch, 0..n as u32);
        scratch.removed.fill(false);
        let mut parts: Vec<(u32, Vec<VertexId>)> = Vec::new();
        // Per component's label, the index of its part, or `u32::MAX`: a
        // search of the parts found so far per vertex was quadratic in
        // them.
        let mut index = vec![u32::MAX; scratch.component.len()];
        for v in forest.ids().filter(|&v| vertex(v)) {
            // A cut link is named by its end in a cut formula's tree.
            let cut = match forest.parent(v) {
                None => forest.dual(v).is_none_or(|d| v < d),
                Some(p) if !vertex(p) => forest.dual(v).is_some_and(|d| v < d),
                Some(p) => forest.kind(p) == Kind::Par && forest.right(p) == Some(v),
            };
            if !cut {
                continue;
            }
            let label = scratch.component[v.index()];
            let at = &mut index[label as usize];
            if *at == u32::MAX {
                *at = parts.len() as u32;
                parts.push((label, Vec::new()));
            }
            parts[*at as usize].1.push(VertexId::of(v));
        }
        parts.sort_by_key(|(label, _)| *label);
        parts.into_iter().map(|(_, part)| part).collect()
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::super::{Criterion, NetError, ProofStructure, VertexId};
    use crate::linear::occurrences::{Forest, OccId};
    use crate::linear::search::generate::{self, Rng, Rules};
    use crate::linear::sequents::{Kind, Sequent};

    /// Wraps a raw vertex id.
    const fn v(id: u32) -> VertexId {
        VertexId::new(id)
    }

    /// Builds the structure of `input` with the links, or panics.
    fn net(input: &str, mix: bool, links: &[(u32, u32)]) -> ProofStructure {
        let s: Sequent = input.parse().unwrap_or_else(|e| panic!("{input:?}: {e}"));
        let links: Vec<(VertexId, VertexId)> = links.iter().map(|&(x, y)| (v(x), v(y))).collect();
        ProofStructure::from_links(
            Forest::new(&s).unwrap(),
            Criterion {
                mix,
                ..Criterion::MLL
            },
            &links,
        )
        .unwrap()
    }

    /// The Danos–Regnier criterion by brute force: every switching is
    /// built and tested for a cycle and for connectedness with a
    /// union-find. Returns whether every switching is acyclic and whether
    /// every switching is connected.
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "a structure of MLL has no other parent"
    )]
    fn enumerate(net: &ProofStructure) -> (bool, bool) {
        let f = net.forest();
        let pars: Vec<OccId> = f.ids().filter(|&p| f.kind(p) == Kind::Par).collect();
        assert!(pars.len() <= 16, "too many switchings to enumerate");
        let (mut acyclic, mut connected) = (true, true);
        for switching in 0..1u32 << pars.len() {
            let keeps_left = |p: OccId| {
                let i = pars.iter().position(|&q| q == p).unwrap();
                switching & (1 << i) == 0
            };
            let mut parent: Vec<usize> = (0..f.len()).collect();
            fn find(parent: &mut [usize], mut x: usize) -> usize {
                while parent[x] != x {
                    x = parent[x];
                }
                x
            }
            let mut components = f.len();
            let mut cyclic = false;
            let mut join = |x: OccId, y: OccId| {
                let (a, b) = (find(&mut parent, x.index()), find(&mut parent, y.index()));
                if a == b {
                    cyclic = true;
                } else {
                    parent[a] = b;
                    components -= 1;
                }
            };
            for v in f.ids() {
                if let Some(p) = f.parent(v) {
                    let kept = match f.kind(p) {
                        Kind::Tensor => true,
                        Kind::Par => keeps_left(p) == (f.left(p) == Some(v)),
                        _ => unreachable!(),
                    };
                    if kept {
                        join(p, v);
                    }
                }
            }
            for &(x, y) in net.links() {
                join(x.occ(), y.occ());
            }
            acyclic &= !cyclic;
            connected &= components == 1;
        }
        (acyclic, connected)
    }

    /// Checks that a reported cycle is a switching cycle: a simple cycle of
    /// the graph that uses at most one premise edge of every `⅋`.
    fn check_cycle(net: &ProofStructure, cycle: &[VertexId]) {
        let f = net.forest();
        let cycle: Vec<OccId> = cycle.iter().map(|v| v.occ()).collect();
        assert!(cycle.len() >= 3, "{cycle:?}");
        let mut sorted = cycle.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), cycle.len(), "simple: {cycle:?}");
        for i in 0..cycle.len() {
            let (a, b, c) = (
                cycle[(i + cycle.len() - 1) % cycle.len()],
                cycle[i],
                cycle[(i + 1) % cycle.len()],
            );
            let adjacent = f.parent(b) == Some(c)
                || f.parent(c) == Some(b)
                || net.partner(VertexId::of(b)) == Some(VertexId::of(c));
            assert!(adjacent, "{b:?} and {c:?} are not adjacent in {cycle:?}");
            if f.kind(b) == Kind::Par {
                assert!(
                    f.parent(a) != Some(b) || f.parent(c) != Some(b),
                    "{cycle:?} passes {b:?} through both premises"
                );
            }
        }
    }

    /// The classic nets: the axiom-tensor example is a net, a link below a
    /// `⊗` is a cycle, below a `⅋` it is a net, two separate axioms under a
    /// `⅋` are a net with Mix only, and a partial or empty structure is
    /// neither.
    #[test]
    fn classic_structures() {
        // ⊢ ~A, A ⊗ ~B, B: 0 ~A, 1 ⊗, 2 A, 3 ~B, 4 B.
        assert_eq!(
            net("A, A -o B |- B", false, &[(0, 2), (3, 4)]).is_correct(|_| false),
            Ok(())
        );
        let partial = net("A, A -o B |- B", false, &[(0, 2)]);
        assert_eq!(
            partial.is_correct(|_| false),
            Err(NetError::Unlinked { vertex: v(4) })
        );
        assert!(partial.is_acyclic(&mut partial.scratch()));
        // ⊢ A ⊗ ~A: 0 ⊗, 1 A, 2 ~A.
        let cyclic = net("|- A * ~A", true, &[(1, 2)]);
        let Err(NetError::SwitchingCycle { cycle }) = cyclic.is_correct(|_| false) else {
            panic!("a link below a ⊗ is a switching cycle");
        };
        check_cycle(&cyclic, &cycle);
        assert_eq!(cycle, [v(0), v(1), v(2)]);
        assert!(!cyclic.is_acyclic(&mut cyclic.scratch()));
        // ⊢ A ⅋ ~A: the cycle through both premises does not count.
        assert_eq!(
            net("|- A par ~A", false, &[(1, 2)]).is_correct(|_| false),
            Ok(())
        );
        // ⊢ A ⅋ B, ~A, ~B: 0 ⅋, 1 A, 2 B, 3 ~A, 4 ~B.
        assert_eq!(
            net("|- A par B, ~A, ~B", false, &[(1, 3), (2, 4)]).is_correct(|_| false),
            Err(NetError::Disconnected {
                parts: vec![vec![v(0), v(3)], vec![v(2), v(4)]]
            })
        );
        assert_eq!(
            net("|- A par B, ~A, ~B", true, &[(1, 3), (2, 4)]).is_correct(|_| false),
            Ok(())
        );
        // ⊢ (A ⅋ ~A) ⅋ (B ⅋ ~B): 0 ⅋, 1 ⅋, 2 A, 3 ~A, 4 ⅋, 5 B, 6 ~B. The
        // switching that keeps every left premise drops the edge from 0 to
        // 4, which leaves 4, 5 and 6 apart.

        assert_eq!(
            net("|- (A par ~A) par (B par ~B)", false, &[(2, 3), (5, 6)]).is_correct(|_| false),
            Err(NetError::Disconnected {
                parts: vec![vec![v(0), v(3)], vec![v(4), v(6)]]
            })
        );
        // ⊢ A ⊗ B, ~A ⊗ ~B: 0 ⊗, 1 A, 2 B, 3 ⊗, 4 ~A, 5 ~B: a cycle through
        // both tensors.
        let cyclic = net("|- A * B, ~A * ~B", true, &[(1, 4), (2, 5)]);
        let Err(NetError::SwitchingCycle { cycle }) = cyclic.is_correct(|_| false) else {
            panic!("two tensors joined twice are a switching cycle");
        };
        check_cycle(&cyclic, &cycle);
        assert_eq!(cycle.len(), 6);
        assert_eq!(
            net("|-", true, &[]).is_correct(|_| false),
            Err(NetError::Empty)
        );
    }

    /// The criterion agrees with the brute-force enumeration of switchings
    /// on random linkings of random sequents, with and without Mix,
    /// partial and complete, and every witness it names is one.
    #[test]
    fn agrees_with_enumeration() {
        let mut cases = (0, 0, 0);
        for (seed, mix) in [(1, false), (2, true)] {
            let mut rng = Rng::new(seed);
            for i in 0..200 {
                // Sequents that need Mix give disconnected structures when
                // Mix is not allowed; mutants give partial and cyclic ones.
                let rules = Rules {
                    units: false,
                    additives: false,
                    mix: i % 3 == 2,
                    exponentials: false,
                };
                let budget = 2 + rng.below(9);
                let mut formulas = generate::provable(&mut rng, rules, 3, budget).formulas;
                if i % 2 == 1 {
                    generate::mutate(&mut rng, &mut formulas, 3);
                }
                let text = generate::sequent(&formulas);
                let s: Sequent = text.parse().unwrap();
                let forest = Forest::new(&s).unwrap();
                // A random pairing per atom, as far as the counts allow.
                let mut pairs = Vec::new();
                for a in 0..s.atom_names().len() {
                    let atom = crate::linear::sequents::Atom::new(a as u32);
                    let vars = forest.literals(atom, crate::linear::occurrences::Sign::Atom);
                    let mut duals = forest
                        .literals(atom, crate::linear::occurrences::Sign::Dual)
                        .to_vec();
                    for k in (1..duals.len()).rev() {
                        duals.swap(k, rng.below(k + 1));
                    }
                    pairs.extend(vars.iter().copied().zip(duals));
                }
                for k in (1..pairs.len()).rev() {
                    pairs.swap(k, rng.below(k + 1));
                }
                let mut net = ProofStructure::new(
                    forest,
                    Criterion {
                        mix,
                        ..Criterion::MLL
                    },
                )
                .unwrap();
                let mut scratch = net.scratch();
                for &(x, y) in &pairs {
                    net.link(VertexId::of(x), VertexId::of(y)).unwrap();
                    let (acyclic, _) = enumerate(&net);
                    assert_eq!(
                        net.is_acyclic(&mut scratch),
                        acyclic,
                        "{text:?} with {:?}",
                        net.links()
                    );
                }
                let (acyclic, connected) = enumerate(&net);
                let verdict = net.is_correct(|_| false);
                let expected =
                    !net.forest().is_empty() && net.is_complete() && acyclic && (mix || connected);
                assert_eq!(
                    verdict.is_ok(),
                    expected,
                    "{text:?} with {:?}: {verdict:?}",
                    net.links()
                );
                match &verdict {
                    Ok(()) => cases.0 += 1,
                    Err(NetError::SwitchingCycle { cycle }) => {
                        check_cycle(&net, cycle);
                        cases.1 += 1;
                    }
                    Err(NetError::Disconnected { parts }) => {
                        let f = net.forest();
                        let switched = f.ids().filter(|&v| f.kind(v) == Kind::Tensor).count() * 2
                            + f.ids().filter(|&v| f.kind(v) == Kind::Par).count()
                            + net.links().len();
                        assert_eq!(parts.len(), f.len() - switched, "{text:?}");
                        for &r in f.roots() {
                            assert_eq!(
                                parts
                                    .iter()
                                    .filter(|p| p.contains(&VertexId::of(r)))
                                    .count(),
                                1,
                                "{text:?}"
                            );
                        }
                        cases.2 += 1;
                    }
                    Err(NetError::Unlinked { .. }) => {}
                    Err(e) => panic!("{text:?}: {e}"),
                }
            }
        }
        assert!(cases.0 > 20 && cases.1 > 20 && cases.2 > 5, "{cases:?}");
    }
}
