# Step 54: intuitionistic logic decided natively, by Dyckhoff's LJT

You are working in the linlog repository (`deduxit/deduxit`).
CLAUDE.md applies throughout: jj only (never git), thematic commits as
soon as a unit is done, doc comments on every item, the verification
table, no pushing. This prompt was written on 2026-11-08 by step 46's
second session, from the author's decisions of that day, and is finished
at the review of step 53: it says what is fixed, and the planning
session adds what steps 47 to 53 left, the oracles and the sessions'
goals then.

Read `plan/notes/research/next/ordinary-sequents.md` (sections 2 and 3,
candidate 1), `plan/later.md` ("Ordinary logic through its embeddings":
the loop check; "Follow-ups: ordinary logic"), the report of step 25
(the ILTP measurement), `.claude/rules/core-ordinary.md`, and
`plan/README.md`'s D17, D19, D24 and D27.

## Goal

Every propositional intuitionistic (and minimal) sequent gets a verdict
from a search that always terminates: a native engine on Dyckhoff's
contraction-free calculus LJT (G4ip), beside the embedding into linear
logic, whose copy bound answers "unknown" where the logic is decidable.
Today the embedding decides 108 of ILTP's 274 propositional problems at
2 s (step 25's measurement), and a loop check would refute 35 more.

## Fixed now

- Release 0.4.0 is "intuitionistic logic decided, with Kripke
  countermodels, tableaux and truth tables" (D27); this step is its
  engine, step 55 its countermodels.
- **No verdict without a check**: the engine's proof is an LJ derivation
  that `ordinary::Derivation::check` accepts (G4ip's rules are derivable
  in LJ; the translation is sourced, or the checker gains the rules with
  the panel's review), and its "not valid" is what step 55 certifies;
  until then it is the search's word, said so, as today's
  `Exhausted` is.
- **The search terminates by the calculus, and stays within its
  bounds**: `Limits`, the stop poll, no recursion on depth, the memory
  account; hard problems may still end at the time limit, said so (the
  research note: the search never runs without end, it may still time
  out).
- **The default by measurement** (D19): the engine becomes the ordinary
  layer's default for intuitionistic and minimal logic only where it
  wins on ILTP and the families, cross-checked against the embedding's
  verdicts by the harness (a `MISMATCH` is a bug); the embedding stays
  for classical logic and as an option.
- What the engine copies from the linear engines (an arena, a memo, a
  stop) is recorded in the report for step 67 (D25), not factored out.

## How the step runs

Two sessions on Opus 5.5 at `high`; the panel of step 26's prompt for the
engine and any checker change (argument on Fable 5.1 at `high`); the
harness's ILTP run and the target sets coordinated as timed runs.
Deliverables: the commits, the manual and the guide for AI models, the
oracles, `plan/reports/54-ljt-engine.md`.
