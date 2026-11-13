# Step 71: the simply typed lambda calculus: typing derivations and inhabitation

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and, for the page, in
the web client (`deduxit/deduxit.github.io`). CLAUDE.md applies
throughout: jj only (never git), thematic commits as soon as a unit is
done, doc comments on every item, the verification table, no pushing.
This prompt was written on 2026-11-08 by step 46's second session, from
the author's decisions of that day, and is finished at the review of
step 69: it says what is fixed.

Read `plan/notes/research/next/lambda-calculi.md` whole,
`scope-types-higher.md` (types-higher-1 and -8), `scope-applications.md`
(applications-3), the reports of steps 50 and 60, and `plan/README.md`'s
D27 and D30.

## Goal

The Curry–Howard side as a tool: Church-style simply typed lambda-terms
checked, with their typing derivations drawn as trees and built step by
step; the linear lambda calculus of step 50's read-back checked the
same way (closing its round trip); and inhabitation: whether a type has
an inhabitant, its inhabitants enumerated and counted within a bound,
uniqueness decided, through the engines deduxit already has (a type is a
formula, an inhabitant a proof read back).

## Fixed now

- **A term data model beside the formulas**, settled by the first
  session (decisions the later sessions build on), taking step 50's
  terms.
- STLC inhabitation is PSPACE-complete (Statman), so the enumeration is
  bounded and its answers three-valued.
- System F stays a research item (D30): checking only, after the
  decidable suites; inference and inhabitation are undecidable there.
- Type inference, a reduction stepper and a Rocq checker for typing
  derivations (lambda-calculi-4, -5, -7) are kept in `later.md`.

## How the step runs

Three to four sessions: the first (the term model) on Fable 5.2 at
`high`, the rest on Opus 5.5 at `high`; the panel for the type checker.
Deliverables: the commits, the client's commits,
`plan/reports/71-typed-lambda.md`.
