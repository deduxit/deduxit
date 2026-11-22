# Changelog

Every release of the crates `deduxit` (the library) and `deduxit-cli`
(the `deduxit` command), called `linlog` and `linlog-cli` (the `linlog`
command) up to 0.2.0, newest first. The form is that of
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the
versions follow [Semantic Versioning](https://semver.org/) as Cargo reads
it for versions below 1.0: a change that breaks a caller raises the
second number (0.1.x to 0.2.0), anything else the third.

## [Unreleased]

### What breaks, and what to do instead

- **`ordinary::Node` has two variants more**, `Forall` and `Exists`, for
  the quantifiers of first-order formulas. A `match` over `Node` needs an
  arm for each.

- **`ordinary::Rule` has four variants more**, `ForallLeft`,
  `ForallRight`, `ExistsLeft` and `ExistsRight`, after `Open`.

- **`ordinary::Outcome` has the field `structure`**, a finite structure
  that refutes a first-order sequent. Struct expressions of `Outcome`
  were refused already (`#[non_exhaustive]`); a pattern that names every
  field needs `..`.

### Added

- **`ordinary::tptp_text`** gives a TPTP file's text from its bytes for
  `read_tptp`: a byte that is not UTF-8 in a `%` comment, where some of
  ILTP's problems write an author's name in Latin-1, is read, and one
  anywhere else is `Error::Tptp` naming its line.
- **Normal forms at first order**: `seq normalize --to prenex`,
  `miniscope`, `skolem`, `herbrand` and `clausal`, and `--to nnf` at first
  order, every step shown; the protocol's `ordinary.prenex`,
  `ordinary.miniscope`, `ordinary.skolem`, `ordinary.herbrand` and
  `ordinary.clausal`. Each equivalence step's law is checked by its own
  derivation, of LJ where the law holds in intuitionistic logic
  (`normal::Rule::intuitionistic`, `normal::Claim::Law`); each Skolem or
  Herbrand step's symbol is checked new and applied to exactly the
  variables free in its quantifier (`normal::Step::term`), and the
  result is checked to imply the input's negation (Skolem, clausal) or
  to follow from the input (Herbrand, `normal::Promise::Equivalid`).
- **Herbrand's theorem**: `deduxit herbrand SEQUENT` makes the clausal
  form of a first-order sequent, puts closed terms for its variables up
  to `--depth`, and has the SAT solver refute the ground instances; the
  instances it needs are the certificate, and a derivation of LK of the
  sequent as written, each Skolem term an eigenvariable again, is built
  from them and checked. The library's `ordinary::herbrand`, the
  protocol's `ordinary.instances` and the wire form at level 28.
- **Interpolants at first order**: `interpolate` takes first-order
  sequents in all three logics. A term of one half alone is bound in the
  interpolant by `∃` (the first half's) or `∀` (the second's), and the
  check holds the interpolant's predicates, functions and constants to
  both halves.
- **The negative translations at first order**: `seq translate --by
  kolmogorov`, `godel-gentzen` and `kuroda` take first-order sequents
  (Kuroda's puts `¬¬` after every `∀`, Gödel and Gentzen's writes
  `∃x. A` as `¬∀x. ¬A`), and `compare` checks the verdict kept as in
  propositional logic. Glivenko's translation refuses them, since its
  theorem fails at first order.
- **The quantifier rules of LK, LJ and LM**: the checker takes `∀L`,
  `∀R`, `∃L` and `∃R` with their eigenvariable conditions, each inference
  recording its witness or eigenvariable (`Inference::term`).
- **First-order proofs step by step** in `ordinary::Interactive` and
  `deduxit interact --logic`: `∀R` and `∃L` name a fresh eigenvariable,
  `∀L` and `∃R` take a witness term read by `Interactive::witness`
  (`apply 0 R0 existsR c`); `Step::with_witness`, `Needs::Witness`,
  `StepError::NoWitness` and `StepError::Witness` are new.
- **Open witnesses in the ordinary session**: `∀L` and `∃R` with no
  witness leave it open (`$1`, `$2`, …), an axiom fills it by
  unification, `undo` takes the filling back, and a session document
  keeps the bindings (`"bindings"`) and is checked under them when read.
  A witness still open in a finished proof is a fresh constant, so
  `∀x. p(x) ⊢ ∃x. p(x)` proves with no constant in the sequent.
  `Interactive::bindings`, `StepError::Binding` and `StepError::Terms`
  are new; `deduxit interact --logic` prints `filled $1 with TERM`. A
  step whose instance or filling would make the proof's terms pass
  `limits.terms` is refused, so a session writes no document its reader
  refuses.
- **First-order derivations drawn and read**: LaTeX, Typst and SVG draw
  them, the web bindings' ordinary step takes `"witness"`, and a TPTP
  problem without a conjecture reads as its hypotheses' refutation, a
  `lemma` or a `theorem` as a hypothesis. The flake's package
  `iltp-first-order` fetches ILTP's 2 670 first-order problems, which
  `deduxit-bench run --tptp` reads, includes inlined.
- **`check --tree never` writes no tree of an ordinary derivation**, as
  `prove` already did; a deep derivation's check no longer spends its
  time writing a tree nobody sees.
- **`include` in the command's TPTP problems**: `include('NAME').` is
  read from `$TPTP`, else the problem's directory or one above it; a
  missing file and an include of itself are refused by name.
- **TPTP names renamed on reading**: a name this syntax keeps for itself
  (`par`, `true`) gets `_` after it, and a function symbol named like a
  predicate `_f`, so that the sequent reads back.
- **First-order formulas in ordinary logic**: predicates applied to
  terms and the quantifiers, `forall x. p(x) -> q(x), p(a) |- q(a)` in
  the native syntax and `! [X] : (p(X) => q(X))` in TPTP's. Formulas
  equal up to the names of their bound variables are one formula.
- **First-order sequents decided through the embedding**:
  `ordinary::decide` and `prove --logic` take the embedding, the default
  method at first order (`Options::method_for`). A valid sequent gets a
  checked derivation of LK, LJ or LM, its quantifier rules read back
  from the linear proof's instances; one whose image is unprovable is
  unknown with the new reason `Reason::Incomplete` (exit 3), since the
  translation may lose first-order proofs. `Formulas::is_first_order` is
  public.
- **A connection prover of classical logic** (`ordinary::connection`,
  `Method::Connection`, `prove --logic classical --method connection`):
  Otten's non-clausal connection calculus, nanoCoP's, on the formulas'
  own structure, with regularity, lemmata and a bound on the active path
  that it deepens. It decides classical sequents, first-order ones
  included, and is the default for a first-order classical sequent, the
  embedding deciding where it gives up at a bound. A valid sequent gets
  a derivation of LK built from the proof's substitution and checked;
  one where no quantifier needs a witness term (every `∀` one to prove,
  every `∃` one to assume, negations and implications taken into
  account) is refuted by an exhaustive search; the others are unknown at the
  bound, the new reason `Reason::PathLength` (`--path-length N|none`,
  `search::Options::path_length`, by default 8 in the library and none
  in the command), or `Reason::Incomplete`.
  `search::Options::restricted_backtracking`
  (`--restricted-backtracking`) keeps each literal's first solution,
  never answering "not valid". `deduxit-bench run --engines connection`
  runs it.
- **The connection prover decides intuitionistic logic** (`prove --logic
  intuitionistic --method connection`): nanoCoP-i's prefixed calculus,
  in which a classical connection proof is an intuitionistic one when its
  prefixes unify (T-string unification) and its eigenvariables meet the
  domain condition. "Valid" comes with a derivation of LJ, found by a
  search the proof guides and checked; otherwise the answer is unknown,
  even without quantifiers: the search can miss a proof and never shows
  that none exists. The default
  of intuitionistic logic does not change.
- **Grounding** (`ordinary::grounding`, `Method::Grounding`, `prove
  --logic … --method grounding`, behind the feature `sat`): the
  Bernays–Schönfinkel class (no function symbols, no `∀` under an `∃`
  after the quantifiers are moved inward) and the monadic class (no
  function symbols, predicates of at most one argument) decided by
  grounding over the constants and the SAT solver, in every logic. A
  valid classical sequent gets the solver's checked clausal proof, by
  default the connection prover's derivation of LK where it finds one
  within `EVIDENCE_WORK`; a sequent that is not valid gets a finite
  structure (`ordinary::Structure`, `FiniteCountermodel`), checked by
  evaluating the sequent in it (`Structure::refutes`,
  `FiniteCountermodel::check`) and printed after the verdict line. It is
  the default for the sequents of both classes; outside classical logic
  a classically valid one, and in every logic a sequent outside the
  classes, goes to the next method. `check` evaluates an outcome's
  structure again, and the new `Error::Structure` (code
  `invalid_structure`, kind invalid) says what a wrong one gets wrong.
  `deduxit-bench run --engines grounding` runs it.
- **The command shares its time limit at first order**: where the
  default of a first-order sequent is grounding or the connection
  prover, it gets 90% of the time limit and the embedding the rest.
- **A search for finite models** (`ordinary::models`, `Method::Models`,
  `prove --logic … --method models`, behind the feature `sat`): for 1,
  2, … elements in turn, it writes the sequent's falsity in a structure
  of that size as a propositional formula and asks the SAT solver for a
  model, after MACE and Paradox (both quantifiers expanded over the
  domain, each function symbol's values as atoms that must give each
  argument tuple one value, the constants' symmetries broken). A
  structure it finds is checked against the sequent and refutes it in
  every logic; it never answers "valid", and a sequent no structure
  within the bound refutes is unknown with the new reason
  `Reason::ModelSize` (`search::Options::model_size`, `--model-size
  N|none`, by default 6 in the library and none in the command). The
  default runs it too: the command beside the connection prover in
  classical logic, on a thread of its own, the first to answer winning
  (on one thread, after the prover's 70% of the time limit); the library
  last, on a first-order sequent the embedding leaves unknown, in every
  logic. On
  ILTP's first-order problems it refutes 210 classically at 2 s, among
  them all of grounding's 77 and nanoCoP's 84. `deduxit-bench run
  --engines models` runs it.
- **Kripke models with growing domains** (`ordinary::KripkeStructure`,
  `KripkeWorld`, `KripkeCountermodel`, `Outcome::kripke`): in
  intuitionistic and minimal logic the model search also tries Kripke
  models of 1, 2, … worlds with domains growing along the order, each a
  propositional sequent for the SAT solver (the order, the domains, the
  predicates and `⊥` growing, the function tables agreeing, forcing by
  its definition), and checks the model it finds by the forcing relation
  before "not valid"; past both bounds the answer is unknown with the new
  reason `Reason::KripkeSize` (`search::Options::model_worlds`,
  `--model-worlds N|none`, by default 4 in the library and none in the
  command). The library's default asks it after the embedding, the
  command beside it on a thread of its own, which takes one of the
  command's threads. In intuitionistic logic a finite model never
  refutes `¬¬A` for a classically valid `A`, so Glivenko's failure at
  first order stays unknown.
- **Finite structures interpret function symbols**
  (`Structure::functions`, `Structure::with_functions`): each function
  symbol of one argument or more with its table, the values of its
  argument tuples in lexicographic order, printed after the constants
  (`f: 0 ↦ 1, 1 ↦ 0`).
- **Otten's connection provers in the comparison**: the flake fetches
  leanCoP 2.1, nanoCoP 2.0, nanoCoP-i 2.0 and iLeanCoP 1.2 at their
  released archives and runs them on SWI-Prolog (`prover-leancop`,
  `prover-nanocop`, `prover-nanocopi`, `prover-ileancop`);
  `deduxit-bench run --tools leancop,nanocop,nanocopi,ileancop` runs
  them on TPTP problems.
- **`Error::FirstOrder`** (code `first_order`, kind unsupported): a
  method that decides or transforms propositional sequents (the SAT
  solver, LJT, the truth table, the tableau, the normal
  forms, resolution, the interpolants, the negative translations, the
  Kripke model's check, the goal state's `close` and `hint`, the
  certificate over `Prop`, the lambda-term) refuses a first-order sequent
  by name.

### Changed

- **`--derivation-limit` bounds what is drawn, not what is built, under
  `--logic`**: a derivation of LK or LJ is built and checked within the
  memory bound whatever its size, so `-q` and `--format json` give the
  checked "valid" where the drawing would be too large (ILTP's SYJ202,
  Cook's pigeonhole formulas, at sizes 4 to 6), and the drawing past the
  limit is left out with its verdict kept. The tableau method still
  builds its tableau within the limit. The default of intuitionistic
  logic keeps LJT's derivation where its drawing fits the limit, and the
  SAT solver's otherwise. In the library, every reader back of an
  ordinary derivation (the embedding's, LJT's, the SAT solver's, the
  connection provers') is bounded by `limits.memory_bytes`, the
  protocol's `ordinary.draw`, `ordinary.export` and
  `ordinary.certificate` refuse a drawing past
  `limits.derivation_bytes`, and `ordinary::Derivation::drawn_bytes`
  estimates a drawing by the measure that bound takes, for a caller that
  draws a decided derivation itself.
- **A bound or the stop that ends the read-back or a check after the
  search decided is an unknown, not an error**: `ordinary::decide` and
  each method's `decide` (LJT, the SAT solver, the connection provers,
  the model checks of the truth table and the tableau) answer
  `Ok(Unknown(reason))` with the search's statistics, as a bound that
  ends the search does, so `prove --format json` writes that outcome
  (before, nothing on standard output) and a named method's bound there
  is exit status 3, not 2. A defect stays an error; so do the
  embedding's read-back in `ordinary::decide` and a truth table or
  tableau that a bound ends while it is built.
- The default of intuitionistic and minimal logic gives LJT at most
  16 384 sequents to find the derivation it prefers to the SAT solver's,
  so a sequent the solver decides at once is answered at once: ILTP's
  SYJ202+1.005 in 0.05 s, where LJT's search took the whole time limit.
- Where the default of intuitionistic or minimal logic stops at a bound
  and the embedding decides, `prove`'s verdict line says which bound
  stopped it.
- The modal search in K4, KD4 and S4 makes its worlds in depth and keeps
  what it learns about a world across branches: a world made again under
  the same conditions reuses the earlier world's part of the model, or
  closes its branch at once where the earlier one could not be satisfied.
  A sequent of KD4 that took 20.5 million units of work now takes about
  58 000. Every "valid" still comes with a checked derivation, every
  countermodel is checked as before.
- The web package's PNG and PDF renderers are a module of their own,
  `deduxit_web_render_bg.wasm` (the crate `deduxit-web-render`), with
  the one call `render`, which left the bindings' module. The page
  loads it on a visitor's first PNG or PDF export, so the module every
  visitor loads is 2.4 MB at gzip, down from 3.9 MB.

### Fixed

- `ordinary::Derivation::term` answers `Error::ReadBack` for a derivation
  its read-back cannot read, where it panicked; the crate makes no such
  derivation, and the checker refuses one read from JSON.
- An unknown verdict at the memory bound names the bound given: the Horn
  engine's said "the memory limit of 0 B" under `--memory-limit 1MiB`.
- `prove --logic` reads a TPTP problem whose comments hold bytes that
  are not UTF-8: 107 of ILTP's first-order problems write an author's
  name in Latin-1, and the command refused them as not UTF-8 text. Such
  a byte in a formula is refused with its line.
- `Sequent::add` and `add_cut` on a JSON sequent with quantifiers over
  nullary atoms only, joined with a first-order sequent, gave its atoms
  the other sequent's names: `⊢ ∀x. (a ⅋ ~b)` with `⊢ p(c), q` added
  printed `⊢ ∀x. (p(c) ⅋ ~q), p(c), q`.
- The Rocq certificate of an LJ derivation read from JSON that uses
  LK's one-premise `¬L` (`Γ, ¬A ⊢` from `Γ ⊢ A`, which the checker
  takes) was a term Rocq refused. No search makes that form.

### Wire forms

- **Level 25**: a first-order ordinary sequent or derivation, its
  formulas with `{"∀": a}` and `{"∃": a}`, and its tables under the
  linear sequent's keys `symbols`, `fo`, the atoms as `{"P": [symbol,
  [terms]]}` and `binders`. A propositional one stays at level 8. A
  derivation's quantifier rule names its term by `"term": k`, an
  eigenvariable among the terms being `{"E": n}`. An ordinary outcome
  with a first-order derivation, or unknown for the new reason
  `{"kind": "incomplete"}`, is written at level 25 too, and so is one
  decided by the connection prover (`"search": {"method": "connection",
  "statistics": {…}}`) or unknown at its bound (`{"kind": "path_length",
  "length": 8}`). The search options have the keys `path_length` and
  `restricted_backtracking`. One decided by grounding is written at level
  25 (`"search": {"method": "grounding", "statistics": {"domain",
  "atoms", "nodes", "sat"}}`), and a refutation by it holds
  `"structure": {"sequent": …, "structure": {"size", "constants",
  "predicates"}}`.
- **Level 26**: an ordinary outcome of the model search (`"search":
  {"method": "models", "statistics": {"size", "atoms", "nodes",
  "sat"}}`), one unknown at its bound (`{"kind": "model_size", "size":
  6}`), and one whose structure has function tables (`"functions":
  [["f", [1, 0]]]`, left out where there are none). The search options
  have the key `model_size`.
- **Level 27**: an ordinary outcome refuted by a Kripke model with
  growing domains (`"kripke_structure": {"sequent": …, "model":
  {"worlds": […], "constants": […], "world": 0}}`), or unknown at its
  bounds (`{"kind": "kripke_size", "worlds": 4, "size": 6}`); the model
  search's statistics have `worlds` where it tried a Kripke model. The
  search options have the key `model_worlds`.

## [0.7.0] - 2026-11-20

Resolution and the simply typed lambda calculus. deduxit rewrites a
classical sequent into negation, conjunctive or disjunctive normal form,
or into clauses by Tseitin's or Plaisted and Greenbaum's encoding, one
law per step, and checks the result with its SAT solver. It refutes the
clauses of a sequent's negation by resolution or by DPLL, checks every
refutation step by step and every model clause by clause, and checks an
outside SAT solver's DRAT or LRAT proof. `term check` reads a term of
the simply typed, the linear or the affine lambda calculus and checks
its type; `term inhabit` finds a term of a type, read back from a
derivation and checked, or shows that it has none; `term enumerate`
lists and counts a type's terms up to βη-equality. A well-typed term's
typing derivation is drawn as a tree in every export. `grade` grades an
answer to a problem in one call, and a dataset of 843 problems with
verified answers comes with it, for evaluating models. On the web page,
a classical proof's normal forms are stepped through one law at a time,
its clauses are refuted by resolution, as a list and as a graph, by
DPLL or by hand; a lambda-term is checked and its typing derivation
drawn, and a type's terms are found and counted; and the switch of
invertible rules acts in LK and LJ tabs too.

### What breaks, and what to do instead

- **A variable of a `lambda::Term` has a name**, an identifier, where it
  had a letter and an index: `lambda::Variable::name` replaces `letter`
  and `index`, and `Variable` is no longer `Copy` (clone it, or keep its
  `VarId`). In JSON a variable is `{"type": 0, "name": "x"}` where it
  was `{"type": 0, "letter": "x", "index": 0}`. Digits at a name's end
  still print as an index (`x1` is `x₁`).

Nothing else that 0.6.0 offers was removed or changed: its other calls,
types, command lines, error codes and JSON forms are as they were. Every
new variant or field belongs to a `#[non_exhaustive]` type, so a
program's patterns go on compiling. What a program may notice is under
"Changed".

### Changed

- With `--lambda-types` (`lambda.types`), `inl` and `inr` now show the
  sum type they give, as `false` and `abort` show theirs, and a
  subscript that is no atom or constant is in brackets:
  `inl_(a ⊕ b)(x)`, `abort_(a → b)(z)`. A term printed so reads back.
  In SVG a subscript and a variable's index are set lowered, where
  `_` and subscript digits, which the math fonts lack, were written.
- The value `--lambda` adds to an outcome in JSON has the key `version`
  (23), as the term's own document.
- `protocol::suites()` lists five suites: `lambda` joins them.
- The help of `check` names `--dimacs` and `--proof-format`, and the
  list of commands names `resolve`, `grade` and `term`.

### Added

- Normal forms of classical sequents, every step shown and the result
  checked: `seq normalize --to nnf|cnf|dnf|tseitin|plaisted-greenbaum`,
  CNF by default. Negation, conjunctive and disjunctive normal form are
  made by equivalence laws, one law per step; Tseitin's and Plaisted and
  Greenbaum's encodings give the clauses of the sequent's negation, a new
  atom for each compound subformula. The SAT solver checks that each
  formula is equivalent to its form, or that the clauses can be
  satisfied exactly when the sequent can be false. `--form-limit`
  (`limits.form_occurrences`, 10000 by default) bounds the form.
- In the library, `ordinary::normal` (`rewrite`, `normalize`,
  `NormalForm` with its `steps` and `checks`), `Limits::form_occurrences`
  with `Refusal::Form` (code `form_too_large`) and `Phase::Rewrite`.
- In the protocol, the operations `ordinary.nnf`, `ordinary.cnf`,
  `ordinary.dnf`, `ordinary.tseitin` and `ordinary.plaisted_greenbaum`,
  with the promises `equivalent` and `equisatisfiable`, so a workspace
  takes a normal form as an edge.
- Resolution and DPLL at teaching scale: `resolve SEQUENT` turns the
  sequent's negation into clauses (`--clauses cnf|tseitin|plaisted-greenbaum`),
  or `resolve --dimacs FILE` reads them, and refutes them by saturation
  or with `--method dpll`, which prints every step; `--clause-limit` and
  `--step-limit` bound the search. Every refutation is checked step by
  step and every model clause by clause. `check` checks a refutation in
  JSON, and `check --dimacs FILE PROOF` an outside SAT solver's DRAT or
  LRAT proof.
- In the library, `ordinary::resolution`: `Clauses`, `Refutation` (built
  by hand with `resolve`, checked with `check`), `decide`, `clauses_of`
  and, with `sat`, `check_proof`; the errors `InvalidRefutation` (code
  `invalid_clause_refutation`) and `ClauseFile` (code `clause_file`). In the
  protocol, the operation `ordinary.resolve` and the kind
  `ordinary.resolution`.
- The library type-checks lambda-terms. `lambda::Term::parse` reads a
  term of the simply typed, the linear or the affine lambda calculus
  from text, in Church's style (every `λ` with its variable's type), as
  a term, a term with its type or a whole judgement `x : a ⊢ M : b`.
  `lambda::Term::check` checks it and returns the type of every subterm
  (`lambda::Typing`): the types fit, every variable is used as the
  calculus allows, and the term has the type claimed. A term that is
  not well typed is refused with `Error::IllTyped` (code `ill_typed`),
  which says why and where. The terms read back from proofs are checked
  the same way.
