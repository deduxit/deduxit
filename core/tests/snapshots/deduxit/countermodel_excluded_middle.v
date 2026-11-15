From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import KripkeCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lj (atom := string) intuitionistic [] (vee (var "a") (neg (var "a"))).
Proof.
  apply (kripke_refutation String.eqb String.eqb_spec intuitionistic
    {| later := [[1]; []];
       atoms := [[]; ["a"]];
       absurd := [] |} 0).
  vm_compute. reflexivity.
Qed.
