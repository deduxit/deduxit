// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! What a decision is asked: the logic, the translation into linear
//! logic and its target, the method, and the options that hold them.

use super::Sequent;
use crate::Error;
use crate::linear::fragment::{Fragment, Mode};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// An ordinary propositional logic.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Logic {
    /// Classical logic, whose derivations are LK's.
    Classical,
    /// Intuitionistic logic, whose derivations are LJ's.
    #[default]
    Intuitionistic,
    /// Minimal logic: intuitionistic logic without ex falso. False is a
    /// proposition like any other and does not prove every formula.
    Minimal,
}

impl Logic {
    /// Every logic, in the order of [`NAMES`](Self::NAMES).
    pub const ALL: &'static [Self] = &[Self::Classical, Self::Intuitionistic, Self::Minimal];

    /// The logics' names, which [`name`](Self::name) writes and `FromStr`
    /// reads.
    pub const NAMES: &'static [&'static str] = &["classical", "intuitionistic", "minimal"];

    /// Returns the logic's name in lower case.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Classical => "classical",
            Self::Intuitionistic => "intuitionistic",
            Self::Minimal => "minimal",
        }
    }

    /// Returns the name of its sequent calculus: LK, or LJ for the other
    /// two.
    pub const fn calculus(self) -> &'static str {
        match self {
            Self::Classical => "LK",
            Self::Intuitionistic | Self::Minimal => "LJ",
        }
    }
}

impl Display for Logic {
    /// Writes the logic's [`name`](Self::name).
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

impl std::str::FromStr for Logic {
    type Err = Error;

    /// Reads a logic's [`name`](Self::name), one of
    /// [`NAMES`](Self::NAMES).
    fn from_str(name: &str) -> Result<Self, Error> {
        crate::name::named(Self::ALL, Self::name, name, "logic", Self::NAMES)
    }
}

/// A translation of ordinary logic into linear logic, by its name.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum Translation {
    /// Classical logic into affine MALL: the one-sided sequent in negation
    /// normal form, `∧` as `&`, `∨` as `⅋`, true as `⊤`, false as `⊥`, an
    /// implication `A → B` as `¬A ∨ B` and `A ↔ B` as `(A → B) ∧ (B → A)`.
    #[cfg_attr(feature = "serialize", serde(rename = "affine"))]
    Affine,
    /// Girard's call-by-name translation into ILL: `A → B` as `!A ⊸ B`,
    /// `∧` as `&`, `A ∨ B` as `!A ⊕ !B`, `¬A` as `!A ⊸ 0`, true as `⊤`,
    /// false as `0`, every hypothesis under `!`.
    #[cfg_attr(feature = "serialize", serde(rename = "cbn"))]
    CallByName,
    /// Girard's call-by-value translation into ILL: an atom `a` as `!a`,
    /// `A → B` as `!(A ⊸ B)`, `∧` as `⊗`, `∨` as `⊕`, `¬A` as `!(A ⊸ 0)`,
    /// true as `1`, false as `0`.
    #[cfg_attr(feature = "serialize", serde(rename = "cbv"))]
    CallByValue,
    /// Liang and Miller's 0/1 translation into ILL, as the LLTP library
    /// states it: hypotheses by the negative translation `t₀` under `!`,
    /// the goal by the positive one `t₁`, with `t₀(A ∧ B) = !t₀A & !t₀B`,
    /// `t₀(A → B) = !t₁A ⊸ !t₀B`, `t₁(A ∧ B) = !(t₁A & t₁B)`,
    /// `t₁(A → B) = !(!t₀A ⊸ t₁B)`, `A ∨ B` as `!A ⊕ !B` in both, true as
    /// `⊤` and `1`, false as `0`, and `¬A` as `A → false`.
    #[cfg_attr(feature = "serialize", serde(rename = "01"))]
    ZeroOne,
}

