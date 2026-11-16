// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Terms and sequents read from text, in the syntax a calculus's print
//! forms give its connectives: what [`Calculus::show`] writes reads back.

use super::FORMULA;
use super::calculus::{Associativity, Calculus, Fixity};
use super::terms::{TermId, Terms};
use crate::Error;
use crate::errors::ParseError;
use crate::hash::HashMap;
use crate::limits::{Limits, Refusal};

/// ASCII spellings of common symbols, each read as its symbol where the
/// calculus declares the symbol and not the spelling itself.
const ASCII: [(&str, &str); 16] = [
    ("|-", "⊢"),
    ("[]", "□"),
    ("<>", "◇"),
    ("->", "→"),
    ("<->", "↔"),
    ("/\\", "∧"),
    ("\\/", "∨"),
    ("~", "¬"),
    ("-o", "⊸"),
    ("*", "⊗"),
    ("par", "⅋"),
    ("+", "⊕"),
    ("top", "⊤"),
    ("bot", "⊥"),
    ("true", "⊤"),
    ("false", "⊥"),
];

/// What a symbol of the text stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Symbol {
    /// The connective at this index.
    Connective(u32),
    /// The mark of the zone at this index.
    Mark(usize),
}

/// A token of the text and the byte where it starts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Token<'a> {
    /// `(`
    Open,
    /// `)`
    Close,
    /// `,`
    Comma,
    /// `;`
    Semicolon,
    /// A declared connective or mark.
    Symbol(Symbol),
    /// A name the calculus does not declare: an atom.
    Word(&'a str),
}

/// What the parse of one term built: an atom by name, or a connective
/// applied to earlier entries.
#[derive(Clone, Debug)]
enum Parsed<'a> {
    /// An atom's name and where it stands.
    Atom(&'a str, usize),
    /// A connective, the entries of its arguments, and where it stands.
    Apply(u32, Vec<usize>, usize),
}

/// What waits on the operator stack.
#[derive(Clone, Copy, Debug)]
enum Waiting {
    /// A prefix or infix connective and where it stands.
    Operator(u32, usize),
    /// An open bracket, with the function it belongs to and the number of
    /// arguments read so far, and where it stands.
    Bracket(Option<u32>, usize, usize),
}

/// How a connective is read: its fixity as written or by its arity, how
/// tightly it binds and how a chain of it groups.
#[derive(Clone, Copy, Debug)]
struct Reading {
    /// Where it stands among its arguments.
    fixity: Fixity,
    /// How tightly it binds.
    precedence: u32,
    /// How a chain of it groups.
    associativity: Associativity,
    /// How many arguments it takes.
    arity: usize,
}

/// The tokens of a text, read by the symbols a calculus declares.
struct Lexer<'a> {
    /// The text.
    text: &'a str,
    /// Every symbol by its spelling.
    symbols: HashMap<String, Symbol>,
}

impl<'a> Lexer<'a> {
    /// Returns the lexer of `text` for `calculus`.
    fn new(calculus: &Calculus, text: &'a str) -> Self {
        let mut symbols: HashMap<String, Symbol> = HashMap::default();
        for (index, zone) in calculus.form().sequent.iter().enumerate() {
            if let Some(mark) = &zone.mark {
                symbols.insert(mark.clone(), Symbol::Mark(index));
            }
        }
        for (index, connective) in calculus.form().connectives.iter().enumerate() {
            let symbol = Symbol::Connective(index as u32);
            symbols.insert(connective.name.clone(), symbol);
            if let Some(written) = &connective.symbol {
                symbols.insert(written.clone(), symbol);
            }
        }
        for (spelling, symbol) in ASCII {
            if let Some(&found) = symbols.get(symbol)
                && !symbols.contains_key(spelling)
            {
                symbols.insert(spelling.to_owned(), found);
            }
        }
        Self { text, symbols }
    }

