// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! What the text parsers of ordinary and modal formulas share: the text
//! and the place in it, what ends a formula, and the reading of a
//! sequent's two sides from its formulas.

use crate::Error;
use crate::errors::ParseError;
use crate::limits::Refusal;

/// What a formula's list ended with.
pub(crate) enum End {
    /// A comma: another formula of the same side follows.
    Comma,
    /// The turnstile.
    Turnstile,
    /// The end of the text.
    Stop,
}

/// What may stand after a formula inside a parenthesis.
pub(crate) const IN_BRACKETS: &[&str] = &["a connective", "`)`"];

/// What may stand after a formula of a sequent outside every parenthesis.
pub(crate) const AFTER_FORMULA: &[&str] = &["a connective", "`,`", "`|-`", "the end"];

/// A text in the reading.
pub(crate) struct Text<'a> {
    /// The text.
    pub(crate) input: &'a str,
    /// The byte offset of the next character.
    pub(crate) at: usize,
}

impl<'a> Text<'a> {
    /// Returns the error for the character at byte `at`, where `expected`
    /// could have stood.
    pub(crate) fn unexpected(&self, at: usize, expected: &'static [&'static str]) -> Error {
        Error::from(ParseError::new(self.input, at, expected))
    }

    /// Skips white space and returns where the next character stands.
    pub(crate) fn next_at(&mut self) -> usize {
        self.rest();
        self.at
    }

    /// Skips white space and returns the rest of the text. Out of line:
    /// inlined, the operator tokens' comparisons became calls of `memcmp`
    /// (the `read-tptp` journey counted 2 % more).
    #[inline(never)]
    pub(crate) fn rest(&mut self) -> &'a str {
        let rest = &self.input[self.at..];
        self.at += rest.len() - rest.trim_start().len();
        &self.input[self.at..]
    }

    /// Consumes `token` if the text goes on with it.
    pub(crate) fn eat(&mut self, token: &str) -> bool {
        let found = self.rest().starts_with(token);
        if found {
            self.at += token.len();
        }
        found
    }

    /// Consumes the turnstile, `|-` or `⊢`, if the text goes on with it.
    pub(crate) fn turnstile(&mut self) -> bool {
        self.eat("|-") || self.eat("⊢")
    }

    /// Reads an identifier if one starts here.
    #[expect(
        clippy::inline_always,
        reason = "the parsers' operand loops call it, and the ordinary parser's \
                  journeys count it inlined there"
    )]
    #[inline(always)]
    pub(crate) fn identifier(&mut self) -> Option<&'a str> {
        let rest = self.rest();
        let first = rest.chars().next()?;
        if !crate::name::starts_identifier(first) {
            return None;
        }
        let end = rest
            .find(|c| !unicode_ident::is_xid_continue(c))
            .unwrap_or(rest.len());
        self.at += end;
        Some(&rest[..end])
    }

    /// Consumes what ends a formula of a sequent outside every parenthesis,
    /// if that comes next, and says what it was.
    pub(crate) fn end(&mut self) -> Option<End> {
        if self.eat(",") {
            Some(End::Comma)
        } else if self.turnstile() {
            Some(End::Turnstile)
        } else if self.rest().is_empty() {
            Some(End::Stop)
        } else {
            None
        }
    }
}

/// A sequent's formulas left of the turnstile and right of it.
pub(crate) type Sides<I> = (Vec<I>, Vec<I>);

/// A parser of a sequent's formulas.
pub(crate) trait Reads<'a> {
    /// The id of a formula read.
    type Id;

    /// Returns the text in the reading.
    fn text(&mut self) -> &mut Text<'a>;

    /// Reads one formula up to a comma, the turnstile or the end of the
    /// text outside every parenthesis, which it says.
    ///
    /// # Errors
    ///
    /// Where the text holds no formula there, or a bound refuses it.
    fn formula(&mut self) -> Result<(Self::Id, End), Error>;
}

/// Reads the formulas of a sequent, `formulas |- formulas` with either side
/// empty, or a list of formulas to prove, and returns its two sides. More
/// formulas than `limit` are refused.
///
/// # Errors
///
/// [`Error::Parse`] for text that is no sequent, a second turnstile among
/// them, what `formula` answers, and [`Refusal::Occurrences`] past the
/// bound.
pub(crate) fn sides<'a, R: Reads<'a>>(
    reader: &mut R,
    limit: Option<u64>,
) -> Result<Sides<R::Id>, Error> {
    let (mut left, mut right) = (Vec::new(), Vec::new());
    let mut turnstile = false;
    // An empty side: the turnstile first, or nothing after it.
    if reader.text().turnstile() {
        turnstile = true;
    }
    if !(turnstile && reader.text().rest().is_empty()) {
        loop {
            let (formula, end) = reader.formula()?;
            if turnstile { &mut right } else { &mut left }.push(formula);
            count_within(left.len() + right.len(), limit)?;
            match end {
                End::Comma => {}
                End::Turnstile if !turnstile => {
                    turnstile = true;
                    if reader.text().rest().is_empty() {
                        break;
                    }
                }
                End::Turnstile => {
                    // The second turnstile, `|-` or `⊢`, as one token.
                    let text = reader.text();
                    let read = &text.input[..text.at];
                    let start = read.len() - if read.ends_with("|-") { 2 } else { "⊢".len() };
                    let span = start..text.at;
                    let expected = &["a connective", "`,`", "the end"];
                    return Err(ParseError::spanning(text.input, span, expected).into());
                }
                End::Stop => break,
            }
        }
    }
    if !turnstile {
        // One formula, or a list, to prove.
        std::mem::swap(&mut left, &mut right);
    }
    Ok((left, right))
}

/// Fails once `count` formulas, of the arena or of a sequent, pass
/// `limit`: each is at least one subformula occurrence, however much of
/// the arena a formula of the sequent shares with the others.
///
/// # Errors
///
/// [`Refusal::Occurrences`] past `limit`.
pub(crate) fn count_within(count: usize, limit: Option<u64>) -> Result<(), Error> {
    match limit {
        Some(limit) if count as u64 > limit => Err(Error::Refused(Refusal::Occurrences {
            occurrences: limit.saturating_add(1),
            limit,
        })),
        _ => Ok(()),
    }
}
