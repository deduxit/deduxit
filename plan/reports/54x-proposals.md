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
