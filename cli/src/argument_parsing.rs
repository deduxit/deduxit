// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use clap::builder::TypedValueParser;
use clap::{Args, Parser, Subcommand, ValueEnum};
use deduxit::export::rocq::Kernel;
use deduxit::export::svg::MathFont;
use deduxit::linear::proofs::{Compact, cut};
use deduxit::linear::search::{Cadence, Engine, Options, Schedule};
use deduxit::{
    Clock, Limits,
    linear::{Bias, Fragment, Mode},
};
use std::path::PathBuf;
use std::time::Duration;

/// The syntax of linear logic, for the help texts below.
macro_rules! linear_syntax {
    () => {
        "\
Sequent syntax: `A, B |- C, D` (or `⊢`), either side may be empty.
  tensor  A * B, A ⊗ B        par   A | B, A par B, A ⅋ B
  with    A & B               plus  A + B, A ⊕ B
  linear implication  A -o B, A ⊸ B
  negation  ~A, A^            exponentials  !A, ?A
  units  1, bot, ⊥, top, ⊤, 0
  quantifiers  forall x. A, ∀x. A, exists x. A, ∃x. A, forall x y. A
  predicates  p(x, f(a)): a name bound by a quantifier is a variable, any
    other name a constant, or a function symbol before a bracket
Binding, tightest first: A^, then ~ ! ?, then *, |, &, +, then -o, which
groups to the right, and last a quantifier, which reaches right as far as
it can."
    };
}

/// The heading of the flags that say where the input comes from and how
/// it is read.
const INPUT: &str = "Input";
/// The heading of the flags that pick the logic.
const LOGIC: &str = "Logic";
/// The heading of the flags of the search.
const SEARCH: &str = "Search";
/// The heading of the limits on time, memory and work.
const LIMITS: &str = "Limits";
/// The heading of the flags of the output.
const OUTPUT: &str = "Output";
/// The heading of the flags of a batch.
const BATCH: &str = "Batch";

/// How to write a sequent of linear or of ordinary logic, for the help
/// texts below.
macro_rules! ordinary_syntax {
    () => {
        concat!(
            linear_syntax!(),
            "

With --logic the input is ordinary logic: `a, a -> b |- b`, or a formula alone.
  and  a /\\ b, a ∧ b         or   a \\/ b, a ∨ b
  implies  a -> b, a → b      iff  a <-> b, a ↔ b
  not  ~a, ¬a                 constants  true, ⊤, false, ⊥
Binding, tightest first: ~, /\\, \\/, ->, and last <->; -> and <-> group to
the right."
        )
    };
}

/// The line that ends `-h` where it leaves out options that `--help`
/// shows.
macro_rules! every_option {
    () => {
        "--help shows every option."
    };
}

/// How to write a sequent, shown under the help of every command that reads
/// one.
const SYNTAX: &str = linear_syntax!();

/// How to write a sequent, and that `--help` shows every option, shown
/// under the short help of `interact`.
const SYNTAX_SHORT: &str = concat!(linear_syntax!(), "\n\n", every_option!());

/// How to write a sequent of linear or of ordinary logic, shown under the
/// help of the commands that read ordinary logic with `--logic`.
const ORDINARY: &str = ordinary_syntax!();

/// How to write a sequent of linear or of ordinary logic, and that
/// `--help` shows every option, shown under the short help of `prove`.
const ORDINARY_SHORT: &str = concat!(ordinary_syntax!(), "\n\n", every_option!());

/// That `--help` shows every option, shown under the short help of
/// `check` and `eliminate`.
const EVERY_OPTION: &str = every_option!();

/// Decide, check, prove step by step and export sequents of linear logic,
/// and ordinary logic through its translations
#[derive(Parser, Debug)]
#[command(version, long_about = None, after_help = ORDINARY)]
pub(crate) struct Cli {
    /// What to do.
    #[command(subcommand)]
    pub(crate) command: Command,
}

/// The commands.
#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Decide whether a sequent is provable, and print a proof if it is
    ///
    /// The command finds the sequent's fragment (MLL, MALL, …) and picks
    /// the engine for it; the first line of the output names both. The exit
    /// status is 0 for provable, 1 for unprovable, 3 for unknown (the
    /// search stopped before it decided) and 2 for an error.
    ///
    /// To decide many sequents in one call (a batch), give `--file` several
    /// times, a directory, `--files-from`, or an input format of many
    /// sequents: `lines`, `jsonl` or `problems`. On standard input such a
    /// format is a stream, answered line by line. Paths are taken
    /// literally, with no glob or `~`. A directory is walked in sorted
    /// order, following links, for the files with the input format's
    /// extension (`.p` and `.json` for `auto`).
    ///
    /// Each sequent is named by its path as given, by its line's name, or
    /// by `FILE:LINE`. It gets one line, `NAME: VERDICT`, in input order,
    /// as soon as it is decided. With `--format json` the line is a JSON
    /// Lines record of `version`, `name` and the outcome's keys. An entry
    /// that could not be decided has the verdict `error` and a `message`,
    /// and the library's `error` where the library caused it. A drawn
    /// format writes a file per proved sequent into `--output DIR`. The
    /// mode is the flags', or a problem file's column or a record's.
    ///
    /// A batch's exit status is the worst of its entries: an error (2)
    /// before unknown (3) before unprovable (1) before proved (0). A batch
    /// with no sequent is an error. Ctrl-C ends a batch's input, and its
    /// status is then unknown at best.
    #[command(after_help = ORDINARY_SHORT, after_long_help = ORDINARY)]
    Prove(ProveArgs),
    /// Check a proof read from JSON, as `prove --format json` writes it, or
    /// the refutation of an unprovable outcome
    ///
    /// An outcome of `prove --logic`, or a derivation of LK or LJ, is
    /// checked by the rules of its calculus, with the image's proof or
    /// refutation where the embedding found one. One that holds nothing to
    /// check (a clausal proof's counts, a "not valid" without a refutation,
    /// an unknown) gets 3.
    ///
    /// The mode is the one the file states, or the flags' where it states
    /// none. A mode flag that names another mode is an error. The exit
    /// status is 0 for a valid proof, 1 for an invalid one, 3 for unknown
    /// (a limit stopped the check, so the proof is neither valid nor
    /// invalid) and 2 for an error.
    ///
    /// A refutation is checked without a search, in the mode its file
    /// states; it says nothing about other modes. The exit status is 0
    /// where it holds and 1 where it does not. An outcome refuted only by
    /// an exhausted search has nothing to check, and gets 3.
    ///
    /// With `--format json` the verdict is `{"version", "verdict",
    /// "mode"}`, or `"logic"` in place of `"mode"` for ordinary logic, the
    /// verdict being `valid`, `invalid` or `unknown`. The
    /// last two add the library's `error` object and its `message`, with
    /// formulas.
    #[command(after_help = EVERY_OPTION, after_long_help = "")]
    Check(CheckArgs),
    /// Grade an answer to a problem: whether it proves or refutes the
    /// sequent given, checked as `check` and `interact` check
    ///
    /// The problem is the sequent, in linear logic by default, or in the
    /// logic --logic, --system or --calculus names. The answer is read from
    /// --answer or standard input, and its kind from its shape:
    ///
    /// - a JSON document: a proof or an outcome as `prove --format json`
    ///   or a session's `proof FILE` writes it, a Kripke model that refutes
    ///   the sequent (`prove`'s `countermodel`), or a derivation in a
    ///   calculus given as data;
    /// - a signed tableau of classical logic as `check --tableau` reads it,
    ///   its first line starting with a line number;
    /// - otherwise, a script of `interact`'s commands, one per line, which
    ///   must end with the proof finished. `close`, `invertible`, `save` and
    ///   `load` are
    ///   refused, since the steps must be the answer's own, and the
    ///   commands that only show the session are passed over, as are blank
    ///   lines, `#` comments and a Markdown code block's fences.
    ///
    /// A proof or a refutation must be of the problem's sequent, in its
    /// logic. The verdict is `correct`, `incorrect` (with the first step
    /// that failed, for a script) or `unknown`. The exit status is 0 for
    /// correct, 1 for incorrect (an answer that does not read is
    /// incorrect), 3 for unknown (a limit stopped the check, or the answer
    /// holds nothing to check, such as a refutation only the search vouches
    /// for) and 2 for an
    /// error of the problem or the flags. With `--format json` the verdict
    /// is `{"version", "verdict", "answer", "message"}`, `answer` the kind
    /// read, and `line` the script's line that failed first.
    #[command(after_help = EVERY_OPTION, after_long_help = "")]
    Grade(GradeArgs),
    /// Eliminate the cuts of a proof read from JSON, step by step
    ///
    /// The proof is first checked in the mode its file states, or the
    /// flags' where it states none, as `check` does. Then its cuts are
    /// reduced one step at a time until no cut is left or a bound is
    /// reached. Each step takes the first cut in the proof, one with no cut
    /// above it. `--steps=1` takes a single step.
    ///
    /// Each step is printed with its case and the cut formula, then the
    /// proof reached, drawn as `check` draws it. The cases are the axiom
    /// case, a key case such as ⊗ against ⅋, a commutative case,
    /// weakening, dereliction, contraction, and a box entering a box. Every
    /// proof reached is checked.
    ///
    /// The exit status is 0 when the proof ends cut-free or after the steps
    /// `--steps` asked for, 1 for a proof that is not valid, and 2 for an
    /// error. It is 3 when the default bound on steps, another bound or the
    /// time limit stopped the elimination with cuts left; the proof reached
    /// is still printed. With `--format json` the output is the record
    /// `{"version", "proof", "steps", "end"}`: the proof reached; each
    /// step's `cut` (the node it was taken on), `formula` (the occurrence
    /// cut) and `case`; and `end`, which is `cut_free`, or `refused` with
    /// the `reason`.
    ///
    /// With `--net`, for the sequents `--net` takes, the cuts are
    /// eliminated on the proof's net instead. Each step reduces the first
    /// cut link: the axiom case, the key case of ⊗ against ⅋, and on boxes
    /// weakening, dereliction, contraction and a box entering a box. The
    /// net reached is printed as text. `--format svg`, `png` or `pdf` with
    /// `--output FILE` draws every net into a file of its own: FILE-k
    /// (net-1.svg for net.svg) is the net step k was taken on, with its cut
    /// link and what it acts on highlighted, and FILE is the net reached.
    /// Each file is written as soon as it is drawn. Each drawing is held
    /// to --derivation-limit, and the steps' files together to
    /// --drawings-limit. `--format json` writes the record `{"version",
    /// "net", "steps", "end"}`, each step's `cut` being the two vertices of
    /// its cut link.
    #[command(after_help = EVERY_OPTION, after_long_help = "")]
    Eliminate(EliminateArgs),
    /// Prove a sequent step by step, reading commands from standard input
    ///
    /// The session starts with the whole sequent as its one open goal.
    /// Give the sequent as an argument or with `--file`: standard input
    /// carries the commands. Write one command per line. Its words are
    /// split as a shell splits them, so quotes keep a name with blanks one
    /// word.
    ///
    /// `goals` lists the open goals, with the position of each formula.
    /// `rules G P` lists the rules that act on formula P of goal G. `apply
    /// G P RULE [P…]` applies a rule; for a ⊗ or Mix, the further positions
    /// are the formulas that go to the left premise. `cut G C [P…]` cuts
    /// goal G on the formula C, written as the mode writes formulas (quoted
    /// if it has blanks). The formulas at the positions P go to the first
    /// premise, which proves C, and the others to the second, which uses
    /// it. `undo` takes back the last step. `invertible on|off` turns on or
    /// off applying every invertible rule by itself. `close [G]` lets the
    /// search close goal G, or every open goal. `show [--FORMAT] [FILE]`
    /// draws the derivation so far, open goals included, as text, or
    /// --latex, --typst, --svg, --png or --pdf. `proof [--FORMAT] [FILE]`
    /// checks the finished proof and prints or writes it, as a certificate
    /// with --rocq. Given a FILE and no format, both write the format the
    /// file's extension names; where it names none, `show` writes text and
    /// `proof` writes JSON for `check`. `lambda` prints the finished proof's
    /// lambda-term. `save FILE` keeps the session as JSON, and `load FILE`
    /// resumes a session of the same sequent and mode. `help` lists the
    /// commands, and `quit` ends the session.
    ///
    /// A RULE is a rule's usual name or its ASCII spelling: ax, 1, bot,
    /// top, *, par, &, +1, +2, ?d, ?c, ?w, !, mix, and wk (affine
    /// weakening). In intuitionistic mode the rules are two-sided: -oL,
    /// -oR, *L, *R, &L1, &L2, &R, +L, +R1, +R2, 1L, 1R, 0L, topR, !L, !R,
    /// !c, !w. `rules` prints the names it applies.
    ///
    /// The exit status is 0 when the session ends with a finished proof
    /// that checks, 1 otherwise, and 2 when the session cannot start or
    /// read its input. A command that fails prints `error: …` on standard
    /// output, and the session goes on.
    #[command(after_help = SYNTAX_SHORT, after_long_help = SYNTAX)]
    Interact(InteractArgs),
    /// Find a Craig interpolant of a valid sequent of ordinary logic, and
    /// check it
    ///
    /// The sequent is split into two halves: by default every formula left
    /// of ⊢ is in the first half and every formula right of it in the
    /// second, and --first names the first half's formulas instead. An
    /// interpolant I of `Γ₁, Γ₂ ⊢ Δ₁, Δ₂` is a formula with `Γ₁ ⊢ Δ₁, I` and
    /// `I, Γ₂ ⊢ Δ₂` both valid, whose atoms are in both halves. For `A ⊢ B`
    /// that is `A ⊢ I` and `I ⊢ B`.
    ///
    /// The sequent is decided through its translation into linear logic,
    /// and the interpolant read off its derivation by Maehara's method. It
    /// is checked before it is printed: both halves are decided again, and
    /// its atoms compared. Each half's line says how it was decided.
    ///
    /// The exit status is 0 for an interpolant, 1 for a sequent that is not
    /// valid (it has none, and the line after the verdict gives the
    /// countermodel where there is one), 3 for unknown (the time limit or
    /// another limit stopped the search, or a half's check) and 2 for an
    /// error. With `--format json` the answer is the record `{"version",
    /// "deduxit", "logic", "partition", "outcome", "interpolant"}`: the sequent's
    /// outcome, and the interpolant's text and both halves, each with its
    /// outcome.
    #[command(after_help = ORDINARY_SHORT, after_long_help = ORDINARY)]
    Interpolate(InterpolateArgs),
    /// Refute a set of clauses by resolution or DPLL, and check the
    /// refutation
    ///
    /// The input is a classical sequent, whose negation is turned into
    /// clauses (--clauses), or a file of clauses in the DIMACS CNF format
    /// (--dimacs). A sequent is valid exactly when the clauses of its
    /// negation are unsatisfiable. A clause is a disjunction of atoms and
    /// negated atoms; resolution derives from a clause with an atom p and
    /// one with ¬p the rest of both, and a refutation derives the empty
    /// clause □.
    ///
    /// --method saturation (the default) resolves the shortest clause not
    /// yet used with every clause used, leaving out tautologies and
    /// clauses another one contains, until □ appears or nothing new does.
    /// --method dpll assigns the literals of unit clauses, then pure
    /// literals, then decides an atom and backtracks at a conflict, and
    /// prints every step; the refutation is read off its steps.
    ///
    /// The command prints the clauses, numbered from 1, then the
    /// refutation, each resolvent with the clauses and the atom it comes
    /// from, or a model of the clauses. Every refutation is checked step
    /// by step before it is printed, and every model against each clause.
    /// The exit status is 0 for unsatisfiable clauses (a valid sequent),
    /// 1 for satisfiable ones, 3 where --clause-limit or --step-limit or
    /// the time limit ended the search, and 2 for an error. With `--format
    /// json` the answer is the record `{"version", "deduxit", "method",
    /// "verdict", "atoms", "clauses", "resolvents", "true_atoms", "bound",
    /// "trace", "statistics"}`, clauses numbered from 0 and literals as
    /// DIMACS writes them.
    #[command(after_help = ORDINARY)]
    Resolve(ResolveArgs),
    /// Print a sequent, convert it to JSON, name its fragment, translate it
    /// from classical logic, or rewrite it into a normal form
    Seq {
        /// What to do with the sequent.
        #[command(subcommand)]
        command: SeqCommand,
    },
    /// Check a lambda-term's type, or find and count the terms of a type
    ///
    /// A term of the simply typed lambda calculus is a program, and its
    /// type is a formula of intuitionistic logic: a term of type A is a
    /// proof of A. `term check` reads a term and prints its type, `term
    /// inhabit` finds a term of a type or shows that it has none, and `term
    /// enumerate` lists a type's terms and counts them.
    Term {
        /// What to do.
        #[command(subcommand)]
        command: TermCommand,
    },
    /// Keep sequents and what is made from them in a workspace file
    ///
    /// A workspace holds nodes: a sequent you give, or a value made from
    /// other nodes by an operation (a translation, a decision, a proof
    /// started). Change a sequent, and what was made from it is out of
    /// date until `run` makes it again; `undo` and `redo` take a whole
    /// change back and bring it back. `operations` lists what each kind of
    /// value takes. Each command reads the file, changes it and writes it
    /// back, and lists the nodes it touched: the number, the kind, the
    /// value in a line, and how it was made.
    Workspace {
        /// What to do with the workspace.
        #[command(subcommand)]
        command: WorkspaceCommand,
    },
}

