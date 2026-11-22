# Translate, rewrite and interpolate a sequent

deduxit turns a sequent of [ordinary logic](ordinary-logic.md) into
another and checks what the change promises before it prints the
result. Three changes are built: the negative translations of classical
logic into intuitionistic and minimal logic, the normal forms of
classical formulas, and Craig interpolants. Herbrand's theorem builds on
the clausal form: it proves a first-order sequent from ground instances
of its clauses.

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
| `godel-gentzen` | `¬¬` before every atom, `A ∨ B` as `¬(¬A ∧ ¬B)` and `∃x. A` as `¬∀x. ¬A` | minimal logic |
| `kuroda` | `¬¬` before every formula of the sequent and after every `∀` | intuitionistic logic |
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

The first three translate first-order sequents too:

```console
$ deduxit seq translate --by kuroda 'forall x. p(x) |- exists y. p(y)'
Kuroda's translation into intuitionistic logic: ¬¬(∀x. ¬¬p(x)) ⊢ ¬¬(∃y. p(y))
classical logic: valid, a checked derivation of LK (5 inferences)
intuitionistic logic: valid, a checked derivation of LJ (12 inferences)
The verdicts agree, as Kuroda's translation claims.
```

Glivenko's theorem fails at first order, so `--by glivenko` refuses a
first-order sequent: `⊢ ∀x. (p(x) ∨ ¬p(x))` holds classically, but its
double negation does not hold intuitionistically.

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

## Rewrite a sequent into a normal form

`seq normalize` rewrites each formula of a classical sequent into a
normal form, one law per step, and prints every step with the sequent
after it. Conjunctive normal form (CNF) is the default: a conjunction of
clauses, each clause a disjunction of atoms and negated atoms.

```console
$ deduxit seq normalize '~(a -> b /\ c)'
Conjunctive normal form: ⊢ a ∧ (¬b ∨ ¬c)
1. A → B ≡ ¬A ∨ B, at a → (b ∧ c):
   ⊢ ¬(¬a ∨ (b ∧ c))
2. ¬(A ∨ B) ≡ ¬A ∧ ¬B, at ¬(¬a ∨ (b ∧ c)):
   ⊢ ¬¬a ∧ ¬(b ∧ c)
3. ¬¬A ≡ A, at ¬¬a:
   ⊢ a ∧ ¬(b ∧ c)
4. ¬(A ∧ B) ≡ ¬A ∨ ¬B, at ¬(b ∧ c):
   ⊢ a ∧ (¬b ∨ ¬c)
every formula is equivalent to its normal form: valid, a checked clausal proof
```

Each step names its law and the subformula it rewrites; where that
subformula occurs more than once, every occurrence is rewritten at once.
The laws come in phases: first `→` and `↔`, then the negations, then the
constants `⊤` and `⊥`, then distribution. In each phase the step takes
the leftmost place it fits, outermost first.

`--to` names the form:

| `--to` | the form | what it keeps |
|---|---|---|
| `nnf` | negation normal form: no `→` or `↔`, `¬` before atoms alone | each formula's meaning |
| `cnf` (the default) | a conjunction of clauses, by distribution | each formula's meaning |
| `dnf` | a disjunction of conjunctions of literals, by distribution | each formula's meaning |
| `tseitin` | Tseitin's clauses of the sequent's negation | whether the sequent is valid |
| `plaisted-greenbaum` | Plaisted and Greenbaum's clauses | whether the sequent is valid |
| `prenex` | every quantifier in front | each formula's meaning |
| `miniscope` | negation normal form, every quantifier as far in as it goes | each formula's meaning |
| `skolem` | the sequent's negation, each `∃` replaced by a term of new symbols | whether the sequent is valid |
| `herbrand` | the sequent, each strong quantifier replaced by a term of new symbols | whether the sequent is valid |
| `clausal` | the Skolem form's clauses, each closed by `∀`s in front | whether the sequent is valid |

