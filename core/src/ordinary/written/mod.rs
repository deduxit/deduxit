// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A signed tableau of classical logic written by hand, read from text
//! and reviewed step by step.
//!
//! The text is the one `deduxit prove --method tableau` prints, and what
//! a student writes on paper as Smullyan, the Open Logic Project and
//! forall x lay out a tree: a numbered line per signed formula, the line
//! it expands at its end, `×` under a branch that closes and `open` under
//! one that stays open. A branch starts with `├─`, `└─`, `|-`, `+-`,
//! `` `- `` or a plain `-`, indented under the line it goes on from:
//!
//! ```text
//! 1 T p ∨ q
//! 2 F q ∨ p
//! 3 F q        2
//! 4 F p        2
//! - 5 T p      1
//!   × 4, 5
//! - 6 T q      1
//!   × 3, 6
//! ```

use super::parse::formula_in;
use super::{Countermodel, Formulas, Logic, Node, NodeId, Sequent, Sign, Signed};
use crate::Error;
use crate::errors::ParseError;
use crate::hash::{HashMap, HashSet};
use crate::kripke::{Model, World};
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::rules::{Calculus, Derivation, Node as RuleNode, Step as RuleStep, TermId, terms_of};

/// The bytes a planned node of the derivation holds per member of its
/// branch, with its share of the derivation's own copy.
const MEMBER_BYTES: u64 = 16;

/// The bytes a planned node holds beside its members.
const NODE_BYTES: u64 = 96;

/// A connective a rule of the tableau takes apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Connective {
    /// `¬`
    Not,
    /// `∧`
    And,
    /// `∨`
    Or,
    /// `→`
    Implies,
    /// `↔`
    Iff,
}

impl Connective {
    /// Returns the connective's symbol.
    const fn symbol(self) -> &'static str {
        match self {
            Self::Not => "¬",
            Self::And => "∧",
            Self::Or => "∨",
            Self::Implies => "→",
            Self::Iff => "↔",
        }
    }

    /// Returns the connective a spelling names, Unicode or ASCII.
    fn of(spelling: &str) -> Option<Self> {
        Some(match spelling {
            "¬" | "~" => Self::Not,
            "∧" | "/\\" => Self::And,
            "∨" | "\\/" => Self::Or,
            "→" | "->" => Self::Implies,
            "↔" | "<->" => Self::Iff,
            _ => return None,
        })
    }
}

/// The rule a line names: the sign and the connective of the formula it
/// expands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Named {
    /// The sign.
    sign: Sign,
    /// The connective.
    connective: Connective,
}

impl Named {
    /// Reads a rule's name as a tableau writes it (`T∧`, `∧T`, `T/\`) or
    /// as LK names it (`∧L` for a true formula, `∧R` for a false one),
    /// with an index (`₁`, `₂`, `1`, `2`) after it, which says nothing
    /// more.
    fn read(word: &str) -> Option<Self> {
        let word = word.trim_end_matches(['₁', '₂', '1', '2']);
        let sign = |letter: &str| match letter {
            "T" | "L" => Some(Sign::True),
            "F" | "R" => Some(Sign::False),
            _ => None,
        };
        let first = word.get(..1).and_then(|letter| match letter {
            "T" | "F" => sign(letter).zip(Connective::of(&word[1..])),
            _ => None,
        });
        let last = word.len().checked_sub(1).and_then(|cut| {
            word.get(cut..)
                .and_then(sign)
                .zip(word.get(..cut).and_then(Connective::of))
        });
        first
            .or(last)
            .map(|(sign, connective)| Self { sign, connective })
    }

    /// Returns the name as a tableau writes it, `T∧`.
    fn name(self) -> String {
        format!("{}{}", self.sign.letter(), self.connective.symbol())
    }
}

/// A numbered line of a written tableau.
#[derive(Clone, Debug)]
struct Line {
    /// The number the line was written with.
    number: u32,
    /// The text line it stands on, from 1.
    row: u32,
    /// The signed formula.
    signed: Signed,
    /// The number of the line it expands, if it names one.
    cites: Option<u32>,
    /// The rule it names, if it names one.
    rule: Option<Named>,
}

/// How a branch is marked at its end.
#[derive(Clone, Debug)]
enum Mark {
    /// `×`, with the numbers of the lines it names (none, one or two).
    Closed(Vec<u32>),
    /// `open`.
    Open,
}

/// A branch of a written tableau: the lines it adds to its parent's, and
/// what comes after them.
#[derive(Clone, Debug, Default)]
struct Branch {
    /// The branch it goes on from; `None` for the root.
    parent: Option<u32>,
    /// Its lines, by index, in the order written.
    lines: Vec<u32>,
    /// The branches it splits into, in the order written.
    children: Vec<u32>,
    /// Its mark, with the text line the mark stands on.
    mark: Option<(Mark, u32)>,
}

/// A signed tableau of classical logic written by hand, read from text.
///
/// Each line of the text is one of these, after any indentation:
/// - a numbered line: its number (`3` or `3.`), its sign `T` (true) or
///   `F` (false), the formula in the syntax of an ordinary sequent, and at
///   its end the number of the line it expands, which the rule's name may
///   precede (`T∧ 2`, `∧L 2`, `T/\ 2`). The first lines, which name no
///   line, are the sequent's: its hypotheses true and its conclusions
///   false;
/// - `×` (or `x`, `*`, `⊗`) under the last line of a branch that closes,
///   with the two lines of a formula true and false (`× 3, 5`) or the line
///   of `⊥` true or `⊤` false, or none;
/// - `open` under the last line of a branch that stays open;
/// - an empty line, which is skipped.
///
/// A line goes on the branch whose lines it is indented with. A branch
/// starts with `├─`, `└─`, `|-`, `+-`, `` `- `` or `-` in the column of
/// the lines it goes on from, and its lines are indented to the column
/// after the marker: the text `deduxit prove --method tableau` prints,
/// and a plain list as well. A split's branches each start with the
/// results of one expansion.
///
/// [`review`](Self::review) checks it rule by rule.
///
/// Needs the cargo features `parse`, `serialize` and `interactive` (all
/// on by default).
#[derive(Clone, Debug)]
pub struct WrittenTableau {
    /// The formulas of the lines.
    formulas: Formulas,
    /// The numbered lines, in the order written.
    lines: Vec<Line>,
    /// The branches, the root first, each before the branches it splits
    /// into, in the order written.
    branches: Vec<Branch>,
}

/// How a reviewed tableau stands.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Standing {
    /// Every step follows the rules and every branch closes: the sequent
    /// is valid.
    Closed,
    /// Every step follows the rules, every branch is finished, and a
    /// branch is open: the sequent is not valid.
    Open,
    /// Every step so far follows the rules, and a branch is not finished
    /// or not marked.
    Unfinished,
    /// A step does not follow the rules, or a branch is marked wrongly.
    Wrong,
}

impl Standing {
    /// Returns the standing as a word: `closed`, `open`, `unfinished` or
    /// `wrong`.
    pub const fn word(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
            Self::Unfinished => "unfinished",
            Self::Wrong => "wrong",
        }
    }
}

