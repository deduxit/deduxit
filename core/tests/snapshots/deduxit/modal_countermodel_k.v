From Stdlib Require Import List String.
From Deduxit.Modal Require Import Countermodel.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ valid (atom := string) {| serial := false; reflexive := false; symmetric := false; transitive := false; euclidean := false |} [box (var "p")] [var "p"].
Proof.
  apply (modal_refutation String.eqb
    {| arrows := [[]];
       atoms := [[]] |}
    {| serial := false; reflexive := false; symmetric := false; transitive := false; euclidean := false |} 0).
  vm_compute. reflexivity.
Qed.
