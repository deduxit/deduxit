// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The resources a call may use, what a long call tells its stop, and why
//! a call was refused without an answer.
//!
//! Every long call of the crate takes a [`Limits`] and a stop. The stop is
//! a closure `FnMut(Progress) -> bool`. The call asks it after a bounded
//! amount of work, and ends when it returns `true`; `|_| false` never
//! stops. The crate reads no clock. A front end with a deadline reads its
//! own clock in the stop, as often as the work done ([`Progress::work`])
//! says it should. A call that a bound or the stop ends answers with a
//! [`Refusal`], which is never a verdict on its input.
//!
//! The long calls, the fields of [`Limits`] that bound each, and how often
//! each asks its stop:
//!
//! | call | bound by | asks the stop |
//! |---|---|---|
//! | `Sequent::parse_within`, `lltp::read`, `mist::read`, `ordinary::Sequent::parse_within`, `ordinary::read_tptp`, `wire::upgrade`, `wire::Within` | `occurrences` | never: linear in the input |
//! | `Forest::within`, `Interactive::within`, `ordinary::translate` | `occurrences` | never: linear in the bound |
//! | `search::prove_within`, `search::prove_goal` | `memory_bytes`, `recursion_depth`, `occurrences`, `work` (the search's and the check's) | at every node an engine searches, and in its long passes |
//! | `search::batch::run`, `search::batch::prove` | each search's limits, the batch's memory shared out | through each problem's work, and `Results::cancel` |
//! | `Proof::check_within` | `memory_bytes`, `work` | every 4 096 nodes or 65 536 units of work |
//! | `proofs::cut::step`, `proofs::cut::eliminate` | `cut::Options::steps` and `nodes`; `memory_bytes` and `work` for each check, `memory_bytes` for a step's specialized erasure and the proof it makes | between steps, as each check does, and every 4 096 places a specialized erasure makes |
//! | `Proof::derivation_size`, `Proof::derivation_within`, `Interactive::derivation_within` | `derivation_bytes`, `memory_bytes` | as the check, then at every inference built |
//! | `Interactive::close`, `close_with`, `close_all`, `proof` | the search's and the derivation's | as the search and the derivation |
//! | `ProofStructure::is_correct` | none: its memory is linear in the structure | at every round of the criterion |
//! | `ProofStructure::from_proof`, `ProofStructure::sequentialize` | `memory_bytes` | at every node, at every stage |
//! | `ordinary::Image::read_back`, `ordinary::Derivation::check` | `derivation_bytes`, `memory_bytes`; `work` inferences checked | at every inference |
//! | `ordinary::decide` | all of the above it runs | as each step |
//! | `ordinary::Interactive::within` | `occurrences` | never: linear in the bound |
//! | `ordinary::Interactive::apply` | `derivation_bytes` of the start, for the run of structural rules an axiom or a shared rule makes, by an estimate before it is made | never: within the bound |
//! | `ordinary::Interactive::close`, `hint` | as `ordinary::decide`; `derivation_bytes` for the graft | as `ordinary::decide` |
//! | `ordinary::Interactive::derivation_within`, `finished` | `derivation_bytes` by an estimate before anything is built; `work` for the check | before building, then as the check |
//! | `export::{latex, typst, svg, rocq}::write`, `Derivation::write_text`, `write_steps` | the derivation's, when it was made | at every inference written |
//! | `export::{latex, typst, svg}::sequent` | `derivation_bytes`, by an estimate before anything is laid out | never: linear in the estimate |
//! | `export::svg::net` | `derivation_bytes` | as the criterion |
//! | `export::{png, pdf}::from_svg` | `memory_bytes`, the pixel bound of the options | never: refused before the render starts |

