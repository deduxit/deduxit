# Step 56: classical tableaux and truth tables

## Session 56a: the library and the command

Opus 5.5 at `high`, the reused worker of 55a, 55b and 54w-e4b (same
conversation), in the jj workspace `../deduxit-wt-step-54s-c1-161790`,
from the brief `start-56a.md` and `plan/56-tableaux-truth-tables.md`
("Fixed now"), on 2026-11-15, cores 4 to 6.

### Outcome

Classical logic is decided by its truth table and by a signed analytic
tableau, as two methods of the ordinary suite, in the library, the
command and the bindings:

- `ordinary::Method::Table` (`--method table`): every row, falsifying
  rows marked, the first as a checked one-world countermodel; bounded by
  the new `Limits::table_atoms`.
- `ordinary::Method::Tableau` (`--method tableau`, feature
  `interactive`): a signed tableau built step by step in the goal state,
  a closed one the checked derivation of LK it is, an open branch a
  checked countermodel; `Tableau::of` reads any derivation of LK as a
  tableau.
- Both outcomes carry the table or the tableau in `search`, written at
  the new wire level 14; the bindings' `ordinary_table` and
  `ordinary_tableau` answer them for the page (56b).

The commits on `main@origin` (`pwwwlmqk`): `vwukrxys` (the library),
`lsvsmlsp` (the command), `mwvppnvt` (the behaviour lock), `xxzumoyk`
(the bindings), `sxpqpnyr` (level 14's stability file), `lmqlmozx` (the
manual, the guide, the changelog, the rules), `pzmqzprr` (the review's
fixes) and this report's.

### The tableau's form, and why

