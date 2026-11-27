#!/usr/bin/env bash
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# Pins the statistics of ordinary logic's two connection provers, the
# classical and the intuitionistic one, on the ILTP problems each decides
# within a second: every line of bench/problems/connection.txt (`LOGIC
# PATH') run with `--engines connection' on one thread into
# bench/targets/connection-LABEL.csv, so that
#
#   bench/connection-targets.sh LABEL
#   deduxit-bench summary --against bench/targets/connection-87a.csv \
#     bench/targets/connection-LABEL.csv --counters
#
# names every problem whose verdict or counters moved: `nodes' (the
# extension steps) and `work' (every step). On one thread they are a
# function of the input, so they compare whatever the machine's load; a
# change of the provers that must not change their search keeps them.
#
# Every problem was decided within a second where the limit is five, so
# the load does not change which are decided. It runs as the user unit
# `deduxit-connection-targets', one stream per core of `cores' (by
# default 2 and 3, the target sets' slot; the environment's CORES, a list
# such as "4 5 6", names others), 8 GiB and no swap for all of it and
# 6 GiB of address space per process. About a minute on two cores.
#
# The problems need ILTP: nix build .#iltp -o bench/iltp and
# nix build .#iltp-first-order -o bench/iltp-first-order
set -euo pipefail
self=$(realpath "$0")
cd "$(dirname "$self")/.."
label=${1:?usage: bench/connection-targets.sh LABEL}
list=bench/problems/connection.txt
out=bench/targets
read -r -a cores <<<"${CORES:-2 3}"

for library in bench/iltp bench/iltp-first-order; do
  if [ ! -d "$library/Problems" ]; then
    echo "no ILTP problems at $library: nix build .#${library#bench/} -o $library" >&2
    exit 1
  fi
done

if [ "${2:-}" != --inside ]; then
  # A copy of the binary, so that builds in the checkout meanwhile do not
  # touch the one the unit runs.
  cargo build --release --locked --package deduxit-bench
  install -D "${CARGO_TARGET_DIR:-target}/release/deduxit-bench" \
    "target/targets/connection-$label/deduxit-bench"
  mkdir -p "$out"
  systemctl --user reset-failed deduxit-connection-targets.service 2>/dev/null || true
  systemd-run --user --unit=deduxit-connection-targets --same-dir --collect \
    -p MemoryMax=8G -p MemorySwapMax=0 -p OOMPolicy=continue -p LimitCORE=0 \
    --setenv=PATH="$PATH" --setenv=CORES="${cores[*]}" "$self" "$label" --inside
  echo "into $out/connection-$label.csv; follow it with: journalctl --user -fu deduxit-connection-targets"
  exit
fi

bench=target/targets/connection-$label/deduxit-bench
streams=${#cores[@]}

# stream K: the K-th shard of each logic's problems, on the K-th core.
stream() {
  local k=$1 logic
  for logic in classical intuitionistic; do
    local files=()
    while read -r named path; do
      [ "$named" = "$logic" ] && files+=(--tptp "$path")
    done <"$list"
    prlimit --as=$((6 << 30)) taskset -c "${cores[k - 1]}" "$bench" run "${files[@]}" \
      --logics "$logic" --engines connection --jobs 1 --timeout 5 --shard "$k/$streams" \
      --output "$out/connection-$label.$k.csv" --append --resume 2>>"$out/connection-$label.log"
  done
}

pids=()
for k in $(seq 1 "$streams"); do
  stream "$k" &
  pids+=($!)
done
wait "${pids[@]}"

{
  head -n 1 "$out/connection-$label.1.csv"
  for k in $(seq 1 "$streams"); do
    tail -n +2 "$out/connection-$label.$k.csv"
  done
} >"$out/connection-$label.csv"
for k in $(seq 1 "$streams"); do
  rm "$out/connection-$label.$k.csv"
done