impl Translation {
    /// Every translation, in the order of [`NAMES`](Self::NAMES).
    pub const ALL: &'static [Self] = &[
        Self::Affine,
        Self::CallByName,
        Self::CallByValue,
        Self::ZeroOne,
    ];

    /// The translation that decides intuitionistic and minimal logic by
    /// default, chosen by a run of the ILTP library's propositional
    /// problems.
    pub const DEFAULT_INTUITIONISTIC: Self = Self::CallByName;

    /// The translations' names, which [`name`](Self::name) writes and
    /// `FromStr` reads.
    pub const NAMES: &'static [&'static str] = &["affine", "cbn", "cbv", "01"];

    /// Returns the translation's short name: `affine`, `cbn`, `cbv` or
    /// `01`, as the command and the JSON spell it.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Affine => "affine",
            Self::CallByName => "cbn",
            Self::CallByValue => "cbv",
            Self::ZeroOne => "01",
        }
    }

    /// Returns the translation the logic is decided with by default:
    /// [`Affine`](Self::Affine) for classical logic,
    /// [`DEFAULT_INTUITIONISTIC`](Self::DEFAULT_INTUITIONISTIC) for the
    /// others.
    pub const fn default_for(logic: Logic) -> Self {
        match logic {
            Logic::Classical => Self::Affine,
            Logic::Intuitionistic | Logic::Minimal => Self::DEFAULT_INTUITIONISTIC,
        }
    }

    /// Returns whether the translation decides the logic: the affine one
    /// classical logic, the others intuitionistic and minimal logic.
    pub const fn decides(self, logic: Logic) -> bool {
        matches!(
            (self, logic),
            (Self::Affine, Logic::Classical)
                | (
                    Self::CallByName | Self::CallByValue | Self::ZeroOne,
                    Logic::Intuitionistic | Logic::Minimal
                )
        )
    }

    /// Returns the fragment and mode of the translation's images: MALL in
    /// classical affine mode, or LL in intuitionistic mode.
    pub const fn target(self) -> Target {
        match self {
            Self::Affine => Target {
                fragment: Fragment::MALL,
                mode: Mode::CLASSICAL.with_affine(),
            },
            Self::CallByName | Self::CallByValue | Self::ZeroOne => Target {
                fragment: Fragment::LL,
                mode: Mode::INTUITIONISTIC,
            },
        }
    }
}

/// Where a translation's images lie: the largest fragment and the mode
/// they are decided in.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Target {
    /// The fragment.
    pub fragment: Fragment,
    /// The mode.
    pub mode: Mode,
}

impl Display for Target {
    /// Writes the fragment's name in the mode, after `affine` in classical
    /// affine mode: `affine MALL`, `ILL`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.mode.is_affine() && !self.mode.is_intuitionistic() {
            f.write_str("affine ")?;
        }
        f.write_str(self.fragment.name_in(self.mode))
    }
}

impl Display for Translation {
    /// Writes the translation's [`name`](Self::name).
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

impl std::str::FromStr for Translation {
    type Err = Error;

