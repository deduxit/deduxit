# Prove in first-order logic

First-order logic adds quantifiers over terms: `forall x. A` (`∀x. A`)
says that `A` holds for every `x`, and `exists x. A` (`∃x. A`) that it
holds for some `x`. `prove` decides such a sequent as it decides any
other. A proof names, for each `∃` it opens, the term it chose for `x`,
called the witness:

<!-- readme-check: machine -->
```console
$ deduxit prove "p(a) & p(b), q(b) |- exists x. (p(x) * q(x))"
provable (MALL1, classical, focus engine)
     ───────────── ax   ───────────── ax
     ⊢ ~p(b), p(b)      ⊢ ~q(b), q(b)
     ──────────────────────────────── ⊗
       ⊢ ~p(b), ~q(b), p(b) ⊗ q(b)
    ───────────────────────────────── ∃
    ⊢ ~p(b), ~q(b), ∃x. (p(x) ⊗ q(x))
───────────────────────────────────────── ⊕₂
⊢ ~p(a) ⊕ ~p(b), ~q(b), ∃x. (p(x) ⊗ q(x))
```

The `∃` step fills `x` with the witness `b`. The proof passes the checker
before it is shown, as every proof deduxit reports does.

## Write a first-order sequent

- An atom may be a predicate applied to terms, as in `p(x, f(a))`.
- A name bound by an enclosing quantifier is a variable. Any other name
  is a constant, or a function symbol when a bracket follows it.
- A symbol has one arity (number of arguments) throughout the sequent,
  and a name is a predicate or a function symbol, not both.
- A quantifier binds loosest. Its scope runs to the right as far as it
  can: to a closing bracket, a comma, the turnstile or the end.
- Every sequent is closed: it has no free variable.

`seq print` writes the one-sided form, where a negation swaps the
quantifiers. `seq fragment` names a first-order fragment with a `1` after
it, as in `MLL1`. A sequent whose atoms have arguments but which has no
quantifier is propositional, so its fragment has no `1` and every engine
decides it:

<!-- readme-check: machine -->
```console
$ deduxit seq print "forall x. p(x) -o q(x), p(a) |- exists y. q(y)"
⊢ ∃x. (p(x) ⊗ ~q(x)), ~p(a), ∃y. q(y)
$ deduxit seq print "|- ∀x y. ~r(x, f(y)) ⅋ r(x, f(y)), (exists z. p(z))^"
⊢ ∀x. (∀y. (~r(x, f(y)) ⅋ r(x, f(y)))), ∀z. ~p(z)
$ deduxit seq fragment "forall x. p(x) |- p(a)"
MLL1
$ deduxit seq fragment "p(a), !p(a) -o q(f(a)) |- q(f(a))"
MELL
```

The syntax chapter lists every connective ([The syntax](syntax.md)).

## Read the verdict

Without the exponentials `!` and `?`, the search decides every sequent,
since MLL1 and MALL1 are decidable: "unprovable" means that no proof
exists. The order of quantifiers matters. `∀x. ∃y. r(x, y)` does not give
`∃y. ∀x. r(x, y)`, while the converse holds:

<!-- readme-check: machine -->
```console
$ deduxit prove "p(a), p(a) -o q(f(a)) |- q(f(a))"
provable (MLL, classical, focus engine)
───────────── ax   ─────────────────── ax
⊢ ~p(a), p(a)      ⊢ ~q(f(a)), q(f(a))
────────────────────────────────────── ⊗
  ⊢ ~p(a), p(a) ⊗ ~q(f(a)), q(f(a))
$ deduxit prove "forall x. exists y. r(x, y) |- exists y. forall x. r(x, y)"; echo "exit $?"
unprovable (MLL1, classical, focus engine): the search was exhaustive
exit 1
$ deduxit prove -i "exists y. forall x. r(x, y) |- forall x. exists y. r(x, y)"
provable (IMLL1, intuitionistic, two-sided engine)
          ───────────────── ax
          r(b, a) ⊢ r(b, a)
        ───────────────────── ∃R
        r(b, a) ⊢ ∃y. r(b, y)
      ───────────────────────── ∀L
      ∀x. r(x, a) ⊢ ∃y. r(b, y)
   ─────────────────────────────── ∀R
   ∀x. r(x, a) ⊢ ∀x. (∃y. r(x, y))
───────────────────────────────────── ∃L
∃y. (∀x. r(x, y)) ⊢ ∀x. (∃y. r(x, y))
```

