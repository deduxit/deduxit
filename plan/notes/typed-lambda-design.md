# The simply typed lambda calculus: the design (step 71)

Written 2026-11-20 by session 71a, before its code. It settles the term
model, the text syntax, the JSON form, the type checker, what a typing
derivation is, how inhabitation goes through the engines, and the
protocol's kinds and operations. 71b and 71c build on it. Each decision
has its sources and the alternatives weighed. Where the code that
landed differs, the report (`plan/reports/71-typed-lambda.md`) says so.

## 1. What a user gets

- **Check a term.** A text such as `λf:a → b. λx:a. f x`, or a
  judgement `x : a, f : a → b ⊢ f x : b`, is read, typed and checked.
  The answer is the term's type, or the first place where the term is
  ill typed and why.
- **Check a proof's program.** The term a proof reads back as (step 50)
  is checked by the same checker, in the linear, the affine or the
  simple calculus. Until now only the tests checked it.
- **See the derivation.** A checked term has one typing derivation. It
  is drawn as a tree and can be built step by step (71b, 71c).
- **Find terms of a type.** Inhabitation, enumeration, counting and
  uniqueness go through the engines (71b).

## 2. The term model

**Decision: one model, step 50's `lambda::Term`, extended.** The prompt
fixes that the model takes step 50's terms. The alternatives were a
second model for parsed terms beside it, or a tree with boxed children.
A second model would need a converter and two checkers. A boxed tree
recurses, which the crate forbids over anything read from input
(`core.md`, "Nothing recurses over a formula").

What the model is:

- **Nodes in an arena, operands first, a DAG** (as step 50). A walk is
  a loop in index order. A read-back shares a left rule's term between
  the places it goes to. A parsed term is a tree.
- **A variable is an entry of the term's table, named by its `VarId`,
  with its type.** A binder names the variable it binds, and a use names
  the same entry. So identity is the id, not the name and not a
  de Bruijn index.
  - This is Church's own typing, in which every variable has its type
    as part of it (Church 1940, "A formulation of the simple theory of
    types", J. Symbolic Logic 5; Barendregt, Dekkers and Statman,
    *Lambda Calculus with Types*, 2013, part I, call it λ→ à la Church
    and set it beside the form with contexts, à la de Bruijn). A
    subterm's type then depends on the subterm alone, which is what
    lets one pass over the DAG type every node once.
  - Alternatives weighed. **De Bruijn indices** (de Bruijn 1972; TAPL
    ch. 6): no names to capture, but a shared node would mean different
    variables at different depths, so the DAG would have to go, and
    every printed term needs names made up again. **Locally nameless**
    (Charguéraud 2012), which `sequents::term` uses for first-order
    binders: the same objection for shared nodes, plus opening and
    closing at every binder. **Names as identity** (strings): capture
    and shadowing become the checker's problem at every use.
- **A variable's name is a string**, kept for printing and parsing
  only. Step 50 had a letter and an index; a parsed term needs the
  names its author wrote (`id`, `succ`). Trailing digits print as an
  index (`x1` is `x₁` in Unicode, `x_{1}` in LaTeX), as step 50's names
  did. (An API and wire change: `Variable::letter` and `index` go,
  `Variable::name` comes; the JSON key `letter` with `index` becomes
  `name`. Nothing read the old keys: the form was written only.)
- **Types are the term's own arena** (as step 50), hash-consed: equal
  types are one id, so the checker compares types as numbers. The JSON
  reader interns what it reads, atoms and types, so a document that
  writes one twice reads it as one.
- **Church-style: every annotation a unique type needs is in the
  data.** `Lambda` has its variable's type, `Left` and `Right` their sum
  type, `False` its goal's (step 50 chose these fields for this). A
  pattern's variables (`let … be x ⊗ y`, `case`, `copy`, `promote`) have
  their types in the table, and the checker compares them with the
  scrutinee's.
- **The linear and affine calculi are the same model** (`Calculus`
  says which discipline holds). One checker, one parser, one JSON form.
- **Where quantifiers go (D17)**: as `core-lambda.md` says, a `Node`
  for type abstraction and one for type application, a `Type` variant
  with a bound type variable. System F stays checking only (D30).

