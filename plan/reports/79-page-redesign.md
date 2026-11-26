# Step 79: the page rebuilt to the redesign, first order on it

## Session 79a: the frame

Opus 5.5 at `high`, 2026-11-25 to 26, in the deduxit workspace
`deduxit-wt-step-79a-…` (on `ykykzyqz`, the step's prompt) and the client
workspace `step79a` (`/home/tux/Projects/own/deduxit.github.io-79a`) on the
redesign's head `ottzpzmk`, cores 4 to 9. Nothing pushed; the client's
`main` untouched.

### Outcome

The page has the guide's frame and scale, and proves sequents in it again.
The top row holds the word mark, a chip per open sequent, "+", the guide's
switch, light or dark and Settings; the rail holds the chosen sequent's
views and "+ view"; the stage keeps the drawing, with the toggles at its top
left, the tools at its top right, the zoom at its lower right and the bottom
box with its notes at its bottom. A sequent not yet entered shows the new
tab view (board D1). The LK and linear proof views work inside the frame as
before, and so do natural deduction, modal logic, lambda-terms, the truth
table, the tableaux, the model editor, translations, interpolants, normal
forms, resolution, cut elimination, course packs and the exports. The bottom
band, the stage's view tabs, the Logic popover, the Workspace panel, Learn
and Practice are gone with their tests and states; "Learn" and "Practice" are
inactive "Coming soon" buttons.

The client's commits on the redesign's chain (change ids):
- `nrynynyl` Take the interface's fonts, Ysabeau Office and Gentium Book
  Plus, from npm (the lock, the licences table and check, README, the
  layout's imports and its viewport);
- `lqzlonkr` Build the page's frame to the redesign (the scale, the frame,
  the window sizes, the keys, the new tab view, Restore, light and dark, the
  removals, every Playwright spec and state walk adapted);
- `ruoyqrpt` Describe the new frame in the client's rules and CLAUDE.md;
- `lqyuxrrm` Fix the frame's review findings;
- `qykrxzmt` Open More in the state walks only where the window shows it;
- `wxknvkvo` Close More's row on any tool's press, and measure the model
  editor's chips.

The signing hook also signed the redesign chain's earlier commits when it
signed `nrynynyl` (their change ids, `ottzpzmk` among them, are unchanged;
their commit ids are new).

### What was built, by the guide's sections

- **The scale** (`src/styles/tokens.css`): the guide's sixteen colours by
  their own names (`--color-paper` … `--color-over`, `edge` new), each with
  its light and dark value; the type (Ysabeau Office 400/500/600, Gentium Book
  Plus 700, the four interface sizes and the two headings, the line heights,
  the weights); the spacing scale (2, 4, 8, 12, 16, 24, 32, 48, in rem); the
  nested corners; 1-pixel borders, 2 for the focus ring and an open goal; the
  controls' sizes; the icons' strokes; the motion (the scheme's fade through
  the View Transitions API, 0.5 s; a panel 0.2 s; the knob 0.15 s; none under
  reduced motion). `src/styles/colours.test.ts` holds the guide's pairs:
  ink, ink2 and soft at 4.5:1 on paper, band, wash and over; edge at 3:1 on
  paper and band; paper on lapis; lapis, deep, good and bad at 4.5:1 as text;
  the net colours at 4.5:1 on paper; faint between 3:1 and 4.5:1, so that it
  never passes for text. `tokens.test.ts` holds the spacing to the guide's
  scale and every media query to the window classes.
- **The frame** (`src/islands/Frame.tsx`, `App.tsx`): the top row (word mark
  as a plain link that opens the new tab view in place; the chips a tab list,
  each named with its state, a check or a cross for proved or refuted; "+";
  "?" with `aria-pressed`; light or dark; Settings), the rail (a cell per
  view, the open goals' count from CSS, "9+" past nine, a cross for a refuted
  sequent, the net in one capsule with its proof once the proof is finished,
  the views added by "+ view"; "+ view" a dashed cell with its grouped list,
  "Prove", "Decide", "Translate", "Read off", a faint entry with its reason,
  an entry there with a check), the toggles' place, the tools (a toolbar in
  groups: Undo, Redo, Steps; Hint, Close all or Stop, the caret for the
  search's settings; the view's switches, "W C" in LK, Cut and Two-sided in
  linear logic; Export, Import; with the guide on Learn, Practice and Tour),
  the zoom group, the bottom box with Enter (the text written in signs once
  it reads), the notes above it (the box's problem with its place marked and
  the box's red edge, the banner, the verdict "Proved.", a notice with its
  Undo), the panels left of the tools (Steps, the search's settings,
  Export, Import, the sheets), the Settings page in the stage's place, the
  keys and About as dialogs, the drop frame under the top row.
