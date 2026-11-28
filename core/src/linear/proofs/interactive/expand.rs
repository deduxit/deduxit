// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The steps of a session: the rules a formula admits, a rule applied and
//! expanded into its premises, undo, and a goal closed by the search.

use super::{
    Applicable, Candidate, Closed, GoalId, Interactive, Needs, Status, Step, StepError, Witness,
};
use crate::errors::describe::{Names, cut};
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::occurrences::instances::{Framing, eigen_names};
use crate::linear::occurrences::{Member, OccId, Reading, Side};
use crate::linear::proofs::derivation::{Derivation, InfId, Inference, ViewOptions};
use crate::linear::proofs::multiset::Multiset;
use crate::linear::proofs::{Branch, Named, Proof, Rule};
use crate::linear::search::{self, Options, Verdict};
use crate::linear::sequents::Kind;
use crate::unify::Clash;
use crate::{Error, WitnessFault};

impl Interactive {
    /// Returns the refusal of a rule that does not act on the formula of
    /// `member`, at `position` of its goal, with the formula's text.
    fn rule_refused(&self, rule: Named, position: usize, member: Member) -> StepError {
        let reading = self.reading();
        let framing = self
            .fo
            .as_ref()
            .map(|fo| Framing::new(&self.forest.sequent().fo, &fo.shown));
        let names = Names {
            forest: &self.forest,
            reading: reading.as_ref(),
            framing: framing.as_ref(),
        };
        // The writer stops at the budget, so a formula that unfolds past
        // any memory is never written out. A `String` takes any text, so
        // only the budget ends the writing, with `…`.
        let mut text = String::new();
        let _ = cut(&mut text, Some(names), member, Some(StepError::SHOWN));
        StepError::Rule {
            rule,
            position,
            formula: text.into_boxed_str(),
        }
    }

    /// Returns the occurrences of an open goal's sequent, or the refusal.
    pub(super) fn open(&self, id: GoalId) -> Result<Vec<OccId>, StepError> {
        let goal = self.goal(id)?;
        Ok(goal.iter().map(|&m| self.occurrence(m)).collect())
    }

    /// Returns the rules that can act on the formula at `position` of the
    /// open goal, judged by the formula's connective and the mode alone.
    ///
    /// They are the axiom on a literal, `⊗ ⅋ 1 ⊥ & ⊕₁ ⊕₂ ⊤ ! ∀ ∃` on their
    /// connectives, `?d ?c ?w` on a `?` formula, `wk` on any other formula
    /// in affine mode, and Mix on any formula when the mode has it. In
    /// intuitionistic mode they carry their two-sided names. Nothing acts
    /// on `0`. Whether the rest of the goal lets a rule apply (the axiom's
    /// dual, the context of `1` and `!`, a valid split, one succedent) is
    /// for [`apply`](Self::apply) to decide.
    ///
    /// # Errors
    ///
    /// [`StepError::NoGoal`] for a goal that is not open, and
    /// [`StepError::NoFormula`] for a position outside its sequent.
    pub fn rules(&self, goal: GoalId, position: usize) -> Result<Vec<Applicable>, StepError> {
        let sequent = self.open(goal)?;
        let o = formula_at(&sequent, position)?;
        use Rule::*;
        let mut rules = match self.forest.kind(o) {
            Kind::Atom | Kind::DualAtom => vec![Ax],
            Kind::Tensor => vec![Tensor],
            Kind::Par => vec![Par],
            Kind::One => vec![One],
            Kind::Bot => vec![Bot],
            Kind::With => vec![With],
            Kind::Plus => vec![PlusLeft, PlusRight],
            Kind::Top => vec![Top],
            Kind::Zero => vec![],
            Kind::Bang => vec![Promotion],
            Kind::Quest => vec![Dereliction, Contraction, Weakening],
            Kind::Forall => vec![Forall],
            Kind::Exists => vec![Exists],
        };
        if self.mode.affine && self.forest.kind(o) != Kind::Quest {
            rules.push(AffineWeakening);
        }
        if self.mode.mix {
            rules.push(Mix);
        }
        let reading = self.reading();
        Ok(rules
            .into_iter()
            .map(|rule| Applicable {
                rule: Named::applied(rule, reading.as_ref(), o),
                needs: match rule {
                    Tensor | Mix => Needs::Split,
                    Exists => Needs::Witness,
                    Ax | Par | One | Bot | With | PlusLeft | PlusRight | Top | Promotion
                    | Dereliction | Contraction | Weakening | AffineWeakening | Cut | Forall
                    | Open => Needs::Nothing,
                },
            })
            .collect())
    }

