# The audit's method (step 86)

How phase B finds, verifies and ranks, fixed before any agent runs. The
criteria are `criteria.md`, the numbers `baselines.md`, the evidence of
an outside user `consumer-test.md`, and what every agent reads first
`brief.md`. Phase B started at the planning session's "go" on
2026-11-27, after step 85's library part had landed; it reads and
changes nothing.

## What phase B reads

The tree at main after step 85 (`vqwqlqkl`), in the audit's workspace
(`../deduxit-wt-audit`, moved there with `jj new main@origin`), and
`../rocq-deduxit` at its `main`. The measured baselines are taken again
at that commit first (`baselines.md`), so that every finding's numbers
and line references are of the code the refactor starts from.

## The grid

**Dimensions**, each read for in every unit, each finding under one:

| dimension | criteria |
|---|---|
| idiom | E1 to E7, the C items on naming and predictability |
| architecture and duplication | E5, E6, P13, D4, Q9 |
| performance | P1 to P12, Q10 |
| the public API and its extensibility | the C items, X1 to X9, B4, Q8 |
| text | T1 to T4, E4, the C items on documentation, Q11 |

The command adds L1 to L13 and D1; the Rocq library Q1 to Q11.

**Units.** The prompt's nine areas, split so that one finder reads all
of its unit's source, tests apart: at most about 14 000 lines. The
units are data, `plan/audit/units.json`; `plan/audit/units.py` sizes
them, checks that every source file is in one, and writes each unit's
file list for its finder. Lines are of main at `vqwqlqkl`, test files
apart.

| unit | area | what | lines | rules files |
|---|---|---|---:|---|
| 1a | 1 | the sequents, parsing, fragments and occurrences of linear logic, and the first-order terms and names every suite shares | 11 127 | `core`, `core-sequents`, `core-forest` |
| 1b | 1 | the proofs of linear logic, their checker, derivations and sizes | 13 772 | `core`, `core-proofs`, `core-derivations` |
| 1c | 1 | cut elimination and the checked refutations of linear logic | 7 016 | `core`, `core-proofs`, `core-search`, `core-inverse` |
| 2a | 2 | the front door of the linear search: options, outcomes, the choice of engine, memory, the batch and the parallel runtime | 8 035 | `core`, `core-search`, `core-batch`, `core-parallel` |
| 2b | 2 | the focused engine of linear logic | 13 566 | `core`, `core-focus`, `core-parallel` |
| 2c | 2 | the Horn engine, the inverse method, and the readers and families of benchmark inputs | 8 278 | `core`, `core-horn`, `core-inverse`, `core-inputs` |
| 3 | 3 | proof nets and the net engine | 11 064 | `core`, `core-nets` |
| 4a | 4 | the core of ordinary logic: formulas, parsing, translation, derivations, the front door, models and truth tables | 12 443 | `core`, `core-ordinary` |
| 4b | 4 | the propositional engines of ordinary logic: LJT, SAT, normal natural deduction, interpolants, tableaux, written proofs | 13 926 | `core`, `core-ordinary` |
| 4c | 4 | the first-order engines of ordinary logic: the connection method, resolution, Herbrand | 14 457 | `core`, `core-ordinary` |
| 4d | 4 | countermodels and certificates of ordinary logic, and modal logic | 13 996 | `core`, `core-ordinary`, `core-modal` |
| 4e | 4 | the step-by-step states of ordinary logic and of calculi given as data, and what they share | 9 055 | `core`, `core-ordinary`, `core-rules`, `core-derivations` |
| 4f | 4 | the step-by-step state of linear logic | 7 640 | `core`, `core-derivations` |
| 5a | 5 | lambda-terms: untyped, simply typed, linear and System F | 11 036 | `core`, `core-lambda` |
| 5b | 5 | calculi given as rule files, and the workspace | 10 170 | `core`, `core-rules`, `core-workspace` |
| 6a | 6 | what the suites share: the crate root, errors, limits, settings, the wire forms and serialization | 13 186 | `core`, `core-sequents` |
| 6b | 6 | the exports: LaTeX, Typst, SVG, PNG, PDF and Rocq | 13 665 | `core`, `core-export` |
| 7a | 7 | the command's interface: its arguments, help, settings, input and output, and prove, check, seq and the batch; every `--help`; the manual's command chapters | 10 561 | `cli` |
| 7b | 7 | the command's other files: the session, what it shows, and the suites' commands | 11 122 | `cli` |
| 8 | 8 | the web bindings as a consumer of the library, and the protocol they call; how `bench/src` and the fuzz targets call the library | 13 087 | `web`, `core-protocol`, `core` |
| 9 | 9 | `rocq-deduxit`: `theories/` (linear, intuitionistic and modal), `tests/`, `bridges/`, the opam file, the project file, the flake's checks; the exporter's preludes for Q8 | 12 036 | the library's `rocq.md` and CLAUDE.md |

Twenty-one units, 239 000 lines. Phase A planned twenty; steps 82 to 85
added 7 000 lines to the core crate, and three units outgrew the bound:
- lambda-terms (11 000 lines with untyped terms and System F) are a
  unit of their own, 5a, and modal logic joins the countermodels of
  ordinary logic in 4d, with which it shares the Kripke models;
