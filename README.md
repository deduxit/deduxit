# <picture><source media="(prefers-color-scheme: dark)" srcset="https://deduxit.github.io/logo/wordmark-dark.svg"><img alt="deduxit" src="https://deduxit.github.io/logo/wordmark-light.svg" height="80"></picture>

> **dēdūxit** [deːˈduːk.sɪt] *Latin, verb*: "it has derived", the perfect of *dēdūcō*, to lead down, derive.

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.23448588.svg)](https://doi.org/10.5281/zenodo.23448588)

Logic suites for all your needs, linear logic first: a command line
program and a Rust library that parse, print and decide sequents of classical and
intuitionistic linear logic and their fragments, keep the proofs in a
checkable form and show them as derivation trees or proof nets, prove
step by step, export to LaTeX, Typst, SVG and Rocq, decide ordinary
propositional logic through its embeddings into linear logic, and read
intuitionistic proofs back as lambda-terms.

deduxit is a collection of logic suites, grown one suite at a time: the
crates `deduxit` and `deduxit-cli`, the command `deduxit`, whose
`--logic` chooses the suite, and the library's modules `deduxit::linear`,
`deduxit::ordinary` and `deduxit::lambda`. Up to 0.2.0 it was called
linlog.

Version 0.3.0 is the first under the name deduxit: the linear suite is
the module `deduxit::linear`, ordinary logic is proved step by step in
LK and LJ, intuitionistic proofs read back as lambda-terms, the
one-sided count and the state equation are checked without the search
and certified in Rocq, and the mark dēdūxit replaces linlog's logo; the
[changelog](CHANGELOG.md) says what changed and what breaks since 0.2.0.

deduxit's manual, [deduxit.github.io/deduxit/manual](https://deduxit.github.io/deduxit/manual/),
describes every command, format and engine with real output, and the
benchmarks and the comparison with the other provers with their figures
(the other provers beside 0.1.0's engines on the libraries, and beside
0.2.0's on the growing families, where two of them refute provable
sequents once 63 formulas stand beside a `⊗`); the [web page](https://deduxit.github.io/) proves step by step in
the browser on the same library.

## Install

The command `deduxit` is the package `deduxit-cli`. Each
[release](https://github.com/deduxit/deduxit/releases) has it built
for Linux (an archive, a `.deb`, an `.rpm` and an Arch Linux package),
macOS and Windows; crates.io and the flake of this repository build it:

```sh
cargo install --locked deduxit-cli                        # a Rust toolchain builds it
nix run github:deduxit/deduxit -- --help           # run it once, without installing
nix profile add github:deduxit/deduxit#deduxit-cli  # install it into your profile
```

Building needs Rust 1.92 or later. The library is the crate
[`deduxit`](https://crates.io/crates/deduxit):

```toml
[dependencies]
deduxit = "0.3"
```

The manual's
[Installation](https://deduxit.github.io/deduxit/manual/installation.html)
has the rest: the commands for Debian and Ubuntu, Fedora, Arch Linux,
NixOS and Nix, macOS and Windows, with how to update and remove, which
build to prefer for long searches, the flake as an input of a NixOS or
Home Manager configuration, the library's features and the Rocq kernel
that checks the certificates.

## A first proof

```console
$ deduxit prove "A, A -o B |- B"
provable (MLL, classical, focus engine)
─────── ax   ─────── ax
⊢ ~A, A      ⊢ ~B, B
──────────────────── ⊗
  ⊢ ~A, A ⊗ ~B, B
$ deduxit prove "A |- B"
unprovable (MLL, classical, focus engine): ~A occurs 1 more time than A in the one-sided sequent, so they cannot all meet in axioms
```

The verdict, the fragment the sequent lives in and the engine that
searched, then the proof as a derivation tree of the one-sided sequent
calculus, which the library's independent checker has validated, or
why there is none. `deduxit --help` and `deduxit <command> --help`
document every option.

## Where to go next

- **The manual**,
  [deduxit.github.io/deduxit/manual](https://deduxit.github.io/deduxit/manual/),
  for `main`: [the commands](https://deduxit.github.io/deduxit/manual/commands.html)
  and their output formats, interactive proving, cut and its
  elimination, proof nets, first-order and ordinary logic, proofs as
  programs, many
  sequents in one call,
  [the benchmarks and the comparison](https://deduxit.github.io/deduxit/manual/benchmarks.html)
  with the other provers, what exists and what is planned, the
  architecture and development. A release's manual is the `docs/`
  directory of its tag.
- **The web page**, [deduxit.github.io](https://deduxit.github.io/):
  proving step by step in the browser, with lessons.
- **For AI models**: a guide to the command and the library, the Agent
  Skill [`skills/deduxit/`](https://github.com/deduxit/deduxit/tree/main/skills/deduxit)
  (a summary, `SKILL.md`, and the full reference beside it, whose every
  example a test runs), served as
  [llms.txt](https://deduxit.github.io/deduxit/llms.txt) and
  [llms-full.txt](https://deduxit.github.io/deduxit/llms-full.txt);
  copy the directory into an agent's skills (`~/.claude/skills/` for
  Claude Code) to give it the guide.
- **The Rocq library**, `rocq-deduxit`, in its own repository
  [deduxit/rocq-deduxit](https://github.com/deduxit/rocq-deduxit):
  the calculi, checkers and soundness theorems the certificates of
  `--kernel deduxit` are proved with.
- **The library's API**: [docs.rs/deduxit](https://docs.rs/deduxit) for the
  releases, [deduxit.github.io/deduxit](https://deduxit.github.io/deduxit/)
  for `main`, rebuilt on every push.
- **The changes of every release**:
  [CHANGELOG.md](https://github.com/deduxit/deduxit/blob/main/CHANGELOG.md).

deduxit's code, tests and documentation were written with substantial AI
assistance, by Anthropic's Claude models in Claude Code, in steps the
author planned, directed and accepted. Every proof the library returns
has passed its independent checker, and a Rocq certificate can be
checked by Rocq. deduxit is very much inspired by
[Click & coLLecT](https://www.click-and-collect.linear-logic.org);
feature requests and contributions are welcome.

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
