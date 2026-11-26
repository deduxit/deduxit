---
name: deduxit
description: A collection of logic suites in one command, `deduxit`, that decides, checks, proves step by step and exports sequents of ordinary logic (classical, intuitionistic and minimal, propositional and first-order), of linear logic (classical, intuitionistic, affine, with Mix, cyclic, the Lambek calculus, first-order) and of the modal logics from K to S5, checks lambda-terms, and checks derivations in calculi given as rule files. Use it to learn whether a sequent is provable and why not (a refutation, a checked Kripke countermodel or finite structure, a truth table or a tableau), to get a checked proof as a derivation tree, proof net, JSON, LaTeX, Typst, SVG, PNG, PDF or Rocq certificate, to check a proof file or eliminate its cuts, to apply proof steps from a script, to grade an answer, or to decide many sequents in one call.
license: EUPL-1.2
compatibility: Needs the `deduxit` command, from `cargo install --locked deduxit-cli` or `nix run github:deduxit/deduxit -- ARGS`.
---

# deduxit

> `deduxit` is a collection of logic suites: it decides sequents of ordinary, linear and modal logic, and checks lambda-terms and derivations in rule files. A proof it prints has passed an independent checker; an unprovable verdict says why where it can; an unknown verdict names the limit that ended the search and the flag to raise.

`deduxit COMMAND --help` documents every flag, and [reference.md](reference.md) everything, with examples.

