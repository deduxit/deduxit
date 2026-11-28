// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Witnesses: the open witnesses an axiom fills by unification, the
//! eigenvariables, and the quantifier steps that take them.

use super::{
    BindingFault, Context, Interactive, Step, StepError, axiom, formula_at, premise, shown,
};
use crate::first_order::{Eigen, FoTerm, FoTermId};
use crate::limits::Refusal;
use crate::linear::proofs::{GoalId, InfId};
use crate::ordinary::derivation::check_inference;
use crate::ordinary::instance::Resolution;
use crate::ordinary::translate::Extra;
use crate::ordinary::{Formulas, Inference, Logic, Node, NodeId, Rule, Side};
use crate::unify::{Clash, Unifier};
use crate::{Error, WitnessFault};

/// Returns the refusal of a bound that building formulas or terms of the
/// proof's own arena met: nothing else fails there, since every id the
/// steps build from is the arena's and every witness is closed.
fn bound_met(error: Error) -> Refusal {
    let Error::Refused(refusal) = error else {
        unreachable!("building from the proof's own arena failed: {error}")
    };
    refusal
}

/// An inference's sides and term as the step that made it left them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Made {
    /// The formulas left of `⊢`.
    pub(super) left: Vec<NodeId>,
    /// The formulas right of `⊢`.
    pub(super) right: Vec<NodeId>,
    /// The term of a quantifier rule.
    pub(super) term: Option<FoTermId>,
}

impl Made {
    /// Returns the sides and term of `inference`.
    pub(super) fn of(inference: &Inference) -> Self {
        Self {
            left: inference.left.clone(),
            right: inference.right.clone(),
            term: inference.term,
        }
    }

    /// Returns whether `inference` has these sides and this term.
    pub(super) fn shows(&self, inference: &Inference) -> bool {
        self.left == inference.left && self.right == inference.right && self.term == inference.term
    }
}

/// The open witnesses of a proof in progress: the witnesses of `∀L` and
/// `∃R` steps taken without one, which an axiom fills by unification.
///
/// The inferences show every binding in force: an inference that holds a
/// bound witness holds its term in its place, so the checker's rules read
/// them as they read any inference. What an inference was made as, before
/// a binding changed it, is kept beside, so that an undo shows it again.
#[derive(Clone, Debug, Default)]
pub(super) struct Witnesses {
    /// The open witnesses by number, their levels and bindings, and the
    /// trail of bindings by step. A witness's level is one more than the
    /// largest eigenvariable in its step's scope, so a binding takes no
    /// eigenvariable made after it.
    pub(super) unifier: Unifier,
    /// The step that made each open witness, by number.
    pub(super) made: Vec<u32>,
    /// Per inference, its sides and term as made, where a binding changed
    /// them since; as long as the inferences.
    pub(super) raw: Vec<Option<Made>>,
    /// Per open witness, the inferences registered as showing it: every
    /// inference that showed it since it was made and still stands, so
    /// every one that shows it now, and every one a binding of it changed.
    /// A binding is shown, and taken back, through these alone.
    pub(super) holders: Vec<crate::hash::HashSet<u32>>,
    /// Per inference, the open witnesses it is registered under; as long
    /// as the inferences.
    pub(super) registered: Vec<Vec<u32>>,
}

impl Witnesses {
    /// Returns no witnesses for a state of `len` inferences.
    pub(super) fn new(len: usize) -> Self {
        Self {
            raw: vec![None; len],
            registered: vec![Vec::new(); len],
            ..Self::default()
        }
    }

    /// Makes an unbound open witness of term `term` at `level`, made by the
    /// step numbered `step`.
    pub(super) fn push(&mut self, term: FoTermId, level: u32, step: u32) {
        self.unifier.push_meta(term, level);
        self.made.push(step);
        self.holders.push(crate::hash::HashSet::default());
    }

    /// Registers the inference `i` under each of the open witnesses
    /// `metas` that it is not registered under yet.
    pub(super) fn register(&mut self, i: usize, metas: &[u32]) {
        // Fewer inferences than a `u32` counts.
        let id = i as u32;
        for &k in metas {
            if let Some(holders) = self.holders.get_mut(k as usize)
                && holders.insert(id)
            {
                self.registered[i].push(k);
            }
        }
    }

