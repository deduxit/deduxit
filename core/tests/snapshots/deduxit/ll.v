From Stdlib Require Import List String.
From Deduxit.Linear Require Import Soundness.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ll (atom := string) classical [wn (aplus (covar "A") (covar "B")); tens (oc (var "A")) (oc (var "B"))].
Proof.
  apply (soundness_classical String.eqb String.eqb_spec
    (let f'1 := aplus (covar "A") (covar "B") in
     (Quest f'1 (Tensor (oc (var "A")) (oc (var "B")) (Bang (var "A") (Copy f'1 (Plus (covar "A") (covar "B") Left (Ax "A")))) (Bang (var "B") (Copy f'1 (Plus (covar "A") (covar "B") Right (Ax "B")))))))).
  vm_compute. reflexivity.
Qed.
