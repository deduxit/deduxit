# Refute clauses by resolution and DPLL

A clause is a disjunction of atoms and negated atoms, such as `¬a ∨ b`.
A sequent of classical [ordinary logic](ordinary-logic.md) is valid
exactly when the clauses of its negation can never all be true.
`deduxit resolve` turns a sequent's negation into clauses, or reads
clauses from a DIMACS file, and refutes them: it derives the empty clause
`□`, which no assignment makes true. Every refutation is checked step by
step before it is printed.

## Refute a sequent

The resolution rule takes a clause with an atom `p` and a clause with
`¬p`, and derives the rest of both, joined: their resolvent.

```console
$ deduxit resolve 'a -> b, b -> c |- a -> c'
valid in classical logic: the clauses of its negation are unsatisfiable, a checked refutation of 3 resolutions
clauses of its negation by conjunctive normal form, checked:
1. ¬a ∨ b
2. ¬b ∨ c
3. a
4. ¬c
resolution:
5. b  from 3, 1 on a
6. ¬b  from 2, 4 on c
7. □  from 5, 6 on b
```

The formula right of `⊢` moved left negated, and each formula became
clauses by [conjunctive normal form](transformations.md#rewrite-a-sequent-into-a-normal-form),
whose rewriting is checked as `seq normalize` checks it. `--clauses
tseitin` or `--clauses plaisted-greenbaum` take an encoding instead,
whose new atoms are `t1`, `t2`, …. Each resolvent names the two clauses
it comes from and the atom resolved on.

The search resolves the shortest clause not yet used with every clause
used. It leaves out a tautology (a clause with an atom and its negation)
and a clause that contains a clause already kept, which makes it end:
when nothing new appears, the clauses can all be true together. A
sequent that is not valid then has a model of the clauses of its
negation, which is checked against each clause:

```console
$ deduxit resolve --clauses tseitin 'a \/ b |- a'; echo "exit $?"
not valid in classical logic: a checked model of the clauses of its negation, b, t1 true, every other atom false
clauses of its negation by Tseitin's encoding, checked:
1. t1
2. ¬a
3. a ∨ b ∨ ¬t1
4. ¬a ∨ t1
5. ¬b ∨ t1
exit 1
```

## Refute a first-order sequent

A first-order sequent's negation becomes its
[clausal form](transformations.md#at-first-order): each clause a
disjunction of literals whose variables, written `X1`, `X2`, …, stand
for any term. Two clauses resolve on a literal of each when a
substitution, their most general unifier, makes the two atoms equal; the
resolvent is the rest of both under it:

```console
$ deduxit resolve 'forall x. (p(x) -> q(f(x))), p(a) |- exists y. q(y)'
valid in classical logic: the clauses of its negation are unsatisfiable, a checked refutation of 2 inferences, read back as a checked derivation of LK (11 inferences)
clauses of its negation by clausal normal form, each clause's variables X1, X2, …:
1. ¬p(X1) ∨ q(f(X1))
2. p(a)
3. ¬q(X1)
resolution, the first parent's variables X1, …, the second's Y1, …:
4. ¬p(X1)  from 3, 1 with X1 := f(Y1)
5. □  from 4, 2 with X1 := a
```

Each step renames the two clauses' variables apart, the first's `X1`, …
and the second's `Y1`, …, and shows the unifier. Factoring unifies two
literals of one clause, which binary resolution alone needs for `p(x) ∨
p(y)` against `¬p(u) ∨ ¬p(v)`:

```console
$ deduxit resolve 'forall x y. (p(x) \/ p(y)), forall u v. (~p(u) \/ ~p(v)) |-'
valid in classical logic: the clauses of its negation are unsatisfiable, a checked refutation of 3 inferences, read back as a checked derivation of LK (19 inferences)
clauses of its negation by clausal normal form, each clause's variables X1, X2, …:
1. p(X1) ∨ p(X2)
2. ¬p(X1) ∨ ¬p(X2)
resolution, the first parent's variables X1, …, the second's Y1, …:
3. p(X1)  factor of 1 with X1 := X2
4. ¬p(X1)  factor of 2 with X1 := X2
5. □  from 4, 3 with X1 := Y1
```

The search takes the lightest clause not yet used and resolves it with
every clause used; the clauses of the negated goals go first (the set of
support), and the hypotheses' clauses are resolved with each other only
once those run out. deduxit checks the refutation step by step, each
unifier computed again, then reads it back as a derivation of LK of the
sequent as written, the way [Herbrand's theorem](transformations.md#prove-a-first-order-sequent-by-herbrands-theorem)
does, and checks that too. Where the read-back does not reach, the line
says so, and the verdict rests on the refutation and the clausal form's
Skolem symbols.

At first order no model of the clauses is built, so a sequent that is
not valid ends unknown, exit status 3: when nothing new appears, or at
`--clause-limit` and `--step-limit`. `prove --logic classical` refutes
such a sequent where it can. `--method dpll` and `--clauses` other than
`cnf` are propositional and refused at first order.

A sequent with `=` is resolved with **paramodulation** by default: an
equation `s = t` of one clause rewrites a subterm of another clause's
literal that unifies with `s`, putting `t` there, under the unifier.
Reflexivity, `X1 = X1`, joins the clauses; paramodulation never rewrites
a variable:

```console
$ deduxit resolve 'forall x. f(x) = x |- f(f(c)) = c'
valid in classical logic: the clauses of its negation are unsatisfiable, a checked refutation of 2 inferences; no derivation of LK was read back (the proof read back is not a derivation of LK: a refutation by paramodulation is checked step by step; LK has no rule for the step), so the verdict rests on the refutation and the clausal form's Skolem symbols
clauses of its negation by clausal normal form, each clause's variables X1, X2, …:
1. f(X1) = X1
2. X1 = X1
3. ¬f(f(c)) = c
resolution, the first parent's variables X1, …, the second's Y1, …:
4. ¬f(c) = c  3 rewritten by 1 with X1 := f(c)
5. □  from 4, 1 with Y1 := c
```

The checker computes each rewrite again. LK has no rule for it, so such a
refutation is not read back as a derivation. With few clauses the search
also ends where nothing new appears: `a = b |- a = c` saturates.
`--equality axioms` takes the equality axioms as clauses instead
(reflexivity, symmetry, transitivity, and that equal arguments give equal
values or the same truth) and resolves alone; its refutations read back
as derivations of LK, and its saturation rarely ends.

## Read clauses from a DIMACS file

`--dimacs FILE` reads clauses in the DIMACS CNF format that SAT solvers
read: comment lines starting with `c`, the header `p cnf VARIABLES
CLAUSES`, then each clause as numbers ended by `0`, a number k for the
variable k and its negative for the variable's negation. The variable k
is the atom `xk`.

```console
$ cat four.cnf
c four clauses over two atoms that no assignment satisfies
p cnf 2 4
1 2 0
-1 2 0
1 -2 0
-1 -2 0
$ deduxit resolve --dimacs four.cnf
unsatisfiable: a checked refutation of 3 resolutions
clauses:
1. x1 ∨ x2
2. ¬x1 ∨ x2
3. x1 ∨ ¬x2
4. ¬x1 ∨ ¬x2
resolution:
5. x1  from 1, 3 on x2
6. ¬x1  from 2, 4 on x2
7. □  from 5, 6 on x1
```

## Trace DPLL

`--method dpll` searches by the procedure of Davis, Putnam, Logemann
and Loveland and prints every step. A clause with one literal left makes
it true (unit), a literal whose negation is in no clause still open is
made true (pure), and otherwise an atom is decided true. At a conflict,
a clause all false, the last decision is flipped (backtrack). The
refutation is read off the steps and checked:

```console
$ deduxit resolve --method dpll --dimacs four.cnf
unsatisfiable: a checked refutation of 3 resolutions
clauses:
1. x1 ∨ x2
2. ¬x1 ∨ x2
3. x1 ∨ ¬x2
4. ¬x1 ∨ ¬x2
DPLL:
decide x1 (level 1)
  unit x2 from clause 2
  conflict at clause 4
backtrack: ¬x1 (level 1)
  unit x2 from clause 1
  conflict at clause 3
resolution read off the steps:
5. ¬x1  from 2, 4 on x2
6. x1  from 1, 3 on x2
7. □  from 6, 5 on x1
```

## Bounds

Resolution can need exponentially many clauses. `--clause-limit N` (2000
by default) bounds the clauses the search keeps and `--step-limit N`
(100000 by default) the inferences it makes or the steps of DPLL; `none`
lifts either. Past a bound the answer is unknown:

```console
$ deduxit resolve --step-limit 1 'a -> b, b -> c |- a -> c'; echo "exit $?"
unknown: the bound of 1 step was reached; raise it with --step-limit
clauses of its negation by conjunctive normal form, checked:
1. ¬a ∨ b
2. ¬b ∨ c
3. a
4. ¬c
exit 3
```

The exit status is 0 for unsatisfiable clauses (a valid sequent), 1 for
satisfiable ones, 3 for unknown and 2 for an error.

## Check a refutation

A refutation written by hand, or by a program, is JSON at wire level 22:
the atoms, the clauses as DIMACS writes them, and the resolvents, each
with the numbers of its two clauses (counted from 0, the given clauses
first), the atom counted from 1, and the clause it derives. The first
clause holds the atom, the second its negation:

```json
{"version":22,"atoms":["x1","x2"],"clauses":[[1,2],[-1,2],[1,-2],[-1,-2]],
 "resolvents":[{"first":0,"second":1,"atom":1,"clause":[2]},
               {"first":2,"second":3,"atom":1,"clause":[-2]},
               {"first":4,"second":5,"atom":2,"clause":[]}]}
```

`deduxit check` checks every resolvent and that one is empty:

```console
$ deduxit check refutation.json
valid: a refutation of 4 clauses by 3 resolutions
```

## Check an outside solver's proof

A SAT solver such as CaDiCaL or Kissat writes its proof that clauses are
unsatisfiable in the DRAT format (clauses added, and deleted after `d`)
or the LRAT format (each clause numbered, with the clauses it uses).
`deduxit check --dimacs FILE PROOF` checks such a proof against the
clauses: every clause it adds must follow by unit propagation from the
clauses kept, and it must add the empty clause.

```console
$ cat four.drat
2 0
0
$ deduxit check --dimacs four.cnf four.drat
valid: the proof refutes the 4 clauses, 2 steps checked
$ deduxit check --dimacs four.cnf four.lrat
valid: the proof refutes the 4 clauses, 2 steps checked
```

DRAT is read as text or in its binary form; a file ending in `.lrat` is
LRAT, and `--proof-format drat|lrat` names the format otherwise. A step
that is only a RAT step (resolution asymmetric tautology, which some
solvers' extended techniques write) is not accepted. The exit status is
0 for a proof that holds and 1 for one that does not.

## JSON output and the library

`resolve --format json` writes one document at wire level 22:
`{"version", "deduxit", "method", "verdict", "atoms", "clauses",
"resolvents", "true_atoms", "bound", "trace", "statistics"}`, `verdict`
being `unsatisfiable` with the refutation's `resolvents`, `satisfiable`
with the `true_atoms` of a model, or `unknown` with the `bound`; `trace`
holds DPLL's steps, each tagged `event`. A first-order sequent's
document is at level 29: `{"version", "deduxit", "method", "verdict",
"bound", "sequent", "clausal", "clauses", "inferences", "derivation",
"not_read_back", "statistics"}`, its clauses as text, each inference's
`rule` (`resolution` with `first`, `second`, `first_literal` and
`second_literal`, `factor` with `parent`, or `paramodulation` with
`from`, `from_literal`, `into`, `into_literal`, the subterm's `position`
counted from 0 as the literal is written, each term before its
arguments, and `reverse` where the equation is read right to left), its
`unifier` and the
`clause` it made, numbered on after the given clauses; `verdict` is
`unsatisfiable`, `saturated` or `unknown`.

In a program, `deduxit::ordinary::resolution::decide` decides a
`Clauses` set, `Clauses::from_dimacs` and `resolution::clauses_of` make
one, `Refutation::resolve` takes a step by hand and `Refutation::check`
checks a refutation; `resolution::check_proof` checks an outside proof.
At first order `resolution::first_order::resolve` decides a sequent,
`first_order::clauses_of` gives its clauses, `Clauses::infer` takes a
step by hand and `first_order::Refutation::check` checks a refutation.
The protocol's operation `ordinary.resolve` decides a sequent's clauses
in a [workspace](workspace.md).
