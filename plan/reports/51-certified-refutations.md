# Step 51: the one-sided and state-equation refutations, checked and certified

## Session 51a: the invariants

### Outcome

The Rocq library `rocq-linlog` proves both refutations, in every mode
linlog states them in, with every theorem's assumptions closed under the
global context. Its `nix flake check` passed at the head of three
commits in the workspace `../rocq-linlog-51a` on `main`: change ids
`olskqlku` ("Prove weighted counts: the one-sided count and the state
equation"), `tvlumoul` ("Raise the version to 0.3.0 with the weighted
counts in the changelog") and `wtkykwzw` ("Let a dead atom die in the
sign of a Horn program's bodies", the reviewer's finding). Nothing is
pushed.

One theorem certifies both refutations. `Weights.weighted_refutation`
says that weights per atom, a set of dead atoms and the sign they die in,
which the Boolean test
`overweight` accepts by computation refute a sequent, without weakening
and without cut, Mix allowed. The one-sided count is that test with the
weight 1 (or −1) on one atom. The state equation is the test with the
Horn engine's Farkas weights on the atoms and the atoms outside the
closure of the initial marking declared dead. With weakening (affine
mode, where the state equation refutes coverability),
`Weights.affine_refutation` does the same for sequents whose formulas
have outputs as a Horn program's have. The first-order form of the count
is `FirstOrderWeights.weighted_refutation`, and each one-sided theorem
has its two-sided form through the translation.

### Decisions

