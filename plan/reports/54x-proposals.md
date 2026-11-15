# Step 54x: the held proposals

## Session 54x: the held proposals

Session 54x (Opus 5.5 at `high`, 2026-11-14, cores 10-12), in the jj
workspace `../deduxit-wt-jovial-jones-32763f`, from the brief
`/tmp/linlog-handoff/start-54x.md`. The nine proposals the step
sessions of 2026-11-14 held for the author, which the planning session
accepted, are built, each a commit of its own (two of them with a
behaviour-lock commit after them), on `main@origin` (`umxtwmuk`) since
the rebase at the end. Nothing was pushed.

| change | what |
|---|---|
| `pnzntuur` | 1: `Family::instance` refuses an index past `instances` (`Error::FamilyIndex`) |
| `nyztpokp` | 2: `mist::read` refuses more antecedents than a `u32` counts (`Refusal::Index`) |
| `yqqwyvkt` | 3: `with_lambda` on `latex::Options`, `typst::Options` and `svg::Style` |
| `tupvqnmo` | 4: `Settings::deterministic()`, which `prove --deterministic` applies |
| `nzyrxuok` | 5: `StepError::Rule` carries the formula's text |
| `qkwxrpqo` | 6: the intuitionistic count refutation names the side with no literal |
| `stlvutno` | the behaviour lock of 6 |
| `qyowyxtw` | 7: four of the command's texts |
| `rtkxspvy` | the behaviour lock of 7 |
| `yyokxtlz` | 8: the harness's strings |
| `uymvslvr` | 9: `rocq.nix` against the app's NanoYalla; `packages.check-certificate` |
| `ympppsxt` | this report |

### Each item

1. **`Family::instance` refuses an index from `instances` on**, with the
   new `Error::FamilyIndex { family, index, instances }` (code
   `family_index`, kind malformed, feature `parse`), a sibling of
   `Error::FamilySize`: the two say different things (a size the family
   has no instance of, an index past its shapes), and a sibling keeps
   `FamilySize`'s fields, which a program may read. The check comes
   after the size's, before the generator runs. `deduxit-bench one
   --problem family:NAME:SIZE:INDEX` goes through `Family::instance`
   (`problems::load`), so it refuses the index too. Test: the size
   test (`refuses_sizes_without_instances`) gained `contraction` at
   index `instances`; the errors' sample list and `CODES` have the new
   variant, and `serialize/errors.rs` writes its fields (one arm, the
   54s-b2 file's only line changed by this session).
2. **`mist::read`'s antecedents** go through a private
   `antecedents(roots) -> Result<u32, Error>`, `u32::try_from` with
   `Refusal::Index { what: Space::Member, count, most: u32::MAX }`.
   The input that reaches it needs about four billion roots, which no
   test can build, so the test calls the function at `u32::MAX + 1` and
   `+ 2` roots (64-bit targets only, where `usize` holds them).
3. **`with_lambda`** on the three options values, `#[must_use]`, as
   every other field has its `with_*`. Additive; no test, as the other
   builders have none.
4. **`Settings::deterministic(self) -> Self`** (`#[must_use]`), beside
   `Settings::stop`: one job, `Schedule::Turns`, no pool
   (`clock.pool_after_ms` `None`), the rest kept. The name is the
   flag's, as a consuming builder method of this kind is named in
   current Rust (an adjective that reads `Settings::default()
   .deterministic()`); there is no field to call it `with_`. The command
   applies it for `--deterministic` (`cli/src/settings.rs`), so the two
   cannot drift. The crate root's doctest, the manual's library chapter
   (a copy of that doctest) and the guide's reference §16 bullet use it
   now. Test: `deterministic_keeps_one_thread`, and its doctest.
