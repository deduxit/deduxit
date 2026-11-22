# The release of 0.8.0: notes, commands and checks

Written by session 80 (2026-11-23) for the planning session and the
author. Two repositories release: rocq-deduxit 0.8.0 first, since
deduxit 0.8.0's certificates of first-order countermodels import its
new theorems, then deduxit 0.8.0, whose `export::rocq::DEDUXIT` names
it. The tags are held for the author: no unattended yes covers a tag.

## What 0.8.0 is

"First-order ordinary logic" (the author, 2026-11-15): steps 74 to 78
with the fixups beside them (13 to 25). First-order sequents of
classical, intuitionistic and minimal logic, with equality, read from
text, JSON and TPTP (`fof`, `cnf`); proved step by step in LK, LJ and
LM; decided by the connection provers, grounding and the embedding;
refuted by finite structures and Kripke models with growing domains,
drawn and certified in Rocq; prenex, miniscope, Skolem, Herbrand and
clausal forms; Herbrand's theorem; first-order resolution with
paramodulation; the negative translations and interpolants at first
order; `prove --szs`. The web page takes first-order logic after this
release (step 79, waiting for the author's redesign of the site, which
is frozen until 0.8.0 is out).

## The order of the tags and the publishing

1. **rocq-deduxit's release commit on its `main`**: `zxknsvsx` ("Date
   the release of 0.8.0", on `vxkypqzt`), pushed by the planning
   session with `plan/push-main.sh --rocq` (its `nix flake check`
   passed here on 2026-11-23). Its CI (`ci.yml`) green at that commit.
