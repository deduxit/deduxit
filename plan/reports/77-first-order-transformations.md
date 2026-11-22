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

## Session 77b: Herbrand, the translations and interpolants

Opus 5.5 at `high`, in the same workspace and conversation as 77a, cores
10 to 12, from `plan/77-first-order-transformations.md` and the brief
`/tmp/linlog-handoff/start-77b.md`, beside 76b and 78a. Nothing was
pushed.

### What landed

- `kpukwryx`: **the negative translations at first order**.
  `negative::row` has the quantifier rows: Kuroda's `¬¬` after each `∀`
  (`Shape::ForallDoubled`), Gödel and Gentzen's `∃x. A` as `¬∀x. ¬A`
  (`Shape::ExistsAsForall`), Kolmogorov's `¬¬` before every node, binders
  included. A first-order sequent is translated in a copy of its arena,
  so atoms and binder names stay. Glivenko's refuses a first-order
  sequent (`Error::FirstOrder`): its theorem fails there. `compare`
  decides both sides as before; a test pins the written forms and the
  agreement of all three on chosen sequents. The intuitionistic side
  needs more than the library's default three copies there (the test
  lifts them; the command has no copy bound).
- `tzrrwkmz`, `ytoztwuy`, `vynyznnu`: **Herbrand's theorem**
  (`ordinary::herbrand`, feature `sat`; `deduxit herbrand SEQUENT
  [--depth N]`; the protocol's `ordinary.instances`; wire level 27, the
  schema's `herbrand`).
  - The clausal form by `normal::rewrite` (steps shown, unchecked), then
    ground instances over the terms level by level up to `--depth` (3 by
    default), each level decided by the SAT solver, its clausal proof
    checked; the first refuted set is cut one instance at a time to
    those the solver needs (`Statistics::kept`).
  - The certificate of a valid verdict is a **derivation of LK of the
    sequent as written**, checked (`Build`, modelled on the connection
    read-back's construction): `∀L`/`∃R` take the instances' terms as
    witnesses (and a constant that is no Skolem term, for a first
    instance before any eigenvariable exists), `∀R`/`∃L` the Skolem term
    the clausal form replaced the quantifier by, found by matching the
    quantifier's negation normal form (moved left of `⊢`) against each
    Skolem step (`matched`, `normal::negation_normal`); the search runs
    with Skolem terms for eigenvariables, and `deskolemized` turns each
    into a fresh eigenvariable, least nested first.
  - The command prints the clausal form's steps, the instances kept with
    the terms put for each clause's variables, and the verdict; exit 0
    valid, 3 unknown, 2 error.
  - Tests: chosen valid sequents (the drinker, nested Skolem terms, an
    equivalence, a two-step chain), the cut, the unknowns, two
    quantifiers of one variable, the protocol's run and check (a forged
    input fails, an unknown is `None`), and 300 generated first-order
    sequents against the default prover (a one-off run of 2 000 found all
    402 classically valid ones, each derivation checked, no defect).
- `pzuyoqvp`: **Craig interpolants at first order** in LK, LJ and LM.
  After each inference, every term of the interpolant whose head symbol
  or eigenvariable is not a name of both halves of the conclusion is
  bound by a quantifier of its own: `∃` for a name absent from the
  second half (or in neither half with the principal formula in the
  first, a strong rule's eigenvariable), `∀` otherwise, which keeps the
  eigenvariable conditions of both halves. A term that holds an earlier
  binder's variable is bound right inside that binder (LJ's `∀x. r(x,
  f(x)) ⊢ ∀x. ∃y. r(x, y)` gives `∀x. ∃y. r(x, y)`). The check holds the
  interpolant's predicates, functions, constants and atoms to both
  halves and wants it closed. 300 generated sequents split at random in
  the three logics (a one-off run of 2 000: 1 876 interpolants, 1 875
  checked, no defect).
  - A latent fault found on the way, fixed for both kinds: the root
    inference may list the sequent's formulas in another order than the
    derivation's sequent (the connection read-back's weakenings do), and
    Maehara's method read the halves by position, giving `⊤` for `r,
    p(a) ⊢ p(a)` split at `p(a)`. The root's halves are now drawn by
    formula. No propositional derivation the tests make reorders.
- `zwzuoryq`: the status chapter. The manual and the guide had 77a's
  normal forms already; this chain added the first-order translations,
  Herbrand's theorem and the first-order interpolants to the
  transformations chapter, the protocol chapter and the guide.
- `cnf`, `dnf`, `tseitin` and `plaisted-greenbaum` still refuse a
  first-order sequent: no caller needs them (Herbrand's theorem works on
  the clausal form).

**Departures from the design**:
- Herbrand's valid verdict carries the derivation of LK when it is
  built; where a bound refuses the derivation, it carries the ground
  instances' clausal proof instead, which rests on Skolem's theorem and
  the clausal form's structural check (departs from the design, which
  wanted the verdict on the checker alone; the prompt asks for the
  instances' certificate, which this keeps).
- A sequent whose instances over a finite universe are satisfiable is
  not valid, but Herbrand's search answers unknown (`Incomplete`): no
  checked countermodel is built there, and `prove` gives one (departs
  from the design).
- The deskolemized derivation's witnesses are every term of the kept
  instances for every `∀` hypothesis and `∃` goal, not the instances'
  own terms per quantifier: a clause's variables do not name the
  sequent's quantifiers after prenexing and distribution. The cut keeps
  the set small (departs from the design).

### Fresh review

`fresh-reviewer` (Fable 5.2, `medium`, the row's reviewer). No wrong
"valid" anywhere: 400 random sequents with function symbols and 300
valid by construction agreed with `prove --logic classical`; the
translations (570 runs, no disagreement) and the interpolants (780 runs
in the three logics, no error, about 25 hand-made splits with a function
of one half and a constant of the other) held. Four findings, all in
Herbrand's part, all fixed in `tzrrwkmz`, each with a test:
- **A valid sequent ended in `Error::ReadBack`**: `∀y. ∃x. p(x, y), ∃x.
  p(x, a) ⊢ ∃x. ¬¬p(x, a)`, where an instance of the first hypothesis is
  the second, which met both Skolem steps and took the first's term,
  while the kept instances used the second's. `skolem` now takes the
  step whose term the kept instances use.
- **A propositional sequent's JSON held a derivation its reader
  refused**: the fresh constant `c1` gave the arena first-order tables
  without its atoms' entries. No constant is made where no clause has a
  variable.
- **The instance enumeration asked no stop**: `--timeout 2s` ran past
  100 s. The universe's terms and the instances ask it once in 1 024.
- **A solver ended by a bound read as "the instances have a model"**:
  its unknown now ends the search for its reason (`memory limit`), and
  the protocol's `Limit` is the reason's name and setting, as for every
  other ordinary operation.

### Checks

- `cargo clippy --workspace --all-targets -- --deny warnings` and
  `cargo test --workspace` after each commit and after the review's
  fixes; README's, the manual's and the guide's examples run (the
  manual's new `herbrand`, translation and interpolation blocks among
  them).
- Both `cargo hack` runs and `cargo hack check --feature-powerset --depth
  2 --tests -p deduxit` with warnings denied, after the gate's first run
  found `term_text` behind `interactive` (moved to the general block) and
  `Map::Replace` built only with `sat` (gated), both folded into
  `tzrrwkmz`, and a test variable only `interactive` reads (into
  `pzuyoqvp`).
- The gate passed on the chain before the review (the ratchet under its
  ceilings; `render-latex` +1.87 %, no journey runs the new code).
- The behaviour lock: `vynyznnu` adds `herbrand`'s three calls and
  reblesses `workspace-operations` and `error-no-command` (the new
  operation and command).
- The full check's first run (15:09 to 15:29, after the rebase onto
  `main@origin`, 76b's code with it) failed in `doc-private` (a link to
  `super::decide`, both a function and a module, now `fn@`, folded into
  `tzrrwkmz`) and `budget`: the web module at 2 767 129 bytes at gzip,
  past 2 750 000 (76b's 2 708 195 plus about 59 KB for Herbrand's
  theorem, the interpolants' quantifiers and the protocol's operation);
  `vqwryqkz` raises the ceiling to 2 800 000.
- The rebase met 76b's wire level 27 (Kripke models with growing
  domains): Herbrand's form is level 28 (`wire::HERBRAND`, the schema's
  `minimum`, the protocol's kind, the manual, the guide, the lock).
- `nix flake check` under the machine lock at the report's commit
  (15:30 to 15:52), on `main@origin` (`nkluqwmk`, 76b's code with it):
  every check passed, the budget's module at 2 767 129 bytes. Only this
  line changed after.

### What 77c must know

- Glivenko's translation refuses a first-order sequent
  (`Error::FirstOrder`, "takes propositional sequents only"); the
  failure 77c shows (`⊢ ∀x. (p(x) ∨ ¬p(x))` classically, its double
  negation not intuitionistically) is the one the manual and the module
  docs now name, so 77c's infinite model fits that text.
- `compare` at first order needs the copy bound lifted on the
  intuitionistic side (the library's default of 3 is too low for
  Kuroda's and Kolmogorov's images of small sequents); the command lifts
  it.
- Wire level 28 is Herbrand's (76b took 27). 78a may add a level of its
  own: whichever lands second renumbers (`wire::LEVEL`, the table, the
  schema's `minimum`, the protocol's kind `level`, the lock's
  `herbrand-json`).
- Herbrand's search answers unknown for a sequent that is not valid; a
  checked Herbrand model over a finite universe would be a small
  addition if 76's structures are wanted there.
