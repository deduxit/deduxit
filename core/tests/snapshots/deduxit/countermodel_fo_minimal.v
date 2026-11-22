From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import FirstOrderCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lj (symbol := string) {| Calculus.explosion := false; Calculus.cut := true |} [] (neg (neg (imp bot (var "p" [(tapp "a" [])])))).
Proof.
  apply (fo_kripke_refutation String.eqb String.eqb_spec _
    {| later := [[1]; []];
       sizes := [1; 1];
       tables := [[("a", 0, [0])]; [("a", 0, [0])]];
       facts := [[]; []];
       absurd := [1] |} 0).
  vm_compute. reflexivity.
Qed.
