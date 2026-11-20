// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The search's tests: each answer's evidence, the axioms of the cube,
//! the order of the systems, every small frame as a second reference,
//! classical logic, Gödel's translation from intuitionistic logic, and
//! the bounds.

use super::*;
use crate::kripke::{Model, World};
use crate::limits::{Limits, Refusal};
use crate::linear::search::Reason;
use crate::linear::search::generate::Rng;

/// The work every test decision may do: generous for the formulas below,
/// and an end where a search would not end.
const WORK: u64 = 1 << 22;

/// Decides `text` in `system` within [`WORK`] and returns whether it is
/// valid; an unknown verdict fails the test.
fn valid(text: &str, system: System) -> bool {
    let sequent: Sequent = text.parse().unwrap();
    decided(&sequent, system)
}

/// Decides a sequent in `system` within [`WORK`] and returns whether it
/// is valid; an unknown verdict fails the test, and a "not valid"
/// without a countermodel does too.
fn decided(sequent: &Sequent, system: System) -> bool {
    let limits = Limits::default().with_work(Some(WORK));
    let options = Options::default().with_system(system);
    let outcome = decide_within(sequent, &options, &limits, |_| false)
        .unwrap_or_else(|e| panic!("{sequent} in {system}: {e}"));
    match outcome.verdict {
        Verdict::Valid(_) => true,
        Verdict::NotValid => {
            assert!(
                outcome.countermodel.is_some(),
                "{sequent} in {system}: no model"
            );
            false
        }
        Verdict::Unknown(reason) => panic!("{sequent} in {system}: unknown, {reason}"),
    }
}

/// Each axiom of the cube is valid exactly in the systems whose frames
/// meet its condition, and K's in every system.
#[test]
fn the_axioms_are_valid_where_their_conditions_hold() {
    for &system in System::ALL {
        assert!(valid("[](p -> q) -> ([]p -> []q)", system), "K in {system}");
        assert!(
            valid("<>(p \\/ q) <-> <>p \\/ <>q", system),
            "◇ in {system}"
        );
        for (axiom, condition) in [
            ("[]p -> <>p", Condition::Serial),
            ("[]p -> p", Condition::Reflexive),
            ("p -> []<>p", Condition::Symmetric),
            ("[]p -> [][]p", Condition::Transitive),
            ("<>p -> []<>p", Condition::Euclidean),
        ] {
            assert_eq!(
                valid(axiom, system),
                system.has(condition),
                "{axiom} in {system}"
            );
        }
    }
}

/// A sequent's sides: hypotheses and the formulas right of `⊢` read at
/// one world, so `□p ⊢ p` is T's axiom and `⊢ p, ¬p` excluded middle.
#[test]
fn a_sequent_is_read_at_one_world() {
    assert!(!valid("[]p |- p", System::K));
    assert!(valid("[]p |- p", System::T));
    assert!(valid("|- p, ~p", System::K));
    assert!(valid("<>p, [](p -> q) |- <>q", System::K));
    assert!(!valid("<>p, <>q |- <>(p /\\ q)", System::S5));
    assert!(valid("<>[]p |- p", System::S5));
    assert!(!valid("<>[]p |- p", System::S4));
    assert!(valid("[](p <-> q) |- []p <-> []q", System::K));
}

/// Where a countermodel needs a loop, blocking gives it: Löb's axiom is
/// valid on no frame of K4 or KD4 that has a cycle, McKinsey's and
/// Grzegorczyk's are not valid in S4, and each countermodel is checked.
#[test]
fn blocking_gives_countermodels_with_loops() {
    let lob = "[]([]p -> p) -> []p";
    for system in [System::K4, System::KD4, System::S4, System::S5] {
        assert!(!valid(lob, system), "Löb in {system}");
    }
    assert!(!valid("[]<>p -> <>[]p", System::S4));
    assert!(!valid("[]<>p -> <>[]p", System::S5));
    let grz = "[]([](p -> []p) -> p) -> p";
    assert!(!valid(grz, System::S4) && !valid(grz, System::T));
    assert!(valid("<>[]p -> []<>p", System::KD45) && valid("<>[]p -> []<>p", System::K5));
}