    /// Forgets the registrations of the inferences from `len` on, which an
    /// undo took away.
    pub(super) fn forget_from(&mut self, len: usize) {
        for (i, metas) in self.registered.iter().enumerate().skip(len) {
            for &k in metas {
                if let Some(holders) = self.holders.get_mut(k as usize) {
                    // Fewer inferences than a `u32` counts.
                    holders.remove(&(i as u32));
                }
            }
        }
        self.registered.truncate(len);
        self.raw.truncate(len);
    }

    /// Returns the inferences registered under any of the open witnesses
    /// `metas`, ascending.
    pub(super) fn holding(&self, metas: impl Iterator<Item = u32>) -> Vec<usize> {
        let mut found: Vec<usize> = metas
            .filter_map(|k| self.holders.get(k as usize))
            .flatten()
            .map(|&i| i as usize)
            .collect();
        found.sort_unstable();
        found.dedup();
        found
    }
}

/// Why an axiom's formulas do not unify as a session may bind them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Unbound {
    /// The formulas differ in a connective, a predicate or a symbol, or a
    /// witness would occur in its own term.
    Differ,
    /// A witness would take this eigenvariable, made after it or on
    /// another branch.
    Eigen(Eigen),
    /// The proof's terms would unfold to this many symbols written out,
    /// past the bound on terms.
    Large(u64),
}

/// Unifies the formulas `left` and `right` of an axiom in `formulas`,
/// binding open witnesses under `step`. A binding takes no eigenvariable at
/// or above its witness's level and none that `in_scope` refuses, and no
/// term that unfolds to more than `most` symbols. With `arena`, the terms
/// of the arena once the bindings are shown must unfold to at most `most`
/// symbols too, by the measure a document's reader takes
/// ([`arena_terms`]): the session asks it, so that it writes no document
/// its reader refuses; the reader, whose document that measure has
/// bounded already, does not, since its arena is the last step's. On a
/// refusal nothing is left bound.
pub(super) fn unify_axiom(
    formulas: &Formulas,
    unifier: &mut Unifier,
    (left, right): (NodeId, NodeId),
    step: u32,
    in_scope: &impl Fn(FoTermId) -> bool,
    most: u64,
    arena: bool,
) -> Result<(), Unbound> {
    let pairs = formulas.term_pairs(left, right).ok_or(Unbound::Differ)?;
    let terms = formulas.first_order();
    let was: Vec<bool> = unifier.metas().iter().map(|m| m.bound.is_some()).collect();
    let mark = unifier.trail_len();
    match unifier.unify(terms, pairs, step, &|_| false) {
        Ok(()) => {}
        Err(Clash::Eigen(e)) => return Err(Unbound::Eigen(e)),
        Err(Clash::Terms) => return Err(Unbound::Differ),
    }
    // Each new binding's term written out, its bindings followed: shared
    // terms could stand for one exponentially long.
    let mut sizes: crate::hash::HashMap<FoTermId, u64> = crate::hash::HashMap::default();
    let new: Vec<FoTermId> = unifier
        .metas()
        .iter()
        .zip(&was)
        .filter(|&(_, &was)| !was)
        .filter_map(|(m, _)| m.bound)
        .collect();
    let mut largest = 0;
    for &t in &new {
        largest = largest.max(unfolded(terms, unifier, t, &mut sizes));
    }
    if largest > most {
        unifier.undo_to(mark);
        return Err(Unbound::Large(largest));
    }
    let mut total = if arena {
        arena_terms(terms).max(largest)
    } else {
        0
    };
    // Each atom that holds a witness bound now makes at most one atom more,
    // its arguments resolved: what showing the bindings builds.
    let fresh: Vec<bool> = unifier
        .metas()
        .iter()
        .zip(&was)
        .map(|(m, &was)| !was && m.bound.is_some())
        .collect();
    let mut holds = vec![false; terms.terms.len()];
    for (n, term) in terms.terms.iter().enumerate() {
        holds[n] = match *term {
            FoTerm::Meta(k) => fresh.get(k as usize).copied().unwrap_or(false),
            FoTerm::App(_, args) => terms.list(args).iter().any(|a| holds[a.index()]),
            FoTerm::Bound(_) | FoTerm::Eigen(_) => false,
        };
    }
    for &(_, args) in terms.atoms.iter().filter(|_| arena) {
        let list = terms.list(args);
        if list.iter().any(|a| holds[a.index()]) {
            for &a in list {
                total = total.saturating_add(unfolded(terms, unifier, a, &mut sizes));
            }
        }
    }
    if total > most {
        unifier.undo_to(mark);
        return Err(Unbound::Large(total));
    }
    // Every eigenvariable a new binding names, its bindings followed.
    let mut seen = crate::hash::HashSet::default();
    let mut stack = new;
    while let Some(u) = stack.pop() {
        let u = unifier.resolve(terms, u);
        if !seen.insert(u) {
            continue;
        }
        match terms.terms[u.index()] {
            FoTerm::App(_, args) => stack.extend_from_slice(terms.list(args)),
            FoTerm::Eigen(e) if !in_scope(u) => {
                unifier.undo_to(mark);
                return Err(Unbound::Eigen(e));
            }
            FoTerm::Eigen(_) | FoTerm::Meta(_) | FoTerm::Bound(_) => {}
        }
    }
    Ok(())
}