The focused engine decides a sequent with a quantifier in the classical
modes, with Mix and in affine mode too. The two-sided engine decides it
in the intuitionistic modes, affine included. Both find witnesses the
same way:
- The witness of each `∃` stays open as a variable until an axiom
  matches two atoms. The match fills it in (unification), never with a
  term that holds the variable itself.
- Each `∀` opens its formula with an eigenvariable: a fresh constant
  that stands for any term. It is kept out of every witness chosen
  before it.
- Where one premise's witness fails the other premise, the search asks
  the first premise for another witness, and resumes that premise's
  search where it stopped.

A refutation may count atoms, as it does without quantifiers. A
predicate with a bound variable in one of its atoms is counted over all
its atoms, since an instance may be any of them:

<!-- readme-check: machine -->
```console
$ deduxit prove "forall x. p(x), p(a) |- p(b)"; echo "exit $?"
unprovable (MLL1, classical, focus engine): p(…) occurs 1 more time on the left than on the right, every atom of the predicate counted, since one has a bound variable, so they cannot all meet in axioms; with -a (affine), an unused hypothesis is allowed
exit 1
```

## With exponentials

First-order linear logic with exponentials is undecidable. The copy
bound and the three verdicts work as they do without quantifiers ([Decide
a sequent](commands.md)): `--copies N` bounds how often a `?` formula is
copied on one branch, and `--copies none`, the default, deepens the bound
while the time limit lasts. A sequent the search can neither prove nor
refute ends at the time limit as unknown.

<!-- readme-check: machine -->
```console
$ deduxit prove --copies 2 "p(z), !forall x. (p(x) -o p(s(x))) |- p(s(s(z)))" | head -1
provable (MELL1, classical, focus engine)
$ deduxit prove -i --copies 6 "!(forall x. (p(x) -o p(x))), !(q -o r), !(r -o q), p(e) |- r"; echo "exit $?"
unprovable (IMELL1, intuitionistic, two-sided engine): the search was exhaustive
exit 1
```

The second sequent is refuted by the loop check. A sequent met again on
its own branch, as the same instance, is a loop, so a hypothesis under
`!` that leads back to where it started is refuted instead of being cut
at the copy bound. The memo remembers a failure for every renaming of
the witnesses it leaves open. `--memo-limit 0` switches both off, and the
second sequent is then unknown at its copy bound.

Two bounds of their own cut a branch where the search would go further.
A search that cut one is unknown, not unprovable:
- `--witness-depth N` (64 by default) is the deepest term a witness is
  filled in with.
- `--instances N` (65536 by default) is the most instances of quantified
  formulas the search holds at once.

<!-- readme-check: machine -->
```console
$ deduxit prove --witness-depth 2 "|- exists x. p(x), ~p(f(f(f(a))))" | sed 's/after .*;/after …;/'; echo "exit $?"
unknown (MLL1, classical, focus engine): the witness depth of 2 was reached after …; raise it with --witness-depth N
exit 3
```

For the same verdict on every machine, bound the search with `--work-limit
N` or `--copies N` and lift the time limit with `--timeout none`.
`--deterministic` also makes the proof and `--stats` the same on every run.

## Statistics and threads

On a first-order sequent `--stats` adds two counters: the unifications of
two literals tried, and the witnesses bound. In JSON they are
`unifications` and `bindings`, written only where they are not zero.

