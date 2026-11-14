// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Reads the problems of the LLTP library (`github.com/meta-logic/lltp`),
//! the benchmark collection of linear logic provers.
//!
//! An LLTP file is a list of TPTP-style clauses `fof(name, role,
//! formula).`. [`read`] turns it into the sequent `axioms ⊢ conjecture`
//! and the provability the file's header claims ([`Status`]). The
//! formulas use this crate's text syntax (`*`, `|`, `&`, `+`, `-o`, `!`,
//! `?`, postfix `^`, `1`, `0`, `bot`, `top`) with the same precedences,
//! so the reader joins them into one sequent and parses that.
//!
//! The file does not say whether its problem is classical or
//! intuitionistic. The library keeps its intuitionistic problems under
//! `ILL/` and its classical ones under `CLL/`: pick the
//! [`Mode`](crate::linear::Mode) by that.
//!
//! # Examples
//!
//! Read a problem from `ILL/` and prove it:
//!
//! ```
//! use deduxit::Limits;
//! use deduxit::linear::Mode;
//! use deduxit::linear::lltp::{Status, read};
//! use deduxit::linear::search::{Options, prove};
//!
//! // A file's text, as `std::fs::read_to_string` returns it.
//! let text = "% Status (intuit.) : Theorem\n\
//!             fof(ax1, axiom, !(a -o b)).\n\
//!             fof(ax2, axiom, a).\n\
//!             fof(con, conjecture, b * 1).\n";
//! let problem = read(text, &Limits::default())?;
//! assert_eq!(problem.sequent, "!(a -o b), a |- b * 1".parse()?);
//! assert_eq!(problem.status, Some(Status::Theorem));
//!
//! let outcome = prove(&problem.sequent, Mode::INTUITIONISTIC, &Options::default())?;
//! assert!(outcome.verdict.proof().is_some());
//! # Ok::<(), deduxit::Error>(())
//! ```
//!
//! Needs the cargo feature `parse` (on by default).

use crate::Limits;
use crate::errors::ParseError;
use crate::{Error, linear::Sequent};

/// The character [`read`] puts for a `-` inside an atom name.
///
/// The library's Petri nets name places like `P-start_1_1`. This crate's
/// names cannot hold a `-`, which starts the operator `-o`. `‿` joins
/// words in a Unicode identifier, so `P‿start_1_1` is a name.
pub const HYPHEN: char = '‿';

/// The character [`read`] puts for a `.` inside an atom name.
///
/// Some of the library's Petri nets name places like
/// `merge.s00001061.input`. In the file a `.` ends a clause, and this
/// crate's names cannot hold one. `·` can stand in a Unicode identifier,
/// so `merge·s00001061·input` is a name.
pub const DOT: char = '·';

/// A problem read from an LLTP file by [`read`].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Problem {
    /// The problem's sequent: the axioms and hypotheses left of `⊢`, in
    /// the order of the file, and the conjecture right of it.
    pub sequent: Sequent,
    /// The provability the file's header claims, or `None` if it has no
    /// `Status` line this reader understands. It is the library's claim,
    /// not a checked fact.
    pub status: Option<Status>,
}

/// The provability an LLTP file's header claims.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// `Theorem`: the sequent is provable.
    Theorem,
    /// `Non-Theorem` or `CounterSatisfiable`: the sequent is not provable.
    NonTheorem,
}

