// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Step-by-step proving in LK and LJ: a derivation of the ordinary
//! sequent built top-down by the rules of [`Rule`], with open goals as
//! leaves. A client names a goal, a formula in it by its side and
//! position, and a rule; the rule is validated against the formula and
//! the logic, every inference a step makes is checked by the rules the
//! derivation's checker applies, and the goals it leaves are returned.
//! The search closes a goal through the embeddings ([`decide()`]), and a
//! finished state is a [`Derivation`] that [`Derivation::check`]
//! validates, so this layer adds nothing to what is trusted.
//!
//! Needs the cargo feature `interactive` (on by default).

use super::derivation::{check_inference, same};
use super::translate::Extra;
use super::{Derivation, Formulas, Inference, Logic, Node, NodeId, Outcome, Rule, Sequent, Side};
use super::{Translation, Verdict, bound, decide};
use crate::Error;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::proofs::{GoalId, InfId, Size, Structural, ViewOptions};
use crate::linear::search;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Why a rule does not apply to a goal as asked. Positions are those of
/// the side of the goal's sequent named, as the client named them.
///
/// Needs the cargo feature `interactive` (on by default).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize),
    serde(tag = "kind", rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum StepError {
    /// No open goal has this id.
    #[non_exhaustive]
    NoGoal {
        /// The id asked for.
        goal: GoalId,
    },
    /// The side has no formula at the position; it has `len` formulas.
    #[non_exhaustive]
    NoFormula {
        /// The side asked for.
        side: Side,
        /// The position asked for.
        position: usize,
        /// How many formulas the side has.
        len: usize,
    },
    /// The rule does not act on the formula: its connective is another,
    /// or the rule is for the other side of `⊢`.
    #[non_exhaustive]
    Rule {
        /// The rule asked for.
        rule: Rule,
        /// The side of the formula.
        side: Side,
        /// The position of the formula.
        position: usize,
    },
    /// The rule is not one of the logic's: ex falso in minimal logic, and
    /// in intuitionistic and minimal logic a rule whose premise would have
    /// two formulas right of `⊢` or none, where every goal keeps one.
    #[non_exhaustive]
    Logic {
        /// The rule asked for.
        rule: Rule,
        /// The logic of the proof.
        logic: Logic,
    },
    /// The axiom needs the formula on the other side of `⊢` too.
    #[non_exhaustive]
    NoPartner {
        /// The side of the formula.
        side: Side,
        /// The position of the formula.
        position: usize,
    },
    /// In intuitionistic or minimal logic, a premise would have this many
    /// formulas right of `⊢` instead of one.
    #[non_exhaustive]
    Succedents {
        /// How many formulas would stand right of `⊢`.
        count: usize,
    },
    /// A position of the split is out of range, repeated, or the formula
    /// the rule acts on.
    #[non_exhaustive]
    Split {
        /// The side of the position.
        side: Side,
        /// The offending position.
        position: usize,
    },
    /// A split was given to a rule with fewer than two premises.
    #[non_exhaustive]
    NoSplit {
        /// The rule asked for.
        rule: Rule,
    },
    /// The run of weakenings below an axiom, or of contractions below a
    /// shared rule, would be estimated past the derivation's bound in
    /// bytes the state was started under.
    #[non_exhaustive]
    Size {
        /// The estimate of the run, in bytes.
        estimate_bytes: u64,
        /// The bound, `Limits::derivation_bytes`.
        limit_bytes: u64,
    },
}

/// Returns `left of ⊢` or `right of ⊢`.
const fn of(side: Side) -> &'static str {
    match side {
        Side::Left => "left of ⊢",
        Side::Right => "right of ⊢",
    }
}

/// Returns what a rule acts on, as a phrase: `a conjunction right of ⊢`.
const fn target(rule: Rule) -> &'static str {
    use Rule::*;
    match rule {
        Axiom => "a formula that stands on both sides of ⊢",
        WeakenLeft | ContractLeft => "any formula left of ⊢",
        WeakenRight | ContractRight => "any formula right of ⊢",
        AndLeft | AndLeft1 | AndLeft2 => "a conjunction left of ⊢",
        AndRight => "a conjunction right of ⊢",
        OrLeft => "a disjunction left of ⊢",
        OrRight | OrRight1 | OrRight2 => "a disjunction right of ⊢",
        ImpliesLeft => "an implication left of ⊢",
        ImpliesRight => "an implication right of ⊢",
        NotLeft => "a negation left of ⊢",
        NotRight => "a negation right of ⊢",
        IffLeft | IffLeft1 | IffLeft2 => "an equivalence left of ⊢",
        IffRight => "an equivalence right of ⊢",
        FalseLeft => "⊥ left of ⊢",
        FalseRight => "⊥ right of ⊢",
        TrueLeft => "⊤ left of ⊢",
        TrueRight => "⊤ right of ⊢",
        Cut => "no formula: its cut formula is none of the goal's",
        Open => "no formula: it is the mark of an open goal",
    }
}

impl Display for StepError {
    /// Writes the refusal as a phrase, naming rules, sides and positions.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            StepError::NoGoal { goal } => write!(f, "there is no open goal {}", goal.get()),
            StepError::NoFormula {
                side,
                position,
                len,
            } => write!(
                f,
                "the goal has no formula {position} {}: it has {}",
                of(*side),
                crate::errors::counted(*len, "formula", "formulas")
            ),
            StepError::Rule {
                rule,
                side,
                position,
            } => write!(
                f,
                "the rule {rule} does not act on formula {position} {}: it acts on {}",
                of(*side),
                target(*rule)
            ),
            StepError::Logic { rule, logic } => {
                let why = match (rule, logic) {
                    (Rule::FalseLeft, Logic::Minimal) => "minimal logic has no ex falso",
                    (Rule::ContractRight | Rule::OrRight, _) => {
                        "its premise would have two formulas right of ⊢, and every goal keeps one"
                    }
                    _ => "its premise would have no formula right of ⊢, and every goal keeps one",
                };
                write!(
                    f,
                    "the rule {rule} is not one of {} in {logic} logic: {why}",
                    logic.calculus()
                )
            }
            StepError::NoPartner { side, position } => write!(
                f,
                "the axiom needs formula {position} {} on the other side of ⊢ too",
                of(*side)
            ),
            StepError::Succedents { count } => write!(
                f,
                "a premise would have {} right of ⊢ instead of one, which every goal of LJ \
                 keeps: give the step a split that sends the formula right of ⊢ to the premise \
                 that takes none of its own",
                crate::errors::counted(*count, "formula", "formulas")
            ),
            StepError::Split { side, position } => write!(
                f,
                "position {position} {} of the split is out of range, repeated, or the formula \
                 the rule acts on",
                of(*side)
            ),
            StepError::Size {
                estimate_bytes,
                limit_bytes,
            } => write!(
                f,
                "the step's structural rules would be estimated at {estimate_bytes} bytes, past \
                 the bound of {limit_bytes} bytes the proof was started under"
            ),
            StepError::NoSplit { rule } => {
                write!(
                    f,
                    "the rule {rule} has fewer than two premises and takes no split"
                )
            }
        }
    }
}

impl std::error::Error for StepError {}

/// How a step of a rule with two premises gives them the goal's context,
/// the formulas the rule does not act on.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Context {
    /// Both premises take the whole context, as LJ's `∧R` does.
    #[default]
    Shared,
    /// The formulas at these positions go to the first premise, the rest
    /// to the second, as LJ's `→L` splits them.
    #[non_exhaustive]
    Split {
        /// The positions left of `⊢` going to the first premise.
        left: Vec<usize>,
        /// The positions right of `⊢` going to the first premise.
        right: Vec<usize>,
    },
}

