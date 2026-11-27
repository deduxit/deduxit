# The audit's criteria (step 86)

What the audit of the Rust and Rocq libraries judges against, and what
the refactor steps judge their own work against. Every finding cites one
criterion by its identifier. `method.md` says how findings are found,
verified and ranked; `baselines.md` holds the numbers the criteria refer
to.

Every source below was read on 2026-11-27 from the page itself, through
fetch agents that returned each page's content with its URL; nothing is
from memory. Where a page was read only in part, the section says so.
The criteria are paraphrases in this project's terms, with the source's
own identifier or section kept so that a finder can look the rule up.

## How a criterion is used

- A finding has a file and a line, one criterion's identifier, and a
  witness: a snippet, a measurement, a failing use, or the duplicate it
  repeats. A claim without a witness is no finding.
- **What a machine already settles is no finding.** A lint that the
  workspace turns on, a check of `nix flake check`, a test: a finder
  cites their failure as evidence and does not re-argue what they
  decide. What the workspace *allows* is in scope (see E7).
- **The standing criteria still hold.** Step 28's rubric
  (`plan/notes/audit-rubric.md`) has the author's criteria of 2026-10-03
  (A1 to A6), the standing ones (S1 to S19: bounded time and memory,
  sound verdicts, no panic on input, nothing recurses over a formula,
  one error family, wasm stays possible, the deterministic search, the
  behaviour lock) and the AIP rules for the JSON wire forms. They were
  not read again from their sources this session. A refactor may not
  break one; a finding may cite one.
- Identifiers: `C-…` the Rust API Guidelines' own, `E` idiom, `P`
  performance, `X` extensibility, `L` the command line, `T` text, `Q`
  Rocq, `B` what the request implies besides, `D` what the author has
  decided already.

## D. Decided by the author already

- **D1. No logic is the default in 0.9.0** (the author, 2026-11-27,
  through the planning session). Every command that reads a sequent
  requires the logic to be named, where today `prove` without `--logic`
  or `--system` reads linear logic. The API proposal designs the flags
  in the most idiomatic current form (L6, L8); it is not a question to
  put again. Its form, decided by the author later that day: the suite
  is a subcommand under the verb (`prove ordinary`, `prove linear`,
  `prove modal`; `api-proposal.md`, P1). Whether a suite then has a
  default member (classical, classical, K) is the one choice P1 leaves
  to the author's sign-off.
- **D2. The API may change significantly**; the next release is 0.9.0.
  A break is no cost by itself. It needs a reason, a line in the
  changelog and a migration note (B3).
- **D3. The exit statuses stay**: 0 proved or valid, 1 unprovable or
  invalid, 2 error, 3 unknown, 130 at a second Ctrl-C.
- **D4. No suite is the main one** (`writing.md`, rule 3). Where an
  order is needed: ordinary logic, linear logic, modal logic,
  lambda-terms.

## C. The Rust API (the Rust API Guidelines' checklist)

Source: <https://rust-lang.github.io/api-guidelines/checklist.html> and
its eleven chapter pages (`naming.html`, `interoperability.html`,
`macros.html`, `documentation.html`, `predictability.html`,
`flexibility.html`, `type-safety.html`, `dependability.html`,
`debuggability.html`, `future-proofing.html`, `necessities.html`), all
loaded in full. The checklist has 54 items in 11 sections; an item's
anchor is its identifier in lower case
(`…/naming.html#c-case`). They apply to the public surface of `deduxit`
and, where it is a library, `deduxit_cli`.

