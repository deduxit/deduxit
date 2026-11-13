# Step 54u: the usability review's accepted changes

The step's sessions each add a section here (`plan/54u-usability.md`,
"The sessions"): 54u-a, the library; 54u-b, the command; 54u-c and
54u-d, the page; 54u-e, checking a certificate in one command.

## Session 54u-a: the library

Session 1 of `plan/54u-usability.md` (Opus 5.5 at `high`, 2026-11-14,
cores 7-9), in the jj workspace `../deduxit-wt-step-54u-a-859b32`. The
four accepted questions of `plan/notes/usability.md` §6 "The library"
are built, each a commit of its own with its test and the manual's and
the guide's sentences; none breaks the library's API, and the behaviour
lock did not change. Nothing was pushed.

| commit | what |
|---|---|
| `vynxmlpk` | Q1: `Settings::stop`, and the command's stop through it |
| `npryuptt` | Q2: `Display` for `lambda::Term`, `export::{latex, typst, svg, rocq}::to_string` |
| `ylwrpsux` | Q3: `Deserialize` for the linear `Outcome`, `Engine::Unknown` |
| `umkqnukq` | Q4: `ShapeError::Formula` carries the formula's text |
| `vulvktpt` | this report |

### What was built, and why each choice

- **Q1, `Settings::stop(&self, elapsed) -> impl Fn(Progress) -> bool`**
  (`core/src/settings.rs`). It fires once `elapsed()` reaches
  `clock.time_limit_ms`, in every phase but `Phase::Refute`, never
  without a limit, and calls `elapsed` only where it can fire. The
  crate reads no clock.
  - **(departs from the design)** The accepted signature takes
    `impl FnMut() -> Duration` and returns `impl FnMut(Progress) ->
    bool`. It takes `E: Fn() -> Duration` and returns `impl Fn(Progress)
    -> bool + use<E>` instead. The command's race (`search::race`) needs
    a stop that threads share, `Fn + Sync`; an opaque return type leaks
    its auto traits, so the stop is `Sync` exactly when `elapsed` is,
    and a `Fn` is an `FnMut` wherever one is asked for. Every clock a
    caller has is a `Fn` (`Instant::elapsed`, `performance.now()`); a
    `FnMut` clock is a counter, which a caller writes as its own stop.
    `use<E>` keeps the stop free of `&self`'s lifetime.
  - The command (`cli/src/prove.rs`, `halting`, which `searched`, the
    batch and a session's `close` call) now builds its stop from
    `Settings::stop` under the deadline's limit, plus Ctrl-C in every
    phase. Its time limit stays a flag a timer raises (`Deadline`): the
    elapsed time is read off the flag, zero before and `Duration::MAX`
    after, so no poll reads a clock (the rule `cli.md` gives for it).
    The limit is passed in milliseconds rounded up, so that a limit
    under a millisecond does not become zero and fire before the flag.
    A batch passes the entry's limit, or the batch's where the entry has
    none, with both flags. `halting` makes a `Settings::default()` with
    that clock for the call (one per search), since the batch and the
    session hold their limits, not the settings.
  - The crate root's recipe for the command's verdicts now calls it
    (`core/src/lib.rs`): it was the closure FL1 found wrong, which also
    stopped the refutation.
- **Q2, printing without a stop.**
  - `impl Display for lambda::Term` (`core/src/lambda/print.rs`) writes
    the term under the default `Options` and `Limits`. A term whose text
    would pass `derivation_bytes` (a pair under a left rule doubles it)
    writes the refusal's message between angle brackets: the printer
    refuses before it writes, and a `Display` that returned `fmt::Error`
    would make `to_string` panic, which the standard library's docs
    reserve for a failing writer.
  - `export::{latex, typst, svg, rocq}::to_string(derivation, &options)
    -> Result<String, Error>` beside each `write`: the `write` into a
    `String` with a stop that never fires. Their text is linear in the
    derivation, which was made within its own limits, and the
    lambda-term under it is bounded by the default limits, as the
    `write`s already were. PNG and PDF have none: their `from_svg`
    returns bytes already.
- **Q3, `Deserialize` for `Outcome`** (`core/src/serialize/search.rs`,
  `OutcomeForm`, and `wire::Readable` with form `outcome`, so
  `wire::upgrade` reads it within the caller's limits). The proof goes
  through `Proof`'s own reader with the outcome's mode recorded, the
  disproof through `DisproofForm`'s, the reason through `Why`; `deduxit`
  is not read, `checked` is false where it is missing, `net` is `None`.
  - **An unknown engine name** reads as the new variant
    `Engine::Unknown`, serde's `#[serde(other)]` idiom (written by hand,
    since `Engine` reads from its name). `Engine` is `#[non_exhaustive]`,
    so the variant is an addition. Forced through `Options::engine` it is
    `Error::UnknownName` in `prepare`, before any engine runs; it is not
    in `Engine::ALL` or `NAMES`, lists no counters, its name is
    `unknown`, and an options value still refuses an unknown name.
  - The test `outcomes_read_back` (`core/tests/stability.rs`) reads
    every outcome of the JSON lock (`core/tests/lock/json.txt`: every
    verdict, refutation and reason it holds, at levels 1 and 2, the
    levels the lock holds) and writes it back byte for byte, reads the
    outcomes 0.1.0 wrote, and reads `"engine": "sat"` as
    `Engine::Unknown`.
- **Q4, the formula in the error.** `ShapeError::Formula` gains the
  field `formula: Option<Box<str>>` (its variant is `#[non_exhaustive]`,
  so a field is an addition). `Reading::new` and `of_mode` fill it, since
  they have the forest: every caller that reads a sequent intuitionistically
  (`prove` in intuitionistic mode, the checker, the exports, the
  session) gets it. `Display` writes it in place of the id; `describe`
  is unchanged; an error made without the text prints the id. The text
  is cut at 4 096 characters with `…`, as `describe::TERM` cuts a term,
  since a shared subformula of a short JSON sequent can unfold past any
  memory. The message's words are kept ("the subformula A ⅋ B is neither
  an intuitionistic formula nor …", as `describe` writes it), so the
  command's output and the behaviour lock do not change; 54w-d rewords
  the message with the rest.
- **Docs, added, not rewritten:** the manual names `Settings::stop`
  (`docs/src/commands.md`, after the time limit), reading an `Outcome`
  (the same chapter, after `--format json`), the error's `Display`
  (after the intuitionistic errors), the exports' `to_string`
  (`output-formats.md`) and `Display` for `Term` (`terms.md`); the
  guide's §16 names all four. The `Outcome` type's docs show reading one
  back as a doc test; `wire.rs` no longer calls the outcome written only.
- **Rules:** `core.md` (the API at a glance; the stop decision notes the
  printers without a stop), `core-search.md` (the refutation's stop
  through `Settings::stop`; `Engine::Unknown`), `core-sequents.md` (the
  outcome is read), `core-export.md` (`to_string`, replacing "no
  `derivation(…) -> String` twins"), `core-lambda.md` (`Display` never
  fails), `core-forest.md` (the error's text), `cli.md` (the stop).

### Options and front ends (D15)

None of the four adds an option. `Settings::stop` reads the existing
`clock.time_limit_ms`, which the command's `--timeout` and a front end's
settings JSON set; the web bindings keep their own stop by work, since a
tab has no clock the library may read. `Display` and `to_string` use the
default options and limits; a front end that wants others keeps calling
`write`. The outcome reader reads what the writer writes.

### The checks

| check | result |
|---|---|
| the four focused tests: `the_stop_lets_the_refutation_run_past_the_limit` (settings), `a_term_displays_as_written` (lambda), `to_string_is_write_into_a_string` (export), `outcomes_read_back` (stability), and Q4's in `dispatch_intuitionistic` and the reading's `shapes` | pass |
| `cargo clippy --workspace --all-targets -- --deny warnings`, `cargo test --workspace` (capped, cores 7-9) | pass: 748 tests and 65 doc tests, both behaviour locks and `readme` among them |
| `breaks 0.3.0` | see below |
| the gate (`systemd-run … taskset -c 7-9 nix develop -c gate`) at the last code commit, after the reviewer's fix | `gate: passed`; the ratchet's journeys within their ceilings; the cache model's misses moved on `search-qbf-20-2` and `check-qbf-20-2` (by its own words a question, not a failure; none of the four touches those searches' loops) |
| `nix flake check` under `/tmp/linlog-machine.lock`, `--max-jobs 4 --cores 8`, at the same code with this report | exit 0 |

`breaks 0.3.0` lists, of this chain, only additions: the four
`to_string`, `Settings::stop`, `Display` for `Term`, `Engine::Unknown`,
`ShapeError::Formula::formula`, and `Deserialize` and `Readable` for
`Outcome`. Its three semver failures and two removed fields are step
54's, already on `main` before this chain (`ordinary::Rule::Cut`, which
moves `Rule::Open`'s discriminant and order, and `ordinary::Outcome`'s
`translation` and `linear`). The same two tools run between `main`
(`povyqwuq`) and this chain's head, from a temporary jj workspace at
`main` removed after: cargo-semver-checks "229 checks: 229 pass, 31
skip … no semver update required", and cargo-public-api's diff "Removed
items … (none)", "Changed items … (none)", the additions above.

