#!/usr/bin/env bash
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# push-ci.sh NAME REV
#
# Pushes REV as the branch `ci/NAME`, which CI checks as it checks `main`
# (every flake check, in groups on GitHub's runners, the results uploaded
# to the cache), waits for that run and exits with its outcome: 0 every
# job passed, 1 a job failed, the key is locked or the push failed. The
# branch moves to REV even backwards, so a name is reused for a chain's
# next check. When it passed, `plan/push-main.sh REV` moves `main` there,
# and `main`'s own run fetches every result. Long: run it in the
# background.
set -euo pipefail

if (($# != 2)); then
  echo "usage: $0 NAME REV" >&2
  exit 2
fi
name=ci/$1
cd "$(dirname "$0")/.."
want=$(jj log --no-graph -r "$2" -T 'change_id ++ "\n"')
if [[ -z $want || $want == *$'\n'* ]]; then
  echo "push-ci: $2 is not one revision" >&2
  exit 2
fi
# GitHub's `ssh -T` exits 1 even when it authenticates, so the output decides.
if [[ $(timeout 20 ssh -o BatchMode=yes -T git@github.com 2>&1) != *successfully* ]]; then
  echo "push-ci: the SSH key is locked; nothing pushed" >&2
  exit 1
fi
jj bookmark set "$name" --allow-backwards -r "change_id($want)"
GIT_SSH_COMMAND='ssh -o BatchMode=yes' timeout 60 jj git push --bookmark "$name"
# The push signs what is unsigned, so the commit is read back from the remote.
sha=$(jj log --no-graph -r "$name@origin" -T 'commit_id')
run=
for _ in $(seq 30); do
  run=$(gh run list --repo deduxit/deduxit --workflow ci.yml --branch "$name" \
    --commit "$sha" --json databaseId --jq '.[0].databaseId // empty')
  [[ -n $run ]] && break
  sleep 10
done
if [[ -z $run ]]; then
  echo "push-ci: no CI run of $name at ${sha:0:12} after five minutes" >&2
  exit 1
fi
echo "push-ci: $name is ${sha:0:12} (change ${want:0:12}), run https://github.com/deduxit/deduxit/actions/runs/$run"
# The watch's exit status alone is not the verdict: on a dropped connection
# `gh run watch` has exited 0 with the run still going (2026-11-20). So
# watch until the run is completed, then read its conclusion.
until [[ $(gh run view "$run" --repo deduxit/deduxit --json status --jq .status 2>/dev/null) == completed ]]; do
  gh run watch "$run" --repo deduxit/deduxit --compact --interval 60 >/dev/null 2>&1 || sleep 60
done
conclusion=$(gh run view "$run" --repo deduxit/deduxit --json conclusion --jq .conclusion)
echo "push-ci: run $run $conclusion"
[[ $conclusion == success ]]
