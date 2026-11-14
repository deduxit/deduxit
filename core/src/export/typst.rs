// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Writes sequents and proofs as Typst: formulas in math mode, and proofs
//! as proof trees of the `curryst` package or of deduxit's own layout.
//!
//! | to get | call |
//! |---|---|
//! | a proof tree | [`to_string`] |
//! | a proof tree, written into your own buffer with a stop | [`write()`] |
//! | a sequent alone | [`sequent`] |
//!
//! [`Options`] holds the choices: a fragment or a whole document, the rule
//! labels, how an open goal looks, the layout and its spacing.
//!
//! Needs the cargo feature `typst` (on by default).
//!
//! # Examples
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::export::typst;
//! use deduxit::{Limits, linear::{Mode, Options, Sequent, Verdict, prove}};
//!
//! let sequent: Sequent = "A, A -o B |- B".parse()?;
//! let outcome = prove(&sequent, Mode::INTUITIONISTIC, &Options::default())?;
//! let Verdict::Proved(proof) = &outcome.verdict else {
//!     panic!("provable");
//! };
//! let options = typst::Options::default();
//! let tree = typst::to_string(&proof.derivation()?, &options)?;
//! assert_eq!(
//!     tree,
//!     r#"#prooftree(
//!   rule(
//!     name: $⊸ upright(L)$,
//!     rule(name: $"ax"$, $A ⊢ A$),
//!     rule(name: $"ax"$, $B ⊢ B$),
//!     $A, A ⊸ B ⊢ B$,
//!   ),
//! )"#
//! );
//!
//! // The sequent alone, one-sided as classical linear logic writes it.
//! let printed = typst::sequent(&sequent, Mode::CLASSICAL, &options, &Limits::default())?;
//! assert_eq!(printed, "$⊢ A^⊥, A ⊗ B^⊥, B$");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # What the output needs
//!
//! By default the output is a fragment: a sequent between `$`s, a
//! `#prooftree(…)` call of curryst, or a `#context` block of deduxit's own
//! layout. Paste it into a document. A curryst tree needs `prooftree` and
//! `rule` imported from curryst, version [`CURRYST`]; the own layout needs no
//! package. With [`Form::Standalone`] the output is a document whose page
//! is the size of its content, and which imports curryst itself where the
//! tree needs it.
//!
//! The output chooses no font. It takes the fonts of the document it goes
//! into.
//!
//! # The two layouts
//!
//! curryst's `rule(name: …, premises…, conclusion)` nests the premises
//! inside their conclusion, and curryst nests its layout at every level of
//! the tree. So Typst refuses a curryst tree with a branch of more than
//! [`CURRYST_HEIGHT`] inferences that has a rule of two premises on it
//! ("maximum show rule depth exceeded").
//!
//! By default ([`Layout::Auto`]) a higher tree is written in deduxit's own
//! layout instead. That layout lists the inferences in a `#context` block,
//! measures them with Typst, lays the tree out as the SVG drawing does,
//! and places every piece in one box. Nothing is nested per level, so the
//! height of the tree reaches no limit of Typst's.
//!
//! curryst cannot line up turnstiles, so every sequent is centred, in
//! both layouts.
//!
//! # How it is written
//!
//! - The connectives are their Unicode characters (`⊗ ⅋ ⊕ ⊸ ⊢ ⊥ ⊤`).
//!   Typst's math reads them with their Unicode math class, and no
//!   renaming of Typst's symbol names can break them.
//! - `&` is escaped and set as a binary operator. `?` is set as an
//!   ordinary symbol, so that no punctuation space follows it. `1` is
//!   bold.
//! - By default an open goal is its sequent under vertical dots, with no
//!   inference line ([`Options::open`]).
//!
//! # Atom names
//!
//! A name of one letter is written as it is, in math italic. Any other
//! name is a string in `italic(…)`, with `"` and `\` escaped: Typst would
//! read a longer name as a variable. Typst reads Unicode throughout, so
//! names beyond ASCII need nothing else.

use super::notation::{Step, flush, walk};
use super::{Drawable, Form};
use crate::Error;
use crate::linear::proofs::style::{Drawn, Part, parts};
use crate::linear::proofs::{Labels, OpenGoal};
use crate::linear::sequents::Sequent;
use crate::linear::sequents::notation::{Atoms, Notation};
use crate::ordinary::Symbols;
use std::fmt::Write;

