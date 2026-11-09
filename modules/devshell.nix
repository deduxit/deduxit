# linlog © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The development shell; `menu' lists its commands. Their names avoid shell
# builtins, which win over PATH (`test' in bash and zsh, `run' and `watch' in
# nushell).
{ inputs, ... }:
{
  imports = [ inputs.devshell.flakeModule ];

  perSystem =
    {
      lib,
      pkgs,
      rustToolchain,
      valgrindRuns,
      ...
    }:
    {
      devshells.default = {
        devshell.packages = [
          rustToolchain
          # rustc links through `cc'; a devshell, unlike mkShell, has none.
          pkgs.stdenv.cc
          pkgs.cargo-hack
          pkgs.cargo-deny
          # Mutation testing (mutants/run.sh), whose tests nextest runs.
          pkgs.cargo-mutants
          pkgs.cargo-nextest
          # The break since a release (`breaks'): rustdoc's JSON of both
          # versions, read by two tools that see different things.
          pkgs.cargo-semver-checks
          pkgs.cargo-public-api
          pkgs.bacon
          pkgs.zellij
        ]
        # Instruction counts: the ratchet's journeys and their profiles,
        # where valgrind runs (not on aarch64-darwin; `ratchet.nix').
        ++ lib.optional valgrindRuns pkgs.valgrind.out;

        commands = [
          {
            name = "check";
            help = "Run every flake check: build, clippy, tests (release and debug assertions), rustdoc (public and private items), the manual and its links, wasm, cargo-deny, cargo-hack, export, rocq, rocq-linlog, rocq-linlog-library, bench, ratchet, deadnix, actionlint, formatting, claude-hooks, conventions, typos, shear";
            command = "nix flake check";
          }
          {
            name = "gate";
            help = "What every commit passes: clippy, the tests (the behaviour lock among them), cargo-hack's feature checks and the ratchet";
            command = ''
              set -eu
              cargo clippy --workspace --all-targets --locked -- --deny warnings
              cargo test --workspace --locked
              # Warnings denied: an import or item left unused in one feature
              # set fails here. Their own target directory, since RUSTFLAGS
              # are part of every artifact and would rebuild the tests'.
              RUSTFLAGS="-D warnings" CARGO_TARGET_DIR=target/hack \
                cargo hack check --each-feature --locked --package deduxit
              RUSTFLAGS="-D warnings" CARGO_TARGET_DIR=target/hack \
                cargo hack check --feature-powerset --depth 2 --locked --package deduxit
              cargo run --release --locked --package deduxit-bench -- ratchet --check --jobs 4
              echo "gate: passed"
            '';
          }
          {
            name = "tests";
            help = "Run the workspace's tests (`cargo test --workspace`)";
            command = ''cargo test --workspace "$@"'';
          }
          {
            name = "ratchet";
            help = "Count the journeys' instructions under callgrind against bench/ceilings.csv, with the cache model's misses against bench/misses.csv (`ratchet --lower` writes the counts that went down, `--record-misses` the misses)";
            command = ''cargo run --release --locked --package deduxit-bench -- ratchet --jobs 4 "$@"'';
          }
          {
            name = "breaks";
            help = "List what breaks a user of the library since a release (`breaks 0.1.0`): cargo-semver-checks against crates.io's version, then cargo-public-api's diff of the two public APIs with every feature";
            command = ''
              set -eu
              base=''${1:?usage: breaks VERSION, a version of linlog on crates.io}
              out=$(mktemp -d)
              trap 'rm -rf "$out"' EXIT
              # Exit status 100 when a check fails, which is what is asked.
              cargo semver-checks --package deduxit --baseline-version "$base" \
                --release-type patch --color never || [ $? = 100 ]
              # rustdoc's JSON is unstable; cargo-semver-checks unlocks it on
              # the stable compiler the same way. Its own JSON documents
              # private items, so the public API is built afresh, the
              # published crate's from its package.
              curl --silent --fail --location \
                --user-agent "linlog breaks (github.com/deduxit/deduxit)" \
                "https://static.crates.io/crates/linlog/linlog-$base.crate" | tar -xz -C "$out"
              for side in base head; do
                if [ "$side" = base ]; then
                  manifest="$out/linlog-$base/Cargo.toml"
                else
                  manifest="$PRJ_ROOT/core/Cargo.toml"
                fi
                RUSTC_BOOTSTRAP=1 CARGO_TARGET_DIR="$out/$side" cargo rustdoc --quiet \
                  --manifest-path "$manifest" --all-features --lib \
                  -- -Z unstable-options --output-format json
              done
              cargo public-api diff "$out/base/doc/linlog.json" "$out/head/doc/linlog.json"
            '';
          }
          {
            name = "launch";
            help = "Build and run the CLI; arguments are passed on";
            command = ''cargo run --package deduxit-cli -- "$@"'';
          }
          {
            name = "live";
            help = "Re-check on every save with bacon (`live clippy`, `live test` for other jobs)";
            command = ''bacon "$@"'';
          }
          {
            name = "dev";
            help = "Enter the zellij development layout";
            command = ''zellij --layout "$PRJ_ROOT/zellij.kdl"'';
          }
          {
            name = "up";
            help = "Update the flake inputs and commit flake.lock with jj (unverified; /update-deps verifies first)";
            # Not `nix flake update --commit-lock-file': that is a git commit.
            command = ''
              set -eu

              # Keep the bump out of whatever work @ already holds.
              isolated=0
              if [ -n "$(jj diff -r @ --name-only)" ] || \
                 [ -n "$(jj log --no-graph -r @ -T description)" ]; then
                jj new
                isolated=1
              fi

              err=$(mktemp)
              trap 'rm -f "$err"' EXIT

              if ! nix flake update 2>"$err"; then
                cat "$err" >&2
                if [ "$isolated" = 1 ]; then jj abandon; fi
                exit 1
              fi
              cat "$err" >&2

              if [ -z "$(jj diff -r @ --name-only)" ]; then
                echo "flake.lock is already up to date; nothing to commit."
                if [ "$isolated" = 1 ]; then jj abandon; fi
                exit 0
              fi

              # nix's bullet list on stderr is the body --commit-lock-file writes.
              updates=$(awk '/^•/{p=1} p' "$err")
              if [ -n "$updates" ]; then
                msg="flake.lock: Update

              Flake lock file updates:

              $updates"
              else
                msg="flake.lock: Update"
              fi

              jj commit -m "$msg"
            '';
          }
        ];
      };
    };
}