/// Reads the text of an LLTP file into its sequent and the status its
/// header claims.
///
/// - Every clause with the role `axiom` or `hypothesis` is a formula left
///   of `⊢`, in the order of the file. The one clause with the role
///   `conjecture` is the formula right of it.
/// - From `%` to the end of a line is a comment.
/// - The status is the first `Status (intuit.)` or `Status (linear)`
///   comment's, else the first plain `Status` comment's. The library's
///   translations of intuitionistic problems carry the classical source's
///   status first.
/// - A `-` or `.` between two name characters (ASCII letters, digits and
///   `_`) is part of the name, and becomes [`HYPHEN`] or [`DOT`]. A `-o`
///   there is the connective when the character after it cannot be in a
///   name: `a-o b` is `a ⊸ b`. `a-ob` reads both ways and is refused.
///
/// The sequent is bounded by `limits` as [`Sequent::parse_within`] bounds
/// it.
///
/// # Errors
///
/// - [`Error::Lltp`] for text that is not a list of `fof(name, role,
///   formula).` clauses: an annotation after a formula, an empty
///   formula, a role other than `axiom`, `hypothesis` or `conjecture`, no
///   conjecture, or `-o` between two characters of a name;
/// - [`Error::SeveralConjectures`] for a second conjecture, which it
///   names: no convention fixes what several conjectures mean in linear
///   logic;
/// - [`Error::Parse`] for a formula this crate's parser refuses, placed
///   at its line and column in the file;
/// - [`Error::Refused`] for a sequent past `limits`, as
///   [`Sequent::parse_within`] refuses it.
pub fn read(text: &str, limits: &Limits) -> Result<Problem, Error> {
    let clauses = clauses(text, |message| Error::Lltp { message })?;
    // The sequent as text, and where each formula of the file starts in it.
    let mut sequent = String::new();
    let mut pieces = Vec::new();
    let ranges = clauses.hypotheses.iter().chain([&clauses.conjecture]);
    for (i, range) in ranges.enumerate() {
        sequent.push_str(match i {
            0 => "",
            _ if i == clauses.hypotheses.len() => " |- ",
            _ => ", ",
        });
        pieces.push((sequent.len(), range.clone()));
        sequent.push_str(&clauses.code[range.clone()]);
    }
    if clauses.hypotheses.is_empty() {
        sequent.insert_str(0, "|- ");
        pieces[0].0 += 3;
    }
    let sequent = Sequent::parse_within(&sequent, limits).map_err(|error| match error {
        Error::Parse(error) => {
            // Maps a byte of the sequent to its byte in the clauses' text,
            // and says whether it lies at or past its formula's end; a byte
            // past the end maps to the end.
            let at = |byte: usize| {
                let (start, range) = pieces
                    .iter()
                    .rev()
                    .find(|(start, _)| *start <= byte)
                    .unwrap_or(&pieces[0]);
                let offset = byte.saturating_sub(*start);
                (range.start + offset.min(range.len()), offset >= range.len())
            };
            Error::Parse(Box::new(clauses.located(text, &error, at)))
        }
        other => other,
    })?;
    Ok(Problem {
        sequent,
        status: clauses.status,
    })
}

/// The clauses of a TPTP-style file: its text with the names made
/// readable, where each formula stands in that text, by role, and the
/// status its header claims.
pub(crate) struct Clauses {
    /// The file's text without comments, a `-` inside a name written as
    /// [`HYPHEN`] and a `.` as [`DOT`].
    code: String,
    /// The formulas of the clauses with the role `axiom` or `hypothesis`.
    hypotheses: Vec<std::ops::Range<usize>>,
    /// The formula of the one clause with the role `conjecture`.
    conjecture: std::ops::Range<usize>,
    /// The provability the header claims.
    pub(crate) status: Option<Status>,
}

impl Clauses {
    /// Returns the hypotheses' formulas, in the order of the file.
    pub(crate) fn hypotheses(&self) -> impl Iterator<Item = &str> {
        self.hypotheses.iter().map(|r| &self.code[r.clone()])
    }

    /// Returns the clauses' text: the file's without comments, with the
    /// names made readable.
    pub(crate) fn code(&self) -> &str {
        &self.code
    }

    /// Returns the conjecture's formula.
    pub(crate) fn conjecture(&self) -> &str {
        &self.code[self.conjecture.clone()]
    }

    /// Returns the byte of `file`, the text these clauses were read from,
    /// that byte `at` of the clauses' text stands for. The clauses' text
    /// keeps the file's lines, and one character for each of the file's
    /// up to a comment, so a line and a character's place in it carry
    /// over.
    fn in_file(&self, file: &str, at: usize) -> usize {
        let before = &self.code[..at];
        let line = before.matches('\n').count();
        let column = before[before.rfind('\n').map_or(0, |n| n + 1)..]
            .chars()
            .count();
        let start: usize = file.split_inclusive('\n').take(line).map(str::len).sum();
        let rest = &file[start..];
        start
            + rest
                .char_indices()
                .nth(column)
                .map_or(rest.len(), |(i, _)| i)
    }

