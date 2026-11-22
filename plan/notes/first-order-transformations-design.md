# The transformations at first order: design (step 77)

Written by session 77a (2026-11-22), from `plan/77-first-order-transformations.md`
and the reading it lists: the reports of 63t, 70, 74 and 75, the designs
`first-order-design.md` (sections 3, 5, 10) and `first-order-search-design.md`
(section 5), and the modules `normal.rs`, `negative.rs`, `interpolant.rs`,
`grounding.rs`, `translate.rs` and `instance.rs`.

## 1. What a user gets

- `deduxit seq normalize --to prenex|miniscope|skolem|herbrand|clausal`
  on a classical first-order sequent, and `--to nnf` at first order,
  every step printed as 70's steps are (the law, where it applied, the
  sequent after), then each check with its verdict.
- The protocol's `ordinary.prenex`, `ordinary.miniscope`,
  `ordinary.skolem`, `ordinary.herbrand` and `ordinary.clausal`, beside
  `ordinary.nnf`, which takes first-order sequents from now on.
- The library: `normal::Form` gains the five forms, `normal::Rule` the
  quantifier laws, `Rule::intuitionistic` says whether a law holds in
  LJ; `normalize` checks them as below. The JSON document stays 70's
  (`normal_form`, level 21), written at level 25 when its sequents are
  first-order, as every form holding a first-order sequent is.
- `cnf`, `dnf`, `tseitin` and `plaisted-greenbaum` keep refusing a
  first-order sequent (`Error::FirstOrder`): the clausal form is the
  first-order CNF, and the definitional forms at first order (new
  predicates over the free variables, Nonnengart and Weidenbach) wait
  for a caller.

## 2. One rewriting engine, extended

**Decision: the first-order forms are new forms of `normal.rs`'s engine,
with its steps, phases and checks**, not a module of their own.
- The engine already rewrites a node wherever it occurs, and with de
  Bruijn indices that is right under binders too: an open subformula is
  one node at every depth, and an equivalence of open formulas (De
  Morgan, distribution, `A → B ≡ ¬A ∨ B`) holds under every value of its
  free variables, so replacing it everywhere keeps each formula's
  meaning (the replacement theorem: van Dalen, *Logic and Structure*,
  Thm. 3.4.x; Troelstra and Schwichtenberg, *Basic Proof Theory*,
  §2.1.x, which holds in LJ as in LK). The propositional laws
  therefore apply to first-order formulas unchanged, which is what a
  textbook does.
- **A quantifier law moves a binder, so it shifts indices**: pulling
  `∀x` out of `(∀x. A) ∧ B` puts `B` under one more binder, `B` lifted
  by one (`↑B`); pushing it in (`∀x. (A ∧ B) ≡ (∀x. A) ∧ B`) needs `B`
  free of the binder's variable (no `#0`) and lowers `B` by one. The
  side condition of the textbook laws ("`x` not free in `B`") is in de
  Bruijn terms "`B` holds no `#0`" for the inward direction and holds by
  construction for the outward one (`B` came from outside the binder).
  A new module `bruijn.rs` holds the shift and the substitution of an
  open term, both iterative and memoised per node and depth, as
  `instance.rs`'s substitution is (whose `instantiate` takes closed
  terms only).
- Alternatives weighed: *a quantifier-aware module beside `normal.rs`*
  (two engines, two step types, two JSON forms, for laws that compose:
  the clausal form runs NNF, Skolemization, prenex and CNF in one
  rewriting); *named variables with renaming*: equal formulas would not
  be one node, and the checker compares ids.

## 3. The laws and where they hold

`A`, `B` formulas; in an inward law `B` holds no variable of the binder.
"LJ" marks a law whose two directions LJ proves; "LK" one that only LK
proves (one direction is intuitionistic, the step says which). Sources:
van Dalen §3.5 (the prenex lemmas), Troelstra and van Dalen,
*Constructivism in Mathematics* I, §2.x (which hold intuitionistically).

