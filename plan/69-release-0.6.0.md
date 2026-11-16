# Step 69: the release of 0.6.0

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and the Rocq
libraries step 66 changed. CLAUDE.md applies throughout; tagging and
publishing are the author's acts. This prompt was written on 2026-11-08
by step 46's second session and is finished at the review of step 68: it
says what is fixed.

Read `plan/64-release-0.5.0.md` and its report, `.claude/rules/ci.md`,
`CHANGELOG.md` and the reports of steps 65 to 68.

## Fixed now

0.6.0 is "modal logics" (D27): steps 65 to 68. As the releases before
it: `breaks 0.5.0` (and the kernel crate's re-exports, if step 67
extracted one, with no break), the versions, `status.md` true, the guide
within its budget, `nix flake check`, the release workflow's dry run
(and, if the kernel crate is published now, its Trusted Publishing
configuration on the author's checklist), the release notes, the
author's checklist; a baseline night the author's to give. One session
on Opus 5.5 at `high`. Deliverable: `plan/reports/69-release-0.6.0.md`.

## What steps 65 to 68 left you (the review of 68c, 2026-11-16)

- **What 0.6.0 holds** (each with a report under `plan/reports/`):
  modal logic in the fifteen systems of the cube (`deduxit::modal`,
  `--system`, wire level 20, the page's modal tab and model editor);
  the labelled calculus step by step (`[]`/`<>`, forward steps in a
  sequent calculus, `rules::Interactive::extend`); the Rocq library's
  `Deduxit.Modal` and modal certificates (`--format rocq` of modal
  outcomes and derivations); the tableau checker (`check --tableau`,
  `ordinary_tableau_review`, four new rules in `tableau.json`); 54x6's
  trimmed module and wire docs; 54x7 if it lands before you (Ser with
  one label, auto-reverse in ordinary sessions, the modal fuzz run). The
  web client's course packs, check view, embed, print, model editor and
  tableau editor ship with the client.
- **No kernel crate** (67 ended at its gate): nothing to re-export.
- **The Rocq library is released first**: `rocq-deduxit` gained
  `Deduxit.Modal` under `[Unreleased]` at 0.4.0, so it gets its next
  version (with deduxit's number when it changes, as the Rocq chapter
  says: 0.6.0), its opam version and `export::rocq::DEDUXIT` together,
  in its own workspace (`jj -R /home/tux/Projects/own/rocq-deduxit
  workspace add --name step69 /home/tux/Projects/own/rocq-deduxit-69 -r
  main@origin`), its `nix flake check`; the planning session tags and
  pushes it before deduxit, then moves deduxit's lock.
- **The behaviour lock's workspace input** names the version that wrote
  it (63g-b): bless it again in the version bump's commit, as 64 did.
- **A new variant of a fieldless enum goes last** (64's rule;
  `rocq::Unsupported::Assumption` came at 66c).
- **The tags**: the author's yes is asked at this review for 0.6.0 (the
  yes of 2026-11-16 was for 0.5.0).

## The session (planned at this review)

- **69**, after 54x7 has landed: one session on Opus 5.5 at `high`,
  everything above but the tags and the publish.
