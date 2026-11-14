# Decide a sequent

`deduxit prove` decides whether a sequent is provable. Give the sequent
in quotes:

```console
$ deduxit prove "A, A -o B |- B"
provable (MLL, classical, focus engine)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B

$ deduxit prove "A & B |- A + B"
provable (ALL, classical, additive engine)
    ─────── ax
    ⊢ ~A, A
  ─────────── ⊕₁
  ⊢ ~A, A ⊕ B
──────────────── ⊕₁
⊢ ~A ⊕ ~B, A ⊕ B
```

The first line is the verdict. In brackets are the fragment the sequent
uses, the logic and the engine that searched. The proof follows as a
derivation tree, which the proof checker has verified. The tree is
one-sided: `⊢ ~A, A` is the axiom `A ⊢ A`. [The syntax](syntax.md) lists
the connectives, and `--file FILE` reads the sequent from a file
(`--file -` from standard input).

`deduxit prove -h` lists the common options by what they set (the logic,
the input, the output, the search, the limits); `deduxit prove --help`
lists every option.

## Read the verdict

There are three verdicts, and the exit status gives each to a script:

| verdict | exit status | meaning |
|---|---|---|
| `provable` | 0 | a proof was found and checked |
| `unprovable` | 1 | no proof exists; the line says why |
| `unknown` | 3 | a limit stopped the search first; the line says which and the flag to raise |

An error, such as a sequent that does not parse, exits with 2.

## Read why a sequent is unprovable

An unprovable verdict gives its reason after the colon:

```console
$ deduxit prove "|- A par B, ~A, ~B"
unprovable (MLL, classical, focus engine): the count equation fails: a provable one-sided sequent of MLL has exactly #⊗ − #⅋ − #1 + #⊥ + 2 formulas, here 0 − 1 − 0 + 0 + 2 = 1, and this one has 3
$ deduxit prove "A |- B"
unprovable (MLL, classical, focus engine): A on the left has no A on the right to meet in an axiom
$ deduxit prove "!A, A |- 1"
unprovable (MELL, classical, horn engine): A occurs 1 more time on the left than on the right outside the exponentials, and every occurrence of it below one, which copies add, is on the left, so they cannot all meet in axioms
$ deduxit prove "A, B |- A"
unprovable (MLL, classical, focus engine): B on the left has no B on the right to meet in an axiom; with -a (affine), an unused hypothesis is allowed
```

