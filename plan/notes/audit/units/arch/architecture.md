# The core crate as one design (unit arch)

Read at the audit's commit: every module's root and module doc, `lib.rs`,
`limits.rs`, `settings.rs`, `kripke.rs`, `goal_tree.rs`,
`protocol/mod.rs`, the front doors (`linear/search/mod.rs` in its parts,
`ordinary/decide.rs`, `modal/decide.rs`), the inventories, `core.md`,
`core-protocol.md`, parts of `core-search.md`, `core-modal.md`,
`core-ordinary.md` and `web.md`, `consumer-test.md`, and the earlier
analysis of the same question, `plan/notes/kernel-boundary.md` (step 67,
2026-11-16). Paths are under `core/src/` unless they say otherwise.
Findings are in `findings.md`; `F-arch-NN` below points at them.

## 1. The design as it is

**One crate, 207 688 lines** (`baselines.md`), in three kinds of module.

- **The suites**, beside each other: `linear` (90 122 lines: `sequents`,
  `parse`, `fragment`, `occurrences`, `proofs`, `search`, `nets`, the
  readers `lltp` and `mist`, `families`), `ordinary` (55 577), `modal`
  (6 427), `lambda` (9 670), and `rules` (7 905), the calculi given as
  rule files, "which belongs to no suite" (`core.md`, Layout).
- **What they share**, at the root: `errors` (one `Error`), `limits`
  (`Limits`, `Progress`, `Phase`, `Refusal`), `settings`, `wire`,
  `kripke` (the model as data), `hash`, and crate-private `first_order`
  (terms and symbols), `name`, `unify`, `goal_tree`.
- **What stands above every suite**, also at the root: `export` (six
  formats over `Drawable`, an enum of the suites' derivations),
  `serialize` (private, a file per suite), `protocol` (each suite's kinds
  and operations as data, run over JSON) and `workspace` (values linked
  by those operations).

**Who names whom** (`deps.md`, test files apart). The intended order is
shared ← suites ← export, serialize, protocol ← workspace. The code
departs from it in four ways:

1. **`linear` is the shared layer's second half.** `ordinary` names 62
   paths of `linear`, `lambda` 19, `rules` 8, `modal` 4; the root's own
   `goal_tree`, `unify`, `settings`, `export::notation` and `errors`
   name it too. Part of that is the embedding (ordinary logic is decided
   through a linear image, a lambda-term is read off a linear proof),
   which is the suites' business. The rest is shared machinery that
   lives in `linear` because linear logic came first:
   - the vocabulary of every front door: `search::Reason`
     (`linear/search/mod.rs:3481`) and four fields of `search::Options`
     (`:3016-3037`);
   - the drawing of any derivation: the trait `Drawn`
     (`linear/proofs/style.rs:231`), `InfId`, `Labels`, `OpenGoal`,
     `TextOptions`, `Structural`, the printer `Notation`
     (`linear/sequents/notation.rs`) and the walk `Stops`
     (`linear/sequents/fmt.rs`);
   - the memory account (`linear/search/memory.rs:21`), the session's
     `GoalId`, the first-order terms' public path
     (`linear::sequents::fo`), the clause syntax of TPTP
     (`linear::lltp::{Clauses, Status}`).
2. **`linear` names `ordinary`**: the shared printer is in the linear
   suite and prints ordinary formulas
   (`linear/sequents/notation.rs:20`).
3. **`ordinary` and `rules` name each other**: `rules` holds natural
   deduction, the tableau and LK and LJ as calculi given as data, and a
   session's search through `ordinary::decide` (2 213 lines:
   `rules/{ordinary,natural,tableau,search}.rs`); `ordinary` writes a
   reviewed tableau as a `rules::Derivation`
   (`ordinary/written/mod.rs:32`).
