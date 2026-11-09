// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The planar criterion of the linking searches, for the ordered modes: a
//! wrapped criterion, plus no two links crossing in the cyclic order of
//! the literals, which is the order of their occurrence ids.
//!
//! A structure of unit-free MLL is a proof net of cyclic MLL exactly when
//! it is a planar proof net ([`ProofStructure::is_planar`]). So the search
//! rejects every candidate link that would cross a link made, and leaves
//! the rest to the criterion it wraps. That is the Danos–Regnier criterion
//! of the classical search for cyclic MLL, and the essential nets'
//! criterion for the Lambek calculus (Lamarche and Retoré's planar
//! essential nets).
//!
//! Links that do not cross split the literals into regions. A literal's
//! region is the innermost link around it in the linear order of the
//! literals, or none. A new link crosses no link exactly when its two
//! literals lie in one region. If one of them lies inside a made link and
//! the other outside it, that link crosses the new one. If a made link
//! crosses the new one, one of the new link's literals lies inside it and
//! the other outside, so their regions differ. The search keeps every
//! literal's region: a link rewrites the regions inside its span, and an
//! unlink restores them.
//!
//! A link also leaves the literals strictly between its two ends to be
//! linked among themselves, since a link from inside to outside would
//! cross it. So every atom must have as many `a` as `~a` there; the linked
//! ones are pairs already. That depends on the two ends alone, and is
//! compared through prefix sums of a hash of the signed atoms. Equal
//! counts give equal sums, so the comparison never rejects a link a net
//! could hold. Two unequal counts that happen to give equal sums let
//! through a link that the search finds dead later.

use super::Reason;
use super::net::Correctness;
use crate::linear::nets::VertexId;
use crate::linear::occurrences::{Forest, OccId, Sign};
use crate::linear::sequents::Atom;

/// The raw index that stands for "no literal": the region outside every
/// link, and an occurrence that is no literal.
const NONE: u32 = u32::MAX;

/// A criterion with planarity: the regions of the literals between the
/// links made, and the prefix sums that say which spans are balanced.
pub(super) struct Planar<C> {
    /// The criterion wrapped, which holds the links.
    inner: C,
    /// Per occurrence, its place among the literals in id order, or
    /// `NONE` for a compound one. A place is below the number of
    /// literals, which is below the forest's length and fits a `u32`.
    place: Box<[u32]>,
    /// Per place, its region: the place of the left end of the innermost
    /// link around it, or `NONE`.
    region: Box<[u32]>,
    /// Per place `i`, the sum of the atom weights of the literals at places
    /// before `i`: added for `a`, subtracted for `~a`, wrapping. It is a
    /// hash of the signed counts and is only ever compared, so a wrap costs
    /// no soundness.
    prefix: Box<[u64]>,
    /// The links made, as the places of their two ends, the left one
    /// first, with the region the left end was in.
    made: Vec<(u32, u32, u32)>,
}

impl<C: Correctness> Planar<C> {
    /// Returns the criterion `inner` with planarity. `inner` is over a
    /// forest of unit-free MLL and has no link yet.
    pub(super) fn new(inner: C) -> Self {
        let forest = inner.forest();
        debug_assert!(inner.links().is_empty(), "no link is made yet");
        let mut place = vec![NONE; forest.len()].into_boxed_slice();
        let mut prefix = vec![0u64];
        let mut sum = 0u64;
        for o in forest.ids() {
            let (Some(atom), Some(sign)) = (forest.atom(o), forest.sign(o)) else {
                continue;
            };
            place[o.index()] = (prefix.len() - 1) as u32;
            let weight = weight(atom);
            sum = match sign {
                Sign::Atom => sum.wrapping_add(weight),
                Sign::Dual => sum.wrapping_sub(weight),
            };
            prefix.push(sum);
        }
        let literals = prefix.len() - 1;
        Self {
            inner,
            place,
            region: vec![NONE; literals].into_boxed_slice(),
            prefix: prefix.into_boxed_slice(),
            made: Vec::with_capacity(literals / 2),
        }
    }

    /// Returns the criterion wrapped, which holds the links.
    pub(super) fn into_inner(self) -> C {
        self.inner
    }

