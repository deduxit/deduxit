// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Building a derivation step by step in a calculus given as data: open
//! goals, the rules that apply to one, a rule applied backwards from its
//! conclusion, a goal closed by a hypothesis, and undo. Every step is
//! checked by the calculus's checker before the state takes it.

use super::calculus::{Calculus, Discipline, Entry, Find, Pattern, Rule};
use super::derivation::{Derivation, Node, ScopeId, Step};
use super::terms::{Term, TermId, Terms};
use crate::Error;
use crate::limits::{Limits, Refusal};

/// An open goal of a session, by its place in the session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GoalId(u32);

impl GoalId {
    /// Returns the goal at `index` of a session, which the session's own
    /// calls refuse where no open goal is there.
    pub const fn new(index: u32) -> Self {
        Self(index)
    }

    /// Returns the goal's place in the session.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// A rule to apply backwards to a goal, as [`Interactive::apply`] takes it.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Apply<'a> {
    /// The rule's name.
    pub rule: &'a str,
    /// The members of the goal the rule acts on, by their number across
    /// the goal's zones ([`Interactive::members`]), one for each position
    /// the rule asks of its conclusion ([`Calculus::positions`] counts
    /// them with the premises').
    pub at: &'a [u32],
    /// Values for what the goal leaves open, each a variable or a
    /// context of the rule by name and its text: a formula for a
    /// variable, members separated by `,` for a context.
    pub given: &'a [(&'a str, &'a str)],
}

impl<'a> Apply<'a> {
    /// Returns the step of `rule`, acting on nothing and given nothing.
    pub const fn new(rule: &'a str) -> Self {
        Self {
            rule,
            at: &[],
            given: &[],
        }
    }

    /// Returns the step acting on these members of the goal.
    #[must_use]
    pub const fn at(mut self, at: &'a [u32]) -> Self {
        self.at = at;
        self
    }

    /// Returns the step with these values given.
    #[must_use]
    pub const fn given(mut self, given: &'a [(&'a str, &'a str)]) -> Self {
        self.given = given;
        self
    }
}

/// A rule that applies to a goal, as [`Interactive::rules`] lists it.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Applicable {
    /// The rule's name.
    pub rule: String,
    /// The members of the goal it acts on, by their number.
    pub at: Vec<u32>,
    /// The variables and contexts the step needs given, by name: what the
    /// goal leaves open. Empty where the step can be taken as it is.
    pub needs: Vec<String>,
}

/// What a node of a session is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Done {
    /// An open goal.
    Open,
    /// Concluded by the rule of this name, with the positions it gives.
    Rule(String, Vec<u32>),
    /// A hypothesis of its scope.
    Hypothesis,
}

/// A node of a session, the root first and every premise after its
/// conclusion.
#[derive(Clone, Debug)]
pub(crate) struct Goal {
    /// The members of each zone.
    pub(crate) sequent: Vec<Vec<TermId>>,
    /// What it is.
    pub(crate) done: Done,
    /// Its premises, by their place in the session.
    pub(crate) premises: Vec<u32>,
    /// Its scope.
    pub(crate) scope: u32,
    /// The scope it was opened in, which a hypothesis leaves for the
    /// scope that discharges it.
    pub(crate) opened: u32,
}

/// A scope of a session.
#[derive(Clone, Debug)]
pub(crate) struct Scope {
    /// The scope around it; the root scope's is its own.
    pub(crate) parent: u32,
    /// The node and the premise of it that lie in this scope, the one
    /// rule that discharges its hypotheses; `None` for the root scope.
    pub(crate) owner: Option<(u32, u32)>,
    /// The hypotheses it offers: those the rule discharges, or for the
    /// root scope the session's assumptions.
    pub(crate) hypotheses: Vec<Vec<Vec<TermId>>>,
}

/// A session of step-by-step proving in a calculus given as data: a
/// derivation built from its conclusion up, with open goals.
///
/// [`rules`](Self::rules) lists the rules that apply to a goal,
/// [`apply`](Self::apply) applies one backwards, asking what the goal
/// leaves open (a formula only a premise holds, how a context splits),
/// [`assume`](Self::assume) closes a goal by a hypothesis in force, and
/// [`undo`](Self::undo) takes back the last step. The checker checks each
/// step as it is made, so no state holds a step the calculus refuses, and
/// [`finished`](Self::finished) checks the whole derivation: its verdict is
/// "valid in the calculus given". Nothing is searched.
///
/// # JSON
///
/// With the feature `serialize` a session is the document `{"version": 17,
/// "calculus": {…}, "terms": […], "scopes": […], "nodes": […], "history":
/// […]}` ([`wire`](crate::wire)): the rule file, the terms as a
/// derivation writes them, the scopes (each its `parent`, its `owner` node
/// and premise, and the `hypotheses` it offers; the first is the root's,
/// whose hypotheses are the assumptions), the nodes from the conclusion
/// up (each a derivation's node, its `premises` after it, `opened` where a
/// hypothesis left the scope it was opened in), and the goals the steps
/// closed, in order. Reading one checks every step taken.
#[derive(Clone, Debug)]
pub struct Interactive {
    /// The calculus.
    pub(crate) calculus: Calculus,
    /// The terms.
    pub(crate) terms: Terms,
    /// The scopes, the root scope first.
    pub(crate) scopes: Vec<Scope>,
    /// The nodes, the root first.
    pub(crate) nodes: Vec<Goal>,
    /// The goals the steps closed, in order.
    pub(crate) history: Vec<u32>,
    /// The bounds the session keeps to.
    pub(crate) limits: Limits,
}

/// The members of each zone of a sequent.
type Sequent = Vec<Vec<TermId>>;

