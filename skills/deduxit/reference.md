# deduxit: the reference

> **dēdūxit** [deːˈduːk.sɪt] *Latin, verb*: "it has derived", the perfect of *dēdūcō*, to lead down, derive.

deduxit decides, checks and proves sequents of linear logic and of
ordinary propositional logic. This reference covers the `deduxit`
command (its flags, formats, JSON forms and exit statuses), then the
Rust library, the WebAssembly bindings and the Rocq library.

Every example is a real call with its real output; only the times vary.
`deduxit COMMAND -h` shows the common flags and `deduxit COMMAND --help`
every flag. A flag that sets a key of the settings file names the key in
parentheses there. The manual, <https://deduxit.github.io/deduxit/manual/>,
says the same at more length and has the benchmarks.

Contents:

1. Running it
2. Writing a sequent
3. Choosing the logic
4. Deciding a sequent: `prove`
5. Why unprovable, why unknown
6. Limits and the search
7. Output formats
8. JSON forms
9. Checking a proof: `check`
10. Step by step: `interact`
11. A sequent alone: `seq`
12. Many sequents in one call
13. Ordinary logic
14. Cyclic linear logic and the Lambek calculus
15. Settings files and styles
16. The Rust library
17. The web bindings
18. Rocq certificates and the Rocq library
19. Mistakes to avoid

## 1. Running it

Install the command `deduxit` with `cargo install --locked deduxit-cli`,
or run it without installing with `nix run github:deduxit/deduxit -- ARGS`.
Each GitHub release also has it prebuilt for Linux (an archive, and
`.deb`, `.rpm` and Arch Linux packages), macOS and Windows; the manual's
installation chapter says how to install each.

The commands:

- `prove` decides a sequent (sections 4 to 7), or many (section 12);
- `check` checks a proof or a refutation (section 9);
- `eliminate` eliminates the cuts of a proof (section 10);
- `interact` proves a sequent step by step (section 10);
- `seq print`, `seq json` and `seq fragment` show a sequent (section 11).

The exit status is the same for every command:

| status | `prove` | `check` | `interact` |
|---|---|---|---|
| 0 | provable (`valid` under `--logic`) | valid proof | the session ended with a checked proof |
| 1 | unprovable (`not valid`) | invalid proof | it ended without one |
| 2 | an error: bad input, a flag that does not apply, an engine forced on a sequent it cannot search | an error | the session could not start or read its input |
| 3 | unknown: a time limit, a bound, a limit or Ctrl-C ended the search | a limit gave the check up | |

A batch exits with its worst entry: an error (2) before unknown (3)
before unprovable (1) before proved (0). A reader that closes the pipe
early (`| head -1`) changes no status. The first Ctrl-C stops the
search, whose verdict is then unknown; a second one ends the command
with status 130.

```console
$ deduxit prove -q "A |- A"; echo "exit $?"
provable (MLL, classical, focus engine)
exit 0
$ deduxit prove --engine net "A & B |- A"; echo "exit $?"
error: proof nets exist for MLL without units only, not for ALL
exit 2
$ echo '{"version":1,"sequent":{"terms":[{"V":0},{"V":1}],"roots":[0,1],"atoms":["A","B"]},"nodes":[{"ax":[0,1]}]}' | deduxit check; echo "exit $?"
invalid proof of ⊢ A, B (classical): node 0 (ax on A, B): the literals are not an atom and its negation
exit 1
$ deduxit prove --format json "A * B |- B * A" | deduxit check --work-limit 1; echo "exit $?"
unknown: the proof of ⊢ ~A ⅋ ~B, B ⊗ A (classical) is not checked: the check stopped at node 1 without a verdict on the proof: the call did the most work allowed, 1 unit; raise it with --work-limit N
exit 3
$ echo quit | deduxit interact "A |- A"; echo "exit $?"
exit 1
```

## 2. Writing a sequent

A sequent is `A, B |- C, D` (or with `⊢`). Either side may be empty: a
formula alone is `|- A`. Quote the sequent for the shell.

| connective | ASCII | Unicode |
|---|---|---|
| tensor | `A * B` | `A ⊗ B` |
| par | `A par B`, `A \| B` | `A ⅋ B` |
| with | `A & B` | |
| plus | `A + B` | `A ⊕ B` |
| linear implication | `A -o B` | `A ⊸ B` |
| negation | `~A`, `A^` | |
| exponentials | `!A`, `?A` | |
| units | `1`, `bot`, `top`, `0` | `⊥`, `⊤` |
| quantifiers | `forall x. A`, `exists x. A`, `forall x y. A` | `∀x. A`, `∃x. A` |
| predicates | `p(x, f(a))` | |

Binding, tightest first: `A^`, then `~ ! ?`, then `*`, `|`, `&`, `+`,
and last `-o`, which groups to the right. A quantifier binds loosest and
reaches right as far as it can (to `)`, `,`, `|-` or the end), so
`forall x. p(x) -o q(x)` is `∀x. (p(x) ⊸ q(x))`.

### Atoms and quantifiers

An atom is an identifier, or a predicate applied to terms in brackets.
Inside the brackets, an identifier that an enclosing quantifier binds is
its variable; any other identifier is a constant, or a function symbol
before a bracket. Each symbol has one arity throughout, a name is a
predicate or a function symbol but not both, and no variable is free. A
sequent with arguments and no quantifier is propositional, and every
engine decides it.

What each command does with a sequent that has a quantifier:

- JSON reads and writes it at wire level 7 (section 8), and `check`
  checks its proofs (section 9).
- `prove` decides it in the classical modes (with `--mix` and
  `--affine` too) with the focused engine, and with `-i` (and
  `--affine`) with the two-sided engine. MLL1 and MALL1 are decided, and a
  sequent with `!` and `?` within the copy bound.
- The search leaves the witness of an `∃` open and fills it in by
  unification where an axiom matches two atoms. The proof names every
  witness.
- The memo and the loop check know a sequent by its instances, so a `!`
  hypothesis that leads back to where it started is refuted.
  `--memo-limit 0` switches both off.
- `--jobs` shares the choices nearest the root among threads, where no
  rule around them may ask for another witness.
- `--stats` adds the unifications tried and the metavariables bound (in
  JSON `unifications` and `bindings`, written only where not zero).
- `interact` proves it step by step with the quantifier rules (section
  10), and `--format rocq` certifies its proofs in the Rocq library's
  first-order development (section 18).
- The proof-net outputs (`--net`) refuse it with an error, exit status 2.

```console
$ deduxit seq print "forall x. p(x) -o q(x), p(a) |- exists y. q(y)"
⊢ ∃x. (p(x) ⊗ ~q(x)), ~p(a), ∃y. q(y)
$ deduxit seq fragment "forall x. p(x) |- p(a)"
MLL1
$ deduxit prove "forall x. p(x) |- p(a)"; echo "exit $?"
provable (MLL1, classical, focus engine)
  ───────────── ax
  ⊢ p(a), ~p(a)
───────────────── ∃
⊢ ∃x. ~p(x), p(a)
exit 0
$ deduxit prove -q "forall x. exists y. r(x, y) |- exists y. forall x. r(x, y)"; echo "exit $?"
unprovable (MLL1, classical, focus engine): the search was exhaustive
exit 1
$ deduxit prove -i "forall x. p(x) |- p(a)"
provable (IMLL1, intuitionistic, two-sided engine)
  ─────────── ax
  p(a) ⊢ p(a)
─────────────── ∀L
∀x. p(x) ⊢ p(a)
```

### The one-sided sequent

The search works on the one-sided sequent: `Γ ⊢ Δ` becomes `⊢ ~Γ, Δ` in
negation normal form, and `A -o B` becomes `~A par B`. `seq print` shows
it:

```console
$ deduxit seq print "A * B -o C |- A -o B -o C"
⊢ (A ⊗ B) ⊗ ~C, ~A ⅋ (~B ⅋ C)
```

### Reading a file

`--file PATH` (or `-f`; `-` is standard input) reads the sequent from a
file. Its extension gives its kind, unless `--input-format` names
another:

- `.p`: a problem of the LLTP library (under `--logic`, a TPTP problem);
- `.spec`: a coverability problem in the format of the Mist tool, always
  decided in affine mode;
- `.json`: a sequent in JSON (section 8);
- anything else: the text syntax, which may span lines.

Two limits refuse a sequent before anything unfolds it, with exit
status 2. By default a sequent has at most `--occurrence-limit 50000000`
subformula occurrences (`none` lifts it), and its atoms' arguments
unfold to at most `--term-limit 50000000` first-order terms.

## 3. Choosing the logic

| flag | logic |
|---|---|
| none | classical linear logic |
| `-i`, `--intuitionistic` | intuitionistic linear logic: hypotheses left of `⊢`, exactly one formula right of it, in its own syntax (`⊗ ⊸ & ⊕ !`, `1 ⊤ 0`); `~`, `^`, `par`, `\|`, `?` and `bot` are refused with what to write instead, unless `--ill-syntax one-sided` reads the classical syntax as the one-sided sequent it stands for |
| `-a`, `--affine` | a hypothesis may go unused (weakening); with `-i` intuitionistic affine logic |
| `--mix` | the Mix rule, which proves `⊢ Γ, Δ` from `⊢ Γ` and `⊢ Δ` |
| `--cyclic` | cyclic linear logic, unit-free MLL in a fixed order (section 14) |
| `--lambek` | the Lambek calculus, `--empty-antecedents` for L* (section 14) |
| `--logic classical\|intuitionistic\|minimal` | ordinary propositional logic, by LJT for intuitionistic and minimal logic and through a translation into linear logic for classical logic (section 13) |