/// The derivation is in the system's calculus: S4's proof of the axiom 4
/// uses transitivity, so G3K refuses it.
#[test]
fn a_proof_is_checked_in_its_systems_calculus() {
    let sequent: Sequent = "[]p -> [][]p".parse().unwrap();
    let outcome = decide(&sequent, &Options::default().with_system(System::S4)).unwrap();
    let Verdict::Valid(Evidence::Derivation(derivation)) = outcome.verdict else {
        panic!("valid in S4");
    };
    let limits = Limits::default();
    let s4 = calculus(System::S4, &limits).unwrap();
    assert!(derivation.check(&s4, &limits, |_| false).is_ok());
    let k = calculus(System::K, &limits).unwrap();
    assert!(matches!(
        derivation.check(&k, &limits, |_| false),
        Err(crate::Error::InvalidDerivation { .. })
    ));
    assert!(outcome.statistics.inferences > 0);
}

/// Returns a formula of depth `depth` at most over `atoms` and the
/// constants, built with every connective and both modalities.
fn generated(rng: &mut Rng, formulas: &mut Formulas, atoms: &[NodeId], depth: usize) -> NodeId {
    if depth == 0 || rng.one_in(4) {
        let leaf = rng.below(2 * atoms.len() + 1);
        return match leaf.checked_sub(2 * atoms.len()) {
            None => atoms[leaf % atoms.len()],
            Some(_) => formulas
                .add(if rng.one_in(2) {
                    Node::True
                } else {
                    Node::False
                })
                .unwrap(),
        };
    }
    let a = generated(rng, formulas, atoms, depth - 1);
    let node = match rng.below(7) {
        0 => Node::Not(a),
        1 | 2 => Node::Box(a),
        3 => Node::Diamond(a),
        choice => {
            let b = generated(rng, formulas, atoms, depth - 1);
            match choice {
                4 => Node::And(a, b),
                5 => Node::Or(a, b),
                _ => Node::Implies(a, b),
            }
        }
    };
    formulas.add(node).unwrap()
}

/// Returns a sequent over `p` and `q` of up to two hypotheses and one
/// formula right of `⊢`, each of depth four at most.
fn generated_sequent(rng: &mut Rng) -> Sequent {
    let mut formulas = Formulas::default();
    let atoms = [formulas.atom("p").unwrap(), formulas.atom("q").unwrap()];
    let left = (0..rng.below(3))
        .map(|_| generated(rng, &mut formulas, &atoms, 4))
        .collect();
    let right = vec![generated(rng, &mut formulas, &atoms, 4)];
    Sequent::new(formulas, left, right).unwrap()
}

/// Returns whether every model of up to `most` worlds whose relation
/// meets the system's conditions satisfies the sequent at every world: a
/// necessary condition of validity, which the second reference asks of
/// every "valid". Relations of three worlds are sampled by `rng`.
fn holds_on_small_frames(sequent: &Sequent, system: System, rng: &mut Rng) -> bool {
    let limits = Limits::default();
    for worlds in 1..=3_u32 {
        let relations = 1_u32 << (worlds * worlds);
        let tried: Vec<u32> = if worlds < 3 {
            (0..relations).collect()
        } else {
            (0..48)
                .map(|_| rng.below(relations as usize) as u32)
                .collect()
        };
        for relation in tried {
            for valuation in 0..1_u32 << (2 * worlds) {
                let model = Model::new(
                    (0..worlds)
                        .map(|w| {
                            let atoms = ["p", "q"]
                                .iter()
                                .enumerate()
                                .filter(|&(a, _)| valuation >> (2 * w + a as u32) & 1 == 1)
                                .map(|(_, &name)| name.to_owned())
                                .collect();
                            let arrows = (0..worlds)
                                .filter(|&v| relation >> (worlds * w + v) & 1 == 1)
                                .collect();
                            World::new(atoms, false, arrows)
                        })
                        .collect(),
                    0,
                );
                match model.truth(sequent, system, &limits, |_| false) {
                    Ok(truth) => {
                        if truth.failing().is_some() {
                            return false;
                        }
                    }
                    Err(crate::Error::Model { .. }) => {}
                    Err(e) => panic!("{e}"),
                }
            }
        }
    }
    true
}