- **The state equation is proved as a weighted count of the derivation,
  not through firing sequences (departs from the design).** The plan
  reads "a proof of a Horn program's goal gives a firing sequence whose
  counts solve the state equation". Mechanising that needs the Horn
  shape over every sequent of a proof (curried clauses, partial
  clauses, the one-output count of `core-horn.md`) and an induction on
  the dyadic proof. Instead the library proves an invariant of every
  derivation of the calculus, whatever its shape: each formula has a set
  of possible contributions to a weighted count of its literals
  (`Weights.out`), `?A` contributing the sum of any number of `A`'s, and
  every derivable sequent can contribute zero or absorb
  (`weighted_balance`). The copies of a clause under `!` are the firing
  counts `xₜ` of the state equation, so a Farkas certificate makes every
  contribution of the goal positive. The Horn engine's refinement, that
  transitions which can never fire are dropped from the inequalities, is
  not linear (with them, the state equation of `a, !(a ⊗ a ⊸ b), !(c ⊸
  c ⊗ a) ⊢ a ⊗ b` has a solution). It enters through dead atoms: an atom
  declared dead contributes `dead` as a positive literal (nothing a sum
  cancels, as `0`) and `absorb` as a negative one (as `⊤`), so its axiom
  still settles and the invariant holds for every choice of dead atoms.
  Which literal dies is a parameter `p` (the reviewer's finding, below).
  It must be the bodies' sign, positive for every intuitionistic
  program; a classical one-sided program may have negative bodies.
  A `⊗` keeps a dead side dead whatever the other absorbs (`otimes`),
  since its sides are proved apart. A clause that can never fire has a
  dead atom in its body, so its copies contribute only zero. The
  theorem refutes exactly what `Refutation::StateEquation` claims, in
  each mode: every certificate `certify` accepts passes the test (the
  argument is in "What the next sessions must know"), and the test's
  inputs are the certificate's atom weights and the dead atoms, which a
  pass over the program computes without a search.
- **With weakening the invariant counts outputs.** Weakening drops any
  formula, so the count holds only of what a Horn program's proof
  never drops. `outputs` counts one for each literal of the output sign
  and for `1`, `⊤` and `!A`, a `⊗` one fewer than its sides, a `⅋` the
  sum and an additive the larger. Every derivable sequent of `shaped`
  formulas (each subformula's count non-negative, none below a `?`) has
  at least one output. One with exactly one output settles with the
  formulas without an output dropped (`affine_balance`), which is
  `core-horn.md`'s count `q − 1 = mixes + weakened outputs` made an
  invariant. The weights need not be non-negative for soundness: the
  test bounds a droppable formula by zero as well as by its own sum.
- **The one-sided count and the state equation are one theorem.** The
  one-sided count is the weighted count with one atom weighed: copies of
  one sign add contributions of one sign, which the bound of `?A`
  (`lstar`) follows. Every count Rust states passes the test. The
  Rocq bound of a formula is at least the Rust interval's `least`:
  equal on literals, units, `⊗`, `⅋`, `&`, `⊕` and exponentials, and
  `0` contributes `never`, above any number. So `least > 0` in Rust,
  or `most < 0` with the weight negated, gives a positive Rocq bound;
  no `⊤` is in the goal, since the tally absorbs then.
- **The first-order count weighs by predicate or by closed atom.** A
  quantifier's rule changes no literal's predicate, and no literal
  without a bound variable. linlog's counts group the atoms of a
  predicate one of whose atoms has a bound variable
  (`Predicates::counted`) and count the others alone. So `value p ts` is
  `weight p` for a grouped predicate and `table p ts` for a `fixed`
  one, and the test asks every literal of a fixed predicate closed
  (`pinned`). This covers `OneSidedSymbol`, `UnbalancedSymbol`, and
  `OneSided` and `Unbalanced` on a sequent with a quantifier. The
  two-sided form needed the embedding of the first-order two-sided
  calculus in the one-sided one, which the library lacked
  (`FirstOrderWeights.ill_ll`).
- **What I read for the form.** The library's own `Refutations.v`
  (fixpoints that `cbn [...]` refolds, Boolean tests decided by
  `vm_compute`, `forallb_perm`) and its tests' certificate form. The Rocq
  9.0–9.3 import filter `Import M(T(..), f)`, which `FirstOrderWeights.v`
  uses to take the outcomes from `Weights` without the propositional
  names, is checked by the flake on all four versions.
- **The version is 0.3.0, unreleased.** `rocq-linlog.opam` states
  0.3.0, and the changelog has the section `[0.3.0] - unreleased` with
  what was added. `CITATION.cff` stays at 0.2.0 with its release date,
  since it describes the released version; the release (step 53) dates
  the changelog's section and moves the citation file, as
  `release.yml` requires. linlog's constant `export::rocq::LINLOG` must
  equal the opam version, so 51b raises it to 0.3.0 when the lock moves.

### Checks

- [x] Both refutations stated and proved: `Weights.weighted_balance`,
  `weighted_refutation`, `iweighted_refutation` (linear, Mix or not,
  classical and intuitionistic), `affine_balance`, `affine_refutation`,
  `iaffine_refutation` (with weakening), `FirstOrderWeights.weighted_balance`,
  `weighted_refutation`, `ill_ll`, `iweighted_refutation` (first order,
  linear).
- [x] Tests: `tests/Weights.v`, fourteen lemmas. Seven are
  certificates: the one-sided count two-sided, with Mix, and of the
  negative literal; the state equation with a dead clause, with
  positive and with negative bodies; the affine state equation
  two-sided and one-sided with negative bodies. Seven are rejections:
  the state equation without its dead atom, the dead sign wrong, a
  derivable sequent, a `⊤`, a calculus with weakening under the linear
  test, the wrong output sign, and a derivable affine sequent.
  `tests/FirstOrderWeights.v`, four lemmas:
  the count over a predicate, over one closed atom, the latter refused
  when counted by predicate, and an unpinned fixed predicate refused.
  Each compiled silently (exit 0).
- [x] `Print Assumptions` of the ten new theorems added to
  `tests/Assumptions.v`: 33 lines, each "Closed under the global
  context".
- [x] The library's `nix flake check` at `wtkykwzw` (commit `c63ff4db`,
  after the fix; before it also at `tvlumoul`): exit 0, "all checks
  passed!", 136 lines "Closed under the global context" (33 per
  Rocq 9.0–9.3, the rest the bridges'), under `flock -o /tmp/linlog-machine.lock … --max-jobs 4
  --cores 8`.
- [x] Version and changelog: `0.3.0`, "[0.3.0] - unreleased".
- [x] The fresh-context reviewer's round, one finding, answered and fixed
  (below); the flake check run again after the fix.

### The reviewer's round

One fresh-context reviewer (Opus 5.5 at `high`) checked each statement
against linlog's `Refutation::OneSided`, `OneSidedSymbol` and
`StateEquation` (`focus/counts.rs`, `horn/mod.rs`, `horn/equation.rs`,
`core-horn.md`), and checked that no assumption is open. It reported one
finding and confirmed the rest: the one-sided count and its first-order
form pass the test whenever Rust states them, the state equation passes
it for positive bodies in both modes, the induction covers every rule
of both calculi, and every theorem's assumptions are closed.

1. **Dead atoms broke the test for Horn programs whose bodies are
   negative literals.** A dead atom's positive literal was `dead` and
   its negative one absorbed. In a classical program with bodies `~a`,
   a clause that never fires has `~c` in its body, which absorbed, so
   the test refused a refutation the Horn engine states. The witness
   is `⊢ a, ?(~a ⊗ (a ⅋ a)), ?((~a ⊗ ~c) ⊗ b), ~b`: weight `b ↦ 1`,
   `certify` accepts, the test answered `false`. **Holds; fixed** at its
   cause, not by weakening a statement. The sign that dies is a
   parameter `p` of every theorem of `Weights.v`, the same `p` that
   names the outputs' sign with weakening. Soundness holds for either
   sign, since an axiom on a dead atom pairs `dead` with `absorb`. The
   witness is now the certificate `state_equation_negative_bodies`, and
   `negative_bodies_wrong_sign` pins that the other sign refuses. The
   reviewer had not run linlog to see whether it states
   `StateEquation` for that exact sequent rather than `Exhausted`. That
   does not matter, since the theorem must reach every certificate
   `certify` accepts, and this one is accepted.

### Modes the refutations do not reach

- **Cut.** Neither invariant survives a cut, whose formula is no
  subformula of the conclusion. Every test asks `cut R = false`, as the
  balance does, and linlog's searches are cut-free.
- **The one-sided count in affine mode.** Weakening drops any literal,
  and linlog states no count there (`intervals_run`).
- **The first-order count with weakening.** It is not stated, since
  linlog states no count in affine mode. Nor is the first-order state
  equation, since the Horn engine reads propositional programs only.

### What the next sessions must know

The theorems, as Rocq states them. The propositional ones are in
`From Linlog Require Import Weights.` (which exports `Refutations`,
`Calculus` and `Intuitionistic`). `y : atom -> Z` and `d : atom -> bool`
are arguments before the others, and `p : bool` after them (the sign
of a dead atom's literal that contributes nothing, `true` for the
positive one; with weakening also the outputs' sign; a Horn program's
bodies' sign in both roles):

```coq
Theorem weighted_refutation : forall {atom} (y : atom -> Z) (d : atom -> bool) (p : bool) R l,
  overweight y d p R l = true -> ~ ll R l.
Theorem iweighted_refutation : forall {atom} y d p R G C,
  overweight y d p (lrules R) (one_sided G C) = true -> ~ ill R G C.
Theorem affine_refutation : forall {atom} y d (p : bool) R l,
  overweight_affine y d p R l = true -> ~ ll R l.
Theorem iaffine_refutation : forall {atom} y d p R G C,
  overweight_affine y d p (lrules R) (one_sided G C) = true -> ~ ill R G C.
(* the invariants they rest on *)
Theorem weighted_balance : forall {atom} y d p R l, ll R l ->
  weakening R = false -> cut R = false ->
  exists o, outs y d p nodrop l o /\ settled o = true.
Theorem affine_balance : forall {atom} y d p R l, ll R l -> cut R = false ->
  forallb (shaped p) l = true ->
  (1 <= outputs_all p l)%Z /\
  (outputs_all p l = 1%Z -> exists o, outs y d (droppable p) l o /\ settled o = true).
```

`overweight y d p R l` is `negb (weakening R) && negb (cut R) && negb
(existsb (absorbs y d p nodrop) l) && positive (leasts y d p nodrop l)`.
`overweight_affine y d p R l` is `negb (cut R) && forallb (shaped p) l
&& (outputs_all p l =? 1) && negb (existsb (absorbs y d (droppable p))
l) && positive (leasts y d p (droppable p) l)`. `weight_of String.eqb
[("a", 1%Z); …]` is the weight a table gives an atom, zero for one it
does not list. A dead set is written as the classical certificate writes
its assignment, `fun a => existsb (String.eqb a) ["c"; …]`. The statement
is the one the existing refutations state: `ll (atom := string) R [...]`
one-sided, with `classical` or the record literal, and `ill (atom :=
string) intuitionistic [...] C` two-sided.

The first-order ones, `From Linlog Require Import FirstOrderCalculus
FirstOrderIntuitionistic FirstOrderWeights.`, with `(symbol := string)`:

```coq
Theorem FirstOrderWeights.weighted_refutation :
  forall {symbol} (fixed : symbol -> bool) (table : symbol -> list (term symbol) -> Z)
    (weight : symbol -> Z) R l,
  overweight fixed table weight R l = true -> ~ ll R l.
Theorem FirstOrderWeights.iweighted_refutation : forall {symbol} fixed table weight R G C,
  overweight fixed table weight (Refutations.lrules R) (one_sided G C) = true ->
  ~ ill R G C.
```

There the literal `p(ts)` weighs `table p ts` if `fixed p`, else
`weight p`. The test also asks every literal of a fixed predicate closed.

**What the exporter writes, per refutation** (the proof is always
`apply (THEOREM …). vm_compute. reflexivity.`):

- `Refutation::OneSided { atom, copied, .. }`: `weighted_refutation
  (weight_of String.eqb [("ATOM", W)]) (fun a => existsb (String.eqb a)
  []) true RULES`, `W` = `1%Z` when `copied` is the atom, `(-1)%Z` when it is
  the dual. Intuitionistic: `iweighted_refutation … intuitionistic` over
  the two-sided statement. Nothing else is needed: least, most and
  copied are recomputed by `vm_compute`.
- `Refutation::OneSidedSymbol { symbol, copied, .. }` (and, if 51b
  wishes, `UnbalancedSymbol`, or `OneSided`/`Unbalanced` of a sequent
  with a quantifier): `FirstOrderWeights.weighted_refutation FIXED TABLE
  WEIGHT RULES`. Here `FIXED` is the predicates none of whose atoms has
  a bound variable (`fun p => existsb (String.eqb p) [...]`). `WEIGHT`
  is `±1` on the symbol for a grouped predicate. For an atom of a fixed
  predicate, `TABLE` is `fun p ts => if String.eqb p "P" && terms_eqb
  String.eqb ts [TERMS] then ±1%Z else 0%Z` (`TermsCheck.terms_eqb`).
  Today the writer refuses every sequent with a quantifier, and this is
  the first first-order refutation the library certifies.
- `Refutation::StateEquation { atoms, .. }` in linear mode (Mix or not):
  `weighted_refutation (weight_of String.eqb [(NAME, W); …]) (fun a =>
  existsb (String.eqb a) [DEAD…]) P RULES`, or the two-sided form. `P` is
  `true` when the bodies' literals are positive (always in
  intuitionistic mode) and `false` when they are negative (`~a`: the
  goal's literals are negative). `W` is each listed atom's weight,
  negated when `P` is `false`. `DEAD` is
  every atom of the sequent outside the closure: start from the atoms
  the markings hold (the inputs, the literals of the heads' sign outside
  the clauses) and add the head atoms of every clause, used once or
  under `!`, whose body atoms all lie in the set, until nothing changes.
  That is `live`'s closure without the class places, which are always
  in it. The ticket weights (`clauses`) and the `dropped` list are not
  written: the test sums the clauses' own literals, which the ticket
  inequality `y·Cₜ − z ≤ 0` bounds below by `−z`.
- The same in affine mode: `affine_refutation (weight_of …) (fun a =>
  …) P RULES`, or `iaffine_refutation … P affine`, with the weights as
  `certify` gives them (not negative). `P` is `true` when the bodies'
  literals are positive and `false` when they are negative, and the
  weights are not negated in affine mode.
- A worked instance, `tests/Weights.v`'s `state_equation_dead`: the
  program `a, !(a ⊗ a ⊸ b), !(c ⊸ c ⊗ a) ⊢ a ⊗ b`. The initial
  marking holds `a`. The first clause's body `a` is in the set, so `b`
  joins it. The second's body `c` never does, so `c` is dead. The
  weights `a ↦ 1`, `b ↦ 2` make the first clause's effect `2 − 2 = 0`
  and the target `a + b` against the initial `a` raised by 2:
  ```coq
  Lemma state_equation_dead :
    ~ ill (atom := string) intuitionistic
        [ivar "a"; ioc (ilmap (itens (ivar "a") (ivar "a")) (ivar "b"));
         ioc (ilmap (ivar "c") (itens (ivar "c") (ivar "a")))]
        (itens (ivar "a") (ivar "b")).
  Proof.
    apply (iweighted_refutation (weight_of String.eqb [("a", 1%Z); ("b", 2%Z)])
      (fun a => existsb (String.eqb a) ["c"]) true intuitionistic).
    vm_compute. reflexivity.
  Qed.
  ```
  Without the dead `c`, the same weights fail the test
  (`state_equation_needs_dead`): each copy of the second clause lowers
  the count by one. With negative bodies, `state_equation_negative_bodies`
  writes `⊢ a, ?(~a ⊗ (a ⅋ a)), ?((~a ⊗ ~c) ⊗ b), ~b` with the weight
  `b ↦ -1` (the engine's `1`, negated), `c` dead and `P = false`.
- **Why every accepted certificate passes the test** (for 51b's Rust
  check, which may recompute the same bound in place of the Farkas
  inequalities). Read the sequent one-sided, bodies positive. A marking
  token `~a` contributes `−y_a`, the goal `+y·M`, and a clause used
  once `y·pre − y·post = −y·Cₜ`. A clause under `?` contributes `lstar`
  of its own sum, which is `from 0` exactly when `−y·Cₜ ≥ 0`. A clause
  with a dead body atom contributes `never`, so its `?` contributes
  `from 0`, and as a clause used once it refutes outright. The bound is
  `y·(M − M₀) − Σ y·C_c ≥ y·(M − M₀) − Σ z > 0`. In affine mode a
  droppable formula is bounded by `min(0, its sum)`. With `y, z ≥ 0` a
  token's bound is unchanged, a clause's is at least `−z`, and the
  goal's literals are outputs, never dropped.
- **What stays uncertified.** `Refutation::Exhausted` and
  `Refutation::Saturated` have no theorem (`Unsupported::Saturated`
  stays). Every other refutation now has one. 51b's
  `Disproof::check` arms must recompute the invariant from the goal
  alone. For the one-sided count, that is the Rust interval with copies
  of one sign, as `check_unbalanced` does without copies. For the state
  equation, it is the closure, the dead clauses and the Farkas
  inequalities over the program read again, with the arithmetic that
  `certify` uses (`i128`, checked).
- **Pitfalls met.** `split` closes a reflexive equality (eq has one
  constructor), so `repeat split` leaves fewer goals than conjuncts.
  `cbn` unfolds `opar (num 0) o` into a match no lemma rewrites, so the
  proofs use `cbn [app]` and `cbn [absorbs least]`. A section's
  `Variable` used before `induction` is not generalised, so the
  induction hypotheses do not take the side conditions. An existence
  proof for `out (fall A)` needs an instance, so it goes by `size`.

## Session 51b: the certificates and the check

### Outcome

Every refutation linlog states is now checked without the search and
certified in Rocq, except an exhausted search and a saturated set (which
`Disproof::check` verifies but the library has no theorem for), and the
count equation of a sequent with a quantifier. `Disproof::check` accepts
a one-sided count (`OneSided`, `OneSidedSymbol`) and a state equation
(`StateEquation`) by running the Rocq library's own weighted test,
transcribed; the exporter's second kernel writes their certificates
against 51a's theorems, and the first-order counts too; `linlog check`
reads an unprovable outcome and checks its refutation with a proof's
exit statuses; the comparison's driver marks linlog's refutations
checked when `Disproof::check` accepts them. The commits, in the
workspace `../linlog-step51b` on `main`, nothing pushed:

- `umomxnty` "flake.lock: Update": the input `rocq-linlog` at `c63ff4db`
  (0.3.0, unreleased) and `export::rocq::LINLOG` at `0.3.0`, which the
  check `rocq-linlog` asserts equal to the library's version.
- `qzstopmr` "Check one-sided counts and state equations by the
  library's weighted count".
- `mzvrslmy` "Certify one-sided counts and state equations in Rocq".
- `xoxlwupv` "Let check read a disproof and check its refutation".
- `lkzlzlvw` "Bless the behaviour lock: check reads a refutation, an
  exhausted search has its own refusal".
- `tsumszxq` "Credit linlog's checked refutations in the comparison".

### Decisions

- **The check runs the library's test, not the Farkas inequalities
  (departs from the design).** The prompt and 51a's notes read "the
  closure, the dead clauses and the Farkas inequalities over the program
  read again". Instead `search/weighted.rs` transcribes `Weights.v`'s
  `overweight` and `overweight_affine` (`least` with `keep` at every
  subformula, `absorbs` with `reach`, `lstar`, `outputs`, `shaped`), and
  both new arms end in it. The library proves that test sound for any
  weights, dead atoms and sign, so the soundness of an accepted
  refutation rests on one Rocq theorem and on the transcription, which
  the panel read against the Rocq text, instead of on a second argument
  of the Farkas kind written in prose. The Horn reading the check still
  needs (`dead_atoms`: the markings' atoms and the heads of clauses whose
  bodies are reached) only chooses the dead set, which soundness does not
  depend on; both body signs are tried, positive first. The ticket
  weights (`clauses`) and the `dropped` list are the engine's account
  and are not compared: the test needs neither, and 51a showed every
  vector `certify` accepts passes it, which the target sets confirm on
  every state equation they hold.
- **The one-sided count is recomputed, then weighed.** `check_copies`
  recomputes the interval outside the goal's exponentials and the sign
  of the counted literals below them, as `check_balance` does without
  copies: the stated numbers must be the goal's (`Mismatch`), copies of
  both signs are `NotApplicable`, and zero must lie outside on the side
  copies cannot reach (`Holds`). Then the weighted test with `±1` must
  accept, so the decisive step is again the library's test. The search's
  `copied` is the forest's sign, so a goal with no copy of its own passes
  with either; copies of the other sign in the goal are a `Mismatch`.
- **A goal one of whose atoms no firing reaches is refuted whatever the
  weights.** Its literal in the goal is dead and contributes nothing, so
  the test accepts; the library proves that too. Two of my first planted
  "wrong" state equations turned out right for that reason; the planted
  ones in the test are now programs where the weights matter.
- **The certificate's weights are negated with negative bodies in both
  modes.** 51a's notes say the affine weights are "not negated", but its
  own test `state_equation_affine_negative` negates them, and so must the
  certificate: the goal's literals are `~a`, which contribute `−y`. The
  exporter takes the sign, the weights and the dead set from
  `weighted::state_equation`, the check's own reading, so the
  certificate is written from exactly what the check accepted.
- **The first-order counts are certified.** 51a left `UnbalancedSymbol`
  and the counts of a sequent with a quantifier to 51b's wish. They go
  through the same writer: the statement over the first-order calculi
  (`Fo` without framing), and `FirstOrderWeights.weighted_refutation
  FIXED TABLE WEIGHT _`, `FIXED` the predicates with no open atom, a
  closed atom's count a `TABLE` matching its terms, a predicate's count
  the unit on its name in both functions. The count equation of such a
  sequent keeps `Unsupported::Quantifiers`, whose message now says so.
- **An exhausted search is refused by `Unsupported::Exhausted`** before
  the check runs, so every refusal of the writer is an `Unsupported`
  variant with its reason (requirement 2); the behaviour lock's line
  changed, in a commit of its own.
- **`linlog check` checks a refutation in the mode its file states
  (departs from the design: "the flags give the mode").** With no mode
  flag it checks in the file's mode; a flag that names another mode is
  an error naming both (exit 2); the verdict line and the JSON say the
  mode checked. The reason, the supervisor's decision on my first
  version, which checked in the flags' mode as for a proof: a refutation
  is a claim about one mode, and checking it in another answers a
  question nobody asked. Proofs have the same defect (53a's item
  "`linlog check FILE` ignores the file's own mode"), left to 53a; the
  refutations already follow the file. Exit 0 valid, 1 invalid, 3 for an
  exhausted search (no check), a limit or the time limit; the check runs
  on a thread within the time limit, since a saturated set's check has
  no stop. `--format rocq` writes the certificate of a refutation that
  holds, as `prove` does; the other drawn formats write the verdict
  alone.
- **The comparison's driver reads linlog's outcome as JSON.** The driver
  had only the verdict line; the refutation needs the outcome. linlog
  now runs with `--format json`, so the timed run serialises a proof's
  nodes too, small beside the proof check it already includes; a re-run
  untimed would double the time of every unprovable row and check
  another run's refutation. `bench/TOOLS.md` and `bench.md` say that
  comparisons before this one printed the verdict line only.
- **The summary of the guide is unchanged.** `SKILL.md` was 5 bytes
  under its budget; the disproof check is in `reference.md` (section 9,
  with examples, and section 18), as conduct.md says a feature goes
  where the summary has no room.
- **What I read for the form.** The library's `Weights.v`,
  `FirstOrderWeights.v` and its tests, whose certificate form the writer
  follows (`weight_of String.eqb`, `fun a => existsb (String.eqb a)
  [...]`, `vm_compute. reflexivity.`); compiling the twenty snapshots
  against the pinned library found that the first-order prelude needs
  `Bool` for `&&` and both need `ZArith` for `%Z`.

### Options

No option was added: the check and the certificate have no choice to
vary. The web bindings' `refutation_checked` and `certificate` calls
take the new arms and certificates unchanged; the place for quantifiers
(D17) is the first-order writer, which this session used.

### Checks

- [x] The lock: "flake.lock: Update" (`umomxnty`) moved `rocq-linlog`
  from `22f7de20` to `c63ff4db`, and `export::rocq::LINLOG` is `0.3.0`
  in the same commit.
- [x] The certificates: the twenty snapshots `refuted_*.v` compiled
  silently against the pinned library (Rocq 9.1, the store paths of the
  flake's nixpkgs and input), each `Print Assumptions` "Closed under the
  global context"; a planted wrong weight and a planted wrong dead set
  were refused ("Unable to unify true with false"). The flake's check
  `rocq-linlog` compiles them, nine the command writes, and refuses its
  own planted weight; its result is under `nix flake check` below.
- [x] The refusals: `Unsupported::Exhausted`, `Unsupported::Saturated`
  and `Unsupported::Quantifiers` (the count equation of a quantified
  sequent), pinned by `disproof_certificates` (`core/tests/export.rs`).
- [x] The check's tests: `a_wrong_count_with_copies_or_state_equation_is_refused`
  (the search's refutations of both kinds in every mode accepted;
  planted wrong ones of each kind refused with `Mismatch`, `Holds` or
  `NotApplicable`), `a_sum_past_128_bits_is_refused` and
  `the_test_refuses_a_sum_past_128_bits`, all passing; the
  no-default-features build free of warnings.
- [x] `linlog check`: `check_s_verdicts` (`cli/tests/cli.rs`) pins a
  refutation that holds (0), a planted wrong one (1, `invalid_refutation`),
  an exhausted search (3, `uncertified`), a contradicting mode flag (2)
  and an unknown outcome (2); `cargo test -p linlog-cli --test readme`
  passed (7 tests), with the manual's (`commands.md`, `rocq.md`) and the
  guide's (`reference.md` sections 9 and 18) new examples.
- [x] The comparison's driver: `linlog_s_refutations_are_checked`
  (`bench/src/tools.rs`) pins `ok` for a one-sided count and a state
  equation, `failed: …` for a planted change, empty for an exhausted
  search.
- [x] `cargo test --workspace` and `cargo clippy --workspace
  --all-targets -- --deny warnings`, capped: passed.
- [x] The gate (`nix develop -c gate`, capped, cores 13–15): "gate:
  passed", exit 0, the ratchet within its ceilings (its misses note on
  three journeys is not a failure).
- [x] The focused target set `bench/targets/51b.csv` against
  `bench/targets/47.csv`: no verdict differs, and none of the 145 rows
  both decide alike differs in `nodes`, `splits`, `memo_hits`,
  `memo_entries`, `links` or `tests`; every `unprovable` row's
  `checked` is `ok` (33) or empty (57, exhausted), none `failed`.
- [x] The net target set `bench/targets/net-51b.csv` against
  `net-47.csv`: no verdict differs, none of the 64 decided rows differs
  in its counters.
- [x] `linlog-bench run --all-families --timeout 5` (one thread, capped):
  195 rows, no `MISMATCH` in `summary`; its 66 unprovable rows checked
  `ok` (17) or empty (49).
- [x] `nix flake check` after the rebase onto `main` (under the machine
  lock, `--max-jobs 4 --cores 8`): exit 0, "all checks passed!", the
  check `rocq-linlog` compiling 66 certificates, each closed under the
  global context, and `rocq-linlog-library` built; `nix build` of both
  checks exit 0. Its first run failed at `doc-private`: rustdoc resolved
  the module doc's link to `overweight` in the parent module's scope
  (the module has an outer doc line there too); the link now names its
  full path, folded into `qzstopmr`.
- [x] The panel's round, below: two reviewers without a finding, one
  finding that holds, fixed, and one remark answered in the doc comment.

### The panel's round

Three fresh-context reviewers read `Disproof::check`'s new arms, each
given the commits, the claim and the code but not my reasoning, each
trying to refute it one way; a finding counts only with a witness I
reproduce.

- **The argument** (Fable 5.1 at `high`, reading only): derived the test
  from `Weights.v` and `FirstOrderWeights.v` first, then checked
  `weighted.rs` line by line (`keep` at every occurrence, `absorbs` with
  `reach`, `lstar`, `outputs`, `shaped`, the sequent-level conditions),
  the forest against the library's formula list (one-sided, and
  intuitionistic as `one_sided G C`), goals off the roots with repeated
  members, the mode mapping (no cut field in `Mode`, weakening exactly in
  affine mode, Mix allowed by both theorems), `check_copies` and
  `state_equation`, and that the exporter writes the very weights, dead
  set and sign the check used. It checked the first-order and the
  `Unbalanced`/`UnbalancedSymbol` certificates beyond its scope too.
  **No finding**: the claim holds.
- **The integers and limits** (Sonnet 5.5 at `high`, core 6 then 12):
  drove every bound with inputs (10⁶ literals weighing `i64::MIN` and
  `i64::MAX` in `⊗` and `⅋` towers, a member repeated 2·10⁶ times, a
  count past `i32`, repeated body literals for `dead_atoms`), and ran a
  differential fuzz of 1 516 416 checks on 3 000 random sequents in
  three modes (every `OneSided` in a range and 40 random `StateEquation`s
  per sequent), 29 214 accepted, none for a sequent the search proves, no
  panic. Two findings:
  1. *The committed test reaches the 128-bit refusal through `Low::sum`
     alone, not through `overweight`.* **Holds; fixed**: the test
     `the_test_refuses_a_sum_past_128_bits` drives `overweight` there
     with weights past any an `i64` gives, at both sites (a `⊗` and the
     members' total). No real input gets there (±2⁹⁵ per member), as
     its doc comment says.
  2. *The check's time is linear in the goal's occurrences counted with
     repeats.* Not a correctness finding; the doc comment of
     `Disproof::check` now says so.
- **Counterexamples** (Opus 5.5 at `high`, cores 10–11): against about
  10 600 (sequent, mode) pairs that the test-only reference provers
  (`reference.rs`, `reference_fo.rs`) prove, from random classical
  sequents and Horn programs (either body sign, `1`, `⊥`, several goal
  members, curried clauses), the generator's provable sequents for every
  rule set and their mutants, first-order sequents and the shapes the
  brief named, in classical linear, Mix, affine and affine-with-Mix and
  intuitionistic linear and affine modes, it tried about 12.7 million
  crafted refutations: every weight vector in [−2, 2] on up to four
  atoms with every dead set and both signs directly on `overweight`,
  the same as `StateEquation`s through `Disproof::check`, every
  `OneSided` with `least ≤ most` in [−6, 6] and both signs, the
  first-order counts, and the search's refutations moved to the other
  modes (886 still accepted, none where the reference proves the
  sequent). **No finding**: no refutation of a provable sequent was
  accepted, and all 1 471 refutations of the three kinds the search gave
  passed the check. Its limits, as it states them: the families at small
  sizes gave none of the three kinds, so the second claim rests on
  generated sequents (the target sets above add theirs: every
  `unprovable` row's `checked` is `ok` or empty, none `failed`), the
  weights were exhaustive within [−2, 2] only, and it did not attack the
  JSON reader's goal list (the integers reviewer did, with repeated and
  nested members).

No finding of the panel needed a witness reproduced: neither the
argument nor the counterexamples found one, and the integers reviewer's
holding finding is a test that was missing, which the new test now
pins.

### What the next sessions must know

- **What is certified now.** Every refutation kind but `Exhausted` is
  checked by `Disproof::check` without the search, and every kind but
  `Exhausted` and `Saturated` has a Rocq certificate for linlog's
  library, with one exception: the count equation of a sequent with a
  quantifier (`Unsupported::Quantifiers`). The one-sided counts and the
  state equation are the library's weighted count (`Weights.v`), the
  counts of a sequent with a quantifier its first-order weighted count
  (`FirstOrderWeights.v`).
- **What stays uncertified.** `Refutation::Exhausted` (no invariant;
  `RefutationFault::Uncertified`, `Unsupported::Exhausted`); a saturated
  set has a check but no theorem (`Unsupported::Saturated`); the count
  equation with quantifiers has no first-order theorem; the affine
  refutations of the Horn engine's backward coverability are
  `Exhausted` (status.md plans them for 0.4.0).
- **The weighted test is transcribed, so it moves with the library.**
  `search/weighted.rs` must compute the bound of `Weights.v` (and of
  `FirstOrderWeights.v`), never more generously; a change of the
  library's test is a change there in the same lock bump, and the
  rules file `core-search.md` lists the points where a transcription can
  slip (`keep` at every subformula, `!A` never absorbing, `reach`).
- **Every refutation certificate now needs the library at 0.3.0**: the
  propositional prelude imports `Weights`. The manual's `rocq.md` still
  says that linlog 0.2.0's certificates compile against rocq-linlog
  0.2.0, which is true of the release; step 53 moves that sentence, the
  pin's tag (`v0.3.0`) and the opam line with the release.
- **The state equation's certificate is the check's reading.** The
  exporter takes the bodies' sign, the negated weights and the dead set
  from `weighted::state_equation`; a new kind of Horn program the engine
  learns to refute must keep passing that reading, or the check refuses
  its refutation (the target sets' `checked` column says `failed:`).
- **A goal with an atom no firing reaches is refuted by the dead atom
  alone**, whatever the weights; a test that plants wrong weights must
  use a program whose atoms are all reachable.
- **The comparison's driver reads JSON** (`--format json`), so the next
  comparison's linlog rows serialise their proofs inside the timed run
  and their refutations are checked after it; earlier comparisons'
  `checked` columns are empty for every `unprovable`.
- **`linlog check` checks a refutation in the mode its file states**
  (a mode flag naming another is an error, exit 2), while a proof is
  still checked in the flags' mode; 53a's item "`linlog check FILE`
  ignores the file's own mode" is the proof path alone now.
