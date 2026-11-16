From Stdlib Require Import List String.
From Deduxit.Modal Require Import Check.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : valid (atom := string) {| serial := true; reflexive := false; symmetric := false; transitive := false; euclidean := false |} [box (var "p")] [dia (var "p")].
Proof.
  apply (modal_soundness String.eqb String.eqb_spec {| serial := true; reflexive := false; symmetric := false; transitive := false; euclidean := false |} "w0"
    (Ser "w0" "w1" (LBox "w0" "w1" (var "p") (RDia "w0" "w1" (var "p") (Ax "w1" (var "p")))))).
  vm_compute. reflexivity.
Qed.