/// A step of a proof in progress: the rule applied to the formula at a
/// position of a side of a goal, and for a rule with two premises how
/// they share the context.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    /// The side of the formula.
    pub side: Side,
    /// The position of the formula on its side.
    pub position: usize,
    /// The rule.
    pub rule: Rule,
    /// How two premises take the context.
    pub context: Context,
}

impl Step {
    /// Returns the step of `rule` on the formula at `position` of `side`,
    /// its premises sharing the context.
    pub const fn new(side: Side, position: usize, rule: Rule) -> Self {
        Self {
            side,
            position,
            rule,
            context: Context::Shared,
        }
    }

    /// Returns the step with the formulas at the positions given going to
    /// the first premise, left and right of `⊢`, and the rest to the
    /// second.
    #[must_use]
    pub fn split(self, left: &[usize], right: &[usize]) -> Self {
        Self {
            context: Context::Split {
                left: left.to_vec(),
                right: right.to_vec(),
            },
            ..self
        }
    }
}

/// A rule that can act on a formula of a goal, and what a step of it
/// needs beyond the formula.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Applicable {
    /// The rule.
    pub rule: Rule,
    /// What the step needs.
    pub needs: Needs,
}

/// What a step of a rule needs beyond the formula.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Needs {
    /// Nothing.
    Nothing,
    /// How its two premises take the context: shared unless a split is
    /// given.
    Context,
}

/// What [`Interactive::close`] did: the outcome of deciding the goal, and
/// whether its derivation, if any, was grafted onto the goal.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Closed {
    /// The outcome of deciding the goal as a sequent of its own; a
    /// derivation in it is the goal's, kept when its graft was refused.
    pub outcome: Outcome,
    /// `Ok` when the derivation was grafted or there was none; the
    /// refusal of the graft by its size otherwise, the goal then still
    /// open.
    pub grafted: Result<(), Refusal>,
}

/// What [`Interactive::hint`] found: the outcome of deciding the goal,
/// and the first steps of the derivation found, as steps that
/// [`Interactive::apply`] takes in turn.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Hint {
    /// The outcome of deciding the goal as a sequent of its own.
    pub outcome: Outcome,
    /// The first steps of the derivation found, each with its formula
    /// and, for two premises, how they took the context: the weakenings
    /// and contractions at its root and the first rule that is not one,
    /// which ends the list, the first step on the goal asked for and each
    /// other on the goal the step before it opened. Empty unless the goal
    /// is valid. All are read off one derivation, so that following them
    /// makes progress: a contraction alone, hinted again, could be
    /// answered by a weakening of the copy.
    pub steps: Vec<Step>,
}

/// What a rule makes of its principal formula in each premise: the
/// formulas that take its place on its side and those that join the
/// other side.
enum Shape {
    /// No premise.
    Leaf,
    /// One premise.
    Unary(Vec<NodeId>, Vec<NodeId>),
    /// Two premises, which take the context as a step says.
    Binary([(Vec<NodeId>, Vec<NodeId>); 2]),
}

/// A proof in progress of an ordinary sequent in a logic: the formulas,
/// the inferences of a derivation of LK or LJ made so far with the open
/// goals as leaves, and the steps taken. It starts from the sequent as
/// one open goal; rules are applied to a formula of a goal by side and
/// position, the search closes goals, [`undo`](Self::undo) retracts the
/// last step, and once no goal is open [`finished`](Self::finished)
/// gives the derivation, checked.
///
/// In intuitionistic and minimal logic every goal has exactly one formula
/// right of `⊢`: a sequent with none starts as `Γ ⊢ ⊥`, as it is decided,
/// and the rules whose premise would have none or two (`WR`, `CR`, `⊥R`
/// and `∨R` with both disjuncts) are not offered; `¬R` and `¬L` take the
/// form of `→R` and `→L` for `A → ⊥`.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::Limits;
/// use deduxit::ordinary::{Interactive, Logic, Rule, Sequent, Side, Step};
///
/// let sequent: Sequent = "a /\\ b |- b /\\ a".parse()?;
/// let mut state = Interactive::new(&sequent, Logic::Intuitionistic)?;
/// let root = state.goals().next().unwrap();
/// let [goal] = state.apply(root, &Step::new(Side::Left, 0, Rule::AndLeft))?[..] else {
///     unreachable!()
/// };
/// let goals = state.apply(goal, &Step::new(Side::Right, 0, Rule::AndRight))?;
/// for goal in goals {
///     state.apply(goal, &Step::new(Side::Right, 0, Rule::Axiom))?;
/// }
/// let derivation = state.finished(&Limits::default(), |_| false)?;
/// assert_eq!(derivation.inference(derivation.root()).rule(), Rule::AndLeft);
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// # JSON
///
/// With the feature `serialize` a proof in progress is `{"version": 8,
/// "logic": "intuitionistic", "sequent": {…}, "inferences": […],
/// "history": […]}` ([`wire`](crate::wire)): the sequent as the ordinary
/// sequent's form, the inferences in the state's own order, the
/// conclusion first, each `{"left": [ids], "right": [ids], "rule":
/// name, "principal": ["left", position], "premises": [indices]}`, an
/// open goal its two sides alone, and the history as the inferences the
/// steps closed, in order. Reading it back checks every inference by the
/// checker's rules, so a state read is as sound as one built through
/// [`apply`](Self::apply).
///
/// Needs the cargo feature `interactive` (on by default).
#[derive(Clone, Debug)]
pub struct Interactive {
    /// The logic.
    logic: Logic,
    /// The formulas, with the nodes the rules need: false, and the two
    /// implications of every equivalence.
    formulas: Formulas,
    /// The node of false.
    falsity: NodeId,
    /// The inferences, top-down: the root first, then the inferences each
    /// step made, in order; an open goal has [`Rule::Open`].
    inferences: Vec<Inference>,
    /// The goals closed so far, in order, for undo.
    history: Vec<InfId>,
    /// The bound on the bytes of a step's run of structural rules, the
    /// `derivation_bytes` the state was started or read under.
    limit: Option<u64>,
}

/// Returns an open goal with the sides given.
const fn open(left: Vec<NodeId>, right: Vec<NodeId>) -> Inference {
    Inference {
        left,
        right,
        rule: Rule::Open,
        principal: None,
        premises: Vec::new(),
    }
}

impl Interactive {
    /// Starts a proof of the sequent in the logic, with the whole sequent
    /// as the one open goal; an intuitionistic or minimal sequent with
    /// nothing right of `⊢` as `Γ ⊢ ⊥`.
    ///
    /// # Errors
    ///
    /// [`Error::Succedents`] for an intuitionistic or minimal sequent with
    /// more than one formula right of `⊢`, and [`Refusal::Occurrences`]
    /// past the default bound.
    pub fn new(sequent: &Sequent, logic: Logic) -> Result<Self, Error> {
        Self::within(sequent, logic, &Limits::default())
    }

