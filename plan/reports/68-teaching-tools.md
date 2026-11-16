# Step 68: tools for teachers

## Session 68a: the tableau checker

Opus 5.5 at `high` (a reused worker, its conversation cleared), in the
workspace `../deduxit-wt-step-58b-8c0bbc`, from the brief
`/tmp/linlog-handoff/start-68a.md` and `plan/68-teaching-tools.md`, on
2026-11-16, cores 10 to 12.

### Outcome

A student's signed tableau of classical logic, written by hand as text,
is checked step by step in the library, the command and the bindings:

- `ordinary::WrittenTableau::parse_within(text, &limits)` reads it;
  `review(&calculus, expected, &limits, stop)` answers a
  `TableauReview`: `standing` (`Closed`, `Open`, `Unfinished`,
  `Wrong`), the sequent its first lines give, the text line and the
  tableau's line the message is about, the message in words, and an open
  tableau's checked countermodel.
- `deduxit check --tableau [FILE] [--sequent SEQ]`: the verdict line
  `{closed|open|unfinished|wrong} tableau of SEQUENT: MESSAGE`, exit 0
  for a right and finished tableau, 1 for a wrong or unfinished one, 2
  for text that is no tableau (a caret under the place), 3 at a limit;
  `--format json` the review.
- The bindings' `ordinary_tableau_review(text, sequent, settings)`, with
  its test under Node and natively, listed among the ordinary suite's
  extras by `describe`.
