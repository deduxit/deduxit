From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import FirstOrderCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lk (symbol := string) true [var "p" [(tapp "a" [])]] [var "p" [(tapp "f" [(tapp "a" [])])]; var "r" []].
Proof.
  apply (fo_classical_refutation String.eqb true
    {| size := 2;
       functions := [("a", 0, [0]); ("f", 1, [1; 0])];
       predicates := [("p", [0])] |}).
  vm_compute. reflexivity.
Qed.
