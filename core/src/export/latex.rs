// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Writes sequents and proofs as LaTeX: formulas in math mode, and proofs
//! as proof trees of the `ebproof` package.
//!
//! | to get | call |
//! |---|---|
//! | a proof tree | [`to_string`] |
//! | a proof tree, written into your own buffer with a stop | [`write()`] |
//! | a sequent alone | [`sequent`] |
//!
//! [`Options`] holds the choices: a fragment or a whole document, the rule
//! labels, how an open goal looks, and the preamble.
//!
//! Needs the cargo feature `latex` (on by default).
//!
//! # Examples
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::export::latex;
//! use deduxit::{Limits, linear::{Mode, Options, Sequent, Verdict, prove}};
//!
//! let sequent: Sequent = "A, A -o B |- B".parse()?;
//! let outcome = prove(&sequent, Mode::INTUITIONISTIC, &Options::default())?;
//! let Verdict::Proved(proof) = &outcome.verdict else {
//!     panic!("provable");
//! };
//! let options = latex::Options::default();
//! let tree = latex::to_string(&proof.derivation()?, &options)?;
//! assert_eq!(
//!     tree,
//!     r"\begin{prooftree}
//! \infer0[$\mathrm{ax}$]{A &\vdash A}
//! \infer0[$\mathrm{ax}$]{B &\vdash B}
//! \infer2[$\multimap\mathrm{L}$]{A, A \multimap B &\vdash B}
//! \end{prooftree}"
//! );
//!
//! // The sequent alone, one-sided as classical linear logic writes it.
//! let printed = latex::sequent(&sequent, Mode::CLASSICAL, &options, &Limits::default())?;
//! assert_eq!(printed, r"$\vdash A^\bot, A \otimes B^\bot, B$");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # What the output needs
//!
//! By default the output is a fragment: a sequent between `$`s, or a
//! `prooftree` environment. Paste it into a document that loads the
//! packages `amssymb`, `cmll` and `ebproof`. With [`Form::Standalone`] it
//! is a document of the `standalone` class, which crops the page to its
//! content. It compiles with pdfLaTeX.
//!
//! The output chooses no font. It takes the fonts of the document it goes
//! into.
//!
//! # How it is written
//!
//! - The connectives are those of `cmll` and `amssymb`: `\otimes`,
//!   `\parr`, `\with`, `\oplus`, `\multimap`, `\oc`, `\wn`, `\mathbf{1}`,
//!   `\bot`, `\top`, `0`, `A^\bot` and `\vdash`.
//! - A proof tree has one ebproof statement per inference, each premise
//!   before its conclusion.
//! - In a two-sided sequent the turnstiles line up down the tree, with
//!   ebproof's `&`, unless [`Options::align`] is off. A one-sided sequent
//!   would line up at its left edge, so it is centred instead.
//!
//! # Atom names
//!
//! - A name of one ASCII letter is written as it is, in math italic.
//! - A name of one Greek letter is its command, such as `\alpha` or
//!   `\Gamma`. A Greek letter that LaTeX has no command for, such as a
//!   capital alpha, is the Latin letter it looks like.
//! - Any other name goes into `\mathit{…}`. There `{ } $ # % & _` take a
//!   backslash, `^`, `~` and `\` are written as text, and a space is kept.
//!   A Greek letter is its command in braces. The marks `‿` and `·`, which
//!   `lltp::read` puts into the names of the LLTP library's problems, are
//!   `\smallsmile` and `\cdotp`.
//!
//! So names of ASCII letters, of Greek letters and of the LLTP library
//! compile with pdfLaTeX. Any other character is written as it is. It
//! needs a Unicode engine, such as LuaLaTeX with `unicode-math`, or a
//! declaration of the character for pdfLaTeX.