/// Returns the measure of an arena's terms that a document's reader holds
/// to `limits.terms`: the terms the atoms' arguments unfold to, summed over
/// the atoms, or the largest term where it is larger.
pub(super) fn arena_terms(terms: &crate::first_order::FirstOrder<NodeId>) -> u64 {
    let unfolded = terms.unfolded();
    let largest = unfolded.iter().copied().max().unwrap_or(0);
    terms
        .atoms
        .iter()
        .fold(0u64, |sum, &(_, args)| {
            terms
                .list(args)
                .iter()
                .fold(sum, |sum, t| sum.saturating_add(unfolded[t.index()]))
        })
        .max(largest)
}

/// Returns how many symbols the term `t` has written out, its bound
/// witnesses replaced by their terms, saturating; `sizes` keeps what was
/// counted. It walks on a stack of its own, each shared term once.
fn unfolded(
    terms: &crate::first_order::FirstOrder<NodeId>,
    unifier: &Unifier,
    t: FoTermId,
    sizes: &mut crate::hash::HashMap<FoTermId, u64>,
) -> u64 {
    let mut stack = vec![(t, false)];
    while let Some((u, done)) = stack.pop() {
        if sizes.contains_key(&u) {
            continue;
        }
        // A witness an undone step made stays in the arena's atoms with a
        // number past the live ones: it is a leaf.
        let r = match terms.terms[u.index()] {
            FoTerm::Meta(k) if k as usize >= unifier.metas().len() => u,
            FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Eigen(_) | FoTerm::Meta(_) => {
                unifier.resolve(terms, u)
            }
        };
        let parts: &[FoTermId] = match terms.terms[r.index()] {
            FoTerm::App(_, args) => terms.list(args),
            FoTerm::Bound(_) | FoTerm::Eigen(_) | FoTerm::Meta(_) => &[],
        };
        if done {
            let size = parts.iter().fold(1u64, |sum, p| {
                sum.saturating_add(sizes.get(p).copied().unwrap_or(0))
            });
            sizes.insert(u, size);
        } else {
            stack.push((u, true));
            stack.extend(parts.iter().map(|&p| (p, false)));
        }
    }
    sizes.get(&t).copied().unwrap_or(1)
}

/// Adds both implications of every equivalence of the arena from node
/// `from` on, which `shape` and the checker look up: a binding or an
/// instance can build an equivalence the arena lacked. An implication
/// added is no equivalence, so one pass suffices.
pub(super) fn implications_from(formulas: &mut Formulas, from: usize) -> Result<(), Error> {
    let mut n = from;
    while n < formulas.len() {
        // As many nodes as the arena's ids number.
        if let Node::Iff(a, b) = formulas.node(NodeId(n as u32)) {
            formulas.add(Node::Implies(a, b))?;
            formulas.add(Node::Implies(b, a))?;
        }
        n += 1;
    }
    Ok(())
}

