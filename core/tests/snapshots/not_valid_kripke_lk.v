Lemma certificate : ~ (forall a b : Prop, (a -> b) -> b -> a).
Proof.
  intro H. specialize (H False True).
  pose proof (H ltac:(intro x2; exact I)) as x0; pose proof (x0 ltac:(exact I)) as x1; exact x1.
Qed.
