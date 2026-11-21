# Step 74: first-order ordinary logic, its language and calculi

## Session 74a: the design and the data model

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc` on
`main@origin`. (departs from the plan: Opus in place of Fable,
unattended) Nothing was pushed.

### Outcome

The design is written: `plan/notes/first-order-design.md`. It decides
the language, the data model, the calculi and their checker, the
sessions' witnesses, the text, JSON and TPTP forms, the exports, the
Rocq certificates and the protocol, and what steps 75 to 79 build on.
Each decision gives its sources and the alternatives weighed.

**No code landed.** (departs from the plan: the brief asks for the
arena, the terms and the parser "as far as the session reaches")

- The first unit of code is `ordinary::Node`'s two variants. The
  compiler names 67 `match`es in 30 files that each need an arm for them.
- Most of those arms hold only behind every deciding method's refusal
  at its door. So the variants, the arms, the doors and their tests are
  one change, which must not land half done.
- This session ran 71a, 71b's review, 71c and 74a in one conversation,
  and that change did not fit in what was left of it. It is 74b's,
  first; the figures it compares against are below.
- A smaller slice was weighed and set aside: terms and ground atoms
  without binders, which every propositional engine already decides
  correctly. It would still need the JSON form of terms, since the JSON
  reader refuses an atom named `p(a)`. Without that, a sequent the
  parser reads would be written to JSON and refused when read back.

The commits:

| change | subject |
|---|---|
| `zzqkrqxr` | Design first-order ordinary logic: the language, the arena and the calculi |

This report is the change after it.

### The design's decisions

The note argues each; in brief:

- **One arena.** `ordinary::Node` gains `Forall(NodeId)` and
  `Exists(NodeId)`. An atom stays an index, its predicate and arguments
  in step 42's first-order table beside the nodes, so the node stays 12
  bytes.
- **Bound variables are de Bruijn indices** (locally nameless, as step
  42's linear terms), so formulas equal up to bound names are one node
  and the checker's comparison by id holds through quantifiers.
- **Instances are nodes**: `Formulas::instantiate` builds `A[t/x]`,
  hash-consed, memoised per node and binder depth. The linear suite's
  frames are not taken: they exist because its checker numbers the
  occurrences of one forest, and the ordinary derivation writes every
  sequent whole.
- **The terms are step 42's** (`FoTerm`, `FirstOrder`, `Interner`), not
  a third copy, as the prompt fixes. Their move from `linear::sequents`
  to the crate's root is kernel-boundary item 1's case, a commit of its
  own.
- **The calculi are G3c, G3i and G3m's quantifier rules** (Troelstra and
  Schwichtenberg, *Basic Proof Theory*, 2nd ed., §3.1). The checker also
  takes G1's forms. Every inference records its term. The eigenvariable
  condition is local: the eigenvariable occurs in no formula of the
  conclusion, since every inference holds its whole sequent.
- **Sessions take a witness as the linear session does**: a typed term,
  or an open witness `$1` that a later axiom fills by unification with
  levels. The linear session's unifier is lifted into the shared terms
  module.
- **Rule files do not carry the quantifier rules yet**: a connective of
  step 60 cannot bind, which is 71a's reason too. The plan's item "rule
  files with binders" is where they come.
- **The forms**:
  - the native syntax is the linear suite's (`forall x. A`, a quantifier
    binding loosest);
  - the TPTP reader follows TPTP's own grammar (a quantified formula is
    a unit);
  - JSON at wire level 25 with step 42's keys;
  - ILTP's first-order problems come as a package of their own.
- **The Rocq certificates are terms over `Prop`**, as the propositional
  ones are, with no module of the library: a domain `(D : Type) (d :
  D)`, predicates and functions over it, `∀` and `∃` as Rocq's own.
  (departs from the design: the plan's default was a new module of
  rocq-deduxit; the note says why a shallow statement certifies the same
  with less to trust)
- **The deciding methods refuse a first-order sequent at their door**
  with one error, `Error::FirstOrder { method }`, until step 75's
  searches and step 77's transformations.
- **What 75 needs**: the embedding's pattern table gains Girard's
  quantifier rows, and the read-back the quantifier rules. Then the
  focused engine's first-order search (42d) is a semi-decision of
  first-order LK and LJ from the start.

A question the prompt does not settle, decided by the literature: **the
scope of a quantifier in the native syntax**. It reaches as far right as
it can, as in the linear suite and in the texts that write a dot after
the variable; TPTP's own reader keeps TPTP's grammar.

### The measurements (D17)

The ratchet before any change of `ordinary::Node`, at `zzqkrqxr` (capped,
cores 10 to 12, callgrind's instruction counts, which do not depend on
the machine's load):

| journey | instructions |
|---|--:|
| `read-text` | 12 433 368 |
| `read-json` | 12 226 550 |
| `read-tptp` | 33 313 038 |
| `ordinary-pigeons` | 31 715 255 |
| `render-latex` | 5 780 834 |
| `render-svg` | 79 508 723 |

The other journeys are linear; the run's table is the ratchet's output
of that day. 74b measures again after the change of `Node`, and runs the
propositional ordinary suite's rows of the target set and ILTP's
propositional problems against these.

### What was checked, and by what

- The design's claims about the code were read in the code. Two reading
  sub-agents (Sonnet 5.5 at `high`, `deduxit-read-74a-plan` and
  `deduxit-read-74a-code`) mapped the plan's material and the linear
  suite's first-order code, with file and line citations. The session
  checked the points the design rests on itself: the ordinary node and
  arena, the checker's inference form, the parser's operand loop, the
  first-order table and its interner, and the sequent's JSON form.
- The compiler listed the 67 `match`es the two variants reach (the run
  is in this report's first section).
- No code changed, so the gate has nothing to judge beyond the last
  one's; `nix flake check` runs at the commit that holds this report,
  so its exit status cannot be in it: it is in the session's last
  message to `deduxit-orchestrator`.

### The reviewer's round

The fresh-context reviewer (`deduxit-review-74a`, Fable 5.2, as the
brief asked, since the writer is Opus) read the note against the prompt,
the brief, kernel-boundary section 6, the rules and the code it makes
claims about. It ran nothing.

It found that these held:
- the data model;
- the G3 rules for all three logics;
- the local eigenvariable condition, with the principal formula kept or
  dropped, shared or split contexts, and LJ's cut;
- the Rocq statement over a non-empty domain;
- the syntax;
- D17's account;
- that steps 75 to 79 are possible on this model.

It reported eight findings. All hold, and the note is amended for each
in the change that holds this report:

1. **An open witness cannot be filled in place** when instances are
   nodes: `p($1)` and `p(a)` are two ids, and the axiom compares ids.
   The note now has a bind substitute through every stored sequent of
   the session, with the replaced ids recorded for `undo`. The checker
   refuses `Meta` in a derivation read from JSON.
2. **An eigenvariable that no rule binds had no Rocq term.** Such an
   eigenvariable is used in a sibling branch, below its `∀R`, or as the
   constant an open witness became. It is now `d` outside the subtree
   of the inference that introduces it, with the soundness argument.
3. **The checker cannot build nodes** (`&self`). `instantiate` has a
   find-only form for it, and the interner's maps stay in `Formulas`.
4. **The term printer, the atom key and the sequent reader** take an
   arena to hold no `Eigen` or `Meta` (`unreachable!`). They are now
   listed as code 74 extends.
5. **Binder names kept per node print wrong formulas without renaming.**
   Every target's printer now renames as the linear one does,
   eigenvariables' names included.
6. **Kernel-boundary items 2 and 3 were not decided.** The note now
   decides them: `Arena<N>` is deferred to its own trigger, and the
   parser over tables is not shared, since the loops no longer share
   their shape. Item 5 is 74c's.
7. **TPTP's unbound upper-case word** is a variable, not a constant:
   the TPTP reader refuses it.
8. **The admissible view could draw a `∀R` with its eigenvariable in
   its conclusion.** It now keeps such a weakening.

### What the next sessions must know

- **74b first: the node change, whole.**
  - Add `Node::Forall` and `Node::Exists`.
  - Give every `match` its arm. The compiler names them: 67 in 30 files,
    `normal.rs` with 18, `ljt.rs` with 6 and `sat/` with 11.
  - Refuse a first-order sequent at the door of every method that
    decides or transforms: `decide`, `ljt::decide`, `sat::decide`,
    `truth_table`, the tableau, `normal`, `resolution`, `interpolant`,
    `negative`, `translate`, `Interactive::within`, `WrittenTableau`'s
    review and `Model::forcing`. Only then does an inner arm's
    `unreachable!` hold.
  - Test each door.
  - Measure the ratchet against the table above.
- **Then, in this order**: the arena's first-order table (step 42's,
  moved to the root), the native parser's binders and terms (out of
  line, `#[cold]`, as the linear parser's are), the printer with binder
  names (rename a binder whose name an enclosing one has), the JSON
  form at level 25, the TPTP dialect. Then 74c's checker and sessions,
  and 74d's exports and certificates.