use crate::errors::counted;
use std::fmt::{self, Display, Formatter};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// The resources a call may use, which the library enforces. `None` in an
/// optional field means no bound. The defaults are the named constants.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Serialize, Deserialize),
    serde(default, deny_unknown_fields)
)]
pub struct Limits {
    /// The most bytes a call holds at once: a search's growing structures,
    /// a check's pass, a derivation's making, a render's estimate.
    ///
    /// A search counts its input too, the sequent and its forest. Other
    /// calls do not: there the input is bounded by
    /// [`occurrences`](Self::occurrences). The proof returned, the threads'
    /// stacks and the allocator's own overhead are never counted.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub memory_bytes: Option<u64>,
    /// The most subformula occurrences a sequent may unfold to, on every
    /// reader and wherever a forest is built. The text readers also refuse
    /// a text nested deeper than the bound (more `(` or `~` open at once),
    /// as if it had that many occurrences.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub occurrences: Option<u64>,
    /// The most first-order terms a sequent's atoms may unfold to
    /// ([`Sequent::fo_size`](crate::linear::Sequent::fo_size)), and the most
    /// entries a proof's instance table may hold, on every reader. A short
    /// document of shared terms can name an atom of exponential length.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub terms: Option<u64>,
    /// The most bytes a derivation, or the drawing of a net or a sequent,
    /// may be estimated at. Past it nothing is made.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub derivation_bytes: Option<u64>,
    /// The most units of work a call may do. The unit depends on the
    /// call:
    /// - the proof checker counts a node, or an entry of a sequent its
    ///   rules handle;
    /// - the ordinary checker counts an inference;
    /// - a search counts its engine's unit, which each
    ///   [`Engine`](crate::linear::search::Engine) variant names. Where
    ///   another engine takes a goal over at an engine's limit, the units of
    ///   both count together.
    ///
    /// A search past the bound is unknown with `Reason::WorkLimit`, and so
    /// is a search whose proof's check passes it. A check alone refuses
    /// with `Error::Check(CheckError::Refused)`.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub work: Option<u64>,
    /// The most atoms a truth table may have: its rows double with each.
    /// [`DEFAULT_TABLE_ATOMS`](Self::DEFAULT_TABLE_ATOMS) by default.
    pub table_atoms: Option<u32>,
    /// The most subformula occurrences a normal form may unfold to, and
    /// each sequent on the way to it (`deduxit::ordinary::normal`):
    /// distribution can make a formula exponentially longer.
    /// [`DEFAULT_FORM_OCCURRENCES`](Self::DEFAULT_FORM_OCCURRENCES) by
    /// default.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub form_occurrences: Option<u64>,
    /// The deepest recursion on any one stack. Every thread the library
    /// starts is sized for it ([`stack_bytes`](Self::stack_bytes)).
    ///
    /// The focused engine takes a level per asynchronous phase, per stable
    /// sequent and per focus. A chain of `⅋`, `⊥` and `?` rules, or of
    /// forced splits, takes none. So a formula copied `k` times on a
    /// branch takes `k` focus levels. The additive path takes a level per
    /// pair.
    pub recursion_depth: u32,
    /// The bounds of a workspace of linked values
    /// (`deduxit::workspace`), each refused at its edge.
    pub workspace: WorkspaceLimits,
}

impl Limits {
    /// The default memory bound: one gibibyte.
    pub const DEFAULT_MEMORY_BYTES: u64 = 1 << 30;
    /// The default bound on occurrences: fifty million, above the largest
    /// problem of the LLTP library (27.8 million). A forest takes about
    /// 25 bytes an occurrence.
    pub const DEFAULT_OCCURRENCES: u64 = 50_000_000;
    /// The default bound on first-order terms: as many as occurrences, so
    /// that a sequent's terms take no more room than its forest.
    pub const DEFAULT_TERMS: u64 = 50_000_000;
    /// The default bound on a derivation's estimated size: 64 mebibytes.
    pub const DEFAULT_DERIVATION_BYTES: u64 = 64 << 20;
    /// The default bound on a truth table's atoms: ten, so 1024 rows, about
    /// as many as a reader scans; a table that large is refused unless the
    /// bound is raised.
    pub const DEFAULT_TABLE_ATOMS: u32 = 10;
    /// The default bound on a normal form's occurrences: ten thousand,
    /// some pages of text, which every step of the rewriting is shown
    /// with; a larger form is refused unless the bound is raised.
    pub const DEFAULT_FORM_OCCURRENCES: u64 = 10_000;
    /// The default recursion depth: 2048 levels, which fit a main
    /// thread's stack of 8 MiB in an optimized build.
    pub const DEFAULT_RECURSION_DEPTH: u32 = 2048;

