// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use super::translate::{Core, Image, pattern};
use super::{Formulas, Logic, Node, NodeId, Translation};
use crate::first_order::FoTermId;
use crate::limits::{Limits, Phase, Progress, Refusal, Space};
use crate::linear::occurrences::OccId;
use crate::linear::proofs::style::Drawn;
use crate::linear::proofs::{
    Compact, InfId, Labels, Rule as Linear, Sides, Size, Structural, TextOptions, ViewOptions,
};
#[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
use crate::linear::sequents::notation::Notation;
use crate::linear::sequents::notation::TEXT;
use crate::{Error, linear::Proof};
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use std::sync::atomic::{AtomicBool, Ordering};

/// A side of the turnstile.
///
/// # JSON
///
/// With the feature `serialize`, `"left"` or `"right"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Side {
    /// Left of `⊢`: a hypothesis.
    Left,
    /// Right of `⊢`.
    Right,
}

impl Side {
    /// Returns the other side.
    pub(crate) const fn flip(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

/// A rule of LK or LJ, the sequent calculi of classical and of
/// intuitionistic logic, as a [`Derivation`] names it.
///
/// The rules are Gentzen's, with explicit weakening and contraction. A
/// rule with two premises takes its context either whole into both (as
/// `∧R` of LJ does) or split between them (as `→L` of LJ does). An axiom
/// `A ⊢ A` has no context, while `⊤R` and `⊥L` take any. `¬A` is a
/// connective of its own: in LJ its left rule has the form of `→L` for
/// `A → ⊥`, and in LK the one premise `Γ ⊢ A, Δ`.
///
/// Every LJ sequent has at most one formula right of `⊢`. Minimal logic
/// has no `⊥L` and exactly one formula right of `⊢` in every sequent
/// (G1m): one rule would read an empty right side as `⊥` and another as
/// any formula, which together give ex falso. LJ also has Gentzen's cut;
/// LK's derivations have none.
///
/// The quantifier rules are G3's (Troelstra and Schwichtenberg, *Basic
/// Proof Theory*, §3.1): `∀L` and `∃R` take any closed term as their
/// witness, and `∀R` and `∃L` an eigenvariable that occurs in no formula
/// of their conclusion. Each records its term
/// ([`Inference::term`]), so the checker builds the instance and compares.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rule {
    /// `ax`: `A ⊢ A`.
    Axiom,
    /// `WL`, weakening of a hypothesis.
    WeakenLeft,
    /// `WR`, weakening right of `⊢`.
    WeakenRight,
    /// `CL`, contraction of a hypothesis.
    ContractLeft,
    /// `CR`, contraction right of `⊢`.
    ContractRight,
    /// `∧L`, both conjuncts.
    AndLeft,
    /// `∧L₁`, the first conjunct.
    AndLeft1,
    /// `∧L₂`, the second conjunct.
    AndLeft2,
    /// `∧R`
    AndRight,
    /// `∨L`
    OrLeft,
    /// `∨R`, both disjuncts (LK).
    OrRight,
    /// `∨R₁`
    OrRight1,
    /// `∨R₂`
    OrRight2,
    /// `→L`
    ImpliesLeft,
    /// `→R`
    ImpliesRight,
    /// `¬L`
    NotLeft,
    /// `¬R`
    NotRight,
    /// `↔L`, both implications.
    IffLeft,
    /// `↔L₁`, the implication from left to right.
    IffLeft1,
    /// `↔L₂`, the implication from right to left.
    IffLeft2,
    /// `↔R`
    IffRight,
    /// `⊥L`, ex falso.
    FalseLeft,
    /// `⊥R`, false dropped from the right.
    FalseRight,
    /// `⊤L`, true dropped from the hypotheses.
    TrueLeft,
    /// `⊤R`
    TrueRight,
    /// `cut`, in LJ only: `Γ ⊢ C` and `Δ, C ⊢ E` conclude `Γ, Δ ⊢ E`
    /// (Gentzen's cut, the context split between the premises). The cut
    /// formula `C` is the one right of `⊢` in the first premise; it is no
    /// formula of the conclusion, so the inference has no principal
    /// formula. Where the conclusion has nothing right of `⊢`, both
    /// premises may take its whole context instead.
    Cut,
    /// An open goal of a proof in progress. No derivation the checker
    /// accepts has one.
    Open,
    /// `∀L`: `A[t/x], Γ ⊢ Δ` concludes `∀x. A, Γ ⊢ Δ` for any closed term
    /// `t`, the inference's [`term`](Inference::term); the premise may
    /// also keep `∀x. A`.
    ForallLeft,
    /// `∀R`: `Γ ⊢ Δ, A[y/x]` concludes `Γ ⊢ Δ, ∀x. A`, where the
    /// eigenvariable `y`, the inference's [`term`](Inference::term),
    /// occurs in no formula of the conclusion.
    ForallRight,
    /// `∃L`: `A[y/x], Γ ⊢ Δ` concludes `∃x. A, Γ ⊢ Δ`, where the
    /// eigenvariable `y`, the inference's [`term`](Inference::term),
    /// occurs in no formula of the conclusion.
    ExistsLeft,
    /// `∃R`: `Γ ⊢ Δ, A[t/x]` concludes `Γ ⊢ Δ, ∃x. A` for any closed term
    /// `t`, the inference's [`term`](Inference::term); in LK the premise
    /// may also keep `∃x. A`.
    ExistsRight,
}

/// Each rule's two labels in markup, in the order of [`Rule::ALL`]:
/// upright, and with the side and its number lowered as a subscript
/// (`∧_{L1}`). The upright label is the rule's name in markup, which a
/// test compares.
const LABELS: [(&str, &str); Rule::ALL.len()] = [
    ("ax", "ax"),
    ("WL", "W_L"),
    ("WR", "W_R"),
    ("CL", "C_L"),
    ("CR", "C_R"),
    ("∧L", "∧_L"),
    ("∧L_1", "∧_{L1}"),
    ("∧L_2", "∧_{L2}"),
    ("∧R", "∧_R"),
    ("∨L", "∨_L"),
    ("∨R", "∨_R"),
    ("∨R_1", "∨_{R1}"),
    ("∨R_2", "∨_{R2}"),
    ("→L", "→_L"),
    ("→R", "→_R"),
    ("¬L", "¬_L"),
    ("¬R", "¬_R"),
    ("↔L", "↔_L"),
    ("↔L_1", "↔_{L1}"),
    ("↔L_2", "↔_{L2}"),
    ("↔R", "↔_R"),
    ("⊥L", "⊥_L"),
    ("⊥R", "⊥_R"),
    ("⊤L", "⊤_L"),
    ("⊤R", "⊤_R"),
    ("cut", "cut"),
    ("open", "open"),
    ("∀L", "∀_L"),
    ("∀R", "∀_R"),
    ("∃L", "∃_L"),
    ("∃R", "∃_R"),
];

impl Rule {
    /// Every rule, in the order of declaration, which is the order of
    /// `rule as usize`.
    pub(crate) const ALL: [Self; 31] = {
        use Rule::*;
        [
            Axiom,
            WeakenLeft,
            WeakenRight,
            ContractLeft,
            ContractRight,
            AndLeft,
            AndLeft1,
            AndLeft2,
            AndRight,
            OrLeft,
            OrRight,
            OrRight1,
            OrRight2,
            ImpliesLeft,
            ImpliesRight,
            NotLeft,
            NotRight,
            IffLeft,
            IffLeft1,
            IffLeft2,
            IffRight,
            FalseLeft,
            FalseRight,
            TrueLeft,
            TrueRight,
            Cut,
            Open,
            ForallLeft,
            ForallRight,
            ExistsLeft,
            ExistsRight,
        ]
    };

    /// Returns the rule's usual spelling.
    pub const fn name(self) -> &'static str {
        use Rule::*;
        match self {
            Axiom => "ax",
            WeakenLeft => "WL",
            WeakenRight => "WR",
            ContractLeft => "CL",
            ContractRight => "CR",
            AndLeft => "∧L",
            AndLeft1 => "∧L₁",
            AndLeft2 => "∧L₂",
            AndRight => "∧R",
            OrLeft => "∨L",
            OrRight => "∨R",
            OrRight1 => "∨R₁",
            OrRight2 => "∨R₂",
            ImpliesLeft => "→L",
            ImpliesRight => "→R",
            NotLeft => "¬L",
            NotRight => "¬R",
            IffLeft => "↔L",
            IffLeft1 => "↔L₁",
            IffLeft2 => "↔L₂",
            IffRight => "↔R",
            FalseLeft => "⊥L",
            FalseRight => "⊥R",
            TrueLeft => "⊤L",
            TrueRight => "⊤R",
            Cut => "cut",
            Open => "open",
            ForallLeft => "∀L",
            ForallRight => "∀R",
            ExistsLeft => "∃L",
            ExistsRight => "∃R",
        }
    }

    /// Returns whether the rule is structural: a weakening or a
    /// contraction.
    pub const fn is_structural(self) -> bool {
        matches!(
            self,
            Self::WeakenLeft | Self::WeakenRight | Self::ContractLeft | Self::ContractRight
        )
    }

    /// Returns the label in the markup the exports set (a subscript as
    /// `_1`), upright, or with the side and its number lowered as a
    /// subscript where `subscript` is set.
    pub(crate) const fn markup(self, subscript: bool) -> &'static str {
        let (upright, lowered) = LABELS[self as usize];
        if subscript { lowered } else { upright }
    }
}

impl Display for Rule {
    /// Writes the rule's [`name`](Self::name).
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

/// The ASCII spellings of the rules, each beside its rule: the
/// connectives as the parser of ordinary formulas reads them (`/\`, `\/`,
/// `->`, `~`, `<->`) or as words (`and`, `or`, `imp`, `not`, `iff`,
/// `false`, `true`), a subscript as a digit.
const ASCII: [(&str, Rule); 40] = [
    ("/\\L", Rule::AndLeft),
    ("/\\L1", Rule::AndLeft1),
    ("/\\L2", Rule::AndLeft2),
    ("/\\R", Rule::AndRight),
    ("\\/L", Rule::OrLeft),
    ("\\/R", Rule::OrRight),
    ("\\/R1", Rule::OrRight1),
    ("\\/R2", Rule::OrRight2),
    ("->L", Rule::ImpliesLeft),
    ("->R", Rule::ImpliesRight),
    ("~L", Rule::NotLeft),
    ("~R", Rule::NotRight),
    ("<->L", Rule::IffLeft),
    ("<->L1", Rule::IffLeft1),
    ("<->L2", Rule::IffLeft2),
    ("<->R", Rule::IffRight),
    ("andL", Rule::AndLeft),
    ("andL1", Rule::AndLeft1),
    ("andL2", Rule::AndLeft2),
    ("andR", Rule::AndRight),
    ("orL", Rule::OrLeft),
    ("orR", Rule::OrRight),
    ("orR1", Rule::OrRight1),
    ("orR2", Rule::OrRight2),
    ("impL", Rule::ImpliesLeft),
    ("impR", Rule::ImpliesRight),
    ("notL", Rule::NotLeft),
    ("notR", Rule::NotRight),
    ("iffL", Rule::IffLeft),
    ("iffL1", Rule::IffLeft1),
    ("iffL2", Rule::IffLeft2),
    ("iffR", Rule::IffRight),
    ("falseL", Rule::FalseLeft),
    ("falseR", Rule::FalseRight),
    ("trueL", Rule::TrueLeft),
    ("trueR", Rule::TrueRight),
    ("forallL", Rule::ForallLeft),
    ("forallR", Rule::ForallRight),
    ("existsL", Rule::ExistsLeft),
    ("existsR", Rule::ExistsRight),
];

/// The names of every rule a step takes, all but [`Rule::Cut`] and
/// [`Rule::Open`], as an unknown name's error lists them.
const NAMES: [&str; Rule::ALL.len() - 2] = {
    let mut names = [""; Rule::ALL.len() - 2];
    let mut i = 0;
    let mut n = 0;
    while i < Rule::ALL.len() {
        if !matches!(Rule::ALL[i], Rule::Cut | Rule::Open) {
            names[n] = Rule::ALL[i].name();
            n += 1;
        }
        i += 1;
    }
    names
};

/// The rules a step takes, in the order of [`NAMES`].
const TAKEN: [Rule; Rule::ALL.len() - 2] = {
    let mut rules = [Rule::Axiom; Rule::ALL.len() - 2];
    let mut i = 0;
    let mut n = 0;
    while i < Rule::ALL.len() {
        if !matches!(Rule::ALL[i], Rule::Cut | Rule::Open) {
            rules[n] = Rule::ALL[i];
            n += 1;
        }
        i += 1;
    }
    rules
};

impl std::str::FromStr for Rule {
    type Err = Error;