| section | items, and what each asks here |
|---|---|
| Naming | **C-CASE**: RFC 430 casing; in a type name an acronym is one word (`Uuid`, not `UUID`). **C-CONV**: `as_` is free and borrowed to borrowed, `to_` is expensive, `into_` consumes; a wrapper gives its value by `into_inner`. **C-GETTER**: no `get_` prefix; `get` only where one thing can be got. **C-ITER**: a collection has `iter`, `iter_mut`, `into_iter`. **C-ITER-TY**: an iterator type is named after its method (`walk` returns `Walk`). **C-FEATURE**: a feature is named for what it adds, without `use-` or `with-`, never negative. **C-WORD-ORDER**: one word order across the crate (`ParseError`, `CheckError`, `NetError`: verb or noun first, then `Error`). |
| Interoperability | **C-COMMON-TRAITS**: every public type implements each of `Copy`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`, `Debug`, `Display`, `Default` that fits, since no other crate can add them. **C-CONV-TRAITS**: `From`, `TryFrom`, `AsRef`, `AsMut` where they make sense; `From` only where nothing is lost; never `Into` or `TryInto` by hand. **C-COLLECT**: a collection implements `FromIterator` and `Extend`. **C-SERDE**: data structures implement `Serialize` and `Deserialize`, behind a feature the guideline names `serde`. **C-SEND-SYNC**: types are `Send` and `Sync` where possible, with a test that asserts it. **C-GOOD-ERR**: every error type implements `std::error::Error`, `Send`, `Sync`; never `()`; `Display` in lower case without a final full stop. **C-NUM-FMT**: only for number types with bit operations. **C-RW-VALUE**: a function generic over a reader or writer takes it by value, and its docs say `&mut w` works. |
| Macros | Not applied: the crates export no macro (`grep macro_export core/src` finds none). |
| Documentation | **C-CRATE-DOC**: the crate docs are thorough and have examples. **C-EXAMPLE**: every public module, trait, struct, enum, function and method has an example, within reason; a link to another item's example can do; an example shows why one would use the item. **C-QUESTION-MARK**: examples use `?`, not `unwrap`. **C-FAILURE**: `# Errors`, `# Panics` and `# Safety` sections where a function can fail, panic or is unsafe. **C-LINK**: prose links the items it names. **C-METADATA**: `Cargo.toml` has `authors`, `description`, `license`, `repository`, `keywords`, `categories`; `documentation` only if not docs.rs; `homepage` only for a site that is neither. **C-RELNOTES**: the changelog lists every significant change, marks the breaking ones, and each release has a tag. **C-HIDDEN**: rustdoc shows what a user needs and no more (`#[doc(hidden)]`, `pub(crate)`). |
| Predictability | **C-SMART-PTR**: none here. **C-CONV-SPECIFIC**: a conversion lives on the more specific type. **C-METHOD**: a function with a clear receiver is a method. **C-NO-OUT**: no out-parameter; return a tuple or struct, except to fill a buffer the caller owns. **C-OVERLOAD**: an operator impl means what the operator means. **C-DEREF**: only smart pointers implement `Deref`. **C-CTOR**: constructors are inherent associated functions: `new`, `with_…`, `from_…`; `Default` and `new` agree. |
| Flexibility | **C-INTERMEDIATE**: a function returns the intermediate results a caller would otherwise compute again. **C-CALLER-CONTROL**: take by value what the function must own, by reference what it only reads; no clone inside that the caller could have avoided. **C-GENERIC**: a parameter asks for the least it needs (`impl IntoIterator<Item = T>`, `AsRef<str>`), at the stated cost in code size (see P4). **C-OBJECT**: a trait that may be used as an object is object-safe. |
| Type safety | **C-NEWTYPE**: a newtype keeps two meanings of one primitive apart (ids of different arenas). **C-CUSTOM-TYPE**: an argument is an enum or a struct where a bare `bool`, `u8` or `Option` would hide its meaning. **C-BITFLAG**: a set of flags is a flag type, not an enum. **C-BUILDER**: a value with many options is built by a builder or by `with_…` methods. |
| Dependability | **C-VALIDATE**: a function validates its arguments: by the type first, at run time next, by `debug_assert!` or an `_unchecked` sibling last. **C-DTOR-FAIL**, **C-DTOR-BLOCK**: no destructor fails or blocks. |
| Debuggability | **C-DEBUG**: every public type implements `Debug` (the workspace warns on `missing_debug_implementations`). **C-DEBUG-NONEMPTY**: no `Debug` output is empty. |
| Future proofing | **C-SEALED**: a trait that only this crate may implement has a private supertrait, and its docs say it is sealed. **C-STRUCT-PRIVATE**: fields are private unless the struct is passive data. **C-NEWTYPE-HIDE**: a returned type hides its representation (a newtype or `impl Trait`). **C-STRUCT-BOUNDS**: no derivable bound (`Clone`, `Debug`, `PartialEq`, serde's) on a struct's definition. `#[non_exhaustive]` is the crate's own rule (`core.md`, Decisions). |
| Necessities | **C-STABLE**: no type of an unstable dependency in a public signature (read for `serde`, `rayon`, `batsat`, `resvg`, `krilla`). **C-PERMISSIVE**: not applied; the licence is the author's choice, EUPL-1.2. |

## E. Rust idiom

- **E1. Edition 2024, as the edition guide describes it.** Source:
  <https://doc.rust-lang.org/edition-guide/rust-2024/> and each page
  under it, all loaded. The workspace is on edition 2024 with
  `unsafe_code = "forbid"`, so the pages on `unsafe` do not apply. What
  a reviewer expects:
  - `let` chains in `if` and `while` where they flatten nested `if let`
    (`let-chains.html`; the workspace's `rust-version` 1.92 has them);
  - return-position `impl Trait` without `Captures` or `+ 'a` tricks,
    and `use<..>` only to capture less (`rpit-lifetime-capture.html`);
  - no reliance on a temporary of an `if let` scrutinee or of a tail
    expression living longer than the edition gives it
    (`temporary-if-let-scope.html`, `temporary-tail-expr-scope.html`);
  - patterns fully explicit or fully by default binding, never mixed
    (`match-ergonomics.html`);
  - `for x in boxed_slice` by value, `.iter()` for references
    (`intoiterator-box-slice.html`);
  - `package.rust-version` declared, resolver 3 in the workspace
    (`cargo-resolver.html`); a `rustfmt.toml` that names the style
    edition, since an editor may run rustfmt alone
    (`rustfmt-style-edition.html`);
  - doctests that do not assert on line numbers or type names, since the
    edition merges them into one binary (`rustdoc-doctests.html`).
- **E2. The toolchain's standard library is used where it replaces
  hand-written code**, within `rust-version` 1.92 (the devshell has
  1.100). Clippy's `manual_*` lints settle most of it; the rest is a
  finder's judgement with the std item named.
- **E3. The Rust Style Guide beyond rustfmt.** Source:
  <https://doc.rust-lang.org/style-guide/> with `items.html`,
  `statements.html`, `expressions.html`, `advice.html`, `cargo.html`
  (`types.html` and `principles.html` not read). Comments are line
  comments and complete sentences; `///` over `/** */`, `//!` only for
  a module or the crate, doc comments before attributes, one `derive`
  per item; expression-oriented code (`let x = if … else …;`);
  qualified enum names unless in the prelude; no `#[path]`; a tuple
  struct with more than a few fields becomes a struct with names; an
  enum with many struct-like variants may want its structs factored
  out; `Cargo.toml` keys sorted, `description` last in `[package]`.
- **E4. Documentation as the rustdoc book asks.** Source:
  <https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html>,
  `write-documentation/documentation-tests.html`, `lints.html`. Every
  public item is documented; the summary is one line that stands alone;
  no restating of argument and return types that the signature links; a
  `# Panics` section where a known edge panics; examples are whole
  programs with the irrelevant lines hidden by `#`, use `?` with a
  hidden `Ok::<(), E>(())`, and avoid `ignore` (prefer `no_run`,
  `text`, or hidden lines; an `ignore` gives its reason). The rustdoc
  lints the workspace has on are `missing_crate_level_docs` and
  `unescaped_backticks` beside the defaults
  (`broken_intra_doc_links`, `private_intra_doc_links`, `bare_urls`,
  `invalid_html_tags`, `invalid_rust_codeblocks`,
  `redundant_explicit_links`).
- **E5. Module shape.** A file that a reader cannot hold is split along
  its seams (the rubric's A2). `baselines.md` lists 94 files over 1 000
  lines, six over 3 000 and two over 5 000
  (`linear/search/mod.rs`, `ordinary/interactive.rs`); a finding names
  the seam, not the length.
- **E6. One thing done once** (the rubric's A6): no copied walk, table
  or verdict line; a duplicate is a finding with both places cited.
- **E7. Clippy as the workspace configures it** (`Cargo.toml`,
  `[workspace.lints]`): pedantic is on, with 10 lints set aside on
  purpose and 44 allowed only because they fire today (3 of rustc, 9
  picked from nursery and restriction, 32 pedantic). The 44 are known
  debt, counted per lint and area in `baselines.md` (3 108 sites with
  the ten set aside included). A finding is a lint whose sites should
  be fixed so that it goes to `warn`, or a set-aside lint whose reason
  no longer holds; a single site is no finding.
  `cast_possible_truncation` alone has 939 sites, each a place where an
  integer may be cut, so it is read with S3 of the rubric.

## P. Performance

Source: The Rust Performance Book,
<https://nnethercote.github.io/perf-book/>, all eighteen chapters
loaded (`benchmarking.html`, `build-configuration.html`, `linting.html`,
`profiling.html`, `inlining.html`, `hashing.html`,
`heap-allocations.html`, `type-sizes.html`,
`standard-library-types.html`, `iterators.html`, `bounds-checks.html`,
`io.html`, `logging-and-debugging.html`, `wrapper-types.html`,
`machine-code.html`, `parallelism.html`, `general-tips.html`,
`compile-times.html`).

- **P1. Measure before changing** (`benchmarking.html`,
  `profiling.html`, `general-tips.html`). A performance finding states
  a number from an instrument: a journey's instruction count or cache
  misses (the ratchet, callgrind and cachegrind), an allocation count
  (DHAT), a counter of the target sets, pinned CPU time
  (`bench/pinned-time.py`), or the baseline's times. Instruction counts
  stand in for time where wall time is noisy; the book names both.
  Reading alone yields a candidate to measure, not a finding. Optimise
  only what is hot; the largest gains are algorithms and data
  structures.
- **P2. Allocation rate** (`heap-allocations.html`). On a hot path: no
  allocation per step that a reused collection (`clear`, a workhorse
  buffer passed as `&mut`), `with_capacity`, `clone_from`, a `SmallVec`
  for short lists or a `Cow` would avoid; no `format!` or `to_string`
  where a `write!` into a kept buffer does; no `collect` whose result
  is only iterated. DHAT finds the sites; the book's rule of thumb is
  that ten allocations fewer per million instructions is about one per
  cent.
- **P3. Clones on hot paths** (`heap-allocations.html`, "clone"). A
  clone of heap data on a journey is justified or removed;
  `redundant_clone` has 39 sites (E7).
- **P4. Dispatch chosen by measurement** (`inlining.html`,
  `compile-times.html`). Dynamic dispatch where an enum or a generic
  would do costs on a hot loop; a generic body instantiated per caller
  costs code size and compile time (`cargo llvm-lines`; the fix is an
  inner non-generic function). The crate's rule that a long body takes
  its stop as `&mut dyn FnMut` took 129 KB off the web module
  (`core.md`), so a finding in either direction carries its count:
  instructions for the hot side, bytes for the cold.
- **P5. Data layout** (`type-sizes.html`). Often-instantiated types are
  small: indices as `u32`, a boxed slice where a `Vec` never grows, the
  large variant of an enum boxed, nothing over 128 bytes copied in a
  loop. `-Zprint-type-sizes` shows the layout; a type whose size
  matters has a static assertion (the crate asserts `Error` at 64
  bytes).
- **P6. Hashing** (`hashing.html`, `linting.html`). One fast hasher on
  hot tables (the crate's `hash` module, foldhash) and no stray
  `std::collections::HashMap` with SipHash beside it; the book's means
  is Clippy's `disallowed_types` in `clippy.toml`.
- **P7. Inlining** (`inlining.html`). `#[inline(always)]` (26 sites)
  and `#[cold]` rest on a measurement; a large function with one hot
  call site is split into a hot inlined part and a cold outlined one.
- **P8. Iterators, bounds checks, standard types** (`iterators.html`,
  `bounds-checks.html`, `standard-library-types.html`): `extend` over
  collect-and-append, `swap_remove` and `retain` where order allows,
  lazy `_else` forms where the argument is costly, a slice taken before
  an indexed loop. Only on a path a profile shows.
- **P9. Input and output** (`io.html`). The command locks standard
  output once and buffers it for many small writes, and flushes
  explicitly; readers are buffered.
- **P10. Build configuration** (`build-configuration.html`). The
  release profile has fat LTO and one codegen unit already. Open, each
  to be measured before it is proposed: `panic = "abort"` for the
  command, another allocator for the command, `opt-level` for the web
  module's size, a custom profile for tests.
- **P11. No work for what is off** (`logging-and-debugging.html`):
  nothing is collected for a diagnostic that is not shown; a hot
  assertion that soundness does not need is a `debug_assert!`. A
  checker's assertion stays.
- **P12. Compile time** (`compile-times.html`): `cargo build --timings`
  for the crate graph, `cargo llvm-lines` for generic bloat. Build cost
  is a baseline (B7).
- **P13. An architecture states its cost.** A finding that proposes a
  boundary, a trait or a generic over a hot path names the journeys and
  target rows that cover it and what it expects of them. A change that
  moves a counter of the target sets is a search change, not a
  refactor, and says so.

## X. Extensibility

Source: the author's request (`plan/86-audit-refactor.md`): "a
researcher implementing a new engine reuses how sequents and formulas
are handled", and a downstream crate builds on "the sequents, formulas,
the forest, the checkers and the exports without forking"; with
C-SEALED, C-INTERMEDIATE, C-GENERIC, C-VALIDATE and C-STRUCT-PRIVATE
above. The evidence is the consumer test (`consumer-test.md`).

- **X1. Formulas and sequents are built, read and walked through the
  public API alone**, in every suite: constructed without the text
  parser, each node inspected, subformulas followed, two formulas
  compared, a formula printed.
- **X2. The shared machinery is reachable**: the occurrence forest, the
  limits and the stop, the error family, so that an outside engine
  bounds itself and refuses as a built-in one does.
- **X3. An outside engine's proof is checked.** A proof term or a
  derivation is built through a public constructor that validates its
  shape, and the crate's checker decides it. No public path makes a
  value that reads as checked without the check: the checkers stay the
  trust boundary.
- **X4. An outside engine can answer in the crate's terms**: it can
  return the verdict and evidence types the front ends print, or the
  docs say plainly what it returns instead.
- **X5. What comes after a proof works for any proof**: the derivation
  view, the exports, cut elimination and the JSON forms take a checked
  proof whoever made it.
- **X6. A trait is sealed only where soundness needs it**, and its docs
  say that it is sealed and why (C-SEALED). An enum is closed only
  where a missed case must not compile (`core.md`, Decisions).
- **X7. A part can be taken alone.** With `default-features = false` a
  user gets the data model and the checkers with few dependencies; each
  feature cuts along a line a user needs (B4).
- **X8. The way is written down.** A manual page "Write your own
  engine" builds a toy engine on the public API and has its proofs
  checked, and the readme test runs it (B5).
- **X9. Nothing is public by accident.** Every public item is meant for
  a user, documented with an example where one helps, or becomes
  `pub(crate)` (B4). The surface today is 5 128 items
  (`baselines.md`).

## L. The command line

Sources: the Command Line Interface Guidelines, <https://clig.dev/>
(one page; its second half read again by offset and compared with the
page's source, so every section rests on the page's text; "Subcommands"
has no anchor of its own); and Command Line Applications in Rust,
<https://rust-cli.github.io/book/> (all thirteen chapters loaded; the
"Configuration environments" part of `in-depth/config-files.html` is an
unwritten placeholder). They apply as far as they fit `deduxit-cli`:
it reads no secret, talks to no server and changes no state but the
files a user names, so the guidelines on secrets, analytics, network
proxies and destructive confirmations do not apply.

- **L1. The basics** (<https://clig.dev/#the-basics>;
  `in-depth/exit-code.html`). Data goes to standard output, messages to
  standard error; the exit statuses are a documented contract (D3).
- **L2. Help** (<https://clig.dev/#help>). `-h` and `--help` on the
  command and every subcommand, `help SUBCOMMAND` too; help leads with
  examples and the most common flags and commands, and links the web
  manual; a mistyped flag or subcommand gets a suggestion and is never
  run corrected; with nothing to read on an interactive terminal the
  command says what it waits for, or prints help, and does not hang
  silently.
- **L3. Documentation in the terminal and on the web**
  (<https://clig.dev/#documentation>; `in-depth/docs.html`): a man page
  rendered from the clap definition (`clap_mangen`) and shell
  completions (`clap_complete`, named in `tutorial/cli-args.html` and
  `tutorial/packaging.html`), shipped in the release archive, if they
  serve users.
- **L4. Output** (<https://clig.dev/#output>;
  `tutorial/output.html`, `in-depth/machine-communication.html`).
  Human-readable first; a machine-readable form that is an interface
  (clig names the flag `--json`; deduxit has `--format json`, which the
  audit weighs against the standard name); a stream of results is one
  JSON document per line; a terminal is detected per stream
  (`std::io::IsTerminal`); colour is off when the stream is no
  terminal, `NO_COLOR` is set and not empty, `TERM=dumb`, or
  `--no-color` is passed; success prints briefly, `-q` prints less;
  nothing only a developer understands by default.
- **L5. Errors** (<https://clig.dev/#errors>;
  `tutorial/errors.html`, `in-depth/human-communication.html`). An
  expected error is rewritten for a person and says what to do; the
  most important line comes last; an unexpected one gives a way to
  report it without a wall of text (the book names `human-panic` for
  panics); errors keep their cause (`anyhow` context).
- **L6. Arguments and flags**
  (<https://clig.dev/#arguments-and-flags>). Flags over positional
  arguments, except the one primary argument; a long form for every
  flag, one letter only for the common ones; the standard names where
  one exists (`-h/--help`, `--version`, `-o/--output`, `-q/--quiet`,
  `-f/--force`, `-n/--dry-run`, `--json`, `-d/--debug`, `--no-input`;
  `-v` avoided or kept for one meaning); `-` for standard input and
  output where a file is read or written; the default is what most
  users want, except where D1 asks for a named logic; flags work before
  and after a subcommand where possible.
- **L7. Interactivity** (<https://clig.dev/#interactivity>). A prompt
  only when standard input is a terminal, and never required: the
  session of `interact` can be driven from a file or a pipe, and says
  how to leave.
- **L8. Subcommands** (clig.dev, "Subcommands"). The same flag means
  the same thing under every subcommand, with the same output shapes;
  names follow one pattern (noun then verb is the common one), none
  near another in meaning; no catch-all subcommand and no arbitrary
  abbreviation (<https://clig.dev/#future-proofing>).
- **L9. Robustness** (<https://clig.dev/#robustness-guidelines>).
  Input is validated before work starts; something is printed within
  about a tenth of a second or the wait is explained; a long run shows
  progress on a terminal; the time limit is configurable with a
  sensible default; nothing needs cleaning up after an interruption.
- **L10. Future-proofing** (<https://clig.dev/#future-proofing>).
  Subcommands, flags, environment variables and the JSON forms are
  commitments; a change is additive where it can be, and a break (D2)
  is written into the changelog with the replacement.
- **L11. Signals** (<https://clig.dev/#signals>;
  `in-depth/signals.html`). The first Ctrl-C ends the search and says
  what a second will do; the second quits at once (130).
- **L12. Configuration and environment**
  (<https://clig.dev/#configuration>,
  <https://clig.dev/#environment-variables>). Flags win over
  environment variables, those over a project's file, that over a
  user's; a user's file follows the XDG directories; variable names are
  upper case with underscores; `NO_COLOR`, `TERM`, `PAGER`, `COLUMNS`
  are honoured where they apply.
- **L13. Testing and distribution** (`tutorial/testing.html`,
  `tutorial/packaging.html`; <https://clig.dev/#distribution>). The
  binary is tested as a binary (`assert_cmd`, `predicates`,
  `assert_fs`), on observable behaviour and not on the layout of
  generated help; it installs by `cargo install` and as one binary with
  its man page and completions.

## T. Text

Source: `.claude/rules/writing.md`, the author's standing instruction of
2026-11-09, for rustdoc, comments, help, messages and errors, the
manual, the guide for models, the changelog and the Rocq library's
README and docs.

- **T1. Concise, plain language**: short sentences in the active voice,
  common words, a logic term explained or linked at first use, what a
  thing does before how.
- **T2. Self-contained**: no narration of how the project was made; a
  comment says why, or what the code cannot.
- **T3. Written for someone using it**: starts from what the reader
  wants to do, with an example that runs; tutorial, how-to, reference
  and explanation kept apart; no suite given a place of honour (D4).
- **T4. Legible**: "is" and "has", no inflated words, no negative
  parallelism, no reflexive triads, no trailing "-ing" commentary, no
  filler addressed to the reader; sentence-case headings, bold only for
  what must be found.

A text finding quotes the sentence and gives the rewrite. The
`conventions` check settles mentions of the plan; `typos` spelling.

## Q. The Rocq library

- **Q1. Packaging for the Rocq opam archive.** Source:
  <https://rocq-prover.org/docs/opam-packaging>, loaded in full. The
  package is named `rocq-…`; the opam file has `synopsis`,
  `description`, `maintainer`, `authors`, `license`, `homepage`,
  `bug-reports`, `dev-repo`, `doc`, `depends` with a lower and an upper
  bound on `rocq-core` and `rocq-stdlib` so that every installable
  version compiles, `build: [make "-j%{jobs}%"]`, `install: [make
  "install"]`, and `tags` (`logpath:`, `category:`, `keyword:`,
  `date:`); the release is an uploaded archive with a SHA-512, since
  GitHub's generated tarballs keep no stable checksum; `opam lint
  --check-upstream` is clean and the package installs from a local
  repository; no `Admitted`, every axiom documented, a changelog, and a
  list of renamings with a new version. The 9.0 changes add: depend on
  `rocq-core` and `rocq-stdlib`, never on `coq` or the `rocq-prover`
  metapackage
  (<https://rocq-prover.org/doc/V9.0.0/refman/changes.html>).
- **Q2. The rocq-community manifesto**, as far as it fits a project
  outside the organisation. Source:
  <https://github.com/rocq-community/manifesto> (README, in full) and
  <https://github.com/rocq-community/templates> (`ref.yml`, the opam,
  CI and README templates). Named maintainers; an OSI-approved licence;
  continuous integration over the supported Rocq versions, the latest
  stable among them at all times; compatibility considered in every
  change to what has users. The template's README has a description,
  the meta data (authors, licence, compatible versions, dependencies,
  namespace) and build and installation instructions, and its CI is a
  matrix of images, one per tested version.
- **Q3. Naming and proof style.** No current official page fixes them:
  the standard library's contribution guide leaves its style section
  open (<https://github.com/rocq-prover/stdlib/blob/master/CONTRIBUTING.md>),
  and the rule this step's prompt recalled, capitalised predicates and
  modules with lower-case functions, is on no page read; the only
  written proposal, for Coq 8.4, says inductive predicates are lower
  case and that a library with capitalised ones keeps its own style. So
  the criterion is consistency, from two wiki pages:
  - <https://github.com/rocq-prover/rocq/wiki/CoqStyle>: `Proof.` on a
    line of its own; every hypothesis named (`intros` names, `as`
    clauses); one naming scheme for inductive types, constructors and
    lemmas, the standard library's patterns where in doubt (its lemma
    suffixes such as `_comm`, `_assoc`, `_spec`, `_iff`, `_inj`,
    `_refl`, `_trans`, `_dec`); structure in coqdoc comments.
  - <https://github.com/rocq-prover/rocq/wiki/Anti-patterns>: no hint
    in the `core` database, a database of the library's own; notations
    in a scope; no script that depends on the order of goals (bullets)
    or on generated names; bullets not nested deeply; no automation
    that breaks when it gets stronger (`auto with …` or `intuition`
    followed by a tactic that expects a certain goal); files split by
    topic.
- **Q4. Rocq 9's names and deprecations**, on every version the opam
  file admits (9.0 to 9.3). Sources: the changes pages of the
  reference manual for 9.0 to 9.3
  (`https://rocq-prover.org/doc/V9.N.0/refman/changes.html`, each read
  to offset 200 000 at most) and of the standard library for 9.0 to 9.2
  (`…/refman-stdlib/changes.html`). `From Stdlib Require`, never `From
  Coq` or a bare `Require` (warnings `deprecated-from-Coq`,
  `deprecated-dirpath-Coq`, `deprecated-missing-stdlib`); the `rocq`
  commands. No deprecation warning on any admitted version: explicit
  `Require` of what 9.1 no longer brings in transitively; no implicit
  creation of a hint database and no `Notation` for an abbreviation
  (deprecated in 9.2); no reliance on `intuition` running `auto with *`
  (removed in 9.2); every interactive proof opens with `Proof` (9.3
  warns), no non-boolean `if` (deprecated in 9.3).
- **Q5. Robust proofs, by the manual's own means.** Source:
  <https://rocq-prover.org/doc/V9.1.0/refman/proofs/writing-proofs/proof-mode.html>
  and `proof-engine/vernacular-commands.html`,
  `practical-tools/coq-commands.html`, `practical-tools/utilities.html`,
  `proofs/automatic-tactics/auto.html` (the last two read to about
  100 000 characters). Strict bullets and braces; `Default Goal
  Selector` set to `!` as the manual recommends, so that each tactic
  names its goal; the library compiles with `-mangle-names`, the
  manual's linter for generated names; warnings turned into errors by
  `+name` in the `Warnings` option or `-w`, passed through `-arg` in
  the project file; `-Q`, not `-R`; hints with a stated locality and a
  database of the library's own.
- **Q6. Closed theorems.** Every theorem a certificate rests on prints
  "Closed under the global context" under `Print Assumptions`
  (`vernacular-commands.html#coq:cmd.Print-Assumptions`); no
  `Admitted`, `Axiom` or `Parameter` (the library's check greps for
  them). The audit asks whether the list of checked theorems is every
  one a certificate or a user relies on.
- **Q7. The versions claimed are the versions tested**: the opam bounds
  and the CI's matrix agree, and the installation from the opam file is
  tested per version (Q1, Q2).
- **Q8. The certificates' interface is a contract.** The modules,
  constructors and theorem names that deduxit's exporter writes into a
  certificate, and that the bridges import, are the library's
  interface; each is listed, and a change to one moves the exporter in
  the same pair of changes (the brief; `rocq-deduxit`'s CLAUDE.md).
  What is not interface can be internal.
- **Q9. No needless duplication between the calculi**, and a shared
  lemma moves to `Deduxit.Base` once two calculi use it (the library's
  stated shape). Propositional and first-order developments that repeat
  each other are the place to look.
- **Q10. Checking time.** The time Rocq takes to compile the library
  and to check each certificate is measured; a checker that computes in
  the kernel is written so that it does (the Rust side's P1, applied to
  `vm_compute` and `Qed`).
- **Q11. Text**: the README, the doc comments and the opam description
  follow T1 to T4.

## B. What the request implies besides

From `plan/86-audit-refactor.md`, "What the request implies besides".

- **B1. A safety net before any change.** A refactor keeps: the
  behaviour lock and the examples of README, the manual and the guide;
  the counters of the focused and net target sets against their
  oracles; the ratchet's ceilings; the mutation baseline of the
  checker, the readers and the front door, and the fuzz targets; Kani's
  harnesses; the Rocq certificates' checks and the library's own.
- **B2. API changes are one decision.** The audit proposes the 0.9.0
  surface once, crate by crate, each change with its reason and the
  practice it follows; the author signs it off before any step
  implements it. D1 is decided already.
- **B3. Each break is written down**: `breaks 0.8.0` after each
  refactor step, the changelog's "Changed" and "Removed" kept true, a
  migration note for library users.
- **B4. The public surface is small on purpose** (X9), and the nine
  default features and three optional ones cut along lines users need
  (X7, C-FEATURE).
- **B5. A researcher's how-to** is the API's acceptance test (X8).
- **B6. The bindings and the client follow.** `web/` is a consumer of
  the API and its calls are a contract with the client: an API change
  is carried through `web/`, the client's calls and the module's size
  budget (2 885 650 bytes at gzip against a ceiling of 2 900 000).
- **B7. Build cost does not grow without a reason**: compile time, test
  time, the wasm module's bytes and the lock's packages (175 against a
  ceiling of 180), before and after.
- **B8. Dependencies earn their place** (the rubric's S8): `cargo deny`
  and `cargo shear` pass; the audit looks for a well-made crate that
  would replace ad-hoc code and for a dependency that no longer pays.
- **B9. The Rocq library's surface is stated** (Q8).
