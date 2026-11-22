# First-order countermodels: the design (step 76)

Written by session 76a (2026-11-22), from step 76's prompt, 75d's
`FiniteCountermodel`, the reports of steps 55, 74 and 75, and the
sources below. 76a builds sections 2 to 5 and 9 (the classical half);
76b sections 6 and 7 (Kripke models with growing domains); 76c section
8 (the Rocq lemmas and certificates), the drawing's last formats and the
docs.

Sources read:
- [MACE2] W. McCune, *MACE 2.0 Reference Manual and Guide*, ANL/MCS-TM-249,
  2001 (arXiv cs/0106042), §5 "How MACE works": flattening to relations,
  "well-defined and closed" clauses, one SAT problem per domain size, and
  its scalability limit (a 7-variable clause gives 6⁷ instances at size 6).
- [MACE4] W. McCune, *Mace4 Reference Manual and Guide*, ANL/MCS-TM-264,
  2003 (arXiv cs/0310055), §1 and §5: why Mace4 left the SAT encoding for
  SEM's ground equational search (flattening blows up clauses with many
  variables or deep terms), and the least-number heuristic for symmetry.
- [RSV16] G. Reger, M. Suda, A. Voronkov, *Finding Finite Models in
  Multi-Sorted First-Order Logic*, SAT 2016 (arXiv 1604.08040), §3: the
  MACE-style method as Paradox does it, DC-models (Theorems 1 to 3),
  flattening, functionality and totality clauses, definition introduction,
  clause splitting, Paradox's symmetry-breaking clauses on constants
  (§3.4), maximum sizes (§3.5), and why Vampire is not incremental across
  sizes (variable elimination).
- [PARADOX] K. Claessen, N. Sörensson, *New Techniques that Improve
  MACE-style Finite Model Finding*, CADE-19 workshop on model computation,
  2003. Not reachable from this machine (both copies redirect, the thesis
  that holds it refuses); its techniques are cited here as [RSV16] §3
  states them, with the citation [5] there.
- rocq-deduxit's `Intuitionistic/Kripke.v` (forcing, `persistence`,
  `kripke_soundness`, `kripke_countermodel`, `eval`, `classical_refutation`)
  and `Intuitionistic/KripkeCheck.v` (a boolean checker reflected into
  forcing, `kripke_refutation`), which a propositional countermodel's
  certificate rests on.
- For section 6: A. S. Troelstra and D. van Dalen, *Constructivism in
  Mathematics* I (1988), §2.5–2.6 (Kripke models with growing domains,
  completeness for them, and for trees).

## 1. What a user gets

- `prove --logic classical` on a first-order sequent outside grounding's
  classes that has a finite countermodel answers "not valid" with a
  checked finite structure: its domain, each constant's element, each
  function's table and each predicate's true tuples, printed after the
  verdict line, drawn, written in JSON, certified (76c).
- `--method models` names the search; `--model-size N|none` bounds it.
  Past the bound or the time the answer is unknown, never "valid".
- Intuitionistic and minimal logic (76b): a finite Kripke model with
  growing domains, checked the same way; until 76b their first-order
  "not valid" is a classical structure's (grounding's or this search's),
  which refutes in every logic.

## 2. The type: `Structure` with functions (76a), Kripke models (76b)

**Decision**: `Structure` gains `functions`: each function symbol of
arity k ≥ 1 by name with its table, the values of its `size^k` argument
tuples in lexicographic order (`[["f", [1, 0]]]` for `f(0) = 1, f(1) = 0`).
A constant stays in `constants` (arity 0). The table is total by its
length, and the evaluator reads `f(d̄)` at index `Σ dᵢ·size^(k−1−i)`.

- Why a full table, not a list of (tuple, value) pairs: a structure must
  interpret every function everywhere, and a list must then be checked
  for totality and for duplicates; the table makes both a length test.
  The size is `size^k` either way.
- `FiniteCountermodel { sequent, structure }` keeps its shape; its check
  (`FiniteCountermodel::check`, the evaluator over the original sequent)
  stays the one trust. A function symbol the sequent uses and the
  structure leaves out, a table of the wrong length, a value past the
  domain are `Error::Structure`; the up-front pass checks only the terms
  the sequent's formulas use (75d's rule).
- JSON: `"functions"` optional (`serde(default)`), written only when not
  empty, so every level-25 structure is written and read as before; a
  structure with functions is written at **level 26** (`wire::FUNCTIONS`),
  since a level-25 reader's `deny_unknown_fields` refuses the key. The
  wire schema's `structure` gains `functions`.