    /// Returns the tokens and where each starts.
    fn tokens(&self) -> Result<Vec<(Token<'a>, usize)>, Error> {
        let text = self.text;
        let word = |c: char| c.is_alphanumeric() || c == '_' || c == '′' || c == '\'';
        let mut tokens = Vec::new();
        let mut at = 0;
        while at < text.len() {
            let rest = &text[at..];
            let Some(c) = rest.chars().next() else { break };
            if c.is_whitespace() {
                at += c.len_utf8();
                continue;
            }
            let single = match c {
                '(' => Some(Token::Open),
                ')' => Some(Token::Close),
                ',' => Some(Token::Comma),
                ';' => Some(Token::Semicolon),
                _ => None,
            };
            if let Some(token) = single {
                tokens.push((token, at));
                at += 1;
                continue;
            }
            // A word is read whole: a symbol where it is one, else an atom.
            if word(c) {
                let end = rest.find(|c: char| !word(c)).unwrap_or(rest.len());
                let found = &rest[..end];
                let token = match self.symbols.get(found) {
                    Some(&symbol) => Token::Symbol(symbol),
                    None => Token::Word(found),
                };
                tokens.push((token, at));
                at += end;
                continue;
            }
            // Otherwise the longest symbol that starts here.
            let longest = self
                .symbols
                .iter()
                .filter(|(spelling, _)| {
                    rest.starts_with(spelling.as_str())
                        && !spelling.chars().next().is_some_and(word)
                })
                .max_by_key(|(spelling, _)| spelling.len());
            let Some((spelling, &symbol)) = longest else {
                return Err(error(text, at, &["a formula", "a connective"]));
            };
            tokens.push((Token::Symbol(symbol), at));
            at += spelling.len();
        }
        Ok(tokens)
    }
}

/// Returns the parse error at byte `at` of `text`.
fn error(text: &str, at: usize, expected: &'static [&'static str]) -> Error {
    Error::Parse(Box::new(ParseError::new(text, at, expected)))
}

impl Calculus {
    /// Returns how the connective at `index` is read.
    fn reading(&self, index: u32) -> Reading {
        let form = &self.form().connectives[index as usize];
        let arity = form.arity as usize;
        let fixity = match (form.fixity, arity) {
            (_, 0) => Fixity::Function,
            (Some(Fixity::Prefix) | None, 1) => Fixity::Prefix,
            (Some(Fixity::Postfix), 1) => Fixity::Postfix,
            (Some(Fixity::Infix) | None, 2) => Fixity::Infix,
            _ => Fixity::Function,
        };
        let precedence = form.precedence.unwrap_or(match fixity {
            Fixity::Infix => 0,
            Fixity::Prefix | Fixity::Postfix | Fixity::Function => u32::MAX,
        });
        Reading {
            fixity,
            precedence,
            associativity: form.associativity.unwrap_or(Associativity::None),
            arity,
        }
    }

