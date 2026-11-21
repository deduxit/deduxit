# Step 75: proof search at first order

## Session 75a: the design and the embedding's search

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`,
on fixup-13 (`pvnmzrvm`), cores 10 to 12. Nothing was pushed.

### Outcome

- **The design** is `plan/notes/first-order-search-design.md`: the
  embedding's quantifier rows and their read-back (§2), the classical
  connection prover (nanoCoP's non-clausal calculus, §3), the
  intuitionistic one (nanoCoP-i's prefixes, §4), the decidable
  fragments (§5), the bounds and "unknown" (§6), the dispatch (§7) and
  the comparison with leanCoP, nanoCoP and iLeanCoP (§8). Each choice
  names its sources and the alternatives weighed.
- **The embedding decides first-order sequents**: the quantifier rows of
  every translation, the image's first-order tables, the read-back of
  the focused engine's first-order proofs to LK, LJ and LM (checked),
  and `decide` and `prove --logic` answering through it: "valid" with a
  checked derivation, "unknown" (`Reason::Incomplete`) where the image
  is unprovable, never "not valid".
- The connection provers, the fragments and the comparison are 75b to
  75d's ("What the next sessions must know").

### Decisions and sources

The design note holds them with their sources; in brief:
- `∀` takes `∧`'s row and `∃` `∨`'s, with one operand: Girard 1987 §5
  (cbn: `(∃x. A)* = ∃x. !A*`), cbv's `∀` under `!` as every negative
  formula of that translation; the 0/1 rows by analogy, since LLTP's
  translation (`tptpparser/translation.ml`) is propositional; the affine
  row puts `∀` on the right and `∃` on the left, as `∧` and `∨`.
- An unprovable image is "unknown" at first order: the affine image has
  no contraction, which G3c's `∃R` and `∀L` need (the drinker's
  sequent), and the intuitionistic rows are not shown faithful here.
  "Not valid" at first order comes from the fragments (75d) and step
  76's finite models.
- The native provers are nanoCoP (classical) and nanoCoP-i
  (intuitionistic), written anew from the papers (their code is GPL and
  is never copied), each proof read back to G3 by Kreitz and Schmitt's
  conversion and checked inference by inference.

### The code

- `core/src/ordinary/translate.rs`: `Core::Forall` and `Core::Exists`,
  the six rows, the image's tables copied from the ordinary arena's
  (`falsity_predicate` for minimal logic's `false`), binders named; the
  embedding's door at first order is gone.
- `core/src/ordinary/derivation.rs`: the read-back's members past the
  forest (`Instances`: the proof's own terms interned into a copy of
  the image's arena, each member's formula instantiated by its frame,
  innermost first, memoised), the quantifier rules (`rule_of`'s four
  rows, the term from `opened`), `Extra::add` after instances were
  built.
- `core/src/ordinary/decide.rs`, `mod.rs`: `Options::method_for`, the
  embedding at first order by default; `Image::unprovable`, unknown at
  first order; no countermodel there. `Formulas::is_first_order` is
  public, for the command.
- `core/src/linear/search/mod.rs`, `serialize/search.rs`,
  `serialize/ordinary.rs`: `Reason::Incomplete` (`incomplete`, no
  setting), the outcome at level 25 where its derivation is first-order
  or its reason this one, the reader taking that unknown over an
  unprovable image.
- `cli/`: `prove` and the batch ask `method_for`; the verdict line and
  the exit status follow an incomplete image (`ordinary::incomplete`,
  `ordinary::status`).
- `core/src/ordinary/interactive.rs` (74e's file): three lines, a door
  in `Interactive::decide`, since `close` and `hint` relied on the
  embedding's refusal and the graft has no arm for a binder.
  (departs from the brief, which keeps this session out of 74e's code:
  without it `close` on a first-order goal reached an `unreachable!`)
- Docs: the manual's "First-order sequents", `status.md`, the guide's
  reference, the changelog, `core-ordinary.md`.

### Measurements

`deduxit-bench run --tptp bench/iltp-first-order/Problems --logics
intuitionistic,classical --engines auto --timeout 2`, three shards on
cores 10 to 12, each alone on the machine's cores it was given; before
is fixup-13's last run (the base of this chain), after is `rzyvstzu`:

| rows | before | after |
|---|--:|--:|
| refused, equality | 3 460 | 3 460 |
| refused, first_order | 1 732 | 0 |
| proved, classical | 69 | 357 |
| proved, intuitionistic | 35 | 226 |
| unprovable (propositional problems) | 34 | 34 |
| unknown, `incomplete` (classical / intuitionistic) | 0 | 286 / 34 |
| unknown, copy bound (intuitionistic) | 0 | 614 |
| unknown, time limit (classical / intuitionistic) | 0 / 2 | 215 / 28 |
| unknown, memory limit (classical) | 0 | 53 |
| unknown, derivation limit (classical / intuitionistic) | 0 | 23 / 1 |
| unknown, instances (classical) | 0 | 1 |
| error, a variable no quantifier binds | 8 | 8 |

- No verdict contradicts a file's status (950 rows say `provable`, 94
  `unprovable`), and `summary` finds no problem with two verdicts.
- The 34 unprovable rows are propositional problems of the first-order
  directory, refuted by the SAT solver as before.
- One row changed that is no first-order row: SYJ202+1.010 in classical
  logic, proved at 2 101 ms before (past the 2 s limit, the classical
  default's evidence included) and stopped at 2 010 ms after. It sat on
  the limit both times.
- The copy-bound rows are the harness's: it passes the library's
  default bound of 3, which the intuitionistic images reach (the
  command has none).
- D17: the gate's ratchet holds the propositional journeys' instruction
  counts (below); the read-back's propositional path adds one
  comparison per member (`Instances::node`), the translation one test
  per atom.

### Panel and review

The panel on the read-back and `decide` (argument on Fable 5.2,
counterexamples on Opus 5.5, integers on Sonnet 5.5, all at `high`),
each from the brief in the scratchpad (`panel75a-brief.md`):
- **Argument**: one break. `decide` of a sequent made over a
  derivation's arena (`Sequent::new(d.formulas().clone(), …)`, all
  public) panicked in the focused engine (`framed.rs`, "a sequent's
  arena holds no eigenvariable"): the image copied the arena's
  eigenvariable terms. Fixed: the image reads each eigenvariable or open
  witness as a constant of a fresh symbol at the same index (`constants`)
  and names its atoms by key again, and the read-back numbers the
  proof's eigenvariables past the arena's; a test decides every
  inference's sequent of a derivation's arena in every logic. The rest
  held: members instantiated innermost first, `opened` exact (the
  premise is the conclusion less the principal plus one body member,
  read from `build.rs`), the term ids shared, `Extra::add` needed and
  enough, the eigenvariable condition kept for every proof the linear
  checker takes, `rule_of`'s sides; 420 command runs and 124 exports
  clean. Its note: on the command's path a refused read-back leaves
  "valid" as `Evidence::Image`, as for propositional sequents, resting on
  the linear checker and the rows' soundness.
- **Counterexamples**: none. 4 690 generated sequents through every
  logic and translation (11 680 read-backs, 13 662 "valid" checked again,
  each against a second reference of its own: finite models of size 1 to
  3 and Kripke models on four frames), about 2 950 command runs and 200
  batch lines per logic: no wrong "valid", no read-back refused, no
  panic, no "not valid" at first order, exit statuses right, level 25.
  Five classical read-backs were refused past the derivation limit,
  which the contract allows.
- **Integers**: one break. The read-back instantiated a frame term by
  term, cubic in its length: `∀x₀…xₙ. p(x₀,…,xₙ) ⊢ p(c₀,…,cₙ)` (10 KB
  at n = 700) took 8 GiB under every default bound, and overshot a 2 s
  limit by 17 s at n = 500. Fixed: `Formulas::instantiate_frame` puts a
  whole frame in at once (one substitution of the frame's terms, a
  variable past it moving in by the frame's length): n = 700 now takes
  0.4 s and 84 MiB, n = 300 24 MiB (1 463 before). The casts, the tables'
  growth, `Instances`' indexing, `opened`, `push`'s ids and deep nesting
  to 60 000 held.
- The ratchet (the gate) caught `ordinary-pigeons` 13.6 % over its
  ceiling: the per-member loop collected a `Result` with no size hint and
  called a function per member. The propositional loop is back (the
  proof has no framing): 31 774 054 instructions against the base's
  31 750 475 (+0.07 %); the base itself is 1.55 % over the ceiling,
  within the tolerance.

The fresh-context review (`fresh-reviewer` on Fable 5.2, `high`; it ran
42 sequents under every translation, 5 040 `decide_within` calls under
small limits and early stops, the harness on a TPTP file): no wrong
"valid", no "not valid" at first order, no panic; six findings:
1. **An open witness as a quantifier's term**: a sequent over a session's
   arena with `$1` gave a read-back the checker refuses (`an open
   witness`), surfacing as a defect. Fixed: the read-back answers
   `OPEN_WITNESS` there and `decide` and `Image::outcome` answer unknown,
   `incomplete` (in `within`, and `Image::checked` for `outcome`);
   tested in every logic. Through a helper in `within` the check was
   instantiated anew and the ratchet's `ordinary-pigeons` went to +2.09 %
   (the flake check's first run); inlined as before, 31 797 472 (+0.15 %
   against the base).
2. **The command's "valid" without a checked derivation** at first order
   under `-q`, `--tree never` or a refused read-back (`Evidence::Image`).
   Kept, as for propositional sequents: the answer rests on the linear
   checker and the rows (erasing the linear structure gives the LK/LJ
   proof, which is what the read-back builds and the counterexamples
   member checked 11 680 times); the library's `decide` stays strict.
   (departs from the prompt's "valid by the derivation's checker" on the
   command's quiet path, as the propositional path already did)
3. **The harness's `embedding` engine wrote `unprovable`** for a valid
   first-order problem (`SYN999+1`, a classical theorem): fixed,
   `Searched::incomplete` writes `unknown`, `incomplete`, as `decide`
   does; the witness now reads so. `bench.md` says it.
4. **Four texts false**: the goal state's door named the embedding (now
   "a session's close and hint"), `Error::FirstOrder`'s doc, the outcome's
   JSON doc (level 25 added) and `Image::outcome`'s doc: fixed.
5. **`--method`'s help and `cli.md`**: fixed; the lock's `help-prove.txt`
   reblessed in a commit of its own (`lskvpqul`).
6. **The protocol's `ordinary.embed` promise `same_verdict`** would fail
   once a native prover proves a sequent whose image is unprovable: its
   check now reads an unprovable image of a first-order sequent as
   unknown.

### Checks

- `cargo clippy --workspace --all-targets -- --deny warnings`, clean.
- `cargo test --workspace`: passed on the final tree (after the rebase
  onto 74e and every fix), README's, the manual's and the guide's
  examples and the behaviour lock among them.
- The gate passed on `rzyvstzu` before the panel's and the review's fixes;
  `cargo hack check --each-feature -p deduxit` on the final tree (the
  dead-code `expect` of `instantiate`).
- The ratchet's `ordinary-pigeons` pinned against the base's own count
  (above).
- The ILTP pass before and after (above), and the integers member's
  witnesses run again on the fix (n = 300, 500, 700 intuitionistic, 600
  classical: valid, at most 84 MiB and 0.4 s).
- `nix flake check` under the machine lock: its first runs failed the
  private docs (an ambiguous link to `decide`, fixed), the ratchet
  (above) and the budget (the bindings' module at 2 500 135 bytes at
  gzip, 135 over the 2 500 000 of then). fixup-14 raised that ceiling to
  2 560 000 for its own growth meanwhile; rebased onto it, the module
  passes, so this chain raises nothing. On the final head, rebased onto
  fixup-14 (`wryvwwxt`): `cargo test --workspace` passed and `nix flake
  check` passed every check.

### What the next sessions must know

- **Where a native prover plugs in**: a `Method` variant (`connection`),
  its arm in `decide.rs`'s `within`, and `Options::method_for`'s
  first-order default (now the embedding) moved to it where 75d's
  measurement puts it ahead (design §7); the command asks `method_for`
  before `by_native` (`cli/src/prove.rs`, `cli/src/batch.rs`), so a
  native method there goes through `decide_natively`, which needs its
  arm. The embedding stays the fallback where the native one gives way
  at a bound (`gives_way`).
- **Its read-back** builds instances the way this one does: closed
  terms interned into the derivation's arena (`Formulas::interner`,
  visible inside `ordinary`), `Formulas::instantiate` innermost first,
  then `Extra::add` once (an instance of `↔` needs its implications).
  Every inference goes through the checker; `Inference::term` carries
  the witness or eigenvariable.
- **`Reason::Incomplete`** is general: the connection prover with
  restricted backtracking (nanoCoP's default) ends incomplete too, and
  answers the same reason.
- **The ground case is wasted**: a first-order sequent without binders
  (`p(a) ⊢ p(b)`) is refuted by the image's classical assignment, which
  is a Herbrand model and so a sound "not valid"; this session keeps the
  uniform "unknown" (design §7). Taking it is a few lines in
  `Image::unprovable` with a countermodel through `Countermodel::found`
  (whose check refuses first-order sequents today, so the check needs
  ground atoms first).
- **The goal state's `close` and `hint` refuse first-order goals** by a
  door in `Interactive::decide` (74e's file), because `graft`'s `ids_of`
  has no arm for a binder; grafting a first-order derivation is the
  session's own follow-up.
- **The intuitionistic images hit the library's copy bound of 3** on
  simple sequents (`∀x. (p(x) ↔ q(x)), q(a) ⊢ p(a)` under 01): `decide`
  with the default search options answers unknown there, the command
  (no copy bound) proves it.