```console
$ deduxit seq normalize --to dnf '(a \/ b) /\ ~c'
Disjunctive normal form: ⊢ (a ∧ ¬c) ∨ (b ∧ ¬c)
1. A ∧ (B ∨ C) ≡ (A ∧ B) ∨ (A ∧ C), at (a ∨ b) ∧ ¬c:
   ⊢ (a ∧ ¬c) ∨ (b ∧ ¬c)
every formula is equivalent to its normal form: valid, a checked clausal proof
```

Distribution can make a formula exponentially longer. `--form-limit`
(the settings key `limits.form_occurrences`, 10000 by default) bounds
the size of the normal form and of every sequent on the way to it, and
`none` lifts it:

```console
$ deduxit seq normalize --form-limit 8 'a /\ b \/ c /\ d'; echo "exit $?"
error: the normal form would grow to 11 occurrences, more than the limit of 8; raise the limit with --form-limit, or take --to tseitin, whose clauses grow with the sequent
exit 2
```

The two encodings answer whether the sequent is valid, which is whether
it can be false. Each formula right of `⊢` moves left, negated. Then a
new atom names each compound subformula, the innermost first, with the
clauses that tie the atom to what it names. The result is a sequent of
clauses, `C1, …, Cn ⊢`, which is valid exactly when the input is.
Tseitin's encoding gives each new atom the clauses of both directions of
its definition; Plaisted and Greenbaum's only the direction the
subformula's place needs, so fewer clauses:

```console
$ deduxit seq normalize --to plaisted-greenbaum 'a -> b |- ~(b /\ c)'
Plaisted and Greenbaum's encoding: t1, t3, (¬t1 ∨ ¬a) ∨ b, ¬t2 ∨ b, ¬t2 ∨ c, ¬t3 ∨ t2 ⊢
1. ¬(b ∧ c) moves left of ⊢ as ¬¬(b ∧ c)
2. t1 names a → b: (¬t1 ∨ ¬a) ∨ b
3. t2 names b ∧ c: ¬t2 ∨ b, ¬t2 ∨ c
4. t3 names ¬¬t2: ¬t3 ∨ t2
wherever the sequent is false, the clauses hold with the new atoms as defined: valid, a checked clausal proof
wherever the clauses hold, the sequent is false: valid, a checked clausal proof
```

The new atoms are `t1`, `t2`, …, or `t_1`, … where the sequent has an
atom named `t` and a number.

### At first order

`nnf`, `prenex`, `miniscope`, `skolem`, `herbrand` and `clausal` take
first-order sequents; the other forms take propositional ones. Prenex
form pulls each quantifier out of the formula as written, one law per
step:

```console
$ deduxit seq normalize --to prenex '(forall x. p(x)) -> exists y. q(y)'
Prenex normal form: ⊢ ∃x. ∃y. (p(x) → q(y))
1. (∀x. A) → B ≡ ∃x. (A → B), x not free in B, at (∀x. p(x)) → (∃y. q(y)):
   ⊢ ∃x. (p(x) → (∃y. q(y)))
2. A → ∃x. B ≡ ∃x. (A → B), x not free in A:
   ⊢ ∃x. ∃y. (p(x) → q(y))
the law of rewrite 1: valid, a checked derivation of LK (26 inferences)
the law of rewrite 2: valid, a checked derivation of LK (27 inferences)
every formula is equivalent to its normal form: valid, a checked derivation of LK (34 inferences)
```

Each step's law is checked on its own: a derivation of LJ where the law
holds in intuitionistic logic, of LK where only classical logic proves
it (both laws above), each variable the subformula leaves free taken as
a new constant. A step that rewrites a subformula under a quantifier
shows the sequent after it but not the subformula, whose variable is
bound outside it.

The Skolem and clausal forms replace each `∃` of the sequent's negation
by a new function symbol (`sk1`, `sk2`, …) applied to the variables free
in it; equal formulas share their symbol. The result is valid exactly
when the input is:

```console
$ deduxit seq normalize --to clausal 'forall x. exists y. r(x, y) |- exists z. r(a, z)'
Clausal normal form: ∀x. r(x, sk1(x)), ∀z. ¬r(a, z) ⊢
1. ∃z. r(a, z) moves left of ⊢ as ¬(∃z. r(a, z))
2. ¬∃x. A ≡ ∀x. ¬A, at ¬(∃z. r(a, z)):
   ∀x. ∃y. r(x, y), ∀z. ¬r(a, z) ⊢
3. ∃y. A becomes A[f(x̄)/y], f a new function symbol over the variables x̄ free in ∃y. A: satisfiable exactly when the input is:
   ∀x. r(x, sk1(x)), ∀z. ¬r(a, z) ⊢
the law of rewrite 2: valid, a checked derivation of LJ (15 inferences)
wherever the clauses hold, the sequent is false: valid, a checked derivation of LK (7 inferences)
```

One direction of the promise has a derivation: the clauses imply the
sequent's negation, each Skolem term a witness of its `∃`. The other,
that a model of the negation gives one of the clauses, needs a choice of
witnesses and has no derivation; deduxit checks the construction it
rests on instead: every symbol is new and applied to exactly the
variables free in its quantifier. `herbrand` keeps the sides and
replaces each strong quantifier (an `∃` left of `⊢`, a `∀` right of it),
and checks that the input implies the result.

Before it prints a form, deduxit checks what the form keeps, deciding
each claim by its SAT solver, whose clausal proof deduxit's checker
accepts, or at first order by the logic's default method, as above. For
CNF, DNF and NNF the claim is that each formula is
equivalent to its normal form. For an encoding there are two claims:
wherever the sequent is false, the clauses hold once each new atom takes
the value of what it names; and wherever the clauses hold, the sequent
is false. Together they say that the clauses can be satisfied exactly
when the sequent can be false. The exit status is 0 where the check
passes, 3 where a limit stopped it and 2 for an error. A claim found
false would be a defect of deduxit, and is an error.

## Prove a first-order sequent by Herbrand's theorem

