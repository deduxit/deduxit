// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! How a search runs: the [`Options`] and the values they hold, the
//! focused engine's [`Bias`], the threads ([`Jobs`]) and the inverse
//! method's turns ([`Cadence`], [`Schedule`]).

use super::Engine;
#[cfg(feature = "parallel")]
use super::Pool;
#[cfg(doc)]
use super::{Reason, Refutation, Statistics, Verdict, prove};
use crate::Error;
#[cfg(doc)]
use crate::limits::Limits;
use crate::linear::fragment::Fragment;
#[cfg(doc)]
use crate::linear::proofs::Proof;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::num::NonZeroUsize;
use std::str::FromStr;

/// How the focused engine chooses the positive literal of every atom.
///
/// Focusing is complete for every choice, so no rule changes what is
/// provable. The rules differ in speed and, with exponentials, in the
/// copies a branch of the proofs they lead to needs.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Bias {
    /// The rule chosen per sequent:
    /// - [`Factors`](Self::Factors) for a sequent without exponentials;
    /// - [`Rarer`](Self::Rarer) for a search with weakening;
    /// - for a sequent with a `!` or a `?` in linear mode, a search under
    ///   each rule. It answers with the first that decides, so it decides
    ///   whatever either does: the backward search within the copy bound,
    ///   the forward one within a bound of its own where the goal is a
    ///   Horn program. This holds without a stop; under a time limit, each
    ///   search has its share of the time.
    ///
    /// Each choice is what measured best there:
    /// - without exponentials, the factor rule visits up to a third of the
    ///   stable sequents of the rarer one (unsolvable 3-Partition with bins
    ///   of five: 971 against 3 373; a random QBF of 20 variables: 60 883
    ///   against 105 667);
    /// - under weakening nothing forces a split, and the factor rule
    ///   visited up to 700 times the stable sequents of the rarer one on
    ///   random affine sequents;
    /// - with exponentials neither rule wins. Of the Petri nets of the
    ///   LLTP library within 5 s, the pair decides 1 520, the backward
    ///   search run alone 442 and the forward one run alone 1 576. Where
    ///   each decides, the pair costs 1.3 times the backward search and
    ///   1.6 times the forward one.
    #[default]
    Auto,
    /// The literal with fewer occurrences in the sequent is positive, and
    /// `Atom` when both have the same number. With Horn-like hypotheses
    /// this mostly chains backward from the goal, which keeps the copies
    /// per branch low.
    Rarer,
    /// The literal that is more often a direct factor of a `⊗` is
    /// positive, an occurrence counting half for every `&` or `⊕` above
    /// it. On a tie, the rarer literal is. A `⊗` with a positive literal
    /// for a factor takes exactly the dual literal for it, so its split
    /// needs no search. With Horn-like hypotheses this chains forward from
    /// the facts, one copy per step on a single branch. That is fast, but
    /// needs a copy bound as large as the number of steps.
    Factors,
}

impl Bias {
    /// Every rule, in the order of [`NAMES`](Self::NAMES).
    pub const ALL: &'static [Self] = &[Self::Auto, Self::Rarer, Self::Factors];

    /// The rules' names, as [`name`](Self::name) writes them and
    /// [`FromStr`] reads them.
    pub const NAMES: &'static [&'static str] = &["auto", "rarer", "factors"];

    /// Returns the rule's name: `auto`, `rarer` or `factors`.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Rarer => "rarer",
            Self::Factors => "factors",
        }
    }
}

impl Display for Bias {
    /// Writes the rule's [`name`](Self::name).
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

impl FromStr for Bias {
    type Err = Error;

    /// Reads a rule's name, one of [`NAMES`](Self::NAMES).
    fn from_str(name: &str) -> Result<Self, Error> {
        Self::ALL
            .iter()
            .copied()
            .find(|bias| bias.name() == name)
            .ok_or_else(|| Error::UnknownName {
                what: "bias",
                name: name.into(),
                known: Self::NAMES,
            })
    }
}

/// How many threads a search may use. In JSON it is `"auto"` or the
/// number.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Jobs {
    /// Every thread the machine runs at once, as the platform tells, or
    /// one where it does not tell. A front end resolves it once to the
    /// count it shows.
    Auto,
    /// This many. One runs the sequential engines, whose proof is a
    /// function of the input. Zero counts as one, and more than
    /// [`Options::MAX_JOBS`] as that many.
    Count(usize),
}