/// On generated sequents, in every system: every answer has its checked
/// evidence; no small model of the system refutes a sequent called
/// valid; and what is valid in a system is valid in every system that
/// extends it.
#[test]
fn generated_sequents_agree_with_small_frames_and_the_order_of_systems() {
    let mut rng = Rng::new(20_261_116);
    for _ in 0..120 {
        let sequent = generated_sequent(&mut rng);
        let verdicts: Vec<bool> = System::ALL
            .iter()
            .map(|&system| decided(&sequent, system))
            .collect();
        for (&system, &valid) in System::ALL.iter().zip(&verdicts) {
            if valid {
                assert!(
                    holds_on_small_frames(&sequent, system, &mut rng),
                    "{sequent} called valid in {system}, refuted on a small frame"
                );
            }
            for (&other, &also) in System::ALL.iter().zip(&verdicts) {
                if valid && other.extends(system) {
                    assert!(also, "{sequent}: valid in {system}, not in {other}");
                }
            }
        }
    }
}

/// A sequent without modalities is valid in every system exactly when
/// its truth table says so: generated sequents of `¬`, `∧`, `∨` and `→`
/// over two atoms and the constants, the table read off the models of
/// one world, and a few hand-written ones.
#[test]
fn classical_sequents_keep_their_verdicts() {
    for (text, classical) in [
        ("p \\/ ~p", true),
        ("((p -> q) -> p) -> p", true),
        ("p -> q", false),
        ("p <-> q |- (p -> q) /\\ (q -> p)", true),
    ] {
        for &system in System::ALL {
            assert_eq!(valid(text, system), classical, "{text} in {system}");
        }
    }
    let limits = Limits::default();
    let mut rng = Rng::new(11);
    for _ in 0..60 {
        // Every connective but the modalities.
        let mut formulas = Formulas::default();
        let atoms = [formulas.atom("p").unwrap(), formulas.atom("q").unwrap()];
        let mut formula = |rng: &mut Rng| {
            let mut made = atoms.to_vec();
            for _ in 0..rng.below(6) {
                let a = made[rng.below(made.len())];
                let b = made[rng.below(made.len())];
                let node = match rng.below(5) {
                    0 => Node::Not(a),
                    1 => Node::And(a, b),
                    2 => Node::Or(a, b),
                    3 => Node::Implies(a, b),
                    _ => Node::False,
                };
                made.push(formulas.add(node).unwrap());
            }
            made[made.len() - 1]
        };
        let left = (0..rng.below(3)).map(|_| formula(&mut rng)).collect();
        let right = vec![formula(&mut rng)];
        let sequent = Sequent::new(formulas, left, right).unwrap();
        let table = (0..4_u32).all(|valuation| {
            let atoms = ["p", "q"]
                .iter()
                .enumerate()
                .filter(|&(a, _)| valuation >> a & 1 == 1)
                .map(|(_, &name)| name.to_owned())
                .collect();
            let world = Model::new(vec![World::new(atoms, false, Vec::new())], 0);
            let truth = world
                .truth(&sequent, System::K, &limits, |_| false)
                .unwrap();
            truth.sequent_holds(0)
        });
        for &system in System::ALL {
            assert_eq!(decided(&sequent, system), table, "{sequent} in {system}");
        }
    }
}

