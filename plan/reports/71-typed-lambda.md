# Step 71: the simply typed lambda calculus: typing derivations and inhabitation

## Session 71a: the term model

### Outcome

The library reads and type-checks lambda-terms. `lambda::Term::parse`
(file `core/src/lambda/parse.rs`) reads a term of the simply typed, the
linear or the affine lambda calculus from text, and `lambda::Term::check`
(file `core/src/lambda/check.rs`) checks it and returns the type of
every node, `lambda::Typing`, which is the term's typing derivation. A
term is also a JSON document, at wire level 23, which `wire::upgrade`
reads (files `core/src/lambda/read.rs` and
`core/src/serialize/lambda.rs`). The terms that step 50 reads back from
proofs are the same data, so the checker closes that round trip: every
read-back term of the tests is now checked by the crate's checker, read
back from its own text in both notations and from its JSON, beside the
tests' own checker of step 50.

The design is `plan/notes/typed-lambda-design.md`, written before the
code. The command, the protocol's suite, the derivation's drawing and
inhabitation are not built: the list for 71b is at the end.

The commits, in the workspace `../deduxit-wt-step-58b-8c0bbc` on
`main@origin`:

| change | subject |
|---|---|
| `pnsnmqkm` | Design the simply typed lambda calculus: the term model, its syntax and its checker |
| `typqovru` | Read and type-check lambda-terms |
| `llytnxyx` | Answer the review of the lambda-terms' checker and reader |
| `slsolovt` | Leave the deep lambda-term test out of the 32-bit run: it starts a thread |

This report is the change after them.

### The design's decisions and their sources

Each is argued in the design note with the alternatives; in brief:

- **One term model, step 50's, extended** (the prompt fixes that it
  takes step 50's terms). Nodes in an arena, operands first, a DAG, so
  nothing recurses.
- **A variable is an entry of the term's table with its type, named by
  its id.** This is Church's own typing (Church 1940; λ→ à la Church in
  Barendregt, Dekkers and Statman 2013), which makes a node's type
  depend on the node alone, so a shared node is typed once. De Bruijn
  indices and locally nameless terms were weighed and left: a shared
  node would mean different variables at different depths.
- **A variable's name is a string** (was a letter and an index), so a
  parsed term keeps the names its author wrote. This is the one API and
  wire change of the session: `Variable::name` for `letter` and `index`,
  and in JSON `{"type", "name"}`. Nothing read the old keys; the form
  was written only.
- **The text is Church-style: what the printer writes with `types` on.**
  Every `λ` has its variable's type. `inl`, `inr`, `abort` and `false`
  have the type they give as a subscript (TAPL §11.9 mends the same
  ambiguity with an ascription). No type is inferred, as the prompt
  keeps inference out; bidirectional checking was weighed and left for
  that reason.
- **The type checker is `lambda`'s own, and STLC's typing rules are no
  rule file.** The prompt left this to "what step 60's checker can
  express". It cannot express that `λ` binds: the abstraction rule is
  sound only when its variable is not declared in the context, step
  60's `fresh` reads any occurrence in the conclusion, and the variable
  occurs there under its own `λ`. Three more reasons are in the note:
  rule files cannot print `λx:A. M` or juxtaposition, the linear
  calculus has nodes of any arity, and a stored derivation of a
  syntax-directed system holds nothing the term does not.
- **The typing derivation is the checked term** (`Typing`: a type per
  node). A view that draws it implements the exports' `Drawn`, as
  `rules::Drawing` does (71b).
- **The checker also refuses a capture by name**: where a variable is
  bound, no other variable of its name is free in the body. Otherwise a
  document could be well typed and print a text that reads as another
  term. This is Barendregt's variable convention (1984, 2.1.13) as a
  condition.
- **Inhabitation** (71b) decides through `ordinary::decide` and reads
  the proof back, and enumerates long normal forms by a search of its
  own (Ben-Yelles 1979; Hindley 1997, ch. 8), each inhabitant checked.
- **The protocol** (71b): a suite `lambda` with the kinds
  `lambda.term`, `lambda.type`, `lambda.typing`, `lambda.inhabitants`
  and the operations the note's table lists.

A question the prompt does not settle, decided here by the literature:
**whether shadowing is allowed in text.** It is: a name is its
innermost binder's variable (Barendregt 1984, §2.1; PLFA "Lambda"), and
the checker's capture condition applies only to terms as data, where a
use can name an outer variable past an inner one of the same name,
which no text can write.

The books and papers are cited from memory of their contents. They
were not read again this session. 71b checks a citation before the
manual states it.

### The code

