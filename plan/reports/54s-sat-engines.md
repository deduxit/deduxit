# Step 54s: SAT-based engines for intuitionistic, minimal and classical logic

The step's sessions each add a section here: 54s-a (the engine), 54s-b
(classical logic by CDCL), 54s-c (the measurement and the default).

## Session 54s-a: the engine

Opus 5.5 at `high`, in the jj workspace `../deduxit-wt-step-54s-a-f3463c`,
from `plan/54s-sat-engines.md` (item 1 of "The sessions") and the brief
`start-54s-a.md` of the planning session, on 2026-11-14, cores 4 to 6.
The author was away; every decision below was made by reading the papers
and the pinned sources, and its reason is given.

### Outcome

- **`ordinary::sat::decide`** decides intuitionistic and minimal
  propositional sequents by intuit's search with intuitR's restarts over
  the SAT solver batsat, and is `ordinary::Method::Sat` (`sat`) beside
  `embedding` and `ljt`, behind the new default feature `sat`. Every
  "valid" comes with a derivation of LJ that `Derivation::check` accepts,
  read off the search (LJT only where a bound refuses that); every "not
  valid" with a finite Kripke model (`sat::Model`) that `Model::refutes`
  checks by computing forcing within `Model::MOST_CELLS`.
- **On ILTP it agrees with LJT everywhere**: all 400 verdicts LJT gives
  at 2 s (200 problems per logic, `core/tests/iltp/ljt.csv`) are the SAT
  engine's too, every derivation read off the search (none from LJT) and
  passing the checker, every model refuting its sequent; the whole set
  takes 0.25 s in release at `rzwuzkol`, the slowest problem 0.17 s
  (SYJ212+1.016, intuitionistic), on cores 4 to 6 of a shared machine
  (not a timed run). The test, `sat_agrees_with_ljt_on_iltp` in
  `core/tests/iltp.rs`, is ignored as the LJT test is (the library is
  not in the repository): `cargo test --release -p deduxit --test iltp
  sat_agrees -- --ignored` with `bench/iltp` built, "400 verdicts of LJT
  agreed; undecided: []", 0.30 s.
