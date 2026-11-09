# The release of 0.3.0: notes, commands and checks

Written by session 53b (2026-11-09) for the planning session and the
author. The tags are `v0.3.0` in `deduxit/rocq-deduxit` first, then in
`deduxit/deduxit` (`plan/53-release-0.3.0.md`, item 6); this file says
what each release's notes will say, what to run after the tags, and
what the author checks.

## The GitHub release notes

Both release workflows write the notes themselves from the tagged
tree, so the drafts below are the text they will produce, generated
from the changelogs at the commits to tag by the workflows' own `awk`:

- **deduxit** (`.github/workflows/release.yml`, job `release`): the
  title `deduxit 0.3.0`; the notes are `CHANGELOG.md`'s section
  `## [0.3.0] - 2026-11-09` (from its first line to the next `## [`),
  then the line naming the manual's Installation chapter and
  `SHA256SUMS`. Attached: `deduxit-0.3.0.crate` and
  `deduxit-cli-0.3.0.crate` from static.crates.io, the archives and
  Linux packages of `packages.yml`, `SHA256SUMS`, each attested.
- **rocq-deduxit** (its `release.yml`): the title `rocq-deduxit 0.3.0`;
  the notes are its `CHANGELOG.md`'s section `## [0.3.0] - 2026-11-09`,
  then the line `The source archive for opam: …, sha512=…` with the
  tag archive's SHA-512, which the opam submission takes.

