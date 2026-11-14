# Prove in the Lambek calculus

The Lambek calculus is a logic of word order, used for the grammar of
natural language. Its sequents `A₁, …, Aₙ ⊢ B` are intuitionistic (one
formula on the right), and the hypotheses keep the order they are
written in. `--lambek` decides it:

```console
$ deduxit prove --lambek "c / b, a, a \ b |- c"
provable (IMLL, Lambek, essential engine)
───── ax   ───── ax
b ⊢ b      c ⊢ c
──────────────── /L   ───── ax
  c / b, b ⊢ c        a ⊢ a
  ───────────────────────── \L
     c / b, a, a \ b ⊢ c
$ deduxit prove --lambek "a, b |- b * a"
unprovable (IMLL, Lambek, essential engine): the search was exhaustive
```

The formulas are built from atoms with `*` and two divisions:
- `A \ B` gives `B` from an `A` on its left, as `A -o B` does;
- `B / A` gives `B` from an `A` on its right.

The derivations are two-sided, with the divisions' rules. The text is
read as `--cyclic` reads it ([Prove in cyclic linear logic](cyclic.md)),
since the calculus is the intuitionistic part of cyclic linear logic.
`seq print --lambek` shows the sequent as the calculus reads it:

```console
$ deduxit seq print --lambek "b / a, a |- b"
b / a, a ⊢ b
```

## Allow empty antecedents

In the Lambek calculus every sequent of a proof has a hypothesis.
`--empty-antecedents` allows sequents without one, which gives the
calculus L*:

```console
$ deduxit prove --lambek "e |- e * (a \ a)"
unprovable (IMLL, Lambek, essential engine): the search was exhaustive
$ deduxit prove --lambek --empty-antecedents "e |- e * (a \ a)"
provable (IMLL, Lambek with empty antecedents, essential engine)
            ───── ax
            a ⊢ a
───── ax   ─────── \R
e ⊢ e      ⊢ a \ a
────────────────── ⊗R
 e ⊢ e ⊗ (a \ a)
$ deduxit prove --lambek "|- a \ a"
error: not an intuitionistic sequent: the sequent has no formula left of ⊢, which the Lambek calculus without empty antecedents needs
```

In a batch ([Decide many sequents at once](batch.md)) and in problem
files, the modes are named `lambek` and `lambek-star`.

## How the calculus is decided

The essential engine decides the calculus. It searches for an axiom
linking that is a planar essential net:
- no two links cross in the cyclic order;
- every implication dominates its antecedents;
- without empty antecedents, the body of every division has a hypothesis
  of its own.
