# Decide modal logic

Modal logic adds two prefixes to ordinary logic: `□A`, "necessarily A",
and `◇A`, "possibly A". A Kripke model gives them their meaning: it has
worlds, each world sees some worlds, and `□A` holds at a world when `A`
holds at every world it sees, `◇A` when `A` holds at one of them. Which
sequents are valid depends on what the seeing may be: a system names
the conditions, such as S4, where every world sees itself and whatever
the worlds it sees see.

Name the system with `--system` and give `prove` a formula or a
sequent, one at a time (a batch of many is refused):

```console
$ deduxit prove --system S4 '[]p -> [][]p'
valid in S4: ⊢ □p → □□p, a checked derivation of 6 inferences in G3S4
  0. w0 : □p, w0 R w1, w1 R w2, w2 : p, w0 R w2 ⊢ w0 : □p → □□p, w0 : □□p, w1 : □p, w2 : p    ax
  1. w0 : □p, w0 R w1, w1 R w2, w0 R w2 ⊢ w0 : □p → □□p, w0 : □□p, w1 : □p, w2 : p    L□ 0
  2. w0 : □p, w0 R w1, w1 R w2 ⊢ w0 : □p → □□p, w0 : □□p, w1 : □p, w2 : p    Trans 1
  3. w0 : □p, w0 R w1 ⊢ w0 : □p → □□p, w0 : □□p, w1 : □p    R□ 2
  4. w0 : □p ⊢ w0 : □p → □□p, w0 : □□p    R□ 3
  5. ⊢ w0 : □p → □□p    R→ 4
$ deduxit prove --system K '[]p -> [][]p'; echo "exit $?"
not valid in K: a checked countermodel of 3 worlds refutes ⊢ □p → □□p at world 0
world 0 (where the sequent fails): forces nothing; sees 1
world 1: forces p; sees 2
world 2: forces nothing; sees no world
exit 1
```

## Read the answer

- `valid`: the sequent holds at every world of every model of the
  system. deduxit shows its derivation and checks it before it is shown.
  The exit status is 0.
- `not valid`: some model of the system has a world where every formula
  left of `⊢` is true and every formula right of it false. deduxit shows
  that model and checks it first. Above, world 0 sees world 1, where `p`
  holds, so `□p` is true at world 0; world 1 sees world 2, where `p` does
  not hold, so `□□p` is false at world 0. The exit status is 1.
- `unknown`: a limit, such as the time limit, stopped the search before
  it decided. The exit status is 3, and the line says which limit it
  was.

The derivation is in G3S4, the labelled sequent calculus of S4: each
formula carries the world it is about (`w0 : □p`), and `w0 R w1` says
that world 0 sees world 1. Its rules are those of ordinary logic, `L□`,
`R□`, `L◇`, `R◇` for the prefixes, and one rule per condition of the
system (`Trans` above). Each line is a sequent, the rule that made it
and the lines it was made from; the last line is the sequent you gave,
every formula at world 0.

## Write a formula

The syntax is that of [ordinary logic](ordinary-logic.md) with the
prefixes `[]`/`□` and `<>`/`◇`, which bind as tightly as `~`: `[]p -> q`
is `(□p) → q`, and `[]~p` is `□(¬p)`.

```console
$ deduxit prove --system K -q '~<>p <-> []~p'
valid in K: ⊢ ¬◇p ↔ □¬p, a checked derivation of 11 inferences in G3K
$ deduxit prove --system S5 -q '<>[]p -> []<>p'
valid in S5: ⊢ ◇□p → □◇p, a checked derivation of 8 inferences in G3S5
$ deduxit prove --system S4 -q '<>[]p -> []<>p'; echo "exit $?"
not valid in S4: a checked countermodel of 3 worlds refutes ⊢ ◇□p → □◇p at world 0
exit 1
```

## The systems

A system is the set of conditions its models meet. K has none; each
other system adds some of these:

| condition | every world… | axiom | rule |
|---|---|---|---|
| serial | sees some world | D: `□A → ◇A` | `Ser` |
| reflexive | sees itself | T: `□A → A` | `Ref` |
| symmetric | is seen by every world it sees | B: `A → □◇A` | `Sym` |
| transitive | sees every world the worlds it sees see | 4: `□A → □□A` | `Trans` |
| euclidean | sees only worlds that see each other and themselves | 5: `◇A → □◇A` | `Eucl` |

The fifteen systems these give are K, D, T, KB, KDB, B, K4, KD4, S4, K5,
KD5, K45, KD45, KB5 and S5. `--system` takes those names and any other
name of one: K followed by the axioms' letters (`KT4` is S4, `KT` is T,
`KTB` is B), in any case.

```console
$ deduxit prove --system kt -q '[]p |- p'
valid in T: □p ⊢ p, a checked derivation of 3 inferences in G3T
```

## Check an outcome or a model

`--format json` writes the outcome, with the derivation or the
countermodel. `check` reads it and checks its evidence again:

```console
$ deduxit prove --system K --format json --output outcome.json '[]p -> [][]p'; echo "exit $?"
exit 1
$ deduxit check outcome.json
not valid in K: ⊢ □p → □□p, its countermodel checked
```

A derivation alone, the outcome's `derivation`, is checked by `check`
too, which takes its calculus from the name it carries (`G3S4`), or by
`check --calculus G3S4`.

`check --system` checks a Kripke model you write against a sequent: the
model's JSON lists the worlds, each with the atoms it forces and the
worlds it sees. The check says whether the sequent holds at every world,
or at the world `--world` names, and where it fails, the value there of
each formula and the world that shows it. The model must meet the
system's conditions as written: a model of S4 lists every world's arrow
to itself. Otherwise the check names the first condition it fails, exit
status 2:

```console
$ echo '{"worlds": [{"atoms": [], "arrows": [1]}, {"atoms": ["p"], "arrows": [2]}, {"atoms": []}]}' > model.json
$ deduxit check --system K --sequent '[]p |- [][]p' model.json; echo "exit $?"
fails in K at world 0: □p ⊢ □□p; every formula left of ⊢ is true there and every formula right of it false
  □p is true
  □□p is false: world 1 shows it
world 0 (where the sequent fails): forces nothing; sees 1
world 1: forces p; sees 2
world 2: forces nothing; sees no world
exit 1
$ deduxit check --system T --sequent '[]p |- [][]p' model.json; echo "exit $?"
error: the model is no model of T: not reflexive: world 0 does not see itself
exit 2
```

## Draw a derivation or a model

`--format latex`, `typst` and `svg` draw the derivation as a tree, and
`svg`, `png` and `pdf` draw a countermodel, an arrow from each world to
each world it sees:

```console
$ deduxit prove --system S4 --output proof.tex '[]p |- [][]p'
$ deduxit prove --system K --output model.svg '[]p |- [][]p'; echo "exit $?"
exit 1
```

## Limits

The search always ends: it makes a bounded number of worlds for each
sequent, and stops exploring a world that holds what an earlier one
holds. On a large sequent that can still take long. The time limit (`--timeout`, 2 seconds by
default) and the work limit (`--work-limit`) stop it before then, with
`unknown`:

```console
$ deduxit prove --system S4 --work-limit 1 '[](p -> []p) -> (p -> []p)'; echo "exit $?"
unknown in S4: the work limit of 1 was reached after …; raise it with --work-limit N
exit 3
```

`--stats` counts the worlds the search made, the splits it took, the
worlds it did not explore again, its units of work and the inferences of
the derivation.
