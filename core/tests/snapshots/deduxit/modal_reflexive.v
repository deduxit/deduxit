From Stdlib Require Import List String.
From Deduxit.Modal Require Import Check.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : valid (atom := string) {| serial := true; reflexive := true; symmetric := false; transitive := false; euclidean := false |} [box (var "p")] [var "p"].
Proof.
  apply (modal_soundness String.eqb String.eqb_spec {| serial := true; reflexive := true; symmetric := false; transitive := false; euclidean := false |} "w0"
    (Ref "w0" (LBox "w0" "w0" (var "p") (Ax "w0" (var "p"))))).
  vm_compute. reflexivity.
Qed.
