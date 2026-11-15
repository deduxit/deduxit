# Translate a sequent and find its interpolant

deduxit turns a sequent of [ordinary logic](ordinary-logic.md) into
another and checks what the change promises before it prints the
result. Two changes are built: the negative translations of classical
logic into intuitionistic and minimal logic, and Craig interpolants.

## Translate classical logic into a weaker one

A negative translation turns a classical sequent into one that a weaker
logic proves exactly when classical logic proves the first. `seq
translate` prints the translation, decides the sequent in classical logic
and its translation in the weaker logic, and prints both verdicts:

```console
$ deduxit seq translate 'a \/ ~a'
Kolmogorov's translation into minimal logic: ⊢ ¬¬(¬¬a ∨ ¬¬¬¬¬a)
classical logic: valid, a checked derivation of LK (3 inferences)
minimal logic: valid, a checked derivation of LJ (65 inferences)
The verdicts agree, as Kolmogorov's translation claims.
```

`--by` names the translation:

| `--by` | what it does | the logic it reaches |
|---|---|---|
| `kolmogorov` (the default) | `¬¬` before every subformula | minimal logic |
| `godel-gentzen` | `¬¬` before every atom, and `A ∨ B` as `¬(¬A ∧ ¬B)` | minimal logic |
| `kuroda` | `¬¬` before every formula of the sequent | intuitionistic logic |
| `glivenko` | `¬¬` before the formula right of `⊢` | intuitionistic logic |

Several formulas right of `⊢` are translated as their disjunction. In
propositional logic Kuroda's translation adds no more than `¬¬` before
each formula: the rest of it acts on quantifiers. Glivenko's theorem
says that `⊢ A` holds classically exactly when `⊢ ¬¬A` holds
intuitionistically:

```console
$ deduxit seq translate --by glivenko '~~a -> a'
Glivenko's translation into intuitionistic logic: ⊢ ¬¬(¬¬a → a)
classical logic: valid, a checked derivation of LK (4 inferences)
intuitionistic logic: valid, a checked derivation of LJ (16 inferences)
The verdicts agree, as Glivenko's translation claims.
```

`--to intuitionistic` translates by Kolmogorov's or Gödel and Gentzen's
translation into intuitionistic logic, which proves what minimal logic
proves. Kuroda's and Glivenko's translations do not reach minimal logic,
and `--to minimal` with them is an error.

A sequent that is not valid stays not valid, and each verdict comes
with the countermodel that shows it:

```console
$ deduxit seq translate --by godel-gentzen 'a |- b'; echo "exit $?"
Gödel and Gentzen's translation into minimal logic: ¬¬a ⊢ ¬¬b
classical logic: not valid, a checked Kripke model refutes it:
  world 0 (the model's world): forces a
minimal logic: not valid, a checked Kripke model refutes it:
  world 0 (the model's world): forces nothing; below 1
  world 1: forces a; below 2
  world 2: forces a
The verdicts agree, as Gödel and Gentzen's translation claims.
exit 1
```

The exit status is 0 where both verdicts are valid, 1 where neither is
and 3 where a search stopped at a limit. Verdicts that differ would be a
defect of deduxit; the command reports it as an error, exit status 2.

## Find an interpolant

Split a valid sequent into two halves. An interpolant is a formula `I`
that the first half proves, that proves the second half, and whose atoms
are in both halves. For `A ⊢ B` that is `A ⊢ I` and `I ⊢ B`:

```console
$ deduxit interpolate 'a /\ b |- b \/ c'
interpolant: b
a ∧ b ⊢ b: valid, a checked derivation of LK (3 inferences)
b ⊢ b ∨ c: valid, a checked derivation of LK (3 inferences)
```

By default the formulas left of `⊢` are the first half and those right
of it the second. `--first` names the first half's formulas instead,
`L0` being the first formula left of `⊢` and `R0` the first right of it.
In classical logic the first half may hold formulas right of `⊢`: the
interpolant then goes right of `⊢` in the first half.

```console
$ deduxit interpolate --first L0,R0 'a, b |- a /\ b, c'
interpolant: ¬b
a ⊢ a ∧ b, ¬b: valid, a checked derivation of LK (6 inferences)
¬b, b ⊢ c: valid, a checked derivation of LK (3 inferences)
```

`--logic` names the logic, classical by default. In intuitionistic and
minimal logic the formula right of `⊢` is always in the second half:

```console
$ deduxit interpolate --logic intuitionistic --first L1 'a, a -> b |- b'
interpolant: a → b
a → b ⊢ a → b: valid, a checked derivation of LJ (1 inference)
a → b, a ⊢ b: valid, a checked derivation of LJ (5 inferences)
```

deduxit reads the interpolant off a derivation without cuts by
Maehara's method, and checks it before it prints it: it decides both
halves again and compares the atoms. A sequent that is not valid has no
interpolant:

```console
$ deduxit interpolate 'a |- b'; echo "exit $?"
classical logic: not valid, a checked Kripke model refutes it:
  world 0 (the model's world): forces a
A sequent that is not valid has no interpolant.
exit 1
```

The exit status is 0 for a checked interpolant, 1 for a sequent that is
not valid, 3 where a search stopped at a limit (by default after 2 s,
`--timeout`) and 2 for an error.

## JSON output

`--format json` writes the whole answer as one document at wire level
16. `seq translate` writes `{"version", "deduxit", "translation",
"target", "agreement", "sequent", "translated", "classical",
"decided"}`: both sequents, both outcomes with their derivations or
countermodels, and `agreement`, which is `valid`, `not_valid` or
`unknown`. `interpolate` writes `{"version", "deduxit", "logic",
"partition", "outcome", "interpolant"}`: the positions of the first half,
the sequent's outcome, and the interpolant's `text` with its `first` and
`second` halves, each a `sequent` with its `outcome`.

In a program, `deduxit::ordinary::negative::compare` and
`deduxit::ordinary::interpolant::interpolate` return the same answers,
and the web bindings' `ordinary_translate` and `ordinary_interpolate`
answer the same JSON.