/// What the review of a written tableau found.
///
/// With the feature `serialize` it is written as `{"version": 14,
/// "standing": "wrong", "sequent": {…}, "text_line": 5, "line": 4,
/// "message": "line 4: …"}`, with `countermodel` (`{"sequent", "model"}`)
/// for an open tableau and without `text_line` and `line` where the
/// message names no place.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct TableauReview {
    /// How the tableau stands.
    pub standing: Standing,
    /// The sequent its first lines give.
    pub sequent: Sequent,
    /// The text line, from 1, that the message is about: the first wrong
    /// step or mark, the end of the first branch not finished, or of the
    /// open branch.
    pub text_line: Option<u32>,
    /// The number of the tableau's line that the message is about.
    pub line: Option<u32>,
    /// What the review found, in words, naming the lines by their
    /// numbers.
    pub message: String,
    /// For an open tableau, the model its open branch gives, checked
    /// against the sequent; `None` where it is too large to check.
    pub countermodel: Option<Countermodel>,
}

/// Returns a parse error at byte `at` of `text`, with what could have
/// stood there and a hint.
fn refused(text: &str, at: usize, expected: &'static [&'static str], hint: &'static str) -> Error {
    ParseError::new(text, at, expected).with_hint(hint).into()
}

/// The hint for a numbered line that is not one.
const LINE_SHAPE: &str =
    "a line is its number, T or F, the formula, and the number of the line it expands";

/// The hint for a line indented under no branch.
const INDENT: &str = "indent a line as the other lines of its branch, or start a branch with `-`";

impl WrittenTableau {
    /// Reads a tableau from its text, the formulas within
    /// `limits.occurrences`.
    ///
    /// # Errors
    ///
    /// [`Error::Parse`] at the first place that is no tableau, with a
    /// hint where the layout is the fault; [`Error::AtomName`] and
    /// [`Refusal::Occurrences`] as the sequent's parser gives them.
    pub fn parse_within(text: &str, limits: &Limits) -> Result<Self, Error> {
        let mut tableau = Self {
            formulas: Formulas::default(),
            lines: Vec::new(),
            branches: Vec::new(),
        };
        let mut numbers: HashSet<u32> = HashSet::default();
        // The branches a line may go on, the innermost last, each with
        // the column of its lines.
        let mut stack: Vec<(u32, usize)> = Vec::new();
        let mut start = 0;
        for (index, piece) in text.split('\n').enumerate() {
            let at = start;
            start += piece.len() + 1;
            let row = u32::try_from(index + 1).map_err(|_| too_many(index))?;
            let line = piece.strip_suffix('\r').unwrap_or(piece);
            // The layout: spaces, bars and a branch's marker.
            let (mut column, mut marker, mut previous) = (0, None, None);
            let mut content = line.len();
            for (offset, c) in line.char_indices() {
                match c {
                    ' ' | '\t' | '│' | '|' | '─' => {}
                    '├' | '└' | '+' | '\\' | '`' | '-' => {
                        if marker.is_none() {
                            let bar = c == '-' && matches!(previous, Some('|' | '│'));
                            marker = Some(if bar { column - 1 } else { column });
                        }
                    }
                    _ => {
                        content = offset;
                        break;
                    }
                }
                previous = Some(c);
                column += 1;
            }
            let body = &line[content..];
            if body.trim().is_empty() {
                if marker.is_some() {
                    return Err(refused(
                        text,
                        at + line.len(),
                        &["a line number"],
                        "a branch starts with a numbered line",
                    ));
                }
                continue;
            }
            let content = at + content;
            let item = tableau.item(text, content, at + line.len(), row, limits)?;
            // The branch the item goes on.
            let branch = match (marker, stack.last()) {
                (None, None) => {
                    tableau.branches.push(Branch::default());
                    stack.push((0, column));
                    0
                }
                (Some(_), None) => {
                    return Err(refused(
                        text,
                        content,
                        &[],
                        "the first lines are the sequent's and start no branch",
                    ));
                }
                (Some(marker), Some(_)) => {
                    while stack.last().is_some_and(|&(_, col)| col > marker) {
                        stack.pop();
                    }
                    let Some(&(parent, _)) = stack.last() else {
                        return Err(refused(text, content, &[], INDENT));
                    };
                    if tableau.branches[parent as usize].mark.is_some() {
                        return Err(refused(
                            text,
                            content,
                            &[],
                            "a branch ends at its mark, × or open",
                        ));
                    }
                    if matches!(item, Item::Mark(_)) {
                        return Err(refused(
                            text,
                            content,
                            &["a line number"],
                            "a branch starts with a numbered line",
                        ));
                    }
                    let id = u32::try_from(tableau.branches.len())
                        .map_err(|_| too_many(tableau.branches.len()))?;
                    tableau.branches.push(Branch {
                        parent: Some(parent),
                        ..Branch::default()
                    });
                    tableau.branches[parent as usize].children.push(id);
                    stack.push((id, column));
                    id
                }
                (None, Some(_)) => {
                    while stack.last().is_some_and(|&(_, col)| col > column) {
                        stack.pop();
                    }
                    match stack.last() {
                        Some(&(branch, col)) if col == column => {
                            let held = &tableau.branches[branch as usize];
                            if held.mark.is_some() {
                                return Err(refused(
                                    text,
                                    content,
                                    &[],
                                    "a branch ends at its mark, × or open",
                                ));
                            }
                            if !held.children.is_empty() {
                                return Err(refused(
                                    text,
                                    content,
                                    &[],
                                    "the lines after a split go in one of its branches",
                                ));
                            }
                            branch
                        }
                        _ => return Err(refused(text, content, &[], INDENT)),
                    }
                }
            };
            match item {
                Item::Line(line) => {
                    if !numbers.insert(line.number) {
                        return Err(refused(
                            text,
                            content,
                            &[],
                            "each line has a number of its own",
                        ));
                    }
                    let id = u32::try_from(tableau.lines.len())
                        .map_err(|_| too_many(tableau.lines.len()))?;
                    tableau.lines.push(line);
                    tableau.branches[branch as usize].lines.push(id);
                }
                Item::Mark(mark) => {
                    if tableau.branches[branch as usize].lines.is_empty() {
                        return Err(refused(
                            text,
                            content,
                            &["a line number"],
                            "the tableau starts with the sequent's lines",
                        ));
                    }
                    tableau.branches[branch as usize].mark = Some((mark, row));
                }
            }
        }
        if tableau.lines.is_empty() {
            return Err(refused(text, text.len(), &["a line number"], LINE_SHAPE));
        }
        Ok(tableau)
    }

    /// Reads the item at `text[content..end]`: a numbered line or a mark.
    fn item(
        &mut self,
        text: &str,
        content: usize,
        end: usize,
        row: u32,
        limits: &Limits,
    ) -> Result<Item, Error> {
        let body = &text[content..end];
        let first = body.chars().next().unwrap_or(' ');
        if first.is_ascii_digit() {
            return self
                .numbered(text, content, end, row, limits)
                .map(Item::Line);
        }
        if body
            .get(..4)
            .is_some_and(|w| w.eq_ignore_ascii_case("open"))
        {
            let rest = &body[4..];
            if rest.trim().is_empty() {
                return Ok(Item::Mark(Mark::Open));
            }
            let at = content + 4 + (rest.len() - rest.trim_start().len());
            return Err(refused(
                text,
                at,
                &["the end of the line"],
                "`open` stands alone",
            ));
        }
        if matches!(first, '×' | '⊗' | 'x' | 'X' | '*') {
            let rest = &body[first.len_utf8()..];
            let starts = rest.chars().next();
            if starts.is_none_or(|c| c.is_whitespace() || c.is_ascii_digit()) {
                let mut cites = Vec::new();
                let base = content + first.len_utf8();
                for (offset, word) in words(rest, |c| c.is_whitespace() || c == ',') {
                    cites.push(number(text, base + offset, word)?);
                }
                return Ok(Item::Mark(Mark::Closed(cites)));
            }
        }
        Err(refused(
            text,
            content,
            &["a line number", "`×`", "`open`"],
            LINE_SHAPE,
        ))
    }

