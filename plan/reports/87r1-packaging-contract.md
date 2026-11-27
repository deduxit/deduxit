# Step 87r1: the Rocq library's packaging, CI and contract

Opus 5.5 at `high`, in a rocq-deduxit workspace of its own
(`../rocq-deduxit-wt-87r`, on `main@origin` `uytplsqq`), theme H1 of the
audit (F-9-01, F-9-02, F-9-03, F-9-09, F-9-11, F-9-12), with each
finding's verdict. The exporter's side of the contract (F-9-01's other
half, F-4d-08) is 87c3's; what it must match is below.

## Outcome

Three commits in rocq-deduxit, on `main@origin` `uytplsqq`:
`ywmqstsn` (the checkers' `let*` at level 10), `xonkwzoq` (the
interface, README's examples and Rocq's warnings in the check),
`xrzlqylk` (opam in CI, the standard library's bound, the release
archive, the records). No statement, no lemma's name and no path of the
library changed.

- **The contract (F-9-01).** `tests/Interface/` holds one file per
  prelude of deduxit's exporter (`core/src/export/rocq/mod.rs`'s ten
  `DEDUXIT_*_PRELUDE`s: `OneSided.v`, `TwoSided.v`, `FirstOrder.v`,
  `FirstOrderTwoSided.v`, `Refutation.v`, `FirstOrderRefutation.v`,
  `FirstOrderModel.v`, `Kripke.v`, `Modal.v`, `ModalCountermodel.v`) and
  `Named.v` (the two theorems deduxit's messages cite). Each starts with
  its prelude verbatim and uses every name a certificate writes, by
  `Definition`s that print nothing (the verdict: the check fails on any
  output, so no `Check`): each constructor applied to arguments of its
  types, each record as a literal of all its fields (a missing field is
  refused), each theorem applied as the certificate applies it, with
  its conclusion stated (`_ = true -> ll (atom := string) … l`), which
  pins the name, the argument order, the implicit's name and the
  conclusion, and leaves the checker's own names (`check`, `falsifies`)
  free. Two planted faults were refused (a record literal without
  `cut`, `soundness_classical` at the Mix calculus). README has a
  section "The certificates' interface" with the same names by prelude,
  and the changelog says a change that breaks it is a breaking change.
- **README's examples (F-9-09).** The check writes each ```` ```coq ````
  block of README to a file of its own (the verdict: two blocks define
  `excluded_middle`) and compiles it in silence; all five compile. The
  `console` block runs deduxit, which this flake lacks, so nothing runs
  it; its flags change with D1 in part 2.
- **Warnings and the goal selector (F-9-11).** The check's build of the
  library (every admitted Rocq) passes `COQEXTRAFLAGS`: `-w +default`
  (every warning Rocq gives by default an error) and `-set "Default
  Goal Selector=!"`. The package keeps Rocq's defaults, as the verdict
  asked, since an opam user's `make` with a later Rocq may warn where
  the admitted ones do not. The selector cannot go into `_RocqProject`:
  `rocq makefile` splits an `-arg` value at its spaces (tried). Every
  proof already passed under `!`. Three default warnings fired on 9.2
  and 9.3:
  - `at-level-200-changed` (9.3) on the checkers' local `let*`
    notation, three copies: fixed as the warning prescribes (`at level
    10`, no associativity, `k at level 200`), valid on every version, in
    checker files outside the trusted base; no term changes.
  - `notation-for-abbreviation` (9.2): F-9-04, 87r3's. Kept a plain
    warning by name until then.
  - `register-all` (9.2, new): `Terms.v`'s `term` is nested in `list`,
    and 9.2 asks for `Scheme All`, which 9.0 and 9.1 lack. Kept a plain
    warning by name; it can go when 9.0 and 9.1 are dropped. (Not in the
    audit: the inventory ran without warnings as errors.)
  Rocq ignores a warning name it does not know, so the names are inert
  on 9.0 and 9.1.
- **The standard library's bound (F-9-12).** `"rocq-stdlib" {>= "9.0" &
  < "9.3~"}`. opam has stdlib 9.0.0, 9.1.0 and 9.2.0 (read from
  opam.ocaml.org on 2026-11-27; 9.2.0 asks `rocq-core >= 9.1`), so the
  bound admits every stdlib an admitted Rocq takes. opam pairs Rocq
  9.0.1 with stdlib 9.1.0 and 9.1.1 with 9.2.0, pairs nixpkgs never
  builds: `scripts/opam-install.sh 9.0.1 9.1.1` installed and imported
  the library from its opam file on both (here, with the network).
- **opam in CI (F-9-03).** `.github/workflows/opam.yml`, a reusable
  workflow: `scripts/opam-install.sh` in the flake's opam shell, a job
  per Rocq 9.0.1, 9.1.1, 9.2.0, 9.3.0. `ci.yml` calls it beside `nix
  flake check`; `release.yml` calls it and the release and Pages jobs
  wait for it. The verdict's two sub-points that fell stay out: no
  `--check-upstream` (the repository's opam file has no `url`), and the
  bridges stay on one Rocq (they are not part of the package).
- **The release archive (F-9-02).** `release.yml` makes the archive with
  `git archive --prefix=rocq-deduxit-VERSION/`, uploads it with the
  release, and gives its download URL and SHA-512 in the notes, for the
  opam file's `url` section.

## What the exporter must match (for 87c3)

The contract is `tests/Interface/`, file by prelude; the exporter's
"one place" for the library's constructors (87c3, F-4d-08) is a table
per library module, three formula inductives and their proof terms
apart, since a rename in one leaves the others:

| library module | names the exporter writes |
|---|---|
| `Linear.Formulas` | `var covar one bot top zero tens parr awith aplus oc wn` |
| `Linear.FirstOrderFormulas` | the same, `var`/`covar` with a term list, and `fall fex` |
| `Linear.Terms` | `bound eigen tapp` |
| `Linear.Intuitionistic` | `ivar ione itens ilmap itop iwith izero iplus ioc`; `intuitionistic affine`; fields `iweakening icut` |
| `Linear.FirstOrderIntuitionistic` | the same with `iall iex` |
| `Linear.Proofs`, `FirstOrderProofs` | `Ax Tensor Par One Bot With Plus Top Bang Quest Copy Weaken Mix Cut` (`Left Right`), and `Forall Exists` |
| `Linear.IntuitionisticCheck`, `FirstOrderIntuitionisticCheck` | `IAx IOneR IOneL ITensR ITensL ILmapR ILmapL ITopR IWithR IWithL1 IWithL2 IZeroL IPlusR1 IPlusR2 IPlusL IOcR IDe IWk ICo IWeaken ICut`, and `IAllR IExL IExR IAllL` |
| `Linear.Calculus` | `classical`; fields `mix weakening cut` (qualified `Calculus.` under the first-order preludes) |
| `Intuitionistic.Formulas`, `FirstOrderFormulas` | constructors `var bot wedge vee imp` (and `fall fex`), definitions `top neg equiv` |
| `Intuitionistic.Calculus` | `intuitionistic minimal`; fields `explosion cut` (qualified `Calculus.` under the first-order model prelude) |
| `Intuitionistic.KripkeCheck` | fields `later atoms absurd` |
| `Intuitionistic.FirstOrderCheck` | fields `size functions predicates`; `later sizes tables facts absurd` |
| `Modal.Formulas` | `var top bot neg wedge vee imp equiv box dia` (all constructors) |
| `Modal.Labelled`, `Modal.Kripke` | `lab rel`; fields `serial reflexive symmetric transitive euclidean` |
| `Modal.Check` | `Ax LBot RTop LNeg RNeg LAnd RAnd LOr ROr LImp RImp LIff RIff LBox RBox LDia RDia Ref Ser Sym Trans Eucl` |
| `Modal.Countermodel` | fields `arrows atoms` |

The theorems, by prelude, with the arguments a certificate gives:
`soundness_classical`, `soundness_mix`, `soundness_affine` (`String.eqb
String.eqb_spec TERM`), `soundness` (and the record); `soundness_ill`
(`String.eqb String.eqb_spec RULES TERM`); the first-order
`soundness`, `soundness_classical`, `soundness_mix`, `soundness_affine`
and `soundness_ill` alike (the first-order `soundness_mix` is in no
snapshot: `deduxit prove --mix` of a quantified sequent writes it); `classical_refutation`, `unbalanced_refutation`,
`equation_refutation`, `weighted_refutation`, `affine_refutation` and
each `i…` form, with `weight_of String.eqb`;
`FirstOrderWeights.weighted_refutation` and `iweighted_refutation`,
qualified, with `TermsCheck.terms_eqb String.eqb`; `kripke_refutation`
(`String.eqb String.eqb_spec RULES MODEL WORLD`); `fo_classical_refutation`
(`String.eqb true STRUCTURE`), `fo_kripke_refutation` (`String.eqb
String.eqb_spec _ MODEL WORLD`); `modal_soundness` (`String.eqb
String.eqb_spec CONDITIONS LABEL TERM`), `labelled_soundness` (no
label); `modal_refutation` (`String.eqb MODEL CONDITIONS WORLD`). The
statements name the calculus with `(atom := string)` or `(symbol :=
string)`: the implicit's name is part of the contract. The messages of
`ordinary::negative` name `chain_excluded_middle` and
`chain_double_negation_shift` of `Deduxit.Intuitionistic.FirstOrderKripke`.

How the list was made: every certificate of deduxit's snapshots
(`core/tests/snapshots/deduxit/`, 71) and of its `rocq-deduxit` check's
commands (37, written with the workspace's debug binary) compiled
against the library with `.glob` files; the `Deduxit.` references by
prelude, completed with the constructors and fields no certificate
happened to use and checked against the exporter's string literals.

## The net

- The library's check on all four Rocq versions under the strict build:
  failed first on 9.2 and 9.3 (the three warnings above), passes since.
- `nix flake check` in rocq-deduxit at the last commit (`xrzlqylk`,
  commit `165177f4`, pinned by `?rev=`): passes, every check (the four
  versions, the bridges, treefmt, conventions, typos, deadnix,
  actionlint).
- deduxit's `rocq-deduxit` and `rocq-deduxit-library` against that
  commit (`--override-input rocq-deduxit
  git+file:///home/tux/Projects/own/rocq-deduxit-wt-87r?rev=165177f4…`):
  pass.
