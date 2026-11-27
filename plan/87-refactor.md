# Step 87: the refactor for 0.9.0, in steps

You are working in deduxit (`deduxit/deduxit`), its Rocq library
rocq-deduxit (`../rocq-deduxit`) and, where a step says so, the web
client (`deduxit.github.io`). CLAUDE.md, each repository's rules and
`plan/conduct.md` apply throughout. This plan was written on 2026-11-27
by the session that led step 86's audit; the planning session runs its
steps like any other.

Read first: `plan/notes/audit/findings.md` (the themes this plan
follows, A1 to I2), `plan/notes/audit/api-proposal.md` (the 0.9.0
surface, P1 to P25) with the author's answer to it, and
`plan/notes/audit/baselines.md` (the numbers every step compares
against). A step's findings are in `plan/notes/audit/units/UNIT/`:
read each finding's `findings.md` entry **and its verdict** in
`verdicts.md` before acting on it, since a verdict often corrects the
place, the cost or the change.

## What holds for every step

- **The proposal is the author's to sign.** Steps marked with a part
  (P1, P5, …) implement that part as signed. A part the author changed
  or declined changes or removes its step: the planning session amends
  this file before starting it. Nothing public changes that no signed
  part names.
- **Behaviour is kept unless the step's first line says otherwise.** A
  step that finds it must change behaviour to go on stops and says so.
- **The base** is main with fixup-31 (the fixes of F-4d-01 and
  F-4c-01). The first step takes the measured baselines again at that
  commit if any source file changed since `vqwqlqkl`.
- **The net**, after each step, in this order:
  1. CLAUDE.md's table for what the step touched (`gate` for any code);
  2. the target sets with equal counters: `bench/targets.sh LABEL` and
     `bench/net-targets.sh LABEL` against `bench/targets/86.csv` and
     `net-86.csv` (equal to the oracles `47.csv` and `net-47.csv` at
     the base), by `deduxit-bench summary --against … --counters`, for
     every step that touches `core/src/linear` or what it calls;
  3. the ratchet: no journey above its ceiling's tolerance; a count
     that falls lowers its ceiling in a commit of its own;
  4. `breaks 0.8.0`: the step's report lists what it shows, and it
     shows nothing the signed proposal does not name;
  5. for the Rocq library: its own `nix flake check`, and deduxit's
     `rocq-deduxit` and `rocq-deduxit-library` checks against it.
- **The panel** (`panel-counterexamples`, `panel-argument`,
  `panel-integers`) runs on every step marked "panel": it touches a
  checker, a search or a certificate.
- **Sessions**: Opus 5.5 at `high`, `fresh-reviewer` on Fable 5.2 at
  `medium`, one session per step. A step that proves too large for one
  session divides itself along its themes and says so; it does not
  leave a theme half done.
- **Records**: each step adds to the changelog's `[Unreleased]`
  ("Changed", "Removed", and the migration table for library users of
  the proposal's section 11), corrects the rules file of the area it
  touched (the audit found stale lines: `core-derivations.md:297`,
  `core-search.md:403-407`), and writes `plan/reports/87XN-….md`.
- **One wire level** for all of 0.9.0's changes of a JSON form: the
  first step that changes one bumps `wire::LEVEL` and writes the
  upgrade from 0.8.0's forms; the later ones add to the same level.
- **Text** a person reads follows `.claude/rules/writing.md`; no suite
  is the main one.

## The steps

Waves run in order; inside a wave, steps listed as "beside" touch
different files and can run at the same time.

### Wave A: the net's gaps (no change of behaviour)

**87a. Journeys and pins for what nothing measures.** Themes D1, and
the pins C2 and A3 need. Adds ratchet and DHAT journeys, each with a
ceiling: ordinary logic's SAT method, LJT, the classical and the
intuitionistic connection prover, propositional and first-order
resolution; a first-order search of linear logic; the modal tableau;
the finite and the Kripke model search; lambda-terms (check, reduce,
inhabit); one session per suite (`candidates`, `apply`, `close`); cut
elimination; a saturated refutation's check; the inverse method; a rule
file's checker; a workspace run. Takes the generators' share out of the
measured part of the existing journeys (F-2c-12), with the ceilings
re-recorded in a commit of its own that says why. Pins the two
connection provers' statistics on a set of ILTP rows (a file under
`bench/`, compared as the target sets are). Files: `bench/` alone.
Alone in its wave: every later step reads its ceilings.