- A lambda-term is a JSON document of its own, at wire level 23, which
  `wire::upgrade` reads.
- `deduxit term check TERM` checks a lambda-term and prints its
  judgement with every type, or why it is ill typed (exit 1);
  `--calculus` picks the simple, the linear or the affine calculus.
- Inhabitation of simple types. `lambda::inhabit` (`deduxit term
  inhabit`) decides whether a type, an ordinary formula, has a term, and
  gives the term read back from the derivation and checked, or says that
  it has none. `lambda::enumerate` (`deduxit term enumerate`) lists a
  type's terms up to βη-equality, as long normal forms, and counts them:
  exactly, infinitely many, or at least as many as a search within
  `lambda::InhabitOptions` met; every term listed is checked. It counts
  the types built from atoms, `→`, `∧`, `↔` and `⊤`, and refuses one with
  `∨` or `⊥` with `Error::NotEnumerable` (code `not_enumerable`).
- The protocol has a suite `lambda`: `lambda.check` (a term to its
  typing, promise `well_typed`), `lambda.inhabit` (an intuitionistic
  statement to `lambda.inhabitation`, promise `decides`),
  `lambda.enumerate` (to `lambda.inhabitants`, promise `well_typed` of
  every term) and `lambda.duplicate`; `protocol::statement("lambda", …)`
  makes a term's statement from its text.
- `lambda::Calculus` has its names (`ALL`, `NAMES`, `name`, `FromStr`).
- A well-typed lambda-term's typing derivation is drawn in every export:
  `lambda::Drawing::new` checks the term and lays its derivation out,
  each inference a judgement `Γ ⊢ M : A` under the rule of natural
  deduction its term stands for, and LaTeX, Typst and SVG take it as
  they take a proof (`export::Drawable::Lambda`). In the linear calculi
  each premise's context holds the variables it uses. Rocq refuses it
  with `rocq::Unsupported::Typing`. The protocol's `lambda.draw` and
  `lambda.export` draw a checked term's derivation or write it in LaTeX
  or Typst.
- `deduxit grade SEQUENT` grades an answer to a problem, read from
  standard input or `--answer FILE`, by its shape: a session script that
  must finish the proof by its own steps (`close` and `invertible`
  refused), a proof or an outcome as
  `prove --format json` writes it, a Kripke model that refutes the
  sequent, a signed tableau, or a derivation in a calculus given as
  data. The answer must be about the problem's sequent, in its logic.
  The exit status is 0 for correct, 1 for incorrect, 3 for unknown;
  `--format json` writes the verdict as one object.
- A dataset of problems with verified answers for evaluating models,
  `dataset/problems.jsonl.gz`: 843 sequents of linear, ordinary and
  modal logic, each with the verdict deduxit decided and the answer
  `grade` checked, in the fields evaluation harnesses read (`id`,
  `input`, `target`, `metadata`), with its card, Croissant metadata and
  the script that builds it. `deduxit-bench instances` prints the
  generated families' instances it reads.
- In the web bindings, `ordinary_normalize`, `ordinary_resolve`, and a
  refutation by hand: `ordinary_clauses`, `ordinary_resolve_step` and
  `ordinary_refutation_check`; in the library `Refutation::check_steps`,
  which checks a refutation in progress. The lambda calls read and check
  a term (`lambda_read`, `lambda_check`), draw and export its derivation
  (`lambda_draw`, `lambda_export`), read a type and find, list and count
  its terms (`lambda_read_type`, `lambda_inhabit`, `lambda_enumerate`).

### Wire forms

- Level 21 is a classical sequent's normal form with its steps and the
  claims of its promise decided (`seq normalize --format json`), written
  only.
- Level 22 is a refutation by resolution, read and written, and a
  clause set decided (`resolve --format json`), written only.
- Level 23 is a lambda-term (`term check --format json`, and the value
  `--lambda` adds to an outcome), read and written.
- Level 24 is a type's terms listed and counted, and whether a type has
  a term (`term enumerate` and `term inhabit --format json`), written
  only.
- `wire::LEVEL` is 24. Every document of levels 1 to 20 reads and is
  written as before; 0.6.0 refuses levels 21 to 24 by name.

## [0.6.0] - 2026-11-20

Modal logic, and tools for teachers. deduxit decides a sequent of
propositional modal logic in any of the fifteen normal systems from K
to S5: a valid one with a checked derivation in the system's labelled
sequent calculus, one that is not valid with a checked Kripke
countermodel. Both become Rocq certificates for rocq-deduxit 0.6.0,
whose `Deduxit.Modal` proves the labelled calculus sound. A labelled
sequent can be proved step by step, with forward steps, and a Kripke
model you write is checked against a sequent. `check --tableau` checks a
signed tableau written by hand and names its first wrong step. On the
web page, the modal tab decides, draws and proves step by step; a model
editor changes or builds a countermodel and checks it; a tableau editor
checks a tableau as it is written. In LK and LJ, a session proving
step by step can apply the invertible rules by itself. A teacher hands
out a course pack of problems as a link, reads the students' hand-ins
in a check view, embeds a tab in another page and prints a handout.

### What breaks, and what to do instead

Nothing that 0.5.0 offers was removed or changed: its calls, types,
command lines, error codes and JSON forms are as they were. Every new
variant or field belongs to a `#[non_exhaustive]` type, and the new
`rocq::Unsupported::Assumption` comes after the other variants, so
their numbers stay. What a program may notice is under "Changed".

### Changed

- **A name a rule asks to be new is new to the rule's other variables
  too**: a derivation or a session in a labelled modal calculus no
  longer takes `Ser` with one new label at both ends of its edge
  (`w1 R w1`), which made "a new world that sees itself". The rule
  file's `fresh` says so.
- **`check --format json` names the modal system** under the new key
  `system`, for a modal outcome or a model checked under `--system`.
- **Rule labels with `□` or `◇`** are set as symbols in every export:
  `\Box` and `\Diamond` in LaTeX.
- **The help of `check`** names the new flags `--system` and
  `--tableau`, and says that `--sequent` takes a modal sequent or the
  sequent a tableau starts from; **the help of `prove`** names
  `--system`; **the help of `interact`** names `--invertible`.
- **The protocol has a fourth suite**, `modal`, with its kinds and
  operations, so a program that walks `protocol::suites()` or the
  catalogue meets them.
- **0.5.0 refuses what 0.6.0 writes at wire level 20**, naming the
  level (`unsupported_version`).

### Added

- **The invertible rules in LK and LJ** (`ordinary::Interactive::invert`,
  `invert_all`, `is_invertible` and `invertible_steps`; `interact --logic … --invertible` and
  the session command `invertible on|off`; the bindings'
  `interactive.invertible` and `ordinary_invert_all`): the axiom and the
  rules of the connectives with a shared context are applied by
  themselves, but in LJ `→L`, `¬L` and `∨R₁`, `∨R₂`, which can lose a
  proof.
- **Modal formulas and systems** (the library, `deduxit::modal`): a
  sequent of propositional modal logic is read from text, the syntax of
  ordinary logic with the prefixes `[]` or `□` and `<>` or `◇`
  (`[]p -> [][]p`), and from JSON, and written as both.
  `modal::System` is one of the fifteen normal systems from K to S5,
  held as the conditions its frames meet (`modal::Condition`: serial,
  reflexive, symmetric, transitive, euclidean), so
  `System::T.with(Condition::Euclidean)` is `System::S5`; it reads the
  usual names (`S4`, `KT4`, `KD45`). A sequent with `□` or `◇` is
  written at wire level 20.
- **Modal sequents decided in all fifteen systems** (the library,
  `modal::decide`): a valid sequent comes with a derivation in the
  system's labelled sequent calculus (`modal::calculus`, Negri's G3K
  with the frame rules of the system, as a calculus given as data),
  which the checker of `deduxit::rules` accepted; a sequent that is not
  valid comes with a Kripke model whose relation meets the system's
  frame conditions and that refutes it, checked too where it is small
  enough to check. `Model::truth`
  and `Model::frame_fault` check a model you give against a modal
  sequent and a system, naming the frame condition that fails and the
  worlds that show it. The outcome has a JSON form at level 20.
- **Modal logic in the command** (`--system`): `deduxit prove --system
  S4 '[]p -> [][]p'` decides a modal sequent and prints the derivation,
  or the countermodel's worlds and what each sees; `--format json`
  writes the outcome, which `check` checks again. `check --system K
  --sequent SEQUENT MODEL` checks a Kripke model you write. A derivation
  is drawn in LaTeX, Typst and SVG, and a countermodel in SVG, PNG and
  PDF, with an arrow from each world to each world it sees.
  `check --calculus` takes the labelled calculi by name (`G3S4`), and
  `workspace new --system` keeps a modal sequent.
