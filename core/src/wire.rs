// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The wire forms: how a value crosses a process boundary, as JSON or any
//! format serde writes, and the rule that keeps a stored document
//! readable by later releases. Needs the cargo feature `serialize` (on by
//! default).
//!
//! # The level
//!
//! Every top-level document starts with `"version": n`, the lowest wire
//! level whose reader understands all of it. [`LEVEL`] is the highest
//! level this build reads and writes. The writer computes the level from
//! the content, so a value that a later release represents as this one
//! does keeps its bytes.
//!
//! | level | what it adds | what is written at it |
//! |---|---|---|
//! | 1 | the forms of 0.1.0 | every document that no row below names |
//! | 2 | the classical refutation, `{"kind": "classical", "assignment": […]}`; the ordered modes: `"mode": "cyclic"`, `"lambek"` and `"lambek-star"`, the rules of the Lambek calculus's divisions, and `"order": "cyclic"` in a proof structure | a disproof or unprovable outcome that holds a classical refutation; a document of an ordered mode |
//! | 3 | the refutation by a saturated set, `{"kind": "saturated", "positive": […], "sequents": […]}` | a disproof or unprovable outcome that holds one |
//! | 4 | the proof structure with boxes, whose `"vertices"` key makes its links name vertices, not occurrences | every such structure; the reader of a proof structure refuses a document with that key |
//! | 5 | the intuitionistic sequent, a form whose terms are the connectives of intuitionistic linear logic | every such sequent |
//! | 6 | the cut: a sequent's cut formulas, `"cuts": […]`, without which a proof's cut nodes would name occurrences past its forest, and the proof's tag `cut` | a document whose sequent holds a cut formula: a proof with a cut, the record of a cut elimination |
//! | 7 | the first-order sequent: `"symbols"`, `"fo"`, `"binders"`, an atom as `{"P": [symbol, [terms]]}`, the terms `∀` and `∃` | a document whose sequent is first-order |
//! | 8 | the forms of ordinary logic: a sequent, a derivation of LK or LJ, the outcome of deciding a sequent, a proof in progress | each of those |
//! | 9 | LJ's cut, `"rule": "cut"` | an ordinary derivation, outcome or proof in progress that holds a cut |
//! | 10 | a sequent valid by a clausal proof, `"clausal_proof": {…}` in place of the derivation | an ordinary outcome that holds one |
//!
//! A reader refuses a document above its level by name ([`Error::Version`],
//! code `unsupported_version`). It does so before reading the other keys
//! when `version` comes first, as the writer puts it. A document without
//! `version` reads as level 1. Nested values carry no `version`.
//!
//! These raise the level: a new tag; a new value of an enumeration that a
//! reader refuses when it does not know it (a mode, a criterion, the kind
//! of a refutation or a reason); a key whose presence changes what the
//! other keys mean. A key that an older reader may ignore without
//! misreading (a counter, an error's detail) does not.
//!
//! # Reading
//!
//! [`upgrade`] is the one reader of every form. It reads a document of any
//! level up to [`LEVEL`] as a value of this release, and counts every
//! sequent's occurrences against `limits.occurrences` before anything
//! unfolds. An older level would be read through one step per level that
//! changed a form; no level has changed one, since each only added kinds,
//! modes, forms, keys, tags or a rule. [`Within`] does the same as a serde
//! seed, for a value nested in a document of the caller's. The plain
//! `Deserialize` impls read within the default [`Limits`].
//!
//! A data form (a sequent, a proof, a structure, a session) ignores the
//! keys it does not know, which lets an outcome be read as a proof. An
//! options form refuses them by name. The keys every level-1 value has
//! are required, so a document of another shape is refused naming the key
//! ([`Error::Json`]). Documents from before 0.1.0 (with the keys `ids`,
//! `var_dict` or `proof`, or a mode as an object of flags) are not read.
//!
//! # The forms
//!
//! Enumerations are written by name, never by number. Integers are exact
//! in JavaScript: a bound past 2⁵³ is written `null`, and a count that
//! saturates is written `u64::MAX`, meaning "at least". The examples use
//! `A, A -o B |- B`, which is `⊢ ~A, A ⊗ ~B, B` one-sided:
//!
//! - **Sequent**: `{"version": 1, "terms": [{"D": 0}, {"V": 0}, {"D": 1},
//!   {"⊗": [1, 2]}, {"V": 1}], "roots": [0, 3, 4], "atoms": ["A", "B"],
//!   "antecedents": 2}`, see [`Sequent`](crate::linear::Sequent).
//! - **Proof**: `{"version": 1, "sequent": {…}, "nodes": [{"ax": [0, 2]},
//!   {"ax": [3, 4]}, {"⊗": [1, 0, 1]}], "mode": "classical"}`, with `goal`
//!   for a proof of a goal off the roots, see [`Proof`](crate::linear::Proof). A
//!   proof with cuts is written at level 6: its sequent holds `"cuts": […]`
//!   and a cut node is `{"cut": [a, l, r]}`, `a` the occurrence of the cut
//!   formula. A proof of a first-order sequent is written at level 7, as
//!   its sequent is: the nodes `{"∀": [m, a, p]}` (a member, its
//!   eigenvariable, the premise) and `{"∃": [m, w, p]}` (a member, its
//!   witness, a term, the premise), and `"instances": {"members":
//!   [[occurrence, frame], …], "frames": [[parent, term], …], "terms":
//!   […]}`, the members past the forest, the frames with the innermost
//!   binder's term first and `null` past the outermost, and the proof's
//!   terms numbered on from the sequent's `fo`, an eigenvariable `{"E":
//!   k}`; see [`Instances`](crate::linear::Instances).
//! - **Disproof**: `{"version": 1, "sequent": {…}, "mode": "classical",
//!   "refutation": {"kind": "unbalanced", "atom": 0, "least": 2, "most":
//!   2}}`, with `goal` for a goal off the roots, see
//!   [`Disproof`](crate::linear::Disproof).
//! - **Proof structure**: `{"version": 1, "sequent": {…}, "mix": false,
//!   "links": [[0, 2], [3, 4]]}`, see
//!   [`ProofStructure`](crate::linear::ProofStructure). A structure with cut
//!   links is written at level 6, its sequent holding the cut formulas:
//!   `"cuts": [[2, 5]]`, each cut link by its two vertices, a key written
//!   only where there are cut links, as in a structure with boxes.
//! - **Proof structure with boxes**: `{"version": 4, "sequent": {…},
//!   "mix": false, "vertices": [[3, 1, 1], …], "links": [[2, 3]],
//!   "weakenings": []}`: the roots are vertices `0` to `r − 1`, then each
//!   of `vertices` as `[occurrence, parent, box]`, `box` the `!` vertex of
//!   its innermost box or `null`; `links` and `weakenings` (a `?` vertex
//!   without copies and the literal it hangs on) name vertices; see
//!   [`MellStructure`](crate::linear::MellStructure). With cut links it is
//!   written at level 6, an end of a cut link `[occurrence, null, box]`.
//! - **Intuitionistic sequent**: `{"version": 5, "terms": [{"V": 0},
//!   {"V": 1}, {"⊸": [0, 1]}], "atoms": ["A", "B"], "hypotheses": [2,
//!   0], "goal": 1}` for `A ⊸ B, A ⊢ B`: the terms `V`, `1`, `0`, `⊤`,
//!   `⊗`, `⊸` (the antecedent first), `&`, `⊕` and `!`, every operand
//!   before its term; see [`IllSequent`](crate::linear::IllSequent). With the
//!   quantifiers `∀` and `∃` and the first-order keys it is written at
//!   level 7, as the one-sided sequent is.
//! - **First-order sequent**: `{"version": 7, "terms": [{"D": 0}, {"∃":
//!   0}, {"V": 1}], "roots": [1, 2], "atoms": [{"P": [0, [0]]},
//!   {"P": [0, [1]]}], "symbols": [["p", 1], ["a", 0]], "fo": [{"B": 0},
//!   {"F": [1, []]}], "binders": ["x"], "antecedents": 1}` for `forall x.
//!   p(x) |- p(a)`: `symbols` each a name with its arity, `fo` the terms
//!   (`B` a bound variable, the innermost binder's `0`; `F` a symbol
//!   applied to terms of `fo`, each before its term), every atom a
//!   predicate applied to terms, a binder as `{"∀": body}` or `{"∃":
//!   body}`, and `binders` the variables' names, one per binder in the
//!   order of the terms.
//! - **Session**: `{"version": 1, "sequent": {…}, "mode":
//!   "intuitionistic", "inferences": […], "history": […]}`, and for a
//!   session that keeps how its sequent was written `"written":
//!   {"inputs": […], "implications": […]}`, a key that raises no level,
//!   since a reader that ignores it reads the session as it is and draws
//!   it one-sided; see `Interactive` (feature `interactive`).
//! - **Ordinary sequent**: `{"version": 8, "formulas": [{"V": 0}, {"V":
//!   1}, {"→": [0, 1]}], "atoms": ["a", "b"], "left": [2, 0], "right": [1]}`
//!   for `a → b, a ⊢ b`: the formulas `V`, `⊤`, `⊥`, `¬`, `∧`, `∨`, `→`
//!   and `↔`, every operand before its formula, and the two sides as
//!   indices into them; see [`ordinary::Sequent`](crate::ordinary::Sequent).
//! - **Ordinary derivation**: `{"version": 8, "logic": "intuitionistic",
//!   "sequent": {…}, "inferences": [{"left": [0], "right": [0], "rule":
//!   "ax"}, …, {"left": [2, 0], "right": [1], "rule": "→L", "principal":
//!   ["left", 0], "premises": [0, 1]}]}`: the sequent with every formula
//!   the inferences name, the inferences premises before conclusions and
//!   the root last, each its two sides, its rule's name, its principal
//!   formula's side and position and its premises; reading one checks its
//!   shape (a tree, its root last), and [`Derivation::check`] its rules;
//!   see [`ordinary::Derivation`](crate::ordinary::Derivation).
//!
//!   [`Derivation::check`]: crate::ordinary::Derivation::check
//! - **Ordinary outcome** (written only): `{"version": 8, "deduxit":
//!   "0.3.0", "verdict": "valid", "logic": "classical", "translation":
//!   "affine", "engine": "focus", "statistics": {…}, "derivation": {…}}`,
//!   `verdict` one of `valid`, `not_valid` and `unknown` (with a `reason`
//!   in an outcome's form), the engine and statistics the image's search's;
//!   see [`ordinary::Outcome`](crate::ordinary::Outcome). A sequent valid
//!   by a clausal proof, as the SAT solver decides classical logic, is
//!   written at level 10 with `"engine": "sat"` and `"clausal_proof":
//!   {"variables": 7, "clauses": 12, "steps": 3}` in place of
//!   `derivation`: the variables and clauses of the sequent's negation and
//!   the steps of the proof that this crate's checker accepted.
//! - **Ordinary session**: `{"version": 8, "logic": "intuitionistic",
//!   "sequent": {…}, "inferences": […], "history": […]}`, the inferences in
//!   the state's own order, the root first, an open goal its two sides
//!   alone; reading one checks every inference by the checker's rules; see
//!   `ordinary::Interactive` (feature `interactive`).
//! - **Cut elimination** (written only): `{"version": 6, "proof": {…},
//!   "steps": [{"cut": 7, "formula": 9, "case": "tensor_par"}, …],
//!   "end": "cut_free"}`, or `"end": "refused"` with the `reason` in an
//!   error's form; see [`Elimination`](crate::linear::proofs::cut::Elimination).
//! - **Outcome**: `{"version": 1, "deduxit": "0.1.0",
//!   "verdict": "proved", "checked": true, "fragment": "MLL", "mode":
//!   "classical", "engine": "net", "statistics": {…}, "sequent": {…},
//!   "nodes": […]}`, a proof's keys for `proved` and a disproof's for
//!   `unprovable`, so that it reads back as a proof or as a disproof; see
//!   [`Outcome`](crate::linear::Outcome).

