# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# What the collection may cost as it grows: the crates Cargo.lock holds and
# the web module's download, each under a ceiling. A change that passes one
# raises it here, in a commit that says what earned the room; the budget
# says when a cost grew, not whether it may.
{
  perSystem =
    {
      config,
      pkgs,
      ...
    }:
    let
      # The lock's packages, the workspace's own members among them: 173
      # on 2026-11-14.
      mostPackages = 180;
      packages = builtins.length (builtins.fromTOML (builtins.readFile ../Cargo.lock)).package;

      # The module the web page loads, in bytes at `gzip -9', what a
      # browser downloads: 2 842 389 on 2026-11-14 (2 603 759 at 0.2.0).
      mostModuleBytes = 3000000;
    in
    {
      checks.budget =
        pkgs.runCommand "check-budget"
          {
            nativeBuildInputs = [ pkgs.gzip ];
          }
          ''
            if [ ${toString packages} -gt ${toString mostPackages} ]; then
              echo "Cargo.lock holds ${toString packages} packages, over the budget of ${toString mostPackages}"
              exit 1
            fi
            bytes=$(gzip -9 -c ${config.packages.deduxit-web}/deduxit_web_bg.wasm | wc -c)
            if [ "$bytes" -gt ${toString mostModuleBytes} ]; then
              echo "the web module is $bytes bytes at gzip -9, over the budget of ${toString mostModuleBytes}"
              exit 1
            fi
            echo "${toString packages} packages, the module $bytes bytes at gzip -9" > $out
          '';
    };
}