/// What `workspace` does.
#[derive(Subcommand, Clone, Debug)]
pub(crate) enum WorkspaceCommand {
    /// Make a workspace file, with one sequent if one is given
    New {
        /// The workspace file to make; an existing file is not replaced
        file: PathBuf,
        /// The sequent
        sequent: Option<String>,
        /// The sequent's calculus.
        #[command(flatten)]
        statement: StatementArgs,
    },
    /// Add a sequent
    Add {
        /// The workspace file
        file: PathBuf,
        /// The sequent
        sequent: String,
        /// The sequent's calculus.
        #[command(flatten)]
        statement: StatementArgs,
    },
    /// Make a node from other nodes by an operation, and run it
    ///
    /// The nodes come first, by their numbers, then the operation, such as
    /// `ordinary.translate`; `operations` lists them. The node runs under
    /// the limits given here, which it keeps.
    Apply {
        /// The workspace file
        file: PathBuf,
        /// The input nodes, then the operation
        #[arg(required = true, num_args = 1..)]
        words: Vec<String>,
        /// A parameter of the operation, as KEY=VALUE, such as
        /// translation=glivenko; a value that is no JSON is a string
        #[arg(long = "set", value_name = "KEY=VALUE")]
        set: Vec<String>,
        /// Make the node without running it
        #[arg(long)]
        no_run: bool,
        /// The limits the node keeps, and the time limit.
        #[command(flatten)]
        common: WorkspaceArgs,
    },
    /// Change a given node's sequent, or a node's parameters or limits
    Set {
        /// The workspace file
        file: PathBuf,
        /// The node
        node: u32,
        /// The node's new sequent, in its calculus
        sequent: Option<String>,
        /// A parameter of the node's operation, as KEY=VALUE
        #[arg(long = "set", value_name = "KEY=VALUE")]
        set: Vec<String>,
        /// The limits the node keeps, and the time limit.
        #[command(flatten)]
        common: WorkspaceArgs,
    },
    /// Make again what is out of date or not run: every node, or those
    /// named and what they are made from
    Run {
        /// The workspace file
        file: PathBuf,
        /// The nodes
        nodes: Vec<u32>,
        /// Also make each node's check of what its operation promises
        #[arg(long)]
        checks: bool,
        /// The time limit and the workspace's bounds.
        #[command(flatten)]
        common: RunArgs,
    },
    /// Remove a node and every node made from it
    Remove {
        /// The workspace file
        file: PathBuf,
        /// The node
        node: u32,
    },
    /// Take back the last change
    Undo {
        /// The workspace file
        file: PathBuf,
    },
    /// Bring back the change undo took back last
    Redo {
        /// The workspace file
        file: PathBuf,
    },
    /// List the changes, those undo takes back and those redo brings back
    Log {
        /// The workspace file
        file: PathBuf,
    },
    /// List the nodes, or write one node's value
    Show {
        /// The workspace file
        file: PathBuf,
        /// The node whose value to write
        node: Option<u32>,
        /// The output format: text, or json for the workspace's document,
        /// or for a node its value's document
        #[arg(long, value_enum, value_name = "FORMAT", default_value_t = AnswerFormat::Text)]
        format: AnswerFormat,
        /// Write to this file instead of standard output
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },
    /// Check again every node's check, searches included
    Check {
        /// The workspace file
        file: PathBuf,
        /// The time limit and the workspace's bounds.
        #[command(flatten)]
        common: RunArgs,
    },
    /// List the operations, or those that take a value of one kind
    Operations {
        /// The kind, such as ordinary.sequent
        kind: Option<String>,
    },
}

/// The calculus a sequent of a workspace is claimed in.
#[derive(Args, Clone, Debug)]
pub(crate) struct StatementArgs {
    /// The sequent is of ordinary logic, claimed in this logic; without
    /// it or --system, of linear logic
    #[arg(long, value_enum, value_name = "LOGIC")]
    pub(crate) logic: Option<LogicArg>,
    /// The sequent is of modal logic, claimed in this system, such as S4
    #[arg(long, value_name = "SYSTEM", value_parser = parse_system, conflicts_with = "logic")]
    pub(crate) system: Option<deduxit::modal::System>,
    /// The linear mode the sequent is claimed in, such as classical or
    /// intuitionistic
    #[arg(
        long,
        value_name = "MODE",
        default_value = "classical",
        conflicts_with_all = ["logic", "system"]
    )]
    pub(crate) mode: String,
}

/// What `workspace apply` and `workspace set` read: the limits a node
/// keeps and the time limit of the run.
#[derive(Args, Clone, Debug)]
pub(crate) struct WorkspaceArgs {
    /// Give up after this long, such as 500ms, 10s, 2m or 1h, or `none`
    /// (clock.time_limit_ms); a node the time limit stopped stays as it
    /// was
    #[arg(help_heading = LIMITS, long, value_name = "DURATION", value_parser = parse_time, default_value_t = Time(Some(DEFAULT_TIMEOUT)))]
    pub(crate) timeout: Time,
    /// The most memory a search may hold, which the node keeps; see `prove
    /// --memory-limit` (limits.memory_bytes)
    #[arg(help_heading = LIMITS, long, value_name = "SIZE", value_parser = parse_bytes, default_value_t = Bytes(Some(Limits::DEFAULT_MEMORY_BYTES)))]
    pub(crate) memory_limit: Bytes,
    /// The most units of work each search may do, or `none`, which the
    /// node keeps; see `prove --work-limit` (limits.work)
    #[arg(help_heading = LIMITS, long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(None))]
    pub(crate) work_limit: Count<u64>,
    /// Read the settings from a JSON file; the flags override it
    ///
    /// The file is an object with any of the keys `clock`, `limits`,
    /// `search`, `view`, `styles`, `batch`, `ordinary`, `interactive`,
    /// `input`, `cut` and `lambda`. A node keeps the keys its operation
    /// reads, such as `{"limits": {"work": 100000}}`; the bounds of the
    /// workspace itself are `limits.workspace`. A field left out keeps the
    /// command's default, and a misspelt one is an error.
    #[arg(long, value_name = "PATH")]
    pub(crate) settings: Option<PathBuf>,
}

/// What `workspace run` and `workspace check` read: the time limit of
/// the run and the workspace's bounds. Each node runs under the limits it
/// keeps, so these commands take no other limit.
#[derive(Args, Clone, Debug)]
pub(crate) struct RunArgs {
    /// Give up after this long, such as 500ms, 10s, 2m or 1h, or `none`
    /// (clock.time_limit_ms); the nodes run before it stay
    #[arg(help_heading = LIMITS, long, value_name = "DURATION", value_parser = parse_time, default_value_t = Time(Some(DEFAULT_TIMEOUT)))]
    pub(crate) timeout: Time,
    /// Read the settings from a JSON file; the flags override it
    ///
    /// The file is an object with any of the keys `clock`, `limits`,
    /// `search`, `view`, `styles`, `batch`, `ordinary`, `interactive`,
    /// `input`, `cut` and `lambda`. Each node runs under the limits it
    /// keeps, so only the time limit and the workspace's own bounds,
    /// `limits.workspace`, are read here; `workspace set` changes a node's
    /// limits. A misspelt field is an error.
    #[arg(long, value_name = "PATH")]
    pub(crate) settings: Option<PathBuf>,
}

impl Command {
    /// Returns, for a command that reads one sequent, the words that name
    /// it, where its sequent comes from, and whether it reads ordinary
    /// logic.
    pub(crate) fn sequent_input(&self) -> Option<(&'static [&'static str], &SequentInput, bool)> {
        match self {
            Command::Prove(args) => Some((&["prove"], &args.input, args.logic.logic.is_some())),
            Command::Interact(args) => {
                Some((&["interact"], &args.input, args.logic.logic.is_some()))
            }
            Command::Seq {
                command: SeqCommand::Print { input, logic, .. },
            } => Some((&["seq", "print"], input, logic.logic.is_some())),
            Command::Seq {
                command: SeqCommand::Json { input, .. },
            } => Some((&["seq", "json"], input, false)),
            Command::Seq {
                command: SeqCommand::Fragment { input, .. },
            } => Some((&["seq", "fragment"], input, false)),
            Command::Interpolate(args) => Some((&["interpolate"], &args.common.input, true)),
            Command::Resolve(args) => Some((&["resolve"], &args.common.input, true)),
            Command::Grade(args) => Some((&["grade"], &args.input, args.logic.is_some())),
            Command::Seq {
                command: SeqCommand::Translate(args),
            } => Some((&["seq", "translate"], &args.common.input, true)),
            Command::Seq {
                command: SeqCommand::Normalize(args),
            } => Some((&["seq", "normalize"], &args.common.input, true)),
            Command::Term {
                command: TermCommand::Inhabit(args),
            } => Some((&["term", "inhabit"], &args.common.input, true)),
            Command::Term {
                command: TermCommand::Enumerate(args),
            } => Some((&["term", "enumerate"], &args.common.input, true)),
            Command::Check(_)
            | Command::Eliminate(_)
            | Command::Workspace { .. }
            | Command::Term {
                command: TermCommand::Check(_),
            } => None,
        }
    }

    /// Returns whether the command would take `--logic` as it was called:
    /// it has the flag, and no mode flag that the flag refuses is given.
    pub(crate) fn takes_logic(&self) -> bool {
        let plain = |mode: &ModeArgs| {
            !(mode.intuitionistic || mode.affine || mode.mix || mode.cyclic || mode.lambek)
        };
        match self {
            Command::Prove(args) => plain(&args.mode),
            Command::Interact(args) => plain(&args.mode),
            Command::Grade(args) => plain(&args.mode),
            Command::Seq {
                command:
                    SeqCommand::Print {
                        intuitionistic,
                        cyclic,
                        lambek,
                        ..
                    },
            } => !(*intuitionistic || *cyclic || *lambek),
            Command::Seq { .. }
            | Command::Workspace { .. }
            | Command::Check(_)
            | Command::Eliminate(_)
            | Command::Interpolate(_)
            | Command::Resolve(_)
            | Command::Term { .. } => false,
        }
    }
}

/// The arguments of `prove`.
#[derive(Args, Clone, Debug)]
pub(crate) struct ProveArgs {
    /// The logic.
    #[command(flatten)]
    pub(crate) mode: ModeArgs,
    /// The ordinary logic the input is in, if any.
    #[command(flatten)]
    pub(crate) logic: LogicArgs,
    /// Read the input as modal logic and decide it in this system
    ///
    /// The input is a formula or a sequent of propositional modal logic:
    /// ordinary logic's syntax with the prefixes [] (necessarily) and <>
    /// (possibly), as `[]p -> [][]p`, or its JSON form. The system is one of
    /// K, D, T, KB, KDB, B, K4, KD4, S4, K5, KD5, K45, KD45, KB5 and S5, or
    /// another name of one, such as KT4 for S4. A valid sequent is shown as
    /// a checked derivation in the system's labelled sequent calculus,
    /// G3 and the system's name, which `check --calculus` reads; one that is
    /// not valid with a checked Kripke countermodel, each world's arrows the
    /// worlds it sees. The JSON output is the modal outcome.
    #[arg(help_heading = LOGIC, long, value_name = "SYSTEM", value_parser = parse_system, conflicts_with = "logic")]
    pub(crate) system: Option<deduxit::modal::System>,
    /// With --logic, how to decide the sequent: through its embedding into
    /// linear logic, by LJT, by a SAT solver, by its truth table, by a
    /// tableau, by a connection prover, by grounding or by a search for a
    /// finite structure that refutes it (ordinary.method)
    ///
    /// By default every logic is decided by the SAT solver: intuitionistic
    /// and minimal logic with LJT's derivation where LJT gives one, and
    /// classical logic with the embedding's output (its derivation of LK,
    /// refutation and certificate) where the embedding decides within a
    /// short bound of work. A first-order sequent is decided by grounding
    /// where it lies in the classes grounding decides, else by the
    /// connection prover in classical logic and through the embedding in
    /// the others, the methods that take one; in classical logic the
    /// model search runs beside the connection prover, on a thread of its
    /// own, the first to answer winning; where they give up, the embedding
    /// decides in its place.
    /// LJT is Dyckhoff's calculus for intuitionistic and minimal logic. The
    /// SAT solver decides every logic; a valid classical sequent gets a
    /// checked clausal proof and no derivation. --translation and --linear
    /// name the embedding, so either of them alone decides through it.
    /// The connection prover decides classical and intuitionistic logic,
    /// first-order sequents included: a valid sequent gets a checked
    /// derivation of LK or LJ, and a search that finds no proof is
    /// unknown, never not valid, but for a classical sequent without
    /// quantifiers (--path-length bounds it).
    /// Grounding decides the first-order sequents without function
    /// symbols where no ∀ stands under an ∃ (the Bernays–Schönfinkel
    /// class) or no predicate takes two or more arguments (the monadic
    /// class); "under" is read once negations are pushed inward, the
    /// hypotheses negated, and the quantifiers moved inward as far as they
    /// go. It replaces the quantifiers by the finitely many constants and
    /// hands the result to the SAT solver. A sequent that is not valid
    /// gets a checked finite structure, printed after the verdict: its
    /// elements, each constant's element and where each predicate holds.
    /// In intuitionistic and minimal logic a classically valid sequent is
    /// unknown to grounding.
    /// The model search looks for a finite structure that refutes a
    /// first-order sequent, of 1, 2, … elements (--model-size bounds
    /// them), by the SAT solver. A structure it finds is checked and
    /// printed after the verdict, each function's table among the rest;
    /// it refutes the sequent in every logic. It never finds a sequent
    /// valid: past the bound or the time limit the sequent is unknown.
    /// The truth table and the tableau decide classical logic, and print
    /// the table or the tableau in place of a derivation: the table's rows
    /// that falsify the sequent marked `×`, the tableau's lines numbered,
    /// each branch closed with `×` or left open. --table-atoms bounds the
    /// table.
    /// Under LJT, the SAT solver, the table, the tableau, the
    /// connection prover, grounding and the model search, --stats prints
    /// the method's counters. Under every method --format json writes the
    /// outcome of the ordinary sequent, its "search" tagged by "method".
    #[arg(help_heading = LOGIC, long, value_enum, value_name = "METHOD", requires = "logic")]
    pub(crate) method: Option<MethodArg>,
    /// The sequent.
    #[command(flatten)]
    pub(crate) input: SequentInput,
    /// With --logic, show the linear proof of the image instead of the
    /// proof read back as LK or LJ
    #[arg(help_heading = OUTPUT, long, requires = "logic")]
    pub(crate) linear: bool,
    /// How a derivation of LK or LJ shows its structural rules.
    #[command(flatten)]
    pub(crate) structural: StructuralArgs,
    /// Search in this fragment instead of the sequent's own
    /// (search.fragment)
    ///
    /// A sequent outside it is an error. A fragment larger than the
    /// sequent's switches off the prunes that hold only in the smaller one,
    /// which lets you compare the searches with and without them.
    #[arg(help_heading = SEARCH, long, value_enum, value_name = "FRAGMENT")]
    pub(crate) fragment: Option<FragmentArg>,
    /// The engine to search with, or `auto` for the one the fragment and
    /// the mode call for (search.engine)
    ///
    /// `focus` is focused sequent search, for every classical fragment,
    /// with or without units, Mix, exponentials and weakening. `two-sided`
    /// is the same for every intuitionistic fragment. `net` is proof-net
    /// search, for MLL without units, with or without Mix, and in
    /// intuitionistic mode for IMLL without units by its embedding.
    /// `additive` is the fast path for two additive-only formulas, in every
    /// mode. `horn` explores the markings of a Petri net, for Horn programs
    /// (atoms, implications between tensors of atoms, such implications
    /// under !, and one goal that is a tensor of atoms): reachability in
    /// linear mode, coverability in affine mode. `essential` is proof-net
    /// search over essential nets, directed by the intuitionistic reading,
    /// for IMLL without units. `inverse` is the focused inverse method, a
    /// forward search from the axioms, in every mode but the ordered ones;
    /// it decides without exponentials and needs no copy bound with them.
    /// Named here, it searches alone. Without --engine, on one thread, it
    /// also takes turns beside the engine on an intuitionistic goal with an
    /// exponential and on a goal under Mix without weakening, and on some
    /// sequents of MLL without Mix it takes over where the focus engine
    /// meets its memory bound.
    #[arg(help_heading = SEARCH, long, value_name = "ENGINE", value_parser = engine_name(), default_value = "auto")]
    pub(crate) engine: Forced,
    /// The search's options, bounds and threads.
    #[command(flatten)]
    pub(crate) search: SearchArgs,
    /// Report the proof the search found without checking it (search.check)
    ///
    /// By default every proof passes the proof checker before anything is
    /// reported. The checker shares no code with the search. A proof it
    /// rejects is an error (exit status 2), not a verdict.
    #[arg(help_heading = SEARCH, long)]
    pub(crate) no_check: bool,
    /// Where and how to write the result.
    #[command(flatten)]
    pub(crate) output: OutputArgs,
    /// Also print what the search cost: the counters of the engine that
    /// ran, a line each, and the time
    ///
    /// The focused engines count stable sequents, memo use, splits and the
    /// copy bound reached. The net engine counts literals chosen, links
    /// tried and exact tests run, and the essential-net engine the same
    /// with tests of the dominator condition. The additive engine counts
    /// pairs of subformulas, the Horn engine markings, and the inverse
    /// method the derived sequents taken up, subsumed and kept. Every
    /// engine counts its units of work. JSON output always has the counts.
    /// The time is printed only as text, and as `seconds` in a batch's JSON
    /// records.
    #[arg(help_heading = OUTPUT, long)]
    pub(crate) stats: bool,
    /// The most atoms a truth table may have, or `none`
    /// (limits.table_atoms)
    ///
    /// The rows double with each atom, so `--method table` refuses a
    /// sequent of more atoms (exit status 2) unless this is raised.
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u32>, default_value_t = Count(Some(Limits::DEFAULT_TABLE_ATOMS)))]
    pub(crate) table_atoms: Count<u32>,
    /// Read the settings from a JSON file; the flags override it
    ///
    /// The file is an object with any of the keys `clock`, `limits`,
    /// `search`, `view`, `styles`, `batch`, `ordinary`, `interactive`,
    /// `input`, `cut` and `lambda`. Each holds the fields that the flags
    /// name in parentheses, such as `{"search":
    /// {"copies": 3}, "limits": {"memory_bytes": null}}`. A field left out
    /// keeps the command's default, and a misspelt one is an error.
    /// `ordinary` applies under --logic, which names the logic.
    /// `batch.mode` is a batch's mode unless a mode flag is given; one
    /// sequent's mode is the flags'.
    #[arg(long, value_name = "PATH")]
    pub(crate) settings: Option<PathBuf>,
    /// How a batch of many sequents runs.
    #[command(flatten)]
    pub(crate) batch: BatchArgs,
}

