// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Ordinary logic: classical, intuitionistic and minimal, propositional
//! and first-order.
//!
//! [`decide()`] says whether a [`Sequent`] is valid in a [`Logic`]. A
//! valid sequent comes with checked [`Evidence`]: a derivation of LK
//! (classical logic) or LJ (intuitionistic and minimal logic), Gentzen's
//! sequent calculi, which [`Derivation::check`] accepted; or, from the SAT
//! solver in classical logic, a clausal proof that the crate's checker
//! accepted. A sequent that is not valid comes with a checked Kripke model
//! that refutes it ([`Countermodel`]) where the method finds one.
//! [`decide_within`] does the same under the caller's bounds, search
//! options and stop.
//!
//! [`Method`] chooses how a sequent is decided:
//! - a SAT solver, the default for every logic; it needs the cargo feature
//!   `sat`, on by default;
//! - Dyckhoff's calculus LJT, for intuitionistic and minimal logic, their
//!   default without `sat`;
//! - the embedding, classical logic's default without `sat`. It translates
//!   the sequent into linear logic by a [`Translation`] ([`translate()`]
//!   returns an [`Image`]: a linear sequent and the mode to prove it in),
//!   and reads the linear proof back ([`Image::read_back`]). Classical
//!   logic goes into affine MALL, intuitionistic and minimal logic into
//!   intuitionistic linear logic.
//!
//! [`Method::default_for`] says which evidence the default keeps from the
//! other methods.
//!
//! [`WrittenTableau`] reads a signed tableau of classical logic that a
//! student wrote as text and [reviews](WrittenTableau::review) it rule by
//! rule, naming the first step that breaks a rule.
//!
//! A sequent may be first-order, with predicates and quantifiers
//! ([`Sequent`] has the syntax). Grounding, the connection prover and the
//! embedding decide it, and the finite model search may refute it;
//! [`Options::method_for`] says which runs by default. The propositional
//! methods refuse such a sequent ([`Error::FirstOrder`]), and a
//! derivation of one is checked as any other.
//!
//! The excluded middle is not valid in intuitionistic logic, the default:
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::ordinary::{Options, Sequent, Verdict, decide};
//!
//! let sequent: Sequent = "a \\/ ~a".parse()?;
//! let outcome = decide(&sequent, &Options::default())?;
//! match &outcome.verdict {
//!     Verdict::Valid(evidence) => println!("valid\n{}", evidence.derivation()?),
//!     Verdict::NotValid => println!("not valid"),
//!     _ => println!("unknown"),
//! }
//! if let Some(countermodel) = &outcome.countermodel {
//!     println!("{}", countermodel.model);
//! }
//! assert!(matches!(outcome.verdict, Verdict::NotValid));
//! # Ok::<(), deduxit::Error>(())
//! ```

// A new variant must be decided at every arm, never fall into an
// existing one.
#![deny(clippy::wildcard_enum_match_arm)]

/// A branch of a derivation of LK as two builders make it.
mod branch;
/// What moving or removing a binder does to de Bruijn indices.
mod bruijn;
pub mod connection;
/// Deciding an ordinary sequent in one call.
mod decide;
/// The derivations of LK and LJ, the read-back and its check.
mod derivation;
/// What the SAT encodings of first-order models share.
#[cfg(feature = "sat")]
mod encode;
pub mod equality;
#[cfg(feature = "sat")]
pub mod grounding;
#[cfg(feature = "sat")]
pub mod herbrand;
/// The instances of a quantified formula, built or found.
mod instance;
/// Step-by-step proving in LK and LJ.
#[cfg(feature = "interactive")]
mod interactive;
pub mod interpolant;
/// The invertible rules of LK and LJ, applied by the goal state.
#[cfg(feature = "interactive")]
mod invert;
/// The propositional sequent of a Kripke model's search.
#[cfg(feature = "sat")]
mod kripke_search;
/// Kripke models of first-order sequents with growing domains.
mod kripke_structure;
pub mod ljt;
/// The check of a Kripke model.
mod model;
#[cfg(feature = "sat")]
pub mod models;
pub mod negative;
pub mod normal;
#[cfg(feature = "parse")]
mod parse;
/// Every subformula's polarity where a sequent fails.
mod polarity;
pub mod resolution;
/// The certificates over `Prop`.
#[cfg(feature = "rocq")]
pub(crate) mod rocq;
/// The certificates of first-order countermodels for deduxit's library.
#[cfg(feature = "rocq")]
pub(crate) mod rocq_models;
#[cfg(feature = "sat")]
pub mod sat;
/// One world's reading of a first-order sequent, which the countermodels'
/// checks share.
mod semantics;
/// Finite structures, the countermodels of first-order sequents.
mod structure;
/// The translations into linear logic.
mod table;
#[cfg(feature = "interactive")]
mod tableau;
/// Formulas copied between arenas: the goal state's search, the normal
/// forms' claims, the tableau review's expected sequent.
mod transfer;
mod translate;
/// The terms that formulas reach, each once.
mod walk;
/// A tableau written by hand, read and reviewed.
#[cfg(all(feature = "interactive", feature = "parse", feature = "serialize"))]
mod written;

pub use decide::{
    EVIDENCE_WORK, Evidence, Outcome, Search, Verdict, decide, decide_within, gives_way,
};
pub use derivation::{Derivation, Inference, Rule, Side};
#[cfg(all(feature = "interactive", feature = "serialize"))]
pub(crate) use interactive::Parts;
#[cfg(feature = "interactive")]
pub use interactive::{
    Applicable, BindingFault, Candidate, Closed, Context, Hint, Interactive, Needs, Status, Step,
    StepError,
};
pub use kripke_structure::{KripkeCountermodel, KripkeStructure, KripkeWorld};
pub use model::{Countermodel, Forcing, MODEL_CELLS};
#[cfg(feature = "parse")]
pub use parse::{Problem, read_tptp, tptp_text};
pub use structure::{Evaluation, FiniteCountermodel, Structure};
pub use table::{TruthTable, truth_table};
#[cfg(feature = "interactive")]
pub use tableau::{End, Sign, Signed, Tableau, TableauNode};
pub use translate::{FALSE, Image, translate};
#[cfg(all(feature = "interactive", feature = "parse", feature = "serialize"))]
pub use written::{Standing, TableauReview, WrittenTableau};

use crate::Error;
use crate::arena::Arena;
use crate::first_order::{Args, FirstOrder, FoTerm, FoTermId, Interner};
use crate::hash::HashMap;
use crate::linear::fragment::{Fragment, Mode};
use crate::linear::sequents::notation::TEXT;
use crate::linear::sequents::{Stop, Stops};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult, Write};