    /// No bound but the default recursion depth, which every stack needs.
    pub const UNBOUNDED: Self = Self {
        memory_bytes: None,
        occurrences: None,
        terms: None,
        derivation_bytes: None,
        work: None,
        table_atoms: None,
        form_occurrences: None,
        recursion_depth: Self::DEFAULT_RECURSION_DEPTH,
        workspace: WorkspaceLimits::UNBOUNDED,
    };

    /// The stack one level of the search's recursion may take: twice the
    /// most it was measured to take on x86-64. The deepest was a Mix of
    /// pairs whose parts are recursed into (6.5 KiB unoptimized, 1.4 KiB
    /// optimized), against a chain of tensors whose splits are searched
    /// (5.8 KiB, 1.2 KiB) and nested `&` (1.4 KiB, 0.4 KiB).
    const PER_LEVEL: usize = if cfg!(debug_assertions) { 13312 } else { 3072 };

    /// What a thread needs besides the recursion: the frames of its caller,
    /// the search's set-up and the check of its proof.
    const RESERVE: usize = 1 << 20;

    /// Returns the stack a thread needs to recurse to
    /// [`recursion_depth`](Self::recursion_depth), at least a main
    /// thread's 8 MiB.
    #[must_use]
    pub const fn stack_bytes(&self) -> usize {
        let needed = (self.recursion_depth as usize)
            .saturating_mul(Self::PER_LEVEL)
            .saturating_add(Self::RESERVE);
        if needed > 8 << 20 { needed } else { 8 << 20 }
    }

    /// Returns the deepest recursion a stack of `bytes` holds. It is the
    /// inverse of [`stack_bytes`](Self::stack_bytes) above its floor of
    /// 8 MiB.
    #[must_use]
    pub const fn recursion_depth_for_stack(bytes: usize) -> u32 {
        let levels = bytes.saturating_sub(Self::RESERVE) / Self::PER_LEVEL;
        if levels > u32::MAX as usize {
            u32::MAX
        } else {
            levels as u32
        }
    }

    /// Returns the limits with another memory bound.
    #[must_use]
    pub const fn with_memory_bytes(mut self, memory_bytes: Option<u64>) -> Self {
        self.memory_bytes = memory_bytes;
        self
    }

    /// Returns the limits with another bound on occurrences.
    #[must_use]
    pub const fn with_occurrences(mut self, occurrences: Option<u64>) -> Self {
        self.occurrences = occurrences;
        self
    }

    /// Returns the limits with another bound on first-order terms.
    #[must_use]
    pub const fn with_terms(mut self, terms: Option<u64>) -> Self {
        self.terms = terms;
        self
    }

    /// Returns the limits with another bound on a derivation's size.
    #[must_use]
    pub const fn with_derivation_bytes(mut self, derivation_bytes: Option<u64>) -> Self {
        self.derivation_bytes = derivation_bytes;
        self
    }

    /// Returns the limits with another bound on a search's work.
    #[must_use]
    pub const fn with_work(mut self, work: Option<u64>) -> Self {
        self.work = work;
        self
    }

    /// Returns the limits with another bound on a truth table's atoms.
    #[must_use]
    pub const fn with_table_atoms(mut self, table_atoms: Option<u32>) -> Self {
        self.table_atoms = table_atoms;
        self
    }

    /// Returns the limits with another bound on a normal form's
    /// occurrences.
    #[must_use]
    pub const fn with_form_occurrences(mut self, form_occurrences: Option<u64>) -> Self {
        self.form_occurrences = form_occurrences;
        self
    }

    /// Returns the limits with another recursion depth.
    #[must_use]
    pub const fn with_recursion_depth(mut self, recursion_depth: u32) -> Self {
        self.recursion_depth = recursion_depth;
        self
    }

    /// Returns the limits with other bounds of a workspace.
    #[must_use]
    pub const fn with_workspace(mut self, workspace: WorkspaceLimits) -> Self {
        self.workspace = workspace;
        self
    }
}

impl Default for Limits {
    /// Returns the default bounds, each a named constant.
    fn default() -> Self {
        Self {
            memory_bytes: Some(Self::DEFAULT_MEMORY_BYTES),
            occurrences: Some(Self::DEFAULT_OCCURRENCES),
            terms: Some(Self::DEFAULT_TERMS),
            derivation_bytes: Some(Self::DEFAULT_DERIVATION_BYTES),
            work: None,
            table_atoms: Some(Self::DEFAULT_TABLE_ATOMS),
            form_occurrences: Some(Self::DEFAULT_FORM_OCCURRENCES),
            recursion_depth: Self::DEFAULT_RECURSION_DEPTH,
            workspace: WorkspaceLimits::default(),
        }
    }
}

