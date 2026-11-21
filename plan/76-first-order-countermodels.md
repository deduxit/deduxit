# Step 76: countermodels at first order

You are working in deduxit (`deduxit/deduxit`, the checkout
`~/Projects/own/deduxit`), and for the certificates in the Rocq library
(`deduxit/rocq-deduxit`, `../rocq-deduxit`). CLAUDE.md applies
throughout: jj only (never git), thematic commits as soon as a unit is
done, doc comments on every item, the verification table, no pushing.
This prompt was written on 2026-11-21 by the planning session while step
75's last sessions ran (75c, 75d), from the step table's row 76; its
section "What step 75 left you" is finished at 75's review.

Read the reports of steps 55 (Kripke countermodels: `ordinary::model`,
`Countermodel::found`, the Rocq certificate of a model), 74 and 75
(`plan/reports/74-first-order-ordinary.md`,
`plan/reports/75-first-order-search.md`, every session; 75d's finite
structure for the decidable fragments is the type this step extends),
the designs `plan/notes/first-order-design.md` (section 10) and
`plan/notes/first-order-search-design.md` (sections 5 to 7), 54s (the
SAT engine), `.claude/rules/core-ordinary.md`, `core-export.md`, `core.md`
and `plan/README.md`'s D17, D19 and D31; in rocq-deduxit its CLAUDE.md,
rules and the modules a model's certificate uses (`Deduxit.Intuitionistic`).

## Goal

"Not valid" at first order wherever a finite model shows it: for
classical logic a finite structure found through the SAT engine (MACE's
method: flatten, ground over a domain of size n, n growing), for
intuitionistic and minimal logic a finite Kripke model with growing
domains; every model checked against the sequent by an evaluator before
it is an answer, drawn as the propositional models are, and certified in
Rocq by lemmas of the library. "Unknown" where none is found within the
bound.

## Fixed now

- **One countermodel type at first order**, extending 75d's finite
  structure (a domain per world, constants' and functions' tables,
  predicates' true tuples per world, the accessibility order and the
  domains growing along it), checked by `Countermodel::found`'s path: no
  unchecked "not valid".
- **Classical models by MACE's method** through the SAT engine (`sat`
  feature): sizes 1, 2, … up to a bound, symmetry breaking where the
  literature argues it; the decidable fragments of 75d answer first, and
  the model search is the fallback for a classically invalid sequent
  outside them.
- **Kripke models with growing domains** for intuitionistic and minimal
  logic: a bounded search (frames and domains up to a bound) whose model
  is checked; an exhausted bounded search is no answer ("unknown"), as
  75a's design says.
- **Drawn and exported** as the propositional models are (SVG, LaTeX,
  Typst, the JSON at a wire level of its own), in the command (`prove`
  prints the model), the protocol and the bindings (the page is 79's).
- **Certified in Rocq**: a model's certificate states the sequent's
  refutation in the model and proves it, resting on lemmas of
  rocq-deduxit (a first-order Kripke semantics with growing domains,
  monotonicity; the classical case as one world). A change of the
  library is made, checked and pushed there first, then the lock moves
  here. Opus writes the lemmas; Fable 5.2 at `high` only if Opus visibly
  falls short (the row; while the planning session is unattended a
  switch to Fable blocks it, so then the lemma waits rather than
  switching).
- **D17**: propositional sequents go as today; the ratchet and the
  propositional ILTP rows unchanged.
- **Measured** on ILTP's first-order problems: how many of the
  non-theorems (the files' `CounterSatisfiable` and `Non-Theorem`
  statuses) get a checked model, beside nanoCoP-i's and iLeanCoP's
  answers where they give any.

## How the step runs

Two to three sessions, Opus 5.5 at `high`, the panel (argument on Fable
5.2, counterexamples on Opus, integers on Sonnet) on each search, check
and certificate, `fresh-reviewer` on Fable once per session:
- **76a**: the design (the model type, MACE's flattening and symmetry
  breaking, the Kripke search's bounds, the Rocq semantics and lemmas
  the certificates need), then the classical model search and the
  evaluator.
- **76b**: the Kripke models with growing domains, their search, check
  and drawing; minimal logic.
- **76c**: the Rocq lemmas in rocq-deduxit and the certificates, the
  measurements, the manual and the guide.

Deliverables: the commits (both repositories),
`plan/notes/first-order-countermodels-design.md`,
`plan/reports/76-first-order-countermodels.md`.

## What step 75 left you

(Finished at 75's review.)
