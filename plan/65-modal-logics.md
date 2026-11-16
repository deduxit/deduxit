# Step 65: modal logics K to S5, decided with Kripke countermodels

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and, for the page, in
the web client (`deduxit/deduxit.github.io`). CLAUDE.md applies
throughout: jj only (never git), thematic commits as soon as a unit is
done, doc comments on every item, the verification table, no pushing.
This prompt was written on 2026-11-08 by step 46's second session, from
the author's decisions of that day, and is finished at the review of
step 64: it says what is fixed.

Read `plan/notes/research/next/modal-logics.md` whole, `scope-interop.md`
(interop-11, TPTP's non-classical syntax, for later), the reports of
steps 54, 55 and 60, and `plan/README.md`'s D17, D24, D25, D27 and D29.

## Goal

The collection's third logic family (D27): propositional modal logic in
the normal systems from K to S5 (at least K, T, K4, S4 and S5; D and B
where the session finds them cheap), each decided by a terminating
procedure, every "not valid" answered by a finite Kripke model that is
checked against the system's frame conditions and drawn, in the command
and as a suite of the page.

## Fixed now

- **A modal formula arena of its own**, beside the ordinary one: the
  first session settles it and the system's representation (a value,
  as D2 made fragments values), decisions the later sessions and step 66
  build on.
- **Decision by a calculus with loop checks per system**, from the
  note's sources; complexity as the note states it (K, T, S4
  PSPACE-complete; S5 validity coNP); not a modal SAT competitor.
- **Countermodels reuse step 55's output kind**, with the frame
  conditions (reflexive, transitive, symmetric, euclidean, serial)
  checked on the model; the check is a checker (the panel). A proof is
  given in the calculus the search uses and checked; the labelled
  calculus step by step is step 66's.
- **A model the user gives** (the author, 2026-11-08): step 55's checker
  takes a finite Kripke model with a system named. It says whether the
  model satisfies a modal sequent at a world or at every world, and
  where not, which world fails and why. It first checks the system's
  frame conditions on the model and names the one that fails, if any.
  The same command and bindings as step 55's; the page's model editor
  is step 68's.
- **This is the second non-embedded suite** (D25): its report records
  what it copied from the linear and ordinary suites, which step 67
  reads.
- First-order modal logic, nested sequents, multimodal and epistemic
  logics are kept in `later.md`.

## How the step runs

Three to four sessions: the first (the arena, the systems, the
procedure's design) on Fable 5.2 at `high`, the rest on Opus 5.5 at
`high`; the panel of step 26's prompt for the search and the countermodel
check; the page in the client's repository as a suite of the registry.
Deliverables: the commits, the client's commits,
`plan/reports/65-modal-logics.md`.

## What steps 55 to 63 left you (written at the review of 63c, 2026-11-16, before 64)

The first session starts before the release of 0.5.0, on the author's
yes of 2026-11-16; its code lands after the tag. The rest of this
prompt is finished at the review of 64.
- **Step 55's countermodel** (`ordinary::kripke`, wire level 13, the
  checker, the drawing on the page) is the output kind to reuse; its
  report and `core-ordinary.md` say how it is built and checked.
- **Step 60's generic derivation** (`deduxit::rules`, rule files at
  level 15) is where step 66's labelled calculus lives; a proof here is
  in the calculus the search uses.
- **Step 63a's protocol** (`deduxit::protocol`, `Suite`, the registry,
  `wire.schema.json`): the modal suite implements `Suite` from the start,
  its kinds and operations in the catalogue, its forms in the schema.
  63g's workspace graph takes it from there without work of its own.
- **Wire levels**: 18 is the last on `main`; 63g-b takes 19. Take the
  next free when the code lands, after 63g-b.

## The sessions (planned 2026-11-16)

- **65a, the design** (Fable 5.2 at `high`): the modal arena, the
  systems as values, the procedure per system with its loop checks and
  sources, the proof and the countermodel, the user's model checked,
  the protocol's kinds and operations, in `plan/notes/modal-design.md`,
  then the code as far as the session reaches (the arena and the
  systems), landing after 0.5.0's tag.
- 65b to 65d: the search and its panel, the command, the page (Opus
  5.5), planned at the review of 64.
