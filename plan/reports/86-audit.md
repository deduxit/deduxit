# Step 86: the audit of the Rust and Rocq libraries (report)

Run on 2026-11-27 by one session on Fable 5.2 at `high` ("Rust and Rocq
library audit"), in the workspace `../deduxit-wt-audit`, from
`plan/86-audit-refactor.md`. Phases A to C are done; phase D, the
refactor, waits for the author's answer to the API proposal and the
planning session's go.

## What was done

- **Phase A** (beside steps 82 to 85): the criteria, each read from its
  source that day (`plan/notes/audit/criteria.md`); the method
  (`method.md`); the baselines; and a consumer test, two toy engines
  built on the public API from a crate outside the workspace, with
  thirteen points where the API got in the way (`consumer-test.md`).
- **Phase B** (after step 85's library part, at `vqwqlqkl`): the
  baselines again at that commit, the inventories, and the audit in
  five stages of agents; then the synthesis.
- **Phase C**: the refactor's plan.

What it left, all under `plan/notes/audit/` unless said:

| file | what |
|---|---|
| `findings.md` | the 371 findings that stand, in 45 ranked themes, and the 8 rejected |
| `findings-index.md` | all 379 in one table |
| `api-proposal.md` | the 0.9.0 surface in 25 parts, for the author's sign-off |
| `plan/87-refactor.md` | the refactor in 49 steps: 28 that keep the API, then 21 that change it |
| `units/UNIT/` | each unit's findings, verdicts and notes; `arch/architecture.md` is the map of the crate |
| `baselines.md`, `inventories/` | the numbers at the refactor's base, and the tools' lists |
| `standing.md` | the recorded decisions a proposal must answer |
| `rejected.md`, `handovers.md` | what the verifiers refuted; what one unit asked of another |
| `criteria.md`, `method.md`, `brief.md`, `consumer-test.md` | what the audit judged against and how |
| `journal.md`, `workflow.md`, `stage5-args.json` | where the work stood at the pause, and the workflow's script |
| `plan/audit/` | the scripts: the inventories (`stage0.sh`), the units, the journal, the index and the themes |
| `bench/targets/86.csv`, `net-86.csv` | the target sets at the base, equal to their oracles |

## The baselines

At `vqwqlqkl`: both target sets equal their oracles (no verdict and no
counter differs on 145 and 64 rows), so the refactor starts from the
searches the oracles recorded. The ratchet is within its tolerance on
all twenty journeys. Compile and test times, the web module's bytes,
the public API's size (531 distinct items, 93 under several paths, 82
unused outside the crate), Clippy's allowed lints (3 184 sites),
duplicate code (173 clones of 100 tokens or more) and each journey's
allocations are in `baselines.md`. A second measurement during the
synthesis attributed the web module's bytes by the library's modules,
before and after `wasm-opt` (`inventories/wasm.md`, `wasm-opt.md`).

## The workflow's shape

Twenty-one units of at most 14 500 lines of source (the plan had
twenty; steps 82 to 85 grew the core crate by 7 000 lines, and three
units outgrew the bound). Each unit: one finder (Opus 5.5 at `high`)
that read all of it for five dimensions, then one verifier (Fable 5.2
at `medium`) told to refute each finding. Before them, one architecture
reader over the whole crate (Fable 5.2 at `high`), verified by Opus 5.5
at `high`. One workflow per stage, a finder and its verifier as a
pipeline per unit.

| stage | units | agents | minutes | findings | confirmed | candidates | rejected |
|---|---|---:|---:|---:|---:|---:|---:|
| 0 | the inventories, by tools and one Sonnet agent | 1 | | | | | |
| 1 | the architecture reader | 2 | 29 | 27 | 25 | 2 | 0 |
| 2 | 1a, 1b, 6a, 7a, 9 | 10 | 16 | 91 | 80 | 8 | 3 |
| 3 | 2a, 2b, 3, 4a, 6b | 10 | 13 | 82 | 69 | 9 | 4 |
| 4 | 4b, 4c, 4d, 4e, 5a | 10 | 13 | 80 | 65 | 15 | 0 |
| 5 | 1c, 2c, 4f, 5b, 7b, 8 | 12 | 14 | 99 | 86 | 12 | 1 |
| | | 45 | | 379 | 325 | 46 | 8 |

## The cost

Tokens by stage and model. "Written" is what entered the cache, "read"
what was read back from it; new input outside the cache was negligible.

| stage | Opus written | Opus read | Opus output | Fable written | Fable read | Fable output |
|---|---:|---:|---:|---:|---:|---:|
| 1 | 293 175 | 21 629 329 | 55 885 | 531 834 | 29 478 443 | 125 818 |
| 2 | 2 180 991 | 98 903 386 | 337 073 | 740 843 | 24 244 290 | 124 809 |
| 3 | 2 016 186 | 79 707 335 | 249 982 | 655 489 | 12 748 423 | 98 813 |
| 4 | 2 001 722 | 68 882 657 | 274 765 | 622 527 | 9 361 473 | 86 287 |
| 5 | 2 095 215 | 85 455 869 | 317 501 | 845 066 | 12 976 042 | 113 495 |
| all stages | 8 587 289 | 354 578 576 | 1 235 206 | 3 395 759 | 88 808 671 | 549 222 |

Beside the stages: the leader's own session in phases B and C (Fable:
about 0.7 M written, 88 M read, 0.34 M output when this was written),
one Sonnet agent (79 157 in all), a second verification of unit 9 and
the review of the proposal (one agent each). Fable was about a third of
the run, the leader included.

