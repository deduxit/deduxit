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

## Session 76b: Kripke models with growing domains

Opus 5.5 at `high`, in the same workspace, on 76a's head (`kpwlyunq`),
cores 7 to 9; the panel on 13 to 15. Nothing was pushed.

### Outcome

- **The type and its check**: `ordinary::KripkeStructure` (worlds with a
  prefix domain each, arrows, function tables per world, predicates,
  `falsity`; constants once; the refuted world), `KripkeWorld`,
  `KripkeCountermodel`, `Outcome::kripke`, the JSON
  `"kripke_structure"` at wire level 27, the text form a line per
  world. The check is the forcing relation's definition on a stack of
  its own, with the growth of domains, predicates and `falsity`, the
  tables' agreement and the acyclic order checked apart.
- **The bounded search**: in intuitionistic and minimal logic
  `models::decide` tries, for L = 1, 2, …, the classical structures of L
  elements and then the Kripke models of min(L, worlds) worlds with
  domains of min(L, size) elements, each one propositional sequent for
  `sat::falsify` (`kripke_search`); the model read off is checked before
  "not valid"; past both bounds the answer is unknown,
  `Reason::KripkeSize` (`search::Options::model_worlds`, library 4,
  command none; `--model-worlds`).
- **In the dispatch**: the library's default after the embedding (the
  same call as the classical model search's, which now includes the
  Kripke models); the command races the model search beside the
  embedding on a thread of its own, which takes one of the command's
  threads; a proof stops the model search at once (measured, below:
  a long theorem 1 to 3 % slower).
- **Glivenko's failure has no finite countermodel** (design §10): in a
  finite frame every world sees a maximal one, which forces what is
  classically true in its structure, so `¬¬A` is forced everywhere for
  every classically valid `A`. The search answers unknown on
  `¬¬∀x (p(x) ∨ ¬p(x))` and on the double-negation shift, which a test
  pins; the countermodels are infinite chains (world n with domain
  `0 .. n`, `p(k)` forced from world k + 1 on). The planning session was
  told at once, since step 77's item waits for it.

### Measurements

ILTP's 2 670 first-order problems at 2 s, intuitionistic logic, three
shards on cores 7 to 9 (`bench/defaults/kripke-iltp-first-order-{1,2,3}.csv`,
`--engines models,auto`; the 1 730 with equality are refused, four do
not read):

| method | not valid | intuitionistic Non-Theorems refuted | slowest |
|---|--:|--:|--:|
| model search (structures and Kripke models) | **324** | 93 of 94 | 1.97 s |
| nanoCoP-i 2.0 (75d's comparison) | 86 | – | – |
| iLeanCoP 1.2 (the same) | 74 | – | – |
| deduxit's default at 75d | 109 | – | – |

- The 324 hold every one of nanoCoP-i's 86 and iLeanCoP's 74; the rest
  are marked Unsolved (190) or Open (41) intuitionistically. None is
  marked Theorem. Median 0.5 ms, 90% within 19 ms, 99% within 0.5 s.
- **The library's default** (`auto`): 226 proved and 304 refuted (76a:
  224 and 226 with the classical model search alone; 75d: 224 and 109).
  Its refutations: the model search 196, grounding 75, the SAT solver 33
  (sequents without quantifiers). No verdict contradicts a status.
- **The command's race**, on the 530 problems the library's default
  decides, three at a time on cores 7 to 9 (`--timeout 2s`, the default
  threads): all 226 theorems proved, 298 of the 304 refutations; the
  other 6 end at grounding's time limit or its bound on the ground
  formula, which the library without a clock passes (SYN418, SYN420,
  SYN421, SYN544, SYN545, SYN547), as before this step.

### The panel

Argument (Fable 5.2), counterexamples (Opus 5.5), integers (Sonnet
5.5), all at `high`, on cores 13 to 15 beside 77a's and 78a's runs.
None found a wrong "not valid":
- the argument member stated the invariant (a poset, domains growing,
  persistent interpretations, atoms and minimal logic's `⊥` growing,
  forcing by definition, the refuted world forcing Γ and not C, one
  world classically: Kripke soundness for LJ and LM), checked it at every
  site, and ran 620 000 (sequent, model) pairs against a recursive
  evaluator and validator of its own (0 disagreements) and 277 models of
  the search (0 wrong);
