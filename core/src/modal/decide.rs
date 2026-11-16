// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Deciding a modal sequent in a system: the search, the derivation it
//! gives read into the system's labelled calculus and checked there, or
//! the countermodel it gives checked against the system.

use super::calculus::{self, COLON, LABEL, RELATION};
use super::search::{Found, Member, Search};
use super::{Countermodel, Node, NodeId, Sequent, System};
use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Limits, Progress, Refusal};
use crate::linear::search::Reason;
use crate::rules::{self, TermId};

/// How a modal sequent is decided: the system.
#[non_exhaustive]
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(default, deny_unknown_fields)]
pub struct Options {
    /// The system, K by default: the logic of every frame, whose valid
    /// sequents are valid in every system.
    pub system: System,
}

impl Options {
    /// Returns the options with the system given.
    #[must_use]
    pub const fn with_system(self, system: System) -> Self {
        Self { system }
    }
}

/// Whether a modal sequent is valid in its system.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Verdict {
    /// The sequent is valid, and here is the checked evidence.
    Valid(Evidence),
    /// The sequent is not valid; the outcome's countermodel shows it where
    /// it was small enough to check.
    NotValid,
    /// The search stopped before it could decide, for the reason given.
    Unknown(Reason),
}

/// What shows a modal sequent valid.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Evidence {
    /// A derivation in the system's labelled calculus
    /// ([`calculus`](super::calculus())), which the generic checker
    /// accepted. Its root is `w0 : Γ ⊢ w0 : Δ`.
    Derivation(Box<rules::Derivation>),
}

/// What the search counted.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Statistics {
    /// The worlds made, on every branch.
    pub worlds: u64,
    /// The branches split by a rule of two premises.
    pub splits: u64,
    /// The tests that found a world blocked by an ancestor (K4, KD4, S4).
    pub blocked: u64,
    /// The units of work: a member of a branch acted on, a pending
    /// formula or an ancestor looked at.
    pub work: u64,
    /// The inferences of the derivation of a valid sequent.
    pub inferences: u64,
}

/// The outcome of deciding a modal sequent.
///
/// # JSON
///
/// With the cargo feature `serialize` an outcome is written as `{"version":
/// 20, "deduxit": "…", "verdict": "valid", "system": "S4", "statistics":
/// {…}, "sequent": {…}, "derivation": {…}}`: `verdict` is `valid`,
/// `not_valid` or `unknown` (with `reason` as a linear outcome writes it),
/// `sequent` the sequent's form, `derivation` that of a
/// [`rules::Derivation`] without its `version`, and for `not_valid` by a
/// model `"countermodel": {"model": {…}}`, the Kripke model's form. It
/// reads back unchecked: the derivation's check
/// ([`rules::Derivation::check`] in [`calculus`](super::calculus())) and
/// the countermodel's ([`Countermodel::check`]) are the caller's.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Outcome {
    /// The verdict, with the evidence of a valid sequent.
    pub verdict: Verdict,
    /// The sequent decided.
    pub sequent: Sequent,
    /// The system it was decided in.
    pub system: System,
    /// The Kripke model that refutes a sequent that is not valid, checked
    /// against the system's frame conditions. `None` for every other
    /// verdict, and where the model is too large to check (past
    /// [`MODEL_CELLS`](crate::ordinary::MODEL_CELLS) or
    /// `limits.memory_bytes`).
    pub countermodel: Option<Countermodel>,
    /// What the search counted.
    pub statistics: Statistics,
}

/// Decides `sequent` in the system of `options` within the default
/// limits and without a stop: the search ends by its loop checks, and the
/// default memory bound ends it where it grows too large.
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::modal::{Options, Sequent, System, Verdict, decide};
///
/// let axiom: Sequent = "[]p -> [][]p".parse()?;
/// let s4 = decide(&axiom, &Options::default().with_system(System::S4))?;
/// assert!(matches!(s4.verdict, Verdict::Valid(_)));
/// let k = decide(&axiom, &Options::default())?;
/// assert!(matches!(k.verdict, Verdict::NotValid));
/// println!("{}", k.countermodel.unwrap().model);
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// # Errors
///
/// As [`decide_within`].
pub fn decide(sequent: &Sequent, options: &Options) -> Result<Outcome, Error> {
    decide_within(sequent, options, &Limits::default(), |_| false)
}

