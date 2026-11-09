#!/usr/bin/env bash
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# keys-warm.sh [--check]
#
# Keeps the signing key and the SSH key unlocked in gpg-agent for as long
# as their caches allow, by using both every four minutes in the user unit
# `step-gpg-warm`: the SSH key's cache ends 30 minutes after its last use
# (default-cache-ttl-ssh), so signing alone lets it lapse. Neither use
# ever asks for a passphrase; when one fails, the unit ends, and the
# author enters the passphrases again. With --check, prints whether each
# key is unlocked now and exits non-zero if one is not.
set -euo pipefail

key=flgrubm@grubmueller.dev
sign() { echo x | gpg --batch --pinentry-mode error --local-user "$key" --sign -o /dev/null 2>/dev/null; }
# GitHub's `ssh -T` exits 1 even when it authenticates, so the output decides.
ssh_ok() { [[ $(timeout 20 ssh -o BatchMode=yes -T git@github.com 2>&1) == *successfully* ]]; }

if [[ ${1-} == --check ]]; then
  status=0
  if sign; then echo "signing: unlocked"; else
    echo "signing: locked"
    status=1
  fi
  if ssh_ok; then echo "ssh: unlocked"; else
    echo "ssh: locked"
    status=1
  fi
  exit "$status"
fi

if ! sign || ! ssh_ok; then
  # Unlocking the SSH key through gpg-agent unlocks the signing key with
  # it (the same key's passphrase); unlocking the signing key alone
  # leaves the SSH key locked.
  echo "a key is locked: unlock it through SSH, which unlocks both:" >&2
  echo "  ssh -T git@github.com" >&2
  exit 1
fi
systemctl --user stop step-gpg-warm 2>/dev/null || true
systemd-run --user --unit=step-gpg-warm --collect bash -c "
  while echo x | gpg --batch --pinentry-mode error --local-user $key --sign -o /dev/null \
    && timeout 20 ssh -o BatchMode=yes -T git@github.com 2>&1 | grep -q successfully; do
    sleep 240
  done"
