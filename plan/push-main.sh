#!/usr/bin/env bash
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# push-main.sh [--client|--rocq|--placeholder] REV
#
# Moves `main` to REV and pushes it, failing within a minute instead of
# waiting on a passphrase prompt when the SSH key is locked, and checks
# that `main@origin` is REV afterwards (a push once failed silently).
# With --client, in the web client's repository beside this one
# (../linlog-prover.github.io), which the planning session pushes the
# same way; with --rocq, in the Rocq library's repository beside it
# (../rocq-linlog), whose first push creates `main` on the remote; with
# --placeholder, in the old organisation's redirect site
# (../linlog-prover-placeholder). Exit status 0 pushed, 1 the key is
# locked or the push failed.
set -euo pipefail

repo="$(dirname "$0")/.."
if [[ ${1-} == --client ]]; then
  repo="$repo/../linlog-prover.github.io"
  shift
elif [[ ${1-} == --rocq ]]; then
  repo="$repo/../rocq-linlog"
  shift
elif [[ ${1-} == --placeholder ]]; then
  repo="$repo/../linlog-prover-placeholder"
  shift
fi
if (($# != 1)); then
  echo "usage: $0 [--client|--rocq|--placeholder] REV" >&2
  exit 2
fi
cd "$repo"
# By change id: with `git.sign-on-push` the push first signs the commits
# still unsigned, which gives them new commit ids.
want=$(jj log --no-graph -r "$1" -T 'change_id ++ "\n"')
if [[ -z $want || $want == *$'\n'* ]]; then
  echo "push-main: $1 is not one revision" >&2
  exit 2
fi
# GitHub's `ssh -T` exits 1 even when it authenticates, so the output decides.
if [[ $(timeout 20 ssh -o BatchMode=yes -T git@github.com 2>&1) != *successfully* ]]; then
  echo "push-main: the SSH key is locked; nothing pushed" >&2
  exit 1
fi
if [[ -n $(jj log --no-graph -r 'present(main@origin)' -T 'commit_id') ]]; then
  jj bookmark set main -r "change_id($want)"
  GIT_SSH_COMMAND='ssh -o BatchMode=yes' timeout 60 jj git push --bookmark main
else
  # The remote has no `main` yet: create it there and track it.
  GIT_SSH_COMMAND='ssh -o BatchMode=yes' timeout 60 jj git push --named "main=change_id($want)"
fi
got=$(jj log --no-graph -r main@origin -T 'change_id')
if [[ $got != "$want" ]]; then
  echo "push-main: main@origin is change ${got:0:12}, not ${want:0:12}" >&2
  exit 1
fi
echo "push-main: main@origin is $(jj log --no-graph -r main@origin -T 'commit_id.short(12) ++ " " ++ if(signature, "signed", "UNSIGNED")') (change ${want:0:12})"
