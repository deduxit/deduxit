# Step 49: ordinary logic step by step, in the command and on the page

You are working in the linlog repository (after step 48, the repository
`sequentium/sequentium`; the crate and the command are still `linlog`).
CLAUDE.md applies throughout: jj only (never git), thematic commits as
soon as a unit is done, doc comments on every item, the verification
table, no pushing. The step has three sessions, the last in the web
client's repository. This prompt was written on 2026-11-08 by step 46's
second session, from the author's decisions of that day. It is complete;
the planning session adds the facts of steps 47 and 48 at their review.

Read `.claude/rules/core-ordinary.md` and `core-derivations.md`,
`plan/later.md` ("Follow-ups: ordinary logic", and "Ordinary logic's JSON
forms" under "Follow-ups: the web bindings"), the report of step 25
(`plan/reports/25-ordinary-logic.md`), `plan/notes/research/next/`
`ordinary-sequents.md` (candidate 3 and section 5) and `web-front-end.md`
(section 1 and candidates 1 and 2), `plan/web/design.md` (D2, D35, D50,
D56, D57, D58), `.claude/rules/web.md`, and `plan/README.md`'s D13, D15,
D16, D24, D25, D29.

## Goal

A first logic course can use linlog for what it teaches first: LK and LJ
(classical, intuitionistic and minimal propositional logic) proved step
by step, in the command (`linlog interact` with the ordinary logics) and
on the web page, where a tab's calculus choice offers "Ordinary" beside
"Linear" for the first time. This is the collection's second suite made
whole (D24): `core/src/ordinary/` already decides through the
embeddings, reads the proof back as an LK or LJ derivation and checks it
(`Derivation::check`); it has no step-by-step state, no JSON forms, and
its tab is greyed on the page.

## Fixed now (the author, 2026-11-08, and step 46's research)

- **A native goal state of ordinary sequents** (`ordinary-sequents.md`,
  candidate 3, design b), not a session over the linear image (design
  a, `later.md`'s sketch of `interact --logic`, which this step replaces):
  the goals are two-sided ordinary sequents, the rules are the
  checker's rules (`ordinary::Rule`), a finished state is an
  `ordinary::Derivation` that `Derivation::check` validates, so the
  trusted base does not grow. The linear session (`core-derivations.md`)
  is the model of its shape (D13: a partial derivation, goals, rules
  applicable to a goal, apply, undo, close a goal by search within a
  budget, a hint read off the search's proof); "close" and "hint" use
  `ordinary::decide` and read the found derivation's first rule. Say in
  the report what the two sessions' code copied from each other: that
  record is what the kernel crate is later extracted from (D25, step 67),
  and nothing generic is factored out now.
- **Rule sets as a display choice**: the same derivation shown with the
  structural rules explicit or admissible where the checker's rule table
  allows both, as an option value (D15), not as two calculi.
- **Termination is honest**: a goal the search cannot decide answers
  "unknown" with its reason, as the command does today (the 35 ILTP
  problems a loop check would refute are step 54's).
- **The page shows LK and LJ, never the linear image** by default
  (`web-front-end.md`, candidate 2's risk).
- **The client's suite registry** (`web-front-end.md`, candidate 1),
  written with this suite and not before it: the closed `Mode` of the
  client becomes a suite, a method and the suite's parameters; the tab's
  calculus popover lists the registered suites, each with its own
  switches; the link and the stored proofs gain a suite field, old links
  and old stored proofs read as the linear suite (the link codec's
  version mark, `link.ts`), tested. One module stays (D29).
- **Wire forms**: the ordinary sequent, outcome, derivation and session
  state as JSON at a new wire level, raised as `wire.rs` says, each an
  addition.

## Requirements

1. **The library** (session 49a): the goal state, its operations and its
   JSON (`serialize`), behind the features its parts need, with every
   bound the linear session keeps (`Limits`, the stop poll, no recursion
   on depth, a size estimate before building), and one focused test per
   behaviour: a finished state checks, undo restores, a rule refused on a
   goal it does not fit says why, an old JSON form still reads.
2. **The command** (session 49b): `linlog interact` takes the ordinary
   logics (`--logic classical|intuitionistic|minimal`, the names the
   ordinary layer has), with the session commands the linear session has
   where they mean the same, the derivation printed as an LK or LJ tree
   and exported as the ordinary layer exports; the manual's chapter and
   the guide for AI models (`skills/linlog/`) in the same commits, every
   example run by `cli/tests/readme.rs`.
3. **The bindings** (49b): the calls the page needs for an ordinary tab
   (start, view, rules, apply, undo, close, hint, draw, export), each
   stateless JSON in and out as the linear calls are, and `about`
   listing the suites with their names and parameters; their tests under
   Node.
4. **The page** (session 49c, in the client's repository with its own
   CLAUDE.md): the registry, the Ordinary segment switched on with
   classical, intuitionistic and minimal, the stage drawing the ordinary
   derivation, the bottom band's exports, a lesson's worth of examples
   in the sequent field's menu, and the client's README brought up to
   date (it still says 425 problems where the store holds 452, 392
   provable where it holds 407). The client's open entries of
   `plan/later.md` go with it ("Follow-ups: the web client" and
   "Follow-ups: interactive proving"): the phone's goal bar and dock
   wrapping their last button since "Cut", the tour's flaky stop over a
   39-px formula button, a replayed classical session showing its cut
   one-sided in the two-sided view, and the compact net's edges starting
   below the literal. Learn and Practice stay linear in this step; they
   are keyed by suite in step 63.

## Constraints

- **The propositional linear case does not move** (D17): the target sets
  of step 47 (`bench/targets/47.csv`, `net-47.csv`) compared with
  `summary --against … --counters`, every decided row's counters equal,
  and the ratchet within its ceilings.
- The web module's size against step 47's figure, said in 49b's report;
  a growth beyond a few percent is explained.
- The client's lock moves to the commit of 49b after the planning
  session has pushed it.

## How the step runs

Three sessions on Opus 5.5 at `high` (49a the library, 49b the command
and the bindings, 49c the page), each reviewed by the planning session
before the next. In 49a a fresh-context reviewer on Opus 5.5 at `high`
checks that no rule application can yield a state whose finished
derivation the checker rejects, and that every bound holds, reporting
only what breaks correctness or a stated requirement. 49c uses the
`frontend-design` plugin as step 38 did. Deliverables: the commits, the
client's commits, `plan/reports/49-ordinary-interactive.md` with each
session's section and "What the next sessions must know" (the registry's
shape for steps 56, 62 and 65, and what the goal state copied from the
linear session).