5. **`StepError::Rule { rule, position, formula }`**: `formula` is a
   `Box<str>`, the formula as the session writes it
   (`Interactive::formula`: under the reading in an intuitionistic
   mode, an instance with its terms in a session with quantifiers), cut
   after 4 096 characters with `…`, as `ShapeError::Formula` carries its
   text. `Display` says it: `the rule ⅋ does not act on formula 1, A ⊗
   ~B`. `expand` makes it from the occurrence; `apply`, the call front
   ends meet, makes it from the goal's member in a session with
   quantifiers, where the occurrence alone would print the open
   formula. The command's FC6 message now reads `{error}; `rules G P`
   lists the rules that do`, word for word what it printed before. The
   bindings' error JSON has `details.formula` by the variant's derived
   serde; `a_refused_step_says_why` in `web/tests/calls.rs` checks it
   (four lines added to a 54s-b2 file). Not a break: the variant is
   `#[non_exhaustive]`, so no pattern outside the crate names all its
   fields and no struct expression builds it (`breaks 0.3.0` lists it under the added items only).
6. **The intuitionistic count refutation names the empty side**: where
   the side with fewer literals has none, `A on the right has no A on
   the left to meet in an axiom (an implication's antecedent counts on
   the other side)`. `placed` computes it, as for the two-sided
   classical sequent: under the reading an atom in output position is
   the positive literal, so a literal's sign is its side. Only for a
   sequent refuted as a whole (`Sides::Intuitionistic { whole }`): a
   session's goal is not the sequent's roots. The note on implications
   stays, as in the counted phrase, since without a reading the
   phrase cannot tell whether a literal crossed. Test:
   `an_intuitionistic_count_names_the_empty_side`; the lock's
   `prove-intuitionistic-unprovable` and the manual's `--ill-syntax
   one-sided` example changed.
7. **The command's texts**: every `--settings` help (prove, check,
   interact, seq print) names the eleven top-level keys, and
   `the_settings_help_names_every_key` holds each to `Settings`' JSON
   keys; `--engine`'s help says the inverse method searches alone when
   named, takes turns without `--engine` on an intuitionistic goal with
   an exponential and on a goal under Mix, and takes over on some MLL
   sequents without Mix at the focus engine's memory bound;
   `NetError::NotMell` names the units where they alone stand in the
   way (`proof nets with boxes exist only for MELL without units, and
   the sequent has a unit`), tested in `intuitionistic_nets`; and both
   sessions say `"x" is not a goal: a goal is a number, and `goals`
   lists them` without Rust's parse error, tested in
   `interactive_session` and `ordinary_session`.
8. **The harness's strings**: the ranking's contradictions heading
   (deduxit's proofs and refutations are checked but for an exhausted
   search; Sympli checks its proofs; no other verdict is checked) and
   its legend, whose `wrong` now lists the three cases `judge` marks,
   in shorter sentences; `summary`'s `1 run failed`; `compare`'s legend
   (`crash` is any crash, `killed` also while loading, and `error`,
   which was missing); the `batch-families` journey's summary (one
   sequent per worker, each on one thread, which `Cores::Across` is;
   one line of a 54s-b2 file); and the six longest `summary` and
   `claim` strings of the families (`chain`, `lambek-fo`,
   `contraction`, `wide-m1`, `euler`, `reach-fo`), in shorter
   sentences with the same content. No published report was
   regenerated; the journeys' code did not change, so no ceiling moved.
9. **The flake**: `check-certificate.nix` hands its NanoYalla library to
   `rocq.nix` as the module argument `nanoyalla` (`_module.args`, the
   repository's way of sharing between perSystem modules, as
   `toolchain.nix` does), and the `rocq` check compiles every
   certificate with that library on `ROCQPATH` instead of compiling its
   own copy, so the kernel is built once. `packages.check-certificate`
   is the app's program, for `nix shell` and `nix profile install`; the
   manual's Rocq chapter and the changelog name the install.

### The checks

- `cargo clippy --workspace --all-targets -- --deny warnings` and the
  tests of each area after each item, capped on cores 10-12.
- The gate (`nix develop -c gate`, capped on cores 10-12) at the chain
  before the reviewer's two fixes: passed (clippy, the tests with the
  behaviour lock, both `cargo hack` runs, the ratchet; no ceiling
  moved, the misses of `search-qbf-20-2`, `search-chain-128` and
  `check-qbf-20-2` moved by more than 5 %, which the ratchet names and
  never fails on). After the fixes: `cargo clippy --workspace`, the
  session tests, `core/tests/first_order.rs`, every test of
  `deduxit-cli` (the lock blessed for the help's one line) and
  `web/tests/calls.rs`, all passing; `nix flake check` (below) runs the
  rest at the last commit.
- `cargo run --release -p deduxit-bench -- run --all-families --timeout
  5` and `summary`: 195 runs, `mismatch` 0 in every family, no verdicts
  that disagree.
- `nix build .#checks.x86_64-linux.rocq
  .#checks.x86_64-linux.check-certificate
  .#packages.x86_64-linux.check-certificate` under the machine lock:
  all three built.