    /// Returns every rule that acts on the formula at `position` of the
    /// open goal, as [`rules`](Self::rules) lists them, each with whether a
    /// step of it is legal on the goal as it stands: it applies, it takes
    /// a split or a witness and some choice is legal, or it is refused
    /// with [`apply`](Self::apply)'s reason.
    ///
    /// Each rule costs at most one `apply`, and less without quantifiers,
    /// where the checks run without a step. A rule that takes a split is
    /// judged by one split the goal allows, found in time linear in the
    /// goal; one that takes a witness by its witness left open. The
    /// session does not change.
    ///
    /// # Errors
    ///
    /// As [`rules`](Self::rules): [`StepError::NoGoal`] for a goal that is
    /// not open, and [`StepError::NoFormula`] for a position outside its
    /// sequent.
    pub fn candidates(&self, goal: GoalId, position: usize) -> Result<Vec<Candidate>, StepError> {
        let rules = self.rules(goal, position)?;
        let sequent = self.open(goal)?;
        let reading = self.reading();
        // A step at first order makes table entries, so it is taken on a
        // copy and taken back.
        let mut trial: Option<Self> = None;
        let mut candidates = Vec::with_capacity(rules.len());
        for Applicable { rule, needs } in rules {
            let step = match needs {
                Needs::Split => self.allowed_split(reading.as_ref(), &sequent, position, rule),
                Needs::Nothing | Needs::Witness => Step::new(position, rule),
            };
            let judged = if self.fo.is_some() {
                let trial = trial.get_or_insert_with(|| self.clone());
                trial.apply(goal, &step).map(|_| {
                    trial.undo();
                })
            } else {
                step.left_positions(sequent.len()).and_then(|left| {
                    self.expand(reading.as_ref(), &sequent, position, rule, &left)
                        .map(drop)
                })
            };
            let status = match (judged, needs) {
                (Ok(()), Needs::Nothing) => Status::Applies,
                (Ok(()), needs) => Status::NeedsInput { needs },
                (Err(error), _) => Status::Refused(error),
            };
            candidates.push(Candidate { rule, status });
        }
        Ok(candidates)
    }

    /// Returns a step of `rule`, a `⊗` or Mix on the formula at `position`
    /// of `sequent`, whose split the goal allows where there is one, else
    /// the step that sends every other formula to the right premise.
    ///
    /// Only the two arcs round from the formula are tried, the left
    /// premise's last: in an ordered mode they are the only splits, and in
    /// a commutative one some arc is allowed whenever some split is, since
    /// what a split must meet (one formula right of `⊢` in each premise, a
    /// formula left of it in the Lambek calculus without empty
    /// antecedents, a right premise that is not empty for Mix) counts the
    /// formulas of each kind on each side, and an arc can put the one
    /// formula right of `⊢` on either side. The counts are prefix sums, so
    /// the search is linear in the goal.
    fn allowed_split(
        &self,
        reading: Option<&Reading>,
        sequent: &[OccId],
        position: usize,
        rule: Named,
    ) -> Step {
        let f = &self.forest;
        let o = sequent[position];
        let round: Vec<usize> = (position + 1..sequent.len()).chain(0..position).collect();
        let output = |x: OccId| reading.is_some_and(|r| r.position(x) == Side::Output);
        // What the principal formula adds to each premise: its operands
        // for `⊗`, itself to the left one for Mix.
        let (left_own, right_own) = if rule.rule == Rule::Mix {
            (vec![o], vec![])
        } else {
            let operands = (f.left(o), f.right(o));
            (
                operands.0.into_iter().collect(),
                operands.1.into_iter().collect(),
            )
        };
        let count = |list: &[OccId]| {
            let outputs = list.iter().filter(|&&x| output(x)).count();
            (outputs, list.len() - outputs)
        };
        let (left_out, left_in) = count(&left_own);
        let (right_out, right_in) = count(&right_own);
        let outputs: Vec<bool> = round.iter().map(|&p| output(sequent[p])).collect();
        let total_out = outputs.iter().filter(|&&b| b).count();
        let total_in = outputs.len() - total_out;
        let empty_allowed = self.mode.allows_empty_antecedents();
        // Cut point `k`: the first `k` formulas round go right, the rest
        // left; `k` = all of them is the step without a split.
        let mut prefix_out = 0;
        let mut allowed = None;
        for k in 0..=round.len() {
            if k > 0 && outputs[k - 1] {
                prefix_out += 1;
            }
            let prefix_in = k - prefix_out;
            let (l_out, l_in) = (
                total_out - prefix_out + left_out,
                total_in - prefix_in + left_in,
            );
            let (r_out, r_in) = (prefix_out + right_out, prefix_in + right_in);
            let fits = (rule.rule != Rule::Mix || r_out + r_in > 0)
                && (reading.is_none()
                    || (l_out == 1 && r_out == 1 && (empty_allowed || (l_in > 0 && r_in > 0))));
            if fits {
                allowed = Some(k);
            }
        }
        let step = Step::new(position, rule);
        match allowed {
            Some(k) if k < round.len() => step.left(&round[k..]),
            Some(_) | None => step,
        }
    }

