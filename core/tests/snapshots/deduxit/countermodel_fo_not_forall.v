From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import FirstOrderCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lj (symbol := string) {| Calculus.explosion := true; Calculus.cut := true |} [neg (fall (var "p" [bound 0]))] (fex (neg (var "p" [bound 0]))).
Proof.
  apply (fo_kripke_refutation String.eqb String.eqb_spec _
    {| later := [[1]; []];
       sizes := [1; 2];
       tables := [[]; []];
       facts := [[]; [("p", [0])]];
       absurd := [] |} 0).
  vm_compute. reflexivity.
Qed.