<!-- readme-check: machine -->
```console
$ deduxit prove -q --deterministic --stats "p(a), p(b) & q(c) |- exists x. (p(x) * p(b))"
provable (MALL1, classical, focus engine)
stable sequents visited: 2
from the memo: 0
memo entries at most: 0
splits examined: 6
unifications tried: 5
metavariables bound: 3
units of work: 36
time: 391.45µs
```

On several threads (`--jobs`, and the pool that joins the first thread
by default), a first-order sequent is searched as a propositional one is:
the threads share the choices nearest the root. Only a choice where no
rule around it may ask its premise for another witness is shared:

<!-- readme-check: machine -->
```console
$ deduxit prove -q -j 4 "p(a1), p(a2), p(a3), p(a4), q(b) |- (exists x. (p(x) * q(x))) * (exists x. p(x)) * (exists x. p(x)) * (exists x. p(x))"; echo "exit $?"
unprovable (MLL1, classical, focus engine): the search was exhaustive
exit 1
```

## Prove step by step

In `interact` ([Prove step by step](interactive.md)) the quantifier
rules come in two kinds:
- `∀R` and `∃L` (`∀` one-sided) open the formula with a fresh
  eigenvariable, written `a`, `b`, … (a letter no symbol of the sequent
  takes).
- `∃R` and `∀L` (`∃` one-sided) take their witness after the rule's
  name. The witness is a term of the sequent's function symbols and of
  the eigenvariables of the `∀R` and `∃L` steps below the goal. With no
  term, the witness stays open as `$1`, `$2`, … (a `$` that no formula
  holds), and a later axiom fills it by unification, or `close` by the
  search.

An open witness never takes an eigenvariable made after it, which would
stand in the conclusion of its own step. This is the eigenvariable
condition, and `apply` reports a breach by the eigenvariable's name.
`undo` of the step that filled a witness leaves it open again. `save`
keeps the witnesses and what fills them (the session's JSON at wire level
7, its `instances`, with each step's mark), so that `undo` in a session
resumed from its file drops what the step made, as in the session saved:

```console
$ deduxit interact -i "exists y. forall x. r(x, y) |- forall x. exists y. r(x, y)"
> apply 0 0 ∃L
opened goal 1: 1: ∀x. r(x, a) ⊢ 0: ∀x. (∃y. r(x, y))
> apply 1 0 ∀R
opened goal 2: 0: ∀x. r(x, a) ⊢ 1: ∃y. r(b, y)
> rules 2 0
∀L (with a witness or none)
> apply 2 0 ∀L
opened goal 3: 1: r($1, a) ⊢ 0: ∃y. r(b, y)
> apply 3 0 ∃R a
opened goal 4: 0: r($1, a) ⊢ 1: r(b, a)
> apply 4 0 ax
closed; no goal is open: `proof` checks the proof
> undo
reopened goal 4: 0: r($1, a) ⊢ 1: r(b, a)
> close
goal 4: provable (IMLL1, intuitionistic, two-sided engine)
no goal is open: `proof` checks the proof
> proof
valid proof (intuitionistic)
          ───────────────── ax
          r(b, a) ⊢ r(b, a)
        ───────────────────── ∃R
        r(b, a) ⊢ ∃y. r(b, y)
      ───────────────────────── ∀L
      ∀x. r(x, a) ⊢ ∃y. r(b, y)
   ─────────────────────────────── ∀R
   ∀x. r(x, a) ⊢ ∀x. (∃y. r(x, y))
───────────────────────────────────── ∃L
∃y. (∀x. r(x, y)) ⊢ ∀x. (∃y. r(x, y))
```

A cut formula, like a witness, may use only the sequent's symbols (with
their arities) and the eigenvariables of the `∀R` and `∃L` steps below
the goal, quantifiers included. A cut on `p(a)` is then the lemma about
the `a` at hand, the instance of `∀x. p(x)`'s body. `cut` refuses a name
the sequent lacks or has with another arity, and an eigenvariable of
another branch, by name:

```console
$ printf 'apply 0 2 forallR\ncut 1 "r(a)" 1\ncut 1 "p(a)" 1\nclose\nproof\n' | deduxit interact -i "forall x. (p(x) -o q(x)), forall x. p(x) |- forall y. q(y)"
opened goal 1: 0: ∀x. (p(x) ⊸ q(x)), 1: ∀x. p(x) ⊢ 2: q(a)
error: the cut formula names r, which the sequent does not have with this arity and role: in a session with quantifiers, a cut formula may use only the sequent's symbols and the goal's eigenvariables
opened goal 2: 0: ∀x. p(x) ⊢ 1: p(a)
opened goal 3: 0: ∀x. (p(x) ⊸ q(x)), 2: p(a) ⊢ 1: q(a)
goal 2: provable (IMLL1, intuitionistic, two-sided engine)
goal 3: provable (IMLL1, intuitionistic, two-sided engine)
no goal is open: `proof` checks the proof
valid proof (intuitionistic)
                      ─────────── ax   ─────────── ax
                      p(a) ⊢ p(a)      q(a) ⊢ q(a)
  ─────────── ax      ──────────────────────────── ⊸L
  p(a) ⊢ p(a)           p(a), p(a) ⊸ q(a) ⊢ q(a)
─────────────── ∀L   ────────────────────────────── ∀L
∀x. p(x) ⊢ p(a)      ∀x. (p(x) ⊸ q(x)), p(a) ⊢ q(a)
─────────────────────────────────────────────────── cut
        ∀x. (p(x) ⊸ q(x)), ∀x. p(x) ⊢ q(a)
      ────────────────────────────────────── ∀R
      ∀x. (p(x) ⊸ q(x)), ∀x. p(x) ⊢ ∀y. q(y)
```

## Check a first-order proof

