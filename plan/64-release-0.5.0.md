# Step 64: the release of 0.5.0

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and, if it changed,
the Rocq library (`deduxit/rocq-deduxit`). CLAUDE.md applies
throughout; tagging and publishing are the author's acts. This prompt
was written on 2026-11-08 by step 46's second session and is finished at
the review of step 63: it says what is fixed.

Read `plan/59-release-0.4.0.md` and its report, `.claude/rules/ci.md`,
`CHANGELOG.md` and the reports of steps 60 to 63.

## Fixed now

0.5.0 is "custom rules and natural deduction" (D27): steps 60 to 63. As
the releases before it: `breaks 0.4.0` and every break in the changelog,
the versions, `status.md` true, the guide within its budget, `nix flake
check`, the release workflow's dry run, the release notes, the author's
checklist; the rule language's JSON Schema published with the release.
One session on Opus 5.5 at `high`. Deliverable:
`plan/reports/64-release-0.5.0.md`.

## What steps 60 to 63 left you (the review of 63c, 2026-11-16)

- **What 0.5.0 holds** (steps 60, 61, 62, 63, 63t and 63g, each with a
  report under `plan/reports/`): calculi given as data (rule files at
  wire level 15, LK, LJ, LM, LL, the signed tableau, NJ, NK, NM and
  variants), sessions in them backwards and forwards (levels 17, 18),
  Gentzen trees and Fitch proofs in SVG, LaTeX (`fitch` 1.0) and Typst
  (fitchcraft 1.0.1), the negative translations and interpolants (level
  16), `deduxit::protocol` and `wire.schema.json`, and the workspace
  graph (63g-b's library and command at level 19, 63g-c's page). The
  web client's courses by suite (63b, 63c) ship with the client, not the
  crates.
- **New features of the core crate**: `protocol` (on by default, brings
  `serde_json` as an optional dependency) and whatever 63g-b adds
  (`workspace`); `breaks 0.4.0` and the feature tables of the manual's
  installation chapter must agree.
- **The schemas published**: the rule language's (`rules::SCHEMA`) and
  `docs/src/wire.schema.json` (63a), both with the release.
- **The baseline of 2026-11-16** (`bench/results/2026-11-16/`) is the
  latest; the release notes may cite it.
- **The tags** (the author's yes of 2026-11-16 for 0.5.0): rocq-deduxit
  first if it moved since v0.4.0, then deduxit, signed tag objects made
  by the planning session after CI is green at the release commit; the
  crates.io publish waits for the author's approval in the
  `crates-io` environment.

## The session (planned at this review)

- **64**, after 63g-c has landed: one session on Opus 5.5 at `high`,
  everything above but the tags and the publish.
