// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The Horn engine's tests.

#![cfg(feature = "parse")]

use super::equation::{Equation, certify};
use super::{Program, cover, proof, reach};
use crate::Error;
use crate::linear::families::FAMILIES;
use crate::linear::fragment::Mode;
use crate::linear::occurrences::Forest;
use crate::linear::search::memory::Account;
use crate::linear::search::{Engine, Options, Reason, Refutation, Task, Verdict, prove};
use crate::linear::sequents::{Atom, Sequent};

/// Returns the options that force the engine.
fn horn() -> Options {
    Options::default().with_engine(Some(Engine::Horn))
}

/// Returns the program of a sequent read in classical mode.
fn program(forest: &Forest) -> Program {
    let task = Task {
        forest,
        goal: forest.roots(),
        fragment: forest.sequent().fragment(),
        mode: Mode::CLASSICAL,
        reading: None,
        roots: true,
        program: std::sync::OnceLock::new(),
    };
    Program::read(&task).expect("a Horn program")
}

/// The engine decides the generated families that are Horn programs, in
/// the mode each instance is made for, and the counters in
/// intuitionistic mode as well. Every proof passes the checker.
#[test]
fn decides_the_horn_families() {
    for (name, sizes) in [
        ("counter", &[2, 4, 8, 16, 64][..]),
        ("counter-over", &[2, 4, 8, 16, 64]),
        ("partition-yes", &[2, 4, 6]),
        ("partition-no", &[3, 4, 5]),
    ] {
        let family = FAMILIES.iter().find(|f| f.name == name).unwrap();
        for &size in sizes {
            let instance = family.instance(size, 0).unwrap();
            let mut modes = vec![instance.mode];
            if name.starts_with("counter") {
                modes.push(Mode::INTUITIONISTIC);
            }
            for mode in modes {
                let outcome = prove(&instance.sequent, mode, &horn()).unwrap();
                let proved = match outcome.verdict {
                    Verdict::Proved(_) => true,
                    Verdict::Unprovable(_) => false,
                    Verdict::Unknown(reason) => panic!("{}: {reason}", instance.name),
                };
                assert_eq!(
                    proved, instance.provable,
                    "{} in {mode} mode",
                    instance.name
                );
            }
        }
    }
}

/// A goal that is no Horn program, or lies beyond MELL, is refused with
/// the error a forced engine gets.
#[test]
fn refuses_what_it_does_not_decide() {
    for text in [
        "a & b |- a",
        "!(a -o a * a), a |- ?b",
        "|- a, ~a, b, ~b",
        "!(a -o b) |- a -o b",
    ] {
        let sequent: Sequent = text.parse().unwrap();
        let error = prove(&sequent, Mode::CLASSICAL, &horn()).unwrap_err();
        assert!(
            matches!(
                error,
                Error::EngineRefused {
                    engine: Engine::Horn,
                    because: crate::linear::search::NotTaken::Shape
                        | crate::linear::search::NotTaken::Fragment { .. },
                }
            ),
            "{text}: {error}"
        );
    }
}

/// Returns the verdict of a sequent under the engine as a word, which
/// names a refutation by the state equation apart.
fn verdict(text: &str, mode: Mode) -> &'static str {
    let sequent: Sequent = text.parse().unwrap();
    match prove(&sequent, mode, &horn()).unwrap().verdict {
        Verdict::Proved(_) => "proved",
        Verdict::Unprovable(d) if matches!(d.refutation(), Refutation::StateEquation(_)) => {
            "state equation"
        }
        Verdict::Unprovable(_) => "unprovable",
        Verdict::Unknown(reason) => panic!("{text} in {mode} mode: {reason}"),
    }
}

/// The refutation the counts give before any search comes back with the
/// answer, so the front door takes it as it is and does not count again.
/// Here it refutes `c`, the first atom: nothing balances its `~c` outside
/// the `!`, since the `!` holds a `~c` too.
#[test]
fn counted_refutations_are_handed_back() {
    use crate::linear::search::{Decide, OneSided};
    let sequent: Sequent = "c, !c |- a".parse().unwrap();
    let forest = Forest::new(&sequent).unwrap();
    let task = Task {
        forest: &forest,
        goal: forest.roots(),
        fragment: sequent.fragment(),
        mode: Mode::CLASSICAL,
        reading: None,
        roots: true,
        program: std::sync::OnceLock::new(),
    };
    let limits = crate::Limits::default();
    let answer = super::Horn
        .decide(
            &task,
            &horn(),
            &limits,
            &Account::new(None),
            &crate::linear::search::Work::new(None),
            &mut |_| false,
        )
        .unwrap();
    assert!(matches!(answer.result, Ok(None)));
    assert!(
        matches!(
            answer.refutation,
            Some(Refutation::OneSided(OneSided { .. }))
        ),
        "{:?}",
        answer.refutation
    );
}