use super::notation::{Step, flush, walk};
use super::{Drawable, Form};
use crate::Error;
use crate::linear::proofs::style::{Drawn, Part, parts};
use crate::linear::proofs::{Labels, OpenGoal};
use crate::linear::sequents::Sequent;
use crate::linear::sequents::notation::{Atoms, Notation};
use crate::ordinary::Symbols;
use std::fmt::Write;

/// The LaTeX spelling of formulas and sequents.
const NOTATION: Notation = Notation {
    tensor: r"\otimes",
    par: r"\parr",
    with: r"\with",
    plus: r"\oplus",
    lollipop: r"\multimap",
    under: r"\mathbin{\backslash}",
    over: r"\mathbin{/}",
    bang: r"\oc ",
    quest: r"\wn ",
    one: r"\mathbf{1}",
    bot: r"\bot",
    top: r"\top",
    zero: "0",
    dual_prefix: "",
    dual: r"^\bot",
    turnstile: r"\vdash",
    align: "&",
    atom: Atoms::Latex,
    forall: r"\forall ",
    exists: r"\exists ",
    dot: r".\,",
    ordinary: Symbols {
        not: r"\lnot ",
        and: r"\land",
        or: r"\lor",
        implies: r"\to",
        iff: r"\leftrightarrow",
        truth: r"\top",
        falsity: r"\bot",
    },
};

/// The preamble of a standalone document. It loads `amssymb` for
/// `\multimap`, and `cmll` for `\parr`, `\with`, `\oc` and `\wn`.
const PREAMBLE: &str = r"\documentclass[border=5pt]{standalone}
\usepackage{amssymb}
\usepackage{cmll}
";

/// Returns the math-mode spelling of a Greek letter: its command, or, for
/// a letter LaTeX has no command for (omicron, and the capitals shaped like
/// Latin letters), the Latin letter it looks like. Anything else is
/// `None`.
fn greek(c: char) -> Option<&'static str> {
    Some(match c {
        'α' => r"\alpha",
        'β' => r"\beta",
        'γ' => r"\gamma",
        'δ' => r"\delta",
        'ε' => r"\varepsilon",
        'ϵ' => r"\epsilon",
        'ζ' => r"\zeta",
        'η' => r"\eta",
        'θ' => r"\theta",
        'ϑ' => r"\vartheta",
        'ι' => r"\iota",
        'κ' => r"\kappa",
        'λ' => r"\lambda",
        'μ' => r"\mu",
        'ν' => r"\nu",
        'ξ' => r"\xi",
        'ο' => "o",
        'π' => r"\pi",
        'ϖ' => r"\varpi",
        'ρ' => r"\rho",
        'ϱ' => r"\varrho",
        'σ' => r"\sigma",
        'ς' => r"\varsigma",
        'τ' => r"\tau",
        'υ' => r"\upsilon",
        'φ' => r"\varphi",
        'ϕ' => r"\phi",
        'χ' => r"\chi",
        'ψ' => r"\psi",
        'ω' => r"\omega",
        'Γ' => r"\Gamma",
        'Δ' => r"\Delta",
        'Θ' => r"\Theta",
        'Λ' => r"\Lambda",
        'Ξ' => r"\Xi",
        'Π' => r"\Pi",
        'Σ' => r"\Sigma",
        'Υ' => r"\Upsilon",
        'Φ' => r"\Phi",
        'Ψ' => r"\Psi",
        'Ω' => r"\Omega",
        'Α' => "A",
        'Β' => "B",
        'Ε' => "E",
        'Ζ' => "Z",
        'Η' => "H",
        'Ι' => "I",
        'Κ' => "K",
        'Μ' => "M",
        'Ν' => "N",
        'Ο' => "O",
        'Ρ' => "P",
        'Τ' => "T",
        'Χ' => "X",
        _ => return None,
    })
}