4. **The sums.** `Error` (about 75 variants, 36 paths of `linear`, 11 of
   `ordinary`), `Settings` (six of its eleven fields are the linear
   suite's options), `wire::LEVEL`, `Drawable`, `Notation` and the
   protocol's `SUITES` name every suite by construction. They are the
   reason the crate is one crate (`kernel-boundary.md`, 4.3).

**The front doors** have one shape by the author's decision
(`core-ordinary.md`, "Deciding"): `prove`/`prove_within` (linear),
`decide`/`decide_within` (ordinary, modal), each returning an `Outcome`
with a `Verdict`, every long call taking `&Limits` and a stop
`FnMut(Progress) -> bool`. There are three `Outcome`s, three `Verdict`s
and three `Statistics`, one per suite, and one `Reason`.

**The checkers** are four, sharing no code: the linear pass
(`linear/proofs/check/`), `ordinary::Derivation::check`,
`rules::Derivation::check` (which the modal suite's derivations go
through), and the model checks. A proof term is public and can be built
outside the crate in the linear suite (`Proof::new`) and in `rules`
(`Derivation::new`, `push`); in the ordinary suite and for lambda-terms
it cannot (F-arch-05).

**The engines** of the linear suite implement one crate-private trait,
`Decide` (`linear/search/mod.rs:1477`), behind a closed enum `Engine`
and a dispatch table; `conclude` is the one place an engine's answer
becomes a `Verdict`. The ordinary suite's eight methods are arms of one
`match` (`ordinary/decide.rs:821`), each a function of its own
signature.

**The sessions** are three types named `Interactive` (linear, ordinary,
`rules`), each with `goals`, `candidates`, `apply`, `undo`, `close`,
`hint`, a derivation and a JSON form, and no interface in common. The
library's `protocol` does not cover a session's steps; the bindings do,
in a table of their own (`web/src/protocol.rs:151`) over three sets of
exports written by hand.

**The features**: nine on by default (`parse`, `serialize`,
`interactive`, `latex`, `typst`, `svg`, `rocq`, `sat`, `protocol`) and
three off (`parallel`, `png`, `pdf`). Every engine of every suite is
compiled under every feature set.

## 2. Module boundaries

**Modules that hold more than one thing.**

- `linear/search/mod.rs` (5 552 lines) is the front door, the engine
  interface, the dispatch table, the catalogue of engines with their
  documentation, every options enum, `Options`, `Outcome`, `Verdict`,
  `Reason` and `Statistics`. Its seams follow its own section order:
  `search/options.rs` (`Bias`, `Jobs`, `Cadence`, `Schedule`, `Options`,
  lines 2400 to 3389), `search/outcome.rs` (`Outcome`, `Verdict`,
  `Statistics`, 3389 to 3846), `search/engine.rs` (`Engine`, `Counter`,
  `NotTaken`, `Task`, `Answer`, `Decide`, the dispatch, 1362 to 2400),
  and `mod.rs` keeps `prove`, `prove_goal`, `race` and `conclude`
  (F-arch-19).
- `ordinary/mod.rs` (2 379) is the decision's vocabulary (`Logic`,
  `Translation`, `Method`, `Options`, lines 162 to 707) and the formula
  arena with its printer and sequent (709 to 1770). Seams:
  `ordinary/formulas.rs` and `ordinary/options.rs` (F-arch-20).
- `ordinary/interactive.rs` (5 258) is one file where the linear
  session is nine (`linear/proofs/interactive/`): its seams are the
  linear session's (`error`, `step`, `expand`, `witness`, `close`,
  `replay`, `derivation`) (F-arch-20).
- `rules` holds the generic checker and the ordinary suite's natural
  deduction (F-arch-09).
- `kripke::Model` is data at the root whose methods are written in two
  suites (`ordinary/model.rs:175`, `modal/model.rs:176`); the modal check
  is bounded by `ordinary::MODEL_CELLS` (F-arch-21).

**One thing spread over several modules.**

- Drawing a derivation: `export/` (the formats), `linear/proofs/style.rs`
  (the trait, the labels), `linear/proofs/fmt.rs` (the text tree),
  `linear/sequents/notation.rs` (the printer of every suite's formulas),
  `linear/sequents/fmt.rs` (the walk) (F-arch-06, F-arch-07).
- A front door's answer: `Reason` in `linear::search`, `Refusal` in
  `limits`, the map from one to the other in `ordinary/decide.rs:354`
  (`ended`) and again in each protocol suite (F-arch-01).
- The registry of suites: `protocol::SUITES`, `OPERATIONS`, `KINDS`, the
  `match` of `protocol::statement`, and the bindings' `SUITES`
  (F-arch-11, F-arch-12).
- Countermodels of ordinary logic: `ordinary/{model, models,
  kripke_search, kripke_structure, structure, rocq_models}.rs`, six
  files at the suite's top level with near names (`model.rs` checks a
  Kripke model, `models.rs` searches finite models, `export/models.rs`
  lays one out). A directory `ordinary/models/` would hold them; this is
  unit 4d's to judge.

**Files too large to hold in mind**: the six over 3 000 lines
(`baselines.md`). Beside the three above: `ordinary/normal.rs` (3 688),
`ordinary/derivation.rs` (2 919: the rules, the checker, the read-back
from a linear proof at line 1324, the drawing), which the units' finders
read.

## 3. Layering: what the suites share and what each owns

For each group of what one suite names of another:

| group | witness | verdict |
|---|---|---|
| `linear::search::Reason` in `ordinary::Verdict::Unknown`, `modal::Verdict::Unknown`, `lambda::Inhabitation::Unknown`, `workspace` | `public-api.txt:5260`, `:4396`, `:474`; three of its thirteen variants are ordinary logic's alone (`PathLength`, `ModelSize`, `KripkeSize`, `linear/search/mod.rs:3559-3581`) | **move to the root**, beside `Refusal`, as one enum: `deduxit::limits::Reason`, re-exported as `deduxit::Reason`. Its JSON stays (F-arch-01) |
| `linear::search::Options` as the second options value of `ordinary::decide_within`, with `path_length`, `restricted_backtracking`, `model_size`, `model_worlds` | `ordinary/decide.rs:751`, `linear/search/mod.rs:3016-3037` ("Only that prover reads it") | **split**: the four fields move to `ordinary::Options`; `decide_within` keeps taking the linear options for the embedding it runs (F-arch-02) |
| `linear::search::{prove_within, prove_goal, Goal, Outcome, Verdict, Disproof, Refutation, Classical}`, `linear::{Sequent, Forest, Proof, Mode, Fragment, Member, OccId, TermId, Atom}`, `occurrences::{Reading, Side, IllConnective}` | `ordinary/translate.rs`, `ordinary/derivation.rs:1324`, `lambda/linear.rs` | **stay**: this is the embedding and the read-back, a suite using another through its public door |
| `linear::search::memory::Account` in LJT, the SAT engine, resolution and the modal tableau | `ordinary/ljt.rs`, `ordinary/sat/*.rs`, `ordinary/resolution/outside.rs`, `modal/search.rs` | **move to the root** (crate-private, no path changes); recorded as due since 2026-11-16 (F-arch-06) |
| `linear::sequents::{Stop, Stops}`, `notation::{Notation, TEXT, Atoms}` | `ordinary/mod.rs:153-154`, `modal/mod.rs:71`, `rules/print.rs:10`; `notation.rs:20` names `ordinary` back | **move to the root** as a module `notation` (crate-private); the cycle goes with it (F-arch-06) |
| `linear::proofs::style::Drawn` implemented by `ordinary`, `lambda`, `rules` | `ordinary/derivation.rs:1149`, `lambda/draw.rs:597`, `rules/draw.rs:276` | **move to `export`** beside its walker (F-arch-06); public later if outside derivations are to be drawn (F-arch-26) |
| `linear::proofs::{InfId, GoalId, Labels, OpenGoal, TextOptions, Structural, interactive::Inverted}` in the public signatures of `ordinary`, `export`, `Settings` | `public-api.txt:17`, `:395`, `:5282-5290`, `:5369`, `:5384` | **move to the root** (public: a break), the linear suite re-exporting none of them (F-arch-07) |
| `linear::sequents::fo::{FoTerm, FoTermId, Symbol, Args, Eigen}`, defined at the root, public only under `linear` | `core.md` Layout; `public-api.txt:5492` (`ordinary::Step::witness`) | **publish at the root** as `deduxit::terms` (F-arch-07) |
| `linear::lltp::{Clauses, Status, clauses}` in the TPTP reader | `ordinary/parse.rs:15`; `ordinary::Problem::status: Option<linear::lltp::Status>` | **move to the root** as the private `tptp` clause reader; `Status` public at the root (F-arch-07) |
| `linear::search::generate::Rng` in twelve ordinary test modules | `ordinary/models.rs:1034` and others | test-only; a `#[cfg(test)]` root module when the others move. No finding |
| `ordinary::MODEL_CELLS` in `modal` | `modal/model.rs:18` | **move to `kripke`** (F-arch-21) |
| `rules` ↔ `ordinary` | above | **the ordinary part of `rules` moves to `ordinary`** (F-arch-09) |
| `modal` → `rules` | `modal/decide.rs:57` | **stay**: a suite answering with the generic derivation, the model for a new logic |
| `lambda` → `ordinary`, `linear` | `lambda/simple.rs`, `lambda/linear.rs`, `lambda/inhabit.rs` | **stay**: read-backs and the search by LJT |

After these moves the graph has no cycle: shared root ← `linear`; root ←
`rules`; root, `linear`, `rules` ← `ordinary`; root, `rules` ← `modal`;
root, `linear`, `ordinary` ← `lambda`; all ← `export`, `serialize`,
`settings`, `protocol` ← `workspace`. `errors` stays the one module that
names every suite, by the decision "one error family".

`kernel-boundary.md` listed the crate-private moves and left the public
ones "until a split re-exports them", so that no public path changed.
D2 and D4 are later than that note and remove its ground: a shared
notion that a user must write as `deduxit::linear::…` to use ordinary or
modal logic is what D4 rules out.

