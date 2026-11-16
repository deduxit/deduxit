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
use crate::limits::{Limits, Phase, Progress, Refusal};
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
    /// The tests that found a world blocked by an earlier world (K4, KD4,
    /// S4).
    pub blocked: u64,
    /// The units of work: a member of a branch acted on, a pending
    /// formula or an earlier world looked at.
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
/// reads back unchecked: [`check`](Self::check) checks its evidence
/// against its sequent and system.
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

impl Outcome {
    /// Checks the outcome's evidence against its sequent and system, as an
    /// outcome read from JSON needs: a derivation concludes `w0 : Γ ⊢ w0 :
    /// Δ` for the sequent `Γ ⊢ Δ`, rests on no assumption, and passes the
    /// checker in the system's calculus ([`calculus`](super::calculus()));
    /// a countermodel refutes the sequent in the system
    /// ([`Countermodel::check`]), the countermodel's sequent the outcome's
    /// own (the same text). An outcome without evidence (unknown, or not
    /// valid without a model) has nothing to check.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidDerivation`] for a derivation of another sequent, or
    /// one that rests on an assumption, or one the checker refuses;
    /// [`Error::Model`] for a model that is none of the system or does not
    /// refute the sequent; [`Error::Refused`] where a bound or the stop
    /// refuses a check.
    pub fn check(
        &self,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        if let Some(countermodel) = &self.countermodel {
            if countermodel.sequent.to_string() != self.sequent.to_string() {
                return Err(Error::Model {
                    reason: format!(
                        "the model is said to refute {}, the outcome's sequent is {}",
                        countermodel.sequent, self.sequent
                    ),
                });
            }
            if countermodel.system != self.system {
                return Err(Error::Model {
                    reason: format!(
                        "the model is said to be one of {}, the outcome's system is {}",
                        countermodel.system, self.system
                    ),
                });
            }
            countermodel.check(limits, &mut stop)?;
        }
        let Verdict::Valid(Evidence::Derivation(derivation)) = &self.verdict else {
            return Ok(());
        };
        let refused = |reason: &str| Error::InvalidDerivation {
            calculus: derivation.calculus().into(),
            reason: reason.to_owned(),
        };
        if derivation.calculus() != calculus::name(self.system) {
            return Err(refused("it is in another system's calculus"));
        }
        if derivation.assumptions().next().is_some() {
            return Err(refused("it rests on an assumption"));
        }
        let root = derivation.root().ok_or_else(|| refused("it has no node"))?;
        // The root the sequent asks for, its terms made in a copy so that
        // a term the derivation lacks shows as a difference.
        let mut terms = Terms {
            derivation: (**derivation).clone(),
            sequent: &self.sequent,
            formulas: Vec::new(),
            labels: Vec::new(),
            members: HashMap::default(),
        };
        let mut zones: [Vec<TermId>; 2] = [Vec::new(), Vec::new()];
        let members = self
            .sequent
            .left()
            .iter()
            .map(|&id| super::search::formula(0, id, false))
            .chain(
                self.sequent
                    .right()
                    .iter()
                    .map(|&id| super::search::formula(0, id, true)),
            );
        for member in members {
            let (zone, term) = terms.member(member)?;
            zones[zone].push(term);
        }
        for (zone, wanted) in zones.iter_mut().enumerate() {
            wanted.sort_unstable();
            wanted.dedup();
            let mut found = root.zone(zone).to_vec();
            found.sort_unstable();
            found.dedup();
            if found != *wanted {
                return Err(refused("its root is not the outcome's sequent at w0"));
            }
        }
        let calculus = calculus::calculus(self.system, limits)?;
        derivation.check(&calculus, limits, stop)
    }
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
/// conditions and the sequent. A model too large to check (past
/// [`MODEL_CELLS`](crate::ordinary::MODEL_CELLS) cells or
/// `limits.memory_bytes`) is dropped and "not valid" stands without it,
/// resting on the search's argument alone, as the ordinary suite's does.
/// The search counts its work against `limits.work` and its memory
/// against `limits.memory_bytes`; past either, or when the stop fires,
/// the verdict is unknown with the reason.
///
/// # Errors
///
/// [`Refusal::Output`] where the derivation would pass
/// `limits.derivation_bytes`; [`Refusal::Memory`], [`Refusal::Work`] and
/// [`Refusal::Stopped`] where a bound or the stop refuses the check of
/// the derivation or of the model (each check has `limits.work` of its
/// own), or the stop the making of the derivation: none of these is a
/// verdict. [`Error::ReadBack`] where the checker refuses the derivation
/// and [`Error::Countermodel`] where the model check refuses the model
/// are defects of the search.
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
            let derivation = derivation(&search, root, sequent, system, limits, &mut stop)?;
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

/// The bytes of a term made for a formula of the arena or a member: its
/// entry in the terms' arena and their maps, an atom's name, and a
/// member's entry in the map of members. Measured at about 400 bytes for
/// an atom and its member together.
const TERM_BYTES: u64 = 256;

/// The members put on the sequent between two asks of the stop while a
/// derivation is made.
const POLL: u64 = 1 << 12;

/// The terms of a derivation in the making, each member's made once.
struct Terms<'a> {
    /// The derivation.
    derivation: rules::Derivation,
    /// The sequent's arena.
    sequent: &'a Sequent,
    /// The term of each formula of the arena, made in the arena's order up
    /// to the last asked for.
    formulas: Vec<TermId>,
    /// The term of each label, made on first use.
    labels: Vec<Option<TermId>>,
    /// The term of each member, made on first use.
    members: HashMap<Member, TermId>,
}