/// Returns the error for a step the calculus does not allow.
fn refused(reason: impl Into<String>) -> Error {
    Error::StepRefused {
        reason: reason.into(),
    }
}

/// What a backward step makes of a goal: each premise's sequent and the
/// hypotheses it discharges, and the positions the node gives.
struct Plan {
    /// Per premise its sequent, and the hypotheses it discharges where
    /// the rule discharges any.
    premises: Vec<(Sequent, Option<Vec<Sequent>>)>,
    /// The positions the node gives its rule.
    at: Vec<u32>,
}

/// Why a backward step cannot be planned.
enum Unplanned {
    /// It needs these variables or contexts given.
    Needs(Vec<String>),
    /// It does not apply, for this reason, or a bound refused it.
    Error(Error),
}

impl From<Error> for Unplanned {
    fn from(error: Error) -> Self {
        Self::Error(error)
    }
}

impl Interactive {
    /// Returns a session that derives the sequent `goal` in `calculus`
    /// from the hypotheses `assumptions`, each a sequent of the calculus,
    /// both read as [`Calculus::parse_sequent`] reads one. The session
    /// keeps to `limits` at every step.
    ///
    /// # Errors
    ///
    /// [`Error::Parse`] for a text that is no sequent of the calculus,
    /// [`Error::StepRefused`] for a sequent whose zones break the
    /// calculus's bounds, and [`Refusal::Occurrences`] past
    /// `limits.occurrences`.
    pub fn within(
        calculus: Calculus,
        goal: &str,
        assumptions: &[&str],
        limits: &Limits,
    ) -> Result<Self, Error> {
        let mut terms = Terms::default();
        let sequent = calculus.parse_sequent(&mut terms, goal, limits)?;
        let mut hypotheses = Vec::with_capacity(assumptions.len());
        for text in assumptions {
            hypotheses.push(calculus.parse_sequent(&mut terms, text, limits)?);
        }
        let session = Self {
            calculus,
            terms,
            scopes: vec![Scope {
                parent: 0,
                owner: None,
                hypotheses,
            }],
            nodes: vec![Goal {
                sequent,
                done: Done::Open,
                premises: Vec::new(),
                scope: 0,
                opened: 0,
            }],
            history: Vec::new(),
            limits: *limits,
        };
        for sequent in
            std::iter::once(&session.nodes[0].sequent).chain(&session.scopes[0].hypotheses)
        {
            session.shaped(sequent)?;
        }
        Ok(session)
    }

    /// Returns the calculus.
    pub const fn calculus(&self) -> &Calculus {
        &self.calculus
    }

    /// Returns the terms the goals are made of.
    pub const fn terms(&self) -> &Terms {
        &self.terms
    }

    /// Returns the session's assumptions: the hypotheses the root scope
    /// offers, which the derivation may leave undischarged.
    pub fn assumptions(&self) -> &[Vec<Vec<TermId>>] {
        &self.scopes[0].hypotheses
    }

    /// Returns the open goals, in the order they were opened.
    pub fn goals(&self) -> Vec<GoalId> {
        (0..self.nodes.len())
            .filter(|&n| self.nodes[n].done == Done::Open)
            .map(|n| GoalId(n as u32))
            .collect()
    }

    /// Returns whether every goal is closed.
    pub fn is_finished(&self) -> bool {
        self.nodes.iter().all(|node| node.done != Done::Open)
    }

    /// Returns the members of each zone of the goal `goal`.
    ///
    /// # Panics
    ///
    /// For a goal of another session, past this one's nodes.
    pub fn goal(&self, goal: GoalId) -> &[Vec<TermId>] {
        &self.nodes[goal.index()].sequent
    }

    /// Returns the members of the goal `goal` with their zones, in the
    /// order [`Apply::at`] numbers them: the first zone's, then the
    /// next's.
    ///
    /// # Panics
    ///
    /// For a goal of another session.
    pub fn members(&self, goal: GoalId) -> Vec<(usize, TermId)> {
        self.goal(goal)
            .iter()
            .enumerate()
            .flat_map(|(zone, members)| members.iter().map(move |&m| (zone, m)))
            .collect()
    }

    /// Returns the hypotheses in force at the goal `goal`: those its scope
    /// and every scope around it offer, the innermost first, the
    /// session's assumptions last.
    ///
    /// # Panics
    ///
    /// For a goal of another session.
    pub fn hypotheses(&self, goal: GoalId) -> Vec<&[Vec<TermId>]> {
        let mut out = Vec::new();
        let mut scope = self.nodes[goal.index()].scope;
        loop {
            out.extend(
                self.scopes[scope as usize]
                    .hypotheses
                    .iter()
                    .map(Vec::as_slice),
            );
            if scope == 0 {
                break;
            }
            scope = self.scopes[scope as usize].parent;
        }
        out
    }

    /// Returns the open goal at `goal`, or the refusal of a goal that is
    /// none.
    fn open(&self, goal: GoalId) -> Result<&Goal, Error> {
        match self.nodes.get(goal.index()) {
            Some(node) if node.done == Done::Open => Ok(node),
            _ => Err(refused(format!("there is no open goal {}", goal.index()))),
        }
    }

    /// Refuses a sequent whose zones break the calculus's bounds or hold
    /// a member of a set twice.
    fn shaped(&self, sequent: &[Vec<TermId>]) -> Result<(), Error> {
        for (zone, members) in self.calculus.zones.iter().zip(sequent) {
            let len = members.len() as u64;
            if len < u64::from(zone.least) || zone.most.is_some_and(|m| len > u64::from(m)) {
                return Err(refused(format!(
                    "the zone {:?} would hold {len} members, outside what the calculus allows",
                    zone.name
                )));
            }
            if zone.discipline == Discipline::Set {
                let mut sorted = members.clone();
                sorted.sort_unstable();
                if sorted.windows(2).any(|pair| pair[0] == pair[1]) {
                    return Err(refused(format!(
                        "the zone {:?} is a set and would hold a member twice",
                        zone.name
                    )));
                }
            }
        }
        Ok(())
    }