## 3. The text syntax

**Decision: the parser reads what the printer writes with `types` on,
in both notations**, for all three calculi. Print a term with
`Options::with_types(true)` and the text reads back as the same term up
to the names' table.

- **Binders carry their type**: `λx:A. M`, or `\x:A. M`. A `λ` without
  a type is refused with a message that says to write it: the calculus
  is Church-style and deduxit infers no types (type inference stays in
  `plan/later.md`, lambda-calculi-4).
- **Injections and `abort` carry their type as a subscript when types
  are shown**: `inl_(a ∨ b)(M)`, `inr_(a ∨ b)(M)`, `abort_a(M)`,
  `false_a(x⃗; z)`. Without it `inl(M)` has many types, which TAPL
  (§11.9, "Sums", uniqueness of types) mends with an ascription `inl t
  as T`. Step 50's printer already wrote `false_A` and `abort_A` so;
  the injections follow them. (A printer change: with `types` on, `inl`
  and `inr` now show their sum, and a subscript that is no atom or
  constant is bracketed, `abort_(a → b)(z)`, where it was written bare
  and could not be read.)
- **Application is juxtaposition, left-associative; a binder's body
  runs as far right as it can** (Barendregt 1984, §2.1; TAPL §5.1).
- **Types are written as the sequent parsers read them**, with their
  precedences: in the simple calculus `~`/`¬` (tightest), `/\`/`∧`,
  `\/`/`∨`, `->`/`→` (to the right), `<->`/`↔` (loosest), `true`/`⊤`,
  `false`/`⊥` (the manual's "Ordinary logic"); in the linear calculi
  `!` (tightest), `*`/`⊗`, `&`, `+`/`⊕`, `-o`/`⊸` (to the right), `1`,
  `0`, `top`/`⊤` (the manual's "Syntax"). `¬A` is `A → ⊥` and `A ↔ B`
  is `(A → B) ∧ (B → A)`, as the read-back types them.
- **A judgement**: `x : A, y : B ⊢ M : C`. The context declares the
  free variables, in order. `⊢` (or `|-`) and the context may be left
  out for a closed term, and `: C` may be left out: the type is then
  what the term has. A type given is compared with it.
- **Names**: an identifier as an atom's is (`XID_Start`, then
  `XID_Continue`), then optional subscript digits. `λ` starts no name.
  The keywords (`let`, `be`, `in`, `case`, `of`, `inl`, `inr`, `fst`,
  `snd`, `true`, `false`, `abort`, `promote`, `for`, `derelict`,
  `discard`, `copy`, `as`) name no variable.
- **Scope**: a name is its innermost binder's variable; a later
  declaration of the context shadows an earlier one. A name no binder
  or declaration has is an error at its place.
- **The parser types as it reads.** A pattern's variables take their
  types from the scrutinee, so the parser must know each subterm's type
  when it closes it. It calls the checker's own rule for one node
  (section 5), so the rules exist once. A type error found while
  reading carries its place in the text.
- **Bounds**: no recursion (a stack of open constructs), the tokens
  counted against `limits.occurrences`, as the sequent parsers count.

Alternative weighed: **bidirectional checking** (Pierce and Turner
2000; Dunfield and Krishnaswami 2021), which would read `λx. x` against
a given type. It makes the default printed judgement of a normal term
readable, but not a cut's redex, and it is inference by another name,
which the prompt keeps out. The Church-style syntax is the one the
prompt names.

## 4. The JSON form

**Decision: the term's document, at wire level 23**, read by
`wire::upgrade::<lambda::Term, _>`:

```json
{"version": 23, "calculus": "simple", "atoms": ["a"],
 "types": [{"kind": "atom", "atom": 0},
           {"kind": "lollipop", "from": 0, "to": 0}],
 "variables": [{"type": 0, "name": "x"}],
 "context": [], "goal": 1,
 "nodes": [{"kind": "var", "variable": 0},
           {"kind": "lambda", "variable": 0, "body": 0}],
 "root": 1}