/// The bounds of a workspace: how much of each thing it holds at once.
/// `None` in a field means no bound.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Serialize, Deserialize),
    serde(default, deny_unknown_fields)
)]
pub struct WorkspaceLimits {
    /// The most nodes alive.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub nodes: Option<u64>,
    /// The most records of runs kept; the oldest that no node shows and no
    /// edit names give way first.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub records: Option<u64>,
    /// The most edits that undo can take back; the oldest is forgotten
    /// past it, and a document past it is refused.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub log: Option<u64>,
    /// The most bytes of the values held, as their JSON's length.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub bytes: Option<u64>,
}

impl WorkspaceLimits {
    /// The default bound on nodes.
    pub const DEFAULT_NODES: u64 = 1024;
    /// The default bound on records.
    pub const DEFAULT_RECORDS: u64 = 4096;
    /// The default bound on the edits undo keeps.
    pub const DEFAULT_LOG: u64 = 256;
    /// The default bound on the values' bytes: 64 mebibytes.
    pub const DEFAULT_BYTES: u64 = 64 << 20;

    /// No bound.
    pub const UNBOUNDED: Self = Self {
        nodes: None,
        records: None,
        log: None,
        bytes: None,
    };

    /// Returns the bounds with another bound on nodes.
    #[must_use]
    pub const fn with_nodes(mut self, nodes: Option<u64>) -> Self {
        self.nodes = nodes;
        self
    }

    /// Returns the bounds with another bound on records.
    #[must_use]
    pub const fn with_records(mut self, records: Option<u64>) -> Self {
        self.records = records;
        self
    }

    /// Returns the bounds with another bound on the edits kept.
    #[must_use]
    pub const fn with_log(mut self, log: Option<u64>) -> Self {
        self.log = log;
        self
    }

    /// Returns the bounds with another bound on bytes.
    #[must_use]
    pub const fn with_bytes(mut self, bytes: Option<u64>) -> Self {
        self.bytes = bytes;
        self
    }
}

impl Default for WorkspaceLimits {
    /// Returns the default bounds, each a named constant.
    fn default() -> Self {
        Self {
            nodes: Some(Self::DEFAULT_NODES),
            records: Some(Self::DEFAULT_RECORDS),
            log: Some(Self::DEFAULT_LOG),
            bytes: Some(Self::DEFAULT_BYTES),
        }
    }
}

/// What a [`Refusal::Workspace`] counts.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Held {
    /// Nodes alive.
    Nodes,
    /// Records of runs kept.
    Records,
    /// Edits in the log, of a document read.
    Log,
    /// Bytes of the values held.
    Bytes,
}

impl Held {
    /// Returns the settings key of its bound.
    pub const fn setting(self) -> &'static str {
        match self {
            Self::Nodes => "limits.workspace.nodes",
            Self::Records => "limits.workspace.records",
            Self::Log => "limits.workspace.log",
            Self::Bytes => "limits.workspace.bytes",
        }
    }

    /// Returns its name in the plural, for a message.
    const fn plural(self) -> &'static str {
        match self {
            Self::Nodes => "nodes",
            Self::Records => "records",
            Self::Log => "edits",
            Self::Bytes => "bytes",
        }
    }
}

/// What a long call tells its stop each time it asks it.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Progress {
    /// The units of work done since the previous poll, in the unit of
    /// [`Limits::work`].
    pub work: u64,
    /// The units of work done since the call began.
    pub done: u64,
    /// The bytes the call holds now, by its own count. The proof checker's
    /// pass and the calls built on it fill it in. The net calls give their
    /// estimate, and the criterion alone its working memory. A search
    /// leaves it zero, since the two searches of its default bias each
    /// count their own share.
    pub held_bytes: u64,
    /// What the call is doing.
    pub phase: Phase,
    /// The place of the item being worked on, in a call that runs several:
    /// a goal of a session's `close_all`. Zero for every other call.
    pub item: u32,
}

