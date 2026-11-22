# Step 78: equality and first-order resolution

You are working in deduxit (`deduxit/deduxit`, the checkout
`~/Projects/own/deduxit`). CLAUDE.md applies throughout: jj only (never
git), thematic commits as soon as a unit is done, doc comments on every
item, the verification table, no pushing. This prompt was written on
2026-11-22 by the planning session, on the author's request to run more
in parallel ("Can you also run 78 or anything else in parallel as
well?"), from the step table's row 78; steps 76 and 77 run beside it.

Read `plan/notes/research/next/more-logics.md` (more-logics-1) and the
resolution research items the row names (resolution-2, resolution-4,
interop-5, in `plan/notes/research/`), the reports of steps 70
(`plan/reports/70-resolution.md`: propositional resolution, its checker),
74 and 75 (the first-order arena, the TPTP reader, which refuses `=` and
`!=` today with a hint; grounding; the connection provers; the
comparison and ILTP's first-order package, whose 1 730 problems with
equality are refused), `.claude/rules/core-ordinary.md`, `core.md`,
`cli.md` and `plan/README.md`'s D17 and D25.

## Goal

First-order logic with equality decided as far as the existing methods
reach once `=` has its axioms, and resolution at first order with
unification, step by step, at teaching scale with a checked refutation;
TPTP's `cnf` input and SZS status lines for interoperation.

## Fixed now

- **Equality as axioms first** (more-logics-1): `=` and `!=` read in the
  native syntax and TPTP; the equality axioms (reflexivity, symmetry,
  transitivity, congruence for each function and predicate symbol of the
  sequent) added where a method needs them (the embedding's image, the
  connection provers' matrix, grounding), every proof still read back to
  LK or LJ and checked with the axioms as hypotheses the derivation
  shows; a native rule (paramodulation, or equality in the connection
  calculus) only where the measurement on ILTP's 1 730 equality problems
  asks for it (D19), argued.
- **Resolution at first order**: binary resolution with the shared
  unifier (`crate::unify`), factoring, step by step in the command and
  the protocol as 70's propositional resolution is, on the clausal form
  of step 77 (77a's); a refutation checked by its own checker (each
  resolvent's unifier recomputed), and a refutation read back as "valid"
  of the sequent it refutes the negation of. At teaching scale: no
  claim to compete with a saturation prover.
- **TPTP `cnf` input** (the command's and the harness's readers, the
  clauses read as the universal closure of their disjunction) and **SZS
  status lines** (`% SZS status Theorem for NAME` and the others, under a
  flag the command documents) for the tools that read them.
- **D17**: propositional sequents go as today; the TPTP reader's journeys
  (`read-tptp`, `read-lltp`) measured before and after.
- **Measured**: ILTP's equality problems before (all refused) and after,
  every verdict against both statuses.

## How the step runs

Three sessions, Opus 5.5 at `high`, the panel (argument on Fable 5.2,
counterexamples on Opus, integers on Sonnet) on the equality axioms'
soundness and the resolution checker, `fresh-reviewer` on Fable once per
session:
- **78a** (now, beside 76 and 77): equality read and axiomatised, its
  measurement, `cnf` input and SZS status lines.
- **78b** (after 77a's clausal form lands): first-order resolution with
  unification, factoring, its checker and its steps.
- **78c**: a native equality rule if 78a's measurement asks for one, the
  manual and the guide.

The website is frozen until 0.8.0 (the author, 2026-11-22): the page's
resolution and equality are no part of this step.

Deliverables: the commits, `plan/notes/equality-resolution-design.md`,
`plan/reports/78-equality-resolution.md`.
