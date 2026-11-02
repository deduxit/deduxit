# linlog © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The check `rocq-linlog': every certificate the core tests pin for linlog's
# own Rocq library in core/tests/snapshots/linlog, and some the CLI writes,
# first-order ones among them, compiled against that library as the flake
# input `rocq-linlog' pins it (github:linlog-prover/rocq-linlog, built with
# this flake's nixpkgs, so with the Rocq the NanoYalla check uses, rocq.nix);
# a certificate with a planted fault must be refused. Rocq prints nothing for
# a file it accepts, so any other output fails the check. The library's own
# tests, its bridges and its theorems' assumptions are checked in its
# repository.
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
      library = inputs.rocq-linlog.packages.${system}.rocq-linlog;
      userContrib = "lib/coq/${rocq.rocq-version}/user-contrib";

      snapshots = lib.fileset.toSource {
        root = ../core/tests/snapshots/linlog;
        fileset = lib.fileset.fileFilter (file: file.hasExt "v") ../core/tests/snapshots/linlog;
      };

      # The version `linlog::export::rocq::LINLOG' names must be the pinned
      # library's.
      named = builtins.head (
        builtins.match ''.*pub const LINLOG: &str = "([0-9.]+)";.*'' (
          builtins.readFile ../core/src/export/rocq/mod.rs
        )
      );
    in
    {
      checks.rocq-linlog =
        assert lib.assertMsg (
          named == library.version
        ) "export::rocq::LINLOG is ${named}, the pinned library ${library.version}";
        pkgs.runCommand "check-rocq-linlog"
          {
            nativeBuildInputs = [
              rocq
              stdlib
              library
              config.packages.linlog-cli
            ];
            ROCQPATH = "${stdlib}/${userContrib}:${library}/${userContrib}";
          }
          ''
            export HOME=$TMPDIR
            mkdir exporter
            cp ${snapshots}/*.v exporter/
            cd exporter
            linlog prove --mix --format rocq --standalone --output cli_mix.v 'A, B |- A, B * (C -o C)'
            linlog prove --affine --format rocq --standalone --output cli_affine.v '!A, A -o B, C |- B'
            linlog prove -i --format rocq --standalone --output cli_ill.v '!A, A -o B |- B * !A'
            linlog prove -i --affine --format rocq --standalone --output cli_ill_affine.v \
              'A, B, A -o C |- C * 1'
            # A goal built from top and 0 alone: the reading's goal.
            linlog prove -i --format rocq --standalone --output cli_ill_units.v '0, top |- top'
            linlog prove --kernel linlog --format rocq --standalone --output cli_classical.v \
              '!(A & B) |- !A * !B'
            # Certificates of non-derivability, one-sided and two-sided; the
            # command exits with 1 for an unprovable sequent.
            linlog prove --format rocq --standalone --output cli_refuted.v 'A, A -o B |- B * B' || [ $? -eq 1 ]
            linlog prove --mix --format rocq --standalone --output cli_refuted_mix.v '|- (A * ~A) * 1' || [ $? -eq 1 ]
            linlog prove -i --format rocq --standalone --output cli_refuted_ill.v 'A + B |- A & B' || [ $? -eq 1 ]
            linlog prove --copies 1 --format rocq --standalone --output cli_refuted_unknown.v \
              '!(A -o A * A), A |- ?B' || [ $? -eq 1 ]
            # First-order proofs: classical with a witness no ∀ step
            # introduces, two-sided, affine, and a session's open witness
            # closed into such a constant.
            linlog prove --kernel linlog --format rocq --standalone --output cli_fo.v \
              'forall x. p(x) |- exists y. p(y)'
            linlog prove -i --format rocq --standalone --output cli_fo_ill.v \
              'exists y. forall x. r(x, y) |- forall x. exists y. r(x, y)'
            linlog prove --affine --format rocq --standalone --output cli_fo_affine.v \
              'forall x. p(x), q |- exists y. p(y)'
            linlog prove -i --affine --format rocq --standalone --output cli_fo_ill_affine.v \
              'forall x. p(x), q |- exists y. p(y)'
            printf '%s\n' 'apply 0 0 ∀L' 'apply 1 0 ∃R' 'close 2' 'proof --rocq cli_fo_session.v' |
              linlog interact -i --style rocq.form=standalone 'forall x. p(x) |- exists y. p(y)' >/dev/null
            for file in *.v; do
              rocq compile "$file" 2>&1 | tee log
              if [ -s log ]; then exit 1; fi
            done
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
            touch $out
          '';
    };
}
