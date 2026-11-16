# The certificate of a labelled modal derivation (66a's design, for 66b and 66c)

Written by session 66a on 2026-11-16. It says what the exporter writes
for a derivation in a labelled calculus `G3S` (`modal::calculus(system)`)
and what the Rocq library must prove so that the file compiles exactly
when the derivation is sound. 66b proves the library side; 66c (or 66b,
as planned) writes the exporter and the flake check. No Rocq code is
written here.

## 1. What a certificate states

- **The claim is semantic, not a calculus's derivability.** The lemma of
  a certificate states the sequent valid over the system's frame class,
  so that the trusted base is the definition of forcing and of the frame
  conditions alone, never the rule table:

  ```coq
  Lemma certificate :
    valid (atom := string) {| serial := true; reflexive := true;
      symmetric := false; transitive := true; euclidean := false |}
      [box (var "p")] [var "p"].
  ```

  `valid C Γ Δ`: for every type `W`, relation `R`, valuation
  `V : W -> atom -> Prop` with `frame C R` and every world `w`, if `w`
  forces every formula of `Γ` then it forces one of `Δ` (local
  consequence, the reading of `core-modal.md`).
- **The system is the closed set of conditions** deduxit's `System` holds
  (`System::conditions()`), written as the record, so the statement's
  class is the system's class whatever axioms it was named by (`KB5` and
  `KB4` write the same record). S4's record marks `serial` too, since
  reflexivity gives it and `G3S4` has `Ser`: a record that left it out
  would refuse the `Ser` steps the generic checker accepts (the review
  of 66a). `frame C R` is the conjunction of the
  conditions the record marks, each in its first-order form (serial
  `forall x, exists y, R x y`, reflexive, symmetric, transitive,
  euclidean `R x y -> R x z -> R y z`).
- **A root that is not `w0 : Γ ⊢ w0 : Δ`** (a session's derivation may
  end anywhere, `w0 : □p, w0 R w1 ⊢ w1 : p` say) is stated as labelled
  validity of the root itself: `lvalid C L Rt` with `L` and `Rt` lists
  of items, for every frame of the class, valuation and assignment of
  the labels to worlds. `valid` is the corollary for roots at one label
  with no relational atom (`local`, 66b's lemma), and the exporter
  writes `valid` exactly there, for whatever label the root is at.

## 2. The data the library defines

- `formula (atom)`: `var`, `top`, `bot`, `neg`, `wedge`, `vee`, `imp`,
  `equiv`, `box`, `dia`: one constructor per node of deduxit's modal
  arena (`modal::Node`), so the exporter needs no rewriting and the
  forcing clauses are one line each. Alternative: `top`, `neg`,
  `equiv` and `dia` as definitions over five constructors, as
  `Deduxit.Intuitionistic` does; then `L◇`/`R◇` and `L↔`/`R↔` are
  derived rules in the checker, and a mismatch between deduxit's rules
  and the definitions' unfolding becomes the checker's problem. The
  first is recommended: the term mirrors the rule file one to one.
- `label := string` (the labels as deduxit writes them, `w0`, `y1`),
  compared by the `String.eqb` the certificate passes, as atoms are.
- `item := lab (x : label) (A : formula) | rel (x y : label)`, a sequent
  two lists of items. Item truth under an assignment `ρ : label -> W`:
  `lab x A` is `forces (ρ x) A`, `rel x y` is `R (ρ x) (ρ y)`.
- `proof`, one constructor per rule of the rule file, carrying what
  the checker cannot find without search, and nothing it can compute:

  | rule | constructor | carries |
  |---|---|---|
  | `ax` | `Ax x A` | the label and the formula |
  | `L⊥`, `R⊤` | `LBot x`, `RTop x` | the label |
  | `L¬`, `R¬`, `L∧`, `R∨`, `R→` | `LNeg x A p` … | the principal's parts, the premise |
  | `R∧`, `L∨`, `L→`, `L↔`, `R↔` | `RAnd x A B p q` … | the parts, both premises |
  | `L□`, `R◇` | `LBox x y A p` | the edge used, the formula |
  | `R□`, `L◇` | `RBox x y A p` | the new label `y` |
  | `Ref` | `Ref x p` | the label |
  | `Ser` | `Ser x y p` | the new label |
  | `Sym` | `Sym x y p` | the edge |
  | `Trans`, `Eucl` | `Trans x y z p` | both edges |

  A premise is the conclusion with the rule's new items added (every
  rule keeps its principal formula and its contexts are the whole
  zones, `modal::calculus`'s doc), so the term needs no sequent per
  node.

## 3. The checker and the theorems (66b)

- `mcheck (eqb) (C : conditions) (p : proof) (L Rt : list item) : bool`,
  structural recursion on `p`, top-down from the given conclusion: each
  constructor tests its principal items' membership (`existsb` with the
  item equality built on `eqb`), its side condition (a new label occurs
  in no item of `L` and `Rt`; a frame rule only where `C` marks its
  condition), and checks its premises on `L` and `Rt` extended by the
  new items. `Ax x A` asks `lab x A` on both sides; `LBot` and `RTop`
  ask their item.