A change of either text is a change of that changelog before the tag;
after it, `gh release edit v0.3.0 --notes-file …` (an administrative
act, the author's yes first).

The generated texts are at the end of this file.

## After the tags: the opam submission

The change `mmsqlyzk` "Add rocq-deduxit.0.3.0" in
`~/Projects/own/rocq-opam` holds
`released/packages/rocq-deduxit/rocq-deduxit.0.3.0/opam` with a
placeholder checksum (128 zeros under a comment) and the tag
`"date:2026-11-02"`. Once the library's release workflow has published
the release (its notes give the SHA-512), the planning session runs, in
zsh:

```sh
cd ~/Projects/own/rocq-opam
f=released/packages/rocq-deduxit/rocq-deduxit.0.3.0/opam
url=https://github.com/deduxit/rocq-deduxit/archive/refs/tags/v0.3.0.tar.gz
# The archive's SHA-512, computed here and read from the release notes:
# the two must be equal.
sum=$(curl --silent --fail --location "$url" | sha512sum | cut -d ' ' -f 1)
notes=$(curl --silent --fail https://api.github.com/repos/deduxit/rocq-deduxit/releases/tags/v0.3.0 \
  | python3 -c 'import json,sys,re; print(re.search(r"sha512=([0-9a-f]{128})", json.load(sys.stdin)["body"]).group(1))')
[ "$sum" = "$notes" ] && echo "sha512 agrees" || echo "sha512 DIFFERS: $sum vs $notes"
# Edit the change in place: the checksum, the placeholder's comment, and
# the date tag the library's own opam file states (2026-11-09).
jj edit mmsqlyzk
python3 - "$f" "$sum" <<'EOF'
import re, sys
path, digest = sys.argv[1], sys.argv[2]
text = open(path).read()
text = re.sub(r"  # A placeholder:.*?\n(?=  checksum:)", "", text, flags=re.S)
text = text.replace("sha512=" + "0" * 128, "sha512=" + digest)
text = text.replace('"date:2026-11-02"', '"date:2026-11-09"')
open(path, "w").write(text)
EOF
jj diff
# The lint the archive's CI runs, from the library's opam shell:
nix develop ~/Projects/own/rocq-linlog#opam -c opam lint --check-upstream "$f"
jj describe -m "Add rocq-deduxit.0.3.0

The Rocq library of deduxit, under the logical root Deduxit, its first
submission: the archive never gets rocq-linlog, which it was named up
to 0.2.0."
jj new
```

Before the pull request, as for 0.2.0, install the package from the
submitted file into a fresh switch for each admitted Rocq version: in
the library's checkout, `nix develop .#opam -c scripts/opam-install.sh
9.0.1 9.1.1 9.2.0 9.3.0` (network, about an hour, in a capped unit),
and note the versions it installed for the pull request's "Checked"
line. Then the author's steps in `plan/handoff.md` ("The opam pull
request"): the fork, the push of `rocq-deduxit.0.3.0=mmsqlyzk`, the
pull request.

### The pull request's text

Title: `Add rocq-deduxit.0.3.0`. Body:

> Adds `rocq-deduxit.0.3.0`, the Rocq library of
> [deduxit](https://github.com/deduxit/deduxit), a collection of logic
> suites whose prover of classical and intuitionistic linear logic
> exports its proofs as Rocq scripts checked against this library. Under
> the logical root `Deduxit`, `Deduxit.Linear` holds the one-sided
> sequent calculus of linear logic (with Mix, weakening and cut as
> parameters), the two-sided calculus of intuitionistic linear logic and
> its affine variant, their first-order versions, checkers for
> deduxit's proof terms with the theorems that an accepted term proves
> its sequent, and certificates that a sequent is not derivable. No
> theorem rests on an axiom: each one's assumptions are closed under
> the global context. Up to 0.2.0 it was `rocq-linlog`, never submitted
> here.
>
> - Source: the GitHub archive of the tag `v0.3.0`, with the SHA-512
>   its release notes state.
> - Depends on `rocq-core` (>= 9.0, < 9.4) and `rocq-stdlib`; built
>   with `make`.
> - Checked: `opam lint --check-upstream`, and installed from this
>   branch into a fresh switch for each of `rocq-core` VERSIONS (the
>   versions `scripts/opam-install.sh` installed).
> - Documentation: https://deduxit.github.io/rocq-deduxit/ ; archived
>   on Zenodo, https://doi.org/10.5281/zenodo.23899109.
> - The library, like deduxit, was written with substantial AI
>   assistance (Anthropic's Claude models in Claude Code), as its
>   description says, in steps I planned, directed and accepted.

## After the tags: the Zenodo check

Zenodo's GitHub integration archives each GitHub release as a new
version of the repository's concept, titled from `CITATION.cff`: from
0.3.0 "deduxit" and "rocq-deduxit" (0.1.0's and 0.2.0's records keep
"linlog" and "rocq-linlog"). The integration follows a repository by
its GitHub id, which a rename keeps, so each new version should join
its concept. To check, an hour after each release is published (zsh,
no login):

```sh
for concept in 23448588 23899109; do
  curl --silent --fail "https://zenodo.org/api/records?q=conceptrecid:$concept&all_versions=true&size=20" \
    | python3 -c 'import json,sys
for h in json.load(sys.stdin)["hits"]["hits"]:
    m = h["metadata"]
    print(h["id"], h["conceptrecid"], m["title"], m.get("version"), m["publication_date"])'
done
```

- **Expected**: under 23448588 a record titled `deduxit`, version
  `v0.3.0`, above 0.2.0's (record 23899551); under 23899109 one titled
  `rocq-deduxit`, version `v0.3.0`, above 0.2.0's (23899110).
  `https://doi.org/10.5281/zenodo.23448588` and `…23899109` then
  resolve to the new records.
- **If a version is missing after a day**: the hook did not fire. In
  Zenodo, Account → GitHub, check that `deduxit/deduxit` (or
  `deduxit/rocq-deduxit`) is listed and switched on under its new
  name, and that the release shows there; if it failed, the page names
  the error. Making the version by hand also works: on the 0.2.0
  record, "New version", upload the release's source archive, set the
  version `v0.3.0` and the title from `CITATION.cff`, publish.
- **If a new version started a new concept** (its `conceptrecid` is
  not 23448588 or 23899109; the search for the right concept lacks it,
  so search by title, `q=title:deduxit`): the citation file's DOI and
  README's badge name the old concept, so the stray record must join
  it. A published record cannot be moved or deleted by its owner; ask
  Zenodo's support (zenodo.org/support, "Technical support") to move
  record N into concept 23448588 (or 23899109) as a new version, or to
  delete it once the author has made the version by hand under the
  right concept as above. Until then nothing cites the stray DOI.

## The organisation's profile README

For the planning session to create (`deduxit/.github`, a new public
repository, the author's yes first), the file `profile/README.md`:

```markdown
<picture><source media="(prefers-color-scheme: dark)" srcset="https://deduxit.github.io/logo/wordmark-dark.svg"><img alt="deduxit" src="https://deduxit.github.io/logo/wordmark-light.svg" height="64"></picture>

> **dēdūxit** [deːˈduːk.sɪt] *Latin, verb*: "it has derived", the perfect of *dēdūcō*, to lead down, derive.

Logic suites, linear logic first: a command line program and a Rust
library that decide, check, prove step by step and export sequents of
classical and intuitionistic linear logic, and ordinary propositional
logic through its embeddings.

- [deduxit](https://github.com/deduxit/deduxit): the command `deduxit`
  and the crates `deduxit` and `deduxit-cli`; its
  [manual](https://deduxit.github.io/deduxit/manual/).
- [deduxit.github.io](https://deduxit.github.io/): prove step by step
  in the browser.
- [rocq-deduxit](https://github.com/deduxit/rocq-deduxit): the Rocq
  library its certificates are checked against.

Up to version 0.2.0 deduxit was called linlog.
```

## The author's checklist

Before the tags:

- [ ] Verify on crates.io that `linlog` and `linlog-cli` each have a
  trusted publisher naming owner `deduxit`, repository `deduxit`,
  workflow `release.yml`, environment `crates-io` (the author set it
  after the rename; crates.io matches the repository by name, so the
  old `linlog-prover`/`linlog` entry matches nothing).
- [ ] The environment `crates-io` of `deduxit/deduxit` holds the secret
  `CRATES_IO_FIRST_PUBLISH_TOKEN` (the token "deduxit first release",
  scopes `publish-new` and `publish-update`, crates `deduxit` and
  `deduxit-cli`).

After the tags (the planning session makes them):

- [ ] Approve the job `cargo publish` in the environment `crates-io`.
  It publishes `deduxit` and `deduxit-cli` by the secret, then `linlog`
  and `linlog-cli` 0.3.0 by Trusted Publishing; a rerun after a failure
  finishes what is left.
- [ ] Add Trusted Publishing to `deduxit` and `deduxit-cli` on
  crates.io: owner `deduxit`, repository `deduxit`, workflow
  `release.yml`, environment `crates-io`.
- [ ] Delete the secret `CRATES_IO_FIRST_PUBLISH_TOKEN` from the
  environment, and revoke the token "deduxit first release" on
  crates.io.
- [ ] The Zenodo check above, for both releases.
- [ ] The opam pull request, after the planning session has filled in
  the checksum and run the installs (above).
- [ ] The organisation's profile README, if wanted (above).

## The notes as the workflows will write them

Generated on 2026-11-09 from the commits to tag, by the workflows' own `awk`; the checksum line of rocq-deduxit's notes is filled in by its workflow from the tag's archive.

### deduxit 0.3.0

````markdown

The third release, the first under the name deduxit: a collection of
logic suites, linear logic first, its linear suite now the module
`deduxit::linear`. Ordinary logic is proved step by step in LK and LJ,
intuitionistic proofs read back as lambda-terms, the one-sided count
and the state equation are checked without the search and certified in
Rocq, the inverse method takes over wide multiplicative sequents where
the focused engine runs out of memory, and the mark dēdūxit replaces
linlog's logo.

### What breaks, and what to do instead

Found by tools, not by memory: `cargo semver-checks` against the
published `linlog` 0.2.0 and `cargo public-api`'s diff of the two public
APIs with every feature (the devshell's `breaks 0.2.0`, which names the
old package deduxit to compare it), and, for the command, the help and
outputs the behaviour lock pins at 0.2.0's tag and now. Of the diff's
items, all but `cut::eliminate` and `ordinary::Rule::Open` below are
the rename and the move under `deduxit::linear`; every other change is
an addition.

- **The crates, the command and the library are renamed deduxit.** The
  library is the crate `deduxit` (was `linlog`) and the command the
  package `deduxit-cli` with the binary `deduxit` (was `linlog-cli` and
  `linlog`), with the same verbs and flags. Depend on `deduxit = "0.3"`
  and install `cargo install --locked deduxit-cli`; the flake's package
  is `deduxit-cli`, its default program `deduxit`. The last releases of
  the old names, `linlog` 0.3.0 and `linlog-cli` 0.3.0, point here:
  `linlog` re-exports `deduxit` (its root and `deduxit::linear`, so
  0.2.0's paths still name their items), and `linlog-cli`'s binary
  `linlog` says once on standard error that it is now `deduxit` and runs
  it. Neither is released again.
- **The linear suite is the module `deduxit::linear`.** The items 0.2.0
  had at the root and its modules `sequents`, `occurrences`, `fragment`,
  `proofs`, `nets`, `search`, `lltp`, `mist` and `families` are under
  `deduxit::linear` (`deduxit::linear::{Sequent, Mode, Options, prove}`,
  `deduxit::linear::search::…`); the error family, the limits and
  progress, the settings, the exports and the wire forms stay at the
  root (`deduxit::{Error, Limits, Progress, Settings}`,
  `deduxit::export`, `deduxit::wire`), beside `deduxit::ordinary` and
  `deduxit::lambda`.
- **Written under the new name, the old still read**:
  - an outcome names its writer's version under `"deduxit"` (was
    `"linlog"`); a file 0.2.0 wrote reads and checks as before;
  - the settings values `deduxit` of `rocq.kernel`, `typst.layout` and
    `svg.net_look` (was `linlog`: deduxit's Rocq library, its own Typst
    layout and its own look of a net); a settings file that says
    `linlog` still loads, and so does `--style KEY=linlog`, which reads
    its value as the settings file does; the flag `--kernel` takes the
    new name only.
- **The Rocq library is `rocq-deduxit`**, logical root `Deduxit` with
  linear logic under `Deduxit.Linear` (was `rocq-linlog`, `Linlog`): a
  certificate for the kernel `deduxit` (`--kernel deduxit`, was
  `--kernel linlog`) imports `From Deduxit.Linear Require …`, and
  compiles against `rocq-deduxit` 0.3.0. `export::rocq::Kernel::Deduxit`
  and the constants `export::rocq::DEDUXIT` and `DEDUXIT_*_PRELUDE` were
  `Kernel::Linlog`, `LINLOG` and `LINLOG_*_PRELUDE`; `typst::Layout::Deduxit`,
  `svg::NetPreset::Deduxit` and `svg::NetLook::DEDUXIT` were `Linlog`
  and `LINLOG`.
- **`linear::proofs::cut::eliminate` takes the first step's cut**, `at`, an
  `Option<NodeId>` after the mode (`None` the strategy's, as before), as
  `cut::step` does; `deduxit eliminate --at NODE` sets it.
- **`ordinary::Rule` has the variant `Open`**, the rule of an open goal
  in a proof in progress (`ordinary::Interactive`); `ordinary::Rule` is
  a closed enum, so a `match` on it needs an arm for `Open`, which no
  finished derivation contains.
- **The command, where a script may see it**:
  - `check` and `eliminate` check a proof in the mode its file states;
    a mode flag that names another is an error (exit status 2) where
    0.2.0 checked in the flags' mode: drop the flag, or remove the
    file's `"mode"` to choose one;
  - a proof whose Rocq certificate is refused keeps its verdict and its
    exit status, with `no certificate: …` on standard error, where 0.2.0
    exited 2; a session's `proof FILE.v` then makes no file;
  - a batch names each Rocq certificate's file by Rocq identifiers
    (`modus_ponens.v`, `entry_1_1_p.v`), and every lemma is
    `certificate`, which the module qualifies;
  - `prove --logic` draws a derivation of LK or LJ with each weakening
    and contraction left out where it is admissible (`--structural
    admissible`, the default): `--structural explicit` draws every one,
    as 0.2.0 did; a certificate names every structural rule either way.

Not breaks, since their types are `#[non_exhaustive]`, but new: the
variants `Error::NoTerm` and `Error::OrdinaryStep`,
`export::rocq::Unsupported::Exhausted` (after the others, so no
discriminant moved), and the fields `ViewOptions::structural`,
`Settings::lambda` and `lambda` of `latex::Options`, `typst::Options`
and `svg::Style`.

### Added

- **Ordinary logic step by step**: `ordinary::Interactive` (feature
  `interactive`), a goal state of two-sided ordinary sequents whose
  rules are the checker's, with `goals`, `rules`, `apply` (a `Step`, and
  for a rule of two premises a split of the context), `undo`, `close`
  (the goal decided through `ordinary::decide` and the derivation
  grafted), `hint` and `finished`, the derivation the checker of LK and
  LJ validates; `FromStr` for `ordinary::Rule`. The command's `interact
  --logic classical|intuitionistic|minimal SEQUENT`, with the linear
  session's commands where they mean the same and a formula named by
  side and position (`L0`, `R1`); the bindings' `ordinary_*` calls.
- **Weakening and contraction shown or left out**:
  `ViewOptions::structural` (`Structural::Admissible`, the default, or
  `Explicit`), the flag `--structural` and the settings key
  `view.structural`.
- **Proofs as programs**: `Derivation::term` reads a two-sided proof of
  intuitionistic linear logic back as a term of the linear lambda
  calculus of Benton, Bierman, de Paiva and Hyland (the affine one in
  affine mode, a cut the redex `(λx:A. N) M`), and
  `ordinary::Derivation::term` an LJ derivation as a simply typed term;
  `lambda::Term` with its JSON form (written only), printed under
  `lambda::Options` (`notation`, Unicode or ASCII, and `types`); the
  flags `--lambda`, `--lambda-notation` and `--lambda-types`, the
  settings key `lambda`, the session command `lambda`, the key `lambda`
  of an outcome's JSON, and the term under the conclusion in LaTeX,
  Typst, SVG, PNG and PDF. What has no term is refused with
  `Error::NoTerm` and its reason (`lambda::NoTerm`). The manual's
  chapter "Proofs as programs" states the calculus with its sources.
- **Refutations checked and certified**: `Disproof::check` accepts a
  one-sided count (`OneSided`, `OneSidedSymbol`) and a state equation
  (`StateEquation`) by the Rocq library's own weighted test, and the
  second kernel writes their certificates, the first-order counts
  included, against `rocq-deduxit` 0.3.0's weighted refutations;
  `deduxit check` reads an unprovable outcome and checks its
  refutation, with a proof's exit statuses.
- **The inverse method where the focused engine runs out of memory**:
  on a unit-free MLL sequent whose atoms occur once with each sign, in
  classical mode without Mix, the inverse method takes over before the
  net engine (`wide-m1` with 8 192 pairs proved in 3.05 s against
  9.48 s).
- **The command**: `check -f, --file PATH` beside the argument;
  `eliminate --at NODE` names the cut taken first; `--style`'s help
  lists the rules of a label table, and a dotted key such as
  `labels.table.RULE=LABEL` makes the table it names.
- **The last releases of the old names**: `linlog` 0.3.0 re-exports
  `deduxit` (its root and `deduxit::linear`, so 0.2.0's paths still name
  their items) and `linlog-cli` 0.3.0's binary `linlog` runs `deduxit`
  after a line on standard error.

### Wire forms

- Level 8 adds the forms of ordinary logic: an ordinary sequent, a
  derivation of LK or LJ, the outcome of deciding an ordinary sequent
  and a proof in progress of one, each always written at level 8;
  every other document is written as before. An
  outcome's version key is `"deduxit"`; a document of 0.2.0, whose key
  is `"linlog"`, reads as before.

### Changed

- **The mark**: the word dēdūxit with lapis long marks (CC BY 4.0,
  served at deduxit.github.io/logo/), in README, the manual and the
  rustdoc, replaces linlog's logo, the lollipop.
- **The addresses**: the repository is `deduxit/deduxit`, the manual
  and the API at deduxit.github.io/deduxit/, the web page at
  deduxit.github.io and the Rocq library at `deduxit/rocq-deduxit`;
  the old ones redirect.
- An essential net's input side is drawn in the blue of `⅋` (`#0072b2`,
  `#56b4e9` dark) by default, as the web page draws it, where it was
  green.
- `eliminate` names a cut of an invalid proof by its instance, as
  `check` does, where it named the raw id (`cut on #8`); `interact`'s
  `cut` explains an unquoted formula with blanks and says to quote it;
  the eigenvariable refusal names `∃L` among the steps that introduce
  one.

### Fixed

- A rule node's label in the rules look divided by zero when the node
  had no room.
- A forest past a proof structure's vertex bound (`nets::MOST`) made the
  net engines panic; it is refused in their `admits` and in
  `ProofStructure`'s constructor.


Install it as the manual's [Installation](https://deduxit.github.io/deduxit/manual/installation.html) says; `SHA256SUMS` holds the files' sums.
````

### rocq-deduxit 0.3.0

````markdown

The library renamed `rocq-deduxit`, under the logical root `Deduxit`,
and certificates of non-derivability where copies add literals, beside
deduxit 0.3.0 (linlog renamed), whose certificates for `--kernel
deduxit` it checks.

### Changed

- **Breaking:** the package is `rocq-deduxit`, the repository
  `github.com/deduxit/rocq-deduxit` and the documentation
  `https://deduxit.github.io/rocq-deduxit/`. The library has one logical
  root, `Deduxit`, with a namespace per calculus, and every module of
  0.2.0 is now under `Deduxit.Linear`: `From Linlog Require Import
  Soundness.` becomes `From Deduxit.Linear Require Import Soundness.`,
  and `Linlog.Calculus.ll` is `Deduxit.Linear.Calculus.ll`. No module,
  definition or theorem changed its own name; the files moved from
  `theories/` to `theories/Linear/`, and the package installs into
  `user-contrib/Deduxit/Linear`. The tests and the bridges to NanoYalla
  and Yalla, which are not installed, take the paths
  `Deduxit.Linear.Tests` and `Deduxit.Linear.Bridges`.
- `_RocqProject` maps `theories` with `-Q` instead of `-R`, as Rocq
  loads an installed library, so the build sees the paths a user sees.

### Added

- Weighted counts (`Weights.v`): an integer weight per atom, and atoms
  declared dead, whose literal of a sign `p` names contributes nothing a
  sum can cancel and whose other literal absorbs. Without weakening and
  without cut, Mix allowed, every derivable sequent can contribute zero
  to the weighted count or absorb, copies of `?` formulas included
  (`weighted_balance`), so weights that the test `overweight` accepts by
  computation refute a sequent (`weighted_refutation`, and two-sided
  `iweighted_refutation`). With the weight `1` on one atom this is the
  one-sided count, where every literal of the atom below an exponential
  has one sign; with a Farkas certificate of a Horn program's state
  equation and the atoms no firing reaches declared dead in the bodies'
  sign, it is the state equation's refutation.
- The same with weakening (`affine_balance`, `overweight_affine`,
  `affine_refutation`, `iaffine_refutation`) for sequents whose
  formulas have outputs as a Horn program's do (`outputs`, `shaped`):
  a derivation then drops only formulas without an output.
- The weighted count of first-order sequents without weakening
  (`FirstOrderWeights.v`), each literal weighed by its predicate or, for
  a predicate whose literals are all closed, by the literal; and the
  embedding of the first-order two-sided calculus in the one-sided one
  (`FirstOrderWeights.ill_ll`).
- `weight_of`, the weight a table gives an atom, for certificates.


The source archive for opam: `https://github.com/deduxit/rocq-deduxit/archive/refs/tags/v0.3.0.tar.gz`, `sha512=…`.
````
