# The brief every agent of the audit reads first (step 86, phase B)

You are one agent of an audit of deduxit's Rust libraries and its Rocq
library before the release of 0.9.0. The prompt that sent you here names
your role (finder, verifier or architecture reader), your unit, your
model and your effort. This file holds what all roles share, then one
section per role. Read all of it, then the files it names for your role.

## Where everything is

Read only these two trees, by absolute path:
- `/home/tux/Projects/own/deduxit-wt-audit`: deduxit at the commit the
  audit reads. Never read a file of this repository under
  `/home/tux/Projects/own/deduxit`: that is another checkout, at another
  commit, and its line numbers differ.
- `/home/tux/Projects/own/rocq-deduxit`: the Rocq library.

In the first tree (`A` below stands for it):
- `A/plan/notes/audit/criteria.md`: the criteria, each with an
  identifier (D1 to D4, the `C-…` items, E, P, X, L, T, Q, B) and its
  source. Every finding names one.
- `A/plan/notes/audit-rubric.md`: the standing criteria of earlier
  audits (A1 to A6, S1 to S19), which still hold: a change you propose
  must not break one.
- `A/plan/notes/audit/consumer-test.md`: what got in the way of an
  outside crate that built two toy engines on the public API.
- `A/plan/notes/audit/baselines.md`: the numbers (the ratchet, the
  budget, the public API's size, Clippy's allowed lints, lines).
- `A/plan/notes/audit/rejected.md`: findings a verifier refuted. Do not
  raise one again unless you bring a witness it lacked.
- `A/plan/notes/audit/handovers.md`: what the finders of earlier units
  ask of later ones, by unit: pointers to look at, not findings.
- `A/.claude/rules/*.md`: the rules files. They record choices made on
  purpose, with their reasons. A finding against a recorded choice must
  say why its reason no longer holds.
- `A/target/audit/`: the inventories, made by tools:
  - `units.md` and `units/UNIT/files.txt`: each unit's files;
  - `lines.md`: lines per module and the largest files;
  - `public-api.txt` and `surface.md`: the public items, those reachable
    under several paths, those without an example, those that nothing
    outside `core/src` names;
  - `deps.md`: which module of the core crate names which;
  - `duplicates.md`: duplicate code of 100 tokens or more, with places;
  - `clippy.md`: the lints `Cargo.toml` allows, counted by area, and the
    functions over 100 lines;
  - `text.md`: sentences that match a pattern of the writing rules;
  - `help.md` and `help/`: every `--help` of the command and its flags
    as one table;
  - `type-sizes.txt` (12 MB: `grep` it for a type's name, never read it
    whole), `llvm-lines.txt`, `dhat.md`: the layout of every type, the
    functions with the most LLVM IR, each journey's allocations and the
    frames that allocate most;
  - `ratchet.out`: each journey's instructions under callgrind against
    its ceiling;
  - `timings.md`: compile and test times;
  - `rocq.md`: the Rocq library's build, warnings and assumptions.
  An inventory is a pointer, not a finding: open the place it names
  before you rely on it.

## Rules

