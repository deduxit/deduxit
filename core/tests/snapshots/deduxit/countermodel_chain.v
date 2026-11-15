From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import KripkeCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lj (atom := string) intuitionistic [top; equiv (var "a") (var "b"); imp bot (var "c")] (imp (neg (neg (var "a"))) (var "a")).
Proof.
  apply (kripke_refutation String.eqb String.eqb_spec intuitionistic
    {| later := [[1; 2]; [2]; []];
       atoms := [[]; []; ["a"; "b"]];
       absurd := [] |} 0).
  vm_compute. reflexivity.
Qed.