    /// Reads a rule from its [`name`](Self::name) or an ASCII spelling of
    /// it: `/\R` or `andR` for `∧R`, `\/R1` or `orR1` for `∨R₁`, `->L` or
    /// `impL` for `→L`, `~R` or `notR` for `¬R`, `<->L2` or `iffL2` for
    /// `↔L₂`, `falseL` for `⊥L`, `trueR` for `⊤R`, `forallL` for `∀L`,
    /// `existsR` for `∃R`, and a subscript as a
    /// digit after the Unicode connective too (`∧L1`). A cut and an open
    /// goal are no rules a step takes, so `cut` and `open` are refused.
    fn from_str(text: &str) -> Result<Self, Error> {
        let digits = text.replace('1', "₁").replace('2', "₂");
        NAMES
            .iter()
            .position(|&name| name == text || name == digits)
            .map(|i| TAKEN[i])
            .or_else(|| {
                ASCII
                    .iter()
                    .find(|(name, _)| *name == text)
                    .map(|&(_, rule)| rule)
            })
            .ok_or_else(|| Error::UnknownName {
                what: "rule of LK or LJ",
                name: text.into(),
                known: &NAMES,
            })
    }
}

/// One inference of a derivation of LK or LJ: the sequent it concludes,
/// the rule, the formula the rule introduces, and its premises.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Inference {
    /// The hypotheses of the sequent concluded, a formula repeated as often
    /// as the sequent holds it.
    pub(crate) left: Vec<NodeId>,
    /// The formulas right of `⊢`.
    pub(crate) right: Vec<NodeId>,
    /// The rule.
    pub(crate) rule: Rule,
    /// The side and the position there of the formula the rule
    /// introduces, removes or copies; `None` for an axiom, a cut and an
    /// open goal.
    pub(crate) principal: Option<(Side, usize)>,
    /// The premises, in the rule's order.
    pub(crate) premises: Vec<InfId>,
    /// The term of a quantifier rule: the witness of `∀L` and `∃R`, the
    /// eigenvariable of `∀R` and `∃L`; `None` for every other rule.
    pub(crate) term: Option<FoTermId>,
}

impl Inference {
    /// Returns the hypotheses of the sequent concluded, a formula repeated
    /// as often as the sequent holds it.
    pub fn left(&self) -> &[NodeId] {
        &self.left
    }

    /// Returns the formulas right of `⊢`.
    pub fn right(&self) -> &[NodeId] {
        &self.right
    }

    /// Returns the rule.
    pub const fn rule(&self) -> Rule {
        self.rule
    }

    /// Returns the side and the position there of the formula the rule
    /// introduces, removes or copies; `None` for an axiom, a cut and an
    /// open goal.
    pub const fn principal(&self) -> Option<(Side, usize)> {
        self.principal
    }

    /// Returns the premises, in the rule's order.
    pub fn premises(&self) -> &[InfId] {
        &self.premises
    }

    /// Returns the term of a quantifier rule, an id of the derivation's
    /// first-order terms: the witness of `∀L` and `∃R`, the eigenvariable
    /// of `∀R` and `∃L`; `None` for every other rule.
    pub const fn term(&self) -> Option<FoTermId> {
        self.term
    }

    /// Returns the eigenvariable of a `∀R` or an `∃L`.
    pub(crate) fn eigenvariable(&self) -> Option<FoTermId> {
        if matches!(self.rule, Rule::ForallRight | Rule::ExistsLeft) {
            self.term
        } else {
            None
        }
    }

    /// Returns the formulas of one side.
    pub(crate) fn side(&self, side: Side) -> &[NodeId] {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }
}

/// A derivation of LK or LJ (see [`Rule`]): its inferences, premises
/// before their conclusion and the root last.
///
/// [`decide`](super::decide()) gives one for a valid sequent, from every
/// method but the SAT solver in classical logic. [`Image::read_back`]
/// reads one back from a linear proof of an image, and a proof in
/// progress makes one step by step (feature `interactive`).
///
/// # JSON
///
/// With the feature `serialize` a derivation is the document
/// `{"version": 8, "logic": "classical", "sequent": {…}, "inferences":
/// […]}` ([`wire`](crate::wire)). `sequent` is in the form of a
/// [`Sequent`](super::Sequent), with every formula the inferences name.
/// Each inference is `{"left": [ids], "right": [ids], "rule": name,
/// "principal": ["right", position], "premises": [indices]}`, without
/// `principal` or `premises` where it has none; a quantifier rule adds
/// `"term": k`, its term by index in the sequent's `fo`, where an
/// eigenvariable is `{"E": n}`. A derivation with LJ's
/// cut ([`Rule::Cut`]) is written at level 9, since a reader of level 8
/// does not know the rule. Reading one checks its shape (a tree, its root
/// last), not its rules: that is [`check`](Self::check)'s.
#[derive(Clone, Debug)]
pub struct Derivation {
    /// The logic.
    logic: Logic,
    /// The formulas, those an equivalence or a negation needs included.
    formulas: Formulas,
    /// The hypotheses of the sequent derived.
    left: Vec<NodeId>,
    /// The formulas right of `⊢` in the sequent derived.
    right: Vec<NodeId>,
    /// The inferences, premises before conclusions.
    inferences: Vec<Inference>,
    /// Whether the derivation is one its checker accepts, or a view of
    /// one. It is false for a derivation read from a document until
    /// [`check`](Derivation::check) accepts it.
    trusted: Trusted,
}

/// Whether a derivation is known to pass its check: a flag that
/// [`Derivation::check`] sets through a shared reference.
#[derive(Debug)]
struct Trusted(AtomicBool);

impl Trusted {
    /// Returns the flag, set or not.
    const fn new(trusted: bool) -> Self {
        Self(AtomicBool::new(trusted))
    }

    /// Returns whether the flag is set.
    fn get(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

impl Clone for Trusted {
    /// Copies the flag.
    fn clone(&self) -> Self {
        Self::new(self.get())
    }
}

impl Derivation {
    /// Returns the logic.
    pub fn logic(&self) -> Logic {
        self.logic
    }

    /// Returns the formulas the inferences refer to.
    pub fn formulas(&self) -> &Formulas {
        &self.formulas
    }

    /// Returns every inference, premises before conclusions, the root last.
    pub fn inferences(&self) -> &[Inference] {
        &self.inferences
    }

    /// Returns the derivation read as a term of the simply typed lambda
    /// calculus ([`lambda`](crate::lambda)), asking `stop` once per
    /// inference. The derivation is one of LJ, in either view of its
    /// structural rules.
    ///
    /// # Errors
    ///
    /// [`Error::NoTerm`] for a derivation of LK or one with open goals;
    /// [`Refusal::Stopped`] when `stop` fired. A derivation read from JSON
    /// is checked first unless [`check`](Self::check) has accepted it. So
    /// one that breaks a rule gets [`check`](Self::check)'s error, and so
    /// does a view of it with its structural rules left out: check the
    /// derivation, then take the view. [`Error::ReadBack`], a defect, for
    /// a derivation the read-back cannot read (a hypothesis out of scope,
    /// a premise or an operand missing): the checker refuses such a
    /// derivation, and the crate makes none.
    pub fn term(
        &self,
        limits: &crate::Limits,
        mut stop: impl FnMut(crate::Progress) -> bool,
    ) -> Result<crate::lambda::Term, Error> {
        // A derivation of LK or with an open goal has no term whether it
        // checks or not: the read-back refuses it before it reads anything.
        self.propositional("a lambda-term")?;
        let refused =
            self.logic == Logic::Classical || self.inferences.iter().any(|i| i.rule == Rule::Open);
        if !self.trusted.get() && !refused {
            self.check(limits, &mut stop)?;
        }
        crate::lambda::simple::read(self, limits, stop)
    }

    /// Returns the inference at `id`, which must belong to this derivation.
    ///
    /// # Panics
    ///
    /// For an id of another one, past its end.
    pub fn inference(&self, id: InfId) -> &Inference {
        &self.inferences[id.index()]
    }

    /// Returns the root.
    pub fn root(&self) -> InfId {
        // Every derivation has at least one inference: its builders make
        // one, and the JSON reader refuses a derivation without.
        InfId::new(self.inferences.len() as u32 - 1)
    }

    /// Returns the derivation of `logic` over `formulas` that concludes
    /// `left ⊢ right` with `inferences`, which have its shape: premises
    /// before conclusions, the root last, at least one. Nothing is
    /// checked: [`check`](Self::check) judges it.
    pub(crate) fn from_parts(
        logic: Logic,
        formulas: Formulas,
        (left, right): (Vec<NodeId>, Vec<NodeId>),
        inferences: Vec<Inference>,
    ) -> Self {
        Self {
            logic,
            formulas,
            left,
            right,
            inferences,
            trusted: Trusted::new(true),
        }
    }

    /// Returns the derivation marked as read from a document: its term
    /// is read only once [`check`](Self::check) accepts it.
    #[cfg(feature = "serialize")]
    pub(crate) fn untrusted(self) -> Self {
        self.trusted.0.store(false, Ordering::Relaxed);
        self
    }

    /// Fails with [`Error::FirstOrder`] naming `method` where the
    /// derivation is first-order: its arena is, or an inference names a
    /// quantifier rule, as a document may over a propositional arena.
    pub(crate) fn propositional(&self, method: &'static str) -> Result<(), Error> {
        self.formulas.propositional(method)?;
        let quantified = self.inferences.iter().any(|i| {
            matches!(
                i.rule,
                Rule::ForallLeft | Rule::ForallRight | Rule::ExistsLeft | Rule::ExistsRight
            )
        });
        if quantified {
            return Err(Error::FirstOrder { method });
        }
        Ok(())
    }

    /// Returns the sequent the derivation concludes, its two sides.
    pub fn sequent(&self) -> (&[NodeId], &[NodeId]) {
        (&self.left, &self.right)
    }

    /// Returns the derivation as `structural` shows it.
    ///
    /// Explicit, it is the derivation itself. Admissible, it leaves out
    /// every weakening and contraction whose branch the rules above
    /// close: the lowest conclusion of each such run becomes the
    /// conclusion of the first rule above it that is not structural, whose
    /// principal formula is found there by formula. A run with an open goal
    /// above it stays, so that an open goal shows its own sequent. The
    /// admissible view is no derivation [`check`](Self::check) accepts:
    /// its axioms have a context.
    pub fn shown(&self, structural: Structural) -> Cow<'_, Self> {
        match structural {
            Structural::Explicit => Cow::Borrowed(self),
            Structural::Admissible => Cow::Owned(self.admissible().0),
        }
    }

    /// Returns the derivation shown with its structural rules admissible,
    /// as [`shown`](Self::shown) does, and for each inference of it the
    /// inference of this derivation it shows.
    pub(crate) fn admissible(&self) -> (Self, Vec<InfId>) {
        let mut out: Vec<Inference> = Vec::with_capacity(self.inferences.len());
        // Per inference of this derivation, the one of `out` that shows it
        // and whether the rules above it close their branch.
        let mut shown: Vec<(InfId, bool)> = Vec::with_capacity(self.inferences.len());
        let mut origin = Vec::with_capacity(self.inferences.len());
        for (n, inference) in self.inferences.iter().enumerate() {
            if inference.rule.is_structural()
                && let [premise] = inference.premises[..]
                && let (above, true) = shown[premise.index()]
                // The rule above takes this conclusion where its principal
                // formula is in it, as in a derivation the checker accepts,
                // whose runs only add a formula or remove one of two
                // copies; elsewhere this inference is shown.
                && let Some(principal) = out[above.index()].principal.map_or(Some(None), |(side, at)| {
                    let id = out[above.index()].side(side)[at];
                    let found = inference.side(side).iter().position(|&x| x == id);
                    found.map(|at| Some((side, at)))
                })
                // A rule's eigenvariable stays out of its conclusion: a
                // weakening that adds a formula holding it is shown.
                && !out[above.index()].eigenvariable().is_some_and(|y| {
                    self.formulas
                        .mentions(inference.left.iter().chain(&inference.right).copied(), y)
                })
            {
                let target = &mut out[above.index()];
                target.left.clone_from(&inference.left);
                target.right.clone_from(&inference.right);
                target.principal = principal;
                shown.push((above, true));
                continue;
            }
            let premises = inference
                .premises
                .iter()
                .map(|p| shown[p.index()].0)
                .collect();
            // Fits a `u32`: no more inferences are shown than this
            // derivation has.
            let id = InfId::new(out.len() as u32);
            out.push(Inference {
                premises,
                ..inference.clone()
            });
            origin.push(InfId::new(n as u32));
            let closes = inference.rule != Rule::Open
                && inference.premises.iter().all(|p| shown[p.index()].1);
            shown.push((id, closes));
        }
        let derivation = Self {
            logic: self.logic,
            formulas: self.formulas.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
            inferences: out,
            trusted: self.trusted.clone(),
        };
        (derivation, origin)
    }

    /// Checks the derivation by the rules of its logic, as [`Rule`]
    /// describes them, and that its root concludes the derivation's
    /// sequent. Each inference is checked on its own, from the formulas as
    /// they stand, trusting nothing of whatever made the derivation.
    /// Checks `limits.work` inferences at most and asks `stop` before
    /// each.
    ///
    /// # Errors
    ///
    /// [`Error::ReadBack`] naming the first inference that breaks a rule;
    /// [`Refusal::Work`] past the bound and [`Refusal::Stopped`] when
    /// `stop` fired, neither a verdict on the derivation.
    pub fn check(
        &self,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        if let Some(limit) = limits.work
            && self.inferences.len() as u64 > limit
        {
            return Err(Error::Refused(Refusal::Work { limit }));
        }
        let calculus = self.logic.calculus();
        let fail = |n: usize, why: String| Error::ReadBack {
            calculus,
            reason: format!("inference {n} ({}): {why}", self.inferences[n].rule),
        };
        let root = self
            .inferences
            .len()
            .checked_sub(1)
            .ok_or(Error::ReadBack {
                calculus,
                reason: "no inference".to_owned(),
            })?;
        let root_inference = &self.inferences[root];
        if !same(&root_inference.left, &self.left) || !same(&root_inference.right, &self.right) {
            return Err(fail(
                root,
                "the root does not conclude the sequent".to_owned(),
            ));
        }
        for (n, inference) in self.inferences.iter().enumerate() {
            if stop(Progress::new(Phase::Check, 1, n as u64 + 1)) {
                return Err(Error::Refused(Refusal::Stopped {
                    phase: Phase::Check,
                }));
            }
            if inference.premises.iter().any(|p| p.index() >= n) {
                return Err(fail(n, "a premise does not precede it".to_owned()));
            }
            let premises: Vec<&Inference> = inference
                .premises
                .iter()
                .map(|&p| &self.inferences[p.index()])
                .collect();
            // An open witness belongs to a proof in progress: a finished
            // derivation names a term for every witness.
            if inference.term.is_some_and(|t| self.formulas.holds_meta(t)) {
                return Err(fail(n, "an open witness".to_owned()));
            }
            check_inference(&self.formulas, self.logic, inference, &premises)
                .map_err(|why| fail(n, why))?;
        }
        self.trusted.0.store(true, Ordering::Relaxed);
        Ok(())
    }
}

/// The formulas a rule adds to the context in each premise, by side: at
/// most two premises, each with at most two formulas, kept inline since
/// the checker makes one for every inference it checks.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Added {
    /// Each premise's formulas, in the rule's order.
    lists: [[Option<(Side, NodeId)>; 2]; 2],
}

