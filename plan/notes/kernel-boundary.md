# The kernel crate: the inventory, the boundary and the gate's answer

Written 2026-11-16 by session 67a (`plan/67-kernel-crate.md`, its first
session) on `main` at `ytokpkzq`, with 66c's modal certificates in
flight. Paths are under `core/src/` unless they say otherwise; line
numbers are those of that commit.

## 1. The answer

**Not yet a crate.** The step ends at its gate, and the list is kept in
`plan/later.md` ("The kernel crate: ended at the gate"). Four findings
decide it, each shown below:

1. **The suites are modules of one crate, and nothing but that crate
   would depend on a kernel.** D25 and the structure note describe a
   kernel under suite crates. The suites were built as the modules
   `linear`, `ordinary`, `modal`, `rules` and `lambda` of `deduxit`, and
   no step up to 79 plans a suite in a crate of its own. The line
   between what every suite shares and the suites exists already: it is
   the crate's root (`core.md`, "Layout").
2. **A published crate cannot depend on an unpublished one**, so the
   prompt's two fixed points (the kernel "unpublished, a path crate at
   0.0.0" and "`deduxit` re-exports at the old paths") cannot both
   hold. A kernel that `deduxit` depends on is a third crate on
   crates.io at every release.
3. **What the root shares is mostly sums over the suites**: `Error`,
   `Settings`, the wire level, `export::Drawable`, the protocol's
   registry. A crate below the suites cannot hold them, and nearly
   every suite-free item returns `Error`.
4. **What at least two suites copied is about 1 050 lines of 156 447**
   (0.7 %), and a crate buys no build time, no speed and no size.

The boundary a later split would follow is section 5, its triggers are
section 7 and its sessions section 8. Section 6 lists what can become
one copy inside the crate without a crate, each with the step that
should take it.

## 2. What the code is today

- `core/` is the one library crate, 156 447 lines (`wc -l` over
  `core/src/**/*.rs`). `cli`, `web`, `bench` and `compat/linlog` depend
  on it (their `Cargo.toml`); it is the one library the other crates
  build on, but for the command's own (`deduxit_cli`).
- The root holds `errors`, `limits`, `settings`, `wire`, `kripke`,
  `hash`, `export`, `serialize`, `protocol` and `workspace`; the suites
  are modules beside each other. `core.md` ("Layout") already says: an
  item that a second suite comes to share without being linear logic
  moves to the root then, not before.
- The plan to step 79 adds no suite crate: 68 builds teaching tools, 70
  works on the ordinary arena, 71 gives the lambda suite a term model,
  74 to 79 take the ordinary suite to first order "on step 42's arena
  and unifier".

## 3. The inventory

Each row says what the records claim, what the code shows, and its
class: **copied** (two suites hold the same code), **shared** (one copy
that several suites use), **one suite** (a single user), or **sum** (a
type that names every suite).

### 3.1 The prompt's candidates