## 4. Duplication a trait or a generic would remove, and the opposite

**Worth one copy.**

- **The propositional arena, its sequent, parser and JSON form**, twice
  (`ordinary`, `modal`): `modal/mod.rs:170` ↔ `ordinary/mod.rs:824` and
  four more pairs, six pairs in the parsers, six in `serialize/`
  (`duplicates.md`). `core-modal.md:263` records the copy "for the
  kernel crate" and `later.md` its trigger, step 74, which has landed.
  Shared form: a crate-private `Arena<N: Shape>` at the root (nodes,
  the hash-consing map, the foreign-id check, the `u32` refusal), with
  `trait Shape: Copy + Eq + Hash { fn operands(self) -> (Option<Id>,
  Option<Id>); }`; the two `Formulas` stay public types holding one
  each (F-arch-08).
- **The two connection provers**: thirteen pairs between
  `ordinary/connection/prove.rs` and `…/intuitionistic/prove.rs`, up to
  115 lines (`prove.rs:650` ↔ `intuitionistic/prove.rs:705`: `on_path`,
  `try_literal` word for word). Shared form: one search generic over
  what the intuitionistic prover adds (the prefixes and their
  unification), monomorphised, so no dispatch enters the loop
  (F-arch-24).
- **The LaTeX and the Typst writer**: six pairs, 90 lines the longest
  (`export/latex.rs:480` ↔ `export/typst.rs:550`): the same writer over
  two tables of spellings (F-arch-25).
- **The protocol's suites**: thirteen rows of `duplicates.md` between
  `protocol/{linear, ordinary, modal, lambda, rules}.rs` (the draw,
  export and duplicate operations, the limit of a reason). Shared form:
  functions in `protocol/shared.rs` (F-arch-11).
- **The sessions' drivers in the front ends**: `cli/src/interact/mod.rs`
  ↔ `interact/ordinary.rs` (five pairs), `web/src/calls.rs:1709` ↔
  `web/src/ordinary.rs:1324` ↔ `web/src/rules.rs:818`. The copies exist
  because the three sessions have no interface in common (F-arch-12).

**An abstraction that would cost more than it saves.**

- **One generic goal state** for the three sessions. They differ in what
  a step records, what `undo` restores and what closes a goal
  (`kernel-boundary.md`, 6.5); the shared walks are already free
  functions (`goal_tree.rs`). The interface worth having is over JSON,
  at the protocol (section 7), not a generic type.
- **One `Verdict` type for every suite.** The payloads differ in every
  variant (a `Proof`, a derivation or a clausal proof, a
  `rules::Derivation`); `protocol::Verdict` is already the word they
  share. What should agree is the shape: where the counter-evidence
  lives (F-arch-13).
- **A trait over the four checkers.** Each checks another object by
  another pass; nothing is shared but the meter, which is a struct
  (`rules/check.rs:29` and `modal/model.rs:376` hold it twice, recorded
  in `later.md`).
- **`ProofStructure` and `MellStructure` as one type.** They share
  graph code (`nets/graph.rs:309` ↔ `nets/mell/graph.rs:523`), which
  unit 3 judges; the two structures differ in what a vertex is.

## 5. The engine interface (X4)

What a researcher needs, and what the crate gives today:

| need | linear | ordinary | modal, other logics |
|---|---|---|---|
| build a sequent without text | no (`Sequent::from_parts` is `pub(crate)` and behind `parse`, `sequents/mod.rs:415`) | yes (`Formulas::add`, `atom`, `Sequent::new`) | yes (the same in `modal`; `rules::Terms`) |
| read a formula | yes (`Sequent::term`, `Forest`) | yes (`Formulas::node`) | yes |
| build a proof | yes (`Proof::new`) | no (`Derivation::from_parts` is `pub(crate)`, `ordinary/derivation.rs:605`) | yes (`rules::Derivation::new`, `push`) |
| have it checked | yes (`Proof::check`) | yes, once built | yes (`Derivation::check(&calculus, …)`) |
| build a refutation | yes (`Disproof::new`, unchecked for `Exhausted`) | yes (`kripke::Model`, `Countermodel::new`, `check`) | yes |
| poll the stop, refuse at a limit | no (`Progress::new` is `pub(crate)`, `limits.rs:429`; `Refusal` has no constructor) | no | no |
| answer with an `Outcome` | no (`#[non_exhaustive]`, no constructor) | no | no |
| be run by the front door, the batch, the command | no (`Decide` is crate-private, `Engine` a closed list) | no | no |
| draw and export | yes (`Proof::derivation`) | yes, once built | yes (`rules::Drawing`) |