- `breaks 0.3.0`: cargo-semver-checks fails three checks, all ordinary
  logic's from earlier steps (`ordinary::Rule::Cut`, the moved
  `Rule::Open`, `Outcome`'s removed `translation` and `linear`); the
  public API's diff lists this session's items as additions only
  (`with_lambda` three times, `StepError::Rule::formula`).
- `nix flake check` at the last commit, under the machine lock:
  "all checks passed!" (x86_64-linux; the other systems are omitted
  as always).

### The reviewer

`fresh-reviewer` (Fable 5.2 at `medium`) read the chain, the brief and
the code, probed with the debug binaries, and found nothing that breaks
correctness or the brief. Its three points:
1. **`--engine`'s help overstated the turns**: they come on one thread
   only, and under Mix only without weakening (`Modes::Mix` is
   `!affine`). Held; the help says both now (folded into `qyowyxtw`,
   its lock line into `rtkxspvy`).
2. **`rule_refused` wrote the whole formula and cut it afterwards**
   (`Interactive::formula` builds a `String`, an instance's terms
   uncut), so the 4 096-character cap bounded the error's text but not
   the work. Held; it now writes through the error writer's budget
   (`errors::describe::cut` with the session's `Names`: forest,
   reading and framing), which stops at the cap, as `ShapeError`'s
   formula does (folded into `nzyrxuok`). Probed by hand: a first-order
   refusal names `~p(a) ⅋ ~q(a)` for a given witness and `~p($1) ⅋
   ~q($1)` for an open one.
3. **A refusal raised while a first-order session file is replayed**
   names the occurrence's open formula, since only `apply` swaps in the
   goal's instance. Left: only a session file whose rule does not fit
   its formula reaches it, and the reader then refuses the file with
   that step's error (`Error::Step`); recorded below.

### Departures

- Item 5's `breaks` ran against 0.3.0, the last release, where the brief
  names 0.1.0: the changelog's Unreleased section is measured against
  0.3.0, and a diff from 0.1.0 would mix two releases' breaks in (departs
  from the design). Nothing of this session breaks a user, so "What
  breaks" gained no line.
- Item 7 changed the goal's message only, as the brief names it; the
  linear session's positions keep Rust's parse error (below).
