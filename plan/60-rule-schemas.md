# Step 60: rules as data, a generic derivation and its checker

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`).
CLAUDE.md applies throughout: jj only (never git), thematic commits as
soon as a unit is done, doc comments on every item, the verification
table, no pushing. This prompt was written on 2026-11-08 by step 46's
second session, from the author's decisions of that day, and is finished
at the review of step 59: it says what is fixed.

Read `plan/notes/research/next/custom-rules.md` whole, and the sections
5 of `natural-deduction.md`, `tableaux.md` and `modal-logics.md` (what
each method asks of a generic derivation), `structure.md` (structure-4
and structure-5), the report of 46 (session 46a, "Custom rules", and
"What the next sessions must know"), `.claude/rules/core-proofs.md` (the
linear checker's shape and bounds), and `plan/README.md`'s D15 to D17,
D25 and D28.

## Goal

The layer every later method stands on, and the author's wish for
custom rules (D28): a calculus given as data, a derivation whose nodes
name its rules, and a checker that decides whether a derivation is
correct in that calculus. It sits beside the fast linear checker,
engines and Rocq library, which stay as they are; it is never a rewrite
of them.

## Fixed now (D28 and step 46's research)

- **Rules as data, JSON first**: connectives with arity and print forms,
  the structural discipline per side (set, multiset or sequence), rules
  as premises and conclusion over schematic formulas and contexts, and a
  fixed menu of side conditions: a context restricted to formulas of one
  connective (as `!Γ`), the freshness of an eigenvariable, labels and
  relational atoms. A text syntax may follow the JSON, not replace it.
- **The derivation carries, from its first design**, what natural
  deduction, tableaux and labelled modal calculi need (46a, "What the
  next sessions must know"): hypothesis nodes with an identity and a
  discharge link from the closing rule to its leaves (natural deduction),
  a scope tree (Fitch), closure of a branch by complementary literals
  (tableaux), labels and relational atoms as a sort (modal). A method
  that would need the type changed later is a failure of this step.
- **The checker by matching**: first-order matching of the schemas plus
  the splitting of a context by its discipline; decidable, and bounded
  as the linear checker is (a size estimate before building, no
  recursion on depth, `Limits`, the stop poll, integers that cannot
  wrap). Its verdict reads "valid in the calculus given": no
  certificate, since the user gives no semantics, and no search (D28;
  bounded search is kept in `later.md` until asked).
- **Calculi shipped as data**: LK and LJ as rule files the checker
  accepts on the ordinary layer's derivations (the cross-check that the
  generic checker and `ordinary::Derivation::check` agree), and the
  linear calculus as a file whose derivations agree with the linear
  checker's on a sample.
- **Where it lives**: a module of the `deduxit` crate until step 67
  decides the kernel (D25); what it copies is recorded for that step.

## How the step runs

Two to three sessions. The first, which settles the rule language and
the derivation type (decisions every later method builds on), on Fable
5.2 at `high` ("Why these models and efforts": a design session); the
rest on Opus 5.5 at `high`. The checker is reviewed by the panel of step
26's prompt (argument on Fable 5.2 at `high`). Deliverables: the
commits, the manual's chapter, the guide's lines, the JSON Schema of the
rule language, `plan/reports/60-rule-schemas.md`.

## What steps 54 to 59 left you (the review of 59, 2026-11-15)

- **The ordinary suite grew methods and artifacts**: the SAT method
  (classical, intuitionistic, minimal), LJT, the truth table and the
  signed tableau (56a: `Tableau::of` reads any LK derivation as a
  tableau; built by its method in the LK goal state). The generic
  derivation's closure of a branch by complementary literals should be
  able to state 56a's tableau as a rule file: that is a cross-check
  worth having beside LK's and LJ's.
- **The Kripke model is shared at the root** (`deduxit::kripke`, data
  only, each suite checking it by its own rules; 55b), with its JSON at
  wire level 13. Labels and relational atoms of the modal calculi
  (step 65, 66) should be able to name its worlds and arrows.
- **Wire levels** stand at 14 (11 the Horn engine's basis, 12 the
  ordinary outcome, 13 the Kripke model, 14 the table and the tableau).
  The rule language and the generic derivation take new levels by
  `core-sequents.md`'s rules.
- **The workspace graph (63g) will hold these artifacts**: a rule file
  and a derivation in it are immutable values with a JSON form, so that
  the graph can store, cache and share them without a format of its own.
  Design them so.
- **Translations (63t) and the normal forms (70)** are rewrites; the
  rule language need not express them, but its connective table should
  let a rewrite name the formulas it produces.
- **The text everyone reads follows `.claude/rules/writing.md`**
  (2026-11-09), and a new interactive feature is **not overwhelming**
  (the author, 2026-11-15): the single-task path stays as simple as
  before.
- **CI**: Kani's harnesses run in `kani.yml`, not in `nix flake check`;
  a checker of integers gains harnesses there (`core-proofs.md`).
