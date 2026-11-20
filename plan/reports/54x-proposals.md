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

## Session 54x4: three recurring loose ends

Opus 5.5 at `high`, the reused worker of 54s-c2 to 63t-b (same
conversation), in the jj workspace `../deduxit-wt-step-54s-c1-161790`
and the client's `../deduxit.github.io-63tb`, from the brief
`start-54x4.md`, on 2026-11-15, cores 4 to 6.

The chain: deduxit `ywtsxtrt` (item 2) and `qropoqnt` (item 3) on
`main@origin` `sxwwwyql`, this report after them; the client
`kvwnuzlv` (item 1) on its `main@origin` `uxuxvlps`. Items 2 and 3 were
measured before they were fixed, and in both the cause was elsewhere
than the brief supposed; each fix is at the cause found.

### The items

1. **The flaky client test** (`kvwnuzlv`). Cause, in the test: the
   failure's log (the 63t-b flake check) shows `before` holding both
   open goals of `∧R` (`a, b ⊢ a` and `a, b ⊢ b`, joined by the space),
   so it was read off the drawing before the `ax` step's: `rule()` in
   `tests/prover.ts` returns once the rule is pressed, and nothing
   waited for the step's drawing; the page drew the right goal (the
   failure's `received` is it), but `before` never matched. Unloaded,
   the drawing always won (a probe read one open goal right after the
   click in 50 of 50 runs, throttled or not), so the race shows only
   under the flake check's load. Fix: wait for one `g.open-goal` before
   reading (the goal's buttons are laid only over the current
   session's drawing, so the `a, formula 3` button clicked exists over
   `∧R`'s drawing alone, and one open goal means the step is drawn).
   `tests/cut.spec.ts` ("a replayed classical cut…") had the same
   read after `cut()`, which returns when the chooser closes; it now
   waits for the second premise's button, which only the cut's drawing
   has. Shown: before the fix 40 runs (`CI=1`, one core, four workers)
   and 80 (one core, eight workers) passed, since the race needs the
   whole suite's load; after it, both tests 80 times each (one core,
   eight workers, `--retries 0`): 160 passed; the client's flake check
   then had no flaky test (249 passed), where this test had failed
   once in each of three that day.
2. **The tableau's stop** (`ywtsxtrt`). Measured with a probe: no step
   of `tableau::decide` took 5 ms on SYJ202+1.008; the stop fired at
   2.000 s, and dropping the goal state then took 0.56 s (4.4 million
   inferences). The axiom closes a branch by weakening every other
   formula of its goal away, one inference each holding the rest, so
   the state grew with the square of every closed goal. Asking the stop
   inside a step, as the brief proposed, would have changed nothing
   (departs from the design). Fix: the search (`expand`) returns each
   branch's closing step (axiom, `⊥L`, `⊤R`) untaken, and `decide`
   applies them after the search, asking the stop before each. The
   tree, so the tableau and the derivation, are the same; the work and
   memory counts are unchanged. Harness at 2 s: SYJ202+1.008 from
   2 538 ms (`timeout`) to 1 329 ms (`memory_limit`: it gets further
   and reaches the bound on the goals it holds), SYJ206+1.020 from
   2 496 ms to 2 205 ms, SYJ201+1.010 proved as before (21 ms). What is
   left, 0.2 s on SYJ206+1.020, is freeing the expansions themselves
   (1.7 million inferences, three vectors each); one allocation per
   inference would be the next step, a change of `Inference`'s storage
   across the suite. Test: `a_stopped_search_holds_no_closed_branch`
   (six steps stopped hold at most 13 inferences; before, the sixth was
   an axiom of about 85 weakenings).