impl Default for Jobs {
    /// One thread.
    fn default() -> Self {
        Self::Count(1)
    }
}

impl From<usize> for Jobs {
    /// That many threads.
    fn from(count: usize) -> Self {
        Self::Count(count)
    }
}

impl Jobs {
    /// Returns the number of threads: the count asked for, or the
    /// machine's for [`Auto`](Self::Auto), at least one and at most
    /// [`Options::MAX_JOBS`].
    pub fn count(self) -> usize {
        let count = match self {
            Self::Auto => std::thread::available_parallelism().map_or(1, NonZeroUsize::get),
            Self::Count(count) => count,
        };
        count.clamp(1, Options::MAX_JOBS)
    }
}

/// How often the net engines run their test. In JSON it is `"auto"` or
/// the number of links between two tests.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Cadence {
    /// After every link on a structure of at most
    /// [`AUTO_SMALL`](Self::AUTO_SMALL) occurrences, and after every
    /// [`AUTO_PERIOD`](Self::AUTO_PERIOD)th link on a larger one.
    #[default]
    Auto,
    /// After this many links; zero counts as one.
    Every(u32),
}

impl Cadence {
    /// The most occurrences a structure has on which [`Auto`](Self::Auto)
    /// tests after every link.
    pub const AUTO_SMALL: usize = 200;

    /// How many links go between two tests of [`Auto`](Self::Auto) on a
    /// larger structure.
    pub const AUTO_PERIOD: u32 = 4;
}

impl From<u32> for Cadence {
    /// Every so many links.
    fn from(links: u32) -> Self {
        Self::Every(links)
    }
}

impl From<Option<u32>> for Cadence {
    /// Every so many links, or the default for `None`.
    fn from(links: Option<u32>) -> Self {
        links.map_or(Self::Auto, Self::Every)
    }
}

/// How the two searches of the default bias share one thread. In JSON
/// it is `"auto"` or `"turns"`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum Schedule {
    /// In slices on two threads where the feature `parallel` starts a
    /// second one, and in turns otherwise.
    #[default]
    Auto,
    /// In turns on the calling thread, each turn from the search's start.
    /// So the polls and the answer are the same on every build, the
    /// WebAssembly build included.
    Turns,
}

