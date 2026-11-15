From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import KripkeCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lj (atom := string) intuitionistic [imp (var "a") (vee (var "b") (var "c"))] (vee (imp (var "a") (var "b")) (imp (var "a") (var "c"))).
Proof.
  apply (kripke_refutation String.eqb String.eqb_spec intuitionistic
    {| later := [[1; 2]; []; []];
       atoms := [[]; ["a"; "b"; "d"]; ["a"; "c"]];
       absurd := [] |} 0).
  vm_compute. reflexivity.
Qed.