/// Nets whose markings grow without end are decided.
///
/// In linear mode, the state equation refutes where it has no solution
/// once the transitions that can never fire are left out, or a backward
/// search refutes when its markings run out. In affine mode the backward
/// coverability search decides. It refutes what no firing sequence
/// covers, and proves what one does, with the tokens and clauses left
/// over weakened.
#[test]
fn decides_unbounded_nets() {
    for (text, linear, affine) in [
        // Nothing makes `c`, which the counts see, since every `c` below
        // the `!` is in a body.
        (
            "!d, !((c * d * d) -o 1) |- c * c",
            "unprovable",
            "unprovable",
        ),
        // Nothing makes `c` but a clause that can never fire.
        (
            "!d, !((c * d * d) -o 1), !(e -o c) |- c * c",
            "state equation",
            "unprovable",
        ),
        // The tokens of `A` only grow, and the goal has fewer.
        (
            "!(A -o A * A), !(B * B -o C), A, B |- C",
            "state equation",
            "unprovable",
        ),
        // The clause that makes `b` needs a `c`, which is never marked.
        // Without that clause, nothing makes `b`.
        (
            "!(a -o a * a), !(a * c -o b * c), a |- b",
            "state equation",
            "unprovable",
        ),
        // The equation has a solution, `a` and the clause used once each
        // fired once, but the clause needs two `a`. Backward, the goal has
        // no predecessor.
        ("!(1 -o 1), !a, (a * a -o a) |- 1", "unprovable", "proved"),
        // Covered with an `a`, a `c` and a clause used once to spare.
        (
            "!(a -o a * a), a -o c, c -o b, !(a * a * a -o b), a |- b * b",
            "",
            "proved",
        ),
    ] {
        if !linear.is_empty() {
            assert_eq!(verdict(text, Mode::CLASSICAL), linear, "{text}");
            assert_eq!(verdict(text, Mode::INTUITIONISTIC), linear, "{text}");
        }
        // Whether the backward search or the equation refutes first
        // depends on their shares of the work.
        for mode in [
            Mode::CLASSICAL.with_affine(),
            Mode::INTUITIONISTIC.with_affine(),
        ] {
            let found = match verdict(text, mode) {
                "state equation" => "unprovable",
                found => found,
            };
            assert_eq!(found, affine, "{text} in {mode} mode");
        }
    }
}

/// A refutation by the state equation names every weight that is not
/// zero: the atoms', and a class's ticket under each of its clauses used
/// once. It also names the clauses that can never fire, which the
/// inequalities leave out.
#[test]
fn the_certificate_names_its_clauses() {
    let certificate = |text: &str| {
        let sequent: Sequent = text.parse().unwrap();
        let outcome = prove(&sequent, Mode::CLASSICAL, &horn()).unwrap();
        let Verdict::Unprovable(disproof) = outcome.verdict else {
            panic!("{text}: {:?}", outcome.verdict);
        };
        let Refutation::StateEquation(certificate) = disproof.refutation().clone() else {
            panic!("{text}: {disproof}");
        };
        (Forest::new(&sequent).unwrap(), certificate)
    };
    // The atoms are `a`, `c`, `b`. The clause that needs a `c` is under
    // the second `!`.
    let (forest, dead) = certificate("!(a -o a * a), !(a * c -o b * c), a |- b");
    let needs_c = forest.children(forest.roots()[1]).next().unwrap();
    assert_eq!(dead.atoms, [(Atom::new(0), -1), (Atom::new(2), 1)]);
    assert_eq!((dead.clauses, dead.dropped), (vec![], vec![needs_c]));
    // One clause used once makes one `b`, and the goal asks for two. Its
    // ticket weighs as much as a `b`.
    let (forest, once) = certificate("!(a -o a * a), !(b -o b), (a * a -o b), a |- b * b");
    assert_eq!(once.atoms, [(Atom::new(1), 1)]);
    assert_eq!(
        (once.clauses, once.dropped),
        (vec![(forest.roots()[2], 1)], vec![])
    );
}