deduxit detects the fragment (MLL, MLL with units, ALL, MALL, MELL, LL,
and IMLL to ILL in intuitionistic mode), and the fragment picks the
engine. `seq fragment` names it:

```console
$ deduxit seq fragment "A & B |- 1"
MALL
$ deduxit seq fragment -i "A & B |- 1"
IMALL
```

With `-i`, a classical symbol or a second formula right of `|-` is an
error that says what to write instead:

```console
$ deduxit prove -i "|- ~A par B"
error: cannot parse the sequent
  |- ~A par B
     ^ unexpected "~"
  intuitionistic linear logic has no negation: write a negated formula on the other side of |- (A |- B, not |- ~A, B), and ~A par B as A -o B; --ill-syntax one-sided reads the classical syntax
$ deduxit prove -i "A |- B, C"
error: not an intuitionistic sequent: an intuitionistic sequent has exactly one formula right of ⊢, not 2: write the hypotheses left of ⊢
```

Classical linear logic proves more than intuitionistic linear logic, for
example with `0`:

```console
$ deduxit prove -q "((A * top) & (B * top)) -o 0 |- (A -o C) + (B -o C)"
provable (MALL, classical, focus engine)
$ deduxit prove -i -q "((A * top) & (B * top)) -o 0 |- (A -o C) + (B -o C)"
unprovable (IMALL, intuitionistic, two-sided engine): the search was exhaustive
```

## 4. Deciding a sequent: `prove`

`deduxit prove SEQUENT` prints the verdict line and, for a provable
sequent, a derivation. The proof checker, which shares no code with the
search, has passed every proof before it is printed. The verdict line is
`provable`, `unprovable` or `unknown`, then in parentheses the fragment,
the mode and the engine, and after a colon the reason.

```console
$ deduxit prove "A, A -o B |- B"
provable (MLL, classical, focus engine)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B
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
```