- **The unit tests** (`core/src/ordinary/sat/mod.rs`): agreement with LJT
  on 400 generated sequents in both logics with every derivation read
  off (`agrees_with_ljt`), the separating sequents of both logics, a
  planted wrong model refused for each of the check's seven reasons
  (`a_planted_wrong_model_is_refused`), the bounds (work, stop, memory,
  the variables' index bound, the derivation's size, the model's cells),
  the stop asked in the last scan of the worlds, and the LJT route.
- **Batsat** is in `core/` only, under `cargo deny check` ("advisories
  ok, bans ok, licenses ok, sources ok"), and builds for
  `wasm32-unknown-unknown` (the flake's `wasm` check lists `sat`; the
  bindings enable it).
- **The checks**, at `rzwuzkol` (the chain's last code change): the
  gate on cores 4 to 6 ("gate: passed": clippy, the workspace's tests,
  both `cargo hack` runs, the ratchet; one cache-miss notice on
  `search-qbf-20-2`, never a failure); `nix flake check` under the
  machine lock ("all checks passed!", the `wasm` check with `sat` in its
  list and `web-wasm-tests` among them); the focused engine's target set
  (a copy of `bench/targets.sh` with its streams on cores 4 and 5, since
  2 and 3 are kept for timed runs; the counters do not depend on the
  core) against `bench/targets/47.csv` with `summary --against …
  --counters`: no verdict differs, and none of the 145 rows decided alike
  differs in `nodes`, `splits`, `memo_hits`, `memo_entries`, `links` or
  `tests`. The report's own change is checked again by `nix flake check`
  at its commit.

### The panel and the reviewer

Each member ran from a fresh context on the engine, the read-back and
the model's check (`panel-argument` on Fable 5.2, `panel-counterexamples`
on Opus 5.5, `panel-integers` on Sonnet 5.5, all at `high`), on the
first change `tnlvtqnu`.

- **Argument: the soundness argument holds.** It stated the forcing
  invariant of `Model::refutes` from the code and checked every line,
  traced every path of `run` (no verdict leaves without the checker or
  the model's check), and the completeness side: the polarity invariant
  of the model and every clause kind's rule in the tableau. Its probe,
  19 000 decisions with equivalences in both logics and 15 crafted
  cases, found no disagreement with LJT, no error and no LJT fallback.
  - *The check's table is a byte per cell, not a bit as `MOST_CELLS`
    says* (up to 128 MiB, not 16): fixed, the table is bits
    (`xuztxvsr`).
  - *A sentence of the rules file stated the polarity invariant the
    wrong way round*: rewritten (`xuztxvsr`).
- **Counterexamples: no wrong verdict.** About 1.08 million decisions of
  random sequents (atoms a to f, every connective, depth up to 5) in both
  logics, each against LJT, a truth table, its own exact decision
  procedure (worlds as locally consistent assignments, eliminated to a
  fixpoint) where it applies, and its own forcing; 2.73 million
  hand-built or mutated models against `refutes`; the first 99 ILTP rows
  at a work bound: nothing contradicted, no `ReadBack`, `Countermodel`
  or panic.
  - *Building the model's order ignored the memory bound and the stop*
    (`⊢ B_16`, `B_k = (p_k → B_{k-1}) ∨ (¬p_k → B_{k-1})`, 184 423
    worlds: 4.2 GB and 111 s without a poll): fixed (`oyqkppwu`), the bit
    per pair of worlds is charged to the account, the stop is asked
    every 256 comparisons, and the worlds stay charged while the model is
    made (`the_model_s_order_keeps_to_its_bounds`).
- **Integers: two missed bounds, the rest safe.** It checked every cast
  of `sat/`, `MOST_VARIABLES` against batsat's literal encoding, the work
  counters, the account's releases (over-release cannot happen: counted
  against 112 123 deletions), the cells, and the builder's ids.
  - *The last scan of the worlds asked no stop* (a disjunction of 5 000
    implications: 50 s without a poll, cubic in the worlds): fixed
    (`xuztxvsr`), the scan asks every 256 pairs, reported with
    `Progress::work` 256 (`the_scan_of_the_worlds_asks_the_stop`).
  - *The setup built the solver before the account was compared* (8
    million conjuncts: 6.8 GB before the refusal): fixed (`xuztxvsr`),
    each variable is charged (`VARIABLE_BYTES`, 128) and each clause as
    it is added, and the setup refuses once over the bound, never holding
    the clauses beside the solver.
  - *The final conflict of a refuted call was charged and never
    released*: fixed before the report reached me (`xuztxvsr`).
  - *Refusals without a test*: the cells refusal before the model is
    built and the edges' part of the cells are not reached by a test
    (reaching them takes a model of millions of worlds); the `u32`
    refusal of the derivation's ids cannot be reached before the size and
    memory bounds. Left as they are, said here.
- **SYJ205, found by my own ILTP run, not the panel**: with the world
  assumed before `a` and `¬b`, the clauses learnt carried the world's
  arbitrary choices, one per subset of hypotheses (32 890 at size 13,
  sizes 14 to 20 undecided at 20 s). Assuming `a` and `¬b` first
  (`xuztxvsr`) gives 4 clauses at size 3 where there were 41, and the
  whole ILTP set in 0.23 s. Examining the newest world first, tried on
  the way, left SYJ212+1.020 undecided; the oldest first stays.

The fresh-context reviewer (`fresh-reviewer`, Fable 5.2 at `medium`) on
the chain up to `oyqkppwu`, against the brief, the shared rules and item
1 of the prompt: **no findings**. Its own probe, 1 654 344 random
sequents in both logics against LJT with every derivation and model
checked again, and 9 926 064 planted models against its own forcing,
found nothing. Its notes, each answered:
- *A stop during the LJT route was reported as the read-back's
  refusal* (documented, raised as a question): changed, a stop stays a
  stop, tested in `ljt_gives_the_derivation_the_read_back_cannot`
  (`rzwuzkol`).
- *The test build without default features fails in
  `core/src/lambda/tests.rs`* (the parser used without a `parse` gate,
  lines 407, 786, 855, 869): not this chain's, and not fixed here; a
  follow-up for the lambda module's owner.

### The algorithm as built, and its sources

- **Sources read** (by a Sonnet sub-agent at `high`, whose extraction I
  checked against the code I then wrote): Claessen and Rosén, LPAR-20
  2015, read in the LaTeX source of the paper in the authors' repository
  (the LNCS PDF was not reachable); Fiorentini, CADE-28 2021, the
  author's PDF with its appendix (the prompt names Fiorentini and
  Ferrari; the CADE paper has Fiorentini alone, Ferrari is on the later
  JAR 2024 paper); Fiorentini, Goré and Graham-Lengrand, TABLEAUX 2019,
  the author's PDF. intuitR's code is BSD-3-Clause and intuit's
  repository has an empty `LICENSE` (no grant); neither's code was read,
  only intuitR's licence and README.
- **The clauses** (`sat/mod.rs`, `Search::new`). Every formula of the
  sequent is a variable of the solver, not a fresh atom: a flat clause is
  then a formula's own rule read as an implication between its parts
  (`x → a`, `x → b` and `a ∧ b → x` for `x = a ∧ b`; `x ∧ a → b` and
  `b → x` for `x = a → b`; …), and the read-back needs no definitions to
  unfold. Which clauses a formula gets follows its polarity, as intuit's
  one-directional definitions do: a negative formula (a hypothesis, the
  antecedent of a positive implication) its left rule, a positive one its
  right rule, and a positive implication or negation its implication
  clause `(a → b) → x` (`¬a` as `(a → ⊥) → x`). The polarity is one pass
  from the arena's last formula down; an equivalence gives both
  polarities to its parts. Hypotheses are unit clauses; the goal is
  assumed false; `⊥` is a variable, false by a unit clause in
  intuitionistic logic and free in minimal logic (minimal logic is the
  same search with `⊥` an atom, as the prompt says).
- **The loop is intuitR's `proveR`** (`Search::run`, `realise`): worlds are
  the solver's models of the flat clauses with the goal false; a pair of
  a world `w` and an implication clause `(a → b) → c` is realised when
  `a`, `b` or `c` holds at `w` or a world above `w` has `a` and not `b`;
  the first unrealised pair calls the solver under `w`'s true variables,
  `a` and `¬b`. A model is a new world; a refutation learns
  `failed \ {a} → c` from the failed assumptions and restarts with no
  worlds (the CADE paper's S6). Where no pair is unrealised, the worlds
  ordered by inclusion are a Kripke model in which the goal fails
  (Proposition 2 of the CADE paper). Realisation only grows with the
  worlds, so one cursor walks the pairs per round. The pair order (the
  paper leaves it open) is the worlds in the order found, then the
  implications in the arena's order.
- **"Valid": the derivation read off the search** (`sat/read_back.rs`),
  by the correspondence of the TABLEAUX paper: the run is a derivation
  of the CADE paper's calculus C→, a branch of learnt clauses closed by
  one classical call. Each learnt clause `B → c` (with `c = a → b`)
  becomes the formula `b₁ → (… → (bₙ → c))`, derived by `→R` from a
  derivation of `Γ, earlier lemmas, B, a ⊢ b`, and cut below the
  derivation of `Γ, lemmas ⊢ C`, so that its uses are runs of `→L`. Each
  classical call (flat clauses entail an atom) is turned into LJ by a
  tableau: the rules the clauses stand for are applied forward (left
  rules on negative formulas and on the lemmas' formulas, right rules on
  positive formulas, each of the latter a cut), and a negative disjunction
  is split only where nothing else applies. Since the clauses entail the
  goal classically (the TABLEAUX paper's Lemma 2: flat clauses decide an
  atom alike in both logics), every branch closes; one that does not is
  a defect (`Error::ReadBack`). Each derivation keeps only the hypotheses
  it uses, a step whose formulas are unused is left out, and a split whose
  first branch ignores its disjunct keeps that branch alone.
- **LJT where a bound refuses the read-back** (`by_ljt`): the brief's
  "else LJT on the same sequent within the limits". The read-back reaches
  every proof (the tableau is complete), so LJT is asked only where the
  derivation's size, the memory or the work refuses it; where LJT gives
  no derivation either, the read-back's refusal is the answer, which
  `gives_way` treats as LJT's own today. LJT refuting a sequent the
  search proved is `Error::ReadBack`.
- **"Not valid": the Kripke model** (`sat/model.rs`): `Model` (worlds, the
  root first) and `World` (the atoms forced by index, `⊥` forced in
  minimal logic, the edges to the worlds directly above, each a later
  world). The engine's model is the last round's worlds sorted by size,
  with the covering pairs of inclusion as edges, each world its true
  atoms. `Model::refutes(&sequent, logic, stop)` checks it independently:
  edges forward, the root below every world, atoms ascending and
  existing, monotone along every edge, `⊥` only in minimal logic; then
  forcing of every formula of the arena at every world, the last first
  (an implication from its local value and the worlds directly above,
  `¬A` as `A → ⊥`), the root forcing every hypothesis and not the goal.
  Its bound is declared at the type: `Model::MOST_CELLS` (2²⁷ cells, a
  world or an edge times a formula, a bit each), refused as
  `Refusal::Memory` past it, and the engine refuses before building a
  model of more worlds times formulas.

### The dependency and why

- **batsat 0.6.0** (MIT, pure Rust, one dependency `bit-vec` 0.5.1),
  added with the `new-tool` skill in `core/` only, optional, behind the
  new default feature `sat` (`sat = ["dep:batsat"]`), without its
  `logging` feature. The research note (`plan/notes/research/sat-rust.md`,
  section 4) chose it; I read its pinned source for what the engine
  relies on: `Solver<Cb: Callbacks>`, `solve_limited`, `value_lit`
  reading a copy of the model after the trail is popped, `unsat_core`
  being `analyze_final`'s conflict (the negations of the failed
  assumptions, which the trait negates back), `Callbacks::stop` polled
  in `within_budget` at every decision without a conflict, and
  `on_delete_clause` called by `reduce_db` for learnt clauses only.
- **The command and the page get it**: `sat` is in the features the
  command's and the bindings' manifests name, and in the flake's `wasm`
  check's list; CLAUDE.md's feature list and the crate docs say so.
- **The trait** (`sat/solver.rs`, crate-private): `Solver` with
  `new_var`, `add_clause`, `solve(assumptions) -> Answer`
  (`Satisfiable`, `Unsatisfiable`, `Stopped(Reason)`), `value`, `failed`,
  `calls`, `conflicts`; `Batsat` implements it over
  `batsat::Solver<Bridge>`. `Bridge` is the callbacks: the caller's stop
  (a `&mut dyn FnMut(Progress) -> bool` in a `RefCell`, since batsat
  polls through `&self`), the work count (a conflict, that is a clause
  learnt, or a call of `solve`, each a unit of `limits.work`), and the
  memory account (every clause added and learnt charged by its literals
  and 32 bytes, a deleted one released). The stop is also asked once per
  call before solving, so that a call that makes no decision still polls.

### Decisions, each with its reason

- **Variables are formulas, not fresh atoms.** intuit's clausification
  introduces a fresh atom per subformula; here the subformula itself is
  the variable. The clauses are the same up to renaming, and the
  read-back then needs no definition of a fresh atom to unfold: every
  clause is a rule of LJ on formulas the sequent has.
- **One method `sat` for SAT-based engines** (`Method::Sat`, the module
  `ordinary::sat`): 54s-b's classical CDCL is the same kind of engine on
  the same solver, so `Method::Sat` can decide classical logic too when
  it lands (`Method::decides` then changes). Named after what the user
  chooses (a SAT solver), not after a prover.
- **The variant is behind the feature** (`#[cfg(feature = "sat")]` on
  `Method::Sat` and `Search::Sat`, two `ALL`/`NAMES` constants by cfg):
  every item stays live in every combination (`cargo hack` shows it), and
  both enums are `#[non_exhaustive]`, so the feature is additive.
- **The model stays on `sat::Outcome`**, not on `ordinary::Outcome` or
  `Verdict::NotValid`: the brief makes it data and a test here and step
  55 an output kind, which is where `Verdict` or the outcome's JSON
  change.
- **`Error::Countermodel` is a new variant of kind `defect`** (code
  `countermodel`): a model the engine found that fails the check is the
  engine's fault, as `ReadBack` is. A model given by a user (step 55)
  failing would be `invalid`; that is step 55's to add.
- **The read-back falls back to LJT on a bound only** (not on the stop,
  nor on a defect): the brief's "else LJT on the same sequent within the
  limits". A test-only switch (`run`'s `read_off`) takes that route,
  since no small input makes the read-back's derivation larger than
  LJT's.
- **A refused model is a refusal before it is built**: worlds times
  formulas past `MOST_CELLS` is `Refusal::Memory`, so no world index is
  cast past `u32` and no model is built that cannot be checked.
- **The solver's final conflict of a refutation is released from the
  account**: batsat reports it through `on_new_clause` as a learnt clause
  but does not keep it, and charging it would have grown the account by
  a clause per refuted call.
- **What I did not build**: no `--method sat` in the command (54s-c's),
  no JSON for the model (step 55), no measurement (54s-c). The harness's
  ordinary child answers `sat` with its existing "does not run" row.


### What the next sessions must know

- **For 54s-b (classical CDCL).** The solver is behind the crate-private
  `ordinary::sat::solver::Solver` (`solver.rs`); `Batsat` holds
  `Bridge` as batsat's `Callbacks`, so DRUP logging goes into `Bridge::on_new_clause`/`on_delete_clause` (today they
  count work and charge memory). `on_new_clause` also reports the final
  conflict of a refuted call (the core, not kept by the solver):
  `Batsat::solve` releases its charge, and a proof logger must not log
  it as a lemma. `Method::Sat` is the method to extend
  (`Method::decides` says intuitionistic and minimal today; `decide.rs`
  dispatches `Method::Sat` to `sat::decide`, which refuses classical
  logic with `Error::Logic`). batsat's `add_clause_reuse` sorts the
  clause and drops literals false at level 0 without telling, which a
  DRUP checker must accept (RUP does).
- **For 54s-c (the measurement and the default).** The command does not
  know `sat` yet: clap's `MethodArg` has two values, and a settings file
  whose `ordinary.method` is `sat` reaches `prove`'s image path, which
  runs the embedding without saying so (`prove.rs` asks only `==
  Method::Ljt`); wiring `--method sat` fixes both. The harness's ordinary
  child answers `sat` with "the harness does not run sat"
  (`bench/src/ordinary.rs`). `sat::Statistics` (`calls`, `conflicts`,
  `lemmas`, `worlds`, `facts`, `ljt`) serializes under the outcome's
  `statistics` with `engine` `sat`. The bindings get the method through
  `ordinary::decide` already.
- **For step 55 (countermodels).** The model is `sat::Model` on
  `sat::Outcome::model`, checked by `Model::refutes` before it is
  returned; `ordinary::Outcome` drops it. Worlds are sorted by size, the
  root first, edges are the covering pairs of inclusion, atoms by index
  into the sequent's atom names (the arena the engine adds to keeps
  them). A model a user gives that fails the check needs an error of
  kind `invalid`, beside `Error::Countermodel` (a defect). intuitR
  promises no smallest model.
- **The read-back's cost**: every lemma's tableau takes every earlier
  lemma's formula as a hypothesis (only the lemmas the derivation uses
  are derived, from the last down), so a run of thousands of lemmas
  costs quadratically there. On ILTP the most lemmas are 3 752
  (SYJ212+1.016, intuitionistic) and 858 (SYJ206+1.009, minimal), each
  problem with its read-back within 0.17 s.
- **The ILTP test's oracle** is `core/tests/iltp/ljt.csv`, LJT's 400
  verdicts at 2 s copied from `bench/defaults/ljt-iltp.csv`; a new timed
  run that changes LJT's verdicts changes it.
- **Follow-ups**: `read_back.rs`'s derivation builder `Lj` is a copy of
  LJT's (`ljt.rs`), push, weakening, contraction and `in_order`; one
  shared builder in `derivation.rs` would serve both (not done: it would
  have touched LJT's translation).
