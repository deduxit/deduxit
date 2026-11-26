# <picture><source media="(prefers-color-scheme: dark)" srcset="https://deduxit.github.io/logo/wordmark-dark.svg"><img alt="deduxit" src="https://deduxit.github.io/logo/wordmark-light.svg" height="80"></picture>

> **dēdūxit** [deːˈduːk.sɪt] *Latin, verb*: "it has derived", the perfect of *dēdūcō*, to lead down, derive.

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.23448588.svg)](https://doi.org/10.5281/zenodo.23448588)

deduxit is a collection of logic suites. It decides whether a sequent
is provable, and shows the proof or says why there is none. It is a
command, `deduxit`, and a Rust library, the crate `deduxit`. Its suites
are:

- ordinary logic: classical, intuitionistic and minimal, propositional
  and first-order, with equality. A sequent that is not valid gets a
  Kripke countermodel or a finite structure. Classical logic also has
  truth tables, tableaux, normal forms, Herbrand's theorem and
  resolution.
- linear logic: classical and intuitionistic, affine, first-order and
  cyclic, and the Lambek calculus, with proof nets.
- modal logic: the fifteen propositional systems from K to S5, with a
  derivation or a Kripke countermodel.
- lambda-terms: a proof read as a program, the terms you write checked,
  and a type's terms found.
- calculi given as rule files, natural deduction among them, drawn as
  trees or as Fitch proofs.

deduxit proves step by step, grades an answer to a problem, and exports
proofs to LaTeX, Typst, SVG, PNG, PDF and Rocq. The
[web page](https://deduxit.github.io/) proves step by step in the
browser with the same library.

## Install

Each [release](https://github.com/deduxit/deduxit/releases) has the
command built for Linux (an archive, a `.deb`, an `.rpm` and an Arch
Linux package), macOS and Windows. With Rust 1.92 or later, or with Nix:

```sh
cargo install --locked deduxit-cli                  # build and install it
nix run github:deduxit/deduxit -- --help            # run it once, without installing
nix profile add github:deduxit/deduxit#deduxit-cli  # install it into your profile
```

The library is the crate [`deduxit`](https://crates.io/crates/deduxit):

```toml
[dependencies]
deduxit = "0.8"
```

The manual's
[Install deduxit](https://deduxit.github.io/deduxit/manual/installation.html)
has the commands for each system, updating and removing, and the
library's features.

## A first proof

`--logic` names the logic of an ordinary sequent:

```console
$ deduxit prove --logic intuitionistic "A, A -> B |- B"
valid (intuitionistic logic by the SAT solver)
───── ax   ──────── ax
A ⊢ A      B, A ⊢ B
─────────────────── →L
   A, A → B ⊢ B
$ deduxit prove --logic intuitionistic "|- A \/ ~A"
not valid (intuitionistic logic by the SAT solver): a checked countermodel refutes it
world 0 (the model's world): forces nothing; below 1
world 1: forces A
```

The first line is the verdict, with the logic and the method that
decided it. Then comes the proof, which a checker has verified, or the
reason there is none. Here a Kripke countermodel shows why `A \/ ~A`
does not hold: at world 0, `A` is not yet true, and `~A` fails since
`A` holds at world 1 above it.

Without `--logic`, `prove` reads linear logic, where each hypothesis is
used exactly once:

```console
$ deduxit prove "A, A -o B |- B"
provable (MLL, classical, focus engine)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B
$ deduxit prove "A |- B"
unprovable (MLL, classical, focus engine): A on the left has no A on the right to meet in an axiom
```

The verdict names the fragment of linear logic the sequent uses, the
logic and the engine that searched. The proof is written one-sided, the
standard form for classical linear logic: `⊢ ~A, A` is the axiom
`A ⊢ A`. `--classical-sides two` writes it with the sides you gave:

```console
$ deduxit prove --classical-sides two "A, A -o B |- B"
provable (MLL, classical, focus engine)
───── ax   ───── ax
A ⊢ A      B ⊢ B
──────────────── ⊸L
  A, A ⊸ B ⊢ B
```

`--system` names a modal logic, and `deduxit term` works with
lambda-terms:

```console
$ deduxit prove --system K "[]A |- A"
not valid in K: a checked countermodel of 1 world refutes □A ⊢ A at world 0
world 0 (where the sequent fails): forces nothing; sees no world
$ deduxit term inhabit "|- (A -> B) -> (B -> C) -> A -> C"
inhabited
⊢ λf:(A → B). λg:(B → C). λx:A. g (f x) : (A → B) → ((B → C) → (A → C))
```

The exit status is 0 for provable or valid, 1 for unprovable or not
valid, 3 for unknown and 2 for an error. `deduxit <command> --help`
shows every option.

## Where to go next

- **The manual**,
  [deduxit.github.io/deduxit/manual](https://deduxit.github.io/deduxit/manual/),
  starts with a short tour, then has a guide for each task (deciding a
  sequent in each logic, checking a proof, exporting, Rocq certificates,
  proving step by step, the library), the reference, how the search
  works, and [the benchmarks](https://deduxit.github.io/deduxit/manual/benchmarks.html)
  beside the other provers. It describes `main`; a release's manual is
  the `docs/` directory of its tag.
- **The web page**, [deduxit.github.io](https://deduxit.github.io/):
  proving step by step in the browser, with lessons.
- **The library's API**: [docs.rs/deduxit](https://docs.rs/deduxit) for
  the releases, [deduxit.github.io/deduxit](https://deduxit.github.io/deduxit/)
  for `main`.
- **For AI models**: the Agent Skill
  [`skills/deduxit/`](https://github.com/deduxit/deduxit/tree/main/skills/deduxit),
  a summary and a full reference whose examples a test runs, also served
  as [llms.txt](https://deduxit.github.io/deduxit/llms.txt) and
  [llms-full.txt](https://deduxit.github.io/deduxit/llms-full.txt). Copy
  the directory into an agent's skills (`~/.claude/skills/` for Claude
  Code).
- **The Rocq library**, `rocq-deduxit`
  ([deduxit/rocq-deduxit](https://github.com/deduxit/rocq-deduxit)): the
  calculi, checkers and soundness theorems that the certificates of
  `--kernel deduxit` are proved with.
- **What changed**: the [changelog](CHANGELOG.md).

deduxit's code, tests and documentation were written with substantial AI
assistance, by Anthropic's Claude models in Claude Code, in steps the
author planned, directed and accepted. Every proof the library returns
has passed its independent checker, and a Rocq certificate can be
checked by Rocq. deduxit is much inspired by
[Click & coLLecT](https://www.click-and-collect.linear-logic.org).
Feature requests and contributions are welcome.

## Citing

Each release is archived on Zenodo. To cite deduxit, use the DOI
[10.5281/zenodo.23448588](https://doi.org/10.5281/zenodo.23448588),
which always resolves to the latest version, or the DOI of the version
you used, which its Zenodo record gives. [`CITATION.cff`](CITATION.cff)
holds the entry, and GitHub's "Cite this repository" writes it as BibTeX
or APA.

## Licence

deduxit is © Fabian Lukas Grubmüller and licensed under the European
Union Public Licence, version 1.2 (EUPL-1.2), whose text is in
[`LICENSE`](LICENSE).

One part of the repository is someone else's work under its own licence:
the five math fonts in [`cli/fonts/`](cli/fonts), which the `deduxit`
command embeds to draw proofs, four under the SIL Open Font License 1.1:
Euler Math, © the American Mathematical Society, Khaled Hosny and
Daniel Flipo ([`cli/fonts/OFL.txt`](cli/fonts/OFL.txt)); STIX Two Math,
© the STIX Fonts Project Authors
([`cli/fonts/STIXTwoMath-Regular-OFL.txt`](cli/fonts/STIXTwoMath-Regular-OFL.txt));
Garamond-Math, © Yuansheng Zhao and Xiangdong Zeng
([`cli/fonts/Garamond-Math-OFL.txt`](cli/fonts/Garamond-Math-OFL.txt));
and Asana Math, © Apostolos Syropoulos, Young Ryu and Claudio Beccari
([`cli/fonts/Asana-Math-OFL.txt`](cli/fonts/Asana-Math-OFL.txt)); and
New Computer Modern Math (its Book weight), © Antonis Tsolomitis, under
the GUST Font License, which is the LaTeX Project Public License 1.3c
([`cli/fonts/NewCMMath-Book-LICENSE.txt`](cli/fonts/NewCMMath-Book-LICENSE.txt),
which also says where the complete package is). The package
`deduxit-cli` is therefore `EUPL-1.2 AND OFL-1.1 AND LPPL-1.3c`; the
library `deduxit` contains no font. Drawings and documents made with the OFL
fonts are not covered by their licence (its condition 5); a PNG holds
no font, while a PDF embeds the part of the font its text uses, and for
New Computer Modern Math the LPPL makes no exception for documents.

deduxit's mark, at the head of this file, is not in the repository: it
is served by [deduxit's site](https://deduxit.github.io/logo/LICENSE.txt),
© Fabian Lukas Grubmüller, under
[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). The name
deduxit and its mark identify this project, and may not be used to
suggest its endorsement of anything else.

The problem libraries and the other provers that the benchmarks and the
comparison use are not part of this repository: the flake fetches each
at a pinned version when it is needed, under its own licence. The LLTP
library is GPL-3.0, the ILTP library states no licence, qcover's
problems are Apache-2.0, and the manual's
["Beside the other provers"](https://deduxit.github.io/deduxit/manual/benchmarks.html#beside-the-other-provers) gives the
provers' licences.
