# Rocq certificates

`--format rocq` writes the derivation as a Rocq certificate for one of
two kernels, which `--kernel` names: `nanoyalla`, the default for
classical linear logic, or `deduxit`, deduxit's own Rocq library
([deduxit/rocq-deduxit](https://github.com/deduxit/rocq-deduxit)), the
default for Mix, affine and intuitionistic logic and for a sequent with
a quantifier. The verdict becomes a
comment, and `--standalone` adds the import lines.

For `nanoyalla` it is a proof script that the Rocq kernel
[NanoYalla](https://github.com/ComputerAidedLL/click-and-collect/tree/master/nanoyalla)
checks, the kernel of Click & coLLecT: a lemma stating the one-sided
sequent over the atoms as `formula` variables, proved by one derived rule
of the kernel per inference, with an exchange before a `⊗` where the
kernel's list sequents need one, and closed by `Qed`, so Rocq accepts the
file only if the kernel accepts the proof. A proof with Mix or with the
weakening of affine mode has no certificate there, since the kernel has
no such rule (a line on standard error says so, and the verdict and its
exit status stand); an intuitionistic proof is certified as the
classical proof it is.

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

To check a certificate, install NanoYalla 1.1.3 (the `nanoyalla`
directory of the Click & coLLecT repository) with Rocq 9 and its standard
library: `./configure && make && make install` there, or
`rocq compile -R . NanoYalla nanoll.v` and the same for `macroll.v`, then
compile the certificate with the kernel on the load path:

```console
$ deduxit prove --format rocq --standalone --output proof.v "!A, !B |- !(A * A)"
$ rocq compile -R path/to/nanoyalla NanoYalla proof.v
```

Rocq prints nothing for a certificate it accepts. The kernel needs no
Yalla installation, and the certificate uses no cut and no axiom.

For `deduxit` it is a lemma stating the sequent over the library's own
calculus, with the atoms as strings: one-sided in the calculus of the
mode (`ll classical`, or with Mix or weakening), and two-sided for an
intuitionistic proof (`ill intuitionistic` or `ill affine`, hypotheses
and one goal). It is proved by the library's soundness theorem from the
derivation written as a proof term, which the library's checker accepts
by computation, so Rocq accepts the file only if the checker accepts the
term; every rule has its term, Mix and weakening included:

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
development, in every mode: the predicates applied to terms
(`tapp "f" [...]`), a bound variable as a de Bruijn index (`bound 0`,
the innermost quantifier's), an eigenvariable by its number
(`eigen 0`), every instance with its quantifiers' terms substituted,
and the calculus's optional rules named by their module
(`Calculus.classical`, `Intuitionistic.affine`). A `∀` step names the
body it opens and its eigenvariable, which occurs nowhere in its
conclusion, an `∃` step its witness, here a constant that no `∀` step
introduces, as a domain is never empty:

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

An unprovable sequent gets a certificate too where its refutation is one
that `check` verifies, a saturated set excepted: a lemma that the mode's
calculus does not derive the sequent, proved by the library's theorem of
the refutation's invariant, which the library's test accepts by
computation. An unbalanced atom names the atom, a classical assignment
its true atoms, and the count equation nothing. A count with copies of
one sign and a Horn program's state equation are weighted counts: each
atom weighs an integer, `1` or `−1` on the counted atom or the state
equation's weights, the atoms no firing of the program reaches are
declared dead, and the library's test asks every contribution of the
sequent to the weighted count of its literals to be above zero, the
copies of a `?` formula included (with weakening, of the formulas with
one output). A sequent with a quantifier is stated over the first-order
calculus, its counts weighing a predicate whole or one closed atom; its
count equation has no certificate. An exhausted search has none either,
which a line on standard error says:

```console
$ deduxit prove --format rocq "A |- B"
(* unprovable (MLL, classical, focus engine): ~A occurs 1 more time than A in the one-sided sequent, so they cannot all meet in axioms *)
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

The library is the package `rocq-deduxit`, in a repository of its own,
[deduxit/rocq-deduxit](https://github.com/deduxit/rocq-deduxit),
on Rocq 9.0 to 9.3 and its standard library alone, its linear logic
under the logical path `Deduxit.Linear`; its version is deduxit's, so
deduxit 0.3.0's certificates compile against rocq-deduxit 0.3.0. Install it with opam, from the Rocq opam archive once the release
is there, or from its tag meanwhile:

```sh
opam repo add rocq-released https://rocq-prover.org/opam/released
opam install rocq-deduxit
opam pin add rocq-deduxit https://github.com/deduxit/rocq-deduxit.git#v0.3.0
```

`opam upgrade rocq-deduxit` takes a newer release from the archive and
`opam remove rocq-deduxit` removes it; a package pinned to a tag stays
there until pinned to the next.

or build it with Nix (`nix build github:deduxit/rocq-deduxit/v0.3.0`)
or `make` in a checkout with Rocq on the path. Installed, a certificate
compiles as it is; built in place, with the theories on the load path:

```console
$ deduxit prove --mix --format rocq --standalone --output mix.v "A, B |- A, B"
$ rocq compile -Q path/to/rocq-deduxit/theories Deduxit mix.v
```

A certificate is a module, named by its file, so another file (`use.v`
here) requires it and applies its lemma; an intuitionistic certificate
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

with the certificate written beside it as `mp.v` and both compiled under
one logical name:

```sh
deduxit prove -i --format rocq --standalone --output mp.v "A, A -o B |- B"
rocq compile -R . Proofs mp.v
rocq compile -R . Proofs use.v
```

The library's README says what it covers and what not, and its
documentation is at
[deduxit.github.io/rocq-deduxit](https://deduxit.github.io/rocq-deduxit/).
