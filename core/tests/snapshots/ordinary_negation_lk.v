From Stdlib Require Import Classical_Prop.

Lemma certificate : forall a : Prop, a -> (~ ~ a).
Proof.
  exact (fun (a : Prop) (h'1 : a) => NNPP (~ ~ a) (fun k'2 : ~ (~ ~ a) => (k'2 (fun h'3 : (~ a) => (let k'4 : ~ a := h'3 in (k'4 h'1)))))).
Qed.

