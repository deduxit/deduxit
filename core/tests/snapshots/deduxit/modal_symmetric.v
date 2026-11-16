From Stdlib Require Import List String.
From Deduxit.Modal Require Import Check.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : valid (atom := string) {| serial := true; reflexive := true; symmetric := true; transitive := false; euclidean := false |} [var "p"] [box (dia (var "p"))].
Proof.
  apply (modal_soundness String.eqb String.eqb_spec {| serial := true; reflexive := true; symmetric := true; transitive := false; euclidean := false |} "w0"
    (RBox "w0" "w1" (dia (var "p")) (Sym "w0" "w1" (RDia "w1" "w0" (var "p") (Ax "w0" (var "p")))))).
  vm_compute. reflexivity.
Qed.