### The reviewer

`fresh-reviewer` (deduxit-fresh-reviewer-54u-a, Fable 5.2 at `medium`,
read only) judged the four commits against the brief's criteria: Q1 to
Q4, additions only, the docs in the same commit, the writing rules. It
reported one finding, and none on the other criteria (it checked Q1's
stop case by case against the old rule: the refutation, a zero, a
sub-millisecond and an unrepresentable limit, a batch with and without
a per-sequent limit; every `implementation()` call against
`Engine::Unknown`; the size assertion of `Error`).

- **Low: `Outcome::checked`'s doc said it is false only where
  `Options::check` was off**, which an outcome read from a document
  contradicts: its `checked` is the document's word, and the reader
  checks indices, not the proof. **Fixed** in `ylwrpsux`: the field's
  doc says so and points to `Proof::check`. Reading `checked` as false
  always would have broken the byte-for-byte write-back, so the doc is
  the fix, as the reviewer suggested.
- It noted, without a finding, that the test reads the lock's levels
  (1 and 2) and the outcomes 0.1.0 wrote, not outcomes at levels 3, 6
  and 7 or of a goal off the roots; those go through the same `Proof`
  and `Disproof` readers the stability test already runs at those
  levels, which the brief's "each level the behaviour lock holds" does
  not ask beyond.