    /// Returns the criterion wrapped.
    pub(super) const fn inner(&self) -> &C {
        &self.inner
    }

    /// Returns whether no two of the links the wrapped criterion holds
    /// cross: in the order of the literals, each closing end closes the
    /// innermost link open.
    fn is_planar(&self) -> bool {
        let mut ends: Vec<(u32, u32)> = self
            .inner
            .links()
            .iter()
            .flat_map(|&(x, y)| {
                let (left, right) = self.ends(x.occ(), y.occ());
                [(left, right), (right, left)]
            })
            .collect();
        ends.sort_unstable();
        let mut open = Vec::new();
        ends.into_iter().all(|(at, other)| {
            if other > at {
                open.push(at);
                true
            } else {
                open.pop() == Some(other)
            }
        })
    }

    /// Returns the places of two literals, the smaller first.
    fn ends(&self, x: OccId, y: OccId) -> (u32, u32) {
        let (p, q) = (self.place[x.index()], self.place[y.index()]);
        (p.min(q), p.max(q))
    }
}

/// Returns the weight of an atom in the prefix sums: a hash of its index
/// (the SplitMix64 finalizer), made odd, so that the weights of different
/// atoms are unlikely to cancel.
fn weight(atom: Atom) -> u64 {
    let mut z = u64::from(atom.get()).wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    (z ^ (z >> 31)) | 1
}

impl<C: Correctness> Correctness for Planar<C> {
    fn forest(&self) -> &Forest {
        self.inner.forest()
    }

    fn mate(&self, x: OccId) -> Option<OccId> {
        self.inner.mate(x)
    }

    fn links(&self) -> &[(VertexId, VertexId)] {
        self.inner.links()
    }

    fn is_complete(&self) -> bool {
        self.inner.is_complete()
    }

    /// Rejects a link whose two literals lie in different regions: it
    /// would cross a link made, in every completion. Rejects one between
    /// whose ends some atom has more `a` than `~a`, or fewer: those would
    /// have to link across it. Then asks the criterion wrapped.
    fn admits(&self, x: OccId, y: OccId) -> bool {
        let (left, right) = self.ends(x, y);
        self.region[left as usize] == self.region[right as usize]
            && self.prefix[left as usize + 1] == self.prefix[right as usize]
            && self.inner.admits(x, y)
    }

    /// Links the literals, and moves the places between them that were in
    /// their region into the link's own region.
    fn link(&mut self, x: OccId, y: OccId) -> Result<(), Reason> {
        let (left, right) = self.ends(x, y);
        // The wrapped criterion first: a refusal leaves both as they were.
        self.inner.link(x, y)?;
        let outer = self.region[left as usize];
        for region in &mut self.region[left as usize + 1..right as usize] {
            if *region == outer {
                *region = left;
            }
        }
        self.made.push((left, right, outer));
        Ok(())
    }

    /// Takes the last link back, and with it its region, and returns the
    /// literal it was made from.
    fn unlink(&mut self) -> OccId {
        let (left, right, outer) = self.made.pop().expect("a link to take back");
        for region in &mut self.region[left as usize + 1..right as usize] {
            if *region == left {
                *region = outer;
            }
        }
        self.inner.unlink()
    }

    /// Runs the wrapped criterion's exact test. No link made crosses
    /// another, so a complete linking that passes is a planar net of that
    /// criterion.
    fn test(&mut self) -> bool {
        let correct = self.inner.test();
        debug_assert!(
            !self.is_complete() || self.is_planar(),
            "every link was admitted in its region"
        );
        correct
    }