    /// Returns whether two sequents are equal as their zones' disciplines
    /// read them.
    fn same(&self, a: &[Vec<TermId>], b: &[Vec<TermId>]) -> bool {
        a.len() == b.len()
            && self
                .calculus
                .zones
                .iter()
                .zip(a.iter().zip(b))
                .all(|(zone, (x, y))| {
                    let (mut x, mut y) = (x.clone(), y.clone());
                    if zone.discipline != Discipline::Sequence {
                        x.sort_unstable();
                        y.sort_unstable();
                    }
                    if zone.discipline == Discipline::Set {
                        x.dedup();
                        y.dedup();
                    }
                    x == y
                })
    }

    /// Returns the session's derivation, open goals as such: the nodes in
    /// postorder, premises before conclusions, the root last, and for each
    /// node of the session its place in the derivation.
    pub(crate) fn derived(&self) -> Result<(Derivation, Vec<u32>), Error> {
        let mut out = Derivation::new(self.calculus.name(), self.calculus.zones.len());
        out.adopt_terms(self.terms.clone());
        for scope in &self.scopes[1..] {
            out.scope(scope.parent)?;
        }
        let mut places = vec![u32::MAX; self.nodes.len()];
        // Postorder by a stack of (node, whether its premises are done).
        let mut stack = vec![(0u32, false)];
        while let Some((node, done)) = stack.pop() {
            let goal = &self.nodes[node as usize];
            if !done {
                stack.push((node, true));
                stack.extend(goal.premises.iter().rev().map(|&p| (p, false)));
                continue;
            }
            let premises: Vec<u32> = goal.premises.iter().map(|&p| places[p as usize]).collect();
            let (step, at) = match &goal.done {
                Done::Open => (Step::Open, &[][..]),
                Done::Hypothesis => (Step::Hypothesis, &[][..]),
                Done::Rule(name, at) => (Step::Rule(name), &at[..]),
            };
            let id = out.push(
                Node::new(&goal.sequent, step)
                    .premises(&premises)
                    .positions(at)
                    .scope(ScopeId::new(goal.scope)),
            )?;
            places[node as usize] = id.index() as u32;
        }
        Ok((out, places))
    }

    /// Checks the node at `node` of the session against its rule, or as
    /// a hypothesis, the rest taken as it is.
    fn verify(&self, node: u32) -> Result<(), Error> {
        let (derivation, places) = self.derived()?;
        derivation
            .check_node(
                &self.calculus,
                places[node as usize] as usize,
                &self.limits,
                |_| false,
            )
            .map_err(|error| match error {
                Error::InvalidDerivation { reason, .. } => refused(reason),
                other => other,
            })
    }

    /// Lists the rules that apply to the goal `goal`: for each rule and
    /// each choice of the members it acts on, the step if the checker
    /// takes it, or what it needs given. A rule that asks positions of
    /// more members than [`RULE_CHOICES`](Self::RULE_CHOICES) choices
    /// allow is left out.
    ///
    /// # Errors
    ///
    /// [`Error::StepRefused`] for a goal that is not open, and a refusal
    /// of a bound.
    pub fn rules(&self, goal: GoalId) -> Result<Vec<Applicable>, Error> {
        self.open(goal)?;
        let mut out: Vec<Applicable> = Vec::new();
        let mut seen: Vec<(String, Vec<TermId>)> = Vec::new();
        let flat = self.members(goal);
        for (index, rule) in self.calculus.rules.iter().enumerate() {
            let name = &self.calculus.form().rules[index].name;
            // The zones of the conclusion's members found by position.
            let zones: Vec<usize> = (0..self.calculus.zones.len())
                .flat_map(|zone| {
                    rule.conclusion.zones[zone]
                        .iter()
                        .filter(|e| matches!(e, Entry::Item(item) if item.find == Find::Position))
                        .map(move |_| zone)
                })
                .collect();
            let choices: Vec<Vec<u32>> = zones
                .iter()
                .map(|&zone| {
                    flat.iter()
                        .enumerate()
                        .filter(|&(_, &(z, _))| z == zone)
                        .map(|(n, _)| n as u32)
                        .collect()
                })
                .collect();
            let count = choices
                .iter()
                .try_fold(1usize, |n, c| n.checked_mul(c.len()))
                .unwrap_or(usize::MAX);
            if count > Self::RULE_CHOICES {
                continue;
            }
            for pick in 0..count {
                let mut at = Vec::with_capacity(choices.len());
                let mut rest = pick;
                for choice in &choices {
                    at.push(choice[rest % choice.len()]);
                    rest /= choice.len();
                }
                // Equal members at other places give the same step.
                let key: Vec<TermId> = at.iter().map(|&n| flat[n as usize].1).collect();
                if seen.iter().any(|(r, k)| r == name && *k == key) {
                    continue;
                }
                let step = Apply::new(name).at(&at);
                let mut trial = self.clone();
                match trial.plan(goal, rule, &step) {
                    Ok(plan) => {
                        if trial.commit(goal, name, plan).is_ok() {
                            seen.push((name.clone(), key));
                            out.push(Applicable {
                                rule: name.clone(),
                                at,
                                needs: Vec::new(),
                            });
                        }
                    }
                    Err(Unplanned::Needs(needs)) => {
                        seen.push((name.clone(), key));
                        out.push(Applicable {
                            rule: name.clone(),
                            at,
                            needs,
                        });
                    }
                    Err(Unplanned::Error(Error::Refused(refusal))) => {
                        return Err(Error::Refused(refusal));
                    }
                    Err(Unplanned::Error(_)) => {}
                }
            }
        }
        Ok(out)
    }

