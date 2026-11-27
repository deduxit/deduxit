# Unit arch: findings

Read in full: 8 files (`core/src/lib.rs`, `limits.rs`, `settings.rs`,
`kripke.rs`, `goal_tree.rs`, `linear/mod.rs`, `rules/mod.rs` and
`core/Cargo.toml`). Not read in full: every other file of the crate, as
the role asks: `protocol/mod.rs` to line 979 of 1 029 (its types,
registry and front doors), each module's root to the end of its
declarations (`ordinary/mod.rs` to line 900, `modal/mod.rs` to 240,
`lambda/mod.rs` to 300, `export/mod.rs` to 330, `errors/mod.rs` to 355),
the front doors in their parts (`linear/search/mod.rs` lines 1 to 240,
1355 to 1535, 2836 to 3065, 3370 to 3830; `ordinary/decide.rs` 14 to 365
and 700 to 940; `modal/decide.rs` 1 to 290; `ordinary/derivation.rs` 470
to 630), and outlines by `grep` of the rest. `architecture.md` beside
this file has the design and the sketches the findings point at.

## F-arch-01

- dimension: architecture
- criterion: D4
- place: core/src/linear/search/mod.rs:3481
- kind: API break
- cost: medium
- candidate: no

**Claim.** `Reason`, why any suite's search stopped without deciding, is a type of the linear suite, and holds reasons that only ordinary logic's engines give.

**Witness.**

```rust
// core/src/linear/search/mod.rs:3557
    /// The connection prover reached its bound on the active path
    /// ([`Options::path_length`]) without a proof.
    PathLength {
        /// The bound.
        length: u32,
    },
```

```
// target/audit/public-api.txt:5260, :4396, :474
+pub deduxit::ordinary::Verdict::Unknown(deduxit::linear::search::Reason)
+pub deduxit::modal::Verdict::Unknown(deduxit::linear::search::Reason)
+pub deduxit::lambda::Inhabitation::Unknown(deduxit::linear::search::Reason)
```

`core/src/modal/decide.rs:14` is `use crate::linear::search::Reason;`. `ModelSize` (`:3566`) and `KripkeSize` (`:3576`) are the finite model search's. `core.md` (Layout) says "an item that a second suite comes to share without being linear logic moves to the root then".

**Change.** Move the enum whole to `limits`, beside `Refusal`, and re-export it as `deduxit::Reason`; the map from a `Refusal` to a `Reason` (`ordinary/decide.rs:354`, `ended`) moves there with it, where the protocol's suites can use it. The JSON form and `Reason::NAMES` do not change.

**Value.** A user of ordinary or modal logic matches on `deduxit::Reason` and no longer imports the linear suite to read a verdict; a reader finds the two ways a call ends without an answer in one module.

**Net.** The behaviour lock, the `wire-schema` check and `core/tests/stability.rs` (the form is unchanged); `breaks 0.8.0` lists the moved path. No checker, search or certificate is touched.

## F-arch-02

- dimension: api
- criterion: D4
- place: core/src/linear/search/mod.rs:3016; core/src/ordinary/decide.rs:751
- kind: API break
- cost: large
- candidate: no

**Claim.** Four options that only ordinary logic's engines read are fields of the linear search's `Options`, so deciding an ordinary sequent takes two options values, one of them another suite's.

**Witness.**

```rust
// core/src/linear/search/mod.rs:3015
    /// unknown with [`Reason::PathLength`]. Only that prover reads it.
    pub path_length: Option<u32>,
```

```rust
// core/src/ordinary/decide.rs:748
pub fn decide_within(
    sequent: &Sequent,
    options: &Options,
    search: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
```

The others are `restricted_backtracking` (`:3022`), `model_size` (`:3029`) and `model_worlds` (`:3037`). `Settings::default()` sets three of them on the linear options (`core/src/settings.rs:163-165`). `core-ordinary.md` ("Deciding") records the author's choice of the shape `decide`/`decide_within`; it gives no reason for where these four fields live, and D4 is later.

**Change.** Move the four fields to `ordinary::Options` (with their `with_…` setters and defaults); `decide_within` keeps a `&linear::search::Options` for the embedding alone, named so (`embedding`). The settings keys become `ordinary.path_length`, `ordinary.restricted_backtracking`, `ordinary.model_size` and `ordinary.model_worlds`, and `Reason::setting` follows.

**Value.** A user of the library sets the connection prover's bound on the options of the logic it belongs to; a user of the command or the page reads `ordinary.*` for ordinary logic's settings and `search.*` for the linear search alone.

**Net.** The behaviour lock and the examples of the manual and the guide (the settings keys are named there and checked by `cli/tests/readme.rs`); the protocol's `reads` lists, which are cache keys (`core-protocol.md`), change with the keys; the tab's defaults in `web-tab`. No engine's search changes: the target sets' counters stay equal.

## F-arch-03

- dimension: api
- criterion: X2
- place: core/src/limits.rs:429; core/src/limits.rs:593
- kind: refactor
- cost: small
- candidate: no

**Claim.** An engine outside the crate can neither poll the stop it is given nor refuse as the crate's engines do, because `Progress` and `Refusal` have no public constructor.

**Witness.**

```rust
// core/src/limits.rs:426
impl Progress {
    /// Returns the progress of a call in `phase` that has done `done`
    /// units of work, `work` of them since the previous poll.
    pub(crate) const fn new(phase: Phase, work: u64, done: u64) -> Self {
```

`Progress` is `#[non_exhaustive]` (`:404`), and every variant of `Refusal` is a `#[non_exhaustive]` struct variant (`:595-705`). The consumer test's attempts 1 and 3 fail with E0639.

**Change.** Make `Progress::new` public and add `with_held_bytes`; add `Refusal::stopped(phase)`, `Refusal::work(limit)` and `Refusal::memory(phase, limit_bytes, needed_bytes)`; add a public `limits::Meter` (count, poll, refuse), which `rules/check.rs:29` and `modal/model.rs:376` each hold as a private struct today (`architecture.md`, section 5, part 1).

