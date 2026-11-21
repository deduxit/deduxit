# Step 75: proof search at first order

You are working in deduxit (`deduxit/deduxit`, the checkout
`~/Projects/own/deduxit`). CLAUDE.md applies throughout: jj only (never
git), thematic commits as soon as a unit is done, doc comments on every
item, the verification table, no pushing. This prompt was written on
2026-11-21 by the planning session at the review of 74d (74e, the open
witness, still running), from the step table's row 75.

Read the reports of step 74 (`plan/reports/74-first-order-ordinary.md`,
every session) and its design (`plan/notes/first-order-design.md`,
sections 3, 5 and 10), step 42's report (the first-order linear arena
and the focused engine's first-order search, 42d), the reports of steps
54 and 54s (LJT and the SAT engine, the method dispatch), 55 (Kripke
countermodels), `plan/notes/research/next/more-logics.md`
(more-logics-6), `proof-search-specifications.md` ("First-order
fragments"), `.claude/rules/core.md`, `core-ordinary.md`, `core-focus.md`,
`core-search.md` and `plan/README.md`'s D17, D19, D24 and D25.

## Goal

A semi-decision of first-order classical, intuitionistic and minimal
logic: "valid" with a derivation of LK, LJ or LM that the checker
accepts, "not valid" only where a decidable fragment or a finite
argument shows it (the countermodels are step 76's), and "unknown" at a
bound otherwise. The command's `prove --logic …` and the protocol's
`ordinary.decide` answer first-order sequents instead of refusing them.

## Fixed now

- **Through the embeddings first** (design section 10): the embedding's
  pattern table gains the quantifier rows, the focused engine's
  first-order search (step 42) proves the image, and the read-back maps
  the linear quantifier rules to `∀L`, `∀R`, `∃L`, `∃R` with the
  witnesses the linear proof holds. This is a semi-decision of LK and
  LJ from its first session, and the measure the native engines beat.
- **Native engines beside it**: classical logic by a connection
  calculus or free-variable tableau of the leanCoP family; intuitionistic
  (and minimal, where it carries) by a prefixed connection calculus of
  the nanoCoP family. Each proof read back to LK or LJ and checked; a
  native proof that does not read back is a bug, never an answer.
- **Decidable fragments recognised and decided**: the monadic fragment
  and Bernays and Schönfinkel's class (∃*∀*, no function symbols) at
  least, with "not valid" answers there (a finite model, written as the
  countermodel step 76 checks, or a refutation the fragment's argument
  certifies: the design says which).
- **Every answer checked**: "valid" by the derivation's checker,
  "not valid" by the fragment's argument; never an unchecked verdict.
- **D17**: the propositional ordinary suite's target figures, ratchet and
  ILTP rows stay as they are; `read-tptp` has about 1 % of tolerance
  left (fixup-13), so a change there is measured first.
- **Measured** on ILTP's first-order problems (the package
  `iltp-first-order`, read by the harness since 74d: 1 732 rows refused
  `first_order` today) and a TPTP subset, beside iLeanCoP and nanoCoP
  (fetched by the flake at pinned commits, licences checked, run by the
  comparison harness as the propositional provers are).
- **Not overwhelming** (the author): a propositional sequent goes the way
  it goes today; first-order search is reached only by a first-order
  sequent.

## How the step runs

Three to four sessions, Opus 5.5 at `high`; the panel (argument on
Fable 5.2, counterexamples on Opus, integers on Sonnet) on each engine
and read-back; `fresh-reviewer` on Fable once per session:
- **75a, the design and the embedding's search**: the research (leanCoP
  2.x, nanoCoP 2.0, iLeanCoP, their proof formats and how each reads
  back to G3; the decidable fragments' deciders; how "unknown" bounds
  are set and reported), written as `plan/notes/first-order-search-design.md`
  with each choice argued; then the quantifier rows of the embedding,
  the read-back of first-order linear proofs and the dispatch, as far as
  the session reaches. It may start before 74e lands, since the search
  does not touch the session; it rebases onto 74e when that lands.
- **75b, the classical connection prover** and its read-back.
- **75c, the intuitionistic prefixed connection prover** and its
  read-back; minimal logic where it carries.
- **75d, the decidable fragments and the measurements** beside iLeanCoP
  and nanoCoP, the manual, the guide.

Deliverables: the commits, `plan/notes/first-order-search-design.md`,
`plan/reports/75-first-order-search.md`.
