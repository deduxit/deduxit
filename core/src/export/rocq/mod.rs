// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Writes a proof as a Rocq certificate: a lemma that states the sequent,
//! with a proof that Rocq checks.
//!
//! A certificate lets you trust a proof without trusting deduxit: Rocq
//! compiles the file only if the proof is correct.
//!
//! | to get | call |
//! |---|---|
//! | the certificate of a proof | [`to_string`] |
//! | the same, written into your own buffer, with a stop | [`write()`] |
//! | a certificate that a sequent has no proof | [`write_disproof`] |
//! | a certificate that an ordinary sequent is not valid | [`write_not_valid`] |
//!
//! Each takes an [`Options`]: start from its `default()` and set the
//! fields you want to change. By default a certificate is the lemma
//! `certificate` alone ([`Form::Fragment`]), for a file that has the
//! imports already. [`Form::Standalone`] writes a whole file, which starts
//! with the imports its kernel needs ([`Options::prelude`]).
//!
//! Needs the cargo feature `rocq` (on by default).
//!
//! # Examples
//!
//! A proof of classical linear logic, certified for NanoYalla:
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::export::{Form, rocq};
//! use deduxit::linear::{Mode, Options, Sequent, Verdict, prove};
//!
//! let sequent: Sequent = "A, A -o B |- B".parse()?;
//! let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
//! let Verdict::Proved(proof) = &outcome.verdict else {
//!     panic!("provable");
//! };
//! let certificate = rocq::to_string(&proof.derivation()?, &rocq::Options::default())?;
//! assert_eq!(
//!     certificate,
//!     "Lemma certificate (A B : formula) : ll [dual A; tens A (dual B); B].
//! Proof.
//! apply (tens_r_ext [dual A]); cbn_sequent.
//! {
//!   ax_expansion.
//! }
//! {
//!   ax_expansion.
//! }
//! Qed."
//! );
//!
//! // An intuitionistic proof goes to deduxit's library. As a whole file, it
//! // starts with the library's imports.
//! let outcome = prove(&sequent, Mode::INTUITIONISTIC, &Options::default())?;
//! let Verdict::Proved(proof) = &outcome.verdict else {
//!     panic!("provable");
//! };
//! let options = rocq::Options::default().with_form(Form::Standalone);
//! let file = rocq::to_string(&proof.derivation()?, &options)?;
//! assert!(file.starts_with("From Stdlib Require Import List String.\nFrom Deduxit.Linear"));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Which kernel a certificate needs
//!
//! A certificate of linear logic is written for one of two kernels, the
//! Rocq libraries that define the calculus ([`Kernel`]):
//! - NanoYalla, version [`NANOYALLA`], the kernel of the Click & coLLecT
//!   prover: classical linear logic, one-sided, without cut. A whole file
//!   imports it with `From NanoYalla Require Import macroll.`
//! - deduxit's own Rocq library `rocq-deduxit`, version [`DEDUXIT`]
//!   (<https://github.com/deduxit/rocq-deduxit>): linear logic one-sided,
//!   with Mix and weakening where the mode allows them, and
//!   intuitionistic linear logic two-sided, each with cut and quantifiers.
//!   A whole file imports it with lines that start `From Deduxit.Linear`.
//!
//! [`Options::kernel`] names the kernel. Without it, the proof decides:
//! - deduxit's library takes a proof of an intuitionistic, affine or Mix
//!   mode, a proof that uses Mix, affine weakening, a cut or a quantifier,
//!   and a two-sided proof that records no mode;
//! - NanoYalla takes every other proof, one of classical linear logic.
//!
//! A certificate that a sequent has no proof always needs deduxit's
//! library. A certificate of ordinary logic (LK or LJ) is stated over
//! Rocq's `Prop` and needs no kernel; a classical one uses the excluded
//! middle of Rocq's standard library.
//!
//! Neither kernel takes an open goal, a compact derivation (one inference
//! for a run of a structural rule), or a proof of the cyclic mode, since
//! their sequents are unordered. Such a proof has no certificate
//! ([`Unsupported`]).
//!
//! # Checking a certificate
//!
//! Write the certificate as a whole file and run deduxit's app for it,
//! which needs only Nix with flakes:
//!
//! ```text
//! nix run github:deduxit/deduxit#check-certificate -- proof.v
//! ```
//!
//! It loads Rocq's standard library and both kernels, and the file's own
//! import lines pick the kernel. It exits with 0 when every lemma is
//! proved without an axiom (a classical certificate over `Prop` may use
//! the excluded middle), 1 when Rocq refuses the file or a lemma rests on
//! another axiom, and 2 when the file cannot be read.
//!
//! Without Nix, compile the whole file with Rocq 9 and its standard
//! library, with the kernel's library on Rocq's load path.
//!
//! # NanoYalla's script
//!
//! The lemma states the sequent as a list of the kernel's formulas, over
//! the atoms as variables of type `formula`:
//! `ll [dual A; tens A (dual B); B]`. Its proof applies one derived rule
//! of the kernel's `macroll` module per inference, conclusion first:
//! `tens_r_ext`, `parr_r_ext`, `oc_r_ext`, `co_r_ext` and so on, and
//! `ax_expansion` for an axiom. `Qed` closes it, so Rocq accepts the file
//! only if the kernel accepts the proof.
//!
//! The kernel's sequents are lists, and its rules act on the formula at a
//! given position. So the script keeps the list of every goal. The lemma
//! states the conclusion in occurrence order, and each rule takes the
//! formulas before its principal formula. Only a `⊗` needs its goal in a
//! given order: its left premise's formulas before the `⊗` and its right
//! premise's after it. Where the goal is not in that order, an exchange
//! `ex_perm_r` comes first.
//!
//! The certificate is one of classical linear logic. A two-sided
//! derivation is written as the one-sided derivation behind it (the
//! one-sided [`Rule`] of every [`Named`](crate::linear::proofs::Named)
//! rule), which proves the same sequent read one-sided. The kernel has no
//! rule for Mix, the weakening of affine mode, a cut or a quantifier, so
//! [`Options::kernel`] set to NanoYalla refuses a derivation with one.
//!
//! An atom's name becomes a Rocq identifier. Letters, digits, `_` and `'`
//! stay. Any other character becomes its code point in hexadecimal between
//! underscores, and a leading digit or `'` gets a `_` before it. A name
//! that clashes with a keyword, a name of the kernel, the lemma's name or
//! another atom gets `'` appended; later clashes of the same identifier
//! get `'2`, `'3` and so on.
//!
//! # deduxit's proof term
//!
//! The lemma states the sequent over the library's own calculus: `ll`
//! one-sided, or `ill` two-sided for a two-sided derivation, with the
//! rules the mode allows and the proof uses. Its proof applies the
//! library's soundness theorem to the derivation written as a proof term,
//! and Rocq's `vm_compute` runs the library's checker on that term. Atoms
//! are Rocq strings, so no atom is renamed. Every rule has a term.

