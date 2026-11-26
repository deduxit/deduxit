# Step 84: untyped lambda calculi, a β stepper and principal types

You are working in deduxit (`deduxit/deduxit`) and its web client
(`deduxit.github.io`). CLAUDE.md applies throughout. This prompt was
written on 2026-11-26 by the planning session, after the author's review
of the redesigned page (`plan/79-page-redesign.md`, "The author's
review").

Read `plan/later.md`, "Untyped lambda calculi as a second axis (the
redesign, 2026-11-25)", whole; `plan/notes/research/next/lambda-calculi.md`
(the kept items lambda-calculi-4 and lambda-calculi-5);
`plan/71-typed-lambda.md`, its report and `plan/notes/typed-lambda-design.md`;
the rules file of the lambda-terms module and of the bindings; and in
the client the guide's board D12 and `.claude/rules/web.md`.

## Goal

`lambda::Calculus` has three calculi, all Church-style (linear, affine,
simple). The author's yes (2026-11-26, "do all of them") adds typing as
a second axis beside the use of variables:
- read and write a term whose `λ` carries no type;
- the untyped calculi, unrestricted, linear and affine: check the use of
  a term's variables, with the refusal's reason;
- a β stepper, for typed and untyped terms: one step at a time, naming
  each step's redex, within a bound, ending in "normal form" or "not
  finished at the bound" (never "no normal form": an untyped
  unrestricted term need not normalise);
- principal type inference (lambda-calculi-4): an untyped term's
  principal simple type, by unification, or the reason it has none;
- the page: board D12's "Untyped" toggle active, the stepper and the
  principal type as the guide draws them.

System F stays a possibility (the author's "maybe", in the same
`later.md` section); not this step.

## Fixed now

- The shape is the most idiomatic, current one: names and outputs as
  current tools and textbooks have them, from their current
  documentation, not memory; say in the report what you took.
- The stepper's strategy is named (normal order by default, which
  finds a normal form when one exists); every bound is a setting with a
  refusal past it, as the module's other searches have.
- A principal type is checked: the term typed at it by the existing
  checker, and a test that every type the checker accepts for the term
  is an instance of it.
- Library, bindings (rows in `plan/web/README.md`, tests), the CLI where
  the lambda commands live (the manual's chapter, the guide for AI
  models), the changelog's `[Unreleased]`, `breaks 0.8.0`.

## How the step runs

One session, Opus 5.5 at `high`, `fresh-reviewer` (Fable 5.2 at
`medium`). The library and the bindings first, "84: deduxit ready" to
the planning session, which pushes deduxit's `main` after its review and
the full check; then the client's input moved and the page, with
Playwright tests. The report, `plan/reports/84-untyped-lambda.md`.
