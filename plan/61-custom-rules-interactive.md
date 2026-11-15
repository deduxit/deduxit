# Step 61: building derivations with custom rules, in the command and on the page

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and, for the page, in
the web client (`deduxit/deduxit.github.io`). CLAUDE.md applies
throughout: jj only (never git), thematic commits as soon as a unit is
done, doc comments on every item, the verification table, no pushing.
This prompt was written on 2026-11-08 by step 46's second session, from
the author's decisions of that day, and is finished at the review of
step 60: it says what is fixed.

Read `plan/notes/research/next/custom-rules.md` (candidates 2 and 6),
`web-front-end.md` (candidates 1 and 5), the reports of steps 49 and 60,
`.claude/rules/core-derivations.md` (interactive proving), and `plan/README.md`'s D13, D15,
D28 and D29.

## Goal

The author's "a way to add custom derivation rules and a way to
interactively build derivations with them" (2026-11-08): a session over
step 60's generic derivation, in the command (`deduxit interact
--calculus FILE`, with `rules`, `apply`, `undo`, `show`, `save`, `load`)
and on the page (a tab whose calculus is a rule file the user loads or
writes), the rules applicable to a goal listed by matching, every output
marked "valid in the calculus given".

## Fixed now

- No search and no certificate (D28); a goal is closed only by the
  user's rules, and the finished derivation is checked by step 60's
  checker.
- On the page the custom calculus is a suite of the registry (step 49),
  its rule file kept with the session's link and autosave, nothing sent
  to a server (`plan/web/design.md`'s D36).
- **Theories as hypotheses** (custom-rules-6): a chapter of the manual,
  no code, on what the Horn engine already runs (rule systems written as
  `!(a ⊗ b ⊸ c)`), and where a custom calculus is the better tool.
- The JSON forms of the session at a new wire level, as the linear
  session's.

## How the step runs

Two to three sessions on Opus 5.5 at `high` (the library and the
command; the bindings and the page, with the `frontend-design` plugin);
the `fresh-reviewer` agent (Fable 5.2 at `medium`) on the session's state
(no operation yields a state the checker rejects). Deliverables: the
commits, the client's commits, the manual and guide,
`plan/reports/61-custom-rules-interactive.md`.

## What step 60 left you (the review of 60, 2026-11-15)

- **`deduxit::rules`** is built (60a on Fable, 60b on Opus with the
  panel): the rule file and the generic derivation at wire level 15, the
  checker by matching, the printer (`Calculus::show`, `show_sequent`),
  the shipped calculi LK, LJ, LM, LL and the signed tableau, and `check
  --calculus FILE|LK|LJ|LM|LL|tableau` (`cli/src/calculus.rs`,
  `BUILT_IN`). Read `plan/reports/60-rule-schemas.md` whole, its "What
  the next sessions must know" above all, and `plan/notes/rule-schemas.md`
  (the design).
- **The backward step is yours**: from a goal, a rule and the positions,
  the premises. The compiled `Rule` has what it needs (`Shape`, `Entry`,
  `Find`, the ordered `equations`, all crate-private); a premise's
  patterns are instantiated with `Terms::apply`; what the conclusion
  leaves open (a variable no conclusion member binds, how a split
  context divides) is asked of the user, as the linear session asks a
  witness or a split. `Calculus::positions(rule)` is a count today: add
  the list of which member each position stands for. An open goal is
  `Step::Open`, which the checker refuses, so a finished session's
  derivation is checked whole, as the ordinary goal state's is.
- **Drawings**: `check` shows a generic derivation as text or JSON only.
  The page needs a drawing: the exports' `Drawable` over a
  `rules::Derivation` (a tree, every node one parent), which 62 extends
  with a Fitch list.
- **Wire levels**: 15 is the rule file and the derivation; 63t-a, beside
  you, takes 16. The session's JSON takes the next free level.
- **Not overwhelming** (the author, 2026-11-15): a user who never loads
  a rule file sees nothing new; on the page the custom calculus is one
  more suite in the registry, and its first screen explains in a
  sentence what a rule file is, with a built-in calculus (LJ or the
  tableau) to start from.
- **The workspace graph (63g)** will hold a rule file and a derivation
  in it as nodes: keep them immutable values with their JSON forms.