// The flake's `export` check compiles the output with the curryst of
// nixpkgs, so this version and that one change together.
/// The version of curryst the output is written for, and the one a
/// standalone document imports.
pub const CURRYST: &str = "0.6.0";

/// The Typst spelling of formulas and sequents.
const NOTATION: Notation = Notation {
    tensor: "⊗",
    par: "⅋",
    with: r#"class("binary", \&)"#,
    plus: "⊕",
    lollipop: "⊸",
    under: r#"class("binary", backslash)"#,
    over: r#"class("binary", slash)"#,
    bang: "!",
    quest: r#"class("normal", ?)"#,
    one: "bold(1)",
    bot: "⊥",
    top: "⊤",
    zero: "0",
    dual_prefix: "",
    dual: "^⊥",
    turnstile: "⊢",
    align: "",
    atom: Atoms::Typst,
    forall: "forall ",
    exists: "exists ",
    dot: ". ",
    ordinary: Symbols::UNICODE,
};

/// Writes an atom's name in math mode: one letter as it is, any other
/// name as an italic string.
pub(crate) fn atom<W: Write + ?Sized>(out: &mut W, name: &str) -> std::fmt::Result {
    let mut chars = name.chars();
    if let (Some(c), None) = (chars.next(), chars.next())
        && c.is_alphabetic()
    {
        return out.write_char(c);
    }
    out.write_str("italic(\"")?;
    for c in name.chars() {
        if c == '"' || c == '\\' {
            out.write_char('\\')?;
        }
        out.write_char(c)?;
    }
    out.write_str("\")")
}

/// The default page setup of a standalone document: a page as large as its
/// content, with a margin of 5 pt.
pub const PAGE: &str = "#set page(width: auto, height: auto, margin: 5pt)";

// Measured with Typst 0.15.1 and curryst 0.6.0: a tree ten inferences high
// with a binary rule on the branch fails and nine compiles, on chains of
// `⊗` and on full binary trees. Chains of one-premise rules compile at
// thirteen. The number moves with the two versions.
/// The highest tree, in inferences on its longest branch, that curryst
/// [`CURRYST`] sets. Typst refuses a higher one that has a rule of two
/// premises on that branch ("maximum show rule depth exceeded").
pub const CURRYST_HEIGHT: usize = 9;

/// Which code sets a Typst proof tree: the curryst package, or Typst code
/// of deduxit's own written into the output.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Layout {
    /// curryst for a tree of at most [`CURRYST_HEIGHT`] inferences on
    /// its longest branch, deduxit's own layout for a higher one.
    #[default]
    Auto,
    /// curryst's `prooftree` and `rule`, which the document imports.
    Curryst,
    /// deduxit's own layout: Typst code in the output that measures every
    /// sequent and places it. It needs no package and has no limit on the
    /// height.
    ///
    /// JSON reads `"linlog"` as this value too, the name deduxit 0.2.0
    /// wrote, so its settings files still load.
    #[cfg_attr(feature = "serialize", serde(alias = "linlog"))]
    Deduxit,
}

/// A Typst length, such as `1.5em`, for the spacing of deduxit's own
/// layout.
///
/// The output writes a length as code, so a length is checked when it is
/// made: digits, then a `.` and more digits or not, then one of the units
/// [`UNITS`](Self::UNITS). Nothing else can reach the output through an
/// option. In JSON (cargo feature `serialize`) a length is its text.
///
/// # Examples
///
/// ```
/// use deduxit::export::typst::Length;
///
/// let gap: Length = "1.5em".parse()?;
/// assert_eq!(gap.as_str(), "1.5em");
/// assert!(Length::new("1.5 em").is_err());
/// # Ok::<(), deduxit::Error>(())
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Length(Box<str>);

impl Length {
    /// The units a length may have.
    pub const UNITS: [&'static str; 5] = ["pt", "mm", "cm", "in", "em"];

    /// Returns the length that `text` writes.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidOption`] for text that is not digits, an optional
    /// fraction and one of the [`UNITS`](Self::UNITS).
    pub fn new(text: &str) -> Result<Self, Error> {
        let number = text.trim_end_matches(|c: char| c.is_ascii_alphabetic());
        let (whole, fraction) = number.split_once('.').unwrap_or((number, "0"));
        let digits = |part: &str| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit());
        if digits(whole) && digits(fraction) && Self::UNITS.contains(&&text[number.len()..]) {
            Ok(Self(text.into()))
        } else {
            Err(Error::InvalidOption {
                key: "typst",
                message: format!(
                    "`{text}` is no Typst length: digits, a fraction or not, and one of {}",
                    Self::UNITS.join(", ")
                ),
            })
        }
    }

