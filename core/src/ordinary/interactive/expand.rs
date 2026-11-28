// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The steps of a session: a rule checked against its formula and the
//! logic, expanded into its premises and recorded, and undone.

use super::witness::Made;
use super::{Context, Interactive, Shape, Step, StepError, formula_at, open};
use crate::linear::proofs::{GoalId, InfId, Size};
use crate::ordinary::derivation::check_inference;
use crate::ordinary::instance::Resolution;
use crate::ordinary::{Inference, Logic, Node, NodeId, Rule, Side};

/// Returns the open premise that keeps the goal's context formulas for
/// which `keep` holds, in their order, the principal formula's place on
/// its side taken by `same` and `other` joining the other side's end.
pub(super) fn premise(
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
            term: None,
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
        term: None,
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
pub(super) fn axiom(
    goal: &Inference,
    side: Side,
    position: usize,
    partner: usize,
) -> Vec<Inference> {
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
                term: None,
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
        term: None,
        left,
        right,
        rule: Rule::Axiom,
        principal: None,
        premises: Vec::new(),
    });
    made
}

impl Interactive {
    /// Returns what `rule` makes of the formula `x` at `position` of
    /// `side`, or why it does not act on it in this logic. The axiom and
    /// an open goal are no shape.
    pub(super) fn shape(
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
            ForallLeft => side == Left && matches!(node, Node::Forall(_)),
            ForallRight => side == Right && matches!(node, Node::Forall(_)),
            ExistsLeft => side == Left && matches!(node, Node::Exists(_)),
            ExistsRight => side == Right && matches!(node, Node::Exists(_)),
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
            | IffLeft | IffLeft1 | IffLeft2 | IffRight | TrueLeft | TrueRight | Cut | Open
            | ForallLeft | ForallRight | ExistsLeft | ExistsRight => false,
        };
        if refused {
            return Err(StepError::Logic {
                rule,
                logic: self.logic,
            });
        }
        let formulas = &self.formulas;
        let id = |node| {
            formulas
                .find(node)
                .expect("the state's arena holds its extra nodes")
        };
        // Equivalences have their implications in the arena, which the
        // state added at its start.
        let iff = |a, b| (id(Node::Implies(a, b)), id(Node::Implies(b, a)));
        let falsity = self.falsity;
        Ok(match (rule, node) {
            (ForallLeft | ForallRight | ExistsLeft | ExistsRight, _) => Shape::Instance,
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
    ///
    /// The axiom closes a goal whose formula stands on both sides of `⊢`,
    /// weakening every other formula away first. The derivation shows
    /// these weakenings unless its structural rules are drawn admissible
    /// ([`Structural`](crate::linear::proofs::Structural)). A rule with two premises gives each the whole
    /// context unless the step splits it ([`Step::split`]). In
    /// intuitionistic and minimal logic, `→L` and `¬L` have a first premise
    /// with its own formula right of `⊢`. With a shared context they give
    /// every hypothesis to both premises and the goal's formula right of
    /// `⊢` to the second; the hypotheses are contracted first, which an
    /// admissible drawing leaves out.
    ///
    /// Every inference the step makes is checked by the rules of the
    /// derivation's checker before the state changes. A refused step
    /// changes nothing.
    ///
    /// # Errors
    ///
    /// A [`StepError`] that names what the rule needed: an open goal, a
    /// formula at the position, a rule that acts on it in the logic, the
    /// formula on both sides for the axiom, a split that fits, and in
    /// intuitionistic and minimal logic one formula right of `⊢` in every
    /// premise.
    pub fn apply(&mut self, goal: GoalId, step: &Step) -> Result<Vec<GoalId>, StepError> {
        if matches!(
            step.rule,
            Rule::ForallLeft | Rule::ForallRight | Rule::ExistsLeft | Rule::ExistsRight
        ) {
            let made = self.instance_step(goal, step)?;
            return Ok(self.record(goal, made));
        }
        if step.witness.is_some() {
            return Err(StepError::NoWitness { rule: step.rule });
        }
        if step.rule == Rule::Axiom
            && self.witnesses.is_some()
            && let Some(opened) = self.unifying_axiom(goal, step)?
        {
            return Ok(opened);
        }
        let made = self.expand(goal, step)?;
        Ok(self.record(goal, made))
    }

    /// Returns the inferences a step makes, the goal's own first with its
    /// rule, the others after it, each premise of one of them indexing
    /// this list; the premises left open have [`Rule::Open`].
    pub(super) fn expand(&self, id: GoalId, step: &Step) -> Result<Vec<Inference>, StepError> {
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
            // One weakening per formula, each holding what is left of the
            // sequent.
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
                Shape::Instance => unreachable!("apply takes the quantifier rules apart"),
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
                term: None,
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

    /// Refuses a run of `inferences` inferences, each sequent at most
    /// `len` formulas, past the state's bound. It uses the size estimate's
    /// measure, counting every formula as three characters (at least one,
    /// and two for its separator). An axiom's weakenings and a shared
    /// rule's contractions grow with the square of the goal, so they are
    /// checked before they are made.
    pub(super) fn run_within(&self, inferences: usize, len: usize) -> Result<(), StepError> {
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
    pub(super) fn checked(&self, made: Vec<Inference>) -> Vec<Inference> {
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
    pub(super) fn record(&mut self, goal: GoalId, made: Vec<Inference>) -> Vec<GoalId> {
        let len = made.len();
        self.record_made(goal, made, vec![None; len])
    }

    /// Records a step as [`record`](Self::record) does, `raw` saying for
    /// each inference made what it was made as where a binding of the step
    /// changed it.
    pub(super) fn record_made(
        &mut self,
        goal: GoalId,
        mut made: Vec<Inference>,
        raw: Vec<Option<Made>>,
    ) -> Vec<GoalId> {
        if let Some(w) = self.witnesses.as_deref_mut() {
            let mut raw = raw.into_iter();
            // The goal keeps what its sides were made as; its rule's term
            // is the step's.
            let first = raw.next().flatten();
            if let Some(kept) = &mut w.raw[goal.index()] {
                kept.term = first.map_or(made.first().and_then(|i| i.term), |m| m.term);
            }
            w.raw.extend(raw);
        }
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
        if let Some(w) = self.witnesses.as_deref_mut() {
            w.registered.resize(self.inferences.len(), Vec::new());
            for i in std::iter::once(goal.index()).chain(base..self.inferences.len()) {
                let inference = &self.inferences[i];
                let metas = self.formulas.metas_in(
                    inference.left.iter().chain(&inference.right).copied(),
                    inference.term.into_iter(),
                );
                w.register(i, &metas);
            }
        }
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
        inference.term = None;
        if let Some(w) = self.witnesses.as_deref_mut() {
            w.forget_from(self.inferences.len());
            if let Some(kept) = &mut w.raw[goal.index()] {
                kept.term = None;
            }
            // The step's number, under which its bindings were trailed.
            let number = self.history.len() as u32;
            let was: Vec<bool> = w
                .unifier
                .metas()
                .iter()
                .map(|m| m.bound.is_some())
                .collect();
            w.unifier.undo(number);
            while w.made.last().is_some_and(|&s| s >= number) {
                w.made.pop();
            }
            w.unifier.truncate_metas(w.made.len());
            w.holders.truncate(w.made.len());
            // The witnesses the step had bound, and the inferences that
            // showed them before.
            let unbound: Vec<u32> = (0..w.unifier.next_meta())
                .filter(|&k| was[k as usize] && w.unifier.metas()[k as usize].bound.is_none())
                .collect();
            let changed = w.holding(unbound.into_iter());
            if !changed.is_empty() {
                self.reshow(&changed);
            }
            if self.witnesses.as_ref().is_some_and(|w| w.made.is_empty()) {
                self.witnesses = None;
            }
        }
        Some(GoalId::new(goal.get()))
    }

    /// Shows the bindings in force, from what it was made as, in every
    /// inference of `changed` that a binding changed.
    fn reshow(&mut self, changed: &[usize]) {
        let Some(w) = self.witnesses.as_deref_mut() else {
            return;
        };
        let bindings = w.unifier.metas().iter().map(|m| m.bound).collect();
        let mut resolution = Resolution::new(bindings);
        for &i in changed {
            let (Some(inference), Some(raw)) = (self.inferences.get_mut(i), w.raw.get_mut(i))
            else {
                continue;
            };
            let Some(made) = raw else { continue };
            let mut again = |list: &[NodeId]| {
                list.iter()
                    .map(|&n| resolution.node(&mut self.formulas, n))
                    .collect::<Result<Vec<_>, _>>()
                    .expect("a binding taken back shows nodes the arena held before")
            };
            inference.left = again(&made.left);
            inference.right = again(&made.right);
            inference.term = made.term.map(|t| resolution.term(&mut self.formulas, t));
            if made.shows(inference) {
                *raw = None;
            }
        }
    }

    /// Returns the inferences above `id`, excluding it, in no particular
    /// order.
    fn subtree(&self, id: InfId) -> Vec<InfId> {
        crate::goal_tree::subtree(|i| &self.inferences[i.index()].premises[..], id)
    }
}
