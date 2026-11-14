# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The Rocq certificates check: every certificate the core tests pin in
# core/tests/snapshots and those the CLI writes below are compiled against
# NanoYalla, the kernel the scripts are written for, as the app
# `check-certificate' builds it (check-certificate.nix: the flake input
# compiled with Rocq and its standard library from nixpkgs), so the kernel
# is built once. Rocq prints nothing for a script it accepts, so any output
# from a certificate fails the check. `Qed' closes every lemma, so a script
# Rocq accepts is a proof the kernel checked, not one it admitted. The
# closure is Rocq's, about 1.2 GB, all from the binary cache.
{ inputs, ... }:
{
  perSystem =
    {
      config,
      pkgs,
      nanoyalla,
      ...
    }:
    let
      snapshots = pkgs.lib.fileset.toSource {
        root = ../core/tests/snapshots;
        fileset = pkgs.lib.fileset.fileFilter (file: file.hasExt "v") ../core/tests/snapshots;
      };

      stdlib = pkgs.rocqPackages.stdlib;
      userContrib = "lib/coq/${pkgs.rocq-core.rocq-version}/user-contrib";

      # The version `deduxit::export::rocq::NANOYALLA' names must be the
      # kernel's the input pins, whose README says which it is.
      named = builtins.head (
        builtins.match ''.*pub const NANOYALLA: &str = "([0-9.]+)";.*'' (
          builtins.readFile ../core/src/export/rocq/mod.rs
        )
      );
      pinned = builtins.head (
        builtins.match "# This is NanoYalla version ([0-9.]+).*" (
          builtins.readFile "${inputs.nanoyalla}/nanoyalla/README.md"
        )
      );
    in
    {
      checks.rocq =
        assert pkgs.lib.assertMsg (
          named == pinned
        ) "export::rocq::NANOYALLA is ${named}, the pinned kernel ${pinned}";
        pkgs.runCommand "check-rocq"
          {
            nativeBuildInputs = [
              config.packages.deduxit-cli
              pkgs.rocq-core
              stdlib
            ];
            ROCQPATH = "${stdlib}/${userContrib}:${nanoyalla}/${userContrib}";
          }
          ''
            export HOME=$TMPDIR
            cp ${snapshots}/*.v .
            deduxit prove --format rocq --standalone --output cli.v 'A * top |- A * (B + top)'
            # An intuitionistic proof goes to deduxit's library unless the
            # kernel is named.
            deduxit prove -i --kernel nanoyalla --format rocq --standalone --output cli_ill.v \
              '!A, A -o B |- B * !A'
            # Certificates over Prop of ordinary logic: classical through the
            # excluded middle, intuitionistic with no axiom.
            deduxit prove --logic classical --format rocq --standalone --output cli_lk.v \
              '(a <-> b) -> ~(a /\ ~b) /\ (((a -> c) -> a) -> a)'
            deduxit prove --logic intuitionistic --format rocq --standalone --output cli_lj.v \
              'a \/ b, ~a, (b -> c) /\ true |- ~~c'
            # LJT's derivations hold cuts, which a certificate writes as lets;
            # the embedding's hold none.
            deduxit prove --logic intuitionistic --method embedding --copies 8 --format rocq \
              --standalone --output cli_lj_embedding.v 'a \/ b, ~a, (b -> c) /\ true |- ~~c'
            deduxit prove --logic minimal --format rocq --standalone --output cli_lm.v \
              '(a \/ b) -> c, (a <-> b) -> d, (a -> b) -> e |- (a -> c) /\ ((b -> a) -> (a -> b) -> d) /\ (b -> e)'
            # Without atoms or hypotheses, a certificate binds nothing.
            deduxit prove --logic classical --format rocq --standalone --output cli_lk_true.v 'true'
            deduxit prove --logic intuitionistic --format rocq --standalone --output cli_lj_true.v 'true'
            # Not valid, by an assignment of the image's classical refutation;
            # the command exits with 1 for a sequent that is not valid.
            deduxit prove --logic classical --format rocq --standalone --output cli_lk_not_valid.v \
              'a -> b, b |- a' || [ $? -eq 1 ]
            deduxit prove --logic intuitionistic --format rocq --standalone --output cli_lj_not_valid.v \
              'a \/ (b -> a)' || [ $? -eq 1 ]
            for file in *.v; do
              rocq compile "$file" 2>&1 | tee log
              if [ -s log ]; then exit 1; fi
            done
            touch $out
          '';
    };
}
