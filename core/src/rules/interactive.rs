// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Building a derivation step by step in a calculus given as data: open
//! goals, the rules that apply to one, a rule applied backwards from its
//! conclusion, a goal closed by a hypothesis, and undo. Every step is
//! checked by the calculus's checker before the state takes it.

use super::calculus::{Calculus, Discipline, Entry, Find, Pattern, Rule};
use super::derivation::{Derivation, Node, NodeId, ScopeId, Step};
use super::terms::{Term, TermId, Terms};
use crate::limits::{Limits, Refusal};
use crate::{Backwards, Error, StepFault};

/// An open goal of a session, by its place in the session. Its JSON is
/// that number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
#[serde(transparent)]
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
    /// The premises' first members, by term, in the rule's order, which
    /// bind what they hold: how a proof found is taken step by step,
    /// whose premises are known.
    pub(crate) premises: &'a [TermId],
}

impl<'a> Apply<'a> {
    /// Returns the step of `rule`, acting on nothing and given nothing.
    pub const fn new(rule: &'a str) -> Self {
        Self {
            rule,
            at: &[],
            given: &[],
            premises: &[],
        }
    }

    /// Returns the step with these premises' first members, by term.
    #[must_use]
    pub(crate) const fn premises(mut self, premises: &'a [TermId]) -> Self {
        self.premises = premises;
        self
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

/// A rule of the calculus tried on a goal, at one choice of the members it
/// acts on, and whether the step is legal there, as
/// [`Interactive::candidates`] lists them.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Candidate {
    /// The rule's name.
    pub rule: String,
    /// The members of the goal it acts on, by their number; empty for a
    /// rule refused at every choice.
    pub at: Vec<u32>,
    /// Whether the step is legal.
    pub status: Status,
}

/// Whether a candidate step is legal on the goal as it stands. Whether
/// the goals it opens can be closed is not judged.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Status {
    /// The step applies as it is.
    Applies,
    /// The step needs these variables or contexts given: what the goal
    /// leaves open.
    #[non_exhaustive]
    NeedsInput {
        /// Their names.
        needs: Vec<String>,
    },
    /// No choice of members makes the step legal. The refusal is
    /// [`apply`](Interactive::apply)'s at the first choice:
    /// [`Error::StepRefused`] with its fault, or the refusal of a value's
    /// text.
    Refused(Error),
}

/// A line of a session that a step may cite, by its number: a hypothesis
/// a scope offers, or a line a forward step derived. Lines are numbered
/// from 1 in the order they enter the session, the assumptions first, and
/// keep their numbers. Its JSON is that number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
#[serde(transparent)]
pub struct LineId(u32);

impl LineId {
    /// Returns the line numbered `number`, counted from 1, which the
    /// session's own calls refuse where no line in force has it.
    pub const fn new(number: u32) -> Self {
        Self(number)
    }

    /// Returns the line's number, counted from 1.
    pub const fn number(self) -> u32 {
        self.0
    }
}

/// A rule to apply forwards from lines in force, as
/// [`Interactive::derive`] takes it.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Derive<'a> {
    /// The rule's name.
    pub rule: &'a str,
    /// The lines the rule's premises are, in the rule's order.
    pub lines: &'a [LineId],
    /// Values for what the lines leave open, each a variable of the rule
    /// by name and its text: a formula only the conclusion holds.
    pub given: &'a [(&'a str, &'a str)],
}

impl<'a> Derive<'a> {
    /// Returns the step of `rule`, citing nothing and given nothing.
    pub const fn new(rule: &'a str) -> Self {
        Self {
            rule,
            lines: &[],
            given: &[],
        }
    }

    /// Returns the step citing these lines as its premises.
    #[must_use]
    pub const fn lines(mut self, lines: &'a [LineId]) -> Self {
        self.lines = lines;
        self
    }

    /// Returns the step with these values given.
    #[must_use]
    pub const fn given(mut self, given: &'a [(&'a str, &'a str)]) -> Self {
        self.given = given;
        self
    }
}

/// What [`Interactive::undo`] took back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Undone {
    /// A step that closed this goal, which is open again.
    Reopened(GoalId),
    /// A forward step, whose line is gone.
    Removed(LineId),
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
    /// Closed by the line a forward step derived, at this node.
    Line(u32),
}

