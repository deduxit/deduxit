# Step 70: resolution at teaching scale

## Session 70a: the normal forms

Opus 5.5 at `high`, a reused worker in the jj workspace
`../deduxit-wt-step-58a-610ed3`, from the brief `start-70a.md` on
2026-11-20, cores 7 to 9; begun on `main@origin` `xuptrvts` (0.6.0
prepared), rebased onto `ttwsorlx` (0.6.0 published) without conflict and
then onto `kpuzxuyv` (step 72's `grade` and dataset).

### Outcome

A classical sequent is rewritten into a normal form step by step, and
what the form promises is checked by the SAT solver before the result is
given. Five forms: negation normal form, conjunctive and disjunctive
normal form by distribution (each formula equivalent to its form), and
the clauses of Tseitin's and of Plaisted and Greenbaum's encodings of the
sequent's negation (valid exactly when the input is). They are in the
library (`ordinary::normal`), the command (`seq normalize`), the JSON at
wire level 21, and the protocol as five operations of the ordinary suite
with the promises `equivalent` and `equisatisfiable`, so the workspace
takes them as edges. The manual's transformations chapter, the protocol
and status chapters, the guide and the changelog say so. The page is
70c's.

The chain on `main@origin` `kpuzxuyv`, the report last:

| change | what |
|---|---|
| `pruxoxoo` | the library: `ordinary::normal`, `Limits::form_occurrences` with `Refusal::Form` and `Phase::Rewrite`, the level-21 document, the error code in the schema |
| `sppxpntk` | the protocol: `ordinary.nnf`, `cnf`, `dnf`, `tseitin`, `plaisted_greenbaum`, the promises and their checks |
| `qknsnxmm` | the command: `seq normalize --to FORM --form-limit N`, the schema's `normal_form`, the schema check's two documents |
| `mqunoltr` | the behaviour lock: five calls of `seq normalize`, the command list and the operations' list |
| `wtsnzqps` | the manual, the guide, the changelog; README's test looks up `seq normalize`'s help |
| `vokmoxwy` | the rules files |
| (this report) | `plan/reports/70-resolution.md` |

### The forms and their bounds

- **Rewriting by laws.** NNF, CNF and DNF apply one equivalence law per
  step, in phases: `→` and `↔` (`A ↔ B` as `(¬A ∨ B) ∧ (A ∨ ¬B)`, or by
  cases `(A ∧ B) ∨ (¬A ∧ ¬B)` on the way to DNF), then the negations (De
  Morgan, double negation, `¬⊤`, `¬⊥`), then the constants (CNF and DNF
  only), then distribution. Each step takes the leftmost outermost place
  where a law of its phase fits, the order a textbook rewrites in. Two
  tables are the rewriting (`law`: which law applies at a node, `apply`:
  what it makes); nothing recurses over a formula: every pass collects
  the nodes a sequent reaches with a stack and sorts them by id.
- **A step rewrites every occurrence of its subformula at once**, since
  equal formulas are one node of the arena. That is how the arena works
  and halves the steps of a repeated subformula; the text says so.
- **The encodings** move each formula right of `⊢` left, negated, then
  name each compound subformula, innermost first, by `t1`, `t2`, …, with
  the clauses of its definition: both directions for Tseitin, the
  direction the subformula's polarity needs for Plaisted and Greenbaum
  (the classical encoder's polarity rule). A negation of a named
  subformula is a literal and gets no atom. `⊤` drops a clause, `⊥` drops
  from one, the empty clause is `⊥`. The new atoms' prefix is `t`, with
  `_` added until no input atom is the prefix and digits.
- **The bound**: `Limits::form_occurrences`, default 10 000
  (`DEFAULT_FORM_OCCURRENCES`), a teaching scale like `table_atoms`, held
  by every sequent on the way, the input included; past it
  `Refusal::Form` (code `form_too_large`, setting
  `limits.form_occurrences`), the command's `--form-limit N|none`. Each
  pass is charged to `limits.work`, the steps and new nodes to
  `limits.memory_bytes`, and the stop is asked once per step. A CNF of
  512 clauses of 17 literals (`(a1 ∧ b1) ∨ … ∨ (a9 ∧ b9)`, about 9 000
  occurrences) took 1.4 s in a debug build with its check; ten pairs are
  refused at the default.
- **Options for every front end** (D15, D16): `normal::Options { form }`
  (CNF by default), plain data with serde; the command maps `--to` onto
  it, the protocol names the form by its operation. The bound is a
  setting (`limits.form_occurrences`), since it bounds the call, not the
  question.

### The rewrites shown

A `Step` is the rule (`Rule`, its `name` and `law` as text), `before`
and `after` (ids of the result's arena, which is the input's grown, so
every intermediate formula stays and the JSON's numbers index the
result's `formulas`), the input's formulas after the step (`left`,
`right`) and the clauses it `added`. The sequent after step k is `left`
with the clauses of steps 1 to k, so an encoding's steps store linearly
many ids. The command prints a rewrite as `n. LAW, at BEFORE:` and the
sequent after it, a `Negate` step as `δ moves left of ⊢ as ¬δ`, a
`Define` step as `t names G: CLAUSES`. This is the data 70c's stepper
reads (`--format json`, or the library's `NormalForm`).

### The checks of each promise

- **Equivalent forms**: `⊢ (A₁ ↔ A₁') ∧ …` is decided in classical logic
  by `Method::Sat`, whose clausal proof `drup::check` accepts before the
  claim is valid. The claims copy the input into the result's arena by
  atom names (`transfer`), so a result from elsewhere (the protocol's
  output, a test's planted one) is judged as given.
- **Encodings**: first `fresh` (each definition names an atom the input
  lacks and no earlier definition named, by a formula over the input's
  atoms and earlier new atoms, so every assignment of the input's atoms
  extends uniquely to one of the definitions); then two claims decided by
  the SAT solver: Extends, `N, t₁ ↔ G₁, … ⊢ ∧C` (a model of the input's
  negation `N`, extended by the definitions, satisfies the clauses), and
  Restricts, `C ⊢ ∧N` (a model of the clauses, restricted, is one of
  `N`). Together the clauses are satisfiable exactly when `N` is, which is
  "valid exactly when the input is".
- **The result's shape** is checked too (`in_form`): NNF, CNF (a
  conjunction of clauses, or a constant alone), DNF, or a sequent of
  clauses; failing is `ClaimFails`.
- **A claim decided not valid** is `Error::ClaimFails` (kind defect),
  never an answer; one undecided leaves `NormalForm::is_checked` false,
  the command's exit 3.
- **Tests** (`normal::tests`): each form's steps and result on chosen
  sequents, pinned; 60 generated sequents under every form, every result
  checked, and every step of an equivalent form decided an equivalence
  on its own for a sixth of them; the bound refusing past it and
  admitting 32 clauses under it; new atoms avoiding `t1`, `t_1` and
  `t__2`; the check refusing a planted non-equivalent result and a
  definition that names an input atom.

### The command and the protocol

- **`seq normalize`** sits under `seq` beside `translate`, as the
  command's idiom for a transformation of a sequent (`interpolate` is a
  command of its own because it decides). `--to nnf|cnf|dnf|tseitin|
  plaisted-greenbaum`, CNF by default (what a newcomer means by "normal
  form" in a first course); `--form-limit`; the shared `--format`,
  `--output`, `--timeout`, `--memory-limit`, `--work-limit`,
  `--settings`. Text: the form's title and result, a line per step, a
  line per claim with its verdict and evidence. JSON: the level-21
  document `{"version", "deduxit", "form", "promise", "checked",
  "sequent", "result", "steps", "checks"}`, described in the manual's
  schema (`normal_form`, `normal_step`, `normal_check`) and written by the
  schema check. Exit 0 checked, 3 a claim unknown or the time limit, 2 an
  error (a form past the bound names `--form-limit` and `--to tseitin`).
- **The protocol**: `ordinary.nnf`, `ordinary.cnf`, `ordinary.dnf`,
  `ordinary.tseitin`, `ordinary.plaisted_greenbaum`, each a classical
  `ordinary.sequent` to a classical one (another logic is `InvalidOption`
  key `logic`), instant, reading the sizes, `limits.work`,
  `limits.memory_bytes` and `limits.form_occurrences`. Promises
  `equivalent` (one search) and `equisatisfiable` (two). The check of
  `equivalent` holds the given output to the input; that of
  `equisatisfiable` makes the form again (the output sequent carries no
  definitions), compares texts (part `form`), then asks the remade
  form's claims of `ordinary.decide` with `{"method": "sat"}`. A wrong
  output fails each check (`a_normal_form_keeps_its_promise_and_its_check_says_so`).
- **Wire level 21** (`wire::FORMS`), `wire::LEVEL` now 21; nothing reads
  the document back (written only, like level 16's), since a front end
  holds the sequent and asks again.

### Checks

- **The rebase onto step 72** conflicted in `cli/src/settings.rs` (72's
  `GradeArgs::settings` beside `NormalizeArgs::settings`: both kept) and
  in `CHANGELOG.md` (both "Added" lists under Unreleased, 72's first);
  README's test's `COMMANDS` merged to 23 entries under a length of 22,
  set to 23 in `wtsnzqps`. The command's suite passed after.
- **The gate** (capped, cores 7 to 9) passed three times: before the
  reviews' fixes, after the rebase onto `ttwsorlx`, and at the head on
  `kpuzxuyv` (the last ratchet's notes: cache misses moved on
  `search-chain-128` and `check-qbf-20-2`, which this chain does not
  touch):
  clippy, the tests, both `cargo hack` runs and the ratchet, every
  journey within its ceiling with the same counts both times (the
  largest change `render-latex` +1.86 %; `render-svg`'s cache misses
  moved 11.9 %, a note, not a failure: nothing of the drawing changed).
- **The tests**: the core crate's suite (527, one ignored) and the
  command's (the lock, README's, the manual's and the guide's examples
  among them) passed; after the panel's fixes the normal forms' and the
  protocol's tests again. The generated-sequents test takes 0.13 s
  since the claims go to the SAT solver directly (31 s while they went
  to the classical default, which ran the embedding).
- **`nix flake check`** under the machine lock passed three times, the
  last at the head on `kpuzxuyv`: every check, `wire-schema` among them
  (93 documents, the two normal forms and 72's included, 0 failures),
  72's `dataset`, `web-wasm-tests`, `manual`,
  `manual-links`, `conventions`, `typos`. **The web module** is 3 690 645
  bytes at gzip, under the 3 750 000 ceiling; before the rebase it was
  3 688 695, and the store's latest figures from other builds of the day
  were 3 671 626 and 3 671 856, so the five operations add about 17 KB.
- **By hand**: each form on the brief's kinds of input in the debug
  binary; a CNF of 512 clauses near the bound in 1.4 s with its check;
  ten pairs refused at the default naming `--form-limit`.

### The panel's and the reviewer's findings

**The panel's argument member** (`panel-argument`, Fable 5.2 at high)
stated the invariants from the code, checked every site and ran 13 edge
sequents under each encoding and 7 under each equivalent form: the
argument holds; no path reports a check held without the claims decided
valid, and Plaisted and Greenbaum's polarity keeps the claim "the clauses
conclude the input's negation". Four gaps at the edges, all fixed and
each pinned by a test (folded into `pruxoxoo` and `sppxpntk`):

1. **`claims` ignored an encoding's right side**: a library caller who
   set `result` to `¬a ⊢ ¬a` for the input `⊢ a` had both claims valid
   and the promise false. A result with a formula right of `⊢` is now
   `ClaimFails`.
2. **The protocol's check dropped the output's logic**: `a ∨ ¬a` claimed
   intuitionistically passed as the NNF of the classical one. Part
   `logic` fails it first.
3. **A failed re-check left the old checks** (`is_checked` true for a
   value changed since): `check` clears them first.
4. **A step's id outside the arena panicked** in `fresh` instead of
   being refused: `Error::IndexOutOfBounds` now.

**The fresh reviewer** (`fresh-reviewer`, Fable 5.2 at medium) ran about
60 calls, a fuzz of 400 random sequents under every form against an
evaluator of its own (2 000 runs, all agreeing: equivalent forms formula
by formula, encodings satisfiable exactly where the input fails), and
3 000-deep nestings without a stack overflow. Four findings, all fixed
(folded into `pruxoxoo`, `qknsnxmm` and `vokmoxwy`):

1. **The freshness check was cubic and asked no stop**: an `atoms_of`
   per definition over an arena that grows with the square of a deep
   chain; `(a → (a → … b))` 1 500 deep under `--form-limit none` took
   15.6 s past a 3 s time limit. It is one ranking pass and one pass of
   highest ranks now, linear: the same input takes 1.4 s, both claims
   checked (debug build, either encoding).
2. **The refusal suggested `--to tseitin` to an encoding and to NNF**:
   only CNF and DNF get that suggestion now.
3. **`wire.rs`'s level table stopped at 20**: row 21 and the normal
   form's bullet added.
4. **An encoding's last simplification had no step**: `⊢ ⊤` gave `⊥ ⊢`
   with only the `Negate` step shown. A negated constant is now a
   `NotTrue` or `NotFalse` step and a `⊤` hypothesis is left out by a
   new `TrueHypothesis` step, so the sequent after the last step is the
   result, which the generated test holds every form to.

The reviewer's notes that are not findings: a bound ending the rewrite
is exit 2 (a refusal) while one ending a check is exit 3 (an unknown
claim), as `cli.md` words it; the check of `equivalent` holds an
equivalent output whether or not it is in the form, as the promise says
(the library's `rewrite` checks the form itself).

### Departures

- **Normal forms of a sequent, formula by formula; encodings of its
  negation.** The brief speaks of "a formula's CNF". deduxit's statements
  are sequents, so the equivalent forms rewrite each formula of the
  sequent in its place, and a formula typed alone is `⊢ F`. The encodings
  answer validity, so they encode the negation `Γ, ¬Δ`; a formula's own
  clauses are those of `F ⊢`. Plain to read in both, and what 70b's
  refutation takes (departs from the design only in that the brief left
  the unit open).
- **The claims are decided by `Method::Sat` explicitly**, not the
  classical default, which with the feature `sat` also runs the embedding
  for a derivation: the default would have made each check an embedding
  search, and the brief names the SAT engine's checked proof.
- **No bindings call** (`ordinary_normalize`): the brief gives 70a the
  library, the command and the protocol; the bindings reach the five
  operations through the workspace calls already, and the stepper's call
  is 70c's.

### What the next sessions must know

- **70b (resolution and DPLL)**: `normal::rewrite` with `Form::Tseitin`
  or `PlaistedGreenbaum` gives a sequent `C₁, …, Cₙ ⊢` whose hypotheses
  are clauses (a disjunction of literals, left-nested, or a literal, or
  `⊥` for the empty clause), unsatisfiable exactly when the input is
  valid: the input of a resolution refutation. A CNF by distribution
  gives conjunctions per formula instead; split them at `∧` for clauses.
  An outside proof checked through 54s's `drup` needs an error of kind
  `invalid` (core-ordinary.md says so); `Refusal::Form` is no model for
  it.
- **70c (the page)**: the stepper reads `NormalForm`'s JSON (level 21):
  each step's `before` and `after` are indices of `result.formulas`, the
  sequent after step k is the step's `left` with the `added` clauses of
  steps 1 to k, then `⊢` and `right`. A call `ordinary_normalize(sequent,
  options, settings)` over `normal::normalize` with the tab's limits is
  the natural one; `about` would list it under the ordinary suite's
  `transformations`. The tab's `limits.form_occurrences` is the
  library's 10 000 unless the tab sets one.
- **The web module's size**: the five operations add the rewriting and
  its checks to every build of the bindings (the workspace links every
  operation); `nix flake check`'s `budget` measures it, below.
- **Quantifiers (step 74)**: `law` and `apply` take rows for `∀` and
  `∃` (prenex moves, `¬∀ ≡ ∃¬`), and the encodings need Skolem terms or
  a refusal there; `Node` gaining binders breaks every exhaustive match
  in `normal.rs` on purpose.

## Session 70b: resolution and DPLL

Opus 5.5 at `high`, the same worker and conversation as 70a, in the jj
workspace `../deduxit-wt-step-58a-610ed3`, from the brief
`start-70b.md` on 2026-11-20, cores 7 to 9; begun on `main@origin`
`krpxlpnw` (70a pushed) and rebased onto `qomlluyl` (the CI work)
without conflict.

### Outcome

Clauses of classical logic are refuted by resolution and by DPLL at
teaching scale, and every refutation is checked step by step. A clause
set comes from a sequent's negation (through 70a's checked normal forms)
or from a DIMACS file. A refutation is a graph of resolvents, built by
hand one step at a time or found by saturation within bounds of clauses
and steps; DPLL prints every step and its refutation is read off the
steps. An outside SAT solver's DRAT (text or binary) or LRAT proof is
checked by 54s's DRUP checker. In the library (`ordinary::resolution`),
the command (`resolve`, `check` of a refutation, `check --dimacs` of an
outside proof), the JSON at wire level 22, the protocol
(`ordinary.resolve`), a new manual chapter, the guide and the changelog.
The page is 70c's.

The chain on `main@origin` `qomlluyl`, the report last:

| change | what |
|---|---|
| `mplyqsmv` | the library: `ordinary::resolution` (clauses, the refutation and its checker, saturation, DPLL, outside proofs), the errors `InvalidRefutation` and `ClauseFile`, level 22 |
| `vswytytu` | the protocol: `ordinary.resolve`, the kind `ordinary.resolution` |
| `uyrtpsqr` | the command: `resolve`, `check --dimacs`, `check` of a refutation; the schema's level-22 forms |
| `omvlxlnl` | the behaviour lock: ten calls over four input files |
| `vwytpuum` | the manual's chapter, the protocol and status chapters, the guide, the changelog |
| `uyrwyptm` | the rules files |
| (this report) | `plan/reports/70-resolution.md` |

### What was built

- **Clause sets** (`Clauses`): atoms by name, clauses sorted and each
  literal once. `from_dimacs` (comments, the header, `%` as SATLIB ends
  files; the variable k is the atom `xk`); `of_sequent` (a sequent of
  clauses or conjunctions of clauses, nothing right of `⊢`);
  `clauses_of` (a sequent's negation by CNF, Tseitin or
  Plaisted–Greenbaum, the normal form's promise checked);
  `to_sequent` back. `Literal` is `2·atom + sign` in a `u32`.
- **The refutation** (`Refutation`): the set and its resolvents, each
  with the numbers of its two clauses (the set's from 0, then the
  resolvents), the pivot atom and the clause. It is the graph 70c draws:
  nodes the set's clauses and the resolvents, two edges into each
  resolvent. `resolve(first, second, atom)` is the step by hand (either
  order of the two clauses; refused with `StepRefused` where they do not
  clash on the atom), `is_finished` says whether `□` is derived,
  `trimmed` keeps what the first `□` needs.
- **The searches** (`decide`, `Options { method, clauses, steps }`,
  defaults 2 000 clauses and 100 000 steps): saturation, shortest clause
  first, tautologies and subsumed clauses left out, and when saturated
  DPLL for the model; DPLL, classic (no learning), units, then pure
  literals, then decisions, every step an `Event` (`unit`, `pure`,
  `decide`, `conflict`, `backtrack`), the refutation read off going back.
  `Verdict::Refuted`, `Satisfiable { true_atoms }` or `Unknown(Bound)`.
- **Options for every front end** (D15): `resolution::Options` (plain
  data with serde) is the call's input, as `negative::Options` is; the
  command maps `--method`, `--clause-limit`, `--step-limit` onto it and
  the protocol its parameters, so no settings key holds it.

### The checker

- **`Refutation::check`** is the trust: every resolvent's clauses come
  before it, the first holds the atom and the second its negation, the
  clause is the first without `p` joined with the second without `¬p`,
  and some clause is empty. A resolvent is a unit of `limits.work`, the
  stop asked once per resolvent. Its fault is `Error::InvalidRefutation`
  (kind invalid) with the step counted from 1.
- **`decide` checks what the searches found**: the refutation by
  `check` and against the input set, the model against every clause; a
  failure is `ClaimFails`, a defect. The searches are untrusted.
- **Outside proofs** (`check_proof`, feature `sat`): DRAT as text or
  binary (a binary proof starts with `a` or holds a control byte, as
  drat-trim tells them apart), LRAT as text with its numbering (hints
  ignored); every addition is checked by RUP in `drup::check`, deletions
  only weaken, so a misread proof can be refused but not wrongly
  accepted. A RAT step fails (no RAT check; CaDiCaL and Kissat write RUP
  proofs unless asked for more). No solver dependency: the checker is
  54s's.

### The command and the protocol

- **`resolve SEQUENT`** or **`resolve --dimacs FILE`**, `--method
  saturation|dpll`, `--clauses cnf|tseitin|plaisted-greenbaum`,
  `--clause-limit N`, `--step-limit N`: text with the verdict line, how a
  sequent's clauses were made, the clauses numbered from 1, DPLL's steps
  indented by their decisions, the resolvents with their parents and
  atom; JSON the level-22 document. Exit 0 unsatisfiable (a valid
  sequent), 1 satisfiable, 3 unknown, 2 error. It is a command of its
  own, as `interpolate` is, since it decides.
- **`check FILE`** reads a refutation's JSON (told by `resolvents`);
  **`check --dimacs FILE PROOF`** an outside proof (`--proof-format`, or
  the extension `.lrat`). Exit 0 holds, 1 does not, 3 at a limit; text
  only.
- **The protocol**: `ordinary.resolve`, a classical sequent to
  `ordinary.resolution` (the command's JSON), parameters `method`,
  `form`, `clauses`, `steps`, promise `decides`; its check makes the
  clauses again from the input, compares them, then checks the
  refutation through the level-22 reader or the model.
- **Wire level 22** (`wire::CLAUSES`): the refutation (read and
  written) and the clause set decided (written only), in the manual's
  schema (`clause_refutation`, `clause_resolution`, `resolvent`,
  `dpll_event`).

### The panel's and the reviewer's findings

**The panel** (step 26's three members, on the library's change before
the command was written): every finding is fixed in `mplyqsmv` and
pinned by a test.

- **Counterexamples** (`panel-counterexamples`, Opus 5.5 at high): one
  refutation of the checker's soundness, found also by the argument
  member on its own. The resolvent dropped every literal of the pivot's
  atom from both clauses, so a parent holding `p` and `¬p` lost both:
  `{a ∨ ¬a}` was "refuted" in one step, and `x1 ∨ x2, ¬x1 ∨ ¬x2, ¬x2`
  (satisfiable) by hand through the tautology `x2 ∨ ¬x2`; 15 of 60 000
  mutated documents were accepted for satisfiable sets. The rule now
  takes `p` from the first clause alone and `¬p` from the second
  (`a_tautology_is_no_way_to_the_empty_clause`). The searches were never
  wrong (saturation keeps no tautology, DPLL's parents never hold both),
  but their re-check rested on the same rule. Beside it: 90 000 runs of
  both searches, 120 000 DRAT proofs as text and binary and 120 000 LRAT
  proofs, all agreeing with its own brute force and checkers.
- **Argument** (`panel-argument`, Fable 5.2 at high): the same fault in
  the checker, with three witnesses, one without any tautology in the
  set and none resolving a clause with itself. Saturation's completeness
  (forward subsumption alone), DPLL's read-off (the clause carried false
  under the trail that remains) and the outside readers (a misread can
  lose a proof, never accept one) hold, each stated from the code. Two
  smaller points, both taken: DPLL ignored the memory bound (it charges
  its events and resolvents now), and `ClausalProof`'s docs described
  only the SAT solver's own proofs (now any set's).
- **Integers** (`panel-integers`, Sonnet 5.5 at high): `Literal::new`
  wrapped past 2³¹ atoms, through `resolve` (atom 2³¹ aliased atom 0 in a
  release build) and `Clauses::new`: `new` answers `None` from
  `MOST_ATOMS` on, the crate's `of` is used for atoms of a set only,
  `resolve` refuses an atom past the set's, and `of_sequent` counts the
  atoms first. A 32-byte DIMACS header of 50 million variables held 7 GB
  past a 1 GiB bound: `from_dimacs` refuses at the memory bound before it
  makes names, `Clauses::new` no longer copies them three times, and
  `decide` charges the set and its per-atom arrays first. LRAT was
  charged half its memory: each clause twice now. Every other limit it
  drove (headers to 2⁶⁴, literals at `i64::MIN`, binary varints of ten
  bytes, LRAT ids at `u64::MAX`, bounds 0 and 1) was refused or right.

**The fresh reviewer** (`fresh-reviewer`, Fable 5.2 at medium): 150
random DIMACS sets and about 60 sequents through every form and both
methods against a truth table, hand-written refutations (each fault
refused), DRAT text and binary, LRAT, standard input and time limits.
No wrong verdict. Its findings, all fixed:

1. **Two errors shared a code**: `InvalidRefutation` had the linear
   suite's `invalid_refutation`; it is `invalid_clause_refutation` now
   (the gate's `the_codes_are_listed` had caught it too).
2. **The step bound answered unknown within it**: saturation's model
   search counted against the bound its resolvents had used, DPLL tested
   the bound before it looked at the state (a trace of exactly N steps
   was unknown at N), and saturation reported a step past the bound. The
   bound is now tested before each step is made (a test pins a trace of
   six at the bounds six and five).
3. **"1 steps checked"**: the plural fixed.

Its questions, answered: `check` reads the four-key refutation, not
`resolve`'s answer (as documented: the answer is written only); a set
holding `□` with an empty DRAT proof is refused for want of an added
empty clause, as DRAT's format asks; and a bound or a stop that ends
`decide` (memory, work, a stop during the final check) is now unknown
with the flag to raise, exit 3, not an error.

### Checks

- **The tests**: the core crate's unit tests (536 and one ignored) and
  the command's suite (the lock, README's, the manual's and the guide's
  examples) passed after every round of fixes; the resolution tests
  (nine) pin the checker's refusals, the tautology witnesses, the trace,
  the bounds and their edge, DIMACS, the outside formats, and both
  searches against the SAT solver on 80 generated sequents (more than
  250 answers).
- **The gate** (capped, cores 7 to 9) passed on the chain before the
  reviewer's fixes, after one failure that the reviewer named too (the
  shared error code). After the rebase the head took the full check in
  its place, as CLAUDE.md now says.
- **`nix flake check`** under the machine lock, `--keep-going`, at the
  head on `qomlluyl`: all checks passed, `wire-schema` among them (97
  documents, the four of `resolve` included, 0 failures). Two earlier
  runs failed and were fixed in `mplyqsmv`: `treefmt` (a module line a
  script added unformatted) and `doc-private` (two links in private
  module docs). **The web module** is 3 721 760 bytes at gzip, under the
  3 750 000 ceiling; it was 3 690 645 after 70a, so resolution adds
  about 31 KB, and 28 KB are left.

### Departures

- **No `close` or interactive session for resolution in the command.**
  The brief's "built by hand step by step" is the library's
  `Refutation::resolve` (for 70c's page) and a refutation written as
  JSON, which `check` checks; a session of the command, like `interact`,
  was not asked for (departs from the design only in reading "by hand"
  as the library's step).
- **`resolve` is a command of its own**, as `interpolate` is, since it
  decides; and `check` takes the outside proofs and refutations, as it
  takes every proof.
- **The bounds are options of the call** (`--clause-limit`,
  `--step-limit`, the protocol's parameters), not settings keys, as
  63t's options are: they shape the question, not the machine's budget.
- **A RAT step is refused**: the checker has no RAT check, and the brief
  allows a checker of deduxit's own; CaDiCaL and Kissat write RUP proofs
  by default.

### What the next sessions must know

- **70c (the page)**: a call `ordinary_resolve(sequent, options,
  settings)` over `resolution::clauses_of` and `decide` answers the
  level-22 document (`resolvents` with `first`, `second`, `atom`,
  `clause`; DPLL's `trace`), which is the graph: nodes the clauses then
  the resolvents, edges `first → k` and `second → k`. For by-hand steps
  the page keeps a `Refutation` document (level 22, read and written)
  and a call over `Refutation::resolve` adds a step (refused with
  `step_refused` where the clauses do not clash on the atom); `check`
  and `is_finished` say when `□` is reached. The protocol's
  `ordinary.resolve` already runs in the workspace.
- **Teaching text**: the command's words (unit, pure, decide, conflict,
  backtrack; "from 3, 1 on a") are the manual's; the page can take them.
- **The module's ceiling**: 28 KB are left under 3 750 000 at gzip; the
  page's calls over `resolution` add little (the code is linked already
  through the protocol), but a raise of the ceiling would be a commit of
  its own that says what earned the room.
- **Later**: first-order resolution and the TPTP `cnf` reader with SZS
  output are in `plan/later.md`; RAT steps and a resolution session in
  the command would be additions there.

## Session 70c: the page

Opus 5.5 at `high`, the same worker and conversation as 70a and 70b,
from the brief `start-70c.md` on 2026-11-20, cores 7 to 9: deduxit in
the jj workspace `../deduxit-wt-step-58a-610ed3` on `main@origin`
`tkmmrwos` (70b pushed), the client in its own workspace
`../deduxit.github.io-70c` (workspace `step70c`) on its `main@origin`
`okwokwwv`, its deduxit input overridden by `git+file://` of this chain.

### Outcome

A started classical proof's Logic popover offers two more buttons under
the translation's and the interpolant's: **Rewrite into a normal form**
and **Refute by resolution**. Each opens a sheet over the stage with its
verdict first. The normal form is stepped through one law at a time. The
clauses of the sequent's negation are refuted by a search, traced by
DPLL, or by hand, and the refutation is listed and drawn as a graph,
the two in step and operable from the keyboard. A visitor who presses
neither sees nothing new.

deduxit's chain, on `main@origin` `tkmmrwos`:

| change | what |
|---|---|
| `omwlqkqk` | the library: `Refutation::check_steps`, a refutation in progress checked step by step |
| `sywtmkly` | the bindings: `ordinary_normalize`, `ordinary_resolve`, `ordinary_clauses`, `ordinary_resolve_step`, `ordinary_refutation_check`; `about`'s transformations; their test |
| `vqyvyllr` | the guide, the changelog, the web plan's table, the bindings' rules |
| `ormpkqwk` | the law of a definition's step: its clauses may be one direction only (Plaisted–Greenbaum), after the reviewer |
| (this report) | `plan/reports/70-resolution.md` |

The client's chain, on its `main@origin` `okwokwwv`:

| change | what |
|---|---|
| `ynqvywmw` | `src/lib/clausal.ts` (the forms, the steps in signs, the words, clauses, the atoms two clauses clash on, the graph's layout) and its tests, the API's and the worker's calls |
| `ponxukxv` | `src/islands/Clausal.tsx` (the two sheets and the graph), the popover's two buttons, the band's sheets, the styles, the client's `web.md` |
| `nxxlqsyv` | `tests/clausal.spec.ts`, the walk `clausal` in `tests/states.ts`, its axe and target-size tests |

### The bindings

- `ordinary_normalize(sequent, options, settings)` and
  `ordinary_resolve(sequent, options, settings)` answer 70a's level-21
  and 70b's level-22 documents; `options` are `{"form"}` and `{"form",
  "method", "clauses", "steps"}`, `""` for CNF and saturation within the
  library's bounds. `about` lists them under the ordinary suite's
  `transformations` (`normal` with its `forms`, `resolution` with its
  `methods` and `clauses`), and the tab's `settings` name
  `limits.form_occurrences`.
- **A refutation by hand**: `ordinary_clauses` starts one (the level-22
  refutation with no resolvent); `ordinary_resolve_step(refutation,
  first, second, atom, settings)` checks every step the page sent and
  adds the resolvent, answering `{"refutation", "added", "finished"}`;
  `ordinary_refutation_check` checks one. The check of a refutation in
  progress is the library's (`Refutation::check_steps`, every resolvent
  checked, no `□` asked), since a front end builds no error of the
  library's (core.md).
- **The module's size**: 3 741 399 bytes at gzip, about 20 KB for these
  calls (3 721 760 after 70b), 8.6 KB under the 3 750 000 ceiling; no
  raise was needed.

### The page

Designed with the education skills' cognitive-load analysis and
worked-example fading, and the frontend-design plugin's rule of one
large thing per sheet.

- **Where they are asked for**: the Logic popover of a started
  classical proof, under the two transformations of 63t-b, as sheets
  over the stage (light dismiss, Escape, the title focused). Not a view
  tab: the stage's row of views is full on a phone, and the author's
  rule (not overwhelming) wants nothing new for a visitor who does not
  ask.
- **The normal form**: CNF first (a first course's), then NNF, DNF,
  Tseitin and Plaisted–Greenbaum, each with a line of what it is. The
  check's keyword first ("Equivalent", "Same verdict", "Not checked"),
  the result large, then a stepper from the sequent as typed through
  each step: the law in the formulas' font, where it acts, and the
  sequent after it. One new thing per step; the last step's sequent is
  the result (a unit test holds it for every form).
- **Resolution**: three ways, the worked example first: Search (a
  refutation found), DPLL (its steps, indented by their decisions, in
  words with the literals in the formulas' font, then the refutation
  read off), and By hand (choose two clauses; the atoms they clash on
  are buttons "Resolve on p"; deduxit adds the resolvent; Undo takes the
  last back; "You derived □" when deduxit says the refutation is
  finished). A pair that clashes on no atom is said in the status. A
  sequent that is not valid says the model of its clauses in words.
- **The graph view** (web-front-end-6): the given clauses on the top
  row, each resolvent one row below its deeper parent, an edge from each
  of its two clauses; the edges an `aria-hidden` SVG, each clause an
  HTML button over it (44 by 44 pixels, a phone's target), placed
  through the CSSOM (no `style` attribute, the CSP's rule). The list and
  the graph share one choice: a clause chosen in either is pressed in
  both, and the two edges into a chosen resolvent are marked. Tab and
  Enter or Space operate both; the graph scrolls sideways where it is
  wider than the sheet.
- **The look**: no new colour or font; the controls are the page's
  (segmented controls, small buttons, the `kw` keywords), every value a
  token (`tokens.test.ts`).

### The reviewer's findings

`fresh-reviewer` once, on Fable 5.2 at medium, with the page in a
browser (the built site served, the sheets driven by Playwright). Four
findings, all fixed in the chains above:

1. **"You derived □" for a set that holds □ already.** `|- true` gives
   the empty clause among the given ones; the page claimed a derivation
   deduxit never made. The page now says so from deduxit's answer alone:
   "You derived □" only when the last step's answer says `finished`,
   reset by Undo, and a set with a given □ has its own sentence. An e2e
   test covers it.
2. **The Plaisted–Greenbaum law overclaimed.** The definition step's law
   said "the clauses of t ↔ A" where the encoding adds one direction
   only. The library's law now says so (`ormpkqwk`), and so does the
   sheet's opening line.
3. **A long clause overflowed its node.** The graph's node now cuts its
   text with an ellipsis; the full clause stays in the button's name and
   in the list beside it.
4. **The focus fell to the page after a step or an Undo** (both buttons
   leave the DOM). The focus now goes to the new clause's list button
   after a step and to Undo after an undo (the first clause when nothing
   is left to undo); the by-hand e2e test holds both.

It also noted that `ordinary_refutation_check` is bound and tested but
the page does not call it: each step already checks the whole
refutation, so a separate check would repeat it. Kept for a front end
that stores a refutation and reads it back.

### Checks

All at the heads above, after the reviewer's fixes, under the machine
lock on cores 7 to 9.

- **deduxit**: `nix flake check --keep-going` at the head: every check
  passed, the gate's five parts with it (so no separate gate), the
  bindings' tests under Node (`web-wasm-tests`, the new test
  `a_sequent_is_normalized_and_resolved` among them), `budget` (the
  module at 3 741 399 bytes), `conventions` and `doc-private`. A first
  run had failed `conventions` alone, on a doc comment of
  `web/src/ordinary.rs` that read like a plan's step numbers; reworded
  in `sywtmkly`.
- **The client**: `nix flake check --keep-going --override-input
  deduxit git+file://` of this chain: every check passed, among them
  `unit` (712 tests in 33 files, 8 of them `clausal.test.ts`'s against
  the real module), `e2e` (354 tests, all passed: `clausal.spec.ts`'s
  six, and the walk `clausal` under axe and at the 24- and 44-pixel
  targets), `lint`, `typecheck`, `no-style-attributes`, `csp` and
  `licences`.

### Departures

- The clauses on the page are CNF's alone, with no choice of Tseitin or
  Plaisted–Greenbaum for resolution: the binding takes `clauses`, the
  page does not offer it, to keep the sheet to one choice (the way).
  (departs from the design)
- The sheets are in the Logic popover, not views beside the derivation
  (see "The page"). (departs from the design)
- The graph scrolls sideways where it is wider than the sheet; it is
  not scaled down, since a scaled node would fall below the targets'
  sizes. (departs from the design)
- `ordinary_refutation_check` is bound, not called by the page (see the
  reviewer's findings).

### What the next sessions must know

For 73, the release of 0.7.0:

- The client's chain was built and checked against this deduxit chain
  through `--override-input deduxit git+file://…`. Its `flake.lock`
  still pins the deduxit before 70c: after the push, move the client's
  input (`nix flake update deduxit`, a change of its own) before the
  client's chain lands, or its build lacks the five calls and its unit
  and e2e tests fail.
- The module is 3 741 399 bytes at gzip, 8.6 KB under the 3 750 000
  ceiling. A further binding of any size will need the ceiling raised,
  with its reason, in a commit of its own.
- The changelog's entries for the five calls and
  `Refutation::check_steps` are under "Unreleased".
- The level-22 refutation is read and written by the page; a change of
  its form is a change of the client's `src/lib/clausal.ts` too.
