#import "@preview/curryst:0.6.0": prooftree, rule
#set page(width: auto, height: auto, margin: 5pt)

#prooftree(
  rule(
    name: $→ upright(I)$,
    rule(
      name: $→ upright(I)$,
      rule(
        name: $∨ upright(E)$,
        rule(name: $"ax"$, $f : a → b, x : a ∨ c ⊢ x : a ∨ c$),
        rule(
          name: $∨ upright(I)_1$,
          rule(
            name: $→ upright(E)$,
            rule(name: $"ax"$, $f : a → b, x : a ∨ c, y : a ⊢ f : a → b$),
            rule(name: $"ax"$, $f : a → b, x : a ∨ c, y : a ⊢ y : a$),
            $f : a → b, x : a ∨ c, y : a ⊢ f space y : b$,
          ),
          $f : a → b, x : a ∨ c, y : a ⊢ "inl"(f space y) : b ∨ c$,
        ),
        rule(
          name: $∨ upright(I)_2$,
          rule(name: $"ax"$, $f : a → b, x : a ∨ c, z : c ⊢ z : c$),
          $f : a → b, x : a ∨ c, z : c ⊢ "inr"(z) : b ∨ c$,
        ),
        $f : a → b, x : a ∨ c ⊢ "case" space x space "of" space "inl"(y) → "inl"(f space y) ‖ "inr"(z) → "inr"(z) : b ∨ c$,
      ),
      $f : a → b ⊢ lambda x. "case" space x space "of" space "inl"(y) → "inl"(f space y) ‖ "inr"(z) → "inr"(z) : (a ∨ c) → (b ∨ c)$,
    ),
    $⊢ lambda f. lambda x. "case" space x space "of" space "inl"(y) → "inl"(f space y) ‖ "inr"(z) → "inr"(z) : (a → b) → ((a ∨ c) → (b ∨ c))$,
  ),
)