/// The check of the weights is exact. Weights whose products wrap in 64
/// bits are no refutation, although wrapping arithmetic would find that
/// the transition lowers the weighted count.
#[test]
fn checks_the_weights_exactly() {
    let sequent: Sequent = "!(a -o b * b), a |- b * b * b".parse().unwrap();
    let forest = Forest::new(&sequent).unwrap();
    let net = program(&forest);
    let (a, b) = (net.place_of[0], net.place_of[1]);
    // `b` weighs `i64::MAX`. The transition raises the count by
    // `2 · i64::MAX − 1`, which wraps to `−3` in 64 bits, and the goal's
    // `3 · i64::MAX − 1` wraps to a number above zero.
    assert!(!certify(&net, false, &[(a, 1), (b, i64::MAX)]));
    // The transition does not raise the count, but the goal does not ask
    // for it raised either.
    assert!(!certify(&net, false, &[(a, 3), (b, 1)]));
    for affine in [false, true] {
        assert!(certify(&net, affine, &[(a, 2), (b, 1)]));
    }
}

/// Every number the engine's answer rests on refuses at its limit
/// instead of wrapping: the markings kept, a token count, the proof's
/// nodes, and the memory bound.
#[test]
fn refuses_at_its_limits() {
    let account = Account::new(None);
    let sequent: Sequent = FAMILIES
        .iter()
        .find(|f| f.name == "counter")
        .unwrap()
        .instance(8, 0)
        .unwrap()
        .sequent;
    let forest = Forest::new(&sequent).unwrap();
    let counter = program(&forest);
    let equation = || Equation::new(false, &account);
    let (found, _) = reach::search(&counter, &account, 2, &mut equation(), &mut |_| false);
    assert_eq!(found, Err(Reason::IndexLimit));
    let (found, _) = reach::search(
        &counter,
        &account,
        reach::MOST_MARKINGS,
        &mut equation(),
        &mut |_| false,
    );
    let firings = found.unwrap().expect("the counter is provable");
    let built = proof::build(&forest, &counter, &firings, false, &account, 3);
    assert_eq!(built.err(), Some(Reason::IndexLimit));
    let affine = || Equation::new(true, &account);
    let (found, _) = cover::search(&counter, &account, 1, &mut affine(), &mut |_| false);
    assert_eq!(found, Err(Reason::IndexLimit));

    // A clause that adds almost 2³² tokens passes the count's bound at
    // its second firing. The state equation, which would refute the goal,
    // has no room here.
    let sequent: Sequent = "!(a -o a * a), a |- b".parse().unwrap();
    let forest = Forest::new(&sequent).unwrap();
    let mut growing = program(&forest);
    let outputs = growing.transitions[0].outputs as usize;
    growing.arcs[outputs].1 = u32::MAX - 1;
    let no_room = Account::new(Some(0));
    let (found, _) = reach::search(
        &growing,
        &account,
        reach::MOST_MARKINGS,
        &mut Equation::new(false, &no_room),
        &mut |_| false,
    );
    assert_eq!(found, Err(Reason::IndexLimit));

    // Backward, a clause that takes almost 2³² tokens and gives one back
    // asks for more than 2³² before its own output. The state equation has
    // a solution, since `c` makes as many `a` as asked.
    let sequent: Sequent = "!(a * a -o a), !(a -o b), !(c -o c * a), c |- b"
        .parse()
        .unwrap();
    let forest = Forest::new(&sequent).unwrap();
    let mut taking = program(&forest);
    for t in 0..2 {
        let inputs = taking.transitions[t].inputs as usize;
        taking.arcs[inputs].1 = u32::MAX - 1;
    }
    let (found, _) = cover::search(
        &taking,
        &account,
        reach::MOST_MARKINGS,
        &mut affine(),
        &mut |_| false,
    );
    assert_eq!(found, Err(Reason::IndexLimit));

    // Only its markings could refute this one, and they grow without end
    // in both directions. The tokens `a` stay odd, so the goal is never
    // reached, but the state equation has a rational solution, and the
    // tokens `d` grow and shrink.
    let sequent: Sequent =
        "!(a -o a * a * a), !(a * a -o b), !(d -o d * d), !(d * d -o d), a, d |- b * d"
            .parse()
            .unwrap();
    let limits = crate::Limits::default().with_memory_bytes(Some(4096));
    let outcome =
        crate::linear::search::prove_within(&sequent, Mode::CLASSICAL, &horn(), &limits, |_| false)
            .unwrap();
    assert!(
        matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::MemoryLimit { limit_bytes: 4096 })
        ),
        "{:?}",
        outcome.verdict
    );
}

