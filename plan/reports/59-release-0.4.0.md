# Step 59: the release of 0.4.0, prepared

Opus 5.5 at `high`, a reused worker started by `deduxit-orchestrator`
from `/tmp/linlog-handoff/start-59.md` on 2026-11-15, cores 10 to 12, in
the jj workspaces `../deduxit-wt-step-58b-8c0bbc` (deduxit, on
`main@origin` `zpuvxkpt`) and `../rocq-deduxit-59` (the Rocq library,
workspace `step59`).

## Outcome

0.4.0 is ready to tag in both repositories, up to the author's acts:

- **rocq-deduxit** `svrorwzs` (`ef979655f775`), "Date the release of
  0.4.0", pushed by the planning session at this session's request. The
  tag is `v0.4.0`, message "rocq-deduxit 0.4.0", first.
- **deduxit**: the chain below, ending in this report's commit. The tag
  is `v0.4.0`, message "deduxit 0.4.0", once the chain is on `main` and
  56b's page is on the client's `main`.
- The release notes, what a user can do now, the checks before and after
  the tags and the author's checklist are in
  `plan/notes/release-0.4.0.md`.

## The break: `breaks 0.3.0`, and how each line is covered

`breaks 0.3.0` (cores 10–12, at `main@origin` `zpuvxkpt`):

- cargo-semver-checks: 229 checks, 224 pass, 4 fail, 1 warning.
- cargo-public-api: 2 removed, 3 changed (and one auto-trait line),
  the rest added.

| line | covered by |
|---|---|
| discriminant changed: `rocq::Unsupported::Quantifiers` 9→10, `Exhausted` 10→11 | removed: `Unsupported::Basis` (57) was inserted before them; `yqxkskqr` moves it last, as 0.3.0 put `Exhausted` last for the same reason (`core.md`: a downstream `as` cast reads the number even on a non-exhaustive enum). The changelog keeps `Basis` under Added. |
| enum variant added on an exhaustive enum: `ordinary::Rule::Cut` | a new bullet of "What breaks" (`zwtoktqq`): LJ's cut, which LJT's derivations hold; a `match` adds its arm. It is a closed enum by decision (`core.md`), so the variant is a planned 0.y break. |
| discriminant changed and `PartialOrd` order: `ordinary::Rule::Open` 25→26 | the same bullet: `Cut` comes before `Open`, which moves `Open`'s number and place in the derived order. |
| `ordinary::decide` takes 2 parameters, not 5 | listed (55a): `decide(&sequent, &options)`, `decide_within` for the long form. |
| `ordinary::Outcome`'s fields `translation` and `linear` removed | listed (54s/55a): `Outcome::search`. |
| `ordinary::Verdict::Valid(Box<Derivation>)` → `Valid(Evidence)` | listed (54s): `Evidence::Derivation`, `evidence.derivation()`. |
| `impl Freeze` → `impl !Freeze` for `ordinary::Derivation` | none: `Freeze` is a compiler-internal auto trait that stable code cannot name or bound on; the derivation gained interior mutability, an `AtomicBool` that `Derivation::check` sets through a shared reference once the derivation passes. No caller outside the compiler sees it. |

The two lists of types (`core.md`, "Decisions"), checked against the
head's rustdoc JSON with every feature by a script: the closed enums
are exactly the seventeen listed, and 146 public enums and structs with
a public field are `#[non_exhaustive]`. Five of those were missing from
`core.md`'s list (`ordinary::Method`, `ordinary::Search`,
`ordinary::interactive::{Context, Hint}`, `proofs::Structural`); `xnzzkzkw`
adds them and recounts. No marked type turned closed, so no other new
variant or field is a break. `Limits::table_atoms` (56a) is a field of a
marked struct.

Beyond the tools, from the reports of steps 54 to 58 (a reading agent's
summary, each claim checked here against the code or the report):