impl Progress {
    /// Returns the progress of a call in `phase` that has done `done`
    /// units of work, `work` of them since the previous poll.
    pub(crate) const fn new(phase: Phase, work: u64, done: u64) -> Self {
        Self {
            work,
            done,
            held_bytes: 0,
            phase,
            item: 0,
        }
    }
}

/// What a long call is doing when it asks its stop or is refused.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Phase {
    /// Reading an input.
    #[default]
    Read,
    /// Searching for a proof.
    Search,
    /// Refuting a sequent after a search.
    Refute,
    /// Checking a proof.
    Check,
    /// Making a derivation.
    View,
    /// Writing a derivation in a format.
    Write,
    /// Rendering a drawing.
    Render,
    /// Building or judging a proof net.
    Net,
    /// Reading a proof back as one of ordinary logic or as a lambda-term.
    ReadBack,
    /// Eliminating the cuts of a proof.
    Eliminate,
    /// Rewriting a formula into a normal form.
    Rewrite,
}

impl Phase {
    /// Returns the phase as a lowercase noun phrase, as messages use it.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Read => "reading",
            Self::Search => "search",
            Self::Refute => "refutation",
            Self::Check => "check",
            Self::View => "derivation",
            Self::Write => "writing",
            Self::Render => "render",
            Self::Net => "net",
            Self::ReadBack => "read-back",
            Self::Eliminate => "elimination",
            Self::Rewrite => "rewriting",
        }
    }
}

/// What a count of [`Refusal::Index`] counts: the spaces whose indices
/// have a fixed width.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Space {
    /// The atoms of a sequent.
    Atom,
    /// The terms of a sequent's arena.
    Term,
    /// The nodes of a proof.
    Node,
    /// The occurrences of a forest.
    Occurrence,
    /// The members of a sequent of a proof, a derivation or a session.
    Member,
    /// The vertices of a proof structure.
    Vertex,
    /// The inferences of a derivation.
    Inference,
    /// The formulas of an ordinary arena.
    Formula,
    /// The symbols of a sequent's first-order tables.
    Symbol,
    /// The worlds of a Kripke model.
    World,
    /// The scopes of a derivation in a calculus given as data.
    Scope,
}

impl Space {
    /// Returns the noun that names one member of the space.
    pub(crate) const fn singular(self) -> &'static str {
        match self {
            Self::Atom => "atom",
            Self::Term => "term",
            Self::Node => "node",
            Self::Occurrence => "occurrence",
            Self::Member => "member",
            Self::Vertex => "vertex",
            Self::Inference => "inference",
            Self::Formula => "formula",
            Self::Symbol => "symbol",
            Self::World => "world",
            Self::Scope => "scope",
        }
    }

    /// Returns the plural noun that counts members of the space.
    pub const fn plural(self) -> &'static str {
        match self {
            Self::Atom => "atoms",
            Self::Term => "terms",
            Self::Node => "nodes",
            Self::Occurrence => "occurrences",
            Self::Member => "members",
            Self::Vertex => "vertices",
            Self::Inference => "inferences",
            Self::Formula => "formulas",
            Self::Symbol => "symbols",
            Self::World => "worlds",
            Self::Scope => "scopes",
        }
    }
}

/// Which bound of cut elimination a [`Refusal::Elimination`] reached.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Bound {
    /// The number of steps.
    Steps,
    /// The nodes of a proof a step makes.
    Nodes,
    /// The vertices of a proof net a step makes.
    Vertices,
}

