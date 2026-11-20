# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The dataset of problems with verified answers (dataset/): every tenth
# problem is decided again by the command built here and its stored
# answer graded again, which must give the same verdicts; the data file's
# checksum must match SHA256SUMS.
{
  perSystem =
    {
      config,
      lib,
      pkgs,
      ...
    }:
    let
      source = lib.fileset.toSource {
        root = ../.;
        fileset = ../dataset;
      };
    in
    {
      checks.dataset =
        pkgs.runCommand "check-dataset"
          {
            nativeBuildInputs = [ pkgs.python3 ];
          }
          ''
            cd ${source}/dataset
            sha256sum --check --quiet SHA256SUMS
            python3 build.py verify --deduxit ${config.packages.deduxit-cli}/bin/deduxit \
              problems.jsonl.gz > $out
            cat $out
          '';
    };
}