- **The model** (`core/src/lambda/mod.rs`): `Variable` has `name`.
  `Types` is the hash-consed arena of types that the read-backs, the
  parser, the JSON reader and the checker share. `Calculus` has `ALL`,
  `NAMES`, `name` and `FromStr`.
- **The checker** (`check.rs`): one pass over the nodes.
  - `rule` gives a node's type from its operands' by its calculus's
    rule. The parser calls the same function, so the rules exist once.
  - `Pass` keeps each node's free variables with their use counts. A
    node's map is moved to its last user and copied for the others, so
    a tree costs what small-to-large merging costs. It checks the
    discipline where a variable is bound: exactly once (linear), at most
    once (affine), any number (simple). A pair's components and a
    case's branches share their variables.
  - The simple calculus refuses the nodes and types it does not have.
  - Bounds: a memory estimate before anything is made, the maps kept
    against `limits.memory_bytes`, a unit of work per node and per use
    moved or compared against `limits.work`, the stop every 1024 units.
    Counts saturate, and `Uses`' doc says why that accepts nothing.
  - `Error::IllTyped { reason, node, span }` (kind `invalid`, code
    `ill_typed`): the reason in words with the types written, each cut
    at 200 characters.
- **The parser** (`parse.rs`, feature `parse`): a cursor and a stack of
  open constructs, no recursion. It reads a term, a term with its type,
  or a judgement, in Unicode or ASCII, all 19 kinds of node and both
  type languages, with the sequent parsers' precedences. It types each
  node as it closes it, since a pattern's variables take their types
  from the term taken apart. A type error has its place in the text.
- **The printer** (`print.rs`): with `types` on, `inl` and `inr` show
  their sum type as `false` and `abort` showed theirs, and a subscript
  that is no atom or constant is bracketed in Unicode and ASCII
  (`abort_(a → b)(z)`), where it was written bare and could not be
  read. A name's trailing digits print as the index.
- **The JSON form** (wire level 23): `Term`'s `Serialize` writes
  `version` first; `Term::from_parts` checks the shape (every index in
  range, operands first, names valid, the parts within
  `limits.occurrences`) and interns the types.
- **Options (D15, D16)**: the session adds no option. The printer's are
  step 50's `lambda::Options`; the parser takes the calculus as an
  argument and `Limits`.
- **Quantifiers (D17)**: as `core-lambda.md` says, a `Node` for type
  abstraction and application and a `Type` with a bound type variable;
  `rule` gains their arms.

Departures:

- **The value `--lambda` writes in JSON changed** (a variable's `name`
  for `letter` and `index`, and the key `version`), and so did the
  output of `--lambda-types` for a term with `inl` or `inr`. Both follow
  from the design, are in the changelog under "Changed", and the manual
  and the guide show the new forms. (departs from the design of step
  50)
- **The command has no term subcommand yet**, so the manual describes
  the check as the library's. The brief asks for the code "as far as
  the session reaches".

### What was checked, and by what

- **The tests** (`core/src/lambda/tests.rs`, 18, all passing):
  - **Two checkers agree.** Every term the tests read back (18 ILL
    sequents with all 19 kinds of node, an affine proof, the proofs
    with cuts, 11 LJ sequents in the explicit and the admissible view)
    passes the tests' own recursive checker of step 50 and then
    `agrees`: `Term::check` accepts it with the goal's type; its
    judgement printed with types in Unicode and in ASCII is read by
    `Term::parse`, checked, and printed again to the same text; and its
    JSON reads back to an equal term.
  - **Refusals.** Twelve ill-typed texts over the three calculi, each
    with its reason word for word and a place or a node
    (`an_ill_typed_term_is_refused_with_its_reason`); seven texts that
    are no term, each at its byte and with what was expected
    (`text_that_is_no_term_is_refused_at_its_place`); documents that are
    no term, a level above this build's, and a capture by name
    (`a_document_that_is_no_term_is_refused`,
    `a_capture_is_refused_whatever_binds`).
  - **The syntax** as documented: precedences, application to the left,
    shadowing, indices as subscripts, the subscripts of `abort` and
    `false` (`the_syntax_reads_as_documented`).
  - **Bounds.** The reader past `limits.occurrences`, the check past
    `limits.work` and `limits.memory_bytes`, the stop
    (`the_reader_and_the_checker_keep_their_bounds`); 50 000 binders, a
    type nested 50 000 deep and arguments nested 50 000 deep read,
    checked and written on a 256 KiB stack
    (`any_depth_is_read_and_checked_on_a_small_stack`).
- **The export snapshots** of a term with types shown (`term.tex`,
  `term.typ`, `term-deduxit.typ`, `term.svg`) changed in the `inl` and
  `inr` subscripts alone; `term-cut` and `term-lj` are unchanged.
