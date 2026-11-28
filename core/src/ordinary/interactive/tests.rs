// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The goal state's tests.

#[cfg(feature = "sat")]
use super::close::{Carry, with_derivation};
use super::*;
#[cfg(feature = "serialize")]
use crate::WitnessFault;
use crate::linear::proofs::{Structural, ViewOptions};
use crate::linear::search;
use crate::linear::search::generate::Rng;
#[cfg(feature = "serialize")]
use crate::ordinary::Derivation;
use crate::ordinary::derivation::check_inference;
#[cfg(feature = "sat")]
use crate::ordinary::transfer::{self, Parameters};
use crate::ordinary::{Evidence, Verdict};
use Side::{Left, Right};

/// Starts a proof of `text` in `logic` and returns it with its goal.
fn start(text: &str, logic: Logic) -> (Interactive, GoalId) {
    let sequent: Sequent = text.parse().unwrap();
    let state = Interactive::new(&sequent, logic).unwrap();
    let goal = state.goals().next().unwrap();
    (state, goal)
}

/// Returns the position on `side` of the goal of the formula printed
/// as `text`.
fn at(state: &Interactive, goal: GoalId, side: Side, text: &str) -> usize {
    let (left, right) = state.goal(goal).unwrap();
    let list = if side == Left { left } else { right };
    list.iter()
        .position(|&k| state.formulas().formula(k).to_string() == text)
        .unwrap_or_else(|| panic!("no {text} {side:?} in goal {}", goal.get()))
}

/// Applies `rule` to the formula printed as `text` on `side`, the
/// formulas printed as `first` going to the first premise if any, and
/// returns the goals opened.
fn step(
    state: &mut Interactive,
    goal: GoalId,
    (side, text): (Side, &str),
    rule: Rule,
    first: Option<(&[&str], &[&str])>,
) -> Vec<GoalId> {
    let mut step = Step::new(side, at(state, goal, side, text), rule);
    if let Some((left, right)) = first {
        let left: Vec<usize> = left.iter().map(|t| at(state, goal, Left, t)).collect();
        let right: Vec<usize> = right.iter().map(|t| at(state, goal, Right, t)).collect();
        step = step.split(&left, &right);
    }
    state
        .apply(goal, &step)
        .unwrap_or_else(|e| panic!("{rule} on {text}: {e}"))
}

/// Closes the goal by the axiom on the formula printed as `text` right
/// of `⊢`.
fn axiom(state: &mut Interactive, goal: GoalId, text: &str) {
    assert_eq!(step(state, goal, (Right, text), Rule::Axiom, None), []);
}

/// A state finished through the rules is a derivation the checker
/// accepts, in each logic: the axiom with a context, a shared and a
/// split context, the negations as implications into ⊥ in LJ and as
/// themselves in LK, an equivalence, and contraction.
#[test]
fn a_finished_state_checks() {
    let finished = |s: &Interactive| {
        assert!(s.is_complete());
        s.finished(&Limits::default(), |_| false).unwrap()
    };
    for logic in [Logic::Intuitionistic, Logic::Minimal] {
        let (mut s, g) = start("a /\\ b |- b /\\ a", logic);
        let [g] = step(&mut s, g, (Left, "a ∧ b"), Rule::AndLeft, None)[..] else {
            panic!()
        };
        for (g, text) in step(&mut s, g, (Right, "b ∧ a"), Rule::AndRight, None)
            .into_iter()
            .zip(["b", "a"])
        {
            axiom(&mut s, g, text);
        }
        finished(&s);
        let (mut s, g) = start("a -> b, a |- b", logic);
        let first: &[&str] = &["a"];
        let [p, q] = step(
            &mut s,
            g,
            (Left, "a → b"),
            Rule::ImpliesLeft,
            Some((first, &[])),
        )[..] else {
            panic!()
        };
        axiom(&mut s, p, "a");
        axiom(&mut s, q, "b");
        finished(&s);
        // a ⊢ ¬¬a: ¬R gives a, ¬a ⊢ ⊥, and ¬L the premises a ⊢ a and
        // ⊥ ⊢ ⊥, an axiom in minimal logic too.
        let (mut s, g) = start("a |- ~~a", logic);
        let [g] = step(&mut s, g, (Right, "¬¬a"), Rule::NotRight, None)[..] else {
            panic!()
        };
        let [p, q] = step(&mut s, g, (Left, "¬a"), Rule::NotLeft, Some((first, &[])))[..] else {
            panic!()
        };
        axiom(&mut s, p, "a");
        axiom(&mut s, q, "⊥");
        let derivation = finished(&s);
        assert_eq!(derivation.sequent().1.len(), 1);
    }
    // Classically: the excluded middle by ¬R as LK has it, Peirce's law
    // with a shared context, and an equivalence.
    let k = Logic::Classical;
    let (mut s, g) = start("|- a \\/ ~a", k);
    let [g] = step(&mut s, g, (Right, "a ∨ ¬a"), Rule::OrRight, None)[..] else {
        panic!()
    };
    let [g] = step(&mut s, g, (Right, "¬a"), Rule::NotRight, None)[..] else {
        panic!()
    };
    assert_eq!(s.goal(g).unwrap().0.len(), 1);
    axiom(&mut s, g, "a");
    finished(&s);
    let (mut s, g) = start("|- ((a -> b) -> a) -> a", k);
    let [g] = step(
        &mut s,
        g,
        (Right, "((a → b) → a) → a"),
        Rule::ImpliesRight,
        None,
    )[..] else {
        panic!()
    };
    let [p, q] = step(&mut s, g, (Left, "(a → b) → a"), Rule::ImpliesLeft, None)[..] else {
        panic!()
    };
    let [p] = step(&mut s, p, (Right, "a → b"), Rule::ImpliesRight, None)[..] else {
        panic!()
    };
    axiom(&mut s, p, "a");
    axiom(&mut s, q, "a");
    finished(&s);
    let (mut s, g) = start("a <-> b, b |- a", k);
    let [g] = step(&mut s, g, (Left, "a ↔ b"), Rule::IffLeft2, None)[..] else {
        panic!()
    };
    let [g] = step(&mut s, g, (Left, "b"), Rule::ContractLeft, None)[..] else {
        panic!()
    };
    let [p, q] = step(
        &mut s,
        g,
        (Left, "b → a"),
        Rule::ImpliesLeft,
        Some((&["b"], &[])),
    )[..] else {
        panic!()
    };
    axiom(&mut s, p, "b");
    axiom(&mut s, q, "a");
    finished(&s);
}

/// The same derivation drawn with its structural rules explicit and
/// admissible: the weakenings below an axiom and the contraction below
/// the rule that uses both copies are left out, their conclusions the
/// rule's.
#[test]
fn structural_rules_explicit_or_admissible() {
    let (mut s, g) = start("a -> b, a, c |- b", Logic::Intuitionistic);
    let [g] = step(&mut s, g, (Left, "a"), Rule::ContractLeft, None)[..] else {
        panic!()
    };
    let first: &[&str] = &["a"];
    let [p, q] = step(
        &mut s,
        g,
        (Left, "a → b"),
        Rule::ImpliesLeft,
        Some((first, &[])),
    )[..] else {
        panic!()
    };
    axiom(&mut s, p, "a");
    axiom(&mut s, q, "b");
    let view = |structural| {
        let view = ViewOptions::default().with_structural(structural);
        let derivation = s
            .derivation_within(&view, &Limits::default(), |_| false)
            .unwrap();
        (derivation.to_string(), s.derivation_ids(structural))
    };
    let (explicit, ids) = view(Structural::Explicit);
    assert_eq!(
        explicit,
        "             ───── ax\n\
            \x20            b ⊢ b\n\
            \x20           ──────── WL\n\
            \x20           b, a ⊢ b\n\
             ───── ax   ─────────── WL\n\
             a ⊢ a      b, a, c ⊢ b\n\
             ────────────────────── →L\n\
            \x20 a → b, a, a, c ⊢ b\n\
            \x20 ────────────────── CL\n\
            \x20  a → b, a, c ⊢ b"
    );
    assert_eq!(ids.len(), 6);
    let (admissible, ids) = view(Structural::Admissible);
    assert_eq!(
        admissible,
        "───── ax   ─────────── ax\n\
             a ⊢ a      b, a, c ⊢ b\n\
             ────────────────────── →L\n\
            \x20  a → b, a, c ⊢ b"
    );
    // The root shown is the →L inference, number 3 of the state.
    assert_eq!(ids.len(), 3);
    assert_eq!(s.inferences[ids[2].index()].rule, Rule::ImpliesLeft);
    // An open goal keeps the weakening above which it stands.
    let (mut s, g) = start("a, c |- a", Logic::Intuitionistic);
    step(&mut s, g, (Left, "c"), Rule::WeakenLeft, None);
    let view = ViewOptions::default().with_structural(Structural::Admissible);
    let shown = s
        .derivation_within(&view, &Limits::default(), |_| false)
        .unwrap();
    assert_eq!(shown.to_string(), " a ⊢ a\n──────── WL\na, c ⊢ a");
}

/// LJ's →L shares its context by default: both premises keep every
/// hypothesis, the goal right of ⊢ goes to the second, the
/// contractions the checker needs are made below and left out of the
/// admissible drawing, and the finished derivation checks.
#[test]
fn a_shared_implication_left_checks() {
    for logic in [Logic::Intuitionistic, Logic::Minimal] {
        let (mut s, g) = start("c, a -> b, a |- b", logic);
        let [p, q] = step(&mut s, g, (Left, "a → b"), Rule::ImpliesLeft, None)[..] else {
            panic!()
        };
        let text = |s: &Interactive, g| {
            let (left, right) = s.goal(g).unwrap();
            let side = |list: &[NodeId]| -> Vec<String> {
                list.iter()
                    .map(|&k| s.formulas().formula(k).to_string())
                    .collect()
            };
            (side(left), side(right))
        };
        assert_eq!(
            text(&s, p),
            (vec!["c".into(), "a".into()], vec!["a".into()])
        );
        assert_eq!(
            text(&s, q),
            (vec!["c".into(), "b".into(), "a".into()], vec!["b".into()])
        );
        axiom(&mut s, p, "a");
        axiom(&mut s, q, "b");
        let derivation = s.finished(&Limits::default(), |_| false).unwrap();
        let contractions = derivation
            .inferences()
            .iter()
            .filter(|i| i.rule() == Rule::ContractLeft)
            .count();
        assert_eq!(contractions, 2);
        let shown = s.derivation().unwrap().to_string();
        assert_eq!(
            shown,
            "──────── ax   ─────────── ax\n\
                 c, a ⊢ a      c, b, a ⊢ b\n\
                 ───────────────────────── →L\n\
                \x20    c, a → b, a ⊢ b"
        );
        assert_eq!(s.undo(), Some(q));
    }
}

