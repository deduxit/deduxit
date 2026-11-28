// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! What a search returns: the [`Outcome`], its [`Verdict`], the
//! [`Reason`] it gave up and its [`Statistics`].

#[cfg(doc)]
use super::{Bias, Refutation, Schedule};
use super::{Disproof, Engine, Options};
use crate::limits::Limits;
#[cfg(doc)]
use crate::limits::Progress;
use crate::linear::fragment::{Fragment, Mode};
use crate::linear::nets::ProofStructure;
use crate::linear::proofs::{Bytes, Proof};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// What a search returned: the verdict, and how it was reached.
///
/// # JSON
///
/// With the feature `serialize`, an outcome is written as one object
/// ([`wire`](crate::wire)) and read back from it. Its keys:
/// - `version`, and `deduxit`, the version of the crate that wrote it;
/// - `verdict`: `"proved"`, `"unprovable"` or `"unknown"`;
/// - `checked`, for a proved sequent;
/// - `refutation`, for an unprovable one, in the form [`Refutation`]
///   gives;
/// - `reason`, for an unknown one, in the form [`Reason`] gives, tagged
///   by `kind`;
/// - `fragment`, its name in the mode, as [`Fragment::name_in`] gives it;
/// - `mode`, its name;
/// - `engine` and `statistics`;
/// - for a proved sequent, the proof's own keys `sequent`, `nodes` and
///   `goal`, so that the outcome reads back as a [`Proof`];
/// - for an unprovable one, the [`Disproof`]'s `sequent` and `goal`.
///
/// The command's `prove --format json` writes it. Reading checks every
/// index the proof or the disproof names, not the proof or the
/// refutation: [`Proof::check`] and [`Disproof::check`] do that. An engine
/// name this version does not know reads as [`Engine::Unknown`], the
/// `deduxit` key is not read, `checked` is false where it is missing, and
/// [`net`](Self::net) is `None`.
///
#[cfg_attr(all(feature = "parse", feature = "serialize"), doc = "```")]
#[cfg_attr(not(all(feature = "parse", feature = "serialize")), doc = "```ignore")]
/// use deduxit::linear::{Mode, Outcome, Sequent, prove, search::Options};
///
/// let sequent: Sequent = "A |- A".parse()?;
/// let text = serde_json::to_string(&prove(&sequent, Mode::CLASSICAL, &Options::default())?)?;
/// let outcome: Outcome = serde_json::from_str(&text)?;
/// if let Some(proof) = outcome.verdict.proof() {
///     proof.check(outcome.mode)?;
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::linear::search::{Engine, Options, prove};
/// use deduxit::linear::{Mode, Sequent};
///
/// let sequent: Sequent = "|- a, ~a".parse()?;
/// let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
/// assert_eq!(outcome.verdict.name(), "proved");
/// assert!(outcome.checked);
/// assert_eq!(outcome.engine, Engine::Focus);
/// println!("{} units of work", outcome.statistics.work);
/// # Ok::<(), deduxit::Error>(())
/// ```
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Outcome {
    /// The verdict, with the proof if there is one.
    pub verdict: Verdict,
    /// The fragment searched in: the detected one, or the one the options
    /// asserted.
    pub fragment: Fragment,
    /// The mode searched in.
    pub mode: Mode,
    /// The engine that ran.
    pub engine: Engine,
    /// What the search cost.
    pub statistics: Statistics,
    /// The proof net the proof was read off, when the net engine found
    /// one. `None` for the other engines and for any other verdict.
    pub net: Option<Box<ProofStructure>>,
    /// Whether the proof passed the checker before it was returned. It is
    /// false only where [`Options::check`] switched the check off, and for
    /// every verdict but a proof. In an outcome read from a document it is
    /// what the document says: [`Proof::check`] checks the proof.
    pub checked: bool,
    /// The name a document gave the engine where `engine` is
    /// [`Engine::Unknown`], so that the outcome is written back with it.
    #[cfg_attr(
        not(feature = "serialize"),
        expect(dead_code, reason = "only the JSON forms read and write it")
    )]
    pub(crate) named: Option<Box<str>>,
}

