# Step 87d4: certificates with shared subterms, bounded (report, paused)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-fixup-30-3e23c4`, from `plan/87-refactor.md` (87d4) and
the planning session's briefs `start-87-common.md` and `start-87d4.md`,
on cores 13 to 15, on 87c3's head `loqwlmwu`. **Paused at 20:28** at the
planning session's "Pause" (the author needs the machine at 20:35):
the code is committed, the net and the panel are not done.

## What landed (committed)

- `tyryyoly` "Hold a classical countermodel's Rocq certificate to
  derivation_bytes": `ordinary/rocq.rs`, `falsified` takes a limit and
  `falsified_bytes` estimates the text bottom up, saturating; the
  countermodel's certificate passes `limits.derivation_bytes`,
  `write_not_valid` none (it takes no limits).
  `a_classical_countermodel_keeps_to_the_bound` (forty levels of `p ∧ p`
  under 1 MiB) passes, and without the bound it is killed at a 2 GB cap.
- `rylkosmn` "Bind the formulas a kernel certificate names twice with
  let, inside the term": `export/rocq/shared.rs` (new) and
  `deduxit.rs`. A first walk hash-conses the formulas the term names,
  each compound piece written at two or more places is bound `let f'N :=
  … in` inside the term, and the statement is written out. Nine
  snapshots under `core/tests/snapshots/deduxit/` change (`ll.v`,
  `labels.v`, `labels_ill.v`, six first-order ones). The manual
  (`docs/src/rocq.md`, a new console example), the guide
  (`skills/deduxit/reference.md`), the changelog (Changed, Fixed) and
  `core-export.md` (two bullets) are in the same commit.

## Measured (before the pause)

`rocq compile` of the certificate of `!A ⊢ A^n` (left-nested), core 13,
five runs each, alternating, under the machine lock, against the
compiled library of 87r4 (`scratchpad/speed/libA`), medians:

| n | old (`p`) | new (`n`, deduxit at `rylkosmn`) | 87r4's `let` form (`l`) | sizes old / new |
|---:|---:|---:|---:|---|
| 64 | 0.751 s | 0.511 s | 0.516 s | 38 KB / 7.2 KB |
| 128 | 1.667 s | 0.678 s | 0.676 s | 146 KB / 14 KB |
| 256 | 5.008 s | 1.325 s | 1.400 s | 570 KB / 29 KB |

So 3.8 times faster at 256, as 87r4 measured by hand. Inputs and script:
`scratchpad/speed/n{64,128,256}.v`, `time2.py`.

Checked: `cargo clippy -p deduxit --all-features --all-targets`, the
export snapshots (49), the library's Rocq and export unit tests,
`cargo test -p deduxit-cli --test readme` (the manual's new example).

## Checklist for the resume

1. **The panel** (stopped at the pause; brief
   `side-session-notes/…/87d4-panel-brief.md`): start the three members
   again with the same brief (`panel-argument` fable, `panel-counterexamples`
   opus, `panel-integers` sonnet, each `high`). None had reported at the
   pause: the counterexamples member was building the head, the integers
   member had no probe running. The counterexamples member's jj
   workspaces `cx87d4-base` and `cx87d4-head` (in the session's
   scratchpad, at `loqwlmwu` and the head) are kept for it; `jj workspace
   forget` them once the panel is done. Fix and commit what they find.
2. **The nix checks**, never run (they waited on the lock): `jj st`,
   then `flock -o /tmp/linlog-machine.lock nix build --max-jobs 2 --cores 3
   --no-link .#checks.x86_64-linux.export .#checks.x86_64-linux.rocq
   .#checks.x86_64-linux.rocq-deduxit .#checks.x86_64-linux.check-certificate`.
   `rocq-deduxit` is the one that compiles the new certificates against
   the pinned library.
3. **The gate** (capped, cores 13 to 15) and `breaks 0.8.0`.
4. **fresh-reviewer** once (fable) on `loqwlmwu..` the head.
5. This report finished (the net, the panel, the reviewer), `nix flake
   check` at its head under the lock, then "87d4: done, head …" to
   deduxit-orchestrator.

Open for the planning session: `write_not_valid` (the image's
refutation's certificate) shares `falsified` but takes no limits, so it
is still unbounded; a bound there needs a limits argument (an API change,
part 2) or the default limits.