/// The search's options, bounds and threads, on `prove` and on a
/// session's `close`; each flag sets one key of `--settings`.
#[derive(Args, Clone, Debug)]
pub(crate) struct SearchArgs {
    /// How the focused engines pick the positive literal of every atom
    /// (search.bias)
    ///
    /// No choice changes what is provable. `rarer` mostly chains backward
    /// from the goal. `factors` mostly chains forward from the hypotheses:
    /// on Horn clauses under `!`, such as a Petri net, that is often much
    /// faster, but it takes one copy per step on a single branch. `auto`
    /// takes `rarer` in affine mode and `factors` in linear mode without
    /// exponentials. In linear mode with exponentials it runs both searches
    /// and answers with the first that decides: in turns on one core, side
    /// by side on several.
    #[arg(help_heading = SEARCH, hide_short_help = true, long, value_enum, value_name = "BIAS", default_value_t = BiasArg::Auto)]
    pub(crate) bias: BiasArg,
    /// How often `?` formulas may be copied on one branch of the proof, or
    /// `none` for no bound (search.copies)
    ///
    /// The search tries the bounds 0, 1, … in turn. By default it goes on
    /// to the next bound until it decides or the time limit passes, and an
    /// unknown verdict says which bound it reached. With a bound, a sequent
    /// with exponentials that has no proof within it is unknown (exit
    /// status 3), unless a smaller bound already exhausted the search; it
    /// is then unprovable. The bound does not affect sequents without
    /// exponentials.
    #[arg(help_heading = SEARCH, long, value_name = "N", value_parser = parse_count::<u32>, default_value_t = Count(None))]
    pub(crate) copies: Count<u32>,
    /// How often `?` formulas may be copied on one branch of the forward
    /// search that `--bias auto` runs (search.forward_copies)
    ///
    /// It applies to a Horn program: clauses such as `!(a * b -o c * d)`,
    /// a marking and a goal of atoms, as in a Petri net. There a copy is
    /// one step of a chain, and a chain of n steps needs n copies. On a
    /// Horn program the forward search's bound is this or `--copies`,
    /// whichever is larger; on any other sequent, and under Mix, it is
    /// `--copies`. Without a `--copies` bound this flag has no effect: both
    /// searches deepen while the time limit lasts.
    #[arg(help_heading = SEARCH, hide_short_help = true, long, value_name = "N", default_value_t = Options::DEFAULT_FORWARD_COPIES)]
    pub(crate) forward_copies: u32,
    /// Give up after this long, such as 500ms, 10s, 2m or 1h, or `none`
    /// for no limit (clock.time_limit_ms)
    ///
    /// The verdict is then unknown (exit status 3). The time counts from
    /// the start of the command, so it covers reading and parsing the
    /// sequent as well as the search: a sequent that is not read in time
    /// is given up on like one that is not decided. Without a limit, the
    /// search runs until it decides or you press Ctrl-C; with exponentials
    /// and no `--copies` bound, that can be forever. In a session the limit
    /// is each `close`'s.
    #[arg(help_heading = LIMITS, long, value_name = "DURATION", value_parser = parse_time, default_value_t = Time(Some(DEFAULT_TIMEOUT)))]
    pub(crate) timeout: Time,
    /// The most decided sequents the search remembers at once
    /// (search.memo_limit)
    ///
    /// When the memo is full it is emptied, which costs time but never
    /// changes a verdict. Zero switches the memo off. `--memory-limit`
    /// bounds the memo in bytes; this flag bounds it more finely.
    #[arg(help_heading = SEARCH, hide_short_help = true, long, value_name = "N", default_value_t = Options::DEFAULT_MEMO_LIMIT)]
    pub(crate) memo_limit: u32,
    /// The most work the classical refutation may do after a search that
    /// did not prove the sequent, in formulas evaluated, or 0 for none
    /// (search.classical_work)
    ///
    /// It looks for truth values of the atoms that make the sequent false
    /// read classically: `!` and `?` erased, `*` and `&` as "and", `|` and
    /// `+` as "or", `1` and `top` true, `bot` and `0` false. No provable
    /// sequent has such values. When it finds them, a search that was
    /// unknown becomes unprovable (exit status 1), and an exhausted one
    /// gets the values as its reason. It runs after the search, past the
    /// time limit but within this bound, and Ctrl-C stops it. It never
    /// changes a verdict the search gave.
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", default_value_t = Options::DEFAULT_CLASSICAL_WORK)]
    pub(crate) classical_work: u64,
    /// The deepest term a variable of a first-order search may be bound
    /// to (search.witness_depth)
    ///
    /// The focused engine searches a sequent with quantifiers by leaving
    /// the witness of each `exists` open, and filling it in where an axiom
    /// matches two atoms. A constant has depth 0, and `f(t)` one more than
    /// `t`. A branch that would bind a deeper term is cut, and a search
    /// that cut one is unknown (exit status 3), not unprovable.
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", default_value_t = Options::DEFAULT_WITNESS_DEPTH)]
    pub(crate) witness_depth: u32,
    /// The most instances of quantified formulas a first-order search
    /// may hold at once (search.instances)
    ///
    /// Each `forall` and `exists` the search steps through opens one, and
    /// copies of quantified hypotheses under `!` multiply them. A branch
    /// that would hold more is cut, and a search that cut one is unknown
    /// (exit status 3), not unprovable.
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", default_value_t = Options::DEFAULT_INSTANCES)]
    pub(crate) instances: u32,
    /// The longest active path the connection prover searches with, or
    /// `none` for no bound (search.path_length)
    ///
    /// The connection prover (--method connection) tries the bounds 1, 2,
    /// … in turn, and only a step that makes a fresh variable is held to
    /// the bound. By default it goes on until it decides or the time limit
    /// passes; with a bound, a sequent with no proof within it is unknown
    /// (exit status 3).
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u32>, default_value_t = Count(None))]
    pub(crate) path_length: Count<u32>,
    /// The most elements of a finite structure the model search tries, or
    /// `none` for no bound (search.model_size)
    ///
    /// The model search (--method models) tries structures of 1, 2, …
    /// elements in turn. By default it goes on until it finds one or the
    /// time limit passes; with a bound, a sequent no structure within it
    /// refutes is unknown (exit status 3).
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u32>, default_value_t = Count(None))]
    pub(crate) model_size: Count<u32>,
    /// The most worlds of a Kripke model the model search tries in
    /// intuitionistic and minimal logic, or `none` for no bound
    /// (search.model_worlds)
    ///
    /// Beside structures of 1, 2, … elements the model search (--method
    /// models) tries Kripke models of 1, 2, … worlds, each world's domain
    /// of up to as many elements as a structure. By default it goes on
    /// until it finds one or the time limit passes; with bounds, a sequent
    /// no model within them refutes is unknown (exit status 3).
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u32>, default_value_t = Count(None))]
    pub(crate) model_worlds: Count<u32>,
    /// Keep the connection prover's first solution of each literal, never
    /// trying another (search.restricted_backtracking)
    ///
    /// The search often ends sooner, but it may miss a proof, so a sequent
    /// it finds no proof of is unknown (exit status 3), never not valid.
    #[arg(help_heading = SEARCH, hide_short_help = true, long)]
    pub(crate) restricted_backtracking: bool,
    /// The most memory the search may hold: a number of bytes with a unit
    /// such as 512MiB or 4GiB, or `none` for no limit (limits.memory_bytes)
    ///
    /// The limit counts the sequent as the search holds it, about 25 bytes
    /// for each subformula occurrence, and what grows with the search: what
    /// it remembers, the proofs it keeps and what every level of its
    /// recursion takes. A memo that no longer fits is emptied first, as at
    /// `--memo-limit`; when that is not enough, the verdict is unknown
    /// (exit status 3). The proof's check and the derivation built from it
    /// are under the same limit: a derivation estimated above it is left
    /// out even with `--derivation-limit none`. By default one thread and,
    /// after `--pool-after`, a pool beside it search at once, both together
    /// within the limit. A png or pdf render is estimated before it starts,
    /// its time counted as the memory the renderer fills in it, and left
    /// out above the limit; the default allows a few seconds of rendering.
    #[arg(help_heading = LIMITS, long, value_name = "SIZE", value_parser = parse_bytes, default_value_t = Bytes(Some(Limits::DEFAULT_MEMORY_BYTES)))]
    pub(crate) memory_limit: Bytes,
    /// The deepest nesting of rules on one branch before the search gives
    /// up (limits.recursion_depth)
    ///
    /// Raise it for sequents with thousands of connectives. The search runs
    /// on a thread whose stack grows with the limit.
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", default_value_t = Limits::DEFAULT_RECURSION_DEPTH)]
    pub(crate) recursion_limit: u32,
    /// The most units of work the search may do, or `none` (limits.work)
    ///
    /// A unit is what each engine counts as it goes: a stable sequent or a
    /// split for the focused engine, a literal or a failed test for the net
    /// engines, a pair for the additive one, a marking for the Horn engine.
    /// So this limit gives the same verdict on every machine, which a time
    /// limit does not. Past it the verdict is unknown (exit status 3).
    /// `--stats` prints the units a search did.
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(None))]
    pub(crate) work_limit: Count<u64>,
    /// How many threads the search may use (search.jobs)
    ///
    /// By default one thread searches first, for the time `--pool-after`
    /// gives, and then the machine's other threads join it. So a small
    /// sequent is decided at once and always the same way, and a hard one
    /// gets every core. A number given here is used from the start,
    /// unless `--pool-after` is given too. A number above the machine's
    /// parallelism is taken as that, with a note on standard error. With
    /// more than one thread the focused engine and the net engine search in
    /// parallel; the proof found may then differ from run to run, but the
    /// verdict never does.
    #[arg(help_heading = SEARCH, short, long, value_name = "N")]
    pub(crate) jobs: Option<usize>,
    /// How long one thread searches before the other threads join it,
    /// such as 100ms; 0 starts them at once (clock.pool_after_ms)
    ///
    /// The default is 100ms when `--jobs` is not given, and 0 when it is.
    /// The single thread goes on searching beside a pool of the other
    /// threads, which starts the search afresh, and the first to decide
    /// answers. Both together hold at most `--memory-limit`. Below three
    /// threads there is no single thread: two search on a pool from the
    /// start. The additive, Horn, essential-net and inverse engines run on
    /// one thread and get no pool.
    #[arg(help_heading = SEARCH, hide_short_help = true, long, value_name = "DURATION", value_parser = parse_duration)]
    pub(crate) pool_after: Option<Duration>,
    /// Search on one thread, so that the same input always gives the same
    /// proof and the same counts
    ///
    /// The same as `--jobs 1 --schedule turns`. It takes precedence over
    /// `--jobs`, `--pool-after` and `--schedule`.
    #[arg(help_heading = SEARCH, long)]
    pub(crate) deterministic: bool,
    /// How often the net engine runs its exact test of correctness, and
    /// the essential-net engine its test of the dominator condition: after
    /// this many links, or `auto` (search.test_period)
    ///
    /// `auto` tests after every link on a small structure, and every few
    /// links on a large one. The verdict never depends on it.
    #[arg(help_heading = SEARCH, hide_short_help = true, long, value_name = "N", value_parser = parse_cadence, default_value = "auto")]
    pub(crate) test_period: Cadence,
    /// How the two searches of `--bias auto` share one thread
    /// (search.schedule)
    #[arg(help_heading = SEARCH, hide_short_help = true, long, value_enum, value_name = "SCHEDULE", default_value_t = ScheduleArg::Auto)]
    pub(crate) schedule: ScheduleArg,
}

/// How the two searches of the default bias share one thread.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScheduleArg {
    /// In slices on two threads
    Auto,
    /// In turns on the calling thread, each turn from the search's start,
    /// which gives the same answer and the same counts on every machine
    Turns,
}

impl From<ScheduleArg> for Schedule {
    /// Returns the library's value of the choice.
    fn from(schedule: ScheduleArg) -> Self {
        match schedule {
            ScheduleArg::Auto => Self::Auto,
            ScheduleArg::Turns => Self::Turns,
        }
    }
}

/// Parses a test period: a number of links, or `auto`.
fn parse_cadence(text: &str) -> Result<Cadence, String> {
    if text == "auto" {
        return Ok(Cadence::Auto);
    }
    text.parse()
        .map(Cadence::Every)
        .map_err(|_| format!("{text:?} is neither a number of links nor `auto`"))
}

/// How `prove` decides many sequents in one call (`prove --help` says
/// when a call is a batch and what it writes).
#[derive(Args, Clone, Debug)]
pub(crate) struct BatchArgs {
    /// Read the input paths from this file, one per line, or from standard
    /// input with `-`, as it streams
    #[arg(help_heading = BATCH, hide_short_help = true, long, value_name = "LIST")]
    pub(crate) files_from: Option<PathBuf>,
    /// Take the paths of --files-from as separated by NUL characters, as
    /// `find -print0` writes them
    #[arg(help_heading = BATCH, hide_short_help = true, long)]
    pub(crate) null: bool,
    /// How many sequents of a batch to decide at once, across the cores
    /// (batch.workers)
    ///
    /// By default as many as the machine runs threads, or fewer when
    /// --batch-memory cannot hold that many searches of --memory-limit
    /// each.
    #[arg(help_heading = BATCH, hide_short_help = true, long, value_name = "N")]
    pub(crate) workers: Option<usize>,
    /// Where a batch's threads go (batch.cores)
    ///
    /// `across` gives each sequent one thread for its --timeout, as
    /// --deterministic does. It answers the most sequents in a given time,
    /// and every answer depends on its input alone. A sequent that needs
    /// more time on one thread, or the pool that a single call races beside
    /// one thread, stays unknown there. `within` gives each sequent the race
    /// on every thread, as a single call does. To decide what a batch left
    /// unknown across the cores, run those sequents again with --cores
    /// within or a longer --timeout.
    #[arg(help_heading = BATCH, hide_short_help = true, long, value_enum, value_name = "WHERE", default_value_t = CoresArg::Auto)]
    pub(crate) cores: CoresArg,
    /// The most memory all the searches of a batch hold together: a size
    /// such as 8GiB, or `none` (batch.total_memory_bytes)
    ///
    /// By default half the memory the process may use: the machine's, or
    /// its control group's limit where one is set. Each search holds at
    /// most --memory-limit, and as many run at once as this holds. Within
    /// one sequent, the race of one thread and a pool is two searches,
    /// which share it.
    #[arg(help_heading = BATCH, hide_short_help = true, long, value_name = "SIZE", value_parser = parse_bytes)]
    pub(crate) batch_memory: Option<Bytes>,
    /// Give up on the rest of a batch after this long, or `none`
    /// (clock.batch_time_limit_ms)
    ///
    /// The searches still running are stopped, and the sequents not begun
    /// are unknown. --timeout is each sequent's own limit.
    #[arg(help_heading = BATCH, hide_short_help = true, long, value_name = "DURATION", value_parser = parse_time, default_value_t = Time(None))]
    pub(crate) batch_timeout: Time,
    /// Decide each sequent of a batch in a child process of its own, so
    /// that one that exhausts the memory or the stack ends only itself
    #[arg(help_heading = BATCH, hide_short_help = true, long)]
    pub(crate) isolate: bool,
    /// The name of the one sequent a child of --isolate decides.
    #[arg(long, hide = true, value_name = "NAME", allow_hyphen_values = true)]
    pub(crate) entry_name: Option<String>,
    /// The mode of the one sequent a child of --isolate decides.
    #[arg(long, hide = true, value_name = "MODE", requires = "entry_name")]
    pub(crate) entry_mode: Option<String>,
    /// The format of the one sequent a child of --isolate decides.
    #[arg(
        long,
        hide = true,
        value_enum,
        value_name = "FORMAT",
        requires = "entry_name"
    )]
    pub(crate) entry_format: Option<InputFormat>,
    /// The file of the one sequent a child of --isolate decides; standard
    /// input when absent.
    #[arg(
        long,
        hide = true,
        value_name = "PATH",
        requires = "entry_name",
        allow_hyphen_values = true
    )]
    pub(crate) entry_file: Option<PathBuf>,
}

/// Where a batch's threads go.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CoresArg {
    /// Across when the batch has at least as many sequents as --workers,
    /// else within; within on a stream from standard input, whose next
    /// sequent may wait for an answer
    Auto,
    /// One sequent per worker, each on one thread, as with --deterministic
    Across,
    /// One sequent at a time, with the threads --jobs gives
    Within,
}

