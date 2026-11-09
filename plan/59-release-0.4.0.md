# Step 59: the release of 0.4.0

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and the Rocq library
(`deduxit/rocq-deduxit`). CLAUDE.md applies throughout; tagging and
publishing are the author's acts. This prompt was written on 2026-11-08
by step 46's second session and is finished at the review of step 58: it
says what is fixed.

Read `plan/53-release-0.3.0.md` and its report (the release from the new
home, and what Zenodo did), `.claude/rules/ci.md`, `CHANGELOG.md` and
the reports of steps 54 to 58.

## Fixed now

0.4.0 is "intuitionistic logic decided, with Kripke countermodels,
tableaux and truth tables" (D27): steps 54 to 58. As 0.3.0's: `breaks
0.3.0` and every break in the changelog, both repositories' versions,
`status.md` true, the guide within its budget, `nix flake check` in
both, the release workflow's dry run, the release notes drafted, the
author's checklist. A baseline night is the author's to give if they
want the engines' times recorded for the release (the LJT engine is new);
otherwise the target sets and the harness's ILTP run stand.
One session on Opus 5.5 at `high`. Deliverable:
`plan/reports/59-release-0.4.0.md`.

## The author's wording of the collection (2026-11-09)

From 0.4.0, when intuitionistic logic is decided by its own engine and
no longer only through linear logic:
- **No logic is first.** The README, the manual's introduction, the
  crate docs, the guide for AI models, `CITATION.cff`'s abstract and the
  Cargo descriptions describe deduxit as a collection of logic suites,
  linear logic one of them. Drop "linear logic first" and every other
  wording that singles it out.
- **The name linlog is phased out.** It stays in exactly one history
  note in README and the manual's introduction ("It began as linlog, a
  linear logic library (up to 0.2.0), and is growing to take in more
  calculi."), in the final `compat/` crates, in the changelog's past
  sections and in the reports. Everything else that still says linlog
  (the Cachix cache `linlog-prover` and the machine lock's path aside,
  which are infrastructure) moves to deduxit.
- **The dictionary line** stays directly under each title, with no
  lead-in.
