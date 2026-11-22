# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The checks as CI runs them: one GitHub Actions job per group below, each
# on a runner of its own, so that the long builds run side by side instead
# of sharing one runner's four cores. `.github/workflows/checks.yml' reads
# the matrix with `nix eval --json .#githubActions.matrix' and builds each
# group's checks with `nix build'. The output's name and shape follow
# nix-community/nix-github-actions (`name', `system', `os' per entry), with
# the checks of a group as a list.
#
# A group holds the checks that build the same package (the command, the
# harness, the web module, the documentation), so that no two runners build
# one derivation, or a check heavy enough for a runner of its own. Every
# check no group names runs in the group `rest', so a new check is never
# left out; a name here that is no check fails the evaluation.
{ lib, withSystem, ... }:
{
  flake.githubActions = withSystem "x86_64-linux" (
    { config, system, ... }:
    let
      groups = {
        # The longest checks, alone.
        features = [ "features" ];
        features-tests = [ "features-tests" ];
        core-tests = [ "test-core" ];
        core-tests-debug-assertions = [ "test-debug-assertions-core" ];
        # The library on WebAssembly: its unit tests on wasm32-wasip1, and
        # Clippy for the browser's target.
        core-wasm32 = [
          "web-core-wasm32"
          "wasm"
        ];
        # The bindings' tests natively, and the package `deduxit-web' with
        # every check that reads it.
        web = [
          "test-web"
          "test-debug-assertions-web"
          "clippy-web"
        ];
        web-package = [
          "web-package"
          "web-features"
          "budget"
          "web-wasm-tests"
          "web-clippy-wasm32"
          "web-bindgen"
        ];
        cli = [
          "test-cli"
          "test-debug-assertions-cli"
          "clippy-cli"
        ];
        # The package `deduxit-cli' and every check that runs it.
        cli-outputs = [
          "build"
          "export"
          "rocq"
          "rocq-deduxit"
          "rocq-deduxit-library"
          "wire-schema"
          "check-certificate"
        ];
        # The package `deduxit-bench' and every check that runs it, with
        # the harness's own tests.
        bench = [
          "bench"
          "ratchet"
          "test-bench"
          "test-debug-assertions-bench"
          "clippy-bench"
        ];
        # The published site (rustdoc and the manual) and what reads it.
        docs = [
          "doc"
          "doc-private"
          "manual"
          "manual-links"
        ];
        compat = [
          "test-compat"
          "test-debug-assertions-compat"
          "clippy-compat"
          "package"
          "msrv"
        ];
      };

      checks = builtins.attrNames config.checks;
      named = lib.concatLists (builtins.attrValues groups);
      unknown = lib.subtractLists checks named;
      twice = lib.filter (name: lib.count (n: n == name) named > 1) (lib.unique named);
      rest = lib.subtractLists named checks;
    in
    assert lib.assertMsg (unknown == [ ]) "modules/ci.nix names no check: ${toString unknown}";
    assert lib.assertMsg (twice == [ ]) "modules/ci.nix names a check twice: ${toString twice}";
    {
      matrix.include = lib.mapAttrsToList (name: checks: {
        inherit name system checks;
        os = "ubuntu-latest";
      }) (groups // lib.optionalAttrs (rest != [ ]) { inherit rest; });
    }
  );
}
