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

## Session 54s-b: classical logic by CDCL

Opus 5.5 at `high`, in the jj workspace `../deduxit-wt-step-54s-b-d8f4a6`,
from `plan/54s-sat-engines.md` (item 2 of "The sessions") and the brief
`start-54s-b.md` of the planning session, on 2026-11-14, cores 10 to 12.
The author was away; each decision below gives its reason.

### Outcome

- **`ordinary::sat::decide` and `Method::Sat` decide classical logic**
  by one call of batsat on deduxit's own Tseitin clauses of the
  sequent's negation (`sat/classical.rs`). "Not valid" comes with a
  falsifying assignment, a `sat::Model` of one world that
  `Model::refutes` checks by evaluation. "Valid" comes with a DRUP proof
  recorded in memory from batsat's `on_new_clause` and
  `on_delete_clause`, which deduxit's own RUP checker (`sat/drup.rs`)
  accepts against the encoder's clauses, and then with the LK
  derivation of the embedding.
- **drat-trim accepts the recorded stream** (2023-05-22 from nixpkgs, a
  test-time program): "s VERIFIED" for Peirce's law and for the
  pigeonhole sequent of eight pigeons, whose proof has 2 553 steps with
  1 050 deletions (`drat_trim_accepts_the_proof`, ignored, run with
  drat-trim on the `PATH`).
- **A planted wrong step is refused** (`a_planted_wrong_step_is_refused`
  on hand-made proofs, `a_planted_wrong_proof_step_is_refused` on a real
  one).
- **On ILTP the method agrees with the embedding on every problem the
  embedding decides** at 2 000 000 units of work and 2 GiB
  (`sat_agrees_with_the_embedding_in_classical_logic` in
  `core/tests/iltp.rs`, ignored as its neighbours; release, cores 10 to
  12, 391 s at `ntquonzl`, most of it the stopped pigeonhole runs): 124
  verdicts agree, none differs, no problem is decided by the solver
  alone against the embedding. 132 more are valid by a checked clausal
  proof without a derivation from the embedding: on 127 its search
  stops at the work bound (the method answers unknown with that reason
  and `proof_steps` above zero), on 4 the read-back's derivation is past
  its 64 MiB bound and on SYN007+1.014 the translation passes the
  occurrence bound (both refusals). 18 (SYJ202 and SYJ208 from size 12,
  pigeonhole formulas, exponential for resolution) stop the solver at
  the bound, where the embedding fails too. So the CDCL decides 256 of
  the 274 problems with a checked certificate; the API lets it report
  124 of them (see "Decisions").
