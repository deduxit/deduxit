// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The invertible rules of LK and LJ, applied by the goal state itself.
//!
//! A rule is invertible when its premises are provable whenever its
//! conclusion is, so a session may take such steps without being asked.
//! In LK every logical rule with a shared context is; in LJ and minimal
//! logic `→L`, `¬L` and `∨R₁`, `∨R₂` are not (each drops the formula
//! right of `⊢` or chooses a disjunct).

use super::interactive::{Context, Interactive, Step};
use super::{Logic, Node, NodeId, Rule, Side};
use crate::limits::{Limits, Phase, Refusal};
use crate::linear::proofs::GoalId;
use crate::linear::proofs::interactive::Inverted;

/// Returns the rule invertible in `logic` that acts on a formula of the
/// kind of `node` on `side`, or `None`: the axiom, a weakening or a
/// contraction never, and no rule on an atom.
fn invertible(logic: Logic, side: Side, node: Node) -> Option<Rule> {
    use Side::{Left, Right};
    let classical = matches!(logic, Logic::Classical);
    match (side, node) {
        (_, Node::Atom(_)) => None,
        // A fresh eigenvariable loses nothing; a witness is a choice.
        (Right, Node::Forall(_)) => Some(Rule::ForallRight),
        (Left, Node::Exists(_)) => Some(Rule::ExistsLeft),
        (Left, Node::Forall(_)) | (Right, Node::Exists(_)) => None,
        (Left, Node::And(..)) => Some(Rule::AndLeft),
        (Left, Node::Or(..)) => Some(Rule::OrLeft),
        (Left, Node::Iff(..)) => Some(Rule::IffLeft),
        (Left, Node::True) => Some(Rule::TrueLeft),
        (Left, Node::False) => match logic {
            Logic::Minimal => None,
            Logic::Classical | Logic::Intuitionistic => Some(Rule::FalseLeft),
        },
        // LJ's first premise drops the formula right of `⊢`.
        (Left, Node::Implies(..)) => classical.then_some(Rule::ImpliesLeft),
        (Left, Node::Not(_)) => classical.then_some(Rule::NotLeft),
        (Right, Node::And(..)) => Some(Rule::AndRight),
        (Right, Node::Implies(..)) => Some(Rule::ImpliesRight),
        (Right, Node::Not(_)) => Some(Rule::NotRight),
        (Right, Node::Iff(..)) => Some(Rule::IffRight),
        (Right, Node::True) => Some(Rule::TrueRight),
        // LJ's `∨R` chooses a disjunct, and only LK has `⊥R`.
        (Right, Node::Or(..)) => classical.then_some(Rule::OrRight),
        (Right, Node::False) => classical.then_some(Rule::FalseRight),
    }
}

/// Returns whether a rule closes its goal.
const fn closes(rule: Rule) -> bool {
    matches!(rule, Rule::Axiom | Rule::FalseLeft | Rule::TrueRight)
}

impl Interactive {
    /// Returns whether a step on a sequent of this session (an open goal's
    /// sides, or an inference's conclusion) applies a rule invertible in
    /// the session's logic, with the context shared.
    ///
    /// The invertible rules are the axiom, and every rule of a connective
    /// with a shared context in LK (`⊥R` and `⊤L`, which drop a constant,
    /// among them); in LJ and minimal logic all of them but `→L`, `¬L`
    /// (whose first premise drops the formula right of `⊢`) and `∨R₁`,
    /// `∨R₂` (which choose a disjunct). `∧L₁`, `∧L₂`, `↔L₁` and `↔L₂`
    /// drop a part, and no structural rule is among them.
    pub fn is_invertible(&self, (left, right): (&[NodeId], &[NodeId]), step: &Step) -> bool {
        if step.context != Context::Shared {
            return false;
        }
        let side = match step.side {
            Side::Left => left,
            Side::Right => right,
        };
        let Some(&x) = side.get(step.position) else {
            return false;
        };
        if step.rule == Rule::Axiom {
            let other = match step.side {
                Side::Left => right,
                Side::Right => left,
            };
            return other.contains(&x);
        }
        invertible(self.logic(), step.side, self.formulas().node(x)) == Some(step.rule)
    }

