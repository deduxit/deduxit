// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Ordinary logic: classical, intuitionistic and minimal, propositional
//! and first-order.
//!
//! [`decide()`] says whether a [`Sequent`] is valid in a [`Logic`]. A
//! valid sequent comes with checked [`Evidence`]: a derivation of LK
//! (classical logic) or LJ (intuitionistic and minimal logic), Gentzen's
//! sequent calculi, which [`Derivation::check`] accepted; or, from the SAT
//! solver in classical logic, a clausal proof that the crate's checker
//! accepted. A sequent that is not valid comes with a checked Kripke model
//! that refutes it ([`Countermodel`]) where the method finds one.
//! [`decide_within`] does the same under the caller's bounds, search
//! options and stop.
//!
//! [`Method`] chooses how a sequent is decided:
//! - a SAT solver, the default for every logic; it needs the cargo feature
//!   `sat`, on by default;
//! - Dyckhoff's calculus LJT, for intuitionistic and minimal logic, their
//!   default without `sat`;
//! - the embedding, classical logic's default without `sat`. It translates
//!   the sequent into linear logic by a [`Translation`] ([`translate()`]
//!   returns an [`Image`]: a linear sequent and the mode to prove it in),
//!   and reads the linear proof back ([`Image::read_back`]). Classical
//!   logic goes into affine MALL, intuitionistic and minimal logic into
//!   intuitionistic linear logic.
//!
//! [`Method::default_for`] says which evidence the default keeps from the
//! other methods.
//!
//! [`WrittenTableau`] reads a signed tableau of classical logic that a
//! student wrote as text and [reviews](WrittenTableau::review) it rule by
//! rule, naming the first step that breaks a rule.
//!
//! A sequent may be first-order, with predicates and quantifiers
//! ([`Sequent`] has the syntax). Grounding, the connection prover and the
//! embedding decide it, and the finite model search may refute it;
//! [`Options::method_for`] says which runs by default. The propositional
//! methods refuse such a sequent ([`Error::FirstOrder`](crate::Error::FirstOrder)), and a
//! derivation of one is checked as any other.
//!
//! The excluded middle is not valid in intuitionistic logic, the default:
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::ordinary::{Options, Sequent, Verdict, decide};
//!
//! let sequent: Sequent = "a \\/ ~a".parse()?;
//! let outcome = decide(&sequent, &Options::default())?;
//! match &outcome.verdict {
//!     Verdict::Valid(evidence) => println!("valid\n{}", evidence.derivation()?),
//!     Verdict::NotValid => println!("not valid"),
//!     _ => println!("unknown"),
//! }
//! if let Some(countermodel) = &outcome.countermodel {
//!     println!("{}", countermodel.model);
//! }
//! assert!(matches!(outcome.verdict, Verdict::NotValid));
//! # Ok::<(), deduxit::Error>(())
//! ```

// A new variant must be decided at every arm, never fall into an
// existing one.
#![deny(clippy::wildcard_enum_match_arm)]

/// A branch of a derivation of LK as two builders make it.
mod branch;
/// What moving or removing a binder does to de Bruijn indices.
mod bruijn;
pub mod connection;
/// The countermodels and their checks: a Kripke model of a propositional
/// sequent, a finite structure and a Kripke model with growing domains of a
/// first-order one.
mod countermodel;
/// Deciding an ordinary sequent in one call.
mod decide;
/// The derivations of LK and LJ, the read-back and its check.
mod derivation;
pub mod equality;
/// The formula arena: nodes, formulas and sequents.
mod formulas;
#[cfg(feature = "sat")]
pub mod grounding;
#[cfg(feature = "sat")]
pub mod herbrand;
/// The instances of a quantified formula, built or found.
mod instance;
/// Step-by-step proving in LK and LJ.
#[cfg(feature = "interactive")]
mod interactive;
pub mod interpolant;
pub mod ljt;
#[cfg(feature = "sat")]
pub mod models;
pub mod negative;
pub mod normal;
/// The decision's vocabulary: logics, translations, methods, options.
mod options;
#[cfg(feature = "parse")]
mod parse;
/// Every subformula's polarity where a sequent fails.
mod polarity;
pub mod resolution;
/// The certificates over `Prop`.
#[cfg(feature = "rocq")]
pub(crate) mod rocq;
#[cfg(feature = "sat")]
pub mod sat;
/// The translations into linear logic.
mod table;
#[cfg(feature = "interactive")]
mod tableau;
/// Formulas copied between arenas: the goal state's search, the normal
/// forms' claims, the tableau review's expected sequent.
pub(crate) mod transfer;
mod translate;
/// The terms that formulas reach, each once.
mod walk;
/// A tableau written by hand, read and reviewed.
#[cfg(all(feature = "interactive", feature = "parse", feature = "serialize"))]
mod written;