- `g3 C : list item -> list item -> Prop`, the calculus as an inductive
  with the same rules over lists (membership, not context splitting), so
  that the checker's theorem and the semantics' meet in one place.
- Theorems, in the order 66b proves them:
  1. `mcheck_sound : mcheck eqb C p L Rt = true -> g3 C L Rt`.
  2. `g3_sound : g3 C L Rt -> lvalid C L Rt`, by induction: each rule
     preserves labelled validity on frames of the class. The two rules
     with a new label need the assignment changed at `y` only (the
     freshness side condition is what makes the premise's assumption
     usable), and `Ser` needs the serial condition's witness.
  3. `local : lvalid C [lab x A1; …] [lab x B1; …] -> valid C [A1; …] [B1; …]`.
  4. `modal_soundness eqb eqb_spec C x p Γ Δ : mcheck eqb C p (map (lab x) Γ) (map (lab x) Δ) = true -> valid C Γ Δ`,
     the theorem a certificate applies, the root's label `x` a
     parameter (a session's root may be at `u`, not `w0`), and
     `labelled_soundness` for `lvalid` roots.
- **Classical reasoning is needed** with these definitions (the review
  of 66a): G3K derives `⊢ w0 : p ∨ ¬p`, and `valid` over `V : W -> atom
  -> Prop` with one world is then `P ∨ ¬P` for every proposition `P`.
  `R→`, `R¬` and `R□` are the cases that need it. Two ways, 66b
  chooses:
  - `Classical_Prop.classic` assumed and named: the soundness theorem's
    `Print Assumptions` lists it alone, the certificate imports
    `Classical_Prop`, and the flake's `check-certificate` already
    accepts exactly that axiom in a file that imports it (the classical
    certificates over `Prop`). Simplest, and the claim stays about
    `Prop`-valued models.
  - Forcing made decidable (`V : W -> atom -> bool` over a finite or
    decidable frame) or read negatively (every clause of `forces` stable
    under double negation), which keeps the assumptions closed but
    weakens or reshapes the statement a reader checks.
  Recommended: the first, stated in the lemma's comment, as the
  classical certificates do. The library's own check
  (`rocq-deduxit-library`) asks every theorem to be closed under the
  global context, so 66b lets `classic` alone through for the modal
  theorems there, by name, as `check-certificate` does.

## 4. What the exporter writes (66c)

```coq
From Stdlib Require Import List String Classical_Prop.
From Deduxit.Modal Require Import Check.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : valid (atom := string) {| … |} [box (var "p")] [var "p"].
Proof.
  apply (modal_soundness String.eqb String.eqb_spec {| … |} "w0"
    (Ref "w0" (LBox "w0" "w0" (var "p") (Ax "w0" (var "p"))))).
  vm_compute. reflexivity.
Qed.
```

- **Where it is called**: `export::rocq::write` takes a `Drawable::Rules`
  today and refuses it (`Unsupported::Calculus`); a derivation whose
  calculus is a `G3…` of `modal::calculus` is written instead (the
  calculus recognised by its name and by `form` being `modal::calculus`'s
  for that system, so a user's rule file named `G3S4` is still refused).
  The command's `prove --system S --format rocq` (a left-out line
  today), `check --calculus G3S --format rocq` and a session's `proof`
  reach it.
- **The term**: a walk from the root over the derivation's nodes, each
  rule name to its constructor, the principal item read from the node's
  positions in the rule compiler's order (`core-modal.md`, "The order of
  the members a node gives by position"), the new label from the
  premise's position member. Formulas are written from the rules terms
  (`rules::Term`, connectives by name: `⊤ ⊥ ¬ ∧ ∨ → ↔ □ ◇`), atoms as
  `var "p"` with `"` doubled, labels as strings. A node shared by two
  parents (a derivation file may be a DAG) is written once under a
  `let`, so the term stays as large as the derivation, never its
  unfolding; the estimate before writing is against
  `limits.derivation_bytes`, as the other certificates are.
- **What is refused**: a derivation that the generic checker refuses
  (checked first, as the other writers do), one with an open goal or a
  hypothesis (`Unsupported::Open`), a calculus that is not one of
  `modal::calculus`'s (`Unsupported::Calculus`, as today).
- **Not valid**: a modal countermodel's certificate is the same shape as
  the Kripke one (`kripke_refutation`): a finite model `{| arrows :=
  …; atoms := … |}`, `frame` checked by computation on the arrows as
  written (deduxit closes nothing, `core-modal.md`), forcing computed,
  and `modal_refutation … w` proving `~ valid C Γ Δ`. It is cheap
  beside the soundness theorem and closes the suite's certificates;
  66b decides whether it fits its sessions.
- **The flake check**: the certificates of `core/tests/snapshots/deduxit/`
  (modal ones added there, `modal_*.v`) and a few the command writes are
  compiled against the pinned library by the `rocq-deduxit` check, and a
  planted fault (a wrong label at `R□`, a frame rule the system lacks) is
  refused.

## 5. In `rocq-deduxit`: the module `Deduxit.Modal`

**The author chose `rocq-deduxit` (2026-11-16).** The library's
`_RocqProject` maps `-Q theories Deduxit`, so files under
`theories/Modal/` are `Deduxit.Modal.*`, beside `Deduxit.Linear` and
`Deduxit.Intuitionistic`, each line added to `_RocqProject` in
dependency order. Proposed files and names (66b may rename inside a
file; the exporter needs only what `Check` exports, listed last):

| file | module | holds |
|---|---|---|
| `theories/Modal/Formulas.v` | `Deduxit.Modal.Formulas` | `formula (atom : Type)`: `var`, `top`, `bot`, `neg`, `wedge`, `vee`, `imp`, `equiv`, `box`, `dia`; `label := string`; `item atom`: `lab (x : label) (A : formula atom)`, `rel (x y : label)`; the item equality from an `atom_eqb` and `String.eqb` |
| `theories/Modal/Kripke.v` | `Deduxit.Modal.Kripke` | `conditions`, a record of five `bool` fields `serial`, `reflexive`, `symmetric`, `transitive`, `euclidean`; `frame C R`; `forces R V w A`; `holds R V ρ i` for an item under `ρ : label -> W`; `valid C Γ Δ`, `lvalid C L Rt` |
| `theories/Modal/Calculus.v` | `Deduxit.Modal.Calculus` | `g3 C : list item -> list item -> Prop`, the rules of section 3 over lists by membership |
| `theories/Modal/Proofs.v` | `Deduxit.Modal.Proofs` | `proof atom`, the constructors of section 2's table: `Ax`, `LBot`, `RTop`, `LNeg`, `RNeg`, `LAnd`, `RAnd`, `LOr`, `ROr`, `LImp`, `RImp`, `LIff`, `RIff`, `LBox`, `RBox`, `LDia`, `RDia`, `Ref`, `Ser`, `Sym`, `Trans`, `Eucl` |
| `theories/Modal/Soundness.v` | `Deduxit.Modal.Soundness` | `g3_sound`, `local` (section 3) |
| `theories/Modal/Check.v` | `Deduxit.Modal.Check` | `mcheck`, `mcheck_sound`, `modal_soundness`, `labelled_soundness`; it `Require Export`s `Formulas`, `Kripke` and `Proofs`, so a certificate imports it alone |

The constructor names are short because a certificate imports
`Deduxit.Modal.Check` and nothing of `Deduxit.Linear`, whose `Ax` they
would otherwise meet; a test file that imports both qualifies them.

**What a certificate imports**: `From Stdlib Require Import List String
Classical_Prop.` (the last only if 66b takes `classic`, section 3), then
`From Deduxit.Modal Require Import Check.`, `Import ListNotations.`,
`Open Scope string_scope.`, one constant in the exporter
(`rocq::DEDUXIT_MODAL_PRELUDE`, as `DEDUXIT_KRIPKE_PRELUDE` is). The
statement names `valid (atom := string)` and the record `{| serial :=
…; … |}`; the proof `apply (modal_soundness String.eqb String.eqb_spec
C "w0" P). vm_compute. reflexivity.`, as in section 4.

**How the flake compiles them** (`modules/rocq-deduxit.nix`, the check
`rocq-deduxit`, against the pinned input; nothing new to wire, since
`ROCQPATH` already holds the library's `user-contrib`):
- The exporter's snapshots `core/tests/snapshots/deduxit/modal_*.v`
  (pinned by `core/tests/export.rs` as the other kernels' are) are
  copied in with every `.v` of that directory, as today.
- Certificates the command writes are added to the script: `prove
  --system S4 --format rocq --standalone` of `[]p -> [][]p` (the search's
  derivation, `Trans` in it), one of KD45 (`Ser`, `Eucl`), and a
  session's finished derivation (`interact --calculus G3T` with `proof`
  to a file, then `check --calculus G3T --format rocq`), so the three
  ways a labelled derivation reaches the writer are compiled.
- Each is compiled with `rocq compile`, silence asked, as now.
- `Print Assumptions certificate` asks "Closed under the global context"
  of every file today. With `classic` taken, the loop accepts for
  `modal_*.v` and `cli_modal_*.v` exactly the one axiom
  `Classical_Prop.classic`, as `check-certificate` does for a file that
  imports `Classical_Prop`; the library's own check
  (`rocq-deduxit-library`) accepts it by name for the modal theorems
  alone.
- Two planted faults must be refused: a certificate whose `R□` names a
  label of its conclusion (by `sed`), and a record with `transitive :=
  false` under a derivation that uses `Trans`.
- `rocq::DEDUXIT` names the library's version for every certificate,
  asserted against the pinned opam file at evaluation as now, so the
  exporter's modal certificates move with the pin.

## 6. Sources

- Negri, "Proof analysis in modal logic", J. Philos. Logic 34 (2005),
  507–544, Table II and p. 515: the rules and the frame rules. Their
  soundness for Kripke semantics is by induction on the derivation,
  reading a labelled sequent under an assignment of the labels to
  worlds; 66b reads her statement of it (this session did not, and 65's
  sources record no theorem number for it).
- `modal::calculus`'s doc comment: the departures from Table II (kept
  principal formulas, the axiom on any formula, `L¬`, `R¬`, `R⊤`, `L↔`,
  `R↔`, no initial sequent on a relational atom); each is sound by the
  same induction, and the kept principal formula is what lets the
  checker add items without matching contexts.
- `rocq-deduxit` at the pinned commit: `Deduxit.Linear.IntuitionisticCheck`
  (`soundness_ill`, the shape `apply … vm_compute. reflexivity.`) and
  `Deduxit.Intuitionistic.KripkeCheck` (`kripke_refutation`, a finite
  model checked by computation).