    /// Reads a translation's [`name`](Self::name), one of
    /// [`NAMES`](Self::NAMES).
    fn from_str(name: &str) -> Result<Self, Error> {
        crate::name::named(Self::ALL, Self::name, name, "translation", Self::NAMES)
    }
}

/// How an ordinary sequent is decided: through its embedding into linear
/// logic, or by a calculus of its own logic.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Method {
    /// The sequent's image under a [`Translation`] is searched in linear
    /// logic, and the proof read back as a derivation of LK or LJ
    /// ([`decide()`](super::decide())). It decides every logic; intuitionistic and minimal
    /// logic go through exponentials, so the search may answer "unknown".
    Embedding,
    /// Dyckhoff's calculus LJT ([`ljt::decide`](super::ljt::decide)), whose search always
    /// ends: intuitionistic and minimal logic only.
    Ljt,
    /// A SAT solver ([`sat::decide`](super::sat::decide)), whose search always ends. A sequent
    /// that is not valid gets a Kripke model. In intuitionistic and
    /// minimal logic the solver works on clauses for the implications; in
    /// classical logic on the clauses of the sequent's negation, and a
    /// valid sequent gets a checked clausal proof. Needs the cargo feature
    /// `sat`.
    #[cfg(feature = "sat")]
    Sat,
    /// The truth table ([`truth_table`](super::truth_table)): every assignment of the atoms,
    /// classical logic only, within [`Limits::table_atoms`](crate::Limits::table_atoms). A sequent
    /// that is not valid gets its first falsifying row as a countermodel.
    Table,
    /// A signed analytic tableau ([`Tableau`](super::Tableau)), classical logic only. A
    /// closed tableau is a derivation of LK, which is checked; an open
    /// branch gives a countermodel, which is checked too. Needs the cargo
    /// feature `interactive`.
    #[cfg(feature = "interactive")]
    Tableau,
    /// A non-clausal connection calculus ([`connection::decide`](super::connection::decide)),
    /// nanoCoP's for classical logic and nanoCoP-i's, with prefixes, for
    /// intuitionistic logic, first-order sequents included. A valid sequent
    /// gets a checked derivation of LK or LJ. Classically, where the search
    /// has no fresh variable to make it is exhaustive and answers "not
    /// valid"; elsewhere, and always intuitionistically, it deepens its
    /// bound on the active path and answers "unknown" where none is found.
    Connection,
    /// Grounding over the constants and the SAT solver
    /// ([`grounding::decide`](super::grounding::decide)), for the first-order sequents of the
    /// Bernays–Schönfinkel and the monadic class: valid with a checked
    /// clausal proof, or not valid with a checked finite [`Structure`](super::Structure),
    /// which refutes the sequent in every logic. A valid sequent is only
    /// valid in classical logic. Needs the cargo feature `sat`.
    #[cfg(feature = "sat")]
    Grounding,
    /// The finite model search ([`models::decide`](super::models::decide)): domains of 1, 2, …
    /// elements up to [`search::Options::model_size`](crate::linear::search::Options::model_size),
    /// each written as a propositional formula for the SAT solver. A
    /// sequent that a structure of that size refutes is not valid, with
    /// the checked [`Structure`](super::Structure), in every logic; the search never answers
    /// "valid". Needs the cargo feature `sat`.
    #[cfg(feature = "sat")]
    Models,
}

