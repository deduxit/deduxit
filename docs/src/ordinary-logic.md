# Ordinary logic

`--logic classical`, `intuitionistic` or `minimal` reads a formula or a
sequent of ordinary propositional logic instead, written with `->`/`→`,
`/\`/`∧`, `\/`/`∨`, `~`/`¬`, `<->`/`↔`, `true`/`⊤` and `false`/`⊥`
(`~` binds tightest, then `/\`, `\/`, `->`, and last `<->`; `->` and
`<->` group to the right). `prove` decides it and draws the proof as a
derivation of LK or LJ, which is checked by the rules of the logic
before it is shown:

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
valid (intuitionistic logic by LJT)
  ───── ax        ───── ax
  a ⊢ a           b ⊢ b
───────── ∨R₂   ───────── ∨R₁
a ⊢ b ∨ a       b ⊢ b ∨ a
───────────────────────── ∨L
      a ∨ b ⊢ b ∨ a
   ─────────────────── →R
   ⊢ (a ∨ b) → (b ∨ a)
$ deduxit prove --logic intuitionistic '~~a -> a'
not valid (intuitionistic logic by LJT): the search was exhaustive
```

## How a sequent is decided

`--method` chooses how (the key `ordinary.method`):

- `ljt` is Dyckhoff's calculus LJT, for intuitionistic and minimal logic.
  Its search always ends, so every such sequent is answered `valid` or
  `not valid` unless the time limit or the memory runs out first. It is
  the default for both logics.
- `embedding` translates the sequent into linear logic, searches the
  image there and reads the proof back. It decides all three logics and
  is the default for classical logic.

On the ILTP library's 274 problems, with 2 seconds each, LJT decides 200
in intuitionistic logic and 200 in minimal logic, the embedding 188 in
each, and LJT is the faster on all but two of the problems both decide,
which take under a millisecond either way. A derivation
from LJT can be far larger than the search behind it. Where LJT stops at
a bound other than the time limit, such as a derivation past
`--derivation-limit`, the default hands the sequent to the embedding,
and the verdict line names the translation. `--method ljt` reports the
bound instead. `--translation` and `--linear` name the embedding, so
either of them alone chooses it:

```console
$ deduxit prove --logic intuitionistic --method embedding -q '~~a -> a'
not valid (intuitionistic logic by the cbn translation into ILL, two-sided engine): the image is unprovable: the search was exhaustive
$ deduxit prove --logic minimal --stats -q 'a, a -> b |- b'
valid (minimal logic by LJT)
sequents expanded: 2
memo hits: 0
memo entries: 2
refuted by an earlier failure: 0
time: …
```

LJT's derivation of LJ uses Gentzen's cut for four of its rules, so its
lambda-term may hold a redex and its certificate a `let`. With
`--format json` the output is the outcome of the ordinary sequent: its
`verdict` (`valid`, `not_valid` or `unknown` with a `reason`), its
`logic`, the `engine` `ljt`, its `statistics` and the `derivation`.
`check` reads proofs of linear logic, so it takes the embedding's
outcome and refuses LJT's.

The derivation is checked with every weakening and contraction as an
inference of its own, and drawn with each of them left out where it is
admissible, so that it reads as in G3, whose axiom takes a context
(`a ⊢ b, a` above) and whose rules contract what they use twice;
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

