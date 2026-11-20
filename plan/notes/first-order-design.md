# First-order ordinary logic: the design (step 74)

Written 2026-11-20 by session 74a, before its code. It settles the
language, the data model, the calculi and their checker, the sessions'
witnesses, the forms (text, JSON, TPTP), the exports and certificates,
the protocol, and what steps 75 to 79 build on. Each decision names its
sources and the alternatives weighed. Where the code that landed
differs, `plan/reports/74-first-order-ordinary.md` says so.

Sources read for it:
- the plan: `plan/74-first-order-ordinary.md`, `plan/README.md` (D17,
  D24, D25, D27, the step table's rows 74 to 79), the reports of steps
  42, 65 and 73, `plan/notes/kernel-boundary.md` (section 6);
- the rules: `.claude/rules/core-ordinary.md`, `core-modal.md`,
  `core-rules.md`, `core-protocol.md`;
- the code: `core/src/linear/sequents/fo.rs`, `linear/parse/`,
  `linear/occurrences/instances.rs`, `linear/proofs/check/`,
  `linear/proofs/interactive/`, `ordinary/*.rs`.

The literature is cited from memory of its contents; it was not read
again this session.

## 1. What a user gets

- An ordinary sequent may have predicates with arguments, function
  symbols and the quantifiers: `forall x. p(x) -> q(x), p(a) |- q(a)`.
- Derivations of LK, LJ and LM with the quantifier rules are checked,
  built step by step (a witness typed, or left open and filled by an
  axiom), drawn and exported, and certified in Rocq.
- Nothing decides first-order validity in this step (it is undecidable:
  Church 1936, Turing 1936). A first-order sequent given to a deciding
  method is refused by name, until step 75's searches.
- A propositional sequent sees no change, and pays nothing (D17).

## 2. The language

- **Terms**: a variable bound by a quantifier, a constant, or a function
  symbol applied to terms: `f(x, a)`.
- **Formulas**: `⊤`, `⊥`, `¬`, `∧`, `∨`, `→`, `↔` as today; an atom is
  a predicate symbol applied to terms (`p(a)`), a nullary one being
  today's propositional atom; `∀x. A` and `∃x. A`.
- **Closed sequents only.** A name a quantifier binds is a variable;
  any other name in term position is a constant. This is step 42's
  decision for the linear suite (its decision 3) and TPTP `fof`'s (every
  variable bound). Free variables as implicit universal closure (as in
  some textbooks, e.g. Troelstra and Schwichtenberg's free-variable
  sequents) were weighed: they would make `p(x) |- p(y)` mean something
  the user did not write, and a constant reads the same way in every
  logic.
- **One arity and one role per name**: a name is a predicate or a
  function symbol, with one arity, in a sequent (step 42's decision 2,
  and TPTP's).
- **No equality yet.** `=` is step 78's (equality as axioms, then
  resolution); the parsers refuse it with a hint until then.
- **The domain is not empty**, as the classical and the intuitionistic
  first-order calculi assume (`∀x. p(x) ⊢ ∃x. p(x)` is derivable in
  both). The Rocq statement makes it explicit (section 8).

## 3. The data model

**Decision: one arena.** `ordinary::Node` gains `Forall(NodeId)` and
`Exists(NodeId)`. `Node::Atom(u32)` stays an index into the atom table,
whose entries carry a predicate symbol and its arguments in a
first-order table beside the nodes. The node stays 12 bytes (its
assertion holds), and a propositional formula is built of the same
nodes as today.

- **Bound variables are de Bruijn indices** (locally nameless: de
  Bruijn 1972, Charguéraud 2012), as step 42's linear terms are. Two
  formulas equal up to the names of bound variables are one node, so
  the checker's comparison of formulas by id keeps working through
  quantifiers. Each binder node keeps the name it was first written
  with, for printing.