/// What a search found: a proof, that there is none, or that it could not
/// tell.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::linear::search::{Options, Verdict, prove};
/// use deduxit::linear::{Mode, Sequent};
///
/// let sequent: Sequent = "|- a, a".parse()?;
/// let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
/// match &outcome.verdict {
///     Verdict::Proved(proof) => println!("{}", proof.derivation()?),
///     // The disproof prints why the sequent has no proof.
///     Verdict::Unprovable(d) => println!("{d}"),
///     Verdict::Unknown(reason) => println!("unknown: {reason}"),
/// }
/// assert!(matches!(outcome.verdict, Verdict::Unprovable(_)));
/// # Ok::<(), deduxit::Error>(())
/// ```
#[derive(Clone, Debug)]
pub enum Verdict {
    /// The sequent is provable, with a proof. It is boxed because a proof
    /// carries its forest.
    Proved(Box<Proof>),
    /// The sequent is not provable, with what can be said of why. The
    /// search was exhaustive, or a refuter found a refutation. It is boxed
    /// as a proof is.
    Unprovable(Box<Disproof>),
    /// The search stopped before it could decide, for the reason given.
    Unknown(Reason),
}

impl Verdict {
    /// The words of the verdicts, as [`Verdict::name`] gives them and the
    /// JSON form's `verdict` writes them.
    pub const NAMES: &'static [&'static str] = &["proved", "unprovable", "unknown"];

    /// Returns the verdict's word: `proved`, `unprovable` or `unknown`.
    pub const fn name(&self) -> &'static str {
        match self {
            Verdict::Proved(_) => "proved",
            Verdict::Unprovable(_) => "unprovable",
            Verdict::Unknown(_) => "unknown",
        }
    }

    /// Returns the proof, if the sequent was proved.
    pub fn proof(&self) -> Option<&Proof> {
        match self {
            Verdict::Proved(proof) => Some(proof),
            _ => None,
        }
    }
}

