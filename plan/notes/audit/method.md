# The audit's method (step 86)

How phase B finds, verifies and ranks, fixed before any agent runs. The
criteria are `criteria.md`, the numbers `baselines.md`, the evidence of
an outside user `consumer-test.md`. Phase B starts at the planning
session's "go", after step 85 has landed; it reads and changes nothing.

## What phase B reads

The tree at main after step 85, in the audit's workspace
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
of its unit's source, tests apart: at most about 14 000 lines, one unit
of 16 000 where splitting would hide the duplication it is read for.
Lines are of `baselines.md` (source without test files) and move with
steps 82 to 85; the split is by module, so it stands.

| unit | area | modules | lines | rules files |
|---|---|---|---:|---|
| 1a | 1 | `linear/sequents`, `linear/parse`, `linear/fragment.rs`, `linear/occurrences`, `first_order.rs`, `name.rs` | 11 100 | `core-sequents`, `core-forest` |
| 1b | 1 | `linear/proofs`: `mod.rs`, `rule.rs`, `check/`, `derivation/`, `size.rs`, `fmt.rs`, `style.rs`, `multiset.rs` | 12 600 | `core-proofs`, `core-derivations` |
| 1c | 1 | `linear/proofs/cut`, `search/refutation.rs`, `saturated.rs`, `weighted.rs`, `basis.rs`, `classical.rs` | 7 000 | `core-proofs`, `core-search`, `core-inverse` |
| 2a | 2 | `search/mod.rs`, `memory.rs`, `work.rs`, `additive.rs`, `batch.rs`, `parallel.rs` | 8 000 | `core-search`, `core-batch`, `core-parallel` |
| 2b | 2 | `search/focus/` | 13 600 | `core-focus`, `core-parallel` |
| 2c | 2 | `search/horn/`, `search/inverse/`, `linear/families.rs`, `lltp.rs`, `mist.rs` | 8 300 | `core-horn`, `core-inverse`, `core-inputs` |
| 3 | 3 | `linear/nets/`, `search/net.rs`, `essential.rs`, `planar.rs` | 11 100 | `core-nets` |
| 4a | 4 | `ordinary`: `mod.rs`, `parse.rs`, `translate.rs`, `derivation.rs`, `decide.rs`, `bruijn.rs`, `instance.rs`, `equality.rs`, `model.rs`, `table.rs`, `negative.rs`; `kripke.rs` | 12 400 | `core-ordinary` |
| 4b | 4 | `ordinary`: `ljt.rs`, `sat/`, `normal.rs`, `interpolant.rs`, `tableau.rs`, `written/` | 13 900 | `core-ordinary` |
| 4c | 4 | `ordinary`: `connection/`, `resolution/`, `herbrand.rs` | 14 500 | `core-ordinary` |
| 4d | 4 | `ordinary`: `grounding.rs`, `models.rs`, `kripke_search.rs`, `kripke_structure.rs`, `structure.rs`, `rocq.rs`, `rocq_models.rs` | 7 900 | `core-ordinary` |
| 4e | 4 | the interactive states of every suite: `ordinary/interactive.rs`, `invert.rs`, `linear/proofs/interactive/`, `rules/interactive.rs`, `goal_tree.rs`, `unify.rs` | 16 000 | `core-derivations`, `core-ordinary`, `core-rules` |
| 5a | 5 | `modal/`, `lambda/` | 13 800 | `core-modal`, `core-lambda` |
| 5b | 5, 6 | `rules/` but its interactive state, `protocol/`, `workspace/` | 13 700 | `core-rules`, `core-protocol`, `core-workspace` |
| 6a | 6 | `errors/`, `limits.rs`, `settings.rs`, `wire.rs`, `serialize/`, `lib.rs`, `hash.rs` | 13 000 | `core`, `core-sequents` |
| 6b | 6 | `export/` | 13 700 | `core-export` |
| 7a | 7 | the command's interface: `argument_parsing.rs`, `lib.rs`, `main.rs`, `settings.rs`, `io.rs`, `limit.rs`, `style.rs`, `seq.rs`, `prove.rs`, `check.rs`, `batch.rs`; every `--help`; the manual's command chapters | 10 300 | `cli` |
| 7b | 7 | the command's other files: `interact/`, `show.rs`, `ordinary.rs`, `modal.rs`, `calculus.rs`, `eliminate.rs`, `grade.rs`, `hint.rs`, `resolve.rs`, `tableau.rs`, `term.rs`, `transform.rs`, `workspace.rs` | 10 800 | `cli` |
| 8 | 8 | `web/`, `web-render/`, `web-tab/` as a consumer of the API; how `bench/src` and the fuzz targets call it | 7 500 | `web`, `bench` |
| 9 | 9 | `rocq-deduxit`: `theories/`, `tests/`, `bridges/`, the opam file, the project file, the flake's checks; the exporter's preludes for Q8 | 11 100 | the library's `rocq.md` and CLAUDE.md |

Twenty units. The rules files are long (`core-ordinary.md` has 2 650
lines); a finder reads the parts on its modules, since a rules file
records choices made on purpose, and a finding against one must say why
its reason no longer holds.