**Commands.** `deduxit prove SEQUENT` decides a sequent and prints a proof; `deduxit check FILE` checks a proof that `prove --format json` wrote; `deduxit eliminate FILE` eliminates its cuts step by step; `deduxit interact SEQUENT` applies proof steps read from standard input; `deduxit seq print|json|fragment SEQUENT` prints the sequent, writes its JSON or names its fragment; `deduxit workspace` keeps a sequent and what is made of it; `deduxit grade SEQUENT` grades an answer to it from standard input (a session script, `prove`'s JSON, a countermodel or a tableau), exit 0 correct, 1 incorrect; `deduxit term check|inhabit` types a lambda-term or finds one of a type; `check --calculus LK FILE` checks a derivation in a built-in calculus or a rule file.

**The logic.** `--logic classical|intuitionistic|minimal` reads ordinary logic, with `->`, `/\`, `\/`, `~`, `<->`, `true` and `false`, and answers `valid` or `not valid`; `--method` picks how it is decided. Otherwise it reads classical linear logic, or: `-i` intuitionistic (hypotheses left of `|-`, exactly one formula right of it, and no `~`, `par`, `|`, `?` or `bot`: write `A -o B`), `-a` affine (a hypothesis may go unused), `--mix`, `--cyclic` (order kept up to rotation), `--lambek` (with `A \ B` and `B / A`). `--system S4` (K to S5) reads modal logic, with `[]` and `<>` added.

**Syntax.** Quote the sequent for the shell: `"A, A -o B |- B"` (`⊢` also works; either side may be empty). After a common mistake the command's tips say what to write. Linear connectives: `*` (⊗), `par` or `|` (⅋), `&`, `+` (⊕), `-o` (⊸), `~A` or `A^` (negation), `!A`, `?A`; units `1`, `bot`, `top`, `0`. Binding, tightest first: `~ ! ?`, then `*`, `|`, `&`, `+`, and `-o`, which groups to the right. Atoms are identifiers, or predicates over terms, `p(x, f(a))`; `forall x. A` and `exists x. A` bind loosest, under `--logic` and in linear logic but for `--cyclic` and `--lambek`.

**The answer.** The first line is the verdict, then the fragment, the mode and the engine in parentheses, and after a colon the reason. The exit status says the same:

- 0: provable (`valid` under `--logic`, `valid proof` for `check`);
- 1: unprovable (`not valid`, an invalid proof);
- 2: an error, such as a parse error, a flag that does not apply, or `-i` given a classical symbol;
- 3: unknown: a limit ended the search first, which is not unprovable.

```console
$ deduxit prove "A * B |- B * A"
provable (MLL, classical, focus engine)
─────── ax   ─────── ax
⊢ ~B, B      ⊢ ~A, A
──────────────────── ⊗
  ⊢ ~A, ~B, B ⊗ A
  ──────────────── ⅋
  ⊢ ~A ⅋ ~B, B ⊗ A
$ deduxit prove -q "A |- B"; echo "exit $?"
unprovable (MLL, classical, focus engine): A on the left has no A on the right to meet in an axiom
exit 1
$ deduxit prove -q --copies 1 "!(A & B) |- A * B"; echo "exit $?"
unknown (LL, classical, focus engine): the copy bound of 1 was reached after 3.56ms; raise it with --copies N, or lift it with --copies none to deepen it while the time limit lasts
exit 3
$ deduxit prove --engine net "A & B |- A"; echo "exit $?"
error: proof nets exist for MLL without units only, not for ALL
exit 2
$ deduxit prove --logic intuitionistic -q "~~a -> a"
not valid (intuitionistic logic by the SAT solver): a checked countermodel refutes it
```

The derivation is one-sided (`Γ ⊢ Δ` read as `⊢ ~Γ, Δ`); `-i` and `--logic` draw two-sided ones, `--classical-sides two` a classical one as written.

**Limits.** By default a search stops after `--timeout 2s` (from the command's start; `none` for no limit) and within `--memory-limit 1GiB`; with `!` or `?` it deepens how often a `?` formula may be copied, and `--copies N` caps that. For the same verdict on every machine, bound the search with `--work-limit N` or `--copies N` and lift the time limit with `--timeout none`. `--deterministic` also makes the proof and `--stats` the same on every run.

**Output.** `-q` prints the verdict line alone. `--format json` writes one JSON object: `verdict` (`proved`, `unprovable` or `unknown`), `refutation` or `reason`, `fragment`, `mode`, `engine`, `statistics` and the proof, which `check` reads back. `check` takes the logic from the file's `"mode"`; a mode flag that names another logic is an error (exit 2). `--format latex|typst|svg|png|pdf|rocq` or `--output FILE` with that extension exports the derivation; `--net` gives the proof net instead (MLL and MELL without units); `--stats` adds the search's counters.

```console
$ deduxit prove --format json "A |- A" > proof.json
$ deduxit check -q proof.json
valid proof of ⊢ ~A, A (classical)
$ deduxit prove --format latex "A, A -o B |- B"
% provable (MLL, classical, focus engine)
\begin{prooftree}
\infer0[$\mathrm{ax}$]{\vdash A^\bot, A}
\infer0[$\mathrm{ax}$]{\vdash B^\bot, B}
\infer2[$\otimes$]{\vdash A^\bot, A \otimes B^\bot, B}
\end{prooftree}
```

**Many sequents.** `--input-format lines` reads `NAME: SEQUENT` or `SEQUENT` per line (from `--file` or standard input; blank lines and `#` comments skipped) and answers `NAME: VERDICT` per line in input order; the exit status is the worst: an error (2) before unknown (3) before unprovable (1) before proved (0). `--format json` makes each answer a JSON line.

**Step by step.** `interact` reads one command per line: `goals`, `rules G P`, `apply G P RULE [P…]` (goal G, the formula at position P; for `*` the positions of G's other formulas that go to the left premise follow), `cut G C [P…]` (a cut on the formula C, the positions P going to the premise that proves it), `undo`, `close [G]` (the search closes a goal), `show`, `proof [FILE]` (checks the finished proof; a FILE gets its JSON, for `check`), `save FILE` and `load FILE` (the session's JSON), `quit`.

The session starts with goal 0; a step closes its goal and opens new ones, newly numbered (`opened goal 1: ⊢ 0: ~A, 1: A`), each goal's formulas numbered from 0: read the numbers from the output before the next step. A RULE is `ax` (on either literal), `*`, `par`, `&`, `+1`, `+2`, `1`, `bot`, `top`, `?d`, `?c`, `?w`, `!`, `mix` or `wk`, and with `-i` the two-sided names (`-oL`, `-oR`, `*L`, `*R`, …, in reference.md). It exits 0 when the session ends with a checked proof, 1 otherwise. With `--logic`, LK's or LJ's rules (`andR`, `->L`) on formulas named `L0`, `R1` (reference.md, section 13).

```console
$ printf 'apply 0 2 * 0\nclose\nproof\n' | deduxit interact "A, B |- A * B"
opened goal 1: ⊢ 0: ~A, 1: A
opened goal 2: ⊢ 0: ~B, 1: B
goal 1: provable (MLL, classical, focus engine)
goal 2: provable (MLL, classical, focus engine)
no goal is open: `proof` checks the proof
valid proof (classical)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, ~B, A ⊗ B
```

**The library** is the Rust crate `deduxit`, a module per suite: `ordinary::decide`, `linear::prove`, `modal::decide`, `lambda` and `rules` (reference.md, section 16).

## Docs

- [Reference](reference.md): every command, flag, format and JSON form, with examples
- [Manual](https://deduxit.github.io/deduxit/manual/): a tour, a guide per task, the reference
- [Library API](https://deduxit.github.io/deduxit/deduxit/): the Rust crate's documentation

## Optional

- [Rocq library](https://github.com/deduxit/rocq-deduxit): the calculi and checkers the `--kernel deduxit` certificates are proved with
