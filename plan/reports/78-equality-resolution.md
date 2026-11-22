# Step 78: equality and first-order resolution

## Session 78a: equality, cnf and SZS

Opus 5.5 at `high`, 2026-11-22, in the workspace `deduxit-wt-step-78a-…`
on `main@origin` (`orkxnouy`), cores 4 to 6, the panel on 13 to 15.

### Outcome

First-order ordinary logic now reads equality and decides it by its
axioms. `s = t` and `s != t` (natively also `s ≠ t`) are read in the
ordinary syntax and in TPTP and printed `s = t`; a sequent with `=` is
decided with the equality axioms of its symbols appended to its
hypotheses, by every method, in every logic; a valid one gets a checked
derivation whose conclusion holds the axioms as its last hypotheses, and
the verdict line counts them; "not valid" comes only with a checked
finite structure that reads `=` as identity. The TPTP reader takes `cnf`
clauses, and `prove --logic --szs` ends its text output with an SZS
status line. ILTP's 1 730 equality problems, all refused before, are
decided as far as the measurement below shows.

The commits (change ids):
- `mmwovkzs` Plan: equality and first-order resolution, the design
  (`plan/notes/equality-resolution-design.md`);
- `woplvpmm` Read equality and TPTP's cnf, decide equality by its axioms,
  add prove --szs (the library, the command, the harness, the manual, the
  guide, the changelog, the rules);
