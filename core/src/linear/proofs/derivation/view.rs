// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The public side of the derivation view: its options, its inferences
//! and what a renderer asks of a derivation.

use super::build::unfold;
use crate::Error;
use crate::limits::{Limits, Progress};
use crate::linear::fragment::Mode;
use crate::linear::occurrences::instances::Framing;
use crate::linear::occurrences::{Forest, Member, OccId, Reading};
use crate::linear::proofs::check;
use crate::linear::proofs::{Named, NodeId, Proof};

/// The index of an inference in a derivation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize), serde(transparent))]
pub struct InfId(u32);

impl InfId {
    /// Wraps a raw index.
    pub const fn new(index: u32) -> Self {
        Self(index)
    }

    /// Returns the raw index.
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Returns the index as a `usize`, for indexing the inferences.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// How a derivation is drawn: compact or not, one-sided or two-sided.
///
/// Every call that builds a [`Derivation`] takes these options, whatever
/// it then draws or writes. The default draws every rule as its own
/// inference, one-sided for classical linear logic and two-sided for a
/// proof found in intuitionistic mode.
///
/// The [`Limits`] passed beside the options keep a call from building
/// more than the machine can hold:
/// - `derivation_bytes` is the most bytes a derivation may be estimated
///   to take ([`Size::bytes`](crate::linear::Size::bytes)) and still be
///   built. A derivation can be larger than its proof by any factor: the
///   proof stores a shared subproof once and the derivation repeats it,
///   and every inference holds its whole sequent.
/// - `memory_bytes` is the most the building may hold at once. It bounds
///   each pass of the checker on the way, and the derivation itself by
///   the same estimate.
///
/// # Examples
///
/// A proof found in intuitionistic mode is drawn two-sided by default.
/// [`Sides::One`] draws it one-sided instead:
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::Limits;
/// use deduxit::linear::proofs::Sides;
/// use deduxit::linear::{IllSequent, Mode, Options, Verdict, ViewOptions, prove};
///
/// let sequent: IllSequent = "A, A -o B |- B".parse()?;
/// let outcome = prove(sequent.sequent(), Mode::INTUITIONISTIC, &Options::default())?;
/// let Verdict::Proved(proof) = &outcome.verdict else {
///     panic!("provable");
/// };
/// let view = ViewOptions::default().with_sides(Sides::One);
/// // The closure `|_| false` never stops the building.
/// let derivation = proof.derivation_within(&view, &Limits::default(), |_| false)?;
/// let lines = [
///     "─────── ax   ─────── ax",
///     "⊢ ~A, A      ⊢ ~B, B",
///     "──────────────────── ⊗",
///     "  ⊢ ~A, A ⊗ ~B, B",
/// ];
/// assert_eq!(derivation.to_string(), lines.join("\n"));
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// [`Compact::Always`] draws a run of one structural rule as one
/// inference, its label marked with a star:
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::Limits;
/// use deduxit::linear::proofs::Compact;
/// use deduxit::linear::{Mode, Options, Sequent, Verdict, ViewOptions, prove};
///
/// let sequent: Sequent = "|- 1, ?A, ?B".parse()?;
/// let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
/// let Verdict::Proved(proof) = &outcome.verdict else {
///     panic!("provable");
/// };
/// let full = [
///     "    ─── 1",
///     "    ⊢ 1",
///     "  ─────── ?w",
///     "  ⊢ 1, ?A",
///     "─────────── ?w",
///     "⊢ 1, ?A, ?B",
/// ];
/// assert_eq!(proof.derivation()?.to_string(), full.join("\n"));
///
/// let view = ViewOptions::default().with_compact(Compact::Always);
/// let derivation = proof.derivation_within(&view, &Limits::default(), |_| false)?;
/// let compact = ["    ─── 1", "    ⊢ 1", "─────────── ?w*", "⊢ 1, ?A, ?B"];
/// assert_eq!(derivation.to_string(), compact.join("\n"));
/// assert_eq!(derivation.inference(derivation.root()).times(), 2);
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// # JSON
///
/// With the feature `serialize` an object `{"compact": "auto", "sides":
/// "auto", "classical": "one", "structural": "admissible"}`, `compact` one
/// of `"auto"`, `"always"` and `"never"`, `sides` one of `"auto"`, `"one"`
/// and `"two"`, `classical` one of `"one"` and `"two"`, `structural` one
/// of `"explicit"` and `"admissible"`; a key left out keeps its default,
/// and an unknown one is refused, as for every options value.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct ViewOptions {
    /// Whether a run of one structural rule, such as several weakenings in
    /// a row, is drawn as one inference. [`Compact::Auto`] by default.
    pub compact: Compact,
    /// Whether the derivation is one-sided (`⊢ Γ`) or two-sided
    /// (`Γ ⊢ A`). [`Sides::Auto`] by default: two-sided for a proof found
    /// in intuitionistic mode.
    pub sides: Sides,
    /// Whether a front end draws the derivation of a classical sequent
    /// one-sided or with each formula on the side it was written on
    /// ([`TwoSided`](crate::linear::TwoSided)). One-sided by default
    /// ([`DEFAULT_CLASSICAL`](Self::DEFAULT_CLASSICAL)).
    ///
    /// The library's own derivations are one-sided whatever this says. A
    /// front end that holds the sequent as written
    /// ([`ClassicalSequent`](crate::linear::ClassicalSequent)) reads this
    /// switch and makes the two-sided view from the one-sided derivation.
    pub classical: ClassicalSides,
    /// Whether a derivation of LK or LJ shows its weakenings and
    /// contractions. Left out where admissible by default
    /// ([`DEFAULT_STRUCTURAL`](Self::DEFAULT_STRUCTURAL)). A linear
    /// derivation ignores it: [`compact`](Self::compact) decides how its
    /// structural rules are drawn.
    pub structural: Structural,
}