- **The manual's and the guide's examples**: `cargo test -p deduxit-cli
  --test readme` passed, 7 of 7, with the changed `--lambda` JSON of
  `docs/src/terms.md`.
- **The gate** and **`nix flake check`**: see the lines at the end of
  this section, written when they ended.

- **The gate** (clippy on the workspace with warnings denied, every
  test, both `cargo hack` runs, the ratchet), capped on cores 10 to 12:
  exit 0 and `gate: passed` at `typqovru`, and again on the tree of
  `llytnxyx` with the review's fixes. The ratchet held every journey
  within its ceiling. It noted that cache misses moved on
  `search-chain-128` and `check-qbf-20-2`, which it says is never a
  failure; the search and the proof checker are unchanged.
- **`nix flake check`**, under the machine lock. A first run at
  `llytnxyx` failed in one check, `web-core-wasm32` (the library's unit
  tests on a 32-bit target under WASI): the new test
  `any_depth_is_read_and_checked_on_a_small_stack` starts a thread,
  which WASI cannot. It joined that check's list of tests left out
  (`modules/web.nix`, `slsolovt`), as step 50's test of the same kind
  is, and `nix build .#checks.x86_64-linux.web-core-wasm32` then passed
  (exit 0). The whole check runs again at the commit that holds this
  report, so its exit status cannot be in it: it is in the session's
  last message to `deduxit-orchestrator`.