impl Added {
    /// Returns the formulas each premise adds, one list per premise.
    fn of(premises: &[&[(Side, NodeId)]]) -> Self {
        let mut added = Self::default();
        for (list, premise) in added.lists.iter_mut().zip(premises) {
            for (slot, &formula) in list.iter_mut().zip(*premise) {
                *slot = Some(formula);
            }
        }
        added
    }

    /// Returns the formulas premise `k` adds.
    pub(super) fn premise(&self, k: usize) -> Vec<(Side, NodeId)> {
        self.lists[k].iter().flatten().copied().collect()
    }
}

/// Checks one inference of a derivation of `logic` over `formulas`
/// against its rule, given its premises: first the number of formulas
/// right of `⊢` its logic allows, then the rule. [`Derivation::check`]
/// calls it for every inference, and a proof in progress for every step
/// it takes.
///
/// Returns, for each premise, the formulas the rule adds to the
/// conclusion's context there (without its principal formula); a premise
/// holds them and the context, whole or the part a split gives it.
pub(super) fn check_inference(
    formulas: &Formulas,
    logic: Logic,
    inference: &Inference,
    premises: &[&Inference],
) -> Result<Added, String> {
    use Rule::*;
    use Side::{Left, Right};
    if inference.rule == Open {
        return Err("an open goal".to_owned());
    }
    if logic != Logic::Classical && inference.right.len() > 1 {
        return Err("more than one formula right of ⊢".to_owned());
    }
    // One rule reads an empty right side as ⊥ and another as any formula
    // (`WR`, a split `∨L`), which together give ex falso.
    if logic == Logic::Minimal && inference.right.is_empty() {
        return Err("no formula right of ⊢ in minimal logic".to_owned());
    }
    let f = formulas;
    let count = |n: usize| {
        if premises.len() == n {
            Ok(())
        } else {
            Err(format!("{} premises instead of {n}", premises.len()))
        }
    };
    let quantifier = matches!(
        inference.rule,
        ForallLeft | ForallRight | ExistsLeft | ExistsRight
    );
    if quantifier != inference.term.is_some() {
        return Err(if quantifier {
            "a quantifier rule without its term".to_owned()
        } else {
            "a term on a rule that takes none".to_owned()
        });
    }
    if inference.rule == Axiom {
        count(0)?;
        return match (&inference.left[..], &inference.right[..]) {
            ([a], [b]) if a == b && inference.principal.is_none() => Ok(Added::default()),
            _ => Err("not a sequent A ⊢ A".to_owned()),
        };
    }
    // The principal formula and its position, `None` for a cut.
    let principal = match inference.principal {
        Some((side, at)) => {
            let id = *inference
                .side(side)
                .get(at)
                .ok_or("the principal position lies outside the sequent")?;
            Some((side, at, id))
        }
        None => None,
    };
    // The sequent without the principal formula.
    let mut context = (inference.left.clone(), inference.right.clone());
    match principal {
        Some((Left, at, _)) => drop(context.0.remove(at)),
        Some((Right, at, _)) => drop(context.1.remove(at)),
        None => {}
    }
    let with = |added: &[(Side, NodeId)]| {
        let mut sequent = context.clone();
        for &(side, id) in added {
            match side {
                Left => sequent.0.push(id),
                Right => sequent.1.push(id),
            }
        }
        sequent
    };
    // One premise that is the context with the formulas given added.
    let unary = |added: &[(Side, NodeId)]| {
        count(1)?;
        if is(premises[0], &with(added)) {
            Ok(Added::of(&[added]))
        } else {
            Err("the premise is not what the rule makes of the conclusion".to_owned())
        }
    };
    // Two premises, each the context, whole or a part of a split, with
    // the formulas given added.
    let binary = |first: &[(Side, NodeId)], second: &[(Side, NodeId)]| {
        count(2)?;
        let added = || Added::of(&[first, second]);
        if is(premises[0], &with(first)) && is(premises[1], &with(second)) {
            return Ok(added());
        }
        // Split: what each premise holds beyond its formulas is a part
        // of the context, the two parts together all of it.
        let rest = |p: &Inference, added: &[(Side, NodeId)]| {
            let mut left = p.left.clone();
            let mut right = p.right.clone();
            for &(side, id) in added {
                let list = if side == Left { &mut left } else { &mut right };
                let i = list.iter().position(|&x| x == id)?;
                list.remove(i);
            }
            Some((left, right))
        };
        if let (Some(mut a), Some(b)) = (rest(premises[0], first), rest(premises[1], second)) {
            a.0.extend(b.0);
            a.1.extend(b.1);
            if same(&a.0, &context.0) && same(&a.1, &context.1) {
                return Ok(added());
            }
        }
        Err("the premises do not share or split the context".to_owned())
    };
    if inference.rule == Cut {
        // LK's derivations are the embedding's read-backs, which have no
        // cut, and its certificate has no term for one.
        if logic == Logic::Classical {
            return Err("LK's derivations have no cut".to_owned());
        }
        if principal.is_some() {
            return Err("a cut has no principal formula".to_owned());
        }
        count(2)?;
        // The cut formula is the one right of `⊢` in the first premise,
        // which LJ's count above holds to at most one.
        let Some(&cut) = premises[0].right.first() else {
            return Err("the first premise has no cut formula".to_owned());
        };
        return binary(&[(Right, cut)], &[(Left, cut)]);
    }
    let Some((side, _, principal)) = principal else {
        return Err("no principal formula".to_owned());
    };
    let node = f.node(principal);
    let leaf = || count(0).map(|()| Added::default());
    // The instance `A[t/x]` of the principal's body: the checker builds
    // nothing, so a premise that holds it finds it in the arena, and a
    // miss is a premise that does not.
    let instance = |body: NodeId| {
        let term = inference.term.ok_or("a quantifier rule without its term")?;
        formulas
            .instance(body, term)
            .ok_or_else(|| "the premise is not what the rule makes of the conclusion".to_owned())
    };
    // `∀R` and `∃L`: the term is an eigenvariable that occurs in no
    // formula of the conclusion.
    let eigenvariable = || {
        let term = inference.term.ok_or("a quantifier rule without its term")?;
        if !formulas.is_eigenvariable(term) {
            return Err("the term of the rule is no eigenvariable".to_owned());
        }
        if formulas.mentions(inference.left.iter().chain(&inference.right).copied(), term) {
            return Err("the eigenvariable occurs in the conclusion".to_owned());
        }
        Ok(())
    };
    match (inference.rule, side, node) {
        (WeakenLeft, Left, _) | (WeakenRight, Right, _) => unary(&[]),
        (ContractLeft, Left, _) => unary(&[(Left, principal), (Left, principal)]),
        (ContractRight, Right, _) => unary(&[(Right, principal), (Right, principal)]),
        (AndLeft, Left, Node::And(a, b)) => unary(&[(Left, a), (Left, b)]),
        (AndLeft1, Left, Node::And(a, _)) => unary(&[(Left, a)]),
        (AndLeft2, Left, Node::And(_, b)) => unary(&[(Left, b)]),
        (AndRight, Right, Node::And(a, b)) => binary(&[(Right, a)], &[(Right, b)]),
        (OrLeft, Left, Node::Or(a, b)) => binary(&[(Left, a)], &[(Left, b)]),
        (OrRight, Right, Node::Or(a, b)) => unary(&[(Right, a), (Right, b)]),
        (OrRight1, Right, Node::Or(a, _)) => unary(&[(Right, a)]),
        (OrRight2, Right, Node::Or(_, b)) => unary(&[(Right, b)]),
        (ImpliesLeft, Left, Node::Implies(a, b)) => binary(&[(Right, a)], &[(Left, b)]),
        (ImpliesRight, Right, Node::Implies(a, b)) => unary(&[(Left, a), (Right, b)]),
        // `¬A` as itself, or as `A → ⊥` in LJ. LK takes only its own
        // forms, the ones its certificate reads.
        (NotRight, Right, Node::Not(a)) if logic == Logic::Classical => unary(&[(Left, a)]),
        (NotLeft, Left, Node::Not(_)) if logic == Logic::Classical && premises.len() != 1 => {
            Err("LK's ¬L has one premise".to_owned())
        }
        (NotRight, Right, Node::Not(a)) => unary(&[(Left, a)]).or_else(|_| {
            let falsity = formulas.ids.get(&Node::False).copied();
            unary(&[
                (Left, a),
                (Right, falsity.ok_or("no ⊥ for the form of →R")?),
            ])
        }),
        (NotLeft, Left, Node::Not(a)) if premises.len() == 1 => unary(&[(Right, a)]),
        (NotLeft, Left, Node::Not(a)) => {
            let falsity = formulas.ids.get(&Node::False).copied();
            let falsity = falsity.ok_or("no ⊥ for the form of →L")?;
            binary(&[(Right, a)], &[(Left, falsity)])
        }
        (IffLeft | IffLeft1 | IffLeft2 | IffRight, _, Node::Iff(a, b)) => {
            let forward = formulas.ids.get(&Node::Implies(a, b)).copied();
            let backward = formulas.ids.get(&Node::Implies(b, a)).copied();
            let (Some(forward), Some(backward)) = (forward, backward) else {
                return Err("the implications of the equivalence are missing".to_owned());
            };
            match (inference.rule, side) {
                (IffLeft, Left) => unary(&[(Left, forward), (Left, backward)]),
                (IffLeft1, Left) => unary(&[(Left, forward)]),
                (IffLeft2, Left) => unary(&[(Left, backward)]),
                (IffRight, Right) => binary(&[(Right, forward)], &[(Right, backward)]),
                _ => Err("the rule does not apply on this side".to_owned()),
            }
        }
        // G3's form keeps the principal formula in the premise, G1's drops
        // it; its contraction is admissible.
        (ForallLeft, Left, Node::Forall(body)) => {
            let a = instance(body)?;
            unary(&[(Left, a), (Left, principal)]).or_else(|_| unary(&[(Left, a)]))
        }
        (ExistsRight, Right, Node::Exists(body)) => {
            let a = instance(body)?;
            // Only LK's premise has room for the kept formula.
            if logic == Logic::Classical {
                unary(&[(Right, a), (Right, principal)]).or_else(|_| unary(&[(Right, a)]))
            } else {
                unary(&[(Right, a)])
            }
        }
        (ForallRight, Right, Node::Forall(body)) => {
            eigenvariable()?;
            unary(&[(Right, instance(body)?)])
        }
        (ExistsLeft, Left, Node::Exists(body)) => {
            eigenvariable()?;
            unary(&[(Left, instance(body)?)])
        }
        (FalseLeft, Left, Node::False) if logic != Logic::Minimal => leaf(),
        (FalseLeft, ..) if logic == Logic::Minimal => {
            Err("minimal logic has no ex falso".to_owned())
        }
        (FalseRight, Right, Node::False) | (TrueLeft, Left, Node::True) => unary(&[]),
        (TrueRight, Right, Node::True) => leaf(),
        _ => Err("the rule does not apply to the principal formula".to_owned()),
    }
}

impl Derivation {
    /// Returns the bytes a drawing of the derivation is estimated at, by
    /// the measure [`Limits::derivation_bytes`] bounds: a front end draws
    /// it only within that bound. The derivation is built within the
    /// memory bound alone, so a valid verdict and its checked derivation
    /// stand where a drawing of it would be too large.
    pub fn drawn_bytes(&self) -> u64 {
        let widths = widths(&self.formulas);
        self.inferences.iter().fold(0u64, |sum, i| {
            sum.saturating_add(inference_bytes(&widths, &i.left, &i.right))
        })
    }

    /// Returns the width in characters and the number of lines of the tree
    /// that [`write_text`](Self::write_text) draws under the options,
    /// without drawing it.
    pub fn text_size(&self, options: &TextOptions) -> (usize, usize) {
        crate::linear::proofs::fmt::text_size(self, options)
    }

