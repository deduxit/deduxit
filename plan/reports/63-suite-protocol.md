# Step 63: the suites' written protocol, and Learn and Practice by suite

## Session 63a: the protocol

Opus 5.5 at `high` (the worker that designed 63g-a on Fable, its
conversation kept), in the workspace `../deduxit-wt-step-58a-610ed3`, from
the brief `/tmp/linlog-handoff/start-63a.md`, on 2026-11-16, cores 7 to 9.
The library and the schema were written first; the bindings after 62d
landed on `main`.

### Outcome

- **The library** has `deduxit::protocol` (feature `protocol`, on by
  default): each suite's kinds of values and operations between them as
  data, with each operation's parameters, the settings it reads, what it
  promises and what checking that costs; the trait `Suite` a suite
  implements (not sealed); the registry of a build's suites; and `run`,
  `check`, `read`, `summary` and `catalogue` over JSON. The three suites
  implement it: 11 operations of linear logic, 13 of ordinary logic, 6
  of calculi given as data, and 18 kinds (4 of them renderings no suite
  owns).
- **The protocol written down**: the manual's reference chapter "What
  each suite offers" (`docs/src/protocol.md`): the kinds, the
  operations, the promises and their checks, what a run depends on, the
  web module's calls by suite, and `wire.schema.json`, a JSON Schema of
  every wire form (interop-1), published with the manual and checked by
  the flake check `wire-schema` against 81 documents the command writes.
- **The bindings** speak it: `describe()` writes which call of each
  suite is which of the protocol's eleven (with their arguments, `null`
  where a suite lacks one), each suite's further calls, its layouts and
  how its proofs are built, and the library's catalogue; every `view`
  answers `layout` (`tree`, or `list` for a Fitch proof); the ordinary
  suite gains `ordinary_read` and `ordinary_certificate`, the two calls
  it lacked. Every existing call keeps its name and its answer.
- **The module question**: one module stays (D29). After 62d the module
  is 3 151 428 bytes at `gzip -9`, 3 158 833 with the protocol (7 405
  more), under the ceiling of 3 300 000.

### The library: the trait and the registry

- `deduxit::protocol` (`core/src/protocol/`): `Kind` (name, role, the
  schema's form, level, stored, label), `Operation` (inputs' kinds,
  output kind, class `derived`/`work`/`copy`, cost `instant`/`search`,
  parameters, `reads`, promise, label), `Promise` (name, the check's
  cost), the trait `Suite` (`name`, `label`, `kinds`, `operations`,
  `read`, `summary`, `run`, `check`, the last taking `Ask`, a run through
  the caller, so the workspace answers it from its cache), `suites()`,
  `shared()`, `operation`, `kind`, and the front doors `run`, `check`,
  `read`, `summary`, `catalogue`. It writes down 63g's section 3: the
  kinds and operations named there, with these differences (each departs
  from the design, none from the prompt):
  - **Values cross as documents with their `version`**, not as nested
    forms (63g section 10 said nested values carry none): a value then
    reads alone through `wire::upgrade`, and the protocol needs no
    nested writer per form. The workspace (63g-b) strips or keeps them
    when it nests them.
  - **A statement is the sequent's document with `logic` or `mode` beside
    its keys**, which the sequent's reader ignores, so it needs no new
    form and no wire level; 63g had a form `{"logic", "sequent", "text"}`.
    `wire::LEVEL` stays 18, and level 19 is still free for 63g-b's
    workspace.
  - **`duplicate` is one operation per suite** (`linear.duplicate`,
    `ordinary.duplicate`, `rules.duplicate`), since an operation's output
    has one kind; 63g had one shared `duplicate`.
  - **Drawings and texts are four kinds no suite owns** (`drawing`,
    `text`, `certificate`, `term`), each an object of the protocol's own;
    `export` writes `text`, `latex` or `typst`, and `draw` the SVG.
  - **`rules.draw` and `rules.export` take a session**, which holds its
    calculus; a derivation alone does not.
  - **The Kripke model is no kind**: an outcome holds it, and `draw` and
    `certificate` take the outcome, as the bindings do.
- **The feature `protocol`** needs `serialize`, `parse` and `interactive`
  and brings `serde_json` as an optional dependency of the core crate (a
  dev-dependency before): values are JSON text (decided here, 63g's
  section 18 point 1). The command does not enable it; the bindings do,
  and the command's tests (the manual's names).