### Wave B: fixes (each changes behaviour, each small)

**87b1. Bounds and misleading calls of the linear suite.** *Changes
behaviour.* Themes A2 and A4, the linear findings: F-6a-01, F-1c-01,
F-2b-01, F-3-03, F-2a-01, F-1a-14, F-1a-07, F-6a-02, F-2c-01, F-2a-13,
F-4f-03, F-4f-07, and the linear half of F-4e-07. With P15: the public
`linear::nets::Net` and the call that chooses a proof's net (F-3-01).
Panel (the entry of the refutations' check; the net's criterion is
called, not changed).

**87b2. Bounds and misleading calls of the other suites.** *Changes
behaviour.* F-4a-01, F-4c-10, F-4c-09, F-8-12, F-4e-02, the ordinary
half of F-4e-07, F-5b-11, F-5b-01, F-5b-07, F-4b-05, F-6b-16. Beside
87b1.

**87b3. Front ends that answer wrongly.** *Changes behaviour.* Theme
A3's command and bindings findings: F-7b-01, F-7b-02, F-7a-07, F-7a-02,
F-7b-03, F-7b-04, F-8-02, and the three front ends call `Net::of`
(F-8-01, F-7b-14), with a protocol test of the proof that cuts one
occurrence twice. After 87b1. Files: `cli/`, `web/`,
`core/src/protocol/linear.rs`.

### Wave C: the layering (P3, P4, P2, P11); paths change, behaviour does not

One after another: each rewrites imports across the crate.

**87c1. `Reason`, the memory account, refusals and the meter.** P3 (two
rows), P7; themes B2, C6; F-arch-01, F-4d-14, F-arch-03, F-6a-03,
F-2a-10, F-5a-05. `limits::Reason`; `memory` at the root;
`Refusal`'s constructors used at the 103 sites that build one by hand;
`limits::Meter` in place of the five private meters.

**87c2. `notation`, `terms`, `tptp`.** P3; F-1a-02, F-1a-03, F-arch-06,
F-arch-07. Removes the one place where `linear` names `ordinary`.

**87c3. `view` and `session`.** P3; F-1b-01, F-arch-07, F-4e-03,
F-4f-09. *Changes behaviour in one point*: `Labels::Table` is keyed by
a rule's name as text, so a user's labels reach ordinary logic,
lambda-terms and rule files (F-1b-04, F-5a-09).

**87c4. One options value per front door, and no default logic in the
library.** *Changes the API's behaviour.* P4, P2; F-arch-02, F-4b-15,
F-4c-04, F-8-10, F-2a-12, F-4a-03. The bindings' `about` follows.

**87c5. The Kripke model in one place, and the evidence inside the
verdict.** *Changes JSON forms (the wire level).* P11; F-arch-21,
F-4a-12, F-arch-13, F-arch-23, F-2a-09, F-1c-03, F-4d-13, F-4b-04,
F-4c-05, F-6b-10. Panel (the model checks move, unchanged).

### Wave D: the API's additions and names (P6 to P15, P20 to P22)

**87d1. The linear sequent's builder.** P6; F-1a-01, F-1a-08, F-1b-09,
F-2a-03, F-4a-16. Routes the families, `mist.rs`'s arena and the
embedding through it; the families' arenas stay byte-identical (the
behaviour lock, `core/tests/serialize.rs`, `deduxit-bench run
--all-families --timeout 5`).

**87d2. Builders of the other suites.** P6; F-4a-02, F-arch-05,
F-5a-02, F-5a-03, F-5a-04, F-5b-03, F-5b-04. Beside 87d1.

**87d3. The engine interface and the manual's page.** *Changes the
API.* P8, P9, P10; theme B4; F-arch-04, F-arch-26, F-2a-02, F-1c-02,
unit 4a's remark on
`ordinary::Outcome`. `Outcome`'s fields private; the type that names an
outcome's engine; `Decide`, `Found`, `prove_with`;
`Outcome::of_derivation` and `of_countermodel`; the page "Write your
own engine", run by the readme test; `plan/notes/audit/consumer-test.md`
run again as the acceptance test, its thirteen points answered in the
report. After 87d1 and 87c1. Panel (the front door's `conclude` is the
trust boundary).

**87d4. Errors.** *Changes JSON forms.* P12; F-6a-15, F-5b-05, F-4e-06,
F-6a-13, F-8-05. Beside 87d1 and 87d2 if 87c5 has landed.