/// The arguments of `interact`.
#[derive(Args, Debug)]
pub(crate) struct InteractArgs {
    /// The sequent, as an argument or with --file, since standard input
    /// carries the commands.
    #[command(flatten)]
    pub(crate) input: SequentInput,
    /// Resume a session saved with `save`; the file gives the mode or the
    /// logic, so the mode flags and --logic are refused
    #[arg(
        help_heading = INPUT,
        long,
        value_name = "PATH",
        conflicts_with_all = ["sequent", "file", "intuitionistic", "affine", "mix", "logic", "calculus", "assume"]
    )]
    pub(crate) state: Option<PathBuf>,
    /// Prove in a calculus given as data: a rule file, or one built in
    /// (LK, LJ, LM, LL, tableau, and natural deduction's NJ, NK, NM,
    /// NK-DNE, NK-LEM, NJ-defined, NK-defined, NM-defined)
    ///
    /// The sequent is then written in the calculus's own syntax, its
    /// connectives by their symbols or names (`->` reads as `→`, `/\` as
    /// `∧`, `|-` as `⊢` where the calculus has them), and the session's
    /// commands apply its rules backwards. Each step is checked by the
    /// calculus's checker, and the finished derivation is "valid in the
    /// calculus given": the derivation keeps to the rules, which nothing
    /// checks for soundness. See `check --calculus`.
    #[arg(help_heading = LOGIC, long, value_name = "CALCULUS", conflicts_with_all = ["logic", "intuitionistic", "affine", "mix"])]
    pub(crate) calculus: Option<PathBuf>,
    /// With --calculus, a hypothesis the derivation may use and leave
    /// undischarged, a sequent of the calculus; repeat it for more
    #[arg(help_heading = LOGIC, long, value_name = "SEQUENT", requires = "calculus")]
    pub(crate) assume: Vec<String>,
    /// The logic.
    #[command(flatten)]
    pub(crate) mode: ModeArgs,
    /// The ordinary logic the sequent is in, if any: the session then
    /// proves it in LK or LJ.
    #[command(flatten)]
    pub(crate) logic: LogicArgs,
    /// How `show` and `proof` draw a derivation of LK or LJ.
    #[command(flatten)]
    pub(crate) structural: StructuralArgs,
    /// How the command `lambda` prints the proof's lambda-term.
    #[command(flatten)]
    pub(crate) lambda_style: LambdaArgs,
    /// The search's options, bounds and threads, for a `close`.
    #[command(flatten)]
    pub(crate) search: SearchArgs,
    /// Apply every invertible rule by itself, at the start and to the goals
    /// each step opens (interactive.invertible)
    ///
    /// The invertible rules are ⅋, ⊥, & and ⊤; ! when every other formula
    /// is a ?; and 1, ⊗ and the axiom when their formulas are the whole
    /// goal. Under --logic, the axiom and every rule of a connective with
    /// a shared context, but in LJ →L, ¬L and ∨R₁, ∨R₂. The session command
    /// `invertible on|off` turns this on or off.
    #[arg(long)]
    pub(crate) invertible: bool,
    /// The largest derivation `close` grafts, by its estimated size; see
    /// `prove --derivation-limit` (limits.derivation_bytes)
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "SIZE", value_parser = parse_bytes, default_value_t = Bytes(Some(Limits::DEFAULT_DERIVATION_BYTES)))]
    pub(crate) derivation_limit: Bytes,
    /// How `show` and `proof` write; a `--style` key names its format
    /// with a prefix here, `latex.labels=subscript`.
    #[command(flatten)]
    pub(crate) style: StyleArgs,
    /// The Rocq certificate's lemma and prelude, for `proof --rocq`.
    #[command(flatten)]
    pub(crate) rocq: RocqArgs,
    /// Read the settings from a JSON file; the flags override it
    ///
    /// The file is an object with any of the keys `clock`, `limits`,
    /// `search`, `view`, `styles`, `batch`, `ordinary`, `interactive`,
    /// `input`, `cut` and `lambda`. Each holds the fields that the flags
    /// name in parentheses, such as `{"search":
    /// {"copies": 3}, "limits": {"memory_bytes": null}}`. A field left out
    /// keeps the command's default, and a misspelt one is an error.
    /// `ordinary` applies under --logic, which names the logic.
    #[arg(long, value_name = "PATH")]
    pub(crate) settings: Option<PathBuf>,
}

/// The time limit of `prove` and of a session's `close` when none is
/// given: what a larger copy bound still decides against what every
/// undecided sequent then waits. Of the problems of the LLTP library that
/// a copy bound of 3 leaves undecided and a larger bound decides, nine in
/// ten are decided within a second or two, while the many that nothing
/// decides each wait the whole limit.
pub(crate) const DEFAULT_TIMEOUT: Duration = Duration::from_millis(Clock::DEFAULT_TIME_LIMIT_MS);

/// The arguments of `check`.
#[derive(Args, Debug)]
pub(crate) struct CheckArgs {
    /// The most memory the check may hold: a number of bytes with a unit
    /// such as 512MiB or 4GiB, or `none` for no limit
    ///
    /// A check that would pass it stops, and the verdict is unknown (exit
    /// status 3): the proof is neither valid nor invalid. A derivation, or a
    /// png or pdf render, estimated above it is left out, as past
    /// `--derivation-limit`. See `prove --memory-limit`
    /// (limits.memory_bytes).
    #[arg(help_heading = LIMITS, long, value_name = "SIZE", value_parser = parse_bytes, default_value_t = Bytes(Some(Limits::DEFAULT_MEMORY_BYTES)))]
    pub(crate) memory_limit: Bytes,
    /// Give up after this long, such as 500ms, 10s, 2m or 1h, or `none`
    /// (clock.time_limit_ms)
    ///
    /// A proof file can make its check quadratic, with a large shared
    /// subproof read by many nodes. When the time runs out, the verdict is
    /// unknown (exit status 3). The time counts from the start of the
    /// command, and covers the derivation drawn after the check.
    #[arg(help_heading = LIMITS, long, value_name = "DURATION", value_parser = parse_time, default_value_t = Time(Some(DEFAULT_TIMEOUT)))]
    pub(crate) timeout: Time,
    /// The most units of work the check may do, or `none` (limits.work)
    ///
    /// A unit is a node of the proof, or an entry of a sequent handed to a
    /// node. Past the limit the verdict is unknown (exit status 3).
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(None))]
    pub(crate) work_limit: Count<u64>,
    /// The most atoms a truth table may have, or `none`
    /// (limits.table_atoms); see `prove --table-atoms`
    ///
    /// A truth table that `prove --method table --table-atoms N` made with
    /// more atoms is checked only with the same bound.
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u32>, default_value_t = Count(Some(Limits::DEFAULT_TABLE_ATOMS)))]
    pub(crate) table_atoms: Count<u32>,
    /// The most subformula occurrences the proof's sequent may have, or
    /// `none` (limits.occurrences); see `prove --occurrence-limit`
    #[arg(help_heading = INPUT, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(Some(Limits::DEFAULT_OCCURRENCES)))]
    pub(crate) occurrence_limit: Count<u64>,
    /// The most first-order terms the proof's sequent may unfold to and
    /// entries its instance table may hold, or `none` (limits.terms); see
    /// `prove --term-limit`
    #[arg(help_heading = INPUT, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(Some(Limits::DEFAULT_TERMS)))]
    pub(crate) term_limit: Count<u64>,
    /// The proof file, or standard input when absent or `-`
    #[arg(value_name = "PROOF")]
    pub(crate) proof: Option<PathBuf>,
    /// Read the proof from this file, or `-` for standard input, as the
    /// argument PROOF does
    #[arg(short, long, value_name = "PATH", conflicts_with = "proof")]
    pub(crate) file: Option<PathBuf>,
    /// The logic the proof must hold in.
    #[command(flatten)]
    pub(crate) mode: ModeArgs,
    /// With a Kripke model, the ordinary logic to check it in
    ///
    /// The file is then a Kripke model in JSON, as `prove --logic
    /// --format json` writes one under `countermodel`, and --sequent names
    /// the sequent to check it against.
    #[arg(help_heading = LOGIC, long, value_enum, value_name = "LOGIC")]
    pub(crate) logic: Option<LogicArg>,
    /// With a Kripke model, the modal system to check it in
    ///
    /// The file is then a Kripke model in JSON, as `prove --system --format
    /// json` writes one under `countermodel`, each world's arrows the worlds
    /// it sees, and --sequent names the modal sequent to check it against.
    /// The model must meet the system's frame conditions on its arrows as
    /// written (a model of S4 lists its loops); the check names the first
    /// condition it does not meet. Then it says whether the sequent holds at
    /// the world --world names, or at every world, and where it fails, the
    /// value there of each subformula and the world each false □ or true ◇
    /// looks to. The exit status is 0 where the sequent holds, 1 where it
    /// fails and 2 for a model that is none of the system.
    #[arg(help_heading = LOGIC, long, value_name = "SYSTEM", value_parser = parse_system, conflicts_with = "logic")]
    pub(crate) system: Option<deduxit::modal::System>,
    /// With a Kripke model, the sequent of ordinary logic (--logic) or of
    /// modal logic (--system) to check it against; with --tableau, the
    /// sequent the tableau must start from
    ///
    /// The model must be one of the logic: its worlds ordered by its
    /// arrows, with no arrows round, each world forcing what every world
    /// below it forces, and ⊥ forced only in minimal logic. Then the check
    /// says whether the sequent holds at the world --world names, or at
    /// every world: at every world at or above it that forces each formula
    /// left of ⊢, the formula right of it is forced. Where it fails, it
    /// names a world that forces each formula left of ⊢ and not the one
    /// right of it, and what that world forces of each subformula. The
    /// exit status is 0 where the sequent holds, 1 where it fails and 2
    /// for a model that is not one of the logic.
    #[arg(help_heading = INPUT, long, value_name = "SEQUENT")]
    pub(crate) sequent: Option<String>,
    /// With a Kripke model, the world to check the sequent at, by index;
    /// every world when absent
    #[arg(help_heading = INPUT, long, value_name = "N", requires = "sequent")]
    pub(crate) world: Option<u32>,
    /// The file is a signed tableau of classical logic written by hand, as
    /// text, to check step by step
    ///
    /// Each line is a number, T (true) or F (false), a formula, and at its
    /// end the number of the line it expands, which a rule's name may
    /// precede (T∧ 2 or ∧L 2). The first lines, which expand no line, are
    /// the sequent's. A branch that closes ends with × and the two lines of
    /// a formula true and false, one that stays open with `open`. A branch
    /// starts with `-` (or ├─ and └─, as `prove --method tableau` prints a
    /// tableau) under the lines it goes on from, its lines indented after
    /// it. The check names the first step that breaks a rule, a branch
    /// marked wrongly, or the first branch not finished. A tableau without
    /// fault is checked again as a derivation in the built-in calculus
    /// `tableau`; an open one's branch gives a checked countermodel. The
    /// exit status is 0 for a tableau that is right and finished, closed or
    /// open, and 1 for one with a fault or not finished.
    #[arg(help_heading = INPUT, long, conflicts_with_all = ["logic", "system", "calculus", "world"])]
    pub(crate) tableau: bool,
    /// The file is an outside SAT solver's proof that the clauses of this
    /// DIMACS CNF file are unsatisfiable, in the DRAT or LRAT format
    ///
    /// Every clause the proof adds must follow by unit propagation from
    /// the clauses kept, and the proof must add the empty clause. The
    /// exit status is 0 for a proof that holds and 1 for one that does
    /// not.
    #[arg(help_heading = INPUT, long, value_name = "PATH", conflicts_with_all = ["logic", "system", "calculus", "world", "tableau"])]
    pub(crate) dimacs: Option<PathBuf>,
    /// The format of the proof --dimacs checks: drat (text or binary) or
    /// lrat; by default lrat for a file ending in .lrat, else drat
    #[arg(help_heading = INPUT, long, value_enum, value_name = "FORMAT", requires = "dimacs")]
    pub(crate) proof_format: Option<ProofFormatArg>,
    /// With a derivation in a calculus given as data, the calculus to
    /// check it in: a rule file, or one built in (LK, LJ, LM, LL,
    /// tableau, and natural deduction's NJ, NK, NM, NK-DNE, NK-LEM,
    /// NJ-defined, NK-defined, NM-defined)
    ///
    /// The file is then such a derivation in JSON, which names its
    /// calculus; one that names a built-in calculus is checked in it
    /// without the flag. A file named like a built-in calculus is given
    /// with its path, ./LK. The verdict reads "valid in the calculus
    /// given": the derivation keeps to the rules, which nothing checks
    /// for soundness.
    #[arg(help_heading = INPUT, long, value_name = "CALCULUS")]
    pub(crate) calculus: Option<PathBuf>,
    /// With a derivation in a calculus given as data, how it is written:
    /// as a proof tree, or as a Fitch proof (view.layout)
    ///
    /// As a tree, text lists the derivation's nodes, and latex, typst and
    /// svg draw it with the conclusion at the root; in natural deduction a
    /// hypothesis stands alone at a leaf, the ones a rule discharges in
    /// brackets with a number, which the rule's name carries. As a Fitch
    /// proof, every format writes numbered lines, each with its rule and
    /// the lines it cites, a subproof's lines together under its
    /// hypotheses; latex uses the package fitch and typst the package
    /// fitchcraft. A derivation whose lines cannot be put in such an
    /// order is left out with the reason, and the verdict stands.
    #[arg(help_heading = OUTPUT, long, value_enum, value_name = "LAYOUT", default_value_t = LayoutArg::Tree)]
    pub(crate) layout: LayoutArg,
    /// Where to write the result.
    #[command(flatten)]
    pub(crate) output: OutputArgs,
    /// Read the settings from a JSON file; the flags override it
    ///
    /// The file is an object with any of the keys `clock`, `limits`,
    /// `search`, `view`, `styles`, `batch`, `ordinary`, `interactive`,
    /// `input`, `cut` and `lambda`. Each holds the fields that the flags
    /// name in parentheses, such as `{"search":
    /// {"copies": 3}, "limits": {"memory_bytes": null}}`. A field left out
    /// keeps the command's default, and a misspelt one is an error.
    /// `ordinary` applies under --logic, which names the logic.
    #[arg(long, value_name = "PATH")]
    pub(crate) settings: Option<PathBuf>,
}

impl CheckArgs {
    /// Returns the proof file, from `--file` or the argument, or `None`
    /// for standard input.
    pub(crate) fn source(&self) -> Option<PathBuf> {
        self.file.clone().or_else(|| self.proof.clone())
    }
}

/// The arguments of `grade`.
#[derive(Args, Clone, Debug)]
pub(crate) struct GradeArgs {
    /// The problem's sequent.
    #[command(flatten)]
    pub(crate) input: SequentInput,
    /// The answer to grade, a file; standard input when absent
    #[arg(help_heading = INPUT, long, value_name = "PATH")]
    pub(crate) answer: Option<PathBuf>,
    /// The linear logic the problem is in.
    #[command(flatten)]
    pub(crate) mode: ModeArgs,
    /// The problem is a sequent of this ordinary logic
    #[arg(help_heading = LOGIC, long, value_enum, value_name = "LOGIC")]
    pub(crate) logic: Option<LogicArg>,
    /// The problem is a sequent of this modal system: K, D, T, B, S4, S5,
    /// or K or D followed by axiom letters, such as KB4
    #[arg(help_heading = LOGIC, long, value_name = "SYSTEM", value_parser = parse_system, conflicts_with = "logic")]
    pub(crate) system: Option<deduxit::modal::System>,
    /// The problem is a sequent to derive in this calculus given as data: a
    /// rule file, or one built in (LK, LJ, LM, LL, tableau, NJ, NK, NM, …)
    #[arg(help_heading = LOGIC, long, value_name = "CALCULUS", conflicts_with_all = ["logic", "system", "intuitionistic", "affine", "mix"])]
    pub(crate) calculus: Option<PathBuf>,
    /// With --calculus, a sequent the derivation may assume; repeat it for
    /// more
    #[arg(help_heading = LOGIC, long, value_name = "SEQUENT", requires = "calculus")]
    pub(crate) assume: Vec<String>,
    /// The output format: text, or json for the verdict as one object
    #[arg(help_heading = OUTPUT, long, value_enum, value_name = "FORMAT", default_value_t = AnswerFormat::Text)]
    pub(crate) format: AnswerFormat,
    /// Give up after this long, such as 500ms, 10s, 2m or 1h, or `none`
    /// (clock.time_limit_ms)
    ///
    /// The check counts against it, from the start of the command. When
    /// the time runs out, the verdict is unknown (exit status 3).
    #[arg(help_heading = LIMITS, long, value_name = "DURATION", value_parser = parse_time, default_value_t = Time(Some(DEFAULT_TIMEOUT)))]
    pub(crate) timeout: Time,
    /// The most memory the check may hold: a number of bytes with a unit
    /// such as 512MiB or 4GiB, or `none` for no limit
    /// (limits.memory_bytes)
    #[arg(help_heading = LIMITS, long, value_name = "SIZE", value_parser = parse_bytes, default_value_t = Bytes(Some(Limits::DEFAULT_MEMORY_BYTES)))]
    pub(crate) memory_limit: Bytes,
    /// The most units of work the check may do, or `none` (limits.work)
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(None))]
    pub(crate) work_limit: Count<u64>,
    /// Read the settings from a JSON file; the flags override it
    ///
    /// The file is an object with any of the keys `clock`, `limits`,
    /// `search`, `view`, `styles`, `batch`, `ordinary`, `interactive`,
    /// `input`, `cut` and `lambda`. Each holds the fields that the flags
    /// name in parentheses, such as `{"limits": {"work": 100000}}`. A field
    /// left out keeps the command's default, and a misspelt one is an
    /// error.
    #[arg(long, value_name = "PATH")]
    pub(crate) settings: Option<PathBuf>,
}