use super::notation::{Step, flush, walk};
use super::{Drawable, Form};
use crate::hash::{HashMap, HashSet};
use crate::linear::occurrences::{Forest, OccId};
use crate::linear::proofs::{Derivation, InfId, Rule};
use crate::linear::sequents::{Sequent, Stop, Stops, Term, TermId};
use crate::{Error, linear::Mode};
use std::fmt::Write;
use thiserror::Error;

mod deduxit;

/// The version of NanoYalla the scripts are written for. NanoYalla is the
/// `nanoyalla` directory of Click & coLLecT, whose `macroll` module
/// defines the derived rules the scripts apply. It builds with Rocq 9 and
/// its standard library, without Yalla.
pub const NANOYALLA: &str = "1.1.3";

/// The options of a certificate. Start from `default()` and set the
/// fields that differ.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Options {
    /// The lemma alone, or a whole file that starts with the prelude.
    pub form: Form,
    /// The name of the lemma.
    pub lemma: Identifier,
    /// The lines a standalone file starts with, before the lemma, or
    /// `None` for each certificate's own imports:
    /// - for NanoYalla, its `macroll` module ([`NANOYALLA_PRELUDE`]);
    /// - for deduxit's library, [`DEDUXIT_PRELUDE`] or
    ///   [`DEDUXIT_TWO_SIDED_PRELUDE`], for a sequent with a quantifier
    ///   [`DEDUXIT_FIRST_ORDER_PRELUDE`] or
    ///   [`DEDUXIT_FIRST_ORDER_TWO_SIDED_PRELUDE`], and for a certificate
    ///   that a sequent has no proof [`DEDUXIT_REFUTATION_PRELUDE`] or
    ///   [`DEDUXIT_FIRST_ORDER_REFUTATION_PRELUDE`];
    /// - for a classical certificate over `Prop`, the standard library's
    ///   excluded middle ([`CLASSICAL_PRELUDE`]);
    /// - for one of LJ, nothing.
    pub prelude: Option<String>,
    /// The kernel a certificate of linear logic is written for, or `None`
    /// for the one the proof chooses, as the
    /// [module docs](crate::export::rocq) say: in short, NanoYalla for
    /// classical linear logic, deduxit's library for the rest. A
    /// certificate of ordinary logic needs no kernel and ignores it.
    pub kernel: Option<Kernel>,
}

impl Default for Options {
    /// Returns the lemma `certificate` alone, and each target's own
    /// prelude for a whole file.
    fn default() -> Self {
        Self {
            form: Form::Fragment,
            lemma: Identifier::new("certificate").expect("an identifier"),
            prelude: None,
            kernel: None,
        }
    }
}

impl Options {
    /// Returns the options with the form given.
    #[must_use]
    pub fn with_form(self, form: Form) -> Self {
        Self { form, ..self }
    }

    /// Returns the options with the lemma given.
    #[must_use]
    pub fn with_lemma(self, lemma: Identifier) -> Self {
        Self { lemma, ..self }
    }

    /// Returns the options with the prelude given, `None` for each
    /// target's own.
    #[must_use]
    pub fn with_prelude(self, prelude: Option<String>) -> Self {
        Self { prelude, ..self }
    }

    /// Returns the options with the kernel given, `None` for the one the
    /// mode chooses.
    #[must_use]
    pub fn with_kernel(self, kernel: Option<Kernel>) -> Self {
        Self { kernel, ..self }
    }
}

/// The Rocq kernel a certificate of linear logic is written for.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Kernel {
    /// NanoYalla [`NANOYALLA`], the kernel of Click & coLLecT: classical
    /// linear logic, one-sided, without Mix, cut, quantifiers or the
    /// weakening of affine mode. A proof script checks the proof rule by
    /// rule.
    NanoYalla,
    /// deduxit's own Rocq library [`DEDUXIT`]: one-sided linear logic with
    /// Mix and weakening where the mode allows them, and two-sided
    /// intuitionistic linear logic and its affine variant, each with cut
    /// where the proof uses one. The library's checker checks the proof
    /// as a term. Read from `"linlog"` too, its name up to 0.2.0.
    #[cfg_attr(feature = "serialize", serde(alias = "linlog"))]
    Deduxit,
}

/// The version of deduxit's Rocq library the certificates of its kernel
/// are written for. The library is the package `rocq-deduxit` of the
/// repository <https://github.com/deduxit/rocq-deduxit>, logical root
/// `Deduxit`, its linear logic under `Deduxit.Linear`. It builds with
/// Rocq 9 and its standard library.
pub const DEDUXIT: &str = "0.3.0";