    /// Returns the length's text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for Length {
    type Err = Error;

    /// Reads a length as [`new`](Self::new) does.
    fn from_str(text: &str) -> Result<Self, Error> {
        Self::new(text)
    }
}

impl std::fmt::Display for Length {
    /// Writes the length as Typst reads it.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(feature = "serialize")]
impl serde::Serialize for Length {
    /// Serializes the length as its text.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

#[cfg(feature = "serialize")]
impl<'a> serde::Deserialize<'a> for Length {
    /// Deserializes a length from its text, refusing what
    /// [`new`](Self::new) refuses.
    fn deserialize<D: serde::Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let text = <std::borrow::Cow<'a, str>>::deserialize(deserializer)?;
        Self::new(&text).map_err(serde::de::Error::custom)
    }
}

/// The choices of the Typst output.
///
/// Start from `Options::default()` and change the fields you want, by
/// assignment or with the `with_*` methods. The struct is
/// `#[non_exhaustive]`, so a field added later breaks no caller.
///
/// # Examples
///
/// A sequent as a document that compiles on its own:
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::export::{Form, typst};
/// use deduxit::{Limits, linear::{Mode, Sequent}};
///
/// let sequent: Sequent = "A |- A".parse()?;
/// let options = typst::Options::default().with_form(Form::Standalone);
/// let document = typst::sequent(&sequent, Mode::CLASSICAL, &options, &Limits::default())?;
/// assert_eq!(document, format!("{}\n\n$⊢ A^⊥, A$", typst::PAGE));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Options {
    /// A fragment to paste, or a document that compiles on its own.
    pub form: Form,
    /// The labels of the rules.
    pub labels: Labels,
    /// How an open goal is drawn: under `dots.v`, bare, as a leaf rule
    /// labelled with a mark, or under a dashed line.
    pub open: OpenGoal,
    /// The line of a standalone document that imports `prooftree` and
    /// `rule` from curryst. Only a curryst tree writes it. It is written
    /// as it is, as Typst code.
    pub import: String,
    /// The page setup of a standalone document, written as it is, as Typst
    /// code.
    pub page: String,
    /// Which code sets the tree.
    pub layout: Layout,
    /// The least space between two premises side by side in deduxit's own
    /// layout.
    pub premise_gap: Length,
    /// The space between an inference line and its rule's label in
    /// deduxit's own layout.
    pub label_gap: Length,
    /// The least height of the space an inference line stands in, between
    /// its premises and its conclusion, in deduxit's own layout. A taller
    /// label makes it taller.
    pub band: Length,
    /// The thickness of an inference line in deduxit's own layout.
    pub stroke: Length,
    /// The derivation's lambda-term, written under its conclusion as its
    /// typing judgement ([`lambda`](crate::lambda)), with these options.
    /// `None`, the default, writes no term.
    ///
    /// The term is in Typst's notation, whatever
    /// [`notation`](crate::lambda::Options::notation) says. Before anything
    /// is written, [`write()`] refuses a derivation that has no term
    /// ([`Error::NoTerm`]), and a term whose text would pass the default
    /// output bound
    /// ([`Limits::DEFAULT_DERIVATION_BYTES`](crate::Limits::DEFAULT_DERIVATION_BYTES)).
    pub lambda: Option<crate::lambda::Options>,
}

impl Default for Options {
    /// Returns the options of a fragment with upright labels, an open goal
    /// under vertical dots, the import of curryst [`CURRYST`], the page
    /// [`PAGE`], curryst up to its height and deduxit's own layout above
    /// it, curryst's spacing and line, and no lambda-term.
    fn default() -> Self {
        Self {
            form: Form::Fragment,
            labels: Labels::Upright,
            open: OpenGoal::Dots,
            import: format!("#import \"@preview/curryst:{CURRYST}\": prooftree, rule"),
            page: PAGE.to_owned(),
            layout: Layout::Auto,
            premise_gap: Length("1.5em".into()),
            label_gap: Length("0.2em".into()),
            band: Length("0.8em".into()),
            stroke: Length("0.05em".into()),
            lambda: None,
        }
    }
}

impl Options {
    /// Returns the options with another [`form`](Self::form).
    #[must_use]
    pub fn with_form(self, form: Form) -> Self {
        Self { form, ..self }
    }

    /// Returns the options with another [`labels`](Self::labels).
    #[must_use]
    pub fn with_labels(self, labels: Labels) -> Self {
        Self { labels, ..self }
    }

