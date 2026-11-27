From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import FirstOrderCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lk (symbol := string) true [var "=" [(tapp "a" []); (tapp "a" [])]] [var "=" [(tapp "a" []); (tapp "b" [])]].
Proof.
  apply (fo_classical_refutation String.eqb true
    {| size := 2;
       functions := [("a", 0, [0]); ("b", 0, [1])];
       predicates := [("=", [0; 0]); ("=", [1; 1])] |}).
  vm_compute. reflexivity.
Qed.
