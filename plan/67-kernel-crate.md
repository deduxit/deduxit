# Step 67: the kernel crate, if the suites show what they share

You are working in the linlog repository (`sequentium/sequentium`).
CLAUDE.md applies throughout: jj only (never git), thematic commits as
soon as a unit is done, doc comments on every item, the verification
table, no pushing. This prompt was written on 2026-11-08 by step 46's
second session, from the author's decisions of that day, and is finished
at the review of step 66: it says what is fixed.

Read `plan/notes/research/next/structure.md` whole, the records of what
each suite copied (the reports of steps 49, 54, 60 and 65), step 47's
build figures, `.claude/rules/core.md`, and `plan/README.md`'s D17, D18
(the API after a release), D24 and D25.

## Goal

One copy of what the suites share, extracted only now that a second
non-embedded suite (the modal suite, step 65) has shown what it is
(D25): a kernel crate holding the generic parts, the suites depending on
it, every published path of `linlog` still working.

## Fixed now (D25 and the structure note's structure-5)

- **The gate**: the session first lists, from the suites' records, what
  each copied; it extracts only what at least two suites copied. If that
  is too little to earn a crate, the step ends there, says so, and is
  kept in `later.md` with the list.
- **The boundary's candidates** (structure-5): the error family,
  `Limits`, `Progress`, `Refusal`, `Settings` and `Clock`, the wire
  policy, the checker's shape and its observers, the export layouts
  behind a small trait for a derivation tree, interactive proving's
  goals and undo, the front door's `Outcome`, `Verdict` and `Reason`,
  the parallel runtime. The arena, the occurrence forest and the proof
  term stay in `linlog`, so the propositional speed does not move (D17:
  the target sets' counters equal, the ratchet within its ceilings).
- **The crate** starts as an unpublished path crate at version 0.0.0,
  published only when a second suite crate depends on it; `linlog`
  re-exports at the old paths, checked by `breaks` against the last
  release with no break; the workspace moves to a flat `crates/` only
  when a second crate beyond `linlog` exists, `default-members` the
  command and the library.

## How the step runs

Three to five sessions: the first (the inventory and the boundary,
judged against the records) on Fable 5.1 at `high`, the extraction on
Opus 5.5 at `high`, each session's D17 check. Deliverables: the commits,
CLAUDE.md and `.claude/rules/` for the new layout,
`plan/reports/67-kernel-crate.md`.
