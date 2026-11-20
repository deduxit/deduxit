# Step 70: resolution at teaching scale

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and, for the page, in
the web client (`deduxit/deduxit.github.io`). CLAUDE.md applies
throughout: jj only (never git), thematic commits as soon as a unit is
done, doc comments on every item, the verification table, no pushing.
This prompt was written on 2026-11-08 by step 46's second session, from
the author's decisions of that day, and is finished at the review of
step 69: it says what is fixed.

Read `plan/notes/research/next/resolution.md` whole, `more-logics.md`
(candidate 7), the reports of steps 56 and 63, `deny.toml`, and
`plan/README.md`'s D10, D16, D27 and D29.

## Goal

The clausal method of a first course, for propositional logic: a
formula's CNF shown step by step, a resolution refutation built by hand
or found within a bound and drawn as a graph, DPLL traced, and a
refutation read and checked; in the command and as a method on the page.

## Fixed now

- **Teaching scale for the method shown** (the research's limit):
  bounds on clauses and steps by default (D16), "unknown" beyond them.
  Since 2026-11-09 classical logic is decided by a CDCL engine with
  checked LRAT proofs (step 54s, the author's word); this step shows
  resolution and DPLL by hand and traced, and reuses 54s's LRAT checker
  for the proofs of outside solvers.
- **A checked refutation**: a resolution proof is checked step by step;
  a RUP or DRAT/LRAT proof from an outside solver is read and checked
  only if a crate with a licence `deny.toml` allows does it, or by a
  small checker of deduxit's own (resolution-3 reduced, more-logics-7);
  no solver dependency with another licence (`splr` is MPL-2.0).
- DIMACS in, the ordinary syntax in; the TPTP `cnf` reader with SZS
  output and first-order resolution are kept in `later.md`.
- The graph view (web-front-end-6) built here for the page.

## How the step runs

Three to four sessions on Opus 5.5 at `high`; the panel for the
refutation checker. Deliverables: the commits, the client's commits,
`plan/reports/70-resolution.md`.

## What steps 63 to 69 left you (the review of 69, 2026-11-20)

- **The normal forms are this step's too** (the author, 2026-11-15, in
  the step table's row): NNF, CNF and DNF by distribution, each
  equivalent to its input under a size bound that refuses, and CNF by
  Tseitin's and Plaisted and Greenbaum's encodings, equisatisfiable;
  every rewrite shown, the result checked (equivalence by the SAT
  engine `ordinary::sat`, equisatisfiability through the definitions),
  in the library, the command and on the page.
- **The protocol and the workspace** (63a, 63g): each transformation is
  an operation of the ordinary suite with its promise (`equivalent`,
  `equisatisfiable`) and the check that holds it, so the workspace
  graph takes it as an edge without work of its own; 63t's translations
  are the pattern (`ordinary::negative`, `interpolant`).
- **The SAT engine and its LRAT checker** (54s) are the classical
  default; reuse the checker for the proofs of outside solvers.
- **No kernel crate** (67): the ordinary arena is where the clauses'
  formulas live; step 74 later gives it quantifiers.
- **The web module's ceiling** is 3 750 000 bytes at gzip; it was
  3 637 866 after 65d; say what your calls add.
- **Wire levels**: 20 is the last (modal, 0.6.0); take the next free.

## The sessions (planned at this review)

- **70a, the normal forms** (library and command): NNF, CNF, DNF,
  Tseitin and Plaisted–Greenbaum, every rewrite shown, the checks, the
  protocol's operations; the panel's argument member (Fable) on the
  promises and their checks.
- **70b, resolution and DPLL** (library and command): a refutation
  built by hand or found within a bound, DPLL traced, the refutation
  checker, DIMACS in, an outside RUP/DRAT/LRAT proof through 54s's
  checker; the panel on the checker.
- **70c, the page**: the normal forms and resolution as methods of the
  ordinary tab, the graph view (web-front-end-6).