| candidate | the code | class |
|---|---|---|
| The error family | `Error` (`errors/mod.rs:53`) has 19 variants whose payload is a suite's type: `Check`, `Reduction`, `Rejected` (`linear::proofs::CheckError`), `Refutation`, `Net`, `Step`, `OrdinaryStep`, `NotIntuitionistic`, `Symbol`, `OrderedMode`, `OrderedFragment`, `NoEngine`, `FragmentMismatch`, `Translation`, `Logic`, `NegativeTarget`, `EngineRefused`, `Unsupported`, `NoTerm` (imports at `:15-21`). `kind`, `code` and `setting` match on them (`:848`, `:931`, `:1028`). `Owner` asks for a linear `Forest` (`errors/describe.rs:121`). `ParseError` (`errors/parse.rs`) and `ErrorKind` name no suite. | sum, but `ParseError` |
| `Limits`, `Progress`, `Phase`, `Space`, `Bound`, `Refusal`, `Held`, `WorkspaceLimits` | `limits.rs`, 844 lines. No suite type in code; four doc links name linear items (`:73`, `:88`, `:633-634`). Uses `errors::counted` and `serialize::exact`. | shared, suite-free |
| `Settings`, `Clock` | `Clock` (`settings.rs:37`) is suite-free. `Settings` (`:128`) has eight fields of suites' options, `search`, `view`, `batch`, `interactive`, `input`, `cut` (linear), `ordinary`, `lambda`, and `styles`, the exports' own sum (`export::Styles`). | `Clock` shared; `Settings` a sum |
| The wire policy | `LEVEL`, `Readable`, `Within`, `upgrade`, `fail`, `version` (`wire.rs:271-434`) use `Error` and `Limits` only. `level_of_sequent`, `level_in`, `level_of` (`:469-491`) take linear types. Every `Readable` impl is in `serialize/` beside its type, but the workspace's (`workspace/form.rs:321`). | machinery shared; the level table a sum |
| The checker's shape and its observers | Four checkers, none sharing code: the linear pass (`linear/proofs/check/pass.rs`, `Allowance` `:55`), `ordinary::Derivation::check` (`ordinary/derivation.rs:637-681`), `rules::Derivation::check` (`rules/check.rs`), the model checks (`ordinary/model.rs`, `modal/model.rs`). The checker of calculi given as data is the one a second suite reuses: the modal suite's derivation is a `rules::Derivation` (`modal/decide.rs:57`). | one suite each; `rules` shared |
| The checkers' meters | `Meter` in `rules/check.rs:29` (`tick` `:44`, `ask` `:59`, `POLL = 1 << 12` `:21`) and `Meter` in `modal/model.rs:376` (`tick` `:400`, `POLL` `:31`) are the same count, bound and poll, about 40 lines each. The memory pre-check and its `refused` closure are the same text in `ordinary/model.rs:263-282` and `modal/model.rs:245-263`. Ten more sites poll per unit inline. Every engine and checker has poll constants of its own: 28 by `grep 'const [A-Z_]*POLL'`, ten of them outside `linear/`. | copied, small |
| The export layouts behind a trait for a derivation tree | Done inside the crate: the crate-private trait `Drawn` (`linear/proofs/style.rs:231`), implemented for the linear `Derivation` (`:318`), `TwoSided` (`linear/proofs/derivation/classical.rs:823`), `ordinary::Derivation` (`ordinary/derivation.rs:970`) and `rules::Drawing` (`rules/draw.rs:276`); `latex::tree`, `typst::tree`, `svg::tree::draw`, the text tree and `export::notation::walk` are generic over it. Its signature names `Labels`, `Notation` and `InfId`, three linear paths. `Drawable` (`export/mod.rs:225`) is the public enum over the suites' derivations. | shared (the trait); `Drawable` a sum |
| Interactive proving's goals and undo | Linear and ordinary hold the same text, or the same algorithm under other names, for `subtree` (`linear/proofs/interactive/expand.rs:690-698`, `ordinary/interactive.rs:939-947`), `postorder` (`interactive/terms.rs:18-34`, `ordinary/interactive.rs:1326-1341`), the core of `undo` (`expand.rs:658-670`, `:924-935`), the graft's renumbering (`expand.rs:858-864`, `:1081-1087`) and the reader's history walk (`interactive/replay.rs:163-177`, `:1653-1667`). `subtree` and `postorder` are the same text and the core of `undo` nearly; the renumbering and the walk are the same algorithm with other names and checks. The cited ranges are about 60 lines a side, the functions that hold them about 230 of the ordinary state's 2 381 lines (the linear one has 7 540). `rules::Interactive` has other algorithms (no graft, scopes, lines; `undo` at `rules/interactive.rs:1305`). `GoalId` and `InfId` are the linear types, used by `ordinary` as they are. | copied (two of three), small |
| The front door's `Outcome`, `Verdict`, `Reason` | `Reason` (`linear/search/mod.rs:3384`) is used as it is by `ordinary::Verdict::Unknown` (`ordinary/decide.rs:151`), `modal::Verdict::Unknown` (`modal/decide.rs:47`), `ljt`, `sat`, `modal/search.rs` and `workspace/mod.rs:1535`; three of its nine variants are linear only (`CopyBound`, `WitnessDepth`, `Instances`). `Outcome`, `Verdict`, `Evidence` and `Statistics` are one per suite, alike in shape and different in every payload. `protocol::Verdict` (`protocol/mod.rs:287`) is the suite-free one. | `Reason` shared; the rest one suite each |
| The parallel runtime | `linear/search/parallel.rs` and `work.rs` have no user outside `linear/`. | one suite: fails the gate |

