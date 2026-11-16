# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The JSON documents the command writes match the wire schema the manual
# publishes (docs/src/wire.schema.json): _schema/documents.sh writes one of
# every form on small inputs, and _schema/check.py validates each and
# requires a document of every form the command can write.
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
        fileset = lib.fileset.unions [
          ../docs/src/wire.schema.json
          ./_schema
        ];
      };
      python = pkgs.python3.withPackages (ps: [ ps.jsonschema ]);
    in
    {
      checks.wire-schema =
        pkgs.runCommand "check-wire-schema"
          {
            nativeBuildInputs = [ python ];
          }
          ''
            mkdir documents
            written=0
            bash ${source}/modules/_schema/documents.sh \
              ${config.packages.deduxit-cli}/bin/deduxit documents || written=$?
            checked=0
            python3 ${source}/modules/_schema/check.py \
              ${source}/docs/src/wire.schema.json documents > report || checked=$?
            cat report
            if [ "$written" != 0 ] || [ "$checked" != 0 ]; then
              exit 1
            fi
            cp report $out
          '';
    };
}