A classical derivation is one-sided. `--classical-sides two` draws it
two-sided, as the sequent was written: the rules are named by side
(`⊗L`, `⅋R`, …), and a step `~L` or `~R` moves a formula where a negation
was written. A first-order derivation then has `∀L`, `∀R`, `∃L` and `∃R`,
with its instances written out. Intuitionistic derivations are always
two-sided.

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
```

What `prove` prints:

- `-q` (`--quiet`) prints the verdict line alone.
- `--stats` adds the search's counters, a line each, and the time.
- `--no-check` reports a proof without running the checker. With the
  check, a proof the checker rejects is an error (exit status 2), never a
  verdict.
- On a terminal, the text tree is printed only if it fits the screen.
  `--tree always` prints it whatever its size, and `--tree never` leaves
  it out. Into a file or a pipe it is always written.

### Engines

`--engine NAME` picks the engine. The default is `--engine auto`, the
engine the fragment and the mode call for.

| engine | searches |
|---|---|
| `focus` | focused sequent search, every classical fragment, with or without units, Mix, exponentials and weakening |
| `two-sided` | the same for every intuitionistic fragment |
| `net` | proof-net search, MLL without units, with or without Mix, and IMLL without units by its embedding; the engine of the cyclic mode |
| `additive` | two additive-only formulas, in every mode |
| `horn` | the markings of a Petri net, for Horn programs (atoms, implications between tensors of atoms, such implications under `!`, one goal that is a tensor of atoms): reachability in linear mode, coverability in affine mode |
| `essential` | essential nets, IMLL without units; the engine of the Lambek calculus, elsewhere only when named |
| `inverse` | the focused inverse method, a forward search from the axioms, in every mode but the cyclic and Lambek ones; it runs by itself only when named |

`--engine auto` also gives the inverse method turns beside the default
engine, on intuitionistic goals with an exponential and under Mix. On
unit-free MLL without weakening whose atoms occur once with each sign,
an engine that stops at its recursion or memory limit hands the sequent
over: to the inverse method after the memory limit in classical mode
without Mix, to the net engine otherwise.

`--fragment NAME` (`mll`, `mll-units`, `all`, `mall`, `mell`, `ll`)
searches in that fragment instead of the detected one. A sequent outside
it is an error.

### The lambda-term: `--lambda`

`--lambda` prints the program an intuitionistic proof is: its typing
judgement, on a line after the derivation.

- A proof with `-i` is a term of the linear lambda calculus of Benton,
  Bierman, de Paiva and Hyland (Bierman's thesis, UCAM-CL-TR-346,
  chapter 3); with `-a` too, a term of its affine variant.
- A derivation of LJ (`--logic intuitionistic` or `minimal`) is a simply
  typed term, with `¬A` read as `A → ⊥`.
- A cut is a redex `(λx:A. N) M`.
- A proof with no term (classical, Mix, cyclic, Lambek, first-order, LK)
  gets `no lambda-term:` and the reason.

`--lambda-notation ascii` writes the term in ASCII, and `--lambda-types`
types every binder (`lambda.notation`, `lambda.types`). `check --lambda`
and the session command `lambda` print the same. LaTeX, Typst, SVG, PNG
and PDF write the judgement under the conclusion. JSON adds the key
`lambda`: the term's nodes, types and variables with `text` and
`judgement`, or `error`.

```console
$ deduxit prove -i --lambda --tree never "|- a -o b -o a * b"
provable (IMLL, intuitionistic, two-sided engine)
lambda-term: ⊢ λx. λy. x ⊗ y : a ⊸ (b ⊸ (a ⊗ b))
$ deduxit prove --logic intuitionistic --lambda --tree never "|- (a -> b) -> (b -> c) -> a -> c"
valid (intuitionistic logic by LJT)
lambda-term: ⊢ λf. λg. λx. g (f x) : (a → b) → ((b → c) → (a → c))
$ deduxit prove -i --lambda --tree never "a & b |- b & a"
provable (IALL, intuitionistic, additive engine)
lambda-term: x : a & b ⊢ ⟨snd(x), fst(x)⟩ : b & a
```

## 5. Why unprovable, why unknown

### Unprovable

An unprovable verdict gives its reason. The JSON `refutation` holds the
same, tagged by `kind`:

| `kind` | the verdict line says |
|---|---|
| `unbalanced` | an atom's literals cannot all meet their duals in axioms |
| `one_sided` | the same, counted outside the exponentials |
| `unbalanced_symbol`, `one_sided_symbol` | the same for a predicate with a bound variable in one of its atoms, every atom of it counted together (`"symbol"` its index) |
| `equation` | the count equation of the multiplicatives fails |
| `state_equation` | a Horn program's state equation has no solution: weights for the atoms that no clause raises and the goal needs raised |
| `classical` | the sequent is false read classically, with the atoms' values: no provable sequent is |
| `saturated` | the inverse method's saturated set, closed under the rules, lacks the goal |
| `exhausted` | the search was exhaustive |

For a sequent written with formulas left of `⊢`, the counts speak of its
sides; a negated formula or an implication's antecedent counts on the
other side. Where a hypothesis is the atom left over and nothing is
missing on the right, the line adds that `-a` lets a hypothesis go
unused.

```console
$ deduxit prove -q "A, B |- A"
unprovable (MLL, classical, focus engine): B on the left has no B on the right to meet in an axiom; with -a (affine), an unused hypothesis is allowed
$ deduxit prove "|- A par B, ~A, ~B"
unprovable (MLL, classical, focus engine): the count equation fails: a provable one-sided sequent of MLL has exactly #⊗ − #⅋ − #1 + #⊥ + 2 formulas, here 0 − 1 − 0 + 0 + 2 = 1, and this one has 3
$ deduxit prove -q "!(A -o A * A), !(A -o B), !(B * B -o C), A |- C * ?D"
unprovable (MELL, classical, focus engine): the classical reading is false where A, B and C are true and every other atom false (! and ? erased, ⊗ and & as and, ⅋ and ⊕ as or, 1 and ⊤ true, ⊥ and 0 false), and every provable sequent reads as a tautology
$ deduxit prove -q "!(a -o a * a), !(b * b -o c), a, b |- c"
unprovable (MELL, classical, horn engine): the state equation of the Petri net has no solution, so no firing of its clauses yields the goal's atoms: weighting each a by −2, b by 1 and c by 2, no clause that can fire raises the weighted count of the atoms, and the goal asks it raised
```

The classical refutation runs after a search that did not prove the
sequent, within `--classical-work N` formulas evaluated. The default is
`--classical-work 67108864`, and `0` switches it off. It runs past the
time limit, within that bound. It can turn an unknown search into
unprovable, and it never changes a verdict the search gave.

### Unknown

Unknown means a limit ended the search before it decided. It is not
unprovable. MELL has no known decision procedure and full linear logic
is undecidable, so with `!` or `?` the search deepens a bound on how
often a `?` formula is copied on one branch.

The line says which limit ended the search, after how long, at which
copy bound, and which flag raises the limit. The JSON `reason` is tagged
by `kind`: `stopped`, `copy_bound`, `memory_limit`, `work_limit`,
`recursion_limit`, `index_limit`, `unchecked`, and for a first-order
search `witness_depth` and `instances`.

```console
$ deduxit prove -q --copies 1 "!(A & B) |- A * B"
unknown (LL, classical, focus engine): the copy bound of 1 was reached after 3.56ms; raise it with --copies N, or lift it with --copies none to deepen it while the time limit lasts
$ deduxit prove --memory-limit 100 "|- (a & b) + (a & c), ~a par (~b & ~c)"
unknown (MALL, classical, focus engine): the memory limit of 100 B was reached after 32.28µs; raise it with --memory-limit SIZE
```

A time limit's line says at which copy bound the search was:

<!-- readme-check: machine -->
```console
$ deduxit prove -q --timeout 1s --classical-work 0 "!(A -o A * A), !(A -o B), !(B * B -o C), A |- C * ?D"; echo "exit $?"
unknown (MELL, classical, focus engine): the time limit of 1s was reached at a copy bound of 46; --timeout DURATION gives the search longer
exit 3
```

## 6. Limits and the search

A search that reaches a limit on its time, copies, memory, work,
recursion, witnesses or instances is unknown (exit status 3).

| flag | default | what it bounds |
|---|---|---|
| `--timeout DURATION` | `2s` | the whole command, reading included (`500ms`, `10s`, `2m`, `1h`, `none`) |
| `--copies N` | `none` | how often a `?` formula may be copied on one branch; by default the bound deepens until the search decides or the time is up |
| `--forward-copies N` | `30` | the forward search's copy bound on a Horn program, under `--copies` |
| `--memory-limit SIZE` | `1GiB` | the search's memory (`512MiB`, `4GiB`, `none`) |
| `--work-limit N` | `none` | units of work: the same verdict on every machine |
| `--recursion-limit N` | `2048` | the nesting of rules on one branch |
| `--memo-limit N` | `1048576` | the decided sequents remembered at once; a full memo is emptied, which never changes a verdict |
| `--classical-work N` | `67108864` | the classical refutation after the search (section 5) |
| `--witness-depth N` | `64` | the deepest term a first-order search fills a witness in with |
| `--instances N` | `65536` | the instances of quantified formulas a first-order search holds at once |
| `--derivation-limit SIZE` | `64MiB` | the derivation built from a proof; above it the verdict stands without it |
| `--occurrence-limit N` | `50000000` | the sequent's subformula occurrences; a larger sequent is refused, exit status 2 |
| `--term-limit N` | `50000000` | the first-order terms the sequent's atoms unfold to, and a proof's instance table; past it, refused |

### Threads

One thread searches first. The default is `--pool-after 100ms`: after
that, the machine's other threads join it. `-j N` (`--jobs`) uses N
threads from the start. `--deterministic` runs one thread throughout.

For the same verdict on every machine, bound the search with `--work-limit
N` or `--copies N` and lift the time limit with `--timeout none`.
`--deterministic` also makes the proof and `--stats` the same on every run.

```console
$ deduxit prove -q --deterministic --stats "!(a * a -o b), !(b * b -o c), !(c * c -o d), a, a, a, a, a, a, a, a |- d"
provable (MELL, classical, horn engine)
markings reached: 7
markings met again: 0
markings kept: 7
units of work: 6
time: 45.06µs
```

### Bias

`--bias auto|rarer|factors` chooses how the focused engines pick each
atom's positive literal. It changes the search, never what is provable.

## 7. Output formats

`--format text|json|latex|typst|svg|png|pdf|rocq` names the format
(`--format tex` is read as `latex`). Without it, `--output FILE` (`-o`)
takes the format its extension names: `.txt`, `.json`, `.tex`, `.typ`,
`.svg`, `.png`, `.pdf` or `.v`.

A file is made only when it holds a derivation or a net, and in JSON
always. For an unprovable sequent the verdict goes to standard error and
no file is made. PNG and PDF are never written to a terminal.

- `latex`: the verdict as a comment, then a proof tree for the ebproof
  package, with the connectives of cmll and amssymb.
- `typst`: a proof tree for the curryst package, or, for a high tree,
  in deduxit's own layout (`--style layout=deduxit` asks for it always).
- `svg`, `png`, `pdf`: the derivation drawn, set in Euler Math unless
  `--font stix-two|garamond|asana|new-computer-modern` names another.
- `rocq`: a certificate (section 18).

Three flags change any format:

- `--standalone` writes a document that compiles on its own (latex,
  typst, rocq).
- `--no-verdict` leaves the verdict line out.
- `--compact always|auto|never` draws a run of one structural rule as
  one inference with a starred label (`?w*`).

```console
$ deduxit prove -i --format latex "A, A -o B |- B"
% provable (IMLL, intuitionistic, two-sided engine)
\begin{prooftree}
\infer0[$\mathrm{ax}$]{A &\vdash A}
\infer0[$\mathrm{ax}$]{B &\vdash B}
\infer2[$\multimap\mathrm{L}$]{A, A \multimap B &\vdash B}
\end{prooftree}
$ deduxit prove --output proof.svg "A, A -o B |- B"
$ deduxit prove -i --output proof.pdf "1, A & B, B -o C |- C"
provable (IMALL, intuitionistic, two-sided engine)
```

### Proof nets: `--net`

`--net` writes the proof net instead of the derivation. It takes MLL
without units (with or without Mix), and MELL without units with
`!`-boxes, classical or intuitionistic.

- As text: the sequent, the axiom links as pairs of literals with their
  positions, and the correctness criterion's verdict.
- As `json`: the net's document.
- As `svg`, `png` or `pdf`: the net drawn.

```console
$ deduxit prove --net "|- A * B, C * (~A par ~B), ~C"
provable (MLL, classical, focus engine)
⊢ A ⊗ B, C ⊗ (~A ⅋ ~B), ~C
A[1] — ~A[6]
B[2] — ~B[7]
C[4] — ~C[8]
proof net (the correctness criterion holds)
$ deduxit prove --engine net "A & B |- A"
error: proof nets exist for MLL without units only, not for ALL
```

## 8. JSON forms

### Sequents

Every document carries `version`, its wire level. A sequent in JSON has
these keys:

- `atoms`: the atoms' names. Terms refer to an atom by its index.
- `terms`: `{"V": i}` is atom `i` and `{"D": i}` its dual. Every other
  term is an object of one key that names earlier terms by index
  (`{"⊗": [0, 1]}`, `⅋`, `&`, `⊕`, `{"!": 2}`, `{"?": 2}`). A unit is
  its symbol (`"1"`, `"⊥"`, `"⊤"`, `"0"`).
- `roots`: the formulas, in order.
- `antecedents`: how many of the roots stood left of `⊢`.

`seq json -i` writes the intuitionistic form, with `hypotheses` and
`goal` in place of `roots`.

```console
$ deduxit seq json "A |- A"
{"version":1,"terms":[{"D":0},{"V":0}],"roots":[0,1],"atoms":["A"],"antecedents":1}
$ deduxit seq json -i "A * B -o C |- A -o B -o C"
{"version":5,"terms":[{"V":0},{"V":1},{"⊗":[0,1]},{"V":2},{"⊸":[2,3]},{"⊸":[1,3]},{"⊸":[0,5]}],"atoms":["A","B","C"],"hypotheses":[4],"goal":6}
```

### Outcomes and proofs

`prove --format json` writes the outcome, with these keys:

- `verdict`: `proved`, `unprovable` or `unknown`;
- `checked`, for a proof: whether the checker passed it;
- `refutation` for an unprovable sequent, `reason` for an unknown one
  (section 5);
- `fragment`, `mode`, `engine`, and `statistics` (the counters by name);
- for a proof, its `sequent` and `nodes`, so that `check` reads the
  outcome as a proof.

A node is one rule instance. Premises come before conclusions, and the
root is last. `{"ax": [x, y]}` is an axiom on two literals, `{"⊗": [o,
l, r]}` a rule on occurrence `o` with its two premises' indices, and
`{"⅋": [o, p]}` one with one premise. An occurrence is a subformula's
number in the sequent's numbering.

A proof with a cut (from `interact`'s `cut`) is at wire level 6. Its
sequent's `cuts` name the cut formulas, numbered after the sequent's
own, and a cut is `{"cut": [a, l, r]}`.

```console
$ deduxit prove --format json "A |- A"
{"version":1,"deduxit":"…","verdict":"proved","checked":true,"fragment":"MLL","mode":"classical","engine":"focus","statistics":{"nodes":1,"memo_hits":0,"memo_entries":1,"splits":0,"links":0,"tests":0,"copies":0,"forward_copies":0,"work":16},"sequent":{"terms":[{"D":0},{"V":0}],"roots":[0,1],"atoms":["A"],"antecedents":1},"nodes":[{"ax":[0,1]}]}
$ deduxit prove --format json "A |- B"
{"version":1,"deduxit":"…","verdict":"unprovable","refutation":{"kind":"unbalanced","atom":0,"least":-1,"most":-1},"fragment":"MLL","mode":"classical","engine":"focus","statistics":{"nodes":1,"memo_hits":0,"memo_entries":1,"splits":0,"links":0,"tests":0,"copies":0,"forward_copies":0,"work":16},"sequent":{"terms":[{"D":0},{"V":1}],"roots":[0,1],"atoms":["A","B"],"antecedents":1}}
```

### Errors

An error of the library, in a batch record, in `check`'s JSON or from
the web bindings, is `{"code", "kind", "message", "setting", "details"}`.
`code` is the stable reason to branch on. `setting` is the settings key
of the bound that refused the call, where a bound did.

## 9. Checking a proof: `check`

`deduxit check FILE` (or `--file FILE`, or standard input) reads a proof
as `prove --format json` writes it. The checker shares no code with the
search.

- The mode is the one the file's `"mode"` states, or the flags' for a
  file that states none. A mode flag that names another mode is an error
  that names both, exit status 2.
- `--format json` writes `{"version", "verdict", "mode"}`. The verdict is
  `valid`, `invalid` or `unknown`, with the library's `error` for the
  last two.
- The default time limit is `--timeout 2s`. `--work-limit` and
  `--memory-limit` bound the check too. A check that a limit gives up is
  unknown, exit status 3.
- It takes `prove`'s output formats and `--net`.

```console
$ deduxit prove --format json "A |- A" | deduxit check --quiet
valid proof of ⊢ ~A, A (classical)
$ deduxit prove --format json "A |- A" | deduxit check --format json
{"version":1,"verdict":"valid","mode":"classical"}
```

### Refutations

`check` also checks an unprovable outcome (or a disproof document, which
has the same keys): its refutation (section 5), without the search, in
the mode the file states. The exit status is:

- 0 for a refutation that holds;
- 1 for one that does not (error code `invalid_refutation`);
- 3 for `exhausted`, which only the search vouches for (code
  `uncertified`), and at the time limit.

Every other kind has a check. With `--format rocq`, a refutation that
holds is written as its certificate (section 18).

```console
$ deduxit prove -i --format json "a, a -o b, !(b -o b * c), !(b -o d) |- b * d" | deduxit check --format json
{"version":1,"verdict":"valid","mode":"intuitionistic"}
$ deduxit prove --format json "A & B |- A * B" | deduxit check; echo "exit $?"
unknown: the refutation of A & B ⊢ A ⊗ B (classical) is not checked: a refutation of kind `exhausted` rests on the search and has no check
exit 3
```

### First-order proofs

A proof of a first-order sequent (wire level 7) adds the steps
`{"∀": [member, eigenvariable, premise]}` and `{"∃": [member, term,
premise]}`, and the key `instances`:

- `members`: each `[occurrence, frame]`, for a member past the
  sequent's occurrences (an occurrence under quantifiers, with the terms
  they were opened with);
- `frames`: each `[parent, term]`, the innermost quantifier's term
  first, `null` past the outermost;
- `terms`: the proof's own terms after the sequent's `fo`, an
  eigenvariable written `{"E": k}`.

`check` holds such a proof to the eigenvariable condition. An
eigenvariable that a `∀` step introduces is introduced by that step
alone, and only witnesses above that step name it. An eigenvariable that
no step introduces is a constant outside the signature. The error names
the eigenvariable that breaks the condition (`eigenvariable a occurs
outside the ∀ step that introduces it`).

```console
$ echo '{"version":7,"sequent":{"terms":[{"D":0},{"∃":0},{"V":1}],"roots":[1,2],"atoms":[{"P":[0,[0]]},{"P":[0,[2]]}],"symbols":[["p",1],["a",0],["f",1]],"fo":[{"B":0},{"F":[1,[]]},{"F":[2,[1]]}],"binders":["x"],"antecedents":1},"nodes":[{"ax":[3,2]},{"∃":[0,2,0]}],"instances":{"members":[[1,0]],"frames":[[null,2]],"terms":[]}}' | deduxit check --quiet
valid proof of ⊢ ∃x. ~p(x), p(f(a)) (classical)
```

## 10. Step by step: `interact`

`deduxit interact SEQUENT` (or `--file`) starts a session with the
sequent as goal 0. It reads one command per line from standard input.

- A formula is named by its position in its goal, from 0, as `goals`
  lists them.
- A step closes its goal and opens its premises as new goals, with new
  numbers. Each new goal is listed with its own positions (`opened goal
  1: ⊢ 0: ~A, 1: A`); a premise the step closes at once is `closed`.
- In `apply G P RULE [P…]` every position is one of goal G before the
  step. For `⊗`, the further positions are G's other formulas that go to
  the left premise; the rest go to the right. `ax` takes either of its
  two literals.
- A command that fails prints `error: …`, and the session goes on. A
  script that misnumbers a goal therefore goes on failing: read the
  numbers from the output before you write the next step.
- The session exits with 0 when it ends with a checked proof, and with 1
  otherwise.

| session command | what it does |
|---|---|
| `goals` | the open goals, with the positions of their formulas |
| `rules G P` | the rules that act on formula P of goal G |
| `apply G P RULE [P…]` | applies a rule; for `⊗` or Mix the further positions go to the left premise |
| `apply G P RULE [T]` | `∃R` or `∀L` (`exists` one-sided) with the witness T, or open without one |
| `cut G C [P…]` | cuts goal G on the formula C (quoted if it has blanks): the positions P go to the first premise, which proves C, the others to the second, which uses it |
| `undo` | retracts the last step |
| `invertible on\|off` | applies every invertible rule by itself, or stops doing so |
| `close [G]` | lets the search close goal G, or every open goal |
| `show [--FORMAT] [FILE]` | the derivation so far, open goals included: text, or `--latex`, `--typst`, `--svg`, `--png`, `--pdf` (these two need a FILE) |
| `proof [--FORMAT] [FILE]` | checks the finished proof and prints it or writes it: text, `--json`, `--latex`, `--typst`, `--svg`, `--png`, `--pdf` or `--rocq`; a FILE without a format takes its extension's, else JSON |
| `lambda` | the finished proof's lambda-term, as `--lambda` prints it, or why it has none |
| `save FILE` | writes the session as JSON, which `load` and `--state` read and `check` does not |
| `load FILE` | resumes a session of the same sequent and mode |
| `help` | the list |
| `quit` | ends the session |

### Rules

A RULE is its usual name or its ASCII spelling:

- one-sided: `ax`, `1`, `bot`, `top`, `*`, `par`, `&`, `+1`, `+2`, `?d`,
  `?c`, `?w`, `!`, `mix`, `wk`;
- intuitionistic: `-oL`, `-oR`, `*L`, `*R`, `&L1`, `&L2`, `&R`, `+L`,
  `+R1`, `+R2`, `1L`, `1R`, `0L`, `topR`, `!L`, `!R`, `!c`, `!w`;
- quantifiers: `forall` and `exists` one-sided, and `forallR`,
  `existsL`, `existsR`, `forallL` two-sided.

`rules` prints the names it applies. An unknown name's error lists the
session's own rules, the ASCII spelling first. A split that leaves a
premise without a proof (the atoms' counts show it) is made with a
warning, and `undo` takes it back:

```console
$ printf 'apply 0 2 *\nundo\napply 0 2 tensor 0\n' | deduxit interact "A, B |- A * B"
opened goal 1: ⊢ 0: A
opened goal 2: ⊢ 0: ~A, 1: ~B, 2: B
warning: one premise has no proof, so this split cannot be closed; `undo` takes it back
reopened goal 0: ⊢ 0: ~A, 1: ~B, 2: A ⊗ B
error: unknown rule `tensor`; the rules here are ax, * (⊗), par (⅋), 1, bot (⊥), &, +1 (⊕₁), +2 (⊕₂), top (⊤), !, ?d, ?c and ?w
```

`--state FILE` resumes a saved session; the mode is then the file's.
`--invertible` applies the invertible rules from the start. The search
flags of `prove` apply to `close`.

### Quantifiers

- `∀R` and `∃L` (`forall` one-sided) open the formula with a fresh
  eigenvariable, written `a`, `b`, … (a letter that no symbol takes).
- `∃R` and `∀L` (`exists` one-sided) take the witness after the rule's
  name. It is a term of the sequent's function symbols and of the
  eigenvariables of the `∀R` and `∃L` steps below the goal.
- Without a witness, it stays open, written `$1`, `$2`, …. An axiom
  fills it by unification, or `close` by the search.
- An open witness never takes an eigenvariable made after it (the
  eigenvariable condition): `error: the eigenvariable b cannot be used
  here: …`.
- `undo` of the step that filled a witness opens it again.
- `save` keeps the witnesses (the session's `instances`, wire level 7)
  with each step's mark, so `undo` after `--state` drops what the step
  made.
- `cut` takes a formula of the sequent's symbols, quantifiers among
  them, and of the eigenvariables of the `∀R` and `∃L` steps below the
  goal: `cut 1 "p(a)" 1` after `∀R` made `a` cuts on the instance of
  `∀a. p(a)`'s body. It refuses a name the sequent lacks or has with
  another arity (`error: the cut formula names r, …`), and an
  eigenvariable of another branch (`error: the eigenvariable a cannot be
  used here: …`).

```console
$ printf 'apply 0 0 forallL\napply 1 1 ax\nproof\n' | deduxit interact -i "forall x. p(x) |- p(f(a))"
opened goal 1: 1: p($1) ⊢ 0: p(f(a))
closed; no goal is open: `proof` checks the proof
valid proof (intuitionistic)
───────────────── ax
p(f(a)) ⊢ p(f(a))
────────────────── ∀L
∀x. p(x) ⊢ p(f(a))
```

### Cuts

`cut G C [P…]` is the one step that does not act on a formula of the
goal. C is written as the mode writes formulas (intuitionistic with
`-i`, as the sequent is). Of a goal `Γ, Δ ⊢ D` it leaves `Γ ⊢ C` (the
formulas at P) and `Δ, C ⊢ D`. `undo` takes it back with its formula.
The search never makes a cut: `close` proves each premise without one.
The finished proof keeps the cut; its JSON is at wire level 6, which
`check` reads.

```console
$ printf 'goals\nrules 0 2\napply 0 2 * 0\nclose\nproof\n' | deduxit interact "A, B |- A * B"
goal 0: ⊢ 0: ~A, 1: ~B, 2: A ⊗ B
⊗ (with a split)
opened goal 1: ⊢ 0: ~A, 1: A
opened goal 2: ⊢ 0: ~B, 1: B
goal 1: provable (MLL, classical, focus engine)
goal 2: provable (MLL, classical, focus engine)
no goal is open: `proof` checks the proof
valid proof (classical)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, ~B, A ⊗ B
$ printf 'apply 0 1 -oL 0\napply 1 0 ax\napply 2 0 ax\nproof --json p.json\n' | deduxit interact -i "A, A -o B |- B"
opened goal 1: 0: A ⊢ 1: A
opened goal 2: 0: B ⊢ 1: B
closed
closed; no goal is open: `proof` checks the proof
valid proof written to p.json
$ deduxit check -i -q p.json
valid proof of A, A ⊸ B ⊢ B (intuitionistic)
$ printf 'cut 0 B 0 1\nclose\n' | deduxit interact -i "A, A -o B, B -o C |- C"
opened goal 1: 0: A, 1: A ⊸ B ⊢ 2: B
opened goal 2: 0: B ⊸ C, 2: B ⊢ 1: C
goal 1: provable (IMLL, intuitionistic, two-sided engine)
goal 2: provable (IMLL, intuitionistic, two-sided engine)
no goal is open: `proof` checks the proof
```

### Eliminating cuts: `eliminate`

`deduxit eliminate FILE` eliminates the cuts of a proof file step by
step. It checks the proof in its file's mode first; it takes `check`'s
flags and formats.

- Each step reduces the first cut with no cut above it. `--at NODE`
  takes the first step on the cut at that index of the file's `nodes`
  instead.
- Each step prints its case (`the axiom case`, `the key case of ⊗ and
  ⅋`, `of 1 and ⊥`, `of & and ⊕`, `of ∀ and ∃`, `a commutative case`,
  `weakening`, `dereliction`, `contraction`, `a box entering a box`) and
  the cut formula.
- Every proof reached is checked, and the last one is drawn.
- A first-order proof is eliminated alike, its cuts on quantified
  formulas or on instances. The key case of ∀ and ∃ puts the ∃ step's
  witness for the ∀ step's eigenvariable throughout the ∀ premise. Where
  another cut still reads that premise, it is copied: the copy's own ∀
  steps get fresh eigenvariables, and the copy counts against
  `--node-limit` before it is made. Each step's line writes the instance
  it cut.

Two bounds keep a call finite, and at either the proof reached is
printed with its cuts. `--steps N` (`cut.steps`) bounds the steps; the
default is `--steps 10000`. `--node-limit N` (`cut.nodes`) bounds the
nodes of a proof a step makes; the default is `--node-limit 100000`.
`--steps 1` takes one step.

The exit status is 0 when no cut is left or the steps asked for with
`--steps` were taken, and 1 for an invalid proof. It is 3 when the
default bound on steps, another bound or the time limit stopped the
elimination with cuts left.

`--format json` writes `{"version", "proof", "steps": [{"cut",
"formula", "case", "above"}], "end"}`, at its proof's wire level (6 with
cuts, 7 at first order). `end` is `cut_free`, or `refused` with a
`reason`.

`--net` eliminates on the proof's net instead, for the sequents `--net`
takes:

- The cuts are cut links. Each step reduces the first cut link: `the
  axiom case`, `the key case of ⊗ and ⅋`, and on boxes `weakening`,
  `dereliction`, `contraction` and `a box entering a box`. There is no
  commutative case.
- Every net reached passes the correctness criterion. The net reached
  is printed as text, a cut link as `cut A[x] — A⊥[y]`.
- `--node-limit` counts a net's vertices.
- `--format svg|png|pdf --output FILE` writes `FILE-k`, the net step k
  was taken on with its cut link and what it acts on highlighted, and
  `FILE`, the net reached. Each drawing is held to `--derivation-limit`.
  The default is `--drawings-limit 256MiB` (`cut.drawn_bytes`) for the
  steps' files together; past it the steps go on undrawn, and standard
  error says from which step.
- `--format json` writes `{"version", "net", "steps": [{"cut": [x, y],
  "case", "above"}], "end"}`.

```console
$ printf 'cut 0 !a 0\nclose\nproof c.json\n' | deduxit interact -i "!a |- a * a" > /dev/null
$ deduxit eliminate -q -i --steps 2 c.json
1. contraction on the cut of !a
2. dereliction on the cut of !a
proof of !a ⊢ a ⊗ a (intuitionistic) with 2 cuts left after 2 steps: the elimination took the most steps allowed, 2; --steps N allows more
$ deduxit eliminate -q -i c.json
1. contraction on the cut of !a
2. dereliction on the cut of !a
3. a commutative case on the cut of a
4. the axiom case on the cut of a
5. a commutative case on the cut of a
6. the axiom case on the cut of a
cut-free proof of !a ⊢ a ⊗ a (intuitionistic) after 6 steps
$ deduxit eliminate -q -i --net c.json
1. contraction on the cut of !a
2. dereliction on the cut of !a
3. the axiom case on the cut of a
4. dereliction on the cut of !a
5. the axiom case on the cut of a
cut-free proof net of !a ⊢ a ⊗ a (intuitionistic) after 5 steps
$ echo '{"version":7,"sequent":{"terms":[{"D":0},{"∃":0},{"V":1},{"V":0},{"∀":3}],"roots":[1,2],"atoms":[{"P":[0,[0]]},{"P":[0,[1]]}],"symbols":[["p",1],["a",0]],"fo":[{"B":0},{"F":[1,[]]}],"binders":["x","x"],"antecedents":1,"cuts":[4]},"nodes":[{"ax":[7,8]},{"∃":[0,2,0]},{"∀":[3,0,1]},{"ax":[9,2]},{"∃":[5,1,3]},{"cut":[3,2,4]}],"instances":{"members":[[1,0],[4,0],[6,1]],"frames":[[null,2],[null,1]],"terms":[{"E":0}]}}' > cut.json
$ deduxit eliminate -q -i cut.json
1. the key case of ∀ and ∃ on the cut of ∀x. p(x)
2. the axiom case on the cut of p(a)
cut-free proof of ∀x. p(x) ⊢ p(a) (intuitionistic) after 2 steps
```

## 11. A sequent alone: `seq`

- `seq print` prints the sequent one-sided, in negation normal form.
  With `-i` it prints it two-sided, as intuitionistic linear logic reads
  it, and under `--logic` it prints the translated sequent.
  `--format text|latex|typst|svg|png|pdf` picks the format.
- `seq json` writes the sequent's JSON (section 8), with `-i` the
  intuitionistic form. `--optimize` merges equal subformulas of a JSON
  input.
- `seq fragment` names the sequent's fragment.

```console
$ deduxit seq print -i "A * B -o C |- A -o B -o C"
(A ⊗ B) ⊸ C ⊢ A ⊸ (B ⊸ C)
$ deduxit seq print --format typst "A * B -o C |- A -o B -o C"
$⊢ (A ⊗ B) ⊗ C^⊥, A^⊥ ⅋ (B^⊥ ⅋ C)$
```

## 12. Many sequents in one call

`prove` decides a batch when it is given any of these:

- `--file` more than once;
- a directory, walked in sorted order for `.p` and `.json` files;
- `--files-from LIST`: one path per line (`--null` for NUL-separated
  paths, `-` for standard input);
- an input format of many sequents, below.

The input formats of many sequents:

- `--input-format lines`: `NAME: SEQUENT` or `SEQUENT` per line; blank
  lines and everything from `#` on are skipped;