- **Wire level 9** (LJ's cut, `"rule": "cut"`) was missing from the
  changelog's "Wire forms"; added.
- **Level 10's line** said clausal-proof outcomes are written at level
  10; since 55a every ordinary outcome is written at 12, so the line now
  says no document of 0.4.0 is at level 10.
- **The classical default's JSON**: where the embedding proves the
  translated sequent but its LK derivation passes the derivation limit
  (8 ILTP problems, 54s), `--format json` writes the solver's clausal
  proof where 0.3.0 wrote the translated sequent's proof. The break's
  bullet now says so.
- Not breaks: the bindings' outcome shape (`deduxit-web` is unpublished;
  the client moved with it in 55d), the rewritten messages (under
  "Changed", which says the words changed and nothing a script parses
  other than words).

## What was prepared

- **The version** is 0.4.0 in the manifests and both locks (`cargo
  update --workspace`, and `-p deduxit` in `fuzz/`, which moved only the
  workspace's own packages), `CITATION.cff` (2026-11-15), the dependency
  lines of README, the library chapter and the installation chapter,
  the installation chapter's file names, the JSON examples of the
  manual (`ordinary-logic.md`, `terms.md`) and of the docs (`wire.rs`,
  `ordinary/decide.rs`). The compat crates keep their literal 0.3.0;
  their requirement on `deduxit` follows the workspace, and nothing
  publishes them again (the workflow skips them).
- **The bless** (`rqumlyxl`, a commit of its own): the JSON lock's 18
  outcomes, the serialize test's five and the wire documents of levels
  11 to 14 change by the version string alone (the lock compared with
  the version masked: equal). Levels 11 to 14 are written back byte for
  byte by `stability.rs`, and no release wrote them before 0.4.0, so they
  state 0.4.0; levels 1 to 10 keep what their versions wrote.
- **The changelog** (`zwtoktqq`): `## [0.4.0] - 2026-11-15` under an
  empty `## [Unreleased]`, a first paragraph that says what 0.4.0 brings
  (the page's countermodel, truth table and tableau and the Rocq
  library's 0.4.0 among it), the breaks above, the link `[0.4.0]`.
- **The status chapter** (`yprnvwrt`): truth tables, tableaux and Kripke
  countermodels moved from "Planned" to what exists, the page's views,
  the Rocq library's intuitionistic calculi, `close` and `hint` as they
  are (through the embedding, with the SAT solver's countermodel in the
  bindings), the embeddings one way among several; the 0.4.0 bullet of
  "Planned" removed.
- **The author's wording of the collection** (`rrxuulrn`):
  - README and the manual's introduction open with "deduxit is a
    collection of logic suites", name the linear and the propositional
    suites side by side, and carry the history note word for word: "It
    began as linlog, a linear logic library (up to 0.2.0), and is
    growing to take in more calculi." README's "What changed" lost its
    linlog sentence.
  - README's "A first proof" and the introduction (a new section, "Your
    first countermodel") show `prove --logic intuitionistic "|- A \/
    ~A"` with its countermodel, so the first page shows both suites; the
    examples test runs both.
  - The crate docs, the guide's description and first lines (summary
    and reference), the Cargo descriptions of `deduxit` and
    `deduxit-cli`, the flake's description and CLAUDE.md's project line
    drop "linear logic first". The guide's summary body is 7 997 of its
    8 000 bytes, 93 lines.
  - `CITATION.cff`'s abstract is in short sentences (54w's note), with
    the countermodels, the keywords "intuitionistic logic" and "Kripke
    semantics".
  - The dictionary line stays directly under every title (unchanged).
  - linlog elsewhere, checked by a search of the tree: `compat/`, the
    changelog's past sections, the serde aliases and their doc comments,
    the wire documents of 0.1.0 and 0.2.0, the tests of what 0.2.0
    wrote, `deny.toml`'s exception for `linlog-cli`, the harness's
    renaming of old CSV rows, the dated results and comparison data, the
    architecture and status chapters' lines on the compat crates,
    `release.yml`'s and `install.yml`'s handling of the old names, the
    Cachix cache and the machine lock. One current instruction named a
    workspace `../linlog-deps-old` (`update-deps`); `lusllslv` renames it.