pub use countermodel::{
    Countermodel, Evaluation, FiniteCountermodel, Forcing, KripkeCountermodel, KripkeStructure,
    KripkeWorld, MODEL_CELLS, Structure,
};
pub use decide::{
    EVIDENCE_WORK, Evidence, Outcome, Search, Verdict, decide, decide_within, gives_way,
};
pub use derivation::{Derivation, Inference, Rule, Side};
#[cfg(feature = "serialize")]
pub(crate) use formulas::closed;
pub use formulas::{Formula, Formulas, Node, NodeId, Sequent};
pub(crate) use formulas::{REFUSED, Symbols, bound};
#[cfg(all(feature = "interactive", feature = "serialize"))]
pub(crate) use interactive::Parts;
#[cfg(feature = "interactive")]
pub use interactive::{
    Applicable, BindingFault, Candidate, Closed, Context, Hint, Interactive, Needs, Status, Step,
    StepError,
};
pub use options::{Logic, Method, Options, Target, Translation};
#[cfg(feature = "parse")]
pub use parse::{Problem, read_tptp, tptp_text};
pub use table::{TruthTable, truth_table};
#[cfg(feature = "interactive")]
pub use tableau::{End, Sign, Signed, Tableau, TableauNode};
pub use translate::{FALSE, Image, translate};
#[cfg(all(feature = "interactive", feature = "parse", feature = "serialize"))]
pub use written::{Standing, TableauReview, WrittenTableau};

#[cfg(all(test, feature = "parse"))]
mod tests {
    #[cfg(feature = "sat")]
    use super::options::{GROUNDS_NODES, grounds};
    use super::*;
    use crate::Error;
    use crate::hash::HashMap;
    use crate::limits::{Refusal, Space};
    use crate::linear::search::generate::Rng;
    use crate::linear::{Options as Search, Verdict, prove};

    /// Decides `text` in `logic` by `translation` with a copy bound of
    /// eight, and returns whether it was proved, with the derivation read
    /// back checked.
    fn decide(text: &str, logic: Logic, translation: Translation) -> Option<bool> {
        let sequent: Sequent = text.parse().unwrap();
        let image = translate(&sequent, logic, translation, &crate::Limits::default()).unwrap();
        let search = Search::default().with_copies(Some(8));
        let outcome = prove(image.sequent(), image.mode(), &search).unwrap();
        match &outcome.verdict {
            Verdict::Proved(proof) => {
                let unbounded = crate::Limits::default();
                let derivation = image.read_back(proof, &unbounded, |_| false).unwrap();
                derivation
                    .check(&unbounded, |_| false)
                    .unwrap_or_else(|e| panic!("{text} ({translation}): {e}"));
                Some(true)
            }
            Verdict::Unprovable(_) => Some(false),
            Verdict::Unknown(_) => None,
        }
    }

    /// The default's test of whether grounding decides a sequent ends at
    /// its bound on work: a congruence axiom of a predicate of 500
    /// arguments lies in the Bernays–Schönfinkel class, but its normal
    /// form is a nest of a thousand quantifiers whose making is quadratic,
    /// past the bound, so the default takes another method; a sequent of
    /// the class of ordinary size still grounds.
    #[cfg(feature = "sat")]
    #[test]
    fn the_grounding_test_ends_at_its_work_bound() {
        let small: Sequent = "forall x. (p(x) -> q(x)), p(a) |- q(a)".parse().unwrap();
        assert!(grounds(&small));
        let n = 500;
        let xs: Vec<String> = (0..n).map(|i| format!("x{i}")).collect();
        let ys: Vec<String> = (0..n).map(|i| format!("y{i}")).collect();
        let equal: Vec<String> = (0..n).map(|i| format!("e(x{i}, y{i})")).collect();
        let text = format!(
            "forall {} {}. ({} /\\ p({}) -> p({})) |- r",
            xs.join(" "),
            ys.join(" "),
            equal.join(" /\\ "),
            xs.join(", "),
            ys.join(", "),
        );
        let wide: Sequent = text.parse().unwrap();
        assert!(!grounds(&wide));
        let unbounded = crate::Limits::default().with_occurrences(Some(GROUNDS_NODES));
        assert!(grounding::applies(&wide, &unbounded).unwrap());
    }