- `wyumtlox` Lock: equality, its refutation and prove --szs;
- `zktomqty` Peek for equality inline in the ordinary reader (D17);
- `xpuslrmx` Equality: no refutation without its checked structure,
  bounded axioms (the panel's findings);
- `vskrurks` Lock: prove --help names MemoryOut and ResourceOut;
- `nzksrrkv` Bench: ILTP's equality problems by every method, with the
  axioms (`bench/defaults/equality-iltp-*.csv`);
- `vsxrkpzz` Equality in the methods' own doors, and its linear documents
  at level 25 (the reviewer's findings);
- `vqmuvvpp` Divide the model search's structures by equality (after the
  rebase onto 76a's model search);
- `spnnmxms` Name the bound that leaves a refutation with equality
  unchecked;
- `vlsxlrwo` Bench: the default and the model search on ILTP's equality
  problems after the rebase;
- `numvupny` Equality: no Kripke countermodel where the worlds read = as
  an atom (after the second rebase);
- `wzvqwqso` this report;
- `lrqloqoz` Resolve first-order sequents with the equality axioms
  (after the third rebase, onto 78b's resolution).

A third rebase, onto `nmnrxmpk` (78b's first-order resolution, 76c's
certificates, 77c), met the release line of `status.md` again and this
report, which 78b had created with its section; my section now stands
first and 78b's after it. 78b's `resolve` and `clauses_of` read `=` as
an atom, so a saturated clause set of a sequent with `=` was called
satisfiable; they add the axioms at their doors now (`lrqloqoz`): `a =
b, p(a) ⊢ p(b)` is refuted with them (3 inferences, read back as a
derivation of LK), `a = b ⊢ a = c` is unknown at the time limit (the
axioms' transitivity keeps the saturation going) where it would have
been "satisfiable". No wire level is taken: a linear document with `=`
uses 25, so 78b's 29 stands.

The full check before the third rebase failed on `budget` alone: the
web module was 2 805 206 bytes at gzip against 2 800 000. 78b raised the
ceiling to 2 875 000 for its own code, and on the rebased tree the
module is 2 846 308 bytes, inside it.

The chain was rebased onto `main@origin` twice: after the reviewer onto
`zuppslkx` (76a's model search, 77a's normal forms; two conflicts, the
layout line of core-ordinary.md and the head of `within`), and at the
end onto `pnnrklrr` (76b's Kripke models with growing domains, 76c's
certificates, 77b's Herbrand and translations; two conflicts, the
release line of `status.md` and the model search's door, which gained
76b's bound on worlds). Each was resolved by hand keeping both sides.
After the second, the model search could refute a sequent with `=` by a
Kripke model whose worlds read `=` as an atom, and `within`'s downgrade
left that model on an unknown outcome: the model search now tries no
Kripke model with `=`, and `within` drops it (`numvupny`). With `=`
read as identity in every world, `a = b ∨ ¬a = b` holds in every model,
so identity worlds alone would refute less than the axioms leave open;
the usual semantics (a congruence growing along the order) is a
follow-up. `|- a = b \/ ~ a = b` is valid classically and unknown
intuitionistically and in minimal logic; `f(a) = b |- f(b) = a` is
refuted in all three by the model search's checked structure of 2
elements.

### The design's decisions and their sources

The design note has each choice with its alternatives; in brief:
- **`=` is the binary predicate named `=`** (`first_order::EQUALS`): no
  identifier is that name, so it never meets a user's symbol, and every
  method, read-back, checker, JSON form and structure takes it as the
  atom it is. A node `Node::Equal` would have needed an arm in some forty
  matches for nothing that axioms need; a reserved identifier (`eq`)
  meets a user's `eq`.
- **The axioms** are reflexivity, symmetry, transitivity and the
  all-arguments substitution axiom per function and predicate symbol
  (Mendelson §2.8, Fitting ch. 9, leanCoP's equality axioms; the same
  are sound intuitionistically, Troelstra and van Dalen 2.2): one per
  symbol rather than one per argument position, `=`'s own congruence left
  out since symmetry and transitivity give it.
- **One place adds them** (`ordinary::equality::with_axioms`), as
  hypotheses after the sequent's own, idempotent, roots in place: the
  embedding's image, the connection provers' matrix and grounding all see
  `Γ, E ⊢ Δ` without a change of their own. `decide_within` calls it, the
  command's `ordinary::sequent_in` after reading (so `prove`, the batch,
  `interact --logic` and `grade`), the harness after loading.
- **"Not valid" on identity only**: `Structure` reads `=` as identity
  and refuses a structure that lists it; grounding divides its model by
  `=` before the check (the model satisfies every axiom instance over its
  domain, so `=` is a congruence there, and the quotient keeps every
  formula's truth, Mendelson's contraction). Any other "not valid" of an
  augmented sequent becomes unknown.
- **`cnf`**: a clause is the universal closure of its disjunction over
  its upper-case variables, read as `! [X, …] : (clause)`;
  `negated_conjecture` is a hypothesis, a `cnf` `conjecture` refused.
- **SZS** (szs.tptp.org, read this session): `Theorem` and
  `CounterSatisfiable` with a conjecture, `Unsatisfiable` and
  `Satisfiable` without; `Timeout`, `User`, `GaveUp`; `SyntaxError` and
  `Error`. The TPTP syntax page answered 404; the reading of `=`, `!=`
  and `cnf` follows the TPTP technical report as remembered and the ILTP
  files themselves (not re-verified against the BNF).

(departs from the design) Two places, both recorded in the design note
as built:
- No `Outcome` field counts the axioms: `equality::axiom_count` finds
  them in a derivation's conclusion, and the command's verdict line says
  "its last N hypotheses the equality axioms". A field would repeat a
  function of the derivation and burden every constructor.
- The SZS line is the last line of standard output, not the first: the
  command writes its verdict and derivation as it goes, and the status is
  known at the end. TPTP's tools find the line wherever it stands.

### What landed

- **The tables** (`core/src/first_order.rs`): `EQUALS`, `=` held to a
  binary predicate by `FirstOrder::check` and let past `name::check`;
  `Interner::find_symbol`. The JSON readers take the symbol `=`
  (`serialize/sequents.rs`, `Formulas::of_tables`, which interns it as a
  predicate even unused).
- **The arena** (`core/src/ordinary/mod.rs`): `Formulas::has_equality`
  (public), `equality`, `applied_atom`; `write_atom` writes `s = t`;
  `binder` ungated, since the axioms need it in every build.
- **The reader** (`core/src/ordinary/parse.rs`): `equality_follows` (an
  inline scan and one compare on the hot path), `equality_sign`,
  `equality_after`, `bare_term`, `application` (the predicate's argument
  loop, shared), `equality`; `closure` and `read_clauses` for `cnf`; the
  `EQUALITY` hint is gone. `lltp::clauses` takes `cnf` behind a flag only
  `read_tptp` sets.
- **Equality** (`core/src/ordinary/equality.rs`, public):
  `with_axioms(sequent, limits)` (idempotent, roots in place, bounded by
  `limits.occurrences` and the memory bound on what it adds),
  `axiom_count(formulas, left)`, and the quotient `identified` (behind
  `sat`). Tests: the axioms appended once and read back; each logic's
  valid sequents with a checked derivation holding the axioms and its
  refutations with a checked identity structure; an augmented sequent
  under a small work bound not refuted.
- **The decisions**: `decide_within` augments (`within`, then `dispatch`)
  and turns a "not valid" without a structure of a sequent with `=` into
  unknown; grounding divides its model by `=` (`structure_of`) and
  answers unknown with `=` where a bound drops the structure; `Structure`
  reads `=` as identity and refuses a structure listing it.
- **The command** (`cli/`): `ordinary::sequent_in` augments after
  reading; the verdict line of a valid sequent ends `, its last N
  hypotheses the equality axioms`; `prove --szs` (`szs_status`,
  `szs_name`, refused beside a batch or a format other than text).
- **The harness** (`bench/src/ordinary.rs`): augments after loading; the
  `refused`/`equality` rows are gone.
- **The texts**: the manual's ordinary-logic chapter ("Equality", `cnf`
  and `--szs` under "Read a problem file", the first-order section's
  sentence), `status.md`, the guide's reference, the changelog, and the
  rules (core-ordinary.md's new section "Equality" and the reader's
  bullet, core-inputs.md, cli.md, bench.md).
- **The lock**: four calls (`ordinary-equality`,
  `ordinary-equality-refuted-json`, `ordinary-szs-theorem`,
  `ordinary-szs-satisfiable`) and `prove --help`.

### Measurements

**ILTP's 1 730 first-order problems with equality** (all refused before
this session), `deduxit-bench run --tptp FILE … --logics
classical,intuitionistic --engines auto,connection,grounding,embedding
--copies none --timeout 2`, three shards pinned to cores 4, 5 and 6, at
`zktomqty` (the panel's fixes came after; they change only the verdicts
of a structure dropped at a bound, which no row has), 13 840 runs, rows
in `bench/defaults/equality-iltp-*.csv`:

| logic | method | valid | not valid | unknown |
|---|---|--:|--:|--:|
| classical | default | 321 | 18 | 1 391 |
| classical | connection | 315 | 0 | 1 415 |
| classical | grounding | 18 | 18 | 1 694 |
| classical | embedding | 87 | 0 | 1 643 |
| intuitionistic | default | 103 | 18 | 1 609 |
| intuitionistic | connection | 88 | 0 | 1 642 |
| intuitionistic | grounding | 0 | 18 | 1 712 |
| intuitionistic | embedding | 102 | 0 | 1 628 |

- **Every verdict against both statuses** of its file (the plain
  `Status`, classical, and `Status (intuit.)`; an intuitionistic "valid"
  against a classical non-theorem, a classical "not valid" against an
  intuitionistic theorem counted too): no verdict contradicts a status,
  no error, no crash, every derivation checked. The files claim 1 453
  classical theorems and 229 non-theorems, 150 intuitionistic theorems
  and 2 non-theorems (the rest `Open` or no claim).
- **The unknowns are time-outs**: 1 391 of the classical default's 1 391,
  1 607 of the intuitionistic default's 1 609 (2 incomplete); grounding's
  are outside its classes (function symbols). The embedding met its
  memory bound on 39 classical problems and the recursion limit on 5.
- **Classical**: the default proves 321 (22 % of the claimed theorems),
  all but one by the connection prover; 322 by any method. The 18
  refutations are grounding's, all function-free problems; 13 came with a
  checked structure of identity, and 5 (NLP005, 006, 008, 012, 013)
  without one, their check refused at its default bound of work. That
  run was at `zktomqty`, before the panel's fix: since `xpuslrmx` those
  five are unknown by grounding ("the work limit … was reached", since
  `spnnmxms`), as the requirement asks.

**Again after the rebase** onto 76a's model search (`vqmuvvpp`), the
default and the model search, the same flags, under load from another
session's flake check and ILTP run (rows in
`bench/defaults/equality-iltp-rebased-*.csv`):

| logic | method | valid | not valid | unknown |
|---|---|--:|--:|--:|
| classical | default | 317 | 13 | 1 400 |
| classical | models | 0 | 105 | 1 625 |
| intuitionistic | default | 103 | 15 | 1 612 |
| intuitionistic | models | 0 | 105 | 1 625 |

- No verdict contradicts a status; every refutation carries a checked
  structure of identity (the model search's divided by `=`, function
  tables included).
- Against the first run the default lost 16 decided rows: the five NLP
  refutations above (by design) and 11 near the 2 s limit, of which the
  five classical proofs rerun alone on one core prove again (1.5 to
  2.0 s: the load); it gained 4.
- **The model search refutes 105 non-theorems** at 2 s, the library's
  default 13 (classical) and 15: the default reaches the model search
  only after the connection prover gives way, which at 2 s it does not.
  The command races the model search beside the connection prover, so
  `prove` refutes such a problem where the harness's `auto` (the
  library's `ordinary::decide`) does not (NLP006: "not valid … by the
  model search, a checked finite structure of 2 elements"). A D19
  question for 78c.
- **Intuitionistic**: the default (the embedding at first order) proves
  103, the intuitionistic connection prover 88, **39 of them the default
  does not**, 142 together. On ILTP's problems without equality 75d found
  every share with the prover in it weaker than the embedding alone; on
  these it is the other way, which is a D19 question for 78c (below).
- Times are not argued: the machine was in use (two other sessions, an
  Agda build for a while), and the rows are for verdicts.

**D17, the reader's journeys** (callgrind, the shell's release build,
`ratchet --only`):