/// The knobs of a search: which engine and fragment to use instead of the
/// detected ones, how many copies to allow, how much to remember, and how
/// many threads to use.
///
/// The defaults suit a sequent of a few hundred occurrences on a thread
/// with the usual stack. The bounds on memory, occurrences, recursion and
/// work are not options: they are the [`Limits`] a search is given. The
/// bounds on memory and work bind the check of the proof found as well.
///
/// Every field has a builder, so the options chain from the default.
///
/// # JSON
///
/// With the feature `serialize`, the options are an object of the fields
/// below. A missing field takes its default, and a misspelt one is
/// refused.
/// - `"engine"` and `"fragment"`: a name or `"auto"`;
/// - `"bias"` and `"schedule"`: a name;
/// - `"copies"`: a number, or `null` for no bound;
/// - `"forward_copies"`, `"memo_limit"`, `"classical_work"`,
///   `"witness_depth"` and `"instances"`: numbers;
/// - `"test_period"` and `"jobs"`: a number or `"auto"`;
/// - `"check"`: a boolean.
///
/// The pool is never written.
///
/// # Examples
///
/// ```
/// use deduxit::linear::Fragment;
/// use deduxit::linear::search::{Bias, Options};
///
/// // Search in MALL with a smaller memo, the rarer bias and no copy bound.
/// let options = Options::default()
///     .with_memo_limit(1 << 16)
///     .with_fragment(Some(Fragment::MALL))
///     .with_bias(Bias::Rarer)
///     .with_copies(None);
/// assert_eq!(options.copies, None);
/// assert_eq!(options.bias, Bias::Rarer);
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(default, deny_unknown_fields)
)]
pub struct Options {
    /// The engine to use, or `None` for the one the detected fragment and
    /// the mode call for.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::auto"))]
    pub engine: Option<Engine>,
    /// The fragment to search in, or `None` for the detected one.
    ///
    /// A sequent outside it is refused. A fragment larger than the
    /// detected one switches off the prunes that only hold in the smaller
    /// one.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::auto"))]
    pub fragment: Option<Fragment>,
    /// How the focused engine picks the positive literal of every atom.
    /// No choice changes what is provable.
    ///
    /// [`Bias::Rarer`] mostly chains backward from the goal, and
    /// [`Bias::Factors`] forward from the hypotheses. Take a sequent with
    /// exponentials whose hypotheses are Horn clauses, such as a Petri
    /// net. The forward search often decides it in milliseconds where the
    /// backward one does not decide at all. But its proofs take one copy
    /// per step on a single branch.
    ///
    /// The default, [`Bias::Auto`], is `Factors` without exponentials and
    /// `Rarer` under weakening. On a sequent with exponentials in linear
    /// mode it runs both searches, the backward one within
    /// [`copies`](Self::copies) and the forward one within
    /// [`forward_copies`](Self::forward_copies), and answers with the first
    /// that decides. Either rule named explicitly is that search alone,
    /// within `copies`. Only the focused engine reads it.
    pub bias: Bias,
    /// The most copies of `?` formulas one branch of a proof may take, or
    /// `None` for no bound.
    ///
    /// The search deepens the bound from zero, one level after another. A
    /// sequent that has no proof within the bound is [`Reason::CopyBound`].
    /// The exception is a level that finished without ever reaching its
    /// bound: that makes the sequent [`Verdict::Unprovable`].
    ///
    /// Without a bound, the search goes on to the next level until it
    /// decides, its stop condition fires or a limit binds.
    /// [`Statistics::copies`] then says how far it got. That is the choice
    /// of a caller with a time limit, such as the command: some provable
    /// sequent needs more copies than any given bound, and full linear
    /// logic is undecidable.
    ///
    /// Without exponentials the bound has no effect. A proof found at some
    /// level may reuse a memoized subproof found with more copies left, so
    /// the bound limits the search, not the proof returned. Only the
    /// focused engine reads it: the net and additive engines' goals have no
    /// exponentials, and the Horn engine keeps every marking once and
    /// needs no bound.
    pub copies: Option<u32>,
    /// The most copies of `?` formulas one branch may take in the forward
    /// search that [`Bias::Auto`] runs beside the backward one on a Horn
    /// program.
    ///
    /// A Horn program here is clauses `!(a ⊗ b ⊸ c ⊗ d)` (or such a clause
    /// used once), a marking and a goal of atoms, as a Petri net is. A
    /// forward chain takes a copy per step, all on one branch, so it wants
    /// a larger bound than [`copies`](Self::copies). The forward search
    /// runs within the larger of the two, never within less than `copies`.
    ///
    /// It has no effect on any other goal, under another bias, under Mix,
    /// under weakening, or without a copy bound: unbounded, the forward
    /// search deepens as far as its share of the work takes it. Only the
    /// focused engine reads it.
    pub forward_copies: u32,
    /// The most stable sequents the memo holds at once. Zero switches the
    /// memo off.
    ///
    /// A full memo is emptied, which costs time but not correctness. The
    /// two searches that [`Bias::Auto`] runs on a sequent with
    /// exponentials hold a memo of this size each. This is the finer knob
    /// beside the memory bound
    /// ([`Limits::memory_bytes`](crate::Limits::memory_bytes)), which
    /// bounds the memo in bytes: a table that fits the processor's cache
    /// can be faster than one that fits the memory. The focused engine and
    /// the additive path read it; the net engine keeps no memo.
    pub memo_limit: u32,
    /// How often the net engine runs its exact acyclicity test, and the
    /// essential-net engine its test of the dominator condition.
    ///
    /// The test also runs on every complete linking. So the cadence trades
    /// time per link against how long a doomed branch is followed. Only
    /// those two engines read it.
    pub test_period: Cadence,
    /// How many threads the search may use.
    ///
    /// One, the default, runs the sequential engines, whose proof is a
    /// function of the input. More than one, with the `parallel` feature,
    /// runs the focused engine and the net engine on that many threads of
    /// a pool of their own. The pool may find a different proof, but never
    /// contradicts the sequential verdict. One of the two may decide where
    /// the other stops at the copy bound or the recursion limit, and which
    /// one depends on how the threads interleave. Without the feature, or
    /// for the engines that are not [`Engine::parallel`], the search stays
    /// sequential.
    ///
    /// A search starts no more threads than the machine runs at once
    /// ([`std::thread::available_parallelism`], where the platform tells).
    /// Threads beyond that only take turns on the same processors.
    pub jobs: Jobs,
    /// How the two searches of the default bias share one thread.
    pub schedule: Schedule,
    /// Whether a proof passes the checker ([`Proof::check`], which shares
    /// no code with the engines) before the search returns it.
    ///
    /// A proof that does not pass is [`Error::Rejected`], never a verdict.
    /// A proof of a goal is checked against the goal. Without the check,
    /// the proof is the engine's word. A caller that checks it itself, or
    /// times the search alone, may prefer that.
    pub check: bool,
    /// The most work the classical refuter may do after a search that did
    /// not prove the goal, in terms evaluated. Zero switches it off.
    ///
    /// The refuter looks for an assignment of truth values to the atoms
    /// under which the goal's classical reading is false. That reading
    /// erases `!` and `?`, reads `⊗` and `&` as "and", `⅋` and `⊕` as "or",
    /// `1` and `⊤` as true, and `⊥` and `0` as false. No provable sequent
    /// has such an assignment, in any mode. So an assignment makes an
    /// undecided search [`Verdict::Unprovable`] with
    /// [`Refutation::Classical`]. It also gives an exhausted search that
    /// certificate, where the counts give none.
    ///
    /// It never changes a verdict the search gave, nor its counters. It
    /// runs under the caller's stop and the memory bound, on the calling
    /// thread, whatever engine searched.
    pub classical_work: u64,
    /// The deepest term a metavariable of a first-order search may be
    /// bound to.
    ///
    /// A constant's depth is 0, and an application's is one more than its
    /// deepest argument's, measured as the binding is made. The focused
    /// engine searches a first-order sequent with a metavariable for every
    /// witness of an `∃`, and binds it where an axiom unifies two atoms. A
    /// branch whose binding would be deeper is cut there. A search that
    /// cut one is not [`Verdict::Unprovable`] but [`Reason::WitnessDepth`].
    ///
    /// A sequent without exponentials needs no witness deeper than its
    /// terms together. So the bound matters where copies of a `∀`
    /// hypothesis build a term step by step. Only the focused engine reads
    /// it, on first-order sequents.
    pub witness_depth: u32,
    /// The most instances of quantified formulas a first-order search may
    /// hold at once.
    ///
    /// Every `∀` and `∃` step on the branches of the proof in the making
    /// opens one. A step past the bound cuts its branch, and a search that
    /// cut one is not [`Verdict::Unprovable`] but [`Reason::Instances`].
    /// It bounds what the instances hold, which the copies of quantified
    /// hypotheses multiply. Only the focused engine reads it, on
    /// first-order sequents.
    pub instances: u32,
    /// The longest active path the connection prover of classical logic
    /// searches with (`ordinary::Method::Connection`), or `None` for no
    /// bound.
    ///
    /// The prover deepens the bound from 1. A step that copies a clause
    /// with a fresh variable may only lengthen a shorter path. A search
    /// that reached the bound without a proof is not "not valid" but
    /// unknown with [`Reason::PathLength`]. Only that prover reads it.
    pub path_length: Option<u32>,
    /// Whether the connection prover backtracks into a literal it has
    /// solved (`false`, the default) or keeps the first solution
    /// (restricted backtracking, Otten 2010). Restricted backtracking
    /// often proves sooner but may miss a proof, so a search under it
    /// never answers "not valid". Only that prover reads it.
    pub restricted_backtracking: bool,
    /// The largest domain the finite model search of ordinary logic
    /// (`ordinary::Method::Models`) tries, or `None` for no bound.
    ///
    /// The search tries domains of 1, 2, … elements up to the bound. One
    /// that found no structure that refutes the sequent is not "valid" but
    /// unknown with [`Reason::ModelSize`]. Only that search reads it.
    pub model_size: Option<u32>,
    /// The most worlds of a Kripke model the finite model search tries in
    /// intuitionistic and minimal logic, or `None` for no bound.
    ///
    /// The search tries models of 1, 2, … worlds beside 1, 2, … elements
    /// a domain. One that found no model that refutes the sequent within
    /// both bounds is unknown with [`Reason::KripkeSize`]. Only that search
    /// reads it.
    pub model_worlds: Option<u32>,
    /// The thread pools a parallel search borrows instead of starting
    /// threads of its own. `None`, the default, is a pool built for the
    /// search and dropped with it.
    ///
    /// A pool changes nothing a search does or finds. It only starts its
    /// threads once for many searches. Options are equal only if they name
    /// clones of the same pool, or both name none. The pool is never
    /// written.
    ///
    /// Needs the cargo feature `parallel` (off by default).
    #[cfg(feature = "parallel")]
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub pool: Option<Pool>,
}