- the counterexamples member compared 3 680 sequents with a brute-force
  enumerator of every Kripke model within the bounds (up to 4 worlds,
  the diamond included): the search found a model exactly where one
  exists (1 514), no sequent either the embedding or the connection
  prover proves got a model, the command's race on 300 sequents agreed,
  and `deduxit check` accepted exactly the 2 870 refuting models among
  6 154 forgeries and the 977 among 5 070 random models.

Their findings were bounds and one panic, answered in `zxtlwutr`:
- **`--model-size 0` panicked** (argument and integers): a pair with no
  element indexed an empty domain; in the command's race the panic
  replaced a valid answer by exit 101. Now unknown at once in every
  logic.
- **The protocol's check of `decides` passed over a Kripke model**
  (argument): `Recorded`, a forged one too. Now `KripkeCountermodel::check`
  (`Held` honest, `Failed` forged, a test).
- **Passes that counted no work** (argument and integers): the function
  agreement (a function of 10⁶ arguments ran 16 s past a 1 s stop), the
  closure lists (cubic in a complete order, 4 000 worlds 7.5 s past a 2 s
  limit), a term's arguments in both evaluators. Now each counts its
  steps (an application `1 + arity`) against `limits.work` and the stop.
- **The text and the account** (integers): the model's text listed every
  arrow and was built after a refused check (765 MB at 40 MiB); a world
  was charged 96 bytes where 240 were measured. Now arrows are capped as
  tables are, the text is built only for a valid refutation, and a world
  is charged 256 bytes. The JSON read itself stays outside the account,
  as for every form (`serialize`'s reading is bounded by the document).

### The reviewer's round

`fresh-reviewer` (Fable 5.2, medium) on `zuquzurw::zxtlwutr`. It found no
wrong "not valid", agreed with the Glivenko argument for intuitionistic
logic (it covers the double-negation shift too) and with pinning
"unknown"; its findings:
- **The race slowed a valid answer** (high): the model thread ran on
  top of `--jobs`, a long theorem 12 to 17 % slower (16.4–16.8 s against
  18.4–19.6 s at `--jobs 3` on three cores). Answered in `tmyyoyzo`: the
  race's model search takes one of the command's threads; the same
  witness then took 16.2–16.5 s against 16.0 s for the embedding alone,
  three alternating runs each on cores 7 to 9 (departs from the design's
  condition by that remainder, which a second search costs).
- **"A finite model never refutes ¬¬A for a classically valid A" is
  false in minimal logic** (medium): a maximal world may force `⊥`;
  `|- ~~(false -> p(a))` is refuted minimally. Answered: the manual, the
  guide, the changelog and the rules say it of intuitionistic logic and
  name minimal logic's case.
- **The JSON form came a commit before its documentation** (low):
  answered by folding the command's and the documentation's commit into
  the core's (`kylzmpzn`).
- **The guide's `--stats` list and cli.md's counter count** (low):
  updated.
- **A question**: the search's domains are initial segments of one
  numbering, so two worlds not ordered add the same new elements, and a
  diamond whose middle worlds add different elements below a common top
  is not among the models searched. Answered: `Reason::KripkeSize` now
  reads "the search found no Kripke model of up to …", the reason's doc
  and the manual say what kind of model is searched, and the rules note
  that models over trees, which suffice for completeness, lose nothing.

### Departures

- **The race costs a valid answer 1 to 3 %** on a long theorem
  (departs from the design's "no valid answer slower" by that much,
  measured): the model search shares the machine with the embedding's
  search, which keeps all but one of the command's threads.
- **The batch does not race**: `deduxit prove --batch` leaves
  intuitionistic and minimal first-order sequents to the embedding alone
  (open, below).
- **The library's default asks the Kripke search after the embedding in
  intuitionistic and minimal logic through the same call as the
  classical model search** (as the brief asks); the classical sizes are
  tried first at each level, since a classical structure refutes in
  every logic and costs less.
- **Glivenko's failure is pinned as unknown, not as a countermodel**:
  no finite model exists (design §10); step 77's plan moved to a Rocq
  lemma on an infinite model meanwhile.

### What was checked, and by what

- `cargo clippy --workspace --all-targets --all-features -- --deny
  warnings` and `cargo test --workspace` at every commit's build here.
- The gate: passed on 76b's first head (`ordinary-pigeons` −0.82 %: D17's
  propositional path unchanged), after the panel's fixes, and on the
  rebased head (`ordinary-pigeons` −0.82 %).