    /// Every logic and translation reads back from its name, an unknown
    /// name is refused naming the known ones, and the readers and the
    /// translation keep to the occurrence bound.
    #[test]
    fn names_read_back_and_bounds_hold() {
        // Every enum read by `crate::name::named` lists in `NAMES` the
        // names of `ALL`, in order.
        fn listed<T: Copy>(all: &[T], name: impl Fn(T) -> &'static str, names: &[&str]) {
            assert_eq!(all.iter().map(|&t| name(t)).collect::<Vec<_>>(), names);
        }
        listed(Logic::ALL, Logic::name, Logic::NAMES);
        listed(Translation::ALL, Translation::name, Translation::NAMES);
        listed(Method::ALL, Method::name, Method::NAMES);
        listed(normal::Form::ALL, normal::Form::name, normal::Form::NAMES);
        listed(
            negative::Translation::ALL,
            negative::Translation::name,
            negative::Translation::NAMES,
        );
        listed(
            resolution::Method::ALL,
            resolution::Method::name,
            resolution::Method::NAMES,
        );
        listed(
            resolution::Equality::ALL,
            resolution::Equality::name,
            resolution::Equality::NAMES,
        );
        listed(
            crate::modal::Condition::ALL,
            crate::modal::Condition::name,
            crate::modal::Condition::NAMES,
        );
        listed(
            crate::lambda::Strategy::ALL,
            crate::lambda::Strategy::name,
            crate::lambda::Strategy::NAMES,
        );
        for &logic in Logic::ALL {
            assert_eq!(logic.name().parse::<Logic>().unwrap(), logic);
        }
        for &translation in Translation::ALL {
            assert_eq!(
                translation.name().parse::<Translation>().unwrap(),
                translation
            );
        }
        assert!(matches!(
            "cbz".parse::<Translation>(),
            Err(Error::UnknownName { known, .. }) if known == Translation::NAMES
        ));
        let bound = |n| crate::Limits::default().with_occurrences(Some(n));
        let refused = |result: Result<(), Error>| {
            matches!(result, Err(Error::Refused(Refusal::Occurrences { .. })))
        };
        // Five formulas: a, b, c, a ∧ b and the implication.
        assert!(Sequent::parse_within("a /\\ b -> c", &bound(5)).is_ok());
        assert!(refused(
            Sequent::parse_within("a /\\ b -> c", &bound(4)).map(|_| ())
        ));
        let tptp = "fof(c, conjecture, (a & b) => c).";
        assert!(read_tptp(tptp, &bound(5)).is_ok());
        assert!(refused(read_tptp(tptp, &bound(4)).map(|_| ())));
        // One atom repeated: one formula of the arena, but each repeat is
        // an occurrence of the image.
        let repeated = ["a"; 6].join(", ");
        assert!(Sequent::parse_within(&repeated, &bound(6)).is_ok());
        assert!(refused(
            Sequent::parse_within(&repeated, &bound(5)).map(|_| ())
        ));
        // A parenthesis waits but makes no formula: `~(~(a))` is three
        // formulas and read at a bound of three.
        assert!(Sequent::parse_within("~(~(a))", &bound(3)).is_ok());
        // What waits for its operand counts too: a run of `~` or of `->`
        // is refused before the parse error after it.
        for run in ["~".repeat(100), "a -> ".repeat(100), "(".repeat(100)] {
            assert!(refused(
                Sequent::parse_within(&format!("{run}a b"), &bound(10)).map(|_| ())
            ));
        }
        let hypotheses = "fof(h, axiom, a).\n".repeat(5) + "fof(c, conjecture, a).";
        assert!(read_tptp(&hypotheses, &bound(6)).is_ok());
        assert!(refused(read_tptp(&hypotheses, &bound(5)).map(|_| ())));
        let sequent: Sequent = "a /\\ b -> c".parse().unwrap();
        let image = translate(
            &sequent,
            Logic::Classical,
            Translation::Affine,
            &bound(1_000),
        )
        .unwrap();
        let size = image.sequent().occurrences();
        let classical = |n| translate(&sequent, Logic::Classical, Translation::Affine, &bound(n));
        assert!(classical(size).is_ok());
        assert!(refused(classical(size - 1).map(|_| ())));
    }