- The evaluator's bounds: the work bound counts every term evaluation,
  and the tables are charged to `limits.memory_bytes` (75d's
  `ENTRY_BYTES` per entry), so a 20-byte JSON table claiming `size^k`
  entries is refused before it is held (the reader refuses a table whose
  length is not `size^arity`, and `size^arity` past `u32` or the memory
  bound before it allocates).

**For 76b**: `FirstOrderKripke` (name settled there): worlds as
`kripke::Model` has them (arrows to the worlds directly above, a root);
per world a domain size `dᵥ` with `dᵥ ≤ dᵤ` along every arrow (the
domain of `v` is the prefix `0 .. dᵥ`, so growth is a size test);
constants' elements in the root's domain; each function's table per
world, agreeing on the arguments of the smaller domain along every arrow
(persistence of terms); each predicate's true tuples per world, growing
along arrows; minimal logic's `⊥` as a nullary predicate that may be
forced. A classical `Structure` is the one-world case. Its check is the
forcing relation over the original sequent (Troelstra–van Dalen
§2.5.2), iterative, with the same bounds.

## 3. The classical search

**Decision**: for n = 1, 2, … up to the bound, ground the sequent's
refutation over the domain `{0, …, n−1}` and ask the SAT engine whether
it has a model; the first model found is read off as a structure and
checked.

### 3.1 The encoding, and why not MACE's clauses

The sequent fails in a structure exactly when its validity formula `F =
⋀Γ → ⋁Δ` is false there. 75d's `prepare` already builds `F` in negation
normal form (the hypotheses negated), with named variables, an
equivalence expanded both ways. Over a domain of size n:

- `∀x. A` becomes `⋀_{d<n} A[d]` and `∃x. A` becomes `⋁_{d<n} A[d]`:
  both quantifiers expand (grounding's walk expanded only `∃`, its `∀`
  being Skolem constants). No Skolem function is introduced.
- An atom `p(t₁, …, tₖ)` under bindings `ē` becomes `p(e₁, …, eₖ)` when
  every `tᵢ` is a variable or a constant's element; a term with a
  function symbol goes through **value atoms**: `f(d̄) ↦ e` is a
  propositional atom `F_f(d̄, e)` per principal term (`f` applied to
  domain elements, [RSV16] §3.1) and value. A ground term `t` has the
  formula `val(t, e)` ("t denotes e"): for a domain element `d`, true if
  `d = e`; for `f(s₁, …, sₖ)`, `⋁_{e₁…eₖ} (⋀ᵢ val(sᵢ, eᵢ) ∧ F_f(ē, e))`.
  The atom is then `⋁_{e₁…eₖ} (⋀ᵢ val(tᵢ, eᵢ) ∧ p(ē))`. Every `val` is
  memoised per (term, bindings of its free variables, value), so a term
  costs its table, `n^(k+1)`, once.
- **Functionality and totality** (Paradox, [RSV16] §3.2): per principal
  term `f(d̄)`, `⋁_e F_f(d̄, e)` (at least one value) and `¬F_f(d̄, e₁) ∨
  ¬F_f(d̄, e₂)` for `e₁ < e₂` (at most one). A constant is a principal
  term of arity 0 with the same clauses. Under them each `val(t, e)` is
  true for exactly one `e`, and the expansion of an atom is equivalent to
  the atom in the structure the `F` atoms define (the induction of
  [RSV16] Theorems 1 to 3, read for formulas).
- The SAT engine's classical path (`sat::decide`, Tseitin with
  polarities) takes the result as the sequent `Ax ⊢ F_n`: `Ax` the
  functionality and totality formulas, `F_n` the expansion. "Not valid"
  gives an assignment that makes `Ax` true and `F_n` false, which is a
  structure of size n falsifying `F`; "valid" says no structure of size n
  does.

