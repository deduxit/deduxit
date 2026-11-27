# Step 86: an audit and refactor of the Rust and Rocq libraries, for 0.9.0

You are working in deduxit (`deduxit/deduxit`), its Rocq library
rocq-deduxit (`../rocq-deduxit`) and, where an API change reaches it,
the web client (`deduxit.github.io`). CLAUDE.md, each repository's rules
and `plan/conduct.md` apply throughout. This prompt was written on
2026-11-27 by the planning session at the author's request.

## The author's request (2026-11-27)

After steps 82 to 85, audit and refactor the Rust and the Rocq
libraries, so that:
- the code is idiomatic and follows current best practice;
- the architecture is clean, with the right amount of abstraction and
  no needless duplication;
- performance is as good as is reasonable, and no architectural choice
  costs it needlessly;
- the API is sensible and lets others use its parts on their own (a
  researcher implementing a new engine reuses how sequents and formulas
  are handled);
- the CLI's commands are natural and follow current practice for Rust
  command-line tools, as far as it applies to `deduxit-cli`;
- every human-facing text is concise and plain, with no narration, easy
  to understand and self-contained (`.claude/rules/writing.md`);
- the API may change significantly: the next release is 0.9.0.

The author gave the audit the right to run a reasonably sized workflow
of agents on several models.

## Quality criteria

The audit judges against written criteria. Phase A writes them out as
`plan/notes/audit/criteria.md`, each criterion citing its source, read
this session from the source itself, not from memory. The sources:

- **Rust API.** The Rust API Guidelines' checklist
  (rust-lang.github.io/api-guidelines/checklist.html): naming,
  interoperability (the common traits, conversions, `Error` types),
  predictability, flexibility, type safety, dependability, debuggability,
  future-proofing (`#[non_exhaustive]`, sealed traits, private fields)
  and documentation (crate docs, examples that run, errors and panics
  documented).
- **Rust idiom.** The Rust 2024 edition, Clippy's groups as the
  workspace configures them, the Rust reference's and std's own
  conventions.