    /// Applies a step to the open goal and returns the goals it opens, in
    /// the order of the rule's premises. A rule that closes the goal (the
    /// axiom, `1`, `⊤`) opens none.
    ///
    /// Only `⊗` and Mix take a split ([`Step::left`]). In intuitionistic
    /// mode the rule may be given by its classical or its two-sided name,
    /// and the session records the two-sided one. A step that fails changes
    /// nothing.
    ///
    /// With quantifiers:
    /// - `∀R` and `∃L` (`∀` one-sided) open the body with a fresh
    ///   eigenvariable.
    /// - `∃R` and `∀L` (`∃` one-sided) open it with the step's witness, or
    ///   with a metavariable when the witness is open. A later axiom or
    ///   [`close`](Self::close) binds the metavariable, and
    ///   [`undo`](Self::undo) of that step unbinds it.
    /// - The axiom unifies its two literals' terms, binding open witnesses,
    ///   with the occurs check and the eigenvariable condition: an open
    ///   witness never takes an eigenvariable made after it.
    ///
    /// # Errors
    ///
    /// A [`StepError`] that names what the step needed: an open goal, a
    /// formula at the position, a rule that acts on it in the mode, a split
    /// that fits, the dual of an axiom, a context of `?` formulas for `!`
    /// and none for `1`, a witness only where the rule takes one, a term of
    /// the sequent's symbols and the eigenvariables in scope.
    pub fn apply(&mut self, goal: GoalId, step: &Step) -> Result<Vec<GoalId>, StepError> {
        let sequent = self.open(goal)?;
        let (position, rule) = (step.position, step.rule);
        let left = step.left_positions(sequent.len())?;
        let left = &left[..];
        let reading = self.reading();
        let (rule, premises) = {
            let o = formula_at(&sequent, position)?;
            // The goal's member: in a session with quantifiers, an
            // instance, which the refusal writes with its terms.
            let member = self.goal(goal)?[position];
            let rule = match &reading {
                Some(reading) => {
                    let named = Named::applied(rule.rule, Some(reading), o);
                    if rule.side.is_some() && rule != named {
                        return Err(self.rule_refused(rule, position, member));
                    }
                    named
                }
                None => rule,
            };
            let premises = self
                .expand(reading.as_ref(), &sequent, position, rule, left)
                .map_err(|error| {
                    if let StepError::Rule { rule, position, .. } = &error
                        && self.fo.is_some()
                    {
                        self.rule_refused(*rule, *position, member)
                    } else {
                        error
                    }
                })?;
            (rule, premises)
        };
        if step.witness != Witness::Open && rule.rule != Rule::Exists {
            return Err(StepError::NoWitness { rule });
        }
        let premises = if self.fo.is_some() {
            self.framed_premises(goal, position, rule.rule, left, &step.witness)?
        } else {
            premises.into_iter().map(Multiset::into_members).collect()
        };
        Ok(self.record(goal, position, rule, premises))
    }

    /// Returns the premises of a rule on a goal of a first-order session
    /// as members, once the checks on occurrences have passed.
    ///
    /// The premises hold the children of the principal formula under its
    /// frame, and a binder's body under its frame extended by a fresh
    /// eigenvariable, the witness given or a fresh metavariable. The axiom
    /// has none and unifies its literals' terms. A step that fails, or
    /// would take the session past its bound on terms, goes back to its
    /// mark.
    pub(super) fn framed_premises(
        &mut self,
        goal: GoalId,
        position: usize,
        rule: Rule,
        left: &[usize],
        witness: &Witness,
    ) -> Result<Vec<Vec<Member>>, StepError> {
        // The number the step will have. It fits a `u32`: there are fewer
        // steps than inferences, which an `InfId` numbers.
        let step = self.step_number();
        let mark = self.fo.as_ref().expect("a first-order session").mark();
        let made = self
            .make_premises(goal, position, rule, left, witness)
            .and_then(|premises| {
                let q = self.fo.as_ref().expect("a first-order session");
                match q.limit {
                    Some(limit) if q.too_large() => Err(StepError::Witness {
                        fault: Box::new(WitnessFault::Table { limit }),
                    }),
                    _ => Ok(premises),
                }
            });
        let q = self.fo.as_mut().expect("a first-order session");
        match made {
            Ok(premises) => {
                q.made(step, mark);
                Ok(premises)
            }
            // A step that fails leaves the session as it found it.
            Err(error) => {
                q.back_to(mark);
                self.show();
                Err(error)
            }
        }
    }

