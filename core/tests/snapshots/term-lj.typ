#import "@preview/curryst:0.6.0": prooftree, rule
#set page(width: auto, height: auto, margin: 5pt)

#stack(spacing: 0.8em, align(center, prooftree(
  rule(
    name: $∧ upright(R)$,
    rule(
      name: $∨ upright(L)$,
      rule(
        name: $¬ upright(L)$,
        rule(name: $"ax"$, $a ⊢ a$),
        rule(name: $⊥ upright(L)$, $⊥ ⊢ b$),
        $a, ¬a ⊢ b$,
      ),
      rule(
        name: $"WL"$,
        rule(name: $"ax"$, $b ⊢ b$),
        $b, ¬a ⊢ b$,
      ),
      $a ∨ b, ¬a ⊢ b$,
    ),
    rule(name: $⊤ upright(R)$, $a ∨ b, ¬a ⊢ ⊤$),
    $a ∨ b, ¬a ⊢ b ∧ ⊤$,
  ),
)), align(center, $x : a ∨ b, f : a → ⊥ ⊢ ⟨"case" space x space "of" space "inl"(y) → "abort"_(b)(f space y) ‖ "inr"(z) → z, ⟨⟩⟩ : b ∧ ⊤$))
