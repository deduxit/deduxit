# The order: what to generalise first, what to add in which order

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by the
synthesis session from the twenty-four research notes beside it; every
claim about another tool or project is theirs and carries its link
there. "(inference)" marks this note's own conclusions.

## 1. What the notes say about order

- `structure.md` (structure-5 and its recommendation): `core/src/ordinary/`
  built its own arena, derivation and checker beside the linear ones, not
  on them, and works; so a kernel should be extracted from what a second
  non-embedded suite and `ordinary/` have copied, not written before.
  The engines call the generic parts only for errors, limits and the stop
  poll, so moving the generic list into a kernel crate does not cross the
  inner loops (D17's speed).
- `web-front-end.md` (recommendation): make the greyed "Ordinary" segment
  real first and write the suite registry as part of it; write the
  protocol from the two suites that then exist; choose one module or one
  per suite when a suite from another repository arrives.
- `custom-rules.md`, `natural-deduction.md` section 5, `tableaux.md`
  section 5, `modal-logics.md` section 5: natural deduction (discharge at
  a distance, a scope tree for Fitch), tableaux (branches closed by
  complementary literals) and labelled modal sequents (labels and
  relational atoms) each need a derivation type whose nodes are rule
  instances given as data; the custom-rules layer is that type.
- `ordinary-sequents.md`: a native LJT engine and Kripke countermodels
  are two sessions each and close the "unknown" answers of the
  intuitionistic embeddings (108 of ILTP's 274 problems decided at 2 s by
  step 25's measurement).
- `lambda-calculi.md` and `scope-types-higher.md`: reading ILL and LJ
  proofs back as lambda-terms is one to two sessions on what exists and
  serves all three audiences.
- `cyclic-proofs.md`: provability is undecidable in every variant found;
  only checking is decidable; no teaching use found. `lambda-calculi.md`:
  System F inference and inhabitation are undecidable; only checking.
- `name-and-organisation.md`: the cheapest moment for a rename is before
  the opam pull request is submitted.
- `users.md`: no public evidence of use beyond the author's, so no
  candidate can claim a user's demand; the order must rest on the kind
  of tool and on cost.

## 2. The principle (inference)

Generalise only what two suites have copied; release after every suite;
put the cheap, high-value items of the linear suite into the same
releases, so that linear-logic users gain at every step; keep the
undecidable areas (cyclic proofs, System F) as checkers and research
items after the decidable ones.

## 3. The order

The releases and their contents are the table "The order: each step
leaves a released, useful tool" in `plan/reports/46-research-next.md`
(0.3.0 ordinary logic on the page; 0.4.0 intuitionistic logic decided
with countermodels; 0.5.0 custom rules and natural deduction; 0.6.0
modal logics; 0.7.0 resolution and the typed lambda calculus; then the
author's choice). The ranked list there places every candidate of the
notes.

## 4. What would change the order (inference)

- If the author does not want the collection: 0.3.0 without the
  registry (the Ordinary tab still, as D57 planned), then the linear
  items (lambda-terms, certified refutations, the dispatch rows,
  exponentials by signature, explaining unprovability) in the order of
  `scope-logics.md`, `scope-proof-theory.md` and `scope-engines.md`.
- If the author wants modal logic early: 0.6.0's engine (modal-logics-1)
  does not need the generic derivation type and can move before 0.5.0;
  its step-by-step calculus (modal-logics-2) does need it.
- If a client without a shell asks for linlog: the MCP server
  (llm-agents-1, 1.5 sessions) joins the next release.
- If a user asks for the first-order classical mainstream: more-logics-1
  with a TPTP `fof` reader for first-order formulas moves before 0.7.0.
