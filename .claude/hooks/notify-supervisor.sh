#!/usr/bin/env bash
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# Notification hook: appends the payload, one line of JSON after the time,
# to a log the planning session reads, so that a step session waiting on a
# permission prompt or for input is seen without reading its terminal. The
# payload's `cwd` says which workspace, its `notification_type` and
# `message` what it waits on. Fails open: no error stops the session.
dir="${XDG_RUNTIME_DIR:-/tmp}/deduxit-sessions"
mkdir -p "$dir" 2>/dev/null || exit 0
payload=$(cat 2>/dev/null) || exit 0
printf '%s\t%s\n' "$(date -Iseconds)" "$(printf '%s' "$payload" | tr -d '\n')" \
  >>"$dir/notifications.log" 2>/dev/null
exit 0