2. **rocq-deduxit's tag**: the signed tag object `v0.8.0` at that
   commit, message "rocq-deduxit 0.8.0", after the author's yes. Its
   release workflow checks the tag against the opam file, the citation
   file and the changelog, runs `nix flake check`, makes the GitHub
   release with the changelog's section and the SHA-512 of the source
   archive (what the opam archive's file needs), and publishes the
   documentation on Pages.
3. **deduxit's lock moves to the tagged commit**, in the release chain,
   just below the commit that names it:

   ```sh
   jj new suvnulxw                 # the report's change, below zpxsuouq
   nix flake update rocq-deduxit   # moves that input alone
   jj commit -m "flake.lock: Update" flake.lock
   jj rebase -s zpxsuouq -d @-     # "Name rocq-deduxit 0.8.0 in the certificates" on top of it
   jj abandon @                    # the empty change jj commit left beside it
   ```

   `zpxsuouq` sets `DEDUXIT` to "0.8.0" (with the Rocq chapter's
   versions and install lines and the guide's tag line), and the flake
   asserts it equal to the pinned library's opam version, so before the
   lock moves that commit fails `rocq-deduxit` at evaluation; after, it
   passes (checked here against the release commit by
   `--override-input rocq-deduxit git+file:///…?rev=e3eecb30…`:
   `rocq-deduxit` and `rocq-deduxit-library` built).
4. **deduxit's chain on `main`**, with `plan/push-main.sh zpxsuouq`
   (the head, which holds the lock's move; pushing the lock commit alone
   would leave `main` failing the same assertion), and
   `ci.yml` green there (the groups of `checks.yml`, each check pushed
   to the cache); `kani.yml`'s last run on `main` green (`gh run list
   --repo deduxit/deduxit --workflow kani.yml`, a read).
5. **deduxit's tag**: the signed tag object `v0.8.0`, message "deduxit
   0.8.0", after the author's yes.
6. **The release workflow**: `tag` (run locally on 2026-11-23: passed),
   `checks` (a lookup of what `main`'s run pushed), `packages` beside
   them, `publish` waiting for the author's approval in the environment
   `crates-io`, `release` after all three.
7. **After**: `install.yml` after the release, on the release's own
   files; the Zenodo check for both repositories; the web client's input
   `deduxit` stays where it is until the site's freeze ends (step 79,
   after the author's redesign).

## The GitHub release notes

The workflow's job `release` titles it `deduxit 0.8.0`; the notes are
`CHANGELOG.md`'s section `## [0.8.0] - 2026-11-23` (to the next `## [`),
then the line naming the manual's Installation chapter and `SHA256SUMS`.
Attached: `deduxit-0.8.0.crate` and `deduxit-cli-0.8.0.crate` from
static.crates.io, the archives and Linux packages of `packages.yml`,
`wire.schema.json` (now with levels 25 to 29) and `rules.schema.json`,
all in `SHA256SUMS`, each attested. The text the workflow will write is
at the end of this file.

A change of the text is a change of the changelog before the tag; after
it, `gh release edit v0.8.0 --notes-file …` (an administrative act, the
author's yes first).

**The date.** Both changelogs, both `CITATION.cff` files and the opam
file say 2026-11-23. If a tag is made on another day, the planning
session moves that repository's dates to that day before tagging (the
workflows check only that the section is dated and the files state the
version).

## What a user can do now

For an announcement (each command run with the release build on
2026-11-23):

- Prove a first-order sequent with a checked derivation: `deduxit prove
  --logic classical 'forall x. (p(x) -> q(x)), p(a) |- q(a)'`, in
  intuitionistic or minimal logic too, from text, JSON or a TPTP problem
  (`fof` and `cnf`, ILTP's 2 670 first-order problems).
- See why one is not valid: `deduxit prove --logic classical 'forall x.
  exists y. r(x,y) |- exists y. forall x. r(x,y)'` prints a checked
  structure of two elements; `deduxit prove --logic intuitionistic
  '~forall x. p(x) |- exists x. ~p(x)'` a checked Kripke model whose
  domains grow. Both draw as SVG, PNG and PDF and become Rocq
  certificates (`--format rocq`) for rocq-deduxit 0.8.0.
- Reason with equality: `deduxit prove --logic classical 'a = b, p(a)
  |- p(b)'` adds the axioms of `=` and shows them in the derivation.
- Rewrite a sequent into prenex, miniscope, Skolem, Herbrand or clausal
  form, each step checked: `deduxit seq normalize --to clausal 'forall
  x. exists y. r(x, y) |- exists y. r(a, y)'`.
- Prove one by Herbrand's theorem: `deduxit herbrand 'forall x. p(x) |-
  p(f(a))'` shows the ground instances the SAT solver refutes and the
  derivation of LK built from them.
- Refute its clauses by resolution with unification (and
  paramodulation for `=`): `deduxit resolve 'forall x. (p(x) -> q(f(x))),
  p(a) |- exists y. q(y)'`, read back as a checked derivation.
- Prove it step by step: `deduxit interact --logic classical`, with
  `∀L` and `∃R` taking a witness or leaving one open for an axiom to
  fill.
- Speak to the TPTP World: `prove --szs` ends with `% SZS status …`.
- In the library: `ordinary::connection`, `grounding`, `models`,
  `herbrand`, `equality`, `resolution::first_order`, the finite
  structures and Kripke models with growing domains; in the protocol,
  the first-order normal forms and `ordinary.instances`.

## Before the tags

- rocq-deduxit's release commit pushed and its CI green; the author's
  yes to its tag.
- `ci.yml` green on `main` at deduxit's commit to tag, which includes
  the lock's move and `zpxsuouq`; `kani.yml`'s last run on `main` green.
- fixup-25 (the clausal normal form within its bounds) and fixup-26
  (the connection provers' setup within its bounds): each writes its
  entry under `[Unreleased]`. One that lands before the tag is folded
  into 0.8.0's section (into the entry of the feature it bounds, both
  being new in 0.8.0) and the chain rebased onto it.
- The web module is 2 846 308 bytes at gzip (78b), under its ceiling of
  2 875 000; the `budget` check passed in this chain's full check
  (2026-11-23, 00:26 to 00:51, every check).

## After the tags

- The publish job asks the author's approval in the environment
  `crates-io`. `deduxit` and `deduxit-cli` exist on crates.io with their
  trusted publishers, so the job publishes both through Trusted
  Publishing; `linlog` and `linlog-cli` are skipped, since crates.io has
  their 0.3.0; no secret is used (the selection, run locally on
  2026-11-23: `new=` none, `existing=deduxit deduxit-cli`).
- The release job attaches `wire.schema.json` and `rules.schema.json`;
  check that the release lists both and that `SHA256SUMS` names them.
- `install.yml`'s run after the release: green, on the release's own
  files.
- The Zenodo check, as for 0.3.0 (`plan/notes/release-0.3.0.md`, "After
  the tags: the Zenodo check"), with `v0.8.0` for deduxit (a record
  under 10.5281/zenodo.23448588) and for rocq-deduxit (under
  10.5281/zenodo.23899109).
- **The opam submission of rocq-deduxit can go now** (the author,
  2026-11-09: "until 0.8.0 or later"). It needs: the tag `v0.8.0` and
  its GitHub release, whose notes carry the SHA-512 of
  `https://github.com/deduxit/rocq-deduxit/archive/refs/tags/v0.8.0.tar.gz`;
  the package file `released/packages/rocq-deduxit/rocq-deduxit.0.8.0/opam`
  in a fork of `rocq-prover/opam` (the opam file at the tag with a `url {
  src: … checksum: "sha512=…" }` section added; the draft in
  `~/Projects/own/rocq-opam` is for 0.3.0 and must be redone from the
  tag's file); `opam lint` and the install on every admitted Rocq
  (`nix develop .#opam -c scripts/opam-install.sh 9.0.1 9.1.1 9.2.0
  9.3.0` in rocq-deduxit, about an hour, network); and the pull request,
  which is the author's act from their account.

## The author's checklist

Before the tags:

- [ ] Nothing to set: the trusted publishers of `deduxit` and
  `deduxit-cli` name `deduxit`/`deduxit`/`release.yml`/`crates-io`, and
  no secret is needed. Confirm on crates.io that both still list
  theirs.
- [ ] Say yes to rocq-deduxit's tag `v0.8.0`, once its CI is green at
  the release commit.
- [ ] After the planning session has moved deduxit's lock to that tag
  and `ci.yml` is green on `main`, say yes to deduxit's tag `v0.8.0`.

After the tags (the planning session makes them, signed tag objects):

- [ ] Approve the job `cargo publish` in the environment `crates-io`.
- [ ] The Zenodo check above, if the planning session has not reported
  it.
- [ ] When you choose: the opam submission of rocq-deduxit (above).

## The notes as the workflow will write them


First-order ordinary logic. deduxit reads first-order sequents of
classical, intuitionistic and minimal logic, with equality, in its own
syntax and in TPTP's `fof` and `cnf`, and proves them step by step in
LK, LJ and LM, with the quantifier rules and witnesses an axiom can
fill. It decides them by a connection prover, by grounding for the
Bernays–Schönfinkel and monadic classes, and through the embedding into
first-order linear logic, each "valid" with a checked derivation (or,
from grounding, a checked clausal proof), and
refutes them by finite structures and Kripke models with growing
domains, each checked, drawn and certified in Rocq. It rewrites a
sequent into prenex, miniscope, Skolem, Herbrand and clausal form, each
step checked, proves it by Herbrand's theorem, resolves its clauses with
unification and paramodulation, translates it into intuitionistic logic
and finds its interpolants; `prove --szs` speaks the TPTP World's
status line. The web page takes first-order logic after this release.

### What breaks, and what to do instead

- **`ordinary::Node` has two variants more**, `Forall` and `Exists`, for
  the quantifiers of first-order formulas. A `match` over `Node` needs an
  arm for each.

- **`ordinary::Rule` has four variants more**, `ForallLeft`,
  `ForallRight`, `ExistsLeft` and `ExistsRight`, after `Open`.

Nothing else that 0.7.0 offers was removed or changed: its other calls,
types, command lines, error codes and JSON forms are as they were. The
term types `Symbol`, `FoTerm`, `FoTermId`, `Args`, `Eigen` and
`SymbolFault` are defined in one place for every suite now, and keep
every path they had (`linear::sequents::fo::FoTerm` among them). Every
other new variant or field belongs to a `#[non_exhaustive]` type
(`ordinary::Outcome`'s `structure` and `kripke` among them), so a
program's patterns go on compiling. What a program may notice is under
"Changed".

### Added

- **First-order formulas in ordinary logic**: predicates applied to
  terms and the quantifiers, `forall x. p(x) -> q(x), p(a) |- q(a)` in
  the native syntax and `! [X] : (p(X) => q(X))` in TPTP's. Formulas
  equal up to the names of their bound variables are one formula.

- **Equality in ordinary logic.** `s = t` and `s != t` (natively also
  `s ≠ t`) are read in the ordinary syntax and in TPTP, and printed
  `s = t`. A sequent with `=` is decided with the equality axioms of its
  symbols appended to its hypotheses (`ordinary::equality::with_axioms`;
  `ordinary::decide_within`, `prove --logic`, its batch and `interact
  --logic` do it themselves), so a valid one's derivation shows them as
  its last hypotheses (`equality::axiom_count`, and the verdict line
  counts them), and `grade` accepts a derivation that holds any of them
  beside the problem's own hypotheses. `resolve` and the other commands
  that read a sequent take it as written. A finite structure reads `=`
  as identity, and "not valid" comes only with such a checked structure.
  `Formulas::has_equality`
  says whether an arena uses `=`; in JSON it is the binary predicate
  named `"="`.

- **TPTP's `cnf`**: `read_tptp` and the command read `cnf(NAME, ROLE,
  CLAUSE).`, each clause the universal closure of its disjunction, the
  role `negated_conjecture` a hypothesis.

- **`include` in the command's TPTP problems**: `include('NAME').` is
  read from `$TPTP`, else the problem's directory or one above it; a
  missing file and an include of itself are refused by name.

- **TPTP names renamed on reading**: a name this syntax keeps for itself
  (`par`, `true`) gets `_` after it, and a function symbol named like a
  predicate `_f`, so that the sequent reads back.

- **`ordinary::tptp_text`** gives a TPTP file's text from its bytes for
  `read_tptp`: a byte that is not UTF-8 in a `%` comment, where some of
  ILTP's problems write an author's name in Latin-1, is read, and one
  anywhere else is `Error::Tptp` naming its line.

- **Linear sequents with `=`**, an ordinary sequent's image, print `s = t`
  and read back: the linear parser reads `=`, `!=` and `≠` between terms.

- **The quantifier rules of LK, LJ and LM**: the checker takes `∀L`,
  `∀R`, `∃L` and `∃R` with their eigenvariable conditions, each inference
  recording its witness or eigenvariable (`Inference::term`).

- **First-order proofs step by step** in `ordinary::Interactive` and
  `deduxit interact --logic`: `∀R` and `∃L` name a fresh eigenvariable,
  `∀L` and `∃R` take a witness term read by `Interactive::witness`
  (`apply 0 R0 existsR c`); `Step::with_witness`, `Needs::Witness`,
  `StepError::NoWitness` and `StepError::Witness` are new.

- **Open witnesses in the ordinary session**: `∀L` and `∃R` with no
  witness leave it open (`$1`, `$2`, …), an axiom fills it by
  unification, `undo` takes the filling back, and a session document
  keeps the bindings (`"bindings"`) and is checked under them when read.
  A witness still open in a finished proof is a fresh constant, so
  `∀x. p(x) ⊢ ∃x. p(x)` proves with no constant in the sequent.
  `Interactive::bindings`, `StepError::Binding` and `StepError::Terms`
  are new; `deduxit interact --logic` prints `filled $1 with TERM`. A
  step whose instance or filling would make the proof's terms pass
  `limits.terms` is refused, so a session writes no document its reader
  refuses.

- **First-order derivations drawn and read**: LaTeX, Typst and SVG draw
  them, the web bindings' ordinary step takes `"witness"`, and a TPTP
  problem without a conjecture reads as its hypotheses' refutation, a
  `lemma` or a `theorem` as a hypothesis. The flake's package
  `iltp-first-order` fetches ILTP's 2 670 first-order problems, which
  `deduxit-bench run --tptp` reads, includes inlined.

- **First-order sequents decided through the embedding**:
  `ordinary::decide` and `prove --logic` take the embedding, the default
  method at first order (`Options::method_for`). A valid sequent gets a
  checked derivation of LK, LJ or LM, its quantifier rules read back
  from the linear proof's instances; one whose image is unprovable is
  unknown with the new reason `Reason::Incomplete` (exit 3), since the
  translation may lose first-order proofs. `Formulas::is_first_order` is
  public.

- **A connection prover of classical logic** (`ordinary::connection`,
  `Method::Connection`, `prove --logic classical --method connection`):
  Otten's non-clausal connection calculus, nanoCoP's, on the formulas'
  own structure, with regularity, lemmata and a bound on the active path
  that it deepens. It decides classical sequents, first-order ones
  included, and is the default for a first-order classical sequent, the
  embedding deciding where it gives up at a bound; with no clock, the
  library's default gives it 2²⁶ of its steps. A valid sequent gets
  a derivation of LK built from the proof's substitution and checked;
  one where no quantifier needs a witness term (every `∀` one to prove,
  every `∃` one to assume, negations and implications taken into
  account) is refuted by an exhaustive search; the others are unknown at the
  bound, the new reason `Reason::PathLength` (`--path-length N|none`,
  `search::Options::path_length`, by default 8 in the library and none
  in the command), or `Reason::Incomplete`.
  `search::Options::restricted_backtracking`
  (`--restricted-backtracking`) keeps each literal's first solution,
  never answering "not valid". `deduxit-bench run --engines connection`
  runs it.

- **The connection prover decides intuitionistic logic** (`prove --logic
  intuitionistic --method connection`): nanoCoP-i's prefixed calculus,
  in which a classical connection proof is an intuitionistic one when its
  prefixes unify (T-string unification) and its eigenvariables meet the
  domain condition. "Valid" comes with a derivation of LJ, found by a
  search the proof guides and checked; otherwise the answer is unknown,
  even without quantifiers: the search can miss a proof and never shows
  that none exists. The default
  of intuitionistic logic does not change.

- **Grounding** (`ordinary::grounding`, `Method::Grounding`, `prove
  --logic … --method grounding`, behind the feature `sat`): the
  Bernays–Schönfinkel class (no function symbols, no `∀` under an `∃`
  after the quantifiers are moved inward) and the monadic class (no
  function symbols, predicates of at most one argument) decided by
  grounding over the constants and the SAT solver, in every logic. A
  valid classical sequent gets the solver's checked clausal proof, by
  default the connection prover's derivation of LK where it finds one
  within `EVIDENCE_WORK`; a sequent that is not valid gets a finite
  structure (`ordinary::Structure`, `FiniteCountermodel`), checked by
  evaluating the sequent in it (`Structure::refutes`,
  `FiniteCountermodel::check`) and printed after the verdict line. It is
  the default for the sequents of both classes; outside classical logic
  a classically valid one, and in every logic a sequent outside the
  classes, goes to the next method. `check` evaluates an outcome's
  structure again, and the new `Error::Structure` (code
  `invalid_structure`, kind invalid) says what a wrong one gets wrong.
  `deduxit-bench run --engines grounding` runs it.

- **The command shares its time limit at first order**: where the
  default of a first-order sequent is grounding or the connection
  prover, it gets 90% of the time limit and the embedding the rest.

- **A search for finite models** (`ordinary::models`, `Method::Models`,
  `prove --logic … --method models`, behind the feature `sat`): for 1,
  2, … elements in turn, it writes the sequent's falsity in a structure
  of that size as a propositional formula and asks the SAT solver for a
  model, after MACE and Paradox (both quantifiers expanded over the
  domain, each function symbol's values as atoms that must give each
  argument tuple one value, the constants' symmetries broken). A
  structure it finds is checked against the sequent and refutes it in
  every logic; it never answers "valid", and a sequent no structure
  within the bound refutes is unknown with the new reason
  `Reason::ModelSize` (`search::Options::model_size`, `--model-size
  N|none`, by default 6 in the library and none in the command). The
  default runs it too: the command beside the connection prover in
  classical logic, on a thread of its own, the first to answer winning
  (on one thread, after the prover's 70% of the time limit); the library
  last, on a first-order sequent the embedding leaves unknown, in every
  logic. On
  ILTP's first-order problems it refutes 210 classically at 2 s, among
  them all of grounding's 77 and nanoCoP's 84. `deduxit-bench run
  --engines models` runs it.

- **Kripke models with growing domains** (`ordinary::KripkeStructure`,
  `KripkeWorld`, `KripkeCountermodel`, `Outcome::kripke`): in
  intuitionistic and minimal logic the model search also tries Kripke
  models of 1, 2, … worlds with domains growing along the order, each a
  propositional sequent for the SAT solver (the order, the domains, the
  predicates and `⊥` growing, the function tables agreeing, forcing by
  its definition), and checks the model it finds by the forcing relation
  before "not valid"; past both bounds the answer is unknown with the new
  reason `Reason::KripkeSize` (`search::Options::model_worlds`,
  `--model-worlds N|none`, by default 4 in the library and none in the
  command). The library's default asks it after the embedding, the
  command beside it on a thread of its own, which takes one of the
  command's threads; on one thread (`--jobs 1`, `--deterministic`, an
  entry of a batch) the command asks it first, for 5% of the time limit,
  and the embedding takes the rest. In intuitionistic logic a finite model never
  refutes `¬¬A` for a classically valid `A`, so Glivenko's failure at
  first order stays unknown.

- **Finite structures interpret function symbols**
  (`Structure::functions`, `Structure::with_functions`): each function
  symbol of one argument or more with its table, the values of its
  argument tuples in lexicographic order, printed after the constants
  (`f: 0 ↦ 1, 1 ↦ 0`).

- **First-order countermodels drawn and certified**: `svg::structure`
  and `svg::kripke_structure` draw a finite structure or a Kripke model
  with growing domains as the propositional models are drawn, each
  world's box with its domain, values and facts; `latex::structure`,
  `latex::kripke_structure`, `typst::structure` and
  `typst::kripke_structure` write them as tables; `rocq::write_structure`
  and `rocq::write_kripke_structure` write the certificate for deduxit's
  Rocq library, `~ lk …` by `fo_classical_refutation` or `~ lj …` by
  `fo_kripke_refutation`, with the prelude
  `rocq::DEDUXIT_FIRST_ORDER_MODEL_PRELUDE`. `prove` and `check` write
  them for `--format svg|png|pdf|latex|typst|rocq`, and the protocol's
  `ordinary.draw`, `ordinary.export` (which takes an outcome now) and
  `ordinary.certificate` give them. The certificates need rocq-deduxit
  0.8.0, and state the calculi with cut. A model too large for
  Rocq, which counts every number one by one, has no certificate
  (`rocq::Unsupported::LargeModel`, a new variant): a domain of more
  than 4 096 elements (`rocq::MOST_DOMAIN`), a table or a list of more
  than 8 192 entries (`rocq::MOST_ENTRIES`), or numbers that add up to
  more than 1 048 576.

- **`prove --szs`** ends the text output of `--logic` with an SZS status
  line, `% SZS status STATUS for NAME`, for the tools of the TPTP World.

- **Otten's connection provers in the comparison**: the flake fetches
  leanCoP 2.1, nanoCoP 2.0, nanoCoP-i 2.0 and iLeanCoP 1.2 at their
  released archives and runs them on SWI-Prolog (`prover-leancop`,
  `prover-nanocop`, `prover-nanocopi`, `prover-ileancop`);
  `deduxit-bench run --tools leancop,nanocop,nanocopi,ileancop` runs
  them on TPTP problems.

- **Normal forms at first order**: `seq normalize --to prenex`,
  `miniscope`, `skolem`, `herbrand` and `clausal`, and `--to nnf` at first
  order, every step shown; the protocol's `ordinary.prenex`,
  `ordinary.miniscope`, `ordinary.skolem`, `ordinary.herbrand` and
  `ordinary.clausal`. Each equivalence step's law is checked by its own
  derivation, of LJ where the law holds in intuitionistic logic
  (`normal::Rule::intuitionistic`, `normal::Claim::Law`); each Skolem or
  Herbrand step's symbol is checked new and applied to exactly the
  variables free in its quantifier (`normal::Step::term`), and the
  result is checked to imply the input's negation (Skolem, clausal) or
  to follow from the input (Herbrand, `normal::Promise::Equivalid`).

- **Herbrand's theorem**: `deduxit herbrand SEQUENT` makes the clausal
  form of a first-order sequent, puts closed terms for its variables up
  to `--depth`, and has the SAT solver refute the ground instances; the
  instances it needs are the certificate, and a derivation of LK of the
  sequent as written, each Skolem term an eigenvariable again, is built
  from them and checked. The library's `ordinary::herbrand`, the
  protocol's `ordinary.instances` and the wire form at level 28.

- **Resolution at first order**: `deduxit resolve` takes first-order
  sequents: binary resolution with the most general unifier and
  factoring on the clausal form, each step with its unifier, the clauses
  of the negated goals the set of support. Every refutation is checked
  step by step and read back as a checked derivation of LK where it
  reaches. The library's `ordinary::resolution::first_order`, the
  protocol's `ordinary.resolve` at first order, the wire form at level
  29.

- **Paramodulation in first-order resolution.** A sequent with `=` is
  resolved with paramodulation by default (`resolution::Equality`,
  `Options::equality`, `resolve --equality paramodulation|axioms`): an
  equation rewrites a subterm of another clause's literal as a step of
  its own (`first_order::Step::Paramodulation`), which the checker
  computes again, with reflexivity as a clause
  (`equality::with_reflexivity`). `axioms` resolves with every equality
  axiom instead.

- **Interpolants at first order**: `interpolate` takes first-order
  sequents in all three logics. A term of one half alone is bound in the
  interpolant by `∃` (the first half's) or `∀` (the second's), and the
  check holds the interpolant's predicates, functions and constants to
  both halves.

- **The negative translations at first order**: `seq translate --by
  kolmogorov`, `godel-gentzen` and `kuroda` take first-order sequents
  (Kuroda's puts `¬¬` after every `∀`, Gödel and Gentzen's writes
  `∃x. A` as `¬∀x. ¬A`), and `compare` checks the verdict kept as in
  propositional logic. Glivenko's translation refuses them with the
  new error `Error::GlivenkoFirstOrder` (code `glivenko_first_order`),
  whose message says why: its theorem fails at first order, on
  `¬¬∀x. (p(x) ∨ ¬p(x))` and the double-negation shift, which no finite
  Kripke model refutes; rocq-deduxit's `chain_excluded_middle` and
  `chain_double_negation_shift` prove that an infinite one does.

- **`check --tree never` writes no tree of an ordinary derivation**, as
  `prove` already did; a deep derivation's check no longer spends its
  time writing a tree nobody sees.

- **`Error::FirstOrder`** (code `first_order`, kind unsupported): a
  method that decides or transforms propositional sequents (the SAT
  solver, LJT, the truth table, the tableau, the normal
  forms, resolution, the interpolants, the negative translations, the
  Kripke model's check, the goal state's `close` and `hint`, the
  certificate over `Prop`, the lambda-term) refuses a first-order sequent
  by name.

### Changed

- **`--derivation-limit` bounds what is drawn, not what is built, under
  `--logic`**: a derivation of LK or LJ is built and checked within the
  memory bound whatever its size, so `-q` and `--format json` give the
  checked "valid" where the drawing would be too large (ILTP's SYJ202,
  Cook's pigeonhole formulas, at sizes 4 to 6), and the drawing past the
  limit is left out with its verdict kept. The tableau method still
  builds its tableau within the limit. The default of intuitionistic
  logic keeps LJT's derivation where its drawing fits the limit, and the
  SAT solver's otherwise. In the library, every reader back of an
  ordinary derivation (the embedding's, LJT's, the SAT solver's, the
  connection provers') is bounded by `limits.memory_bytes`, the
  protocol's `ordinary.draw`, `ordinary.export` and
  `ordinary.certificate` refuse a drawing past
  `limits.derivation_bytes`, and `ordinary::Derivation::drawn_bytes`
  estimates a drawing by the measure that bound takes, for a caller that
  draws a decided derivation itself.
- **A bound or the stop that ends the read-back or a check after the
  search decided is an unknown, not an error**: `ordinary::decide` and
  each method's `decide` (LJT, the SAT solver, the connection provers,
  the model checks of the truth table and the tableau) answer
  `Ok(Unknown(reason))` with the search's statistics, as a bound that
  ends the search does, so `prove --format json` writes that outcome
  (before, nothing on standard output) and a named method's bound there
  is exit status 3, not 2. A defect stays an error; so do the
  embedding's read-back in `ordinary::decide` and a truth table or
  tableau that a bound ends while it is built.
- The default of intuitionistic and minimal logic gives LJT at most
  16 384 sequents to find the derivation it prefers to the SAT solver's,
  so a sequent the solver decides at once is answered at once: ILTP's
  SYJ202+1.005 in 0.05 s, where LJT's search took the whole time limit.
- Where the default of intuitionistic or minimal logic stops at a bound
  and the embedding decides, `prove`'s verdict line says which bound
  stopped it.
- The modal search in K4, KD4 and S4 makes its worlds in depth and keeps
  what it learns about a world across branches: a world made again under
  the same conditions reuses the earlier world's part of the model, or
  closes its branch at once where the earlier one could not be satisfied.
  A sequent of KD4 that took 20.5 million units of work now takes about
  58 000. Every "valid" still comes with a checked derivation, every
  countermodel is checked as before.
- The web package's PNG and PDF renderers are a module of their own,
  `deduxit_web_render_bg.wasm` (the crate `deduxit-web-render`), with
  the one call `render`, which left the bindings' module. The page
  loads it on a visitor's first PNG or PDF export, so the module every
  visitor loads is 2.8 MB at gzip, with first-order logic in it, where
  0.7.0's was 3.8 MB.
- The help of `prove` names the methods `connection`, `grounding` and
  `models` and the flag `--szs`; that of `prove` and `interact` the
  connection prover's and the model search's bounds; that of `seq print`
  and the syntax summary equality; the list of commands names
  `herbrand`; `workspace operations` lists the first-order operations.

### Fixed

- `ordinary::Derivation::term` answers `Error::ReadBack` for a derivation
  its read-back cannot read, where it panicked; the crate makes no such
  derivation, and the checker refuses one read from JSON.
- An unknown verdict at the memory bound names the bound given: the Horn
  engine's said "the memory limit of 0 B" under `--memory-limit 1MiB`.
- `prove --logic` reads a TPTP problem whose comments hold bytes that
  are not UTF-8: 107 of ILTP's first-order problems write an author's
  name in Latin-1, and the command refused them as not UTF-8 text. Such
  a byte in a formula is refused with its line.
- `Sequent::add` and `add_cut` on a JSON sequent with quantifiers over
  nullary atoms only, joined with a first-order sequent, gave its atoms
  the other sequent's names: `⊢ ∀x. (a ⅋ ~b)` with `⊢ p(c), q` added
  printed `⊢ ∀x. (p(c) ⅋ ~q), p(c), q`.
- The Rocq certificate of an LJ derivation read from JSON that uses
  LK's one-premise `¬L` (`Γ, ¬A ⊢` from `Γ ⊢ A`, which the checker
  takes) was a term Rocq refused. No search makes that form.

### Wire forms

- **Level 25**: a first-order ordinary sequent or derivation, its
  formulas with `{"∀": a}` and `{"∃": a}`, and its tables under the
  linear sequent's keys `symbols`, `fo`, the atoms as `{"P": [symbol,
  [terms]]}` and `binders`. A propositional one stays at level 8. A
  derivation's quantifier rule names its term by `"term": k`, an
  eigenvariable among the terms being `{"E": n}`. An ordinary outcome
  with a first-order derivation, or unknown for the new reason
  `{"kind": "incomplete"}`, is written at level 25 too, and so is one
  decided by the connection prover (`"search": {"method": "connection",
  "statistics": {…}}`) or unknown at its bound (`{"kind": "path_length",
  "length": 8}`). The search options have the keys `path_length` and
  `restricted_backtracking`. One decided by grounding is written at level
  25 (`"search": {"method": "grounding", "statistics": {"domain",
  "atoms", "nodes", "sat"}}`), and a refutation by it holds
  `"structure": {"sequent": …, "structure": {"size", "constants",
  "predicates"}}`.
- **Level 26**: an ordinary outcome of the model search (`"search":
  {"method": "models", "statistics": {"size", "atoms", "nodes",
  "sat"}}`), one unknown at its bound (`{"kind": "model_size", "size":
  6}`), and one whose structure has function tables (`"functions":
  [["f", [1, 0]]]`, left out where there are none). The search options
  have the key `model_size`.
- **Level 27**: an ordinary outcome refuted by a Kripke model with
  growing domains (`"kripke_structure": {"sequent": …, "model":
  {"worlds": […], "constants": […], "world": 0}}`), or unknown at its
  bounds (`{"kind": "kripke_size", "worlds": 4, "size": 6}`); the model
  search's statistics have `worlds` where it tried a Kripke model. The
  search options have the key `model_worlds`.
- **Level 28**: Herbrand's theorem applied (`herbrand --format json`):
  the clausal form with its steps, the ground instances, the SAT
  solver's outcome on them and the derivation built from them. Written
  only.
- **Level 29**: first-order resolution (`resolve --format json` on a
  first-order sequent): the clausal form with its steps, the clauses,
  and each inference (resolution, factor or paramodulation) with its
  parents, literals, unifier and clause, and the derivation of LK read
  back. Written only.
- `wire::LEVEL` is 29. Every document of levels 1 to 24 reads and is
  written as before; 0.7.0 refuses levels 25 to 29 by name.


Install it as the manual's [Installation](https://deduxit.github.io/deduxit/manual/installation.html) says; `SHA256SUMS` holds the files' sums.
