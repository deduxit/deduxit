#!/usr/bin/env bash
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The inventories of step 86's audit (plan/notes/audit/method.md, stage 0),
# each into target/audit/ of the workspace it is run in:
#
#   plan/audit/stage0.sh all             # every part, in order
#   plan/audit/stage0.sh lines text     # the parts named
#
# Parts: lines, api, clippy, duplicates, text, help, sizes, llvm, dhat,
# rocq (the Rocq library's inventory), wasm (the web module's bytes by
# the library's modules), and `timings' (compile and test times), which
# `all' leaves out.
# `lines', `text' and `help' (once the command is built) take seconds; the
# others build: every build and run is in a scope of its own, capped at
# 8 GiB without swap, on the cores of `cores', with three jobs, and the
# two tools nixpkgs has (PMD, cargo-llvm-lines) are fetched under the
# machine lock. ROCQ is the Rocq library's checkout, read only.
set -euo pipefail

cores=${AUDIT_CORES:-13-15}
rocq=${ROCQ:-../rocq-deduxit}
root=$(cd "$(dirname "$0")/../.." && pwd)
out=$root/target/audit
tools=$root/plan/audit
lock=/tmp/linlog-machine.lock
cd "$root"
mkdir -p "$out"

# Runs a command in a capped scope on the audit's cores; the arguments up
# to `--' are settings of the environment, NAME=VALUE.
capped() {
  local env=(--setenv=CARGO_BUILD_JOBS=3)
  while [ "$1" != -- ]; do
    env+=("--setenv=$1")
    shift
  done
  shift
  systemd-run --user --scope --quiet -p MemoryMax=8G -p MemorySwapMax=0 \
    "${env[@]}" taskset -c "$cores" "$@"
}

# Fetches a package of the flake's nixpkgs and links it at target/NAME.
tool() {
  [ -e "target/$1" ] ||
    flock -o "$lock" nix build --inputs-from "$root" "nixpkgs#$1" \
      --max-jobs 1 --cores 3 --out-link "target/$1"
}

lines() {
  python3 "$tools/lines.py" "$root" "$rocq" >"$out/lines.md"
}

# The public API: rustdoc's JSON with every feature, listed by a diff
# against an empty crate of the same name, then read by surface.py.
api() {
  capped RUSTC_BOOTSTRAP=1 "CARGO_TARGET_DIR=$root/target/api" -- \
    cargo rustdoc --quiet --locked --manifest-path core/Cargo.toml --all-features --lib \
    -- -Z unstable-options --output-format json
  mkdir -p "$out/empty/src"
  printf '[package]\nname = "deduxit"\nversion = "0.0.0"\nedition = "2024"\n\n[workspace]\n' \
    >"$out/empty/Cargo.toml"
  printf '//! Empty.\n' >"$out/empty/src/lib.rs"
  capped RUSTC_BOOTSTRAP=1 "CARGO_TARGET_DIR=$root/target/api-empty" -- \
    cargo rustdoc --quiet --manifest-path "$out/empty/Cargo.toml" --lib \
    -- -Z unstable-options --output-format json
  capped -- cargo public-api -sss diff target/api-empty/doc/deduxit.json \
    target/api/doc/deduxit.json >"$out/public-api.txt"
  python3 "$tools/surface.py" target/api/doc/deduxit.json "$root" >"$out/surface.md"
}

# Clippy with every lint that Cargo.toml allows turned to a warning.
clippy() {
  local flags
  mapfile -t flags < <(
    python3 - "$root/Cargo.toml" <<'EOF'
import sys
import tomllib

with open(sys.argv[1], "rb") as f:
    lints = tomllib.load(f)["workspace"]["lints"]
for group, prefix in (("rust", ""), ("clippy", "clippy::")):
    for name, level in lints[group].items():
        if level == "allow":
            print("-W")
            print(prefix + name)
EOF
  )
  capped -- cargo clippy --workspace --all-targets --locked --message-format=json \
    -- "${flags[@]}" >"$out/clippy.json" 2>"$out/clippy.err"
  python3 "$tools/clippy.py" "$out/clippy.json" >"$out/clippy.md"
}