/// The arguments of `eliminate`.
#[derive(Args, Debug)]
pub(crate) struct EliminateArgs {
    /// The most steps to take, or `none` (cut.steps)
    ///
    /// Past it, the proof reached is printed with the cuts it still has.
    /// The exit status is then 0 for a bound given here or in a settings
    /// file, and 3 for the default, which only keeps the call bounded.
    #[arg(long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(Some(cut::Options::DEFAULT_STEPS)))]
    pub(crate) steps: Count<u64>,
    /// The most nodes the proof a step makes may have (with --net, the
    /// vertices of the net), or `none` (cut.nodes)
    ///
    /// The exponentials can make a proof grow beyond any practical size. A
    /// step that would pass the limit is not taken, and the proof reached
    /// is printed (exit status 3).
    #[arg(long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(Some(cut::Options::DEFAULT_NODES)))]
    pub(crate) node_limit: Count<u64>,
    /// With --net and a drawing format, the most bytes the steps' drawings
    /// take together, or `none` (cut.drawn_bytes)
    ///
    /// Past it the steps are still taken and printed, but not drawn, and a
    /// line on standard error says from which step. FILE still holds the
    /// net reached.
    #[arg(long, value_name = "SIZE", value_parser = parse_bytes, default_value_t = Bytes(Some(cut::Options::DEFAULT_DRAWN_BYTES)))]
    pub(crate) drawings_limit: Bytes,
    /// Take the first step on the cut at this node, its index in the file's
    /// `nodes`, and the other steps as usual
    ///
    /// The checker's messages and the record's `cut` number the nodes the
    /// same way.
    #[arg(long, value_name = "NODE", conflicts_with = "net")]
    pub(crate) at: Option<u32>,
    /// The bounds, the mode and the output, as `check` takes them.
    #[command(flatten)]
    pub(crate) check: CheckArgs,
}

/// The subcommands of `seq`.
#[derive(Subcommand, Debug)]
pub(crate) enum SeqCommand {
    /// Print a sequent one-sided, in negation normal form, or two-sided
    ///
    /// `A, A -o B |- B` prints as `⊢ ~A, A ⊗ ~B, B`: hypotheses are negated
    /// onto the right, implications become pars, and negation is pushed down
    /// to the atoms. With --intuitionistic it prints as `A, A ⊸ B ⊢ B`
    /// again, which requires an intuitionistic sequent: one formula on the
    /// right, and pars only as implications.
    #[command(after_help = ORDINARY)]
    Print {
        /// The sequent.
        #[command(flatten)]
        input: SequentInput,
        /// Print the sequent two-sided, as intuitionistic linear logic
        /// reads it
        #[arg(short, long)]
        intuitionistic: bool,
        /// Read the text as cyclic linear logic does: the dual of a
        /// product reversed, ~(A * B) as ~B par ~A, and the formulas left
        /// of ⊢ in the reverse order
        #[arg(long, conflicts_with = "intuitionistic")]
        cyclic: bool,
        /// Read the text as --cyclic does and print the sequent two-sided,
        /// as the Lambek calculus reads it: with its divisions A \ B and
        /// B / A and its hypotheses in their order
        #[arg(long, conflicts_with_all = ["intuitionistic", "cyclic"])]
        lambek: bool,
        /// The ordinary logic the input is in, if any.
        #[command(flatten)]
        logic: LogicArgs,
        /// The output format
        #[arg(long, value_enum, value_name = "FORMAT", default_value_t = SequentFormat::Text)]
        format: SequentFormat,
        /// Write a document that compiles on its own instead of a fragment
        /// to paste (latex and typst)
        #[arg(long)]
        standalone: bool,
        /// Write to this file instead of standard output
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
        /// How the output looks.
        #[command(flatten)]
        style: StyleArgs,
        /// The most memory a png or pdf render may take, by its estimate: a
        /// number of bytes with a unit, such as 512MiB or 4GiB, or `none` for
        /// no limit; see `prove --memory-limit`
        #[arg(long, value_name = "SIZE", value_parser = parse_bytes, default_value_t = Bytes(Some(Limits::DEFAULT_MEMORY_BYTES)))]
        memory_limit: Bytes,
        /// Read the settings from a JSON file; the flags override it
        ///
        /// The file is an object with any of the keys `clock`, `limits`,
        /// `search`, `view`, `styles`, `batch`, `ordinary`, `interactive`,
        /// `input`, `cut` and `lambda`. Each holds the fields that the
        /// flags name in parentheses, such as
        /// `{"search": {"copies": 3}, "limits": {"memory_bytes": null}}`. A
        /// field left out keeps the command's default, and a misspelt one
        /// is an error. `ordinary` applies under --logic, which names the
        /// logic.
        #[arg(long, value_name = "PATH")]
        settings: Option<PathBuf>,
    },
    /// Translate a classical sequent into intuitionistic or minimal logic,
    /// and check that its verdict stays
    ///
    /// A negative translation turns a sequent of classical logic into one
    /// that a weaker logic proves exactly when classical logic proves the
    /// first. kolmogorov puts ¬¬ before every subformula; godel-gentzen
    /// before every atom, and writes A ∨ B as ¬(¬A ∧ ¬B); both reach
    /// minimal logic. kuroda puts ¬¬ before every formula of the sequent,
    /// and glivenko before the formula right of ⊢ alone; both reach
    /// intuitionistic logic. Several formulas right of ⊢ are translated as
    /// their disjunction.
    ///
    /// The command prints the translation, then decides the sequent in
    /// classical logic and its translation in the target logic, and prints
    /// both verdicts, with the countermodel of a sequent that is not
    /// valid. Verdicts that differ would be a defect of deduxit, and are an
    /// error. The exit status is 0 where both are valid, 1 where neither
    /// is, 3 for unknown (a limit stopped a search) and 2 for an error.
    /// With `--format json` the answer is the record `{"version",
    /// "deduxit", "translation", "target", "agreement", "sequent",
    /// "translated", "classical", "decided"}`, both outcomes with their
    /// evidence.
    #[command(after_help = ORDINARY)]
    Translate(TranslateArgs),
    /// Rewrite a classical sequent into a normal form step by step, and
    /// check that it keeps what the form promises
    ///
    /// nnf is negation normal form: no → or ↔, and ¬ before atoms alone.
    /// cnf (the default) is conjunctive normal form, a conjunction of
    /// clauses (disjunctions of atoms and negated atoms), and dnf
    /// disjunctive normal form, a disjunction of conjunctions; both are
    /// made by distribution. These three rewrite each formula of the
    /// sequent by equivalence laws, one law per step, and each formula
    /// stays equivalent to its normal form.
    ///
    /// tseitin and plaisted-greenbaum turn the sequent into clauses that
    /// can all be true exactly when the sequent can be false: each formula
    /// right of ⊢ moves left negated, then a new atom (t1, t2, …) names
    /// each compound subformula, innermost first, with the clauses of its
    /// definition. plaisted-greenbaum keeps only the direction of each
    /// definition that the subformula's place needs. The result `C1, …, Cn
    /// ⊢` is valid exactly when the sequent is.
    ///
    /// The command prints the normal form, each step with its law and the
    /// sequent after it (for an encoding, the clauses each new atom adds),
    /// then the check: deduxit decides, by the SAT solver, that each
    /// formula is equivalent to its normal form, or that the input with the
    /// definitions implies the clauses and the clauses imply the input. A
    /// check that fails would be a defect of deduxit, and is an error. The
    /// exit status is 0 where the check passes, 3 where a limit stopped it
    /// and 2 for an error, a form past --form-limit included. With
    /// `--format json` the answer is the record `{"version", "deduxit",
    /// "form", "promise", "checked", "sequent", "result", "steps",
    /// "checks"}`, the steps' numbers indices into the result's formulas.
    #[command(after_help = ORDINARY)]
    Normalize(NormalizeArgs),
    /// Print a sequent as JSON, the form `--input-format json` reads
    #[command(after_help = SYNTAX)]
    Json {
        /// The sequent.
        #[command(flatten)]
        input: SequentInput,
        /// Merge equal subformulas, drop unreferenced ones and sort the
        /// formulas of a JSON input, as parsing text always does
        #[arg(long)]
        optimize: bool,
        /// Read the text as cyclic linear logic does: the dual of a
        /// product reversed, ~(A * B) as ~B par ~A, and the formulas left
        /// of ⊢ in the reverse order
        #[arg(long)]
        cyclic: bool,
        /// Print the intuitionistic form: the connectives of intuitionistic
        /// linear logic, the hypotheses and the goal, which --intuitionistic
        /// also reads
        #[arg(short, long, conflicts_with_all = ["cyclic", "optimize"])]
        intuitionistic: bool,
        /// Write to this file instead of standard output
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
        /// The most bytes of input read: a number of bytes with a unit such
        /// as 512MiB or 4GiB, or `none` for no limit
        #[arg(long, value_name = "SIZE", value_parser = parse_bytes, default_value_t = Bytes(Some(Limits::DEFAULT_MEMORY_BYTES)))]
        memory_limit: Bytes,
    },
    /// Print the smallest fragment that holds a sequent: MLL, MLL with
    /// units, ALL, MALL, MELL or LL, or with --intuitionistic IMLL, IMLL
    /// with units, IALL, IMALL, IMELL or ILL
    #[command(after_help = SYNTAX)]
    Fragment {
        /// The sequent.
        #[command(flatten)]
        input: SequentInput,
        /// Name the intuitionistic fragment
        #[arg(short, long)]
        intuitionistic: bool,
        /// The most bytes of input read: a number of bytes with a unit such
        /// as 512MiB or 4GiB, or `none` for no limit
        #[arg(long, value_name = "SIZE", value_parser = parse_bytes, default_value_t = Bytes(Some(Limits::DEFAULT_MEMORY_BYTES)))]
        memory_limit: Bytes,
    },
}

/// What `seq translate` and `interpolate` share: the sequent, the
/// output and the limits.
#[derive(Args, Clone, Debug)]
pub(crate) struct TransformArgs {
    /// The sequent, of ordinary logic.
    #[command(flatten)]
    pub(crate) input: SequentInput,
    /// The output format: text, or json for the whole answer with every
    /// sequent, outcome and derivation
    #[arg(help_heading = OUTPUT, long, value_enum, value_name = "FORMAT", default_value_t = AnswerFormat::Text)]
    pub(crate) format: AnswerFormat,
    /// Write to this file instead of standard output
    #[arg(help_heading = OUTPUT, short, long, value_name = "PATH")]
    pub(crate) output: Option<PathBuf>,
    /// Give up after this long, such as 500ms, 10s, 2m or 1h, or `none`
    /// (clock.time_limit_ms)
    ///
    /// Every search the command runs counts against it, from the start of
    /// the command. When the time runs out, the verdict is unknown (exit
    /// status 3).
    #[arg(help_heading = LIMITS, long, value_name = "DURATION", value_parser = parse_time, default_value_t = Time(Some(DEFAULT_TIMEOUT)))]
    pub(crate) timeout: Time,
    /// The most memory a search may hold: a number of bytes with a unit
    /// such as 512MiB or 4GiB, or `none` for no limit; see `prove
    /// --memory-limit` (limits.memory_bytes)
    #[arg(help_heading = LIMITS, long, value_name = "SIZE", value_parser = parse_bytes, default_value_t = Bytes(Some(Limits::DEFAULT_MEMORY_BYTES)))]
    pub(crate) memory_limit: Bytes,
    /// The most units of work each search may do, or `none`; see `prove
    /// --work-limit` (limits.work)
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(None))]
    pub(crate) work_limit: Count<u64>,
    /// Read the settings from a JSON file; the flags override it
    ///
    /// The file is an object with any of the keys `clock`, `limits`,
    /// `search`, `view`, `styles`, `batch`, `ordinary`, `interactive`,
    /// `input`, `cut` and `lambda`. Each holds the fields that the flags
    /// name in parentheses, such as `{"limits": {"work": 100000}}`. A field
    /// left out keeps the command's default, and a misspelt one is an
    /// error.
    #[arg(long, value_name = "PATH")]
    pub(crate) settings: Option<PathBuf>,
}

/// The format of an answer that is no drawing: text or JSON.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AnswerFormat {
    /// Lines of text
    Text,
    /// One JSON document
    Json,
}

/// The arguments of `seq translate`.
#[derive(Args, Clone, Debug)]
pub(crate) struct TranslateArgs {
    /// The sequent, the output and the limits.
    #[command(flatten)]
    pub(crate) common: TransformArgs,
    /// The negative translation
    #[arg(help_heading = LOGIC, long, value_enum, value_name = "TRANSLATION", default_value_t = NegativeArg::Kolmogorov)]
    pub(crate) by: NegativeArg,
    /// The logic to translate into: minimal or intuitionistic; by default
    /// the weakest logic the translation reaches, minimal logic for
    /// kolmogorov and godel-gentzen, intuitionistic logic for kuroda and
    /// glivenko
    #[arg(help_heading = LOGIC, long, value_enum, value_name = "LOGIC")]
    pub(crate) to: Option<LogicArg>,
}

/// The arguments of `seq normalize`.
#[derive(Args, Clone, Debug)]
pub(crate) struct NormalizeArgs {
    /// The sequent, the output and the limits.
    #[command(flatten)]
    pub(crate) common: TransformArgs,
    /// The normal form
    #[arg(help_heading = LOGIC, long, value_enum, value_name = "FORM", default_value_t = FormArg::Cnf)]
    pub(crate) to: FormArg,
    /// The most subformula occurrences the normal form, and every sequent
    /// on the way to it, may have, or `none` (limits.form_occurrences)
    ///
    /// Distribution can make a formula exponentially longer, so a form past
    /// the limit is refused (exit status 2) unless this is raised.
    #[arg(help_heading = LIMITS, long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(Some(Limits::DEFAULT_FORM_OCCURRENCES)))]
    pub(crate) form_limit: Count<u64>,
}

/// A normal form of classical formulas.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FormArg {
    /// Negation normal form: ∧, ∨ and ¬ before atoms
    Nnf,
    /// Conjunctive normal form by distribution: a conjunction of clauses
    Cnf,
    /// Disjunctive normal form by distribution: a disjunction of
    /// conjunctions of literals
    Dnf,
    /// Tseitin's clauses of the sequent's negation, a new atom per compound
    /// subformula
    Tseitin,
    /// Plaisted and Greenbaum's clauses: Tseitin's, each definition in the
    /// direction its subformula's polarity needs
    PlaistedGreenbaum,
    /// Prenex normal form: every quantifier in front
    Prenex,
    /// Miniscope form: every quantifier moved as far in as it goes
    Miniscope,
    /// Skolem normal form: the sequent's negation, each ∃ replaced by a
    /// term of new function symbols
    Skolem,
    /// Herbrand normal form: the sequent, each strong quantifier replaced
    /// by a term of new function symbols
    Herbrand,
    /// Clausal normal form: the Skolem form's clauses, each closed by ∀s
    Clausal,
}

impl From<FormArg> for deduxit::ordinary::normal::Form {
    fn from(form: FormArg) -> Self {
        match form {
            FormArg::Nnf => Self::Nnf,
            FormArg::Cnf => Self::Cnf,
            FormArg::Dnf => Self::Dnf,
            FormArg::Tseitin => Self::Tseitin,
            FormArg::Prenex => Self::Prenex,
            FormArg::Miniscope => Self::Miniscope,
            FormArg::Skolem => Self::Skolem,
            FormArg::Herbrand => Self::Herbrand,
            FormArg::Clausal => Self::Clausal,
            FormArg::PlaistedGreenbaum => Self::PlaistedGreenbaum,
        }
    }
}

/// The arguments of `resolve`.
#[derive(Args, Clone, Debug)]
pub(crate) struct ResolveArgs {
    /// The sequent, the output and the limits.
    #[command(flatten)]
    pub(crate) common: TransformArgs,
    /// Read the clauses from this file in the DIMACS CNF format, or from
    /// standard input with `-`, in place of a sequent
    ///
    /// Comment lines start with c, the header is `p cnf VARIABLES
    /// CLAUSES`, and each clause is its literals, k for the variable k and
    /// -k for its negation, ended by 0. The variable k is the atom xk.
    #[arg(help_heading = INPUT, long, value_name = "PATH", conflicts_with_all = ["sequent", "file"])]
    pub(crate) dimacs: Option<PathBuf>,
    /// How to search
    #[arg(help_heading = SEARCH, long, value_enum, value_name = "METHOD", default_value_t = ResolutionArg::Saturation)]
    pub(crate) method: ResolutionArg,
    /// How a sequent's negation is turned into clauses: cnf, conjunctive
    /// normal form by distribution, or the encodings tseitin and
    /// plaisted-greenbaum (see `seq normalize`)
    ///
    /// The formulas right of ⊢ move left negated. The rewriting is checked
    /// as `seq normalize` checks it.
    #[arg(help_heading = INPUT, long, value_enum, value_name = "FORM", default_value_t = ClausesArg::Cnf, conflicts_with = "dimacs")]
    pub(crate) clauses: ClausesArg,
    /// The most clauses saturation keeps, the input's included, or `none`
    #[arg(help_heading = LIMITS, long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(Some(deduxit::ordinary::resolution::Options::DEFAULT_CLAUSES)))]
    pub(crate) clause_limit: Count<u64>,
    /// The most steps, resolvents made or steps of DPLL, or `none`
    #[arg(help_heading = LIMITS, long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(Some(deduxit::ordinary::resolution::Options::DEFAULT_STEPS)))]
    pub(crate) step_limit: Count<u64>,
}

/// How `resolve` searches.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ResolutionArg {
    /// Saturation by resolution, the shortest clause first
    Saturation,
    /// DPLL: unit clauses, pure literals, decisions and backtracking
    Dpll,
}

/// How a sequent's negation becomes clauses.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ClausesArg {
    /// Conjunctive normal form by distribution
    Cnf,
    /// Tseitin's encoding
    Tseitin,
    /// Plaisted and Greenbaum's encoding
    PlaistedGreenbaum,
}