/// What an arm of a propositional method says of a binder: the method
/// refused the first-order sequent at its door
/// ([`Formulas::propositional`]), so none reaches it.
pub(crate) const REFUSED: &str = "a propositional method refuses a first-order sequent at its door";

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
    /// ([`decide()`]). It decides every logic; intuitionistic and minimal
    /// logic go through exponentials, so the search may answer "unknown".
    Embedding,
    /// Dyckhoff's calculus LJT ([`ljt::decide`]), whose search always
    /// ends: intuitionistic and minimal logic only.
    Ljt,
    /// A SAT solver ([`sat::decide`]), whose search always ends. A sequent
    /// that is not valid gets a Kripke model. In intuitionistic and
    /// minimal logic the solver works on clauses for the implications; in
    /// classical logic on the clauses of the sequent's negation, and a
    /// valid sequent gets a checked clausal proof. Needs the cargo feature
    /// `sat`.
    #[cfg(feature = "sat")]
    Sat,
    /// The truth table ([`truth_table`]): every assignment of the atoms,
    /// classical logic only, within [`Limits::table_atoms`](crate::Limits::table_atoms). A sequent
    /// that is not valid gets its first falsifying row as a countermodel.
    Table,
    /// A signed analytic tableau ([`Tableau`]), classical logic only. A
    /// closed tableau is a derivation of LK, which is checked; an open
    /// branch gives a countermodel, which is checked too. Needs the cargo
    /// feature `interactive`.
    #[cfg(feature = "interactive")]
    Tableau,
    /// A non-clausal connection calculus ([`connection::decide`]),
    /// nanoCoP's for classical logic and nanoCoP-i's, with prefixes, for
    /// intuitionistic logic, first-order sequents included. A valid sequent
    /// gets a checked derivation of LK or LJ. Classically, where the search
    /// has no fresh variable to make it is exhaustive and answers "not
    /// valid"; elsewhere, and always intuitionistically, it deepens its
    /// bound on the active path and answers "unknown" where none is found.
    Connection,
    /// Grounding over the constants and the SAT solver
    /// ([`grounding::decide`]), for the first-order sequents of the
    /// Bernays–Schönfinkel and the monadic class: valid with a checked
    /// clausal proof, or not valid with a checked finite [`Structure`],
    /// which refutes the sequent in every logic. A valid sequent is only
    /// valid in classical logic. Needs the cargo feature `sat`.
    #[cfg(feature = "sat")]
    Grounding,
    /// The finite model search ([`models::decide`]): domains of 1, 2, …
    /// elements up to [`search::Options::model_size`](crate::linear::search::Options::model_size),
    /// each written as a propositional formula for the SAT solver. A
    /// sequent that a structure of that size refutes is not valid, with
    /// the checked [`Structure`], in every logic; the search never answers
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
    /// within [`EVIDENCE_WORK`]; in intuitionistic and minimal logic it
    /// prefers LJT's derivation. Where a bound stops the SAT solver, the
    /// default gives way to the embedding ([`gives_way`]).
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
    /// [`decide()`] decides by it; where the method gives way (a bound, or
    /// a sequent grounding finds valid only classically), the embedding
    /// decides in its place, and where that leaves a first-order sequent
    /// unknown, the finite model search (with the feature `sat`) may
    /// refute it.
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
fn grounds(sequent: &Sequent) -> bool {
    let limits = crate::Limits::default().with_occurrences(Some(GROUNDS_NODES));
    grounding::applies_within(sequent, &limits, &mut |progress| {
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
const GROUNDS_NODES: u64 = 1 << 20;

/// The index of a formula in a [`Formulas`] arena.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(u32);

impl NodeId {
    /// Returns the index as a `usize`, for indexing the arena.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// A node of an ordinary formula, which refers to its subformulas by
/// arena index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    /// An atom, by its index among the arena's atom names.
    Atom(u32),
    /// `⊤`, true.
    True,
    /// `⊥`, false.
    False,
    /// `¬A`
    Not(NodeId),
    /// `A ∧ B`
    And(NodeId, NodeId),
    /// `A ∨ B`
    Or(NodeId, NodeId),
    /// `A → B`
    Implies(NodeId, NodeId),
    /// `A ↔ B`
    Iff(NodeId, NodeId),
    /// `∀x. A`: the body `A`, whose bound variable `x` its atoms' terms
    /// name by the de Bruijn index of this binder.
    Forall(NodeId),
    /// `∃x. A`: the body `A`, as for [`Forall`](Self::Forall).
    Exists(NodeId),
}

/// A node stays a tag and at most two arena indices, since the arena and
/// the translation's tables grow with its size.
const _: () = assert!(size_of::<Node>() == 12);

impl Node {
    /// Returns the subformulas, in the order they are written.
    pub const fn operands(self) -> (Option<NodeId>, Option<NodeId>) {
        match self {
            Self::Atom(_) | Self::True | Self::False => (None, None),
            Self::Not(a) | Self::Forall(a) | Self::Exists(a) => (Some(a), None),
            Self::And(a, b) | Self::Or(a, b) | Self::Implies(a, b) | Self::Iff(a, b) => {
                (Some(a), Some(b))
            }
        }
    }
}

/// The formulas of ordinary sequents, as an arena in which every
/// subformula precedes the formulas built from it and equal formulas are
/// one node, so that two formulas are equal exactly when their ids are.
///
/// A bound variable is the de Bruijn index of its binder, so two formulas
/// that differ only in the names of their bound variables are one node.
/// A binder keeps the name it was first written with, for printing.
#[derive(Clone, Default)]
pub struct Formulas {
    /// The nodes and the atom names. The name of an atom with arguments
    /// is its canonical text, `p(f(a), #0)`.
    arena: Arena<Node>,
    /// The first-order tables and their lookup maps: the symbols, the
    /// terms, each atom's predicate and arguments, and every binder's
    /// name. Made at the first binder, predicate with arguments or term;
    /// empty exactly when the arena has no binder and no predicate with
    /// arguments, and then every atom's entry is missing too. A
    /// propositional arena pays a pointer for it.
    fo: Option<Box<Interner<NodeId>>>,
}

/// An arena read from first-order tables, the index in it of every atom
/// of the tables and the id in it of every term of the tables.
#[cfg(feature = "serialize")]
pub(crate) type OfTables = (Formulas, Vec<u32>, Vec<FoTermId>);

/// The tables of an arena that has none.
static NO_TABLES: FirstOrder<NodeId> = FirstOrder::new();

impl Debug for Formulas {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut out = f.debug_struct("Formulas");
        self.arena.fields(&mut out);
        out.field("fo", &self.fo).finish()
    }
}

impl crate::arena::Id for NodeId {
    fn at(index: u32) -> Self {
        Self(index)
    }

    fn place(self) -> usize {
        self.index()
    }
}

impl crate::arena::Shape for Node {
    type Id = NodeId;

    fn operands(self) -> (Option<NodeId>, Option<NodeId>) {
        Self::operands(self)
    }

    fn atom(self) -> Option<u32> {
        if let Self::Atom(atom) = self {
            Some(atom)
        } else {
            None
        }
    }
}

impl Formulas {
    /// Returns the id of a node the arena holds, if it holds it.
    pub(crate) fn find(&self, node: Node) -> Option<NodeId> {
        self.arena.find(node)
    }

    /// The most nodes an arena holds: the ids are `u32`.
    pub const MOST: usize = Arena::<Node>::MOST;

    /// Returns the node with the id given, which must belong to this arena.
    ///
    /// # Panics
    ///
    /// If the id is past the arena's end, as an id of another arena may
    /// be.
    pub fn node(&self, id: NodeId) -> Node {
        self.arena.node(id)
    }

    /// Returns the nodes, each after its subformulas.
    pub(crate) fn nodes(&self) -> &[Node] {
        self.arena.nodes()
    }

    /// Returns the number of nodes.
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    /// Returns whether the arena has no node.
    pub fn is_empty(&self) -> bool {
        self.arena.len() == 0
    }

    /// Returns the name of the atom with the index given.
    pub fn atom_name(&self, atom: u32) -> &str {
        self.arena.atom_name(atom)
    }

    /// Returns the atom names, in the order of their indices.
    pub fn atom_names(&self) -> &[String] {
        self.arena.atom_names()
    }

    /// Returns the id of `node`, adding it unless the arena has it.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for a subformula that is no node of
    /// this arena (an id of another one) or an atom that is not one of its
    /// names, and [`Refusal::Index`](crate::Refusal::Index) when the arena
    /// is full.
    pub fn add(&mut self, node: Node) -> Result<NodeId, Error> {
        if let Some(id) = self.arena.find(node) {
            return Ok(id);
        }
        self.arena.admit(node)?;
        if matches!(node, Node::Forall(_) | Node::Exists(_)) {
            self.enter_first_order()?;
        }
        let id = self.arena.push(node);
        if matches!(node, Node::Forall(_) | Node::Exists(_)) {
            // A binder made without a name prints its variable as `x`.
            self.interner().tables.binders.push((id, "x".to_owned()));
        }
        Ok(id)
    }

    /// Fails unless `id` is a node of this arena.
    fn holds(&self, id: NodeId) -> Result<(), Error> {
        self.arena.holds(id)
    }

    /// Returns the atom named `name`, adding it unless the arena has it.
    /// The name is kept in Unicode normal form NFC, so one name is one atom
    /// however its accents are encoded. It must be an identifier of the
    /// text syntax and no keyword, so that a formula written as text reads
    /// back as itself: not `true` or `false`, and no keyword of the linear
    /// syntax either, since the translations keep the atom's name.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a name that is no identifier or a keyword,
    /// the quantifiers' `forall` and `exists` among them, and
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub fn atom(&mut self, name: &str) -> Result<NodeId, Error> {
        let Self { arena, fo } = self;
        let index = arena.atom(name, |name| {
            if fo.as_deref().is_some_and(|fo| !fo.tables.is_empty()) {
                nullary(fo, name)
            } else {
                Ok(())
            }
        })?;
        self.add(Node::Atom(index))
    }

    /// Returns the arena of the first-order `tables`, read from a
    /// document and checked, without a node yet, the index in it of every
    /// atom of the tables (an atom equal to an earlier one is that one)
    /// and the id in it of every term of the tables. Every symbol, term and argument list is interned again, so the
    /// arena holds each term once however often the document lists it.
    /// `None` where the tables have no term and every atom is nullary:
    /// the document is a propositional sequent, its atoms by name.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a symbol named by a keyword of the ordinary
    /// syntax.
    #[cfg(feature = "serialize")]
    pub(crate) fn of_tables(tables: &FirstOrder<NodeId>) -> Result<Option<OfTables>, Error> {
        let propositional = tables.terms.is_empty()
            && tables
                .atoms
                .iter()
                .all(|&(_, args)| tables.list(args).is_empty());
        if propositional {
            return Ok(None);
        }
        let mut fo = Interner::new();
        let mut predicates = vec![false; tables.symbols.len()];
        for &(p, _) in &tables.atoms {
            predicates[p.index()] = true;
        }
        let mut symbols = Vec::with_capacity(tables.symbols.len());
        for ((name, arity), &predicate) in tables.symbols.iter().zip(&predicates) {
            // Equality is a binary predicate, which `FirstOrder::check`
            // has held it to; no identifier is its name.
            let equals = name == crate::first_order::EQUALS;
            if !equals {
                crate::name::check_ordinary(name)?;
            }
            // `=` is a predicate though no atom uses it: interned as a
            // function, it would refuse the atoms the axioms make.
            let symbol = fo
                .symbol(name, *arity, predicate || equals)
                .map_err(|fault| Error::Symbol {
                    name: name.clone(),
                    fault,
                })?;
            symbols.push(symbol);
        }
        let mut terms: Vec<crate::first_order::FoTermId> = Vec::with_capacity(tables.terms.len());
        for &term in &tables.terms {
            let id = match term {
                crate::first_order::FoTerm::Bound(_) => fo.term(term),
                crate::first_order::FoTerm::App(f, args) => {
                    let items: Vec<_> =
                        tables.list(args).iter().map(|t| terms[t.index()]).collect();
                    let args = fo.list(&items);
                    fo.term(crate::first_order::FoTerm::App(symbols[f.index()], args))
                }
                // A derivation's or a goal state's terms, which the reader
                // takes where the document is one.
                crate::first_order::FoTerm::Eigen(_) | crate::first_order::FoTerm::Meta(_) => {
                    fo.term(term)
                }
            };
            terms.push(id);
        }
        let mut formulas = Self {
            fo: Some(Box::new(fo)),
            ..Self::default()
        };
        let mut indices = Vec::with_capacity(tables.atoms.len());
        for &(p, args) in &tables.atoms {
            let items: Vec<_> = tables.list(args).iter().map(|t| terms[t.index()]).collect();
            let args = formulas.interner().list(&items);
            let symbol = symbols[p.index()];
            let key = formulas.tables().atom_key(symbol, args);
            let (index, new) = formulas.arena.keyed(key);
            if new {
                formulas.interner().tables.atoms.push((symbol, args));
            }
            indices.push(index);
        }
        Ok(Some((formulas, indices, terms)))
    }

    /// Returns whether the arena is first-order: it holds a binder or a
    /// predicate with arguments. The methods that decide propositional
    /// logic refuse such an arena ([`Error::FirstOrder`]).
    pub fn is_first_order(&self) -> bool {
        !self.tables().is_empty()
    }

    /// Fails with [`Error::FirstOrder`] naming `method` where the arena is
    /// first-order.
    pub(crate) fn propositional(&self, method: &'static str) -> Result<(), Error> {
        if self.is_first_order() {
            Err(Error::FirstOrder { method })
        } else {
            Ok(())
        }
    }

    /// Returns the first-order tables, empty where the arena has none.
    pub(crate) fn first_order(&self) -> &FirstOrder<NodeId> {
        self.tables()
    }

    /// Returns the first-order tables, empty where the arena has none.
    fn tables(&self) -> &FirstOrder<NodeId> {
        self.fo.as_deref().map_or(&NO_TABLES, |fo| &fo.tables)
    }

    /// Returns the first-order tables and their maps, made empty where the
    /// arena has none yet.
    fn interner(&mut self) -> &mut Interner<NodeId> {
        interner(&mut self.fo)
    }

    /// Returns whether a symbol of the arena is named `name`.
    fn has_symbol(&self, name: &str) -> bool {
        self.fo
            .as_ref()
            .is_some_and(|fo| fo.find_symbol(name).is_some())
    }

    /// Gives every atom without one its nullary predicate, so that the
    /// tables have an entry per atom, as a first-order arena's must.
    ///
    /// # Errors
    ///
    /// [`Error::Symbol`] where an atom's name is a function symbol's.
    #[cold]
    #[inline(never)]
    fn enter_first_order(&mut self) -> Result<(), Error> {
        let had = self.tables().atoms.len();
        for n in had..self.arena.atom_names().len() {
            let Self { arena, fo } = self;
            let result = nullary(fo, &arena.atom_names()[n]);
            if let Err(error) = result {
                // All or none, so that the entries stay parallel.
                self.interner().tables.atoms.truncate(had);
                return Err(error);
            }
        }
        Ok(())
    }

    /// Returns the term made by the function symbol `name` applied to
    /// `args`, interned: a constant where `args` is empty.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a name that is no identifier or a keyword,
    /// and [`Error::Symbol`] where the name is a predicate or has another
    /// arity.
    #[cfg(any(test, feature = "parse"))]
    pub(crate) fn function(&mut self, name: &str, args: &[FoTermId]) -> Result<FoTermId, Error> {
        let name = &*crate::name::normalized(name);
        crate::name::check_ordinary(name)?;
        let symbol = self
            .interner()
            .symbol(name, args.len() as u32, false)
            .map_err(|fault| Error::Symbol {
                name: name.to_owned(),
                fault,
            })?;
        let args = self.interner().list(args);
        Ok(self.interner().term(FoTerm::App(symbol, args)))
    }

    /// Returns the bound variable of the binder `index` binders out, as a
    /// term: `0` is the innermost binder's variable.
    #[cfg(any(test, feature = "parse"))]
    pub(crate) fn variable(&mut self, index: u32) -> FoTermId {
        self.interner().term(FoTerm::Bound(index))
    }

    /// Returns the atom of the predicate `name` applied to `args`, adding
    /// it unless the arena has it. The parser calls it with arguments
    /// only; a nullary predicate is an atom ([`atom`](Self::atom)), and
    /// called with none it makes the arena first-order all the same.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a name that is no identifier or a keyword,
    /// [`Error::Symbol`] where the name is a function symbol or has another
    /// arity, and [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    #[cfg(any(test, feature = "parse"))]
    pub(crate) fn predicate(&mut self, name: &str, args: &[FoTermId]) -> Result<NodeId, Error> {
        let name = &*crate::name::normalized(name);
        crate::name::check_ordinary(name)?;
        self.enter_first_order()?;
        let symbol = self
            .interner()
            .symbol(name, args.len() as u32, true)
            .map_err(|fault| Error::Symbol {
                name: name.to_owned(),
                fault,
            })?;
        let args = self.interner().list(args);
        self.predicate_atom(symbol, args)
    }

    /// Returns whether the arena uses equality, the predicate written `s =
    /// t`: a sequent over it is decided with the equality axioms
    /// ([`equality::with_axioms`]).
    pub fn has_equality(&self) -> bool {
        self.fo
            .as_ref()
            .is_some_and(|fo| fo.find_symbol(crate::first_order::EQUALS).is_some())
    }

    /// Returns the atom `left = right`, adding it unless the arena has it,
    /// and making the arena first-order where it was not.
    ///
    /// # Errors
    ///
    /// [`Error::Symbol`] where an atom's name is a function symbol's, and
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub(crate) fn equality(&mut self, left: FoTermId, right: FoTermId) -> Result<NodeId, Error> {
        self.enter_first_order()?;
        let name = crate::first_order::EQUALS;
        let symbol = self
            .interner()
            .symbol(name, 2, true)
            .map_err(|fault| Error::Symbol {
                name: name.to_owned(),
                fault,
            })?;
        let args = self.interner().list(&[left, right]);
        self.predicate_atom(symbol, args)
    }

    /// Returns the atom of the predicate `symbol` of the tables applied to
    /// `args`, adding it unless the arena has it. The arena must be
    /// first-order, `args` as many as the symbol's arity.
    pub(crate) fn applied_atom(
        &mut self,
        symbol: crate::first_order::Symbol,
        args: &[FoTermId],
    ) -> Result<NodeId, Error> {
        let args = self.interner().list(args);
        self.predicate_atom(symbol, args)
    }

    /// Returns the atom of `symbol` applied to `args`, both of the tables,
    /// adding it unless the arena has it. The arena must be first-order.
    fn predicate_atom(
        &mut self,
        symbol: crate::first_order::Symbol,
        args: Args,
    ) -> Result<NodeId, Error> {
        let key = self.tables().atom_key(symbol, args);
        // By `entry`, so that `atom`'s lookup and insertion keep the
        // parsers as their one caller and inline there.
        let (index, new) = self.arena.keyed(key);
        if new {
            self.interner().tables.atoms.push((symbol, args));
        }
        self.add(Node::Atom(index))
    }

    /// Returns `∀x. body` (`forall`) or `∃x. body`, adding it unless the
    /// arena has it. A binder added here keeps `name` for printing; one
    /// the arena had keeps the name it was first given.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a name that is no identifier or a keyword,
    /// [`Error::IndexOutOfBounds`] for a body of another arena, and
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub(crate) fn binder(
        &mut self,
        forall: bool,
        body: NodeId,
        name: &str,
    ) -> Result<NodeId, Error> {
        let name = crate::name::normalized(name);
        crate::name::check_ordinary(&name)?;
        let len = self.len();
        let id = self.add(if forall {
            Node::Forall(body)
        } else {
            Node::Exists(body)
        })?;
        if self.len() > len
            && let Some(entry) = self.interner().tables.binders.last_mut()
        {
            // The node is new, so its entry is the last.
            entry.1 = name.into_owned();
        }
        Ok(id)
    }

    /// Returns per node how many binders it needs above it: one more than
    /// the largest index of a bound variable that no binder inside it
    /// binds, `0` for a closed formula.
    fn loose(&self) -> Vec<u32> {
        let terms = self.tables().loose();
        let mut loose: Vec<u32> = Vec::with_capacity(self.len());
        for &node in self.nodes() {
            let of = |k: NodeId| loose[k.index()];
            loose.push(match node {
                Node::Atom(a) => match self.tables().atoms.get(a as usize) {
                    Some(&(_, args)) => self
                        .tables()
                        .list(args)
                        .iter()
                        .map(|t| terms[t.index()])
                        .max()
                        .unwrap_or(0),
                    None => 0,
                },
                Node::True | Node::False => 0,
                Node::Not(a) => of(a),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    of(a).max(of(b))
                }
                Node::Forall(a) | Node::Exists(a) => of(a).saturating_sub(1),
            });
        }
        loose
    }

    /// Returns the formula at `id` as a value that prints it.
    pub fn formula(&self, id: NodeId) -> Formula<'_> {
        Formula { formulas: self, id }
    }

    /// Writes the formula at `id` with the spellings given, every binary
    /// subformula in brackets, and the whole in brackets too if it is
    /// binary and `brackets` is set. `atom` writes an atom of the arena,
    /// `name` a symbol's or a bound variable's name, each in the target's
    /// markup.
    pub(crate) fn write<W: Write>(
        &self,
        out: &mut W,
        id: NodeId,
        brackets: bool,
        symbols: &Symbols,
        atom: impl Fn(&mut W, u32) -> FmtResult,
        name: impl Fn(&mut W, &str) -> FmtResult,
    ) -> FmtResult {
        if self.is_first_order() {
            return self.write_first_order(out, id, brackets, symbols, &atom, &name);
        }
        for visit in Stops::new(id, brackets, |k| self.node(k).operands()) {
            match visit {
                Stop::Enter(k, nested) => match self.node(k) {
                    Node::Atom(a) => atom(out, a)?,
                    Node::True => out.write_str(symbols.truth)?,
                    Node::False => out.write_str(symbols.falsity)?,
                    Node::Not(_) => out.write_str(symbols.not)?,
                    Node::Forall(_) | Node::Exists(_) => {
                        unreachable!("an arena with a binder is first-order")
                    }
                    _ if nested => out.write_char('(')?,
                    Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => {}
                },
                Stop::Between(k) => self.between(out, k, symbols)?,
                Stop::Exit(k, nested) => {
                    if nested && self.node(k).operands().1.is_some() {
                        out.write_char(')')?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Writes the connective of the binary formula `k` with a space on
    /// either side.
    fn between<W: Write>(&self, out: &mut W, k: NodeId, symbols: &Symbols) -> FmtResult {
        out.write_char(' ')?;
        out.write_str(match self.node(k) {
            Node::And(..) => symbols.and,
            Node::Or(..) => symbols.or,
            Node::Implies(..) => symbols.implies,
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::Iff(..)
            | Node::Forall(_)
            | Node::Exists(_) => symbols.iff,
        })?;
        out.write_char(' ')
    }

    /// Writes the formula at `id` of a first-order arena as
    /// [`write`](Self::write) does. A binder's variable is written by its
    /// recorded name, or by that name with `_` and a number after it where
    /// a symbol or an enclosing binder has the name, so the text reads
    /// back as the formula. A binder inside another formula is in
    /// brackets, but the body of a binder, since a binder reaches as far
    /// right as it can.
    #[cold]
    #[inline(never)]
    fn write_first_order<W: Write>(
        &self,
        out: &mut W,
        id: NodeId,
        brackets: bool,
        symbols: &Symbols,
        atom: &impl Fn(&mut W, u32) -> FmtResult,
        name: &impl Fn(&mut W, &str) -> FmtResult,
    ) -> FmtResult {
        let tables = self.tables();
        let eigens = self.eigen_names();
        // A lookup per candidate, never a set of every symbol: the
        // printer runs once per formula, and a sequent's formulas share
        // one arena.
        let taken = |candidate: &str| self.has_symbol(candidate) || self.is_eigen_name(candidate);
        // The names of the open binders, the innermost last, and how many
        // open binders have each, so that a clash costs one lookup.
        let mut scope: Vec<String> = Vec::new();
        let mut open_names: HashMap<String, u32> = HashMap::default();
        // Per recorded name, the suffix to try first (0 the name itself):
        // every smaller one is taken. A binder's exit lowers it again, so
        // the name written is the first free one, found in constant time
        // where binders of one name nest deep.
        let mut next: HashMap<&str, u32> = HashMap::default();
        // Per open binder, its recorded name and the suffix written.
        let mut suffixes: Vec<(&str, u32)> = Vec::new();
        // Per compound formula entered, whether a bracket was opened.
        let mut opened: Vec<bool> = Vec::new();
        // Whether the next formula entered is a binder's body.
        let mut body = false;
        for visit in Stops::new(id, brackets, |k| self.node(k).operands()) {
            match visit {
                Stop::Enter(k, nested) => {
                    let in_body = std::mem::take(&mut body);
                    match self.node(k) {
                        Node::Atom(a) => self.write_atom(out, a, &scope, eigens, atom, name)?,
                        Node::True => out.write_str(symbols.truth)?,
                        Node::False => out.write_str(symbols.falsity)?,
                        Node::Not(_) => {
                            out.write_str(symbols.not)?;
                            opened.push(false);
                        }
                        Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => {
                            if nested {
                                out.write_char('(')?;
                            }
                            opened.push(nested);
                        }
                        node @ (Node::Forall(_) | Node::Exists(_)) => {
                            let open = nested && !in_body;
                            if open {
                                out.write_char('(')?;
                            }
                            opened.push(open);
                            out.write_str(if matches!(node, Node::Forall(_)) {
                                symbols.forall
                            } else {
                                symbols.exists
                            })?;
                            let recorded = tables.binder_name(k).unwrap_or("x");
                            let clashes = |candidate: &str| {
                                taken(candidate) || open_names.contains_key(candidate)
                            };
                            let candidate = |n: u32| {
                                if n == 0 {
                                    recorded.to_owned()
                                } else {
                                    format!("{recorded}_{n}")
                                }
                            };
                            let mut n = next.get(recorded).copied().unwrap_or(0);
                            let mut written = candidate(n);
                            while clashes(&written) {
                                n += 1;
                                written = candidate(n);
                            }
                            next.insert(recorded, n + 1);
                            suffixes.push((recorded, n));
                            name(out, &written)?;
                            out.write_str(symbols.dot)?;
                            *open_names.entry(written.clone()).or_insert(0) += 1;
                            scope.push(written);
                            body = true;
                        }
                    }
                }
                Stop::Between(k) => self.between(out, k, symbols)?,
                Stop::Exit(k, _) => {
                    if matches!(self.node(k), Node::Forall(_) | Node::Exists(_))
                        && let Some(name) = scope.pop()
                        && let Some((recorded, n)) = suffixes.pop()
                        && let Some(count) = open_names.get_mut(&name)
                    {
                        let first = next.entry(recorded).or_insert(n);
                        *first = (*first).min(n);
                        *count -= 1;
                        if *count == 0 {
                            open_names.remove(&name);
                        }
                    }
                    if opened.pop() == Some(true) {
                        out.write_char(')')?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Writes the atom `a` of a first-order arena: by `atom` where it has
    /// no arguments, else as its predicate applied to its terms, a bound
    /// variable by the name `scope` gives its binder, an eigenvariable by
    /// its name in `eigens` and an open witness as `$1`, `$2`, ….
    fn write_atom<W: Write>(
        &self,
        out: &mut W,
        a: u32,
        scope: &[String],
        eigens: &[(u32, String)],
        atom: &impl Fn(&mut W, u32) -> FmtResult,
        name: &impl Fn(&mut W, &str) -> FmtResult,
    ) -> FmtResult {
        let tables = self.tables();
        let Some(&(symbol, args)) = tables.atoms.get(a as usize) else {
            return atom(out, a);
        };
        let list = tables.list(args);
        if list.is_empty() {
            return atom(out, a);
        }
        let term = |out: &mut W, t: FoTermId| {
            tables.write_with(
                out,
                t,
                &mut |o, s| name(o, s),
                &mut |o, index| match scope.len().checked_sub(index as usize + 1) {
                    Some(at) => name(o, &scope[at]),
                    None => write!(o, "#{index}"),
                },
                &mut |o, other| match other {
                    FoTerm::Eigen(e) => {
                        let at = eigens
                            .binary_search_by_key(&e.get(), |&(k, _)| k)
                            .expect("every eigenvariable of the arena has its name");
                        name(o, &eigens[at].1)
                    }
                    FoTerm::Meta(k) => {
                        name(o, &crate::linear::occurrences::instances::meta_name(k))
                    }
                    FoTerm::Bound(_) | FoTerm::App(..) => Ok(()),
                },
            )
        };
        let predicate = tables.symbols[symbol.index()].0.as_str();
        if let [left, right] = *list
            && predicate == crate::first_order::EQUALS
        {
            term(out, left)?;
            out.write_str(" = ")?;
            return term(out, right);
        }
        name(out, predicate)?;
        out.write_char('(')?;
        for (i, &t) in list.iter().enumerate() {
            if i > 0 {
                out.write_str(", ")?;
            }
            term(out, t)?;
        }
        out.write_char(')')
    }
}

/// Returns the first-order tables and their maps of an arena, made empty
/// where it has none yet.
fn interner(fo: &mut Option<Box<Interner<NodeId>>>) -> &mut Interner<NodeId> {
    let fo = fo.get_or_insert_with(Box::default);
    // The names printed follow the tables, which the caller may change.
    fo.eigens.take();
    fo
}

/// Adds the nullary predicate `name` to an arena's tables, as the entry of
/// the next atom.
#[cold]
#[inline(never)]
fn nullary(fo: &mut Option<Box<Interner<NodeId>>>, name: &str) -> Result<(), Error> {
    let symbol = interner(fo)
        .symbol(name, 0, true)
        .map_err(|fault| Error::Symbol {
            name: name.to_owned(),
            fault,
        })?;
    let args = interner(fo).list(&[]);
    interner(fo).tables.atoms.push((symbol, args));
    Ok(())
}

/// How a target spells the connectives and constants of ordinary
/// formulas.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Symbols {
    /// `¬`, written directly before its operand.
    pub(crate) not: &'static str,
    /// `∧`
    pub(crate) and: &'static str,
    /// `∨`
    pub(crate) or: &'static str,
    /// `→`
    pub(crate) implies: &'static str,
    /// `↔`
    pub(crate) iff: &'static str,
    /// `⊤`
    pub(crate) truth: &'static str,
    /// `⊥`
    pub(crate) falsity: &'static str,
    /// `∀`, with what stands between it and the variable's name.
    pub(crate) forall: &'static str,
    /// `∃`, with what stands between it and the variable's name.
    pub(crate) exists: &'static str,
    /// What stands between a binder's variable and its body.
    pub(crate) dot: &'static str,
}

impl Symbols {
    /// The Unicode symbols, which `Display` writes.
    pub(crate) const UNICODE: Self = Self {
        not: "¬",
        and: "∧",
        or: "∨",
        implies: "→",
        iff: "↔",
        truth: "⊤",
        falsity: "⊥",
        forall: "∀",
        exists: "∃",
        dot: ". ",
    };
}

/// A formula of an arena, as a value that prints it with the Unicode
/// symbols, every binary subformula in brackets.
#[derive(Clone, Copy, Debug)]
pub struct Formula<'a> {
    /// The arena.
    formulas: &'a Formulas,
    /// The formula.
    id: NodeId,
}

impl Display for Formula<'_> {
    /// Writes the formula: `(a ∧ b) → ¬c`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.formulas.write(
            f,
            self.id,
            false,
            &Symbols::UNICODE,
            |o, a| o.write_str(self.formulas.atom_name(a)),
            |o, name| o.write_str(name),
        )
    }
}

/// A sequent of ordinary propositional logic, `Γ ⊢ Δ`: the hypotheses `Γ`
/// left of the turnstile, and the formulas `Δ` right of it. Classically
/// `Δ` reads as the disjunction of its formulas. In intuitionistic and
/// minimal logic `Δ` holds one goal at most, and an empty `Δ` means
/// false.
///
/// # Syntax
///
/// With the cargo feature `parse`, `"…".parse::<ordinary::Sequent>()`
/// reads a sequent written `formulas |- formulas` (or `⊢`), each side a
/// list separated by commas and possibly empty; a text without a turnstile
/// is one formula to prove. From the loosest binding to the tightest, a
/// formula is built with `<->`/`↔` (to the right), `->`/`→` (to the
/// right), `\/`/`∨`, `/\`/`∧` (these to the left) and the prefix `~`/`¬`,
/// over the constants `true`/`⊤` and `false`/`⊥`, atoms and parentheses.
/// An atom is a Unicode identifier other than a keyword: `true`, `false`,
/// the linear syntax's `par`, `top` and `bot`, and `forall` and `exists`.
/// The symbols of linear logic (`&`, `|`, `*`, `-o`, …) are not read
/// here, so that a linear connective is never taken for an ordinary one.
///
/// A first-order formula has predicates and quantifiers:
/// `forall x. p(x) -> q(x), p(a) |- q(a)`. An atom may be a predicate
/// applied to terms, `p(x, f(a))`, with no space before the bracket; a
/// term is a variable, a constant or a function symbol applied to terms.
/// `forall x y. A` (or `∀x y. A`) and `exists x. A` (or `∃x. A`) bind
/// loosest and reach as far right as they can, so `forall x. p(x) ->
/// q(x)` is `∀x. (p(x) → q(x))`. A name that a quantifier around it binds
/// is that variable; any other name in a term is a constant or a function
/// symbol. A name has one arity and is a predicate or a function symbol,
/// not both. Every formula is closed. `s = t` says two terms are equal,
/// and `s != t` (or `s ≠ t`) is `¬(s = t)`; a sequent that uses `=` is
/// decided with the equality axioms ([`equality::with_axioms`]).
///
/// `Display` writes the Unicode symbols with every nested binary formula
/// in brackets, which this syntax reads back.
///
/// # JSON
///
/// With the cargo feature `serialize` a sequent is `{"version": 8,
/// "formulas": [{"V": 0}, {"V": 1}, {"→": [0, 1]}], "atoms": ["a", "b"],
/// "left": [0], "right": [1]}` for `a ⊢ b` with the arena `a, b, a → b`
/// ([`wire`](crate::wire)): the formulas in the arena's order, every
/// operand before the formula built from it, as `{"V": atom}`, `"⊤"`,
/// `"⊥"`, `{"¬": a}`, `{"∧": [a, b]}`, `{"∨": [a, b]}`, `{"→": [a, b]}`
/// and `{"↔": [a, b]}`, and the two sides as indices into them.
///
/// A first-order sequent is written at level 25 with the keys of a
/// first-order linear sequent ([`linear::Sequent`](crate::linear::Sequent)):
/// `symbols`, `fo` (the terms), the atoms as `{"P": [symbol, [terms]]}`,
/// and `binders`, the name of each binder's variable in the order of the
/// binders among the formulas, which are `{"∀": a}` and `{"∃": a}`.
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// let sequent: deduxit::ordinary::Sequent = "a /\\ b -> c |- ~c -> ~a \\/ ~b".parse()?;
/// assert_eq!(sequent.to_string(), "(a ∧ b) → c ⊢ ¬c → (¬a ∨ ¬b)");
/// # Ok::<(), deduxit::Error>(())
/// ```
#[derive(Clone, Debug)]
pub struct Sequent {
    /// The formulas.
    formulas: Formulas,
    /// The hypotheses, in the order written.
    left: Vec<NodeId>,
    /// The formulas right of the turnstile, in the order written.
    right: Vec<NodeId>,
}

impl Sequent {
    /// Returns the sequent `left ⊢ right` over the formulas given.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for an id of the two lists that is no
    /// node of `formulas`, and [`Error::LooseVariable`] for a formula with
    /// a bound variable that no binder of it binds.
    pub fn new(formulas: Formulas, left: Vec<NodeId>, right: Vec<NodeId>) -> Result<Self, Error> {
        for &id in left.iter().chain(&right) {
            formulas.holds(id)?;
        }
        if formulas.is_first_order() {
            closed(&formulas, left.iter().chain(&right))?;
        }
        Ok(Self {
            formulas,
            left,
            right,
        })
    }

    /// Returns the arena of the formulas.
    pub fn formulas(&self) -> &Formulas {
        &self.formulas
    }

    /// Returns the formulas left of the turnstile, in the order written.
    pub fn left(&self) -> &[NodeId] {
        &self.left
    }

    /// Returns the formulas right of the turnstile, in the order written.
    pub fn right(&self) -> &[NodeId] {
        &self.right
    }
}

impl Display for Sequent {
    /// Writes `Γ ⊢ Δ` with the Unicode symbols.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let sides = (&self.left[..], &self.right[..]);
        TEXT.ordinary(f, &self.formulas, sides, false, false)
    }
}

/// Fails with [`Error::LooseVariable`] at the first of `roots` that has a
/// bound variable no binder of it binds.
#[cold]
#[inline(never)]
pub(crate) fn closed<'a>(
    formulas: &Formulas,
    roots: impl Iterator<Item = &'a NodeId>,
) -> Result<(), Error> {
    let loose = formulas.loose();
    for id in roots {
        if loose[id.index()] > 0 {
            return Err(Error::LooseVariable { term: id.index() });
        }
    }
    Ok(())
}

/// Checks that a sequent over the formulas unfolds to at most
/// `limits.occurrences` subformula occurrences.
pub(crate) fn bound(
    formulas: &Formulas,
    left: &[NodeId],
    right: &[NodeId],
    limits: &crate::Limits,
) -> Result<(), Error> {
    formulas.arena.bound(left, right, limits)
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::limits::{Refusal, Space};
    use crate::linear::search::generate::Rng;
    use crate::linear::{Options as Search, Verdict, prove};

    /// Decides `text` in `logic` by `translation` with a copy bound of
    /// eight, and returns whether it was proved, with the derivation read
    /// back checked.
    fn decide(text: &str, logic: Logic, translation: Translation) -> Option<bool> {
        let sequent: Sequent = text.parse().unwrap();
        let image = translate(&sequent, logic, translation, &crate::Limits::default()).unwrap();
        let search = Search::default().with_copies(Some(8));
        let outcome = prove(image.sequent(), image.mode(), &search).unwrap();
        match &outcome.verdict {
            Verdict::Proved(proof) => {
                let unbounded = crate::Limits::default();
                let derivation = image.read_back(proof, &unbounded, |_| false).unwrap();
                derivation
                    .check(&unbounded, |_| false)
                    .unwrap_or_else(|e| panic!("{text} ({translation}): {e}"));
                Some(true)
            }
            Verdict::Unprovable(_) => Some(false),
            Verdict::Unknown(_) => None,
        }
    }

    /// The default's test of whether grounding decides a sequent ends at
    /// its bound on work: a congruence axiom of a predicate of 500
    /// arguments lies in the Bernays–Schönfinkel class, but its normal
    /// form is a nest of a thousand quantifiers whose making is quadratic,
    /// past the bound, so the default takes another method; a sequent of
    /// the class of ordinary size still grounds.
    #[cfg(feature = "sat")]
    #[test]
    fn the_grounding_test_ends_at_its_work_bound() {
        let small: Sequent = "forall x. (p(x) -> q(x)), p(a) |- q(a)".parse().unwrap();
        assert!(grounds(&small));
        let n = 500;
        let xs: Vec<String> = (0..n).map(|i| format!("x{i}")).collect();
        let ys: Vec<String> = (0..n).map(|i| format!("y{i}")).collect();
        let equal: Vec<String> = (0..n).map(|i| format!("e(x{i}, y{i})")).collect();
        let text = format!(
            "forall {} {}. ({} /\\ p({}) -> p({})) |- r",
            xs.join(" "),
            ys.join(" "),
            equal.join(" /\\ "),
            xs.join(", "),
            ys.join(", "),
        );
        let wide: Sequent = text.parse().unwrap();
        assert!(!grounds(&wide));
        let unbounded = crate::Limits::default().with_occurrences(Some(GROUNDS_NODES));
        assert!(grounding::applies(&wide, &unbounded).unwrap());
    }

    /// Every logic and translation reads back from its name, an unknown
    /// name is refused naming the known ones, and the readers and the
    /// translation keep to the occurrence bound.
    #[test]
    fn names_read_back_and_bounds_hold() {
        // Every enum read by `crate::name::named` lists in `NAMES` the
        // names of `ALL`, in order.
        fn listed<T: Copy>(all: &[T], name: impl Fn(T) -> &'static str, names: &[&str]) {
            assert_eq!(all.iter().map(|&t| name(t)).collect::<Vec<_>>(), names);
        }
        listed(Logic::ALL, Logic::name, Logic::NAMES);
        listed(Translation::ALL, Translation::name, Translation::NAMES);
        listed(Method::ALL, Method::name, Method::NAMES);
        listed(normal::Form::ALL, normal::Form::name, normal::Form::NAMES);
        listed(
            negative::Translation::ALL,
            negative::Translation::name,
            negative::Translation::NAMES,
        );
        listed(
            resolution::Method::ALL,
            resolution::Method::name,
            resolution::Method::NAMES,
        );
        listed(
            resolution::Equality::ALL,
            resolution::Equality::name,
            resolution::Equality::NAMES,
        );
        listed(
            crate::modal::Condition::ALL,
            crate::modal::Condition::name,
            crate::modal::Condition::NAMES,
        );
        listed(
            crate::lambda::Strategy::ALL,
            crate::lambda::Strategy::name,
            crate::lambda::Strategy::NAMES,
        );
        for &logic in Logic::ALL {
            assert_eq!(logic.name().parse::<Logic>().unwrap(), logic);
        }
        for &translation in Translation::ALL {
            assert_eq!(
                translation.name().parse::<Translation>().unwrap(),
                translation
            );
        }
        assert!(matches!(
            "cbz".parse::<Translation>(),
            Err(Error::UnknownName { known, .. }) if known == Translation::NAMES
        ));
        let bound = |n| crate::Limits::default().with_occurrences(Some(n));
        let refused = |result: Result<(), Error>| {
            matches!(result, Err(Error::Refused(Refusal::Occurrences { .. })))
        };
        // Five formulas: a, b, c, a ∧ b and the implication.
        assert!(Sequent::parse_within("a /\\ b -> c", &bound(5)).is_ok());
        assert!(refused(
            Sequent::parse_within("a /\\ b -> c", &bound(4)).map(|_| ())
        ));
        let tptp = "fof(c, conjecture, (a & b) => c).";
        assert!(read_tptp(tptp, &bound(5)).is_ok());
        assert!(refused(read_tptp(tptp, &bound(4)).map(|_| ())));
        // One atom repeated: one formula of the arena, but each repeat is
        // an occurrence of the image.
        let repeated = ["a"; 6].join(", ");
        assert!(Sequent::parse_within(&repeated, &bound(6)).is_ok());
        assert!(refused(
            Sequent::parse_within(&repeated, &bound(5)).map(|_| ())
        ));
        // A parenthesis waits but makes no formula: `~(~(a))` is three
        // formulas and read at a bound of three.
        assert!(Sequent::parse_within("~(~(a))", &bound(3)).is_ok());
        // What waits for its operand counts too: a run of `~` or of `->`
        // is refused before the parse error after it.
        for run in ["~".repeat(100), "a -> ".repeat(100), "(".repeat(100)] {
            assert!(refused(
                Sequent::parse_within(&format!("{run}a b"), &bound(10)).map(|_| ())
            ));
        }
        let hypotheses = "fof(h, axiom, a).\n".repeat(5) + "fof(c, conjecture, a).";
        assert!(read_tptp(&hypotheses, &bound(6)).is_ok());
        assert!(refused(read_tptp(&hypotheses, &bound(5)).map(|_| ())));
        let sequent: Sequent = "a /\\ b -> c".parse().unwrap();
        let image = translate(
            &sequent,
            Logic::Classical,
            Translation::Affine,
            &bound(1_000),
        )
        .unwrap();
        let size = image.sequent().occurrences();
        let classical = |n| translate(&sequent, Logic::Classical, Translation::Affine, &bound(n));
        assert!(classical(size).is_ok());
        assert!(refused(classical(size - 1).map(|_| ())));
    }

    /// A name is one atom however its accents are encoded, and a keyword
    /// of either syntax names none.
    #[test]
    fn atom_names_are_composed_identifiers() {
        let mut formulas = Formulas::default();
        let composed = formulas.atom("\u{e9}").unwrap();
        assert_eq!(formulas.atom("e\u{301}").unwrap(), composed);
        assert_eq!(formulas.atom_names(), ["\u{e9}"]);
        for name in ["top", "true", "forall", "a b"] {
            assert!(
                matches!(formulas.atom(name), Err(Error::AtomName { .. })),
                "{name}"
            );
        }
    }

    /// An arena refuses a subformula of another arena and an atom
    /// outside its names, and a sequent an id outside its arena.
    #[test]
    fn foreign_ids_are_refused() {
        let mut other = Formulas::default();
        let (a, b) = (other.atom("a").unwrap(), other.atom("b").unwrap());
        let and = other.add(Node::And(a, b)).unwrap();
        let mut formulas = Formulas::default();
        let x = formulas.atom("x").unwrap();
        let refused = |result: Result<_, Error>, space| matches!(result, Err(Error::IndexOutOfBounds { space: s, .. }) if s == space);
        assert!(refused(formulas.add(Node::Not(and)), Space::Formula));
        assert!(refused(formulas.add(Node::Atom(5)), Space::Atom));
        let sequent = Sequent::new(formulas.clone(), vec![x], vec![and]);
        assert!(refused(sequent.map(|_| x), Space::Formula));
        assert!(Sequent::new(formulas, vec![], vec![x]).is_ok());
    }

    /// A TPTP problem's atom `false`, which the ordinary syntax reads as
    /// the constant and minimal logic's false is the atom of, is renamed:
    /// it never stands for minimal logic's false.
    #[test]
    fn false_is_no_atom_of_the_sequent() {
        let read = read_tptp(
            "fof(a, axiom, p). fof(b, axiom, ~p). fof(c, conjecture, false).",
            &crate::Limits::default(),
        )
        .unwrap();
        let names = read.sequent.formulas().atom_names();
        assert!(names.iter().all(|n| n != FALSE), "{names:?}");
        assert_eq!(read.sequent.to_string(), "p, ¬p ⊢ false_");
    }

    /// Every translation decides the sequents of its logic as the logic
    /// does, and every proof reads back as a derivation its checker
    /// accepts: the connectives on both sides, nested negations, the
    /// constants, contraction, and the sequents that separate the logics.
    #[test]
    fn decides_and_reads_back() {
        // (sequent, classical, intuitionistic, minimal)
        let cases = [
            ("a -> a", true, true, true),
            ("a \\/ ~a", true, false, false),
            ("~~a -> a", true, false, false),
            ("a -> ~~a", true, true, true),
            ("((a -> b) -> a) -> a", true, false, false),
            ("false -> a", true, true, false),
            ("~a -> a -> b", true, true, false),
            ("~a -> a -> ~b", true, true, true),
            ("a /\\ b <-> b /\\ a", true, true, true),
            ("a \\/ b, ~a |- b", true, true, false),
            ("(a -> b) /\\ (a -> c) |- a -> b /\\ c", true, true, true),
            ("a -> b, b -> c |- a -> c", true, true, true),
            ("~(a \\/ b) <-> ~a /\\ ~b", true, true, true),
            ("~(a /\\ b) -> ~a \\/ ~b", true, false, false),
            ("true, a |- a /\\ true", true, true, true),
            ("a -> (a -> b) -> b", true, true, true),
            ("(a <-> b) -> (b <-> a)", true, true, true),
            ("a |- b", false, false, false),
            ("a \\/ b |- a /\\ b", false, false, false),
            ("~~(a \\/ ~a)", true, true, true),
            // Minimal logic's false is an atom, copied like any other.
            ("false |- ~true", true, true, true),
            ("false |- a -> a", true, true, true),
            ("q |- r -> ((false \\/ q) -> r)", true, true, true),
        ];
        for (text, classical, intuitionistic, minimal) in cases {
            assert_eq!(
                decide(text, Logic::Classical, Translation::Affine),
                Some(classical),
                "{text} classically"
            );
            for translation in [
                Translation::CallByName,
                Translation::CallByValue,
                Translation::ZeroOne,
            ] {
                for (logic, valid) in [
                    (Logic::Intuitionistic, intuitionistic),
                    (Logic::Minimal, minimal),
                ] {
                    let verdict = decide(text, logic, translation);
                    assert!(
                        verdict.is_none() || verdict == Some(valid),
                        "{text} in {logic} logic by {translation}: {verdict:?}"
                    );
                    if valid {
                        assert_eq!(
                            verdict,
                            Some(true),
                            "{text} in {logic} logic by {translation}"
                        );
                    }
                }
            }
        }
    }

    /// Returns a formula of depth `depth` at most over `atoms`, true and
    /// false, built with `¬`, `∧`, `∨` and `→`.
    fn generated(rng: &mut Rng, formulas: &mut Formulas, atoms: &[NodeId], depth: usize) -> NodeId {
        if depth == 0 || rng.one_in(4) {
            // Each atom twice as likely as each constant.
            let leaf = rng.below(2 * atoms.len() + 2);
            let node = match leaf.checked_sub(2 * atoms.len()) {
                None => return atoms[leaf % atoms.len()],
                Some(0) => Node::True,
                Some(_) => Node::False,
            };
            return formulas.add(node).unwrap();
        }
        let a = generated(rng, formulas, atoms, depth - 1);
        let node = match rng.below(4) {
            0 => Node::Not(a),
            choice => {
                let b = generated(rng, formulas, atoms, depth - 1);
                match choice {
                    1 => Node::And(a, b),
                    2 => Node::Or(a, b),
                    _ => Node::Implies(a, b),
                }
            }
        };
        formulas.add(node).unwrap()
    }

    /// Returns a sequent in the atoms `a`, `b` and `c` of up to two
    /// hypotheses and at most one formula right of `⊢`, each of depth
    /// three at most.
    pub(super) fn generated_sequent(rng: &mut Rng) -> Sequent {
        let mut formulas = Formulas::default();
        let atoms: Vec<_> = ["a", "b", "c"]
            .into_iter()
            .map(|name| formulas.atom(name).unwrap())
            .collect();
        let left = (0..rng.below(3))
            .map(|_| generated(rng, &mut formulas, &atoms, 3))
            .collect();
        let right = (0..usize::from(!rng.one_in(6)))
            .map(|_| generated(rng, &mut formulas, &atoms, 3))
            .collect();
        Sequent::new(formulas, left, right).unwrap()
    }

    /// Returns whether every assignment of the sequent's atoms that makes
    /// all its hypotheses true makes some formula right of `⊢` true.
    pub(super) fn valid_by_truth_table(sequent: &Sequent) -> bool {
        let formulas = sequent.formulas();
        (0..1_u32 << formulas.atom_names().len()).all(|assignment| {
            // Every node after its operands, so one pass evaluates all.
            let mut value: Vec<bool> = Vec::with_capacity(formulas.len());
            for n in 0..formulas.len() {
                let of = |id: NodeId| value[id.index()];
                let holds = match formulas.node(NodeId(n as u32)) {
                    Node::Atom(atom) => assignment >> atom & 1 == 1,
                    Node::True => true,
                    Node::False => false,
                    Node::Not(a) => !of(a),
                    Node::And(a, b) => of(a) && of(b),
                    Node::Or(a, b) => of(a) || of(b),
                    Node::Implies(a, b) => !of(a) || of(b),
                    Node::Iff(a, b) => of(a) == of(b),
                    Node::Forall(_) | Node::Exists(_) => {
                        unreachable!("the samples are propositional")
                    }
                };
                value.push(holds);
            }
            !sequent.left().iter().all(|id| value[id.index()])
                || sequent.right().iter().any(|id| value[id.index()])
        })
    }

    /// On a few hundred generated sequents, each logic's translations
    /// agree: wherever two of them decide a sequent they give it the same
    /// verdict, and each decides at least half of the sequents. Classical
    /// logic's verdicts are the truth table's, and a sequent valid in
    /// intuitionistic or minimal logic is valid by the truth table.
    #[test]
    fn translations_agree_with_each_other_and_the_truth_table() {
        const SEQUENTS: usize = 300;
        // Bounds that keep the run to a second or two unoptimized.
        let search = Search::default().with_copies(Some(6));
        let limits = crate::Limits::default().with_work(Some(50_000));
        let mut rng = Rng::new(2_026);
        let mut decided: HashMap<(Logic, Translation), usize> = HashMap::default();
        for _ in 0..SEQUENTS {
            let sequent = generated_sequent(&mut rng);
            let classical = valid_by_truth_table(&sequent);
            for &logic in Logic::ALL {
                let verdicts: Vec<_> = Translation::ALL
                    .iter()
                    .filter(|translation| translation.decides(logic))
                    .map(|&translation| {
                        let options = Options::default()
                            .with_logic(logic)
                            .with_translation(Some(translation));
                        let outcome =
                            super::decide_within(&sequent, &options, &search, &limits, |_| false)
                                .unwrap_or_else(|e| {
                                    panic!("{sequent} in {logic} logic by {translation}: {e}")
                                });
                        let valid = match outcome.verdict {
                            super::Verdict::Valid(_) => Some(true),
                            super::Verdict::NotValid => Some(false),
                            super::Verdict::Unknown(_) => None,
                        };
                        if valid.is_some() {
                            *decided.entry((logic, translation)).or_default() += 1;
                        }
                        (translation, valid)
                    })
                    .collect();
                let answers: Vec<bool> = verdicts.iter().filter_map(|&(_, valid)| valid).collect();
                assert!(
                    answers.windows(2).all(|pair| pair[0] == pair[1]),
                    "{sequent} in {logic} logic: {verdicts:?}"
                );
                if logic == Logic::Classical {
                    assert!(
                        answers.iter().all(|&valid| valid == classical),
                        "{sequent} classically: {verdicts:?}, the truth table says {classical}"
                    );
                } else {
                    assert!(
                        classical || !answers.contains(&true),
                        "{sequent} in {logic} logic: {verdicts:?}, the truth table refutes it"
                    );
                }
            }
        }
        for &logic in Logic::ALL {
            for &translation in Translation::ALL.iter().filter(|t| t.decides(logic)) {
                let count = decided.get(&(logic, translation)).copied().unwrap_or(0);
                assert!(
                    2 * count >= SEQUENTS,
                    "{translation} decided {count} of {SEQUENTS} sequents in {logic} logic"
                );
            }
        }
    }
}

#[cfg(test)]
mod first_order_tests {
    use super::*;
    use crate::Limits;
    use crate::linear::proofs::InfId;

    /// Returns `∀x. p(x) ⊢ ∀x. p(x)` with the axiom that derives it.
    fn quantified() -> (Sequent, Derivation) {
        let mut formulas = Formulas::default();
        let x = formulas.variable(0);
        let body = formulas.predicate("p", &[x]).unwrap();
        let all = formulas.binder(true, body, "x").unwrap();
        let sequent = Sequent::new(formulas.clone(), vec![all], vec![all]).unwrap();
        let axiom = Inference {
            term: None,
            left: vec![all],
            right: vec![all],
            rule: Rule::Axiom,
            principal: None,
            premises: Vec::<InfId>::new(),
        };
        let derivation = Derivation::from_parts(
            Logic::Classical,
            formulas,
            (vec![all], vec![all]),
            vec![axiom],
        );
        (sequent, derivation)
    }

    /// Asserts that `result` is the refusal of a first-order sequent.
    #[track_caller]
    fn refused<T: std::fmt::Debug>(result: Result<T, Error>) {
        match result {
            Err(Error::FirstOrder { .. }) => {}
            other => panic!("expected the first-order refusal, got {other:?}"),
        }
    }

    #[test]
    fn every_propositional_method_refuses_a_first_order_sequent() {
        let (sequent, derivation) = quantified();
        let limits = Limits::default();
        for &logic in Logic::ALL {
            // The embedding decides it, and is the default there.
            for method in [None, Some(Method::Embedding)] {
                let options = Options::default().with_logic(logic).with_method(method);
                let outcome = decide(&sequent, &options).unwrap();
                assert!(matches!(outcome.verdict, Verdict::Valid(_)), "{logic:?}");
            }
            for &method in Method::ALL {
                // The connection prover decides classical first-order
                // sequents and refuses the other logics by name; grounding
                // decides those of its classes.
                if super::decide::takes_first_order(method) {
                    continue;
                }
                let options = Options::default()
                    .with_logic(logic)
                    .with_method(Some(method));
                refused(decide(&sequent, &options));
            }
            refused(ljt::decide(&sequent, logic, &limits, |_| false));
            #[cfg(feature = "sat")]
            {
                refused(sat::decide(&sequent, logic, &limits, |_| false));
                refused(sat::decide_preferring_ljt(&sequent, logic, &limits, |_| {
                    false
                }));
            }
            refused(
                crate::kripke::Model::new(vec![crate::kripke::World::default()], 0).forcing(
                    &sequent,
                    logic,
                    &limits,
                    |_| false,
                ),
            );
        }
        refused(truth_table(&sequent, &limits, |_| false));
        refused(normal::rewrite(
            &sequent,
            &normal::Options::default(),
            &limits,
            |_| false,
        ));
        refused(resolution::Clauses::of_sequent(&sequent));
        // Glivenko's theorem fails at first order, which its refusal says;
        // the other translations take first-order sequents.
        assert!(matches!(
            negative::translate(
                &sequent,
                &negative::Options::default().with_translation(negative::Translation::Glivenko),
                &limits,
            ),
            Err(Error::GlivenkoFirstOrder)
        ));
        refused(derivation.term(&limits, |_| false));
        refused(crate::lambda::enumerate(
            &sequent,
            &crate::lambda::InhabitOptions::default(),
            &limits,
            |_| false,
        ));
        // A normal form's public fields can be given a first-order sequent.
        let mut normal = normal::rewrite(
            &Sequent::new(Formulas::default(), vec![], vec![]).unwrap(),
            &normal::Options::default(),
            &limits,
            |_| false,
        )
        .unwrap();
        normal.sequent = sequent.clone();
        refused(normal.claims());
        #[cfg(feature = "interactive")]
        refused(Tableau::of(&derivation, |_| false));
        #[cfg(feature = "rocq")]
        refused(crate::export::rocq::write(
            &derivation,
            &crate::export::rocq::Options::default(),
            &mut String::new(),
            |_| false,
        ));
        #[cfg(feature = "serialize")]
        refused(crate::rules::Derivation::from_ordinary(&derivation));
        #[cfg(all(feature = "interactive", feature = "parse", feature = "serialize"))]
        {
            let tableau = WrittenTableau::parse_within("1 F p\n", &limits).unwrap();
            let mut json = serde_json::Deserializer::from_str(crate::rules::TABLEAU);
            let calculus =
                crate::wire::upgrade::<crate::rules::Calculus, _>(&mut json, &limits).unwrap();
            refused(tableau.review(&calculus, Some(&sequent), &limits, |_| false));
        }
    }

    #[test]
    fn a_first_order_formula_prints_with_its_binders_renamed_apart() {
        let mut formulas = Formulas::default();
        // `∀y. ∃x. r(y, x)` and `∃x. ∃z. r(x, z)` share `∃x. r(#1, #0)`.
        let (outer, inner) = (formulas.variable(1), formulas.variable(0));
        let r = formulas.predicate("r", &[outer, inner]).unwrap();
        let some = formulas.binder(false, r, "x").unwrap();
        let first = formulas.binder(true, some, "y").unwrap();
        let second = formulas.binder(false, some, "x").unwrap();
        let a = formulas.function("a", &[]).unwrap();
        let p = formulas.predicate("p", &[a]).unwrap();
        let both = formulas.add(Node::And(first, p)).unwrap();
        let sequent = Sequent::new(formulas, vec![both], vec![second]).unwrap();
        assert_eq!(
            sequent.to_string(),
            "(∀y. ∃x. r(y, x)) ∧ p(a) ⊢ ∃x. ∃x_1. r(x, x_1)"
        );
    }

    #[test]
    fn a_formula_with_a_loose_variable_is_no_formula_of_a_sequent() {
        let mut formulas = Formulas::default();
        let x = formulas.variable(0);
        let body = formulas.predicate("p", &[x]).unwrap();
        assert!(matches!(
            Sequent::new(formulas, vec![], vec![body]),
            Err(Error::LooseVariable { .. })
        ));
    }

    /// Reads `json` as a document of the form `T`.
    #[cfg(feature = "serialize")]
    fn read<T: crate::wire::Readable>(json: &str) -> Result<T, Error> {
        let mut json = serde_json::Deserializer::from_str(json);
        crate::wire::upgrade::<T, _>(&mut json, &Limits::default())
    }

    /// The readers hold a document to what the arena promises: closed
    /// formulas in a derivation, each term once, no first-order tableau,
    /// and a document of nullary predicates propositional.
    #[test]
    #[cfg(feature = "serialize")]
    fn documents_are_held_to_the_arena_s_promises() {
        let loose = r#"{"version":25,"logic":"classical","sequent":{"formulas":[{"V":0}],"atoms":[{"P":[0,[0]]}],"symbols":[["p",1]],"fo":[{"B":0}],"left":[0],"right":[0]},"inferences":[{"left":[0],"right":[0],"rule":"ax"}]}"#;
        assert!(matches!(
            read::<Derivation>(loose),
            Err(Error::LooseVariable { .. })
        ));
        // `a` twice: an instance is found however the document names it.
        let twice = r#"{"version":25,"formulas":[{"V":0},{"∀":0},{"V":1}],"atoms":[{"P":[1,[2]]},{"P":[1,[1]]}],"symbols":[["a",0],["p",1]],"fo":[{"F":[0,[]]},{"F":[0,[]]},{"B":0}],"binders":["x"],"left":[2],"right":[1]}"#;
        let sequent: Sequent = read(twice).unwrap();
        let formulas = sequent.formulas();
        let Node::Forall(body) = formulas.node(sequent.right()[0]) else {
            panic!("a binder")
        };
        let a = formulas
            .tables()
            .terms
            .iter()
            .position(|t| matches!(t, crate::first_order::FoTerm::App(..)));
        let a = crate::first_order::FoTermId::new(a.unwrap() as u32);
        assert_eq!(formulas.instance(body, a), Some(sequent.left()[0]));
        #[cfg(feature = "interactive")]
        refused(read::<Tableau>(
            r#"{"version":25,"sequent":{"formulas":[{"V":0},{"∀":0}],"atoms":["q"],"binders":["x"],"left":[1],"right":[1]},"nodes":[{"lines":[["T",1],["F",1]],"end":{"kind":"closed","true_line":0,"false_line":1}}]}"#,
        ));
        let nullary = r#"{"version":25,"formulas":[{"V":0}],"atoms":[{"P":[0,[]]}],"symbols":[["p",0]],"left":[],"right":[0]}"#;
        let sequent: Sequent = read(nullary).unwrap();
        assert!(!sequent.formulas().is_first_order());
        assert_eq!(sequent.to_string(), "⊢ p");
    }

    #[test]
    fn a_name_has_one_role() {
        let mut formulas = Formulas::default();
        let a = formulas.function("a", &[]).unwrap();
        formulas.predicate("p", &[a]).unwrap();
        // `a` names a constant, so it names no atom.
        assert!(matches!(formulas.atom("a"), Err(Error::Symbol { .. })));
        assert!(matches!(
            formulas.predicate("p", &[a, a]),
            Err(Error::Symbol { .. })
        ));
    }
}
