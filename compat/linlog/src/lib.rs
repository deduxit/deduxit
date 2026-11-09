// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! **linlog is now [deduxit](https://crates.io/crates/deduxit).** This
//! last release under the old name re-exports it: the root of deduxit and,
//! where linlog 0.2.0 had them, the items of its linear suite,
//! `deduxit::linear`. Depend on `deduxit` instead; its documentation is
//! at <https://docs.rs/deduxit>, and the repository's changelog says what
//! changed in 0.3.0.
//!
//! A path of 0.2.0 still names its item:
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use linlog::{Mode, Options, Sequent, Verdict, prove};
//!
//! let sequent: Sequent = "A, A -o B |- B".parse()?;
//! let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
//! assert!(matches!(outcome.verdict, Verdict::Proved(_)));
//! # Ok::<(), linlog::Error>(())
//! ```

pub use deduxit::linear::*;
pub use deduxit::*;