- **The terms are step 42's** (`FoTerm`: `Bound`, `App`, and in a
  derivation's or session's arena `Eigen` and `Meta`), interned by its
  `Interner` in a `FirstOrder` table: the symbols, the terms, the
  argument lists and each atom's predicate and arguments, all empty for
  a propositional arena. That table and its interner are logic-neutral
  (`fo.rs` has no linear logic in it but the binders' key), so the
  ordinary arena holds one; its binders' names are the ordinary arena's
  own (keyed by `NodeId`).
  - Taken from step 42, not copied a third time: the plan fixes it, and
    the unifier, the term printer, the term JSON and the limit
    `limits.terms` come with the table.
  - The move of `fo.rs` from `linear::sequents` to the crate's root is
    kernel-boundary item 1's case ("move to the root what other suites
    use from under `linear`"). It changes no public path (the linear
    suite re-exports its types) and no counter, so it may be its own
    commit; until then the ordinary suite names it by path, as it names
    `linear::sequents::name` today.
- **Instances are nodes.** The premise of `∀L` holds `A[t/x]`. The
  linear suite reads an instance through frames and never builds it,
  because its checker numbers occurrences of a fixed forest. The
  ordinary derivation instead writes every inference's sequent as node
  ids of its own arena, which already grows with the extra nodes of
  `↔` and `¬`. So `Formulas::instantiate(body, term)` builds `A[t/x]` as
  nodes, hash-consed, by an iterative walk memoised per node and binder
  depth (a node shared at two depths is substituted at each). The
  checker then compares premises as multisets of ids, unchanged.
- **Alternatives weighed.**
  - *A first-order arena beside the propositional one*, as the modal
    suite is a copy (65a). Every rule, checker arm, export and session
    would exist twice, and the embedding's table would need a second
    image function. The modal copy was right for a logic of its own
    syntax and rules; first-order logic is the same suite's language
    extended.
  - *Atoms with their arguments inline in the node*: past 12 bytes for
    every node, propositional ones included. The arena and the
    translation's tables grow with the node (`core-ordinary.md`).
  - *Named variables with capture-avoiding substitution*: equality of
    formulas would no longer be equality of ids.
- **What the change costs the propositional suites** (D17):
  - two arms per `match` over `Node`, which the crate's
    `wildcard_enum_match_arm` makes explicit at every site;
  - one empty table in `Formulas`, and a test of its emptiness where a
    method refuses first-order input.
  - The ratchet's journeys `read-tptp`, `read-json` and
    `ordinary-pigeons` and the whole ratchet are measured before and
    after (the report has the figures).
- **The engines that decide propositional logic refuse a first-order
  sequent at their door**: LJT, the SAT solver, the truth table, the
  tableau, the normal forms, resolution and the interpolants (Maehara's
  method needs the eigenvariable condition in the interpolant; step 77).
  One error, `Error::FirstOrder { method }` (kind `unsupported`, code
  `first_order`), says which method took propositional sequents only.
  The embedding takes them from step 75, when the linear engines' search
  is the semi-decision; until then it refuses them too.