/// Decides `sequent` in the system of `options`, within `limits`, asking
/// `stop` as it goes.
///
/// The search is a labelled tableau for every system: its closed branches
/// are a derivation in the system's labelled calculus, which the generic
/// checker checks before "valid" is answered, and an open branch is a
/// Kripke model, which the model check checks against the system's frame
/// conditions and the sequent before "not valid" is answered. The search
/// counts its work against `limits.work` and its memory against
/// `limits.memory_bytes`; past either, or when the stop fires, the
/// verdict is unknown with the reason.
///
/// # Errors
///
/// [`Refusal::Output`] where the derivation would pass
/// `limits.derivation_bytes`, and [`Refusal::Memory`], [`Refusal::Work`]
/// and [`Refusal::Stopped`] where a bound or the stop refuses its check:
/// none of these is a verdict. [`Error::ReadBack`] where the checker
/// refuses the derivation and [`Error::Countermodel`] where the model
/// check refuses the model are defects of the search.
pub fn decide_within(
    sequent: &Sequent,
    options: &Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    let system = options.system;
    let mut search = Search::new(sequent, system, limits);
    let found = search.run(&mut stop);
    let counts = search.counts;
    let mut statistics = Statistics {
        worlds: counts.worlds,
        splits: counts.splits,
        blocked: counts.blocked,
        work: counts.work,
        inferences: 0,
    };
    let (verdict, countermodel) = match found {
        Err(reason) => (Verdict::Unknown(reason), None),
        Ok(Found::Open(model)) => {
            let countermodel =
                Countermodel::found(sequent.clone(), system, model, limits, &mut stop)?;
            (Verdict::NotValid, countermodel)
        }
        Ok(Found::Closed(root)) => {
            let derivation = derivation(&search, root, sequent, system, limits)?;
            statistics.inferences = derivation.len() as u64;
            let calculus = calculus::calculus(system, limits)?;
            derivation
                .check(&calculus, limits, &mut stop)
                .map_err(|error| {
                    if let Error::InvalidDerivation { reason, .. } = error {
                        Error::ReadBack {
                            calculus: calculus::name(system),
                            reason,
                        }
                    } else {
                        error
                    }
                })?;
            (
                Verdict::Valid(Evidence::Derivation(Box::new(derivation))),
                None,
            )
        }
    };
    Ok(Outcome {
        verdict,
        sequent: sequent.clone(),
        system,
        countermodel,
        statistics,
    })
}

/// The bytes of a node of a derivation besides its members.
const NODE_BYTES: u64 = 48;

/// The bytes of a member of a node's sequent.
const MEMBER_BYTES: u64 = 4;

/// The terms of a derivation in the making, each member's made once.
struct Terms<'a> {
    /// The derivation.
    derivation: rules::Derivation,
    /// The sequent's arena.
    sequent: &'a Sequent,
    /// The term of each formula of the arena, made on first use.
    formulas: Vec<Option<TermId>>,
    /// The term of each label, made on first use.
    labels: Vec<Option<TermId>>,
    /// The term of each member, made on first use.
    members: HashMap<Member, TermId>,
}

impl Terms<'_> {
    /// Returns the term of a formula of the arena.
    fn formula(&mut self, id: NodeId) -> Result<TermId, Error> {
        if let Some(term) = self.formulas[id.index()] {
            return Ok(term);
        }
        // Every operand precedes its formula, so one pass up to it makes
        // them all without recursion.
        for i in 0..=id.index() {
            if self.formulas[i].is_some() {
                continue;
            }
            let f = |k: NodeId, formulas: &[Option<TermId>]| {
                formulas[k.index()].unwrap_or_else(|| unreachable!("an operand comes first"))
            };
            let terms = self.derivation.terms_mut();
            let fs = &self.formulas;
            let term = match self.sequent.formulas().node(NodeId(i as u32)) {
                Node::Atom(a) => terms.atom(self.sequent.formulas().atom_name(a), "formula")?,
                Node::True => terms.apply("⊤", &[])?,
                Node::False => terms.apply("⊥", &[])?,
                Node::Not(a) => terms.apply("¬", &[f(a, fs)])?,
                Node::Box(a) => terms.apply("□", &[f(a, fs)])?,
                Node::Diamond(a) => terms.apply("◇", &[f(a, fs)])?,
                Node::And(a, b) => terms.apply("∧", &[f(a, fs), f(b, fs)])?,
                Node::Or(a, b) => terms.apply("∨", &[f(a, fs), f(b, fs)])?,
                Node::Implies(a, b) => terms.apply("→", &[f(a, fs), f(b, fs)])?,
                Node::Iff(a, b) => terms.apply("↔", &[f(a, fs), f(b, fs)])?,
            };
            self.formulas[i] = Some(term);
        }
        Ok(self.formulas[id.index()].unwrap_or_else(|| unreachable!("made above")))
    }

    /// Returns the term of a label, `w` and its number.
    fn label(&mut self, world: u32) -> Result<TermId, Error> {
        let i = world as usize;
        if i >= self.labels.len() {
            self.labels.resize(i + 1, None);
        }
        if let Some(term) = self.labels[i] {
            return Ok(term);
        }
        let term = self
            .derivation
            .terms_mut()
            .atom(&format!("w{world}"), LABEL)?;
        self.labels[i] = Some(term);
        Ok(term)
    }

    /// Returns the term of a member and its zone: `0` left of `⊢`, `1`
    /// right of it.
    fn member(&mut self, member: Member) -> Result<(usize, TermId), Error> {
        let zone = match member {
            Member::Formula { right, .. } => usize::from(right),
            Member::Edge(..) => 0,
        };
        if let Some(&term) = self.members.get(&member) {
            return Ok((zone, term));
        }
        let term = match member {
            Member::Formula { world, node, .. } => {
                let (label, formula) = (self.label(world)?, self.formula(NodeId(node))?);
                self.derivation
                    .terms_mut()
                    .apply(COLON, &[label, formula])?
            }
            Member::Edge(x, y) => {
                let (x, y) = (self.label(x)?, self.label(y)?);
                self.derivation.terms_mut().apply(RELATION, &[x, y])?
            }
        };
        self.members.insert(member, term);
        Ok((zone, term))
    }
}