### 3.2 What the code shows besides

| item | the code | class |
|---|---|---|
| The propositional arena and sequent | `modal/mod.rs:71-480` against `ordinary/mod.rs:539-956`: `NodeId`, `Formulas` (`:141`, `:593`), `add` (`:206`, `:664`), `holds`, `atom`, `formula`, `Sequent` (`:386`, `:877`), `occurrences` and `bound` are identical up to the name; `Node` differs by `Box` and `Diamond`, the printer by its symbols. About 300 modal lines are identical after renaming (`diff` of the two files, 109 of 410 differ). | copied |
| The parser | `modal/parse.rs` against `ordinary/parse.rs`: `Parser`, `Pending`, `formula` (`:178`, `:157`), `reduce` (`:281`, `:291`), `wait`, `within`, `add`, `count_within` (`:378`, `:397`), `FromStr` and `parse_within`. About 296 of 387 modal lines identical. The ordinary one adds the TPTP dialect, the modal one a prefix table. The linear parser (`linear/parse/mod.rs:553-833`) is the same algorithm over characters with binders and terms; `rules/text.rs` is a table-driven shunting yard over tokens: neither shares code. | copied (two) |
| The sequent's JSON form | `serialize/modal.rs:33-236` against `serialize/ordinary.rs:35-231`: `FormulaForm` (`:37`, `:39`), `SequentForm`, `within`. About 143 of 204 modal lines identical. | copied |
| The memory account | `linear/search/memory.rs` (`Account` `:21`, 309 lines, no `use crate::`), used by `ordinary/ljt.rs`, five files of `ordinary/sat/` and `modal/search.rs`. | shared, under `linear` |
| Names and the walk | `linear/sequents/name.rs` (149 lines), used by `ordinary`, `modal`, `rules/terms.rs`, `serialize/kripke.rs`; `linear/sequents/fmt.rs` (`Stop`, `Stops`, 264 lines), used by `ordinary`, `modal`, `ordinary/rocq.rs`. | shared, under `linear` |
| The notation table | `Notation` (`linear/sequents/notation.rs:28`) carries the linear symbols, `ordinary: Symbols` (`:89`) and `necessary`/`possible` (`:81`, `:87`); its methods name `Forest`, `Member`, `Reading`. `lambda/print.rs:44` has a table of its own. | a sum |
| The Kripke model and its drawing | `kripke.rs` (125 lines) and `export/svg/kripke.rs` (594) name no suite; `ordinary` and `modal` each wrap a `Countermodel` around the model. | shared |
| The renderers | `export/svg/font.rs` (529), `export/png.rs` (225), `export/pdf.rs` (344) and the render costs in `export/mod.rs` name no suite. | shared |
| The protocol | `protocol/mod.rs`'s data types (`Kind`, `Operation`, `Promise`, `Call`, `Check`, `Status`, `Verdict`, the trait `Suite` `:355`) and `shared.rs` name no suite. `SUITES` (`:417`) and `statement` (`:671-706`) name all four. `Limit { reason, setting }` is built three times (`protocol/linear.rs:529`, `modal.rs:480`, `ordinary.rs:475`). | data shared; the registry a sum |
| The LJT engine's runtime | Step 54's record holds: its own memo (`ordinary/ljt.rs:291`), step arena, `POLL` (`:85`) and work count; `Account` and `Reason` used as they are. | shape only |
| The front ends | The command shares `Commands`, `run`, `load` and the word splitter (`cli/src/interact/mod.rs:135-300`); `goals`, `opened` and the `save` arm are copies in its three session files. The bindings copy `Changed`, `session_of`, `written` and `undo` per suite (`web/src/calls.rs`, `ordinary.rs`, `rules.rs`). | copied, outside the library |