    /// Writes the derivation as a tree of sequents `Γ ⊢ Δ`, one line per
    /// row, without a trailing newline, as
    /// [`crate::linear::Derivation::write_text`] draws a linear one, and
    /// asks `stop` before every piece of a row. The labels are those of
    /// [`Rule`]. A label table ([`Labels::Table`]) is keyed by the linear
    /// rules, so under one the labels stay upright.
    ///
    /// # Errors
    ///
    /// [`Refusal::Stopped`] when `stop` fired, and
    /// [`Error::WriteFailed`] when `out` refused the text.
    pub fn write_text(
        &self,
        options: &TextOptions,
        out: &mut impl Write,
        stop: impl FnMut(crate::limits::Progress) -> bool,
    ) -> Result<(), Error> {
        crate::linear::proofs::fmt::write_text(
            self,
            options,
            out,
            crate::limits::counting(stop, crate::limits::Phase::Write),
        )
    }

    /// Writes the derivation as a numbered list of its inferences, one per
    /// line, premises before their conclusion and the root last, as
    /// [`crate::linear::Derivation::write_steps`] reads a linear one out.
    /// It is the reading of a derivation for a screen reader. Asks `stop`
    /// before every line.
    ///
    /// # Errors
    ///
    /// [`Refusal::Stopped`] when `stop` fired, and
    /// [`Error::WriteFailed`] when `out` refused the text.
    pub fn write_steps(
        &self,
        out: &mut impl Write,
        stop: impl FnMut(crate::limits::Progress) -> bool,
    ) -> Result<(), Error> {
        crate::linear::proofs::fmt::write_steps(
            self,
            out,
            crate::limits::counting(stop, crate::limits::Phase::Write),
            |out, id| self.write_sequent(out, id),
        )
    }
}

impl Display for Derivation {
    /// Draws the derivation as a tree of sequents under the default
    /// [`TextOptions`], one line per row, without a trailing newline.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.write_text(&TextOptions::default(), f, |_| false)
            .map_err(|_| std::fmt::Error)
    }
}

impl Drawn for Derivation {
    fn rules(&self) -> usize {
        Rule::ALL.len()
    }

    fn open(&self) -> Option<usize> {
        Some(Rule::Open as usize)
    }

    fn markup<'a>(&'a self, rule: usize, labels: &'a Labels) -> Option<&'a str> {
        let rule = Rule::ALL[rule];
        match labels {
            _ if rule == Rule::Open => None,
            Labels::Upright | Labels::Table(_) => Some(rule.markup(false)),
            Labels::Subscript => Some(rule.markup(true)),
            Labels::Off => None,
        }
    }

    fn len(&self) -> usize {
        self.inferences.len()
    }

    fn root(&self) -> InfId {
        Derivation::root(self)
    }

    fn premises(&self, id: InfId) -> &[InfId] {
        &self.inference(id).premises
    }

    fn rule(&self, id: InfId) -> usize {
        self.inference(id).rule as usize
    }

    fn name(&self, id: InfId) -> &'static str {
        self.inference(id).rule.name()
    }

    fn times(&self, _: InfId) -> u32 {
        1
    }

    fn write_sequent(&self, out: &mut impl Write, id: InfId) -> FmtResult {
        let inference = self.inference(id);
        let sides = (&inference.left[..], &inference.right[..]);
        TEXT.ordinary(out, &self.formulas, sides, false, false)
    }

    #[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
    fn sequent(
        &self,
        out: &mut String,
        notation: &Notation,
        id: InfId,
        aligned: bool,
        marks: bool,
    ) {
        let inference = self.inference(id);
        let sides = (&inference.left[..], &inference.right[..]);
        notation
            .ordinary(out, &self.formulas, sides, aligned, marks)
            .unwrap();
    }

    #[cfg(feature = "svg")]
    fn positions(&self, id: InfId) -> Vec<usize> {
        let inference = self.inference(id);
        (0..inference.left.len() + inference.right.len()).collect()
    }
}

/// Returns whether two lists are the same multiset. Lists of different
/// lengths, or equal as they stand, are answered at once; short lists are
/// compared by counting, and only long ones by sorting copies, so that
/// the checker allocates nothing for an inference of short sequents.
pub(super) fn same(a: &[NodeId], b: &[NodeId]) -> bool {
    /// The longest lists compared by counting, which takes the square of
    /// their length in comparisons.
    const SHORT: usize = 16;
    if a.len() != b.len() {
        return false;
    }
    if a == b {
        return true;
    }
    if a.len() <= SHORT {
        let count = |list: &[NodeId], x: NodeId| list.iter().filter(|&&y| y == x).count();
        return a.iter().all(|&x| count(a, x) == count(b, x));
    }
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    a.sort_unstable();
    b.sort_unstable();
    a == b
}

/// Returns, for every formula of the arena, the characters it is written
/// with, every binary subformula in brackets, saturating: the size
/// estimate's measure of a formula.
pub(super) fn widths(formulas: &Formulas) -> Vec<u64> {
    let mut weight: Vec<u64> = Vec::with_capacity(formulas.len());
    extend_widths(formulas, &mut weight);
    weight
}

/// Extends `weight`, the [`widths`] of the arena's first nodes, to every
/// node of the arena: for a caller that adds nodes as it goes.
pub(super) fn extend_widths(formulas: &Formulas, weight: &mut Vec<u64>) {
    // One pass: every subformula precedes the formulas built from it.
    for node in &formulas.nodes[weight.len()..] {
        let of = |k: NodeId| weight[k.index()];
        let w = match *node {
            Node::Atom(a) => formulas.atom_name(a).chars().count() as u64,
            Node::True | Node::False => 1,
            Node::Not(a) => of(a).saturating_add(1),
            // ` ∧ ` and the brackets.
            Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                of(a).saturating_add(of(b)).saturating_add(5)
            }
            // `∀x. `, the brackets, and room for a renamed variable.
            Node::Forall(a) | Node::Exists(a) => {
                let name = formulas
                    .first_order()
                    .binder_name(NodeId(weight.len() as u32))
                    .map_or(1, |n| n.chars().count() as u64);
                of(a)
                    .saturating_add(name.saturating_mul(2))
                    .saturating_add(9)
            }
        };
        weight.push(w);
    }
}

/// Returns the bytes an inference concluding `left ⊢ right` is estimated
/// at by the measure of [`Size::bytes`]: [`Size::BYTES_PER_INFERENCE`],
/// and [`Size::BYTES_PER_CHARACTER`] per character of its sequent
/// written, each formula with two for its separator; saturating.
pub(super) fn inference_bytes(widths: &[u64], left: &[NodeId], right: &[NodeId]) -> u64 {
    // Two for the turnstile and a space beside it. `Γ ⊢ Δ` writes one
    // separator fewer than formulas on each side and ` ⊢ ` between them,
    // so with two per formula the count is never below the sequent as
    // written, whichever side is empty.
    let characters = left.iter().chain(right).fold(2u64, |sum, k| {
        sum.saturating_add(widths[k.index()]).saturating_add(2)
    });
    Size::BYTES_PER_INFERENCE.saturating_add(characters.saturating_mul(Size::BYTES_PER_CHARACTER))
}

/// Returns whether an inference concludes the sequent given, as multisets.
fn is(inference: &Inference, sequent: &(Vec<NodeId>, Vec<NodeId>)) -> bool {
    same(&inference.left, &sequent.0) && same(&inference.right, &sequent.1)
}

/// A formula followed through its classical negations.
struct Unwound {
    /// The negations passed, outermost first, each with its side.
    chain: Vec<(NodeId, Side)>,
    /// The formula under them, its translation function and its side.
    core: (NodeId, u8, Side),
}

/// What a linear occurrence is the image of.
#[derive(Clone, Copy, Debug)]
struct Tag {
    /// The ordinary formula, as the sequents show it.
    node: NodeId,
    /// Its translation function.
    function: u8,
    /// The `!`s above the pattern's connective still to pass.
    bangs: u8,
    /// Its side.
    side: Side,
}

impl Image {
    /// Unfolds a proof of the image into the derivation the read-back
    /// reads: two-sided for an image in ILL, one-sided otherwise, never
    /// compact, within `limits` and until `stop` fires, as
    /// [`Proof::derivation_within`] does.
    pub(crate) fn linear_derivation<'p>(
        &self,
        proof: &'p Proof,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<crate::linear::Derivation<'p>, Error> {
        let sides = if self.mode.intuitionistic {
            Sides::Two
        } else {
            Sides::One
        };
        let view = ViewOptions::default()
            .with_compact(Compact::Never)
            .with_sides(sides);
        // What is built is bounded by the memory; `derivation_bytes`
        // bounds what is drawn of the read-back.
        let limits = limits.with_derivation_bytes(None);
        proof.derivation_within(&view, &limits, stop)
    }

