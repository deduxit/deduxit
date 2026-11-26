// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Why a step of a proof by hand is refused, as fields: what a witness
//! lacks ([`WitnessFault`]), and why a calculus given as data or a
//! refutation by hand refuses a step ([`StepFault`]).

use std::fmt::{Display, Formatter, Result as FmtResult};

/// Why a witness is not a term of the goal: what is wrong with its text,
/// or with the term given.
///
/// Both proofs by hand that take witnesses use it: the linear session's
/// `StepError::Witness` and the ordinary one's. Its `Display` is the
/// phrase after "the witness is not a term here:".
///
/// # JSON
///
/// With the feature `serialize`, a step error's `details` hold it
/// flattened: `"fault"` names the variant in snake case (`unknown_symbol`,
/// `arity`, …), beside the variant's fields.
///
/// Needs the cargo feature `interactive` (on by default).
#[cfg(feature = "interactive")]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize),
    serde(tag = "fault", rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum WitnessFault {
    /// A bracket or a comma stands where a term should.
    #[non_exhaustive]
    Missing {
        /// The token, `(`, `)` or `,`.
        before: String,
    },
    /// A bracket follows no function symbol.
    Bracket,
    /// A comma or a closing bracket stands outside every application.
    #[non_exhaustive]
    Unopened {
        /// The token, `,` or `)`.
        token: String,
    },
    /// A term follows a term without a comma between them.
    #[non_exhaustive]
    Comma {
        /// The token after the first term.
        token: String,
    },
    /// A bracket is not closed.
    Unclosed,
    /// The text or the list holds no term.
    Empty,
    /// A name is no function symbol of the sequent and no eigenvariable.
    #[non_exhaustive]
    UnknownSymbol {
        /// The name.
        name: String,
    },
    /// A function symbol is named by a number the sequent does not have.
    #[non_exhaustive]
    SymbolIndex {
        /// The number.
        symbol: u32,
    },
    /// A name is a predicate of the sequent, not a function symbol.
    #[non_exhaustive]
    Predicate {
        /// The name.
        name: String,
    },
    /// A function symbol is given another number of arguments than it
    /// takes.
    #[non_exhaustive]
    Arity {
        /// The symbol.
        name: String,
        /// How many arguments it takes.
        arity: u32,
        /// How many it was given, or `None` for a symbol written without
        /// brackets.
        given: Option<usize>,
    },
    /// An eigenvariable is written with arguments.
    #[non_exhaustive]
    Applied {
        /// The eigenvariable, as the proof writes it.
        name: String,
    },
    /// The term holds an eigenvariable outside the goal's scope: no `∀R`
    /// or `∃L` below the goal introduces it.
    #[non_exhaustive]
    OutOfScope {
        /// The eigenvariable, as the proof writes it.
        name: String,
    },
    /// A node of a term given flat names a node that does not come before
    /// it.
    #[non_exhaustive]
    NodeOrder {
        /// The node, counted from 0.
        node: usize,
        /// The node it names.
        names: u32,
    },
    /// The text has more symbols than the proof's bound on terms.
    #[non_exhaustive]
    Symbols {
        /// The symbols the text names.
        symbols: u64,
        /// The bound, `Limits::terms`.
        limit: u64,
    },
    /// The session's table of terms would pass its bound on terms.
    #[non_exhaustive]
    Table {
        /// The bound, `Limits::terms`.
        limit: u64,
    },
    /// The proof has made as many eigenvariables as it can number, so none
    /// is fresh.
    Eigenvariables,
    /// The proof has as many open witnesses as it can number.
    Witnesses,
    /// The term given is no term of the proof.
    NotATerm,
    /// The term given holds a bound variable or an open witness.
    Variable,
    /// Building the instance passed a bound of the proof's formulas.
    #[non_exhaustive]
    Refused {
        /// The bound.
        refusal: crate::limits::Refusal,
    },
}

#[cfg(feature = "interactive")]
impl Display for WitnessFault {
    /// Writes what is wrong, as a phrase.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Missing { before } => write!(f, "a term is missing before {before:?}"),
            Self::Bracket => f.write_str("a bracket follows no function symbol"),
            Self::Unopened { token } => write!(f, "{token:?} closes no application"),
            Self::Comma { token } => write!(f, "{token:?} follows a term without a comma"),
            Self::Unclosed => f.write_str("a bracket is not closed"),
            Self::Empty => f.write_str("there is no term"),
            Self::UnknownSymbol { name } => write!(f, "{name} is not a symbol of the sequent"),
            Self::SymbolIndex { symbol } => {
                write!(f, "symbol {symbol} is not one of the sequent's")
            }
            Self::Predicate { name } => write!(f, "{name} is a predicate, not a term"),
            Self::Arity { name, arity, given } => {
                write!(f, "{name} takes {arity} arguments")?;
                match given {
                    Some(given) => write!(f, ", not {given}"),
                    None => Ok(()),
                }
            }
            Self::Applied { name } => write!(f, "the eigenvariable {name} takes no arguments"),
            Self::OutOfScope { name } => write!(
                f,
                "the eigenvariable {name} is not in the goal's scope: no ∀R or ∃L below the \
                 goal introduces it"
            ),
            Self::NodeOrder { node, names } => {
                write!(f, "node {node} names node {names}, which is not before it")
            }
            Self::Symbols { symbols, limit } => write!(
                f,
                "it has {symbols} symbols, more than the bound of {limit} on terms"
            ),
            Self::Table { limit } => write!(
                f,
                "the session would hold or unfold to more than {limit} terms"
            ),
            Self::Eigenvariables => {
                f.write_str("the proof has made as many eigenvariables as it can number")
            }
            Self::Witnesses => f.write_str("the proof has as many open witnesses as it can number"),
            Self::NotATerm => f.write_str("it is no term of the proof"),
            Self::Variable => f.write_str("it holds a variable"),
            Self::Refused { refusal } => write!(f, "{refusal}"),
        }
    }
}