    /// A name is one atom however its accents are encoded, and a keyword
    /// of either syntax names none.
    #[test]
    fn atom_names_are_composed_identifiers() {
        let mut formulas = Formulas::default();
        let composed = formulas.atom("\u{e9}").unwrap();
        assert_eq!(formulas.atom("e\u{301}").unwrap(), composed);
        assert_eq!(formulas.atom_names(), ["\u{e9}"]);
        for name in ["top", "true", "forall", "a b"] {
            assert!(
                matches!(formulas.atom(name), Err(Error::AtomName { .. })),
                "{name}"
            );
        }
    }

    /// An arena refuses a subformula of another arena and an atom
    /// outside its names, and a sequent an id outside its arena.
    #[test]
    fn foreign_ids_are_refused() {
        let mut other = Formulas::default();
        let (a, b) = (other.atom("a").unwrap(), other.atom("b").unwrap());
        let and = other.add(Node::And(a, b)).unwrap();
        let mut formulas = Formulas::default();
        let x = formulas.atom("x").unwrap();
        let refused = |result: Result<_, Error>, space| matches!(result, Err(Error::IndexOutOfBounds { space: s, .. }) if s == space);
        assert!(refused(formulas.add(Node::Not(and)), Space::Formula));
        assert!(refused(formulas.add(Node::Atom(5)), Space::Atom));
        let sequent = Sequent::new(formulas.clone(), vec![x], vec![and]);
        assert!(refused(sequent.map(|_| x), Space::Formula));
        assert!(Sequent::new(formulas, vec![], vec![x]).is_ok());
    }

    /// A TPTP problem's atom `false`, which the ordinary syntax reads as
    /// the constant and minimal logic's false is the atom of, is renamed:
    /// it never stands for minimal logic's false.
    #[test]
    fn false_is_no_atom_of_the_sequent() {
        let read = read_tptp(
            "fof(a, axiom, p). fof(b, axiom, ~p). fof(c, conjecture, false).",
            &crate::Limits::default(),
        )
        .unwrap();
        let names = read.sequent.formulas().atom_names();
        assert!(names.iter().all(|n| n != FALSE), "{names:?}");
        assert_eq!(read.sequent.to_string(), "p, ¬p ⊢ false_");
    }