- the protocol joins the bindings that call it, in 8;
- the interactive states are two units: 4e (ordinary logic and calculi
  given as data, which share `goal_tree.rs` and `unify.rs`) and 4f
  (linear logic). The duplication between them is the architecture
  reader's to judge, with the clone detector's list.

Phase A's table also missed `linear/proofs/first_order.rs` (1 151
lines), now in 1b, and `ordinary/transfer.rs`, now in 4d.

The rules files are long (`core-ordinary.md` has 2 710 lines); a finder
reads the parts on its modules, since a rules file records choices made
on purpose, and a finding against one must say why its reason no longer
holds.

## The agents

Every agent reads `brief.md` first: where everything is, the machine's
rules (below), what a finding is and its form, and its role's
procedure. Its prompt names its role, unit, model and effort.

- **Inventories** (stage 0): plain tools run by the leader, and Sonnet
  5.5 at `medium` where a list needs reading. They give every finder
  facts instead of impressions:
  - the public items (`cargo public-api`, as in `baselines.md`), the
    items reachable under several paths, and each public item's uses
    outside its module in the workspace's other crates, tests and doc
    examples: the candidates for `pub(crate)` (X9);
  - public items without an example, from the rustdoc JSON (C-EXAMPLE);
  - which module of the core crate names which, and what one suite
    names of another (D4);
  - the functions over 100 lines with their lengths (Clippy's
    `too_many_lines`) and the files over 1 000 lines (E5);
  - duplicate code by PMD's clone detector CPD (E6);
  - sentences that match the patterns of `writing.md`, rule 4, in doc
    comments, help texts and messages, each then read by a Sonnet agent
    that drops the false matches (T4);
  - the sizes of the types the journeys allocate most
    (`-Zprint-type-sizes`), the allocation sites of each journey (DHAT,
    `valgrind --tool=dhat` on `deduxit-bench journey`), and the
    functions with the most LLVM IR (`cargo llvm-lines`) (P2, P4, P5);
  - every subcommand's `--help`, and the flags as a table: name, short
    form, value, the subcommands that take it (L2, L6, L8);
  - for the Rocq library: `Print Assumptions` of every theorem and
    lemma against the ones `tests/Assumptions.v` asks about, the
    warnings of a build with every warning on, a build with
    `-mangle-names`, the build's time per file, and the modules the
    exporter's certificates import (Q5 to Q8, Q10).
  The scripts are `plan/audit/`: `stage0.sh PART…` runs each part
  capped on cores 13 to 15 into `target/audit/` (`lines`, `api`,
  `clippy`, `duplicates`, `text`, `help`, `sizes`, `llvm`, `dhat`,
  `rocq`, and `timings`, which `all` leaves out), with `lines.py`,
  `surface.py`, `clippy.py`, `duplicates.py`, `text.py`, `help.py`,
  `rocq.py`, `timings.py`, `deps.py` and `units.py` behind it.
  - CPD reads Rust, but its lexer stops at some literals (`'∨'`,
    `"\\/"`, `"~"`); `duplicates.py` first copies the sources with
    comments and literals emptied, and then no file is skipped.
  - `text.py` finds few matches (86 in 66 702 lines of prose at 0.8.0),
    so text findings come from reading, not from patterns.
- **The architecture reader**: one agent over the whole crate, Fable
  5.2 at `high`. It reads every `mod.rs` and module doc, the public
  listing, `core.md`, the inventories and `consumer-test.md`, and code
  where a boundary needs it. It judges module boundaries, the layering
  of the suites over the shared parts (D4: what of the linear suite the
  others name), where a trait or a generic would remove duplication,
  where an abstraction costs more than it saves, the engine interface
  an outside crate would need (X4), the crate's shape (one crate or
  several, and whether suites could load as modules of their own in the
  browser, since the bindings' module grew to 2 987 284 bytes at gzip),
  the gaps the last steps' reports name (`rules_candidates`, the
  protocol's `KINDS` without a kind for lambda-terms), and the
  features. It runs before the finders and does not see their
  findings. It writes `architecture.md` and its findings.
- **Finders**: one per unit, Opus 5.5 at `high`, read-only. A finder
  reads its whole unit file by file, keeping notes in a file as it
  reads (its context may be compacted), then reports at most 25
  findings, the most valuable first, each in the form below. A
  performance claim from reading is marked a candidate (P1).
- **Verifiers**: one per finder, on another model than its finder's:
  Fable 5.2 at `medium` for the Opus finders, Opus 5.5 at `high` for
  the architecture reader. A verifier is told to refute. For each
  finding it opens the file at the line and checks that the witness is
  there and shows what is claimed, that both places of a duplicate
  exist and do the same, that the criterion says what the finding says
  it does and is not settled by a machine or by a decision in a rules
  file, and that the proposed change would not break a standing
  criterion (S1 to S19). Its verdict is confirmed, confirmed as a
  candidate to measure, or rejected with the reason.
