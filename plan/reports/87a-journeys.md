# Step 87a: journeys and pins for what nothing measures (report)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-step-84-dc9a99`, from `plan/87-refactor.md` (87a, wave A)
and the planning session's briefs `start-87-common.md` and
`start-87a.md`, on cores 4 to 9 (2 and 3 for the target sets, given by
the planning session). The base is main at `kmxoxyxv` (fixup-31 on it).
Files changed: `bench/` and its rules file, `core-inputs.md`'s path to
the journeys, this report and the baselines.

## Outcome

- **Every engine and checker D1 names has a journey** with a ceiling of
  instructions and one of heap blocks: 22 new journeys, 42 in all.
- **Each journey's heap blocks are counted and held to a ceiling**: the
  ratchet counts how often the allocator was entered inside the measured
  part, read off callgrind's instruction positions, so a journey's input
  stays out (F-2c-12's mend; see the first departure below). The counts
  equal DHAT's of the measured part's stacks.
- **The two connection provers' statistics are pinned** on 1 312 ILTP
  problems, compared as the target sets are, `work` among the counters
  (C2's net for 87c2).
- **The measured baselines are taken again at `kmxoxyxv`**: both target
  sets equal the oracles; the ratchet's counts, the web module, the LLVM
  lines, the allocations and the compile times are in `baselines.md`,
  section "Again at the refactor's base".
- **Found on the way**: main's flake `ratchet` check fails at
  `kmxoxyxv`, on `render-latex` and `render-svg` (sandbox's build 2.10 %
  and 2.02 % over their ceilings); raised in a commit of its own.

## The new journeys

Each prepares its input outside `measured`, runs one thread, and fails
when its verdict is not the one expected. Sizes were chosen for counts
of a few million to sixty million instructions, so that the ratchet's
time stays about what it was.

| journey | what it runs | instructions | blocks |
|---|---|--:|--:|
| `ordinary-sat-classical` | `sat::decide`, classical, pigeonhole 7 in 6, the clausal proof checked | 21 131 448 | 5 856 |
| `ordinary-sat-intuitionistic` | `sat::decide`, intuitionistic, SYJ202 at 4 | 27 851 236 | 124 077 |
| `ordinary-ljt` | `ljt::decide`, Korn and Kreitz (SYJ205) at 30 | 26 052 945 | 43 462 |
| `ordinary-connection-classical` | `connection::decide`, classical, a first-order chain of 24 | 9 311 985 | 13 189 |
| `ordinary-connection-intuitionistic` | `connection::decide`, intuitionistic, a nest of 24 implications | 7 207 733 | 16 316 |
| `ordinary-resolution` | `resolution::decide`, saturation, pigeonhole clauses 4 in 3 | 9 448 633 | 11 941 |
| `ordinary-resolution-first-order` | `first_order::resolve`, a chain of 16 | 5 930 262 | 16 725 |
| `ordinary-models` | `models::decide`, classical, a structure of three elements | 622 448 | 1 415 |
| `ordinary-kripke` | `models::decide`, intuitionistic, a Kripke model | 385 496 | 862 |
| `session-ordinary` | `ordinary::Interactive`, LK, pigeonhole 3 in 2 | 39 291 320 | 76 986 |
| `search-chain-fo-32` | `linear::prove`, `chain-fo/32` | 24 878 189 | 18 874 |
| `search-inverse` | `Engine::Inverse` forced, `contraction/16#0` | 4 455 677 | 5 006 |
| `check-saturated` | `Disproof::check` of the saturated set of `contraction/8#1` | 22 297 817 | 38 722 |
| `cut-elimination` | `cut::eliminate`, `!a ⊢ a ⊗ …` (24) through a cut on `!a` | 11 716 358 | 16 053 |
| `session-linear` | `linear::Interactive`, classical MALL, 16 conjuncts | 2 256 947 | 4 181 |
| `modal-tableau` | `modal::decide_within`, K, Halpern and Moses at 3 | 9 655 516 | 27 359 |
| `lambda-check` | `Term::check`, 4 000 nested redexes | 7 588 815 | 4 008 |
| `lambda-reduce` | `Term::reduce`, 100 redexes to normal form | 30 802 732 | 60 085 |
| `lambda-inhabit` | `lambda::enumerate`, `a → … → a` (8) | 469 788 | 1 290 |
| `session-rules` | `rules::Interactive`, NJ, a conjunction of 3 reversed | 55 867 171 | 141 308 |
| `rules-check` | `rules::Derivation::check` on LK, a pigeonhole derivation | 23 672 639 | 57 940 |
| `workspace-run` | `Workspace::run`, `What::Check`, a chain of 16 in intuitionistic logic | 2 987 650 | 4 214 |

- **A session journey is one driver**: on each open goal the candidates
  at every formula, the first that applies and is invertible taken
  (`is_invertible`), else the goal closed by the search. Modal logic and
  lambda-terms have no session in the library, so "one session per
  suite" is three. The rule files have no `is_invertible`;
  `session-rules` takes `→I` and `∧I` by name.
- **`workspace-run` decides in intuitionistic logic**: in classical
  logic the default's evidence search through the embedding was 98 % of
  the count (208 million, almost all the focused engine), which another
  journey measures.
- **Determinism**: instructions moved by at most 125 between the
  shell's and the sandbox's builds on the new journeys
  (`ordinary-resolution-first-order`). Two journeys' counts move between
  runs of one build by tens of instructions
  (`ordinary-resolution-first-order` up to 174, `modal-tableau` 41;
  their blocks not at all): std's `HashMap` and `HashSet`, seeded per
  process, in `ordinary/normal.rs`, `ordinary/instance.rs` and
  `modal/model.rs` (theme D4, 87d2). Every other count was equal in two
  runs.

## The connection provers' pin

`bench/connection-targets.sh LABEL`, `bench/problems/connection.txt`,
the oracle `bench/targets/connection-87a.csv` (`bench/TARGETS.md`, "The
connection provers' pin"): 910 classical and 248 intuitionistic
first-order problems that each prover decided within a second in
`bench/defaults/connection-setup-iltp-*-after.csv`, and the 144
classical and 10 intuitionistic problems of the propositional library
alone that it decides within a second, all at a limit of 5 s. Two runs
on six cores decided every row alike with no counter apart (`nodes`, the
extensions, and `work`, every step); about a minute a run.

- `--counters` now compares `work`, `unifications` and `bindings` where
  both rows have them (`compare::WHERE_BOTH`): `nodes` alone is a
  connection prover's extensions. On every decided row of `86.csv`,
  `87a-base.csv`, `net-86.csv` and `net-87a-base.csv` they agree, so the
  target sets' comparison loses nothing; the pinned output of
  `bench/tests/compare.rs` names them.
- A propositional problem of `bench/iltp` that the first-order library
  also holds (the same file) is listed once: a file given by path has no
  family, and two rows of one name would be one key.

## Departures

- **The generators' share (F-2c-12) is taken out of the instrument, not
  the journeys** (departs from the plan). The plan asked to take the
  generators out of the measured part and re-record the ceilings. The
  verdict on F-2c-12 (`units/2c/verdicts.md`) found every generator
  already outside `measured`; only DHAT, which profiles the whole
  process, counted them. So the ratchet now counts each journey's blocks
  inside the measured part, with a ceiling per journey:
  `search-additive-14` has 65 602 blocks there, where DHAT counted
  167 253 for the process. No instruction count moved, so the
  re-recording the plan foresaw did not arise; the ceilings that did
  move are below.
- **How the blocks are counted took three tries** (departs from the
  design note of the morning). A counting global allocator: the
  workspace forbids `unsafe`, and it would have added instructions to
  every count. Then callgrind's `calls=` lines into `malloc`, `calloc`,
  `realloc` and the aligned allocators (`mkoxtzom`): they depend on how
  a build reaches the allocator (an inlined `__rust_alloc`, tail jumps),
  counted twice `read-text`'s blocks and differed by 487 between two
  builds of `workspace-run` whose allocator ran alike (same entries,
  same inclusive cost). Last, how often each allocator's first
  instruction ran (`--dump-instr=yes`, `vylrnovn`): equal to DHAT's
  blocks under `journeys::measured` (`read-text` 4 188, `workspace-run`
  4 214) and the same in both builds. The blocks ceilings were recorded
  again by it.
- **Three ceilings raised** (`lulwvwwy`): `render-latex` and
  `render-svg` to the sandbox's instruction counts, over before this
  step; `read-json`'s blocks then, which the re-recording replaced.
- **No DHAT ceiling**: "DHAT journeys, each with a ceiling" is met by
  the blocks' ceiling in the ratchet; `plan/audit/stage0.sh dhat` reads
  its journeys from `bench/ceilings.csv` and profiled all 42.

## The net

- **`nix flake check`** at `nlyqlxlz`, under the machine lock: passes
  (exit 0). Its `ratchet` check counts all 42 journeys in the sandbox's
  build: every blocks count equal to its ceiling (recorded from the
  shell's build), the largest instruction count 1.53 % over its ceiling
  (`check-qbf-20-2`, as at the base).
- **The bench crate**: Clippy with `--deny warnings` and its tests (21
  unit tests, the comparison's and the manual's) after each change.
- **The shell's ratchet**, all 42 journeys, two runs of one build with
  `--check`: both pass; the counts as under "Determinism".
- **The target sets** at the base: equal to `86.csv` and `net-86.csv`
  (145 and 64 rows, no counter apart). Not run again: nothing under
  `core/` changed.
- **The connection pin**: two runs equal (above).
- **`breaks 0.8.0`**: what it shows is main's, since this step changed
  nothing under `core/`: cargo-semver-checks fails two checks (the
  `reason` fields of `StepError::Witness`, `StepError::Binding` and
  `Error::StepRefused`; System F's variants of the closed `lambda::Node`
  and `lambda::Type`), and cargo-public-api's diff lists the additions
  since 0.8.0. Run outside the devshell's shell, the command needs
  `PRJ_ROOT` set (its first run stopped after cargo-semver-checks).
- **No panel**: the brief says none for this step.

## The review

`fresh-reviewer` on Fable 5.2 at `medium`, given the five commits up to
`lulwvwwy`, the briefs and six criteria (determinism and expected
verdicts, the blocks count, the pin's script and list, the comparison,
the scope, the docs). It reproduced what it claimed with scratch builds
and planted rows. Three findings, all fixed:

1. `workspace-run` checked only that its node ran, not what it decided:
   a sequent that is not valid ran as well. It now reads the node's
   outcome and asks `"verdict": "valid"` (`nlyqlxlz`).
2. `connection-targets.sh` waited for its streams in one `wait`, which
   answers the last one's status alone, so a failed stream joined its
   file short and exited 0; `targets.sh` had the same, inherited. One
   `wait` per stream now in both (`nlyqlxlz`).
3. The rules file and `TOLERANCE`'s doc said the two builds count under
   0.1 % apart but on `render-svg`, which this step's own measurement
   contradicts (about 1 % on both renderers): corrected (`vylrnovn`).

It checked as sound the blocks count of the time (eight journeys
against an independent parse), the ceilings file's reading, checking
and lowering, the journeys' determinism and inputs, the pin's list (no
duplicate key, all decided, the longest 847 ms) and that a planted
`work` or `nodes` change shows in the comparison, the comparison on
every pair of target-set files, and the scope (nothing under `core/`,
`cli/` or `web/`; `Cargo.lock` unchanged). The blocks count it checked
was then replaced for the reason above, found while fixing its first
finding: the fix moved `workspace-run`'s count of calls by 487 with the
instructions nearly equal.

## What the next steps must know

- **87c2**: `bench/connection-targets.sh LABEL` on any cores
  (`CORES="4 5 6"` through `systemd-run --setenv`, by default 2 and 3),
  then `deduxit-bench summary --against
  bench/targets/connection-87a.csv bench/targets/connection-LABEL.csv
  --counters`; a counter that moves is a search change and stops the
  step. Needs `bench/iltp` and `bench/iltp-first-order`.
- **Wave D (87d1, 87d3)**: each journey's blocks are in the ratchet's
  table beside its instructions; a candidate's gain is the two counts
  before and after, both from one build. `ratchet --keep DIR` and
  `callgrind_annotate` give the functions; DHAT's frames come from
  `plan/audit/stage0.sh dhat`, whose tables include the input's
  generation (whole process).
- **The ratchet takes longer**: 42 journeys; the new ones add 344
  million instructions to the 2.87 billion of the first twenty (their
  ceilings), and
  `--dump-instr=yes` makes callgrind's files larger, not its counts.
- **Not changed, outside `bench/`**: CLAUDE.md's line on `ratchet` and
  `modules/devshell.nix`'s help text name the instruction counts and the
  misses, not the blocks; `modules/ratchet.nix`'s comment likewise.

## Commits

On `lltmsktv` (main at the start, fixup-31's `kmxoxyxv` below it):

| change | commit | what |
|---|---|---|
| `upvnuszs` | `a47b966eff05` | the target sets at the refactor's base |
| `mkoxtzom` | `94d46e746771` | the ratchet's blocks and their ceilings (counted by calls, replaced in `vylrnovn`) |
| `vxrnxkzx` | `1c39679b2297` | the 22 journeys, their ceilings and misses |
| `mqkuxuuu` | `8f0753eb2bc3` | the connection provers' pin; `--counters` compares `work` |
| `lulwvwwy` | `37914a0a5127` | three ceilings at the larger build's count |
| `ywyqnmqs` | `1ce5d58b87f4` | the rules: two counts move with std's seeded tables |
| `zqtnpkvz` | `c810d43ba99d` | the measured baselines again at the base |
| `vylrnovn` | `2af9c2b81b19` | blocks counted by the allocator's entries, re-recorded; the builds' spread |
| `nlyqlxlz` | `beced076905f` | the review's fixes: `workspace-run`'s verdict, one `wait` per stream |
| this report | | |
