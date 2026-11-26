# Getting started

> **dēdūxit** [deːˈduːk.sɪt] *Latin, verb*: "it has derived", the perfect of *dēdūcō*, to lead down, derive.

deduxit is a collection of logic suites. It decides whether a sequent
is provable, and shows the proof or says why there is none. It is a
command, `deduxit`, and a Rust library, the crate `deduxit`. Its suites
are:

- [ordinary logic](ordinary-logic.md): classical, intuitionistic and
  minimal, propositional and
  [first-order](ordinary-logic.md#first-order-sequents);
- [linear logic](commands.md): classical and intuitionistic, affine,
  [first-order](first-order.md), [cyclic](cyclic.md) and the [Lambek calculus](lambek.md);
- [modal logic](modal-logic.md): the fifteen propositional systems from
  K to S5;
- [lambda-terms](terms.md): a proof read as a program, the terms you
  write checked, and a type's terms found;
- [calculi given as rule files](rules.md), natural deduction among
  them.

The [web page](https://deduxit.github.io/) proves step by step in the
browser with the same library.

This page is a short tour, one suite at a time. It assumes you have installed the command ([Install deduxit](installation.md); `nix run
github:deduxit/deduxit -- prove "A |- A"` runs it without installing).
This manual describes the `main` branch; a release's manual is the
`docs/` directory of its tag.

## Your first proof

A sequent `A, A -> B |- B` says: from `A` and "`A` implies `B`", derive
`B`. `--logic` names the logic to read it in, here intuitionistic
logic:

```console
$ deduxit prove --logic intuitionistic "A, A -> B |- B"
valid (intuitionistic logic by the SAT solver)
───── ax   ──────── ax
A ⊢ A      B, A ⊢ B
─────────────────── →L
   A, A → B ⊢ B
```

The first line is the verdict. In brackets are the logic and the method
that decided it. Below it is the proof, a derivation tree read from the
leaves down. Every proof deduxit reports has passed its proof checker.

Quote the sequent, so that the shell passes it as one word. The ASCII
connectives are `->` (→), `/\` (∧), `\/` (∨), `~` (¬) and `<->` (↔);
[Decide ordinary logic](ordinary-logic.md) has the others.

## Your first countermodel

In intuitionistic logic `A \/ ~A` is not valid, and deduxit shows why
with a Kripke countermodel:

```console
$ deduxit prove --logic intuitionistic "|- A \/ ~A"
not valid (intuitionistic logic by the SAT solver): a checked countermodel refutes it
world 0 (the model's world): forces nothing; below 1
world 1: forces A
```

A Kripke model is a set of worlds, each forcing some atoms, and a world
forces what every world below it forces. At world 0, `A` is not yet
true, and `~A` fails since `A` holds at world 1 above it. [Decide
ordinary logic](ordinary-logic.md) has the other logics, the truth table
and the tableau of classical logic, and the proofs.

The exit status tells a script the verdict: 0 valid, 1 not valid,
3 unknown, 2 an error.

```console
$ deduxit prove -q --logic intuitionistic "|- A \/ ~A"; echo "exit $?"
not valid (intuitionistic logic by the SAT solver): a checked countermodel refutes it
exit 1
```

`-q` prints the verdict line alone. A third answer, "unknown", comes when
a limit stopped the search first, most often the time limit of two
seconds. Its line names the flag that gives the search more room;
[Decide a sequent](commands.md) says more.

## Linear logic

Without `--logic`, `prove` reads linear logic. There every hypothesis
must be used exactly once, and `-o` is linear implication:

```console
$ deduxit prove "A, A -o B |- B"
provable (MLL, classical, focus engine)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B
```

In brackets are the fragment of linear logic the sequent uses (MLL:
only the multiplicative connectives), the logic (classical linear logic
by default) and the engine that searched. The tree is written
one-sided, the standard form for classical linear logic: every formula
moves to the right of `⊢`, and a formula on the left becomes its
negation `~`. So `⊢ ~A, A` is the axiom `A ⊢ A`. To see the tree with
the sides you wrote, add `--classical-sides two`:

```console
$ deduxit prove --classical-sides two "A, A -o B |- B"
provable (MLL, classical, focus engine)
───── ax   ───── ax
A ⊢ A      B ⊢ B
──────────────── ⊸L
  A, A ⊸ B ⊢ B
```

The ASCII connectives are `*` (tensor, ⊗), `par` (⅋), `&` (with), `+`
(plus, ⊕), `-o` (⊸), `~` (negation), `!` and `?`; [The
syntax](syntax.md) lists them all.

Since every hypothesis is used once, `A, B |- A` has no proof:

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

The `wk` step (weakening) drops the unused `~B`. `-i` reads the sequent
in intuitionistic linear logic, with exactly one formula on the right,
and draws the proof two-sided. `!A` means "as many `A` as you like":

```console
$ deduxit prove -i -q "A |- A * A"
unprovable (IMLL, intuitionistic, two-sided engine): A occurs 1 more time on the right than on the left, so they cannot all meet in axioms (an implication's antecedent counts on the other side)
$ deduxit prove -i -q "!A |- A * A"
provable (IMELL, intuitionistic, horn engine)
```

## Modal logic

`--system` names a modal logic, from K to S5. `[]A` is "necessarily
`A`" (□A) and `<>A` "possibly `A`" (◇A). A valid sequent gets a
derivation in a labelled sequent calculus, one line per inference, and
one that is not valid a Kripke countermodel:

```console
$ deduxit prove --system T "[]A |- A"
valid in T: □A ⊢ A, a checked derivation of 3 inferences in G3T
  0. w0 : □A, w0 : A, w0 R w0 ⊢ w0 : A    ax
  1. w0 : □A, w0 R w0 ⊢ w0 : A    L□ 0
  2. w0 : □A ⊢ w0 : A    Ref 1
$ deduxit prove --system K "[]A |- A"
not valid in K: a checked countermodel of 1 world refutes □A ⊢ A at world 0
world 0 (where the sequent fails): forces nothing; sees no world
```

In T every world sees itself, so what holds at every world it sees holds
there too; in K a world may see none. [Decide modal logic](modal-logic.md)
lists the systems.

## Lambda-terms

An intuitionistic proof is also a program, written as a lambda-term.
`--lambda` prints it under the proof:

```console
$ deduxit prove --logic intuitionistic --lambda "|- A -> B -> A"
valid (intuitionistic logic by the SAT solver)
  ──────── ax
  A, B ⊢ A
  ───────── →R
  A ⊢ B → A
───────────── →R
⊢ A → (B → A)
lambda-term: ⊢ λx. λy. x : A → (B → A)
```

`deduxit term` works the other way: `term check` gives the type of a
term you write, and `term inhabit` finds a term of a type:

```console
$ deduxit term check '\f:a -> b. \x:a. f x'
well typed
⊢ λf:(a → b). λx:a. f x : (a → b) → (a → b)
```

[Read a proof as a program](terms.md) says more.

## Calculi of your own

`deduxit check --calculus` checks a derivation against the rules of a
calculus: one you write as a JSON rule file, or a built-in one such as
LK, LJ or natural deduction's NJ. `deduxit interact --calculus` builds
such a derivation step by step. [Check a derivation in your own
rules](rules.md) and [Check a proof in natural
deduction](natural-deduction.md) show how.

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