impl Default for Options {
    /// Returns the default options:
    /// - the engine and fragment chosen by detection, and the default bias;
    /// - a copy bound of [`DEFAULT_COPIES`](Self::DEFAULT_COPIES), and one
    ///   of [`DEFAULT_FORWARD_COPIES`](Self::DEFAULT_FORWARD_COPIES) for the
    ///   forward search of the default bias;
    /// - a memo of at most [`DEFAULT_MEMO_LIMIT`](Self::DEFAULT_MEMO_LIMIT)
    ///   stable sequents;
    /// - the net engine's exact test at its default cadence;
    /// - one thread, and the default schedule;
    /// - every proof checked ([`DEFAULT_CHECK`](Self::DEFAULT_CHECK));
    /// - the classical refuter's bound
    ///   [`DEFAULT_CLASSICAL_WORK`](Self::DEFAULT_CLASSICAL_WORK);
    /// - the first-order bounds
    ///   [`DEFAULT_WITNESS_DEPTH`](Self::DEFAULT_WITNESS_DEPTH) and
    ///   [`DEFAULT_INSTANCES`](Self::DEFAULT_INSTANCES);
    /// - the connection prover's bound
    ///   [`DEFAULT_PATH_LENGTH`](Self::DEFAULT_PATH_LENGTH), and full
    ///   backtracking;
    /// - the finite model search's bounds
    ///   [`DEFAULT_MODEL_SIZE`](Self::DEFAULT_MODEL_SIZE) and
    ///   [`DEFAULT_MODEL_WORLDS`](Self::DEFAULT_MODEL_WORLDS).
    fn default() -> Self {
        Self {
            engine: None,
            fragment: None,
            bias: Bias::Auto,
            copies: Some(Self::DEFAULT_COPIES),
            forward_copies: Self::DEFAULT_FORWARD_COPIES,
            memo_limit: Self::DEFAULT_MEMO_LIMIT,
            test_period: Cadence::Auto,
            jobs: Jobs::Count(1),
            schedule: Schedule::Auto,
            check: Self::DEFAULT_CHECK,
            classical_work: Self::DEFAULT_CLASSICAL_WORK,
            witness_depth: Self::DEFAULT_WITNESS_DEPTH,
            instances: Self::DEFAULT_INSTANCES,
            path_length: Some(Self::DEFAULT_PATH_LENGTH),
            restricted_backtracking: false,
            model_size: Some(Self::DEFAULT_MODEL_SIZE),
            model_worlds: Some(Self::DEFAULT_MODEL_WORLDS),
            #[cfg(feature = "parallel")]
            pool: None,
        }
    }
}