- **The printer must rename**: a node is shared by every formula equal
  up to bound names, and its binder keeps the first name written, so two
  nested binders can carry one name. Print a fresh name where an
  enclosing binder has it.
- **The JSON reader refuses an atom named by its canonical text**
  (`p(a)`): ground first-order atoms need the level-25 form before any
  first-order sequent reaches a writer.

## Session 74b: first-order formulas

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`,
cores 10 to 12, on 74a's design (`svtkmtqv`), rebased onto `main@origin`
at the end. Nothing was pushed.

### Outcome

The brief's three items landed, the third with the TPTP dialect too:

| change | subject |
|---|---|
| `poxmvprw` | Move the first-order tables and the name check to the crate's root |
| `soksuzkn` | Give ordinary formulas binders, instances and the first-order refusal |
| `oqwyknno` | Read first-order ordinary formulas from text, TPTP and JSON |
| `qkmrmkuy` | Answer the review of step 74b: doc comments, the code first_order in the wire schema, an unmatched bracket refused |
| `ovqmkmtv` | Raise the web module's budget for first-order ordinary formulas |

This report is the change after them (`wtusxsoz`), with the target
set's file `bench/targets/74b.csv`.

- **Item 1, the terms at the root.** `core/src/first_order.rs` holds
  step 42's `FirstOrder` and `Interner`, generic over the id of a binder
  (`K`), and `core/src/name.rs` the check of names; both are
  crate-private. `linear::sequents::fo` re-exports the public types
  (`Args`, `Eigen`, `FoTerm`, `FoTermId`, `Symbol`, `SymbolFault`) and
  names `FirstOrder<TermId>` and `Interner<TermId>` by alias, so no
  public path and no linear line changed but the imports. The target
  set is equal to the oracle (below).
  - The unifier was not lifted (departs from the brief: "step 42's terms
    and unifier shared"). `Quantified::unify` is woven into the linear
    session's instance table, and the ordinary suite has no caller of it
    until 74c's sessions; moved now, it would be dead code in the crate's
    root. The design puts the lift with the goal tree's helpers in 74c
    (its section 4a, item 5), which this follows.
- **Item 2, the node change, whole.** `Node::Forall(body)` and
  `Node::Exists(body)` over de Bruijn indices, every arm the compiler
  named (the library's, the tests' and the other suites'), `Formulas::instantiate` and its find-only form
  `instance`, and `Error::FirstOrder { method }` (code `first_order`,
  kind unsupported) at the door of every method that decides or
  transforms: `decide`, LJT, the SAT solver, the truth table, the
  tableau, the normal forms, resolution, the interpolants, the negative
  translations, the embedding, the Kripke model's check, the goal state
  (`within` and its reader), the written tableau's review, the Rocq
  certificate, the lambda-term (`Derivation::term`, `enumerate`), the
  rule file's derivations (`from_ordinary`, `from_tableau`) and the
  tableau's reader. One test asks every door
  (`every_propositional_method_refuses_a_first_order_sequent`).
  - The arena holds its first-order part as `Option<Box<Interner>>`,
    made at the first binder or predicate (departs from the design,
    which had the tables in `Formulas` beside the nodes): inline, the
    interner made every arena larger and `lambda::Inhabitation`'s
    variants lopsided (clippy's `large_enum_variant`), while boxed a
    propositional arena pays one pointer.
  - The printer renames a binder whose name a symbol or an enclosing
    binder has as `x_1`, `x_2` (departs from the design's `x1`: the
    linear printer's scheme, so both suites print alike).
  - A sequent or derivation whose arena is first-order is written at
    wire level 25 (`wire::LEVEL` is 25 now).
- **Item 3, the readers.**
  - The native syntax: `forall x y. A`, `∀x. A`, `exists`, `∃`, a binder
    reaching as far right as it can; `p(x, f(a))`; a name in term
    position the innermost bound variable of that name, else a constant
    or function symbol; `=` and `!=` refused with a hint (write a
    predicate).
  - TPTP: `! [X, Y] : A` and `? [X] : A` binding the unit after them, an
    upper-case word that no quantifier binds refused. The clause
    splitter shared with LLTP counts `[` and `]` with the parentheses,
    or a quantifier's `[X, Y]` read as an annotation.
  - JSON at level 25: the linear sequent's keys `symbols`, `fo`,
    predicate atoms `{"P": [symbol, [terms]]}`, and `binders`, written
    and read through the linear form's `Tables` (made generic), the
    tables interned again on reading so each term is held once.
    `core/tests/wire/level-25.txt` pins a sequent and a derivation, read
    back byte for byte by `the_twenty_fifth_level_reads`.
  - ILTP's first-order package was not reached: it is 74c's (below).

### The measurements (D17)

The ratchet's instruction counts (callgrind, the gate's build), against
74a's table:

| journey | 74a | after item 1 | after item 2 | after item 3 |
|---|--:|--:|--:|--:|
| `read-text` | 12 433 368 | 12 433 368 | 12 433 368 | 12 433 368 |
| `read-json` | 12 226 550 | 12 226 550 | 12 226 550 | 12 226 550 |
| `read-tptp` | 33 313 038 | 33 313 038 | 33 500 147 | 33 661 297 |
| `ordinary-pigeons` | 31 715 255 | 31 715 255 | 31 718 076 | 31 718 052 |
| `render-latex` | 5 780 834 | 5 780 834 | 5 780 834 | 5 780 834 |
| `render-svg` | 79 508 723 | 79 508 723 | 79 508 723 | 79 508 723 |

- `read-tptp` is the one that moved: +0.56 % at item 2 (`Formulas::atom`
  asks whether the arena is first-order when it adds a name, `add`
  whether a node is a binder) and +0.48 % more at item 3, so +1.05 % in
  all. That is 0.89 % over its ceiling (33 363 861), inside the ratchet's
  2 % tolerance, so the gate passes, but the propositional TPTP reader
  pays (departs from the design: D17's "pays nothing"; for the planning
  session to accept, or to send back). `read-lltp`, a linear journey,
  moved +0.25 % with the clause splitter's bracket arm (28 308 772, 0.10 %
  over its ceiling). The next change on either journey has about 1 % of
  tolerance left.
- The parser's first version cost `read-tptp` +4.5 %, all inlining: a
  second caller of `identifier`, `Formulas::atom` and the name map's
  `get` and `insert` (from the cold first-order paths) took them out of
  the operand loop, and `rest`, inlined instead, turned the operator
  tokens' comparisons into `memcmp` calls (40 232 calls against 2 416).
  The cold paths now have their own reader of a word (`word`) and add
  atoms by `entry`, `rest` is kept out of line, and the binder's closing
  is a cold function; each step was measured by callgrind against the
  parser of item 2 built the same way (`core-ordinary.md` has the rule).
- **ILTP's 274 propositional problems at 2 s**, at `oqwyknno`, with the
  baseline's settings but one run each (`--copies none --timeout 2`;
  intuitionistic and minimal logic by the embedding, LJT, the SAT solver
  and the default; classical logic by the embedding, the SAT solver, the
  default, the truth table and the tableau), three shards on cores 10 to
  12, against the baseline of 2026-11-16 (`iltp.csv`,
  `iltp-classical.csv`): of the 3 562 rows, 2 732 are decided by both,
  and **no verdict differs**. 13 rows the baseline decided end at the
  time limit here (the largest SYJ202, 204, 205, 206, 208, 210, 211 and
  212 instances, which the baseline decided in 1.0 to 1.5 s on a
  performance core by night; these ran on efficiency cores by day);
  nothing is decided here that the baseline did not decide. The same
  three `error` rows as the baseline's (SYN007+1.014 by the embedding,
  past the occurrence bound). Not a timed run: verdicts only.
- The focused engine's target set after item 1
  (`bench/targets/74b.csv`): `summary --against bench/targets/47.csv
  bench/targets/74b.csv --counters` names no verdict that differs and,
  of the 145 problems both decide alike on one thread, none whose
  `nodes`, `splits`, `memo_hits`, `memo_entries`, `links` or `tests`
  differ. Items 2 and 3 do not touch the linear suite but for the
  shared clause splitter.

### The panel's argument member

`deduxit-panel-74b` (`panel-argument`, Fable 5.2 at `high`) on
`instantiate`, `instance`, the arena's invariant, the parser's scoping
and the doors, before commit 3. It stated the invariants from the code,
then checked every site, and ran its witnesses once in a scratch copy.
It found items 1 (the substitution) and 4 (the parser and the printer)
to hold, and these, each answered in `oqwyknno` with a test that runs
its witness:

1. **Three public paths reached a `REFUSED` arm with no door**:
   `lambda::enumerate` (and the protocol's `lambda.enumerate`, a panic
   on `forall x. q |- forall x. q`), `rules::Derivation::from_tableau`
   on a tableau read from JSON, and `NormalForm::claims` after a public
   field is set. Each has its door now, `has_type` answers `false` for a
   first-order statement, and the tableau's reader refuses a
   first-order sequent.
2. **The derivation reader took loose variables**: a derivation of
   `p(#0) ⊢ p(#0)` read and checked. It runs the scope pass over every
   inference's formulas now, as the design says.
3. **`enter_first_order` was not atomic**: a failed entry left the
   predicates short of the atoms, and a later binder misaligned them
   (`f ⊢ b` read back as `b ⊢ b`). It truncates on failure now, and the
   state it needed (tables without an atom, from a document of symbols
   alone) no longer arises.
4. **A document of nullary predicates read as first-order**, so `⊢ p`
   written as `{"P": [0, []]}` was refused by every method. Tables
   without a term whose atoms are all nullary read as propositional now.
5. **`instance` missed what `instantiate` built** where a document lists
   a term twice: the reader kept both ids. The reader interns every
   symbol, term and list again now, so the arena is canonical.
6. **A witness with a bound variable is captured**: `#0` put for `x` in
   `∃y. r(x, y)` gave `∃y. r(y, y)`. `instantiate` refuses such a term
   (`Error::LooseVariable`) and `instance` answers `None`: the checker
   of 74c takes the witness from a document, which may name any term.

Nothing it found let a wrong inference pass: the checker has no
quantifier rule yet.

### The reviewer's round

`deduxit-review-74b` (`fresh-reviewer`, Fable 5.2 at `medium`) read the
three commits against the brief, the design and the rules; it ran
nothing (its cores were another session's). It found the move, the JSON
byte for byte, the level, the errors, the printer and parser round trip,
the panel's fixes, the changelog and the feature gates to hold, and:

1. **A test's doc comment sat on the wrong test** (`reads_tptp`'s above
   the new `reads_binders_and_terms`): moved back.
2. **`Formulas::atom`'s doc called `forall` and `exists` reserved for a
   later syntax**: they are the quantifiers' keywords now; fixed.
3. **The report said `read-tptp` was under its ceiling**: it is 0.89 %
   over, inside the tolerance. Corrected above, and the cost is marked
   as a departure from D17.
4. **No ILTP evidence in the report**: the run had not ended when it
   read; the result is above.
5. **The unifier not moved**: confirmed as declared (the departure
   above).

Its two questions: the manual's wire schema lacks the code
`first_order` (added to its enum now) and the level-25 forms (the
manual is 74d's, below); and an unmatched `]` in a TPTP clause ends
the formula in the clause splitter, where it was ignored before. The
clause is refused all the same, since `).` does not follow the formula's
end; a test pins that `p ] q` is an error. A guarded arm that ignored it
as before cost `read-lltp` 0.7 % and `read-tptp` 0.5 % more, measured
by the gate, so the arm that ends the formula stays (as
`depth.saturating_sub(1)`: the plain `depth -= 1` counted 0.3 % and
0.2 % more, by the shape the loop compiled to).

### What was checked, and by what

- The gate (clippy, the tests, both `cargo hack` runs, the ratchet) at
  each of the three commits, capped on cores 10 to 12; the last after
  the reviewer's fixes too.
- The target set after item 1, compared with the oracle (above).
- ILTP's propositional problems at 2 s, verdicts against the last
  baseline (above).
- `read-tptp` by callgrind against item 2's parser built the same way,
  at each step of the inlining work (above).
- `nix flake check` under the machine lock, after the rebase onto
  `main@origin`. Its first run found three things the gate does not
  check, each answered in the commit it belongs to:
  - `treefmt`: the files changed by script were not formatted (the hook
    formats only an editor's writes); `nix fmt`, folded into each
    commit by `jj absorb`.
  - `web-core-wasm32`: the instance test of 50 000 levels started a
    thread of a small stack, which WASI cannot; it runs on the test's
    own thread now, still deeper than a recursion would survive.
  - `budget`: the web module is 3 903 905 bytes at gzip, 3.9 KB over its
    ceiling of 3 900 000; raised to 3 950 000 in `ovqmkmtv` with the
    figure and what earned it, as `modules/budget.nix` asks.
  The final run's exit status is in the session's last message to
  `deduxit-orchestrator`, since it runs on the commit that holds this
  report.

### What the next sessions must know

- **For 74c, the checker and the sessions:**
  - `Formulas::instantiate` (builds `A[t/x]`) and `instance` (finds it,
    builds nothing) are in `core/src/ordinary/instance.rs`, tested, and
    marked `expect(dead_code)` outside the tests, as are the interner's
    `find_term` and `find_list` and `Formulas::predicate_atom` without
    `parse`: the checker's quantifier rules are their first caller, and
    the commit that calls them drops the marks. Both refuse a witness
    with a bound variable; the checker takes the witness from a document.
  - The derivation reader already runs the scope pass over every
    inference, so `instance`'s precondition holds for a derivation read
    from JSON.
  - `FirstOrder::write` and the JSON reader still take a sequent's
    arena to hold no `Eigen` or `Meta`: the derivation's and the
    session's arenas will, so both learn to write and read them (the
    design's section 3).
  - The goal state refuses a first-order sequent at `within` and at its
    reader (`Error::FirstOrder { method: "the goal state" }`), and the
    invertible rules' arm on a binder is `unreachable!` behind that
    door: 74c lifts the door when the session has its quantifier rules.
  - The unifier (`Quantified::unify`) is still the linear session's; the
    design moves it with the goal tree's helpers in 74c.
  - **ILTP's first-order package** was not reached: the flake's `iltp`
    package fetches the 274 propositional problems only. The design
    asks for a package of their own (the library states no licence),
    `bench/` reading it, and the statuses read as the propositional
    ones. The TPTP reader takes `fof` with quantifiers now, so the
    package is the remaining part.
  - Every new method that decides or transforms gets its door
    (`Formulas::propositional`) and its line in
    `every_propositional_method_refuses_a_first_order_sequent`; the
    panel found three that were missing.
- **For 74d, the exports and the docs:**
  - LaTeX, Typst and SVG print first-order formulas through
    `Formulas::write` (`Symbols` has `forall`, `exists` and `dot` per
    target; the derivation's width estimate counts the binders' names).
    No snapshot pins a first-order ordinary derivation yet.
  - The Rocq certificate refuses a first-order derivation
    (`Error::FirstOrder { method: "a certificate over Prop" }`).
  - The manual and the guide for AI models were not changed (the
    design gives them to 74d): the command reads first-order ordinary
    sequents now and answers the refusal (`deduxit prove --logic
    classical 'forall x. p(x) |- p(a)'` exits 2 with "the SAT solver
    takes propositional sequents only"). The manual's wire schema
    (`docs/src/wire.schema.json`) has the code `first_order` but not the
    level-25 forms (`{"∀": a}`, `symbols`, `fo`, predicate atoms,
    `binders`) in its ordinary sequent and derivation.
- **For the planning session:** the D17 cost on `read-tptp` (+1.05 %)
  and the unifier left for 74c, both marked as departures above.

## Session 74c: the quantifier rules and the sessions

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`,
cores 10 to 12, on `main@origin` (`pxkvxurv`). Nothing was pushed.

### Outcome

Items 1 and 2 of the brief landed, and the command part of item 3;
the open witness of the design's section 6, the goal tree's free
functions and item 4 (ILTP's first-order package) did not (below).

| change | subject |
|---|---|
| `vwzkwrvq` | Check the quantifier rules of LK, LJ and LM |
| `yvsrmstl` | Prove first-order sequents step by step in LK, LJ and LM |
| `vwklqmwk` | Answer the panel on the quantifier rules |
| `zyqttuyl` | Answer the review of the quantifier rules |
| `tlnqlrvn` | Move the interactive sessions' unifier to the crate's root |

This report is the change after them.

- **Item 1, the checker** (`vwzkwrvq`): `ordinary::Rule` gains
  `ForallLeft`, `ForallRight`, `ExistsLeft`, `ExistsRight` (after `Open`,
  so that no discriminant moves; `NAMES` and the new `TAKEN` skip `Cut`
  and `Open` by name), and `Inference` a `term`, the witness or the
  eigenvariable, `"term": k` in the JSON (an index of the sequent's `fo`,
  an eigenvariable there `{"E": n}`). `check_inference` takes G3's forms
  and G1's (the principal of `∀L` and `∃R` dropped; `∃R` keeps it in LK
  only), finds the instance with `Formulas::instance` (which builds no
  node), refuses a term on any other rule and a quantifier rule without
  one, and for `∀R` and `∃L` asks that the term be an eigenvariable
  occurring in no formula of the conclusion (`Formulas::mentions`).
  `Derivation::check` refuses a witness holding an open witness. The
  derivation reader takes `{"E": n}` (`serialize::sequents::Extension`)
  and refuses `{"M": k}`; the admissible view keeps a weakening that
  would bring an eigenvariable into its rule's conclusion. The marks
  `expect(dead_code)` on `instance`, `find_term` and `find_list` are
  gone; `instantiate` and `predicate_atom` keep one where `interactive`
  is off, since the goal state is the caller of `instantiate`.
- **Item 2, the sessions** (`yvsrmstl`): `ordinary::Interactive` takes
  first-order sequents (`within` and its reader lifted). `∀R` and `∃L`
  name a fresh eigenvariable, numbered past every eigenvariable of the
  live inferences, so that an undo and the same step again name the same
  one; `∀L` and `∃R` take a witness, `Step::with_witness`, read from text
  by `Interactive::witness` (the sequent's function symbols and the
  eigenvariables in the goal's scope, by the names they print with) and
  held to the goal's scope, as the linear session holds a given witness.
  `Needs::Witness`, `StepError::NoWitness` and `StepError::Witness` are
  new. `∀R` and `∃L` are invertible, `∀L` and `∃R` not. `close` and
  `hint` answer the first-order refusal through `decide` until step 75.
  Eigenvariables print as `a`, `b`, …, skipping the symbols, ranked by
  number (`Formulas::eigen_names`; the linear proofs' scheme, where the
  design said `y`, `z`, …, so both suites print alike).
- **Item 3, the command**: `deduxit interact --logic …` at first order
  (`apply G F forallL TERM`, `rules` saying `∃R (with a witness)`, the
  quantifier rules listed in an unknown rule's message only for a
  sequent with a binder) and `deduxit check` of a first-order derivation,
  both pinned by `first_order_ordinary_session` in `cli/tests/cli.rs`.
  The manual's chapter and the guide are 74d's by the design; nothing of
  them changed here.
- **The unifier's move** (`tlnqlrvn`): the linear session's unifier
  (metavariables, levels, the trail, the occurs check) is
  `crate::unify::Unifier` over the trait `unify::Terms`, which the
  linear session's terms and `FirstOrder` implement; written by the
  sub-agent `deduxit-unify-74c` (Opus 5.5 at `high`) in a workspace of
  its own, brought in by `jj restore`, read and gated here. The linear
  session's behaviour is unchanged (its tests, and the target set below).

### Not done, and why

- **The open witness** (design section 6: `$1`, filled by a later
  axiom's unification, a bind substituting through every stored sequent)
  (departs from the design). A witness is typed or the step is refused.
  The design's substitution needs, for `undo` and for a document read
  back, either the replaced sequents per step or the raw sequents beside
  the shown ones, and a reader that checks every closed inference under
  the bindings made by its step; that is a piece of its own, and it did
  not fit beside the panel's and the reviewer's rounds. What it costs: a
  sequent with no constant, `∀x. p(x) ⊢ ∃x. p(x)`, has no witness to
  type, so the session cannot prove it (the checker takes a derivation
  of it with an eigenvariable as the witness). The unifier it needs is
  at the root now, with `FirstOrder` implementing its trait.
- **The goal tree's free functions** (kernel-boundary item 5) were not
  factored out (departs from the design, section 4a): without the open
  witness the ordinary session shares nothing more with the linear one
  than before, and the item is a refactoring with no caller asking for
  it yet. For the session that builds the open witness.
- **Item 4, ILTP's first-order package**: not reached; for 74d.

### The measurements (D17)

The ratchet's instruction counts (callgrind, the gate's build), against
74b's last:

| journey | 74b | `vwzkwrvq` | `yvsrmstl` | `vwklqmwk` |
|---|--:|--:|--:|--:|
| `read-text` | 12 433 368 | 12 433 368 | 12 433 368 | 12 433 368 |
| `read-json` | 12 226 550 | 12 226 551 | 12 226 551 | 12 226 553 |
| `read-tptp` | 33 661 297 | 33 661 297 | 33 661 297 | 33 661 297 |
| `read-lltp` | 28 308 772 | 28 308 772 | 28 308 772 | 28 308 772 |
| `ordinary-pigeons` | 31 718 052 | 31 750 478 | 31 750 475 | 31 750 475 |
| `render-latex` | 5 780 834 | 5 780 834 | 5 780 834 | 5 780 834 |
| `render-svg` | 79 508 723 | 79 508 723 | 79 508 723 | 79 508 723 |

- `read-tptp` and `read-lltp` did not move, so their tolerance left is
  what 74b left (0.89 % and 0.10 % over their ceilings).
- `ordinary-pigeons` +0.10 % at the checker: every inference now asks
  whether its rule is a quantifier rule and whether it has a term.
- The unifier's move (`tlnqlrvn`) left every journey's count as it was
  at `vwklqmwk`, the search journeys included.
- The focused engine's target set after the move
  (`bench/targets/74c.csv`, the script as it stands, its two streams on
  cores 2 and 3): `summary --against bench/targets/47.csv
  bench/targets/74c.csv --counters` names no verdict that differs, no
  problem one decides and the other not, and of the 145 problems both
  decide alike on one thread none whose `nodes`, `splits`, `memo_hits`,
  `memo_entries`, `links` or `tests` differ.

### The panel

On `vwzkwrvq` and `yvsrmstl`, from one brief
(the claims: the checker's soundness, the readers, the goal state's
steps and reader, the admissible view):
`deduxit-panel-argument-74c` (`panel-argument`, Fable 5.2 at `high`),
`deduxit-panel-counterexamples-74c` (`panel-counterexamples`, Opus 5.5
at `high`), `deduxit-panel-integers-74c` (`panel-integers`, Sonnet 5.5
at `high`). None found an accepted invalid derivation: the
counterexamples member ran 9 000 random sessions in the three logics
(115 138 steps, 51 609 save and load trips), 153 600 mutants of 768
finished derivations and 72 000 mutants of sessions, each closed
inference checked against random classical structures and constant-
domain Kripke models by a reference of its own; the argument member
stated the invariant (the local eigenvariable condition, the instance
by lookup, closed witnesses, injective atom keys) and found it at every
site. Their findings, each answered in `vwklqmwk` with a test:

1. **A panic in the goal state** (argument and counterexamples): an
   instance holding an equivalence the arena lacked had no implications,
   which `shape` looks up (`apply 0 L0 forallL c` then `rules 1 L0` on
   `forall x. (p(x) <-> q(x)) |- p(c)`; the invertible run on
   `|- forall x. (p(x) <-> p(x))`). A quantifier step runs `Extra::add`
   now (`an_instance_s_equivalence_has_its_implications`).
2. **An unbounded check** (argument and integers): a shared term no atom
   names, as a witness, had its instance's key written out in full
   (`t(n+1) = f(t(n), t(n))`: a 1 KB document took 11.6 s and 427 MB at
   26 levels). `Tables::read` counts the largest term against
   `limits.terms` now, and `held` walks a witness with a seen set
   (`documents_with_quantifier_rules_stay_within_their_bounds`).
3. **The session reader** (argument and counterexamples) ran no `closed`
   pass and took `{"M": k}`, which no step makes: a session of
   `p(#0) ⊢ p(#0)` finished as a "valid proof in LK". It runs the pass
   and refuses open witnesses now
   (`a_first_order_session_document_is_held_to_its_steps`).
4. **Two `unreachable!` arms on rule names** (argument): a propositional
   derivation document naming `∀L` passed the arena's door of
   `Tableau::of` and `rules::Derivation::from_ordinary` and panicked.
   `Derivation::propositional` (the arena and no quantifier rule) is
   their door now, and the interpolant's, the term's and the
   certificate's.
5. **`derivation_ids(Admissible)` diverged from the admissible view**
   (argument) on a session document with a weakening below `∀R` that
   brings in its eigenvariable (the web page's map of drawn inferences
   to goals). It maps a first-order state with its formulas now.
6. **A derivation of an arena with a binder and no atom did not read
   back** (counterexamples): `|- forall x. true` was written without its
   terms, so `"term"` named none. `Tables::of_tables` writes the terms
   where there are any.
7. **Cost** (integers): the walks allocated vectors as long as the arena
   per inference (a chain of 20 000 vacuous `∀R` checked in 90 s in a
   debug build), and the fresh eigenvariable walked every inference per
   step. The walks keep sets of what they visit now, and freshness is one
   walk. Looking at that chain, the first-order printer renamed nested
   binders of one name by scanning the scope for each suffix tried
   (cubic in the depth: `check` printing the verdict's sequent took
   minutes); it keeps a next suffix per name now, lowered when a binder
   closes. What remains is the command's text tree size, which it
   estimates even under `--tree never` and which is quadratic in a
   formula's depth, propositional ones too (a follow-up below).

### The reviewer's round

`deduxit-review-74c` (`fresh-reviewer`, Fable 5.2 at `medium`) read the
three code commits against the brief, the design and the rules, and ran
tampered derivations and sessions through the command. It found no
soundness fault, no reachable panic and no dead item in five feature
sets, and:

1. **An empty Rocq certificate with exit 0** for a first-order
   derivation: the command's writer took `Error::FirstOrder` for an
   error of its output. It says `no certificate: …` now
   (`cli/src/show.rs`), pinned in `first_order_ordinary_session`.
2. **No open witness**: the departure above, now in the changelog too.
3. **The session reader took an open goal with a term**, which no step
   makes; refused now.
4. **Two false statements**: the changelog's list of refusals named the
   goal state, and the rules said the session writes no level 25; both
   corrected.

Its question, whether a document marked at a lower level that holds
quantifier rules should be refused: no reader refuses content newer than
its stated level elsewhere (the level is the lowest that holds a
document, and a reader refuses a higher one), so it is left as it is.

### What was checked, and by what

- The gate (clippy, the tests, both `cargo hack` runs, the ratchet),
  capped on cores 10 to 12, at `vwzkwrvq`, `yvsrmstl` and `vwklqmwk`;
  at `zyqttuyl` clippy and `cargo test --workspace`; at `tlnqlrvn` the
  gate again, which covers `zyqttuyl` too.
- The target set after the unifier's move: equal to the oracle (above).
- `nix flake check` at the last commit: in the session's last message.

### What the next sessions must know

- **For 74d, the exports and the docs:**
  - A derivation's inference has `term()`; an eigenvariable prints by
    `Formulas::eigen_names` (`a`, `b`, … past the symbols, ranked by
    number). The exports draw first-order derivations through
    `Formulas::write`; the rule labels `∀L`, `∀R`, `∃L`, `∃R` are in
    `LABELS`. No snapshot pins a first-order drawing yet.
  - The Rocq certificate refuses a first-order derivation
    (`Derivation::propositional`); the command says `no certificate: …`.
  - The manual and the guide describe none of this step: `interact
    --logic` at first order (`apply 0 R0 existsR c`, `forallR`), `check`
    of a first-order derivation, the JSON's `"term"` and `{"E": n}`
    (the wire schema too).
  - ILTP's first-order package (item 4 of 74c's brief) is still to do.
- **For the open witness** (a later session): the unifier is
  `crate::unify::Unifier`, and `FirstOrder<K>` implements `unify::Terms`.
  The design's bind substitutes through every stored sequent; `undo`
  and the reader then need what each binding replaced. The checker
  already takes an inference with a `Meta` witness (`check_inference`),
  and `Derivation::check` refuses one.
- **Follow-ups found, not fixed:** the command computes a derivation's
  text tree size even under `--tree never` (`check` of a 5 000-deep
  formula spends most of its time there, propositional or not).

## Session 74d: the exports, the docs and ILTP's problems

Opus 5.5 at `high`, the same conversation as 74c, in the workspace
`../deduxit-wt-step-58b-8c0bbc`, cores 10 to 12, on 74c's `ttvlrvqv`.
Nothing was pushed.

### Outcome

All four items landed.

| change | subject |
|---|---|
| `ntvpoqru` | Pin the exports of a first-order derivation of LJ |
| `lwzspuss` | Take a witness in the bindings' ordinary step, and run the protocol at first order |
| `pkoxyyvk` | Read TPTP problems without a conjecture, and lemmas and theorems as hypotheses |
| `wnquroov` | Fetch ILTP's first-order problems and read them in the harness |
| `ryyruyqq` | Describe first-order ordinary logic in the manual, the wire schema and the guide |

- **Item 1, the exports**: `first_order_ordinary_derivation` in
  `core/tests/export.rs` builds `∃x. ∀y. r(x, y) ⊢ ∀y. ∃x. r(x, y)` in LJ
  through the public goal state (`∀R`, `∃L`, `∀L a`, `∃R b`, the axiom)
  and pins it in LaTeX, Typst and SVG (`ordinary-first-order.{tex,typ,
  svg}`): the binders (`\forall y.\,`, `∀y.`), the eigenvariables `a`
  and `b` by `Formulas::eigen_names`, the labels `∀L` … `∃R`. The export
  check compiles every snapshot by its glob, so it covers them with no
  change of `modules/export.nix`. No code of the exporters changed: 74b's
  printer and 74c's names were enough.
- **Item 2, the protocol and the bindings**: the protocol's `reads`
  named `limits.terms` everywhere already, and its operations take
  first-order values as they stand (design section 9: no new kind or
  operation). `a_first_order_statement_is_proved_step_by_step_and_rendered`
  pins that `ordinary.decide` and `ordinary.certificate` answer
  `first_order`, `ordinary.start` writes a level-25 session, and a
  finished session's `ordinary.derivation` is read, exported in LaTeX and
  drawn. The bindings' ordinary step (`web/src/ordinary.rs`, `StepForm`)
  takes `"witness": "TEXT"` (read by `Interactive::witness`), and
  `ordinary_rules` writes `"witness": true` only where a rule takes one,
  so propositional answers are as before
  (`a_first_order_step_takes_a_witness`).
- **Item 3, the docs**: the manual's ordinary chapter has a section
  "First-order sequents" (the syntax, the refusal, a session with `∃L`,
  `∃R a`, `∀L a` and `check` of its derivation, the typed witness and the
  open one to come, G3's and G1's forms); `status.md` moves first-order
  ordinary logic from planned to built, with what is still planned;
  `docs/src/wire.schema.json` has the level-25 keys of the ordinary
  sequent (`symbols`, `fo`, `binders`, predicate atoms), `∀` and `∃` among
  the formulas, the quantifier rules, an inference's `term` and `{"E": n}`
  in an ordinary derivation's terms, and the `wire-schema` check writes a
  first-order session and derivation to validate them
  (`modules/_schema/documents.sh`). The guide for AI models
  (`skills/deduxit/reference.md`) has "First-order sequents in LK or LJ"
  with a session that runs, the quantifier rules' spellings, and the
  bindings' `witness`; the summary did not change (it names no ordinary
  session's rules).
- **Item 4, ILTP's first-order problems**: the flake's package
  `iltp-first-order` (`modules/bench.nix`, v1.1.2, no licence stated, as
  `iltp`), `bench/iltp-first-order` ignored by jj. The harness reads them
  with `--tptp`: `load_ordinary` inlines `include('Axioms/…')` from the
  nearest directory above the problem that has the file, and reads a
  Latin-1 byte (some SYJ headers) as U+FFFD; a method's first-order
  refusal is a `refused` row `first_order`, the reader's refusal of `=` a
  `refused` row `equality`. The reader takes a problem without a
  conjecture as the hypotheses' refutation (TPTP's meaning; 182 problems)
  and the roles `lemma` and `theorem` as hypotheses (11 problems).

### The verdict pass

`deduxit-bench run --tptp bench/iltp-first-order/Problems --logics
intuitionistic,classical --engines auto --timeout 2`, three shards on
cores 10 to 12, after the reader's fixes: 2 670 problems, 5 340 rows.

| rows | verdict, reason |
|--:|---|
| 3 356 | refused, equality |
| 1 580 | refused, first_order |
| 103 | proved (ground problems, decided as propositional ones) |
| 34 | unprovable |
| 3 | unknown, timeout |
| 156 | error: a symbol named `par`, a keyword of the linear syntax (78 problems) |
| 104 | error: a name used as a predicate and as a function symbol (52 problems) |
| 4 | error: a variable no quantifier binds (GEJ056+1, GEJ056+2: a fault of the files, whose conjecture writes `(! [A,B] : …) => (? [X] : … apt(A,X) …)`) |

No verdict disagrees with a status. The 130 problems of the first two
error kinds are refusals the design made on purpose (one role per name,
the keywords of every suite refused as names, so that a formula reads
back and a translation keeps its names): accepting them needs a
renaming of names in the TPTP reader, which changes the library's
behaviour, so it is a question for the author, with the recommendation
to rename on reading (`par` as `par_`, a function symbol beside a
predicate of its name as `name_f`), the names TPTP allows being no names
of this syntax. Before the fixes the pass also had 182 problems without
a conjecture, 11 with `lemma` clauses and 98 Latin-1 files as errors.

### The measurements (D17)

The ratchet at `ryyruyqq`, against 74c's last:
- `read-lltp` 28 308 772 → 28 412 656 (+0.37 %; 0.47 % over its ceiling,
  inside the 2 % tolerance): the clause splitter's two roles more and
  its optional conjecture, which LLTP's reader shares.
- `read-tptp` 33 661 297 → 33 667 470 (+0.02 %).
- Every other journey as it was; the exports and the bindings touch no
  journey's code.

### The reviewer's round

`deduxit-review-74d` (`fresh-reviewer`, Fable 5.2 at `medium`) read the
five commits against the brief, the design and the rules, reran the
verdict pass (the same counts up to one timeout) and the docs'
examples (byte for byte), and checked that every `equality` row has a
real `=` or `!=` and that `first_order` comes from the methods alone.
Its finding and questions, each answered in the change after
`ryyruyqq`:

1. **A wrong expected verdict for 120 first-order problems**: an
   `(intuit.)` status of `Open` fell back to the plain classical
   `Theorem`, so step 75 would have called an unprovable verdict a
   mismatch. A status for the logic that claims neither is now no claim
   (`lltp::clauses`); no propositional ILTP or LLTP file reads otherwise.
2. **An empty `.p` file was decided** once a missing conjecture read as
   `Γ ⊢`. `read_tptp` refuses a file of no formula now.
3. **The manual's prose had no word of the level-25 JSON** (the schema
   had every key): the ordinary chapter's first-order section now names
   them.

### What was checked, and by what

- The gate at `ryyruyqq` (clippy, the tests, both `cargo hack` runs, the
  ratchet) on cores 10 to 12; at the review's answer the gate again.
- `nix build` of the checks `export`, `manual`, `manual-links` and
  `wire-schema` at `ryyruyqq`, under the machine lock: passed.
- The verdict pass over ILTP's first-order problems (above).
- `nix flake check` at the last commit: in the session's last message.

### What 74e and step 75 must know

- **74e, the open witness**: the manual's section "First-order
  sequents" says the witness is typed and that a sequent without a
  constant has no proof yet; the open witness changes that paragraph and
  the guide's matching bullet. The bindings' step takes `"witness"` as
  text; an open one is that key left out, which today the library
  refuses with "needs a witness term".
- **Step 75, the searches**: the first-order ILTP pass is the measure
  waiting: `refused first_order` rows turn into verdicts. The statuses
  are read as the propositional ones, and 3 356 rows wait for equality
  (step 78). The command's own TPTP reader (`--input-format tptp`) does
  not inline includes; the harness does.
- **Follow-ups**: the 130 problems the reader refuses by their names
  (above, for the author); the command's text tree size under `--tree
  never` (74c's).

## Session 74e: the open witness

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58a-610ed3`,
cores 7 to 9 (the panel on 13 to 15), on `main@origin` (`xmpyquxt`, rebased at the end onto `ppwlxtmm`, 74d,
then onto `vuvzpwxw`, fixup-13).
Nothing was pushed.

### Outcome

The three items of the brief landed.

| change | subject |
|---|---|
| `uqsmzkru` | Share the goal tree's walks between the linear and ordinary sessions |
| `vprlwqpt` | Record the target set after sharing the goal tree |
| `ukloyqvl` | Leave a quantifier's witness open in the ordinary session |
| `qktxlzlz` | Leave a witness open in interact --logic |
| `komxmpty` | Pin the open witness in the web bindings' ordinary session |
| `xrwyuotv` | Answer the panel's argument member on the open witness |
| `uswrllmz` | Hold a session's bindings to the measure its reader takes |
| `llouuqnp` | Answer the panel's counterexamples member on the open witness |
| `oqxqwwzw` | Answer the review of the open witness |

This report is the change after them.

- **Item 1, the open witness** (`ukloyqvl`), as the design's section 6
  has it, with the choices below.
  - `∀L` and `∃R` without a witness put `Meta(k)` (`$1` for the first)
    in the instance. Its level is one more than the largest eigenvariable
    in the goal's scope.
  - The axiom, where no formula on the other side is equal to the
    step's, takes the first that unifies: `Formulas::term_pairs` gives the
    term pairs of the two formulas (the left one first), and the shared
    `crate::unify::Unifier` binds them under the step's number. A binding
    that would take an eigenvariable at or above a witness's level, or
    one outside the goal's scope, is `StepError::Binding` (new), which
    names the eigenvariable.
  - **A bind substitutes** (the design's "a bind substitutes"): every
    inference shows the bindings in force (`bind`, a `Resolution` that
    replaces each bound witness, through chains of bindings), so
    `check_inference`, the views, the exports and the command read them
    unchanged. Every inference that changes is checked first; one that a
    binding would break refuses the step and changes nothing.
  - **What `undo` and the reader need** (the 74c report's open
    question): the session keeps, beside each inference a binding
    changed, what it was made as (`Witnesses::raw`), and `undo` shows the
    rest again from it (`reshow`). This is the "raw sequents beside the
    shown ones" of the two ways 74c named; the other, the replaced
    sequents per step, holds a copy per binding where this holds one per
    inference.
  - **The finished proof** (the brief's question): a witness no axiom
    bound becomes, once no goal is open, an eigenvariable numbered past
    every one the arena holds, which no `∀R` or `∃L` introduces, so the
    checker reads it as a constant (`finishing`). This is the linear
    session's rule (`proof` there replaces an open witness `k` by the
    eigenvariable `eigens + k`), and the checker's local eigenvariable
    condition takes it. `∀x. p(x) ⊢ ∃x. p(x)` with no constant proves in
    LK, LJ and LM (`an_axiom_fills_the_open_witnesses`). While a goal is
    open the derivation shows `$1`; LaTeX writes `\$1`, Typst a string.
  - **The document**: the inferences as made and `"bindings": [[k,
    term, step]]` (written only where there is one), at level 25, which
    no release has read yet, so no new level. The reader replays the
    steps in order (`Replay`, run only for a document with an open
    witness): each step's witness made at the level the tree gives, an
    axiom step whose `A ⊢ A` was made unequal binding by the same
    `unify_axiom` on those two made formulas (the very formulas the live
    step unified), every inference of the step checked under the
    bindings then, everything again under the final bindings, and the
    bindings found compared with the document's. A propositional or
    witness-free document reads as before.
  - `rules` offers the axiom where a formula unifies; the invertible run
    closes equal formulas only, and `invertible_steps` leaves out an
    axiom that bound a witness (a binding is a choice).
  - `Interactive::bindings()` (new) gives each bound witness's name and
    term as text.
  - The shared unifier now refuses a binding to a bound variable
    (`Clash::Terms`, where it had `unreachable!`): the ordinary session
    unifies whole formulas, whose atoms under a binder hold its
    variable. The linear session's terms are closed, so nothing changes
    for it.
- **Item 2, the goal tree's free functions** (`uqsmzkru`,
  kernel-boundary item 5): `crate::goal_tree` holds `subtree`,
  `postorder`, the reader's history walk `added` and the graft's
  `grafted`, over each inference's premises; both sessions call them,
  each reader keeping its own messages. The open witness's reader is the
  third caller of `added` (it needs each step's range of inferences).
  The linear session's behaviour is unchanged: its tests, and the target
  set (below).
- **Item 3, the command and the bindings** (`qktxlzlz`, `komxmpty`):
  `interact --logic` takes `apply G F forallL` with no term, lists `∀L
  (with a witness or none)`, prints `filled $2 with $1` after the axiom,
  and refuses a binding with the eigenvariable named
  (`ordinary_open_witness` in `cli/tests/cli.rs`). In the web bindings
a step without `"witness"` (74d gave the step that key) leaves the
witness open, and the session document carries the bindings between
calls (`an_open_witness_is_filled_by_the_axiom`). The manual's
first-order section (74d's) says that a witness may be left open, with
an example, and its JSON paragraph the `{"M": k}` and `bindings`; the
guide has one bullet, the wire schema the `bindings` key of the
ordinary session, the changelog one entry: the witness's lines only.

### Departures

- **The level is one more than the largest eigenvariable in the goal's
  scope**, where the linear session counts the eigenvariables made
  before the witness, anywhere (departs from the design, which says
  "levels, as in 42e"). The reader then computes it from the tree alone
  (`Tree::level`), so the document holds no levels. The two refuse the
  same bindings once the scope is asked too: an eigenvariable made later
  anywhere has a number at least the level (eigenvariables are numbered
  past every live one), and one of a sibling branch made earlier is
  refused by the scope.
- **The document holds the inferences as made, not as shown** (the
  design named no form): the reader replays the bindings from them, so
  an undo after a read takes them back as in a live session.

### The measurements (D17)

The ratchet's instruction counts (callgrind, the gate's build) at
`komxmpty`, against 74c's last (`vwklqmwk`):

| journey | 74c | 74e |
|---|--:|--:|
| `read-text` | 12 433 368 | 12 433 368 |
| `read-json` | 12 226 553 | 12 226 553 |
| `read-tptp` | 33 661 297 | 33 661 297 |
| `read-lltp` | 28 308 772 | 28 308 772 |
| `ordinary-pigeons` | 31 750 475 | 31 750 492 |
| `render-latex` | 5 780 834 | 5 780 834 |
| `render-svg` | 79 508 723 | 79 508 723 |

- At `llouuqnp` (after the panel's fixes) the gate counted `read-tptp`
  at 33 736 850, 0.22 % above its count at `komxmpty`, within the
  tolerance (1.12 % over its ceiling, where 74b left 0.89 %). The
  journey reaches none of the code those commits changed. Counted in
  one build environment for both (`ratchet --only read-tptp --keep`,
  `callgrind_annotate` by function): 33 684 493 at `komxmpty`, 33 736 850
  at `llouuqnp`, the whole difference one inlining decision of the
  fat-LTO build, the interner's `HashMap<String, Symbol>` lookup now a
  call of its own (+431 040) where it was inlined into `Parser::formula`
  (−355 487), every other function within a few hundred instructions.
  By `bench.md` that is a question for pinned time, not a regression;
  no pinned run was made (a timed run the brief does not name).
- No journey drives a session; the propositional paths the change
  touches (the session's reader and writer, `Substitution`, the
  serializer's `Extension`) are on none of them, and every count but
  `ordinary-pigeons` (+17 instructions) is 74c's to the instruction.
- The focused engine's target set after the goal tree's move
  (`bench/targets/74e.csv`, the script as it stands, its two streams on
  cores 2 and 3 as the script pins them): `summary --against
  bench/targets/47.csv bench/targets/74e.csv --counters` names no verdict
  that differs, no problem one decides and the other not, and of the 145
  problems both decide alike on one thread none whose `nodes`, `splits`,
  `memo_hits`, `memo_entries`, `links` or `tests` differ.

### The panel

On `ukloyqvl` to `komxmpty`, from one brief (the claims: the soundness
of every state a session reaches or reads, undo, the reader, the levels
and scope, the bounds, the goal tree):
`deduxit-panel-argument-74e` (`panel-argument`, Fable 5.2 at `high`),
`deduxit-panel-counterexamples-74e` (`panel-counterexamples`, Opus 5.5
at `high`), `deduxit-panel-integers-74e` (`panel-integers`, Sonnet 5.5
at `high`), on cores 13 to 15. None found a finished derivation that
fails the checker: `bind`'s check of every inference a binding changes,
the reader's checks and the final `Derivation::check` held. Their
findings, each answered in `xrwyuotv`, `uswrllmz` or `llouuqnp` with a
test:

1. **The reader panicked or accepted what no session writes**
   (argument, integers): a `∀L`/`∃R` with a `Meta` term in an inference
   no step closed was never made a witness (a 433-byte document panicked
   in `Unifier::resolve`); a session's history forged to `[]` read with
   no witness and then panicked or failed `finished`; a cut formula
   holding `$4000000001` read and `rules` then panicked. The reader now
   refuses a closed inference no step closed, takes the replay for any
   document whose formulas hold a `Meta`, and refuses a step whose
   inferences or premises hold a witness not made yet
   (`a_witness_no_step_left_open_is_refused`).
2. **A binding built an equivalence without its implications**
   (argument): `rules` then panicked on `p(a) ↔ q` after `∀x. ((p(x) ↔
   q) ∧ s(x)) ⊢ s(a) ∧ u`'s four steps, a finished proof failed its
   check, and a sound binding was refused as an eigenvariable fault.
   `bind`, `finishing` and the reader add the implications of what they
   build (`implications_from`;
   `a_binding_builds_an_equivalence_s_implications`).
3. **The reader refused two documents sessions wrote** (argument): an
   axiom after a weakening the user took was read as the weakening's
   step, and an axiom whose goal an earlier binding had changed was
   unified on the sides as first made. The chain now stops at a goal
   another step closed, and the reader unifies the made formulas
   resolved under the bindings before the step, which are the shown
   formulas the live step unified (`a_session_s_own_bindings_read_back`).
   The rules file's claim that the made formulas are those the live step
   unified was wrong for the second case.
4. **Bounds** (argument, integers): a chain of bindings of shared terms
   (20 links: 3.9 s, 44 MB of atom keys) and the reader's rebuild of
   every inference per step (4 000 steps: 32 s). The reader builds a
   step's inferences per step and all of them once at the end (0.34 s
   where it was 103 s, integers' N = 400). A binding is held to
   `limits.terms` by the reader's own measure, predicted before anything
   is built (`uswrllmz`): a per-binding bound alone (`xrwyuotv`) let a
   session of 96 steps write a 129 KB document its reader refused, with
   a peak of 1.5 to 4.8 GB; now that session's 91st step is refused
   (83 886 254 symbols against 50 000 000), and at 22 links its document
   reads back as itself (`a_binding_is_held_to_the_bound_on_terms`).
5. **Undo did not give back the document byte for byte** (argument): a
   step that built nodes leaves them in the arena, which the document
   writes whole. The state is otherwise the one before, and a typed
   witness's instance (74c) behaves alike. Not changed: the arena only
   grows, as everywhere in the goal state; the claim in the brief was
   too strong, and the rules file says so.
6. **Not fixed, a follow-up** (integers): a unifying axiom shows the
   bindings through every inference, so a session's time grows with
   the square of its length: 25 ms per axiom at 1 200 steps of width
   100, 334 ms at width 400; an undo of a binding likewise. The linear
   session shows its whole table after every change too. Restricting the
   walk to the inferences that hold a newly bound witness needs a map
   from witnesses to inferences.

7. **A finished derivation's document held the session's open
   witnesses** (counterexamples): the derivation's arena kept `{"M": k}`
   terms its inferences no longer used, and the derivation reader refuses
   any, so the manual's own example wrote a `proof --json` that `deduxit
   check` refused (2 999 of 3 558 finished derivations of one run).
   `finishing` now turns every `Meta` term of the copy, in place, into a
   fresh eigenvariable and writes the atoms' keys again
   (`an_axiom_fills_the_open_witnesses` reads the derivation back and
   checks it).
8. **A document read until a witness was opened** (counterexamples): an
   inference no step closed (a history forged to `[]` under a closed
   root) is taken by the plain reader, but the replay refused it once a
   step left a witness open. The replay checks such inferences before
   its first step, holding no witness
   (`a_witness_no_step_left_open_is_refused`).
9. **An eigenvariable near `u32::MAX`** in a forged document makes
   `finished` refuse the complete session with the index refusal
   (counterexamples, integers): a refusal, not a fault; not changed.

The counterexamples member ran 10 767 random sessions on `xrwyuotv`
(112 742 steps taken, 12 794 refused, 25 578 witnesses left open, 1 570
axioms that bound, 3 558 finished derivations, 56 354 documents read
back, 59 356 steps of a twin read from JSON in lockstep), 69 170
mutated documents (21 282 accepted, each driven 8 steps and undone), and
11 308 sessions and 66 300 mutants on `komxmpty`, each closed inference
checked against its own semantic reference (random structures of 1 to 3
elements) and every finished derivation's root against countermodels.
None found an unsound state; findings 1 to 3 and 7 to 8 are the classes
it found, fixed before its last runs or after. The integers member drove
the witness and eigenvariable numbers, the steps' numbers, the bindings'
indices and every walk to their limits: refusals or bounded, but for
findings 4 and 6.

### The reviewer's round

`deduxit-review-74e` (`fresh-reviewer`, Fable 5.2) read the chain up to
`llouuqnp` against the brief, the design and the rules, and ran 75
seeds of random sessions in its own copy (3 775 steps that bound, 1 052
finished proofs: every property held but the first finding). Its four
findings, answered in `oqxqwwzw` with a test each:

1. **A panic after an undone open-witness step**: the undone step's
   atom stays in the arena with a witness numbered past the live ones,
   and the measure of the arena resolved it (`interact --logic` on
   `∀x. ∀y. r(x, y) ⊢ ∀y. r(a, y)`, two `∀L` without a witness, `undo`,
   `rules 1 L0`: exit 101). `unfolded` takes such a witness as a leaf
   (`an_undone_witness_left_in_the_arena_is_passed_by`).
2. **The reader refused a session's own document under a bound on
   terms**: it added an axiom step's growth to the last step's arena.
   The reader binds with the per-binding bound alone, its arena being
   bounded by `Tables::read` already
   (`a_session_under_a_bound_on_terms_reads_back_under_it`). The
   reviewer also noted, outside this change, that a typed witness's
   instance (74c) is held to no bound on terms, so a session under a
   small bound can write a document its reader refuses; left as it is.
3. **The command printed the wrong filling** when an axiom filled an
   earlier-numbered witness than the last one filled (`bindings` comes by
   witness number): it compares by name now (`ordinary_open_witness`).
4. **The rustdoc's JSON section** of `ordinary::Interactive` did not
   describe `{"M": k}`, `bindings` and the inferences as made; it does.

It confirmed the target set against the oracle (284 rows, the same
verdicts, every counter of the decided rows equal) and the manual's,
guide's and changelog's lines.

### What was checked, and by what

- The gate (clippy, the tests, both `cargo hack` runs, the ratchet),
  capped on cores 7 to 9: at `komxmpty`, at `llouuqnp`, after the
  rebase onto 74d at `oqxqwwzw`, and after the rebase onto fixup-13 at
  `ktsrqsps`: passed each time.
- Clippy and the tests at every other commit; both `cargo hack` runs
  after the feature gating (`Replace::Metas`, `goal_tree::added`,
  `holds_open_witness`).
- The target set after the goal tree's move: equal to the oracle
  (above).
- `nix flake check` under the machine lock: every check passed at the
  report's commit on 74d's base; at the last commit, on fixup-13's base,
  in the session's last message.
- The rebase onto `main@origin` (`ppwlxtmm`, 74d's chain) met 74d's
  lines in the changelog, `web.md` and the web test (both tests kept;
  74d's `StepForm` takes a `"witness"` now, so a step without one is
  what leaves it open) and in the manual's new first-order section,
  whose "the witness is always typed" and `(with a witness)` lines
  changed to the open witness's; the paragraph and example of
  `qktxlzlz` moved into that section. The rebase onto fixup-13 met its
  doc comment of `ordinary_session` beside the new test (both kept).

### What step 75 must know

- A search's derivation grafted onto a session goal (`close`) holds no
  open witness; a goal that holds one is decided by `decide`, which
  refuses first-order sequents until step 75. When 75 lets `close` and
  `hint` decide a first-order goal, a goal with open witnesses needs the
  linear session's treatment (`handover.rs`: the witnesses as `∃` and the
  later eigenvariables as `∀` in a prefix), and the graft must bind them;
  the graft today keeps no `raw` for the inferences it adds.
- `Witnesses::raw` is `Some` only where a binding changed an inference;
  any new code that replaces an inference's sides (a graft, a cut) must
  keep it, or an undo shows the wrong sequent.
- `unify_axiom` and `Formulas::term_pairs` unify whole formulas; a
  search that unifies literals (75) can call the same `Unifier`.
