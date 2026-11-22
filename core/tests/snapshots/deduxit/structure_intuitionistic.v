From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import FirstOrderCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lj (symbol := string) {| Calculus.explosion := true; Calculus.cut := true |} [fall (vee (var "p" [bound 0]) (var "q" [bound 0]))] (fex (var "p" [bound 0])).
Proof.
  apply (fo_kripke_refutation String.eqb String.eqb_spec _
    {| later := [[]];
       sizes := [1];
       tables := [[]];
       facts := [[("q", [0])]];
       absurd := [] |} 0).
  vm_compute. reflexivity.
Qed.