/// The prelude of a one-sided certificate for deduxit's library: the
/// imports of its soundness theorems, and of lists and strings, whose
/// notations the lemma uses.
pub const DEDUXIT_PRELUDE: &str = "From Stdlib Require Import List String.
From Deduxit.Linear Require Import Soundness.
Import ListNotations.
Open Scope string_scope.";

/// The prelude of a two-sided certificate for deduxit's library: as
/// [`DEDUXIT_PRELUDE`], with the two-sided calculus and its theorem.
pub const DEDUXIT_TWO_SIDED_PRELUDE: &str = "From Stdlib Require Import List String.
From Deduxit.Linear Require Import IntuitionisticCheck.
Import ListNotations.
Open Scope string_scope.";

/// The prelude of a certificate of a first-order derivation for deduxit's
/// library: as [`DEDUXIT_PRELUDE`], with the first-order calculus and its
/// theorems. It also requires the propositional calculus's module, whose
/// optional rules the statement names.
pub const DEDUXIT_FIRST_ORDER_PRELUDE: &str = "From Stdlib Require Import List String.
From Deduxit.Linear Require Calculus.
From Deduxit.Linear Require Import FirstOrderSoundness.
Import ListNotations.
Open Scope string_scope.";

/// The prelude of a two-sided certificate of a first-order derivation for
/// deduxit's library: as [`DEDUXIT_FIRST_ORDER_PRELUDE`], with the two-sided
/// calculus and its theorem.
pub const DEDUXIT_FIRST_ORDER_TWO_SIDED_PRELUDE: &str = "From Stdlib Require Import List String.
From Deduxit.Linear Require Intuitionistic.
From Deduxit.Linear Require Import FirstOrderIntuitionisticCheck.
Import ListNotations.
Open Scope string_scope.";

/// The prelude of a certificate that a sequent has no proof, for deduxit's
/// library: the imports of its refutation theorems (`Weights`, which
/// exports the others), and of lists, strings and integers, whose
/// notations the lemma uses.
pub const DEDUXIT_REFUTATION_PRELUDE: &str = "From Stdlib Require Import List String ZArith.
From Deduxit.Linear Require Import Weights.
Import ListNotations.
Open Scope string_scope.";

/// The prelude of a certificate that a sequent with a quantifier has no
/// proof, for deduxit's library: as [`DEDUXIT_REFUTATION_PRELUDE`], with
/// the first-order calculi, the first-order weighted count, Booleans, and
/// the modules whose names the statement and the weights use.
pub const DEDUXIT_FIRST_ORDER_REFUTATION_PRELUDE: &str =
    "From Stdlib Require Import Bool List String ZArith.
From Deduxit.Linear Require Calculus Intuitionistic TermsCheck.
From Deduxit.Linear Require Import FirstOrderCalculus FirstOrderIntuitionistic FirstOrderWeights.
Import ListNotations.
Open Scope string_scope.";

/// The prelude of a certificate for NanoYalla: the import of its
/// `macroll` module, which brings the kernel's `ll`, its formulas and its
/// derived rules into scope.
pub const NANOYALLA_PRELUDE: &str = "From NanoYalla Require Import macroll.";

/// The prelude of a classical certificate over `Prop`: the import of the
/// standard library's excluded middle, for `NNPP`.
pub const CLASSICAL_PRELUDE: &str = "From Stdlib Require Import Classical_Prop.";

/// A Rocq identifier, which a lemma's name must be: an ASCII letter or
/// `_`, then letters, digits, `_` and `'`, and no keyword of Rocq.
///
/// A name the kernel uses is allowed: the atoms are renamed around it
/// instead. So a valid name stays valid whatever kernel a later release
/// adds.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier(Box<str>);

impl Identifier {
    /// Returns the identifier `name` is.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidOption`] for a name that is no identifier or a
    /// keyword.
    pub fn new(name: &str) -> Result<Self, Error> {
        let mut chars = name.chars();
        let lexical = chars
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
            && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '\'');
        let why = if !lexical {
            "is no Rocq identifier: an ASCII letter or _, then letters, digits, _ and '"
        } else if KEYWORDS.contains(&name) {
            "is a keyword of Rocq"
        } else {
            return Ok(Self(name.into()));
        };
        Err(Error::InvalidOption {
            key: "rocq.lemma",
            message: format!("`{name}` {why}"),
        })
    }

    /// Returns the identifier's text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for Identifier {
    type Err = Error;

    /// Reads an identifier as [`new`](Self::new) does.
    fn from_str(name: &str) -> Result<Self, Error> {
        Self::new(name)
    }
}

