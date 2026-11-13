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