    /// Makes the premises of [`framed_premises`](Self::framed_premises).
    fn make_premises(
        &mut self,
        goal: GoalId,
        position: usize,
        rule: Rule,
        left: &[usize],
        witness: &Witness,
    ) -> Result<Vec<Vec<Member>>, StepError> {
        use Rule::*;
        let members = self.goal(goal)?.to_vec();
        let m = members[position];
        // The number the step will have, under which its bindings are
        // trailed. It fits a `u32`: there are fewer steps than inferences,
        // which an `InfId` numbers.
        let step = self.step_number();
        if rule == Ax {
            let [x, y] = members[..] else {
                unreachable!("the occurrences' check holds two literals");
            };
            let forest = &self.forest;
            let q = self.fo.as_mut().expect("a first-order session");
            let lx = q.literal(forest, x);
            let ly = q.literal(forest, y);
            let no_dual = StepError::NoDual { position };
            if forest.kind(q.instance(x).0) == forest.kind(q.instance(y).0) {
                return Err(no_dual);
            }
            match (lx, ly) {
                (Some((p, a)), Some((r, b))) if p == r => {
                    let pairs = a.into_iter().zip(b).collect();
                    match q.unify(forest, pairs, step) {
                        Ok(()) => {}
                        Err(Clash::Terms) => return Err(no_dual),
                        Err(Clash::Eigen(a)) => {
                            let names =
                                eigen_names(&forest.sequent().fo, &q.shown.eigenvariables());
                            return Err(StepError::Eigenvariable {
                                name: names[&a].clone(),
                                cause: super::EigenCause::AfterWitness,
                            });
                        }
                    }
                }
                (None, None) if forest.atom(q.instance(x).0) == forest.atom(q.instance(y).0) => {}
                _ => return Err(no_dual),
            }
            self.show();
            return Ok(Vec::new());
        }
        let opened = match (rule, witness) {
            (Forall, _) => {
                let q = self.fo.as_mut().expect("a first-order session");
                let Some((_, a)) = q.fresh_eigen() else {
                    return Err(StepError::Witness {
                        fault: Box::new(WitnessFault::Eigenvariables),
                    });
                };
                Some(a)
            }
            (Exists, Witness::Open) => {
                let q = self.fo.as_mut().expect("a first-order session");
                Some(q.fresh_meta())
            }
            (Exists, Witness::Term(nodes)) => Some(self.witness_term(goal, nodes)?),
            _ => None,
        };
        let forest = &self.forest;
        let q = self.fo.as_mut().expect("a first-order session");
        let (o, frame) = q.instance(m);
        // The instances the premises name, made where they are new.
        for c in [forest.left(o), forest.right(o)].into_iter().flatten() {
            if !forest.kind(o).is_binder() {
                q.member(c, frame);
            }
        }
        let body = opened.map(|t| {
            let pushed = q.frame(frame, t);
            q.member(forest.left(o).expect("a binder's body"), Some(pushed))
        });
        let premises = self
            .mirror(&members, position, rule, left, body)
            .expect("the premises' instances were made");
        self.show();
        Ok(premises)
    }

    /// Returns the premises of a rule on a first-order session's goal as
    /// members: the goal's members with the principal formula's children
    /// under its frame, or the binder's `body`, in its place.
    ///
    /// Returns `None` where an instance they name was never made. The
    /// caller has passed the checks on occurrences.
    pub(super) fn mirror(
        &self,
        members: &[Member],
        position: usize,
        rule: Rule,
        left: &[usize],
        body: Option<Member>,
    ) -> Option<Vec<Vec<Member>>> {
        use Rule::*;
        let forest = &self.forest;
        let q = self.fo.as_ref().expect("a first-order session");
        let m = members[position];
        let (o, frame) = q.instance(m);
        let child = |c: Option<OccId>| q.find(c.expect("a child of the connective"), frame);
        let l = matches!(
            rule,
            Tensor | Par | With | PlusLeft | Promotion | Dereliction
        )
        .then(|| child(forest.left(o)))
        .map(|c| c.ok_or(()));
        let r = matches!(rule, Tensor | Par | With | PlusRight)
            .then(|| child(forest.right(o)))
            .map(|c| c.ok_or(()));
        let (l, r) = (l.transpose().ok()?, r.transpose().ok()?);
        let mut rest: Vec<Member> = members.to_vec();
        rest.remove(position);
        let mut going_left: Vec<Member> = left.iter().map(|&p| members[p]).collect();
        let mut going_right = Multiset::of(rest.iter().copied())
            .difference(&Multiset::of(going_left.iter().copied()))
            .as_slice()
            .to_vec();
        let with = |added: &[Option<Member>]| {
            let mut premise = rest.clone();
            premise.extend(added.iter().flatten());
            premise
        };
        let mut premises = match rule {
            Ax | One | Top => vec![],
            Tensor => {
                going_left.push(l?);
                going_right.push(r?);
                vec![going_left, going_right]
            }
            Mix => {
                going_left.push(m);
                vec![going_left, going_right]
            }
            Par => vec![with(&[l, r])],
            Bot | Weakening | AffineWeakening => vec![with(&[])],
            With => vec![with(&[l]), with(&[r])],
            PlusLeft | Promotion | Dereliction => vec![with(&[l])],
            PlusRight => vec![with(&[r])],
            Contraction => vec![with(&[Some(m), Some(m)])],
            Forall | Exists => vec![with(&[Some(body?)])],
            Cut | Open => unreachable!("no step of the rule reaches here"),
        };
        for premise in &mut premises {
            premise.sort_unstable();
        }
        Some(premises)
    }

    /// Remakes the shown instances of a first-order session after a
    /// change.
    pub(super) fn show(&mut self) {
        if let Some(q) = self.fo.as_mut() {
            q.show(&self.forest);
        }
    }

