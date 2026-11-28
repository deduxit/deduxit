// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The check of a refutation by a basis ([`Refutation::Basis`]), apart
//! from the backward search that found it.
//!
//! The goal is read as a Petri net here again, by code of its own. Each
//! atom is a place, and so is each class of the basis, which holds a
//! ticket per unused clause of the class. A clause under `?` is a
//! transition that fires any number of times; a clause used once also
//! takes a ticket of its class. The initial marking is the goal's
//! markings and every ticket; the target is the goal's own body.
//!
//! A cut-free proof of the goal gives a firing sequence from the initial
//! marking to a marking at least the target, where a step may also drop
//! tokens (a weakening). Weigh each subformula: a body literal and `1`
//! one, a head literal and `⊥` nothing, `⊗` its sides less one, `⅋` its
//! sides, a `?` member nothing. Every rule keeps a sequent's weight or
//! raises it (Mix and a weakening of a weighed formula raise it), the
//! goal weighs one, and so does every sequent of its proof: no Mix, no
//! goal weakened, one goal-like member each. By induction on the proof,
//! its clauses then fire in sequence, each used once at most once. So
//! the target is covered from the initial marking.
//!
//! That covers every proof in every mode: an intuitionistic or ordered
//! proof is a classical one of the one-sided sequent, Mix is admissible
//! with weakening, and classical linear logic, its affine variant and
//! its variant with Mix each have cut elimination, so a proof with a cut
//! has a cut-free one. The check therefore does not read the mode.
//!
//! Let `U` be the markings at least one of the basis. When `U` holds every
//! marking at least the target and, with a marking `m`, every marking
//! from which a clause leads to `m`, then `U` holds every marking of the
//! sequence, read from its end, the initial one included: a drop leads to
//! a smaller marking, and `U` is closed upward. The check asks for each
//! of these, so a basis it accepts, whose `U` lacks the initial marking,
//! proves the goal unprovable.
//!
//! The marking from which clause `t` leads to one at least `m` is at
//! least `max(m − out(t), 0) + in(t)`. Where `t` has no output on `m`'s
//! places, that is `m` plus the inputs, which `m` itself is at most, so
//! only the clauses with an output there are tried. A clause that can
//! never fire is not tried either: a place no firing sequence from the
//! initial marking marks keeps every clause that takes from it unfired.
//!
//! [`Refutation::Basis`]: super::Refutation::Basis

use super::refutation::{Basis, Failed, Marking, RefutationFault};
use super::weighted::{HornMember, horn_member};
use crate::hash::HashMap;
use crate::linear::occurrences::{Forest, OccId, Sign};

/// The invariant the faults of the check name.
const INVARIANT: &str = "basis of the coverable markings";

/// A transition's arcs: the places it takes from and gives to, each with
/// its weight, by increasing place. A weight counts literals of one
/// clause, fewer than the forest's 2³² occurrences, or is one ticket.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Arcs {
    /// The places the transition takes from.
    inputs: Vec<(usize, u64)>,
    /// The places the transition gives to.
    outputs: Vec<(usize, u64)>,
}

/// The goal read as a Petri net over the sequent's atoms.
struct Net {
    /// The clauses under `?`.
    reusable: Vec<Arcs>,
    /// The clauses used once, by occurrence.
    once: Vec<(OccId, Arcs)>,
    /// The tokens of the goal's markings, per atom: a count of literals of
    /// the goal, below 2³².
    initial: Vec<u64>,
    /// The goal's body, per atom, a count of literals as `initial` is.
    target: Vec<u64>,
}

/// Returns the atoms, with their counts, of the literals of the sign
/// `sign` in the subtree of `o`, by increasing atom.
fn counted(forest: &Forest, o: OccId, sign: Sign) -> Vec<(usize, u64)> {
    runs(
        forest
            .subtree(o)
            .filter(|&x| forest.sign(x) == Some(sign))
            .map(|x| forest.atom(x).expect("a literal has an atom").index())
            .collect(),
    )
}

/// Returns the atoms of `atoms`, with how often each occurs, by increasing
/// atom.
fn runs(mut atoms: Vec<usize>) -> Vec<(usize, u64)> {
    atoms.sort_unstable();
    let mut counts: Vec<(usize, u64)> = Vec::new();
    for a in atoms {
        match counts.last_mut() {
            Some((last, n)) if *last == a => *n += 1,
            _ => counts.push((a, 1)),
        }
    }
    counts
}