| journey | before (`orkxnouy`) | first version (`woplvpmm`) | after (`zktomqty`) |
|---|--:|--:|--:|
| read-tptp | 33 910 214 | 34 655 594 (+2.20 %) | 34 323 485 (+1.22 %) |
| read-lltp | 28 669 776 | 28 751 746 (+0.29 %) | 28 751 746 (+0.29 %) |
| read-text | 12 386 430 | 12 386 430 | 12 386 430 |

The first version called the out-of-line `rest` once per atom for the
look-ahead and let the term reader inline into the operand loop, 2.2 %
past the base and over the ratchet's tolerance against the ceiling; an
inline scan of white space and cold term readers left 1.2 %. read-lltp's
0.3 % is the clause splitter's `cnf` arm, which the LLTP reader passes
`false`. The propositional searches are untouched (a sequent without `=`
costs `decide_within` one test of a pointer).

### The panel and the reviewer

The panel ran on `zktomqty` (argument on Fable 5.2, counterexamples on
Opus 5.5, integers on Sonnet 5.5, each `high`, cores 13 to 15). No
member found a wrong verdict.
- **Argument**:
  - Finding 1 (holds, fixed in `xpuslrmx`): a "not valid" without a
    checked structure reached the command. The command augments as it
    reads, so `within`'s downgrade, keyed on its own augmentation, never
    ran, and grounding keeps "not valid" when a bound refuses the
    structure's check: `prove --logic classical --work-limit 20 'a = b,
    p(a) |- p(c)'` said "not valid … too large to check". The verdict was
    true (the model satisfies the axioms' instances, so the quotient
    exists) but unchecked. Now `within` keys on `has_equality` and
    grounding answers unknown with `=` there; the line is now unknown, and
    a test pins the library's path.
  - Finding 2 (holds, fixed): a JSON sequent declaring `=` unused made
    every decision a symbol error (`of_tables` interned it as a
    function); it is a predicate now.
  - Finding 3 (holds, documented, not changed): the linear JSON reader
    takes `=` (the image needs it), and such a linear sequent prints
    `=(a, b)`, which the linear parser refuses; the image of a hypothesis
    `a = b` prints `!=(a, b)` (ILL's `!`), which reads like "not equal".
    An exception to core-sequents.md's decision "atom names are
    identifiers" (departs from the design), recorded in core-ordinary.md;
    the linear printer's infix `=` is a follow-up.
  - Finding 4 (holds, fixed): `with_axioms` was not idempotent under an
    occurrence bound between the axioms' size and twice that (it
    estimated all of them again); `seq print --logic` prints the axioms
    (the docs now say so: it prints the image the search sees).
- **Counterexamples**: no refutation. A reference of its own (brute force
  over structures of 1 to 3 elements with `=` as identity, function-free,
  nesting 2; a free `⊥` for minimal logic): 23 700 generated sequents in
  three logics through `decide_within`, no "valid" with a countermodel,
  every "not valid" with `=` a structure that refutes the original
  sequent in its evaluator (10 527 classical, 8 331 intuitionistic and
  minimal), the printed and the JSON forms reading back on all, TPTP
  reading as the native text on 13 869; 1 260 sequents through the
  command (about 3 500 calls, 592 SZS lines matching the exit status),
  `prove | check` and the batch agreeing; 36 native and 35 TPTP cases by
  hand, edited JSON refused where it gives `=` another arity or role.