- `jsonl`: a sequent as `seq json` writes it, or `{"name", "mode",
  "sequent"}` with the sequent as JSON or text, per line;
- `problems`: `NAME; MODE; EXPECTED; COPIES; SEQUENT` per line.

Every sequent gets one line, `NAME: VERDICT`, in input order, as soon as
it is decided. Its name is its path, its line's name, or `FILE:LINE`.

- With `--format json` each line is a JSON record: `version` and `name`
  first, then the outcome's keys, or `"verdict": "error"` with a
  `message`.
- A drawn format writes a file per proved sequent into `--output DIR`.
  A Rocq certificate's file is named by a Rocq identifier: `modus
  ponens` becomes `modus_ponens.v`.
- The exit status is the worst of the entries (section 1).
- On standard input the batch is a stream, answered line by line.

```console
$ printf 'identity: A |- A\nmp: A, A -o B |- B\nA |- B\n' > sequents.txt
$ deduxit prove --input-format lines --file sequents.txt
identity: provable (MLL, classical, focus engine)
mp: provable (MLL, classical, focus engine)
sequents.txt:3: unprovable (MLL, classical, focus engine): A on the left has no A on the right to meet in an axiom
$ echo '{"name": "pair", "mode": "intuitionistic", "sequent": "A, B |- A * B"}' | deduxit prove --input-format jsonl --format json
{"version":1,"name":"pair","deduxit":"…","verdict":"proved","checked":true,"fragment":"IMLL","mode":"intuitionistic","engine":"two-sided","statistics":{"nodes":1,"memo_hits":0,"memo_entries":1,"splits":1,"links":0,"tests":0,"copies":0,"forward_copies":0,"work":16},"sequent":{"terms":[{"D":0},{"D":1},{"V":0},{"V":1},{"⊗":[2,3]}],"roots":[0,1,4],"atoms":["A","B"],"antecedents":2},"nodes":[{"ax":[3,0]},{"ax":[4,1]},{"⊗":[2,0,1]}]}
```

A record or a problem names its mode as one of `classical`, `mix`,
`affine`, `affine-mix`, `intuitionistic`, `intuitionistic-affine`,
`cyclic`, `lambek` or `lambek-star`.

### Running a batch

- `--timeout` is each sequent's limit, and `--batch-timeout` the whole
  batch's.
- `--workers N` decides N sequents at once, and `--batch-memory SIZE`
  bounds the memory of all searches together.
- `--isolate` decides each sequent in a child process of its own.
- `--cores across` gives each sequent one thread; `--cores within` gives
  each sequent every thread, one sequent at a time. The default is
  `--cores auto`: across when the batch has at least as many sequents as
  workers, else within, and within on a stream from standard input.

What a batch across the cores leaves unknown is worth a second pass
with `--cores within` or a longer `--timeout`.

## 13. Ordinary logic

`--logic classical`, `intuitionistic` or `minimal` reads a formula or a
sequent of propositional logic. The connectives are `->`/`→`, `/\`/`∧`,
`\/`/`∨`, `~`/`¬`, `<->`/`↔`, and the constants `true`/`⊤` and
`false`/`⊥`. `~` binds tightest, then `/\`, `\/`, `->`, and last `<->`;
`->` and `<->` group to the right. A `.p` file is a TPTP problem.

The verdict is `valid`, `not valid` or `unknown`. The proof is drawn as
a derivation of LK or LJ, which the checker of that calculus passes
before it is shown.

### How the sequent is decided: `--method`

`--method` (`ordinary.method`) chooses how a sequent is decided.

`ljt` is Dyckhoff's calculus LJT, the default for intuitionistic and
minimal logic:

- Its search always ends, so it answers `unknown` only at the time limit
  or at a bound.
- Where another bound stops it (a derivation past `--derivation-limit`,
  the memory), the default hands the sequent to the embedding, and the
  verdict line names the translation.
- Named as `--method ljt`, it reports that bound instead (exit status 2
  for a derivation it could not check). It refuses classical logic (exit
  status 2).
- `--stats` prints `sequents expanded`, `memo hits`, `memo entries` and
  `refuted by an earlier failure`.
- Its derivations use LJ's cut, so their JSON is at wire level 9, their
  lambda-terms may hold redexes and their certificates `let`s.

`embedding` is the default for classical logic. It searches the
sequent's image in linear logic (the translated sequent) and reads the
proof back.

- Classical logic goes into affine MALL, with no copy bound.
- Intuitionistic and minimal logic go into ILL by `--translation cbn`
  (the default), `cbv` or `01`, under the time limit, so they may be
  answered unknown.
- `--translation` or `--linear` (show the linear proof of the image)
  alone chooses the embedding.
- `seq print --logic` prints the image.

### JSON

Under LJT, `--format json` writes the outcome of the ordinary sequent:
`{"version": 8, "deduxit": …, "verdict": "valid"|"not_valid"|"unknown",
"reason": …, "logic": …, "engine": "ljt", "statistics": {"sequents",
"memo_hits", "memo_entries", "subsumed"}, "derivation": …}`. `check`
refuses it (exit status 2); in the library, `ordinary::Derivation::check`
checks its derivation. Through the embedding, `--format json` writes the
image's outcome, as without `--logic`, and `check` reads it.

### Examples

```console
$ deduxit prove --logic intuitionistic 'a \/ b -> b \/ a'
valid (intuitionistic logic by LJT)
  ───── ax        ───── ax
  a ⊢ a           b ⊢ b
