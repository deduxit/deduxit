# Step 65: modal logics K to S5, decided with Kripke countermodels

You are working in linlog (`sequentium/sequentium`) and, for the page, in
the web client (`sequentium/sequentium.github.io`). CLAUDE.md applies
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
procedure's design) on Fable 5.1 at `high`, the rest on Opus 5.5 at
`high`; the panel of step 26's prompt for the search and the countermodel
check; the page in the client's repository as a suite of the registry.
Deliverables: the commits, the client's commits,
`plan/reports/65-modal-logics.md`.
