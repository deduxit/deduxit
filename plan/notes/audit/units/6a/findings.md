# Unit 6a: findings

Read in full: 23 files. Not read in full: none.

## F-6a-01

- dimension: api
- criterion: S1
- place: core/src/serialize/sequents.rs:578
- kind: refactor
- cost: medium
- candidate: no

**Claim.** A proof, a proof structure (with or without boxes) and a session read through `wire::upgrade` build their sequent within the default `Limits`, not the caller's, so `limits.terms` set lower than the default is ignored on those readers.

**Witness.**

```rust
impl TryFrom<Sequent> for Seq {
    type Error = crate::Error;
    ...
    fn try_from(s: Sequent) -> Result<Seq, Self::Error> {
        Self::read_within(s, &Limits::default())
    }
}
```

The callers that hold the caller's `limits` and still go through it:
`core/src/serialize/proofs.rs:303`, `core/src/serialize/nets.rs:123`,
`core/src/serialize/nets.rs:313` and `core/src/serialize/interactive.rs:259`
(`crate::linear::Sequent::try_from(...)?`, then `Forest::from_owned(sequent, limits)`).
The sequent and disproof readers use `self.sequent.within(limits)` (`sequents.rs:556`,
`search.rs:592`). Run with `target/debug/deduxit`:

```text
$ deduxit prove 'p(f(f(a))) |- p(f(f(a)))' --term-limit 1
error: the first-order terms unfold to at least 3 terms, more than the limit of 1; raise it with --term-limit   (exit 2)
$ deduxit prove 'p(f(f(a))) |- p(f(f(a)))' --format json > fo-outcome.json
$ deduxit check --file fo-outcome.json --term-limit 1
valid proof of ⊢ ~p(f(f(a))), p(f(f(a))) (classical)   (exit 0)
```

`check` reads it with `wire::upgrade::<Proof, _>(&mut document, limits)` (`cli/src/check.rs:1279`).

**Change.** Pass the caller's limits: call `Seq::read_within(proxy, limits)` (or `Sequent::within`) at the four places, and drop the `TryFrom` impl so no reader can reach the default again.

**Value.** A user of the library or the command who lowers the term bound (a browser tab, a server) gets it honoured on every reader, as `Limits::terms` promises ("on every reader"); the key building that the bound guards (exponential in a short document) is bounded by their number.

**Net.** `core/tests/serialize.rs` and the behaviour lock keep every form's bytes; add one test that a proof document past `limits.terms` is refused. No checker, search or certificate is touched.

## F-6a-02

- dimension: api
- criterion: C-GOOD-ERR
- place: core/src/wire.rs:434
- kind: API break
- cost: small
- candidate: no

**Claim.** `wire::Within`, the public seed, returns only the deserializer's text: the library's typed error (`Refusal`, `Version`, `IndexOutOfBounds`) that `wire::fail` stores in the thread-local `CAUSE` is read back by `upgrade` alone, so its one outside user reports a refusal as malformed input.

**Witness.**

```rust
impl<'de, T: Readable> DeserializeSeed<'de> for Within<'_, T> {
    type Value = T;

    /// Reads the value within the seed's limits.
    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<T, D::Error> {
        T::read(deserializer, self.limits)
    }
}
```

`cli/src/io.rs:402-406` reads an intuitionistic JSON sequent with it and adds
`.context("not an intuitionistic sequent in JSON")`. With a 5-occurrence
`{"version":5,…}` file:

```text
$ deduxit prove --file ill.json --intuitionistic --occurrence-limit 2
error: not an intuitionistic sequent in JSON: the sequent unfolds to at least 5 subformula occurrences, more than the limit of 2
```

while the one-sided JSON path says "…more than the limit of 2; raise it with --occurrence-limit".

**Change.** Remove `Within` from the public API (its one outside caller can call `wire::upgrade`, which takes any deserializer and answers `Error`), or keep it for nested values and document that its error is text only; move `cli/src/io.rs:402` to `upgrade`.

**Value.** A front end gets an `Error` with its `kind` and `code` from every public reader, so it can tell a refusal (exit 3 territory) from a malformed file; the command stops calling a valid document "not an intuitionistic sequent".

