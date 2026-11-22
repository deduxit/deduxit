# Step 78: equality and first-order resolution

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