```

- It is step 50's written form with `version` first and `name` for
  `letter` and `index`. The value the command writes under `lambda` and
  the bindings' term call answer is this document with `text` and
  `judgement` beside its keys, which the reader ignores, so both read
  back as terms.
- **Reading checks the shape, not the typing**: every index in range,
  operands before their node, types before the types they build, atom
  and variable names valid, `promote` with as many terms as variables,
  the counts within `limits.occurrences`. A term read is then checked
  by `Term::check`, as a derivation read is checked by its `check`.
- Level 23 because a reader below it has no term form at all.

## 5. The type checker

**Decision: a checker of `lambda`'s own, `Term::check`, over the
term's data; not a rule file.** Section 6 says why no rule file.

- **One pass types every node** (operands first, so a loop): each
  node's type from its operands' types by its rule. A variable's type
  is its table entry's. The rules are step 50's calculi: Bierman's
  thesis, figure 3.1 with 3.2's `true` and `false` (linear), and
  *Proofs and Types* §3.1 and §10.6 (simple). The reference is step
  50's test checker (`core/src/lambda/tests.rs`), which this replaces.
- **The same pass keeps each node's free variables with their use
  counts** and checks the discipline where a variable is bound:
  exactly once (linear), at most once (affine), any number (simple).
  A pair's components and a case's branches share their variables:
  equal counts in the linear calculus, the larger otherwise. `true` and
  `false` use up their arguments. A `promote`'s body uses only its own
  variables. At the root the free variables are the context's. A
  pattern's later variable and a later declaration are the inner ones,
  as the text reads them.
  - A node shared by several is counted at every place: a node's map
    is moved to its last user and copied for the others, each copy
    counted against `limits.work` and the maps held against
    `limits.memory_bytes`.
  - Counts saturate at `u64::MAX`. That never accepts a term: every
    variable left free is bound or refused, and where one is bound the
    linear and the affine calculus compare its count with 1.
- **Names are checked where a variable is bound**: no other variable of
  the same name is free in the body. Otherwise the printed term would
  read back as another term (`λx:a. λx:b. x` with the outer `x` meant),
  and a reader trusts the text. This is Barendregt's variable
  convention (1984, 2.1.13) made a condition, and the reason no rule
  file could carry these rules (section 6).
- **The result is the typing** (`lambda::Typing`): the type of every
  node, in the term's arena of types with the types the rules made
  added. Since typing is syntax-directed (uniqueness of types, TAPL
  theorem 9.3.3, with the annotations of section 2), this is the whole
  derivation: node by node, the rule is the node's kind, the premises
  are its operands, and the context is the binders on the way.
- **Errors**: `Error::IllTyped` (kind `invalid`, code `ill_typed`),
  with the reason in words (the subterm abbreviated, the types
  written), the node, and for a text the place. A bound's refusal is
  `Error::Refused`.
- **Trust.** The checker is a second checker in the trusted base
  (lambda-calculi-2 says so). 71b runs the panel on it. Its integers:
  the use counts above; indices are `u32` and checked by the reader.

## 6. The typing derivation: why no rule file

The prompt asks whether STLC's typing rules are a rule file, as natural
deduction's are, "decided by what step 60's checker can express".

**Decision: no. The typing derivation is the checked term itself
(`Typing`), drawn through a view of `lambda`'s own.** Four reasons, the
first decisive:

1. **A rule file cannot say that `λ` binds.** The rule
   `Γ, x:A ⊢ M : B / Γ ⊢ λx:A. M : A → B` is sound only when `x` is
   not declared in `Γ` (Barendregt's convention; PLFA's lookup rule
   `S` asks `x ≢ y` instead). Step 60's `fresh` asks that the atom
   occur nowhere in the node's sequent, and `x` occurs in the
   conclusion's own term, under the `λ`. A connective has no `binds`
   (`core-rules.md`, "Where quantifiers go"). With a set of
   declarations as the context, such a file accepts
   `⊢ λx:a. λx:b. x : a → b → a`, whose type is `a → b → b`. With
   hypotheses as nodes, as NJ has them, the same.
2. **Its terms are first-order with prefix, infix, postfix or function
   print forms.** Application by juxtaposition and `λx:A. M` cannot be
   written, so a derivation would show `f · x` where the term's printer
   shows `f x`: two notations for one term in one tool.
3. **The linear calculus needs more**: `true(x⃗)`, `false(x⃗; z)` and
   `promote` take any number of arguments, and a connective's arity is
   fixed. So the linear terms need a checker of their own whatever STLC
   gets, and one checker for three calculi is less to trust than a
   checker and a file.
4. **A stored derivation would hold nothing new.** The derivation is a
   function of the term, so checking one given from outside is checking
   the term and comparing.

Alternatives weighed:

- **A rule file with de Bruijn indices** (`Γ, A ⊢ 0 : A`, a weakening
  rule for `n + 1`, a sequence as the context) is expressible and
  sound. It is not the exercise courses set, which has names (TAPL
  ch. 9; PLFA "Lambda"; the Northeastern CS4400 notes the research
  cites).
- **Extending rule files with `binds`** (free occurrence, then `fresh`
  on the abstraction's variable) is the right way to give STLC, and
  later System F and first-order natural deduction, as files. It
  changes step 60's trusted checker, its wire level and its sessions,
  and wants its own panel. It is recorded for the plan, not done here.

What the view is (71b builds it):

- **`lambda::Derivation`**, made from a term and its typing: one
  inference per node of the tree the term unfolds to, its rule named
  after the node (`var`, `abs`, `app`; `pair`, `fst`, `snd`, `inl`,
  `inr`, `case`, `unit`, `abort`; the linear ones by their rules'
  names), its judgement `Γ ⊢ M : A` with `Γ` the context and the
  binders above. It implements the exports' `Drawn`
  (`linear::proofs::style`), as `rules::Drawing` does, and joins
  `export::Drawable`, so LaTeX, Typst, SVG, PNG and PDF draw it with
  the term printer's spellings.
- **Its size is bounded before anything is laid out**: every judgement
  writes its whole subterm, so the text is quadratic in the term's
  depth, and a shared node is drawn at every place. The estimate goes
  against `limits.derivation_bytes`.
- **Built step by step** on the page: the page holds the term and
  which nodes are open. A step names a rule for a goal and, where the
  rule needs one, a type (the argument's type at `app`). The library
  answers from the typing whether it is right and gives the premises.
  No session type is stored: the state is the term and a set of node
  ids, which the page keeps. (71c; the library's part is one call.)

## 7. Inhabitation through the engines (71b)

A simple type built from atoms, `→`, `∧`, `∨`, `⊤` and `⊥` is an
ordinary formula, and a closed term of it is a proof of that formula in
intuitionistic logic (Curry–Howard; *Proofs and Types* ch. 3).
Inhabitation is PSPACE-complete (Statman 1979), so every answer is
within bounds and three-valued.

- **Decide**: `inhabited(type)` is `ordinary::decide` of `⊢ A` in
  intuitionistic logic (LJT, or the SAT engine for the verdict). Valid
  gives an LJ derivation, `ordinary::Derivation::term` reads it back,
  and `Term::check` checks the inhabitant. Not valid comes with the
  checked Kripke countermodel, which is the evidence that no term
  exists. A linear type goes the same way through `linear::prove` in
  an intuitionistic mode and `Derivation::term`.
- **Enumerate and count**: inhabitants are counted **up to βη**, as
  long normal forms (Ben-Yelles 1979; Hindley, *Basic Simple Type
  Theory*, 1997, ch. 8): each is `λx⃗. y M₁ … Mₙ` with `y` in scope and
  each `Mᵢ` a long normal form of its type. Enumerating them is a
  search of its own over the term's shape, by depth, since the engines
  return one proof and identify proofs differently (the focused
  calculus up to its permutations, LJT up to its own). Bounds: a depth,
  a count (`max_terms`), `limits.work`.
- **Answers**: `Inhabitants { terms, complete }`: `complete` when the
  search space was exhausted within the depth and the type is known to
  have no longer normal form (finitely many: Ben-Yelles's criterion, or
  the search ending); otherwise "at least n". `unique` is "exactly
  one, complete". An uninhabited type is decided by the engine first,
  so the enumeration never has to prove emptiness.
- **Each inhabitant is checked** by `Term::check`, so the enumeration
  is not trusted.
- **Options (D15, D16)**: `lambda::InhabitOptions { depth, max_terms }`
  with named defaults and serde, the settings key `inhabit`; the
  command and the page map onto it.

## 8. The protocol

A suite `lambda` (`core/src/protocol/lambda.rs`, one entry in
`SUITES`). Kinds and operations, as 63a's registry lists them:

| kind | form | what it is |
|---|---|---|
| `lambda.term` | the term's document (level 23) | a term with its context and its claimed type |
| `lambda.type` | the protocol's own: `{"calculus", "text"}` | a type, as the parser reads it |
| `lambda.typing` | the protocol's own: the term's document with `typing`, a type per node | a checked term |
| `lambda.inhabitants` | the protocol's own: `{"type", "terms": […], "complete"}` | terms of a type |

The shared kind `term` (a proof's term as text, `shared.rs`) stays;
`lambda.term` is the value, `term` its rendering.

| operation | from → to | promise | cost | reads |
|---|---|---|---|---|
| `lambda.read` | text → `lambda.term` | `parses` | instant | `limits.occurrences` |
| `lambda.check` | `lambda.term` → `lambda.typing` | `well_typed` (the check is `Term::check` again) | instant | `limits.work`, `limits.memory_bytes` |
| `lambda.draw`, `lambda.export` | `lambda.typing` → drawing, text | none (a rendering) | instant | the styles, `limits.derivation_bytes` |
| `linear.term`, `ordinary.term` | a proof or derivation → `lambda.term` | `well_typed` | instant | `limits.derivation_bytes` |
| `lambda.inhabit` | `lambda.type` → `lambda.term` | `well_typed`, or the countermodel's check | bounded | the search's keys, `limits` |
| `lambda.enumerate` | `lambda.type` → `lambda.inhabitants` | every term `well_typed`; `complete` recorded | bounded | `inhabit`, `limits.work` |

The command: `deduxit term check TEXT` (the type or the error, `--format
json`, `--calculus`), `term inhabit TYPE`, `term enumerate TYPE`, and
`--derivation` to draw. The names are 71b's to settle with the manual.

## 9. What stays out

Type inference, a reduction stepper and a Rocq checker for typing
derivations (lambda-calculi-4, -5, -7) are in `plan/later.md`. System F
is D30's research item. Rule files with binders are a plan item this
note raises.

## Sources

- A. Church, "A formulation of the simple theory of types", J. Symbolic
  Logic 5 (1940).
- H. Barendregt, *The Lambda Calculus*, 1984, §2.1 (the variable
  convention, 2.1.13); H. Barendregt, W. Dekkers, R. Statman, *Lambda
  Calculus with Types*, CUP 2013, part I (λ→ à la Curry, Church and
  de Bruijn).
- B. Pierce, *Types and Programming Languages*, MIT 2002: ch. 5, 6, 9
  (uniqueness of types, 9.3.3), §11.9 (sums and their ascription).
- J.-Y. Girard, Y. Lafont, P. Taylor, *Proofs and Types*, CUP 1989,
  §3.1, §10.6. G. M. Bierman, *On Intuitionistic Linear Logic*,
  UCAM-CL-TR-346, ch. 3 (step 50's sources).
- P. Wadler, W. Kokke, J. Siek, *Programming Language Foundations in
  Agda*, chapters "Lambda" and "DeBruijn".
- N. de Bruijn, "Lambda calculus notation with nameless dummies", 1972.
  A. Charguéraud, "The locally nameless representation", JAR 49 (2012).
- R. Statman, "Intuitionistic propositional logic is polynomial-space
  complete", TCS 9 (1979). C.-B. Ben-Yelles, *Type assignment in the
  lambda-calculus*, PhD thesis, Swansea 1979. J. R. Hindley, *Basic
  Simple Type Theory*, CUP 1997, ch. 8.
- J. Dunfield, N. Krishnaswami, "Bidirectional typing", ACM Computing
  Surveys 54 (2021).
- This repository: `plan/notes/research/next/lambda-calculi.md`,
  `plan/notes/rule-schemas.md`, `.claude/rules/core-rules.md`,
  `core-lambda.md`, `plan/reports/50-lambda-terms.md`.

The books and papers are cited from memory of their contents, not read
again this session; the repository's files were read. 71b checks a
citation before the manual states it.