**The smallest interface that closes the gaps**, in three parts.

*Part 1: the shared machinery (X2), in `limits`.*

```rust
impl Progress {
    /// The progress of a call in `phase`: `work` units since the last
    /// poll, `done` since the call began.
    pub const fn new(phase: Phase, work: u64, done: u64) -> Self;
    pub const fn with_held_bytes(self, held_bytes: u64) -> Self;
}

impl Refusal {
    pub const fn stopped(phase: Phase) -> Self;
    pub const fn work(limit: u64) -> Self;
    pub const fn memory(phase: Phase, limit_bytes: u64, needed_bytes: Option<u64>) -> Self;
}

/// Counts a call's work, asks its stop every `period` units and refuses
/// at `limits.work`.
pub struct Meter<'a> { /* private */ }

impl<'a> Meter<'a> {
    pub fn new(limits: &Limits, phase: Phase, stop: &'a mut dyn FnMut(Progress) -> bool) -> Self;
    /// `Err(Refusal::Stopped)` or `Err(Refusal::Work)`.
    pub fn tick(&mut self, units: u64) -> Result<(), Refusal>;
    pub const fn done(&self) -> u64;
}
```

`Progress::new` exists and becomes public. `Meter` is the struct that
`rules/check.rs:29` and `modal/model.rs:376` each hold; one copy serves
them and an outside engine. Cost: one file, and the two internal meters
replaced.

*Part 2: the values (X1, X3).*

```rust
// linear::sequents
impl Sequent {
    /// Interns an atom's name. `Error::AtomName` for a name the text
    /// syntax would not read back.
    pub fn atom(&mut self, name: &str) -> Result<Atom, Error>;
    /// Adds a term whose operands the arena holds
    /// (`Error::IndexOutOfBounds` otherwise, `Refusal::Index` when full).
    pub fn push(&mut self, term: Term) -> Result<TermId, Error>;
    /// Makes `term` the next formula of the sequent.
    pub fn push_root(&mut self, term: TermId) -> Result<(), Error>;
}

// ordinary
impl Inference {
    pub fn new(left: Vec<NodeId>, right: Vec<NodeId>, rule: Rule) -> Self;
    #[must_use] pub fn with_principal(self, side: Side, position: usize) -> Self;
    #[must_use] pub fn with_premises(self, premises: &[InfId]) -> Self;
    #[must_use] pub fn with_term(self, term: FoTermId) -> Self;
}

impl Derivation {
    /// A derivation of these inferences, premises before conclusions,
    /// the root last. Checks the shape; `check` judges the rules.
    pub fn new(logic: Logic, formulas: Formulas, left: Vec<NodeId>,
               right: Vec<NodeId>, inferences: Vec<Inference>) -> Result<Self, Error>;
}
```

`Sequent::check` (`sequents/mod.rs:586`) and the JSON reader of an
ordinary derivation already hold the validation; the constructors call
it. `Derivation::new` marks the value unchecked (`Trusted::new(false)`),
as a derivation read from JSON is. Cost: two files.

*Part 3: the answer and the run (X4).*

```rust
// linear::search
/// A search an outside crate brings.
pub trait Decide {
    /// Its name, as an outcome and the statistics write it.
    fn name(&self) -> &'static str;
    /// Refuses a goal it does not decide (`Error::EngineRefused`).
    fn admits(&self, goal: Goal<'_>, mode: Mode) -> Result<(), Error>;
    fn decide(&self, goal: Goal<'_>, mode: Mode, limits: &Limits,
              stop: &mut dyn FnMut(Progress) -> bool) -> Result<Found, Error>;
}

#[non_exhaustive]
pub enum Found {
    /// A proof of the goal, which `prove_with` checks.
    Proved(Proof),
    /// No proof exists: the search was exhaustive, or here is why.
    Unprovable(Option<Refutation>),
    /// The search gave up.
    Unknown(Reason),
}

/// Decides `goal` by `engine` and answers as `prove_goal` does: the proof
/// checked against the goal unless `options.check` is off, the
/// refutation checked or the classical refuter run, the outcome's
/// `engine` `Engine::External`.
pub fn prove_with(engine: &dyn Decide, goal: Goal<'_>, mode: Mode, options: &Options,
                  limits: &Limits, stop: impl FnMut(Progress) -> bool) -> Result<Outcome, Error>;
```

and for the ordinary suite, where an engine's evidence is a derivation
or a model:

```rust
impl Outcome {
    /// `Valid` with `derivation` once `Derivation::check` accepts it.
    pub fn of_derivation(derivation: Derivation, limits: &Limits,
                         stop: impl FnMut(Progress) -> bool) -> Result<Self, Error>;
    /// `NotValid` with `countermodel` once its check accepts it.
    pub fn of_countermodel(countermodel: Countermodel, logic: Logic, limits: &Limits,
                           stop: impl FnMut(Progress) -> bool) -> Result<Self, Error>;
    pub fn unknown(logic: Logic, reason: Reason) -> Self;
}
```

**What it costs the present code.** The crate-private trait keeps its
signature (it takes a `Task`, the `memory::Account`, the shared `Work`
count and a stop over `u64`, which the built-in engines need and an
outside one should not see) under another name; an adapter of about 40
lines implements it for `&dyn Decide`, turning the `Task` into a `Goal`
and the unit stop into a `Progress` stop. `conclude` stays the one place
an answer becomes a verdict, so the checker stays the trust boundary: no
path makes an `Outcome` that says `checked` without the check.
`Engine` gains a last variant `External`, and `Search` (ordinary) one of
the same name; both enums are `#[non_exhaustive]`. The dispatch never
picks an outside engine. No loop of a built-in engine changes, so the
target sets' counters are the proof that nothing moved (F-arch-03,
F-arch-04, F-arch-05).

**For a logic the crate does not have**, the interface exists: a
`rules::Calculus` (a rule file) and a `rules::Derivation`, checked by
`Derivation::check` and drawn by `rules::Drawing`. The modal suite is
the worked example, and the manual page X8 asks for should show both
ways: a new search for a calculus the crate checks natively, and a new
logic through a rule file.

**What the value promises** (consumer test, point 5). The ordinary
derivation carries its own flag (`trusted`, `ordinary/derivation.rs:510`)
and the linear proof does not (`Outcome::checked`,
`linear/search/mod.rs:3408`). One answer for both suites, said in the
docs of `Verdict::Proved` and `Evidence`, is enough for 0.9.0: "a value
in a verdict has passed the checker unless `checked` says otherwise; a
value you build or read has not until you call `check`" (F-arch-23).

## 6. The crate's shape

**The numbers** (`timings.md`, cores 13 to 15, three jobs): the core
crate alone in the dev profile takes 40 s with the default features and
16 s without them; in the workspace build its unit is 50.3 s of 87 s;
a rebuild after its root is touched is 11 s; in the release profile its
unit is 129 s and the command's link under fat LTO another 127 s. The
tests build in 120 s and run in 202 s.

**Would a split into crates serve users?** Not by itself, and not for
0.9.0.

- *Compile time.* A user compiles the crate once. Split into a shared
  crate, one per suite and the exports, cargo would build suites side by
  side, which shortens the 40 s on a machine with cores to spare and
  changes nothing in CPU time; a user of one suite would skip the others
  (the linear suite alone is 43 % of the lines). The same saving comes
  from a cargo feature per suite inside one crate, without a second
  name on crates.io. The 24 s that the default features add are serde's
  derives, the readers, the exports and the protocol, which a split
  does not remove.
- *The developer's loop.* 11 s is the best case today. A leaf crate
  would rebuild alone, but the command, the bindings and the harness
  depend on everything and relink either way.
- *The module's size.* None: one cdylib links what its exports reach,
  whatever the crate graph (section 6.1).
- *What it costs.* `kernel-boundary.md` 4.2 and 4.3 still hold: every
  crate `deduxit` depends on is published at each release, and `Error`,
  `Settings`, the wire level, `Drawable`, `Notation` and the registry
  are sums that must stay above the suites, so the shared crate needs an
  error of its own and every moved function changes its error type.

**What would serve users now** is the layering of section 3 inside the
crate. It is what a split would need first, it is what makes a feature
per suite possible, and each move stands on its own. After it, a feature
per suite (`linear`, `ordinary = ["linear"]`, `modal`, `lambda =
["ordinary"]`, `rules`) is a matter of `cfg` on five `pub mod` lines,
the sums' variants and fields, and `Drawable`; `Error` already carries
variants behind features. Whether it pays is a measurement: the share
of the 16 s that each suite's engines take (F-arch-17).

**What reopens the split**: as recorded (a suite from another
repository that must not link the linear suite; a decision to give a
suite an owner of its own). I add one: the engine interface of section
5 attracting outside engines that want the data model and the checkers
without 200 000 lines of engines. The feature of F-arch-17 answers that
too, so the split stays the second choice.

### 6.1 The web module, and suites as modules of their own

The module is 2 987 284 bytes at gzip, 549 168 more than on 2026-11-21
when the renderers left it (`web.md`, "The budget"). D29 keeps one
module "until a measured size … asks for a module per suite loaded
lazily"; the size is measured, so the question is open by the plan's own
rule.