    /// Every translation decides the sequents of its logic as the logic
    /// does, and every proof reads back as a derivation its checker
    /// accepts: the connectives on both sides, nested negations, the
    /// constants, contraction, and the sequents that separate the logics.
    #[test]
    fn decides_and_reads_back() {
        // (sequent, classical, intuitionistic, minimal)
        let cases = [
            ("a -> a", true, true, true),
            ("a \\/ ~a", true, false, false),
            ("~~a -> a", true, false, false),
            ("a -> ~~a", true, true, true),
            ("((a -> b) -> a) -> a", true, false, false),
            ("false -> a", true, true, false),
            ("~a -> a -> b", true, true, false),
            ("~a -> a -> ~b", true, true, true),
            ("a /\\ b <-> b /\\ a", true, true, true),
            ("a \\/ b, ~a |- b", true, true, false),
            ("(a -> b) /\\ (a -> c) |- a -> b /\\ c", true, true, true),
            ("a -> b, b -> c |- a -> c", true, true, true),
            ("~(a \\/ b) <-> ~a /\\ ~b", true, true, true),
            ("~(a /\\ b) -> ~a \\/ ~b", true, false, false),
            ("true, a |- a /\\ true", true, true, true),
            ("a -> (a -> b) -> b", true, true, true),
            ("(a <-> b) -> (b <-> a)", true, true, true),
            ("a |- b", false, false, false),
            ("a \\/ b |- a /\\ b", false, false, false),
            ("~~(a \\/ ~a)", true, true, true),
            // Minimal logic's false is an atom, copied like any other.
            ("false |- ~true", true, true, true),
            ("false |- a -> a", true, true, true),
            ("q |- r -> ((false \\/ q) -> r)", true, true, true),
        ];
        for (text, classical, intuitionistic, minimal) in cases {
            assert_eq!(
                decide(text, Logic::Classical, Translation::Affine),
                Some(classical),
                "{text} classically"
            );
            for translation in [
                Translation::CallByName,
                Translation::CallByValue,
                Translation::ZeroOne,
            ] {
                for (logic, valid) in [
                    (Logic::Intuitionistic, intuitionistic),
                    (Logic::Minimal, minimal),
                ] {
                    let verdict = decide(text, logic, translation);
                    assert!(
                        verdict.is_none() || verdict == Some(valid),
                        "{text} in {logic} logic by {translation}: {verdict:?}"
                    );
                    if valid {
                        assert_eq!(
                            verdict,
                            Some(true),
                            "{text} in {logic} logic by {translation}"
                        );
                    }
                }
            }
        }
    }

    /// Returns a formula of depth `depth` at most over `atoms`, true and
    /// false, built with `¬`, `∧`, `∨` and `→`.
    fn generated(rng: &mut Rng, formulas: &mut Formulas, atoms: &[NodeId], depth: usize) -> NodeId {
        if depth == 0 || rng.one_in(4) {
            // Each atom twice as likely as each constant.
            let leaf = rng.below(2 * atoms.len() + 2);
            let node = match leaf.checked_sub(2 * atoms.len()) {
                None => return atoms[leaf % atoms.len()],
                Some(0) => Node::True,
                Some(_) => Node::False,
            };
            return formulas.add(node).unwrap();
        }
        let a = generated(rng, formulas, atoms, depth - 1);
        let node = match rng.below(4) {
            0 => Node::Not(a),
            choice => {
                let b = generated(rng, formulas, atoms, depth - 1);
                match choice {
                    1 => Node::And(a, b),
                    2 => Node::Or(a, b),
                    _ => Node::Implies(a, b),
                }
            }
        };
        formulas.add(node).unwrap()
    }

    /// Returns a sequent in the atoms `a`, `b` and `c` of up to two
    /// hypotheses and at most one formula right of `⊢`, each of depth
    /// three at most.
    pub(super) fn generated_sequent(rng: &mut Rng) -> Sequent {
        let mut formulas = Formulas::default();
        let atoms: Vec<_> = ["a", "b", "c"]
            .into_iter()
            .map(|name| formulas.atom(name).unwrap())
            .collect();
        let left = (0..rng.below(3))
            .map(|_| generated(rng, &mut formulas, &atoms, 3))
            .collect();
        let right = (0..usize::from(!rng.one_in(6)))
            .map(|_| generated(rng, &mut formulas, &atoms, 3))
            .collect();
        Sequent::new(formulas, left, right).unwrap()
    }

    /// Returns whether every assignment of the sequent's atoms that makes
    /// all its hypotheses true makes some formula right of `⊢` true.
    pub(super) fn valid_by_truth_table(sequent: &Sequent) -> bool {
        let formulas = sequent.formulas();
        (0..1_u32 << formulas.atom_names().len()).all(|assignment| {
            // Every node after its operands, so one pass evaluates all.
            let mut value: Vec<bool> = Vec::with_capacity(formulas.len());
            for n in 0..formulas.len() {
                let of = |id: NodeId| value[id.index()];
                let holds = match formulas.node(NodeId(n as u32)) {
                    Node::Atom(atom) => assignment >> atom & 1 == 1,
                    Node::True => true,
                    Node::False => false,
                    Node::Not(a) => !of(a),
                    Node::And(a, b) => of(a) && of(b),
                    Node::Or(a, b) => of(a) || of(b),
                    Node::Implies(a, b) => !of(a) || of(b),
                    Node::Iff(a, b) => of(a) == of(b),
                    Node::Forall(_) | Node::Exists(_) => {
                        unreachable!("the samples are propositional")
                    }
                };
                value.push(holds);
            }
            !sequent.left().iter().all(|id| value[id.index()])
                || sequent.right().iter().any(|id| value[id.index()])
        })
    }

