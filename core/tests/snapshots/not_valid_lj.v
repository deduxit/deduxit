Lemma certificate : ~ (forall a b : Prop, (a \/ (b -> a))).
Proof.
  intro H. specialize (H False True).
  destruct H as [x0 | x0]; [exact x0 | pose proof (x0 ltac:(exact I)) as x1; exact x1].
Qed.
