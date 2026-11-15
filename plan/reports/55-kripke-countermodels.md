# Step 55: Kripke countermodels, and ordinary logic's shape

The step's sessions each add a section here: 55a (ordinary logic's
shape, Q5 and Q12), 55b (the countermodel), 55c (the library's lemma),
55d (the drawing and the page).

## Session 55a: ordinary logic's shape

Opus 5.5 at `high`, the reused worker of 54s-c3 (the app refused to
clear the conversation, so 54s-c3's context stayed; the work followed
the brief `start-55a.md`), in the jj workspace
`../deduxit-wt-step-54s-c1-161790`, from "First: ordinary logic's shape"
of `plan/55-kripke-countermodels.md`, on 2026-11-14, cores 4 to 6.

### Outcome

- **Q5**: `ordinary::decide(&sequent, &options)` decides within the
  default limits and the linear search's default options, beside
  `ordinary::decide_within(&sequent, &options, &search, &limits, stop)`,
  the linear suite's `prove`/`prove_within` shape. The module doc's
  example is three lines.
- **Q12**: every ordinary outcome is one document at wire level 12:
  `{"version", "deduxit", "verdict", "reason"?, "logic", "search",
  "derivation"? | "clausal_proof"?}`, `search` an object tagged by
  `method`: `{"method": "embedding", "translation", "image": {…}}` with
  the image's whole outcome nested (an outcome's keys without `version`
  and `deduxit`), `{"method": "ljt", "statistics"}`, `{"method": "sat",
  "statistics"}`. It covers every method the code has, the defaults
  included (their `search` is the method that gave the evidence: the
  SAT solver's, with LJT's statistics inside where LJT gave the
  derivation, or the embedding's where it gave the classical default's
  evidence).
- **The ordinary outcome reads back** (`wire::upgrade::<ordinary::Outcome,
  _>`, `Deserialize`), its derivation and the image's proof or disproof
  by their own readers, unchecked.
- **`check` reads every file `prove --logic` writes**, and a bare
  derivation of LK or LJ.
- **The command and the batch** write the ordinary outcome under
  `--logic` for every method; `--linear` writes the image's outcome
  alone, as before.
- **The bindings** write the new shape through the library (their
  `close` and `hint` serialize `ordinary::Outcome`); their tests follow.