- Two files of 54s-b2's were touched, one place each:
  `core/src/serialize/errors.rs` (the arm of `FamilyIndex`) and
  `web/tests/calls.rs` (four lines in `a_refused_step_says_why`); one
  line of `bench/src/journeys.rs` (the journey's summary).

### What the next sessions must know

- The linear session's positions still answer `"x" is not a position:
  invalid digit found in string` (Rust's parse error), the same defect
  item 7 fixed for goals; the brief named goals only. A follow-up.
- `compare`'s "Of the 1 problems both decide alike" has the plural
  defect `summary`'s "1 runs failed" had.
- `StepError::Rule`'s `formula` is filled wherever the error is made:
  a new site that refuses a rule calls `Interactive::rule_refused`
  (`expand.rs`), never builds the variant itself.
- A refusal raised while a first-order session file is replayed
  (`replay.rs`) carries the occurrence's open formula, not the
  instance's: only `apply` writes the goal's member.
- `Settings::deterministic` is where `--deterministic`'s meaning lives;
  a change of what it sets changes the command and every library
  caller at once.

## Session 54x2: four small follow-ups

Session 54x2 (Opus 5.5 at `high`, 2026-11-15, cores 10-12), in the jj
workspace `../deduxit-wt-step-58b-8c0bbc` (a reused worker), from the
brief `/tmp/linlog-handoff/start-54x2.md`. Four follow-ups the reports
of 54x and 58b left, each a commit of its own, items 3 and 4 each
followed by their export snapshots and their behaviour-lock lines in
commits of their own. Nothing was pushed.

| change | what |
|---|---|
| `xnlspnvz` | 1: the linear session says what a position is |
| `ltzttrpt` | 2: `compare`'s "1 problem … 1 differs" |
| `ulwnqzrm` | 3: a derivation's `<desc>` in the title's notation |
| `kwkzlovp` | the export snapshots of 3 |
| `swurvkqo` | the behaviour lock of 3 |
| `qytskkxu` | 4: a net's `<desc>` in sentences |
| `tysppmzm` | the export snapshots of 4 |
| `pswmostn` | the behaviour lock of 4 |
| `nsvntkxq` | the render-svg ceiling lowered |
| `llpsqton` | this report |

### Each item

1. **The linear session's positions**: `positions` and `position` in
   `cli/src/interact/mod.rs` read a word through one closure,
   `read_position`, which answers `"x" is not a position: a position is
   a number, and `goals` lists them`, the words 54x gave goals, without
   Rust's parse error. The cut's hint for an unquoted formula keeps its
   context in front (`quote a cut formula that has blanks: … "&" is not
   a position: …`). Test: `interactive_session` gained `rules 0 y`, and
   `an_unquoted_cut_formula_is_explained` reads the new words; the
   changelog's line on the session's refusals names both.
2. **`compare`'s plural**: "Of the 1 problem both decide alike on one
   thread, 1 differs in …", both nouns and the verb by their counts.
   Test: `bench/tests/compare.rs`, whose two outputs have one problem
   each, now expects the singular.
3. **A derivation's `<desc>` in the drawing's notation**:
   `proofs::fmt::write_steps` takes the sequent's writer as a closure;
   the public `write_steps` of both derivations pass the text tree's
   (`~A`, unchanged), and the SVG tree passes `Drawn::sequent` with
   `PLAIN`, the call that writes the `<title>`, so a negated atom is
   `A⊥` in both, a two-sided derivation stays two-sided, and an instance
   reads `p(a, b)⊥`. The numbering and the words (`3. …, by ⊗ from 1 and
   2.`) are as they were. Test:
   `the_description_writes_formulas_as_the_title_does`; nine export
   snapshots and the lock's `format-svg` (and `format-png`, whose PNG
   carries the description) changed only in `~A` to `A⊥`.
4. **A net's `<desc>` in sentences**: `net::describe` and
   `mell::describe`, with the shared `net_conclusion`, `net_formula`,
   `net_link` and `net_verdict` in `svg/mod.rs`, write one sentence a
   line: `It concludes ⊢ A⊥ ⅋ B⊥, B ⊗ A.`, `An axiom link joins formula
   1, A⊥, and formula 5, A.` (a cut link `A cut link joins …`), for a
   net with boxes `Box 6 holds formula 7, a ⊗ a, the premise of formula
   6, !(a ⊗ a).` (` It lies in box 3.` when nested) and `Formula 0, ?a⊥,
   has 2 copies: formula 3, a⊥, in box 6; formula 4, a⊥, in box 6.` or
   `…, has no copy and hangs on the axiom link of formula 11, a.`, then
   `It is a proof net: the correctness criterion holds.` (with Mix, or
   planar, in the words of the text form). A formula is named by its
   vertex, the text form's number: in MLL that is the occurrence the
   drawing's id `o<n>` names; a box is named by its `!` vertex, the
   drawing's `b<k>`. Formulas are in `PLAIN`, under the reading in an
   essential drawing. The verdict stated is `is_correct`'s, as the text
   form's was. The drawing's estimate no longer counts the description:
   it refuses before the criterion runs, as before, and
   `described_bytes` counts the description after the criterion, against
   what the bound leaves (`within`, `described_within`). Tests:
   `a_net_is_described_in_sentences` (the whole description of `A ⊗ B ⊢
   B ⊗ A`'s net) and `crossing_links_are_described_as_no_net` (the
   reviewer's witness, below); 38 export snapshots and the lock's
   `format-net-svg` changed in the description alone.

The web client's tests read the description in two places
(`tests/prover.spec.ts`, `tests/lessons.spec.ts` of
`deduxit.github.io`): a net's `desc` contains `proof net`, which the
verdict sentence keeps, and a derivation's contains `by ⅋ from` and
`A ⊗ B ⊢ B ⊗ A, by ⊗L from`, which item 3 keeps (no negated atom in
either). The rule labels' numbering is the steps', unchanged. The
bindings' own tests (`web/tests/`) read no description.

### The checks

- `cargo clippy --workspace --all-targets -- --deny warnings` and the
  tests of each area after each item, capped on cores 10-12.
- The gate (`nix develop -c gate`, capped on cores 10-12) three times:
  on the chain before the reviewer's fixes, on the chain with them
  rebased onto `xxrrnlxl`, and rebased onto 55a's `kquywyxw`: passed
  each time (clippy, the tests with the behaviour lock, both `cargo
  hack` runs, the ratchet). The last rebase, onto `ntxxpkty`, brought a
  report file alone. The
  ratchet named render-svg 6.61 % below its ceiling (78 295 779 against
  83 835 448), the same count both times, and the misses of
  `check-qbf-20-2`, which it never fails on. The only code on
  render-svg's path that differs from `main@origin` is item 3, whose
  description writes a sequent into one string and escapes it once
  where every piece went through the escaping writer apart; `ratchet
  --lower` lowered that ceiling alone (`nsvntkxq`).
- `cargo run --release -p deduxit-bench -- run --all-families --timeout
  5` and `summary`: 195 runs, 26 rows, `mismatch` 0 in every row.
- `nix flake check` on the chain over `kquywyxw`, under the machine
  lock: "all checks passed!" (x86_64-linux; the other systems are
  omitted as always).

### The reviewer

`fresh-reviewer` (Fable 5.2 at `medium`) read the chain, the brief and
the code, and reproduced two findings, both in item 4; items 1 to 3 and
the snapshot and lock commits it found sound.
1. **A net with crossing links was described as planar.** Under
   `Style::essential` the verdict drawn is `is_essential`'s, which does
   not test planarity, and the description stated that verdict: a cyclic
   structure `a, b ⊢ b ⊗ a` with crossing links read "It is a planar
   proof net: … no two links cross". The text form it replaced ran
   `is_correct`. Held: the description states `is_correct`'s verdict
   always (run beside the drawn one under the reading, only when the
   style has a description); `crossing_links_are_described_as_no_net`
   pins the witness. The essential snapshot `essential-around.svg` says
   the switching cycle again, as on `main`.
2. **The criterion ran before the size refusal**, which I had moved so
   that the estimate could count the description, under a comment that
   called it linear; `is_essential` is quadratic (0.14 ms at 200
   implications, 30.5 ms at 3 200, all spent before a refusal that came
   from the estimate alone). Held: the estimate refuses before the
   criterion again, without the description, and `described_bytes`
   counts the description after it against what the bound leaves.
Both fixes are folded into `qytskkxu` and `tysppmzm`.

### Departures

- Item 3 is broader than the brief's `fmt.rs`: the shared
  `proofs::fmt::write_steps` takes the sequent's writer, so the SVG
  writes the title's notation while the public `write_steps` of both
  derivations keep the text tree's `~A` (a library call that asks for
  the text keeps getting it).
- Item 4 names a formula by its vertex, the number the net's text form
  and `--net` print. In MLL that is the occurrence of the drawing's id
  `o<n>`; in a net with boxes the drawing's id is `o<n>` by occurrence
  where an occurrence has one vertex and `v<k>` by vertex otherwise, so
  the description's number is the drawing's only in the second case
  (departs from the design: "numbered as the drawing numbers them").
  Boxes are the drawing's `b<k>` exactly.
- The order of a net's description is conclusion, cut links, axiom
  links, boxes, then each `?` formula's copies: the brief names the
  first three; the copies are what the text form had besides.

### What the next sessions must know

- A non-net's reason in a net's description is `describe`'s, which
  writes `~A` and brackets ids (`A ⊗ ~A[0]`): `Described` has no
  notation of its own. A notation for it would go through
  `errors::describe::Names` (four constructors) to `occurrence`. The
  command and the bindings draw only nets of checked proofs, so only a
  library caller drawing a structure it built reaches it.
- Under `Style::essential` an MLL net's description states
  `is_correct`'s verdict while the highlight is `is_essential`'s; the
  two agree on whether it is a net but not on the witness
  (`essential-around.svg`: the description names a switching cycle,
  the drawing a path around an implication).
- The net's description is counted after the criterion
  (`described_bytes`); the drawing's estimate leaves it out. A new
  sentence needs nothing more: it is counted by writing it.
- render-svg's ceiling is 78 295 779 now; a chain stacked with this one
  that moves the SVG writer compares against that.

## Session 54x3: small follow-ups before 0.4.0

Opus 5.5 at `high`, the worker of 55c and 55d (same conversation), in
the jj workspace `../deduxit-wt-step-58a-610ed3`, from the brief
`start-54x3.md`, on 2026-11-15, cores 7 to 9.

### The items

1. **`check --table-atoms N|none`** (`qkzpvqqz`, the lock's help file in
   `zykskkvk`): `CheckArgs` takes the flag `prove` has, the same key
   `limits.table_atoms`, so a table `prove --table-atoms 11` made is
   checked with `check --table-atoms 11` (a settings file is no longer
   needed). `eliminate`, which flattens `CheckArgs`, takes it too and
   reads no table: harmless. Tests: `check_takes_the_table_bound` (a
   table of two atoms refused under one, checked under two) and the
   `CHECK` row of `each_flag_overrides_its_key_in_a_file`. The manual's
   ordinary chapter, the guide and the changelog's table entry say so
   (lines of 59's files: `CHANGELOG.md`'s truth-table bullet,
   `skills/deduxit/reference.md`'s `table` bullet,
   `docs/src/ordinary-logic.md`'s bound paragraph).
2. **The wire error's article** (`pnkskkyl`): `Error::Json` writes "not
   an ordinary outcome", "not a proof", the article from the form's
   first letter (`article`). Test: `a_wire_error_takes_its_article`. No
   lock, manual or guide text carried the old wording.
3. **`translate`'s antecedents** (`lmtumvuq`): `u32::try_from` and
   `Refusal::Index { what: Space::Member, most: u32::MAX }`, as
   `mist::antecedents` does; `translate`'s `# Errors` names it. Test:
   `the_antecedents_fit_32_bits` (its 64-bit half behind a `cfg`).
4. **`inference_bytes`'s 2** (`txrnnypz`): right as it is. The comment
   says why: `Γ ⊢ Δ` writes one separator fewer than formulas per side
   and ` ⊢ ` between, so two per formula plus two is never below the
   sequent written, whichever side is empty (the reviewer checked it
   against `Notation::ordinary`: 1 to 3 characters above).
5. **`sat::decide`'s stop** (`qpwswvvm`): its doc lists every place:
   every call of the solver and every decision in it; the scan for an
   unrealised implication and the ordering of a model's worlds, every
   256; the model's check, once per world per pass; the read-back, LJT
   and the derivation's check; in classical logic the encoding and the
   handing of clauses to the solver, every 4 096, and the clausal
   proof's check.

### The reviewer

`fresh-reviewer` (Fable 5.2 at medium): one finding, fixed in
`qpwswvvm`: the classical path's handing of clauses to the solver
(every 4 096) was missing from the list, and "between two of its
calls" understated the poll at the start of every call. Its note,
`cli.md`'s `--table-atoms` "on `prove`", is fixed in `qkzpvqqz`.

### The checks

- The gate passed (cores 7 to 9) before the reviewer's two doc and
  rules lines; `cargo doc --document-private-items --all-features` with
  warnings denied clean.
- `nix flake check` (under the machine lock, `--max-jobs 4 --cores 8`)
  at `mrrpnlyk` on `main@origin` `zpuvxkpt`: all checks passed. A first
  run failed `wasm`: the test of item 3 imported for its 64-bit half
  alone, unused on 32 bits; the imports moved inside that half
  (`lmtumvuq`). This line is the only change after it.

### What the next sessions must know

- A new flag that sets a limit a document is read within belongs on
  `check` as on `prove` (`CheckArgs::settings` and the `CHECK` row of
  the settings test), or the document `prove` writes under a raised
  bound cannot be checked without a settings file.
- A new wire form's name needs nothing for its article: `article`
  reads the first letter (a name sounded otherwise than written, "an
  hour", would need a case there).
