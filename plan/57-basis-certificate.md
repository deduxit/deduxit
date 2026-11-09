# Step 57: a checked basis for non-coverability

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`).
CLAUDE.md applies throughout: jj only (never git), thematic commits as
soon as a unit is done, doc comments on every item, the verification
table, no pushing. This prompt was written on 2026-11-08 by step 46's
second session, from the author's decisions of that day, and is finished
at the review of step 53: it says what is fixed.

Read `plan/notes/research/next/scope-semantics-decision.md`
(semantics-decision-1 and -6), `.claude/rules/core-horn.md` (the
backward search for coverability in affine mode), `core-search.md`
(`Disproof::check`), the reports of steps 27 and 51, and `plan/README.md`'s
D19, D27 and D31.

## Goal

The Horn engine's "not provable" in affine mode, a non-coverability
found by backward search, carries a certificate that is checked without
the search: the upward-closed set the backward search saturated,
given by its finite basis, shown closed under the program's clauses
backwards and not containing the initial marking. The check is
quadratic in the basis at most; it joins `Disproof::check` as a checked
refutation, as step 37's saturated set did for the inverse method.

## Fixed now

- **A check, then perhaps a lemma**: this step writes the certificate
  and its check in Rust; a Rocq lemma for it is kept for later (the
  research ranks it after step 55's), said in `later.md`.
- **The check is a checker** (`conduct.md`): its integers (markings,
  counts) cannot wrap, said at each declaration or refused; the panel of
  step 26's prompt reviews it, a planted wrong basis refused by a test.
- The search does not change (the target sets' counters equal); the
  basis is what the backward search already holds.

## How the step runs

One to two sessions on Opus 5.5 at `high`, the panel for the check.
Deliverables: the commits, the manual's paragraph and the guide's line,
`plan/reports/57-basis-certificate.md`.
