# Read a proof as a program

Every intuitionistic proof is also a program: its formulas are the
program's types, and its hypotheses the program's inputs. This is the
Curry-Howard correspondence. `--lambda` prints the program of a proof as
a lambda-term, the notation in which `λx. M` is the function of `x`
whose result is `M`, and `f x` applies `f` to `x`:

```console
$ deduxit prove -i --lambda 'a, a -o b |- b'
provable (IMLL, intuitionistic, two-sided engine)
───── ax   ───── ax
a ⊢ a      b ⊢ b
──────────────── ⊸L
  a, a ⊸ b ⊢ b
lambda-term: x : a, f : a ⊸ b ⊢ f x : b
```

The last line is the term's typing judgement. Left of `⊢` stand the
hypotheses' variables with their types: `x` has the type `a`, and `f`
the type `a ⊸ b`. Right of it stands the term, `f x`, with its type,
`b`. The proof that `b` follows from `a` and `a ⊸ b` is the program
that applies `f` to `x`.

A proof of intuitionistic linear logic (`-i`) is a term of a linear
lambda calculus, and a derivation of LJ (`--logic intuitionistic` or
`minimal`) a term of the simply typed lambda calculus. deduxit reads
every intuitionistic proof it finds or checks back as its term. The term
is computed from the proof the checker accepted, one construction per
rule. The proof stays the certificate. The command `term` works the
other way round: it [checks a term](#check-a-term) you write, and
[finds the terms of a type](#find-a-term-of-a-type).

## The linear lambda calculus

In a linear lambda calculus a term uses each variable once, as linear
logic uses each hypothesis once. A variable of a `!` type may be copied
or discarded, and both components of a pair use the same variables,
since a program takes only one of them.

A proof of intuitionistic linear logic reads back as a term of the
linear lambda calculus of Benton, Bierman, de Paiva and Hyland, "A term
calculus for intuitionistic linear logic" (TLCA 1993, LNCS 664, pages 75
to 90). deduxit takes it in the sequent calculus form, with the
additives and their units, that G. M. Bierman gives in *On
Intuitionistic Linear Logic* (PhD thesis, University of Cambridge, 1993,
published as technical report UCAM-CL-TR-346, 1994), chapter 3, figure
3.2. Each rule of the two-sided sequent calculus has its term:

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

Every form in the table is the thesis's except the cut's. The thesis's
cut is the substitution `N[x := M]` itself; deduxit writes it as the
redex whose reduction is that substitution, in the calculus's own
syntax. A left rule substitutes, as in the thesis, so the term of a
cut-free proof is in normal form. `true` and `false` take every other
hypothesis, since `⊤` and `0` discard a linear context only by consuming
it. The two components of a pair share their context:

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
calculus, so this reading is deduxit's own.

```console
$ deduxit prove -i --affine --lambda --tree never 'a, b |- a'
provable (IMLL, intuitionistic affine, two-sided engine)
lambda-term: x : a, y : b ⊢ x : a
```

## Cuts are redexes

A cut on `A` between a proof of `Γ ⊢ M : A` and one of `Δ, x : A ⊢ N :
C` is the redex `(λx:A. N) M`: a function applied to its argument.
Reducing it is the substitution of the thesis's cut, and eliminating the
cut (`deduxit eliminate`, see [Eliminate cuts](cut.md)) reduces the
term. The session command `lambda` shows the term of a session that
cuts:

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

A derivation of LJ, from intuitionistic or minimal logic (`--logic`, see
[Decide ordinary logic](ordinary-logic.md)), reads back to the simply
typed lambda calculus with products, sums, a unit and an empty type, as
J.-Y. Girard, Y. Lafont and P. Taylor give it in *Proofs and Types*
(Cambridge University Press, 1989), sections 3.1.2 and 10.6. Their `π₁`,
`π₂`, `ι₁`, `ι₂`, `δ` and `ε` are written `fst`, `snd`, `inl`, `inr`,
`case` and `abort`. The book names no term of the unit type, so `⟨⟩`, the empty
pair, is deduxit's own notation for it. `¬A` is `A → ⊥`, and `A ↔ B` the
pair of both implications:

```console
$ deduxit prove --logic intuitionistic --lambda --tree never '|- (a -> b) -> (b -> c) -> a -> c'
valid (intuitionistic logic by the SAT solver)
lambda-term: ⊢ λf. λg. λx. g (f x) : (a → b) → ((b → c) → (a → c))
$ deduxit prove --logic intuitionistic --lambda --tree never '~(a \/ b) |- ~a /\ ~b'
valid (intuitionistic logic by the SAT solver)
lambda-term: f : (a ∨ b) → ⊥ ⊢ ⟨λx. f inl(x), λy. f inr(y)⟩ : (a → ⊥) ∧ (b → ⊥)
$ deduxit prove --logic minimal --lambda --tree never '|- a -> ~~a'
valid (minimal logic by the SAT solver)
lambda-term: ⊢ λx. λf. f x : a → ((a → ⊥) → ⊥)
```

Under `--logic`, `--linear` prints the term of the linear proof of the
image instead.

## What has no term

A proof of classical linear logic may have several conclusions, which no
lambda-term types. These have no term either:
- a derivation of LK;
- a proof with Mix;
- a proof of the cyclic or the Lambek calculus, whose terms belong to an
  ordered calculus of their own;
- a proof with first-order quantifiers, which are not quantifiers over
  types.

The line says why:

```console
$ deduxit prove --lambda --tree never '|- a, ~a'
provable (MLL, classical, focus engine)
no lambda-term: a proof of classical logic may have several conclusions, which no lambda-term types; a term needs an intuitionistic proof
$ deduxit prove -i --lambda --tree never 'forall x. p(x) |- p(c)'
provable (IMLL1, intuitionistic, two-sided engine)
no lambda-term: quantifiers over first-order terms are not quantifiers over types
```

## How the term is printed

`--lambda-notation ascii` writes the term in ASCII, a notation of
deduxit's own: `\x. M` for `λx. M`, `M * N` for `M ⊗ N`, `()` for `∗`,
`<M, N>` for `⟨M, N⟩`, `->` and `|` in a case, and the types as the parser
reads them. `--lambda-types` gives every binder its variable's type,
`λx:A. M`, and `inl`, `inr`, `false` and `abort` the type they give as
a subscript, `inl_(a ⊕ b)(x)`. The settings keys are `lambda.notation`
and `lambda.types`:

```console
$ deduxit prove -i --lambda --lambda-notation ascii --lambda-types --tree never '|- a -o b -o a * b'
provable (IMLL, intuitionistic, two-sided engine)
lambda-term: |- \x:a. \y:b. x * y : a -o (b -o (a * b))
```

In a program that uses the library, a `lambda::Term` prints with
`Display` (`term.to_string()` is the term alone, in the default
notation), and `Term::write` and `Term::write_judgement` take the
options. With the types shown, `Term::parse` reads the text back.

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

In JSON the outcome gains the key `lambda`: the term's document (its
wire level as `version`, its calculus, atoms, types, variables, the
hypotheses' variables as `context`, its type as `goal`, and its nodes,
each after the nodes it names, `root` the whole term), with its `text`
and its `judgement` as printed, or `{"error": …}` where there is none:

```console
$ deduxit prove -i --format json --lambda 'a |- a'
{"checked":true,"deduxit":"0.7.0","engine":"two-sided","fragment":"IMLL","lambda":{"atoms":["a"],"calculus":"linear","context":[0],"goal":0,"judgement":"x : a ⊢ x : a","nodes":[{"kind":"var","variable":0}],"root":0,"text":"x","types":[{"atom":0,"kind":"atom"}],"variables":[{"name":"x","type":0}],"version":23},"mode":"intuitionistic","nodes":[{"ax":[0,1]}],"sequent":{"antecedents":1,"atoms":["a"],"roots":[0,1],"terms":[{"D":0},{"V":0}]},"statistics":{"copies":0,"forward_copies":0,"links":0,"memo_entries":1,"memo_hits":0,"nodes":1,"splits":0,"tests":0,"work":16},"verdict":"proved","version":1}
```

A term repeats a left rule's term wherever its hypothesis goes, in both
components of a pair, so its text can be longer than the derivation by
any factor. A term whose text would pass `--derivation-limit` is not
written, and the line says so.

## A term as data

The value under `lambda` is a document of its own, at wire level 23. A
type and a node are objects tagged by `kind`, and name what they are
built from by index:

```json
{"version": 23, "calculus": "simple", "atoms": ["a"],
 "types": [{"kind": "atom", "atom": 0},
           {"kind": "lollipop", "from": 0, "to": 0}],
 "variables": [{"type": 0, "name": "x"}],
 "context": [], "goal": 1,
 "nodes": [{"kind": "var", "variable": 0},
           {"kind": "lambda", "variable": 0, "body": 0}],
 "root": 1}
```