    /// Returns the options with another [`open`](Self::open).
    #[must_use]
    pub fn with_open(self, open: OpenGoal) -> Self {
        Self { open, ..self }
    }

    /// Returns the options with another [`import`](Self::import).
    #[must_use]
    pub fn with_import(self, import: String) -> Self {
        Self { import, ..self }
    }

    /// Returns the options with another [`page`](Self::page).
    #[must_use]
    pub fn with_page(self, page: String) -> Self {
        Self { page, ..self }
    }

    /// Returns the options with another [`layout`](Self::layout).
    #[must_use]
    pub fn with_layout(self, layout: Layout) -> Self {
        Self { layout, ..self }
    }

    /// Returns the options with another [`premise_gap`](Self::premise_gap).
    #[must_use]
    pub fn with_premise_gap(self, premise_gap: Length) -> Self {
        Self {
            premise_gap,
            ..self
        }
    }

    /// Returns the options with another [`label_gap`](Self::label_gap).
    #[must_use]
    pub fn with_label_gap(self, label_gap: Length) -> Self {
        Self { label_gap, ..self }
    }

    /// Returns the options with another [`band`](Self::band).
    #[must_use]
    pub fn with_band(self, band: Length) -> Self {
        Self { band, ..self }
    }

    /// Returns the options with another [`stroke`](Self::stroke).
    #[must_use]
    pub fn with_stroke(self, stroke: Length) -> Self {
        Self { stroke, ..self }
    }
}

/// Writes a rule label's markup in math mode: symbols as their
/// characters, one letter as `upright(L)`, longer text as a string, and
/// subscripts after `_`. Text that follows something gets a space before
/// it.
fn label(out: &mut String, markup: &str) {
    let text = |out: &mut String, text: &str| {
        let mut chars = text.chars();
        if let (Some(c), None) = (chars.next(), chars.next())
            && c.is_alphabetic()
        {
            out.push_str("upright(");
            out.push(c);
            out.push(')');
            return;
        }
        out.push('"');
        for c in text.chars() {
            if c == '"' || c == '\\' {
                out.push('\\');
            }
            out.push(c);
        }
        out.push('"');
    };
    let start = out.len();
    for part in parts(markup) {
        match part {
            Part::Symbol(c) => out.push_str(match c {
                '&' => r"\&",
                '?' => NOTATION.quest,
                '1' => NOTATION.one,
                '⊗' => "⊗",
                '⅋' => "⅋",
                '⊕' => "⊕",
                '⊸' => "⊸",
                '\\' => "backslash",
                '/' => "slash",
                '!' => "!",
                '⊤' => "⊤",
                '⊥' => "⊥",
                '*' => "^*",
                '∧' => "∧",
                '∨' => "∨",
                '→' => "→",
                '¬' => "¬",
                '↔' => "↔",
                '∀' => "forall",
                '∃' => "exists",
                _ => "0",
            }),
            Part::Text(t) => {
                if out.len() > start {
                    out.push(' ');
                }
                text(out, t);
            }
            Part::Sub(t) if t.len() == 1 && t.chars().all(|c| c.is_ascii_digit()) => {
                out.push('_');
                out.push_str(t);
            }
            Part::Sub(t) => {
                out.push_str("_(");
                text(out, t);
                out.push(')');
            }
        }
    }
}

/// Returns `math` in the options' form: as it is for a fragment, or after
/// the page setup for a standalone document.
fn formed(math: String, options: &Options) -> String {
    match options.form {
        Form::Fragment => math,
        Form::Standalone => format!("{}\n\n{math}", options.page.trim_end()),
    }
}

/// The bytes a sequent's text is estimated at per occurrence, besides
/// its longest atom name: at least what any connective, its brackets and
/// its separator take.
const PER_OCCURRENCE: u64 = 32;

/// Returns a sequent as Typst math between `$`s, or a standalone document
/// that holds it, as [`Options::form`] says.
///
/// In an intuitionistic mode the sequent is two-sided, as its
/// intuitionistic reading has it. In any other mode it is one-sided.
///
/// # Errors
///
/// - [`Refusal::Output`](crate::Refusal::Output) for a sequent whose text
///   is estimated past `limits.derivation_bytes`, before anything is laid
///   out. The estimate is 32 bytes plus the longest atom name for each
///   occurrence of a subformula.
/// - In an intuitionistic mode, [`Error::NotIntuitionistic`] for a
///   sequent that has no intuitionistic reading, and the refusal of
///   [`Forest::within`](crate::linear::Forest::within).
pub fn sequent(
    sequent: &Sequent,
    mode: crate::linear::Mode,
    options: &Options,
    limits: &crate::Limits,
) -> Result<String, Error> {
    super::math(sequent, mode, PER_OCCURRENCE, limits, &NOTATION).map(|out| formed(out, options))
}