**What makes it possible already.** Nothing is kept between calls
(`web.md`): every call is JSON in, JSON out, so a module boundary costs
no protocol. The renderers show the build: a cdylib of its own with its
own features of the library, built by a cargo of its own.

**What stands in the way.**

1. *The workspace links every suite.* `workspace` calls `protocol::run`
   on the static registry (`workspace/mod.rs:1062`, `:1084`, `:1743`),
   and the registry holds every suite (`protocol/mod.rs:418`). Any module
   that exports a `workspace_*` call is the whole library. A module per
   suite needs the registry to be a value: `Workspace::compute` and
   `run` taking `&Registry`, the page's worker answering a run of another
   suite by calling that suite's module (the workspace already separates
   `compute` from `accept`, which is the seam). F-arch-10 asks for the
   same value for another reason.
2. *The suites contain each other.* A module for ordinary logic holds
   the linear search (the embedding), one for lambda-terms holds both,
   one for modal logic holds `rules`. Modules side by side would each
   carry the shared code again (the readers, serde_json, `Error`'s
   messages, the formatting of `std`). So a visitor of two suites could
   download more than today.
3. *The sums.* `Settings` is read whole by every call
   (`deny_unknown_fields`), and its serde code names every options type.

**The options, and what each needs.**

| option | what it needs | what it may give |
|---|---|---|
| A. Measure first: `twiggy top` and `twiggy dominators` on the unstripped module, bytes by the library's module path | a build with names kept; no change of code | the facts every other option waits for: what each suite, the exports, serde and the protocol weigh |
| B. One module, built for size: `wasm-opt -Oz` or `opt-level = "s"` for the bindings (P10 lists it as open), against the journeys' time in the module | two builds and a timing in Node | bytes without a change of design |
| C. A late module for what a visitor rarely opens first, cut where the dependency goes one way: the exports to LaTeX, Typst and Rocq and the certificates (a derivation's JSON in, text out), as the renderers are | the readers and the derivation views in both modules; a second loader in the client | the exporters' bytes off the first load, less the shared code carried twice |
| D. A module per suite | the registry as a value (1), a feature per suite in the bindings, the client loading by suite, the shared code in each | a visitor of one suite loads one suite; of several, more than today unless the shared part is small |

A before any other; B is the cheap one; C follows the renderers'
precedent; D is the largest and the only one that needs the library to
change. I could not judge the bytes: `llvm-lines.txt` and
`type-sizes.txt` were not made when I read, and no inventory attributes
the module's bytes (F-arch-18).

## 7. The gaps the last steps' reports name

**`rules_candidates`.** Step 81 gave the three sessions one call,
`candidates`, and one JSON form for its answer; the bindings export it
three times (`candidates`, `ordinary_candidates`, `rules_candidates`,
`web/src/lib.rs:109`, `:308`, `:557`), each over a function of its own
(`web/src/calls.rs:715`, `web/src/ordinary.rs:270`,
`web/src/rules.rs:422`), and the form `{"rule", "at", "status", "needs",
"refusal"}` is the bindings' (`web/src/calls.rs:642`), not the
library's. The same holds for `start`, `view`, `rules`, `apply`, `undo`,
`close` and `hint`: twelve calls a suite, listed by hand in
`web/src/protocol.rs:151-476` with `offered(…)` or `lacks(…)`, beside
the library's `protocol`, which lists operations and knows nothing of a
session's steps. So there are two protocols, and the second is where a
suite's session is described.

**The protocol's `KINDS`** (`protocol/mod.rs:505`) lists no kind of the
lambda suite, which has five (`protocol/lambda.rs:29`). It is a second
list of what the registry already holds, as `OPERATIONS` (`:432`) is,
kept true by a test for one of the two.

**A consistent design.**

1. *One registry, written once.* `SUITES` in the page's order (ordinary,
   linear, modal, lambda, then `rules`). `OPERATIONS` and `KINDS` are
   read from it: the `known` list of `Error::UnknownName` is built once
   from `suites()` (a `LazyLock<Vec<&'static str>>`, lent as
   `&'static [&'static str]`). `statement` becomes a method of `Suite`
   (`fn statement(&self, text: &str, parameter: &str, limits: &Limits)
   -> Option<Result<String, Error>>`, `None` for a suite without typed
   statements), so `protocol::statement` is a lookup and its `match`
   goes.