/// A witness of the ordinary session read from text: its names resolved
/// in the sequent's symbols and the eigenvariables in the goal's scope,
/// its terms made in the session's arena.
struct Reading<'a> {
    /// The session.
    state: &'a mut Interactive,
    /// Every eigenvariable of the arena with the name it is written by.
    eigens: &'a [(u32, String)],
    /// The eigenvariables in the goal's scope.
    scope: &'a [FoTermId],
}

impl crate::witness::Witnesses for Reading<'_> {
    type Term = FoTermId;
    type Function = String;

    const BRACKET_AFTER_TERM: bool = false;

    fn name(
        &mut self,
        word: &str,
        applied: bool,
    ) -> Result<crate::witness::Name<FoTermId, String>, WitnessFault> {
        use crate::witness::Name;
        if let Some(&(k, _)) = self.eigens.iter().find(|(_, n)| n == word) {
            if applied {
                return Err(WitnessFault::Applied { name: word.into() });
            }
            let term = self
                .state
                .formulas
                .interner()
                .term(FoTerm::Eigen(Eigen::new(k)));
            if !self.scope.contains(&term) {
                return Err(WitnessFault::OutOfScope { name: word.into() });
            }
            Ok(Name::Term(term))
        } else if applied {
            Ok(Name::Applied(word.to_owned()))
        } else {
            self.state.applied(word, &[]).map(Name::Term)
        }
    }

    fn apply(&mut self, f: String, args: Vec<FoTermId>) -> Result<FoTermId, WitnessFault> {
        self.state.applied(&f, &args)
    }
}

impl Interactive {
    /// Closes the goal by the axiom on the formula the step names and the
    /// first formula on the other side of `⊢` that unifies with it, where
    /// none is equal to it: the open witnesses take the unifier's terms,
    /// every inference that holds them shows those terms, and the axiom's
    /// weakenings and `A ⊢ A` are made as [`axiom`] makes them. Returns the
    /// goals opened, none, or `None` where a formula on the other side is
    /// equal to it, which needs no binding.
    pub(super) fn unifying_axiom(
        &mut self,
        id: GoalId,
        step: &Step,
    ) -> Result<Option<Vec<GoalId>>, StepError> {
        let (side, position) = (step.side, step.position);
        let goal = self.open(id)?;
        let x = formula_at(goal, side, position)?;
        if step.context != Context::Shared || goal.side(side.flip()).contains(&x) {
            return Ok(None);
        }
        let len = goal.left.len() + goal.right.len();
        self.run_within(len, len)?;
        let goal = goal.clone();
        let scope = self.scope(InfId::new(id.get()));
        // The step's number, under which its bindings are trailed. Fewer
        // steps than inferences, which a `u32` counts.
        let number = self.history.len() as u32;
        let Some(w) = self.witnesses.as_deref_mut() else {
            return Ok(None);
        };
        let mark = w.unifier.trail_len();
        let was: Vec<bool> = w
            .unifier
            .metas()
            .iter()
            .map(|m| m.bound.is_some())
            .collect();
        let mut refused = Unbound::Differ;
        let mut partner = None;
        for (at, &y) in goal.side(side.flip()).iter().enumerate() {
            let pair = match side {
                Side::Left => (x, y),
                Side::Right => (y, x),
            };
            let in_scope = |e| scope.contains(&e);
            match unify_axiom(
                &self.formulas,
                &mut w.unifier,
                pair,
                number,
                &in_scope,
                self.terms,
                true,
            ) {
                Ok(()) => {
                    partner = Some(at);
                    break;
                }
                Err(Unbound::Differ) => {}
                Err(why) if refused == Unbound::Differ => refused = why,
                Err(_) => {}
            }
        }
        let Some(partner) = partner else {
            return Err(match refused {
                Unbound::Eigen(e) => {
                    let name = self.eigen_name(e);
                    StepError::Binding {
                        fault: Box::new(BindingFault::Eigenvariable { name }),
                    }
                }
                Unbound::Large(size) => StepError::Terms {
                    terms: size,
                    limit: self.terms,
                },
                Unbound::Differ => StepError::NoPartner { side, position },
            });
        };
        let made = axiom(&goal, side, position, partner);
        let fresh: Vec<u32> = self.witnesses.as_deref().map_or_else(Vec::new, |w| {
            (0..w.unifier.next_meta())
                .filter(|&k| w.unifier.metas()[k as usize].bound.is_some() && !was[k as usize])
                .collect()
        });
        match self.bind(&made, &fresh) {
            Ok((shown, raw)) => {
                let opened = self.record_made(id, shown, raw);
                Ok(Some(opened))
            }
            Err(error) => {
                if let Some(w) = self.witnesses.as_deref_mut() {
                    w.unifier.undo_to(mark);
                }
                Err(error)
            }
        }
    }

