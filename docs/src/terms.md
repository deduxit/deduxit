# Proofs as programs

An intuitionistic proof is a program: by the Curry-Howard correspondence
a proof of intuitionistic linear logic is a term of a linear lambda
calculus, whose type is the proved sequent, and a derivation of LJ is a
term of the simply typed lambda calculus. deduxit reads every
intuitionistic proof it finds or checks back as its term. `--lambda`
prints the term's typing judgement, the hypotheses' variables with their
types, then the term and its type:

```console
$ deduxit prove -i --lambda 'a, a -o b |- b'
provable (IMLL, intuitionistic, two-sided engine)
───── ax   ───── ax
a ⊢ a      b ⊢ b
──────────────── ⊸L
  a, a ⊸ b ⊢ b
lambda-term: x : a, f : a ⊸ b ⊢ f x : b
```

The term is computed from the proof the checker accepted, one
construction per rule; it is shown, not trusted, and the proof stays the
certificate.

## The linear lambda calculus

A proof of intuitionistic linear logic reads back as a term of the
linear lambda calculus of Benton, Bierman, de Paiva and Hyland, "A term
calculus for intuitionistic linear logic" (TLCA 1993, LNCS 664, pages
75 to 90), in the sequent calculus form that G. M. Bierman gives it, with
the additives and their units, in *On Intuitionistic Linear Logic* (PhD
thesis, University of Cambridge, 1993, published as technical report
UCAM-CL-TR-346, 1994), chapter 3, figure 3.2. Each rule of the two-sided
sequent calculus has its term:

| rule | term |
|---|---|
| axiom | `x` |
| `⊸R`, `⊸L` | `λx:A. M`; `N[x := f M]` |
| `⊗R`, `⊗L` | `M ⊗ N`; `let z be x ⊗ y in M` |
| `1R`, `1L` | `∗`; `let z be ∗ in M` |
| `&R`, `&L₁`, `&L₂` | `⟨M, N⟩`; `M[x := fst(z)]`, `M[x := snd(z)]` |
| `⊕R₁`, `⊕R₂`, `⊕L` | `inl(M)`, `inr(M)`; `case z of inl(x) → M ‖ inr(y) → N` |
| `⊤R`, `0L` | `true(x⃗)`; `false_A(x⃗; z)` |
| `!R`, `!L` | `promote y⃗ for x⃗ in M`; `M[x := derelict(z)]` |
| `!w`, `!c` | `discard z in M`; `copy z as x, y in M` |
| cut | `(λx:A. N) M` |

Every form in the table is the thesis's, but the cut's: the thesis's cut
is the substitution `N[x := M]` itself, and deduxit writes it as the redex
whose reduction that substitution is, in the calculus's own syntax. A
left rule substitutes, as the thesis does, so the term of a cut-free
proof is in normal form. `true` and `false` take every other hypothesis,
since `⊤` and `0` discard a linear context only by consuming it, and the
two components of a pair share their context:

```console
$ deduxit prove -i --lambda --tree never '|- a -o b -o a * b'
provable (IMLL, intuitionistic, two-sided engine)
lambda-term: ⊢ λx. λy. x ⊗ y : a ⊸ (b ⊸ (a ⊗ b))
$ deduxit prove -i --lambda --tree never '1 * a |- a'
provable (IMLL with units, intuitionistic, two-sided engine)
lambda-term: x : 1 ⊗ a ⊢ let x be y ⊗ z in let y be ∗ in z : a
$ deduxit prove -i --lambda --tree never 'a & b |- b & a'
provable (IALL, intuitionistic, additive engine)
lambda-term: x : a & b ⊢ ⟨snd(x), fst(x)⟩ : b & a
$ deduxit prove -i --lambda --tree never 'a + b |- b + a'
provable (IALL, intuitionistic, additive engine)
lambda-term: x : a ⊕ b ⊢ case x of inl(y) → inr(y) ‖ inr(z) → inl(z) : b ⊕ a
$ deduxit prove -i --lambda --tree never 'a, b |- top'
provable (IALL, intuitionistic, two-sided engine)
lambda-term: x : a, y : b ⊢ true(x, y) : ⊤
$ deduxit prove -i --lambda --tree never '0, a |- b'
provable (IALL, intuitionistic, two-sided engine)
lambda-term: x : 0, y : a ⊢ false(y; x) : b
$ deduxit prove -i --lambda --tree never '!a |- a * a'
provable (IMELL, intuitionistic, horn engine)
lambda-term: x : !a ⊢ copy x as y, z in derelict(z) ⊗ derelict(y) : a ⊗ a
$ deduxit prove -i --lambda --tree never '!(a -o b), !a |- !b'
provable (IMELL, intuitionistic, two-sided engine)
lambda-term: x : !(a ⊸ b), y : !a ⊢ promote x, y for z, u in derelict(z) derelict(u) : !b
```