- **Signed** (Smullyan's `T` and `F`), not unsigned. A two-sided
  sequent of LK is a branch exactly: its hypotheses are the `T`
  formulas, the formulas right of `⊢` the `F` ones. So a signed tableau
  is LK upside down rule for rule, which the design asks, and an open
  branch reads as an assignment directly. The Open Logic Project uses
  signed tableaux for that reason (the research note's [3]); an
  unsigned one would be the one-sided calculus, which the ordinary suite
  does not have.
- **The order of expansion is Smullyan's systematic one**: a closing
  rule where one applies (a formula true and false, `⊥` true, `⊤`
  false), else the first formula with a one-premise rule (α: `T ∧`,
  `T ¬`, `T ↔`, `F ∨`, `F →`, `F ¬`, the hypotheses first), else the
  first with two (β: `T ∨`, `T →`, `F ∧`, `F ↔`). α before β keeps the
  tree small; closing first ends a branch as soon as it can. `⊤` true and
  `⊥` false stay as lines, as atoms do.
- **An expansion writes its results even where the branch has them**:
  a node's lines are what the premise has beyond the conclusion,
  counted as a multiset, so `T a → b` on a branch with `F a` and `T b`
  still writes `F a | T b`, as a textbook does; a contraction or a
  weakening writes nothing.
- **Built in the goal state, checked as LK** (departs from the design
  in its letter: "a view of the LK read-back"). The method builds its
  own derivation of LK by applying the tableau's rules in
  `ordinary::Interactive`, which checks every inference by the
  checker's rule before the state changes; a closed state's derivation
  is then checked whole (`finished`) and is the outcome's evidence, and
  the tableau is `Tableau::of` of that derivation. So a closed tableau is
  a checked derivation of LK read upside down, and nothing is trusted
  beyond the existing checker and the countermodel check. Why not the
  decided outcome's read-back: a "not valid" has no derivation to view;
  the classical default's "valid" is often the SAT solver's clausal
  proof, which has none either; and the embedding's read-back follows
  the linear search's order (β before α, weakenings mid-tree), which is
  not a tableau a course draws. `Tableau::of` stays the view of any
  derivation of LK, the read-back's included.
- **An open branch** is a goal with nothing left to expand and nothing
  closing it. Its true atoms make every formula on it true or false as
  signed (the rules are invertible), so they falsify the sequent: that
  one-world model is checked by `Countermodel::found` against the
  sequent before it is the outcome's countermodel.

### The truth table and its bound

- A row per assignment, from every atom true to every atom false, the
  first atom slowest (the textbooks' order), with the value of every
  formula of the sequent's arena; a row falsifies the sequent where
  every hypothesis is true and every formula right of `⊢` false.
- **The bound** (D16): `Limits::table_atoms`, by default
  `DEFAULT_TABLE_ATOMS` = 10, so 1024 rows: about what a reader scans,
  and a table that answers at once. Past it, `Refusal::Atoms` (code
  `too_many_atoms`, kind limit, setting `limits.table_atoms`) before
  anything is made; the command names `--table-atoms N|none`. The memory
  bound counts the table's bits, the work bound its rows, and cells past
  a `usize` are `Refusal::Index`.
- A "valid" table's evidence is `Evidence::Table`, which has no
  derivation (`Error::NoDerivation` now names what the sequent is valid
  by in its field `evidence`).

### The command's and the bindings' forms

- **Methods, not a view**: the brief left the choice to the command's
  shape. `--method` is already how a tab and the command choose how a
  sequent is decided, and the table and the tableau decide (they do not
  only draw another method's answer); the page's registry (D29) names
  methods too. So `--method table|tableau` on `prove --logic classical`
  (settings `ordinary.method`), refused for the other logics with
  `Error::Logic`, which now names the method.
- **Text**: the verdict line (`valid (classical logic by its truth
  table): no row falsifies it`, `not valid (…): 1 of its 4 rows
  falsifies it, a checked countermodel`, `valid (classical logic by a
  tableau): every branch closes, a checked derivation of LK`, `not valid
  (…): an open branch gives a checked countermodel`), then the table
  (a column per atom, `│`, the hypotheses, `⊢`, the formulas right of
  it, `×` after a falsifying row) or the tableau (numbered lines with
  their sign, the rule and the line they expand, `├─`/`└─` for a split,
  `× i, j`, `open`), then the countermodel. `--stats` prints `rows` and
  `falsifying rows`, or `lines` and `branches`. The drawn formats draw a
  closed tableau's derivation; a table has none (a left-out line).
- **JSON** (wire level 14): the ordinary outcome, its `search`
  `{"method": "table", "table": {"sequent", "rows": ["TFT…"],
  "falsifying"}}` or `{"method": "tableau", "tableau": {"sequent",
  "nodes"}}`; level 14 because a reader of level 12 refuses an unknown
  method by name (it would not misread it, but cannot read it). The
  other outcomes stay at 12. A table document is read by computing the
  table again and comparing; a tableau by its shape and indices.
- **`check`** reads both outcomes: a valid table is `valid truth table
  of … : none of its N rows falsifies it` (the reader recomputed it), a
  valid tableau its derivation checked, a not valid one its
  countermodel checked.
- **The bindings**: `ordinary_table(sequent, settings)` and
  `ordinary_tableau(sequent, settings)` answer the outcome as JSON;
  `about` lists them under the ordinary suite's `methods` (`method`,
  `call`, `logics`) and `limits.table_atoms` among its settings.

### The checks

- Clippy on the workspace with `--deny warnings`, and the tests of
  every crate (the core's, the command's with the lock and README's,
  the manual's and the guide's examples, the bindings' natively): pass,
  cores 4 to 6, after the review's fixes.
- The rustdoc with `-D warnings`, every feature, public and private
  items: clean.
- The gate (`nix develop -c gate`, cores 4 to 6): passed at `lmqlmozx`;
  `pzmqzprr` after it, with clippy and every crate's tests rerun.
- Every family at 5 s: 195 rows, no `MISMATCH`.
- The ILTP agreement tests (`--release --test iltp -- --ignored`): 3
  passed, 789 s.
- `nix flake check` (under the machine lock, `--max-jobs 4 --cores 8`)
  at the head rebased onto `main@origin` `xyyzyqnz`: passed, the
  bindings' tests under Node (`web-wasm-tests`) and both `cargo hack`
  runs among its checks; this line is the only change after it.

### The reviewer

`fresh-reviewer` once (Fable 5.2, medium) on the six changes then
made. **The brief's criterion holds**: `Method::Tableau` answers valid
only through `Interactive::finished`, which checks the derivation
whole, and not valid only with a model `Countermodel::found` checked
against the sequent; the closed tableau is `Tableau::of` of that checked
derivation; the truth table's verdict, row order, bound and indices are
right; no forged tableau document makes `line`, `write_text` or
`parent_rule` panic. Its six findings, all fixed in `pzmqzprr`:

1. **A forged table outcome checked valid**: the outcome's reader took
   the verdict word over a table with a falsifying row. It now refuses a
   verdict its table or tableau does not bear out (`valid` with a
   falsifying row or an open branch, `not_valid` without); the stability
   test refuses a flipped tableau outcome.
2. **"a checked countermodel" without one**: the verdict line said so
   where `Countermodel::found` dropped a model too large to check. It
   now tests the countermodel, as LJT's and the solver's lines do.
3. **The tableau ignored `--memory-limit`** (the pigeonhole principle
   for 8 pigeons took 4.2 GB under 16 MiB): every goal a step opens is
   now charged (`GOAL_BYTES` and 8 bytes a formula), refused as
   `Refusal::Memory`; the same input now stops at the bound.
4. **A formula written twice was cited twice**: `Tableau::of` found a
   principal's line by searching the branch for an equal formula. It now
   ties each position of a sequent to its line, through the structural
   rules too; and the open branch's countermodel lists each atom once.
5. **`Tableau::of` panicked** on a JSON derivation with a structural
   rule and no premise: now `Error::ReadBack`.
6. **The table's memory test left out the falsifying list** (8 bytes a
   row): counted now.

Its two smaller points: `End::Open`'s doc spoke of the branch where it
means the formulas left to expand (fixed); and `check` of a table made
with `--table-atoms` above 10 needs the bound in a settings file, since
`check` has no flag for it (open, below).

### Departures from the design

- The tableau is built by the method in the goal state and checked as
  LK, not read from the decided outcome's read-back (departs from the
  design in its letter): why above. A closed tableau is still a checked
  derivation of LK upside down, and `Tableau::of` is the view of any
  derivation of LK.
- `Error::Logic` gained a field `method` and `Error::NoDerivation` a
  field `evidence` (both unreleased variants' shapes; the changelog's
  breaking list says so).

### What is open

- `check` reads a table's outcome within its own `limits.table_atoms`
  (10 by default), with no flag to raise it; a table made with
  `--table-atoms 11` is checked through `--settings` with the key. A
  `--table-atoms` on `check` is a small addition if the author wants it.
- The wire error's message reads "not a ordinary outcome" (the form's
  name after "a"), a slip outside this step's files.

### What the next sessions must know

- **For 56b (the page)**:
  - Call `ordinary_table(sequent, settings)` and
    `ordinary_tableau(sequent, settings)`; `about().suites[1].methods`
    lists them with the logics they decide (classical only). The
    answer is the ordinary outcome: `verdict`, `search.table` or
    `search.tableau`, `countermodel` for `not_valid` (55b's form, one
    world), `derivation` for a valid tableau.
  - A table: `search.table.sequent` is the ordinary sequent's form;
    `rows[r]` has a `T` or `F` per formula of that form, in its order
    (operands first), so a column per hypothesis and conclusion is
    `rows[r][id]` with `id` from `sequent.left` and `sequent.right`, and
    an atom's column the atom's formula; `falsifying` lists the rows to
    mark. Past `limits.table_atoms` the call fails with code
    `too_many_atoms`.
  - A tableau: `nodes` in preorder, node 0 the root (the sequent's
    lines); each node's `lines` are `[sign, formula]`, numbered across
    the tableau from 0 in node order (the text adds 1); a node with
    `children` has `expands: [line, rule]` (the line its children
    expand, the rule's name as LK's), a leaf has `end`: `closed`
    (`true_line`, `false_line`), `falsity` or `truth` (`line`), `open`
    or `unexpanded`. The rule names are LK's (`∧L`, `→R`), which the
    page's rule hints already cover.
  - The tableau's tree is the page's to lay out (the text form is
    `Tableau::write_text`); a drawing in the library is not in this
    step.
- **For later steps**: intuitionistic tableaux and many-valued tables
  are not here (`plan/later.md`). A tableau checker for a student's own
  tree (the research note's tableaux-2) would read the JSON form and
  check each expansion against `next_step`'s rules; the reader today
  checks only the shape.

## Session 56b: the page

Opus 5.5 at `high`, the reused worker of 55a to 56a (same conversation),
in the client's jj workspace `../deduxit.github.io-56b` (on its
`main@origin` `vtvywvym`, deduxit pinned at `oyrxnpoz`) and this
workspace for the report, from the brief `start-56b.md`, on 2026-11-15,
cores 13 to 15.

### Outcome

A started proof of classical ordinary logic offers two more tabs beside
Tree and Net, **Table** and **Tableau**. Each shows, in the drawing's
place, the sequent decided by that method (the bindings'
`ordinary_table` and `ordinary_tableau`): the verdict in one sentence that
names its reason, then the truth table or the tableau, then for a sequent
that is not valid its countermodel (55d's drawing). The proof stays as it
was; Tree brings it back. Nothing else changes for a visitor who does not
press them.

The client's commits, on its `main@origin` `vtvywvym`: `ylwvmwwz` (each
formula of an ordinary sequent's JSON written on its own), `lkxxqzrs`
(reading the table and the tableau from deduxit: `src/lib/methods.ts`, its
unit tests, the two calls), `pkpkmnou` (the countermodel's figure in a
module of its own), `trnxxwzo` (the two views, the tabs, the styles, the
client's `web.md`), `kltrzkxo` (the interaction tests, the `methods` walk of
`tests/states.ts`, the axe and target specs), `tsxslurw` (the review's
fixes).

### How the views were designed

- **The frontend-design plugin**: the page's visual language is fixed by
  its tokens (palette, Figtree for words, Euler Math for formulas), so the
  views add no colour and no font: the verdict's keyword in the ok or bad
  token beside its word, `F` values in the dim ink, a falsifying row in the
  bad wash with `×` and its word. The one bold element is the tableau's
  branch connectors: a bar the two branches hang from, side by side on a
  wide screen, stacked with a rule on a phone. No cards, no eyebrows, no
  motion.
- **The cognitive-load analysis** (the education skills'
  `cognitive-load-analyser`), applied:
  - one sentence above each view says the verdict and why, naming the
    values ("With a false, b true, every formula left of ⊢ is true and
    every formula right of it false: row 3"): the student does not search
    the table and translate a row (split attention);
  - the table has only the columns the verdict needs, the atoms and the
    sequent's own formulas, not every subformula (redundancy), with ⊢
    standing between the formulas left of it and right of it;
  - a falsifying row is marked by a sign and a word, not by colour alone;
    the first one's mark ("× the countermodel") takes the student to the
    countermodel under the table;
  - an open branch of the tableau says its assignment where it ends ("open:
    a false, b true"); a closed one shows `× 4, 5` and says "closed: b is
    true at line 4 and false at line 5" to a screen reader;
  - one view at a time, and only on request (not overwhelming).
- **A table past the bound** (deduxit's `limits.table_atoms`, 10) says so
  in the page's words ("This sequent has 11 atoms, so its truth table has
  2048 rows: more than the page draws at once. The tableau decides it
  without the rows.") and offers "Draw all 2048 rows" up to 14 atoms, by
  sending the bound raised.

### Views, not a method of the tab (departs from the design)

The brief, after D29, has the methods chosen in the tab beside the
default, and the client's rules foresaw a second method as a field of the
tab's calculus, the link's `k` and the shelf. The session made them views
of the stage instead, beside Tree and Net. A truth table and a tableau
decide the sequent at once and hold no steps, so a tab of their own would
hold nothing to do, and switching the calculus restarts a proof; as views
they keep the proof, need no change of the link, the shelf, the tour or
the restore bar, and cost a visitor who does not press them nothing. The
registry's `methods` and the link's `k` stay free for a method with steps
(a student's own tableau, the research note's tableaux-2).

### The checks

- The client's unit tests (Vitest, `tests`): 605 passed, with the new
  `src/lib/methods.test.ts` against deduxit's real module.
- Lint and type-check (`lint`: Biome, `astro check`): clean.
- The views' e2e tests (`tests/methods.spec.ts`, 11), the `methods` walk
  under axe (light and dark, desktop and phone: no violation) and the
  target sizes (24 pixels on a desktop, 44 on a phone): passed.
- The client's `nix flake check` (under the machine lock) at `tsxslurw`:
  passed, its e2e suite 237 tests, its unit suite 605.
- In a browser (the built-in pane, the dev server on cores 13 to 15):
  both views at a desktop's and a phone's width; the 11-atom table drawn
  in 100 ms (27 s before the model was memoized, below).

### The reviewer

`fresh-reviewer` once (Fable 5.2, medium), which drove the page with a
Playwright script and read its screenshots. It accepted the views as
meeting D29's intent, to be marked as a departure (done above), and
confirmed the verdict sentences, the marked row, the countermodel's
agreement with the open branch, the refusal and the phone layout. Four
findings, fixed in `tsxslurw`:

1. **A key changed the hidden proof**: `C` behind the table closed every
   goal unseen. Now any change of the proof while a view shows brings the
   tree back, so the visitor sees what changed (keys and the bottom band
   alike); a test presses `C` behind the table.
2. **The valid tableau's sentence was false where ⊥ true or ⊤ false
   closes a branch** ("each makes one formula both true and false"): now
   "Every branch closes, so no assignment makes …"; a test on `false |- a`.
3. **The restore bar covered the view's sentence**, on a phone most of
   it: the panel starts under the bar while it shows; a test measures the
   two boxes.
4. **A stale Table or Tableau tab** stayed set for a proof that has none
   (a finished intuitionistic proof lost its "Proved" line): the stage
   goes back to Tree when the proof shown has no table; a test switches
   the logic.

Before the review the session found and fixed one more: the table's
model was built in JSX props, which Solid reads as a getter at every
use, so the table was rebuilt for every cell (27 s for 2048 rows); it is
memoized now.

### What the next sessions must know

- **The two views** are `src/islands/Methods.tsx` over `src/lib/methods.ts`
  (the forms and the models: columns and rows, nested branches with each
  end in words); the stage shows one while `ui.tab` is `table` or
  `tableau` and the proof is classical and started (`decidable`), and
  hides its camera, buttons and dock (`data-method`).
- **A prop is a getter**: a model built in JSX props is rebuilt at every
  read; the table built per cell took 27 s for 2048 rows. Build it once
  (`createMemo`), as `Methods.tsx` does.
- **For step 63 (Learn and Practice)**: the views are ready for lessons on
  truth tables and tableaux; a lesson can open a classical tab and press
  the tab. The rule names of the tableau (`∧L 1`) carry no hint yet: the
  rule hints (`RuleHint.tsx`) are tied to the derivation's drawing; a
  hint per tableau rule would join them there.
- **A student's own tableau** (checked step by step) would be a method of
  the tab with steps, the link's `k` and the registry's `methods`; deduxit's
  tableau reader checks only the shape today (56a's report).
