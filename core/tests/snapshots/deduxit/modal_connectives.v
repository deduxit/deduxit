From Stdlib Require Import List String.
From Deduxit.Modal Require Import Check.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : valid (atom := string) {| serial := false; reflexive := false; symmetric := false; transitive := false; euclidean := false |} [box (equiv (var "p") (var "q")); neg (dia (neg (var "r"))); top] [wedge (imp (box (var "p")) (box (var "q"))) (vee (var "r") bot); box (var "r")].
Proof.
  apply (modal_soundness String.eqb String.eqb_spec {| serial := false; reflexive := false; symmetric := false; transitive := false; euclidean := false |} "w0"
    (LNeg "w0" (dia (neg (var "r"))) (RBox "w0" "w1" (var "r") (RDia "w0" "w1" (neg (var "r")) (RNeg "w1" (var "r") (Ax "w1" (var "r"))))))).
  vm_compute. reflexivity.
Qed.