A proof of affine linear logic (`--affine`) reads back to the affine
lambda calculus: the same terms, with weakening left implicit, so a
variable may go unused. Neither the paper nor the thesis gives an affine
calculus; this reading is deduxit's own.

```console
$ deduxit prove -i --affine --lambda --tree never 'a, b |- a'
provable (IMLL, intuitionistic affine, two-sided engine)
lambda-term: x : a, y : b ⊢ x : a
```

## Cuts are redexes

A cut on `A` between a proof of `Γ ⊢ M : A` and one of `Δ, x : A ⊢ N :
C` is the redex `(λx:A. N) M`, whose reduction is the substitution of
the thesis's cut: eliminating the cut (`deduxit eliminate`, the chapter
[Cut and its elimination](cut.md)) reduces the term. A session that
cuts shows it:

```console
$ deduxit interact -i 'a, a -o b |- b'
> cut 0 b 0 1
> close
> lambda
opened goal 1: 0: a, 1: a ⊸ b ⊢ 2: b
opened goal 2: 1: b ⊢ 0: b
goal 1: provable (IMLL, intuitionistic, two-sided engine)
goal 2: provable (IMLL, intuitionistic, two-sided engine)
no goal is open: `proof` checks the proof
x : a, f : a ⊸ b ⊢ (λy. y) (f x) : b
```

## The simply typed lambda calculus

A derivation of LJ, from intuitionistic or minimal logic (`--logic`, the
chapter [Ordinary logic](ordinary-logic.md)), reads back to the simply
typed lambda calculus with products, sums, a unit and an empty type, as
J.-Y. Girard, Y. Lafont and P. Taylor give it in *Proofs and Types*
(Cambridge University Press, 1989), sections 3.1.2 and 10.6, their `π₁`,
`π₂`, `ι₁`, `ι₂`, `δ` and `ε` written `fst`, `snd`, `inl`, `inr`, `case`
and `abort`. The book names no term of the unit type; `⟨⟩`, the empty
pair, is deduxit's own notation for it. `¬A` is `A → ⊥`, and `A ↔ B` the
pair of both implications:

```console
$ deduxit prove --logic intuitionistic --lambda --tree never '|- (a -> b) -> (b -> c) -> a -> c'
valid (intuitionistic logic by the cbn translation into ILL, two-sided engine)
lambda-term: ⊢ λf. λg. λx. g (f x) : (a → b) → ((b → c) → (a → c))
$ deduxit prove --logic intuitionistic --lambda --tree never '~(a \/ b) |- ~a /\ ~b'
valid (intuitionistic logic by the cbn translation into ILL, two-sided engine)
lambda-term: f : (a ∨ b) → ⊥ ⊢ ⟨λx. f inl(x), λy. f inr(y)⟩ : (a → ⊥) ∧ (b → ⊥)
$ deduxit prove --logic minimal --lambda --tree never '|- a -> ~~a'
valid (minimal logic by the cbn translation into ILL, two-sided engine)
lambda-term: ⊢ λx. λf. f x : a → ((a → ⊥) → ⊥)
```