- `nix flake check --keep-going` before the reviewer's round failed three
  checks: `budget` (2 708 195 bytes against 2 700 000, answered in
  `nqqvzlvv`) and the two wasm32 Clippy checks (a lint expectation that
  does not fire on 32-bit, answered by a `target_pointer_width` guard);
  on the rebased final head every check passed.
- The measurement and the race's check on ILTP (above); the panel's and
  the reviewer's witnesses rerun on the fixed build.
- Rebased onto `main@origin` (`zuppslkx`, 77a's chain) without
  conflicts.

### What 76c must know

- **The types to certify**: `KripkeStructure` (worlds with prefix
  domains `0 .. size`, arrows to the worlds directly above, tables per
  world over its own domain agreeing along arrows, predicates and
  minimal logic's `falsity` growing, constants once in every domain, the
  refuted world) and `Structure` (`From<Structure>` is the one-world
  case). The check (`KripkeStructure::refutes`) is the definition a
  Rocq checker reflects: the growth and agreement conditions, then
  forcing with `→`, `¬`, `↔`, `∀` over the worlds at or above, `∃` over
  the world's own domain.
- **Prefix domains** make "D(v) ⊆ D(u)" a size test and the inclusion
  the identity on indices, which a boolean checker in Rocq can use as
  is; the design's §8 names the lemmas (persistence, soundness of LJ for
  such models, a boolean forcing function reflected,
  `fo_kripke_refutation`).
- **Glivenko's failure** needs an infinite model: world n with domain
  `0 .. n` on a chain, `p(k)` forced from world k + 1 on; it refutes
  `¬¬∀x (p(x) ∨ ¬p(x))` and the double-negation shift. Step 77's plan
  takes it as a Rocq lemma.
- **Drawing**: the text form is a line per world (`world 0 (the model's
  world): domain {0}; below 1`); SVG, LaTeX and Typst draw no first-order
  model yet.
- **Open**: the batch's race; the connection prover's long run at the
  library's path length of 8 on some sequents (76a's open item), which
  keeps the library's default from reaching the embedding and the model
  search there.

## Session 76c: certified, drawn and documented

