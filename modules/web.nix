# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The web bindings (`web/', the crate deduxit-web) and the PNG and PDF
# renderers (`web-render/', deduxit-web-render, a module of their own that
# the page loads on its first such export) as a browser runs them: the
# package a page loads, both built for wasm32 with the wasm-bindgen CLI
# and wasm-opt, which the client's flake takes through its pinned input;
# and the checks that run them there. The CLI must be the version of the
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

      # The two modules' crates. Each is built by a cargo of its own, so
      # that the library's features are its own: built together, the
      # bindings' module would be compiled with the renderers' `png' and
      # `pdf'.
      modules = [
        "deduxit-web"
        "deduxit-web-render"
      ];
      eachModule = f: lib.concatMapStrings (crate: f crate + "\n") modules;
      webPackages = workspace.selections.web;

      # The library's unit tests on a 32-bit target: libtest runs on WASI
      # under wasmtime, which the web target has no runner for, and the two
      # share wasm32's data layout. Left out: the tests that start a thread
      # (a search on a thread of its own stack, a batch's workers), which
      # WASI cannot, and two that pin byte counts of 64-bit layouts: the
      # Horn engine's least memory for a search, and the bound at which a
      # proof's check, not its search, runs out of memory (on 32 bits the
      # check fits).
      core32Skipped = [
        "lambda::tests::any_depth_is_read_and_checked_on_a_small_stack"
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
            ${eachModule (crate: "cargoWithProfile build --locked --package ${crate}")}
            cargo test --locked ${webPackages} --no-run
            cargo test --locked --package deduxit-web-tab --no-run
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
          # The tab's crate once more alone, with only the library's features
          # it asks for, which the renderers' module is built with: tested
          # beside the bindings, it would have theirs.
          buildPhaseCargoCommand = ''
            cargo test --locked --target ${target} ${webPackages}
            cargo test --locked --target ${target} --package deduxit-web-tab
          '';
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
          # One file of notices for both modules, a section each.
          buildPhaseCargoCommand = ''
            ${eachModule (crate: "cargo build --locked --release --target ${target} --package ${crate}")}
            notices() {
              cargo about generate --frozen --manifest-path "$1/Cargo.toml" \
                --config web/about.toml web/about.hbs
            }
            {
              echo "deduxit_web_bg.wasm, deduxit's module, contains code of the following crates,"
              echo "under the licences below."
              notices web
              echo
              echo "================================================================================"
              echo "deduxit_web_render_bg.wasm, the module of deduxit's PNG and PDF renderers,"
              echo "contains code of the following crates, under the licences below."
              notices web-render
            } > THIRD-PARTY-NOTICES.txt
          '';
          installPhaseCommand = ''
            ${eachModule (
              crate:
              let
                name = lib.replaceStrings [ "-" ] [ "_" ] crate;
              in
              ''
                wasm-bindgen --target web --out-dir $out target/${target}/release/${name}.wasm
                wasm-opt -O3 $out/${name}_bg.wasm -o $out/${name}_bg.wasm
              ''
            )}
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
        # The WebAssembly features the module uses are those Rust enables
        # for wasm32 by default since 1.87 (LLVM's `generic' CPU), which
        # every current browser runs: a toolchain bump or a crate that
        # turns on another one (SIMD, threads, tail calls) fails here, and
        # moving the baseline is a decision of its own (web.md).
        web-features = pkgs.runCommand "check-web-features" { nativeBuildInputs = [ pkgs.wasm-tools ]; } ''
          for module in ${config.packages.deduxit-web}/*_bg.wasm; do
            wasm-tools validate \
              --features mvp,mutable-global,sign-extension,saturating-float-to-int,multi-value,reference-types,bulk-memory \
              "$module"
          done
          touch $out
        '';
        # Clippy on the bindings and their tests for the browser's target,
        # where a lint can fire that the native build never sees.
        web-clippy-wasm32 = craneLib.mkCargoDerivation (
          args
          // {
            pnameSuffix = "-web-clippy-wasm32";
            cargoArtifacts = wasmArtifacts;
            buildPhaseCargoCommand = ''
              cargo clippy --locked --target ${target} ${webPackages} --all-targets \
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
