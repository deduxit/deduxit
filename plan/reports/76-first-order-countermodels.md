# Step 76: countermodels at first order

## Session 76a: the design and the classical models

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58a-610ed3`,
on 75d's head (`ovnllzuu`), cores 7 to 9; the panel on 13 to 15.
Nothing was pushed.

### Outcome

- **The design** is `plan/notes/first-order-countermodels-design.md`:
  what a user gets (§1), the type (§2: `Structure` with function tables
  now, Kripke models with growing domains for 76b), the classical search
  (§3: the encoding against MACE's and Paradox's, symmetry breaking,
  sizes and bounds, the answer), the dispatch (§4), the drawing (§5),
  76b's Kripke models and their check (§6, §7), the Rocq side for 76c
  (§8), and what each session builds (§9). Each choice names its
  sources and the alternatives weighed.
- **Finite structures interpret function symbols**
  (`Structure::functions`, `with_functions`): tables checked up front,
  evaluated by a stack of their own, printed, JSON key `functions` at
  wire level 26 (`wire::MODELS`), left out where empty so grounding's
  documents stay at 25.
- **The classical model search** (`ordinary::models`, `Method::Models`,
  `--method models`, `--model-size N|none`, `search::Options::model_size`,
  `Reason::ModelSize`): for 1, 2, … elements up to
  the bound (library 6, command none), the sequent's falsity in a
  structure of that size as a propositional sequent for the SAT solver
  (`sat::falsify`, no clausal proof), the structure read off the model
  and checked by `FiniteCountermodel::check` before "not valid", in
  every logic; never "valid".
- **In the dispatch**: the library's default asks the model search last, after the
  embedding, on a first-order sequent left unknown, in every logic
  (departs from the design, below); the command races it beside the
  connection prover on a thread of its own in classical logic (on one
  thread: the prover's 70% of the time limit, then the model search to
  the first-order share), the design's alternative chosen by the
  measurement.

### Decisions and sources

The design note holds them with their sources; in brief:
- **Both quantifiers expanded over the domain, no Skolem function**
  (§3.1): `∀x. A` is the conjunction of `A` at every element and `∃x.
  A` the disjunction, which is the semantics of the quantifiers in a
  structure of `n` elements, so nothing has to stay consistent across
  occurrences. MACE2 and Paradox Skolemize and flatten clauses because
  their solvers take clauses; deduxit's encoder names subformulas
  itself. A Skolem symbol per variable was 75d's unsound grounding
  (SYN316+1 "valid"), and per occurrence it costs what the expansion
  costs. Sources: McCune, MACE 2.0 (arXiv cs/0106042) §5 on
  flattening's blow-up; Mace4 (arXiv cs/0310055) §1; Reger, Suda and
  Voronkov, SAT 2016 (arXiv 1604.08040) §3, which describes Paradox
  (Claessen and Sörensson's paper itself was not reachable: its
  addresses redirect or refuse).
- **Function symbols as value atoms** with Paradox's totality and
  functionality (RSV16 §3.2): `f(d̄) ↦ e` an atom per principal term and
  value, exactly one value per principal term; a term denotes `e` where
  its arguments denote some tuple whose image is `e`, memoised per
  application and the values of its free variables, so a term costs its
  table once.
- **Symmetry breaking on the constants** (RSV16 §3.4 after Paradox):
  the i-th constant takes one of the first i + 1 elements, and m > 0
  only where an earlier one takes m − 1. Principal terms of functions in
  the second clause and Mace4's least-number heuristic are left out
  until measured. The structure read off is checked whatever the
  clauses say, so a wrong clause loses models (unknown), never makes a
  wrong "not valid".
- **Each size afresh**, not Paradox's incremental solver (§3.3): the
  last size dominates, and building afresh keeps each size's statistics
  and memory account its own. No clausal proof is recorded for a size
  without a model (`sat::falsify`): it proves nothing.
- **The answer**: the structure is checked by
  `FiniteCountermodel::check` before it is one (failing it is a defect,
  `Error::Countermodel`); "not valid" holds in every logic (a classical
  structure is a Kripke model of one world); the search never answers
  "valid".

### The code

- `pnnpvwxv` "Interpret function symbols in finite structures":
  `Structure::functions` and `with_functions`, the up-front checks (a
  table of exactly `size^arity` values, values in the domain, a table
  for every function symbol the sequent applies), the evaluator's term
  walk on a stack of its own, the memory charge, `Display`, wire level
  26 (`wire::MODELS`) where a structure has a table, the schema.
- `nsrqyzqz` "Search for finite structures that refute a first-order
  sequent": `search::Options::model_size` (`DEFAULT_MODEL_SIZE` 6,
  `None` in `Settings::default()`), `Reason::ModelSize` (last);
  grounding's normal form with function applications
  (`normal_form(…, functions)`, `Term::App`, `Nnf::apps`; grounding
  still passes `false`); `ordinary::models` (the expansion, the value
  atoms, totality, functionality, symmetry, the read-off, the check);
  `sat::falsify`; `Method::Models`, `Search::Models`, the outcome's
  JSON at level 26, the protocol's method list; the library default's
  model search after the embedding; tests (hand cases with function
  symbols at their least sizes, an infinite-model-only sequent, the
  stop, the default, agreement with the connection prover on 300
  generated sequents with function symbols).
- `zvwsmssm` "Run the model search in the command, beside the connection
  prover, and document it": `--method models`, `--model-size`,
  `by_connection` (the race with half the memory bound each, or the
  sequential split on one thread), the verdict and advice lines,
  `--stats`, `check`'s naming; the manual (a "Finite models" section with
  its examples, the JSON, the options), the guide, the changelog, the
  status chapter, the schema check's documents.
- `omrwynkl` "Lock the model search's answers and the help that names
  it": four lock calls and the help of `prove` and `interact`.
- `mwrnqwrp` "Run the model search in the harness and measure it on
  ILTP": `--engines models`, the rows under `bench/defaults/`.
- `ppwmtklp` "Pin the model search's outcomes at wire level 26":
  `core/tests/wire/level-26.txt` and its stability test.
- `kxwltnzl` "Bound the structure's text, its terms' evaluation and the
  normal form's term walk": the panel's findings (below).
- `ulvxmrqs` "Make room in the web module's budget for the model
  search": the bindings' module at 2 661 758 bytes at gzip, the ceiling
  2 700 000.
- `pozpkqpl` "Give the model search a grace after the prover gives way,
  and race it as the rules say": the reviewer's findings (below).

### Departures

- **The library's default asks the model search after the embedding,
  not before it** (departs from the design, §4, which put it after the
  connection prover and before the embedding). Measured: before the
  embedding it lost SYJ018+1, which the embedding proves in 50 ms, since
  the model search ran on that theorem until the harness's stop. After
  the embedding it only replaces an unknown with a checked refutation.
  The library's intuitionistic and minimal default asks it too (the
  design left those logics to 76b): a classical structure refutes there
  as well.
- **The command races, it does not share sequentially**: the design
  named both and left the choice to the measurement with "no valid
  answer slower" as the condition. A sequential split at the default
  2 s limit loses theorems the prover proves after 1.4 s (GEJ011+2,
  GEJ012+2) or refutations; the race keeps both, at the cost of a second
  thread and half the memory bound for each search. On one thread
  (`--jobs 1`, `--deterministic`) the split is 70% for the prover, then
  the model search to the first-order share of 90%. Where the prover
  gives way at a bound, the model search goes on for 5% of the time limit
  (`MODEL_GRACE`), then the embedding takes over (the reviewer's
  finding, below).
- **The text of a structure shows at most 64 entries of a table or
  tuples of a predicate**, the rest counted (an addition the panel's
  findings asked for; grounding's structures print the same way).

### Measurements

ILTP's 2 670 first-order problems at 2 s, classical logic, three shards
on cores 7 to 9 (the final search; the 1 730 with equality are refused,
four do not read), `bench/defaults/models-iltp-first-order-{1,2,3}.csv`
(`--engines models,auto`) and `connection-iltp-first-order-{1,2,3}.csv`
(`--engines connection,embedding`, for the shares):

| method | not valid | of them CounterSatisfiable | slowest |
|---|--:|--:|--:|
| model search | **210** | 176 of 178 | 0.40 s |
| grounding (75d) | 77 | – | – |
| nanoCoP 2.0 (comparison of 75d) | 84 | – | – |
| leanCoP 2.1 (the same) | 26 | – | – |

- The model search's 210 hold every one of grounding's 77 and of
  nanoCoP's and leanCoP's refutations; the other 34 are 29 problems
  marked Satisfiable (no conjecture: the sequent is `Γ ⊢`, and the
  structure satisfies the axioms) and 5 without a status. The two CounterSatisfiable problems
  it misses, SYN330+1 and SYN335+1, end at the limit. No verdict
  contradicts a status: none of the 210 is marked Theorem or
  Unsatisfiable. Every structure was checked inside the timed run.
- Median 0.7 ms, 90% within 13 ms, 99% within 177 ms.
- 72 problems ended at `model_size` without a bound: sequents without
  quantifiers and function symbols, where the measured build stopped at
  the constants' number of elements; the panel found that answer below
  the bound misleading, and the final build tries every size up to the
  bound there too (the 210 refutations are unchanged: a refuting
  structure there is found at the constants' number or below).
- **The command's shares**, simulated from the per-problem times (the
  prover's, the model search's and the embedding's) at 2 s: the
  prover's 90% and the embedding's 10% prove 604 and refute 1; the
  prover at 30%, 50%, 70% or 80%, the model search to 90%, prove 601,
  602, 602 or 603 and refute 210, 210, 210 or 208. At 5 s every split
  from 40% on proves 604 and refutes 210. The race keeps the prover's
  90%: 604 proved, and the model search's 210 found within 0.4 s.
- **The library's default** (`auto`, no clock, the harness's 2 s stop)
  with the model search before the embedding proved 673 and refuted 97
  (75d: 674 and 77): it lost SYJ018+1 to the model search, which ran on
  it until the stop, and gained SYJ208+1.010 (75d's 1.98 s); hence the
  model search after the embedding. With the model search after the
  embedding (the final build, `bench/defaults/models-default-iltp-first-order-{1,2,3}.csv`,
  classical and intuitionistic, `--engines auto`; the machine was loaded
  by other sessions' builds, load average up to 16, so its times are
  disturbed and its verdicts are what to read):

  | logic | proved | not valid | of them by the model search | 75d: proved, not valid |
  |---|--:|--:|--:|--:|
  | classical | 673 | 89 | 13 | 674, 77 |
  | intuitionistic | 224 | 226 | 118 | 224, 109 |

  No verdict contradicts a status. SYJ036+1 (proved by grounding in
  2.2 s, late in 75d's run too) and SYN539+1 (refuted by grounding in
  0.9 s on a quiet core, 1.8 s to 2 s under that load on either build)
  ended at the limit. Classically the library's prover and embedding
  take most of the 2 s before the model search starts, so the command's
  race is where the 210 show; intuitionistically the embedding gives up
  sooner and the model search adds 117 refutations. The `auto` rows of
  `models-iltp-first-order-*.csv` are the intermediate order (the model
  search before the embedding), kept as the measurement that moved it.

### The panel

Argument (Fable 5.2), counterexamples (Opus 5.5), integers (Sonnet
5.5), all at `high`, on cores 13 to 15, on the head at `ppwmtklp`.
None found a wrong "not valid": the argument member checked every path
that answers it (the search, the library's default after the embedding,
the command's race and its sequential split, the batch, `check` and the
protocol) and compared 36 000 (sequent, structure) pairs with an
evaluator of its own; the counterexamples member compared 8 350
sequents with a brute-force enumerator of its own (the size found always
the least, no structure missed within the bound, every structure
refuting by its evaluator, 2 053 mutated structures judged alike by
`check`), and ran the defaults, the connection prover and grounding on
thousands more with no contradiction. Their findings were bounds, all
answered in `kxwltnzl`:
- **The text of a large function table** (counterexamples and
  integers): `Display` built a string per entry, some fifty times the
  bytes the search charged, unbounded and with no stop; a sequent of
  189 bytes was OOM-killed at 8 GiB under the default 2 s limit, and
  `check` of the command's own JSON likewise. Now the first 64 entries
  are shown and the rest counted (`SHOWN`); `big26` prints in 0.14 s at
  270 MB.
- **A table past the allocator under `--memory-limit none`**
  (counterexamples and integers): an abort or a panic (`capacity
  overflow`). Now `try_reserve_exact` and a refusal at `isize::MAX / 4`
  cells: unknown at the memory bound.
- **The evaluator's terms** (argument): a JSON sequent sharing its
  subterms (`t(k+1) = g(tk, tk)`) was evaluated as its tree with no
  work counted and no stop asked: 26.5 s under a work bound of 1 000,
  `check` with `--timeout 1s` after 10 s. Each application is now an
  evaluation (`tick`): refused at once.
- **The normal form's term walk** (argument and integers): run again
  for every entry of a literal (nested equivalences enter one literal
  2^depth times: 29.7 s before the first stop), uncharged, and its
  applications' free-variable lists quadratic in a nest of quantifiers
  (806 MB under `--memory-limit 64MiB`); the command's race also waited
  on the model thread there (a theorem the prover proved in 0.1 s came
  after 11 s). Now each term is read once per scope (`Nnf::terms`), the
  lists are charged (`free_entries`, which also bounds grounding's own
  pre-existing overrun on plain nested quantifiers, 1.7 GB under 64 MiB
  before, 85 MB now) and the bound is checked after each application;
  the witnesses answer in 0.01 s to 0.7 s.
- **`Signature::of` quadratic in the function symbols** (integers):
  400 000 symbols overran a 2 s limit by 21 s; a set now (2.35 s, the
  rest the input's size).
- **`ModelSize` below the bound** (argument): the cutoff at the
  constants' number answered "no structure of up to 2 elements" with
  `--model-size 6` and advised raising it; dropped, the search tries
  every size up to the bound.
- **`canonical`'s keys** (argument, not reachable below 90 GB): the
  multiplier 3 under `saturating_mul` moved the saturation point; the
  key now holds a kind and a value per term, which cannot saturate.

### The reviewer's round

`fresh-reviewer` (Fable 5.2, medium) on `yvrnlpvp::kxwltnzl`, during
and after the rebase. Nothing wrong in soundness, the check, the bounds
or the wire forms. Its findings, answered in `pozpkqpl` and by folding
the documentation into `zvwsmssm`:
- **A "valid" waited 90% of the time limit where the prover gave way at
  once** (high): the race stopped the model search only on a verdict,
  and the one-thread path started it unbounded after a give-way, so the
  embedding's theorems waited (`--path-length 1`, a theorem the
  embedding proves in 0.3 s: 18 s at `--timeout 20`). First answer:
  stop the model search when the prover gives way; that lost the model
  search's main case, since on non-theorems the prover often ends at
  once with `incomplete` (`forall x. (p(x) -> p(f(x))), p(a) |- p(b)`
  went back to unknown). Measured instead: of the model search's 210
  structures, 30 come where the prover gave way first, every one within
  2% of the limit after; the one theorem the embedding proves after a
  give-way (SYJ018+1) took it 2 ms. So the model search goes on for
  `MODEL_GRACE`, 5% of the limit (0.1 s at 2 s, 0.1 s without a limit),
  after the prover gives way: the witness answers in 1.0 s at 20 s, the
  refutation is back.
- **Which of the two answers a sequent both refute varies between runs**
  (medium): the prover's exhaustive refutation and the model search's
  race; the verdict is the same, the method named and the structure
  differ. Answered as documented behaviour, as in the linear search's
  race between its thread and its pool: `--jobs 1` and `--deterministic`
  give one answer (the rules and `by_connection`'s doc say so).
- **The rules described the one-thread path as the dispatch** (low):
  `cli.md` and `core-ordinary.md` now describe the race first.
- **The flag and its documentation were in different commits** (low):
  folded into `zvwsmssm`.
- **A question**: the library's default on `∀x ¬r(x,x), transitivity,
  ∀x ∃y r(x,y) ⊢` ran past 600 s. The connection prover at the library's
  path length of 8 runs past a minute on it (the embedding and the model
  search answer in milliseconds); it predates this step and is open.

### What was checked, and by what

- `cargo clippy --workspace --all-targets --all-features -- --deny
  warnings`: clean at every commit's build here.
- `cargo test --workspace`: every test passes (the manual's and the
  guide's examples among them, the new "Finite models" section's
  included), before and after the panel's fixes.
- The gate (capped, cores 7 to 9): passed on the head before the panel
  (`ordinary-pigeons` −0.01 % against its ceiling: D17's propositional
  path unchanged) and on the rebased head after the panel's fixes
  (`ordinary-pigeons` −0.82 %).
- `nix build` of `checks.manual`, `manual-links` and `wire-schema`
  (113 documents, the model search's three among them, no failure)
  before the panel; the full `nix flake check` on the rebased head
  failed one check, `budget` (the bindings' module at 2 661 758 bytes
  at gzip against 2 650 000), answered in `ulvxmrqs`; then, after the
  reviewer's fixes, `nix flake check --keep-going` on the final head
  passed every check, and the gate passed there too.
- The measurement: the model search and the default on ILTP's
  first-order problems (above), no verdict against a status.
- The panel's witnesses rerun on the fixed release build (the panel
  section).
- Rebased onto `main@origin` (`rtwwqsvz`, fixup-22): conflicts in
  `cli/src/ordinary.rs` and `cli/src/batch.rs` (fixup-21's stop that
  notes the read-back's start, now beside the race's `halt`) and in the
  options tables of the manual and the guide (fixup-21's
  `--derivation-limit` line); the level-26 stability test needs `sat`
  under fixup-22's tests in every pair of features.

### What the next sessions must know

**For 76b (Kripke models with growing domains):**
- The design's §6 and §7 are the plan: worlds at most 4, elements at
  most 4 per world by default, the SAT encoding per (worlds, elements)
  pair, the check the one trust.
- Reuse: `grounding::normal_form(…, functions: true)` gives the formula
  with function applications interned (`Nnf::apps`, every argument
  before its application); `models::Writer`'s value atoms, `over_tuples`
  and `term` show how a term's denotation is written per element, and
  `sat::falsify` is the solver call without a clausal proof. A Kripke
  search writes forcing per world with persistence, so `expand` does not
  carry over as it is.
- The type: extend `Structure`'s ideas to a model per world (domains
  growing along the order, constants and functions' tables shared or
  growing, predicates growing), `FiniteCountermodel`'s check path; the
  wire level after 26.
- The library's default asks the classical model search after the
  embedding in every logic; a Kripke search would come after it in
  intuitionistic and minimal logic. The command's intuitionistic default
  is the embedding alone: a Kripke search there needs a share or a race
  as the classical one has (`cli/src/ordinary.rs`, `by_connection`).

**For 76c (the Rocq side, the drawings):**
- A finite structure with function tables has no certificate yet: the
  design's §8 (`fo_classical_refutation`, the boolean checker reflected)
  covers it; the Rocq writer refuses a first-order refutation today.
- The text form is `Structure`'s `Display` (`f: 0 ↦ 1, 1 ↦ 0`,
  `g: (0, 1) ↦ 0, …`, `f: always v`); SVG, LaTeX and Typst draw no
  structure yet.

**Open:**
- The connection prover at the library's default path length of 8 runs
  past a minute on `∀x ¬r(x,x), ∀x∀y∀z (r(x,y) ∧ r(y,z) → r(x,z)),
  ∀x ∃y r(x,y) ⊢` (the reviewer's question), so the library's default
  does not reach the embedding and the model search there; it predates
  this step.
- The model search's encoding expands a literal over every tuple its
  open arguments can denote (`n^k` disjuncts for `k` arguments that are
  function terms): Paradox's flattening with fresh variables would
  share these; not measured to matter on ILTP (every structure within
  0.4 s), but a sequent with wide terms grows fast with the size.
- No incrementality across sizes, and no symmetry breaking on function
  values (RSV16 §3.4's second clause, Mace4's least-number heuristic):
  measure on harder sets before adding.