3. **The focused engine's late first poll** (`qropoqnt`). Measured: on
   SYJ206+1.020's affine image (12.6 million occurrences) the 4.9
   million units were 49 `&` rules of 98 303 each (the forest's width
   over 128), done in 3 ms; the 0.6 s before them were the translation
   (0.26 s) and the set-up's counts (0.31 s, 770 polls that report no
   work), and freeing took 0.21 s after. An earlier poll in the node
   would have saved 3 ms. Making the set-up's polls report work would
   have moved counters (the inverse method's turns come at counts of
   the engine's work, and a refutation's counts pass would meet the
   bound), so the fix is where the cost arose (departs from the
   design): the classical default's evidence attempt translates within
   as many occurrences as its bound of work, `EVIDENCE_WORK` (or the
   caller's lower `--occurrence-limit`), in the library
   (`with_evidence`) and the command (`bounded`, in `prove` and the
   batch); a refused image leaves the solver's answer, as any refused
   attempt did. The 124 ILTP images with evidence have at most 175
   occurrences; the 8 past 2²⁰ cost 3.7 s together for nothing.
   `prove --logic classical` on SYJ206+1.020 went from 0.84 s and
   775 MB to under 10 ms and 10 MB; the attempt now costs at most
   0.1 s on ILTP (SYJ206+1.016, 786 415 occurrences). The engine is
   untouched. Test: `a_large_image_gives_no_evidence_and_costs_no_search`
   (a chain of `↔` whose image passes 2²⁰ occurrences: the solver's
   clausal proof, and the stop asked exactly as often as by the solver
   alone). The `later.md` entry is closed with these numbers.

### The checks

- **The target set against 47's** (`bench/targets.sh` run from a copy
  pinned to cores 4 and 5, label `54x4`, at `qropoqnt`):
  `summary --against bench/targets/47.csv … --counters`: no verdict
  differs, and of the 145 problems both decide alike on one thread none
  differs in nodes, splits, memo hits, memo entries, links or tests.
  The CSV is kept out of the tree.
- **The gate** (capped, cores 4 to 6) at `qropoqnt` before the
  reviewer's text fixes and the rebase: passed, the ratchet within its
  ceilings.
- **Every family at 5 s** (release, cores 4 to 6, after the rebase):
  195 problems (100 proved, 66 refuted, 29 unknown), "every problem
  decided in a mode has one verdict".
- The workspace's tests and Clippy before each commit; the manual's and
  the guide's examples (`--test readme`) after the reviewer's text
  fixes.
- **The client**: both tests repeated as above; its unit suite 613 and
  e2e suite 249 passed; its `nix flake check` at `kvwnuzlv` under the
  machine lock: all checks passed, no test flaky.
- **`nix flake check`** at this report's commit, under the machine
  lock: all checks passed on `main@origin` `ntlzslyx`, and again after
  the rebase onto `sxwwwyql` (61b's bindings and drawings of calculi
  given as data came in); this line is the only change since.

### The reviewer

`fresh-reviewer` (Fable 5.2, medium) read both chains against the
brief, ran the command on core 6 and the 274 ILTP problems as a batch
(123 derivations and one refutation by the embedding, the 124 as
before). It found the three fixes sound (the client's waits are
conditions on the page's own invariant; deferring the closings keeps
the tree; lower `--work-limit` and `--occurrence-limit` still apply, the
library and the command compute the same bounds). Three findings:

1. **The manual and the guide still gave the old cost** (at most 0.7 s)
   and did not say the image is bounded: `docs/src/ordinary-logic.md`
   and `skills/deduxit/reference.md` now say "on an image of at most
   2²⁰ occurrences … at most 0.1 s" and that a lower `--work-limit` or
   `--occurrence-limit` lowers its bound; folded into `qropoqnt`.
2. **The tableau's test called `Interactive::parts`, which only
   `serialize` builds**: it is gated on `serialize` now, as the accessor
   is; folded into `ywtsxtrt`. The test build with `parse,interactive`
   and no `serialize` fails anyway, in the linear goal state's tests
   (`linear/proofs/interactive/mod.rs`, nine calls of `inferences`),
   before this chain: a follow-up below.
3. **The report was not in the chain yet**: this section.