Under `--logic`, `--linear` prints the term of the linear proof of the
image instead.

## What has no term

A proof of classical linear logic has several conclusions, which no
lambda-term types; nor have a derivation of LK, a proof with Mix, a proof
of the cyclic or the Lambek calculus (whose terms belong to an ordered
calculus of their own), or a proof with first-order quantifiers, which
are no quantifiers over types. The line says why:

```console
$ deduxit prove --lambda --tree never '|- a, ~a'
provable (MLL, classical, focus engine)
no lambda-term: a proof of classical logic has several conclusions, which no lambda-term types
$ deduxit prove -i --lambda --tree never 'forall x. p(x) |- p(c)'
provable (IMLL1, intuitionistic, two-sided engine)
no lambda-term: quantifiers over first-order terms are no quantifiers over types
```

## How the term is printed

`--lambda-notation ascii` writes it in ASCII, a notation of deduxit's own
(`\x. M` for `λx. M`, `M * N` for `M ⊗ N`, `()` for `∗`, `<M, N>` for
`⟨M, N⟩`, `->` and `|` in a case, the types as the parser reads them), and `--lambda-types` gives every binder its variable's type,
`λx:A. M`, and `false` its goal's, as the calculus writes them; the
settings keys are `lambda.notation` and `lambda.types`:

```console
$ deduxit prove -i --lambda --lambda-notation ascii --lambda-types --tree never '|- a -o b -o a * b'
provable (IMLL, intuitionistic, two-sided engine)
lambda-term: |- \x:a. \y:b. x * y : a -o (b -o (a * b))
```

`check --lambda` prints the term of a proof read from a file, and the
session command `lambda` of `interact` that of the finished proof or
derivation:

```console
$ deduxit prove -i --format json 'a & b |- b & a' > proof.json
$ deduxit check -i --lambda --tree never proof.json
valid proof of a & b ⊢ b & a (intuitionistic)
lambda-term: x : a & b ⊢ ⟨snd(x), fst(x)⟩ : b & a
```

In LaTeX, Typst, SVG, PNG and PDF the judgement stands under the
conclusion of the derivation:

```console
$ deduxit prove -i --lambda --format latex 'a, a -o b |- b'
% provable (IMLL, intuitionistic, two-sided engine)
\begin{prooftree}
\infer0[$\mathrm{ax}$]{a &\vdash a}
\infer0[$\mathrm{ax}$]{b &\vdash b}
\infer2[$\multimap\mathrm{L}$]{a, a \multimap b &\vdash b}
\infer[no rule]1{x : a, f : a \multimap b \vdash f\ x : b}
\end{prooftree}
```

In JSON the outcome gains the key `lambda`: the term as data (its
calculus, atoms, types, variables, the hypotheses' variables as
`context`, its type as `goal`, and its nodes, each after the nodes it
names, `root` the whole term), with its `text` and its `judgement` as
printed, or `{"error": …}` where there is none:

```console
$ deduxit prove -i --format json --lambda 'a |- a'
{"checked":true,"deduxit":"0.2.0","engine":"two-sided","fragment":"IMLL","lambda":{"atoms":["a"],"calculus":"linear","context":[0],"goal":0,"judgement":"x : a ⊢ x : a","nodes":[{"kind":"var","variable":0}],"root":0,"text":"x","types":[{"atom":0,"kind":"atom"}],"variables":[{"index":0,"letter":"x","type":0}]},"mode":"intuitionistic","nodes":[{"ax":[0,1]}],"sequent":{"antecedents":1,"atoms":["a"],"roots":[0,1],"terms":[{"D":0},{"V":0}]},"statistics":{"copies":0,"forward_copies":0,"links":0,"memo_entries":1,"memo_hits":0,"nodes":1,"splits":0,"tests":0,"work":16},"verdict":"proved","version":1}
```

A term repeats a left rule's term wherever its hypothesis goes, in both
components of a pair, so its text can be longer than the derivation by
any factor; a term whose text would pass `--derivation-limit` is not
written, and the line says so.
