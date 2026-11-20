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
