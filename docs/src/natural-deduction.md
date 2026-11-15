# Check a proof in natural deduction

Natural deduction is the proof system most logic courses teach first.
deduxit ships its usual rule sets for classical, intuitionistic and
minimal logic as built-in calculi, and `deduxit check` checks a proof
in them step by step. A proof is a derivation in JSON, as in
[Check a derivation in your own rules](rules.md); `deduxit interact
--calculus NJ` builds one for you.

## Check a proof

This proof derives `b ∨ a` from the assumption `a ∨ b` in NJ, natural
deduction for intuitionistic logic. `∨E` takes the assumption apart: it
opens one subproof that assumes `a` and one that assumes `b`, and both
conclude `b ∨ a`:

```console
$ cat > swap.json
> {"version": 15, "calculus": "NJ",
>  "terms": ["a", "b", ["∨", 0, 1], ["∨", 1, 0]],
>  "scopes": [0, 0],
>  "nodes": [
>   {"sequent": [[2]], "hypothesis": true},
>   {"sequent": [[0]], "hypothesis": true, "scope": 1},
>   {"sequent": [[3]], "rule": "∨I₂", "premises": [1], "scope": 1},
>   {"sequent": [[1]], "hypothesis": true, "scope": 2},
>   {"sequent": [[3]], "rule": "∨I₁", "premises": [3], "scope": 2},
>   {"sequent": [[3]], "rule": "∨E", "premises": [0, 2, 4]}]}
$ deduxit check swap.json
valid in the calculus given (NJ): b ∨ a, a derivation of 6 nodes from 1 assumption
  0. a ∨ b    hypothesis
  1.   a    hypothesis
  2.   b ∨ a    ∨I₂ 1
  3.   b    hypothesis
  4.   b ∨ a    ∨I₁ 3
  5. b ∨ a    ∨E 0, 2, 4
```

Each node is a line of the proof: one formula, and the rule and the
lines it follows from, or `hypothesis`. A hypothesis lies in a scope,
which is a subproof, and the list indents the lines of each subproof.
`scopes` gives each scope's parent: here scopes 1 and 2 both lie in the
outermost scope 0. A rule that discharges a hypothesis takes its
premise from a subproof just inside its own scope, so `∨E` discharges
`a` in scope 1 and `b` in scope 2. A hypothesis in the outermost scope
is an assumption, and the verdict counts them.

The proof names its calculus, so `check` needs no flag. `--calculus`
names a built-in calculus or a rule file of your own, and the proof
must name the same calculus.

## Choose the rule set

| calculus | logic | how it differs |
|---|---|---|
| `NJ` | intuitionistic | Gentzen's rules, with `¬I`, `¬E` and ex falso `⊥E` |
| `NK` | classical | NJ and `RAA`: `⊥` derived from `¬A` gives `A` |
| `NM` | minimal | NJ without `⊥E`, so `⊥` is a formula with no rule |
| `NK-DNE` | classical | NJ and `¬¬E`: `¬¬A` gives `A` |
| `NK-LEM` | classical | NJ and `LEM`: `A ∨ ¬A` from nothing |
| `NJ-defined` | intuitionistic | NJ without `¬`: write `¬A` as `A → ⊥` |
| `NK-defined` | classical | NK without `¬`: `RAA` discharges `A → ⊥` |
| `NM-defined` | minimal | NM without `¬` |

Each rule file names its sources in its `description`: Gentzen's
"Untersuchungen über das logische Schließen" (1935) for NJ and NK with
`LEM`, Prawitz's "Natural Deduction" (1965) for `RAA` and for `¬A` as
`A → ⊥`, Johansson (1937) for minimal logic, Troelstra and
Schwichtenberg's "Basic Proof Theory" (2000), and Huth and Ryan's
"Logic in Computer Science" (2004) for `¬¬E`.

The rules of NJ, the premises in the order a proof lists them:

| rule | premises | concludes | discharges |
|---|---|---|---|
| `⊤I` | none | `⊤` | |
| `∧I` | `A`, `B` | `A ∧ B` | |
| `∧E₁`, `∧E₂` | `A ∧ B` | `A`, `B` | |
| `∨I₁`, `∨I₂` | `A`, `B` | `A ∨ B` | |
| `∨E` | `A ∨ B`, `C`, `C` | `C` | `A` in the second premise, `B` in the third |
| `→I` | `B` | `A → B` | `A` |
| `→E` | `A → B`, `A` | `B` | |
| `¬I` | `⊥` | `¬A` | `A` |
| `¬E` | `¬A`, `A` | `⊥` | |
| `↔I` | `B`, `A` | `A ↔ B` | `A` in the first premise, `B` in the second |
| `↔E₁` | `A ↔ B`, `A` | `B` | |
| `↔E₂` | `A ↔ B`, `B` | `A` | |
| `⊥E` | `⊥` | `A` | |

Every elimination takes its main premise first, the formula it takes
apart. A discharge may be empty: `→I` concludes `A → B` from `B` alone.
The other files have these rules but for the differences in the table
above: NM has no `⊥E`, and the files with `-defined` have no `¬`, `¬I`
or `¬E`, whose work `→I` and `→E` do.

A proof that uses a rule its calculus lacks is invalid, exit status 1.
Here `RAA` proves `a` from `¬¬a` in NK, and NJ refuses the same proof,
named as one of NJ:

```console
$ cat > dne.json
> {"version": 15, "calculus": "NK",
>  "terms": ["a", ["¬", 0], ["¬", 1], ["⊥"]],
>  "scopes": [0],
>  "nodes": [
>   {"sequent": [[2]], "hypothesis": true},
>   {"sequent": [[1]], "hypothesis": true, "scope": 1},
>   {"sequent": [[3]], "rule": "¬E", "premises": [0, 1], "scope": 1},
>   {"sequent": [[0]], "rule": "RAA", "premises": [2]}]}
$ deduxit check dne.json
valid in the calculus given (NK): a, a derivation of 4 nodes from 1 assumption
  0. ¬¬a    hypothesis
  1.   ¬a    hypothesis
  2.   ⊥    ¬E 0, 1
  3. a    RAA 2
$ cat > dne-nj.json
> {"version": 15, "calculus": "NJ",
>  "terms": ["a", ["¬", 0], ["¬", 1], ["⊥"]],
>  "scopes": [0],
>  "nodes": [
>   {"sequent": [[2]], "hypothesis": true},
>   {"sequent": [[1]], "hypothesis": true, "scope": 1},
>   {"sequent": [[3]], "rule": "¬E", "premises": [0, 1], "scope": 1},
>   {"sequent": [[0]], "rule": "RAA", "premises": [2]}]}
$ deduxit check dne-nj.json; echo "exit $?"
invalid in the calculus given (NJ): node 3 (RAA, concluding a): the calculus has no rule of this name
exit 1
```

## Build a proof step by step

`deduxit interact --calculus NJ` builds a proof from its conclusion up.
`--assume` gives the assumptions. `apply` applies a rule backwards and
asks for what the goal leaves open by name: `∨E` must be told the
disjunction it takes apart. `assume` closes a goal by a hypothesis in
force:

```console
$ deduxit interact --calculus NJ --assume "a \/ b" "b \/ a"
> apply 0 ∨E A=a B=b
opened goal 1: 0: a ∨ b
    hypotheses: a ∨ b
opened goal 2: 0: b ∨ a
    hypotheses: a; a ∨ b
opened goal 3: 0: b ∨ a
    hypotheses: b; a ∨ b
> assume 1
closed
> apply 2 ∨I₂
opened goal 4: 0: a
    hypotheses: a; a ∨ b
> assume 4
closed
> apply 3 ∨I₁
opened goal 5: 0: b
    hypotheses: b; a ∨ b
> assume 5
closed; no goal is open: `proof` checks the proof
> proof swap.json
valid in the calculus given (NJ): b ∨ a; written to swap.json
```

`proof FILE` writes the proof that `check` reads. The session's
commands are in [Check a derivation in your own rules](rules.md#build-a-derivation-step-by-step).

## What the rule sets prove

A proof exists in NJ exactly when `deduxit prove --logic
intuitionistic` says the sequent is valid, in NK exactly when it is
classically valid, and in NM exactly when it is valid in minimal logic.
The same holds for each variant. deduxit's tests check this both ways:
every rule is valid in its logic and the classical rules and ex falso
are not valid in a weaker one, and the proofs the ordinary suite finds
for valid sequents, rewritten into each rule set, pass the check. When
a proof will not come together, `deduxit prove --logic` tells you
whether the sequent is valid at all, and gives a countermodel when it
is not.