**87d5. Names.** *Renames.* P13; F-1a-05 (the verbs), F-4e-01, F-4f-06,
F-4a-13, F-5a-06, F-7a-03 (the library's `NAMES`; the command reads them
in 87e1). After 87d2 and 87d4.

**87d6. The protocol's registry and the library calls the front ends
lack.** P14, first half; F-arch-10, F-arch-11, F-8-06, F-8-13, F-8-16,
F-8-04, F-7a-17, F-5b-02, F-7b-10, F-8-07, F-8-09, F-8-11, F-6a-07;
and the labels of ordinary logic's and the modal tableau's statistics
move from the command into the library, as the linear engines' are
(F-7b-15).

**87d7. The session over JSON.** *Changes the bindings' calls; a pair
with the client.* P14, second half; F-arch-12, F-4f-01, F-8-08, F-8-14,
F-7b-06. The library's `Session` trait and wire forms; the bindings'
one set of session calls; "87d7: deduxit ready" to the planning
session; then the client's calls. After 87d5 and 87d6.

**87d8. One path per type, the unused public items, the common
traits.** P20, P21; themes B7, B8. Last of the wave, since it reads the
final paths; its report lists each of the 82 items and what became of
it.

### Wave E: the command (P1, P5, P16 to P19)

**87e1. One decision, one answer, the flags declared once.** Theme C12;
F-7a-12, F-7a-13, F-7a-14, F-7b-07, F-7b-09, F-7b-16, F-7b-17, F-7b-21,
F-7b-08; `grade` reads the answer value. *One search change*: the
library's default replaces the command's own copy for first-order
ordinary sequents (F-7b-05); the benchmark's verdicts on the ILTP
first-order rows show whether anything moves, and the step says so.

**87e2. The selector of the logic.** *Changes the command.* P1, P16,
P17, P5 (the settings' keys, with the bindings and the client's stored
settings); F-7a-01, F-7a-04, F-7a-05, F-6a-09, F-arch-14, F-arch-15.
Every console block of README, the manual and the guide; the behaviour
lock; `hint.rs`. After 87e1.

**87e3. Completions, the man page, the sessions at a terminal.** P18,
P19; F-7a-11, F-7b-12, F-7b-18, F-7b-19. Beside 87e2 where files
allow, else after it.

### Wave F: duplication (no change of behaviour); beside each other by area

**87f1. One arena for ordinary and modal logic.** C1.

**87f2. The connection provers and the builders of LK.** C2. Needs
87a's pin; a difference in the pinned statistics is a search change and
stops the step. Panel.

**87f3. The exports.** C3. The snapshots byte for byte; no certificate's
text changes; the table of the Rocq library's constructors becomes the
one place the exporter names them (with 87r1). Panel (certificates).

**87f4. The nets' graph kernel and sequentialization.** C4. The net
target set's counters. Panel (the correctness criterion).

**87f5. Walks, copies, and the repeats of ordinary and modal logic.**
C5, C10. Panel for F-4d-02 (two evaluators of one semantics).

**87f6. The repeats of the linear suite's data model, proofs and
refutations.** C9 (units 1a, 1b, 1c), C7's F-1c-11. F-1b-05 moves the
checker's arithmetic into the functions Kani checks: Kani's harnesses
(`nix build .#kani-harnesses`), panel.

**87f7. The repeats of the linear search's engines.** C9 (units 2a, 2b,
2c), C7's F-2c-11, C6's F-2b-07 and F-2c-15. Both target sets'
counters; panel.

**87f8. The sessions' shared code, lambda-terms, rule files, the
workspace, the wire forms.** C8 (library side), C11.

### Wave G: performance, by measurement

**87g1. The measured hot spots.** D2. Each change with its journey's
blocks and instructions before and after in the report; the linear
checker's (F-1b-02) under the panel and Kani's harnesses.

**87g2. The stop shims, the hasher, the web module's first two mends.**
D3, D4, P25 steps 1 and 2; F-8-03, F-5a-07. `clippy.toml` with
`disallowed-types`. Measured by `cargo llvm-lines` and the module's
bytes at gzip.

**87g3. The candidates.** D5, on 87a's journeys: each measured, kept if
a count moves, recorded as tried in its unit's rules file if not.
F-1c-13 and F-2a-08 would be search changes: only with the planning
session's word. Divides by suite if it is too large for one session.