/// Whether a derivation of LK or LJ shows its structural rules.
///
/// [`Explicit`](Self::Explicit) draws every weakening and contraction as
/// an inference, as the calculus and its checker have them.
/// [`Admissible`](Self::Admissible) leaves each out where it is
/// admissible and shows its conclusion as that of the rule above it. The
/// derivation then reads as in the calculus whose axiom takes a context
/// and whose rules contract what they use twice. Only the drawing
/// differs: the derivation is the same.
///
/// # JSON
///
/// With the feature `serialize`, `"explicit"` or `"admissible"`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Structural {
    /// Every weakening and contraction drawn as an inference.
    Explicit,
    /// Every weakening or contraction left out where the rules above it
    /// close their branch.
    #[default]
    Admissible,
}

/// Whether a front end draws a derivation of classical linear logic
/// one-sided, as the calculus is, or two-sided, as its sequent was
/// written ([`TwoSided`](crate::linear::TwoSided)).
///
/// # JSON
///
/// With the feature `serialize`, `"one"` or `"two"`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum ClassicalSides {
    /// One-sided: every formula right of `⊢`.
    #[default]
    One,
    /// Two-sided: each formula on the side of `⊢` it was written on, each
    /// rule named by its side, and a negation step where a negation was
    /// written.
    Two,
}

/// Whether a derivation is drawn one-sided (`⊢ Γ`), as classical linear
/// logic writes it, or two-sided (`Γ ⊢ A`), as intuitionistic linear logic
/// does.
///
/// # JSON
///
/// With the feature `serialize`, `"auto"`, `"one"` or `"two"`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Sides {
    /// Two-sided exactly when the proof was found in intuitionistic mode
    /// ([`Proof::mode`]), one-sided otherwise.
    #[default]
    Auto,
    /// One-sided, every formula right of `⊢`.
    One,
    /// Two-sided, as intuitionistic linear logic reads the sequent, with
    /// the intuitionistic rule names (`⊸L`, `!L`, …). Building fails for
    /// a proof that the checker refuses in intuitionistic mode.
    Two,
}

/// Whether a derivation draws a run of one structural rule as one
/// inference.
///
/// A run is a chain of the same weakening or contraction, such as the
/// weakenings of every unused `?` formula. Drawn as one inference, its
/// label carries a star (`?w*`) and [`Inference::times`] says how many
/// rules it stands for. A compact derivation can be far smaller than the
/// full one.
///
/// # JSON
///
/// With the feature `serialize`, `"auto"`, `"always"` or `"never"`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Compact {
    /// Every rule as its own inference, unless the full derivation would
    /// pass a bound of the [`Limits`] and the compact one would not. With
    /// no bound in bytes set, it never compacts.
    #[default]
    Auto,
    /// Every run as one inference, even where the full derivation fits.
    Always,
    /// Every rule as its own inference.
    Never,
}