### Proposals beyond the items, for the author

- `Settings::stop` could take an `FnMut` clock as the design had it, at
  the cost of the race: a second method, or a wrapper type, would
  serve both. I recommend leaving it as built until a caller with a
  mutable clock appears.
- `Engine::Unknown` loses the name it was read from: an outcome read and
  written again says `"engine": "unknown"`. Keeping the name needs a
  field (a `Box<str>` in `Outcome`, or an `Engine` that is not `Copy`).
  I recommend keeping it as built: the name of an engine this version
  cannot run tells a program nothing it can act on.
- `Outcome::net` could be read back from the proof (`from_proof`) where
  the outcome names the net engine; it is `None` now, as the writer
  never wrote it.

### What the next sessions must know

- **54w-c (README, the manual, the guide):** the sentences added are
  short and placed where each area is described; rewrite around them.
  The crate root's recipe now uses `Settings::stop`, so the FL1 fix
  (the refutation's stop) is done in code; the guide's §16 paragraph
  names all four calls in one run of prose, which 54w-c may split.
- **54w-e3 (export):** each format has `to_string` beside `write`;
  `core-export.md` says so. The module docs of `export` and its four
  formats do not mention `to_string` yet beyond the item docs.
- **54w-e5 (the root, `web/`, `bench/`, `compat/`):** the crate root's
  second recipe is `Settings::stop` now; `lib.rs`'s paragraph before it
  still explains the copy bound at length (FL8's work). The web
  bindings use none of the four.
- **54u-b**, beside: `cli/src/prove.rs`'s `halting` and `searched`
  changed signature (`searched` takes `(limit, passed)`); a rebase over
  it touches only those lines and their three callers. The behaviour
  lock did not move here.
- The error text of Q4 kept the old words, so the message rewrite of
  54w-d changes `reading.rs`'s `write` and the tests that pin it
  (`reading.rs`'s `shapes`, `search/mod.rs`'s
  `dispatch_intuitionistic`, `cli/tests/cli.rs`, `docs/src/commands.md`)
  in one place.
- The chain's commits are unsigned: the signing hook found the key
  locked; the push signs them.