/// Reads the goal as a Horn program whose bodies' literals have the sign
/// `body`, or returns `None` where it is none.
///
/// A head is a `⅋` and `⊥` tree of literals of the other sign. A clause
/// is a `⊗` tree of body literals, `1` and at most one head. A member
/// under `?` is a clause with a head. Any other member is a head (a
/// marking), a clause with a head (used once), or the goal, a clause
/// without one, of which there is exactly one.
fn read(forest: &Forest, goal: &[OccId], body: Sign) -> Option<Net> {
    let atoms = forest.sequent().atom_names().len();
    let mut net = Net {
        reusable: Vec::new(),
        once: Vec::new(),
        initial: vec![0; atoms],
        target: vec![0; atoms],
    };
    let mut goal_found = false;
    for &member in goal {
        let HornMember::Clause {
            reusable,
            inputs,
            head,
        } = horn_member(forest, member, body)?
        else {
            for (a, n) in counted(forest, member, !body) {
                net.initial[a] += n;
            }
            continue;
        };
        let arcs = Arcs {
            inputs: runs(inputs),
            outputs: head.map_or_else(Vec::new, |h| counted(forest, h, !body)),
        };
        match (reusable, head.is_some()) {
            (true, true) => net.reusable.push(arcs),
            (true, false) => return None,
            (false, true) => net.once.push((member, arcs)),
            (false, false) if goal_found => return None,
            (false, false) => {
                goal_found = true;
                for &(a, n) in &arcs.inputs {
                    net.target[a] = n;
                }
            }
        }
    }
    goal_found.then_some(net)
}

/// Returns the places of a marking, each with its count: the atoms, then
/// the classes, which follow the `atoms` atoms. Both are by increasing
/// place.
fn entries(marking: &Marking, atoms: usize) -> impl Iterator<Item = (usize, u32)> + '_ {
    marking.atoms.iter().map(|&(a, n)| (a.index(), n)).chain(
        marking
            .tickets
            .iter()
            .map(move |&(k, n)| (atoms + k as usize, n)),
    )
}

/// Returns whether every count of `marking` is at most the count of its
/// place in `counts`.
fn at_most(marking: &Marking, atoms: usize, counts: &[u64]) -> bool {
    entries(marking, atoms).all(|(p, n)| u64::from(n) <= counts[p])
}

/// Returns, for each place, whether some firing sequence from `initial`
/// marks it: the least set that holds the initially marked places and
/// the outputs of every transition whose inputs it holds.
fn live(initial: &[u64], transitions: &[Arcs]) -> Vec<bool> {
    let mut marked: Vec<bool> = initial.iter().map(|&n| n > 0).collect();
    // The inputs each transition still waits for, and the transitions
    // waiting for each place, once per input.
    let mut waiting: Vec<usize> = Vec::with_capacity(transitions.len());
    let mut by_input: Vec<Vec<usize>> = vec![Vec::new(); initial.len()];
    let mut ready = Vec::new();
    for (t, arcs) in transitions.iter().enumerate() {
        let mut unmarked = 0;
        for &(p, _) in &arcs.inputs {
            if !marked[p] {
                unmarked += 1;
                by_input[p].push(t);
            }
        }
        waiting.push(unmarked);
        if unmarked == 0 {
            ready.push(t);
        }
    }
    while let Some(t) = ready.pop() {
        for &(p, _) in &transitions[t].outputs {
            if !marked[p] {
                marked[p] = true;
                for &u in &by_input[p] {
                    waiting[u] -= 1;
                    if waiting[u] == 0 {
                        ready.push(u);
                    }
                }
            }
        }
    }
    marked
}