- **`reads`** lists, per operation, every settings key its output
  depends on (the readers' `limits.occurrences` and `limits.terms`
  included; `search` for all the search's keys), checked by reading each
  function the run calls and by the reviewer's sweep of 620 single-key
  changes (one key missing, `limits.derivation_bytes` of
  `ordinary.derivation`, now added). Over-inclusion only costs the cache
  a miss.
- **Errors**: no new code. An unknown operation, kind, format or half is
  `unknown_name`, another number of inputs or bad parameters
  `invalid_option`, a value of another kind `json`.
- **Tests** (`core/src/protocol/tests.rs`, 13): the registry against its
  name lists, refusals, a translation and its check, a countermodel, an
  unknown naming its bound, an interpolant and its halves, an embedding,
  a linear proof with its net and certificate, an intuitionistic goal
  keeping its sides, a calculus given as data, and one per review
  finding (a check left open by a bound, an interpolant held to its
  input, a value of another kind refused).

### The calls and the schema

- **The manual's reference chapter** `docs/src/protocol.md` ("What each
  suite offers"): the library's example (a crate doctest too, as
  `rust_examples_are_doctests` asks), the kinds, the operations, the
  promises and their checks' statuses, what a run depends on, the web
  module's calls by suite, the schema. The guide's reference names the
  module; the status and the installation chapters list the feature.
- **The schema** (`docs/src/wire.schema.json`, draft 2020-12, 87 forms,
  published at `deduxit.github.io/deduxit/manual/wire.schema.json`):
  every document the library writes and reads, by level, written by a
  sub-agent on Opus 5.5 from the serde code, and the protocol's own
  forms added here. `additionalProperties: false` throughout, so a key a
  writer adds later fails. The check `wire-schema` (`modules/schema.nix`)
  runs the command over 81 cases (`modules/_schema/documents.sh`) and
  validates each with Python's `jsonschema` (`check.py`), which also
  fails a document that still passes with a key added, and wants a
  document of every top-level form but six it lists with why. No crate
  was added: a validator crate would have brought some twenty packages
  to a lock at 173 of its 180. The rule file keeps its own schema
  (`rules::SCHEMA`); the protocol's forms are held to the schema by name
  (`web/tests/protocol.rs`), since no command writes them.
- What the schema's author found in the code's documents (no Rust
  changed, for a later step): `wire.rs` lists neither the net
  elimination's record nor a first-order session's `instances`; it says
  an outcome holds the model as `countermodel` where the writer nests it
  as `countermodel.model`; `seq translate`'s help lists its record's keys
  without `deduxit`; and a session's `written` key is never written by
  the command (only the bindings start a session from typed text).

### The bindings

- `describe()` (`web/src/protocol.rs`): `about`'s answer with `calls`
  (the protocol's eleven names), and per suite `calls` (each `{"call",
  "export", "takes"}`), `extra` (its calls beyond the protocol's),
  `layouts` and `builds` (its methods: the ordinary suite's `natural`
  runs the `rules` suite's calls, `calculi` naming NK, NJ and NM per
  logic, which answers 62d's question of how a method names its calls),
  and `catalogue`. `about` stays as it was, `methods` (the ordinary
  suite's table and tableau) included.
- **What each suite lacks, said**: the `rules` suite has no `close`,
  `hint` or `certificate` (no search, no Rocq export of a rule file);
  its `read` reads a rule file. The ordinary suite gained `read` and
  `certificate`: `ordinary_read(input, logic, settings)` refuses what
  `ordinary_start` refuses, `ordinary_certificate(outcome, settings)`
  certifies a derivation or a countermodel.
- Every `view` answers `layout`: `tree`, or `list` for a Fitch proof.
  `graph` and `text` are the protocol's other two, which no suite's view
  draws into yet.
- Tests: `web/tests/protocol.rs`, 4: every export `describe` names exists
  in `lib.rs`, every view's layout, the ordinary suite's read and
  certificate, every kind's form in the schema.

### The module's measurement and the answer

`nix build .#deduxit-web` under the machine lock, `gzip -9` of
`deduxit_web_bg.wasm`:

| build | raw | gzip -9 |
|---|---|---|
| `main` after 62d (`252b2ff5`) | 8 480 353 | 3 151 428 |
| this chain | 8 512 908 | 3 158 833 |

