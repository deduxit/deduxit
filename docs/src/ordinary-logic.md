# Decide ordinary logic

deduxit also decides ordinary propositional logic, in three variants:
- classical logic, where every formula is true or false, so that
  `a \/ ~a` holds;
- intuitionistic logic, where a proof of a formula must build it, so
  that `a \/ ~a` and `~~a -> a` do not hold in general;
- minimal logic, intuitionistic logic in which false is an ordinary
  atom: nothing follows from it.

Name the logic with `--logic` and give `prove` a formula or a sequent:

```console
$ deduxit prove --logic classical '((a -> b) -> a) -> a'
valid (classical logic by the affine translation into affine MALL, focus engine)
 ──────── ax
 a ⊢ b, a
────────── →R   ───── ax
⊢ a → b, a      a ⊢ a
───────────────────── →L
   (a → b) → a ⊢ a
 ─────────────────── →R
 ⊢ ((a → b) → a) → a
$ deduxit prove --logic intuitionistic 'a \/ b -> b \/ a'
valid (intuitionistic logic by the SAT solver)
  ───── ax        ───── ax
  a ⊢ a           b ⊢ b
───────── ∨R₂   ───────── ∨R₁
a ⊢ b ∨ a       b ⊢ b ∨ a
───────────────────────── ∨L
      a ∨ b ⊢ b ∨ a
   ─────────────────── →R
   ⊢ (a ∨ b) → (b ∨ a)
$ deduxit prove --logic intuitionistic '~~a -> a'
not valid (intuitionistic logic by the SAT solver): a checked countermodel refutes it
world 0 (the model's world): forces nothing; below 1, 2
world 1: forces a; below 3
world 2: forces nothing; below 3
world 3: forces a
```

## Read the answer

- `valid`: the sequent is a theorem of the logic. deduxit shows its proof
  in Gentzen's sequent calculus, LK for classical logic and LJ for the
  other two, and checks it by the rules of the logic before it is shown.
  The exit status is 0.
