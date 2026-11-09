# Step 61: building derivations with custom rules, in the command and on the page

You are working in deduxit (`deduxit/deduxit`, the checkout `../linlog`) and, for the page, in
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
a fresh-context reviewer on Opus 5.5 at `high` on the session's state
(no operation yields a state the checker rejects). Deliverables: the
commits, the client's commits, the manual and guide,
`plan/reports/61-custom-rules-interactive.md`.