Through the embedding, classical logic goes into affine MALL without
exponentials (negation normal form, `∧` as `&`, `∨` as `⅋`), where the
search needs no copy bound. Intuitionistic logic goes into ILL by
`--translation cbn` (Girard's call-by-name translation, the default),
`cbv` (his call-by-value one) or `01` (Liang and Miller's 0/1
translation); minimal logic is the same with false an atom. There the
search deepens its copy bound under the time limit, so an intuitionistic
formula may be answered unknown (exit status 3). `seq print` shows the
image, and `--linear` the linear proof of it:

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

A `.p` file under `--logic` is a problem in TPTP's syntax, as the ILTP
library writes them (`nix build .#iltp` fetches its 274 propositional
problems), and the batch takes a directory of them. `--format latex`,
`typst`, `svg`, `png` and `pdf` draw the derivation of LK or LJ, and
`--format rocq` writes a certificate over `Prop` that needs no library
(a classical one the standard library's excluded middle), and for a
sequent that is not valid, where its image's refutation is a classical
assignment, the lemma that the formula does not hold for all
propositions, proved by instantiating the atoms with `True` and `False`:

```console
$ echo 'fof(pel1, conjecture, (p => q) <=> (~ q => ~ p)).' > pel1.p
$ deduxit prove --logic intuitionistic --quiet --file pel1.p
not valid (intuitionistic logic by LJT): the search was exhaustive
$ deduxit prove --logic classical --quiet --file pel1.p
valid (classical logic by the affine translation into affine MALL, focus engine)
$ deduxit prove --logic classical --format rocq 'a \/ ~a'
(* valid (classical logic by the affine translation into affine MALL, focus engine) *)
Lemma certificate : forall a : Prop, (a \/ ~ a).
Proof.
  exact (fun (a : Prop) => NNPP (a \/ ~ a) (fun k'1 : ~ (a \/ ~ a) => (let k'2 : ~ a := fun x' => k'1 (or_introl x') in let k'3 : ~ (~ a) := fun x' => k'1 (or_intror x') in (k'3 (fun h'4 : a => (k'2 h'4)))))).
Qed.
$ deduxit prove --logic classical --format rocq 'a -> b, b |- a'
(* not valid (classical logic by the affine translation into affine MALL, focus engine): the image is unprovable: the classical reading is false where b is true and every other atom false (! and ? erased, ⊗ and & as and, ⅋ and ⊕ as or, 1 and ⊤ true, ⊥ and 0 false), and every provable sequent reads as a tautology *)
Lemma certificate : ~ (forall a b : Prop, (a -> b) -> b -> a).
Proof.
  intro H. specialize (H False True). tauto.
Qed.
```

## Step by step

`deduxit interact --logic classical`, `intuitionistic` or `minimal`
proves the sequent step by step by the rules of LK or LJ, with the
commands of a linear session (see [Interactive proving](interactive.md)):
`goals`, `rules`, `apply`, `undo`, `close`, `show`, `proof`, `save`,
`load`, `help` and `quit`. A formula is named by its side and its position
there, `L0`, `L1`, … left of `⊢` and `R0`, `R1`, … right of it, as `goals`
lists them; a rule by its name or an ASCII spelling (`andR`, `orR1`, `->L`
or `impL`, `~R` or `notR`, `iffL1`, `falseL`, `WL`, `CR`). The axiom closes
a goal that has its formula on both sides, whatever else it holds. A rule
with two premises gives both the whole context, as LJ's `→L` does in G3,
unless the formulas named after the rule go to the first premise and
the rest to the second (`apply 1 L1 ->L L0` sends `L0` to the first).
`close` decides a goal through the embedding, whose derivations have no
cut, by the translation `--translation` chooses, and grafts the
derivation found, which `undo` takes back whole:

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

In intuitionistic and minimal logic every goal keeps one formula right
of `⊢`: a sequent with none starts as `Γ ⊢ ⊥`, the rules of LK that
would leave a premise with none or two (`WR`, `CR`, `⊥R`, `∨R` with both
disjuncts) are refused with the reason, and `¬R` and `¬L` take the form
of `→R` and `→L` for `A → ⊥`; minimal logic has no `⊥L`. The goals the
search cannot decide stay open with the reason, as `prove` gives it.
`show` draws the derivation so far, open goals as bare sequents, and
`proof` checks the finished one and prints it in every format of
`prove`, `--structural` choosing how the structural rules are drawn
(`proof --rocq` names every one); `save` writes the session as JSON,
which `--state FILE` resumes:

```console
$ deduxit interact --logic minimal 'a |- ~~a'
> apply 0 R0 ~R
opened goal 1: L0: a, L1: ¬a ⊢ R0: ⊥
> apply 1 R0 falseR
error: the rule ⊥R is not one of LJ in minimal logic: its premise would have no formula right of ⊢, and every goal keeps one
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