Not verified: the checker has had no panel (71b's), no fuzz target and
no mutation run. The reviewer fuzzed it from outside (below).

### The reviewer's round

The fresh-context reviewer (`deduxit-review-71a`, Opus 5.5 at `high`,
never the writer's model) read both commits against six criteria and
tested from a scratch crate of its own: fixed JSON probes, about 100
parser texts, 1.25 million random documents (check, print in both
notations, read again, check again) and 1.1 million random token texts.
It found no panic and no loop, the typing and use-counting rules sound
wherever it looked, and the design note's claim about rule files right
by reading `core-rules.md`. It reported six findings. All hold; five
are fixed and one is left open.

1. **The capture check missed a pattern's earlier variable shadowed by
   a later one of its name.** `let`, `copy` and `promote` bound their
   variables left first, while the text makes the later one the inner.
   In the affine calculus `z : a ⊗ b ⊢ let z be x ⊗ x in x` with the
   first `x` meant passed and printed a text that reads with type `b`.
   **Fixed**: the later variable is bound first, as the context's was;
   a pattern that names one variable twice is refused
   (`Fault::Twice`). Pinned by `a_capture_is_refused_whatever_binds`.
2. **The JSON reader did not compose names and did not merge equal
   atoms.** A variable named `é` in two spellings escaped the capture
   check, and with `"atoms": ["a", "a"]` a well-typed term was refused
   ("an argument of type a, and its argument has type a"). **Fixed**:
   `from_parts` composes every name (NFC) and gives equal atom names
   one index. Pinned by the same test.
3. **`check` did not honour `limits.memory_bytes`**: the table of
   names was not counted, and a million variables held 39 times the
   bound. **Fixed**: `VARIABLE_BYTES` per variable, the tables of names
   and of types made at full size so that no growth holds two copies,
   and `USE_BYTES` doubled to cover a map while it grows. Pinned by the
   estimate's lower bound in
   `the_reader_and_the_checker_keep_their_bounds`. The reviewer's
   allocator measurement was not repeated after the fix.
4. **`Term::parse` was quadratic**: every use scanned the whole scope
   (481 ms for 40 000 binders and uses, four times per doubling).
   **Fixed**: the scope is a table by name, each name's innermost last.
   Not measured again; the loop that scanned is gone.
5. **A comment on the count of a node's users was false**: past 2³²
   users the count would not have saturated harmlessly, it would have
   underflowed. **Fixed at the cause**: the counts are `u64`, which the
   operands of any term in memory cannot pass, and nothing saturates.
6. **Three texts said something false.** The guide's example
   `x : a |- f x : b` does not parse (`f` is not declared): **fixed**.
   `Term::parse`'s docs left `!` out of what may follow `_`: **fixed**.
   Every syntax error of `Term::parse` starts "cannot parse the
   sequent:", the shared wording of `Error::Parse`: **left open**. The
   wording belongs to every parser's errors, and the command words its
   own message around the parse error, as it does for sequents; 71b's
   `term` command does the same, or the shared wording changes in a
   commit of its own.

### What the next sessions must know

- **71b: the checker is trusted and wants its panel.** Its argument is
  in `check.rs`'s docs and `core-lambda.md`: types as ids (no type
  twice in an arena), the uses counted per place over the DAG
  (`waiting`, `take`), `share` and `bind`, the saturating counts, the
  capture condition. The tests' recursive `Checker` is a second
  implementation to test against.
- **71b builds, in this order of use to a reader:**
  1. the command (`deduxit term check TEXT` with `--calculus`,
     `--format json`, and the manual's section with real invocations;
     `cli/tests/readme.rs` then runs them);
  2. `lambda::Derivation`, the view of a checked term as a tree, behind
     `Drawn` and `export::Drawable` (the design note, section 6), its
     size bounded before layout;
  3. the protocol's suite `lambda` (section 8), with its schema entries
     (`typed_term` is in `docs/src/wire.schema.json` already);
  4. inhabitation and enumeration (section 7), with
     `lambda::InhabitOptions`;
  5. a fuzz target for `Term::parse` and for the term's JSON reader
     (`fuzz/`, `.claude/rules/evidence.md`), and the mutation run's
     scope if the checker joins it.
- **The parser types as it reads.** `Term::parse` returns only terms
  whose types fit; the use discipline and the capture condition are
  `check`'s. A caller wants both calls.
- **The text syntax needs types shown.** The default printed judgement
  (types off) does not read back; `Options::with_types(true)` does.
- **A term's JSON is the document under `lambda`.** The command's value
  and the bindings' answer read back with `wire::upgrade::<Term, _>`,
  which ignores `text` and `judgement`.
- **71c: step by step.** The page holds the term and which nodes are
  open, and asks the library for the typing; no session type is stored
  (the design note, section 6). The bindings need one call that parses
  and checks, answering the typing or the error with its `span`.
- **For the plan: rule files with binders.** A connective that binds
  (`binds`, with `fresh` reading free occurrences) would let STLC, and
  later System F and first-order natural deduction, be rule files with
  sessions for free. It changes step 60's trusted checker and its wire
  level, so it is a step of its own.

## Session 71b: inhabitation

Opus 5.5 at `high`, the worker of 70a to 70c keeping its conversation,
from the brief `start-71b.md` on 2026-11-20, cores 7 to 9, in the jj
workspace `../deduxit-wt-step-58a-610ed3`, started on 71a's `typqovru`
before it landed and rebased onto `main@origin` once 71a was reviewed
and pushed (`ruywkzzp`, its review's fixes `llytnxyx` and `slsolovt`).

### Outcome

A simple type's terms are found, listed and counted, through the
engines deduxit already has, and the command and the protocol offer it.

- `lambda::inhabit` decides `Γ ⊢ A` in intuitionistic logic
  (`ordinary::decide_within`, LJT again where the default method gives
  no derivation), reads the derivation back as a term and checks it
  (`Term::check`), or answers that the type has none, with the
  engine's checked Kripke countermodel where it gave one.
- `lambda::enumerate` lists a type's terms up to βη-equality, as long
  normal forms, and counts them: exactly n, infinitely many, or at
  least n where a bound cut the search. Every term listed is checked.
  Uniqueness is `Inhabitants::unique` (exactly one).
- The command `term`: `term check TERM` (the judgement with every type,
  or why the term is ill typed), `term inhabit TYPE`, `term enumerate
  TYPE`.
- The protocol's suite `lambda` and its four kinds; the JSON at wire
  level 24; the manual, the guide, the changelog, the schema and the
  schema check.
- Two fuzz targets: the term's text and JSON readers, and inhabitation.
- The panel on the type checker and the enumeration found two real gaps
  in 71a's checker and reader and smaller points in the enumeration;
  all fixed.

The chain, on `main@origin`:

| change | what |
|---|---|
| `wwwzyquq` | the library (`core/src/lambda/inhabit.rs`, `Error::NotEnumerable`, wire level 24, the forms) and the protocol's suite `lambda`, the schema's forms |
| `oympyqtm` | the command `term`, the manual's sections, the guide, the changelog, the schema check's documents |
| `qkyomwul` | the behaviour lock: the term command's calls, and two listings that name the new command and suite |
| `wovprokx` | the fuzz targets `lambda_text` and `lambda_inhabit`, their seeds |
| `wsxxlomu` | the panel's fixes in the enumeration: a cut list keeps the shortest terms; the protocol's checks ask for the simple calculus |
| `nwkpmoks` | the panel's fix: everything the enumeration holds counted against `limits.memory_bytes` |
| `wnnpvwqm` | the panel's fix: the depth of a cut list found by halving |
| `xsuxsnuq` | the web module's budget: 3 831 827 bytes at gzip with the suite `lambda`, the ceiling raised from 3 750 000 to 3 900 000 |
| `onyzmrnt` | the reviewer's findings: the ways to an atom and the tables' keys held against the memory bound; the help, the wire table, the fuzz target's header and the "at least" wording |
| (this report) | `plan/reports/71-typed-lambda.md` |

### What was built, and its sources

- **Deciding** goes through the ordinary engines, as the design note's
  section 7 planned: a simple type built from atoms, `→`, `∧`, `∨`, `⊤`
  and `⊥` is an ordinary formula, and a closed term of it is a proof
  in intuitionistic logic (*Proofs and Types*, ch. 3). The type is the
  ordinary formula itself, typed by the read-back's own conversion
  (`simple::type_of`, factored out of `simple.rs` for it), so `¬A` is
  `A → ⊥` and `A ↔ B` a pair of implications in both. Statman (TCS 9,
  1979) makes it PSPACE-complete, so every answer is within bounds.
- **Enumeration** is a search of its own over long normal forms
  (C.-B. Ben-Yelles, *Type Assignment in the Lambda-Calculus: Syntax &
  Semantics*, PhD thesis, Swansea, 1979; J. R. Hindley, *Basic Simple
  Type Theory*, CUP 1997, ch. 8, "Counting a type's inhabitants"; both
  checked against the publishers' listings this session): a function's
  term is a `λ`, a pair's a pair, `⊤`'s the unit, an atom's a variable
  taken apart down to it. The count's argument:
  - every goal's shape (the set of types in scope and the goal) is
    decided by the engines before the search enters it, and a head is
    taken only when every argument's shape has a term, so every goal
    entered lies in some inhabitant;
  - a shape met again inside itself means infinitely many terms (the
    outer goal's term stands for the inner one, a longer term each
    time);
  - without such a repeat the pruned tree is finite, so a search that
    met no depth bound and no undecided shape met every term: the count
    is exact.

  Hindley's chapter decides finiteness by bounds on a term's depth; the
  repeat test here is a sufficient condition the engines' decisions make
  exact, and the panel's argument member checked both halves.
- **The fragment** is atoms, `→`, `∧`, `↔` and `⊤`. With `∨` or `⊥`
  (so `¬`), βη-equality identifies terms that long normal forms tell
  apart (G. Scherer and D. Rémy, "Which simple types have a unique
  inhabitant?", ICFP 2015, pp. 243-255; checked), so a count there would
  be wrong: `enumerate` refuses such a type with `Error::NotEnumerable`
  (code `not_enumerable`, kind unsupported); `inhabit` takes every type.
- **Bounds and options (D15, D16)**: `lambda::InhabitOptions { depth,
  max_terms }` (8 and 32 by default), with serde; `limits.work` and
  `limits.memory_bytes`. The counts are saturating `u64`; a saturated
  count is "at least". A list cut at `max_terms` keeps the shortest
  terms (every term of the deepest depth whose terms all fit, found by
  running again at smaller depths with the engines' decisions kept).
- **JSON** (wire level 24, `wire::INHABITANTS`, written only):
  `Inhabitants` is `{"version", "deduxit", "count": {"kind":
  "exactly"|"infinite"|"at_least", "count"}, "terms": [term documents]}`,
  `Inhabitation` `{"version", "deduxit", "verdict":
  "inhabited"|"empty"|"unknown", "term"|"countermodel"|"reason"}`;
  `Typing` serializes as `{"types", "nodes"}`.
- **The command** (`cli/src/term.rs`): `term check` reads the term as
  an argument, `--file` or standard input, in `--calculus simple`
  (default), `linear` or `affine`, and prints `well typed` and the
  judgement with types (so it reads back), or `ill typed: …` with exit
  1; its JSON is the term's document, its keys first, so the record
  reads back as the term, then `deduxit`, `verdict`, `typing`,
  `judgement`. `term inhabit` and `term enumerate` take the type as an
  ordinary formula (`TransformArgs`: text or file, the limits, the time
  limit); `enumerate` has `--depth` and `--max-terms`. Exit 0 for a
  type with a term, 1 for none, 3 when a bound or the time ran out.
- **The protocol** (`core/src/protocol/lambda.rs`): kinds `lambda.term`
  (a statement, form `typed_term`; `protocol::statement("lambda", text,
  calculus, …)` makes one), `lambda.typing`, `lambda.inhabitation`,
  `lambda.inhabitants`; operations `lambda.check` (promise
  `well_typed`, new: the output's term equals the input's and its
  typing the checker's), `lambda.inhabit` (`decides`: the term checked
  and held to the type by `has_type`, an `empty` answer `Recorded`),
  `lambda.enumerate` (`well_typed` of every term listed) and
  `lambda.duplicate`. A type is an `ordinary.sequent` claimed in
  intuitionistic logic. The schema has the forms `lambda_typing`,
  `lambda_ill_typed`, `lambda_inhabitation`, `lambda_inhabitants`, and
  the error codes `ill_typed` (71a's, missing from it) and
  `not_enumerable`; the schema check writes a document of each.
- **Fuzzing**: `lambda_text` (the first byte the calculus; a text or a
  JSON document; whatever reads and checks prints with types as a text
  that reads back, checks and prints the same; a document writes back
  to its bytes) and `lambda_inhabit` (the first byte the depth and the
  list's length; no defect, every term found checks, an exact count
  lists all its terms up to the length). They build (`cargo check
  --manifest-path fuzz/Cargo.toml --bins` in the fuzz shell); no run
  was made this session.

### The panel's and the reviewer's findings

The panel ran on the type checker (71a's `check.rs`, `parse.rs`,
`read.rs`) and on the enumeration, each member from a fresh context:
`panel-counterexamples` (Opus 5.5, high), `panel-argument` (Fable 5.2,
high), `panel-integers` (Sonnet 5.5, high). The tree changed under them
as the findings were fixed; each re-ran its witnesses on the fixed
tree.

**Found and fixed:**

1. **The capture check bound a pattern's variables outermost first**
   (argument and counterexamples, both with witnesses). `let z be x₁ ⊗
   x₂ in x₁`, both named `x`, was accepted in the affine calculus and
   printed a text that reads as `x₂`; `promote` likewise; the parser's
   own `let z be x * x in x` was refused. 71a's review found and fixed
   the same in `llytnxyx` (the later variable bound first, a pattern
   naming one variable twice refused); this session's fix of it,
   `qmmnvkyt`, was abandoned at the rebase. The counterexamples member's
   checker fuzz gave 24 accepted mismatches on the old order and 0 on
   the fixed one.
2. **`has_type` held a term of another type when a document named an
   atom twice** (argument and counterexamples): `from_parts` took
   `["a", "a", "b"]`, and the protocol's check of `lambda.inhabit`
   answered "held" for `b ⊢ b` with a term printed `x : a ⊢ x : a`.
   71a's review (`llytnxyx`) makes the reader merge equal names, which
   closes it; this session's refusal of them was abandoned with
   `qmmnvkyt`.
3. **The enumeration overran `limits.memory_bytes`**
   (counterexamples, integers): only the arena of long normal forms was
   counted; `|- ((((a ∧ a) ∧ a) → ((a → a) → a)) → (a → a))` at depth 7
   with 3 000 terms listed held 1.1 GB under a bound of 100 MB, and at
   the default bound the process passed 3 GiB. Fixed in `nwkpmoks`:
   `Search::held` counts the arena, the variables and contexts, the
   lists and tasks waiting and the terms built; the peak is now 102 MB
   at 100 MiB, 342 MB at 400 MiB, 840 MB at 1 GiB, each refused with
   `Refusal::Memory` (`a_search_past_the_memory_bound_is_refused`).
4. **A cut list held the deepest terms** (argument): `Join` filled the
   list from the first candidate, the outermost variable, deep terms
   first, so `|- (a → a → a) → a → a` with five terms listed lacked
   `λf. λx. x`. Fixed in `wsxxlomu`: where the count passes `max_terms`,
   every term of the deepest depth whose terms all fit is listed first
   (`terms_come_shortest_first_and_the_count_goes_past_the_list`).
5. **That fix ran a search per depth step** (integers): a loop down from
   the depth made `--depth 4294967295 --max-terms 1 'a -> a -> a'` take
   some 25 minutes where it answered at once before. Fixed in
   `wnnpvwqm`: the depth is found by halving, some 32 searches at
   `u32::MAX` (`a_cut_list_at_the_largest_depth_costs_a_few_searches`).
6. **The protocol's checks took a term of a linear calculus**
   (argument): sound for inhabitation, but not the simple term promised.
   `holds` now fails part `calculus` for one (`wsxxlomu`).
7. **`inhabit` did not hold its term to the sequent** (argument): a
   trusted step of the read-back; a `debug_assert!` of `has_type` now
   states it (`wsxxlomu`), and the protocol's check holds it for real.

**Answered, not changed:**

- **A count past 2⁵³ is a plain JSON number** (integers), which a
  JavaScript reader rounds. The crate's decision on wire numbers
  (`core-sequents.md`, "Numbers on the wire") lets a written-only count
  pass 2⁵³, `u64::MAX` meaning "at least"; `Count`'s doc says so now.
- **The checker refuses two well-typed documents by design**
  (argument): one whose simple calculus lists an unused linear type,
  and one with an ill-typed node the root does not reach. Both are
  71a's choices and refuse, never accept.
- **After a repeat the search goes on to the depth** (argument): the
  numerals at depth 80 end in `Refusal::Memory`, not `Infinite`. A
  refusal, not a wrong answer; stopping at the first repeat would lose
  the list.
- **The checker's `waiting` comment claimed the reverse of the code**
  (argument): 71a's review made the counts `u64` and rewrote it.
- **`run` sums the lists waiting at each task** (integers): quadratic in
  the stack of lists along one way, which is a few per goal on it.

**What the panel ran and found sound:**

- **Counterexamples**: a reference enumerator of long normal forms and
  a G4ip prover of its own, a checker of all three calculi that unfolds
  the DAG and resolves names; about 11 000 random sequents (1 to 3
  atoms, 0 to 2 hypotheses, depths 1 to 8), every `Exactly n` equal to
  the reference at the depth and ten deeper, no `Infinite` where the
  reference levels off, every `AtLeast n` equal within the depth, 10 590
  listed sets compared (each term checks, none twice, none deeper, no
  list short); 10 500 sequents with `∨`, `⊥`, `¬`, `↔`, `⊤` for
  `inhabit`, every verdict as G4ip's, every term of the right judgement;
  1 954 terms and about 104 800 mutants for the checker, 0
  disagreements on the fixed tree.
- **Argument**: every typing rule against Bierman fig. 3.2 and the
  simple calculus, the use counts and `share`/`bind`, saturation, the
  capture condition at `λ`, `case` and the context; the enumeration's
  three claims (every goal entered is inhabited, `on_way` exact,
  `Infinite`/`Exactly`/`AtLeast` sound), the stack discipline, caps and
  saturation, `term`, `has_type` with distinct names; 2 000 random
  sequents against a brute-force count, 0 mismatches.
- **Integers**: the counts saturating at 2⁶⁴ (`∧` of 63 to 200 parts),
  `max_terms` 0 and `u32::MAX`, depth 0, `limits.work` and
  `memory_bytes` from 0 up, the stop; types shared as a DAG (`↔` nested
  to 1 001) and nested 20 000 deep on a 256 KiB stack; 70 000
  hypotheses; the checker's counts driven by 2¹²⁹ uses; 400 000
  token-soup texts, 1.5 million mutated texts and 300 000 mutated
  documents with ids at every limit, 0 panics; the command's flags at
  their limits.

`fresh-reviewer` once, on Fable 5.2 at medium, after the panel's
fixes and the rebase. It found no wrong count, no ill-typed term and no
wrong `Infinite`, and four faults, all fixed in `onyzmrnt`:

1. **The memory bound was overrun still**, by the ways down to a head's
   atom: a type shared as a DAG has exponentially many, each a list of
   its own, and neither they nor the tables' keys were counted. Its
   621-byte JSON sequent (`((a ∧ a) ∧ (a ∧ a))…` sixteen deep, sixty
   times left of `⊢`) held 2.3 GB under the default 1 GiB for 400 s.
   Now they are counted as they grow, by capacity (a count by length
   still reached 1.9 GB, the vectors' doubling): the witness is refused
   at 1.08 GB under 1 GiB and at 304 MB under 300 MB
   (`a_type_shared_as_a_dag_is_held_to_the_memory_bound`).
2. **`term check`'s help named a key `term`** the record does not
   write (the term's keys come first, flattened). Reworded.
3. **The fuzz target `lambda_inhabit` claimed to check a term's type**
   against the sequent, which it cannot (`has_type` is crate-private);
   its header now says what it checks, and that the library's debug
   assertion holds the type in a debug build, which the fuzz run makes.
4. **The wire module's table of levels stopped at 23.** Level 24 has
   its row.

Its two questions are answered in the same commit: "at least" comes
from the depth or a limit (a saturated count, an undecided shape), so
the command's line, the help, the manual and the guide say both; and
the schema's `lambda_ill_typed` no longer mentions a parse error, which
`term check` answers with exit 2 and no document.

### Checks

All on cores 7 to 9, capped; the flake check under the machine lock.

- **Between commits**: clippy with warnings denied and `cargo test
  --workspace` after each unit (the gate's parts but the two `cargo
  hack` runs and the ratchet, which the full check holds); the core's
  576 unit tests and every other target passed on the rebased chain.
- **`nix flake check --keep-going` at the head**, the gate's five parts
  with it: every check passed on the second run. Its first run failed `budget` alone: the web module
  at 3 831 827 bytes at gzip, 82 KB over the ceiling of 3 750 000, since
  the protocol's suite `lambda` links the term's parser and checker and
  the enumeration into the workspace's runs, which the module carries.
  The ceiling is 3 900 000 in `xsuxsnuq`, a commit of its own with the
  figure (`budget.nix`: "the budget says when a cost grew, not whether
  it may"); the planning session may prefer to keep the suite out of
  the module instead.
- **The schema check's documents**, written by the command and
  validated against `docs/src/wire.schema.json` with Python's
  `jsonschema` before the flake check ran `wire-schema`: the seven new
  documents passed, and no unknown key passes.
- **The README's, the manual's and the guide's examples**
  (`cli/tests/readme.rs`): the three new sections run, every command,
  flag and key they name exists.
- **The behaviour lock**: nine new calls and two listings, blessed in
  `qkyomwul`, a commit of its own.
- **The fuzz targets** build in the fuzz shell; they were not run.
- **The panel and the reviewer** ran their own probes (above): about
  11 000 sequents against a reference enumerator, 10 500 against G4ip,
  104 800 checker mutants, 2 000 more sequents against a brute-force
  count, and the integers to their limits.

### Departures

- **No `lambda::Derivation` yet** (departs from the design): the design
  note's section 6 has 71b build the typing derivation as a view
  behind the exports' `Drawn`, and 71a's list put it second. It needs a
  judgement writer per subterm in every target's notation, an arm of
  `export::Drawable` in every exporter and its size bound; it is left
  for 71c, which the planning session gave it (on the exports, from
  `Typing`).
- **Linear types are not inhabited** (departs from the design): section
  7 has a linear type go "the same way" through `linear::prove`; the
  brief names simple types, and this session did not build the linear
  path.
- **No settings key `inhabit`** (departs from the design): the depth and
  the list's length are options of the call, as `resolve`'s bounds are,
  so no settings key holds them; the command has flags and the protocol
  parameters.
- **No kind `lambda.type`** (departs from the design): a type is an
  `ordinary.sequent` statement in intuitionistic logic, which a front
  end already makes, and its left formulas are the free variables.
- **`enumerate` refuses `∨` and `⊥`**: the design's section 7 promised
  counting up to βη without saying the fragment; with sums the long
  normal forms are not the βη-classes (Scherer and Rémy), so the
  refusal is the honest answer.
- **The guide's summary does not name `term`**: it was at its budget of
  8 000 bytes; the reference has a section.

### What the next sessions must know

**For 71c (the page, and the typing derivation's drawing):**

- The bindings need these calls: `term check` as the protocol's
  `lambda.check` writes it (the term's document, its keys first, with
  `typing` and `judgement`), `lambda::inhabit` and `lambda::enumerate`
  over an ordinary sequent in intuitionistic logic. Their JSON is
  level 24 (`Inhabitation`, `Inhabitants`); `Count` is tagged by `kind`.
- `enumerate` refuses `∨`, `⊥` and `¬` with `Error::NotEnumerable`
  (code `not_enumerable`): the page says the type is outside the
  counted fragment and still offers `inhabit`, which takes every type.
- A list cut at `max_terms` keeps the shortest terms; the count is
  the full search's. `Inhabitants::unique` is `Some(true)` only for
  exactly one.
- The terms' names follow the read-back's letters (`f`, `g`, `h` for
  functions, `x`, `y`, `z`, … otherwise), distinct along a path.
- The typing derivation is yours: `Typing` gives every node's type, and
  the judgement's printer is `Term::write_judgement`; the design note's
  section 6 says what the view is.
- The module's size: this session adds no binding, so it measured
  nothing; the five calls of 70c left 8.6 KB under the 3 750 000-byte
  ceiling, and the enumeration and the term checker will not fit in
  that without the ceiling raised.

**For 73 (the release of 0.7.0):**

- Wire level 24 is new (`wire::INHABITANTS`, written only); `LEVEL` is
  24. The schema has `lambda_typing`, `lambda_ill_typed`,
  `lambda_inhabitation`, `lambda_inhabitants` and the error codes
  `ill_typed` and `not_enumerable`.
- New public items for the marked list (`core.md` names them):
  `lambda::{Count, InhabitOptions, Inhabitation, Inhabitants}`, the
  functions `lambda::inhabit` and `lambda::enumerate`,
  `Error::NotEnumerable`. The changelog has them under "Unreleased".
- The protocol has a fifth suite, `lambda`, with the promise
  `well_typed`; `catalogue()` lists five suites.
- The fuzz targets `lambda_text` and `lambda_inhabit` build but have not
  run; the next fuzz run takes them (`fuzz/seed.sh` seeds them, so run it
  after a build of the command).
- `term check`'s judgement brackets every compound type of a `λ` and
  the right operand of `→` (`λf:(a → b). … : (a → b) → (a → b)`):
  that is the printer's (step 50's and 71a's), shown as it is in the
  manual and the lock; a printer that leaves out redundant brackets
  changes both.