    /// Returns the derivation of the ordinary sequent that a proof of the
    /// image reads back as: one of LK in classical logic, of LJ in
    /// intuitionistic and minimal logic. [`Derivation::check`] checks it.
    ///
    /// A linear rule on an image is the rule of its connective. A
    /// dereliction or a promotion is no inference, and a contraction or a
    /// weakening is one of the ordinary formula. A classical negation has
    /// no image of its own, so it is a `¬` rule where its operand is taken
    /// apart. The proof is unfolded first, never compact (two-sided for an
    /// image in ILL), within `limits.memory_bytes`;
    /// `limits.derivation_bytes` bounds what is drawn, not what is built.
    /// `stop` is asked as that unfolding asks it,
    /// and once per inference read.
    ///
    /// # Errors
    ///
    /// [`Error::GoalProof`] for a proof of a goal other than the image's
    /// roots, whose read-back would conclude another sequent; as
    /// [`Proof::derivation_within`] for the unfolding; [`Error::ReadBack`]
    /// for a proof that is not of this image, or a rule with no reading (a
    /// cut); [`Refusal::Stopped`] when `stop` fired.
    pub fn read_back(
        &self,
        proof: &Proof,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Derivation, Error> {
        self.read_back_with(proof, limits, &mut stop)
    }

    /// [`read_back`](Self::read_back) with the stop as a trait object, so
    /// that the read-back is compiled once, not once per caller's closure.
    /// Its `expect`s state that the forest's preorder tags a parent before
    /// its children, which the walk establishes.
    fn read_back_with(
        &self,
        proof: &Proof,
        limits: &Limits,
        mut stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Derivation, Error> {
        if proof.goal().is_some() {
            return Err(Error::GoalProof);
        }
        let linear = &self.linear_derivation(proof, limits, &mut stop)?;
        let calculus = self.logic.calculus();
        let fail = |reason: String| Error::ReadBack { calculus, reason };
        let forest = linear.forest();
        if forest.sequent() != &self.sequent {
            return Err(fail("the proof is not of the image".to_owned()));
        }
        // The derivation's arena, which gains the instances of quantified
        // formulas the proof holds.
        let mut arena = self.ordinary.formulas.clone();
        let formulas = &self.ordinary.formulas;
        // Every occurrence's tag, parents before children.
        let mut tags: Vec<Option<Tag>> = vec![None; forest.len()];
        for (&root, info) in forest.roots().iter().zip(&self.roots) {
            tags[root.index()] = Some(Tag {
                node: info.node,
                function: info.function,
                bangs: info.bangs,
                side: info.side,
            });
        }
        for o in forest.ids() {
            let tag = tags[o.index()].expect("a parent is tagged before its children");
            if tag.bangs > 0 {
                if let Some(child) = forest.left(o) {
                    tags[child.index()] = Some(Tag {
                        bangs: tag.bangs - 1,
                        ..tag
                    });
                }
                continue;
            }
            let (node, function, side) = self.unwind(tag.node, tag.function, tag.side).core;
            let p = pattern(
                formulas,
                &self.extra,
                self.translation,
                self.logic,
                node,
                function,
            );
            for (k, child) in forest.children(o).enumerate() {
                let Some((x, fx, extra)) = p.operands[k] else {
                    return Err(fail(format!("occurrence {} has no operand {k}", o.index())));
                };
                let px = pattern(formulas, &self.extra, self.translation, self.logic, x, fx);
                let flips = match p.core {
                    Core::Lollipop => k == 0,
                    Core::Atom
                    | Core::Unit(_)
                    | Core::Tensor
                    | Core::Par
                    | Core::With
                    | Core::Plus
                    | Core::Pass
                    | Core::Forall
                    | Core::Exists => false,
                };
                // The classical implication's antecedent is on the other
                // side through its function.
                let side = if self.translation == Translation::Affine {
                    if fx == 0 { Side::Right } else { Side::Left }
                } else if flips {
                    side.flip()
                } else {
                    side
                };
                tags[child.index()] = Some(Tag {
                    node: x,
                    function: fx,
                    bangs: px.bangs + extra,
                    side,
                });
            }
        }
        let tag = |o: OccId| tags[o.index()].expect("every occurrence is tagged");
        let mut instances = Instances::new(proof, &mut arena).map_err(&fail)?;
        let propositional = linear.framing().is_none();
        // What the instances add to the arena is held beside the
        // inferences: a member past the forest builds its formula, and a
        // deep nest of binders builds hundreds of bytes a member.
        let mut grown = if propositional {
            ArenaSize::default()
        } else {
            ArenaSize {
                bytes: 0,
                ..ArenaSize::of(&arena, &ArenaSize::default())
            }
        };

        let mut out = Out {
            inferences: Vec::new(),
            held: 0,
            limit: limits.memory_bytes.unwrap_or(u64::MAX),
        };
        // Per linear inference, the ordinary inference that stands for it.
        let mut mapped: Vec<InfId> = Vec::with_capacity(linear.inferences().len());
        for (i, inference) in linear.inferences().iter().enumerate() {
            if stop(Progress::new(Phase::ReadBack, 1, i as u64 + 1)) {
                return Err(Error::Refused(Refusal::Stopped {
                    phase: Phase::ReadBack,
                }));
            }
            // The linear derivation is held beside the one read back.
            out.held = out.held.saturating_add(
                (size_of_val(inference) + size_of_val(inference.sequent.as_slice())) as u64,
            );
            // A member of the forest is its occurrence's formula; only an
            // instance is built. A proof without instances, every proof of
            // a propositional image, takes the plain loop.
            let members: Vec<(NodeId, Side)> = if propositional {
                inference
                    .sequent
                    .iter()
                    .map(|&m| {
                        let t = tag(m.occ());
                        (t.node, t.side)
                    })
                    .collect()
            } else {
                let mut members = Vec::with_capacity(inference.sequent.len());
                for &m in &inference.sequent {
                    let t = tag(linear.occurrence(m));
                    let node = if instances.is_instance(m) {
                        instances.node(&mut arena, m, t.node, calculus)?
                    } else {
                        t.node
                    };
                    members.push((node, t.side));
                }
                let now = ArenaSize::of(&arena, &grown);
                out.held = out.held.saturating_add(now.bytes - grown.bytes);
                grown = now;
                members
            };
            let premises: Vec<InfId> = inference
                .premises
                .iter()
                .map(|p| mapped[p.index()])
                .collect();
            let rule = inference.rule.rule;
            let next = match rule {
                Linear::Promotion | Linear::Dereliction => premises[0],
                Linear::Contraction | Linear::Weakening | Linear::AffineWeakening => {
                    let at = inference
                        .principal
                        .expect("a structural rule has a principal formula");
                    let (_, side) = members[at];
                    let rule = match (rule, side) {
                        (Linear::Contraction, Side::Left) => Rule::ContractLeft,
                        (Linear::Contraction, Side::Right) => Rule::ContractRight,
                        (_, Side::Left) => Rule::WeakenLeft,
                        (_, Side::Right) => Rule::WeakenRight,
                    };
                    out.push(&members, at, rule, premises)?
                }
                // Every proof the search finds is cut-free, so the
                // read-back needs no rule for a cut.
                Linear::Cut => {
                    return Err(fail(
                        "a cut, which the read-back has no rule for".to_owned(),
                    ));
                }
                Linear::Ax => {
                    let mut members = members;
                    let chains: Vec<_> = (0..2)
                        .map(|at| {
                            let (node, side) = members[at];
                            let Unwound { chain, core } = self.unwind_in(&arena, node, 0, side);
                            members[at] = (core.0, core.2);
                            (at, chain)
                        })
                        .collect();
                    let mut id = out.push(&members, usize::MAX, Rule::Axiom, premises)?;
                    for (at, chain) in chains {
                        id = out.negations(&mut members, at, chain, id, &mut stop)?;
                    }
                    id
                }
                rule @ Linear::Tensor
                | rule @ Linear::Par
                | rule @ Linear::One
                | rule @ Linear::Bot
                | rule @ Linear::With
                | rule @ Linear::PlusLeft
                | rule @ Linear::PlusRight
                | rule @ Linear::Top
                | rule @ Linear::Mix
                | rule @ Linear::Open
                | rule @ Linear::Forall
                | rule @ Linear::Exists => {
                    let at = inference
                        .principal
                        .ok_or_else(|| fail("no principal formula".to_owned()))?;
                    let mut members = members;
                    let (node, side) = members[at];
                    let Unwound { chain, core } = self.unwind_in(&arena, node, 0, side);
                    members[at] = (core.0, core.2);
                    let ordinary =
                        self.rule_of(arena.node(core.0), core.2, rule)
                            .ok_or_else(|| {
                                fail(format!(
                                    "the linear rule {rule} on {} has no reading",
                                    arena.formula(core.0)
                                ))
                            })?;
                    // A quantifier rule's term is the one its premise's new
                    // instance was opened with.
                    let term = if matches!(rule, Linear::Forall | Linear::Exists) {
                        let premise = linear.inference(inference.premises[0]);
                        let body =
                            opened(&premise.sequent, &inference.sequent, at).ok_or_else(|| {
                                fail("a quantifier rule without its instance".to_owned())
                            })?;
                        let term = instances.witness(body).map_err(&fail)?;
                        // The checker names no open witness as a term.
                        if arena.holds_meta(term) {
                            return Err(fail(OPEN_WITNESS.to_owned()));
                        }
                        Some(term)
                    } else {
                        None
                    };
                    let id = out.push(&members, at, ordinary, premises)?;
                    out.inferences[id.index()].term = term;
                    out.negations(&mut members, at, chain, id, &mut stop)?
                }
            };
            mapped.push(next);
        }
        // An instance of an equivalence needs its implications, which the
        // checker looks up.
        if arena.len() > formulas.len() {
            super::translate::Extra::add(&mut arena)?;
        }
        Ok(Derivation {
            logic: self.logic,
            formulas: arena,
            left: self.ordinary.left.clone(),
            right: self.ordinary.right.clone(),
            inferences: out.inferences,
            trusted: Trusted::new(true),
        })
    }

    /// Returns the classical negations above a formula, which have no
    /// image of their own, and the formula under them, whose connective
    /// its image has. Other translations pass no negation.
    fn unwind(&self, node: NodeId, function: u8, side: Side) -> Unwound {
        self.unwind_in(&self.ordinary.formulas, node, function, side)
    }

    /// [`unwind`](Self::unwind) in `formulas`, an arena that extends the
    /// image's with instances.
    fn unwind_in(
        &self,
        formulas: &Formulas,
        mut node: NodeId,
        mut function: u8,
        mut side: Side,
    ) -> Unwound {
        let mut chain = Vec::new();
        if self.translation == Translation::Affine {
            while let Node::Not(a) = formulas.node(node) {
                chain.push((node, side));
                node = a;
                side = side.flip();
                function = 1 - function;
            }
        }
        Unwound {
            chain,
            core: (node, function, side),
        }
    }

    /// Returns the ordinary rule that the linear rule `rule`, by its
    /// classical name, stands for on the image of `node` on `side`; `None`
    /// where it stands for none.
    fn rule_of(&self, node: Node, side: Side, rule: Linear) -> Option<Rule> {
        use Linear as L;
        use Side::{Left, Right};
        Some(match (node, side, rule) {
            (Node::And(..), Right, L::With | L::Tensor) => Rule::AndRight,
            (Node::And(..), Left, L::Par) => Rule::AndLeft,
            (Node::And(..), Left, L::PlusLeft) => Rule::AndLeft1,
            (Node::And(..), Left, L::PlusRight) => Rule::AndLeft2,
            (Node::Or(..), Right, L::Par) => Rule::OrRight,
            (Node::Or(..), Right, L::PlusLeft) => Rule::OrRight1,
            (Node::Or(..), Right, L::PlusRight) => Rule::OrRight2,
            (Node::Or(..), Left, L::With) => Rule::OrLeft,
            (Node::Implies(..), Right, L::Par) => Rule::ImpliesRight,
            (Node::Implies(..), Left, L::Tensor | L::With) => Rule::ImpliesLeft,
            (Node::Not(_), Right, L::Par) => Rule::NotRight,
            (Node::Not(_), Left, L::Tensor) => Rule::NotLeft,
            (Node::Iff(..), Right, L::With | L::Tensor) => Rule::IffRight,
            (Node::Iff(..), Left, L::Par) => Rule::IffLeft,
            (Node::Iff(..), Left, L::PlusLeft) => Rule::IffLeft1,
            (Node::Iff(..), Left, L::PlusRight) => Rule::IffLeft2,
            (Node::True, Right, L::Top | L::One) => Rule::TrueRight,
            (Node::True, Left, L::Bot) => Rule::TrueLeft,
            (Node::False, Left, L::Top) => Rule::FalseLeft,
            (Node::False, Right, L::Bot) => Rule::FalseRight,
            (Node::Forall(_), Right, L::Forall) => Rule::ForallRight,
            (Node::Forall(_), Left, L::Exists) => Rule::ForallLeft,
            (Node::Exists(_), Right, L::Exists) => Rule::ExistsRight,
            (Node::Exists(_), Left, L::Forall) => Rule::ExistsLeft,
            _ => return None,
        })
    }
}

/// The size of an arena as the read-back charges it: its nodes, terms,
/// argument lists and atoms with their keys, each with the hash entry that
/// finds it.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct ArenaSize {
    /// The formulas.
    nodes: usize,
    /// The terms.
    terms: usize,
    /// The arguments of every list.
    args: usize,
    /// The lists.
    lists: usize,
    /// The atoms.
    atoms: usize,
    /// The bytes all of them hold, estimated.
    pub(super) bytes: u64,
}

impl ArenaSize {
    /// The bytes a formula is charged, with its hash entry and a binder's
    /// name: the read-back of a sequent of 600 and of 1 500 nested `∀`
    /// held about 160 bytes a formula of the arena it grew.
    const NODE: u64 = 192;
    /// The bytes a term is charged, with its hash entry.
    const ENTRY: u64 = 64;
    /// The bytes an atom is charged besides its key's text, which is held
    /// twice (its name and the key that finds it).
    const ATOM: u64 = 96;

    /// Returns the size of `formulas`, counting the atoms' keys past
    /// `before`'s atoms only, since the arena only grows.
    pub(super) fn of(formulas: &Formulas, before: &Self) -> Self {
        let tables = formulas.first_order();
        let (nodes, terms) = (formulas.len(), tables.terms.len());
        let (args, lists) = (tables.args.len(), tables.lists.len());
        let atoms = formulas.atom_names().len();
        let keys: u64 = formulas.atom_names()[before.atoms.min(atoms)..]
            .iter()
            .map(|name| 2 * name.len() as u64 + Self::ATOM)
            .sum();
        let grown = |now: usize, then: usize, each: u64| (now.saturating_sub(then) as u64) * each;
        let bytes = before
            .bytes
            .saturating_add(grown(nodes, before.nodes, Self::NODE))
            .saturating_add(grown(terms, before.terms, Self::ENTRY))
            .saturating_add(grown(args, before.args, 8))
            .saturating_add(grown(lists, before.lists, 32))
            .saturating_add(keys);
        Self {
            nodes,
            terms,
            args,
            lists,
            atoms,
            bytes,
        }
    }
}

/// The inferences read back, and the memory they and the linear
/// derivation hold, against the bound.
struct Out {
    /// The inferences made, premises first.
    inferences: Vec<Inference>,
    /// The bytes held.
    held: u64,
    /// The bound on them.
    limit: u64,
}

impl Out {
    /// Adds the inference concluding `members` by `rule`, with the formula
    /// at `at` principal (none for an axiom), and returns its id; refuses
    /// past the memory bound or 2³² inferences.
    fn push(
        &mut self,
        members: &[(NodeId, Side)],
        at: usize,
        rule: Rule,
        premises: Vec<InfId>,
    ) -> Result<InfId, Error> {
        self.held = self.held.saturating_add(
            (size_of::<Inference>() + (members.len() + premises.len()) * size_of::<u32>()) as u64,
        );
        if self.held > self.limit {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::ReadBack,
                limit_bytes: self.limit,
                needed_bytes: None,
            }));
        }
        let n = self.inferences.len();
        let id = u32::try_from(n).map_err(|_| {
            Error::Refused(Refusal::Index {
                what: Space::Inference,
                count: n as u64 + 1,
                most: u64::from(u32::MAX),
            })
        })?;
        let lefts = members
            .iter()
            .filter(|&&(_, side)| side == Side::Left)
            .count();
        let (mut left, mut right) = (
            Vec::with_capacity(lefts),
            Vec::with_capacity(members.len() - lefts),
        );
        let mut principal = None;
        for (i, &(node, side)) in members.iter().enumerate() {
            let list = if side == Side::Left {
                &mut left
            } else {
                &mut right
            };
            if i == at {
                principal = Some((side, list.len()));
            }
            list.push(node);
        }
        self.inferences.push(Inference {
            term: None,
            left,
            right,
            rule,
            principal,
            premises,
        });
        Ok(InfId::new(id))
    }

    /// Adds below `id` the `¬` inferences that take the formula at `at`
    /// from the formula under its negations back through the negations of
    /// `chain`, the innermost first, and returns the last one; asks `stop`
    /// once per inference, since a formula may be under many negations.
    fn negations(
        &mut self,
        members: &mut [(NodeId, Side)],
        at: usize,
        chain: Vec<(NodeId, Side)>,
        mut id: InfId,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<InfId, Error> {
        for (node, side) in chain.into_iter().rev() {
            if stop(Progress::new(
                Phase::ReadBack,
                1,
                self.inferences.len() as u64,
            )) {
                return Err(Error::Refused(Refusal::Stopped {
                    phase: Phase::ReadBack,
                }));
            }
            members[at] = (node, side);
            let rule = match side {
                Side::Left => Rule::NotLeft,
                Side::Right => Rule::NotRight,
            };
            id = self.push(members, at, rule, vec![id])?;
        }
        Ok(id)
    }
}