- **`Node` is a closed enum of the calculus** (`core.md`'s decision):
  the two variants are a planned 0.y break and a line of the changelog.

## 4. The forms

- **Text** (the native syntax), as the linear suite reads it (42a):
  - `forall x. A`, `exists x. A`, `∀x. A`, `∃x. A`, and several
    variables at once: `forall x y. A`;
  - a quantifier binds loosest and reaches as far right as it can;
  - `p(x, f(a))` is a predicate applied to terms, a name followed by a
    bracket a function symbol;
  - `forall` and `exists` name no atom (`check_ordinary` refuses them
    already).
  - So `forall x. p(x) -> q(x)` is `∀x. (p(x) → q(x))`, as in the
    linear suite and in most logic texts that write a dot. One syntax
    for both suites matters more than any one textbook's convention.
- **TPTP `fof`** (Sutcliffe, *The TPTP Problem Library*, its BNF):
  `! [X, Y] : A` and `? [X] : A`, upper-case variables, lower-case
  functors and constants. In TPTP a quantified formula is a unit
  formula, so its body is a unit too: `! [X] : p(X) & q` is
  `(∀x. p(x)) ∧ q`. The reader follows TPTP's grammar, not the native
  one. `=` and `!=` are refused with a hint (step 78), as are `$i`,
  `tff` and `thf`, as today.
- **ILTP's first-order problems** (Raths, Otten and Kreitz, "The ILTP
  Problem Library for Intuitionistic Logic", JAR 38, 2007; v1.1.2) are
  TPTP files. A package of their own, as `iltp` is for the 274
  propositional ones, fetched because the library states no licence;
  `bench/` reads them (74b). Their status lines are read as the
  propositional ones are.
- **JSON at wire level 25** (24 is 71b's, the last): the ordinary
  sequent's form with step 42's first-order keys (`symbols`, `fo`,
  `binders`, atoms as `{"P": [symbol, [terms]]}`, formulas `{"∀": body}`
  and `{"∃": body}`), so one reader of terms serves both suites. The
  level is read from the content: a propositional sequent stays at 8,
  as a modal sequent without `□` does.

## 5. The calculi

**Decision: the G3 quantifier rules of Troelstra and Schwichtenberg,
*Basic Proof Theory* (2nd ed., 2000), §3.1, for LK (G3c), LJ (G3i) and
LM (G3m)**, as the propositional rules are. With `y` an eigenvariable
and `t` any term:

| rule | premise | conclusion | condition |
|---|---|---|---|
| `∀L` | `A[t/x], ∀x.A, Γ ⊢ Δ` | `∀x.A, Γ ⊢ Δ` | |
| `∀R` | `Γ ⊢ Δ, A[y/x]` | `Γ ⊢ Δ, ∀x.A` | `y` not in the conclusion |
| `∃L` | `A[y/x], Γ ⊢ Δ` | `∃x.A, Γ ⊢ Δ` | `y` not in the conclusion |
| `∃R` | `Γ ⊢ Δ, ∃x.A, A[t/x]` | `Γ ⊢ Δ, ∃x.A` | |

In LJ and LM the right side holds one formula, so `∃R`'s premise is
`Γ ⊢ A[t/x]`.

- **The checker accepts G1's forms too**, as it does for the
  propositional rules (G1c/G1i/G1m with additive variants): the
  principal formula kept in `∀L`'s and `∃R`'s premise or dropped (its
  contraction is admissible).
- **The eigenvariable condition is local**: every inference of an
  ordinary derivation holds its whole sequent, so `∀R`'s and `∃L`'s
  condition is that the eigenvariable occurs in no formula of the
  conclusion. This is the textbook condition. The linear checker needs
  a pass over the whole proof only because its nodes name occurrences
  of one forest.
- **Every inference records its term** (the witness or the
  eigenvariable), so the checker does not search for it: `term` in the
  inference's JSON. The checker builds `A[t/x]` and compares.
- **Eigenvariables are `Eigen(k)` terms**, numbered in the derivation,
  named for printing (`y`, `z`, … avoiding every name the sequent uses).
  The alternative, fresh constants, would make a constant typed by the
  user and one made by `∀R` look alike in the JSON, and the Rocq
  certificate binds them differently (section 8).
- **Rule files** (step 60) **do not carry the quantifier rules yet.** A
  rule file's connective cannot bind a variable, and `fresh` reads any
  occurrence in the conclusion, so `∀R`'s condition and `A[t/x]` cannot
  be said (the same reason as 71a's for the typing rules). The plan's
  item "rule files with binders" (71a's report) is where they come;
  natural deduction's first-order files (NJ with `∀I`, `∀E`, `∃I`, `∃E`)
  wait for it.
- **Cut**: LJ's cut stays as it is (Gentzen's), and the checker takes it
  at first order.

## 6. Sessions

The goal state (`ordinary::Interactive`) builds G3 derivations, as
today.
- **`∀R` and `∃L`** open their goal with a fresh eigenvariable the
  session names.
- **`∀L` and `∃R`** take a term the user types, read against the
  sequent's symbols and the eigenvariables in the goal's scope, or leave
  it open: a metavariable `$1` that a later axiom fills by unification,
  as step 42e's linear session does. `undo` unbinds what its step bound.
- **The scope**: an open witness never takes an eigenvariable made after
  it (levels, as in 42e and in the linear search: Shankar's and
  Paulson's unification with levels), and an eigenvariable of a sibling
  branch is refused.
- **The unifier is the linear session's** (`Quantified::unify`, over
  closed term ids with an occurs check, levels and a trail), lifted into
  the shared term module so that both sessions call one (kernel
  boundary item 5's goal tree helpers come with it).
- **`close` and `hint`** decide a goal through the embedding at first
  order from step 75; until then they answer the refusal.
- **Its JSON** keeps the open witnesses as `{"M": k}` and the
  eigenvariables as `{"E": k}` (step 42's tags), at wire level 25.

## 7. The exports

- The formula printer writes `∀x. A` and `∃x. A` (`\forall x.\,A` in
  LaTeX, `forall x. A` in Typst, the Unicode signs in SVG, whose
  advances Euler Math has, since the linear drawings set them), terms as
  `f(x, a)`, an eigenvariable by its name, an open witness `?1`.
- Bracketing: a quantified formula as an operand is bracketed, since it
  reaches right.
- The rule labels: `∀L`, `∀R`, `∃L`, `∃R`.

## 8. The Rocq certificates

**Decision: a term over `Prop`, as the propositional certificates are,
with no module of the library.** (departs from the plan's default of a
module of rocq-deduxit)

- The statement: `forall (D : Type) (d : D) (p : D -> Prop) (f : D ->
  D) …, hypotheses -> goal`. The domain is a `Type` of its own, made
  non-empty by `d`. Each predicate is `D -> … -> Prop`, each function
  symbol `D -> … -> D`. `∀x. A` is Rocq's `forall x : D, A`, and `∃x. A`
  is `exists x : D, A`.
- The proof is a term: `∀R` a `fun y : D => …`, `∀L` an application to
  the witness, `∃R` `ex_intro`, `∃L` `ex_ind` (or a `match`). The
  eigenvariable condition is Rocq's own scoping. LK uses `NNPP`, as the
  propositional classical certificates do.
- Why not a library module: the propositional ordinary certificates
  already need no library, so Rocq's kernel is all a reader trusts, and
  the shallow statement is the formula as a mathematician reads it. A
  deep embedding (formulas as data, a checker, a soundness theorem, as
  `Deduxit.Modal` has) would add a trusted statement of the semantics
  and work in the library, for no certificate this one cannot give.
- What the library will need: first-order Kripke models with growing
  domains for step 76's refutations, which are data checked against a
  semantics, and so a module (`Deduxit.Intuitionistic` gains terms).
  That is step 76's.

## 9. The protocol

- **Kinds**: `ordinary.sequent`, `ordinary.derivation` and
  `ordinary.session` hold first-order values too; their forms' level
  rises with the content. No new kind.
- **Operations**: `ordinary.start`, `ordinary.derivation`,
  `ordinary.draw`, `ordinary.export` and `ordinary.certificate` take
  first-order values. `ordinary.decide` and the transformations answer
  `first_order` until steps 75 and 77. No new operation in step 74.
- `reads` gains `limits.terms` where a first-order value is read.

## 10. What steps 75 to 79 build on

- **75, search**: the embedding's pattern table gains the quantifier
  rows (Girard's: `∀` as `∀`; `∃` as `∃`, with the `!` placement per
  translation). The read-back maps the linear quantifier rules to
  `∀L`, `∀R`, `∃L`, `∃R`, with the witnesses the linear proof's table
  holds. So the focused engine's first-order search (42d) is a
  semi-decision of first-order LK and LJ from the first day; the native
  engines (a connection calculus, nanoCoP) come beside it. ILTP's
  first-order problems are the measure.
- **76, countermodels**: finite classical models (the MACE method, a
  SAT encoding) and finite Kripke models with growing domains, each
  checked against the sequent, which needs the arena's terms and an
  interpretation of symbols.
- **77, transformations**: prenex forms, Skolemization (new function
  symbols in the arena), clausal form, Herbrand's theorem, the negative
  translations (their rows exist for `∀` and `∃` in `negative.rs`) and
  interpolants with the eigenvariable condition.
- **78, equality and resolution**: `=` as a predicate with its axioms,
  then resolution with the shared unifier, TPTP `cnf` and SZS output.
- **79, the page**: the ordinary and linear tabs at first order.

## 11. What 74 builds, session by session

- **74a**: this note; the arena's two variants, the atoms with
  arguments, the terms through step 42's table, the native parser and
  the printer; every deciding method's refusal; the ratchet before and
  after.
- **74b**: the JSON form (level 25), the TPTP dialect's quantifiers and
  terms, ILTP's first-order package; the move of `fo.rs` to the root.
- **74c**: the checker's quantifier rules and `instantiate`, the
  derivation's JSON, the session's rules with witnesses and the lifted
  unifier, the command's `interact --logic` at first order.
- **74d**: the exports, the Rocq terms, the protocol's reads, the
  manual's chapter and the guide; the panel's argument member on the
  checker's eigenvariable condition.
