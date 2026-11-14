# Draw a proof net

A proof net shows a proof as a graph: the trees of the sequent's
formulas, and axiom links that pair each literal with its dual. Proofs
that differ only in the order of their rules have the same net. Add
`--net` to `prove` to get the net in place of the derivation:

```console
$ deduxit prove --net "|- A * B, C * (~A par ~B), ~C"
provable (MLL, classical, focus engine)
⊢ A ⊗ B, C ⊗ (~A ⅋ ~B), ~C
A[1] — ~A[6]
B[2] — ~B[7]
C[4] — ~C[8]
proof net (the correctness criterion holds)
```

After the verdict come the sequent and the axiom links. Each literal
carries its vertex number, its place when the sequent's subformulas
are numbered in the order they are written. The last line is the
verdict of the correctness criterion. With the net engine
(`--engine net`) the net is the one the search found; with the other
engines it is read off the proof.

## What the criterion checks

A switching keeps one premise edge of every `⅋` and drops the other.
The structure is a proof net, the net of some proof, when every
switching leaves a graph that is connected and has no cycle (Danos and
Regnier's criterion); with Mix, having no cycle is enough. The last
line says which criterion held:

```console
$ deduxit prove --net --mix "|- A, ~A, B, ~B"
provable (MLL, classical with Mix, focus engine)
⊢ A, ~A, B, ~B
A[0] — ~A[1]
B[2] — ~B[3]
proof net with Mix (the correctness criterion with Mix holds)
$ deduxit prove --net --cyclic "A, A -o B |- B"
provable (MLL, cyclic, net engine)
⊢ ~B ⊗ A, ~A, B
~B[1] — B[4]
A[2] — ~A[3]
planar proof net (the correctness criterion holds, and no two links cross)
```

In [cyclic linear logic](cyclic.md) a net must also be planar: no two
links cross.

## The net of a proof file

`check --net` checks a proof file and then shows its net:

```console
$ deduxit prove --output proof.json "A, A -o B |- B"
$ deduxit check --net proof.json
valid proof of ⊢ ~A, A ⊗ ~B, B (classical)
⊢ ~A, A ⊗ ~B, B
~A[0] — A[2]
~B[3] — B[4]
proof net (the correctness criterion holds)
```

## Draw it, or write it as JSON

`--format svg`, `png` or `pdf`, or an `--output` file with one of those
extensions, draws the net: the formula trees, the axiom links as arcs
over the literals, and every `!`-box as a rectangle around its
contents:

```console
$ deduxit prove --net --output net.svg "A, A -o B |- B"
```

[Export a proof](output-formats.md) shows the drawings and their
styles. A drawing estimated above `--derivation-limit` is left out.

`--format json` writes the net's document in place of the outcome. The
library reads it back as a `ProofStructure`, or as a `MellStructure`
for a net with boxes.

## Nets with boxes

A proof with exponentials, in MELL without units, gets a net with
`!`-boxes, in classical mode with or without Mix and in intuitionistic
mode. A vertex is one instance of a subformula, since a `?` formula may
be used several times. Each `?` vertex is listed with its copies, each
with the `!`-box it lies in where that is another box than the `?`'s
own. A `?` formula that is never used is listed with `no copy` and the
literal of the axiom link its weakening hangs on:

```console
$ deduxit prove --net "!a, !b |- !a"
provable (MELL, classical, focus engine)
⊢ ?~a, ?~b, !a
?~a[0]: ~a[3] in !a[2]
?~b[1]: no copy, hangs on a[4]
~a[3] — a[4]
proof net (the correctness criterion holds)
$ deduxit prove --net --format json "!a, !b |- !a"
{"version":4,"sequent":{"terms":[{"D":0},{"?":0},{"D":1},{"?":2},{"V":0},{"!":4}],"roots":[1,3,5],"atoms":["a","b"],"antecedents":2},"mix":false,"vertices":[[1,0,2],[5,2,2]],"links":[[4,3]],"weakenings":[[1,4]]}
```

## Intuitionistic nets

An intuitionistic proof's net is the net of its one-sided sequent,
written as the sequent was. A hypothesis `!A`, whose copies the boxes
use, stands for the conclusion `?~A`, and an axiom links an atom left
of `⊢` to the same atom right of it:

```console
$ deduxit prove --net -i "!a, !(a -o b) |- !b"
provable (IMELL, intuitionistic, two-sided engine)
!a, !(a ⊸ b) ⊢ !b
!a[0]: a[3] in !b[2]
!(a ⊸ b)[1]: a ⊸ b[4] in !b[2]
a[3] — a[5]
b[6] — b[7]
proof net (the correctness criterion holds)
```

## Where nets do not exist

Proof nets exist for MLL and MELL without units. A sequent with units
or additives, and affine mode, have none, and `--net` says so before
the search, with exit status 2:

```console
$ deduxit prove --engine net "A & B |- A"
error: proof nets exist for MLL without units only, not for ALL
$ deduxit prove --net -i --affine "!a |- !a"
error: proof nets exist in linear mode only, with or without --mix, not in intuitionistic affine mode
```
