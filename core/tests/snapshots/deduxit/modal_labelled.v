From Stdlib Require Import List String.
From Deduxit.Modal Require Import Check.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : lvalid (atom := string) {| serial := false; reflexive := false; symmetric := false; transitive := false; euclidean := false |} [lab "w0" (box (var "p")); rel "w0" "w1"] [lab "w1" (var "p")].
Proof.
  apply (labelled_soundness String.eqb String.eqb_spec {| serial := false; reflexive := false; symmetric := false; transitive := false; euclidean := false |}
    (LBox "w0" "w1" (var "p") (Ax "w1" (var "p")))).
  vm_compute. reflexivity.
Qed.
