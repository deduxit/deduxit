# Step 74: first-order ordinary logic, its language and calculi

You are working in deduxit (`deduxit/deduxit`, the checkout
`~/Projects/own/deduxit`), and for the certificates in the Rocq library
(`deduxit/rocq-deduxit`, `../rocq-deduxit`). CLAUDE.md applies
throughout: jj only (never git), thematic commits as soon as a unit is
done, doc comments on every item, the verification table, no pushing.
This prompt was written on 2026-11-20 by the planning session at the
review of step 73, from the author's request of 2026-11-15 ("I
definitely want first-order ordinary logic … including the
transformations") and the step table's row 74.

Read `plan/notes/research/next/more-logics.md` (more-logics-6),
`proof-search-specifications.md` ("First-order fragments"), the reports
of steps 42 (the first-order linear arena, terms and unifier), 54 and
54s (LJT, the SAT engine), 55 (Kripke countermodels), 60 to 62 (rule
files, sessions, natural deduction), 65 (the modal arena, a second copy
of the ordinary one) and 67a (`plan/notes/kernel-boundary.md`, section
6: this step decides the shared arena core, the parser over tables and
the goal tree's free functions), the 73 report's last section (the state
of `later.md`'s items), `.claude/rules/core-ordinary.md`, `core.md` and
`plan/README.md`'s D17, D24, D25 and D27.

## Goal

The ordinary suite at first order: terms, predicates and quantifiers;
sequents read from text, JSON and TPTP's `fof` (ILTP's first-order
problems among them); LK, LJ and LM with the quantifier rules, a
derivation's checker, step-by-step proving in the command and the
bindings, exports and Rocq certificates of derivations. Proof search
(75), countermodels (76), transformations (77), equality and resolution
(78) and the page (79) come after it and build on its data model.

## Fixed now

- **On step 42's terms and unifier**: the first-order linear arena's
  terms, variables, eigenvariables and witnesses are the model to take
  or share, not to copy a third time; 67a's boundary says which parts
  become shared code and which stay in a suite. The propositional speed
  may not move (D17): the propositional ordinary suite's target set,
  ratchet and ILTP rows stay as they are.
- **The calculi** are G3-style LK, LJ and LM with `∀R`, `∀L`, `∃R`,
  `∃L` (eigenvariable conditions checked), as the propositional ones
  are, so step 60's rule files and step 61's sessions can carry them
  too where that serves (decided and said in the design).
- **Witnesses and eigenvariables chosen as the linear session chooses
  them** (step 42's interactive rules): a term typed by the user, or a
  fresh variable the session names.
- **Every derivation checked**; a certificate in Rocq for the
  derivations of LK and LJ through rocq-deduxit (a new module of the
  library, as `Deduxit.Modal` was: the author placed modal soundness
  there on 2026-11-16, and the same place serves here unless the
  design says otherwise).
- **Undecidable**: nothing in this step decides first-order validity;
  the command answers a derivation checked or refused, and a search
  only from step 75.
- **Not overwhelming** (the author): first-order input changes nothing
  for a propositional sequent; the page is step 79's.

## How the step runs

Three to four sessions:
- **74a, the design** (the language and the calculi's data model):
  Fable 5.2 at `high` by the plan. **While the planning session is
  unattended a switch to Fable blocks it on the author's prompt**: then
  74a runs on Opus 5.5 at `high` and its report says "(departs from the
  plan: Opus in place of Fable, unattended)".
- **74b to 74d** on Opus 5.5 at `high`: the parser and the TPTP reader,
  the checker and the sessions, the exports and certificates; the
  panel's argument member on Fable 5.2 for the checker's eigenvariable
  conditions.

Deliverables: the commits, `plan/notes/first-order-design.md`,
`plan/reports/74-first-order-ordinary.md`.