/// Why a search stopped without deciding.
///
/// In JSON (feature `serialize`), a reason is tagged by `kind`:
/// `{"kind": "stopped"}`, `{"kind": "recursion_limit", "depth": 2048}`,
/// `{"kind": "copy_bound", "copies": 3}`, `{"kind": "memory_limit",
/// "limit_bytes": 1073741824}`, `{"kind": "index_limit"}`, `{"kind":
/// "work_limit", "limit": 1000000}`, `{"kind": "unchecked",
/// "limit_bytes": 1073741824}`, `{"kind": "witness_depth", "depth": 64}`
/// `{"kind": "instances", "instances": 65536}`, `{"kind": "incomplete"}`
/// or `{"kind": "path_length", "length": 8}`.
/// A reader refuses a kind it does not know.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Reason {
    /// The caller's stop condition fired: a time limit or an interruption.
    Stopped,
    /// The engine's recursion reached
    /// [`Limits::recursion_depth`](crate::Limits::recursion_depth).
    #[non_exhaustive]
    RecursionLimit {
        /// The recursion depth the limits allowed.
        depth: u32,
    },
    /// Every level up to the bound [`Options::copies`] set hit its bound
    /// on some branch. So a proof with more copies of a `?` formula per
    /// branch may exist.
    #[non_exhaustive]
    CopyBound {
        /// The copy bound.
        copies: u32,
    },
    /// The search would have held more than
    /// [`Limits::memory_bytes`](crate::Limits::memory_bytes).
    ///
    /// That is with its memo emptied, where it keeps one. Or there was no
    /// room left for a memo, the Horn engine's markings, or a proof's
    /// nodes.
    #[non_exhaustive]
    MemoryLimit {
        /// The memory bound, in bytes. It is `u64::MAX` where there was no
        /// bound and the allocator refused what the search asked for.
        limit_bytes: u64,
    },
    /// A structure of the search outgrew what its indices address.
    ///
    /// The limits: the proof arena at 2³¹ nodes, the additive path's at
    /// 2³² nodes, the count invariants at 2³² row entries, the Horn
    /// engine's markings at 2³², and a count of its tokens at 2³². Only a
    /// search without a memory bound gets this far, except for the tokens:
    /// a clause that adds thousands of them at each firing can pass the
    /// limit in a few hundred thousand markings.
    IndexLimit,
    /// The search did the most work [`Limits::work`](crate::Limits::work)
    /// allows, or the check of the proof it found would have done more.
    #[non_exhaustive]
    WorkLimit {
        /// The bound on work.
        limit: u64,
    },
    /// The search found a proof whose check would have held more than
    /// [`Limits::memory_bytes`](crate::Limits::memory_bytes). No proof is
    /// returned that has not passed the checker.
    #[non_exhaustive]
    Unchecked {
        /// The memory bound, in bytes.
        limit_bytes: u64,
    },
    /// A first-order search cut a branch where a metavariable would have
    /// been bound to a term deeper than [`Options::witness_depth`], and
    /// found no proof.
    #[non_exhaustive]
    WitnessDepth {
        /// The witness depth.
        depth: u32,
    },
    /// A first-order search cut a branch where it would have held more
    /// instances of quantified formulas than [`Options::instances`], and
    /// found no proof.
    #[non_exhaustive]
    Instances {
        /// The bound on instances.
        instances: u32,
    },
    /// A search that may miss proofs ended without one, so the sequent may
    /// still hold. An ordinary first-order sequent whose image the linear
    /// search finds unprovable is unknown for this reason: the embedding
    /// is not shown complete at first order (the classical image has no
    /// contraction).
    Incomplete,
    /// The connection prover reached its bound on the active path
    /// ([`Options::path_length`]) without a proof.
    PathLength {
        /// The bound.
        length: u32,
    },
    /// The finite model search tried every domain up to its bound
    /// ([`Options::model_size`]) and found no structure that refutes the
    /// sequent.
    ModelSize {
        /// The bound.
        size: u32,
    },
    /// The finite model search tried every Kripke model of its kind within
    /// its bounds on worlds ([`Options::model_worlds`]) and elements a world
    /// ([`Options::model_size`]) and found none that refutes the sequent.
    /// Its kind: every world's domain an initial segment of one numbering of
    /// the elements, so two worlds that are not ordered add the same new
    /// elements (models over a tree lose nothing by it).
    KripkeSize {
        /// The bound on worlds.
        worlds: u32,
        /// The bound on elements a world.
        size: u32,
    },
}

impl Reason {
    /// The words of the reasons, as [`Reason::name`] gives them and the
    /// JSON form's `kind` writes them.
    pub const NAMES: &'static [&'static str] = &[
        "stopped",
        "recursion_limit",
        "copy_bound",
        "memory_limit",
        "index_limit",
        "work_limit",
        "unchecked",
        "witness_depth",
        "instances",
        "incomplete",
        "path_length",
        "model_size",
        "kripke_size",
    ];

    /// Returns the reason's word, its `kind` in the JSON form.
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Stopped => "stopped",
            Self::RecursionLimit { .. } => "recursion_limit",
            Self::CopyBound { .. } => "copy_bound",
            Self::MemoryLimit { .. } => "memory_limit",
            Self::IndexLimit => "index_limit",
            Self::WorkLimit { .. } => "work_limit",
            Self::Unchecked { .. } => "unchecked",
            Self::WitnessDepth { .. } => "witness_depth",
            Self::Instances { .. } => "instances",
            Self::Incomplete => "incomplete",
            Self::PathLength { .. } => "path_length",
            Self::ModelSize { .. } => "model_size",
            Self::KripkeSize { .. } => "kripke_size",
        }
    }

    /// Returns the reason a search under the options gives up with, given
    /// the reason one of its searches gave.
    ///
    /// A copy bound is [`Options::copies`], the least bound every search
    /// ran within. A memory limit is [`Limits::memory_bytes`], of which a
    /// search may have had a part.
    pub(crate) fn as_set(self, options: &Options, limits: &Limits) -> Self {
        match self {
            Self::CopyBound { .. } => Self::CopyBound {
                copies: options.copy_bound(),
            },
            Self::MemoryLimit { limit_bytes: bytes } => Self::MemoryLimit {
                limit_bytes: limits.memory_bytes.unwrap_or(bytes),
            },
            reason => reason,
        }
    }

    /// Every settings key [`setting`](Self::setting) names, once each:
    /// what a record read back may name, held to the reasons themselves.
    #[cfg(feature = "protocol")]
    pub(crate) const SETTINGS: &'static [&'static str] = &[
        "limits.recursion_depth",
        "search.copies",
        "limits.memory_bytes",
        "limits.work",
        "search.witness_depth",
        "search.instances",
        "search.path_length",
        "search.model_size",
        "search.model_worlds",
    ];

    /// Returns the settings key whose bound the search reached, which a
    /// caller raises to search further.
    ///
    /// It is `None` for what no setting lifts: the stop, the indices, and
    /// the allocator's refusal where there was no memory bound.
    pub const fn setting(&self) -> Option<&'static str> {
        match self {
            Self::Stopped
            | Self::IndexLimit
            | Self::Incomplete
            | Self::MemoryLimit {
                limit_bytes: u64::MAX,
            } => None,
            Self::RecursionLimit { .. } => Some("limits.recursion_depth"),
            Self::CopyBound { .. } => Some("search.copies"),
            Self::MemoryLimit { .. } | Self::Unchecked { .. } => Some("limits.memory_bytes"),
            Self::WorkLimit { .. } => Some("limits.work"),
            Self::WitnessDepth { .. } => Some("search.witness_depth"),
            Self::Instances { .. } => Some("search.instances"),
            Self::PathLength { .. } => Some("search.path_length"),
            Self::ModelSize { .. } => Some("search.model_size"),
            Self::KripkeSize { .. } => Some("search.model_worlds"),
        }
    }
}