    /// Closes an open goal by a rule at a position, opens the premises
    /// [`expand`](Self::expand) computed as goals, records the step, and
    /// returns the new goals.
    pub(super) fn record(
        &mut self,
        goal: GoalId,
        position: usize,
        rule: Named,
        premises: Vec<Vec<Member>>,
    ) -> Vec<GoalId> {
        let principal = rule.rule.has_principal().then_some(position);
        let base = self.inferences.len() as u32;
        let ids: Vec<InfId> = (0..premises.len() as u32)
            .map(|i| InfId::new(base + i))
            .collect();
        for premise in premises {
            self.inferences.push(super::open_goal(premise));
        }
        let inference = &mut self.inferences[goal.index()];
        inference.rule = rule;
        inference.principal = principal;
        inference.premises = ids.clone();
        self.history.push(goal.inf());
        ids.into_iter().map(GoalId::of).collect()
    }

    /// Returns the premises of a rule on a sequent, in the rule's order, or
    /// refuses the rule.
    ///
    /// The rule must act on the formula's connective, be available in the
    /// mode, find the context it needs and take a valid split. In
    /// intuitionistic mode it must leave one formula on the right of `⊢` in
    /// every premise and never weaken that formula. These checks are
    /// complete for the checker: a derivation built only from steps that
    /// pass them always translates into a term the checker accepts.
    pub(super) fn expand(
        &self,
        reading: Option<&Reading>,
        sequent: &[OccId],
        position: usize,
        rule: Named,
        left: &[usize],
    ) -> Result<Vec<Multiset>, StepError> {
        use Rule::*;
        let f = &self.forest;
        let o = formula_at(sequent, position)?;
        let kind = f.kind(o);
        let classical = rule.rule;
        // The rule acts on the connective, and on the right side of ⊢.
        let acts = match classical {
            Ax => kind.is_literal(),
            Forall => kind == Kind::Forall,
            Exists => kind == Kind::Exists,
            Tensor => kind == Kind::Tensor,
            Par => kind == Kind::Par,
            One => kind == Kind::One,
            Bot => kind == Kind::Bot,
            With => kind == Kind::With,
            PlusLeft | PlusRight => kind == Kind::Plus,
            Top => kind == Kind::Top,
            Promotion => kind == Kind::Bang,
            Dereliction | Contraction | Weakening => kind == Kind::Quest,
            AffineWeakening => kind != Kind::Quest,
            Mix => true,
            // A cut is not a step on a formula of the goal: it is made by
            // naming its formula (`cut`).
            Cut | Open => false,
        };
        let named = Named::applied(classical, reading, o);
        if !acts || rule != named {
            return Err(self.rule_refused(rule, position, Member::from(o)));
        }
        let allowed = match classical {
            AffineWeakening => self.mode.affine,
            Mix => self.mode.mix,
            // The ordered calculus is unit-free MLL.
            One | Bot | With | PlusLeft | PlusRight | Top | Promotion | Dereliction
            | Contraction | Weakening => !self.mode.is_ordered(),
            // A session in an ordered mode holds no binder.
            Ax | Tensor | Par | Cut | Open | Forall | Exists => true,
        };
        if !allowed {
            return Err(StepError::Mode {
                rule,
                mode: self.mode,
            });
        }
        let splits = matches!(classical, Tensor | Mix);
        if !splits && !left.is_empty() {
            return Err(StepError::NoSplit { rule });
        }
        // The context without the principal formula, and the part of it
        // going left.
        let mut rest = Multiset::of(sequent.iter().copied());
        rest.remove(o);
        let mut going_left = Multiset::new();
        if splits {
            let used = super::chosen(sequent.len(), Some(position), left)?;
            for &p in left {
                going_left.insert(sequent[p]);
            }
            // In an ordered mode each premise takes an arc: going round
            // from the formula, the right premise's context comes first,
            // then the left one's.
            if self.mode.is_ordered() {
                let round = (position + 1..sequent.len()).chain(0..position);
                let mut left_seen = None;
                for p in round {
                    match (used[p], left_seen) {
                        (true, None) => left_seen = Some(p),
                        (false, Some(first)) => {
                            return Err(StepError::Exchange { position: first });
                        }
                        _ => {}
                    }
                }
            }
        }
        let child = |side: Branch| match side {
            Branch::Left => f.left(o).unwrap(),
            Branch::Right => f.right(o).unwrap(),
        };
        let with = |added: &[OccId]| {
            let mut premise = rest.clone();
            for &a in added {
                premise.insert(a);
            }
            premise
        };
        let premises = match classical {
            Forall | Exists => vec![with(&[child(Branch::Left)])],
            Ax => {
                let [x, y] = sequent else {
                    return Err(StepError::NotAlone { rule, position });
                };
                let other = if position == 0 { *y } else { *x };
                // A first-order session's literals are instances, whose
                // terms `make_premises` unifies: here only their signs
                // are read.
                let dual = if self.fo.is_some() {
                    f.kind(other).is_literal() && f.kind(other) != kind
                } else {
                    f.dual_literals(o, other)
                };
                if !dual {
                    return Err(StepError::NoDual { position });
                }
                vec![]
            }
            One => {
                if sequent.len() != 1 {
                    return Err(StepError::NotAlone { rule, position });
                }
                vec![]
            }
            Top => vec![],
            Tensor => {
                let mut right = rest.difference(&going_left);
                going_left.insert(child(Branch::Left));
                right.insert(child(Branch::Right));
                vec![going_left, right]
            }
            Mix => {
                let right = rest.difference(&going_left);
                // No rule concludes the empty sequent.
                if right.is_empty() {
                    return Err(StepError::EmptyPremise);
                }
                going_left.insert(o);
                vec![going_left, right]
            }
            Par => vec![with(&[child(Branch::Left), child(Branch::Right)])],
            Bot => vec![with(&[])],
            With => vec![with(&[child(Branch::Left)]), with(&[child(Branch::Right)])],
            PlusLeft => vec![with(&[child(Branch::Left)])],
            PlusRight => vec![with(&[child(Branch::Right)])],
            Promotion => {
                if let Some(p) = sequent
                    .iter()
                    .enumerate()
                    .position(|(i, &x)| i != position && f.kind(x) != Kind::Quest)
                {
                    return Err(StepError::NotQuest { position: p });
                }
                vec![with(&[child(Branch::Left)])]
            }
            Dereliction => vec![with(&[child(Branch::Left)])],
            Contraction => vec![with(&[o, o])],
            Weakening | AffineWeakening => {
                if let Some(reading) = reading
                    && reading.position(o) == Side::Output
                {
                    return Err(StepError::Output { position });
                }
                vec![with(&[])]
            }
            Cut | Open => unreachable!("refused above: no step names the rule"),
        };
        if let Some(reading) = reading {
            for premise in &premises {
                let outputs = reading.outputs(premise.as_slice().iter().copied());
                if outputs != 1 {
                    return Err(StepError::Succedents { count: outputs });
                }
                if !self.mode.allows_empty_antecedents() && premise.as_slice().len() <= outputs {
                    return Err(StepError::EmptyAntecedent);
                }
            }
        }
        Ok(premises)
    }