impl Options {
    /// The memo limit of the default options: 2²⁰ stable sequents.
    pub const DEFAULT_MEMO_LIMIT: u32 = 1 << 20;

    /// The copy bound of the default options: three copies per branch, the
    /// bound llprover searches with by default.
    ///
    /// The default options keep a bound because a search without one ends
    /// only when it decides, its stop condition fires or a limit binds,
    /// and [`prove`] has no stop condition. A front end with a clock lifts
    /// it ([`copies`](Self::copies)).
    pub const DEFAULT_COPIES: u32 = 3;

    /// The forward search's copy bound of the default options: thirty
    /// steps of a forward chain.
    ///
    /// A deeper bound decided nothing more within seconds on Petri nets
    /// from practice. The price of the bound is what an undecided search to
    /// that depth costs: mostly milliseconds on a small Horn program, a
    /// second and more on one whose markings grow in several places at
    /// once.
    pub const DEFAULT_FORWARD_COPIES: u32 = 30;

    /// Whether the default options check a proof before returning it: they
    /// do. The check is one pass over the proof, in memory proportional to
    /// it.
    pub const DEFAULT_CHECK: bool = true;

    /// The classical refuter's bound of the default options: 2²⁶ terms
    /// evaluated. That is enough to try every assignment of a goal of 18
    /// atoms and 256 terms, and far more where a partial assignment decides
    /// the reading.
    pub const DEFAULT_CLASSICAL_WORK: u64 = 1 << 26;

