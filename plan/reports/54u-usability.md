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

## Session 54u-b: the command

Item 2 of `plan/54u-usability.md` (Opus 5.5 at `high`, 2026-11-14, cores
10-12, jj workspace `deduxit-wt-step-54u-b-3b6b81`), the author away:
every decision below was taken by research and the code's own idiom,
each with its reason.

### Outcome

Q6, Q8, Q9 and Q10 are built, each with focused tests, and shown
working by capped calls of the command. The chain on `main`
(`povyqwuq`, which is `main@origin`), oldest first:

| change | what |
|---|---|
| `kmokzrnw` | Q6: hints on a newcomer's first errors, `--format tex` |
| `rwuvpows` | Q8: classical refutations by the sides written, the `-a` note |
| `lpnqrkvr` | Q9: `interact`'s rule list and the warning at a lost split |
| `zonptzos` | Q10: `-h` grouped, the expert flags left to `--help` |
| `qxoqulyv` | the behaviour lock's changed lines, alone |
| `rvxklplz` | the rules files |
| (this section) | the report |

Nothing is pushed. The commits are thematic, each builds (`cargo check
--workspace --all-targets` at every one); the lock fails between
`kmokzrnw` and `qxoqulyv` by design, since its blessing is the commit
of its own the step asks for.

### What was built

**Q6, hints that say what to write** (`cli/src/hint.rs`, new; `lib.rs`;
the parser's hint in `core/src/linear/parse/mod.rs`):

```console
$ deduxit "A |- A"
error: unrecognized subcommand 'A |- A'

  tip: to decide a sequent, run deduxit prove "A |- A"
…
$ deduxit prove A -o B
error: cannot parse the sequent
  A
   ^ unexpected end of input, expected a connective, `,` or `|-`
  quote the sequent: deduxit prove "|- A -o B"
$ deduxit prove '!A,' B
error: unexpected argument 'B' found

  tip: quote the sequent: deduxit prove '|- !A, B'
…
$ deduxit prove "A -> B |- A"
error: cannot parse the sequent
  A -> B |- A
     ^ unexpected ">", expected the `o` of `-o`
  -> is ordinary logic's arrow: write -o, or add --logic classical
$ deduxit prove --file s.txt        # id: A |- A, mp: … one per line
error: cannot parse the sequent
  id: A |- A
    ^ unexpected ":" at line 1, character 3, expected a connective, `,` or `|-`
  to read one sequent per line, add --input-format lines
$ deduxit prove -q --format tex "A |- A"
% provable (MLL, classical, focus engine)
```

Decisions:
- The subcommand and unquoted-argument cases are clap's own errors, so
  the hint is clap's `tip:` line (`clap::Error::insert` with
  `ContextKind::Suggested`, read in clap_builder 4.6.7's
  `error/format.rs`), as git's and cargo's are. The subcommand tip comes
  only where clap found no similar subcommand (`deduxit prov` keeps
  clap's "a similar subcommand exists").
- The item names the parse that ends early (`-o B` taken as `--output
  B`); the same mistake also reaches clap as an argument the command does
  not take (`deduxit prove A, B`), which a shell makes of every unquoted
  sequent with a comma. Both get the hint (departs from the design only
  in covering the second form of the same mistake). The command line is
  parsed again without each refused word, at most once per word, until
  the command reads a sequent; the hint then quotes the words of the
  command line as written, from the first of them up to a flag (`-o`
  is the implication), the same words the `parse` path quotes.
- An unquoted linear sequent never has `|-` (the shell reads `|` as a
  pipe), so the hint puts `|- ` before a sequent that has no turnstile,
  so that the command it shows runs. Under `--logic` it does not: an
  ordinary formula stands alone.
- Quoting is POSIX shell's: double quotes, single quotes where the text
  has `!` (history expansion in bash and zsh), `"`, `\`, `$` or `` ` ``.
- The `->` hint is the library's (`ParseError::hint`, `ARROW`, in both
  parsers), so the page shows it too; the command adds `, or add
  --logic classical` where the command takes that flag as it was called
  (not `seq json`, not with `-i`), as it adds `--ill-syntax one-sided`
  after the intuitionistic hints.
- The lines hint: `prove`, text read from a file or standard input, a
  parse error at a `:` or past the first line. `--input-format lines` is
  `prove`'s alone (a batch), so `interact` and `seq` do not get it.
- `--format tex` is `#[value(alias = "tex")]` on `Format::Latex` and on
  `seq print`'s `SequentFormat::Latex`, hidden from the list of values
  as an alias is.

**Q8, refutations in the sequent's terms** (`core/src/linear/search/refutation.rs`,
`cli/src/prove.rs`):

```console
$ deduxit prove "A |- B"
unprovable (MLL, classical, focus engine): A on the left has no A on the right to meet in an axiom
$ deduxit prove "A, B |- A"
unprovable (MLL, classical, focus engine): B on the left has no B on the right to meet in an axiom; with -a (affine), an unused hypothesis is allowed
$ deduxit prove "B, ~A |- B"
unprovable (MLL, classical, focus engine): A on the right has no A on the left to meet in an axiom (a negated formula or an implication's antecedent counts on the other side)
$ deduxit prove "!A, A |- 1"
unprovable (MELL, classical, horn engine): A occurs 1 more time on the left than on the right outside the exponentials, and every occurrence of it below one, which copies add, is on the left, so they cannot all meet in axioms
$ deduxit prove -i "A, B |- A"
unprovable (IMLL, intuitionistic, two-sided engine): B occurs negatively 1 more time than positively (…), so they cannot all meet in axioms; with -a (affine), an unused hypothesis is allowed
```

Decisions:
- "Where the input was two-sided" is read as a classical sequent whose
  `antecedents()` is above zero, refuted as a whole. `⊢ Γ`, a JSON
  sequent without sides and a goal of a session keep the one-sided
  phrase; a session prints its goals one-sided, so its refutations stay
  in those terms.