    /// On a few hundred generated sequents, each logic's translations
    /// agree: wherever two of them decide a sequent they give it the same
    /// verdict, and each decides at least half of the sequents. Classical
    /// logic's verdicts are the truth table's, and a sequent valid in
    /// intuitionistic or minimal logic is valid by the truth table.
    #[test]
    fn translations_agree_with_each_other_and_the_truth_table() {
        const SEQUENTS: usize = 300;
        // Bounds that keep the run to a second or two unoptimized.
        let search = Search::default().with_copies(Some(6));
        let limits = crate::Limits::default().with_work(Some(50_000));
        let mut rng = Rng::new(2_026);
        let mut decided: HashMap<(Logic, Translation), usize> = HashMap::default();
        for _ in 0..SEQUENTS {
            let sequent = generated_sequent(&mut rng);
            let classical = valid_by_truth_table(&sequent);
            for &logic in Logic::ALL {
                let verdicts: Vec<_> = Translation::ALL
                    .iter()
                    .filter(|translation| translation.decides(logic))
                    .map(|&translation| {
                        let options = Options::default()
                            .with_logic(logic)
                            .with_translation(Some(translation));
                        let outcome =
                            super::decide_within(&sequent, &options, &search, &limits, |_| false)
                                .unwrap_or_else(|e| {
                                    panic!("{sequent} in {logic} logic by {translation}: {e}")
                                });
                        let valid = match outcome.verdict {
                            super::Verdict::Valid(_) => Some(true),
                            super::Verdict::NotValid => Some(false),
                            super::Verdict::Unknown(_) => None,
                        };
                        if valid.is_some() {
                            *decided.entry((logic, translation)).or_default() += 1;
                        }
                        (translation, valid)
                    })
                    .collect();
                let answers: Vec<bool> = verdicts.iter().filter_map(|&(_, valid)| valid).collect();
                assert!(
                    answers.windows(2).all(|pair| pair[0] == pair[1]),
                    "{sequent} in {logic} logic: {verdicts:?}"
                );
                if logic == Logic::Classical {
                    assert!(
                        answers.iter().all(|&valid| valid == classical),
                        "{sequent} classically: {verdicts:?}, the truth table says {classical}"
                    );
                } else {
                    assert!(
                        classical || !answers.contains(&true),
                        "{sequent} in {logic} logic: {verdicts:?}, the truth table refutes it"
                    );
                }
            }
        }
        for &logic in Logic::ALL {
            for &translation in Translation::ALL.iter().filter(|t| t.decides(logic)) {
                let count = decided.get(&(logic, translation)).copied().unwrap_or(0);
                assert!(
                    2 * count >= SEQUENTS,
                    "{translation} decided {count} of {SEQUENTS} sequents in {logic} logic"
                );
            }
        }
    }
}

#[cfg(test)]
mod first_order_tests {
    use super::*;
    use crate::linear::proofs::InfId;
    use crate::{Error, Limits};

    /// Returns `∀x. p(x) ⊢ ∀x. p(x)` with the axiom that derives it.
    fn quantified() -> (Sequent, Derivation) {
        let mut formulas = Formulas::default();
        let x = formulas.variable(0);
        let body = formulas.predicate("p", &[x]).unwrap();
        let all = formulas.binder(true, body, "x").unwrap();
        let sequent = Sequent::new(formulas.clone(), vec![all], vec![all]).unwrap();
        let axiom = Inference {
            term: None,
            left: vec![all],
            right: vec![all],
            rule: Rule::Axiom,
            principal: None,
            premises: Vec::<InfId>::new(),
        };
        let derivation = Derivation::from_parts(
            Logic::Classical,
            formulas,
            (vec![all], vec![all]),
            vec![axiom],
        );
        (sequent, derivation)
    }

