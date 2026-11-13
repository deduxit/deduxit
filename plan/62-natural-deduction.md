# Step 62: natural deduction, in Gentzen's trees and in Fitch's style

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and, for the page, in
the web client (`deduxit/deduxit.github.io`). CLAUDE.md applies
throughout: jj only (never git), thematic commits as soon as a unit is
done, doc comments on every item, the verification table, no pushing.
This prompt was written on 2026-11-08 by step 46's second session, from
the author's decisions of that day, and is finished at the review of
step 61: it says what is fixed.

Read `plan/notes/research/next/natural-deduction.md` whole,
`web-front-end.md` (candidate 6, the list view), the reports of steps
49, 60 and 61, and `plan/README.md`'s D15, D27, D28 and D29.

## Goal

The method most logic courses teach first: natural deduction for
classical, intuitionistic and minimal propositional logic, checked and
built step by step, drawn as Gentzen trees and as Fitch proofs, in the
command and on the page as a method of the ordinary suite.

## Fixed now

- **On step 60's generic derivation**, with its discharge links and its
  scope tree; the rule sets as rule files, so a course's own variant of
  the rules is a file (natural-deduction-4), with the textbooks' common
  sets shipped and their sources named.
- **Checked by step 60's checker**; a natural deduction proof's verdict
  says which rule set it is valid in. Translation from LK and LJ proofs
  and normalisation (natural-deduction-2, -3) are kept in `later.md`.
- **The Fitch view** is new drawing work (`web-front-end.md`, candidate
  6): a list layout on the stage and in the exports (LaTeX and Typst
  through the packages their communities use, read at the version built
  against; SVG of deduxit's own), configured by one options value (D15).
- Natural deduction for classical linear logic is not built (no
  standard calculus found, 46a's dropped row).

## How the step runs

Four to six sessions on Opus 5.5 at `high` (the rule sets and the
checker's use; the interactive builder; the drawings and exports; the
page's list view and the tab, with the `frontend-design` plugin), each
reviewed by the planning session; the `fresh-reviewer` agent (Fable 5.2
at `medium`) on the discharge and scope handling. Deliverables: the
commits, the client's commits, `plan/reports/62-natural-deduction.md`.
