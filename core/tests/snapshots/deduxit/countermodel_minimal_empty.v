From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import KripkeCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lj (atom := string) minimal [vee (var "a") (var "b")] bot.
Proof.
  apply (kripke_refutation String.eqb String.eqb_spec minimal
    {| later := [[]];
       atoms := [["a"]];
       absurd := [] |} 0).
  vm_compute. reflexivity.
Qed.
