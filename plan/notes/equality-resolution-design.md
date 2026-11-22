# Equality and first-order resolution: the design

Written by session 78a on 2026-11-22, from the step's prompt
(`plan/78-equality-resolution.md`), more-logics-1
(`plan/notes/research/next/more-logics.md`), resolution-2 and
resolution-4 (`plan/notes/research/next/resolution.md`), interop-5
(`plan/notes/research/next/scope-interop.md`), the reports of steps 70,
74 and 75 and the code at `main` (`orkxnouy`). Part A is 78a's and is
built; part B is the design 78b builds on; part C is what 78c decides.

## Sources

- [M] Mendelson, *Introduction to Mathematical Logic*, §2.8: a
  first-order theory with equality is one with the axioms (A6) `∀x. x =
  x` and (A7) `x = y → (B(x, x) → B(x, y))`, every theorem of such a
  theory holding in every normal model (`=` read as identity), and a
  model of the axioms becoming a normal one by the quotient by `=`
  (Prop. 2.28's contraction).
- [F] Fitting, *First-Order Logic and Automated Theorem Proving*, 2nd ed.,
  ch. 9: the equality axioms for automated provers are reflexivity and
  the replacement axioms for every function and relation symbol of the
  language; a set of sentences has a normal model exactly when it with
  those axioms has a model (Thm 9.3.x, the "equality axioms" route
  before paramodulation in ch. 9.7).
- [TvD] Troelstra and van Dalen, *Constructivism in Mathematics* I, 2.2
  and 2.6: intuitionistic predicate logic with equality has the same
  axioms (reflexivity and replacement), sound in every Kripke model whose
  equality is a congruence at each world, identity included; no decidable
  equality is assumed.
- [LC] Otten, leanCoP 2.0 and ileanCoP 1.2 (IJCAR 2008) and the nanoCoP
  papers already cited in core-ordinary.md: the connection provers add
  the equality axioms (reflexivity, symmetry, transitivity, substitution
  for functions and predicates) to the matrix when `=` occurs; nanoCoP-i
  decides ILTP's equality problems so (Otten 2017, the ILTP tables 75b
  compared against).
- [SZS] The SZS ontologies, `szs.tptp.org` (moved from
  `tptp.org/UserDocs/SZSOntology/`; read 2026-11-22 for this note through
  a fetch, its `/success` and `/nosuccess` pages): a status line `% SZS
  status VALUE for PROBLEM`, exactly once, as early as possible; with a
  conjecture Theorem, CounterSatisfiable (or ContradictoryAxioms); without
  one Unsatisfiable or Satisfiable; Timeout, GaveUp, MemoryOut, User,
  Error, InputError and SyntaxError among the NoSuccess values. Extra
  information may follow after ` : `.
- [TPTP] The TPTP syntax: `<fof_defined_infix_formula> ::= <fof_term>
  <defined_infix_pred> <fof_term>` with `=` and `!=` (`a != b` is `~(a =
  b)`), and `cnf(name, role, clause)` with a clause a disjunction of
  literals (an atom, `~` an atom, or an infix equality or inequality),
  optionally in parentheses, its upper-case variables universally
  quantified, the roles of `fof` and `negated_conjecture`. The syntax page
  (`tptp.org/TPTP/SyntaxBNF.html`) answered 404 at the fetch; this is from
  the TPTP technical report as remembered and from the ILTP library's
  files themselves (1 730 problems write `=` and `!=` exactly so), and is
  marked "not re-verified" in the report.

## A. Equality as axioms (78a, built)

### A1. How `=` is represented: a binary predicate named `=`

`=` is the binary predicate symbol named `=` of the first-order tables
(`first_order::EQUALS`), its atoms keyed `=(s, t)` as every atom is keyed
by its canonical text. No identifier is `=`, so it never meets a user's
symbol, and every method, the read-backs, the checker, the JSON and the
structures take it as the atom it is. The printers write it infix, `s = t`
(`Formulas::write_atom`), so a sequent reads back.

- *Alternative: a node `Node::Equal(s, t)`.* Every match over `Node` (the
  methods' doors, the translations' table, the printers, the readers,
  about forty arms) would need an arm, the linear image a mapping of its
  own, and nothing gained: under axioms equality is a predicate. Rejected.
- *Alternative: a reserved identifier (`eq`).* It meets a user's `eq`.
  Rejected.

### A2. How `=` is read

- TPTP: `s = t` and `s != t` wherever an atomic formula stands; `!=` is
  `¬(s = t)` [TPTP].
- The native syntax: `s = t`, `s != t` and `s ≠ t`, the same.
- The reader looks ahead for `=` after an atom's text (one byte compare
  on the hot path, the rest out of line: the `read-tptp` and `read-text`
  journeys measure it, D17); the operand read is then a term (a
  variable, a constant or a function applied), and the right side one
  more term. `=>` and `<=>` stay TPTP's connectives.