- **Modal logic in the protocol**: the kinds `modal.sequent`,
  `modal.outcome`, `modal.model` and `modal.evaluation`; the operations
  `modal.decide`, `modal.countermodel`, `modal.evaluate` (a model
  checked against a sequent), `modal.derivation`, `modal.calculus` (the
  system's rule file), `modal.draw`, `modal.export` and
  `modal.duplicate`; and the schema's forms `modal_sequent`,
  `modal_outcome`, `modal_statement` and `modal_evaluation`.
- **Modal proofs step by step**: `deduxit interact --calculus G3S4
  'w0 : []p |- w0 : [][]p'` proves a labelled sequent in any system's
  calculus. `derive G RULE N…` applies a rule forwards to members of the
  goal in a sequent calculus: `L□` derives `y : p` from `x : □p` and
  `x R y`, `Trans` derives `x R z`; a rule that acts on or adds a
  member after `⊢`, splits the goal or closes it is refused forwards
  with the reason. In the
  library, `rules::Interactive::extend` and `rules::Calculus::contexts`.
- **Certificates of modal logic** (`--format rocq` under `--system`):
  a valid sequent's labelled derivation as a term of deduxit's Rocq
  library (`Deduxit.Modal`), proving the sequent valid over the system's
  frames by `modal_soundness`, and a countermodel proving it not valid
  by `modal_refutation`; `check --format rocq` of a modal outcome or of
  a derivation in a `G3…` calculus, and a session's `proof --rocq`, the
  same. In the library, `export::rocq::write_labelled`,
  `write_modal_countermodel` and `write_modal_outcome`, and
  `rocq::Unsupported::Assumption` for a derivation that rests on an
  assumption.
- **`[]` and `<>` in a calculus given as data**: a calculus's text
  syntax reads them as `□` and `◇` where it has those connectives.

- **A tableau written by hand, checked step by step** (`check
  --tableau`, the library's `ordinary::WrittenTableau`, the bindings'
  `ordinary_tableau_review`): a signed tableau of classical logic as a
  student writes it, a numbered line per signed formula with the line it
  expands, branches as an indented list or as `prove --method tableau`
  draws them, `×` and `open` under a branch. The check names the first
  step that breaks a rule (its line, the rule, what the rule gives), a
  branch marked wrongly and the first branch not finished; a tableau
  without a fault is checked again as a derivation in the rule file
  `tableau`, and an open one's branch gives a checked countermodel.
  `--sequent` names the sequent it must start from. Exit 0 for a right
  and finished tableau, 1 for a wrong or unfinished one.
- **The textbooks' rules in the rule file `tableau`**: `T↔′` and
  `F↔′` expand an equivalence by its sides' values (`T p, T q` and `F
  p, F q`), beside LK's two implications; `F→₁` and `F→₂` take one
  result of `F→` alone, as `T∧₁` and `F∨₂` already did.

### Wire forms

- Level 20 adds modal logic: a sequent whose formulas hold `□` or `◇`
  (one without is written at level 8, as the ordinary sequent it is),
  and the outcome of deciding one, with its derivation or its
  countermodel. `wire::LEVEL` is 20. Every document of levels 1 to 19
  reads and is written as before.

## [0.5.0] - 2026-11-16

Calculi given as data, and natural deduction. A rule file in JSON
declares a calculus: its connectives, the zones of a sequent and its
rules. deduxit checks a derivation in it, builds one step by step,
backwards from a goal or forwards from numbered lines, and draws it.
Natural deduction for classical, intuitionistic and minimal logic is
built in (NK, NJ, NM and the variants courses use), drawn as a
textbook's tree or as a Fitch proof in LaTeX, Typst and SVG. Classical
sequents are translated into intuitionistic and minimal logic by the
negative translations, and Craig interpolants are read off derivations;
each result is checked. `deduxit::protocol` lists what each suite offers
as data, with a JSON Schema of every form, and a workspace keeps a
sequent and what was made from it, with undo and redo. On the web page
the ordinary tab proves in natural deduction and in a calculus of your
own, translates and interpolates, and links the tabs made from one
sequent; Learn and Practice follow the tab's suite, with a course of six
lessons for ordinary logic. Each release now carries the JSON Schemas of
its version, `wire.schema.json` and `rules.schema.json`.

### What breaks, and what to do instead

Nothing that 0.4.0 offers was removed or changed: its calls, types,
command lines and JSON forms are as they were, and every new variant or
field belongs to a `#[non_exhaustive]` type. What a program may notice
is under "Changed".

### Changed

- **A default build of the library compiles `serde_json`**, which the
  new default feature `protocol` needs. To leave it out, turn the
  default features off and name those you use.
- **Thirteen new error codes** (`Error::code`): `invalid_calculus`,
  `invalid_derivation`, `zones`, `step_refused`, `step_needs`,
  `not_fitch`, `negative_target`, `claim_fails`, `not_cut_free`,
  `partition`, `workspace_limit`, `not_applicable` and `not_given`. Only
  the new calls answer them.
- **`deduxit` without a command lists `interpolate` and `workspace`**
  beside the others, and the help of `check`, `interact`, `prove` and
  `seq print` names the new flags and the style `fitch`.
- **0.4.0 refuses what 0.5.0 writes at wire levels 15 to 19**, naming
  the level (`unsupported_version`), and a settings file that sets one
  of the new keys (`limits.workspace`, `view.layout`, the style
  `fitch`).

### Added

- **Natural deduction drawn as textbooks draw it**: in a tree each
  hypothesis stands alone at a leaf, the ones a rule discharges in
  brackets with a number that the rule carries (`[a]¹` … `→I¹`), in
  LaTeX, Typst and SVG. **A Fitch proof** of any derivation in a
  calculus given as data (`rules::Fitch`): numbered lines, each
  subproof's lines together under its hypotheses, a rule citing lines
  and subproofs (`→I 1–6`), written as text, as SVG, for the LaTeX
  package `fitch` and for the Typst package `fitchcraft`.
  `check --calculus` draws both (`--format latex|typst|svg`, `--layout
  fitch`, the settings key `view.layout`, the style `fitch`); a
  derivation whose lines cannot be put in Fitch's order is refused with
  the reason (`Error::NotFitch`). A session's Fitch proof
  (`rules::Interactive::fitch_derivation`) opens every subproof with its
  hypotheses, those no step has used yet too, and says which Fitch line
  shows each of the session's lines; `Drawing::nodes` and `Fitch::nodes`
  say which node each inference or line shows.

- **A calculus given as data** (the library, `deduxit::rules`): a rule
  file in JSON (connectives, the zones of a sequent, rules as premises
  and a conclusion over schematic formulas and contexts), a derivation
  whose steps name its rules, and `Derivation::check`, which matches
  every step against its rule. The verdict is "valid in the calculus
  given". Hypotheses, discharge and nested scopes serve natural
  deduction; sorts serve signed and labelled formulas. LK, LJ and
  minimal logic's calculus come as rule files, and
  `Derivation::from_ordinary` writes a derivation of LK or LJ in this
  form. Both documents are at wire level 15.
- **`check --calculus CALCULUS`** checks such a derivation against a
  rule file, or a built-in calculus by name (`LK`, `LJ`, `LM`, `LL`,
  `tableau`); a derivation that names a built-in calculus needs no
  flag. The verdict line reads "valid in the calculus given", and the
  derivation's nodes follow as a list indented by scope.
- **Natural deduction** as built-in calculi: `NJ`, `NK` and `NM` for
  intuitionistic, classical and minimal logic, and the variants
  `NK-DNE`, `NK-LEM`, `NJ-defined`, `NK-defined` and `NM-defined`, each
  rule file naming its sources (`rules::NJ`, `rules::NK`, … in the
  library). `check --calculus NJ` and `interact --calculus NJ` take
  them by name. Each derives exactly what the ordinary suite calls
  valid in its logic, which the tests check both ways.
- **`interact --calculus CALCULUS`** builds a derivation step by step
  in a calculus given as data: `rules` lists what applies, `apply`
  applies a rule backwards and asks what the goal leaves open by name,
  `assume` closes a goal by a hypothesis in force (`--assume` adds
  hypotheses), and every step is checked by the calculus's checker. In
  the library, `rules::Interactive`; its JSON is at wire level 17. A
  calculus's terms and sequents read
  from text in its own syntax (`Calculus::parse_term`,
  `parse_sequent`).
- **Forward steps in a session of a calculus given as data**: `derive G
  RULE L…` applies a rule forwards from lines in force and adds its
  conclusion as a new line, beside `apply`'s backward steps; each goal
  lists the lines in force with numbers that never change, and `assume
  G L` closes a goal by line L. In the library, `Interactive::derive`,
  `lines` and `cite`, and `undo` answers `rules::Undone`. A session
  that holds such a line is written at wire level 18.
- **A derivation in a calculus given as data is drawn** as one of LK
  is: `rules::Drawing::new(&calculus, &derivation, &limits)` gives what
  `export::svg`, `latex` and `typst` draw, each node under its rule's
  name and the formulas in the rule file's symbols. Rocq has no
  certificate for it (`Unsupported::Calculus`) and it has no
  lambda-term (`NoTerm::Calculus`). The web bindings build such a
  derivation step by step (`rules_start`, `rules_apply`, `rules_assume`,
  `rules_undo`, `rules_check`, `rules_draw`), in a built-in calculus or a
  rule file the page holds.
- The manual has a chapter on rule systems written as hypotheses and
  decided by the Horn engine, and when a calculus of your own is the
  better tool.
- The library also ships one-sided linear logic and signed tableaux as
  rule files (`rules::LL`, `rules::TABLEAU`), writes a tableau in the
  latter (`Derivation::from_tableau`), and prints a calculus's terms by
  its print forms (`Calculus::show`, `show_sequent`).
- **Negative translations, checked** (`deduxit seq translate`, the
  library's `ordinary::negative`, the bindings' `ordinary_translate`):
  Kolmogorov's and Gödel and Gentzen's translations of classical logic
  into minimal logic, Kuroda's and Glivenko's into intuitionistic logic.
  The sequent is decided classically and its translation in the target
  logic, and both verdicts are shown, with their derivations or
  countermodels in the JSON; verdicts that differ are an error.
- **Craig interpolants, checked** (`deduxit interpolate`, the library's
  `ordinary::interpolant`, the bindings' `ordinary_interpolate`): read
  off a derivation of LK or LJ without cuts by Maehara's method, for a
  split of the sequent you name (`--first`), and checked by deciding
  both halves and comparing their atoms.
- The answers of both are documents at wire level 16, above the rule
  files' 15.
- **What each suite offers, as data** (the library's `protocol`, a
  default feature): the kinds of values, the operations between them
  (`ordinary.translate`, `linear.prove`, …) with their parameters, the
  settings each reads and what each promises, run and checked over JSON
  (`protocol::run`, `protocol::check`, `protocol::catalogue`). The
  manual's chapter "What each suite offers" lists them, with a JSON
  Schema of every form.
- **A workspace** (`deduxit workspace`, the library's `workspace`):
  a JSON file that keeps sequents and what was made from them by those
  operations, each node with its parameters and the limits it ran
  under. Changing a sequent marks what was made from it out of date
  until `workspace run` makes it again; `undo` and `redo` take back and
  bring back whole edits and show the earlier results without running
  them again; `--checks` checks what each operation promises. Reading a
  file checks every value and runs again each run that needs no search.
  Its bounds are the settings `limits.workspace` (nodes, runs kept,
  edits undo keeps, bytes); past one, a change is refused
  (`workspace_limit`). Its JSON is at wire level 19: the highest level
  the library now reads and writes.

### Wire forms

- Level 15 adds a calculus given as data: the rule file, and a
  derivation in such a calculus.
- Level 16 adds a classical sequent's negative translations compared
  and an interpolation, each holding ordinary outcomes nested without
  `version` and `deduxit`. They are written only.
- Level 17 adds a session of step-by-step proving in a calculus given as
  data: the rule file, the terms, the scopes, the nodes from the
  conclusion up and the steps.
- Level 18 adds a line that a forward step derived, a node's `line` and
  a goal's `cites`, in such a session; a session without one is written
  at level 17.
- Level 19 adds the workspace: its values, each its kind's document
  whole, the nodes, given or made by an operation, the records of runs
  and the log of edits. `wire::LEVEL` is 19. Every document of levels 1
  to 14 reads and is written as before.

## [0.4.0] - 2026-11-15

Intuitionistic logic decided by its own engines, with Kripke
countermodels, and truth tables and tableaux for classical logic. A SAT
solver decides classical, intuitionistic and minimal logic by default,
with Dyckhoff's calculus LJT beside it. A sequent that is not valid
comes with a Kripke countermodel, which is checked, printed, drawn and
certified in Rocq, and `check` checks a model you give. The Horn
engine's affine refutations are checked, the command guides a
newcomer's first errors, and the help, the messages and the manual are
rewritten. On the web page, a goal that is not valid shows its
countermodel, and classical logic has its truth table and tableau. The
Rocq library rocq-deduxit 0.4.0 adds intuitionistic and minimal logic
with their Kripke models, which the countermodels' certificates need.

### What breaks, and what to do instead

- **`prove --logic L --format json` writes the ordinary outcome under
  every method**, at wire level 12 (14 for the methods `table` and
  `tableau`). Through the embedding it wrote the
  translated sequent's outcome at the top; now that outcome is nested
  under `search.image`. Under every method the keys `engine`,
  `translation` and `statistics` moved into `search`, an object tagged
  by `method` (`embedding`, `ljt` or `sat`). Read `search.method`,
  `search.translation`, `search.statistics` and `search.image`, or give
  `--linear` for the translated sequent's outcome alone, as before. The
  batch's JSON Lines and files under `--logic` are the same.
- **`ordinary::decide` takes the sequent and the options alone**, within
  the default limits, as `linear::prove` does. The call with a search's
  options, limits and a stop is `ordinary::decide_within(&sequent,
  &options, &search, &limits, stop)`.
- **`prove --logic intuitionistic` and `--logic minimal` decide by a
  SAT solver by default**, whose search always ends, with the derivation
  of LJ that Dyckhoff's calculus LJT gives where it gives one. The
  verdict line reads `valid (intuitionistic logic by the SAT solver)` or
  `not valid (…): a checked countermodel refutes it`, `--stats` prints
  the solver's counters, and `--format json` writes the outcome of the
  ordinary sequent (`verdict` `valid`, `not_valid` or `unknown`,
  `search.method` `sat`) in place of the translated sequent's. Where a
  bound other than
  the time limit stops the solver, the embedding decides as before. For
  the old behaviour, give `--method embedding` or `--translation`.
- **`prove --logic classical` decides by the SAT solver by default**,
  and keeps the embedding's output wherever the embedding gives a
  derivation of LK or a refutation within 2²⁰ units of work
  (`ordinary::EVIDENCE_WORK`): its verdict line, derivation,
  refutation and certificate, as before. Where the embedding proves
  the translated sequent but its derivation of LK passes the
  derivation limit, `--format json` writes the solver's clausal proof
  in place of the translated sequent's proof. Otherwise the solver's answer stands: `valid (classical logic
  by the SAT solver): a checked clausal proof of …`, with no derivation,
  or `not valid (…): a checked countermodel refutes it`. On the ILTP
  library at 2 s the default decides 254 problems where it decided 124.
  For the old behaviour, give `--method embedding`.
- **`ordinary::Outcome` says how the sequent was searched**, in its new
  field `search` (`ordinary::Search::Embedding { translation, linear }`,
  `ordinary::Search::Ljt` or, with the feature `sat`,
  `ordinary::Search::Sat`), which replaces the fields `translation`
  and `linear`. `ordinary::decide` decides every logic by the SAT
  solver (LJT and the embedding without the feature `sat`) unless
  `ordinary::Options` names a translation or a method; in classical
  logic it then asks the embedding for its evidence within
  `ordinary::EVIDENCE_WORK`.
- **`ordinary::Verdict::Valid` holds the evidence**, an
  `ordinary::Evidence`: `Evidence::Derivation` with the checked
  derivation of LK or LJ, as before, or, with the feature `sat`,
  `Evidence::Clausal` with a checked clausal proof. Match
  `Verdict::Valid(Evidence::Derivation(derivation))`, or call
  `evidence.derivation()`, which refuses a clausal proof with the new
  `Error::NoDerivation` (code `no_derivation`; its field `evidence`
  names what the sequent is valid by).
- **`ordinary::Rule` has the variant `Cut`**, Gentzen's cut in LJ,
  which LJT's derivations hold; it comes before `Open`, which moves
  `Open`'s place in the derived order and its number. A `match` over
  `ordinary::Rule` without a wildcard adds an arm for `Rule::Cut`.
- **`Error::Logic` names the method** that does not decide the logic,
  in its new field `method`: LJT for classical logic, as before, and the
  truth table and the tableau for intuitionistic and minimal logic.
  A pattern that names its fields adds `method` or `..`.

### Changed

These outputs changed their words; a script that matches them needs
the new text. The JSON forms, exit statuses, flags and defaults are
unchanged.

- **The refusals of a proof of LK or LJ step by step** (`interact
  --logic`, `ordinary::StepError`) say what to do: a rule the logic
  does not allow is "not available in L logic", with the way out where
  there is one (`∨R₁` or `∨R₂` for `∨R` in LJ), and a position past the
  goal's formulas says positions start at 0.
- **`--net`'s last line** says whose verdict it is: `proof net (the
  correctness criterion holds)`, `proof net with Mix (the correctness
  criterion with Mix holds)` and `planar proof net (the correctness
  criterion holds, and no two links cross)`. It still starts with
  `proof net` or `planar proof net`.
- **A classical refutation of a sequent written with two sides** speaks
  of the sides: `A on the left has no A on the right to meet in an
  axiom`, and counts left and right where both have some. Where an
  unused hypothesis is the problem, the command adds `; with -a
  (affine), an unused hypothesis is allowed`. The JSON `refutation` is
  unchanged.
- **Three refusals say what to do**: in `interact`, a goal that is no
  number says ``"x" is not a goal: a goal is a number, and `goals` lists
  them`` instead of Rust's parse error, and a position that is no number
  says ``"x" is not a position: a position is a number, and `goals`
  lists them``; `prove --net` on MELL with units
  says `proof nets with boxes exist only for MELL without units, and the
  sequent has a unit`.
- **The intuitionistic count refutation** counts by sides too, without
  "positively" and "negatively": `A occurs 1 more time on the right than
  on the left, so they cannot all meet in axioms (an implication's
  antecedent counts on the other side)`. Where one side has no `A`, it
  says so: `A on the right has no A on the left to meet in an axiom (an
  implication's antecedent counts on the other side)`.
- **An SVG drawing's description** (`<desc>`, which a screen reader
  reads, and a PNG and an accessible PDF carry) writes a negated atom
  `A⊥`, as the drawing and its title do, where it wrote `~A`. A net's
  description is short sentences in place of its text form: what it
  concludes, each link with its formulas by number, each box, and
  whether it is a proof net.
- **Ordinary logic's "not valid"** through the embedding says `the
  translated sequent is unprovable` in place of "the image".
- **The help and the messages are rewritten** to say what went wrong and
  what to do. `prove -h` lists the common options under the headings
  Logic, Input, Output, Search and Limits and ends with "--help shows
  every option."; `--help` lists every option. In `interact`, a rule
  that does not act on the formula named now names the formula and
  `rules G P`.
- **The manual is reorganized**: a short tour, a guide for each task
  (among them "Avoid the common mistakes", "Check a proof or a
  refutation" and "Use the library"), the reference, and an explanation
  of how the search works.

### Added

- **Truth tables and tableaux, the methods of a first logic course**,
  for classical logic: `prove --logic classical --method table` prints
  the sequent's truth table, its falsifying rows marked, and `--method
  tableau` a signed tableau (`T` true, `F` false) with numbered lines,
  each naming the rule of LK and the line it expands, and each branch
  closed (`×`) or open. A closed tableau is a derivation of LK built
  step by step in the goal state and checked whole; an open branch and
  the first falsifying row give a checked countermodel. The table is
  bounded by the new `Limits::table_atoms` (10 atoms, 1024 rows, by
  default; `--table-atoms N|none` on `prove` and `check`), refused past it with
  `Refusal::Atoms` (code `too_many_atoms`). In the library:
  `ordinary::Method::Table` and `Method::Tableau`, `ordinary::truth_table`
  and `TruthTable`, `ordinary::Tableau` (`Tableau::of` reads any
  derivation of LK as one) with `TableauNode`, `Sign`, `Signed` and
  `End`, `ordinary::Search::Table` and `Search::Tableau`, and
  `ordinary::Evidence::Table`. The bindings' `ordinary_table` and
  `ordinary_tableau` answer either method's outcome, and `about` lists
  them as the ordinary suite's methods.
- **A "not valid" comes with its Kripke countermodel**: the SAT solver's
  model in every logic, and in classical logic the embedding's
  assignment as a model of one world, checked before the verdict.
  `prove --logic` prints the SAT solver's after the verdict line, a
  world per line (the embedding's verdict line names its assignment, and
  where its refutation is none the solver's model follows it);
  its JSON holds it under `countermodel`, with the sequent it refutes,
  and `check` checks it there. LJT's "not valid" has none. A model too
  large to check (past `ordinary::MODEL_CELLS` cells or the memory
  limit) is reported as too large to show, the verdict kept.
- **`check --logic L --sequent SEQUENT FILE` checks a Kripke model you
  give** against a sequent: that it is a model of the logic, then
  whether the sequent holds at `--world N` or at every world, and where
  it fails, which world and what it forces of each subformula (exit
  status 0, 1, or 2 for no model of the logic). The bindings' call
  `ordinary_model` does the same. A model that fails its check is the
  new `Error::Model` (code `invalid_model`, kind invalid).
- **A Kripke countermodel is drawn**: `--format svg`, `png` and `pdf`
  on a "not valid" with a countermodel draw it, each world a box with
  the atoms it forces (and `⊥` in minimal logic), an arrow up to each
  world directly above it, the model's world outlined in the highlight
  colour, with a title and a description in sentences. `check` draws an
  outcome's countermodel and a model you give at the world where it
  fails. LaTeX and Typst draw none and say so. In the library,
  `export::svg::countermodel`.
- **The bindings give the page a countermodel**: `ordinary_close` and
  `ordinary_hint` answer a goal that is not valid with the SAT solver's
  countermodel in the outcome, and `ordinary_model_drawing` and
  `ordinary_model_certificate` draw it as SVG and write its Rocq
  certificate.
- **A Kripke countermodel has a Rocq certificate**: `--format rocq`
  on a "not valid" of intuitionistic or minimal logic writes the lemma
  `~ lj (atom := string) intuitionistic [Γ] C` (or `minimal`) for the
  Rocq library, proved by its `kripke_refutation` from the model as
  data, which the library checks by computation; a classical model of
  one world gets the certificate over `Prop`. `check --format rocq`
  writes it for an outcome's countermodel and for a model you give, at
  the world where it fails. In the library,
  `export::rocq::write_countermodel` and the prelude
  `DEDUXIT_KRIPKE_PRELUDE`. It needs the Rocq library's
  `Deduxit.Intuitionistic`, which its 0.4.0 adds.
- **`deduxit::kripke::Model`**, the countermodel shared by the suites:
  worlds, their atoms by name, `falsity` for minimal logic, the arrows
  of the order and the world it is about. Ordinary logic checks it
  (`Model::forcing`, `Model::refutes`, `ordinary::Forcing`), and
  `ordinary::Countermodel` pairs it with the sequent it refutes, in the
  new `ordinary::Outcome::countermodel`. It replaces `ordinary::sat::Model`
  and `World`, whose atoms were indices.
- **`check` reads every outcome `prove --logic` writes**, and a
  derivation of LK or LJ: it checks the derivation by the rules of its
  calculus (LJ's cut included) and the translated sequent's proof or
  refutation where the embedding found one. A clausal proof, written as
  its counts, a "not valid" of LJT or the SAT solver, which carries no
  refutation, and an unknown hold nothing to check: `unknown`, exit
  status 3. Its
  JSON names the `logic` in place of the `mode` for these.
- **The ordinary outcome reads back**: `ordinary::Outcome` is
  `Deserialize` and read by `wire::upgrade`, its derivation and the
  translated sequent's proof unchecked until their checkers run, as the
  linear outcome is; `ordinary::ljt::Statistics` and
  `ordinary::sat::Statistics` read back too.
- **`ordinary::Image::outcome(linear, &limits, stop)`** makes the ordinary
  outcome from a search of the translated sequent that the caller ran,
  as the command does: a proof's derivation is read back and checked, or,
  where a bound or the stop refuses it, the new `ordinary::Evidence::Image`
  says the translated sequent's proof stands alone (`evidence.derivation()`
  answers the new `Error::NotBuilt`, code `not_built`).
- **Hints on a newcomer's first errors**: `deduxit "A |- A"` suggests
  `deduxit prove "A |- A"`; an unquoted sequent suggests the command
  quoted; `->` in linear logic says to write `-o` or add `--logic
  classical`; a file of `NAME: SEQUENT` lines read as one sequent
  suggests `--input-format lines`. `--format tex` is read as `latex`.
- **`interact` guides**: an unknown rule's error lists the session's own
  rules with their ASCII spellings, and after a split whose premise has
  no proof a warning says so and that `undo` takes it back.
- **Deciding as the command does, in the library**: `Settings::stop`
  builds the command's stop from the time since the start. It fires at
  the time limit and lets the classical reading after the search run
  past it.
  `Settings::deterministic()` gives the settings of `prove
  --deterministic`: one thread throughout.
- **Printing without a stop**: `export::{latex, typst, svg,
  rocq}::to_string(derivation, &options)` beside each `write`, and
  `Display` for `lambda::Term`.
- **Reading the command's JSON**: the linear `Outcome` implements
  `Deserialize`, so a file of `prove --format json` reads back with its
  verdict, mode and engine. An engine name the library does not know
  reads as the new `Engine::Unknown`.
- **A family refuses an index past its instances**: `Family::instance`
  returns the new `Error::FamilyIndex` (code `family_index`), where it
  returned an instance whose expected verdict could be wrong. `deduxit-bench
  one --problem family:NAME:SIZE:INDEX` refuses it too.
- **`with_lambda`** on `export::latex::Options`, `export::typst::Options`
  and `export::svg::Style`, as every other field has its `with_*`.
- **A refused step names its formula**: `StepError::Rule` carries
  `formula`, the formula as the session writes it, and its message says
  it (`the rule ⅋ does not act on formula 1, A ⊗ ~B`). The web bindings'
  error has it under `details`.
- **Errors show the formula**: `ShapeError::Formula` carries the
  formula's text, so `prove` in an intuitionistic mode names the
  subformula that has no intuitionistic reading.
- **LJT**: `ordinary::ljt::decide` decides intuitionistic and minimal
  logic by Dyckhoff's contraction-free calculus. Its proofs are
  translated into derivations of LJ with Gentzen's cut, which
  `Derivation::check` accepts. `ordinary::Method`,
  `ordinary::Options::method` and `ordinary::gives_way` choose between it
  and the embedding, and the command has `--method embedding|ljt` (the
  key `ordinary.method`).
- **A SAT-based engine for classical, intuitionistic and minimal
  logic**, in the library: `ordinary::sat::decide`
  (`ordinary::Method::Sat`, behind the new default feature `sat`, the
  SAT solver batsat). In intuitionistic and minimal logic every "valid"
  comes with a derivation of LJ that the checker accepts. In classical
  logic it comes with the solver's clausal proof (`sat::ClausalProof`),
  which a checker of deduxit's own accepts before the verdict is given;
  it has no derivation, which `Method::Embedding` gives. Every "not
  valid" comes with a model that `Model::refutes` checks: a finite
  Kripke model (`kripke::Model`), of one world in classical logic. The
  command offers it as `--method sat` (the key `ordinary.method`), with
  its counters under `--stats`, and the harness as the engine `sat`. It
  is the default for intuitionistic and minimal logic, where on the ILTP
  library's 274 problems at 2 s it decides 252 in intuitionistic and 253
  in minimal logic, LJT 200, and takes LJT's derivation where LJT gives
  one (`ordinary::sat::decide_preferring_ljt`), and for classical logic,
  where it keeps the embedding's derivation, refutation and
  certificate wherever the embedding gives them within
  `ordinary::EVIDENCE_WORK`.
- **The harness runs ordinary logic**: `deduxit-bench run --tptp PATH
  --logics …` with the engines `embedding`, `ljt` and `auto`, and
  `summary` lists the problems two configurations decide differently.
- **A certificate is checked in one command**: `nix run
  github:deduxit/deduxit#check-certificate -- proof.v` compiles a
  certificate written with `--standalone` against the pinned NanoYalla
  or rocq-deduxit, the kernel read from its import lines. It refuses a
  lemma that rests on an axiom, such as a proof left `Admitted`. The
  package of the same name installs it (`nix profile install
  github:deduxit/deduxit#check-certificate`).
- **A checked refutation for the Horn engine's backward search**: in
  affine mode, where only that search refutes a Petri net, the
  unprovable verdict carries the markings it kept
  (`Refutation::Basis`, with `Basis` and `Marking`), checked before the
  verdict and by `Disproof::check` and `deduxit check` without the
  search. It was "the search was exhaustive" before. `--format rocq`
  has no certificate for it yet (`Unsupported::Basis`).

### Fixed

- `ordinary::Derivation::term`, and every export that writes a
  lambda-term, panicked on a derivation of LJ read from JSON that breaks
  a rule. Such a derivation is now checked first, and refused with the
  check's `Error::ReadBack`; one that `check` accepted reads as before,
  in either view of its structural rules.

### Wire forms

- Level 9 adds LJ's cut, `"rule": "cut"`, in a derivation of LJ and
  a proof in progress that holds one, written at level 9.
- Level 10 adds the outcome of a sequent valid by a clausal proof:
  `"clausal_proof": {"variables", "clauses", "steps"}` in place of
  `derivation`. 0.4.0 writes every ordinary outcome at level 12 or 14,
  so no document of it is at level 10.
- Level 11 adds the refutation by a basis, `{"kind": "basis", "body",
  "classes", "markings"}`, in a disproof or an unprovable outcome that
  holds one.
- Level 12 writes every ordinary outcome with its `search` tagged by
  `method`, the translated sequent's outcome nested under the embedding's
  `image` (an outcome's keys without `version` and `deduxit`), and reads
  it back; an ordinary outcome below level 12, which no release read, is
  refused. Every other document is written as before.
- Level 13 adds the Kripke model, `{"worlds": [{"atoms", "falsity",
  "arrows"}], "world"}`, as a document, which refuses a key it does not
  know (a model is often written by hand). An ordinary outcome holds one
  under `countermodel` and stays at level 12: a reader of level 12
  ignores the key without misreading the verdict.
- Level 14 adds the truth table, `{"sequent", "rows": ["TFT…"],
  "falsifying": […]}`, and the tableau, `{"sequent", "nodes":
  [{"lines": [["T", formula]], "expands": [line, rule], "children":
  […]} | {"lines", "end": {"kind": …}}]}`, as documents and as the
  `search` of an ordinary outcome of the methods `table` and `tableau`,
  which is written at level 14. Reading a table computes it again from
  its sequent; reading a tableau checks its shape and indices.

## [0.3.0] - 2026-11-09

The third release, the first under the name deduxit: a collection of
logic suites, linear logic first, its linear suite now the module
`deduxit::linear`. Ordinary logic is proved step by step in LK and LJ,
intuitionistic proofs read back as lambda-terms, the one-sided count
and the state equation are checked without the search and certified in
Rocq, the inverse method takes over wide multiplicative sequents where
the focused engine runs out of memory, and the mark dēdūxit replaces
linlog's logo.

### What breaks, and what to do instead

Found by tools, not by memory: `cargo semver-checks` against the
published `linlog` 0.2.0 and `cargo public-api`'s diff of the two public
APIs with every feature (the devshell's `breaks 0.2.0`, which names the
old package deduxit to compare it), and, for the command, the help and
outputs the behaviour lock pins at 0.2.0's tag and now. Of the diff's
items, all but `cut::eliminate` and `ordinary::Rule::Open` below are
the rename and the move under `deduxit::linear`; every other change is
an addition.

- **The crates, the command and the library are renamed deduxit.** The
  library is the crate `deduxit` (was `linlog`) and the command the
  package `deduxit-cli` with the binary `deduxit` (was `linlog-cli` and
  `linlog`), with the same verbs and flags. Depend on `deduxit = "0.3"`
  and install `cargo install --locked deduxit-cli`; the flake's package
  is `deduxit-cli`, its default program `deduxit`. The last releases of
  the old names, `linlog` 0.3.0 and `linlog-cli` 0.3.0, point here:
  `linlog` re-exports `deduxit` (its root and `deduxit::linear`, so
  0.2.0's paths still name their items), and `linlog-cli`'s binary
  `linlog` says once on standard error that it is now `deduxit` and runs
  it. Neither is released again.
- **The linear suite is the module `deduxit::linear`.** The items 0.2.0
  had at the root and its modules `sequents`, `occurrences`, `fragment`,
  `proofs`, `nets`, `search`, `lltp`, `mist` and `families` are under
  `deduxit::linear` (`deduxit::linear::{Sequent, Mode, Options, prove}`,
  `deduxit::linear::search::…`); the error family, the limits and
  progress, the settings, the exports and the wire forms stay at the
  root (`deduxit::{Error, Limits, Progress, Settings}`,
  `deduxit::export`, `deduxit::wire`), beside `deduxit::ordinary` and
  `deduxit::lambda`.
- **Written under the new name, the old still read**:
  - an outcome names its writer's version under `"deduxit"` (was
    `"linlog"`); a file 0.2.0 wrote reads and checks as before;
  - the settings values `deduxit` of `rocq.kernel`, `typst.layout` and
    `svg.net_look` (was `linlog`: deduxit's Rocq library, its own Typst
    layout and its own look of a net); a settings file that says
    `linlog` still loads, and so does `--style KEY=linlog`, which reads
    its value as the settings file does; the flag `--kernel` takes the
    new name only.
- **The Rocq library is `rocq-deduxit`**, logical root `Deduxit` with
  linear logic under `Deduxit.Linear` (was `rocq-linlog`, `Linlog`): a
  certificate for the kernel `deduxit` (`--kernel deduxit`, was
  `--kernel linlog`) imports `From Deduxit.Linear Require …`, and
  compiles against `rocq-deduxit` 0.3.0. `export::rocq::Kernel::Deduxit`
  and the constants `export::rocq::DEDUXIT` and `DEDUXIT_*_PRELUDE` were
  `Kernel::Linlog`, `LINLOG` and `LINLOG_*_PRELUDE`; `typst::Layout::Deduxit`,
  `svg::NetPreset::Deduxit` and `svg::NetLook::DEDUXIT` were `Linlog`
  and `LINLOG`.
- **`linear::proofs::cut::eliminate` takes the first step's cut**, `at`, an
  `Option<NodeId>` after the mode (`None` the strategy's, as before), as
  `cut::step` does; `deduxit eliminate --at NODE` sets it.
- **`ordinary::Rule` has the variant `Open`**, the rule of an open goal
  in a proof in progress (`ordinary::Interactive`); `ordinary::Rule` is
  a closed enum, so a `match` on it needs an arm for `Open`, which no
  finished derivation contains.
- **The command, where a script may see it**:
  - `check` and `eliminate` check a proof in the mode its file states;
    a mode flag that names another is an error (exit status 2) where
    0.2.0 checked in the flags' mode: drop the flag, or remove the
    file's `"mode"` to choose one;
  - a proof whose Rocq certificate is refused keeps its verdict and its
    exit status, with `no certificate: …` on standard error, where 0.2.0
    exited 2; a session's `proof FILE.v` then makes no file;
  - a batch names each Rocq certificate's file by Rocq identifiers
    (`modus_ponens.v`, `entry_1_1_p.v`), and every lemma is
    `certificate`, which the module qualifies;
  - `prove --logic` draws a derivation of LK or LJ with each weakening
    and contraction left out where it is admissible (`--structural
    admissible`, the default): `--structural explicit` draws every one,
    as 0.2.0 did; a certificate names every structural rule either way.

Not breaks, since their types are `#[non_exhaustive]`, but new: the
variants `Error::NoTerm` and `Error::OrdinaryStep`,
`export::rocq::Unsupported::Exhausted` (after the others, so no
discriminant moved), and the fields `ViewOptions::structural`,
`Settings::lambda` and `lambda` of `latex::Options`, `typst::Options`
and `svg::Style`.

### Added

- **Ordinary logic step by step**: `ordinary::Interactive` (feature
  `interactive`), a goal state of two-sided ordinary sequents whose
  rules are the checker's, with `goals`, `rules`, `apply` (a `Step`, and
  for a rule of two premises a split of the context), `undo`, `close`
  (the goal decided through `ordinary::decide` and the derivation
  grafted), `hint` and `finished`, the derivation the checker of LK and
  LJ validates; `FromStr` for `ordinary::Rule`. The command's `interact
  --logic classical|intuitionistic|minimal SEQUENT`, with the linear
  session's commands where they mean the same and a formula named by
  side and position (`L0`, `R1`); the bindings' `ordinary_*` calls.
- **Weakening and contraction shown or left out**:
  `ViewOptions::structural` (`Structural::Admissible`, the default, or
  `Explicit`), the flag `--structural` and the settings key
  `view.structural`.
- **Proofs as programs**: `Derivation::term` reads a two-sided proof of
  intuitionistic linear logic back as a term of the linear lambda
  calculus of Benton, Bierman, de Paiva and Hyland (the affine one in
  affine mode, a cut the redex `(λx:A. N) M`), and
  `ordinary::Derivation::term` an LJ derivation as a simply typed term;
  `lambda::Term` with its JSON form (written only), printed under
  `lambda::Options` (`notation`, Unicode or ASCII, and `types`); the
  flags `--lambda`, `--lambda-notation` and `--lambda-types`, the
  settings key `lambda`, the session command `lambda`, the key `lambda`
  of an outcome's JSON, and the term under the conclusion in LaTeX,
  Typst, SVG, PNG and PDF. What has no term is refused with
  `Error::NoTerm` and its reason (`lambda::NoTerm`). The manual's
  chapter "Proofs as programs" states the calculus with its sources.
- **Refutations checked and certified**: `Disproof::check` accepts a
  one-sided count (`OneSided`, `OneSidedSymbol`) and a state equation
  (`StateEquation`) by the Rocq library's own weighted test, and the
  second kernel writes their certificates, the first-order counts
  included, against `rocq-deduxit` 0.3.0's weighted refutations;
  `deduxit check` reads an unprovable outcome and checks its
  refutation, with a proof's exit statuses.
- **The inverse method where the focused engine runs out of memory**:
  on a unit-free MLL sequent whose atoms occur once with each sign, in
  classical mode without Mix, the inverse method takes over before the
  net engine (`wide-m1` with 8 192 pairs proved in 3.05 s against
  9.48 s).
- **The command**: `check -f, --file PATH` beside the argument;
  `eliminate --at NODE` names the cut taken first; `--style`'s help
  lists the rules of a label table, and a dotted key such as
  `labels.table.RULE=LABEL` makes the table it names.
- **The last releases of the old names**: `linlog` 0.3.0 re-exports
  `deduxit` (its root and `deduxit::linear`, so 0.2.0's paths still name
  their items) and `linlog-cli` 0.3.0's binary `linlog` runs `deduxit`
  after a line on standard error.

### Wire forms

- Level 8 adds the forms of ordinary logic: an ordinary sequent, a
  derivation of LK or LJ, the outcome of deciding an ordinary sequent
  and a proof in progress of one, each always written at level 8;
  every other document is written as before. An
  outcome's version key is `"deduxit"`; a document of 0.2.0, whose key
  is `"linlog"`, reads as before.

### Changed

- **The mark**: the word dēdūxit with lapis long marks (CC BY 4.0,
  served at deduxit.github.io/logo/), in README, the manual and the
  rustdoc, replaces linlog's logo, the lollipop.
- **The addresses**: the repository is `deduxit/deduxit`, the manual
  and the API at deduxit.github.io/deduxit/, the web page at
  deduxit.github.io and the Rocq library at `deduxit/rocq-deduxit`;
  the old ones redirect.
- An essential net's input side is drawn in the blue of `⅋` (`#0072b2`,
  `#56b4e9` dark) by default, as the web page draws it, where it was
  green.
- `eliminate` names a cut of an invalid proof by its instance, as
  `check` does, where it named the raw id (`cut on #8`); `interact`'s
  `cut` explains an unquoted formula with blanks and says to quote it;
  the eigenvariable refusal names `∃L` among the steps that introduce
  one.

### Fixed

- A rule node's label in the rules look divided by zero when the node
  had no room.
- A forest past a proof structure's vertex bound (`nets::MOST`) made the
  net engines panic; it is refused in their `admits` and in
  `ProofStructure`'s constructor.

## [0.2.0] - 2026-11-02

The second release: cut and its elimination, on proofs and on proof
nets, first-order linear logic, the two-sided surface of intuitionistic
sequents, MELL proof nets with boxes, the inverse method, the logo, and
linlog's own Rocq library, now a repository of its own.

### What breaks, and what to do instead

Found by tools, not by memory: `cargo semver-checks` against the
published 0.1.0 and `cargo public-api`'s diff of the two public APIs
with every feature (the devshell's `breaks 0.1.0`), and, for the
command, the help of every subcommand of 0.1.0 and of this release
compared flag by flag and value by value. Every other change of the
library's API in this release is an addition.

- **New variants of the calculus's closed enums**, the break 0.1.0
  announced: `Term::{Forall, Exists}` and `Kind::{Forall, Exists}` (the
  quantifiers), `Node::{Cut, Forall, Exists}` and `Rule::{Cut, Forall,
  Exists}` (cut and the quantifier rules). A `match` on one of them
  stops compiling until it has an arm for each: a quantifier's term
  holds its body (the bound variable's name is
  `Sequent::binder_name`'s), a cut node names an occurrence of a cut
  formula and two premises, a quantifier node its occurrence, an
  eigenvariable (`∀`) or a witness (`∃`) and its premise. Code that
  handles only propositional sequents without cut can send these to an
  error arm: `Sequent::is_first_order` and `Forest::has_binders` say
  beforehand whether a sequent has a quantifier, and `Sequent::cuts`
  whether it has cut formulas.
- **`Rule::ALL` is `[Rule; 19]`** (16 in 0.1.0) **and `Named::ALL`
  `[Named; 41]`** (34): code that names the array's type writes the new
  length, or takes a slice (`&Rule::ALL[..]`) or the `len()`.
- **Moved numbers**: `Rule::Open` is 18 when cast with `as` (15 in
  0.1.0) and sorts after `Cut`, `Forall` and `Exists` under the derived
  `Ord`; `export::rocq::Unsupported::AffineWeakening` is 3 and
  `Compact` 4 (2 and 3), `Unsupported::Cut` coming before them. Code
  that keeps these numbers, or sorts rules and keeps the order, compares
  the variants by name instead.
- **The command**, whose subcommands, flags and their values of 0.1.0
  all remain, behaves otherwise in three places a script may see:
  - an intuitionistic sequent's text (`-i`) is read in the syntax of
    intuitionistic linear logic alone, so `~`, `^`, `par`, `|`, `?` and
    `bot` are refused with a line saying what to write: `--ill-syntax
    one-sided` (settings key `input.intuitionistic`) reads the classical
    syntax as 0.1.0 did;
  - a sequent whose search ends unknown and whose classical reading an
    assignment falsifies is unprovable, exit status 1 where 0.1.0 gave
    3: `--classical-work 0` gives the search's word alone;
  - `--format rocq` with no `--kernel` writes a proof of Mix, affine or
    intuitionistic logic for linlog's Rocq library (`rocq-linlog`),
    which 0.1.0 refused or wrote for NanoYalla: `--kernel nanoyalla`
    keeps NanoYalla's certificate where it has one.

### Added

- First-order linear logic's syntax, in the data model and both
  surfaces: `Term::Forall` and `Term::Exists` (and `Kind`'s), a binder
  over its body with the bound variable as a de Bruijn index, the name
  kept beside the arena for printing (`Sequent::binder_name`); atoms as
  predicates applied to first-order terms, interned by their canonical
  text (`Sequent::atom_predicate`, `atom_is_open`, `symbols`,
  `fo_terms`, `arguments`, `is_first_order`), the terms in the module
  `sequents::fo` (`FoTerm`, `Symbol`, `FoTermId`, `Args`, `SymbolFault`);
  `Fragment::QUANTIFIERS` with the names `MLL1`, `MLL1 with units`,
  `ALL1`, `MALL1`, `MELL1` and `LL1` (and `IMLL1`, …), read back by
  `FromStr`, and `Fragment::has_quantifiers`; `IllConnective::Forall` and
  `Exists`. The text syntax reads `forall x. A`, `exists x. A`, `∀x. A`,
  `∃x. A` and `forall x y. A`, a quantifier binding loosest and reaching
  right as far as it can, and `p(t, …)` with an identifier bound by an
  enclosing quantifier as its variable and any other as a constant or a
  function symbol; a symbol has one arity throughout, a name is a
  predicate or a function symbol, and every sequent is closed. Every
  printer writes the binders and the predicates, a bound name that a
  symbol or an enclosing binder takes written with `_` after it. A
  sequent with a quantifier is refused by its fragment by every engine
  that does not decide first-order logic (below; `Error::NoEngine`, or
  `EngineRefused` for a forced one) and by the proof structures, and by
  the Rocq export for NanoYalla and for refutations
  (`Unsupported::Quantifiers`);
  `Forest::has_binders` says whether one has a binder. A sequent whose
  atoms have arguments but whose formulas have no quantifier is
  propositional and decided by every engine. `Error::Symbol`,
  `Error::LooseVariable` and `Error::Predicates` refuse the tables a
  file cannot hold; `Space::Symbol` names their index space.

- First-order proofs: `Node::Forall(member, eigenvariable, premise)` and
  `Node::Exists(member, witness, premise)`, an eigenvariable an `Eigen`
  and a witness a closed term (`FoTermId`) of the sequent's arena or the
  proof's; the instance table `Instances` (and `FrameId`) that numbers
  every member under binders as an occurrence with a frame of terms,
  the proof's own terms extending the sequent's (`FoTerm::Eigen`), built
  for a forest (`Instances::new`, `eigen`, `apply`, `frame`, `member`)
  and owned by `Proof::with_instances`, which checks it and keeps it
  canonical, and read by `Proof::instances`, `Proof::occurrence` and
  `Proof::formula`. The checker checks such proofs: a frame pass gives
  every `∀` and `∃` step its body's instance and finds a member under
  binders named without one (`Fault::Instance`) and an eigenvariable a
  second `∀` step introduces (`Fault::Reintroduced`), and the bottom-up
  pass compares instances by number, the axiom on two literals whose
  atoms are equal under their frames, and carries every eigenvariable a
  witness or a cut's frame names to the root unless the `∀` step below
  introduces it (`Fault::Eigenvariable`); an eigenvariable no `∀` step
  introduces is a constant outside the signature. `Rule::Forall` and
  `Rule::Exists`, named `∀R`, `∃L`, `∃R` and `∀L` two-sided; the
  derivation, its size, its text and its LaTeX, Typst and SVG drawings
  write every instance with its frame's terms, an eigenvariable by a
  letter no symbol takes. A cut formula with binders names its dual's
  variables as its own. The JSON form of a proof (wire level 7) adds the
  steps `∀` and `∃` and the key `instances`. `Limits::terms` (default
  `Limits::DEFAULT_TERMS`, settings key `limits.terms`) bounds the terms
  a sequent's atoms unfold to (`Sequent::fo_size`) on every reader, and
  a proof's instance table, refused with `Refusal::Terms` (code
  `too_many_terms`); the JSON readers counted them against
  `limits.occurrences` before. The command's `--term-limit` sets it, on
  every command that reads a sequent and on `check`.

- First-order proof search: the focused engine decides a sequent with
  quantifiers in the classical modes, Mix and affine mode included, and
  the two-sided engine in the intuitionistic modes, affine included (the
  dispatch's classical and intuitionistic rows take `LL1`; every other
  engine still refuses quantifiers by its fragment). A witness of
  `∃` is a metavariable that unification binds at the axioms, with the
  occurs check; the eigenvariable condition is kept by levels; a rule of
  two premises asks its first premise for another answer where the
  second fails under the one it gave, resuming its search at its last
  choice point by replaying the choices before it (and searching it
  again with the failed answers excluded where the replay finds the memo
  changed); the proof is returned closed, every
  witness a term and every metavariable left unbound a fresh constant,
  and passes the checker. MLL1 and MALL1 are decided; with exponentials
  the copy bound and the three verdicts work as without quantifiers. The
  memo and the loop check key a first-order stable sequent by its
  instance: a complete failure is remembered for every renaming of its
  open witnesses and eigenvariables, a failure cut by a bound for the
  instance itself, never a proof, and a sequent equal to an ancestor
  whose witnesses are still open is a loop (`--memo-limit 0` switches
  both off). A first-order goal is searched on several threads
  (`Options::jobs`, the command's `--jobs` and the pool that joins one
  thread by default) as a propositional one is, the two searches of the
  default bias side by side and the choices nearest the root split
  among the threads, where no rule around a choice may ask its premise
  for another answer; the threads share one memo, and the thread whose
  alternative proves a choice hands over the bindings and instances its
  proof rests on. The counts carry over: a predicate with an open atom is
  counted over all its atoms and refuted by `Refutation::UnbalancedSymbol`
  and `OneSidedSymbol` (in JSON `unbalanced_symbol` and
  `one_sided_symbol`, the predicate by its index), which
  `Disproof::check` checks (the first; the second, as `one_sided`, rests
  on the search); `Unbalanced` and the count equation are checked on a
  quantified sequent too, a quantifier counted as its body; the
  classical refutation gives nothing for one. The first-order bounds
  `Options::witness_depth` (`DEFAULT_WITNESS_DEPTH`, 64; `--witness-depth`,
  `search.witness_depth`) and `Options::instances` (`DEFAULT_INSTANCES`,
  65536; `--instances`, `search.instances`), each cutting a branch and
  answering `Reason::WitnessDepth` or `Reason::Instances` (in JSON
  `witness_depth` and `instances`). `Statistics::unifications` and
  `Statistics::bindings` count the unifications of two literals a
  first-order search tried and the bindings of a metavariable it made;
  `prove --stats` prints them for a first-order sequent ("unifications
  tried", "metavariables bound"), and their JSON keys are written only
  where they are not zero, so a propositional search's statistics are
  written as before.
- First-order families (`families::FAMILIES`): `lambek-fo`, Lambek
  sequents in Moot and Piazza's translation into first-order MILL, a
  formula at a pair of string positions (five shapes: chains of `\` and
  of `/`, a product, and the unprovable exchanged chain and reversed
  product); `chain-fo`, `p(z), !∀x.(p(x) ⊸ p(s(x))) ⊢ p(sⁿ(z))` provable
  within `n` copies, and the unprovable goal `p(b)`; `witness-fo`, a
  witness among `n` in MALL1 (`p(a1) & … & p(an)`) and in MILL1, each
  provable and with `q(b)` unprovable. `Instance::witness_depth` names
  the witness depth a family's proof needs, as `Instance::copies` names
  its copy bound.
- The families `wide-m1` to `wide-m4` start at one pair: size 0 was the
  empty sequent, marked provable, which it is not without Mix.
- Families that grow: `Family::growth`, the sizes of a family's growing
  set, from easy to past what the engines decide in a minute, and
  `Family::claim`, the strength or weakness of an engine or of another
  prover its growth shows, set for `wide-m1`, `mix`, `counter`, `chain`,
  `lambek-fo`, `chain-fo` and `witness-fo` and for five new families:
  `contraction`, Franzén's formulas (ILTP's SYJ203 and SYJ209) in ILL,
  one `!` clause used `n + 1` times on a branch, provable, and its
  intuitionistic non-theorem; `pigeonhole`, Cook's pigeonhole principle
  (SYJ202) in intuitionistic affine MALL, `n + 1` pigeons provable and
  `n` not; `euler`, Euler tours through the square of a cycle in IMALL,
  provable, and without one edge, unprovable; `duplicate`, `!(a ⊸ a ⊗
  a), a ⊢ a ⊗ … ⊗ a`, provable in ILL and in classical linear logic; and `reach-fo`, reachability through a
  cycle to its exit at first order, provable, and without the exit,
  unprovable.

- Interactive proving with quantifiers: `Interactive` takes a sequent
  with `∀` and `∃`. `∀R` and `∃L` (`Rule::Forall`) open the formula with
  a fresh eigenvariable; `∃R` and `∀L` (`Rule::Exists`) with the step's
  `Witness` (`Step::witness`, `Step::with_witness`, `Needs::Witness`):
  `Witness::Term`, a closed term given flat as `TermNode`s of the
  sequent's function symbols and the eigenvariables in the goal's
  scope, which `Interactive::witness` reads from text, or
  `Witness::Open`, a metavariable (`FoTerm::Meta`, written `$1`, `$2`,
  …) that a later axiom binds by unification with the occurs check, or
  `Interactive::close` by the search, and that never takes an
  eigenvariable made after it; `undo` unbinds what its step bound. New
  refusals `StepError::NoWitness`, `Witness` and `Eigenvariable`. The
  goals, the derivation so far and its exports print every instance;
  `proof` closes an open witness by a fresh constant and checks the
  proof. The session's JSON (wire level 7) gains `instances`: its
  members past the sequent's occurrences as `[occurrence, frame]`,
  frames, terms (an open witness `{"M": k}`), the metavariables' levels,
  the eigenvariables made, the `bindings` in the order made and the
  `marks`, where the table stood before each step that made entries,
  so that `undo` in a session read back drops what its step made as it
  does in the session written; all replayed and checked when read. A cut
  in such a session (`Interactive::cut`, the command's `cut`) takes a
  formula of the sequent's symbols, quantifiers among them, and of the
  eigenvariables in the goal's scope by their letters: a formula naming
  eigenvariables is held as `∀`s around it and cut on its body's
  instance under a frame of those eigenvariables, as the checker's cut
  on an instance is. The members past the forest move past the cut
  formula's trees and the table's terms past the larger arena, `undo`
  moves them back, and a session without quantifiers gains them with
  its cut formula's; `pop_cut` drops the first-order tables' entries
  no formula left reaches. An eigenvariable out of the goal's scope is
  refused with `StepError::Eigenvariable`, a symbol the sequent has not
  or has with another arity or role with `StepError::CutSymbol` (a
  symbol of another arity was renamed apart before).
  `Interactive::formula_as_written` is `None` for an instance.
  The command's `interact` takes the witness
  after `∃R`/`∀L` (`apply G P RULE [TERM]`) and `close` searches such a
  goal on one thread.
- The classical two-sided view draws first-order derivations, with the
  rules `ClassicalRule::{ForallLeft, ForallRight, ExistsLeft,
  ExistsRight}` (`∀L`, `∀R`, `∃L`, `∃R`) and every instance written
  out; `prove --classical-sides two` prints it, in every format.
- Cut elimination on first-order proofs: `proofs::cut::step` and
  `eliminate` take proofs with instances in every mode that has the cut,
  the cut on a quantified formula or on an instance of one, every case
  reading members through the proof's instance table; the new key case
  `Case::ForallExists` (`forall_exists`, "the key case of ∀ and ∃")
  cuts the `∀` formula's body at the `∃` step's witness, the witness put
  for the eigenvariable throughout the `∀` premise, in a copy where
  another cut still reads it, every `∀` step a step remakes while the
  proof still reads the original given an eigenvariable of its own. A
  copy is charged against `cut::Options::nodes` (and
  `limits.memory_bytes`) before it is made, the step refused past it
  with the last checked proof. `Proof::formula` writes an instance with
  its frame's terms, and `linlog eliminate` writes each step's instance.
  The proof structures still refuse a quantifier, and so does
  `eliminate --net`.
- Rocq certificates of first-order proofs. The Rocq library gains a
  first-order development beside the propositional one: `Terms.v`
  (terms with de Bruijn indices for bound variables and eigenvariables
  named by naturals), `FirstOrderFormulas.v`, `FirstOrderCalculus.v`
  (the one-sided calculus `ll` with `fall_r`, whose eigenvariable occurs
  nowhere in its conclusion, and `fex_r`, whose witness is any closed
  term, Mix, weakening and cut as `Calculus.rules` sets them) and
  `FirstOrderIntuitionistic.v` (the two-sided `ill` with `∀R`, `∀L`,
  `∃R`, `∃L`), which join the trusted base; the proof terms
  (`FirstOrderProofs.v`, the two-sided `iproof` with `IAllR`, `IExL`,
  `IExR`, `IAllL`), their checkers in two passes, the eigenvariables
  introduced and then the set carried to the root
  (`FirstOrderCheck.v`, `FirstOrderIntuitionisticCheck.v`), and the
  theorems `soundness`, `soundness_classical`, `soundness_mix`,
  `soundness_affine` and `soundness_ill` for them, closed under the
  global context. The exporter's linlog kernel writes a derivation of a
  sequent with a quantifier for it, in every mode and with cuts, every
  instance with its frame's terms substituted, with the preludes
  `rocq::LINLOG_FIRST_ORDER_PRELUDE` and
  `rocq::LINLOG_FIRST_ORDER_TWO_SIDED_PRELUDE`; with no kernel named
  such a derivation goes to the library, and NanoYalla still refuses it
  (`Unsupported::Quantifiers`), as do the certificates of
  non-derivability.
- `IllSequent`, a sequent of intuitionistic linear logic as written: its
  hypotheses in the order written and exactly one goal, every formula
  built from `⊗`, `⊸`, `&`, `⊕`, `!`, `1`, `⊤`, `0` and atoms. It is
  built by `IllSequent::parse_within` (and `FromStr`), a strict parser
  that refuses the symbols of classical linear logic (`~`, `^`, `⅋`,
  `par`, `|`, `?`, `⊥`, `bot`) with a hint of what to write instead
  (`ParseError::hint`, a new field), by its JSON form, and by
  `IllSequent::read_one_sided`, the explicit intuitionistic reading of a
  one-sided sequent. It lowers once to the one-sided forest the engines
  take and records every occurrence's side there; `IllSequent::reading`
  is the `Reading` over those sides, equal to `Reading::new`'s.
- The intuitionistic sequent's JSON form, `{"version": 5, "terms": […],
  "atoms": […], "hypotheses": […], "goal": n}`, whose terms are the
  connectives of intuitionistic linear logic alone (a classical tag is
  refused), at wire level 5 (below).
- `InputOptions` (`Settings::input`, in JSON `"input"`), how a front end
  reads its input: `intuitionistic`, an `IllSyntax`, `strict` by default
  (`InputOptions::DEFAULT_INTUITIONISTIC`) or `one-sided`.
- Proof nets of unit-free MELL with `!`-boxes, in classical mode with or
  without Mix: `MellStructure`, its vertices instances of the forest's
  occurrences, a generalised `?` vertex with any number of copies of its
  subformula, each weakening (a `?` without copies) hung on an axiom
  link; built from its `Parts` (`Placed` vertices, links, weakenings),
  validated, or desequentialized from a checked proof
  (`MellStructure::from_proof`); the criterion `is_correct`
  (Danos–Regnier at every box depth, boxes contracted, `?` vertices and
  their auxiliary doors switched over their wires) and `sequentialize`
  into a proof the checker accepts; `MellStructure::exist` says where
  they exist. New `NetError` variants: `Placement` (with `Placement`,
  the fault), `Unattached`, `InBox` (a failure inside a box) and
  `NotMell`, in JSON the kinds `placement`, `unattached`, `in_box` and
  `not_mell`.
- A MELL structure's text form (`Display`: each `?` vertex's copies and
  their boxes, a weakening's link, the links, the verdict) and JSON form
  (`Serialize`, `Deserialize`, `wire::Readable`), read and validated as
  `MellStructure::from_parts` is, within the limits.
- `svg::mell_net` draws a proof net with boxes in both drawings, every
  `!`-box a rectangle around its contents with its `!` and an auxiliary
  door per `?` vertex outside with a copy inside on its bottom border,
  nested boxes nested, the ids `o<n>` (the only vertex of occurrence
  `n`), `v<k>` (vertex `k` of an occurrence with several), `b<k>` and
  `l<x>-<y>`.
- A net's look, `svg::Style::net_look` (`svg::NetLook`, in JSON its
  fields or a preset's name): `premises` (`Premises::Dashed`, the
  default, or `Plain`: the edges into a `⅋` or `?` node and the wires
  drawn as every other edge), `conclusions` (`Conclusions::Raised`, the
  default, or `Bottom`: a `!` conclusion with auxiliary doors on the
  bottom row too, under an edge down from its door) and `why_not`
  (`WhyNot::Node`, the default, or `Rules`: a `?` node drawn as a
  dereliction `?d` under each copy, inside its box, binary contractions
  `?c` and a weakening `?w`, each copy's wire through doors of its
  own), and the presets `NetLook::LINLOG` (the default, the drawing as
  before) and `NetLook::TEXTBOOK` (all three the textbooks' way), named
  by `NetPreset`. The command's `--style svg.net_look=textbook` and
  `svg.net_look.premises=plain`, `…conclusions=bottom` and
  `…why_not=rules`, and the bindings' `styles.svg.net_look`. New types
  only; no enumeration gained a variant.
- The command's `--net` gives a MELL proof (classical, with or without
  Mix) its net with boxes in `prove` and `check`, as text, SVG, PNG and
  PDF, and `--net --format json` writes the net's document, of an MLL
  proof too, in place of the outcome when there is a proof.
- The bindings' `draw_net` draws a finished MELL proof's net with boxes,
  and `view` gives no `net` refusal for it.
- Proof nets with boxes for intuitionistic MELL without units: the net
  of the one-sided sequent, decided by the criterion without Mix
  (`MellStructure::exist` admits intuitionistic mode, affine modes still
  not), sequentialized into a proof the checker accepts in intuitionistic
  mode; `MellStructure::reading` gives a structure's reading. The
  command's `prove --net` and `check --net` and the bindings' `draw_net`
  and `view` give an intuitionistic MELL proof its net, and with
  `svg.essential` `svg::mell_net` draws it as an essential net with
  boxes: the reading's formulas, the input side in the colour `input`,
  every edge, wire and link an arrow toward the goal, a hypothesis `!A`'s
  rules `!d`, `!c` and `!w` in the textbook look. No enumeration gained
  a variant.
- The bindings' `view` (and every call that answers with a view) gives
  each goal formula, in intuitionistic mode, its `reading`: the formula
  as `Reading::formula` writes it in its position, a hypothesis `A ⊸ B`
  where `text` has the one-sided `A ⊗ ~B`; the key is absent in
  classical mode. An added key of a call's answer, no wire level.

- The focused inverse method, `Engine::Inverse` (`--engine inverse`, JSON
  `"inverse"`), in every commutative mode: a forward search that derives
  stable sequents from the axioms upward by rules compiled from the
  sequent's focusing phases, keeps a derived sequent unless a kept one
  subsumes it, and proves the goal once every stable sequent of its
  asynchronous phase is derived. It decides without exponentials, needs
  no copy bound with them, and answers `Unprovable` where its saturation
  ends, with the set it saturated with as the refutation (below); its
  rules, sequents and indices count under the memory bound. Its counters
  are `nodes` (sequents taken up), `memo_hits` (sequents subsumed) and
  `memo_entries` (sequents kept). The dispatch gives it a turn (under
  "Changed").
- The refutation by a saturated set, `Refutation::Saturated` with
  `search::Saturated` (the atoms' polarity and the sequents) and
  `search::Stable` (a sequent's linear and unrestricted zones and whether
  it absorbs), in JSON `{"kind": "saturated", "positive": […],
  "sequents": [{"linear": […], "unrestricted": […], "absorbs": …}, …]}`,
  which raises the wire level to 3 for a disproof or an unprovable
  outcome that holds it. `Disproof::check` verifies it apart from the
  search: the rules of the focused calculus read off the sequent again
  and applied to every choice of the set's sequents, each sequent derived
  one the set stands for or one no proof holds, Mix too where the mode
  has it, and a stable sequent of the goal's phase the set does not
  give; the faults `RefutationFault::Unshaped`, `NotClosed` and `Filled`
  say what fails. The search checks it before answering, as it checks a
  proof, unless `Options::check` is off; the counts' and the classical
  refutations are preferred where they apply. The Rocq export refuses it
  (`export::rocq::Unsupported::Saturated`).
- The Rocq library `rocq-linlog` under `rocq/` (logical path `Linlog`,
  Rocq 9 with its standard library and nothing else): formulas in
  negation normal form over an abstract atom type, the one-sided cut-free
  calculus of classical linear logic as a plain inductive with binary Mix
  and general weakening as parameters, linlog's proof term as a datatype,
  a checker as a function and the theorem that a term the checker accepts
  proves its sequent, closed under the global context. The flake builds
  it as a package and compiles the certificates under `rocq/tests/`
  against it.
- In the Rocq library, the theorems for Mix and for affine logic
  (`soundness_mix`, `soundness_affine`), and the two-sided calculus of
  intuitionistic linear logic and its affine variant (`ill`, over
  `iformula` named as in Yalla's `nanoill.v`), with two-sided proof terms,
  their checker and the theorem `soundness_ill`, each closed under the
  global context.
- A second Rocq kernel in the exporter, linlog's own library:
  `rocq::Kernel` (`NanoYalla`, `Linlog`), `rocq::Options::kernel` (JSON
  `styles.rocq.kernel`, `"nanoyalla"` or `"linlog"`) and the command's
  `--kernel`, with the version `rocq::LINLOG` and the preludes
  `LINLOG_PRELUDE` and `LINLOG_TWO_SIDED_PRELUDE`. A certificate for it
  states the sequent in the mode's calculus, two-sided for an
  intuitionistic derivation, and proves it from the proof term; Mix and
  affine weakening have one. The flake's `rocq-linlog` check compiles
  the exporter's certificates for it.
- In the Rocq library, the theorems `mix_to_ll` and `mix_to_classical`
  (`Mix.v`): a derivation with binary Mix gives one of `?(⊥⊗⊥), Γ`
  without Mix and without cut. Beside the library, under `rocq/bridges/`,
  the equivalences of `ll classical` with NanoYalla's `nanoll.ll` and of
  `ill intuitionistic` with Yalla's `nanoill.ill`, which the flake's new
  `rocq-bridges` check compiles against the two kernels' files as its
  inputs pin them (Yalla is a new input), each closed under the global
  context.
- In the Rocq library, the refutations (`Refutations.v`): every
  derivable sequent of `ll R`, for every `R`, reads as a tautology
  classically (`ll_tautology`); without weakening an atom's excess over
  the additive alternatives contains zero (`balance`) and a
  multiplicative sequent with units has the formulas the count equation
  asks for, at least that many with Mix (`count_equation`); and the
  two-sided calculus embeds in the one-sided one (`ill_ll`), so that
  each gives `~ ll R Γ` or `~ ill R Γ C` from an assignment, an atom or
  nothing, tested by computation (`classical_refutation`,
  `unbalanced_refutation`, `equation_refutation` and their two-sided
  forms), all closed under the global context.
- A classical refuter after the search: where a search did not prove its
  goal, an assignment of truth values to the atoms under which the goal
  read classically is false (`!` and `?` erased, `⊗` and `&` as "and",
  `⅋` and `⊕` as "or", `1` and `⊤` true, `⊥` and `0` false), which no
  provable sequent has in any mode, makes an unknown verdict unprovable
  and gives an exhausted search a certificate: the refutation
  `Refutation::Classical` (`search::Classical`, its `assignment` a value
  per atom), bounded by `search::Options::classical_work` (JSON
  `classical_work`, default `Options::DEFAULT_CLASSICAL_WORK`, 2²⁶ terms
  evaluated, zero for none) and the memory bound, and the command's
  `--classical-work N`, which the harness takes too. No verdict a search
  gave changes, nor its counters.
- `Disproof::check`, which verifies a classical assignment, an
  unbalanced atom and the count equation against the sequent, the goal
  and the mode, sharing no code with the search, and its error
  `Error::Refutation` with `search::RefutationFault` (codes
  `invalid_refutation`, kind invalid, and `uncertified`, kind
  unsupported, for the refutations that rest on the search). The
  harness writes its result in the `checked` column of an unprovable
  row.

- Certificates of non-derivability in the exporter, for linlog's
  library: `rocq::write_disproof` writes, for a disproof whose refutation
  `Disproof::check` accepts, the lemma `~ ll R Γ` (or `~ ill R Γ C` in
  intuitionistic mode) proved by the library's refutation theorem by
  computation, with the prelude `rocq::LINLOG_REFUTATION_PRELUDE`, and
  `rocq::write_not_valid` the lemma `~ (forall a b : Prop, F)` for an
  ordinary sequent whose image an assignment refutes, by instantiating
  its atoms; `rocq::Unsupported` gains `Disproof` (NanoYalla named) and
  `NotValid`. The command's `--format rocq` writes them for an
  unprovable or not valid verdict, and says on standard error where
  there is none.
- The WebAssembly bindings `linlog-web` (`web/`, not published): the
  library for a web page, compiled for `wasm32-unknown-unknown` without
  `parallel`, behind calls of JSON in and JSON, SVG or bytes out over a
  session, the settings (read over a browser tab's defaults:
  `linlog_web::tab`) and a search's outcome. The flake builds the module
  with the `wasm-bindgen` CLI of the version `Cargo.lock` pins and
  `wasm-opt`, with the notices of the crates inside it, as the package
  `linlog-web`, and runs its tests under Node (`web-wasm-tests`).
- The flake check `web-core-wasm32` runs the library's tests of the
  proof checker, the size estimate, the memo and the memory account on a
  32-bit target, `wasm32-wasip1` under wasmtime.

- The cyclic mode: cyclic linear logic, multiplicative and without
  units, where a one-sided sequent is a sequence of formulas read up to
  rotation and nothing else is exchanged. `Order` (`Commutative`,
  `Cyclic`), `Mode::CYCLIC` with `Mode::order` and `Mode::is_ordered`,
  the mode word `cyclic`; `Mode::check` refuses an ordered mode with
  weakening or Mix (`Error::OrderedMode`, code `ordered_mode`), and the search a sequent beyond unit-free MLL in
  it (`Error::OrderedFragment`, `ordered_fragment`). The command's
  `--cyclic` (on `prove`, `check`, `interact`, `seq print` and `seq
  json`), and `cyclic` as a batch entry's mode.
- `Sequent::parse_ordered`, which reads a sequent for an order: in the
  cyclic one the dual of a product reverses it (`~(A * B)` is `~B par
  ~A`) and the formulas left of `|-` come reversed, each negated.
- The cyclic checker: `Proof::check(Mode::CYCLIC)` accepts the axiom,
  `⊗` and `⅋` only (`Fault::Forbidden` otherwise) and a `⊗` only when its
  premises' contexts are the two arcs on either side of it
  (`Fault::Order`).
- Planar proof nets: `Criterion::order` and `Criterion::CYCLIC`, under
  which `is_correct` asks besides that no two links cross in the cyclic
  order of the literals; `ProofStructure::is_planar` decides that alone,
  with the witness `NetError::Crossing`. The net engine decides the
  cyclic mode, its default there, by a linking search that admits a
  link only when it crosses no link made and the literals between its
  ends are balanced; it decides goals off the roots in that mode.
- Step-by-step proving in the cyclic mode: a `⊗` splits its goal into
  the two arcs on either side of it, at a cut point (`Step::at`,
  `Split::At`) or by positions that make them, any other split being
  `StepError::Exchange`.
- `rocq::Unsupported::Ordered`: the Rocq exporter refuses a derivation
  or a disproof of the cyclic mode, whose calculi are commutative.
- The Lambek calculus, intuitionistic and in the cyclic order, without
  empty antecedents (`Mode::LAMBEK`, the mode word `lambek`) or with them
  (`Mode::with_empty_antecedents`, `lambek-star`), and
  `Mode::allows_empty_antecedents`. The command's `--lambek` and
  `--empty-antecedents` (on `prove`, `check` and `interact`) and `seq
  print --lambek`.
- The divisions `A \ B` and `B / A` in the ordered parse, as `A -o B` and
  `B par ~A`.
- The ordered reading of the Lambek calculus: `Reading::of_mode`,
  `Reading::is_ordered`, `IllConnective::Under` and `Over`, and
  `ShapeError::EmptyAntecedent` for a sequent without a hypothesis where
  the mode needs one; derivations, LaTeX, Typst and SVG print its
  sequents two-sided in their order with the divisions.
- The Lambek checker: `Proof::check(Mode::LAMBEK)` is the cyclic check
  under the ordered reading with one output and, without empty
  antecedents, an input in every sequent (`Fault::EmptyAntecedent`).
- The rules of the divisions, `\L`, `\R`, `/L` and `/R`: `Named::division`
  and the closed enum `Division` (`Under`, `Over`); a step that would
  leave a premise without a hypothesis is `StepError::EmptyAntecedent`.
- The essential engine decides the Lambek calculus, its default and only
  engine there, by a search for planar essential nets that, without
  empty antecedents, asks the body of every implication but the goal
  for a hypothesis of its own; it decides goals off the roots in the
  mode. The net engine refuses the mode.
- `svg::Style::net` (`svg::NetDrawing`, `--style svg.net=compact`, JSON
  `styles.svg.net`, `"formulas"` by default or `"compact"`): the compact
  drawing of a proof net writes only its literals and its conclusions,
  every inner `⊗` and `⅋` a small circle as before, for a net too wide
  to write every subformula, and now writes the conclusions under the
  edges hanging from its bottom layer.
- Four more math fonts for the drawings beside Euler Math, the default:
  `svg::MathFont` (`Euler`, `StixTwo`, `Garamond`, `Asana`,
  `NewComputerModern`; JSON `"euler"`, `"stix-two"`, `"garamond"`,
  `"asana"`, `"new-computer-modern"`) names one, and `MathFont::font` is
  its `svg::Font`, laid out by the new `svg::Advances` variants
  `StixTwo`, `Garamond`, `Asana` and `NewComputerModern`, each the
  advances of the font's file (STIX Two Math 2.12, Garamond-Math
  2022-01-03, Asana Math 0.962, New Computer Modern Math 7.1.1 Book) as
  Euler Math's are. A net drawn with its formulas starts its edges below
  the font's deepest descender. `svg::Font` reads
  in JSON as a font's name too (`styles.svg.font` `"stix-two"`), so a
  front end chooses a font by name; the command's `--font NAME` sets it,
  embeds the four fonts with their licences beside Euler Math
  (`cli/fonts/`), and renders a PNG or PDF with the font the drawing
  names. Output with the default font is unchanged.
- The invertible rules applied without being asked, in every mode:
  `Interactive::invert(goal, &limits)` and `invert_all(&limits)` apply
  each rule invertible in the session's mode (`⅋`, `⊥`, `&`, `⊤`; `!`
  when every other formula is a `?`; `1`, `⊗` and the axiom when their
  formulas are the whole goal) until none applies, each application an
  ordinary step that `undo` retracts, the steps' inferences bounded by
  `limits.memory_bytes` and returning an `Inverted` (the steps taken,
  the goals left open, the refusal that stopped it if one did);
  `Interactive::is_invertible(sequent, &step)` says whether a step is
  one of them. The options value `proofs::interactive::Options` (its
  field `invertible`, off by default) is the new `Settings::interactive`
  (`{"interactive": {"invertible": true}}`). `linlog interact` takes
  `--invertible` and the command `invertible on|off`, and its `undo`
  retracts a step together with the invertible steps that followed it.
  The web bindings apply them in `start` and `apply` when the setting
  is on (`inverted` in the answer), add the call `invert_all`, list in
  the view the inferences an invertible rule closed (`invertible`), and
  say of a hint whether its rule is invertible.
- `ClassicalSequent`, a sequent of classical linear logic as written: the
  one-sided forest the engines take and, per occurrence, a `Written`
  (its `side` of `⊢` and whether it `is_implication`), recorded where the
  text is read (`ClassicalSequent::parse_within` and `FromStr`, the
  parser extended by a record it fills) and never inferred, or the
  record of a one-sided sequent as its syntax writes it
  (`ClassicalSequent::read_one_sided`). It prints back as written, up
  to brackets and double negations (`Display`, `formula`), and says
  where a negation was written (`is_negated`).
- `TwoSided`, the two-sided view of a classical derivation, built from a
  one-sided `Derivation` and the record (`TwoSided::new`, refused past
  `limits.derivation_bytes`): every sequent split by the side of `⊢` its
  formulas were written on, every rule named two-sided
  (`ClassicalRule`: `⊗L`, `⊗R`, `⅋L`, `⅋R`, `⊸L`, `⊸R`, `1L`, `1R`,
  `⊥L`, `⊥R`, `&L₁`, `&L₂`, `&R`, `⊕L`, `⊕R₁`, `⊕R₂`, `⊤R`, `0L`, `!L`,
  `!R`, `!c`, `!w`, `?L`, `?R`, `?c`, `?w`, `mix`, `wk`), and a negation
  step `~L` or `~R` inserted where a negation was written. It draws as a
  text tree (`Display`, `write_text`) and through `Drawable::Classical`
  (a new variant) in LaTeX, Typst and SVG; Rocq certifies the one-sided
  derivation it shows.
- `ViewOptions::classical`, a `ClassicalSides` (`one`, the default
  `ViewOptions::DEFAULT_CLASSICAL`, or `two`; in JSON `{"view":
  {"classical": "two"}}`): whether a front end that holds a classical
  sequent as written draws its derivation two-sided. The command's
  `--classical-sides one|two` (on `prove`, a batch included, and
  `check`) sets it.
- `Interactive::of_classical` starts a session of a classical sequent as
  written and keeps the record (`Interactive::writing`), so that its
  derivation can be drawn two-sided at any point of the proof; a
  session's JSON carries it as `"written": {"inputs": […],
  "implications": […]}`. The web bindings' `start` keeps it for a typed
  sequent of a commutative classical mode, and `export` and `draw` draw
  such a session two-sided when the settings' `view.classical` is
  `"two"`.

- The web bindings take first-order sessions: a step's JSON has
  `witness`, the term of an `∃R`/`∀L` step as text or left out for an
  open witness; `rules` marks such a rule `"witness": true`; `hint` on
  a goal of instances closes it in a copy of the session and reads its
  first steps, with the witness the search chose where the session
  reads it; a goal formula's `reading` and an elimination's cut formula
  write an instance with its terms; the view's `opened` lists every
  closed quantifier step with the term it took (`null` for a witness
  left open), and `forest`, the forest's length; the tab's `limits.terms` is a million, as its
  occurrences. `Interactive::opened_term` returns the term a quantifier
  step opened its formula with, as it took it. The SVG drawing of a
  first-order session's two-sided derivation placed an instance by its
  member's number and panicked; it places it by its occurrence.

### Wire forms

- Level 7 adds the first-order sequent: `symbols` (each a name with its
  arity), `fo` (the terms: `{"B": n}` a bound variable, `{"F": [symbol,
  [terms]]}` an application), an atom as `{"P": [symbol, [terms]]}`, and
  the terms `{"∀": body}` and `{"∃": body}` with their variables' names
  in `binders`; the intuitionistic sequent's form takes the same keys
  and tags. A document
  whose sequent is first-order is written at level 7; a propositional
  sequent's bytes are as before. A proof of a first-order sequent adds
  the steps `{"∀": [member, eigenvariable, premise]}` and `{"∃":
  [member, term, premise]}` and `"instances": {"members": [[occurrence,
  frame]], "frames": [[parent, term]], "terms": […]}`, the proof's terms
  in `fo`'s form with an eigenvariable as `{"E": k}`, still at level 7.

- Level 2: the refutation kind `classical`, `{"kind": "classical",
  "assignment": [true, false, …]}`, the modes `"cyclic"`, `"lambek"`
  and `"lambek-star"`, the rule names `"\\L"`, `"\\R"`, `"/L"` and
  `"/R"`, the faults and shape errors `"empty_antecedent"`, and a proof
  structure's `"order": "cyclic"`. A disproof or an unprovable outcome
  that holds the refutation, and a document of the cyclic mode or
  order, are written at level 2; every other document stays at level 1,
  byte for byte.
- Level 3: the refutation kind `saturated`, `{"kind": "saturated",
  "positive": ["atom" | "dual", …], "sequents": [{"linear": […],
  "unrestricted": […], "absorbs": …}, …]}`; a disproof or an unprovable
  outcome that holds it is written at level 3.
- Level 4: the proof structure with boxes, `{"version": 4, "sequent":
  {…}, "mix": false, "vertices": [[occurrence, parent, box], …],
  "links": [[x, y], …], "weakenings": [[q, l], …]}`, whose links name
  vertices; it is always written at level 4, and the proof structure's
  reader refuses a document with `vertices`.
- Level 5: the intuitionistic sequent, `{"version": 5, "terms": […],
  "atoms": […], "hypotheses": […], "goal": n}` with the terms `V`, `1`,
  `0`, `⊤`, `⊗`, `⊸`, `&`, `⊕` and `!`; it is always written at level 5.
- Level 6: the cut. A sequent's cut formulas, `"cuts": [a, …]`, the term
  of each cut formula `A` (its dual is in the arena, which the reader
  finds again), and the proof's tag `{"cut": [a, l, r]}`; a session's
  cut inference names its formula, `"cut": a`. A document whose sequent
  holds a cut formula (a proof with a cut, a session with one, an
  outcome of a goal of one) is written at level 6; every other document
  stays as it was.
- Level 6: the cut links of proof structures, `"cuts": [[x, y], …]`, with
  and without boxes, written only where there are some (whose sequent
  holds the cut formulas), and a cut link's end in a structure with
  boxes as `[occurrence, null, box]`.
- A session's optional key `written`, how its sequent was written, raises
  no level: a reader that ignores it reads the session as it is, and
  draws it one-sided.
- The essential-net engine for IMLL without units, `Engine::Essential`
  (`--engine essential`, JSON `"essential"`): the net engine's linking
  search over the essential net the intuitionistic reading directs,
  keeping directed acyclicity by a transitive closure of bits with an
  undo log, counted under the memory bound, and testing the dominator
  condition; it runs only when forced. `ProofStructure::is_essential`
  decides that criterion independently of any search, with the
  witnesses `NetError::DirectedCycle` and `NetError::NoDominator` (and
  `NetError::NotIntuitionistic` for a sequent without a reading), and
  `ProofStructure::reading` gives a structure's reading.
- The drawing of an essential net in the SVG export: `svg::Style`'s
  `essential` (`--style svg.essential=true`, off by default) draws the
  net of an intuitionistic sequent with every edge an arrow from the
  hypotheses toward the goal, the input side in the colour `input`, the
  nodes as `⊸` and `⊗`, each implication joined to its antecedent by a
  dotted line in the colour `dominator`, and the essential criterion's
  witness highlighted.

- `Proof::onto(forest, goal)` carries a proof over to a goal of another
  forest whose formulas are the proof's conclusions up to their order,
  every occurrence renamed to the one in its place, refusing with the new
  `Error::SequentMismatch` (code `sequent_mismatch`, `details.proof` and
  `details.goal` the formulas only on each side) when they differ;
  `Interactive::fill(goal, &proof, …)` grafts such a proof, another
  session's, as one step, checked in the session's mode as `close_with`
  checks; the bindings' `fill(session, goal, source, settings)` takes the
  source as a finished session.
- Proofs with cuts. A sequent holds its cut formulas beside its
  conclusion (`Sequent::cuts`, each a formula `A` and its dual `A⊥`,
  added by `Sequent::add_cut`), which its forest numbers after the
  conclusion, `A` then `A⊥` (`Forest::cut_pairs`, `has_cuts`,
  `is_conclusion`, `dual`, `is_cut_formula`); the reading reads `A` as
  output and `A⊥` as input (`ShapeError::CutFormula`, kind
  `cut_formula`, where it cannot). The proof term `Node::Cut(a, l, r)`
  (tag `cut`) and the checker's rule: the first premise has `a`, an
  occurrence of a cut formula `A`, and the second its dual, in every
  commutative mode, intuitionistically whichever premise proves it;
  `Fault::NotACut` (kind `not_a_cut`) for a cut on another member, and
  `Fault::Forbidden` in an ordered mode. `Rule::Cut` and
  `ClassicalRule::Cut` name it in every view and export (`cut`), and
  `Inference::cut` gives a cut inference's formula. The read-back to LK
  and LJ refuses it, and NanoYalla's certificate too (`Unsupported::Cut`): the
  kernel by default is then linlog's library, whose calculi have the cut
  as an optional rule.
- Cut elimination on proof terms, `proofs::cut`: `step(&proof, mode,
  at, &options, &limits, stop)` reduces the cut at `at`, or the one the
  strategy picks, checks the proof it makes in `mode` and returns it with
  the `Step` (`cut`, the cut formula's occurrence `formula`, its `Case`
  and the node `above` it acted at); `eliminate` takes steps until no
  cut is left or a bound is reached, gives every step and the proof it
  was taken on to a callback, and returns the `Elimination`: the
  last proof reached, which the checker accepted, the steps, and the
  `End` (`CutFree`, or `Refused` with the refusal). Every proof made lives
  over the same forest and cuts only occurrences of the cut formulas'
  trees. The cases (`Case`): `Axiom`, `TensorPar`, `OneBot`, `WithPlus`,
  `Weakening` (also affine weakening, which erases the other premise),
  `Dereliction`, `Contraction`, `BoxIntoBox` and `Commutative`; the
  strategy `Strategy::Topmost`, the cut with no cut above it. Every mode
  with the cut: classical, affine, with Mix, intuitionistic (every
  proof made stays intuitionistic).
- `cut::Options` (`Settings::cut`, in JSON `"cut"`): `strategy`, `steps`
  (`DEFAULT_STEPS`, 10 000) and `nodes`, the most nodes of a proof a
  step makes (`DEFAULT_NODES`, 100 000); `Refusal::Elimination` with its
  `Bound` (`steps` or `nodes`), code `elimination_limit`, settings keys
  `cut.steps` and `cut.nodes`; `Phase::Eliminate`; `Error::NoCut` (code
  `no_cut`, kind unsupported) for a step asked of a node that is no cut
  or of a cut-free proof, and `Error::Reduction` (code
  `reduction_rejected`, kind defect) were the checker ever to refuse a
  proof a step made. `Proof::is_cut_free`.
- The record of an elimination, written only: `{"version", "proof",
  "steps": [{"cut", "formula", "case", "above"}], "end"}`, `end`
  `cut_free` or `refused` with the `reason` in an error's form, at wire
  level 6.
- Cut links in proof nets. `ProofStructure::with_cuts(forest, criterion,
  &cuts, &links)` and `Parts::cuts` join an occurrence of a cut
  formula's tree to its dual (`cuts`, `cut_partner`, `is_cut_free` on
  both structures, `ProofStructure::vertices`); the vertices of an MLL
  structure over a sequent with cut formulas are the conclusions' trees
  and those above the cut links (`vertex` answers `None` for the
  others). `Placed::parent` is an `Option`, `None` for an end of a cut
  link (`Placed::cut`). Both criteria take a cut link as a `⊗` without
  a conclusion; `from_proof` reads a proof's cuts as cut links (in
  classical mode with or without Mix and in intuitionistic mode), and
  `sequentialize` gives a proof with a cut for each. `Placement::Cut` and
  `Placement::Uncut` (kinds `cut` and `uncut`) for a cut link that is
  none and for a link to a vertex above no cut link.
- Cut elimination on proof nets, `nets::cut`: `step(&net, at, &options,
  &limits, stop)` reduces the cut link at the vertex `at` (either end), or
  the strategy's, the first of the net's list, checks the net it makes
  with the criterion and returns it with the `Step` (`cut`, the link's
  two ends, the `Case`, and the vertex `above` it acted at); `eliminate`
  takes steps until no cut link is left or a bound is reached, gives
  every step and the net it was taken on to a callback, and returns the
  `Elimination` (the net reached, the steps, the `End`). The cases
  `Axiom`, `TensorPar`, `Weakening` (a box erased), `Dereliction`,
  `Contraction` (a box duplicated) and `BoxIntoBox`, on `ProofStructure`
  and `MellStructure` (the sealed trait `nets::cut::Net`); `cut::Options`
  bounds it, `nodes` counting a net's vertices (`Bound::Vertices`, key
  `cut.nodes`). `NetError::NoCut` (code `no_cut`) for a vertex at no cut
  link or a net without one, and `NetError::Reduction` (code
  `reduction_rejected`, kind defect) were the criterion ever to refuse a
  net a step made.
- `linlog eliminate --net PROOF`: the elimination on the proof's net, in
  every mode a net exists for: the steps, each with its case and cut
  formula, the end and the net reached as text, every cut link a line
  `cut A[x] — A⊥[y]`; with `--format svg`, `png` or `pdf` and `--output
  FILE`, a drawing per step, `FILE-k` the net step k was taken on with
  its cut link and what it acts on highlighted, `FILE` the net reached;
  with `--format json` the record `{"version", "net", "steps": [{"cut":
  [x, y], "case", "above"}], "end"}` (`Serialize` for
  `nets::cut::Elimination` of either structure). `--node-limit` counts
  a net's vertices there, and `--drawings-limit SIZE`
  (`cut::Options::drawn_bytes`, key `cut.drawn_bytes`, 256 MiB by
  default, `DEFAULT_DRAWN_BYTES`) bounds the bytes the steps' drawings
  take together: past it the steps go on undrawn and a line says from
  which.
- Drawings of nets with cut links and of the steps of an elimination:
  `svg::net` and `svg::mell_net` draw a cut link as a bar under its two
  formulas (id `c<x>-<y>`), at the bottom after the conclusions or
  inside its box; `svg::net_step` and `svg::mell_net_step` draw the net
  a step was taken on with the cut link reduced and what the step acts
  on (the axiom links at its ends, its `⊗` and `⅋`, the box of its `!`,
  the box it enters, the wire to its copy) in the highlight colour.
- `linlog eliminate PROOF`: checks a proof file in the flags' mode and
  eliminates its cuts, printing each step's case and cut formula and the
  derivation of the proof reached, or the record with `--format json`;
  `--steps N` (one step with `--steps 1`) and `--node-limit N` bound it.
- The search decides a goal of a forest with cut formulas on a cut-free
  forest of the goal's formulas, so no engine sees a cut formula and
  every proof found is cut-free, carried back to the goal and checked
  there.
- The cut in interactive proving: `CutFormula`, a cut formula as typed
  (`CutFormula::parse_within(text, mode, &input, &limits)` reads it in
  the syntax of the mode, or `From` an `IllSequent`, a
  `ClassicalSequent` or a `Sequent` `⊢ C`), and
  `Interactive::cut(goal, &formula, left, &limits)`, which adds the cut
  formula to the session's sequent and opens the premises `Γ, A` and
  `Δ, A⊥`, the occurrences of `A` and `A⊥` recorded as typed (right and
  left of `⊢`, intuitionistic sides as written); `undo` takes a cut back
  with its formula; `Error::CutFormula` (code `cut_formula`) for text of
  another number of formulas than one. The command's `interact` has
  `cut G C [P…]`, and the bindings' `cut(session, goal, formula, left,
  settings)`.
- `Inference::node`: in a derivation of a proof term, the node of the
  term whose rule an inference shows (which `Inference`'s equality
  leaves out), which turns a node (a cut a step
  reduced) into the inferences drawn for it.
- Cut elimination in the web bindings, on a finished session's proof:
  `cuts(session, proof, settings)` lists a proof's cuts with their
  formulas and the drawn inferences that show them, beside its
  derivation drawn; `cut_step(session, proof, at, settings)` takes one
  step, the cut `at` or the strategy's, and answers its case, the ids of
  the drawing to highlight and the proof made; `eliminate(session,
  proof, settings)` runs to the end within `cut`, each step with its
  drawing within `cut.drawn_bytes`. On nets `net_cuts`, `net_cut_step`
  (`at` a vertex at either end of a cut link) and `net_eliminate`, the
  net with boxes where the proof cuts an occurrence twice, each step's
  net drawn with its marks. The tab's `cut.drawn_bytes` is 16 MiB
  (`tab::DRAWN_BYTES`).
- `Interactive::formula_as_written(member)`: a formula as the session's
  sequent was written, a cut formula as typed. The bindings' view gives
  each cut's `inference` and `formula` as typed (`cuts`), with which a
  page replays a session's cuts.

- Prebuilt: each release on GitHub carries the command for Linux (one
  binary linked statically against musl, in an archive, a `.deb`, an
  `.rpm` and an Arch Linux package, for x86_64 and aarch64), macOS
  (Apple silicon and Intel) and Windows (x86_64 and ARM64), with their
  SHA-256 sums in `SHA256SUMS` and an attestation of their build; none
  is signed. Every package of the command carries
  `THIRD-PARTY-NOTICES.txt`, the licences of the crates compiled into it.
  The manual's Installation chapter says how to install, update and
  remove it on each system, and a workflow runs its commands there.

### Changed

- The Rocq library `rocq-linlog` (added under "Added" above, under
  `rocq/` until 2026-11-02) lives in a repository of its own,
  [linlog-prover/rocq-linlog](https://github.com/linlog-prover/rocq-linlog),
  with its tests, its certificates, its bridges to NanoYalla and Yalla
  and its own changelog, started from the tree as it stood here. The
  flake takes it as the pinned input `rocq-linlog` and compiles the
  exporter's certificates for `--kernel linlog` against it (the check
  `rocq-linlog`, which requires each certificate's assumptions closed);
  the package `rocq-linlog`, the check `rocq-bridges` and the input
  `yalla` are that repository's now. Its version is linlog's: 0.2.0,
  released beside this one, on Rocq 9.0 to 9.3, installable by opam
  and by Nix.
- `forall` and `exists` are no longer refused as reserved words: they
  are the quantifiers' keywords, and `ParseError::reserved` is never set.
- The hint of a parse error of the strict intuitionistic syntax alone
  gets the command's `--ill-syntax` note; the hints of the first-order
  syntax (a symbol's arity or role, a keyword in a term's place) stand
  on their own.

- A proof net with cut links is drawn under the intuitionistic reading
  with `svg::Style::essential`, as a cut-free one is (the cut formula's
  tree an output one, its dual's an input one), so that every step of an
  intuitionistic elimination is written in its syntax; it was drawn as
  the one-sided net, with classical symbols.
- `Interactive::new` and `within` start from a sequent's conclusion,
  its cut formulas left out: a session's cut formulas are those of its
  cuts, and a session started on a sequent with cut formulas was refused
  when read back.
- `Placed::parent`, of the structures with boxes added in this release,
  is an `Option<VertexId>`, `None` for an end of a cut link (a vertex of
  a structure with boxes that stands for a cut formula's occurrence and
  is no premise); `Placed::new` keeps its signature, and `Placed::cut`
  makes an end of a cut link. Naming a cut link's other end as the
  parent instead would let a walk down the parents go round between the
  two ends without end.
- The command reads the text of an intuitionistic sequent (`-i`, and a
  batch's intuitionistic entries) in the syntax of intuitionistic linear
  logic alone: a `~`, `^`, `par`, `|`, `?` or `bot` is refused with a
  line saying what to write instead. `--ill-syntax one-sided` (the
  settings key `input.intuitionistic`, on `prove`, `interact` and `seq`)
  reads the classical syntax as before, `~(A * ~B), A |- B` as `A ⊸ B, A
  ⊢ B`. A JSON input in intuitionistic mode may be the intuitionistic
  sequent's form as well as the one-sided one, and `seq json -i` writes
  the former. The bindings' `start` and `prove` read a typed sequent of an
  intuitionistic mode the same way, by the settings' `input`, and the new
  call `read_as(input, mode, settings)` reads one for a mode.
- Every output of an intuitionistic sequent writes it as it was written,
  two-sided, with no classical symbol: the refutations' phrases (an atom
  occurring positively more often than negatively, the count equation
  over `⊗` and `⊸` on either side, the classical reading with `⊸` as
  implication), the checker's reports through `describe` (the sequents of
  a failed node two-sided, the rules by their two-sided names) for a
  proof, derivation or session in an intuitionistic mode and wherever
  `Described::in_mode` (new) names one, `Error::SequentMismatch`'s
  formulas, `Interactive::formula`, `Derivation::formula` of a two-sided
  derivation and `Proof::formula` of a proof that records an
  intuitionistic mode, a net's text through `ProofStructure::written_in`
  and `MellStructure::written_in` (new), and an essential net's SVG title
  and description. The command and the bindings draw the net of an
  intuitionistic proof as its essential net, and the bindings' goal
  formulas have `text` as `reading` writes it.

- A net with boxes in linlog's look keeps the wires that run level
  between the same two rows apart, those bound for different `?`
  vertices included, half the node radius at least, the rows drawn as
  far apart as that needs, as the textbook look already did; before,
  two such wires could share one line.
- linlog has a logo, the word with its i drawn as `⊸` stood on end,
  under CC BY 4.0: the README (and so crates.io), the library's rustdoc
  (its logo and favicon) and the Rocq library's new README show it, by
  URL from linlog-prover.github.io, which serves the files.
- The bindings' settings take a string where the tab's settings hold
  an object, such as a math font's name for `styles.svg.font`; an array
  there is still refused.
- Two rows of the dispatch give the inverse method turns beside the
  default engine: on an intuitionistic goal with an exponential (beside
  the two-sided engine) and on a goal under Mix without weakening
  (beside the focused engine), the engine searches alone for 2²⁴ of its
  units of work, then the inverse method takes a turn each time the
  engine's own work has doubled, from its start, of at most a thirty-second
  of that work in its own units, on the same thread, inside a poll of
  the engine, which goes on where it stopped unless the turn decided:
  the turns together do at most a sixteenth of the engine's work. Only on a search of one
  thread with no engine forced (a race's single side takes it); the
  outcome names the engine that answered, and `Statistics::work` counts
  both.
- A compact proof net's premise edges begin below each literal at the
  deepest descender of the drawing's math font, as the drawing with
  formulas does, instead of a tenth of an em below its baseline, inside
  every font's descenders: 210 thousandths of an em for Euler Math, 235,
  290, 286 and 250 for STIX Two Math, Garamond-Math, Asana Math and New
  Computer Modern Math.
- A proof net is drawn as the literature draws one: every subformula
  occurrence of its conclusions written as its formula, where it stands
  in its conclusion, each `⊗` or `⅋` link joining its two premises to
  its conclusion written below them with the connective small at the
  joint, the literals along the top under the axiom links' arcs, and the
  sequent's formulas at the bottom in its order, a literal among them
  reached by its link's arc. An essential net writes every formula as
  the reading reads it. The ids `o<n>` (now each occurrence's formula)
  and `l<m>-<n>` are kept. This is `--format svg --net`, the PNG and PDF
  of a net and the web bindings' `draw_net`.

- In a label table of the exports, `\` and `/` are symbols, set as the
  divisions are: LaTeX `\backslash` and `/`, Typst `backslash` and
  `slash`.

- The command refutes by the classical reading by default: a sequent
  whose search ended unknown (at the time limit, a copy bound or another
  limit) and whose classical reading an assignment falsifies is
  unprovable (exit status 1); `--classical-work 0` gives the search's
  word alone. The refutation runs after the search, past the time limit,
  within its bound of work, and Ctrl-C stops it.

- With no kernel named, `--format rocq` writes a proof of Mix, affine or
  intuitionistic logic for linlog's library, as the mode chooses; a
  classical proof is written for NanoYalla exactly as before, and
  `--kernel nanoyalla` keeps the NanoYalla certificate of an
  intuitionistic proof and the refusal of Mix and affine weakening.
- Unit-free MLL and IMLL go to the focused engines by default, which
  measured faster than the net engine on every sequent both decide
  (`wide` sequents 1.07 to 3.6 times, the Horn encodings of Partition
  and 3-Partition up to 32 000 times); the net engine takes a sequent
  whose atoms occur once with each sign over where they stop at their
  recursion limit or memory bound (a chain
  of thousands of implications, `wide` with 4 096 literals). Before, MLL
  with no literal more than twice went to the net engine.
- The net engine breaks the symmetry of equal literals and equal
  subformulas that are conclusions or leaves of one tree of `⊗` alone
  or `⅋` alone, and checks each atom's balance over its skeleton's
  components: Lincoln's MLL 3-Partition with bins of five is refuted in
  227 nodes instead of 16 million, Matsuoka's Partition with three
  items in 391 instead of 269 950.
- The closed enums of the calculus gain the cut (`Node::Cut`,
  `Rule::Cut`) and the quantifiers, as "What breaks" says.
- The Rocq library's calculi have the cut as an optional rule: `rules`
  gains the field `cut` and `irules` the field `icut`, so a certificate
  that states its calculus by a record literal (a one-sided proof with
  Mix or affine weakening, and its refutations) writes `cut := false`; a
  certificate that names `classical`, `intuitionistic` or `affine` names
  a calculus without cut.

## [0.1.0] - 2026-10-18

The first release.

### What is promised

- **The Rust API of `linlog`** follows semantic versioning from this
  release on. Every public enum and every struct with public fields
  that may grow is `#[non_exhaustive]`, so a new variant or field is no
  break. The enums of the calculus are closed on purpose, so that a
  `match` downstream stops compiling when the calculus grows instead of
  falling into a wildcard arm: `Term`, `Kind`, `Node`, `Rule`,
  `search::Verdict`, `Branch`, `Sign`, `occurrences::Polarity`, `Side`,
  `ordinary::{Node, Rule, Side}` and `sequents::Visit`. Cut and, later,
  the quantifiers will add variants to them, each in a release that
  raises the second number.
- **The cargo features** of `linlog` are `parse`, `serialize`,
  `interactive`, `latex`, `typst`, `svg` and `rocq` (on by default) and
  `parallel`, `png` and `pdf` (off). None is removed within 0.1.
- **The JSON forms** of the data (sequents, proofs, disproofs, proof
  structures, sessions, outcomes) are promised by their wire level, not
  by the crate's version: every such document carries `"version": n`, a
  reader reads every level up to its own and refuses a later one by
  name, and a new level is a line of this file under "Wire forms",
  never by itself a break of the crates. This release writes and reads
  level 1. The options and settings values carry no version and refuse
  a key they do not know, so a later release reads this one's settings,
  and a settings file that uses a later key is refused by name here.
- **The minimum supported Rust version** is 1.92 (`rust-version`), for
  both crates with every feature; raising it is a new 0.y version and a
  line of this file.
- **The command**: within 0.1 its subcommands, flags and exit statuses
  are only added to, and its JSON output (`--format json`) follows the
  wire levels, so a 0.1.x release may write a new level (a new kind of
  refutation, say); its text for people (verdict lines, explanations,
  help) may change in any release. The library
  `linlog_cli` inside the package `linlog-cli` is the binary's and no
  API.
- **Not promised**: the text of `Display` and of error messages (their
  `code` is), the defaults' numbers (`Limits::DEFAULT_*`, the clock's
  defaults), the engines' counters, which engine decides a sequent, and
  the time it takes.

### Wire forms

- Level 1: the sequent, the proof, the disproof, the proof structure,
  the interactive session and the outcome (written only), each
  described on its type and in the module `wire`. The repository keeps
  documents of these forms as this release writes them, which every
  later release must read.

### Added

- Sequents of classical and intuitionistic linear logic, with or without
  units, exponentials, Mix and weakening (affine mode), in a text syntax
  with ASCII and Unicode spellings and a compact JSON form; detection of
  the fragment, from MLL to LL and IMLL to ILL.
- Proof search for every fragment, returning a proof the independent
  checker has validated, "unprovable" after an exhaustive search (with
  the reason where the sequent's counts tell), or "unknown" with the
  bound or limit that ended it: a focused sequent engine, a proof-net
  engine for MLL, a recursion for additive sequents and an engine for
  Horn programs under `!` (Petri nets), chosen by fragment and mode; a
  thread pool behind the feature `parallel`; stop conditions, memory
  and occurrence limits honoured everywhere a search spends time or
  memory.
- Proofs as terms over subformula occurrences, their checker, and the
  derivation view of the one-sided and two-sided sequent calculus.
- Proof nets for MLL with or without Mix: the Danos–Regnier criterion,
  sequentialization and desequentialization.
- Interactive proving: rules applied to the goals of a derivation in
  progress, undo, and the search closing a goal.
- Exports of sequents, derivations and proof nets to LaTeX, Typst and
  SVG, rendered as PNG and PDF behind the features `png` and `pdf`, and
  of finished proofs to Rocq scripts for the NanoYalla kernel.
- Ordinary classical, intuitionistic and minimal propositional logic,
  decided through its translations into linear logic, with the proof
  read back as a checked derivation of LK or LJ, and a reader for TPTP
  problems.
- Readers for the LLTP library's problems and for coverability problems
  in the `.spec` format, and generated problem families with known
  verdicts.
- The command `linlog`: `prove` (one sequent, or many from files,
  directories and standard input), `check`, `interact` and
  `seq print|json|fragment`, with a time limit of two seconds by
  default, Ctrl-C, JSON output and every export. Its exit status is 0
  for proved or valid, 1 for unprovable or invalid, 2 for an error, 3
  for unknown and 130 at a second Ctrl-C.

[0.7.0]: https://github.com/deduxit/deduxit/releases/tag/v0.7.0
[0.6.0]: https://github.com/deduxit/deduxit/releases/tag/v0.6.0
[0.5.0]: https://github.com/deduxit/deduxit/releases/tag/v0.5.0
[0.4.0]: https://github.com/deduxit/deduxit/releases/tag/v0.4.0
[0.3.0]: https://github.com/deduxit/deduxit/releases/tag/v0.3.0
[0.2.0]: https://github.com/linlog-prover/linlog/releases/tag/v0.2.0
[0.1.0]: https://github.com/linlog-prover/linlog/releases/tag/v0.1.0