/// Why a call was given up without an answer: a bound, set or default,
/// refused it, or the caller's stop ended it. A refusal says nothing of
/// the input. With the bound lifted, the call may answer.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(Serialize),
    serde(tag = "kind", rename_all = "snake_case")
)]
pub enum Refusal {
    /// The caller's stop ended the call.
    #[non_exhaustive]
    Stopped {
        /// What the call was doing.
        phase: Phase,
    },
    /// The call would have held more memory than its bound.
    #[non_exhaustive]
    Memory {
        /// What the call was doing.
        phase: Phase,
        /// The bound, [`Limits::memory_bytes`].
        limit_bytes: u64,
        /// What the call would have needed, where it was estimated before
        /// anything was made.
        needed_bytes: Option<u64>,
    },
    /// A sequent unfolds to more occurrences than the bound. The count is
    /// exact, or the number at which counting stopped.
    #[non_exhaustive]
    Occurrences {
        /// How many occurrences, or a number past the bound.
        occurrences: u64,
        /// The bound, [`Limits::occurrences`].
        limit: u64,
    },
    /// A sequent's atoms unfold to more first-order terms than the bound,
    /// or a proof's instance table holds more entries. The count is exact,
    /// or the number at which counting stopped.
    #[non_exhaustive]
    Terms {
        /// How many terms or entries, or a number past the bound.
        terms: u64,
        /// The bound, [`Limits::terms`].
        limit: u64,
    },
    /// An output is estimated past its bound. Nothing was made.
    #[non_exhaustive]
    Output {
        /// What was to be made: a derivation, a drawing.
        what: &'static str,
        /// The estimate.
        estimate_bytes: u64,
        /// The bound, [`Limits::derivation_bytes`].
        limit_bytes: u64,
        /// The least a smaller form of the output would take, where one
        /// was tried: a compact derivation's lower bound.
        least_bytes: Option<u64>,
    },
    /// A call did the most work its bound allows.
    #[non_exhaustive]
    Work {
        /// The bound, [`Limits::work`].
        limit: u64,
    },
    /// An image would have more pixels than its options allow.
    #[non_exhaustive]
    Pixels {
        /// The image's pixels.
        pixels: u64,
        /// The bound of the image's options.
        limit: u64,
    },
    /// Cut elimination reached its bound on steps or on a proof's nodes.
    #[non_exhaustive]
    Elimination {
        /// The bound reached.
        what: Bound,
        /// Its value, [`cut::Options::steps`](crate::linear::proofs::cut::Options::steps)
        /// or [`cut::Options::nodes`](crate::linear::proofs::cut::Options::nodes).
        limit: u64,
    },
    /// A truth table would have more atoms than the bound, and so more
    /// rows than it allows.
    #[non_exhaustive]
    Atoms {
        /// The sequent's atoms.
        atoms: u64,
        /// The bound, [`Limits::table_atoms`].
        limit: u64,
    },
    /// A workspace would hold more than a bound of
    /// [`Limits::workspace`] allows.
    #[non_exhaustive]
    Workspace {
        /// What is counted.
        what: Held,
        /// The count.
        count: u64,
        /// The bound.
        limit: u64,
    },
    /// A count passes what a representation's indices can name, whatever
    /// the bounds say.
    #[non_exhaustive]
    Index {
        /// What is counted.
        what: Space,
        /// The count, or a number past the most.
        count: u64,
        /// The most the representation holds.
        most: u64,
    },
    /// A normal form, or a sequent on the way to it, would unfold to more
    /// occurrences than the bound. The count is the first past it.
    #[non_exhaustive]
    Form {
        /// The occurrences the rewriting reached.
        occurrences: u64,
        /// The bound, [`Limits::form_occurrences`].
        limit: u64,
    },
}

impl Refusal {
    /// Returns the stable name of the refusal that a program branches on.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Stopped { .. } => "stopped",
            Self::Memory { .. } => "memory_limit",
            Self::Occurrences { .. } => "too_many_occurrences",
            Self::Terms { .. } => "too_many_terms",
            Self::Output { .. } => "output_too_large",
            Self::Work { .. } => "work_limit",
            Self::Pixels { .. } => "too_many_pixels",
            Self::Elimination { .. } => "elimination_limit",
            Self::Atoms { .. } => "too_many_atoms",
            Self::Workspace { .. } => "workspace_limit",
            Self::Index { .. } => "index_limit",
            Self::Form { .. } => "form_too_large",
        }
    }

    /// Returns the settings key whose bound refused the call. `None` for
    /// the stop and for a representation's width, which no setting lifts.
    pub const fn setting(&self) -> Option<&'static str> {
        match self {
            Self::Memory { .. } => Some("limits.memory_bytes"),
            Self::Occurrences { .. } => Some("limits.occurrences"),
            Self::Terms { .. } => Some("limits.terms"),
            Self::Output { .. } => Some("limits.derivation_bytes"),
            Self::Work { .. } => Some("limits.work"),
            Self::Atoms { .. } => Some("limits.table_atoms"),
            Self::Form { .. } => Some("limits.form_occurrences"),
            Self::Workspace { what, .. } => Some(what.setting()),
            Self::Pixels { .. } => Some("styles.png.pixels"),
            Self::Elimination {
                what: Bound::Steps, ..
            } => Some("cut.steps"),
            Self::Elimination {
                what: Bound::Nodes | Bound::Vertices,
                ..
            } => Some("cut.nodes"),
            Self::Stopped { .. } | Self::Index { .. } => None,
        }
    }

    /// Returns whether the caller's stop ended the call, not a bound.
    pub const fn is_stop(&self) -> bool {
        matches!(self, Self::Stopped { .. })
    }
}