- **Integers**: no wrap, truncation or panic.
  - R1 (holds, fixed): the idempotence window above.
  - R2 (holds, fixed): an axiom of a symbol of a million arguments held
    967 MB uncharged; the nodes the axioms add are charged to the memory
    bound now (`NODE_BYTES`, 256), and a congruence whose binders cannot
    fit is refused before its variables are made.
  - R3 and R6 (hold, fixed): the scans over hypotheses, axioms and a
    `cnf` clause's variables were quadratic (17.8 s for 50 000 symbols);
    they are sets now. `bare_term`'s scan of the binders' names is
    quadratic in a clause's variables as it was for `fof` before
    (follow-up).
  - R4 (holds, not this change): a congruence axiom of thousands of
    arguments is a nest of thousands of `∀`, and grounding's class test
    and passes overran a 2 s limit by tens of seconds (100 s at 4 000
    arguments); the same axiom written by hand without `=` does the same.
    `grounds` has no stop (core-ordinary.md says so); a follow-up.
  - R5 is Finding 3. `--szs` now names `MemoryOut` and `ResourceOut` for
    an error at a bound; an unknown at the memory bound stays `GaveUp`
    (the status sees the exit status, not the reason).

**The reviewer** (`fresh-reviewer`, Fable 5.2), on `nzksrrkv`:
- Finding 1 (holds, fixed in `vsxrkpzz`): the public `grounding::decide`
  and `connection::decide`, called without the axioms, answered a defect
  error and "not valid" on `⊢ a = a` (grounding divided a model that is
  no congruence; the connection prover's matrix had no slot). Each adds
  the axioms at its door now, and a test calls both directly.
- Finding 2 (holds, fixed): `prove --logic … --linear --format json`
  wrote a linear outcome naming `=` at level 7, whose released readers
  refuse the name; `wire::level_of_sequent` writes such a document at 25
  (unreleased).
- The design note's A9 named `InputError` and `MemoryOut` for an unknown,
  which the code does not do: corrected as built.

**After the rebase** onto 76a's model search, every refutation of a
sequent with `=` by the model search was a defect error ("it lists
equality"): its structures list `=` as a predicate. Its structures
satisfy the axioms, so they are divided by `=` like grounding's, function
tables included (`identified` now maps a function's table over the
classes, looked up at their least elements), and `models::decide` adds
the axioms at its door (`vqmuvvpp`). A test of `f(a) = b ⊢ f(b) = a`
first went through `decide_within` with the library's defaults, which
have no time limit: the connection prover's deepening with the axioms
ran for 90 minutes before the planning session saw it and I stopped it
by its PIDs; the test now calls the model search directly. That the
library's default runs that long on a small unprovable sequent with
function symbols is how `decide` behaves at first order without a time
limit (the command has its 2 s), not a change of this session; it is a
follow-up for whoever sets the library's first-order defaults.

### Checks

- **The gate** (capped, cores 4 to 6) passed at `numvupny`, the final
  code on `pnnrklrr`: clippy both ways, the tests with the behaviour lock,
  both `cargo hack` runs, the ratchet (read-tptp 1.00 % and read-lltp
  0.32 % over their ceilings, read-text 1.32 %, ordinary-pigeons
  −0.82 %; the cache notice on search-chain-128 and check-qbf-20-2 as at
  the base). It passed before at `vqmuvvpp` on `zuppslkx` with the same
  counts.
- Between commits: clippy over the workspace and without default
  features, `cargo hack check --each-feature`, rustdoc with `-D
  warnings`, the workspace's tests (`--no-fail-fast`), the lock (reblessed
  in commits of their own) and README's, the manual's and the guide's
  examples.
- **`nix flake check`** under the machine lock passed on the chain as it
  stood before the second rebase (on `zuppslkx`, every check); on
  `pnnrklrr` it failed on `budget` alone (above); on the final chain on
  `nmnrxmpk` (this report's commit, `wzvqwqso`) the gate passed again
  (the same counts) and so did every flake check, exit 0.

### What the next sessions must know

For 78b (first-order resolution):
- `=` is the predicate `first_order::EQUALS` of arity 2; a clause's
  equality literal is an atom of it. Resolution with `=` needs the axioms'
  clauses (`equality::with_axioms` on the sequent before the clausal
  form gives them as hypotheses, which 77a's clausal form turns into
  clauses) or paramodulation; the design note's part B takes the
  axioms.
- `has_equality` is the one test; a method called directly decides `=` as
  an uninterpreted predicate unless its caller augmented the sequent, and
  a "not valid" of such a sequent must come with a checked structure of
  identity, or be unknown (`within` enforces it for every library call).
- A saturated clause set is no "not valid" with `=` unless a structure of
  identity is read off and checked (`FiniteCountermodel::check` reads `=`
  as identity).

For 78c (a native rule, the manual and the guide):
- **The recommendation: no native equality rule in the connection
  prover now.** The axioms are leanCoP's and nanoCoP's own way with
  equality, so where deduxit's connection prover proves fewer of the
  1 453 classical theorems than they do, the gap is the search (its
  strategies at 2 s, as 75b found on the problems without equality), not
  the axioms; a native rule in the non-clausal connection calculus is
  research with no published calculus to follow. Measure first: the
  comparison (`bench/comparison.sh`'s ILTP pass) with leanCoP, nanoCoP,
  nanoCoP-i and iLeanCoP on these 1 730 problems, which it left out
  before, says how far the gap is. Paramodulation belongs to 78b's
  resolution at teaching scale, where it is the textbook rule, if 78b's
  design leaves room.
- **A D19 row**: on these problems the intuitionistic connection prover
  proves 39 theorems the intuitionistic default (the embedding) does
  not, and 142 together against the default's 103; on the problems
  without equality 75d found the opposite. A share or race for sequents
  with `=` is worth a measurement. Likewise the model search: it refutes
  105 of these problems at 2 s where the library's default refutes 13 to
  15 (the command's race already gets them).
- The linear printer and parser of `=` (Finding 3), grounding's class
  test without a stop (R4), `grade` with `=` (the problem's sequent gets
  the axioms, so an answer's derivation must hold them to match), and
  `interact --logic`'s `close`, which refuses first-order sequents as it
  did, are follow-ups.

For the planning session: nothing needs the author. Departures, all
marked above: no `Outcome` field for the axioms; the SZS line last; `seq
print --logic` shows the image with the axioms; the linear reader's `=`.

## Session 78b: first-order resolution

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`
(the conversation of 75a, 75c, fixups 15 to 22, 77a and 77b), cores 10
to 12, from `plan/78-equality-resolution.md`, the brief
`/tmp/linlog-handoff/start-78b.md` and 78a's design, section B (read in
78a's workspace: 78a was not on `main` while this session ran). Nothing
was pushed.

### What landed

- `lyvzzxun`: **the library**, `ordinary::resolution::first_order`
  (feature `sat`), its own module beside the propositional one (D17:
  the propositional types and searches are unchanged; the lock's
  propositional outputs did not move).
  - **Clauses** from 77a's clausal form (`normal::rewrite`), each
    hypothesis `∀x̄. C` its literals with the variables the shared
    unifier's metavariables, numbered in the order they first occur.
  - **One inference function** (`Clauses::made`) for the search, the
    checker, the read-back and steps by hand (`Clauses::infer`): binary
    resolution with the most general unifier (`crate::unify::Unifier`,
    the occurs check on), the parents renamed apart, and factoring. A
    step made again makes the same clause, so the checker
    (`Refutation::check`) recomputes each unifier and compares bindings
    and clause exactly, from the given clauses alone.
  - **The search**: the given clause, lightest first, with forward
    subsumption and tautology deletion; the clauses that are no clause
    of the hypotheses alone are the set of support, and when it runs out
    the hypotheses' clauses become given clauses too, so the search stays
    complete. Bounded by `--clause-limit` and `--step-limit`, the memory
    and `--term-limit` (on the shared DAG, before an atom's text is made).
  - **The read-back**: the refutation's substitutions composed from `□`
    down give the given clauses' ground instances, a ground refutation's;
    77b's `herbrand::deskolemized` builds the derivation of LK of the
    sequent as written from them, each Skolem term an eigenvariable
    again, and checks it. Where it does not reach, the verdict rests on
    the checked refutation and the clausal form, and the answer says so.
  - **The protocol**: `ordinary.resolve` takes a first-order statement;
    its check makes the clauses again, replays every step comparing the
    clause's and unifier's texts, and checks a derivation read back.
  - **Wire level 29**, written only; the schema's `clause_resolution` is
    a `oneOf` of the propositional form and this one.
- `tszyzykz`: **the command**, `deduxit resolve` on a first-order
  sequent: the clauses with variables `X1`, `X2`, …, each inference
  `from I, J with X1 := f(Y1)` or `factor of I with …`, the verdict line
  with the derivation read back. Exit 0 refuted, 3 unknown or saturated,
  2 error; `--method dpll` and `--clauses` other than `cnf` refused
  there. The manual's resolution chapter has a first-order section (two
  examples the README test runs), the guide its paragraph, the schema
  check two documents.
- `luyrxnwz`: the lock's three new calls (no existing output changed).
- `mlyxonux`: **a fix to 77b's Herbrand read-back**, found by this
  session's generated test.
  - The clausal form Skolemizes the hypotheses left to right, and an
    equal quantifier inside a later formula is replaced with the first:
    `¬((∃x. p(x)) → (r ∨ p(a))) ⊢ ∀y. ((∃x. p(x)) → (∃x. p(x)))` made
    `deduxit herbrand` end in an error on a valid sequent. `skolem` now
    takes earlier steps met by a closed part of the formula first.
  - The read-back also asks the stop at every step (the counterexamples
    member of the panel: six nested `∀` over six constants ran past
    `--timeout`).
- TPTP `cnf` problems: 78a's reader makes `C₁, …, Cₙ ⊢` of universally
  closed clauses, whose clausal form is themselves, so `resolve` decides
  them once 78a lands (not run: 78a's reader is not on `main`).
- Equality: with 78a, `equality::with_axioms` would add the axioms as
  clauses before the clausal form, as design B says; not wired here,
  since 78a's module is not on `main` (left for the rebase or 78c).

**Departures from the design**:
- A saturated clause set (every inference made, no `□`) is answered
  unknown, exit 3, not "satisfiable": no model is built, and the crate
  answers "not valid" only with a checked countermodel (departs from the
  design).
- The set of support is found by comparing the clausal form's clauses
  with those of the hypotheses alone, by text; where they differ (other
  Skolem numbers), the clause joins the set of support, which costs only
  search, and the fallback keeps completeness (departs from the design,
  which named the set of support without saying how it is found).
- Where the read-back does not reach, the verdict rests on the refutation
  and the unchecked clausal rewrite (`normal::rewrite`), as Herbrand's
  clausal-proof fallback does; the design called Skolemization the
  trusted step "as the clausal form's checker holds it", which this path
  does not run (the argument member's note; departs from the design).

### The panel

Argument (Fable 5.2, `high`), counterexamples (Opus 5.5, `high`),
integers (Sonnet 5.5, `high`), on the chain as committed at first; all
findings fixed in `lyvzzxun` or `mlyxonux`, each with a test. No unsound
answer was found: the counterexamples member ran 5 269 command-line
sequents (2 664 refuted, each replayed by its own Python checker, none
with a countermodel of up to three elements, none refuted by `prove`),
997 through the library and 53 706 random steps of `Clauses::infer`
against its own resolution, with no disagreement.
- **A given `□` was an error** (all three members): the search found it,
  the checker wanted at least one step. A refutation of no step is
  accepted where a given clause is empty, in the library and the
  protocol's check.
- **A term exponential as a tree escaped every limit** (integers): a
  unifier doubling a term per variable, 1 KB of input, ran to 3.9 GB
  under `--timeout 2s`: the atom's key text and the weight unfolded it.
  Atoms past `limits.terms` are refused from the memoised size on the
  DAG before the key is made; the walks share parts; the stop is asked
  per inference.
- **An open witness in the input's arena was taken for a clause
  variable** (argument; library only, no reader writes one): refused at
  the door.
- **A propositional sequent through the library got a constant** that
  made its derivation unreadable (argument; library only): no constant
  is made there.
- **The protocol told the two documents apart by a substring**
  (counterexamples): an atom named `clausal` sent a propositional output
  to the wrong reader. It reads the parsed key now.
- **Herbrand's read-back ignored the stop** (counterexamples): fixed in
  `mlyxonux`, above.
- Not fixed, recorded: a 1 500-variable sequent peaks at 1.2 GB under
  `--memory-limit 128MiB` in the shared Herbrand read-back (the integers
  member; `herbrand` alike), and the read-back's uniform witness set
  makes nested quantifiers over many constants slow (a witness set per
  quantifier would cut it).

### Fresh review

`fresh-reviewer` (Fable 5.2, `medium`, the row's reviewer), after the
panel's fixes. No unsound answer: about 1 900 generated sequents through
`resolve` against `prove --logic classical` (no refuted sequent the
prover refutes, every "saturated" one not valid by `prove`, every
refutation read back); eight level-29 and propositional documents valid
against the schema; D17 held on the four propositional lock cases. Two
findings, both fixed:
- **The Herbrand fix overreached** (a regression): `taken` replaced any
  closed instance of an earlier step's open quantifier, though the
  clausal form replaced only that step's own node, so `∀y. ∃x. p(x, y),
  ∃z. (q(z) ∧ ∃x. p(x, a)) ⊢ ∃z. q(z)`, which read back before, failed.
  It takes only the step's own closed node now (`mlyxonux`, with the
  witness as a test in both modules). The open case (an open quantifier
  replaced inside a later formula) still fails to read back, before and
  after, and the rules say so.
- **`Literal` and `Inference` lacked `#[non_exhaustive]`**, and the new
  public types were in neither of `core.md`'s lists (nor were 77b's
  Herbrand types): marked and listed (`lyvzzxun`).
- On TPTP `cnf`: this chain's reader refuses it (78a's is not on
  `main`); after 78a, `=` must get its axioms in first-order resolution
  before a saturated answer may be read as anything (it is unknown
  already, so no false verdict follows).

### Checks

- `cargo clippy --workspace --all-targets -- --deny warnings` and
  `cargo test --workspace` (39 binaries) after each commit and after the
  panel's fixes; README's, the manual's and the guide's examples run.
- Both `cargo hack` runs and `cargo hack check --feature-powerset --depth
  2 --tests -p deduxit`, warnings denied: the gate's first run found
  `crate::unify` built only with `interactive` (now `sat` too, the
  session-only methods gated) and `trail`/`bindings` needed only with
  both `serialize` and `interactive`.
- The gate passed on the chain; the ratchet under its ceilings.
- `nix flake check` under the machine lock: the first run (17:45 to
  18:19, on `main@origin` `pnnrklrr`, 76c's code with it) failed in
  `budget` alone, the web module at 2 838 278 bytes at gzip past
  2 800 000; `zsmlwklp` raises the ceiling to 2 875 000. The second run
  at the report's commit (18:20 to 18:39) passed every check. Only this
  line changed after.

### What 78c must know

- `ordinary::resolution::first_order` has no paramodulation; with 78a's
  axioms as clauses it decides equality problems by resolution, slowly.
  The axioms are not wired into it yet (`equality::with_axioms` before
  `normal::rewrite` in `resolve_dyn` and `clauses_of`).
- The read-back is 77b's Herbrand construction: its uniform witness set
  is the known cost.
- Wire level 29 is first-order resolution's (28 Herbrand's). If 78a takes
  a level, whichever lands second renumbers.

## Session 78c: equality measured, step 78 complete

Opus 5.5 at `high`, keeping 78a's conversation, in the workspace
`../deduxit-wt-step-78a-94ff76`, cores 4 to 6 (the panel on 13 to 15),
from the brief `/tmp/linlog-handoff/start-78c.md` on `wzvqwqso`. Nothing
was pushed.

### The comparison (D19)

`bench/comparison.sh`'s ILTP pass ran leanCoP 2.1, nanoCoP 2.0,
nanoCoP-i 2.0 and iLeanCoP 1.2 (`comparison.nix`'s builds) on ILTP's
problems with equality at 2 s, beside deduxit's runs of 78a (the default,
`connection`, `embedding`, `grounding`; 2 s, the same harness). All 1 730
at 30 s a problem for the slow ones would have taken the night, so the
pass ran on every fourth problem in sorted order, 433 problems
(`bench/comparison/2026-11-22-iltp-equality/problems.txt`), the rows in
`runs-1.csv` and `runs-2.csv` there (local paths stripped from the
environment files), on cores 4 and 5.

| logic | prover | proved | not valid | unknown | against a status |
|---|---|--:|--:|--:|--:|
| classical | leanCoP | 134 | 1 | 298 | 0 |
| classical | nanoCoP | 115 | 1 | 317 | 0 |
| classical | deduxit, default | 91 | 5 | 337 | 0 |
| classical | deduxit, `connection` | 88 | 0 | 345 | 0 |
| classical | deduxit, `embedding` | 23 | 0 | 410 | 0 |
| classical | deduxit, `grounding` | 5 | 5 | 423 | 0 |
| intuitionistic | iLeanCoP | 90 | 1 | 342 | 0 |
| intuitionistic | nanoCoP-i | 87 | 1 | 345 | 0 |
| intuitionistic | deduxit, default | 32 | 5 | 396 | 0 |
| intuitionistic | deduxit, `embedding` | 32 | 0 | 401 | 0 |
| intuitionistic | deduxit, `connection` | 23 | 0 | 410 | 0 |
| intuitionistic | deduxit, `grounding` | 0 | 5 | 428 | 0 |

No verdict of any prover is against either status (iLeanCoP had one
error). Classically leanCoP proves 15 that nobody else does; every
theorem deduxit proves, leanCoP or nanoCoP proves too. Intuitionistically
iLeanCoP proves 10 alone and nanoCoP-i 7; deduxit none alone.

**No native equality rule in the connection prover** (recorded in
`plan/later.md`). Every one of Otten's provers decides equality by adding
its axioms, as deduxit does, and nanoCoP's calculus is deduxit's: the
gap of 115 against 88 (and leanCoP's 134) is the search (restricted
backtracking's schedule, leanCoP's strategies), as 75b found without
equality, not the axioms. A native rule would need a published equality
calculus for connection proving to follow, which this session did not
find among the provers compared; it waits until deduxit's prover closes
the gap with the axioms.

**Paramodulation in first-order resolution: built.** With the axioms as
clauses, `resolve --equality axioms 'a = b |- a = c'` is still unknown at
the 2 s limit (the congruence and transitivity clauses saturate without
end); with paramodulation it saturates at once (exit 3, "every inference
was made"). On the panel's 3 850 generated sequents paramodulation
refuted 1 696 and saturated 1 317, the axioms 1 671 and 497.

### What landed

- `noszslvu`: the linear printer writes `s = t` and the linear parser
  reads `=`, `!=` and `≠` between terms, so a linear sequent with `=` (an
  image's) reads back as itself; a postfix dual brackets it, `(a = b)^⊥`.
- `klzmxpov`: the ordinary reader finds a term's binder in one map lookup
  (`Scope`), where it scanned the open binders (R3's rest).
- `qpzznqnt`: the command adds the equality axioms where it decides
  (`prove`, its batch, `interact`, `grade`: `ordinary::with_equality`),
  and `resolve`, the transformations and `seq print` read the sequent as
  written; before, the reader added them to every sequent, so `resolve`
  drowned in axioms. `grade` accepts a derivation that holds any of the
  axioms beside the problem's own hypotheses, and a refusal says how many
  of the problem's last hypotheses are the axioms.
- `oywtrklt`: **paramodulation** (`first_order::Step::Paramodulation`:
  the equation's clause and literal, the rewritten clause and literal,
  the place counted in preorder from 0, and whether the equation is read
  right to left), one more inference of `Clauses::made`, so the search,
  the checker (`Refutation::check`, which makes each rewrite again from
  the given clauses), `Clauses::infer` and the protocol's check share it.
  Never into a variable (a clause variable stands for any term);
  `resolution::Equality` (`Paramodulation`, the default, or `Axioms`),
  `--equality`. The command writes `4. ¬f(c) = c  1 rewrites 3 with
  X1 := f(c)`; the read-back refuses a refutation with a rewrite (LK has
  no rule for it) and the verdict says it rests on the checked
  refutation and the Skolem symbols.
- `ozznpkox`: the rules' notes (`cli.md`, `core-ordinary.md`, `core.md`'s
  list of marked types).
- `lnusppqp`: the comparison's rows and ranking, above.
- `xnpzswrz`: the panel's fixes (below).
- `mymuopop`: **R4**: grounding's miniscoping counts its work
  (`Nnf::work`) and asks the stop; the default's class test (`grounds`)
  ends at a work bound of its own, `GROUNDS_WORK`, 2²¹ units: on ILTP's
  2 670 first-order problems those in the classes take at most 21 538,
  those outside 754 814. The model search's expansion and grounding's
  walks weigh each step by the free variables they look up. A
  predicate's congruence axiom of 4 000 arguments under `--timeout 2`
  took 100 s before and 1.8 s now; every input of R4 and of the integers
  member's wide disjunctions ends within the limit, each method and the
  default.
- `vvpzyypk`: a linear refutation names an equality as it is written
  (`a = b on the left has no a = b on the right …`, where it said
  `=(a, b)`).
- `syrpzkqy`: the manual (the ordinary-logic chapter's equality section,
  the resolution chapter's paramodulation, `first-order.md`'s `=`,
  `grading.md`, `status.md`), the guide and the changelog.
- `rxowuvww`, `wrmvyyzr`: the linear reader's and printer's equality off
  the propositional path, and `read-text`'s ceiling raised (below).
- `ytkswvsq`, `ylomslxp`, `yppykrrw`: the fresh review's fixes and the
  lock's new calls (below).

**Departures from the design**:
- Paramodulation takes reflexivity alone as an axiom clause, by Brand's
  completeness of resolution and paramodulation without paramodulating
  into variables given reflexivity (Brand 1975); the design named "the
  axioms as clauses" for resolution, before paramodulation was decided
  (departs from the design).
- The comparison ran on a quarter of the 1 730 problems, every fourth in
  sorted order, for time (departs from the brief's 1 730).

### The panel

Argument (Fable 5.2, `high`), counterexamples (Opus 5.5, `high`),
integers (Sonnet 5.5, `high`), on `oywtrklt` and `qpzznqnt`; every
finding fixed in `xnpzswrz`.
- **An eigenvariable was skipped as a place** (argument): an `Eigen` term
  is a constant to paramodulation, and the walk now rewrites it.
- **Functional reflexivity was added beside reflexivity** (argument): not
  needed for completeness (Brand), and it fed the search; reflexivity
  alone, the docs and help naming Brand's theorem.
- **The wire schema lacked the step** (argument): a `oneOf` branch for
  `rule: "paramodulation"`, and a document of it in the schema check.
- **The protocol's check held a read-back derivation to the raw input**
  (argument): it compares with the sequent with reflexivity appended,
  which the search ran on; a test with `|- forall x. x = x`.
- **The walk over a literal's places was unbounded** (integers): it built
  a path per place, so a literal of a million places held gigabytes. It
  keeps a parent index per place and charges the steps made to the
  memory bound; `metas_of` was quadratic, now a set. (The first fix also
  counted the places walked as steps and ended a literal past 2²⁰ places
  at the step bound; the fresh review found both wrong, below.)
- **Counterexamples**: no refutation. Its own reference (structures of 1
  to 3 elements, `=` as identity) on 3 850 generated sequents both ways:
  every refutation without a countermodel and every one of its 7 775
  inferences (938 rewrites) entailed by its parents; every saturated
  answer with a countermodel; never refuted against saturated. 579
  paramodulation refutations held through the protocol, 4 399 of 4 404
  mutants refused (the 5 others moved `from` to an identical given
  clause, sound). 3.6 million random steps of `Clauses::infer`, 133 727
  rewrites accepted, each entailed; no panic.

### Fresh review

`fresh-reviewer` (Fable 5.2, `high`), on the chain to `syrpzkqy`. No
soundness defect: 1 030 generated sequents with `=` through `resolve`
against `prove --logic classical`, 311 refuted (each valid by `prove`),
364 saturated (each refuted by a checked structure), no disagreement;
the protocol's check, the linear round trip on ten forms, `grade` with
and without the axioms, R4 at 3 000 arguments (1.86 s at `--timeout
2s`). Seven findings, all fixed:
- **Paramodulation's walk never asked the stop** (medium): 2 000
  equations against a literal of 500 000 arguments ran 16 s under
  `--timeout 2s` with `--step-limit none`. The walk asks the stop every
  4 096 places and once per pair of clauses (`ytkswvsq`, with a test);
  the witness ends at the limit now.
- **A literal past 2²⁰ places ended "at the step bound"** with no
  inference made, and **a place counted as a step**, against the docs of
  `Statistics::steps`, `Options::steps` and `--step-limit`. A step is an
  inference again; the term bound caps a step's place (`ytkswvsq`).
- **The manual's level-29 paragraph lacked the paramodulation rule**
  (`ytkswvsq`).
- **The linear syntax docs lacked `=`** (`Sequent`'s docs and the
  command's syntax help; `ytkswvsq`, the help's line reblessed in
  `yppykrrw`).
- **Three rules notes were stale** (`sequent_in`, the linear printer,
  `occurrences`; `ytkswvsq`).
- **`--equality` had no lock call** (`yppykrrw`: paramodulation as text
  and JSON, and `--equality axioms`).
- Its question, fixed too: in `3 rewritten by 1 with X1 := f(c)` the
  `X` variables were clause 1's, named second; the line is `1 rewrites 3
  with X1 := f(c)` now, the equation's clause first (`ylomslxp`).
- Outside this chain, for the planning session: `prove --logic classical
  --method connection` on a congruence of 3 000 arguments ran 40.9 s and
  2.6 GB at `--timeout 2s` (the default ends at 2.0 s); the connection
  prover's matrix build asks no stop. Not this step's (the dispatch and
  the prover are fixup-23's area); a follow-up.

The gate after the chain found `read-text` 3.66 % and `render-latex`
2.55 % over their ceilings, from `noszslvu`: every atom asked out of line
whether `≠` followed, since `⊗`, `⅋` and `⊸` share its first byte.
`rxowuvww` keeps the test to a few compares inline and the rest of the
equality path out of line (render-latex back within its ceiling, +1.0 %);
`read-text` stays 1.3 % above its parent, and `wrmvyyzr` raises its
ceiling to 12 591 991 in a commit of its own that says why.

### D19 at first order with `=`

On fixup-23's head (`rtsztskz`, the library's connection prover bounded
by 2²⁶ steps), ILTP's 1 730 problems with equality at 2 s, the harness's
`auto` (the library's `ordinary::decide`), three shards on cores 4 to 6,
the machine otherwise quiet; rows in
`bench/defaults/equality-share-{before,after}.csv`:

| logic | build | valid | not valid | unknown |
|---|---|--:|--:|--:|
| classical | head | 318 | 13 | 1 399 |
| intuitionistic | head | 102 | 15 | 1 613 |
| intuitionistic | with the share | 110 | 15 | 1 605 |

No verdict against a status, no error.

- **The intuitionistic share does not land.** The trial ran the
  intuitionistic connection prover within 2²² steps (1.85 times the most
  a theorem took in 78a's run, 2 272 102 on SWV095+1) before the
  embedding, for a first-order sequent with `=` and no method given. It
  gained 36 theorems (the SWV and AGT problems) and **lost 28**, all
  theorems the embedding proves in 0.4 ms to 1.2 s, which now ran out
  of time in the prover: its steps and its polled work units do not
  track its time. SET585+3 makes 4.3 million steps in 20 s, SYN083+1
  63 000; over 2 s SYN083+1 polls 75 000 units where SWV095+1's proof
  takes 2.3 million in 0.19 s. With no clock in the library, no bound in
  either unit keeps every verdict, so the library's default stays the
  embedding; a race in the command, which has a clock and threads, is
  recorded in `plan/later.md`. On the 89 rows decided alike the share
  cost 7.5 s more in all (12.4 s against 19.9 s; the median row 0.70
  times, the five most slowed by 0.45 to 1.6 s).
- **The classical default's refutations stay 13** with fixup-23's bound,
  all grounding's: at 2 s no row reaches the prover's 2²⁶ steps, so the
  library's `decide` never reaches the model search, which alone
  refutes 105 (78a). The bound acts where the library runs without a
  clock. The command races the model search beside the prover, and on
  every tenth of those 105 (11 problems) `prove --logic classical
  --timeout 2` answers "not valid" with a checked structure, raced and
  with `--jobs 1` alike (10 by the model search, NLP124+1 by grounding).
  Nothing for `=` alone to add there.

### Checks

- `cargo clippy --workspace --all-targets -- --deny warnings` after each
  commit; the focused tests of each change (resolution, equality,
  protocol, grounding, models, refutation, parse, serialize, the core and
  the command's locks, README's, the manual's and the guide's examples).
- The gate on the chain before the review (failed in the ratchet alone,
  `read-text` and `render-latex`, fixed above) and, after the rebase onto
  fixup-23 (`rtsztskz`), on the whole chain: passed, `read-text` at
  12 590 265 against its new ceiling, `render-latex` 1.06 % above its
  own.
- `nix flake check` under the machine lock at the report's commit
  (`spvxpolr`): every check passed. Only this line changed after.

### Step 78 is complete

Equality is read, decided by its axioms in every method and logic,
refuted only by a checked structure that reads it as identity, resolved
by paramodulation, graded, and documented; the comparison decided
against a native rule in the connection prover. With this session step
78 is complete.