/// A derivation the search gives is refused once any one of its nodes
/// names another rule, and an outcome whose countermodel refutes
/// another sequent fails its check.
#[test]
fn forged_evidence_is_refused() {
    use crate::wire;
    let limits = Limits::default();
    let s4 = Options::default().with_system(System::S4);
    let sequent: Sequent = "[]p -> [][]p".parse().unwrap();
    let outcome = decide(&sequent, &s4).unwrap();
    let Verdict::Valid(Evidence::Derivation(derivation)) = &outcome.verdict else {
        panic!("valid in S4");
    };
    let calculus = calculus(System::S4, &limits).unwrap();
    let json: serde_json::Value = serde_json::to_value(&**derivation).unwrap();
    let nodes = json["nodes"].as_array().unwrap().len();
    for node in 0..nodes {
        let mut forged = json.clone();
        let rule = forged["nodes"][node]["rule"].as_str().unwrap().to_owned();
        let other = if rule == "ax" { "Trans" } else { "ax" };
        forged["nodes"][node]["rule"] = other.into();
        let text = forged.to_string();
        let read: crate::rules::Derivation =
            wire::upgrade(&mut serde_json::Deserializer::from_str(&text), &limits).unwrap();
        assert!(
            read.check(&calculus, &limits, |_| false).is_err(),
            "node {node} as {other}"
        );
    }
    let mut refuted = decide(&"|- p".parse().unwrap(), &Options::default()).unwrap();
    assert!(refuted.check(&limits, |_| false).is_ok());
    refuted.sequent = "|- p \\/ ~p".parse().unwrap();
    assert!(matches!(
        refuted.check(&limits, |_| false),
        Err(crate::Error::Model { .. })
    ));
}

/// Returns Gödel's translation of an ordinary sequent into S4: `p` as
/// `□p`, `A → B` as `□(A' → B')`, `¬A` as `□¬A'`, `A ↔ B` as both
/// implications, `∧`, `∨`, `⊤` and `⊥` as they are, and nothing right of
/// `⊢` as `⊥`.
fn goedel(sequent: &crate::ordinary::Sequent) -> Sequent {
    use crate::ordinary::Node as O;
    let source = sequent.formulas();
    let mut formulas = Formulas::default();
    let mut image = Vec::with_capacity(source.len());
    for &node in source.nodes() {
        let at = |k: crate::ordinary::NodeId, image: &[NodeId]| image[k.index()];
        let node = match node {
            O::Atom(a) => {
                let atom = formulas.atom(source.atom_name(a)).unwrap();
                Node::Box(atom)
            }
            O::Forall(_) | O::Exists(_) => unreachable!("the samples are propositional"),
            O::True => Node::True,
            O::False => Node::False,
            O::Not(a) => Node::Box(formulas.add(Node::Not(at(a, &image))).unwrap()),
            O::And(a, b) => Node::And(at(a, &image), at(b, &image)),
            O::Or(a, b) => Node::Or(at(a, &image), at(b, &image)),
            O::Implies(a, b) => Node::Box(
                formulas
                    .add(Node::Implies(at(a, &image), at(b, &image)))
                    .unwrap(),
            ),
            O::Iff(a, b) => {
                let there = formulas
                    .add(Node::Implies(at(a, &image), at(b, &image)))
                    .unwrap();
                let back = formulas
                    .add(Node::Implies(at(b, &image), at(a, &image)))
                    .unwrap();
                let there = formulas.add(Node::Box(there)).unwrap();
                let back = formulas.add(Node::Box(back)).unwrap();
                Node::And(there, back)
            }
        };
        image.push(formulas.add(node).unwrap());
    }
    let left = sequent.left().iter().map(|k| image[k.index()]).collect();
    let right = match sequent.right() {
        [] => vec![formulas.add(Node::False).unwrap()],
        right => right.iter().map(|k| image[k.index()]).collect(),
    };
    Sequent::new(formulas, left, right).unwrap()
}