    /// Starts a proof as [`new`](Self::new) does, of a sequent that
    /// unfolds to at most `limits.occurrences` subformula occurrences.
    ///
    /// # Errors
    ///
    /// As [`new`](Self::new) does, the bound being `limits.occurrences`.
    pub fn within(sequent: &Sequent, logic: Logic, limits: &Limits) -> Result<Self, Error> {
        if logic != Logic::Classical && sequent.right.len() > 1 {
            return Err(Error::Succedents {
                count: sequent.right.len(),
            });
        }
        let mut formulas = sequent.formulas.clone();
        bound(&formulas, &sequent.left, &sequent.right, limits)?;
        let falsity = Extra::add(&mut formulas)?.falsity;
        let mut right = sequent.right.clone();
        if logic != Logic::Classical && right.is_empty() {
            right.push(falsity);
        }
        Ok(Self {
            logic,
            formulas,
            falsity,
            inferences: vec![open(sequent.left.clone(), right)],
            history: Vec::new(),
            limit: limits.derivation_bytes,
        })
    }

    /// Returns the logic.
    pub fn logic(&self) -> Logic {
        self.logic
    }

    /// Returns the formulas the goals and inferences name.
    pub fn formulas(&self) -> &Formulas {
        &self.formulas
    }

    /// Returns the sequent being proved, its two sides.
    pub fn sequent(&self) -> (&[NodeId], &[NodeId]) {
        let root = &self.inferences[0];
        (&root.left, &root.right)
    }