- The `EQUALITY` hint goes: `=` is read.

### A3. Which axioms, and where they go

For a sequent whose tables hold `=`, `equality::axioms` builds, in the
sequent's own arena so that equal formulas are one node:

- reflexivity `∀x. x = x`, symmetry `∀x y. x = y → y = x`, transitivity
  `∀x y z. x = y ∧ y = z → x = z`;
- per function symbol `f` of arity n ≥ 1: `∀x̄ ȳ. x₁ = y₁ ∧ … ∧ xₙ = yₙ →
  f(x̄) = f(ȳ)`;
- per predicate symbol `p` of arity n ≥ 1 other than `=`: `∀x̄ ȳ. x₁ = y₁
  ∧ … ∧ xₙ = yₙ ∧ p(x̄) → p(ȳ)`.

They are [F]'s replacement axioms with symmetry and transitivity added,
as the prompt names them and as leanCoP adds them [LC]: congruence of `=`
itself follows from symmetry and transitivity, so it is left out. The
substitution axiom takes all arguments at once (Fitting's and leanCoP's
form) rather than one position at a time: one axiom per symbol, and the
per-position form needs transitivity chains of length n anyway.

**Where**: `equality::with_axioms(&sequent)` returns the sequent with
those axioms the antecedent lacks appended after the hypotheses, `Γ, E ⊢
Δ` (the roots keep their positions; appended, an equal hypothesis already
there is not added twice, so the call is idempotent). That one place
serves every method the prompt names at once: the embedding's image is
the image of `Γ, E ⊢ Δ`, the connection provers' matrix is that
sequent's, grounding grounds it. No method changes.

- *Alternative: each method adds its own (axiom clauses in the matrix,
  axioms in the image).* Three constructions to keep equal, and a
  derivation read back from a matrix with axiom clauses would show
  hypotheses its sequent lacks. Rejected.
- *Alternative: only the axioms a proof uses.* The search cannot know them
  beforehand; the derivation's root holds all of them, and `G3`'s axioms
  absorb the unused ones. The derivation's hypotheses therefore include
  the axioms it used, which the prompt asks, and the rest as well.

### A4. Where the augmentation runs

- The library: `ordinary::decide_within` augments before the dispatch
  (`within`), so every caller (the protocol, the workspace, the web
  bindings) gets it.
- The command: `prove --logic`, its batch, and `interact --logic` augment
  the sequent once they have read it (`io` reads it, then
  `equality::with_axioms`), so every path (the native methods, the
  classical default with the embedding's evidence, the image path, the
  race) decides one sequent and draws one derivation. A session starts
  with the axioms as hypotheses, which a proof by hand needs as well.
- `seq print` and `seq json` print the sequent as written.

### A5. How a derivation shows them

The derivation's conclusion is `Γ, E ⊢ Δ`: the axioms stand as hypotheses
in every inference below them, drawn, exported and checked as such.
`equality::axiom_count(formulas, left)` says how many of the last
hypotheses of a sequent or a derivation's conclusion are the equality
axioms (it finds them again in a copy of the arena), and the command's
verdict line says it ("its last 4 hypotheses the equality axioms"). As
built, the outcome has no field for it (departs from the first draft,
which named `Outcome::axioms` and a JSON key): the count is a function
of the derivation, so a key would only repeat it, and every constructor
of `Outcome` would carry it. The checker is `Derivation::check`,
unchanged: it checks that the derivation derives its conclusion.

### A6. "Not valid" only from a structure where `=` is identity

- `Structure`'s evaluator interprets `=` as identity, always; a structure
  that lists a predicate `=` is refused by `FiniteCountermodel::check`
  (it would claim another meaning).
- Grounding's model satisfies every axiom's instance over the domain, so
  its `=` is a congruence there; `structure_of` divides the domain by it
  (`equality::identified`: union-find over the true `=` atoms, constants
  and tuples mapped to their classes, the `=` predicate dropped), and the
  quotient structure is what `FiniteCountermodel::check` evaluates. The
  quotient is a homomorphism onto a structure that makes every formula as
  true as before [M, Prop. 2.28], so the check of the quotient is the
  check of the verdict.
- Every other "not valid" of an augmented sequent (none arises today: the
  axioms give the connection prover slots, and the embedding is unknown
  at first order) becomes `Unknown(Incomplete)` in `within`: no checked
  identity structure, no verdict.
- In intuitionistic and minimal logic the axioms are sound [TvD], and a
  classical structure with `=` identity is a one-world Kripke model, so
  the same rule holds.

### A7. Cost and D17