    /// Returns `error`, a parse error in text assembled from the clauses'
    /// formulas, placed in `file`, the text the clauses were read from.
    ///
    /// `at` maps a byte of the parsed text to a byte of the clauses' text,
    /// and says whether it lies at or past a formula's end. An error
    /// there is placed at the file's next character that is not white
    /// space, where the formula should have gone on.
    pub(crate) fn located(
        &self,
        file: &str,
        error: &ParseError,
        at: impl Fn(usize) -> (usize, bool),
    ) -> ParseError {
        let (byte, ended) = at(error.span.start);
        let mut start = self.in_file(file, byte);
        if ended {
            let rest = &file[start..];
            start += rest.len() - rest.trim_start().len();
        }
        // An error that spans a word, or carries a hint, keeps both.
        if error.hint.is_some() || error.span.len() > 1 {
            let end = self.in_file(file, at(error.span.end).0);
            let mut placed = ParseError::spanning(file, start..end.max(start), error.expected);
            if let Some(hint) = error.hint {
                placed = placed.with_hint(hint);
            }
            return placed;
        }
        ParseError::new(file, start, error.expected)
    }
}

/// Splits a file of `fof(name, role, formula).` clauses as [`read`]
/// describes, and checks that no formula is empty and that there is
/// exactly one conjecture.
///
/// `error` makes the error for a file of the wrong form from its message;
/// a second conjecture is [`Error::SeveralConjectures`] whatever `error`
/// makes.
pub(crate) fn clauses(text: &str, error: fn(String) -> Error) -> Result<Clauses, Error> {
    // The first status for the problem's logic, `(intuit.)` or `(linear)`,
    // and the first plain one.
    let (mut status, mut plain) = (None, None);
    let mut code = String::with_capacity(text.len());
    for line in text.lines() {
        let (before, comment) = line.split_once('%').unwrap_or((line, ""));
        if let Some((key, word)) = comment
            .split_once("Status")
            .and_then(|(_, r)| r.rsplit_once(':'))
        {
            let word = word.trim();
            let value = if word.starts_with("Theorem") {
                Some(Status::Theorem)
            } else if word.starts_with("Non-Theorem") || word.starts_with("CounterSatisfiable") {
                Some(Status::NonTheorem)
            } else {
                None
            };
            let slot = if key.contains("(intuit") || key.contains("(linear") {
                &mut status
            } else {
                &mut plain
            };
            if slot.is_none() {
                *slot = value;
            }
        }
        let chars: Vec<char> = before.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            let name = |j: usize| {
                chars
                    .get(j)
                    .is_some_and(|&c| c.is_ascii_alphanumeric() || c == '_')
            };
            // A `-` or `.` between two characters of a name belongs to the
            // name, unless the `-` starts the operator `-o`. A `-o` with a
            // name character on both sides reads both ways and is refused.
            let inner = i > 0 && name(i - 1) && name(i + 1);
            code.push(match c {
                '-' if inner && !(chars[i + 1] == 'o' && !name(i + 2)) => {
                    if chars[i + 1] == 'o' {
                        return Err(error(glued(&chars, i)));
                    }
                    HYPHEN
                }
                '.' if inner => DOT,
                c => c,
            });
        }
        code.push('\n');
    }

    let mut hypotheses = Vec::new();
    let mut conjecture = None;
    let mut rest = code.trim_start();
    while !rest.is_empty() {
        let body = rest
            .strip_prefix("fof")
            .map(str::trim_start)
            .and_then(|r| r.strip_prefix('('))
            .ok_or_else(|| error(format!("expected `fof(` at `{}`", excerpt(rest))))?;
        let (name, body) = body
            .split_once(',')
            .ok_or_else(|| error(format!("a clause without a role at `{}`", excerpt(rest))))?;
        let name = name.trim();
        let (role, body) = body
            .split_once(',')
            .ok_or_else(|| error(format!("clause `{name}` has no formula")))?;
        // The formula runs to the parenthesis that closes `fof(`; a comma
        // outside its parentheses starts an annotation.
        let mut depth = 1usize;
        let end = body
            .char_indices()
            .find(|&(_, c)| {
                match c {
                    '(' => depth += 1,
                    ')' => depth -= 1,
                    _ => {}
                }
                depth == 0 || (depth == 1 && c == ',')
            })
            .ok_or_else(|| error(format!("clause `{name}` is not closed")))?;
        if end.1 == ',' {
            return Err(error(format!(
                "clause `{name}` has an annotation, which this reader does not take"
            )));
        }
        let end = end.0;
        let formula = body[..end].trim();
        if formula.is_empty() {
            return Err(error(format!("clause `{name}` has an empty formula")));
        }
        let start = formula.as_ptr() as usize - code.as_ptr() as usize;
        let range = start..start + formula.len();
        match role.trim() {
            "axiom" | "hypothesis" => hypotheses.push(range),
            "conjecture" if conjecture.is_some() => {
                return Err(Error::SeveralConjectures {
                    second: name.to_owned(),
                });
            }
            "conjecture" => conjecture = Some(range),
            other => {
                return Err(error(format!(
                    "clause `{name}` has the role `{other}`, not axiom, hypothesis or conjecture"
                )));
            }
        }
        rest = body[end + 1..]
            .trim_start()
            .strip_prefix('.')
            .ok_or_else(|| error(format!("clause `{name}` does not end with `).`")))?
            .trim_start();
    }
    let Some(conjecture) = conjecture else {
        return Err(error("no conjecture".to_owned()));
    };
    Ok(Clauses {
        code,
        hypotheses,
        conjecture,
        status: status.or(plain),
    })
}