/// The subcommands of `term`.
#[derive(Subcommand, Debug)]
pub(crate) enum TermCommand {
    /// Check that a lambda-term is well typed, and print its type
    ///
    /// The term is written as the command prints terms with
    /// --lambda-types: every λ says its variable's type, such as `\f:a ->
    /// b. \x:a. f x`, so a term has one type at most and nothing is
    /// inferred. Free variables are declared before ⊢, as in `x : a, f : a
    /// -> b |- f x`, and a type after `:` at the end is checked against
    /// the term's. The chapter "Proofs as programs" of the manual has the
    /// syntax.
    ///
    /// The command prints `well typed` and the judgement with every type,
    /// or `ill typed` and why, with exit status 1. With `--format json`
    /// the answer is the term's own document, then `deduxit`, `verdict`,
    /// `typing` (the type of every node) and `judgement`, so that it reads
    /// back as the term; or `{"version", "deduxit", "verdict": "ill_typed",
    /// "error"}`.
    Check(TermCheckArgs),
    /// Find a term of a type, or show that it has none
    ///
    /// The type is a formula of ordinary logic, such as `a -> b -> a`; the
    /// formulas left of ⊢, if any, are free variables the term may use.
    /// The formula is decided in intuitionistic logic, and the derivation
    /// found is read back as a term and checked. A formula that is not
    /// valid has no term. ¬A is the type A → ⊥.
    ///
    /// The exit status is 0 when the type has a term, 1 when it has none,
    /// and 3 when the time or a limit ran out first. With `--format json`
    /// the answer is `{"version", "deduxit", "verdict", "term"}`, verdict
    /// "inhabited", "empty" (with the `countermodel` where one was found)
    /// or "unknown" (with the `reason`).
    #[command(after_help = ORDINARY)]
    Inhabit(InhabitArgs),
    /// List the terms of a type and count them
    ///
    /// The terms are counted up to βη-equality, as long normal forms: a
    /// function's term is a λ, a pair's a pair, and an atom's a variable
    /// applied to arguments. The count is exact when the search met every
    /// term, "infinitely many" when a term can be put inside itself, as
    /// in the Church numerals of (a -> a) -> a -> a, and "at least" when
    /// --depth or a limit cut the search. The type is built from atoms, ->, /\, <->
    /// and true: with \/ or false, βη-equality makes terms equal that long
    /// normal forms tell apart, and the command refuses them.
    ///
    /// The exit status is 0 when the type has a term, 1 when it has none,
    /// and 3 when the search could not tell. With `--format json` the
    /// answer is `{"version", "deduxit", "count", "terms"}`, `count`
    /// `{"kind": "exactly", "count": n}`, `{"kind": "infinite"}` or
    /// `{"kind": "at_least", "count": n}`.
    #[command(after_help = ORDINARY)]
    Enumerate(EnumerateArgs),
}

/// The arguments of `term check`.
#[derive(Args, Clone, Debug)]
pub(crate) struct TermCheckArgs {
    /// The term, such as "\\x:a. x"; without it, the term is read from
    /// --file, or from standard input
    #[arg(value_name = "TERM", conflicts_with = "file")]
    pub(crate) term: Option<String>,
    /// Read the term from this file, or from standard input with `-`
    #[arg(help_heading = INPUT, short, long, value_name = "PATH")]
    pub(crate) file: Option<PathBuf>,
    /// The calculus: simple (any number of uses), linear (every variable
    /// used once) or affine (at most once)
    #[arg(help_heading = INPUT, long, value_enum, value_name = "CALCULUS", default_value_t = CalculusArg::Simple)]
    pub(crate) calculus: CalculusArg,
    /// The output format: text, or json for the term, its typing and the
    /// judgement
    #[arg(help_heading = OUTPUT, long, value_enum, value_name = "FORMAT", default_value_t = AnswerFormat::Text)]
    pub(crate) format: AnswerFormat,
    /// Write to this file instead of standard output
    #[arg(help_heading = OUTPUT, short, long, value_name = "PATH")]
    pub(crate) output: Option<PathBuf>,
    /// Read the settings from a JSON file
    ///
    /// The file is an object with any of the keys `clock`, `limits`,
    /// `search`, `view`, `styles`, `batch`, `ordinary`, `interactive`,
    /// `input`, `cut` and `lambda`. `lambda` picks the notation
    /// (lambda.notation) and `limits` bounds the term read and its check;
    /// the others are read and not used. A misspelt field is an error.
    #[arg(long, value_name = "PATH")]
    pub(crate) settings: Option<PathBuf>,
}

/// A lambda calculus.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CalculusArg {
    /// The simply typed lambda calculus
    Simple,
    /// The linear lambda calculus
    Linear,
    /// The affine lambda calculus
    Affine,
}

/// The arguments of `term inhabit`.
#[derive(Args, Clone, Debug)]
pub(crate) struct InhabitArgs {
    /// The type, a formula of ordinary logic, the output and the limits.
    #[command(flatten)]
    pub(crate) common: TransformArgs,
}

/// The arguments of `term enumerate`.
#[derive(Args, Clone, Debug)]
pub(crate) struct EnumerateArgs {
    /// The type, a formula of ordinary logic, the output and the limits.
    #[command(flatten)]
    pub(crate) common: TransformArgs,
    /// How deep a term may nest: variables applied inside each other's
    /// arguments
    #[arg(help_heading = SEARCH, long, value_name = "N", default_value_t = deduxit::lambda::InhabitOptions::DEFAULT_DEPTH)]
    pub(crate) depth: u32,
    /// The most terms listed; the count goes on past them
    #[arg(help_heading = SEARCH, long, value_name = "N", default_value_t = deduxit::lambda::InhabitOptions::DEFAULT_MAX_TERMS)]
    pub(crate) max_terms: u32,
}

/// The format of an outside solver's proof.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ProofFormatArg {
    /// DRAT, as text or binary
    Drat,
    /// LRAT, as text
    Lrat,
}

/// A negative translation of classical logic.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NegativeArg {
    /// Kolmogorov's: ¬¬ before every subformula, into minimal logic
    Kolmogorov,
    /// Gödel and Gentzen's: ¬¬ before every atom and A ∨ B as ¬(¬A ∧ ¬B),
    /// into minimal logic
    GodelGentzen,
    /// Kuroda's: ¬¬ before every formula of the sequent, into
    /// intuitionistic logic
    Kuroda,
    /// Glivenko's: ¬¬ before the formula right of ⊢, into intuitionistic
    /// logic
    Glivenko,
}

impl From<NegativeArg> for deduxit::ordinary::negative::Translation {
    fn from(translation: NegativeArg) -> Self {
        match translation {
            NegativeArg::Kolmogorov => Self::Kolmogorov,
            NegativeArg::GodelGentzen => Self::GodelGentzen,
            NegativeArg::Kuroda => Self::Kuroda,
            NegativeArg::Glivenko => Self::Glivenko,
        }
    }
}

/// The arguments of `interpolate`.
#[derive(Args, Clone, Debug)]
pub(crate) struct InterpolateArgs {
    /// The sequent, the output and the limits.
    #[command(flatten)]
    pub(crate) common: TransformArgs,
    /// The ordinary logic of the sequent
    #[arg(help_heading = LOGIC, long, value_enum, value_name = "LOGIC", default_value_t = LogicArg::Classical)]
    pub(crate) logic: LogicArg,
    /// The formulas of the first half, by position, separated by commas: L0
    /// is the first formula left of ⊢, R0 the first right of it; by default
    /// every formula left of ⊢
    ///
    /// In intuitionistic and minimal logic the formula right of ⊢ is always
    /// in the second half.
    #[arg(help_heading = INPUT, long, value_name = "POSITIONS", value_delimiter = ',', value_parser = parse_position)]
    pub(crate) first: Option<Vec<Position>>,
}

/// A formula of a sequent by its side and its position there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Position {
    /// Whether it is right of ⊢.
    pub(crate) right: bool,
    /// Its position on its side, from 0.
    pub(crate) at: usize,
}

/// Reads a position: `L` or `R` and a number, `L0` the first formula left
/// of ⊢.
fn parse_position(text: &str) -> Result<Position, String> {
    let wrong = || {
        format!(
            "{text:?} is no position: write L0 for the first formula left of ⊢, R0 for the first right of it"
        )
    };
    let (right, number) = match text.split_at_checked(1) {
        Some(("L" | "l", number)) => (false, number),
        Some(("R" | "r", number)) => (true, number),
        _ => return Err(wrong()),
    };
    let at = number.parse().map_err(|_| wrong())?;
    Ok(Position { right, at })
}

/// Where a sequent comes from.
#[derive(Args, Clone, Debug)]
pub(crate) struct SequentInput {
    /// The sequent, such as "A, A -o B |- B"; without it, the sequent is
    /// read from --file, or from standard input (but not by `interact`)
    #[arg(value_name = "SEQUENT", conflicts_with = "file")]
    pub(crate) sequent: Option<String>,
    /// Read the sequent from this file, or from standard input with `-`
    /// (not in `interact`, whose commands come from there)
    ///
    /// The file's extension gives its kind unless --input-format says
    /// otherwise. `.p` is an LLTP problem, or under --logic, where a
    /// command takes it, a TPTP problem of ordinary logic. `.spec` is a
    /// coverability problem and `.json` a JSON sequent. Anything else is
    /// the text syntax, which may span lines. `prove` takes the flag more
    /// than once, and a directory, as a batch.
    #[arg(help_heading = INPUT, short, long, value_name = "PATH", action = clap::ArgAction::Append)]
    pub(crate) file: Vec<PathBuf>,
    /// How to read the input, in place of the file's extension
    ///
    /// `lines`, `jsonl` and `problems` hold many sequents, which `prove`
    /// decides as a batch.
    #[arg(help_heading = INPUT, long, value_enum, value_name = "FORMAT", default_value_t = InputFormat::Auto)]
    pub(crate) input_format: InputFormat,
    /// The most subformula occurrences the sequent may have, or `none`
    /// (limits.occurrences)
    ///
    /// A sequent in JSON can share subformulas, so a small file can stand
    /// for a sequent of any size. A sequent past the limit is refused
    /// before it is unfolded. Each occurrence costs about 25 bytes before
    /// any search, and even `none` lets through at most about four billion.
    /// A text nested deeper than the limit, with more `(` or `~` open at
    /// once, is refused the same way.
    #[arg(help_heading = INPUT, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(Some(Limits::DEFAULT_OCCURRENCES)))]
    pub(crate) occurrence_limit: Count<u64>,
    /// The most first-order terms the atoms' arguments may unfold to, or
    /// `none` (limits.terms)
    ///
    /// A first-order sequent in JSON can share its terms, so a small file
    /// can name an atom of any length. A sequent past the limit is refused
    /// before it is unfolded, and so is a proof whose instance table has
    /// more entries.
    #[arg(help_heading = INPUT, hide_short_help = true, long, value_name = "N", value_parser = parse_count::<u64>, default_value_t = Count(Some(Limits::DEFAULT_TERMS)))]
    pub(crate) term_limit: Count<u64>,
    /// How the text of an intuitionistic sequent is read: in its own
    /// syntax alone, or in the classical one (input.intuitionistic)
    ///
    /// With --intuitionistic, write a sequent in the syntax of
    /// intuitionistic linear logic: A -o B, A |- B. A symbol of the
    /// classical syntax (~, ^, par, |, ?, bot) is refused, and the error
    /// says what to write instead. `one-sided` reads the classical syntax
    /// as the one-sided sequent it stands for: ~(A * ~B), A |- B as A ⊸ B,
    /// A ⊢ B.
    #[arg(help_heading = INPUT, long, value_enum, value_name = "SYNTAX", default_value_t = IllSyntaxArg::Strict)]
    pub(crate) ill_syntax: IllSyntaxArg,
}

/// How the text of an intuitionistic sequent is read.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IllSyntaxArg {
    /// The syntax of intuitionistic linear logic alone
    Strict,
    /// The classical syntax, read as the one-sided sequent it stands for
    OneSided,
}

impl From<IllSyntaxArg> for deduxit::linear::IllSyntax {
    /// The library's syntax of the flag's.
    fn from(syntax: IllSyntaxArg) -> Self {
        match syntax {
            IllSyntaxArg::Strict => Self::Strict,
            IllSyntaxArg::OneSided => Self::OneSided,
        }
    }
}

/// How an input is read.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InputFormat {
    /// By the file's extension: `.p` lltp (tptp under --logic, where a
    /// command takes it), `.spec` spec, `.json` json, else text
    Auto,
    /// One sequent in the text syntax, which may span lines
    Text,
    /// One sequent in JSON, as `seq json` writes it
    Json,
    /// A problem of the LLTP library: its axioms ⊢ its conjecture; the file
    /// does not say whether it is intuitionistic, the flags do
    Lltp,
    /// A coverability problem in the .spec format of the Mist tool: its
    /// rules under !, its initial marking ⊢ its target; it is decided in
    /// affine mode, with or without --affine
    Spec,
    /// One sequent per line in the text syntax, as `NAME: SEQUENT` or
    /// `SEQUENT`; blank lines and everything from `#` on are skipped
    Lines,
    /// One JSON value per line: a sequent as `seq json` writes it, or
    /// {"name": …, "mode": …, "sequent": …} with the sequent in JSON or as
    /// text and the mode named as in `problems`; blank lines are skipped
    Jsonl,
    /// A problem of ordinary propositional logic in TPTP syntax, as the
    /// ILTP library writes them: its axioms ⊢ its conjecture (with --logic,
    /// where a `.p` file is one)
    Tptp,
    /// The benchmark harness's problem files: `NAME; MODE; EXPECTED;
    /// COPIES; SEQUENT` per line, MODE one of classical, mix, affine,
    /// affine-mix, intuitionistic, intuitionistic-affine, cyclic, lambek
    /// or lambek-star (whose sequents are read as --cyclic reads them);
    /// EXPECTED and COPIES are not read
    Problems,
}

impl InputFormat {
    /// Returns the format of a file: this one, or for `Auto` the one its
    /// extension names, a `.p` file being a TPTP problem of ordinary logic
    /// when `ordinary` is set and an LLTP problem otherwise.
    #[must_use]
    pub(crate) fn of(self, path: &std::path::Path, ordinary: bool) -> Self {
        if self != Self::Auto {
            return self;
        }
        match path.extension().and_then(|e| e.to_str()) {
            Some("p") if ordinary => Self::Tptp,
            Some("p") => Self::Lltp,
            Some("spec") => Self::Spec,
            Some("json") => Self::Json,
            _ => Self::Text,
        }
    }

    /// Whether an input in the format holds many sequents.
    pub(crate) fn is_many(self) -> bool {
        matches!(self, Self::Lines | Self::Jsonl | Self::Problems)
    }
}

/// The ordinary logic an input is in, and the translation that decides
/// it.
#[derive(Args, Clone, Debug)]
pub(crate) struct LogicArgs {
    /// Read the input as ordinary logic
    ///
    /// The input is a formula or a sequent of propositional logic, in the
    /// syntax below; a `.p` file is a TPTP problem, as the ILTP library
    /// writes them. `prove` decides it and shows the proof as a derivation
    /// of LK (classical) or LJ (intuitionistic, minimal), checked by the
    /// rules of the logic before it is shown. By default a SAT solver, whose
    /// search always ends, decides every logic, and classical logic keeps
    /// the derivation its translation into affine MALL gives within a short
    /// bound; --method chooses. The translated sequent is called the
    /// image. Through the translation into ILL (--method embedding),
    /// intuitionistic and minimal logic may be answered unknown at the time
    /// limit. The JSON output is the outcome of the ordinary sequent, the
    /// image's outcome nested in it; --linear writes the image's alone.
    /// `seq print` prints the image. `interact` proves the sequent step by
    /// step by the rules of LK or LJ, and its `close` decides a goal
    /// through the translation.
    #[arg(help_heading = LOGIC, long, value_enum, value_name = "LOGIC")]
    pub(crate) logic: Option<LogicArg>,
    /// The translation into linear logic: affine for classical logic, cbn
    /// (by default), cbv or 01 for intuitionistic and minimal logic
    /// (ordinary.translation)
    #[arg(help_heading = LOGIC, long, value_enum, value_name = "TRANSLATION", requires = "logic")]
    pub(crate) translation: Option<TranslationArg>,
}

/// An ordinary logic.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LogicArg {
    /// Classical propositional logic, derivations in LK
    Classical,
    /// Intuitionistic propositional logic, derivations in LJ
    Intuitionistic,
    /// Minimal logic: intuitionistic logic without ex falso, where false is
    /// an atom
    Minimal,
}

impl From<LogicArg> for deduxit::ordinary::Logic {
    fn from(logic: LogicArg) -> Self {
        match logic {
            LogicArg::Classical => Self::Classical,
            LogicArg::Intuitionistic => Self::Intuitionistic,
            LogicArg::Minimal => Self::Minimal,
        }
    }
}

/// How a sequent of ordinary logic is decided.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MethodArg {
    /// Through its translation into linear logic, the proof read back as
    /// LK or LJ
    Embedding,
    /// By Dyckhoff's calculus LJT, whose search always ends, for
    /// intuitionistic and minimal logic
    Ljt,
    /// By a SAT solver, whose search always ends, for every logic; a valid
    /// classical sequent gets a checked clausal proof and no derivation
    Sat,
    /// By its truth table, for classical logic: every assignment of its
    /// atoms, within --table-atoms
    Table,
    /// By a signed tableau, for classical logic: a closed tableau is a
    /// checked derivation of LK, an open branch a checked countermodel
    Tableau,
    /// By a non-clausal connection prover, for classical and
    /// intuitionistic logic, first-order sequents included: a valid
    /// sequent gets a checked derivation of LK or LJ
    Connection,
    /// By grounding over the constants and the SAT solver, for first-order
    /// sequents without function symbols where no ∀ stands under an ∃ or
    /// every predicate takes at most one argument: a sequent that is not
    /// valid gets a checked finite structure
    Grounding,
    /// By a search for a finite structure that refutes it, of 1, 2, …
    /// elements, for first-order sequents in every logic: a structure found
    /// is checked; it never finds a sequent valid
    Models,
}