impl std::fmt::Display for Identifier {
    /// Writes the identifier.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(feature = "serialize")]
impl serde::Serialize for Identifier {
    /// Serializes the identifier as its text.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

#[cfg(feature = "serialize")]
impl<'a> serde::Deserialize<'a> for Identifier {
    /// Deserializes an identifier from its text, refusing what
    /// [`new`](Self::new) refuses.
    fn deserialize<D: serde::Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let name = <std::borrow::Cow<'a, str>>::deserialize(deserializer)?;
        Self::new(&name).map_err(serde::de::Error::custom)
    }
}

/// Why a proof, or a refutation, has no certificate.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Error)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Unsupported {
    /// The derivation has an open goal: it is a proof in progress.
    #[error("the proof is not finished: an open goal has no certificate")]
    Open,
    /// The derivation uses Mix, which NanoYalla lacks.
    #[error("NanoYalla has no Mix rule, so a proof with Mix has no certificate")]
    Mix,
    /// The derivation uses a cut, which NanoYalla's kernel lacks: there a
    /// cut is an axiom, or a theorem of the Yalla library.
    #[error(
        "NanoYalla's kernel has no cut rule, so a proof with a cut has its certificate in deduxit's library"
    )]
    Cut,
    /// The derivation weakens a formula that is not a `?` formula, which
    /// NanoYalla cannot.
    #[error(
        "NanoYalla weakens ? formulas only, so a proof with the weakening of affine mode has no certificate"
    )]
    AffineWeakening,
    /// The derivation draws a run of a structural rule as one inference,
    /// which names none of the formulas after the first.
    #[error(
        "a compact derivation names one formula of a run of structural rules, so it has no certificate: build the derivation with every rule"
    )]
    Compact,
    /// The options name NanoYalla for a disproof, but NanoYalla states
    /// derivations only.
    #[error(
        "NanoYalla's kernel states derivations only, so a refutation has its certificate in deduxit's library"
    )]
    Disproof,
    /// The refutation of an ordinary sequent's image is no assignment that
    /// falsifies the sequent read classically.
    #[error(
        "only a classical assignment that falsifies the sequent certifies that it is not valid, and its image's refutation is none"
    )]
    NotValid,
    /// The derivation or disproof is of an ordered mode, which neither
    /// kernel's calculi have: their sequents are commutative.
    #[error(
        "the Rocq calculi are commutative, so a derivation or a disproof of the cyclic mode has no certificate"
    )]
    Ordered,
    /// The disproof's refutation is a saturated set, for which the
    /// library has no theorem. Its certificate is the set itself, which
    /// the disproof's check verifies.
    #[error(
        "deduxit's library has no theorem for a saturated set, so a refutation by one has no certificate in Rocq"
    )]
    Saturated,
    /// The sequent has a quantifier, which NanoYalla's kernel and the
    /// library's theorem of the count equation lack.
    #[error(
        "NanoYalla's kernel and deduxit's theorem of the count equation have no quantifiers, so a sequent with ∀ or ∃ has a certificate for deduxit's library only, and no refutation by the count equation"
    )]
    Quantifiers,
    /// The disproof's refutation is an exhausted search, which no
    /// invariant states: only the search that was exhaustive rules the
    /// proofs out.
    #[error(
        "an exhausted search states no invariant, so only the search rules the proofs out and Rocq has nothing to check"
    )]
    Exhausted,
}

/// Rocq's keywords, which no atom and no lemma may take.
pub(crate) const KEYWORDS: &[&str] = &[
    "_",
    "Axiom",
    "CoFixpoint",
    "Definition",
    "Fixpoint",
    "Hypothesis",
    "IF",
    "Lemma",
    "Parameter",
    "Proof",
    "Prop",
    "Qed",
    "SProp",
    "Set",
    "Theorem",
    "Type",
    "Variable",
    "as",
    "at",
    "by",
    "cofix",
    "discriminated",
    "else",
    "end",
    "exists",
    "exists2",
    "fix",
    "for",
    "forall",
    "fun",
    "if",
    "in",
    "lazymatch",
    "let",
    "match",
    "multimatch",
    "return",
    "then",
    "using",
    "where",
    "with",
];

/// The names of the kernel a script uses, which no atom may take.
const KERNEL: &[&str] = &[
    "Atom",
    "app",
    "aplus",
    "apply",
    "awith",
    "ax_expansion",
    "ax_r_ext",
    "bidual",
    "bot",
    "bot_r_ext",
    "cbn_sequent",
    "co_r_ext",
    "cons",
    "covar",
    "de_r_ext",
    "dual",
    "ex_perm_r",
    "ex_transpL",
    "formula",
    "list",
    "ll",
    "map",
    "nat",
    "nil",
    "O",
    "oc",
    "oc_r_ext",
    "one",
    "one_r_ext",
    "parr",
    "parr_r_ext",
    "permL_of_perm",
    "plus_r1_ext",
    "plus_r2_ext",
    "S",
    "tens",
    "tens_r_ext",
    "top",
    "top_r_ext",
    "var",
    "with_r_ext",
    "wk_r_ext",
    "wn",
    "zero",
];

/// Returns an atom's name as a Rocq identifier, before the check for
/// clashes.
fn identifier(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    if name.starts_with(|c: char| c.is_ascii_digit() || c == '\'') {
        out.push('_');
    }
    for c in name.chars() {
        if c.is_ascii_alphanumeric() || c == '_' || c == '\'' {
            out.push(c);
        } else {
            write!(out, "_{:x}_", c as u32).unwrap();
        }
    }
    if out.is_empty() {
        out.push('_');
    }
    out
}

/// Returns the identifiers of atoms with these names, in their order,
/// each free of clashes with the keywords, the kernel, `lemma` and the
/// others: the first clash of an identifier gets `'` appended, the later
/// ones `'2`, `'3` and so on.
pub(crate) fn identifiers(atoms: &[String], lemma: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::with_capacity(atoms.len());
    // The names taken so far, so that a clash costs one lookup. Comparing
    // with every earlier name is quadratic in the atoms: seconds before
    // the first poll for a hundred thousand atoms.
    let mut taken: HashSet<String> = HashSet::default();
    // Per identifier, the last suffix given to one of its clashes, so
    // that the next clash starts after it. Names that all map to one
    // identifier (`ä` and `_e4_`) would otherwise each try every suffix
    // before theirs, quadratic in their number.
    let mut clashes: HashMap<String, u32> = HashMap::default();
    for name in atoms {
        let base = identifier(name);
        let free = |id: &str, taken: &HashSet<String>| {
            !KEYWORDS.contains(&id) && !KERNEL.contains(&id) && id != lemma && !taken.contains(id)
        };
        let mut id = base.clone();
        if !free(&id, &taken) {
            let suffix = clashes.entry(base.clone()).or_insert(0);
            loop {
                *suffix += 1;
                id = match *suffix {
                    1 => format!("{base}'"),
                    n => format!("{base}'{n}"),
                };
                if free(&id, &taken) {
                    break;
                }
            }
        }
        taken.insert(id.clone());
        names.push(id);
    }
    names
}

