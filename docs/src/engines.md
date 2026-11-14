# How deduxit searches

This page explains how `deduxit prove` finds a proof or shows there is
none: which engine searches which sequent, why the search with
exponentials deepens a bound, how a sequent is refuted, and how threads
share the work. You need none of it to use the command; it helps to
read `--stats`, to choose `--engine`, and to understand an "unknown".

## Which engine searches

deduxit looks at the sequent's connectives (its *fragment*) and picks
an engine. The verdict line names it, and `--engine` forces another:

- The **additive engine** decides a sequent of two formulas built from
  atoms with `&`, `⊕`, `⊤` and `0` alone. It recurses on pairs of subformulas, so it
  runs in time linear in the product of their sizes.
- The **Horn engine** decides a *Horn program*: atoms, implications
  between tensors of atoms, such implications under `!`, and a goal that
  is a tensor of atoms (below).
- The **focus engine** searches everything else in classical linear
  logic: a focused sequent search over bitsets, which applies the rules
  that need no choice at once and makes choices only at *stable*
  sequents.
- The **two-sided engine** is the focus engine's counterpart in
  intuitionistic linear logic. It keeps one goal on every branch.
- The **net engine** searches MLL without units for an axiom linking
  that makes the sequent's formula trees a proof net. It decides cyclic
  linear logic, and in classical linear logic it takes over where
  the focus engine stops at its recursion limit on a sequent whose atoms
  occur once with each sign (a chain of thousands of links), or at its
  memory bound under Mix.
- The **inverse method** (`--engine inverse`) searches forward, from
  the axioms up. Without Mix, it takes over where the focus engine meets
  its memory bound (some sequents of thousands of distinct atoms), three
  to four times as fast there, and the net engine takes over after it
  where it meets a bound of its own. It also takes short turns beside
  the engine on some goals (below).
- The **essential engine** (`--engine essential`) searches IMLL without
  units for an essential net. It decides the Lambek calculus, and in
  intuitionistic linear logic it runs only when asked.

`--fragment` asserts the fragment, and a sequent outside it is an error.
`--stats` prints the counters of the engine that ran. The focus engine
counts the stable sequents it visited, those it found in its memo (the
sequents already decided), and the splits it examined (the ways to
divide a context between the two premises of a `⊗`). The net engine
counts the literals it chose to link, the links it tried and the
correctness tests it ran. Units of work are each engine's steps; on one
thread they are the same on every machine.

```console
$ deduxit prove --mix --stats --deterministic --engine net "|- A par B, ~A, ~B"
provable (MLL, classical with Mix, net engine)
─────── ax   ─────── ax
⊢ A, ~A      ⊢ B, ~B
──────────────────── mix
   ⊢ A, B, ~A, ~B
   ─────────────── ⅋
   ⊢ A ⅋ B, ~A, ~B
literals chosen: 2
links tried: 2
exact tests run: 2
units of work: 2
time: 63.45µs
$ deduxit prove --engine focus --stats --quiet --deterministic "|- A * B, C * (~A par ~B), ~C"
provable (MLL, classical, focus engine)
stable sequents visited: 2
from the memo: 0
memo entries at most: 2
splits examined: 3
units of work: 35
time: 56.66µs
```

## Exponentials and the copy bound

With `!` and `?` (MELL and full linear logic) a proof may copy a `?`
formula any number of times. The focus engine searches under a bound on
how often a `?` formula is copied on one branch, and deepens the bound
from zero: it goes on to the next bound until it decides or the time
limit passes. `--copies N` caps the bound at N. The derivation shows the
standard rules: dereliction (`?d`), contraction (`?c`), weakening (`?w`)
and promotion (`!`).

```console
$ deduxit prove "!A |- !(A * A)"
provable (MELL, classical, focus engine)
─────── ax    ─────── ax
⊢ ~A, A       ⊢ ~A, A
──────── ?d   ──────── ?d
⊢ ?~A, A      ⊢ ?~A, A
────────────────────── ⊗
  ⊢ ?~A, ?~A, A ⊗ A
  ───────────────── ?c
    ⊢ ?~A, A ⊗ A
   ─────────────── !
   ⊢ ?~A, !(A ⊗ A)
```

Deepening is why a call without flags answers within its time limit
whatever it is given, and why an answer near the limit depends on the
machine.

### The bias

The focused engines treat one literal of every atom as positive. This
decides where a proof keeps its focus, never what is provable. `--bias`
names the rule: `factors` makes positive the literal that is more often
a direct factor of a `⊗` (such a `⊗` needs no search for its split),
`rarer` the one with fewer occurrences. Without exponentials the
default, `auto`, is `factors`, and under `--affine` it is `rarer`.

