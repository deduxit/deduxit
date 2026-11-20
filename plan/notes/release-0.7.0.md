# The release of 0.7.0: notes, commands and checks

Written by session 73 (2026-11-20) for the planning session and the
author. The tag is `v0.7.0` in `deduxit/deduxit` alone: rocq-deduxit
has not changed since its 0.6.0 but for a comment, so it has no
release, and deduxit 0.7.0's certificates name rocq-deduxit 0.6.0
(`export::rocq::DEDUXIT` stays "0.6.0", which `modules/rocq-deduxit.nix`
asserts equal to the pinned opam file's version). The tags are held for
the author: the unattended yes of 2026-11-20 covered 0.6.0 alone.

## What 0.7.0 is

"Resolution and the simply typed lambda calculus" (D27): steps 70 to 72,
the last release the roadmap of 2026-11-08 plans, with the CI rework
(ci-speed) and 54x8's switch of invertible rules in the page's ordinary
tabs. On the page: the normal forms' and resolution's sheets (70c) and
the typed lambda calculus's page (71c), which ship with the client.

## The order of the tags and the publishing

1. **deduxit's chain on `main`**, pushed with `plan/push-main.sh @-`.
   No lock moves: rocq-deduxit stays at its `v0.6.0` commit.
2. **deduxit's CI**: `ci.yml` green on `main` at the commit to tag (the
   groups of `checks.yml`, each check pushed to the cache), and
   `kani.yml`'s last run on `main` green (`gh run list --repo
   deduxit/deduxit --workflow kani.yml`, a read).
3. **deduxit's tag**: the signed tag object `v0.7.0`, message "deduxit
   0.7.0", by the planning session after the author's yes.
4. **The release workflow**, the first through the new path:
   - `tag` checks the tag against the manifests, the changelog and
     `CITATION.cff` (run locally on 2026-11-20: passed);
   - `checks` calls `checks.yml` on the tag's commit, which fetches
     what `main`'s run pushed to the cache, so it is a lookup if the tag
     comes after `main`'s run ended, and builds what is missing if not;
   - `packages` (`packages.yml`) builds the archives and the Linux
     packages beside the checks;
   - `publish` waits for `tag` and `checks`, then for the author's
     approval in the environment `crates-io`;
   - `release` waits for all three and makes the GitHub release.
5. **After**: `install.yml` runs after the release and installs the
   release's own files (no longer building them again), asking for the
   release with the job's token; watch that its first run that way is
   green. The client's input `deduxit` moves to the tagged commit; the
   Zenodo check.

## The GitHub release notes

The workflow's job `release` titles it `deduxit 0.7.0`; the notes are
`CHANGELOG.md`'s section `## [0.7.0] - 2026-11-20` (to the next `## [`),
then the line naming the manual's Installation chapter and `SHA256SUMS`.
Attached: `deduxit-0.7.0.crate` and `deduxit-cli-0.7.0.crate` from
static.crates.io, the archives and Linux packages of `packages.yml`,
`wire.schema.json` (now with levels 21 to 24) and `rules.schema.json`,
all in `SHA256SUMS`, each attested. The text the workflow will write is
at the end of this file.