/// Undo takes back the last step, a rule applied or a goal closed by
/// the search, and the state is the one before it.
#[test]
fn undo_restores() {
    let (mut s, g) = start("a /\\ b, c |- b /\\ a", Logic::Intuitionistic);
    let parts = |s: &Interactive| (s.inferences.clone(), s.history.clone());
    let before = parts(&s);
    let [h] = step(&mut s, g, (Left, "a ∧ b"), Rule::AndLeft, None)[..] else {
        panic!()
    };
    let middle = parts(&s);
    let goals = step(&mut s, h, (Right, "b ∧ a"), Rule::AndRight, None);
    axiom(&mut s, goals[0], "b");
    let closed = s
        .close(
            goals[1],
            None,
            &search::Options::default(),
            &Limits::default(),
            |_| false,
        )
        .unwrap();
    assert!(matches!(closed.outcome.verdict, Verdict::Valid(_)) && closed.grafted.is_ok());
    assert!(s.is_complete());
    assert_eq!(s.steps(), 4);
    for _ in 0..3 {
        s.undo();
    }
    assert_eq!(parts(&s), middle);
    assert_eq!(s.undo(), Some(g));
    assert_eq!(parts(&s), before);
    assert_eq!(s.undo(), None);
}

/// A rule refused on a goal it does not fit says why, and changes
/// nothing.
#[test]
fn a_rule_that_does_not_fit_is_refused_with_why() {
    let (mut s, g) = start("a /\\ b, a -> c |- c \\/ a", Logic::Intuitionistic);
    let before = (s.inferences.clone(), s.history.clone());
    let refused = |s: &mut Interactive, step: Step| s.apply(g, &step).unwrap_err();
    let error = refused(&mut s, Step::new(Left, 0, Rule::AndRight));
    assert_eq!(
        error,
        StepError::Rule {
            rule: Rule::AndRight,
            side: Left,
            position: 0
        }
    );
    assert_eq!(
        error.to_string(),
        "the rule ∧R does not act on formula 0 left of ⊢: it acts on a conjunction right of ⊢"
    );
    let error = refused(&mut s, Step::new(Right, 0, Rule::OrRight));
    assert_eq!(
        error.to_string(),
        "the rule ∨R is not available in intuitionistic logic: its premise would have both \
             disjuncts right of ⊢, and every goal there has exactly one formula: use ∨R₁ or ∨R₂ \
             to keep one disjunct"
    );
    // →L sending c ∨ a to the first premise leaves it beside a there.
    let error = refused(
        &mut s,
        Step::new(Left, 1, Rule::ImpliesLeft).split(&[], &[0]),
    );
    assert_eq!(error, StepError::Succedents { count: 2 });
    assert!(
        error
            .to_string()
            .starts_with("a premise would have 2 formulas right of ⊢")
    );
    assert_eq!(
        refused(&mut s, Step::new(Left, 0, Rule::Axiom)).to_string(),
        "the axiom needs formula 0 left of ⊢ to stand right of ⊢ too"
    );
    assert_eq!(
        refused(
            &mut s,
            Step::new(Left, 1, Rule::ImpliesLeft).split(&[1], &[])
        ),
        StepError::Split {
            side: Left,
            position: 1
        }
    );
    assert_eq!(
        refused(&mut s, Step::new(Left, 0, Rule::AndLeft).split(&[1], &[])),
        StepError::NoSplit {
            rule: Rule::AndLeft
        }
    );
    assert_eq!(
        refused(&mut s, Step::new(Right, 3, Rule::OrRight1)).to_string(),
        "the goal has no formula 3 right of ⊢: it has 1 formula there, and positions start at 0"
    );
    assert_eq!(
        s.apply(GoalId::new(9), &Step::new(Left, 0, Rule::AndLeft)),
        Err(StepError::NoGoal {
            goal: GoalId::new(9)
        })
    );
    assert_eq!((s.inferences.clone(), s.history.clone()), before);
    let (mut s, g) = start("false |- a", Logic::Minimal);
    assert_eq!(
        s.apply(g, &Step::new(Left, 0, Rule::FalseLeft))
            .unwrap_err()
            .to_string(),
        "the rule ⊥L is not available in minimal logic: minimal logic has no ex falso"
    );
    let rules = |s: &Interactive, side, position| -> Vec<Rule> {
        s.rules(g, side, position)
            .unwrap()
            .into_iter()
            .map(|a| a.rule)
            .collect()
    };
    assert_eq!(rules(&s, Left, 0), [Rule::WeakenLeft, Rule::ContractLeft]);
    let (s, g) = start("a -> b, a |- b", Logic::Intuitionistic);
    let applicable = s.rules(g, Left, 0).unwrap();
    assert_eq!(
        applicable,
        [
            Applicable {
                rule: Rule::WeakenLeft,
                needs: Needs::Nothing
            },
            Applicable {
                rule: Rule::ContractLeft,
                needs: Needs::Nothing
            },
            Applicable {
                rule: Rule::ImpliesLeft,
                needs: Needs::Context
            },
        ]
    );
    let (s, g) = start("a -> b, a |- a", Logic::Intuitionistic);
    assert_eq!(s.rules(g, Left, 1).unwrap()[0].rule, Rule::Axiom);
}

/// The search closes a goal it proves and leaves one it cannot decide
/// open, saying why; a hint is the first step of the derivation found,
/// which applies to the goal.
#[test]
fn close_and_hint() {
    let options = search::Options::default();
    let limits = Limits::default();
    let (mut s, g) = start("a -> b, b -> c |- a -> c", Logic::Intuitionistic);
    let hint = s.hint(g, None, &options, &limits, |_| false).unwrap();
    let [step] = &hint.steps[..] else {
        panic!("{:?}", hint.steps)
    };
    assert_eq!((step.side, step.rule), (Right, Rule::ImpliesRight));
    let [g] = s.apply(g, step).unwrap()[..] else {
        panic!()
    };
    // The hint of a split →L names the split the derivation made.
    let hint = s.hint(g, None, &options, &limits, |_| false).unwrap();
    let [step] = &hint.steps[..] else {
        panic!("{:?}", hint.steps)
    };
    assert_eq!(step.rule, Rule::ImpliesLeft);
    assert!(matches!(step.context, Context::Split { .. }));
    s.apply(g, step).unwrap();
    for goal in s.goals().collect::<Vec<_>>() {
        let closed = s.close(goal, None, &options, &limits, |_| false).unwrap();
        assert!(matches!(closed.outcome.verdict, Verdict::Valid(_)));
    }
    s.finished(&limits, |_| false).unwrap();
    // ¬¬(a ∨ ¬a) uses its hypothesis twice: a search allowed one copy
    // of each `!` cannot decide it.
    let (mut s, g) = start("|- ~~(a \\/ ~a)", Logic::Intuitionistic);
    let one = options.clone().with_copies(Some(1));
    let closed = s.close(g, None, &one, &limits, |_| false).unwrap();
    let Verdict::Unknown(reason) = closed.outcome.verdict else {
        panic!("{:?}", closed.outcome.verdict)
    };
    assert_eq!(reason, search::Reason::CopyBound { copies: 1 });
    assert_eq!(s.goals().collect::<Vec<_>>(), [g]);
    assert_eq!(s.hint(g, None, &one, &limits, |_| false).unwrap().steps, []);
    // Peirce's law is not intuitionistic: the goal stays open.
    let (mut s, g) = start("|- ((a -> b) -> a) -> a", Logic::Intuitionistic);
    let closed = s.close(g, None, &options, &limits, |_| false).unwrap();
    assert!(matches!(closed.outcome.verdict, Verdict::NotValid));
    assert_eq!(s.steps(), 0);
}

/// Following the hints proves a sequent whose derivation contracts a
/// hypothesis at its root: the contraction comes with the rule that
/// uses both copies. Hinted alone, it could be answered by a weakening
/// of the copy, and the two would alternate for ever.
#[test]
fn following_the_hints_proves_a_sequent() {
    // Every sequent is valid, so a search that deepens its copy bound
    // ends.
    let options = search::Options::default().with_copies(None);
    let limits = Limits::default();
    for (text, logic) in [
        (
            "a /\\ (b \\/ c) |- (a /\\ b) \\/ (a /\\ c)",
            Logic::Intuitionistic,
        ),
        ("|- ~~(a \\/ ~a)", Logic::Intuitionistic),
        ("|- ~~(a \\/ ~a)", Logic::Minimal),
        ("(a -> b) -> a, a -> b |- b", Logic::Intuitionistic),
    ] {
        let (mut s, _) = start(text, logic);
        // Each round takes at least one rule that is not structural.
        for _ in 0..50 {
            let Some(goal) = s.goals().next() else { break };
            let hint = s.hint(goal, None, &options, &limits, |_| false).unwrap();
            let mut goal = goal;
            for step in &hint.steps {
                let opened = s.apply(goal, step).unwrap();
                if let [next, ..] = opened[..] {
                    goal = next;
                }
            }
            assert!(!hint.steps.is_empty(), "{text}");
        }
        assert!(s.is_complete(), "{text}");
        s.finished(&limits, |_| false).unwrap();
    }
}