use crate::Error;
use crate::limits::Limits;
use serde::de::{DeserializeSeed, Deserializer};
use std::cell::RefCell;
use std::marker::PhantomData;

/// The highest wire level this build reads and writes.
pub const LEVEL: u32 = 10;

/// The level of every document that no later level's feature raises.
pub(crate) const BASE: u32 = 1;

/// The level of a proof structure with boxes.
pub(crate) const BOXES: u32 = 4;

/// The level of an intuitionistic sequent.
pub(crate) const ILL: u32 = 5;

/// A form that [`upgrade`] and [`Within`] read, counting its sequents
/// against the caller's limits. It is sealed: only the crate's data forms
/// implement it.
pub trait Readable: Sized + crate::sealed::Readable {
    /// The form's name, as an error names it: `sequent`, `proof`, …
    const FORM: &'static str;

    /// Reads a value of the form from `deserializer`, every sequent within
    /// `limits.occurrences`.
    ///
    /// # Errors
    ///
    /// The deserializer's error for a document of another shape, or for a
    /// value that the form's checks refuse.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error>;
}

/// Reads a value of the form `T` within limits, as a serde seed. It is
/// for a value nested in a document of the caller's, where [`upgrade`]
/// reads a whole document.
#[derive(Clone, Copy, Debug)]
pub struct Within<'a, T> {
    /// The limits.
    limits: &'a Limits,
    /// The form read.
    form: PhantomData<fn() -> T>,
}