**Value.** A researcher's engine honours `Limits` and the stop with three lines, and its refusals sort into exit status 3 through `Error::kind` like the crate's own.

**Net.** Additive; `core/tests/limits.rs` and the two checkers' tests cover the meter. `core.md` says "A front end therefore never builds an error of the library": that rule is about front ends reporting faults, and holds; an engine's refusal is the case X2 names.

## F-arch-04

- dimension: architecture
- criterion: X4
- place: core/src/linear/search/mod.rs:1477; core/src/linear/search/mod.rs:3389
- kind: refactor
- cost: medium
- candidate: no

**Claim.** No engine from another crate can answer in the crate's terms or be run by the front door: the engine interface is crate-private, and `Outcome` cannot be built.

**Witness.**

```rust
// core/src/linear/search/mod.rs:1477
pub(crate) trait Decide {
    /// Refuses a goal the engine does not decide.
```

```rust
// core/src/linear/search/mod.rs:3387
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Outcome {
```

`Task` (`:1362`), `Answer` (`:1416`), `memory::Account` and `Work` in `decide`'s signature (`:1497-1505`) are all `pub(crate)`. Consumer test, point 4.

**Change.** A public trait `linear::search::Decide` over `Goal`, `Mode`, `&Limits` and a `Progress` stop, returning `Found { Proved(Proof), Unprovable(Option<Refutation>), Unknown(Reason) }`, and `prove_with(&dyn Decide, goal, mode, &options, &limits, stop) -> Result<Outcome, Error>`, which goes through `conclude` as every built-in engine does; for the ordinary suite `Outcome::of_derivation`, `of_countermodel` and `unknown`. Signatures and the cost to the present code are in `architecture.md`, section 5, part 3.

**Value.** A researcher's engine returns the `Outcome` the command, the batch and the bindings print, with its proof checked by the crate's checker; the manual's page "Write your own engine" (X8) has an interface to teach.

**Net.** The crate-private trait and every built-in engine keep their code; an adapter implements the private trait for `&dyn Decide`. The target sets' counters (`bench/targets.sh`, `bench/net-targets.sh`) show the searches unchanged. The checker stays the trust boundary: `prove_with` checks before it says `checked`.

## F-arch-05

- dimension: api
- criterion: X1
- place: core/src/linear/sequents/mod.rs:415; core/src/ordinary/derivation.rs:605; core/src/lambda/mod.rs:212
- kind: refactor
- cost: medium
- candidate: no

**Claim.** Three values of the data model can be made only from text or JSON: a linear `Sequent`, an `ordinary::Derivation` and a `lambda::Term`.

**Witness.**

```rust
// core/src/linear/sequents/mod.rs:414
    #[cfg(feature = "parse")]
    pub(crate) fn from_parts(
        terms: Vec<Term>,
        roots: Vec<TermId>,
        atoms: Vec<String>,
        antecedents: Option<u32>,
    ) -> Self {
```

```rust
// core/src/ordinary/derivation.rs:605
    pub(crate) fn from_parts(
        logic: Logic,
        formulas: Formulas,
        (left, right): (Vec<NodeId>, Vec<NodeId>),
        inferences: Vec<Inference>,
    ) -> Self {
```

`ordinary::Inference` has `pub(crate)` fields and getters alone (`ordinary/derivation.rs:400-451`). `lambda::Term` has `parse` and `read` and no constructor, and its ids (`TermId`, `VarId`, `TypeId`) have `index()` alone (`lambda/mod.rs:212-251`; `public-api-dedup.txt:617-642`). With `default-features = false` there is no linear sequent at all. The ordinary and modal arenas show the form that works: `Formulas::add`, `atom`, `Sequent::new`.

**Change.** `Sequent::atom(&mut self, &str)`, `push(&mut self, Term)` and `push_root(&mut self, TermId)`, each validating as `Sequent::check` does (`sequents/mod.rs:586`); `ordinary::Inference::new` with `with_principal`, `with_premises`, `with_term`, and `ordinary::Derivation::new(…) -> Result<Self, Error>`, which checks the shape and marks the value unchecked; a builder for `lambda::Term` on the same lines (`architecture.md`, section 5, part 2).

