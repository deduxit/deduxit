# Step 58: maintenance, and Kani on the checker's integers

You are working in linlog (`deduxit/deduxit`) and, for the
accessibility audit, in the web client (`deduxit/deduxit.github.io`,
`../linlog-prover.github.io`). CLAUDE.md applies throughout: jj only
(never git), thematic commits as soon as a unit is done, the
verification table, no pushing. This prompt was written on 2026-11-08 by
step 46's second session, from the author's decisions of that day, and
is finished at the review of step 53: it says what is fixed.

Read `plan/notes/research/next/maintenance.md` (candidates 1 to 4 and
9), `verification.md` (sections 1 and 2, candidate 6), `web-teaching.md`
(candidate 4), `plan/later.md` ("Follow-ups: the web bindings": Clippy
for wasm32 and the unit suite on 32 bits), `.claude/rules/flake.md`,
`ci.md`, `web.md`, `evidence.md`, the `new-tool` skill, and
`plan/README.md`'s D10, D31 and `conduct.md`'s paragraph on numbers that
soundness rests on.

## Goal

What keeps the collection cheap to maintain as it grows, written down
and checked, and the second step of decision 8 (D31): Kani harnesses
that check, exhaustively within a bound, the integer and index facts the
proof checker's soundness rests on, the one fault class the project has
met (a counter that wrapped, `conduct.md`).

## Fixed now

- **Policies, written where the next session reads them**: the MSRV and
  the update cadence (for example "stable minus two, moved at each
  `update-deps`"; maintenance-1), a scheduled advisory check between the
  `update-deps` runs (the actions are watched already by
  `.github/dependabot.yml`; maintenance-2's rest), a wasm feature
  baseline checked on the built module with a policy for wasm-bindgen
  (maintenance-3), and a budget for crates and the module's size
  (maintenance-9) against step 47's figures.
- **Kani** (verification-6), adopted through the `new-tool` skill at the
  version a lock pins: harnesses for the checker's counters, sums and
  arena indices (`core/src/proofs/check/`), each stating the fact it
  checks and its bound; a flake check or a CI job for them as the tool's
  toolchain allows (a nightly is the fuzz devshell's precedent), never
  in the gate if it makes the gate slow.
- **The wasm leftovers**: Clippy for `wasm32-unknown-unknown` and the
  unit suite on a 32-bit target where it runs without threads.
- **The accessibility audit** (maintenance-4, web-teaching-4): axe or
  Lighthouse on every page of the client, the findings fixed or listed,
  and a screen-reader pass over the drawings' text alternatives; a
  structural twin of the drawing only if the pass shows it is needed.
- Deductive verification of the checker is not this step's (D31: after
  the checker is refactored for it, the tool chosen by a trial in Verus
  and Creusot; `later.md`).

## How the step runs

Two sessions on Opus 5.5 at `high` (the repository's policies, Kani and
the wasm checks; the client's audit); a fresh-context reviewer on Opus
5.5 at `high` checks that each harness states a fact the checker relies
on and that its bound is said. Deliverables: the commits,
`plan/reports/58-maintenance.md`.