/// Returns a sequent's verdict under the engine within a memory bound of
/// `bytes` beside its input. The input is the forest, and the reading in
/// intuitionistic mode.
fn bounded(sequent: &Sequent, mode: Mode, bytes: u64) -> Verdict {
    let forest = Forest::new(sequent).unwrap();
    let reading = if mode.intuitionistic {
        forest.len() * size_of::<crate::linear::occurrences::Side>()
    } else {
        0
    };
    let input = (forest.held_bytes() + reading) as u64;
    let limits = crate::Limits::default().with_memory_bytes(Some(bytes + input));
    crate::linear::search::prove_within(sequent, mode, &horn(), &limits, |_| false)
        .unwrap()
        .verdict
}

/// The backward search tries every transition on every element it takes.
/// Elements leave the queue by their tokens, not by their index, so an
/// element kept later may be taken earlier. A transition already tried on
/// the later one must still be tried on the earlier one. Here that firing
/// is the step that covers the goal.
#[test]
fn tries_every_transition_on_every_element() {
    let text = "(c * b -o c * c * b), (1 -o b * c * b), !(c * b * c -o c * a * a) |- a";
    for mode in [
        Mode::CLASSICAL.with_affine(),
        Mode::INTUITIONISTIC.with_affine(),
    ] {
        assert_eq!(verdict(text, mode), "proved", "{mode}");
    }
}

/// The backward search caps a place at its initial count unless some
/// clause gives it more tokens than it takes. No marking on the way from
/// the initial one has more there. So a place only taken from, or given
/// back as many tokens as taken, is capped, the place of a class of
/// clauses used once among them. A place that a clause raises is not.
#[test]
fn caps_the_places_no_clause_raises() {
    // The places are `a`, `d`, `g`, `b`, `c` and the class of the clause
    // used once. The second clause under `!` gives `d` back after taking
    // an `a`, a place before it. The clause used once raises `c` and takes
    // `b` and its ticket, a place before `c` and one after.
    let sequent: Sequent = "!(a -o a * a), !(a * d -o d * g), a, b, b, b, d, (a * b -o c) |- c"
        .parse()
        .unwrap();
    let forest = Forest::new(&sequent).unwrap();
    let open = u32::MAX;
    assert_eq!(program(&forest).caps(), [open, 1, open, 3, open, 1]);
}

/// Once the forward search has kept its share of markings without
/// deciding, the backward search runs beside it and may find the firing
/// sequence first. Forward, every marking of fewer than 200 tokens comes
/// before the clause used once can fire. Backward, its 200 tokens are
/// walked down to one, with its ticket at its cap, the initial count.
#[test]
fn the_backward_search_finds_a_long_firing_sequence() {
    let tokens = vec!["a"; 200].join(" * ");
    let sequent: Sequent = format!("!(a -o a * a), !(a -o b), !(b -o a), ({tokens} -o c), a |- c")
        .parse()
        .unwrap();
    let outcome = prove(&sequent, Mode::CLASSICAL, &horn()).unwrap();
    assert!(
        matches!(outcome.verdict, Verdict::Proved(_)),
        "{:?}",
        outcome.verdict
    );
    // More markings than the forward search keeps alone.
    assert!(
        outcome.statistics.memo_entries > 1 << 14,
        "{:?}",
        outcome.statistics
    );
}

/// The distance to the target after a firing merges the clause's inputs
/// and outputs by place. So a place that both touch is counted once, even
/// when an output on an earlier place comes first, and the firing that
/// reaches the target is seen to reach it.
#[test]
fn sees_the_firing_that_reaches_the_target() {
    for text in [
        "!(a -o b * a), !(c * c -o b * c), c, c |- b * c",
        "(b -o a), !(a * c -o b * a), b, c |- b * a",
    ] {
        assert_eq!(verdict(text, Mode::CLASSICAL), "proved", "{text}");
    }
}