    /// Reads the numbered line at `text[content..end]`.
    fn numbered(
        &mut self,
        text: &str,
        content: usize,
        end: usize,
        row: u32,
        limits: &Limits,
    ) -> Result<Line, Error> {
        let body = &text[content..end];
        let digits = body
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(body.len());
        let line_number = number(text, content, &body[..digits])?;
        let mut at = digits + usize::from(body[digits..].starts_with('.'));
        let gap = body[at..].len() - body[at..].trim_start().len();
        let sign_at = at + gap;
        let sign = match body.get(sign_at..sign_at + 1) {
            Some("T") => Sign::True,
            Some("F") => Sign::False,
            _ => {
                return Err(refused(
                    text,
                    content + sign_at,
                    &["`T`", "`F`"],
                    LINE_SHAPE,
                ));
            }
        };
        if gap == 0 || !body[sign_at + 1..].starts_with(char::is_whitespace) {
            let place = if gap == 0 { at } else { sign_at + 1 };
            return Err(refused(text, content + place, &["a space"], LINE_SHAPE));
        }
        at = sign_at + 1;
        // The formula, then the number of the line it expands and the
        // rule's name before it, if the line names them.
        let rest = body[at..].trim_end();
        let mut formula_end = at + rest.len();
        let (mut cites, mut rule) = (None, None);
        // Where the formula ends if the word before the number is no rule.
        let mut whole_end = formula_end;
        if let Some((head, last)) = rest.rsplit_once(char::is_whitespace)
            && !last.is_empty()
            && last.bytes().all(|b| b.is_ascii_digit())
            && !head.trim().is_empty()
        {
            cites = Some(number(text, content + at + rest.len() - last.len(), last)?);
            formula_end = at + head.trim_end().len();
            whole_end = formula_end;
            let head = head.trim_end();
            if let Some((before, word)) = head.rsplit_once(char::is_whitespace)
                && !before.trim().is_empty()
                && let Some(named) = Named::read(word)
            {
                rule = Some(named);
                formula_end = at + before.trim_end().len();
            }
        }
        let span = |end: usize| content + at..content + end;
        let formula = match formula_in(text, span(formula_end), &mut self.formulas, limits) {
            // A word that reads as a rule's name may end the formula
            // instead, as `¬R` does in `T p ∧ ¬R 1`: a formula cannot end
            // where the name would leave it (a negation needs a
            // connective before it), so this is the only other reading.
            Err(Error::Parse(_)) if rule.is_some() => {
                let formula = formula_in(text, span(whole_end), &mut self.formulas, limits);
                if formula.is_ok() {
                    rule = None;
                }
                formula?
            }
            read => read?,
        };
        Ok(Line {
            number: line_number,
            row,
            signed: Signed::new(sign, formula),
            cites,
            rule,
        })
    }

    /// Returns the sequent the tableau's first lines give: those signed
    /// `T` left of `⊢` and those signed `F` right of it, in the order
    /// written.
    ///
    /// # Errors
    ///
    /// None in practice: the arena is the tableau's own.
    pub fn sequent(&self) -> Result<Sequent, Error> {
        let (mut left, mut right) = (Vec::new(), Vec::new());
        for &id in self.root_lines() {
            let signed = self.lines[id as usize].signed;
            match signed.sign {
                Sign::True => left.push(signed.formula),
                Sign::False => right.push(signed.formula),
            }
        }
        Sequent::new(self.formulas.clone(), left, right)
    }

    /// Returns the root's first lines, those that name no line.
    fn root_lines(&self) -> &[u32] {
        let lines = &self.branches[0].lines;
        let count = lines
            .iter()
            .take_while(|&&id| self.lines[id as usize].cites.is_none())
            .count();
        &lines[..count]
    }