    /// Returns whether the split of a `⊗` or Mix step passes the focused
    /// engine's count prunes: `false` means that this split cannot be
    /// closed; a split that passes may still fail. The step's rule is the
    /// one judged, and the step is refused as [`apply`](Self::apply)
    /// refuses it where the rule does not act on the formula.
    ///
    /// It builds the count rows of the whole forest, which can be
    /// quadratic in it, with no bound and no stop:
    /// [`split_passes_within`](Self::split_passes_within) is the bounded
    /// form.
    ///
    /// A forest past what the counts can index (2³¹ occurrences, or 2³²
    /// row entries) passes: the test can say nothing of it.
    ///
    /// # Errors
    ///
    /// As [`rules`](Self::rules) does for the goal and the position,
    /// [`StepError::NoSplit`] for a rule that makes no split, and the
    /// errors of `apply` for a rule that does not act on the formula.
    pub fn split_passes(&self, goal: GoalId, step: &Step) -> Result<bool, StepError> {
        match self.split_test(goal, step, &crate::memory::Account::new(None), &mut |_| {
            false
        }) {
            Ok(passes) => Ok(passes),
            Err(Split::Step(error)) => Err(error),
            // With no bound and no stop, only the counts' indices end it.
            Err(Split::Ended(_)) => Ok(true),
        }
    }