───────── ∨R₂   ───────── ∨R₁
a ⊢ b ∨ a       b ⊢ b ∨ a
───────────────────────── ∨L
      a ∨ b ⊢ b ∨ a
   ─────────────────── →R
   ⊢ (a ∨ b) → (b ∨ a)
$ deduxit prove --logic classical -q '((a -> b) -> a) -> a'
valid (classical logic by the affine translation into affine MALL, focus engine)
$ deduxit seq print --logic intuitionistic --translation cbv 'a, a -> b |- b'
!a, !(!a ⊸ !b) ⊢ !b
$ deduxit prove --logic minimal --method embedding -q 'a, a -> b |- b'
valid (minimal logic by the cbn translation into ILL, two-sided engine)
```

`-i`, `-a` and `--mix` are refused beside `--logic`: the translation
decides the mode.

The checker checks a derivation of LK or LJ with every weakening and
contraction. The drawing leaves each one out where it is admissible
(G3's axiom with a context). `--structural explicit` (`view.structural`)
draws every one, and `--format rocq` always names them.

### Step by step in LK or LJ

`deduxit interact --logic LOGIC SEQUENT` proves an ordinary sequent step
by step by the rules of LK or LJ. It has the linear session's commands
(`goals`, `rules`, `apply`, `undo`, `close`, `show`, `proof`, `lambda`,
`save`, `load`, `help`, `quit`), but no `cut` and no `invertible`.

- A formula is named by its side and position: `L0`, `L1`, … left of
  `⊢`, and `R0`, … right of it, as `goals` shows them.
- A rule is its name or an ASCII spelling: `ax`, `WL`, `WR`, `CL`, `CR`,
  `andL`, `andL1`, `andL2`, `andR`, `orL`, `orR`, `orR1`, `orR2`, `impL`
  or `->L`, `impR` or `->R`, `notL` or `~L`, `notR` or `~R`, `iffL`,
  `iffL1`, `iffL2`, `iffR`, `falseL`, `falseR`, `trueL`, `trueR`.
- The axiom closes a goal that has its formula on both sides, whatever
  else it holds.
- A rule with two premises shares the whole context, unless formulas
  named after the rule go to the first premise (`apply G L1 ->L L0`).
- In intuitionistic and minimal logic every goal keeps exactly one
  formula right of `⊢`: `Γ ⊢` starts as `Γ ⊢ ⊥`, and `WR`, `CR`, `⊥R`
  and `∨R` are refused. Minimal logic has no `⊥L`.
- `close [G]` decides a goal through the embedding, whose derivations
  have no cut, and grafts the derivation. A goal it cannot decide stays
  open, with the reason.
- `proof` writes the checked derivation in any format of `prove`.
  `save FILE` writes the session, and `--state FILE` resumes it.
- The session exits with 0 for a finished derivation, and 1 otherwise.

```console
$ printf 'apply 0 R0 ->R\napply 1 R0 andR\nclose\nproof\n' | deduxit interact --logic intuitionistic '|- a /\ b -> b /\ a'
opened goal 1: L0: a ∧ b ⊢ R0: b ∧ a
opened goal 2: L0: a ∧ b ⊢ R0: b
opened goal 3: L0: a ∧ b ⊢ R0: a
goal 2: valid (intuitionistic logic by the cbn translation into ILL, two-sided engine)
goal 3: valid (intuitionistic logic by the cbn translation into ILL, two-sided engine)
no goal is open: `proof` checks the proof
valid proof in LJ (intuitionistic logic)
  ───── ax        ───── ax
  b ⊢ b           a ⊢ a