2. *The session as part of the protocol.* The library gives the
   session's calls over JSON, per suite, behind one trait:

   ```rust
   /// A suite's step-by-step proving, over JSON documents.
   pub trait Session: Sync {
       fn start(&self, statement: &str, settings: &Settings) -> Result<String, Error>;
       fn view(&self, session: &str, settings: &Settings) -> Result<String, Error>;
       /// Every rule at `at` of `goal`, each applying, needing input or
       /// refused with the error `apply` gives: the one form of step 81.
       fn candidates(&self, session: &str, goal: u32, at: &str, settings: &Settings) -> Result<String, Error>;
       fn apply(&self, session: &str, goal: u32, step: &str, settings: &Settings) -> Result<String, Error>;
       fn undo(&self, session: &str, settings: &Settings) -> Result<String, Error>;
       fn close(&self, session: &str, goal: u32, settings: &Settings,
                stop: &mut dyn FnMut(Progress) -> bool) -> Result<String, Error>;
       fn hint(&self, session: &str, goal: u32, settings: &Settings,
               stop: &mut dyn FnMut(Progress) -> bool) -> Result<String, Error>;
   }

   pub trait Suite: Sync {
       // …
       /// Its sessions, `None` for a suite without step-by-step proving.
       fn session(&self) -> Option<&'static dyn Session> { None }
   }
   ```

   The step and candidate forms get a wire form in the library, which
   `web.md` already expects ("When the library gives them one … the
   bindings take it"). The bindings then export one set,
   `session_candidates(suite, session, goal, at, settings)`, and their
   table of twelve calls is the catalogue's `session` entry. The command
   reads the same trait for `interact`.
3. *A registry that can hold a suite from outside*, or a trait that
   says it is sealed: today `Suite`'s docs promise the first
   (`protocol/mod.rs:354`) and the types give the second (F-arch-10).

The three `Interactive` types keep their typed methods. Their names
should agree where they mean the same: `is_complete` (linear, ordinary)
against `is_finished` (`rules/interactive.rs:430`); `goals()` an
iterator against a `Vec` (`rules/interactive.rs:416`); `goal(id)` a
`Result` against a slice that panics; `undo` an `Option<GoalId>`
against `Option<Undone>` (F-arch-12).

## 8. The features

| feature | cuts along a line a user needs? |
|---|---|
| `parse`, `serialize`, `interactive` | yes: a capability across every suite, each with its dependency or its weight (404, 402 and 164 `cfg` sites) |
| `sat` | yes: it brings batsat and four methods |
| `parallel`, `png`, `pdf` | yes: heavy dependencies, and `parallel` cannot build for the web |
| `protocol` | yes, though it is two things (`protocol` and `workspace`) under one name; both need `serde_json`, so one feature is fair |
| `rocq` | yes: the certificates are another thing than a drawing, and the harness builds without it (`bench/Cargo.toml:23`) |
| `latex`, `typst`, `svg` | doubtful: three features with no dependency of their own, 258 `cfg` sites between them, 51 lines of the form `any(latex, typst, svg)`. Every consumer in the workspace enables all three or none (`cli`, `web`, `web-tab`, `bench`); together they gate less than 13 665 of 207 688 lines. One feature `export` (with `png` and `pdf` on top) would leave ten features and 45 pairs for `cargo hack` in place of 66 (F-arch-22) |
| `serialize` | it also turns on `rules` and the modal decision procedure (`lib.rs:350`, `modal/mod.rs:42-64`), which no one would guess from its name (F-arch-16) |

**Is there a feature that gives the data model and the checkers alone?**
No. `default-features = false` removes the readers and writers and
keeps every engine: the focused, Horn, inverse and net engines, LJT, the
connection provers, resolution, the normal forms, the lambda suite's
search. It also removes every way to make a linear sequent (F-arch-05),
so what it leaves cannot be used. What X7 asks for is the opposite cut:
a default feature `search` that gates the engines and the front doors,
so that `default-features = false, features = ["parse"]` is sequents,
forests, proofs, derivations, models and their checkers. The seam is
there in the linear suite once `search/mod.rs` is divided (the outcome
and refutation types and their checks on one side, `prove` and the
engines on the other) and in the ordinary suite at `decide` and the
method modules. Its worth is the share of the 16 s that the engines
take, to be measured (F-arch-17).

## 9. What I could not judge

- The module's bytes by suite, and whether `llvm-lines` shows generic
  bodies compiled per caller beyond those `core.md` records: the
  inventories were not there.
- The inside of the engines, the checkers and the exports: the finders
  read them. Where I name a duplicate inside a unit (the connection
  provers, the LaTeX and Typst writers), I opened both places and no
  more.
- First-order formulas of the ordinary suite built through the public
  API (predicates with arguments, terms): `Formulas` shows `add` and
  `atom` alone in the listing, and I did not follow how a term is made.
- The Rocq library and the command.