/// Writes a derivation as a Typst proof tree into `out`, one inference at
/// a time, and asks `stop` after each.
///
/// For the whole text at once, use [`to_string`]. This function is for
/// writing into your own buffer, or for stopping a large export.
///
/// [`Options::layout`] picks the code:
/// - curryst's is a `#prooftree(…)` call. A rule with premises spans
///   several lines, indented by its depth, and a leaf is one line.
/// - deduxit's own is a `#context` block. It lists the inferences first,
///   one line each, from the root in preorder (each inference before its
///   premises), then the code that sets them.
///
/// With [`Options::lambda`] set, the tree and the judgement under it stand
/// in a `#stack`.
///
/// The output is a fragment or a document, as [`Options::form`] says. A
/// linear derivation is one- or two-sided, as it is. A derivation of LK or
/// LJ is two-sided. Its connectives and constants are the Unicode
/// characters `¬ ∧ ∨ → ↔ ⊤ ⊥`, and its labels those of
/// [`ordinary::Rule`](crate::ordinary::Rule). A [`Labels::Table`] is
/// keyed by the linear rules, so it leaves the labels of LK and LJ
/// upright.
///
/// # Errors
///
/// - [`Refusal::Stopped`](crate::Refusal::Stopped) when `stop` fired.
/// - [`Error::WriteFailed`] when `out` refused the text.
/// - With [`Options::lambda`] set, before anything is written:
///   [`Error::NoTerm`] for a derivation that has no lambda-term, and
///   [`Refusal::Output`](crate::Refusal::Output) for a term whose text is
///   estimated past the default bound.
pub fn write<'a>(
    derivation: impl Into<Drawable<'a>>,
    options: &Options,
    out: &mut impl Write,
    stop: impl FnMut(crate::limits::Progress) -> bool,
) -> Result<(), Error> {
    let mut stop = stop;
    let drawable = derivation.into();
    let judgement = super::judgement(
        drawable,
        options.lambda.as_ref(),
        &crate::lambda::print::TYPST,
        &mut stop,
    )?;
    let judgement = judgement.as_deref();
    let stop = crate::limits::counting(stop, crate::limits::Phase::Write);
    match drawable {
        Drawable::Linear(derivation) => tree(derivation, options, judgement, out, stop),
        Drawable::Ordinary(derivation) => tree(derivation, options, judgement, out, stop),
        Drawable::Classical(view) => tree(view, options, judgement, out, stop),
    }
}

/// Returns a derivation as Typst, whole, as [`write()`] writes it.
///
/// It takes no stop. The text grows linearly with the derivation, which
/// was made within its own limits
/// ([`Proof::derivation`](crate::linear::Proof::derivation) uses the
/// default ones). A lambda-term written under it is bounded by the default
/// [`Limits`](crate::Limits).
///
/// # Errors
///
/// As [`write()`], but never [`Refusal::Stopped`](crate::Refusal::Stopped)
/// or [`Error::WriteFailed`]: there is no stop, and a `String` takes any
/// text.
pub fn to_string<'a>(
    derivation: impl Into<Drawable<'a>>,
    options: &Options,
) -> Result<String, Error> {
    let mut out = String::new();
    write(derivation, options, &mut out, |_| false)?;
    Ok(out)
}

/// Writes any derivation as [`write()`] does, in the layout the options
/// choose, with the judgement given, if any, under its conclusion.
fn tree<T: Drawn>(
    derivation: &T,
    options: &Options,
    judgement: Option<&str>,
    out: &mut impl Write,
    stop: impl FnMut() -> bool,
) -> Result<(), Error> {
    let own = match options.layout {
        Layout::Curryst => false,
        Layout::Deduxit => true,
        Layout::Auto => height(derivation) > CURRYST_HEIGHT,
    };
    if own {
        laid_out(derivation, options, judgement, out, stop)
    } else {
        curryst(derivation, options, judgement, out, stop)
    }
}