- **The Rocq library** (`svrorwzs`): version 0.4.0 in its opam file and
  `CITATION.cff`, the opam tag `date:2026-11-15`, its changelog's
  `[0.4.0]` with a first paragraph and the link, README's tags `v0.4.0`
  and what 0.4.0 covers (the countermodels' certificates; the basis of
  the Horn engine's backward search among what has none), the citation's
  abstract and keywords with the intuitionistic calculi and Kripke
  models, and README naming deduxit's logics evenly.
- **The lock** (`kqzmrrxl`, "flake.lock: Update"): `rocq-deduxit` to
  `ef979655`, with `export::rocq::DEDUXIT` = "0.4.0" in the same commit
  (the flake asserts them equal), and the Rocq chapter's and the guide's
  0.4.0 tags; 55c's open note (the two rise together) is closed by it.
- **The release workflow's dry run**: its crate selection run locally
  against crates.io's API for `v0.4.0`: `linlog 0.3.0` and `linlog-cli
  0.3.0` "on crates.io already", `new=` (none), `existing=deduxit
  deduxit-cli`, so the job publishes both through Trusted Publishing and
  uses no secret. Its `cargo package --locked` of both crates (with
  `--allow-dirty` for the uncommitted lock move then): exit 0, both
  packaged and verified at 0.4.0. `cargo publish --dry-run` is denied to
  this session, as at 0.3.0.
- **The release notes** are the changelogs' sections as the workflows'
  `awk` writes them, generated at the commits to tag into
  `plan/notes/release-0.4.0.md`, with what a user can do now (54u's
  additions among it) for an announcement.

## The checks

Every build and test ran in a capped scope on cores 10–12
(`CARGO_BUILD_JOBS=3`, `RUST_TEST_THREADS=3`); every `nix` build and
check under `flock -o /tmp/linlog-machine.lock` with `--max-jobs 4
--cores 8`.

- **rocq-deduxit**: `nix flake check` at `svrorwzs` (before its commit,
  the same tree): exit 0, "all checks passed!" (every admitted Rocq,
  the tests, the bridges, "Closed under the global context").
- **`breaks 0.3.0`**: above, at `zpuvxkpt`; run again at `wnkvskmo`:
  still 4 failed checks, but the discriminant check now names
  `ordinary::Rule::Open` alone (`Unsupported`'s numbers are 0.3.0's);
  the other three failures, the warning and the public-api diff's
  removed and changed lines are as in the table, each covered.
- **`cargo test --workspace`** after the version raise, before the
  bless: only the tests that pin the version failed (`json_lock`,
  `outcome_json_format`, and `stability.rs`'s `outcomes_read_back` and
  the levels 12 to 14); after the bless the three test files pass
  (1, 24, 13 tests).
- **The examples test** (`cargo test -p deduxit-cli --test readme`)
  after the wording, with the new countermodel examples and the guide's
  budget: 7 passed.
- **The Rocq checks** at the lock move:
  `nix build .#checks.x86_64-linux.rocq-deduxit
  .#checks.x86_64-linux.rocq-deduxit-library`: exit 0 (the exporter's
  certificates, the countermodels' among them, against rocq-deduxit
  0.4.0, and the flake's assertion `DEDUXIT` = the opam version).
- **The workflow's dry run**: above (selection, `cargo package`).
- **The gate** at `kqzmrrxl` (the lock move): exit 0, "gate: passed".
  The ratchet's cache model reported moved misses (`batch-families`
  +7.90 %, `render-svg` +5.21 % on the first level), never a failure.
- **`nix flake check`** at `kqzmrrxl`: exit 0, "all checks passed!".
- After those two, `nswvxkpm` (a plan file) and `wnkvskmo` (the
  changelog, the notes and one doc comment's sentence in
  `ordinary/decide.rs`) changed only prose.

## The reviewer