/// Checks that `basis` proves the goal unprovable, in every mode: that
/// its classes are the goal's clauses used once, each of one body and
/// head; that a marking of it is at most the target and none at most the
/// initial marking; and that it is closed under every clause that can
/// fire, fired backward.
///
/// It asks `stop` after every clause tried, with the units of work since:
/// one for the clause, one per marking of the basis looked at, and one
/// per count of each marking compared. It takes time quadratic in
/// the basis times the clauses, over the markings' places, and memory
/// linear in the places and the clauses beside the basis.
///
/// # Errors
///
/// [`Failed::Fault`] with [`RefutationFault::NotApplicable`] for a
/// sequent with a quantifier or a goal that is no Horn program with
/// bodies of the sign stated, [`RefutationFault::Mismatch`] for classes
/// or markings that are not the goal's, [`RefutationFault::Missed`],
/// [`RefutationFault::Reached`] and [`RefutationFault::Open`] for a basis
/// that does not rule a proof out, and [`Failed::Stopped`] when `stop`
/// answered true.
pub(crate) fn check(
    forest: &Forest,
    goal: &[OccId],
    basis: &Basis,
    stop: &mut dyn FnMut(u64) -> bool,
) -> Result<(), Failed> {
    if forest.has_binders() {
        return Err(RefutationFault::NotApplicable {
            invariant: INVARIANT,
            why: "it is of propositional Horn programs, and the sequent has a quantifier",
        }
        .into());
    }
    let net = read(forest, goal, basis.body).ok_or(RefutationFault::NotApplicable {
        invariant: INVARIANT,
        why: "the goal is no Horn program with bodies of the sign stated",
    })?;
    let atoms = net.initial.len();
    // An atom's index is a `u32`, and there are fewer classes than the
    // forest's 2³² occurrences, but on a 32-bit target the two together
    // could pass a `usize`.
    let places = atoms
        .checked_add(basis.classes.len())
        .ok_or(RefutationFault::Mismatch)?;
    let mut transitions = net.reusable;
    // The classes must be the clauses used once, each as often as the goal
    // names it, each class of one body and head. Its transition also takes
    // a ticket.
    let mut index: HashMap<OccId, Vec<usize>> = HashMap::default();
    for (i, &(o, _)) in net.once.iter().enumerate() {
        index.entry(o).or_default().push(i);
    }
    for (k, class) in basis.classes.iter().enumerate() {
        let mut arcs: Option<&Arcs> = None;
        for o in class {
            let i = index
                .get_mut(o)
                .and_then(Vec::pop)
                .ok_or(RefutationFault::Mismatch)?;
            if arcs.is_some_and(|arcs| *arcs != net.once[i].1) {
                return Err(RefutationFault::Mismatch.into());
            }
            arcs = Some(&net.once[i].1);
        }
        let mut arcs = arcs.ok_or(RefutationFault::Mismatch)?.clone();
        // The class's place follows every atom's, so the inputs stay
        // sorted.
        arcs.inputs.push((atoms + k, 1));
        transitions.push(arcs);
    }
    if index.values().any(|left| !left.is_empty()) {
        return Err(RefutationFault::Mismatch.into());
    }
    // The tickets: a class's clauses, fewer than the forest's occurrences.
    let mut initial = net.initial;
    initial.extend(basis.classes.iter().map(|class| class.len() as u64));
    let mut target = net.target;
    target.resize(places, 0);
    for marking in &basis.markings {
        let atoms_in_order = marking.atoms.windows(2).all(|w| w[0].0 < w[1].0);
        let tickets_in_order = marking.tickets.windows(2).all(|w| w[0].0 < w[1].0);
        // Read before any place is computed: a ticket's place is `atoms`
        // past its class, which a class index past the basis's could take
        // beyond a 32-bit `usize`.
        let counted = marking.atoms.iter().all(|&(_, n)| n > 0)
            && marking.tickets.iter().all(|&(_, n)| n > 0);
        let in_range = marking.atoms.last().is_none_or(|&(a, _)| a.index() < atoms)
            && marking
                .tickets
                .last()
                .is_none_or(|&(k, _)| (k as usize) < basis.classes.len());
        if !(atoms_in_order && tickets_in_order && counted && in_range) {
            return Err(RefutationFault::Mismatch.into());
        }
    }
    let markings = &basis.markings;
    if !markings.iter().any(|m| at_most(m, atoms, &target)) {
        return Err(RefutationFault::Missed.into());
    }
    if markings.iter().any(|m| at_most(m, atoms, &initial)) {
        return Err(RefutationFault::Reached.into());
    }
    // Only the clauses that can fire, indexed by their output places.
    let marked = live(&initial, &transitions);
    let mut by_output: Vec<Vec<usize>> = vec![Vec::new(); places];
    for (t, arcs) in transitions.iter().enumerate() {
        if arcs.inputs.iter().all(|&(p, _)| marked[p]) {
            for &(p, _) in &arcs.outputs {
                by_output[p].push(t);
            }
        }
    }
    // Each marking's tokens, which a marking at most another has no more
    // of. A sum of fewer than 2³³ places of counts below 2³² each may pass
    // a `u64`, so it saturates, and a saturated sum only lets more
    // markings be compared.
    let sum = |counts: &mut dyn Iterator<Item = u64>| counts.fold(0u64, u64::saturating_add);
    let sums: Vec<u64> = markings
        .iter()
        .map(|m| sum(&mut entries(m, atoms).map(|(_, n)| u64::from(n))))
        .collect();
    // The marking a clause fired backward needs, as a count per place:
    // a marking's count below 2³² plus a clause's weight below 2³², so
    // below 2³³.
    let mut probe = vec![0u64; places];
    let mut touched: Vec<usize> = Vec::new();
    // The marking each clause was last tried on, plus one.
    let mut tried = vec![0usize; transitions.len()];
    for (i, m) in markings.iter().enumerate() {
        for (p, _) in entries(m, atoms) {
            for &t in &by_output[p] {
                if tried[t] == i + 1 {
                    continue;
                }
                tried[t] = i + 1;
                let arcs = &transitions[t];
                for (q, n) in entries(m, atoms) {
                    probe[q] = u64::from(n);
                    touched.push(q);
                }
                for &(q, w) in &arcs.outputs {
                    probe[q] = probe[q].saturating_sub(w);
                }
                for &(q, w) in &arcs.inputs {
                    probe[q] += w;
                    touched.push(q);
                }
                let needed = sum(&mut touched.iter().map(|&q| probe[q]));
                let mut units: u64 = 1;
                let found = markings.iter().zip(&sums).any(|(other, &s)| {
                    units = units.saturating_add(1);
                    if s > needed {
                        return false;
                    }
                    let counts = other.atoms.len() + other.tickets.len();
                    units = units.saturating_add(counts as u64);
                    at_most(other, atoms, &probe)
                });
                for &q in &touched {
                    probe[q] = 0;
                }
                touched.clear();
                if !found {
                    return Err(RefutationFault::Open.into());
                }
                if stop(units) {
                    return Err(Failed::Stopped);
                }
            }
        }
    }
    Ok(())
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::linear::search::{Engine, Options, Refutation, Verdict, prove};
    use crate::linear::{Mode, Sequent};

    /// Returns the basis the Horn engine gives for an affine sequent it
    /// refutes by its backward search.
    fn basis_of(text: &str) -> (Forest, Basis) {
        let sequent: Sequent = text.parse().unwrap();
        let options = Options::default().with_engine(Some(Engine::Horn));
        let outcome = prove(&sequent, Mode::CLASSICAL.with_affine(), &options).unwrap();
        let Verdict::Unprovable(disproof) = outcome.verdict else {
            panic!("{text} is unprovable");
        };
        let Refutation::Basis(basis) = disproof.refutation().clone() else {
            panic!("{text}: {:?}", disproof.refutation());
        };
        (Forest::new(&sequent).unwrap(), basis)
    }

    /// Checks `basis` for the roots of `forest`.
    fn checked(forest: &Forest, basis: &Basis) -> Result<(), Failed> {
        check(forest, forest.roots(), basis, &mut |_| false)
    }

    /// A net whose target no firing covers, though its state equation has
    /// a solution, its classical reading is valid and its atoms' counts
    /// say nothing: the clause that makes `c` needs `a` and `b` at once,
    /// and the one `b` there can be costs the one `a`.
    const SHORT: &str = "a, a -o b, !(a * b -o a * b * c) |- c";

    #[test]
    fn the_engine_s_basis_is_checked() {
        let (forest, basis) = basis_of(SHORT);
        assert!(checked(&forest, &basis).is_ok());
        assert_eq!(basis.classes.len(), 1);
    }

    /// Returns the fault of `basis`'s check.
    fn fault(forest: &Forest, basis: &Basis) -> RefutationFault {
        match checked(forest, basis) {
            Err(Failed::Fault(fault)) => fault,
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn a_planted_wrong_basis_is_refused() {
        let (forest, basis) = basis_of(SHORT);
        let atom = |name: &str| forest.sequent().atom(name).unwrap();
        // Without the target's marking the basis says nothing of it.
        let mut missed = basis.clone();
        missed
            .markings
            .retain(|m| m.atoms != vec![(atom("c"), 1)] || !m.tickets.is_empty());
        assert_eq!(fault(&forest, &missed), RefutationFault::Missed);
        // A marking at most the initial one, a single `a`.
        let mut reached = basis.clone();
        reached
            .markings
            .push(Marking::new(vec![(atom("a"), 1)], Vec::new()));
        assert_eq!(fault(&forest, &reached), RefutationFault::Reached);
        // The target alone: the clause that makes `c` needs a marking
        // with an `a` and a `b`.
        let mut open = basis.clone();
        open.markings
            .retain(|m| m.atoms == vec![(atom("c"), 1)] && m.tickets.is_empty());
        assert_eq!(open.markings.len(), 1);
        assert_eq!(fault(&forest, &open), RefutationFault::Open);
        // A count of zero, an atom past the sequent's, atoms out of order
        // and a class past the basis's are not the goal's.
        let mut zero = basis.clone();
        zero.markings[0].atoms.push((atom("c"), 0));
        assert_eq!(fault(&forest, &zero), RefutationFault::Mismatch);
        let mut past = basis.clone();
        past.markings[0]
            .atoms
            .push((crate::linear::sequents::Atom::new(99), 1));
        assert_eq!(fault(&forest, &past), RefutationFault::Mismatch);
        let mut reversed = basis.clone();
        reversed.markings[0].atoms = vec![(atom("c"), 1), (atom("a"), 1)];
        assert_eq!(fault(&forest, &reversed), RefutationFault::Mismatch);
        let mut class = basis.clone();
        class.markings[0].tickets = vec![(1, 1)];
        assert_eq!(fault(&forest, &class), RefutationFault::Mismatch);
        // A class index that would take a ticket's place past a 32-bit
        // `usize` is refused before any place is computed.
        let mut far = basis.clone();
        far.markings[0].tickets = vec![(u32::MAX, 1)];
        assert_eq!(fault(&forest, &far), RefutationFault::Mismatch);
        // The other sign reads no Horn program here.
        let mut other = basis;
        other.body = !other.body;
        assert!(matches!(
            fault(&forest, &other),
            RefutationFault::NotApplicable { .. }
        ));
    }

    #[test]
    fn classes_must_be_the_clauses_used_once() {
        // Two interchangeable clauses used once, one class of them.
        let (forest, basis) = basis_of("a, a -o b, a -o b, !(a * b -o a * b * c) |- c");
        assert!(checked(&forest, &basis).is_ok());
        assert_eq!(basis.classes.len(), 1);
        assert_eq!(basis.classes[0].len(), 2);
        // A clause missing, a clause twice, and a member that is none.
        let mut missing = basis.clone();
        missing.classes[0].pop();
        assert_eq!(fault(&forest, &missing), RefutationFault::Mismatch);
        let mut twice = basis.clone();
        let first = twice.classes[0][0];
        twice.classes[0].push(first);
        assert_eq!(fault(&forest, &twice), RefutationFault::Mismatch);
        let mut stranger = basis.clone();
        stranger.classes[0][0] = forest.roots()[0];
        assert_eq!(fault(&forest, &stranger), RefutationFault::Mismatch);
        // Two clauses of different bodies in one class.
        let (forest, basis) = basis_of("a, a -o b, a * a -o b, !(a * b -o a * b * c) |- c");
        assert_eq!(basis.classes.len(), 2);
        let mut merged = basis;
        merged.classes = vec![merged.classes.concat()];
        merged.markings.clear();
        assert_eq!(fault(&forest, &merged), RefutationFault::Mismatch);
    }

    #[test]
    fn a_member_named_twice_is_two_clauses() {
        // A goal may repeat a member: `a -o b` named twice is two clauses
        // used once, one class of the same occurrence twice.
        let sequent: Sequent = SHORT.parse().unwrap();
        let forest = Forest::new(&sequent).unwrap();
        let roots = forest.roots();
        let members = [0, 1, 1, 2, 3].map(|i| crate::linear::Member::from(roots[i]));
        let goal = crate::linear::search::Goal::new(&forest, &members).unwrap();
        let options = Options::default().with_engine(Some(Engine::Horn));
        let outcome = crate::linear::search::prove_goal(
            goal,
            Mode::CLASSICAL.with_affine(),
            &options,
            &crate::Limits::default(),
            |_| false,
        )
        .unwrap();
        let Verdict::Unprovable(disproof) = outcome.verdict else {
            panic!("{:?}", outcome.verdict);
        };
        let Refutation::Basis(basis) = disproof.refutation() else {
            panic!("{:?}", disproof.refutation());
        };
        assert_eq!(basis.classes, vec![vec![roots[1], roots[1]]]);
        assert_eq!(disproof.check(), Ok(()));
    }

    #[test]
    fn the_check_asks_the_stop() {
        let (forest, basis) = basis_of(SHORT);
        let mut asked = 0;
        let result = check(&forest, forest.roots(), &basis, &mut |_| {
            asked += 1;
            true
        });
        assert!(matches!(result, Err(Failed::Stopped)));
        assert_eq!(asked, 1);
    }
}
