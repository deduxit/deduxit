# Step 50: proofs read back as lambda-terms

You are working in the linlog repository (`sequentium/sequentium` after
step 48; the crate and the command are `linlog`). CLAUDE.md applies
throughout: jj only (never git), thematic commits as soon as a unit is
done, doc comments on every item, the verification table, no pushing.
The step has two sessions, the second in the web client's repository.
This prompt was written on 2026-11-08 by step 46's second session, from
the author's decisions of that day. It is complete; the planning session
adds the facts of step 49 at its review.

Read `plan/notes/research/next/lambda-calculi.md` (section 2 and
candidate 1), `scope-types-higher.md` (types-higher-2 and -8) and
`scope-proof-theory.md` (proof-theory-11), `.claude/rules/core-proofs.md`
and the rules of the two-sided surface and of the ordinary layer
(`core-ordinary.md`), the reports of steps 40 and 41, and `plan/README.md`'s
D13, D15, D16 and D17.

## What step 49 left you (added at its review, 2026-11-08)

- **The ordinary layer proves step by step**: `ordinary::Interactive`
  (feature `interactive`), its JSON at wire level 8 (`wire::ORDINARY`),
  and `ViewOptions::structural` (admissible by default: weakenings and
  contractions hidden where admissible). In LJ every goal keeps one
  formula right of ⊢, `Γ ⊢` read as `Γ ⊢ ⊥`, and a shared →L or ¬L gives
  both premises every hypothesis through hidden contractions. An LJ
  derivation's read-back must therefore read through those structural
  steps (contraction duplicates a variable's uses, weakening leaves one
  unused).
- **The command**: `linlog interact --logic
  classical|intuitionistic|minimal`, and `--structural` on `prove` and
  `interact`. Reports: `plan/reports/49-ordinary-interactive.md`.
- **The bindings**: the nine `ordinary_*` calls, and `about`'s `suites`
  (linear with `mode`, ordinary with `logic`). A new call for the term
  is a row of both suites' calls where it applies (`web.md`).
- **The page**: a suite registry (`src/lib/suites.ts` in the client),
  links at `v=2` and the shelf at version 2. 50b adds the term to the
  bottom band for both suites through the registry.
- **The web module** after 49b is 7 148 412 bytes raw (+3.4 % on step
  47's 6 913 020). Say 50's growth against both figures.

## Goal

Every intuitionistic proof linlog finds or checks shows its program: an
ILL proof (one-sided term with step 40's recorded sides) is read back as
a linear lambda-term, an LJ derivation of the ordinary layer as a simply
typed lambda-term, printed, drawn beside the derivation, exported, and
given on the page. A student sees that a proof of `a ⊸ b ⊸ a ⊗ b` is
`λx. λy. (x, y)` and that an LJ proof of `(a → b) → (b → c) → a → c` is
composition; a model asked for "a program of this type" gets one. It is
the cheapest item of the lambda-calculi research, the bridge to step 71's
checker, and needs nothing new in the search.

## Fixed now

- **A read-back, not a new trusted component**: the term is computed
  from a proof that the checker has accepted; it is printed, never
  trusted. A round trip is the test that matters: the term's type,
  inferred by a small test-only checker (or by step 71's checker once it
  exists), is the proved sequent.
- **The calculus is named and sourced**: the term syntax for `!`, the
  additives and the units follows one published calculus (Benton,
  Bierman, de Paiva and Hyland's linear lambda calculus, or Abramsky's;
  the note gives both) chosen by a research sub-agent's reading of the
  papers and stated in the manual. LJ derivations read back to the
  simply typed lambda calculus with products, sums, unit and empty type.
- **What has no term is refused with its reason**: classical one-sided
  proofs and the classical modes (no lambda-term; a process calculus
  would be another step), Mix, the cyclic and Lambek modes (their own
  calculi), quantified proofs (first-order quantifiers over terms are
  not type quantifiers; the place is kept, D17). Affine proofs read back
  to the affine calculus (a variable may go unused).
- **Cuts are redexes**: a proof with cuts reads back as a term with
  `let` or a redex, so that step 41's cut elimination is visible as
  reduction; equal normal forms after elimination are a test.
- **Configuration** (D15, D16): one options value for the term's syntax
  (the neutral notation by default; a second notation such as Haskell's
  `LinearTypes` only if the session finds it cheap), with serde and a
  flag; nothing hidden in a constant.

## Requirements

1. **The library** (50a): the read-back for ILL proofs and for LJ
   derivations, the printer, the term in the JSON forms and in the
   LaTeX, Typst and SVG exports where a derivation is exported (a term
   beside the conclusion, or at each node as an option), every bound the
   other walks keep (no recursion on depth, `Limits`, the stop poll).
2. **The command** (50a): `prove`, `check` and `interact` print the term
   when asked (one flag or output format, the name the session chooses
   and the manual shows), with a real example in the manual and the guide
   for AI models, run by `cli/tests/readme.rs`.
3. **The page** (50b, in the client's repository): the term of a
   finished intuitionistic proof shown in the bottom band and copied as
   text, for the linear and, after step 49, the ordinary suite; the
   bindings' call and its tests under Node.

## Constraints

The search does not change: the target sets' counters equal against
step 47's oracles and the ratchet within its ceilings. The web module's
growth said against step 47's figure.

## How the step runs

Two sessions on Opus 5.5 at `high`; in 50a a research sub-agent on
Sonnet 5.5 at `medium` reads the two calculi's papers and reports the
term syntax for each rule with links (the sources rule of `conduct.md`),
and a fresh-context reviewer on Opus 5.5 at `high` checks the read-back
rule by rule against the chosen calculus, reporting only what breaks it.
Deliverables: the commits, the client's commit, `plan/reports/50-lambda-terms.md`
with "What the next sessions must know" (the term data's shape, which
step 71's checker takes).