This is `λx:a. x` of type `a → a`. A variable is an entry of
`variables` with its type, and a binder and every use name that entry,
so two variables may have one name.

The library reads the document back (`wire::upgrade`), reads a term
from text (`lambda::Term::parse`), and checks either
(`lambda::Term::check`):

- every node has a type by its rule;
- every variable is used as its calculus allows: exactly once in the
  linear calculus, at most once in the affine one, any number of times
  in the simple one;
- the whole term has the type `goal`, and its free variables are those
  of `context`.

The text the library reads is what `--lambda-types` prints. Every `λ`
says its variable's type, and `inl`, `inr`, `abort` and `false` say the
type they give as a subscript, `inl_(a ∨ b)(x)`: the calculus is typed
in Church's style, and deduxit infers no types. The
[API documentation](https://deduxit.github.io/deduxit/) has the whole
syntax under `lambda::Term::parse`.

## Check a term

`term check` reads a term and prints its type, or says why it has none.
Write it as `--lambda-types` prints terms: every `λ` says its variable's
type.

```console
$ deduxit term check '\f:a -> b. \x:a. f x'
well typed
⊢ λf:(a → b). λx:a. f x : (a → b) → (a → b)
$ deduxit term check --calculus linear '\x:a. x * x'; echo "exit $?"
ill typed: the variable x is used more than once, and the linear lambda calculus uses every variable exactly once
exit 1
```

Declare free variables before `⊢`, as in `x : a |- \y:b. x`, and give a
type after a last `:` to have it compared with the term's. `--calculus`
is `simple` by default, or `linear` or `affine`. The exit status is 0
for a well-typed term and 1 for an ill-typed one. `--format json` writes
the term's document with the type of every node under `typing`.

## Find a term of a type

A term of type `A` is a proof of `A` in intuitionistic logic, so a type
has a term exactly when its formula is valid there. `term inhabit` takes
the type as a formula of [ordinary logic](ordinary-logic.md), decides it,
and reads the proof back as a term, which it checks:

```console
$ deduxit term inhabit 'a /\ b -> b /\ a'
inhabited
⊢ λx:(a ∧ b). ⟨snd(x), fst(x)⟩ : (a ∧ b) → (b ∧ a)
$ deduxit term inhabit '((a -> b) -> a) -> a'; echo "exit $?"
empty: the formula is not valid in intuitionistic logic
countermodel:
  world 0 (the model's world): forces nothing; below 1, 2
  world 1: forces a; below 3
  world 2: forces nothing; below 3
  world 3: forces a
exit 1
```

Peirce's law has no term: the Kripke model shows that the formula is not
valid. Formulas left of `⊢` are free variables the term may use. The
exit status is 0 for a type with a term, 1 for one without, and 3 when
the time or a limit ran out first. Deciding this is PSPACE-complete
(R. Statman, "Intuitionistic propositional logic is polynomial-space
complete", *Theoretical Computer Science* 9, 1979), so a large type may
take long.

## Count a type's terms

`term enumerate` lists a type's terms and counts them. Two terms count
as one when they are equal by β- and η-conversion, so each is listed in
its long normal form: a function is a `λ`, a pair a pair, and an atom a
variable applied to arguments. This is the search that C.-B. Ben-Yelles
gave in his thesis (*Type Assignment in the Lambda-Calculus: Syntax &
Semantics*, Swansea, 1979) and that J. R. Hindley describes in *Basic
Simple Type Theory* (Cambridge University Press, 1997), chapter 8:

```console
$ deduxit term enumerate '(a -> a -> b) -> a -> a -> b'
exactly 4 terms
λf:(a → (a → b)). λx:a. λy:a. f x x
λf:(a → (a → b)). λx:a. λy:a. f x y
λf:(a → (a → b)). λx:a. λy:a. f y x
λf:(a → (a → b)). λx:a. λy:a. f y y
$ deduxit term enumerate --depth 3 '(a -> a) -> a -> a'
infinitely many terms, the first 3 within depth 3
λf:(a → a). λx:a. x
λf:(a → a). λx:a. f x
λf:(a → a). λx:a. f (f x)
```

The count is one of three. It is exact when the search met every term.
It is "infinitely many" when a goal comes back inside itself with the
same types in scope, as `f (…)` does for the Church numerals above: a
term can then be put inside itself as often as wanted. It is "at least"
when `--depth` or a limit cut the search; the depth counts how deep variables are
applied inside each other's arguments (8 by default). `--max-terms`
(32 by default) bounds the list, not the count; a list cut short keeps
the shortest terms. Every term listed is
checked.

The type is built from atoms, `->`, `/\`, `<->` and `true`. With `\/`,
`false` or `~`, βη-equality makes terms equal that long normal forms
tell apart (G. Scherer and D. Rémy, "Which simple types have a unique
inhabitant?", ICFP 2015), and the command refuses the type. `term
inhabit` takes every type.


## A typing derivation drawn

A well-typed term has one typing derivation, and the library draws it
as a proof tree in LaTeX, Typst and SVG, and so in PNG and PDF
(`lambda::Drawing`, which every export takes as it takes a proof). Each
inference concludes the judgement `Γ ⊢ M : A` of a subterm `M`, and its
rule is the rule of natural deduction the term's last construction
stands for: `ax` for a variable, `→I` for `λ`, `→E` for an application,
`∧I`, `∧E₁`, `∨I₁`, `∨E` and so on, with `⊸`, `⊗`, `&`, `⊕` and the
rules of `!` in the linear calculi.

In the simple calculus every premise has its conclusion's context, with
the variables the rule binds added. In the linear calculus the context
is split as the rules of linear logic split it: each variable goes to
the premise that uses it, and the two components of a pair and the two
branches of a `case` share theirs. These are the steps of the
derivation of `let x be y ⊗ z in f derelict(z) ⊗ y`, as an SVG
drawing's description reads them:

```text
1. x : a ⊗ !b ⊢ x : a ⊗ !b, by ax.
2. f : b ⊸ c ⊢ f : b ⊸ c, by ax.
3. z : !b ⊢ z : !b, by ax.
4. z : !b ⊢ derelict(z) : b, by !E from 3.
5. f : b ⊸ c, z : !b ⊢ f derelict(z) : c, by ⊸E from 2 and 4.
6. y : a ⊢ y : a, by ax.
7. f : b ⊸ c, y : a, z : !b ⊢ f derelict(z) ⊗ y : c ⊗ a, by ⊗I from 5 and 6.
8. x : a ⊗ !b, f : b ⊸ c ⊢ let x be y ⊗ z in f derelict(z) ⊗ y : c ⊗ a, by ⊗E from 1 and 7.
```

Each judgement writes its whole subterm, so the drawing grows with the
term's size times its depth; a drawing past `--derivation-limit`'s
bound (`limits.derivation_bytes`) is refused before anything is laid
out. Rocq has no certificate of a typing derivation.