- **Window sizes**: the three classes and the short one as media queries in
  the range syntax (`(width < 600px)`, `(width >= 840px)`, `(height < 480px)`);
  compact: the square mark, no light-or-dark button, the views a row under
  the top row, the tools a bar of Undo, Redo, Hint, Close all and More with
  the rest in More's rows, the bottom box above the bar, the stage ending
  above the box, every panel and the views' list a sheet; medium: a panel
  lies over the stage; expanded: the stage narrows and the drawing moves
  aside; short: the rail scrolls, the rarer tools behind More, the bottom box
  at the lower left; `pointer: coarse` and compact: every target 44 pixels,
  a field's text 16; `100dvh`, `env(safe-area-inset-bottom)`,
  `interactive-widget=resizes-content`; forced colours give every edge a
  real border. The one breakpoint of 640 and its test are replaced.
- **Keys** (`src/lib/keys.ts`, with `keys.test.ts`): the guide's table, read
  by `KeyboardEvent.key`; nothing in a field, with Alt (AltGr) or while
  composing; Settings' "Single keys" (on at first) leaves only the keys with
  Ctrl or Cmd; `?` and Ctrl+/ open the list, whose first row is the guide's
  switch; J/K and ]/[; Ctrl+Z, Ctrl+Shift+Z, Ctrl+Y, U, Shift+U; T and V and
  their Shift; A, N, /, the zoom's keys, `,`. `aria-keyshortcuts` on each
  control with a key, the key in its hover. The chips and the rail are tab
  lists, the tools a toolbar (one stop of Tab, the arrow keys move). `KEYS`
  of `BottomBand.tsx` is gone with the band.
- **The new tab view** (board D1): the four logics (ordinary first), "Open a
  file", "Restore N proofs", Learn and Practice inactive, About; the toggles
  of the logic chosen at its top left; the box; with the guide on the
  logic's examples over the box (the ordinary suite's samples of the chosen
  logic); a file dropped anywhere under the top row; deduxit's JSON pasted
  into the box opens on Enter.
- **Restore**: each visit holds a Web Lock named by its shelf key for as long
  as it is open (`prover.shelfKey()`); a plain visit (no fragment) that finds
  no other shelf key's lock held restores the last visit's proofs by itself;
  otherwise the new tab view offers "Restore N proofs". Settings' "Close
  every sequent" sets the proofs aside as a new visit does, and the notice's
  Undo restores them.
- **Light and dark**: the top row's moon or sun and Settings' Light, Dark and
  System; read before the first paint by the existing script; the scheme
  read into the island at its start.
- **The proof views**: the stage of today (its dock, popovers, choosers,
  rule hints, cut elimination) inside the frame. A Playwright test proves a
  sequent in each: `tests/frame.spec.ts` "the new tab view offers four
  logics, and a sequent entered becomes a chip" and `ordinary.spec.ts`
  (LK by hand), `tests/prover.ts` `proveCommuted` and `prover.spec.ts`
  (linear logic).