/// Writes an atom's name in math mode: one ASCII letter as it is, one
/// Greek letter as [`greek`] spells it, and any other name in
/// `\mathit{…}`, escaped.
pub(crate) fn atom<W: Write + ?Sized>(out: &mut W, name: &str) -> std::fmt::Result {
    let mut chars = name.chars();
    if let (Some(c), None) = (chars.next(), chars.next()) {
        if c.is_ascii_alphabetic() {
            return out.write_char(c);
        }
        if let Some(letter) = greek(c) {
            return out.write_str(letter);
        }
    }
    out.write_str(r"\mathit{")?;
    for c in name.chars() {
        escape(out, c)?;
    }
    out.write_char('}')
}

/// Writes a character of text in math mode: a character TeX treats
/// specially escaped or as text, a Greek letter as its command, and any
/// other character as it is.
fn escape(out: &mut (impl Write + ?Sized), c: char) -> std::fmt::Result {
    match c {
        '\\' => out.write_str(r"\mbox{\textbackslash}"),
        '{' | '}' | '$' | '#' | '%' | '&' | '_' => {
            out.write_char('\\')?;
            out.write_char(c)
        }
        '^' => out.write_str(r"\mbox{\textasciicircum}"),
        '~' => out.write_str(r"\mbox{\textasciitilde}"),
        ' ' => out.write_str(r"\ "),
        // `lltp::HYPHEN` and `lltp::DOT`, spelt out because `lltp` needs
        // the `parse` feature.
        '‿' => out.write_str(r"{\smallsmile}"),
        '·' => out.write_str(r"{\cdotp}"),
        // Braces keep a letter that follows from joining the command's
        // name.
        c => match greek(c) {
            Some(letter) if letter.starts_with('\\') => write!(out, "{{{letter}}}"),
            Some(letter) => out.write_str(letter),
            None => out.write_char(c),
        },
    }
}

/// The choices of the LaTeX output.
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
/// use deduxit::export::{Form, latex};
/// use deduxit::{Limits, linear::{Mode, Sequent}};
///
/// let sequent: Sequent = "A |- A".parse()?;
/// let options = latex::Options::default().with_form(Form::Standalone);
/// let document = latex::sequent(&sequent, Mode::CLASSICAL, &options, &Limits::default())?;
/// assert_eq!(
///     document,
///     r"\documentclass[border=5pt]{standalone}
/// \usepackage{amssymb}
/// \usepackage{cmll}
/// \begin{document}
/// $\vdash A^\bot, A$
/// \end{document}"
/// );
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
    /// How an open goal is drawn: under `\vdots`, bare (`\hypo`), under a
    /// line labelled with a mark, or under ebproof's dashed line.
    pub open: OpenGoal,
    /// Whether the turnstiles of a two-sided tree line up (ebproof's `&`).
    pub align: bool,
    /// The options of every `prooftree` environment, as ebproof reads
    /// them between brackets. Empty for none.
    pub ebproof: String,
    /// What a standalone document starts with, before
    /// `\begin{document}`.
    ///
    /// `None` gives the `standalone` class, cropped to the content, with
    /// `amssymb`, `cmll` and, for a tree, `ebproof`. A preamble of your
    /// own must load those packages itself.
    pub preamble: Option<String>,
    /// The derivation's lambda-term, written under its conclusion as its
    /// typing judgement ([`lambda`](crate::lambda)), with these options.
    /// `None`, the default, writes no term.
    ///
    /// The term is in LaTeX's notation, whatever
    /// [`notation`](crate::lambda::Options::notation) says. Before anything
    /// is written, [`write()`] refuses a derivation that has no term
    /// ([`Error::NoTerm`]), and a term whose text would pass the default
    /// output bound
    /// ([`Limits::DEFAULT_DERIVATION_BYTES`](crate::Limits::DEFAULT_DERIVATION_BYTES)).
    pub lambda: Option<crate::lambda::Options>,
}

