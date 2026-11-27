# Recorded decisions that stand against a proposal of the audit

Kept beside the findings so that the API proposal answers each one: it
either stays within the decision, or asks the author to change it and
says why. A decision is listed with where it is recorded, what it says,
which findings meet it, and what has changed since it was made. The
architecture reader's verifier found the first ones
(`units/arch/verdicts.md`, "What stands against the proposals"); later
stages add theirs.

## The crate's shape

- **One crate with a module per suite, and no feature per suite.** The
  author, 2026-11-09 (`plan/README.md:5570-5581`); restated in
  `plan/notes/kernel-boundary.md:308-311` and `plan/later.md:2699`,
  `:2723`. A facade over a kernel crate and crates per suite "comes
  only when one of these happens: a second consumer needs part of the
  library; an independent suite exists (modal, step 65); a heavy
  dependency arrives; a suite needs its own release pace; the page
  needs a WebAssembly module per suite."
  - Meets: F-arch-17 (a feature `search`, and a feature per suite as
    the alternative to a split), F-arch-18 (the web module's size),
    F-6a-04 (no feature gives the data model and the checkers alone).
  - Since then: modal logic exists as an independent suite, and step 67
    weighed the split and ended at its gate, "not yet a crate"
    (`plan/later.md:2711-2723`). The web module is 2 987 284 bytes at
    gzip. The engine interface (X4, X7) is a new reason: an outside
    engine wants the data model and the checkers without the engines.
  - The proposal must either keep to it (the layering inside the crate,
    which needs no feature) or put the question to the author with the
    measured cost of the engines in a build.
- **The same decision made linear logic the command's default "through
  0.x"** (`plan/README.md:5566-5567`). D1 (the author, 2026-11-27)
  replaces that part: in 0.9.0 no logic is the default.
- **The kernel crate ended at its gate** (step 67, 2026-11-16;
  `plan/notes/kernel-boundary.md`, `plan/later.md:2711`): `Error`,
  `Settings`, the wire level, `Drawable` and the protocol's registry are
  sums over the suites and stay above them. What reopens it: a decision
  to put a suite in a crate of its own, or a suite from another
  repository that must not link the linear suite.
  - Meets: every finding that moves a shared notion out of `linear`
    (F-arch-01, F-arch-06, F-arch-07, F-1a-02, F-1a-03, F-1b-01). They
    agree with it: `plan/later.md:2725-2733` lists the crate-private
    moves as due, "no public path changed". D2 and D4 are later and
    allow the public paths to change as well.

## The public API

- **An item without a caller waits for one** (`core.md:495-497`: "the
  checked builder, `Limits::BROWSER`, `Settings::browser()`: it is
  additive later, and the first release promises no unmeasured
  numbers").
  - Meets: F-arch-03, F-arch-04, F-arch-05, F-1a-01, F-6a-03 (the
    constructors and the engine interface). The caller exists now: the
    manual page "Write your own engine" (X8), which the readme test
    runs, and the consumer test's two toy engines.
- **The checked builder's design is recorded** (R59,
  `plan/notes/api.md:354-359`): `Sequent::builder() -> Builder` with
  `atom(name)`, `term(Term)` (indices checked, hash-consed as the parser
  does, so that a built sequent equals the parsed one), `dual(TermId)`,
  `left(TermId)`, `right(TermId)`, `finish(&Limits)`; a later step adds
  `predicate`, `function`, `bind`.
  - Meets: F-1a-01 and F-arch-05, which propose `Sequent::atom`, `push`
    and `push_root` on the sequent itself. The proposal follows R59
    unless it shows why the methods on `Sequent` serve better.
- **A front end never builds an error of the library** (`core.md`, "One
  error family").
  - Meets: F-arch-03 and F-6a-03 (public constructors of `Refusal`).
    They are for engines; the rule stays for front ends, and the docs
    of the constructors say so.
- **Every long call takes `&Limits` and `stop: impl FnMut(Progress) ->
  bool`, with no `Stop` trait** (`core.md:482-489`).
  - Meets: F-arch-03's `Meter` and F-arch-04's `Decide` (which takes
    `&mut dyn FnMut(Progress) -> bool`): both keep the closure.
    `Progress` is constructible today (`Progress::default()` and public
    fields, `limits.rs:405-423`), so only the refusal is missing.
- **The linear outcome's fields are public** (`linear/search/mod.rs`,
  `Outcome`), so `checked` and `verdict` are writable.
  - Meets: F-arch-04. `prove_with` alone does not keep the checker as
    the trust boundary until an outcome cannot be written to; and an
    outside engine must be refused as a forced engine (`--engine`, the
    JSON names, the dispatch test).
- **The `Suite` trait is not sealed, on purpose**
  (`plan/notes/63g-design.md:814-816`: "the wire forms stay sealed, and
  a suite from another crate is one of the cases step 63's prompt
  names").
  - Meets: F-arch-10, which offers sealing as one of two mends. The
    record chooses the other: the registry becomes a value that can
    hold a suite from outside.

## The features

- **`serialize` gates `rules`** because a calculus stores its serde
  form (`core-rules.md:33-37`), which holds for the built-in rule files
  read from JSON.
  - Meets: F-arch-16, F-6a-04, F-6a-17.
- **A feature `search`** (F-arch-17) would have to be implied by
  `interactive` (`close`, `hint`), by the `rules` session's search and
  by `protocol`, which all call the searches.
- Nothing recorded stands against one feature `export` for `latex`,
  `typst` and `svg` (F-arch-22); `web-render` asks for `svg` alone in
  its own manifest and gets the others by feature unification.

## The web module

- **A module per suite gains little** (54x6, 2026-11-16;
  `plan/later.md:2684-2709`, measured with twiggy): the linear suite was
  1.40 MB raw of deduxit's 3.13 MB, and every suite's module would
  carry the shared code again. The renderers' module, which that note
  recommended, was made on 2026-11-21.
  - What it left for the author: "a registry the bindings build from
    the page's list", so that the workspace links only the operations
    the page uses: about 1.35 MB raw of protocol dispatch and serde
    code came in with the workspace. It is an API change.
  - Meets: F-arch-18 (its option D without separate modules) and
    F-arch-10 (the registry as a value). The attribution is ten days
    and four steps old; the proposal measures again before it chooses.
- The module loads after first paint, in a worker (`web.md:632-634`).
- **The session over JSON** (F-arch-12): `web.md:87-90` expects the
  library to give the step and candidate forms a wire form; nothing
  recorded stands against it. The command's `interact` works on typed
  sessions, so moving it to the same trait is a larger change than the
  bindings'.

## The Rocq library

- **The first-order files leave the propositional ones untouched**
  (`rocq.md`, "the multiset functions again"): the duplication of
  F-9-05, F-9-06 and F-9-14 follows from it. The verifier found no
  reason recorded beyond leaving the propositional files alone; the
  checkers are outside the trusted statements.
- **`Abbreviation` only from Rocq 9.2** (`rocq.md:34-37`) is why the
  library keeps `Notation` for abbreviations (F-9-04); writing the
  application out works on all four versions.
- **The opam installation is not a flake check because it needs the
  network** (`rocq.md:37-39`); that rules out the flake, not a job of
  the library's CI (F-9-03).

## The command and its texts

- **The batch's record is assembled as text** on purpose (`cli.md`;
  F-7a-18, rejected for it).
- **The message of `GlivenkoFirstOrder` carries its countermodel** on
  purpose (`core-ordinary.md:2219-2226`; F-6a-14, rejected for it).