    /// The most choices of members [`rules`](Self::rules) tries for one
    /// rule.
    pub const RULE_CHOICES: usize = 4096;

    /// Applies a rule backwards to the goal `goal`: the goal becomes the
    /// rule's conclusion, and its premises are new open goals, which it
    /// returns. A premise that discharges hypotheses opens a scope of its
    /// own, whose goals may be closed by those hypotheses
    /// ([`assume`](Self::assume)).
    ///
    /// A variable only a premise holds, or a context the goal does not
    /// determine (a split between two contexts), must be given; a variable
    /// the rule asks to be new gets a new atom where it is not given. The
    /// checker checks the step before the state takes it.
    ///
    /// # Errors
    ///
    /// [`Error::StepNeeds`] naming what must be given,
    /// [`Error::StepRefused`] for a step the calculus does not allow (the
    /// reason given), [`Error::Parse`] for a given text that is no term,
    /// and a refusal of a bound.
    pub fn apply(&mut self, goal: GoalId, step: &Apply<'_>) -> Result<Vec<GoalId>, Error> {
        self.open(goal)?;
        let Some(&index) = self.calculus.rule_ids.get(step.rule) else {
            return Err(refused(format!("the calculus has no rule {:?}", step.rule)));
        };
        let rule = self.calculus.rules[index as usize].clone();
        let plan = match self.plan(goal, &rule, step) {
            Ok(plan) => plan,
            Err(Unplanned::Needs(needs)) => return Err(Error::StepNeeds { needs }),
            Err(Unplanned::Error(error)) => return Err(error),
        };
        self.commit(goal, step.rule, plan)
    }