    /// Shows the bindings the unifier holds now in every inference that
    /// shows one of the open witnesses `fresh`, which the step bound, and
    /// returns the inferences `made` as they show them, with what each was
    /// made as where that differs. Only the inferences registered under
    /// `fresh` can change. Every inference that changes is checked by the
    /// checker's rules first; one that a binding would break (an
    /// eigenvariable brought into its rule's conclusion) refuses the
    /// binding and changes nothing.
    fn bind(
        &mut self,
        made: &[Inference],
        fresh: &[u32],
    ) -> Result<(Vec<Inference>, Vec<Option<Made>>), StepError> {
        let Some(w) = self.witnesses.as_deref_mut() else {
            return Ok((made.to_vec(), vec![None; made.len()]));
        };
        let bindings = w.unifier.metas().iter().map(|m| m.bound).collect();
        let mut resolution = Resolution::new(bindings);
        let full = |error: Error| StepError::Binding {
            fault: Box::new(BindingFault::Refused {
                refusal: bound_met(error),
            }),
        };
        let built = self.formulas.len();
        let mut changed = crate::hash::HashMap::default();
        for i in w.holding(fresh.iter().copied()) {
            let inference = &self.inferences[i];
            let shown = shown(&mut resolution, &mut self.formulas, inference).map_err(full)?;
            if shown != *inference {
                changed.insert(i, shown);
            }
        }
        let shown_made = made
            .iter()
            .map(|i| shown(&mut resolution, &mut self.formulas, i))
            .collect::<Result<Vec<_>, _>>()
            .map_err(full)?;
        implications_from(&mut self.formulas, built).map_err(full)?;
        let at = |i: InfId| {
            changed
                .get(&i.index())
                .unwrap_or(&self.inferences[i.index()])
        };
        for inference in changed.values().filter(|i| i.rule != Rule::Open) {
            let premises: Vec<&Inference> = inference.premises.iter().map(|&p| at(p)).collect();
            if check_inference(&self.formulas, self.logic, inference, &premises).is_err() {
                return Err(StepError::Binding {
                    fault: Box::new(BindingFault::Breaks),
                });
            }
        }
        let raw = made
            .iter()
            .zip(&shown_made)
            .map(|(m, s)| (m != s).then(|| Made::of(m)))
            .collect();
        let shown_made = self.checked(shown_made);
        let w = self
            .witnesses
            .as_deref_mut()
            .expect("the witnesses are there");
        for (i, inference) in changed {
            w.raw[i].get_or_insert_with(|| Made::of(&self.inferences[i]));
            let metas = self.formulas.metas_in(
                inference.left.iter().chain(&inference.right).copied(),
                inference.term.into_iter(),
            );
            w.register(i, &metas);
            let at = &mut self.inferences[i];
            at.left = inference.left;
            at.right = inference.right;
            at.term = inference.term;
        }
        Ok((shown_made, raw))
    }