    /// Returns the wrapped criterion's balance. A planar net is a net, so a
    /// component that holds more literals of an atom than the atom has
    /// pairs left has no planar completion either.
    fn balanced(&self, atom: Atom, remaining: u32, tally: &mut [u32]) -> bool {
        self.inner.balanced(atom, remaining, tally)
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use crate::linear::fragment::Mode;
    use crate::linear::nets::{Criterion, ProofStructure, VertexId};
    use crate::linear::occurrences::{Forest, OccId, Sign};
    use crate::linear::search::generate::{self, Lambek, Rng, Rules, Tree};
    use crate::linear::search::{Engine, Options, Verdict, prove};
    use crate::linear::sequents::{Atom, Sequent};

    /// The formulas of a random derivation of cyclic MLL in their cyclic
    /// order, with up to `budget` rules over `atoms` atoms: an axiom, a
    /// `⅋` of two neighbours, or a `⊗` of the last formula of one
    /// derivation and the first of another, each sequence turned at
    /// random as the cyclic calculus allows.
    fn cyclic(rng: &mut Rng, atoms: u8, budget: usize) -> Vec<Tree> {
        let turned = |rng: &mut Rng, mut formulas: Vec<Tree>| {
            let by = rng.below(formulas.len());
            formulas.rotate_left(by);
            formulas
        };
        let mut formulas = if budget <= 1 || rng.below(4) == 0 {
            let a = rng.below(usize::from(atoms)) as u8;
            vec![Tree::Atom(a), Tree::Dual(a)]
        } else if rng.below(2) == 0 {
            let mut left = cyclic(rng, atoms, budget / 2);
            let mut right = cyclic(rng, atoms, budget - budget / 2);
            let a = left.pop().unwrap();
            let b = right.remove(0);
            left.push(Tree::Tensor(Box::new(a), Box::new(b)));
            left.extend(right);
            left
        } else {
            let mut formulas = cyclic(rng, atoms, budget - 1);
            if formulas.len() > 2 {
                let b = formulas.pop().unwrap();
                let a = formulas.pop().unwrap();
                formulas.push(Tree::Par(Box::new(a), Box::new(b)));
            }
            formulas
        };
        formulas = turned(rng, formulas);
        formulas
    }

    /// Whether some planar linking of the literals of `positives` to
    /// unlinked partners completes `net` into a proof net of the
    /// commutative criterion: every linking is tried that has no two
    /// links whose ends alternate, compared pairwise.
    fn some_planar_net(net: &mut ProofStructure, positives: &[OccId]) -> bool {
        let Some((&x, rest)) = positives.split_first() else {
            return net.is_correct(|_| false).is_ok();
        };
        let forest = net.forest().clone();
        for &y in forest.literals(forest.atom(x).unwrap(), Sign::Dual) {
            let (u, v) = (x.min(y), x.max(y));
            let crosses = net.links().iter().any(|&(p, q)| {
                let (p, q) = (p.occ().min(q.occ()), p.occ().max(q.occ()));
                (u < p && p < v && v < q) || (p < u && u < q && q < v)
            });
            if net.partner(VertexId::of(y)).is_none() && !crosses {
                net.link(VertexId::of(x), VertexId::of(y)).unwrap();
                let found = some_planar_net(net, rest);
                net.unlink();
                if found {
                    return true;
                }
            }
        }
        false
    }

    /// Whether some planar linking of the sequent is a proof net, or
    /// `None` when an atom has more than six pairs or its counts differ.
    fn every_planar_linking(sequent: &Sequent) -> Option<bool> {
        let forest = Forest::new(sequent).unwrap();
        let mut positives = Vec::new();
        for a in 0..sequent.atom_names().len() as u32 {
            let a = Atom::new(a);
            let (p, q) = (
                forest.literals(a, Sign::Atom),
                forest.literals(a, Sign::Dual),
            );
            if p.len() != q.len() || p.len() > 6 {
                return None;
            }
            positives.extend_from_slice(p);
        }
        let mut net = ProofStructure::new(forest, Criterion::MLL).unwrap();
        Some(some_planar_net(&mut net, &positives))
    }

    /// The planar engine, the default of the cyclic mode, agrees with the
    /// enumeration of every planar linking on random derivations of the
    /// cyclic calculus, their mutants and random balanced sequents, every
    /// proof through the checker in the cyclic mode and its net planar;
    /// on the same sequents the classical verdict is never below it.
    #[test]
    fn agrees_with_every_planar_linking() {
        let rules = Rules {
            units: false,
            additives: false,
            mix: false,
            exponentials: false,
        };
        let mut rng = Rng::new(3_600);
        let (mut decided, mut provable, mut only_classically) = (0, 0, 0);
        for round in 0..900 {
            let formulas = match round % 3 {
                0 => {
                    let (atoms, budget) = (1 + rng.below(3) as u8, 1 + rng.below(9));
                    cyclic(&mut rng, atoms, budget)
                }
                1 => {
                    let (atoms, budget) = (1 + rng.below(3) as u8, 1 + rng.below(9));
                    let mut formulas = cyclic(&mut rng, atoms, budget);
                    generate::mutate(&mut rng, &mut formulas, 3);
                    formulas
                }
                _ => {
                    let (atoms, pairs) = (1 + rng.below(2) as u8, 1 + rng.below(6));
                    let mut formulas = generate::balanced(&mut rng, atoms, pairs, false);
                    if rng.below(2) == 0 {
                        let budget = 2 + rng.below(6);
                        formulas = generate::provable(&mut rng, rules, 2, budget).formulas;
                    }
                    formulas
                }
            };
            let text = generate::sequent(&formulas);
            let sequent: Sequent = text.parse().unwrap();
            let Some(expected) = every_planar_linking(&sequent) else {
                continue;
            };
            let outcome = prove(&sequent, Mode::CYCLIC, &Options::default()).unwrap();
            assert_eq!(outcome.engine, Engine::Net, "{text}");
            let found = match outcome.verdict {
                Verdict::Proved(proof) => {
                    assert_eq!(proof.check(Mode::CYCLIC), Ok(()), "{text}");
                    let net = outcome.net.expect("the net engine's net");
                    assert_eq!(net.is_correct(|_| false), Ok(()), "{text}");
                    assert_eq!(net.criterion(), Criterion::CYCLIC, "{text}");
                    true
                }
                Verdict::Unprovable(_) => false,
                Verdict::Unknown(reason) => panic!("{text}: {reason}"),
            };
            assert_eq!(found, expected, "{text}");
            let classical = prove(&sequent, Mode::CLASSICAL, &Options::default()).unwrap();
            let classically = matches!(classical.verdict, Verdict::Proved(_));
            assert!(classically || !found, "{text}");
            decided += 1;
            provable += usize::from(found);
            only_classically += usize::from(classically && !found);
        }
        assert!(
            decided > 600 && provable > 200 && only_classically > 50,
            "{decided} decided, {provable} provable, {only_classically} only classically"
        );
    }

    /// A random formula over `atoms` atoms of up to `budget` connectives.
    fn formula(rng: &mut Rng, atoms: u8, budget: usize) -> Lambek {
        let b = Box::new;
        if budget == 0 || rng.below(3) == 0 {
            return Lambek::Atom(rng.below(usize::from(atoms)) as u8);
        }
        let left = rng.below(budget);
        let (x, y) = (
            formula(rng, atoms, left),
            formula(rng, atoms, budget - 1 - left),
        );
        match rng.below(3) {
            0 => Lambek::Tensor(b(x), b(y)),
            1 => Lambek::Under(b(x), b(y)),
            _ => Lambek::Over(b(x), b(y)),
        }
    }

    /// Whether `hypotheses ⊢ goal` is derivable in the cut-free sequent
    /// calculus of the Lambek calculus, with empty antecedents where
    /// `star` is set: every rule tried at every place, each premise
    /// smaller than its conclusion.
    fn derivable(hypotheses: &[Lambek], goal: &Lambek, star: bool) -> bool {
        use Lambek::*;
        if hypotheses.is_empty() && !star {
            return false;
        }
        if let [h] = hypotheses
            && h == goal
            && matches!(goal, Atom(_))
        {
            return true;
        }
        let with = |parts: &[&[Lambek]]| parts.concat();
        let right = match goal {
            Atom(_) => false,
            Tensor(a, c) => (0..=hypotheses.len()).any(|k| {
                let (g, d) = hypotheses.split_at(k);
                derivable(g, a, star) && derivable(d, c, star)
            }),
            Under(a, c) => derivable(&with(&[std::slice::from_ref(a), hypotheses]), c, star),
            Over(c, a) => derivable(&with(&[hypotheses, std::slice::from_ref(a)]), c, star),
        };
        right
            || (0..hypotheses.len()).any(|i| {
                let (before, after) = (&hypotheses[..i], &hypotheses[i + 1..]);
                match &hypotheses[i] {
                    Atom(_) => false,
                    Tensor(a, c) => {
                        let premise = with(&[before, &[(**a).clone(), (**c).clone()], after]);
                        derivable(&premise, goal, star)
                    }
                    Under(a, c) => (0..=i).any(|j| {
                        derivable(&hypotheses[j..i], a, star)
                            && derivable(
                                &with(&[&hypotheses[..j], &[(**c).clone()], after]),
                                goal,
                                star,
                            )
                    }),
                    Over(c, a) => (i + 1..=hypotheses.len()).any(|k| {
                        derivable(&hypotheses[i + 1..k], a, star)
                            && derivable(
                                &with(&[before, &[(**c).clone()], &hypotheses[k..]]),
                                goal,
                                star,
                            )
                    }),
                }
            })
    }

    /// Whether some planar linking of the literals of `positives` to
    /// unlinked partners completes `net` into an essential net whose
    /// sequentialization the checker accepts in `mode`, every linking
    /// tried that has no two links whose ends alternate; on the way, that
    /// the essential and the Danos–Regnier verdicts agree on each.
    fn some_planar_essential_net(
        net: &mut ProofStructure,
        positives: &[OccId],
        mode: Mode,
    ) -> bool {
        let Some((&x, rest)) = positives.split_first() else {
            let essential = net.is_essential(|_| false).is_ok();
            assert_eq!(essential, net.is_correct(|_| false).is_ok(), "{net}");
            return essential && {
                let proof = net
                    .sequentialize(&crate::Limits::default(), |_| false)
                    .unwrap();
                proof.check(mode).is_ok()
            };
        };
        let forest = net.forest().clone();
        for &y in forest.literals(forest.atom(x).unwrap(), Sign::Dual) {
            let (u, v) = (x.min(y), x.max(y));
            let crosses = net.links().iter().any(|&(p, q)| {
                let (p, q) = (p.occ().min(q.occ()), p.occ().max(q.occ()));
                (u < p && p < v && v < q) || (p < u && u < q && q < v)
            });
            if net.partner(VertexId::of(y)).is_none() && !crosses {
                net.link(VertexId::of(x), VertexId::of(y)).unwrap();
                let found = some_planar_essential_net(net, rest, mode);
                net.unlink();
                if found {
                    return true;
                }
            }
        }
        false
    }

    /// The essential engine, the default of the Lambek calculus, agrees
    /// with the enumeration of every planar essential linking whose
    /// sequentialization the checker accepts, and with the cut-free
    /// sequent calculus, with empty antecedents and without, on random
    /// derivations of either calculus, their mutants and random
    /// sequents of at most six pairs per atom; every proof passes the
    /// checker in its mode and its net is planar and essential.
    #[test]
    fn lambek_agrees_with_every_planar_essential_linking() {
        use Lambek::*;
        use generate::{lambek as derivation, lambek_text as text};
        let b = Box::new;
        let (a, c, e) = (Atom(0), Atom(1), Atom(3));
        // Divisions whose bodies have a hypothesis bound by an outer one,
        // or only one bound inside: `e ⊢ ((c \ (c ⊗ a)) / a) ⊗ e` and
        // `e ⊢ e ⊗ (c \ (a \ (a ⊗ c)))` and `e ⊢ e ⊗ (a \ a)` need
        // empty antecedents, `e ⊢ (e ⊗ (c \ (c ⊗ a))) / a` does not.
        let inner = || Under(b(c.clone()), b(Tensor(b(c.clone()), b(a.clone()))));
        let fixed = [
            Tensor(b(Over(b(inner()), b(a.clone()))), b(e.clone())),
            Tensor(
                b(e.clone()),
                b(Under(
                    b(c.clone()),
                    b(Under(b(a.clone()), b(Tensor(b(a.clone()), b(c.clone()))))),
                )),
            ),
            Over(b(Tensor(b(e.clone()), b(inner()))), b(a.clone())),
            Tensor(b(e.clone()), b(Under(b(a.clone()), b(a.clone())))),
        ];
        let mut rng = Rng::new(3_601);
        let (mut decided, mut provable, mut only_with_empty) = (0, 0, 0);
        for round in 0..800 + fixed.len() {
            let star = round % 2 == 1;
            let atoms = 1 + rng.below(3) as u8;
            let budget = 1 + rng.below(8);
            let (mut hypotheses, mut goal) = derivation(&mut rng, atoms, budget, star);
            if let Some(fixed) = fixed.get(round) {
                (hypotheses, goal) = (vec![e.clone()], fixed.clone());
            }
            match round % 5 {
                _ if round < fixed.len() => {}
                0 | 1 => {}
                2 if hypotheses.len() >= 2 => {
                    let i = rng.below(hypotheses.len() - 1);
                    hypotheses.swap(i, i + 1);
                }
                3 => {
                    let flip = |f: Lambek| match f {
                        Under(a, c) => Over(c, a),
                        Over(c, a) => Under(a, c),
                        other => other,
                    };
                    goal = flip(goal);
                    if let Some(h) = hypotheses.pop() {
                        hypotheses.push(flip(h));
                    }
                }
                _ => {
                    let n = 1 + rng.below(3);
                    hypotheses = (0..n)
                        .map(|_| {
                            let size = rng.below(3);
                            formula(&mut rng, atoms, size)
                        })
                        .collect();
                    let size = rng.below(4);
                    goal = formula(&mut rng, atoms, size);
                }
            }
            let input = text(&hypotheses, &goal);
            let sequent = Sequent::parse_ordered(
                &input,
                crate::linear::Order::Cyclic,
                &crate::Limits::default(),
            )
            .unwrap();
            let forest = Forest::new(&sequent).unwrap();
            let mut positives = Vec::new();
            let mut balanced = true;
            for a in 0..sequent.atom_names().len() as u32 {
                let (p, q) = (
                    forest.literals(crate::linear::sequents::Atom::new(a), Sign::Atom),
                    forest.literals(crate::linear::sequents::Atom::new(a), Sign::Dual),
                );
                balanced &= p.len() == q.len() && p.len() <= 6;
                positives.extend_from_slice(p);
            }
            for mode in [Mode::LAMBEK, Mode::LAMBEK.with_empty_antecedents()] {
                let star = mode.allows_empty_antecedents();
                let expected = derivable(&hypotheses, &goal, star);
                if hypotheses.is_empty() && !star {
                    let error = prove(&sequent, mode, &Options::default()).unwrap_err();
                    assert!(
                        matches!(error, crate::Error::NotIntuitionistic(_)),
                        "{input}"
                    );
                    continue;
                }
                if balanced {
                    let mut net = ProofStructure::new(forest.clone(), Criterion::CYCLIC).unwrap();
                    let enumerated = some_planar_essential_net(&mut net, &positives, mode);
                    assert_eq!(enumerated, expected, "{input} in {mode} mode");
                }
                let outcome = prove(&sequent, mode, &Options::default()).unwrap();
                assert_eq!(outcome.engine, Engine::Essential, "{input}");
                let found = match outcome.verdict {
                    Verdict::Proved(proof) => {
                        assert_eq!(proof.check(mode), Ok(()), "{input}");
                        let net = outcome.net.expect("the essential engine's net");
                        assert_eq!(net.is_essential(|_| false), Ok(()), "{input}");
                        assert_eq!(net.is_planar(), Ok(()), "{input}");
                        true
                    }
                    Verdict::Unprovable(_) => false,
                    Verdict::Unknown(reason) => panic!("{input}: {reason}"),
                };
                assert_eq!(found, expected, "{input} in {mode} mode");
                decided += 1;
                provable += usize::from(found);
                only_with_empty +=
                    usize::from(found && star && !derivable(&hypotheses, &goal, false));
            }
        }
        assert!(
            decided > 1200 && provable > 500 && only_with_empty > 30,
            "{decided} decided, {provable} provable, {only_with_empty} only with empty antecedents"
        );
    }
}
