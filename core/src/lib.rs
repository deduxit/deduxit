// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

#![doc(
    html_logo_url = "https://deduxit.github.io/logo/square-dark.svg",
    html_favicon_url = "https://deduxit.github.io/logo/favicon-32.png"
)]
//! **dēdūxit** \[deːˈduːk.sɪt\] *Latin, verb*: \"it has derived\", the perfect of *dēdūcō*, to lead down, derive.
//!
//! A collection of logic suites, a module each: [`ordinary`] logic
//! (classical, intuitionistic and minimal, propositional and first-order),
//! [`linear`] logic, [`modal`] logic from K to S5, [`lambda`]-terms, and
//! `rules`, calculi given as rule files (feature `serialize`). Every proof
//! it returns has passed the crate's checker, unless the caller turns the
//! check off, and every countermodel its own check. A proof prints as a
//! derivation, or exports to LaTeX, Typst, SVG, PNG, PDF or Rocq.
//!
//! A sequent of ordinary logic, here intuitionistic logic, the default:
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::ordinary::{Options, Sequent, Verdict, decide};
//!
//! let sequent: Sequent = "a \\/ ~a".parse()?;
//! let outcome = decide(&sequent, &Options::default())?;
//! match &outcome.verdict {
//!     Verdict::Valid(evidence) => println!("valid\n{}", evidence.derivation()?),
//!     Verdict::NotValid => println!("not valid"),
//!     _ => println!("unknown"),
//! }
//! if let Some(countermodel) = &outcome.countermodel {
//!     println!("{}", countermodel.model);
//! }
//! # Ok::<(), deduxit::Error>(())
//! ```
//!
//! A sequent of linear logic:
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::linear::{Mode, Options, Sequent, Verdict, prove};
//!
//! let sequent: Sequent = "A, A -o B |- B".parse()?;
//! let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
//! match &outcome.verdict {
//!     Verdict::Proved(proof) => println!("provable\n{}", proof.derivation()?),
//!     Verdict::Unprovable(disproof) => println!("unprovable: {disproof}"),
//!     Verdict::Unknown(reason) => println!("unknown: {reason}"),
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! `{disproof}` prints why the sequent is unprovable, in the words of the
//! `deduxit` command; `{:?}` would print the whole data. The examples
//! below also use `serde_json`.
//!
//! This is the library of [deduxit](https://github.com/deduxit/deduxit).
//! A guide to the library and the command for AI models is served beside
//! this documentation: its summary as
//! [`llms.txt`](https://deduxit.github.io/deduxit/llms.txt) and its full
//! reference as
//! [`llms-full.txt`](https://deduxit.github.io/deduxit/llms-full.txt).
//!
//! # What you can do
//!
//! - Decide classical, intuitionistic or minimal logic, propositional or
//!   first-order, with a Kripke countermodel ([`kripke::Model`]) or a
//!   finite structure for what is not valid: [`ordinary`].
//! - Translate a classical sequent into intuitionistic or minimal logic,
//!   or find a Craig interpolant, each checked: `ordinary::negative` and
//!   `ordinary::interpolant`.
//! - Check a signed tableau written by hand, step by step:
//!   `ordinary::WrittenTableau` (features `parse`, `serialize` and
//!   `interactive`).
//! - Rewrite a classical sequent into a normal form, each step checked,
//!   and refute its clauses by resolution or DPLL with a checked
//!   refutation: `ordinary::normal` and `ordinary::resolution`; prove a
//!   first-order one by Herbrand's theorem: `ordinary::herbrand`.
//! - Decide a sequent of linear logic: [`linear::prove`], as above. A
//!   [`Mode`](linear::Mode) is the rules a logic allows: classical or
//!   intuitionistic, with weakening (affine) or Mix, commutative or
//!   ordered.
//! - Read intuitionistic linear input: parse it as an
//!   [`IllSequent`](linear::IllSequent) (below).
//! - Check a linear proof, made here or read from a file:
//!   [`Proof::check`](linear::Proof::check).
//! - Work with proof nets: [`ProofStructure`](linear::ProofStructure).
//! - Prove step by step: `linear::Interactive` (feature `interactive`).
//! - Decide propositional modal logic in the systems from K to S5, with a
//!   derivation in a labelled calculus or a checked Kripke countermodel:
//!   [`modal`]. Either becomes a Rocq certificate under `export::rocq`.
//! - Read a proof as a lambda-term, check a term you write, and find and
//!   count a simple type's terms: [`lambda`].
//! - Check, build and draw a derivation in a calculus you give as a rule
//!   file, natural deduction among the built-in ones, as a tree or as a
//!   Fitch proof: `rules` (feature `serialize`).
//! - Bound a search's time, memory and work: [`linear::prove_within`]
//!   with [`Limits`] and a stop (below).
//! - Get the answer the command gives: [`Settings::stop`] (below).
//! - Print or export a derivation: [`Proof::derivation`](linear::Proof::derivation)
//!   and the formats under [`export`].
//! - List what each suite makes and does, and run it over JSON: `protocol`
//!   (feature `protocol`).
//! - Keep sequents and what was made from them, with undo and redo:
//!   `workspace` (feature `protocol`).
//!
//! Proofs, nets and errors name the *occurrences* of a sequent: an
//! occurrence is one place of a subformula in the sequent, and the
//! sequent's [`Forest`](linear::Forest) numbers them.
//!
//! # A proof, its fragment and its JSON
//!
//! Parse a sequent, see which fragment of linear logic it lives in, prove
//! it, print the derivation, then write the proof as JSON, read it back
//! and check it:
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
//! # Intuitionistic input
//!
//! An [`IllSequent`](linear::IllSequent) reads a sequent of intuitionistic
//! linear logic as it is written, with its hypotheses left of `⊢`. Its
//! errors are in that logic's terms: `par` is refused with a hint to
//! write `A -o B`. A [`Sequent`](linear::Sequent) parsed from the same
//! text and proved in an intuitionistic mode would report a subformula
//! instead.
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::linear::{IllSequent, Mode, Options, Verdict, prove};
//!
//! let ill: IllSequent = "A -o B, B -o C |- A -o C".parse()?;
//! let outcome = prove(ill.sequent(), Mode::INTUITIONISTIC, &Options::default())?;
//! assert!(matches!(outcome.verdict, Verdict::Proved(_)));
//!
//! let Err(error) = "A par B |- B".parse::<IllSequent>() else {
//!     panic!("par is not intuitionistic");
//! };
//! assert!(error.to_string().contains("write ~A par B as A -o B"));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Limits and stops
//!
//! [`Options`](linear::Options) are a search's settings, with their
//! defaults. [`Limits`] are the resources a call may use. The crate reads
//! no clock, so a long call takes a *stop*: a closure that the call asks
//! from time to time and that ends the call when it returns true. The
//! call tells it the [`Progress`] made. `|_| false` never stops. The
//! [`limits`] module lists every long call with its bounds and how often
//! it asks its stop.
//!
//! Where there is no clock, as in a browser, a stop can count the `work`
//! done:
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
//! # The command's answers
//!
//! `Options::default()` bounds the copies of a `?` formula at three per
//! branch, so that a call without a stop ends. The `deduxit` command has
//! a clock instead. It searches without a copy bound until its time limit
//! of two seconds, and after a tenth of a second it adds a pool of the
//! machine's other threads. So the two can answer differently.
//!
//! [`Settings::default()`](Settings) holds the command's settings. To
//! decide what the command decides, search with its `search` options and
//! the stop that [`Settings::stop`] makes from its clock. That stop fires
//! at the time limit but lets the refutation after the search run, as the
//! command does. [`Settings::deterministic`] keeps the search on one
//! thread, as the command's `--deterministic` does:
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::{Settings, linear::{Mode, Verdict, prove_within}};
//! use std::time::Instant;
//!
//! let settings = Settings::default().deterministic();
//! let start = Instant::now();
//! let sequent = "!A |- !A * !A".parse()?;
//! let stop = settings.stop(|| start.elapsed());
//! let outcome = prove_within(&sequent, Mode::CLASSICAL, &settings.search, &settings.limits, stop)?;
//! assert!(matches!(outcome.verdict, Verdict::Proved(_)));
//! # Ok::<(), deduxit::Error>(())
//! ```
//!
//! # What each suite offers, over JSON
//!
//! [`protocol`] lists each suite's kinds of values and the operations
//! between them, and runs an operation on JSON values. Here a classical
//! sequent is translated into intuitionistic logic, and the translation's
//! promise, that both are valid together or not, is checked:
//!
#![cfg_attr(feature = "protocol", doc = "```")]
#![cfg_attr(not(feature = "protocol"), doc = "```ignore")]
//! use deduxit::{Settings, protocol};
//!
//! let settings = Settings::default().deterministic();
//! let sequent = r#"{"version": 8, "formulas": [{"V": 0}, {"¬": 0}, {"∨": [0, 1]}],
//!     "atoms": ["a"], "left": [], "right": [2], "logic": "classical"}"#;
//! let glivenko = r#"{"translation": "glivenko"}"#;
//! let translated =
//!     protocol::run("ordinary.translate", &[sequent], glivenko, &settings, |_| false)?;
//! let text = protocol::summary("ordinary.sequent", &translated.value, &settings.limits)?;
//! assert_eq!(text, "⊢ ¬¬(a ∨ ¬a)");
//! let check = protocol::check(
//!     "ordinary.translate", &[sequent], glivenko, &translated.value, &settings, |_| false,
//! )?;
//! assert_eq!(check.status, protocol::Status::Held);
//! # Ok::<(), deduxit::Error>(())
//! ```
//!
//! # Errors
//!
//! Every call that can fail returns an [`Error`], or a more specific
//! error that converts into one without loss:
//! - a [`CheckError`](linear::CheckError) when a proof fails the checker
//!   ([`Invalid`](linear::CheckError::Invalid)) or its check was refused
//!   ([`Refused`](linear::CheckError::Refused));
//! - a [`NetError`](linear::NetError) for a proof structure;
//! - a [`ShapeError`](linear::ShapeError) for a sequent with no
//!   intuitionistic reading;
//! - a `ParseError` for text that is no sequent.
//!
//! [`Error::kind`] says what sort of failure it is. A call that a bound or
//! the stop ended is a refusal ([`ErrorKind::is_refusal`], with the
//! [`Refusal`]), never a fault of its input. [`Error::code`] is a stable
//! name that a program can branch on.
//!
//! Messages name occurrences by number. `describe`, given the forest or
//! other value that the numbers belong to (an [`Owner`]), prints them as
//! formulas ([`Described`]).
//!
//! # Syntax and JSON
//!
//! [`Sequent`](linear::Sequent) says how a sequent is written: the text
//! that `"…".parse::<Sequent>()` reads and its `Display` writes, and its
//! JSON. The JSON forms are the interchange format of the command, of
//! front ends and of files. Those of [`Proof`](linear::Proof),
//! [`Outcome`](linear::Outcome), [`ProofStructure`](linear::ProofStructure)
//! and `Interactive` are described on those types.
//! [`ordinary::Sequent`] says how a sequent of ordinary logic is written.
//!
//! # Features
//!
//! On by default:
//! - `parse`: the text syntaxes, the LLTP reader `linear::lltp`, the
//!   reader of coverability problems `linear::mist`, the generated
//!   `linear::families` and the TPTP reader `ordinary::read_tptp` with
//!   `ordinary::tptp_text`, which decodes a file's bytes for it;
//! - `serialize`: the JSON forms, through serde;
//! - `interactive`: `Interactive`, proving step by step;
//! - `latex`, `typst`, `svg` and `rocq`: the exports under [`export`];
//! - `sat`: the SAT solver batsat, with which `ordinary::sat` decides
//!   classical, intuitionistic and minimal logic;
//! - `protocol`: `protocol`, what each suite offers as data, run over JSON
//!   with `serde_json`, and `workspace`, values linked by its operations
//!   (it needs `serialize`, `parse` and `interactive`).
//!
//! Off by default:
//! - `parallel`: the search on a pool of
//!   [`Options::jobs`](linear::Options::jobs) threads, and a pool kept
//!   across searches, `linear::search::Pool`; never for WebAssembly;
//! - `png` and `pdf`: the SVG drawings rendered.
//!
//! An item that needs a feature says so. Every feature but `parallel`
//! builds for `wasm32-unknown-unknown`: without `parallel` the crate
//! starts no thread, it never reads a clock, and it takes its time limits
//! as a caller's stop. No feature is removed within a 0.y version.
//!
//! # Stability
//!
//! The API follows semantic versioning as Cargo reads it below 1.0: a
//! breaking change raises the second number.
//!
//! Every public enum, and every struct with public fields that may grow,
//! is `#[non_exhaustive]`. The enums of the calculus are closed on
//! purpose, so that a match over them stops compiling when the calculus
//! grows: `Term`, `Kind`, `Node`, `Rule`, [`Verdict`](linear::Verdict),
//! `Branch`, `Sign`, `linear::occurrences::Polarity`, `Side`, those of
//! [`ordinary`]'s derivations, a lambda-term's nodes and types, and
//! `Visit`, the stops of a walk.
//!
//! The JSON forms are promised by their wire level (the module `wire`,
//! feature `serialize`), not by the crate's version. Display text, the
//! defaults' numbers and the engines' counters are not promised. The
//! repository's changelog lists what each release changed.