/// The read-back's reason where a quantifier's term would be an open
/// witness of the sequent (a parameter the image reads as a constant):
/// no derivation names one as a term, so [`decide()`](super::decide()) leaves
/// such a sequent unknown.
pub(super) const OPEN_WITNESS: &str =
    "a quantifier's term would be an open witness of the sequent, which no derivation names";

/// The formulas of a first-order proof's members past its forest, each
/// the formula of its occurrence with the frame's terms in place of the
/// bound variables, built in the derivation's arena once per member.
struct Instances<'a> {
    /// The proof's table of instances; `None` for a propositional proof.
    table: Option<&'a crate::linear::Instances>,
    /// The forest's length: the first member the table numbers.
    occurrences: usize,
    /// The number of the sequent's own terms, which keep their ids in the
    /// arena, since the image copies the ordinary tables.
    base: usize,
    /// Per term of the table, its id in the arena.
    terms: Vec<FoTermId>,
    /// The formula of every member past the forest built so far.
    built: crate::hash::HashMap<crate::linear::occurrences::Member, NodeId>,
}

impl<'a> Instances<'a> {
    /// Returns the instances of `proof`, its table's terms interned into
    /// `arena` in the table's order, each after the terms it names; or why
    /// a term cannot be read.
    fn new(proof: &'a Proof, arena: &mut Formulas) -> Result<Self, String> {
        use crate::first_order::{Eigen, FoTerm};
        let table = proof.instances();
        let fo = &proof.forest().sequent().fo;
        let base = fo.terms.len();
        let mut terms = Vec::new();
        if let Some(table) = table {
            // The proof's eigenvariables are numbered past the arena's own
            // (a sequent over a derivation's arena holds some), which the
            // image reads as constants of symbols past the arena's.
            let tables = arena.first_order();
            let offset = tables
                .terms
                .iter()
                .filter_map(|t| match *t {
                    FoTerm::Eigen(e) => Some(e.get().saturating_add(1)),
                    FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => None,
                })
                .max()
                .unwrap_or(0);
            let symbols = tables.symbols.len();
            let both = crate::linear::occurrences::instances::Terms { fo, table };
            terms.reserve(table.terms.len());
            for &term in &table.terms {
                let made = match term {
                    FoTerm::Eigen(a) => FoTerm::Eigen(Eigen::new(
                        a.get()
                            .checked_add(offset)
                            .ok_or("an eigenvariable past the numbers")?,
                    )),
                    // A parameter of the arena, as the image's constant.
                    FoTerm::App(f, _) if f.index() >= symbols => {
                        let at = fo
                            .terms
                            .iter()
                            .position(|&t| matches!(t, FoTerm::App(g, _) if g == f))
                            .ok_or("a term of a symbol the sequent lacks")?;
                        terms.push(FoTermId::new(at as u32));
                        continue;
                    }
                    FoTerm::App(f, args) => {
                        let items = both
                            .list(args)
                            .iter()
                            .map(|&t| match t.index().checked_sub(base) {
                                None => Some(t),
                                Some(at) => terms.get(at).copied(),
                            })
                            .collect::<Option<Vec<FoTermId>>>()
                            .ok_or("a term of the proof names a later one")?;
                        FoTerm::App(f, arena.interner().list(&items))
                    }
                    FoTerm::Bound(_) | FoTerm::Meta(_) => {
                        return Err("an open term among the proof's terms".to_owned());
                    }
                };
                terms.push(arena.interner().term(made));
            }
        }
        Ok(Self {
            table,
            occurrences: proof.forest().len(),
            base,
            terms,
            built: crate::hash::HashMap::default(),
        })
    }

    /// Returns the id in the arena of the proof's term `term`.
    fn term(&self, term: FoTermId) -> Result<FoTermId, String> {
        match term.index().checked_sub(self.base) {
            None => Ok(term),
            Some(at) => self
                .terms
                .get(at)
                .copied()
                .ok_or_else(|| "a term past the proof's table".to_owned()),
        }
    }

    /// Returns whether `member` stands past the forest, for an instance.
    #[inline]
    fn is_instance(&self, member: crate::linear::occurrences::Member) -> bool {
        member.get() as usize >= self.occurrences
    }

    /// Returns the formula of `member`, whose occurrence's formula is
    /// `formula`: that formula for an occurrence of the forest, else its
    /// instance by the member's frame, built in `arena`.
    #[cold]
    #[inline(never)]
    fn node(
        &mut self,
        arena: &mut Formulas,
        member: crate::linear::occurrences::Member,
        formula: NodeId,
        calculus: &'static str,
    ) -> Result<NodeId, Error> {
        if (member.get() as usize) < self.occurrences {
            return Ok(formula);
        }
        if let Some(&node) = self.built.get(&member) {
            return Ok(node);
        }
        let fail = |reason: String| Error::ReadBack { calculus, reason };
        let table = self
            .table
            .ok_or_else(|| fail("a member past the forest without a table".to_owned()))?;
        let (_, frame) = table
            .instance(member)
            .ok_or_else(|| fail("a member the table does not hold".to_owned()))?;
        // The frame's terms, the innermost binder's first, all put in at
        // once: term by term, a frame of `n` terms over an atom of `n`
        // arguments built `n` atoms of `n` arguments for each member.
        let terms = table
            .frame_terms(frame)
            .into_iter()
            .map(|term| self.term(term))
            .collect::<Result<Vec<FoTermId>, String>>()
            .map_err(fail)?;
        let node = arena.instantiate_frame(formula, &terms)?;
        self.built.insert(member, node);
        Ok(node)
    }

    /// Returns the id in the arena of the term the instance `body` was
    /// opened with: the innermost term of its frame.
    fn witness(&self, body: crate::linear::occurrences::Member) -> Result<FoTermId, String> {
        let (_, frame) = self
            .table
            .and_then(|table| table.instance(body))
            .ok_or_else(|| "a quantifier rule's instance outside the table".to_owned())?;
        let table = self.table.ok_or_else(String::new)?;
        self.term(table.frame_parts(frame).1)
    }
}