The ceiling is 3 300 000 (`modules/budget.nix`): 141 167 bytes of room
remain. One module stays (D29); a module per suite (web-front-end-4)
waits for its other trigger, a suite from another repository, or for
the next suite that passes the ceiling.

### Auto-reverse (education-3)

Step 38h's switch (`interactive.invertible`) applies the invertible
rules in the linear suite only, through `Interactive::invert_all`. The
new suites need more before the switch can serve them, and this step
adds none of it (a feature, not the protocol's):
- the ordinary suite's session has no judgement of which steps are
  invertible: LK's rules are, LJ's `→L`, `∨R₁`, `∨R₂` and `¬L` are not;
  it needs `is_invertible` and `invert_all` on `ordinary::Interactive`,
  then the bindings' setting as the linear calls have it;
- a calculus given as data cannot know which of its rules are
  invertible: a rule file would have to say so (an optional key per
  rule, a wire level), and the check that a rule so marked is invertible
  is the author's, as the rules are.
Both are follow-ups for `plan/later.md`, for the planning session.

### Checks

- **The gate** (`nix develop -c gate`, capped on cores 7 to 9) at
  `zyxywqwt`: passed (Clippy, the tests with the behaviour lock, both
  `cargo hack` runs, the ratchet within its ceilings).
- **`nix flake check`** under the machine lock at `zyxywqwt` with this
  report (`suyppvno`): passed, among them `test-web` and
  `web-wasm-tests` (the bindings' tests natively and under Node, the
  protocol's four among them), `wire-schema` (81 documents, 0 failures),
  `manual` and `manual-links` (the chapter and its link to the schema),
  `budget`, `features`, `deny`, `shear`, `typos` and `conventions`.
- **Each feature alone and every pair** (`cargo hack`, warnings denied):
  passed after the fixes for the features `protocol` alone and with one
  export feature, which an earlier run found (imports and variants
  unused without `svg`, `rocq`, `latex` or `typst`).
- **The protocol's tests** (13 in the library, 4 in the bindings) and the
  manual's test (`cli/tests/readme.rs`, which now tells the protocol's
  names, `ordinary.decide`, from settings keys) pass.

### The reviewer's findings

`fresh-reviewer` on Fable 5.2 (the writer on Opus) ran every operation
on small inputs, read each output back by its kind, ran each twice,
swept 620 single-key changes of the settings against each `reads`,
validated the command's 81 documents and 70 protocol outputs against the
schema, and Clippy on five feature sets. Eight findings and a question;
all held and were fixed in `zyxywqwt`:
1. **`same_verdict` failed when one side stopped at a bound.** Now the
   check is `open`, the decided side `held`.
2. **The `interpolates` check panicked** on an interpolant whose half
   lacked the formula, which `read` had accepted. Reading an interpolant
   now checks each half holds the formula where it belongs.
3. **The `interpolates` check never looked at the input.** It now
   compares the halves with the input's parts by the partition (a part
   `halves` before the atoms).
4. **`ordinary.derivation`'s `reads` lacked `limits.derivation_bytes`**:
   added.
5. **Inputs of a kind `from` does not list were taken** (an outcome as
   a linear proof, a derivation where an ordinary operation takes a
   session or an outcome): refused now by the operation's `from`.
6. **`read("ordinary.outcome")` took any value with a `verdict`**: only
   an outcome valid by a clausal proof is read by its keys now, its
   `logic` required; every other goes through its reader.
7. **`web/src/rules.rs` was not formatted**: formatted.
8. **The new public types were missing from `core.md`'s marked list**:
   added, 160 now.
- The question (the check's searches did not obey the caller's stop):
  they do now, one stop shared through a `RefCell`.

### What the next sessions must know

- **63b (Learn and Practice by suite, the client)**: `describe()` is the
  registry's source from now on: each suite's calls by the protocol's
  names with their arguments, `null` where a suite lacks one, its
  layouts and its `builds` (a method that runs another suite's calls
  names it in `runs`, with the calculus per logic). `about()` is
  unchanged. Every view answers `layout`.
