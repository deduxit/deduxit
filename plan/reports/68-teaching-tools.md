# Step 68: tools for teachers

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