Why not MACE's and Paradox's way (Skolemize, clausify, flatten each
clause, instantiate every clause's variables):

- **They work on clause sets because their solvers take clauses**;
  deduxit's encoder takes a formula and names its subformulas itself, so
  the clause form, its Skolem functions and its definitions would be a
  second normal form beside the one `prepare` makes.
- **Skolem functions bring 75d's fault back**: one Skolem symbol per
  occurrence is what Herbrand needs, and a constant per variable answered
  ILTP's SYN316+1 "valid". Expanding `∃` over the domain needs no
  Skolem symbol at all: the disjunction is exactly the semantics of `∃`
  in a structure of size n, so there is nothing to keep consistent. (The
  size is the same: a Skolem function of k arguments is `n^k` cells of n
  values; the disjunction is n per instance of its k enclosing
  universals.)
- **Flattening's blow-up is the variables per clause** ([MACE2] §5:
  distributivity's 7 variables give 6⁷ instances; [MACE4] §1, the reason
  Mace4 left SAT). The formula's expansion costs what its quantifier
  nesting and term depth cost, and miniscoping (75d's, run before) keeps
  that nesting as shallow as it goes; the `val` memo keeps each term to
  its table. Paradox's remedies (definition introduction, clause
  splitting, [RSV16] §3.3) are what naming subformulas does for a
  formula: Tseitin's encoder names every subformula, so a shared one is
  a variable once.
- What is lost: Paradox's sort inference and monotonicity (no sorts
  here), and Mace4's ground equational reasoning (no equality yet: the
  readers refuse `=`, step 78 adds it; with equality, `=` becomes the
  diagonal on domain elements and needs no axioms in a DC-model, [RSV16]
  §3.2 "equalities between variables").

### 3.2 Symmetry breaking

