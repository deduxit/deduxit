# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The app `check-certificate': `nix run github:deduxit/deduxit#check-certificate
# -- proof.v' checks a Rocq certificate the command wrote, against the
# kernels the checks `rocq' and `rocq-deduxit' use (rocq.nix,
# rocq-deduxit.nix): NanoYalla from the input `nanoyalla', deduxit's library
# from the input `rocq-deduxit', and nixpkgs' Rocq and standard library. All
# are on the load path, and the file's own import lines pick the kernel, so
# no flag names it. The program is check-certificate.sh. And the check
# `check-certificate', which runs the app on certificates the command writes.
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

      nanoyallaVersion = builtins.head (
        builtins.match "# This is NanoYalla version ([0-9.]+).*" (
          builtins.readFile "${inputs.nanoyalla}/nanoyalla/README.md"
        )
      );

      # NanoYalla installed as a Rocq library: the two files a certificate
      # needs (`macroll' and the `nanoll' it imports), compiled under their
      # logical name. The cut rules the kernel offers in other files are
      # left out, since no certificate uses a cut.
      nanoyalla =
        pkgs.runCommand "nanoyalla-${nanoyallaVersion}"
          {
            nativeBuildInputs = [
              rocq
              stdlib
            ];
            ROCQPATH = "${stdlib}/${userContrib}";
          }
          ''
            export HOME=$TMPDIR
            dir=$out/${userContrib}/NanoYalla
            mkdir -p $dir
            # The kernel's licence (LGPL 2.1) travels with its compiled files.
            cp ${inputs.nanoyalla}/nanoyalla/{nanoll.v,macroll.v,README.md} ${inputs.nanoyalla}/LICENSE $dir/
            chmod -R u+w $dir
            cd $dir
            for file in nanoll macroll; do
              rocq compile -R . NanoYalla $file.v >kernel.log 2>&1 || { cat kernel.log; exit 1; }
            done
            rm -f kernel.log .*.aux
          '';

      app = pkgs.writeShellApplication {
        name = "check-certificate";
        runtimeInputs = [
          rocq
          pkgs.coreutils
          pkgs.gnugrep
          pkgs.gnused
        ];
        inheritPath = false;
        runtimeEnv = {
          ROCQ_LOAD_PATH = lib.concatMapStringsSep ":" (package: "${package}/${userContrib}") [
            stdlib
            nanoyalla
            library
          ];
          ROCQ_VERSION = rocq.version;
          NANOYALLA_VERSION = nanoyallaVersion;
          DEDUXIT_VERSION = library.version;
        };
        text = builtins.readFile ./check-certificate.sh;
      };
    in
    {
      apps.check-certificate = {
        program = app;
        meta.description = "Check a Rocq certificate that deduxit wrote, against the kernel it names";
      };

      checks.check-certificate =
        pkgs.runCommand "check-check-certificate"
          {
            nativeBuildInputs = [
              app
              config.packages.deduxit-cli
            ];
          }
          ''
            export HOME=$TMPDIR
            # Accepted: one certificate for each kernel, and a classical one
            # over Prop, which rests on the excluded middle. The app's output
            # names the kernel it read from the file.
            deduxit prove --format rocq --standalone --output nanoyalla.v 'A * B |- B * A'
            deduxit prove -i --format rocq --standalone --output deduxit.v 'A, A -o B |- B'
            deduxit prove --logic classical --format rocq --standalone --output prop.v \
              '((a -> b) -> a) -> a'
            accepts() {
              check-certificate "$1" | tee log
              grep -q "$2" log || { echo "$1 was not checked against $2"; exit 1; }
            }
            accepts nanoyalla.v NanoYalla
            accepts deduxit.v rocq-deduxit
            accepts prop.v "its standard library"
            # Refused, with exit status 1: a statement its proof does not
            # prove, a lemma left admitted, a lemma proved in a section from
            # a hypothesis that becomes its premise, and a certificate written
            # without its import lines.
            sed 's/tens B A\]/tens A A]/' nanoyalla.v >wrong.v
            if cmp -s nanoyalla.v wrong.v; then echo "no statement was changed"; exit 1; fi
            sed 's/^Qed\.$/Admitted./' deduxit.v >admitted.v
            if cmp -s deduxit.v admitted.v; then echo "no proof was left admitted"; exit 1; fi
            cat >section.v <<'EOF'
            From NanoYalla Require Import macroll.
            Section Absurd.
            Hypothesis absurd : False.
            Lemma certificate (A B : formula) : ll [A; B].
            Proof. destruct absurd. Qed.
            End Absurd.
            EOF
            deduxit prove --format rocq --output fragment.v 'A |- A'
            refuses() {
              status=0
              check-certificate "$1" >log 2>&1 || status=$?
              cat log
              [ "$status" -eq 1 ] || { echo "$1: exit status $status, not 1"; exit 1; }
            }
            refuses wrong.v
            refuses admitted.v
            refuses section.v
            refuses fragment.v
            grep -q -- --standalone log || { echo "no hint for the fragment"; exit 1; }
            touch $out
          '';
    };
}