- The rule file `tableau` gained the textbooks' rules for `↔`, `T↔′`
  and `F↔′` (by the sides' values), beside LK's two implications, and
  `F→₁`, `F→₂` (one result of `F→` alone, Smullyan's rule A).

The commits on `main@origin` (`ylwqwvpw`): `prrsputk` (the library),
`rrwwzwvn` (the command), `qrlumlzs` (the behaviour lock), `lmzzzpkx`
(the bindings), `qrrvkrwu` (the wire schema), `vuyymlzx` (the manual,
the guide, the changelog, the status), `umswwqyz` (the review's
fixes), `xnxxqxwr` and `zlwskqws` (what the flake's spelling and
formatting checks asked), and this report's.

### The written format, and its sources

- **Signed**, `T` and `F`, as Smullyan's tableaux and the Open Logic
  Project's (the research note's [3]) are, and as step 56's tableau and
  its rule file are: a branch is a set of signed formulas, which is what
  the rule file checks. The unsigned trees of forall x (UBC's Ichikawa-
  Jenkins rules, Carnap's `TruthTree`) are not read: their `¬A` on a
  branch is `T ¬A` here, and their rules (`¬(A ∧ B)` into `¬A | ¬B`)
  take two signed steps; a second rule file and table would serve them
  (a follow-up, below).
- **The text `prove --method tableau` prints reads back**, as the
  simplest thing to type and the one the student already sees: a line
  is its number (`3` or `3.`), its sign, the formula in the ordinary
  syntax (ASCII or Unicode), and at its end the number of the line it
  expands, which a rule's name may precede in any of the spellings
  textbooks use (`T∧`, `∧T` as Open Logic writes it, LK's `∧L` as
  deduxit prints it, `T/\`), with an index `₁`/`₂` ignored. The first
  lines that cite no line are the sequent's (Open Logic's "assumption"
  lines).
- **Branches**: the drawn tree (`├─`, `└─`, `│`), its ASCII form (`|-`,
  `` `- ``, `+-`, `\-`, `|`) and a plain indented list (`-` at the
  parent's column, the branch's lines after it) all read, because the
  reader places a line by the column of its content and a branch by the
  column of its marker. The list is what a person types: the course's
  indented tree the brief names.
- **Marks**: `×` (also `x`, `*`, Open Logic's `⊗`) with the two lines
  of a formula true and false, the one line of `⊥` true or `⊤` false, or
  none (the check then finds the pair, as Carnap's "mark branch as
  contradiction" without rows); `open` for a finished open branch.
- **Limits**: the formulas within `limits.occurrences` (the sequent
  parser's own count), line numbers and branches within `u32`, the text
  within the command's memory bound as every input is.

### The check and its messages

- **Two passes, the library's checker last.** The review's own matcher
  walks the branches in text order and names the first fault in words;
  only a tableau without fault is then written as a derivation in the
  rule file `tableau` (a node per expansion, its branch's signed
  formulas as the set zone, the principal by its position, a split's
  premises in the rule's order whatever the branches' order on paper,
  unmarked and open branches as open goals) and checked by step 60's
  `Derivation::check` (all branches closed) or `check_open`. A refusal
  there is `Wrong` with the rule file's reason. So `Closed` rests on the
  rule file's checker and `Open` on that check plus
  `Countermodel::found` of the open branch's true atoms: the matcher
  decides what the student is told, never the verdict. No meter of
  `rules/check.rs` was needed beyond the checker itself (the brief's
  note on 67a): the walk is linear in the text, and the planned nodes,
  which hold a branch each, are charged to `limits.memory_bytes`.
- **The matcher's table is the rule file's**: `shapes` lists exactly
  the rules of `tableau` that expand a signed formula, the partial ones
  (`T∧₁`, `F∨₂`, `T↔₁` …) marked, and a run of lines citing one line
  passes where it is within a result list (as a multiset) and the branch
  with it holds the whole list, which is when the rule's node is an
  instance in a set zone. A result written again where the branch has it
  already passes, as the rule file's sets allow.
- **The messages** (each pinned in `written/tests.rs` or the lock):
  - `line 4: T∧ on line 1 (T p ∧ q) adds T p and T q, not T r`;
  - `line 2: F→ on line 1 (F p → q) adds T p and F q, not T q`;
  - `line 2: T∨ on line 1 (T p ∨ q) splits the branch in two, one that
    starts with T p and one with T q`;
  - a split of a rule that does not split, a split whose branch starts
    with the wrong results (named at that branch's first line), a third
    branch;
  - a line that cites no line, a line not in the tableau, one after it
    or off its branch, an atom, `⊤` true or `⊥` false expanded, a rule
    name that does not fit the line it cites;
  - `×` that names lines which do not close, or names none and nothing
    closes; `open` on a branch that closes or has a line not expanded;
  - unfinished: `the branch ending at line 2 is not finished: line 1
    (T p ∨ q) is not expanded on it`, or `is not marked: it closes on
    …, so mark it ×`, or `is finished and nothing closes it, so mark it
    open`;
  - the expected sequent: `the first lines give the sequent p ⊢ p, and
    the tableau is to be of p ⊢ q`.
- **"Finished"** is the Hintikka condition read off `shapes`: every
  line has a whole result list of one of its rules (not a partial one)
  on the branch. It decides the messages only; the open verdict rests on
  the countermodel's check.
- **Modal**: not done (see Departures).

### The command and the bindings

- **The command's shape**: a flag of `check`, `--tableau`, since
  `check` is where a file a person or another tool made is judged, and
  the file is text that no JSON probe could tell from anything else
  (the command never guesses a kind from content, `cli.md`). `--sequent`
  is the existing flag, its help extended: the sequent the tableau must
  start from, as an exercise sets it. Mode flags, `--logic`, `--system`,
  `--calculus`, `--world` and the drawn formats are refused.
- **The bindings**: `ordinary_tableau_review(text, sequent, settings)`,
  `sequent` `""` for none, the rule file read per call (nothing is kept
  between calls, `web.md`); a text that is no tableau is the library's
  parse failure with its place (`details.line`, `span_utf16`), which
  68c's editor can underline.
- **JSON** (written only, level 14, the tableau's): `{"version": 14,
  "standing", "sequent", "text_line", "line", "message",
  "countermodel"}`, in the manual's schema as `tableau_review`, with
  three documents in the `wire-schema` check's set.
- **Options (D15, D16)**: the review has no option a user would vary
  beyond the input: the bounds are `Limits`' (`occurrences`,
  `memory_bytes`, `work` through the checker) and the expected sequent
  is an argument. The command sets the bounds by its usual flags, the
  page by its settings.
- **Where quantifiers go (D17)**: a line's formula would become a
  first-order formula and `shapes` would gain the γ and δ rules with a
  witness read from the line (the term written), the rule file's
  `fresh` condition the δ rule's; the reader and the walk need nothing
  else.

### The checks

- The library's tests of the review (`ordinary::written`, eleven: the
  method's tableaux of 160 sequents read back in three layouts with
  their verdicts, every expanding line of 40 of them changed to an atom
  no line has and found at or before its line, each message, the
  unfinished cases, the countermodel, the sequent given, the layout's
  errors, the memory bound's refusal, the review's findings).
- The rule-file tests (`core/tests/rules.rs`, the count now 25), the
  behaviour lock (six new calls, `qrlumlzs`), README's, the manual's and
  the guide's examples (`cli/tests/readme.rs`), the bindings' tests
  natively (`web/tests/ordinary.rs`, `protocol.rs`).
- The wire schema's documents with the debug build: 89 documents, the
  three new `tableau_review` ones among them, 0 failures.
- The rustdoc of the library with every feature, warnings denied: clean.
- The gate (`nix develop -c gate`, cores 10 to 12) at `umswwqyz` rebased
  onto `main@origin` `ylwqwvpw`: passed (clippy, every crate's tests, both
  `cargo hack` runs, the ratchet). The ratchet noted cache misses moved
  on `render-svg`, "not a failure"; this step changes no drawing.
- `nix flake check` (under the machine lock, `--max-jobs 4 --cores 8`):
  the first run failed `typos` (a variable `ba`, renamed in
  `xnxxqxwr`) and the second `treefmt` (files edited from the shell,
  formatted in `zlwskqws`); the third, at `zlwskqws` on `main@origin`
  `ylwqwvpw`, passed, the bindings' tests under Node
  (`web-wasm-tests`), `wire-schema` and both `cargo hack` runs among its
  checks. Rebased since onto `ypwvprqu`, which adds only plan files
  (68b's report and the step log).

### The reviewer's findings

`fresh-reviewer` once (Fable 5.2), on the six changes then made, judged
on the brief's criteria. **No wrong tableau accepted** held: it read
`Walk::plan` and `derivation` (branch sets, positions, the premises'
order through the split's `order`) and ran about 720 single and double
mutants of printed tableaux, every `closed` or `open` answer compared
with `prove` on the sequent: no disagreement, no crash. About 400 random
sequents went through `prove --method tableau` and back in every layout.
Its seven findings, six fixed in `umswwqyz`:

1. **A panic on a `×` line with a wide space** (U+3000): `words` counted
   a separator as one byte. It now reads each word's offset off its
   place in the text.
2. **A panic with `--sequent` when the first line cites a line** (`1 T p
   1`): the review indexed the empty list of the sequent's lines. It now
   names the text's first line.
3. **A right tableau refused: `¬R` or `~R` before a line number** read
   as the rule F¬, cutting the formula. A word is now the rule's name
   only where the formula reads without it; else it is the formula's.
4. **A formula's parse error placed at the end of the text**: the parser
   read only up to the formula's end, so its "end of input" pointed past
   the text. `formula_in` now places it at the character after the
   formula, in the whole text.
5. **Smullyan's `F→` with one result refused** (rule A takes α₁ or α₂
   alone, which `T∧₁` and `F∨₂` already allowed): `F→₁` and `F→₂` join
   the rule file and the table. Every rule that does not split now has a
   part per result, so the "is missing" message could no longer happen
   and is gone.
6. **A wrong `T↔` step described by a part of the rule** (`adds T p →
   q`) where the tableau held one implication elsewhere: the message now
   names the rule as a whole, the split by the sides' values.
7. **The empty sequent's tableau** (`prove … ' |- '` prints `open`
   alone) does not read back: not fixed. A tableau of no line is no
   exercise, and the reader would need a branch without lines everywhere
   it assumes one; the reader's error says the tableau starts with the
   sequent's lines.

Each fix has its test in `written/tests.rs`
(`a_right_tableau_reads_as_written`,
`faults_on_odd_input_are_said_not_panicked`), and the witness files
answer as they should through the command. Its aside, `∧L1` with an ASCII
index read into the formula, is taken too: an ASCII `1` or `2` after a
rule's name is read as its index.

### Departures and assumptions

- **No modal tableau** (departs from the design): the brief asks for
  the modal suite's labelled tableau "where the same reader serves".
  The reader would serve (a prefix `w:` before the sign), but the check
  would not: the modal suite's evidence is a derivation in the labelled
  two-sided calculus G3K with relational atoms, so a prefixed tableau
  needs its own table of rules (`T□` per accessible world, `F□` with a
  new world, the frame conditions as rules on the relation) and a
  conversion into that calculus's derivation, a second matcher of the
  size of this one. Left for a later session (below).
- **Signed only**: unsigned trees are not read (above).
- **The rule file `tableau` gained four rules** (`T↔′`, `F↔′`, `F→₁`,
  `F→₂`): the textbooks expand an equivalence by the values of its
  sides, which LK's implication rules do not give, and Smullyan's rule A
  takes one result of an α alone, which the file allowed for `∧` and
  `∨` but not for `→`; without them a textbook tableau would be refused.
  Nothing that read or wrote a derivation in the file changes
  (`from_tableau` never writes them); the rule-file test counts 25
  rules.
- **The rule file is passed to `review`**: the library has `serde_json`
  only under `protocol`, so it cannot read `rules::TABLEAU` itself; the
  command and the bindings read it (two lines, the doc says how).

### What the next sessions must know

- **68c (the page's editor)**: call `ordinary_tableau_review(text,
  sequent, settings)`; underline `text_line` (from 1) and show
  `message`; `standing` `wrong` and `unfinished` are a student's to fix,
  `closed` and `open` are right and finished (with `countermodel` for
  `open`, 68b's editor can open it); a thrown failure of code `parse`
  has `details.line`, `details.column` and `span_utf16` for a caret and
  `details.hint` for the layout's mistakes. The text a student types is
  the list form: `1 T p ∨ q`, `- 5 T p 1` with two spaces of indent
  under it, `× 4, 5`, `open`. The tableau `ordinary_tableau` returns can
  be offered as a model answer through its text (`Tableau`'s display),
  which reads back.
- **68b**: an open tableau's `countermodel` is the ordinary suite's
  one-world model, `{"sequent", "model"}`, which the model editor opens
  as it opens any countermodel.
- **Later**: unsigned trees (a rule file and a `shapes` table of their
  own, with `¬`-rules), prefixed modal tableaux (above), first-order
  tableaux with γ and δ rules (D17 above).

## Session 68b: the model editor

Opus 5.5 at `high` (the 66a to 66c worker, its conversation kept), from
the brief `/tmp/linlog-handoff/start-68b.md`, on 2026-11-16, cores 7 to
9: deduxit in `../deduxit-wt-step-58a-610ed3` on `main@origin`
(`ylwqwvpw`), the client in its workspace `step68b`
(`../deduxit.github.io-68b`) on its `main@origin` (`unnzuxur`).

### What a visitor sees

Nothing new until they ask (the author's rule). The editor opens from
three places:
- a modal countermodel's "Change the model and check it", or "Check a
  model" under a valid modal sequent (the panel's closed `<details>`,
  where 65d's list editor was);
- an ordinary countermodel's figure: "Change this model and check it"
  under the stage's note, the truth table, the tableau and the
  transformations' sheets;
- "Check a model" in the calculus popover of a started ordinary
  sequent, beside "Translate into intuitionistic logic" and "Find an
  interpolant": one world, checked against the tab's sequent.

The ordinary model shows in the drawing's place (the stage's model view,
with a "Model" tab while it shows; Tree leaves it), the modal one in the
panel.

### The editor (the client)

- **A drawing to edit** (`KripkeEditor.tsx`, the data, geometry and
  words in `src/lib/kripke.ts`): each world a box with its number and
  its atoms in the math font, laid out as deduxit draws a model (modal:
  rows top to bottom from the model's world; ordinary: bottom to top by
  height), the arrows drawn between boxes, bent where both directions
  exist, a loop on the box's right.
- **The pointer**: a drag from a world onto another adds or takes away
  that arrow; back onto the world after leaving it, a loop; onto empty
  space, the world moves there. The drag shows what it will do: the
  arrow, or the box's place. A press selects.
- **The keyboard**: every world is a focusable button whose name says
  its atoms, arrows and marks; Enter or Space selects it, the arrow keys
  move it. The selected world's form has every edit as a control: its
  atoms, its arrows as checkboxes ("World 0 sees world 1", "World 0 is
  below world 1"), ⊥ in minimal logic, its removal. Each edit is said
  in a live line. So each drag has a control beside it (WCAG 2.5.7).
- **The check is deduxit's**: `modal_evaluate` (65's checker: the frame
  condition named with its worlds, or the failing world with each
  formula's value and its witness) and `ordinary_model` (55's: a model
  that is none of the logic is "No model of intuitionistic logic" with
  deduxit's reason, "World 1 above world 0 does not force all it
  forces."). The editor sends the model as drawn (`kripkeJson`: the
  arrows as written, ⊥ only in minimal logic), and every change takes
  the last check and its marks away.
- **The findings on the drawing**, each in colour and a pattern or a
  word: the failing world a thick outline and "fails here", a witness a
  dashed outline and "witness", a frame condition's worlds a dashed
  outline, and the arrow the condition asks for drawn dashed
  (`missingArrow`: `x R x` for reflexivity, the arrow back for
  symmetry, `x R z` for transitivity, `y R z` for euclideanness, none
  for seriality, where the hint says to drag an arrow from the world).
- **The bindings**: none changed. `ordinary_model` and `ordinary_read`
  existed; the page's worker had not listed them. The page no
  longer calls `modal_model_drawing`: the editor's drawing carries the
  failing world's marks (below, a departure).
- **The passes**: `udl-barrier-anticipator` (the drag's control
  alternative, marks never by colour alone, names for screen readers,
  `touch-action: none` on the canvas, the boxes' sizes), and
  `cognitive-load-analyser` and `ai-feedback-design-principles` on the
  text: the ordinary view no longer repeats what an arrow means right
  after the countermodel's caption said it, and the frame fault's hint
  names the system and the action ("The dashed arrow is the one T asks
  for: drag it in, or change the arrows around it.").
- **axe and WCAG 2.5.3**: a world's words are drawn in a layer of their
  own (`aria-hidden`, no pointer events); drawn inside the button they
  failed axe's `label-content-name-mismatch`, `aria-hidden` or not.
- **States** (`tests/states.ts`, the 58b rule): the modal walk adds the
  dashed arrow to "modal frame" and "modal editor, a world added"; the
  ordinary walk adds "ordinary model editor", "ordinary model check" and
  "ordinary model refused".

### Checks

- **deduxit**: unchanged but for this report, so no gate and no flake
  check of its own; the module's bytes are those of `main@origin`
  (under 3 750 000 at gzip).
- **The client**, against its pinned deduxit (the calls it uses are in
  it): lint (biome) and `astro check` clean; Vitest 691 passed
  (`kripke.test.ts` 14, among them the dashed arrow checked against
  deduxit's frame check for each condition); Playwright
  `model-editor.spec.ts` 8 (6, and 2 for the reviewer's findings),
  `modal.spec.ts` 8, the walks under axe in
  both schemes at both widths and the target sizes (`accessibility`,
  `targets`): 120 passed, and `ordinary`, `methods`, `transforms`
  passed; `nix flake check` (every check, Vitest 691, the type check,
  the whole e2e suite): passed at `vquzrxyw` (321) and again after the
  reviewer's fixes at `somyrllz` (323).

### The reviewer's findings

`fresh-reviewer` on Fable 5.2, over both client changes, with the page
driven by Playwright on the dev server: the frame conditions right on
nine models (KD, T, KB, K4 twice, K5 twice, S5, KD45: the sentence, the
marked worlds and the dashed arrow), what is sent equal to what is drawn
after every kind of edit, ⊥ sent in minimal logic alone, the keyboard
path whole, and nothing new for a visitor who does not ask. Five
findings, all fixed in `somyrllz` and tested in `model-editor.spec.ts`
but the second:
1. **Medium: the ordinary view kept the world checked at after that
   world was removed**, so the check went to a world that was gone
   while the chooser said "every world". A removal now renumbers it with
   the worlds, or sets every world when it is that one (both suites).
2. **Medium: a modal check's answer in flight was kept when the world
   checked at changed** (`modalCheck` compared the model alone). It
   compares the world too. Not tested: the window is the worker's round
   trip.
3. **Low: adding or removing a world laid every world out again**, where
   the comment and the rules said the places are kept: the layout effect
   ran between the places' change and the model's. Both now change in
   one `batch`.
4. **Low: in ordinary logic a dragged loop was drawn and sent, but
   neither the world's name nor a control showed it.** Ordinary logic
   draws no loop now (every world is above itself already, and deduxit
   refuses the loop as arrows that go round); a loop in a model given is
   said in the name and has its checkbox.
5. **Low: removing the second-to-last world dropped the focus to the
   page**, its button gone with it. The focus goes to the world selected
   next.

### Commits

deduxit: this report's. The client: `yzysoplp` (the drawn editor and
the modal panel's use of it), `vquzrxyw` (the ordinary model view,
Check a model, the states, the spec, the rules), `somyrllz` (the
reviewer's findings), on its `main@origin` `unnzuxur`.

### Departures

- **The failing world is marked on the editor's drawing**, where 65d
  drew the visitor's model a second time by `modal_model_drawing`: two
  drawings of one model on one screen split the visitor's attention.
  (departs from the design: 65d's page)
- **"Check a model" for ordinary logic is in the calculus popover**,
  among the started sequent's other asked-for actions; the More menu is
  the phone's alone. (departs from the design)
- **The places of the worlds are not kept**: the link and the shelf keep
  the modal model as before (the proof's work), the places are laid out
  again on the next visit; the ordinary view's model is the view's
  alone.
- **The editor has no undo of its own**: each edit is one control's
  change, and the drag's preview shows it before it happens.

### What the next sessions must know (68c)

- `ui.modelSeed` (`{model, sequent, logic, found}`) with `ui.tab =
  "model"` opens any ordinary model against any sequent; a pack's
  exercise "find a model where …" can set it. A link carries no model
  yet: a pack that hands one out needs the model's JSON (level 13) in
  its form.
- `KripkeEditor` takes the model, the reading (`modal`,
  `intuitionistic`, `minimal`, `classical`), the marks and the check as
  props, so a check view can show a student's model read-only by
  passing no `onChange` work (a prop to add).
- Print: the canvas is an SVG in the page's colours; its marks keep
  their words and patterns in black and white.

## Session 68c: packs, the check view, the embed and print

Opus 5.5 at `high` (the 68b worker, its conversation kept), from the
brief `/tmp/linlog-handoff/start-68c.md`, on 2026-11-16, cores 7 to 9:
deduxit in `../deduxit-wt-step-58a-610ed3` on `main@origin`
(`ypwvprqu`), for this report only; the client in its workspace
`step68b` (`../deduxit.github.io-68b`) on its `main@origin`
(`somyrllz`).

### What a teacher and a student see

- **A teacher** opens the problems as tabs (typed, or from Practice's
  stores), then Export, "Course pack": a title, the problems (every
  started tab of linear, ordinary or modal logic, each unticked to leave
  it out), "The search may close goals and give hints" (off by default),
  the pack's code, "Copy the link", "Copy the embed code" and "Print the
  problems" (the handout: the title, each problem and its calculus, the
  link and the code).
- **A student** who opens the link gets a tab per problem in its
  calculus, and a bar over the stage: the title, a button per problem
  (a tick on each done), "N of M done" and "Hand in". With the search
  barred, Close goal, Close all and Hint are greyed with the reason as
  their tip; a key or the hint ladder says the reason on the tab's
  banner; the calculus popover says "This is problem N of the pack: its
  sequent and calculus are fixed." in place of its switches. A modal
  problem with the search barred is not decided: its model editor is
  open and empty, and the model the student makes and checks is the
  answer. The answers are kept in the browser under the pack's code, so
  the pack's link opens them again.
- **The hand-in** is a link (`#v=4&h=…`, up to 16 000 characters) or a
  file (`hand-in-<code>.json`), with the pack's code. The page sends
  nothing: the student sends the link or the file.
- **The check view** opens from a hand-in's link, its file in Import or
  dropped on the stage, and leaves the open tabs alone. Per problem: the
  sequent and calculus, deduxit's finding ("Proved: deduxit read every
  step again: the proof is finished and checked.", "Not finished: 1 goal
  is still open.", "Another sequent", "Another calculus", "Refused",
  "Countermodel", "No countermodel", "No model of S4: …", "No answer"),
  and "Open the answer" on a tab of its own. Its head shows the pack's
  code, which the teacher compares with the code their sheet showed.
- **The embed**: `&e=1` on any link (the sheet's embed code is an
  `<iframe>` of the pack's link with it) shows the proof and its actions
  without the site's places (Practice, Learn, the tour, the keys,
  About), the restore bar or a new tab, and an "Open in deduxit" link.
- **Print**: the light scheme in either theme, no band, menu or stage
  controls; with the pack's sheet open, the handout alone, over as many
  pages as it takes.
- **A tableau written by hand** (68a's call, on `main` during this
  session): "Check a tableau" in the calculus popover of a started
  classical sequent opens a field in the drawing's place ("Your
  tableau"), begun with the sequent's own signed lines (deduxit's
  tableau is not shown there), a sentence on the list form and a closed
  example. "Check the tableau" says Closed, Open, Unfinished, Wrong
  step (with "Line N:" and that line selected in the field) or Not read
  (the parse failure's place selected, its hint said); an open one's
  countermodel opens in 68b's model view.

### How it holds

- **The formats** (`src/lib/pack.ts`): a pack `{"pack": 1, "title",
  "search", "problems": [{"suite", "parameter", "sequent"}]}`, a hand-in
  `{"handin": 1, "pack", "answers"}` with an answer per problem (a
  session, a model and its world, or null). Read key by key within
  bounds: no key of its own, a calculus the registry knows, at most 30
  problems, sequents of at most 1 000 characters; the links unpack at
  most `UNPACKED_LIMIT` bytes as every link does. A first draft read
  them with zod, whose probe of `eval` the page's CSP blocked (the e2e
  fixture caught it); a reader by hand also keeps zod off the island's
  first load.
- **The limits are the prover's**: `Limits` (`search`, `fixed`) is a
  signal of the prover, set by the open pack; `close`, `closeAll`,
  `fill`, `hint` (and the ladder in `App.tsx`), `submit` and
  `setCalculus` ask it before they act, so no key or path does more than
  the greyed buttons.
- **The check trusts nothing** (`checkAnswer`): a session is read again
  by its suite's call (deduxit's readers replay and check every step of
  a session they read), its mode or logic held to the problem's, its
  sequent's text (`read_as`, `ordinary_read` on the session's `sequent`)
  to the problem's; a finished linear proof is checked once more by
  `proof`. A model is checked by `modal_evaluate` against the problem.
  The pack inside a hand-in is the student's too, hence its code: 48
  bits of SHA-256 of the pack's JSON with its keys in one order.
- **The passes**: `udl-barrier-anticipator` (every control named, the
  done tick beside the colour, the problem buttons 44 pixels on a
  phone), `cognitive-load-analyser` (one bar of four parts for the
  student, the teacher's sheet one screen, nothing for a visitor without
  a pack), `ai-feedback-design-principles` (each refusal says what is
  fixed and what to do instead: "A new tab takes another sequent.").

### Checks

- **deduxit**: unchanged but for this report; no gate and no flake
  check of its own.
- **The client** at `nnvupxxs`, against its pinned deduxit: lint and
  `astro check` clean, Vitest 698, `nix flake check` passed (the whole
  e2e suite, 335).
- **The client** at `vqzymsrw` (the review's fixes and the tableau
  editor), against deduxit's `main@origin` (`sspmkqrl`, 68a's call) by
  `--override-input deduxit git+file:///…/deduxit-wt-step-58a-610ed3`:
  lint and `astro check` clean; Vitest 703 (`pack.test.ts` 7,
  `written.test.ts` 5 against deduxit's review); Playwright
  `packs.spec.ts` 9, `written.spec.ts` 2, and the walks under axe in
  both schemes at both widths and the target sizes at 24 and 44 pixels,
  the `packing` walk and the tableau's two states among them: 123
  passed; `nix flake check` with the same override: passed (every
  check, the whole e2e suite 340).

### The reviewer's findings

`fresh-reviewer` on Fable 5.2, over `qrkyzmsr` and `nnvupxxs`, with
the page driven by Playwright: the hand-in check held against 29 forged
answers (another sequent, mode or logic, a refused session, a model of
the wrong system, a model for a proof problem and the reverse, a changed
pack's code), no request carried a pack, a hand-in or an answer (every
request a GET to the page's own server), nothing showed for a visitor
without a pack, and the keys, a pasted session and an import kept to the
limits. Seven findings, all fixed in `ozppvoup` (the stage's and the
stylesheet's part in `vqzymsrw`) and tested in `packs.spec.ts` but the
last:
1. **High: a barred classical problem still showed deduxit's table and
   closed tableau.** The Table and Tableau tabs are not offered where
   the search is barred.
2. **High: "Prove it step by step" turned a modal problem into a
   calculus of its own**, which lifted the pack's limits. `modalSteps`,
   `customFrom` and `showModalStatement` ask the pack first.
3. **Medium: a valid modal problem counted as done** with no answer to
   hand in. Done is a finished proof or a model where the sequent fails.
4. **Medium: both bands printed**: the screen's rules outweighed the
   print rule. The print rules weigh the same and come later.
5. **Medium: in the dark theme nothing printed light.** Print sets the
   light scheme over `data-theme`.
6. **Medium: a long handout was cut at one page**, losing the link and
   the code (a fixed box). It is drawn at the body's end and flows.
7. **Low: a barred modal tab said "Deciding" for good.** It says
   "Undecided: find a model" until its model is checked. Not tested
   apart: the walk shows it.
8. **A question**: with "Apply invertible rules by themselves" on, a
   barred problem can finish with no step of the student's. That is no
   search, so the limit's words hold; whether "by hand" bars it is the
   author's to say (below).

The tableau editor (`vqzymsrw`) came after the review: 68a landed
meanwhile. It is covered by its unit tests against deduxit, its spec
and the walk, not by the reviewer.

### Commits

deduxit: this report's. The client: `qrkyzmsr` (the formats and their
links), `nnvupxxs` (the packs on the page, the hand-in, the check view,
the embed and print, the walk and the spec, the rules), `ozppvoup` (the
reviewer's findings), `vqzymsrw` (the tableau written by hand, and the
stage's and stylesheet's share of the findings), on its `main@origin`
`somyrllz`. `vqzymsrw` calls `ordinary_tableau_review`, which the
client's pinned deduxit lacks: the client's lock moves to deduxit's
`main` after this chain is pushed (`nix flake update deduxit`, a
"flake.lock: Update" change), then its `nix flake check`. `ozppvoup`
alone fails its own test of the Table tab, whose fix is in `vqzymsrw`.

### Departures

- **A pack's problems are linear, ordinary (sequent calculus) and modal
  ones.** Natural deduction, the tableau and a calculus of one's own are
  sessions of the rules suite, which the check view does not read yet;
  "the methods allowed" is the search's switch. (departs from the
  design)
- **The tableau editor is not a pack's problem**: a pack's problem is a
  sequent to prove; a tableau to write (a `method` in the problem, read
  by the check view through the review) is for the release's planning.
- **The limits are on the pack's tabs.** A student may prove the same
  sequent in a tab of their own with the search, and the check view
  cannot tell a proof typed by hand from one copied step by step: the
  pack bars a route, it does not prove who did the work.
- **A modal problem's answer is a model.** A valid modal sequent has no
  answer in a pack: its labelled proof is a calculus of its own, which a
  pack's tab keeps out (the review's second finding), so a pack's modal
  problems are ones to refute.
- **The embed has no option of its own** (read-only, Close hidden):
  `&e=1` is one stripped view; a pack's limits apply in it as anywhere.

### What the next sessions must know (the release of 0.6.0)

- **A pack's tableau problem**: the editor exists; a pack's problem
  could carry `"method": "tableau"`, its answer the written text, which
  the check view reviews with `ordinary_tableau_review`.
- **The author's question**: should a pack that bars the search bar
  "Apply invertible rules by themselves" too?
- **The client's lock**: `vqzymsrw` needs deduxit's `main` of 68a; move
  the client's input after the push, before its next `nix flake check`.
- **The check view reads the rules suite next**: natural deduction and
  tableau sessions (`rules_check`, the session's calculus held to the
  problem's), which makes their packs possible.
- **The formats are versioned** (`pack: 1`, `handin: 1`, links `v=4`):
  a change of either adds a version and reads the old one.
- **Before 0.6.0**: the manual and the page's About say what a pack is
  and that nothing is sent; the client's README names `#v=4` beside the
  other link versions.

## Session 68d

Opus 5.5 at `high` (the 68b and 68c worker, its conversation kept),
from the brief `/tmp/linlog-handoff/start-68d.md`, on 2026-11-16, cores
7 to 9: the client in its workspace `step68b` on `tllwnulk` (68c's chain
with its lock update), deduxit for this report.

- **The change** (the author's decision of 2026-11-16 on 68c's
  question): where a course pack bars the search, "Apply invertible
  rules by themselves" has no effect on the pack's tabs. The prover's
  `invertibleOn(id)` is the setting less the pack's bar; every call's
  settings (`json`), the undo's grouping of a step with its invertible
  steps and the setting's own run over the open goals read it. The
  setting stays the visitor's elsewhere, and the Settings sheet says
  "This pack asks for every step by hand." while a barred tab shows.
  The linear suite is the one whose sessions apply the rules today; the
  ordinary one has no such setting yet. The check view takes a hand-in
  as before: a session that holds invertible steps is still a proof.
- **The test**: `packs.spec.ts`, "with the search barred, the invertible
  rules wait for the student too": the setting turned on in a pack of
  two linear problems leaves `⊢ A ⊸ A` open and the bar at 0 of 2, and a
  tab outside the pack is finished by the rules alone.
- **Checks** (the client): lint and `astro check` clean, Vitest 703,
  `packs.spec.ts` 10 passed, `nix flake check` passed (every check,
  the whole e2e suite 341).
- **Commits**: the client `rluokntt` on `tllwnulk`; deduxit this
  report's.
