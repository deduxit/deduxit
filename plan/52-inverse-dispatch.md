# Step 52: dispatch rows for the inverse method, where it wins

You are working in the linlog repository (`sequentium/sequentium` after
step 48). CLAUDE.md applies throughout: jj only (never git), thematic
commits as soon as a unit is done, doc comments on every item, the
verification table, no pushing. One session, mostly measurement. This
prompt was written on 2026-11-08 by step 46's second session, from the
author's decisions of that day. It is complete; the planning session
adds step 47's figures at its review.

Read `plan/notes/research/next/scope-engines.md` (engines-12, and
engines-5 on why no static feature was found), `plan/later.md` ("Found by
the growing families" under "Follow-ups: the benchmarks", and "Follow-ups:
intuitionistic mode", the entry of session 43c on what the inverse method
saturates at once), the report of step 37 ("The dispatch") and of step
45 (45b, the growing families), `.claude/rules/core-search.md` and
`core-inverse.md` (`DISPATCH`, `PROBE_AFTER`, `TURN_SHARE`),
`.claude/rules/bench.md`, `bench/TARGETS.md`, and `plan/README.md`'s D8
and D19.

## Goal

The default decides where an engine linlog already has is measured to
win (D19). The growing families found three classes where the inverse
method forced beats the default and no row of `DISPATCH` gives it a turn:
`wide-m1` (32 768 pairs in 47 s forced, against 16 384 in 35 s by the
default), classical `chain` (2 048 clauses in 4 ms, where the focused
engine meets the recursion limit from 1 024), and `pigeonhole` in
intuitionistic affine MALL without exponentials (6 holes in 0.27 s
against more than a minute). Each gets a row or a turn, earned by a
measurement, or the report says why not.

## Fixed now

- **A row is earned by measurement only** (D19): it must not lose a row
  the default decides in its class. Step 37 found no static feature that
  separates the inverse method's wins from its losses on the
  intuitionistic rows, and the generated families lose on it by
  fragment, so a row by fragment alone is not what is wanted: a turn
  (bounded by `TURN_SHARE`, as on the Mix row) is the cheap form, a row
  by a feature found in the measurement the other.
- **The measurement**: the target sets of step 47 for the counters
  (every decided row's counters equal where the change must not alter
  them, and every changed row named), the growing families at their
  sizes for the times, and a held-out set (rows not used to choose)
  that the choice is checked on; timed runs coordinated with the other
  sessions through `linlog-orchestrator`, on cores 2 to 15, never 0 or
  1, with the before and after on the same cores (they differ in
  speed).
- **What the harness gets wrong is fixed first**: the harness reads
  linlog's reason in the wrong order (`linlog_read` in
  `bench/src/tools.rs`, a follow-up of 45f in `later.md`), a small fix
  with its test before any number is read.

## Requirements

1. A row or turn for each class that wins, with its measurement in a
   comment at the row, in `docs/src/` where the dispatch is described,
   and in the guide for AI models if it names the dispatch.
2. The rows the default decides today decided no slower beyond noise on
   the target sets, by their counters and pinned CPU time
   (`bench/pinned-time.py`, cores 2 and 3).
3. The intuitionistic entry of session 43c (what the default leaves to
   the time limit that the inverse method saturates at once) answered by
   the same measurement: a row, a turn, or kept with its reason in
   `later.md`.

## How the step runs

One session on Opus 5.5 at `high`, by day, its timed runs coordinated.
The dispatch decides which engine runs, not whether a verdict is sound
(every engine's proofs are checked and its refutations are the ones
`Disproof::check` knows), so no panel; a fresh-context reviewer on Opus
5.5 at `high` checks the measurement against D19 (no lost row, the
held-out set, the same cores) and reports only what breaks it.
Deliverables: the commits (the harness fix, the rows, the oracles
`bench/targets/52.csv` and `net-52.csv` if the search's counters moved
on purpose, a commit of their own saying why), and
`plan/reports/52-inverse-dispatch.md`.