Any permutation of the domain maps a model to a model ([RSV16] §3.4), so
a search may require a canonical one. **Decision**: Paradox's clauses on
the sequent's constants `a₁, …, a_l` in their order of first occurrence:
`aᵢ` takes one of the first `i` elements, and if `aᵢ` takes element
`m > 0` then some earlier constant takes `m − 1` ([RSV16] §3.4, citing
Paradox). Every structure is isomorphic to one that meets them (number
the elements in the order the constants first name them, the rest
after), so a size with a model still has one. Principal terms of
functions in the second clause ([RSV16]'s "1 < m < n" case) and Mace4's
least-number heuristic (a search-time rule, not clauses) are left out:
measure first. The structure read off is checked whatever the clauses
say, so a wrong symmetry clause can lose models (an unknown), never
make a wrong "not valid".

### 3.3 Sizes, incrementality, bounds

- Sizes 1, 2, …: a size's problem is built and solved afresh. Paradox
  keeps the solver across sizes ([RSV16] §3.2 "Incrementality"); Vampire
  does not, for variable elimination's sake; batsat has neither problem,
  but the problems grow so fast with n (`n^q` per formula) that the last
  size dominates, and building afresh keeps each size's statistics and
  memory account its own. Measure before adding incrementality.
- **The bound** (`search::Options::model_size`, default 6 in the library,
  `--model-size N|none` in the command, none deepening under the clock):
  sizes past it are not tried; the answer is `Reason::ModelSize { size }`
  ("no model up to size N"). Each size is built within
  `limits.occurrences` (the expansion's nodes) and `limits.memory_bytes`
  (75d's charges), its solver within `limits.work`; a size refused at a
  bound ends the search with that size's unknown. A maximum size from the
  input ([RSV16] §3.5: a sequent without function symbols has a model of
  at most its constants' number plus one per `∃`, if any) is grounding's
  domain already, and such sequents are grounding's.
- Polling: the expansion asks the stop every 4 096 steps (75d's `poll`),
  the solver as it does; a stop is `Unknown(Stopped)`.

### 3.4 The answer

- A model found is read off (each predicate's true tuples, each
  function's table from its `F` atoms, each constant from its atoms) and
  checked by `FiniteCountermodel::check` against the original sequent
  before it is an answer. A check that fails is `Error::Countermodel`, a
  defect. A check refused at a bound is unknown (the search's "not valid"
  has no argument but the model, unlike grounding's).
- "Not valid" holds in every logic: a classical countermodel is a
  one-world Kripke model.
- The outcome: `Search::Models(Statistics)` (the sizes tried, the last
  size's atoms, formulas and solver counters), `structure` as 75d's.

## 4. The dispatch

- **Library default** (`decide_within`, no clock): grounding for its
  classes; else, classical: the connection prover (path bound 8), and
  where it gives way, the model search (`model_size` 6), then the
  embedding; intuitionistic and minimal: the embedding, then (76b) the
  Kripke search. The model search runs only after the provers have given
  way, so a valid sequent the prover proves is as fast as before.
- **The command** (the time limit): the first-order method's share (75d,
  90%) is split between the connection prover and the model search by
  measurement on ILTP's first-order problems (76a): the prover proves
  all but three of its 599 theorems within 0.6 s at 2 s, so most of its
  share past that is wasted on non-theorems, where a model search may
  answer. A race on a second thread (the command starts a pool after
  100 ms for the linear engines) is the alternative; decided by the
  measurement, with "no valid answer slower" as the condition.
- `--method models` (`Method::Models`, a new last variant) runs the
  search alone in every logic (classical "not valid" answers for all
  three; it never answers "valid").

## 5. The drawing (76a: text; 76c: the rest)

The text form (`Structure`'s `Display`, after the verdict line) adds a
line per function: `f: 0 ↦ 1, 1 ↦ 0` for unary ones, a table row per
first argument for binary ones (`g: (0, 0) ↦ 1, …` past that). SVG,
LaTeX and Typst draw the structure as tables (the domain, the
constants, one table per function and predicate); Kripke models
(76b) draw as the propositional ones with each world's domain and true
atoms in its box. The JSON is the outcome's `structure` at level 26.

## 6. Kripke models with growing domains (76b)

- **Semantics** (Troelstra–van Dalen §2.5): worlds partially ordered,
  a domain `Dᵥ` per world with `Dᵥ ⊆ Dᵤ` for `v ≤ u`, constants in the
  root's domain, function tables per world agreeing on smaller domains,
  atoms growing; `v ⊩ ∀x. A` iff for every `u ≥ v` and `d ∈ Dᵤ`, `u ⊩
  A[d]`; `v ⊩ ∃x. A` iff some `d ∈ Dᵥ` has `v ⊩ A[d]`; implication and
  negation as propositionally. First-order intuitionistic logic is
  complete for such models over trees (§2.6), but it has no finite model
  property, so the search is bounded and its exhaustion is no answer
  (75a's design, §5).
- **The search**: one SAT problem per (worlds K, elements N), K and N
  growing in turn: a fixed frame shape per K (rooted trees of K worlds,
  enumerated up to isomorphism, or a tree of depth ≤ K whose shape the
  solver chooses through arrow variables), domain-membership variables
  `dom(v, d)` with growth clauses, atom variables per (world, tuple) with
  persistence clauses, value atoms per (world, principal term) as in 3.1
  with agreement along arrows, and forcing variables per (world,
  subformula instance): `∀` and `→` at `v` expand over the worlds above
  `v` and their domains; the root forces Γ and not C. Bounds: K ≤ 4,
  N ≤ 4 by default (the cost is `K·N^q` per formula), raised by flags.
  intuit's approach (worlds as solver models, implications as a theory,
  54s) is the alternative for the propositional skeleton; its first-order
  extension is not in the literature read, so the direct encoding comes
  first and is measured.
- **Minimal logic**: `⊥` is a nullary predicate that a world may force
  (as `kripke::Model`'s `falsity`), so `⊥` true at a world refutes
  nothing by itself.
- The model is checked by the forcing relation over the original
  sequent before it is an answer; drawn as the propositional models with
  domains.

## 7. What 76b's check must keep

- One evaluator for both kinds: a `Structure` is the one-world Kripke
  model with constant domain, so 76b's forcing on one world must agree
  with `Structure::refutes` (a test runs both on every classical model
  the search finds).
- `kripke::Model`'s propositional checker (`Model::forcing`) stays the
  propositional one; the first-order checker is a new function, its
  bounds `MODEL_CELLS` and the memory account as 55's.

## 8. The Rocq side (76c)

What a certificate of a first-order refutation states, by analogy with
the propositional one (`kripke_refutation`: a boolean check that
computes to `true`, reflected into forcing, and soundness concluding
`~ lj R G C`):

- **In rocq-deduxit** (made, checked and pushed there first): first-order
  formulas over de Bruijn variables with function symbols
  (`Intuitionistic/Formulas.v` extended or a `FirstOrder` module beside
  it), LJ's and LK's quantifier rules as deduxit's checker reads them
  (G3 with the eigenvariable condition), Kripke models with growing
  domains (a domain predicate per world, monotone; interpretations per
  world, persistent), `persistence` for forcing, **soundness** of LJ for
  them and of LK for one-world models with constant domain (a
  `Structure`), a boolean forcing function for finite models
  (`KripkeCheck`'s shape, quantifiers as `forallb`/`existsb` over the
  domain lists) reflected into forcing, and the theorems
  `fo_kripke_refutation` and `fo_classical_refutation`.
- **The certificate** writes the sequent and the model as data and proves
  `~ lj R G C` (or `~ lk …`) by `fo_*_refutation` and `vm_compute` of the
  check, as the propositional certificate does; its size is the model's
  tables, so the command refuses a model past a bound with the usual
  `no certificate: …` line.
- Opus writes the lemmas; the soundness proof is the work (the
  substitution lemma for terms under the de Bruijn representation is its
  core).

## 9. What 76a builds, and what it leaves

76a: `Structure.functions` and its evaluation, JSON (level 26) and
schema; the classical model search (`ordinary::models`, `Method::Models`,
`--method models`, `--model-size`); its place in the dispatch by
measurement; the text form; the measurement on ILTP's first-order
problems (classical `CounterSatisfiable` beside grounding's 77 and
nanoCoP's 84); the panel and the reviewer.

76b: section 6 and 7. 76c: section 8, the drawings in SVG, LaTeX and
Typst, the manual and the guide.

## 10. 76b's decisions (written at its start)

- **The type** `ordinary::KripkeStructure`: `worlds` (each a domain
  size, its arrows to the worlds directly above as `kripke::Model`'s,
  each function's table over its own domain, each predicate's tuples,
  `falsity` for minimal logic), `constants` once for all worlds (each
  element in every world's domain), and `world`, the world refuted.
  Domains are prefixes `0 .. size`, so growth along an arrow is
  `size_v ≤ size_u`; tables agree on the smaller domain's tuples along
  every arrow; predicates and `falsity` grow. `KripkeCountermodel
  { sequent, model }` beside `FiniteCountermodel`, on the outcome as
  `kripke` (JSON `"kripke_structure": {"sequent", "model"}`), wire level
  27.
- **The check** is the forcing relation's definition over the original
  sequent at the refuted world, on a stack of its own, worlds above
  taken in the reflexive-transitive closure of the arrows (Kahn's order
  refuses a cycle): `→`, `¬` and `∀` quantify over the worlds above,
  `∃` over the world's domain, `⊥` is `falsity` in minimal logic and
  never forced otherwise. It does not rely on persistence, which it
  checks apart. One world with no falsity agrees with
  `Structure::refutes` (a test runs both).
- **The search** writes "a Kripke model of K worlds with domains of at
  most N elements refutes the sequent" as one propositional sequent per
  (K, N): an order `le(v, u)` for `v < u` chosen by the solver
  (reflexive, transitive, world 0 below all: every finite rooted poset
  has a numbering of this kind, so no frame enumeration), prefix
  domains `dom(v, d)` growing along `le`, predicate atoms per world
  growing, value atoms per world and principal term (total and
  functional on the world's domain, values in it, agreeing along `le`),
  constants once (in the root's domain, Paradox's symmetry clauses on
  them, which only permute the root's domain and so keep every prefix),
  and the forcing formula `Φ(v, A, ē)` per world, node and values,
  memoised: `→` and `∀` as conjunctions over the worlds `u ≥ v` guarded
  by `le(v, u)` (and `dom(u, d)`), `∃` over `dom(v, d)`. The root forces
  `Γ` and not `C`. A model of K worlds and N elements fits every
  larger pair (a copy of a maximal world, unused elements), so the
  search tries the diagonal: for L = 1, 2, …: the classical (1, L)
  first, which is cheap, then (min(L, worlds), min(L, size)).
- **Bounds**: `search::Options::model_worlds` (library 4, command none)
  beside `model_size`; an exhausted search is unknown,
  `Reason::KripkeSize { worlds, size }`.
- **The dispatch**: `models::decide` in intuitionistic and minimal logic
  runs this search (with the classical sizes first), so the library's
  default after the embedding takes it at once; the command races it
  beside the embedding on a thread of its own, as the classical one
  beside the connection prover, measured.
- **Glivenko's failure has no finite countermodel**: in a finite frame
  every world sees a maximal world, which forces what is classically
  true in its structure, so every world forces `¬¬A` for a classically
  valid `A`. The countermodels of `¬¬∀x (p(x) ∨ ¬p(x))` and of
  `∀x ¬¬p(x) → ¬¬∀x p(x)` are infinite chains (world n with domain
  `0 .. n`, `p(k)` forced from world k + 1 on); the search answers
  unknown on them, which a test pins.