impl ViewOptions {
    /// The default of [`classical`](Self::classical): one-sided, as the
    /// calculus is.
    pub const DEFAULT_CLASSICAL: ClassicalSides = ClassicalSides::One;

    /// The default of [`structural`](Self::structural): structural rules
    /// left out where admissible. This is how a first course writes LK and
    /// LJ: the axiom takes a context, and both premises share the context.
    pub const DEFAULT_STRUCTURAL: Structural = Structural::Admissible;

    /// Returns the options with another [`structural`](Self::structural).
    #[must_use]
    pub const fn with_structural(self, structural: Structural) -> Self {
        Self { structural, ..self }
    }

    /// Returns the options with another [`classical`](Self::classical).
    #[must_use]
    pub const fn with_classical(self, classical: ClassicalSides) -> Self {
        Self { classical, ..self }
    }

    /// Returns the options with another [`compact`](Self::compact).
    #[must_use]
    pub const fn with_compact(self, compact: Compact) -> Self {
        Self { compact, ..self }
    }

    /// Returns the options with another [`sides`](Self::sides).
    #[must_use]
    pub const fn with_sides(self, sides: Sides) -> Self {
        Self { sides, ..self }
    }

    /// Returns whether a derivation of `proof` is two-sided.
    pub(crate) fn two_sided(&self, proof: &Proof) -> bool {
        match self.sides {
            Sides::One => false,
            Sides::Two => true,
            Sides::Auto => proof.mode().is_some_and(Mode::is_intuitionistic),
        }
    }
}

/// One inference of a derivation: the sequent it concludes, its rule, and
/// the inferences of its premises.
///
/// Two inferences are equal when they show the same step, even when they
/// unfold different nodes of a proof term ([`node`](Self::node)).
#[non_exhaustive]
#[derive(Clone, Debug, Eq)]
pub struct Inference {
    /// The sequent concluded, as members in ascending order, each repeated
    /// as often as the sequent holds it.
    pub(crate) sequent: Vec<Member>,
    /// The rule applied, as the derivation names it.
    pub(crate) rule: Named,
    /// The position in `sequent` of the formula the rule introduces or
    /// removes: `None` for an axiom, whose sequent is its two literals, and
    /// for Mix.
    pub(crate) principal: Option<usize>,
    /// The premises, in the rule's order.
    pub(crate) premises: Vec<InfId>,
    /// How many applications of the rule the inference stands for: one,
    /// or more for a run that a compact view draws as one inference
    /// ([`Compact`]). A run's principal formula is that of its lowest
    /// application.
    pub(crate) times: u32,
    /// For a cut, the member of the cut formula `A`. The first premise
    /// holds it, the second its dual.
    pub(crate) cut: Option<Member>,
    /// In a derivation of a proof term, the node whose rule the inference
    /// shows.
    pub(crate) node: Option<NodeId>,
}

impl PartialEq for Inference {
    /// Compares every field but the node. So the same step is one
    /// inference whether it comes from terms numbered differently or from
    /// a session, which names no node.
    fn eq(&self, other: &Self) -> bool {
        // The `..` below hides a new field from the compiler: add it to
        // the comparison by hand.
        let Self {
            sequent,
            rule,
            principal,
            premises,
            times,
            cut,
            ..
        } = self;
        *sequent == other.sequent
            && *rule == other.rule
            && *principal == other.principal
            && *premises == other.premises
            && *times == other.times
            && *cut == other.cut
    }
}

impl Inference {
    /// Returns the sequent concluded, as members in ascending order, each
    /// repeated as often as the sequent holds it.
    ///
    /// In the cyclic mode this is the sequent's cyclic order, read from its
    /// first occurrence. Every sequent of the derivation is then a rotation
    /// of the order its rule reads, so no exchange rule is needed between
    /// two of them.
    pub fn sequent(&self) -> &[Member] {
        &self.sequent
    }

