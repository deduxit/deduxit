# Check a derivation in your own rules

`deduxit check` also checks a derivation against rules you give: a
calculus written as a JSON rule file. You write the rules once, as a
textbook states them, and every derivation in them is checked step by
step. The verdict reads "valid in the calculus given": the derivation
keeps to your rules, and nothing checks that the rules are sound.

## Check a derivation in a built-in calculus

deduxit ships five calculi: `LK`, `LJ` and `LM`, the sequent calculi of
classical, intuitionistic and minimal logic; `LL`, one-sided linear
logic; and `tableau`, signed tableaux for classical logic. A derivation
names its calculus, and one that names a built-in calculus needs no
flag. This one derives `a ∧ b ⊢ b ∧ a` in LK:

```console
$ cat > swap.json
> {"version": 15, "calculus": "LK",
>  "terms": ["a", "b", ["∧", 0, 1], ["∧", 1, 0]],
>  "nodes": [
>   {"sequent": [[1], [1]], "rule": "ax"},
>   {"sequent": [[2], [1]], "rule": "∧L₂", "at": [0], "premises": [0]},
>   {"sequent": [[0], [0]], "rule": "ax"},
>   {"sequent": [[2], [0]], "rule": "∧L₁", "at": [0], "premises": [2]},
>   {"sequent": [[2], [3]], "rule": "∧R", "at": [0], "premises": [1, 3]}]}
$ deduxit check swap.json
valid in the calculus given (LK): a ∧ b ⊢ b ∧ a, a derivation of 5 nodes
  0. b ⊢ b    ax
  1. a ∧ b ⊢ b    ∧L₂ 0
  2. a ⊢ a    ax
  3. a ∧ b ⊢ a    ∧L₁ 2
  4. a ∧ b ⊢ b ∧ a    ∧R 1, 3
$ deduxit check --format json swap.json
{"version":1,"verdict":"valid","calculus":"LK"}
```

A derivation is a list of terms and a list of nodes:
- `terms` holds the formulas. A term is an atom's name, or an array of a
  connective and the numbers of earlier terms: `["∧", 0, 1]` is `a ∧ b`.
- Each node holds its sequent, the rule that concludes it, and the
  numbers of its premises, which come before it. The last node is the
  conclusion.
- A sequent lists each zone's terms by number. LK has two zones, left
  and right of `⊢`.
- `at` gives the position of the formula the rule acts on, here the
  conjunction. A rule names the formulas it needs positions for, and
  `check` says when a node gives too many or too few.

## Write your own rules

A rule file declares the connectives, the zones of a sequent and the
rules. This one is natural deduction for implication and conjunction.
A sequent is one formula, and `→I` discharges a hypothesis:

```console
$ cat > nj.json
> {"version": 15, "name": "NJ",
>  "connectives": [{"name": "→", "arity": 2, "associativity": "right"},
>                  {"name": "∧", "arity": 2, "precedence": 1}],
>  "sequent": [{"name": "formula", "discipline": "multiset", "least": 1, "most": 1}],
>  "variables": {"formula": ["A", "B"]},
>  "rules": [
>   {"name": "→I", "premises": [{"formula": ["B"], "discharges": [{"formula": ["A"]}]}],
>    "conclusion": {"formula": [["→", "A", "B"]]}},
>   {"name": "→E", "premises": [{"formula": [["→", "A", "B"]]}, {"formula": ["A"]}],
>    "conclusion": {"formula": ["B"]}},
>   {"name": "∧I", "premises": [{"formula": ["A"]}, {"formula": ["B"]}],
>    "conclusion": {"formula": [["∧", "A", "B"]]}}]}
$ cat > pair.json
> {"version": 15, "calculus": "NJ",
>  "terms": ["a", "b", ["∧", 0, 1], ["→", 1, 2], ["→", 0, 3]],
>  "scopes": [0, 1],
>  "nodes": [
>   {"sequent": [[0]], "hypothesis": true, "scope": 1},
>   {"sequent": [[1]], "hypothesis": true, "scope": 2},
>   {"sequent": [[2]], "rule": "∧I", "premises": [0, 1], "scope": 2},
>   {"sequent": [[3]], "rule": "→I", "premises": [2], "scope": 1},
>   {"sequent": [[4]], "rule": "→I", "premises": [3]}]}
$ deduxit check --calculus nj.json pair.json
valid in the calculus given (NJ): a → b → a ∧ b, a derivation of 5 nodes
  0.   a    hypothesis
  1.     b    hypothesis
  2.     a ∧ b    ∧I 0, 1
  3.   b → a ∧ b    →I 2
  4. a → b → a ∧ b    →I 3
$ cat > escape.json
> {"version": 15, "calculus": "NJ",
>  "terms": ["a", "b", ["∧", 0, 1]],
>  "scopes": [0],
>  "nodes": [
>   {"sequent": [[1]], "hypothesis": true, "scope": 1},
>   {"sequent": [[0]], "hypothesis": true},
>   {"sequent": [[2]], "rule": "∧I", "premises": [1, 0]}]}
$ deduxit check --calculus nj.json escape.json; echo "exit $?"
invalid in the calculus given (NJ): node 2 (∧I, concluding a ∧ b): a premise lies in a scope that is closed here
exit 1
```