/// A marking kept with a count that needs more than a byte reads back as
/// written. Here 130 tokens on one place move one by one to another.
#[test]
fn keeps_counts_past_a_byte() {
    let given = vec!["a"; 130].join(", ");
    let goal = vec!["b"; 130].join(" * ");
    let text = format!("!(a -o b), {given} |- {goal}");
    assert_eq!(verdict(&text, Mode::CLASSICAL), "proved");
}

/// Out of room, a search takes back the simplex's tableau, and the
/// forward search the backward one's markings, and tries once more. So
/// the simplex never costs it a decision. At these bounds the tableau is
/// held when the search's buffers next grow, and without it the search
/// has room to prove the goal.
#[test]
fn the_simplex_gives_its_memory_back() {
    let sequent: Sequent = "!(c * a -o b), !(1 -o b * d), !(b * b * d -o a), d |- a"
        .parse()
        .unwrap();
    let verdict = bounded(&sequent, Mode::CLASSICAL.with_affine(), 1500);
    assert!(matches!(verdict, Verdict::Proved(_)), "{verdict:?}");
    let partition = FAMILIES
        .iter()
        .find(|f| f.name == "partition-yes")
        .unwrap()
        .instance(4, 0)
        .unwrap()
        .sequent;
    let verdict = bounded(&partition, Mode::CLASSICAL, 8400);
    assert!(matches!(verdict, Verdict::Proved(_)), "{verdict:?}");
}

/// In intuitionistic mode a marking of several atoms is read whole on the
/// left of `⊢`, since each literal lies within the marking. Here the
/// marking is the forest's first occurrence. A clause takes a `c`, so no
/// atom's literals below the `!` all have one sign, and the counts leave
/// the goal to the engine.
#[test]
fn reads_a_marking_of_several_atoms_intuitionistically() {
    let text = "a * c, !(a -o a * c * c), !(c -o 1) |- 1";
    assert_eq!(verdict(text, Mode::INTUITIONISTIC), "state equation");
    assert_eq!(verdict(text, Mode::INTUITIONISTIC.with_affine()), "proved");
}

/// In intuitionistic mode the reading must put on the left of `⊢` exactly
/// the occurrences that hold a head or lie in one, and not what follows
/// the head within its member. The reading is given by hand, since that
/// is the only way to write a clause with its head before a body literal:
/// `a, a ⊸ b ⊢ b` with the clause `b⊥ ⊗ a`.
#[cfg(feature = "interactive")]
#[test]
fn the_reading_puts_exactly_the_heads_on_the_left() {
    use crate::linear::occurrences::{Reading, Side};
    use Side::{Input, Output};
    let sequent: Sequent = "|- ~b * a, ~a, b".parse().unwrap();
    let forest = Forest::new(&sequent).unwrap();
    let reads = |sides: &[Side]| {
        let reading = Reading::of_parts(&forest, sides, forest.roots()[2], false);
        let task = Task {
            forest: &forest,
            goal: forest.roots(),
            fragment: sequent.fragment(),
            mode: Mode::INTUITIONISTIC,
            reading: Some(&reading),
            roots: true,
            program: std::sync::OnceLock::new(),
        };
        Program::read(&task).is_some()
    };
    // The clause, its head, its body literal, the marking and the goal.
    assert!(reads(&[Input, Input, Output, Input, Output]));
    assert!(!reads(&[Input, Input, Input, Input, Output]));
}

/// The simplex polls the stop before every pivot. On a net it would
/// refute, a stop that fires at once ends it with `Reason::Stopped`.
#[test]
fn the_simplex_polls_the_stop() {
    let sequent: Sequent = "!(A -o A * A), !(B * B -o C), A, B |- C".parse().unwrap();
    let forest = Forest::new(&sequent).unwrap();
    let net = program(&forest);
    let account = Account::new(None);
    let mut stopped = Equation::new(false, &account);
    assert_eq!(
        stopped.run(&net, u64::MAX, &mut |_| true),
        Err(Reason::Stopped)
    );
    let mut equation = Equation::new(false, &account);
    assert_eq!(equation.run(&net, u64::MAX, &mut |_| false), Ok(true));
}

