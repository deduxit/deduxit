# Step 77: transformations at first order

## Session 77a: the design and the normal forms

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`
(the conversation of 75a, 75c and fixups 15 to 22), cores 10 to 12, from
`plan/77-first-order-transformations.md` and the brief
`/tmp/linlog-handoff/start-77a.md`, beside 76a. Nothing was pushed.

### The design

`plan/notes/first-order-transformations-design.md` (`rrwtyzrv`). Its
decisions, each argued there with its sources and alternatives:
- **One engine**: the first-order forms are new forms of `normal.rs`,
  with its steps, phases and checks. With de Bruijn indices an open
  subformula is one node at every depth, and an equivalence of open
  formulas holds under every value of its variables, so the
  propositional laws apply under binders unchanged (the replacement
  theorem, which holds in LJ as in LK); only a quantifier law moves a
  binder, and with it the indices of what crosses it (a new module,
  `bruijn.rs`).
- **The laws** (van Dalen's prenex lemmas), each with whether it holds
  in LJ both ways: prenex form keeps `→` (more steps hold in LJ than
  after NNF), miniscoping runs on NNF; Behmann's distribution is left
  out (a decision procedure's step, not a normal form's).
- **Equivalences are checked by a derivation per step** (the law's
  instance, decided and checked, in LJ where the law holds there),
  beside 70's claim on the whole; templates were weighed (forty gadgets
  in two calculi) and left.
- **Skolem and Herbrand**: grounding's rule kept (one symbol per
  quantifier node and value of its free variables, equal nodes sharing
  by δ⁺⁺, never one per variable name); one direction of the promise is
  a derivation (`restricts`, `implied`), the other (a model of the input
  expands to one of the result) needs choice and rests on a structural
  check of the construction. A verdict through the clauses is 77b's
  Herbrand certificate, which should be a deskolemized derivation of LK.
- **Names**: `sk1`, `sk2`, … with `_` after `sk` until no input name
  collides.
- **A finding for 77c**: no finite Kripke model can show Glivenko's
  failure. In a finite frame every world sees a maximal one, which is
  classical, so a classical tautology's double negation holds
  everywhere; `¬¬∀x. (p(x) ∨ ¬p(x))` and the double negation shift fall
  only to infinite frames with growing domains. The planning session
  should decide what 77c checks before it starts.

### The code

- `yptmxuux`: **the library**.
  - `bruijn.rs`: `lifted`, `lowered` (refusing a formula that mentions
    the binder taken away), `substituted` (an open term put in for a
    binder's variable, lifted under the binders inside), `free_variables`,
    `add_like` and `add_named` (a rebuilt binder keeps its name), each
    walk iterative and memoised per node and depth.
  - `normal.rs`: `Form::{Prenex, Miniscope, Skolem, Herbrand, Clausal}`
    and `Form::takes_first_order`; `Promise::Equivalid`; seventeen rules
    (the ten shift and negation laws, the equivalence as two
    implications, the two distributions, vacuous quantifiers,
    `skolemize`, `herbrandize`, `split-conjunction`) with
    `Rule::intuitionistic` and `Rule::is_equivalence`; `Step::term`;
    `Claim::{Law, Implied}` and `Checked::step`. The phases are one
    `Run` (`stages`, `record`, `strong`, `hypotheses`); `quantifier_law`
    and `apply_quantifier` are the table and what it makes;
    `skolemized` the flow of the three forms; `in_form` holds each new
    form; `transfer` rebuilds a first-order input by symbol names;
    `skolem_fresh`, `law_claim` and the claims as designed.
  - The serializer writes `term` and `step` and level 25 at first order;
    the schema has the new names and keys; the protocol has
    `ordinary.prenex`, `miniscope`, `skolem`, `herbrand`, `clausal`
    (`EQUIVALENT`, `EQUISATISFIABLE`, a new `EQUIVALID`), and its check
    decides first-order claims by the classical default.
  - Tests: each form on chosen sequents (steps and result pinned, every
    claim valid), the symbols' prefix, the propositional forms' door,
    24 generated first-order sequents under all six forms (checked or
    unknown at a bound, never a failed claim, at least half checked),
    the protocol's five operations run and checked, a wrong output
    failed.
- `wousrxpl`: **the command**: `seq normalize --to prenex`, `miniscope`,
  `skolem`, `herbrand`, `clausal`; a step under a quantifier prints the
  sequent after it but not the subformula, which holds a variable bound
  outside it; each law's check is named by its step. The manual's
  normal-form section has an "At first order" part with a prenex and a
  clausal example (run by the README test), the guide its sentences, the
  schema check three first-order documents.
- `pvqsnwxz`: the lock's `workspace-operations`.
- The web module grew from 2 631 105 to 2 662 311 bytes at gzip with
  the first-order forms, past the ceiling of 2 650 000 (the `budget`
  check of the full check found it). A raise was committed, then dropped
  at the rebase onto 76a, which raised the ceiling to 2 700 000: with
  both the module is 2 686 828.

**Departures from the design**:
- The law claim is `⊢ C ↔ C'` with each free variable a new constant
  (`c1`, …), not the universal closure: valid exactly when the closure
  is, and the intuitionistic search stopped at copy bound 3 on the
  closure of a distribution under `∀`. The claim is decided with the
  copy bound lifted, since it is valid in its logic by the law's table
  (departs from the design).