- **Read-only.** No edit of either repository, no `jj` command that
  writes, no `git`, no `gh`, no push, nothing installed. The one file
  you write is your notes, `A/target/audit/units/UNIT/notes.md` (in the
  build's output directory, which the repository ignores). Your result
  goes back through the schema your prompt gives, in full; the leader
  writes the result files from it, so write no `findings.md` and no
  `verdicts.md` yourself.
- Each request you make reads your whole context again, so make few of
  them: ask for several files, or several searches, in one message
  whenever they do not depend on each other.
- Read files with the Read tool or `grep`, `head`, `tail`, `cut`, `wc`.
  Never `sed` or `awk`, no `VAR=…` before a command, no `$` in a
  pattern.
- Build or run something only if a witness needs it, and then in a
  capped scope on cores 13 to 15: `systemd-run --user --scope -p
  MemoryMax=8G -p MemorySwapMax=0 --setenv=CARGO_BUILD_JOBS=3
  --setenv=CARGO_TARGET_DIR=/home/tux/Projects/own/deduxit-wt-audit/target/agents
  taskset -c 13-15 COMMAND`, bounded in time, no job count over three.
  No `nix` command. A timed measurement is not yours: say what should
  be measured and the leader runs it.
- Start no agents of your own and no workflow.
- Do not use `max` effort, fast mode or an advisor; your model and
  effort are the ones your prompt names.
- No suite is the main one: ordinary logic, linear logic, modal logic
  and lambda-terms are peers, and so are calculi given as rule files.

## What a finding is

A place where the code, the API, the command or a text breaks a
criterion, shown by a witness, with a change that would mend it. Not a
finding: a matter of taste without a criterion; what a lint, a flake
check or a recorded decision already settles; praise; a summary of the
code.

The author decided (criteria D1 to D4): in 0.9.0 no logic is the
default, the API may break where users gain, the exit statuses stay,
and no suite is the main one. A break needs a gain for a user that you
can name.

A performance claim from reading is a **candidate**: say what to
measure and with which instrument (a journey under callgrind or DHAT, a
target row's counters, pinned time). It is not a finding of fact until
measured.

### The form

Each finding has these fields:

| field | content |
|---|---|
| id | `F-UNIT-NN`, numbered from 01, the most valuable first |
| dimension | `idiom`, `architecture`, `performance`, `api` or `text` |
| criterion | one identifier of `criteria.md` |
| place | `path:line` relative to the repository's root; for a duplicate, both places |
| claim | one sentence |
| witness | the snippet (quoted exactly, a few lines), the measurement with its command, the use that fails, or the duplicate's two places |
| change | what to do, in a sentence or two |
| kind | `refactor` (behaviour kept), `API break`, `search change`, `text` or `build` |
| value | who gains what: a user of the library, of the command, a reader of the code, the machine |
| cost | `small` (one file), `medium` (one module), `large` (across modules or crates) |
| net | which instrument shows behaviour is kept (the behaviour lock, the examples of README and the manual, the target sets' counters, the ratchet, the certificates' checks), and whether the change touches a checker, a search or a certificate |

A pattern that repeats is one finding with all its places (up to ten,
then a count), not one finding per place.

## The finder

You read one unit for five dimensions: idiom, architecture and
duplication, performance, the public API and its extensibility, text.
`A/plan/notes/audit/method.md` ("The grid") says which criteria each
dimension holds.

1. Read `criteria.md`, `consumer-test.md`, `rejected.md` and the lines
   of `handovers.md` for your unit and for every unit. In your unit's
   rules files, read the parts on your unit's modules (`grep -n` for a
   file's name finds them); they are long, so do not read them whole
   unless they are under 700 lines.
2. Read your unit's entries of the inventories: `grep` each of
   `surface.md`, `duplicates.md`, `clippy.md`, `deps.md`, `text.md` for
   your files.
3. Read every file of `A/target/audit/units/UNIT/files.txt` in full, in
   an order that follows the module's structure (its `mod.rs` first),
   several files per message. After every few files, append what you
   noticed to `A/target/audit/units/UNIT/notes.md` (with a shell
   here-document), each note with `path:line` and the few lines that
   show it. Your context may be compacted while you read; the notes
   file is your memory, so keep it current and read it again when you
   have finished reading.
4. Open code outside your unit only where a finding needs it (a
   duplicate's other place, a caller, a test).
5. Choose at most 25 findings, the most valuable first: what many users
   or readers gain from, at a cost the gain is worth. Fewer is fine; do
   not pad. Open each place again and check the line number and the
   quote before you write it.
6. Return the findings through your prompt's schema, every field of
   the form filled: the witness with its lines quoted exactly (Markdown,
   a fenced block for code), the change, the value and the net. The
   leader writes `A/target/audit/units/UNIT/findings.md` from them in
   the layout below.

For a unit of the command, `help.md` and `help/` are part of the unit,
and so are the chapters of the manual on its commands
(`A/docs/src/`). For the Rocq library, `rocq.md` is, and the criteria
are Q1 to Q11 with the text criteria.

### The layout of `findings.md`

````markdown
# Unit UNIT: findings

Read in full: N files. Not read in full: (the files, and why), or none.

## F-UNIT-01

- dimension: api
- criterion: X2
- place: core/src/linear/sequents/mod.rs:412
- kind: API break
- cost: medium
- candidate: no

**Claim.** One sentence.

**Witness.**

```rust
the lines, exactly
```

**Change.** A sentence or two.

**Value.** Who gains what.

**Net.** The instrument; whether a checker, a search or a certificate is touched.
````

`candidate: yes` marks a performance claim that is not measured.

## The verifier

You get one unit's findings in your prompt, in full, written by an
agent on another model. Your task is to refute each finding; one
survives only if you fail to. For each:

1. Open the place. Is the witness there, at that line, and does it show
   what the claim says? A quote that is not in the file, or a claim the
   code around it contradicts, rejects the finding.
2. For a duplicate, open both places: do they do the same, and would
   one implementation serve both?
3. Read the criterion in `criteria.md`: does it say what the finding
   says it does?
4. Is the matter settled already: by a lint or a check, or by a choice
   a rules file records with a reason that still holds? `grep` the
   unit's rules files for the item's name.
5. Would the change break something the finder did not see: a caller
   (`grep` for the item across `A/core`, `A/cli`, `A/web`, `A/bench`,
   `A/docs`, `A/skills`), a soundness argument a rules file gives, a
   wire form, the behaviour lock?
6. Is the value real and the cost honest? A finding whose gain is
   tidiness alone at large cost is confirmed, with that said.

Your verdict on each is one of:
- `confirmed`: the witness holds and the change is sound. Correct the
  place, the cost or the kind where the finder erred, and say so.
- `candidate`: a performance claim that reading supports but only a
  measurement decides; say what to measure.
- `rejected`: with the reason, in a sentence or two that a later finder
  can check.

Return your verdicts through your prompt's schema: for each finding, in
the findings' order, its id, the verdict, the reason in a short
paragraph that a later reader can check (the places you opened), and
any correction. The leader writes `A/target/audit/units/UNIT/verdicts.md`
from them. You do not add findings of your own, with one exception: if
checking a finding shows a defect of correctness (a wrong verdict, a
panic, an unsound check), report it in the schema's `beside`.

## The architecture reader

You judge the whole of the core crate as one design; the finders, who
come after you, read it unit by unit. You do not read every line: you
read every module's root file and module doc, the public listing, the
inventories (`deps.md` and `surface.md` above all, then `duplicates.md`
and `lines.md`), `A/.claude/rules/core.md`, `consumer-test.md`, and
code wherever a boundary needs it. Keep notes in
`A/target/audit/units/arch/notes.md` as you read.

Judge:
- **Module boundaries.** Which modules hold more than one thing, which
  thing is spread over several, which files are too large to hold in
  mind and where they would divide.
- **Layering.** What the suites share and what each owns. `deps.md`
  lists what one suite names of another (ordinary logic names 62 paths
  of the linear suite, among them `linear::search::Reason` and
  `Options`): for each such group, should the item move to a shared
  module, stay, or be duplicated? No suite is the main one (D4), so a
  shared notion that lives in one suite is a finding.
- **Duplication a trait or a generic would remove**, and the opposite:
  an abstraction that costs more than it saves. Name both places and
  what the shared form would be.
- **The engine interface (X4).** What an outside crate needs to write an
  engine for a suite and have its proof checked: constructors for
  sequents and formulas, a way to build a proof or a refutation, the
  checker's entry point, the outcome types, the limits and the stop
  signal. Sketch the smallest interface that gives this, with
  signatures, and say what it costs the present code.
- **The crate's shape.** One crate of about 215 000 lines, nine default
  features and three optional ones. Would a split into several crates
  (the shared data model and checkers; each suite; the exports) serve
  users, compile times (`timings.md`) or the web module's size? The web
  module grew from 2 885 650 to 2 987 284 bytes at gzip in the last
  four steps, and every visitor downloads it: could suites load as
  modules of their own, as the PNG and PDF renderers already do
  (`A/.claude/rules/web.md`)? Say what each option costs and what it
  would need.
- **Gaps the last steps' reports name**: `rules_candidates`, and the
  protocol's `KINDS`, which has no kind for lambda-terms
  (`A/core/src/protocol/`). Say what a consistent design would be.
- **The features.** Do the twelve features cut along lines a user
  needs? Is there one that gives the data model and the checkers alone?

Write two files in `A/target/audit/units/arch/`:
- `architecture.md`: the design as it is (the layers, who names whom,
  in a page or two), then your judgement under the headings above, each
  claim with its place. This is the main input of the API proposal, so
  be concrete: names, signatures, what moves where.
- `findings.md`: up to 30 findings in the layout above, unit `arch`.

Then return the index your prompt's schema asks for.