impl Display for Reason {
    /// Writes the reason as a phrase, such as `the copy bound of 3 was
    /// reached`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Reason::Stopped => f.write_str("the search was stopped"),
            Reason::RecursionLimit { depth } => {
                write!(f, "the recursion limit of {depth} was reached")
            }
            Reason::CopyBound { copies } => {
                write!(f, "the copy bound of {copies} was reached")
            }
            Reason::MemoryLimit {
                limit_bytes: u64::MAX,
            } => f.write_str("the memory the allocator would give was used up"),
            Reason::MemoryLimit { limit_bytes } => {
                write!(f, "the memory limit of {} was reached", Bytes(*limit_bytes))
            }
            Reason::IndexLimit => f.write_str("the search outgrew what its indices address"),
            Reason::WorkLimit { limit } => write!(f, "the work limit of {limit} was reached"),
            Reason::Unchecked { limit_bytes } => write!(
                f,
                "a proof was found, but its check would hold more than the memory limit of {}",
                Bytes(*limit_bytes)
            ),
            Reason::WitnessDepth { depth } => {
                write!(f, "the witness depth of {depth} was reached")
            }
            Reason::Instances { instances } => {
                write!(f, "the bound of {instances} instances was reached")
            }
            Reason::Incomplete => {
                f.write_str("no proof was found, but the search may miss proofs here")
            }
            Reason::PathLength { length } => {
                write!(f, "the bound of {length} on the active path was reached")
            }
            Reason::ModelSize { size } => {
                write!(f, "no structure of up to {size} elements refutes it")
            }
            Reason::KripkeSize { worlds, size } => write!(
                f,
                "the search found no Kripke model of up to {worlds} worlds, each of up to {size} \
                 elements, that refutes it"
            ),
        }
    }
}

