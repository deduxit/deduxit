// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

#![doc(
    html_logo_url = "https://deduxit.github.io/logo/square-dark.svg",
    html_favicon_url = "https://deduxit.github.io/logo/favicon-32.png"
)]
//! **dēdūxit** \[deːˈduːk.sɪt\] *Latin, verb*: \"it has derived\", the perfect of *dēdūcō*, to lead down, derive.
//!
//! Decides sequents of linear logic: parse a sequent, prove it, get a
//! proof the checker has validated, and print or export it. A caller adds
//! the crate from crates.io and, for the examples below, `serde_json`. An
//! *occurrence* is one place of a subformula in a sequent, numbered by the
//! sequent's [`Forest`](linear::Forest); proofs, nets and errors name
//! occurrences by those numbers.
//!
//! The crate is [deduxit](https://github.com/deduxit/deduxit), a
//! collection of logic suites.
//!
//! In more detail: the linear suite, [`linear`], with sequents of linear
//! logic stored as compact arena DAGs in negation normal form, with
//! parsing, printing and serialization; the fragment a sequent lives in
//! and the mode proof search runs in; the occurrence forest of a
//! sequent, the numbering of its subformula occurrences that proof search,
//! proof checking and proof nets are built on; proofs as terms over those
//! occurrences, with the checker that validates them, the derivation view
//! that renders them, and their serialization; proof search, which
//! decides a sequent with the engine its fragment calls for and returns a
//! checked proof; and [`ordinary`] propositional logic, classical,
//! intuitionistic and minimal, decided through its embeddings into linear
//! logic, with the proof read back as a derivation of LK or LJ; and the
//! [`lambda`]-terms of intuitionistic proofs, a proof of intuitionistic
//! linear logic read back as a linear lambda-term and a derivation of LJ
//! as a simply typed one.
//!
//! A guide to the crate and to the `deduxit` command for AI models is
//! served beside this documentation, its summary as
//! [`llms.txt`](https://deduxit.github.io/deduxit/llms.txt) and its
//! full reference as
//! [`llms-full.txt`](https://deduxit.github.io/deduxit/llms-full.txt).
//!
//! # The common path
//!
//! Parse a sequent, see which fragment it lives in, prove it, print the
//! derivation and serialize the proof:
//!
#![cfg_attr(all(feature = "parse", feature = "serialize"), doc = "```")]
#![cfg_attr(not(all(feature = "parse", feature = "serialize")), doc = "```ignore")]
//! use deduxit::linear::{Fragment, Mode, Options, Sequent, Verdict, prove};
//!
//! // Two-sided input becomes one-sided: ⊢ ~A, A ⊗ ~B, B.
//! let sequent: Sequent = "A, A -o B |- B".parse()?;
//! assert_eq!(sequent.fragment(), Fragment::MLL);
//!
//! let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
//! let Verdict::Proved(proof) = &outcome.verdict else {
//!     panic!("the sequent is provable");
//! };
//! assert_eq!(
//!     proof.derivation()?.to_string(),
//!     "─────── ax   ─────── ax\n\
//!      ⊢ ~A, A      ⊢ ~B, B\n\
//!      ──────────────────── ⊗\n\
//!     \x20 ⊢ ~A, A ⊗ ~B, B"
//! );
//!
//! let json = serde_json::to_string(&proof)?;
//! let back: deduxit::linear::Proof = serde_json::from_str(&json)?;
//! back.check(Mode::CLASSICAL)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! A [`Mode`](linear::Mode) is the rules a logic allows, [`Options`](linear::Options) the knobs of a
//! search with their defaults, [`Limits`] the resources a call may use
//! (the [`limits`] module lists every long call with its bounds and how
//! often it asks its stop), and [`prove_within`](linear::prove_within) takes a stop condition
//! for a time limit or an interruption, since this crate has no clock.
//! Each time it asks the stop it tells it the [`Progress`] made, whose
//! `work` a budget can count where there is no clock, as in a browser:
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::{Limits, linear::{Fragment, Mode, Options, Verdict, prove_within}};
//!
//! let mode = Mode::CLASSICAL.with_mix();
//! let options = Options::default().with_fragment(Some(Fragment::MALL));
//! let limits = Limits::default().with_memory_bytes(Some(64 << 20));
//! let mut budget: u64 = 10_000_000;
//! let sequent = "|- A par B, ~A, ~B".parse()?;
//! let outcome = prove_within(&sequent, mode, &options, &limits, |progress| {
//!     budget = budget.saturating_sub(progress.work);
//!     budget == 0
//! })?;
//! assert!(matches!(outcome.verdict, Verdict::Proved(_)));
//! assert_eq!(outcome.fragment, Fragment::MALL);
//! # Ok::<(), deduxit::Error>(())
//! ```
//!
//! **The command's verdicts differ from `Options::default()`'s on purpose.**
//! A library call without a stop must end, so the default options bound
//! the copies of a `?` formula per branch at three; the `deduxit` command
//! has a clock, so it deepens without a bound until its time limit of two
//! seconds, and after a tenth of a second it adds a pool of every other
//! thread beside the first. [`Settings::default()`](Settings) is that
//! behaviour as data: to decide what the command decides, search with its
//! `search` options (no copy bound) and its stop,
//! [`Settings::stop`], which fires at its `clock`'s time limit, on one
//! thread for the command's `--deterministic` answer:
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::{Settings, linear::{Mode, Verdict, prove_within}};
//! use std::time::Instant;
//!
//! let settings = Settings::default();
//! let search = settings.search.clone().with_jobs(1);
//! let start = Instant::now();
//! let sequent = "!A |- !A * !A".parse()?;
//! let stop = settings.stop(|| start.elapsed());
//! let outcome = prove_within(&sequent, Mode::CLASSICAL, &search, &settings.limits, stop)?;
//! assert!(matches!(outcome.verdict, Verdict::Proved(_)));
//! # Ok::<(), deduxit::Error>(())
//! ```
//!
//! Errors are one family, [`Error`]: its [`kind`](Error::kind) says what
//! sort of failure it is, so that a call a bound or a stop refused
//! ([`ErrorKind::is_refusal`], with the [`Refusal`]) is never read as a
//! fault of its input, and its [`code`](Error::code) is the stable reason
//! a program branches on. The types a caller may match on are variants of
//! it, converted without loss: a proof that fails the checker gives a
//! [`CheckError`](linear::CheckError), [`Invalid`](linear::CheckError::Invalid) or
//! [`Refused`](linear::CheckError::Refused); a proof structure a [`NetError`](linear::NetError); a
//! sequent without an intuitionistic reading a [`ShapeError`](linear::ShapeError); a text that
//! is no sequent a `ParseError`. Their messages name occurrences by id,
//! and `describe` on any of them, given the forest or the value the ids
//! belong to (an [`Owner`]), prints them as formulas ([`Described`]).
//!
//! # Syntax and JSON
//!
//! [`Sequent`](linear::Sequent) says how a sequent is written, which is the text that
//! `"…".parse::<Sequent>()` reads and its `Display` writes, and what its
//! JSON is. The JSON forms are the interchange format of the command, of
//! front ends and of files; those of [`Proof`](linear::Proof), [`Outcome`](linear::Outcome),
//! [`ProofStructure`](linear::ProofStructure) and `Interactive` are described on those types.
//! [`ordinary::Sequent`] says how a sequent of ordinary logic is written.
//!
//! # Features
//!
//! On by default: `parse` (the text syntaxes, the LLTP reader `linear::lltp`, the
//! reader of coverability problems `linear::mist`, the generated `linear::families` and
//! the TPTP reader `ordinary::read_tptp`),
//! `serialize` (the JSON forms, through serde), `interactive`
//! (`Interactive`, proving step by step), and the exports
//! `latex`, `typst`, `svg` and `rocq` (under [`export`]). Off by default:
//! `parallel` (the search on a pool of [`Options::jobs`](linear::Options::jobs) threads, and a
//! pool kept across searches, `linear::search::Pool`; never for WebAssembly), and
//! `png` and `pdf` (the SVG drawings rendered). An item that needs a
//! feature says so. Every feature but `parallel` builds for
//! `wasm32-unknown-unknown`: the crate reads no clock, starts no thread
//! without that feature, and takes its time limits as a caller's stop.
//! No feature is removed within a 0.y version.
//!
//! # Stability
//!
//! The API follows semantic versioning as Cargo reads it below 1.0: a
//! break raises the second number. Every public enum and every struct
//! with public fields that may grow is `#[non_exhaustive]`; the enums of
//! the calculus (`Term`, `Kind`, `Node`, `Rule`, [`Verdict`](linear::Verdict), `Branch`,
//! `Sign`, `linear::occurrences::Polarity`, `Side`, those of [`ordinary`]'s
//! derivations, a lambda-term's nodes and types and `Visit`, a walk's
//! stops) are closed on purpose, so
//! that a match over them stops compiling when the calculus grows. The
//! JSON forms of the data are promised by their wire level (the module
//! `wire`, feature `serialize`), not by the crate's version. Display
//! text, the defaults' numbers and the engines' counters are not
//! promised. The repository's changelog states each release's surface.