/// The sequent's size, the derivation's estimate and the stop bound
/// what a state makes.
#[test]
fn the_bounds_hold() {
    let sequent: Sequent = "a /\\ b |- b /\\ a".parse().unwrap();
    // Six occurrences: a ∧ b, a, b, and b ∧ a, b, a.
    let at = |n| Limits::default().with_occurrences(Some(n));
    assert!(Interactive::within(&sequent, Logic::Classical, &at(6)).is_ok());
    assert!(matches!(
        Interactive::within(&sequent, Logic::Classical, &at(5)),
        Err(Error::Refused(Refusal::Occurrences {
            occurrences: 6,
            limit: 5
        }))
    ));
    let s = Interactive::new(&sequent, Logic::Classical).unwrap();
    let view = ViewOptions::default();
    let small = Limits::default().with_derivation_bytes(Some(100));
    assert!(matches!(
        s.derivation_within(&view, &small, |_| false),
        Err(Error::Refused(Refusal::Output { .. }))
    ));
    assert!(matches!(
        s.derivation_within(&view, &Limits::default(), |_| true),
        Err(Error::Refused(Refusal::Stopped { .. }))
    ));
    let mut s = s;
    let g = s.goals().next().unwrap();
    let stopped = s
        .close(
            g,
            None,
            &search::Options::default(),
            &Limits::default(),
            |_| true,
        )
        .unwrap();
    assert!(matches!(
        stopped.outcome.verdict,
        Verdict::Unknown(search::Reason::Stopped)
    ));
    assert_eq!(s.goals().count(), 1);
    assert!(matches!(
        s.finished(&Limits::default(), |_| false),
        Err(Error::OpenGoals { count: 1 })
    ));
    // A shared →L over 200 hypotheses makes 199 contractions of about
    // 400 formulas each: past a bound of a megabyte, refused before
    // anything is made.
    let hypotheses: Vec<String> = (0..200).map(|i| format!("p{i}")).collect();
    let text = format!("a -> b, {}, a |- b", hypotheses.join(", "));
    let sequent: Sequent = text.parse().unwrap();
    let small = Limits::default().with_derivation_bytes(Some(1 << 20));
    let mut s = Interactive::within(&sequent, Logic::Intuitionistic, &small).unwrap();
    let g = s.goals().next().unwrap();
    let refused = s.apply(g, &Step::new(Left, 0, Rule::ImpliesLeft));
    assert!(matches!(
        refused,
        Err(StepError::Size {
            limit_bytes: 1_048_576,
            ..
        })
    ));
    assert_eq!(s.steps(), 0);
    let mut s = Interactive::new(&sequent, Logic::Intuitionistic).unwrap();
    assert_eq!(
        s.apply(g, &Step::new(Left, 0, Rule::ImpliesLeft))
            .map(|g| g.len()),
        Ok(2)
    );
}