### 3.3 The totals

- **Copied by at least two suites**, the gate's measure: the arena and
  sequent 300 lines, the parser 296, the JSON form 143, the goal tree's
  helpers at most 230 (60 of them the same text), the meters and
  pre-checks about 80: about 1 050 lines, 0.7 % of the crate, counting
  the second copy of each.
- **Shared and suite-free today**: `limits` 844, `ParseError` 114,
  `hash` 23, `kripke` 125, the wire machinery about 170, `serialize`'s
  `auto` and `exact` about 60, `name` 149, `Stops` 264, `memory` 309:
  about 2 060 lines; the renderers and the Kripke drawing about 2 070;
  the protocol's data about 460. About 4 600 lines, 3 % of the crate.
- **The records against the code**: 65a's list holds line for line.
  49a's list holds for the linear and ordinary states; the state of
  step 61 for calculi given as data copied the JSON pattern and the
  open-goal count only. 54a's list is shape, not code. Structure-5's
  list was "judged from the rules files" by its own word; the code
  moves `Error` and `Settings` from its generic column to the sums.

## 4. The four findings

### 4.1 One crate, and no second one planned

D25 extracts the kernel "only after a second non-embedded suite has
shown what is shared", and keeps it unpublished "until a second suite
crate depends on it". The second suite came (the modal suite), as a
module. A kernel crate today has one dependant, `deduxit`, and through
it the command, the bindings and the harness. Layering is the only
thing a crate would then add to the root, and nothing has broken it:
every use that crosses is a `crate::linear::…` path that section 6
lists.

### 4.2 Publishing

The Cargo book (read 2026-11-16, "Specifying dependencies"):
"crates.io does not allow packages to be published with dependencies on
code published outside of crates.io itself (dev-dependencies are
ignored)", and a dependency with a `path` and a `version` uses "the
registry version" when published. `deduxit` is on crates.io at 0.5.0
and its `package` check builds it as crates.io gets it. So a kernel it
depends on is published with it: a name claimed for good (a version
"can never be overwritten"), a first publication by a token (a trusted
publisher "can be configured only for a crate that exists",
`release.yml`), the same version at every release, a page on docs.rs,
and the `package`, `msrv`, `features` and `breaks` checks for one more
crate. wasmtime's `wasmtime-internal-*` crates are the precedent the
structure note found for publishing internals without a promise; it is
a cost taken for a split that pays, not a way around it.

### 4.3 The root's shared types are sums