/// Writes the formula at `id` in the kernel's constructors, in brackets if
/// `argument` is set and it is an application.
fn term(out: &mut String, sequent: &Sequent, names: &[String], id: TermId, argument: bool) {
    use Term::*;
    for visit in Stops::new(id, argument, |k| sequent.term(k).operands()) {
        match visit {
            Stop::Enter(k, argument) => {
                let t = sequent.term(k);
                if argument && !matches!(t, Atom(_) | One | Bot | Top | Zero) {
                    out.push('(');
                }
                match t {
                    Atom(a) => out.push_str(&names[a.index()]),
                    DualAtom(a) => {
                        out.push_str("dual ");
                        out.push_str(&names[a.index()]);
                        if argument {
                            out.push(')');
                        }
                    }
                    One => out.push_str("one"),
                    Bot => out.push_str("bot"),
                    Top => out.push_str("top"),
                    Zero => out.push_str("zero"),
                    Tensor(..) => out.push_str("tens "),
                    Par(..) => out.push_str("parr "),
                    With(..) => out.push_str("awith "),
                    Plus(..) => out.push_str("aplus "),
                    Bang(_) => out.push_str("oc "),
                    Quest(_) => out.push_str("wn "),
                    Forall(_) | Exists(_) => {
                        unreachable!("NanoYalla's certificate refuses a sequent with a binder")
                    }
                }
            }
            Stop::Between(_) => out.push(' '),
            // What has subformulas is an application.
            Stop::Exit(_, argument) => {
                if argument {
                    out.push(')');
                }
            }
        }
    }
}

/// Writes occurrences as a list of the kernel's formulas, `[A; dual B]`.
fn list(out: &mut String, forest: &Forest, names: &[String], occurrences: &[OccId]) {
    out.push('[');
    for (i, &o) in occurrences.iter().enumerate() {
        if i > 0 {
            out.push_str("; ");
        }
        term(out, forest.sequent(), names, forest.term(o), false);
    }
    out.push(']');
}

/// The script in progress.
struct Script<'a> {
    /// The derivation being written.
    derivation: &'a Derivation<'a>,
    /// The forest its occurrences index.
    forest: &'a Forest,
    /// The identifier of every atom.
    names: Vec<String>,
    /// The text so far.
    out: String,
    /// The list of formulas Rocq shows as the goal of each inference not
    /// yet written, set when its conclusion is written.
    goals: Vec<Vec<OccId>>,
    /// Whether an inference is a premise of a binary rule, whose script
    /// goes into braces of its own.
    braced: Vec<bool>,
    /// How many braces are open.
    depth: usize,
}