duplicates() {
  tool pmd
  rm -rf "$out/neutral"
  capped -- python3 "$tools/duplicates.py" target/pmd/bin/pmd "$root" "$out/neutral" 100 \
    >"$out/duplicates.md"
}

text() {
  python3 "$tools/text.py" "$root" "$rocq" >"$out/text.md"
}

help() {
  capped -- cargo build --quiet --locked --package deduxit-cli
  python3 "$tools/help.py" target/debug/deduxit "$out/help" >"$out/help.md"
}

# The layout of every type, as the release build has it.
sizes() {
  capped RUSTC_BOOTSTRAP=1 RUSTFLAGS=-Zprint-type-sizes "CARGO_TARGET_DIR=$root/target/sizes" -- \
    cargo build --quiet --release --locked --package deduxit --lib >"$out/type-sizes.txt"
}

# The functions with the most LLVM IR: what generics cost to compile.
llvm() {
  tool cargo-llvm-lines
  capped "CARGO_TARGET_DIR=$root/target/llvm-lines-build" -- \
    target/cargo-llvm-lines/bin/cargo-llvm-lines llvm-lines --package deduxit --lib --release |
    head -200 >"$out/llvm-lines.txt"
}

# Each journey's allocations under DHAT, from a release build with line
# tables in a target directory of its own.
dhat() {
  mkdir -p "$out/dhat"
  capped CARGO_PROFILE_RELEASE_DEBUG=line-tables-only "CARGO_TARGET_DIR=$root/target/dhat" -- \
    cargo build --quiet --release --locked --package deduxit-bench
  local journey
  while IFS=, read -r journey _; do
    [ "$journey" = journey ] && continue
    capped -- valgrind --tool=dhat "--dhat-out-file=$out/dhat/$journey.json" \
      target/dhat/release/deduxit-bench journey "$journey" 2>"$out/dhat/$journey.txt" >/dev/null </dev/null
  done <bench/ceilings.csv
  python3 "$tools/dhat.py" "$out/dhat" >"$out/dhat.md"
}

# The Rocq library's inventory, built in a copy under target/audit/rocq
# with the Rocq of the library's own devshell.
rocq_library() {
  flock -o "$lock" nix develop --no-write-lock-file "$rocq" --command true
  capped -- nix develop --no-write-lock-file "$rocq" --command \
    python3 "$tools/rocq.py" "$rocq" "$root" "$out/rocq" >"$out/rocq.md"
}

# The web module's bytes by the library's modules: the bindings as cargo
# builds them for wasm32, names kept, listed by twiggy.
wasm() {
  tool twiggy
  capped "CARGO_TARGET_DIR=$root/target/wasm-audit" -- \
    cargo build --quiet --locked --release --target wasm32-unknown-unknown --package deduxit-web
  capped -- target/twiggy/bin/twiggy top -n 1000000 --format json \
    target/wasm-audit/wasm32-unknown-unknown/release/deduxit_web.wasm >"$out/twiggy-top.json"
  python3 "$tools/wasm.py" "$out/twiggy-top.json" >"$out/wasm.md"
}

# Compile and test times from an empty target directory. A timed run: not
# a part of `all', and only with the cores to itself.
timings() {
  capped -- python3 "$tools/timings.py" "$root" "$root/target/timing" >"$out/timings.md"
}

for part in "$@"; do
  case $part in
  timings) timings ;;
  wasm) wasm ;;
  rocq) rocq_library ;;
  all)
    lines
    api
    clippy
    duplicates
    text
    help
    sizes
    llvm
    dhat
    rocq_library
    wasm
    ;;
  lines | api | clippy | duplicates | text | help | sizes | llvm | dhat) "$part" ;;
  *)
    echo "stage0.sh: no part named $part" >&2
    exit 2
    ;;
  esac
done