    /// Applies the rules invertible in the session's logic
    /// ([`is_invertible`](Self::is_invertible)) to an open goal, and then
    /// to every goal those steps open, until none applies.
    ///
    /// On each goal it takes a rule that closes the goal (the axiom, `⊥`
    /// true, `⊤` false) first, and otherwise the rule of the first formula
    /// that has one, the hypotheses first. Each application is an ordinary
    /// step, which [`undo`](Self::undo) takes back one at a time. It
    /// always ends: every premise has a lighter sequent than its
    /// conclusion, an equivalence counted as heavier than its two
    /// implications. The bytes of the inferences it adds, by the
    /// estimate [`derivation_within`](Self::derivation_within) bounds a
    /// derivation by (an axiom's weakenings among them), are bounded by
    /// `limits.memory_bytes`: past the bound it stops after the step that
    /// passed it, and the steps it took stand ([`Inverted::finished`]).
    ///
    /// # Errors
    ///
    /// [`StepError::NoGoal`](super::StepError::NoGoal) for a goal that is
    /// not open.
    pub fn invert(&mut self, goal: GoalId, limits: &Limits) -> Result<Inverted, super::StepError> {
        self.goal(goal)?;
        Ok(self.invert_goals(vec![goal], limits))
    }

    /// Applies the invertible rules to every open goal in turn, as
    /// [`invert`](Self::invert) does to one, all within one memory bound.
    pub fn invert_all(&mut self, limits: &Limits) -> Inverted {
        let goals = self.goals().collect();
        self.invert_goals(goals, limits)
    }

    /// Applies the invertible rules to the goals, depth first, within the
    /// memory bound.
    fn invert_goals(&mut self, goals: Vec<GoalId>, limits: &Limits) -> Inverted {
        let mut todo: Vec<GoalId> = goals.into_iter().rev().collect();
        let (mut steps, mut open) = (0, Vec::new());
        // The bytes of the inferences the steps add, by the derivation's
        // own estimate, an axiom's weakenings included. The sum saturates,
        // but the bound stops the run long before that.
        let mut held = 0u64;
        let mut widths = self.widths();
        while let Some(goal) = todo.pop() {
            let Some(step) = self.invertible_step(goal) else {
                open.push(goal);
                continue;
            };
            let from = self.inference_count();
            let Ok(opened) = self.apply(goal, &step) else {
                // A step the state refuses (past the bound on a step's
                // weakenings) leaves the goal open.
                open.push(goal);
                continue;
            };
            steps += 1;
            // A quantifier step adds its instance to the arena.
            if widths.len() < self.formulas().len() {
                widths = self.widths();
            }
            held = held.saturating_add(self.added_bytes(&widths, goal, from));
            if let Some(limit) = limits.memory_bytes
                && held > limit
            {
                open.extend(opened);
                open.extend(todo.into_iter().rev());
                return Inverted {
                    steps,
                    open,
                    finished: Err(Refusal::Memory {
                        phase: Phase::View,
                        limit_bytes: limit,
                        needed_bytes: Some(held),
                    }),
                };
            }
            todo.extend(opened.into_iter().rev());
        }
        Inverted {
            steps,
            open,
            finished: Ok(()),
        }
    }