The JSON form of a first-order sequent is written at wire level 7. Beside
the propositional form's keys it has:
- `symbols`: each symbol's name with its arity;
- `fo`: the first-order terms, `{"B": n}` a bound variable (`0` for the
  innermost quantifier's) and `{"F": [symbol, [terms]]}` an application;
- the atoms as `{"P": [symbol, [terms]]}`, and a quantifier as
  `{"∀": body}` or `{"∃": body}`;
- `binders`: the quantifiers' variable names, one per quantifier in the
  order of the terms.

<!-- readme-check: machine -->
```console
$ deduxit seq json "forall x. p(x) |- p(a)"
{"version":7,"terms":[{"D":0},{"∃":0},{"V":1}],"roots":[1,2],"atoms":[{"P":[0,[0]]},{"P":[0,[1]]}],"symbols":[["p",1],["a",0]],"fo":[{"B":0},{"F":[1,[]]}],"binders":["x"],"antecedents":1}
```

A proof of a first-order sequent names, for every member under
quantifiers, the terms its quantifiers were opened with:
- a `∀` step its eigenvariable, a fresh constant the proof numbers, as
  `{"∀": [member, eigenvariable, premise]}`;
- a `∃` step its witness, a closed term, as `{"∃": [member, term,
  premise]}`.

The proof's `instances` list each member past the sequent's occurrences
as `[occurrence, frame]`, and each frame as `[parent, term]`: the
innermost quantifier's term first, `null` past the outermost. The proof's
own terms follow the sequent's `fo` (an eigenvariable is `{"E": k}`).

`check` checks such a proof with the eigenvariable condition. An
eigenvariable that a `∀` step introduces is introduced by that step
alone, and only witnesses above it name it. One that no step introduces
is a constant outside the signature, a witness like any other. The
derivation writes every instance with its terms, the eigenvariables named
after letters no symbol takes:

<!-- readme-check: machine -->
```console
$ echo '{"version":7,"sequent":{"terms":[{"D":0},{"∃":0},{"V":1}],"roots":[1,2],"atoms":[{"P":[0,[0]]},{"P":[0,[2]]}],"symbols":[["p",1],["a",0],["f",1]],"fo":[{"B":0},{"F":[1,[]]},{"F":[2,[1]]}],"binders":["x"],"antecedents":1},"nodes":[{"ax":[3,2]},{"∃":[0,2,0]}],"instances":{"members":[[1,0]],"frames":[[null,2]],"terms":[]}}' | deduxit check
valid proof of ⊢ ∃x. ~p(x), p(f(a)) (classical)
─────────────────── ax
⊢ p(f(a)), ~p(f(a))
──────────────────── ∃
⊢ ∃x. ~p(x), p(f(a))
```

## Eliminate cuts

`eliminate` ([Eliminate cuts](cut.md)) eliminates the cuts of a
first-order proof as it does a propositional one, in every mode that has
the cut. A cut may be on a formula with quantifiers or on an instance of
one. The key case of `∀` and `∃` cuts the instance of the `∀` formula's
body at the `∃` step's witness: the witness takes the eigenvariable's
place throughout the `∀` step's premise, its instances and the witnesses
above included. Where another cut still reads that premise, this happens
in a copy, and a copied `∀` step takes an eigenvariable of its own. A
copy is counted against `--node-limit` before it is made. Every step's
line writes the instance it cut with its terms.

Here `cut.json` proves `∀x. p(x) ⊢ p(a)` through the cut formula
`∀x. p(x)`, by a `∀` step against a `∃` step with the witness `a`:

```console
$ echo '{"version":7,"sequent":{"terms":[{"D":0},{"∃":0},{"V":1},{"V":0},{"∀":3}],"roots":[1,2],"atoms":[{"P":[0,[0]]},{"P":[0,[1]]}],"symbols":[["p",1],["a",0]],"fo":[{"B":0},{"F":[1,[]]}],"binders":["x","x"],"antecedents":1,"cuts":[4]},"nodes":[{"ax":[7,8]},{"∃":[0,2,0]},{"∀":[3,0,1]},{"ax":[9,2]},{"∃":[5,1,3]},{"cut":[3,2,4]}],"instances":{"members":[[1,0],[4,0],[6,1]],"frames":[[null,2],[null,1]],"terms":[{"E":0}]}}' > cut.json
$ deduxit eliminate cut.json
1. the key case of ∀ and ∃ on the cut of ∀x. p(x)
2. the axiom case on the cut of p(a)
cut-free proof of ⊢ ∃x. ~p(x), p(a) (classical) after 2 steps
  ───────────── ax
  ⊢ p(a), ~p(a)
───────────────── ∃
⊢ ∃x. ~p(x), p(a)
$ deduxit eliminate -i --steps=1 cut.json
1. the key case of ∀ and ∃ on the cut of ∀x. p(x)
proof of ∀x. p(x) ⊢ p(a) (intuitionistic) with 1 cut left after 1 step: the elimination took the most steps allowed, 1; --steps N allows more
  ─────────── ax
  p(a) ⊢ p(a)
─────────────── ∀L   ─────────── ax
∀x. p(x) ⊢ p(a)      p(a) ⊢ p(a)
──────────────────────────────── cut
        ∀x. p(x) ⊢ p(a)
```

## What quantifiers do not have yet

- Proof nets refuse a quantifier, with the exit status 2 of an error.
- A Rocq certificate of a first-order proof is written for deduxit's own
  Rocq library ([Check a certificate in Rocq](rocq.md)); NanoYalla's
  kernel has no quantifiers.
- A first-order proof has no lambda-term ([Read a proof as a
  program](terms.md#what-has-no-term)).

```console
$ deduxit prove --net "forall x. p(x) |- p(a)"; echo "exit $?"
error: proof nets exist for MLL without units only, not for MLL1
exit 2
```