    /// Returns whether the split of a `⊗` or Mix step passes the count
    /// prunes, as [`split_passes`](Self::split_passes) does, within
    /// `limits` and until `stop` fires: the count rows are charged to
    /// `limits.memory_bytes`, and the pass asks `stop` ([`Phase::Search`])
    /// every 2¹⁶ occurrences or row entries visited, on a forest of any
    /// size. It counts no work: `limits.work` does not bound it.
    ///
    /// # Errors
    ///
    /// Those of `split_passes`, as [`Error::Step`], and
    /// [`Error::Refused`] where a bound or `stop` ended the test: no
    /// answer on the split.
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "the counts' pass ends at the stop, the memory or an index alone"
    )]
    pub fn split_passes_within(
        &self,
        goal: GoalId,
        step: &Step,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<bool, Error> {
        let account = crate::memory::Account::new(limits.memory_bytes);
        let mut polled = |_| stop(Progress::new(Phase::Search, 0, 0));
        match self.split_test(goal, step, &account, &mut polled) {
            Ok(passes) => Ok(passes),
            Err(Split::Step(error)) => Err(Error::Step(error)),
            Err(Split::Ended(reason)) => Err(Error::Refused(match reason {
                search::Reason::MemoryLimit { .. } => Refusal::Memory {
                    phase: Phase::Search,
                    limit_bytes: limits.memory_bytes.unwrap_or(u64::MAX),
                    needed_bytes: None,
                },
                // The counts index at most 2³² row entries; the count
                // is a number past it, as the refusal allows.
                search::Reason::IndexLimit => Refusal::Index {
                    what: crate::limits::Space::Occurrence,
                    count: u64::from(u32::MAX) + 1,
                    most: u64::from(u32::MAX),
                },
                search::Reason::Stopped => Refusal::stopped(Phase::Search),
                other => unreachable!("the counts' pass ended at {other:?}"),
            })),
        }
    }

    /// The body of [`split_passes`](Self::split_passes) and
    /// [`split_passes_within`](Self::split_passes_within): the step's
    /// premises made as `apply` makes them, then the counts' test of
    /// them, the rows charged to `account` and the pass asking `stop`.
    fn split_test(
        &self,
        goal: GoalId,
        step: &Step,
        account: &crate::memory::Account,
        stop: &mut dyn FnMut(u64) -> bool,
    ) -> Result<bool, Split> {
        let sequent = self.open(goal)?;
        let (position, rule) = (step.position, step.rule);
        if !matches!(rule.rule, Rule::Tensor | Rule::Mix) {
            return Err(StepError::NoSplit { rule }.into());
        }
        let left = step.left_positions(sequent.len())?;
        let left = &left[..];
        let o = formula_at(&sequent, position)?;
        let reading = self.reading();
        let rule = match &reading {
            Some(reading) => {
                let named = Named::applied(rule.rule, Some(reading), o);
                if rule.side.is_some() && rule != named {
                    let member = self.goal(goal)?[position];
                    return Err(self.rule_refused(rule, position, member).into());
                }
                named
            }
            None => rule,
        };
        let premises = self.expand(reading.as_ref(), &sequent, position, rule, left)?;
        let [l, r] = premises.as_slice() else {
            unreachable!("a split has two premises");
        };
        let fragment = search::goal_fragment(&self.forest, &sequent);
        search::counts::split_passes(
            &self.forest,
            fragment,
            self.mode,
            l.as_slice(),
            r.as_slice(),
            account,
            stop,
        )
        .map_err(Split::Ended)
    }

    /// Takes back the last step (a rule applied, a cut, or a goal closed by
    /// the search) and returns the goal it reopened, or `None` when there
    /// is no step to take back. The open witnesses the step bound are open
    /// again.
    pub fn undo(&mut self) -> Option<GoalId> {
        let goal = self.history.pop()?;
        // Steps only append, so the last step's inferences are the end of
        // the arena.
        let first = self.subtree(goal).into_iter().min();
        if let Some(first) = first {
            self.inferences.truncate(first.index());
        }
        let inference = &mut self.inferences[goal.index()];
        let cut = inference.rule.rule == Rule::Cut;
        inference.rule = Rule::Open.into();
        inference.principal = None;
        inference.premises.clear();
        inference.cut = None;
        // The step's number, under which its bindings were trailed. It
        // fits a `u32`: there are fewer steps than inferences, which an
        // `InfId` numbers.
        let step = self.step_number();
        if let Some(q) = self.fo.as_mut() {
            q.undo(step);
        }
        // Only after the members a cut on an instance made are gone, so
        // that no member left names the cut formula's trees.
        if cut {
            self.uncut();
        }
        self.show();
        Some(GoalId::of(goal))
    }

    /// Returns the inferences above `id`, excluding it, in no particular
    /// order.
    pub(super) fn subtree(&self, id: InfId) -> Vec<InfId> {
        crate::goal_tree::subtree(|i| &self.inferences[i.index()].premises[..], id)
    }

    /// Runs the search on an open goal, as
    /// [`prove_goal`](crate::linear::search::prove_goal) does, and returns
    /// what it did.
    ///
    /// When the search proves the goal, the derivation of its proof is
    /// grafted onto the goal as one step, which [`undo`](Self::undo) takes
    /// back whole. Otherwise nothing changes. The outcome's proof, if any,
    /// proves the goal alone.
    ///
    /// The search runs within `limits`, and the grafted derivation must fit
    /// them too. When it does not, or `stop` ends its unfolding, the goal
    /// stays open and [`Closed::grafted`] says why; the proof stays in the
    /// outcome, so a proved goal is never lost.
    ///
    /// # Errors
    ///
    /// [`Error::Step`] for a goal that is not open, and the errors of the
    /// search ([`prove_goal`](crate::linear::search::prove_goal)). A graft
    /// that the limits or `stop` refuse is no error: [`Closed::grafted`]
    /// holds it.
    pub fn close(
        &mut self,
        goal: GoalId,
        options: &Options,
        view: &ViewOptions,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Closed, Error> {
        if self.fo.is_some() {
            return self.close_framed(goal, options, view, limits, stop);
        }
        let members = self.goal(goal)?;
        let outcome = search::prove_goal(
            search::Goal::new(&self.forest, members)?,
            self.mode,
            options,
            limits,
            &mut stop,
        )?;
        let mut grafted = Ok(());
        if let Verdict::Proved(proof) = &outcome.verdict {
            grafted = super::graft_ended(self.close_with(goal, proof, view, limits, stop))?;
        }
        Ok(Closed { outcome, grafted })
    }

    /// Closes an open goal with a proof of it that the caller's own search
    /// found, as [`close`](Self::close) does with the proof its search
    /// finds. A caller that runs several searches side by side uses this.
    ///
    /// The proof must be over the session's sequent, and its root must
    /// conclude the goal, as a proof from [`prove_goal`] on the goal's
    /// members ([`goal`](Self::goal)) does. It must pass the checker in
    /// the session's mode. Its derivation is grafted within `limits` as one
    /// step.
    ///
    /// [`prove_goal`]: crate::linear::search::prove_goal
    ///
    /// # Errors
    ///
    /// [`Error::Step`] for a goal that is not open, [`Error::ForeignProof`]
    /// for a proof of another sequent, [`Error::GoalMismatch`] for one of
    /// another goal, [`Error::Check`] with [`CheckError::Invalid`](crate::linear::proofs::CheckError::Invalid) for one
    /// the checker rejects in the session's mode; and, leaving the goal
    /// open, [`Error::Check`] with [`CheckError::Refused`](crate::linear::proofs::CheckError::Refused) when a bound or
    /// `stop` ends the check, and [`Error::Refused`] when they refuse the
    /// graft. [`Error::is_refusal`] tells the refusals from the rejection.
    pub fn close_with(
        &mut self,
        goal: GoalId,
        proof: &Proof,
        view: &ViewOptions,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let sequent = self.open(goal)?;
        // The goal's ids name occurrences of the session's forest, which
        // the sequent determines, so an equal sequent means equal ids.
        if proof.sequent() != self.sequent() {
            return Err(Error::ForeignProof);
        }
        let mut concluded: Vec<OccId> = proof.conclusion();
        concluded.sort_unstable();
        if concluded != sequent {
            return Err(Error::GoalMismatch);
        }
        let found = Derivation::of_goal(proof, &sequent, self.mode, view, limits, &mut stop)?;
        self.graft(goal.inf(), found);
        self.history.push(goal.inf());
        Ok(())
    }

    /// Closes an open goal with a proof of an equal sequent made elsewhere,
    /// such as another session's.
    ///
    /// The proof is carried over to the goal ([`Proof::onto`]), its
    /// conclusions matched to the goal's formulas in any order. It then
    /// closes the goal as [`close_with`](Self::close_with) does: checked in
    /// the session's mode, as one step.
    ///
    /// # Errors
    ///
    /// [`Error::Step`] for a goal that is not open,
    /// [`Error::SequentMismatch`] for a proof whose conclusions are not the
    /// goal's formulas, and the errors of
    /// [`close_with`](Self::close_with). Among them is [`Error::Check`]
    /// with [`CheckError::Invalid`](crate::linear::proofs::CheckError::Invalid) for a proof the checker rejects in the
    /// session's mode, such as one that uses another mode's rules.
    pub fn fill(
        &mut self,
        goal: GoalId,
        proof: &Proof,
        view: &ViewOptions,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let members = self.goal(goal)?.to_vec();
        let carried = proof.onto(self.forest.clone(), &members)?;
        self.close_with(goal, &carried, view, limits, stop)
    }

    /// Runs the search on every open goal in turn, as [`close`](Self::close)
    /// does, and returns each goal with what closing it did.
    ///
    /// A goal the search does not prove, or whose closing fails, stays
    /// open, and the run goes on with the next. The limits bound each
    /// goal's search on its own.
    pub fn close_all(
        &mut self,
        options: &Options,
        view: &ViewOptions,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Vec<(GoalId, Result<Closed, Error>)> {
        let goals: Vec<GoalId> = self.goals().collect();
        goals
            .into_iter()
            .enumerate()
            .map(|(i, goal)| {
                // Each goal's polls name it by its place among the goals.
                let item = u32::try_from(i).unwrap_or(u32::MAX);
                let stop = |progress: Progress| stop(Progress { item, ..progress });
                (goal, self.close(goal, options, view, limits, stop))
            })
            .collect()
    }

    /// Replaces the open goal by the root of a derivation given premises
    /// before conclusions, and appends the rest in reverse, so that every
    /// premise has a larger index than its conclusion.
    pub(super) fn graft(&mut self, goal: InfId, mut found: Vec<Inference>) {
        let renumber = crate::goal_tree::grafted(goal, self.inferences.len(), found.len() - 1);
        for inference in &mut found {
            for premise in &mut inference.premises {
                *premise = renumber(*premise);
            }
        }
        let root = found.pop().unwrap();
        self.inferences[goal.index()] = root;
        self.inferences.extend(found.into_iter().rev());
    }
}

/// Returns the formula at a position of a sequent, or the refusal.
pub(super) fn formula_at(sequent: &[OccId], position: usize) -> Result<OccId, StepError> {
    sequent.get(position).copied().ok_or(StepError::NoFormula {
        position,
        len: sequent.len(),
    })
}

/// Why the split test gave no answer: the step's fault, or the reason the
/// counts' pass ended.
enum Split {
    /// The step is refused.
    Step(StepError),
    /// A bound or the stop ended the pass.
    Ended(search::Reason),
}

impl From<StepError> for Split {
    fn from(error: StepError) -> Self {
        Self::Step(error)
    }
}
