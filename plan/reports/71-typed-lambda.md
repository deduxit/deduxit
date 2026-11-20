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