───────── ∧L₂   ───────── ∧L₁
a ∧ b ⊢ b       a ∧ b ⊢ a
───────────────────────── ∧R
      a ∧ b ⊢ b ∧ a
   ─────────────────── →R
   ⊢ (a ∧ b) → (b ∧ a)
```

## 14. Cyclic linear logic and the Lambek calculus

`--cyclic` decides unit-free MLL whose formulas keep their order, up to
rotation. `~(A * B)` is `~B par ~A`, and the formulas left of `|-` come
in the reverse order, each negated. So `A, A -o B |- B` holds and
`A -o B, A |- B` does not.

`--lambek` decides the Lambek calculus: intuitionistic, in the order
written, with `A \ B` and `B / A` beside `*`. Every sequent of a proof
has a hypothesis, unless `--empty-antecedents` is given (the calculus
L*).

Both modes refuse units, additives, exponentials, Mix, weakening and
Rocq certificates.

```console
$ deduxit prove --cyclic "a * b |- b * a"
unprovable (MLL, cyclic, net engine): the search was exhaustive
$ deduxit prove --lambek -q "c / b, a, a \ b |- c"
provable (IMLL, Lambek, essential engine)
```

## 15. Settings files and styles

`--settings FILE` reads the settings from one JSON object. The flags set
the same settings, and each flag's help names its key in parentheses:
`--copies` is `search.copies`, `--timeout` is `clock.time_limit_ms`, and
`--memory-limit` is `limits.memory_bytes`.

- The object's keys are any of `clock`, `limits`, `search`, `view`,
  `styles`, `batch`, `ordinary`, `interactive`, `input`, `cut` and
  `lambda`.
- A key left out keeps the command's default, and a misspelt key is an
  error.
- A flag given beside the file overrides the file.

```console
$ echo '{"search": {"copies": 0, "engine": "focus"}}' > settings.json
$ deduxit prove --settings settings.json --copies 3 "|- ?(~a * ~a), a * a * a * a"
unprovable (MELL, classical, focus engine): the search was exhaustive
```

### Styles

`--style KEY=VALUE` sets one option of an output format, and may be
given more than once.

- KEY names an option of `--format`'s format. Another format's option
  takes its prefix: `text.`, `latex.`, `typst.`, `svg.`, `png.`, `pdf.`
  or `rocq.`.
- A dotted key reaches into an option: `labels.table.⊸L=⊸_L` relabels
  one rule and leaves the others upright.
- `--style-file FILE` reads every option as JSON, an object per format.
- `prove --help` lists every option and the rules' names.

```console
$ deduxit prove --style gap=6 --style bar="=" "A * B |- A * B"
provable (MLL, classical, focus engine)
======= ax      ======= ax
⊢ ~A, A         ⊢ ~B, B
======================= ⊗
    ⊢ ~A, ~B, A ⊗ B
    ================ ⅋
    ⊢ ~A ⅋ ~B, A ⊗ B