impl From<MethodArg> for deduxit::ordinary::Method {
    fn from(method: MethodArg) -> Self {
        match method {
            MethodArg::Embedding => Self::Embedding,
            MethodArg::Ljt => Self::Ljt,
            MethodArg::Sat => Self::Sat,
            MethodArg::Table => Self::Table,
            MethodArg::Tableau => Self::Tableau,
            MethodArg::Connection => Self::Connection,
            MethodArg::Grounding => Self::Grounding,
            MethodArg::Models => Self::Models,
        }
    }
}

/// A translation of ordinary logic into linear logic.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TranslationArg {
    /// Classical logic into affine MALL: negation normal form, ∧ as &, ∨
    /// as ⅋
    Affine,
    /// Girard's call-by-name translation into ILL: A → B as !A ⊸ B, A ∨ B
    /// as !A ⊕ !B, hypotheses under !
    Cbn,
    /// Girard's call-by-value translation into ILL: atoms as !a, A → B as
    /// !(A ⊸ B), ∧ as ⊗
    Cbv,
    /// Liang and Miller's 0/1 translation into ILL, as the LLTP library
    /// states it
    #[value(name = "01")]
    ZeroOne,
}

impl From<TranslationArg> for deduxit::ordinary::Translation {
    fn from(translation: TranslationArg) -> Self {
        match translation {
            TranslationArg::Affine => Self::Affine,
            TranslationArg::Cbn => Self::CallByName,
            TranslationArg::Cbv => Self::CallByValue,
            TranslationArg::ZeroOne => Self::ZeroOne,
        }
    }
}

/// The logic a sequent is proved in.
#[derive(Args, Clone, Debug)]
pub(crate) struct ModeArgs {
    /// Intuitionistic linear logic: hypotheses left of ⊢ and one goal
    /// right of it, written in its own syntax (--ill-syntax), and
    /// two-sided derivations
    #[arg(help_heading = LOGIC, short, long)]
    pub(crate) intuitionistic: bool,
    /// Affine logic: a hypothesis may go unused (weakening)
    #[arg(help_heading = LOGIC, short, long)]
    pub(crate) affine: bool,
    /// Allow the Mix rule, which proves ⊢ Γ, Δ from ⊢ Γ and ⊢ Δ
    #[arg(help_heading = LOGIC, long)]
    pub(crate) mix: bool,
    /// Cyclic linear logic: the order of the formulas counts, up to
    /// rotation (unit-free MLL)
    ///
    /// There is no exchange rule. The text is read with the dual of a
    /// product reversed, ~(A * B) as ~B par ~A, and the formulas left of ⊢
    /// in the reverse order.
    #[arg(help_heading = LOGIC, long, conflicts_with = "intuitionistic")]
    pub(crate) cyclic: bool,
    /// The Lambek calculus: intuitionistic, in the order written, with the
    /// divisions A \ B and B / A beside A * B (unit-free)
    ///
    /// Every sequent has a formula left of ⊢, unless --empty-antecedents
    /// is given. The text is read as --cyclic reads it.
    #[arg(help_heading = LOGIC, long, conflicts_with_all = ["intuitionistic", "cyclic"])]
    pub(crate) lambek: bool,
    /// With --lambek, allow sequents with no formula left of ⊢ (the
    /// calculus L*)
    #[arg(help_heading = LOGIC, long, requires = "lambek")]
    pub(crate) empty_antecedents: bool,
}

impl ModeArgs {
    /// Returns the mode the flags ask for.
    pub(crate) fn mode(&self) -> Mode {
        let mut mode = if self.intuitionistic {
            Mode::INTUITIONISTIC
        } else if self.cyclic {
            Mode::CYCLIC
        } else if self.lambek {
            Mode::LAMBEK
        } else {
            Mode::CLASSICAL
        };
        if self.empty_antecedents {
            mode = mode.with_empty_antecedents();
        }
        if self.affine {
            mode = mode.with_affine();
        }
        if self.mix {
            mode = mode.with_mix();
        }
        mode
    }

    /// Returns whether any flag names a mode.
    pub(crate) const fn given(&self) -> bool {
        self.intuitionistic
            || self.affine
            || self.mix
            || self.cyclic
            || self.lambek
            || self.empty_antecedents
    }
}

/// Where and how a command writes its result.
#[derive(Args, Clone, Debug)]
pub(crate) struct OutputArgs {
    /// The output format; by default the one the `--output` file's
    /// extension names (.txt, .json, .tex, .typ, .svg, .png, .pdf, .v), else
    /// text
    #[arg(help_heading = OUTPUT, long, value_enum, value_name = "FORMAT")]
    pub(crate) format: Option<Format>,
    /// Write to this file instead of standard output
    ///
    /// The file is made only when it holds a derivation or a net, and
    /// always in JSON. Otherwise the verdict goes to standard error.
    #[arg(help_heading = OUTPUT, short, long, value_name = "PATH")]
    pub(crate) output: Option<PathBuf>,
    /// Write the proof net of the proof instead of its derivation, for MLL
    /// and MELL without units
    ///
    /// MLL may have Mix. MELL's net has boxes, classical with or without
    /// Mix, or intuitionistic (the net of the one-sided sequent). As text
    /// the net is the sequent, the copies of every ?, the axiom links as
    /// pairs of literals with their vertex numbers, and the verdict of the
    /// correctness criterion. As json it is the net's document, in place of
    /// the outcome. As svg, png or pdf it is drawn: the formula trees, the
    /// axiom links as arcs over the literals, and every !-box a rectangle
    /// around its contents.
    #[arg(help_heading = OUTPUT, long)]
    pub(crate) net: bool,
    /// Print only the verdict line, not the derivation; with --output,
    /// on standard error, since a file holds a derivation or nothing
    #[arg(help_heading = OUTPUT, short, long)]
    pub(crate) quiet: bool,
    /// Write a document that compiles on its own instead of a fragment to
    /// paste (latex, typst and rocq)
    #[arg(help_heading = OUTPUT, long)]
    pub(crate) standalone: bool,
    /// Leave out the verdict line, so that the output is the derivation
    /// alone; the exit status still gives the verdict
    #[arg(help_heading = OUTPUT, long)]
    pub(crate) no_verdict: bool,
    /// When to write the derivation, in every format
    ///
    /// By default the text tree is printed on a terminal only if it fits:
    /// no line wider than the terminal and no more lines than
    /// `--screens` screens. Otherwise one line says how large the tree is
    /// and how to get it. Into a file or a pipe, and in every other
    /// format, the derivation is written whatever its size, up to
    /// `--derivation-limit`; `never` writes the verdict alone.
    #[arg(help_heading = OUTPUT, long, value_enum, value_name = "WHEN", default_value_t = Tree::Auto)]
    pub(crate) tree: Tree,
    /// How many screens of lines a text tree may fill and still be
    /// printed on a terminal by `--tree auto`, or `none` for any number
    #[arg(help_heading = OUTPUT, long, value_name = "N", value_parser = parse_count::<u32>, default_value_t = Count(Some(SCREENS)))]
    pub(crate) screens: Count<u32>,
    /// The most characters of a sequent in a verdict line, and of a list
    /// of formulas in an error report, or `none` for no limit; a longer
    /// one is cut with `…` and the number of its formulas
    #[arg(help_heading = OUTPUT, long, value_name = "CHARS", value_parser = parse_count::<u32>, default_value_t = Count(Some(ABBREVIATE)))]
    pub(crate) abbreviate: Count<u32>,
    /// How the output looks.
    #[command(flatten)]
    pub(crate) style: StyleArgs,
    /// Print the proof's lambda-term: an intuitionistic proof read back as
    /// a program
    ///
    /// A proof of intuitionistic linear logic (-i, or --linear under
    /// --logic) gives a term of the linear lambda calculus of Benton,
    /// Bierman, de Paiva and Hyland, or of its affine variant under
    /// --affine. A derivation of LJ (--logic intuitionistic or minimal)
    /// gives a simply typed term. The term is printed as its typing
    /// judgement: on a line after the derivation, under the conclusion in
    /// latex, typst, svg, png and pdf, and under the key lambda in json. A
    /// proof that has none (classical, with Mix, cyclic or Lambek, with
    /// quantifiers) gets a line saying why.
    #[arg(help_heading = OUTPUT, long)]
    pub(crate) lambda: bool,
    /// How the lambda-term is printed.
    #[command(flatten)]
    pub(crate) lambda_style: LambdaArgs,
    /// The Rocq certificate's lemma and prelude.
    #[command(flatten)]
    pub(crate) rocq: RocqArgs,
    /// The largest derivation to build, by its estimated size: a number
    /// of bytes with a unit such as 64MiB or 2GiB, or `none` for no limit
    /// (limits.derivation_bytes)
    ///
    /// A derivation writes out the whole sequent at every inference, and
    /// repeats every subproof that the proof shares, so it can be larger
    /// than the proof by any factor. A derivation estimated above the limit
    /// is not built, in any format. The verdict is reported without it,
    /// with a line that says how large it is, and the exit status is the
    /// verdict's. `--format json` writes the proof itself at any size. The
    /// drawing of a proof net (`--net`) is estimated before it is drawn,
    /// and left out above the same limit. Under --logic the derivation of
    /// LK or LJ is built and checked within --memory-limit whatever its
    /// size, so the verdict is a checked one, and this limit bounds only
    /// its drawing (the tableau method builds its tableau within it);
    /// `prove --format json` writes the derivation at any size.
    #[arg(help_heading = LIMITS, hide_short_help = true, long, value_name = "SIZE", value_parser = parse_bytes, default_value_t = Bytes(Some(Limits::DEFAULT_DERIVATION_BYTES)))]
    pub(crate) derivation_limit: Bytes,
    /// When to draw a run of one structural rule, such as the weakenings of
    /// every unused `?` formula, as one inference labelled with a star
    /// (`?w*`); never in rocq (view.compact)
    #[arg(help_heading = OUTPUT, long, value_enum, value_name = "WHEN", default_value_t = CompactArg::Auto)]
    pub(crate) compact: CompactArg,
    /// How to draw a derivation of classical linear logic: one-sided, or
    /// two-sided as the sequent was written; rocq is always one-sided
    /// (view.classical)
    ///
    /// Two-sided, every sequent has its formulas on the sides of `⊢` they
    /// were written on, and the rules are named by side (`⊗L`, `⅋R`, `⊸L`,
    /// …). Where a negation was written, a step `~L` or `~R` moves the
    /// formula across `⊢`. A sequent given as JSON, LLTP or .spec is
    /// written one-sided, its antecedents on the left. Derivations of
    /// intuitionistic linear logic are always two-sided, and those of the
    /// cyclic mode always one-sided.
    #[arg(help_heading = OUTPUT, long, value_enum, value_name = "SIDES", default_value_t = ClassicalSidesArg::One)]
    pub(crate) classical_sides: ClassicalSidesArg,
}

/// How a derivation in a calculus given as data is written.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LayoutArg {
    /// A proof tree
    Tree,
    /// A Fitch proof: numbered lines, subproofs together
    Fitch,
}

impl From<LayoutArg> for deduxit::linear::proofs::Layout {
    /// Returns the library's value of the switch.
    fn from(layout: LayoutArg) -> Self {
        match layout {
            LayoutArg::Tree => Self::Tree,
            LayoutArg::Fitch => Self::Fitch,
        }
    }
}

/// How a derivation of classical linear logic is drawn.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ClassicalSidesArg {
    /// One-sided, every formula right of `⊢`
    One,
    /// Two-sided, as the sequent was written
    Two,
}

impl From<ClassicalSidesArg> for deduxit::linear::ClassicalSides {
    /// Returns the library's value of the switch.
    fn from(sides: ClassicalSidesArg) -> Self {
        match sides {
            ClassicalSidesArg::One => Self::One,
            ClassicalSidesArg::Two => Self::Two,
        }
    }
}

/// How a derivation of LK or LJ shows its weakenings and contractions.
#[derive(Args, Clone, Debug)]
pub(crate) struct StructuralArgs {
    /// How a derivation of LK or LJ (--logic) shows its weakenings and
    /// contractions; rocq always shows them all (view.structural)
    ///
    /// `admissible` leaves each one out where it is admissible, so that the
    /// derivation reads as G3's, whose axiom takes a context and whose
    /// rules contract what they use twice. `explicit` draws every one, as
    /// the checker has it. The derivation is the same either way, and its
    /// explicit form is the one checked. A certificate (--format rocq)
    /// names every structural rule.
    #[arg(help_heading = OUTPUT, long, value_enum, value_name = "RULES", default_value_t = StructuralArg::Admissible)]
    pub(crate) structural: StructuralArg,
}

/// How a proof's lambda-term is printed.
#[derive(Args, Clone, Debug)]
pub(crate) struct LambdaArgs {
    /// The symbols of a lambda-term: unicode, as the calculus writes it
    /// (λx. x ⊗ y, ⟨x, y⟩), or ascii (\x. x * y, <x, y>), the types as the
    /// parser reads them (lambda.notation)
    #[arg(help_heading = OUTPUT, long, value_enum, value_name = "NOTATION", default_value_t = NotationArg::Unicode)]
    pub(crate) lambda_notation: NotationArg,
    /// Write every binder of a lambda-term with its variable's type,
    /// λx:A. M, and false and abort with their goal's (lambda.types)
    #[arg(help_heading = OUTPUT, long)]
    pub(crate) lambda_types: bool,
}

/// The symbols of a lambda-term.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NotationArg {
    /// λ, ⊗, ⟨⟩ and the other symbols of the calculus
    Unicode,
    /// ASCII alone
    Ascii,
}

impl From<NotationArg> for deduxit::lambda::Notation {
    /// Returns the library's notation.
    fn from(notation: NotationArg) -> Self {
        match notation {
            NotationArg::Unicode => Self::Unicode,
            NotationArg::Ascii => Self::Ascii,
        }
    }
}

/// How a derivation of LK or LJ shows its structural rules.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StructuralArg {
    /// Each weakening and contraction left out where it is admissible
    Admissible,
    /// Every weakening and contraction drawn
    Explicit,
}

impl From<StructuralArg> for deduxit::linear::proofs::Structural {
    /// Returns the library's value of the switch.
    fn from(structural: StructuralArg) -> Self {
        match structural {
            StructuralArg::Admissible => Self::Admissible,
            StructuralArg::Explicit => Self::Explicit,
        }
    }
}

/// When a derivation draws a run of one structural rule as one inference.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CompactArg {
    /// Where the whole derivation would pass `--derivation-limit` or, as
    /// a text tree on a terminal, not fit it
    Auto,
    /// Always
    Always,
    /// Never: every rule its own inference
    Never,
}

impl From<CompactArg> for Compact {
    /// Returns the library's value of the switch.
    fn from(compact: CompactArg) -> Self {
        match compact {
            CompactArg::Auto => Self::Auto,
            CompactArg::Always => Self::Always,
            CompactArg::Never => Self::Never,
        }
    }
}

/// When a command writes the derivation of a proof.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Tree {
    /// The text tree on a terminal if it fits, everything else always
    Auto,
    /// Whatever its size, up to `--derivation-limit`
    Always,
    /// Not at all
    Never,
}

/// How many screens of lines a text tree may fill on a terminal by
/// default.
pub(crate) const SCREENS: u32 = 3;

/// The most characters of a sequent in a verdict line by default: two
/// or three lines of a terminal.
pub(crate) const ABBREVIATE: u32 = 200;