    /// Reviews the tableau rule by rule against `calculus`, which is the
    /// rule file [`TABLEAU`](crate::rules::TABLEAU) read, and, where
    /// `expected` is given, checks that it starts from that sequent.
    ///
    /// The review walks the lines in the order written. Each line that
    /// names a line must be one result of expanding it, the lines that
    /// expand it together all of them, and a split's two branches must
    /// start with its two results; a mark `×` must name, or the branch
    /// hold, a formula true and false, `⊥` true or `⊤` false; a mark
    /// `open` must stand under a branch that nothing closes and on which
    /// every formula is expanded. The first fault, by the text's order,
    /// is the answer, said in words with the line it is at.
    ///
    /// A tableau with no fault is then written as a derivation in the
    /// rule file, a node per expansion with its branch's signed formulas,
    /// and checked by [`Derivation::check`]: a closed tableau must pass
    /// it whole and is the evidence that the sequent is valid; an open or
    /// unfinished one must pass it but for its open branches. An open
    /// tableau's first open branch gives a one-world model, its atoms
    /// those true on the branch, checked to refute the sequent. So a
    /// standing of [`Closed`](Standing::Closed) or [`Open`](Standing::Open)
    /// rests on the rule file's checker and the model's check, not on
    /// the review's own reading of the rules.
    ///
    /// The stop is asked at every branch and every node checked.
    ///
    /// # Errors
    ///
    /// [`Refusal::Memory`] where the derivation would hold more than
    /// `limits.memory_bytes`, the checker's refusals, and
    /// [`Refusal::Stopped`] when the stop fired; none is a verdict.
    pub fn review(
        &self,
        calculus: &Calculus,
        expected: Option<&Sequent>,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<TableauReview, Error> {
        self.review_dyn(calculus, expected, limits, &mut stop)
    }

    /// [`review`](Self::review), its stop as a trait object.
    fn review_dyn(
        &self,
        calculus: &Calculus,
        expected: Option<&Sequent>,
        limits: &Limits,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<TableauReview, Error> {
        let sequent = self.sequent()?;
        let answer = |standing, place: Option<&Line>, message| TableauReview {
            standing,
            sequent: sequent.clone(),
            text_line: place.map(|line| line.row),
            line: place.map(|line| line.number),
            message,
            countermodel: None,
        };
        if let Some(expected) = expected
            && let Some(message) = self.differs(expected)?
        {
            // The first line, whether or not it is the sequent's.
            let first = &self.lines[self.branches[0].lines[0] as usize];
            return Ok(answer(Standing::Wrong, Some(first), message));
        }
        let mut walk = Walk::new(self, limits);
        walk.run(stop)?;
        if let Some(fault) = walk.fault {
            return Ok(TableauReview {
                standing: Standing::Wrong,
                sequent: sequent.clone(),
                text_line: Some(fault.row),
                line: fault.line,
                message: fault.message,
                countermodel: None,
            });
        }
        // The rule file's checker has the last word on every step.
        let closed = walk.leaves.iter().all(|leaf| leaf.closed);
        let derivation = walk.derivation()?;
        let checked = if closed {
            derivation.check(calculus, limits, &mut *stop)
        } else {
            derivation.check_open(calculus, limits, &mut *stop)
        };
        match checked {
            Ok(()) => {}
            Err(Error::InvalidDerivation { reason, .. }) => {
                return Ok(answer(
                    Standing::Wrong,
                    None,
                    format!("the tableau's rule file refuses it: {reason}"),
                ));
            }
            Err(error) => return Err(error),
        }
        if closed {
            return Ok(answer(
                Standing::Closed,
                None,
                "every step follows the rules and every branch closes".to_owned(),
            ));
        }
        if let Some(leaf) = walk.leaves.iter().find(|leaf| leaf.unfinished.is_some()) {
            let last = &self.lines[leaf.last as usize];
            let mut review = answer(
                Standing::Unfinished,
                Some(last),
                format!(
                    "every step so far follows the rules; the branch ending at line {} {}",
                    last.number,
                    leaf.unfinished.as_deref().unwrap_or_default()
                ),
            );
            review.text_line = Some(leaf.row);
            return Ok(review);
        }
        let Some(leaf) = walk.leaves.iter().find(|leaf| !leaf.closed) else {
            unreachable!("a tableau not closed has an open branch")
        };
        let last = &self.lines[leaf.last as usize];
        let atoms = self.true_atoms(&leaf.branch);
        let model = Model::new(vec![World::new(atoms, false, Vec::new())], 0);
        let countermodel =
            Countermodel::found(sequent.clone(), model, Logic::Classical, limits, stop)?;
        let mut review = answer(
            Standing::Open,
            Some(last),
            format!(
                "every step follows the rules, every branch is finished, and the branch \
                 ending at line {} is open",
                last.number
            ),
        );
        review.text_line = Some(leaf.row);
        review.countermodel = countermodel;
        Ok(review)
    }

    /// Returns the names of the atoms true on a branch, each once, sorted.
    fn true_atoms(&self, branch: &[u32]) -> Vec<String> {
        let mut atoms: Vec<String> = branch
            .iter()
            .filter_map(|&id| {
                let signed = self.lines[id as usize].signed;
                match (signed.sign, self.formulas.node(signed.formula)) {
                    (Sign::True, Node::Atom(atom)) => {
                        Some(self.formulas.atom_name(atom).to_owned())
                    }
                    (Sign::True | Sign::False, _) => None,
                }
            })
            .collect();
        atoms.sort_unstable();
        atoms.dedup();
        atoms
    }

    /// Returns what tells the tableau's first lines from the sequent
    /// `expected`, or `None` where they are its formulas, each signed by
    /// its side (as sets).
    fn differs(&self, expected: &Sequent) -> Result<Option<String>, Error> {
        let mut formulas = self.formulas.clone();
        let source = expected.formulas();
        let mut ids: Vec<NodeId> = Vec::with_capacity(source.len());
        for &node in source.nodes() {
            let at = |id: NodeId| ids[id.index()];
            let id = match node {
                Node::Atom(atom) => formulas.atom(source.atom_name(atom))?,
                Node::True | Node::False => formulas.add(node)?,
                Node::Not(a) => formulas.add(Node::Not(at(a)))?,
                Node::And(a, b) => formulas.add(Node::And(at(a), at(b)))?,
                Node::Or(a, b) => formulas.add(Node::Or(at(a), at(b)))?,
                Node::Implies(a, b) => formulas.add(Node::Implies(at(a), at(b)))?,
                Node::Iff(a, b) => formulas.add(Node::Iff(at(a), at(b)))?,
            };
            ids.push(id);
        }
        let mut wanted: Vec<(bool, NodeId)> = expected
            .left()
            .iter()
            .map(|&x| (true, ids[x.index()]))
            .chain(expected.right().iter().map(|&x| (false, ids[x.index()])))
            .collect();
        let mut written: Vec<(bool, NodeId)> = self
            .root_lines()
            .iter()
            .map(|&id| {
                let signed = self.lines[id as usize].signed;
                (signed.sign == Sign::True, signed.formula)
            })
            .collect();
        for list in [&mut wanted, &mut written] {
            list.sort_unstable();
            list.dedup();
        }
        if wanted == written {
            return Ok(None);
        }
        Ok(Some(format!(
            "the first lines give the sequent {}, and the tableau is to be of {expected}",
            self.sequent()?
        )))
    }

    /// Writes a line as `line 3 (T p ∧ q)`.
    fn cite(&self, id: u32) -> String {
        let line = &self.lines[id as usize];
        format!("line {} ({})", line.number, self.signed(line.signed))
    }

    /// Writes a signed formula as `T p ∧ q`.
    fn signed(&self, signed: Signed) -> String {
        format!(
            "{} {}",
            signed.sign.letter(),
            self.formulas.formula(signed.formula)
        )
    }

    /// Writes signed formulas joined by commas and a last "and".
    fn list(&self, signed: &[Signed]) -> String {
        let mut out = String::new();
        for (k, &s) in signed.iter().enumerate() {
            if k > 0 {
                out.push_str(if k + 1 == signed.len() { " and " } else { ", " });
            }
            out.push_str(&self.signed(s));
        }
        out
    }
}

/// A line of the text: a numbered line or a branch's mark.
enum Item {
    /// A numbered line.
    Line(Line),
    /// A mark.
    Mark(Mark),
}

/// Returns the words of `text` split by `split`, each with its byte
/// offset.
fn words(text: &str, split: impl Fn(char) -> bool) -> impl Iterator<Item = (usize, &str)> {
    // A separator may be several bytes long (U+3000 is white space), so
    // each word's offset is read off its place in the text.
    let start = text.as_ptr() as usize;
    text.split(split)
        .filter(|word| !word.is_empty())
        .map(move |word| (word.as_ptr() as usize - start, word))
}

/// Reads a line's number at byte `at` of `text`.
fn number(text: &str, at: usize, word: &str) -> Result<u32, Error> {
    word.parse::<u32>().map_err(|_| {
        let end = at + word.len().max(1).min(text.len() - at);
        ParseError::spanning(text, at..end, &["a line number below 2³²"]).into()
    })
}

/// Returns the refusal of more lines or branches than a `u32` numbers.
fn too_many(count: usize) -> Error {
    Error::Refused(Refusal::Index {
        what: crate::limits::Space::Inference,
        count: count as u64,
        most: u64::from(u32::MAX),
    })
}

/// A rule of the tableau's rule file that expands a signed formula.
#[derive(Clone, Debug)]
struct Shape {
    /// Its name in the rule file.
    name: &'static str,
    /// Its name as a student reads it.
    shown: &'static str,
    /// What each branch it makes adds: one list without a split, two
    /// with one.
    results: Vec<Vec<Signed>>,
    /// Whether it writes only part of a fuller rule's results.
    partial: bool,
}

impl Shape {
    /// Returns a shape.
    fn new(name: &'static str, shown: &'static str, results: Vec<Vec<Signed>>) -> Self {
        Self {
            name,
            shown,
            results,
            partial: false,
        }
    }

    /// Returns the shape as part of a fuller rule.
    fn part(mut self) -> Self {
        self.partial = true;
        self
    }
}

/// Returns every rule of the tableau that expands `signed`, the textbook
/// form first: none for an atom, `⊤` or `⊥`.
fn shapes(formulas: &Formulas, signed: Signed) -> Vec<Shape> {
    use Sign::{False as F, True as T};
    let s = Signed::new;
    let implication = |a, b| formulas.find(Node::Implies(a, b));
    match (signed.sign, formulas.node(signed.formula)) {
        (_, Node::Atom(_) | Node::True | Node::False) => Vec::new(),
        (T, Node::Not(a)) => vec![Shape::new("T¬", "T¬", vec![vec![s(F, a)]])],
        (F, Node::Not(a)) => vec![Shape::new("F¬", "F¬", vec![vec![s(T, a)]])],
        (T, Node::And(a, b)) => vec![
            Shape::new("T∧", "T∧", vec![vec![s(T, a), s(T, b)]]),
            Shape::new("T∧₁", "T∧", vec![vec![s(T, a)]]).part(),
            Shape::new("T∧₂", "T∧", vec![vec![s(T, b)]]).part(),
        ],
        (F, Node::And(a, b)) => vec![Shape::new("F∧", "F∧", vec![vec![s(F, a)], vec![s(F, b)]])],
        (T, Node::Or(a, b)) => vec![Shape::new("T∨", "T∨", vec![vec![s(T, a)], vec![s(T, b)]])],
        (F, Node::Or(a, b)) => vec![
            Shape::new("F∨", "F∨", vec![vec![s(F, a), s(F, b)]]),
            Shape::new("F∨₁", "F∨", vec![vec![s(F, a)]]).part(),
            Shape::new("F∨₂", "F∨", vec![vec![s(F, b)]]).part(),
        ],
        (T, Node::Implies(a, b)) => {
            vec![Shape::new("T→", "T→", vec![vec![s(F, a)], vec![s(T, b)]])]
        }
        (F, Node::Implies(a, b)) => {
            vec![
                Shape::new("F→", "F→", vec![vec![s(T, a), s(F, b)]]),
                Shape::new("F→₁", "F→", vec![vec![s(T, a)]]).part(),
                Shape::new("F→₂", "F→", vec![vec![s(F, b)]]).part(),
            ]
        }
        (T, Node::Iff(a, b)) => {
            let mut all = vec![Shape::new(
                "T↔′",
                "T↔",
                vec![vec![s(T, a), s(T, b)], vec![s(F, a), s(F, b)]],
            )];
            let (there, back) = (implication(a, b), implication(b, a));
            if let (Some(there), Some(back)) = (there, back) {
                all.push(Shape::new("T↔", "T↔", vec![vec![s(T, there), s(T, back)]]));
            }
            if let Some(there) = there {
                all.push(Shape::new("T↔₁", "T↔", vec![vec![s(T, there)]]).part());
            }
            if let Some(back) = back {
                all.push(Shape::new("T↔₂", "T↔", vec![vec![s(T, back)]]).part());
            }
            all
        }
        (F, Node::Iff(a, b)) => {
            let mut all = vec![Shape::new(
                "F↔′",
                "F↔",
                vec![vec![s(T, a), s(F, b)], vec![s(F, a), s(T, b)]],
            )];
            if let (Some(there), Some(back)) = (implication(a, b), implication(b, a)) {
                all.push(Shape::new(
                    "F↔",
                    "F↔",
                    vec![vec![s(F, there)], vec![s(F, back)]],
                ));
            }
            all
        }
    }
}

/// Returns whether every member of `part` is in `whole`, counted as
/// multisets.
fn within(part: &[Signed], whole: &[Signed]) -> bool {
    let mut free = vec![true; whole.len()];
    part.iter().all(|x| {
        let found = (0..whole.len()).find(|&k| free[k] && whole[k] == *x);
        found.is_some_and(|k| {
            free[k] = false;
            true
        })
    })
}

/// The first fault the review found.
#[derive(Clone, Debug)]
struct Fault {
    /// The text line it is at.
    row: u32,
    /// The tableau's line it is at, if any.
    line: Option<u32>,
    /// What is wrong.
    message: String,
}

/// The end of a branch, as the review found it.
#[derive(Clone, Debug)]
struct Leaf {
    /// The branch's last line.
    last: u32,
    /// The text line the branch ends on: its mark's, else its last
    /// line's.
    row: u32,
    /// Whether it closes.
    closed: bool,
    /// For a branch not finished or not marked, what is left, in words.
    unfinished: Option<String>,
    /// The lines on the branch, kept for an open one only.
    branch: Vec<u32>,
}

/// A node of the derivation the tableau is written as, planned in the
/// order of the text (each before its premises).
#[derive(Clone, Debug)]
struct Planned {
    /// The lines of its branch, each signed formula once.
    branch: Vec<u32>,
    /// The rule, or `None` for an open goal.
    rule: Option<&'static str>,
    /// The lines its positions name.
    positions: Vec<u32>,
    /// Its premises, by planned index (`usize::MAX` until planned).
    premises: Vec<usize>,
}

/// Per branch of a split: how many of its first lines the split took,
/// and where its first node goes.
type Passed = (usize, Option<(usize, usize)>);

/// The walk of the review over a tableau's branches.
struct Walk<'a> {
    /// The tableau.
    tableau: &'a WrittenTableau,
    /// The bounds.
    limits: &'a Limits,
    /// Each line's index by its number.
    by_number: HashMap<u32, u32>,
    /// The lines on the current branch, in order.
    path: Vec<u32>,
    /// Whether each line is on the current branch.
    on_path: Vec<bool>,
    /// How many times each signed formula is on the current branch.
    present: HashMap<Signed, u32>,
    /// The first fault, by text line.
    fault: Option<Fault>,
    /// Each branch's end, in the order written.
    leaves: Vec<Leaf>,
    /// The derivation's nodes.
    planned: Vec<Planned>,
    /// The bytes the planned nodes hold.
    held: u64,
}

impl<'a> Walk<'a> {
    /// Returns a walk over `tableau`.
    fn new(tableau: &'a WrittenTableau, limits: &'a Limits) -> Self {
        let by_number = tableau
            .lines
            .iter()
            .enumerate()
            .map(|(id, line)| (line.number, id as u32))
            .collect();
        Self {
            tableau,
            limits,
            by_number,
            path: Vec::new(),
            on_path: vec![false; tableau.lines.len()],
            present: HashMap::default(),
            fault: None,
            leaves: Vec::new(),
            planned: Vec::new(),
            held: 0,
        }
    }

