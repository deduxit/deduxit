#import "@preview/fitchcraft:1.0.1": *
#set page(width: auto, height: auto, margin: 5pt)

#proof((
  open,
  $a ∨ b$,
  assume,
  open,
  $a$,
  assume,
  ($b ∨ a$, [$∨ upright(I)_2$ 2]),
  close,
  open,
  $b$,
  assume,
  ($b ∨ a$, [$∨ upright(I)_1$ 4]),
  close,
  ($b ∨ a$, [$∨ upright(E)$ 1, 2–3, 4–5]),
  close,
  ($a ∨ b → b ∨ a$, [$→ upright(I)$ 1–6]),
))
