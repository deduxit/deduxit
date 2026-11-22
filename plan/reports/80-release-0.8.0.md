# Step 80: the release of 0.8.0, prepared

Opus 5.5 at `high`, the worker of 75b, 75d, 76a to 76c, 77c and fixup-24
keeping its conversation, started by `deduxit-orchestrator` from
`/tmp/linlog-handoff/start-80.md` on 2026-11-23, cores 7 to 9, in the jj
workspace `../deduxit-wt-step-58a-610ed3`, begun on `main@origin`
`opqvwylk` ("Plan: 78c landed, the lock fixed forward; step 80
started") and rebased at the end onto `mrlkmlxy` ("Plan: fixup-26
started; nothing new after step 80"), which adds plan files alone;
rocq-deduxit in its checkout on its `main@origin` `vxkypqzt`.
Nothing was tagged, released or pushed.

## Outcome

0.8.0 is ready to tag in both repositories, up to the author's acts.
Both releases are dated 2026-11-23.

- **rocq-deduxit 0.8.0** (`zxknsvsx`, "Date the release of 0.8.0"): the
  changelog's `[Unreleased]` became `## [0.8.0] - 2026-11-23` with a
  first paragraph and its link, and the header says why there is no
  0.7.0; the opam file's version, date, description (first-order logic,
  Kripke models with growing domains, finite structures) and a keyword
  `first-order logic`; `CITATION.cff`'s version, date, abstract and
  keyword; README's install lines at `v0.8.0`, its opening naming LJ,
  LM and LK at first order, and "What it covers" naming deduxit 0.8.0's
  first-order countermodels in place of "the next release". No
  statement or proof changed. Its `nix flake check` passed. The
  planning session pushes it; its tag goes first.
- **deduxit 0.8.0**: the chain below. Its last commit, `zpxsuouq`, names
  rocq-deduxit 0.8.0 in `export::rocq::DEDUXIT`, and goes on top of the
  lock moved to rocq-deduxit's tagged commit, which the brief puts after
  the author's tag; until then it fails the flake's evaluation (the
  assertion that `DEDUXIT` equals the pinned library's version), so the
  full check ran on the chain below it, and `zpxsuouq` was checked
  against the local release commit with `--override-input`. The release
  notes, what a user can do now, the order of the tags and the
  publishing and the author's checklist are in
  `plan/notes/release-0.8.0.md`.
- **fixup-25** (the clausal normal form within its bounds) and
  **fixup-26** (the connection provers' setup within its bounds) had not
  landed when this chain was ready; the notes say how their changelog
  entries fold in.

## The break: `breaks 0.7.0`, and how each line is covered

`breaks 0.7.0` (cores 7 to 9, at `opqvwylk`, before the version raise):

- cargo-semver-checks: 229 checks, 228 pass, 1 fail, 31 skip,
  "semver requires new major version": `enum_variant_added`, the
  exhaustive `ordinary::Node`'s `Forall` and `Exists` and
  `ordinary::Rule`'s `ForallLeft`, `ForallRight`, `ExistsLeft` and
  `ExistsRight`.
- cargo-public-api: 390 lines Removed and 47 pairs Changed, all of one
  kind: the term types `Symbol`, `FoTerm`, `FoTermId`, `Args`, `Eigen`
  and `SymbolFault` are defined in the crate's private `first_order`
  module since step 74b, and the tool prints each item under its new
  path. A script matched every removed line to an added one and every
  changed pair to the same line under the new path (0 without a
  counterpart). `linear::sequents::fo` re-exports them, so
  `linear::sequents::fo::FoTerm` and every other old path still resolve.
  1 419 lines Added (`ordinary::{connection, equality, grounding,
  herbrand, models}`, `resolution::first_order`, the structures and
  Kripke models with growing domains, `export::rocq`'s first-order
  writers and bounds, the search options' defaults and the rest of
  steps 74 to 78).

**How each line is covered.** The six variants are the changelog's
"What breaks" (written by 74b and 74c, which planned them as the 0.y
bump the closed enums take). The moved paths break nothing; "What
breaks" says so, with the paths kept. Every other new variant or field
belongs to a `#[non_exhaustive]` type: `[Unreleased]` listed
`Outcome`'s new field `structure` as a break, which it is not
(`Outcome` was marked at 0.7.0), and it moved into that sentence (the
reviewer's finding 4).

**The error codes**: 0.7.0's 85, none removed or renamed, and three new
(`first_order`, `glivenko_first_order`, `invalid_structure`), compared
with `v0.7.0`'s `Error::CODES`. Two of them stand within the list, by
kind, not at its end; the list promises no order.

**The two lists of types**, against the head's rustdoc JSON with every
feature (built as `breaks` builds it), by a script over every public
enum and struct with a public field by its defining path: 19 closed
enums, exactly `core.md`'s list (`FoTerm` now defined as
`first_order::FoTerm`), and 240 `#[non_exhaustive]` types (217 at
0.7.0). The 26 public enums and structs `breaks` lists as added are
each in `core.md`'s list or outside both lists (`first_order::Clause`,
`Clauses` and `Refutation` have no public field) but for
`ordinary::connection::{Outcome, Statistics}` (75b), now listed. The four
marked structs without a public field are as at 0.7.0. `core.md` gives
the counts at 0.8.0 (`wyxvmvpr`).

## What was prepared

- **The version** (`qoulmrtv`) is 0.8.0 in the manifests and both locks
  (`cargo metadata`; `cargo update --offline -p deduxit` in `fuzz/`),
  `CITATION.cff` (2026-11-23), the dependency lines of README, the
  library chapter and the installation chapter, the installation
  chapter's file names, the JSON examples of the docs (`wire.rs`,
  `ordinary/decide.rs`, `negative.rs`, `interpolant.rs`, `normal.rs`,
  `resolution/mod.rs`) and the manual (`terms.md`, `ordinary-logic.md`),
  and the behaviour lock's inputs (`cli/tests/lock/inputs/workspace.json`,
  `MODAL_OUTCOME`, `STRUCTURE` and `WRONG_STRUCTURE` in
  `cli/tests/lock.rs`) in the bump's own commit. Not changed: the compat
  crates' 0.3.0, the dataset card's "`grade`, which comes with 0.7.0",
  the stability test's "as 0.7.0 wrote them" (history), and
  `DEDUXIT`'s 0.6.0 until `zpxsuouq`.
