From Stdlib Require Import List String.
From Deduxit.Modal Require Import Countermodel.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ valid (atom := string) {| serial := true; reflexive := true; symmetric := false; transitive := true; euclidean := false |} [dia (box (var "p"))] [box (dia (var "p"))].
Proof.
  apply (modal_refutation String.eqb
    {| arrows := [[0; 1; 2]; [1]; [2]];
       atoms := [[]; ["p"]; []] |}
    {| serial := true; reflexive := true; symmetric := false; transitive := true; euclidean := false |} 0).
  vm_compute. reflexivity.
Qed.
