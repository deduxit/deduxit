From Stdlib Require Import List String.
From Deduxit.Linear Require Calculus.
From Deduxit.Linear Require Import FirstOrderSoundness.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ll (symbol := string) Calculus.classical [fall (fex (covar "p" [bound 1; bound 0])); fall (fex (var "p" [bound 0; bound 1]))].
Proof.
  apply (soundness_classical String.eqb String.eqb_spec
    (Forall (fex (covar "p" [bound 1; bound 0])) 0 (Forall (fex (var "p" [bound 0; bound 1])) 1 (Exists (covar "p" [eigen 0; bound 0]) (eigen 1) (Exists (var "p" [bound 0; eigen 1]) (eigen 0) (Ax "p" [eigen 0; eigen 1])))))).
  vm_compute. reflexivity.
Qed.