/// The hash-consed arena of formulas that ordinary and modal logic share.
mod arena;
/// The error types of this crate.
mod errors;
pub mod export;
/// The first-order terms, symbols and binders of a formula arena.
mod first_order;
/// The walks over a proof in progress as a tree of inferences, for the
/// interactive sessions.
#[cfg(feature = "interactive")]
mod goal_tree;
/// The hash tables of this crate.
mod hash;
pub mod kripke;
pub mod lambda;
pub mod limits;
pub mod linear;
pub mod modal;
/// Atom names: identifiers of the text syntax that no keyword takes.
mod name;
pub mod ordinary;
#[cfg(feature = "protocol")]
pub mod protocol;
#[cfg(feature = "serialize")]
pub mod rules;
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
/// Unification of first-order terms with metavariables, for the
/// interactive sessions and first-order resolution.
#[cfg(any(feature = "interactive", feature = "sat"))]
mod unify;
#[cfg(feature = "serialize")]
pub mod wire;
#[cfg(feature = "protocol")]
pub mod workspace;

#[cfg(feature = "parse")]
pub use errors::ParseError;

#[cfg(feature = "interactive")]
pub use errors::WitnessFault;
pub use errors::{Backwards, Described, Error, ErrorKind, Owner, StepFault};
pub use limits::{Limits, Phase, Progress, Refusal};
pub use settings::{Clock, Settings};