    /// Takes a planned step: the premises appended as open goals, the
    /// goal closed, the step checked; a step the checker refuses is taken
    /// back.
    fn commit(&mut self, goal: GoalId, rule: &str, plan: Plan) -> Result<Vec<GoalId>, Error> {
        let before = (self.nodes.len(), self.scopes.len());
        let total = self
            .nodes
            .iter()
            .map(|n| n.sequent.iter().map(Vec::len).sum::<usize>())
            .chain(
                plan.premises
                    .iter()
                    .map(|(s, _)| s.iter().map(Vec::len).sum()),
            )
            .fold(0u64, |sum, n| sum.saturating_add(n as u64));
        if let Some(limit) = self.limits.occurrences
            && total > limit
        {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: total,
                limit,
            }));
        }
        let scope = self.nodes[goal.index()].scope;
        let mut premises = Vec::with_capacity(plan.premises.len());
        for (index, (sequent, discharges)) in plan.premises.into_iter().enumerate() {
            self.shaped(&sequent)?;
            let place = self.nodes.len() as u32;
            let opened = match discharges {
                Some(hypotheses) => {
                    self.scopes.push(Scope {
                        parent: scope,
                        owner: Some((goal.0, index as u32)),
                        hypotheses,
                    });
                    (self.scopes.len() - 1) as u32
                }
                None => scope,
            };
            self.nodes.push(Goal {
                sequent,
                done: Done::Open,
                premises: Vec::new(),
                scope: opened,
                opened,
            });
            premises.push(place);
        }
        let node = &mut self.nodes[goal.index()];
        node.done = Done::Rule(rule.to_owned(), plan.at);
        node.premises.clone_from(&premises);
        if let Err(error) = self.verify(goal.0) {
            self.back(goal, before);
            return Err(error);
        }
        self.history.push(goal.0);
        Ok(premises.into_iter().map(GoalId).collect())
    }

    /// Takes back a step on `goal` that left the session with `before`
    /// nodes and scopes.
    fn back(&mut self, goal: GoalId, before: (usize, usize)) {
        self.nodes.truncate(before.0);
        self.scopes.truncate(before.1);
        let node = &mut self.nodes[goal.index()];
        node.done = Done::Open;
        node.premises.clear();
        node.scope = node.opened;
    }

    /// Closes the goal `goal` by a hypothesis in force: one the scope it
    /// lies in or a scope around it offers, the innermost first, or one of
    /// the session's assumptions. The checker checks it at the rule that
    /// discharges it.
    ///
    /// # Errors
    ///
    /// [`Error::StepRefused`] for a goal that no hypothesis in force is.
    pub fn assume(&mut self, goal: GoalId) -> Result<(), Error> {
        let mut scope = self.open(goal)?.scope;
        loop {
            let offered = &self.scopes[scope as usize];
            let found = offered
                .hypotheses
                .iter()
                .any(|h| self.same(h, &self.nodes[goal.index()].sequent));
            if found {
                let owner = offered.owner;
                let node = &mut self.nodes[goal.index()];
                node.done = Done::Hypothesis;
                node.scope = scope;
                // The rule that discharges it checks it; an assumption is
                // the session's own.
                if let Some((owner, _)) = owner
                    && let Err(error) = self.verify(owner)
                {
                    let node = &mut self.nodes[goal.index()];
                    node.done = Done::Open;
                    node.scope = node.opened;
                    return Err(error);
                }
                self.history.push(goal.0);
                return Ok(());
            }
            if scope == 0 {
                return Err(refused("the goal is no hypothesis in force"));
            }
            scope = self.scopes[scope as usize].parent;
        }
    }

    /// Takes back the last step and returns the goal it reopened, or
    /// `None` where no step is left to take back.
    pub fn undo(&mut self) -> Option<GoalId> {
        let goal = GoalId(self.history.pop()?);
        let node = &self.nodes[goal.index()];
        let before = match node.done {
            Done::Rule(..) => {
                let nodes = node
                    .premises
                    .first()
                    .map_or(self.nodes.len(), |&p| p as usize);
                let scopes = self
                    .scopes
                    .iter()
                    .position(|s| s.owner.is_some_and(|(o, _)| o == goal.0))
                    .unwrap_or(self.scopes.len());
                (nodes, scopes)
            }
            Done::Hypothesis | Done::Open => (self.nodes.len(), self.scopes.len()),
        };
        self.back(goal, before);
        Some(goal)
    }

    /// Returns the derivation built so far, open goals as
    /// [`Step::Open`](super::Step::Open), and for each derivation node the
    /// goal it is, where it is one.
    ///
    /// # Errors
    ///
    /// A refusal where the derivation's indices would pass a `u32`.
    pub fn derivation(&self) -> Result<Derivation, Error> {
        self.derived().map(|(derivation, _)| derivation)
    }

    /// Returns the finished derivation, checked whole by the calculus's
    /// checker within the session's limits: valid in the calculus given.
    ///
    /// # Errors
    ///
    /// [`Error::OpenGoals`] while a goal is open,
    /// [`Error::InvalidDerivation`] should the check refuse it, and a
    /// refusal of a bound or of `stop`.
    pub fn finished(&self, stop: impl FnMut(crate::Progress) -> bool) -> Result<Derivation, Error> {
        let open = self.goals().len();
        if open > 0 {
            return Err(Error::OpenGoals { count: open });
        }
        let derivation = self.derivation()?;
        derivation.check(&self.calculus, &self.limits, stop)?;
        Ok(derivation)
    }

    /// Plans a backward step of `rule` on `goal`.
    fn plan(&mut self, goal: GoalId, rule: &Rule, step: &Apply<'_>) -> Result<Plan, Unplanned> {
        let calculus = self.calculus.clone();
        let zones = calculus.zones.len();
        let sequent = self.nodes[goal.index()].sequent.clone();
        let flat: Vec<(usize, usize)> = sequent
            .iter()
            .enumerate()
            .flat_map(|(zone, members)| (0..members.len()).map(move |p| (zone, p)))
            .collect();
        let mut bindings: Vec<Option<TermId>> = vec![None; calculus.variables as usize];
        let mut contexts: Vec<Option<Vec<TermId>>> = vec![None; calculus.contexts.len()];
        // The conclusion's written members.
        let mut at = step.at.iter();
        let mut taken: Vec<Vec<bool>> = sequent.iter().map(|z| vec![false; z.len()]).collect();
        let mut places: Vec<Vec<usize>> = vec![Vec::new(); zones];
        for zone in 0..zones {
            let members = &sequent[zone];
            for entry in &rule.conclusion.zones[zone] {
                let Entry::Item(item) = entry else { continue };
                let place = match item.find {
                    Find::Start(p) => Some(p as usize),
                    Find::End(p) => members.len().checked_sub(1 + p as usize),
                    Find::Position | Find::Value => {
                        let Some(&number) = at.next() else {
                            return Err(refused(
                                "the step names fewer members than the rule acts on",
                            )
                            .into());
                        };
                        match flat.get(number as usize) {
                            Some(&(z, p)) if z == zone => Some(p),
                            _ => {
                                return Err(refused(format!(
                                    "member {number} is not in the zone {:?}, where the rule acts",
                                    calculus.zones[zone].name
                                ))
                                .into());
                            }
                        }
                    }
                };
                let Some(place) = place.filter(|&p| p < members.len()) else {
                    return Err(refused("the goal has no member where the rule writes one").into());
                };
                if calculus.zones[zone].discipline != Discipline::Set && taken[zone][place] {
                    return Err(
                        refused("two members the rule writes out stand at one place").into(),
                    );
                }
                if !self.matches(rule, item.root, members[place], &mut bindings) {
                    return Err(refused(format!(
                        "{} is no instance of what the rule writes out",
                        calculus.show(&self.terms, members[place]).limit(200)
                    ))
                    .into());
                }
                taken[zone][place] = true;
                places[zone].push(place);
            }
        }
        // Only the conclusion's members found by position give positions.
        let mut positions = Vec::new();
        for zone in 0..zones {
            let mut k = 0;
            for entry in &rule.conclusion.zones[zone] {
                let Entry::Item(item) = entry else { continue };
                if item.find == Find::Position {
                    positions.push(places[zone][k] as u32);
                }
                k += 1;
            }
        }
        if at.next().is_some() {
            return Err(refused("the step names more members than the rule acts on").into());
        }
        // What is given.
        for &(name, text) in step.given {
            if let Some(v) = calculus.variable_names.iter().position(|(n, _)| n == name) {
                let sort = calculus.sort_name(calculus.variable_names[v].1).to_owned();
                let term = calculus.parse_term(&mut self.terms, text, &sort, &self.limits)?;
                match bindings[v] {
                    Some(bound) if bound != term => {
                        return Err(refused(format!(
                            "{name} is {} in the goal",
                            calculus.show(&self.terms, bound).limit(200)
                        ))
                        .into());
                    }
                    _ => bindings[v] = Some(term),
                }
            } else if let Some(c) = calculus.context_names.iter().position(|(n, _)| n == name) {
                let Some(sort) = calculus.context_names[c].1 else {
                    return Err(refused(format!("no rule uses the context {name}")).into());
                };
                let members = calculus.parse_list(&mut self.terms, text, sort, &self.limits)?;
                contexts[c] = Some(members);
            } else {
                return Err(refused(format!(
                    "{name:?} is no variable or context of the calculus"
                ))
                .into());
            }
        }
        // The contexts the conclusion determines.
        let mut needs: Vec<String> = Vec::new();
        for zone in 0..zones {
            let discipline = calculus.zones[zone].discipline;
            let members = &sequent[zone];
            let entries = &rule.conclusion.zones[zone];
            if discipline == Discipline::Set {
                for entry in entries {
                    let &Entry::Context { variable, under } = entry else {
                        continue;
                    };
                    let value: Vec<TermId> = members
                        .iter()
                        .filter_map(|&m| self.unmap(m, under))
                        .collect();
                    let slot = &mut contexts[variable as usize];
                    *slot = Some(match slot.take() {
                        None => value,
                        Some(known) => known.into_iter().filter(|m| value.contains(m)).collect(),
                    });
                }
                continue;
            }
            // Segments: all of a multiset zone; in a sequence, those
            // between the written members.
            let mut bounds: Vec<(usize, usize)> = Vec::new();
            if discipline == Discipline::Sequence {
                let mut sorted = places[zone].clone();
                if sorted.windows(2).any(|w| w[0] >= w[1]) {
                    return Err(
                        refused("the members the rule writes out stand in another order").into(),
                    );
                }
                let mut from = 0;
                sorted.push(members.len());
                for p in sorted {
                    bounds.push((from, p));
                    from = p + 1;
                }
            } else {
                bounds.push((0, members.len()));
            }
            let mut segment = 0;
            let mut contexts_in: Vec<Vec<(u32, Option<u32>)>> = vec![Vec::new(); bounds.len()];
            for entry in entries {
                match *entry {
                    Entry::Item(_) if discipline == Discipline::Sequence => segment += 1,
                    Entry::Item(_) => {}
                    Entry::Context { variable, under } => {
                        contexts_in[segment].push((variable, under));
                    }
                }
            }
            for (k, &(from, to)) in bounds.iter().enumerate() {
                let mut rest: Vec<TermId> = (from..to)
                    .filter(|&p| !taken[zone][p])
                    .map(|p| members[p])
                    .collect();
                let unknown: Vec<(u32, Option<u32>)> = contexts_in[k]
                    .iter()
                    .copied()
                    .filter(|&(v, _)| contexts[v as usize].is_none())
                    .collect();
                for &(variable, under) in &contexts_in[k] {
                    let Some(known) = contexts[variable as usize].clone() else {
                        continue;
                    };
                    let mapped = self.map(&known, under)?;
                    rest = remove(&rest, &mapped, discipline == Discipline::Sequence)
                        .ok_or_else(|| refused("a given context is not what the goal holds"))?;
                }
                match unknown.as_slice() {
                    [] if rest.is_empty() => {}
                    [] => {
                        return Err(refused(
                            "the goal holds members the rule does not account for",
                        )
                        .into());
                    }
                    [(variable, under)] => {
                        let value: Option<Vec<TermId>> =
                            rest.iter().map(|&m| self.unmap(m, *under)).collect();
                        let value = value.ok_or_else(|| {
                            refused("a member of a context is not under its connective")
                        })?;
                        contexts[*variable as usize] = Some(value);
                    }
                    many => needs.extend(
                        many.iter()
                            .skip(1)
                            .map(|&(v, _)| calculus.context_names[v as usize].0.clone()),
                    ),
                }
            }
        }
        // A split is asked first: once given, the rest follows.
        if !needs.is_empty() {
            needs.sort();
            needs.dedup();
            return Err(Unplanned::Needs(needs));
        }
        // New atoms for the variables the rule asks to be new.
        for &variable in &rule.fresh {
            if bindings[variable as usize].is_none() {
                let (name, sort) = calculus.variable_names[variable as usize].clone();
                let atom = self.new_atom(&name, sort)?;
                bindings[variable as usize] = Some(atom);
            }
        }
        // The premises.
        let mut premises = Vec::with_capacity(rule.premises.len());
        let mut premise_positions = Vec::new();
        for premise in &rule.premises {
            let mut built = vec![Vec::new(); zones];
            let mut pending: Vec<(usize, TermId)> = Vec::new();
            for zone in 0..zones {
                for entry in &premise.shape.zones[zone] {
                    match *entry {
                        Entry::Item(item) => {
                            if let Some(term) =
                                self.instance(rule, item.first, item.root, &bindings, &mut needs)?
                            {
                                if item.find == Find::Position {
                                    pending.push((zone, term));
                                }
                                built[zone].push(term);
                            }
                        }
                        Entry::Context { variable, under } => {
                            match contexts[variable as usize].clone() {
                                Some(known) => built[zone].extend(self.map(&known, under)?),
                                None => {
                                    let name = calculus.context_names[variable as usize].0.clone();
                                    if !needs.contains(&name) {
                                        needs.push(name);
                                    }
                                }
                            }
                        }
                    }
                }
                if calculus.zones[zone].discipline == Discipline::Set {
                    let mut seen = Vec::new();
                    built[zone].retain(|m| {
                        let fresh = !seen.contains(m);
                        seen.push(*m);
                        fresh
                    });
                }
            }
            // A member found by position: where it stands now. Two equal
            // members of a multiset are interchangeable.
            let mut used: Vec<Vec<bool>> = built.iter().map(|z| vec![false; z.len()]).collect();
            for (zone, term) in pending {
                let place = built[zone]
                    .iter()
                    .enumerate()
                    .position(|(p, &m)| m == term && !used[zone][p])
                    .or_else(|| built[zone].iter().position(|&m| m == term));
                if let Some(place) = place {
                    used[zone][place] = true;
                    premise_positions.push(place as u32);
                }
            }
            let discharges = if premise.discharges.is_empty() {
                None
            } else {
                let mut hypotheses = Vec::with_capacity(premise.discharges.len());
                for hypothesis in &premise.discharges {
                    let mut sequent = vec![Vec::new(); zones];
                    for (zone, items) in hypothesis.iter().enumerate() {
                        for &(first, root) in items {
                            if let Some(term) =
                                self.instance(rule, first, root, &bindings, &mut needs)?
                            {
                                sequent[zone].push(term);
                            }
                        }
                    }
                    hypotheses.push(sequent);
                }
                Some(hypotheses)
            };
            premises.push((built, discharges));
        }
        if !needs.is_empty() {
            needs.sort();
            needs.dedup();
            return Err(Unplanned::Needs(needs));
        }
        for &variable in &rule.atomic {
            let bound = bindings[variable as usize];
            if !bound.is_some_and(|t| matches!(self.terms.term(t), Term::Atom { .. })) {
                return Err(refused(format!(
                    "{} must be an atom",
                    calculus.variable_names[variable as usize].0
                ))
                .into());
            }
        }
        positions.extend(premise_positions);
        Ok(Plan {
            premises,
            at: positions,
        })
    }

    /// Returns whether `term` is an instance of the pattern at `root`,
    /// binding the variables it meets first.
    fn matches(
        &self,
        rule: &Rule,
        root: u32,
        term: TermId,
        bindings: &mut [Option<TermId>],
    ) -> bool {
        let mut stack = vec![(root, term)];
        while let Some((pattern, term)) = stack.pop() {
            match rule.patterns[pattern as usize] {
                Pattern::Variable(v) => match bindings[v as usize] {
                    None => bindings[v as usize] = Some(term),
                    Some(bound) if bound == term => {}
                    Some(_) => return false,
                },
                Pattern::Apply {
                    connective,
                    start,
                    len,
                } => {
                    let name = &self.calculus.form().connectives[connective as usize].name;
                    let Term::Apply {
                        connective: found,
                        arguments,
                    } = self.terms.term(term)
                    else {
                        return false;
                    };
                    if found != name || arguments.len() != len as usize {
                        return false;
                    }
                    stack.extend(
                        rule.arguments(start, len)
                            .iter()
                            .copied()
                            .zip(arguments.iter().copied()),
                    );
                }
            }
        }
        true
    }

    /// Builds the instance of the pattern with nodes `first` to `root`,
    /// or returns `None` and adds to `needs` the variables it lacks.
    fn instance(
        &mut self,
        rule: &Rule,
        first: u32,
        root: u32,
        bindings: &[Option<TermId>],
        needs: &mut Vec<String>,
    ) -> Result<Option<TermId>, Error> {
        let mut built: Vec<Option<TermId>> = Vec::with_capacity((root - first) as usize + 1);
        for node in first..=root {
            let term = match rule.patterns[node as usize] {
                Pattern::Variable(v) => {
                    let bound = bindings[v as usize];
                    if bound.is_none() {
                        let name = self.calculus.variable_names[v as usize].0.clone();
                        if !needs.contains(&name) {
                            needs.push(name);
                        }
                    }
                    bound
                }
                Pattern::Apply {
                    connective,
                    start,
                    len,
                } => {
                    let arguments: Option<Vec<TermId>> = rule
                        .arguments(start, len)
                        .iter()
                        .map(|&a| built[(a - first) as usize])
                        .collect();
                    match arguments {
                        Some(arguments) => {
                            let name = self.calculus.form().connectives[connective as usize]
                                .name
                                .clone();
                            Some(self.terms.apply(&name, &arguments)?)
                        }
                        None => None,
                    }
                }
            };
            built.push(term);
        }
        Ok(built.last().copied().flatten())
    }

    /// Returns each member of a context with the connective applied, or
    /// the members as they are.
    fn map(&mut self, members: &[TermId], under: Option<u32>) -> Result<Vec<TermId>, Error> {
        let Some(connective) = under else {
            return Ok(members.to_vec());
        };
        let name = self.calculus.form().connectives[connective as usize]
            .name
            .clone();
        members
            .iter()
            .map(|&m| self.terms.apply(&name, &[m]))
            .collect()
    }

    /// Returns what stands under the connective in `member`, or the member
    /// itself where there is none; `None` where it is not built with it.
    fn unmap(&self, member: TermId, under: Option<u32>) -> Option<TermId> {
        let Some(connective) = under else {
            return Some(member);
        };
        let name = &self.calculus.form().connectives[connective as usize].name;
        match self.terms.term(member) {
            Term::Apply {
                connective: found,
                arguments: &[argument],
            } if found == name => Some(argument),
            Term::Apply { .. } | Term::Atom { .. } => None,
        }
    }

    /// Returns an atom of the sort at `sort` named after `base` that no
    /// term of the session names.
    fn new_atom(&mut self, base: &str, sort: u32) -> Result<TermId, Error> {
        let sort_name = self.calculus.sort_name(sort).to_owned();
        let base: String = base
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let base = if base.is_empty() {
            "a".to_owned()
        } else {
            base.to_lowercase()
        };
        let mut n = 0u64;
        loop {
            let name = if n == 0 {
                base.clone()
            } else {
                format!("{base}{n}")
            };
            if !self.terms.names(&name) {
                return self.terms.atom(&name, &sort_name);
            }
            n += 1;
        }
    }
}

