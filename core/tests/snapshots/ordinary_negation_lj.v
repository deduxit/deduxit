Lemma certificate : forall a : Prop, a -> (~ ~ a).
Proof.
  exact (fun (a : Prop) (h'1 : a) => (fun h'2 : (~ a) => (h'2 (h'1)))).
Qed.