/// Returns an ordinary sequent over `a` and `b` of up to two hypotheses
/// and one goal, each of depth three at most.
fn ordinary_sequent(rng: &mut Rng) -> crate::ordinary::Sequent {
    use crate::ordinary::{Formulas as Ordinary, Node as O, NodeId as Id};
    /// Returns a formula of depth `depth` at most.
    fn formula(rng: &mut Rng, formulas: &mut Ordinary, atoms: &[Id], depth: usize) -> Id {
        if depth == 0 || rng.one_in(4) {
            return atoms[rng.below(atoms.len())];
        }
        let a = formula(rng, formulas, atoms, depth - 1);
        let node = match rng.below(4) {
            0 => O::Not(a),
            choice => {
                let b = formula(rng, formulas, atoms, depth - 1);
                match choice {
                    1 => O::And(a, b),
                    2 => O::Or(a, b),
                    _ => O::Implies(a, b),
                }
            }
        };
        formulas.add(node).unwrap()
    }
    let mut formulas = Ordinary::default();
    let atoms = [formulas.atom("a").unwrap(), formulas.atom("b").unwrap()];
    let left = (0..rng.below(3))
        .map(|_| formula(rng, &mut formulas, &atoms, 3))
        .collect();
    let right = vec![formula(rng, &mut formulas, &atoms, 3)];
    crate::ordinary::Sequent::new(formulas, left, right).unwrap()
}

/// Gödel's translation: an ordinary sequent is intuitionistically valid
/// exactly when its translation is valid in S4, the ordinary suite
/// deciding the one and the modal search the other.
#[test]
fn intuitionistic_logic_is_s4_through_goedels_translation() {
    let mut cases: Vec<crate::ordinary::Sequent> = [
        "a \\/ ~a",
        "~~a -> a",
        "a -> ~~a",
        "((a -> b) -> a) -> a",
        "(a -> b) \\/ (b -> a)",
        "~(a /\\ b) -> ~a \\/ ~b",
        "~a \\/ ~b -> ~(a /\\ b)",
        "a -> b, b -> c |- a -> c",
        "a <-> b |- b <-> a",
        "~~(a \\/ ~a)",
        "a |-",
    ]
    .iter()
    .map(|text| text.parse().unwrap())
    .collect();
    let mut rng = Rng::new(7);
    for _ in 0..60 {
        cases.push(ordinary_sequent(&mut rng));
    }
    let intuitionistic = crate::ordinary::Options::default();
    for sequent in cases {
        if sequent.right().len() > 1 {
            continue;
        }
        let ordinary = crate::ordinary::decide(&sequent, &intuitionistic).unwrap();
        let expected = match ordinary.verdict {
            crate::ordinary::Verdict::Valid(_) => true,
            crate::ordinary::Verdict::NotValid => false,
            crate::ordinary::Verdict::Unknown(_) => continue,
        };
        let image = goedel(&sequent);
        assert_eq!(
            decided(&image, System::S4),
            expected,
            "{sequent} as {image}"
        );
    }
}

/// The search keeps to its bounds: the work bound and the stop give an
/// unknown verdict, a derivation past its bound is refused before it is
/// made, and a tight memory bound gives an unknown verdict.
#[test]
fn the_search_keeps_to_its_bounds() {
    let sequent: Sequent = "[](p -> q) -> ([]p -> []q)".parse().unwrap();
    let options = Options::default();
    let work = Limits::default().with_work(Some(3));
    let outcome = decide_within(&sequent, &options, &work, |_| false).unwrap();
    assert!(matches!(
        outcome.verdict,
        Verdict::Unknown(Reason::WorkLimit { limit: 3 })
    ));
    let long: Sequent = (0..400)
        .map(|i| format!("p{i}"))
        .collect::<Vec<_>>()
        .join(" -> ")
        .parse()
        .unwrap();
    let search = |p: crate::Progress| p.phase == crate::Phase::Search;
    let outcome = decide_within(&long, &options, &Limits::default(), search).unwrap();
    assert!(matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)));
    let small = Limits::default().with_derivation_bytes(Some(64));
    assert!(matches!(
        decide_within(&sequent, &options, &small, |_| false),
        Err(crate::Error::Refused(Refusal::Output { .. }))
    ));
    let tight = Limits::default().with_memory_bytes(Some(256));
    let outcome = decide_within(&long, &options, &tight, |_| false).unwrap();
    assert!(matches!(
        outcome.verdict,
        Verdict::Unknown(Reason::MemoryLimit { .. })
    ));
}