/// A step by hand: the side and position of its formula, its rule and
/// its witness's text.
#[cfg(feature = "serialize")]
type HandStep = (Side, usize, Rule, Option<&'static str>);

/// A quantifier step's term written out reads back in a session of
/// the same sequent, so the steps alone make the same document again:
/// an eigenvariable, a given witness, and one left open that the axiom
/// fills.
#[test]
#[cfg(feature = "serialize")]
fn a_step_s_term_written_out_replays_the_proof() {
    use Rule::{Axiom, ExistsRight, ForallLeft, ForallRight};
    let cases: [(&str, &[HandStep]); 2] = [
        (
            "forall x. p(x) |- forall y. p(f(y))",
            &[
                (Right, 0, ForallRight, None),
                (Left, 0, ForallLeft, Some("f(a)")),
                (Left, 0, Axiom, None),
            ],
        ),
        (
            "forall x. p(x) |- p(c)",
            &[(Left, 0, ForallLeft, None), (Left, 0, Axiom, None)],
        ),
    ];
    for (text, steps) in cases {
        let sequent: Sequent = text.parse().unwrap();
        let mut s = Interactive::new(&sequent, Logic::Classical).unwrap();
        let mut goals = Vec::new();
        for &(side, position, rule, witness) in steps {
            let goal = s.goals().next().unwrap();
            let mut step = Step::new(side, position, rule);
            if let Some(witness) = witness {
                step = step.with_witness(s.witness(goal, witness).unwrap());
            }
            s.apply(goal, &step).unwrap();
            goals.push(goal);
        }
        assert!(s.is_complete());
        // The steps again, each witness from the term written out.
        let mut again = Interactive::new(&sequent, Logic::Classical).unwrap();
        for (&(side, position, rule, _), &goal) in steps.iter().zip(&goals) {
            let mut step = Step::new(side, position, rule);
            if let Some(term) = s.opened_term(goal)
                && matches!(rule, ForallLeft | ExistsRight)
                && !term.starts_with('$')
            {
                step = step.with_witness(again.witness(goal, &term).unwrap());
            }
            again.apply(goal, &step).unwrap();
        }
        assert_eq!(
            serde_json::to_string(&again).unwrap(),
            serde_json::to_string(&s).unwrap(),
            "{text}"
        );
        let terms: Vec<Option<String>> = (0..s.inferences.len() as u32)
            .map(|g| s.opened_term(GoalId::new(g)))
            .collect();
        assert!(terms.contains(&Some(witness_or_open(steps))), "{terms:?}");
    }
}

/// Returns the term the first quantifier step of `steps` that takes a
/// witness is written with: the witness given, or `$1`.
#[cfg(feature = "serialize")]
fn witness_or_open(steps: &[HandStep]) -> String {
    use Rule::ForallLeft;
    steps
        .iter()
        .find(|step| step.2 == ForallLeft)
        .map_or("$1", |step| step.3.unwrap_or("$1"))
        .to_owned()
}

/// Every candidate's status is what `apply` says of its step with the
/// default input, and every rule `rules` offers is a candidate that is
/// not refused: on generated sequents in every logic and a few with
/// quantifiers, along random walks.
#[test]
fn candidates_agree_with_apply() {
    let mut rng = Rng::new(81);
    let mut sequents: Vec<Sequent> = [
        "forall x. p(x) |- exists y. p(y)",
        "p(a), forall x. (p(x) -> q(x)) |- q(a)",
        "exists x. p(x) |- forall y. p(y)",
    ]
    .iter()
    .map(|t| t.parse().unwrap())
    .collect();
    sequents.extend((0..40).map(|_| super::super::tests::generated_sequent(&mut rng)));
    let mut judged = 0;
    for sequent in &sequents {
        for &logic in Logic::ALL {
            let Ok(mut s) = Interactive::new(sequent, logic) else {
                continue;
            };
            for _ in 0..8 {
                let goals: Vec<GoalId> = s.goals().collect();
                if goals.is_empty() {
                    break;
                }
                let goal = goals[rng.below(goals.len())];
                let (left, right) = s.goal(goal).unwrap();
                let places: Vec<(Side, usize)> = (0..left.len())
                    .map(|p| (Left, p))
                    .chain((0..right.len()).map(|p| (Right, p)))
                    .collect();
                let mut legal = Vec::new();
                for &(side, position) in &places {
                    let candidates = s.candidates(goal, side, position).unwrap();
                    for offered in s.rules(goal, side, position).unwrap() {
                        assert!(
                            candidates.iter().any(|c| c.rule == offered.rule
                                && !matches!(c.status, Status::Refused(_))),
                            "{} offered in {sequent} ({logic})",
                            offered.rule
                        );
                    }
                    for Candidate { rule, status } in candidates {
                        judged += 1;
                        let step = Step::new(side, position, rule);
                        let tried = s.clone().apply(goal, &step);
                        match status {
                            Status::Applies | Status::NeedsInput { .. } => {
                                assert!(tried.is_ok(), "{rule} in {sequent} ({logic})");
                                legal.push(step);
                            }
                            Status::Refused(error) => assert_eq!(tried.unwrap_err(), error),
                        }
                    }
                }
                if legal.is_empty() {
                    break;
                }
                let step = &legal[rng.below(legal.len())];
                s.apply(goal, step).unwrap();
            }
        }
    }
    assert!(judged > 1_000, "{judged}");
}

/// On generated sequents in every logic, random steps with random
/// splits and undos never make an inference the checker refuses, and
/// every state whose open goals the search then proves finishes as a
/// derivation the checker accepts; written as JSON, each state reads
/// back as itself.
#[test]
fn random_steps_finish_as_checked_derivations() {
    const SEQUENTS: usize = 60;
    let search = search::Options::default().with_copies(Some(4));
    let limits = Limits::default().with_work(Some(20_000));
    let mut rng = Rng::new(49);
    let mut finished = 0;
    for _ in 0..SEQUENTS {
        let sequent = super::super::tests::generated_sequent(&mut rng);
        for &logic in Logic::ALL {
            let Ok(mut s) = Interactive::new(&sequent, logic) else {
                continue;
            };
            for _ in 0..12 {
                if rng.one_in(6) {
                    s.undo();
                    continue;
                }
                let goals: Vec<GoalId> = s.goals().collect();
                let Some(&goal) = goals.get(rng.below(goals.len().max(1))) else {
                    break;
                };
                let (left, right) = s.goal(goal).unwrap();
                let (nl, nr) = (left.len(), right.len());
                if nl + nr == 0 {
                    break;
                }
                let k = rng.below(nl + nr);
                let (side, position) = if k < nl { (Left, k) } else { (Right, k - nl) };
                let rules = s.rules(goal, side, position).unwrap();
                let Some(applicable) = rules.get(rng.below(rules.len().max(1))) else {
                    continue;
                };
                let mut step = Step::new(side, position, applicable.rule);
                if applicable.needs == Needs::Context && rng.one_in(2) {
                    let pick = |n: usize, rng: &mut Rng, s: Side| -> Vec<usize> {
                        (0..n)
                            .filter(|&i| !(s == side && i == position) && rng.one_in(2))
                            .collect()
                    };
                    let first = (pick(nl, &mut rng, Left), pick(nr, &mut rng, Right));
                    step = step.split(&first.0, &first.1);
                }
                let _ = s.apply(goal, &step);
            }
            #[cfg(feature = "serialize")]
            {
                let json = serde_json::to_string(&s).unwrap();
                let mut document = serde_json::Deserializer::from_str(&json);
                let read: Interactive = crate::wire::upgrade(&mut document, &limits)
                    .unwrap_or_else(|e| panic!("{sequent} in {logic}: {e}\n{json}"));
                assert_eq!(serde_json::to_string(&read).unwrap(), json);
            }
            let goals: Vec<GoalId> = s.goals().collect();
            let proved = goals.into_iter().all(|goal| {
                s.close(goal, None, &search, &limits, |_| false)
                    .is_ok_and(|c| matches!(c.outcome.verdict, Verdict::Valid(_)))
            });
            if proved {
                s.finished(&Limits::default(), |_| false)
                    .unwrap_or_else(|e| panic!("{sequent} in {logic}: {e}"));
                finished += 1;
            }
        }
    }
    // Random steps often weaken a formula the goal needs: a quarter
    // of the states finishing keeps the check from being vacuous.
    assert!(finished >= SEQUENTS / 4, "{finished} states finished");
}

/// A document that no sequence of steps writes is refused: an
/// inference that breaks its rule, a step that names no inference it
/// could have closed, and a derivation that is no tree.
#[cfg(feature = "serialize")]
#[test]
fn a_forged_document_is_refused() {
    let read = |json: &str| {
        let mut document = serde_json::Deserializer::from_str(json);
        crate::wire::upgrade::<Interactive, _>(&mut document, &Limits::default()).map(|_| ())
    };
    let session = r#"{"version":8,"logic":"intuitionistic","sequent":{"formulas":[{"V":0},{"V":1},{"→":[0,1]},"⊥"],"atoms":["a","b"],"left":[2,0],"right":[1]},"inferences":[{"left":[2,0],"right":[1],"rule":"→L","principal":["left",0],"premises":[1,2]},{"left":[0],"right":[0],"rule":"ax"},{"left":[1],"right":[1]}],"history":[0,1]}"#;
    assert_eq!(read(session), Ok(()));
    let inconsistent = |json: &str| matches!(read(json), Err(Error::InconsistentSession { .. }));
    assert!(inconsistent(
        &session.replace(r#""rule":"ax""#, r#""rule":"⊤R""#)
    ));
    assert!(inconsistent(&session.replace(
        r#""left":[0],"right":[0]"#,
        r#""left":[1],"right":[1]"#
    )));
    assert!(inconsistent(
        &session.replace(r#""history":[0,1]"#, r#""history":[1,0]"#)
    ));
    let derivation = r#"{"version":8,"logic":"classical","sequent":{"formulas":[{"V":0}],"atoms":["a"],"left":[0],"right":[0]},"inferences":[{"left":[0],"right":[0],"rule":"ax"},{"left":[0],"right":[0],"rule":"ax"}]}"#;
    let derivation_of = |json: &str| {
        let mut document = serde_json::Deserializer::from_str(json);
        crate::wire::upgrade::<Derivation, _>(&mut document, &Limits::default())
    };
    assert!(matches!(derivation_of(derivation), Err(Error::Json { .. })));
    // An open goal must be a leaf: drawn bare with a premise above it,
    // it would underflow the layout's height.
    let open = r#"{"version":8,"logic":"classical","sequent":{"formulas":[{"V":0},{"V":1}],"atoms":["a","b"],"left":[1],"right":[1]},"inferences":[{"left":[1],"right":[1],"rule":"ax"},{"left":[1],"right":[1],"rule":"open","premises":[0]}]}"#;
    assert!(matches!(derivation_of(open), Err(Error::Json { .. })));
    // A step of LJ that closed `a, ¬a ⊢`: undone, it would reopen a
    // goal without a formula right of ⊢, whose search's derivation
    // concludes `a, ¬a ⊢ ⊥`.
    let empty = r#"{"version":8,"logic":"intuitionistic","sequent":{"formulas":[{"V":0},{"V":1},{"¬":0}],"atoms":["a","b"],"left":[0,2],"right":[1]},"inferences":[{"left":[0,2],"right":[1],"rule":"WR","principal":["right",0],"premises":[1]},{"left":[0,2],"right":[],"rule":"¬L","principal":["left",1],"premises":[2]},{"left":[0],"right":[0],"rule":"ax"}],"history":[0,1,2]}"#;
    assert!(inconsistent(empty));
    // LK's ¬R has its own form only, which its certificate reads.
    let arrow = r#"{"version":8,"logic":"classical","sequent":{"formulas":[{"V":0},"⊥",{"¬":0}],"atoms":["a"],"left":[1],"right":[2]},"inferences":[{"left":[1],"right":[1],"rule":"ax"},{"left":[1,0],"right":[1],"rule":"WL","principal":["left",1],"premises":[0]},{"left":[1],"right":[2],"rule":"¬R","principal":["right",0],"premises":[1]}]}"#;
    let arrow = derivation_of(arrow).unwrap();
    assert!(matches!(
        arrow.check(&Limits::default(), |_| false),
        Err(Error::InvalidDerivation { .. })
    ));
    #[cfg(feature = "rocq")]
    {
        let mut out = String::new();
        let options = crate::export::rocq::Options::default();
        let written = crate::export::rocq::write(&arrow, &options, &mut out, |_| false);
        assert!(written.is_err());
    }
    // A run whose conclusion lacks the principal of the rule above is
    // drawn, not left out.
    let lacking = r#"{"version":8,"logic":"classical","sequent":{"formulas":[{"V":0},{"V":1},"⊤"],"atoms":["a","b"],"left":[],"right":[1,1]},"inferences":[{"left":[],"right":[0,2],"rule":"⊤R","principal":["right",1]},{"left":[],"right":[1],"rule":"WR","principal":["right",0],"premises":[0]},{"left":[],"right":[1,1],"rule":"WR","principal":["right",0],"premises":[1]}]}"#;
    let lacking = derivation_of(lacking).unwrap();
    assert_eq!(lacking.shown(Structural::Admissible).inferences().len(), 2);
}

/// Applies the quantifier rule `rule` to the formula printed as `text`
/// on `side`, with the witness read from `witness` if any, and returns
/// the goals opened.
fn quantifier(
    state: &mut Interactive,
    goal: GoalId,
    (side, text): (Side, &str),
    rule: Rule,
    witness: Option<&str>,
) -> Result<Vec<GoalId>, StepError> {
    let mut step = Step::new(side, at(state, goal, side, text), rule);
    if let Some(witness) = witness {
        step = step.with_witness(state.witness(goal, witness)?);
    }
    state.apply(goal, &step)
}

/// The quantifier rules take a typed witness, held to the goal's
/// scope, and name a fresh eigenvariable, the same one again after an
/// undo; the finished proof checks and its JSON reads back.
#[test]
fn the_quantifier_rules_take_witnesses_and_eigenvariables() {
    let (mut s, g) = start(
        "forall x. p(x) -> q(x), p(c) |- exists y. q(y)",
        Logic::Intuitionistic,
    );
    let [g] = quantifier(&mut s, g, (Right, "∃y. q(y)"), Rule::ExistsRight, Some("c")).unwrap()[..]
    else {
        panic!()
    };
    let [g] = quantifier(
        &mut s,
        g,
        (Left, "∀x. (p(x) → q(x))"),
        Rule::ForallLeft,
        Some("c"),
    )
    .unwrap()[..] else {
        panic!()
    };
    let [first, second] = step(&mut s, g, (Left, "p(c) → q(c)"), Rule::ImpliesLeft, None)[..]
    else {
        panic!()
    };
    axiom(&mut s, first, "p(c)");
    axiom(&mut s, second, "q(c)");
    s.finished(&Limits::default(), |_| false).unwrap();
    for (witness, why) in [
        ("d", "not a symbol"),
        ("p", "predicate"),
        ("c(c)", "arguments"),
        ("", "no term"),
    ] {
        let (mut s, g) = start("forall x. p(x) |- p(c)", Logic::Classical);
        let error = quantifier(
            &mut s,
            g,
            (Left, "∀x. p(x)"),
            Rule::ForallLeft,
            Some(witness),
        )
        .unwrap_err();
        assert!(error.to_string().contains(why), "{witness}: {error}");
    }
    // `∃L` names `a`, the first name no symbol has; undone and taken
    // The right side's `∃y. p(y)` is the hypotheses' formula, which
    // prints with the name written first.
    // again, it names `a` again. The other branch cannot use it.
    let (mut s, g) = start(
        "(exists x. p(x)) \\/ (exists x. p(x)) |- exists y. p(y)",
        Logic::Classical,
    );
    let [g, other] = step(
        &mut s,
        g,
        (Left, "(∃x. p(x)) ∨ (∃x. p(x))"),
        Rule::OrLeft,
        None,
    )[..] else {
        panic!()
    };
    let made = |s: &mut Interactive| {
        let [h] = quantifier(s, g, (Left, "∃x. p(x)"), Rule::ExistsLeft, None).unwrap()[..]
        else {
            panic!()
        };
        at(s, h, Left, "p(a)");
        h
    };
    made(&mut s);
    s.undo();
    let h = made(&mut s);
    let error = quantifier(
        &mut s,
        other,
        (Right, "∃x. p(x)"),
        Rule::ExistsRight,
        Some("a"),
    )
    .unwrap_err();
    assert!(
        error.to_string().contains("not in the goal's scope"),
        "{error}"
    );
    let [h] = quantifier(&mut s, h, (Right, "∃x. p(x)"), Rule::ExistsRight, Some("a")).unwrap()[..]
    else {
        panic!()
    };
    axiom(&mut s, h, "p(a)");
    let [o] = quantifier(&mut s, other, (Left, "∃x. p(x)"), Rule::ExistsLeft, None).unwrap()[..]
    else {
        panic!()
    };
    let [o] = quantifier(&mut s, o, (Right, "∃x. p(x)"), Rule::ExistsRight, Some("b")).unwrap()[..]
    else {
        panic!()
    };
    axiom(&mut s, o, "p(b)");
    let derivation = s.finished(&Limits::default(), |_| false).unwrap();
    assert!(
        derivation
            .inferences()
            .iter()
            .any(|i| i.rule() == Rule::ExistsLeft)
    );
    let (mut open, g) = start("forall x. p(x) |- p(c)", Logic::Classical);
    let [g] = quantifier(&mut open, g, (Left, "∀x. p(x)"), Rule::ForallLeft, None).unwrap()[..]
    else {
        panic!()
    };
    at(&open, g, Left, "p($1)");
    #[cfg(feature = "serialize")]
    {
        let json = serde_json::to_string(&s).unwrap();
        let mut read: Interactive = serde_json::from_str(&json).unwrap();
        assert_eq!(serde_json::to_string(&read).unwrap(), json);
        read.undo();
        read.undo();
        assert!(!read.is_complete());
    }
}

/// `∀R` and `∃L` are invertible, `∀L` and `∃R` are not.
#[test]
fn the_eigenvariable_rules_are_invertible() {
    let (mut s, _) = start(
        "exists x. p(x) |- forall y. (q -> q)",
        Logic::Intuitionistic,
    );
    s.invert_all(&Limits::default());
    assert!(s.is_complete(), "{:?}", s.goals().collect::<Vec<_>>());
    s.finished(&Limits::default(), |_| false).unwrap();
    let (mut s, _) = start("forall x. p(x) |- exists y. p(y)", Logic::Classical);
    s.invert_all(&Limits::default());
    assert_eq!(s.steps(), 0);
}

/// An instance holding an equivalence the arena lacked gets its
/// implications, so the rules on it are offered and taken.
#[test]
fn an_instance_s_equivalence_has_its_implications() {
    let (mut s, g) = start("forall x. (p(x) <-> q(x)), p(c) |- q(c)", Logic::Classical);
    let [g] = quantifier(
        &mut s,
        g,
        (Left, "∀x. (p(x) ↔ q(x))"),
        Rule::ForallLeft,
        Some("c"),
    )
    .unwrap()[..] else {
        panic!()
    };
    let at = at(&s, g, Left, "p(c) ↔ q(c)");
    assert!(
        s.rules(g, Left, at)
            .unwrap()
            .iter()
            .any(|r| r.rule == Rule::IffLeft)
    );
    let (mut s, _) = start("|- forall x. (p(x) <-> p(x))", Logic::Intuitionistic);
    s.invert_all(&Limits::default());
    assert!(s.is_complete());
}

/// A session document is held to what a step makes: closed formulas,
/// no open witness; and the drawn inferences' goals follow the
/// admissible view, which keeps a weakening that brings an
/// eigenvariable into its rule's conclusion.
#[cfg(feature = "serialize")]
#[test]
fn a_first_order_session_document_is_held_to_its_steps() {
    let session = |fo: &str| {
        format!(
            r#"{{"version":25,"logic":"classical","sequent":{{"formulas":[{{"V":0}}],"atoms":[{{"P":[0,[0]]}}],"symbols":[["p",1]],"fo":[{fo}],"left":[0],"right":[0]}},"inferences":[{{"left":[0],"right":[0]}}],"history":[]}}"#
        )
    };
    assert!(serde_json::from_str::<Interactive>(&session(r#"{"B":0}"#)).is_err());
    assert!(serde_json::from_str::<Interactive>(&session(r#"{"M":0}"#)).is_err());
    let view = r#"{"version":25,"logic":"classical","sequent":{"formulas":[{"V":0},{"∀":0},{"V":1},{"V":2}],"atoms":[{"P":[0,[0]]},{"P":[1,[1]]},{"P":[0,[1]]}],"symbols":[["p",1],["q",1]],"fo":[{"B":0},{"E":0}],"binders":["x"],"left":[2,1],"right":[1]},"inferences":[{"left":[2,1],"right":[1],"rule":"WL","principal":["left",0],"premises":[1]},{"left":[1],"right":[1],"rule":"∀R","principal":["right",0],"premises":[2],"term":1},{"left":[1],"right":[3],"rule":"∀L","principal":["left",0],"premises":[3],"term":1},{"left":[3,1],"right":[3],"rule":"WL","principal":["left",1],"premises":[4]},{"left":[3],"right":[3],"rule":"ax"}],"history":[0,1,2,3]}"#;
    let state: Interactive = serde_json::from_str(view).unwrap();
    let shown = state
        .derivation_within(&ViewOptions::default(), &Limits::default(), |_| false)
        .unwrap();
    assert_eq!(
        state.derivation_ids(Structural::Admissible).len(),
        shown.inferences().len()
    );
    // An eigenvariable where no atom is reads back with its term.
    let (mut s, g) = start("|- forall x. true", Logic::Classical);
    let [g] = quantifier(&mut s, g, (Right, "∀x. ⊤"), Rule::ForallRight, None).unwrap()[..]
    else {
        panic!()
    };
    step(&mut s, g, (Right, "⊤"), Rule::TrueRight, None);
    let json = serde_json::to_string(&s).unwrap();
    assert_eq!(
        serde_json::to_string(&serde_json::from_str::<Interactive>(&json).unwrap()).unwrap(),
        json
    );
    let derivation = s.finished(&Limits::default(), |_| false).unwrap();
    let read: Derivation =
        serde_json::from_str(&serde_json::to_string(&derivation).unwrap()).unwrap();
    read.check(&Limits::default(), |_| false).unwrap();
}

/// Applies the axiom to the formula printed as `text` on `side`.
fn closes(
    state: &mut Interactive,
    goal: GoalId,
    (side, text): (Side, &str),
) -> Result<Vec<GoalId>, StepError> {
    let position = at(state, goal, side, text);
    state.apply(goal, &Step::new(side, position, Rule::Axiom))
}

/// `∀x. p(x) ⊢ ∃x. p(x)` with no constant proves in LK, LJ and LM: two
/// open witnesses, one bound to the other by the axiom and the other
/// a fresh eigenvariable in the finished derivation, which checks. The
/// binding shows in every inference, an undo takes it back, and a
/// session document reads back as itself before and after it.
#[test]
fn an_axiom_fills_the_open_witnesses() {
    for logic in [Logic::Classical, Logic::Intuitionistic, Logic::Minimal] {
        let (mut s, g) = start("forall x. p(x) |- exists x. p(x)", logic);
        let [g] = quantifier(&mut s, g, (Right, "∃x. p(x)"), Rule::ExistsRight, None).unwrap()[..]
        else {
            panic!()
        };
        let [g] = quantifier(&mut s, g, (Left, "∀x. p(x)"), Rule::ForallLeft, None).unwrap()[..]
        else {
            panic!()
        };
        assert!(
            s.rules(g, Left, at(&s, g, Left, "p($2)"))
                .unwrap()
                .iter()
                .any(|r| r.rule == Rule::Axiom)
        );
        #[cfg(feature = "serialize")]
        let before = serde_json::to_string(&s).unwrap();
        assert_eq!(closes(&mut s, g, (Left, "p($2)")).unwrap(), []);
        assert_eq!(s.bindings(), [("$2".to_owned(), "$1".to_owned())]);
        let derivation = s.finished(&Limits::default(), |_| false).unwrap();
        // The witness left open is an eigenvariable no rule introduces,
        // and the derivation's document holds no open witness.
        assert!(derivation.inferences().iter().all(|i| {
            i.term()
                .is_none_or(|t| derivation.formulas().is_eigenvariable(t))
        }));
        #[cfg(feature = "serialize")]
        {
            let json = serde_json::to_string(&derivation).unwrap();
            let read: Derivation = serde_json::from_str(&json).unwrap();
            read.check(&Limits::default(), |_| false).unwrap();
        }
        #[cfg(feature = "serialize")]
        {
            let after = serde_json::to_string(&s).unwrap();
            assert!(after.contains(r#""bindings":[[1,"#), "{after}");
            let mut read: Interactive = serde_json::from_str(&after).unwrap();
            assert_eq!(serde_json::to_string(&read).unwrap(), after);
            read.undo();
            assert_eq!(serde_json::to_string(&read).unwrap(), before);
            let read: Interactive = serde_json::from_str(&before).unwrap();
            assert_eq!(serde_json::to_string(&read).unwrap(), before);
        }
        s.undo();
        assert_eq!(s.bindings(), []);
        at(&s, g, Left, "p($2)");
        s.undo();
        s.undo();
        assert!(s.witnesses.is_none());
    }
}

/// An open witness takes no eigenvariable made after it: in
/// `∀x. ∃y. r(x, y) ⊢ ∃y. ∀x. r(x, y)` the axiom would have to give
/// `$1`, left open first, the eigenvariable of the later `∃L`.
#[test]
fn an_open_witness_takes_no_later_eigenvariable() {
    let (mut s, g) = start(
        "forall x. exists y. r(x, y) |- exists y. forall x. r(x, y)",
        Logic::Classical,
    );
    let [g] = quantifier(
        &mut s,
        g,
        (Right, "∃y. ∀x. r(x, y)"),
        Rule::ExistsRight,
        None,
    )
    .unwrap()[..] else {
        panic!()
    };
    let [g] = quantifier(&mut s, g, (Right, "∀x. r(x, $1)"), Rule::ForallRight, None).unwrap()[..]
    else {
        panic!()
    };
    let [g] = quantifier(&mut s, g, (Left, "∀x. ∃y. r(x, y)"), Rule::ForallLeft, None).unwrap()[..]
    else {
        panic!()
    };
    let [g] = quantifier(&mut s, g, (Left, "∃y. r($2, y)"), Rule::ExistsLeft, None).unwrap()[..]
    else {
        panic!()
    };
    let error = closes(&mut s, g, (Left, "r($2, b)")).unwrap_err();
    assert!(matches!(error, StepError::Binding { .. }), "{error}");
    assert!(error.to_string().contains("eigenvariable b"), "{error}");
    assert_eq!(s.bindings(), []);
    at(&s, g, Left, "r($2, b)");
}

/// A session document with open witnesses is read as its steps made it:
/// bindings that are not the axiom's, an axiom whose formulas do not
/// unify, and an open witness in the sequent are refused.
#[cfg(feature = "serialize")]
#[test]
fn a_session_document_is_held_to_its_bindings() {
    let (mut s, g) = start("forall x. p(x) |- exists x. p(x)", Logic::Classical);
    let [g] = quantifier(&mut s, g, (Right, "∃x. p(x)"), Rule::ExistsRight, None).unwrap()[..]
    else {
        panic!()
    };
    let [g] = quantifier(&mut s, g, (Left, "∀x. p(x)"), Rule::ForallLeft, None).unwrap()[..]
    else {
        panic!()
    };
    closes(&mut s, g, (Left, "p($2)")).unwrap();
    let json = serde_json::to_string(&s).unwrap();
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    let refused = |change: &dyn Fn(&mut serde_json::Value)| {
        let mut forged = value.clone();
        change(&mut forged);
        serde_json::from_value::<Interactive>(forged).is_err()
    };
    // No binding, or one the axiom does not make.
    assert!(refused(&|v| {
        v.as_object_mut().unwrap().remove("bindings");
    }));
    assert!(refused(&|v| v["bindings"][0][2] = 0.into()));
    // The axiom's formulas, made as `p($2) ⊢ p($1)`, made equal.
    let ax = value["inferences"]
        .as_array()
        .unwrap()
        .iter()
        .position(|i| i["rule"] == "ax")
        .unwrap();
    assert!(refused(&|v| {
        let left = v["inferences"][ax]["left"].clone();
        v["inferences"][ax]["right"] = left;
    }));
}

/// Applies `rule` at `position` of `side` with no witness or split and
/// returns the goals opened.
fn at_position(
    state: &mut Interactive,
    goal: GoalId,
    (side, position): (Side, usize),
    rule: Rule,
) -> Result<Vec<GoalId>, StepError> {
    state.apply(goal, &Step::new(side, position, rule))
}

/// Returns whether the state's JSON reads back as itself.
#[cfg(feature = "serialize")]
fn reads_back(state: &Interactive) -> bool {
    let json = serde_json::to_string(state).unwrap();
    let read: Interactive = serde_json::from_str(&json).unwrap_or_else(|e| panic!("{e}: {json}"));
    serde_json::to_string(&read).unwrap() == json
}

/// A binding that builds an equivalence the arena lacked builds its
/// implications too, which the rules on it and the checker look up:
/// the rules list it, the finished derivation checks, and a sound
/// binding through it is taken.
#[test]
fn a_binding_builds_an_equivalence_s_implications() {
    let one = |state: &mut Interactive, goal, at, rule| {
        let opened = at_position(state, goal, at, rule).unwrap();
        assert_eq!(opened.len(), 1, "{rule}");
        opened[0]
    };
    let (mut s, g) = start(
        "forall x. ((p(x) <-> q) /\\ s(x)) |- s(a) /\\ u",
        Logic::Classical,
    );
    let g = one(&mut s, g, (Left, 0), Rule::ForallLeft);
    let g = one(&mut s, g, (Left, 0), Rule::AndLeft);
    let [first, second] = at_position(&mut s, g, (Right, 0), Rule::AndRight).unwrap()[..] else {
        panic!()
    };
    closes(&mut s, first, (Left, "s($1)")).unwrap();
    assert!(
        s.rules(second, Left, 0)
            .unwrap()
            .iter()
            .any(|r| r.rule == Rule::IffLeft1)
    );
    one(&mut s, second, (Left, 0), Rule::IffLeft1);
    let (mut s, g) = start("forall x. (q <-> r(x)), q |- q", Logic::Classical);
    let g = one(&mut s, g, (Left, 0), Rule::ForallLeft);
    let g = one(&mut s, g, (Left, 0), Rule::IffLeft1);
    closes(&mut s, g, (Left, "q")).unwrap();
    s.finished(&Limits::default(), |_| false).unwrap();
    let (mut s, g) = start("forall x. (p(x) <-> q), p(a) |- q", Logic::Classical);
    let g = one(&mut s, g, (Left, 0), Rule::ForallLeft);
    let g = one(&mut s, g, (Left, 0), Rule::IffLeft1);
    let [first, second] = at_position(&mut s, g, (Left, 0), Rule::ImpliesLeft).unwrap()[..] else {
        panic!()
    };
    closes(&mut s, first, (Right, "p($1)")).unwrap();
    closes(&mut s, second, (Left, "q")).unwrap();
    s.finished(&Limits::default(), |_| false).unwrap();
    #[cfg(feature = "serialize")]
    assert!(reads_back(&s));
}

/// A session's own document reads back as itself where the axiom
/// follows a weakening the user took, and where the goal the axiom
/// closes was changed by an earlier binding.
#[cfg(feature = "serialize")]
#[test]
fn a_session_s_own_bindings_read_back() {
    let (mut s, g) = start("p(a), q |- exists x. p(x)", Logic::Intuitionistic);
    let [g] = at_position(&mut s, g, (Right, 0), Rule::ExistsRight).unwrap()[..] else {
        panic!()
    };
    let [g] = at_position(&mut s, g, (Left, 1), Rule::WeakenLeft).unwrap()[..] else {
        panic!()
    };
    closes(&mut s, g, (Left, "p(a)")).unwrap();
    assert!(reads_back(&s));
    let (mut s, g) = start(
        "r(c) |- exists x. (r(x) /\\ exists u. (q(u) -> q(g(x))))",
        Logic::Intuitionistic,
    );
    let [g] = at_position(&mut s, g, (Right, 0), Rule::ExistsRight).unwrap()[..] else {
        panic!()
    };
    let split = Step::new(Right, 0, Rule::AndRight).split(&[0], &[]);
    let [first, second] = s.apply(g, &split).unwrap()[..] else {
        panic!()
    };
    let [g] = at_position(&mut s, second, (Right, 0), Rule::ExistsRight).unwrap()[..] else {
        panic!()
    };
    let [g] = at_position(&mut s, g, (Right, 0), Rule::ImpliesRight).unwrap()[..] else {
        panic!()
    };
    closes(&mut s, first, (Left, "r(c)")).unwrap();
    closes(&mut s, g, (Left, "q($2)")).unwrap();
    s.finished(&Limits::default(), |_| false).unwrap();
    assert!(reads_back(&s));
}

/// The reader refuses an open witness that no step left open, or one
/// held by an inference no step closed, and does not panic on it.
#[cfg(feature = "serialize")]
#[test]
fn a_witness_no_step_left_open_is_refused() {
    let forged = r#"{"version":25,"logic":"intuitionistic","sequent":{"formulas":[{"V":0},{"V":1},{"∃":1},"⊥",{"V":2}],"atoms":[{"P":[1,[0]]},{"P":[1,[1]]},{"P":[1,[2]]}],"symbols":[["a",0],["p",1]],"fo":[{"F":[0,[]]},{"B":0},{"M":0}],"binders":["x"],"left":[0],"right":[2]},"inferences":[{"left":[0],"right":[2],"rule":"∃R","principal":["right",0],"premises":[1],"term":2},{"left":[0],"right":[4],"rule":"ax"}],"history":[1],"bindings":[[0,0,0]]}"#;
    assert!(serde_json::from_str::<Interactive>(forged).is_err());
    let (mut s, g) = start("p(a) |- exists x. p(x)", Logic::Intuitionistic);
    at_position(&mut s, g, (Right, 0), Rule::ExistsRight).unwrap();
    let mut value = serde_json::to_value(&s).unwrap();
    value["history"] = serde_json::json!([]);
    assert!(serde_json::from_value::<Interactive>(value).is_err());
    // An inference no step closed, which the reader takes, stays
    // readable once a witness is left open after it.
    let (mut s, g) = start(
        "exists x. forall y. s |- forall y. exists x. s",
        Logic::Intuitionistic,
    );
    let [g] = at_position(&mut s, g, (Right, 0), Rule::ForallRight).unwrap()[..] else {
        panic!()
    };
    let mut value = serde_json::to_value(&s).unwrap();
    value["history"] = serde_json::json!([]);
    let mut read: Interactive = serde_json::from_value(value).unwrap();
    at_position(&mut read, g, (Right, 0), Rule::ExistsRight).unwrap();
    assert!(reads_back(&read));
}

/// A binding whose term would unfold past the bound on terms is
/// refused, the state as it was.
#[test]
fn a_binding_is_held_to_the_bound_on_terms() {
    let sequent: Sequent = "forall x y. (p(x) /\\ (p(f(y, y)) -> q)) |- q"
        .parse()
        .unwrap();
    let limits = Limits {
        terms: Some(8),
        ..Limits::default()
    };
    let mut s = Interactive::within(&sequent, Logic::Classical, &limits).unwrap();
    let mut g = GoalId::new(0);
    for rule in [Rule::ForallLeft, Rule::ForallLeft, Rule::AndLeft] {
        g = at_position(&mut s, g, (Left, 0), rule).unwrap()[0];
    }
    let [first, _] = at_position(&mut s, g, (Left, 1), Rule::ImpliesLeft).unwrap()[..] else {
        panic!()
    };
    let error = closes(&mut s, first, (Left, "p($1)")).unwrap_err();
    assert!(
        matches!(
            error,
            StepError::Terms {
                terms: 11,
                limit: 8
            }
        ),
        "{error}"
    );
    assert_eq!(s.bindings(), []);
}

/// A witness an undone step made stays in the arena's atoms, numbered
/// past the live witnesses; the axiom's measure of the arena passes it
/// by.
#[test]
fn an_undone_witness_left_in_the_arena_is_passed_by() {
    let (mut s, g) = start(
        "forall x. forall y. r(x, y) |- forall y. r(a, y)",
        Logic::Classical,
    );
    let [g] = at_position(&mut s, g, (Left, 0), Rule::ForallLeft).unwrap()[..] else {
        panic!()
    };
    at_position(&mut s, g, (Left, 0), Rule::ForallLeft).unwrap();
    s.undo();
    s.rules(g, Left, 0).unwrap();
    assert!(closes(&mut s, g, (Left, "∀y. r($1, y)")).is_ok());
}

/// A session held to a bound on terms writes documents its reader
/// takes under the same bound, the steps after a filling growing the
/// arena.
#[cfg(feature = "serialize")]
#[test]
fn a_session_under_a_bound_on_terms_reads_back_under_it() {
    let sequent: Sequent = "forall x. p(x) |- (exists x. p(x)) /\\ (exists x. p(x))"
        .parse()
        .unwrap();
    let limits = Limits {
        terms: Some(5),
        ..Limits::default()
    };
    let mut s = Interactive::within(&sequent, Logic::Classical, &limits).unwrap();
    let [first, second] =
        at_position(&mut s, GoalId::new(0), (Right, 0), Rule::AndRight).unwrap()[..]
    else {
        panic!()
    };
    let [g] = at_position(&mut s, first, (Right, 0), Rule::ExistsRight).unwrap()[..] else {
        panic!()
    };
    let [g] = at_position(&mut s, g, (Left, 0), Rule::ForallLeft).unwrap()[..] else {
        panic!()
    };
    closes(&mut s, g, (Left, "p($2)")).unwrap();
    let [g] = at_position(&mut s, second, (Right, 0), Rule::ExistsRight).unwrap()[..] else {
        panic!()
    };
    at_position(&mut s, g, (Left, 0), Rule::ForallLeft).unwrap();
    let json = serde_json::to_string(&s).unwrap();
    let mut reader = serde_json::Deserializer::from_str(&json);
    let read: Interactive = crate::wire::upgrade(&mut reader, &limits).unwrap();
    assert_eq!(serde_json::to_string(&read).unwrap(), json);
}

/// Returns where the state is not what a walk over every inference
/// would make it: an inference that does not show the bindings in force
/// over what it was made as, a form as made kept where nothing changed
/// it, or a closed inference the checker's rules refuse.
fn walked_differs(state: &Interactive) -> Option<String> {
    let mut formulas = state.formulas.clone();
    let w = state.witnesses.as_deref();
    let bindings = w.map_or_else(Vec::new, |w| {
        w.unifier.metas().iter().map(|m| m.bound).collect()
    });
    let mut resolution = Resolution::new(bindings);
    for (i, inference) in state.inferences.iter().enumerate() {
        let raw = w.and_then(|w| w.raw[i].as_ref());
        let mut made = inference.clone();
        if let Some(raw) = raw {
            made.left.clone_from(&raw.left);
            made.right.clone_from(&raw.right);
            made.term = raw.term;
        }
        let expected = shown(&mut resolution, &mut formulas, &made).unwrap();
        if expected != *inference {
            return Some(format!("inference {i} does not show the bindings"));
        }
        if raw.is_some_and(|raw| raw.shows(inference)) {
            return Some(format!("inference {i} keeps a form as made that shows"));
        }
        if inference.rule != Rule::Open {
            let premises: Vec<&Inference> = inference
                .premises
                .iter()
                .map(|p| &state.inferences[p.index()])
                .collect();
            if let Err(why) = check_inference(&state.formulas, state.logic, inference, &premises) {
                return Some(format!("inference {i}: {why}"));
            }
        }
    }
    None
}

/// Random first-order sessions with open witnesses, fillings, undos and
/// documents read back show, after every step, what a walk over every
/// inference would show, and every closed inference checks.
#[test]
fn a_binding_shown_through_its_holders_is_the_walk_s() {
    const SEQUENTS: [&str; 8] = [
        "forall x. p(x) |- exists x. p(x)",
        "forall x. exists y. r(x, y) |- exists y. forall x. r(x, y)",
        "forall x. (p(x) -> q(x)), forall x. p(x) |- forall x. q(x)",
        "exists x. (p(x) /\\ q(x)) |- (exists x. p(x)) /\\ (exists x. q(x))",
        "forall x. forall y. r(x, y) |- forall y. forall x. r(x, y)",
        "forall x. p(f(x)) |- (exists x. p(x)) /\\ (exists x. p(x))",
        "forall x. (p(x) <-> q), p(a) |- q",
        "r(c) |- exists x. (r(x) /\\ exists u. (q(u) -> q(g(x))))",
    ];
    let mut seed = 0x2545_f491_4f6c_dd1du64;
    let mut next = |n: usize| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed % n as u64) as usize
    };
    let mut bound = 0;
    for round in 0..90 {
        let logic = [Logic::Classical, Logic::Intuitionistic, Logic::Minimal][round % 3];
        let (mut s, _) = start(SEQUENTS[round % SEQUENTS.len()], logic);
        for _ in 0..40 {
            let goals: Vec<GoalId> = s.goals().collect();
            match next(10) {
                0 | 1 => {
                    s.undo();
                }
                #[cfg(feature = "serialize")]
                2 => {
                    let json = serde_json::to_string(&s).unwrap();
                    s = serde_json::from_str(&json).unwrap_or_else(|e| panic!("{e}: {json}"));
                }
                _ if !goals.is_empty() => {
                    let g = goals[next(goals.len())];
                    let (left, right) = s.goal(g).unwrap();
                    let (side, len) = if next(2) == 0 {
                        (Left, left.len())
                    } else {
                        (Right, right.len())
                    };
                    if len == 0 {
                        continue;
                    }
                    let position = next(len);
                    let rules = s.rules(g, side, position).unwrap();
                    if rules.is_empty() {
                        continue;
                    }
                    let rule = rules[next(rules.len())].rule;
                    let before = s.bindings().len();
                    if s.apply(g, &Step::new(side, position, rule)).is_ok()
                        && s.bindings().len() > before
                    {
                        bound += 1;
                    }
                }
                _ => {}
            }
            if let Some(why) = walked_differs(&s) {
                panic!("round {round}: {why}");
            }
        }
    }
    assert!(bound > 20, "only {bound} fillings");
}

/// Each text that is no witness is refused for its first fault, a
/// bracket after a whole term as a missing comma.
#[test]
fn a_witness_text_is_refused_for_its_first_fault() {
    let sequent: Sequent = "forall x. p(x) |- q(f(a, b))".parse().unwrap();
    let mut state = Interactive::new(&sequent, Logic::Classical).unwrap();
    let name = |error: StepError| {
        let StepError::Witness { fault } = error else {
            panic!("not a witness's fault: {error}");
        };
        format!("{fault:?}")
            .split([' ', '{'])
            .next()
            .unwrap_or_default()
            .to_owned()
    };
    for (text, fault) in [
        ("", "Empty"),
        ("(", "Bracket"),
        (")", "Missing"),
        ("a b", "Comma"),
        ("a)", "Unopened"),
        ("f(a, b", "Unclosed"),
        ("f(a, b)(a)", "Comma"),
        ("g", "UnknownSymbol"),
        ("p", "Predicate"),
        ("f(a)", "Arity"),
        ("f", "Arity"),
        ("a(b)", "Arity"),
    ] {
        let error = state.witness(GoalId::new(0), text).unwrap_err();
        assert_eq!(name(error), fault, "{text}");
    }
    // Several faults: an applied name is resolved at its closing
    // bracket, so the bracket left open is the first.
    for (text, fault) in [
        ("g(a", "Unclosed"),
        ("p(a", "Unclosed"),
        ("g(h)", r#"UnknownSymbol { name: "h" }"#),
    ] {
        let error = state.witness(GoalId::new(0), text).unwrap_err();
        let StepError::Witness { fault: found } = error else {
            panic!("not a witness's fault: {error}");
        };
        assert!(format!("{found:?}").starts_with(fault), "{text}: {found:?}");
    }
    assert!(state.witness(GoalId::new(0), "f(a, f(b, a))").is_ok());
}

/// A typed witness and every instance are held to the bound on terms
/// by the measure the reader takes, so a session under a small bound
/// writes documents its reader takes under it.
#[cfg(feature = "serialize")]
#[test]
fn an_instance_is_held_to_the_bound_on_terms() {
    let within = |text: &str, terms| {
        let sequent: Sequent = text.parse().unwrap();
        let limits = Limits {
            terms: Some(terms),
            ..Limits::default()
        };
        (
            Interactive::within(&sequent, Logic::Classical, &limits).unwrap(),
            limits,
        )
    };
    let (mut s, _) = within("forall x. p(x, x) |- q", 4);
    let error = s.witness(GoalId::new(0), "f(f(f(f(a))))").unwrap_err();
    assert!(
        matches!(
            &error,
            StepError::Witness { fault }
                if matches!(**fault, WitnessFault::Symbols { symbols: 5, .. })
        ),
        "{error}"
    );
    // A sequent past the bound starts no proof.
    let sequent: Sequent = "p(f(f(f(f(c))))) |- q".parse().unwrap();
    let limits = Limits {
        terms: Some(3),
        ..Limits::default()
    };
    assert!(matches!(
        Interactive::within(&sequent, Logic::Classical, &limits),
        Err(Error::Refused(Refusal::Terms { terms: 5, limit: 3 }))
    ));
    let (mut s, _) = within("forall x. p(x, x), r(f(a)) |- q", 5);
    let witness = s.witness(GoalId::new(0), "f(a)").unwrap();
    let step = Step::new(Left, 0, Rule::ForallLeft).with_witness(witness);
    let error = s.apply(GoalId::new(0), &step).unwrap_err();
    assert!(
        matches!(error, StepError::Terms { limit: 5, .. }),
        "{error}"
    );
    // The review's session: whatever steps the bound leaves, the
    // document reads back under it.
    for terms in 3..8 {
        let (mut s, limits) = within(
            "forall x. p(x) |- (exists x. p(x)) /\\ (exists x. p(x))",
            terms,
        );
        let steps = [
            (0, Right, Rule::AndRight),
            (1, Right, Rule::ExistsRight),
            (3, Left, Rule::ForallLeft),
            (2, Right, Rule::ExistsRight),
            (5, Left, Rule::ForallLeft),
        ];
        for (goal, side, rule) in steps {
            let _ = s.apply(GoalId::new(goal), &Step::new(side, 0, rule));
        }
        let json = serde_json::to_string(&s).unwrap();
        let mut reader = serde_json::Deserializer::from_str(&json);
        let read: Result<Interactive, _> = crate::wire::upgrade(&mut reader, &limits);
        assert!(read.is_ok(), "terms {terms}: {:?}", read.err());
    }
}

/// A first-order goal is closed by the search, each inference of the
/// derivation checked as it enters, in LK and LJ, or refuted by a
/// finite structure. An eigenvariable of the goal is a constant there,
/// named as the proof prints it, and the derivation's own
/// eigenvariables come after every one the proof holds.
#[test]
fn the_search_closes_or_refutes_a_first_order_goal() {
    let options = search::Options::default();
    let limits = Limits::default();
    for logic in [Logic::Classical, Logic::Intuitionistic] {
        let (mut s, g) = start(
            "forall x. (p(x) -> q(x)), exists x. p(x) |- exists y. q(y)",
            logic,
        );
        let closed = s.close(g, None, &options, &limits, |_| false).unwrap();
        assert!(
            matches!(
                closed.outcome.verdict,
                Verdict::Valid(Evidence::Derivation(_))
            ),
            "{logic:?}: {:?}",
            closed.outcome.verdict
        );
        assert!(s.is_complete(), "{logic:?}");
        s.finished(&limits, |_| false).unwrap();
        #[cfg(feature = "serialize")]
        assert!(reads_back(&s), "{logic:?}");
    }
    // The goal holds the eigenvariable `a`; the derivation's `∀R`
    // names another.
    let (mut s, g) = start(
        "|- forall x. (p(x) -> forall y. (q(y) -> p(x) /\\ q(y)))",
        Logic::Intuitionistic,
    );
    let [g] = at_position(&mut s, g, (Right, 0), Rule::ForallRight).unwrap()[..] else {
        panic!()
    };
    let closed = s.close(g, None, &options, &limits, |_| false).unwrap();
    assert!(matches!(closed.outcome.verdict, Verdict::Valid(_)));
    assert!(s.is_complete());
    s.finished(&limits, |_| false).unwrap();
    // `⊢ p(a)` is refuted by a structure whose constant `a` is the
    // eigenvariable.
    let (mut s, g) = start("|- forall x. p(x)", Logic::Classical);
    let [g] = at_position(&mut s, g, (Right, 0), Rule::ForallRight).unwrap()[..] else {
        panic!()
    };
    let closed = s.close(g, None, &options, &limits, |_| false).unwrap();
    assert!(matches!(closed.outcome.verdict, Verdict::NotValid));
    let structure = closed.outcome.structure.expect("a finite structure");
    assert_eq!(structure.sequent.to_string(), "⊢ p(a)");
    assert_eq!(s.goals().collect::<Vec<_>>(), [g]);
    // With `=` and without its axioms the goal is valid with identity,
    // and its derivation, which holds the axioms, is not grafted.
    let (mut s, g) = start("a = b, p(a) |- p(b)", Logic::Classical);
    let closed = s.close(g, None, &options, &limits, |_| false).unwrap();
    assert!(matches!(closed.outcome.verdict, Verdict::Valid(_)));
    assert_eq!((closed.grafted, s.steps()), (Ok(()), 0));
    assert_eq!(
        s.hint(g, None, &options, &limits, |_| false).unwrap().steps,
        []
    );
}

/// A graft that would take the proof's arena past the bound on terms
/// is refused, the goal left open, so the session's document still
/// reads back under the bound: the search held only the goal's own
/// arena to it.
#[test]
fn a_graft_is_held_to_the_bound_on_terms() {
    let sequent: Sequent = "|- (exists y. (q(y) -> q(y))) /\\ (forall x. (p(x) -> p(x)))"
        .parse()
        .unwrap();
    let limits = Limits {
        terms: Some(2),
        ..Limits::default()
    };
    let mut s = Interactive::within(&sequent, Logic::Classical, &limits).unwrap();
    let g = s.goals().next().unwrap();
    let opened = at_position(&mut s, g, (Right, 0), Rule::AndRight).unwrap();
    let goal = opened[1];
    let options = search::Options::default();
    let closed = s.close(goal, None, &options, &limits, |_| false).unwrap();
    assert!(matches!(closed.outcome.verdict, Verdict::Valid(_)));
    assert!(
        matches!(closed.grafted, Err(Refusal::Terms { limit: 2, .. })),
        "{:?}",
        closed.grafted
    );
    assert!(s.goals().any(|open| open == goal));
    assert!(arena_terms(s.formulas.first_order()) <= 2);
}

/// A derivation's own symbol, Herbrand's fresh constant `c1`, is
/// renamed apart from the proof's names when it is grafted: here `c1`
/// is an atom of another branch, a predicate, which the constant would
/// meet in another role.
#[cfg(feature = "sat")]
#[test]
fn a_derivation_s_own_symbols_are_renamed_apart() {
    let limits = Limits::default();
    let (mut s, g) = start(
        "|- (c1 -> c1) /\\ ((forall x. p(x)) -> exists x. p(x))",
        Logic::Classical,
    );
    let opened = at_position(&mut s, g, (Right, 0), Rule::AndRight).unwrap();
    // The second goal in an arena of its own, as `close` makes it, and
    // the derivation Herbrand's theorem builds of it.
    let inference = s.open(opened[1]).unwrap().clone();
    let roots: Vec<NodeId> = inference
        .left
        .iter()
        .chain(&inference.right)
        .copied()
        .collect();
    let mut formulas = Formulas::default();
    let copied = transfer::copy(
        &mut formulas,
        &s.formulas,
        Some(&roots),
        &Parameters::Constants,
    )
    .unwrap();
    let sequent = Sequent {
        left: inference.left.iter().map(|&n| copied.node(n)).collect(),
        right: inference.right.iter().map(|&n| copied.node(n)).collect(),
        formulas,
    };
    let options = super::super::herbrand::Options::default();
    let found = super::super::herbrand::herbrand(&sequent, &options, &limits, |_| false).unwrap();
    let Verdict::Valid(Evidence::Derivation(derivation)) = found.verdict else {
        panic!("{:?}", found.verdict)
    };
    let symbols = &derivation.formulas().first_order().symbols;
    assert!(
        symbols
            .iter()
            .any(|(name, arity)| name == "c1" && *arity == 0)
    );
    let carry = Carry {
        constants: copied.constants,
        names: copied.names,
    };
    s.graft(opened[1], &derivation, Some(&carry), &limits)
        .unwrap();
    let closed = s.close(
        opened[0],
        None,
        &search::Options::default(),
        &limits,
        |_| false,
    );
    assert!(matches!(closed.unwrap().outcome.verdict, Verdict::Valid(_)));
    assert!(s.is_complete());
    s.finished(&limits, |_| false).unwrap();
}

/// Following the hints proves a first-order sequent: a witness the
/// goal holds comes with its step.
#[test]
fn following_first_order_hints_proves_a_sequent() {
    let options = search::Options::default();
    let limits = Limits::default();
    for (text, logic) in [
        (
            "forall x. (p(x) -> q(x)), exists x. p(x) |- exists y. q(y)",
            Logic::Intuitionistic,
        ),
        (
            "forall x. (p(x) -> q(x)), p(c), r |- q(c) /\\ r",
            Logic::Classical,
        ),
    ] {
        let (mut s, _) = start(text, logic);
        for _ in 0..50 {
            let Some(goal) = s.goals().next() else { break };
            let hint = s.hint(goal, None, &options, &limits, |_| false).unwrap();
            assert!(!hint.steps.is_empty(), "{text}: {:?}", hint.outcome.verdict);
            let mut goal = goal;
            for step in &hint.steps {
                if matches!(step.rule, Rule::ForallLeft | Rule::ExistsRight) {
                    assert!(step.witness.is_some(), "{text}");
                }
                let opened = s.apply(goal, step).unwrap();
                if let [next, ..] = opened[..] {
                    goal = next;
                }
            }
        }
        assert!(s.is_complete(), "{text}");
        s.finished(&limits, |_| false).unwrap();
    }
}

/// A goal that holds an open witness is not refuted, its countermodel
/// being one of the witness as a constant; one that holds it and is
/// valid whatever it becomes is closed, and the session document
/// reads back with the graft beside the witness.
#[cfg(feature = "serialize")]
#[test]
fn a_goal_with_an_open_witness_is_closed_but_never_refuted() {
    let options = search::Options::default();
    let limits = Limits::default();
    let (mut s, g) = start("|- exists x. p(x)", Logic::Classical);
    let [g] = at_position(&mut s, g, (Right, 0), Rule::ExistsRight).unwrap()[..] else {
        panic!()
    };
    let closed = s.close(g, None, &options, &limits, |_| false).unwrap();
    assert!(matches!(
        closed.outcome.verdict,
        Verdict::Unknown(search::Reason::Incomplete)
    ));
    assert!(closed.outcome.structure.is_none());
    let (mut s, g) = start("|- exists x. (p(x) -> p(x))", Logic::Classical);
    let [g] = at_position(&mut s, g, (Right, 0), Rule::ExistsRight).unwrap()[..] else {
        panic!()
    };
    let closed = s.close(g, None, &options, &limits, |_| false).unwrap();
    assert!(matches!(closed.outcome.verdict, Verdict::Valid(_)));
    assert!(s.is_complete());
    assert!(reads_back(&s));
    s.finished(&limits, |_| false).unwrap();
}

/// A valid first-order sequent whose evidence is grounding's clausal
/// proof gets the derivation of LK that Herbrand's theorem builds,
/// checked, which concludes the sequent.
#[cfg(feature = "sat")]
#[test]
fn a_clausal_proof_becomes_a_derivation() {
    let sequent: Sequent = "forall x. (p(x) -> q(x)), p(c) |- q(c)".parse().unwrap();
    let limits = Limits::default();
    let grounded = super::super::grounding::decide(&sequent, Logic::Classical, &limits, |_| false)
        .unwrap()
        .unwrap();
    let mut outcome = Outcome::from(grounded);
    assert!(matches!(
        outcome.verdict,
        Verdict::Valid(Evidence::Clausal(_))
    ));
    with_derivation(&mut outcome, &sequent, &limits, &mut |_| false).unwrap();
    let Verdict::Valid(Evidence::Derivation(found)) = &outcome.verdict else {
        panic!("{:?}", outcome.verdict)
    };
    found.check(&limits, |_| false).unwrap();
    let root = found.inference(found.root());
    let text = |list: &[NodeId]| {
        let mut texts: Vec<String> = list
            .iter()
            .map(|&k| found.formulas().formula(k).to_string())
            .collect();
        texts.sort();
        texts
    };
    let written = |list: &[NodeId]| {
        let mut texts: Vec<String> = list
            .iter()
            .map(|&k| sequent.formulas().formula(k).to_string())
            .collect();
        texts.sort();
        texts
    };
    assert_eq!(text(&root.left), written(sequent.left()));
    assert_eq!(text(&root.right), written(sequent.right()));
}
