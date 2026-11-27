// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Sequentialization: from a proof net to a proof term, by the splitting
//! tensor lemma.
//!
//! A sub-net is given by its conclusions. While one of them is a `⅋`, it
//! is replaced by its premises, and a `⅋` rule goes below whatever proves
//! the rest. The *plain graph* of a sub-net is its formula trees and
//! links, with no switching. With Mix allowed, a sub-net whose plain
//! graph falls into several parts is proved part by part, and the parts
//! are joined with Mix. Two literals alone are an axiom.
//!
//! Otherwise some `⊗` conclusion is *splitting*: the net without it is
//! two nets, one holding each premise. That is the case exactly when its
//! edge to a premise is a bridge of the plain graph, an edge whose
//! removal disconnects its two ends. A cut link is such a `⊗` without a
//! conclusion: it is splitting when it is a bridge, and its rule is the
//! cut. One depth-first search per stage finds the parts and the bridges
//! at once, so the whole sequentialization costs at most the square of
//! the net's size.

use super::assemble::{Assemble, Assembly, Open, Parts, Rule};
use super::{ProofStructure, SEQUENTIALIZE_BYTES, STOPPED, Scratch, VertexId, afford};
use crate::Error;
use crate::limits::{Limits, Phase, Progress};
use crate::linear::occurrences::OccId;
use crate::linear::proofs::{Node, Proof};
use crate::linear::sequents::Kind;

impl ProofStructure {
    /// Turns a proof net into a proof term of its sequent, which the proof
    /// checker accepts.
    ///
    /// The proof has `⅋`, `⊗` and axiom nodes, a cut for each cut link,
    /// and Mix nodes where the structure falls apart. It records the mode
    /// it is meant for: classical, with Mix where the criterion allows it,
    /// or the cyclic mode under a criterion of the cyclic order.
    ///
    /// The call takes time quadratic in the structure at most. It holds
    /// within `limits.memory_bytes`, which it compares with an estimate
    /// before it makes anything. It asks `stop` as
    /// [`is_correct`](Self::is_correct) does, and once per stage, each
    /// stage a search of the sub-net left.
    ///
    /// # Errors
    ///
    /// The criterion's [`NetError`](super::NetError) if the structure is
    /// not a proof net; [`NetError::Refused`](super::NetError::Refused)
    /// when the bound or the stop ended it.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "parse", doc = "```")]
    #[cfg_attr(not(feature = "parse"), doc = "```ignore")]
    /// use deduxit::Limits;
    /// use deduxit::linear::{Criterion, Forest, Mode, ProofStructure, Sequent, VertexId};
    ///
    /// // ⊢ ~A, A ⊗ ~B, B, with the vertices 0: ~A, 1: A ⊗ ~B, 2: A,
    /// // 3: ~B, 4: B.
    /// let sequent: Sequent = "A, A -o B |- B".parse()?;
    /// let v = VertexId::new;
    /// let links = [(v(0), v(2)), (v(3), v(4))];
    /// let net = ProofStructure::from_links(Forest::new(&sequent)?, Criterion::MLL, &links)?;
    ///
    /// // Two axioms and the ⊗.
    /// let limits = Limits::default();
    /// let proof = net.sequentialize(&limits, |_| false)?;
    /// assert_eq!(proof.nodes().len(), 3);
    /// assert_eq!(proof.check(Mode::CLASSICAL), Ok(()));
    ///
    /// // The proof's net is the net it came from.
    /// let back = ProofStructure::from_proof(&proof, Criterion::MLL, &limits, |_| false)?;
    /// assert_eq!(back.links(), net.links());
    /// # Ok::<(), deduxit::Error>(())
    /// ```
    #[expect(
        clippy::missing_panics_doc,
        reason = "the expect states that a sequentialization keeps premises before conclusions, which the stages establish"
    )]
    pub fn sequentialize(
        &self,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Proof, Error> {
        let forest = &self.forest;
        let arena = forest.sequent().terms().len() as u64
            * size_of::<crate::linear::sequents::Term>() as u64;
        let bytes = (forest.len() as u64 * SEQUENTIALIZE_BYTES).saturating_add(arena);
        afford(bytes, limits)?;
        self.is_correct(|progress| {
            stop(Progress {
                held_bytes: bytes,
                ..progress
            })
        })?;
        let mut run = Sequentialization {
            net: self,
            scratch: self.scratch(),
            proof: Assembly::with_capacity(self.forest.len()),
        };
        let mut stages = 0;
        let mut stop = |stage: u64| {
            stages += stage;
            stop(Progress {
                held_bytes: bytes,
                ..Progress::new(Phase::Net, stage, stages)
            })
        };
        let cuts = self.cuts.iter().map(|&(x, _)| x.occ()).collect();
        let root = run.run((self.forest.roots().to_vec(), cuts), &mut stop);
        let Some(root) = root else {
            return Err(STOPPED.into());
        };
        let mode = self.criterion.mode();
        let proof = Proof::new(self.forest.clone(), run.proof.nodes, root)
            .expect("premises precede conclusions and every occurrence is the forest's")
            .with_mode(mode);
        debug_assert_eq!(proof.check(mode), Ok(()));
        Ok(proof)
    }
}