- **The checks**: the gate at `wlvqmtux` on cores 10 to 12 ("gate:
  passed"), and `nix flake check` under the machine lock at the report's
  commit (below, "The checks").

### The encoder

- **`ordinary::sat::classical::encode`** (crate-private) is Tseitin's
  encoding with polarities (Plaisted and Greenbaum, J. Symbolic
  Computation 2, 1986). Every formula the sequent holds is a variable,
  numbered from 0 in the arena's order; a formula that must be true where
  the sequent fails (a hypothesis, and the parts that make a true formula
  true) gets "formula → definition", one that must be false "definition
  → formula", an equivalence gives both polarities to its parts. Unit
  clauses make every hypothesis true and every formula right of `⊢`
  false, so several formulas right of `⊢` and empty sides need nothing
  special.
- **Its soundness argument**: every definitional clause holds when each
  variable takes its formula's value under any assignment of the atoms,
  and the units hold exactly where that assignment falsifies the
  sequent; so the clauses have a model wherever the sequent is
  falsifiable, and a refutation of them shows it valid.
- **Why polarities**: half the clauses of full Tseitin, the usual
  practice of structure-preserving clause forms, and the same
  one-directional definitions 54s-a's intuitionistic clauses use. It has
  its own polarity pass (no `⊥` variable, equivalences without the extra
  implication nodes), so the trusted encoder reads alone.
- **"Not valid"**: the solver's model read on the atoms' variables is a
  `sat::Model` of one world (the brief's "one-world case of step 55's
  countermodel", the same data), and `Model::refutes` now takes classical
  logic: one world, no `⊥`, every hypothesis forced, no formula right of
  `⊢` forced, which is classical evaluation and needs nothing of the
  encoder.

### The proof stream, and what drat-trim showed

- **Recorded in memory from batsat's callbacks** (`Batsat::recording`,
  in `solver.rs`): `on_new_clause` appends each learnt clause, units
  included, and `on_delete_clause` each clause `reduce_db` deletes, as
  `drup::Step`s charged to the memory account. batsat never learns the
  empty clause (a conflict at level 0 returns at once), so the run
  appends it, and the checker must derive it by RUP. A refuted call
  under assumptions reports its final conflict as a learnt clause, which
  `solve` pops from the proof (classical logic calls with none).
- **drat-trim accepts it.** The ignored test
  `drat_trim_accepts_the_proof` writes the encoder's clauses as DIMACS
  and the proof as DRUP text for Peirce's law and the pigeonhole sequent
  of eight pigeons and seven holes, and runs drat-trim 2023-05-22 from
  nixpkgs (MIT; a test-time program from `nix build nixpkgs#drat-trim`,
  never a dependency). Its output, run at `vukwwvlw` and again, the
  same, at `wlvqmtux`:
  - Peirce: 6 clauses, 1 step; "UNSAT via unit propagation on the input
    instance", "s VERIFIED". drat-trim also printed "turning on binary
    mode checking … wrong binary prefix" for the proof that is the empty
    clause alone: its format guess reads the two bytes `0\n`; harmless.
  - Pigeonhole 8/7: 644 clauses, 2 553 steps of which 1 050 deletions;
    "1201 of 1504 lemmas in core using 20858 resolution steps", "s
    VERIFIED", 0.040 s.
  - Two warnings on the pigeonhole proof, "deleted clause on proof line
    … does not occur". I traced them to batsat's `remove_satisfied`
    (`core.rs`), which trims literals false at level 0 from learnt and
    original clauses in place without telling the callbacks: the later
    deletion names the trimmed clause, which was never added. RUP needs
    no trimmed clause (the longer one stays, and the level-0 units are
    propagated again), so the stream is complete; deduxit's checker
    ignores such a deletion as drat-trim does.

### The checker and its integers

- **`ordinary::sat::drup::check`** (crate-private, in the style of
  `linear::search::refutation`'s check: no counter that can wrap, each
  bound said at its declaration) keeps the formula's and the proof's
  clauses not deleted, checks each added clause by reverse unit
  propagation (RUP: its literals assigned false, the unit clauses
  propagated, a clause met with every literal false), and accepts at the
  first empty clause it adds. Hints are found by propagation, as the
  brief allows: no fork of batsat.
- **Each check starts from no assignment and undoes everything after**,
  so nothing but the clauses and their two watched literals carries from
  one step to the next. Deleting a unit or a reason clause therefore
  cannot leave a stale assignment, the known fault of DRUP checkers that
  keep a top-level trail. The price is that each check propagates the
  unit clauses again; on the sizes deduxit decides that is cheap (the
  checks below take milliseconds).
- **Its soundness needs** that propagation assigns a clause's last
  non-false literal and calls a conflict on a clause all false, and the
  watch invariant those rest on (a clause in the list of `l` has `l`
  among its first two literals); the watches' completeness only avoids
  false refusals. Clauses are sorted
  and deduplicated when kept, a tautology is never kept, a deletion of a
  clause not kept is ignored (sound: the clauses kept are then more,
  each still a consequence of the formula; drat-trim does the same with
  a warning), a literal past the formula's variables is a flaw.
- **Its integers**: literal codes `2·var + sign` fit a `u32` since every
  variable is below `MOST_VARIABLES` (`u32::MAX / 2 − 1`); clause indices
  are `usize`, bounded by the clause vector's length; `steps` is bounded
  by `limits.work` and by the proof held in memory; `ticks` resets at
  `POLL` (4 096). Bounds: a step is a unit of `limits.work`
  (`Refusal::Work`), the clauses, watches and index are charged to the
  memory account (`Refusal::Memory`), the stop is asked every `POLL`
  units of work: a step, a clause kept, a unit assigned, a literal
  propagated, a clause visited (`Refusal::Stopped`), each tested
  (`the_bounds_refuse_a_check`).
- **A planted wrong step is refused**: `a_planted_wrong_step_is_refused`
  (the checker alone: a step that does not follow, a missing empty
  clause, a deletion a later step needed, a literal of no variable) and
  `a_planted_wrong_proof_step_is_refused` (a real proof of the
  pigeonhole sequent of five pigeons with its learnt clauses taken away,
  and with a unit planted before them; a one-world model with a true
  conclusion, and one of two worlds).
- **A failed check is `Error::ReadBack`** (calculus LK, kind `defect`):
  the stream is the engine's own, so a flaw is the engine's fault, as a
  derivation failing its checker is.

### Decisions, each with its reason

- **One method `sat` for all three logics** (`Method::Sat`,
  `Method::decides` now true for classical logic): 54s-a's report
  anticipated it, and the user chooses a SAT solver, not a calculus.
  `sat::decide` dispatches classical logic to `classical::decide`.
- **"Valid" still carries an LK derivation, the embedding's**
  (departs from the design). The prompt has "valid" come with the
  checked proof "and with the LK derivation where the existing
  read-back reaches it". `ordinary::Verdict::Valid` holds a derivation
  and nothing else, and every engine's "valid" today is a checked
  derivation; a verdict "valid by a checked clausal proof, no
  derivation" is an API change (`Verdict`, its JSON, the command's
  output, the bindings, the harness), which `writing.md` and CLAUDE.md
  send to the author. So, within the API as it is: the clausal proof is
  checked first, always, and then `ordinary::decide` with
  `Method::Embedding` gives the derivation within the same limits; where
  the embedding's search stops first, the outcome is its
  `Verdict::Unknown` with its reason (the bound that stopped it, which
  names the setting that lifts it), and `Statistics::proof_steps`
  above zero says the clausal proof passed; where the read-back of its
  proof is refused, that refusal, as for LJT's derivation past its
  bound. The embedding refuting a sequent whose proof passed is
  `Error::ReadBack`.
  **Recommendation for the author**: a verdict for a sequent valid by a
  checked certificate other than a derivation, so that the method
  answers "valid" where the embedding does not reach a derivation (the
  idiom of `bv_decide` in Lean and of the SAT Competition's certified
  track: the checked LRAT/DRUP proof is the certificate). The most
  direct form is a second payload of `Verdict::Valid` (an enum of a
  derivation or a clausal proof), or a variant beside it; either is
  54s-c's or a later step's, after the author's answer.