impl<'a, T> Within<'a, T> {
    /// Returns the seed that reads within `limits`.
    pub const fn new(limits: &'a Limits) -> Self {
        Self {
            limits,
            form: PhantomData,
        }
    }
}

impl<'de, T: Readable> DeserializeSeed<'de> for Within<'_, T> {
    type Value = T;

    /// Reads the value within the seed's limits.
    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<T, D::Error> {
        T::read(deserializer, self.limits)
    }
}

/// Reads a document of any level up to [`LEVEL`] and returns it as a
/// value of this release, every sequent within `limits.occurrences`.
///
/// It reads one value and leaves the deserializer after it. A caller
/// whose input is that one document ends the deserializer (`end()` in
/// serde_json), which refuses anything that follows.
///
/// # Errors
///
/// - [`Error::Version`] for a document above [`LEVEL`];
/// - the error of the form's checks for a value they refuse: an index out
///   of bounds, a sequent past the bound, links that are no links;
/// - [`Error::Json`] for a document of another shape, naming what is
///   missing or wrong.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::{Error, Limits, wire, linear::Sequent};
///
/// let json = r#"{"version": 1, "terms": [{"V": 0}], "roots": [0], "atoms": ["A"]}"#;
/// let mut document = serde_json::Deserializer::from_str(json);
/// let sequent: Sequent = wire::upgrade(&mut document, &Limits::default())?;
/// document.end().expect("one document and nothing after it");
/// assert_eq!(sequent.to_string(), "⊢ A");
///
/// let newer = r#"{"version": 11, "terms": []}"#;
/// let mut document = serde_json::Deserializer::from_str(newer);
/// let error = wire::upgrade::<Sequent, _>(&mut document, &Limits::default()).unwrap_err();
/// assert!(matches!(error, Error::Version { found: 11, .. }));
/// # Ok::<(), Error>(())
/// ```
pub fn upgrade<'de, T: Readable, D: Deserializer<'de>>(
    document: D,
    limits: &Limits,
) -> Result<T, Error> {
    CAUSE.with_borrow_mut(|cause| *cause = None);
    let read = T::read(document, limits);
    let cause = CAUSE.with_borrow_mut(Option::take);
    read.map_err(|error| match cause {
        // The key names no form; the caller's is the one read.
        Some(Error::Version {
            found, supported, ..
        }) => Error::Version {
            form: T::FORM,
            found,
            supported,
        },
        Some(cause) => cause,
        None => Error::Json {
            form: T::FORM,
            message: error.to_string(),
        },
    })
}