    /// Returns the name an eigenvariable is printed with.
    fn eigen_name(&self, e: Eigen) -> String {
        self.formulas
            .eigen_names()
            .iter()
            .find(|&&(k, _)| k == e.get())
            .map_or_else(|| format!("#{}", e.get()), |(_, name)| name.clone())
    }

    /// Returns the inferences a step of a quantifier rule makes: the goal's
    /// own, with the step's witness or a fresh eigenvariable as its term,
    /// and its premise, open, which holds the instance of the quantified
    /// formula's body in its place (beside it for `∀L`, and for `∃R` in
    /// LK). The instance is built in the arena where it is new.
    pub(super) fn instance_step(
        &mut self,
        id: GoalId,
        step: &Step,
    ) -> Result<Vec<Inference>, StepError> {
        let (side, position, rule) = (step.side, step.position, step.rule);
        let x = formula_at(self.open(id)?, side, position)?;
        self.shape(rule, side, position, x)?;
        if step.context != Context::Shared {
            return Err(StepError::NoSplit { rule });
        }
        let (Node::Forall(body) | Node::Exists(body)) = self.formulas.node(x) else {
            unreachable!("{rule} fits a quantified formula alone")
        };
        let eigenvariable = matches!(rule, Rule::ForallRight | Rule::ExistsLeft);
        // `∀L` keeps its principal, and `∃R` where the right side has room.
        let keep = !eigenvariable && (rule == Rule::ForallLeft || self.logic == Logic::Classical);
        // An open witness's number and level, made once the instance is.
        let mut open = None;
        let term = match step.witness {
            Some(_) if eigenvariable => return Err(StepError::NoWitness { rule }),
            None if eigenvariable => self.fresh_eigenvariable()?,
            Some(t) => self.held(id, t)?,
            None => {
                let (term, level) = self.open_witness(id)?;
                open = Some(level);
                term
            }
        };
        self.instance_within(body, term)?;
        let instance =
            self.formulas
                .instantiate(body, term)
                .map_err(|error| StepError::Witness {
                    fault: Box::new(WitnessFault::Refused {
                        refusal: bound_met(error),
                    }),
                })?;
        // An instance may hold an equivalence the arena did not have, whose
        // implications `shape` looks up.
        Extra::add(&mut self.formulas).map_err(|error| StepError::Witness {
            fault: Box::new(WitnessFault::Refused {
                refusal: bound_met(error),
            }),
        })?;
        if let Some(level) = open {
            let (len, number) = (self.inferences.len(), self.history.len() as u32);
            let w = self
                .witnesses
                .get_or_insert_with(|| Box::new(Witnesses::new(len)));
            w.push(term, level, number);
        }
        let goal = self.open(id)?;
        let same = if keep {
            vec![instance, x]
        } else {
            vec![instance]
        };
        let made = vec![
            Inference {
                term: Some(term),
                left: goal.left.clone(),
                right: goal.right.clone(),
                rule,
                principal: Some((side, position)),
                premises: vec![InfId::new(1)],
            },
            premise(goal, side, position, &same, &[], |_, _| true),
        ];
        Ok(self.checked(made))
    }