- `scripts/opam-install.sh 9.0.1 9.1.1`: installed, imported and
  removed on both, `opam lint` clean. 9.2.0 and 9.3.0 were not run here
  (CI's job runs all four).
- No statement and no lemma's name changed; `tests/Assumptions.v`
  unchanged and closed.
- The new CI job and the release's archive cannot run before a push and
  a tag: `actionlint` (the flake's check) is all that checked them.

## The reviewer

`fresh-reviewer` on Fable, once, on the three commits; three findings,
all mended in the commits they belong to:
1. The first-order `soundness_mix` was neither pinned nor listed: the
   exporter writes it for a first-order proof in Mix mode
   (`deduxit.rs:161-169`), but no snapshot holds that case, so the
   `.glob` method missed it. Confirmed by a run (`deduxit prove --mix
   'forall x. p(x) |- exists y. p(y)'` applies it); now in
   `FirstOrder.v`, README's table and above. The rules file says a name
   the exporter writes is read from its code too.
2. README's table left out `bound`, `eigen`, `tapp` on two first-order
   rows; added.
3. "Every default warning an error" said without the two exceptions in
   README, the changelog and the module's header; said now.
It found criteria 2 to 5 sound: the preludes verbatim, the `let*`
change unable to reparse a use (all `Local`, `k` still at level 200),
the strict flags on the check's build only, the workflows and the bound.

## After the push: fixup-32

The new opam job failed on main in all four versions: it passed the
version in an environment variable named `ROCQ`, which the makefile
`rocq makefile` writes reads as the command that runs Rocq, so opam's
build of the standard library ran `9.1.1` as a program ("command not
found"). The local run had passed because it gave the versions as
arguments. The variable is `ROCQ_VERSION` now (`kzoyyzun`), the rules
file says which names to avoid, and the job's command with the job's
environment installed and imported the library on Rocq 9.1.1 here.

## Left open

- The `console` example of README (`deduxit prove --mix …`) runs in no
  check of either repository; D1 changes its flags in part 2.
- `register-all` and `notation-for-abbreviation` stay plain warnings by
  name (above); 87r3 removes the second with F-9-04.
