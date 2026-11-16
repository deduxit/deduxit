From Stdlib Require Import List String.
From Deduxit.Modal Require Import Countermodel.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ valid (atom := string) {| serial := true; reflexive := true; symmetric := false; transitive := false; euclidean := false |} [] [imp (box (var "p")) (box (box (var "p")))].
Proof.
  apply (modal_refutation String.eqb
    {| arrows := [[0; 1]; [1; 2]; [2]];
       atoms := [["p"]; ["p"]; []] |}
    {| serial := true; reflexive := true; symmetric := false; transitive := false; euclidean := false |} 0).
  vm_compute. reflexivity.
Qed.