    /// Notes a fault at a line, unless one earlier in the text is noted.
    fn fault(&mut self, at: u32, message: String) {
        let line = &self.tableau.lines[at as usize];
        self.fault_at(line.row, Some(line.number), message);
    }

    /// Notes a fault at a text line, unless one earlier is noted.
    fn fault_at(&mut self, row: u32, line: Option<u32>, message: String) {
        if self.fault.as_ref().is_none_or(|fault| row < fault.row) {
            self.fault = Some(Fault { row, line, message });
        }
    }

    /// Puts a line on the current branch.
    fn push(&mut self, id: u32) {
        self.path.push(id);
        self.on_path[id as usize] = true;
        *self
            .present
            .entry(self.tableau.lines[id as usize].signed)
            .or_default() += 1;
    }

    /// Takes the current branch back to its first `len` lines.
    fn truncate(&mut self, len: usize) {
        while self.path.len() > len {
            let Some(id) = self.path.pop() else { break };
            self.on_path[id as usize] = false;
            let signed = self.tableau.lines[id as usize].signed;
            if let Some(count) = self.present.get_mut(&signed) {
                *count -= 1;
                if *count == 0 {
                    self.present.remove(&signed);
                }
            }
        }
    }

    /// Returns whether a signed formula is on the current branch.
    fn holds(&self, signed: Signed) -> bool {
        self.present.contains_key(&signed)
    }

