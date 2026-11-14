# Getting started

> **dēdūxit** [deːˈduːk.sɪt] *Latin, verb*: "it has derived", the perfect of *dēdūcō*, to lead down, derive.

deduxit decides whether a sequent of linear logic is provable, and shows
the proof or says why there is none. It is a command, `deduxit`, and a
Rust library, the crate `deduxit`. It also handles intuitionistic linear
logic, first-order and cyclic linear logic, the Lambek calculus and
ordinary propositional logic. The [web page](https://deduxit.github.io/)
proves step by step in the browser with the same library.

This page is a short tour. It assumes you have installed the command ([Install deduxit](installation.md); `nix run
github:deduxit/deduxit -- prove "A |- A"` runs it without installing).
This manual describes the `main` branch; a release's manual is the
`docs/` directory of its tag.

## Your first proof

A sequent `A, A -o B |- B` says: from `A` and "`A` implies `B`", using
each exactly once, derive `B`. `-o` is linear implication. Ask deduxit
to prove it:

```console
$ deduxit prove "A, A -o B |- B"
provable (MLL, classical, focus engine)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B
```

The first line is the verdict. In brackets are the fragment of linear
logic the sequent uses (MLL: only the multiplicative connectives), the
logic (classical linear logic by default) and the engine that searched.
Below it is the proof, a derivation tree read from the leaves down.
Every proof deduxit reports has passed its proof checker.

The tree is written one-sided, the standard form for classical linear
logic: every formula moves to the right of `⊢`, and a formula on the
left becomes its negation `~`. So `⊢ ~A, A` is the axiom `A ⊢ A`. To see
the tree with the sides you wrote, add `--classical-sides two`:

```console
$ deduxit prove --classical-sides two "A, A -o B |- B"
provable (MLL, classical, focus engine)
───── ax   ───── ax
A ⊢ A      B ⊢ B
──────────────── ⊸L
  A, A ⊸ B ⊢ B
```

Quote the sequent, so that the shell passes it as one word. The ASCII
connectives are `*` (tensor, ⊗), `par` (⅋), `&` (with), `+` (plus, ⊕),
`-o` (⊸), `~` (negation), `!` and `?`; [The syntax](syntax.md) lists
them all.

## Your first unprovable sequent

In linear logic every hypothesis must be used exactly once. So `A, B |-
A` has no proof:

```console
$ deduxit prove "A, B |- A"
unprovable (MLL, classical, focus engine): B on the left has no B on the right to meet in an axiom; with -a (affine), an unused hypothesis is allowed
```

The reason says what is wrong: `B` is never used. The note names
`-a`, affine mode, where a hypothesis may go unused:

```console
$ deduxit prove -a "A, B |- A"
provable (MLL, classical affine, focus engine)
  ─────── ax
  ⊢ ~A, A
─────────── wk
⊢ ~A, ~B, A
```

The `wk` step (weakening) drops the unused `~B`.

The exit status tells a script the verdict: 0 provable, 1 unprovable,
3 unknown, 2 an error.

```console
$ deduxit prove -q "A |- B"; echo "exit $?"
unprovable (MLL, classical, focus engine): A on the left has no A on the right to meet in an axiom
exit 1
```

`-q` prints the verdict line alone. A third answer, "unknown", comes when
a limit stopped the search first, most often the time limit of two
seconds. Its line names the flag that gives the search more room;
[Decide a sequent](commands.md) says more.

## Your first intuitionistic proof

`-i` reads the sequent in intuitionistic linear logic: hypotheses on
the left, exactly one formula on the right. The proof is two-sided:

```console
$ deduxit prove -i "A, A -o B |- B"
provable (IMLL, intuitionistic, two-sided engine)
───── ax   ───── ax
A ⊢ A      B ⊢ B
──────────────── ⊸L
  A, A ⊸ B ⊢ B
$ deduxit prove -i "A |- A * A"
unprovable (IMLL, intuitionistic, two-sided engine): A occurs 1 more time on the right than on the left, so they cannot all meet in axioms (an implication's antecedent counts on the other side)
```

`A * A` needs two copies of `A`, and there is one. With `!A`, "as many
`A` as you like", the sequent is provable:

```console
$ deduxit prove -q "!A |- A * A"
provable (MELL, classical, horn engine)
```

## Export the proof

`--format` writes the proof as LaTeX, Typst, SVG, PNG, PDF, a Rocq
certificate or JSON. `--output` writes it to a file, in the format its
extension names:

```console
$ deduxit prove --format latex "A, A -o B |- B"
% provable (MLL, classical, focus engine)
\begin{prooftree}
\infer0[$\mathrm{ax}$]{\vdash A^\bot, A}
\infer0[$\mathrm{ax}$]{\vdash B^\bot, B}
\infer2[$\otimes$]{\vdash A^\bot, A \otimes B^\bot, B}
\end{prooftree}
$ deduxit prove --output proof.svg "A, A -o B |- B"
```

The LaTeX uses the `ebproof` package; `--standalone` writes a whole
document. [Export a proof](output-formats.md) covers every format.

## Check a proof

The JSON form is a proof file. `deduxit check` verifies it with the
checker alone, without searching:

```console
$ deduxit prove --format json "A, A -o B |- B" > proof.json
$ deduxit check proof.json
valid proof of ⊢ ~A, A ⊗ ~B, B (classical)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B
```

## Where to go next

- **A task:** the how-to guides, from [Decide a sequent](commands.md)
  and [Avoid the common mistakes](mistakes.md) to
  [Prove step by step](interactive.md), [Check a certificate in
  Rocq](rocq.md) and [Use the library](library.md).
- **A flag:** `deduxit <command> -h` lists the common options,
  `deduxit <command> --help` every one. [Limits, defaults and exit
  statuses](options.md) and [The syntax](syntax.md) are the reference.
- **How it works:** [How deduxit searches](engines.md) and
  [Architecture](architecture.md).
- **The library's API:** [docs.rs/deduxit](https://docs.rs/deduxit) for
  the releases, [deduxit.github.io/deduxit](https://deduxit.github.io/deduxit/)
  for `main`.
- **For AI models:** the guide
  [`skills/deduxit/`](https://github.com/deduxit/deduxit/tree/main/skills/deduxit),
  served as [llms.txt](https://deduxit.github.io/deduxit/llms.txt) and
  [llms-full.txt](https://deduxit.github.io/deduxit/llms-full.txt).
- **What changed:** the
  [changelog](https://github.com/deduxit/deduxit/blob/main/CHANGELOG.md).