### Wave H: structure (moves only); beside each other by file

**87h1.** `linear/search/mod.rs` into its five parts (F-arch-19), with
`the_documented_dispatch_is_the_dispatch` following `Engine`.
**87h2.** `ordinary/interactive.rs` along the linear session's seams
(F-4e-05, F-arch-20), and `ordinary/mod.rs`.
**87h3.** `ordinary/normal.rs`, `resolution/first_order.rs`,
`herbrand.rs` (F-4b-08, F-4c-11).
**87h4.** `focus/framed.rs`, `Counts::new_until`, the counts and the
bias out of the focused engine (F-2b-06, F-2b-13, F-2b-03); the target
set's counters.
**87h5.** Ordinary logic's part of `rules` into `ordinary`, the
countermodels into a directory, the refutations' checks into a module
of their own, `occurrences` free of `proofs` (theme E2).
**87h6.** `svg::mell::draw`, the nets' long functions, `serialize/
ordinary.rs`, and the small matters of idiom (F-6b-03, F-3-11, F-6a-19,
theme E3); a lint whose last site goes is turned on.

### Wave I: text, one pass per area

**87i1.** The front pages in the page's order: the crate docs, each
suite's module docs, the exports', the help's syntax section, the
manual's chapter on proving step by step (F1), with the examples of P22.
**87i2.** Messages (F2): the sessions', the rule files', the readers'.
The behaviour lock and the page's hints change with them.
**87i3.** Docs that fell behind the code (F3), each list that repeats a
table of the code tested against it.
**87i4.** The help and the manual's commands (F4's command findings),
after 87e2.

### Wave J: the build

**87j1. The features.** P24, theme I1: `export`, `serde`, the docs of what
`serde` gates; the measurement the proposal asks for (the engines'
share of a build without default features), reported to the author.
**87j2. The web module, the registry from the page's list.** P25 steps
3 and 4, by measurement after 87g2 and 87d6; a pair with the client.

### The Rocq library (its own repository; beside every wave)

**87r1. Packaging, CI and the contract.** H1; with 87f3 for the
exporter's side (F-9-01, F-4d-08).
**87r2. One copy of what the checkers share.** H3. No trusted statement
changes; `tests/Assumptions.v` and the certificates against the
library; a rename moves deduxit's exporter in the same pair of changes.
Panel (`panel-argument` on the statements' identity).
**87r3. Proofs that do not rely on generated names.** H2; the build
with `-mangle-names` becomes a check.
**87r4. Text, and the two candidates of speed.** H4, measured on
`rocq compile` time before any change of the exporter.

### Last

**87z. Before the release.** The baselines taken again and compared
with `baselines.md` (compile and test times, the ratchet, the module's
bytes, the public API's size, Clippy's allowed lints, duplicates);
`breaks 0.8.0` in full; the changelog's migration table checked against
it; the consumer test's crate built once more against the result. Then
the release of 0.9.0, prepared as step 80 prepared 0.8.0.

## What runs beside what

| wave | steps | order |
|---|---|---|
| A | 87a | alone, first |
| B | 87b1, 87b2, then 87b3 | b1 beside b2 |
| C | 87c1 to 87c5 | one after another |
| D | 87d1 beside 87d2 (and 87d4); 87d3; 87d5; 87d6; 87d7; 87d8 | as listed |
| E | 87e1; 87e2; 87e3 | after D |
| F | 87f1 to 87f8 | beside each other; F may start after C, beside D, where a step's files are free |
| G | 87g1 beside 87g2; 87g3 | after 87a and F |
| H | 87h1 to 87h6 | beside each other, after D and F |
| I | 87i1 to 87i4 | after E and H |
| J | 87j1; 87j2 | after D |
| R | 87r1 to 87r4 | beside everything; r1 with 87f3 |

Forty-eight steps: eleven with the panel (87b1, 87c5, 87d3, 87f2 to
87f7, 87g1, 87r2), four of text (87i1 to 87i4), the others of the small
kind, by the kinds of `plan/notes/step-durations.md`.

## What the plan leaves out

- The eight rejected findings (`plan/notes/audit/rejected.md`).
- What the proposal does not propose (its section 13): a crate per
  suite, a generic goal state, one verdict type, a public `Drawn`, the
  rename of the linear suite's `Term`.
- A feature for the data model and the checkers alone, until 87j1's
  measurement and the author's word.
