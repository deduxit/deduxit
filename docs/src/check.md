# Check a proof or a refutation

`deduxit check` verifies a proof file with the proof checker alone,
without searching. The proof file is what `prove --format json` writes.
`check` reads it from its argument, from `--file`, or from standard
input:

```console
$ deduxit prove --format json "A |- A" > identity.json
$ deduxit check -q --file identity.json
valid proof of ⊢ ~A, A (classical)
$ deduxit prove --format json "A |- A" | deduxit check --format json
{"version":1,"verdict":"valid","mode":"classical"}
```

The file states its mode, and `check` checks in that mode. A mode flag
that names another mode is an error.

## Exit status and limits

`check` exits with 0 for a valid proof, 1 for an invalid one, and 3 when
a limit stopped the check before a verdict. A crafted proof file can
make the check slow, so it has the time limit of `prove`, two seconds by
default (`--timeout none` lifts it), counted from the start of the
command, and `--work-limit N` bounds its work. In JSON, an invalid or
unknown verdict carries the library's error object.

## Check a refutation

The outcome of an unprovable sequent holds its refutation, the reason
the verdict line gives. `check` verifies it without the search, in the
mode the file states, since a refutation is a claim about that mode
alone. Each kind of refutation has a check: an unbalanced atom, a count
with copies of one sign, the count equation, a Horn program's state
equation, a classical assignment and a saturated set. The exit status is
0 for a refutation that holds and 1 for one that does not. "The search
was exhaustive" rests on the search alone and has no check, so it exits
with 3, as does a check stopped by the time limit:

```console
$ deduxit prove -i --format json "a, a -o b, !(b -o b * c), !(b -o d) |- b * d" | deduxit check
valid refutation of a, a ⊸ b, !(b ⊸ (b ⊗ c)), !(b ⊸ d) ⊢ b ⊗ d (intuitionistic): the state equation of the Petri net has no solution, so no firing of its clauses yields the goal's atoms: weighting each a by 2, b by 1, d by 1 and the clauses used once by weights of their own, no clause that can fire raises the weighted count of the atoms, and the goal asks it raised
$ deduxit prove --format json "A & B |- A * B" | deduxit check; echo "exit $?"
unknown: the refutation of A & B ⊢ A ⊗ B (classical) is not checked: a refutation of kind `exhausted` rests on the search and has no check
exit 3
```

With `--format rocq`, a refutation that holds is written as its Rocq
certificate, as `prove` writes it ([Check a certificate in
Rocq](rocq.md)).

## Draw the checked proof

`check` draws a valid proof in any format `prove` writes, and `--net`
draws its proof net ([Draw a proof net](proof-nets.md)). `--no-verdict`
leaves the verdict line out, and `--abbreviate N` shortens long
sequents in the verdict line and the error report to N characters
(`none` for the whole text).