Opus 5.5 at `high` up to the three commits' first gate (the Rocq
library, the writer, the drawings, the command, the protocol and the
docs were written on it); **Fable 5.2 at `high` from the panel's
findings on** (the author's word, relayed by the planning session): the
fixes of the panel's and the reviewer's findings, the measurement and
this report.

### Outcome

**rocq-deduxit**, one commit on `main@origin` (`mmvluwsx`): change
`vmoknupz`, "Add first-order countermodels: Kripke models with growing
domains, LK and finite structures". No `Admitted`, no axiom: every
lemma closed, and the eight new theorems answer `Closed under the
global context` in `tests/Assumptions.v`.

- `Intuitionistic/FirstOrderFormulas.v` and `FirstOrderCalculus.v`
  (trusted): formulas over `Deduxit.Linear.Terms` (`var p [terms]`,
  `bot`, `wedge`, `vee`, `imp`, `fall`, `fex`; `top`, `neg`, `equiv`
  defined), `lj R Γ C` (LJ and LM over the propositional `rules`
  record) and `lk c Γ Δ` (LK, cut when `c = true`), each G1 with shared
  contexts, `∀L`/`∃R` with any `tclosed` witness and `∀R`/`∃L` with an
  eigenvariable fresh for the conclusion.
- `FirstOrderKripke.v`: `model` (any world and element type; `dom w`
  growing, `fn w` closed in the domain and agreeing with the world
  below on its elements, `pred w` and `absurd` growing, `d0` in every
  domain), `forces w η ρ A`, `persistence`, `forces_subst`,
  `kripke_soundness`, `kripke_countermodel`; the section `Chain`.
- `FirstOrderClassical.v`: boolean evaluation in a finite structure,
  `classical_soundness` of LK (constructive: truth is computed),
  `classical_countermodel`; `eval_fast`, which stops early, proved equal.
- `FirstOrderCheck.v`: the data (`structure`, `kripke_model`; a table
  `(f, arity, values)`, facts `(p, tuple)`), `srefutes`, `kwell_formed`,
  `kforces`/`kforces_fast`, `krefutes`, and the theorems
  `fo_classical_refutation : srefutes eqb S Γ Δ = true -> ~ lk c Γ Δ`
  and `fo_kripke_refutation : (reflect …) -> krefutes eqb M R w Γ C =
  true -> ~ lj R Γ C`. The classical structure is also the one-world
  Kripke model (deduxit writes it so in LJ and LM).
- `tests/Intuitionistic/FirstOrder.v` (certificates in the writer's
  form, the chain's theorems applied, `krefutes … = false` and
  `srefutes … = false` per condition, two tests that the check stops
  early), README, CHANGELOG, `rocq.md`, `typos.toml` (`fo`).

**deduxit**, on `xukuqmwp`:

- `nvoklttl` "Hold the Kripke check's order and growth passes to the
  work bound": 76b's check counted those steps and compared them with
  the bound only in the agreement pass (the integers member's finding).
- `yxsmmuuq` "Certify and draw first-order countermodels": the writer
  (`ordinary/rocq_models.rs`; `rocq::write_structure`,
  `write_kripke_structure`, `DEDUXIT_FIRST_ORDER_MODEL_PRELUDE`,
  `Unsupported::Domain`, `MOST_DOMAIN`), the drawings' layout
  (`export/models.rs`) and the three targets (`svg::structure`,
  `svg::kripke_structure`, the same in `latex` and `typst`), tests and
  24 snapshots.
- `wumrutkq` "Write first-order countermodels' certificates and
  drawings from the command and the protocol": `prove` and `check`
  (`show::FirstOrderModel`), `ordinary.draw`, `ordinary.export` (takes an
  outcome now) and `ordinary.certificate`, the `rocq-deduxit` check's
  three command-written certificates and a planted fault, the manual
  (`ordinary-logic.md` "Finite models", `rocq.md`, `status.md`,
  `workspace.md`), the guide, the changelog, the rules.

### What step 77c needs

The lemmas, in `Deduxit.Intuitionistic.FirstOrderKripke`, for every
symbol type, every `R : Calculus.rules` (LJ and LM, with cut or without)
and every predicate symbol `p`:

```coq
Theorem chain_excluded_middle : forall R (p : symbol),
  ~ lj R [] (neg (neg (fall (vee (var p [bound 0]) (neg (var p [bound 0])))))).

Theorem chain_double_negation_shift : forall R (p : symbol),
  ~ lj R [] (imp (fall (neg (neg (var p [bound 0])))) (neg (neg (fall (var p [bound 0]))))).
```

The model is `chain_model : forall R, model nat le nat 0 chain_dom
chain_fn chain_pred chain_absurd R`: worlds the natural numbers in
their order, `chain_dom n d := d <= n`, `chain_pred n p [k] := k < n`
(every unary predicate holds of `k` from world `k + 1` on), every
function `0`, no world forcing `⊥`. No world forces `∀x. (p(x) ∨
¬p(x))` or `∀x. p(x)` (at world `n` the element `n` fails now and holds
at `n + 1`), and every world forces `∀x. ¬¬p(x)`. The manual's "Finite
models" already names the model and both lemmas; 77c shows it.

### Measured

A sample of the models the model search finds on ILTP's first-order
problems (every problem `kripke-iltp-first-order-1.csv` lists as
refuted, 104, and the first 60 of `models-iltp-first-order-1.csv`;
`--method models --timeout 20s --format rocq --standalone`, cores 7-9):
158 refuted and certified (six of the 164 were generated-family names
my script gave the wrong path), **all 158 certificates compile**
against the library built in place: 60 by `fo_classical_refutation`
(classical logic), 98 by `fo_kripke_refutation` (intuitionistic logic),
of which 27 are Kripke models of more than one world and 71 the one
world of a structure.

D17: the ratchet passed in the gate; propositional paths are untouched
(the behaviour lock unchanged).

### The panel

On the Rocq statements against the Rust check, the writer and the
drawings. The argument member ran on Fable (started while the session
was on Opus, so it was not the writer's model), counterexamples on
Opus, integers on Sonnet, each at `high`.

- **No soundness break**: no certificate that compiles states anything
  false or another sequent. The counterexamples member compiled 788
  certificates of generated sequents and 125 of ILTP problems, each
  also against a statement written by its own de Bruijn writer
  (`exact certificate`), with its own evaluator agreeing on every
  model; 708 SVGs parsed and 530 LaTeX and Typst tables compiled.
- **Rocq did not stop early** (argument): `forallb` and `&&` evaluate
  every member under `vm_compute`, so a model the Rust check accepts in
  30 ms (eight nested `∀` over eleven elements, by grounding) did not
  compile in 200 s. Answered in the library: `eval_fast` and
  `kforces_fast` (`allb`, `anyb`, `if`), proved equal to the plain
  functions, are what the checks compute; the witness compiles in under
  a second, and two tests pin it.
- **A table listed twice** (all three): the Rust checks read the last,
  the writer wrote the first, so Rocq refused a model `check` accepted.
  The writer and the drawings take the last.
- **Tables of symbols no formula uses** (counterexamples, argument): the
  writer took its symbols from the whole term table, the check from the
  formulas' terms. `applied` walks the formulas as the check does (I
  had found and fixed this while the panel ran; their witnesses
  compile).
- **The statements were cut-free** (argument, counterexamples):
  deduxit's checker takes `Cut` in LJ and LM and an empty right side's
  `WR`, which the cut-free `lj` covers only by cut elimination, unproved
  in the library. The certificates now state the calculi with cut
  (`cut := true`, `lk true`); the theorems hold for every choice.
- **A domain of 2³² elements** (integers, counterexamples): the Rust
  check accepts it where no quantifier ranges over it, Rocq's unary
  numbers took 8 GB. The writer refuses a model too large for Rocq
  (`Unsupported::LargeModel`, a `no certificate: …` line; the bounds
  are in the reviewer's round).
- **The layout** (integers): the SVG's words listed the whole domain
  (16 GB for 2³²; now `0 to N`), `vec![0; arity]` for an unused
  symbol's arity of 4·10⁹ (a symbol of more than 16 arguments is
  counted, not shown), scans per table and per constant without a stop
  (48 s at a 1 s limit; maps and a poll per world now: 0.25 s).
- **76b's check** (integers): its closure and growth passes were not
  held to the work bound (commit `nvoklttl`, with a test).
- The manual named one of the chain's two formulas (argument): both
  are written now.

Every witness was run again on the fixed build: the ten forged outcomes
compile, the large domain is refused with its line and drawn in 12 MB.

Left as it is, said in the rules: Rocq finds a table or a fact by a
scan, so a sequent of thousands of function symbols compiles slowly
though the Rust check is fast.

### The reviewer's round

`fresh-reviewer` on Opus 5.5 at `high` (the session was on Fable by
then). It found every item of the brief present, no `Admitted` or
axiom, the writer agreeing with the Rust check on a list of points it
compiled certificates for, and nothing against the writing rule; and
two findings, both answered:

- **A model `check` accepts whose certificate Rocq cannot compile**: a
  table of 30 000 values overflowed Rocq's stack (a list literal is
  read recursively; 20 000 entries compile, 24 000 do not), and a table
  of 20 000 values of 19 999 took 57 s and 7.4 GB (unary numbers,
  compared one by one). The writer now counts what it writes
  (`Bounded::number`, `Bounded::entries`) and refuses with
  `Unsupported::LargeModel` a domain past 4 096 elements
  (`rocq::MOST_DOMAIN`; Rocq warns of a number from 5 000 on, and a
  certificate compiles in silence), a list past 8 192 entries
  (`rocq::MOST_ENTRIES`), or numbers adding up past 2²⁰. At that edge
  (4 096 elements, a table of 4 096 values of 250) a certificate
  compiles in 6.8 s and 1.1 GB. The statements "costs what deduxit's
  own check costs" in the library's changelog and a comment, and the
  rule's sentence, are corrected.
- **A table of a symbol the sequent does not name was drawn as unary**
  (`h(0)` twice for a table of four values): a table is shown only
  where its length is `size^arity`, the arity the sequent's or, for an
  unnamed symbol, the one the length gives; otherwise it is counted.

The model search's own models are far below these bounds (the 158 of
the measurement compile as before).