A sequent without `=` pays one lookup of the symbol `=` in the tables
(`Formulas::has_equality`, none for a propositional arena). The reader's
look-ahead is one compare per atom; the journeys `read-tptp`,
`read-lltp` and `read-text` measure it before and after.

### A8. TPTP's `cnf`

- `lltp::clauses` takes `cnf(name, role, clause).` beside `fof` for the
  TPTP reader (the LLTP reader still refuses it, as before), recording
  which clauses are `cnf`; the role `negated_conjecture` is a hypothesis
  (its clause is already negated) and a `cnf` clause with the role
  `conjecture` is refused, since TPTP gives it no meaning there.
- A `cnf` clause is read as the universal closure of its disjunction
  [TPTP]: its upper-case words, in order of first occurrence, are bound
  by one `∀` around the clause (the reader parses `! [X, Y] : (clause)`,
  error positions mapped back into the file).
- A file of `cnf` clauses has no conjecture, so it asks for its clauses'
  refutation, `C₁, …, Cₙ ⊢`, as a `fof` file without a conjecture does.

### A9. SZS status lines

- `prove --logic --szs` writes `% SZS status VALUE for NAME` as the last
  line of standard output, after the output as without the flag (as
  built; the first draft said first, as [SZS] recommends "as early as
  possible", but the command writes its verdict and derivation as it
  goes, and the status is known only at the end; a tool greps for the
  line wherever it stands); `NAME` is
  the problem file's name without its extension, `stdin` for standard
  input and `sequent` for the argument [SZS: "the problem name"].
- VALUE: with a conjecture (something right of `⊢`) `Theorem` for valid,
  `CounterSatisfiable` for not valid; without one `Unsatisfiable` and
  `Satisfiable` [SZS]; unknown at the time limit `Timeout`, at the memory
  limit `MemoryOut`, interrupted `User`, at any other bound or incomplete
  `GaveUp`; a syntax error `SyntaxError`, another input error
  `InputError`, any other error `Error`. The statuses mean validity in
  the logic decided (`--logic`), as ILTP reads them for intuitionistic
  logic.
- Refused beside `--format` other than text and beside a batch (exit 2):
  the line would break JSON, and a TPTP tool runs one problem per call.
  A flag with no settings key, as `--quiet`: it changes only what the
  command prints.
- No `% SZS output start Proof`: the derivation is no TSTP derivation, and
  writing one is a follow-up (resolution-4's TSTP output).

### A10. The measurement

ILTP's 1 730 first-order problems with equality (`bench/iltp-first-order`,
`deduxit-bench run --tptp … --logics classical,intuitionistic` per
method: the default, `embedding`, `connection`, `grounding`; 2 s, cores
4 to 6), every verdict against the file's classical and intuitionistic
statuses (a "valid" against a non-theorem or a "not valid" against a
theorem is a mismatch, a bug). The report gives proved and refuted per
logic and method, the time-outs, and whether a native rule is needed:
78c's question (part C).

## B. First-order resolution (78b, designed here)

- **Clauses** from 77a's clausal form (`normal.rs`'s first-order form, its
  Skolemization), as a list of literals over the arena's terms, each
  clause with its own variables (numbered per clause, renamed apart at
  each inference). Equality literals are literals of the predicate `=`;
  with equality the equality axioms of A3 enter as clauses (their clausal
  form), so resolution decides equality problems as 78a's methods do.
- **Rules**: binary resolution with the most general unifier of two
  complementary literals (`crate::unify::Unifier`, over a store of the
  clauses' terms, the occurs check on), and factoring (two literals of one
  clause unified). No paramodulation in 78b (part C).
- **Steps**: the command's `resolve` and the protocol's resolution as 70's
  propositional steps are: choose two clauses and two literals, see the
  unifier or why unification fails (a clash, the occurs check), the
  resolvent added; `--method` saturation by given clause with set of
  support as the search, bounded by clauses and steps (the existing
  `--clause-limit`, `--step-limit`).
- **The refutation** records per resolvent its parents, the literals and
  the unifier (as bindings); its checker recomputes each unifier from the
  parents' literals, applies it, compares the resolvent up to renaming
  of variables, and ends at the empty clause. Refutation is read back as
  "valid" of `Γ ⊢ Δ` whose negation's clauses it refutes, classical logic
  only; Skolemization is the trusted step, as the clausal form's checker
  holds it (77a).
- **D17**: the propositional `resolution` stays as it is; the first-order
  path is its own module beside it.

## C. A native equality rule (78c decides)

The measurement of A10 decides it (D19). Paramodulation in resolution, or
equality in the connection calculus (Otten's equality handling for
nanoCoP is axioms too), is worth a session only where the axioms leave
many problems unknown that a rule would decide within the default time;
the report of 78a gives the counts and the recommendation.
