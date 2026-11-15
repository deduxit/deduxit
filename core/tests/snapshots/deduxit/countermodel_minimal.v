From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import KripkeCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lj (atom := string) minimal [var "a"; neg (var "a")] (var "b").
Proof.
  apply (kripke_refutation String.eqb String.eqb_spec minimal
    {| later := [[]];
       atoms := [["a"]];
       absurd := [0] |} 0).
  vm_compute. reflexivity.
Qed.
