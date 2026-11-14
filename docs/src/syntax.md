# The syntax

This page lists how to write a sequent of linear logic, the input of
`prove`, `interact` and `seq`. Ordinary logic, read under
`--logic`, has a syntax of its own: see
[Decide ordinary logic](ordinary-logic.md).

## Sequents

A sequent is two lists of formulas separated by the turnstile, `|-` or
`⊢`: `A, B |- C`. The formulas in a list are separated by commas.
Either list may be empty (`|- A`, `A |-`, `|-`), but the turnstile is
always written. Spaces and line breaks may stand between any two
tokens.

## Connectives and units

| ASCII | Unicode | name | meaning in a few words |
|---|---|---|---|
| `A * B` | `A ⊗ B` | tensor | both, each from its own part of the resources |
| `A \| B` or `A par B` | `A ⅋ B` | par | the dual of tensor; `A -o B` is `~A par B` |
| `A & B` | | with | one of the two, chosen by whoever uses it |
| `A + B` | `A ⊕ B` | plus | one of the two, chosen by whoever gives it |
| `A -o B` | `A ⊸ B` | linear implication | spend an `A` to get a `B` |
| `!A` | | of course | as many copies of `A` as needed, none included |
| `?A` | | why not | the dual of `!` |
| `1` | | one | the unit of tensor |
| `bot` | `⊥` | bottom | the unit of par |
| `top` | `⊤` | top | the unit of with |
| `0` | | zero | the unit of plus |

`par`, `bot` and `top` are keywords only as whole words: `bottom` is an
atom.

## Negation

`~A` and `A^` both negate `A`. `~` is written before its formula and `^`
after it. Two negations cancel: `~~A` is `A`.

## Atoms

An atom is a name: a letter or `_`, then letters, digits and `_`
(Unicode identifiers, so `é` and `α` are letters). A name written with
composed or decomposed accents is one atom. The keywords `par`, `bot`,
`top`, `forall` and `exists` name no atom.

## Precedence and grouping

From the loosest to the tightest:

| connective | groups |
|---|---|
| `-o` | to the right: `A -o B -o C` is `A -o (B -o C)` |
| `+` | to the left |
| `&` | to the left |
| `par`, `\|` | to the left |
| `*` | to the left: `A * B * C` is `(A * B) * C` |
| `~`, `!`, `?`, `^` | bind tightest |

Parentheses group. A prefix `~`, `!` or `?` takes the formula right
after it, so `!A * B` is `(!A) * B`. A postfix `^` negates the formula
just before it: `!A^` is `!(A^)`, and `(!A)^` is the negation of `!A`.

`seq print` shows how a sequent is grouped:

```console
$ deduxit seq print "|- a * b | c & d + e"
⊢ (((a ⊗ b) ⅋ c) & d) ⊕ e
$ deduxit seq print -i "a -o b -o c |- a -o c"
a ⊸ (b ⊸ c) ⊢ a ⊸ c
$ deduxit seq print "|- !a^, ~(a * b)"
⊢ !~a, ~a ⅋ ~b
```

## How a sequent is read

deduxit reads every sequent as a one-sided sequent `⊢ Γ`. The formulas
left of the turnstile move to the right, negated. `A -o B` becomes
`~A par B`, and negation is pushed down to the atoms. The formulas keep
the order they were written in. `seq print` writes this form, and `-i`
the two-sided form of intuitionistic linear logic:

```console
$ deduxit seq print "A, A -o B |- B"
⊢ ~A, A ⊗ ~B, B
$ deduxit seq print -i "A, A -o B |- B"
A, A ⊸ B ⊢ B
```

The text `seq print` writes reads back as the sequent it shows:
one-sided without `-i`, two-sided with it.

## Intuitionistic linear logic

With `-i` (`--intuitionistic`), a sequent is read in the syntax of
intuitionistic linear logic:
- one formula right of the turnstile, any number left of it;
- the connectives `*`, `&`, `+`, `-o`, `!` and the units `1`, `top`
  and `0`;
- no negation (`~`, `^`), no par (`|`, `par`, `⅋`), no `?` and no
  `bot`.

A symbol of the classical syntax is refused, and the error says what to
write instead:

```console
$ deduxit seq print -i "|- ~A par B"
error: cannot parse the sequent
  |- ~A par B
     ^ unexpected "~"
  intuitionistic linear logic has no negation: write a negated formula on the other side of |- (A |- B, not |- ~A, B), and ~A par B as A -o B; --ill-syntax one-sided reads the classical syntax
$ deduxit seq print -i --ill-syntax one-sided "|- ~A par B"
⊢ A ⊸ B
```

`--ill-syntax one-sided` reads the classical syntax instead, as the
one-sided sequent it stands for. The sequent must still have an
intuitionistic reading.

## First-order logic

A first-order sequent adds quantifiers and predicates over terms
([Prove in first-order logic](first-order.md)):
- `forall x. A` (`∀x. A`) and `exists x. A` (`∃x. A`). `forall x y. A`
  binds both variables.
- A quantifier binds loosest. Its scope runs right as far as it can: to
  a `)`, a `,`, the turnstile or the end. So `forall x. p(x) * q` is
  `forall x. (p(x) * q)`.
- An atom may be a predicate applied to terms: `p(x, f(a))`. Inside the
  brackets, a name bound by an enclosing quantifier is that variable.
  Any other name is a constant, or a function symbol when `(` follows
  it.
- A symbol takes the same number of arguments everywhere. A name is a
  predicate or a function symbol, not both.
- Every variable is bound by a quantifier.

```console
$ deduxit seq print "forall x. p(x) |- exists y. p(y)"
⊢ ∃x. ~p(x), ∃y. p(y)
```

## The Lambek calculus

With `--cyclic` or `--lambek`, the order of the formulas matters, and
the text also reads the two divisions `A \ B` and `B / A`. They bind as
loosely as `-o`. [Prove in the Lambek calculus](lambek.md) and
[Prove in cyclic linear logic](cyclic.md) say how these modes read a
sequent.
