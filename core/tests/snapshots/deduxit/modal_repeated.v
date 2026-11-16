From Stdlib Require Import List String.
From Deduxit.Modal Require Import Check.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : valid (atom := string) {| serial := false; reflexive := false; symmetric := false; transitive := false; euclidean := false |} [var "q"; var "p"; var "q"] [var "p"; var "r"; var "p"].
Proof.
  apply (modal_soundness String.eqb String.eqb_spec {| serial := false; reflexive := false; symmetric := false; transitive := false; euclidean := false |} "w0"
    (Ax "w0" (var "p"))).
  vm_compute. reflexivity.
Qed.