The commits on `main@origin` (`rvuqtyyl`): `rpotllmp` (the library:
Q5, the shape, the reader, level 12, `Image::outcome`,
`Evidence::Image`, `Error::NotBuilt`, the bindings' tests), `ptutvqvm`
(the command's JSON and `check`, the manual, the guide, the changelog),
`vumlvmvs` (the lock's JSON lines and the call `check-ordinary`),
`rspvxrkv` (the reviewer's findings), `yylvmzpl` (the lock's help
files), and this report's.

### The API and the JSON form, and why

- **`decide` and `decide_within`** take the linear suite's names and
  arguments (`prove`, `prove_within`), so a reader of one suite reads
  the other (Q5's recommendation). `decide` uses `search::Options::
  default()`, whose copy bound is 3, as `linear::prove` does: a call
  without a stop must end.
- **`search` tagged by `method`**: an internally tagged object, the
  form serde's `#[serde(tag = …)]` gives and JSON Schema's
  discriminator describes, as this crate already tags refutations and
  reasons by `kind` (Q12's recommendation). The tag's values are
  `Method::name` (`embedding`, `ljt`, `sat`), the words of `--method`.
  The defaults need no tag of their own: the outcome's `search` is the
  method whose evidence it carries (the SAT solver's with `ljt` inside
  its statistics where LJT gave the derivation; the embedding's where
  the classical default kept its evidence), which is what a reader
  needs to know, and the logic's default is in the documentation.
- **The image's outcome nested whole** under `image`, its keys as
  `prove` writes them without `--logic` but for `version` and `deduxit`
  (nested values carry none): a script that read the image's proof or
  refutation at the top reads it under `search.image`, and `check` reads
  it with the linear outcome's own reader.
- **The evidence keys stay at the top** (`derivation`, `clausal_proof`):
  they are the ordinary verdict's, beside which 55b's countermodel goes.
  A valid outcome with neither is `Evidence::Image`.
- **`Evidence::Image` and `Image::outcome`** (new API, needed by the
  shape; departs from the design, which named neither): the command
  answers "valid" for an image it proved whose derivation of LK a bound
  or the time limit refused (it has always printed the verdict and said
  the derivation was not built). The ordinary outcome had no evidence
  for that case; refusing it would have turned those answers into
  errors. `Image::outcome` makes the ordinary outcome from the
  command's own search (its race, the batch), reads the derivation back
  and checks it, and gives `Evidence::Image` where a bound or the stop
  refuses it; `decide` keeps its rule (a refused derivation is that
  refusal). `Evidence::derivation` answers the new `Error::NotBuilt`
  (kind limit, code `not_built`) for it.
- **Reading**: `wire::Readable` of form `ordinary outcome`, the
  derivation and the image's proof or disproof by their own readers,
  unchecked, as the linear outcome's `checked` is read. It refuses an
  outcome below level 12 (no release read the ordinary outcome: 0.3.0
  wrote it "written only", without the image's proof, so it cannot be
  rebuilt), one valid by a clausal proof (the document holds the
  proof's counts; a `ClausalProof` is built only by its checker), and
  one whose verdict the image's outcome does not bear out.

### The wire level

Level 12 (`wire::SEARCH`, `wire::LEVEL` 12): every ordinary outcome.
The session wrote it as level 11; step 57 reached `main` first with
level 11 for the basis refutation, so the rebase moved the ordinary
outcome to 12. `wire::CLAUSAL` (10) went, since nothing is written at 10
any more; the table keeps the row. `core/tests/wire/level-12.txt` holds
three outcomes (LJT's, the embedding's, the SAT solver's refutation),
which `the_twelfth_level_reads` reads and writes byte for byte. Every
earlier readable document is unchanged (the stability tests of levels 1
to 11 pass).

### Every reader changed

- `ordinary::Outcome`'s writer and its new reader
  (`serialize/ordinary.rs`), the linear outcome's writer split into
  the document and `Nested` (`serialize/search.rs`).
- The command: `prove`'s JSON under `--logic` (`Image::outcome`, then
  `ordinary::write_json`; `--linear` and `--net` as before), the batch's
  records and JSON files, `check` (`read_document` probes `logic`,
  `verdict` and `clausal_proof`; `check_ordinary`, `check_derivation`,
  `unchecked_into`; `Checked`'s `logic`), `eliminate`'s message for an
  ordinary file.
- The bindings: `close` and `hint` serialize the library's outcome, so
  they write the new shape without a change; their tests assert level 12
  and `search.method`.
- The harness: its calls of `decide` became `decide_within`; it writes
  no ordinary outcome.
- The goal state (`interactive.rs`) calls `decide_within`.

### What breaks (0.4.0)

In the changelog's breaks: `ordinary::decide`'s arguments (`breaks
0.3.0`: `function_parameter_count_changed`), and the JSON of `prove
--logic` under every method. `breaks 0.3.0`'s other failures are
earlier steps', already listed (`Outcome::translation` and `linear`,
`Rule::Cut`). Added, not breaking: `decide_within`, `Image::outcome`,
`Evidence::Image` and `Error::NotBuilt` (both enums
`#[non_exhaustive]`), the outcome's reader, the statistics' readers.

### The reviewer

`fresh-reviewer` (Fable 5.2 at medium) on `rpotllmp`, `ptutvqvm`,
`vumlvmvs`. It found no way to make `check` call valid what does not
check. Its findings:
- *`prove --help` stated the old shape in three places*: fixed in
  `rspvxrkv`, the lock's help files in `yylvmzpl`.
- *`check`'s help, the guide and the check chapter did not say what
  `check` reads or its `logic` key*: fixed in `rspvxrkv`.
- *An ordinary outcome found unknown was an error, exit 2*: now
  unknown, exit 3, holding nothing to check (the linear `check` keeps
  exit 2 for an unknown linear outcome, as before).
- *The reader builds `Evidence::Image` unchecked, though its doc said
  the checker accepted the proof*: the doc now says so of
  `Image::outcome` alone, and `Evidence`'s doc that evidence read from
  JSON is unchecked until its checker runs, as the linear outcome's is;
  `check` checks it (an image proof broken by hand is invalid, exit 1,
  the reviewer's witness).

### The checks

- The gate at `vumlvmvs` (cores 4 to 6): passed (the usual note of
  moved misses on `search-qbf-20-2`, no failure). The gate again at `yylvmzpl`, after the reviewer's
  fixes: passed.
- `cargo test --workspace` after the reviewer's fixes: no failure;
  clippy with `--deny warnings` clean on the workspace; the library's
  docs with every feature build with `-D warnings`.
- The ILTP agreement tests (`cargo test -p deduxit --release --test
  iltp -- --ignored`): the three pass (757 s).
- Every family at 5 s (`run --all-families --timeout 5`): 195 runs,
  `summary --strict` passes, no mismatch.
- The bindings' tests: their native run passes in the workspace test,
  and under Node in `nix flake check`'s `web-wasm-tests`.
- After rebasing onto `main@origin` twice (step 57, which took wire
  level 11, so the ordinary outcome moved to 12; then the dependency
  update, Rust 1.100.0): `cargo test --workspace` and clippy at the head,
  the gate at the rebased head (passed), and `nix flake check` under the
  machine lock at `kquywyxw`: "all checks passed!".
- `breaks 0.3.0`: run, its failures in "What breaks" above.

### Departures from the design

- `Evidence::Image`, `Error::NotBuilt` and `Image::outcome` are new API
  the prompt does not name (departs from the design); why above.
- An ordinary outcome below level 12 is refused, not upgraded (departs
  from the design, which says earlier documents stay readable): no
  release ever read it, and 0.3.0's lacks the image's proof that the new
  form holds.

### What the next sessions must know (55b)

- **Where the model goes**: a key `countermodel` beside `derivation` and
  `clausal_proof` at the top of the ordinary outcome, for `not_valid`,
  read and written in `serialize/ordinary.rs` (`OutcomeForm`,
  `OutcomeRead`). In the library, `ordinary::Outcome` needs the model
  kept: `From<sat::Outcome>` drops `sat::Outcome::model` today
  (`decide.rs`); the verdict `NotValid` could carry it, as `Valid`
  carries its `Evidence`, or a field beside `search`. Level 11 can take
  the key without a new level only if a reader of 11 that ignores it
  misreads nothing (it does not: the verdict stands without it), so a
  new level is not needed for the key alone.
- `check` answers a "not valid" without a refutation as unknown
  (`check_ordinary`, the arm `(NotValid, None)`): with the model, that
  arm checks it (`Model::refutes`) and answers valid or invalid.
- The SAT solver's `NotValid` in classical logic is one world; the
  embedding's `NotValid` carries the image's refutation already.

## Session 55b: the countermodel

Opus 5.5 at `high`, the reused worker of 55a (same conversation), in
the jj workspace `../deduxit-wt-step-54s-c1-161790`, from "Fixed now" of
`plan/55-kripke-countermodels.md` and the brief `start-55b.md`, on
2026-11-15, cores 4 to 6.

### Outcome

- **The output kind** is `deduxit::kripke::Model`, a root module shared
  by the suites: worlds, each with the atoms it forces by name,
  `falsity` (minimal logic's `⊥`), and its arrows to the worlds directly
  above it, the order being their reflexive and transitive closure; and
  `world`, the world the model is about (for a countermodel, where the
  sequent fails). It is data: each suite checks it by its own rules, so
  the modal suite (step 65) reads the arrows as its accessibility with
  its frame conditions; the classical assignment is the one-world case.
- **Its sources**: the SAT method's model (now the default's in
  intuitionistic and minimal logic) and, in classical logic, the SAT
  solver's assignment or the embedding's (the image's
  `Refutation::Classical`, its atoms the ordinary atoms by name), each
  checked by `Model::refutes` before it is kept. `ordinary::Countermodel`
  pairs the model with the sequent it refutes, and
  `ordinary::Outcome::countermodel` holds it for a "not valid". LJT's
  "not valid" has none (below).
- **The check is a checker** (`core/src/ordinary/model.rs`):
  `Model::forcing` checks the model is one of the logic, then computes
  forcing of every formula of the sequent's arena at every world, within
  `ordinary::MODEL_CELLS` (2²⁷ cells: worlds and arrows times formulas,
  16 MiB of bits), said at its declaration, and within
  `limits.memory_bytes` with what it holds beside the table; past either
  the check is refused as memory, and an engine's model it refuses is
  dropped (`Countermodel::found`): the verdict stays "not valid" and its
  line says "a countermodel of N worlds refutes it, too large to show and
  check".
- **A model the user gives**: `check --logic L --sequent SEQUENT
  [--world N] model.json` and the bindings' `ordinary_model`, below.
- **`check` checks an outcome's countermodel** (55a's `(NotValid, None)`
  arm): valid refutation, exit 0; invalid, exit 1.
- **`prove --logic`** prints the model after a "not valid" verdict
  line, a world per line (`world 0 (the model's world): forces nothing;
  below 1`).

The commits on `main@origin` (`kquywyxw`): `qrzksyox` (the library: the
kind, the check, the sources, the outcome's field, the JSON, level 13),
`tsnkovrm` (the command, the bindings, the manual, the guide, the
changelog), `lyoprosz` (the lock), `sskymumt` (the panel's fixes), `vyzvoxok`
(the reviewer's) and this report's.

### The output kind and its JSON

`{"version": 13, "worlds": [{"atoms": [], "arrows": [1]}, {"atoms":
["a"]}], "world": 0}`: `falsity` written only where true, `arrows` only
where there are some, `world` defaulting to 0 on reading. The reader
composes every name (NFC) and checks it as a sequent's atom name is
(`name::check`), and refuses more worlds, arrows and atoms together than
`limits.occurrences`; whether the model is one of a logic is the
check's. In an ordinary outcome: `"countermodel": {"sequent": {…},
"model": {…}}` (nested, no version), beside `derivation` and
`clausal_proof`.

**The level**, decided by `wire.rs`'s rules: the model is a new form,
so its document is at level 13 (`wire::KRIPKE`), as new forms took
levels 4, 5 and 8. The outcome's `countermodel` is a key a reader of
level 12 ignores without misreading the verdict, which the rules say
raises no level, so the outcome stays at 12. `core/tests/wire/level-13.txt`
holds a model and an outcome with one.

### The extraction per method

- **The SAT method** (intuitionistic, minimal): its model, the worlds
  of the last round ordered by inclusion (54s-a), now with the atoms by
  name and the root as the model's world.
- **The SAT method in classical logic**: the assignment, one world.
- **The embedding in classical logic**: the image's classical
  assignment, one world, checked against `Image::ordinary`. The
  embedding in intuitionistic and minimal logic refutes the image in
  linear logic (`Disproof`), which is no Kripke model: its "not valid"
  carries the image's refutation, as before, and no countermodel.
- **LJT** (`--method ljt`): no model (departs from the design, which
  asks for Pinto and Dyckhoff's extraction where LJT is the default; it
  no longer is). Pinto and Dyckhoff (*Loop-free construction of
  counter-models for intuitionistic propositional logic*, 1995) build a
  model from a failed search of their own calculus, a G4ip variant
  whose every failed node keeps the alternatives that failed, joined
  under a new root. deduxit's LJT keeps a failed sequent's verdict in
  its memo and nothing of why, refutes some sequents by failure
  subsumption (a stronger sequent failed before, whose model would have
  to be carried to the weaker one), and stops a `→→L` frame at its
  second premise's failure: the failed search it would read is not
  kept. Recording it is a change to the engine's search, a step of its
  own; since 54s-c2 the SAT method is the default and gives a model for
  every sequent it finds not valid, so LJT's "not valid" says "the
  search was exhaustive", and `check` answers it unknown (exit 3).

### The check and its bound

`Model::forcing` checks: at least one world, `world` among them; every
arrow to a world of the model and none going round (Kahn's algorithm on
the arrows, which also gives the order the table is filled in, every
world after the worlds above it); atoms and `falsity` growing along
every arrow, by name, so an atom the sequent does not name counts as
well; `falsity` in minimal logic only; one world in classical logic.
Then forcing of every formula at every world: an atom where the world
forces it, `⊥` where it forces `falsity`, `∧` and `∨` from their parts,
an implication, a negation (`¬A` as `A → ⊥`, so in minimal logic a
world forcing `⊥` forces `¬A`) and an equivalence where they hold there
and at every world directly above. `Forcing::holds(w)` is the sequent at
every world at or above `w`, `failing(w)` a world at or above `w` that
forces every hypothesis and not the conclusion, `at(w)` each formula's
forcing; `Model::refutes` asks the model's world to force Γ and not C
(classically none of Δ; `⊥` for an empty right side). An arrow written
twice is one. The stop is asked once per world in each of the three
passes (the atoms as indices, the arrows' check, the table), so no pass
runs long without it. `MODEL_CELLS` is 2²⁷: the cells (worlds plus
arrows, times the arena's formulas) are compared with it before the
table is made, and the table with a few words per world, arrow and atom
with `limits.memory_bytes`. `forcing` and `refutes` take `&Limits`, as
every long call of the library does. A model that fails is
`Error::Model` (kind invalid); `Error::Countermodel` is kept for an
engine's model that fails, a defect.

### The user's model

`check --logic L --sequent SEQUENT [--world N] FILE`, `FILE` the
model's JSON (the key `worlds` tells it from the other documents `check`
reads). `check` is the command that checks a claim read from a file
(proofs, refutations, outcomes), and a model is a claim about a
sequent, so it takes the sequent as a flag rather than a new subcommand
(the CLI's existing shape; `eliminate`, which flattens `check`'s
arguments, refuses the flags). It answers "the model satisfies S at
world N" (or "at every world"), exit 0, or "the model does not satisfy
S …: world V forces each formula left of ⊢ and not the one right of
it" with a line per subformula of what world V forces, exit 1; a model
of another logic is an error naming why, exit 2. JSON: `check`'s form
with `logic` and `failing: {"world", "forcing": [{"formula",
"forced"}]}`. The bindings' `ordinary_model(model, sequent, logic,
world, settings)` answers `{"holds", "failing"}` the same way.

### Beth models, weighed

On a finite tree, Beth forcing differs from Kripke forcing in `∨`: a
node forces `A ∨ B` when some bar above it has every node forcing `A`
or `B` (a root with two leaves forcing `p` and `q` forces `p ∨ q` in
Beth's sense, not in Kripke's). It is decidable there and could be a
second semantics of the same data restricted to trees. Not built
(departs from the design only in that the prompt allowed it): no engine
of deduxit produces Beth models, so the semantics would serve only a
model the user gives, whose editor is step 68's; and finite Kripke
models are already complete for intuitionistic logic (the finite model
property), so Beth models add no refutation. The shape when step 68
wants it: an `ordinary::Semantics::{Kripke, Beth}` argument to
`forcing`, Beth refusing a model that is not a tree, `∨` (and an atom,
by the covering condition) read through bars. An infinite fan needs a
finite presentation, and none is standard: it stays out.

### The panel and the reviewer

Three members, each from a fresh context, on the check and its
sources: `panel-counterexamples` (Opus 5.5, high), `panel-argument`
(Fable 5.2, high), `panel-integers` (Sonnet 5.5, high). None found a
wrong answer: the counterexamples member compared the check with a
reference of its own (Floyd–Warshall closure, forcing quantified over
every world above) on 260 000 random models and sequents in the three
logics, 91 325 runs of the SAT method (72 546 countermodels) and 10 022
of the classical default, with no mismatch; the argument member stated
the invariant (a model of the logic; the table equals forcing because
Kahn's order puts each world after the worlds above it; persistence
gives soundness, Kripke 1965, Segerberg 1968 for minimal logic) and
found it held at every site. What they found, and what became of it:

- **A "not valid" lost as an error** (argument 1, integers R3): the SAT
  method's pre-test counted worlds times formulas, the check worlds and
  arrows, so a model in the band between was built and its check's
  memory refusal left `sat::decide` through `?`. Every engine's model now
  goes through `Countermodel::found`, which drops a model the check
  refuses for its size and keeps the verdict; the classical paths had
  no pre-test at all and go through it too. Fixed.
- **The classical default dropped the solver's model** where the
  embedding's refutation was not an assignment (argument 2, `prove
  --classical-work 0 'a |- b'`): `with_evidence` and the command's JSON
  (`ordinary::solver_model`) now take the solver's. Fixed.
- **`check` did not read an embedding outcome's countermodel**
  (argument 3: a forged model beside a true refutation passed): it is
  now checked first, and a wrong one is the answer. Fixed.
- **Two passes ran unpolled** (all three: the subset test per arrow,
  quadratic with duplicate arrows, a minute under `--timeout 1s`; "at
  every world" in `check` and the bindings, a search per world,
  quadratic, 178 s on a chain of 100 000 worlds). The check now asks the
  stop once per world in each of its three passes, reads an arrow
  written twice as one, looks a world's sorted atom indices up in each
  world directly above, and `Forcing::failing_anywhere` scans `fails`.
  Fixed.
- **The check's memory beside the table was not bounded** (integers R4:
  110 bytes a world, 35 an atom, gigabytes at the cell bound): `forcing`
  and `refutes` now take `&Limits`, as every long call does, and refuse
  past `limits.memory_bytes` with the table and a few words per world,
  arrow and atom (`WORLD_BYTES`, `ATOM_BYTES`). Fixed.
- **Error kinds** (argument 6): a model that fails is now `Error::Model`
  (kind invalid, code `invalid_model`), so a wrong file is no defect;
  `Error::Countermodel` stays the defect of an engine's model. Two
  formulas right of `⊢` are an error in `check --sequent` (exit 2) and
  an invalid refutation in an outcome, no longer "unknown". Fixed.
- **The manual's negation sentence** was wrong in minimal logic
  (argument 5): rewritten. Fixed.
- **`Forcing::forces` past the last world** could read the last word's
  padding (argument 7): it now asserts the world, as its `# Panics`
  says. Kahn's count is a `usize` (argument 8, integers). Fixed.
- **A misspelt key of a model document is ignored** (counterexamples'
  note): kept, since the wire rules let a reader ignore a key it does
  not know, so that a later level may add one.

Then `fresh-reviewer` once (Fable 5.2, medium), on the four changes
then made: the check sound as far as it could break it (every branch
read, thirteen edge cases run), the level decision right by `wire.rs`'s
rule, and three findings, all fixed in `vyzvoxok`: the guide gave
`forcing` and `refutes` without `&Limits`; the rules still named the
deleted `sat/model.rs` and not `ordinary/model.rs`; and the SAT method's
order of worlds charged to its memory account before the check
(`kripke`, n²/8 bytes) still turned a "not valid" into
`Unknown(MemoryLimit)` past the account, now a "not valid" without its
model. Its question, not a finding: a misspelt key of a model (`above`
for `arrows`) is ignored as the wire rules say, so a hand-written model
can be read as other worlds than meant and answered quietly; whether
the model document should refuse unknown keys is for the author (open,
below). A stop during an engine's model check is the stop (documented
on `Countermodel::found`), where the embedding's path drops the model.

### The checks

- `cargo clippy --workspace --all-targets -- --deny warnings` and
  `cargo test --workspace`: pass at `sskymumt` (cores 4 to 6), and
  clippy, the SAT tests and the guide's test again at `vyzvoxok`.
- The gate (`nix develop -c gate`, cores 4 to 6): passed at `sskymumt`;
  `vyzvoxok` after it changes one arm of `sat::run`, the guide and a
  rules file.
- Every family at 5 s (`deduxit-bench run --all-families --timeout 5`):
  195 rows, no `MISMATCH`.
- The ILTP agreement tests (`cargo test -p deduxit --release --test iltp
  -- --ignored`): 3 passed, 774 s.
- The panel's witnesses rerun at `sskymumt`: the chain of 100 000 worlds
  answers in 180 ms (was 178 s), the world of 30 000 atoms and 30 000
  copies of an arrow in 63 ms (was 57 s), the forged embedding outcome
  is an invalid refutation (exit 1, was valid), `prove --classical-work
  0 'a |- b'` writes the solver's model in its JSON, the band witness
  `win.txt` under `--method sat` is "not valid … too large to show and
  check", exit 1 (was exit 2), and a wrong model in an outcome writes
  `invalid_model`.
- `nix flake check` (under the machine lock, `--max-jobs 4 --cores 8`)
  at the head rebased onto `main@origin` `zpqkxmzo`: passed, the
  bindings' tests under Node (`web-wasm-tests`) among its checks; this
  line is the only change after it.

### Departures from the design

- LJT's "not valid" gets no model (departs from the design): why above.
- Beth models are weighed, not built (departs from the design in what
  the prompt left open): why above.
- A model's worlds form a partial order: arrows that go round are
  refused, not read as a preorder of equivalent worlds (departs from the
  design's "a preorder"): a preorder's model is equivalent to its
  quotient, whose worlds are the classes, and the user writes one world
  for each class.
- `ordinary::sat::Model` and `World` are gone, replaced by the shared
  `kripke::Model` with atoms by name (no release had them).

### What is open

- Whether a model document should refuse a key it does not know: the
  wire rules let every reader ignore one, so a misspelt `arrows` reads
  as worlds without arrows and a hand-written model is answered
  quietly (the reviewer's question). For the author.
- The embedding's text output under the classical default names the
  image's refutation; where that is no assignment (`--classical-work 0`)
  the solver's model is in the JSON only.

### What the next sessions must know

- **55c (the lemma and the exporter)**: the certificate is of an
  `ordinary::Countermodel` (sequent and model, checked): the library's
  lemma takes a finite model (worlds, the arrows' closure, the
  valuation, `falsity` for minimal logic) and a world forcing Γ and not
  C. The order is the reflexive transitive closure of the arrows, which
  are acyclic; forcing is the table `Model::forcing` computes, which the
  exporter can write as data the lemma's checker recomputes. In
  classical logic the model is one world (the assignment).
- **55d (the drawing and the page)**: draw `kripke::Model` (worlds as
  nodes with their atoms, the arrows as the order, `world` marked); its
  `Display` is the text form the command prints. The page reads it in
  the ordinary outcome's `countermodel` and checks a model with the
  bindings' `ordinary_model` (`{"holds", "failing"}`).
- **Step 65 (modal logics)**: reuse `kripke::Model` as data (worlds,
  arrows as accessibility, atoms by name, `world`) and write the modal
  check beside `ordinary/model.rs`, with the frame conditions in place
  of ordinary logic's partial order and monotone valuation. Its JSON
  (level 13) needs no change; a frame condition the model claims would
  be a new key.

## Session 55c: the Kripke lemma and the exporter

Opus 5.5 at `high`, a reused worker (brief `start-55c.md`), in the jj
workspace `../deduxit-wt-step-58a-610ed3` and, for the library,
`../rocq-deduxit-55c` (workspace `step55c` of `rocq-deduxit`), on
2026-11-15, cores 7 to 9.

### Where it stands

The library's half is done and checked; the exporter's half waits for
55b on `main@origin`.

### The library's half (`rocq-deduxit`)

Two commits on its `main@origin` (`sqmpzpzx`): `womvrznp` (the
calculus, the semantics, the checker, the tests, the check's second
loop) and `xmsnqtwk` (README, changelog's Unreleased, opam
description and keywords, CLAUDE.md, `rocq.md`). Not pushed: the
planning session pushes them (`plan/push-main.sh --rocq`).

- **A namespace of its own**, `Deduxit.Intuitionistic`
  (`theories/Intuitionistic/`), as the library's shape of 2026-11-09
  names it; nothing shared with `Deduxit.Linear`, so no `Deduxit.Base`.
- **The trusted base**: `Formulas.v` (`var`, `bot`, `wedge`, `vee`,
  `imp`; `top := imp bot bot`, `neg A := imp A bot`, `equiv A B := wedge
  (imp A B) (imp B A)`, as Troelstra and Schwichtenberg define them)
  and `Calculus.v` (`rules` with `explosion` and `cut`;
  `intuitionistic` = LJ, `minimal` = LM; `lj R Γ C`). `lj` is G1i with
  shared contexts: axiom on any formula, exchange as one `Permutation`
  rule, weakening, contraction, `bot_l` under `explosion` (G1i's
  weakening on the right, an empty succedent being `⊥`), the
  connectives' rules on the first hypothesis, `cut_r` under `cut`. LM
  is the same without `bot_l`: `⊥` is one more atom. The larger
  calculus on purpose: a refutation of it covers every smaller one.
- **The lemma** (`Kripke.v`): `model R` over any `world` type with a
  `Prop` preorder `le`, a monotone valuation `val` and monotone worlds
  `absurd` forcing `⊥`, empty under `explosion`; `forces` the textbook
  recursion. `persistence`; `kripke_soundness : lj R G C -> forall w,
  Forall (forces w) G -> forces w C` for every `R`, cut included;
  `kripke_countermodel : Forall (forces w) G -> ~ forces w C -> ~ lj R
  G C`. The corollary `classical_refutation : forall R v G C, forallb
  (eval v) G && negb (eval v C) = true -> ~ lj R G C`, the one-world
  model over `unit`.
- **The checker and its theorem** (`KripkeCheck.v`): the data
  `finite_model atom = {| later : list (list nat); atoms : list (list
  atom); absurd : list nat |}`, worlds `0 … length later − 1`, `later
  w` the worlds later than `w` (reflexive pairs implied: `ups w = w ::
  later[w]`), `atoms w` the atoms `w` forces (missing = none), `absurd`
  the worlds forcing `⊥`. `well_formed R` checks transitivity, that
  later worlds are worlds, monotone atoms and `⊥`, and `absurd = []`
  under `explosion`. `forced A` is the vector of worlds forcing `A`,
  bottom up (`O(|A|·n²)` with `nth`). `refutes atom_eqb M R w G C`
  asks `well_formed`, `w < size`, every `G` forced at `w` and `C` not.
  `kripke_refutation : forall atom_eqb, (forall a b, reflect (a = b)
  (atom_eqb a b)) -> forall R M w G C, refutes atom_eqb M R w G C =
  true -> ~ lj R G C`.
- **The certificate's form** the exporter will write:
  `Lemma … : ~ lj intuitionistic [Γ] C.` (or `minimal`), `Proof. apply
  (kripke_refutation String.eqb String.eqb_spec intuitionistic {| later
  := [[1]; []]; atoms := [[]; ["a"]]; absurd := [] |} 0). vm_compute.
  reflexivity. Qed.`, under `From Deduxit.Intuitionistic Require Import
  KripkeCheck.`; a sequent with no atom writes `(atom := string)`.
- **Assumptions**: `tests/Assumptions.v` names the four theorems by
  their full paths; each answers `Closed under the global context` (37
  lines, 33 before).
- **Tests** (`tests/Intuitionistic/Kripke.v`, compiled as
  `Deduxit.Intuitionistic.Tests` by a second loop of the check):
  excluded middle, double negation (with cut), Peirce, `a ↔ ¬¬a` beside
  `⊤`, `a → b ∨ c ⊢ (a → b) ∨ (a → c)` (three worlds), minimal `a, ¬a ⊢
  b` and `¬a ⊢ b` (a later world forcing `⊥`), the one-world corollary,
  a sequent with no atom; and `refutes … = false` for a world forcing
  `⊥` in LJ, an intransitive order, a forgotten atom, a forgotten `⊥`,
  a later world past the worlds, a refuting world past them, a world
  forcing the conclusion, a hypothesis not forced.
- **Checks**: the library's `nix flake check` passes at `xmsnqtwk`
  (every admitted Rocq, 9.0 to 9.3; conventions, typos, treefmt).