## The agents

Every brief carries the machine's rules (below), the unit, the criteria
file, the unit's rules files, `consumer-test.md`, the list of rejected
findings so far, the finding's form, and its model and effort.

- **Inventories** (stage 0): plain tools run by the leader, and Sonnet
  5.5 at `medium` where a list needs reading. They give every finder
  facts instead of impressions:
  - the public items (`cargo public-api`, as in `baselines.md`), the
    items reachable under several paths, and each public item's uses
    outside its module in the workspace's other crates, tests and doc
    examples: the candidates for `pub(crate)` (X9);
  - public items without an example, from the rustdoc JSON (C-EXAMPLE);
  - the functions over 100 lines with their lengths (Clippy's
    `too_many_lines`, 209 today) and the files over 1 000 lines (E5);
  - duplicate code by a clone detector: PMD's CPD is in nixpkgs (7.28);
    whether it reads Rust is checked first, a token-based script
    otherwise (E6);
  - sentences that match the patterns of `writing.md`, rule 4, in doc
    comments, help texts and messages, each then read by a Sonnet agent
    that drops the false matches (T4);
  - the sizes of the types the journeys allocate most
    (`-Zprint-type-sizes`), the allocation sites of each journey (DHAT,
    `valgrind --tool=dhat` on `deduxit-bench journey`), and the
    functions with the most LLVM IR (`cargo llvm-lines`, in nixpkgs)
    (P2, P4, P5);
  - every subcommand's `--help`, and the flags as a table: name, short
    form, value, default, the subcommands that take it (L2, L6, L8);
  - for the Rocq library: the theorems `tests/Assumptions.v` prints
    against all theorems, the warnings of a build with every warning
    on, a build with `-mangle-names`, the build's time per file (Q5,
    Q6, Q10).
- **The architecture reader**: one agent over the whole crate, Fable
  5.2 at `high`. It reads every `mod.rs` and module doc, the public
  listing, `core.md`, the inventories and `consumer-test.md`, and code
  where a boundary needs it. It judges module boundaries, the layering
  of the suites over the shared parts (D4: what of the linear suite the
  others name), where a trait or a generic would remove duplication,
  where an abstraction costs more than it saves, and the engine
  interface an outside crate would need (X4). It runs before the
  finders and does not see their findings.
- **Finders**: one per unit, Opus 5.5 at `high`, read-only. A finder
  reads its whole unit, then reports at most 25 findings, the most
  valuable first, each in the form below. A performance claim from
  reading is marked a candidate (P1).
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

Each is one workflow of at most ten agents; the leader reads each
stage's result, updates the list of rejected findings and corrects the
next briefs before starting the next. The `workflow-authoring` skill is
loaded before the first script is written.

| stage | agents | what |
|---|---|---|
| 0 | the leader's tools, up to 4 Sonnet | the baselines again, the inventories |
| 1 | 1 Fable, 1 Opus | the architecture reader and its verifier |
| 2 | 5 Opus, 5 Fable | units 1a, 1b, 6a, 7a, 9, each verified as it finishes |
| 3 | 5 Opus, 5 Fable | units 2a, 2b, 3, 4a, 6b |
| 4 | 5 Opus, 5 Fable | units 4b, 4c, 4d, 4e, 5a |
| 5 | 5 Opus, 5 Fable | units 1c, 2c, 5b, 7b, 8 |
| 6 | the leader | measurements, deduplication, `findings.md`, `api-proposal.md` |

Stage 2 holds the units the API proposal rests on most (the data
model, the proofs, the shared layers, the command's interface, the Rocq
library), so that the proposal's outline exists while the rest is read.

## Cost

Forty-six agent runs: 20 finders and the architecture reader's verifier
on Opus 5.5 at `high`; 20 verifiers and the architecture reader on
Fable 5.2; up to 4 on Sonnet 5.5. A finder reads about 150 000 tokens of
source and 20 000 to 40 000 of criteria and rules, and works over it for
perhaps 300 000 to 500 000 tokens with cache reads; a verifier reads the
cited places only, about 100 000. That is of the order of 8 to 10
million tokens on Opus and 2 to 3 million on Fable, the Fable share well
under half. The numbers are estimates from the units' sizes, not
measurements; the leader reports stage 2's real use to the planning
session before starting stage 3, and the later stages are sized by it.

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
  git, no `gh`, no push.
- Files are read with the Read tool or `grep`, `head`, `tail`, `cut`,
  `wc`; never `sed` or `awk`, no `VAR=…` prefix, no `$` in a pattern.
- Anything that builds or runs is in a capped scope on cores 13 to 15
  (`systemd-run --user --scope -p MemoryMax=8G -p MemorySwapMax=0
  taskset -c 13-15 …`, `CARGO_BUILD_JOBS=3`), bounded in size and time,
  and no job count exceeds three; `nix build` only under `flock -o
  /tmp/linlog-machine.lock`; a timed run is asked of the leader, who
  asks the planning session.
- The agent's model and effort, named in the brief.
- Report only what breaks a criterion, with its witness; no finding for
  what a lint, a check or a recorded decision settles.
