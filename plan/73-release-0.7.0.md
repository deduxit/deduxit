# Step 73: the release of 0.7.0

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and the Rocq
libraries that changed. CLAUDE.md applies throughout; tagging and
publishing are the author's acts. This prompt was written on 2026-11-08
by step 46's second session and is finished at the review of step 72: it
says what is fixed.

Read `plan/69-release-0.6.0.md` and its report, `.claude/rules/ci.md`,
`CHANGELOG.md` and the reports of steps 70 to 72.

## Fixed now

0.7.0 is "resolution and the simply typed lambda calculus" (D27): steps
70 to 72, the last release the roadmap of 2026-11-08 plans. As the
releases before it: `breaks 0.6.0`, the versions, `status.md` true, the
guide within its budget, `nix flake check`, the release workflow's dry
run, the release notes, the author's checklist. Its report ends with
the state of every item `plan/later.md` keeps for after 0.7.0, for the
next research step the author starts (D30, D31: the research items and
the deferred verification). One session on Opus 5.5 at `high`.
Deliverable: `plan/reports/73-release-0.7.0.md`.

## What steps 70 to 72 left you (written 2026-11-20, while 71c runs)

- **What 0.7.0 holds**: normal forms (`seq normalize`, wire 21) and
  resolution and DPLL (`resolve`, `check --dimacs`, wire 22) of step 70,
  with the page's sheets; the simply typed lambda calculus (71a's
  parser and checker, wire 23; 71b's `term check|inhabit|enumerate`, the
  suite `lambda`, wire 24; 71c's drawings and page, landing now); the
  dataset and `grade` of step 72; the CI rework (ci-speed: CI as a
  matrix, Pages from CI, the release through `checks.yml`); 54x8's
  ordinary auto-reverse on the page.
- **The release workflow changed** (ci-speed): `release.yml` calls
  `checks.yml` and builds its packages beside the checks; this is its
  first release run that way. Install after the release now tests the
  release's own files and asks for the release with the job's token.
  Watch both in the dry run as far as one can be run locally.
- **The changed JSON of `--lambda`** (71a: a variable's `name`, a
  `version` key) is under "Changed"; `breaks 0.6.0` will list API
  additions only if nothing else moved.
- **The Rocq library** has not moved since v0.6.0 but for a comment
  (`twvrwvrk`): no Rocq release unless a theorem changed.
- **The tags are held for the author** (unattended: the yes of
  2026-11-20 covered 0.6.0 only). Prepare everything up to them.
- **The web module's ceiling** is 3 900 000 bytes at gzip (71b).

## The session (planned 2026-11-20)

- **73**, now, beside 71c: everything but the tags and the publish;
  71c's changelog entries folded in when it lands, as 69 folded 54x7.