    /// Asserts that `result` is the refusal of a first-order sequent.
    #[track_caller]
    fn refused<T: std::fmt::Debug>(result: Result<T, Error>) {
        match result {
            Err(Error::FirstOrder { .. }) => {}
            other => panic!("expected the first-order refusal, got {other:?}"),
        }
    }

    #[test]
    fn every_propositional_method_refuses_a_first_order_sequent() {
        let (sequent, derivation) = quantified();
        let limits = Limits::default();
        for &logic in Logic::ALL {
            // The embedding decides it, and is the default there.
            for method in [None, Some(Method::Embedding)] {
                let options = Options::default().with_logic(logic).with_method(method);
                let outcome = decide(&sequent, &options).unwrap();
                assert!(matches!(outcome.verdict, Verdict::Valid(_)), "{logic:?}");
            }
            for &method in Method::ALL {
                // The connection prover decides classical first-order
                // sequents and refuses the other logics by name; grounding
                // decides those of its classes.
                if super::decide::takes_first_order(method) {
                    continue;
                }
                let options = Options::default()
                    .with_logic(logic)
                    .with_method(Some(method));
                refused(decide(&sequent, &options));
            }
            refused(ljt::decide(&sequent, logic, &limits, |_| false));
            #[cfg(feature = "sat")]
            {
                refused(sat::decide(&sequent, logic, &limits, |_| false));
                refused(sat::decide_preferring_ljt(&sequent, logic, &limits, |_| {
                    false
                }));
            }
            refused(
                crate::kripke::Model::new(vec![crate::kripke::World::default()], 0).forcing(
                    &sequent,
                    logic,
                    &limits,
                    |_| false,
                ),
            );
        }
        refused(truth_table(&sequent, &limits, |_| false));
        refused(normal::rewrite(
            &sequent,
            &normal::Options::default(),
            &limits,
            |_| false,
        ));
        refused(resolution::Clauses::of_sequent(&sequent));
        // Glivenko's theorem fails at first order, which its refusal says;
        // the other translations take first-order sequents.
        assert!(matches!(
            negative::translate(
                &sequent,
                &negative::Options::default().with_translation(negative::Translation::Glivenko),
                &limits,
            ),
            Err(Error::GlivenkoFirstOrder)
        ));
        refused(derivation.term(&limits, |_| false));
        refused(crate::lambda::enumerate(
            &sequent,
            &crate::lambda::InhabitOptions::default(),
            &limits,
            |_| false,
        ));
        // A normal form's public fields can be given a first-order sequent.
        let mut normal = normal::rewrite(
            &Sequent::new(Formulas::default(), vec![], vec![]).unwrap(),
            &normal::Options::default(),
            &limits,
            |_| false,
        )
        .unwrap();
        normal.sequent = sequent.clone();
        refused(normal.claims());
        #[cfg(feature = "interactive")]
        refused(Tableau::of(&derivation, |_| false));
        #[cfg(feature = "rocq")]
        refused(crate::export::rocq::write(
            &derivation,
            &crate::export::rocq::Options::default(),
            &mut String::new(),
            |_| false,
        ));
        #[cfg(feature = "serialize")]
        refused(crate::rules::Derivation::from_ordinary(&derivation));
        #[cfg(all(feature = "interactive", feature = "parse", feature = "serialize"))]
        {
            let tableau = WrittenTableau::parse_within("1 F p\n", &limits).unwrap();
            let mut json = serde_json::Deserializer::from_str(crate::rules::TABLEAU);
            let calculus =
                crate::wire::upgrade::<crate::rules::Calculus, _>(&mut json, &limits).unwrap();
            refused(tableau.review(&calculus, Some(&sequent), &limits, |_| false));
        }
    }