**Value.** An engine that makes a formula (a cut formula, an interpolant, a translation's image) or a derivation of LK or LJ hands it to the checker without printing and parsing it; a generated family needs no parser.

**Net.** Additive. The constructors run the validation the JSON readers run, so `core/tests/serialize.rs` and the readers' fuzz targets are the model for their tests; `Derivation::check` and `Proof::check` stay the only way to a checked value.

## F-arch-06

- dimension: architecture
- criterion: D4
- place: core/src/linear/search/memory.rs:21; core/src/linear/proofs/style.rs:231; core/src/linear/sequents/notation.rs:20; core/src/ordinary/mod.rs:154
- kind: refactor
- cost: medium
- candidate: no

**Claim.** Four crate-private pieces that every suite uses live in the linear suite, one of them making the linear suite name the ordinary one.

**Witness.**

```rust
// core/src/linear/proofs/style.rs:227
/// What an output needs from a derivation to draw it: the tree, each
/// inference's rule and label, and each inference's sequent, as text or
/// in a target's notation. The linear derivations and those of LK and LJ
/// implement it, so every output draws both.
pub(crate) trait Drawn {
```

```rust
// core/src/linear/sequents/notation.rs:20
use crate::ordinary::{Formulas, NodeId, Symbols};
```

`Drawn` is implemented in `ordinary/derivation.rs:1149`, `lambda/draw.rs:597` and `rules/draw.rs:276`. `memory::Account` is named by `ordinary/ljt.rs`, five files of `ordinary/sat/`, `ordinary/resolution/outside.rs` and `modal/search.rs`. `Stop` and `Stops` are imported at `ordinary/mod.rs:154` and `modal/mod.rs:71`. `plan/later.md:2727` lists the account, `Stops` and `Drawn` as a move "by the first session that edits one of them", since 2026-11-16; `name.rs`, `goal_tree.rs` and `first_order.rs` were moved, these were not.

**Change.** `linear/search/memory.rs` to `core/src/memory.rs`; `Stops` and `Notation` to a root module `notation`; `Drawn` to `export` beside `export/notation.rs`'s walk. No public path changes.

**Value.** A reader of the ordinary or the modal suite finds the shared machinery at the root, as `core.md`'s layout rule says; `linear` no longer names `ordinary`, which a feature per suite needs.

**Net.** The gate (the code is the same in the same crate); the ratchet's journeys, since `Account` is charged in the focused engine's loop: it stays in one crate under fat LTO, so no count should move.

## F-arch-07

- dimension: api
- criterion: D4
- place: core/src/linear/sequents/mod.rs:21; core/src/linear/proofs/mod.rs:47; core/src/linear/proofs/mod.rs:52; core/src/ordinary/parse.rs:15
- kind: API break
- cost: large
- candidate: no

**Claim.** Public types that belong to every suite's derivations, sessions and readers are reachable only as items of the linear suite, and stand in the public signatures of `ordinary`, `export` and `Settings`.

**Witness.**

```
// target/audit/public-api.txt
17:   +pub deduxit::export::latex::Options::labels: deduxit::linear::proofs::style::Labels
395:  +pub deduxit::export::Styles::text: deduxit::linear::proofs::TextOptions
5285: +pub fn deduxit::ordinary::Derivation::root(&self) -> deduxit::linear::InfId
5287: +pub fn deduxit::ordinary::Derivation::shown(&self, deduxit::linear::proofs::Structural) -> alloc::borrow::Cow<'_, Self>
5369: +pub fn deduxit::ordinary::Interactive::goals(&self) -> impl core::iter::traits::iterator::Iterator<Item = deduxit::linear::proofs::interactive::GoalId> + '_
5458: +pub deduxit::ordinary::Problem::status: core::option::Option<deduxit::linear::lltp::Status>
5492: +pub deduxit::ordinary::Step::witness: core::option::Option<deduxit::linear::sequents::FoTermId>
```

`FoTermId` is defined at the root (`core/src/first_order.rs`, a private module) and published at `core/src/linear/sequents/mod.rs:21` (`pub use fo::{Args, Eigen, FoTerm, FoTermId, Symbol, SymbolFault};`). `kernel-boundary.md` (6.1) left `InfId`, `GoalId` and `Labels` in place "until a split re-exports them", so that no public path changed; D2 and D4 are later and remove that ground.

**Change.** Publish at the root: `deduxit::terms` (the first-order `FoTerm`, `FoTermId`, `Symbol`, `Args`, `Eigen`, `SymbolFault`), and in a root module for derivations (`deduxit::derivation`, or `export`): `InfId`, `GoalId`, `Labels`, `OpenGoal`, `TextOptions`, `Structural`, `Inverted`; the TPTP clause reader and its `Status` at the root. The linear suite re-exports none of them.

**Value.** A user of ordinary or modal logic writes no `deduxit::linear::` path unless they use linear logic; rustdoc shows each shared type once, under the module that owns it.

**Net.** `breaks 0.8.0` lists every moved path for the changelog and the migration note; the manual's and the guide's examples (`cli/tests/readme.rs`) name some of them. No behaviour changes.

## F-arch-08

- dimension: architecture
- criterion: E6
- place: core/src/modal/mod.rs:170 and core/src/ordinary/mod.rs:824; core/src/modal/parse.rs:333 and core/src/ordinary/parse.rs:771; core/src/serialize/modal.rs:67 and core/src/serialize/ordinary.rs:71
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The hash-consed arena of propositional formulas, its sequent, its parser and its JSON form exist twice, in the ordinary and the modal suite, and the step the plan waited for has landed.

**Witness.** `duplicates.md`: `modal/mod.rs:170` ↔ `ordinary/mod.rs:824` (63 lines), `:236` ↔ `:898`, `:338` ↔ `:1607` (71 lines), `:456` ↔ `:1744`, `:470` ↔ `:1757`; six pairs between `modal/parse.rs` and `ordinary/parse.rs`; six between `serialize/modal.rs` and `serialize/ordinary.rs`. Both arenas' `add`:

```rust
// core/src/modal/mod.rs:206 and core/src/ordinary/mod.rs:864
    pub fn add(&mut self, node: Node) -> Result<NodeId, Error> {
        if let Some(&id) = self.ids.get(&node) {
            return Ok(id);
        }
        let (a, b) = node.operands();
        for id in [a, b].into_iter().flatten() {
            self.holds(id)?;
        }
```

`core-modal.md:263` records the copy "for the kernel crate"; `plan/later.md:2728` makes it one copy at "step 74, which gives `ordinary::Node` quantifiers", and `ordinary::Node::Forall` is at `ordinary/mod.rs:742`.

**Change.** A crate-private `Arena<N>` at the root (the nodes, the hash-consing map, the foreign-id check, the `u32` refusal) over a small trait of the node (`operands`), a shared loop for the two parsers over a table of operators, and one sequent form generic over the formula's form. `ordinary::Formulas` and `modal::Formulas` stay two public types with distinct ids.

**Value.** A reader, and the next suite with formulas of this shape: a fix to the arena or the parser is made once.

**Net.** The ratchet's `read-tptp`, `read-json` and `ordinary-pigeons` (the arena is on their paths; the generic is monomorphised, so no count should move); the parse and serialize tests; the fuzz targets of both readers.

## F-arch-09

- dimension: architecture
- criterion: C-CONV-SPECIFIC
- place: core/src/rules/ordinary.rs:131; core/src/rules/search.rs:15; core/src/ordinary/written/mod.rs:32
- kind: API break
- cost: medium
- candidate: no

**Claim.** The module of calculi given as data, "which belongs to no suite", holds 2 213 lines of ordinary logic, and the two modules name each other.

**Witness.**

```rust
// core/src/rules/search.rs:14
use crate::linear::search;
use crate::ordinary::{self, Evidence, Formulas, Method, Verdict};
```

```rust
// core/src/rules/ordinary.rs:131
    pub fn from_ordinary(derivation: &ordinary::Derivation) -> Result<Self, Error> {
```

```rust
// core/src/ordinary/written/mod.rs:32
use crate::rules::{Calculus, Derivation, Node as RuleNode, Step as RuleStep, TermId, terms_of};
```

`rules/ordinary.rs` (221 lines), `natural.rs` (1 249), `tableau.rs` (155) and `search.rs` (588) import `crate::ordinary`; the other nine files of `rules` name no suite but for the drawing types of F-arch-06 and F-arch-07.

**Change.** Move the four files to `ordinary::natural` (and `ordinary::tableau`), and put each conversion on the specific type: `ordinary::Derivation::to_rules()` and `natural(…)`, `Tableau::to_rules()`, the session's `close` and `hint` for natural deduction in the ordinary suite.

**Value.** `rules` is the generic checker and nothing else, so a new logic reads one module to answer with a rule file and a derivation; a user looking for natural deduction of ordinary logic finds it in the ordinary suite.

**Net.** `core/tests/rules.rs` and `core/tests/natural.rs`; the behaviour lock; `breaks 0.8.0` for the moved functions. The rule files and the checker are untouched.

## F-arch-10

- dimension: api
- criterion: X6
- place: core/src/protocol/mod.rs:354
- kind: API break
- cost: small
- candidate: no

**Claim.** The docs of `protocol::Suite` say a suite of another crate may implement it, and the types make that impossible.

**Witness.**

```rust
// core/src/protocol/mod.rs:354
/// What a suite offers. Each suite of the crate implements it, and a
/// suite of another crate may; [`suites`] lists this build's.
pub trait Suite: Sync {
```

`kinds()` returns `&'static [Kind]`, and `Kind`, `Operation`, `Parameter` and `Promise` are `#[non_exhaustive]` structs without a constructor (`:123-197`); `run` returns `Produced`, whose constructor is private (`:259`), and `check` a `Check` that is `#[non_exhaustive]` (`:311`). The registry is a fixed static (`:418`), and `run`, `check`, `kind`, `operation` and the workspace (`workspace/mod.rs:1062`) read that static alone.

**Change.** For 0.9.0, seal the trait as `Owner` and `wire::Readable` are sealed and say so in its docs. If outside suites are wanted (and for a web module per suite, F-arch-18), the other way: constructors for the data types and a `Registry` value that `run`, `check` and `Workspace` take, `Registry::default()` holding this build's suites.

**Value.** A user is not told of an extension point that does not exist; the author decides the open registry once, with its cost known.

**Net.** Sealing changes no behaviour (`protocol/tests.rs`). The open registry would thread one parameter through `workspace` and the bindings' workspace calls.

## F-arch-11

- dimension: architecture
- criterion: E6
- place: core/src/protocol/mod.rs:505; core/src/protocol/mod.rs:432; core/src/protocol/mod.rs:706
- kind: refactor
- cost: small
- candidate: no

**Claim.** The protocol's registry is written four times in one file, and one copy has drifted: `KINDS` names no kind of the lambda suite.

**Witness.**

```rust
// core/src/protocol/mod.rs:523
    "modal.sequent",
    "modal.outcome",
    "modal.model",
    "modal.evaluation",
    "drawing",
    "text",
    "certificate",
    "term",
];
```

`core/src/protocol/lambda.rs:29` has `static KINDS: [Kind; 5]` (`lambda.term`, `lambda.typing`, `lambda.inhabitation`, `lambda.inhabitants`, `lambda.reduction`). `core-protocol.md:173` records it as "a gap that predates them; the test holds `OPERATIONS` alone". `KINDS` is the `known` list of `Error::UnknownName` (`:576`), so the message for a misspelt kind lists no lambda kind. The other copies: `SUITES` (`:418`), `OPERATIONS` (`:432`), and the `match` of `statement` (`:706-734`) with its own list of suites (`:732`).

**Change.** Build both `known` lists once from `suites()` (a `LazyLock<Vec<&'static str>>` lent as a static slice) and delete the two constants and their test; make `statement` a method of `Suite`. With it, move the duplicated blocks of `protocol/{linear, ordinary, modal, lambda, rules}.rs` (`duplicates.md`: `linear.rs:234` ↔ `ordinary.rs:489`, `lambda.rs:197` ↔ `modal.rs:173` ↔ `rules.rs:132`, and eleven more rows) into `protocol/shared.rs`.

**Value.** A new suite or kind is added in one place (`core-protocol.md`, "Adding an operation", lists six today); the error a front end shows is complete.

**Net.** `protocol/tests.rs`; the catalogue's JSON, which the `wire-schema` check validates, is unchanged.

## F-arch-12

- dimension: architecture
- criterion: E6
- place: web/src/protocol.rs:151; web/src/lib.rs:557; core/src/rules/interactive.rs:430; core/src/linear/proofs/interactive/mod.rs:905
- kind: API break
- cost: large
- candidate: no

**Claim.** The three sessions have no interface in common, so every session call is written three times in the bindings and described in a second protocol there, outside the library's.

**Witness.**

```rust
// web/src/lib.rs:557
    pub fn rules_candidates(session: &str, goal: f64, settings: &str) -> Result<String, JsValue> {
```

with `candidates` at `:109` and `ordinary_candidates` at `:308`, over `web/src/calls.rs:715`, `web/src/ordinary.rs:270` and `web/src/rules.rs:422`; the one answer form is documented in the bindings (`web/src/calls.rs:642`). `web/src/protocol.rs:151` is `pub const SUITES: [Speaks; 5]`, twelve calls a suite written as `offered("candidates", "rules_candidates", GOAL)` (`:332`) or `lacks("candidates")` (`:378`). The copies show in `duplicates.md`: `web/src/calls.rs:1709` ↔ `web/src/ordinary.rs:1324`, `web/src/ordinary.rs:1332` ↔ `web/src/rules.rs:818`, `cli/src/interact/mod.rs:593` ↔ `cli/src/interact/ordinary.rs:254` and four more pairs there. The typed methods differ in name where they mean the same: `is_complete` (`linear/proofs/interactive/mod.rs:905`) against `is_finished` (`rules/interactive.rs:430`); `goals()` an iterator (`linear/proofs/interactive/mod.rs:882`) against a `Vec` (`rules/interactive.rs:416`).

**Change.** A trait `protocol::Session` over JSON documents (`start`, `view`, `candidates`, `apply`, `undo`, `close`, `hint`), implemented by the three suites and offered by `Suite::session()`; the step and candidate forms become wire forms of the library, as `web.md` expects ("When the library gives them one … the bindings take it"). The bindings export one set of session calls with the suite as an argument and drop their table; the three `Interactive` types agree on `is_finished`, `goals()` and `goal(id)` (`architecture.md`, section 7).

**Value.** A fourth session costs one implementation, not three sets of calls in two front ends; the page and the command read which calls a suite has from the library's catalogue.

**Net.** `web/tests/*.rs` and the client's calls (B6: the exports are a contract with the client, changed with it); `cli/tests/lock.rs` for `interact`. The sessions' own code is untouched but for the renames. The module's bytes are measured before and after (fewer exports, one more table).

## F-arch-13

- dimension: api
- criterion: C-CUSTOM-TYPE
- place: core/src/ordinary/decide.rs:72; core/src/modal/decide.rs:106
- kind: API break
- cost: medium
- candidate: no

**Claim.** What refutes a sequent that is not valid is three optional fields beside the verdict in the ordinary outcome and one in the modal one, where the linear verdict carries its disproof in the variant; an outcome can then say "valid" and hold a countermodel.

**Witness.**

```rust
// core/src/ordinary/decide.rs:72, :79, :85
    pub countermodel: Option<super::Countermodel>,
    pub structure: Option<Box<super::FiniteCountermodel>>,
    pub kripke: Option<Box<super::KripkeCountermodel>>,
```

```rust
// core/src/linear/search/mod.rs:3440
    Unprovable(Box<Disproof>),
```

Each field's doc says "`None` for every other verdict", and the outcome's reader refuses "one with a countermodel beside a verdict other than `not_valid`" (`ordinary/decide.rs:50`): the invariant is kept by a reader, not by the type.

**Change.** `Verdict::NotValid(Option<Refuter>)` with `enum Refuter { Kripke(Countermodel), Finite(Box<FiniteCountermodel>), Growing(Box<KripkeCountermodel>) }` in the ordinary suite, `Verdict::NotValid(Option<Countermodel>)` in the modal one. The JSON forms stay as they are.

**Value.** A user of the library matches one value to get the verdict and its evidence, in the same shape in three suites; the state the reader refuses cannot be built.

**Net.** The wire forms are unchanged (`wire-schema`, `core/tests/serialize.rs`); `breaks 0.8.0` lists the fields removed. The checks of the models are untouched.

## F-arch-14

- dimension: api
- criterion: D4
- place: core/src/settings.rs:135; core/src/errors/mod.rs:302
- kind: API break
- cost: large
- candidate: no

**Claim.** In `Settings` and in `Error` the linear suite's items carry no suite's name and the other suites' do, so the names make it the one that needs none.

**Witness.**

```rust
// core/src/settings.rs:134
    /// The search's options.
    pub search: Options,
    /// How a derivation is shown.
    pub view: ViewOptions,
```

with `batch`, `interactive`, `input` and `cut` (`:141-150`), all the linear suite's options, beside `pub ordinary: ordinary::Options` (`:143`) and `pub lambda: crate::lambda::Options` (`:152`); the modal suite has no field.

```rust
// core/src/errors/mod.rs:302 and :309
    Step(#[from] crate::linear::proofs::StepError),
    OrdinaryStep(#[from] crate::ordinary::StepError),
```

**Change.** Group the linear suite's options under `Settings::linear` (`search`, `batch`, `input`, `cut`, `interactive`), keep what every suite reads at the top (`clock`, `limits`, `view`, `styles`), add `modal`; rename `Error::Step` to `LinearStep`. The settings keys become `linear.search.copies` and so on.

**Value.** A user of the command or the page reads from a key which suite it configures; D4 holds in the one value every front end shows.

**Net.** Every settings key is in the manual, the guide, the command's help and the tab's defaults, and `cli/tests/readme.rs` checks each name: the cost is there, and the author may weigh it against F-arch-02 alone. The error's `code` need not change.

## F-arch-15

- dimension: api
- criterion: D4
- place: web/src/lib.rs:97; core/src/protocol/mod.rs:418; core/src/protocol/mod.rs:732
- kind: API break
- cost: large
- candidate: no

**Claim.** The bindings export the linear suite's calls without a prefix and every other suite's with one, and the protocol lists the linear suite first.

**Witness.**

```rust
// web/src/lib.rs:97 and :296
    pub fn start(sequent: &str, mode: &str, settings: &str) -> Result<String, JsValue> {
    pub fn ordinary_start(sequent: &str, logic: &str, settings: &str) -> Result<String, JsValue> {
```

```rust
// core/src/protocol/mod.rs:417
/// The suites of this build, in the order a front end shows them.
static SUITES: [&dyn Suite; 5] = [
    &linear::LINEAR,
    &ordinary::ORDINARY,
```

and `known: &["linear", "ordinary", "modal", "lambda"]` (`:732`).

**Change.** Order both lists as D4 gives it (ordinary, linear, modal, lambda, then the calculi given as data). Name the linear exports `linear_start`, `linear_apply` and so on, or drop the per-suite sets for the one of F-arch-12.

**Value.** The page's menus come out in the order the author decided without the client reordering them; a reader of the bindings sees five suites named alike.

**Net.** B6: the exports are the client's contract, so the change lands with the client's (`web/tests/calls.rs`, the client's own checks). The catalogue's order is data the page reads; `protocol/tests.rs` pins it.

## F-arch-16

- dimension: api
- criterion: C-FEATURE
- place: core/src/lib.rs:350; core/src/modal/mod.rs:45
- kind: build
- cost: medium
- candidate: no

**Claim.** The feature `serialize`, named for the JSON forms, also turns on a proof checker (`rules`) and the decision procedure of modal logic.

**Witness.**

```rust
// core/src/lib.rs:350
#[cfg(feature = "serialize")]
pub mod rules;
```

```rust
// core/src/modal/mod.rs:44
/// Deciding a sequent in a system.
#[cfg(feature = "serialize")]
mod decide;
```

`modal/mod.rs:20` says "Deciding needs the cargo feature `serialize`, as calculi given as data do", and `modal/decide.rs:19-22` derives serde's traits without a `cfg` for that reason.

**Change.** Gate on `serialize` only what reads or writes a document: the rule file's reader and the forms. `rules::Calculus` gets a constructor from its parts, which `modal::calculus` uses, and `rules`, `modal::decide` and the modal tableau build without serde.

**Value.** A user who decides modal logic or checks a derivation in a built-in calculus with `default-features = false` gets it; a feature does what its name says.

**Net.** Both `cargo hack` runs (each feature and every pair) and `features-tests`; no behaviour changes with the default features.

## F-arch-17

- dimension: architecture
- criterion: X7
- place: core/Cargo.toml:48; core/src/linear/mod.rs:25
- kind: build
- cost: large
- candidate: yes

**Claim.** No feature gives the data model and the checkers alone: `default-features = false` removes every reader and keeps every engine of every suite.

**Witness.**

```rust
// core/src/linear/mod.rs:24
pub mod proofs;
pub mod search;
```

`search` and the ordinary suite's `connection`, `ljt`, `normal` and `resolution` (`ordinary/mod.rs:69-102`) carry no `cfg`. `timings.md`: the core crate alone in the dev profile takes 40 s with the default features and 16 s with none, and what the 16 s build is cannot make a linear sequent (F-arch-05).

**Change.** A default feature `search` that gates the engines and the front doors (`linear::search`'s engines, `ordinary::decide` and its method modules, the sessions' `close` and `hint`), leaving sequents, forests, proofs, derivations, models and the checkers, the refutations' checks among them. It needs `linear/search/mod.rs` divided first (F-arch-19), since the outcome and refutation types live with the front door.

**Value.** A researcher's crate that brings its own engine compiles the model and the checkers and not the engines; X7 is met.

**Net.** To measure before it is proposed: the share of the 16 s the engines take, by a build with the engine modules compiled out on a scratch copy (`cargo build --timings`), or `cargo llvm-lines` by module path. Both `cargo hack` runs grow by one feature (78 pairs for 13 features). No behaviour changes with the default features.

## F-arch-18

- dimension: performance
- criterion: B6
- place: core/src/workspace/mod.rs:1062; core/src/protocol/mod.rs:418
- kind: build
- cost: large
- candidate: yes

**Claim.** The web module grew by 549 168 bytes at gzip in six days and every visitor loads it whole; D29's condition for a module per suite ("a measured size") is met, and the library has one obstacle to it: the workspace runs operations through a static registry of every suite.

**Witness.**

```rust
// core/src/workspace/mod.rs:1062
                match protocol::run(
```

`protocol::run` reads `static SUITES` (`protocol/mod.rs:418`), so a module that exports any `workspace_*` call links every suite. `web.md` ("The budget"): 2 438 116 bytes on 2026-11-21 once the renderers left, 2 987 284 now by the audit's brief, the ceiling raised three times in between (2 650 000, 2 900 000, 2 950 000, 3 020 000). The renderers' module (`web.md`, "The renderers are a module of their own") took 1.48 MB off the first load at a boundary of JSON in, bytes out, and nothing is kept between calls. The package is built with `wasm-opt -O3` (`web.md`, "The build").

**Change.** In this order: (A) attribute the bytes (`twiggy top` and `twiggy dominators` on a build with names kept, by the library's module path); (B) build the module for size (`wasm-opt -Oz`, or `opt-level = "s"` for the bindings, which P10 lists as open) and time the calls in it; (C) if the exporters weigh enough, a late module for LaTeX, Typst and Rocq as the renderers are; (D) only then a module per suite, which needs the registry as a value (F-arch-10) so that the workspace's module does not hold the suites (`architecture.md`, 6.1).

**Value.** A visitor of the page downloads less before the first proof; the budget stops being raised at each step.

**Net.** `budget` (the bytes), `web-wasm-tests`, the client's own checks. Instruments: `twiggy` for A; `gzip -9 | wc -c` and a timed run of `web/tests/calls.rs` under Node for B. Not a finding of fact until A is run.

## F-arch-19

- dimension: architecture
- criterion: E5
- place: core/src/linear/search/mod.rs:1362; core/src/linear/search/mod.rs:1976; core/src/linear/search/mod.rs:2855; core/src/linear/search/mod.rs:3389
- kind: refactor
- cost: medium
- candidate: no

**Claim.** `linear/search/mod.rs` holds five things in 5 552 lines, each with a clear edge.

**Witness.** Its items by line (`grep -n` of the item heads): `prove` to `prepare`, the front door (238 to 1 357), the engine interface `Task`, `Finished`, `Answer`, `Decide` (1 362 to 1 506), the dispatch and its table (1 515 to 1 970), `pub enum Engine`, `Counter`, `NotTaken` (1 976 to 2 400), `Bias`, `Jobs`, `Cadence`, `Schedule`, `pub struct Options` (2 400 to 3 389), `Outcome`, `Verdict`, `Reason`, `Statistics` (3 389 to 3 846), the tests after. The module doc (`:4-16`) describes the first alone.

**Change.** `search/options.rs`, `search/outcome.rs` and `search/engine.rs` (the catalogue, the private trait and the dispatch), `mod.rs` keeping the front door and re-exporting the rest at today's paths.

**Value.** A reader opens the file of the thing they look for; F-arch-01, F-arch-02, F-arch-04 and F-arch-17 each change one of the parts and are smaller after it.

**Net.** No public path changes (`breaks 0.8.0` empty for it); the target sets' counters, since the front door is on every search's path; the gate.

## F-arch-20

- dimension: architecture
- criterion: E5
- place: core/src/ordinary/mod.rs:709; core/src/ordinary/interactive.rs:1069
- kind: refactor
- cost: medium
- candidate: no

**Claim.** `ordinary/mod.rs` is the decision's vocabulary and the formula arena in one file, and `ordinary/interactive.rs` is one file of 5 258 lines where the linear session, its model, is nine.

**Witness.** `ordinary/mod.rs`: `Logic`, `Translation`, `Target`, `Method`, `Options` (lines 167 to 707), then `NodeId`, `Node`, `Formulas`, `Formula`, `Sequent` (709 to 1 770), tests to 2 379. `ordinary/interactive.rs`: the errors (45 to 345), `Step`, `Applicable`, `Candidate`, `Closed`, `Hint` (402 to 560), `Interactive` and its witnesses (635 to 1 070), `rules` and `candidates` (1 274 to 1 400), `apply` (1 600), `witness` (2 054), `undo` (2 394), `close` and `hint` (2 508 to 2 990), the derivation (2 995 to 3 400), `Replay` (3 633), `Tree` (3 944). `linear/proofs/interactive/` has `mod.rs`, `expand.rs`, `witness.rs`, `terms.rs`, `replay.rs` and four more.

**Change.** `ordinary/formulas.rs` and `ordinary/options.rs`; `ordinary/interactive/{mod, error, step, expand, witness, close, derivation, replay}.rs` on the linear session's seams.

**Value.** A reader, and the reviewer of any later change to the session; the arena's file is what F-arch-08 edits.

**Net.** No public path changes; the gate; `core/tests/first_order.rs` and the session's tests.

## F-arch-21

- dimension: architecture
- criterion: D4
- place: core/src/ordinary/model.rs:175 and core/src/modal/model.rs:176; core/src/modal/model.rs:18
- kind: API break
- cost: small
- candidate: no

**Claim.** The shared Kripke model has its methods written in two suites, undivided in rustdoc, and the modal check is bounded by a constant of the ordinary suite.

**Witness.**

```rust
// core/src/modal/model.rs:18
use crate::ordinary::MODEL_CELLS;
```

`ordinary/model.rs:175` is `impl Model {` with `forcing` (`:210`) and `refutes` (`:449`); `modal/model.rs:176` is `impl Model {` with `frame_fault` (`:196`) and `truth` (`:225`). `modal/decide.rs:104` documents the modal outcome with `[MODEL_CELLS](crate::ordinary::MODEL_CELLS)`. Consumer test, points 10 and 11: two calls check a countermodel (`Model::refutes` and `Countermodel::check`), and which to call is not said.

**Change.** `kripke::MODEL_CELLS` at the root. Keep the model as data and put each suite's check on that suite's `Countermodel` (`ordinary::Countermodel::check`, `modal::Countermodel::check`), `forcing` and `truth` beside them as that suite's functions over a `&Model`.

**Value.** A user of one suite sees that suite's check on the page of that suite's type, and one way to check a countermodel.

**Net.** The model checks' tests in both suites; `breaks 0.8.0` for the moved methods. The checks' code is untouched.

## F-arch-22

- dimension: api
- criterion: C-FEATURE
- place: core/Cargo.toml:60; core/src/export/mod.rs:85
- kind: build
- cost: medium
- candidate: no

**Claim.** `latex`, `typst` and `svg` are three features that no consumer in the workspace enables apart, with no dependency of their own, and they cost 258 `cfg` sites and 21 of the 66 feature pairs the checks build.

**Witness.**

```rust
// core/src/export/mod.rs:85
#[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
pub(crate) mod models;
```

`grep -rn 'feature = "latex"' core/src` finds 91 lines, `typst` 82, `svg` 85; 51 lines are an `any(…)` that names one of them. `cli/Cargo.toml:50`, `web/Cargo.toml:18`, `web-tab/Cargo.toml:19` and `bench/Cargo.toml:23` each list all three. `core/Cargo.toml:60-67` gives them no dependency (`latex = []`, `svg = []`, `typst = []`).

**Change.** One feature `export` for the three text formats, `png` and `pdf` on top of it, `rocq` as it is (the harness builds without it). `lib.rs:309` promises no feature is removed "within a 0.y version"; 0.9.0 is the next one.

**Value.** A user reads ten features instead of twelve; `features` and `features-tests` build 45 pairs instead of 66 (B7).

**Net.** Both `cargo hack` runs; the `export` check. A user who enabled one format alone gets all three, which costs them compile time and no behaviour.

## F-arch-23

- dimension: api
- criterion: X3
- place: core/src/ordinary/derivation.rs:510; core/src/linear/search/mod.rs:3408
- kind: text
- cost: small
- candidate: no

**Claim.** Whether a proof in hand has passed its checker is recorded in two ways in two suites, and neither is said on the types a user holds.

**Witness.**

```rust
// core/src/ordinary/derivation.rs:507
    /// Whether the derivation is one its checker accepts, or a view of
    /// one. It is false for a derivation read from a document until
    /// [`check`](Derivation::check) accepts it.
    trusted: Trusted,
```

```rust
// core/src/linear/search/mod.rs:3404
    /// Whether the proof passed the checker before it was returned. It is
    /// false only where [`Options::check`] switched the check off, and for
    /// every verdict but a proof. In an outcome read from a document it is
    /// what the document says: [`Proof::check`] checks the proof.
    pub checked: bool,
```

The ordinary flag is private with no getter; a linear `Proof` carries none, and `Verdict::Proved(Box<Proof>)` holds any proof (consumer test, point 5).

**Change.** Say the rule once on `linear::Proof`, `Verdict::Proved`, `ordinary::Derivation` and `Evidence`: a value in a verdict the crate returned has passed the checker unless the outcome's `checked` is false; a value built or read has not until `check` is called. Give `ordinary::Derivation` a public `is_checked()` over its flag.

**Value.** A user who receives a proof from another program knows what to call before trusting it.

**Net.** Docs and one getter; no checker changes.

## F-arch-24

- dimension: architecture
- criterion: E6
- place: core/src/ordinary/connection/prove.rs:650 and core/src/ordinary/connection/intuitionistic/prove.rs:705
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The intuitionistic connection prover repeats the classical one's search loop: thirteen duplicates of up to 115 lines.

**Witness.** `duplicates.md`: `intuitionistic/prove.rs:705` ↔ `prove.rs:650` (108 lines), `:535` ↔ `:485` (101), `:61` ↔ `:83` (115), and ten more. Both files hold, word for word (`prove.rs:652`, `intuitionistic/prove.rs:707`):

```rust
    /// Returns whether the path `prefix` is `path` or lies below it.
    fn on_path(&self, prefix: u32, path: u32) -> bool {
        if prefix == NONE {
            return true;
        }
        let depth = self.paths[prefix as usize].depth;
```

`core-ordinary.md:927` records that "the matrix is the classical prover's"; it records no reason for a second copy of the search.

**Change.** One search generic over what the intuitionistic prover adds (the prefixes and their unification), instantiated twice, so that no dynamic call enters the loop.

**Value.** A reader, and every later change to the search (restricted backtracking, a new strategy), which is otherwise made twice.

**Net.** `connection::Statistics` must be equal before and after on the ILTP rows the comparison runs use, and the verdicts with them: by P13 a change that moves a counter is a search change. The ratchet has no journey through this prover, so the leader names the rows.

## F-arch-25

- dimension: architecture
- criterion: E6
- place: core/src/export/latex.rs:480 and core/src/export/typst.rs:550
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The LaTeX and the Typst writer are the same code over two spellings: six duplicates between the two files, the longest 90 lines.

**Witness.** `duplicates.md`: `latex.rs:480` ↔ `typst.rs:550` (90 lines), `:22` ↔ `:22` (75), `:265` ↔ `:305` (46), `:671` ↔ `:751`, `:596` ↔ `:669`, `:614` ↔ `:686`. Both files hold (`latex.rs:483`, `typst.rs:553`):

```rust
/// The bytes a sequent's text is estimated at per occurrence, besides
/// its longest atom name: at least what any connective, its brackets and
/// its separator take.
const PER_OCCURRENCE: u64 = 32;
```

and the same dispatch over `Drawable` (`latex.rs:677-684`).

**Change.** One module for a text format over a table of its spellings (the document's frame, a tree node, a Fitch line, a model's table), the two formats two tables; `sequent`, the model's table, `write` and the `Drawable` dispatch written once.

**Value.** A reader; a new text format is a table.

**Net.** The export test's snapshots (`core/tests/export.rs`) and the `export` check, which compiles the output: byte for byte the same. The ratchet's `render-latex` and `render-typst`.

## F-arch-26

- dimension: api
- criterion: X5
- place: core/src/export/mod.rs:235; core/src/linear/proofs/style.rs:231
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The exports take a closed list of the crate's own derivations, so a derivation from outside is drawn only by rewriting it as a `rules::Derivation`, which the docs do not say.

**Witness.**

```rust
// core/src/export/mod.rs:233
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum Drawable<'a> {
    /// A derivation of linear logic.
    Linear(&'a crate::linear::Derivation<'a>),
```

Every arm but the Fitch one calls the same generic `tree(x, options, judgement, out, stop)` over the crate-private trait `Drawn` (`export/latex.rs:677-684`).

**Change.** Say on `Drawable` that a derivation of another calculus is exported through `rules::Derivation` and `rules::Drawing`. After F-arch-06 and F-arch-07, the author may publish the trait as `export::Draw` and add `Drawable::Other(&'a dyn Draw)`.

**Value.** A researcher's proof object in a calculus of their own is drawn by the crate's layouts, and they know by which way.

**Net.** The sentence costs nothing. The trait's methods are generic over the writer today (`write_sequent(&self, out: &mut impl Write, …)`, `style.rs:266`), so an object-safe form needs `&mut dyn Write`: by P4 that is measured on the ratchet's three `render-` journeys before it is taken.

## F-arch-27

- dimension: text
- criterion: T3
- place: core/src/lib.rs:18; core/src/lib.rs:111
- kind: text
- cost: small
- candidate: no

**Claim.** The crate docs call intuitionistic logic "the default" and give four of their worked sections to the linear suite, and say nowhere where the author of an engine starts.

**Witness.**

```rust
// core/src/lib.rs:18
//! A sequent of ordinary logic, here intuitionistic logic, the default:
```

The sections "A proof, its fragment and its JSON" (`:111`), "Intuitionistic input" (`:143`), "Limits and stops" (`:167`) and "The command's answers" (`:198`) each show the linear suite alone; the modal suite and lambda-terms have no example, and "Errors" (`:253`) names the linear suite's errors alone. `ordinary::Logic` has `#[default] Intuitionistic` (`ordinary/mod.rs:171`), which D1 removes for the command.

**Change.** One short example per suite in the page's order, the limits and the stop shown once on a call of any suite, and a section "Write your own engine" that names `Forest`, `Proof::new`, `Proof::check` and the interface of F-arch-03 to F-arch-05, linking the manual's page (X8). Drop "the default" with D1's change to `Logic`.

**Value.** A reader of docs.rs who came for ordinary or modal logic finds their suite at the same depth as the others; a researcher finds the first call.

**Net.** The doctests (`cargo test --doc -p deduxit`); no code changes but D1's.