impl Method {
    /// Every method, in the order of [`NAMES`](Self::NAMES).
    pub const ALL: &'static [Self] = &[
        Self::Embedding,
        Self::Ljt,
        #[cfg(feature = "sat")]
        Self::Sat,
        Self::Table,
        #[cfg(feature = "interactive")]
        Self::Tableau,
        Self::Connection,
        #[cfg(feature = "sat")]
        Self::Grounding,
        #[cfg(feature = "sat")]
        Self::Models,
    ];

    /// The methods' names, which [`name`](Self::name) writes and `FromStr`
    /// reads.
    pub const NAMES: &'static [&'static str] = &[
        "embedding",
        "ljt",
        #[cfg(feature = "sat")]
        "sat",
        "table",
        #[cfg(feature = "interactive")]
        "tableau",
        "connection",
        #[cfg(feature = "sat")]
        "grounding",
        #[cfg(feature = "sat")]
        "models",
    ];

    /// Returns the method's name: `embedding`, `ljt`, `sat`, `table`,
    /// `tableau`, `connection`, `grounding` or `models`, as the command and
    /// the JSON spell it.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Embedding => "embedding",
            Self::Ljt => "ljt",
            #[cfg(feature = "sat")]
            Self::Sat => "sat",
            Self::Table => "table",
            #[cfg(feature = "interactive")]
            Self::Tableau => "tableau",
            Self::Connection => "connection",
            #[cfg(feature = "sat")]
            Self::Grounding => "grounding",
            #[cfg(feature = "sat")]
            Self::Models => "models",
        }
    }

    /// Returns the method's name in a sentence: `LJT`, `a truth table`.
    pub(crate) const fn title(self) -> &'static str {
        match self {
            Self::Embedding => "the embedding",
            Self::Ljt => "LJT",
            #[cfg(feature = "sat")]
            Self::Sat => "the SAT solver",
            Self::Table => "a truth table",
            #[cfg(feature = "interactive")]
            Self::Tableau => "a tableau",
            Self::Connection => "the connection prover",
            #[cfg(feature = "sat")]
            Self::Grounding => "grounding",
            #[cfg(feature = "sat")]
            Self::Models => "the finite model search",
        }
    }

    /// Returns the logics the method decides, in words.
    pub(crate) const fn logics(self) -> &'static str {
        match self {
            Self::Embedding => "every logic",
            #[cfg(feature = "sat")]
            Self::Sat => "every logic",
            Self::Ljt => "intuitionistic and minimal logic",
            Self::Table => "classical logic",
            #[cfg(feature = "interactive")]
            Self::Tableau => "classical logic",
            Self::Connection => "classical and intuitionistic logic",
            #[cfg(feature = "sat")]
            Self::Grounding => "every logic, refuting only in intuitionistic and minimal logic",
            #[cfg(feature = "sat")]
            Self::Models => "every logic, refuting only",
        }
    }

    /// Returns the method the logic is decided with by default: the SAT
    /// solver, chosen by a run of the ILTP library's propositional
    /// problems. Without the cargo feature `sat` it is the embedding for
    /// classical logic and LJT for the others.
    ///
    /// The default keeps the evidence of the other methods. In classical
    /// logic it asks the embedding for a derivation of LK or a refutation,
    /// within [`EVIDENCE_WORK`](super::EVIDENCE_WORK); in intuitionistic and minimal logic it
    /// prefers LJT's derivation. Where a bound stops the SAT solver, the
    /// default gives way to the embedding ([`gives_way`](super::gives_way)).
    pub const fn default_for(logic: Logic) -> Self {
        match logic {
            #[cfg(feature = "sat")]
            Logic::Classical | Logic::Intuitionistic | Logic::Minimal => Self::Sat,
            #[cfg(not(feature = "sat"))]
            Logic::Classical => Self::Embedding,
            #[cfg(not(feature = "sat"))]
            Logic::Intuitionistic | Logic::Minimal => Self::Ljt,
        }
    }

    /// Returns whether the method decides the logic: the embedding and the
    /// SAT solver every logic, LJT intuitionistic and minimal logic, the
    /// truth table, the tableau and the connection prover classical logic.
    pub const fn decides(self, logic: Logic) -> bool {
        match self {
            Self::Embedding => true,
            Self::Ljt => !matches!(logic, Logic::Classical),
            #[cfg(feature = "sat")]
            Self::Sat => true,
            Self::Table => matches!(logic, Logic::Classical),
            #[cfg(feature = "interactive")]
            Self::Tableau => matches!(logic, Logic::Classical),
            Self::Connection => matches!(logic, Logic::Classical | Logic::Intuitionistic),
            #[cfg(feature = "sat")]
            Self::Grounding | Self::Models => true,
        }
    }
}

impl Display for Method {
    /// Writes the method's [`name`](Self::name).
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

impl std::str::FromStr for Method {
    type Err = Error;

    /// Reads a method's [`name`](Self::name), one of
    /// [`NAMES`](Self::NAMES).
    fn from_str(name: &str) -> Result<Self, Error> {
        crate::name::named(Self::ALL, Self::name, name, "method", Self::NAMES)
    }
}

/// How an ordinary sequent is decided: the logic, the method, and the
/// translation of the embedding.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Options {
    /// The logic.
    pub logic: Logic,
    /// The translation, or `None` for the logic's default. A translation
    /// given without a method chooses the embedding.
    pub translation: Option<Translation>,
    /// The method, or `None` for the embedding when a translation is
    /// given and otherwise the logic's default ([`Method::default_for`]).
    pub method: Option<Method>,
}