- **The bless** (`qosokkms`): the JSON lock's 18 outcomes, the serialize
  test's five, the six `check` outputs of the command's lock that echo
  the input's version, and `core/tests/wire/level-26.txt` and
  `level-27.txt`, written by builds before any release and first
  written by 0.8.0 (as 69 did for level 20): 34 lines out and 34 in,
  equal with the version masked.
- **No new stability file**: levels 25 to 27 have theirs; 28 and 29 are
  written only.
- **The status chapter** (`mlqtuqru`): the planned 0.8.0 bullet became
  "after 0.8.0: first-order logic on the web page"; "not yet placed in
  a release" names Rocq certificates of first-order derivations of LK,
  LJ and LM; a certificate over `Prop` and a session's `close` and
  `hint` are said to take propositional sequents (both refuse a
  first-order one, tried with the release build); the command's list
  names `herbrand`; the open witness is named.
- **The descriptions** (`nwyyomxo`): README's opening, `CITATION.cff`'s
  abstract, its reference to rocq-deduxit and a keyword `first-order
  logic`, both crates' descriptions and the crate docs' "What you can
  do" name first-order logic and Herbrand's theorem.
- **The changelog** (`rysqymvw`): `## [0.8.0] - 2026-11-23` under an
  empty `## [Unreleased]`, a first paragraph of what 0.8.0 brings (the
  page's part after it), "What breaks" (the six variants, then a
  paragraph that nothing else breaks and the moved paths resolve), "Added" grouped by theme (reading, proving step by step,
  deciding and refuting, the transformations, the rest), "Changed" with
  a line for the help, the command list and the workspace's operations
  (every locked output changed since `v0.7.0` besides the version: the
  help of `prove`, `interact`, `seq print` and `check`, the command list,
  `workspace operations`), "Fixed" (five faults of code 0.7.0 had),
  "Wire forms" with levels 28 and 29 and `wire::LEVEL` 29, the link
  `[0.8.0]`. fixup-23's two defaults, which changed what two "Added"
  entries said, are in them: the library's default connection prover
  works within 2²⁶ of its steps, and the model search goes first on one
  thread; "the certificates need rocq-deduxit's next release" names
  0.8.0; fixup-11's web module size compares 0.8.0 with 0.7.0 (2.8 MB
  against 3.8 MB) where it gave the day of the split's figures.
- **The rules** (`wyxvmvpr`): `core.md`'s counts at 0.8.0 and
  `ordinary::connection::{Outcome, Statistics}` in the marked list.
- **The guide** is within its budget: the summary is unchanged;
  `reference.md`'s tag line changes in `zpxsuouq` (`v0.8.0` for deduxit
  0.8.0, `v0.6.0` for 0.6.0 and 0.7.0); README's test runs its examples.
- **`zpxsuouq`**: `DEDUXIT` "0.8.0", its doc naming
  `Deduxit.Intuitionistic`'s first-order logic; the Rocq chapter's
  pinned versions, its paragraph on the library's version and its
  install lines; the guide's tag line.
- **The release workflow's dry run**:
  - its tag check, the job's three tests on the tree (mirrored in Python
    rather than run with its `sed`): the manifests' 0.8.0 equals the
    tag, the changelog's section is dated, `CITATION.cff` states it;
  - its crate selection against crates.io's API: `linlog 0.3.0` and
    `linlog-cli 0.3.0` "on crates.io already", `new=` none,
    `existing=deduxit deduxit-cli`, so the job publishes both through
    Trusted Publishing and uses no secret;
  - its `cargo package --locked --package=deduxit --package=deduxit-cli`
    from a clean tree at `zpxsuouq`: exit 0, both packaged (557 and 288
    files) and verified at 0.8.0;
  - `cargo publish --dry-run` is denied to this session, as before.
- **The release notes** (`vwopwqru`): `plan/notes/release-0.8.0.md`, the
  workflow's text generated from the changelog as its `awk` reads it,
  what a user can do now (each command run with the release build), the
  order of the acts and the author's checklist, with what the opam
  submission needs.

## The checks

Every build and test ran in a capped scope on cores 7 to 9
(`CARGO_BUILD_JOBS=3`, `RUST_TEST_THREADS=3`); every nix build under
`flock -o /tmp/linlog-machine.lock`.

- **rocq-deduxit's `nix flake check`** at `zxknsvsx` (`--max-jobs 2
  --cores 6`): "all checks passed!".
- **`breaks 0.7.0`** and the lists of types: above.
- **`cargo test --workspace`** after the version raise, before the
  bless: only `json_lock`, `outcome_json_format` and the command's
  `behaviour_lock` failed, on the version alone, as at 0.7.0. After the
  bless the locks pass.
- **`zpxsuouq` against rocq-deduxit 0.8.0**: `nix build
  .#checks.x86_64-linux.rocq-deduxit
  .#checks.x86_64-linux.rocq-deduxit-library --override-input
  rocq-deduxit git+file:///home/tux/Projects/own/rocq-deduxit?rev=e3eecb30…`
  (the release commit): built, so the assertion holds and every
  certificate compiles against it.
- **`cargo package`**, the tag check and the crate selection: above.
- **`nix flake check --keep-going`** at `vwopwqru` (the chain below
  `zpxsuouq`; 00:26 to 00:51): "all checks passed!", the gate's six
  parts among them. Only the changelog's and the notes' text changed
  after, by the review's fixes; no check reads either.

## The reviewer

`fresh-reviewer` (Fable 5.2, `high`) read the chain and rocq-deduxit's
commit against the brief and 73's pattern. Five findings, none in the
code, all fixed in the commits they concern:

1. **The notes pushed the wrong commit after the lock's move**: after
   step 3's commands `zpxsuouq` is a sibling of `@`, so `plan/push-main.sh
   @-` would have moved `main` to the lock commit, where the lock pins
   0.8.0 and `DEDUXIT` still says 0.6.0, failing the flake's evaluation.
   Step 3 now abandons the empty change and step 4 pushes `zpxsuouq`
   (`vwopwqru`).
2. **The changelog gave the web module's size of the day of the split**
   (2.4 MB against 3.9 MB, 2026-11-21, never released); for 0.8.0
   against 0.7.0 it is 2.8 MB against 3.8 MB (`budget.nix`'s figures;
   `rysqymvw`, and the notes' copy).
3. **The notes said the `budget` check passed before the full check had
   ended**: the sentence now cites the run's result below (`vwopwqru`).
4. **"What breaks" listed `Outcome`'s new field**, which breaks nothing
   (`Outcome` was `#[non_exhaustive]` at 0.7.0), against the section's
   own closing paragraph: the entry is gone and the field is named in
   that paragraph (`rysqymvw`).
