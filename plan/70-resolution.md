# Step 70: resolution at teaching scale

You are working in linlog (`deduxit/deduxit`) and, for the page, in
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

- **Teaching scale, never a SAT solver** (the research's limit, kept
  from the ordinary layer's): bounds on clauses and steps by default
  (D16), "unknown" beyond them.
- **A checked refutation**: a resolution proof is checked step by step;
  a RUP or DRAT/LRAT proof from an outside solver is read and checked
  only if a crate with a licence `deny.toml` allows does it, or by a
  small checker of linlog's own (resolution-3 reduced, more-logics-7);
  no solver dependency with another licence (`splr` is MPL-2.0).
- DIMACS in, the ordinary syntax in; the TPTP `cnf` reader with SZS
  output and first-order resolution are kept in `later.md`.
- The graph view (web-front-end-6) built here for the page.

## How the step runs

Three to four sessions on Opus 5.5 at `high`; the panel for the
refutation checker. Deliverables: the commits, the client's commits,
`plan/reports/70-resolution.md`.
