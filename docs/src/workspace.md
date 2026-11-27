# Keep a sequent and what is made from it

A workspace file keeps a sequent beside what deduxit made from it: its
translation, the translation's decision, a proof started, each with how
it was made. Change the sequent, and what was made from it shows as out
of date until you make it again; undo takes a whole change back.

## Make a workspace and add to it

`workspace new` makes the file, here with a classical sequent. Each node
has a number, its kind and its value in a line:

```console
$ deduxit workspace new lem.json '|- a \/ ~a' --logic classical
0  ordinary.sequent  ⊢ a ∨ ¬a (classical)
```

`workspace apply` makes a node from other nodes by an operation and runs
it. The nodes come first, then the operation; `--set` gives the
operation's parameters. The line under the node says how it was made and
whether the check of what the operation promises held: a negative
translation promises that both sequents are valid together or not, and
deduxit decides both to check it.

```console
$ deduxit workspace new lem.json '|- a \/ ~a' --logic classical
0  ordinary.sequent  ⊢ a ∨ ¬a (classical)
$ deduxit workspace apply lem.json 0 ordinary.translate --set translation=glivenko
1  ordinary.sequent  ⊢ ¬¬(a ∨ ¬a) (intuitionistic)
   from 0 by ordinary.translate translation=glivenko; check: held
$ deduxit workspace apply lem.json 1 ordinary.decide
2  ordinary.outcome  Valid
   from 1 by ordinary.decide; check: held
```

`workspace operations` lists what each kind of value takes:

```console
$ deduxit workspace operations ordinary.outcome
ordinary.derivation  ordinary.session or ordinary.outcome -> ordinary.derivation: The derivation
ordinary.draw  ordinary.session or ordinary.derivation or ordinary.outcome -> drawing: A drawing
ordinary.export  ordinary.session or ordinary.derivation or ordinary.outcome -> text: An export
ordinary.certificate  ordinary.derivation or ordinary.outcome -> certificate: A Rocq certificate
```

[What each suite offers](protocol.md) lists every kind and operation,
with the parameters each takes.

## Change a sequent and make again what follows

`workspace set` gives a node a new sequent. What was made from it is out
of date, and keeps its last value until `workspace run` makes it again.
`--checks` makes each node's check too. The exit status is the worst
verdict run: 0 valid, 1 not valid, 3 not decided, 2 an error or a check
that failed. Each node's verdict is the one its command gives: a
resolution that finds the clauses satisfiable is 1, as `resolve` is,
and so is an interpolation of a sequent that is not valid.

```console
$ deduxit workspace new lem.json '|- a \/ ~a' --logic classical
0  ordinary.sequent  ⊢ a ∨ ¬a (classical)
$ deduxit workspace apply lem.json 0 ordinary.translate --set translation=glivenko
1  ordinary.sequent  ⊢ ¬¬(a ∨ ¬a) (intuitionistic)
   from 0 by ordinary.translate translation=glivenko; check: held
$ deduxit workspace apply lem.json 1 ordinary.decide
2  ordinary.outcome  Valid
   from 1 by ordinary.decide; check: held
$ deduxit workspace set lem.json 0 '|- a -> b \/ ~b'
0  ordinary.sequent  ⊢ a → (b ∨ ¬b) (classical)
1  ordinary.sequent  ⊢ ¬¬(a ∨ ¬a) (intuitionistic)
   from 0 by ordinary.translate translation=glivenko; check: recorded
   out of date: node 0 changed
2  ordinary.outcome  Valid
   from 1 by ordinary.decide; check: held
   out of date: it waits for node 1
$ deduxit workspace run lem.json --checks; echo "exit $?"
0  ordinary.sequent  ⊢ a → (b ∨ ¬b) (classical)
1  ordinary.sequent  ⊢ ¬¬(a → (b ∨ ¬b)) (intuitionistic)
   from 0 by ordinary.translate translation=glivenko; check: held
2  ordinary.outcome  Valid
   from 1 by ordinary.decide; check: held
exit 0
$ deduxit workspace undo lem.json
undone: sequent change. Nothing is out of date.
$ deduxit workspace log lem.json
undo takes back: ordinary.decide
redo brings back: sequent change
```

A check that needs a search reads back from the file as `recorded`: it
held when it was made, and `workspace check` makes every check again.
Its exit status is the checks': 0 where all hold, 1 where one fails, 3
where one could not be made within its node's limits.
Undo shows again what was made before the change without running it:
the file keeps each run, keyed by its operation, its parameters, the
limits it ran under and its inputs. `workspace remove` removes a node
and every node made from it.

## Limits a node keeps

A node runs under the limits given when it was made (`--work-limit`,
`--memory-limit`, `--settings`), and keeps them: a later run of
`workspace run` with other flags changes no node. `workspace set NODE
--work-limit N` gives one node other limits, and a node a bound stopped
says which:

```console
$ deduxit workspace new peirce.json '|- ((a -> b) -> a) -> a' --logic classical
0  ordinary.sequent  ⊢ ((a → b) → a) → a (classical)
$ deduxit workspace apply peirce.json 0 ordinary.decide --set method=embedding --work-limit 1; echo "exit $?"
1  ordinary.outcome  Not decided
   from 0 by ordinary.decide method=embedding
   not decided: the run stopped at its bound limits.work
exit 3
$ deduxit workspace set peirce.json 1 --work-limit none
1  ordinary.outcome  Not decided
   from 0 by ordinary.decide method=embedding
   not decided: the run stopped at its bound limits.work
   out of date: its parameters or limits changed
$ deduxit workspace run peirce.json; echo "exit $?"
0  ordinary.sequent  ⊢ ((a → b) → a) → a (classical)
1  ordinary.outcome  Valid
   from 0 by ordinary.decide method=embedding
exit 0
```

`--timeout` stops a run without changing the node, which then stays as
it was.

## The file

The file is JSON at wire level 19: each value as its kind's document,
the nodes, the runs kept, and the changes undo takes back. `workspace
show FILE --format json` writes it, and `workspace show FILE NODE
--format json` writes one node's value, which the other commands read:
`check` reads an outcome or a derivation, `interact --state` a session.
Reading a file checks every value, runs again each run that needs no
search and drops one whose output differs, and makes again each check
that needs no search. The bounds of a workspace are `limits.workspace`
in [the settings file](settings.md): its nodes, its runs kept, the
changes undo keeps and the bytes of its values. Past the number of
changes kept, the oldest is forgotten; past any other bound, the change
or the file is refused.