/// Halpern and Moses's formulas force a binary tree of worlds of depth
/// `m` in K: the search's countermodel of their negation has at least
/// `2^m` worlds, and it stays within the work it is given.
#[test]
fn a_formula_can_force_an_exponential_model() {
    for m in 1..=3_usize {
        let d = |i: usize| format!("d{i}");
        let p = |i: usize| format!("p{i}");
        let depth: Vec<String> = (1..=m + 1)
            .map(|i| format!("({} -> {})", d(i), d(i - 1)))
            .collect();
        let determined: Vec<String> = (1..=m)
            .map(|i| {
                format!(
                    "({d} -> (({p} -> [](({d}) -> {p})) /\\ (~{p} -> []({d} -> ~{p}))))",
                    d = d(i),
                    p = p(i)
                )
            })
            .collect();
        let branching: Vec<String> = (0..m)
            .map(|i| {
                format!(
                    "(({} /\\ ~{}) -> (<>({} /\\ ~{} /\\ {}) /\\ <>({} /\\ ~{} /\\ ~{})))",
                    d(i),
                    d(i + 1),
                    d(i + 1),
                    d(i + 2),
                    p(i + 1),
                    d(i + 1),
                    d(i + 2),
                    p(i + 1)
                )
            })
            .collect();
        let body = format!(
            "({}) /\\ ({}) /\\ ({})",
            depth.join(" /\\ "),
            determined.join(" /\\ "),
            branching.join(" /\\ ")
        );
        let boxes: Vec<String> = (0..=m)
            .map(|i| format!("{}({body})", "[]".repeat(i)))
            .collect();
        // Satisfiable: its negation is not valid, and its countermodel
        // is a model of it.
        let text = format!("{} /\\ ~{} /\\ {} |-", d(0), d(1), boxes.join(" /\\ "));
        let sequent: Sequent = text.parse().unwrap();
        let limits = Limits::default().with_work(Some(WORK));
        let outcome = decide_within(&sequent, &Options::default(), &limits, |_| false).unwrap();
        assert!(matches!(outcome.verdict, Verdict::NotValid), "m = {m}");
        let model = outcome.countermodel.unwrap().model;
        assert!(
            model.worlds.len() >= 1 << m,
            "m = {m}: {} worlds",
            model.worlds.len()
        );
    }
}

/// A split whose first premise closes without its formula is not
/// searched further: worlds that each split once, beside one world that
/// closes, cost a split each, where without the pruning they would cost
/// `2^n`.
#[test]
fn independent_splits_do_not_multiply() {
    let n = 16;
    let mut parts: Vec<String> = (0..n).map(|i| format!("<>(a{i} \\/ b{i})")).collect();
    parts.push("<>(c /\\ ~c)".to_owned());
    let sequent: Sequent = format!("{} |-", parts.join(", ")).parse().unwrap();
    let outcome = decide(&sequent, &Options::default()).unwrap();
    assert!(matches!(outcome.verdict, Verdict::Valid(_)));
    assert!(
        outcome.statistics.splits <= n,
        "{}",
        outcome.statistics.splits
    );
    assert!(outcome.statistics.inferences <= 4);
}