- **Performance.** The Rust Performance Book (nnethercote's): measure
  before changing; allocation rate (DHAT), clones on hot paths, the cost
  of dynamic dispatch where a generic or an enum would do, data layout.
  It also states what an architecture costs on the repository's own
  instruments (below).
- **Extensibility.** A downstream crate can build a new engine on the
  sequents, formulas, the forest, the checkers and the exports without
  forking. Traits are sealed only where soundness needs it; checkers
  stay the trust boundary, so a third-party engine's proof is checked.
- **CLI.** The Command Line Interface Guidelines (clig.dev) and the Rust
  CLI book (rust-cli.github.io/book): subcommand and flag naming, help
  and errors, stdout for data and stderr for diagnostics, exit statuses
  as a documented contract (deduxit's 0/1/2/3/130 stay), `--format
  json` as an interface, `NO_COLOR` and terminals, shell completions and
  a man page (`clap_complete`, `clap_mangen`) if they serve users.
- **Text.** `.claude/rules/writing.md`, for rustdoc, comments, help,
  messages, the manual, the guide and the Rocq library's docs.
- **Rocq.**
  - The Rocq reference manual of the pinned version.
  - The opam packaging page (rocq-prover.org/docs/opam-packaging).
  - The rocq-community manifesto and its templates.
  - The standard library's contribution guide for naming: capitalised
    predicates and modules, lowercase functions.
  - Rocq 9's `From Stdlib` namespace and its deprecations.
  - Proof robustness: no reliance on generated names, bullets and
    focusing, the warnings the library turns into errors.
  - Every theorem's assumptions closed (`Print Assumptions`).
  - A CI matrix over the Rocq versions the opam file claims.
  - The certificates' interface with deduxit's exporter as a stable
    contract.

Phase A also adds the points below, which the author did not name but
which a refactor of this size needs.

## What the request implies besides

- **A safety net before any change.** Every refactor is shown to keep
  behaviour, with the repository's own instruments:
  - the behaviour lock and README's, the manual's and the guide's
    examples;
  - the focused engine's oracle target set and the net engine's, whose
    counters must stay equal on a refactor (CLAUDE.md's table);
  - the ratchet's instruction counts and their ceilings;
  - mutation testing of the checker, the readers and the front door, and
    the fuzz targets;
  - Kani's harnesses of the checker's integers;
  - the Rocq certificates' checks and `rocq-deduxit-library`.

  A change that moves a counter is a search change, not a refactor, and
  says so.
- **API changes as one decision.** `writing.md` sends API changes to the
  author. Phase B ends with one API proposal: the 0.9.0 surface, crate by
  crate, each change with its reason and the most idiomatic current
  practice it follows. The author signs it off once, and the refactor
  steps implement it.
- **Each release's break written down.** `breaks 0.8.0`
  (cargo-semver-checks, then cargo-public-api's diff) after each
  refactor step, the changelog's "Changed" and "Removed" kept true, and
  a migration note for library users in the changelog.
- **The public surface kept small on purpose.** What is public is a
  promise: everything public is either meant for users (documented,
  with an example) or becomes `pub(crate)`. Features: whether the nine
  default features and three optional ones still cut along lines users
  need.
- **A researcher's how-to.** The extensibility goal gets a manual page,
  "Write your own engine": a worked example that builds a toy engine on
  the public API and has its proofs checked. It is the acceptance test of
  the API, run by the readme test.
- **The bindings and the client.** The web bindings (`web/`) are a
  consumer of the API, and their calls a contract with the client. An
  API change carries through `web/`, the client's calls and the
  module's size budget.
- **Build cost.** Compile time, test time, the wasm module's bytes and
  `Cargo.lock`'s packages are measured before and after, and none grows
  without a reason.
- **Dependencies.** Each earns its place (CLAUDE.md), with `cargo deny`
  and `cargo shear`. Look for a well-made crate that would replace
  ad-hoc code, and for a dependency that no longer pays for itself.
- **The Rocq library's surface.** Which modules are its interface (what
  the exporter's certificates and NanoYalla's bridges import) and which
  are internal; a refactor keeps the interface or moves deduxit's
  exporter with it in the same pair of changes.

## Decided by the author already

- **No default logic** (2026-11-27, on fixup-30's question). Today
  `prove` without `--logic` or `--system` reads linear logic. In 0.9.0
  every command that reads a sequent (`prove` and its batch, `check`,
  `interact`, `seq …`, and any other) requires the logic to be named, so
  that no suite is the default.
  - The audit's API proposal designs the flags in the most idiomatic
    current form: one selector for the suite, or a required group of the
    present flags, with clap's error naming the choices.
  - The refactor's CLI session implements it, with every console block
    of README, the manual and the guide, the behaviour lock and the
    batch's format.
  - The web bindings are untouched: each call already names its suite.

## How the step runs

The session that leads the audit runs on Fable 5.2 at `high`; the
author started it ("Rust and Rocq library audit"). The work has four
phases, and the leader checks with the planning session between them.

### Phase A: criteria and baselines (now, beside steps 82 to 85)

Phase A changes no code, so it can run while steps 82 to 85 change it.
- **Criteria**: `plan/notes/audit/criteria.md`, as above.
- **The audit's method**, `plan/notes/audit/method.md`, fixed before any
  agent runs: the areas, the dimensions, the agents and their models and
  efforts, the verification, the budget.
- **Baselines** on today's `main` (one run each, under the machine lock,
  coordinated with the planning session because the step sessions use
  the machine): the ratchet's counts, `breaks 0.8.0`'s surface listing
  (cargo-public-api), compile and test times, the wasm module's bytes,
  the focused and net target sets if the planning session gives the
  slot, line counts per module, Clippy with the pedantic group's
  allowed lints counted.
- **An API consumer test**: a scratch crate outside the workspace that
  tries to build a toy engine (say, a naive backward search for
  multiplicative linear logic, or a truth-table method for ordinary
  logic) on the public API alone, recording each place the API gets in
  the way. That record is evidence for phase B.

### Phase B: the audit (after step 85 lands; read-only)

A workflow of agents over a fixed grid, with every finding verified
before it is reported. The size follows the author's "reasonably sized"
and the session's medium guideline. About ten agents per stage is the
upper bound, more stages rather than wider ones.
- **The areas** (about 210 000 lines of Rust in `core/`, 21 000 in
  `cli/`, 11 000 of Rocq):
  1. sequents, the forest, proofs and the checker;
  2. the linear search engines (front door, focused, Horn, inverse,
     parallel);
  3. nets and their engine;
  4. ordinary logic, propositional and first-order, with its
     interactive state;
  5. modal logic, lambda-terms and calculi given as data;
  6. the shared layers: errors, limits, settings, `wire`, `serialize`,
     `protocol`, `workspace`, the exports;
  7. the command (`cli/`);
  8. the bindings (`web/`, `web-render/`, `web-tab/`) as an API consumer;
  9. rocq-deduxit.
- **The dimensions** each area is read for: idiom, architecture and
  duplication, performance, the public API and its extensibility, and
  text.
- **Finders**: one agent per area, Opus 5.5 at `high`, read-only, each
  briefed with the criteria, its area's rules file and the consumer
  test's record. Each finding has a file and a line, the criterion it
  breaks, and a witness: a snippet, a measurement, a failing use, or the
  duplicate it repeats.
- **One architecture reader**: a single agent across the whole crate,
  Fable 5.2 at `high`. Architecture needs one continuous judgement, so
  it is not split by area. It covers module boundaries, the layering of
  the suites over the shared parts, where a trait or a generic would
  remove duplication, and where an abstraction costs more than it
  saves.
- **Verifiers**: each finding goes to a verifier on another model than
  its finder's (Fable 5.2 at `medium` for Opus finders; Opus 5.5 at
  `high` for the Fable reader), told to refute it. It survives only with
  its witness checked: a duplicate shown, a measurement rerun, a
  criterion quoted. Findings are deduplicated across areas, and a list
  of rejected findings is kept so that none is raised twice.
- **Inventories**, where a tool does better than a model: Sonnet 5.5 at
  `medium`, or plain tools, for the public items' listing, dead and
  near-dead public items, doc comments that narrate, duplicate code
  (a clone detector if one is in nixpkgs), and long functions.
- **Synthesis** by the leader: `plan/notes/audit/findings.md`, ranked by
  value against cost and risk, and `plan/notes/audit/api-proposal.md`,
  the 0.9.0 surface for the author's sign-off.
- **Cost**: the workflow's agents count against the account's weekly
  limit, with Fable at most half of it (`plan/README.md`'s role table).
  The leader estimates the run before starting it and tells the planning
  session.

### Phase C: the refactor plan

From the findings and the signed-off proposal, the leader writes the
refactor as steps 87a, 87b and so on in `plan/87-refactor.md`. Each step
is one area or one cross-cutting change, small enough for one session,
and behaviour-preserving first: a step that changes behaviour (an API
change or a search change) says so in its first line. The steps are
ordered so that the safety net runs green after each. They name which
can run side by side (no two on the same files), and the planning
session runs them like any other step.

### Phase D: the refactor steps

Each runs as a step session on Opus 5.5 at `high`, with `fresh-reviewer`
on Fable 5.2 at `medium`. The panel runs on any change to a checker, a
search or a certificate. Each session verifies with:
- CLAUDE.md's table;
- the oracle target sets with equal counters;
- the ratchet;
- `breaks 0.8.0`;
- and, for the Rocq library, its own `nix flake check` and the
  certificates against it.

After the last step, the release of 0.9.0, prepared as 80 prepared
0.8.0.

## Rules for the leader

- **Workspace.** The session started in the main checkout
  (`/home/tux/Projects/own/deduxit`), which the planning session uses.
  Before writing anything, make a jj workspace of your own: `cd
  /home/tux/Projects/own/deduxit && jj workspace add --name audit
  ../deduxit-wt-audit && cd ../deduxit-wt-audit && jj new main@origin`.
  Work there through absolute paths. In rocq-deduxit, read only until
  phase D.
- jj only, never git, not even to read; commit the notes as each is
  done (`jj commit -m`), push nothing. The planning session pushes.
- **Cores**: 13 to 15 for your agents' runs and builds, each capped
  (`systemd-run --user --scope -p MemoryMax=8G -p MemorySwapMax=0
  taskset -c 13-15 …`). Every `nix build` and `nix flake check` runs
  under `flock -o /tmp/linlog-machine.lock`. Cores 0 and 1 are the
  desktop's, 2 and 3 the planning session's, 4 to 12 the step sessions'.
  A timed run is asked of the planning session first.
- Read files with the Read tool or `grep`, `head`, `tail`, `cut`, `wc`;
  never `sed` or `awk`, no `VAR=…` prefix, no `$` in a pattern; no `gh`.
- Every agent's brief carries these rules, its model and its effort.
  Never `max`, `ultracode`, fast mode or the advisor.
- **Workflows**: load the `workflow-authoring` skill before writing one.
  The author allowed a reasonably sized one. Run the phase B audit as
  one workflow, or as a few, one per stage.
- Report to deduxit-orchestrator with `SendMessage` at the end of each
  phase, in one line ("audit: phase A done, …"), and wait for its "go"
  before phase B and before phase D.

## Deliverables

- `plan/notes/audit/criteria.md` and `method.md`;
- the baselines, `plan/notes/audit/baselines.md`;
- the consumer test's record;
- `findings.md` and `api-proposal.md`;
- `plan/87-refactor.md`;
- a report, `plan/reports/86-audit.md`: what was done, the workflow's
  shape and cost, what the verifiers rejected and why, and what is left
  open.
