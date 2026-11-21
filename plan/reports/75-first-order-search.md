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

## Session 75b: the classical connection prover

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58a-610ed3`,
cores 7 to 9 (the panel on 13 to 15), on 75a's chain (`rzyvstzu`), on
`main@origin` once 75a landed. Nothing was pushed.

### Outcome

| change | subject |
|---|---|
| `yzyosluw` | Decide classical sequents by a non-clausal connection prover, read back to LK |
| `qqnvuyxs` | Decide classical sequents by the connection prover in prove and the harness: --method connection, --path-length, --restricted-backtracking |
| `xupqprul` | Rebless the behaviour lock: the connection prover's calls, and its flags in the help |
| `zwrynkrq` | Read connection proofs back where a witness's δ-formula lies on another branch, and weaken early |
| `runsrlso` | Document the connection prover: the manual, the guide, the changelog, the wire schema and the rules |
| `uqtosvxy` | Rebless the behaviour lock: the connection prover's derivations weaken what no longer helps |

- **The engine** (`core/src/ordinary/connection/`, public
  `ordinary::connection::decide`, `Method::Connection`, name
  `connection`): Otten's non-clausal connection calculus (TABLEAUX 2011,
  Fig. 6; nanoCoP, IJCAR 2016) written from the papers' text (75a's
  downloads), with positive start clauses (Lemma 4), regularity
  (Lemma 5), lemmata, the cut after a literal solved without binding an
  older variable, iterative deepening on the active path, Skolem terms
  for δ-formulas, and restricted backtracking as an option
  (`search::Options::restricted_backtracking`, off by default). Its
  bound is `search::Options::path_length` (library default 8, the
  command none) and its unknown `Reason::PathLength { length }`
  (`path_length`, setting `search.path_length`).
- **"Not valid"** only where the matrix has no slot (no γ-formula: no
  `∀` with polarity 1, no `∃` with polarity 0), the search exhausted
  with restricted backtracking off;
  there every copy is the clause and the search is the calculus's whole
  search on positions (Theorem 4 with Lemmas 4 and 5). With slots a
  failed search is `Reason::Incomplete` or `PathLength`.
- **The read-back to G3c** (`read_back.rs`): the substitution and the
  literals used (each in its copy) are the oracle; the derivation of
  the sequent as written is built branch by branch by a fixed order
  (close, weaken what cannot help, one-premise rules, `∀R`/`∃L` with
  the Skolem term's eigenvariable, `∀L`/`∃R` keeping the principal with
  each needed witness, two-premise rules preferring one that closes a
  premise, and last a witness whose δ-formula lies on another branch
  as a constant of this one), every inference then checked by
  `Derivation::check`. A branch nothing closes is `Error::ReadBack`, a
  defect, never a verdict.
- **The dispatch**: `Options::method_for` makes the connection prover
  the default of a first-order classical sequent (no method, no
  translation); where it gives way at a bound (`gives_way`), the
  embedding decides. Propositional sequents go as before (D17); the
  prover takes them only when named.
- **The command and the harness**: `--method connection`,
  `--path-length N|none`, `--restricted-backtracking`; `--stats` prints
  its five counters; `deduxit-bench run --engines connection`.

### Departures

- **Skolem terms take every γ-slot around their δ-formula**, not only
  those free in its body (departs from the design, §3: "Skolem terms
  over the free variables of their subformula"). Then each Skolem
  ground term names one δ-instance of the expanded formula, which the
  read-back gives one eigenvariable; with inner Skolemization two copies
  of a δ-formula on two branches share a Skolem constant that a sequent
  derivation cannot give one eigenvariable.
- **Copies are regions of the matrix with multiplicity**, not nanoCoP's
  replacement `M[C1\C2]` (departs from the design's "nanoCoP's
  calculus" in the letter): a copy is a sibling of its clause in the
  region of the latest copy on the path around it, taking that copy's
  variables for its non-free slots, and an extension clause is one every
  path through the active path enters in that region (Definition 8
  read on paths, `region_for`). Without slots this is the calculus on
  positions exactly; with slots it is sound, and complete only as far
  as the one region it picks per clause allows (hence `Incomplete`).
- **Lemmata are confined to the solved literal's clause** (departs from
  nanoCoP's sharing with the rest of the clause and its descendants): a
  first version shared them as nanoCoP does and "proved" the
  propositional non-theorem `b ∨ c, ((⊥ ∧ ⊥) ∧ ⊤) ∨ ((c → c) → ¬c) ⊢ (b
  ∧ (c ∧ a)) ∧ (a ∧ (c → c))`, which the read-back refused; a literal
  solved inside a decomposed sub-clause closed one outside it. The rule
  kept is the one a path argument justifies (`core-ordinary.md`).
- **The read-back does not follow Kreitz and Schmitt's reduction
  ordering** (departs from the design's "the order of the rules from
  Kreitz and Schmitt's reduction ordering, the β-split pruning
  theirs"): it expands every formula above a used literal in a fixed
  order and lets the checker judge. Four ILTP theorems (SYN069, SYN374,
  SYN375, SYN377) needed the last rule (a witness whose δ-formula lies
  on another branch, a constant there), and 59 more read back within
  the derivation bound once useless formulas were weakened early.
- **The command's default runs the prover to the time limit** (no path
  bound), so it gives way to the embedding only at another bound; the
  library's default (path 8) gives way there. Six ILTP problems the
  embedding proves at 2 s time out in the prover (below).

### Measurements

`deduxit-bench run --tptp bench/iltp-first-order/Problems --logics
classical --timeout 2`, three shards on cores 7, 8 and 9, the harness's
binary copied out of the tree; the first run with `--engines
embedding,connection` before the read-back's fixes (`zwrynkrq`), the
second with `--engines connection,auto` after them (`auto` is the
library's default call: the prover to its bound of 8, then the
embedding):

| rows (classical, 2 670 problems) | embedding | connection, first | connection, second | default, second |
|---|--:|--:|--:|--:|
| refused, equality | 1 730 | 1 730 | 1 730 | 1 730 |
| proved | 373 | 540 | **599** | 594 |
| unprovable (propositional problems) | 1 | 1 | 1 | 1 |
| unknown, `incomplete` | 289 | 30 | 30 | 55 |
| unknown, time limit | 160 | 284 | 284 | 286 |
| unknown, memory limit | 111 | 0 | 0 | 0 |
| unknown, derivation limit | 0 | 80 | 21 | 0 |
| other (parse errors, occurrences, instances) | 6 | 5 | 5 | 4 |

- A third run with `--engines connection,auto` after the panel's and
  the reviewer's answers (the chain's head): the prover 599, the default
  593, the same seven left to the embedding; the one row the matrix
  refused for its occurrences before is the memory bound's now
  (`unknown`, `memory_limit`). Over the embedding's first run: both 366,
  the prover alone 233, the embedding alone 7.
- No verdict contradicts a file's status, and no problem gets two
  verdicts. The 30 `incomplete` rows are exhausted searches with
  variables (the KRS and SWV sets, SYN320 and like): none is stated a
  theorem, and none is proved by the embedding.
- Of the problems either proves at 2 s: both 358 (first run), the
  prover alone 182 (241 in the second), the embedding alone 15, of which
  the read-back's fixes recovered eight (the derivation limit); the
  seven left (SYJ035 to SYJ037, SYN067, SYN349, COM003, SYJ018) time out
  in the prover. The default recovers all but one of them where the
  prover stops at its bound before the time limit; the command's default
  (no bound) does not.
- The generated set: the panel's counterexamples member ran 35 499
  generated sequents (8 499 mutants), propositional, ground first-order
  and with mixed quantifiers, against an exact reference and a
  countermodel search; and the module's test
  `first_order_sequents_agree_with_the_embedding` holds the prover to
  prove at least what the embedding proves on 300 generated sequents.
- D17: the default of a propositional sequent is unchanged; the gate's
  ratchet holds the journeys' counts (below).

### The panel

- **Counterexamples** (Opus 5.5): no wrong verdict in 35 499 generated
  sequents and 940 ILTP problems. One defect: a `⊤` right or `⊥` left
  under a γ-formula (`forall x. false |-`, `|- exists x. true`) proved by
  the search and refused by the read-back, which found no witness for the
  γ-formula and weakened it away. Answered: such a unit's γ-formulas get
  the stand-in witness (`oracle`); five of its witnesses are tests.
- **Argument** (Fable 5.2): claim 1 ("not valid") holds, every pruning
  checked against Theorem 4 and Lemmas 4 and 5; claim 2's "valid" is
  always checked, and the search's proofs are matrix proofs (regions and
  lemmata sound). One defect: the read-back's last rule took the first
  pending witness and made a Skolem eigenvariable a constant whose
  δ-formula a later witness brought in (`forall w. (p(w) -> n), (exists
  u. s(u)) \/ b, b -> forall y. (s(y) \/ exists z. p(z)), forall v.
  ~s(v) |- n`, valid, answered `Error::ReadBack`). Answered with its
  proposal: the least nested pending witness first (`nesting`), which it
  argued sound; its witness is a test. Also answered: the generated test
  skipped every error, so it now fails on any but a refusal.
- **Integers** (Sonnet 5.5): no wrap reached, no recursion, the
  read-back's bounds hold. Findings and answers:
  - the matrix's slot tables are quadratic in a nest of quantifiers and
    were built outside the memory bound and the stop (26 000 nested `∀`
    took 8 GB): the build now polls both (`matrix::Budget`), counts its
    lists, and is answered unknown at the bound;
  - the oracle keyed witnesses by the list of terms, quadratic in the
    nest (77 s for 30 000): witness lists are interned and it asks the
    stop;
  - the stop and the work bound were asked once per loop iteration, so a
    goal with thousands of failing extensions ran twenty million steps
    past a work bound of a thousand: every step ticks now, and the
    matrix's bytes count in the memory bound;
  - `--path-length 0` was reported as 1: a bound of 0 is one round at 0;
  - the occurrences past the `u32` indices were `Refusal::Occurrences`
    with a wrong limit, and the tables' `u32` casts unchecked: `indexed`
    and `MOST_ENTRIES` refuse with `Refusal::Index` and
    `Reason::IndexLimit`.
  `the_bounds_hold_where_steps_are_dear` pins the first, third and
  fourth.

### The reviewer's round

`fresh-reviewer` (Fable 5.2 at `medium`) on the chain after the panel's
answers found no soundness break (its differential runs: 2 500
sequents with δ-quantifiers only against the SAT solver on their
Skolemized form, 1 700 general first-order sequents against the
embedding, no mismatch and no read-back failure). Its findings and the
answers:

1. **The protocol's `ordinary.decide` did not list `connection`** among
   its method values (`core/src/protocol/ordinary.rs`): added. The web
   bindings' table of calls (`web/src/calls.rs`) has no entry for it;
   the page's methods are the web front end's to add (below).
2. **The docs said "not valid" comes for a sequent with no `∀` left of
   `⊢` and no `∃` right of it**, which negation and an implication's
   antecedent make false both ways (`(forall x. p(x)) -> q |- q` is
   refuted): reworded everywhere by what a quantifier needs, a witness
   term (every `∀` one to prove, every `∃` one to assume).
3. **The measurement was not in the report** at the head it read:
   written above.
4. **The module docs called the deepening "complete"**, against
   `Reason::Incomplete`: reworded ("no proof is out of reach for want of
   copies").
5. **The manual's limits table lacked `--path-length`** and
   `--restricted-backtracking` had no invocation: both added
   (`options.md`, a console block in "First-order sequents").
6. **A comment narrated an earlier version** of the read-back's last
   rule: cut.

### What was checked, and by what

- **The gate** (capped, cores 7 to 9) at the panel's answers: Clippy
  (the workspace, the core crate without `parallel`), every test, both
  `cargo hack` runs passed; its ratchet failed on `ordinary-pigeons`
  alone, which `main@origin` fails the same way (below).
- **`nix flake check --keep-going`** at the chain's head under the
  machine lock: every check passed but `doc`, `doc-private` and
  `manual-links` (a redundant explicit link in `connection/mod.rs`,
  fixed in `yzyosluw`; the three rebuilt and passed) and `ratchet` (the
  base's excess). `budget`, `web-package`, `wire-schema`, `features`,
  `test-*`, `export`, `rocq`, `manual` among the passed.
- **`cargo test -p deduxit-cli`** after the reviewer's answers: the
  behaviour lock, README's, the manual's and the guide's examples, the
  CLI's tests, all passed.
- **The behaviour lock** was reblessed twice in commits of their own
  (`xupqprul`: four calls of the prover and the help's two flags;
  `uqtosvxy`: its derivations, which weaken earlier).
- **ILTP**: the three runs above; no verdict against a status, no two
  verdicts for one problem.
- **The panel and the reviewer**: above, every finding answered in the
  code and pinned by a test where it had a witness.

### What 75c and 75d must know

- **Sharing the method** (agreed with 75c): one `Method::Connection` for
  every logic it comes to decide; 75c's prover lives under
  `core/src/ordinary/connection/intuitionistic/`, and changes only
  `Method::logics`, `decides`, `connection::run`'s refusal and
  `method_for` for its logics. It may build on `matrix.rs` and
  `terms.rs`, whose shapes the panel's answers changed (`Matrix::of`
  takes the stop, `Matrix::opaque`, `Matrix::held_bytes`,
  `Terms::longest`).
- **Copies as regions are this prover's own choice**: a prefixed
  calculus copies per literal instance, and its read-back to LJ needs
  the same discipline as `read_back.rs` (Skolem terms over every
  γ-slot around, the order of the last rule by nesting, the stand-in
  witness for an unbound variable and for units under γ-formulas).
- **The command's default runs the prover to the time limit**: at the
  2 s default six ILTP theorems the embedding proves time out in the
  prover and are not handed over (the library's default, with its path
  bound of 8, hands them over). A portfolio or a time share (75d, or
  the planning session's call) would recover them.
- **The web page has no `connection` call**: `web/src/calls.rs`'s
  table of the page's methods and the client (`../deduxit.github.io`)
  are the web front end's; the protocol's `ordinary.decide` lists the
  method.
- **The measurement beside nanoCoP and leanCoP is 75d's**: the
  harness runs `--engines connection` on `--tptp`, and the comparison's
  drivers are design §8's.
- **The ratchet's `ordinary-pigeons` is over its ceiling on `main`
  already** (32 402 779 instructions on `main@origin`, `tuskuswx`, under
  this session's symbols build; 32 399 463 at this chain's head; the
  ceiling 31 264 589, +3.6 %): the gate's ratchet fails on it at both,
  which this chain leaves to the planning session (a ceiling's raise is
  a commit of its own that says why).

## Session 75c: the intuitionistic connection prover

Opus 5.5 at `high`, in the workspace `../deduxit-wt-step-58b-8c0bbc`,
cores 10 to 12, continuing 75a's conversation. Nothing was pushed.

### Outcome

- `ordinary::connection::decide` and `prove --logic intuitionistic
  --method connection` decide intuitionistic sequents, first-order ones
  included, by nanoCoP-i's prefixed non-clausal connection calculus
  (Otten, TABLEAUX 2017; ARQNL 2016), under the one `Method::Connection`
  agreed with 75b. "Valid" comes with a derivation of LJ that
  `Derivation::check` accepted; every other answer is "unknown"
  (`Incomplete`, `PathLength`, the time limit), since the search is no
  decision procedure even without quantifiers.
- The module `core/src/ordinary/connection/intuitionistic/`: `prefix.rs`
  (the prefix characters, their copies, Otten and Kreitz's Lemma 8 test
  and T-string unification), `prove.rs` (the classical search, copied
  and changed), `read_back.rs` (a G3i search the proof guides, and the
  emission of LJ), `mod.rs`, `tests.rs`.
- **Minimal logic stays on the embedding**: the connection prover
  refuses it (`Error::Logic`). The plan (`⊥` an atom, `¬A` as `A → ⊥`)
  needs a map from the rewritten sequent's derivation back to the
  original, which is ambiguous where the original holds both `¬A` and
  `A → ⊥`; the differential test was not run, so nothing says it agrees.
  (departs from the brief, which asked for the differential result)
- The default of intuitionistic logic does not change (75d's call).

### The calculus, from the papers

A research agent read the papers (not the provers' GPL code) into notes
kept with the step's scratch files; what the code rests on:
- Prefix characters: a variable for `¬`, `→`, `∀` and an atom with
  polarity 1, a constant with polarity 0 (Otten 2017, Table 1); a
  literal's prefix the characters from the root down to it, its own
  included.
- The domain condition compares the prefix of the γ-position *with* its
  own character (ileanCoP 2005, Definition 6): with the plain prefix
  `∀x(A ∨ B(x)) ⊢ A ∨ ∀x B(x)`, not intuitionistically valid, would be
  proved.
- T-string unification's rules R8 and R9 read `V s⁺` (Otten and Kreitz
  1996); the 2016 and 2017 papers print `V a u`, a misprint.
- nanoCoP-i searches classically, filters each connection's prefixes by
  Lemma 8, and unifies all the equations once a classical proof is found,
  backtracking on failure (Otten 2016 §3.2, 2017 §6).
- Kreitz and Schmitt's conversion (Information and Computation 2000)
  gives a multi-succedent LJ proof; its second step to single-succedent
  LJ (Egly and Schmitt 1999) was not available.

### Departures

- **Prefix unification at the end of each proof, not after each
  connection** (departs from the design, §4): nanoCoP-i's own scheme;
  unifying per connection means backtracking over the unifiers inside
  the search, since T-string unification is not unitary.
- **The read-back is a G3i search the proof guides, not Kreitz and
  Schmitt's conversion** (departs from the design, §4): the conversion
  is multi-succedent. The search takes only the substitution's instances,
  each Skolem term as the eigenvariable its δ-rule made on the branch or
  as a constant of the branch (75b's finding: its δ-rule may lie on
  another branch), so it is complete where Kreitz and Schmitt's theorem
  says the instances suffice, within its bounds.
- **A proof whose prefixes unified but that does not read back is
  rejected and the search goes on** (departs from the brief, which calls
  that a bug's refusal): the reduction ordering's cycles through prefix
  constants are not checked apart (the classical Skolem terms do not take
  prefix variables), so the read-back is the admissibility check of last
  resort. `decide_counting` counts such proofs; `agrees_with_ljt` holds
  the count to zero on 300 generated sequents.
- **Based on 75b's moving chain** (departs from the brief's `jj new
  mxyrmqtz`), to share its matrix and terms: three times a rewrite in
  75b's workspace raced a snapshot here, and files vanished from disk;
  each time jj's operation log held the work (`jj --at-op OP
  --ignore-working-copy diff`, then the full side of the divergence kept;
  once I kept the wrong side and restored the lost file from the
  abandoned commit). From then on the chain sat on `jj duplicate`d copies
  of 75b's commits, and moved onto `main` when 75b landed. A jj command
  here once signed six of 75b's unsigned commits (75b was told).

### What went wrong on the way, found by the measurements

- Copies renamed a prefix variable only where every literal of its
  position lay in the clause, and constants per copy: `⊢ ¬¬(a ∨ ¬a)`
  was unknown. Definition 5's renaming and constants as Skolem terms over
  the variables above fixed it.
- The final unification cloned the whole system at each step and asked
  no stop: 8 to 12 s under a 2 s limit (SYN070, TOP022). Now one equation
  and the bindings per state, the stop every 256 steps.
- The read-back's search asked the stop per sequent only, while one
  sequent built an instance per witness: 6.7 s under 2 s (SYJ004).
  Instances are now built once and every child counts as work.
- Six problems failed the checker: a child re-read its parent's binders
  through the branch's current map of Skolem terms, which the branch had
  changed. Each binder's term is now kept as bound.
- A δ-formula whose variable no literal holds has no Skolem symbol in
  the matrix: four panics (SYN346, SYN400, SYN932, SYN933).

### Measurements

ILTP's first-order problems, intuitionistic logic, `deduxit-bench run
--tptp bench/iltp-first-order/Problems --logics intuitionistic
--engines connection --timeout 2`, three shards on cores 10 to 12, at
`qkvktkmy` (after the panel's and the review's fixes), beside the
embedding's row of 75a
(the default, `auto`):

| rows | connection | embedding (75a) |
|---|--:|--:|
| proved | 155 | 226 |
| proved by this one only | 34 | 105 |
| unknown, time limit | 667 | 28 |
| unknown, `incomplete` | 107 | 34 |
| unknown, derivation limit | 6 | 1 |
| unknown, memory limit / copy bound | 1 / 0 | 0 / 614 |
| refused, equality | 1 730 | 1 730 |
| error, the TPTP reader (GEJ056, GEJ057) | 4 | 4 |

- No verdict contradicts a status; no crash; no row past 2.4 s. The
  four errors are the TPTP reader's (an unbound name where a term is
  expected), the same under both methods.
- Before the fixes (at `szpmmnsy`): 148 proved; the fixes lost none
  (the same 155 at `uztnoqsq` and at `qkvktkmy`) and gained seven (SET575+3, SYJ018+1, SYJ022+1, SYJ027+1, SYN362+1,
  SYN375+1, SYN382+1), the read-back's order of witnesses most likely.
- Together they prove 260 of ILTP's first-order theorems, the
  embedding alone 226. nanoCoP-i proves 704 to 839 at 100 s (Otten 2017
  Table 3; 2021 Table 4), so the prover's search, not its idea, is what
  is behind: no restricted backtracking schedule, no strategy portfolio,
  2 s.
- Generated propositional sequents against LJT (`agrees_with_ljt`): no
  "valid" LJT refutes, at least four fifths of LJT's valid ones proved
  within a path bound of 6, every unified proof read back.

### Panel and review

The panel ran on the chain rebased onto `main` (75b landed). The machine
stop at 14:20 cut the counterexamples and integers members; they ran
again from the start after the reboot, on the same commit, told of the
argument member's findings.
- **Argument** (Fable 5.2, `high`): soundness holds (no "valid" without
  the check; every arm of `Emit::emit` is a form `check_inference`
  takes; a child keeps its parent's binders' terms; a reused
  eigenvariable passes `mentions`); T-string unification holds (a
  transcription against an oracle on all 3 969 pairs up to length 5: no
  false "no" from `unifiable`, no wrong unifier); copies and constants
  as the rules say. Breaks of the bounds and one lost proof:
  - B1: `Prefixes::of` kept every literal's chain and keyed each
    constant by the list of variables above it, neither charged nor
    polled: `|- ~…~p` with 100 000 negations took 4.8 GB and 4.8 s
    under `--memory-limit 64MiB --timeout 2s`.
  - B2: the read-back's search copied the sequent per level, uncharged:
    725 MiB under 64 MiB on `a0, …, a29999 |- a0 /\ (… /\ a1499)`.
  - The read-back lost `forall x. exists y. r(x,y) |- exists y z w.
    (r(c,y) /\ r(y,z) /\ r(z,w))` (valid).
  - Notes: constants ignore term variables (Definition 8 has them), so
    the read-back rejects more than the cycles the rules name (`|-
    exists x. (p(x) -> p(f(x)))`; sound, a lost proof); an arena
    holding `Eigen(u32::MAX)` saturated `next_eigen`.
- **Counterexamples** (Opus 5.5, `high`): no soundness failure. 2 200
  random propositional sequents against a G4ip of its own, LJT and
  random Kripke models; 2 400 random first-order ones and 26 by hand
  (DNS, CD, the drinker, IP, Markov unknown, their valid forms proved)
  against random constant-domain models; 3 200 unification systems
  against a brute-force solver (no wrong unifier, no false "no" from
  `unifiable`, the ten rules checked against the 1996 table). One
  break: `free_vars`' suffix unions were quadratic in a clause's width
  (`|- ((a0 -> b0) \/ (c0 -> d0)) /\ …` with 8 000 conjuncts: 5.1 s
  past a 2 s stop, 16 000 past 8 GB). It also saw the derivation bound
  (64 MiB) refuse the read-back of small valid sequents (4 of 400 at
  depth 5): a refusal, exit 2, as the rules say for any read-back.
- **Integers** (Sonnet 5.5, `high`): no wrap, panic or wrong verdict.
  Two breaks of the time limit: R1, the same width as above (100 000
  hypotheses, 7 s at 1 s); R2, the read-back walked shared proof terms
  as trees (`p0(a), forall x. (p_k(x) -> p_{k+1}(f(x,x))) |- exists y.
  p_n(y)`: 13.5 s at 2 s for n = 22, doubling per level), and so did
  the domain equations. Noted: `Prefixes::count` could reach the
  auxiliary variables' numbers after 2³¹ fresh variables; the
  recursion depth held at 16 300 levels in release.
- **What was fixed** (one commit after the prover's):
  - prefixes read up parent links, constants keyed by an interned list,
    the free variables' lists made by walking variable-only links and
    charged and polled (`Tally`), the barred sets counted per variable
    over a clause's kids; the search counts prefix characters read
    towards its polls and charges the prefixes' tables and equations;
  - unification bounded by the characters its waiting states hold
    (`UNIFY_HELD`) and polled by the characters it handles;
  - the read-back charges every open level and every step to the
    memory bound (a refusal there is `MemoryLimit`, as the stop is
    `Stopped`), polls by hypotheses copied, recurses to half the
    recursion depth (a level takes 14.9 KiB of stack unoptimized
    against the 13 KiB a level is sized for: 2 000 nested implications
    overflowed a debug build), and visits each part of a shared term
    once (`tm_of`'s memo, `term`, `ready`, `constants`, the domain
    equations);
  - the order: the goal's `∃R` with witnesses the branch has met, then
    the hypotheses' choices, then the goal's other witnesses, the
    anonymous witness last everywhere (it fit the goal first and spent
    the step bound on the lost sequent, which now reads back and is in
    the test);
  - `Prefixes::exhausted` refuses to unify past 2³¹ numbers;
    eigenvariable numbers that run out give no derivation.
  - After: B1's witness 0.02 s and 22 MB; B2's refused at the bound;
    30 000 nested implications 0.86 s and 35 MB (1.9 GB before); the
    wide witnesses 0.1 s (refused at the memory bound where it is
    64 MiB); R2's n = 22 0.4 s.
- **Left, in the classical prover's shared code**: R2's input stays
  exponential in `connection/terms.rs` or `matrix.rs`, the same in both
  logics (n = 26 at 2 s: classical 8.5 s and 2 GB, intuitionistic
  5.2 s). Not this session's code; a follow-up for 75b's files
  (`Terms::occurs` walks a shared term as a tree).
- **`fresh-reviewer`** (Fable 5.2, `high`, at `uztnoqsq`): soundness
  and the read-back's promise held (about 12 300 fuzzed sequents and 28
  by hand: no `Error::ReadBack`, no panic, no "not valid"; 16 000-deep
  chains did not overflow). It found five breaks of the bounds and
  three wrong texts, all fixed in `qkvktkmy` (the lock's help in
  `wuplrsyt`):
  - F1: the copied search counted the ways a literal tries but polled
    only in its main loop, without the classical prover's later
    `tick`/`halted`, `MOST_ENTRIES` and `eclauses` mark: 102 million
    steps, 5.3 s at a 1 s limit, 981 MB under 64 MiB, and `--work-limit
    1000` ignored. Now ported: 1.1 s, and the work limit at 1 024
    steps.
  - F2: each child in the read-back instantiated its subformula afresh
    and counted one unit: 12.6 s at 1 s (900 conjuncts over a
    disjunction of 100 000). A child is now its parent's instance's
    operand or the binder's body instantiated: 0.3 s.
  - F3: instances charged 64 bytes each: 4.56 GB under 64 MiB. Now by
    the nodes they add: refused at the memory bound, 237 MB peak (the
    input alone is 950 KB of text and its matrix), 2.54 s at a 2 s
    limit.
  - F4: the emitted derivation held to `derivation_bytes` alone:
    3.64 GB under 64 MiB with `--derivation-limit none`. Now charged:
    the memory limit at 67 ms.
  - F5: `--path-length 0` searched at 1. Now one round at 0, as the
    classical prover.
  - T1, T2: `connection::decide`'s docs and `prove --help` still said
    classical logic only; T3: "any other is unknown" read as a promise
    that every valid sequent is proved. All three reworded.

### Checks

- `cargo test --workspace` passed on the chain rebased onto `main`
  (`vuyutrwt`), and again at `uztnoqsq` (after the panel's fixes);
  `cargo clippy --workspace --all-targets -- --deny warnings` clean at
  both; the gate passed at both (`ordinary-pigeons` 32 380 060 against
  its ceiling of 31 797 472, +1.83 %, within the tolerance, as on
  `main`).
- The prover as first committed had not been through `nix fmt`; the
  fixes' commit carries its formatting.
- `nix flake check` under the machine lock: the first run (23:29)
  failed `doc-private` (a redundant link target in the prover's
  `decide`, fixed in its commit) and `budget` (the web module at
  2 590 533 bytes at `gzip -9`, over 2 560 000: raised to 2 650 000 in
  `llzyrxmy`, which says what earned the room); the second (23:46 to
  00:15, at the head with this report) passed every check. Only this
  section's text changed after it.

### What 75d must know

- **The method**: intuitionistic logic goes through `Method::Connection`
  (`connection::run` dispatches by logic); minimal logic is refused
  there (`Error::Logic`) and stays on the embedding: its differential
  test was not run (the Outcome above says why).
- **Not a default yet**: `Options::method_for` still sends a
  first-order intuitionistic sequent to the embedding. The measurement
  says the two complement each other (155 and 226 proved, 260 together,
  34 by the connection prover alone), so a race or a fallback, not a
  switch, is what the data supports; the classical default's
  `gives_way` already treats `Incomplete` as giving way.
- **It never answers "not valid"**: every failure is `Incomplete` or
  `PathLength`; a race may treat its unknowns as no information.
- **Its unknowns at the time limit dominate** (667 of the ILTP rows at
  2 s): the search has no restricted-backtracking schedule and no
  portfolio; nanoCoP-i's numbers are at 100 s.
- **Refusals**: a read-back past `derivation_bytes` is an error (exit
  2), as for every read-back; the stop and the memory bound are
  unknowns.
- **Open in shared code**: chains of shared bindings are exponential in
  `connection/terms.rs` (both logics); constants ignore term variables,
  which loses proofs, never soundness.

## Session 75d: the fragments, the comparison and the defaults

Opus 5.5 at high, cores 7-9, the panel on 14 and 15 (13 was busy);
on 75b's chain, rebased onto `main@origin` (`vuyutrwt`) after the pause
of 2026-11-21, and onto 75c (`koqkturo`) on 2026-11-22: the conflicts
were the method's tables (75c's intuitionistic line for the connection
prover beside grounding's), the help, the changelog, the manual, the
rules, the budget and this report.

### Outcome

- **Grounding** (`ordinary::grounding`, `Method::Grounding`, `--method
  grounding`, feature `sat`): the Bernays–Schönfinkel class and the
  monadic class (through miniscoping with Behmann's distribution,
  bounded), grounded over the constants and decided by the SAT solver,
  in every logic. "Valid" only classically, with the checked clausal
  proof, and by default the connection prover's derivation of LK where
  it finds one within `EVIDENCE_WORK`; "not valid" in every logic with
  a `FiniteCountermodel` (the sequent and a `Structure`) checked by an
  evaluator over the original sequent. `check` evaluates an outcome's
  structure again (`Error::Structure`, `invalid_structure`, for a wrong
  one), and so does the protocol's `decides`.
- **The defaults**: grounding for a first-order sequent of its classes
  in every logic (within `grounding::DEFAULT_NODES`, 2²¹ nodes); then
  the connection prover in classical logic, the embedding in the
  others. The command gives grounding or the connection prover 90% of
  its time limit and the embedding the rest (`FIRST_ORDER_SHARE`).
- **The comparison**: leanCoP 2.1, nanoCoP 2.0, nanoCoP-i 2.0 and
  iLeanCoP 1.2, fetched by hash at their leancop.de archives (GPL, no
  version stated, recorded in `cop.nix`'s sources and `bench/TOOLS.md`),
  run on SWI-Prolog by `deduxit-bench run --tools …`.
- **The docs**: the manual (the decidable classes with console
  examples, the default order and the share, the outcome's `structure`),
  `status.md`, `benchmarks.md`, the guide, the changelog, the schema
  (grounding's search and structure, and documents of grounding's and the
  connection prover's outcomes in the `wire-schema` check, which had none
  of the latter), the rules files.

The chain, in order: `lokltvyz` the decider, the command, the check and
the protocol; `znytlnqn` its lock; `nuqswqnw` the set-up's polls and
memory, the structure kept or dropped, the fallback to the connection
prover; `lzzlwoux` the command's time share; `qoqtqzrp` the harness;
`yutxzwtu` the comparison's provers; `lrwyqtsm` miniscoping's
distribution; `unxzultl` the docs; `plrxolky` the default's bound;
`yxuppzvw` the panel's fixes (a Skolem constant per `∀` node, the
bounds); `ywxpknqr` the memoised walk and δ⁺⁺ sharing; `wwutuxqn` the
reviewer's two; `kzrnkuqx` the comparison's and the defaults' rows;
`kzwmvzmz` the lock's rebless; `ywyrqxpp` the web module's size; then
this report.

### Departures

- **The comparison runs on ILTP's 940 first-order problems without
  equality alone** (departs from the design, §8, and the brief: "and a
  TPTP subset"). The flake fetches no TPTP library, ILTP's problems are
  TPTP problems, and the 1 730 with equality would measure nothing of
  deduxit, which refuses them. 107 of the 940 hold Latin-1 in their
  comments, which the command refuses as not UTF-8; every tool got those
  converted.
- **The provers run under a driver of ours** (departs from the design,
  §8, which reads their output lines and exit statuses):
  `modules/_provers/cop.pl` runs each release script's strategies in
  their order with their shares of the time limit, in one SWI-Prolog
  process, and prints `verdict(…)`. The scripts poll `ps` once a second
  and so answer nothing before a second has passed, which at 2 s would
  measure the scripts, not the provers.
- **iLeanCoP 1.2 runs on SWI-Prolog with TPTP read by leanCoP 2.1's
  reader** (departs from the design: its release needs ECLiPSe, which
  nixpkgs lacks, and TPTP2X): `lib(iso)` left out, ECLiPSe's global
  occurs check as SWI-Prolog's, a `retract_all/1` shim; the changes are
  recorded in its source record (`changed`).
- **The finite countermodel's type is this step's** (departs from the
  design in order, §5: "step 76 owns the countermodel's type"): kept to
  the sequent and a structure of constants and predicates, so that step
  76 extends it.
- **A structure too large to check is dropped and the verdict kept**
  (departs from the design, §5, which makes the checked structure the
  evidence of "not valid"), as a Kripke countermodel too large to check
  is: the solver's model of the ground sequent is checked, and "not
  valid" holds for anything grounding admits, in the class or not (the
  argument panel's step 3).
- **Grounding is the default for its classes in every logic, before
  the connection prover** (departs from the design, §7, which names the
  connection prover alone): by the measurement below.
- **The monadic class is decided as far as bounded miniscoping brings
  it into the Bernays–Schönfinkel form** (departs from the design, §5:
  "miniscoping turns it into the EPR form"): distribution is bounded
  (`DISTRIBUTE_DEPTH` 8, `DISTRIBUTE_NODES` 2²⁰, the copies' size checked
  before they are made); a sequent past them is outside the class, which
  is sound.
- **75c's prover is not in the intuitionistic default** (departs from
  the planning session's instruction at the rebase, "75c's prover in the
  intuitionistic dispatch", and from this report's own plan before it,
  on the rule that defaults follow a measurement): after the rebase onto
  75c, `deduxit-bench run --tptp … --logics intuitionistic --engines
  connection,embedding --copies none --timeout 2`, three shards on cores
  7-9 (`bench/defaults/intuitionistic-iltp-first-order-{1,2,3}.csv`): the
  embedding without a copy bound, as the command runs it, proves 374 of
  ILTP's first-order problems intuitionistically, the prover 155, both
  together 376 (the embedding misses two of the prover's). Simulated
  from those times at 2 s, the prover first with 90%, 50% or 5% of the
  time and the embedding after proves 338, 362 or 373, and the
  embedding first then the prover at most 372: every order with the
  prover in it loses theorems. So the intuitionistic default stays
  grounding, then the embedding; the prover is `--method connection`.
  75c's own figures (155 and 226, 260 together) set the embedding's copy
  bound at 3, the library's, where the embedding gives way early.

### Measurements

The defaults (`bench/defaults/grounding-iltp-first-order-{1,2,3}.csv`,
2026-11-21, the final build, `--engines grounding,auto` at 2 s on
ILTP's 2 670 first-order problems, three shards on cores 7-9; `auto` is
the library's default, which has no clock and so no share):

| logic | default proved | default refuted | grounding refuted | grounding proved |
|---|--:|--:|--:|--:|
| classical | 674 | 77 | 77 | 360 |
| intuitionistic | 224 | 109 | 77 | – |
| minimal | 189 | 111 | 77 | – |

- 1 730 problems with equality are refused and four do not read, in
  every row. No verdict contradicts a file's intuitionistic status or
  (checked here for the first time) its classical `Status`, and no
  problem gets two verdicts.
- Against 75b's default (594 proved, 1 refuted), the classical default
  decides 158 more and loses SYJ208+1.010 (75b proved it in 1.98 s
  against the 2 s limit). Of the classical default's verdicts, 524 are
  the connection prover's, 157 grounding's, 55 the embedding's.
- **SYN316+1 was answered "valid" before the panel's fix** (`∃x ∀y (F(x)
  ↔ F(y))`, classically CounterSatisfiable; the harness compares with
  the intuitionistic status only, so my own checks missed it until the
  classical status was read): the shared Skolem constant of a split `∀`.
  It is refuted now.
- **The command's share**: simulated from 75b's per-problem times
  (`r3` and the first run's embedding rows), the prover alone proves 599
  at 2 s; 90% for it and 10% for the embedding 604, 50% 602; the prover
  proves all but three of its theorems within 0.6 s, and five of the
  seven theorems the embedding alone proves take it under 6 ms. The
  comparison's deduxit column (the command with the share) proves 689
  and refutes 77 of the 940 classically.
- **`DEFAULT_NODES`**: the largest grounding decided within 2 s has
  65 540 atoms (SYJ036+1, 1.5 s); SYJ020+1's grew until the limit and
  kept the connection prover, which proves it in 0.94 s, from running.

### The comparison

`bench/COMPARISON.md`, last section, and
`bench/comparison/2026-11-21-iltp-first-order/` (`runs-{1,2,3}.csv`,
`ranking.md`): each problem once per tool under runexec at 2 s and
4 GiB, three streams on cores 7-9.

| logic | tool | decided | proved | not valid |
|---|---|--:|--:|--:|
| classical | deduxit | **766** | 689 | 77 |
| classical | nanoCoP 2.0 | 713 | 629 | 84 |
| classical | leanCoP 2.1 | 666 | 640 | 26 |
| intuitionistic | nanoCoP-i 2.0 | **568** | 482 | 86 |
| intuitionistic | iLeanCoP 1.2 | 559 | 485 | 74 |
| intuitionistic | deduxit | 486 | 377 | 109 |

No contradictions between tools or against a status. On the problems
all three decide (626 classical, 376 intuitionistic) the medians are
5.3 ms, 35 ms and 39 ms classically, 6.0 ms, 41 ms and 35 ms
intuitionistically. Alone: deduxit 124, nanoCoP 51, leanCoP 4
(classical); deduxit 98, nanoCoP-i 21, iLeanCoP 18 (intuitionistic,
the provers' mostly GEJ theorems the embedding leaves open at 2 s).

### The panel

Argument (Fable 5.2), counterexamples (Opus 5.5), integers (Sonnet
5.5), all at high, on cores 14 and 15; a first run was cut by the
graceful stop before it reported and started afresh after.

- **Argument: a wrong "valid"** (`q(a), ~q(b) ⊢ ∃z ∀x ((p(x) ∨ q(z)) ∧
  (¬p(x) ∨ ¬q(z)))`): miniscoping splits a `∀` over `∧` into nodes of one
  variable, and the Skolem constants were keyed by variable. Answered in
  `yxuppzvw`: one constant per `∀` node and value of its free `∀`
  variables (a first pass visits each pair once); the counterexamples
  member found five more witnesses on the old build, all refuted with
  checked structures on the new one, and ILTP's SYN316+1 was one too.
- **Argument and integers: the set-up was unbounded**: `in_class`
  walked the shared DAG as a tree (256⁴ paths, killed at 8 GiB);
  distribution had no size check and `prepare` copied a chain's operands
  at every level (quadratic: 100 000 conjuncts took 8 GB); the normal
  form was not charged. Answered in `yxuppzvw`: one pass for the class
  test, n-ary junctions as a chain is entered, distribution's size
  checked before it copies, the normal form and the ground tables
  charged (the estimates doubled after the integers member measured them
  1.4 to 2.2 times short), a memory refusal unknown at that bound in the
  set-up and the solver alike (it was exit 2 in one, 3 in the other).
- **Argument: an unused constant of the arena** made a right refutation
  "invalid": only the terms the sequent's formulas use are checked now.
- **Integers: a structure read from JSON grew 8 to 13 times its text**,
  uncharged: its tables are charged before its sets are built; serde's
  own reading of the document is still bounded by the document's size
  alone.
- **Counterexamples: a stall the Skolem fix brought** (a BS sequent with
  equivalences, 24 ms before, the time limit after): distribution shares
  subformulas and the ground walk took them as a tree, ninety million
  steps for 1 940 nodes. Answered in `ywxpknqr`: the walk memoised by
  node and the values of its free variables, and equal `∀` nodes sharing
  their constant (δ⁺⁺). Its slow monadic sequents went from past 20 s to
  1.7 s or a memory refusal at 3 s.
- What held: the counterexamples member's own reference (exhaustive
  model search, exact on the monadic class) agreed on 11 000 sequents of
  the new build in all three logics, 22 000 mutated structures and 1 616
  `check` documents; the argument member checked the normal form, the
  equivalence of every miniscoping step, the evaluator and the dispatch.
- Left: the overrun after a stop (freeing the ground arena's per-atom
  names: 0.45 s at 2 s, 1.1 s at 4 s, worse with a raised memory limit);
  `grounds` (the default's class test, several times per command) has no
  stop, bounded by `GROUNDS_NODES` and linear.

### The reviewer's round

`fresh-reviewer` (Fable 5.2), once, after the panel's fixes. No
soundness break (about 16 000 sequents against its brute-force model
search, every forged structure refused, the comparison's figures
recounted). Answered: the manual's three structure examples still showed
the elements of the walk before the memo (`wwutuxqn`, and the lock's
rebless `kzwmvzmz`); `check` named a grounding outcome without its
structure LJT's (`wwutuxqn`); the TPTP subset is stated as a departure
above. Left to the author: `modules/_provers/ileancop_main.pl` (ours,
EUPL header, ten lines of glue) follows leanCoP 2.1's `leancop_main/3`
(GPL) closely, since it calls the same reader, `leancop_equal` and
`prove2` in the same order; whether that counts as vendoring is the
author's call, and it can be rewritten or dropped (iLeanCoP's column
with it).

### What was checked, and by what

- **The gate** at `kzwmvzmz` passed (clippy, the tests, the lock and
  README's and the guide's examples, both `cargo hack` runs, the
  ratchet: `ordinary-pigeons` 32 394 319 instructions, +1.88 % against
  the raised ceiling and within its tolerance, the propositional path as
  before).
- **`nix flake check`** at `ywyrqxpp` under the machine lock: every
  check passed, after the web module's budget was raised in a commit of
  its own (2 590 216 bytes at gzip with grounding, over the 2 560 000 it
  had). After the rebase onto 75c, which raised the ceiling to 2 650 000
  for its own prover, the module is 2 631 105 bytes with both provers and
  grounding, so the ceiling stays and `ywyrqxpp` only records the
  figure. Among them `wire-schema` with the new
  documents of grounding's and the connection prover's outcomes and
  `check-structure`'s, and `manual` and `manual-links`.
- **The tests**: `grounding`'s hand cases (both classes, the panel's
  and the counterexamples member's witnesses of the split `∀`, the
  distribution's monadic sequents, the three logics) and 400 generated
  function-free sequents against the connection prover; the lock's nine
  grounding calls (refutation, JSON, clausal proof, intuitionistic, both
  unknowns, `check` right, wrong and in JSON).
- **ILTP**: the defaults' and the comparison's runs above, every verdict
  against both statuses of every file, none contradicted.
- **The panel and the reviewer**: above.

### What step 76 must know

- **The type**: `ordinary::FiniteCountermodel { sequent, structure }`
  and `Structure { size, constants, predicates }` (both
  `#[non_exhaustive]`), JSON `"structure": {"sequent", "structure":
  {"size", "constants", "predicates"}}` beside `not_valid` (level 25),
  `deny_unknown_fields` on `Structure`. A predicate left out holds of
  nothing; a nullary one holds where it lists the empty tuple.
- **The check is the one trust**: `FiniteCountermodel::check(logic,
  limits, stop)` evaluates the original sequent (`Structure::refutes`,
  iterative, short cuts, `limits.work` or `Structure::WORK`, the stop
  every 4 096, the tables charged); `Error::Structure` for a structure
  that cannot interpret the sequent or does not refute it. It refuses
  function symbols with arguments today: a structure that interprets
  functions extends `Structure`.
- **What the fragments leave**: every sequent with function symbols,
  Ackermann's and Gödel's classes (a finite-model search, not a
  grounding), monadic sequents past bounded miniscoping, equality, and
  in intuitionistic and minimal logic every classically valid sequent
  (grounding answers `Unknown(Incomplete)`, and the default goes on).
  A first-order "not valid" in those logics now comes only from a
  classical structure; a Kripke model with domains is step 76's.
- **The rule a change of miniscoping must keep**: one Skolem constant
  per `∀` occurrence of the formula grounded (per node and value of its
  free `∀` variables), equal nodes sharing (δ⁺⁺). A constant per variable
  was unsound.

### For the planning session

- `ileancop_main.pl`'s closeness to GPL code (above), the author's call.
- The command refuses TPTP files that are not UTF-8 (107 of ILTP's 940
  first-order problems without equality): a follow-up (read TPTP
  comments as Latin-1, or bytes).
- 75c landed; its prover stays out of the intuitionistic default by the
  measurement under "Departures". A race of the two (the prover on a
  second thread) would add at most the two theorems the embedding
  misses; minimal logic stays on the embedding, as 75c left it.
- The web page has no `grounding` call (`web/src/calls.rs` and the
  client are the web front end's; the protocol's `ordinary.decide` lists
  the method).