    /// The witness depth of the default options: terms 64 applications
    /// deep. That is far past any depth a sequent written by hand asks
    /// for, and a branch that builds a term forever reaches it quickly.
    pub const DEFAULT_WITNESS_DEPTH: u32 = 64;

    /// The instances of the default options: 2¹⁶ instances of quantified
    /// formulas held at once, some megabytes of the search's tables.
    pub const DEFAULT_INSTANCES: u32 = 1 << 16;

    /// The connection prover's bound on the active path in the default
    /// options: a library call without a stop must end, and a front end
    /// with a clock lifts it ([`path_length`](Self::path_length)).
    pub const DEFAULT_PATH_LENGTH: u32 = 8;

    /// The finite model search's largest domain in the default options: a
    /// library call without a stop must end, and a front end with a clock
    /// lifts it ([`model_size`](Self::model_size)).
    pub const DEFAULT_MODEL_SIZE: u32 = 6;

    /// The finite model search's most worlds of a Kripke model in the
    /// default options ([`model_worlds`](Self::model_worlds)).
    pub const DEFAULT_MODEL_WORLDS: u32 = 4;

    /// The most threads the options name: more are taken as this many.
    ///
    /// A pool beyond it has no known use, and a count without a bound
    /// starts whatever it is given: ten thousand threads on `A ⊢ A` cost
    /// three minutes of processor time.
    pub const MAX_JOBS: usize = 256;

    /// Returns the options with another bound on the classical refuter's
    /// work ([`classical_work`](Self::classical_work)), zero for none.
    #[must_use]
    pub fn with_classical_work(self, classical_work: u64) -> Self {
        Self {
            classical_work,
            ..self
        }
    }

    /// Returns the options with another
    /// [`witness_depth`](Self::witness_depth).
    #[must_use]
    pub fn with_witness_depth(self, witness_depth: u32) -> Self {
        Self {
            witness_depth,
            ..self
        }
    }

    /// Returns the options with another bound on
    /// [`instances`](Self::instances).
    #[must_use]
    pub fn with_instances(self, instances: u32) -> Self {
        Self { instances, ..self }
    }

    /// Returns the options with another bound on the connection prover's
    /// [`path_length`](Self::path_length), `None` for none.
    #[must_use]
    pub fn with_path_length(self, path_length: Option<u32>) -> Self {
        Self {
            path_length,
            ..self
        }
    }

