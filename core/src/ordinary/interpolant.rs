// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Craig interpolants of valid sequents, read off derivations of LK and LJ
//! by Maehara's method and checked.
//!
//! Split a valid sequent `Γ ⊢ Δ` into two halves, `Γ₁ ⊢ Δ₁` and
//! `Γ₂ ⊢ Δ₂`. An interpolant is a formula `I` with
//! 1. `Γ₁ ⊢ Δ₁, I` valid,
//! 2. `I, Γ₂ ⊢ Δ₂` valid, and
//! 3. every atom of `I` in both halves (`⊤` and `⊥` are no atoms).
//!
//! For `A ⊢ B`, with `A` the first half, that is Craig's `A ⊢ I` and
//! `I ⊢ B`. In intuitionistic and minimal logic the formula right of `⊢`
//! is in the second half.
//!
//! Maehara's method builds `I` inference by inference from a derivation
//! without cuts (Takeuti, *Proof Theory*, §6; Troelstra and Schwichtenberg,
//! *Basic Proof Theory*, §4.4). [`interpolant`] reads it off a derivation
//! and checks all three conditions: it decides both halves, and a half that
//! is not valid, or an atom outside the two halves, is [`Error::ClaimFails`],
//! a defect. [`interpolate`] decides the sequent through the embedding,
//! whose derivations have no cut, and reads the interpolant off its
//! derivation.
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::ordinary::interpolant::{self, Partition};
//! use deduxit::ordinary::{Logic, Options, Sequent};
//!
//! let sequent: Sequent = "a /\\ b |- b \\/ c".parse()?;
//! let options = Options::default().with_logic(Logic::Classical);
//! let found = interpolant::interpolate_default(&sequent, &options, &Partition::hypotheses(&sequent))?;
//! let interpolant = found.interpolant.expect("the sequent is valid");
//! assert_eq!(interpolant.formula().to_string(), "b");
//! # Ok::<(), deduxit::Error>(())
//! ```

use super::{
    Derivation, Formula, Formulas, Logic, Method, Node, NodeId, Options, Outcome, Rule, Sequent,
    Side, Verdict, decide_within,
};
use crate::Error;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::search;

/// The formulas of a sequent that go into the first half, by position;
/// the others go into the second.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Partition {
    /// The positions of the hypotheses in the first half.
    pub left: Vec<usize>,
    /// The positions of the formulas right of `⊢` in the first half.
    pub right: Vec<usize>,
}

impl Partition {
    /// Returns the partition of `sequent` whose first half is every
    /// hypothesis and whose second half is every formula right of `⊢`:
    /// Craig's split of `A ⊢ B`.
    pub fn hypotheses(sequent: &Sequent) -> Self {
        Self {
            left: (0..sequent.left().len()).collect(),
            right: Vec::new(),
        }
    }

    /// Returns the partition with these positions of hypotheses in the
    /// first half.
    #[must_use]
    pub fn with_left(self, left: Vec<usize>) -> Self {
        Self { left, ..self }
    }

    /// Returns the partition with these positions right of `⊢` in the
    /// first half.
    #[must_use]
    pub fn with_right(self, right: Vec<usize>) -> Self {
        Self { right, ..self }
    }

    /// Returns, for a sequent of `left` hypotheses and `right` formulas
    /// right of `⊢`, whether each is in the first half.
    ///
    /// # Errors
    ///
    /// [`Error::Partition`] for a position outside the sequent or named
    /// twice, and for a formula right of `⊢` in the first half outside
    /// classical logic.
    fn halves(&self, logic: Logic, left: usize, right: usize) -> Result<Parts, Error> {
        if logic != Logic::Classical && !self.right.is_empty() {
            return Err(Error::Partition {
                reason: format!(
                    "in {logic} logic the formula right of ⊢ belongs to the second half"
                ),
            });
        }
        let mark = |positions: &[usize], len: usize, side: &str| {
            let mut first = vec![false; len];
            for &p in positions {
                match first.get_mut(p) {
                    None => {
                        return Err(Error::Partition {
                            reason: format!("the sequent has no {side} formula {p}"),
                        });
                    }
                    Some(true) => {
                        return Err(Error::Partition {
                            reason: format!("the {side} formula {p} is named twice"),
                        });
                    }
                    Some(slot) => *slot = true,
                }
            }
            Ok(first)
        };
        Ok(Parts {
            left: mark(&self.left, left, "left")?,
            right: mark(&self.right, right, "right")?,
        })
    }
}

