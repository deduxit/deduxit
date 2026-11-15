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

## What steps 59 to 61 left you (the review of 61a, 2026-11-15)

- **Step 60 built the base** (`deduxit::rules`): rule files at wire
  level 15 with their JSON Schema, the generic derivation with scopes
  and discharge, the checker by matching, the printer, the shipped
  calculi LK, LJ, LM, LL and the signed tableau, and `check --calculus`.
  60b's report names what is yours: the Fitch layout's own rules (a
  subproof's lines contiguous, its assumption first, a citation of its
  last line) are not the checker's; a vacuous discharge is accepted; a
  set zone's discharge is compared as a set; the manual chapter's NJ is
  the pattern of a rule file.
- **Step 61a built the interactive session** (`rules::Interactive`,
  `interact --calculus`, wire level 17): `apply` backwards, `assume` a
  hypothesis in force, `undo`, the session embedding its rule file. The
  builder of this step is that session over the natural deduction rule
  files, not a second one; forward steps (Fitch's habit) may need an
  addition, said in the report.
- **61b** (the bindings, the drawing of a generic derivation as a tree,
  the custom calculus on the page) runs now; this step's page sessions
  come after it and take its tree drawing as the Gentzen view.
- **Wire levels**: 16 is 63t-a's, 17 61a's; take the next free.
- **The workspace graph (63g)** will hold a natural deduction proof as a
  node: an immutable value with its JSON form.
- **Not overwhelming** (the author): natural deduction is a method the
  ordinary tab offers, chosen, never forced on a sequent's first view.

## The sessions (planned at this review)

- **62a**, the rule sets and the checker's use: NJ, NK and NM (and the
  common variants a course meets) as rule files with their sources, the
  cross-check against the ordinary suite's verdicts on ILTP's small
  problems, `check --calculus NJ|NK|NM`, the manual. Opus 5.5 at `high`,
  the fresh reviewer on Fable 5.2.
- **62b**, the builder: forward and backward steps over the rule files,
  `interact --calculus NJ`, Fitch's citations.
- **62c**, the drawings and exports: Gentzen trees from 61b's drawing,
  the Fitch list in SVG, LaTeX (the community's package) and Typst.
- **62d** (and **62e** if needed), the page: the method in the ordinary
  tab, the Fitch list view, in the client, with the `frontend-design`
  plugin.