impl Terms<'_> {
    /// Returns the term of a formula of the arena, making it and every
    /// formula before it in the arena's order not made yet: every operand
    /// precedes its formula, so this needs no recursion, and the arena is
    /// made once in all.
    fn formula(&mut self, id: NodeId) -> Result<TermId, Error> {
        while self.formulas.len() <= id.index() {
            let fs = &self.formulas;
            let f = |k: NodeId| fs[k.index()];
            let terms = self.derivation.terms_mut();
            let term = match self.sequent.formulas().node(NodeId(fs.len() as u32)) {
                Node::Atom(a) => terms.atom(self.sequent.formulas().atom_name(a), "formula")?,
                Node::True => terms.apply("⊤", &[])?,
                Node::False => terms.apply("⊥", &[])?,
                Node::Not(a) => terms.apply("¬", &[f(a)])?,
                Node::Box(a) => terms.apply("□", &[f(a)])?,
                Node::Diamond(a) => terms.apply("◇", &[f(a)])?,
                Node::And(a, b) => terms.apply("∧", &[f(a), f(b)])?,
                Node::Or(a, b) => terms.apply("∨", &[f(a), f(b)])?,
                Node::Implies(a, b) => terms.apply("→", &[f(a), f(b)])?,
                Node::Iff(a, b) => terms.apply("↔", &[f(a), f(b)])?,
            };
            self.formulas.push(term);
        }
        Ok(self.formulas[id.index()])
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
    stop: &mut impl FnMut(Progress) -> bool,
) -> Result<rules::Derivation, Error> {
    let (steps, proofs) = (&search.steps, &search.proofs);
    // The size first, by a walk that adds each premise's members to its
    // conclusion's count, before anything is made: the nodes, their
    // members, and a term for each formula of the arena and each member
    // put on a sequent (at most).
    let base = (sequent.left().len() + sequent.right().len()) as u64;
    let mut estimate = (sequent.formulas().len() as u64)
        .saturating_add(base)
        .saturating_mul(TERM_BYTES);
    let mut stack = vec![(root, base)];
    while let Some((node, size)) = stack.pop() {
        estimate = estimate
            .saturating_add(NODE_BYTES)
            .saturating_add(size.saturating_mul(MEMBER_BYTES));
        let proof = &proofs[node as usize];
        let step = &steps[proof.step as usize];
        for (i, &premise) in proof.premises.iter().enumerate() {
            let added = step.premises[i].len() as u64;
            estimate = estimate.saturating_add(added.saturating_mul(TERM_BYTES));
            stack.push((premise, size.saturating_add(added)));
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
        formulas: Vec::new(),
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
    // The members put on a sequent, for the stop's asks.
    let (mut put, mut asked) = (0u64, 0u64);
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
            put = put.saturating_add(step.premises[top.next].len() as u64 + 1);
            if put - asked >= POLL {
                if stop(Progress::new(Phase::View, put - asked, put)) {
                    return Err(Error::Refused(Refusal::Stopped { phase: Phase::View }));
                }
                asked = put;
            }
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