/// Which half each formula of a sequent is in: `true` for the first.
#[derive(Clone, Debug, Default)]
struct Parts {
    /// The hypotheses' halves.
    left: Vec<bool>,
    /// The halves of the formulas right of `⊢`.
    right: Vec<bool>,
}

impl Parts {
    /// Returns the halves of one side.
    fn side(&self, side: Side) -> &[bool] {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    /// Returns the parts with the two halves exchanged.
    fn flipped(self) -> Self {
        Self {
            left: self.left.into_iter().map(|p| !p).collect(),
            right: self.right.into_iter().map(|p| !p).collect(),
        }
    }
}

/// An interpolant and the two halves it was checked on.
///
/// # JSON
///
/// With the feature `serialize`, inside an [`Interpolation`]'s document:
/// `{"text": "b", "first": {"sequent": {…}, "outcome": {…}}, "second":
/// {…}}`, the interpolant's text and each half as a sequent (in the form
/// of a [`Sequent`]) with its outcome (in the form of an [`Outcome`]),
/// neither with a `version`. The interpolant is the last formula right of
/// `⊢` in the first half and the first hypothesis of the second.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Interpolant {
    /// The first half with the interpolant added right of `⊢`, last:
    /// `Γ₁ ⊢ Δ₁, I`.
    pub first: Sequent,
    /// The interpolant added as the first hypothesis of the second half:
    /// `I, Γ₂ ⊢ Δ₂`.
    pub second: Sequent,
    /// The first half decided in the logic.
    pub first_outcome: Outcome,
    /// The second half decided in the logic.
    pub second_outcome: Outcome,
    /// The interpolant, a formula of both halves' arena.
    id: NodeId,
}

impl Interpolant {
    /// Returns the interpolant, as a value that prints it.
    pub fn formula(&self) -> Formula<'_> {
        self.first.formulas().formula(self.id)
    }

    /// Returns the interpolant's id in the arena of both halves.
    pub const fn id(&self) -> NodeId {
        self.id
    }

    /// Returns whether both halves were decided valid: false where a
    /// search stopped before it decided one of them.
    pub fn is_checked(&self) -> bool {
        matches!(self.first_outcome.verdict, Verdict::Valid(_))
            && matches!(self.second_outcome.verdict, Verdict::Valid(_))
    }
}

/// A sequent decided, and the interpolant of a partition of it where it
/// is valid: what [`interpolate`] returns.
///
/// # JSON
///
/// With the feature `serialize` an interpolation is the document
/// `{"version": 16, "deduxit": "0.6.0", "logic": "classical",
/// "partition": {"left": [0], "right": []}, "outcome": {…},
/// "interpolant": {…}}` ([`wire`](crate::wire)), written only: the
/// outcome of the sequent (in the form of an [`Outcome`], without a
/// `version`), and the [`Interpolant`] where the sequent is valid.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Interpolation {
    /// The partition.
    pub partition: Partition,
    /// The sequent decided, with its derivation where it is valid.
    pub outcome: Outcome,
    /// The interpolant, where the sequent is valid.
    pub interpolant: Option<Interpolant>,
}