impl Options {
    /// Returns the options with the logic given.
    #[must_use]
    pub const fn with_logic(self, logic: Logic) -> Self {
        Self { logic, ..self }
    }

    /// Returns the options with the translation given, `None` for the
    /// logic's default.
    #[must_use]
    pub const fn with_translation(self, translation: Option<Translation>) -> Self {
        Self {
            translation,
            ..self
        }
    }

    /// Returns the translation these options choose.
    pub const fn translation(&self) -> Translation {
        match self.translation {
            Some(translation) => translation,
            None => Translation::default_for(self.logic),
        }
    }

    /// Returns the options with the method given, `None` for the
    /// default.
    #[must_use]
    pub const fn with_method(self, method: Option<Method>) -> Self {
        Self { method, ..self }
    }

    /// Returns the method these options choose: the one given, else the
    /// embedding when a translation is given, else the logic's default.
    pub const fn method(&self) -> Method {
        match (self.method, self.translation) {
            (Some(method), _) => method,
            (None, Some(_)) => Method::Embedding,
            (None, None) => Method::default_for(self.logic),
        }
    }

    /// Returns the method that decides `sequent` under these options:
    /// [`method`](Self::method), but for a first-order sequent when no
    /// method and no translation is given grounding where the sequent lies
    /// in the Bernays–Schönfinkel or the monadic class (with the feature
    /// `sat`), else the connection prover in classical logic and the
    /// embedding in the others, the methods that take quantifiers.
    /// [`decide()`](super::decide()) decides by it; with no method given, a first-order
    /// sequent is decided in the order [`decide_within`](super::decide_within) gives: a first
    /// attempt by each method that applies, the finite model search (with
    /// the feature `sat`), then the attempts that ran out of work again.
    pub fn method_for(&self, sequent: &Sequent) -> Method {
        match (self.method, self.translation) {
            #[cfg(feature = "sat")]
            (None, None) if sequent.formulas().is_first_order() && grounds(sequent) => {
                Method::Grounding
            }
            (None, None)
                if sequent.formulas().is_first_order() && self.logic == Logic::Classical =>
            {
                Method::Connection
            }
            (None, _) if sequent.formulas().is_first_order() => Method::Embedding,
            _ => self.method(),
        }
    }
}

/// Returns whether grounding decides `sequent`: whether it lies in the
/// Bernays–Schönfinkel or the monadic class, read within a normal form of
/// at most [`GROUNDS_NODES`] nodes made within [`GROUNDS_WORK`] units of
/// work, past which the default does not try. The test takes no stop of
/// the caller's, so its own bound ends it.
#[cfg(feature = "sat")]
pub(super) fn grounds(sequent: &Sequent) -> bool {
    let limits = crate::Limits::default().with_occurrences(Some(GROUNDS_NODES));
    super::grounding::applies_within(sequent, &limits, &mut |progress| {
        progress.done > GROUNDS_WORK
    })
    .unwrap_or(false)
}

/// The most work of the normal form in which the default asks whether
/// grounding decides a first-order sequent, in grounding's units (a node
/// made, an operand or a free variable read, by the normal form and then
/// miniscoping): 2¹⁸, about five times the most an ILTP problem in the
/// classes takes (52 154, SYJ212+1.010) and some milliseconds, while a
/// congruence axiom of thousands of arguments, a
/// nest of thousands of quantifiers whose normal form is quadratic, took
/// minutes. The command asks several times per sequent.
#[cfg(feature = "sat")]
const GROUNDS_WORK: u64 = 1 << 18;

/// The most nodes of the normal form in which the default asks whether
/// grounding decides a first-order sequent: 2²⁰, far above ILTP's
/// problems in the classes, while an equivalence nested deep, whose normal
/// form doubles per level, is left to the other methods.
#[cfg(feature = "sat")]
pub(super) const GROUNDS_NODES: u64 = 1 << 20;
