From Stdlib Require Import List String.
From Deduxit.Modal Require Import Check.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : valid (atom := string) {| serial := false; reflexive := false; symmetric := false; transitive := false; euclidean := true |} [dia (var "p")] [box (dia (var "p"))].
Proof.
  apply (modal_soundness String.eqb String.eqb_spec {| serial := false; reflexive := false; symmetric := false; transitive := false; euclidean := true |} "w0"
    (LDia "w0" "w1" (var "p") (RBox "w0" "w2" (dia (var "p")) (Eucl "w0" "w2" "w1" (RDia "w2" "w1" (var "p") (Ax "w1" (var "p"))))))).
  vm_compute. reflexivity.
Qed.
