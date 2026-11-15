# Write rules as hypotheses

Many systems of rules need no calculus of their own: a vending machine,
a chemical reaction, a game, a protocol, a Petri net. Each rule turns
some resources into others, and linear logic says exactly that. Write a
rule as an implication under `!`, which lets it fire as often as it
applies, and the resources as hypotheses. A sequent then asks whether
the resources can be turned into the goal, and `deduxit prove` decides
it. This chapter shows how, and when a calculus of your own is the
better tool.

## A rule system as a sequent

A machine that sells tea for one coin and cake for two:

```console
$ deduxit prove -i --tree never "!(coin -o tea), !(coin * coin -o cake), coin, coin, coin |- tea * cake"
provable (IMELL, intuitionistic, horn engine)
$ deduxit prove -i "!(coin -o tea), !(coin * coin -o cake), coin, coin |- tea * cake"
unprovable (IMELL, intuitionistic, horn engine): the search was exhaustive
```

- `!(coin -o tea)` is the rule: one coin gives one tea, as often as
  there are coins. Without the `!` the rule would fire once.
- `coin, coin, coin` are the resources: three coins, each used once.
- `tea * cake` is the goal: a tea and a cake together.

Three coins buy both; two do not. Each resource is used exactly once,
so a coin left over makes a sequent unprovable too. `-a` (affine mode)
allows resources to be left unused:

```console
$ deduxit prove -i "!(coin -o tea), coin, coin |- tea"
unprovable (IMELL, intuitionistic, horn engine): the search was exhaustive
$ deduxit prove -i -a --tree never "!(coin -o tea), coin, coin |- tea"
provable (IMELL, intuitionistic affine, horn engine)
```

A sequent of this shape (rules `!(a * b -o c * d)`, resources, a goal
of atoms) is a Horn program, and deduxit decides it with the Horn
engine, which reads it as a Petri net: the rules are transitions, the
resources a marking. The engine finds a proof as a sequence of firings,
or a refutation that `deduxit check` can verify ([Check a proof or a
refutation](check.md)). The proof is an ordinary derivation of linear
logic: the drawing above reads, from the bottom up, as the rules fired.

## When to write a calculus instead

Rules as hypotheses suit rules that consume and produce things. They do
not suit rules about the shape of a derivation itself: a rule that
discharges an assumption, a modal rule that boxes a whole context, a
rule with a side condition such as a new name, or a sequent with
several zones. Those are rules of a calculus, and deduxit checks and
builds derivations in a calculus you give as a rule file ([Check a
derivation in your own rules](rules.md)).

| you want | write |
|---|---|
| resources turned into others, counted | rules as hypotheses, `prove` |
| to know whether a goal can be reached, with a refutation if not | rules as hypotheses, `prove` |
| the rules of a logic, a textbook's sequent calculus or natural deduction | a rule file, `check --calculus`, `interact --calculus` |
| to check a derivation someone wrote by hand | a rule file, `check --calculus` |

`prove` searches and decides; a calculus given as data is only checked
and built step by step, never searched.