    /// Returns the rule applied, as the derivation names it.
    pub const fn rule(&self) -> Named {
        self.rule
    }

    /// Returns the position in [`sequent`](Self::sequent) of the formula
    /// the rule introduces or removes: `None` for an axiom, whose sequent
    /// is its two literals, for Mix and for an open goal.
    pub const fn principal(&self) -> Option<usize> {
        self.principal
    }

    /// Returns the premises, in the rule's order.
    pub fn premises(&self) -> &[InfId] {
        &self.premises
    }

    /// Returns how many applications of the rule the inference stands for:
    /// one, or more for a run that a compact view draws as one inference
    /// ([`Compact`]). A run's principal formula is that of its lowest
    /// application.
    pub const fn times(&self) -> u32 {
        self.times
    }

    /// Returns, for a cut, the member of its cut formula `A`; `None` for
    /// every other rule. The first premise's sequent holds `A`, the
    /// second its dual ([`Forest::dual`](crate::linear::Forest::dual)).
    pub const fn cut(&self) -> Option<Member> {
        self.cut
    }

    /// Returns the node of the proof term whose rule the inference shows.
    ///
    /// This maps a node to the inferences drawn for it. A node the term
    /// shares unfolds once for every place that reads it, so several
    /// inferences can name one node. A compact run has its lowest
    /// application's node.
    ///
    /// `None` for an inference the derivation adds below a node (the
    /// weakening of a `?` formula the subproof does not use, the
    /// contraction of the `?` formulas both premises of a `⊗`, Mix or cut
    /// use), and for every inference of a session's derivation.
    pub const fn node(&self) -> Option<NodeId> {
        self.node
    }
}

/// A proof drawn as a tree of inferences in the standard sequent calculus.
///
/// A [`Proof`] is a compact term. Its derivation is the tree a textbook
/// shows, each inference with its whole sequent: one-sided (`⊢ Γ`) for
/// classical linear logic, two-sided (`Γ ⊢ A`) for intuitionistic linear
/// logic. Premises come before their conclusion, and the root is the last
/// inference.
///
/// [`Proof::derivation`] and [`Proof::derivation_within`] build one, and
/// [`Display`](std::fmt::Display) draws it as text ([`Proof::derivation`]
/// has an example). [`ViewOptions`] says how it is drawn.
///
/// A derivation has no JSON form of its own: the proof's JSON and the
/// [`ViewOptions`] it was built with make it again.
///
/// # Examples
///
/// Walk the inferences, from the root up:
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::linear::{Mode, Options, Sequent, Verdict, prove};
///
/// let sequent: Sequent = "|- A * B, ~A, ~B".parse()?;
/// let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
/// let Verdict::Proved(proof) = &outcome.verdict else {
///     panic!("provable");
/// };
/// let derivation = proof.derivation()?;
/// let root = derivation.inference(derivation.root());
/// assert_eq!(root.rule().to_string(), "⊗");
/// let formulas: Vec<String> = root
///     .sequent()
///     .iter()
///     .map(|&member| derivation.formula(member).to_string())
///     .collect();
/// assert_eq!(formulas, ["A ⊗ B", "~A", "~B"]);
/// for &premise in root.premises() {
///     assert_eq!(derivation.inference(premise).rule().to_string(), "ax");
/// }
/// # Ok::<(), deduxit::Error>(())
/// ```
#[derive(Clone, Debug)]
pub struct Derivation<'a> {
    /// The forest the sequents' occurrences index.
    forest: &'a Forest,
    /// The intuitionistic reading, for a two-sided derivation.
    reading: Option<Reading<'a>>,
    /// The inferences, premises before conclusions, the root last.
    inferences: Vec<Inference>,
    /// Whether the root concludes a goal of a proof rather than the
    /// sequent.
    of_goal: bool,
    /// The mode the proof is meant for, where it records one.
    mode: Option<Mode>,
    /// The instances of the proof's members past the forest, where it
    /// has them.
    framing: Option<Box<Framing<'a>>>,
}

