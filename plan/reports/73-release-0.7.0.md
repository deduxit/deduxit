# Step 73: the release of 0.7.0, prepared

Opus 5.5 at `high`, the worker of 70a to 71b keeping its conversation,
started by `deduxit-orchestrator` from `/tmp/linlog-handoff/start-73.md`
on 2026-11-20, cores 7 to 9, in the jj workspace
`../deduxit-wt-step-58a-610ed3`, begun on `main@origin` `zpzqprxk`
("Plan: 73 started") and rebased at the end onto `tlwompqv` ("Plan:
step 71 complete", above 71c). rocq-deduxit is not released (below), so no
second repository.

## Outcome

0.7.0 is ready to tag in deduxit, up to the author's acts. The release
is dated 2026-11-20. The tag is held for the author: the unattended yes
of 2026-11-20 covered 0.6.0 alone.

- **No rocq-deduxit release.** The library has not changed since its
  v0.6.0 but for a comment (`twvrwvrk`); no theorem moved, so deduxit
  0.7.0's certificates compile against rocq-deduxit 0.6.0, which
  `export::rocq::DEDUXIT` keeps naming. The Rocq chapter says so, and
  the guide's tag line names `v0.6.0` for deduxit 0.6.0 and 0.7.0.
- **deduxit 0.7.0**: the chain below. The release notes, what a user
  can do now, the order of the tag and the publishing and the author's
  checklist are in `plan/notes/release-0.7.0.md`.
- **71c, folded in.** It landed on `main` after the chain was ready
  (`wwwpmwzl..twpnvsto`, then the planning session's `lwqryurw`), with
  two entries under `[Unreleased]`: the SVG subscripts, which the rebase
  put into 0.7.0's "Changed" by itself, and the typing derivation's
  drawing, which conflicted with the dated section and is now in 0.7.0's
  "Added", where 71c had put it, with the protocol's `lambda.draw` and
  `lambda.export` named beside it. Its bindings' lambda calls, which its
  changelog did not name, are under the web bindings' entry; the
  section's first paragraph names the drawing and the page's lambda
  part. The status chapter names the bindings' and the page's lambda
  part (71c had written its line on the drawing); the notes give the
  module's size with 71c, 3 846 685 bytes at gzip. The lists of types
  are unchanged by it (19 closed, 217 marked: `lambda::Drawing` has no
  public field, `Drawable::Lambda` and `rocq::Unsupported::Typing` are
  variants of marked enums, the latter last); no new error code.

## The break: `breaks 0.6.0`, and how each line is covered

`breaks 0.6.0` (cores 7 to 9, at `zpzqprxk`, before the version raise):

- cargo-semver-checks: 229 checks, 227 pass, 2 fail, 31 skip, "semver
  requires new major version":
  - `derive_trait_impl_removed`: `lambda::Variable` no longer derives
    `Copy`;
  - `inherent_method_missing`: `Variable::letter` and `Variable::index`.
- cargo-public-api: Removed those three (`index`, `letter`, `impl Copy
  for Variable`); Changed one, `lambda::Term::serialize`, whose
  signature is the same (a derived impl became a hand-written one, the
  generic parameter renamed from `__S` to `S`), so no caller breaks; 982
  lines Added (`ordinary::normal`, `ordinary::resolution`, the lambda
  parser, checker and inhabitation, `Limits::form_occurrences`,
  `Refusal::Form`, the new errors and the rest of steps 70 to 72).

**How each line is covered.** The three removed items are 71a's
renaming of a variable's letter and index to a name, a 0.y break, which
0.7.0's minor raise allows: the changelog's "What breaks, and what to do
instead" names them and says what to use (`Variable::name`, a clone or
the `VarId`), with the JSON form's change beside. The "Changed" line is
none. Nothing else breaks: every new variant or field belongs to a
`#[non_exhaustive]` type.

**The error codes**: 0.6.0's 80 in their order, then 5 new
(`form_too_large`, `invalid_clause_refutation`, `clause_file`,
`ill_typed`, `not_enumerable`), compared with `v0.6.0`'s
`Error::CODES`.

**The two lists of types**, against the head's rustdoc JSON with every
feature, by a script matching each public enum and struct with a public
field by its defining path: 19 closed enums, exactly `core.md`'s list
(no new closed enum since 0.6.0), and 217 `#[non_exhaustive]` types (196
at 0.6.0, and 21 new: `ordinary::normal`'s eight, `ordinary::resolution`'s
nine, `lambda`'s four), each named in `core.md`'s list. The four marked
structs without a public field are as at 0.6.0. `core.md` gives the
counts at 0.7.0 (`ywosuvwr`).

## What was prepared

- **The version** (`qrkqxmol`) is 0.7.0 in the manifests and both
  locks (`cargo metadata`; `cargo update --offline -p deduxit` in
  `fuzz/`), `CITATION.cff`, the dependency lines of README and the
  library chapter, the installation chapter's file names, the JSON
  examples of the docs (`wire.rs`, `ordinary/decide.rs`, `negative.rs`,
  `interpolant.rs`, `normal.rs`, `resolution/mod.rs`) and the manual
  (`terms.md`, `ordinary-logic.md`), the behaviour lock's inputs
  (`cli/tests/lock/inputs/workspace.json`, `MODAL_OUTCOME`), the Rocq
  chapter's sentence on which library 0.7.0's certificates compile
  against, the guide's tag line and the dataset card's "which comes with
  0.7.0". Not changed: curryst's 0.6.0 (`export::typst::CURRYST` and the
  Typst snapshots), batsat's, `DEDUXIT`'s, and the dataset's Croissant
  record, which says truly that deduxit 0.6.0 built it.
- **The bless** (`rotwzrkp`): the JSON lock's 18 outcomes, the serialize
  test's five, and the three modal `check` outputs of the command's lock
  change by the version string alone (checked: the 26 lines removed and
  the 26 added are equal with the version masked).
- **Levels 22 and 23 kept** (`tzyuvrop`): no file under
  `core/tests/wire/` held what this release writes at its new levels.
  `level-22.txt` holds two refutations (saturation, DPLL) and
  `level-23.txt` four terms (linear, affine, simple with sums, simple
  from the enumeration), each written by this build's command; the
  stability test reads each back to its bytes and checks it (the
  refutation's `check`, the term's `Term::check`). Levels 21 and 24 are
  written only.
- **The status chapter** (`kroqurtp`): the command's list names
  `resolve`, `grade` and `term`; `grade` and the dataset have bullets;
  the bindings' and the page's normal forms and resolution; the page's
  switch of invertible rules in LK and LJ tabs (54x8); the 0.7.0 bullet
  of "Planned" removed; after the fold, the bindings' and the page's
  lambda part (71c rewrote the line on the typing derivation itself).
- **The descriptions** (`qkzlynkw`): README's opening, `CITATION.cff`'s
  abstract and keywords (`resolution`, `lambda calculus`), both crates'
  descriptions and the crate docs' "What you can do" name normal forms,
  resolution, lambda-terms and grading.
- **The changelog** (`lqusopsm`, with 71c folded in): `## [0.7.0] - 2026-11-20` under an
  empty `## [Unreleased]`, a first paragraph of what 0.7.0 brings (the
  page's part among it), "What breaks" (the variable's name, with what
  to do), "Changed" (the `--lambda-types` subscripts, the `version` key
  of `--lambda`'s value, the protocol's fifth suite, the help), the
  steps' "Added" grouped by theme, "Wire forms" for levels 21 to 24 and
  `wire::LEVEL` 24, the link `[0.7.0]`. No "Fixed": every fault the
  reports fixed was in code new in 0.7.0.
- **The rules** (`ywosuvwr`): `core.md`'s counts at 0.7.0.
- **The guide** is within its budget: the summary is unchanged since
  70a (at 7 999 of 8 000 bytes); `reference.md` gained 71b's `term`
  section and the tag line; its examples test passed.
- **The release workflow's dry run**, as far as one runs locally:
  - its tag check, run as the job's script with `TAG=v0.7.0`: "passed"
    (the manifests' 0.7.0, the changelog's dated section, `CITATION.cff`);
  - its crate selection against crates.io's API: `linlog 0.3.0` and
    `linlog-cli 0.3.0` "on crates.io already", `new=` (none),
    `existing=deduxit deduxit-cli`, so the job publishes both through
    Trusted Publishing and uses no secret;
  - its `cargo package --locked --package=deduxit --package=deduxit-cli`:
    exit 0, both packaged (491 and 255 files) and verified at 0.7.0,
    from a clean tree;
  - **the new path** (ci-speed): `release.yml`'s `checks` job now calls
    `checks.yml` on the tag's commit instead of running `nix flake
    check`, which fetches from the cache what `main`'s run pushed, and
    `packages` builds beside it; `publish` waits for `tag` and
    `checks`, `release` for all three. This is the path's first
    release; locally nothing more of it runs. `install.yml` after the
    release now installs the release's own files with the job's token:
    the notes ask the planning session to watch its first run.
  - `cargo publish --dry-run` is denied to this session, as before.
- **The release notes** (`wkptyqzr`) are the changelog's section as the
  workflow's `awk` writes it, generated into
  `plan/notes/release-0.7.0.md`, with what a user can do now and the
  checklist.

## The checks

Every build and test ran in a capped scope on cores 7 to 9
(`CARGO_BUILD_JOBS=3`, `RUST_TEST_THREADS=3`); every nix build under
`flock -o /tmp/linlog-machine.lock` with `--max-jobs 4 --cores 8`.

- **`breaks 0.6.0`** and the lists of types: above.
- **`cargo test --workspace`** after the version raise, before the
  bless: only `json_lock`, `outcome_json_format` and the command's
  `behaviour_lock` failed, on the version alone, as at 0.6.0; every other
  test binary passed. After the bless, the locks pass.
- **The stability test** with levels 22 and 23: 17 tests passed.
- **`cargo package`** and the release's tag check and crate selection:
  above.
- **`nix flake check --keep-going`** at the release head (`wkptyqzr`,
  before the reviewer's one-line fix to the installation chapter):
  "all checks passed!", the gate's five parts among them.
- **After the fold of 71c and the reviewer's fix**, at the rebased head
  on `main@origin` `lwqryurw`: the lists of types again (above), and
  `nix flake check --keep-going`: "all checks passed!". The chain was
  then rebased onto `tlwompqv` ("Plan: step 71 complete"), which
  changes `plan/README.md` alone, so no check reads it.

## The reviewer

`fresh-reviewer` (Fable 5.2 at medium) read the chain against the
brief before 71c landed. One finding, fixed in the commit it concerns:

1. `docs/src/installation.md:238` still told a reader to depend on
   `deduxit = "0.6"`, while README and the library chapter said `"0.7"`;
   the releases before moved that line too, and no test reads it. Now
   `"0.7"`, in the version raise (`qrkqxmol`).

It also noted that the notes' last section differed from the workflow's
by blank lines; it is now the workflow's bytes (the `awk`'s output, then
the `echo`'d blank line and the install line), in `wkptyqzr`.

It confirmed the rest: every remaining 0.6.0 names another thing
(curryst, batsat, rocq-deduxit, history, the dataset's maker); the bless
pairs line for line with the version masked; the changelog's form and
every name it cites, "What breaks" against `breaks`, every locked
output changed since `v0.6.0` covered; the 21 new marked types; the
status chapter; the notes' example commands run as stated; the stability
tests can fail; the readme, lock, stability and serialize tests pass.

## Departures

- **Levels 22 and 23 got their stability files in the release**
  (departs from the design): the steps that made the levels wrote none;
  the release adds them, with this build's documents.
- **No rocq-deduxit release**, as the prompt foresaw ("no Rocq release
  unless a theorem changed").
- README's opening, `CITATION.cff`'s abstract and keywords and both
  crates' descriptions gained 0.7.0's suites; the brief names README for
  installation only (departs from the design, as at 0.6.0).

## What is left open

- The tag and the publishing, in the order of
  `plan/notes/release-0.7.0.md`, with the author's checklist there.
- The web client moves its `deduxit` input to the tagged commit after
  the release.
- The fuzz targets of 71b (`lambda_text`, `lambda_inhabit`) build but
  have not run.

## What the next sessions must know

- The manual's examples mask `"deduxit": "…"`, so a version left behind
  passes every test; this release grepped `docs/`, the doc comments and
  `skills/` for 0.6.0 and kept only the other projects' 0.6.0.
- A new wire level that a reader reads back gets its
  `core/tests/wire/level-N.txt` in the step that makes it, as
  `core-sequents.md` says; 70b and 71a did not, and this release did.
- `later.md` stops at 2026-11-16: the follow-ups of steps 63 to 72 are
  in their reports alone (the survey below lists them).

## The commits

On `main@origin` `tlwompqv`, oldest first:

- `qrkqxmol` Raise the version to 0.7.0
- `rotwzrkp` Bless the behaviour lock: an outcome states version 0.7.0
- `tzyuvrop` Keep what 0.7.0 writes at levels 22 and 23
- `kroqurtp` Keep the status chapter true for 0.7.0
- `qkzlynkw` Describe 0.7.0's normal forms, resolution, lambda-terms and
  grading
- `lqusopsm` Date 0.7.0's section of the changelog, with what breaks and
  its wire levels
- `ywosuvwr` Rules: count the marked types at 0.7.0
- `wkptyqzr` Plan: the release notes of 0.7.0 and the author's checklist
- (this report) Report 73: the release of 0.7.0, prepared

## The state of what later.md keeps for after 0.7.0

Read on 2026-11-20 at `main` `zpzqprxk` ("Plan: 73 started"), with `plan/later.md` whole, `plan/README.md` (D24 to D32, the step table and status log) and the reports of steps 47 to 72. Step 71c (the page) was still running. Steps 74 to 80 (0.8.0) are written but not started.

How to read it. An item is one candidate id of step 46's ranked list, or one entry of a later.md list or section. A line may hold several ids. "Waits for" says what it needs before work starts. "Taken" means a step closed it, with the report that shows it. `later.md` ends at the entry of 2026-11-16 (67a); the sections of 2026-11-17 to 2026-11-20 it was meant to gain were never written, so the follow-ups of steps 63 to 72 that only a report records are listed under "Recorded only in reports" in each group, and the planning session may want to copy them into `later.md`.

### Counts

| group | ranked candidates | entries | what it holds |
|---|--:|--:|---|
| Research items (D30 and the rest) | 37 | 0 | cyclic proofs, System F, deep inference, proof theory, logics beyond the suites |
| Deferred verification (D31) | 9 | 6 | deductive verification, a second checker, certificates still missing |
| Engine and performance follow-ups | 14 | 21 | measured-only lists, the dedicated-tools comparison, KD4, SAT tier |
| The web client | 14 | 10 | the learner's record, provability light, module size, judgement calls |
| The author's own | 8 | 12 | Zenodo, nixpkgs, MCP, opam, LLTP header report, install channels |
| The rest | 47 | 3 | applications, interop, types, modal and natural-deduction extensions |
| Assigned to steps 74 to 78 (planned, not done) | 6 | 0 | first-order ordinary logic, equality, resolution at first order, TPTP |
| Total | 135 | 52 | |

Of the 135 ranked candidates, 123 are rows marked "kept", six are half-kept (verification-2's third part, engines-7's environment, education-3's rule-file half, web-teaching-4's review, applications-4's further targets, structure-5) and six are assigned to the planned steps. Dropped, not listed: 12 ranked candidates (name-and-organisation-1 and -2, structure-3 and -4, verification-7, proof-theory-2, tableaux-5, more-logics-8, -9, -12, -13, -14), step 46's own dropped row (nine kinds of project), and one entry (nets on the command line, dropped with MALL nets).

No kept ranked candidate was closed by a step it was not assigned to. Several small entries of `later.md`'s lists were closed by the 54x follow-up sessions, which no table row names; they are under "Closed meanwhile" at the end. No report names a candidate id, so a match is by topic. Where no report touches a topic, I say so or give "still open" with the wait. One thing I could not place: see "Could not place" at the end.

### Research items

D30 (decision 7): the author starts these after the decidable suites. None of the six has been started; no report mentions them.

- `cyclic-proofs-1`: a checker for the trace condition on cyclic pre-proofs (the kernel of the area). Still open. Waits for the author's go; needs a graph proof type.
- `cyclic-proofs-3`: μMALL, finitary and circular (logics-5 restated). Still open. Research; no tool to compare with; μMALL first if any (D30).
- `cyclic-proofs-2`: cyclic LK with inductive definitions (CLKIDω), check and step by step. Still open. After cyclic-proofs-1.
- `cyclic-proofs-4`: Cyclist-style bounded search with back-links for one fragment. Still open. After cyclic-proofs-1.
- `cyclic-proofs-5`: Rocq certificates for the trace condition. Still open. After cyclic-proofs-1; also belongs to D31.
- `lambda-calculi-6`: System F as checking only. Still open. D30; step 71 (simply typed, done in 71a and 71b) is its prerequisite and now exists. Inference and inhabitation stay out (undecidable).

The other research notes, each still open and each waiting for the author's choice or for research:

- `logics-1`, `logics-2`, `logics-6` (exponential half): exponentials by signature (SELL), graded and bounded exponentials, relevant logic LR as a mode. After 0.7.0, the author's choice; every exponential case of cut elimination and the Rocq mirror per label are the cost. LR proper has no tool or audience seen.
- `logics-3`: ordered cut, then Lambek with modalities. Research.
- `logics-4`: deep inference (BV, MAV, pomset nets). Research project.
- `logics-5`: fixed points beyond μMALL. Research.
- `logics-7`: light logics. Small; no step asks.
- `logics-8`: adjoint logic's functional reading. After logics-1.
- `logics-9`: further items, each read at abstract level only. Per item.
- `proof-theory-9`: explaining unprovability (a failing slice, missing resources, the stuck leaf drawn). After 0.7.0, the author's choice. Step 54w wrote an "Explanation" chapter on why a sequent is unprovable (`reports/54w-text.md`), which is text, not the feature.
- `proof-theory-1`, `-12`, `-13`: proof equivalence and canonical proofs; interpolants by cut introduction; MALL type isomorphisms. Research. (Step 63t built Craig interpolants for LK and LJ by Maehara's method, which is not proof-theory-12.)
- `proof-theory-3`: unification nets for first-order MLL. Needs a first-order net data model.
- `proof-theory-4`: subformula linking in the interactive session. Small; the step that next changes the session may take it.
- `proof-theory-5`, `-6`: calculus of structures and BV; proof compression. Research.
- `proof-theory-7`: a token machine on nets. Linear-specific; no step asks.
- `proof-theory-8`: a dialogue (game) mode. Game rules per logic to source.
- `proof-theory-10`: combinatorial proofs for the ordinary layer. After the suites it would draw.
- `custom-rules-3`: bounded search under user rules. D28: no search until asked. Never "unprovable".
- `custom-rules-4`, `-5`: static analysis of rule sets; synthetic rules from axioms. Research; mostly "unknown".
- `semantics-decision-2`, `-3`: Presburger invariants, ideal Karp-Miller. Measured only.
- `semantics-decision-7`, `-8`: finite phase countermodels; Petri nets with data. Research.
- `engines-2`, `-3`, `-5`, `-6`: lazy SAT over linkings for MLL; integer and trap refutations through a solver; learned dispatch; learned choice ordering. Research; hand rows first for engines-5 (step 52 did them).

### Deferred verification (D31)

D31: first the refutations (step 51, done), then Kani for the checker's integers (step 58, done; 11 facts), then deductive verification once the checker is refactored for it.

- `verification-4`: deductive verification of the Rust checker, the tool chosen by a trial in Verus and Creusot. Still open. Waits for the checker refactor and the author's go. Step 58a's Kani facts are the specifications to start from, and it lists what Kani does not reach (zones of two members, whole-pass facts). Prusti does not fit.
- `verification-5`: refinement of the Rust checker to the Rocq checker (Aeneas or hax). Still open. As verification-4; the translators' Rocq back ends are least mature.
- `verification-1`: a second, differently shaped checker, fuzzed against the first. Still open. Removes nothing from the trusted base.
- `verification-3`: a checker extracted from Rocq, run in the command and the browser. Still open. Only if a user without Rocq needs a verified verdict.
- `verification-2` part (iii) with `semantics-decision-10`: Rocq certificates for saturated refutations, which need focusing completeness in Rocq. Still open. A project of its own. Step 51 closed (i) and (ii) (`reports/51-certified-refutations.md`); a saturated set still has a check and no theorem (`Unsupported::Saturated`).
- `semantics-decision-12`: failure certificates of the backward search. Still open. `Refutation::Exhausted` has no invariant (`Unsupported::Exhausted`).
- `proof-theory-14`: a certified net criterion (sequentialization) in the Rocq library. Still open. In the library's repository; the sequentialization theorem is the hard part.
- `lambda-calculi-7`: a Rocq checker for typing derivations. Still open. Step 71 built the Rust checker and the typing derivation (71a, 71c drawing); no report touches the Rocq side.

Entries:

- **The basis's Rocq lemma** (semantics-decision-1's second half, "Follow-ups: certified refutations"). Still open. Step 57 built the checked basis (`search/basis.rs`) and left `--kernel deduxit` refusing it (`Unsupported::Basis`). The lemma needs the converse of the Horn induction, a net reading of the goal, and a decision procedure for `vm_compute` (`reports/57-basis-certificate.md`).
- **Second certificate kernels' leftovers**: an unfinished proof with open goals as hypotheses; NanoYalla's `ex_perm_r` cost and identifier escaping. Kept, no user asked.
- **Unrun fuzz targets**: `lambda_text` and `lambda_inhabit` (71b) build and have not run; the next fuzz run takes them after `fuzz/seed.sh`. The modal targets ran to the cap with coverage still growing (54x7). Recorded only in reports.
- **First-order table with binders and no atoms** (`Sequent::join_first_order`, found by 54w-e1). Still open, unconfirmed; the test (build the table from JSON, join a formula) was never written. `reports/54w-text.md` line 1019 repeats it.
- **The Rocq writer and LJ's unary `¬L`** (found by 49a's reviewer): the checker accepts it and the writer would write an ill-typed term; nothing reaches it today. Still open.
- **Kani's weekly run and the unreached facts** (`kani.yml` on `main` is the only check since 2026-11-15; 58a lists what the harnesses do not reach). Recorded only in reports; watch, not an action.

### Engine and performance follow-ups

All of these wait for a measurement that asks (D19), a timed run the author gives, or a user. No step of the roadmap to 0.7.0 changed the focused, Horn, net or parallel engines (step 57 added a certificate to the Horn engine and no search change; step 52 added dispatch rows to the inverse method).

- `engines-13`, `engines-14`: a depth that does not grow with the formula, for LinearOne's eight problems; faster copy-bound deepening. Measured only; a performance step the author chooses.
- `engines-4`: encodings as exports (DIMACS, SMT-LIB, ASP, QBF). Kept with engines-1 (done in 54s). Step 70b reads DIMACS but writes none.
- `engines-9`: search state kept across calls. No client asks.
- `engines-10`, `-11`: competition inputs for the Horn engine (STRIPS, PNML); linear logic in the TPTP World. Need interop-5, which step 78 plans.
- `engines-15`: the inverse method at first order. Saturation does not terminate.
- `engines-16`: a first-order comparison library. None exists.
- `engines-17`: a generic focused engine over rule schemas. D28: no search under custom rules.
- `semantics-decision-4`, `-5`, `-6`, `-9`: directed reachability; a PNML front door; affine saturation with subsumption (partly built in the affine inverse method); Horn shapes beyond `!`-Horn. Speed only, or no step asks, or the gain is uncounted.
- `engines-7` (the environment half): a proof-search environment for learning. The dataset half is done by step 72; the environment is the MCP server's work seen by another consumer.

Entries:

- **The focused engine's whole follow-up list** (the gap study's probes A2 to A4, the fixed cost per call, the memo leftovers, `Θ`'s sort, the hand-over at the recursion limit, Mix's costs, the default bias's unit of work, deepening from the root, five LLTP problems at copy bound 3, the pool's arena and memory, the checker on a hostile file, the forward search's stop on GPPP nets, the one-sign count's tightening). Kept; each waits for a measurement. 54x4 closed two (the tableau's stop, the large image's first poll).
- **The Horn engine's list** (66 undecided nets, refutations no engine reaches, the simplex's limits, memory at tight bounds, the shares, forward coverability, the frontier's duplicates, a shortest firing sequence). Kept.
- **Parallel search's list** (late cancellation at `&`, the short cube list, speculative work, the per-worker arena, a runtime kept across calls, duplicated exploration, a thread sanitizer). Kept; no measurement asks.
- **The net engine's list** (test period, partner counts per literal, bridges for sub-nets; a conditional symmetry break for the planar search; the quadratic `choose`). Kept; the vertex-bound panic is done (53a).
- **The inverse method's leftovers** (Mix, Petri nets, intuitionistic libraries) after step 52's dispatch rows. Kept.
- **Benchmarks, found by comparisons** (`chain-fo` without the family's copy bound, `wide-m1`'s memory at 8 192, `pigeonhole` #1's late classical refutation, the resumption switch at first order). Kept with engines-13 and -14.
- **A turn bounded in time** (the case of `chain`, the classical readings and `pigeonhole`; found by step 52, unassigned). Step 52 dropped the turn because it lost no row the default decides; still open.
- **Benchmark leftovers** (`bench/reruns.txt`, the forward bound tuned on more nets, step 28's older lists). Kept; measured only.
- **A search that can be suspended** (an explicit stack in the focused engine). Kept; no measurement under wasm asked for it.
- **Deferred beyond the 2026-10-03 table**: per-worker proof arena, duplicated `&` exploration, thread sanitizer; sharing among interchangeable sequents, split order, `&` as intersection; the net engine on a sub-forest. Kept, as above.
- **Coverability against the dedicated tools** (2026-11-09): ICover, BFC and FastForward join the comparison first, then QCover's pruning inside `cover.rs` if they win. Open. Waits for a night the author gives; the search must not change (step 57's rule).
- **A native SAT tier** (CaDiCaL behind a feature, 2026-11-09). Open. Step 54s decided batsat is enough for the classical default (254 of 274 ILTP at 2 s); no user has met the solver as the limit.
- **KD4's cost** (a cache of world signatures across branches, 65b). Open. Waits for a user who meets it; the other fourteen systems ended 630 150 decisions within 2^24 units.
- **Derivations of pigeonhole formulas pass the derivation limit** (SYJ202; 54s-c3) and **LJT's derivations outgrow its search**. Open. Two ways, the author's to choose: bound by inferences and keep `derivation_bytes` for what is drawn, or shared subderivations with a checker that checks each once (the panel's, a change to the checker). This is also `ordinary-sequents-4`'s DAG read-back, under "The rest" below.
- **LJT's other leftovers** (`interact --logic`'s `close` and `hint` decide through the embedding; the verdict line does not say LJT gave way; "limit of 0 B" at `--memory-limit 1MiB`; `Derivation::term` checks nothing). Open. `check` reading an ordinary outcome is the author's to ask for.
- **`comparison.sh` has no ILTP pass** (54s). Open; rerun with the baseline.
- **`cargo test -p deduxit` without `parallel` warns** on test-only items, and no check clippies without `parallel` (49a, 53). Open.
- **Interactive and exports leftovers**: a session's PNG or PDF render has no time limit; cut in a first-order session; two-sided rule names on reading a state; the ratchet's `derivation-chain-64` margin; the size estimate's shadowing case; Greek as mathematical italic and a smaller row height in the SVG; term against net normal forms only without doubled vertices. Kept, no user asked.
- **The "unknown" verdict's wording** (needs measurements on LLTP of which unknowns a larger limit ever decided) and **a batch's second pass inside the command**. Kept; the second waits for a user.
- **The written succedent of `⊤`/`0`-only formulas; the additive path on more roots** (intuitionistic mode). Kept; verdicts are unaffected (brute-forced over 607 464 cases).
- **The tableau's expansions' allocation** (one allocation per inference; 54x4 left 0.2 s on SYJ206+1.020). Recorded only in reports.

### The web client

- `web-front-end-4`: one wasm module per suite, loaded lazily. Kept, decided in step 63 against 47's figures: only when one of D29's triggers holds (a suite from another repository, or a measured size). Step 63g-c raised the ceiling to 3.9 MB; 71b has it at 3 831 827 bytes at gzip (68 KB under 3 900 000).
- `web-teaching-1`, `-6`, `education-11`: a learner's local record, spaced review, the record's views. After Learn by suite (63 built it).
- `web-teaching-3`, `education-1`, `engines-8`: a provability light on open goals. Waits for the page's D11 (provability on request only).
- `web-teaching-8`: a classroom view for a projector. No step asks.
- `web-teaching-10`: a second interface language. English only until asked.
- `education-2`: net exercises. Linear-specific, after the suites.
- `education-5`: generated exercises. Needs a confirmed measure of difficulty.
- `education-9`: resource puzzles. Need a Petri-net drawing.
- `web-teaching-4` (the remainder): a review by a screen-reader user, and a structural twin of long drawings. Step 58b did the audit and a read of Chromium's tree; no session can supply the review, which stays with the author. Also open from 58b: rule labels named by their step, and the label targets on a phone.
- `education-3` (the remainder): auto-reverse. Done for the linear suite (38h), the ordinary suite (54x7, page 54x8) and the packs (68d); for calculi given as data a rule file needs an optional `invertible` key per rule and a wire level (63a's note, "Auto-reverse" entry of `later.md`). Open.

Entries:

- **The web module's size after the workspace** (63g-c, 54x6). Open, the author's. 54x6 took back 129 KB. What is left: the protocol's dispatch and readers (about 1.35 MB raw; a trim needs a registry the bindings build from the page's list, an API change), and a separate module for the PNG and PDF renderers (27 % of the module). 70c left 8.6 KB; 71b raised the ceiling to 3 900 000. 71c and the first-order page (79) will press on it.
- **The bindings' leftovers**: a turn's set-up kept across restarts (costs 19.4 times on one problem without threads); the first steps of a goal's proof, `Step` and `Applicable` as wire forms; a call over `search::engine_for`; the inverse method's turns rarely come in a tab (recursion limit 384). Kept; the page has not asked.
- **The parser reads no `a⊥`** (the library): a student typing what the drawings show gets an error. Unassigned; no report shows it closed.
- **Drop the client's Biome override** at the first client `flake.lock` update whose nixpkgs has a Biome build on cache.nixos.org (53n). Open; a check, not a feature.
- **Ordinary session's leftovers on the page** (49c): the graft's mark does not survive a link; the tour's Escape race; no settings row for `ordinary.translation`; the examples are checked by Vitest and by the flake's `ordinary-problems` check (63 moved them; not re-verified).
- **A pack's tableau problem and the check view reading the rules suite** (68): a pack problem could carry `"method": "tableau"`; natural deduction and tableau sessions need `rules_check` in the view. Open.
- **Resolution on the page** (70c): CNF only (no Tseitin or Plaisted and Greenbaum choice); sheets in the Logic popover, not views beside the derivation; the graph scrolls. Departures marked; open if a course asks.
- **The page's views for modal and rules sessions** (65d, 66): "Prove it step by step" as a second method of the modal suite. Recorded only in reports.
- **The client's CI e2e** (6.4 of 7.2 minutes; sharding if it grows). Recorded only in reports.
- **Eight client entries "for the author to revisit"** ("Kept as built", 2026-10-23: Practice hides its answer, near misses, light dismiss, fonts, the phone band, and so on). Open with the author; each is a client session if the author names one.

### The author's own items

Ranked candidates, each waiting for the author's account, word or a user:

- `users-1`: Discussions on, a "who uses deduxit" request. Kept, D32: no Discussions for now.
- `users-2`: a monthly reading of the public counters. The planning session's, when the author asks.
- `users-3`: reach users once, with something to try. Kept, D32: no announcement for now.
- `maintenance-7`: install channels in order (winget, Homebrew tap, Scoop, Chocolatey). Open with the author; needs their accounts.
- `maintenance-6`: code signing (macOS notarisation, Windows Authenticode). Open with the author; accounts and fees.
- `maintenance-5`: keyless cosign signatures. A release step takes it if the author asks.
- `maintenance-8`: a glibc prebuilt binary. No user asked.
- `llm-agents-1`: an MCP server (`deduxit mcp`). Step 44 said no for now; build it when a client without a shell asks.

Entries:

- **A Zenodo community** (`zenodo.org/communities/deduxit`) and a `.zenodo.json`. Open with the author; raise it at a release or when there are more kinds of records. Zenodo archived 0.6.0 (10.5281/zenodo.24393820) without it.
- **nixpkgs.** Kept at the author's word (2026-11-03): raise when there is public evidence of use beyond the author.
- **The opam submission of `rocq-deduxit`**, then `--kernel deduxit` as the default in every mode. The author said wait until more features (2026-11-09); the library is at 0.6.0 and tagged, the file in `~/Projects/own/rocq-opam` is for 0.3.0 and must be redone.
- **The LLTP header report** (drafted in `plan/notes/lltp-headers.md`). Open with the author; theirs to send.
- **The comparison on the libraries with 0.2.0's engines** and the baseline night. Open with the author: it needs a night they give; step 51's driver change makes refutations count as checked.
- **treefmt-nix's Biome module and `--config-path`.** An upstream issue or pull request if the author wants one.
- **The web client's judgement calls of step 38's sweep, and Safari's trackpad pinch** (a check by hand on a Mac or iPhone). Open with the author.
- **ILTP in the dataset** (the library states no licence; a word from its maintainers), **the dataset's licence** (CC BY 4.0 chosen) and **a hub upload** (Hugging Face, Croissant for NeurIPS). Step 72's, the author's acts.
- **Whether a model document should refuse a key it does not know** (55's reviewer). For the author; the wire rules say readers ignore unknown keys.
- **An `EVIDENCE_WORK` settings key**, and any other API change that would help a user (`writing.md`: ask with a recommendation). Open.
- **CI left to the author**: let this machine read the Cachix cache (a NixOS setting); delete the probe branches `ci-speed-probe`, `ci-speed-before`, `ci/ci-speed`. From 2026-11-20.
- **The organisation's profile README, the client's `llms.txt`, the page's About and the repositories' descriptions** may still say "linear logic first" (59; the planning session checks them at a release). Open.

### The rest

Ranked candidates, each kept for the reason shown; none has been started.

- `structure-2`: a repository per suite, for a suite a third party writes (D25). Waits for such a suite.
- `structure-5` (half-kept): the kernel crate. Step 67 ended at the gate on 2026-11-16: no crate. It reopens if a suite goes into a crate of its own that `deduxit` depends on, or a suite from another repository must not link the linear suite.
- `applications-1`: plans and certificates from the Horn engine. Research.
- `applications-2`, `-9`: the rule-system front end (run, explore, play) and first-order rule systems by grounding. After step 60's rules; no step asks.
- `applications-3`: enumerate and count proofs. Step 71b's enumeration (of simple types, not linear proofs) may serve it.
- `applications-4` (half-kept): proofs as programs and processes. One target (lambda-terms) done in step 50; a process calculus or other targets when asked.
- `applications-5`: a logic-programming mode, after quantifiers. After 0.7.0, the author's choice.
- `applications-6`, `-7`, `-8`, `-10`: reaction networks (an SBML subset to choose); credentials; budgets; workflow specifications. Documentation, no audience asked.
- `natural-deduction-2`, `-3`: translation from LK and LJ; normalisation and linear natural deduction. After 62 (done); Zucker's non-bijection bounds the claim.
- `natural-deduction-5`: Fitch modal natural deduction. After 62 and 65 (both done).
- `lambda-calculi-4`, `-5`: type inference; a reduction stepper tied to cut elimination. After 71; step 71a kept inference out by its brief.
- `modal-logics-4`, `-6`, `-7`, `-8`: nested sequents; S4 into linear logic as a cross-check; multimodal and epistemic; first-order modal logic (undecidable). After 65 and 66 (done).
- `interop-2`: Lean 4 certificates over cslib. cslib has no release.
- `interop-3`: a Lambdapi theory. No linear encoding there yet.
- `interop-4`: focused certificates with holes. Research.
- `interop-6`, `-12`: proof-assistant tactics that call deduxit; a version-checked `serve`. Portability unread.
- `interop-7`: a Python package. Not recommended yet.
- `interop-8`, `-9`, `-10`: a WIT component; a language server and tree-sitter grammar; notebooks. No client or step asks.
- `interop-11`: SZS and TPTP for the collection, native NXF reading. Needs interop-5 (step 78) and the modal suite.
- `llm-agents-3`: a Python package. Not recommended yet.
- `llm-agents-4`: structured hints and errors for models. Waits for a trial with a small model.
- `llm-agents-5`: a narrated `--explain`. Not recommended yet.
- `ordinary-sequents-4`: a DAG read-back so seven large classical proofs (8 to 32 million inferences) can be checked. Open; the same change as the SYJ202 entry above and a change to the checker (the panel's).
- `ordinary-sequents-6`: a structural-rule dial for LK and LJ. Needs logics-1; R is undecidable.
- `tableaux-4`: modal tableaux. Overtaken: the labelled calculus (step 66) and the modal search (65) cover the ground.
- `more-logics-3`, `-4`, `-5`: a QBF reader; ALC (about one session over the modal suite); Hoare logic as a rule file. No step asks.
- `more-logics-10`, `-11`: many-valued and paraconsistent logics as a mode of the tables and tableaux; epistemic, deontic and conditional logics as modes of the modal suite. After 56 and 65 (both done).
- `types-higher-3` to `-7`: session types; a certificate back end for linear type checkers; graded modalities; linear logic programming with terms; relevant mode. After 71 (steps 71a and 71b done).

Entries:

- **The kernel crate's list** (67a): the memory account, name checks, `Stops` and `Drawn` under `linear`; the propositional arena (about 300 lines twice); the parser (296); the sequent's JSON form (143); the goal tree's helpers; the checkers' meter; `protocol::Limit` built three times; the front ends' per-suite copies. Kept, each taken by the first session that edits it. Step 74 changes `ordinary::Node` and decides the shared arena, the parser and the goal tree's free functions.
- **Resolution's leftovers** (70b): RAT steps are refused (CaDiCaL and Kissat write RUP by default); a resolution session in the command ("by hand" is the library's `Refutation::resolve` and the page); first-order resolution is step 78.
- **Typed lambda leftovers** (71a, 71b): linear types are not inhabited (a linear `inhabit` path); no settings key `inhabit`; `Term::parse`'s errors say "cannot parse the sequent" (a shared wording of every parser). Open.

### Assigned to steps 74 to 78 (planned, not done)

Prompts are written (`plan/74-*.md` to `plan/79-*.md`); none has started.

- `more-logics-6`: first-order intuitionistic logic with Kripke countermodels. Steps 74, 75 and 76.
- `more-logics-1`: classical first-order logic with equality, as axioms first. Step 78.
- `tableaux-3`: first-order tableaux with free variables and connection provers. Step 75.
- `resolution-2`: first-order resolution at teaching scale. Step 78, after resolution-4.
- `resolution-4`: the TPTP `cnf` reader and SZS output. Step 78.
- `interop-5`: TPTP conventions in and out. Step 78.

### Closed meanwhile (taken or done by a step it was not assigned to)

For the next step's reading: these were open entries of `later.md` and are closed, and `later.md` does not all say so.

- Seriality with one new label (66b): taken by 54x7 (`reports/54x-proposals.md`, "Seriality with one new label"); `Ser` with `x = y` is refused by the checker with no new rule syntax. `later.md` still lists it open.
- Auto-reverse for the ordinary suite (63a): library, command and bindings by 54x7; the page's switch by 54x8. `later.md` still lists it open; its second half (rule files) is open, above.
- The web module's size: 54x6 took 129 KB back and the entry in `later.md` is rewritten; 71b raised the ceiling to 3 900 000.
- The tableau's missed stop and the focused engine's late first poll: 54x4; `later.md` says so for the second.
- `check --table-atoms N`, the wire error's article, `translate`'s antecedents: 54x3 (found by 56's report).
- A derivation's and a net's `<desc>` in the drawing's notation and in sentences: 54x2 (58b's proposals).
- Kani harnesses, clippy for wasm32, the 32-bit unit suite, the budget, the MSRV and cadence policy, the advisory check: 58a, as assigned.
- The one-sided and state-equation refutations certified and `check` reading a disproof: 51, as assigned.
- A checked basis for the Horn engine's non-coverability: 57, as assigned (the Rocq lemma stays open).
- Normal forms, resolution, DPLL, DRAT and LRAT reading (resolution-1, -3, more-logics-7): 70a to 70c, as assigned; `resolution-3` is reduced to a reader and checker.
- The simply typed checker, inhabitation and its counting (lambda-calculi-2 and -3, types-higher-1 and -8): 71a and 71b, as assigned; 71c is running.
- The model dataset and the grading mode (llm-agents-2 and -6, engines-7 dataset): 72, as assigned.

### Could not place

- `logics-9`: the table says "per item, each read at abstract level only"; I could not find which items the note holds. Listed as research, still open.
- `web-teaching-9`: the ranked table gives it to step 68 as "a teacher's check view", while the research note describes a classroom study with people. Step 68 built the check view; the study with people is nowhere. Not counted in any group.
- The parser reading `a⊥`: no report shows it closed; I did not read the parser. Listed as open.
- 58b's proposal "rule labels named by their step": the client commit "Give every rule label a hint" (2026-11-09) may be related; no report says it closed the proposal. Listed with the screen-reader item.
- Whether the flake's `ordinary-problems` check now holds the page's examples (49c asked step 63 to do it; 63's report names the check with 62 problems and 5 sessions). Probably done; not confirmed.
- `later.md` has no entry for follow-ups that only reports record (steps 63 to 72): the unrun lambda fuzz targets, RAT, the pack's tableau problem, 70c's page departures, 71's linear inhabitation. I listed them under their groups; the planning session may want to copy them.