- **Gone**: the bottom band, the stage's view tabs, the Logic popover with
  its suite buttons and sheets' buttons (the sheets open from "+ view"), the
  status popover, the calculus mark and its fragment, the per-tab close, the
  restore bar, More's sheet, the Workspace panel with its button, docking,
  graph and Undo, Learn (`Learn.tsx`, `Course.tsx`), Practice
  (`Practice.tsx`), the band's word mark with its dictionary card, the
  calculus of one's own's entry; with them `learn.spec.ts`,
  `lessons.spec.ts`, `practice.spec.ts`, `course.spec.ts`,
  `custom.spec.ts`, `chip.spec.ts` and the walks `learning`, `toLessons`,
  `attempting`, `answers`, `coursing`, `customising`, `practising`. About's,
  llms.txt's and the tour's tests moved to `tests/about.spec.ts`.

### Decisions where the guide left a choice

- **A chip is a proof of today's state** (one sequent, one calculus); the
  rail's cells are the views the stage could show before (the proof, its net,
  the truth table, the tableau, the tableau by hand, a model), and the sheets
  (translation, interpolant, normal form, resolution) open from "+ view" as
  panels until 79d makes them views. "A copy of this proof" opens a chip, as
  "Duplicate" did.
- **The toggles change the calculus as before**: with steps the restart
  warning asks first. The guide's "a view opens beside" is 79b's.
- **The goal's dock stays** on the drawing (its bar, its notes, its Close
  goal and new tab button) until 79b's goal menu replaces it; only its Cut
  moved to the tools.
- **The examples** shown with the guide on are the stored samples of the
  chosen ordinary logic (the linear suite has none to show yet).
- **The bottom box binds the proof's text**: Enter on a changed text restarts
  the sequent in place, as the field did.
- **More's rows** take the bottom box's place while open, and a tool pressed
  there closes them.
- **The box takes the focus** on the new tab view where nothing else has it,
  with a fine pointer only (a touch screen would open its keyboard).

### Departures from the design (for the author)