    #[test]
    fn a_first_order_formula_prints_with_its_binders_renamed_apart() {
        let mut formulas = Formulas::default();
        // `∀y. ∃x. r(y, x)` and `∃x. ∃z. r(x, z)` share `∃x. r(#1, #0)`.
        let (outer, inner) = (formulas.variable(1), formulas.variable(0));
        let r = formulas.predicate("r", &[outer, inner]).unwrap();
        let some = formulas.binder(false, r, "x").unwrap();
        let first = formulas.binder(true, some, "y").unwrap();
        let second = formulas.binder(false, some, "x").unwrap();
        let a = formulas.function("a", &[]).unwrap();
        let p = formulas.predicate("p", &[a]).unwrap();
        let both = formulas.add(Node::And(first, p)).unwrap();
        let sequent = Sequent::new(formulas, vec![both], vec![second]).unwrap();
        assert_eq!(
            sequent.to_string(),
            "(∀y. ∃x. r(y, x)) ∧ p(a) ⊢ ∃x. ∃x_1. r(x, x_1)"
        );
    }

    #[test]
    fn a_formula_with_a_loose_variable_is_no_formula_of_a_sequent() {
        let mut formulas = Formulas::default();
        let x = formulas.variable(0);
        let body = formulas.predicate("p", &[x]).unwrap();
        assert!(matches!(
            Sequent::new(formulas, vec![], vec![body]),
            Err(Error::LooseVariable { .. })
        ));
    }

    /// Reads `json` as a document of the form `T`.
    #[cfg(feature = "serialize")]
    fn read<T: crate::wire::Readable>(json: &str) -> Result<T, Error> {
        let mut json = serde_json::Deserializer::from_str(json);
        crate::wire::upgrade::<T, _>(&mut json, &Limits::default())
    }

    /// The readers hold a document to what the arena promises: closed
    /// formulas in a derivation, each term once, no first-order tableau,
    /// and a document of nullary predicates propositional.
    #[test]
    #[cfg(feature = "serialize")]
    fn documents_are_held_to_the_arena_s_promises() {
        let loose = r#"{"version":25,"logic":"classical","sequent":{"formulas":[{"V":0}],"atoms":[{"P":[0,[0]]}],"symbols":[["p",1]],"fo":[{"B":0}],"left":[0],"right":[0]},"inferences":[{"left":[0],"right":[0],"rule":"ax"}]}"#;
        assert!(matches!(
            read::<Derivation>(loose),
            Err(Error::LooseVariable { .. })
        ));
        // `a` twice: an instance is found however the document names it.
        let twice = r#"{"version":25,"formulas":[{"V":0},{"∀":0},{"V":1}],"atoms":[{"P":[1,[2]]},{"P":[1,[1]]}],"symbols":[["a",0],["p",1]],"fo":[{"F":[0,[]]},{"F":[0,[]]},{"B":0}],"binders":["x"],"left":[2],"right":[1]}"#;
        let sequent: Sequent = read(twice).unwrap();
        let formulas = sequent.formulas();
        let Node::Forall(body) = formulas.node(sequent.right()[0]) else {
            panic!("a binder")
        };
        let a = formulas
            .tables()
            .terms
            .iter()
            .position(|t| matches!(t, crate::first_order::FoTerm::App(..)));
        let a = crate::first_order::FoTermId::new(a.unwrap() as u32);
        assert_eq!(formulas.instance(body, a), Some(sequent.left()[0]));
        #[cfg(feature = "interactive")]
        refused(read::<Tableau>(
            r#"{"version":25,"sequent":{"formulas":[{"V":0},{"∀":0}],"atoms":["q"],"binders":["x"],"left":[1],"right":[1]},"nodes":[{"lines":[["T",1],["F",1]],"end":{"kind":"closed","true_line":0,"false_line":1}}]}"#,
        ));
        let nullary = r#"{"version":25,"formulas":[{"V":0}],"atoms":[{"P":[0,[]]}],"symbols":[["p",0]],"left":[],"right":[0]}"#;
        let sequent: Sequent = read(nullary).unwrap();
        assert!(!sequent.formulas().is_first_order());
        assert_eq!(sequent.to_string(), "⊢ p");
    }

    #[test]
    fn a_name_has_one_role() {
        let mut formulas = Formulas::default();
        let a = formulas.function("a", &[]).unwrap();
        formulas.predicate("p", &[a]).unwrap();
        // `a` names a constant, so it names no atom.
        assert!(matches!(formulas.atom("a"), Err(Error::Symbol { .. })));
        assert!(matches!(
            formulas.predicate("p", &[a, a]),
            Err(Error::Symbol { .. })
        ));
    }
}