### What the next sessions must know

- **A test that reads the drawing after `rule()`, `cut()` or another
  helper that only clicks must wait for the step's drawing first** (a
  goal's button that only the new drawing has, or the count of open
  goals): the helpers return before the page answers, and the race
  shows only under the flake check's load.
- **The tableau's goal state grows with every premise's copy of its
  goal**; past 1.7 million inferences freeing it is a fifth of a second.
  A storage of `Inference` with one allocation (the sides and premises
  in one slice) would cut that and the memory alike.
- **The set-up's polls report no work**, on purpose (the slices and the
  counters); a work bound therefore cannot end a search during its
  set-up, which on millions of occurrences is most of a second. A
  caller that bounds work and meets large inputs bounds the input too,
  as the evidence attempt now does.
- **Follow-up**: `cargo test -p deduxit --no-default-features --features
  parse,interactive --lib` does not build (the linear goal state's tests
  call `Interactive::inferences`, which needs `serialize`); no check
  builds that combination's tests.

## Session 54x5: the tableau's memory account

Opus 5.5 at `high`, the reused worker of 54s-c2 to 54x4 (same
conversation), in the jj workspace `../deduxit-wt-step-54s-c1-161790`,
from the brief `start-54x5.md`, on 2026-11-16, cores 4 to 6. The fix is
`ykkxnxwr` on `main@origin`, this report after it.

### The measurement