    /// Returns the invertible step to take on an open goal, or `None`: a
    /// step that closes the goal first, else the first formula's, the
    /// hypotheses before the formulas right of `⊢`.
    fn invertible_step(&self, goal: GoalId) -> Option<Step> {
        let (left, right) = self.goal(goal).ok()?;
        let logic = self.logic();
        let formulas = self.formulas();
        if let Some(position) = left.iter().position(|x| right.contains(x)) {
            return Some(Step::new(Side::Left, position, Rule::Axiom));
        }
        let sides = [(Side::Left, left), (Side::Right, right)];
        for closing in [true, false] {
            for (side, list) in sides {
                for (position, &x) in list.iter().enumerate() {
                    if let Some(rule) = invertible(logic, side, formulas.node(x))
                        && closes(rule) == closing
                    {
                        return Some(Step::new(side, position, rule));
                    }
                }
            }
        }
        None
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::super::{Interactive, Logic, Options, Rule, Sequent, Side, Step, Verdict, decide};
    use crate::Limits;
    use crate::linear::proofs::GoalId;

    /// Returns whether the open goals of a session are all valid in its
    /// logic, by the default decision.
    fn all_valid(state: &Interactive, logic: Logic) -> bool {
        state.goals().all(|goal| {
            let (left, right) = state.goal(goal).unwrap();
            let sequent =
                Sequent::new(state.formulas().clone(), left.to_vec(), right.to_vec()).unwrap();
            let options = Options::default().with_logic(logic);
            matches!(
                decide(&sequent, &options).unwrap().verdict,
                Verdict::Valid(_)
            )
        })
    }

    #[test]
    fn inverting_keeps_every_valid_goal_valid_and_ends() {
        let mut rng = crate::linear::search::generate::Rng::new(5407);
        for _ in 0..200 {
            let sequent = super::super::tests::generated_sequent(&mut rng);
            for logic in [Logic::Classical, Logic::Intuitionistic, Logic::Minimal] {
                // LJ takes one formula right of ⊢ at most.
                let Ok(mut state) = Interactive::new(&sequent, logic) else {
                    continue;
                };
                let valid = all_valid(&state, logic);
                let inverted = state.invert_all(&Limits::default());
                assert!(inverted.finished.is_ok());
                let mut left = inverted.open.clone();
                left.sort_unstable();
                assert_eq!(left, state.goals().collect::<Vec<_>>());
                if valid {
                    assert!(all_valid(&state, logic), "{sequent} in {logic} logic");
                }
                // Every step it took is one undo.
                for _ in 0..inverted.steps {
                    state.undo();
                }
                assert_eq!(state.steps(), 0);
            }
        }
    }

    #[test]
    fn lj_leaves_its_rules_that_are_not_invertible() {
        for (text, logic, open) in [
            ("a -> b |- a -> b", Logic::Intuitionistic, 0),
            ("a -> b, a |- b", Logic::Intuitionistic, 1),
            ("~a, a |- b", Logic::Minimal, 1),
            ("|- a \\/ b", Logic::Intuitionistic, 1),
            ("|- a \\/ ~a", Logic::Classical, 0),
            ("a -> b, a |- b", Logic::Classical, 0),
            ("false |- a", Logic::Minimal, 1),
            ("a /\\ b |- b /\\ a", Logic::Minimal, 0),
        ] {
            let sequent: Sequent = text.parse().unwrap();
            let mut state = Interactive::new(&sequent, logic).unwrap();
            let inverted = state.invert_all(&Limits::default());
            assert_eq!(inverted.open.len(), open, "{text} in {logic} logic");
        }
    }

    #[test]
    fn the_steps_taken_by_an_invertible_rule_are_listed_by_their_goal() {
        // An axiom with a context closes its goal through weakenings.
        let sequent: Sequent = "a, b |- a".parse().unwrap();
        let mut state = Interactive::new(&sequent, Logic::Classical).unwrap();
        state.invert_all(&Limits::default());
        assert_eq!(state.invertible_steps(), [GoalId::new(0)]);
        // A split context is the user's choice, not an invertible step.
        let sequent: Sequent = "c, d |- a /\\ b".parse().unwrap();
        let mut state = Interactive::new(&sequent, Logic::Classical).unwrap();
        let step = Step::new(Side::Right, 0, Rule::AndRight).split(&[0], &[]);
        state.apply(GoalId::new(0), &step).unwrap();
        assert_eq!(state.invertible_steps(), []);
    }

    #[test]
    fn an_axiom_s_weakenings_count_against_the_bound() {
        let atoms: Vec<String> = (0..100).map(|k| format!("a{k}")).collect();
        let text = format!("{} |- {}", atoms.join(", "), atoms.join(" /\\ "));
        let sequent: Sequent = text.parse().unwrap();
        let mut state = Interactive::new(&sequent, Logic::Classical).unwrap();
        let inverted = state.invert_all(&Limits::default().with_memory_bytes(Some(262_144)));
        assert!(inverted.finished.is_err());
    }

    #[test]
    fn inverting_stops_at_the_memory_bound() {
        let sequent: Sequent = "a /\\ b, c /\\ d |- (a \\/ b) /\\ (c \\/ d) /\\ (a \\/ d)"
            .parse()
            .unwrap();
        let mut state = Interactive::new(&sequent, Logic::Classical).unwrap();
        let inverted = state.invert_all(&Limits::default().with_memory_bytes(Some(64)));
        assert!(inverted.finished.is_err());
        assert!(inverted.steps > 0);
        let mut left = inverted.open.clone();
        left.sort_unstable();
        assert_eq!(left, state.goals().collect::<Vec<_>>());
    }
}