- **The proof is not on `sat::Outcome`**: the checked proof is dropped
  after its check, and `Statistics::proof_steps` and `clauses` say what
  was checked. No caller needs the proof yet (the crate's rule "an item
  without a caller waits for one"); step 70 is its first.
- **The embedding's stop is a trait object** (`by_embedding`):
  `ordinary::decide` reaches `sat::decide` again through `Method::Sat`,
  and a generic stop wrapped itself in `&mut` without end at compile
  time.

### Changes to 54s-a's files

- `solver.rs`: `Bridge` gains `proof: Option<Vec<Step>>`, appended by
  `on_new_clause` and `on_delete_clause` (`Bridge::record`, charged to
  the account) where `Batsat::recording` asks for it; `Batsat::proof`
  takes it; `solve` pops a refuted call's final conflict from the proof
  as it already released its charge; `Batsat::ours` turns batsat's
  literal into ours (used by `failed` too); `Solver::new_var` documents
  the numbering from 0.
- `model.rs`: `Model::refutes` takes classical logic (one world, no
  `⊥`, every formula right of `⊢` unforced) instead of refusing it.
- `mod.rs`: the dispatch, the two statistics, the docs; the test that
  pinned `Error::Logic` for classical logic now pins a classical verdict
  (the behaviour was meant to change).
- `ordinary/mod.rs`: `Method::Sat`'s docs and `decides`;
  `valid_by_truth_table` (a test helper) is `pub(super)` for the
  classical tests.

### The panel and the reviewer

Each member ran from a fresh context on the change `vukwwvlw` (the
encoder, the checker, the recording, the one-world model's check):
`panel-argument` on Fable 5.2, `panel-counterexamples` on Opus 5.5,
`panel-integers` on Sonnet 5.5, all at `high`, on cores 10 to 12.

- **Argument: the argument holds.** It stated the invariants from the
  code first (every definitional clause true under the formulas' own
  values; every kept clause entailed; every trailed literal implied; the
  watch invariant; the full undo on every exit), checked every site of
  the encoder, the checker, the model's check and the recording, and the
  converse of the encoder (a model of the clauses falsifies the
  sequent). Two places where my argument in the rules differed from the
  code, neither a fault:
  - *Soundness rests on the watch invariant too*: `propagate` takes a
    clause's second literal to be the one just falsified without
    testing it, so a clause in the list of `l` must have `l` among its
    first two literals. The code keeps it at every site; the rules now
    say so (`wlvqmtux`).
  - *A deletion of a trimmed clause may delete another kept clause of
    the same literals*, not only match nothing: sound, since a deletion
    only weakens; the rules now say so (`wlvqmtux`).
- **Integers: nothing wraps, truncates, panics or underflows**; every
  limit it drove ends in a refusal (the variables' bound, the solver's
  and the checker's work, memory, the stop; `empty` cannot underflow,
  driven with overflow checks on). Two findings, both fixed in
  `wlvqmtux`:
  - *The check's stop polls came late*: each check assigns every unit
    clause, which was not counted, so with a million unit hypotheses
    polls came 0.6 to 1.3 s apart, with three million up to 7 s. Now a
    unit assigned, a literal propagated and a clause of the formula
    kept each count toward `POLL`; the test drives a check of 4 096 unit
    clauses eight times and asserts eight polls at least
    (`the_bounds_refuse_a_check`).
  - *The encoder and the solver's setup asked no bound*: 3 million
    conjuncts ran 1.7 s before the first poll, and 8 million unit
    hypotheses under a 400 MB bound held about 1 GB before the refusal,
    since the solver copies the clauses and `add_clause` only charges.
    Now the encoder asks the stop every `POLL` formulas and the setup
    asks the solver's `poll` (stop, work, memory) every `POLL` clauses
    (`the_encoder_asks_the_stop`). The checker's charge per clause was
    2.4 times short of the memory it held: it is now `CLAUSE_BYTES`
    (224) beside 8 bytes a literal, and the encoder's 16 bytes more a
    clause.
  - Noted, not changed: the solver, the check and the embedding each
    have the whole `limits.work`, so the method may do three times the
    bound; the docs of `sat::decide` now say so. The cells bound of a
    one-world model is not driven by a classical test (the same code as
    the intuitionistic bound, which is). The check's charge of its
    per-literal vectors saturates, so on a 32-bit target it refuses
    before allocating, never wraps.
- **Counterexamples: none.** Against a RUP reference of its own
  (multiset of sorted clauses, naive propagation to a fixpoint, the same
  deletion rules), a brute-force SAT check up to 22 variables and its own
  truth table, at `vukwwvlw` and again at `wlvqmtux`:
  - 2 820 000 random formulas with random proofs (empty, repeated and
    tautological clauses, deletions of kept and absent clauses, variables
    out of range): the two checkers agree on every answer and step, and
    every accepted proof's formula is unsatisfiable;
  - the solver's own proofs of 9 010 random formulas and pigeonhole
    instances, and 113 803 mutants of them (a step dropped, a literal
    flipped, a needed clause deleted, a unit deleted, a formula clause
    dropped, a clause strengthened, …): no disagreement;
  - long proofs: 175 random 3-SAT refutations of up to 200 variables
    with 1 085 377 steps, 503 901 of them deletions, the longest 74 997
    steps, with mutants: no disagreement;
  - 37 000 random classical sequents (up to 10 atoms, depth 6, every
    connective, 0 to 4 formulas a side, repeats): every falsifying
    assignment extended by evaluation satisfies every clause of `encode`,
    every "valid" is valid by the truth table, `refutes` accepts a
    one-world model exactly when it falsifies the sequent; about 10 400
    through the public `sat::decide` at 2 000 000 units of work, each
    agreeing with the truth table, the rest a refusal or an unknown,
    never `ReadBack` or `Countermodel`.

The fresh-context reviewer (`fresh-reviewer`, Fable 5.2 at `medium`) on
the chain up to `wlvqmtux`, against the brief, the shared rules and
item 2 of the prompt: one finding, reproduced, and two questions.
- *A bound of the embedding other than the stop, the work and the
  memory was reported as a derivation past its size bound*:
  `by_embedding` mapped a recursion, copy, index, witness or instance
  bound onto `Refusal::Output` with an estimate of 0, so `|- ((a -> b) ->
  a) -> a` at a recursion depth of 1 gave "the derivation is estimated
  at 0 bytes, more than the limit of 67108864", naming
  `limits.derivation_bytes`. It holds. Fixed (`ntquonzl`): where the
  embedding's search stops, the outcome is its unknown with its own
  reason, whichever bound it is (`the_embedding_s_bound_is_its_unknown`);
  the rules, the docs and this report say so.
- *The ILTP test could not tell a refused check from a missing
  derivation* (a question): it counted every refusal as "proved by the
  clausal proof alone". Now a missing derivation is an unknown with
  `proof_steps` above zero and a refused check an error of its own, and
  the test counts them apart; the numbers above are from the run after
  the fix.
- *`drup::check` trusts its caller to keep the variables below
  `MOST_VARIABLES`* (a question): the encoder refuses past it first, so
  no code reaches it today; step 70's reader of outside proofs must
  refuse a larger header itself (said in "What the next sessions must
  know").

### The checks

- `cargo clippy --workspace --all-targets -- --deny warnings` and
  `cargo test --workspace` (every suite passed) during the work; the
  `sat` library alone and with `serialize`, without default features,
  free of warnings.
- The gate (`systemd-run … taskset -c 10-12 nix develop -c gate`):
  "gate: passed" at `wlvqmtux`, the chain before the reviewer's fix.
- After the reviewer's fix and the rebase onto `main@origin`
  (`tuknpoln`), the gate and `nix flake check` (under the machine lock,
  `--max-jobs 4 --cores 8`) run at this report's commit, the chain's
  last; their results are in the session's final message to the
  planning session, since a report cannot record the check of its own
  commit.
- The outside checker and the ILTP agreement as above, the latter at
  `ntquonzl` (the chain's last code).

### What the next sessions must know

- **For 54s-c (the measurement and the default).** `Method::Sat` now
  decides classical logic (`Method::decides` says so), so `--method sat`
  covers all three logics once wired. A classical "valid" costs the
  solver, the check and then the embedding's whole search for the
  derivation, so on valid problems the method cannot beat the embedding
  until the author answers the verdict question above (where the
  embedding stops, the method answers unknown with the embedding's
  reason and `proof_steps` above zero); on "not valid" it
  answers in one solver call with a checked assignment. The harness's
  ordinary child still answers `sat` with "does not run"; its
  `Method::decides` guard (`bench/src/ordinary.rs`) now lets classical
  logic through to that answer. `sat::Statistics` gained `clauses` and
  `proof_steps` (zero outside classical logic), written in its JSON.
  Pigeonhole problems (ILTP's SYJ202) are exponential for resolution:
  the solver stops at the work bound from about twelve holes, where the
  embedding fails too.
- **For step 70 (outside proofs).** The checker is
  `ordinary::sat::drup::check(variables, formula, proof, limits, account,
  stop)`, crate-private, over `solver::Lit` and `drup::Step`; it returns
  a `Rejection` (a `Flaw` or a `Refusal`). For outside proofs it needs: a
  DIMACS and DRAT reader (the test-only `drup::dimacs` writes both), an
  error of kind `invalid` for a flaw (today's caller maps a flaw to
  `Error::ReadBack`, a defect, since the stream is the engine's own), and
  RAT steps if the solvers it reads use them (CaDiCaL's and MiniSat's
  proofs are RUP only). Variables past the formula's are a flaw today;
  extended resolution would need them. The checker trusts its caller to
  keep the formula's variables below `MOST_VARIABLES` (its literal codes
  are `u32`); a DIMACS header above that must be refused by the reader. LRAT hints would make each step
  check its hints instead of propagating everything; the checker's
  from-scratch propagation per step costs the unit clauses' closure each
  time.
- **batsat's stream** is complete for RUP but not literal-exact:
  `remove_satisfied` trims level-0-false literals in place without a
  log, so drat-trim warns on deletions of trimmed clauses. A fork that
  logged the trimmed clause as an addition and the original as a
  deletion would make the stream exact; nothing needs it now.
- **The rules** (`.claude/rules/core-ordinary.md`, "Classical logic")
  hold the encoder's soundness argument, the checker's invariants
  (including the watch invariant soundness rests on) and what batsat
  does not log.

## Session 54s-c1: intuitR and intuit

Opus 5.5 at `high`, in the jj workspace `../deduxit-wt-step-54s-c1-161790`,
from `plan/54s-sat-engines.md` (item 3, the bullet "intuitR and intuit
beside deduxit", up to the timed run) and the brief `start-54s-c1.md`, on
2026-11-14, cores 7 to 9.

### Outcome

- Both provers are built from their sources at pinned commits
  (`modules/_provers/intuit.nix`, `intuitr.nix`, the packages
  `prover-intuit` and `prover-intuitr`, in `.#comparison` and its
  `tools.json` as `intuit` and `intuitr`), run as programs, never linked.
- `run --tools` now takes TPTP problems: each runs in every logic of
  `--logics` with the tools that read TPTP, deduxit's two columns in all
  three logics, intuit and intuitR in intuitionistic logic. Every tool
  reads the ILTP file as it is; no translator was needed.
- The smoke run: 15 ILTP problems, four tools, 60 runs, every verdict
  equal to the file's `Status (intuit.)` (below). No timed run.
- `bench/TOOLS.md` has a section of its own for the two provers beside
  deduxit's ordinary logic, `.claude/rules/bench.md` and `flake.md` the
  rest.

### The sources and pins

| | intuit | intuitR |
|---|---|---|
| repository | github.com/koengit/intuit | github.com/cfiorentini/intuitR |
| commit | `dd965ce7c5ebfa22983bba98ced77be7fc1f0aa5` (2020-09-21, the last of 47) | `0cbf0fd078567e12808ee006015d15a3deacab63` (2026-10-27, "small changes"; six commits since 2026-10-21 fixed its build: imports, the cabal file, `->` and `<->` in its syntax) |
| hash | `sha256-gf+5DG7oSKNyeFSfGFh1xVvW4lMUHX492G7JYzGbeWo=` | `sha256-5oTZAwfnasDcp6b0GjqMDsnGdL/9IcpxGpBJX2kSgk4=` |
| licence | none stated: `LICENSE` is an empty file, `intuit.cabal`'s `license` field is commented out | BSD-3-Clause (`LICENSE`, Camillo Fiorentini 2021; `intuitR.cabal` says BSD3) |
| build | `haskellPackages.mkDerivation` with GHC 9.10.3 and `minisat` 0.1.4; the pragma `MonadFailDesugaring` removed from `ParseProblem.hs` and `Parsek.hs` (GHC 9.10 refuses the name; the behaviour is the only one since 8.8) | the same, with `partial-order` 0.2.0.0 `markUnbroken` and `dontCheck` (nixpkgs marks it broken for a failed build of its tests, hydra 2025-09-19); `intuitT/`, the traced variant in the same repository, is not built |
| closure | 72.8 MiB | 73.8 MiB |

The Haskell package `minisat` (BSD-3-Clause) bundles MiniSat; the
research note's open point (which MiniSat version) was not looked into.
Both derivations are written out, not `callCabal2nix`, so the flake has
no import from a derivation.

**The licence question.** intuit states no licence, like the Maude
prover and APLL before it. The brief's stop is a licence that forbids
putting the built program in the public Cachix cache; no build of a
prover reaches it: `nix flake check` builds none of them (`.#comparison`
is no check), CI pushes only what the checks build, and the Comparison
workflow has no Cachix step. So no message was sent, and nothing of
intuit is published. Should a check ever build `.#comparison`, intuit's
missing licence is the question to ask the author first.

### How each reads ILTP's problems

ILTP's 274 problems use the roles `axiom` (2 093) and `conjecture` (274,
one per file) and the connectives `~ & | => <=>`, with one `$true`
(SYN915+1) and one `$false` (SYN916+1); no `include`, no `<=`, `<~>`,
`~|` or `~&`. Both tools' own TPTP readers take that much:
- intuit's `ParseProblem.hs`: every TPTP binary connective, `$true`,
  `$false`, roles axiom and conjecture (and more), the formula `(∧
  axioms) ⇒ conjecture`, exactly one conjecture or an error;
- intuitR's `ParserTPTP.hs` (parsec): `fof` with the roles axiom and
  conjecture only, `~ & | => <=> -> <->`, `$true`, `$false`, `&` and `|`
  at one level of precedence (ILTP parenthesises every mix, as TPTP
  requires).

So each gets the file as it is (`one_ordinary`), and the harness's
translators are not used. Both print `+++ RESULT: Valid
(intuitionistically)` or `+++ RESULT: CounterSatisfiable
(intuitionistically)` and exit 0 either way. A parse error prints no
result line: intuit writes `*** PARSE ERROR` and its parser's
`Unexpected`/`Expected` lines and exits 0, intuitR `user error (parse
error)` and exits 1 (both tried on `p & & q`). So the reader takes the
result line whatever the status, and anything else is an `error` row
named by the last line.
Neither has a time limit or a memory limit of its own: runexec ends
them about two seconds past the limit (`unknown`/`timeout`) or at the
cgroup's memory (`unknown`/`out_of_memory`). intuitR runs with its
default, weak clausification (`-w`); its `-s`, strong, is not a
configuration of the harness.

deduxit's columns call `prove --file F --input-format tptp --logic L
--timeout T --memory-limit M --format json --quiet` (and
`--deterministic` for the one-thread column). The exit status is the
verdict (0 valid, 1 not valid, 3 unknown, 2 an error); the JSON gives
the `engine` (`ljt`, or the embedding's linear engine) and an unknown's
`reason.kind` (`stopped` written `timeout`), the same reader as for
linear problems less the linear refutation's check. A proof's `checked`
is `ok`, since the command checks every derivation it reports; a "not
valid" has nothing the harness checks today.

The expected verdict is the child's (`bench/src/ordinary.rs`'s
`expected`, made `pub(crate)`): ILTP's intuitionistic status, a Theorem
also classical, a Non-Theorem also minimal.

### The smoke run

`deduxit-bench run --tools deduxit,deduxit-default,intuit,intuitr` on the
12 smallest files by size (SYJ101+1 to SYJ104+1, SYJ105+1.002 to .004,
SYJ206+1.001 and .002, SYJ212+1.001 to .003) and SYN915+1, SYN916+1
(`$true`, `$false`) and SYN001+1 (a Non-Theorem with negations),
`--logics intuitionistic`, 10 s, the debug build of the harness and CLI
with the built provers and BenchExec 3.35's runexec on PATH, one core
(7):

| tool | proved | unprovable | against the status | other |
|---|---|---|---|---|
| deduxit | 10 | 5 | 0 | 0 |
| deduxit-default | 10 | 5 | 0 | 0 |
| intuit | 10 | 5 | 0 | 0 |
| intuitr | 10 | 5 | 0 | 0 |

10 Theorems and 5 Non-Theorems, every row as the file says; each run
took 7 to 12 ms of wall time (process start included). `ranking` reads
the rows as the group "ILTP (ordinary logic)". SYJ212+1.001 in
`--logics classical,minimal`: deduxit proved it classically and refuted
it in minimal logic, intuit and intuitR `refused`/`fragment`.
The same 15 problems again with the release build, `nix build
.#comparison`'s `bin` on PATH and its `tools.json` as
`DEDUXIT_COMPARISON_TOOLS` (as `comparison.sh` runs), `--tptp
bench/iltp/Problems --only …`, 10 s, core 7: the same 60 verdicts, each
equal to the file's status, 4.6 to 7.7 ms of wall time per run, and the
rows' `tool_version` from the record (`2020-09-21`, `2026-10-27`, deduxit
`0.3.0`).

### Decisions, each with its reason

- **The file as it is, not a translation.** Both tools read every
  construct ILTP uses, and their own reader is the configuration their
  authors measured; a translation through deduxit's reader would add a
  step that could differ from the problem.
- **An `Ordinary` part of the driver, not a second table of drivers.**
  deduxit's two columns run both kinds of problem, so one name in
  `--tools` stays one driver; the linear fields of intuit and intuitR are
  stubs that `covers` (`nothing`) never lets run. The other way, a type
  for a run that is linear or ordinary passed to every function pointer,
  would have changed every driver's signature.
- **A table of its own in `bench/TOOLS.md`** (departs from the design:
  the brief says the file's table includes both). That table compares
  provers of linear logic in rows (fragments, modes, proof nets, LLTP
  input) that mean nothing for intuitionistic propositional logic; the
  new section has the rows that do, with deduxit's ordinary logic as
  the third column.
- **No flake check runs the tools**: none existed to extend (the brief:
  "if it runs the tools on the smallest problems"), and adding one would
  build intuit in CI and push it to the cache.
- **`bench/src/ordinary.rs`'s `expected` made `pub(crate)`**, one word in
  54s-b2's file, so that the tools' rows and the child's agree by
  construction. Conflicts there are that line alone.

### The checks

- `nix build .#prover-intuit .#prover-intuitr` under the machine lock:
  both built (after intuit's pragma was removed), and `nix build
  .#comparison`: built, its `tools.json` recording both.
- `cargo clippy -p deduxit-bench --all-targets -- --deny warnings` and
  `cargo test -p deduxit-bench` (cores 7 to 9): clean, the tests pass,
  the new `intuit_s_result_lines_are_read` among them.
- The gate (`systemd-run … taskset -c 7-9 nix develop -c gate`): passed.
  Its cache model named search-qbf-20-2 and search-chain-128 for misses
  moved over 5 %, a note and no failure; this chain changes nothing the
  journeys run (`bench/src/journeys.rs` and the library are untouched).
- The smoke runs above, debug and release.
- `nix flake check` at the report's commit, under the machine lock:
  "all checks passed!" (x86_64-linux).

### The reviewer

`fresh-reviewer` (Fable 5.2 at `medium`) on the three commits against
the brief: the builds pinned, the licences recorded, the verdicts read
right, the harness right for linear and TPTP rows. Nothing blocks. It
checked the pins and the built outputs, every claim of the new
`TOOLS.md` section against the sources (the licences, the line numbers,
the options), that all 274 ILTP files use only `axiom`, one
`conjecture` and operators both readers take, the expected column on
Theorem, Non-Theorem and Unsolved files, a timeout read as
`unknown`/`timeout` for all four tools, a parse error of each tool read
as `error`, `--resume --append` with two more logics (9 rows skipped, 18
run, the counts to 27/27), and a linear row unchanged (`--tools
deduxit,intuit --family partition-no=3`, intuit `refused`). Two minor
findings, both reproduced:

1. "`runexec` ends them a second after the run's" was off by a second:
   at `--timeout 1` intuit's and intuitR's undecided rows took 3.01 s.
   Answered: `TOOLS.md`, the rules and this report now say about two
   seconds past the limit (runexec's wall-time limit is the limit plus
   `GRACE` rounded up, and it ends the process about a second after
   that).
2. A TPTP file that deduxit's reader refuses gets an `error` row for
   every tool, intuit and intuitR included, though they did not run.
   Answered: left as it is. The linear path does the same, the row is
   never a verdict, and every ILTP file loads: step 54's
   `bench/defaults/ljt-iltp.csv` has rows for all 274, and its six
   `error` rows are the embedding's size bound, not the reader.
   Recorded here for 54s-c2.

### What the next sessions must know

- **For 54s-c2 (the timed run).** The command, from a checkout with
  `nix build .#comparison`'s `bin` on PATH and
  `DEDUXIT_COMPARISON_TOOLS` its `share/deduxit-comparison/tools.json`,
  as `bench/comparison.sh` sets them:
  `taskset -c 2 deduxit-bench run --tools deduxit,deduxit-default,intuit,intuitr --tptp bench/iltp/Problems --logics intuitionistic --timeout 2 --output iltp-tools.csv`
  (`--shard 1/2` and `2/2` on cores 2 and 3). Pass a directory, not
  files: a file given alone has an empty `family`. The family is the
  directory relative to the parent of the path given, so `--tptp
  bench/iltp/Problems` names `Problems/SYN` and `Problems/LCL`, while
  a generated ILTP family is its own name (`SYJ212`). `comparison.sh` has no
  ILTP pass; adding one is a stream with `--tptp`.
- intuit and intuitR stop only when runexec ends them: its wall-time
  limit is the limit plus `GRACE` (1 s) rounded up, and it ends the
  process about a second after that (the reviewer measured 3.01 s at
  `--timeout 1`); so their undecided rows' `time_ms` is about 4 s at a
  2 s limit, deduxit's about 2 s. The ranking
  counts both as not solved; a figure draws both at the limit.
- intuit's GHC runtime has no heap limit: a run that grows meets the
  cgroup's 4 GiB and reads `out_of_memory`.
- intuitR prints its own times after the result (`Clausification time`,
  `Prover time`, `Total time`, CPU time from `getCPUTime`), so its `raw`
  column is `Total time: …`. Its own `Benchmarks/` holds the 1 200
  problems of its paper (ILTP's among them) and `timings/` its results at
  600 s.
- deduxit's `checked` on an ordinary row is `ok` for a valid one and
  empty for a not valid one: the harness checks no countermodel. Step
  55's countermodels can give the reader something to check.
- In classical logic deduxit's embedding refuses `SYN007+1.014` with
  `the sequent has more subformula occurrences than the limit of
  50000000` (exit 2, an `error` row), seen by hand while probing the
  command; 54s-b's classical engine may answer it.

## Session 54s-b2: valid by a checked clausal proof

Opus 5.5 at `high`, in the jj workspace `../deduxit-wt-step-54s-b2-53b484`,
from item 2b of `plan/54s-sat-engines.md` and the brief
`start-54s-b2.md` of the planning session, on 2026-11-14, cores 4 to 6.
The author's decision of 2026-11-14 ("Go with your recommendation"): a
classical "valid" may rest on a checked clausal proof alone.

### Outcome

- **`ordinary::Verdict::Valid` holds an `ordinary::Evidence`**:
  `Evidence::Derivation(Box<Derivation>)`, a derivation of LK or LJ that
  `Derivation::check` accepted, as every "valid" was before, or (feature
  `sat`) `Evidence::Clausal(sat::ClausalProof)`, a clausal proof that
  `drup::check` accepted.
- **`ClausalProof` can only come from the check**: its fields are
  private and its one constructor, `ClausalProof::checked`
  (`pub(super)`, in `sat/drup.rs`), runs `drup::check` and builds the
  value only when the check accepts. It holds what was checked (the
  variables, the clauses of the sequent's negation, the proof's steps),
  read through `variables()`, `clauses()` and `steps()`, and writes
  itself for a person ("a checked clausal proof of 3 steps against the
  12 clauses of the sequent's negation").
- **`Method::Sat` in classical logic answers "valid" as soon as the
  check passes**, with the clausal proof as its evidence; it no longer
  runs the embedding.
- **`Evidence::derivation()`** returns the derivation, or the new
  `Error::NoDerivation` (kind `unsupported`, code `no_derivation`) for a
  clausal proof, whose message names the method `embedding`. Every
  reader that needs a derivation goes through it.
- **The JSON form**: an outcome valid by a clausal proof is written at
  the new wire level 10 (`wire::CLAUSAL`, `wire::LEVEL` now 10) with
  `"clausal_proof": {"variables", "clauses", "steps"}` in place of
  `derivation`; every other document keeps its level and its bytes.
- **On ILTP the method now answers what it proves.**
  `sat_agrees_with_the_embedding_in_classical_logic` (`core/tests/iltp.rs`,
  ignored as its neighbours; release, cores 4 to 6, 408 s at
  `spvlxqou`, at 2 000 000 units of work and 2 GiB): 124 verdicts agree
  with the embedding, none differs, and the solver alone decides 132
  more, all valid by a checked clausal proof (the SYJ201 family from
  size 3 and the rest 54s-b listed), so 256 of the 274 problems are
  decided with a checked certificate, where 54s-b's API let the method
  report 124. The 18 left stop at the work bound (the pigeonhole
  problems, as before).
- **A unit test pins a clausal "valid" where the embedding does not
  reach**: `valid_by_the_clausal_proof_where_the_embedding_stops`
  (Peirce's law at a recursion depth of 1, and the pigeonhole sequent of
  six pigeons at 100 000 units of work, each unknown to the embedding
  and valid by its clausal proof, whose `derivation()` is
  `NoDerivation`).

### Decisions, each with its reason

- **The form: a payload of `Valid`, an enum `Evidence`**, as 54s-b
  recommended, rather than a second variant `Verdict::ValidByClauses`.
  "Valid" is one answer whatever shows it, so the many readers that only
  ask "valid or not" (`Valid(_)`: the command's exit status, the
  harness, the agreement tests) keep their arm unchanged, and only the
  readers that need a derivation name the kind of evidence. A second
  variant would have made every such reader add an arm that says the
  same thing. `Evidence` is `#[non_exhaustive]`, since step 70's outside
  proofs or an LRAT proof may add kinds; it joins the marked types of
  `core.md`. `Evidence::Clausal` is behind the feature `sat`, as
  `Search::Sat` is.
- **A private constructor, the check inside it**: the brief asks that
  no caller can build "valid" from an unchecked stream. With
  `ClausalProof::checked` the only way to the value, even code inside
  the crate cannot make one without `drup::check` accepting; the type is
  the witness of the check, as `Derivation` read back and checked is
  for the embedding.
- **It holds the counts, not the proof.** The clauses and the steps are
  dropped after the check, as 54s-b dropped them: no caller reads them
  (the command's line and the JSON need the counts, the web client
  shows what the JSON holds), and an outcome holding a proof of
  millions of steps would hold that memory for as long as the caller
  keeps the verdict. A caller that wants the DRUP text for an outside
  checker (drat-trim) is a later, additive accessor
  (`ClausalProof::write_drup`, with the test-only `drup::dimacs` made
  public), which "an item without a caller waits for one" leaves to its
  first caller.
- **`Method::Sat` no longer tries the embedding**, on no terms. Its
  cost was the embedding's whole search after the solver's answer, so
  on valid sequents the method could never beat the embedding, and 54s-c2
  would have measured the embedding again. A caller that needs the
  derivation has a plain way: `ordinary::decide` with
  `Method::Embedding`, which is also the default of classical logic
  (`Options::default().with_logic(Logic::Classical)`); the refusal
  `Error::NoDerivation` names it. A bounded attempt (the embedding within
  a fraction of the limits) was not taken: it would need a constant no
  measurement backs, and the evidence a call returns would then depend
  on the machine's speed under a time limit.
- **One refusal at the evidence, not one per export** (departs from the
  design). The brief has each export (LaTeX, Typst, SVG, Rocq, a term)
  refuse a clausal proof. The exports take a `&Derivation` by type
  (`Drawable`), so none can be handed a clausal proof at all; the only
  place a verdict becomes a derivation is `Evidence::derivation`, which
  refuses with one error whose message says what the evidence is and
  names the method that gives a derivation. The command's drawing in
  every format, its `--lambda`, the batch's file and the goal state's
  `close` and `hint` all go through it; the test is
  `valid_by_the_clausal_proof_where_the_embedding_stops` (the refusal)
  and the command's `a_clausal_proof_is_named_on_the_verdict_line`.
- **The wire level rises to 10** for an outcome valid by a clausal
  proof alone: a reader of level 9 takes a valid outcome to carry a
  derivation, and the key's presence changes what `valid` means there
  (`wire.rs`'s rule). Nothing reads an ordinary outcome back, so no
  `level-10.txt` file is added (level 9 has none either); the doctest of
  `wire::upgrade` named 10 as an unknown level and now names 11.
- **The command** cannot reach a clausal proof yet (`--method` takes
  `embedding` and `ljt`; the value `sat` is 54s-c2's), but its readers
  take one: `ljt_verdict_line` names the method from the outcome (`by
  LJT` or `by the SAT solver`) and a clausal proof after the verdict,
  `valid (classical logic by the SAT solver): a checked clausal proof of
  …`; a drawing in any format is left out with the library's
  `NoDerivation` message, as a refused certificate is; the batch writes
  no file and adds that note to the entry's line.

### Every reader of a verdict, and what it does now

- **`ordinary::decide`, `ljt::decide`, `sat::decide` (intuitionistic
  and minimal)**: build `Evidence::Derivation` after `check`, as before.
  `sat::by_ljt` takes LJT's derivation by value and answers
  `NoDerivation` for a clausal proof, which LJT never gives.
- **`sat::classical::decide`**: `ClausalProof::checked` in place of
  `drup::check` and `by_embedding` (removed); a flaw is still
  `Error::ReadBack`, a refusal of the check still that refusal.
- **`serialize/ordinary.rs`**: the outcome writes `clausal_proof` at
  level 10, the derivation as before; `ClausalProof` derives its
  `Serialize` (written only).
- **`ordinary::interactive`**: `close` grafts and `hint` reads
  `evidence.derivation()?`; its decisions name `Method::Embedding`, so a
  clausal proof never reaches it, and if one did it would be the error,
  never a graft.
- **`lambda`**: the term is a derivation's (`Derivation::term`); its
  tests match `Evidence::Derivation`. An outcome's term goes through
  `Evidence::derivation` in the command.
- **The exports**: unchanged, since they take a derivation by type; the
  export test matches `Evidence::Derivation`.
- **The command** (`cli/src/ordinary.rs`, `cli/src/batch.rs`): above.
  The exit status of a clausal "valid" is 0, `Valid(_)` as before.
- **The bindings** (`web/`): no code reads an ordinary verdict but
  through the outcome's JSON, and a session's `close` and `hint` decide
  by the embedding. `about`'s `level` is now 10;
  `about_lists_the_suites` pins it, and `close_and_hint_decide_a_goal`
  pins that a classical goal's outcome holds a derivation at level 8.
- **The harness** (`bench/`): its ordinary child counts `Valid(_)` as
  `proved` with `checked` `ok`, whatever the evidence, so a clausal
  "valid" counts once 54s-c2 lets the child run `sat` (it still answers
  "the harness does not run sat"). The journeys decide by the embedding
  and LJT. No change.
- **The ILTP tests** (`core/tests/iltp.rs`): the classical agreement
  test now counts a valid by its clausal proof as decided (its steps
  equal to the statistics'), and prints how many the solver alone
  decides and how many of those are valid.

### The panel and the reviewer

`panel-argument` (Fable 5.2 at `high`, a fresh context) on `spvlxqou`,
the one invariant soundness rests on: "valid" is built only from a
derivation `Derivation::check` accepted or a clausal proof `drup::check`
accepted. **The argument holds.** It stated the invariant from the code
first (a `ClausalProof` arises only as the `Ok` of `checked`; the
clauses checked are `encode`'s for the sequent decided, unmodified; each
of the four non-test constructions of `Verdict::Valid`, in `decide.rs`,
`ljt.rs`, `sat/mod.rs` and `sat/classical.rs`, follows its check; no
reader turns a clausal proof into a derivation), then checked every
site: the private fields and the one struct literal, the derives (no
`Default`, `Deserialize` or `From`), `unsafe_code` forbidden, the
outcome written only, no constructor of a verdict in the command, the
bindings, the harness, `compat` or `fuzz`, and every reader through
`Evidence::derivation`. One difference from the rules' argument, not a
fault: the type proves that a check ran, and the single call site
proves that it ran on this sequent's clauses. `ClausalProof` is `Copy`
and `Evidence::Clausal` a public variant, so a caller of the library
could put one sequent's proof into another sequent's verdict, as it can
already wrap any derivation in `Evidence::Derivation`; no code of the
workspace does. The rules now say so (below). The brief names the
argument member alone; the counterexample and integer members were not
run, since the change adds no arithmetic and no search, and the checker
and the encoder they would test are 54s-b's, which its panel tested.

The fresh-context reviewer (`fresh-reviewer`, Fable 5.2 at `medium`) on
`spvlxqou` and `opqwwqkr`, against the brief, the shared rules and item
2b of the prompt: **no finding**. It checked the four constructions of
`Verdict::Valid`, read `drup::check` again as the one guard of a
classical "valid", every reader, the wire level's documentation, the
tests the brief names, the error family's tables and the features
(`sat` on and off), and judged the one refusal at `Evidence::derivation`
to leave no requirement unmet. Two observations, neither of this chain:
- `cargo check -p deduxit --no-default-features --features parse
  --all-targets` fails to compile the library's tests, with or without
  `sat`: `core/src/linear/nets/cut/tests.rs:8` imports
  `proofs::Interactive` without the feature `interactive`. Not this
  chain's file; a follow-up for the planning session (the `cargo hack`
  runs check the library, not its tests, so the gate does not see it).
- The command's left-out line for a clausal proof and the batch's "no
  file is written" note have no test that runs them: they are
  unreachable until 54s-c2 wires `--method sat`, which should cover them
  then (below).

### The checks

- `cargo clippy --workspace --all-targets -- --deny warnings` clean, and
  `cargo clippy -p deduxit --all-targets` clean without default
  features, with `sat` alone, `sat,serialize` and every default but
  `sat`; `RUSTFLAGS="-W unreachable_pub -W unnameable_types" cargo
  check -p deduxit --all-features` clean.
- `cargo test --workspace --no-fail-fast`: every suite passed (27
  results), README's, the manual's and the guide's examples among them.
  A first run failed one doctest, `wire::upgrade`'s, which used level 10
  as the level past `LEVEL`; it now uses 11.
- The bindings' tests under Node (`nix build
  .#checks.x86_64-linux.web-wasm-tests` under the machine lock): 29 and
  6 passed.
- `cargo run --release -p deduxit-bench -- run --all-families --timeout
  5`: 195 runs, `summary --strict` passes, no verdict disagrees, no
  mismatch.
- The ILTP agreement, above (408 s, passed).
- The gate (`systemd-run … taskset -c 4-6 nix develop -c gate`): "gate:
  passed" at `opqwwqkr`; the ratchet named moved cache misses on three
  linear journeys this chain does not touch (a layout's, never a
  failure).
- The behaviour lock is unchanged: the command prints nothing new for
  any input it accepts today.
- `nix flake check` at the chain's last commit, after the rebase, under
  the machine lock: its result is in the session's final message, since
  a report cannot record the check of its own commit.

### Departures from the design

- One refusal at the evidence (`Evidence::derivation`,
  `Error::NoDerivation`) rather than one per export (departs from the
  design); the reason is under "Decisions".
- `panel-argument` alone, as the brief names it; the counterexample and
  integer members were not run (the brief does not ask for them, and the
  change adds no search and no arithmetic).

### What the next sessions must know

- **For 54s-c2 (the measurement, the default, `--method sat`).**
  - `Method::Sat` in classical logic now costs the encoder, one solver
    call and the check, and answers "valid" with `Evidence::Clausal`;
    the embedding is not run. 54s-b's ILTP numbers (256 of 274 decided
    with a checked certificate at 2 000 000 units of work) are now what
    the method answers, not 124.
  - The command's `by_ljt` and the batch's `Shared::by_ljt` call
    `ljt::decide` directly. Wiring `--method sat` means calling
    `ordinary::sat::decide` (or `ordinary::decide`) there; the readers
    after the call already take a clausal proof (`ljt_verdict_line`
    names "the SAT solver" from `outcome.method()`, the drawing and
    `--lambda` go through `Evidence::derivation`, the batch writes no
    file). The function names still say LJT; renaming them is yours.
    `ljt_statistics` prints LJT's counters only, and `--net`'s refusal
    names LJT.
  - A settings file's `ordinary.method: "sat"` is read today (the
    library's `Method` reads `sat`), and `prove` then takes the image
    path, since it dispatches on `Method::Ljt` alone: the embedding
    decides. Refuse it or wire it.
  - The harness's ordinary child counts `Valid(_)` as proved; its
    `sat` arm answers "the harness does not run sat" until you add it.
- **For step 70 (outside proofs).** `ClausalProof::checked` is the
  pattern: an outside DRUP or LRAT proof becomes evidence only through a
  constructor that runs the check. Its error for a flaw must be of kind
  `invalid`, not the `ReadBack` defect the engine's own stream gets.
  `Evidence` is `#[non_exhaustive]`, so an `Evidence::Lrat` or a
  `ClausalProof` that keeps its steps (to write them as DRUP for
  drat-trim) is additive.
- **For the web client.** Once its pin moves past this chain, `about`
  reports `level` 10. No call of the bindings answers a clausal proof
  today (`ordinary_close` and `ordinary_hint` decide by the embedding);
  an outcome of `ordinary::decide` by the SAT solver, if a later call
  offers one, has `"engine": "sat"`, no `derivation` and
  `"clausal_proof": {"variables", "clauses", "steps"}`, which the page
  shows as text (`ClausalProof`'s `Display` is the command's wording)
  and cannot draw.