- **62a's witness no longer passes the bound.** `prove --logic
  classical --method tableau --memory-limit L -f SYJ202+1.004.p` at
  `main@origin` (54x4 included) refused for memory at every limit, its
  peak resident memory about 0.8 times the limit: 219 MB at 256 MiB,
  430 MB at 512 MiB, 851 MB at 1 GiB, 1.69 GB at 2 GiB. 62a measured
  before 54x4, when each axiom's weakenings grew the state during the
  search, uncharged.
- **The same structure was still uncharged after the search.** Since
  54x4 the closing steps run after the search; an axiom on a goal of n
  formulas builds a weakening per formula but two, each holding the
  rest, and nothing charged them. Witness: 600 atoms each side, ten
  disjunctions and `z ∨ z` (2 048 branches of about 1 200 formulas,
  closed by axioms on `z`): the search held a few megabytes and passed,
  and the closing phase was killed at a 6 GiB scope under
  `--memory-limit 256MiB` (and under 1 GiB), 4.5 s in.

### The cause and the fix

Each axiom's weakenings are charged when the search meets it
(`axiom_bytes`: `n − 1` inferences of `n(n − 1)/2` ids for a goal of
`n` formulas, at the opened goal's rates, `GOAL_BYTES` and 8 bytes an
id), and the memory test runs after every step, closings included. So
the search refuses with `Refusal::Memory` before the closing phase
builds what would pass the bound. `⊥L` and `⊤R` replace the goal in
place and add nothing.

- **The factor.** Where the closings are built (300 atoms a side,
  2 048 branches, `--memory-limit 4GiB`), the charge was 3.1 GiB and
  the resident peak 1.72 GB: the charge is about twice what the state
  holds (ids are 4 bytes, charged 8, as the opened goals' are), so a
  refusal comes early. The witness now refuses in under 0.2 s at every
  limit from 64 MiB to 4 GiB, peaking at 9 to 18 MB; SYJ202+1.004 at 12
  to 228 MB.
- **What the bound does not cover** (the reviewer's measurement): the
  derivation `finished` builds from the closed state, its check and
  `Tableau::of` are the proof returned, bounded by `derivation_bytes`
  (64 MiB by default, by an estimate 2 to 9 times their resident size),
  as `Limits` documents for every proof returned. Under
  `--derivation-limit none`, a run just within the memory bound peaked
  1.1 times it where axioms dominate and up to 1.7 times where many
  small goals do; with the default derivation limit the peak stays below
  the memory bound. Charging the derivation to `memory_bytes` would
  change that documented contract for every method, and was left.
- **The test**: `the_axioms_weakenings_count_against_the_memory_bound`,
  128 branches of 128 formulas, charged some 11 MB once closed (they
  hold 6 to 7 MB) where the search holds 0.3 MB: a bound of 1 MiB
  refuses for memory (before, it answered valid), 64 MiB lets it
  finish. With the charge removed it fails.

### The checks

- **ILTP's 274 classical problems under the tableau at 2 s**, the
  harness built before and after the fix, side by side on cores 4 and
  5: every verdict the same (184 proved, 1 refuted, 89 unknown). 37
  unknowns now stop at the memory bound where they ran into the time
  limit (34) or the derivation limit (3): the search refuses as soon as
  its closed state would pass the bound. The pass took 15.7 s summed
  against 134.6 s, its longest row 0.74 s against 3.1 s.
- **The gate** (capped, cores 4 to 6) at `ykkxnxwr` before the
  reviewer's two text fixes: passed, the ratchet within its ceilings.
- Clippy on the crate with every feature, and the ordinary suite's 70
  tests, before the gate.
- **`nix flake check`** at this report's commit, under the machine
  lock, on `main@origin` `snqrmoss`: all checks passed (this line is
  the only change after it).

### The reviewer

`fresh-reviewer` (Fable 5.2, medium) ran the release command on core 6
and checked `axiom_bytes` against what `axiom` builds (n − 2 weakenings
and the axiom, about n(n − 1)/2 ids net: the charge is above it for
every n ≥ 2, all arithmetic saturating, n ≥ 2 by `next_step`). Nothing
breaks correctness; the one caller of `tableau::decide` is
`Method::Tableau`, so no other method moves. Two text findings, both
answered in `ykkxnxwr`:

1. **The peak past the bound under `--derivation-limit none`**
   (1.1 to 1.7 times, measured at the least limit not refused): the
   derivation and the tableau after the state are bounded by the
   derivation limit, not the memory bound. The rules file and this
   report now say which bound covers what, with the factor.
2. **The test's comment gave the charge (11 MB) as what the state
   holds** (6 to 7 MB): it now says both.

## Session 54x6: the web module's size, and three loose ends

Opus 5.5 at `high`, a reused worker, in the jj workspace
`../deduxit-wt-step-58a-610ed3`, from the brief `start-54x6.md`, on
2026-11-16, cores 7 to 9. The chain is `yrvrponq` (the wire docs),
`wqopplrx` (the trim), `kvwxqypz` (the ceiling) and this report, made
on `lroloqww` and rebased onto `main@origin` `vyktrwxy` (0.5.0
published at `ruwkkrsz`, with the modal chain 65a to 65c, and a plan
commit after it); the client's change is `tokunpkt` on the
client's `main@origin` `koqworoq`, in its workspace `step54x6`
(`../deduxit.github.io-54x6`).

### The module's size

Measured as `bench/footprint.py` says: the module built again with its
names (`cargo build --release` for wasm32, `wasm-bindgen`, `wasm-opt -O3
-g`), twiggy's CSV, and `wasm-opt -O3` of the same build for the size at
`gzip -9`, which came within 22 bytes of the package's 3 670 553.

| build | raw code and data | gzip -9 |
|---|---|---|
| before the workspace's calls (`mpwqoysw`) | 8 496 993 | 3 166 018 |
| `main@origin` (`lroloqww`) | 10 446 036 | 3 670 575 |
| first trim (the ordinary engines) | | 3 567 069 |
| this chain (`wqopplrx`) | 9 850 130 | 3 541 359 |

**What came in** (raw bytes, before → after the workspace's calls):

| group or crate | before | after | change |
|---|---|---|---|
| deduxit and deduxit-web | 2 783 076 | 3 928 688 | +1 145 612 |
| the standard library (`core`, `alloc`) | 1 238 205 | 1 741 331 | +503 126 |
| the other crates (`serde_json`, `serde`, `serde_core`) | 383 162 | 573 308 | +190 146 |
| data segments | 825 777 | 883 903 | +58 126 |
| the bindings' shims | 581 175 | 633 151 | +51 976 |
| the PNG and PDF renderers' crates | 2 685 598 | 2 685 655 | +57 |

In deduxit, by module: `protocol` +413 288 (it was not linked
before: `ordinary` 173 KB, `linear` 101 KB, `document` 53 KB, `rules`
48 KB of it), `ordinary` +340 534, `serialize` +160 206, `workspace`
+111 202, `linear` +73 145, `export::svg` +39 161, `settings` +20 990
(its serializer). `core`'s growth is serde: the deserializers of every
kind's form (`serialize::ordinary`, `rules`, `search`, `sequents`,
`proofs`, `interactive`, the workspace's own form), which only the
workspace's reads of each kind reach.

**What was trimmed**: copies of one function that differ only in the
stop closure's type. Each caller's closure, and each `&mut` added when
the stop is passed on, is a type of its own; the protocol hands the
suites a `&mut dyn`, and the bodies below re-borrowed it at every depth.
At `main@origin` the module held 697 KB of deduxit's generics past their
first copy (235 KB before the workspace's calls): six copies of
`sat::run` (145 KB), five of `decide_within` (83 KB), four of
`ljt::run`, five of `Countermodel::found`, nine of `Image::read_back`,
three of `svg::write`. Each body now takes `&mut dyn FnMut(Progress) ->
bool` behind its public `impl FnMut` entry, which passes `&mut stop`
(no public signature changed): `sat::run`, `ljt::run`, `decide_within`
(`within`), `Image::read_back` (`read_back_with`), `Model::forcing`
(`forcing_with`), `Countermodel::found` (crate-private, takes the trait
object itself), `export::judgement` (likewise) and `svg::write`
(`drawn`). The first five took 103 506 bytes at gzip, the last three
25 710: 129 216 in all, a quarter of what the workspace added. The
bindings' own calls share those bodies now, so `ordinary` is smaller
than before the workspace (277 KB against 323 KB). `core.md` has the
rule.

**The ceiling** is 3 750 000 (`kvwxqypz`), the trimmed size with the
margin the last one had (6 %).

**What was not trimmed, and why** (in `later.md`'s section, rewritten):
- `linear::search::prove_goal`, four copies, 23 KB raw: the focused
  engine's front door, whose change the rules check with the target
  set on the timed cores, which the brief did not name.
- The protocol's dispatch and the readers and writers of every kind
  that only the workspace reaches, about 1.35 MB raw. The page links
  `ordinary.translate` and `ordinary.start`; a trim needs the library to
  let the bindings name the operations and kinds a workspace may run, an
  API change and the author's to decide (a feature per suite is against
  the plan's "no feature per suite").
- **A module per suite (web-front-end-4) would gain little**: the linear
  suite is 1.40 MB raw of deduxit's 3.13 MB, and each suite's module
  would carry the PNG and PDF renderers (2.69 MB raw, 27 % of the
  module) and the protocol. The larger first-load gain is a module of
  its own for the renderers, loaded at the first PNG or PDF export.

### The wire docs (`yrvrponq`)

Each of 63a's findings, against the writers and the command's output:
- `wire.rs` lists the record of a cut elimination on a net (`version`
  the net's level, at least 6 since its sequent keeps the cut formulas;
  `net` without its own `version`; each step's `cut` as two vertices,
  `case` and `above`; `end` and `reason`).
- The session's entry has a first-order session's `instances` (level 7;
  `members`, `frames`, `terms` with the open witnesses `{"M": k}`,
  `eigens`, and `metas`, `constants`, `bindings`, `marks` where not
  empty).
- The level-13 row, the ordinary outcome's entry and the Kripke model's
  say the model is nested as `countermodel.model` beside
  `countermodel.sequent`.
- `seq translate`'s and `interpolate`'s help, and the guide, list
  `deduxit` among their records' keys (the manual already did).
- `written`: no key is dead. `Interactive::of_classical`, which the web
  bindings call for a typed sequent, keeps it; the command never starts
  such a session, but its `load` keeps the key of a document that has
  it and `save` writes it back. `wire.rs` and the schema's description
  of the session say so.

### The client (`tokunpkt`)

The workspace panel's Undo and Redo read "Undoing NAME…" and "Redoing
NAME…" from the press until the tabs show the change, both disabled,
the pressed one `aria-busy`; a second press does nothing (it queued a
second undo before). The name is the one pressed: the view names the
next action as soon as it is read again. The client's `web.md` says
so. No test pins the busy state: it lasts as long as the worker takes,
which a test cannot hold open without a hook of its own; the e2e tests
that press Undo and Redo pass.

### The checks

- **The gate** (capped, cores 7 to 9) at `kvwxqypz` before the
  reviewer's fixes (two doc paragraphs and the client's label): passed.
  The ratchet within its ceilings; against `main@origin` counted the
  same way, every journey within 0.15 % but `read-json` +0.88 % (this
  chain does not touch the readers: layout) and `ordinary-pigeons`
  +0.85 % (the trait-object stop of the SAT search).
- `cargo doc -p deduxit --all-features` with warnings denied (the new
  links), clippy on the crate with every feature.
- **The client**: Vitest (667) and lint before the fix, lint after, and
  its `nix flake check` at `tokunpkt` under the machine lock: all
  checks passed, the e2e suite 300 passed.
- **`nix flake check`** at the report's commit under the machine lock,
  the bindings' tests under Node among them: all checks passed. Its
  `budget` check gives the package at 3 541 980 bytes at `gzip -9`
  (173 packages), the figure `budget.nix` and `web.md` now name; the
  table above is this measurement's rebuild, 621 bytes below. Only the
  ceiling's comment changed after it.
- **After the rebase onto `ruwkkrsz`**: clippy on the workspace, then
  `nix flake check` at the rebased report under the machine lock: all
  checks passed. The modal suite adds 80 988 bytes: the package is
  3 622 968 at `gzip -9`, 3.5 % under the ceiling. `budget.nix` and
  `web.md` name that figure; only those comments and this report
  changed after the check.

### The reviewer

`fresh-reviewer` (Fable 5.2, medium) read the chain and the client's
change and ran the command for the wire docs. Three findings, all
answered:
1. **The net record's example said `"version": 4`**; a record with a
   step is at least 6, since the net keeps its cut formulas (its
   witness: `eliminate --net --format json` of the manual's
   `contraction.json`). Fixed in `yrvrponq`, with that record's step.
2. **"The command's sessions never keep `written`" was false** for a
   session the command loads: `load` keeps the key and `save` writes it.
   `wire.rs` now says both; the schema's wording was right.
3. **The busy label read the name live**, so after the view was read
   again it named the next action ("Undoing sequent change…" while
   "restart" was undone). The name is now kept as pressed.
Its question on 3 670 575 against 3 670 553: the first is this
measurement's rebuild (`wasm-opt -O3` of the named build), the second
the package's. The ceiling's comment now names the package's trimmed
size, 3 541 980, which the flake check printed.

## Session 54x7: three loose ends before 0.6.0

Opus 5.5 at `high` (the 68a worker, its conversation kept), in the
workspace `../deduxit-wt-step-58b-8c0bbc`, from the brief
`/tmp/linlog-handoff/start-54x7.md`, on 2026-11-16, cores 10 to 12 and
the fuzz run on 12 to 15.

### Outcome

- **Seriality with one new label is refused** at the cause, with the
  rule language's existing `fresh`: a name a rule asks to be new must
  also differ from every other variable of the rule. A `G3…` derivation
  or session no longer takes `Ser` with `w1 R w1` for a new `w1`.
- **The ordinary suite applies its invertible rules by itself**:
  `ordinary::Interactive::invert`, `invert_all`, `is_invertible`; the
  command's `--invertible` and `invertible on|off` under `--logic`; the
  bindings' `interactive.invertible`, `ordinary_invert_all` and the
  view's `invertible`. The page's switch is a later client step.
- **The modal fuzz targets ran**, `json_modal` seeded with the level-20
  documents and the command's modal outcomes.

The commits on `main@origin` (`smyxwoln`): `xvpnroyr` (item 1),
`opwrroou` (item 2), `yorxttur` (its behaviour lock: `interact --help`),
`oxlxvppv` (the seeding), `losmmunn` (the review's findings), and this
report's.

### 1. Seriality with one new label

- **The cause**: `Ser`'s `x` is bound from its premise alone, so the
  checker's `fresh` on `y` (new to the conclusion and the hypotheses in
  force) held for `x = y = w1`.
- **The change, with no new side condition**: a name a rule asks to be
  new must also differ from the atom any other variable of the rule is
  bound to (`Checker::conditions`, checked after the conclusion and the
  hypotheses, so a name in the conclusion is still said as such). A
  variable bound from the conclusion already made such a name occur
  there, so only a variable bound from a premise alone is affected:
  `Ser`'s `x`, in no other rule of the built-in files or the fifteen
  modal calculi (`R□` and `L◇` bind `x` from their conclusion). No
  syntax and no wire level change: the rule file's `fresh` now says
  "and that no other variable of the rule matches" (the schema's
  description, the manual's rules chapter). A session goes through the
  same checker, so `apply 0 Ser x=w1 y=w1` is refused with "a name the
  rule asks to be new is one another of its variables names".
- **The Rocq library**: its `g3`'s `ser` takes any `x` and a fresh `y`
  (66b proved it sound for that wider rule, `beside_*` in
  `Soundness.v`); deduxit now accepts a subset of those instances, so
  every derivation it accepts still has a term `mcheck` accepts. No
  Rocq change. The header comment of `theories/Modal/Calculus.v` still
  says deduxit's rule allows `y` itself (the reviewer read it): a line
  for that repository, not this step.
- Test: `seriality_s_new_world_is_not_the_one_that_sees_it`
  (`core/tests/rules.rs`), a derivation file in G3D with `w0 R w1`
  (passes) and `w1 R w1` (refused).

### 2. Auto-reverse for the ordinary suite

- **The library** (`core/src/ordinary/invert.rs`): `invert(goal,
  &limits)`, `invert_all(&limits)`, `is_invertible((left, right),
  &step)` and `invertible_steps()` on `ordinary::Interactive`, the
  linear session's shape (`Inverted` reused). The set: the axiom, and
  in LK every rule of a connective with a shared context (`⊤L` and `⊥R`
  among them); in LJ and minimal logic not `→L`, `¬L`, `∨R₁`, `∨R₂`;
  minimal logic has no `⊥L`; partial and structural rules never. A
  closing step first, then the first formula's, the hypotheses first;
  every step through `apply`, so the checker's rule checks each.
- **Tests**: on 200 generated sequents in each logic, every goal of a
  valid sequent stays valid after the run, the run ends, and each step
  is one undo; LJ leaves `→L`, `¬L`, `∨R` and minimal logic `⊥L`; the
  memory bound stops a run, an axiom's weakenings counted; the steps
  are listed by their goal, a split never.
- **The command**: `interact --logic … --invertible` and `invertible
  on|off`, `undo` taking a step back with the invertible steps after
  it, as the linear session (`cli/tests/cli.rs`, the manual's example).
- **The bindings**: `interactive.invertible` in `ordinary_start` and
  `ordinary_apply` (`inverted`, `inverted_refused`), `ordinary_invert_all`,
  the view's `invertible` (`Interactive::invertible_steps`), and `about`
  lists `interactive.invertible` among the ordinary suite's settings.
  **The page's switch is a later client step** (the client's ordinary
  tab has no toggle yet).

### 3. The modal fuzz targets

- **Seeds**: `json_modal` from the command's outcomes of seven modal
  sequents in K and S4 and the four documents of
  `core/tests/wire/level-20.txt` (18 seeds); `modal_text` 7,
  `modal_decide` 63. `fuzz/seed.sh` had stopped at the TPTP file and the
  Lambek list that now sit in `bench/problems/`; it reads the two
  commutative lists now.
- **The run** (`fuzz/run.sh --inside json_modal modal_text
  modal_decide` as the user unit `deduxit-fuzz`, cores 12 to 14, debug
  assertions, `STALL` 900 s, `CAP` 5 400 s; the build ran beforehand on
  cores 10 to 12, so `run.sh`'s own build on 2 to 5 was skipped):

  | target | time | ended | coverage | corpus | finds |
  |---|---|---|---|---|---|
  | `json_modal` | 5 401 s | cap | 8 217 | 5 483 | 0 |
  | `modal_text` | 5 401 s | cap | 1 887 | 2 627 | 0 |
  | `modal_decide` | 5 401 s | cap | 5 909 | 3 451 | 0 |

  Nothing found. All three reached the cap with their coverage still
  growing slowly, none stalled (departs from the design: "until their
  coverage stops growing"); their corpora stay under `fuzz/corpus/` in
  this workspace for a longer run.

### The reviewer's findings

`fresh-reviewer` once (Fable 5.2; stopped by the weekly limit on
2026-11-16 and resumed on 2026-11-20 with its work kept). **Item 1
held**: `Ser` with one new label is refused in G3D, G3KD45, G3KDB,
G3KD4 and G3KD5, by derivation and by session; `Ser` with a label of
the conclusion, `R□`, `L◇` and the search's `Ser` still pass; the Rocq
argument covers it. **Item 2's rule set and command held.** Its three
findings, all fixed in `losmmunn`:

1. **The view's `invertible` named a split step**: the bindings' test
   for a shared context allowed each premise one formula fewer. The
   list is now the library's `invertible_steps`, which asks every
   premise to keep as many formulas as its goal.
2. **An axiom step was named by its drawn inference, not its goal**
   (`a, b |- a` gave `[1]`, `undo` reopened 0): `invertible_steps`
   reads the steps off the history, an axiom step as its goal's chain
   of weakenings ending in the axiom.
3. **The memory bound did not count an axiom's weakenings**: 100 atoms
   under 256 KiB made 2.2 MB, and the page's session was then too large
   to draw. Every inference a step adds is now charged by the
   derivation's own estimate; the witness is a test.

### Checks

- Clippy on the workspace with `--deny warnings`, and the tests of the
  areas touched as they were made: `core/tests/rules.rs` (25, the new
  test among them), the modal and ordinary unit tests, the bindings'
  `web/tests/ordinary.rs` and `protocol.rs` natively, the command's
  `cli.rs`, README's, the manual's and the guide's examples, the
  behaviour lock (`interact --help` reblessed, `yorxttur`).
- The gate (`nix develop -c gate`) at `oxlxvppv` (cores 10 and 11,
  beside the fuzz run) and again at `losmmunn` after the review's fixes
  (cores 10 to 12): passed both times (clippy, every crate's tests, both
  `cargo hack` runs, the ratchet; its note on `render-svg`'s cache
  misses, "not a failure", as before this step).
- `nix flake check --keep-going` (under the machine lock, `--max-jobs 4
  --cores 8`) at `losmmunn` on `main@origin` `smyxwoln`: passed, the
  bindings' tests under Node among its checks. This report's commit,
  plan files alone, is the only change after it.

### Departures

- The fuzz targets stopped at the cap, not at a stall (above).
- No new side condition in the rule language: `fresh` was sharpened,
  which is no API change (no syntax, no wire level); a stored derivation
  of `Ser` with `x = y` is now refused, which was the point.