/// Returns the number of inferences on the longest branch of a
/// derivation.
fn height<T: Drawn>(derivation: &T) -> usize {
    // Premises come before their conclusions, the root last.
    let mut heights = vec![0; derivation.len()];
    for i in 0..heights.len() {
        let premises = derivation.premises(crate::linear::InfId::new(i as u32));
        let above = premises.iter().map(|p| heights[p.index()]);
        heights[i] = 1 + above.max().unwrap_or(0);
    }
    heights.last().copied().unwrap_or(0)
}

/// The Typst code of deduxit's own layout, which ends the `#context`
/// block after the spacing and the inferences.
///
/// The inferences are `nodes`: each its number of premises, its label and
/// its conclusion, from the root in preorder. An open goal has `none` for
/// its premises and is drawn as `open` says, or, under a mark, is a leaf
/// labelled with the mark.
///
/// The code measures every sequent and label and lays the tree out as the
/// SVG drawing does. Premises stand side by side, `gap` apart, with their
/// conclusions centred over the conclusion below. The inference line spans
/// both, with the label after it. Every piece is placed in one box:
/// nothing is nested per level, so no height of tree reaches a limit of
/// Typst's.
///
/// The sequents of a row share a baseline. A sequent's ascent is its
/// height less its depth, and its depth is what a strut of height `tall`
/// before it adds to the height.
const LAYOUT: &str = r#"  let n = nodes.len()
  let tall = (3em).to-absolute()
  let metrics(c) = {
    let size = measure(box(c))
    let low = measure(box[#box(height: tall, width: 0pt)#c]).height - tall
    (size.width, size.height - low, low)
  }
  let (dots-width, dots-up, dots-down) = metrics(dots)
  let (width, up, down, wide, left, offset) = ((0pt,) * n,) * 6
  let bar = (none,) * n
  let names = (none,) * n
  let stack = ()
  for i in range(n - 1, -1, step: -1) {
    let (k, name, c) = nodes.at(i)
    let (w, a, d) = metrics(c)
    width.at(i) = w
    up.at(i) = a
    down.at(i) = d
    if k == none {
      let reach = if open == "dots" { dots-width } else { 0pt }
      wide.at(i) = calc.max(w, reach)
      left.at(i) = calc.max(reach - w, 0pt) / 2
      if open == "dashed" { bar.at(i) = (left.at(i), left.at(i) + w) }
      stack.push(i)
      continue
    }
    let kids = ()
    for _ in range(k) { kids.push(stack.pop()) }
    let x = 0pt
    let span = none
    for p in kids {
      offset.at(p) = x
      let s = x + left.at(p)
      span = (if span == none { s } else { span.at(0 ) }, s + width.at(p))
      x += wide.at(p) + gap
    }
    let (l, b) = if span == none { (0pt, (0pt, w)) } else {
      let l = (span.at(0) + span.at(1) - w) / 2
      (l, (calc.min(span.at(0), l), calc.max(span.at(1), l + w)))
    }
    let shift = calc.max(-l, 0pt)
    for p in kids { offset.at(p) += shift }
    let named = 0pt
    if name != none {
      let size = measure(box(name))
      names.at(i) = (size.width, size.height)
      named = name-gap + size.width
    }
    let row = if k == 0 { 0pt } else { x - gap }
    wide.at(i) = calc.max(row, b.at(1) + named) + shift
    left.at(i) = l + shift
    bar.at(i) = (b.at(0) + shift, b.at(1) + shift)
    stack.push(i)
  }
  let (x, depth) = ((0pt,) * n, (0,) * n)
  let parents = ()
  for i in range(n) {
    if parents.len() > 0 {
      let (p, r) = parents.last()
      x.at(i) = x.at(p) + offset.at(i)
      depth.at(i) = depth.at(p) + 1
      if r == 1 { let _ = parents.pop() } else { parents.at(-1) = (p, r - 1) }
    }
    let k = nodes.at(i).at(0)
    if k != none and k > 0 { parents.push((i, k)) }
  }
  let rows = calc.max(..depth) + 2
  let (row-up, row-down, between) = ((0pt,) * rows,) * 3
  for i in range(n) {
    let j = depth.at(i)
    row-up.at(j) = calc.max(row-up.at(j), up.at(i))
    row-down.at(j) = calc.max(row-down.at(j), down.at(i))
    if bar.at(i) != none {
      let tall = if names.at(i) == none { 0pt } else { names.at(i).at(1) }
      between.at(j) = calc.max(between.at(j), band, tall)
    }
    if nodes.at(i).at(0) == none and open == "dots" {
      between.at(j) = calc.max(between.at(j), band / 2)
      row-up.at(j + 1) = calc.max(row-up.at(j + 1), dots-up)
      row-down.at(j + 1) = calc.max(row-down.at(j + 1), dots-down)
    }
  }
  let base = (0pt,) * rows
  base.at(rows - 1) = row-up.at(rows - 1)
  for j in range(rows - 2, -1, step: -1) {
    base.at(j) = base.at(j + 1) + row-down.at(j + 1) + between.at(j) + row-up.at(j)
  }
  box(width: wide.at(0), height: base.at(0) + row-down.at(0), baseline: row-down.at(0), {
    for i in range(n) {
      let (k, name, c) = nodes.at(i)
      let (j, at) = (depth.at(i), x.at(i))
      place(dx: at + left.at(i), dy: base.at(j) - up.at(i), box(c))
      if k == none and open == "dots" {
        let dx = at + (wide.at(i) - dots-width) / 2
        place(dx: dx, dy: base.at(j + 1) - dots-up, box(dots))
      }
      if bar.at(i) != none {
        let (start, end) = bar.at(i)
        let y = base.at(j) - row-up.at(j) - between.at(j) / 2
        let dash = if k == none { "dashed" } else { none }
        place(dx: at + start, dy: y, line(length: end - start, stroke: (thickness: stroke, dash: dash)))
        if names.at(i) != none {
          let (w, h) = names.at(i)
          place(dx: at + end + name-gap, dy: y - h / 2, box(name))
        }
      }
    }
  })
}
"#;

/// Writes a derivation in deduxit's own layout into `out`, one inference
/// at a time, and asks `stop` after each. With a judgement, the tree and
/// the judgement under it stand in a stack.
fn laid_out<T: Drawn>(
    derivation: &T,
    options: &Options,
    judgement: Option<&str>,
    out: &mut impl Write,
    mut stop: impl FnMut() -> bool,
) -> Result<(), Error> {
    if options.form == Form::Standalone {
        write!(out, "{}\n\n", options.page.trim_end())?;
    }
    if judgement.is_some() {
        out.write_str(STACK)?;
    } else {
        out.write_char('#')?;
    }
    let open = match options.open {
        OpenGoal::Dots => "dots",
        OpenGoal::Dashed => "dashed",
        OpenGoal::Bare | OpenGoal::Mark(_) => "bare",
    };
    write!(
        out,
        "context {{\n  let open = \"{open}\"\n  let gap = ({}).to-absolute()\n  \
         let name-gap = ({}).to-absolute()\n  let band = ({}).to-absolute()\n  \
         let stroke = {}\n  let dots = $dots.v$\n  let nodes = (\n",
        options.premise_gap, options.label_gap, options.band, options.stroke
    )?;
    let mut buffer = String::new();
    walk(derivation, |step| {
        let Step::Enter(id, _) = step else {
            return Ok(());
        };
        let markup = derivation.label(id, &options.labels);
        let (premises, name) = match (derivation.is_open(id), &options.open) {
            (true, OpenGoal::Mark(mark)) => ("0".to_owned(), Some(mark.as_str())),
            (true, _) => ("none".to_owned(), None),
            _ => (derivation.premises(id).len().to_string(), markup.as_deref()),
        };
        write!(buffer, "    ({premises}, ")?;
        match name {
            Some(name) => {
                buffer.push('$');
                label(&mut buffer, name);
                buffer.push('$');
            }
            None => buffer.push_str("none"),
        }
        buffer.push_str(", $");
        derivation.sequent(&mut buffer, &NOTATION, id, false, false);
        buffer.push_str("$),\n");
        flush(out, &mut buffer, &mut stop)
    })?;
    out.write_str("  )\n")?;
    out.write_str(LAYOUT.trim_end())?;
    under(out, judgement)
}

/// The code that opens the stack of a tree and the judgement under it.
const STACK: &str = "#stack(spacing: 0.8em, align(center, ";

/// Writes the judgement given under the tree and closes the stack that
/// [`STACK`] opened. Without a judgement it writes nothing.
fn under(out: &mut impl Write, judgement: Option<&str>) -> Result<(), Error> {
    if let Some(judgement) = judgement {
        write!(out, "), align(center, ${judgement}$))")?;
    }
    Ok(())
}

/// Writes a derivation as a curryst tree into `out`, one inference at a
/// time, and asks `stop` after each. With a judgement, the tree and the
/// judgement under it stand in a stack.
fn curryst<T: Drawn>(
    derivation: &T,
    options: &Options,
    judgement: Option<&str>,
    out: &mut impl Write,
    mut stop: impl FnMut() -> bool,
) -> Result<(), Error> {
    if options.form == Form::Standalone {
        write!(
            out,
            "{}\n{}\n\n",
            options.import.trim_end(),
            options.page.trim_end()
        )?;
    }
    out.write_str(if judgement.is_some() { STACK } else { "#" })?;
    out.write_str("prooftree(\n")?;
    let indent = |out: &mut String, depth: usize| {
        for _ in 0..depth {
            out.push_str("  ");
        }
    };
    let conclusion = |out: &mut String, id| {
        out.push('$');
        derivation.sequent(out, &NOTATION, id, false, false);
        out.push('$');
    };
    let mut buffer = String::new();
    walk(derivation, |step| {
        match step {
            Step::Enter(id, depth) => {
                let premises = derivation.premises(id);
                indent(&mut buffer, depth + 1);
                // A grid of one column is as wide as the sequent, and so
                // is a line across it.
                let open = |out: &mut String, grid: &str| {
                    out.push_str(grid);
                    conclusion(out, id);
                    out.push_str("),\n");
                };
                match (derivation.is_open(id), &options.open) {
                    (true, OpenGoal::Dots) => open(
                        &mut buffer,
                        "grid(align: center, row-gutter: 0.4em, $dots.v$, ",
                    ),
                    (true, OpenGoal::Dashed) => open(
                        &mut buffer,
                        "grid(align: center, inset: (top: 0.3em), \
                         grid.hline(stroke: (dash: \"dashed\")), ",
                    ),
                    (true, OpenGoal::Bare) => {
                        conclusion(&mut buffer, id);
                        buffer.push_str(",\n");
                    }
                    (true, OpenGoal::Mark(mark)) => {
                        buffer.push_str("rule(name: $");
                        label(&mut buffer, mark);
                        buffer.push_str("$, ");
                        conclusion(&mut buffer, id);
                        buffer.push_str("),\n");
                    }
                    _ => {
                        let markup = derivation.label(id, &options.labels);
                        let markup = markup.as_deref();
                        let name = |out: &mut String| {
                            if let Some(markup) = markup {
                                out.push_str("name: $");
                                label(out, markup);
                                out.push('$');
                            }
                        };
                        buffer.push_str("rule(");
                        if premises.is_empty() {
                            name(&mut buffer);
                            if markup.is_some() {
                                buffer.push_str(", ");
                            }
                            conclusion(&mut buffer, id);
                            buffer.push_str("),\n");
                        } else if markup.is_some() {
                            buffer.push('\n');
                            indent(&mut buffer, depth + 2);
                            name(&mut buffer);
                            buffer.push_str(",\n");
                        } else {
                            buffer.push('\n');
                        }
                    }
                }
            }
            Step::Exit(id, depth) => {
                if !derivation.premises(id).is_empty() {
                    indent(&mut buffer, depth + 2);
                    conclusion(&mut buffer, id);
                    buffer.push_str(",\n");
                    indent(&mut buffer, depth + 1);
                    buffer.push_str("),\n");
                }
            }
        }
        flush(out, &mut buffer, &mut stop)
    })?;
    out.write_char(')')?;
    under(out, judgement)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns an atom's name as the Typst export writes it.
    fn escaped(name: &str) -> String {
        let mut out = String::new();
        atom(&mut out, name).unwrap();
        out
    }

    /// A length is a number and a unit, and nothing else reaches the
    /// output as code.
    #[test]
    fn lengths_are_numbers_and_units() {
        for text in ["1.5em", "0pt", "12mm", "2.25in", "3cm"] {
            assert_eq!(Length::new(text).unwrap().as_str(), text);
        }
        for text in [
            "", "em", "1", "1.em", ".5em", "1.5 em", "-1em", "1em)", "1fr", "1em+2pt",
        ] {
            assert!(
                matches!(
                    Length::new(text),
                    Err(Error::InvalidOption { key: "typst", .. })
                ),
                "{text:?}"
            );
        }
    }

    /// One letter stays as it is. Every other name is an italic string,
    /// with quotes and backslashes escaped.
    #[test]
    fn names_are_escaped() {
        for (name, expected) in [
            ("A", "A"),
            ("α", "α"),
            ("foo", r#"italic("foo")"#),
            ("x_1", r#"italic("x_1")"#),
            (r#"a"b\c"#, r#"italic("a\"b\\c")"#),
        ] {
            assert_eq!(escaped(name), expected, "{name:?}");
        }
    }
}
