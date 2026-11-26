# Step 82: a first-order goal closed by proof search, or refuted

You are working in deduxit (`deduxit/deduxit`) and its web client
(`deduxit.github.io`). CLAUDE.md applies throughout. This prompt was
written on 2026-11-26 by the planning session, after the author's review
of the redesigned page (`plan/79-page-redesign.md`, "The author's
review").

Read the reports of steps 75 (proof search at first order) and 76
(countermodels at first order), the section of 79c in
`plan/reports/79-page-redesign.md` ("The bindings call and its tests",
"Departures"), the rules files of the ordinary suite, its interactive
state and the bindings, and `.claude/rules/web.md` in the client.

## Goal

The goal state of the ordinary suite refuses `close` and `hint` on a
first-order goal. Since 79c the bindings answer such a goal with the
finite model search alone (`refuted` in `web/src/ordinary.rs`), so the
page can refute a first-order goal but never prove one. The author's
word (2026-11-26): "do what is most idiomatic".

The idiomatic answer of an interactive prover's automation is to try to
prove the goal and, where that fails, to look for a countermodel (as
Isabelle's automation and its model finder do side by side, and as a
tableau prover's open branch does). So:
- `close` on a first-order goal searches for a proof within the
  settings' bounds with the first-order engines of step 75, and grafts
  the derivation it finds, checked by the ordinary checker, as `close`
  does for a propositional goal;
- where no proof is found within the bounds, it searches for a finite
  countermodel (step 76) and answers it, checked;
- otherwise unknown, with the bound that stopped it;
- `hint` gives the first steps of a proof found, or the countermodel,
  the same way; the page's marking of goals with no proof uses the same
  call within its smaller budget.

Check the current practice of interactive provers from their current
documentation, not memory, and say in the report what you took.

## Fixed now

- **Nothing grafted unchecked.** A derivation the search returns is
  checked before it enters the session. If an engine returns a
  certificate that is not an LK or LJ derivation, its translation into
  one is part of this step, with its own tests.
- A goal naming an eigenvariable is searched as it is: the eigenvariable
  is a fresh constant there.
- The bindings' `refuted` branch goes; `close` and `hint` take the
  library's answer.
- Library, bindings (a row in `plan/web/README.md`, a test), the guide
  for AI models and the manual where they describe the goal state, the
  changelog's `[Unreleased]`, `breaks 0.8.0`.
- If the search moves the focused or net engines' behaviour, the target
  sets as CLAUDE.md's table says; a search on the client's stored
  first-order problems measured (time per close, median and slowest).

## How the step runs

One session, Opus 5.5 at `high`, `fresh-reviewer` (Fable 5.2 at
`medium`). The library and the bindings first, "82: deduxit ready" to
the planning session, which pushes deduxit's `main` after its review and
the full check; then the client's input moved and the page: the goal's
menu and the notes say "proved" for a first-order goal closed by the
search, and 79c's departure ("refuted but never proved") is gone, with
Playwright tests. The report, `plan/reports/82-first-order-close.md`.
