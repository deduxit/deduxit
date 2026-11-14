# Check a certificate in Rocq

`--format rocq` writes a proof as a certificate: a file for the proof
assistant [Rocq](https://rocq-prover.org) whose lemma states the
sequent and whose proof Rocq checks. A proof Rocq accepts does not
depend on deduxit's own checker.

## Check one with Nix

Write the certificate with `--standalone`, which adds the import lines,
and give it to the flake app `check-certificate`. It needs only Nix with
flakes enabled (see [Install deduxit](installation.md)):

<!-- readme-check: skip, Nix runs the app; the flake check check-certificate runs it on a certificate of each kernel -->
```console
$ deduxit prove --format rocq --standalone --output proof.v "A * B |- B * A"
$ nix run github:deduxit/deduxit#check-certificate -- proof.v
Checking proof.v with Rocq 9.1.1 and NanoYalla 1.1.3.
Lemma certificate: proved, with no axiom.
The certificate holds.
```

The app reads the kernel from the file's import lines and compiles the
file against the versions the flake pins: Rocq and its standard library
from nixpkgs, NanoYalla 1.1.3 and rocq-deduxit 0.3.0. A release's tag
after the repository's name (`github:deduxit/deduxit/TAG`) takes that
release's versions. Then the app asks Rocq which axioms each lemma
rests on, since a proof left `Admitted` compiles too. Its exit status
is 0 when the certificate holds, 1 when Rocq refuses the file or a
lemma rests on an axiom (a classical certificate over `Prop` may use
the excluded middle), and 2 when the file cannot be read.

The app reads the lemma's statement from the file, so it cannot see a
file that redefines the names its statement uses (its own `Definition
ll`, say): such a file proves something else. Check a certificate that
deduxit wrote, or read its statement.

Without Nix, install the kernel the certificate needs and compile it
with Rocq ([Check one without Nix](#check-one-without-nix)).

## The kernels

A certificate is written for one of two kernels, which `--kernel`
names:
- `nanoyalla`, the default for classical linear logic;
- `deduxit`, deduxit's own Rocq library
  ([deduxit/rocq-deduxit](https://github.com/deduxit/rocq-deduxit)),
  the default for Mix, affine and intuitionistic logic, for a sequent
  with a quantifier, and the only kernel for a refutation.

The verdict line becomes a comment of the certificate.

### NanoYalla

For `nanoyalla` the certificate is a proof script that
[NanoYalla](https://github.com/ComputerAidedLL/click-and-collect/tree/master/nanoyalla)
checks, the Rocq kernel of Click & coLLecT. It is a lemma stating the
one-sided sequent over the atoms as `formula` variables. Its proof uses
one derived rule of the kernel per inference, with an exchange before a
`⊗` where the kernel's list sequents need one, and ends with `Qed`, so
Rocq accepts the file only if the kernel accepts the proof. The kernel
has no rule for Mix or for the weakening of affine mode, so a proof
with either has no certificate here: a line on standard error says so,
and the verdict and its exit status stand. An intuitionistic proof is
certified as the classical proof it is.

```console
$ deduxit prove --format rocq "A * B |- B * A"
(* provable (MLL, classical, focus engine) *)
Lemma certificate (A B : formula) : ll [parr (dual A) (dual B); tens B A].
Proof.
apply (parr_r_ext []); cbn_sequent.
apply (ex_perm_r [2; 0; 1] [dual B; tens B A; dual A]).
apply (tens_r_ext [dual B]); cbn_sequent.
{
  ax_expansion.
}
{
  ax_expansion.
}
Qed.
```

### rocq-deduxit

For `deduxit` the certificate is a lemma stating the sequent over the
library's own calculus, with the atoms as strings. It is one-sided in
the calculus of the mode (`ll classical`, or with Mix or weakening),
and two-sided for an intuitionistic proof (`ill intuitionistic` or `ill
affine`, hypotheses and one goal). The lemma is proved by the library's
soundness theorem from the derivation written as a proof term, which
the library's checker accepts by computation. So Rocq accepts the file
only if the checker accepts the term. Every rule has its term, Mix and
weakening included:

```console
$ deduxit prove -i --format rocq "A, A -o B |- B"
(* provable (IMLL, intuitionistic, two-sided engine) *)
Lemma certificate : ill (atom := string) intuitionistic [ivar "A"; ilmap (ivar "A") (ivar "B")] (ivar "B").
Proof.
  apply (soundness_ill String.eqb String.eqb_spec intuitionistic
    (ILmapL (ivar "A") (ivar "B") (IAx "A") (IAx "B"))).
  vm_compute. reflexivity.
Qed.
$ deduxit prove --format rocq --kernel deduxit "A * B |- B * A"
(* provable (MLL, classical, focus engine) *)
Lemma certificate : ll (atom := string) classical [parr (covar "A") (covar "B"); tens (var "B") (var "A")].
Proof.
  apply (soundness_classical String.eqb String.eqb_spec
    (Par (covar "A") (covar "B") (Tensor (var "B") (var "A") (Ax "B") (Ax "A")))).
  vm_compute. reflexivity.
Qed.
```

A first-order proof gets a certificate of the library's first-order
development, in every mode:
- the predicates are applied to terms (`tapp "f" [...]`);
- a bound variable is a de Bruijn index (`bound 0`, the innermost
  quantifier's), and an eigenvariable is named by its number
  (`eigen 0`);
- every instance has its quantifiers' terms substituted;
- the calculus's optional rules are named by their module
  (`Calculus.classical`, `Intuitionistic.affine`).

A `∀` step names the body it opens and its eigenvariable, which occurs
nowhere in its conclusion. An `∃` step names its witness, here a
constant that no `∀` step introduces, as a domain is never empty:

```console
$ deduxit prove --format rocq "forall x. p(x) |- exists y. p(y)"
(* provable (MLL1, classical, focus engine) *)
Lemma certificate : ll (symbol := string) Calculus.classical [fex (covar "p" [bound 0]); fex (var "p" [bound 0])].
Proof.
  apply (soundness_classical String.eqb String.eqb_spec
    (Exists (covar "p" [bound 0]) (eigen 0) (Exists (var "p" [bound 0]) (eigen 0) (Ax "p" [eigen 0])))).
  vm_compute. reflexivity.
Qed.
```

## Certificates of unprovability

An unprovable sequent gets a certificate too, where its refutation is
one that `check` verifies, a saturated set excepted. It is a lemma that
the mode's calculus does not derive the sequent, proved by the
library's theorem of the refutation's invariant, which the library's
test accepts by computation. These certificates are always for
rocq-deduxit.

What the certificate names depends on the refutation:
- an unbalanced atom names the atom;
- a classical assignment names its true atoms;
- the count equation names nothing;
- a count with copies of one sign, and a Horn program's state equation,
  are weighted counts. Each atom weighs an integer: `1` or `−1` on the
  counted atom, or the state equation's weights. The atoms no firing of
  the program reaches are declared dead. The library's test asks every
  contribution of the sequent to the weighted count of its literals to
  be above zero, the copies of a `?` formula included (with weakening,
  of the formulas with one output).

A sequent with a quantifier is stated over the first-order calculus,
its counts weighing a predicate whole or one closed atom; its count
equation has no certificate. An exhausted search has none either, which
a line on standard error says:

```console
$ deduxit prove --format rocq "A |- B"
(* unprovable (MLL, classical, focus engine): A on the left has no A on the right to meet in an axiom *)
Lemma certificate : ~ ll (atom := string) classical [covar "A"; var "B"].
Proof.
  apply (unbalanced_refutation String.eqb "A" _).
  vm_compute. reflexivity.
Qed.
$ deduxit prove -i --format rocq "A + B |- A"
(* unprovable (IALL, intuitionistic, additive engine): the classical reading is false where B is true and every other atom false (! erased, ⊗ and & as and, ⊕ as or, ⊸ as implies, 1 and ⊤ true, 0 false), and every provable sequent reads as a tautology *)
Lemma certificate : ~ ill (atom := string) intuitionistic [iplus (ivar "A") (ivar "B")] (ivar "A").
Proof.
  apply (iclassical_refutation _ (fun a => existsb (String.eqb a) ["B"])).
  vm_compute. reflexivity.
Qed.
$ deduxit prove -i --format rocq "a, a -o b, !(b -o b * c), !(b -o d) |- b * d"
(* unprovable (IMELL, intuitionistic, horn engine): the state equation of the Petri net has no solution, so no firing of its clauses yields the goal's atoms: weighting each a by 2, b by 1, d by 1 and the clauses used once by weights of their own, no clause that can fire raises the weighted count of the atoms, and the goal asks it raised *)
Lemma certificate : ~ ill (atom := string) intuitionistic [ivar "a"; ilmap (ivar "a") (ivar "b"); ioc (ilmap (ivar "b") (itens (ivar "b") (ivar "c"))); ioc (ilmap (ivar "b") (ivar "d"))] (itens (ivar "b") (ivar "d")).
Proof.
  apply (iweighted_refutation (weight_of String.eqb [("a", 2%Z); ("b", 1%Z); ("d", 1%Z)])
    (fun a => existsb (String.eqb a) []) true _).
  vm_compute. reflexivity.
Qed.
$ deduxit prove --format rocq "A & B |- A * B"
no certificate: an exhausted search states no invariant, so only the search rules the proofs out and Rocq has nothing to check
(* unprovable (MALL, classical, focus engine): the search was exhaustive *)
```

## Check one without Nix

### NanoYalla

Install NanoYalla 1.1.3 (the `nanoyalla` directory of the Click &
coLLecT repository) with Rocq 9 and its standard library: run
`./configure && make && make install` there, or `rocq compile -R .
NanoYalla nanoll.v` and the same for `macroll.v`. Then compile the
certificate with the kernel on the load path:

```console
$ deduxit prove --format rocq --standalone --output proof.v "!A, !B |- !(A * A)"
$ rocq compile -R path/to/nanoyalla NanoYalla proof.v
```

Rocq prints nothing for a certificate it accepts. The kernel needs no
Yalla installation, and the certificate uses no cut and no axiom.

### rocq-deduxit

The library is the package `rocq-deduxit`, in a repository of its own,
[deduxit/rocq-deduxit](https://github.com/deduxit/rocq-deduxit). It
runs on Rocq 9.0 to 9.3 with its standard library alone, and its linear
logic is under the logical path `Deduxit.Linear`. Its version is
deduxit's, so deduxit 0.3.0's certificates compile against
rocq-deduxit 0.3.0.

Build it with Nix (`nix build github:deduxit/rocq-deduxit/v0.3.0`), or
with `make` in a checkout with Rocq on the path. Built in place, a
certificate compiles with the theories on the load path:

```console
$ deduxit prove --mix --format rocq --standalone --output mix.v "A, B |- A, B"
$ rocq compile -Q path/to/rocq-deduxit/theories Deduxit mix.v
```

To install it with opam, pin it to its tag. The package is not yet in
the Rocq opam archive, so `opam install rocq-deduxit` fails today; once
a release is there, the archive's two lines install it:

```sh
opam pin add rocq-deduxit https://github.com/deduxit/rocq-deduxit.git#v0.3.0
```

```sh
opam repo add rocq-released https://rocq-prover.org/opam/released
opam install rocq-deduxit
```

Installed, a certificate compiles as it is. `opam upgrade rocq-deduxit`
takes a newer release from the archive and `opam remove rocq-deduxit`
removes it; a package pinned to a tag stays there until pinned to the
next.

### Use a certificate in another file

A certificate is a module, named by its file, so another file (`use.v`
here) requires it and applies its lemma. An intuitionistic certificate
states the two-sided sequent itself, so it proves that goal as written:

```coq
From Stdlib Require Import List String.
From Deduxit.Linear Require Import IntuitionisticCheck.
Import ListNotations.
Open Scope string_scope.
From Proofs Require mp.

Goal ill (atom := string) intuitionistic [ivar "A"; ilmap (ivar "A") (ivar "B")] (ivar "B").
Proof. apply mp.certificate. Qed.
```

Write the certificate beside it as `mp.v`, and compile both under one
logical name:

```sh
deduxit prove -i --format rocq --standalone --output mp.v "A, A -o B |- B"
rocq compile -R . Proofs mp.v
rocq compile -R . Proofs use.v
```

The library's README says what it covers and what not, and its
documentation is at
[deduxit.github.io/rocq-deduxit](https://deduxit.github.io/rocq-deduxit/).
