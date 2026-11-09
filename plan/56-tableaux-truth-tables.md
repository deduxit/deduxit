# Step 56: classical tableaux and truth tables, the beginner's two methods

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and, for the page, in
the web client (`../linlog-prover.github.io`, `deduxit/deduxit.github.io`).
CLAUDE.md applies throughout: jj only (never git), thematic commits as
soon as a unit is done, doc comments on every item, the verification
table, no pushing. This prompt was written on 2026-11-08 by step 46's
second session, from the author's decisions of that day, and is finished
at the review of step 55: it says what is fixed.

Read `plan/notes/research/next/tableaux.md` (candidate 1 and section 5),
`more-logics.md` (candidate 2, truth tables, and candidate 10, which a
later step may add as a mode of this one), `web-front-end.md`
(candidate 6, the views), the reports of steps 49 and 55, and
`plan/README.md`'s D15, D16, D27 and D29.

## Goal

The two methods every first logic course and teaching tool starts with,
for classical propositional logic: the truth table of a formula or
sequent, and the analytic tableau, both in the command and as methods of
the ordinary suite on the page (D29: the tab chooses suite, method and
parameters).

## Fixed now

- **A closed classical tableau is an LK proof upside down**
  (`tableaux.md`): the tableau is a view of the LK read-back the
  ordinary layer already checks, not a second engine; an open, finished
  branch gives a falsifying assignment, the one-world case of step 55's
  countermodel kind, checked as that is.
- **Truth tables are bounded** (D16): a default limit on the atoms (the
  rows double with each), refused with its reason beyond it, a flag to
  raise it within `Limits`.
- **New views on the page** (`web-front-end.md`, candidate 6): a
  tableau tree and a table, each a layout the stage draws, built with
  this suite; the registry of step 49 gains the methods. Learn and
  Practice content for them waits for step 63.
- Intuitionistic tableaux and many-valued tables are not in this step
  (kept in `later.md`).

## How the step runs

Two to three sessions on Opus 5.5 at `high` (the library and the command;
the page in the client's repository, with the `frontend-design`
plugin); a fresh-context reviewer on Opus 5.5 at `high` checks that a
closed tableau corresponds to a checked LK derivation and an open branch
to a checked assignment. Deliverables: the commits, the client's commits,
`plan/reports/56-tableaux-truth-tables.md`.