- **63g-b (the library and the command of the workspace)**:
  - Take every kind and operation from `deduxit::protocol`: `run` with
    the recipe's settings, `check` with an `Ask` that answers from the
    workspace's records, `Produced::limit` for an unknown's bound (its
    `reason` and its `setting`, `None` where no key lifts it, as 63g's
    review asked), `Check::parts` naming each part.
  - The cache key is an operation's `reads`; a check's result depends on
    the `reads` of the operations it asks for.
  - Values are documents with their `version`; nest them as the
    workspace's form says (63g section 10) or keep them whole, and say
    which. `wire::LEVEL` is still 18.
  - An ordinary outcome valid by a clausal proof has no reader: store it
    in memory only, as 63g said; `protocol::read` takes it by its keys.
  - `linear.goal` refuses a goal holding instances of quantified
    formulas.
  - The feature `protocol` is off in the command: turn it on there for
    `deduxit workspace`.
- **63g-c (the page)**: `describe()`'s `catalogue` lists each kind's
  label and each operation's label, parameters and promise, which the
  cards and menus read. A check's statuses are `held`, `failed`, `open`,
  `recorded`, `outside`, `none`.

## Session 63b: Learn and Practice by suite

Opus 5.5 at high, cores 10-12, the reviewer on Fable 5.2. From the brief
`start-63b.md`. The client on its `main@origin` (`pytslsny`, 62d and its
lock update); deduxit unchanged but for the design note
`plan/notes/63b-lesson-format.md` and this report.

### Outcome

- The course format is data with a schema per suite, its JSON Schemas
  beside the content.
- The linear course moved under `src/content/linear/` unchanged, which
  a test shows file by file.
- The ordinary suite has a store of 62 problems, decided by the module
  in a unit test and by the command in a flake check.
- Practice and Learn's tutorial follow the current tab's suite; the
  ordinary course's lesson 3 (the sequent calculus LK) is written and
  plays on the stage, its other five lessons are listed "to come".

What each suite has now: **linear**, a course (eleven lessons, 452
problems, the rules reference); **ordinary**, a store of 62 problems to
decide in Practice and one lesson of six; **a calculus of one's own**,
no course (a tool: Learn says so in one sentence).

### The format

- **A folder per suite**: `src/content/linear/` (`problems/`,
  `lessons/`, `rules/`) and `src/content/ordinary/` (`problems/`,
  `lessons/`).
- **Schemas from shared parts** (`src/lib/course.ts`): a lesson's core
  every suite shares (D56: `lesson`, `title`, `worked`, `choices`,
  `steps`, `faded`, `explain`, `hinge`); `linearLesson` is the core and
  the linear parts, field for field the schema the lessons had;
  `ordinaryLesson` the core, `method` (`sequent`, `natural`, `tableau`,
  `table`), `logic`, `layout`, `model`; `linearProblem`,
  `ordinaryProblem`. Their JSON Schemas (`z.toJSONSchema`, the input
  side) are `src/content/<suite>/lesson.schema.json` and
  `problem.schema.json`, held equal by `course.test.ts`
  (`DEDUXIT_WRITE_SCHEMAS=1` writes them after a change on purpose): the
  format other tools read (education-10).
- **A worked proof needs no step format**: the player reads the
  finished session beside the problem (`<id>.sequent.json`), takes it
  back with `ordinary_undo` one step at a time, and shows state `k` on
  the lesson's tab; the rules of the steps are the session's history's.
  The custom-calculus calls have `undo` too, so natural deduction and
  tableau lessons play the same way (63c). The linear player is
  unchanged.

### The move

The collections keep their names and every id (an Astro id is the path
under the collection's base, so no `worked:` reference and no template
key changed); only the bases, `index.astro`'s globs, the tests' paths,
the flake's `problems` check and two comments moved. `moved.json` holds
every moved file's SHA-256, taken before the move (912 files), and
`src/content/linear/moved.test.ts` holds the tree to it; the linear
lessons' and problems' own tests and the Learn and Practice e2e specs
pass unchanged.

### The content and the skills

- **The store** (`src/content/ordinary/problems/`, 62): 48 laws, each
  a valid problem in the weakest logic that proves it and a not-valid
  one in the strongest logic where it fails (27 minimal, 4 needing ex
  falso, 10 classical, 7 fallacies): the store shows where each logic
  stops. Verdicts decided by `deduxit prove --logic` before they were
  written, then by the module in `problems.test.ts` and by the command
  in the flake's `ordinary-problems` check.
- **The course's order** (`explicit-instruction-sequence-builder`): 1
  valid by the truth table, 2 signed tableaux, 3 the sequent calculus
  LK, 4 natural deduction backwards, 5 forwards, 6 classical,
  intuitionistic and minimal with a countermodel; each one objective and
  one misconception (the design note lists them).
