#!/usr/bin/env bash
# linlog © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# The growing sets of linlog's engines (`linlog-bench families` lists each
# family's sizes and claim): every family under the default and every
# engine forced, and the passes that switch off the mechanism a claim is
# about, so that a figure draws the claim beside its comparison:
#
#   engines.csv     every growing set, the default and each engine forced
#                   (the comparison of the engines' strengths: wide-m1,
#                   counter, mix, contraction, pigeonhole, euler, duplicate)
#   memo-off.csv    chain and reach-fo with the memo off (--memo-limit 0),
#                   beside the default's line of engines.csv
#   deepening.csv   chain under the default deepening (--copies none),
#                   beside the family's own copy bound
#   pool.csv        witness-fo on four threads, beside one thread
#
# Each run has the time limit; a configuration whose run on a size is
# refused or meets the limit or the memory skips that shape's larger sizes.
#
#   bench/growing.sh [--out DIR] [--cores LIST] [--pool-cores LIST]
#                    [--timeout S] [--bin PATH]
#
# --out DIR          where the files go (default: bench/growing/DAY)
# --cores LIST       one sequential stream per core, comma-separated
#                    (default 2,3); the streams split engines.csv by shape,
#                    and the first also runs memo-off.csv and deepening.csv
# --pool-cores LIST  the cores of pool.csv, run alone after the streams
#                    (default 2-5)
# --timeout S        the time limit of every run (default 60)
# --bin PATH         the linlog-bench binary (default: target/release's)
#
# A file is resumed when the script runs again on the same DIR. The other
# provers run on the growing sets through `bench/comparison.sh --growing`.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
out=$root/bench/growing/$(date +%F)
cores=2,3
pool_cores=2-5
timeout=60
bin=$root/target/release/linlog-bench
while [ $# -gt 0 ]; do
  case $1 in
  --out)
    out=$2
    shift
    ;;
  --cores)
    cores=$2
    shift
    ;;
  --pool-cores)
    pool_cores=$2
    shift
    ;;
  --timeout)
    timeout=$2
    shift
    ;;
  --bin)
    bin=$2
    shift
    ;;
  *)
    echo "unknown option $1" >&2
    exit 2
    ;;
  esac
  shift
done
mkdir -p "$out"

# One pass into a file, resumed if it has rows.
pass() {
  local cpus=$1 file=$2
  shift 2
  local resume=()
  [ -s "$out/$file" ] && resume=(--append --resume)
  taskset -c "$cpus" "$bin" run --growing --timeout "$timeout" \
    --output "$out/$file" "${resume[@]}" "$@" 2>>"$out/${file%.csv}.log"
}

IFS=, read -ra streams <<<"$cores"
pids=()
for i in "${!streams[@]}"; do
  (
    pass "${streams[$i]}" "engines-$((i + 1)).csv" \
      --engines auto,focus,two-sided,net,horn,additive,essential,inverse \
      --shard "$((i + 1))/${#streams[@]}"
    if [ "$i" = 0 ]; then
      pass "${streams[$i]}" memo-off.csv --only chain/,reach-fo/ --memo-limit 0
      pass "${streams[$i]}" deepening.csv --only chain/ --copies none
    fi
  ) &
  pids+=($!)
done
status=0
for pid in "${pids[@]}"; do
  wait "$pid" || status=1
done
pass "$pool_cores" pool.csv --only witness-fo/ --jobs 4
exit "$status"
