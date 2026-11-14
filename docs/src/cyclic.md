# Prove in cyclic linear logic

Cyclic linear logic is linear logic in which the order of the formulas
counts. A sequent is a ring of formulas: it may be rotated, but no two
formulas may change places otherwise. `--cyclic` decides it, for the
multiplicatives `⊗` and `⅋` without units:

```console
$ deduxit prove --cyclic "a * b |- a * b"
provable (MLL, cyclic, net engine)
─────── ax   ─────── ax
⊢ ~a, a      ⊢ ~b, b
──────────────────── ⊗
  ⊢ ~b, ~a, a ⊗ b
  ──────────────── ⅋
  ⊢ ~b ⅋ ~a, a ⊗ b
$ deduxit prove --cyclic "a * b |- b * a"
unprovable (MLL, cyclic, net engine): the search was exhaustive
```

The second sequent holds in ordinary linear logic, where `a ⊗ b` and
`b ⊗ a` say the same. In the cyclic calculus they do not.

## How the order is read

- A sequent is one-sided, read up to rotation.
- `⊗` joins the formulas on either side of it, and `⅋` two neighbours.
- The dual of a product reverses it: `~(A * B)` is `~B par ~A`.
- The formulas left of `|-` come in the reverse order, each negated.
- `A -o B` is `~A par B`, so `A, A -o B |- B` holds and `A -o B, A |- B`
  does not.

`seq print --cyclic` and `seq json --cyclic` show the one-sided sequent
deduxit reads:

```console
$ deduxit seq print --cyclic "a * b |- a * b"
⊢ ~b ⅋ ~a, a ⊗ b
```

Every sequent of a derivation is written in its cyclic order, read from
its first formula. In a batch ([Decide many sequents at
once](batch.md)), a JSON Lines record names the mode `cyclic`.

## Draw the proof net

The net engine decides the mode. It searches for an axiom linking that
is a proof net and planar: no two links cross when the literals stand in
their order around a circle. `--net` draws the net so ([Draw a proof
net](proof-nets.md)):

```console
$ deduxit prove --cyclic --net "a, a -o b |- b"
provable (MLL, cyclic, net engine)
⊢ ~b ⊗ a, ~a, b
~b[1] — b[4]
a[2] — ~a[3]
planar proof net (the correctness criterion holds, and no two links cross)
```

## What the mode refuses

The mode refuses units, additives, exponentials, Mix, weakening and
`--intuitionistic`, and every engine but the net engine. It has no Rocq
certificates, since the Rocq calculi are commutative: the verdict
stands, and a line on standard error says why there is no certificate.

```console
$ deduxit prove --cyclic "|- a * b, 1"
error: cyclic mode decides unit-free MLL, and the sequent lies in MLL with units
$ deduxit prove --cyclic --format rocq "a |- a"
no certificate: the Rocq calculi are commutative, so a derivation or a disproof of the cyclic mode has no certificate
(* provable (MLL, cyclic, net engine) *)
```