- A law that holds in LJ is decided there within `LAW_WORK` (2²⁰ units)
  and in LK past it: the design said the instances always end in
  practice, and the review found one in twelve generated clausal forms
  where the intuitionistic search did not, starving the claims after it
  (departs from the design).

### Fresh review

`fresh-reviewer` (Fable 5.2, `medium`, the row's reviewer). No
soundness break: every quantifier law is a classical equivalence with
its side condition enforced before each lowering, `apply_quantifier`
takes the operand `quantifier_law` found in every case, the shifts
capture nothing, every LJ mark of `Rule::intuitionistic` is right,
Skolem's and Herbrand's steps are as designed, D17 holds, and a
differential of about 450 generated first-order sequents through the six
forms gave no failed claim, no panic and no verdict that differed
between input and result. Its findings, all fixed in `yptmxuux` and
`wousrxpl`:
- **The check did not hold the result to the steps** for Skolem,
  clausal and Herbrand: a result set by hand (`⊥ ⊢` for the Skolem form
  of `p(a) ⊢`) was checked, and a step forged between two others could
  write a Skolem symbol before its own step. `replay` now makes the
  steps again from the input in the result's arena (each equivalence its
  law where it applies, each Skolem symbol absent from the sequent
  before it, the last sequent the result);
  `a_result_set_by_hand_is_held_to_the_steps`.
- **The protocol chapter** lacked the five operations and `equivalid`,
  and the transformations chapter named the old five: both added.
- **The manual said every claim is decided by the SAT solver**, false at
  first order: it says the logic's default there.
- An observation, fixed: an LJ law claim that did not end starved the
  claims after it (`--timeout 15` left "the law of step 42: unknown" on
  a vacuous `∀`); the bounded attempt with LK after it decides that
  sequent in 1.2 s.

### Checks

- `cargo clippy --workspace --all-targets -- --deny warnings` and
  `cargo test --workspace` (39 binaries) after each commit and after the
  review's fixes; the README test runs both new examples.
- `nix build .#checks.x86_64-linux.wire-schema`: the three new
  first-order documents valid against the schema.
- The full check's first runs failed in `conventions` (a comment naming
  a step, and the check lines "the law of step N" in the manual, which
  read as a plan's step: they say "rewrite" now), `treefmt` (`mod
  bruijn;` out of rustfmt's order) and `budget` (above), each fixed in
  its commit or, the budget, by 76a's raise.
- The gate passed on the chain before the review's fixes; the ratchet
  unmoved (`ordinary-pigeons` +1.03 % as since fixup-19; no journey runs
  `normal.rs`).
- `nix flake check` under the machine lock at the report's commit
  (12:14 to 12:39), on `main@origin` (`klzknswt`, 76a's code with it):
  every check passed, the budget's module at 2 686 828 bytes. Only this
  line changed after.

### What the next sessions must know

- **77b** builds on `normal.rs`'s clausal form for Herbrand's theorem:
  the clauses are `∀x̄. C` hypotheses, their instances the SAT solver's
  ground problem. The design recommends a deskolemized derivation of LK
  as the certificate of a valid verdict (the connection read-back's
  construction maps Skolem terms to eigenvariables, least nested first),
  not the refutation with Skolem's theorem alone.
- `negative.rs`'s `∀` and `∃` rows are `unreachable!` behind its door
  today: 77b fills them (Kuroda's `¬¬` after each `∀`; Gödel and
  Gentzen's `∃x. A` as `¬∀x. ¬A`).
- Interpolants at first order (77b) need the eigenvariable condition in
  Maehara's method and a symbol check of the interpolant.
- `cnf`, `dnf`, `tseitin` and `plaisted-greenbaum` still refuse a
  first-order sequent: the clausal form is the first-order CNF, and the
  definitional forms over free variables wait for a caller.
- The Extends direction of Skolem's promise has no derivation; where a
  finite model of the input is at hand (76's), its expansion by the
  Skolem symbols can be computed and checked by the evaluator.
- 77c: the finding above about finite Kripke models.