- `not valid`: no proof exists in LK or LJ, so the sequent is not a
  theorem of that logic. `~~a -> a` above is a classical theorem, but not
  an intuitionistic one. The exit status is 1. The lines after it are a
  countermodel ([A countermodel](#a-countermodel)).
- `unknown`: a limit, such as the time limit, stopped the search before
  it decided. The exit status is 3, and the line says which limit it
  was.

The words in brackets name the logic and how deduxit decided the
sequent: by LJT, or through a translation into linear logic and the
engine that searched there ([How a sequent is
decided](#how-a-sequent-is-decided)).

The line of a classical sequent that is not valid may name an
assignment of truth values that makes it false. Here `a -> b` fails
where `a` is true and `b` false:

```console
$ deduxit prove --logic classical 'a -> b'; echo "exit $?"
not valid (classical logic by the affine translation into affine MALL, focus engine): the translated sequent is unprovable: the classical reading is false where a is true and every other atom false (! and ? erased, ⊗ and & as and, ⅋ and ⊕ as or, 1 and ⊤ true, ⊥ and 0 false), and every provable sequent reads as a tautology
exit 1
```

## Write a formula

The connectives are `->`/`→`, `/\`/`∧`, `\/`/`∨`, `~`/`¬`, `<->`/`↔`,
`true`/`⊤` and `false`/`⊥`. `~` binds tightest, then `/\`, `\/`, `->`,
and last `<->`. `->` and `<->` group to the right. A sequent such as
`a, a -> b |- b` has hypotheses left of `|-`; a formula alone is the
sequent with nothing on the left.

## How a sequent is decided

`--method` chooses how (the settings key `ordinary.method`):
- `sat` is a SAT solver, for all three logics, and the default for
  each. Its search always ends. A sequent
  that is not valid comes with a countermodel, which is checked: a
  Kripke model in intuitionistic and minimal logic, an assignment of the
  atoms in classical logic. A valid sequent comes with a derivation of
  LJ in intuitionistic and minimal logic. By default that derivation is
  LJT's, which is mostly smaller, where LJT finds it within 16 384
  sequents and its drawing fits `--derivation-limit`, and otherwise the
  one read off the solver's search; `--method sat` always reads it off
  the search.
  In classical logic a valid sequent comes with the solver's clausal
  proof: every step follows from the clauses of the sequent's negation
  and the steps before it by unit propagation, and the last step is the
  empty clause. A checker of deduxit's own accepts it before the verdict
  is given. A clausal proof is no derivation, so nothing is drawn.
- `embedding` translates the sequent into linear logic, searches the
  image there and reads the proof back as one of LK or LJ. It decides
  all three logics, and gives the derivation of LK, the refutation and
  the certificates that the SAT solver does not give.
- `ljt` is Dyckhoff's calculus LJT, for intuitionistic and minimal logic.
  Its search always ends too.
- `table` and `tableau`, for classical logic, decide the sequent by its
  truth table or by a tableau, as a first logic course does, and print
  the table or the tableau ([Truth tables and tableaux](#truth-tables-and-tableaux)).
- `connection` is a connection prover for classical and intuitionistic
  logic, first-order
  sequents included, and the default for a first-order classical
  sequent ([First-order sequents](#first-order-sequents)).
- `grounding` decides the first-order sequents of two decidable classes
  in every logic, and is the default for them ([Decidable
  classes](#decidable-classes)).
- `models` searches for a finite structure that refutes a first-order
  sequent, in every logic ([Finite models](#finite-models)).

```console
$ deduxit prove --logic classical --method sat -q '((a -> b) -> a) -> a'
valid (classical logic by the SAT solver): a checked clausal proof of 1 step against the 6 clauses of the sequent's negation
$ deduxit prove --logic intuitionistic --method sat '((a -> b) -> a) -> a'; echo "exit $?"
not valid (intuitionistic logic by the SAT solver): a checked countermodel refutes it
world 0 (the model's world): forces nothing; below 1, 2
world 1: forces a; below 3
world 2: forces nothing; below 3
world 3: forces a
exit 1
```

On the ILTP library's 274 problems, with 2 seconds each, the SAT solver
decides 252 in intuitionistic logic and 253 in minimal logic, LJT 200 and
the embedding 188 in each. In classical logic the SAT solver decides 254
and the embedding 157. The SAT solver decides every problem the others
decide. The classical default decides the same 254, and gives the
embedding's output for the 124 the embedding answers with its evidence
within 2 seconds: 123 derivations of LK and one refutation. The 186 problems all three decide in intuitionistic logic take
it 26 ms in all, LJT 313 ms and the embedding 11.6 s ([the
measurement](results/iltp-2026-11-14.md)).

In classical logic the default keeps the embedding's output where the
embedding gives it quickly. Once the SAT solver has decided, the
embedding searches for at most 2²⁰ units of work, on an image of at
most 2²⁰ occurrences, which costs a median of 16 ms on ILTP and at most
0.1 s. Where it
proves the sequent, you get its verdict line, its derivation of LK and
its certificate, as `--method embedding` gives them; where it refutes
the sequent, its refutation. Otherwise the solver's clausal proof or
countermodel stands. A larger `--work-limit` or `--occurrence-limit`
does not raise these bounds, and a smaller one lowers them;
`--method embedding` gives the embedding the whole time limit.

Where the SAT solver stops at a bound other than the time limit, such
as `--work-limit` or `--memory-limit`, the default hands the sequent to
the embedding. The verdict line names the translation, then says which
bound stopped the SAT solver, so you know which one to raise:

```console
$ deduxit prove --logic intuitionistic --work-limit 6 -q 'a -> b, b -> c |- a -> c'
unknown (intuitionistic logic by the cbn translation into ILL, two-sided engine): the work limit of 6 was reached after … at a copy bound of 0; raise it with --work-limit N; the SAT solver gave no verdict: the work limit of 6 was reached, and the embedding took over
```

`--derivation-limit` bounds only what is drawn: a valid sequent whose
derivation is too large to draw keeps its verdict, and `--format json`
writes the derivation.

`--method sat` reports the bound instead. `--translation` and `--linear`
name the embedding, so either of them alone chooses it. Through the
embedding, a `not valid` says that the translated sequent is
unprovable:

```console
$ deduxit prove --logic intuitionistic --method embedding -q '~~a -> a'
not valid (intuitionistic logic by the cbn translation into ILL, two-sided engine): the translated sequent is unprovable: the search was exhaustive
$ deduxit prove --logic minimal --stats -q 'a, a -> b |- b'
valid (minimal logic by the SAT solver)
solver calls: 1
conflicts: 0
lemmas: 0
worlds: 0
facts derived: 3
clauses: 0
proof steps: 0
derivation by LJT: 2 sequents expanded
time: …
```

LJT's derivation of LJ uses Gentzen's cut for four of its rules, so its
lambda-term ([Read a proof as a program](terms.md)) may hold a redex and
its certificate a `let`.

### The translations

Through the embedding, classical logic goes into affine MALL without
exponentials (negation normal form, `∧` as `&`, `∨` as `⅋`), where the
search needs no copy bound. Intuitionistic logic goes into
intuitionistic linear logic (ILL) by one of three translations, which
`--translation` names:
- `cbn`, Girard's call-by-name translation, the default;
- `cbv`, his call-by-value one;
- `01`, Liang and Miller's 0/1 translation.

Minimal logic is the same with false an atom. In ILL the search deepens
its copy bound under the time limit, so an intuitionistic formula may be
answered unknown (exit status 3). `seq print` shows the image, and
`--linear` the linear proof of it:

```console
$ deduxit seq print --logic intuitionistic --translation cbv 'a, a -> b |- b'
!a, !(!a ⊸ !b) ⊢ !b
$ deduxit seq print --logic minimal '~a |- a -> b'
!(!a ⊸ false) ⊢ !a ⊸ b
$ deduxit prove --logic intuitionistic --linear 'a, a -> b |- b'
valid (intuitionistic logic by the cbn translation into ILL, two-sided engine)
───── ax
a ⊢ a
────── !L
!a ⊢ a
─────── !R   ───── ax
!a ⊢ !a      b ⊢ b
────────────────── ⊸L
  !a, !a ⊸ b ⊢ b
 ───────────────── !L
 !a, !(!a ⊸ b) ⊢ b
```

## A countermodel

A sequent that is not valid comes with a Kripke model that shows why.
The model has worlds, each forcing some atoms. A world lies below the
worlds its arrows reach, and every world forces what each world below it
forces. A world forces `A -> B` when every world at or above it that
forces `A` forces `B`, and `~A` is `A -> false`. Only in minimal logic
can a world force `false`; in the other logics a world forces `~A` when
no world at or above it forces `A`.
The model's world forces every formula left of `|-` and not the one
right of it, so the sequent cannot be a theorem. In the example at the
top, world 0 forces neither `a` nor `~a`, since world 1 above it forces
`a`; so it forces `~~a` and not `a`.

In classical logic the model has one world: the atoms it forces are
true, the others false. By default the verdict line names those atoms
itself; where it cannot, the model follows it. In minimal logic a world may force `false`, as
an atom. deduxit checks the model before it gives the verdict, and
`check` checks it again in a file `prove --format json` wrote. LJT
(`--method ljt`) finds no model: its "not valid" says the search was
exhaustive. A model past a size bound is not shown or checked, and the
verdict line says so.

To check a model of your own, write it in JSON and give `check` the
logic and the sequent. The model holds each world's atoms and its
arrows, by the index of the world above:

```console
$ echo '{"worlds": [{"atoms": [], "arrows": [1]}, {"atoms": ["a"]}]}' > model.json
$ deduxit check --logic intuitionistic --sequent 'a \/ ~a' model.json; echo "exit $?"
the model does not satisfy ⊢ a ∨ ¬a at every world: world 0 forces each formula left of ⊢ and not the one right of it
  at world 0: a not forced
  at world 0: ¬a not forced
  at world 0: a ∨ ¬a not forced
exit 1
$ deduxit check --logic intuitionistic --sequent 'a \/ ~a' --world 1 model.json
the model satisfies ⊢ a ∨ ¬a at world 1
```

A world gets `"falsity": true` where it forces `false` (minimal logic).
`check` first checks that the file is a model of the logic: its arrows
do not go round, every world forces what each world below it forces,
and only minimal logic has `falsity`. A key it does not know, such as a
misspelt `arrows`, is an error. The exit status is 0 where the sequent
holds, 1 where it fails, and 2 for a file that is no model of the logic.

`--format svg`, `png` and `pdf` draw the model: each world a box with
the atoms it forces, its number `w₀`, `w₁`, … beside it, an arrow up to
each world directly above it, and the model's world outlined in orange.
The drawing's description, which a screen reader reads, says what each
world forces and why the sequent fails. LaTeX and Typst draw no model.
`check` draws one too: the countermodel of an outcome, or a model of
your own at the world where it fails.

```console
$ deduxit prove --logic intuitionistic --format svg --output model.svg 'a \/ ~a'
```

`--format rocq` writes the model as a certificate for deduxit's Rocq
library ([Check a certificate in Rocq](rocq.md)): a lemma that LJ, or
minimal logic, does not derive the sequent, proved by the library's
theorem that a Kripke model refutes what it does not satisfy. The model
is written as data: for each world the worlds above it, the atoms it
forces, and the worlds that force `false`. The library checks it by
computation. `check --format rocq` writes the same for a countermodel
in an outcome, and for a model of your own at the world where it fails.

```console
$ deduxit prove --logic intuitionistic --format rocq 'a \/ ~a'
(* not valid (intuitionistic logic by the SAT solver): a checked countermodel refutes it *)
Lemma certificate : ~ lj (atom := string) intuitionistic [] (vee (var "a") (neg (var "a"))).
Proof.
  apply (kripke_refutation String.eqb String.eqb_spec intuitionistic
    {| later := [[1]; []];
       atoms := [[]; ["a"]];
       absurd := [] |} 0).
  vm_compute. reflexivity.
Qed.
```

## Truth tables and tableaux

For classical logic, `--method table` and `--method tableau` decide a
sequent the way a first logic course does, and print the table or the
tableau in place of a derivation.

The truth table has a row for every assignment of the atoms, from all
true to all false. A row marked `×` makes every formula left of `|-`
true and every formula right of it false, so it falsifies the sequent:

```console
$ deduxit prove --logic classical --method table 'a -> b |- b -> a'; echo "exit $?"
not valid (classical logic by its truth table): 1 of its 4 rows falsifies it, a checked countermodel
a  b  │  a → b  ⊢  b → a
T  T  │    T         T
T  F  │    F         T
F  T  │    T         F    ×
F  F  │    T         T
world 0 (the model's world): forces b
exit 1
```

The rows double with each atom, so `--method table` refuses a sequent
of more than 10 atoms. `--table-atoms N` raises the bound (the settings
key `limits.table_atoms`), and `none` lifts it. `check` takes the same
flag to check a table made with a raised bound.

A tableau marks each formula `T` (true) or `F` (false). It starts from
the sequent, its hypotheses true and the formulas right of `|-` false,
and expands one formula at a time: a true `p ∨ q` splits the branch in
two, one where `p` is true and one where `q` is. A branch closes, `×`,
where a formula is both true and false, `⊥` true or `⊤` false. Each line
names the rule of LK that made it and the line it expands:

```console
$ deduxit prove --logic classical --method tableau 'p \/ q, p -> r, q -> r |- r'
valid (classical logic by a tableau): every branch closes, a checked derivation of LK
1  T p ∨ q
2  T p → r
3  T q → r
4  F r
├─ 5  T p         ∨L 1
│  ├─ 6  F p      →L 2
│  │  × 5, 6
│  └─ 7  T r      →L 2
│     × 7, 4
└─ 8  T q         ∨L 1
   ├─ 9  F p      →L 2
   │  ├─ 10  F q  →L 3
   │  │  × 8, 10
   │  └─ 11  T r  →L 3
   │     × 11, 4
   └─ 12  T r     →L 2
      × 12, 4
```

A closed tableau is a derivation of LK read upside down: a branch is a
goal, its `T` formulas left of `⊢` and its `F` formulas right. deduxit
builds it step by step, checks every step by the rules of LK, and checks
the finished derivation, which `--format latex` and the other drawn
formats draw. A branch left open, with nothing more to expand, gives an
assignment that falsifies the sequent: its true atoms are those the
countermodel forces.

```console
$ deduxit prove --logic classical --method tableau 'a -> b |- b -> a'
not valid (classical logic by a tableau): an open branch gives a checked countermodel
1  T a → b
2  F b → a
3  T b      →R 2
4  F a      →R 2
├─ 5  F a   →L 1
│  open
└─ 6  T b   →L 1
   open
world 0 (the model's world): forces b
```

With `--format json`, the outcome holds the table or the tableau under
`search` (wire level 14): the table's rows as a `T` or `F` per formula of
its sequent's form, and the tableau's nodes, each before its children,
with its signed lines and either the line it expands or how its branch
ends.

## Check a tableau written by hand

`deduxit check --tableau` reads a signed tableau that a student wrote,
checks it rule by rule, and names the first step that breaks a rule.
The tableau is text, one line per signed formula, as on paper and as in
the forall x and Open Logic Project books:

- a line is its number, `T` or `F`, the formula, and at its end the
  number of the line it expands. A rule's name may come before that
  number (`F∨ 2`, `∨R 2` or `F\/ 2`), and is then checked too;
- the first lines, which expand no line, are the sequent's: its
  hypotheses `T` and its conclusions `F`;
- a branch starts with `-` under the lines it goes on from, its lines
  indented to the column after the `-`. The tree that
  `prove --method tableau` prints, with `├─` and `└─`, reads as well;
- `×` (or `x`) under a branch closes it, followed by the two lines of a
  formula true and false, or by the one line of `⊥` true or `⊤` false.
  `open` marks a branch that stays open.

A tableau with every step right and every branch closed proves the
sequent valid:

```console
$ deduxit check --tableau; echo "exit $?"
> 1 T p \/ q
> 2 F q \/ p
> 3 F q        2
> 4 F p        2
> - 5 T p      1
>   × 4, 5
> - 6 T q      1
>   × 3, 6
closed tableau of p ∨ q ⊢ q ∨ p: every step follows the rules and every branch closes, so the sequent is valid in classical logic
exit 0
```

A wrong step is named with its line, its rule and what the rule gives,
and the exit status is 1:

```console
$ deduxit check --tableau; echo "exit $?"
> 1 T p /\ q
> 2 F q
> 3 T p        1
> 4 T r        1
wrong tableau of p ∧ q ⊢ q: line 4: T∧ on line 1 (T p ∧ q) adds T p and T q, not T r
exit 1
```

The check also says where a branch is marked wrongly (`×` with nothing
that closes it, `open` on a branch that closes or has a formula left to
expand) and which branch is not finished yet. A finished open branch
makes the sequent not valid: its true atoms give a countermodel, which
is checked against the sequent and printed. `--sequent` names the
sequent the tableau must start from, as an exercise sets it:

```console
$ deduxit check --tableau --sequent 'a -> b |- b -> a'
> 1 T a -> b
> 2 F b -> a
> 3 T b 2
> 4 F a 2
> - 5 F a 1
>   open
> - 6 T b 1
>   open
open tableau of a → b ⊢ b → a: every step follows the rules, every branch is finished, and the branch ending at line 5 is open, so the sequent is not valid in classical logic: a checked countermodel
world 0 (the model's world): forces b
```

An equivalence expands either way: by its sides' values, as the
textbooks do (`T p ↔ q` into `T p, T q` and `F p, F q`), or into its two
implications, as LK does. A tableau without a fault is checked once more
as a derivation in the built-in calculus `tableau` (see
[Calculi given as data](rules.md)), so a closed tableau is a checked
proof. `--format json` writes the review: `{"version": 14, "standing":
"closed"|"open"|"unfinished"|"wrong", "sequent", "text_line", "line",
"message", "countermodel"}`, where `text_line` is the line of the text
the message is about and `line` the tableau's own number for it.

## Draw the derivation

The derivation is checked with every weakening and contraction as an
inference of its own. It is drawn with each of them left out where it is
admissible, so that it reads as in G3, whose axiom takes a context
(`a ⊢ b, a` above) and whose rules contract what they use twice.
`--structural explicit` draws every one (the key `view.structural`), and
a certificate names every one whatever the flag says:

```console
$ deduxit prove --logic classical --structural explicit '((a -> b) -> a) -> a'
valid (classical logic by the affine translation into affine MALL, focus engine)
  ───── ax
  a ⊢ a
 ──────── WR
 a ⊢ b, a
────────── →R   ───── ax
⊢ a → b, a      a ⊢ a
───────────────────── →L
   (a → b) → a ⊢ a
 ─────────────────── →R
 ⊢ ((a → b) → a) → a
```

`--format latex`, `typst`, `svg`, `png` and `pdf` draw the derivation of
LK or LJ. `--format rocq` writes a certificate over `Prop` that needs no
library; a classical one uses the standard library's excluded middle.
For a sequent that is not valid, where its image's refutation or the SAT
solver's model is a classical assignment, the certificate is the lemma
that the formula does not hold for all propositions, proved by
instantiating the atoms with `True` and `False`. A Kripke countermodel
of intuitionistic or minimal logic has a certificate of its own
([A countermodel](#a-countermodel)).

```console
$ deduxit prove --logic classical --format rocq 'a \/ ~a'
(* valid (classical logic by the affine translation into affine MALL, focus engine) *)
Lemma certificate : forall a : Prop, (a \/ ~ a).
Proof.
  exact (fun (a : Prop) => NNPP (a \/ ~ a) (fun k'1 : ~ (a \/ ~ a) => (let k'2 : ~ a := fun x' => k'1 (or_introl x') in let k'3 : ~ (~ a) := fun x' => k'1 (or_intror x') in (k'3 (fun h'4 : a => (k'2 h'4)))))).
Qed.
$ deduxit prove --logic classical --format rocq 'a -> b, b |- a'
(* not valid (classical logic by the affine translation into affine MALL, focus engine): the translated sequent is unprovable: the classical reading is false where b is true and every other atom false (! and ? erased, ⊗ and & as and, ⅋ and ⊕ as or, 1 and ⊤ true, ⊥ and 0 false), and every provable sequent reads as a tautology *)
Lemma certificate : ~ (forall a b : Prop, (a -> b) -> b -> a).
Proof.
  intro H. specialize (H False True). tauto.
Qed.
```

## Read a problem file

A `.p` file under `--logic` is a problem in TPTP's syntax, as the ILTP
library writes them (`nix build .#iltp` fetches its 274 propositional
problems). The batch ([Decide many sequents at once](batch.md)) takes a
directory of them. A name that TPTP allows and this syntax keeps for
itself is renamed so that the sequent reads back: a keyword such as
`par` or `true` becomes `par_` or `true_`, and a function symbol named
like a predicate gets `_f` after its name. A line `include('NAME').`
reads the file it names, found under the directory `$TPTP` names, where
set, else under the problem's directory or one above it, as TPTP's
library lays its axioms out (`nix build .#iltp-first-order` fetches
ILTP's 2 670 first-order problems with their `Axioms/`). A missing file
and an include of itself are refused by name. A file need not be UTF-8
in its comments, where some of ILTP's problems write an author's name in
Latin-1: such a byte is read as it comes, since comments are skipped. In
a formula it is refused with the line it is on.

```console
$ echo 'fof(pel1, conjecture, (p => q) <=> (~ q => ~ p)).' > pel1.p
$ deduxit prove --logic intuitionistic --quiet --file pel1.p
not valid (intuitionistic logic by the SAT solver): a checked countermodel refutes it
$ deduxit prove --logic classical --quiet --file pel1.p
valid (classical logic by the affine translation into affine MALL, focus engine)
```

## First-order sequents

A formula may apply predicates to terms and bind variables:
`forall x. p(x) -> q(x)` and `exists x. p(x)` (or `∀x.` and `∃x.`; a
quantifier reaches as far right as it can, and `forall x y.` binds
two). A name in a term is the variable of the nearest quantifier that
binds it, else a constant or a function symbol, as in `p(f(a), x)`. TPTP's
`! [X] :` and `? [X] :` read the same way. There is no equality yet: `=`
is refused, with a hint to write a predicate.

`prove` decides a first-order sequent by grounding where it lies in a
class grounding decides ([Decidable classes](#decidable-classes)), in
every logic. Otherwise it decides it by the connection prover in
classical logic, and through the embedding in intuitionistic and minimal
logic. Under a time limit, grounding or the connection prover gets 90%
of it and the embedding the rest: on ILTP's first-order problems that
proves five theorems more at 2 seconds. In classical logic a search for
a finite structure runs beside the prover ([Finite models](#finite-models)). A valid sequent gets a checked
derivation with the quantifier rules:

```console
$ deduxit prove --logic classical '|- exists x. (p(x) -> forall y. p(y))'
valid (classical logic by the connection prover)
              ─────────────────────────── ax
              p(b), p(a) ⊢ p(b), ∀y. p(y)
─────────────────────────────────────────────────────── →R
p(a) ⊢ ∃x. (p(x) → (∀y. p(y))), p(b), p(b) → (∀y. p(y))
─────────────────────────────────────────────────────── ∃R
         p(a) ⊢ ∃x. (p(x) → (∀y. p(y))), p(b)
       ──────────────────────────────────────── ∀R
       p(a) ⊢ ∃x. (p(x) → (∀y. p(y))), ∀y. p(y)
     ──────────────────────────────────────────── →R
     ⊢ ∃x. (p(x) → (∀y. p(y))), p(a) → (∀y. p(y))
     ──────────────────────────────────────────── ∃R
              ⊢ ∃x. (p(x) → (∀y. p(y)))
$ deduxit prove --logic intuitionistic 'forall x. p(x) |- exists y. p(y)'
valid (intuitionistic logic by the cbn translation into ILL, two-sided engine)
    ─────────── ax
    p(a) ⊢ p(a)
  ─────────────── ∀L
  ∀x. p(x) ⊢ p(a)
─────────────────── ∃R
∀x. p(x) ⊢ ∃y. p(y)
```

The connection prover works on the formulas as they are written, not
on a clausal form. It connects each literal with a complementary one,
renaming the variables of a quantified formula where it needs the
formula more than once, so the drinker's sequent above, which uses its
witness twice, is valid. Its search deepens a bound on the length of
the path of literals it has entered. It answers "not valid" only where
no quantifier needs a witness term: every `∀` is one to prove and every
`∃` one to assume, negations and implications taken into account. Its
search then ends:

```console
$ deduxit prove --logic classical --method connection 'exists x. p(x) |- forall y. p(y)'; echo "exit $?"
not valid (classical logic by the connection prover): the search was exhaustive
exit 1
$ deduxit prove --logic classical --method connection --path-length 2 'forall x. (p(x) -> p(f(x))), p(a) |- p(f(f(f(a))))'; echo "exit $?"
unknown (classical logic by the connection prover): the bound of 2 on the active path was reached after …; raise it with --path-length N, or lift it with --path-length none to deepen it while the time limit lasts
exit 3
```

Any other sequent it finds no proof of is unknown: by default it
deepens while the time limit lasts (`--path-length N` stops it at a
bound), and where it gives up at a bound the embedding decides in its
place. `--restricted-backtracking` keeps the first way it closes each
literal, which often finds a proof sooner but may miss one, so a sequent
it finds none of is unknown, even one the search above refuted:

```console
$ deduxit prove --logic classical --method connection --restricted-backtracking 'p(a) |- p(b)'; echo "exit $?"
unknown (classical logic by the connection prover): no proof was found, but the search may miss proofs here after …
exit 3
```

`--stats` prints its steps, its extension and reduction steps, the
literals it closed by an equal one solved before, and the bound it
reached.

`--method connection` decides intuitionistic sequents too. A classical
connection proof is an intuitionistic one when its literals' prefixes,
strings that stand for the worlds of a Kripke model, can be made equal.
It answers "valid" only with a checked derivation of LJ, and
"unknown" otherwise, even without quantifiers: it cannot show that no
proof exists, and it can miss one. The default of intuitionistic logic stays the embedding:

```console
$ deduxit prove --logic intuitionistic --method connection 'exists x. p(x), forall x. (p(x) -> q(x)) |- exists y. q(y)'
valid (intuitionistic logic by the connection prover)
─────────────────────────────────────────── ax   ──────────────────────────────────── ax
p(a), ∀x. (p(x) → q(x)), p(a) → q(a) ⊢ p(a)      q(a), ∀x. (p(x) → q(x)), p(a) ⊢ q(a)
───────────────────────────────────────────────────────────────────────────────────── →L
                     ∀x. (p(x) → q(x)), p(a), p(a) → q(a) ⊢ q(a)
                     ─────────────────────────────────────────── ∀L
                           ∀x. (p(x) → q(x)), p(a) ⊢ q(a)
                         ────────────────────────────────── ∃R
                         ∀x. (p(x) → q(x)), p(a) ⊢ ∃y. q(y)
                       ────────────────────────────────────── ∃L
                       ∃x. p(x), ∀x. (p(x) → q(x)) ⊢ ∃y. q(y)
$ deduxit prove --logic intuitionistic --method connection '|- exists x. (p(x) -> forall y. p(y))'; echo "exit $?"
unknown (intuitionistic logic by the connection prover): no proof was found, but the search may miss proofs here after …
exit 3
```

### Decidable classes

Validity is decidable for two classes of first-order sequents, and
`--method grounding` decides them:
- the Bernays–Schönfinkel class: no function symbols, and no `∀` under
  an `∃`;
- the monadic class: no function symbols, and no predicate of two or
  more arguments.

"Under" is read with negations pushed inward, the hypotheses negated
(as `⊢` reads them), and every quantifier moved inward as far as it
goes, distributing `∧` and `∨` where that frees it (up to a bound), so
`∀x. ∃y. (p(x) ∧ q(y))` counts as `(∀x. p(x)) ∧ (∃y. q(y))`.
Grounding replaces each `∀` by a fresh constant and each `∃` by the
disjunction of its instances over the finitely many constants, and hands
the result to the SAT solver. A valid sequent gets the solver's checked
clausal proof, and by default the connection prover's derivation of LK
where that prover finds one quickly. A sequent that is not valid gets a
finite structure, which deduxit checks against the sequent before it
answers: its elements, the element each constant names, and the set
each predicate holds of:

```console
$ deduxit prove --logic classical 'exists x. p(x), q |- forall x. p(x)'; echo "exit $?"
not valid (classical logic by grounding): a checked finite structure of 2 elements refutes it
domain: {0, 1}
p: {1}
q: true
exit 1
```

A structure refutes the sequent in intuitionistic and minimal logic too,
since what is valid there is valid classically. A sequent grounding
finds valid is left to the other methods in those logics, since
classical validity says nothing of them:

```console
$ deduxit prove --logic intuitionistic 'exists x. p(x) |- forall x. p(x)'; echo "exit $?"
not valid (intuitionistic logic by grounding): a checked finite structure of 2 elements refutes it
domain: {0, 1}
p: {1}
exit 1
$ deduxit prove --logic minimal --method grounding '|- exists x. (p(x) -> forall y. p(y))'; echo "exit $?"
unknown (minimal logic by grounding): the sequent is valid in classical logic, which says nothing of minimal logic
exit 3
```

`check` reads the outcome's JSON and evaluates its structure again.
`--stats` prints the elements, the ground atoms and formulas, and the
solver's counters. A structure too large to check within the limits is
left out, and the verdict line says so; the verdict then rests on the
solver's model of the grounded sequent, which was checked.

```console
$ deduxit prove --logic classical --format json 'exists x. p(x), q |- forall x. p(x)' > s.json; echo "exit $?"
exit 1
$ deduxit check s.json
valid refutation of ∃x. p(x), q ⊢ ∀x. p(x) (classical logic): a finite structure of 2 elements refutes it
domain: {0, 1}
p: {1}
q: true
```

### Finite models

`--method models` looks for a finite structure that refutes a
first-order sequent, function symbols included. It tries structures of
1, 2, … elements in turn: for each size it writes "the sequent fails in
a structure of this size" as a formula of propositional logic, every
`∀` and `∃` spelled out over the elements, and asks the SAT solver for
a model. A structure it finds is checked against the sequent before the
answer, and printed after the verdict line with each function's table:

```console
$ deduxit prove --logic classical --method models 'forall x. r(x, f(x)) |- exists x. r(f(x), x)'; echo "exit $?"
not valid (classical logic by the model search): a checked finite structure of 3 elements refutes it
domain: {0, 1, 2}
f: 0 ↦ 1, 1 ↦ 2, 2 ↦ 0
r: {(0, 1), (1, 2), (2, 0)}
exit 1
```

The structure refutes the sequent in intuitionistic and minimal logic
too. The search never finds a sequent valid: a sequent that no structure
up to `--model-size N` elements refutes is unknown (exit 3), and without
the flag the search goes on until the time limit. Some sequents are
refuted only by infinite structures, such as a strict order without a
greatest element.

```console
$ deduxit prove --logic classical --method models --model-size 2 'forall x. p(x) |- p(f(a))'; echo "exit $?"
unknown (classical logic by the model search): no structure of up to 2 elements refutes it after …; raise the bound with --model-size N, or lift it with --model-size none to try larger structures while the time limit lasts
exit 3
```

In classical logic `prove` runs the search by default beside the
connection prover, on a thread of its own, the first to answer winning;
where the prover gives up at a bound, the search goes on for 5% of the
time limit and the embedding then takes over. On one thread (`--jobs 1`) the prover gets 70% of
the time limit and the search the rest of the first-order share. Without the time limit, the
library runs it last, on a first-order sequent the embedding leaves
unknown, in every logic. On ILTP's first-order problems it refutes 210
in classical logic at 2 seconds, each within 0.4 seconds: 176 of the 178
the library marks CounterSatisfiable that have no equality, among them
every one of the 77 grounding refutes and the 84 nanoCoP refutes.

```console
$ deduxit prove --logic classical 'forall x. (p(x) -> p(f(x))), p(a) |- p(b)'; echo "exit $?"
not valid (classical logic by the model search): a checked finite structure of 2 elements refutes it
domain: {0, 1}
a = 0
b = 1
f: 0 ↦ 0, 1 ↦ 1
p: {0}
exit 1
```

In intuitionistic and minimal logic the search also tries Kripke models
with growing domains: worlds ordered by arrows, each with a domain that
no world above shrinks, the predicates growing along the arrows, and in
minimal logic `⊥` forced where the model says. For 1, 2, … it tries the
structures of that many elements, then the models of that many worlds
(`--model-worlds N` bounds them) with domains of up to that many
elements (`--model-size N`). The model is checked before the answer,
and printed with a line per world, its arrows to the worlds directly
above it after `below`:

```console
$ deduxit prove --logic intuitionistic --method models '(forall x. p(x)) -> q |- exists x. (p(x) -> q)'; echo "exit $?"
not valid (intuitionistic logic by the model search): a checked Kripke model of 2 worlds with growing domains refutes it
world 0 (the model's world): domain {0}; below 1
world 1: domain {0, 1}; p: {0}
exit 1
```

By default `prove` runs this search beside the embedding in
intuitionistic and minimal logic, on a thread of its own, the first to
answer winning. In intuitionistic logic a finite model never refutes
`¬¬A` where `A` is valid classically, since every world of a finite
model sees a world with none above it, where intuitionistic and
classical forcing agree. So the failure of Glivenko's theorem at first
order shows only on infinite models: `¬¬∀x. (p(x) ∨ ¬p(x))` is valid
classically and not intuitionistically, and the search answers unknown
on it. (In minimal logic a world with none above it may force `⊥`, so
there finite models refute such formulas, `¬¬(⊥ → p(a))` among them.)
The search's domains are initial segments of one numbering of the
elements, so two worlds that are not ordered add the same new elements;
models over a tree, which suffice for completeness, lose nothing by it.

`--stats` prints the largest structure tried, the worlds of the last
Kripke model tried, the atoms and formulas of the last formula, and the
solver's counters.

The translation into linear logic may lose a first-order proof: the
classical image cannot use a hypothesis twice. So where the image is
unprovable, the answer is unknown (exit 3), never "not valid". The
other methods refuse a first-order sequent, naming themselves (exit 2):

```console
$ deduxit prove --logic classical --method embedding '|- exists x. (p(x) -> forall y. p(y))'; echo "exit $?"
unknown (classical logic by the affine translation into affine MALL, focus engine): the translated sequent is unprovable, which leaves a first-order sequent open, since the translation may lose its proofs
exit 3
$ deduxit prove --logic classical --method sat 'forall x. p(x) |- p(a)'; echo "exit $?"
error: the SAT solver takes propositional sequents only, and this one is first-order
exit 2
```

A session proves one step by step with the quantifier rules of LK, LJ
and LM:
- `∀R` and `∃L` (`forallR`, `existsL`) need nothing: the session names
  a fresh eigenvariable, `a`, `b`, … skipping the sequent's own names.
- `∀L` and `∃R` (`forallL`, `existsR`) take a witness term after the
  rule's name, or none. It may use the sequent's function symbols and
  the eigenvariables that a step below the goal introduced. `rules`
  marks these rules `(with a witness or none)`.
- Without a witness the session leaves it open, as `$1`, `$2`, …, and
  the axiom fills it with the term that makes its formula equal to one
  on the other side of `⊢`. A witness that would need an eigenvariable
  introduced after it was left open is refused. One still open when the
  proof is finished becomes a fresh constant, so `∀x. p(x) ⊢ ∃x. p(x)`
  proves with no constant in the sequent (below).
- `close` refuses a first-order goal: the session cannot graft a
  first-order derivation yet.

`check` checks a finished derivation that `proof FILE` wrote:

```console
$ deduxit interact --logic intuitionistic 'forall x. (p(x) -> q(x)), exists x. p(x) |- exists y. q(y)'
> apply 0 L1 existsL
opened goal 1: L0: ∀x. (p(x) → q(x)), L1: p(a) ⊢ R0: ∃y. q(y)
> rules 1 R0
∃R (with a witness or none)
> apply 1 R0 existsR a
opened goal 2: L0: ∀x. (p(x) → q(x)), L1: p(a) ⊢ R0: q(a)
> apply 2 L0 forallL a
opened goal 3: L0: p(a) → q(a), L1: ∀x. (p(x) → q(x)), L2: p(a) ⊢ R0: q(a)
> apply 3 L0 ->L
opened goal 6: L0: ∀x. (p(x) → q(x)), L1: p(a) ⊢ R0: p(a)
opened goal 7: L0: q(a), L1: ∀x. (p(x) → q(x)), L2: p(a) ⊢ R0: q(a)
> apply 6 R0 ax
closed
> apply 7 R0 ax
closed; no goal is open: `proof` checks the proof
> proof d.json
valid proof written to d.json
$ deduxit check d.json
valid derivation in LJ of ∀x. (p(x) → q(x)), ∃x. p(x) ⊢ ∃y. q(y) (intuitionistic logic)
────────────────────────────── ax   ──────────────────────────────────── ax
∀x. (p(x) → q(x)), p(a) ⊢ p(a)      q(a), ∀x. (p(x) → q(x)), p(a) ⊢ q(a)
──────────────────────────────────────────────────────────────────────── →L
              p(a) → q(a), ∀x. (p(x) → q(x)), p(a) ⊢ q(a)
              ─────────────────────────────────────────── ∀L
                    ∀x. (p(x) → q(x)), p(a) ⊢ q(a)
                  ────────────────────────────────── ∃R
                  ∀x. (p(x) → q(x)), p(a) ⊢ ∃y. q(y)
                ────────────────────────────────────── ∃L
                ∀x. (p(x) → q(x)), ∃x. p(x) ⊢ ∃y. q(y)
```

Both witnesses of `∀x. p(x) ⊢ ∃x. p(x)` are left open, and the axiom
fills one with the other:

```console
$ deduxit interact --logic intuitionistic 'forall x. p(x) |- exists x. p(x)'
> apply 0 R0 existsR
opened goal 1: L0: ∀x. p(x) ⊢ R0: p($1)
> apply 1 L0 forallL
opened goal 2: L0: p($2), L1: ∀x. p(x) ⊢ R0: p($1)
> apply 2 L0 ax
filled $2 with $1
closed; no goal is open: `proof` checks the proof
> proof
valid proof in LJ (intuitionistic logic)
───────────────────── ax
p(a), ∀x. p(x) ⊢ p(a)
───────────────────── ∀L
   ∀x. p(x) ⊢ p(a)
 ─────────────────── ∃R
 ∀x. p(x) ⊢ ∃x. p(x)
```

The checker takes the rules in G3's form and in G1's: `∀L` may keep
its formula in the premise or drop it, and so may `∃R` in LK. An
eigenvariable must occur nowhere in the conclusion of its `∀R` or
`∃L`. LaTeX, Typst and SVG draw such a derivation as they draw any
other; it has no Rocq certificate yet (`no certificate: …`).

A first-order sequent, derivation or session is JSON at wire level 25
(the manual's schema, `wire.schema.json`, has every key). Beside the
propositional keys the sequent has the first-order sequent's of linear
logic ([Prove in first-order logic](first-order.md)): `symbols` (each a
name and its arity), `fo` (the terms: `{"B": k}` the variable of the
k-th quantifier out, `{"F": [symbol, [terms]]}` an application), each
atom as `{"P": [symbol, [terms]]}`, and `binders`, one name per
quantifier. A formula `{"∀": a}` or `{"∃": a}` binds in its operand `a`.
An outcome refuted by grounding or the model search holds `"structure":
{"sequent": …, "structure": {"size": 2, "constants": [["a", 0]],
"functions": [["f", [1, 0]]], "predicates": [["p", [[0]]]]}}`: the
sequent, the number of elements, each constant's element, each function
symbol's values in the lexicographic order of its arguments (`f(0)`,
`f(1)`; the key left out where there is no function symbol), and each
predicate's tuples, a predicate left out holding of none.
A quantifier inference names its term by `"term": k`, a position in
`fo`, where a derivation's eigenvariable is `{"E": n}`. A session's open
witness is `{"M": k}` (`$1` for `k = 0`), and `"bindings": [[k, term,
step]]` lists the witnesses an axiom filled; its inferences are written
as their steps made them, and reading the session fills them again.

## JSON output

With `--format json` every method writes the outcome of the ordinary
sequent: its `verdict` (`valid`, `not_valid`, or `unknown` with a
`reason`), its `logic`, the `search` that decided it and, for a valid
sequent, its evidence. `search` says the method in `method`; LJT and the
SAT solver add their `statistics`. A bound or the time limit that ends
the read-back or a check after the search decided gives `unknown` with
that `reason` and the search's statistics. A truth table or a tableau
that a bound or the time limit ends before it is built (for the
tableau, before its derivation is checked) has no outcome, since the
outcome holds the table or the tableau; its line goes to standard error
alone:

```console
$ deduxit prove --logic intuitionistic --format json '~~a -> a'
{"version":12,"deduxit":"0.7.0","verdict":"not_valid","logic":"intuitionistic","search":{"method":"sat","statistics":{"calls":4,"conflicts":0,"lemmas":0,"worlds":4,"facts":0,"clauses":0,"proof_steps":0,"ljt":null}},"countermodel":{"sequent":{"formulas":[{"V":0},{"¬":0},{"¬":1},{"→":[2,0]}],"atoms":["a"],"left":[],"right":[3]},"model":{"worlds":[{"atoms":[],"arrows":[1,2]},{"atoms":["a"],"arrows":[3]},{"atoms":[],"arrows":[3]},{"atoms":["a"]}],"world":0}}}
```

Through the embedding, `search` holds the `translation` and, under
`image`, the outcome of the search on the image, as `prove` writes it
without `--logic` (its proof, or its refutation):

```console
$ deduxit prove --logic classical --method embedding --format json 'a |- b'
{"version":12,"deduxit":"0.7.0","verdict":"not_valid","logic":"classical","search":{"method":"embedding","translation":"affine","image":{"verdict":"unprovable","refutation":{"kind":"classical","assignment":[true,false]},"fragment":"MLL","mode":"affine","engine":"focus","statistics":{"nodes":1,"memo_hits":0,"memo_entries":1,"splits":0,"links":0,"tests":0,"copies":0,"forward_copies":0,"work":16},"sequent":{"terms":[{"D":0},{"V":1}],"roots":[0,1],"atoms":["a","b"],"antecedents":0}}},"countermodel":{"sequent":{"formulas":[{"V":0},{"V":1},"⊥"],"atoms":["a","b"],"left":[0],"right":[1]},"model":{"worlds":[{"atoms":["a"]}],"world":0}}}
```

A valid sequent has its `derivation`. One valid by a clausal proof has
`clausal_proof` in its place, with the counts of its variables, clauses
and steps. Where the embedding proved the image but a bound or the time
limit refused its derivation, neither is there, and the image's proof
is the evidence:

```console
$ deduxit prove --logic classical --method sat --format json 'a \/ ~a'
{"version":12,"deduxit":"0.7.0","verdict":"valid","logic":"classical","search":{"method":"sat","statistics":{"calls":1,"conflicts":1,"lemmas":0,"worlds":0,"facts":0,"clauses":4,"proof_steps":1,"ljt":null}},"clausal_proof":{"variables":3,"clauses":4,"steps":1}}
```

`--linear` writes the image's outcome alone, as without `--logic`.

`check` reads every outcome `prove --logic` writes, and a derivation of
LK or LJ. It checks the derivation by the rules of its calculus, and the
image's proof or refutation where the embedding found one:

```console
$ deduxit prove --logic intuitionistic --format json 'a |- a' > valid.json
$ deduxit check --file valid.json
valid derivation in LJ of a ⊢ a (intuitionistic logic)
───── ax
a ⊢ a
```

A clausal proof is written as its counts, LJT and the SAT solver write
no refutation of a sequent that is not valid, and an unknown holds no
proof: there `check` says the file holds nothing to check, with exit
status 3.

## Step by step

`deduxit interact --logic classical`, `intuitionistic` or `minimal`
proves the sequent step by step by the rules of LK or LJ. The session
has the commands of a linear session ([Prove step by
step](interactive.md)): `goals`, `rules`, `apply`, `undo`, `close`,
`invertible`, `show`, `proof`, `save`, `load`, `help` and `quit`.
- A formula is named by its side and its position there: `L0`, `L1`, …
  left of `⊢` and `R0`, `R1`, … right of it, as `goals` lists them.
- A rule is named by its name or an ASCII spelling: `andR`, `orR1`,
  `->L` or `impL`, `~R` or `notR`, `iffL1`, `falseL`, `WL`, `CR`.
- The axiom closes a goal that has its formula on both sides, whatever
  else it holds.
- A rule with two premises gives both the whole context, as LJ's `→L`
  does in G3. Formulas named after the rule go to the first premise
  instead, and the rest to the second: `apply 1 L1 ->L L0` sends `L0`
  to the first.
- `close` decides a goal through the embedding, whose derivations have
  no cut, by the translation `--translation` chooses. It grafts the
  derivation found, which `undo` takes back whole.
- `invertible on` (or `--invertible` from the start) applies every
  invertible rule by itself: the axiom and the rules of the connectives
  with a shared context, but in LJ `→L`, `¬L` and `∨R₁`, `∨R₂`, which
  can lose a proof. `undo` then takes back a step with the steps that
  followed it.

```console
$ deduxit interact --logic intuitionistic 'a -> b, b -> c |- a -> c'
> goals
goal 0: L0: a → b, L1: b → c ⊢ R0: a → c
> apply 0 R0 ->R
opened goal 1: L0: a → b, L1: b → c, L2: a ⊢ R0: c
> rules 1 L1
WL  CL  →L (with a split or none)
> apply 1 L1 ->L
opened goal 4: L0: a → b, L1: a ⊢ R0: b
opened goal 5: L0: a → b, L1: c, L2: a ⊢ R0: c
> apply 5 L1 ax
closed
> close 4
goal 4: valid (intuitionistic logic by the cbn translation into ILL, two-sided engine)
no goal is open: `proof` checks the proof
> proof
valid proof in LJ (intuitionistic logic)
───── ax   ───── ax
a ⊢ a      b ⊢ b
──────────────── →L   ─────────────── ax
  a → b, a ⊢ b        a → b, c, a ⊢ c
  ─────────────────────────────────── →L
          a → b, b → c, a ⊢ c
          ──────────────────── →R
          a → b, b → c ⊢ a → c
```

With the invertible rules on, LJ leaves `→L` to the user:

```console
$ deduxit interact --logic intuitionistic --invertible 'c, a -> b, a |- b /\ c'
applied 2 invertible steps
> goals
goal 1: L0: c, L1: a → b, L2: a ⊢ R0: b
> apply 1 L1 ->L
then 2 invertible steps
closed; no goal is open: `proof` checks the proof
```

In intuitionistic and minimal logic every goal keeps one formula right
of `⊢`:
- A sequent with none starts as `Γ ⊢ ⊥`.
- The rules of LK that would leave a premise with none or two (`WR`,
  `CR`, `⊥R`, `∨R` with both disjuncts) are refused with the reason.
- `¬R` and `¬L` take the form of `→R` and `→L` for `A → ⊥`.
- Minimal logic has no `⊥L`.

The goals the search cannot decide stay open with the reason, as `prove`
gives it. `show` draws the derivation so far, open goals as bare
sequents. `proof` checks the finished one and prints it in every format
of `prove`, `--structural` choosing how the structural rules are drawn
(`proof --rocq` names every one). `save` writes the session as JSON,
which `--state FILE` resumes:

```console
$ deduxit interact --logic minimal 'a |- ~~a'
> apply 0 R0 ~R
opened goal 1: L0: a, L1: ¬a ⊢ R0: ⊥
> apply 1 R0 falseR
error: the rule ⊥R is not available in minimal logic: its premise would have no formula right of ⊢, and every goal of LJ has exactly one
> apply 1 L1 ~L
opened goal 3: L0: a ⊢ R0: a
opened goal 4: L0: a, L1: ⊥ ⊢ R0: ⊥
> close
goal 3: valid (minimal logic by the cbn translation into ILL, horn engine)
goal 4: valid (minimal logic by the cbn translation into ILL, horn engine)
no goal is open: `proof` checks the proof
> proof --rocq
valid proof in LJ (minimal logic)
Lemma certificate : forall a : Prop, a -> (~ ~ a).
Proof.
  exact (fun (a : Prop) (h'1 : a) => (fun h'2 : (~ a) => (let h'3 : False := h'2 (h'1) in h'3))).
Qed.
```