```

## 16. The Rust library

The crate `deduxit` on crates.io holds all of it. Its default features
are `parse`, `serialize`, `interactive`, `latex`, `typst`, `svg`, `rocq`
and `sat`; `parallel`, `png` and `pdf` are off. Its documentation is at
<https://deduxit.github.io/deduxit/deduxit/> for `main`, and on docs.rs
for the releases:

- the text syntax on `Sequent`;
- every JSON form on its type: `Sequent`, `Proof`, `Outcome`,
  `ProofStructure` and `Interactive`, and ordinary logic's
  `ordinary::Sequent`, `ordinary::Derivation`, `ordinary::Outcome` and
  `ordinary::Interactive` at wire level 8 (level 9 where a derivation
  holds LJ's `cut`);
- each engine on `Engine`.

### Deciding a sequent

The shortest program parses a sequent, proves it, draws the derivation
and reads the proof back from JSON:

```rust
use deduxit::linear::{Fragment, Mode, Options, Sequent, Verdict, prove};

// Two-sided input becomes one-sided: ⊢ ~A, A ⊗ ~B, B.
let sequent: Sequent = "A, A -o B |- B".parse()?;
assert_eq!(sequent.fragment(), Fragment::MLL);

let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
let Verdict::Proved(proof) = &outcome.verdict else {
    panic!("the sequent is provable");
};
assert_eq!(
    proof.derivation()?.to_string(),
    "─────── ax   ─────── ax\n\
     ⊢ ~A, A      ⊢ ~B, B\n\
     ──────────────────── ⊗\n\
    \x20 ⊢ ~A, A ⊗ ~B, B"
);

let json = serde_json::to_string(&proof)?;
let back: deduxit::linear::Proof = serde_json::from_str(&json)?;
back.check(Mode::CLASSICAL)?;
```

`Options::default()` bounds the copies of a `?` formula at 3: a library
call without a stop must end. The command has a time limit instead, and
deepens the copy bound until the search decides or the time is up.

### Time limits and the command's settings

The crate reads no clock. A long call takes `Limits` and a stop closure
that the caller writes.

- `prove_within(&sequent, mode, &options, &limits, stop)` searches
  within `limits` until `stop` returns true. The stop is told the work
  done at each poll, so a caller without a clock can bound the work.
- `Settings::default()` is the command's settings as data: its search
  options (no copy bound), its limits and its clock.
- `settings.stop(|| start.elapsed())` (`Settings::stop`) is the
  command's stop. It is given the time since the caller started, and
  fires at `clock.time_limit_ms` in every phase but the classical
  refutation after the search, which runs on within its own work bound.
- To decide as `deduxit prove --deterministic` does, call `prove_within`
  with `settings.search` on one thread
  (`.with_jobs(1).with_schedule(Schedule::Turns)`, `Schedule` from
  `deduxit::linear::search`), `settings.limits` and that stop.

### Errors

Every error is `deduxit::Error`. Its `code()` is the stable reason to
branch on. In an intuitionistic mode, a sequent with no intuitionistic
reading is `Error::NotIntuitionistic`; its message names the subformula
that has no reading. An error made without the forest names
occurrences by id, and `describe(&forest)` names them by formula.

### JSON, printing and exports

- `serde_json::from_str::<Outcome>(&json)` reads back the outcome that
  `prove --format json` writes, with its proof or disproof. An engine
  name that a later version writes reads as `Engine::Unknown`.
- A `lambda::Term` prints with `Display`, in the default notation and
  within the default limits, so `term.to_string()` is its text.
- `export::latex::to_string(&derivation, &options)` returns a derivation
  as LaTeX in one string, with no stop to write: the text is linear in
  the derivation, which was made within its limits. `typst::to_string`,
  `svg::to_string` and `rocq::to_string` do the same for their formats.
- Each export's `write` takes any `fmt::Write` and a stop, for a
  derivation written under a time limit.

### Cut elimination

- `proofs::cut::step(&proof, mode, at, &options, &limits, stop)` reduces
  one cut: the node `at`, or with `None` the one the strategy picks. It
  returns the checked proof and the `Step` (`cut`, `formula`, `case`).
- `proofs::cut::eliminate(&proof, mode, at, &options, &limits, stop,
  each)` runs to the end within `cut::Options`, its first step on the
  node `at` where one is given. `each` sees every step and the proof it
  was taken on. It returns the proof reached, the steps and the `End`.
- At first order, a `Step`'s `formula` is a member of the proof the step
  was taken on; that proof's `Proof::formula` writes it as the instance
  it is.
- On nets, `nets::cut::step(&net, at, &options, &limits, stop)` reduces
  one cut link of a `ProofStructure` or a `MellStructure` (`at` a vertex
  of a cut link), and `nets::cut::eliminate(&net, &options, &limits,
  stop, each)` runs to the end; `each` sees every step and the net it
  was taken on.
- `svg::net_step` and `svg::mell_net_step` draw a step; the cut link's
  id is `c<x>-<y>`.

### Ordinary logic

- `ordinary::decide(&sequent, &options, &search, &limits, stop)` decides
  by `options.method()`. `ordinary::Options::method` is `None` by
  default, which means LJT (`ordinary::Method::Ljt`) for intuitionistic
  and minimal logic, and the embedding where a translation is given or
  where LJT gives way (`ordinary::gives_way`).
- The `ordinary::Outcome` says which method ran, in `search`:
  `ordinary::Search::Embedding` with the image's outcome, or
  `ordinary::Search::Ljt` with LJT's statistics.
- `ordinary::ljt::decide(&sequent, logic, &limits, stop)` is LJT alone.
  Its search always ends: it has no copy bound, so it answers `unknown`
  only at a bound or the stop. Its derivation is LJ's with cuts, checked
  like any other.
- `ordinary::Interactive` proves an LK or LJ sequent step by step:
  `rules`, `apply`, `undo`, `close` and `hint`, the last two through
  `ordinary::decide`. Its finished derivation passes
  `Derivation::check`.
- `ordinary::Rule` reads from its name or an ASCII spelling
  (`"impL".parse::<ordinary::Rule>()`), as the command's `apply` does.
- `ViewOptions::structural` draws a derivation of LK or LJ with its
  weakenings and contractions `explicit` or `admissible`.
- `Derivation::term` reads the lambda-term of a checked derivation, or
  of a view of one.

## 17. The web bindings

An agent with a shell uses the command instead; this section is for a
program in a browser.

The page at <https://deduxit.github.io/> is the prover in a browser, for
people: a sequent proved step by step with the mouse. Its engine is the
unpublished crate `deduxit-web` (`web/` in the repository).
`nix build github:deduxit/deduxit#deduxit-web` builds the WebAssembly
module and its JavaScript.

- Every call takes JSON text and answers JSON, SVG or bytes. No call
  keeps anything for the next.
- Every call but `about` and `defaults` takes the page's settings: the
  JSON of the library's `Settings`, laid over a browser tab's defaults
  (which `defaults` returns).
- A failed call throws the error's JSON (section 8).

The calls of linear logic:

- reading: `about`, `defaults`, `read`, `read_as`;
- a session: `start`, `view`, `rules`, `split_passes`, `apply`, `cut`,
  `invert_all`, `undo`, `close`, `fill`, `close_all`, `hint`, `proof`,
  `term` (the lambda-term);
- one call: `prove`, `draw`, `draw_net`, `export`, `render`,
  `certificate`;