/// How the outputs look: one options value per format, read from a file
/// and changed by flags.
#[derive(Args, Clone, Debug, Default)]
pub(crate) struct StyleArgs {
    /// Set an option of an output format, such as `labels=subscript`,
    /// `latex.open=bare` or `svg.font.family=monospace` (repeatable)
    /// (styles.FORMAT.KEY)
    ///
    /// KEY is a field of the format's options, dotted into nested ones, as
    /// `--style-file` holds them. Without a prefix (`text.`, `latex.`,
    /// `typst.`, `svg.`, `png.`, `pdf.` or `rocq.`) it names a field of the
    /// format `--format` gives. VALUE is JSON, or else a string: `gap=5`,
    /// `ids=true`, `open={"mark":"?"}`, `labels.table.⊸L=⊸_L`. `null` lifts
    /// a bound (`pixels=null`).
    ///
    /// text: labels, open, bar, gap.
    ///
    /// latex: form, labels, open, align, ebproof, preamble, lambda. lambda
    /// is the lambda-term under the conclusion, null or
    /// {"notation","types"}, which --lambda sets.
    ///
    /// typst: form, labels, open, import, page, layout, premise_gap,
    /// label_gap, band, stroke, lambda. layout is auto, curryst or deduxit:
    /// curryst for a tree up to nine inferences high, deduxit's own code
    /// above. premise_gap, label_gap, band and stroke are lengths of
    /// deduxit's own layout: a number and pt, mm, cm, in or em.
    ///
    /// svg: font (family, advances), labels, open, ids, description,
    /// font_size, label_size, line_height, premise_gap, literal_gap,
    /// label_gap, margin, stroke_width, link_height, link_cap, node_radius,
    /// text, line, par, link, highlight, background, essential, input,
    /// dominator, lambda, net, net_look. net is formulas, a proof net with
    /// every subformula written, or compact, with only its literals and
    /// conclusions written and its inner links drawn as circles. net_look
    /// is a net's look: textbook, or deduxit (the default), or its fields.
    /// Its field premises is dashed or plain, for the edges into a ⅋ or ?
    /// node. conclusions is raised or bottom: a ! with auxiliary doors
    /// written a row above the ? nodes, or on the bottom row. why_not is
    /// node or rules: a ? node drawn as one node, or as the rules'
    /// dereliction, contraction and weakening nodes.
    ///
    /// png: scale, pixels. pdf: embed_text, compatible, accessible, title,
    /// language, date. rocq: form, lemma, prelude, kernel, where the
    /// command writes Rocq. fitch: hypothesis, open, the justification a
    /// Fitch proof writes beside a hypothesis and an open line, in every
    /// format (check --calculus --layout fitch).
    ///
    /// labels is upright, subscript, off or {"table":{RULE:LABEL}}.
    /// `labels.table.RULE=LABEL` sets one rule's label and leaves the
    /// others upright. RULE is ax, ⊗, ⅋, 1, ⊥, &, ⊕₁, ⊕₂, ⊤, !, ?d, ?c, ?w,
    /// mix, wk, cut, ∀, ∃, ⊸L, ⊸R, ⊗L, ⊗R, &L₁, &L₂, &R, ⊕L, ⊕R₁, ⊕R₂, 1L,
    /// 1R, 0L, ⊤R, !L, !R, !c, !w, ∀R, ∃L, ∃R, ∀L, \L, \R, /L or /R. open,
    /// the mark of an open goal, is dots, bare, dashed or {"mark":TEXT}.
    #[arg(help_heading = OUTPUT, long = "style", value_name = "KEY=VALUE")]
    pub(crate) style: Vec<String>,
    /// Read the output formats' options from a JSON file (styles)
    ///
    /// The file is an object with a key per format: text, latex, typst,
    /// svg, png, pdf, and rocq where the command writes Rocq. Each holds the
    /// fields `--style` names. A field left out keeps its default.
    #[arg(help_heading = OUTPUT, long, value_name = "PATH")]
    pub(crate) style_file: Option<PathBuf>,
    /// The math font of a drawing (SVG, PNG, PDF); by default Euler Math
    /// (styles.svg.font)
    #[arg(help_heading = OUTPUT, long, value_enum, value_name = "FONT")]
    pub(crate) font: Option<FontArg>,
}

/// A math font the command embeds, for its drawings.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FontArg {
    /// Euler Math 0.75, upright, after Hermann Zapf's Euler for the AMS
    Euler,
    /// STIX Two Math 2.12, in the style of Times
    StixTwo,
    /// Garamond-Math 2022-01-03, matching EB Garamond
    Garamond,
    /// Asana Math 0.962, in the style of Palatino
    Asana,
    /// New Computer Modern Math 7.1.1 (Book), the Computer Modern of TeX
    NewComputerModern,
}

impl From<FontArg> for MathFont {
    /// Returns the library's math font.
    fn from(font: FontArg) -> Self {
        match font {
            FontArg::Euler => Self::Euler,
            FontArg::StixTwo => Self::StixTwo,
            FontArg::Garamond => Self::Garamond,
            FontArg::Asana => Self::Asana,
            FontArg::NewComputerModern => Self::NewComputerModern,
        }
    }
}

/// The Rocq certificate's flags, on the commands that write one.
#[derive(Args, Clone, Debug, Default)]
pub(crate) struct RocqArgs {
    /// The Rocq kernel to write a certificate of linear logic for; by
    /// default nanoyalla for classical linear logic, and deduxit for Mix,
    /// affine and intuitionistic logic and a sequent with a quantifier
    /// (styles.rocq.kernel)
    #[arg(help_heading = OUTPUT, long, value_enum, value_name = "KERNEL")]
    pub(crate) kernel: Option<KernelArg>,
    /// The name of the lemma of a Rocq certificate (styles.rocq.lemma)
    #[arg(help_heading = OUTPUT, long, value_name = "NAME")]
    pub(crate) lemma: Option<String>,
    /// The lines a standalone Rocq file starts with, before the lemma, in
    /// place of the certificate's own import (styles.rocq.prelude)
    #[arg(help_heading = OUTPUT, long, value_name = "TEXT")]
    pub(crate) prelude: Option<String>,
}

/// The Rocq kernel a certificate is written for.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum KernelArg {
    /// NanoYalla 1.1.3, the kernel of Click & coLLecT: classical linear
    /// logic one-sided, a proof script checked rule by rule
    Nanoyalla,
    /// deduxit's own Rocq library (the package rocq-deduxit, from
    /// github.com/deduxit/rocq-deduxit): every mode in its own calculus, intuitionistic
    /// logic two-sided, a proof term checked by the library's checker
    Deduxit,
}

impl From<KernelArg> for Kernel {
    /// Returns the library's kernel.
    fn from(kernel: KernelArg) -> Self {
        match kernel {
            KernelArg::Nanoyalla => Self::NanoYalla,
            KernelArg::Deduxit => Self::Deduxit,
        }
    }
}

/// A number of bytes, or none for no limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Bytes(pub Option<u64>);

impl std::fmt::Display for Bytes {
    /// Writes the bytes as a size flag reads them, in the largest unit
    /// that divides them.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => f.write_str("none"),
            Some(bytes) => {
                let (mut value, mut unit) = (bytes, 0);
                while unit + 1 < UNITS.len() && value >= 1024 && value.is_multiple_of(1024) {
                    (value, unit) = (value / 1024, unit + 1);
                }
                write!(f, "{value}{}", UNITS[unit])
            }
        }
    }
}

/// A number, or none for no bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Count<T>(pub Option<T>);

impl<T: std::fmt::Display> std::fmt::Display for Count<T> {
    /// Writes the number as a flag reads it.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => f.write_str("none"),
            Some(n) => write!(f, "{n}"),
        }
    }
}

/// Reads a modal system by any of its names.
fn parse_system(text: &str) -> Result<deduxit::modal::System, String> {
    text.parse()
        .map_err(|error: deduxit::Error| error.to_string())
}

/// Parses a number, or `none`.
fn parse_count<T: std::str::FromStr>(text: &str) -> Result<Count<T>, String> {
    if text == "none" {
        return Ok(Count(None));
    }
    text.parse()
        .map(|n| Count(Some(n)))
        .map_err(|_| format!("{text:?} is neither a number nor `none`"))
}

/// A time limit, or none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Time(pub Option<Duration>);

impl std::fmt::Display for Time {
    /// Writes the limit as `--timeout` reads it.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => f.write_str("none"),
            Some(t) => write!(f, "{t:?}"),
        }
    }
}

/// Parses a time limit: a duration, or `none`.
fn parse_time(text: &str) -> Result<Time, String> {
    if text == "none" {
        return Ok(Time(None));
    }
    parse_duration(text).map(|t| Time(Some(t)))
}

/// The units of a size, each 1024 of the one before.
const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];

/// Parses a size such as `64MiB`, `2GiB` or `1000000` (bytes), or `none`.
fn parse_bytes(text: &str) -> Result<Bytes, String> {
    if text == "none" {
        return Ok(Bytes(None));
    }
    let split = text
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(text.len());
    let (number, unit) = text.split_at(split);
    let unit = unit.trim();
    let power = UNITS
        .iter()
        .position(|u| u.eq_ignore_ascii_case(unit) || (unit.is_empty() && *u == "B"))
        .ok_or_else(|| format!("unknown unit {unit:?}; use B, KiB, MiB, GiB or TiB, or `none`"))?;
    let number: u64 = number
        .parse()
        .map_err(|_| format!("{text:?} is not a size such as 64MiB or 2GiB, or `none`"))?;
    Ok(Bytes(Some(
        number.saturating_mul(1024u64.saturating_pow(power as u32)),
    )))
}

/// The output formats of `prove` and `check`.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Format {
    /// The verdict on one line, then the derivation as a text tree
    Text,
    /// JSON: for `prove` the outcome (verdict, fragment, mode, engine,
    /// statistics, and the proof, which `check` reads), one record per line
    /// in a batch, and under --logic the ordinary outcome, with its
    /// derivation of LK or LJ; for `check` the verdict and error
    Json,
    /// The verdict as a comment, then the derivation as a LaTeX proof tree
    /// of the ebproof package, with the connectives of cmll and amssymb
    #[value(alias = "tex")]
    Latex,
    /// The verdict as a comment, then the derivation as a Typst proof tree:
    /// with the curryst package for a low tree, in deduxit's own layout for
    /// a high one (`--style typst.layout`)
    Typst,
    /// The verdict as an XML comment, then the derivation drawn as an SVG
    /// document, set in the Euler Math font unless `--font` names another
    Svg,
    /// The derivation drawn as a PNG image, the verdict on standard error;
    /// written to a file or a pipe, never to a terminal
    Png,
    /// The derivation drawn as a PDF document, its text selectable, the
    /// verdict on standard error; written to a file or a pipe, never to a
    /// terminal
    Pdf,
    /// The verdict as a comment, then the derivation as a Rocq certificate
    /// for the kernel `--kernel` names or the mode chooses. For NanoYalla
    /// 1.1.3, the kernel of Click & coLLecT (the nanoyalla directory of
    /// github.com/ComputerAidedLL/click-and-collect, built with Rocq 9 and
    /// its standard library, no Yalla needed), it is a lemma stating the
    /// sequent one-sided, proved rule by rule; this kernel refuses Mix and
    /// the weakening of affine mode. For deduxit's own library, it is a
    /// lemma stating the sequent in the mode's calculus, two-sided for
    /// intuitionistic logic, proved from the proof term by the library's
    /// checker. The lemma ends with Qed; `--standalone` adds the imports
    /// (`--prelude`), and `--lemma` names the lemma
    Rocq,
}

impl Format {
    /// Returns the format a file's extension names, if it names one.
    pub(crate) fn of_path(path: &std::path::Path) -> Option<Self> {
        let extension = path.extension()?.to_str()?.to_ascii_lowercase();
        Self::value_variants()
            .iter()
            .copied()
            .find(|format| format.extension() == extension)
    }

    /// Returns the extension of a file in the format.
    pub(crate) fn extension(self) -> &'static str {
        match self {
            Self::Text => "txt",
            Self::Json => "json",
            Self::Latex => "tex",
            Self::Typst => "typ",
            Self::Svg => "svg",
            Self::Png => "png",
            Self::Pdf => "pdf",
            Self::Rocq => "v",
        }
    }

    /// Returns the name the format's options have in the styles, if it
    /// has options.
    pub(crate) fn style_key(self) -> Option<&'static str> {
        Some(match self {
            Self::Text => "text",
            Self::Json => return None,
            Self::Latex => "latex",
            Self::Typst => "typst",
            Self::Svg => "svg",
            Self::Png => "png",
            Self::Pdf => "pdf",
            Self::Rocq => "rocq",
        })
    }

    /// Whether the format is binary, so that it goes to a file or a pipe
    /// and its verdict to standard error.
    pub(crate) fn is_binary(self) -> bool {
        matches!(self, Self::Png | Self::Pdf)
    }

    /// Whether the format has a document form, which `--standalone` asks
    /// for (an SVG is always a whole document).
    pub(crate) fn has_document(self) -> bool {
        matches!(self, Self::Latex | Self::Typst | Self::Rocq)
    }

    /// Returns the format's name as a message writes it.
    pub(crate) fn title(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Json => "JSON",
            Self::Latex => "LaTeX",
            Self::Typst => "Typst",
            Self::Svg => "SVG",
            Self::Png => "PNG",
            Self::Pdf => "PDF",
            Self::Rocq => "Rocq",
        }
    }
}

impl OutputArgs {
    /// Returns the format the output is written in: `--format`, else the
    /// one the output file's extension names, else text.
    pub(crate) fn format(&self) -> Format {
        self.format
            .or_else(|| self.output.as_deref().and_then(Format::of_path))
            .unwrap_or(Format::Text)
    }
}

/// The output formats of `seq print`.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SequentFormat {
    /// Unicode text
    Text,
    /// LaTeX math, with the connectives of cmll and amssymb
    #[value(alias = "tex")]
    Latex,
    /// Typst math
    Typst,
    /// An SVG document of one line, set in the Euler Math font
    Svg,
    /// A PNG image of the SVG document's line
    Png,
    /// A PDF document of the SVG document's line
    Pdf,
}

impl From<SequentFormat> for Format {
    /// Returns the output format of the same name.
    fn from(format: SequentFormat) -> Self {
        match format {
            SequentFormat::Text => Self::Text,
            SequentFormat::Latex => Self::Latex,
            SequentFormat::Typst => Self::Typst,
            SequentFormat::Svg => Self::Svg,
            SequentFormat::Png => Self::Png,
            SequentFormat::Pdf => Self::Pdf,
        }
    }
}

/// The fragments `--fragment` names.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FragmentArg {
    /// Multiplicatives: ⊗ ⅋
    Mll,
    /// Multiplicatives and their units: ⊗ ⅋ 1 ⊥
    MllUnits,
    /// Additives only: & ⊕ ⊤ 0
    All,
    /// Multiplicatives and additives with units
    Mall,
    /// Multiplicatives and exponentials with units
    Mell,
    /// Every connective
    Ll,
}

impl From<FragmentArg> for Fragment {
    /// Returns the fragment the name stands for.
    fn from(f: FragmentArg) -> Self {
        match f {
            FragmentArg::Mll => Fragment::MLL,
            FragmentArg::MllUnits => Fragment::MLL_WITH_UNITS,
            FragmentArg::All => Fragment::ADDITIVE,
            FragmentArg::Mall => Fragment::MALL,
            FragmentArg::Mell => Fragment::MELL,
            FragmentArg::Ll => Fragment::LL,
        }
    }
}

/// The engine `--engine` forces, or `None` for the automatic choice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Forced(pub Option<Engine>);

/// Reads an engine's name, as the library spells it, or `auto`.
fn engine_name() -> impl TypedValueParser<Value = Forced> {
    clap::builder::PossibleValuesParser::new(
        std::iter::once("auto").chain(Engine::NAMES.iter().copied()),
    )
    .map(|name| Forced(name.parse().ok()))
}

/// The rules `--bias` names.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BiasArg {
    /// `rarer` in affine mode, `factors` in linear mode without
    /// exponentials, and in linear mode with exponentials both, each in a
    /// search of its own
    Auto,
    /// The literal with fewer occurrences is positive
    Rarer,
    /// The literal that is more often a direct factor of a tensor is
    /// positive, so that more splits are forced
    Factors,
}

impl From<BiasArg> for Bias {
    /// Returns the rule the name stands for.
    fn from(b: BiasArg) -> Self {
        match b {
            BiasArg::Auto => Bias::Auto,
            BiasArg::Rarer => Bias::Rarer,
            BiasArg::Factors => Bias::Factors,
        }
    }
}

/// Parses a duration such as `500ms`, `10s`, `1.5m` or `1h`; a bare number
/// is seconds.
fn parse_duration(text: &str) -> Result<Duration, String> {
    let split = text
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
        .unwrap_or(text.len());
    let (number, unit) = text.split_at(split);
    let seconds_per_unit = match unit.trim() {
        "ms" => 0.001,
        "" | "s" => 1.0,
        "m" | "min" => 60.0,
        "h" => 3600.0,
        _ => return Err(format!("unknown unit {unit:?}; use ms, s, m or h")),
    };
    let number: f64 = number
        .parse()
        .map_err(|_| format!("{text:?} is not a duration such as 500ms, 10s, 2m or 1h"))?;
    Duration::try_from_secs_f64(number * seconds_per_unit).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    /// The argument definitions are consistent, as clap checks them.
    #[test]
    fn arguments_are_consistent() {
        Cli::command().debug_assert();
    }

    /// The `--style` help names every field of every format's options,
    /// as the settings' JSON form has them.
    #[test]
    fn the_style_help_names_every_option() {
        let command = Cli::command();
        let prove = command.find_subcommand("prove").unwrap();
        let style = prove
            .get_arguments()
            .find(|a| a.get_id() == "style")
            .unwrap();
        let help = style.get_long_help().unwrap().to_string();
        let styles = serde_json::to_value(deduxit::export::Styles::default()).unwrap();
        for (format, fields) in styles.as_object().unwrap() {
            assert!(help.contains(&format!("{format}:")), "{format}");
            for field in fields.as_object().unwrap().keys() {
                assert!(help.contains(field.as_str()), "{format}.{field}");
            }
        }
    }

    /// Every `--settings` help names every top-level key of the settings
    /// file.
    #[test]
    fn the_settings_help_names_every_key() {
        let settings = serde_json::to_value(deduxit::Settings::default()).unwrap();
        let mut commands = vec![Cli::command()];
        let mut seen = 0;
        while let Some(command) = commands.pop() {
            if let Some(argument) = command.get_arguments().find(|a| a.get_id() == "settings") {
                let help = argument.get_long_help().unwrap().to_string();
                for key in settings.as_object().unwrap().keys() {
                    let key = format!("`{key}`");
                    assert!(help.contains(&key), "{}: {key}", command.get_name());
                }
                seen += 1;
            }
            commands.extend(command.get_subcommands().cloned());
        }
        assert!(seen >= 4, "{seen}");
    }

    /// Durations take a unit, seconds by default.
    #[test]
    fn durations() {
        for (text, millis) in [
            ("500ms", 500),
            ("10s", 10_000),
            ("1.5m", 90_000),
            ("1h", 3_600_000),
            ("2", 2000),
        ] {
            assert_eq!(parse_duration(text), Ok(Duration::from_millis(millis)));
        }
        for text in ["", "s", "10 days", "-1s", "1e3s"] {
            assert!(parse_duration(text).is_err(), "{text:?}");
        }
    }
}
