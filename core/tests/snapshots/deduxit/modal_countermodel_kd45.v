From Stdlib Require Import List String.
From Deduxit.Modal Require Import Countermodel.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ valid (atom := string) {| serial := true; reflexive := false; symmetric := false; transitive := true; euclidean := true |} [box (var "p")] [var "p"].
Proof.
  apply (modal_refutation String.eqb
    {| arrows := [[1]; [1]];
       atoms := [[]; ["p"]] |}
    {| serial := true; reflexive := false; symmetric := false; transitive := true; euclidean := true |} 0).
  vm_compute. reflexivity.
Qed.