A change of the text is a change of the changelog before the tag; after
it, `gh release edit v0.7.0 --notes-file …` (an administrative act, the
author's yes first).

**The date.** The changelog and `CITATION.cff` say 2026-11-20. If the
tag is made on another day, the planning session moves both to that day
before tagging (the workflow checks only that the section is dated and
the citation states the version).

## What a user can do now

For an announcement:

- Rewrite a classical sequent into a normal form, one law per step,
  each form checked by the SAT solver: `deduxit seq normalize --to cnf
  '~(a -> b /\ c)'`, or the clauses of its negation by Tseitin's or
  Plaisted and Greenbaum's encoding (`--to tseitin`).
- Refute those clauses by resolution or DPLL, every refutation checked
  step by step: `deduxit resolve 'a -> b, b -> c |- a -> c'`, `--method
  dpll` with every step printed, or `--dimacs FILE`. Check an outside
  SAT solver's DRAT or LRAT proof: `deduxit check --dimacs f.cnf
  f.drat`.
- Check a lambda-term you write, in the simply typed, the linear or the
  affine calculus: `deduxit term check '\f:a -> b. \x:a. f x'`.
- Find a term of a type, or see why it has none: `deduxit term inhabit
  'a /\ b -> b /\ a'`; list and count a type's terms up to βη-equality:
  `deduxit term enumerate '(a -> a -> b) -> a -> a -> b'` (exactly 4),
  the Church numerals' type infinitely many.
- Grade an answer to a problem in one call: `deduxit grade 'a -o b, a
  |- b' < answer.txt`, a session script, a proof or an outcome, a
  countermodel, a tableau or a derivation; and a dataset of 843
  problems with verified answers under `dataset/`, for evaluating
  models.
- In the library: `ordinary::normal`, `ordinary::resolution`,
  `lambda::Term::parse` and `check`, `lambda::inhabit` and `enumerate`;
  in the protocol, the normal forms, `ordinary.resolve` and the suite
  `lambda`.
- On the page: the normal forms stepped through, resolution as a list
  and a graph, DPLL with its trace, a refutation by hand, the typed
  lambda calculus's page, and the switch of invertible rules in LK and
  LJ tabs. The page sends nothing.

## Before the tag

- 71c is on `main` (`twpnvsto`) and folded into the changelog's 0.7.0
  section: the typing derivation's drawing under "Added", the SVG
  subscripts under "Changed", the protocol's `lambda.draw` and
  `lambda.export` and the bindings' lambda calls beside 71b's.
- `ci.yml` green on `main` at the commit to tag; `kani.yml`'s last run
  on `main` green.
- The client's `main` has the page's 0.7.0 part (70c, 71c, 54x8) and
  moves its input `deduxit` once this chain is pushed.
- The web module is 3 846 685 bytes at gzip with 71c's calls, under
  the ceiling of 3 900 000 (71b raised it); 53 KB are left.

## After the tag

- The publish job asks the author's approval in the environment
  `crates-io`. `deduxit` and `deduxit-cli` exist on crates.io with their
  trusted publishers, so the job publishes both through Trusted
  Publishing; `linlog` and `linlog-cli` are skipped, since crates.io has
  their 0.3.0; no secret is used (the selection, run locally on
  2026-11-20: `new=` none, `existing=deduxit deduxit-cli`).
- The release job attaches `wire.schema.json` and `rules.schema.json`;
  check that the release lists both and that `SHA256SUMS` names them.
- `install.yml`'s run after the release: green, on the release's own
  files.
- The planning session moves the client's input `deduxit` to the tagged
  commit, and checks the live site and docs.rs.
- The Zenodo check, as for 0.3.0 (`plan/notes/release-0.3.0.md`, "After
  the tags: the Zenodo check"), with `v0.7.0`: a record titled `deduxit`
  under 10.5281/zenodo.23448588.
- **opam waits** (the author, 2026-11-09, until 0.8.0 or later).

## The author's checklist

Before the tag:

- [ ] Nothing to set: the trusted publishers of `deduxit` and
  `deduxit-cli` name `deduxit`/`deduxit`/`release.yml`/`crates-io`, and
  no secret is needed. Confirm on crates.io that both still list
  theirs.
- [ ] Say yes to the tag of 0.7.0, once CI is green at the release
  commit.

After the tag (the planning session makes it, a signed tag object):

- [ ] Approve the job `cargo publish` in the environment `crates-io`.
- [ ] The Zenodo check above, if the planning session has not reported
  it.

## The notes as the workflow will write them


Resolution and the simply typed lambda calculus. deduxit rewrites a
classical sequent into negation, conjunctive or disjunctive normal form,
or into clauses by Tseitin's or Plaisted and Greenbaum's encoding, one
law per step, and checks the result with its SAT solver. It refutes the
clauses of a sequent's negation by resolution or by DPLL, checks every
refutation step by step and every model clause by clause, and checks an
outside SAT solver's DRAT or LRAT proof. `term check` reads a term of
the simply typed, the linear or the affine lambda calculus and checks
its type; `term inhabit` finds a term of a type, read back from a
derivation and checked, or shows that it has none; `term enumerate`
lists and counts a type's terms up to βη-equality. A well-typed term's
typing derivation is drawn as a tree in every export. `grade` grades an
answer to a problem in one call, and a dataset of 843 problems with
verified answers comes with it, for evaluating models. On the web page,
a classical proof's normal forms are stepped through one law at a time,
its clauses are refuted by resolution, as a list and as a graph, by
DPLL or by hand; a lambda-term is checked and its typing derivation
drawn, and a type's terms are found and counted; and the switch of
invertible rules acts in LK and LJ tabs too.

### What breaks, and what to do instead

- **A variable of a `lambda::Term` has a name**, an identifier, where it
  had a letter and an index: `lambda::Variable::name` replaces `letter`
  and `index`, and `Variable` is no longer `Copy` (clone it, or keep its
  `VarId`). In JSON a variable is `{"type": 0, "name": "x"}` where it
  was `{"type": 0, "letter": "x", "index": 0}`. Digits at a name's end
  still print as an index (`x1` is `x₁`).

Nothing else that 0.6.0 offers was removed or changed: its other calls,
types, command lines, error codes and JSON forms are as they were. Every
new variant or field belongs to a `#[non_exhaustive]` type, so a
program's patterns go on compiling. What a program may notice is under
"Changed".

### Changed

- With `--lambda-types` (`lambda.types`), `inl` and `inr` now show the
  sum type they give, as `false` and `abort` show theirs, and a
  subscript that is no atom or constant is in brackets:
  `inl_(a ⊕ b)(x)`, `abort_(a → b)(z)`. A term printed so reads back.
  In SVG a subscript and a variable's index are set lowered, where
  `_` and subscript digits, which the math fonts lack, were written.
- The value `--lambda` adds to an outcome in JSON has the key `version`
  (23), as the term's own document.
- `protocol::suites()` lists five suites: `lambda` joins them.
- The help of `check` names `--dimacs` and `--proof-format`, and the
  list of commands names `resolve`, `grade` and `term`.

### Added

- Normal forms of classical sequents, every step shown and the result
  checked: `seq normalize --to nnf|cnf|dnf|tseitin|plaisted-greenbaum`,
  CNF by default. Negation, conjunctive and disjunctive normal form are
  made by equivalence laws, one law per step; Tseitin's and Plaisted and
  Greenbaum's encodings give the clauses of the sequent's negation, a new
  atom for each compound subformula. The SAT solver checks that each
  formula is equivalent to its form, or that the clauses can be
  satisfied exactly when the sequent can be false. `--form-limit`
  (`limits.form_occurrences`, 10000 by default) bounds the form.
- In the library, `ordinary::normal` (`rewrite`, `normalize`,
  `NormalForm` with its `steps` and `checks`), `Limits::form_occurrences`
  with `Refusal::Form` (code `form_too_large`) and `Phase::Rewrite`.
- In the protocol, the operations `ordinary.nnf`, `ordinary.cnf`,
  `ordinary.dnf`, `ordinary.tseitin` and `ordinary.plaisted_greenbaum`,
  with the promises `equivalent` and `equisatisfiable`, so a workspace
  takes a normal form as an edge.
- Resolution and DPLL at teaching scale: `resolve SEQUENT` turns the
  sequent's negation into clauses (`--clauses cnf|tseitin|plaisted-greenbaum`),
  or `resolve --dimacs FILE` reads them, and refutes them by saturation
  or with `--method dpll`, which prints every step; `--clause-limit` and
  `--step-limit` bound the search. Every refutation is checked step by
  step and every model clause by clause. `check` checks a refutation in
  JSON, and `check --dimacs FILE PROOF` an outside SAT solver's DRAT or
  LRAT proof.
- In the library, `ordinary::resolution`: `Clauses`, `Refutation` (built
  by hand with `resolve`, checked with `check`), `decide`, `clauses_of`
  and, with `sat`, `check_proof`; the errors `InvalidRefutation` (code
  `invalid_clause_refutation`) and `ClauseFile` (code `clause_file`). In the
  protocol, the operation `ordinary.resolve` and the kind
  `ordinary.resolution`.
- The library type-checks lambda-terms. `lambda::Term::parse` reads a
  term of the simply typed, the linear or the affine lambda calculus
  from text, in Church's style (every `λ` with its variable's type), as
  a term, a term with its type or a whole judgement `x : a ⊢ M : b`.
  `lambda::Term::check` checks it and returns the type of every subterm
  (`lambda::Typing`): the types fit, every variable is used as the
  calculus allows, and the term has the type claimed. A term that is
  not well typed is refused with `Error::IllTyped` (code `ill_typed`),
  which says why and where. The terms read back from proofs are checked
  the same way.
- A lambda-term is a JSON document of its own, at wire level 23, which
  `wire::upgrade` reads.
- `deduxit term check TERM` checks a lambda-term and prints its
  judgement with every type, or why it is ill typed (exit 1);
  `--calculus` picks the simple, the linear or the affine calculus.
- Inhabitation of simple types. `lambda::inhabit` (`deduxit term
  inhabit`) decides whether a type, an ordinary formula, has a term, and
  gives the term read back from the derivation and checked, or says that
  it has none. `lambda::enumerate` (`deduxit term enumerate`) lists a
  type's terms up to βη-equality, as long normal forms, and counts them:
  exactly, infinitely many, or at least as many as a search within
  `lambda::InhabitOptions` met; every term listed is checked. It counts
  the types built from atoms, `→`, `∧`, `↔` and `⊤`, and refuses one with
  `∨` or `⊥` with `Error::NotEnumerable` (code `not_enumerable`).
- The protocol has a suite `lambda`: `lambda.check` (a term to its
  typing, promise `well_typed`), `lambda.inhabit` (an intuitionistic
  statement to `lambda.inhabitation`, promise `decides`),
  `lambda.enumerate` (to `lambda.inhabitants`, promise `well_typed` of
  every term) and `lambda.duplicate`; `protocol::statement("lambda", …)`
  makes a term's statement from its text.
- `lambda::Calculus` has its names (`ALL`, `NAMES`, `name`, `FromStr`).
- A well-typed lambda-term's typing derivation is drawn in every export:
  `lambda::Drawing::new` checks the term and lays its derivation out,
  each inference a judgement `Γ ⊢ M : A` under the rule of natural
  deduction its term stands for, and LaTeX, Typst and SVG take it as
  they take a proof (`export::Drawable::Lambda`). In the linear calculi
  each premise's context holds the variables it uses. Rocq refuses it
  with `rocq::Unsupported::Typing`. The protocol's `lambda.draw` and
  `lambda.export` draw a checked term's derivation or write it in LaTeX
  or Typst.
- `deduxit grade SEQUENT` grades an answer to a problem, read from
  standard input or `--answer FILE`, by its shape: a session script that
  must finish the proof by its own steps (`close` and `invertible`
  refused), a proof or an outcome as
  `prove --format json` writes it, a Kripke model that refutes the
  sequent, a signed tableau, or a derivation in a calculus given as
  data. The answer must be about the problem's sequent, in its logic.
  The exit status is 0 for correct, 1 for incorrect, 3 for unknown;
  `--format json` writes the verdict as one object.
- A dataset of problems with verified answers for evaluating models,
  `dataset/problems.jsonl.gz`: 843 sequents of linear, ordinary and
  modal logic, each with the verdict deduxit decided and the answer
  `grade` checked, in the fields evaluation harnesses read (`id`,
  `input`, `target`, `metadata`), with its card, Croissant metadata and
  the script that builds it. `deduxit-bench instances` prints the
  generated families' instances it reads.
- In the web bindings, `ordinary_normalize`, `ordinary_resolve`, and a
  refutation by hand: `ordinary_clauses`, `ordinary_resolve_step` and
  `ordinary_refutation_check`; in the library `Refutation::check_steps`,
  which checks a refutation in progress. The lambda calls read and check
  a term (`lambda_read`, `lambda_check`), draw and export its derivation
  (`lambda_draw`, `lambda_export`), read a type and find, list and count
  its terms (`lambda_read_type`, `lambda_inhabit`, `lambda_enumerate`).

### Wire forms

- Level 21 is a classical sequent's normal form with its steps and the
  claims of its promise decided (`seq normalize --format json`), written
  only.
- Level 22 is a refutation by resolution, read and written, and a
  clause set decided (`resolve --format json`), written only.
- Level 23 is a lambda-term (`term check --format json`, and the value
  `--lambda` adds to an outcome), read and written.
- Level 24 is a type's terms listed and counted, and whether a type has
  a term (`term enumerate` and `term inhabit --format json`), written
  only.
- `wire::LEVEL` is 24. Every document of levels 1 to 20 reads and is
  written as before; 0.6.0 refuses levels 21 to 24 by name.


Install it as the manual's [Installation](https://deduxit.github.io/deduxit/manual/installation.html) says; `SHA256SUMS` holds the files' sums.