5. **The opening said every "valid" comes with a derivation**; grounding
   gives a checked clausal proof, as its own entry says. The sentence
   says so now (`rysqymvw`).

It confirmed the rest: every remaining 0.7.0 names history; the bless
changes 26 bytes, each a 7 become 8; the locks', readme's, stability's
and serialize tests pass at `vwopwqru`; `breaks`' every line accounted
for; the error codes; every former `[Unreleased]` sentence in the
section, every locked output changed since `v0.7.0` covered, every name
cited existing; each claim of the status chapter tried with the release
build; the notes' text equal to the changelog's section; the guide at
7 999 bytes and 93 lines; the 240 marked types plus `Computed` in
`core.md`'s list; rocq-deduxit's commit consistent, no `.v` file
touched, its check passed. It made and at once abandoned an empty
change on top of `zpxsuouq` by mistake (no content changed).

## Departures

- **`zpxsuouq` comes last and fails until the lock moves** (departs from
  0.6.0's chain, which moved the lock itself): the brief moves the lock
  after the author's tag, and `DEDUXIT` must equal the pinned version, so
  the commit that names 0.8.0 waits on top, and the notes give the
  commands.
- **0.8.0 ships without Rocq certificates of first-order derivations**
  (departs from the design): 74a's design (§8) planned them as terms
  over `Prop`, the old status chapter promised them for 0.8.0, and the
  writer refuses a first-order derivation ("a certificate over Prop takes
  propositional sequents only"). No report of 74 to 78 records them as
  deferred. The first-order countermodels have their certificates. The
  status chapter now says so; a step after 0.8.0 builds them, or the
  author drops them.
- One entry of `[Unreleased]`'s "Fixed" (grounding and the model search
  stopping at the time limit on a nest of quantifiers) is left out: it
  fixed code new in 0.8.0, which no 0.7.0 user met; the features'
  entries describe the behaviour.
- README's opening, `CITATION.cff`'s abstract and keywords and both
  crates' descriptions gained first-order logic; the brief names README
  for installation only (departs from the design, as at 0.6.0 and
  0.7.0).

## What is left open

- The tags and the publishing, in the order of
  `plan/notes/release-0.8.0.md`, with the author's checklist there.
- fixup-25's and fixup-26's changelog entries, folded into 0.8.0's
  section when they land.
- The opam submission of rocq-deduxit, the author's when they choose.
- Rocq certificates of first-order derivations (above).
- `CLAUDE.md`'s first paragraph still says deduxit decides "classical,
  intuitionistic and minimal propositional logic"; the Claude setup is
  the planning session's to amend.

## The commits

deduxit, on `main@origin` `mrlkmlxy`, oldest first:

- `qoulmrtv` Raise the version to 0.8.0
- `qosokkms` Bless the behaviour lock: an outcome states version 0.8.0
- `mlqtuqru` Keep the status chapter true for 0.8.0
- `nwyyomxo` Describe 0.8.0's first-order logic and Herbrand's theorem
- `rysqymvw` Date 0.8.0's section of the changelog, with what breaks and
  its wire levels
- `wyxvmvpr` Rules: count the marked types at 0.8.0
- `vwopwqru` Plan: the release notes of 0.8.0 and the author's checklist
- (this report) Report 80: the release of 0.8.0, prepared
- `zpxsuouq` Name rocq-deduxit 0.8.0 in the certificates (after the lock
  moves; see above)

rocq-deduxit, on `main@origin` `vxkypqzt`: `zxknsvsx` Date the release
of 0.8.0.

## The state of what later.md keeps for after 0.7.0 and 0.8.0

Read on 2026-11-23 at `main@origin` `opqvwylk` ("Plan: 78c landed, the lock fixed forward; step 80 started"), with `plan/later.md` whole, `plan/README.md` (D24 to D32, the step table and the status log to 2026-11-23 00:25), the reports of steps 74 to 78, the sections of fixups 10 to 24 in `plan/reports/fixups.md`, and 73's survey as the base. Steps 74 to 78 are complete. fixup-25 (77a's clausal normal form within its bounds) started on 2026-11-22 at 22:55 and has no section in `fixups.md` and no commit on `main@origin` yet. No decision after D32 is recorded; the author's later word that moves the plan is in the status log (2026-11-22 10:10: the website frozen until 0.8.0, step 79 after it).

How to read it, as in 73's survey: an item is one candidate id of step 46's ranked list, or one entry of a `later.md` list or section. "Taken" names the step or fixup and its report. "Changed" means a step touched it and something is left. Where no report touches an item, the line says so.

`later.md` since 73. It gained three sections, each from a report: "The modal walk in depth: rows it slows" (fixup-10, 2026-11-21), "A native equality rule in the connection prover" and "The intuitionistic connection prover beside the embedding, with `=`" (both 78c, 2026-11-22). The fixups and the planning session removed or rewrote these entries: LJ's unary `¬L` and the warnings without `parallel` (fixup-12), the first-order table with binders and no atoms (fixup-12), LJT's four leftovers (fixup-18), the pigeonhole entry rewritten to its second way and `comparison.sh`'s ILTP pass removed (fixup-19), KD4's cost (fixup-10) and seriality with one new label (54x7, removed 2026-11-20), auto-reverse narrowed to rule files, and the web module's size marked with fixup-11's renderers' module. Entries that fixups 19 to 21 added and closed again leave no trace. The step-46 table at the top of `later.md` still shows the six candidates as "74, 75, 76" and "78", and the kernel crate's list does not record what step 74 took. The follow-ups of steps 63 to 78 that only a report records are still not copied into `later.md`; section 2 below lists those of 74 to 78.

### Counts