| law | name | holds |
|---|---|---|
| `¬∀x. A ≡ ∃x. ¬A` | `not-forall` | LK (`←` in LJ) |
| `¬∃x. A ≡ ∀x. ¬A` | `not-exists` | LJ |
| `(∀x. A) ∧ B ≡ ∀x. (A ∧ B)`, either way round | `forall-and` | LJ |
| `(∃x. A) ∧ B ≡ ∃x. (A ∧ B)` | `exists-and` | LJ |
| `(∀x. A) ∨ B ≡ ∀x. (A ∨ B)` | `forall-or` | LK (`→` in LJ) |
| `(∃x. A) ∨ B ≡ ∃x. (A ∨ B)` | `exists-or` | LJ |
| `(∀x. A) → B ≡ ∃x. (A → B)` | `forall-antecedent` | LK (`←` in LJ) |
| `(∃x. A) → B ≡ ∀x. (A → B)` | `exists-antecedent` | LJ |
| `A → ∀x. B ≡ ∀x. (A → B)` | `forall-consequent` | LJ |
| `A → ∃x. B ≡ ∃x. (A → B)` | `exists-consequent` | LK (`←` in LJ) |
| `A ↔ B ≡ (A → B) ∧ (B → A)` | `equivalence-as-implications` | LJ |
| `∀x. (A ∧ B) ≡ (∀x. A) ∧ (∀x. B)` | `forall-distribute` | LJ |
| `∃x. (A ∨ B) ≡ (∃x. A) ∨ (∃x. B)` | `exists-distribute` | LJ |
| `∀x. B ≡ B`, `∃x. B ≡ B` (`B` without `x`) | `vacuous` | LJ |