- **Measurements**: the leader runs the confirmed performance
  candidates worth it on the repository's instruments (a journey under
  callgrind or DHAT, a target row's counters, pinned time through the
  planning session). A candidate that is not measured stays one in the
  findings and is ranked as such.

No agent runs at `max`, in fast mode or with the advisor.

**Where the results are.** Each agent writes its result as a file in
`target/audit/units/UNIT/` of the audit's workspace (`notes.md`,
`findings.md`, `verdicts.md`; the architecture reader's
`architecture.md`), in the layout `brief.md` fixes, and returns a short
index to the workflow. The files are the record; the leader copies them
under `plan/notes/audit/units/` after each stage, so that they are
committed.

## A finding's form

| field | content |
|---|---|
| unit, dimension | as above |
| criterion | one identifier of `criteria.md` or of the rubric |
| place | file and line; for a duplicate, both |
| claim | one sentence |
| witness | a snippet, a measurement with its command, a use that fails, or the duplicate |
| change | what to do, in a sentence or two |
| kind | refactor (behaviour kept), API break, search change, text, build |
| value | who gains what: a user of the library, of the command, a reader of the code, the machine |
| cost | small (one file), medium (one module), large (across modules or crates) |
| net | which instrument of B1 shows that behaviour is kept, and whether the change touches a checker, a search or a certificate (then the panel runs in phase D) |

## Stages

Each is one workflow of about ten agents; the leader reads each stage's
result, updates the list of rejected findings (`rejected.md`) and
corrects the next briefs before starting the next. The
`workflow-authoring` skill is loaded before the first script is
written.

| stage | agents | what |
|---|---|---|
| 0 | the leader's tools, 1 Sonnet | the baselines again, the inventories |
| 1 | 1 Fable, 1 Opus | the architecture reader and its verifier |
| 2 | 5 Opus, 5 Fable | units 1a, 1b, 6a, 7a, 9, each verified as it finishes |
| 3 | 5 Opus, 5 Fable | units 2a, 2b, 3, 4a, 6b |
| 4 | 5 Opus, 5 Fable | units 4b, 4c, 4d, 4e, 5a |
| 5 | 6 Opus, 6 Fable | units 1c, 2c, 4f, 5b, 7b, 8 |
| 6 | the leader | measurements, deduplication, `findings.md`, `api-proposal.md` |

Stage 2 holds the units the API proposal rests on most (the data
model, the proofs, the shared layers, the command's interface, the Rocq
library), so that the proposal's outline exists while the rest is read.
After stage 2 the leader reports its tokens per model to the planning
session, which decides the size of stages 3 to 5 against the account's
weekly limit; Fable stays under half of what the run uses.

## Cost

Forty-five agent runs: 21 finders and the architecture reader's
verifier on Opus 5.5 at `high`; 21 verifiers and the architecture
reader on Fable 5.2; one on Sonnet 5.5. A unit's source is 300 to 550
kB, which is 100 000 to 180 000 tokens to read once, with 20 000 to
40 000 of criteria and rules beside it; a verifier reads the cited
places only. Phase A put the run at 8 to 10 million tokens on Opus and
2 to 3 million on Fable. Those numbers are estimates from the units'
sizes, not measurements, and how far cached context counts is not
known before a run; stage 2's real use decides.

A leaner run, if the planning session asks for one: twelve units of up
to 20 000 lines that are skimmed along the inventories instead of read
whole, in three stages. It costs about half and finds what the
inventories point at; it would miss what only reading shows, which is
most of idiom and text. The author asked for the entire libraries, so
the full grid is the plan.

## Synthesis

- **`findings.md`**: the confirmed findings, deduplicated across units
  (one finding with all its places where a pattern repeats), ranked by
  value against cost and risk: first what many users gain from at small
  cost and with an instrument that covers it; last what is large, gains
  only tidiness, or touches a checker. The rejected findings follow,
  each with the verifier's reason.
- **`api-proposal.md`**: the 0.9.0 surface for the author's one
  sign-off, crate by crate and module by module: what stays, what is
  renamed or moved, what becomes private, what is added (the
  constructors and the engine interface that `consumer-test.md` found
  missing, if the audit confirms them), each with its reason and the
  practice it follows; the command's flags under D1; the features; the
  Rocq library's interface; and what each change costs the bindings and
  the client.

## The rules every agent's brief carries

- Read-only: no edit of the repository, no jj command that writes, no
  git, no `gh`, no push. An agent writes only its own result files
  under `target/audit/units/UNIT/`.
- Files are read with the Read tool or `grep`, `head`, `tail`, `cut`,
  `wc`; never `sed` or `awk`, no `VAR=…` prefix, no `$` in a pattern.
- Anything that builds or runs is in a capped scope on cores 13 to 15
  (`systemd-run --user --scope -p MemoryMax=8G -p MemorySwapMax=0
  taskset -c 13-15 …`, `CARGO_BUILD_JOBS=3`), bounded in size and time,
  and no job count exceeds three; no `nix` command; a timed run is
  asked of the leader, who asks the planning session.
- The agent's model and effort, named in the brief.
- Report only what breaks a criterion, with its witness; no finding for
  what a lint, a check or a recorded decision settles.