Most reasons count something: an atom whose occurrences cannot all
meet in axioms, or the multiplicative connectives against the formulas.
Where the counts say nothing, the reason is "the search was exhaustive".
[How deduxit searches](engines.md#why-a-sequent-is-unprovable) explains
each reason. `deduxit check` verifies a refutation without the search
([Check a proof or a refutation](check.md)).

## When the answer is unknown

With `!` and `?` a proof may copy formulas, and the search cannot try
every number of copies. So it deepens a bound on the copies until it
decides or the time limit passes, two seconds by default. Full linear
logic is undecidable, and for MELL no decision procedure is known, so
some sequents end unknown. The line says which limit ended the search
and how to raise it:

<!-- readme-check: machine -->
```console
$ deduxit prove -q "!(A & B) |- A * B"
provable (LL, classical, focus engine)
$ deduxit prove -q --copies 1 "!(A & B) |- A * B"
unknown (LL, classical, focus engine): the copy bound of 1 was reached after 54.08µs; raise it with --copies N, or lift it with --copies none to deepen it while the time limit lasts
$ deduxit prove -q "A |- !A"
unprovable (MELL, classical, focus engine): the search was exhaustive
$ deduxit prove -q --classical-work 0 "!(A -o A * A), !(A -o B), !(B * B -o C), A |- C * ?D"
unknown (MELL, classical, focus engine): the time limit of 2s was reached at a copy bound of 97; --timeout DURATION gives the search longer
```

`--timeout 10s` gives the search ten seconds, and `--timeout none` no
limit. `--copies N` stops the deepening at N copies. `--memory-limit`
(one gibibyte by default) bounds the memory; [Limits, defaults and exit
statuses](options.md) lists every limit.

After a search that found no proof, deduxit also reads the sequent in
classical propositional logic. If some truth values of the atoms make it
false, no proof exists, and an unknown becomes unprovable. The last
sequent above is refuted so (`--classical-work 0` turned this off
there):

```console
$ deduxit prove -q "!(A -o A * A), !(A -o B), !(B * B -o C), A |- C * ?D"
unprovable (MELL, classical, focus engine): the classical reading is false where A, B and C are true and every other atom false (! and ? erased, ⊗ and & as and, ⅋ and ⊕ as or, 1 and ⊤ true, ⊥ and 0 false), and every provable sequent reads as a tautology
```

## Get the same answer every time

An answer near the time limit depends on the machine's speed. For the
same verdict on every machine, bound the search with `--work-limit N` or
`--copies N` and lift the time limit with `--timeout none`.
`--deterministic` also makes the proof and `--stats` the same on every
run. `--stats` prints what the search did:

```console
$ deduxit prove -q --deterministic --stats --engine focus --bias rarer --copies 3 "!(A -o B), !(B -o C), !(C -o D), !(D -o E), A |- E"
unknown (MELL, classical, focus engine): the copy bound of 3 was reached after 36.65µs; raise it with --copies N, or lift it with --copies none to deepen it while the time limit lasts
stable sequents visited: 10
from the memo: 0
memo entries at most: 4
splits examined: 24
copy bound reached: 3
units of work: 250
time: 36.65µs
$ deduxit prove -q --deterministic --stats --engine focus --bias rarer --timeout none "!(A -o B), !(B -o C), !(C -o D), !(D -o E), A |- E"
provable (MELL, classical, focus engine)
stable sequents visited: 15
from the memo: 0
memo entries at most: 5
splits examined: 28
copy bound reached: 4
units of work: 380
time: 60.01µs
```

While a search runs longer than half a second, a line on standard error
says so, if standard error is a terminal. It goes when the answer comes.

## Choose the logic

By default a sequent is read in classical linear logic. Flags choose
another:

- `-a` (`--affine`): a hypothesis may go unused. The derivation shows
  the step as `wk`, weakening.
- `--mix`: the Mix rule joins two proofs side by side.
- `-i` (`--intuitionistic`): intuitionistic linear logic, below.
- `--cyclic`, `--lambek`: the ordered logics
  ([cyclic linear logic](cyclic.md), [the Lambek calculus](lambek.md)).
- `--logic classical|intuitionistic|minimal`: ordinary logic
  ([Decide ordinary logic](ordinary-logic.md)).

<!-- readme-check: machine -->
```console
$ deduxit prove -a "A, B |- A"
provable (MLL, classical affine, focus engine)
  ─────── ax
  ⊢ ~A, A
─────────── wk
⊢ ~A, ~B, A
$ deduxit prove -q -a --classical-work 0 "!(A -o A * A), A |- ?B"
unknown (MELL, classical affine, focus engine): the time limit of 2s was reached at a copy bound of 18; --timeout DURATION gives the search longer
```

### Intuitionistic linear logic

`-i` reads the sequent as written: the hypotheses left of `⊢`, exactly
one formula right of it, in the connectives `⊗ ⊸ & ⊕ !` and the units
`1 ⊤ 0`. The proof is two-sided, with the rules of intuitionistic linear
logic, and the verdict line names the intuitionistic fragment:

```console
$ deduxit prove -i "A -o B -o C, A * B |- C"
provable (IMLL, intuitionistic, two-sided engine)
           ───── ax   ───── ax
           B ⊢ B      C ⊢ C
───── ax   ──────────────── ⊸L
A ⊢ A        B ⊸ C, B ⊢ C
───────────────────────── ⊸L
  A ⊸ (B ⊸ C), A, B ⊢ C
  ────────────────────── ⊗L
  A ⊸ (B ⊸ C), A ⊗ B ⊢ C
$ deduxit prove -i "!A, !(A -o B) |- !B & A"
provable (ILL, intuitionistic, two-sided engine)
───── ax
A ⊢ A
────── !L   ───── ax
!A ⊢ A      B ⊢ B
───────────────── ⊸L         ───── ax
  !A, A ⊸ B ⊢ B              A ⊢ A
 ──────────────── !L         ────── !L
 !A, !(A ⊸ B) ⊢ B            !A ⊢ A
 ───────────────── !R   ──────────────── !w
 !A, !(A ⊸ B) ⊢ !B      !A, !(A ⊸ B) ⊢ A
 ─────────────────────────────────────── &R
          !A, !(A ⊸ B) ⊢ !B & A
```

A classical symbol (`~`, `^`, `par`, `|`, `?`, `bot`) is refused with
what to write instead. `--ill-syntax one-sided` reads the classical
syntax as the one-sided sequent it stands for, so `~(A * ~B), A |- B`
reads as `A ⊸ B, A ⊢ B`:

```console
$ deduxit prove -i "|- ~A par B"
error: cannot parse the sequent
  |- ~A par B
     ^ unexpected "~"
  intuitionistic linear logic has no negation: write a negated formula on the other side of |- (A |- B, not |- ~A, B), and ~A par B as A -o B; --ill-syntax one-sided reads the classical syntax
$ deduxit prove -i -q --ill-syntax one-sided "|- ~A par B"
unprovable (IMLL, intuitionistic, two-sided engine): A occurs 1 more time on the left than on the right, so they cannot all meet in axioms (an implication's antecedent counts on the other side)
$ deduxit prove -i --ill-syntax one-sided "|- A par B"
error: not an intuitionistic sequent: the subformula A ⅋ B is neither an intuitionistic formula nor the negation of one: intuitionistic linear logic has ⅋ only in A ⊸ B (~A ⅋ B), and ? only under a negation
$ deduxit prove -i "A |- B, C"
error: not an intuitionistic sequent: an intuitionistic sequent has exactly one formula right of ⊢, not 2: write the hypotheses left of ⊢
```

Classical linear logic proves more than intuitionistic linear logic
once `0` is around:

```console
$ deduxit prove -q "((A * top) & (B * top)) -o 0 |- (A -o C) + (B -o C)"
provable (MALL, classical, focus engine)
$ deduxit prove -i -q "((A * top) & (B * top)) -o 0 |- (A -o C) + (B -o C)"
unprovable (IMALL, intuitionistic, two-sided engine): the search was exhaustive
```

## Show the derivation

A derivation can be much larger than the proof: it writes the whole
sequent at every step and repeats every part the proof shares. So
deduxit shows it only where that makes sense, and otherwise keeps the
verdict and says in one line what it left out and how to get it.

On a terminal, the text tree is printed if its widest line fits the
terminal and it is at most three screens long. `--tree always` prints it
regardless, and `--tree never` leaves it out. Into a file or a pipe it
is always written. On a terminal 30 columns wide:

<!-- readme-check: terminal -->
```console
$ deduxit prove "A, A -o B, B -o C, C -o D |- D"
provable (MLL, classical, focus engine)
the proof tree is not shown: 7 inferences, at least 31 columns by 8 lines, for a terminal of 30 columns and at most 30 lines (3 screens); print it with --tree always, write it with --output FILE, or get the proof with --format json
```

In every format, a derivation estimated above `--derivation-limit`
(64 MiB by default; `none` lifts it) is not built. The line then goes to
standard error, unless the output is a terminal:

```console
$ deduxit prove --derivation-limit 100 "A, A -o B |- B" > verdict.txt
the derivation is not written: its 3 inferences with 29 characters of sequents are estimated at 616 B, over the limit of 100 B; --format json writes the proof itself, --derivation-limit SIZE raises the limit and --derivation-limit none lifts it
$ cat verdict.txt
provable (MLL, classical, focus engine)
```

`--compact always` draws a run of one structural rule, such as the
weakenings of every unused `?` formula, as one step whose label has a
star. The default, `--compact auto`, does so where the whole derivation
would pass `--derivation-limit` or not fit the terminal; `--compact
never` draws every step. Rocq certificates always have every step.

```console
$ deduxit prove -i --compact always '!A, !B, !C |- 1'
provable (IMELL, intuitionistic, horn engine)
     ─── 1R
     ⊢ 1
────────────── !w*
!A, !B, !C ⊢ 1
```

### Two-sided classical derivations

A classical derivation is one-sided by default. `--classical-sides two`
draws it with the formulas on the sides of `⊢` they were written on, in
every format but Rocq. The rules are named by side (`⊗L`, `⅋R`, `⊸L`,
…), and a step `~L` or `~R` moves a negated formula across `⊢`. A
sequent given as JSON, LLTP or `.spec` keeps no record of how it was
written, so its formulas take their one-sided form (`~A ⅋ B` for `A -o
B`), its antecedents on the left.

```console
$ deduxit prove --classical-sides two "A par B |- ~(~A * ~B)"
provable (MLL, classical, focus engine)
───── ax   ───── ax
A ⊢ A      B ⊢ B
──────────────── ⅋L
  A ⅋ B ⊢ A, B
  ───────────── ~L
  A ⅋ B, ~A ⊢ B
 ─────────────── ~L
 A ⅋ B, ~A, ~B ⊢
 ──────────────── ⊗L
 A ⅋ B, ~A ⊗ ~B ⊢
────────────────── ~R
A ⅋ B ⊢ ~(~A ⊗ ~B)
$ deduxit prove --classical-sides two --format latex "A -o B, A |- B"
% provable (MLL, classical, focus engine)
\begin{prooftree}
\infer0[$\mathrm{ax}$]{A &\vdash A}
\infer0[$\mathrm{ax}$]{B &\vdash B}
\infer2[$\multimap\mathrm{L}$]{A \multimap B, A &\vdash B}
\end{prooftree}
```

A first-order derivation is drawn so too, its quantifier steps named by
side (`∀L`, `∀R`, `∃L`, `∃R`) and every instance written out:

```console
$ deduxit prove --classical-sides two "|- ~(forall x. p(x)), p(a)"
provable (MLL1, classical, focus engine)
    ─────────── ax
    p(a) ⊢ p(a)
  ─────────────── ∀L
  ∀x. p(x) ⊢ p(a)
─────────────────── ~R
⊢ ~(∀x. p(x)), p(a)
```

The time limit and Ctrl-C also stop building and writing a derivation.
An `--output` file then holds a whole output or is left as it was.

## Get the outcome as JSON

`--format json` writes the outcome as one JSON object: the verdict, the
fragment, the mode, the engine, the statistics, and the proof or the
refutation. It is also a proof file, which `deduxit check` verifies
without the search:

```console
$ deduxit prove --format json "A |- A"
{"version":1,"deduxit":"…","verdict":"proved","checked":true,"fragment":"MLL","mode":"classical","engine":"focus","statistics":{"nodes":1,"memo_hits":0,"memo_entries":1,"splits":0,"links":0,"tests":0,"copies":0,"forward_copies":0,"work":16},"sequent":{"terms":[{"D":0},{"V":0}],"roots":[0,1],"atoms":["A"],"antecedents":1},"nodes":[{"ax":[0,1]}]}
$ deduxit prove --format json "A |- A" | deduxit check --quiet
valid proof of ⊢ ~A, A (classical)
```

A Rust program reads the outcome back with the library
([Use the library](library.md)). The other formats, LaTeX, Typst, SVG,
PNG, PDF and Rocq, are in [Export a proof](output-formats.md).