impl Script<'_> {
    /// Writes a tactic line at the current indentation.
    fn line(&mut self, text: &str) {
        for _ in 0..self.depth {
            self.out.push_str("  ");
        }
        self.out.push_str(text);
        self.out.push('\n');
    }

    /// Writes a rule's tactic, `apply (<rule> <arguments>); cbn_sequent.`,
    /// without the `cbn_sequent` when the rule leaves no goal.
    fn apply(&mut self, rule: &str, arguments: &[String], leaf: bool) {
        let mut text = format!("apply ({rule}");
        for a in arguments {
            text.push(' ');
            text.push_str(a);
        }
        text.push(')');
        text.push_str(if leaf { "." } else { "; cbn_sequent." });
        self.line(&text);
    }

    /// Returns occurrences as a list of the kernel's formulas.
    fn list(&self, occurrences: &[OccId]) -> String {
        let mut out = String::new();
        list(&mut out, self.forest, &self.names, occurrences);
        out
    }

    /// Writes the tactic of the inference `id`, whose goal is `goal`, and
    /// sets the goals of its premises.
    fn inference(&mut self, id: InfId, goal: Vec<OccId>) {
        let (derivation, forest) = (self.derivation, self.forest);
        let inference = derivation.inference(id);
        let rule = inference.rule.rule;
        if rule == Rule::Ax {
            self.line("ax_expansion.");
            return;
        }
        let o = inference.sequent[inference.principal.unwrap()];
        let at = goal.iter().position(|&x| x == o.occ()).unwrap();
        let (before, after) = (&goal[..at], &goal[at + 1..]);
        let prefix = self.list(before);
        let (left, right) = (forest.left(o.occ()), forest.right(o.occ()));
        // The goal of the premise when the rule replaces the principal
        // formula by `with`.
        let replaced = |with: &[OccId]| {
            let mut up = Vec::with_capacity(goal.len() + 1);
            up.extend_from_slice(before);
            up.extend_from_slice(with);
            up.extend_from_slice(after);
            up
        };
        let premise = |script: &mut Self, up: Vec<OccId>| {
            script.goals[inference.premises[0].index()] = up;
        };
        match rule {
            // The export refuses a sequent with binders.
            Rule::Forall | Rule::Exists => unreachable!("NanoYalla's certificate refuses a binder"),
            Rule::One => self.line("apply one_r_ext."),
            Rule::Top => self.apply("top_r_ext", &[prefix], true),
            Rule::Bot => {
                self.apply("bot_r_ext", &[prefix], false);
                premise(self, replaced(&[]));
            }
            Rule::Par => {
                self.apply("parr_r_ext", &[prefix], false);
                premise(self, replaced(&[left.unwrap(), right.unwrap()]));
            }
            Rule::PlusLeft => {
                self.apply("plus_r1_ext", &[prefix], false);
                premise(self, replaced(&[left.unwrap()]));
            }
            Rule::PlusRight => {
                self.apply("plus_r2_ext", &[prefix], false);
                premise(self, replaced(&[right.unwrap()]));
            }
            Rule::Dereliction => {
                self.apply("de_r_ext", &[prefix], false);
                premise(self, replaced(&[left.unwrap()]));
            }
            Rule::Weakening => {
                self.apply("wk_r_ext", &[prefix], false);
                premise(self, replaced(&[]));
            }
            Rule::Contraction => {
                self.apply("co_r_ext", &[prefix], false);
                premise(self, replaced(&[o.occ(), o.occ()]));
            }
            Rule::Promotion => {
                // The context is `?` formulas, which the rule takes
                // without their `?`.
                let under: Vec<OccId> = before
                    .iter()
                    .chain(after)
                    .map(|&q| forest.left(q).unwrap())
                    .collect();
                let (l1, l2) = under.split_at(before.len());
                let mut a = String::from("(");
                term(
                    &mut a,
                    forest.sequent(),
                    &self.names,
                    forest.term(left.unwrap()),
                    false,
                );
                a.push(')');
                let arguments = [self.list(l1), a, self.list(l2)];
                self.apply("oc_r_ext", &arguments, false);
                premise(self, replaced(&[left.unwrap()]));
            }
            Rule::With => {
                self.apply("with_r_ext", &[prefix], false);
                let (l, r) = (inference.premises[0], inference.premises[1]);
                self.goals[l.index()] = replaced(&[left.unwrap()]);
                self.goals[r.index()] = replaced(&[right.unwrap()]);
                self.braced[l.index()] = true;
                self.braced[r.index()] = true;
            }
            Rule::Tensor => self.tensor(id, goal),
            Rule::Ax | Rule::Mix | Rule::AffineWeakening | Rule::Cut | Rule::Open => {
                unreachable!("refused before anything is written")
            }
        }
    }

    /// Writes the tactics of a `⊗`: the exchange that puts the left
    /// premise's context before the `⊗` and the right premise's after it,
    /// when the goal is not in that order already, then the rule.
    fn tensor(&mut self, id: InfId, goal: Vec<OccId>) {
        let (derivation, forest) = (self.derivation, self.forest);
        let inference = derivation.inference(id);
        let o = inference.sequent[inference.principal.unwrap()];
        let (a, b) = (
            forest.left(o.occ()).unwrap(),
            forest.right(o.occ()).unwrap(),
        );
        let (l, r) = (inference.premises[0], inference.premises[1]);
        // The left context, consumed as the goal's formulas are assigned.
        let mut left: Vec<OccId> = derivation
            .inference(l)
            .sequent
            .iter()
            .map(|m| m.occ())
            .collect();
        left.remove(left.iter().position(|&x| x == a).unwrap());
        let (mut before, mut after) = (Vec::new(), Vec::new());
        let mut principal = false;
        for &x in &goal {
            if x == o.occ() && !principal {
                principal = true;
            } else if let Some(i) = left.iter().position(|&y| y == x) {
                left.remove(i);
                before.push(x);
            } else {
                after.push(x);
            }
        }
        let mut target = before.clone();
        target.push(o.occ());
        target.extend_from_slice(&after);
        if target != goal {
            // `ex_perm_r p l` proves `l` permuted so that position `i`
            // holds `l[p[i]]`, from `l`.
            let mut used = vec![false; target.len()];
            let p: Vec<String> = goal
                .iter()
                .map(|&x| {
                    let j = (0..target.len())
                        .find(|&j| !used[j] && target[j] == x)
                        .unwrap();
                    used[j] = true;
                    j.to_string()
                })
                .collect();
            let arguments = [format!("[{}]", p.join("; ")), self.list(&target)];
            self.apply("ex_perm_r", &arguments, true);
        }
        let prefix = self.list(&before);
        self.apply("tens_r_ext", &[prefix], false);
        before.push(a);
        after.insert(0, b);
        self.goals[l.index()] = before;
        self.goals[r.index()] = after;
        self.braced[l.index()] = true;
        self.braced[r.index()] = true;
    }
}

/// Writes the certificate of a derivation into `out`, in the options'
/// form, one inference at a time, and asks `stop` after each.
///
/// [`to_string`] returns the same text as a `String`. Use `write` to
/// write into your own buffer, or to stop a large certificate. A
/// derivation without a certificate is refused before anything is
/// written.
///
/// What the certificate states depends on the derivation:
/// - A linear derivation is a lemma for the kernel that
///   [`Options::kernel`] names or the proof chooses. For NanoYalla its
///   proof is a script, checked one-sided. For deduxit's library it is a
///   proof term, one-sided, or two-sided for a two-sided derivation.
/// - A derivation of LK or LJ, which must have passed
///   [`check`](crate::ordinary::Derivation::check), is the lemma
///   `options.lemma` that states the ordinary sequent over `Prop`. Every
///   atom is a proposition bound by `forall`, the hypotheses are premises,
///   and the formulas right of `⊢` are their disjunction (`False` for
///   none). The proof is a term made rule by rule. It needs no library,
///   but a classical one uses the excluded middle of the standard library
///   (`NNPP`), which a standalone file imports through `options.prelude`,
///   by default [`CLASSICAL_PRELUDE`].
///
/// # Errors
///
/// - [`Error::Unsupported`] for a derivation without a certificate: an
///   open goal, a compact run or a proof of the cyclic mode, and for
///   NanoYalla Mix, affine weakening, a cut or a quantifier.
/// - [`Error::GoalProof`] for a proof of a goal, not of the sequent.
/// - [`Refusal::Stopped`](crate::Refusal::Stopped) when `stop` fired.
/// - [`Error::WriteFailed`] when `out` refused the text.
pub fn write<'a>(
    derivation: impl Into<Drawable<'a>>,
    options: &Options,
    out: &mut impl Write,
    stop: impl FnMut(crate::limits::Progress) -> bool,
) -> Result<(), Error> {
    let stop = crate::limits::counting(stop, crate::limits::Phase::Write);
    match derivation.into() {
        Drawable::Linear(derivation) => linear(derivation, options, out, stop),
        Drawable::Ordinary(derivation) => {
            crate::ordinary::rocq::write(derivation, options, out, stop)
        }
        // The certificate is the proof's, whichever way it is drawn.
        Drawable::Classical(view) => linear(view.derivation(), options, out, stop),
    }
}