/// A sequentialization in progress.
struct Sequentialization<'a> {
    /// The proof net.
    net: &'a ProofStructure,
    /// Working memory for the searches; conclusions already handled are
    /// deleted in it, so a sub-net is exactly what its conclusions reach.
    scratch: Scratch,
    /// The proof so far: a sub-net to prove is named by its conclusions
    /// and the cut links at their ends in a cut formula's tree.
    proof: Assembly<(Vec<OccId>, Vec<OccId>)>,
}

impl Assemble for Sequentialization<'_> {
    type Goal = (Vec<OccId>, Vec<OccId>);

    fn assembly(&mut self) -> &mut Assembly<Self::Goal> {
        &mut self.proof
    }

    /// Runs one stage on the sub-net with the given conclusions and cut
    /// links, each cut link named by its end in a cut formula's tree.
    ///
    /// The stage proves the sub-net by an axiom, or leaves the steps that
    /// prove it. Those are its parts with a Mix after each but the first,
    /// or the two sides of its splitting `⊗` or cut link with the `⊗` or
    /// cut after them. The rules of the `⅋` conclusions it opened come
    /// after either.
    // Inlined into the shared loop: made apart, its searches count 0.7 %
    // more on the net engine's journey (`search-wide-m2-256`), which ends
    // by sequentializing the net found.
    #[inline(always)]
    fn stage(&mut self, (mut gamma, cuts): Self::Goal) {
        let f = self.net.forest();
        let graph = &self.net.graph;
        let child = |o: OccId, left: bool| if left { f.left(o) } else { f.right(o) }.unwrap();
        let across = |a: OccId| f.dual(a).expect("a cut link's end in a cut formula");

        // Every ⅋ conclusion is opened, outermost first; its rule is
        // applied last, below the rest.
        let mut pars = Vec::new();
        let mut i = 0;
        while i < gamma.len() {
            let c = gamma[i];
            if f.kind(c) == Kind::Par {
                gamma[i] = child(c, true);
                gamma.push(child(c, false));
                self.scratch.delete(c.get());
                pars.push(Open::Par(c));
            } else {
                i += 1;
            }
        }
        if !pars.is_empty() {
            self.proof.apply(Rule::Opened(pars));
        }

        let roots = gamma.iter().chain(&cuts).map(|o| o.get());
        let parts = graph.search(&mut self.scratch, roots);
        if parts > 1 {
            debug_assert!(
                self.net.criterion.mix,
                "a proof net without Mix is connected"
            );
            // The parts in the order of their first conclusion, of `gamma`
            // then of `cuts`.
            let labels = gamma.iter().chain(&cuts);
            let parts = Parts::of(labels.map(|&c| self.scratch.component(c.get())));
            let mut goals: Vec<(Vec<OccId>, Vec<OccId>)> =
                vec![(Vec::new(), Vec::new()); parts.len()];
            for &c in &gamma {
                goals[parts.index(self.scratch.component(c.get()))]
                    .0
                    .push(c);
            }
            for &c in &cuts {
                goals[parts.index(self.scratch.component(c.get()))]
                    .1
                    .push(c);
            }
            parts.leave(&mut self.proof, goals);
        } else if cuts.is_empty() && gamma.iter().all(|&c| f.is_literal(c)) {
            debug_assert!(
                gamma.len() == 2
                    && self.net.partner(VertexId::of(gamma[0])) == Some(VertexId::of(gamma[1]))
            );
            let (x, y) = (gamma[0].min(gamma[1]), gamma[0].max(gamma[1]));
            self.proof.proved_by(Node::Ax(x.into(), y.into()));
        } else {
            // The splitting ⊗ conclusion with the smallest id, one whose
            // premise edge is a bridge, or else the splitting cut link
            // with the smallest end, whose edge is a bridge.
            let tensor = gamma
                .iter()
                .copied()
                .filter(|&c| f.kind(c) == Kind::Tensor)
                .filter(|&c| self.scratch.is_bridge(c.get(), child(c, true).get()))
                .min();
            let cut = || {
                cuts.iter()
                    .copied()
                    .filter(|&a| self.scratch.is_bridge(a.get(), across(a).get()))
                    .min()
            };
            let (split, left_end, right_end) = match tensor {
                Some(t) => {
                    self.scratch.delete(t.get());
                    (Rule::Tensor(t), child(t, true), child(t, false))
                }
                None => {
                    let a = cut().expect(
                        "a connected proof net whose conclusions are not all literals has a \
                         splitting ⊗ or cut link",
                    );
                    graph.remove_cut(&mut self.scratch, a.get());
                    (Rule::Cut(a), a, across(a))
                }
            };
            graph.search(&mut self.scratch, [left_end.get()]);
            let (mut left, mut right) = (vec![left_end], vec![right_end]);
            let (mut left_cuts, mut right_cuts) = (Vec::new(), Vec::new());
            for &c in &gamma {
                if matches!(split, Rule::Tensor(t) if t == c) {
                    continue;
                }
                if self.scratch.reached(c.get()) {
                    left.push(c);
                } else {
                    right.push(c);
                }
            }
            for &a in &cuts {
                if matches!(split, Rule::Cut(b) if b == a) {
                    continue;
                }
                if self.scratch.reached(a.get()) {
                    left_cuts.push(a);
                } else {
                    right_cuts.push(a);
                }
            }
            self.proof.apply(split);
            self.proof.prove((right, right_cuts));
            self.proof.prove((left, left_cuts));
        }
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::super::{Criterion, ProofStructure, VertexId};
    use crate::limits::Limits;
    use crate::linear::fragment::Mode;
    use crate::linear::occurrences::{Forest, Member, OccId};
    #[cfg(feature = "parse")]
    use crate::linear::proofs::{Node, NodeId};
    use crate::linear::sequents::Sequent;

    /// Wraps a raw id.
    #[cfg(feature = "parse")]
    const fn o(id: u32) -> OccId {
        OccId::new(id)
    }

    /// Builds the structure of `input` with the links, or panics.
    #[cfg(feature = "parse")]
    fn net(input: &str, mix: bool, links: &[(u32, u32)]) -> ProofStructure {
        let s: Sequent = input.parse().unwrap_or_else(|e| panic!("{input:?}: {e}"));
        let links: Vec<(VertexId, VertexId)> = links
            .iter()
            .map(|&(x, y)| (VertexId::new(x), VertexId::new(y)))
            .collect();
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

    /// A net whose derivation is thousands of inferences high is
    /// sequentialized on a stack too small for a recursion to follow it,
    /// 64 bytes a level: the tensors of `⊢ a1 ⊗ (a2 ⊗ (… ⊗ an)), ~a1, …,
    /// ~an` split off one axiom at a time.
    #[test]
    fn a_high_derivation_needs_no_stack() {
        use crate::linear::occurrences::Sign;
        use crate::linear::sequents::{Atom, Term, TermId};
        const PAIRS: u32 = 2048;
        // Terms 2i and 2i + 1 are the two literals of atom i.
        let mut terms: Vec<Term> = (0..PAIRS)
            .map(Atom::new)
            .flat_map(|a| [Term::Atom(a), Term::DualAtom(a)])
            .collect();
        let mut chain = TermId::new(2 * (PAIRS - 1));
        for a in (0..PAIRS - 1).rev() {
            terms.push(Term::Tensor(TermId::new(2 * a), chain));
            chain = TermId::new(terms.len() as u32 - 1);
        }
        let mut roots = vec![chain];
        roots.extend((0..PAIRS).map(|a| TermId::new(2 * a + 1)));
        let atoms = (0..PAIRS).map(|a| format!("a{a}")).collect();
        let forest = Forest::new(&Sequent {
            terms,
            roots,
            atoms,
            antecedents: None,
            cuts: vec![],
            fo: Default::default(),
        })
        .unwrap();
        let literal = |a, sign| forest.literals(Atom::new(a), sign)[0];
        let links: Vec<(VertexId, VertexId)> = (0..PAIRS)
            .map(|a| {
                let (x, y) = (literal(a, Sign::Atom), literal(a, Sign::Dual));
                (VertexId::new(x.get()), VertexId::new(y.get()))
            })
            .collect();
        let net = ProofStructure::from_links(forest, Criterion::MLL, &links).unwrap();
        let thread = std::thread::Builder::new().stack_size(128 * 1024);
        let proof = thread
            .spawn(move || net.sequentialize(&Limits::default(), |_| false))
            .unwrap();
        let proof = proof.join().unwrap().unwrap();
        assert_eq!(proof.nodes().len(), 2 * PAIRS as usize - 1);
        assert_eq!(proof.check(Mode::CLASSICAL), Ok(()));
    }

    /// The classic nets sequentialize into the expected terms: the ⅋ below
    /// the ⊗ that is not splitting until it is opened, a Mix for a
    /// disconnected net, and an error for a structure that is no net.
    #[cfg(feature = "parse")]
    #[test]
    fn classic_nets() {
        use Node::*;
        let n = NodeId::new;
        // ⊢ A ⊗ B, ~A ⅋ ~B: 0 ⊗, 1 A, 2 B, 3 ⅋, 4 ~A, 5 ~B. The ⊗ splits
        // only once the ⅋ is opened.
        let proof = net("|- A * B, ~A par ~B", false, &[(1, 4), (2, 5)])
            .sequentialize(&Limits::default(), |_| false)
            .unwrap();
        assert_eq!(
            proof.nodes(),
            [
                Ax(o(1).into(), o(4).into()),
                Ax(o(2).into(), o(5).into()),
                Tensor(Member::from(o(0)), n(0), n(1)),
                Par(Member::from(o(3)), n(2)),
            ]
        );
        // ⊢ A ⅋ B, ~A, ~B with Mix: 0 ⅋, 1 A, 2 B, 3 ~A, 4 ~B.
        let proof = net("|- A par B, ~A, ~B", true, &[(1, 3), (2, 4)])
            .sequentialize(&Limits::default(), |_| false)
            .unwrap();
        assert_eq!(
            proof.nodes(),
            [
                Ax(o(1).into(), o(3).into()),
                Ax(o(2).into(), o(4).into()),
                Mix(n(0), n(1)),
                Par(Member::from(o(0)), n(2))
            ]
        );
        assert_eq!(proof.check(Mode::CLASSICAL.with_mix()), Ok(()));
        // ⊢ A ⊗ B, C ⊗ (~A ⅋ ~B), ~C: 0 ⊗, 1 A, 2 B, 3 ⊗, 4 C, 5 ⅋, 6 ~A,
        // 7 ~B, 8 ~C: the first ⊗ is not splitting, the second is.
        let proof = net(
            "|- A * B, C * (~A par ~B), ~C",
            false,
            &[(1, 6), (2, 7), (4, 8)],
        )
        .sequentialize(&Limits::default(), |_| false)
        .unwrap();
        assert_eq!(
            proof.nodes(),
            [
                Ax(o(4).into(), o(8).into()),
                Ax(o(1).into(), o(6).into()),
                Ax(o(2).into(), o(7).into()),
                Tensor(Member::from(o(0)), n(1), n(2)),
                Par(Member::from(o(5)), n(3)),
                Tensor(Member::from(o(3)), n(0), n(4)),
            ]
        );
        assert!(
            net("|- A par B, ~A, ~B", false, &[(1, 3), (2, 4)])
                .sequentialize(&Limits::default(), |_| false)
                .is_err()
        );
    }
}