/// The searches charge every buffer they grow, at its capacity. Each
/// decides these nets from exactly the least bound below, and refuses at
/// one byte less. The simplex has no room here.
///
/// The bounds are the account's, so they move when what is charged
/// changes. Forward, the markings of the partition run out; backward, its
/// elements. The third net is refuted by the backward search beside the
/// forward one, which keeps 2¹⁴ markings first. The fourth is proved by
/// the first marking the backward search computes, so its bound is its
/// first element's, and the last buffer of it to grow is the queue.
#[test]
fn charges_what_it_grows() {
    let partition: Sequent = FAMILIES
        .iter()
        .find(|f| f.name == "partition-no")
        .unwrap()
        .instance(4, 0)
        .unwrap()
        .sequent;
    let both: Sequent = "!(c * a -o b), !(1 -o b * d), !(b * b * d -o a), d |- a"
        .parse()
        .unwrap();
    let one: Sequent = "a, !(a -o b) |- b".parse().unwrap();
    let no_room = Account::new(Some(0));
    for (sequent, affine, least, proved) in [
        (&partition, false, 7536, false),
        (&partition, true, 116_204, false),
        (&both, false, 659_984, false),
        (&one, true, 1171, true),
    ] {
        let forest = Forest::new(sequent).unwrap();
        let net = program(&forest);
        let within = |bytes| {
            let account = Account::new(Some(bytes));
            let mut equation = Equation::new(affine, &no_room);
            let search = if affine { cover::search } else { reach::search };
            search(
                &net,
                &account,
                reach::MOST_MARKINGS,
                &mut equation,
                &mut |_| false,
            )
            .0
        };
        assert_eq!(
            within(least).map(|f| f.is_some()),
            Ok(proved),
            "{sequent} within {least}"
        );
        assert_eq!(
            within(least - 1),
            Err(Reason::MemoryLimit {
                limit_bytes: least - 1
            }),
            "{sequent}"
        );
    }
}

/// The searches' counters on these runs, with no room for the simplex
/// so that the search alone decides. They are the markings taken
/// (`nodes`), those dropped as kept already, covered or above a cap
/// (`memo_hits`), those kept (`memo_entries`), and the units of work
/// polled.
///
/// The numbers are a function of the net, so they change only when the
/// search does. The runs are forward and backward on the partitions, and
/// the net that the backward reachability search refutes beside the
/// forward one.
#[test]
fn counts_its_work() {
    let family = |name: &str| -> Sequent {
        FAMILIES
            .iter()
            .find(|f| f.name == name)
            .unwrap()
            .instance(4, 0)
            .unwrap()
            .sequent
    };
    let (yes, no) = (family("partition-yes"), family("partition-no"));
    let both: Sequent = "!(c * a -o b), !(1 -o b * d), !(b * b * d -o a), d |- a"
        .parse()
        .unwrap();
    let no_room = Account::new(Some(0));
    for (sequent, affine, proved, counters) in [
        (&yes, false, true, [75, 27, 48, 74]),
        (&no, false, false, [217, 136, 81, 217]),
        (&yes, true, true, [126, 44, 82, 145]),
        (&no, true, false, [2289, 1848, 442, 2731]),
        (&both, false, false, [32_233, 15_846, 16_387, 32_233]),
    ] {
        let forest = Forest::new(sequent).unwrap();
        let net = program(&forest);
        let account = Account::new(None);
        let mut equation = Equation::new(affine, &no_room);
        let mut work = 0;
        let mut stop = |units| {
            work += units;
            false
        };
        let search = if affine { cover::search } else { reach::search };
        let (found, statistics) = search(
            &net,
            &account,
            reach::MOST_MARKINGS,
            &mut equation,
            &mut stop,
        );
        assert_eq!(found.map(|f| f.is_some()), Ok(proved), "{sequent}");
        let counted = [
            statistics.nodes,
            statistics.memo_hits,
            statistics.memo_entries,
            work,
        ];
        assert_eq!(counted, counters, "{sequent} affine {affine}");
    }
}

