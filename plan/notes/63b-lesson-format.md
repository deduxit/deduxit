# 63b: the lesson format by suite, and the ordinary course

Session 63b's design, before any client commit (2026-11-16). Sources:
the step prompt (`plan/63-suite-protocol.md`), web-teaching-5,
education-3 and education-10, web-front-end-7, D47, D56 and D60, and a
survey of the client at `kvwnuzlv` plus 62d.

## What the client has

- Six content collections (`src/content.config.ts`), every one keyed by
  a linear `Mode`: `problems` (452 files, 407 with a session; classical
  239, intuitionistic 192, the rest affine, Mix and both), `lessons` (11,
  two with an attempt or refusal session beside them), `rules` (36),
  `tour` (8), `texts` (4), `examples` (the ordinary field's menu).
- Learn's list is a hardcoded array of eleven names (`Learn.tsx`), the
  next lesson `n + 1`. A worked proof replays linear `Action`s
  (`steps.ts`, `state.tsx` `worked`, `linear(example.mode)`).
- Generic already: explain, hinge, and predict-by-rule-name. Linear:
  detour (a split), faded (a replay prefix), attempt, unknown, eliminate,
  counts, refusal, gallery, rewrite.
- Practice reads `/problems.json` (`offered`), filters by linear
  connectives, logics and source, and tracks a proved problem only in a
  linear tab.
- Lesson 11 teaches ordinary logic through the translations inside the
  linear course: it stays there, as the bridge lesson web-teaching-5
  names.

## The format

- **A folder per suite**: `src/content/linear/` and
  `src/content/ordinary/`, each with `lessons/` and `problems/` (and
  `rules/` for linear). A custom calculus has none: the page says it is
  a tool, not a course.
- **Collections per suite, ids unchanged**: `problems`, `lessons`,
  `rules` keep their names and their ids with their `base` moved to
  `src/content/linear/…` (an Astro id is the path under the base, so no
  `worked:` reference and no template key changes); the ordinary suite
  adds `ordinaryLessons` and `ordinaryProblems`. The move is renames
  only: a test holds every moved file's SHA-256 to a manifest taken
  before the move, and the e2e lessons and states pass unchanged.
- **One schema per suite, built from shared parts**: the lesson's core
  is every suite's (`lesson`, `title`, `worked`, `steps` of `{rule,
  reason}`, `choices`, `faded`, `explain`, `hinge`), as zod parts in
  `src/lib/course.ts`; the linear schema is the core plus today's
  linear parts, unchanged; the ordinary schema is the core plus
  `method` (`sequent`, `natural`, `tableau`, `table`) and `layout`
  (`tree`, `fitch`) for natural deduction, and an optional `model` (a
  problem whose countermodel the lesson reads). The JSON Schema of each
  (`z.toJSONSchema`) is committed beside the content
  (`src/content/<suite>/lesson.schema.json`, `problem.schema.json`),
  held equal by a test: the format other tools read (education-10).
- **An ordinary problem**: `sequent` in the ordinary syntax, `logic`,
  `verdict` (`valid`, `not_valid`) in that logic, `level`, `categories`
  (`conjunction`, `disjunction`, `implication`, `negation`, `constants`,
  and `classical`, `intuitionistic`, `minimal`, `literature`),
  `methods` it suits, optional `lesson` and `order`; beside it the
  session of each method it has a worked proof in:
  `<id>.sequent.json` (an ordinary session, level 8) and
  `<id>.natural.json` / `<id>.tableau.json` (a session of the `rules`
  suite in NJ, NK, NM or `tableau`, level 17 or 18).
- **Playing a worked proof without a step format**: the ordinary and
  the custom-calculus bindings both have `undo`, so the generic player
  reads the finished session, takes it back step by step to its start,
  and keeps every state: step `k` of the worked proof is the state with
  `k` steps; the faded proof starts at state `faded`; a step's rule is
  the one the session's next state added. The linear player stays as it
  is. (A protocol `undo` in 63a would serve the same player.)
- **Checks**: `ordinaryProblems.test.ts` holds every verdict to
  `ordinary_start` and `ordinary_close` in Node, every session to its
  sequent and its finish (`ordinary_view`, `rules_check`); the flake's
  `problems` check gains the ordinary store, each problem through
  `deduxit prove --logic L --format json` and each natural or tableau
  session through `deduxit check --calculus`.

## Learn and Practice by suite

- **The course follows the tab's suite**: a linear tab's Learn is
  exactly today's (the eleven lessons, nothing added to its screen); an
  ordinary tab's Learn lists the ordinary course; a custom-calculus
  tab's says in one sentence that this suite has no course. The
  tutorial's list ends with one line naming the other suite's course,
  which opens it in a new tab of that suite (one control, at the end:
  the visitor who comes for linear logic meets it only after the
  eleventh lesson).
- **Practice follows the tab's suite** too: its filters are the suite's
  (`GROUPS` per suite), its store `/problems.json` per suite, its
  proved-tracking keyed by suite and parameter.
- **Auto-reverse** (education-3): the ordinary sessions apply no
  invertible rule by themselves today (38h's toggle is the linear
  suite's). Whether the ordinary lessons need it is answered by the
  content: LK's propositional rules are all invertible but the
  contractions, so a lesson on LK either plays them or turns them on;
  answered in the report once the sequent lesson is written.

## The ordinary course (the progression)

Built with `explicit-instruction-sequence-builder` (the worked proof
the "I do", the faded proof the "we do", practice the "you do"); each
lesson one objective, its hinge one misconception.

1. **Valid, by the truth table.** Objective: a sequent is valid when no
   row makes every formula left of ⊢ true and every formula right of it
   false. Worked: `a → b, a ⊢ b`, then `a → b, b ⊢ a` with its
   falsifying row. Hinge: "valid means true in some row". Fading: the
   student finds the falsifying row of the second.
2. **Signed tableaux.** Objective: take formulas apart by their sign
   until every branch closes on a clash. Worked: `⊢ (a ∧ b) → (b ∧ a)`
   in the `tableau` calculus. Hinge: "an open branch means the tableau
   is unfinished" (it is a countermodel). Fading: the last split.
3. **The sequent calculus LK.** Objective: build a proof backwards from
   the goal, the rules taking one connective apart. Worked: `a ∧ b ⊢
   b ∧ a`, then `⊢ a ∨ ¬a`. Hinge: "∨R must choose a disjunct" (LJ's
   rule, not LK's). Fading: the closing axioms.
4. **Natural deduction, backwards.** Objective: an introduction rule
   builds the goal's connective and may open a subproof whose hypothesis
   it discharges. Worked: `⊢ a → (b → a)` in NJ, the Fitch view.
   Hinge: "a discharged hypothesis must be used" (a vacuous discharge
   is fine). Fading: the second `→I`.
5. **Natural deduction, forwards.** Objective: an elimination rule
   derives a new line from lines in force. Worked: `a ∧ b ⊢ b ∧ a`
   (`∧E` twice, then `∧I`), then `(a → b) ∧ (b → c) ⊢ a → c`. Hinge:
   "any line above may be cited" (not one of a closed subproof).
   Fading: the last elimination.
6. **Classical, intuitionistic, minimal: a countermodel.** Objective: a
   sequent valid classically may have no intuitionistic proof, and a
   Kripke model shows it. Worked: `⊢ a ∨ ¬a` in NK by `RAA`, then its
   countermodel in intuitionistic logic. Hinge: "no proof found means
   the search gave up". Fading: none; practice decides sequents in the
   three logics.

Practice: 60 to 80 problems across the six (connectives, logics, levels
1 to 3), each decided by the command.

## Open, for the planning session

- Whether the course follows the tab's suite (above) or a suite switch
  in Learn's header; the first adds nothing to a linear visitor's
  screen.
- Lesson 2 builds the tableau in the custom calculus `tableau`; the
  ordinary suite's own tableau (`ordinary_tableau`) is drawn, not
  built. A lesson that only plays it is the alternative.