impl<'a> Derivation<'a> {
    /// Unfolds a proof into the one-sided derivation of classical linear
    /// logic, unless it is larger than `limits` allow or `stop` fires on
    /// the way. `stop` is asked in the checker's passes and once per node
    /// unfolded.
    ///
    /// The proof must be correct; the unfolding fails as
    /// [`check`](Proof::check) would if it is not. It is checked in the
    /// most liberal one-sided mode, so the derivation shows every rule the
    /// proof uses, whatever mode the proof was found in.
    pub(crate) fn new(
        proof: &'a Proof,
        view: &ViewOptions,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
        Self::build(proof, Self::ONE_SIDED, None, view, limits, stop)
    }

    /// The most inferences a derivation holds: as many as an [`InfId`]
    /// counts.
    pub const MOST: u64 = u32::MAX as u64;

    /// The rules of a one-sided derivation: every rule a proof can use.
    pub(crate) const ONE_SIDED: Mode = Mode::CLASSICAL.with_affine().with_mix();

    /// The rules of a two-sided derivation. Weakening is among them, so
    /// that a proof that uses it can be shown.
    pub(crate) const TWO_SIDED: Mode = Mode::INTUITIONISTIC.with_affine();

    /// The rules of a two-sided derivation of `proof`: of the Lambek
    /// calculus with empty antecedents for a proof meant for an ordered
    /// mode, read in its order, and [`TWO_SIDED`](Self::TWO_SIDED) for
    /// any other.
    pub(crate) fn two_sided_mode(proof: &Proof) -> Mode {
        if proof.mode().is_some_and(Mode::is_ordered) {
            Mode::LAMBEK.with_empty_antecedents()
        } else {
            Self::TWO_SIDED
        }
    }

    /// Unfolds a proof into the two-sided derivation of intuitionistic
    /// linear logic, `Γ ⊢ A` at every inference with the intuitionistic
    /// rule names, unless it is larger than `limits` allow or `stop` fires
    /// on the way.
    ///
    /// The proof must pass the checker in intuitionistic mode (affine or
    /// not); a proof meant for an ordered mode must pass it in the Lambek
    /// calculus with empty antecedents, read in its order. The unfolding
    /// fails as that check would.
    pub(crate) fn two_sided(
        proof: &'a Proof,
        view: &ViewOptions,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
        let mode = Self::two_sided_mode(proof);
        let reading = check::reading(proof, mode)?;
        Self::build(proof, mode, reading, view, limits, stop)
    }

    /// Checks the proof in `mode` and unfolds it, two-sided when a reading
    /// is given.
    fn build(
        proof: &'a Proof,
        mode: Mode,
        reading: Option<Reading<'a>>,
        view: &ViewOptions,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
        let conclusion = proof.conclusion();
        let inferences = unfold(
            proof,
            &conclusion,
            mode,
            reading.as_ref(),
            view,
            limits,
            &mut stop,
        )?;
        Ok(Self {
            forest: proof.forest(),
            reading,
            inferences,
            of_goal: proof.goal().is_some(),
            mode: proof.mode(),
            framing: proof
                .instances()
                .map(|table| Box::new(Framing::new(&proof.forest().sequent().fo, table))),
        })
    }

    /// Wraps inferences that already have the derivation's shape: premises
    /// before conclusions, the root last, two-sided under a reading.
    #[cfg(feature = "interactive")]
    pub(crate) fn from_parts(
        forest: &'a Forest,
        reading: Option<Reading<'a>>,
        inferences: Vec<Inference>,
        mode: Mode,
    ) -> Self {
        debug_assert_ne!(inferences, []);
        Self {
            forest,
            reading,
            inferences,
            of_goal: false,
            mode: Some(mode),
            framing: None,
        }
    }

    /// Returns the derivation with its members past the forest standing
    /// for the instances `table` numbers, as a session's do.
    #[cfg(feature = "interactive")]
    pub(crate) fn with_framing(self, table: &'a crate::linear::Instances) -> Self {
        Self {
            framing: Some(Box::new(Framing::new(&self.forest.sequent().fo, table))),
            ..self
        }
    }

    /// Returns whether the root concludes a goal of its proof rather than
    /// the sequent.
    pub(crate) const fn is_of_goal(&self) -> bool {
        self.of_goal
    }

    /// Returns the mode the proof is meant for, where it records one.
    pub(crate) const fn mode(&self) -> Option<Mode> {
        self.mode
    }

