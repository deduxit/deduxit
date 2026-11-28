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

## Draw a proof

`check` writes a proof as a tree by default, and as a Fitch proof with
`--layout fitch`. This proof puts the one above inside `→I`, which
discharges the assumption `a ∨ b`:

```console
$ cat > or.json
> {"version": 15, "calculus": "NJ",
>  "terms": ["a", "b", ["∨", 0, 1], ["∨", 1, 0], ["→", 2, 3]],
>  "scopes": [0, 1, 1],
>  "nodes": [
>   {"sequent": [[2]], "hypothesis": true, "scope": 1},
>   {"sequent": [[0]], "hypothesis": true, "scope": 2},
>   {"sequent": [[3]], "rule": "∨I₂", "premises": [1], "scope": 2},
>   {"sequent": [[1]], "hypothesis": true, "scope": 3},
>   {"sequent": [[3]], "rule": "∨I₁", "premises": [3], "scope": 3},
>   {"sequent": [[3]], "rule": "∨E", "premises": [0, 2, 4], "scope": 1},
>   {"sequent": [[4]], "rule": "→I", "premises": [5]}]}
$ deduxit check or.json --layout fitch
valid in the calculus given (NJ): a ∨ b → b ∨ a, a derivation of 7 nodes
1 │ │ a ∨ b
  │ ├───
2 │ │ │ a
  │ │ ├───
3 │ │ │ b ∨ a       ∨I₂ 2
4 │ │ │ b
  │ │ ├───
5 │ │ │ b ∨ a       ∨I₁ 4
6 │ │ b ∨ a         ∨E 1, 2–3, 4–5
7 │ a ∨ b → b ∨ a   →I 1–6
$ deduxit check or.json --format latex --standalone --output or.tex
$ deduxit check or.json --layout fitch --format typst --standalone --output or.typ
$ deduxit check or.json --layout fitch --format png --output or.png
valid in the calculus given (NJ): a ∨ b → b ∨ a, a derivation of 7 nodes
```

`--format png` and `pdf` render the same drawing, the verdict on
standard error.

Each line has a number, its formula, and the rule with the lines it
follows from. A subproof's lines stand together, its hypotheses first
above a short bar, and a line at its left runs down to its last line.
A rule that discharges cites the subproof by its first and last line:
`→I 1–6` takes the subproof that assumes `a ∨ b` and ends in `b ∨ a`.
The lines need not be in the file's order: `check` puts each subproof
together, its last line the one the rule takes. A proof whose lines
cannot be put in such an order, such as one where a line inside a
subproof uses the subproof's last line, is left out with the reason,
and the verdict stands.

`--format latex`, `typst` and `svg` draw either layout, as the last two
commands do. In the tree each hypothesis stands alone at a leaf. The
hypotheses a rule discharges are in brackets with a number, and the
rule carries the same number: `[a]¹` and `[b]²` above `∨E¹˒²`, and
`[a ∨ b]³` above `→I³`. A Fitch proof is written for the LaTeX package
`fitch` and the Typst package `fitchcraft`.

`--style fitch.hypothesis=Hyp` writes `Hyp` as a hypothesis's
justification, as some textbooks do.

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

`deduxit interact --calculus NJ` builds a proof in two directions, as
on paper:
- `derive G RULE L…` works forwards: it applies a rule to lines in
  force and adds the conclusion as a new line, which later steps cite.
- `apply G RULE` works backwards from goal G. A rule that discharges a
  hypothesis (`→I`, `∨E`, `¬I`, `↔I`, `RAA`) is applied this way, and it
  opens its subproof: the hypothesis becomes a new line, in force only
  inside it.
- `assume G` closes a goal by the line in force equal to it, and
  `assume G L` by line L.

Each goal lists the lines in force with their numbers, which never
change. A line of an enclosing subproof is in force in the subproofs
inside it, so no reiteration is needed:

```console
$ deduxit interact --calculus NJ "(a -> b) -> (b -> c) -> a -> c"
> apply 0 →I
opened goal 1: 0: (b → c) → a → c
    lines: 1. a → b
> apply 1 →I
opened goal 2: 0: a → c
    lines: 1. a → b; 2. b → c
> apply 2 →I
opened goal 3: 0: c
    lines: 1. a → b; 2. b → c; 3. a
> derive 3 →E 1 3
line 4: b
> derive 3 →E 2 4
line 5: c
> assume 3
closed; no goal is open: `proof` checks the proof
> proof
valid in the calculus given (NJ): (a → b) → (b → c) → a → c
  0.   a → b    hypothesis
  1.       a    hypothesis
  2.       b    →E 0, 1
  3.     b → c    hypothesis
  4.       c    →E 3, 2
  5.     a → c    →I 4
  6.   (b → c) → a → c    →I 5
  7. (a → b) → (b → c) → a → c    →I 6
```

A step names what its lines leave open as `NAME=TEXT`. `∨I₁` from `a`
needs `B=b` to conclude `a ∨ b`. Backwards, `∨E` must be told the
disjunction it takes apart, and it opens two subproofs:

```console
$ deduxit interact --calculus NJ --assume "a \/ b" "b \/ a"
> apply 0 ∨E A=a B=b
opened goal 1: 0: a ∨ b
    lines: 1. a ∨ b
opened goal 2: 0: b ∨ a
    lines: 1. a ∨ b; 2. a
opened goal 3: 0: b ∨ a
    lines: 1. a ∨ b; 3. b
> assume 1
closed
> derive 2 ∨I₂ 2 A=b
line 4: b ∨ a
> assume 2 4
closed
> derive 3 ∨I₁ 3 B=a
line 5: b ∨ a
> assume 3 5
closed; no goal is open: `proof` checks the proof
> proof swap.json
valid in the calculus given (NJ): b ∨ a; written to swap.json
```

`proof FILE` writes the proof that `check` reads. A line of a subproof
that is closed is out of force: citing it is refused, and so is a line
that is not the goal. `undo` takes back the last step, a forward one
too, and `help` lists every command.

## Let the search close a goal

`close G` lets deduxit's search decide goal G, and `close` every open
goal. It works in the natural deduction rule sets deduxit ships (NJ,
NK, NM and their variants). The search proves the goal, with the lines
in force as hypotheses, in the sequent calculus of the rule set's
logic. It then rewrites that proof into the rule set and takes it step
by step, each step checked as yours are. The proof is normal: no
introduction rule is followed by the elimination of its connective.
`undo` takes it back a step at a time:

```console
$ deduxit interact --calculus NJ "(a -> b) -> (b -> c) -> a -> c"
> apply 0 →I
opened goal 1: 0: (b → c) → a → c
    lines: 1. a → b
> close 1
goal 1: valid in intuitionistic logic: closed by 7 steps of NJ
no goal is open: `proof` checks the proof
> show
  0.     b → c    hypothesis
  1.   a → b    hypothesis
  2.       a    hypothesis
  3.       b    →E 1, 2
  4.       c    →E 0, 3
  5.     a → c    →I 4
  6.   (b → c) → a → c    →I 5
  7. (a → b) → (b → c) → a → c    →I 6
```

A goal with no proof in the rule set's logic stays open:

```console
$ deduxit interact --calculus NJ "((a -> b) -> a) -> a"
> close
goal 0: not valid in intuitionistic logic: the goal has no proof in NJ
```

In NK the same goal closes, by `RAA`.

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