/// The simplex's weights for nets whose state equation has no solution,
/// and the units it polls: one before each pivot and one at the optimum.
///
/// Each weight is a place and its weight, the places numbered in the
/// order the goal's atoms are met. In affine mode the simplex enters the
/// surplus columns, and no weight is negative. The tokens of `z` grow
/// without end, so only the equation refutes these nets.
#[test]
fn the_simplex_finds_these_weights() {
    for (text, linear, affine) in [
        (
            "!(A -o A * A), !(B * B -o C), A, B |- C",
            (vec![(0, -2), (1, 1), (2, 2)], 2),
            (vec![(0, 0), (1, 1), (2, 2)], 3),
        ),
        (
            "!(a * a -o b), !(b -o a * a), !(b * b -o c), !(c -o b * b), !(z -o z * z), z, a |- c",
            (vec![(0, 1), (1, 2), (2, 4), (3, -4)], 3),
            (vec![(0, 1), (1, 2), (2, 4), (3, 0)], 4),
        ),
        (
            "!(a -o b * b), !(b * b * b -o c), !(z -o z * z), z, a, a |- c * c",
            (vec![(0, 2), (1, 1), (2, 3), (3, -3)], 3),
            (vec![(0, 2), (1, 1), (2, 3), (3, 0)], 4),
        ),
        (
            "!(a * b -o c), !(c -o a * a), !(c -o b * b), a, b, !(z -o z * z), z |- c * c",
            (vec![(0, 1), (1, 1), (2, 2), (3, -2)], 3),
            (vec![(0, 1), (1, 1), (2, 2), (3, 0)], 4),
        ),
    ] {
        let sequent: Sequent = text.parse().unwrap();
        let forest = Forest::new(&sequent).unwrap();
        let net = program(&forest);
        for (affine, (weights, polls)) in [(false, linear), (true, affine)] {
            let account = Account::new(None);
            let mut equation = Equation::new(affine, &account);
            let mut polled = 0;
            let refuted = equation.run(&net, u64::MAX, &mut |units| {
                polled += units;
                false
            });
            assert_eq!(refuted, Ok(true), "{text} affine {affine}");
            assert_eq!(
                (equation.certificate(), polled),
                (Some(&weights[..]), polls),
                "{text} affine {affine}"
            );
        }
    }
}

/// A search that runs out of room gives its memory back, and the simplex
/// then runs to its end with the rest of the bound. At this bound the
/// search cannot keep its first marking, but the tableau fits.
#[test]
fn the_simplex_refutes_after_the_search() {
    let sequent: Sequent = "!(A -o A * A), !(B * B -o C), A, B |- C".parse().unwrap();
    let verdict = bounded(&sequent, Mode::CLASSICAL, 300);
    assert!(
        matches!(&verdict, Verdict::Unprovable(d) if matches!(d.refutation(), Refutation::StateEquation(_))),
        "{verdict:?}"
    );
}

/// Out of room for its frontier, the forward search takes back the
/// simplex's tableau and expands the marking again. The tableau is built
/// before the search and held. At this bound the frontier's growth, not
/// the markings', is the first to find no room, and without the tableau
/// the counter is proved.
#[test]
fn the_frontier_takes_the_simplex_memory() {
    let sequent: Sequent = FAMILIES
        .iter()
        .find(|f| f.name == "counter")
        .unwrap()
        .instance(16, 0)
        .unwrap()
        .sequent;
    let forest = Forest::new(&sequent).unwrap();
    let net = program(&forest);
    let account = Account::new(Some(1040));
    let mut equation = Equation::new(false, &account);
    // Enough for the set-up and not for a pivot.
    let set_up = 3 * (net.places as u64).pow(2) + net.arcs.len() as u64;
    assert_eq!(equation.run(&net, set_up, &mut |_| false), Ok(false));
    assert!(account.used() > 0);
    let (found, _) = reach::search(
        &net,
        &account,
        reach::MOST_MARKINGS,
        &mut equation,
        &mut |_| false,
    );
    assert!(matches!(found, Ok(Some(_))), "{found:?}");
}

/// A task reads its goal once. Every call returns the same program,
/// without the transitions that can never fire, and the dispatch's net
/// feature agrees with it.
#[test]
fn a_task_reads_its_program_once() {
    let sequent: Sequent = "!(a -o b), !(c -o d), a |- b".parse().unwrap();
    let forest = Forest::new(&sequent).unwrap();
    let task = Task {
        forest: &forest,
        goal: forest.roots(),
        fragment: sequent.fragment(),
        mode: Mode::CLASSICAL,
        reading: None,
        roots: true,
        program: std::sync::OnceLock::new(),
    };
    let first = Program::of(&task).expect("a Horn program");
    assert!(std::ptr::eq(first, Program::of(&task).unwrap()));
    assert!(super::is_net(&task));
    assert!(std::ptr::eq(first, Program::of(&task).unwrap()));
    // `c -o d` waits for a `c`, which is never marked, so only one
    // transition is left.
    assert_eq!(first.transitions.len(), 1);
}