/// Decides `sequent` as `options` say and reads the interpolant of
/// `partition` off a derivation without cuts where it is valid
/// ([`interpolant`]). Where the method's derivation has a cut (LJT, the SAT
/// solver) or there is none (a clausal proof), and `options` name no
/// method, the sequent is decided again through the embedding, whose
/// derivations have none. A sequent that is not valid has no
/// interpolant, and neither has one the searches do not decide or whose
/// derivation the embedding does not find within the limits.
///
/// # Errors
///
/// [`Error::Partition`] for a partition that is not one of the sequent;
/// [`decide_within`]'s errors; for a method named whose derivation has a
/// cut, [`Error::NotCutFree`], and for one that gives no derivation,
/// [`Error::NoDerivation`]; [`interpolant`]'s errors; [`Error::ReadBack`]
/// where the embedding finds not valid what the method found valid, a
/// defect.
pub fn interpolate(
    sequent: &Sequent,
    options: &Options,
    partition: &Partition,
    search: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Interpolation, Error> {
    let logic = options.logic;
    partition.halves(logic, sequent.left().len(), sequent.right().len())?;
    let decided = decide_within(sequent, options, search, limits, &mut stop)?;
    let cut_free = |outcome: &Outcome| match &outcome.verdict {
        Verdict::Valid(evidence) => match evidence.derivation() {
            Ok(d) if !d.inferences().iter().any(|i| i.rule == Rule::Cut) => Ok(Some(d.clone())),
            Ok(_) => Err(Error::NotCutFree),
            Err(error) => Err(error),
        },
        Verdict::NotValid | Verdict::Unknown(_) => Ok(None),
    };
    let (outcome, derivation) = match cut_free(&decided) {
        Ok(derivation) => (decided, derivation),
        Err(error) if options.method.is_some() => return Err(error),
        Err(_) => {
            let embedding = options.with_method(Some(Method::Embedding));
            let embedded = decide_within(sequent, &embedding, search, limits, &mut stop)?;
            match embedded.verdict {
                Verdict::Valid(_) => {
                    let derivation = cut_free(&embedded)?;
                    (embedded, derivation)
                }
                Verdict::NotValid => {
                    return Err(Error::ReadBack {
                        calculus: logic.calculus(),
                        reason: "the embedding finds the sequent not valid, which the method found valid"
                            .to_owned(),
                    });
                }
                // The method's verdict stands without an interpolant.
                Verdict::Unknown(_) => (decided, None),
            }
        }
    };
    // The derivation concludes the sequent, or in intuitionistic and
    // minimal logic `Γ ⊢ ⊥` for `Γ ⊢`: its hypotheses are the sequent's, in
    // their order, and its formula right of `⊢` is in the second half.
    let interpolant = match derivation {
        Some(derivation) => Some(interpolant(
            &derivation,
            partition,
            search,
            limits,
            &mut stop,
        )?),
        None => None,
    };
    Ok(Interpolation {
        partition: partition.clone(),
        outcome,
        interpolant,
    })
}

/// Interpolates as [`interpolate`] does within the default [`Limits`] and
/// with the linear search's default options.
///
/// # Errors
///
/// As [`interpolate`].
pub fn interpolate_default(
    sequent: &Sequent,
    options: &Options,
    partition: &Partition,
) -> Result<Interpolation, Error> {
    interpolate(
        sequent,
        options,
        partition,
        &search::Options::default(),
        &Limits::default(),
        |_| false,
    )
}

/// Returns the interpolant of `partition`, positions of the sequent
/// `derivation` concludes, read off the derivation by Maehara's method and
/// checked: both halves decided in the derivation's logic by its default
/// method under `search`, `limits` and `stop`, and its atoms compared.
/// The derivation is checked first, and the stop asked once per inference.
///
/// A half the search does not decide leaves the interpolant unchecked
/// ([`Interpolant::is_checked`]).
///
/// # Errors
///
/// - [`Error::NotCutFree`] for a derivation with a cut;
/// - [`Error::Partition`] for a partition that is not one of its sequent;
/// - [`Derivation::check`]'s errors for one that breaks a rule;
/// - [`Refusal::Occurrences`] for halves that unfold past
///   `limits.occurrences`, [`Refusal::Stopped`] when `stop` fired;
/// - [`Error::ClaimFails`] for a half decided not valid or an atom of the
///   interpolant outside a half, a defect.
pub fn interpolant(
    derivation: &Derivation,
    partition: &Partition,
    search: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Interpolant, Error> {
    let logic = derivation.logic();
    if derivation.inferences().iter().any(|i| i.rule == Rule::Cut) {
        return Err(Error::NotCutFree);
    }
    let (left, right) = derivation.sequent();
    let root_parts = partition.halves(logic, left.len(), right.len())?;
    derivation.check(limits, &mut stop)?;
    let mut out = derivation.formulas().clone();
    let id = maehara(derivation, root_parts.clone(), &mut out, &mut stop)?;
    checked(
        logic,
        (left, right),
        &root_parts,
        (out, id),
        search,
        limits,
        stop,
    )
}

/// Returns the interpolant `id` of `out` for the halves `parts` of the
/// sequent `left ⊢ right` of `logic`, once it passes the three checks:
/// its atoms in both halves, and both halves decided, neither not valid.
///
/// # Errors
///
/// [`Error::ClaimFails`] for a check it fails; [`Refusal::Occurrences`]
/// for halves past `limits.occurrences`; [`decide_within`]'s errors.
fn checked(
    logic: Logic,
    (left, right): (&[NodeId], &[NodeId]),
    root_parts: &Parts,
    (out, id): (Formulas, NodeId),
    search: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Interpolant, Error> {
    let (mut first_left, mut first_right, mut second_left, mut second_right) =
        (Vec::new(), Vec::new(), vec![id], Vec::new());
    for (side, ids) in [(Side::Left, left), (Side::Right, right)] {
        for (&f, &first) in ids.iter().zip(root_parts.side(side)) {
            match (side, first) {
                (Side::Left, true) => first_left.push(f),
                (Side::Left, false) => second_left.push(f),
                (Side::Right, true) => first_right.push(f),
                (Side::Right, false) => second_right.push(f),
            }
        }
    }
    first_right.push(id);
    let claim = || "Maehara's interpolant".to_owned();
    // The interpolant's atoms must be atoms of both halves.
    let atoms = |roots: &[NodeId]| atoms_of(&out, roots);
    let first_atoms = atoms(&[&first_left[..], &first_right[..first_right.len() - 1]].concat());
    let second_atoms = atoms(&[&second_left[1..], &second_right[..]].concat());
    if let Some(a) = atoms(&[id])
        .iter()
        .enumerate()
        .find(|&(a, &used)| used && !(first_atoms[a] && second_atoms[a]))
        .map(|(a, _)| a)
    {
        return Err(Error::ClaimFails {
            claim: claim(),
            reason: format!("its atom {} is not in both halves", out.atom_name(a as u32)),
        });
    }
    super::bound(&out, &first_left, &first_right, limits)?;
    super::bound(&out, &second_left, &second_right, limits)?;
    let first = Sequent::new(out.clone(), first_left, first_right)?;
    let second = Sequent::new(out, second_left, second_right)?;
    let options = Options::default().with_logic(logic);
    let first_outcome = decide_within(&first, &options, search, limits, &mut stop)?;
    let second_outcome = decide_within(&second, &options, search, limits, &mut stop)?;
    for (half, outcome, sequent) in [
        ("first", &first_outcome, &first),
        ("second", &second_outcome, &second),
    ] {
        if matches!(outcome.verdict, Verdict::NotValid) {
            return Err(Error::ClaimFails {
                claim: claim(),
                reason: format!("the {half} half {sequent} is not valid in {logic} logic"),
            });
        }
    }
    Ok(Interpolant {
        first,
        second,
        first_outcome,
        second_outcome,
        id,
    })
}

/// Returns, by atom index, whether an atom occurs in the formulas at
/// `roots`; every operand precedes its formula, so one pass down the arena
/// marks what the roots reach.
pub(crate) fn atoms_of(formulas: &Formulas, roots: &[NodeId]) -> Vec<bool> {
    let mut reached = vec![false; formulas.len()];
    for root in roots {
        reached[root.index()] = true;
    }
    let mut atoms = vec![false; formulas.atom_names().len()];
    for k in (0..formulas.len()).rev() {
        if !reached[k] {
            continue;
        }
        let node = formulas.nodes[k];
        if let Node::Atom(a) = node {
            atoms[a as usize] = true;
        }
        let (a, b) = node.operands();
        for id in [a, b].into_iter().flatten() {
            reached[id.index()] = true;
        }
    }
    atoms
}

/// Returns Maehara's interpolant of a checked derivation without cuts for
/// the halves `root` of its sequent, built in `out`.
///
/// Going up, each premise takes the halves of the conclusion: a formula
/// of the context keeps its half, and the formulas the rule makes from the
/// principal formula take the principal formula's. In LJ a premise whose
/// formula right of `⊢` would be in the first half (the first premise of
/// `→L` or `¬L` on a hypothesis of the first half) takes its halves
/// exchanged. Going down, each inference's interpolant is made from its
/// premises':
/// - an axiom `A ⊢ A`: `⊥`, `⊤`, `A` or `¬A` as its two sides are in the
///   first, the second, the first and second, or the second and first half;
/// - another rule without premises: `⊥` for a principal formula in the
///   first half, `⊤` for one in the second;
/// - one premise: its interpolant, negated where its halves are exchanged;
/// - two premises: their disjunction for a principal formula in the first
///   half, their conjunction for one in the second, and `I₁ → I₂` where the
///   first premise's halves are exchanged.
fn maehara(
    derivation: &Derivation,
    root: Parts,
    out: &mut Formulas,
    stop: &mut impl FnMut(Progress) -> bool,
) -> Result<NodeId, Error> {
    let logic = derivation.logic();
    let inferences = derivation.inferences();
    let n = inferences.len();
    let defect = |why: &str| Error::ReadBack {
        calculus: logic.calculus(),
        reason: why.to_owned(),
    };
    let mut parts: Vec<Option<Parts>> = vec![None; n];
    let mut exchanged = vec![false; n];
    parts[n - 1] = Some(root);
    // Down the indices: every inference after its premises, so a
    // conclusion's halves are known before its premises'.
    for i in (0..n).rev() {
        if stop(Progress::new(Phase::ReadBack, 1, (n - i) as u64)) {
            return Err(Refusal::Stopped {
                phase: Phase::ReadBack,
            }
            .into());
        }
        let inference = &inferences[i];
        let conclusion = parts[i]
            .clone()
            .ok_or_else(|| defect("an inference below no other"))?;
        let premises: Vec<&super::Inference> = inference
            .premises
            .iter()
            .map(|&p| &inferences[p.index()])
            .collect();
        let added =
            super::derivation::check_inference(derivation.formulas(), logic, inference, &premises)
                .map_err(|why| defect(&why))?;
        let principal = inference
            .principal
            .map(|(side, at)| (side, at, conclusion.side(side)[at]));
        // The conclusion's context, each formula with its half.
        let mut context = Vec::new();
        for (side, ids) in [
            (Side::Left, &inference.left),
            (Side::Right, &inference.right),
        ] {
            for (at, (&f, &first)) in ids.iter().zip(conclusion.side(side)).enumerate() {
                if principal.is_none_or(|(s, a, _)| (s, a) != (side, at)) {
                    context.push((side, f, first));
                }
            }
        }
        let half = principal.map(|(.., first)| first);
        // A split gives each formula of the context to one premise; a
        // shared context goes whole into each.
        let added: Vec<Vec<(Side, NodeId)>> =
            (0..premises.len()).map(|k| added.premise(k)).collect();
        let split = premise_parts(&premises, &added, &context, half, true);
        let assigned = match split {
            Some(assigned) => assigned,
            None => premise_parts(&premises, &added, &context, half, false)
                .ok_or_else(|| defect("the premises do not share or split the context"))?,
        };
        for (&p, mut premise) in inference.premises.iter().zip(assigned) {
            if logic != Logic::Classical && premise.right.iter().any(|&first| first) {
                premise = premise.flipped();
                exchanged[p.index()] = true;
            }
            if parts[p.index()].replace(premise).is_some() {
                return Err(defect("an inference is the premise of two"));
            }
        }
    }
    let mut build = Build {
        out,
        minimal: logic == Logic::Minimal,
    };
    let mut made: Vec<NodeId> = Vec::with_capacity(n);
    for (i, inference) in inferences.iter().enumerate() {
        let halves = parts[i]
            .as_ref()
            .ok_or_else(|| defect("an inference below no other"))?;
        let first = |side: Side, at: usize| halves.side(side)[at];
        let premise = |k: usize| {
            let p = inference.premises[k].index();
            (made[p], exchanged[p])
        };
        let id = match (inference.rule, inference.premises.len()) {
            (Rule::Axiom, _) => match (first(Side::Left, 0), first(Side::Right, 0)) {
                (true, true) => build.falsity()?,
                (false, false) => build.truth()?,
                (true, false) => inference.left[0],
                (false, true) => build.not(inference.left[0])?,
            },
            (_, 0) => {
                let (side, at) = inference
                    .principal
                    .ok_or_else(|| defect("a leaf without principal formula"))?;
                if first(side, at) {
                    build.falsity()?
                } else {
                    build.truth()?
                }
            }
            (_, 1) => match premise(0) {
                (i0, false) => i0,
                (i0, true) => build.not(i0)?,
            },
            (_, 2) => {
                let (side, at) = inference.principal.ok_or_else(|| defect("a cut"))?;
                match (premise(0), premise(1)) {
                    ((i0, false), (i1, false)) if first(side, at) => build.or(i0, i1)?,
                    ((i0, false), (i1, false)) => build.and(i0, i1)?,
                    ((i0, true), (i1, false)) => build.implies(i0, i1)?,
                    ((i0, false), (i1, true)) => build.implies(i1, i0)?,
                    ((_, true), (_, true)) => {
                        return Err(defect("both premises exchange their halves"));
                    }
                }
            }
            _ => return Err(defect("more than two premises")),
        };
        made.push(id);
    }
    Ok(made[n - 1])
}

/// Returns each premise's halves: the formulas the rule adds (`added`)
/// take the principal formula's half `half`, and every other formula one
/// of the context's, of the same side and formula. With `split` the
/// premises draw on one context, which each formula of it goes to once;
/// without, each draws on the whole context. `None` where a premise finds
/// no formula of the context left for one of its own.
fn premise_parts(
    premises: &[&super::Inference],
    added: &[Vec<(Side, NodeId)>],
    context: &[(Side, NodeId, bool)],
    half: Option<bool>,
    split: bool,
) -> Option<Vec<Parts>> {
    let mut pool: Vec<Option<(Side, NodeId, bool)>> = context.iter().copied().map(Some).collect();
    let mut all = Vec::with_capacity(premises.len());
    for (premise, added) in premises.iter().zip(added) {
        if !split {
            pool = context.iter().copied().map(Some).collect();
        }
        let mut fresh = added.clone();
        let mut halves = Parts::default();
        for (side, ids) in [(Side::Left, &premise.left), (Side::Right, &premise.right)] {
            for &f in ids {
                let first = if let Some(k) = fresh.iter().position(|&a| a == (side, f)) {
                    fresh.swap_remove(k);
                    half?
                } else {
                    let slot = pool
                        .iter_mut()
                        .find(|c| matches!(c, Some((s, g, _)) if *s == side && *g == f))?;
                    slot.take()?.2
                };
                match side {
                    Side::Left => halves.left.push(first),
                    Side::Right => halves.right.push(first),
                }
            }
        }
        all.push(halves);
    }
    Some(all)
}

/// Builds the interpolant's formulas, dropping `⊤` and `⊥` where the
/// logic proves the result equivalent: in minimal logic `⊥` is a formula
/// like any other, so only `⊤` is dropped there.
struct Build<'a> {
    /// The arena the formulas go into.
    out: &'a mut Formulas,
    /// Whether the logic is minimal logic.
    minimal: bool,
}

impl Build<'_> {
    /// Returns whether `id` is `⊤`.
    fn is_true(&self, id: NodeId) -> bool {
        self.out.node(id) == Node::True
    }

    /// Returns whether `id` is `⊥` where `⊥` proves everything.
    fn is_false(&self, id: NodeId) -> bool {
        !self.minimal && self.out.node(id) == Node::False
    }

    /// Returns `⊤`.
    fn truth(&mut self) -> Result<NodeId, Error> {
        self.out.add(Node::True)
    }

    /// Returns `⊥`.
    fn falsity(&mut self) -> Result<NodeId, Error> {
        self.out.add(Node::False)
    }

    /// Returns `¬a`.
    fn not(&mut self, a: NodeId) -> Result<NodeId, Error> {
        match self.out.node(a) {
            Node::True => self.falsity(),
            Node::False => self.truth(),
            Node::Atom(_)
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => self.out.add(Node::Not(a)),
        }
    }

    /// Returns `a ∧ b`.
    fn and(&mut self, a: NodeId, b: NodeId) -> Result<NodeId, Error> {
        if self.is_true(a) || a == b {
            Ok(b)
        } else if self.is_true(b) || self.is_false(a) {
            Ok(a)
        } else if self.is_false(b) {
            Ok(b)
        } else {
            self.out.add(Node::And(a, b))
        }
    }

    /// Returns `a ∨ b`.
    fn or(&mut self, a: NodeId, b: NodeId) -> Result<NodeId, Error> {
        if self.is_false(a) || a == b || self.is_true(b) {
            Ok(b)
        } else if self.is_false(b) || self.is_true(a) {
            Ok(a)
        } else {
            self.out.add(Node::Or(a, b))
        }
    }

    /// Returns `a → b`.
    fn implies(&mut self, a: NodeId, b: NodeId) -> Result<NodeId, Error> {
        if self.is_true(a) {
            Ok(b)
        } else if self.is_true(b) || self.is_false(a) || a == b {
            self.truth()
        } else if self.out.node(b) == Node::False {
            self.not(a)
        } else {
            self.out.add(Node::Implies(a, b))
        }
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;

    /// Returns the interpolant of `text` in `logic` with the hypotheses
    /// at `left` and the formulas right of `⊢` at `right` first, checked.
    fn interpolant_of(text: &str, logic: Logic, left: &[usize], right: &[usize]) -> String {
        let sequent: Sequent = text.parse().unwrap();
        let partition = Partition::default()
            .with_left(left.to_vec())
            .with_right(right.to_vec());
        let options = Options::default().with_logic(logic);
        let found = interpolate_default(&sequent, &options, &partition)
            .unwrap_or_else(|e| panic!("{text} in {logic}: {e}"));
        let interpolant = found
            .interpolant
            .unwrap_or_else(|| panic!("{text} in {logic}: no interpolant"));
        assert!(interpolant.is_checked(), "{text} in {logic}");
        interpolant.formula().to_string()
    }

    /// Interpolants of LK and LJ, every rule's case among them, pass all
    /// three checks, and the atoms they keep are the shared ones.
    #[test]
    fn interpolants_pass_their_checks() {
        use Logic::{Classical, Intuitionistic, Minimal};
        assert_eq!(
            interpolant_of("a /\\ b |- b \\/ c", Classical, &[0], &[]),
            "b"
        );
        assert_eq!(
            interpolant_of("a, a -> b |- b \\/ c", Classical, &[0, 1], &[]),
            "b"
        );
        assert_eq!(
            interpolant_of("p /\\ q |- q /\\ (r -> r)", Intuitionistic, &[0], &[]),
            "q"
        );
        assert_eq!(interpolant_of("a |- a", Classical, &[], &[0]), "¬a");
        assert_eq!(interpolant_of("a |- b -> b", Classical, &[0], &[]), "⊤");
        assert_eq!(
            interpolant_of("a /\\ ~a |- b", Intuitionistic, &[0], &[]),
            "⊥"
        );
        for logic in [Classical, Intuitionistic, Minimal] {
            for (text, left) in [
                ("a -> b, b -> c |- a -> c", &[0][..]),
                ("a -> b, b -> c |- a -> c", &[1][..]),
                ("a \\/ b, a -> c, b -> c |- c", &[0, 1][..]),
                ("~a, a \\/ b |- ~~b", &[0][..]),
                ("~a, a \\/ b |- ~~b", &[1][..]),
                ("a <-> b, b |- a", &[0][..]),
                ("(a -> b) -> c, b |- c", &[0][..]),
                ("(a -> b) -> c, b |- c", &[1][..]),
            ] {
                interpolant_of(text, logic, left, &[]);
            }
        }
    }

    /// Generated sequents, split at random, give interpolants that pass
    /// their checks in every logic; a search stopped by its bound of work
    /// gives none and is no failure.
    #[test]
    fn generated_sequents_interpolate() {
        use crate::linear::search::generate::Rng;
        let mut rng = Rng::new(63);
        let limits = Limits::default().with_work(Some(200_000));
        let mut found = 0;
        for _ in 0..200 {
            let sequent = super::super::tests::generated_sequent(&mut rng);
            for &logic in Logic::ALL {
                let pick = |n: usize, rng: &mut Rng| (0..n).filter(|_| rng.one_in(2)).collect();
                let left = pick(sequent.left().len(), &mut rng);
                let right = if logic == Logic::Classical {
                    pick(sequent.right().len(), &mut rng)
                } else {
                    Vec::new()
                };
                let partition = Partition::default().with_left(left).with_right(right);
                let options = Options::default().with_logic(logic);
                match interpolate(
                    &sequent,
                    &options,
                    &partition,
                    &search::Options::default(),
                    &limits,
                    |_| false,
                ) {
                    Ok(interpolation) => found += usize::from(interpolation.interpolant.is_some()),
                    Err(error) if error.is_refusal() => {}
                    Err(error) => panic!("{sequent} in {logic} split {partition:?}: {error}"),
                }
            }
        }
        assert!(found > 100, "{found} interpolants");
    }

    /// The check refuses a formula that is no interpolant: one with an
    /// atom outside a half, and one that makes a half not valid.
    #[test]
    fn the_check_refuses_what_is_no_interpolant() {
        let sequent: Sequent = "a |- a \\/ c".parse().unwrap();
        let parts = Partition::hypotheses(&sequent)
            .halves(Logic::Classical, 1, 1)
            .unwrap();
        let mut out = sequent.formulas().clone();
        let c = out.atom("c").unwrap();
        let truth = out.add(Node::True).unwrap();
        let a = out.atom("a").unwrap();
        for (wrong, why) in [(c, "atom c"), (truth, "second half")] {
            let result = checked(
                Logic::Classical,
                (sequent.left(), sequent.right()),
                &parts,
                (out.clone(), wrong),
                &search::Options::default(),
                &Limits::default(),
                |_| false,
            );
            match result {
                Err(Error::ClaimFails { reason, .. }) => assert!(reason.contains(why), "{reason}"),
                other => panic!("{other:?}"),
            }
        }
        let right = checked(
            Logic::Classical,
            (sequent.left(), sequent.right()),
            &parts,
            (out, a),
            &search::Options::default(),
            &Limits::default(),
            |_| false,
        );
        assert!(right.unwrap().is_checked());
    }

    /// A partition that is not one, a sequent that is not valid and a
    /// derivation with a cut give no interpolant.
    #[test]
    fn what_has_no_interpolant_is_refused() {
        let sequent: Sequent = "a |- a".parse().unwrap();
        let classical = Options::default().with_logic(Logic::Classical);
        let intuitionistic = Options::default();
        for (options, partition) in [
            (classical, Partition::default().with_left(vec![1])),
            (classical, Partition::default().with_left(vec![0, 0])),
            (intuitionistic, Partition::default().with_right(vec![0])),
        ] {
            assert!(matches!(
                interpolate_default(&sequent, &options, &partition),
                Err(Error::Partition { .. })
            ));
        }
        let not_valid: Sequent = "a |- b".parse().unwrap();
        let found = interpolate_default(&not_valid, &classical, &Partition::hypotheses(&not_valid))
            .unwrap();
        assert!(found.interpolant.is_none());
        let ljt = intuitionistic.with_method(Some(Method::Ljt));
        let lemma: Sequent = "(a -> b) -> c, b |- c".parse().unwrap();
        let derivation = super::super::decide(&lemma, &ljt).unwrap();
        let Verdict::Valid(evidence) = derivation.verdict else {
            panic!("valid")
        };
        let derivation = evidence.derivation().unwrap();
        assert!(derivation.inferences().iter().any(|i| i.rule == Rule::Cut));
        {
            assert!(matches!(
                interpolant(
                    derivation,
                    &Partition::default().with_left(vec![0]),
                    &search::Options::default(),
                    &Limits::default(),
                    |_| false
                ),
                Err(Error::NotCutFree)
            ));
        }
    }
}