A hypothesis lies in a scope, as an assumption opens a subproof, and the
list indents each node by the depth of its scope. A rule that
discharges takes its premise from a scope just inside its own, and
every hypothesis of that scope must be one it discharges. A line inside
a closed subproof cannot be used outside it, which is what the last
derivation above does. A hypothesis in the outermost scope stays an assumption: the verdict
then says from how many assumptions the derivation shows its
conclusion.

## The rule file

- `connectives`: each with a `name` and an `arity`. `sorts` gives the
  sorts of its arguments and of the term it makes, where these are not
  formulas: a labelled formula `x : A` takes a label and a formula.
  `symbol`, `fixity` (`prefix`, `infix`, `postfix` or `function`),
  `precedence` (higher binds tighter) and `associativity` say how it is
  printed.
- `sorts`: the sorts beside `formula`, such as `label`.
- `sequent`: the zones in the order written. Each has a `discipline`:
  `set` (no order, no repeats), `multiset` (no order) or `sequence`. A
  zone may have a `mark` written before it (`⊢`), the `sort` of its
  members, and bounds `least` and `most` on its size: LJ is LK with at
  most one formula right of `⊢`.
- `variables`: the schematic variables of each sort; `contexts`: the
  variables that stand for the rest of a zone, such as `Γ`.
- `rules`: each a `name`, its `premises` and its `conclusion`. A sequent
  is an object keyed by zone. An entry is a variable, a context, or an
  array of a connective and its arguments. `["!", "Γ"]` is the context
  `Γ` with `!` on each member. A premise's `discharges` lists the
  hypotheses it discharges.
- Three conditions: `fresh` names variables that must be atoms new to
  the conclusion and to every hypothesis in force (an eigenvariable, a
  new label); `atomic` names variables that must be atoms (the axiom of
  G3, the closure of a tableau branch).

Every name must be declared, so a typo is an error, not a new variable.
A rule whose contexts cannot be told apart without a search is refused.
`deduxit::rules::SCHEMA` in the library is the format's JSON Schema, for
an editor, and the library's documentation of `rules::Calculus` and
`rules::Derivation` gives both forms in full.

## Exit status, JSON and limits

`check` exits with 0 for a valid derivation, 1 for an invalid one, 3
when a limit stopped the check, and 2 for a rule file that is no
calculus. `--format json` writes the verdict as `{"version",
"verdict", "calculus"}`, with the library's `error` and its `message`
for an invalid or unknown verdict, as in the first example.

The check never searches, so its work grows with the derivation's size.
It keeps to `--timeout`, `--work-limit` and `--memory-limit`, as the
check of a proof does, and the list of nodes to `--derivation-limit`.
The derivation is shown as text or JSON alone.