/// Why a calculus given as data refuses a step of a proof by hand, or a
/// refutation by hand refuses a resolvent: the reason of
/// `Error::StepRefused`, as fields.
///
/// Its `Display` is the reason after "the step is refused:".
///
/// # JSON
///
/// With the feature `serialize`, the error's `details` hold it flattened:
/// `"fault"` names the variant in snake case (`no_goal`, `unmatched`, …),
/// beside the variant's fields.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize),
    serde(tag = "fault", rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum StepFault {
    /// No open goal has this number.
    #[non_exhaustive]
    NoGoal {
        /// The goal asked for.
        goal: u32,
    },
    /// No line has this number.
    #[non_exhaustive]
    NoLine {
        /// The line asked for.
        line: u32,
    },
    /// The line lies in a subproof that is closed at the goal.
    #[non_exhaustive]
    NotInForce {
        /// The line.
        line: u32,
        /// The goal.
        goal: u32,
    },
    /// A zone would hold more or fewer members than the calculus allows.
    #[non_exhaustive]
    ZoneSize {
        /// The zone's name.
        zone: String,
        /// How many members it would hold.
        members: u64,
    },
    /// A zone that is a set would hold a member twice.
    #[non_exhaustive]
    Twice {
        /// The zone's name.
        zone: String,
    },
    /// The calculus's checker refuses the step, for its reason.
    #[non_exhaustive]
    Check {
        /// The checker's reason, as it writes it.
        reason: String,
    },
    /// The calculus has no rule of this name.
    #[non_exhaustive]
    NoRule {
        /// The name.
        rule: String,
    },
    /// A forward step names a rule that discharges hypotheses, which only
    /// a backward step takes.
    #[non_exhaustive]
    Discharges {
        /// The rule.
        rule: String,
    },
    /// A forward step cites another number of lines than the rule has
    /// premises.
    #[non_exhaustive]
    Cited {
        /// The rule.
        rule: String,
        /// Its premises.
        premises: usize,
        /// The lines cited.
        lines: usize,
    },
    /// A forward step on the goal's members takes a rule that reads
    /// backwards only.
    #[non_exhaustive]
    Backwards {
        /// The rule.
        rule: String,
        /// Why it reads backwards only.
        why: Backwards,
    },
    /// A forward step derives nothing the goal does not hold already.
    #[non_exhaustive]
    NothingNew {
        /// The rule.
        rule: String,
    },
    /// A forward step from lines takes a rule with contexts, which lines
    /// cannot tell apart.
    #[non_exhaustive]
    Contexts {
        /// The rule.
        rule: String,
    },
    /// A line cited is no instance of the premise it stands for.
    #[non_exhaustive]
    NoInstance {
        /// The line.
        line: u32,
        /// The premise, counted from 1.
        premise: usize,
        /// The rule.
        rule: String,
    },
    /// No line in force is equal to the goal.
    NoLineEqual,
    /// The line is not equal to the goal.
    #[non_exhaustive]
    NotGoal {
        /// The line.
        line: u32,
        /// The goal.
        goal: u32,
    },
    /// The step names fewer members than the rule acts on.
    TooFew,
    /// The step names more members than the rule acts on.
    TooMany,
    /// A member the step names is not in the zone where the rule acts.
    #[non_exhaustive]
    Zone {
        /// The member, by its number across the goal's zones.
        member: u32,
        /// The zone's name.
        zone: String,
    },
    /// The goal has no member where the rule writes one.
    NoPlace,
    /// Two members the rule writes out stand at one place.
    SamePlace,
    /// A member of the goal is no instance of what the rule writes out.
    #[non_exhaustive]
    Unmatched {
        /// The member, as the calculus writes it, cut after 200
        /// characters.
        member: String,
    },
    /// The members the rule writes out stand in another order.
    Order,
    /// A context given is not what the goal holds.
    Context,
    /// The goal holds members the rule does not account for.
    Unaccounted,
    /// A member of a context is not under its connective.
    NotUnder,
    /// A variable given has another value already.
    #[non_exhaustive]
    Bound {
        /// The variable.
        name: String,
        /// Its value, as the calculus writes it, cut after 200 characters.
        value: String,
    },
    /// A context given is one no rule uses.
    #[non_exhaustive]
    Unused {
        /// The context.
        name: String,
    },
    /// A name given is no variable or context of the calculus.
    #[non_exhaustive]
    Unknown {
        /// The name.
        name: String,
    },
    /// A variable the rule asks to be an atom is given something else.
    #[non_exhaustive]
    NotAtom {
        /// The variable.
        name: String,
    },
    /// A refutation has no clause of this number yet.
    #[non_exhaustive]
    NoClause {
        /// How many clauses it has.
        clauses: usize,
    },
    /// The clauses have no atom of this number.
    #[non_exhaustive]
    NoAtom {
        /// How many atoms they have.
        atoms: usize,
    },
    /// Of the two clauses, neither holds the atom while the other holds
    /// its negation.
    #[non_exhaustive]
    NoClash {
        /// The first clause.
        first: usize,
        /// The second clause.
        second: usize,
        /// The atom, by its name.
        atom: String,
    },
}

/// Why a rule reads backwards only, in a forward step on the goal's
/// members ([`StepFault::Backwards`]).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize),
    serde(tag = "kind", rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum Backwards {
    /// It has no premise, so it closes a goal.
    Closes,
    /// It discharges hypotheses.
    Discharges,
    /// It has more than one premise, and a forward step keeps one goal.
    #[non_exhaustive]
    Premises {
        /// How many.
        premises: usize,
    },
    /// It acts on members of another zone than the first.
    #[non_exhaustive]
    ActsOn {
        /// Where it acts, as a phrase: `after ⊢`.
        zone: String,
        /// Where a forward step acts: `before ⊢`.
        first: String,
    },
    /// It drops a member from the goal.
    #[non_exhaustive]
    Drops {
        /// The member, as the calculus writes it.
        member: String,
    },
    /// It adds members to another zone than the first.
    #[non_exhaustive]
    Adds {
        /// Where it adds, as a phrase.
        zone: String,
        /// Where a forward step adds: `before ⊢`.
        first: String,
    },
}

impl Display for Backwards {
    /// Writes why, as a phrase after the rule's name.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Closes => f.write_str("has no premise, so it closes a goal"),
            Self::Discharges => f.write_str("discharges hypotheses"),
            Self::Premises { premises } => write!(
                f,
                "has {premises} premises, and a forward step keeps one goal"
            ),
            Self::ActsOn { zone, first } => write!(
                f,
                "acts on members {zone}, and a forward step acts on members {first} alone"
            ),
            Self::Drops { member } => write!(f, "drops {member} from the goal"),
            Self::Adds { zone, first } => write!(
                f,
                "adds members {zone}, and a forward step adds them only {first}"
            ),
        }
    }
}