### Departures

- **The terms are `Deduxit.Linear.Terms`** (departs from the library's
  rule that `Deduxit.Intuitionistic` shares nothing with
  `Deduxit.Linear`): a copy would give deduxit's two first-order
  developments two term types, and moving the file to a `Deduxit.Base`
  would change a released path. `rocq.md` says so.
- **`lk c` takes a `bool`**, not a `rules` record (the design named
  none): LK has no optional ex falso.
- **Certificates state the calculi with cut** (departs from the
  propositional certificate's `intuitionistic`, which the design took
  as the model): the panel's point above. The propositional Kripke
  certificate could state the same; not changed here (a released
  form).
- **The boolean forcing is a recursion per world that stops early**,
  not `KripkeCheck`'s bottom-up vectors (departs from the design's
  "`KripkeCheck`'s shape"): the bound variables' environment varies per
  element, so there is no vector per subformula.
- **The drawings** (departs from the design's §5): in SVG a structure
  is one box like a world, not tables; in LaTeX and Typst a Kripke
  model is a table with a row per world, since neither has a drawing of
  a propositional model to follow. At most 16 values and 16 facts per
  world are shown.
- **`ordinary.export` takes an outcome** (the catalogue's `from` list
  grew), so that a model exports as text or a table.
- **The gate ran on the whole change before the three commits**, not
  between them (the brief asks it between commits): the first two
  commits are parts of one tested state.
- One `make` of the library ran on cores 10-12 by my mistake (under a
  minute); everything else kept to 7-9, the panel and the reviewer to
  13-15.

### What was checked, and by what

- **rocq-deduxit**: `nix flake check` at `vmoknupz` (its last commit),
  under the machine lock, passed: the library on Rocq 9.0 to 9.3, every
  test, the 53 theorems' assumptions closed, the bridges, `conventions`,
  `typos`, `treefmt`. It was run again after each amendment (the
  short-circuiting evaluators, the corrected comments and rules).
- **deduxit**: the gate on the whole change before the commits (passed,
  the ratchet among it); `cargo clippy --workspace --all-targets` and
  `cargo test --workspace` after every later fix; `nix flake check
  --override-input rocq-deduxit git+file://…/rocq-deduxit` at the
  chain's head rebased onto `main@origin` (`vwrvwvpl`), under the
  machine lock: passed, every check (a first run failed `doc-private`
  alone, on a rustdoc link of the new layout module, fixed). With the override the `rocq-deduxit`
  check compiles the six new snapshots and the three certificates the
  command writes and refuses the planted model; the `export` check
  renders the six SVGs and compiles the six LaTeX and six Typst tables.
- The panel's and the reviewer's witnesses, each run again on the fixed
  build (above).
- Not run: the web bindings' own tests beyond the flake's checks (no
  file under `web/` changed); the comparison; the baseline.

### For the planning session

- **Order of landing**: push rocq-deduxit's `vmoknupz` first, then move
  deduxit's lock (`nix flake update rocq-deduxit`, a commit of its own)
  on top of this chain: until the lock moves, deduxit's `rocq-deduxit`
  check fails without the override, since six new snapshots and three
  command-written certificates need the new library. The library's
  version stays 0.6.0 (`export::rocq::DEDUXIT` unchanged); its release
  is the author's.
- The batch writes no countermodel's drawing or certificate, of either
  kind (as before).
- The propositional Kripke certificate still states the cut-free
  calculus; the same strengthening is one line there and a snapshot
  bless, if the author wants it.