- **Lesson 3** (`cognitive-load-analyser`'s rules from 62d: one new
  thing at a time, the reason beside the step): excluded middle in LK,
  three steps, a prediction per step, the self-explanation at the
  densest point (why LK may keep both disjuncts), the faded proof from
  its first step, four problems as practice (one of them not valid
  intuitionistically), and a hinge on "∨R must choose a disjunct".
- `worked-example-fading-designer`, `practice-problem-sequence-designer`,
  `retrieval-practice-generator` and `ai-feedback-design-principles`
  were not loaded: the form is D56's, fixed, and lesson 3's fading is
  one step; they belong to 63c's five lessons.

### Practice and Learn by suite

- **Practice** follows the current tab's suite: an ordinary tab's lists
  the ordinary store (`/ordinary-problems.json`), filtered by its
  connectives and the logic posed, never a verdict, written in ordinary
  signs, each opening in its logic; every other tab's is the linear
  store as before. Each store keeps its filters and proved problems
  apart in the browser (the linear key unchanged).
- **Learn's tutorial** follows it too: an ordinary tab's is the ordinary
  course, a linear tab's the linear course as it was with one line at
  its end that opens an ordinary tab, a custom tab's one sentence that
  it has no course. Not overwhelming: a visitor who comes for linear
  logic meets one new line, after the eleventh lesson.

### Auto-reverse (education-3)

Not needed by the ordinary course as written: its worked proofs are
played one step at a time with a reason each, and D9 forbids a step the
visitor did not take. LK's propositional rules are all invertible (G3),
so an "apply the invertible steps" button on an ordinary tab would close
much of a classical proof at once, and that would be more useful to a
visitor practising than to a lesson. 38h's toggle (`interactive.invertible`) is the linear calls'; the
ordinary bindings have no such option. A later small step, if wanted.

### Checks

- **The client's unit tests**: 641 passed, among them the move's test
  (912 files), the schemas' (4), the ordinary store's (4: count, every
  verdict by the module, the categories, no sequent twice) and the
  course's (lesson 3: its session against the problem, its steps, the
  undo walk back to the sequent, the hinge).
- **The client's e2e suite**: 279 passed, among them Practice in an
  ordinary tab (`practice.spec.ts`), the course (`course.spec.ts`, 2),
  and the walks `ordinary` (with ordinary Practice) and `coursing` in
  the target and axe specs at both widths and in both schemes.
- **The client's `nix flake check`** at the head under the machine lock:
  passed, the new `ordinary-problems` check among its checks (62
  problems and 1 session as deduxit decides them), the linear store's
  `problems` check on its new path.
- **deduxit**: nothing changed but `plan/` (the design note and this
  report), so no gate or flake check ran for it.

### The reviewer's findings

`fresh-reviewer` on Fable 5.2 at medium, once, on the chain up to the
lesson, with the page served from the e2e build and driven by Playwright.
What held: all 62 verdicts decided anew in the three logics (186 runs of
`prove --logic`), every logic category the weakest logic that proves
it, every one-line text true; the flake check fails on a flipped verdict
and a truncated session; the move, 912 of 912 files equal to their
hashes before it; the four JSON Schemas equal to the schemas; a linear
visitor's Learn and Practice as before; the visitor's path through
lesson 3 and ordinary Practice. Its four findings, all fixed:

1. **Lesson 3 taught the wrong difference between LK and LJ**: that
   LK's `∨R` keeps both disjuncts and LJ's chooses. Gentzen's LK has
   LJ's disjunct-choosing rules too, and the page's LK offers `∨R`,
   `∨R₁` and `∨R₂`; what separates them is how many formulas may stand
   right of `⊢`. The step's reason, the self-explanation, the hinge and
   its misconceptions now say that (one option now names the
   misconception "LK and LJ differ in their rules for the connectives",
   another "no proof found means the search gave up"), `∨R₁` is among
   the rules to predict, and the opening says "at least one formula".
2. **The hinge printed "lJ's"**: lowering a misconception's first letter
   lowered an acronym. Fixed (`lowered` keeps a capital before a
   capital).
3. **Two comments cited a decision number** (D56). Fixed.
4. **The worked proof played on after its tab was closed**. Fixed: the
   next step opens the tab again. A step the visitor takes by hand on
   the lesson's tab is still overwritten by the next prediction, as the
   linear lessons' are.