/// Returns the certificate of a derivation, as [`write()`] writes it.
///
/// It has no stop and no limits of its own: the text grows linearly with
/// the derivation, which was made within its own limits
/// ([`Proof::derivation`](crate::linear::Proof::derivation) uses the
/// default ones). The [module docs](crate::export::rocq) have an example.
///
/// # Errors
///
/// As [`write()`], except [`Refusal::Stopped`](crate::Refusal::Stopped)
/// and [`Error::WriteFailed`], which a `String` never gives.
pub fn to_string<'a>(
    derivation: impl Into<Drawable<'a>>,
    options: &Options,
) -> Result<String, Error> {
    let mut out = String::new();
    write(derivation, options, &mut out, |_| false)?;
    Ok(out)
}

/// Writes the certificate that a disproof's goal has no derivation into
/// `out`, for deduxit's library ([`Kernel::Deduxit`]) whatever the mode.
///
/// The certificate is the lemma `options.lemma` that the mode's calculus
/// does not derive the goal: `~ ll R Γ` one-sided or, in intuitionistic
/// mode, `~ ill R Γ C` two-sided, `R` being the mode's rules. Its proof
/// applies the library's theorem of the refutation's invariant
/// (`classical_refutation`, `unbalanced_refutation`,
/// `equation_refutation`, `weighted_refutation` or `affine_refutation`,
/// or their two-sided forms) to the refutation's data, and Rocq's
/// `vm_compute` checks it. The data are the assignment's true atoms, the
/// unbalanced atom or the weights:
/// - a one-sided count weighs its atom `±1`;
/// - a state equation weighs its atoms as the refutation does, and
///   declares dead, in the bodies' sign, the atoms no firing reaches.
///
/// A sequent with a quantifier is stated over the first-order calculi,
/// and its counts are proved by the first-order weighted theorem.
///
/// Every refutation that [`Disproof::check`](crate::linear::Disproof::check)
/// accepts has a certificate, except a saturated set and the count
/// equation of a sequent with a quantifier. The disproof is checked
/// first. A standalone file starts with `options.prelude`, by default
/// [`DEDUXIT_REFUTATION_PRELUDE`], or
/// [`DEDUXIT_FIRST_ORDER_REFUTATION_PRELUDE`] for a sequent with a
/// quantifier.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::export::rocq::{self, Options};
/// use deduxit::linear::{Mode, Options as Search, Sequent, Verdict, prove};
///
/// let sequent: Sequent = "A |- A * A".parse()?;
/// let outcome = prove(&sequent, Mode::CLASSICAL, &Search::default())?;
/// let Verdict::Unprovable(disproof) = &outcome.verdict else {
///     panic!("unprovable");
/// };
/// let mut certificate = String::new();
/// rocq::write_disproof(disproof, &Options::default(), &mut certificate)?;
/// assert!(certificate.starts_with(
///     r#"Lemma certificate : ~ ll (atom := string) classical [covar "A"; tens (var "A") (var "A")]."#
/// ));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
///
/// - [`Error::Refutation`] for a refutation the check does not accept.
/// - [`Error::Unsupported`] with [`Unsupported::Disproof`] where the
///   options name NanoYalla, [`Unsupported::Ordered`] in the cyclic mode,
///   [`Unsupported::Saturated`] for a refutation by a saturated set,
///   [`Unsupported::Exhausted`] for an exhausted search, and
///   [`Unsupported::Quantifiers`] for the count equation of a sequent
///   with a quantifier.
/// - [`Error::GoalOutputs`] for an intuitionistic goal without exactly
///   one conclusion.
/// - The errors of [`Forest::new`] and of the intuitionistic reading.
/// - [`Error::WriteFailed`] when `out` refused the text.
pub fn write_disproof(
    disproof: &crate::linear::Disproof,
    options: &Options,
    out: &mut impl Write,
) -> Result<(), Error> {
    if options.kernel == Some(Kernel::NanoYalla) {
        return Err(Unsupported::Disproof.into());
    }
    if disproof.mode().is_ordered() {
        return Err(Unsupported::Ordered.into());
    }
    if let crate::linear::search::Refutation::Saturated(_) = disproof.refutation() {
        return Err(Unsupported::Saturated.into());
    }
    if let crate::linear::search::Refutation::Exhausted = disproof.refutation() {
        return Err(Unsupported::Exhausted.into());
    }
    deduxit::disproof(disproof, options, out)
}

/// Writes the certificate that an ordinary sequent is not valid, from its
/// image's disproof: the lemma `options.lemma` stating `~ (forall a b :
/// Prop, Γ -> Δ)`, the sequent as [`write()`] states it over `Prop`,
/// proved by instantiating each atom with `True` or `False` as the
/// image's classical refutation assigns it, which makes the formula
/// false. It needs no library, in classical and intuitionistic logic
/// alike; standalone, it starts with `options.prelude` where one is
/// given.
///
/// # Errors
///
/// [`Error::Unsupported`] with [`Unsupported::NotValid`] where the image's
/// refutation is no classical assignment or does not falsify the
/// sequent, [`Error::Refutation`] where it does not falsify the image,
/// and [`Error::WriteFailed`] when `out` refused the text.
pub fn write_not_valid(
    image: &crate::ordinary::Image,
    disproof: &crate::linear::Disproof,
    options: &Options,
    out: &mut impl Write,
) -> Result<(), Error> {
    crate::ordinary::rocq::not_valid(image, disproof, options, out)
}

