#!/usr/bin/env bash
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# start-session.sh NAME MODEL EFFORT PROMPT_FILE...
# start-session.sh --resume SESSION_ID NAME MODEL EFFORT [MESSAGE]
#
# Starts a background Claude Code session (`claude --bg`) named NAME in the
# repository, in auto mode, with the files concatenated as its prompt (a
# step's `/goal …` text). Or it resumes the session SESSION_ID in the
# background, after a pause (`claude stop`) or a reboot, with MESSAGE
# ("Resume" by default) as its next prompt; a goal the session had when it
# stopped comes back with it. Prints the background job's id and the
# session's id. The planning session runs this to start a step's sessions
# itself.
#
# `claude agents --json` lists the jobs, `claude logs NAME` prints a job's
# recent terminal output, `claude stop JOB` stops one (its conversation is
# kept), `claude attach JOB` opens it in a terminal. SendMessage reaches a
# job by NAME, and an idle one takes the message as a new turn.
set -euo pipefail

resume=()
message=()
if [[ ${1-} == --resume ]]; then
  resume=(--resume "$2")
  shift 2
  if (($# < 3 || $# > 4)); then
    echo "usage: $0 --resume SESSION_ID NAME MODEL EFFORT [MESSAGE]" >&2
    exit 2
  fi
  message=("${4:-Resume}")
elif (($# < 4)); then
  echo "usage: $0 NAME MODEL EFFORT PROMPT_FILE..." >&2
  echo "       $0 --resume SESSION_ID NAME MODEL EFFORT [MESSAGE]" >&2
  exit 2
fi
name=$1 model=$2 effort=$3
shift 3

# A session started from another one inherits markers (CLAUDECODE,
# CLAUDE_CODE_CHILD_SESSION, its messaging socket, …) that make the new
# one a child: no transcript, no entry in the session registry, so no
# --resume and no messages by name. Clear them, so that the new session
# is what the author would start from a terminal. The account
# (CLAUDE_CONFIG_DIR, ~/.claude when unset) is kept, so that the new
# session runs on the same account as the one that starts it.
config_dir=${CLAUDE_CONFIG_DIR-}
while IFS= read -r var; do
  case $var in CLAUDE* | AI_AGENT) unset "$var" ;; esac
done < <(compgen -e)
if [[ -n $config_dir ]]; then
  export CLAUDE_CONFIG_DIR=$config_dir
fi

prompt=("${message[@]}")
if ((! ${#resume[@]})); then
  text=""
  for file in "$@"; do
    text+="$(cat "$file")"$'\n'
  done
  prompt=("$text")
fi

cd "$(dirname "$0")/.."
out=$(claude --bg "${resume[@]}" --name "$name" --model "$model" \
  --effort "$effort" --permission-mode auto "${prompt[@]}" 2>&1) || {
  printf '%s\n' "$out" >&2
  exit 1
}
job=$(grep -oE 'claude attach [0-9a-f]+' <<<"$out" | head -n 1 | cut -d ' ' -f 3)
if [[ -z $job ]]; then
  printf 'session %s: no job id in claude --bg'"'"'s output:\n%s\n' "$name" "$out" >&2
  exit 1
fi

# The job lists its session's id once the session has registered.
for _ in $(seq 60); do
  id=$(claude agents --json 2>/dev/null | python3 -I -c '
import json, sys
data = json.load(sys.stdin)
rows = data if isinstance(data, list) else data.get("agents", [])
for row in rows:
    if row.get("id") == sys.argv[1] and row.get("sessionId"):
        print(row["sessionId"])
        break
' "$job" || true)
  if [[ -n $id ]]; then
    echo "session $name: job $job, session $id"
    exit 0
  fi
  sleep 1
done
echo "session $name: job $job, not registered after a minute" >&2
exit 1
