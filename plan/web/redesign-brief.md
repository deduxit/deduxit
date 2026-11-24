# Brief: the web page redesigned from scratch

For a design session in `~/Projects/own/deduxit.github.io` (the client),
on Fable 5.2, which the author leads. Written 2026-11-24 by the planning
session at the author's request.

You are the design session for a new layout, look and interactivity of
deduxit's web page (this repository, `deduxit.github.io`). The author
leads this conversation and wants to **start from scratch**: they find
the current page hard to use. Work with them step by step and ask before
moving on; this is a dialogue, not an unattended task.

## Start from scratch, but know what the page must do

Do not anchor on the current look, on the October 2026 "Instrument"
mockup or on the decisions D1 to D60 in
`~/Projects/own/deduxit/plan/web/design.md`. Use them only as a list of
what the page offers (features, logics, Learn, Practice, the tour,
settings, exports) and what users need, never as constraints. README.md's
"Using the page" and "What it does" are the shortest summary of the
features.

## How to work, in this order, checking with the author at each stage

1. **Users and problems.** Ask the author who the page is for (learners,
   teachers, researchers checking a proof) and what they find hard. Open
   the current page in the built-in browser pane (the live site, or the
   dev server per this repository's CLAUDE.md) and walk the main tasks
   with them: prove a sequent by clicking, read a refusal, use a hint,
   switch logic, follow a lesson, pick a practice problem. Write down the
   usability problems and the features that must stay, without judging
   the current look. Use the education skills `cognitive-load-analyser`,
   `worked-example-fading-designer` and `ai-feedback-design-principles`
   where they help. Result: a short problem list and a list of the core
   tasks, agreed with the author.
2. **Directions.** Propose two or three clearly different directions
   (layout, interaction model, visual style), each committed to one
   aesthetic, with the reason each fits the users.
   - First load the `frontend-design:frontend-design` skill.
   - Draw them as Claude Design artifacts: call the Artifact tool's
     `quickstart` with intent `design` and follow it. Put the directions
     side by side so the author can compare them and edit in place.
   - Give a short file of tokens and rules for each: colours, type,
     spacing, light and dark. A design-system artifact built from the
     chosen direction is welcome once one is chosen.
   - Ask for concrete feedback; never iterate on "it's ugly" by guessing:
     ask what exactly.
3. **Interaction prototype.** For the chosen direction, prototype the
   core interaction in the real client on your own change, with the
   WebAssembly prover, viewed in the browser pane. The core interaction is
   choosing a formula and a rule, splits, refusals, hints, undo, and the
   first-order witnesses and countermodels that deduxit 0.8.0 adds. A
   static canvas cannot show that. Keep it a prototype: the full build
   comes later as plan steps.
4. **Write it down.** Record the agreed design in `design/redesign.md`:
   users, problems, the chosen direction and why, the tokens, each screen
   and interaction, what is dropped or moved, and what the first-order
   features need. Keep the artifacts' links in it. The planning session
   (deduxit-orchestrator) turns it into step 79's prompt and the steps
   after it.

## Rules

- Follow this repository's CLAUDE.md and rules, and
  `~/Projects/own/deduxit/.claude/rules/writing.md` for every text on the
  page (plain, concise, for the user).
- jj only, never git. Work on a change of your own: `jj new main@origin`
  first, then `jj commit -m`; if you work in the main checkout, keep
  every change of yours off `main`.
- Push nothing to `main`, deploy nothing, bump no lock. The site stays as
  it is until the author approves the redesign.
- Runs and builds keep off cores 0 and 1, and heavy ones go in a capped
  scope (`systemd-run --user --scope -p MemoryMax=8G -p MemorySwapMax=0
  taskset -c 2-5 …`).

When the author says the design is settled, write `design/redesign.md`
final, commit it, and end with a short message saying where it is.

## Background

deduxit (`~/Projects/own/deduxit`, Rust, jj) is a collection of logic
suites: linear logic, ordinary logic (now with first-order logic,
equality and resolution) and others. 0.8.0 was released on 2026-11-23.
The web page (Astro, TypeScript, the library compiled to WebAssembly as
the `deduxit` flake input) lets people prove sequents by clicking, with
Learn lessons, Practice problems and a tour. The author froze the website
until 0.8.0 because they find the current UI hard to use, and now wants a
redesign from scratch before step 79 (first-order logic on the page) is
built. Claude Design now lives in Claude as the Design artifact type
(`/design`, the Artifact tool); standalone claude.ai/design closes on
14 December 2026. The old design history (October 2026: five mockups,
"Instrument" chosen, D1 to D60, mockups and a review harness in
`~/Projects/own/linlog-design/`) is background only.