/// Writes the certificate of a linear derivation for the kernel the
/// options name or the mode chooses, as [`write()`] does.
fn linear(
    derivation: &Derivation,
    options: &Options,
    out: &mut impl Write,
    stop: impl FnMut() -> bool,
) -> Result<(), Error> {
    if derivation.is_of_goal() {
        return Err(Error::GoalProof);
    }
    if derivation.mode().is_some_and(Mode::is_ordered) {
        return Err(Unsupported::Ordered.into());
    }
    let uses = |rule| derivation.inferences().iter().any(|i| i.rule.rule == rule);
    let (mix, weakening, cut) = (
        uses(Rule::Mix),
        uses(Rule::AffineWeakening),
        uses(Rule::Cut),
    );
    let mode = derivation.mode();
    let binders = derivation.forest().has_binders();
    let kernel = options.kernel.unwrap_or(match mode {
        Some(mode) if mode.is_intuitionistic() || mode.is_affine() || mode.has_mix() => {
            Kernel::Deduxit
        }
        // Classical linear logic, or a proof that records no mode and
        // shows its own: NanoYalla, unless it needs what the kernel lacks.
        _ if mix
            || weakening
            || cut
            || binders
            || (mode.is_none() && derivation.reading().is_some()) =>
        {
            Kernel::Deduxit
        }
        _ => Kernel::NanoYalla,
    });
    match kernel {
        // NanoYalla's formulas have no quantifier.
        Kernel::NanoYalla if binders => Err(Unsupported::Quantifiers.into()),
        Kernel::NanoYalla => nanoyalla(derivation, options, out, stop),
        Kernel::Deduxit => {
            // The calculus states what the mode allows and what the proof
            // uses, which a proof checked in another mode may exceed.
            let calculus = deduxit::Calculus {
                mix: mix || mode.is_some_and(Mode::has_mix),
                weakening: weakening || mode.is_some_and(Mode::is_affine),
                cut,
            };
            deduxit::write(derivation, options, calculus, out, stop)
        }
    }
}

/// Writes the NanoYalla certificate of a linear derivation as [`write()`]
/// does.
fn nanoyalla(
    derivation: &Derivation,
    options: &Options,
    out: &mut impl Write,
    mut stop: impl FnMut() -> bool,
) -> Result<(), Error> {
    for inference in derivation.inferences() {
        if inference.times > 1 {
            return Err(Unsupported::Compact.into());
        }
        match inference.rule.rule {
            Rule::Open => return Err(Unsupported::Open.into()),
            Rule::Mix => return Err(Unsupported::Mix.into()),
            Rule::AffineWeakening => return Err(Unsupported::AffineWeakening.into()),
            Rule::Cut => return Err(Unsupported::Cut.into()),
            Rule::Forall | Rule::Exists => return Err(Unsupported::Quantifiers.into()),
            Rule::Ax
            | Rule::Tensor
            | Rule::Par
            | Rule::One
            | Rule::Bot
            | Rule::With
            | Rule::PlusLeft
            | Rule::PlusRight
            | Rule::Top
            | Rule::Promotion
            | Rule::Dereliction
            | Rule::Contraction
            | Rule::Weakening => {}
        }
    }
    let forest = derivation.forest();
    let names = identifiers(forest.sequent().atom_names(), options.lemma.as_str());
    let count = derivation.inferences().len();
    let mut script = Script {
        derivation,
        forest,
        names,
        out: String::new(),
        goals: vec![Vec::new(); count],
        braced: vec![false; count],
        depth: 0,
    };
    let root = derivation.root();
    script.goals[root.index()] = derivation
        .inference(root)
        .sequent
        .iter()
        .map(|m| m.occ())
        .collect();

    if options.form == Form::Standalone {
        let prelude = options.prelude.as_deref().unwrap_or(NANOYALLA_PRELUDE);
        write!(out, "{}\n\n", prelude.trim_end())?;
    }
    write!(script.out, "Lemma {}", options.lemma)?;
    if !script.names.is_empty() {
        write!(script.out, " ({} : formula)", script.names.join(" "))?;
    }
    script.out.push_str(" : ll ");
    let conclusion = script.list(&script.goals[root.index()]);
    script.out.push_str(&conclusion);
    script.out.push_str(".\nProof.\n");
    walk(derivation, |step| {
        match step {
            Step::Enter(id, _) => {
                if script.braced[id.index()] {
                    script.line("{");
                    script.depth += 1;
                }
                let goal = std::mem::take(&mut script.goals[id.index()]);
                script.inference(id, goal);
            }
            Step::Exit(id, _) => {
                if script.braced[id.index()] {
                    script.depth -= 1;
                    script.line("}");
                }
            }
        }
        flush(out, &mut script.out, &mut stop)
    })?;
    out.write_str("Qed.")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Letters, digits, `_` and `'` stay; other characters become their
    /// code points; a clash with a keyword, the kernel, the lemma or
    /// another atom gets `'` appended.
    #[test]
    fn names_become_identifiers() {
        for (name, expected) in [
            ("A", "A"),
            ("x_1", "x_1"),
            ("p'", "p'"),
            ("1a", "_1a"),
            ("α", "_3b1_"),
            ("a b", "a_20_b"),
            ("", "_"),
        ] {
            assert_eq!(identifier(name), expected, "{name:?}");
        }
        let atoms: Vec<String> = ["tens", "with", "certificate", "α", "_3b1_", "_"]
            .map(String::from)
            .into();
        assert_eq!(
            identifiers(&atoms, "certificate"),
            ["tens'", "with'", "certificate'", "_3b1_", "_3b1_'", "_'"]
        );
        // Names that all map to one identifier take one suffix each.
        let atoms: Vec<String> = ["ää", "ä_e4_", "_e4_ä", "_e4__e4_"]
            .map(String::from)
            .into();
        assert_eq!(
            identifiers(&atoms, "certificate"),
            ["_e4__e4_", "_e4__e4_'", "_e4__e4_'2", "_e4__e4_'3"]
        );
    }
}
