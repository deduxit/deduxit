#import "@preview/curryst:0.6.0": prooftree, rule
#set page(width: auto, height: auto, margin: 5pt)

#stack(spacing: 0.8em, align(center, prooftree(
  rule(
    name: $⊗ upright(L)$,
    rule(
      name: $bold(1) upright(L)$,
      rule(
        name: $⊕ upright(L)$,
        rule(
          name: $\& upright(R)$,
          rule(
            name: $⊕ upright(R)_1$,
            rule(
              name: $! upright(L)$,
              rule(
                name: $⊸ upright(L)$,
                rule(name: $"ax"$, $a ⊢ a$),
                rule(name: $"ax"$, $b ⊢ b$),
                $a ⊸ b, a ⊢ b$,
              ),
              $!(a ⊸ b), a ⊢ b$,
            ),
            $!(a ⊸ b), a ⊢ b ⊕ c$,
          ),
          rule(name: $⊤ upright(R)$, $!(a ⊸ b), a ⊢ ⊤$),
          $!(a ⊸ b), a ⊢ (b ⊕ c) class("binary", \&) ⊤$,
        ),
        rule(
          name: $! upright(w)$,
          rule(
            name: $\& upright(R)$,
            rule(
              name: $⊕ upright(R)_2$,
              rule(name: $"ax"$, $c ⊢ c$),
              $c ⊢ b ⊕ c$,
            ),
            rule(name: $⊤ upright(R)$, $c ⊢ ⊤$),
            $c ⊢ (b ⊕ c) class("binary", \&) ⊤$,
          ),
          $!(a ⊸ b), c ⊢ (b ⊕ c) class("binary", \&) ⊤$,
        ),
        $!(a ⊸ b), a ⊕ c ⊢ (b ⊕ c) class("binary", \&) ⊤$,
      ),
      $bold(1), !(a ⊸ b), a ⊕ c ⊢ (b ⊕ c) class("binary", \&) ⊤$,
    ),
    $bold(1) ⊗ !(a ⊸ b), a ⊕ c ⊢ (b ⊕ c) class("binary", \&) ⊤$,
  ),
)), align(center, $x : bold(1) ⊗ !(a ⊸ b), y : a ⊕ c ⊢ "let" space x space "be" space z ⊗ u space "in" space "let" space z space "be" space ∗ space "in" space "case" space y space "of" space "inl"(v) → ⟨"inl"("derelict"(u) space v), "true"(u, v)⟩ ‖ "inr"(w) → "discard" space u space "in" space ⟨"inr"(w), "true"(w)⟩ : (b ⊕ c) \& ⊤$))