A first-order sequent is valid in classical logic exactly when finitely
many ground instances of its clauses have no model (Herbrand's theorem).
`herbrand` makes the clausal form, puts closed terms for the clauses'
variables, the constants first and then each function applied to them,
and has the SAT solver decide the instances at each depth. It prints
the instances it needs, and the derivation of LK of the sequent it
builds from them:

```console
$ deduxit herbrand 'forall x. (p(x) -> p(f(x))), p(a) |- p(f(f(a)))'
Clausal normal form: ∀x. (¬p(x) ∨ p(f(x))), p(a), ¬p(f(f(a))) ⊢
1. p(f(f(a))) moves left of ⊢ as ¬p(f(f(a)))
2. A → B ≡ ¬A ∨ B:
   ∀x. (¬p(x) ∨ p(f(x))), p(a), ¬p(f(f(a))) ⊢
Instances of terms up to depth 1 (2 terms), 4 of 4 kept:
  ¬p(a) ∨ p(f(a))        clause 1, x := a
  ¬p(f(a)) ∨ p(f(f(a)))  clause 1, x := f(a)
  p(a)                   clause 2
  ¬p(f(f(a)))            clause 3
They have no model: the SAT solver's clausal proof is checked.
valid (classical logic by Herbrand's theorem): a checked derivation of LK (18 inferences) built from the instances
```

deduxit checks the solver's clausal proof, keeps only the instances the
solver needs, and builds the derivation from them: each Skolem term of
the clausal form becomes an eigenvariable again, and the terms of the
instances are the witnesses of `∀` left of `⊢` and `∃` right of it.
deduxit's checker accepts the derivation before the verdict is printed,
so the verdict does not rest on the clausal form. `--format json` holds
the derivation.

`--depth` bounds the terms, 3 by default (a constant is of depth 0). A
sequent that is not valid has no such instances, so the search ends
unknown at the depth:

```console
$ deduxit herbrand 'forall x. (p(x) -> p(f(x))) |- p(a)'; echo "exit $?"
Clausal normal form: ∀x. (¬p(x) ∨ p(f(x))), ¬p(a) ⊢
1. p(a) moves left of ⊢ as ¬p(a)
2. A → B ≡ ¬A ∨ B:
   ∀x. (¬p(x) ∨ p(f(x))), ¬p(a) ⊢
unknown: the 5 instances of terms up to depth 3 (4 terms) have a model; raise --depth, or decide the sequent with prove --logic classical
exit 3
```

`prove --logic classical` refutes such a sequent where it can, with a
checked finite structure. The exit status is 0 for a valid sequent, 3
for unknown and 2 for an error.

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

At first order the interpolant may hold no constant, function or free
variable of one half alone: each term made of one is bound by a
quantifier of its own, `∃` for a term of the first half and `∀` for one
of the second:

```console
$ deduxit interpolate 'forall x. (p(x) /\ q(x)) |- exists y. (p(y) \/ r(y))'
interpolant: ∃x. p(x)
∀x. (p(x) ∧ q(x)) ⊢ ∃x. p(x): valid, a checked derivation of LK (7 inferences)
∃x. p(x) ⊢ ∃y. (p(y) ∨ r(y)): valid, a checked derivation of LK (6 inferences)
$ deduxit interpolate --logic intuitionistic 'forall x. r(x, f(x)) |- forall x. exists y. r(x, y)'
interpolant: ∀x. ∃y. r(x, y)
∀x. r(x, f(x)) ⊢ ∀x. ∃y. r(x, y): valid, a checked derivation of LJ (4 inferences)
∀x. ∃y. r(x, y) ⊢ ∀x. ∃y. r(x, y): valid, a checked derivation of LJ (5 inferences)
```

deduxit reads the interpolant off a derivation without cuts by
Maehara's method, and checks it before it prints it: it decides both
halves again and compares the atoms (at first order the predicates,
functions and constants). A sequent that is not valid has no
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

`--format json` writes the whole answer as one document. `seq
normalize` writes it at wire level 21 (25 at first order): `{"version",
"deduxit", "form", "promise", "checked", "sequent", "result", "steps",
"checks"}`, the input and the normal form, each step's `rule`, `law`,
`before` and `after` (numbers of the result's `formulas`), the formulas
on each side after it, the clauses it `added` and a Skolem step's `term`
(a number of the result's terms), and each claim with its sequent and
outcome, a law's with its `step`. The other two write level 16. `seq translate` writes `{"version", "deduxit", "translation",
"target", "agreement", "sequent", "translated", "classical",
"decided"}`: both sequents, both outcomes with their derivations or
countermodels, and `agreement`, which is `valid`, `not_valid` or
`unknown`. `interpolate` writes `{"version", "deduxit", "logic",
"partition", "outcome", "interpolant"}`: the positions of the first half,
the sequent's outcome, and the interpolant's `text` with its `first` and
`second` halves, each a `sequent` with its `outcome`. `herbrand` writes level 28: `{"version", "deduxit", "verdict",
"sequent", "clausal", "depth", "ground", "instances", "refutation",
"derivation", "statistics"}`, the clausal form's `result` and `steps`
as a normal form has them, the instances as the sequent `ground` and
each instance's `clause`, `terms` and `formula`, the SAT solver's
outcome on them, and the derivation of LK.

In a program, `deduxit::ordinary::normal::normalize`,
`deduxit::ordinary::negative::compare` and
`deduxit::ordinary::interpolant::interpolate` return the same answers,
and the web bindings' `ordinary_translate` and `ordinary_interpolate`
answer the same JSON as the command for the other two. The protocol's
operations `ordinary.nnf`, `ordinary.cnf`, `ordinary.dnf`,
`ordinary.tseitin`, `ordinary.plaisted_greenbaum`, `ordinary.prenex`,
`ordinary.miniscope`, `ordinary.skolem`, `ordinary.herbrand` and
`ordinary.clausal` make the normal forms in a [workspace](workspace.md).