/// Returns the derivation of the closed tableau whose root node is
/// `root`, in the system's labelled calculus: each node's sequent is the
/// root's and the members its steps below added, each zone sorted.
fn derivation(
    search: &Search<'_>,
    root: u32,
    sequent: &Sequent,
    system: System,
    limits: &Limits,
) -> Result<rules::Derivation, Error> {
    let (steps, proofs) = (&search.steps, &search.proofs);
    // The size first, by a walk that adds each premise's members to its
    // conclusion's count, before anything is made.
    let base = (sequent.left().len() + sequent.right().len()) as u64;
    let mut estimate = 0u64;
    let mut stack = vec![(root, base)];
    while let Some((node, size)) = stack.pop() {
        estimate = estimate
            .saturating_add(NODE_BYTES)
            .saturating_add(size.saturating_mul(MEMBER_BYTES));
        let proof = &proofs[node as usize];
        let step = &steps[proof.step as usize];
        for (i, &premise) in proof.premises.iter().enumerate() {
            stack.push((premise, size + step.premises[i].len() as u64));
        }
    }
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
    let mut terms = Terms {
        derivation: rules::Derivation::new(calculus::name(system), 2),
        sequent,
        formulas: vec![None; sequent.formulas().len()],
        labels: Vec::new(),
        members: HashMap::default(),
    };
    // The current sequent, each zone sorted.
    let mut zones: [Vec<TermId>; 2] = [Vec::new(), Vec::new()];
    let root_members = sequent
        .left()
        .iter()
        .map(|&id| super::search::formula(0, id, false))
        .chain(
            sequent
                .right()
                .iter()
                .map(|&id| super::search::formula(0, id, true)),
        );
    for member in root_members {
        let (zone, term) = terms.member(member)?;
        if let Err(at) = zones[zone].binary_search(&term) {
            zones[zone].insert(at, term);
        }
    }
    /// A node of the walk: the proof node, the next premise to visit, the
    /// members the premise visited last added, and the nodes made of the
    /// premises so far.
    struct Visit {
        /// The proof node.
        node: u32,
        /// The next premise.
        next: usize,
        /// What the last premise added to the sequent.
        added: Vec<(usize, TermId)>,
        /// The derivation's nodes of the premises visited.
        made: Vec<u32>,
    }
    let mut walk = vec![Visit {
        node: root,
        next: 0,
        added: Vec::new(),
        made: Vec::new(),
    }];
    loop {
        let Some(top) = walk.last_mut() else {
            unreachable!("the walk ends at the root")
        };
        for (zone, term) in top.added.drain(..) {
            if let Ok(at) = zones[zone].binary_search(&term) {
                zones[zone].remove(at);
            }
        }
        let proof = &proofs[top.node as usize];
        let step = &steps[proof.step as usize];
        if let Some(&premise) = proof.premises.get(top.next) {
            let mut added = Vec::new();
            for &member in &step.premises[top.next] {
                let (zone, term) = terms.member(member)?;
                if let Err(at) = zones[zone].binary_search(&term) {
                    zones[zone].insert(at, term);
                    added.push((zone, term));
                }
            }
            let Some(top) = walk.last_mut() else {
                unreachable!("the walk has a top")
            };
            top.added = added;
            top.next += 1;
            walk.push(Visit {
                node: premise,
                next: 0,
                added: Vec::new(),
                made: Vec::new(),
            });
            continue;
        }
        // Every premise is made: the node itself, its sequent the current
        // one.
        let mut positions = Vec::with_capacity(step.written.len() + 1);
        for &member in &step.written {
            let (zone, term) = terms.member(member)?;
            let at = zones[zone]
                .binary_search(&term)
                .unwrap_or_else(|_| unreachable!("a step's members are in its conclusion"));
            positions.push(at as u32);
        }
        if let Some(member) = step.located {
            // Its place in the first premise: the conclusion's members
            // below it, and the premise's added ones.
            let (zone, term) = terms.member(member)?;
            let mut place = zones[zone].partition_point(|&t| t < term);
            for &other in &step.premises[0] {
                let (z, t) = terms.member(other)?;
                if z == zone && t < term && zones[zone].binary_search(&t).is_err() {
                    place += 1;
                }
            }
            positions.push(place as u32);
        }
        let Some(top) = walk.pop() else {
            unreachable!("the walk has a top")
        };
        let made = terms.derivation.push(
            rules::Node::new(&zones, rules::Step::Rule(step.rule.name()))
                .premises(&top.made)
                .positions(&positions),
        )?;
        match walk.last_mut() {
            Some(parent) => parent.made.push(made.index() as u32),
            None => return Ok(terms.derivation),
        }
    }
}