| group | ranked candidates | entries open (of 73's) | entries closed since 73 | added since 73 (in later.md / in reports only) | what it holds |
|---|--:|--:|--:|--:|---|
| Research items (D30 and the rest) | 37 | 0 (0) | 0 | 0 / 0 | cyclic proofs, System F, deep inference, proof theory |
| Deferred verification (D31) | 9 | 3 (6) | 3 | 0 / 3 | deductive verification, certificates still missing |
| Engine and performance follow-ups | 14 | 18 (21) | 3 | 3 / 20 | measured-only lists, the first-order provers' gaps, sessions, LJT |
| The web client | 14 | 10 (10) | 0 | 0 / 3 | the module's size, page calls for first order, judgement calls |
| The author's own | 8 | 12 (12) | 0 | 0 / 4 | Zenodo, nixpkgs, opam, tags, unattended decisions to review |
| The rest | 47 | 3 (3) | 0 | 0 / 3 | applications, interop, types, modal extensions |
| Assigned to steps 74 to 78 | 6 | | | | 3 done, 3 partly done |
| Total | 135 | 46 (52) | 6 | 3 / 33 | |

Of the 135 ranked candidates, 123 kept and 6 half-kept stand as in 73: no step of 74 to 78 started one of them, though four are now unblocked (engines-10's and interop-11's prerequisite interop-5 partly landed, applications-5's quantifiers exist, modal-logics-8 has first-order terms and a modal suite to stand on). Of the six assigned, three are done and three partly done (below). "Changed" entries count as open. Dropped items are as in 73.

### Research items

D30 (decision 7): the author starts these after the decidable suites. No report of 74 to 78 or of the fixups mentions any of them; each line of 73's survey stands.

- `cyclic-proofs-1` to `-5`: the trace-condition checker first, then μMALL, CLKIDω, Cyclist-style search and Rocq certificates. Still open. Waits for the author's go; no report touches them.
- `lambda-calculi-6`: System F as checking only. Still open. Its prerequisite (step 71) exists; no report touches it.
- `logics-1`, `-2`, `-6` (exponential half): SELL, graded and bounded exponentials, LR. Still open, the author's choice after 0.7.0; no report touches them.
- `logics-3`, `-4`, `-5`, `-7`, `-8`, `-9`: ordered cut and Lambek with modalities, deep inference, fixed points beyond μMALL, light logics, adjoint logic, the further items. Still open, research; no report touches them.
- `proof-theory-9`: explaining unprovability. Still open, the author's choice. Step 76 adds first-order countermodels that explain "not valid" where a finite model exists, which is the countermodel feature, not this one.
- `proof-theory-1`, `-12`, `-13`: proof equivalence, interpolants by cut introduction, MALL type isomorphisms. Still open, research. (Step 77b extended Maehara's interpolants to first order, which is not proof-theory-12.)
- `proof-theory-3`: unification nets for first-order MLL. Still open. No report touches a first-order net data model; steps 74 to 78 built the ordinary suite's first order, not the linear nets'.
- `proof-theory-4`: subformula linking in the interactive session. Still open. Steps 74c and 74e changed the ordinary session (quantifier steps, the open witness) and did not take it.
- `proof-theory-5`, `-6`, `-7`, `-8`, `-10`: calculus of structures, proof compression, token machine, dialogue mode, combinatorial proofs. Still open; no report touches them.
- `custom-rules-3`, `-4`, `-5`: bounded search under user rules, static analysis, synthetic rules. Still open (D28); no report touches them.
- `semantics-decision-2`, `-3`, `-7`, `-8`: Presburger invariants, ideal Karp-Miller, finite phase countermodels, Petri nets with data. Still open; no report touches them.
- `engines-2`, `-3`, `-5`, `-6`: lazy SAT over linkings, refutations through a solver, learned dispatch and ordering. Still open, research; no report touches them.

### Deferred verification (D31)

D31's first two parts are done (steps 51 and 58); the third waits for the checker's refactoring. No report of 74 to 78 touches the Rust checker's verification.

- `verification-4`: deductive verification of the Rust checker (Verus or Creusot by a trial). Still open. Waits for the checker refactor and the author's go; no report touches it.
- `verification-5`: refinement to the Rocq checker (Aeneas or hax). Still open, as verification-4.
- `verification-1`: a second checker fuzzed against the first. Still open; no report touches it.
- `verification-3`: a checker extracted from Rocq. Still open; no user asked.
- `verification-2` (iii) with `semantics-decision-10`: certificates for saturated refutations. Still open, a project of its own.
- `semantics-decision-12`: failure certificates of the backward search. Still open.
- `proof-theory-14`: a certified net criterion in the Rocq library. Still open. rocq-deduxit gained first-order Kripke semantics and LJ/LM/LK soundness in 76c, not sequentialization.
- `lambda-calculi-7`: a Rocq checker for typing derivations. Still open; no report touches it.

Entries:

- **The basis's Rocq lemma** (`Unsupported::Basis`). Still open; no report touches it.
- **Second certificate kernels' leftovers** (open goals as hypotheses, `ex_perm_r`, identifier escaping). Kept; no report touches them.
- **Kani's weekly run and the unreached facts**. Watch, not an action; no report touches it.
- **Unrun fuzz targets** (`lambda_text`, `lambda_inhabit`). Taken by 54x9 (status log, 2026-11-20 21:40): five lambda and modal targets ran up to two hours each, two finds fixed (`poyxupyy`, `plmznllv`); four targets reached the cap still growing (a departure), the corpora kept in that workspace.
- **First-order table with binders and no atoms**. Taken by fixup-12 (`fixups.md`, "fixup-12"): a fault, fixed with a test that failed before; the entry is out of `later.md`.
- **The Rocq writer and LJ's unary `¬L`**. Taken by fixup-12: a fault, the writer applies the negation to the premise, both certificates pinned; the entry is out of `later.md`.

### Engine and performance follow-ups

All of the propositional and linear ones still wait for a measurement that asks (D19), a timed run the author gives, or a user. Steps 74 to 78 changed no linear engine: 74b and 74c moved step 42's terms and the unifier to the crate's root with the focused engine's target set equal to `47.csv`, and 74e's shared goal tree likewise.

- `engines-13`, `engines-14`: LinearOne's eight problems; faster copy-bound deepening. Still open, measured only; no report touches them.
- `engines-4`: encodings as exports. Still open. 78a reads TPTP `cnf` and writes SZS lines, not DIMACS or SMT-LIB.
- `engines-9`: search state across calls. Still open; no client asks.
- `engines-10`: Horn engine inputs (STRIPS, PNML). Still open. Its stated prerequisite interop-5 is partly done (78a); no report touches STRIPS or PNML.
- `engines-11`: linear logic in the TPTP World. Still open: it is interop-5's part (c), a proposal to the TPTP authors and the author's outward act; 78a's SZS line is for ordinary logic only.
- `engines-15`: the inverse method at first order. Still open; no report touches it. First order went to the embedding and the connection provers (75).
- `engines-16`: a first-order linear comparison library. Still open. 75d and 78c compared the ordinary suite with leanCoP, nanoCoP and iLeanCoP on ILTP, which is classical and intuitionistic, not linear.
- `engines-17`: a generic focused engine over rule schemas. Still open (D28).
- `semantics-decision-4`, `-5`, `-6`, `-9`. Still open; no report touches them.
- `engines-7` (the environment half). Still open; no report touches it.

Entries:

- **The focused engine's whole follow-up list**. Kept; no report touches it.
- **The Horn engine's list**. Kept; no report touches it.
- **Parallel search's list**. Kept. The command's first-order races (75d, 76a, 76b, fixup-23) use their own threads in `cli/`; none changes the runtime.
- **The net engine's list**. Kept; no report touches it.
- **The inverse method's leftovers**. Kept; no report touches them.
- **Benchmarks found by comparisons** (`chain-fo`, `wide-m1`, `pigeonhole` #1, the resumption switch at first order). Kept; no report touches them.
- **A turn bounded in time** (step 52's). Still open; no report touches it.
- **Benchmark leftovers** (`bench/reruns.txt`, the forward bound, step 28's lists). Kept; no report touches them.
- **A search that can be suspended**. Kept; no report touches it.
- **Deferred beyond the 2026-10-03 table**. Kept; no report touches them.
- **Coverability against the dedicated tools**. Open; waits for a night the author gives.
- **A native SAT tier**. Open; no report shows the solver as a limit. (fixup-19 and fixup-20 made the existing SAT and LJT paths give checked verdicts on SYJ202 up to size 6.)
- **KD4's cost**. Taken by fixup-10 (`fixups.md`, "fixup-10"): a cache of world keys across branches in K4, KD4 and S4, the panel's witness from 20.5 million units to 58 100; its cost (single small rows up to 29 times slower) is the new `later.md` entry "The modal walk in depth".
- **Pigeonhole derivations past the derivation limit, and LJT's derivations outgrowing its search**. Changed. fixup-19 took the first way (decided unattended, status log 2026-11-22 05:00): read-backs charge the memory bound and `--derivation-limit` bounds only the drawing, so `-q` and JSON give the checked "valid" for SYJ202 up to size 6. Left: the second way, shared subderivations with a checker that checks each once (the panel's), which is also `ordinary-sequents-4`; `later.md` says so.
- **LJT's other leftovers**. Changed. fixup-18 took three and closed the stale fourth: "limit of 0 B" traced to the Horn engine's share of the bound, the verdict line says when the default gave way, the lambda read-back refuses what it cannot read, and `check` reads LK and LJ derivations. Left in `later.md`: `interact --logic`'s `close` and `hint` decide through the embedding, so a goal only LJT decides stays open.
- **`comparison.sh` has no ILTP pass**. Taken by fixup-19: ILTP is its third pass.
- **`cargo test -p deduxit` without `parallel` warns**. Taken by fixup-12 (the warnings were gone; the check `clippy-core-default` and its line in the gate hold it), then fixup-21 and fixup-22 built each feature's tests and every pair's (`features-tests`).
- **Interactive and exports leftovers** (a session's PNG or PDF render without a time limit, cut in a first-order session, two-sided rule names, the ratchet's `derivation-chain-64` margin, the size estimate's shadowing case, Greek in the SVG, normal forms without doubled vertices). Kept; no report touches them.
- **The "unknown" verdict's wording, a batch's second pass**. Kept; no report touches them.
- **The written succedent of `⊤`/`0`-only formulas; the additive path**. Kept; no report touches them.
- **The tableau's expansions' allocation**. Recorded only in reports; no report since touches it.

### The web client

The author froze the website on 2026-11-22 (status log, 10:10): nothing goes to `deduxit.github.io` until 0.8.0, no page work and no lock bump. The bindings in `web/`, `web-render/` and `web-tab/` still change with the library.

- `web-front-end-4`: a module per suite. Kept. fixup-11 built the larger first-load gain that 63g-c's entry named instead, a module of its own for the PNG and PDF renderers (bindings 3 913 893 to 2 438 116 bytes at gzip). Since then the ceiling rose for first order from 2 500 000 to 2 875 000 (fixup-14, 75c, 76a, 77b, 78b); 78a measured 2 846 308.
- `web-teaching-1`, `-6`, `education-11`: a learner's record and its views. Still open; frozen with the page.
- `web-teaching-3`, `education-1`, `engines-8`: a provability light. Still open (the page's D11).
- `web-teaching-8`, `-10`, `education-2`, `-5`, `-9`. Still open; no report touches them.
- `web-teaching-4` (the remainder): a screen-reader user's review, a structural twin, rule labels by step, label targets on a phone. Still open with the author; no report touches it.
- `education-3` (the remainder): auto-reverse for rule files. Still open; `later.md`'s entry is narrowed to it.

Entries:

- **The web module's size after the workspace**. Changed: the renderers' module is done (fixup-11, `later.md` says so). Left, the author's: the protocol's dispatch and readers (an API change), and `prove_goal`'s four copies.
- **The bindings' leftovers** (a turn's set-up across restarts, `Step` and `Applicable` as wire forms, a call over `search::engine_for`, the inverse method's turns). Kept; no report touches them.
- **The parser reads no `a⊥`**. Still open; no report touches it.
- **Drop the client's Biome override** at a client lock update. Open; no client lock update since fixup-11's, and none until the freeze ends.
- **Ordinary session's leftovers on the page** (49c). Open; no report touches them.
- **A pack's tableau problem; the check view reading the rules suite** (68). Open; no report touches them.
- **Resolution on the page** (70c's departures). Open; frozen.
- **The page's views for modal and rules sessions**. Recorded only in reports; no report touches them.
- **The client's CI e2e** (its duration). Recorded only in reports; no report touches it. (A red e2e of another cause, the invertible switch while a pack opens, was fixed on 2026-11-20 by `vyxlolom`; status log 20:20.)
- **Eight client entries "for the author to revisit"**. Open with the author, now part of the redesign the author announced.

### The author's own items

- `users-1`, `users-3`: Discussions, an announcement. Kept, D32.
- `users-2`: reading the public counters. The planning session's when asked.
- `maintenance-7`, `-6`: install channels, code signing. Open with the author.
- `maintenance-5`, `-8`: cosign, a glibc binary. Kept.
- `llm-agents-1`: an MCP server. Kept.

Entries:

- **A Zenodo community**. Open with the author. Zenodo archived 0.7.0 as record 24404649 without one (status log, 2026-11-20 19:15).
- **nixpkgs**. Kept at the author's word (2026-11-03).
- **The opam submission of `rocq-deduxit`**. Changed: the author's condition was "0.8.0 or later" (step 80's prompt), so it can go after 0.8.0; the file in `~/Projects/own/rocq-opam` is for 0.3.0 and must be redone for the release step 80 prepares. Theirs to send.
- **The LLTP header report**. Open with the author.
- **The comparison on the libraries with 0.2.0's engines and the baseline night**. Open with the author; `status.md` still lists it as "not yet placed in a release".
- **treefmt-nix's Biome module**. Open if the author wants an upstream issue.
- **The web client's judgement calls and Safari's pinch**. Open with the author.
- **ILTP in the dataset, the dataset's licence, a hub upload**. Open, the author's acts. (ILTP's first-order problems are now a flake package of their own, `iltp-first-order`, 74d; still no licence stated.)
- **Whether a model document should refuse an unknown key**. Open for the author.
- **An `EVIDENCE_WORK` settings key and other API changes**. Open.
- **CI left to the author**. Changed: the author deleted the three probe branches (status log, 2026-11-20 19:15) and replaced the Cachix cache with `deduxit` (19:20). Whether this machine reads the cache as a trusted user, no report says.
- **The organisation's profile README, the client's `llms.txt`, the page's About, the repositories' descriptions** ("linear logic first"). Open; checked at a release, so step 80's. The client's texts are frozen with the page.

### The rest

- `structure-2`: a repository per suite. Kept (D25).
- `structure-5` (half-kept): the kernel crate. Kept; nothing reopened it.
- `applications-1`, `-2`, `-3`, `-4` (half), `-6` to `-10`. Kept; no report touches them.
- `applications-5`: a logic-programming mode "after quantifiers". Still open, the author's choice; ordinary logic has quantifiers, unification and resolution at first order now (74, 78b), so its prerequisite is met. No report touches it.
- `natural-deduction-2`, `-3`, `-5`. Kept. Natural deduction stays propositional; no report touches it at first order.
- `lambda-calculi-4`, `-5`. Kept; no report touches them.
- `modal-logics-4`, `-6`, `-7`. Kept; no report touches them.
- `modal-logics-8`: first-order modal logic. Still open (undecidable). The ordinary suite's first-order terms and the modal suite both exist now; no report touches it.
- `interop-2`, `-3`, `-4`, `-6`, `-7`, `-8`, `-9`, `-10`, `-12`. Kept; no report touches them.
- `interop-11`: SZS and TPTP for the collection, native NXF reading. Still open. Its prerequisites are now partly met: interop-5(a)'s SZS line for ordinary logic (78a) and the modal suite (65). NXF's syntax was never read; no report touches it.
- `llm-agents-3`, `-4`, `-5`. Kept; no report touches them.
- `ordinary-sequents-4`: a DAG read-back. Still open. fixup-19 removed one reason for it (the drawing's limit no longer withholds the verdict); what is left is the derivation's size itself, the same change as the pigeonhole entry's second way and LJT's derivations (a change to the checker, the panel's).
- `ordinary-sequents-6`: a structural-rule dial. Kept (needs logics-1).
- `tableaux-4`: modal tableaux. Overtaken, as in 73.
- `more-logics-3`, `-4`, `-5`, `-10`, `-11`. Kept; no report touches them.
- `types-higher-3` to `-7`. Kept; no report touches them.

Entries:

- **The kernel crate's list** (67a). Changed. Step 74 took three of its rows: step 42's terms moved to the crate's root (74b), the unifier to `crate::unify` (74c), and the goal tree's walks to `crate::goal_tree` (74e, `uqsmzkru`), each with the target set equal to `47.csv`. 74a decided the two rows `later.md` gave it: the propositional arena `Arena<N>` is deferred to its own trigger, and the parser is not shared since the two loops no longer share their shape (`reports/74-first-order-ordinary.md`, 74a's reviewer's finding 6; `notes/kernel-boundary.md`). `later.md`'s table does not record any of this. The rest of the list (the memory account under `linear`, the sequent's JSON form, the checkers' meter, `protocol::Limit`, the front ends' copies) is untouched.
- **Resolution's leftovers** (70b). Changed: first-order resolution is done (78b, with paramodulation in 78c). Left: RAT steps are refused; a resolution session in the command.
- **Typed lambda leftovers** (71a, 71b): linear `inhabit`, a settings key `inhabit`, `Term::parse`'s wording. Open; no report touches them.

### Assigned to steps 74 to 78: three done, three partly done

- `more-logics-6` (first-order intuitionistic logic with Kripke countermodels). Done by 74, 75 and 76: LJ and LM at first order with the checker and sessions (74), the embedding and the prefixed connection prover (75c), Kripke models with growing domains found by a SAT search and checked (76b), certified in rocq-deduxit and drawn (76c). Two parts of its note are not built: Rocq certificates of valid first-order derivations (see "Rocq certificates of first-order derivations" in section 2), and a connection prover for minimal logic (75c; minimal logic stays on the embedding).
- `more-logics-1` (classical first-order logic with equality, axioms first). Done by 78a as axioms in every method and logic, "not valid" only with a structure that reads `=` as identity. The native rule was measured and not taken (78c; `later.md` "A native equality rule"). Its note's "certify through Rocq" for a valid derivation is the same gap as above.
- `resolution-2` (first-order resolution at teaching scale). Done by 78b (binary resolution with factoring, given-clause search with subsumption and a set of support, a checker that recomputes every unifier, read-back to a deskolemized LK derivation) and 78c (paramodulation, reflexivity alone by Brand's theorem).
- `tableaux-3` (first-order tableaux with free variables, and connection provers). Partly done: the classical and intuitionistic connection provers (75b, 75c, nanoCoP's calculi from the papers) with checked read-backs. No free-variable tableau, as a method or as a view, was built; no report says whether the plan counts the connection provers as taking the whole item.
- `resolution-4` (the TPTP `cnf` reader and SZS output). Partly done by 78a: `cnf` read and `prove --logic --szs` writes an SZS status line. Its note's second half, a TSTP derivation written and a GDV-style structural check of one, no report touches.
- `interop-5` (TPTP conventions in and out). Partly done: part (a)'s status line for ordinary logic (78a), without the proof between `SZS output start` and `end`, and not for the linear suite. Part (b), TFF/THF input, and part (c), a logic specification for linear logic in TPTP's non-classical syntax (the author's outward act, and engines-11), no report touches.

### Closed meanwhile (taken or done by a session it was not assigned to)

For the next step's reading; `later.md` says so for most, as noted.

- KD4's cost: fixup-10. `later.md` replaced the entry with its cost.
- Seriality with one new label: 54x7; `later.md`'s entry removed on 2026-11-20.
- Auto-reverse for the ordinary suite: 54x7 and 54x8; `later.md` narrowed to rule files.
- The renderers' module: fixup-11; `later.md` says so.
- The first-order table with binders alone, LJ's unary `¬L` certificate, the warnings without `parallel`: fixup-12; out of `later.md`.
- The lambda fuzz targets' first run: 54x9.
- LJT's leftovers ("limit of 0 B", the verdict line, `Derivation::term`, `check` reading LK and LJ): fixup-18; out of `later.md`.
- The pigeonhole entry's first way and `comparison.sh`'s ILTP pass: fixup-19; LJT's preference within a bound: fixup-20; JSON for an unknown that a bound ends after the search: fixup-21; tests in every pair of features: fixup-22 (the last three added to `later.md` and closed again).
- Follow-ups of steps 74 to 78 closed by fixups: TPTP names refused in 130 problems, `check --tree never`, `include` in the command (fixup-13, from 74c and 74d); the quadratic binding walk and the typed witness held to the bound on terms (fixup-14, from 74e's panel and reviewer); shared terms walked as trees in the connection provers (fixup-15, from 75c); `ileancop_main.pl`'s licence and TPTP files not in UTF-8 (fixup-16, fixup-17, from 75d); the connection prover bounded in the library's default and the batch's first-order race (fixup-23, from 76a and 76b); Herbrand's read-back within the memory bound and the shift's spelling in rocq-deduxit (fixup-24, from 78b and 77c).
- Follow-ups closed within the steps: the open witness (74c's departure, built by 74e); the command's default losing six theorems to the prover (75b, answered by 75d's 90 % share); 78a's linear `=`, grounding's class test without a stop (R4) and `grade` with `=` (all by 78c).

### Could not place

- `logics-9` and `web-teaching-9`: as in 73.
- The parser reading `a⊥`, and whether the flake's `ordinary-problems` check holds the page's examples: as in 73, no report touches them.
- 75a's "the ground case is wasted" (a ground first-order sequent refuted by the image's assignment): 75d's grounding decides the Bernays–Schönfinkel class, which holds every ground sequent, and is the default for its classes; no report says the item is closed by it.
- `tableaux-3`'s free-variable half (above).
- Rocq certificates of valid first-order derivations: 74a's design (`notes/first-order-design.md`, section 8) decided them as terms over `Prop`; the writer refuses a first-order derivation (`core/src/ordinary/rocq.rs`, "a certificate over Prop"), and no report of 74 to 78 records them as built, deferred or dropped. `docs/src/status.md`'s planned 0.8.0 item said "with finite countermodels and Rocq certificates"; step 80 (`mlqtuqru`) lists them as not yet placed in a release.
- Whether this machine reads the Cachix cache `deduxit`: no report says.

## Added by steps 74 to 78 and the fixups

Every follow-up those reports record as not fixed, open, left or deferred, not those fixed within the step or by a later fixup (those are under "Closed meanwhile"). "In later.md" marks the three that `later.md` holds; every other line is recorded only in a report.

### Verification

- **Rocq certificates of first-order derivations of LK, LJ and LM** (74a design, 74b and 74d left the writer's refusal). Waits for a session; no report names it a follow-up (see "Could not place"). Report and design only.
- **First-order inputs and fuzzing**: no fuzz target has changed since 2026-11-20 (54x9), and no report of 74 to 78 says whether `ordinary_text`, `tptp` and the `json_*` targets reach the first-order forms (TPTP `fof` and `cnf` with renaming and `include`, wire levels 25 to 29). Waits for the next fuzz run after `fuzz/seed.sh`. No report.
- **The propositional Kripke certificate still states the cut-free calculus** (76c), while the first-order certificates state the calculi with cut: one line and a snapshot bless, a released form, so the author's word. Report only.

### Engines at first order

- **A native equality rule in the connection prover** (78c). Not built; revisit once deduxit's prover closes the gap with the axioms (leanCoP 134, nanoCoP 115, deduxit 88 on 433 problems). In later.md.
- **The intuitionistic connection prover raced beside the embedding for sequents with `=`** (78c). A share in the library gained 36 and lost 28; a race in the command, then ILTP's 1 730 before and after. In later.md.
- **The modal walk in depth: rows it slows** (fixup-10). For when a user meets one. In later.md.
- **The connection provers' strategies** (75b, 75c, 75d, 78c): no restricted-backtracking schedule or portfolio, so the provers' unknowns at the time limit dominate (667 intuitionistic rows at 2 s, 75c) and Otten's provers prove more with equality. Waits for a performance session. Report only.
- **The intuitionistic connection prover stays out of the default without `=`** (75d): every order or share lost theorems; a race would add at most the two the embedding misses. Waits for a session that wants those two. Report only.
- **No connection prover for minimal logic** (75c): the rewrite's map back is ambiguous between `¬A` and `A → ⊥`, and the differential test was not run. Report only.
- **Constants ignore term variables in the shared term code** (75c): loses proofs, never soundness. Report only.
- **The library's copy bound of 3 on intuitionistic images** (75a, 77b): `decide` with default options answers unknown on small sequents (`∀x. (p(x) ↔ q(x)), q(a) ⊢ p(a)`), and `compare` at first order needs it lifted; the command lifts it. Report only.
- **The connection prover's bound counts steps, not work** (fixup-23, from 76a and 78a): the irreflexive transitive serial sequent still takes 102 s to give way; a bound counting the unifier's work is a change of the prover's measure. Left open. Report only.
- **`--method connection`'s matrix build asks no stop** (78c's reviewer): a congruence of 3 000 arguments ran 41 s and 2.6 GB at a 2 s limit. A follow-up. Report only.
- **The intuitionistic prover finds no proof of R2 at any size, and one instance's build no poll reaches** (fixup-15): not looked into. Report only.
- **77a's clausal normal form overruns its bounds on many hypotheses** (fixup-24: 1 500 hypotheses reached 54 GB). Taken by fixup-25, running, no report yet. Report only.
- **The model search's encoding** (76a): Paradox's flattening for wide terms, incrementality across sizes, symmetry breaking on function values. Measure on harder sets first. Report only.
- **Kripke models with `=`** (78a): the model search tries none; the usual semantics, a congruence growing along the order, is a follow-up. Report only.
- **Herbrand's theorem answers unknown for a sequent that is not valid** (77b): a checked Herbrand model over a finite universe would be a small addition. Report only.
- **A saturated first-order clause set answers unknown** (78b's accepted departure: no model built), and a refutation with a rewrite has no LK read-back (78c). Report only.
- **`bare_term`'s scan of binder names is quadratic in a clause's variables** (78a, R3). Report only.
- **The definitional forms refuse first order** (77a): `cnf`, `dnf`, `tseitin` and `plaisted-greenbaum`; the clausal form is the first-order CNF, and definitional forms over free variables wait for a caller. Report only.
- **Skolem's promise in its Extends direction has no derivation** (77a): a finite model's expansion by the Skolem symbols could be computed and checked. Report only.

### Sessions, the command and the batch

- **The goal state's `close` and `hint` refuse first-order goals** (75a, 78a): a door in `Interactive::decide`; a goal with open witnesses needs the linear session's hand-over treatment and a graft that binds them (74e). Waits for a session. Report only.
- **The batch writes no countermodel's drawing or certificate** (76c), of either kind, as before. Report only.
- **The tableau method still refuses past `--derivation-limit`** (fixup-19): an exception the texts now name, since the tableau is built from that derivation. Report only.
- **The embedding's read-back keeps its refusal at a late bound** (fixup-21's accepted departure from the planning session's decision), and a table or tableau a bound ends while it is built stays an error. Report only.

### The web client and step 79

- **The page has no `connection` or `grounding` call** (75b, 75d): `web/src/calls.rs`'s table and the client are the web front end's. Step 79's. Report only.
- **fixup-11 left two measures**: a throttled timing in a real browser profile with the service worker, and a measure on a phone. Report only.
- **The module's ceiling rose five times for first order** (2 500 000 to 2 875 000 bytes at gzip, fixup-14 to 78b); step 79's page will press on it. Report only.

### The author's own

- **rocq-deduxit's tag before deduxit's 0.8.0** (76c): its first release since v0.6.0; step 80 prepares it, the tag is the author's. Report and step 80's prompt.
- **Decisions taken unattended since 0.7.0, for the author's review** (status log): 74a on Opus in place of Fable; TPTP names renamed on reading (74d, built by fixup-13); the package `deduxit-web` ships two modules (fixup-11); `ileancop_main.pl` under GPL-3.0-or-later (fixup-16); the public `ordinary::tptp_text` (fixup-17, its shape a departure); the pigeonhole entry's first way (fixup-18 and fixup-19); a bound that ends a read-back or check after the search is an unknown (fixup-21). Report and status log only.
- **74a's certificates as terms over `Prop` with no library module** (a departure from the design), which bears on the missing first-order derivation certificates above. Report only.
- **`status.md`'s planned 0.8.0 item named the web page** ("all of it on the web page"), which moved after 0.8.0. Corrected by step 80 (`mlqtuqru`).

### The rest

- **TSTP derivation output and a GDV-style check** (resolution-4's second half). No report touches it.
- **interop-5's parts (b) and (c)**: TFF/THF input; a linear-logic specification for the TPTP World. No report touches them.
- **Free-variable tableaux** (tableaux-3's other half). No report touches it.

## Step 79

Step 79 is "First order on the page: the ordinary tab and the linear tab at first order, witnesses and eigenvariables chosen as the linear page chooses them, countermodels drawn, the transformations shown, Learn's and Practice's lessons on quantifiers" (two to three client sessions, Opus 5.5 at high, a fresh reviewer per session; depends on 63, 76 and 77).

Its prompt is not written: there is no `plan/79-*.md`. The step table still says "written at the review of 78", but the review of 78 wrote step 80's prompt instead. On 2026-11-22 the author froze the website until 0.8.0 ("I don't like the UI, it's not easy to use anymore, so I will change it after 0.8.0 some time"), so step 79 moved after 0.8.0, to be planned with the author's redesign, and step 80 no longer depends on it. It waits for 0.8.0's release and for the author's redesign.

What the reports of 74 to 78 leave for it:

- **The bindings already take first order in part**: the ordinary step takes `"witness"` as text (74d) and a step with no witness for an open one (74e); the protocol runs first-order values with no new kind or operation (74d), and its `ordinary.decide` lists the `connection` and `grounding` methods, which `web/src/calls.rs`'s table of the page's methods does not (75b, 75d).
- **What the page would show**: first-order derivations in SVG (74d pinned LaTeX, Typst and SVG), finite structures and Kripke models with growing domains drawn (76c: in SVG a structure is one box like a world; in LaTeX and Typst a Kripke model is a table, a row per world), the normal forms step by step (77a's five protocol operations), Herbrand's instances (77b), first-order resolution with paramodulation (78b, 78c), Glivenko's refusal at first order with its infinite model (77c), and equality.
- **The client's lock** moved last with fixup-11's push (deduxit `yunvyupu`, 2026-11-21), before steps 75 to 78; the first lock bump after the freeze brings their code and wire levels 26 to 29, with fixup-11's second module already in place.
- **The module's size**: 2 846 308 bytes at gzip under a ceiling of 2 875 000 (78a's figure). The page's first-order calls will add to it.
- **The client's own leftovers** wait with it: the eight entries for the author to revisit, the judgement calls of step 38's sweep, the Biome override, 70c's resolution departures and 68's pack tableau problem, now inside the redesign.