With exponentials neither always wins. On Horn clauses under `!`,
`factors` chains forward from the facts and is often faster by orders
of magnitude, but a forward chain takes one copy per step on a single
branch, where `rarer` chains backward from the goal within a few copies.
So on a sequent with exponentials `auto` runs both searches and answers
with the first that decides: in turns, in slices of work, on one core,
so that the run stays a function of the input, and side by side on
several. Under `--copies N` the backward search keeps to the bound, and
so does the forward one, except on a Horn program, where it runs within
`--forward-copies` (30 by default), a bound in steps of the chain:

```console
$ deduxit prove -q --deterministic --stats --engine focus "!(a * a -o b), !(b * b -o c), !(c * c -o d), a, a, a, a, a, a, a, a |- d"
provable (MELL, classical, focus engine)
stable sequents visited: 47
from the memo: 5
memo entries at most: 9
splits examined: 151
copy bound reached: 7
forward search's copy bound reached: 7
units of work: 1297
time: 136.45µs
$ deduxit prove -q --deterministic --stats --engine focus --bias rarer "!(a * a -o b), !(b * b -o c), !(c * c -o d), a, a, a, a, a, a, a, a |- d"
provable (MELL, classical, focus engine)
stable sequents visited: 230
from the memo: 178
memo entries at most: 34
splits examined: 1036
copy bound reached: 3
units of work: 4256
time: 181.10µs
$ deduxit prove -q --deterministic --stats --engine focus --bias factors --copies 3 "!(a * a -o b), !(b * b -o c), !(c * c -o d), a, a, a, a, a, a, a, a |- d"
unknown (MELL, classical, focus engine): the copy bound of 3 was reached after 55.51µs; raise it with --copies N, or lift it with --copies none to deepen it while the time limit lasts
stable sequents visited: 11
from the memo: 0
memo entries at most: 5
splits examined: 31
copy bound reached: 3
units of work: 322
time: 55.51µs
$ deduxit prove -q --engine focus --copies 1 "!A, !(A -o B), !(B -o C) |- C"
provable (MELL, classical, focus engine)
$ deduxit prove -q --engine focus --copies 1 --forward-copies 1 "!A, !(A -o B), !(B -o C) |- C"
unknown (MELL, classical, focus engine): the copy bound of 1 was reached after 149.56µs; raise it with --copies N, or lift it with --copies none to deepen it while the time limit lasts
```

## The Horn engine

A Horn program is a Petri net. `!(a * b -o c * d)` is a transition that
takes the tokens `a` and `b` and gives `c` and `d`; the atoms left of
`⊢` are the starting marking, and the goal is the marking to reach. With
a clause under `!`, the sequent goes to the Horn engine. It searches the
markings in place of sequents, each reached once and the one nearest the
goal first. It reads the proof off the firing sequence it finds: each
firing is a copy of its clause, whose body takes its atoms by axioms.

It needs no copy bound. It answers "unprovable" when it has seen every
reachable marking without the goal, or when the counts of an atom rule
the goal out. It also answers "unprovable" when the net's *state
equation* has no solution, which a simplex beside the search finds out.
Then weights for the atoms, checked exactly, show that no firing raises
the weighted count of the tokens while the goal needs it raised, however
many markings the net has. A net whose markings grow without end and
whose equation has a solution is searched until the time or the memory
limit.

Two more refutations reach nets whose markings grow without end. A
clause with an atom no reachable marking holds can never fire, and is
left out. And once the search has run a while, the same search runs
backward from the goal beside it, where such a net often has few
markings.

In affine mode, the tokens and clauses left over may be weakened, so
the question is whether some reachable marking covers the goal. The
engine decides it backward from the goal: it keeps the least markings
from which a firing covers the goal, until one lies below the start or
none is new, which always comes. `--stats` counts the markings:

```console
$ deduxit prove "!A, !(A -o B), !(B -o C) |- C"
provable (MELL, classical, horn engine)
             ─────── ax   ─────── ax
             ⊢ ~B, B      ⊢ ~C, C
             ──────────────────── ⊗
               ⊢ ~B, B ⊗ ~C, C
─────── ax    ────────────────── ?d
⊢ ~A, A       ⊢ ~B, ?(B ⊗ ~C), C
──────────────────────────────── ⊗
   ⊢ ~A, A ⊗ ~B, ?(B ⊗ ~C), C
  ───────────────────────────── ?d
  ⊢ ~A, ?(A ⊗ ~B), ?(B ⊗ ~C), C
  ────────────────────────────── ?d
  ⊢ ?~A, ?(A ⊗ ~B), ?(B ⊗ ~C), C
$ deduxit prove -q --deterministic --stats "!(a * a -o b), !(b * b -o c), !(c * c -o d), a, a, a, a, a, a, a, a |- d"
provable (MELL, classical, horn engine)
markings reached: 7
markings met again: 0
markings kept: 7
units of work: 6
time: 45.06µs
$ deduxit prove -q --deterministic --stats "!(a * a -o b), !(b * b -o c), !(c * c -o d), a, a, a, a, a, a, a, a |- d * a"
unprovable (MELL, classical, horn engine): the search was exhaustive
markings reached: 12
markings met again: 2
markings kept: 10
units of work: 13
time: 31.81µs
$ deduxit prove -q "!(a -o a * a), !(b * b -o c), a, b |- c"
unprovable (MELL, classical, horn engine): the state equation of the Petri net has no solution, so no firing of its clauses yields the goal's atoms: weighting each a by −2, b by 1 and c by 2, no clause that can fire raises the weighted count of the atoms, and the goal asks it raised
$ deduxit prove -q --affine --stats "!(a * a -o b * b * b), a, a, a |- b * b * b"
provable (MELL, classical affine, horn engine)
markings computed backward: 1
markings covered already: 0
markings kept: 1
units of work: 2
time: 268.07µs
$ deduxit prove -q --affine "!(a * a -o b * b * b), a, a, a |- b * b * b * b"
unprovable (MELL, classical affine, horn engine): the search was exhaustive
```

The last goal's state equation has a solution, one and a third firings
of the clause, so only the search refutes it. The `a` left over in the
proof before it is weakened.

## The inverse method

`--engine inverse` searches forward by the *focused inverse method*. It
derives stable sequents from the axioms upward, each focusing phase of
the sequent's formulas one rule. It keeps a derived sequent unless one
already kept stands for every sequent it stands for, and proves the goal
once every stable sequent the goal decomposes into is derived. Without
exponentials it can derive finitely many sequents, so it decides. With
them it needs no copy bound: it proves what is provable given time, and
refutes where nothing new comes. This suits a small goal under many
hypotheses that `!` makes reusable.

Where it refutes, the refutation is the set of sequents it saturated
with. A check of its own verifies it before the verdict: it reads the
rules off the sequent again and applies them to every choice of the
set's sequents. Its counters are the sequents taken up, subsumed and
kept, and it runs in every commutative mode.

Without `--engine`, on an intuitionistic goal with an exponential and on
a goal under Mix (not affine), the inverse method takes turns beside the
engine detection picks. That engine searches alone for some hundredths
of a second. Then, each time its work has doubled, the inverse method
takes a turn on the same thread, starting afresh, of at most a
thirty-second of that work, and the engine goes on where it stopped. The
turns cost at most a sixteenth of the engine's work, and a goal decided
within the head start sees none. A goal the inverse method decides in a
turn is answered by it, and the verdict line says `inverse engine`. On
the intuitionistic problems of the LLTP library, this decides in
milliseconds what the backward search leaves undecided after a minute.

```console
$ deduxit prove -i --engine inverse --stats "!(a -o b), !(b -o c), a |- c"
provable (IMELL, intuitionistic, inverse engine)
           ───── ax   ───── ax
           b ⊢ b      c ⊢ c
           ──────────────── ⊸L
             b, b ⊸ c ⊢ c
───── ax    ─────────────── !L
a ⊢ a       b, !(b ⊸ c) ⊢ c
─────────────────────────── ⊸L
  a ⊸ b, !(b ⊸ c), a ⊢ c
 ───────────────────────── !L
 !(a ⊸ b), !(b ⊸ c), a ⊢ c
sequents taken up: 2
sequents subsumed: 0
sequents kept: 3
units of work: 50
time: 102.44µs
$ deduxit prove -i --engine inverse -q "!(a -o b), a |- b * b"
unprovable (IMELL, intuitionistic, inverse engine): the search saturated: its 2 sequents are closed under the rules, and none of them yields the goal
```

## Intuitionistic linear logic

Under `-i` a sequent of two additive-only formulas goes to the additive
engine, in every mode, and a Horn program with a clause under `!` to
the Horn engine, as in classical logic. Everything else goes to the
two-sided engine, with the same copy bound and affine mode as the focus
engine. IMLL without units, each atom once with each sign, goes to the
net engine where the two-sided engine stops at its recursion limit or
memory bound: the classical net of the one-sided sequent is always an
intuitionistic proof.

```console
$ deduxit prove -i -q --stats "(A & B) + (A & C) |- A & (B + C)"
provable (IALL, intuitionistic, additive engine)
pairs of subformulas visited: 20
from the memo: 0
memo entries: 20
units of work: 20
time: 89.90µs
```

