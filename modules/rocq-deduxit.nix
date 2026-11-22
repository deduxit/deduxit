# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The check `rocq-deduxit': every certificate the core tests pin for
# deduxit's own Rocq library in core/tests/snapshots/deduxit, and some the
# CLI writes, first-order and modal ones among them, compiled against that library as
# the flake input `rocq-deduxit' pins it (github:deduxit/rocq-deduxit, built with
# this flake's nixpkgs, so with the Rocq the NanoYalla check uses, rocq.nix);
# a certificate with a planted fault (a wrong witness, a wrong weight, a
# valuation that does not grow along the order, a first-order model whose
# facts do not grow, a modal step on the wrong
# edges, a modal model that misses its system's frame condition) must
# be refused, and a file that requires a certificate and applies its lemma
# compiles. Rocq prints nothing for
# a file it accepts, so any other output fails the check, and `Print
# Assumptions' of each certificate's lemma must answer that it is closed
# under the global context. And the check
# `rocq-deduxit-library': the library's own check, re-exported from the input,
# which compiles its tests and requires its theorems' assumptions to be
# closed under the global context, so that every pin of the library these
# certificates rest on is verified here too (one derivation with the
# library's own, so a pin already checked there is fetched or cached, not
# rerun). Its bridges are checked in its repository alone.
{ inputs, ... }:
{
  perSystem =
    {
      config,
      pkgs,
      system,
      ...
    }:
    let
      inherit (pkgs) lib;
      rocq = pkgs.rocq-core;
      stdlib = pkgs.rocqPackages.stdlib;
      library = inputs.rocq-deduxit.packages.${system}.rocq-deduxit;
      userContrib = "lib/coq/${rocq.rocq-version}/user-contrib";

      snapshots = lib.fileset.toSource {
        root = ../core/tests/snapshots/deduxit;
        fileset = lib.fileset.fileFilter (file: file.hasExt "v") ../core/tests/snapshots/deduxit;
      };

      # The version `deduxit::export::rocq::DEDUXIT' names must be the pinned
      # library's.
      named = builtins.head (
        builtins.match ''.*pub const DEDUXIT: &str = "([0-9.]+)";.*'' (
          builtins.readFile ../core/src/export/rocq/mod.rs
        )
      );
    in
    {
      checks.rocq-deduxit-library = inputs.rocq-deduxit.checks.${system}.rocq-deduxit;

      checks.rocq-deduxit =
        assert lib.assertMsg (
          named == library.version
        ) "export::rocq::DEDUXIT is ${named}, the pinned library ${library.version}";
        pkgs.runCommand "check-rocq-deduxit"
          {
            nativeBuildInputs = [
              rocq
              stdlib
              library
              config.packages.deduxit-cli
            ];
            ROCQPATH = "${stdlib}/${userContrib}:${library}/${userContrib}";
          }
          ''
            export HOME=$TMPDIR
            mkdir exporter
            cp ${snapshots}/*.v exporter/
            cd exporter
            deduxit prove --mix --format rocq --standalone --output cli_mix.v 'A, B |- A, B * (C -o C)'
            deduxit prove --affine --format rocq --standalone --output cli_affine.v '!A, A -o B, C |- B'
            deduxit prove -i --format rocq --standalone --output cli_ill.v '!A, A -o B |- B * !A'
            deduxit prove -i --affine --format rocq --standalone --output cli_ill_affine.v \
              'A, B, A -o C |- C * 1'
            # A goal built from top and 0 alone: the reading's goal.
            deduxit prove -i --format rocq --standalone --output cli_ill_units.v '0, top |- top'
            deduxit prove --kernel deduxit --format rocq --standalone --output cli_classical.v \
              '!(A & B) |- !A * !B'
            # Certificates of non-derivability, one-sided and two-sided; the
            # command exits with 1 for an unprovable sequent.
            deduxit prove --format rocq --standalone --output cli_refuted.v 'A, A -o B |- B * B' || [ $? -eq 1 ]
            deduxit prove --mix --format rocq --standalone --output cli_refuted_mix.v '|- (A * ~A) * 1' || [ $? -eq 1 ]
            deduxit prove -i --format rocq --standalone --output cli_refuted_ill.v 'A + B |- A & B' || [ $? -eq 1 ]
            deduxit prove --copies 1 --format rocq --standalone --output cli_refuted_unknown.v \
              '!(A -o A * A), A |- ?B' || [ $? -eq 1 ]
            # Weighted counts: a count with copies of one sign, state
            # equations (one with atoms no firing reaches, one with a clause
            # used once, one of coverability), and a predicate weighed whole.
            deduxit prove --format rocq --standalone --output cli_refuted_one_sided.v \
              '!d, !((c * d * d) -o 1) |- c * c' || [ $? -eq 1 ]
            deduxit prove -i --format rocq --standalone --output cli_refuted_dead.v \
              '!(a -o a * a), !(a * c -o b * c), a |- b' || [ $? -eq 1 ]
            deduxit prove -i --format rocq --standalone --output cli_refuted_once.v \
              'a, a -o b, !(b -o b * c), !(b -o d) |- b * d' || [ $? -eq 1 ]
            deduxit prove -i --affine --format rocq --standalone --output cli_refuted_cover.v \
              '!(a -o a * c), !(a -o b), a |- a * b' || [ $? -eq 1 ]
            deduxit prove --format rocq --standalone --output cli_refuted_fo.v \
              '|- ~p(a), ?(exists x. ~p(x))' || [ $? -eq 1 ]
            # First-order proofs: classical with a witness no ∀ step
            # introduces, two-sided, affine, and a session's open witness
            # closed into such a constant.
            deduxit prove --kernel deduxit --format rocq --standalone --output cli_fo.v \
              'forall x. p(x) |- exists y. p(y)'
            deduxit prove -i --format rocq --standalone --output cli_fo_ill.v \
              'exists y. forall x. r(x, y) |- forall x. exists y. r(x, y)'
            deduxit prove --affine --format rocq --standalone --output cli_fo_affine.v \
              'forall x. p(x), q |- exists y. p(y)'
            deduxit prove -i --affine --format rocq --standalone --output cli_fo_ill_affine.v \
              'forall x. p(x), q |- exists y. p(y)'
            # Kripke countermodels of intuitionistic and minimal logic: two
            # the search found, one from checking an outcome, one from a
            # model the user gives.
            deduxit prove --logic intuitionistic --format rocq --standalone --output cli_kripke.v \
              '|- a \/ ~a' || [ $? -eq 1 ]
            deduxit prove --logic minimal --format rocq --standalone --output cli_kripke_minimal.v \
              'a, ~a |- b' || [ $? -eq 1 ]
            deduxit prove --logic intuitionistic --format json --output ../peirce.json \
              '((a -> b) -> a) -> a' || [ $? -eq 1 ]
            deduxit check --format rocq --standalone --output cli_kripke_check.v ../peirce.json
            echo '{"version": 13, "worlds": [{"atoms": [], "arrows": [1]}, {"atoms": ["a"]}]}' >../model.json
            deduxit check --logic intuitionistic --sequent '~~a |- a' --format rocq --standalone \
              --output cli_kripke_model.v ../model.json || [ $? -eq 1 ]
            for file in cli_kripke*.v; do grep -q '^Lemma certificate' "$file"; done
            # First-order countermodels: a Kripke model with growing domains
            # and a finite structure the model search finds, and a Kripke
            # model from checking an outcome.
            deduxit prove --logic intuitionistic --format rocq --standalone --output cli_fo_kripke.v \
              '~(forall x. p(x)) |- exists x. ~p(x)' || [ $? -eq 1 ]
            deduxit prove --logic classical --method models --format rocq --standalone \
              --output cli_fo_structure.v 'p(a) |- p(f(a))' || [ $? -eq 1 ]
            deduxit prove --logic intuitionistic --format json --output ../fo.json \
              'forall x. (p(x) \/ q) |- (forall x. p(x)) \/ q' || [ $? -eq 1 ]
            deduxit check --format rocq --standalone --output cli_fo_kripke_check.v ../fo.json
            for file in cli_fo_kripke*.v cli_fo_structure.v; do grep -q '^Lemma certificate' "$file"; done
            printf '%s\n' 'apply 0 0 ∀L' 'apply 1 0 ∃R' 'close 2' 'proof --rocq cli_fo_session.v' |
              deduxit interact -i --style rocq.form=standalone 'forall x. p(x) |- exists y. p(y)' >/dev/null
            # Modal logic: a valid sequent for each frame rule, a derivation
            # a session built, one from checking an outcome, and a
            # countermodel for each kind of system.
            deduxit prove --system T --format rocq --standalone --output cli_modal_ref.v '[]p |- p'
            deduxit prove --system D --format rocq --standalone --output cli_modal_ser.v '[]p |- <>p'
            deduxit prove --system B --format rocq --standalone --output cli_modal_sym.v 'p |- []<>p'
            deduxit prove --system S4 --format rocq --standalone --output cli_modal_trans.v '[]p -> [][]p'
            deduxit prove --system KD45 --format rocq --standalone --output cli_modal_eucl.v \
              '<>p |- []<>p'
            printf '%s\n' 'derive 0 L□ 0 1' 'apply 1 ax 2 3' 'proof --rocq cli_modal_session.v' |
              deduxit interact --calculus G3K --style rocq.form=standalone 'w0 : []p, w0 R w1 |- w1 : p' \
              >/dev/null
            deduxit prove --system S5 --format json --output ../s5.json '<>[]p -> []p'
            deduxit check --format rocq --standalone --output cli_modal_check.v ../s5.json
            for system in K T S4 KD45 S5; do
              deduxit prove --system $system --format rocq --standalone \
                --output "cli_modal_countermodel_$system.v" '<>p |- []p' || [ $? -eq 1 ]
            done
            for file in cli_modal_*.v; do grep -q '^Lemma certificate' "$file"; done
            for file in *.v; do
              rocq compile "$file" 2>&1 | tee log
              if [ -s log ]; then exit 1; fi
            done
            # Every certificate's lemma rests on the library's theorems
            # alone: its assumptions are closed under the global context.
            mkdir ../assumptions
            for file in *.v; do
              { cat "$file"; echo; echo 'Print Assumptions certificate.'; } >"../assumptions/$file"
              (cd ../assumptions && rocq compile "$file") >log 2>&1 || { cat log; exit 1; }
              if [ "$(cat log)" != 'Closed under the global context' ]; then
                echo "$file:"
                cat log
                exit 1
              fi
            done
            echo "$(ls ../assumptions/*.v | wc -l) certificates, each closed under the global context"
            # A planted fault: the first certificate with a wrong witness is
            # refused.
            mkdir ../fault
            sed 's/(Exists (var "p" \[bound 0\]) (eigen 0)/(Exists (var "p" [bound 0]) (eigen 1)/' \
              cli_fo.v >../fault/fault.v
            if cmp -s cli_fo.v ../fault/fault.v; then
              echo "no fault was planted"
              exit 1
            fi
            if rocq compile ../fault/fault.v >/dev/null 2>&1; then
              echo "a certificate with a wrong witness was accepted"
              exit 1
            fi
            # And a state equation with a wrong weight, under which the
            # copies of a clause raise the count.
            sed 's/("d", 1%Z)/("d", 2%Z)/' cli_refuted_once.v >../fault/weight.v
            if cmp -s cli_refuted_once.v ../fault/weight.v; then
              echo "no weight was planted"
              exit 1
            fi
            if rocq compile ../fault/weight.v >/dev/null 2>&1; then
              echo "a certificate with a wrong weight was accepted"
              exit 1
            fi
            # And a model whose atom a later world forgets: one the sequent
            # does not name, so only the model's growth fails.
            sed 's/atoms := \[\[\]; \["a"\]\]/atoms := [["b"]; ["a"]]/' cli_kripke_model.v >../fault/model.v
            if cmp -s cli_kripke_model.v ../fault/model.v; then
              echo "no model was planted"
              exit 1
            fi
            if rocq compile ../fault/model.v >/dev/null 2>&1; then
              echo "a certificate with a model that is no Kripke model was accepted"
              exit 1
            fi
            # And a first-order model whose root holds a fact the world above
            # it drops: one forcing does not need, so only the model's growth
            # fails.
            sed 's/facts := \[\[\]; \[("p", \[0\])\]\]/facts := [[("p", [1])]; [("p", [0])]]/' \
              countermodel_fo_not_forall.v >../fault/facts.v
            if cmp -s countermodel_fo_not_forall.v ../fault/facts.v; then
              echo "no first-order model was planted"
              exit 1
            fi
            if rocq compile ../fault/facts.v >/dev/null 2>&1; then
              echo "a first-order certificate with a model that is no Kripke model was accepted"
              exit 1
            fi
            # And a modal term whose transitivity step names its edges wrongly,
            # and a modal countermodel of S4 whose world forgets its loop.
            sed 's/Trans "w0" "w1" "w2"/Trans "w0" "w2" "w1"/' cli_modal_trans.v >../fault/modal.v
            if cmp -s cli_modal_trans.v ../fault/modal.v; then
              echo "no modal term was planted"
              exit 1
            fi
            if rocq compile ../fault/modal.v >/dev/null 2>&1; then
              echo "a modal certificate with a wrong step was accepted"
              exit 1
            fi
            sed 's/arrows := \[\[0; /arrows := [[/' cli_modal_countermodel_S4.v >../fault/frame.v
            if cmp -s cli_modal_countermodel_S4.v ../fault/frame.v; then
              echo "no frame fault was planted"
              exit 1
            fi
            if rocq compile ../fault/frame.v >/dev/null 2>&1; then
              echo "a modal countermodel that is no model of S4 was accepted"
              exit 1
            fi
            # A certificate is a module that another file requires, applying
            # its lemma to the two-sided goal, as the manual's Rocq chapter
            # shows (docs/src/rocq.md).
            mkdir ../use
            cd ../use
            deduxit prove -i --format rocq --standalone --output mp.v 'A, A -o B |- B'
            cat >use.v <<'EOF'
            From Stdlib Require Import List String.
            From Deduxit.Linear Require Import IntuitionisticCheck.
            Import ListNotations.
            Open Scope string_scope.
            From Proofs Require mp.

            Goal ill (atom := string) intuitionistic [ivar "A"; ilmap (ivar "A") (ivar "B")] (ivar "B").
            Proof. apply mp.certificate. Qed.
            EOF
            rocq compile -R . Proofs mp.v
            rocq compile -R . Proofs use.v 2>&1 | tee log
            if [ -s log ]; then exit 1; fi
            touch $out
          '';
    };
}
