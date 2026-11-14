# Prove step by step

`deduxit interact` builds a proof one rule at a time. Give it the
sequent, then type commands on standard input, one per line. A whole
session:

```console
$ deduxit interact "A, A -o B |- B"
> goals
goal 0: ⊢ 0: ~A, 1: A ⊗ ~B, 2: B
> apply 0 1 * 0
opened goal 1: ⊢ 0: ~A, 1: A
opened goal 2: ⊢ 0: ~B, 1: B
> close
goal 1: provable (MLL, classical, focus engine)
goal 2: provable (MLL, classical, focus engine)
no goal is open: `proof` checks the proof
> proof
valid proof (classical)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B
```

- `goals` lists the open goals. Each goal has a number, and each
  formula in it a position. In the default classical mode a goal is
  one-sided: a hypothesis moves right of `⊢` as its negation, so `A`
  is `~A` and `A -o B` is `A ⊗ ~B`.
- `apply 0 1 * 0` applies the rule `⊗` to formula 1 of goal 0. A `⊗`
  splits the other formulas between its two premises: the positions
  after the rule's name, here 0, go to the left premise, the others to
  the right one.
- `close` lets the search close every open goal, and `close G` closes
  goal G alone.
- `proof` checks the finished proof with the independent checker and
  prints it.

`help` lists every command, and `quit` ends the session.

## Choose a rule

`rules G P` lists the rules that act on formula P of goal G. "With a
split" means that `apply` takes the positions of the formulas that go
to the left premise. With `-i` the session is intuitionistic: goals are
two-sided, and the rules have their two-sided names, such as `⊸L` for
an implication on the left:

```console
$ deduxit interact -i "A, A -o B |- B"
> goals
goal 0: 0: A, 1: A ⊸ B ⊢ 2: B
> rules 0 1
⊸L (with a split)
> apply 0 1 -oL 2
error: a premise would have 2 formulas right of ⊢ instead of one
> apply 0 1 -oL 0
opened goal 1: 0: A ⊢ 1: A
opened goal 2: 0: B ⊢ 1: B
> apply 1 0 ax
closed
> close
goal 2: provable (IMLL, intuitionistic, two-sided engine)
no goal is open: `proof` checks the proof
> proof
valid proof (intuitionistic)
───── ax   ───── ax
A ⊢ A      B ⊢ B
──────────────── ⊸L
  A, A ⊸ B ⊢ B
```

A command that fails prints `error: …`, and the session goes on. Name
a rule by its usual name or by its ASCII spelling:

- classical: `ax`, `1`, `bot`, `top`, `*`, `par`, `&`, `+1`, `+2`,
  `?d`, `?c`, `?w`, `!`, `mix`, and `wk` (weakening, in affine mode);
- intuitionistic: `ax`, `-oL`, `-oR`, `*L`, `*R`, `&L1`, `&L2`, `&R`,
  `+L`, `+R1`, `+R2`, `1L`, `1R`, `0L`, `topR`, `!L`, `!R`, `!c`, `!w`.

An unknown name gets an error that lists the rules of the session, the
ASCII spelling first:

```console
$ deduxit interact "A, B |- A * B"
> apply 0 2 tensor 1
error: unknown rule `tensor`; the rules here are ax, * (⊗), par (⅋), 1, bot (⊥), &, +1 (⊕₁), +2 (⊕₂), top (⊤), !, ?d, ?c and ?w
```

## Take back a split that cannot be closed

When a split leaves a premise that has no proof, as the counts of its
atoms show, `apply` makes it and prints a warning. `undo` takes back
the last step:

```console
$ deduxit interact "A, B |- A * B"
> apply 0 2 * 1
opened goal 1: ⊢ 0: ~B, 1: A
opened goal 2: ⊢ 0: ~A, 1: B
warning: one premise has no proof, so this split cannot be closed; `undo` takes it back
> undo
reopened goal 0: ⊢ 0: ~A, 1: ~B, 2: A ⊗ B
```

Here `~B` went left with `A`, and no axiom pairs them. `apply 0 2 * 0`
sends `~A` left instead and can be closed.

## Prove a lemma with a cut