`Error` is one closed family by decision (`core.md`, "One error
family"), and a suite's own error is a variant of it. So it stands
above the suites in the order of dependencies, with `Settings`, the
wire level's table, `Drawable`, `Notation` and the protocol's registry.
A crate below the suites takes none of them. And the suite-free code
returns `Error`: `kripke::Model::check_world` (`kripke.rs:72`),
`name::check` (`linear/sequents/name.rs:61`), `wire::upgrade`, the
arena's `add`, `png` and `pdf`. Below the suites that code needs an
error of the kernel's own, which `deduxit::Error` converts from, and
every public function that moved would answer the kernel's error where
0.5.0 answers `deduxit::Error`. That is a changed signature, a break by
`breaks` (cargo-public-api lists it under "Changed"), where the prompt
asks for none.

### 4.4 Too little copied, and nothing gained

- **Size of the copies**: 1 050 lines. Kept as public types of their
  own (`ordinary::Formulas` and `modal::Formulas`, each with its doc
  comments and methods), a generic arena saves the bodies of `add`,
  `holds`, `atom`, `occurrences`, `bound` and `Sequent::new`, about 115
  lines a copy, and adds a trait and the wrappers: no fewer lines.
- **Build time**: measured on cores 10 to 12, three jobs, the dev
  profile with default features, incremental compilation off: the
  library alone is 20.0 s (frontend 11.9 s, codegen 8.1 s); a rebuild
  after touching `lib.rs` with incremental compilation on is 3.5 s.
  Step 47 measured 11.8 s (7.7 + 4.2) at about 110 000 lines and found
  the release relink (fat LTO, one codegen unit) to be the release
  build's cost. A kernel of 3 % of the lines takes at most that share
  off the library's unit, and every edit of it rebuilds the library
  anyway.
- **Speed** (D17): none to gain. The generic code would be instantiated
  in `deduxit` either way; the non-generic hot item is
  `memory::Account`, charged in the focused engine's loop, which
  across a crate boundary needs `#[inline]` or the release profile's
  LTO to stay where it is.
- **The module's size**: none to gain; one module links everything.

## 5. The boundary, for when a split is decided

Three layers, by what the code allows:

**The kernel** (suite-free, one crate): `Limits`, `WorkspaceLimits`,
`Held`, `Progress`, `Phase`, `Space`, `Bound`, `Refusal` and `counting`;
`Clock`; `ParseError`; `ErrorKind`; the hash tables; the checks of atom
names; the walk (`Stop`, `Stops`, the public `Visit` and `Walk`); the
memory account; a meter (count, bound, poll, refuse); the wire
machinery (`Readable`, `Within`, `upgrade`, `version`, `fail`, the
exact-integer serde helpers), reading against a level its caller
gives; the Kripke model and its drawing; the
font metrics and the PNG and PDF renderers behind their features; the
layout trait `Drawn` with `InfId` and the three tree layouts, once its
signature takes the rule labels and the notation as its own types; the
protocol's data types and the trait `Suite`; and the generic forms of
section 6 where they were built, the propositional arena's core among
them (departs from the design: structure-5 keeps every formula arena
out of the kernel; D25's arena, the linear one, stays in its suite).

**The suites** keep everything of their calculus: for `linear`, the
arena, the occurrence forest, the proof term, the checker, the engines
and the parallel runtime (D17, D25); for each other suite its node
type, its tables, its checker and its engines.

**The facade `deduxit`** keeps the sums: `Error` with `describe` and
`Owner`, `Settings`, the wire level and its table, `Drawable` and
`Styles`, `Notation`, the protocol's registry and `statement`, the
workspace, and the re-exports at every published path.

- **The kernel's error.** The kernel answers `Refusal`, `ParseError`
  and one small error of its own for what the moved code raises today
  (`IndexOutOfBounds`, `AtomName`, `Json`, `Version`, `WriteFailed`,
  `NotSvg`, `RenderFailed`, `NoDate`). `deduxit::Error` keeps every
  variant and converts from it variant by variant, so `?` keeps
  working. A moved public function answers the kernel's error: each is
  a line of the breaking list of the 0.y release that makes the split
  (departs from the design, which asks for no break; D18 makes a change
  a version).
- **The old paths.** `deduxit::{Limits, Progress, Phase, Refusal,
  Clock, ParseError, ErrorKind}`, `deduxit::limits::*`,
  `deduxit::wire::{Readable, Within}`, `deduxit::kripke::*`, `deduxit::linear::sequents::{Visit, Walk}` and
  `deduxit::linear::InfId` become `pub use` of the kernel's items.
  `deduxit::wire::LEVEL`, which counts every suite's forms, stays the
  facade's constant, and `deduxit::wire::upgrade` stays its function,
  passing that level to the kernel's reader. The
  semver reference lists a `pub use` at the old name as the way to keep
  a moved item, and `breaks 0.y.0` is its check. The sealed traits
  (`sealed::Readable`) move with the trait they seal, and the kernel
  exports the seal to `deduxit` alone through a `#[doc(hidden)]` module,
  which is the one place the kernel's public surface is wider than it
  means.
- **The name and the publication.** `deduxit-kernel`, claimed by the
  author when the split is decided, published with `deduxit` at the
  same version under an exact requirement (`=0.y.z`), its README saying
  that it is `deduxit`'s internals with no promise of its own (departs
  from the design's "unpublished at 0.0.0", by finding 4.2). The
  workspace moves to `crates/` in the same step, `default-members` the
  command and the library.
- **The features.** `serialize`, `png`, `pdf` and `svg` exist in both
  crates, `deduxit`'s forwarding to the kernel's; both `cargo hack`
  runs cover the kernel.
- **The cost**, estimated: four to five sessions (section 8); two
  manifests' worth of checks in `modules/`; the first publication, the
  author's; and at each session the D17 runs. Build time and module
  size as in 4.4: no gain expected, to be measured by
  `cargo build --timings` and `bench/footprint.py wasm` before and
  after.

## 6. One copy inside the crate, without a crate

None of these needs a crate, and none is done by step 67: each is small,
and each has a step that touches the code anyway or a trigger that
makes it pay. `plan/later.md` keeps the list.

1. **Move to the root what other suites use from under `linear`**, the
   layout rule's own case, with no public path changed:
   `linear/search/memory.rs` (crate-private), `linear/sequents/name.rs`
   and `Stops` (crate-private). `Drawn` goes to `export` beside its
   walker; `InfId`, `GoalId` and `Labels` are public and stay where
   they are until a split re-exports them. No effect on the counters:
   the code is the same in the same crate. Taken by the first session
   that edits one of the files, or by the split.
2. **One arena core for the propositional suites**: a crate-private
   `Arena<N>` at the root owning the nodes, the hash-consing map, the
   foreign-id check and the `u32` refusal, asking of its node
   `operands()` and equality, with the name table a separate value
   (first-order atoms will not be plain names). `ordinary::Formulas`
   and `modal::Formulas` stay two public types that hold one each, so
   no public path or method changes and the ids stay distinct types.
   Waits for step 74, which gives `ordinary::Node` quantifiers and
   predicates and so shows what the core must not assume, or for a
   third arena of this shape, whichever comes first. D17 check: the
   ratchet's `read-tptp`, `read-json` and `ordinary-pigeons`.
3. **One parser over tables** (prefix operators, binary operators with
   strength and grouping, constants, a constructor that may make two
   nodes): after step 74 has added binders and terms to the ordinary
   parser, since that decides whether the two still share a loop. Same
   check.
4. **The sequent's JSON form** generic over the formula's form: with
   item 2.
5. **The goal tree's helpers** as three free functions over "the
   premises of an inference" (`subtree`, `postorder`, the reader's
   history walk) and the graft's renumbering: with the step that gives
   the ordinary session its term table (74, by 49a's note). No generic
   goal state: the three sessions differ in what a step records, what
   `undo` restores besides the tree, and what closes a goal, and the
   state for calculi given as data shares none of the algorithms.
6. **One meter** in `limits` for `rules/check.rs` and `modal/model.rs`,
   and one memory pre-check for the two model files: when a third
   checker would write the struct again (step 68's tableau checker is
   the next candidate).
7. **`protocol::Limit` from a `Reason`** in one function for its three
   sites: by any session that edits them.
8. **The front ends' copies** (`goals`, `opened`, `Changed`,
   `session_of`, `written`): the command's and the bindings' own
   matter, outside the library; `cli.md` and `web.md` are where a
   decision on them goes.

## 7. What reopens the question

- **A suite in a crate of its own that `deduxit` is to depend on**: a
  decision to split the library for its build time or for ownership.
  Today's figures are the baseline: 20.0 s for the library's dev unit
  without incremental compilation, 3.5 s for an incremental rebuild.
- **A suite from another repository that must not link the linear
  suite.** One that may link it depends on `deduxit` as it is and needs
  no kernel.
- **A module per suite for the page** (D29's triggers) is not one: a
  smaller module is `deduxit` built with fewer suites, which asks for a
  cargo feature per suite and meets the same sums (`Error` already
  carries `Step` and `Unsupported` behind features).

## 8. The split's sessions, when it is decided

Each on Opus 5.5 at `high` but the first; each ends with the gate, and
with its D17 check: `bench/targets.sh LABEL` and `bench/net-targets.sh
LABEL` compared with `bench/targets/47.csv` and `net-47.csv` by
`deduxit-bench summary --against … --counters` (every decided row's
counters equal), and `ratchet` within its ceilings, a journey that
moves answered by pinned time (`bench.md`, "What a move means").

1. **The boundary against the code of that day** (Fable 5.2, `high`,
   as the prompt gives the boundary's session; the brief speaks of the
   extraction's sessions on Opus 5.5 only):
   this note's inventory run again, the kernel's error settled, the
   name claimed, the breaking list drafted for the author.
2. **The crate and the suite-free values**: `crates/kernel` with
   `limits`, `Clock`, `ParseError`, `ErrorKind`, the hash tables, the
   names, the walk, the memory account; `deduxit` re-exporting. The
   account is the one item of this session under the engines: its
   methods take `#[inline]`, and the pinned time of the focused rows
   is compared on the same cores before and after.
3. **The kernel's error, the wire machinery, the Kripke model, the
   protocol's data**: the conversions, the sealed trait, the wasm32
   checks (`web-core-wasm32`, `web-wasm-tests`). D17: the counters, as
   nothing under the engines moves.
4. **The layouts and the renderers**: `Drawn` with its own label and
   notation types, the tree layouts, the fonts, PNG and PDF behind
   forwarded features; the `export` check and the snapshots unchanged.
   D17: the ratchet's three `render-` journeys and `derivation-chain-64`,
   which step 47 found at +1.95 % of a 2 % tolerance.
5. **The workspace**: `crates/`, `default-members`, the flake's checks
   for two published crates (`package`, `msrv`, `features`, `breaks`),
   `release.yml`'s order, CLAUDE.md and the rules files' `paths`.

## 9. Decided here, where the prompt is silent or the code differs

- The gate is judged on code two suites hold twice, not on what the
  root already shares once: sharing inside one crate needs no
  extraction.
- `Error`, `Settings`, the wire level, `Drawable`, `Notation` and the
  protocol's registry stay in `deduxit` in any split (departs from the
  design: structure-5 lists the error family and `Settings` as
  generic).
- The parallel runtime is no candidate: one suite uses it.
- A split publishes the kernel and is a 0.y release with a breaking
  list (departs from the design: "unpublished at 0.0.0", "no break").
- The in-crate items of section 6 are not run as sessions of step 67;
  each waits for the step named with it.

## 10. Not verified

- No extraction was tried, so the claims about `breaks` (a re-export
  reads as no break, a changed error type as one) rest on the Cargo
  book's semver page and on `core.md`'s account of the two tools, not
  on a run.
- The line counts of "identical after renaming" are `diff` counts of
  the files as they stand, tests left out; the goal tree's 230
  lines are the functions that hold the copied ranges, by the
  reviewer's count.
- The build was timed once, on the slower cores, beside 66c's session
  on cores 7 to 9.
- The speed claim for `memory::Account` across a crate boundary is an
  argument from the profile (`lto = "fat"`, `codegen-units = 1`), not a
  measurement.
- 66c's exporter was not read; it adds certificates under
  `export/rocq`, which stays in the facade in any split.