impl Interactive {
    /// Returns the session these parts make, every step checked: the
    /// shape of the nodes and scopes, every closed node by the calculus's
    /// checker, every hypothesis of the root scope one of the session's
    /// assumptions, and the history as the steps left it.
    pub(crate) fn from_parts(
        calculus: Calculus,
        terms: Terms,
        scopes: Vec<Scope>,
        nodes: Vec<Goal>,
        history: Vec<u32>,
        limits: &Limits,
    ) -> Result<Self, Error> {
        let fault = |reason: &'static str| Error::InconsistentSession { reason };
        let zones = calculus.zones.len();
        if nodes.is_empty() || scopes.is_empty() {
            return Err(fault("a session has a node and a scope"));
        }
        let rules = &calculus.rules;
        // The scopes: each inside an earlier one, the root's its own.
        for (index, scope) in scopes.iter().enumerate() {
            let fits = match index {
                0 => scope.parent == 0 && scope.owner.is_none(),
                _ => (scope.parent as usize) < index && scope.owner.is_some(),
            };
            if !fits {
                return Err(fault("a scope lies in no earlier scope, or has no owner"));
            }
        }
        // The nodes: a tree, the root first, every premise after its
        // conclusion and the premise of exactly one node.
        let mut parent: Vec<Option<(u32, u32)>> = vec![None; nodes.len()];
        for (index, node) in nodes.iter().enumerate() {
            if node.sequent.len() != zones
                || node.scope as usize >= scopes.len()
                || node.opened as usize >= scopes.len()
            {
                return Err(fault("a node has other zones or scopes than the session"));
            }
            match node.done {
                Done::Rule(ref name, _) => {
                    let Some(&rule) = calculus.rule_ids.get(name) else {
                        return Err(fault("a node names no rule of the calculus"));
                    };
                    if node.premises.len() != rules[rule as usize].premises.len() {
                        return Err(fault("a node has another number of premises than its rule"));
                    }
                }
                Done::Open | Done::Hypothesis if node.premises.is_empty() => {}
                Done::Open | Done::Hypothesis => {
                    return Err(fault("an open goal or a hypothesis has premises"));
                }
            }
            if node.done != Done::Hypothesis && node.scope != node.opened {
                return Err(fault("only a hypothesis leaves the scope it was opened in"));
            }
            for (k, &premise) in node.premises.iter().enumerate() {
                let p = premise as usize;
                if p <= index || p >= nodes.len() || parent[p].is_some() {
                    return Err(fault("the nodes are no tree with its root first"));
                }
                parent[p] = Some((index as u32, k as u32));
            }
        }
        if parent.iter().skip(1).any(Option::is_none) || nodes[0].opened != 0 {
            return Err(fault("the nodes are no tree with its root first"));
        }
        // Where each node was opened: its parent's scope, or the scope
        // its parent owns for a premise that discharges.
        for (index, node) in nodes.iter().enumerate().skip(1) {
            let (p, k) = parent[index].expect("every node but the root has a parent");
            let up = &nodes[p as usize];
            let Done::Rule(ref name, _) = up.done else {
                return Err(fault("a premise of a node without a rule"));
            };
            let rule = &rules[calculus.rule_ids[name] as usize];
            let expected = if rule.premises[k as usize].discharges.is_empty() {
                Some(up.scope)
            } else {
                scopes
                    .iter()
                    .position(|s| s.owner == Some((p, k)) && s.parent == up.scope)
                    .map(|s| s as u32)
            };
            if expected != Some(node.opened) {
                return Err(fault(
                    "a goal was opened in another scope than its rule gives",
                ));
            }
        }
        let session = Self {
            calculus,
            terms,
            scopes,
            nodes,
            history,
            limits: *limits,
        };
        // An assumption is the session's own; every open goal keeps to
        // the zones' bounds.
        for node in &session.nodes {
            if node.done == Done::Hypothesis
                && node.scope == 0
                && !session.scopes[0]
                    .hypotheses
                    .iter()
                    .any(|h| session.same(h, &node.sequent))
            {
                return Err(fault("a hypothesis of the root scope is no assumption"));
            }
            if node.done == Done::Open {
                session.shaped(&node.sequent)?;
            }
        }
        for hypothesis in &session.scopes[0].hypotheses {
            session.shaped(hypothesis)?;
        }
        // Every step by the checker.
        let (derivation, _) = session.derived()?;
        derivation.check_open(&session.calculus, limits, |_| false)?;
        // The history, from the last step back: each a distinct closed
        // node, a rule's premises and scopes the last the session held.
        let (mut nodes_left, mut scopes_left) = (session.nodes.len(), session.scopes.len());
        let mut seen = vec![false; session.nodes.len()];
        for &goal in session.history.iter().rev() {
            let g = goal as usize;
            if g >= nodes_left || std::mem::replace(&mut seen[g], true) {
                return Err(fault(
                    "the history names a node twice or one a later step made",
                ));
            }
            let node = &session.nodes[g];
            match node.done {
                Done::Open => return Err(fault("the history names an open goal")),
                Done::Hypothesis => {}
                Done::Rule(..) => {
                    let k = node.premises.len();
                    let suffix = node
                        .premises
                        .iter()
                        .enumerate()
                        .all(|(i, &p)| p as usize == nodes_left - k + i);
                    let owned = session
                        .scopes
                        .iter()
                        .filter(|s| s.owner.is_some_and(|(o, _)| o == goal))
                        .count();
                    let last = session.scopes[scopes_left - owned..scopes_left]
                        .iter()
                        .all(|s| s.owner.is_some_and(|(o, _)| o == goal));
                    if !suffix || !last {
                        return Err(fault("a step's goals or scopes are not the last it made"));
                    }
                    nodes_left -= k;
                    scopes_left -= owned;
                }
            }
        }
        Ok(session)
    }
}

/// Returns `whole` without the members of `part`, the first equal ones of
/// a multiset, or a prefix or suffix of a sequence; `None` where `part`
/// is not in it.
fn remove(whole: &[TermId], part: &[TermId], sequence: bool) -> Option<Vec<TermId>> {
    if sequence {
        if let Some(rest) = whole.strip_prefix(part) {
            return Some(rest.to_vec());
        }
        return whole.strip_suffix(part).map(<[TermId]>::to_vec);
    }
    let mut rest = whole.to_vec();
    for member in part {
        let at = rest.iter().position(|m| m == member)?;
        rest.remove(at);
    }
    Some(rest)
}