- cut elimination step by step and to the end, on terms and on nets:
  `cuts`, `cut_step`, `eliminate`, `net_cuts`, `net_cut_step`,
  `net_eliminate`.

A sequent with quantifiers is typed as for the command. A step's JSON
is `{"position", "rule", "left", "witness"}`: `witness` is the witness
of `∃R` or `∀L` as text, and left out it stays open (`$1`). `rules`
marks such a rule `"witness": true`. `hint` on a goal of instances
closes it in a copy of the session. The first-order bounds are the
settings `search.witness_depth`, `search.instances` and `limits.terms`.

An ordinary tab (LK and LJ) has its own calls, its session the
library's `ordinary::Interactive` at wire level 8:

- `ordinary_start(sequent, logic, settings)`, `ordinary_view`,
  `ordinary_undo`, `ordinary_close`, `ordinary_draw`, `ordinary_term`;
- `ordinary_rules(session, goal, side, position, settings)`;
- `ordinary_apply`, which takes a step `{"side": "left"|"right",
  "position", "rule": "→L", "split": {"left": [..], "right": [..]}}`;
  with `split` left out, the premises share the context;
- `ordinary_hint`, whose `steps` are the weakenings and contractions at
  the root of the derivation found and the rule above them, which
  `ordinary_apply` takes in turn;
- `ordinary_export`, in `text`, `latex`, `typst`, `svg` or `rocq`.

`about` lists the suites (`linear`, `ordinary`), each with the prefix of
its calls, the parameter its start takes (`mode`, `logic`), every value
and the default.

## 18. Rocq certificates and the Rocq library

`--format rocq` writes a certificate: a Rocq file that states the
sequent's derivability, or its non-derivability, and proves it.
`--standalone` adds the imports, and `--lemma NAME` names the lemma.

### Checking a certificate

With Nix (and flakes on), one command checks a certificate written with
`--standalone`: `nix run github:deduxit/deduxit#check-certificate --
proof.v`. It compiles the file against the kernel its import lines
name, at the versions the flake pins, then asks Rocq which axioms each
lemma rests on. It exits with:

- 0 when the certificate holds;
- 1 when Rocq refuses the file, or a lemma rests on an axiom (a
  classical certificate over `Prop` may use the excluded middle);
- 2 when the file cannot be read.

Without Nix, compile the certificate with the kernel on the load path:
`rocq compile -R path/to/nanoyalla NanoYalla proof.v`, or for the kernel
`deduxit` `rocq compile -Q path/to/rocq-deduxit/theories Deduxit proof.v`.
Rocq prints nothing for a certificate it accepts. `rocq-deduxit` is not
yet in opam's archive: build it from its repository, or pin it with opam
at the tag of deduxit's own version (`v0.3.0` for deduxit 0.3.0).

### Kernels

`--kernel` names the kernel a certificate is written for.

- `nanoyalla` is the default for classical linear logic: NanoYalla, the
  kernel of Click & coLLecT, at the version `prove --help` names. Its
  certificate is a proof script checked rule by rule. It refuses Mix and
  affine weakening; a refused certificate leaves the verdict and its
  exit status, with the reason on standard error.
- `deduxit` is the default for Mix, affine and intuitionistic logic and
  for a sequent with a quantifier: the Rocq library `rocq-deduxit`, in
  its own repository (`github.com/deduxit/rocq-deduxit`). Its
  certificate is a proof term that the library's checker accepts.

A first-order proof is stated in the library's first-order development
(`ll (symbol := string) Calculus.classical`,
`ill (symbol := string) Intuitionistic.intuitionistic`, …). Predicates
apply to terms, `bound 0` is the innermost quantifier's variable and
`eigen 0` an eigenvariable. A `∀` step names its body and its
eigenvariable, and a `∃` step its witness.

### Unprovable sequents

An unprovable sequent gets a certificate that the calculus does not
derive it, for every refutation kind but `exhausted`, `saturated` and,
with a quantifier, `equation`. The library's weighted count
(`weighted_refutation`, `affine_refutation`: weights per atom, the
atoms that no firing reaches declared dead) proves `one_sided`, the
`_symbol` kinds and `state_equation`. The other kinds have theorems of
their own.

```console
$ deduxit prove --format rocq "A * B |- B * A"
(* provable (MLL, classical, focus engine) *)
Lemma certificate (A B : formula) : ll [parr (dual A) (dual B); tens B A].
Proof.
apply (parr_r_ext []); cbn_sequent.
apply (ex_perm_r [2; 0; 1] [dual B; tens B A; dual A]).
apply (tens_r_ext [dual B]); cbn_sequent.
{
  ax_expansion.
}
{
  ax_expansion.
}
Qed.
$ deduxit prove --format rocq "forall x. p(x) |- exists y. p(y)"
(* provable (MLL1, classical, focus engine) *)
Lemma certificate : ll (symbol := string) Calculus.classical [fex (covar "p" [bound 0]); fex (var "p" [bound 0])].
Proof.
  apply (soundness_classical String.eqb String.eqb_spec
    (Exists (covar "p" [bound 0]) (eigen 0) (Exists (var "p" [bound 0]) (eigen 0) (Ax "p" [eigen 0])))).
  vm_compute. reflexivity.
Qed.
$ deduxit prove -i --format rocq "!d, !(c * d * d -o 1) |- c * c"
(* unprovable (IMELL, intuitionistic, horn engine): c occurs 2 more times on the right than on the left outside the exponentials, and every occurrence of it below one, which copies add, is on the right, so they cannot all meet in axioms (an implication's antecedent counts on the other side) *)
Lemma certificate : ~ ill (atom := string) intuitionistic [ioc (ivar "d"); ioc (ilmap (itens (itens (ivar "c") (ivar "d")) (ivar "d")) ione)] (itens (ivar "c") (ivar "c")).
Proof.
  apply (iweighted_refutation (weight_of String.eqb [("c", 1%Z)])
    (fun a => existsb (String.eqb a) []) true _).
  vm_compute. reflexivity.
Qed.
$ deduxit prove --format rocq "A & B |- A * B"
no certificate: an exhausted search states no invariant, so only the search rules the proofs out and Rocq has nothing to check
(* unprovable (MALL, classical, focus engine): the search was exhaustive *)
```

## 19. Mistakes to avoid

Each mistake, its fix, and the hint the command prints where it prints
one.

### A sequent without `prove`

`deduxit "A |- A"` is no command. Write `deduxit prove "A |- A"`; the
error suggests it.

```console
$ deduxit "A |- A"; echo "exit $?"
error: unrecognized subcommand 'A |- A'

  tip: to decide a sequent, run deduxit prove "A |- A"

Usage: deduxit <COMMAND>

For more information, try '--help'.
exit 2
```

### An unquoted sequent

`*`, `|`, `&`, `!` and `?` mean something to the shell, and an unquoted
`-o` reads as `--output`. Quote the sequent; the error shows it quoted.

```console
$ deduxit prove A, A -o B ⊢ B; echo "exit $?"
error: unexpected argument 'A' found

  tip: quote the sequent: deduxit prove "A, A -o B ⊢ B"

Usage: deduxit prove [OPTIONS] [SEQUENT]

For more information, try '--help'.
exit 2
```

### `->` in linear logic

`->` is ordinary logic's arrow. Write `-o` for the linear implication,
or add `--logic classical` (or another logic) for ordinary logic; the
error says both.

```console
$ deduxit prove "A, A -> B |- B"
error: cannot parse the sequent
  A, A -> B |- B
        ^ unexpected ">", expected the `o` of `-o`
  -> is ordinary logic's arrow: write -o, or add --logic classical
```

### A file of `NAME: SEQUENT` lines without `--input-format lines`

Without the flag the file is one sequent, and the parse fails at the
first `:`. Add `--input-format lines`; the error suggests it.

```console
$ printf 'identity: A |- A\nmp: A, A -o B |- B\n' > sequents.txt
$ deduxit prove --file sequents.txt
error: cannot parse the sequent
  identity: A |- A
          ^ unexpected ":" at line 1, character 9, expected a connective, `,` or `|-`
  to read one sequent per line, add --input-format lines
```

### Reading unknown as unprovable

Unknown (exit status 3) means a limit ended the search. Raise the limit
the line names, or name a bound (`--copies N`, `--work-limit N`) to get
a definite answer within it.

### The classical syntax with `-i`

Intuitionistic linear logic has no negation: `~`, `par`, `|`, `?` and
`bot` are refused, and so is a second formula right of `|-`. Put the
hypotheses left of `|-` and write `A -o B`; the error says what to write
instead (section 3). `--ill-syntax one-sided` reads the classical syntax
as the one-sided sequent it stands for.

### A mode flag on `check`

`check` takes the mode its file states, so a proof of `prove -i` needs
no flag. A mode flag that names another mode is an error, exit status 2,
and the error says to pass the flags the proof was found with, or none.

### Reading a one-sided derivation as two-sided

A classical derivation is one-sided unless `--classical-sides two` is
given: `⊢ ~A, A` is the axiom `A ⊢ A`.

### An unused hypothesis

Linear logic uses every hypothesis exactly once, so `A, B |- A` is
unprovable. Where an unused hypothesis is the problem, the verdict line
names `-a`, which lets a hypothesis go unused (section 5).