/// Returns the member a one-premise rule's premise holds beyond its
/// conclusion without the principal formula at `at`, both sequents in
/// ascending order: the instance a quantifier rule opened. `None` unless
/// there is exactly one such member and the premise holds the rest.
fn opened(
    premise: &[crate::linear::occurrences::Member],
    conclusion: &[crate::linear::occurrences::Member],
    at: usize,
) -> Option<crate::linear::occurrences::Member> {
    let mut rest = conclusion
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != at)
        .map(|(_, &m)| m)
        .peekable();
    let mut new = None;
    for &m in premise {
        match rest.peek() {
            Some(&r) if r == m => {
                rest.next();
            }
            Some(&r) if r < m => return None,
            _ => {
                if new.replace(m).is_some() {
                    return None;
                }
            }
        }
    }
    if rest.next().is_some() {
        return None;
    }
    new
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::super::{Sequent, translate};
    use super::*;
    use crate::linear::{Options, Verdict, prove};

    /// The text tree draws `Γ ⊢ Δ` with the rules of LJ, and its size is
    /// that of the text.
    #[test]
    fn text_tree() {
        let sequent: Sequent = "a -> b, b -> c |- a -> c".parse().unwrap();
        let image = translate(
            &sequent,
            Logic::Intuitionistic,
            Translation::CallByName,
            &crate::Limits::default(),
        )
        .unwrap();
        let outcome = prove(image.sequent(), image.mode(), &Options::default()).unwrap();
        let Verdict::Proved(proof) = &outcome.verdict else {
            panic!("provable");
        };
        let derivation = image
            .read_back(proof, &crate::Limits::default(), |_| false)
            .unwrap();
        let tree = derivation.to_string();
        assert_eq!(
            tree,
            "───── ax   ───── ax\n\
             a ⊢ a      b ⊢ b\n\
             ──────────────── →L   ───── ax\n\
            \x20 a → b, a ⊢ b        c ⊢ c\n\
            \x20 ───────────────────────── →L\n\
            \x20    a → b, b → c, a ⊢ c\n\
            \x20    ──────────────────── →R\n\
            \x20    a → b, b → c ⊢ a → c"
        );
        let width = tree.lines().map(|l| l.chars().count()).max();
        let size = derivation.text_size(&crate::linear::proofs::TextOptions::default());
        assert_eq!(Some(size), width.map(|w| (w, tree.lines().count())));
        let root = derivation.inferences().last().unwrap();
        assert_eq!(root.rule(), Rule::ImpliesRight);
        assert_eq!((root.left().len(), root.right().len()), (2, 1));
        assert_eq!(root.principal(), Some((Side::Right, 0)));
        assert_eq!(root.premises(), [InfId::new(4)]);
        let mut steps = String::new();
        derivation.write_steps(&mut steps, |_| false).unwrap();
        assert_eq!(steps.lines().count(), 6);
        assert_eq!(
            steps.lines().last(),
            Some("6. a → b, b → c ⊢ a → c, by →R from 5.")
        );
    }

    /// Ex falso through an empty right side, `a, ¬a ⊢` weakened to
    /// `a, ¬a ⊢ b`, is refused in minimal logic and accepted in
    /// intuitionistic logic.
    #[test]
    fn minimal_logic_refuses_an_empty_right_side() {
        let sequent: Sequent = "a, ~a |- b".parse().unwrap();
        let f = sequent.formulas();
        let (a, not_a, b) = (sequent.left()[0], sequent.left()[1], sequent.right()[0]);
        let inference = |left: Vec<NodeId>, right, rule, principal, premises: &[u32]| Inference {
            term: None,
            left,
            right,
            rule,
            principal,
            premises: premises.iter().map(|&p| InfId::new(p)).collect(),
        };
        let forged = |logic| Derivation {
            logic,
            formulas: f.clone(),
            left: vec![a, not_a],
            right: vec![b],
            trusted: Trusted::new(true),
            inferences: vec![
                inference(vec![a], vec![a], Rule::Axiom, None, &[]),
                inference(
                    vec![a, not_a],
                    vec![],
                    Rule::NotLeft,
                    Some((Side::Left, 1)),
                    &[0],
                ),
                inference(
                    vec![a, not_a],
                    vec![b],
                    Rule::WeakenRight,
                    Some((Side::Right, 0)),
                    &[1],
                ),
            ],
        };
        let unbounded = crate::Limits::default();
        assert!(
            forged(Logic::Intuitionistic)
                .check(&unbounded, |_| false)
                .is_ok()
        );
        assert!(matches!(
            forged(Logic::Minimal).check(&unbounded, |_| false),
            Err(Error::ReadBack { .. })
        ));
    }

    /// Both labels of every rule read as its name in plain text, the
    /// subscript one with its side lowered, as the linear rules' upright
    /// ones do (`names_round_trip` in `proofs::style`), and the table is
    /// in the order of `Rule::ALL`.
    #[test]
    fn labels_are_the_names() {
        use crate::linear::proofs::style::plain;
        for (i, rule) in Rule::ALL.into_iter().enumerate() {
            assert_eq!(rule as usize, i, "{rule}");
            assert_eq!(plain(rule.markup(false)), rule.name(), "{rule}");
            assert_eq!(plain(rule.markup(true)), rule.name(), "{rule}");
        }
    }

    /// Every rule a step takes reads back from its name and from its
    /// ASCII spelling, and a cut, an open goal or an unknown name is
    /// refused.
    #[test]
    fn rules_read_from_their_names() {
        for rule in &super::TAKEN {
            assert_eq!(rule.name().parse::<Rule>().unwrap(), *rule);
        }
        for (name, rule) in super::ASCII {
            assert_eq!(name.parse::<Rule>().unwrap(), rule, "{name}");
        }
        assert_eq!("∨R2".parse::<Rule>().unwrap(), Rule::OrRight2);
        for refused in ["cut", "open", "⊗", "and", "-oL"] {
            assert!(matches!(
                refused.parse::<Rule>(),
                Err(Error::UnknownName { .. })
            ));
        }
    }

    /// A proof of a goal off the image's roots is refused before it is
    /// read back: its derivation would conclude another sequent.
    #[test]
    fn a_goal_proof_reads_back_as_nothing() {
        let sequent: Sequent = "a, b |- a".parse().unwrap();
        let image = translate(
            &sequent,
            Logic::Intuitionistic,
            Translation::CallByName,
            &crate::Limits::default(),
        )
        .unwrap();
        let forest = crate::linear::Forest::new(image.sequent()).unwrap();
        let roots = forest.roots();
        let goal = [
            crate::linear::Member::from(roots[0]),
            crate::linear::Member::from(roots[2]),
        ];
        let limits = crate::Limits::default();
        let outcome = crate::linear::search::prove_goal(
            crate::linear::search::Goal::new(&forest, &goal).unwrap(),
            image.mode(),
            &Options::default(),
            &limits,
            |_| false,
        )
        .unwrap();
        let Verdict::Proved(proof) = &outcome.verdict else {
            panic!("a ⊢ a is provable");
        };
        assert_eq!(
            image.read_back(proof, &limits, |_| false).unwrap_err(),
            Error::GoalProof
        );
    }

    /// The read-back holds the inferences it adds to the memory bound: a
    /// classical negation has no image, so a deep chain of them is a tiny
    /// linear proof read back as hundreds of `¬` inferences.
    #[test]
    fn the_read_back_keeps_to_the_memory_bound() {
        let chain = format!("{}a", "~".repeat(400));
        let sequent: Sequent = format!("{chain} |- {chain}").parse().unwrap();
        let image = translate(
            &sequent,
            Logic::Classical,
            Translation::Affine,
            &crate::Limits::default(),
        )
        .unwrap();
        let outcome = prove(image.sequent(), image.mode(), &Options::default()).unwrap();
        let Verdict::Proved(proof) = &outcome.verdict else {
            panic!("A ⊢ A is provable");
        };
        let roomy = crate::Limits::default();
        let derivation = image.read_back(proof, &roomy, |_| false).unwrap();
        assert!(derivation.inferences().len() > 800);
        let tight = roomy.with_memory_bytes(Some(32 * 1024));
        // The linear proof unfolds within the tight bound; its read-back
        // does not.
        assert!(image.linear_derivation(proof, &tight, |_| false).is_ok());
        assert!(matches!(
            image.read_back(proof, &tight, |_| false),
            Err(Error::Refused(Refusal::Memory { .. }))
        ));
    }

    /// The formulas the instances add to the derivation's arena are charged
    /// beside the inferences: under a bound that holds the inferences, the
    /// linear derivation and an eighth of that growth, a proof through a
    /// nest of binders does not read back.
    #[cfg(feature = "parse")]
    #[test]
    fn the_read_back_charges_the_instances_it_builds() {
        let k = 20;
        let vars: Vec<String> = (0..k).map(|i| format!("x{i}")).collect();
        let body: Vec<String> = vars.iter().map(|x| format!("q({x})")).collect();
        let text = format!(
            "forall {}. ({}) |- q(a)",
            vars.join(" "),
            body.join(" /\\ ")
        );
        let sequent: Sequent = text.parse().unwrap();
        let image = translate(
            &sequent,
            Logic::Intuitionistic,
            Translation::CallByName,
            &crate::Limits::default(),
        )
        .unwrap();
        let outcome = prove(image.sequent(), image.mode(), &Options::default()).unwrap();
        let Verdict::Proved(proof) = &outcome.verdict else {
            panic!("provable");
        };
        let unbounded = crate::Limits::default();
        let derivation = image.read_back(proof, &unbounded, |_| false).unwrap();
        let inferences: u64 = derivation
            .inferences()
            .iter()
            .map(|i| {
                (size_of::<Inference>() + (i.left.len() + i.right.len() + i.premises.len()) * 4)
                    as u64
            })
            .sum();
        let linear = image
            .linear_derivation(proof, &unbounded, |_| false)
            .unwrap();
        let unfolded: u64 = linear
            .inferences()
            .iter()
            .map(|i| (size_of_val(i) + size_of_val(i.sequent.as_slice())) as u64)
            .sum();
        let arena = |f: &Formulas| ArenaSize::of(f, &ArenaSize::default()).bytes;
        let growth = arena(derivation.formulas()) - arena(image.ordinary.formulas());
        assert!(growth > inferences, "{growth} bytes of instances");
        // The least bound the linear derivation's view unfolds within: the
        // read-back runs it under the same bound, on its own account.
        let (mut low, mut high) = (0u64, 1 << 30);
        while low + 1 < high {
            let mid = (low + high) / 2;
            let bound = unbounded.with_memory_bytes(Some(mid));
            if image.linear_derivation(proof, &bound, |_| false).is_ok() {
                high = mid;
            } else {
                low = mid;
            }
        }
        let held = inferences + unfolded;
        let tight = unbounded.with_memory_bytes(Some(held.max(high) + growth / 8));
        assert!(matches!(
            image.read_back(proof, &tight, |_| false),
            Err(Error::Refused(Refusal::Memory { .. }))
        ));
        let roomy = unbounded.with_memory_bytes(Some(inferences + unfolded + 2 * growth));
        assert!(image.read_back(proof, &roomy, |_| false).is_ok());
    }

    /// A cut checks in LJ, sharing the context or splitting it, reads back
    /// as a term and is written at the wire level that added it; LK, a
    /// principal formula and a second premise without the cut formula are
    /// refused.
    #[test]
    fn the_checker_takes_a_cut_in_lj() {
        let sequent: Sequent = "a, b |- a".parse().unwrap();
        let (a, b) = (sequent.left()[0], sequent.left()[1]);
        let make = |left: Vec<NodeId>, right, rule, premises: &[u32]| Inference {
            term: None,
            left,
            right,
            rule,
            principal: None,
            premises: premises.iter().map(|&p| InfId::new(p)).collect(),
        };
        // `a ⊢ a` and `a, b ⊢ a` (the latter `b` weakened onto `a ⊢ a`)
        // cut on `a` into `a, b ⊢ a`.
        let mut weakened = make(vec![a, b], vec![a], Rule::WeakenLeft, &[1]);
        weakened.principal = Some((Side::Left, 1));
        let cut = |logic, second: Inference| Derivation {
            logic,
            formulas: sequent.formulas().clone(),
            left: vec![a, b],
            right: vec![a],
            trusted: Trusted::new(true),
            inferences: vec![
                make(vec![a], vec![a], Rule::Axiom, &[]),
                make(vec![a], vec![a], Rule::Axiom, &[]),
                second,
                make(vec![a, b], vec![a], Rule::Cut, &[0, 2]),
            ],
        };
        let unbounded = crate::Limits::default();
        let split = cut(Logic::Minimal, weakened.clone());
        assert_eq!(split.check(&unbounded, |_| false), Ok(()));
        assert!(split.term(&unbounded, |_| false).is_ok());
        #[cfg(feature = "serialize")]
        {
            let json = serde_json::to_value(&split).unwrap();
            assert_eq!(json["version"], crate::wire::ORDINARY_CUT);
        }
        let reason = |d: &Derivation| match d.check(&unbounded, |_| false) {
            Err(Error::ReadBack { reason, .. }) => reason,
            other => panic!("{other:?}"),
        };
        assert!(
            reason(&cut(Logic::Classical, weakened.clone()))
                .contains("LK's derivations have no cut")
        );
        let mut principal = cut(Logic::Intuitionistic, weakened);
        principal.inferences[3].principal = Some((Side::Left, 0));
        assert!(reason(&principal).contains("a cut has no principal formula"));
        // A cut on `b` whose second premise, `a ⊢ a`, has no `b`.
        let mut other = principal;
        other.inferences[3].principal = None;
        other.inferences[0] = make(vec![b], vec![b], Rule::Axiom, &[]);
        other.inferences[3].premises = vec![InfId::new(0), InfId::new(1)];
        assert!(reason(&other).contains("do not share or split"));
    }

    /// Two lists are one multiset whatever their order, short or long,
    /// and two with a member counted differently are not.
    #[test]
    fn same_compares_multisets() {
        let ids = |list: &[u32]| list.iter().map(|&i| NodeId(i)).collect::<Vec<_>>();
        let long: Vec<u32> = (0..40).map(|i| i % 7).collect();
        let reversed: Vec<u32> = long.iter().rev().copied().collect();
        let mut other = long.clone();
        other[0] = 6;
        for (a, b, equal) in [
            (&[1, 2, 2][..], &[2, 1, 2][..], true),
            (&[1, 2, 2], &[1, 1, 2], false),
            (&[1, 2], &[1, 2, 2], false),
            (&long, &reversed, true),
            (&long, &other, false),
        ] {
            assert_eq!(same(&ids(a), &ids(b)), equal, "{a:?} {b:?}");
        }
    }

    /// Every guard of the checker refuses what it guards against: a
    /// derivation read back checks, and each of these breaks of one of its
    /// inferences fails, with the bound and the stop refusing besides.
    #[test]
    fn the_checker_refuses_each_break() {
        let sequent: Sequent = "a -> b, b -> c |- a -> c".parse().unwrap();
        let image = translate(
            &sequent,
            Logic::Intuitionistic,
            Translation::CallByName,
            &crate::Limits::default(),
        )
        .unwrap();
        let outcome = prove(image.sequent(), image.mode(), &Options::default()).unwrap();
        let Verdict::Proved(proof) = &outcome.verdict else {
            panic!("provable");
        };
        let unbounded = crate::Limits::default();
        let derivation = image.read_back(proof, &unbounded, |_| false).unwrap();
        assert_eq!(derivation.check(&unbounded, |_| false), Ok(()));
        let rules: Vec<Rule> = derivation.inferences.iter().map(|i| i.rule).collect();
        let at = |rule| rules.iter().position(|&r| r == rule).unwrap();
        let (axiom, left, right) = (
            at(Rule::Axiom),
            at(Rule::ImpliesLeft),
            at(Rule::ImpliesRight),
        );
        let root = rules.len() - 1;
        // Each break with the reason its guard gives: with the guard
        // gone, another would fail it for another reason, or none would.
        type Break = (&'static str, Box<dyn Fn(&mut Derivation)>);
        let broken: [Break; 12] = [
            (
                "an open goal",
                Box::new(move |d| d.inferences[axiom].rule = Rule::Open),
            ),
            ("the root does not conclude", Box::new(|d| d.right.clear())),
            (
                "not a sequent A ⊢ A",
                Box::new(move |d| d.inferences[axiom].principal = Some((Side::Left, 0))),
            ),
            (
                "a premise does not precede",
                Box::new(move |d| {
                    d.inferences[left].premises[0] = InfId::new(root as u32);
                }),
            ),
            (
                "more than one formula right",
                Box::new(move |d| {
                    let extra = d.inferences[axiom].left[0];
                    d.inferences[axiom].right.push(extra);
                }),
            ),
            (
                "not a sequent A ⊢ A",
                Box::new(move |d| {
                    let other = d.inferences[right].right[0];
                    d.inferences[axiom].right[0] = other;
                }),
            ),
            (
                "no principal formula",
                Box::new(move |d| d.inferences[left].principal = None),
            ),
            (
                "lies outside the sequent",
                Box::new(move |d| {
                    d.inferences[left].principal = Some((Side::Left, 9));
                }),
            ),
            (
                "premises instead of 2",
                Box::new(move |d| {
                    d.inferences[left].premises.pop();
                }),
            ),
            (
                "do not share or split",
                Box::new(move |d| d.inferences[left].premises.reverse()),
            ),
            (
                "does not apply to the principal",
                Box::new(move |d| {
                    d.inferences[right].rule = Rule::AndRight;
                }),
            ),
            (
                "not what the rule makes",
                Box::new(move |d| {
                    d.inferences[right].premises[0] = InfId::new(axiom as u32);
                }),
            ),
        ];
        for (why, mutate) in &broken {
            let mut d = derivation.clone();
            mutate(&mut d);
            match d.check(&unbounded, |_| false) {
                Err(Error::ReadBack { reason, .. }) => {
                    assert!(reason.contains(why), "{why}: {reason}")
                }
                other => panic!("{why}: {other:?}"),
            }
        }
        // A rule whose premises share the context, the second one wrong
        // (`∧R` of LJ under cbn), a `⊥L` in minimal logic, and a `⊥R`
        // read back from a linear `⊥`.
        let read = |input: &str, logic, translation| {
            let sequent: Sequent = input.parse().unwrap();
            let image = translate(&sequent, logic, translation, &crate::Limits::default()).unwrap();
            let outcome = prove(image.sequent(), image.mode(), &Options::default()).unwrap();
            let Verdict::Proved(proof) = &outcome.verdict else {
                panic!("{input} is provable");
            };
            let derivation = image.read_back(proof, &unbounded, |_| false).unwrap();
            assert_eq!(derivation.check(&unbounded, |_| false), Ok(()), "{input}");
            derivation
        };
        let reason = |d: &Derivation| match d.check(&unbounded, |_| false) {
            Err(Error::ReadBack { reason, .. }) => reason,
            other => panic!("{other:?}"),
        };
        let cbn = Translation::CallByName;
        let mut shared = read("a, b |- a /\\ b", Logic::Intuitionistic, cbn);
        let and = shared
            .inferences
            .iter()
            .position(|i| i.rule == Rule::AndRight)
            .unwrap();
        let first = shared.inferences[and].premises[0];
        shared.inferences[and].premises[1] = first;
        assert!(reason(&shared).contains("do not share or split"));
        let mut minimal = read("a -> b, b -> c |- a -> c", Logic::Minimal, cbn);
        let leaf = minimal
            .inferences
            .iter()
            .position(|i| i.rule == Rule::Axiom)
            .unwrap();
        minimal.inferences[leaf].rule = Rule::FalseLeft;
        minimal.inferences[leaf].principal = Some((Side::Left, 0));
        assert!(reason(&minimal).contains("minimal logic has no ex falso"));
        let falsity = read("a |- a, false", Logic::Classical, Translation::Affine);
        assert!(
            falsity
                .inferences
                .iter()
                .any(|i| i.rule == Rule::FalseRight)
        );
        // A split (LJ's →L) whose conclusion holds a hypothesis neither
        // premise has, the right sides agreeing.
        let mut split = derivation.clone();
        let extra = split.inferences[axiom].left[0];
        split.inferences[left].left.push(extra);
        let at_split = format!("inference {left} (→L): the premises do not share or split");
        assert!(reason(&split).starts_with(&at_split), "{}", reason(&split));
        // A contraction whose premise holds its formula not at all, or
        // once (through a weakening), on either side: `a, b ⊢ a` and
        // `a ⊢ a, b` from `a ⊢ a`.
        let base = read("a -> b, a |- b", Logic::Classical, Translation::Affine);
        let atom = |i| base.formulas.ids[&Node::Atom(i)];
        let (a, b) = (atom(0), atom(1));
        let sides = [
            (Rule::ContractLeft, Rule::WeakenLeft, Side::Left),
            (Rule::ContractRight, Rule::WeakenRight, Side::Right),
        ];
        for (contract, weaken, side) in sides {
            let with_b = match side {
                Side::Left => (vec![a, b], vec![a]),
                Side::Right => (vec![a], vec![a, b]),
            };
            let make = |rule, premises: Vec<InfId>| Inference {
                term: None,
                left: with_b.0.clone(),
                right: with_b.1.clone(),
                rule,
                principal: Some((side, 1)),
                premises,
            };
            let axiom = Inference {
                term: None,
                left: vec![a],
                right: vec![a],
                rule: Rule::Axiom,
                principal: None,
                premises: Vec::new(),
            };
            let once = make(weaken, vec![InfId::new(0)]);
            for inferences in [
                vec![axiom.clone(), make(contract, vec![InfId::new(0)])],
                vec![axiom, once, make(contract, vec![InfId::new(1)])],
            ] {
                let mut contracted = base.clone();
                let root = inferences.len() - 1;
                contracted.inferences = inferences;
                (contracted.left, contracted.right) = with_b.clone();
                let expected = format!("inference {root} ({contract}): the premise is not what");
                assert!(reason(&contracted).starts_with(&expected), "{contract}");
            }
        }
        let one = unbounded.with_work(Some(1));
        assert_eq!(
            derivation.check(&one, |_| false),
            Err(Error::Refused(Refusal::Work { limit: 1 }))
        );
        let enough = unbounded.with_work(Some(derivation.inferences.len() as u64));
        assert_eq!(derivation.check(&enough, |_| false), Ok(()));
        assert_eq!(
            derivation.check(&unbounded, |_| true),
            Err(Error::Refused(Refusal::Stopped {
                phase: Phase::Check
            }))
        );
        assert_eq!(
            image
                .read_back(proof, &unbounded, |p| p.phase == Phase::ReadBack)
                .unwrap_err(),
            Error::Refused(Refusal::Stopped {
                phase: Phase::ReadBack
            })
        );
    }

    /// A derivation read from JSON that breaks a rule has no term: its
    /// check's error, never a panic; once checked, its term is read.
    #[cfg(feature = "serialize")]
    #[test]
    fn a_derivation_read_from_json_is_checked_before_its_term() {
        let sequent: Sequent = "|- a -> a".parse().unwrap();
        let options = super::super::Options::default().with_logic(Logic::Intuitionistic);
        let outcome = super::super::decide(&sequent, &options).unwrap();
        let super::super::Verdict::Valid(evidence) = outcome.verdict else {
            panic!("valid");
        };
        let json = serde_json::to_string(evidence.derivation().unwrap()).unwrap();
        let limits = crate::Limits::default();
        let read: Derivation = serde_json::from_str(&json).unwrap();
        assert!(read.term(&limits, |_| false).is_ok());
        // `∧R` in place of `→R`: two premises where the inference has one.
        assert!(json.contains("\"→R\""), "{json}");
        let broken: Derivation = serde_json::from_str(&json.replace("\"→R\"", "\"∧R\"")).unwrap();
        assert!(matches!(
            broken.term(&limits, |_| false),
            Err(Error::ReadBack { .. })
        ));
        // An open goal has no term, checked or not.
        assert!(json.contains("\"ax\""), "{json}");
        let open: Derivation = serde_json::from_str(&json.replace("\"ax\"", "\"open\"")).unwrap();
        assert!(matches!(
            open.term(&limits, |_| false),
            Err(Error::NoTerm { .. })
        ));
    }

    /// Returns `∃x. ∀y. r(x, y) ⊢ ∀y. ∃x. r(x, y)` derived in LJ with all
    /// four quantifier rules, `∀R`'s eigenvariable numbered 0 and `∃L`'s
    /// `inner`, and the terms of both.
    fn swapped(inner: u32) -> (Derivation, [FoTermId; 2]) {
        use crate::first_order::{Eigen, FoTerm};
        let mut f = Formulas::default();
        let (x, y) = (f.variable(1), f.variable(0));
        let r = f.predicate("r", &[x, y]).unwrap();
        let all_y = f.binder(true, r, "y").unwrap();
        let left = f.binder(false, all_y, "x").unwrap();
        let (x, y) = (f.variable(0), f.variable(1));
        let r = f.predicate("r", &[x, y]).unwrap();
        let exists_x = f.binder(false, r, "x").unwrap();
        let right = f.binder(true, exists_x, "y").unwrap();
        let e0 = f.interner().term(FoTerm::Eigen(Eigen::new(0)));
        let e1 = f.interner().term(FoTerm::Eigen(Eigen::new(inner)));
        // `∀y. r(e1, y)`, `r(e1, e0)` and `∃x. r(x, e0)`.
        let all = f.instantiate(all_y, e1).unwrap();
        let Node::Forall(body) = f.node(all) else {
            unreachable!()
        };
        let ground = f.instantiate(body, e0).unwrap();
        let some = f.instantiate(exists_x, e0).unwrap();
        let make = |left, right, rule, principal, premises: &[u32], term| Inference {
            left,
            right,
            rule,
            principal,
            premises: premises.iter().map(|&p| InfId::new(p)).collect(),
            term,
        };
        let inferences = vec![
            make(vec![ground], vec![ground], Rule::Axiom, None, &[], None),
            make(
                vec![ground],
                vec![some],
                Rule::ExistsRight,
                Some((Side::Right, 0)),
                &[0],
                Some(e1),
            ),
            make(
                vec![all],
                vec![some],
                Rule::ForallLeft,
                Some((Side::Left, 0)),
                &[1],
                Some(e0),
            ),
            make(
                vec![left],
                vec![some],
                Rule::ExistsLeft,
                Some((Side::Left, 0)),
                &[2],
                Some(e1),
            ),
            make(
                vec![left],
                vec![right],
                Rule::ForallRight,
                Some((Side::Right, 0)),
                &[3],
                Some(e0),
            ),
        ];
        let derivation = Derivation::from_parts(
            Logic::Intuitionistic,
            f,
            (vec![left], vec![right]),
            inferences,
        );
        (derivation, [e0, e1])
    }

    /// The quantifier rules check, each with the term it records, and each
    /// break of one is refused for its own reason.
    #[test]
    fn the_quantifier_rules_check() {
        let limits = Limits::default();
        let reason = |d: &Derivation| match d.check(&limits, |_| false) {
            Err(Error::ReadBack { reason, .. }) => reason,
            other => panic!("expected a refusal, got {other:?}"),
        };
        let (derivation, [e0, _]) = swapped(1);
        derivation.check(&limits, |_| false).unwrap();
        assert_eq!(
            derivation
                .formulas()
                .formula(derivation.inferences()[2].left()[0])
                .to_string(),
            "∀y. r(b, y)"
        );
        // `∃L` on the eigenvariable `∀R` takes, which its conclusion holds.
        let (bad, _) = swapped(0);
        assert!(reason(&bad).contains("occurs in the conclusion"));
        let broken = |n: usize, change: &dyn Fn(&mut Inference)| {
            let mut d = swapped(1).0;
            change(&mut d.inferences[n]);
            reason(&d)
        };
        let constant = {
            let mut d = swapped(1).0;
            let a = d.formulas.function("a", &[]).unwrap();
            d.inferences[4].term = Some(a);
            reason(&d)
        };
        assert!(constant.contains("no eigenvariable"));
        assert!(broken(2, &|i| i.term = Some(FoTermId::new(0))).contains("not what the rule"));
        assert!(broken(2, &|i| i.term = None).contains("without its term"));
        assert!(broken(0, &|i| i.term = Some(e0)).contains("takes none"));
        let open = {
            let mut d = swapped(1).0;
            let meta = d
                .formulas
                .interner()
                .term(crate::first_order::FoTerm::Meta(0));
            d.inferences[2].term = Some(meta);
            reason(&d)
        };
        assert!(open.contains("an open witness"));
        #[cfg(feature = "serialize")]
        {
            let json = serde_json::to_string(&derivation).unwrap();
            assert!(
                json.contains(r#"{"E":0}"#) && json.contains(r#""term":"#),
                "{json}"
            );
            let read: Derivation = serde_json::from_str(&json).unwrap();
            read.check(&limits, |_| false).unwrap();
            assert_eq!(serde_json::to_string(&read).unwrap(), json);
        }
    }

    /// First-order sequents read back to derivations the checker takes,
    /// under every translation of every logic it decides: each quantifier
    /// rule with its term, the instances built in the derivation's arena.
    #[test]
    fn first_order_proofs_read_back() {
        let limits = crate::Limits::default();
        let valid = [
            "forall x. p(x) |- p(a)",
            "p(a) |- exists x. p(x)",
            "forall x. (p(x) /\\ q(x)) |- forall x. p(x)",
            "exists x. p(x) |- exists y. (p(y) \\/ q(y))",
            "forall x. (p(x) -> q(x)), p(a) |- q(a)",
            "exists x. forall y. r(x, y) |- forall y. exists x. r(x, y)",
            "~(exists x. p(x)) |- forall x. ~p(x)",
            "forall x. p(f(x)) |- p(f(f(a)))",
            "forall x. (p(x) <-> q(x)), q(a) |- p(a)",
            "forall x. p(x) |- p(a) /\\ p(b)",
        ];
        let mut read = 0;
        for logic in Logic::ALL.iter().copied() {
            for translation in Translation::ALL.iter().copied() {
                if !translation.decides(logic) {
                    continue;
                }
                for text in valid {
                    let sequent: Sequent = text.parse().unwrap();
                    let image = translate(&sequent, logic, translation, &limits).unwrap();
                    let options = Options::default().with_copies(None);
                    let outcome = prove(image.sequent(), image.mode(), &options).unwrap();
                    let Verdict::Proved(proof) = &outcome.verdict else {
                        // The affine image has no contraction: only the
                        // sequents that use a hypothesis once are proved.
                        assert_eq!(translation, Translation::Affine, "{text}");
                        continue;
                    };
                    let derivation = image.read_back(proof, &limits, |_| false).unwrap();
                    derivation
                        .check(&limits, |_| false)
                        .unwrap_or_else(|e| panic!("{text} {logic:?} {translation:?}: {e}"));
                    assert!(
                        derivation.inferences().iter().any(|i| i.term().is_some()),
                        "{text}"
                    );
                    read += 1;
                }
            }
        }
        assert!(read >= 40, "{read}");
    }

    /// The admissible view keeps a weakening that brings `∀R`'s
    /// eigenvariable into the conclusion.
    #[test]
    fn the_admissible_view_keeps_an_eigenvariable_out_of_its_conclusion() {
        let (mut derivation, _) = swapped(1);
        // `∃x. r(x, e0)`, weakened in below `∀R` on `e0`.
        let weak = derivation.inferences[1].right[0];
        let root = derivation.inferences.len() as u32 - 1;
        let mut below = derivation.inferences[4].clone();
        below.left.push(weak);
        below.rule = Rule::WeakenLeft;
        below.principal = Some((Side::Left, 1));
        below.premises = vec![InfId::new(root)];
        below.term = None;
        derivation.left.clone_from(&below.left);
        derivation.inferences.push(below);
        derivation.check(&Limits::default(), |_| false).unwrap();
        let shown = derivation.shown(Structural::Admissible);
        assert_eq!(shown.inferences().len(), derivation.inferences().len());
    }

    /// A derivation document whose witness unfolds past `limits.terms`
    /// is refused before any key is written, and a propositional one that
    /// names a quantifier rule is refused by the methods that read rules.
    #[cfg(all(feature = "serialize", feature = "interactive"))]
    #[test]
    fn documents_with_quantifier_rules_stay_within_their_bounds() {
        // `t(n+1) = f(t(n), t(n))`, sixty times: a few hundred bytes for a
        // term of 2⁶⁰ symbols.
        let mut fo = vec![r#"{"F":[1,[]]}"#.to_owned()];
        for n in 0..60 {
            fo.push(format!(r#"{{"F":[2,[{n},{n}]]}}"#));
        }
        let json = format!(
            r#"{{"version":25,"logic":"classical","sequent":{{"formulas":[{{"V":0}},{{"∀":0}}],"atoms":[{{"P":[0,[61]]}}],"symbols":[["p",1],["c",0],["f",2]],"fo":[{},{{"B":0}}],"binders":["x"],"left":[1],"right":[0]}},"inferences":[{{"left":[1],"right":[0],"rule":"∀L","principal":["left",0],"term":60}}]}}"#,
            fo.join(",")
        );
        let read = crate::wire::upgrade::<Derivation, _>(
            &mut serde_json::Deserializer::from_str(&json),
            &Limits::default(),
        );
        assert!(
            matches!(read, Err(Error::Refused(Refusal::Terms { .. }))),
            "{read:?}"
        );
        let quantified = r#"{"version":8,"logic":"classical","sequent":{"formulas":[{"V":0}],"atoms":["p"],"left":[0],"right":[0]},"inferences":[{"left":[0],"right":[0],"rule":"∀L","principal":["left",0]}]}"#;
        let derivation: Derivation = serde_json::from_str(quantified).unwrap();
        assert!(matches!(
            crate::ordinary::Tableau::of(&derivation, |_| false),
            Err(Error::FirstOrder { .. })
        ));
        assert!(matches!(
            crate::rules::Derivation::from_ordinary(&derivation),
            Err(Error::FirstOrder { .. })
        ));
    }
}
