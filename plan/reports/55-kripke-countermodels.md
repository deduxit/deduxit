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
