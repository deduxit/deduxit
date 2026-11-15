# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# Kani, the bounded model checker of the checker's integer harnesses
# (`#[cfg(kani)]' modules of the core crate), and the package
# `kani-harnesses' that runs them. It is no flake check: a harness takes up
# to 8 GB and, on a CI runner, up to 25 minutes, so
# `.github/workflows/kani.yml' builds it weekly and when the checker
# changes, and `nix flake check' skips it.
#
# nixpkgs has no Kani, so it is built from the source the input `kani'
# pins, with the nightly its rust-toolchain.toml names (rustc's private
# crates are its interface, so no other compiler builds it) and
# nixpkgs' CBMC, the version its `kani-dependencies' names (an assertion
# compares them). The release bundle's layout is kept: `bin/' beside the
# libraries Kani compiles for its own sysroot and a `toolchain' link,
# where kani-driver looks for the cargo it runs. The bundle's `setup'
# step, which downloads, is never run.
#
# Linux on x86_64 only: the standard library's Cargo.lock, which the
# sysroot's `-Z build-std' builds against, is read from the toolchain at
# evaluation, and CI and the machine are x86_64-linux.
{ inputs, ... }:
{
  perSystem =
    {
      craneLib,
      lib,
      pkgs,
      system,
      workspace,
      ...
    }:
    let
      src = inputs.kani;
      version = (lib.importTOML "${src}/Cargo.toml").package.version;
      toolchainFile = "${src}/rust-toolchain.toml";
      channel = (lib.importTOML toolchainFile).toolchain.channel;
      nightly = (inputs.rust-overlay.lib.mkRustBin { } pkgs).fromRustupToolchainFile toolchainFile;
      craneKani = (inputs.crane.mkLib pkgs).overrideToolchain (_: nightly);

      # Kani's own dependencies and the standard library's, which its
      # sysroot rebuilds.
      vendor = craneKani.vendorMultipleCargoDeps {
        cargoLockList = [
          "${src}/Cargo.lock"
          "${nightly}/lib/rustlib/src/rust/library/Cargo.lock"
        ];
      };

      cbmcWanted = lib.pipe "${src}/kani-dependencies" [
        builtins.readFile
        (builtins.match ''.*CBMC_VERSION="([^"]+)".*'')
        builtins.head
      ];

      kani =
        assert lib.assertMsg (
          pkgs.cbmc.version == cbmcWanted
        ) "Kani ${version} wants CBMC ${cbmcWanted}, nixpkgs has ${pkgs.cbmc.version}";
        pkgs.stdenv.mkDerivation {
          pname = "kani";
          inherit version src;

          nativeBuildInputs = [
            nightly
            pkgs.makeWrapper
          ];

          # The toolchain's name, which Kani compiles into its binaries;
          # without rustup it only names the bundle's toolchain.
          env.RUSTUP_TOOLCHAIN = channel;

          # kani-compiler links rustc's driver from rustup's directory;
          # here that is the nightly's. Its optional Aeneas backend (`llbc')
          # names the git submodule `charon', which the tarball lacks and
          # cargo would have to resolve: the backend is dropped.
          postPatch = ''
            sed -i -e '/^charon = /d' -e "/^llbc = \['charon'\]/d" kani-compiler/Cargo.toml

            substituteInPlace kani-compiler/build.rs \
              --replace-fail 'let rustup_home = env::var("RUSTUP_HOME").unwrap();' "" \
              --replace-fail 'let rustup_tc = env::var("RUSTUP_TOOLCHAIN").unwrap();' "" \
              --replace-fail 'let rustup_lib = path_str!([&rustup_home, "toolchains", &rustup_tc, "lib"]);' \
                'let rustup_lib = String::from("${nightly}/lib");'
          '';

          configurePhase = ''
            runHook preConfigure
            export CARGO_HOME=$TMPDIR/cargo-home
            mkdir -p $CARGO_HOME
            cp ${vendor}/config.toml $CARGO_HOME/config.toml
            export CARGO_BUILD_JOBS=$NIX_BUILD_CORES
            runHook postConfigure
          '';

          # `build-dev --release' builds kani-compiler and kani-driver into
          # target/kani/bin and compiles Kani's libraries and the standard
          # library with kani-compiler into target/kani's sysroot.
          buildPhase = ''
            runHook preBuild
            cargo run --offline --release --target-dir target/tools -p build-kani -- build-dev --release
            runHook postBuild
          '';

          installPhase = ''
            runHook preInstall
            home=$out/lib/kani
            mkdir -p $home/bin $home/library $out/bin
            # target/kani/bin also holds the proxies that download a bundle.
            cp target/kani/bin/kani-driver target/kani/bin/kani-compiler $home/bin/
            cp -r target/kani/lib target/kani/playback target/kani/no_core $home/
            cp -r library/kani library/kani_macros library/std $home/library/
            ln -s ${nightly} $home/toolchain
            for name in kani cargo-kani; do
              makeWrapper $home/bin/kani-driver $out/bin/$name \
                --argv0 $name \
                --prefix PATH : ${
                  lib.makeBinPath [
                    pkgs.cbmc
                    pkgs.kissat
                    pkgs.stdenv.cc
                  ]
                }
            done
            runHook postInstall
          '';

          meta = {
            description = "Bit-precise bounded model checker for Rust";
            homepage = "https://github.com/model-checking/kani";
            license = with lib.licenses; [
              asl20
              mit
            ];
            mainProgram = "cargo-kani";
            platforms = [ "x86_64-linux" ];
          };
        };

      # The harnesses, every one, as `cargo kani' runs them in the shell:
      # the flags are the workspace's `[workspace.metadata.kani]'. The
      # library without its default features, which the checker needs
      # none of. One harness at a time: CBMC takes up to 8 GB for one, and
      # two of them beside other builds starved a 16 GB CI runner until
      # GitHub lost it. `--output-format terse' prints a summary per harness, not every
      # property.
      harnesses = craneLib.mkCargoDerivation (
        workspace.argsOf [ "core" ]
        // {
          pnameSuffix = "-kani";
          cargoArtifacts = null;
          nativeBuildInputs = [ kani ];
          buildPhaseCargoCommand = ''
            export HOME=$TMPDIR
            cargo kani --package deduxit --no-default-features --jobs 1 --output-format terse
          '';
          doInstallCargoArtifacts = false;
        }
      );
    in
    lib.optionalAttrs (system == "x86_64-linux") {
      packages = {
        inherit kani;
        kani-harnesses = harnesses;
      };
    };
}