    /// Reads one term from `tokens`, a member of the sort at `sort`, into
    /// `terms`. `text` is the whole input, for the errors' places.
    ///
    /// The parse is a shunting-yard over an explicit stack: nothing
    /// recurses, however deep the term is nested.
    fn term_of(
        &self,
        terms: &mut Terms,
        text: &str,
        tokens: &[(Token<'_>, usize)],
        sort: u32,
        limits: &Limits,
    ) -> Result<TermId, Error> {
        let end = tokens.last().map_or(text.len(), |&(_, at)| at);
        let Some(&(_, start)) = tokens.first() else {
            return Err(error(text, end, &["a formula"]));
        };
        if let Some(limit) = limits.occurrences
            && tokens.len() as u64 > limit
        {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: tokens.len() as u64,
                limit,
            }));
        }
        let mut output: Vec<Parsed<'_>> = Vec::with_capacity(tokens.len());
        let mut operands: Vec<usize> = Vec::new();
        let mut waiting: Vec<Waiting> = Vec::new();
        // Whether an operand is expected next.
        let mut operand = true;
        // Pops the operator on top of the stack into the output.
        let reduce = |output: &mut Vec<Parsed<'_>>,
                      operands: &mut Vec<usize>,
                      connective: u32,
                      at: usize,
                      arity: usize|
         -> Result<(), Error> {
            if operands.len() < arity {
                return Err(error(text, at, &["a formula"]));
            }
            let arguments = operands.split_off(operands.len() - arity);
            output.push(Parsed::Apply(connective, arguments, at));
            operands.push(output.len() - 1);
            Ok(())
        };
        let mut index = 0;
        while index <= tokens.len() {
            let current = tokens.get(index).copied();
            index += 1;
            match current {
                Some((Token::Word(name), at)) if operand => {
                    output.push(Parsed::Atom(name, at));
                    operands.push(output.len() - 1);
                    operand = false;
                }
                Some((Token::Symbol(Symbol::Connective(c)), at)) if operand => {
                    let reading = self.reading(c);
                    match reading.fixity {
                        Fixity::Function if reading.arity == 0 => {
                            output.push(Parsed::Apply(c, Vec::new(), at));
                            operands.push(output.len() - 1);
                            operand = false;
                        }
                        Fixity::Function => {
                            if !matches!(tokens.get(index), Some((Token::Open, _))) {
                                let next = tokens.get(index).map_or(text.len(), |&(_, a)| a);
                                return Err(error(text, next, &["("]));
                            }
                            index += 1;
                            waiting.push(Waiting::Bracket(Some(c), 0, at));
                        }
                        Fixity::Prefix => waiting.push(Waiting::Operator(c, at)),
                        Fixity::Infix | Fixity::Postfix => {
                            return Err(error(text, at, &["a formula"]));
                        }
                    }
                }
                Some((Token::Open, at)) if operand => waiting.push(Waiting::Bracket(None, 0, at)),
                Some((Token::Symbol(Symbol::Connective(c)), at)) if !operand => {
                    let reading = self.reading(c);
                    if !matches!(reading.fixity, Fixity::Infix | Fixity::Postfix) {
                        return Err(error(text, at, &["a connective", ",", ")"]));
                    }
                    // Reduce what binds tighter, or as tight where the
                    // chain groups to the left.
                    while let Some(&Waiting::Operator(top, top_at)) = waiting.last() {
                        let other = self.reading(top);
                        let tighter = other.precedence > reading.precedence
                            || (other.precedence == reading.precedence
                                && (other.fixity == Fixity::Prefix
                                    || reading.associativity == Associativity::Left));
                        if !tighter {
                            if other.precedence == reading.precedence
                                && reading.associativity == Associativity::None
                                && other.fixity == Fixity::Infix
                                && top == c
                            {
                                return Err(error(text, at, &["brackets around a chain"]));
                            }
                            break;
                        }
                        waiting.pop();
                        reduce(&mut output, &mut operands, top, top_at, other.arity)?;
                    }
                    if reading.fixity == Fixity::Postfix {
                        reduce(&mut output, &mut operands, c, at, 1)?;
                    } else {
                        waiting.push(Waiting::Operator(c, at));
                        operand = true;
                    }
                }
                Some((Token::Comma, at)) if !operand => {
                    // An argument of a function ends.
                    loop {
                        match waiting.pop() {
                            Some(Waiting::Operator(top, top_at)) => {
                                let arity = self.reading(top).arity;
                                reduce(&mut output, &mut operands, top, top_at, arity)?;
                            }
                            Some(Waiting::Bracket(Some(f), count, f_at)) => {
                                waiting.push(Waiting::Bracket(Some(f), count + 1, f_at));
                                break;
                            }
                            Some(Waiting::Bracket(None, ..)) | None => {
                                return Err(error(text, at, &["a connective", ")"]));
                            }
                        }
                    }
                    operand = true;
                }
                Some((Token::Close, at)) if !operand => loop {
                    match waiting.pop() {
                        Some(Waiting::Operator(top, top_at)) => {
                            let arity = self.reading(top).arity;
                            reduce(&mut output, &mut operands, top, top_at, arity)?;
                        }
                        Some(Waiting::Bracket(None, ..)) => break,
                        Some(Waiting::Bracket(Some(f), count, f_at)) => {
                            let arity = self.reading(f).arity;
                            if count + 1 != arity {
                                return Err(error(text, at, &[","]));
                            }
                            reduce(&mut output, &mut operands, f, f_at, arity)?;
                            break;
                        }
                        None => return Err(error(text, at, &["a connective", "the end"])),
                    }
                },
                None if !operand => {
                    while let Some(top) = waiting.pop() {
                        match top {
                            Waiting::Operator(c, at) => {
                                let arity = self.reading(c).arity;
                                reduce(&mut output, &mut operands, c, at, arity)?;
                            }
                            Waiting::Bracket(_, _, at) => {
                                return Err(error(text, at, &["a matching )"]));
                            }
                        }
                    }
                }
                None => return Err(error(text, end, &["a formula"])),
                Some((_, at)) => {
                    let expected: &'static [&'static str] = if operand {
                        &["a formula"]
                    } else {
                        &["a connective", ",", ")"]
                    };
                    return Err(error(text, at, expected));
                }
            }
        }
        if operands.len() != 1 {
            return Err(error(text, start, &["one formula"]));
        }
        // The sorts, from the root down: each entry's children lie before
        // it, so a pass from the end sees a parent before its children.
        let mut sorts: Vec<Option<u32>> = vec![None; output.len()];
        let root = operands[0];
        sorts[root] = Some(sort);
        for entry in (0..output.len()).rev() {
            let Some(expected) = sorts[entry] else {
                continue;
            };
            if let Parsed::Apply(c, arguments, at) = &output[entry] {
                let signature = &self.connectives[*c as usize];
                if signature.result != expected {
                    return Err(error(text, *at, &["a term of another sort"]));
                }
                for (&argument, &sort) in arguments.iter().zip(&signature.arguments) {
                    sorts[argument] = Some(sort);
                }
            }
        }
        let mut ids: Vec<Option<TermId>> = vec![None; output.len()];
        for (entry, parsed) in output.iter().enumerate() {
            let Some(sort) = sorts[entry] else { continue };
            let id = match parsed {
                Parsed::Atom(name, at) => {
                    let sort_name = self.sort_name(sort);
                    terms.atom(name, sort_name).map_err(|e| match e {
                        Error::AtomName { .. } => error(text, *at, &["an atom's name"]),
                        other => other,
                    })?
                }
                Parsed::Apply(c, arguments, _) => {
                    let arguments: Vec<TermId> = arguments
                        .iter()
                        .map(|&a| ids[a].expect("an argument lies before its term"))
                        .collect();
                    terms.apply(&self.form().connectives[*c as usize].name, &arguments)?
                }
            };
            ids[entry] = Some(id);
        }
        Ok(ids[root].expect("the root is built"))
    }

    /// Returns the name of the sort at `index`.
    pub(crate) fn sort_name(&self, index: u32) -> &str {
        self.sort_ids
            .iter()
            .find(|&(_, &i)| i == index)
            .map_or(FORMULA, |(name, _)| name)
    }

    /// Reads a term of the sort named `sort` from `text`, written as
    /// [`show`](Self::show) writes one, into `terms`. Connectives are read
    /// by their names and symbols, a few by an ASCII spelling (`->` for
    /// `→`, `/\` for `∧`, `[]` for `□`, `<>` for `◇`, `|-` for `⊢`) where
    /// the calculus declares the symbol; any other word is an atom.
    ///
    /// # Errors
    ///
    /// [`Error::Parse`] where the text is no term of the sort, and
    /// [`Refusal::Occurrences`] for a text of more tokens than
    /// `limits.occurrences`.
    pub fn parse_term(
        &self,
        terms: &mut Terms,
        text: &str,
        sort: &str,
        limits: &Limits,
    ) -> Result<TermId, Error> {
        let Some(&sort) = self.sort_ids.get(sort) else {
            return Err(Error::UnknownName {
                what: "sort",
                name: sort.into(),
                known: &[],
            });
        };
        let tokens = Lexer::new(self, text).tokens()?;
        self.term_of(terms, text, &tokens, sort, limits)
    }

    /// Reads a list of terms separated by `,`, each of the sort at
    /// `sort`, into `terms`: what a context of that sort holds.
    #[cfg(feature = "interactive")]
    pub(crate) fn parse_list(
        &self,
        terms: &mut Terms,
        text: &str,
        sort: u32,
        limits: &Limits,
    ) -> Result<Vec<TermId>, Error> {
        let tokens = Lexer::new(self, text).tokens()?;
        self.members(terms, text, &tokens, sort, limits)
    }

    /// Reads the members of a zone, separated by `,` outside brackets.
    fn members(
        &self,
        terms: &mut Terms,
        text: &str,
        tokens: &[(Token<'_>, usize)],
        sort: u32,
        limits: &Limits,
    ) -> Result<Vec<TermId>, Error> {
        let mut members = Vec::new();
        if tokens.is_empty() {
            return Ok(members);
        }
        let mut depth = 0usize;
        let mut from = 0;
        for (index, &(token, _)) in tokens.iter().enumerate() {
            match token {
                Token::Open => depth += 1,
                Token::Close => depth = depth.saturating_sub(1),
                Token::Comma if depth == 0 => {
                    members.push(self.term_of(terms, text, &tokens[from..index], sort, limits)?);
                    from = index + 1;
                }
                _ => {}
            }
        }
        members.push(self.term_of(terms, text, &tokens[from..], sort, limits)?);
        Ok(members)
    }

    /// Reads a sequent of the calculus from `text`, written as
    /// [`show_sequent`](Self::show_sequent) writes one, into `terms`: the
    /// zones in order, a zone with a mark after its mark (`⊢`), the members
    /// of a zone separated by `,` and two zones without a mark between
    /// them by `;`. The first zone's mark may be left out.
    ///
    /// # Errors
    ///
    /// [`Error::Parse`] where the text is no sequent of the calculus, and
    /// [`Refusal::Occurrences`] for a text of more tokens than
    /// `limits.occurrences`.
    pub fn parse_sequent(
        &self,
        terms: &mut Terms,
        text: &str,
        limits: &Limits,
    ) -> Result<Vec<Vec<TermId>>, Error> {
        let tokens = Lexer::new(self, text).tokens()?;
        let zones = self.zones.len();
        let mut sequent = vec![Vec::new(); zones];
        let mut zone = 0;
        let mut from = 0;
        let mut depth = 0usize;
        let mut index = 0;
        while index <= tokens.len() {
            let current = tokens.get(index).copied();
            let next = match current {
                Some((Token::Open, _)) => {
                    depth += 1;
                    None
                }
                Some((Token::Close, _)) => {
                    depth = depth.saturating_sub(1);
                    None
                }
                // A zone with a mark starts at it; the first zone's mark
                // may lead the text.
                Some((Token::Symbol(Symbol::Mark(target)), at)) if depth == 0 => {
                    if target < zone || (target == zone && (zone > 0 || index > 0)) {
                        return Err(error(text, at, &["a formula", ","]));
                    }
                    Some((target, at))
                }
                Some((Token::Semicolon, at)) if depth == 0 => {
                    let target = zone + 1;
                    if target >= zones || self.form().sequent[target].mark.is_some() {
                        return Err(error(text, at, &["a formula", ","]));
                    }
                    Some((target, at))
                }
                None => Some((zones, text.len())),
                Some(_) => None,
            };
            if let Some((target, _)) = next {
                sequent[zone] = self.members(
                    terms,
                    text,
                    &tokens[from..index],
                    self.zones[zone].sort,
                    limits,
                )?;
                zone = target;
                from = index + 1;
            }
            index += 1;
        }
        Ok(sequent)
    }
}
