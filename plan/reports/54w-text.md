# Step 54w: every text a person reads, rewritten

The step's sessions each add a section here (`plan/54w-text.md`, "The
sessions"): 54w-r, the usability review; 54w-a, the web client; 54w-b,
the Rocq library; 54w-c, README, the manual, the changelog's Unreleased
section and the guide; 54w-d, the command's help and messages; 54w-e
and on, the crate's doc comments and comments.

## Session 54w-b: the Rocq library's text

Session 3 of `plan/54w-text.md` (Opus 5.5 at `high`, background job,
2026-11-09), in the Rocq library's jj workspace `../rocq-linlog-54w-b`
(workspace `54w-b`) on its `main` (`tymnvlzv`, "Date the release of
0.3.0"). Nothing was pushed by the session: after "rocq ready",
deduxit-orchestrator pushed the chain, and `main@origin` is `qlxkqlrx`
(`df4b15ec`).

### What changed

The chain on rocq-deduxit's `main`, oldest first (change ids; the
commit ids are the ones the signing hook gave last):

| change | commit | what |
|---|---|---|
| `xkvouuxw` | `7e3d6f76` | `.claude/rules/writing.md`, a copy of deduxit's writing rule: the four rules word for word, the scope and reader paragraphs the library's (README, the doc comments `rocq doc` renders, every comment of `theories/`, `tests/`, `bridges/`, the changelog, the opam description, `CITATION.cff`, the flake's comments); no `paths:`, so it loads at launch. CLAUDE.md points to it under "Conventions" and lists it under "Claude Code setup" |
| `wkupmzuo` | `613e9aa5` | the comments of the propositional calculi and checkers: `Formulas`, `Calculus`, `Proofs`, `Check`, `Soundness`, `Mix`, `Intuitionistic`, `IntuitionisticCheck` |
| `nmrpzlkk` | `696905fd` | the comments of `Refutations` and `Weights` |
| `nxvmzomo` | `5c7c5602` | the comments of the first-order development: `Terms`, `TermsCheck`, `FirstOrder*` |
| `tknulkwv` | `0c491e16` | the comments of every test certificate under `tests/` |
| `utmpnqwr` | `5045ce35` | the comments of the bridges |
| `yswkruxx` | `2edf73b9` | README rewritten for its two readers: "Check a certificate" (the command, the certificate it writes shown and explained, the files a statement rests on, what a certificate trusts) and "Prove with the library" (a lemma by the rules, one by the checker, one refutation, the modules); then what it covers, upgrading from rocq-linlog, the repository, citing, licence. The AI disclaimer of the opam description and `CITATION.cff` added, as deduxit's README has it and the writing rule expects |
| `krywunsw` | `c0fefabe` | the opam description: says what the library checks, and how |
| `qnrlnwzm` | `8fb760c6` | a wrong claim fixed (the reviewer's finding 3): `bang` and `excess` in `Refutations.v` |
| `qlxkqlrx` | `df4b15ec` | README's list of the files a first-order statement rests on completed (the reviewer's finding 4) |

The rewrite keeps every theorem, definition, name and proof script.
Comments say what an item is or does for the reader; the module headers
of the trusted files say "read it to know what a certificate claims";
lists in doc comments render as `rocq doc` lists (checked in the built
pages).

### Wrong or misleading claims fixed

- `FirstOrderFormulas.v`'s header said the first-order development
  "shares no name" with the propositional one; it reuses the names
  (`formula`, `ll`, `proof`, `check`, …), as `rocq.md` says. Now it says
  so, and that a file needing both imports one and qualifies the other.
- `Refutations.v`: `bang` was "nothing where the atom is absent, else
  unbounded" and `excess` "[None] where … the atom lies below an
  exponential". The code keeps `(0, 0)` whenever the interval inside is
  exactly zero, atom present or not (`excess "A" (oc (A ⊗ A⊥))` is
  `Some (0, 0)`, reproduced by the reviewer). Both now say that.
- README listed `Terms.v`, `FirstOrderFormulas.v`, `FirstOrderCalculus.v`
  (or `FirstOrderIntuitionistic.v`) as what a first-order statement rests
  on; the statements also name `Calculus.rules` and
  `Intuitionistic.irules`, so `Calculus.v` and `Intuitionistic.v` are
  now listed.
- The opam description said the certificates "check every proof rule by
  rule", which describes NanoYalla's kernel, not this library's checker
  by computation.
- `tests/FromCommand.v` said its certificates were "translated from
  their JSON form by a scratch program" (how the file was made); now it
  says what they are.

### The checks

| check | result |
|---|---|
| No code change: every `.v` file of `theories/`, `tests/`, `bridges/` compared with `main`, comments stripped, whitespace collapsed (`strip_compare.py`, a lexer that drops nested `(* … *)` outside string literals) | 34 of 34 identical, exit 0, at `qlxkqlrx` |
| The same with string literals kept verbatim (the reviewer's `strict.py`, which closes the first one's blind spot of whitespace inside strings) | 34 files, 0 differ, exit 0 |
| `make` in place (capped, cores 12–15) | exit 0, 20 modules |
| README's examples: the certificate `deduxit prove --mix --format rocq --standalone` writes, a classical one with `--kernel deduxit`, a refutation, and the README's own lemmas, each `rocq compile -Q theories Deduxit` | all accepted; the README's certificate block equals the command's output byte for byte (the reviewer) |
| `nix build .#doc` | exit 0; the lists render as `<ul class="doclist">` |
| `nix flake check --max-jobs 4 --cores 8` under the machine lock, capped on cores 12–15 | exit 0 at `krywunsw` before the review; exit 0 at `qlxkqlrx` (`df4b15ec`) after the fixes, "all checks passed!" |

### The review

The `fresh-reviewer` agent read the chain `main..krywunsw` against the
brief and the writing rules: four findings, all low, each answered.

1. `IntuitionisticCheck.v`'s header said "the terms below a node
   determine its sequent", where the library's "below" means toward the
   root. Fixed in `wkupmzuo`: "a node and the terms of its premises".
2. A proof comment in `FirstOrderIntuitionisticCheck.v` said a premise's
   eigenvariables "are the conclusion's, less the one its step
   introduces"; for two-premise rules they are only among them. Fixed in
   `nxvmzomo`: "among the conclusion's, except …".
3. `bang` and `excess` (pre-existing, kept by the rewrite). Fixed in
   `qnrlnwzm`.
4. README's list of files for first-order statements (pre-existing).
   Fixed in `qlxkqlrx`.

No finding on the other criteria: no code change, no remaining
reference to how the library was made, the README examples true, the
opam description true, the four rules copied word for word.

### Departures from the prompt

- The prompt names "the docs pages": the library has no hand-written
  pages, so they are the doc comments `rocq doc` renders (and the
  forwarding pages `modules/doc.nix` writes, whose one sentence was left
  as it is).
- Beyond README, the doc pages and the comments: the opam description
  (a wrong claim) was rewritten. `CITATION.cff`'s abstract and the
  changelog's released sections were left: they are the 0.3.0 release's
  record, and the changelog has no Unreleased section.
- README gained the AI disclaimer, worded as the opam description's.
- The comments of the flake's modules (`modules/*.nix`) and scripts were
  not reworded: the prompt names `theories/`, `tests/` and `bridges/`.
  The copy of the writing rule covers them from now on.

### What the next sessions must know

- The library's `.claude/rules/writing.md` is a copy of deduxit's: a
  change to the four rules belongs in both repositories. Its scope and
  reader paragraphs are the library's own.
- In the library's comments, "below" a node means toward the root and
  "above" toward the leaves (`a [⊤] above absorbs`); keep that when
  writing new ones.
- A comment must keep an even number of `"`: Rocq lexes strings inside
  comments, so an odd count breaks the file.
- To show that a text-only change touches no code, compare each `.v`
  file with the base after stripping comments: the two comparators of
  this session are in the job's scratch directory, gone with it; the
  method is a lexer that skips nested `(* … *)`, keeps string literals
  verbatim and compares the remaining tokens.
- `CITATION.cff`'s abstract is one long sentence; reword it at the next
  release, when its metadata changes anyway.
- `rocq.md` still says `excess` is `None` "under an exponential that
  contains the atom (`bang` keeps `(0, 0)` alone)"; that parenthesis is
  the accurate part.
