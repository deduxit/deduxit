# Eliminate cuts

A cut proves a formula C once and then uses it as a hypothesis: from
`Γ ⊢ C` and `Δ, C ⊢ D` it concludes `Γ, Δ ⊢ D`. Cut elimination
rewrites a proof with cuts into a proof of the same sequent without
them. `deduxit eliminate` does it on a proof file, one step at a time:

```console
$ printf 'cut 0 B 0 1\nclose\nproof cut.json\n' | deduxit interact -i "A, A -o B, B -o C |- C" > /dev/null
$ deduxit eliminate cut.json
1. a commutative case on the cut of B
2. the axiom case on the cut of B
cut-free proof of A, A ⊸ B, B ⊸ C ⊢ C (intuitionistic) after 2 steps
           ───── ax   ───── ax
           B ⊢ B      C ⊢ C
───── ax   ──────────────── ⊸L
A ⊢ A        B, B ⊸ C ⊢ C
───────────────────────── ⊸L
   A, A ⊸ B, B ⊸ C ⊢ C
```

The first command writes `cut.json`, a proof that cuts on `B`. The
search never makes a cut, so such a proof comes from the `cut` command
of a session ([Prove a lemma with a cut](interactive.md#prove-a-lemma-with-a-cut))
or from a file written by hand.

`eliminate` reads a proof file as a session's `proof FILE` or
`prove --format json` writes it. It checks the proof in the mode the
file states, or the flags' mode where the file states none. Then each
step reduces the first cut of the proof, one with no cut above it, and
prints its case and the cut formula. Every proof reached is checked.
The last one is drawn as [`check`](check.md) draws it, and `-q` leaves
the drawing out.

## The cases

A step is one of these cases:

- the axiom case, where one premise of the cut is an axiom;
- the key case of a connective against its dual, such as `⊗` against
  `⅋`;
- a commutative case, which moves the cut above a rule that does not
  act on the cut formula;
- for the exponentials: weakening, dereliction, contraction, and a box
  entering a box.

A contraction copies a box, so the proof grows:

```console
$ printf 'cut 0 !a 0\nclose\nproof contraction.json\n' | deduxit interact -i '!a |- a * a' > /dev/null
$ deduxit eliminate -q contraction.json
1. contraction on the cut of !a
2. dereliction on the cut of !a
3. a commutative case on the cut of a
4. the axiom case on the cut of a
5. a commutative case on the cut of a
6. the axiom case on the cut of a
cut-free proof of !a ⊢ a ⊗ a (intuitionistic) after 6 steps
```

## Take a few steps, or bound the size

`--steps N` takes N steps at most (10 000 by default) and prints the
proof reached, cuts and all. `--node-limit N` refuses a step that makes
a proof of more than N nodes (100 000 by default): the exponentials can
make a proof grow beyond any practical size. A bound you set with
`--steps` ends with exit status 0; every other bound ends with 3:

```console
$ printf 'cut 0 !a 0\nclose\nproof contraction.json\n' | deduxit interact -i '!a |- a * a' > /dev/null
$ deduxit eliminate --steps=2 contraction.json
1. contraction on the cut of !a
2. dereliction on the cut of !a
proof of !a ⊢ a ⊗ a (intuitionistic) with 2 cuts left after 2 steps: the elimination took the most steps allowed, 2; --steps N allows more
            ───── ax    ───── ax   ───── ax
            a ⊢ a       a ⊢ a      a ⊢ a
───── ax    ────── !L   ──────────────── ⊗R
a ⊢ a       !a ⊢ a        a, a ⊢ a ⊗ a
────── !L   ────────────────────────── cut
!a ⊢ a            !a, a ⊢ a ⊗ a
─────────────────────────────── cut
        !a, !a ⊢ a ⊗ a
        ────────────── !c
          !a ⊢ a ⊗ a
$ deduxit eliminate -q --node-limit 10 contraction.json; echo "exit $?"
proof of !a ⊢ a ⊗ a (intuitionistic) with 1 cut left after 0 steps: the next step of the elimination makes a proof of more than 10 nodes; raise it with --node-limit N
exit 3
```

## Choose the first cut

`--at NODE` takes the first step on the cut at that node of the proof,
and the other steps as usual. NODE is the node's index in the file's
`nodes`, as the checker's messages and the JSON record number nodes. A
node that is no cut is an error. Here the proof has a cut on `a`
(node 6) below a cut on `b` (node 5):

```console
$ printf 'cut 0 a 0\ncut 2 b 0 2\nclose\nproof two.json\n' | deduxit interact -i "a, a -o b |- b" > /dev/null
$ deduxit eliminate -q two.json
1. the axiom case on the cut of b
2. the axiom case on the cut of a
cut-free proof of a, a ⊸ b ⊢ b (intuitionistic) after 2 steps
$ deduxit eliminate -q --at 6 two.json
1. the axiom case on the cut of a
2. the axiom case on the cut of b
cut-free proof of a, a ⊸ b ⊢ b (intuitionistic) after 2 steps
```

## Eliminate on the proof net

With `--net` the cuts are eliminated on the proof's
[proof net](proof-nets.md) instead. It takes the sequents `--net`
takes: MLL without units, and MELL without units with boxes, classical
with or without `--mix`, or intuitionistic.

- The proof's cuts are the net's cut links, and each step reduces the
  first cut link.
- Nets have no commutative case. The cases are the axiom case, the key
  case of `⊗` against `⅋`, and on boxes weakening (which erases a box),
  dereliction (which opens one), contraction (which copies one) and a
  box entering a box.
- Every net reached passes the correctness criterion.
- The net reached is printed as text, each cut link as a line
  `cut A[x] — A⊥[y]`. `--node-limit` counts the net's vertices.

```console
$ printf 'cut 0 !a 0\nclose\nproof contraction.json\n' | deduxit interact -i '!a |- a * a' > /dev/null
$ deduxit eliminate --net --steps=1 contraction.json
1. contraction on the cut of !a
proof net of !a ⊢ a ⊗ a (intuitionistic) with 2 cut links left after 1 step: the elimination took the most steps allowed, 1; --steps N allows more
!a ⊢ a ⊗ a
!a[0]: a[2] in !a[10], a[3] in !a[6]
!a[8]: a[9]
!a[12]: a[13]
cut !a[6] — !a[8]
cut !a[10] — !a[12]
a[2] — a[11]
a[3] — a[7]
a[4] — a[13]
a[5] — a[9]
proof net (the correctness criterion holds)
$ deduxit eliminate --net --format svg --output net.svg contraction.json
1. contraction on the cut of !a
2. dereliction on the cut of !a
3. the axiom case on the cut of a
4. dereliction on the cut of !a
5. the axiom case on the cut of a
cut-free proof net of !a ⊢ a ⊗ a (intuitionistic) after 5 steps
```

`--format svg`, `png` or `pdf` with `--output FILE` draws every net
into a file of its own:

- `FILE-k` is the net step k was taken on (`net-1.svg` for `net.svg`).
  It shows the step's cut link with a bar under its two formulas, and
  highlights what the step acts on: the links at its ends, its `⊗` and
  `⅋`, the box of its `!`, the box it enters. An SVG file's first line
  is a comment that names its step.
- `FILE` is the net reached.

Each file is written as soon as it is drawn. Each drawing is held to
`--derivation-limit`, and the steps' files together take 256 MiB at
most (`--drawings-limit SIZE`, the settings key `cut.drawn_bytes`).
Past that the steps go on without drawings, a line on standard error
says from which step, and `FILE` still holds the net reached.

## JSON output

With `--format json` the output is the record
`{"version", "proof", "steps", "end"}`:

- `proof` is the proof reached;
- each step has its `cut` (a node of the proof it was taken on), its
  `formula`, its `case`, and `above`, the node where the step acted
  besides the cut, where it has one;
- `end` is `cut_free`, or `refused` with its `reason`.

With `--net` the record is `{"version", "net", "steps", "end"}`, with
the net reached, and each step's `cut` is the two vertices of its cut
link.

## Exit status

- 0: no cut is left, or the steps asked for with `--steps` (or a
  settings file's `cut.steps`) were taken.
- 1: the proof is not valid.
- 2: an error.
- 3: the default bound on steps, another bound or the time limit
  stopped the elimination with cuts left. The proof reached is still
  printed.

## In the library

`linear::proofs::cut::step` reduces one cut, the caller's or the
strategy's, and says which cut and which case.
`linear::proofs::cut::eliminate` runs to the end within `cut::Options`,
its first step on a cut the caller names where one is given.
`linear::nets::cut::step` and `linear::nets::cut::eliminate` do the
same on a `ProofStructure` or a `MellStructure`, and `svg::net_step` and
`svg::mell_net_step` draw a step.
