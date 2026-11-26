# Step 79: the page rebuilt to the redesign, first order on it

You are working in the web client (`deduxit/deduxit.github.io`, a jj
workspace of its own beside `~/Projects/own/deduxit.github.io`) and,
where a session says so, in deduxit (`deduxit/deduxit`, the bindings in
`web/`). Both repositories' CLAUDE.md apply: jj only (never git),
thematic commits as soon as a unit is done, the verification tables, no
pushing. This prompt was written on 2026-11-25 by the planning session
(deduxit-orchestrator-2) from the redesign the author settled that day,
at the author's request ("Yes, draft them").

Read, in this order:
1. `design/redesign.md` in the client, at the redesign's head
   (`ottzpzmk`, the change "Record the countermodel tab's sign" and the
   guide's commits before it): the agreed design. It is the spec of this
   step. Where a board and the guide disagree, the guide holds.
2. Its boards, on the canvas the guide links
   (https://claude.ai/artifact/ALuD29h2Z6tiLhThP1ahFM), for the boards
   your session builds. Open it in the built-in browser pane.
3. `design/redesign-notes.md` beside it only where you need to know why
   a choice was made.
4. The client's `.claude/rules/` (`web.md`, `island.md`, `writing.md`),
   deduxit's `.claude/rules/web.md` for the bindings, and
   `plan/conduct.md`.

## Goal

The page is the guide's page: one sequent, many views; sequents across
the top, views down the rail, the stage for the derivation; the guide
for newcomers; every action on a key; WCAG 2.2 AA at every window size
from 320 pixels. Everything the current page does that the guide keeps
works there, and first-order logic (witnesses, eigenvariables,
countermodels) works on it, which step 79 was first planned for.

## Fixed now

- **The guide is the spec.** Do not redesign. A choice the guide leaves
  open is yours: take the one its principles support and say it in the
  report. A place where building shows the guide cannot work as written
  is marked "(departs from the design)" in the report for the author,
  with what you did instead; no question mid-run.
- **One correction to the guide, the author's** (2026-11-25): the page
  forbids only what is not a legal step. A legal step into a goal that
  has no proof is neither greyed nor warned of before it is taken. So
  the guide's line under "The derivation", that a split "says whether
  each can still be closed", is dropped: the split shows its two
  premises and nothing about whether they close (the function table's
  "Every split is legal, so the button is never greyed" holds). The
  marking of a goal that the search has shown to have no proof, after a
  step, stays as the guide has it (a setting, on at first).
- **The site stays as it is until the author approves the built page.**
  Each session's chain goes on the redesign's chain in the client, not
  on `main`; nothing is deployed, the client's `main` moves only when
  the author says so after 79e. Bumping the client's `deduxit` input on
  the redesign's chain is allowed (79c needs it).
- **Gone with the redesign** (the guide's "What goes, and what waits"):
  handing answers in and the check view; a pack's switch that bars the
  search, and a pack's code; a calculus given as a rule file on the
  page; the Workspace panel as a place of its own; the stage's row of
  view tabs, the Logic popover and its sheets; the bottom band; typing a
  tableau as text. Their code leaves the client in the session that
  replaces what they did, with their tests and states. The library and
  the bindings keep every call: the command and other front ends use
  them.
- **Learn and Practice wait.** Their content stays in `src/content/`;
  their islands leave the page; "Learn" and "Practice" are inactive
  buttons with the hover "Coming soon". They come back in a step after
  79, written then.
- **Every link the current page writes still opens** (`#v=1` to `#v=4`,
  `src/lib/link.ts`): a sequent with its views, the linked view shown. A
  hand-in link (`h=`) opens its problems with their proofs, as a pack
  without the check view. Say in the report how each version opens.
- **Untyped lambda-terms are not in deduxit.** "Untyped" is an inactive
  toggle with the hover "Coming soon" until the library has them
  (`plan/later.md`, "Untyped lambda calculi as a second axis").
- **Greyed rules come from trials until the library has the call**
  (`plan/later.md`, "Every candidate rule with why it is refused"): the
  page tries each candidate rule with the suite's `apply` (no state
  change on a refusal), `interactive.invertible` off for the trials, and
  shows the refusal's text. The reason is the library's error (its
  `code`, `kind` and `details`), worded by the page where the details
  carry fields, the library's message where they do not.
- **Rocq's logo** is the one third-party file the client holds, as the
  guide says (the Unlicense). It changes the client's rule "No
  third-party file is committed" (CLAUDE.md, "Licences"): 79e changes
  the rule and the licences check with the file.
- **The stack is the author's stack spec**
  (`~/Projects/own/website-stack/stack-spec.md`), with the client's own
  decisions as §5 in its `docs/stack-spec.md` (hosting stays on GitHub
  Pages, the author's word of 2026-11-25). Read §1, §2, §3.4 (Solid
  1.x, the island rule) and §4 before adding anything. In particular:
  as few npm packages as practical (§1.6), so `lucide-solid` leaves once
  the guide's own icons replace it; no `style` attribute (§4.4).
- **The client's `deduxit` input moves in step 79** (fixup-27 held it,
  2026-11-26): at deduxit's `main` of then, `about` lists the five
  first-order normal forms (prenex, miniscope, Skolem, Herbrand,
  clausal) beside the propositional ones, and the client's
  `src/lib/clausal.test.ts` ("offers the forms deduxit lists") fails
  until the page offers them. The session that first moves the input
  (79c, for its bindings call) adds them to the page's forms, with the
  normal-form view's handling of them left to 79d if 79c's frame has no
  such view yet, and drops the Biome override in the client's flake
  (Biome 2.6.8 is in that nixpkgs). With it, two corrections fixup-27
  found: the test in the client's `.claude/rules/flake.md` for whether
  nixpkgs' Biome is cached becomes the store path's lookup (`nix
  path-info --store https://cache.nixos.org $(nix eval --raw
  --inputs-from . nixpkgs#biome.outPath)`; the one written there tries
  to copy the derivation and fails with 404), and the comment in the
  client's `flake.nix` that its followed inputs are all past fourteen
  days is made true to what happens: they follow deduxit's lock, which
  takes the newest (the fourteen days are npm's and the actions' gate,
  stack spec §3.8). The client's `main` moved under the
  redesign chain meanwhile (npm updates, `tnvosplw`): the planning
  session rebases the chain onto it at 79a's review, and a conflict in
  `package-lock.json` is resolved by `npm install --package-lock-only`
  from the merged `package.json`, never by hand.
- **Fonts** come from npm through the lock, as the client's fonts do:
  Gentium Book Plus and Ysabeau Office are new (SIL OFL 1.1; check that
  the client's licences check and `src/integrations/licences.ts` list
  them).

## How the step runs

Five sessions in order, each on Opus 5.5 at `high`, with the
`frontend-design` plugin and, for the guide's sentences and hints, the
education skills `cognitive-load-analyser` and
`ai-feedback-design-principles`. Each is reviewed by `fresh-reviewer`
(Fable 5.2 at `medium`) against the guide's sections it builds, and by
the planning session. Each builds on the one before; none runs beside
another, since all of them change `state.tsx`, `Stage.tsx` and
`app.css`.

Each session's checks: `nix flake check` in the client at its head
(Vitest, Biome, `astro check`, Playwright with the target-size and axe
walks, the CSP and licences checks), under the machine lock; the
deduxit part, where there is one, by deduxit's verification table
(the bindings' tests under Node at least). Every interaction it builds
gets a Playwright test and every new state a step in `tests/states.ts`,
as the guide's "Building it in this repository" asks. The narrow
layouts are checked at the guide's widths (320, 390 upright, 844 by
390 sideways, 700, 1280), in the browser pane, with screenshots kept in
the session's report folder, not in the repository.

### 79a: the frame

The scale and the frame, with today's proof views moved into them.
- The tokens of "The scale" into `src/styles/tokens.css`, light and
  dark, `edge` new; the colour pairs into `src/styles/colours.test.ts`;
  the type, spacing, corners, borders, controls, icons and motion.
- The frame of "The frame": the top row with the chips and "+", the
  rail with its cells and "+ view" (the list may be empty but for the
  views that exist today), the stage with its dots, the toggles' place,
  the tools' column, the zoom group, the bottom box, the notes.
- "Window sizes": the three classes and the short one, the compact
  layout's bar and row, the sheets, `pointer: coarse`; the one
  breakpoint of today and its test replaced.
- "Keys": the table, the rules (single keys off by a switch, `?` the
  list, `J`/`K`), `aria-keyshortcuts`; `KEYS` in `BottomBand.tsx`
  replaced.
- The new tab view (board D1): four logics, the bottom box, "Open a
  file", drop a file, JSON pasted, About, Learn and Practice inactive.
- Restore: by itself on a plain visit when no other tab of the page is
  open (the Web Locks API: each visit holds a lock named by its shelf
  key), "Restore N proofs" otherwise; Settings' "Close every sequent",
  which can be undone.
- Light and dark from the top row and Settings, read before first
  paint.
- The LK and linear proof views as they work today, inside the frame,
  so that the page proves sequents again before 79b.
- The bottom band, the view tabs, the Logic popover and the Workspace
  panel leave.

### 79b: proving by hand and the search

Boards D2, D4, D6, D7.
- The rule popover: one button per rule, the greyed ones among them by
  trials (above), the card with the rule drawn, the reason on hover and,
  on a touch screen, as a note on a press; the digits and arrows.
- Splits and cut as the guide draws them (two dashed premises, chips
  moved by a press, the apply button), with the correction above.
- The goal's menu at `⊢` (in natural deduction at its `?`): close this
  goal by the search, prove it apart, use a proof made apart, go back.
  A proof made apart closes the goal by replaying its steps on the goal
  (positions mapped from the apart sequent's members to the goal's), its
  steps grey and marked "apart"; only if a replay cannot map them, ask
  the planning session for a library call before building one.
- Undo, redo, Steps; Hint as the guide's ladder from where the visitor
  is; Close all with its caret (the search's settings), Stop, "Search
  longer" after Unknown.
- The marking of goals with no proof (a setting, on at first), and "Go
  back". It runs a search on each open goal after a step: measure what
  it adds to a step on the client's stored problems (the worker's time
  from the step's request to its view, the median and the slowest), and
  report it. If a step on a stored problem passes 200 ms because of it,
  send the planning session the numbers mid-step and go on with the
  rest.
- The trials' cost the same way: the time from a press on a formula to
  the popover with its greyed rules, on the same problems.
- The calculus toggles of ordinary and linear logic and natural
  deduction ("Sequent", "Natural", "Fitch"; the logics; "Mix",
  "Affine"; a net's "Formulas" or "Compact"); a change that would lose
  steps opens a view beside.
- The guide on: the tools' names, one sentence of help above the bottom
  box (with the education skills), the tour.
- A first-time task tried in the browser pane by a fresh agent (Sonnet
  5.5 at `high`) that has not read the guide: prove `a ∧ b ⊢ b ∧ a` in
  LK with the guide on, then a linear `⊗` split; its findings answered
  in the report.

### 79c: first order

Boards D8 and D9, and the guide's "Not drawn" items on first order.
- The witness box in the popover for `∃R` and `∀L`, the refusal of a
  typed witness under its field; `∀R` and `∃L` taking deduxit's
  variable; the eigenvariables as the session names them; in the
  ordinary and the linear suite.
- The countermodel view `⊭` of a first-order sequent: the structure
  drawn, a press on a dot flips a predicate, "+" and "−" an element, the
  sequent's checks and crosses following at once, `⊭` or `⊨`, Undo and
  Redo. The check is the library's (`FiniteCountermodel::check` and
  `Structure::refutes`, `core/src/ordinary/structure.rs`), which name
  the first formula a structure gets wrong; a check or cross on every
  formula is each formula evaluated alone (a sequent of that one
  formula), in the bindings, with no library change unless that cannot
  work. The bindings have no call for it yet, so this session adds one
  in deduxit (`web/src/ordinary.rs`, a
  row in `plan/web/README.md`, a test in `web/tests/ordinary.rs`, the
  protocol's `describe` if it lists the suite's calls), sends "deduxit
  ready" after that part, and bumps the client's input once the
  planning session has pushed it. The propositional and modal Kripke
  models move into the same view.

### 79d: the other views

Boards D3, D11, D12, D13, D14, D15, D19.
- "+ view": the grouped list ("Prove", "Decide", "Translate", "Read
  off"), faint entries with their reason, checks on what is there.
- Every view of D13 and D14: truth table, tableau (search, by hand),
  resolution (search, DPLL, by hand), normal forms with their steps,
  interpolant, a model of one's own, translations with "Prove it in",
  cut elimination, the lambda-term, a copy of a proof; derived views
  joined in the rail, a capsule for a proof and its net, stale views
  with "Update" or "Start again".
- The graph of views (D19) as the rail's first cell. Use the library's
  workspace document as the model of a sequent's views where it fits
  (the current page's `Workspace.tsx` does), and say in the report what
  you took.
- Modal logic (D11): the system's name and its conditions as toggles,
  decided on Enter and on a change, the countermodel view, "Prove step
  by step" in the labelled calculus.
- Lambda-terms (D12): typed, with the use toggle, a term checked and
  drawn or a type with a term found, "List its terms"; "Untyped"
  inactive (above).
- The drawing (D15): pan, zoom, fit, the zoom group, a net's look.

### 79e: export, packs and settings

Boards D10, D16, D17, D18, N1 to N10.
- The Export panel: the entries with their signs, each format's
  settings passed on to deduxit's `--style` options (the guide's "What
  the page needs from elsewhere"), the text a text format writes,
  "Copy" and "Save", the links.
- Course packs: the entry under "Link" with its form, the link, the
  embed code, the printed sheet; a pack as a student meets it, one
  chip with its count and a page of its problems.
- The Settings page (D10): Look, Search per logic, Proving, Keys, This
  browser, About with the keys, the licences and privacy.
- Rocq's logo, with the client's rule and licences check changed.
- The phone and tablet boards checked against the page at their sizes.
- The whole page walked once more at every width of the guide's test
  list, then the report says what the author should look at before
  approving the page.

## What 79a left (its review, 2026-11-26)

- The frame is built (`Frame.tsx`, `Toggles.tsx`, `Panels.tsx`), and
  today's views run inside it; the report's "What 79b must know" and
  "Left open" say what each later session takes over: the goal's dock
  until 79b's menu at `⊢`, the sheets as panels until 79d makes them
  views, the Settings page as today's form until 79e, the hand-in link
  still opening the check view until 79e.
- The redesign chain (with 79a) was rebased onto the client's `main` of
  2026-11-26 (`tnvosplw`); its `package-lock.json` was made again by npm
  from the merged `package.json`, and the client's full check passed at
  the rebased head `wxknvkvo`.
- **Held for the author** (79a's departures, each marked in its report):
  the colour `good` moved towards teal to pass the repository's
  colour-vision test; lapis left the nets' colour-vision groups; the
  touch toggles' cell is 44 high; rule labels get a zoom floor of 24
  pixels; the fragment (MLL, MELL) is no longer shown. They stand as
  built until the author decides.
- **Closing one sequent** (79a found the guide gives no way): decided by
  the planning session while the author was away (departs from the
  design, for the author's review): a chip gets a close button, shown
  on hover and focus and always on a touch screen, which closes that
  sequent with an Undo in the notes, as "Close every sequent" does; and
  the key `W` beside it in the keys' table. 79d builds it with the rest
  of the sequents' and views' management.

## What 79b to 79d left (their reviews, 2026-11-26)

- 79b, 79c and 79d are complete, each checked by the client's full
  check at its head; their departures are marked in their report
  sections and held for the author with 79a's.
- **Closing a chip without a keyboard** (79d: the chips are a tab list,
  which may hold tabs alone, so the close is a target hidden from
  screen readers, and Delete or `W` closes by keyboard; a screen reader
  on a touch screen had no way). Decided by the planning session while
  the author was away (departs from the design, for the author's
  review): the More menu gets "Close this sequent", a real button outside
  the tab list, closing the chosen sequent with the same Undo. 79e
  builds it.
- **Small faults 79d left open, 79e's**: resolution by hand loses its
  resolvents when its cell is left (keep them with the view, as other
  views keep their state); the chip that the embedding's "Prove it in"
  opens is not linked in the workspace, so the graph has no node for it;
  Undo of a chip's close brings its proofs back without their workspace
  links. And the three tests that passed only on a retry, each once
  (`tests/prover.spec.ts`'s reload right after a `goto`, `ordinary.spec.ts`'s
  "goes back to the last goal still valid", 79c's witness target walk):
  fixed at their cause, none by a retry or a longer timeout.

## Deliverables

The client's commits on the redesign's chain, deduxit's commits of 79c
(and of any session that needed the bindings), the client's rules
(`.claude/rules/web.md`, `island.md`) changed to the new page, and
`plan/reports/79-page-redesign.md`, a section per session.