impl Display for StepFault {
    /// Writes the reason, as a phrase.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NoGoal { goal } => write!(f, "there is no open goal {goal}"),
            Self::NoLine { line } => write!(f, "there is no line {line}"),
            Self::NotInForce { line, goal } => write!(
                f,
                "line {line} is not in force at goal {goal}: it lies in a subproof that is \
                 closed there"
            ),
            Self::ZoneSize { zone, members } => write!(
                f,
                "the zone {zone:?} would hold {members} members, outside what the calculus allows"
            ),
            Self::Twice { zone } => {
                write!(
                    f,
                    "the zone {zone:?} is a set and would hold a member twice"
                )
            }
            Self::Check { reason } => f.write_str(reason),
            Self::NoRule { rule } => write!(f, "the calculus has no rule {rule:?}"),
            Self::Discharges { rule } => write!(
                f,
                "{rule} discharges hypotheses: apply it backwards to a goal, which opens its \
                 subproof"
            ),
            Self::Cited {
                rule,
                premises,
                lines,
            } => write!(
                f,
                "{rule} has {premises} premises, and the step cites {lines} lines"
            ),
            Self::Backwards { rule, why } => {
                write!(f, "{rule} {why}: apply it backwards to the goal")
            }
            Self::NothingNew { rule } => {
                write!(f, "{rule} derives nothing the goal does not hold already")
            }
            Self::Contexts { rule } => write!(
                f,
                "{rule} has contexts, which lines cannot tell apart: apply it backwards, or \
                 forwards on the goal's members"
            ),
            Self::NoInstance {
                line,
                premise,
                rule,
            } => write!(
                f,
                "line {line} is no instance of premise {premise} of {rule}"
            ),
            Self::NoLineEqual => f.write_str("the goal is no line in force"),
            Self::NotGoal { line, goal } => write!(f, "line {line} is not goal {goal}"),
            Self::TooFew => f.write_str("the step names fewer members than the rule acts on"),
            Self::TooMany => f.write_str("the step names more members than the rule acts on"),
            Self::Zone { member, zone } => write!(
                f,
                "member {member} is not in the zone {zone:?}, where the rule acts"
            ),
            Self::NoPlace => f.write_str("the goal has no member where the rule writes one"),
            Self::SamePlace => f.write_str("two members the rule writes out stand at one place"),
            Self::Unmatched { member } => {
                write!(f, "{member} is no instance of what the rule writes out")
            }
            Self::Order => f.write_str("the members the rule writes out stand in another order"),
            Self::Context => f.write_str("a given context is not what the goal holds"),
            Self::Unaccounted => {
                f.write_str("the goal holds members the rule does not account for")
            }
            Self::NotUnder => f.write_str("a member of a context is not under its connective"),
            Self::Bound { name, value } => write!(f, "{name} is {value} already"),
            Self::Unused { name } => write!(f, "no rule uses the context {name}"),
            Self::Unknown { name } => {
                write!(f, "{name:?} is no variable or context of the calculus")
            }
            Self::NotAtom { name } => write!(f, "{name} must be an atom"),
            Self::NoClause { clauses } => write!(
                f,
                "there are clauses 0 to {} only",
                clauses.saturating_sub(1)
            ),
            Self::NoAtom { atoms } => {
                write!(f, "there are atoms 0 to {} only", atoms.saturating_sub(1))
            }
            Self::NoClash {
                first,
                second,
                atom,
            } => write!(
                f,
                "of clauses {first} and {second}, one must hold {atom} and the other ¬{atom}"
            ),
        }
    }
}