- The domain is not empty, as in LK and LJ: `∀x. B ⊢ B` takes any term
  for `x`, and the checker takes an eigenvariable no rule introduces as
  a constant (75b's `default`).
- **Prenex form** (`prenex`): first the equivalences split
  (`equivalence-as-implications`, which keeps `→`, so that more steps
  hold in LJ than after NNF), then the eight shift laws and the two
  negation laws, each at the leftmost outermost node where one applies,
  until every binder stands in front. Each step pulls a binder one
  connective out, so it ends. It works on the formula as written
  (van Dalen's lemma), not on its NNF.
- **Miniscoping** (`miniscope`): NNF first (70's laws and the two
  negation laws), then the inward laws (`forall-distribute`,
  `exists-distribute`, `forall-and` and `exists-and` and `forall-or` and
  `exists-or` inward with their side condition, `vacuous`), outermost
  first. Behmann's distribution of `∨` over `∧` under `∀`, which
  grounding adds for the monadic class, is no part of it: it grows the
  formula and is a decision procedure's step, not a normal form's.
- `nnf` at first order adds `not-forall` and `not-exists` to 70's
  negation phase.

## 4. The certificates

### 4.1 Equivalences: a checked derivation per step

**Decision: each step of an equivalent form gets the derivation of its
law instance, decided and checked; the whole result gets 70's claim.**
- The instance is the closed sequent `⊢ ∀ȳ. (C ↔ C')`, `C` the node the
  step rewrote, `C'` what replaced it, `ȳ` their free variables. It is
  decided in intuitionistic logic where the law holds there (an LJ
  derivation, by the embedding or the intuitionistic connection prover),
  else in classical logic (an LK derivation, by the connection prover),
  and the derivation passes `Derivation::check` before the step counts
  as checked. A law marked LJ that LJ does not prove within the bounds is
  an unknown, never a fault; one decided not valid is
  `Error::ClaimFails`.
- The replacement of `C` by `C'` in every formula (the engine's
  substitution, untrusted) is covered by 70's claim on the whole:
  `⊢ (A₁ ↔ A₁') ∧ …` decided classically, its evidence checked.
- **Why decided and not built by template.** A derivation built per law
  (a fixed gadget, as LJT's translation builds) and lifted through the
  context (the replacement theorem's proof) would be certain, but needs
  some forty gadgets in two calculi, each with eigenvariables; the law
  instances are closed sequents of a few nodes, which every method
  decides at once, and a decided derivation passes the same checker. A
  step left unknown at a bound leaves `is_checked` false, as 70's claim
  does.
- Alternative weighed: *70's claim alone*. The prompt asks for a
  derivation per step, and the claim on the whole is a first-order
  search that may not end; the per-step instances always do in practice.

### 4.2 Skolemization and Herbrandization

- **Skolemization** works in refutation form: each formula right of `⊢`
  moves left negated (70's `negate` step), the hypotheses go to NNF, and
  each `∃y. A` in a hypothesis, outermost first, becomes `A[f(x̄)/y]`,
  `x̄` the variables free in `∃y. A` (its `∀` variables, the outer `∃`s
  being replaced already). The promise is 70's equisatisfiable: the
  result is valid exactly when the input is.
- **Herbrandization** works in validity form: the sides stay, and each
  strong quantifier (an `∃` left of `⊢` and a `∀` right of it, by its
  polarity in the formula) becomes its Herbrand term over the weak
  variables free in it. The promise is new, `equivalid`: valid exactly
  when the input is. On a sequent the two meet (Herbrandizing `Γ ⊢ Δ`
  removes what Skolemizing `Γ, ¬Δ` removes); the two forms differ in
  what a reader sees: the refutation form a clausal prover starts from,
  the validity form Herbrand's theorem reads.
- **Grounding's rule is kept** (75d, the argument panel's witness
  SYN316+1): one symbol per quantifier node and value of its free
  variables, never one per variable name. In terms: the Skolem term of
  `∃y. A` is `f(x̄)` over all the variables free in that node (inner
  Skolemization, Nonnengart 1996), so different values of `x̄` give
  different terms; equal nodes are one node of the arena and share their
  symbol, which is Beckert, Hähnle and Schmitt's δ⁺⁺ rule (TABLEAUX
  1993, "The even more liberalized δ-rule"), sound because one witness
  function serves every occurrence of the same formula. Two nodes of one
  variable (miniscoping's split `∀x. A ∧ ∀x. B`) are different nodes and
  get different symbols.
- **The certificate.** Equisatisfiability has two directions:
  - *A model of the result is one of the input*: `Sk(F) ⊢ F` for each
    formula (`∃R` with the Skolem term as the witness), a derivation
    decided and checked as in 4.1 (in LJ: it holds there), the claim
    `restricts` of 70. Herbrandization's dual: `H(G) ⊢ G` for a
    hypothesis and `D ⊢ H(D)` for a formula right of `⊢`, so a valid
    input gives a valid result.
  - *A model of the input expands to one of the result* (Skolem 1920;
    Fitting, *First-Order Logic and Automated Theorem Proving*, §8.x;
    Baaz, Egly and Leitsch, *Normal form transformations*, Handbook of
    Automated Reasoning ch. 5): this needs choice, and `F ⊢ Sk(F)` has
    no derivation in general. It rests on the construction, which a
    structural check confirms (`fresh`, as 70's definitions are
    checked): every Skolem symbol is new (in no formula of the input, a
    name no user wrote), stands at one node class with exactly that
    node's free variables as its arguments in order, and the result is
    the input with each Skolemized node replaced so. Where a finite model
    of the input is at hand (step 76's), its expansion by the symbols can
    be computed and checked by the evaluator: 77b or 76c, not 77a.
  - Alternative weighed: *a derivation from the Skolemized form back*
    covers the first direction only; *the deskolemization of a proof*
    (Skolem terms back to eigenvariables, the connection read-back's
    construction) certifies a *verdict*, not the transformation: it is
    77b's Herbrand certificate (section 5).
- **Names**: Skolem and Herbrand symbols are `sk1`, `sk2`, … in the
  order made, with `_` after `sk` until no symbol of the input is the
  prefix and digits (70's rule for `t1`), arity the number of free
  variables (a constant where none). The symbols table holds predicates
  and functions under one name space, so the check covers both.

### 4.3 Clausal form

`clausal`: `negate`, NNF (with the quantifier negation laws),
Skolemization, the remaining `∀`s pulled to the front (`forall-and`,
`forall-or`: in NNF after Skolemization only `∀` is left, and pulling it
out over `∧` and `∨` holds classically), the matrix to CNF by 70's
distribution under the prefix (an equivalence of open formulas, 2.), then
`forall-distribute` and the conjunction split into hypotheses:
`∀x̄₁. C₁, …, ∀x̄ₙ. Cₙ ⊢`, each a universally closed clause, its
variables apart from the others' by construction (each closure binds its
own). Checks: the equivalence steps as 4.1, the Skolem step as 4.2, the
shape (each hypothesis a `∀`-prefix over a clause), and 70's `restricts`
claim (the clauses conclude the input's negation) decided classically.

## 5. What 77b and 77c build on

- **Herbrand's theorem (77b)**: the clausal form's clauses, ground
  instances over the Herbrand universe found by grounding's SAT path, the
  propositional refutation checked (`drup`). A checked "valid" rests
  either on that refutation with Skolem's theorem (4.2's second
  direction, the structural check) or, better, on a derivation of LK of
  the input built from the instances by deskolemization (the classical
  connection read-back's construction: Skolem terms become
  eigenvariables, the least nested first). The design recommends the
  derivation: the verdict then rests on the checker alone.
- **The negative translations (77b)**: `negative.rs`'s rows for `∀` and
  `∃` are `unreachable!` today behind the door; Kuroda's (`¬¬` after
  each `∀`) and Gödel and Gentzen's (`∃x. A` as `¬∀x. ¬A`, a `¬¬` before
  each atom) fill them, and `compare` decides both sides, an unknown
  side leaving the agreement unknown.
- **Interpolants (77b)**: Maehara's method with the quantifier rules,
  the interpolant quantified over each term of one half only
  (Troelstra and Schwichtenberg §4.4; Fitting §8), its symbols checked to
  occur in both halves, both halves decided.
- **Glivenko's failure (77c), a finding**: no finite Kripke model can
  show it. In a finite frame every world sees a maximal world, whose
  forcing is classical truth in its structure, so a classical tautology
  holds there and its double negation holds everywhere. `¬¬∀x. (p(x) ∨
  ¬p(x))` and the double negation shift `∀x. ¬¬p(x) → ¬¬∀x. p(x)` are
  refuted only by infinite frames with growing domains. 76b's checked
  models are finite, so 77c needs a checked infinite model of a regular
  kind (an ω-chain described by a rule), or the failure shown by another
  certificate. The planning session should decide this before 77c.

## 6. Bounds and D17

- Every sequent on the way is held to `limits.form_occurrences` (70's);
  each Skolem term to `limits.terms` (the reader's measure); the steps
  and new nodes to `limits.memory_bytes`; each pass to `limits.work`;
  the stop asked once per step. Each per-step claim is decided within
  the call's limits and stop.
- Propositional sequents take 70's paths unchanged: the quantifier laws
  never fire without a binder, and the per-step claims run for
  first-order sequents only (70's propositional forms keep their one
  claim). No journey runs `normal.rs`; the ratchet is run as always.

## 7. What 77a builds

The design; `bruijn.rs` (shift, open substitution); the quantifier laws
in `normal.rs` with `nnf`, `prenex` and `miniscope`; Skolemization and
Herbrandization with the structural check and the `restricts` claims;
`clausal`; the per-step claims; the command's `--to` values and the
protocol's operations; tests. 77b: Herbrand's theorem and its
certificate, the negative translations, the interpolants, the manual
and the guide. 77c: Glivenko's failure, after the decision above.