impl Default for Options {
    /// Returns the options of a fragment with upright labels, an open goal
    /// under vertical dots, aligned turnstiles, no ebproof options, the
    /// `standalone` preamble and no lambda-term.
    fn default() -> Self {
        Self {
            form: Form::Fragment,
            labels: Labels::Upright,
            open: OpenGoal::Dots,
            align: true,
            ebproof: String::new(),
            preamble: None,
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

    /// Returns the options with another [`align`](Self::align).
    #[must_use]
    pub fn with_align(self, align: bool) -> Self {
        Self { align, ..self }
    }

    /// Returns the options with another [`ebproof`](Self::ebproof).
    #[must_use]
    pub fn with_ebproof(self, ebproof: String) -> Self {
        Self { ebproof, ..self }
    }

    /// Returns the options with another [`preamble`](Self::preamble).
    #[must_use]
    pub fn with_preamble(self, preamble: Option<String>) -> Self {
        Self { preamble, ..self }
    }
}

/// Writes a rule label's markup in math mode: symbols as the
/// connectives, text in `\mathrm`, subscripts after `_`.
fn label(out: &mut String, markup: &str) {
    let text = |out: &mut String, text: &str| {
        out.push_str(r"\mathrm{");
        for c in text.chars() {
            escape(out, c).unwrap();
        }
        out.push('}');
    };
    for part in parts(markup) {
        match part {
            Part::Symbol(c) => out.push_str(match c {
                '⊗' => NOTATION.tensor,
                '⅋' => NOTATION.par,
                '&' => NOTATION.with,
                '⊕' => NOTATION.plus,
                '⊸' => NOTATION.lollipop,
                '\\' => r"\backslash",
                '/' => "/",
                '!' => r"\oc",
                '?' => r"\wn",
                '⊤' => NOTATION.top,
                '⊥' => NOTATION.bot,
                '1' => NOTATION.one,
                '*' => "^{*}",
                '∧' => NOTATION.ordinary.and,
                '∨' => NOTATION.ordinary.or,
                '→' => NOTATION.ordinary.implies,
                '¬' => r"\lnot",
                '↔' => NOTATION.ordinary.iff,
                '∀' => r"\forall",
                '∃' => r"\exists",
                _ => NOTATION.zero,
            }),
            Part::Text(t) => text(out, t),
            Part::Sub(t) if t.len() == 1 && t.chars().all(|c| c.is_ascii_digit()) => {
                out.push('_');
                out.push_str(t);
            }
            Part::Sub(t) => {
                out.push_str("_{");
                text(out, t);
                out.push('}');
            }
        }
    }
}

/// Writes the start of a standalone document, with `ebproof` loaded if
/// `tree` is set.
fn begin(out: &mut impl Write, options: &Options, tree: bool) -> std::fmt::Result {
    match &options.preamble {
        Some(preamble) => writeln!(out, "{}", preamble.trim_end())?,
        None => {
            out.write_str(PREAMBLE)?;
            if tree {
                out.write_str("\\usepackage{ebproof}\n")?;
            }
        }
    }
    out.write_str("\\begin{document}\n")
}

/// Returns `math` in the options' form: as it is for a fragment, or in a
/// standalone document.
fn formed(math: String, options: &Options) -> String {
    match options.form {
        Form::Fragment => math,
        Form::Standalone => {
            let mut out = String::new();
            begin(&mut out, options, false).expect("a string takes any text");
            out.push_str(&math);
            out.push_str("\n\\end{document}");
            out
        }
    }
}

/// The bytes a sequent's text is estimated at per occurrence, besides
/// its longest atom name: at least what any connective, its brackets and
/// its separator take.
const PER_OCCURRENCE: u64 = 32;

/// Returns a sequent as LaTeX math between `$`s, or a standalone document
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

/// Writes a derivation as an ebproof `prooftree` environment into `out`,
/// one inference at a time, and asks `stop` after each.
///
/// For the whole text at once, use [`to_string`]. This function is for
/// writing into your own buffer, or for stopping a large export.
///
/// The output is a fragment or a document, as [`Options::form`] says. A
/// linear derivation is one- or two-sided, as it is. A derivation of LK or
/// LJ is two-sided, with the turnstiles lined up unless [`Options::align`]
/// is off. Its connectives are `\lnot`, `\land`, `\lor`, `\to` and
/// `\leftrightarrow`, its constants `\top` and `\bot`, and its labels
/// those of [`ordinary::Rule`](crate::ordinary::Rule). A
/// [`Labels::Table`] is keyed by the linear rules, so it leaves the labels
/// of LK and LJ upright.
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
        &crate::lambda::print::LATEX,
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

/// Returns a derivation as LaTeX, whole, as [`write()`] writes it.
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

/// Writes any derivation as [`write()`] does, with the judgement given,
/// if any, under its conclusion.
fn tree<T: Drawn>(
    derivation: &T,
    options: &Options,
    judgement: Option<&str>,
    out: &mut impl Write,
    mut stop: impl FnMut() -> bool,
) -> Result<(), Error> {
    let standalone = options.form == Form::Standalone;
    if standalone {
        begin(out, options, true)?;
    }
    out.write_str("\\begin{prooftree}")?;
    if !options.ebproof.is_empty() {
        write!(out, "[{}]", options.ebproof)?;
    }
    out.write_char('\n')?;
    let mut buffer = String::new();
    walk(derivation, |step| {
        let Step::Exit(id, _) = step else {
            return Ok(());
        };
        let markup = derivation.label(id, &options.labels);
        let markup = markup.as_deref();
        match (derivation.is_open(id), &options.open) {
            (true, OpenGoal::Dots) => buffer.push_str("\\hypo{\\vdots}\n\\infer[no rule]1"),
            (true, OpenGoal::Bare) => buffer.push_str("\\hypo"),
            (true, OpenGoal::Dashed) => buffer.push_str("\\infer[dashed]0"),
            (true, OpenGoal::Mark(mark)) => {
                buffer.push_str("\\infer0[$");
                label(&mut buffer, mark);
                buffer.push_str("$]");
            }
            _ => {
                buffer.push_str("\\infer");
                buffer.push_str(&derivation.premises(id).len().to_string());
                if let Some(markup) = markup {
                    buffer.push_str("[$");
                    label(&mut buffer, markup);
                    buffer.push_str("$]");
                }
            }
        }
        buffer.push('{');
        derivation.sequent(&mut buffer, &NOTATION, id, options.align, false);
        buffer.push_str("}\n");
        flush(out, &mut buffer, &mut stop)
    })?;
    if let Some(judgement) = judgement {
        writeln!(out, "\\infer[no rule]1{{{judgement}}}")?;
    }
    out.write_str("\\end{prooftree}")?;
    if standalone {
        out.write_str("\n\\end{document}")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns an atom's name as the LaTeX export writes it.
    fn escaped(name: &str) -> String {
        let mut out = String::new();
        atom(&mut out, name).unwrap();
        out
    }

    /// One ASCII letter stays as it is, and one Greek letter is its
    /// command. Every other name is set in `\mathit`, with the special
    /// characters escaped, and Greek letters and `lltp`'s marks as
    /// commands in braces.
    #[test]
    fn names_are_escaped() {
        for (name, expected) in [
            ("A", "A"),
            ("p", "p"),
            ("foo", r"\mathit{foo}"),
            ("x_1", r"\mathit{x\_1}"),
            ("α", r"\alpha"),
            ("Γ", r"\Gamma"),
            ("Α", "A"),
            ("αβ", r"\mathit{{\alpha}{\beta}}"),
            ("Οx", r"\mathit{Ox}"),
            ("P‿a·b", r"\mathit{P{\smallsmile}a{\cdotp}b}"),
            ("é", r"\mathit{é}"),
            (
                r"a{b}$#%&^~\ c",
                r"\mathit{a\{b\}\$\#\%\&\mbox{\textasciicircum}\mbox{\textasciitilde}\mbox{\textbackslash}\ c}",
            ),
        ] {
            assert_eq!(escaped(name), expected, "{name:?}");
        }
    }
}