- **`good` is not the guide's value** (departs from the design): light
  `#075e55` for `#0A6B4F`, dark `#86e2ba` for `#6FCBA4`. The repository's
  colour test holds the colours of meaning apart under simulated colour-vision
  deficiencies (OKLab distance 8): the guide's green met lapis under
  tritanopia (5.7 light, 5.8 dark) and the dark red under deuteranopia (5.0);
  no green of the guide's hue at any lightness passed in light, so the light
  one turned 17° towards teal and darker. Both still meet 4.5:1 on paper,
  band, wash and over. The test's groups are the new page's: lapis left the
  net groups (a net is drawn only for a finished proof, where nothing on the
  drawing is chosen), `edge` left the verdicts (it marks no state), `warn` and
  Learn's colour are gone with what used them. The net colours moved within
  their Okabe-Ito hues for the new paper (they are the page's own).
- **Toggles on a touch screen are 44 high in a box of 50** (departs from the
  design): the guide says a cell 36 high in a box of 44, and also that every
  target on a touch screen is at least 44; the target-size walk holds every
  control to 44, so the cell took 44.
- **Rule labels get a zoom floor** (departs from the decision of 2026-11-14,
  which declined one): Learn's Reference was the equivalent control that
  excused a rule label's button under 24 pixels on a fitted phone drawing;
  with Learn gone the excuse went, and the axe walk (now without the
  exception) found the cut elimination's labels at 22.5 pixels on a phone.
  The stage zooms in until every label's button is 24 by 24, unless the
  visitor zoomed by hand.
- **No way to close one sequent**: the guide gives none (the chip has no
  close, and the old tab's × is gone with the tab); only Settings' "Close
  every sequent". A question for the author.
- **The fragment** (MLL, MELL) is no longer shown: the guide has no place for
  it.
- **The drawing's glow** in the dark scheme is gone with the old scale.

- **Lapis left the net's colour-vision groups** (departs from the design, and
  the one test change that looks a weakening; the reviewer's finding 4): the
  guide's lapis and the Okabe-Ito blue the nets' ⅋ edges keep (the colour
  test holds their hue) are both blue, and no blue of that hue clears lapis
  by the test's distances while it keeps 4.5:1 on paper and clears ink (a
  search over lightness and chroma found none in light). With lapis in the
  group the test fails in every column (light: 8.0 full, 4.5 under
  tritanopia). The groups were changed on the ground that a net is drawn
  only for a finished proof, with nothing on it chosen; but the chosen chip
  and the chosen net cell (lapis) stand on the same screen. The author
  decides: a net's ⅋ edges in another Okabe-Ito hue, a lapis further from
  the blue, or the groups as they are now.

### Links of every version

`#v=1` opens in the linear suite (unchanged reader); `#v=2` opens its
sequent with its session in its suite and parameter; `#v=3` (a linked group)
opens every proof of the group as chips, the linked one chosen, with the
note "This link holds N linked tabs."; `#v=4` (a course pack) opens the pack
and its bar; a hand-in link (`h=`) still opens the check view (79e removes
it). A link always opens; the visit's earlier proofs then wait for "Restore".

### Checks

- Vitest: 35 files, 726 tests pass (`keys.test.ts` new; `colours.test.ts`
  with the guide's pairs; `tokens.test.ts` with the guide's spacing and the
  three window classes). Biome, `tsc --noEmit` and `astro check` clean.
- Playwright against a fresh build (`astro build`, served as Pages serves
  it), cores 4 to 9, six workers: 306 passed, among them the axe walks
  (WCAG 2.0 to 2.2 A and AA and best practices, both schemes, desktop and
  phone, now without the rule-label exception) and the target-size walks
  (24 on a desktop, 44 on a phone). The elimination walks also pass in CI
  mode (60 s, `CI=1`) on one core with four workers, where they had failed
  before `wxknvkvo`.
- `nix flake check` in the client under `flock /tmp/linlog-machine.lock`
  with `--max-jobs 2 --cores 6`: passes at `wxknvkvo`, the chain's head
  (exit 0; its e2e check 306 passed). Three earlier runs at earlier heads
  failed in the e2e check, which runs in CI mode in the sandbox; each
  failure was fixed at its cause (below).
- Screenshots at 320 by 700, 390 by 844, 844 by 390, 700 by 900 and 1280
  by 800, light and dark, of the new tab view, an LK proof and the Settings
  page: `/tmp/claude-79a/screens/`; the boards rendered for comparison:
  `/tmp/claude-79a/boards/`; the runs' logs: `/tmp/claude-79a/runs/`.
- What the sandbox found that a local run did not: the controls' colour
  fade caught half-way by axe (the fade went), the walks' More pressed while
  Close all was briefly Stop (More's row now closes on any tool), and a
  phone's elimination press swallowed by the row's light dismiss (the
  same).

### The review

`fresh-reviewer` (Fable 5.2 at `medium`) read `nrynynyl` and `lqzlonkr`
against the prompt's 79a section, the guide and the checklist. Its findings
and the answers:

1. *The tools could not be reached by Tab* (every tool `tabindex=-1` until
   one had the focus; the toolbar test called `focus()`). Held; fixed in
   `lqyuxrrm`: `oneStop` keeps one tool that can act as the stop, as tools
   come, go and turn faint; the chips and the rail keep a stop on the
   Settings page and for a view shown without its cell; the test now
   presses Tab from the rail into the toolbar.
2. *A model cell could show another sequent's model* (one `ui.modelSeed`),
   and a model shown from a countermodel had no cell. Held; fixed: the cell
   reseeds from its own sequent, and a model or a tableau by hand shown by
   a button of its own gets its cell; a test proves each sequent's model.
3. *No sheet had a close button* (the guide's "A sheet has a close
   button"). Held; fixed: the panels and the list of views have one in a
   compact window, where they are sheets; tested in the compact test.
4. *Lapis left the net groups of the colour test.* Held as a finding; not
   changed in the code: reported as a departure above, for the author.
5. *The zoom's buttons and Close goal named no key.* Held; fixed
   (`aria-keyshortcuts`).
6. *The compact bottom box was 44 high, the guide says 48.* Held; fixed
   (`--size-box-compact`, 3rem).
7. *No container query* (the guide: panels and tools size themselves by the
   stage). Held; fixed: the wide panels narrow and a panel moves the drawing
   aside by `@container stage`; the toggles' one line stays a media query
   of the expanded class.
8. Doubtful points: Escape in a Settings field closed the page (fixed:
   Escape in a field leaves the field, a second closes); the keys and About
   dialogs did not take the focus and single keys acted behind them (fixed:
   they take it, give it back, and block every key but Escape); Undo after
   "Close every sequent" also restores proofs set aside before this visit
   (kept: they are the proofs the new tab view offered anyway); "The list of
   keys" and "About" leave the Settings page (kept: one `ui.menu`); stale
   header comments (fixed).

### What 79b must know

- The frame's parts are in `Frame.tsx`; the toggles in `Toggles.tsx`; the
  panels and the Settings page in `Panels.tsx`. The dock (`.dock`, `.gbar`)
  is still the goal's place for its actions; the goal's menu at `⊢`
  replaces it.
- The guide's sentence of help goes into the notes (`BottomBox`'s `notes`
  in `App.tsx`); the guide's switch is `Settings.guide`, and with it on the
  tools show `.tool-name`.
- The tour's texts are the old page's, its spots retargeted to the chip, the
  stage, a formula, the dock and the tools.
- `ui.added` holds the views added per proof; `cellsOf` lists the rail's
  cells; a toggle's change with steps still asks to restart.
- The Settings page is the old panel's form under headings; 79e makes it
  D10.
- A modal proof step by step still runs on `Custom.tsx` (suite `rules`),
  which stays for it; its own entry is gone.
- The workspace's state keeps its undo, redo and remake, unused by the
  page until the views' graph and "Update" (79d).

### Left open

- The author's calls: the colour `good`, lapis against the nets' blue, the
  rule labels' zoom floor, the touch toggles' 44, a way to close one
  sequent (all under "Departures").
- 79b: the goal's menu at `⊢` in place of the dock, the rule popover and
  its card, the guide's sentence and the tour's texts, the view beside the
  present one on a toggle's change, Fitch's toggle.
- 79d: the sheets as views of their own, the graph of views, "Update" and
  "Start again" on stale views (the workspace's undo has no button now).
- 79e: the Settings page as D10 (it is today's form under headings), the
  Export panel, the packs (the hand-in and the pack's bar stay as they
  were), the custom panel's leftovers in `state.tsx` (its entry is gone;
  it runs only the modal proof step by step).
- `src/lib/practice.ts`, `course.ts` and `content.ts` keep Learn's and
  Practice's data with their tests until those return.
- **`lucide-solid` stays for now** (the stack spec's §1.6, as few npm
  packages as practical; the guide draws its own icons on an 18-unit grid):
  five islands still import it after 79a (`App.tsx`, `Frame.tsx`,
  `Stage.tsx`, `Tour.tsx`, `Transforms.tsx`), for the tools, the top row,
  the zoom and the sheets. 79e removes the rest with
  `src/integrations/lucide.ts` and its test, since it draws the Export
  entries' own signs and is the last to touch the tools; 79b to 79d add no
  new import of it.
- **The new fonts** come from npm (`@fontsource/gentium-book-plus` 700,
  `@fontsource/ysabeau-office` 400, 500, 600) and are self-hosted as subset
  woff2 files by unicode range, built under `/fonts/` beside their OFL
  texts, as the client's other fonts are (the stack spec's §2, "Fonts").
- **The frame's commit is one large change** (`lqzlonkr`): the scale, the
  frame, the window sizes, the keys, the new tab view, Restore and the
  removals were built together and depend on one another (the token
  renames run through `app.css`, the removals through `App.tsx`), so only
  the fonts and the rules were split from it; the later commits each hold
  one unit.
