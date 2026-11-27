// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Reading a witness term typed as text, `f(a, g(b))`, which the linear
//! and the ordinary sessions share: the tokens, the stack of open
//! applications and the refusals. Each session resolves the names in
//! its own tables.

use crate::WitnessFault;

/// What a name of a witness's text is.
pub(crate) enum Name<T, F> {
    /// A whole term: an eigenvariable or a constant.
    Term(T),
    /// A function symbol whose arguments follow in brackets.
    Applied(F),
}

/// A session's tables, as the reader of a witness asks them.
pub(crate) trait Witnesses {
    /// A term made.
    type Term: Copy;
    /// A function symbol waiting for its arguments.
    type Function;

    /// Whether a `(` after a whole term is a stray bracket
    /// ([`WitnessFault::Bracket`]), else a missing comma
    /// ([`WitnessFault::Comma`]).
    const BRACKET_AFTER_TERM: bool;

    /// Returns what `name` is, `applied` where a `(` follows it.
    ///
    /// # Errors
    ///
    /// The session's refusal of the name.
    fn name(
        &mut self,
        name: &str,
        applied: bool,
    ) -> Result<Name<Self::Term, Self::Function>, WitnessFault>;

    /// Returns the term of `function` applied to `args`.
    ///
    /// # Errors
    ///
    /// The session's refusal, such as another number of arguments than
    /// the symbol's arity.
    fn apply(
        &mut self,
        function: Self::Function,
        args: Vec<Self::Term>,
    ) -> Result<Self::Term, WitnessFault>;
}

/// Returns the tokens of a witness's text: names, brackets and commas,
/// white space dropped.
pub(crate) fn tokens(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut name = String::new();
    for c in text.chars().chain([' ']) {
        if c.is_whitespace() || matches!(c, '(' | ')' | ',') {
            if !name.is_empty() {
                tokens.push(std::mem::take(&mut name));
            }
            if !c.is_whitespace() {
                tokens.push(c.to_string());
            }
        } else {
            name.push(c);
        }
    }
    tokens
}

/// Returns whether a token is a name, no bracket or comma.
pub(crate) fn is_name(token: &str) -> bool {
    !matches!(token, "(" | ")" | ",")
}

/// Reads the term that `tokens` write, made by `tables`, on a stack of
/// its own: a term of any depth costs no recursion.
///
/// # Errors
///
/// The refusal of the first token that cannot stand where it does, or of
/// the tables.
pub(crate) fn read<W: Witnesses>(
    tables: &mut W,
    tokens: Vec<String>,
) -> Result<W::Term, WitnessFault> {
    // The open applications wait on a stack with their arguments.
    let mut open: Vec<(W::Function, Vec<W::Term>)> = Vec::new();
    let mut done: Option<W::Term> = None;
    let mut tokens = tokens.into_iter().peekable();
    while let Some(token) = tokens.next() {
        match token.as_str() {
            "(" if done.is_none() || W::BRACKET_AFTER_TERM => return Err(WitnessFault::Bracket),
            ")" | "," if done.is_none() => return Err(WitnessFault::Missing { before: token }),
            ")" | "," => {
                let (Some((_, args)), Some(arg)) = (open.last_mut(), done.take()) else {
                    return Err(WitnessFault::Unopened { token });
                };
                args.push(arg);
                if token == ")"
                    && let Some((function, args)) = open.pop()
                {
                    done = Some(tables.apply(function, args)?);
                }
            }
            _ if done.is_some() => return Err(WitnessFault::Comma { token }),
            name => {
                let applied = tokens.peek().is_some_and(|t| t == "(");
                match tables.name(name, applied)? {
                    Name::Term(term) => done = Some(term),
                    Name::Applied(function) => {
                        tokens.next();
                        open.push((function, Vec::new()));
                    }
                }
            }
        }
    }
    if !open.is_empty() {
        return Err(WitnessFault::Unclosed);
    }
    done.ok_or(WitnessFault::Empty)
}
