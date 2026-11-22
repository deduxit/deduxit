From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import FirstOrderCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lk (symbol := string) true [fex (var "p" [bound 0])] [fall (var "p" [bound 0])].
Proof.
  apply (fo_classical_refutation String.eqb true
    {| size := 2;
       functions := [];
       predicates := [("p", [0])] |}).
  vm_compute. reflexivity.
Qed.