- The phrase is the library's `Disproof` `Display`, so every front end
  gets it (the batch, `check`, a session's `close`, the page). The JSON
  `refutation`, the check and the certificates are unchanged. `check`
  names such a sequent as written (`valid refutation of a ⊢ b
  (classical): …`), so that the phrase and the sequent agree.
- "A on the left has no A on the right" is said only where the side with
  fewer literals has none in the roots (`placed`, one pass over the
  arena in index order, a byte a term, as the crate's rule against
  recursion asks); otherwise the excess is counted left and right. The
  note in brackets comes where a literal counts on the other side than
  the formula it stands in, which is how a reader matches `B, ~A |- B`
  to "A on the right".
- The `-a` note names a flag, so it is the command's, not the library's
  (cli.md: no flags in core). It needs four things: a count of an atom
  (or predicate) in excess left of `⊢`, a hypothesis that is that atom,
  no atom in excess right of `⊢` (`right_surplus`), and neither affine
  nor an ordered mode, the sequent refuted whole. The third test is
  mine: without it README's `A |- B` named `-a`, which does not prove
  it. The note says what `-a` allows, not that it proves the sequent;
  `A par B |- A` still gets it and stays unprovable under `-a`.
- Intuitionistic refutations kept their wording; their rewrite is
  54w-d's text work. The count equation's phrase stays one-sided: its
  formula counts one-sided connectives.

**Q9, `interact` that guides** (`cli/src/interact/mod.rs`, `ordinary.rs`):

```console
$ deduxit interact "A, B |- A * B"
> apply 0 2 tensor 1
error: unknown rule `tensor`; the rules here are ax, * (⊗), par (⅋), 1, bot (⊥), &, +1 (⊕₁), +2 (⊕₂), top (⊤), !, ?d, ?c and ?w
> apply 0 2 * 1
opened goal 1: ⊢ 0: ~B, 1: A
opened goal 2: ⊢ 0: ~A, 1: B
warning: one premise has no proof, so this split cannot be closed; `undo` takes it back
> undo
reopened goal 0: ⊢ 0: ~A, 1: ~B, 2: A ⊗ B
```

Decisions:
- The session's rules are its mode's: classical one-sided, intuitionistic
  two-sided (`-oL (⊸L)` …), the cyclic mode's `ax, ⊗, ⅋`, the Lambek
  calculus's `ax, ⊗L, ⊗R, \L, \R, /L, /R`, with `mix` and `wk` where the
  mode has them and the quantifiers' rules where the sequent has a
  binder. `cut` and `open` are not listed: `apply` refuses both (`cut`
  is a command of its own). Each is written `ASCII (name)`; a rule
  whose name is ASCII is written once.
- The ASCII spellings live in the command (`LINEAR_ASCII`, and
  `ordinary::ASCII` for LK and LJ), since the library has no public
  table of them and adding one is an API change; `listed_rules_read_back`
  holds every listed name and spelling to the library's `FromStr`, so
  the two cannot drift.
- A session of LK or LJ gets the same list, with the rules its logic
  refuses left out (`WR`, `CR`, `⊥R`, `∨R` outside LK, `⊥L` in minimal
  logic, as `core-ordinary.md`'s `shape` says).
- The warning is `Interactive::split_passes`, the test the page calls
  at a split ("Cannot close"), asked before `apply` for a `⊗` or Mix
  step and printed after the goals the step opened, with `undo` named
  (C7's friction: `close` did not suggest it). The page's tip reads "One
  premise cannot be proved however the rest goes"; the line says the
  same in the session's words.

**Q10, a short `-h`** (`cli/src/argument_parsing.rs`): `prove -h` lists
45 options (the review's count, `grep -cE '^\s+(-[a-zA-Z], )?--'`; 65
in one block before, 66 in `--help`), `--settings` and `--help` first,
then under Logic, Input, Output, Search and Limits, and ends with
"--help shows every option."; `--help` lists every one under the same
headings.

Decisions:
- The headings are per flag (`help_heading = LOGIC`, …) on the shared
  argument structs, so `interact`, `check`, `eliminate` and `seq print`
  get them too. clap orders headings by the first flag that names each
  (clap_builder 4.6.7, `help_template.rs`, `write_all_args`), so
  `ProveArgs` declares the logic before the input, and the input's size
  limits sit under Input.
- Hidden from `-h` (`hide_short_help`): the prompt's list (`--bias`,
  `--forward-copies`, `--memo-limit`, `--pool-after`, the limits past
  `--timeout` and `--memory-limit`: `--occurrence-limit`, `--term-limit`,
  `--classical-work`, `--witness-depth`, `--instances`,
  `--recursion-limit`, `--work-limit`, `--derivation-limit`), and two
  more of the same search tuning, `--test-period` and `--schedule`
  (departs from the design: not named, but of the same kind as
  `--bias`). "The benchmark inputs" names no flag; I read it as the
  batch's flags for large runs (`--files-from`, `--null`, `--workers`,
  `--cores`, `--batch-memory`, `--batch-timeout`, `--isolate`), which is
  why `-h` has no Batch heading. `--input-format lines` stays visible, so
  a batch is still found from `-h`.
- `check` and `eliminate` hide `--work-limit`, `--occurrence-limit`,
  `--term-limit` and `--derivation-limit` through the shared structs, so
  their `-h` ends with the same line.

### Tests, one per behaviour

- `core`: `a_two_sided_sequent_is_refuted_by_its_sides` (the wording,
  the crossed note, the one-sided cases).
- `cli/tests/cli.rs`: `newcomer_errors_say_what_to_write` (Q6, every
  hint and `tex`), `an_unused_hypothesis_names_affine_mode` (the `-a`
  note and where it is left out, with `-a` proving `A, B |- A`),
  `a_session_lists_its_rules_and_warns_at_a_lost_split` (Q9),
  `short_help_keeps_to_the_common_flags` (Q10).
- `cli/src/hint.rs`: `quoting_keeps_a_sequent_one_word`;
  `cli/src/interact/mod.rs`: `listed_rules_read_back`.
- Two pinned lines of `batch_inputs_and_exit_status` and
  `check_s_verdicts` changed with Q8 (the behaviour meant to change).

### Checks

Each run capped (`systemd-run --user --scope -p MemoryMax=8G -p
MemorySwapMax=0 taskset -c 10-12`, three build jobs and test threads);
the flake check under `flock -o /tmp/linlog-machine.lock` with
`--max-jobs 4 --cores 8`.

- `cargo clippy --workspace --all-targets -- --deny warnings`: clean.
- `cargo test --workspace`: passed, the lock and README's, the
  manual's and the guide's examples among them (`cargo test -p
  deduxit-cli --test readme`: 7 passed; `--test lock`: passed after the
  blessing). The first gate found a pinned phrase in
  `core/tests/first_order.rs` I had not run, now in the Q8 commit.
- `gate` at the final chain: "gate: passed" (the ratchet within its
  ceilings; its note on `search-qbf-20-2`'s cache misses is a question
  for a timed run, not a failure, and no journey runs code this chain
  changed but the parser's one comparison on `-`).
- `nix flake check`: the first run failed only `treefmt` (files I had
  changed from the shell after `nix fmt`); after `nix fmt`, folded into
  the commits they belong to, "all checks passed!" (`--keep-going`).
- Each commit of the chain builds alone (`cargo check --workspace
  --all-targets` at `kmokzrnw`, `rwuvpows` and `lpnqrkvr` after the
  review's fixes; `zonptzos` is the final code).
- Capped calls of the command for each item at the final binary, as
  shown above.

### The reviewer

`fresh-reviewer` (Fable 5.2 at `medium`, its own definition; it read
the chain, the brief and the prompt, not this session's reasoning, and
probed the built binary on core 12) reported eight points. Seven held
and were fixed in the commit they belong to, each with a case added to
that item's test; one was a borderline it did not count.

- **F1, words out of order in the split tip** (`deduxit prove A + B + C`
  suggested `"|- A C B + +"`): `split` removed the last copy of a
  repeated word. Fixed by taking the sequent's words from the command
  line as written, from the first refused word or the sequent, whichever
  is first; `sequent_at` finds the word clap reads as the sequent by
  marking it.
- **F2, `-o C` dropped from the split tip** (`deduxit prove A, B -o C`
  suggested `"|- A, B"`): the re-parse read `-o C` as `--output C`.
  Fixed by the same reading of the command line, which keeps `-o` as
  the implication as the `parse` path did.
- **F3, `--logic classical` named where the command refuses it** (`seq
  json`, `prove -i`): the flag is now added by `hint::parse`, only where
  the command takes `--logic` and no mode flag that it refuses is given
  (`Command::takes_logic`); `parse_error` keeps the library's hint alone.
- **F4, a path that is not UTF-8 stopped working** (`--file f\377.txt`):
  `main` passed lossy strings to clap. It now parses the OS strings and
  gives the hints a lossy copy. A regression of this session, verified
  by hand (no test: the command line is built of `&str` in the tests).
- **F5, the lines hint on one sequent over two lines**: the hint now
  needs an unexpected `:` or an unexpected word at the start of a line
  past the first; the test has the reviewer's input.
- **F6, `check` showed `⊢ ~a, b` beside a phrase about sides**: a
  classical disproof of a sequent written with hypotheses now names the
  sequent as written, `valid refutation of a ⊢ b (classical): …`
  (`ClassicalSequent::read_one_sided`, the library's own reading of a
  one-sided sequent with its sides), and so does `check`'s line for an
  unchecked one; the manual's and the guide's example of that line
  changed with it. This also settles what I had listed as a proposal.
- **F7, a warning for a Mix at a `⊗` formula**: `split_passes` judges a
  `⊗` formula's split as the `⊗`'s whatever the rule, so the warning now
  comes only for a `⊗` at a `⊗` formula or a Mix at another.
- **F8, the Lambek calculus's `\L` and `\R` listed as they cannot be
  typed** (the session's word splitter takes `\` as an escape): they are
  listed as `\\L (\L)`, and `listed_rules_read_back` now reads every
  shown spelling through the splitter, so a spelling that cannot be
  typed fails it.
- **Not counted by the reviewer:** `prove "A |- "` names `-a`, which
  does not prove it. The note says what `-a` allows; making it exact is
  the proposal below.

### Proposals beyond the items, for the author

- The count equation of a two-sided classical sequent could be phrased
  by sides as the intuitionistic one is (`#⊗ᵣ + #⊸ₗ …`).
- `-h` still has 21 Output flags. Hiding the styling ones (`--style-file`,
  `--font`, `--lambda-notation`, `--lambda-types`, `--kernel`,
  `--lemma`, `--prelude`, `--compact`, `--screens`, `--abbreviate`)
  would bring it near 30. `--settings` stands first, under "Options".
- The `-a` note could be exact by an affine search within a small work
  bound; it is a heuristic now (see Q8).

### What the next sessions must know

- **54w-d (the command's help and messages)**: the new texts are in
  `cli/src/hint.rs` (the tips and hints), `cli/src/prove.rs`
  (`affine_note`), `cli/src/interact/mod.rs` (`unknown_rule`, the split
  warning), `cli/src/argument_parsing.rs` (`every_option!`, the heading
  constants) and the library's `refutation.rs` (`write_unbalanced`'s
  two-sided arm) and parser (`ARROW`). `parse_error` (`lib.rs`) appends a
  flag after a library hint by matching the hint's first words; reword
  `ARROW` or the intuitionistic hints and the match moves with them.
  The headings and the hidden flags are in place, so FC8's layout is
  done; the ordering rule is in `cli.md`.
- **54w-c (README, the manual, the guide)**: README's, the commands
  chapter's, the Rocq, first-order and batch chapters' and the guide's
  refutation lines changed; the commands chapter shows the `-a` note,
  the interactive chapter and the guide's §10 the rule list and the
  warning, the guide's §19 the two new mistakes. The page shows the new
  two-sided phrase once the client takes this library (its e2e tests
  may assert the old one: 54u-c and 54u-d).
- **The behaviour lock**: 54u-a changes it too. The second to land
  rebases onto the first and blesses again (`BLESS=1 cargo test -p
  deduxit-cli --test lock`), in a commit of its own.

## Session 54u-c: the page's prover

Item 3 of `plan/54u-usability.md` (Opus 5.5 at `high`, 2026-11-14), in
the web client's jj workspace `../linlog-prover.github.io-54u-c`
(workspace `step54u-c`) on the client's `main` (`qyxnytmo`, which is
also `main@origin`). Nothing was pushed. This deduxit workspace holds
only this section.

### Outcome

W1, W2, W3, W9, W10 and W15 are built in the client, each with its
e2e tests and, where the item has logic outside the island, its unit
tests. The client's `nix flake check` passed at the chain's last
commit, `vqlplqsw`: 584 unit tests and 156 e2e tests (see "The
checks"). The fresh reviewer found one real bug in
W15 (an ordinary proof's Go back landed without its note) and three
smaller points; all four are answered below, the bug fixed.

- **W1, reasons in view.** Every reason is a line under its keyword
  (`Why` in `src/islands/ui.tsx`, class `said-why`), never only a tip:
  - a goal's note (a refused step, a verdict, "Nothing to undo"), whose
    keyword is now a plain `span.kw`, not a button with a tip;
  - the split chooser's Passes and Cannot close;
  - the stage's banner ("Proof not restored");
  - the sequent field's parse error;
  - the status popover's verdict;
  - a greyed export item (`.fn .off`, "Close a goal the search refutes
    first.");
  - a greyed engine in Settings (`.why-not`), the greyed Mix switch in
    intuitionistic mode, and the greyed Net tab once pressed
    (`.net-why`, after the review).
  - The saturation refutation's sentence is split: its first sentence
    in view, the rest behind a "more" button (`Said.more` in
    `src/lib/words.ts`).
- **W2.** No Undo beside a refused step. "Other rules" stands there and
  opens the same formula's rules popover (`Note.position` records the
  refused step's formula; `openRules` in `Stage.tsx`).
- **W3.** With Learn off and no tour running, the empty stage offers
  "Start lesson 1" (Learn opened at lesson 1) and "Take the tour" as
  buttons under the syntax line. Neither starts by itself.
- **W9.** On a wide screen the tab's mark reads "Linear (LL) ▾" or
  "Ordinary (LJ) ▾" (`markLabel` in `src/lib/suites.ts`), the fragment
  between the label and the ▾; a phone keeps the short mark. The
  accessible name stays "Logic: …". The restart warning now says
  "Changing the logic" (after the review).
- **W10.** The goal bar says "Open goal 1 of 2" (`openGoal` in
  `words.ts`), the active goal's place among the open goals; a phone
  says "Goal 1". The Steps list keeps deduxit's ids ("⊗ on goal 2").
- **W15.** When the search refutes a goal that is not the sequent
  itself, the note adds "This goal has no proof. The sequent may still
  have one: an earlier step lost it. Go back takes you to the last step
  that could still be proved." and a Go back button beside the verdict.
  - Go back (`prover.goBack` in `state.tsx`, the walk `wayBack` in
    `src/lib/steps.ts`) asks each goal below the refuted one on its
    branch, nearest first, whether it still has a proof: a replay
    without the steps above that goal, then one search within the
    budget. The sequent's own kept verdict (`Proof.verdict`) replaces
    the search when the sequent itself is asked.
  - The first goal with a proof is shown open. The steps taken back are
    one undo entry, which Redo takes again. The search's own graft is
    never kept.
  - With no such goal, the note says "No step to go back to" (the
    sequent has no proof) or "No way back found" (the budget decided
    nothing).
  - When the sequent is already known to have no proof, the note says
    "The sequent itself has no proof, so no step lost one." and offers
    no Go back.
  - Linear and ordinary proofs alike.

### The client's commits

On the client's `main` (`qyxnytmo`), oldest first, by change id:

| change | what |
|---|---|
| `ypqvmzto` | W1, W2, W3, W9, W10: the reasons in view, Other rules, the empty stage's buttons, the tab's label, the goal bar; the unit tests (`markLabel`, `openGoal`, the saturation's `more`) and the e2e tests; README and the client's `web.md` |
| `ozwnvxwn` | W15: `wayBack` and its unit tests, `goBack`, the lost-goal words and their unit tests, the e2e tests in linear and ordinary logic, the `targets.spec.ts` step |
| `vvtprlnz` | the reviewer's finding 1: Go back in an ordinary proof whose lost step has a structural run under its rule |
| `wrnqvwpw` | the reviewer's findings 2 and 3: the Net tab's reason as a line, "Changing the logic" |
| `vqlplqsw` | the lost goal's and the long reason's target measures as a test of their own at both widths, out of the walk of every control |

### Decisions

- **One `Why` component for every reason.** It renders the sentence and
  a "more" for a long one, so each place says a reason the same way.
  Its "more" state is keyed by the text, so a new reason in the same
  place starts closed without an effect that copies signals (the
  client's `island.md`).
- **A toggled fragment inside a `Show` lost its nodes.** The first
  `Why` put a fragment (a space, a nested `Show`, the button) inside
  `Show`'s callback; pressing "more" removed the button and inserted
  nothing (seen in the built page). Each branch is now one element.
  The comment in `ui.tsx` says so.
- **Go back searches in the session, not the goal's text.** A replay
  that reopens the goal, then `close` on it, is the page's own meaning
  of "this goal has a proof": it holds with cuts, eigenvariables and
  open witnesses, where a sequent written from the goal's formulas
  would not. The cost is one replay per goal asked; the walk stops at
  the first goal with a proof.
- **A goal left undecided does not stop the walk.** The walk goes on to
  the next goal below and lands only where the search found a proof,
  which is what "the last goal that still had a proof" asks. Going back
  further than needed loses steps but never leaves the visitor on a goal
  without a proof.
- **The verdicts kept are the sequent's own (`Proof.verdict`).** The page
  keeps no verdict of an inner goal once a step expands it, so the walk
  searches those. A kept "unknown" is searched again, since its budget
  may differ.
- **The goal's ids stay where the visitor names a goal across steps.**
  The Steps list, the formulas' accessible names ("formula 1 of goal
  3") and the live region keep deduxit's ids; only the bar counts among
  the open goals. W10 names the bar alone.
- **The logic dialog's group names "Calculus" and "Suite" stay.** 54w-a
  kept them because the inner groups are already named "Logic" and
  renaming the outer one gives two groups of one name; the reviewer's
  finding 2 is answered by the visible text only (below).
- **Reasons beyond the four the brief names.** The brief lists "a
  refused step, a verdict, Cannot close, a greyed export item" as
  examples of "every reason". The field's parse error, the banner, the
  status, the greyed engines, the Mix switch and the Net tab got the
  same line, since each is a reason a touch never saw.
- **The Net tab's reason shows on a press**, not always: a line under
  the stage's controls for every unfinished proof would stand there for
  most of a proof. The tab is `aria-disabled`, so it still takes a
  press or Enter.
- **What the decisions rest on:** the usability review's citations
  (NN/g's tooltip guidelines and empty-state article: no tip holds what
  a task needs, help is offered and never imposed), taken as the review
  gives them and not read again; the client's `island.md` for Solid 1.9
  idioms. Playwright's `toHaveText` reads `textContent` unless
  `useInnerText` is set, so the tests of labels with a wide-only and a
  phone-only span set it.

### The checks

| check | result |
|---|---|
| Vitest (`tests`, `--maxWorkers=3`, cores 13-15) | 584 passed (579 before: 5 new tests, `markLabel`, `openGoal`, the lost-goal words, and two of `wayBack`) |
| `biome lint --error-on-warnings`, `astro check`, `tsc --noEmit` | 0 errors |
| e2e, the whole suite in the dev shell (`--workers=3`, cores 13-15) before the review | 151 passed, 3 failed: `modes.spec.ts` and `invertible.spec.ts` read the mark's text and the note's button, which moved (changed with them), and `prover.spec.ts`'s link test, which opens a fresh page and timed out under load; all three passed when run again |
| e2e of the specs the review's fixes touch (`--workers=2`) | 66 passed, then the Net test again with the phone and dismiss specs: 10 passed |
| `targets.spec.ts` after the walk's fix (`--workers=3`) | 10 passed; the walk takes 17 seconds, the new reasons test 4 |
| the client's `nix flake check --keep-going --max-jobs 4 --cores 8`, under the machine lock, at `ozwnvxwn` | exit 0 |
| the same at `wrnqvwpw` | exit 0: 584 unit tests, 153 e2e tests passed and 1 flaky: `targets.spec.ts`'s desktop walk of every control timed out at 60 seconds on the step this session had added to it, and passed on its retry; the run at `ozwnvxwn` had the same flaky test |
| the same at `vqlplqsw`, the chain's last commit, after moving the two new states out of the walk | exit 0: 156 e2e tests passed, none flaky; the unit check's sources were unchanged since the run at `wrnqvwpw`, so its result (584 passed) came from the store |

### The review

The `fresh-reviewer` agent (Fable 5.2 at `medium`) read the first two
commits against the brief, the prompt and the notes, and drove the
built page with a scratch Playwright script on cores 13-15.

1. **Go back in an ordinary proof showed no note when the lost step
   had a structural run under its rule** (`~a, c |- c` in LJ, `¬L`,
   then Close goal and Go back: the root reopened, the dock had no
   note). Upheld. Ordinary `retract` climbs through the step's run to
   the step's goal, so the goal reopened was below the path the walk
   asked, and the note went to a path the new session lacked; the same
   goal was also searched twice. Fixed in `vvtprlnz`: the walk keeps
   each reopened goal's answer by its path, searches it once, applies
   the kept verdict wherever the sequent itself is reopened, and notes
   the goal it shows; an e2e case in `ordinary.spec.ts` drives it.
2. **"One name, Logic": the dialog's outer group is labelled
   "Calculus", and the restart warning said "Changing the calculus".**
   Upheld for the visible text: "Changing the logic restarts this
   proof" (`wrnqvwpw`). The group's accessible name stays, as 54w-a
   decided and recorded (two groups named "Logic" otherwise).
3. **The greyed Net tab's reason was still only a tip.** Upheld, since
   `web.md`'s new rule says every reason is a line: a press on the tab
   shows it until Tree is pressed (`wrnqvwpw`), with an e2e step.
4. **W2 and W3 have e2e tests only, no unit test.** True, and left so:
   neither item has logic outside the island (W2 is a button calling the
   popover's own opener, W3 two buttons setting view state), and the
   client has no unit tests of islands at all. A unit test would have
   to extract a function only for the test. The brief's "unit and e2e
   tests for each" is met for W1, W9, W10 and W15, and by e2e alone for
   W2 and W3.

### Departures from the brief

- W2's and W3's tests are e2e only (review finding 4). (departs from
  the design)
- The verdict sentences 54w-a wrote are unchanged; the lost-goal line
  is added under them. In an ordinary proof a refuted inner goal reads
  "Not valid" with the sentence "The sequent has no derivation in LJ…",
  over the line "The sequent may still have one: an earlier step lost
  it." The two disagree about "the sequent". A proposal below.

### Proposals beyond the items

- Make the ordinary "Not valid" sentence speak of the goal ("This goal
  has no derivation in LJ"), so that it reads right at an inner goal
  (`ordinaryVerdict` in `src/lib/ordinary.ts`).
- When Go back searches the sequent itself and finds no proof, keep
  that as the proof's verdict (`Proof.verdict`), so that the tab's mark
  and the status say it too.
- The formulas' accessible names and the live region still number goals
  by deduxit's ids while the bar counts among the open ones. A screen
  reader hears both numberings; naming a formula's goal by its place
  among the open goals would match the bar but not the Steps list.

### What the next sessions must know

- For 54u-d (Learn, Practice, Export), on the client's chain after
  `wrnqvwpw`:
  - Export's greyed items now show their reason as `span.off` inside
    the item's `.fn`; W7's and W8's new items get it for free through
    `off`.
  - Every reason is a line (`Why` in `ui.tsx`); a new message with a
    reason uses it, and a long one takes `more`.
  - The empty stage's "Take the tour" and "Start lesson 1" contain
    "tour" and "lesson": a test that finds the band's Tour by name
    needs `exact: true` (`learn.spec.ts` now passes it).
  - The goal bar's text is "Open goal N of M"; tests read it with
    `toHaveText(…, { useInnerText: true })`, since the bar holds a
    wide-only and a phone-only span. The tab's mark likewise
    (`.pt-mark .mk`).
  - A note's keyword is a `span.kw`, no longer a button: a test finds
    the note as `.dock .note` and its sentence with `toContainText`.
  - W14 renames the storage keys; nothing of this session reads or
    writes one.
- The client's `.claude/rules/web.md` records each of these, with Go
  back's walk and the ordinary run case.
- The dev shell of a fresh client workspace built biome from source for
  twenty minutes under the machine lock (nixpkgs had no cached build at
  the client's pin); a later workspace on the same pin reuses it.

## Session 54u-e: checking a certificate

Session 5 of `plan/54u-usability.md` (Opus 5.5 at `high`, 2026-11-14,
cores 7-9), in the jj workspace `../deduxit-wt-step-54u-e-1160d0`: Q11's
first half (`plan/notes/usability.md` §6). Nothing was pushed.

### Outcome

`nix run github:deduxit/deduxit#check-certificate -- proof.v` checks a
certificate the command wrote, with nothing installed but Nix. The flake
check `check-certificate` runs it on a certificate of each kernel and on
four files it must refuse. The manual's Rocq chapter leads with it and
the guide for AI models names it.

| change | what |
|---|---|
| `kvqyoyul` | the app, its NanoYalla build and the flake check (`modules/check-certificate.nix`, `modules/check-certificate.sh`), the flake rules and CLAUDE.md |
| `nzmuxnpp` | the manual's Rocq chapter, the guide's section 18, the status chapter, the changelog |
| `rwtnqorq` | the reviewer's findings: the lemmas read from Rocq's `.glob` file, sections and modules refused, odd file names |
| this one | this report |

The app's output on certificates the command wrote, run from an empty
directory outside the repository (the scratchpad), with the workspace's
flake:

```
$ nix run …#check-certificate -- proof.v      # deduxit prove --format rocq --standalone "A * B |- B * A"
Checking proof.v with Rocq 9.1.1 and NanoYalla 1.1.3.
Lemma certificate: proved, with no axiom.
The certificate holds.                         # exit 0
$ nix run …#check-certificate -- mp.v         # deduxit prove -i … "A, A -o B |- B"
Checking mp.v with Rocq 9.1.1 and rocq-deduxit 0.3.0.
Lemma certificate: proved, with no axiom.
The certificate holds.                         # exit 0
$ nix run …#check-certificate -- peirce.v     # deduxit prove --logic classical … "((a -> b) -> a) -> a"
Checking peirce.v with Rocq 9.1.1 and its standard library.
Lemma certificate: proved, with the excluded middle (classic) as its one axiom.
The certificate holds.                         # exit 0
```

The directory held the three `.v` files afterwards and nothing else.

### Decisions, with their reasons

- **All three libraries on the load path at once; the file's imports
  pick.** `ROCQPATH` names the standard library, NanoYalla and the
  pinned rocq-deduxit, so Rocq resolves `From NanoYalla` or `From
  Deduxit` itself and no flag names a kernel. The app reads the import
  lines only to say which kernel it checks against. Picking one library
  per file would add a branch that can only be wrong.
- **NanoYalla built as a Rocq library here.** No package of it exists.
  The derivation compiles the two files a certificate needs (`nanoll.v`,
  `macroll.v`) under `user-contrib/NanoYalla`, with the kernel's LGPL
  licence beside them, since CI pushes the closure to the public cache.
  The cut files are left out: they hold an axiom and no certificate uses
  a cut. `rocq.nix` keeps its own copy (proposal below).
- **`writeShellApplication`, `inheritPath = false`, versions from Nix.**
  This is nixpkgs' current way to package a script: shellcheck at build,
  `set -euo pipefail`, `runtimeEnv` for the versions (Rocq's from
  `rocq-core.version`, NanoYalla's from its README as `rocq.nix` reads
  it, rocq-deduxit's from the package). The app is flake-parts'
  `apps.<name>` with `meta.description`, which `nix flake check` asks
  for (read in flake-parts' `modules/apps.nix` at the pinned revision).
  The script is a file of its own beside the module, so treefmt's shfmt
  and shellcheck format and check it.
- **Compiled in a temporary directory under a fixed name.** Rocq names a
  module after its file, and a file name may be no identifier
  (`modus ponens.v`) or a keyword (`if.v`). The copy is `certificate.v`,
  Rocq's messages get the user's file name back, and nothing is written
  beside the user's file.
- **A lemma that rests on an axiom is refused** (departs from the
  design). The brief asks for exit 0 when the file compiles, but a proof
  left `Admitted` and an `Axiom` compile in silence. A checker that
  passes them checks nothing. So after the compile, each lemma goes to
  `Print Assumptions`, as the check `rocq-deduxit` does for its
  certificates. Only "Closed under the global context" passes, or the
  standard library's excluded middle alone in a file that imports
  `Classical_Prop`. Those are the classical certificates over `Prop`,
  which the check `rocq` compiles too. Every certificate of the command
  that was tried passes. Before the fix that meant the reviewer's 85: the
  29 that `rocq.nix` and `rocq-deduxit.nix` generate, the 9 NanoYalla
  and ordinary snapshots, and the 47 snapshots for rocq-deduxit. After
  it, the 84 of them in its scratch directory.
- **The lemmas come from Rocq's `.glob` file** (after the reviewer's
  findings 1 and 2). It lists every definition from Rocq's own parse,
  with its kind. The app takes `prf` (every theorem keyword, probed) and
  `def` at the top level as the lemmas, and refuses every other kind but
  binders and references: a section (its hypothesis becomes a premise of
  the lemma, so no assumption shows), a module or module type (a lemma
  in a functor is no constant of the file), a variable or an axiom. A
  grep for `Section` would refuse a genuine certificate with an atom
  named `Section`, which Rocq accepts as an identifier (probed).
- **Exit statuses 0, 1, 2**: holds, refused, unreadable. These are the
  command's for proved, unprovable and error.
- **The manual's example is a `readme-check: skip` block.** README's test
  has no rule for `nix`, and `cli/` is 54u-b's. The block shows the
  app's real output, kept equal to it by hand (a note in
  `.claude/rules/flake.md`). The flake check runs the app.

### The checks

- `nix build .#checks.x86_64-linux.check-certificate`: passes at
  `rwtnqorq`. It accepts a NanoYalla certificate, a rocq-deduxit one and
  a classical one over `Prop`, each line naming its kernel. It expects
  exit 1 for a statement its proof does not prove, a proof left
  `Admitted`, a lemma proved in a section from `False`, and a fragment
  without import lines, whose message names `--standalone`.
- `cargo test -p deduxit-cli --test readme`: 7 passed, after the manual
  and the guide changed.
- `nix build` of `manual`, `manual-links`, `conventions`, `typos` and
  `treefmt`: pass.
- `nix flake check` under the machine lock: "all checks passed!" at
  `nzmuxnpp`, and again at this report's change, the chain's last. The app is
  checked as `apps.x86_64-linux.check-certificate` with no warning.
- The app from a clean directory: the output above, at `rwtnqorq`.
- The gate was not run: the chain changes no Rust code. CLAUDE.md asks
  for it on a commit that touches code.

### The reviewer

`fresh-reviewer` (Fable 5.2, medium) ran once on `kvqyoyul` and
`nzmuxnpp`. It ran the app on 85 genuine certificates and on adversarial
files. Its three findings all held, and each is fixed in `rwtnqorq`:

1. **A lemma proved in a `Section` from a `Hypothesis absurd : False`
   exited 0.** The section discharges the hypothesis into the lemma's
   statement, so `Print Assumptions` shows none. Fixed: the `.glob` file
   shows the section, which is refused. The flake check now holds this
   case.
2. **The lemma names came from a `sed` over lines**, so `Lemma a … Qed.
   Lemma b … Admitted.` on one line, a name on the next line,
   `#[local] Lemma` and `Definition … Admitted.` all passed with 0.
   Fixed: the names come from the `.glob` file. All four witnesses now
   exit 1.
3. **File names**: `-p.v` made `grep` read an option, so the kernel line
   was wrong and the excluded middle was refused (exit 1). A module
   named `if` or `Type` failed at `Require`. Fixed: `grep -e … --`, and
   the copy is always `certificate.v`. Both now exit 0.

After the fix, the reviewer's 84 generated and snapshot certificates in
its scratch directory all exit 0. Its adversarial files exit 1, but
`comment.v`, a genuine certificate with a lemma's name in a comment,
which exits 0 as it should.

### What the next sessions must know (54w-c)

- The Rocq chapter (`docs/src/rocq.md`) now has two headings: "Check a
  certificate", which leads, and "The kernels", which holds the rest.
  "To check a certificate without Nix" introduces the hand installs.
  54w-c rewrites the chapter around the app. Keep the opam lines after
  it, and the plain statement that the package is not yet in the
  archive (FR1).
- The example's first line carries versions (Rocq 9.1.1, NanoYalla
  1.1.3, rocq-deduxit 0.3.0). A bump of nixpkgs' Rocq or of either
  kernel changes it, and no test catches that: the block is skipped.
- The guide names the app in `reference.md` section 18 only.
  `SKILL.md` stands at 99 lines, at its budget's edge (fewer than 100).
- `github:deduxit/deduxit/TAG#check-certificate` checks against a
  release's pins. The manual says so without naming a tag, since no
  release holds the app yet (0.4.0 will).
- What the app cannot see: a file that gives the statement's own names
  another meaning (a `Definition ll`). The statement is read from the
  file. The manual does not say this; 54w-c may want a sentence.

### Proposals beyond the item, for the author

- `rocq.nix` could compile its certificates against the NanoYalla
  package `check-certificate.nix` builds, rather than its own copy, so
  the kernel is built once. It is a change of the check `rocq`, outside
  this item.
- `packages.check-certificate` beside the app would let `nix shell` and
  `nix profile install` take the checker. The item asks for the app
  alone.

## Session 54u-d: Learn, Practice and Export

Item 4 of `plan/54u-usability.md` (Opus 5.5 at `high`, 2026-11-14), in
the web client's jj workspace `../deduxit.github.io-54u-d` (workspace
`step54u-d`), its chain rebased onto the client's `main@origin`
(`wxxpuxrx`, which moved during the session). Nothing was pushed. This
deduxit workspace holds only this section.

### Outcome

W5, W6, W7, W8, W11, W12, W13 and W14 are built in the client, each with
its e2e test, and W13 and W14 with a unit test as well.
The client's `nix flake check` passed at the chain's last commit,
`runpsyxs`: 586 unit tests and 162 e2e tests passed, one more e2e test
passed on its retry (a test this chain does not touch; see "The
checks"). The fresh reviewer found two gaps, both fixed.

- **W5, a wrong check answer offers its problem.** When the chosen
  option names a store problem (`problem` in the lesson's hinge), the
  feedback offers "Try ⊢ A, A yourself" first, as the primary button. It
  opens the problem the way a practice row's Prove does. "Try the faded
  proof again" follows it.
- **W6, answer buttons on Learn's practice rows.** Each row has "Has a
  proof" and "No proof" (a group named "Does <title> have a proof?",
  each button `aria-pressed`) beside Prove. A press answers at once, as
  a status: "Right" or "Not quite", then "It has a proof." or "It has no
  proof.", the problem's explanation and Watch. The "Answer" disclosure
  is gone; the answer stays while the lesson is open and may be changed.
- **W7, exports that compile alone.** The export settings now carry
  deduxit's `Form`: LaTeX and Typst stay fragments (`fragment`), and each
  is followed by an indented "Whole document" item (`standalone`). Every
  Rocq file, the proof's and the refutation's ("Not derivable"), is
  `standalone`, so it starts with its `Require` line. Each detail names
  what the file needs: ebproof, cmll and amssymb for LaTeX, installed
  for the whole document; curryst for Typst, which Typst downloads; for
  Rocq, NanoYalla or rocq-deduxit as before, or no library over `Prop`
  for an ordinary proof.
- **W8, a link to the bare sequent.** The menu's link item is now a
  group "Share a link" with two items: "This proof" ("A link to this
  proof as it stands") and "The sequent" ("A link to the sequent alone,
  with no step taken, as an exercise"). The second writes the link
  without its session.
- **W11.** Practice's text says "A problem counts as proved when you
  close every goal yourself; Hint may help." The rule itself is
  unchanged (steps Hint applied count, the search's closes do not).
- **W12, lesson 1's rewrite.** Lesson 1 has a new optional part,
  `rewrite`, one sentence shown in the worked proof's section: "You typed
  `A ⊢ A`, and the stage shows `⊢ A⊥, A`: a hypothesis left of `⊢` moves
  to the right as its dual." A button turns the Two-sided switch on (the
  stage redraws `A ⊢ A`), a line says lessons 1 to 5 use the one-sided
  form, and "Turn it off" turns it off. The lesson's change holds for
  the visit alone and is never stored. If the lesson turned it on, its
  next step and leaving the lesson turn it off again, so the step
  reasons, written one-sided, match the drawing. Lessons 1 to 5 stay
  one-sided.
- **W13, openings below the worked proof.** An opening may now hold one
  thematic break (`***`). The part before it stands above the worked
  proof, the part after it right below the worked proof (`Written`'s
  `part`). See "Decisions" for which lessons moved and why.
- **W14, the storage keys.** `linlog-settings`, `linlog-theme`,
  `linlog-practice`, `linlog-proofs:<visit>` and
  `linlog-proofs-previous` are now `deduxit-…`. `migrate`
  (`src/lib/storage.ts`), the first statement of the island, moves each
  old key's value to its new key and deletes the old key; a value
  already under the new key wins. The pre-paint theme script reads
  `deduxit-theme`, else `linlog-theme`, so the first paint after the
  release keeps a pinned theme. The service worker's cache prefix is
  `deduxit-site-`, and its activation also deletes every `linlog-site-`
  cache. The privacy page lists the new keys. Kept for one release, as
  the item says; the client's `web.md` says to remove the three together.

### The client's commits

On the client's `main@origin` (`wxxpuxrx`), oldest first, by change id:

| change | what |
|---|---|
| `vmoymwou` | W14: `storage.ts` and its unit test, the keys renamed, the pre-paint fallback, the worker's old caches, the privacy page; the e2e tests that name the keys changed with them, one e2e test that loads old keys, one that the worker deletes an old cache |
| `zovprrmr` | W7 and W8: the export forms and the share group; the export e2e test downloads both whole documents and checks every file's form and both links; README and `web.md` |
| `tmrkuzyu` | W11: Practice's text; an e2e test that a proof with a step Hint applied is marked proved and that the text says the rule |
| `ywrxouqs` | W5 and W6: the check's "Try … yourself", the practice rows' answer buttons and feedback; the e2e tests that opened "Answer" changed with it |
| `vwwmmymn` | W12 and W13: the lesson schema's `rewrite`, lesson 1's sentence and buttons, the split openings of lessons 7 to 10; a unit test of the split, e2e tests of both |
| `krukxxuu` | the whole-document rows' label indented, their detail kept in the column (seen in the browser) |
| `ylvtprxt` | a pre-existing race in the link test fixed at its cause (below) |
| `twqsytno` | the lint fix: the share group and a practice row's answers are fieldsets, Practice's heading's `font` first |
| `wuvloosz` | the review's finding 1: Learn's answers and lesson 1's rewrite measured at both widths |
| `runpsyxs` | the review's finding 2: the switch lesson 1 turns on holds for the visit alone |

### Decisions

- **W13: four openings moved, in part.** Each lesson's opening was read
  against what its worked proof shows (the steps and their reasons).
  An opening moved only where a part of it holds more than that, and
  only that part, so that what the worked proof needs to be read stays
  above it (the cut of `plan/notes/usability.md`'s W13: Sweller and
  Cooper 1985, Renkl 2014, as the review cites them; no rewording):
  - Lesson 7: the worked proof (and the attempt before it) shows that a
    stuck attempt proves nothing, which stays above. The three answers
    (Proved, Unprovable, Unknown) and "No proof needs a reason", with
    its list, are what the gallery and the Unknown part show, so they
    moved below.
  - Lesson 8: the worked proof shows a derivation and offers its net;
    paragraph 1 (what a net is) stays. The switching condition and the
    Net tab's reach (MLL, MELL's boxes, essential nets) are what the
    gallery and the check use, so they moved.
  - Lesson 9: the cut stays; cut elimination and its price are what the
    Eliminate part right below the worked proof shows, so they moved.
  - Lesson 10: the syntax and the rules stay, since the worked proof
    needs them; "∀ is like &, ∃ like ⊕" and `!` over `∀` are more than
    the proof shows, so they moved.
  - Kept: lessons 1, 3, 4 and 5 (short, every sentence shown by the
    proof), 2 (its second paragraph explains `A⊥ ⅋ B⊥`, the worked
    proof's first goal), 6 (the intuitionistic reading is needed to read
    the worked sequent) and 11 (the translation is needed to read the
    image proved).
- **W13's mechanism is a thematic break in the Markdown.** The opening
  stays one text in one file, rendered once; `Written` splits the
  clone at its first `<hr>`. `---` would end the frontmatter in the
  tests' reader, so the break is `***`.
- **W12 restores the switch the lesson turned on.** The switch is a
  stored setting of the page. The lesson's change is for the visit
  alone (`setSettings(change, true)`, after the review), and the lesson
  turns it off at its next step and when it closes, but only if it
  turned it on itself, so a visitor who keeps Two-sided on keeps it.
- **W6's answer is not stored.** Nothing asks to keep it across visits,
  and Practice's stored progress is for proofs, not guesses.
- **W8's labels.** The group is named "Share a link", its items "This
  proof" and "The sequent", their details "A link to this proof …" and
  "A link to the sequent …": the item's names, laid out so that the
  label column (96 pixels) holds them on one line.
- **W7's whole documents follow their fragments as sub-rows** with
  "Whole document" as the label; the detail says which format, so a
  screen reader hears "Whole document, A LaTeX file …".
- **W14's migration runs in the island's body,** before `createProver`
  reads the settings and Practice its progress; the pre-paint script
  only reads the old theme key, since it runs on every page and its
  hash is in the policy.
- **The link test's race.** `prover.spec.ts`'s "the link in the fragment
  restores the proof" failed one run in three at the chain's head and
  at `main@origin` alike (`--repeat-each=3`). Its `toHaveURL(/&d=/)`
  already matched the link written for the sequent before the step, so
  the copied URL could hold no step. The test now waits for the URL to
  change after the step; it passed six of six. 54u-c's report saw the
  same test fail once "under load".

### The checks

| check | result |
|---|---|
| Vitest in the dev shell (`--maxWorkers=3`, cores 13-15) | 586 passed (584 before: `storage.test.ts` and the openings' split in `lessons.test.ts` are new) |
| the whole e2e suite in the dev shell (`--workers=3`, cores 13-15), before the review | 160 passed, 1 failed: the link test, a race present on `main@origin` too (below), fixed in `ylvtprxt` |
| the client's `nix flake check --keep-going --max-jobs 4 --cores 8`, under the machine lock, at `ylvtprxt` | exit 1: `lint` failed on three Biome errors (two `div role="group"` that `useSemanticElements` wants as fieldsets, and a `font` shorthand after `line-height` in a rule this chain merged; my own lint run in the dev shell had been misread); unit 586 and e2e 161 passed. Fixed in `twqsytno` |
| the same at `twqsytno` | exit 0: 586 unit, 160 e2e passed, 1 flaky |
| the same at `runpsyxs`, after the review's fixes | exit 1: the new phone measure of Learn's answers measured the stage before its two-sided redraw (a 31-pixel button, on both tries). The test now waits for the drawing (`wuvloosz`); 10 of 10 with `CI=1` locally |
| the same at `runpsyxs` again, the chain's last commit | exit 0, "all checks passed!": 162 e2e tests passed and 1 flaky; the unit check's sources were unchanged since `twqsytno`, so its result (586 passed) came from the store |
| `biome lint --error-on-warnings`, `astro check` | 0 errors at the last commit |

The flaky test both times was `ordinary.spec.ts`'s "the structural
switch with a goal open marks and buttons that goal in the new drawing",
which this chain does not touch; it passed 6 of 6 in the dev shell
(`--repeat-each=6`). It is left as a follow-up.

### The review

The `fresh-reviewer` agent (Fable 5.2 at `medium`) read the chain up to
`ylvtprxt` against the brief, the prompt and the notes, and drove the
built page with scratch Playwright scripts on cores 13-15 (old keys
migrated with a real shelf, every export downloaded in three logics,
both links, lesson 4's check, the split openings).

1. **The target-size walk had no step for the new states** (lesson 1's
   rewrite buttons, "Try … yourself", a practice row answered), which
   the client's `web.md` asks for every new control. Upheld, though
   the reviewer measured them all at or above 44 and 24 pixels. They are
   measured now in a test of their own at both widths, as 54u-c did to
   keep the walk under its timeout (`wuvloosz`). The flake check then
   caught a race in that test (above), not a small control.
2. **A switch the lesson turned on stayed on after a reload,** since
   the setting is stored and the lesson's memory of turning it on is
   not. Upheld. `setSettings` takes a `visitOnly` flag: the lesson's
   change holds for the visit, and the value before it is what is
   stored until the visitor sets the switch themselves (`runpsyxs`);
   the W12 e2e test checks the stored settings.

The reviewer found W5, W6, W7, W8, W11, W13 and W14 sound, and every
changed assertion following a moved text, none weakened.

### Departures from the brief

- W8's items are labelled "This proof" and "The sequent" under the
  group "Share a link", their details "A link to this proof …" and "A
  link to the sequent …", rather than items named "Link to this proof"
  and "Link to the sequent" (the 96-pixel label column). (departs from
  the design)
- Three commits outside the items: the link test's race (`ylvtprxt`),
  found failing on `main@origin` as well, and the lint fix
  (`twqsytno`) and the visit-only setting (`runpsyxs`) that the checks
  and the review asked for.

### Proposals beyond the items

- Lesson 2's second paragraph introduces the classical rewrite (`A |- A`
  is typed for `⊢ A⊥, A`) as if new; lesson 1 now shows it. 54w-c could
  shorten it to De Morgan and the double dual.
- Learn's practice answer could be kept per lesson for the visit, as the
  hinge's choice is not either; nothing asks for it today.

### What the next sessions must know

- For 54w-c, the page's texts this session added or moved, each in the
  writing rules' form and open to its rework: the export details
  (`BottomBand.tsx`'s `items`), the share group, lesson 1's `rewrite`
  and the switch's line (`Learn.tsx`), the practice feedback ("Right",
  "Not quite", "It has a proof."), Practice's text, and the four split
  openings.
- A test that finds a lesson's practice card's status must name it
  (`getByRole("status").filter(…)`): a card answered has two.
- A formula's button keeps its one-sided name while Two-sided is on;
  a test reads the drawing's sequent from `.drawing svg title`.
- Text rendered from Markdown has curly apostrophes (`deduxit’s`): a
  test finds it by a regular expression without the apostrophe.
- The `linlog-*` migration (`src/lib/storage.ts`, the `OLD_THEME_KEY`
  fallback in `theme.ts`, `OLD_PREFIX` in `sw.js`) is removed after one
  release, all three together.