/// Where a line of a session lies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Place {
    /// The hypothesis at `index` that the scope at `scope` offers.
    Offered {
        /// The scope.
        scope: u32,
        /// The hypothesis's place among the scope's.
        index: usize,
    },
    /// The line a forward step derived, at this node.
    Derived(u32),
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
    /// Whether it is a line a forward step derived: it stands outside the
    /// tree from the root, and later steps cite it.
    pub(crate) line: bool,
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
/// derivation built backwards from its conclusion and forwards from the
/// lines in force, with open goals.
///
/// [`rules`](Self::rules) lists the rules that apply to a goal,
/// [`apply`](Self::apply) applies one backwards, asking what the goal
/// leaves open (a formula only a premise holds, how a context splits),
/// [`derive`](Self::derive) applies one forwards from lines in force and
/// adds its conclusion as a new line, [`lines`](Self::lines) lists the
/// lines in force at a goal with their numbers, [`assume`](Self::assume)
/// and [`cite`](Self::cite) close a goal by a line in force, and
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
/// up (each a derivation's node, its `premises`, `opened` where a
/// hypothesis left the scope it was opened in), and the steps in order
/// (each the goal it closed or the line it derived). A session that holds
/// a line a forward step derived is written at level 18: that node has
/// `"line": true` and cites earlier lines among its premises, and a goal
/// closed by such a line has `"cites"`, the line's node. Reading one
/// checks every step taken.
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
fn refused(fault: StepFault) -> Error {
    Error::StepRefused {
        fault: Box::new(fault),
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
                line: false,
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

    /// Returns how many steps the session holds: what [`undo`](Self::undo)
    /// can take back.
    pub fn steps(&self) -> usize {
        self.history.len()
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

    /// Returns the scope at `scope` and every scope around it, the
    /// innermost first.
    fn chain(&self, scope: u32) -> Vec<u32> {
        let mut out = vec![scope];
        let mut scope = scope;
        while scope != 0 {
            scope = self.scopes[scope as usize].parent;
            out.push(scope);
        }
        out
    }

    /// Returns every line of the session in the order of their numbers:
    /// the assumptions, then each scope's hypotheses and each derived line
    /// in the order the steps made them.
    fn line_places(&self) -> Vec<Place> {
        // A scope enters with the premise its step opened in it, a line
        // with its own node: both keys are node places, which grow with
        // the steps.
        let mut entered: Vec<(u32, Place)> = Vec::new();
        for (scope, offered) in self.scopes.iter().enumerate().skip(1) {
            let key = offered
                .owner
                .and_then(|(n, k)| {
                    self.nodes
                        .get(n as usize)?
                        .premises
                        .get(k as usize)
                        .copied()
                })
                .unwrap_or(u32::MAX);
            entered.extend((0..offered.hypotheses.len()).map(|index| {
                (
                    key,
                    Place::Offered {
                        scope: scope as u32,
                        index,
                    },
                )
            }));
        }
        entered.extend(
            (0..self.nodes.len() as u32)
                .filter(|&n| self.nodes[n as usize].line)
                .map(|n| (n, Place::Derived(n))),
        );
        entered.sort_by_key(|&(key, _)| key);
        (0..self.scopes[0].hypotheses.len())
            .map(|index| Place::Offered { scope: 0, index })
            .chain(entered.into_iter().map(|(_, place)| place))
            .collect()
    }

    /// Returns the scope a line lies in.
    fn place_scope(&self, place: Place) -> u32 {
        match place {
            Place::Offered { scope, .. } => scope,
            Place::Derived(node) => self.nodes[node as usize].scope,
        }
    }

    /// Returns a line's sequent.
    fn place_sequent(&self, place: Place) -> &[Vec<TermId>] {
        match place {
            Place::Offered { scope, index } => &self.scopes[scope as usize].hypotheses[index],
            Place::Derived(node) => &self.nodes[node as usize].sequent,
        }
    }

    /// Returns the lines in force at the goal `goal`, with their numbers:
    /// the hypotheses its scope and every scope around it offer, and the
    /// lines derived in those scopes, in the order of their numbers. A
    /// line of a subproof that is closed at the goal is not among them.
    ///
    /// # Panics
    ///
    /// For a goal of another session.
    pub fn lines(&self, goal: GoalId) -> Vec<(LineId, &[Vec<TermId>])> {
        let chain = self.chain(self.nodes[goal.index()].scope);
        self.line_places()
            .into_iter()
            .enumerate()
            .filter(|&(_, place)| chain.contains(&self.place_scope(place)))
            .map(|(n, place)| (LineId(n as u32 + 1), self.place_sequent(place)))
            .collect()
    }

    /// Returns where the line `line` lies, or the refusal of a line not in
    /// force at the goal `goal`.
    fn in_force(&self, goal: GoalId, line: LineId) -> Result<Place, Error> {
        let places = self.line_places();
        let Some(&place) = (line.0 as usize)
            .checked_sub(1)
            .and_then(|index| places.get(index))
        else {
            return Err(refused(StepFault::NoLine { line: line.0 }));
        };
        if self
            .chain(self.nodes[goal.index()].scope)
            .contains(&self.place_scope(place))
        {
            Ok(place)
        } else {
            Err(refused(StepFault::NotInForce {
                line: line.0,
                goal: goal.0,
            }))
        }
    }

    /// Returns the open goal at `goal`, or the refusal of a goal that is
    /// none.
    pub(super) fn open(&self, goal: GoalId) -> Result<&Goal, Error> {
        match self.nodes.get(goal.index()) {
            Some(node) if node.done == Done::Open => Ok(node),
            _ => Err(refused(StepFault::NoGoal { goal: goal.0 })),
        }
    }

    /// Refuses a sequent whose zones break the calculus's bounds or hold
    /// a member of a set twice.
    fn shaped(&self, sequent: &[Vec<TermId>]) -> Result<(), Error> {
        for (zone, members) in self.calculus.zones.iter().zip(sequent) {
            let len = members.len() as u64;
            if len < u64::from(zone.least) || zone.most.is_some_and(|m| len > u64::from(m)) {
                return Err(refused(StepFault::ZoneSize {
                    zone: zone.name.clone(),
                    members: len,
                }));
            }
            if zone.discipline == Discipline::Set {
                let mut sorted = members.clone();
                sorted.sort_unstable();
                if sorted.windows(2).any(|pair| pair[0] == pair[1]) {
                    return Err(refused(StepFault::Twice {
                        zone: zone.name.clone(),
                    }));
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
        self.derived_with(false)
            .map(|(derivation, places, _)| (derivation, places))
    }

    /// [`derived`](Self::derived), with every hypothesis each scope
    /// offers as a node of its own, cited or not, when `offered`: a
    /// scope's just before its first node, or before the rule that
    /// discharges it where no node lies in it, so that a Fitch proof keeps
    /// the derivation's order. Also returns where each scope's hypotheses
    /// start (`u32::MAX` without `offered`).
    fn derived_with(&self, offered: bool) -> Result<(Derivation, Vec<u32>, Vec<u32>), Error> {
        let mut out = Derivation::new(self.calculus.name(), self.calculus.zones.len());
        out.adopt_terms(self.terms.clone());
        for scope in &self.scopes[1..] {
            out.scope(scope.parent)?;
        }
        let mut offset = vec![u32::MAX; self.scopes.len()];
        if offered {
            self.offer(&mut out, 0, &mut offset)?;
        }
        let mut places = vec![u32::MAX; self.nodes.len()];
        // The derived lines first, a line before every line that cites it,
        // then the tree from the root, each in postorder by a stack of
        // (node, whether its premises are done). A line cited twice is one
        // node, and a goal closed by a line is that line's node.
        let roots = (0..self.nodes.len() as u32)
            .filter(|&n| self.nodes[n as usize].line)
            .chain(std::iter::once(0));
        for root in roots {
            let mut stack = vec![(root, false)];
            while let Some((node, done)) = stack.pop() {
                if places[node as usize] != u32::MAX {
                    continue;
                }
                let goal = &self.nodes[node as usize];
                if let Done::Line(line) = goal.done {
                    places[node as usize] = places[line as usize];
                    if places[node as usize] == u32::MAX {
                        return Err(Error::InconsistentSession {
                            reason: "a goal is closed by a line the session does not hold",
                        });
                    }
                    continue;
                }
                if !done {
                    stack.push((node, true));
                    stack.extend(goal.premises.iter().rev().map(|&p| (p, false)));
                    continue;
                }
                let premises: Vec<u32> =
                    goal.premises.iter().map(|&p| places[p as usize]).collect();
                let (step, at) = match &goal.done {
                    Done::Open => (Step::Open, &[][..]),
                    Done::Hypothesis => (Step::Hypothesis, &[][..]),
                    Done::Rule(name, at) => (Step::Rule(name), &at[..]),
                    Done::Line(_) => unreachable!("a goal closed by a line has its line's place"),
                };
                if offered {
                    self.offer(&mut out, goal.scope, &mut offset)?;
                    // A subproof no node lies in, its goal closed by a
                    // hypothesis from outside, opens before its rule.
                    for (scope, opened) in self.scopes.iter().enumerate() {
                        if opened.owner.is_some_and(|(owner, _)| owner == node) {
                            self.offer(&mut out, scope as u32, &mut offset)?;
                        }
                    }
                }
                let id = out.push(
                    Node::new(&goal.sequent, step)
                        .premises(&premises)
                        .positions(at)
                        .scope(ScopeId::new(goal.scope)),
                )?;
                places[node as usize] = id.index() as u32;
            }
        }
        Ok((out, places, offset))
    }

    /// Pushes the hypotheses that the scope at `scope` and every scope
    /// around it offer, the outermost first, for each scope not pushed
    /// yet, and records in `at` where each one's start.
    fn offer(&self, out: &mut Derivation, scope: u32, at: &mut [u32]) -> Result<(), Error> {
        for outer in self.chain(scope).into_iter().rev() {
            if at[outer as usize] != u32::MAX {
                continue;
            }
            at[outer as usize] = out.len() as u32;
            for hypothesis in &self.scopes[outer as usize].hypotheses {
                out.push(Node::new(hypothesis, Step::Hypothesis).scope(ScopeId::new(outer)))?;
            }
        }
        Ok(())
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
                Error::InvalidDerivation { reason, .. } => refused(StepFault::Check { reason }),
                other => other,
            })
    }

    /// Checks every closed node of the session, the open goals taken as
    /// they are.
    fn verify_all(&self) -> Result<(), Error> {
        let (derivation, _) = self.derived()?;
        derivation
            .check_open(&self.calculus, &self.limits, |_| false)
            .map_err(|error| match error {
                Error::InvalidDerivation { reason, .. } => refused(StepFault::Check { reason }),
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
        Ok(self
            .candidates(goal)?
            .into_iter()
            .filter_map(|candidate| {
                let needs = match candidate.status {
                    Status::Applies => Vec::new(),
                    Status::NeedsInput { needs } => needs,
                    Status::Refused(_) => return None,
                };
                Some(Applicable {
                    rule: candidate.rule,
                    at: candidate.at,
                    needs,
                })
            })
            .collect())
    }

    /// Lists every rule of the calculus with whether a step of it is
    /// legal on the goal `goal` as it stands: for each choice of the
    /// members it acts on, the step if the checker takes it or what it
    /// needs given, and for a rule that no choice makes legal one entry
    /// with the refusal at its first choice (with no member to act on,
    /// [`StepFault::NoPlace`]). Each choice costs one step tried on a copy
    /// of the session. A rule that asks positions of more members than
    /// [`RULE_CHOICES`](Self::RULE_CHOICES) choices allow is left out, as
    /// [`rules`](Self::rules) leaves it out.
    ///
    /// # Errors
    ///
    /// [`Error::StepRefused`] for a goal that is not open, and a refusal
    /// of a bound.
    pub fn candidates(&self, goal: GoalId) -> Result<Vec<Candidate>, Error> {
        self.open(goal)?;
        let mut out: Vec<Candidate> = Vec::new();
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
            let mut legal = false;
            let mut first: Option<Error> = None;
            let mut seen: Vec<Vec<TermId>> = Vec::new();
            for pick in 0..count {
                let mut at = Vec::with_capacity(choices.len());
                let mut rest = pick;
                for choice in &choices {
                    at.push(choice[rest % choice.len()]);
                    rest /= choice.len();
                }
                // Equal members at other places give the same step.
                let key: Vec<TermId> = at.iter().map(|&n| flat[n as usize].1).collect();
                if seen.contains(&key) {
                    continue;
                }
                let step = Apply::new(name).at(&at);
                let mut trial = self.clone();
                let status = match trial.plan(goal, rule, &step) {
                    // A step a bound refuses cannot be taken either.
                    Ok(plan) => match trial.commit(goal, name, plan) {
                        Ok(_) => Status::Applies,
                        Err(error) => Status::Refused(error),
                    },
                    Err(Unplanned::Needs(needs)) => Status::NeedsInput { needs },
                    Err(Unplanned::Error(error @ Error::Refused(_))) => return Err(error),
                    Err(Unplanned::Error(error)) => Status::Refused(error),
                };
                if let Status::Refused(error) = status {
                    first.get_or_insert(error);
                    continue;
                }
                seen.push(key);
                legal = true;
                out.push(Candidate {
                    rule: name.clone(),
                    at,
                    status,
                });
            }
            if !legal {
                out.push(Candidate {
                    rule: name.clone(),
                    at: Vec::new(),
                    status: Status::Refused(first.unwrap_or_else(|| refused(StepFault::NoPlace))),
                });
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
            return Err(refused(StepFault::NoRule {
                rule: step.rule.to_owned(),
            }));
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
        self.within_occurrences(plan.premises.iter().map(|(s, _)| s.as_slice()))?;
        let scope = self.nodes[goal.index()].scope;
        let mut premises = Vec::with_capacity(plan.premises.len());
        // Every premise's shape first, so that a refusal leaves nothing.
        for (sequent, _) in &plan.premises {
            self.shaped(sequent)?;
        }
        for (index, (sequent, discharges)) in plan.premises.into_iter().enumerate() {
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
                line: false,
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

    /// Refuses a step that would make the session's nodes hold more
    /// members than `limits.occurrences`, with the sequents it adds.
    fn within_occurrences<'s>(
        &self,
        added: impl Iterator<Item = &'s [Vec<TermId>]>,
    ) -> Result<(), Error> {
        let size = |s: &[Vec<TermId>]| s.iter().map(Vec::len).sum::<usize>() as u64;
        let total = self
            .nodes
            .iter()
            .map(|n| size(&n.sequent))
            .chain(added.map(size))
            .fold(0u64, u64::saturating_add);
        match self.limits.occurrences {
            Some(limit) if total > limit => Err(Error::Refused(Refusal::Occurrences {
                occurrences: total,
                limit,
            })),
            _ => Ok(()),
        }
    }

    /// Applies a rule forwards at the goal `goal`: from lines in force as
    /// its premises, the rule's conclusion becomes a new line in the
    /// goal's scope, which later steps there and in the subproofs inside
    /// it cite, and which closes a goal equal to it
    /// ([`assume`](Self::assume), [`cite`](Self::cite)). It returns the
    /// new line's number; the goal stays open.
    ///
    /// The lines are matched against the premises as each is written. A
    /// variable only the conclusion holds must be given, and one the rule
    /// asks to be new gets a new atom where it is not given. A rule whose
    /// premise discharges hypotheses applies backwards only, which opens
    /// its subproof ([`apply`](Self::apply)), and so does one whose
    /// sequents have a context, which applies forwards to a goal's
    /// members instead ([`extend`](Self::extend)). The checker checks the
    /// step before the state takes it.
    ///
    /// # Errors
    ///
    /// [`Error::StepNeeds`] naming what must be given,
    /// [`Error::StepRefused`] for a line not in force at the goal or a
    /// step the calculus does not allow (the reason given),
    /// [`Error::Parse`] for a given text that is no term, and a refusal of
    /// a bound.
    pub fn derive(&mut self, goal: GoalId, step: &Derive<'_>) -> Result<LineId, Error> {
        let scope = self.open(goal)?.scope;
        let Some(&index) = self.calculus.rule_ids.get(step.rule) else {
            return Err(refused(StepFault::NoRule {
                rule: step.rule.to_owned(),
            }));
        };
        let rule = self.calculus.rules[index as usize].clone();
        if rule.premises.iter().any(|p| !p.discharges.is_empty()) {
            return Err(refused(StepFault::Discharges {
                rule: step.rule.to_owned(),
            }));
        }
        if step.lines.len() != rule.premises.len() {
            return Err(refused(StepFault::Cited {
                rule: step.rule.to_owned(),
                premises: rule.premises.len(),
                lines: step.lines.len(),
            }));
        }
        let places = step
            .lines
            .iter()
            .map(|&line| self.in_force(goal, line))
            .collect::<Result<Vec<_>, _>>()?;
        let cited: Vec<Sequent> = places
            .iter()
            .map(|&place| self.place_sequent(place).to_vec())
            .collect();
        let (sequent, at) = match self.forward(&rule, &cited, step) {
            Ok(planned) => planned,
            Err(Unplanned::Needs(needs)) => return Err(Error::StepNeeds { needs }),
            Err(Unplanned::Error(error)) => return Err(error),
        };
        self.shaped(&sequent)?;
        let copies: Vec<Sequent> = places
            .iter()
            .filter(|place| matches!(place, Place::Offered { .. }))
            .map(|&place| self.place_sequent(place).to_vec())
            .collect();
        self.within_occurrences(
            std::iter::once(sequent.as_slice()).chain(copies.iter().map(Vec::as_slice)),
        )?;
        // The line first, then a hypothesis node for each hypothesis it
        // cites, so that the step's nodes are the last the session holds.
        let line = self.nodes.len() as u32;
        self.nodes.push(Goal {
            sequent,
            done: Done::Rule(step.rule.to_owned(), at),
            premises: Vec::new(),
            scope,
            opened: scope,
            line: true,
        });
        let mut premises = Vec::with_capacity(places.len());
        for place in places {
            match place {
                Place::Derived(node) => premises.push(node),
                Place::Offered { scope: at, index } => {
                    premises.push(self.nodes.len() as u32);
                    let sequent = self.scopes[at as usize].hypotheses[index].clone();
                    self.nodes.push(Goal {
                        sequent,
                        done: Done::Hypothesis,
                        premises: Vec::new(),
                        scope: at,
                        opened: scope,
                        line: false,
                    });
                }
            }
        }
        self.nodes[line as usize].premises = premises;
        // Every closed node: a hypothesis node joins its scope, which the
        // rule that discharges it checks.
        if let Err(error) = self.verify_all() {
            self.nodes.truncate(line as usize);
            return Err(error);
        }
        self.history.push(line);
        Ok(LineId(self.line_places().len() as u32))
    }

    /// Applies a rule forwards to the members of the goal `goal`, in a
    /// sequent calculus: the members `step.at` names are what the rule
    /// reasons from, and what it derives joins them. It returns the goal
    /// that holds them, the old goal's one premise.
    ///
    /// A sequent calculus's rules have contexts, which lines cannot tell
    /// apart ([`derive`](Self::derive)), but a rule whose one premise is
    /// its conclusion with members added to the first zone reads forwards
    /// as it is: in Negri's labelled calculus, `L□` derives `y : A` from
    /// `x : □A` and `x R y`, and `Trans` derives `x R z` from `x R y` and
    /// `y R z`. The step is that rule's backward step on `goal`, so the
    /// derivation is the same as [`apply`](Self::apply) makes. A rule with
    /// another number of premises, one that acts on a member of another
    /// zone (a right rule), one that drops a member of the goal, one that
    /// adds to another zone and one that adds nothing the goal lacks apply
    /// backwards only.
    ///
    /// # Errors
    ///
    /// As [`apply`](Self::apply), and [`Error::StepRefused`] for a rule
    /// that does not read forwards, with the reason.
    pub fn extend(&mut self, goal: GoalId, step: &Apply<'_>) -> Result<GoalId, Error> {
        let before = self.open(goal)?.sequent.clone();
        let Some(&index) = self.calculus.rule_ids.get(step.rule) else {
            return Err(refused(StepFault::NoRule {
                rule: step.rule.to_owned(),
            }));
        };
        let rule = self.calculus.rules[index as usize].clone();
        let backwards = |why: Backwards| {
            Err(refused(StepFault::Backwards {
                rule: step.rule.to_owned(),
                why,
            }))
        };
        match rule.premises.as_slice() {
            [] => return backwards(Backwards::Closes),
            [premise] if !premise.discharges.is_empty() => {
                return backwards(Backwards::Discharges);
            }
            [_] => {}
            premises => {
                return backwards(Backwards::Premises {
                    premises: premises.len(),
                });
            }
        }
        // A zone by the mark around it, `after ⊢` and `before ⊢`, else
        // by its name.
        let marks: Vec<Option<String>> = self
            .calculus
            .marks()
            .map(|mark| mark.map(str::to_owned))
            .collect();
        let names: Vec<String> = self.calculus.zones().map(str::to_owned).collect();
        let zone = |z: usize| match (&marks[z], marks.get(z + 1).and_then(Option::as_ref)) {
            (Some(mark), _) => format!("after {mark}"),
            (None, Some(mark)) => format!("before {mark}"),
            (None, None) => format!("in the zone {}", names[z]),
        };
        // A forward step reasons from hypotheses: a rule that acts on a
        // member of another zone (a right rule, `R¬` whose premise only
        // adds before `⊢` among them) is goal-directed.
        if let Some(z) = (1..rule.conclusion.zones.len()).find(|&z| {
            rule.conclusion.zones[z]
                .iter()
                .any(|entry| matches!(entry, Entry::Item(_)))
        }) {
            return backwards(Backwards::ActsOn {
                zone: zone(z),
                first: zone(0),
            });
        }
        let plan = match self.plan(goal, &rule, step) {
            Ok(plan) => plan,
            Err(Unplanned::Needs(needs)) => return Err(Error::StepNeeds { needs }),
            Err(Unplanned::Error(error)) => return Err(error),
        };
        let mut gained = false;
        for (z, (old, new)) in before.iter().zip(&plan.premises[0].0).enumerate() {
            // Counted as multisets: a set zone holds each member once.
            let (mut old, mut new) = (old.clone(), new.clone());
            old.sort_unstable();
            new.sort_unstable();
            let mut at = 0;
            for member in &old {
                while at < new.len() && new[at] < *member {
                    at += 1;
                }
                if at == new.len() || new[at] != *member {
                    let shown = self.calculus.show(&self.terms, *member);
                    return backwards(Backwards::Drops {
                        member: shown.to_string(),
                    });
                }
                at += 1;
            }
            if new.len() > old.len() {
                if z > 0 {
                    return backwards(Backwards::Adds {
                        zone: zone(z),
                        first: zone(0),
                    });
                }
                gained = true;
            }
        }
        if !gained {
            return Err(refused(StepFault::NothingNew {
                rule: step.rule.to_owned(),
            }));
        }
        let opened = self.commit(goal, step.rule, plan)?;
        Ok(opened[0])
    }

    /// Plans a forward step of `rule` from the sequents `cited`: the
    /// conclusion and the positions the node gives.
    fn forward(
        &mut self,
        rule: &Rule,
        cited: &[Sequent],
        step: &Derive<'_>,
    ) -> Result<(Sequent, Vec<u32>), Unplanned> {
        let zones = self.calculus.zones.len();
        let contexts = rule
            .conclusion
            .zones
            .iter()
            .chain(rule.premises.iter().flat_map(|p| p.shape.zones.iter()))
            .flatten()
            .any(|entry| matches!(entry, Entry::Context { .. }));
        if contexts {
            return Err(refused(StepFault::Contexts {
                rule: step.rule.to_owned(),
            })
            .into());
        }
        let mut bindings: Vec<Option<TermId>> = vec![None; self.calculus.variables as usize];
        let mut premise_positions = Vec::new();
        for (k, (premise, line)) in rule.premises.iter().zip(cited).enumerate() {
            let unlike = || {
                refused(StepFault::NoInstance {
                    line: step.lines[k].0,
                    premise: k + 1,
                    rule: step.rule.to_owned(),
                })
            };
            for (entries, members) in premise.shape.zones.iter().zip(line) {
                if entries.len() != members.len() {
                    return Err(unlike().into());
                }
                for (place, (entry, &member)) in entries.iter().zip(members).enumerate() {
                    let Entry::Item(item) = entry else {
                        return Err(unlike().into());
                    };
                    if !self.matches(rule, item.root, member, &mut bindings) {
                        return Err(unlike().into());
                    }
                    if item.find == Find::Position {
                        premise_positions.push(place as u32);
                    }
                }
            }
        }
        let mut unused = vec![None; self.calculus.contexts.len()];
        self.give(step.given, &mut bindings, &mut unused)?;
        self.fresh(rule, &mut bindings)?;
        let mut needs = Vec::new();
        let mut sequent: Sequent = vec![Vec::new(); zones];
        let mut positions = Vec::new();
        for (zone, entries) in rule.conclusion.zones.iter().enumerate() {
            for entry in entries {
                let Entry::Item(item) = entry else { continue };
                if let Some(term) =
                    self.instance(rule, item.first, item.root, &bindings, &mut needs)?
                {
                    if item.find == Find::Position {
                        positions.push(sequent[zone].len() as u32);
                    }
                    sequent[zone].push(term);
                }
            }
        }
        if !needs.is_empty() {
            needs.sort();
            needs.dedup();
            return Err(Unplanned::Needs(needs));
        }
        self.atomic(rule, &bindings)?;
        positions.extend(premise_positions);
        Ok((sequent, positions))
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

    /// Closes the goal `goal` by a line in force equal to it: a
    /// hypothesis that its scope or a scope around it offers, one of the
    /// session's assumptions, or a line a forward step derived there; the
    /// innermost scope first, and in a scope its hypotheses before its
    /// derived lines. The checker checks a hypothesis at the rule that
    /// discharges it.
    ///
    /// # Errors
    ///
    /// [`Error::StepRefused`] for a goal that no line in force is.
    pub fn assume(&mut self, goal: GoalId) -> Result<(), Error> {
        let scope = self.open(goal)?.scope;
        let places = self.line_places();
        let sequent = &self.nodes[goal.index()].sequent;
        let found = self.chain(scope).into_iter().find_map(|at| {
            places.iter().copied().find(|&place| {
                self.place_scope(place) == at && self.same(self.place_sequent(place), sequent)
            })
        });
        match found {
            Some(place) => self.close_by(goal, place),
            None => Err(refused(StepFault::NoLineEqual)),
        }
    }

    /// Closes the goal `goal` by the line `line`, which must be in force
    /// at it and equal to it.
    ///
    /// # Errors
    ///
    /// [`Error::StepRefused`] for a goal that is not open, a line not in
    /// force at it, or one that is not the goal.
    pub fn cite(&mut self, goal: GoalId, line: LineId) -> Result<(), Error> {
        self.open(goal)?;
        let place = self.in_force(goal, line)?;
        if !self.same(self.place_sequent(place), &self.nodes[goal.index()].sequent) {
            return Err(refused(StepFault::NotGoal {
                line: line.0,
                goal: goal.0,
            }));
        }
        self.close_by(goal, place)
    }

    /// Closes the goal `goal` by the line at `place`, every closed node
    /// checked again: the rule that discharges a hypothesis checks it, and
    /// a rule that asks a name to be new checks it against the hypotheses
    /// in force, this one now among them. An assumption is the session's
    /// own.
    fn close_by(&mut self, goal: GoalId, place: Place) -> Result<(), Error> {
        let node = &mut self.nodes[goal.index()];
        match place {
            Place::Offered { scope, .. } => {
                node.done = Done::Hypothesis;
                node.scope = scope;
            }
            Place::Derived(line) => node.done = Done::Line(line),
        }
        if let Err(error) = self.verify_all() {
            let node = &mut self.nodes[goal.index()];
            node.done = Done::Open;
            node.scope = node.opened;
            return Err(error);
        }
        self.history.push(goal.0);
        Ok(())
    }

    /// Takes back the last step and returns what it took back: the goal
    /// the step closed, which is open again, or the line a forward step
    /// derived, which is gone; `None` where no step is left.
    pub fn undo(&mut self) -> Option<Undone> {
        let goal = GoalId(self.history.pop()?);
        let node = &self.nodes[goal.index()];
        if node.line {
            let line = LineId(self.line_places().len() as u32);
            self.nodes.truncate(goal.index());
            return Some(Undone::Removed(line));
        }
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
            Done::Hypothesis | Done::Line(_) | Done::Open => (self.nodes.len(), self.scopes.len()),
        };
        self.back(goal, before);
        Some(Undone::Reopened(goal))
    }

    /// Returns the derivation built so far, its open goals as
    /// [`Step::Open`] nodes.
    ///
    /// # Errors
    ///
    /// A refusal where the derivation's indices would pass a `u32`.
    pub fn derivation(&self) -> Result<Derivation, Error> {
        self.derived().map(|(derivation, _)| derivation)
    }

    /// Returns the derivation built so far with every hypothesis its
    /// scopes offer as a node, ahead of the rest, whether a step cites it
    /// yet or not: what a [`Fitch`](super::Fitch) proof of the session
    /// takes, so that each subproof opens with its hypotheses, an unused
    /// one too. The derivation cites a hypothesis by a node per use, and
    /// the Fitch proof shows those of one scope with equal formulas as one
    /// line; the proof tree takes [`derivation`](Self::derivation).
    ///
    /// Beside it, at index `n - 1`, the node that shows the session's line
    /// `n`: a hypothesis by its first node, a derived line by its own. The
    /// Fitch proof's line showing that node ([`Fitch::nodes`](super::Fitch::nodes))
    /// is where the line stands in it, numbered otherwise.
    ///
    /// # Errors
    ///
    /// As [`derivation`](Self::derivation).
    pub fn fitch_derivation(&self) -> Result<(Derivation, Vec<NodeId>), Error> {
        let (derivation, places, at) = self.derived_with(true)?;
        let lines = self
            .line_places()
            .into_iter()
            .map(|place| {
                let index = match place {
                    // The first equal hypothesis of the scope: the line the
                    // Fitch proof shows for every one of them.
                    Place::Offered { scope, index } => {
                        let offered = &self.scopes[scope as usize].hypotheses;
                        let first = offered
                            .iter()
                            .position(|h| *h == offered[index])
                            .unwrap_or(index);
                        let start = at[scope as usize];
                        if start == u32::MAX {
                            return Err(Error::InconsistentSession {
                                reason: "a scope's hypotheses were not drawn",
                            });
                        }
                        start as usize + first
                    }
                    Place::Derived(node) => places[node as usize] as usize,
                };
                Ok(derivation.at(index).id())
            })
            .collect::<Result<_, Error>>()?;
        Ok((derivation, lines))
    }

    /// Returns, for each node of [`derivation`](Self::derivation) in its
    /// order, the node of the session it is, as the id the session's calls
    /// take for it while it is an open goal. A drawing of the derivation
    /// ([`Drawing`](super::Drawing)) numbers its inferences in the same
    /// order while the session has no derived line; a line cited twice is
    /// drawn twice, and [`Drawing::nodes`](super::Drawing::nodes) gives
    /// the node of each inference.
    ///
    /// # Errors
    ///
    /// As [`derivation`](Self::derivation).
    pub fn derivation_goals(&self) -> Result<Vec<GoalId>, Error> {
        let (derivation, places) = self.derived()?;
        let mut goals = vec![GoalId(0); derivation.len()];
        for (node, &place) in places.iter().enumerate() {
            // A goal closed by a line is that line's node.
            if !matches!(self.nodes[node].done, Done::Line(_)) {
                goals[place as usize] = GoalId(node as u32);
            }
        }
        Ok(goals)
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
                            return Err(refused(StepFault::TooFew).into());
                        };
                        match flat.get(number as usize) {
                            Some(&(z, p)) if z == zone => Some(p),
                            _ => {
                                return Err(refused(StepFault::Zone {
                                    member: number,
                                    zone: calculus.zones[zone].name.clone(),
                                })
                                .into());
                            }
                        }
                    }
                };
                let Some(place) = place.filter(|&p| p < members.len()) else {
                    return Err(refused(StepFault::NoPlace).into());
                };
                if calculus.zones[zone].discipline != Discipline::Set && taken[zone][place] {
                    return Err(refused(StepFault::SamePlace).into());
                }
                if !self.matches(rule, item.root, members[place], &mut bindings) {
                    return Err(refused(StepFault::Unmatched {
                        member: calculus
                            .show(&self.terms, members[place])
                            .limit(200)
                            .to_string(),
                    })
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
            return Err(refused(StepFault::TooMany).into());
        }
        self.give(step.given, &mut bindings, &mut contexts)?;
        self.bind_premises(rule, step.premises, &mut bindings)?;
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
                    return Err(refused(StepFault::Order).into());
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
                        .ok_or_else(|| refused(StepFault::Context))?;
                }
                match unknown.as_slice() {
                    [] if rest.is_empty() => {}
                    [] => {
                        return Err(refused(StepFault::Unaccounted).into());
                    }
                    [(variable, under)] => {
                        let value: Option<Vec<TermId>> =
                            rest.iter().map(|&m| self.unmap(m, *under)).collect();
                        let value = value.ok_or_else(|| refused(StepFault::NotUnder))?;
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
        self.fresh(rule, &mut bindings)?;
        // The premises.
        let mut premises = Vec::with_capacity(rule.premises.len());
        let mut premise_positions = Vec::new();
        for premise in &rule.premises {
            let mut built = vec![Vec::new(); zones];
            let mut pending: Vec<(usize, usize)> = Vec::new();
            for zone in 0..zones {
                for entry in &premise.shape.zones[zone] {
                    match *entry {
                        Entry::Item(item) => {
                            if let Some(term) =
                                self.instance(rule, item.first, item.root, &bindings, &mut needs)?
                            {
                                if item.find == Find::Position {
                                    pending.push((zone, built[zone].len()));
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
                    // A member written twice stands at its first place.
                    for slot in pending.iter_mut().filter(|(z, _)| *z == zone) {
                        let term = built[zone][slot.1];
                        slot.1 = built[zone]
                            .iter()
                            .position(|&m| m == term)
                            .unwrap_or(slot.1);
                    }
                    let mut seen = Vec::new();
                    let mut kept = Vec::new();
                    for (place, &m) in built[zone].iter().enumerate() {
                        if !seen.contains(&m) {
                            seen.push(m);
                            kept.push(place);
                        }
                    }
                    for slot in pending.iter_mut().filter(|(z, _)| *z == zone) {
                        slot.1 = kept.iter().position(|&p| p == slot.1).unwrap_or(slot.1);
                    }
                    built[zone] = seen;
                }
            }
            // Each member found by position stands where it was built.
            premise_positions.extend(pending.iter().map(|&(_, place)| place as u32));
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
        self.atomic(rule, &bindings)?;
        positions.extend(premise_positions);
        Ok(Plan {
            premises,
            at: positions,
        })
    }

    /// Parses the values given for a step's variables and contexts into
    /// `bindings` and `contexts`.
    fn give(
        &mut self,
        given: &[(&str, &str)],
        bindings: &mut [Option<TermId>],
        contexts: &mut [Option<Vec<TermId>>],
    ) -> Result<(), Unplanned> {
        let calculus = self.calculus.clone();
        for &(name, text) in given {
            if let Some(v) = calculus.variable_names.iter().position(|(n, _)| n == name) {
                let sort = calculus.sort_name(calculus.variable_names[v].1).to_owned();
                let term = calculus.parse_term(&mut self.terms, text, &sort, &self.limits)?;
                match bindings[v] {
                    Some(bound) if bound != term => {
                        return Err(refused(StepFault::Bound {
                            name: name.to_owned(),
                            value: calculus.show(&self.terms, bound).limit(200).to_string(),
                        })
                        .into());
                    }
                    _ => bindings[v] = Some(term),
                }
            } else if let Some(c) = calculus.context_names.iter().position(|(n, _)| n == name) {
                let Some(sort) = calculus.context_names[c].1 else {
                    return Err(refused(StepFault::Unused {
                        name: name.to_owned(),
                    })
                    .into());
                };
                let members = calculus.parse_list(&mut self.terms, text, sort, &self.limits)?;
                contexts[c] = Some(members);
            } else {
                return Err(refused(StepFault::Unknown {
                    name: name.to_owned(),
                })
                .into());
            }
        }
        Ok(())
    }

    /// Binds what the premises' first members hold, matching each against
    /// its premise's first written member, in the rule's order; a member
    /// that does not match refuses the step.
    fn bind_premises(
        &self,
        rule: &Rule,
        premises: &[TermId],
        bindings: &mut [Option<TermId>],
    ) -> Result<(), Unplanned> {
        for (premise, &term) in rule.premises.iter().zip(premises) {
            let item = premise
                .shape
                .zones
                .iter()
                .flatten()
                .find_map(|entry| match entry {
                    Entry::Item(item) => Some(item),
                    Entry::Context { .. } => None,
                });
            if let Some(item) = item
                && !self.matches(rule, item.root, term, bindings)
            {
                return Err(refused(StepFault::Unmatched {
                    member: self.calculus.show(&self.terms, term).limit(200).to_string(),
                })
                .into());
            }
        }
        Ok(())
    }

    /// Returns, by name, the variables of `rule` that the premises' first
    /// members `premises` bind, as [`Apply`]'s premises bind them.
    ///
    /// # Errors
    ///
    /// [`Error::StepRefused`] for an unknown rule or a member that does not
    /// match its premise.
    pub(super) fn premise_bindings(
        &self,
        rule: &str,
        premises: &[TermId],
    ) -> Result<Vec<(String, TermId)>, Error> {
        let Some(&index) = self.calculus.rule_ids.get(rule) else {
            return Err(refused(StepFault::NoRule {
                rule: rule.to_owned(),
            }));
        };
        let rule = &self.calculus.rules[index as usize];
        let mut bindings: Vec<Option<TermId>> = vec![None; self.calculus.variables as usize];
        match self.bind_premises(rule, premises, &mut bindings) {
            Ok(()) => {}
            Err(Unplanned::Error(error)) => return Err(error),
            Err(Unplanned::Needs(needs)) => return Err(Error::StepNeeds { needs }),
        }
        Ok(bindings
            .iter()
            .enumerate()
            .filter_map(|(v, bound)| Some((self.calculus.variable_names[v].0.clone(), (*bound)?)))
            .collect())
    }

    /// Binds every variable the rule asks to be new and that is not bound
    /// to a new atom.
    fn fresh(&mut self, rule: &Rule, bindings: &mut [Option<TermId>]) -> Result<(), Error> {
        let calculus = self.calculus.clone();
        for &variable in &rule.fresh {
            if bindings[variable as usize].is_none() {
                let (name, sort) = calculus.variable_names[variable as usize].clone();
                let atom = self.new_atom(&name, sort)?;
                bindings[variable as usize] = Some(atom);
            }
        }
        Ok(())
    }

    /// Refuses a binding of a variable the rule asks to be an atom that is
    /// no atom.
    fn atomic(&self, rule: &Rule, bindings: &[Option<TermId>]) -> Result<(), Unplanned> {
        let calculus = &self.calculus;
        for &variable in &rule.atomic {
            let bound = bindings[variable as usize];
            if !bound.is_some_and(|t| matches!(self.terms.term(t), Term::Atom { .. })) {
                return Err(refused(StepFault::NotAtom {
                    name: calculus.variable_names[variable as usize].0.clone(),
                })
                .into());
            }
        }
        Ok(())
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
        // conclusion and the premise of exactly one node; beside it the
        // derived lines, each citing earlier lines any number of times.
        let mut parent: Vec<Option<(u32, u32)>> = vec![None; nodes.len()];
        for (index, node) in nodes.iter().enumerate() {
            if node.line {
                let discharges = match node.done {
                    Done::Rule(ref name, _) => calculus.rule_ids.get(name).is_some_and(|&r| {
                        rules[r as usize]
                            .premises
                            .iter()
                            .any(|p| !p.discharges.is_empty())
                    }),
                    Done::Open | Done::Hypothesis | Done::Line(_) => true,
                };
                if index == 0 || discharges || node.scope != node.opened {
                    return Err(fault(
                        "a derived line is the root, discharges, or is no rule's conclusion",
                    ));
                }
            }
            if let Done::Line(line) = node.done {
                let cited = nodes.get(line as usize);
                if node.line || !cited.is_some_and(|c| c.line) {
                    return Err(fault("a goal is closed by no derived line"));
                }
            }
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
                Done::Open | Done::Hypothesis | Done::Line(_) if node.premises.is_empty() => {}
                Done::Open | Done::Hypothesis | Done::Line(_) => {
                    return Err(fault("an open goal or a hypothesis has premises"));
                }
            }
            if node.done != Done::Hypothesis && node.scope != node.opened {
                return Err(fault("only a hypothesis leaves the scope it was opened in"));
            }
            for (k, &premise) in node.premises.iter().enumerate() {
                let p = premise as usize;
                // A line cites an earlier line; no other node cites one.
                if nodes.get(p).is_some_and(|cited| cited.line) {
                    if !node.line || p >= index {
                        return Err(fault("a derived line is cited by no later line"));
                    }
                    continue;
                }
                if p <= index || p >= nodes.len() || parent[p].is_some() {
                    return Err(fault("the nodes are no tree with its root first"));
                }
                parent[p] = Some((index as u32, k as u32));
            }
        }
        let orphan = parent
            .iter()
            .zip(&nodes)
            .skip(1)
            .any(|(up, node)| up.is_none() && !node.line);
        if orphan || nodes[0].opened != 0 {
            return Err(fault("the nodes are no tree with its root first"));
        }
        // Where each node was opened: its parent's scope, or the scope
        // its parent owns for a premise that discharges.
        for (index, node) in nodes.iter().enumerate().skip(1) {
            let Some((p, k)) = parent[index] else {
                // A derived line, opened in its own scope.
                continue;
            };
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
        // Each scope but the root is owned by a rule node whose premise
        // was opened in it, inside the node's scope, one scope a premise.
        for (index, scope) in scopes.iter().enumerate().skip(1) {
            let Some((n, k)) = scope.owner else {
                return Err(fault("a scope lies in no earlier scope, or has no owner"));
            };
            let owned = nodes.get(n as usize).and_then(|node| match node.done {
                Done::Rule(ref name, _) => {
                    let rule = &rules[calculus.rule_ids[name] as usize];
                    let premise = *node.premises.get(k as usize)?;
                    let discharges = !rule.premises.get(k as usize)?.discharges.is_empty();
                    Some(
                        discharges
                            && nodes[premise as usize].opened == index as u32
                            && scope.parent == node.scope,
                    )
                }
                Done::Open | Done::Hypothesis | Done::Line(_) => None,
            });
            let unique = scopes
                .iter()
                .filter(|other| other.owner == scope.owner)
                .count()
                == 1;
            if owned != Some(true) || !unique {
                return Err(fault(
                    "a scope's owner is no rule node whose premise opened it",
                ));
            }
        }
        // A hypothesis lies in the scope it was opened in or one around it.
        for node in &nodes {
            let mut scope = node.opened;
            while scope != node.scope && scope != 0 {
                scope = scopes[scope as usize].parent;
            }
            if scope != node.scope {
                return Err(fault(
                    "a hypothesis lies in no scope around where it was opened",
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
        // A goal closed by a line is that line, which is in force there.
        for node in &session.nodes {
            if let Done::Line(line) = node.done {
                let cited = &session.nodes[line as usize];
                if !session.same(&cited.sequent, &node.sequent)
                    || !session.chain(node.scope).contains(&cited.scope)
                {
                    return Err(fault("a goal is closed by a line that is not it in force"));
                }
            }
        }
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
            if node.line {
                // A forward step made the line and then the hypothesis
                // nodes it cites, the last nodes the session held.
                let copies = node.premises.iter().filter(|&&p| p as usize > g).count();
                let suffix = node
                    .premises
                    .iter()
                    .filter(|&&p| p as usize > g)
                    .enumerate()
                    .all(|(i, &p)| p as usize == g + 1 + i);
                if !suffix || g + 1 + copies != nodes_left {
                    return Err(fault("a step's goals or scopes are not the last it made"));
                }
                nodes_left = g;
                continue;
            }
            match node.done {
                Done::Open => return Err(fault("the history names an open goal")),
                Done::Hypothesis => {}
                // Undoing the forward step first would leave the goal
                // closed by a line that is gone.
                Done::Line(line) if (line as usize) < nodes_left => {}
                Done::Line(_) => {
                    return Err(fault(
                        "a goal closed by a line comes before the line's step",
                    ));
                }
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