impl Display for Refusal {
    /// Writes the reason in a sentence without a full stop.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stopped { phase } => write!(f, "the {} was stopped", phase.name()),
            Self::Memory {
                phase,
                limit_bytes,
                needed_bytes: Some(needed),
            } => write!(
                f,
                "the {} would take {}, more than the memory limit of {}",
                phase.name(),
                counted(*needed, "byte", "bytes"),
                counted(*limit_bytes, "byte", "bytes")
            ),
            Self::Memory {
                phase,
                limit_bytes,
                needed_bytes: None,
            } => write!(
                f,
                "the {} takes more than the memory limit of {}",
                phase.name(),
                counted(*limit_bytes, "byte", "bytes")
            ),
            Self::Occurrences { occurrences, limit } => write!(
                f,
                "the sequent unfolds to at least {}, more than the limit of {limit}",
                counted(
                    *occurrences,
                    "subformula occurrence",
                    "subformula occurrences"
                )
            ),
            Self::Terms { terms, limit } => write!(
                f,
                "the first-order terms unfold to at least {}, more than the limit of {limit}",
                counted(*terms, "term", "terms")
            ),
            Self::Output {
                what,
                estimate_bytes,
                limit_bytes,
                least_bytes,
            } => {
                write!(
                    f,
                    "the {what} is estimated at {}, more than the limit of {limit_bytes}",
                    counted(*estimate_bytes, "byte", "bytes")
                )?;
                match least_bytes {
                    Some(least) => write!(
                        f,
                        ", and a compact one at {} at least",
                        counted(*least, "byte", "bytes")
                    ),
                    None => Ok(()),
                }
            }
            Self::Work { limit } => {
                write!(
                    f,
                    "the call did the most work allowed, {}",
                    counted(*limit, "unit", "units")
                )
            }
            Self::Pixels { pixels, limit } => write!(
                f,
                "the image would have {}, more than the limit of {limit}",
                counted(*pixels, "pixel", "pixels")
            ),
            Self::Elimination {
                what: Bound::Steps,
                limit,
            } => write!(f, "the elimination took the most steps allowed, {limit}"),
            Self::Elimination {
                what: Bound::Nodes,
                limit,
            } => write!(
                f,
                "the next step of the elimination makes a proof of more than {}",
                counted(*limit, "node", "nodes")
            ),
            Self::Elimination {
                what: Bound::Vertices,
                limit,
            } => write!(
                f,
                "the next step of the elimination makes a net of more than {}",
                counted(*limit, "vertex", "vertices")
            ),
            Self::Atoms { atoms, limit } => write!(
                f,
                "the truth table of {} would have 2^{atoms} rows, more than the limit of {} \
                 allows",
                counted(*atoms, "atom", "atoms"),
                counted(*limit, "atom", "atoms")
            ),
            Self::Workspace { what, count, limit } => write!(
                f,
                "the workspace would hold {count} {}, more than the limit of {limit}",
                what.plural()
            ),
            Self::Index { what, count, most } => write!(
                f,
                "{count} {} are more than this representation can index ({most})",
                what.plural()
            ),
            Self::Form { occurrences, limit } => write!(
                f,
                "the normal form would grow to {occurrences} occurrences, more than the \
                 limit of {limit}"
            ),
        }
    }
}

impl std::error::Error for Refusal {}

/// Turns a stop into the plain condition that a loop asks once per unit
/// of work in `phase`. Each call tells the stop one unit and the units
/// done.
pub(crate) fn counting(
    mut stop: impl FnMut(Progress) -> bool,
    phase: Phase,
) -> impl FnMut() -> bool {
    let mut done = 0u64;
    move || {
        done = done.saturating_add(1);
        stop(Progress::new(phase, 1, done))
    }
}