A cut proves a formula C once and then uses it: from `Γ ⊢ C` and
`Δ, C ⊢ D` it concludes `Γ, Δ ⊢ D`. `cut G C P…` cuts goal G on C.
Write C as the session writes formulas (in ILL's syntax with `-i`), in
quotes if it has blanks. The formulas at the positions P go to the
first premise, which proves C. The others go to the second premise,
which takes C as a hypothesis. `undo` takes the cut back with its
formula. The search never makes a cut, and `close` proves each premise
without one:

```console
$ deduxit interact -i "A, A -o B, B -o C |- C"
> cut 0 B 0 1
opened goal 1: 0: A, 1: A ⊸ B ⊢ 2: B
opened goal 2: 0: B ⊸ C, 2: B ⊢ 1: C
> close
goal 1: provable (IMLL, intuitionistic, two-sided engine)
goal 2: provable (IMLL, intuitionistic, two-sided engine)
no goal is open: `proof` checks the proof
> proof
valid proof (intuitionistic)
───── ax   ───── ax   ───── ax   ───── ax
A ⊢ A      B ⊢ B      B ⊢ B      C ⊢ C
──────────────── ⊸L   ──────────────── ⊸L
  A, A ⊸ B ⊢ B          B ⊸ C, B ⊢ C
  ────────────────────────────────── cut
         A, A ⊸ B, B ⊸ C ⊢ C
```

[Eliminate cuts](cut.md) turns such a proof into one without cuts.

## Save a session and resume it

`save FILE` writes the session as JSON. `deduxit interact --state FILE`
starts from a saved session, in the mode it was saved in. Inside a
session, `load FILE` resumes a saved session of the same sequent and
mode. `proof FILE` writes the finished proof as JSON for
[`check`](check.md) and `eliminate`:

```console
$ deduxit interact "A, A -o B |- B"
> apply 0 1 * 0
opened goal 1: ⊢ 0: ~A, 1: A
opened goal 2: ⊢ 0: ~B, 1: B
> save session.json
session written to session.json
$ deduxit interact --state session.json
> close
goal 1: provable (MLL, classical, focus engine)
goal 2: provable (MLL, classical, focus engine)
no goal is open: `proof` checks the proof
> proof proof.json
valid proof written to proof.json
$ deduxit check proof.json
valid proof of ⊢ ~A, A ⊗ ~B, B (classical)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B
```

## Show the derivation and export the proof

`show` draws the derivation so far. An open goal is its sequent alone,
with no bar above it:

```console
$ deduxit interact "A, A -o B |- B"
> apply 0 1 * 0
opened goal 1: ⊢ 0: ~A, 1: A
opened goal 2: ⊢ 0: ~B, 1: B
> show
⊢ ~A, A   ⊢ ~B, B
───────────────── ⊗
 ⊢ ~A, A ⊗ ~B, B
> show tree.svg
derivation so far written to tree.svg
```

- `show [--FORMAT] [FILE]` prints the derivation as text, or writes it
  with `--latex`, `--typst`, `--svg`, `--png` or `--pdf`. PNG and PDF
  need a FILE.
- `proof [--FORMAT] [FILE]` checks the finished proof and prints or
  writes it in the same formats, or with `--json`, or with `--rocq` as a
  certificate for [Rocq](rocq.md).
- Given a FILE and no format, both write the format the file's
  extension names. Where it names none, `show` writes text and `proof`
  writes JSON.
- `lambda` prints the finished proof's lambda-term
  ([Read a proof as a program](terms.md)).

[Export a proof](output-formats.md) describes each format.

## Apply the invertible rules automatically

A rule is invertible when its premises are provable whenever its
conclusion is, so applying it never loses a proof. With `--invertible`,
or after the command `invertible on`, the session applies these rules
by itself, at once to every open goal and then to the goals each step
opens:

- `⅋`, `⊥`, `&` and `⊤`, in intuitionistic mode `⊸R`, `⊗L`, `&R`,
  `⊕L`, `1L`, `⊤R` and `0L`;
- `!` when every other formula is a `?`;
- `1`, `⊗` and the axiom when their formulas are the whole goal.

Each is a step of the session. `undo` takes back a step together with
the invertible steps that followed it, and `invertible off` stops them:

```console
$ deduxit interact --invertible "A -o B, B -o C |- A -o C"
applied 1 invertible step
> goals
goal 1: ⊢ 0: A ⊗ ~B, 1: B ⊗ ~C, 2: ~A, 3: C
> apply 1 0 ⊗ 2
then 1 invertible step
opened goal 3: ⊢ 0: ~B, 1: B ⊗ ~C, 2: C
> apply 3 1 ⊗ 0
then 2 invertible steps
closed; no goal is open: `proof` checks the proof
> undo
retracted 3 steps
reopened goal 3: ⊢ 0: ~B, 1: B ⊗ ~C, 2: C
> invertible off
invertible rules off
> apply 3 1 ⊗ 0
opened goal 4: ⊢ 0: ~B, 1: B
opened goal 5: ⊢ 0: ~C, 1: C
> invertible on
invertible rules on; then 2 invertible steps
no goal is open: `proof` checks the proof
> proof
valid proof (classical)
             ─────── ax   ─────── ax
             ⊢ ~B, B      ⊢ ~C, C
─────── ax   ──────────────────── ⊗
⊢ A, ~A        ⊢ ~B, B ⊗ ~C, C
────────────────────────────── ⊗
   ⊢ A ⊗ ~B, B ⊗ ~C, ~A, C
   ──────────────────────── ⅋
   ⊢ A ⊗ ~B, B ⊗ ~C, ~A ⅋ C
```

## Quantifiers

In a sequent with quantifiers, `∀R` and `∃L` (`forallR`, `existsL`;
one-sided `forall`) open the formula with a fresh eigenvariable, written
`a`, `b`, …. `∃R` and `∀L` (`existsR`, `forallL`; one-sided `exists`)
take their witness after the rule's name: `apply G P existsR T`, where
the term T is made of the sequent's function symbols and the
eigenvariables the goal shows. Without a witness the rule leaves it open,
written `$1`, `$2`, …, for an axiom or `close` to fill in.
[Prove in first-order logic](first-order.md) has a session.

## Ordinary logic

`deduxit interact --logic classical`, `intuitionistic` or `minimal`
proves a sequent of ordinary logic with the rules of LK or LJ, with the
same commands but `cut` and `invertible`. See
[Decide ordinary logic](ordinary-logic.md#step-by-step).

## Exit status

The exit status is 0 when the session ends with a finished proof that
checks, and 1 otherwise. A command that fails prints `error: …` on
standard output, and the session goes on. The status is 2 when the
session cannot start or cannot read its input.

The library's `Interactive` type has the same operations, and its JSON
form is the file `save` writes.