/// What a search cost.
///
/// [`Engine::counters`] says which counters an engine fills, and what
/// each counts for it. Every engine counts `work`. The counters an engine
/// does not list stay zero.
///
/// In JSON (feature `serialize`), it is an object of the counters by
/// name. A missing one reads back as zero. The first-order counters are
/// written only where they are not zero, so a propositional search's
/// object has no first-order keys.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default))]
#[non_exhaustive]
pub struct Statistics {
    /// The nodes of the search. They are, by engine:
    /// - the focused engines: the stable sequents visited, memo hits
    ///   included;
    /// - the net engine: the literals it chose a partner for;
    /// - the additive path: the pairs of subformulas it decided;
    /// - the Horn engine: the markings it reached, the initial one
    ///   included, with those its backward search reached from the goal
    ///   (in affine mode, the elements it computed backward).
    ///
    /// [`Engine::counters`] lists every engine's.
    pub nodes: u64,
    /// The visits answered from the memo. For the focused engines, a
    /// failure that waits on an open ancestor answers some too. For the
    /// Horn engine, the markings it had kept already, in either direction,
    /// and in affine mode the elements an element kept already covers.
    pub memo_hits: u64,
    /// The most stable sequents the memo held at once.
    ///
    /// - On a pool, whose memo is shards each emptied by itself, it is the
    ///   sum of the shards' peaks, which bounds it from above.
    /// - Of the default bias's two searches, where they ran together, it
    ///   is the two memos' together. Where they took turns from their
    ///   start, it is the most of one turn.
    /// - For the additive path, it is the pairs of subformulas its memo
    ///   held at once.
    /// - For the Horn engine, it is the markings it kept in either
    ///   direction, or in affine mode the elements.
    ///
    /// It is a `u64` on every target, as every counter is.
    pub memo_entries: u64,
    /// The context splits examined for `⊗` and Mix. The counts reject most
    /// of them.
    pub splits: u64,
    /// The axiom links the net engine tried. Each was made, and taken back
    /// again unless it is part of the net found.
    pub links: u64,
    /// The exact acyclicity tests the net engine ran.
    pub tests: u64,
    /// The copy bound the last level of the focused engine's deepening
    /// began under. So it says how far a search without a bound got before
    /// it was stopped.
    ///
    /// It is that of the search that decided, else of the one under
    /// [`Options::copies`] (the backward one of the default bias's two).
    /// It is zero without exponentials and for the other engines.
    pub copies: u32,
    /// The copy bound the last level of the forward search of
    /// [`Bias::Auto`] began under. That search runs within
    /// [`Options::forward_copies`] on a Horn program. It is zero where the
    /// forward search did not run.
    pub forward_copies: u32,
    /// The units of work the search did, in its engine's unit (each
    /// [`Engine`] variant says which).
    ///
    /// Where another engine took the goal over at a limit, or the inverse
    /// method took turns beside the engine, it is every engine's units
    /// together. Where every search runs on the calling thread (one
    /// thread, and the default bias's two searches in
    /// [`Schedule::Turns`]), it is what the [`Progress::work`] of the polls
    /// adds up to. Other threads' units come into [`Progress::done`] only.
    pub work: u64,
    /// The unifications of two literals the focused engines tried on a
    /// first-order sequent, at an identity or against a copy of the
    /// unrestricted zone, whether they unified or not.
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "is_zero"))]
    pub unifications: u64,
    /// The bindings of a metavariable the focused engines made on a
    /// first-order sequent. A binding that an undo took back and the
    /// search made again counts again.
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "is_zero"))]
    pub bindings: u64,
}

/// Returns whether a counter is zero, which its JSON then leaves out.
#[cfg(feature = "serialize")]
#[cfg_attr(
    target_pointer_width = "64",
    expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "serde's skip_serializing_if passes a reference"
    )
)]
fn is_zero(n: &u64) -> bool {
    *n == 0
}

impl Statistics {
    /// Adds the counters of another search, one that ran beside this one
    /// or after it, to these.
    ///
    /// The work done adds up. The memo's peak and the copy bounds reached
    /// are the larger of the two, since each search had a memo and a
    /// deepening of its own.
    pub fn add(&mut self, other: &Statistics) {
        self.add_run(other);
        self.memo_hits += other.memo_hits;
        self.memo_entries = self.memo_entries.max(other.memo_entries);
        self.forward_copies = self.forward_copies.max(other.forward_copies);
        self.work += other.work;
    }

    /// Adds another engine's counters to these, except the memo's. Those
    /// describe a table, not a run, and a parallel search reads them off
    /// the one table its workers share.
    pub(crate) fn add_run(&mut self, other: &Statistics) {
        self.nodes += other.nodes;
        self.splits += other.splits;
        self.links += other.links;
        self.tests += other.tests;
        self.copies = self.copies.max(other.copies);
        self.unifications += other.unifications;
        self.bindings += other.bindings;
    }
}
