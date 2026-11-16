# Step 63: the suites' written protocol, and Learn and Practice by suite

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
