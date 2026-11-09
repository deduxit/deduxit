# Step 54: intuitionistic logic decided natively, by Dyckhoff's LJT

You are working in deduxit (`deduxit/deduxit`, the checkout `../linlog`).
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

## What steps 47 to 53 left you (added at the review of step 53, 2026-11-09)

- **The name is deduxit.**
  - The crate is `deduxit`, the command `deduxit` (package
    `deduxit-cli`), the harness `deduxit-bench`, the guide
    `skills/deduxit/`.
  - The linear suite is `deduxit::linear`; ordinary logic stays
    `deduxit::ordinary`, beside it at the root.
  - The planning session is `deduxit-orchestrator`.
  - 0.3.0 is prepared, with its tags waiting for the author. Nothing in
    this step waits for the publication.
- **Where ordinary logic stands:**
  - step 49 gave it goal states (`deduxit::ordinary::interactive`), the
    command's `interact --logic`, and the page's Ordinary tab;
  - step 50 reads its LJ proofs back as typed lambda-terms
    (`deduxit::lambda`);
  - `ordinary::Derivation::check` is the checker your proofs must pass.
- **The oracles:** the target sets `bench/targets/47.csv` and
  `net-47.csv`, unchanged by 52 and 53, since no search changed. This
  step adds a search, so it adds rows of its own and keeps the linear
  rows' counters equal to 47's.
- **The harness reads JSON** (51b): the reason is the outcome's
  `reason.kind`.
- **Timed runs:** ILTP at 2 s and the families are timed runs. Ask
  deduxit-orchestrator for each slot (cores 2-3, under the machine
  lock), as step 52 did. The counters are compared without a slot.
- **Two client-side notes** for 54b, if it touches the page:
  - the client builds Biome without its tests (`modules/biome.nix`);
  - its CI check job has 180 minutes.

## The sessions (added at the review of step 53)

- **54a, the engine** (Opus 5.5 at high, the panel after it):
  - G4ip's search in `deduxit::ordinary` for intuitionistic and
    minimal propositional sequents, terminating by the calculus, within
    `Limits`, the stop poll and the memory account;
  - its proofs translated into LJ derivations that
    `ordinary::Derivation::check` accepts. The translation follows a
    cited source (Dyckhoff 1992, *Contraction-free sequent calculi for
    intuitionistic logic*; Dyckhoff and Negri 2000 for the rules'
    admissibility), or the checker gains the rules under the panel's
    review;
  - "not valid" stays the search's word, reported as the embedding's
    `Exhausted` is;
  - tests: every ILTP problem the embedding decides gets the same
    verdict from the engine, and a planted wrong translation is
    refused by the checker;
  - the library's API, docs and rules (`core-ordinary.md`).

  Not yet the command's default.
- **54b, the default by measurement** (Opus 5.5 at high, after 54a):
  - the harness's engine axis gains the LJT engine;
  - ILTP's 274 problems at 2 s and the families are run, both builds,
    as a timed run;
  - the default for intuitionistic and minimal logic moves only where
    it wins (D19), and the embedding stays for classical logic and as
    an option;
  - the command's option, the manual, the guide, the page's settings
    if they name engines, and the report
    `plan/reports/54-ljt-engine.md`.

## How the step runs

Two sessions on Opus 5.5 at `high`; the panel of step 26's prompt for the
engine and any checker change (argument on Fable 5.1 at `high`); the
harness's ILTP run and the target sets coordinated as timed runs.
Deliverables: the commits, the manual and the guide for AI models, the
oracles, `plan/reports/54-ljt-engine.md`.