`--engine essential` searches IMLL without units for an *essential net*.
The intuitionistic reading directs every edge from the hypotheses toward
the goal. A linking is then a proof net when the directed graph has no
cycle and every implication `A ⊸ B` on the goal's side dominates its
antecedent: every path from `A` to the goal passes through it. The
engine keeps which occurrence reaches which, a matrix of bits that
`--memory-limit` bounds. It was slower than the two-sided engine on all
but the smallest sequents it was measured on, so under `-i` it runs only when
asked:

```console
$ deduxit prove -i --engine essential --net "A -o B, B -o C |- A -o C"
provable (IMLL, intuitionistic, essential engine)
A ⊸ B, B ⊸ C ⊢ A ⊸ C
A[1] — A[7]
B[2] — B[4]
C[5] — C[8]
proof net (the correctness criterion holds)
```

## Why a sequent is unprovable

An "unprovable" verdict carries a reason, which `deduxit check`
verifies without the search ([Check a proof or a
refutation](check.md)):

- **An atom's count.** In every axiom an atom meets its negation. So if
  an atom occurs more often with one sign than the other, whichever
  additive alternatives a proof takes, the occurrences cannot all meet
  in axioms. Under `!` and `?` a copy adds occurrences; the count still
  refutes where all of the atom's occurrences there have one sign. Of a
  sequent written with two sides, the reason speaks of the sides: `A` on
  the right meets `A` on the left, and a negated formula or an
  implication's antecedent counts on the other side. Where a hypothesis
  is the atom left over and nothing is missing on the right, the line
  adds that `-a` lets a hypothesis go unused.
- **The count equation.** A provable one-sided sequent of MLL has exactly
  #⊗ − #⅋ − #1 + #⊥ + 2 formulas.
- **The state equation** of a Horn program, above.
- **The classical reading.** After a search that did not prove the
  sequent, the command reads it in classical propositional logic: `!`
  and `?` erased, `*` and `&` as "and", `par` and `+` as "or", `-o` as
  implication, `1` and `top` true, `bot` and `0` false. It looks for truth
  values of the atoms that make it false. No provable sequent has them,
  in any mode, so an unknown search becomes unprovable with the values as
  its reason, and an exhausted one gets them as a certificate. This
  search runs after the proof search, past the time limit, within
  `--classical-work N` formulas evaluated (2²⁶ by default; `0` turns it
  off), and never changes a verdict the proof search gave.
- **A saturated set** of the inverse method, above.
- **"The search was exhaustive"**: the search tried everything within
  its bounds and none of the above applies. Only the search vouches for
  it, so `check` cannot verify it.

The JSON output holds the same reason as `refutation`, written
one-sided.

## Threads

By default the search runs on one thread first. If that has not
decided within a tenth of a second (`--pool-after`), a pool of the
machine's other cores searches beside it, and the first to decide
answers. So a small sequent is decided at once and always the same way,
a hard one gets the machine, and what one thread decides within the
limit stays decided. The additive, Horn, essential and inverse engines
run on one thread and get no pool.

`--jobs N` (`-j`) runs N threads from the start (after one thread, with
`--pool-after`). `--deterministic` runs the sequential engines
throughout, whose proof and statistics are a function of the input. A
parallel run may find a different proof of the same sequent, never a
different verdict. The focus engine splits the choices nearest the root
among the threads and shares its memo; on a first-order sequent it
splits only the choices that no rule around them may ask for another
witness. The net engine splits its search into cubes at its first
choices, and where every link is forced it makes each link once, as one
thread does. A search never uses more threads than the machine runs at
once, and a larger `--jobs` is taken as that many, with a note:

<!-- readme-check: machine -->
```console
$ deduxit prove -q -j 4 --copies 3 --classical-work 0 "!(A -o A * A), !(B * B -o C), A, B |- A * A * A * ?D"
unknown (MELL, classical, focus engine): the copy bound of 3 was reached after 1.79ms; raise it with --copies N, or lift it with --copies none to deepen it while the time limit lasts
$ deduxit prove -q --deterministic --copies 3 --classical-work 0 "!(A -o A * A), !(B * B -o C), A, B |- A * A * A * ?D"
unknown (MELL, classical, focus engine): the copy bound of 3 was reached after 2.55ms; raise it with --copies N, or lift it with --copies none to deepen it while the time limit lasts
$ deduxit prove -q -j 10000 "A |- A"
note: --jobs 10000 is more than this machine runs at once; a search uses 16
provable (MLL, classical, focus engine)
```