/// The error types of this crate.
mod errors;
pub mod export;
/// The hash tables of this crate.
mod hash;
pub mod lambda;
pub mod limits;
pub mod linear;
pub mod ordinary;
/// The supertraits that keep the crate's public traits to its own types.
mod sealed {
    /// Seals [`Owner`](crate::Owner): only the crate's types own the
    /// occurrences its errors name.
    #[expect(
        unnameable_types,
        reason = "a sealing supertrait is public, so that it may bound a public trait, and unnameable, so that no other crate implements it"
    )]
    pub trait Owner {
        /// Returns the occurrence of every vertex the owner's errors name,
        /// where its vertices are no occurrences: a structure with boxes.
        fn vertices(&self) -> Option<&[crate::linear::occurrences::OccId]> {
            None
        }

        /// Returns the mode the owner records, whose reading writes the
        /// formulas of its errors.
        fn mode(&self) -> Option<crate::linear::Mode> {
            None
        }

        /// Returns the instance table of the members past the forest
        /// that the owner's errors name.
        fn instances(&self) -> Option<&crate::linear::Instances> {
            None
        }
    }

    /// Seals [`Readable`](crate::wire::Readable): only the crate's forms
    /// are read at a wire level.
    #[cfg(feature = "serialize")]
    #[expect(
        unnameable_types,
        reason = "a sealing supertrait is public, so that it may bound a public trait, and unnameable, so that no other crate implements it"
    )]
    pub trait Readable {}

    /// Seals [`Net`](crate::linear::nets::cut::Net): only the crate's structures
    /// have their cuts eliminated, by the steps this trait takes.
    #[expect(
        unnameable_types,
        reason = "a sealing supertrait is public, so that it may bound a public trait, and unnameable, so that no other crate implements it"
    )]
    pub trait Net: Sized {
        /// Returns the cut links, each from its end in a cut formula's
        /// tree.
        fn cut_links(&self) -> &[(crate::linear::nets::VertexId, crate::linear::nets::VertexId)];

        /// Decides whether the structure is a proof net.
        fn check(
            &self,
            stop: &mut dyn FnMut(crate::limits::Progress) -> bool,
        ) -> Result<(), crate::linear::nets::NetError>;

        /// Reduces the cut link `cut` of a proof net and returns the
        /// structure made, unchecked, and the step, within the bound of
        /// `options` on vertices and `limits.memory_bytes`.
        fn reduce(
            &self,
            cut: (crate::linear::nets::VertexId, crate::linear::nets::VertexId),
            options: &crate::linear::proofs::cut::Options,
            limits: &crate::Limits,
        ) -> Result<(Self, crate::linear::nets::cut::Step), crate::Error>;
    }
}
/// Serde support for the crate's data.
#[cfg(feature = "serialize")]
mod serialize;
pub mod settings;
#[cfg(feature = "serialize")]
pub mod wire;

#[cfg(feature = "parse")]
pub use errors::ParseError;

pub use errors::{Described, Error, ErrorKind, Owner};
pub use limits::{Limits, Phase, Progress, Refusal};
pub use settings::{Clock, Settings};