Phase A had estimated 8 to 10 M tokens on Opus and 2 to 3 M on Fable.
That held for what was written and output; the cache reads came on top,
because each of an agent's 40 to 80 requests reads its whole context of
300 000 to 400 000 tokens again. From stage 3 the finders were told to
read several files per request, which cut the requests by a quarter and
the cache reads per unit from 24.6 M to between 15.6 and 18.5 M.

## What the audit found

`findings.md` has it by theme. In short:

- **One soundness defect of a checker**: the first-order Kripke
  countermodel's check accepted `=` unread, so `deduxit check` accepted
  a forged refutation of `⊢ a = a` (F-4d-01; reproduced by the finder,
  the verifier and the leader). With an unbounded write in first-order
  resolution (F-4c-01) it went to fixup-31 the same hour. No other
  finding shows a wrong "proved" or "valid".
- 33 more findings are small defects: calls that do not keep the
  crate's rule on limits and stops, front ends that answer wrongly
  because each chooses by hand what the library should choose once,
  calls that mislead or panic on a valid use.
- 96 findings are the public API: what all suites use still lives in
  the linear suite; an engine cannot be written outside the crate; the
  suites name the same things differently. They became the proposal's
  25 parts.
- 90 are duplication, 57 performance (of which 46 are candidates that
  wait for a measurement, most in code no journey runs), 26 structure,
  38 text, 18 the Rocq library, 11 the command and the build.

## What the verifiers rejected, and why

Eight of 379 (`rejected.md` has each with its reason):
- three for a recorded choice whose reason still holds (the message of
  `GlivenkoFirstOrder`, the batch's record assembled as text, a lint
  set aside on purpose);
- three because the harm claimed does not exist (a sentinel compared
  with its own constant, a bare number that names no vertex, a unit of
  time no flag reads);
- one because the change gains its callers nothing (`Fitch::new`);
- one text finding whose words carry their own meaning.

The verifiers corrected far more than they rejected: in stages 3 to 5,
where a verdict has a field for it, 216 of the 256 findings that stand
carry a correction of their place, cost, kind or change, from a line
number to a change that would not have worked as written. That is why
the refactor's steps are told to read the verdict with the finding.
The architecture reader's verifier also found seven wrong statements of
fact in the map, and the recorded decisions that stand against several
of its proposals (`standing.md`).

## Departures from the method

- **Twenty-one units**, not twenty (above); stage 5 ran six.
- **The harness refuses a subagent's `Write` of a report file.** Stage
  2's finders could not write `findings.md`; the files were recovered
  from the refused calls in their transcripts (`plan/audit/recover.py`).
  From stage 3 the agents return their results through the workflow's
  schema, and `plan/audit/journal.py` writes the files.
- **Unit 9 was verified twice**: its first verifier started before the
  recovered file was on disk and judged from the one-line claims; the
  second, with the full text, confirmed a finding the first had
  rejected. The second stands.
- **Stage 3's launch was refused once** by the session's permission
  classifier, because the planning session's decision on the later
  stages had been recorded on main but not yet sent as a message; it
  went through with the message.
- **A pause** at 09:17 for the author's use of the machine: stage 5 had
  finished by itself two minutes before, so nothing was cut.
- **Twice a push of the planning session crossed a commit of this
  session** and left the working copy's changes on a divergent side;
  both were recovered whole.
- Two agents read a few line ranges with `sed` early in their runs,
  against the brief, and said so; nothing was changed by it.

## What is left open

- **The author's answer to `api-proposal.md`.** Four parts are real
  choices (P1 the command's selector, P5 the settings' keys, P18
  completions and a man page, P24 the features); the plan's steps name
  the parts they implement.
- **Measurements the audit did not make**: every performance candidate
  (the refactor's step 87a adds the journeys they need first); the
  engines' share of a build without default features (87j1); the web
  module after each mend (87d2, 87j2).
- **The client**: whether the bindings' own table of session calls has
  a reader must be checked against the client's main; the checkout
  beside this repository is older than the newer calls.
- **Not read**: the benchmark harness and the fuzz targets but as
  callers of the library; the web client; test files but where a
  finding needed one.
- **The Rocq library** was read only; its repository is untouched.

## For a later audit

- Size the cost by cache reads, not by what is written: a reader of a
  whole unit pays its context once per request.
- Tell the readers to batch their reads from the first stage.
- Give a verifier the findings' full text in its prompt; one that works
  from a one-line claim rejects what the witness would have shown.
- Have agents return results through a schema and write the files from
  the workflow's journal.
- Keep the hand-overs between units: the protocol's wrong answer for a
  net, the sessions' table of names and the bindings' repeated generics
  were each found by a later unit following an earlier one's pointer.