The axe walk of the course then found the licence's link in running text
with no underline (visible in the course's short list, not in the
linear ones): `.frame-licence a` is underlined now, in every place of
Learn.

### Commits

The client (`deduxit.github.io`), on its `main@origin` (`pytslsny`), in
the workspace `deduxit.github.io-61b`:

- `ouymmlyq` Move the linear course under src/content/linear, unchanged.
- `vktpzsqn` Write the course format by suite as schemas, with their
  JSON Schemas beside the content.
- `rummuyoy` Add the ordinary suite's store of 62 problems, each
  decided by deduxit.
- `toopkqxx` Offer each suite's store in Practice: the ordinary store in
  an ordinary tab.
- `vosmqtru` Show the ordinary course in an ordinary tab's Learn, and
  play its lessons' worked proofs.
- `vmtksloy` Write the ordinary course's lesson on the sequent calculus
  LK.

deduxit, on `main@origin`: `tmnpvxqp` (the design note), and the report.

### Departures and decisions

- **The course follows the tab's suite** rather than a suite switch in
  Learn's header (the design note left it open): it adds nothing to a
  linear visitor's Learn but one line.
- **Five of the six ordinary lessons are "to come"** (departs from the
  design: the brief asks for lessons on tableaux and truth tables,
  natural deduction and countermodels too). The format, the store, the
  player and one lesson took this session; the rest is content for 63c,
  which the step plans for when the content needs it.
- **The ordinary store has no `literature` problems**: no source was
  read for page references, and none is cited unread.

### What the next sessions must know

- **63c (the rest of the ordinary course)**:
  - Lessons 1, 2, 4, 5 and 6 (`ORDINARY_LESSONS` in the client's
    `src/lib/content.ts`), each a file `src/content/ordinary/lessons/<n>-<name>.md`
    of `ordinaryLesson` with its worked session beside its problem
    (`<id>.<method>.json`) and problems of the store filed under it
    (`lesson`, `order`).
  - The player (`Course.tsx`) plays the `sequent` method alone: the
    `natural` and `tableau` methods need the same undo-based player over
    `rules_undo` and the natural deduction view (62d's `Natural.tsx`
    keeps its sessions in a module store keyed by tab and sequent: a
    lesson must put its state there), and `table` a player of rows (the
    lesson's `steps` read as the table's rows, its `rule` "falsifies" or
    not: the schema allows it, nothing reads it yet).
  - `ordinary/lessons.test.ts` asserts `method` `sequent` and reads the
    session's history; a natural or tableau lesson needs its own
    branch (`rules_view`, `rules_undo`) and the flake check already
    holds `.natural.json` and `.tableau.json` to `check --calculus`.
  - The skills to load: `worked-example-fading-designer`,
    `practice-problem-sequence-designer`, `retrieval-practice-generator`,
    `ai-feedback-design-principles`.
- **63a (the protocol)**: the course format names the suite by folder
  and the session documents by method; when the protocol gives every
  suite `undo` and `view`, the player in `Course.tsx` takes it (one
  player for every method), and `index.astro`'s glob of
  `<id>.<method>.json` stays.
- **63g-c (the workspace graph)**: a lesson's worked proof is a session
  document beside a problem (an ordinary one at wire level 8, a rules
  one at 17 or 18), and a problem is `{sequent, logic, verdict}`: both
  are values with a JSON form, nodes for the graph as they are.
