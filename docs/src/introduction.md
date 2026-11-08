# linlog

A linear logic suite for all your needs: a command line program and a Rust
library that parse, print and decide sequents of classical and
intuitionistic linear logic and their fragments, keep the proofs in a
checkable form and show them as derivation trees or proof nets, prove
step by step, export to LaTeX, Typst, SVG and Rocq, and decide ordinary
propositional logic through its embeddings into linear logic.

This manual describes the `main` branch of
[the repository](https://github.com/sequentium/sequentium); a released
version's manual is the `docs/` directory of its tag. linlog's
[web page](https://sequentium.github.io/) proves step by step in the
browser on the same library.

## A first proof

```console
$ linlog prove "A, A -o B |- B"
provable (MLL, classical, focus engine)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B
```

The verdict, the fragment the sequent lives in and the engine that
searched, then the proof as a derivation tree of the one-sided sequent
calculus, which the library's independent checker has validated.
[The commands](commands.md) has every command; `linlog --help` and
`linlog <command> --help` document every option.

## Documentation, results and how linlog was written

- The library's API: [docs.rs/linlog](https://docs.rs/linlog) for the
  releases, [sequentium.github.io/sequentium](https://sequentium.github.io/sequentium/)
  for `main`, rebuilt on every push.
- For AI models: a guide to the command and the library, the Agent Skill
  [`skills/linlog/`](https://github.com/sequentium/sequentium/tree/main/skills/linlog)
  (a summary, `SKILL.md`, and the full reference beside it, whose every
  example a test runs), served as
  [llms.txt](https://sequentium.github.io/sequentium/llms.txt) and
  [llms-full.txt](https://sequentium.github.io/sequentium/llms-full.txt);
  copy the directory into an agent's skills (`~/.claude/skills/` for
  Claude Code) to give it the guide.
- The changes of every release:
  [CHANGELOG.md](https://github.com/sequentium/sequentium/blob/main/CHANGELOG.md).
- The benchmarks: the latest baseline of the engines on the generated
  families and the LLTP library,
  [bench/RESULTS.md](https://github.com/sequentium/sequentium/blob/main/bench/RESULTS.md),
  the baselines compared,
  [bench/COMPARISON.md](https://github.com/sequentium/sequentium/blob/main/bench/COMPARISON.md),
  and the comparison with the other provers of linear logic,
  [bench/TOOLS.md](https://github.com/sequentium/sequentium/blob/main/bench/TOOLS.md)
  ([Beside the other provers](benchmarks.md#beside-the-other-provers)),
  so far of 0.1.0's engines on the libraries and, at first order, of
  LinearOne beside 0.2.0's search; the other provers on the growing
  families are not yet run.
- To cite linlog, GitHub's "Cite this repository" reads
  [CITATION.cff](https://github.com/sequentium/sequentium/blob/main/CITATION.cff).
- linlog's code, tests and documentation were written with substantial
  AI assistance, by Anthropic's Claude models in Claude Code, in steps
  the author planned, directed and accepted. Every proof the library
  returns has passed its independent checker, and a Rocq certificate
  can be checked by Rocq.