    /// Reads the derivation back as a term of the linear lambda calculus
    /// ([`lambda`](crate::lambda)), built within `limits` and asking
    /// `stop` once per inference.
    ///
    /// The derivation must be two-sided, of intuitionistic linear logic,
    /// compact or not. Where it weakens a formula without `!`, the term is
    /// of the affine lambda calculus.
    ///
    /// # Errors
    ///
    /// [`Error::NoTerm`] for a derivation that has no term: one-sided (of
    /// classical linear logic), with Mix, of the cyclic or the Lambek
    /// calculus, with quantifiers, or with open goals;
    /// [`Refusal::Stopped`](crate::Refusal::Stopped) when `stop` fired.
    pub fn term(
        &self,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<crate::lambda::Term, Error> {
        crate::lambda::linear::read(self, limits, stop)
    }

    /// Unfolds a proof whose root concludes `goal` rather than the
    /// sequent's roots, as the search from a goal returns it, into the
    /// inferences of its derivation: premises before conclusions, the root
    /// last, two-sided in intuitionistic mode. Fails when the derivation is
    /// larger than `limits` allow, when `stop` fires, and as the checker
    /// in `mode` would on a node that misapplies its rule or a root that
    /// does not conclude the goal.
    #[cfg(feature = "interactive")]
    pub(crate) fn of_goal(
        proof: &'a Proof,
        goal: &[OccId],
        mode: Mode,
        view: &ViewOptions,
        limits: &Limits,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Vec<Inference>, Error> {
        let reading = check::reading(proof, mode)?;
        // A graft is read rule by rule, never drawn compact.
        let view = view.with_compact(Compact::Never);
        let mut grafted = unfold(proof, goal, mode, reading.as_ref(), &view, limits, stop)?;
        // A session keeps no proof term whose nodes these would name.
        for inference in &mut grafted {
            inference.node = None;
        }
        Ok(grafted)
    }

    /// Returns the forest the sequents' members index.
    pub fn forest(&self) -> &'a Forest {
        self.forest
    }

    /// Returns the occurrence a member of the derivation stands for.
    pub fn occurrence(&self, member: Member) -> OccId {
        self.framing
            .as_deref()
            .map_or(member.occ(), |framing| framing.occurrence(member))
    }

    /// Returns the formula a member of the derivation stands for, written
    /// as the derivation shows it: two-sided as its reading writes it,
    /// else one-sided. An instance of a quantified formula's body is
    /// written with its terms in place of the bound variables.
    ///
    /// # Panics
    ///
    /// For a member past the forest that the proof's table of instances
    /// does not hold: no member of this derivation.
    pub fn formula(&self, member: Member) -> impl std::fmt::Display + 'a {
        if let Some(framing) = &self.framing
            && member.index() >= self.forest.len()
        {
            let mut text = String::new();
            crate::linear::sequents::notation::TEXT
                .instance(
                    &mut text,
                    self.forest,
                    self.reading.as_ref(),
                    framing,
                    member,
                    None,
                )
                .expect("writing into a string cannot fail");
            return text;
        }
        match &self.reading {
            Some(reading) => reading.formula(member.occ()).to_string(),
            None => self.forest.formula(member.occ()).to_string(),
        }
    }

    /// Returns the instances of the proof's members past the forest,
    /// where it has them.
    pub(crate) fn framing(&self) -> Option<&Framing<'a>> {
        self.framing.as_deref()
    }

    /// Returns the intuitionistic reading of a two-sided derivation, or
    /// `None` for a one-sided one.
    pub fn reading(&self) -> Option<&Reading<'a>> {
        self.reading.as_ref()
    }

    /// Returns every inference, premises before conclusions, the root last.
    pub fn inferences(&self) -> &[Inference] {
        &self.inferences
    }

    /// Returns the inference at `id`, which must belong to this derivation.
    ///
    /// # Panics
    ///
    /// For an id past its end, as one of another derivation may be.
    pub fn inference(&self, id: InfId) -> &Inference {
        &self.inferences[id.index()]
    }

    /// Returns the root: the inference that concludes the sequent.
    pub fn root(&self) -> InfId {
        // A derivation has at least one inference and at most `MOST`, so
        // this neither wraps nor truncates.
        InfId::new(self.inferences.len() as u32 - 1)
    }
}