thread_local! {
    /// The library's own error that ended the read [`upgrade`] runs on
    /// this thread. A deserializer's error carries it as text only.
    static CAUSE: RefCell<Option<Error>> = const { RefCell::new(None) };
}

/// Returns a deserializer's error for one of the library's own, and
/// keeps the library's error for [`upgrade`] to answer in place of
/// [`Error::Json`].
pub(crate) fn fail<E: serde::de::Error>(error: Error) -> E {
    let message = error.to_string();
    CAUSE.with_borrow_mut(|cause| *cause = Some(error));
    E::custom(message)
}

/// Reads a `version` key: a level up to [`LEVEL`], refused above it.
pub(crate) fn version<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<u32>, D::Error> {
    let found = <u64 as serde::Deserialize>::deserialize(deserializer)?;
    match u32::try_from(found) {
        Ok(level) if level <= LEVEL => Ok(Some(level)),
        _ => Err(fail(Error::Version {
            form: "document",
            found,
            supported: LEVEL,
        })),
    }
}

/// The level of a sequent with cut formulas.
pub(crate) const CUTS: u32 = 6;

/// The level of a first-order sequent.
pub(crate) const TERMS: u32 = 7;

/// The level of the forms of ordinary logic.
pub(crate) const ORDINARY: u32 = 8;

/// The level of a form of ordinary logic that holds LJ's cut.
pub(crate) const ORDINARY_CUT: u32 = 9;

/// The level of an ordinary outcome valid by a clausal proof.
#[cfg(feature = "sat")]
pub(crate) const CLAUSAL: u32 = 10;

/// The level of a document that holds `sequent`: 7 where it is
/// first-order, 6 where it has cut formulas, else 1.
pub(crate) fn level_of_sequent(sequent: &crate::linear::Sequent) -> u32 {
    if sequent.is_first_order() {
        TERMS
    } else if sequent.cuts().is_empty() {
        BASE
    } else {
        CUTS
    }
}

/// The level of a document of `mode`, or of no mode: 2 for an ordered
/// mode, else 1.
pub(crate) const fn level_in(mode: Option<crate::linear::Mode>) -> u32 {
    match mode {
        Some(mode) if mode.is_ordered() => 2,
        _ => BASE,
    }
}

/// The level of a disproof with `refutation` in `mode`: 3 for a
/// saturated set, 2 for a classical refutation or an ordered mode, else
/// 1.
pub(crate) const fn level_of(
    refutation: &crate::linear::search::Refutation,
    mode: crate::linear::Mode,
) -> u32 {
    match refutation {
        crate::linear::search::Refutation::Saturated(_) => 3,
        crate::linear::search::Refutation::Classical(_) => 2,
        _ => level_in(Some(mode)),
    }
}