- **68**: the ordinary store has no `literature` problems; sources read
  with their page would add them (`source`, `url`, as the linear
  store's).

## Session 63c: the ordinary course

Opus 5.5 at high, cores 10-12, the reviewer on Fable 5.2. From the brief
`start-63c.md`, the client on its `main@origin` (`vmtksloy`, 63b).

### Outcome

The ordinary course has its six lessons: the truth table, signed
tableaux, the sequent calculus LK (63b's), natural deduction backwards
and forwards, and the three logics with a countermodel. One player
plays every method: a session taken back step by step (`ordinary_undo`,
`rules_undo`) for the sequent calculus, natural deduction and tableaux,
and a truth table's rows read in the lesson. Every worked proof is a
session the command checks in the flake check; every step's rule is
held to the session (or for a table to deduxit's own table) in a test.

What each suite has now: **linear**, its course (eleven lessons, 452
problems); **ordinary**, a course of six lessons and a store of 62
problems, 22 of them practice of a lesson; **a calculus of one's own**,
no course.

### The lessons and their sources

| lesson | method, logic | worked | practice |
|---|---|---|---|
| 1 Valid, by the truth table | table, classical | `a → b, b ⊢ a` (affirming the consequent): four rows, the third falsifies | 4: denying the antecedent, the converse, one disjunct, modus ponens |
| 2 Signed tableaux | tableau, classical | `¬¬a ⊢ a`: `T¬`, `F¬`, `×` | 4: Dummett's law, a split antecedent, distribution, cases |
| 3 The sequent calculus LK | sequent, classical | `⊢ a ∨ ¬a` (63b) | 4 (63b) |
| 4 Natural deduction, backwards | natural, NJ | `⊢ a → (b → a)`: `→I`, `→I`, a line in force | 4: Frege's axiom, currying, contraction, three negations |
| 5 Natural deduction, forwards | natural, NJ | `a ∧ b ⊢ b ∧ a`: `∧E₂`, `∧E₁`, `∧I`, a line | 3: both at once, the syllogism, uncurrying |
| 6 Classical, intuitionistic, minimal | natural, NK | `⊢ a ∨ ¬a` by `RAA`, nine steps; the countermodel of `⊢ a ∨ ¬a` in intuitionistic logic | 3: contraposition back, a negated implication, Peirce's law |

- The order is 63b's (`explicit-instruction-sequence-builder`); each
  lesson one objective and one misconception in its hinge: "valid means
  true in some row", "an open branch means the tableau is unfinished",
  "a discharged hypothesis must be used", "any line above may be
  cited", "no proof found means the search gave up".
- **Fading** (`worked-example-fading-designer`): from the end, where
  the steps are routine; the visitor takes over after row 2 of the
  table, step 1 of the tableau, step 1 backwards, step 2 forwards, step
  5 of the reductio. Practice is near transfer, in the lesson's method
  (a table, a tableau to build, natural deduction).
- `cognitive-load-analyser` (62d's analysis) and `frontend-design`
  again: one direction at a time, the reason beside the step, the
  table's rows in the lesson so that the stage does not give away the
  prediction. `practice-problem-sequence-designer`,
  `retrieval-practice-generator` and `ai-feedback-design-principles`
  were not loaded (departs from the design): the practice lists are
  three or four problems each from the store, ordered by the store's
  levels, and the feedback is the hinge's form, fixed by the linear
  course; loading them would have cost context the five lessons needed.

### The player

- One `states` per lesson: the finished session and every state before
  it, taken back by `undo`, each shown on the method's stage:
  `sequent` an ordinary tab (`showOrdinarySession`), `tableau` a custom
  tab in the calculus `tableau` (`showCustomSession`), `natural` an
  ordinary tab set to natural deduction (`putNatural`, 62d's view's
  store; its own start leaves a lesson's session be). A goal closed by
  a line is the step `line`, read "a line in force".
- A truth table has no session: its steps are its rows (`holds`,
  `falsifies`), read in the lesson, and the whole table opens on the
  stage at the end; its faded part is the last rows predicted without
  reasons.
- Lesson 6's `model` opens the sequent it refutes in its logic and lets
  deduxit decide it, which draws the countermodel.
- Two faults the walks found, fixed: opening an ordinary sequent dropped
  the calculus's method, so a problem opened for natural deduction came
  up in the sequent calculus (`openOrdinary` keeps it now); and the
  course left Learn when the tableau's custom tab became current (the
  open lesson is module state, `ordinaryLessonOpen`).
- **63a's protocol**: the player calls `ordinary_undo`, `rules_undo`,
  `rules_view` and the stages' own entry points; when the protocol
  gives every suite `undo` and `view`, `Course.tsx`'s `states` and
  `showAt` become one branch.

### Checks

- **The client's unit tests**: 661 passed, among them the course's 29
  (each lesson: its problem valid in the lesson's logic or a classical
  table; its steps against the session's history or deduxit's table,
  each among the rules to predict; the undo walk back to the sequent;
  the hinge and practice; lesson 6's model not valid in its logic).
- **The client's e2e suite**: 283 passed, among them `course.spec.ts`'s
  6 (one per method, the countermodel, a practice problem opening by
  natural deduction, the course kept on the tableau's tab, the Fitch
  drawing of a natural deduction lesson) and the `coursing` walk with
  the table's and a natural deduction lesson's states, in the target
  and axe specs at both widths and in both schemes.
- **The client's `nix flake check`** at the head under the machine lock:
  passed, its `ordinary-problems` check with 62 problems and 5 sessions
  as deduxit decides them.
- **deduxit**: only `plan/` changed (this report), so no gate or flake
  check ran for it.

### The reviewer's findings

`fresh-reviewer` on Fable 5.2 at medium, once, on the two commits, with
the page served from the e2e build and driven by Playwright. What held:
every claim of the five lessons but one (below) against the rule files;
the 23 steps of the four sessions against `interact --state` and
`proof`; the table's rows against the page's table; the five worked
proofs played with right and wrong predictions on their stages, the
self-explanations at their place, the 20 hinge options; the faded parts
of lessons 2, 4 and 5 finished by hand and checked valid; all 18
practice problems opening in their lesson's method; lesson 6's
countermodel; a linear visitor's Learn as before; the flake check
failing on an unfinished session. Its six findings, all fixed:

1. **Lesson 1 asked about rows it never showed**: "Row 1 of 4: does it
   falsify the sequent?" with no assignment anywhere, and the faded
   rows never showed theirs. Each row's assignment (`a true, b true`) is
   now in the question, read off `ordinary_table`; the part is headed
   "The table, row by row", started by "Start with the first row" and
   answered by "Show the answer".
2. **`layout: fitch` was read by nothing**: lessons 4, 5 and 6 played in
   the tree, whose bracket numbers contradicted the reasons' line
   numbers. A natural deduction lesson now sets the view's drawing to
   its `layout` (`setNaturalLayout`); the e2e test checks the Fitch
   drawing's line ids.
3. **Lesson 6's negation clause was wrong**: "`¬A` holds at a world when
   `A` holds at no later world" makes `¬a` hold at the model's last
   world, where `a` holds. Now "neither there nor at any later world",
   in the opening and the hinge.
4. **"All lessons" from the tableau lesson showed the linear course**
   (its custom tab is current). The course now stays in Learn from a
   lesson's opening until the visitor goes to a linear tab.
5. **Lesson 1's practice gave nothing to do**: each problem opened on
   the Table view, which already gives the verdict and the row. It now
   opens on the tree, the visitor asked to decide it row by row and then
   compare with the Table view.
6. **"A …" misconceptions printed a capital mid-sentence**: 63b's
   `lowered` kept every capital before a lower-case letter. It now
   lowers the first letter unless a capital follows it (`LJ's` kept).

Its questions, answered in the content: the logic beside a table's or a
tableau's practice problem is no longer shown (they open in classical
logic); lesson 4's practice needs `→E` and `¬E`, which lesson 5
teaches, and each is still doable by working backwards; lesson 6's
practice is three NK proofs and its countermodel part, not "decides
sequents in the three logics" (the store's Practice does that).

### Commits

The client (`deduxit.github.io`), on its `main@origin` (`vmtksloy`), in
the workspace `deduxit.github.io-61b`:

- `srzvrvnq` Play the ordinary course's lessons by natural deduction, as
  tableaux and as truth tables (the player, `putNatural` and
  `setNaturalLayout`, `showCustomSession`, `openOrdinary` keeping the
  method, the course kept in Learn, the flake check resuming sessions).
- `wqlqqwyo` Write the ordinary course's lessons on truth tables,
  tableaux, natural deduction and the three logics (the five lessons,
  four sessions, 18 problems filed as practice, the tests, the walk).

deduxit: the report.

### Departures and decisions

- The three skills not loaded, above (departs from the design).
- **The flake check resumes sessions** (`interact --state`, then
  `proof`) rather than `check --calculus`, which reads a derivation and
  refused a session document: 63b's script named the wrong command, and
  no session had met it yet.
- **Port 8790** was held by another program on this machine during the
  e2e runs; the local runs used `E2E_PORT=8797`. The flake check runs in
  its sandbox and is not affected.

### What the next sessions must know

- **63g-c (the workspace graph)**: a lesson's worked proof is a session
  document beside its problem, by method (`<id>.sequent.json` at wire
  level 8, `.natural.json` and `.tableau.json` at 17 or 18); a lesson
  of the table method has none. These, and the problems, are the
  values a graph's node would hold.
- **68**: the ordinary store still has no `literature` problems; the
  lessons cite no source but the rule files' own (`calculi/n*.json`,
  `tableau.json`), whose `description` names theirs.
