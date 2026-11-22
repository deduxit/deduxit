# Step 77: transformations at first order

You are working in deduxit (`deduxit/deduxit`, the checkout
`~/Projects/own/deduxit`). CLAUDE.md applies throughout: jj only (never
git), thematic commits as soon as a unit is done, doc comments on every
item, the verification table, no pushing. This prompt was written on
2026-11-22 by the planning session, on the author's go to run step 77
beside step 76 ("Go for it"), from the step table's row 77.

Read the reports of steps 63t (`plan/reports/63t-translations.md`: the
negative translations, Glivenko, Craig interpolants of LK and LJ), 70
(`plan/reports/70-resolution.md`: the normal forms, resolution, DPLL),
74 and 75 (`plan/reports/74-first-order-ordinary.md`,
`plan/reports/75-first-order-search.md`: the first-order arena,
`Formulas::instantiate`, the checker's quantifier rules, grounding and
its Skolem rule), the designs `plan/notes/first-order-design.md`
(sections 3, 5 and 10) and `plan/notes/first-order-search-design.md`
(section 5), `.claude/rules/core-ordinary.md`, `core.md` and
`plan/README.md`'s D17 and D25; the modules `core/src/ordinary/normal.rs`,
`negative.rs`, `interpolant.rs`, `grounding.rs` and `translate.rs`.

## Goal

The propositional transformations of steps 63t and 70 at first order,
each checked, in the library, the command and the protocol (the page is
step 79's), every step of each transformation shown as the propositional
ones are.

## Fixed now

- **Prenex form and its inverse (miniscoping)**, each step an
  equivalence a derivation of LK (or LJ, where it holds
  intuitionistically, and said where it does not) checks; the quantifier
  shifts' side conditions (a variable not free) checked.
- **Skolemization and Herbrandization**: equisatisfiability (validity)
  preserved, with the certificate that covers it (the design names
  which: a derivation from the Skolemized form back, or the argument
  checked on the structure grounding or step 76 gives); grounding's
  rule kept: one Skolem symbol per quantifier occurrence and value of its
  free universal variables, equal nodes sharing (δ⁺⁺), since a constant
  per variable was unsound (ILTP's SYN316+1).
- **Clausal form** at first order (the Skolemized matrix's CNF), with
  steps shown, reusing 70's normal forms.
- **Herbrand's theorem** with the ground instances the SAT engine finds
  as a certificate: a checked "valid" from a finite set of instances
  whose propositional refutation is checked.
- **The negative translations at first order**: Kuroda's and Gödel and
  Gentzen's (their `∀`/`∃` rows exist in `negative.rs`), each checked as
  63t checks them; **the formula on which Glivenko's theorem fails** at
  first order (`∀x. ¬¬p(x) → ¬¬∀x. p(x)` or the design's choice), shown
  with its intuitionistic countermodel. That countermodel is
  necessarily infinite (76b, 2026-11-22: in a finite frame every world
  sees a maximal world, which forces every classical tautology, so
  `¬¬A` holds everywhere; the double-negation shift fails only on an
  infinite chain of worlds with growing domains, world n with elements
  0..n and p(k) from world k+1 on). So the item is **a lemma of
  rocq-deduxit that this infinite model refutes the formula**, on 76c's
  first-order Kripke semantics, shown by 77 with the model's
  description; deduxit's bounded searches answer "unknown" on it, which
  77 says. Everything else in this step rests on steps 74 and 75 and
  runs beside step 76.
- **Craig interpolants of LK and LJ at first order** (Maehara's method
  with the eigenvariable condition: an interpolant's free variables and
  symbols in both sides), each checked by derivations of its two halves.
- **D17**: propositional sequents go as today; the ratchet (its
  `ordinary-pigeons` was raised for step 75's dispatch; a change near it
  is measured first).
- **Every step shown**: the command's `seq` or `prove` options that show
  the propositional transformations do so at first order; the protocol's
  operations take first-order values.

## How the step runs

Two to three sessions, Opus 5.5 at `high`, `fresh-reviewer` on Fable
5.2 once per session (the row's reviewer at `medium`):
- **77a**: the design (the transformations' certificates, the Skolem
  symbols' names in the arena, what each shows), then prenex form and
  miniscoping, Skolemization and Herbrandization, clausal form.
- **77b**: Herbrand's theorem with its certificate, the negative
  translations, Craig interpolants at first order, the manual and the
  guide.
- **77c** (after 76c's semantics): Glivenko's failure shown with the infinite model and its Rocq lemma.

Step 76 runs beside it (76a now, then 76b and 76c, in
`core/src/ordinary`'s model search, the countermodel type and the
dispatch): keep to the transformations' modules and change only your
lines in shared files.

Deliverables: the commits, `plan/notes/first-order-transformations-design.md`,
`plan/reports/77-first-order-transformations.md`.
