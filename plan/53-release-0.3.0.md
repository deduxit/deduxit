# Step 53: the release of 0.3.0, the first from `sequentium`

You are working in linlog (`sequentium/sequentium`) and, for its release,
in the Rocq library (`../rocq-linlog`, `sequentium/rocq-linlog`).
CLAUDE.md applies throughout: jj only (never git), thematic commits as
soon as a unit is done, doc comments on every item, the verification
table, no pushing. Tagging and publishing are the author's acts. The
step has two sessions and comes after steps 47 to 52 are done, reviewed
and on `main`. This prompt was written on 2026-11-08 by step 46's second
session, from the author's decisions of that day. It is complete; the
planning session adds what steps 47 to 52 leave at their reviews.

Read `plan/45-release-0.2.0.md` and its report (how 0.2.0 was made: the
release prepared, the author's acts last, the Rocq library's release
beside it, Zenodo), `plan/notes/distribution.md`, `.claude/rules/ci.md`
(the release workflow, Trusted Publishing), `CHANGELOG.md`, the reports
of steps 47 to 52 (and 48's table of addresses), and the `later.md`
entries named below.

## Goal

0.3.0 is "ordinary logic on the page": LK and LJ step by step in the
command and on the page (step 49), proofs read back as lambda-terms (50),
two more kinds of refutation checked and certified (51), the inverse
method's rows (52), all from the new home (48). It is a 0.y bump because
new variants of closed enums are planned breaks (`core.md`,
"Decisions"); the release says each break plainly. Its second purpose is
to prove the chain after the move: crates.io's Trusted Publishing from
`sequentium/sequentium`, the GitHub release, and Zenodo's version records
joining (or not) the concept DOIs.

## Session 53a: the small defects, before the release

Opus 5.5 at `high`. The open defects of `plan/later.md` that a first user
meets, each a small fix with one focused test, and each closed in
`later.md` with the commit; one that turns out larger is kept there with
its reason:

- the command: `linlog check FILE` ignores the file's own `"mode"`;
  `check -i --mix --net` prints "proof net with Mix" without checking the
  mode; `eliminate --net --output FILE` without `--format` writes
  nothing; `linlog eliminate` names a cut by its raw id (`cut on #8`)
  and has no flag to name it; `interact`'s `cut G C P…` reads the formula
  as one word; the eigenvariable refusal message at `∃L` in the
  two-sided reading; `--style labels.table.RULE=LABEL` fails on an unset
  table and the rule keys are not listed; `check --file` (`check` takes
  the proof as an argument only);
- the library: the division by zero in
  the rules look's label narrowing at a zero-width advance; the essential
  net's input side drawn green by `svg::Style`'s default, and its drawing
  turned off for nets with cut links; the panic past the proof
  structure's vertex bound (`nets::MOST`), made a refusal in
  `Nets::admits` with its test; the batch's and `check`'s records
  writing `wire::LEVEL` as a constant; the derivation builder's
  recursion to the derivation's height; the unused `Member` import in
  `search/focus/tests.rs` without `parallel`; the test
  `a_raised_limit_holds_on_the_pool` overflowing the test thread's stack
  in a debug build;
- the documents: the library's front page whose examples read `Instant`,
  and where the Euler Math font's source is said; the Rocq entries of
  "Follow-ups: a first user's obstacles" (a refused certificate losing
  the verdict, the batch's certificate file names, applying a lemma to a
  two-sided goal), each checked against today's code first;
- the older lists that name steps long done (the exports' lists of steps
  11 and 22, the command's output's of steps 18 and 20, the interactive
  session's of steps 22, 28 and 34: the reading recomputed per
  operation, the filtered `rules` list, a per-goal budget for
  `close_all`, the map from a drawing's inferences): each entry checked
  against the code and closed as done or kept with its reason.

## Session 53b: the release prepared

Opus 5.5 at `high`, after 53a. As 0.2.0's: `breaks 0.2.0` (both tools)
and every break listed in `CHANGELOG.md`; the version raised in both
repositories (the library's 0.3.0 if step 51 changed it, its own
changelog); the command's third-party notices in the packages
(`later.md`, "Follow-ups: the release's licences"), or added; the
manual's `status.md` true; the guide for AI models
within its budget; `nix flake check` in both; the release workflow's
dry run as 0.2.0's was; the GitHub release notes drafted; the Zenodo
check written down for the author: after the tag, whether the new
version record sits under linlog's concept DOI 10.5281/zenodo.23448588
(and the library's under 10.5281/zenodo.23899109) or started a new
concept, and what to do if it did. Then the author tags and publishes.

## How the step runs

Two sessions on Opus 5.5 at `high`, reviewed by the planning session
between them; no benchmark (the engines changed only by step 52's rows,
whose measurement stands). Deliverables: the commits, the draft release
notes, the author's checklist (tag, crates.io through the workflow,
GitHub release, Zenodo, the library's tag), and
`plan/reports/53-release-0.3.0.md`.