    /// Returns the open goals, in the order they were opened.
    pub fn goals(&self) -> impl Iterator<Item = GoalId> + '_ {
        self.inferences
            .iter()
            .enumerate()
            .filter(|(_, inference)| inference.rule == Rule::Open)
            // Fewer inferences than a `u32` counts: each was made by a
            // step, read from a document, or grafted from a derivation.
            .map(|(i, _)| GoalId::new(i as u32))
    }

    /// Returns the two sides of an open goal.
    ///
    /// # Errors
    ///
    /// [`StepError::NoGoal`] if the id is no open goal.
    pub fn goal(&self, id: GoalId) -> Result<(&[NodeId], &[NodeId]), StepError> {
        let goal = self.open(id)?;
        Ok((&goal.left, &goal.right))
    }

    /// Returns the open goal with the id, or the refusal.
    fn open(&self, id: GoalId) -> Result<&Inference, StepError> {
        self.inferences
            .get(id.index())
            .filter(|inference| inference.rule == Rule::Open)
            .ok_or(StepError::NoGoal { goal: id })
    }

    /// Returns whether no goal is open.
    pub fn is_complete(&self) -> bool {
        self.goals().next().is_none()
    }

    /// Returns how many steps were taken: rules applied and goals closed
    /// by the search, less those undone.
    pub fn steps(&self) -> usize {
        self.history.len()
    }

    /// Returns the rules that can act on the formula at `position` of
    /// `side` of the open goal in this logic: the axiom where the formula
    /// stands on the other side too, the rules of its connective on its
    /// side, weakening and contraction, each that the logic has.
    /// Whether a split given fits, and in LJ whether the premises keep one
    /// formula right of `⊢`, is what [`apply`](Self::apply) decides.
    ///
    /// # Errors
    ///
    /// [`StepError::NoGoal`] for a goal that is not open, and
    /// [`StepError::NoFormula`] for a position outside its side.
    pub fn rules(
        &self,
        goal: GoalId,
        side: Side,
        position: usize,
    ) -> Result<Vec<Applicable>, StepError> {
        let goal = self.open(goal)?;
        let x = formula_at(goal, side, position)?;
        let mut rules = Vec::new();
        if goal.side(side.flip()).contains(&x) {
            rules.push(Applicable {
                rule: Rule::Axiom,
                needs: Needs::Nothing,
            });
        }
        for rule in Rule::ALL {
            if let Ok(shape) = self.shape(rule, side, position, x) {
                rules.push(Applicable {
                    rule,
                    needs: match shape {
                        Shape::Binary(_) => Needs::Context,
                        Shape::Leaf | Shape::Unary(..) => Needs::Nothing,
                    },
                });
            }
        }
        Ok(rules)
    }

    /// Returns what `rule` makes of the formula `x` at `position` of
    /// `side`, or why it does not act on it in this logic. The axiom and
    /// an open goal are no shape.
    fn shape(
        &self,
        rule: Rule,
        side: Side,
        position: usize,
        x: NodeId,
    ) -> Result<Shape, StepError> {
        use Rule::*;
        use Side::{Left, Right};
        let node = self.formulas.node(x);
        let fits = match rule {
            Axiom | Cut | Open => false,
            WeakenLeft | ContractLeft => side == Left,
            WeakenRight | ContractRight => side == Right,
            AndLeft | AndLeft1 | AndLeft2 => side == Left && matches!(node, Node::And(..)),
            AndRight => side == Right && matches!(node, Node::And(..)),
            OrLeft => side == Left && matches!(node, Node::Or(..)),
            OrRight | OrRight1 | OrRight2 => side == Right && matches!(node, Node::Or(..)),
            ImpliesLeft => side == Left && matches!(node, Node::Implies(..)),
            ImpliesRight => side == Right && matches!(node, Node::Implies(..)),
            NotLeft => side == Left && matches!(node, Node::Not(_)),
            NotRight => side == Right && matches!(node, Node::Not(_)),
            IffLeft | IffLeft1 | IffLeft2 => side == Left && matches!(node, Node::Iff(..)),
            IffRight => side == Right && matches!(node, Node::Iff(..)),
            FalseLeft => side == Left && node == Node::False,
            FalseRight => side == Right && node == Node::False,
            TrueLeft => side == Left && node == Node::True,
            TrueRight => side == Right && node == Node::True,
        };
        if !fits {
            return Err(StepError::Rule {
                rule,
                side,
                position,
            });
        }
        let classical = self.logic == Logic::Classical;
        let refused = match rule {
            WeakenRight | ContractRight | OrRight | FalseRight => !classical,
            FalseLeft => self.logic == Logic::Minimal,
            Axiom | WeakenLeft | ContractLeft | AndLeft | AndLeft1 | AndLeft2 | AndRight
            | OrLeft | OrRight1 | OrRight2 | ImpliesLeft | ImpliesRight | NotLeft | NotRight
            | IffLeft | IffLeft1 | IffLeft2 | IffRight | TrueLeft | TrueRight | Cut | Open => false,
        };
        if refused {
            return Err(StepError::Logic {
                rule,
                logic: self.logic,
            });
        }
        let ids = &self.formulas.ids;
        // Equivalences have their implications in the arena, which the
        // state added at its start.
        let iff = |a, b| (ids[&Node::Implies(a, b)], ids[&Node::Implies(b, a)]);
        let falsity = self.falsity;
        Ok(match (rule, node) {
            (WeakenLeft | WeakenRight | TrueLeft | FalseRight, _) => Shape::Unary(vec![], vec![]),
            (ContractLeft | ContractRight, _) => Shape::Unary(vec![x, x], vec![]),
            (TrueRight | FalseLeft, _) => Shape::Leaf,
            (AndLeft, Node::And(a, b)) | (OrRight, Node::Or(a, b)) => {
                Shape::Unary(vec![a, b], vec![])
            }
            (AndLeft1, Node::And(a, _)) | (OrRight1, Node::Or(a, _)) => {
                Shape::Unary(vec![a], vec![])
            }
            (AndLeft2, Node::And(_, b)) | (OrRight2, Node::Or(_, b)) => {
                Shape::Unary(vec![b], vec![])
            }
            (ImpliesRight, Node::Implies(a, b)) => Shape::Unary(vec![b], vec![a]),
            (NotRight, Node::Not(a)) if classical => Shape::Unary(vec![], vec![a]),
            (NotRight, Node::Not(a)) => Shape::Unary(vec![falsity], vec![a]),
            (NotLeft, Node::Not(a)) if classical => Shape::Unary(vec![], vec![a]),
            (NotLeft, Node::Not(a)) => Shape::Binary([(vec![], vec![a]), (vec![falsity], vec![])]),
            (IffLeft, Node::Iff(a, b)) => {
                let (forward, backward) = iff(a, b);
                Shape::Unary(vec![forward, backward], vec![])
            }
            (IffLeft1, Node::Iff(a, b)) => Shape::Unary(vec![iff(a, b).0], vec![]),
            (IffLeft2, Node::Iff(a, b)) => Shape::Unary(vec![iff(a, b).1], vec![]),
            (AndRight, Node::And(a, b)) | (OrLeft, Node::Or(a, b)) => {
                Shape::Binary([(vec![a], vec![]), (vec![b], vec![])])
            }
            (ImpliesLeft, Node::Implies(a, b)) => {
                Shape::Binary([(vec![], vec![a]), (vec![b], vec![])])
            }
            (IffRight, Node::Iff(a, b)) => {
                let (forward, backward) = iff(a, b);
                Shape::Binary([(vec![forward], vec![]), (vec![backward], vec![])])
            }
            // Every other pair failed `fits`.
            _ => unreachable!("{rule} fits a formula it has no shape for"),
        })
    }

    /// Applies a step to the open goal and returns the goals it opens, in
    /// the rule's order of premises; a rule without premises opens none.
    /// The axiom closes a goal whose formula stands on both sides of `⊢`,
    /// weakening every other formula first, which the derivation shows as
    /// inferences of their own unless its structural rules are drawn
    /// admissible ([`Structural`]). A rule with two premises gives each
    /// the whole context unless the step splits it ([`Step::split`]); in
    /// intuitionistic and minimal logic `→L` and `¬L`, whose first premise
    /// has its own formula right of `⊢`, give it every hypothesis and the
    /// second premise every hypothesis and the goal's formula right of
    /// `⊢`, the hypotheses contracted first, which an admissible drawing
    /// leaves out.
    /// Every inference the step makes is checked by the rules of the
    /// derivation's checker before the state changes. Fails with what the
    /// rule needed, and changes nothing then.
    ///
    /// # Errors
    ///
    /// A [`StepError`] that names what the rule needed: an open goal, a
    /// formula at the position, a rule that acts on it in the logic, the
    /// formula on both sides for the axiom, a split that fits, and in
    /// intuitionistic and minimal logic one formula right of `⊢` in every
    /// premise.
    pub fn apply(&mut self, goal: GoalId, step: &Step) -> Result<Vec<GoalId>, StepError> {
        let made = self.expand(goal, step)?;
        Ok(self.record(goal, made))
    }

    /// Returns the inferences a step makes, the goal's own first with its
    /// rule, the others after it, each premise of one of them indexing
    /// this list; the premises left open have [`Rule::Open`].
    fn expand(&self, id: GoalId, step: &Step) -> Result<Vec<Inference>, StepError> {
        let goal = self.open(id)?;
        let (side, position, rule) = (step.side, step.position, step.rule);
        let x = formula_at(goal, side, position)?;
        let made = if rule == Rule::Axiom {
            if step.context != Context::Shared {
                return Err(StepError::NoSplit { rule });
            }
            let partner = goal
                .side(side.flip())
                .iter()
                .position(|&y| y == x)
                .ok_or(StepError::NoPartner { side, position })?;
            // A weakening per formula, each with the sequent left.
            let len = goal.left.len() + goal.right.len();
            self.run_within(len, len)?;
            axiom(goal, side, position, partner)
        } else {
            let shape = self.shape(rule, side, position, x)?;
            if step.context != Context::Shared && !matches!(shape, Shape::Binary(_)) {
                return Err(StepError::NoSplit { rule });
            }
            // LJ's `→L` and `¬L` give their first premise a formula right
            // of `⊢` of its own: shared, the hypotheses go to both premises
            // through contractions below, the goal's formula right of `⊢`
            // to the second.
            if self.logic != Logic::Classical
                && step.context == Context::Shared
                && let Shape::Binary([first, second]) = &shape
                && side == Side::Left
                && !first.1.is_empty()
            {
                // A contraction per hypothesis, each sequent longer by one.
                let len = 2 * goal.left.len() + goal.right.len() + first.1.len();
                self.run_within(goal.left.len() + 2, len)?;
                return Ok(self.checked(contracted(goal, position, rule, first, second)));
            }
            let premises = match &shape {
                Shape::Leaf => vec![],
                Shape::Unary(same, other) => {
                    vec![premise(goal, side, position, same, other, |_, _| true)]
                }
                Shape::Binary([first, second]) => {
                    let to_first = self.split(goal, side, position, &step.context)?;
                    let goes = |s: Side, i: usize| match &to_first {
                        None => true,
                        Some((left, right)) => match s {
                            Side::Left => left[i],
                            Side::Right => right[i],
                        },
                    };
                    let stays = |s: Side, i: usize| to_first.is_none() || !goes(s, i);
                    vec![
                        premise(goal, side, position, &first.0, &first.1, goes),
                        premise(goal, side, position, &second.0, &second.1, stays),
                    ]
                }
            };
            if self.logic != Logic::Classical
                && let Some(p) = premises.iter().find(|p| p.right.len() != 1)
            {
                return Err(StepError::Succedents {
                    count: p.right.len(),
                });
            }
            let mut made = vec![Inference {
                left: goal.left.clone(),
                right: goal.right.clone(),
                rule,
                principal: Some((side, position)),
                premises: (1..=premises.len()).map(|k| InfId::new(k as u32)).collect(),
            }];
            made.extend(premises);
            made
        };
        Ok(self.checked(made))
    }

    /// Fails unless a run of `inferences` inferences whose sequents have
    /// at most `len` formulas each is within the state's bound, by the size
    /// estimate's measure with every formula three characters at least
    /// (one and two for its separator): what an axiom's weakenings and a
    /// shared rule's contractions hold grows with the square of the goal.
    fn run_within(&self, inferences: usize, len: usize) -> Result<(), StepError> {
        let (inferences, len) = (inferences as u64, len as u64);
        let bytes = inferences.saturating_mul(
            Size::BYTES_PER_INFERENCE.saturating_add(
                len.saturating_mul(3)
                    .saturating_mul(Size::BYTES_PER_CHARACTER),
            ),
        );
        match self.limit {
            Some(limit) if bytes > limit => Err(StepError::Size {
                estimate_bytes: bytes,
                limit_bytes: limit,
            }),
            _ => Ok(()),
        }
    }

    /// Returns the inferences a step made, each checked by the checker's
    /// own rules, inference by inference: what a step makes is what a
    /// finished derivation's check reads.
    fn checked(&self, made: Vec<Inference>) -> Vec<Inference> {
        for inference in made.iter().filter(|i| i.rule != Rule::Open) {
            let premises: Vec<&Inference> = inference
                .premises
                .iter()
                .map(|p| &made[p.index()])
                .collect();
            if check_inference(&self.formulas, self.logic, inference, &premises).is_err() {
                unreachable!("a step made an inference the checker refuses: {inference:?}");
            }
        }
        made
    }

    /// Returns, for a split, which positions of each side go to the first
    /// premise; `None` for a shared context.
    #[expect(
        clippy::type_complexity,
        reason = "one flag per position of each side, used once"
    )]
    fn split(
        &self,
        goal: &Inference,
        side: Side,
        position: usize,
        context: &Context,
    ) -> Result<Option<(Vec<bool>, Vec<bool>)>, StepError> {
        let Context::Split { left, right } = context else {
            return Ok(None);
        };
        let mut flags = (vec![false; goal.left.len()], vec![false; goal.right.len()]);
        for (s, positions) in [(Side::Left, left), (Side::Right, right)] {
            let list = match s {
                Side::Left => &mut flags.0,
                Side::Right => &mut flags.1,
            };
            for &p in positions {
                let principal = s == side && p == position;
                match list.get_mut(p) {
                    Some(flag) if !*flag && !principal => *flag = true,
                    _ => {
                        return Err(StepError::Split {
                            side: s,
                            position: p,
                        });
                    }
                }
            }
        }
        Ok(Some(flags))
    }

    /// Fills the goal with the first of the inferences made and appends
    /// the others, every premise after its conclusion, records the step
    /// and returns the goals it opened.
    fn record(&mut self, goal: GoalId, mut made: Vec<Inference>) -> Vec<GoalId> {
        let base = self.inferences.len();
        // Inference `k` of the list is the goal for `k = 0`, else the one
        // appended at `base + k - 1`; fewer inferences than a `u32` counts.
        let global = |k: InfId| match k.index() {
            0 => InfId::new(goal.get()),
            k => InfId::new((base + k - 1) as u32),
        };
        for inference in &mut made {
            for p in &mut inference.premises {
                *p = global(*p);
            }
        }
        let opened = made
            .iter()
            .enumerate()
            .filter(|(_, i)| i.rule == Rule::Open)
            .map(|(k, _)| GoalId::new(global(InfId::new(k as u32)).get()))
            .collect();
        let mut made = made.into_iter();
        if let Some(first) = made.next() {
            self.inferences[goal.index()] = first;
        }
        self.inferences.extend(made);
        self.history.push(InfId::new(goal.get()));
        opened
    }

    /// Retracts the last step, a rule applied or a goal closed by the
    /// search, and returns the goal it reopened, or `None` when no step is
    /// left.
    pub fn undo(&mut self) -> Option<GoalId> {
        let goal = self.history.pop()?;
        // The step's inferences are the suffix of the arena.
        if let Some(first) = self.subtree(goal).into_iter().min() {
            self.inferences.truncate(first.index());
        }
        let inference = &mut self.inferences[goal.index()];
        inference.rule = Rule::Open;
        inference.principal = None;
        inference.premises.clear();
        Some(GoalId::new(goal.get()))
    }

    /// Returns the inferences above `id`, excluding it, in no particular
    /// order.
    fn subtree(&self, id: InfId) -> Vec<InfId> {
        let mut ids = self.inferences[id.index()].premises.clone();
        let mut i = 0;
        while i < ids.len() {
            ids.extend(self.inferences[ids[i].index()].premises.iter().copied());
            i += 1;
        }
        ids
    }

    /// Decides the open goal as a sequent of its own in the state's logic,
    /// by the translation given (`None` for the logic's default), with
    /// the search options and within `limits`, until `stop` fires, and
    /// returns the outcome. When the goal is valid, the derivation found
    /// is grafted onto it as one step, which [`undo`](Self::undo) retracts
    /// whole; a goal the search refutes or cannot decide stays open, the
    /// outcome saying why ([`Verdict::Unknown`] with its reason).
    ///
    /// The derivation grafted is within `limits.derivation_bytes` by the
    /// size estimate's measure: a larger one leaves the goal open and
    /// [`Closed::grafted`] says why, the derivation kept in the outcome.
    ///
    /// # Errors
    ///
    /// [`Error::OrdinaryStep`] for a goal that is not open, the errors of
    /// [`decide()`] (a translation that does not decide the logic, the
    /// refusal of a bound or the stop), the goal then still open, and
    /// [`Error::GoalMismatch`] should the derivation found conclude
    /// another sequent than the goal.
    pub fn close(
        &mut self,
        goal: GoalId,
        translation: Option<Translation>,
        search: &search::Options,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Closed, Error> {
        let outcome = self.decide(goal, translation, search, limits, stop)?;
        let grafted = match &outcome.verdict {
            Verdict::Valid(derivation) => match self.graft(goal, derivation, limits) {
                Ok(()) => Ok(()),
                Err(Error::Refused(refusal)) => Err(refusal),
                Err(error) => return Err(error),
            },
            Verdict::NotValid | Verdict::Unknown(_) => Ok(()),
        };
        Ok(Closed { outcome, grafted })
    }

    /// Decides the open goal as [`close`](Self::close) does and returns
    /// the outcome with the first steps of the derivation found, its
    /// structural rules at the root and the rule above them, which
    /// [`apply`](Self::apply) takes in turn ([`Hint::steps`]); the state
    /// is left as it was.
    ///
    /// # Errors
    ///
    /// As [`close`](Self::close) does.
    pub fn hint(
        &self,
        goal: GoalId,
        translation: Option<Translation>,
        search: &search::Options,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Hint, Error> {
        let outcome = self.decide(goal, translation, search, limits, stop)?;
        self.open(goal)?;
        let steps = match &outcome.verdict {
            Verdict::Valid(derivation) => self.first_steps(goal, derivation),
            Verdict::NotValid | Verdict::Unknown(_) => Vec::new(),
        };
        Ok(Hint { outcome, steps })
    }

    /// Decides the open goal as a sequent of its own.
    fn decide(
        &self,
        goal: GoalId,
        translation: Option<Translation>,
        search: &search::Options,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Outcome, Error> {
        let goal = self.open(goal)?;
        let sequent = Sequent {
            formulas: self.formulas.clone(),
            left: goal.left.clone(),
            right: goal.right.clone(),
        };
        let options = super::Options::default()
            .with_logic(self.logic)
            .with_translation(translation);
        decide(&sequent, &options, search, limits, stop)
    }

    /// Returns the id in this state's arena of every formula of `other`,
    /// an arena that extends a copy of it, adding what this one lacks.
    fn ids_of(formulas: &mut Formulas, other: &Formulas) -> Result<Vec<NodeId>, Error> {
        let mut ids: Vec<NodeId> = Vec::with_capacity(other.len());
        // Every subformula precedes the formulas built from it.
        for &node in &other.nodes {
            let at = |k: NodeId| ids[k.index()];
            let id = match node {
                Node::Atom(a) => formulas.atom(other.atom_name(a))?,
                Node::True | Node::False => formulas.add(node)?,
                Node::Not(a) => formulas.add(Node::Not(at(a)))?,
                Node::And(a, b) => formulas.add(Node::And(at(a), at(b)))?,
                Node::Or(a, b) => formulas.add(Node::Or(at(a), at(b)))?,
                Node::Implies(a, b) => formulas.add(Node::Implies(at(a), at(b)))?,
                Node::Iff(a, b) => formulas.add(Node::Iff(at(a), at(b)))?,
            };
            ids.push(id);
        }
        Ok(ids)
    }

    /// Grafts a derivation of the open goal onto it as one step, its root
    /// in the goal's place and the rest appended in reverse, so that every
    /// premise keeps a larger index than its conclusion; refused past
    /// `limits.derivation_bytes` by the size estimate's measure.
    fn graft(&mut self, goal: GoalId, found: &Derivation, limits: &Limits) -> Result<(), Error> {
        let estimate = estimate(found.formulas(), found.inferences());
        if let Some(limit) = limits.derivation_bytes
            && estimate > limit
        {
            return Err(Error::Refused(Refusal::Output {
                what: "derivation",
                estimate_bytes: estimate,
                limit_bytes: limit,
                least_bytes: None,
            }));
        }
        let ids = Self::ids_of(&mut self.formulas, found.formulas())?;
        let map = |list: &[NodeId]| list.iter().map(|k| ids[k.index()]).collect::<Vec<_>>();
        let root = found.inferences().len() - 1;
        let base = self.inferences.len();
        let at = InfId::new(goal.get());
        // The root is the goal; inference `k` below it goes to
        // `base + root - 1 - k`. Fewer inferences than a `u32` counts.
        let renumber = |id: InfId| {
            if id.index() == root {
                at
            } else {
                InfId::new((base + root - 1 - id.index()) as u32)
            }
        };
        let mut grafted: Vec<Inference> = found
            .inferences()
            .iter()
            .map(|inference| Inference {
                left: map(&inference.left),
                right: map(&inference.right),
                rule: inference.rule,
                principal: inference.principal,
                premises: inference.premises.iter().map(|&p| renumber(p)).collect(),
            })
            .collect();
        let mut top = grafted.pop().expect("a derivation has a root");
        // The goal keeps the order of its sides; the root's principal is
        // found there by its formula, the sides being the same multisets.
        let goal_inference = &self.inferences[at.index()];
        if !same(&top.left, &goal_inference.left) || !same(&top.right, &goal_inference.right) {
            return Err(Error::GoalMismatch);
        }
        top.principal = top.principal.map(|(side, position)| {
            let id = top.side(side)[position];
            let list = goal_inference.side(side);
            (side, list.iter().position(|&y| y == id).unwrap_or(position))
        });
        top.left.clone_from(&goal_inference.left);
        top.right.clone_from(&goal_inference.right);
        self.inferences[at.index()] = top;
        self.inferences.extend(grafted.into_iter().rev());
        self.history.push(at);
        Ok(())
    }

    /// Returns the steps of a derivation of the open goal from its root
    /// up to and including the first rule that is not structural, each
    /// applied in a copy of the state to find the goal the next one acts
    /// on. Each turn goes one premise up the derivation, so there are no
    /// more turns than it has inferences; a step the copy refuses (a
    /// bound) ends the list there.
    fn first_steps(&self, goal: GoalId, found: &Derivation) -> Vec<Step> {
        let mut state = self.clone();
        let (mut goal, mut at) = (goal, found.root());
        let mut steps = Vec::new();
        while let Some(step) = state
            .open(goal)
            .ok()
            .and_then(|inference| state.step_at(inference, found, at))
        {
            steps.push(step.clone());
            if !step.rule.is_structural() {
                break;
            }
            match state.apply(goal, &step).as_deref() {
                Ok(&[next]) => goal = next,
                _ => break,
            }
            at = found.inference(at).premises[0];
        }
        steps
    }

    /// Returns the rule of inference `at` of a derivation of the open goal
    /// (whose conclusion is the goal as multisets) as a step on the goal,
    /// its formula found by what it is and, for two premises, the context
    /// as the derivation gave it to them.
    fn step_at(&self, goal: &Inference, found: &Derivation, at: InfId) -> Option<Step> {
        let mut formulas = self.formulas.clone();
        let ids = Self::ids_of(&mut formulas, found.formulas()).ok()?;
        let map = |list: &[NodeId]| list.iter().map(|k| ids[k.index()]).collect::<Vec<_>>();
        let root = found.inference(at);
        let Some((side, at)) = root.principal else {
            // An axiom concludes exactly the goal `A ⊢ A`.
            return Some(Step::new(Side::Right, 0, root.rule));
        };
        let x = ids[root.side(side)[at].index()];
        let position = goal.side(side).iter().position(|&y| y == x)?;
        let step = Step::new(side, position, root.rule);
        let Ok(Shape::Binary([first, _])) = self.shape(root.rule, side, position, x) else {
            return Some(step);
        };
        let given = found.inference(root.premises[0]);
        let (left, right) = (map(&given.left), map(&given.right));
        let shared = premise(goal, side, position, &first.0, &first.1, |_, _| true);
        if same(&shared.left, &left) && same(&shared.right, &right) {
            return Some(step);
        }
        // What the first premise holds beyond the rule's own formulas,
        // each matched to a position of the goal's context.
        let mut rest = (left, right);
        let (same_side, other_side) = match side {
            Side::Left => (&mut rest.0, &mut rest.1),
            Side::Right => (&mut rest.1, &mut rest.0),
        };
        for (list, added) in [(same_side, &first.0), (other_side, &first.1)] {
            for y in added {
                let i = list.iter().position(|z| z == y)?;
                list.swap_remove(i);
            }
        }
        let mut positions = (Vec::new(), Vec::new());
        for (s, held, out) in [
            (Side::Left, &mut rest.0, &mut positions.0),
            (Side::Right, &mut rest.1, &mut positions.1),
        ] {
            for (i, y) in goal.side(s).iter().enumerate() {
                if s == side && i == position {
                    continue;
                }
                if let Some(k) = held.iter().position(|z| z == y) {
                    held.swap_remove(k);
                    out.push(i);
                }
            }
        }
        Some(step.split(&positions.0, &positions.1))
    }

    /// Returns the derivation so far with the open goals as leaves of
    /// [`Rule::Open`], under the default view and limits.
    ///
    /// # Errors
    ///
    /// As [`derivation_within`](Self::derivation_within) does.
    pub fn derivation(&self) -> Result<Derivation, Error> {
        self.derivation_within(&ViewOptions::default(), &Limits::default(), |_| false)
    }

    /// Returns the derivation so far with the open goals as leaves of
    /// [`Rule::Open`], renumbered premises before conclusions, so that its
    /// ids are not this state's ([`derivation_ids`](Self::derivation_ids)
    /// maps them), with its structural rules as `view.structural` draws
    /// them; refused past `limits.derivation_bytes` by its estimate before
    /// anything is built, and when `stop` fires.
    ///
    /// # Errors
    ///
    /// [`Refusal::Output`] past `limits.derivation_bytes`, and
    /// [`Refusal::Stopped`] when `stop` fired.
    pub fn derivation_within(
        &self,
        view: &ViewOptions,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Derivation, Error> {
        let estimate = estimate(&self.formulas, &self.inferences);
        if let Some(limit) = limits.derivation_bytes
            && estimate > limit
        {
            return Err(Error::Refused(Refusal::Output {
                what: "derivation",
                estimate_bytes: estimate,
                limit_bytes: limit,
                least_bytes: None,
            }));
        }
        if stop(Progress::new(Phase::View, 0, 0)) {
            return Err(Error::Refused(Refusal::Stopped { phase: Phase::View }));
        }
        let order = self.postorder();
        let mut new_id = vec![InfId::new(u32::MAX); self.inferences.len()];
        for (i, id) in order.iter().enumerate() {
            // As many as the state's inferences, which a `u32` counts.
            new_id[id.index()] = InfId::new(i as u32);
        }
        let inferences = order
            .iter()
            .map(|id| {
                let inference = &self.inferences[id.index()];
                Inference {
                    premises: inference
                        .premises
                        .iter()
                        .map(|p| new_id[p.index()])
                        .collect(),
                    ..inference.clone()
                }
            })
            .collect();
        let (left, right) = self.sequent();
        let derivation = Derivation::from_parts(
            self.logic,
            self.formulas.clone(),
            (left.to_vec(), right.to_vec()),
            inferences,
        );
        Ok(match view.structural {
            Structural::Explicit => derivation,
            Structural::Admissible => derivation.admissible().0,
        })
    }

    /// Returns, for every inference of the derivation that
    /// [`derivation_within`](Self::derivation_within) gives under the
    /// structural rules drawn as `structural` says, the id of the
    /// inference of this state it shows: what turns an inference of a
    /// drawing into the goal that [`apply`](Self::apply) and
    /// [`close`](Self::close) take.
    pub fn derivation_ids(&self, structural: Structural) -> Vec<GoalId> {
        let order = self.postorder();
        let goal = |id: InfId| GoalId::new(id.get());
        match structural {
            Structural::Explicit => order.into_iter().map(goal).collect(),
            Structural::Admissible => {
                // The shape alone decides what is shown: the derivation's
                // inferences without their sequents.
                let mut new_id = vec![InfId::new(0); self.inferences.len()];
                for (i, id) in order.iter().enumerate() {
                    new_id[id.index()] = InfId::new(i as u32);
                }
                let shape: Vec<Inference> = order
                    .iter()
                    .map(|id| {
                        let inference = &self.inferences[id.index()];
                        Inference {
                            left: Vec::new(),
                            right: Vec::new(),
                            rule: inference.rule,
                            principal: None,
                            premises: inference
                                .premises
                                .iter()
                                .map(|p| new_id[p.index()])
                                .collect(),
                        }
                    })
                    .collect();
                let skeleton = Derivation::from_parts(
                    self.logic,
                    Formulas::default(),
                    (Vec::new(), Vec::new()),
                    shape,
                );
                let (_, origin) = skeleton.admissible();
                origin.into_iter().map(|i| goal(order[i.index()])).collect()
            }
        }
    }

    /// Returns the arena's inferences in postorder, premises before
    /// conclusions, the order of [`derivation`](Self::derivation).
    fn postorder(&self) -> Vec<InfId> {
        let mut order = Vec::with_capacity(self.inferences.len());
        // Postorder without recursion: a frame is an inference and whether
        // its premises were visited.
        let mut stack = vec![(InfId::new(0), false)];
        while let Some((id, visited)) = stack.pop() {
            if visited {
                order.push(id);
            } else {
                stack.push((id, true));
                let premises = &self.inferences[id.index()].premises;
                stack.extend(premises.iter().rev().map(|&p| (p, false)));
            }
        }
        order
    }

    /// Returns the derivation of a finished proof, every structural rule
    /// explicit, checked by [`Derivation::check`] within `limits` and
    /// until `stop` fires: the checker is the final word, though it never
    /// rejects what this state built.
    ///
    /// # Errors
    ///
    /// [`Error::OpenGoals`] while goals are open, the refusals of
    /// [`derivation_within`](Self::derivation_within), and those of
    /// [`Derivation::check`] with its rejection.
    pub fn finished(
        &self,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Derivation, Error> {
        let open = self.goals().count();
        if open > 0 {
            return Err(Error::OpenGoals { count: open });
        }
        let view = ViewOptions::default().with_structural(Structural::Explicit);
        let derivation = self.derivation_within(&view, limits, &mut stop)?;
        derivation.check(limits, stop)?;
        Ok(derivation)
    }
}

/// Returns the formula at a position of a side of a goal, or the
/// refusal.
fn formula_at(goal: &Inference, side: Side, position: usize) -> Result<NodeId, StepError> {
    let list = goal.side(side);
    list.get(position).copied().ok_or(StepError::NoFormula {
        side,
        position,
        len: list.len(),
    })
}

/// Returns the open premise that keeps the goal's context formulas for
/// which `keep` holds, in their order, the principal formula's place on
/// its side taken by `same` and `other` joining the other side's end.
fn premise(
    goal: &Inference,
    side: Side,
    position: usize,
    same: &[NodeId],
    other: &[NodeId],
    keep: impl Fn(Side, usize) -> bool,
) -> Inference {
    let mut sides = [Vec::new(), Vec::new()];
    for (k, s) in [Side::Left, Side::Right].into_iter().enumerate() {
        for (i, &y) in goal.side(s).iter().enumerate() {
            if s == side && i == position {
                sides[k].extend_from_slice(same);
            } else if keep(s, i) {
                sides[k].push(y);
            }
        }
    }
    let [mut left, mut right] = sides;
    match side {
        Side::Left => right.extend_from_slice(other),
        Side::Right => left.extend_from_slice(other),
    }
    open(left, right)
}

/// Returns the inferences of a rule with two premises on the hypothesis
/// at `position` whose first premise has its own formula right of `⊢`
/// (LJ's `→L` and `¬L`), the context shared as the checker can read it:
/// a contraction of every other hypothesis, each copy right after its
/// formula, then the rule with the formulas' first copies going to the
/// first premise and the second copies and the goal's formulas right of
/// `⊢` to the second.
fn contracted(
    goal: &Inference,
    position: usize,
    rule: Rule,
    first: &(Vec<NodeId>, Vec<NodeId>),
    second: &(Vec<NodeId>, Vec<NodeId>),
) -> Vec<Inference> {
    let mut made = Vec::with_capacity(goal.left.len() + 2);
    let mut left = goal.left.clone();
    // Each original hypothesis `i` stands at `i + k` once `k` copies were
    // put before it; `copies[j]` marks the copies of the final sequent.
    let mut copies = vec![false; left.len()];
    let mut k = 0;
    for i in 0..goal.left.len() {
        if i == position {
            continue;
        }
        let at = i + k;
        // Each inference's premise is the next one made; fewer than a
        // `u32` counts, as the goal's formulas are.
        made.push(Inference {
            left: left.clone(),
            right: goal.right.clone(),
            rule: Rule::ContractLeft,
            principal: Some((Side::Left, at)),
            premises: vec![InfId::new(made.len() as u32 + 1)],
        });
        left.insert(at + 1, left[at]);
        copies.insert(at + 1, true);
        k += 1;
    }
    // The rule's formula is an original, never copied, in its place.
    let principal = (0..left.len())
        .filter(|&j| !copies[j])
        .nth(position)
        .expect("the rule's formula is a hypothesis of the goal");
    let contracted = open(left, goal.right.clone());
    let n = made.len() as u32;
    made.push(Inference {
        left: contracted.left.clone(),
        right: contracted.right.clone(),
        rule,
        principal: Some((Side::Left, principal)),
        premises: vec![InfId::new(n + 1), InfId::new(n + 2)],
    });
    let to_first = |s: Side, j: usize| s == Side::Left && !copies[j];
    let to_second = |s: Side, j: usize| s == Side::Right || copies[j];
    made.push(premise(
        &contracted,
        Side::Left,
        principal,
        &first.0,
        &first.1,
        to_first,
    ));
    made.push(premise(
        &contracted,
        Side::Left,
        principal,
        &second.0,
        &second.1,
        to_second,
    ));
    made
}

/// Returns the inferences that close a goal by the axiom on the formula
/// at `position` of `side` and its partner at `partner` on the other side:
/// a weakening of every other formula, the last formula left of `⊢` first,
/// then right of it, and the axiom above them.
fn axiom(goal: &Inference, side: Side, position: usize, partner: usize) -> Vec<Inference> {
    let (mut keep_left, mut keep_right) = match side {
        Side::Left => (position, partner),
        Side::Right => (partner, position),
    };
    let (mut left, mut right) = (goal.left.clone(), goal.right.clone());
    let mut made = Vec::with_capacity(left.len() + right.len() - 1);
    // The formulas left of `⊢` from the last, then those right of it.
    let mut weaken =
        |rule: Rule, s: Side, at: usize, left: &mut Vec<NodeId>, right: &mut Vec<NodeId>| {
            // Each inference's premise is the next one made; fewer than a
            // `u32` counts, as the goal's formulas are.
            let next = InfId::new(made.len() as u32 + 1);
            made.push(Inference {
                left: left.clone(),
                right: right.clone(),
                rule,
                principal: Some((s, at)),
                premises: vec![next],
            });
            match s {
                Side::Left => left.remove(at),
                Side::Right => right.remove(at),
            };
        };
    for at in (0..left.len()).rev() {
        if at != keep_left {
            weaken(Rule::WeakenLeft, Side::Left, at, &mut left, &mut right);
            keep_left -= usize::from(at < keep_left);
        }
    }
    for at in (0..right.len()).rev() {
        if at != keep_right {
            weaken(Rule::WeakenRight, Side::Right, at, &mut left, &mut right);
            keep_right -= usize::from(at < keep_right);
        }
    }
    made.push(Inference {
        left,
        right,
        rule: Rule::Axiom,
        principal: None,
        premises: Vec::new(),
    });
    made
}

/// Returns the bytes the inferences are estimated at by the measure of
/// [`Size::bytes`]: per inference [`Size::BYTES_PER_INFERENCE`] and
/// [`Size::BYTES_PER_CHARACTER`] for every character of its sequent
/// written, each formula with two for its separator, every binary
/// subformula in brackets; saturating.
fn estimate(formulas: &Formulas, inferences: &[Inference]) -> u64 {
    let widths = super::derivation::widths(formulas);
    inferences.iter().fold(0u64, |sum, inference| {
        let bytes = super::derivation::inference_bytes(&widths, &inference.left, &inference.right);
        sum.saturating_add(bytes)
    })
}

/// What a state holds, as its JSON form writes and reads it.
#[cfg(feature = "serialize")]
pub(crate) struct Parts {
    /// The logic.
    pub(crate) logic: Logic,
    /// The formulas.
    pub(crate) formulas: Formulas,
    /// The inferences, the root first.
    pub(crate) inferences: Vec<Inference>,
    /// The steps, each the inference it closed.
    pub(crate) history: Vec<InfId>,
}

#[cfg(feature = "serialize")]
impl Interactive {
    /// Returns what the state holds, for its JSON form.
    pub(crate) fn parts(&self) -> (Logic, &Formulas, &[Inference], &[InfId]) {
        (self.logic, &self.formulas, &self.inferences, &self.history)
    }

    /// Returns the state the parts describe, checked as a state built
    /// through [`apply`](Self::apply) is: the root first, every other
    /// inference the premise of exactly one before it, an open goal with
    /// neither rule nor premises and in intuitionistic and minimal logic
    /// one formula right of `⊢`, every other inference passing the
    /// checker's rules with its premises, the sequent within
    /// `limits.occurrences`, and the history distinct closed inferences
    /// each of whose steps added the suffix of the arena it then had.
    pub(crate) fn from_parts(parts: Parts, limits: &Limits) -> Result<Self, Error> {
        let Parts {
            logic,
            mut formulas,
            inferences,
            history,
        } = parts;
        let inconsistent = |reason| Error::InconsistentSession { reason };
        let root = inferences
            .first()
            .ok_or(inconsistent("the session has no inference"))?;
        bound(&formulas, &root.left, &root.right, limits)?;
        let falsity = Extra::add(&mut formulas)?.falsity;
        let len = inferences.len();
        let mut parents = vec![0u8; len];
        for (n, inference) in inferences.iter().enumerate() {
            for id in inference.left.iter().chain(&inference.right) {
                formulas.holds(*id)?;
            }
            if let Some((side, at)) = inference.principal
                && at >= inference.side(side).len()
            {
                return Err(inconsistent(
                    "a principal position lies outside its sequent",
                ));
            }
            for p in &inference.premises {
                if p.index() <= n || p.index() >= len {
                    return Err(inconsistent("a premise does not follow its conclusion"));
                }
                parents[p.index()] = parents[p.index()].saturating_add(1);
            }
            if inference.rule == Rule::Open {
                let right = inference.right.len();
                if inference.principal.is_some() || !inference.premises.is_empty() {
                    return Err(inconsistent("an open goal has a principal or premises"));
                }
                if logic != Logic::Classical && right != 1 {
                    return Err(inconsistent(
                        "an open goal of LJ has not one formula right of ⊢",
                    ));
                }
                continue;
            }
            let premises: Vec<&Inference> = inference
                .premises
                .iter()
                .map(|p| &inferences[p.index()])
                .collect();
            if check_inference(&formulas, logic, inference, &premises).is_err() {
                return Err(inconsistent("an inference breaks the rule it names"));
            }
        }
        if parents[0] != 0 || parents[1..].iter().any(|&n| n != 1) {
            return Err(inconsistent(
                "an inference is not the premise of exactly one",
            ));
        }
        // The history from the last step back: each step's goal closed,
        // named once, and what it added the suffix of the arena it left,
        // walked without entering what a later step added.
        let mut end = len;
        let mut named = vec![false; len];
        for &goal in history.iter().rev() {
            let g = goal.index();
            if g >= end || named[g] || inferences[g].rule == Rule::Open {
                return Err(inconsistent(
                    "a step names no inference it could have closed",
                ));
            }
            // Its undo reopens the inference as a goal, which in LJ has
            // one formula right of `⊢`.
            if logic != Logic::Classical && inferences[g].right.len() != 1 {
                return Err(inconsistent(
                    "a step of LJ closed a goal without one formula right of ⊢",
                ));
            }
            named[g] = true;
            let mut added = 0;
            let mut least = end;
            let mut stack: Vec<InfId> = inferences[g].premises.clone();
            while let Some(p) = stack.pop() {
                if p.index() >= end {
                    continue;
                }
                added += 1;
                least = least.min(p.index());
                stack.extend(inferences[p.index()].premises.iter().copied());
            }
            if least != end - added || g >= end - added {
                return Err(inconsistent("a step did not add the suffix of the arena"));
            }
            end -= added;
        }
        Ok(Self {
            logic,
            formulas,
            falsity,
            inferences,
            history,
            limit: limits.derivation_bytes,
        })
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::linear::search::generate::Rng;
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
            let [p, q] = step(&mut s, g, (Left, "¬a"), Rule::NotLeft, Some((first, &[])))[..]
            else {
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
            "the rule ∨R is not one of LJ in intuitionistic logic: its premise would have two \
             formulas right of ⊢, and every goal keeps one"
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
            "the axiom needs formula 0 left of ⊢ on the other side of ⊢ too"
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
            "the goal has no formula 3 right of ⊢: it has 1 formula"
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
            "the rule ⊥L is not one of LJ in minimal logic: minimal logic has no ex falso"
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
    /// uses both copies, where hinted alone it was answered by a weakening
    /// of the copy, and the two alternated for ever.
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
        let inconsistent =
            |json: &str| matches!(read(json), Err(Error::InconsistentSession { .. }));
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
        // An open goal is a leaf: drawn bare with a premise above it, its
        // layout's height underflowed.
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
            Err(Error::ReadBack { .. })
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
}