/// Returns the message for a `-o` at `at` of a line's `chars` with a name
/// character on both sides, which reads both as one name and as two
/// names joined by the connective.
#[cold]
fn glued(chars: &[char], at: usize) -> String {
    let name = |c: &char| c.is_ascii_alphanumeric() || *c == '_';
    let start = chars[..at]
        .iter()
        .rposition(|c| !name(c))
        .map_or(0, |n| n + 1);
    let end = chars[at + 2..]
        .iter()
        .position(|c| !name(c))
        .map_or(chars.len(), |n| at + 2 + n);
    let word: String = chars[start..end].iter().collect();
    format!(
        "`{word}` reads as one name and as two joined by `-o`: write a space before `-o` for \
         the connective"
    )
}

/// Returns the first line of `text`, cut after 24 characters, for an
/// error message.
fn excerpt(text: &str) -> &str {
    let line = text.lines().next().unwrap_or_default();
    let end = line.char_indices().nth(24).map_or(line.len(), |(i, _)| i);
    &line[..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A file with clauses over several lines, comments between and after
    /// them, a `hypothesis` role, uppercase atoms, names with `-` and `.`,
    /// postfix negation and every unit reads into the sequent the parser
    /// gives for the same formulas. The intuitionistic status wins over
    /// the plain one before it, and malformed files are refused.
    #[test]
    fn reads_problems() {
        let text = "%---\n% Status   : Theorem\n% Status (intuit.) : Non-Theorem\n%---\n\
                    fof(ax1, axiom, !( (P-a_1) -o\n  (Q.b * Q.b) ) ).   % a transition\n\
                    fof(h, hypothesis, bot | top^).\n\
                    fof(con, conjecture, (Q.b * Q.b) + 0 & 1).\n";
        let problem = read(text, &Limits::default()).unwrap();
        assert_eq!(problem.status, Some(Status::NonTheorem));
        let expected: Sequent = "!(P‿a_1 -o Q·b * Q·b), bot | top^ |- (Q·b * Q·b) + 0 & 1"
            .parse()
            .unwrap();
        assert_eq!(problem.sequent, expected);

        for bad in [
            "fof(a, axiom, a).",
            "fof(a, axiom, a). fof(c, conjecture, a",
            "fof(c, conjecture, a) fof(d, axiom, b).",
            "fof(c, definition, a).",
            "fof(c, conjecture, a, unknown).",
            "cnf(c, conjecture, a).",
            "fof(h, axiom, bot). fof(c, conjecture, ).",
            "fof(h, axiom, ). fof(c, conjecture, a).",
        ] {
            assert!(
                matches!(read(bad, &Limits::default()), Err(Error::Lltp { .. })),
                "{bad:?}"
            );
        }
        // `a-ob` is a name of the library's kind and `a -o b` in this
        // crate's syntax; `a-o b` and `a -ob` are the connective.
        assert!(matches!(
            read("fof(c, conjecture, x * a-ob).", &Limits::default()),
            Err(Error::Lltp { message }) if message.starts_with("`a-ob` reads as one name")
        ));
        for text in ["fof(c, conjecture, a-o b).", "fof(c, conjecture, a -ob)."] {
            let problem = read(text, &Limits::default()).unwrap();
            assert_eq!(problem.sequent, "|- a -o b".parse().unwrap(), "{text}");
        }
        // A hyphen or a dot between name characters is part of the name,
        // a `-o` after a name's last character the connective.
        for (formula, sequent) in [
            ("p.q", "|- p·q"),
            ("a-b-o c", "|- a‿b -o c"),
            ("(a)-o(b)", "|- a -o b"),
            ("a-1 * b.c.d", "|- a‿1 * b·c·d"),
        ] {
            let text = format!("fof(c, conjecture, {formula}).");
            let problem = read(&text, &Limits::default()).unwrap();
            assert_eq!(problem.sequent, sequent.parse().unwrap(), "{formula}");
        }
        // The messages say what is wrong, and where.
        for (text, message) in [
            ("fof(a, axiom, a).", "no conjecture"),
            (
                "cnf(c, conjecture, a).",
                "expected `fof(` at `cnf(c, conjecture, a).`",
            ),
            (
                "fof(c, lemma, a).",
                "clause `c` has the role `lemma`, not axiom, hypothesis or conjecture",
            ),
        ] {
            match read(text, &Limits::default()) {
                Err(Error::Lltp { message: m }) => assert_eq!(m, message, "{text}"),
                other => panic!("{text}: {other:?}"),
            }
        }
    }

    /// A formula that does not parse is reported at its place in the
    /// file, not in the sequent assembled from the clauses: a formula cut
    /// short where its clause closes, and a binder cut short after a
    /// comment and a name with a hyphen.
    #[test]
    fn parse_errors_are_placed_in_the_file() {
        let place = |text: &str| match read(text, &Limits::default()) {
            Err(Error::Parse(e)) => (e.line, e.column, e.found.clone(), e.reserved),
            other => panic!("{other:?}"),
        };
        let text = "fof(a1, axiom, a).\nfof(a2, axiom, b * ).\nfof(c, conjecture, a).";
        assert_eq!(place(text), (2, 20, Some(")".to_owned()), false));
        let text =
            "% a comment\nfof(a1, axiom, P-a % and one more\n * forall).\nfof(c, conjecture, a).";
        assert_eq!(place(text), (3, 10, Some(")".to_owned()), false));
        assert_eq!(
            place("fof(c, conjecture, a * )."),
            (1, 24, Some(")".to_owned()), false)
        );
        // Without hypotheses, inside the conjecture, and after the white
        // space that ends it.
        assert_eq!(
            place("fof(c, conjecture, a * * b)."),
            (1, 24, Some("*".to_owned()), false)
        );
        assert_eq!(
            place("fof(c, conjecture, a *   )."),
            (1, 26, Some(")".to_owned()), false)
        );
    }

    /// A second conjecture is refused by name: joined right of `⊢`, the
    /// two would be read as their par, and `⊢ a, ~a` is provable where
    /// neither `⊢ a` nor `⊢ ~a` is.
    #[test]
    fn refuses_several_conjectures() {
        let text = "fof(c1, conjecture, a). fof(c2, conjecture, a^).";
        assert!(matches!(
            read(text, &Limits::default()),
            Err(Error::SeveralConjectures { second }) if second == "c2"
        ));
    }
}