    /// Returns the options with another bound on the finite model search's
    /// [`model_size`](Self::model_size), `None` for none.
    #[must_use]
    pub fn with_model_size(self, model_size: Option<u32>) -> Self {
        Self { model_size, ..self }
    }

    /// Returns the options with another bound on the finite model search's
    /// [`model_worlds`](Self::model_worlds), `None` for none.
    #[must_use]
    pub fn with_model_worlds(self, model_worlds: Option<u32>) -> Self {
        Self {
            model_worlds,
            ..self
        }
    }

    /// Returns the options with restricted backtracking in the connection
    /// prover switched on or off
    /// ([`restricted_backtracking`](Self::restricted_backtracking)).
    #[must_use]
    pub fn with_restricted_backtracking(self, restricted_backtracking: bool) -> Self {
        Self {
            restricted_backtracking,
            ..self
        }
    }

    /// Returns the options with another [`engine`](Self::engine).
    #[must_use]
    pub fn with_engine(self, engine: Option<Engine>) -> Self {
        Self { engine, ..self }
    }

    /// Returns the options with another [`fragment`](Self::fragment).
    #[must_use]
    pub fn with_fragment(self, fragment: Option<Fragment>) -> Self {
        Self { fragment, ..self }
    }

    /// Returns the options with another [`bias`](Self::bias).
    #[must_use]
    pub fn with_bias(self, bias: Bias) -> Self {
        Self { bias, ..self }
    }

    /// Returns the options with another copy bound
    /// ([`copies`](Self::copies)).
    #[must_use]
    pub fn with_copies(self, copies: Option<u32>) -> Self {
        Self { copies, ..self }
    }

    /// Returns the options with another copy bound for the forward search
    /// ([`forward_copies`](Self::forward_copies)).
    #[must_use]
    pub fn with_forward_copies(self, forward_copies: u32) -> Self {
        Self {
            forward_copies,
            ..self
        }
    }

    /// Returns the options with another [`memo_limit`](Self::memo_limit).
    #[must_use]
    pub fn with_memo_limit(self, memo_limit: u32) -> Self {
        Self { memo_limit, ..self }
    }

    /// Returns the options with another cadence of the net engines' test
    /// ([`test_period`](Self::test_period)).
    #[must_use]
    pub fn with_test_period(self, test_period: impl Into<Cadence>) -> Self {
        Self {
            test_period: test_period.into(),
            ..self
        }
    }

    /// Returns the options with another number of threads
    /// ([`jobs`](Self::jobs)).
    #[must_use]
    pub fn with_jobs(self, jobs: impl Into<Jobs>) -> Self {
        Self {
            jobs: jobs.into(),
            ..self
        }
    }

    /// Returns the options with another [`schedule`](Self::schedule).
    #[must_use]
    pub fn with_schedule(self, schedule: Schedule) -> Self {
        Self { schedule, ..self }
    }

    /// Returns the options with or without the [`check`](Self::check) of
    /// a proof.
    #[must_use]
    pub fn with_check(self, check: bool) -> Self {
        Self { check, ..self }
    }

    /// Returns the options with another [`pool`](Self::pool).
    ///
    /// Needs the cargo feature `parallel` (off by default).
    #[cfg(feature = "parallel")]
    #[must_use]
    pub fn with_pool(self, pool: Option<Pool>) -> Self {
        Self { pool, ..self }
    }

    /// Returns the copy bound as the engines count it. No bound is the
    /// largest, which no search reaches: every level visits a stable
    /// sequent and polls at it, and four billion levels take hours at the
    /// least.
    pub(crate) fn copy_bound(&self) -> u32 {
        self.copies.unwrap_or(u32::MAX)
    }

    /// Returns the threads the search may use: [`Jobs::count`].
    #[cfg(feature = "parallel")]
    pub(crate) fn threads(&self) -> usize {
        self.jobs.count()
    }

    /// Returns the most stable sequents a memo holds.
    pub(crate) fn memo_entries(&self) -> usize {
        usize::try_from(self.memo_limit).unwrap_or(usize::MAX)
    }
}