    /// Refuses the instance of `body` by `term` where the arena's terms
    /// would pass the state's bound on terms once it is built, by the
    /// measure a document's reader takes ([`arena_terms`]): each atom of
    /// `body` that holds a bound variable makes at most one atom more,
    /// every bound variable in it written as `term`. So a session writes no
    /// document its reader refuses. Nothing is built.
    fn instance_within(&self, body: NodeId, term: FoTermId) -> Result<(), StepError> {
        let tables = self.formulas.first_order();
        let unfolded = tables.unfolded();
        let witness = unfolded.get(term.index()).copied().unwrap_or(1);
        // Each term written out with its bound variables as the witness;
        // a term comes after its arguments.
        let mut sizes = vec![0u64; tables.terms.len()];
        for (n, t) in tables.terms.iter().enumerate() {
            sizes[n] = match *t {
                FoTerm::Bound(_) => witness,
                FoTerm::Eigen(_) | FoTerm::Meta(_) => 1,
                FoTerm::App(_, args) => tables
                    .list(args)
                    .iter()
                    .fold(1u64, |sum, a| sum.saturating_add(sizes[a.index()])),
            };
        }
        let loose = tables.loose();
        let mut added = 0u64;
        let mut seen = crate::hash::HashSet::default();
        let mut stack = vec![body];
        while let Some(n) = stack.pop() {
            if !seen.insert(n) {
                continue;
            }
            match self.formulas.node(n) {
                Node::Atom(a) => {
                    let Some(&(_, args)) = tables.atoms.get(a as usize) else {
                        continue;
                    };
                    let list = tables.list(args);
                    if list.iter().any(|t| loose[t.index()] > 0) {
                        for t in list {
                            added = added.saturating_add(sizes[t.index()]);
                        }
                    }
                }
                Node::True | Node::False => {}
                Node::Not(a) | Node::Forall(a) | Node::Exists(a) => stack.push(a),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    stack.push(a);
                    stack.push(b);
                }
            }
        }
        let total = arena_terms(tables).max(witness).saturating_add(added);
        if total > self.terms {
            return Err(StepError::Terms {
                terms: total,
                limit: self.terms,
            });
        }
        Ok(())
    }

    /// Returns the term of the next open witness, `$1` for the first, and
    /// its level for a step on the goal `id`: one more than the largest
    /// eigenvariable in the goal's scope, so that its binding takes none
    /// made after it.
    fn open_witness(&mut self, id: GoalId) -> Result<(FoTermId, u32), StepError> {
        let fault = |fault: WitnessFault| StepError::Witness {
            fault: Box::new(fault),
        };
        let terms = &self.formulas.first_order().terms;
        let level = self
            .scope(InfId::new(id.get()))
            .iter()
            .filter_map(|t| match terms[t.index()] {
                FoTerm::Eigen(e) => Some(e.get()),
                FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => None,
            })
            .max()
            .map_or(Some(0), |e| e.checked_add(1))
            .ok_or_else(|| fault(WitnessFault::Eigenvariables))?;
        let k = self.witnesses.as_ref().map_or(0, |w| w.unifier.next_meta());
        if k == u32::MAX {
            return Err(fault(WitnessFault::Witnesses));
        }
        Ok((self.formulas.interner().term(FoTerm::Meta(k)), level))
    }

    /// Returns the number past every eigenvariable the proof's inferences'
    /// formulas and terms hold, `None` past `u32::MAX`.
    pub(super) fn next_eigenvariable(&self) -> Option<u32> {
        let roots = self
            .inferences
            .iter()
            .flat_map(|i| i.left.iter().chain(&i.right));
        let terms = self.inferences.iter().filter_map(|i| i.term);
        self.formulas
            .largest_eigenvariable(roots.copied(), terms)
            .map_or(Some(0), |k| k.checked_add(1))
    }

    /// Returns the term of an eigenvariable that no inference of the
    /// proof holds: one numbered past every eigenvariable its inferences'
    /// formulas and terms hold, so that taking back a step and taking it
    /// again names the same one.
    fn fresh_eigenvariable(&mut self) -> Result<FoTermId, StepError> {
        let next = self.next_eigenvariable().ok_or(StepError::Witness {
            fault: Box::new(WitnessFault::Eigenvariables),
        })?;
        Ok(self
            .formulas
            .interner()
            .term(FoTerm::Eigen(Eigen::new(next))))
    }

    /// Returns the witness `t` for a step on the goal `id` where it is a
    /// term of the goal: a term of the arena without a bound variable or
    /// an open witness, whose eigenvariables are in the goal's scope
    /// ([`scope`](Self::scope)).
    pub(super) fn held(&self, id: GoalId, t: FoTermId) -> Result<FoTermId, StepError> {
        let fault = |fault: WitnessFault| StepError::Witness {
            fault: Box::new(fault),
        };
        let terms = &self.formulas.first_order().terms;
        if t.index() >= terms.len() {
            return Err(fault(WitnessFault::NotATerm));
        }
        if !self.formulas.is_closed(t) || self.formulas.holds_meta(t) {
            return Err(fault(WitnessFault::Variable));
        }
        let scope = self.scope(InfId::new(id.get()));
        let mut seen = crate::hash::HashSet::default();
        let mut stack = vec![t];
        while let Some(u) = stack.pop() {
            if !seen.insert(u) {
                continue;
            }
            match terms[u.index()] {
                FoTerm::App(_, args) => {
                    stack.extend_from_slice(self.formulas.first_order().list(args))
                }
                FoTerm::Eigen(e) if !scope.contains(&u) => {
                    return Err(fault(WitnessFault::OutOfScope {
                        name: self.eigen_name(e),
                    }));
                }
                FoTerm::Eigen(_) | FoTerm::Bound(_) | FoTerm::Meta(_) => {}
            }
        }
        Ok(t)
    }

    /// Returns the eigenvariables in the scope of the inference `id`: those
    /// the `∀R` and `∃L` inferences below it introduce, on its path from
    /// the root.
    pub(super) fn scope(&self, id: InfId) -> Vec<FoTermId> {
        let mut parent = vec![None; self.inferences.len()];
        for (n, inference) in self.inferences.iter().enumerate() {
            for p in &inference.premises {
                parent[p.index()] = Some(n);
            }
        }
        let mut scope = Vec::new();
        let mut at = parent[id.index()];
        while let Some(n) = at {
            scope.extend(self.inferences[n].eigenvariable());
            at = parent[n];
        }
        scope
    }

    /// Reads a witness for a step on the goal `goal` from text: a term of
    /// the sequent's function symbols, written `f(t, …)` or by its name
    /// for a constant, and of the eigenvariables in the goal's scope by
    /// the names the proof prints them with (`a`, `b`, …, past the
    /// sequent's symbols). Returns its term, made in the proof's formulas
    /// where it is new, for [`Step::with_witness`].
    ///
    /// # Errors
    ///
    /// [`StepError::NoGoal`] for a goal that is not open, and
    /// [`StepError::Witness`] for text that is not such a term: an unknown
    /// name, a predicate, a symbol given the wrong number of arguments, an
    /// eigenvariable outside the goal's scope, or a stray bracket or comma.
    pub fn witness(&mut self, goal: GoalId, text: &str) -> Result<FoTermId, StepError> {
        self.open(goal)?;
        let fault = |fault: WitnessFault| StepError::Witness {
            fault: Box::new(fault),
        };
        let eigens = self.formulas.eigen_names().to_vec();
        let scope = self.scope(InfId::new(goal.get()));
        let tokens = crate::witness::tokens(text);
        // A term of text shares nothing, so it is as long as its names; one
        // past the bound on terms is refused before any of it is made.
        let names = tokens.iter().filter(|t| crate::witness::is_name(t)).count() as u64;
        if names > self.terms {
            return Err(fault(WitnessFault::Symbols {
                symbols: names,
                limit: self.terms,
            }));
        }
        let mut reading = Reading {
            state: self,
            eigens: &eigens,
            scope: &scope,
        };
        crate::witness::read(&mut reading, tokens).map_err(fault)
    }

    /// Returns the term of the sequent's function symbol `name` applied to
    /// `args`, or why there is none.
    fn applied(&mut self, name: &str, args: &[FoTermId]) -> Result<FoTermId, WitnessFault> {
        let tables = self.formulas.first_order();
        let Some(f) = tables.symbols.iter().position(|(s, _)| s == name) else {
            return Err(WitnessFault::UnknownSymbol { name: name.into() });
        };
        if tables.atoms.iter().any(|&(p, _)| p.index() == f) {
            return Err(WitnessFault::Predicate { name: name.into() });
        }
        let arity = tables.symbols[f].1;
        if args.len() != arity as usize {
            return Err(WitnessFault::Arity {
                name: name.into(),
                arity,
                given: Some(args.len()),
            });
        }
        let symbol = crate::first_order::Symbol::new(f as u32);
        let interner = self.formulas.interner();
        let args = interner.list(args);
        Ok(interner.term(FoTerm::App(symbol, args)))
    }
}
