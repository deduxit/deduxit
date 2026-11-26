# Step 85: System F, checked

You are working in deduxit (`deduxit/deduxit`) and its web client
(`deduxit.github.io`). CLAUDE.md applies throughout. This prompt was
written on 2026-11-26 by the planning session at the author's word: "system
F should also be done. Maybe type inference is not necessary, but type
checking should be doable."

Read `plan/later.md`, "Untyped lambda calculi as a second axis (the
redesign, 2026-11-25)", its paragraph "System F, a possibility beside
it"; `plan/notes/research/next/lambda-calculi.md` (the kept item
lambda-calculi-6, D30: Church-style checking only); `plan/71-typed-lambda.md`,
`plan/84-untyped-lambda.md` and their reports; the rules file of the
lambda-terms module and of the bindings; in the client the guide's board
D12 and `.claude/rules/web.md`.

## Goal

System F as the third value of the typing axis step 84 makes (simple
types, System F, untyped), beside the use of variables (unrestricted,
linear, affine):
- types with `∀` over type variables, and terms with type abstraction
  and type application written (Church style), read and written;
- type checking: a term checked against its type, or its type computed
  from the annotations it carries, with the refusal's reason; in the
  linear and the affine calculus the use of term variables checked as
  for the simple types;
- the typing derivation built and drawn as the simple calculi's are,
  and exported (LaTeX, Typst, SVG);
- β-reduction with type β (`(Λα. t) [A]`) in 84's stepper;
- the page: "System F" on D12's typing axis, a term checked and drawn.

## Fixed now

- **No type inference** and no inhabitation search: type inference is
  undecidable (Wells), and the author does not ask for it. A term
  without its type annotations is refused with that reason.
- The shape is the most idiomatic, current one: the notation and names
  as current textbooks (Girard's *Proofs and Types*, Pierce's *Types and
  Programming Languages*) and tools have them, from current sources, not
  memory; say in the report what you took.
- Capture-avoiding substitution of types, with tests that hold it
  (a type variable bound under `∀` never captured).
- Library, bindings (rows in `plan/web/README.md`, tests), the CLI where
  the lambda commands live (the manual's chapter, the guide for AI
  models), the changelog's `[Unreleased]`, `breaks 0.8.0`.

## How the step runs

One session, Opus 5.5 at `high`, `fresh-reviewer` (Fable 5.2 at
`medium`), after step 84 (the same module and the same axis). The
library and the bindings first, "85: deduxit ready" to the planning
session, which pushes deduxit's `main` after its review and the full
check; then the client's input moved and the page, with Playwright
tests. The report, `plan/reports/85-system-f.md`.
