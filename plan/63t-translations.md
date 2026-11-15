# Step 63t: transformations of ordinary logic, checked

You are working in deduxit (`deduxit/deduxit`, the checkout
`~/Projects/own/deduxit`) and, for the page, in the web client
(`../deduxit.github.io`). CLAUDE.md applies throughout: jj only (never
git), thematic commits as soon as a unit is done, doc comments on every
item, the verification table, no pushing. This prompt was written on
2026-11-15 by the planning session from the author's request of that
day ("transformations, like various transformations of classical into
minimal logic"), placed in 0.5.0 and started beside step 60, since it
needs only the ordinary suite of 0.4.0.

Read `.claude/rules/core-ordinary.md` whole, the reports of steps 54,
54s, 55 and 56 (the methods, the countermodel, the tableau),
`plan/README.md`'s rows 63t and 63g (the workspace graph will hold
these transformations as edges), and `.claude/rules/writing.md`.

## Goal

Two transformations of ordinary sequents, each with its result checked,
in the library, the command, the bindings and on the page:
- **the negative translations** of classical logic into intuitionistic
  and minimal logic: Kolmogorov's, Gödel and Gentzen's, Kuroda's, and
  Glivenko's theorem for propositional logic;
- **Craig interpolants** read off derivations of LK and LJ.

## Fixed now

- **A translation is a function on the arena**, pattern-table style as
  `translate.rs` is: one table per translation, never a recursion on a
  formula (`core.md`: nothing recurses over a formula). Its output is an
  ordinary sequent of the target logic, printed and written as any.
- **Each translation's claim is checked, not assumed**: the sequent is
  decided classically, its translation in the target logic, and the two
  verdicts must agree (Kolmogorov's and Gödel and Gentzen's into minimal
  logic, Kuroda's into intuitionistic, Glivenko's: `⊢ ¬¬A` in
  intuitionistic logic exactly when `⊢ A` classically, for
  propositional `A`). Disagreement is a defect (`Error::ReadBack`-like,
  kind defect), never a verdict. The answer shows both verdicts and
  their evidence (derivations or countermodels).
- **Interpolants by Maehara's method** on a derivation of LK or LJ (the
  ordinary suite's, checked first), for a partition of the sequent
  given by the caller: the interpolant `I` with `Γ₁ ⊢ I` (and the
  partition's other half `I, Γ₂ ⊢ Δ`), its atoms common to both halves,
  `⊤`/`⊥` where none. **Checked**: both halves decided and the atom
  condition tested; a derivation with a cut is refused (Maehara needs
  cut-free; LJT's derivations hold cuts, so the interpolant is taken
  from a cut-free one: the embedding's read-back, or the tableau's).
  Bounded as the size estimate bounds a derivation (an interpolant may
  grow with the derivation).
- **The command**: subcommands or flags by the command's shape and
  current CLI idiom (`seq translate --to minimal --by kolmogorov`, an
  `interpolate` with the partition), text and JSON at a new wire level;
  the manual's chapter with real invocations, the guide, the changelog.
- **The bindings and the page**: the translations and the interpolant
  as calls; on the page a translation shown beside the sequent with both
  verdicts, under the author's rule **not overwhelming** (nothing new
  in a single sequent's path unless the user asks for a translation).
  The client's part may be a session of its own (63t-b).
- **Not here**: the workspace graph (63g) and first-order versions (77).

## How the step runs

Two to three sessions on Opus 5.5 at `high`: the library and the command
(with the bindings), then the page in the client. A fresh-context
reviewer on Fable 5.2 per session checks that every claimed agreement
is computed and that an interpolant's three conditions are checked.
Deliverables: the commits, `plan/reports/63t-translations.md`.
