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
