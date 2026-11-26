# Step 81: every candidate rule with its reason, and a step's term written out

You are working in deduxit (`deduxit/deduxit`) and its web client
(`deduxit.github.io`). CLAUDE.md applies throughout. This prompt was
written on 2026-11-26 by the planning session, after the author's review
of the redesigned page (`plan/79-page-redesign.md`, "The author's
review").

Read `plan/later.md`, "Every candidate rule with why it is refused (the
redesign, 2026-11-25)", whole; the sections of 79b ("Departures", "What
79c must know") and 79c ("Departures": an ordinary replay leaves its
witnesses open) of `plan/reports/79-page-redesign.md`; the rules files
of the interactive proving modules (`.claude/rules/core-*.md` for the
linear interactive state, the ordinary suite, calculi given as data, the
suites' protocol) and of the bindings (`.claude/rules/web.md`); and in
the client `.claude/rules/web.md` (the rule popover's trials).

## Goal

Two library wishes the page left, both now the author's yes ("do all of
them", 2026-11-26):

1. **Every candidate rule with its reason.** One call per formula of a
   goal that returns every candidate rule of the calculus, each as
   "applies", "needs input" (a split, a witness, a cut formula, natural
   deduction's values) or "refused" with the step's refusal as the
   library writes it (`code`, `kind`, `details`). In the linear suite,
   the ordinary suite (LK, LJ, LM, first order) and the rules suite
   (natural deduction), through the bindings to the page. The refusals
   that carry a sentence today (`binding`, `witness`, `step_refused`)
   get fields.
2. **A step's term written out.** The ordinary session document records
   a witness term by its index, and the library has no public printer of
   a term, so the page cannot replay an ordinary proof's witnesses: they
   replay open. Give the session document (or a call over it) each
   inference's term as text the reader takes back.

## Fixed now

- **Legal steps only** (the author, 2026-11-25): a rule is refused only
  where the step is not legal on the goal as it stands; whether a legal
  step's goals can be closed is never judged.
- **The shape is the most idiomatic, current one** (the author's
  standing instruction): look at how current proof assistants and
  interactive provers report an inapplicable step, from their current
  documentation, not memory, and say in the report what you took.
- The call costs no more than one `apply` per rule, and less where a
  suite can say it from the formula alone; measure the page's press to
  the popover before and after on the client's stored problems as 79b
  did (its median was 14.9 ms, slowest 20.4 ms).
- Library, bindings (a row in `plan/web/README.md`, a test in
  `web/tests/`, the protocol's `describe`), the guide for AI models and
  the manual where they describe the entry points, the changelog's
  `[Unreleased]`, and `breaks 0.8.0` read for what it lists.

## How the step runs

One session, Opus 5.5 at `high`, in a deduxit workspace and a client
workspace, `fresh-reviewer` (Fable 5.2 at `medium`) before its final
message.
1. The library and the bindings, verified by deduxit's table (the gate,
   the bindings' tests under Node, the `cargo hack` runs if a feature's
   `cfg` is touched), committed thematically. Then "81: deduxit ready,
   <change id>" to the planning session, which reviews it, runs the full
   check on GitHub's runners and pushes deduxit's `main`.
2. After "deduxit pushed": in the client, on its `main@origin`, `nix
   flake update deduxit` as a commit of its own, then the page: the rule
   popover takes the call in place of its trials (the refusal's wording
   kept to the guide), and an ordinary replay writes its witnesses.
   Playwright tests and `tests/states.ts` steps; the client's full `nix
   flake check`.
3. The report, `plan/reports/81-interactive-api.md`: the call's shape
   and why, the measurements, the checks, the reviewer's findings and
   answers, what is left open.

## Deliverables

deduxit's commits (library, bindings, docs), the client's commits on its
`main` (pushed by the planning session after review), the report.