    /// Plans a node on the current branch and links it where `slot`
    /// says; returns its index.
    fn plan(
        &mut self,
        slot: Option<(usize, usize)>,
        rule: Option<&'static str>,
        positions: Vec<u32>,
        premises: usize,
    ) -> Result<usize, Error> {
        let mut seen: HashSet<Signed> = HashSet::default();
        let branch: Vec<u32> = self
            .path
            .iter()
            .copied()
            .filter(|&id| seen.insert(self.tableau.lines[id as usize].signed))
            .collect();
        self.held = self
            .held
            .saturating_add(NODE_BYTES)
            .saturating_add(MEMBER_BYTES.saturating_mul(branch.len() as u64));
        if let Some(limit) = self.limits.memory_bytes
            && self.held > limit
        {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Check,
                limit_bytes: limit,
                needed_bytes: None,
            }));
        }
        let index = self.planned.len();
        self.planned.push(Planned {
            branch,
            rule,
            positions,
            premises: vec![usize::MAX; premises],
        });
        if let Some((parent, k)) = slot {
            self.planned[parent].premises[k] = index;
        }
        Ok(index)
    }

    /// Walks every branch in the order written.
    fn run(&mut self, stop: &mut dyn FnMut(Progress) -> bool) -> Result<(), Error> {
        let tableau = self.tableau;
        // Per branch: the length of the path before its lines, how many
        // of its first lines its parent's split already took, and where
        // its first node goes.
        let count = tableau.branches.len();
        let mut skip = vec![0usize; count];
        let mut slots: Vec<Option<(usize, usize)>> = vec![None; count];
        let mut ends = vec![0usize; count];
        for (index, branch) in tableau.branches.iter().enumerate() {
            if stop(Progress::new(Phase::Check, 1, index as u64)) {
                return Err(Error::Refused(Refusal::Stopped {
                    phase: Phase::Check,
                }));
            }
            let from = branch.parent.map_or(0, |parent| ends[parent as usize]);
            self.truncate(from);
            let mut slot = slots[index];
            // The root's first lines are the sequent's; a branch's first
            // lines are the results its parent's split checked.
            let given = if index == 0 {
                tableau.root_lines()
            } else {
                &branch.lines[..skip[index]]
            };
            for &id in given {
                self.push(id);
            }
            let mut cursor = given.len();
            while cursor < branch.lines.len() {
                let id = branch.lines[cursor];
                let taken = self.expansion(&branch.lines[cursor..], &mut slot)?;
                for &line in &branch.lines[cursor..cursor + taken] {
                    self.push(line);
                }
                if taken == 0 {
                    // A faulty line: kept on the branch so that the walk
                    // goes on.
                    self.push(id);
                    cursor += 1;
                } else {
                    cursor += taken;
                }
            }
            ends[index] = self.path.len();
            if branch.children.is_empty() {
                self.leaf(index, slot)?;
            } else {
                let taken = self.split(branch, slot)?;
                for (k, &child) in branch.children.iter().enumerate() {
                    if let Some(&(count, child_slot)) = taken.get(k) {
                        skip[child as usize] = count;
                        slots[child as usize] = child_slot;
                    }
                }
            }
        }
        Ok(())
    }

    /// Checks the line a run of lines cites: that it exists, is on the
    /// branch above them and can be expanded. Returns its index.
    fn cited(&mut self, first: u32) -> Option<u32> {
        let tableau = self.tableau;
        let line = &tableau.lines[first as usize];
        let Some(number) = line.cites else {
            self.fault(
                first,
                format!(
                    "line {} names no line it expands: only the first lines, the sequent's, \
                     stand without one",
                    line.number
                ),
            );
            return None;
        };
        let Some(&cited) = self.by_number.get(&number) else {
            self.fault(
                first,
                format!(
                    "line {} expands line {number}, which the tableau does not have",
                    line.number
                ),
            );
            return None;
        };
        if !self.on_path[cited as usize] {
            let after = cited >= first;
            self.fault(
                first,
                format!(
                    "line {} expands line {number}, which {}",
                    line.number,
                    if after {
                        "comes after it"
                    } else {
                        "is not on its branch"
                    }
                ),
            );
            return None;
        }
        let signed = tableau.lines[cited as usize].signed;
        let shapes = shapes(&tableau.formulas, signed);
        if shapes.is_empty() {
            let why = match (signed.sign, tableau.formulas.node(signed.formula)) {
                (Sign::True, Node::False) | (Sign::False, Node::True) => {
                    "closes its branch, and no rule expands it"
                }
                (Sign::True, Node::True) | (Sign::False, Node::False) => {
                    "is true on every branch, and no rule expands it"
                }
                _ => "is an atom, which no rule expands",
            };
            self.fault(
                first,
                format!(
                    "line {} expands {}, which {why}",
                    line.number,
                    tableau.cite(cited)
                ),
            );
            return None;
        }
        if let Some(named) = line.rule {
            let shown = shapes[0].shown;
            if named.name() != shown {
                self.fault(
                    first,
                    format!(
                        "line {} names the rule {}, but {} is expanded by {shown}",
                        line.number,
                        named.name(),
                        tableau.cite(cited)
                    ),
                );
                return None;
            }
        }
        Some(cited)
    }

    /// Checks the expansion that `run` starts on the current branch and
    /// returns how many lines it takes, 0 at a fault.
    fn expansion(
        &mut self,
        run: &[u32],
        slot: &mut Option<(usize, usize)>,
    ) -> Result<usize, Error> {
        let tableau = self.tableau;
        let first = run[0];
        let Some(cited) = self.cited(first) else {
            return Ok(0);
        };
        let principal = tableau.lines[cited as usize].signed;
        let all = shapes(&tableau.formulas, principal);
        let ones: Vec<&Shape> = all.iter().filter(|s| s.results.len() == 1).collect();
        let number = tableau.lines[first as usize].number;
        // The rule as a whole (not a part of it), for the messages.
        let whole = all.iter().find(|shape| !shape.partial).unwrap_or(&all[0]);
        let group = if ones.is_empty() {
            0
        } else {
            self.group(run, cited, &ones)
        };
        let signed: Vec<Signed> = run[..group]
            .iter()
            .map(|&id| tableau.lines[id as usize].signed)
            .collect();
        if group == 0 {
            let message = if let [one, two] = whole.results.as_slice() {
                format!(
                    "line {number}: {} on {} splits the branch in two, one that starts with {} \
                     and one with {}",
                    whole.shown,
                    tableau.cite(cited),
                    tableau.list(one),
                    tableau.list(two)
                )
            } else {
                format!(
                    "line {number}: {} on {} adds {}, not {}",
                    whole.shown,
                    tableau.cite(cited),
                    tableau.list(&whole.results[0]),
                    tableau.signed(tableau.lines[first as usize].signed)
                )
            };
            self.fault(first, message);
            return Ok(0);
        }
        let complete = |shape: &Shape| {
            shape.results[0]
                .iter()
                .all(|&r| self.holds(r) || signed.contains(&r))
        };
        // Every rule that does not split has a part for each of its
        // results (`T∧₁`, `F→₂`, …), so a group within a result list is
        // within a list the branch completes: a part's own result.
        let Some(shape) = ones
            .iter()
            .copied()
            .find(|shape| within(&signed, &shape.results[0]) && complete(shape))
        else {
            unreachable!("a group fits a rule's part whenever it fits the rule")
        };
        let at = self.plan(*slot, Some(shape.name), vec![cited], 1)?;
        *slot = Some((at, 0));
        Ok(group)
    }

    /// Returns how many of the lines of `run` that cite `cited` one after
    /// the other fit together within a result list of the shapes given.
    fn group(&self, run: &[u32], cited: u32, shapes: &[&Shape]) -> usize {
        let tableau = self.tableau;
        let number = tableau.lines[cited as usize].number;
        let mut signed: Vec<Signed> = Vec::new();
        for &id in run {
            let line = &tableau.lines[id as usize];
            if line.cites != Some(number) {
                break;
            }
            signed.push(line.signed);
            let fits = shapes
                .iter()
                .any(|shape| shape.results.iter().any(|r| within(&signed, r)));
            if !fits {
                signed.pop();
                break;
            }
        }
        signed.len()
    }

    /// Checks the split at the end of `branch` and plans its node; returns
    /// per child how many of its first lines the split takes and where
    /// the child's first node goes.
    fn split(
        &mut self,
        branch: &Branch,
        slot: Option<(usize, usize)>,
    ) -> Result<Vec<Passed>, Error> {
        let tableau = self.tableau;
        let children = &branch.children;
        let first = |child: u32| tableau.branches[child as usize].lines[0];
        let head = first(children[0]);
        let Some(cited) = self.cited(head) else {
            return Ok(self.passed(children));
        };
        let number = tableau.lines[cited as usize].number;
        for &child in &children[1..] {
            let line = &tableau.lines[first(child) as usize];
            if line.cites != Some(number) {
                self.fault(
                    first(child),
                    format!(
                        "line {} starts a branch beside line {}, which expands line {number}: \
                         the branches of a split start with the results of one expansion",
                        line.number, tableau.lines[head as usize].number
                    ),
                );
                return Ok(self.passed(children));
            }
            if self.cited(first(child)).is_none() {
                return Ok(self.passed(children));
            }
        }
        let principal = tableau.lines[cited as usize].signed;
        let all = shapes(&tableau.formulas, principal);
        let twos: Vec<&Shape> = all.iter().filter(|s| s.results.len() == 2).collect();
        let head_number = tableau.lines[head as usize].number;
        if children.len() == 1 {
            // One branch goes on as the lines before it do.
            if twos.len() == all.len() {
                let shape = twos[0];
                self.fault(
                    head,
                    format!(
                        "line {head_number}: {} on {} splits the branch in two, one that starts \
                         with {} and one with {}; write the second beside it",
                        shape.shown,
                        tableau.cite(cited),
                        tableau.list(&shape.results[0]),
                        tableau.list(&shape.results[1])
                    ),
                );
                return Ok(self.passed(children));
            }
            let mut child_slot = slot;
            let child = &tableau.branches[children[0] as usize];
            let taken = self.expansion(&child.lines, &mut child_slot)?;
            return Ok(vec![(taken, child_slot)]);
        }
        if twos.is_empty() {
            let shape = &all[0];
            self.fault(
                head,
                format!(
                    "line {head_number}: {} on {} adds {} to the branch and does not split it",
                    shape.shown,
                    tableau.cite(cited),
                    tableau.list(&shape.results[0])
                ),
            );
            return Ok(self.passed(children));
        }
        if children.len() > 2 {
            let third = first(children[2]);
            self.fault(
                third,
                format!(
                    "line {}: {} on {} splits the branch in two, and this is a third branch",
                    tableau.lines[third as usize].number,
                    twos[0].shown,
                    tableau.cite(cited)
                ),
            );
            return Ok(self.passed(children));
        }
        let groups: Vec<Vec<Signed>> = children
            .iter()
            .map(|&child| {
                let lines = &tableau.branches[child as usize].lines;
                let count = self.group(lines, cited, &twos);
                lines[..count]
                    .iter()
                    .map(|&id| tableau.lines[id as usize].signed)
                    .collect()
            })
            .collect();
        let fits = |group: &[Signed], results: &[Signed]| {
            !group.is_empty()
                && within(group, results)
                && results.iter().all(|&r| self.holds(r) || group.contains(&r))
        };
        let found = twos.iter().find_map(|shape| {
            let r = &shape.results;
            if fits(&groups[0], &r[0]) && fits(&groups[1], &r[1]) {
                Some((*shape, [0, 1]))
            } else if fits(&groups[0], &r[1]) && fits(&groups[1], &r[0]) {
                Some((*shape, [1, 0]))
            } else {
                None
            }
        });
        let Some((shape, order)) = found else {
            // The branch at fault: the second where the first starts as
            // one of the results.
            let shape = twos[0];
            let r = &shape.results;
            let k = usize::from(fits(&groups[0], &r[0]) || fits(&groups[0], &r[1]));
            let lines = &tableau.branches[children[k] as usize].lines;
            let count = groups[k].len().clamp(1, lines.len());
            let written: Vec<Signed> = lines[..count]
                .iter()
                .map(|&id| tableau.lines[id as usize].signed)
                .collect();
            let at = lines[0];
            self.fault(
                at,
                format!(
                    "line {}: {} on {} splits the branch into one that starts with {} and one \
                     with {}; this branch starts with {}",
                    tableau.lines[at as usize].number,
                    shape.shown,
                    tableau.cite(cited),
                    tableau.list(&r[0]),
                    tableau.list(&r[1]),
                    tableau.list(&written)
                ),
            );
            return Ok(self.passed(children));
        };
        let at = self.plan(slot, Some(shape.name), vec![cited], 2)?;
        Ok(vec![
            (groups[0].len(), Some((at, order[0]))),
            (groups[1].len(), Some((at, order[1]))),
        ])
    }

    /// Returns, for a split at fault, each branch's first lines that cite
    /// what its first line cites, to be passed over: the fault is the
    /// split's, and read again as expansions of their own they would be
    /// said again, at an earlier line than another branch's.
    fn passed(&self, children: &[u32]) -> Vec<Passed> {
        let tableau = self.tableau;
        children
            .iter()
            .map(|&child| {
                let lines = &tableau.branches[child as usize].lines;
                let cites = tableau.lines[lines[0] as usize].cites;
                let count = lines
                    .iter()
                    .take_while(|&&id| tableau.lines[id as usize].cites == cites)
                    .count();
                (count, None)
            })
            .collect()
    }

    /// Returns a pair of lines on the current branch that closes it: a
    /// formula true and false, or the one line of `⊥` true or `⊤` false.
    fn closer(&self) -> Option<Vec<u32>> {
        let tableau = self.tableau;
        let mut first: HashMap<Signed, u32> = HashMap::default();
        for &id in &self.path {
            first.entry(tableau.lines[id as usize].signed).or_insert(id);
        }
        for &id in &self.path {
            let signed = tableau.lines[id as usize].signed;
            match (signed.sign, tableau.formulas.node(signed.formula)) {
                (Sign::True, Node::False) | (Sign::False, Node::True) => return Some(vec![id]),
                (Sign::True, _) => {
                    if let Some(&other) = first.get(&Signed::new(Sign::False, signed.formula)) {
                        return Some(vec![id, other]);
                    }
                }
                (Sign::False, _) => {}
            }
        }
        None
    }

    /// Returns a line on the current branch that is not expanded on it:
    /// none of the results of one of its rules (whole) is on the branch.
    fn unexpanded(&self) -> Option<u32> {
        let tableau = self.tableau;
        self.path.iter().copied().find(|&id| {
            let shapes = shapes(&tableau.formulas, tableau.lines[id as usize].signed);
            !shapes.is_empty()
                && !shapes.iter().any(|shape| {
                    !shape.partial
                        && shape
                            .results
                            .iter()
                            .any(|r| r.iter().all(|&x| self.holds(x)))
                })
        })
    }

    /// Checks the end of a branch and plans its node.
    fn leaf(&mut self, index: usize, slot: Option<(usize, usize)>) -> Result<(), Error> {
        let tableau = self.tableau;
        let branch = &tableau.branches[index];
        let Some(&last) = branch.lines.last() else {
            unreachable!("every branch has a line")
        };
        let last_number = tableau.lines[last as usize].number;
        let row = branch
            .mark
            .as_ref()
            .map_or(tableau.lines[last as usize].row, |&(_, row)| row);
        let mut leaf = Leaf {
            last,
            row,
            closed: false,
            unfinished: None,
            branch: Vec::new(),
        };
        let at_mark = |message: String| (row, Some(last_number), message);
        match &branch.mark {
            Some((Mark::Closed(cites), _)) => {
                let lines = if cites.is_empty() {
                    self.closer()
                } else {
                    self.closing(cites, last_number, row)
                };
                match lines {
                    Some(lines) => {
                        leaf.closed = true;
                        self.close(slot, &lines)?;
                    }
                    None if cites.is_empty() => {
                        let (row, line, message) = at_mark(format!(
                            "the branch ending at line {last_number} is marked ×, and nothing \
                             closes it: no formula on it is both true and false, and neither \
                             ⊥ is true nor ⊤ false"
                        ));
                        self.fault_at(row, line, message);
                    }
                    None => {}
                }
            }
            Some((Mark::Open, _)) => {
                if let Some(lines) = self.closer() {
                    let (row, line, message) = at_mark(format!(
                        "the branch ending at line {last_number} is marked open, and it closes \
                         on {}",
                        self.pair(&lines)
                    ));
                    self.fault_at(row, line, message);
                } else if let Some(id) = self.unexpanded() {
                    let (row, line, message) = at_mark(format!(
                        "the branch ending at line {last_number} is marked open, and {} is not \
                         expanded on it",
                        tableau.cite(id)
                    ));
                    self.fault_at(row, line, message);
                } else {
                    leaf.branch.clone_from(&self.path);
                    self.plan(slot, None, Vec::new(), 0)?;
                }
            }
            None => {
                leaf.unfinished = Some(if let Some(lines) = self.closer() {
                    format!(
                        "is not marked: it closes on {}, so mark it ×",
                        self.pair(&lines)
                    )
                } else if let Some(id) = self.unexpanded() {
                    format!(
                        "is not finished: {} is not expanded on it",
                        tableau.cite(id)
                    )
                } else {
                    "is finished and nothing closes it, so mark it open".to_owned()
                });
                self.plan(slot, None, Vec::new(), 0)?;
            }
        }
        self.leaves.push(leaf);
        Ok(())
    }

    /// Writes the lines that close a branch, `line 3 (T p) and line 5
    /// (F p)` or one line.
    fn pair(&self, lines: &[u32]) -> String {
        let cited: Vec<String> = lines.iter().map(|&id| self.tableau.cite(id)).collect();
        cited.join(" and ")
    }

    /// Checks the lines a mark `×` names; returns them where they close
    /// the branch.
    fn closing(&mut self, cites: &[u32], last: u32, row: u32) -> Option<Vec<u32>> {
        let tableau = self.tableau;
        let mut lines = Vec::with_capacity(cites.len());
        for &number in cites {
            let found = self.by_number.get(&number).copied();
            match found {
                Some(id) if self.on_path[id as usize] => lines.push(id),
                _ => {
                    self.fault_at(
                        row,
                        Some(last),
                        format!(
                            "the × under line {last} names line {number}, which is not on its \
                             branch"
                        ),
                    );
                    return None;
                }
            }
        }
        let signed = |id: u32| tableau.lines[id as usize].signed;
        let closes = match lines.as_slice() {
            [one] => matches!(
                (
                    signed(*one).sign,
                    tableau.formulas.node(signed(*one).formula)
                ),
                (Sign::True, Node::False) | (Sign::False, Node::True)
            ),
            [a, b] => {
                let (a, b) = (signed(*a), signed(*b));
                a.formula == b.formula && a.sign != b.sign
            }
            _ => false,
        };
        if !closes {
            let named = lines.len();
            let what = if named > 2 {
                format!("{named} lines")
            } else {
                self.pair(&lines)
            };
            self.fault_at(
                row,
                Some(last),
                format!(
                    "the × under line {last} names {what}, which do not close the branch: a \
                     branch closes on one formula true and false (two lines), on ⊥ true or on \
                     ⊤ false (one line)"
                ),
            );
            return None;
        }
        // The rule `×` writes the true formula first.
        if let &[a, b] = lines.as_slice()
            && signed(a).sign == Sign::False
        {
            return Some(vec![b, a]);
        }
        Some(lines)
    }

    /// Plans the node that closes the current branch on `lines`.
    fn close(&mut self, slot: Option<(usize, usize)>, lines: &[u32]) -> Result<(), Error> {
        let tableau = self.tableau;
        let rule = match lines {
            [_, _] => "×",
            [one] => {
                if tableau.lines[*one as usize].signed.sign == Sign::True {
                    "T⊥"
                } else {
                    "F⊤"
                }
            }
            _ => unreachable!("a branch closes on one line or two"),
        };
        self.plan(slot, Some(rule), lines.to_vec(), 0)?;
        Ok(())
    }

    /// Returns the derivation in the rule file that the planned nodes
    /// make: a node per expansion, its branch's signed formulas as a set,
    /// premises before conclusions.
    fn derivation(&self) -> Result<Derivation, Error> {
        let tableau = self.tableau;
        let mut out = Derivation::new("tableau", 1);
        let formulas = terms_of(&tableau.formulas, out.terms_mut())?;
        let mut terms: Vec<TermId> = Vec::with_capacity(tableau.lines.len());
        for line in &tableau.lines {
            let sign = line.signed.sign.letter();
            let term = formulas[line.signed.formula.index()];
            terms.push(out.terms_mut().apply(sign, &[term])?);
        }
        let count = self.planned.len();
        let id = |index: usize| u32::try_from(count - 1 - index).map_err(|_| too_many(count));
        for index in (0..count).rev() {
            let planned = &self.planned[index];
            let sequent = [planned
                .branch
                .iter()
                .map(|&line| terms[line as usize])
                .collect::<Vec<TermId>>()];
            let mut positions = Vec::with_capacity(planned.positions.len());
            for &line in &planned.positions {
                let term = terms[line as usize];
                let place = sequent[0].iter().position(|&t| t == term);
                let Some(place) = place else {
                    unreachable!("a principal line is on its node's branch")
                };
                positions.push(u32::try_from(place).map_err(|_| too_many(place))?);
            }
            let premises = planned
                .premises
                .iter()
                .map(|&p| id(p))
                .collect::<Result<Vec<u32>, _>>()?;
            let step = planned.rule.map_or(RuleStep::Open, RuleStep::Rule);
            out.push(
                RuleNode::new(&sequent, step)
                    .premises(&premises)
                    .positions(&positions),
            )?;
        }
        Ok(out)
    }
}

impl std::str::FromStr for WrittenTableau {
    type Err = Error;

    /// Reads a tableau within the default limits.
    fn from_str(s: &str) -> Result<Self, Error> {
        Self::parse_within(s, &Limits::default())
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests;