`fresh-reviewer` (Fable 5.2 at medium) read both repositories' changes
once against six criteria (the breaks against the tools' output, the
versions and dates, the author's wording, the status chapter and the
guide's budget, the notes and checklist, false statements). It built
nothing; it ran both workflows' `awk` on the changelogs and asked
crates.io for `linlog` and `linlog-cli` 0.3.0.

It found the release's mechanics sound: every line of `breaks` covered,
the versions and dates consistent, the wording as the author asked, the
status chapter true, the guide within budget (3 bytes to spare), the
generated notes equal to the workflows' output, the checklist in order.
Its five findings, each a false or stale sentence, all held and are
fixed in `wnkvskmo`:

1. The SAT default's bullet said the JSON names `engine` `sat`; the
   outcome has no `engine` key since 55a (`search.method`). The sentence
   was 54s's.
2. The SAT engine's bullet named `sat::Model`, which 55b replaced by
   `kripke::Model`. The sentence was 54s's.
3. This session's level-10 line said every ordinary outcome is written
   at level 12; the table's and the tableau's are at 14. Fixed there, in
   the first break's bullet, and in `ordinary::Outcome`'s docs ("written
   at level 12 whatever it holds", 55a's).
4. The countermodel certificate's bullet said the Rocq library's "next
   release" adds `Deduxit.Intuitionistic`; it is its 0.4.0.
5. (with 3) the doc comment of `ordinary/decide.rs`.

Its adjacent note, that the LJT bullet's `--method embedding|ljt` lists
two of five methods, describes that bullet's own addition: left.

## Departures from the design

- **The README and the introduction gained an ordinary-logic example**
  (the countermodel of `A ∨ ¬A`), beyond the wording the prompt asks
  for, so that the first page shows no logic first. (departs from the
  design)
- **The release date is 2026-11-15** in both repositories, the day this
  was prepared. The release waits for 56b, so the planning session moves
  the five dates (both changelogs, both citations, the opam tag) if the
  tags are made on another day.
- **The dry run is the workflow's own selection and `cargo package`**,
  not `cargo publish --dry-run`, which this session's permission rules
  deny (as at 0.3.0).
- **No baseline night**: the author's to give (the notes' checklist asks).

## What is left open

- The organisation's profile README (`deduxit/.github`, if it was
  created from `plan/notes/release-0.3.0.md`) says "Logic suites, linear
  logic first": the planning session's, through `gh`, with the author's
  yes for a write to another repository's content if it is not covered.
- The client's `llms.txt`, the page's About and the repositories'
  GitHub descriptions may say "linear logic first" too: not this
  repository's; the planning session checks them at the release.
- `kani.yml`'s run on `main` is the only check of Kani's harnesses since
  2026-11-15; the checklist asks for it green before the tag.
- opam waits for the author (2026-11-09).

## What the next sessions must know

- The next new variant of `rocq::Unsupported`, or of any fieldless enum,
  goes last: `breaks` reports a moved discriminant even on a
  non-exhaustive enum.
- `core.md`'s list of marked types is checked against the rustdoc JSON
  (a script: every public enum and struct with a public field, by its
  `non_exhaustive` attribute; field ids in the JSON are integers while
  the index's keys are strings).
- The compat crates' requirement on `deduxit` follows the workspace's
  version; they are never packaged again, since the workflow skips a
  version crates.io has.

## Commits

deduxit, in `../deduxit-wt-step-58b-8c0bbc`, on `main@origin`
`zpuvxkpt`, oldest first:

- `yqxkskqr` Keep rocq::Unsupported's numbers as 0.3.0 had them: Basis last
- `mzrqotsv` Raise the version to 0.4.0
- `rqumlyxl` Bless the behaviour lock: an outcome states version 0.4.0
- `zwtoktqq` Date 0.4.0's section of the changelog, with every break the tools found
- `yprnvwrt` Keep the status chapter true for 0.4.0
- `xnzzkzkw` Rules: count the marked types at 0.4.0
- `rrxuulrn` Describe deduxit as a collection of logic suites, linear logic one of them
- `lusllslv` Name update-deps' comparison workspace after deduxit
- `kqzmrrxl` flake.lock: Update (rocq-deduxit to `ef979655`, `DEDUXIT` 0.4.0)
- `nswvxkpm` Plan: the release notes of 0.4.0 and the author's checklist
- `wnkvskmo` Correct what 0.4.0's changelog says it writes
- this report

rocq-deduxit, in `../rocq-deduxit-59`, pushed (`main@origin` =
`svrorwzs`, `ef979655f775`):

- `svrorwzs` Date the release of 0.4.0

The workspace `../rocq-deduxit-59` (`step59`) is empty after its commit;
the planning session removes it with this one.
