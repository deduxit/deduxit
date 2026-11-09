# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The command as a release ships it for Linux: `deduxit-static', the binary
# linked statically against musl, which runs on any distribution of its
# architecture, built as crane's musl example builds one, with the
# toolchain rust-toolchain.toml names and the musl target added; and
# `dist', what the release attaches for that architecture: the archive and
# the `.deb', `.rpm' and Arch Linux packages, each with the binary and the
# licences' texts, made by nfpm from one list of files. Only on Linux; the
# release workflow builds macOS's and Windows' archives on their runners.
{ inputs, ... }:
{
  perSystem =
    {
      config,
      craneLib,
      lib,
      pkgs,
      system,
      workspace,
      ...
    }:
    let
      targets = {
        x86_64-linux = "x86_64-unknown-linux-musl";
        aarch64-linux = "aarch64-unknown-linux-musl";
      };
    in
    {
      # The notices of every crate the command contains, on every target a
      # release builds (cli/about.toml), which each of its packages carries.
      packages = {
        notices = craneLib.mkCargoDerivation (
          workspace.argsOf [
            "core"
            "cli"
          ]
          // {
            pname = "deduxit-notices";
            inherit (craneLib.crateNameFromCargoToml { cargoToml = ../Cargo.toml; }) version;
            cargoArtifacts = null;
            nativeBuildInputs = [ pkgs.cargo-about ];
            buildPhaseCargoCommand = ''
              cargo about generate --frozen --manifest-path cli/Cargo.toml \
                --config cli/about.toml cli/about.hbs --output-file THIRD-PARTY-NOTICES.txt
            '';
            installPhaseCommand = "cp THIRD-PARTY-NOTICES.txt $out";
            doInstallCargoArtifacts = false;
          }
        );
      }
      // lib.optionalAttrs (targets ? ${system}) (
        let
          target = targets.${system};
          craneLib = (inputs.crane.mkLib inputs.nixpkgs.legacyPackages.${system}).overrideToolchain (
            p:
            ((inputs.rust-overlay.lib.mkRustBin { } p).fromRustupToolchainFile ../rust-toolchain.toml).override
              { targets = [ target ]; }
          );
          inherit (craneLib.crateNameFromCargoToml { cargoToml = ../Cargo.toml; }) version;
          staticCc = "${pkgs.pkgsStatic.stdenv.cc}/bin/${pkgs.pkgsStatic.stdenv.cc.targetPrefix}cc";
        in
        {
          deduxit-static = craneLib.buildPackage (
            workspace.argsOf [
              "core"
              "cli"
            ]
            // {
              pname = "deduxit-static";
              inherit version;
              cargoExtraArgs = "--locked --package deduxit-cli";
              CARGO_BUILD_TARGET = target;
              CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
              # mimalloc, the static build's allocator, is C: compiled and
              # linked by a C compiler for musl.
              nativeBuildInputs = [ pkgs.pkgsStatic.stdenv.cc ];
              "CC_${builtins.replaceStrings [ "-" ] [ "_" ] target}" = staticCc;
              "CARGO_TARGET_${lib.toUpper (builtins.replaceStrings [ "-" ] [ "_" ] target)}_LINKER" = staticCc;
              # The `test-cli' check runs the tests, on the default build.
              doCheck = false;
              postInstall = ''
                install -Dm644 LICENSE $out/share/doc/deduxit-cli/LICENSE
                install -Dm644 -t $out/share/doc/deduxit-cli cli/fonts/*.txt
                install -Dm644 ${config.packages.notices} $out/share/doc/deduxit-cli/THIRD-PARTY-NOTICES.txt
              '';
              meta.mainProgram = "deduxit";
            }
          );

          dist =
            let
              static = config.packages.deduxit-static;
              doc = "${static}/share/doc/deduxit-cli";
              # The binary, and the licences where each format keeps them:
              # Arch Linux under /usr/share/licenses, the others beside the
              # package's documentation.
              licences = [
                "LICENSE"
                "THIRD-PARTY-NOTICES.txt"
                "OFL.txt"
                "Asana-Math-OFL.txt"
                "Garamond-Math-OFL.txt"
                "STIXTwoMath-Regular-OFL.txt"
                "NewCMMath-Book-LICENSE.txt"
              ];
              nfpm = builtins.toJSON {
                name = "deduxit";
                # nfpm's names, which it turns into each format's own.
                arch =
                  {
                    x86_64-linux = "amd64";
                    aarch64-linux = "arm64";
                  }
                  .${system};
                platform = "linux";
                inherit version;
                release = "1";
                section = "science";
                priority = "optional";
                maintainer = "Fabian Lukas Grubmüller <flgrubm@grubmueller.dev>";
                description = "Decide, check, prove step by step and export sequents of linear logic";
                vendor = "deduxit";
                homepage = "https://github.com/deduxit/deduxit";
                license = "EUPL-1.2 AND OFL-1.1 AND LPPL-1.3c";
                contents = [
                  {
                    src = "${static}/bin/deduxit";
                    dst = "/usr/bin/deduxit";
                    file_info.mode = 493;
                  }
                ]
                ++ map (file: {
                  src = "${doc}/${file}";
                  dst = "/usr/share/doc/deduxit/${file}";
                  packager = "deb";
                  file_info.mode = 420;
                }) licences
                ++ map (file: {
                  src = "${doc}/${file}";
                  dst = "/usr/share/doc/deduxit/${file}";
                  packager = "rpm";
                  type = "doc";
                  file_info.mode = 420;
                }) licences
                ++ map (file: {
                  src = "${doc}/${file}";
                  dst = "/usr/share/licenses/deduxit/${file}";
                  packager = "archlinux";
                  file_info.mode = 420;
                }) licences;
              };
              stem = "deduxit-${version}-${target}";
            in
            pkgs.runCommand "deduxit-dist-${version}"
              {
                nativeBuildInputs = [ pkgs.nfpm ];
                passAsFile = [ "nfpm" ];
                inherit nfpm;
              }
              ''
                mkdir -p $out ${stem}
                cp ${static}/bin/deduxit ${stem}/
                cp ${../README.md} ${stem}/README.md
                for file in ${toString licences}; do cp ${doc}/$file ${stem}/; done
                chmod -R u+w,go-w ${stem}
                tar --sort=name --mtime=@$SOURCE_DATE_EPOCH --owner=0 --group=0 --numeric-owner \
                  -cf - ${stem} | gzip -9n >$out/${stem}.tar.gz
                for packager in deb rpm archlinux; do
                  nfpm package --config $nfpmPath --packager $packager --target $out/
                done
              '';
        }
      );
    };
}