**Net.** `cargo test --workspace` (the CLI's tests read JSON sequents); the wire forms are unchanged. No checker is touched.

## F-6a-03

- dimension: api
- criterion: X2
- place: core/src/limits.rs:593
- kind: API break
- cost: medium
- candidate: no

**Claim.** An outside engine cannot answer in the crate's refusal: every variant of `Refusal` is a `#[non_exhaustive]` struct variant with no constructor, `Progress::new` is `pub(crate)`, and the crate itself spells out the same bound check by hand at about thirty places.

**Witness.**

```rust
pub enum Refusal {
    /// The caller's stop ended the call.
    #[non_exhaustive]
    Stopped {
        /// What the call was doing.
        phase: Phase,
    },
```

```rust
impl Progress {
    /// Returns the progress of a call in `phase` that has done `done`
    /// units of work, `work` of them since the previous poll.
    pub(crate) const fn new(phase: Phase, work: u64, done: u64) -> Self {
```

The consumer test's attempt 3 (`Refusal::Work { limit }`) fails with E0639. A `Progress`
can be made outside only by `Progress::default()` and field writes (it derives `Default`,
`limits.rs:405`), which no doc shows. The hand-written checks, each
`if let Some(limit) = limits.X && n > limit { return Err(Error::Refused(Refusal::X { … })) }`:
`serialize/ill.rs:322`, `serialize/kripke.rs:124`, `serialize/modal.rs:131`,
`serialize/ordinary.rs:162`, `serialize/rules.rs:531`, `serialize/resolution.rs:130`,
`serialize/sequents.rs:373` and `:559`, `serialize/proofs.rs:133`,
`serialize/interactive.rs:282`, and 20 more (`linear/mist.rs:220`,
`linear/occurrences/mod.rs:358`, `lambda/read.rs:161`, `modal/mod.rs:477`,
`ordinary/mod.rs:1764`, `ordinary/translate.rs:602`, `rules/text.rs:232`, …).

**Change.** Add `Progress::new(phase, work, done)` public, and checks on `Limits` that return the refusal: `limits.admit_occurrences(n)?`, `admit_terms(n)?`, `admit_work(done)?` (each `Result<(), Refusal>`), plus `Refusal::stopped(phase)` for a stop; use them at the thirty places. Fields stay hidden, so the reason `core.md` gives for marking the variants (a field added later breaks nothing) still holds.

**Value.** A researcher's engine bounds itself and refuses exactly as a built-in one does, so a front end sorts its answer by `Error::kind` (consumer test points 2 and 3); readers of the crate see one check instead of thirty copies.

**Net.** The behaviour lock and `errors/mod.rs`'s `the_codes_are_listed` keep messages and codes; the readers' refusal tests in `core/tests/serialize.rs`. No checker or search changes its result.

## F-6a-04

- dimension: architecture
- criterion: B4
- place: core/src/lib.rs:350
- kind: build
- cost: large
- candidate: no

**Claim.** The JSON feature `serialize` gates whole suites: calculi given as rule files exist only with it, because `rules::Calculus` stores the serde proxy, and with them the modal decision; a user who wants modal logic without serde, or the data model and checkers alone (consumer test 13), has no cut that gives it.

**Witness.**

```rust
#[cfg(feature = "serialize")]
pub mod rules;
```

`core/src/rules/calculus.rs:12` and `:267`:

```rust
use crate::serialize::rules::{CalculusForm, DISCHARGES, PatternForm, RuleForm, SequentForm};
…
pub struct Calculus {
    /// The rule file.
    form: CalculusForm,
```

`core/src/modal/mod.rs:61-63`:

```rust
#[cfg(feature = "serialize")]
pub use calculus::calculus;
#[cfg(feature = "serialize")]
pub use decide::{Evidence, Options, Outcome, Statistics, Verdict, decide, decide_within};
```

`parse` likewise holds the text syntax and the benchmark readers together (`lltp`, `mist`, `families`, `read_tptp`; `lib.rs:287-290`). `timings.md`: the core crate alone takes 40 s with the defaults and 16 s without, and nothing between them.

**Change.** Keep the rule file's data in a type of `rules` (the checked calculus), with serde as one way in; build `rules` and `modal::decide` without `serialize`; then name the features by what a user takes (for example `rules` and `modal` always on, `serde` for the forms, `inputs` for LLTP, Mist, TPTP and the families apart from `parse`). The architecture reader's proposal decides the exact cut.

**Value.** A library user takes a suite without the JSON layer, and the web module can drop what a page does not use; the features say what they add (C-FEATURE).

**Net.** `nix flake check`'s `features` and `features-tests` (every pair of features), `web-features` and `budget`; the forms' bytes stay (lock, `core/tests/wire/`). No checker or search is touched.

## F-6a-05

- dimension: text
- criterion: D4
- place: core/src/lib.rs:111
- kind: text
- cost: small
- candidate: no

**Claim.** The crate docs give linear logic every walk-through section and the shared notions: after one ordinary example, "A proof, its fragment and its JSON", "Intuitionistic input", "Limits and stops" and "The command's answers" are all linear, the "Errors" section lists only linear errors, and occurrences, a linear notion, are described as what "proofs, nets and errors" name.

**Witness.**

```rust
//! # A proof, its fragment and its JSON
//!
//! Parse a sequent, see which fragment of linear logic it lives in, prove
```

lib.rs:107-109: "Proofs, nets and errors name the *occurrences* of a sequent: … the sequent's [`Forest`](linear::Forest) numbers them." lib.rs:257-263 lists `CheckError`, `NetError`, `ShapeError`, `ParseError` only. lib.rs:167-196 and 198-225 use `linear::prove_within` alone; lib.rs:316-322 lists the closed enums linear first. lib.rs:18 also leans on intuitionistic logic being `ordinary::Options`' default ("here intuitionistic logic, the default"), which D1 removes in the command. `core/Cargo.toml:13-19`: of the five keywords the one suite named is `linear-logic`.

**Change.** Give each suite one short worked example in the page's order (ordinary, linear, modal, lambda-terms, then rule files) with the logic named explicitly; move the linear details (fragments, ILL input, occurrences) to `linear`'s module docs; make "Limits and stops" and "Errors" suite-neutral (a stop with `ordinary::decide_within` or `modal::decide_within`, the error kinds without the linear types first); add a section "Write your own engine" pointing at the constructors and checkers (X8). Keywords: `logic`, `theorem-prover`, `sequent-calculus`, `modal-logic`, `linear-logic` or `proof-checker`.

**Value.** A reader on docs.rs sees four suites as peers and finds each one's first call at once.

**Net.** The doc examples run under `cargo test`; the readme test checks names. Text only.

## F-6a-06

- dimension: api
- criterion: C-HIDDEN
- place: core/src/lib.rs:430
- kind: API break
- cost: medium
- candidate: no

**Claim.** The crate root mixes shared items with one suite's, and re-exports items under two paths: `Owner` and `Described` serve only linear errors, `StepFault` and `Backwards` only rule-file sessions and resolution by hand, while `limits` and `settings` are public modules whose items the root re-exports again (six items under two paths each).

**Witness.**

```rust
#[cfg(feature = "parse")]
pub use errors::ParseError;

#[cfg(feature = "interactive")]
pub use errors::WitnessFault;
pub use errors::{Backwards, Described, Error, ErrorKind, Owner, StepFault};
pub use limits::{Limits, Phase, Progress, Refusal};
pub use settings::{Clock, Settings};
```

`errors/describe.rs:117-121`: "It is implemented by [`Forest`], [`Proof`], [`Derivation`], [`ProofStructure`], [`MellStructure`] and … `Interactive`" — all linear. `surface.md` lists `Clock`, `Limits`, `Phase`, `Progress`, `Refusal`, `Settings` each at two paths, while `errors` is private and `limits::{Bound, Held, Space, WorkspaceLimits}` exist under `limits::` only. `core.md`'s own rule (Layout): "What goes to the root is what `ordinary` or `lambda` uses besides the linear suite and is no part of the linear calculus".

**Change.** One path per item: either keep `limits` and `settings` public without root re-exports, or private with every public item re-exported; move `Owner` and `Described` to `linear` (keeping `Error::describe`), and `StepFault` and `Backwards` to `rules` (`deduxit::Backwards` names nothing a reader can guess).

**Value.** A reader of rustdoc or of an error message sees one path to write; the root lists what every suite shares.

**Net.** `breaks 0.8.0` lists the moves; the CLI, bench and web follow by path. No checker is touched.

## F-6a-07

- dimension: idiom
- criterion: C-SERDE
- place: core/src/serialize/errors.rs:238
- kind: refactor
- cost: large
- candidate: no

**Claim.** Most id newtypes have no serde while a few have it transparent, so about 600 lines of `serialize/` write forms by hand: mirror enums with two converters (`Why` for `Reason`, `WhyNot` for `Refutation`) and hand-written `Serialize` for `Fault`, `ShapeError`, `NetError`, `NotTaken` and `Placement`, where `Refusal`, `StepFault` and `WitnessFault` derive the same tagged shape.

**Witness.**

```rust
impl Serialize for Fault {
    /// Writes the fault's `kind` and its fields.
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Forbidden => entries!(s; "kind" => "forbidden"),
            Self::Shape(e) => entries!(s; "kind" => "shape"; "shape" => e),
            Self::Succedents { count } => entries!(s; "kind" => "succedents"; "count" => count),
            Self::Kind { member } => entries!(s; "kind" => "kind"; "member" => &member.get()),
```

Transparent serde today: `Member` (`linear/occurrences/mod.rs:58-62`), `Eigen` (`first_order.rs:97`), lambda's ids (`lambda/mod.rs:209-259`); Serialize only: `GoalId`, `InfId`; none: `OccId` (`linear/occurrences/mod.rs:28`), `VertexId` (`linear/nets/mod.rs:168`), proofs' `NodeId` (`linear/proofs/mod.rs:80`), `Atom` and `TermId` (`linear/sequents/term.rs:8,36`), `FrameId`, and the `NodeId`s of `ordinary`, `modal` and `rules`. The mirrors: `serialize/search.rs:53-175` (`Why`, a field-for-field copy of `Reason`, which is `Copy`), `search.rs:179-561` (`Literal`, `WhyNot`), `serialize/errors.rs:198-385`.

**Change.** Derive `Serialize, Deserialize` with `serde(transparent)` on every id newtype (behind the feature), and derive the tagged forms (`serde(tag = "kind", rename_all = "snake_case")`) on `Reason`, `Refutation`'s parts, `Fault`, `ShapeError`, `NetError`, `NotTaken`, `Placement` and `Sign` (`rename` to `atom`/`dual`), keeping every byte; delete the mirrors.

**Value.** A library user can serialize the ids and errors they hold; a reader of `serialize/` sees the forms that are proxies on purpose (arenas, documents) and no copies of public enums.

**Net.** The behaviour lock (`core/tests/lock/json.txt`), `core/tests/serialize.rs`, `stability.rs` and `core/tests/wire/` pin every byte; the `wire-schema` flake check. No checker is touched.

## F-6a-08

- dimension: text
- criterion: T3
- place: core/src/errors/mod.rs:1186
- kind: text
- cost: small
- candidate: no

**Claim.** The message of `Error::IndexOutOfBounds` for the spaces `Term` and `Node` is worded for a linear sequent's root and a linear proof's premise, but those spaces are used for first-order terms, rule-file terms, refutation clauses and workspace nodes, where the message is false.

**Witness.**

```rust
fn out_of_bounds(space: Space, index: usize, len: usize) -> String {
    match space {
        …
        Space::Term => format!(
            "a root formula is term {index}, but the arena has {}",
            counted(len, "term", "terms")
        ),
        Space::Node => format!(
            "a proof refers to node {index}, but it has {}",
            counted(len, "node", "nodes")
        ),
```

`core/src/workspace/mod.rs:1566-1571` reports a node the workspace lacks as
`IndexOutOfBounds { space: Space::Node, index, len: 0 }`, so the user reads "a proof refers to node 7, but it has 0 nodes". `rules/terms.rs:195-199` (`Terms::apply`, public), `first_order.rs:457-461`, `ordinary/bruijn.rs:94-98` and `serialize/rules.rs:561-565` give `Space::Term` for an argument or term index ("a root formula is term …"); `serialize/resolution.rs:155-159` gives `Space::Node` for a resolvent's parent clause.

**Change.** Word the two arms by the space alone, as the fallback arm does ("index {index} names no term: there are {len}"), or add the spaces `Clause` and a workspace's `Node`, and let the linear reader that needs "root formula" say it itself.

**Value.** A user of the workspace, the rule files and resolution gets a message about their own input.

**Net.** The lock pins messages of the linear forms; tests that match those two messages change with them. Text only.

## F-6a-09

- dimension: api
- criterion: D4
- place: core/src/settings.rs:129
- kind: API break
- cost: large
- candidate: no

**Claim.** `Settings`, "everything that configures a front end's calls", gives linear logic's options the generic names (`search`, `view`, `input`, `cut`, `interactive`) beside one key per other suite, and the bounds of ordinary logic's engines are set through `search`, which is `linear::search::Options`.

**Witness.**

```rust
pub struct Settings {
    pub clock: Clock,
    pub limits: Limits,
    /// The search's options.
    pub search: Options,
    /// How a derivation is shown.
    pub view: ViewOptions,
    pub styles: Styles,
    pub batch: batch::Options,
    /// How an ordinary sequent is decided.
    pub ordinary: ordinary::Options,
    #[cfg(feature = "interactive")]
    pub interactive: interactive::Options,
    pub input: InputOptions,
    pub cut: cut::Options,
    pub lambda: crate::lambda::Options,
}
```

(doc lines left out). `settings.rs:161-166` sets the connection prover's and the model search's bounds on it (`.with_path_length(None).with_model_size(None).with_model_worlds(None)`), and `protocol/ordinary.rs:969` passes `&settings.search` to `ordinary::decide_within`. The settings keys are documented as `search.copies` and so on (`docs/src/settings.md:28`).

**Change.** Group the settings by suite in the page's order (`ordinary`, `linear { search, view, input, cut, interactive }`, `modal`, `lambda`), with the ordinary engines' bounds under `ordinary`; the shared parts (`clock`, `limits`, `styles`, `batch`) stay at the top. A migration note maps the old keys.

**Value.** A front end or settings file reads which options apply to which logic; ordinary logic's options stop depending on the linear suite's type (consumer test point 10).

**Net.** The CLI's settings tests and the manual's settings chapter, `web-tab`'s tests and the client's calls; no search changes its counters if the values are carried over (target sets). This touches the settings JSON, a contract with the client (B6).

## F-6a-10

- dimension: text
- criterion: T3
- place: core/src/limits.rs:15
- kind: text
- cost: small
- candidate: no

**Claim.** The module's reference table of "the long calls, the fields of Limits that bound each, and how often each asks its stop" lists linear calls, some ordinary ones and the exports, and none of modal logic, lambda-terms, rule files, the protocol or the workspace, nor most of ordinary logic's engines.

**Witness.**

```rust
//! The long calls, the fields of [`Limits`] that bound each, and how often
//! each asks its stop:
//!
//! | call | bound by | asks the stop |
//! |---|---|---|
//! | `Sequent::parse_within`, `lltp::read`, `mist::read`, `ordinary::Sequent::parse_within`, `ordinary::read_tptp`, `wire::upgrade`, `wire::Within` | `occurrences` | never: linear in the input |
//! | `Forest::within`, `Interactive::within`, `ordinary::translate` | `occurrences` | never: linear in the bound |
//! | `search::prove_within`, `search::prove_goal` | `memory_bytes`, `recursion_depth`, `occurrences`, `work` (the search's and the check's) | at every node an engine searches, and in its long passes |
```

Missing, each taking `&Limits` and a stop (`core.md`, "The public API at a glance"): `modal::decide_within`, `Model::frame_fault`/`truth`, `lambda::Term::check`, `inhabit`, `enumerate`, `Term::reduce`, `principal_type`, `rules::Derivation::check`, `protocol::run`, `ordinary::ljt::decide`, `ordinary::sat::decide`, `resolution::decide`, `herbrand`, `normal::rewrite`. Rows 20-29 are the linear suite's.

**Change.** Complete the table, grouped by suite in the page's order, or move each suite's rows to its module and keep here the rule (every long call takes `&Limits` and a stop) with links.

**Value.** A user of modal logic or lambda-terms finds which bound ends their call and how often their stop is asked.

**Net.** Text; `cargo doc` with the link lints.

## F-6a-11

- dimension: architecture
- criterion: E6
- place: core/src/serialize/modal.rs:36
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The modal forms copy ordinary logic's: the formula form, its conversion, the sequent form and its reader, and the verdict word are written twice, though the modal sequent is "an ordinary sequent's form with the formulas `□` and `◇`" (`wire.rs:159-163`).

**Witness.** `duplicates.md`: `serialize/modal.rs:29` and `serialize/ordinary.rs:33` (144 tokens); `modal.rs:67` and `ordinary.rs:71` (187 tokens, 17 lines); `modal.rs:139` and `ordinary.rs:190`; `modal.rs:158` and `ordinary.rs:215` (183 tokens); `modal.rs:212` and `ordinary.rs:293`; `modal.rs:317` and `ordinary.rs:1071`. For instance both readers hold

```rust
            let at = |k: u32| {
                ids.get(k as usize).copied().ok_or(Error::IndexOutOfBounds {
                    space: Space::Formula,
                    index: k as usize,
                    len: n,
                })
            };
```

(`modal.rs:143-149`, `ordinary.rs:193-199`).

**Change.** One formula form with the tags of both (`□`, `◇` beside `∀`, `∃`), and one reader generic over the arena it fills (a small trait with `atom`, `add`, `binder`), each suite refusing the tags it lacks.

**Value.** A tag or check added to one suite's reader reaches the other; about 200 lines go.

**Net.** `serialize/modal.rs`'s tests, the lock and `core/tests/wire/level-*.txt` pin the bytes; the `wire-schema` check. No checker is touched.

## F-6a-12

- dimension: api
- criterion: X9
- place: core/src/errors/parse.rs:31
- kind: API break
- cost: small
- candidate: no

**Claim.** `ParseError::reserved` is a public field documented as always `false`; nothing sets it otherwise, and the `Display` arm that reads it is dead.

**Witness.**

```rust
    /// Whether what was found is a reserved word of the syntax. Always
    /// `false`: no word is reserved, since `forall` and `exists` read as
    /// the quantifiers. The JSON form leaves it out while it is `false`.
    #[cfg_attr(
        feature = "serialize",
        serde(skip_serializing_if = "std::ops::Not::not")
    )]
    pub reserved: bool,
```

`grep` finds it set only at `errors/parse.rs:73` (`reserved: false`) and read at `parse.rs:94`, `linear/lltp.rs:531` and `core/tests/parse.rs:289` (which asserts `false`).

**Change.** Remove the field and its `Display` arm in 0.9.0; the JSON never writes it, so the wire form is unchanged.

**Value.** A user no longer reads or matches on a field that cannot hold.

**Net.** `core/tests/parse.rs`; the JSON is unchanged. No checker is touched.

## F-6a-13

- dimension: idiom
- criterion: C-WORD-ORDER
- place: core/src/errors/mod.rs:1113
- kind: API break
- cost: large
- candidate: no

**Claim.** `Error`'s variant names do not follow one pattern and disagree with their stable codes: `Model` is `invalid_model`, `Structure` is `invalid_structure`, `InvalidRefutation` (clauses) is `invalid_clause_refutation` while `Refutation` (a linear disproof) is `invalid_refutation`, `Reduction` is `reduction_rejected`, `Unsupported` (Rocq) is `no_certificate`.

**Witness.**

```rust
            Self::Rejected(_) => "rejected",
            Self::ReadBack { .. } => "read_back",
            Self::Countermodel { .. } => "countermodel",
            Self::Model { .. } => "invalid_model",
            Self::Structure { .. } => "invalid_structure",
            Self::InvalidCalculus { .. } => "invalid_calculus",
            Self::InvalidDerivation { .. } => "invalid_derivation",
            Self::Zones { .. } => "zones",
            Self::StepRefused { .. } => "step_refused",
            Self::InvalidRefutation { .. } => "invalid_clause_refutation",
```

and `errors/mod.rs:1070` `Self::Refutation(_) => "invalid_refutation"`, `:1102` `Self::Unsupported(_) => "no_certificate"`, `:1132` `Self::Reduction(_) => "reduction_rejected"`.

**Change.** Rename the variants to their codes in 0.9.0 (`InvalidModel`, `InvalidStructure`, `InvalidClauseRefutation`, `InvalidRefutation` for the linear one, `ReductionRejected`, `NoCertificate`); the codes, the wire contract, stay.

**Value.** A program that matches on variants and one that branches on codes use the same names; `Error::Refutation` and `Error::InvalidRefutation` stop meaning each other's case.

**Net.** `the_codes_are_listed`; `breaks 0.8.0`. Callers across the crate, the CLI and the bindings change by name only.

## F-6a-14

- dimension: text
- criterion: T1
- place: core/src/errors/mod.rs:460
- kind: text
- cost: small
- candidate: no

**Claim.** The message of `Error::GlivenkoFirstOrder` is a paragraph of nine lines, with a countermodel's construction and the Rocq library's lemma names, where an error message says what is wrong and what to do.

**Witness.**

```rust
    #[error(
        "Glivenko's translation takes propositional sequents only: its theorem fails at first order, \
         where ⊢ ∀x. (p(x) ∨ ¬p(x)) is valid classically and ⊢ ¬¬∀x. (p(x) ∨ ¬p(x)) is not valid \
         intuitionistically, nor is the double-negation shift ⊢ (∀x. ¬¬p(x)) → ¬¬(∀x. p(x)). No \
         finite Kripke model refutes them, since each world of one sees a last world, where every \
```

**Change.** "Glivenko's translation takes propositional sequents only; Kuroda's translation carries first-order sequents into intuitionistic logic", with the counterexample and the Rocq lemmas in the variant's doc and the manual.

**Value.** A user of the command or the page reads the cause and the remedy in one line.

**Net.** Tests that match the message; text only.

## F-6a-15

- dimension: api
- criterion: D4
- place: core/src/errors/mod.rs:694
- kind: API break
- cost: large
- candidate: no

**Claim.** Only the linear checker's faults are structured (`CheckError` with the node, the rule, the premises and a `Fault`, and `describe` to print formulas); the checks of Kripke models, finite structures, rule-file derivations and calculi answer prose in a `reason: String`, so a front end cannot point at the world or node at fault.

**Witness.**

```rust
    #[error("the Kripke model is wrong: {reason}")]
    #[non_exhaustive]
    Model {
        /// What fails, and at which world.
        reason: String,
    },
```

Likewise `Countermodel` (`errors/mod.rs:686`), `Structure` (`:779`), `InvalidCalculus` (`:703`), `InvalidDerivation` (`:711`, "What fails, and at which node"), `ReadBack` (`:676`), `InvalidRefutation` (`:731`), `NotFitch` (`:761`); their JSON `details` are `{"reason": …}` alone (`serialize/errors.rs:169-174`). `IllTyped` (`:625`) shows the other way: `node` and `span` beside the reason.

**Change.** Give each a fault type as `CheckError` has: the world, node or inference at fault and a `kind`, with the prose kept as `Display`; add the fields to `details`.

**Value.** The web page and other front ends highlight the world or node at fault in every suite, as they can for a linear proof.

**Net.** The checkers' tests and the lock's error lines; the wire `details` grow by keys (no level). The checkers' verdicts do not change, their error values do.

## F-6a-16

- dimension: performance
- criterion: P2
- place: core/src/serialize/proofs.rs:260
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** Writing a value builds an owned copy of it first: a proof's nodes, its sequent's arena and names, a Kripke model's worlds, a structure and a calculus are cloned into proxies only to be written, where `serialize/lambda.rs` writes through borrowed fields.

**Witness.**

```rust
    pub(super) fn keys(p: &Prf) -> Proof {
        Proof {
            version: None,
            sequent: Sequent::from(p.sequent()),
            nodes: p.nodes().iter().copied().map(Step::from).collect(),
```

Also `serialize/sequents.rs:533-549` (every name and binder cloned), `kripke.rs:95-111` (`atoms: w.atoms.clone()`, `arrows: w.arrows.clone()`), `ordinary.rs:586` and `:626` (`c.model.clone()`, `c.structure.clone()`), `interactive.rs:226-251`, `rules.rs:310` and `:728` (`self.form().clone()`), `resolution.rs:229` (a whole `RefutationForm` built to keep its resolvents). `lambda.rs:19-46` borrows (`atoms: Option<&'a [Box<str>]>`, `nodes: &'a [Node]`). No ratchet journey writes JSON (`bench/ceilings.csv` has `read-json` only).

**Change.** Measure first: add a journey that writes a large proof and an ordinary outcome as JSON (as `prove --format json` and the web page do on every answer) and run it under DHAT. If the allocations show, write the proxies with borrowed slices and `collect_seq` over iterators, as `lambda.rs` does.

**Value.** The command and the web module write answers with fewer allocations; the page serializes every value it shows.

**Net.** The new journey's DHAT count and the ratchet; the lock pins the bytes. No checker or search is touched.

## F-6a-17

- dimension: idiom
- criterion: C-FEATURE
- place: core/Cargo.toml:65
- kind: API break
- cost: small
- candidate: no

**Claim.** The feature that adds serde is named `serialize`; the guidelines (C-SERDE) name it `serde`, which Cargo allows beside `dep:serde`.

**Witness.**

```toml
serialize = ["dep:serde"]
```

**Change.** Rename it `serde` in 0.9.0 (with F-6a-04's cut, so that it adds serde alone), and every `cfg(feature = "serialize")`; note it in the changelog.

**Value.** A user enables serde support under the name every crate uses.

**Net.** The `features` flake checks; CLI, bench and web manifests. No checker is touched.

## F-6a-18

- dimension: idiom
- criterion: E6
- place: core/src/wire.rs:620
- kind: refactor
- cost: small
- candidate: no

**Claim.** The wire levels are named constants scattered out of order across `wire.rs`, while levels 2 and 3 are bare numbers at four places, and nothing ties `LEVEL` to the highest level a form is written at.

**Witness.**

```rust
pub(crate) const fn level_of(
    refutation: &crate::linear::search::Refutation,
    mode: crate::linear::Mode,
) -> u32 {
    match refutation {
        crate::linear::search::Refutation::Basis(_) => BASIS,
        crate::linear::search::Refutation::Saturated(_) => 3,
        crate::linear::search::Refutation::Classical(_) => 2,
        _ => level_in(Some(mode)),
    }
}
```

`wire.rs:612` (`Some(mode) if mode.is_ordered() => 2`), `serialize/nets.rs:85` (`.max(2)`). The constants run `LEVEL` 31, `LAMBDA` 23, `UNTYPED` 30, `POLYMORPHIC` 31, `INHABITANTS` 24, `RULES` 15, … (`wire.rs:350-394`), then `CUTS` 6 to `BASIS` 11 after the functions (`:528-587`).

**Change.** One block of constants in level order, with `ORDERED` (2) and `SATURATED` (3), and `const _: () = assert!(LEVEL == POLYMORPHIC)` (or `LEVEL` defined as the last one).

**Value.** A reader finds every level once, in the module doc's order; adding a level cannot leave `LEVEL` behind.

**Net.** `wire_levels` in `core/tests/serialize.rs`; the bytes stay. No checker is touched.

## F-6a-19

- dimension: architecture
- criterion: E5
- place: core/src/serialize/ordinary.rs:1600
- kind: refactor
- cost: small
- candidate: no

**Claim.** `serialize/ordinary.rs` (2 065 lines) holds three things a reader keeps apart: the forms read back (sequent, derivation, outcome, session), the tables and tableaux, and the written-only transforms (comparison, interpolation, review, normal form, Herbrand, first-order resolution), and it rebuilds the sequent form from its three parts at eleven places.

**Witness.** Lines 1-1250 read back, 1252-1598 tables and tableaux, 1600-2065 written-only forms. The repeated call, as at `ordinary.rs:1891`:

```rust
        let form = |s: &Sequent| SequentForm::of(s.formulas(), s.left(), s.right());
```

also at `:585`, `:605`, `:625`, `:1278`, `:1656`, `:1661`, `:1672`, `:1733`, `:1982`, `:2037`; and `ReviewForm` builds a `CountermodelForm` inline (`:1660-1663`) beside `CountermodelForm::of` (`:603-608`).

**Change.** Split into `serialize/ordinary/{forms, tables, transforms}.rs` at those seams, and add `SequentForm::of_sequent(&Sequent)`.

**Value.** A reader of one form opens a file of a few hundred lines.

**Net.** The lock and `core/tests/wire/` pin the bytes; a move only.
