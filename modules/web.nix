# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The web bindings (`web/', the crate deduxit-web) as a browser runs them:
# the package a page loads, built for wasm32 with the wasm-bindgen CLI and
# wasm-opt, which the client's flake takes through its pinned input; and
# the checks that run them there. The CLI must be the version of the
# wasm-bindgen crate that Cargo.lock pins, or the module it writes does
# not load: it is picked by that version from nixpkgs' versioned packages,
# and `web-bindgen' compares the two. A bump of the crate the nixpkgs in
# flake.lock has no CLI for fails here at evaluation.
{
  perSystem =
    {
      config,
      craneLib,
      lib,
      pkgs,
      workspace,
      ...
    }:
    let
      lock = builtins.fromTOML (builtins.readFile ../Cargo.lock);
      bindgenVersion = (lib.findFirst (p: p.name == "wasm-bindgen") null lock.package).version;
      cliName = "wasm-bindgen-cli_${lib.replaceStrings [ "." ] [ "_" ] bindgenVersion}";
      wasm-bindgen-cli =
        pkgs.${cliName}
          or (throw "Cargo.lock pins wasm-bindgen ${bindgenVersion}, and nixpkgs has no ${cliName}");

      target = "wasm32-unknown-unknown";

      # The library's unit tests on a 32-bit target: libtest runs on WASI
      # under wasmtime, which the web target has no runner for, and the two
      # share wasm32's data layout. Left out: the tests that start a thread
      # (a search on a thread of its own stack, a batch's workers), which
      # WASI cannot, and two that pin byte counts of 64-bit layouts: the
      # Horn engine's least memory for a search, and the bound at which a
      # proof's check, not its search, runs out of memory (on 32 bits the
      # check fits).
      core32Skipped = [
        "lambda::tests::any_height_reads_back_on_a_small_stack"
        "linear::nets::sequentialize::tests::a_high_derivation_needs_no_stack"
        "linear::proofs::derivation::tests::any_height_on_a_small_stack"
        "linear::proofs::interactive::tests::a_high_derivation_needs_no_stack"
        "linear::search::batch::tests::a_panic_ends_the_batch_at_its_place"
        "linear::search::focus::tests::limits"
        "linear::search::focus::tests::stops_inside_a_forced_chain"
        "linear::search::horn::tests::charges_what_it_grows"
        "linear::search::tests::a_check_past_the_memory_bound_is_unknown"
      ];

      args = workspace.argsOf [
        "core"
        "core-tests"
        "web"
        "web-tests"
      ];

      # The dependencies for wasm32, of the package and of its tests.
      wasmArtifacts = craneLib.buildDepsOnly (
        args
        // {
          pname = "deduxit-web-wasm";
          CARGO_BUILD_TARGET = target;
          buildPhaseCargoCommand = ''
            cargoWithProfile build --locked --package deduxit-web
            cargo test --locked --package deduxit-web --no-run
          '';
          checkPhaseCargoCommand = "";
          doCheck = false;
        }
      );

      # The bindings' tests under Node, as wasm-bindgen-test-runner runs
      # them with Node's own stack, which is a browser's size.
      wasmTests = craneLib.mkCargoDerivation (
        args
        // {
          pnameSuffix = "-web-wasm-tests";
          cargoArtifacts = wasmArtifacts;
          nativeBuildInputs = [
            wasm-bindgen-cli
            pkgs.nodejs-slim
          ];
          CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER = "wasm-bindgen-test-runner";
          WASM_BINDGEN_TEST_TIMEOUT = "300";
          buildPhaseCargoCommand = "cargo test --locked --target ${target} --package deduxit-web";
          doInstallCargoArtifacts = false;
        }
      );

      core32Tests = craneLib.mkCargoDerivation (
        workspace.argsOf [
          "core"
          "core-tests"
        ]
        // {
          pnameSuffix = "-core-wasm32";
          cargoArtifacts = null;
          nativeBuildInputs = [ pkgs.wasmtime ];
          # Some tests read files of the source tree (the fonts, the pinned
          # proofs) by the absolute path cargo compiled in.
          buildPhaseCargoCommand = ''
            export HOME=$TMPDIR
            export CARGO_TARGET_WASM32_WASIP1_RUNNER="wasmtime run --dir=$PWD::$PWD --"
            cargo test --locked --target wasm32-wasip1 --package deduxit --lib -- \
              ${lib.concatMapStringsSep " " (name: "--skip ${name}") core32Skipped}
          '';
          doInstallCargoArtifacts = false;
        }
      );
    in
    {
      packages.deduxit-web = craneLib.mkCargoDerivation (
        args
        // {
          pname = "deduxit-web";
          inherit (craneLib.crateNameFromCargoToml { cargoToml = ../Cargo.toml; }) version;
          cargoArtifacts = wasmArtifacts;
          nativeBuildInputs = [
            wasm-bindgen-cli
            pkgs.binaryen
            pkgs.cargo-about
          ];
          buildPhaseCargoCommand = ''
            cargo build --locked --release --target ${target} --package deduxit-web
            cargo about generate --frozen --manifest-path web/Cargo.toml \
              --config web/about.toml web/about.hbs --output-file THIRD-PARTY-NOTICES.txt
          '';
          installPhaseCommand = ''
            wasm-bindgen --target web --out-dir $out \
              target/${target}/release/deduxit_web.wasm
            wasm-opt -O3 $out/deduxit_web_bg.wasm -o $out/deduxit_web_bg.wasm
            install -Dm644 LICENSE $out/LICENSE
            install -Dm644 THIRD-PARTY-NOTICES.txt $out/THIRD-PARTY-NOTICES.txt
          '';
          doInstallCargoArtifacts = false;
        }
      );

      checks = {
        web-package = config.packages.deduxit-web;
        web-wasm-tests = wasmTests;
        web-core-wasm32 = core32Tests;
        # Clippy on the bindings and their tests for the browser's target,
        # where a lint can fire that the native build never sees.
        web-clippy-wasm32 = craneLib.mkCargoDerivation (
          args
          // {
            pnameSuffix = "-web-clippy-wasm32";
            cargoArtifacts = wasmArtifacts;
            buildPhaseCargoCommand = ''
              cargo clippy --locked --target ${target} --package deduxit-web --all-targets \
                -- --deny warnings
            '';
            doInstallCargoArtifacts = false;
          }
        );
        # The CLI the package is built with speaks the crate's version.
        web-bindgen = pkgs.runCommand "check-web-bindgen" { nativeBuildInputs = [ wasm-bindgen-cli ]; } ''
          test "$(wasm-bindgen --version)" = "wasm-bindgen ${bindgenVersion}"
          touch $out
        '';
      };

      devshells.default.devshell.packages = [
        wasm-bindgen-cli
        pkgs.binaryen
        pkgs.nodejs-slim
        pkgs.wasmtime
      ];
    };
}
