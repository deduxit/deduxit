# The baseline night: checklist

The planning session keeps this file current. The author says when the
machine is the benchmark's; nothing here starts before that. What the
script measures and why is in `bench/baseline.sh`'s header and
`.claude/rules/bench.md` ("`baseline.sh`"); this is the order of the
evening and the morning.

Written 2026-11-15 (baseline-prep), for the fifth baseline: the first
with ordinary logic (ILTP's `iltp` and `iltp-classical` in stage 1).
The fourth, `bench/results/2026-11-02/`, ran beside other work, confined
to CPUs 2-9; the third, `2026-10-17`, is the last taken on a whole idle
machine and is the one to compare times with.

## Before: the day

- [ ] `main` holds what is to be measured; the planning session's
      checkout is at it (`jj new main@origin`), `jj st` clean. The
      unit reads its commit from `@-` and lists the files `@` changes.
- [ ] The libraries are there (the script refuses without them):
      `ls bench/lltp/ILL bench/iltp/Problems`, else
      `nix build .#lltp -o bench/lltp` and `nix build .#iltp -o bench/iltp`.
- [ ] The devshell rooted, so that a collection cannot take the unit's
      `PATH`: `nix develop --profile target/devshell-profile -c true`.
- [ ] The release build warm, so the night does not start with a
      compile: `CARGO_TARGET_DIR=target/baseline cargo build --release
      --locked -p deduxit-bench` (in the devshell, capped as usual).

## Before: the hour it starts

- [ ] **Sessions paused.** Every step session and worker of the
      planning session gets "Pause" (the `graceful-stop` skill);
      `list_sessions {"linked": true}` shows none running. No `nix
      flake check`, no gate, no mutants or fuzz unit, no `targets.sh`:
      `systemctl --user list-units 'deduxit-*' --state=active` shows
      nothing but what is meant to run.
- [ ] **Timers.** `systemctl list-timers --all` and
      `systemctl --user list-timers --all`. On 2026-11-15 they were:
  - system, which the author stops (root): `nix-gc` (00:01),
    `nix-optimise` (about 03:50), `fstrim` (Mondays about 00:45),
    `borgbackup-job-eth` (daily about 12:10; stop it if the run may
    reach noon). `sudo systemctl stop nix-gc.timer nix-optimise.timer
    fstrim.timer borgbackup-job-eth.timer`, and `start` them again in
    the morning.
  - user: `obsidian-snapshot`. The unit stops the user timers due during
    the run and starts them again when it ends (`ExecStopPost`), so
    nothing to do by hand.
  - left running, as the script does: `logrotate`, `fwupd-refresh`,
    `systemd-tmpfiles-clean`.
- [ ] **Power and lid.** On mains, checked by hand: the unit runs the
      script with `--force`, so `--detach` starts on battery without a
      word and `--arm` waits for mains only until its latest start, then
      starts anyway and records `BATTERY`. The unit's inhibitor keeps the
      lid and the idle manager from suspending.
- [ ] Optionally, the system slices off the performance cores (root,
      until a reboot): `sudo systemctl set-property --runtime
      system.slice AllowedCPUs=4-15` and the same for `init.scope`;
      `AllowedCPUs=` in the morning gives them back.

## The start

- [ ] On a night the author gives the whole machine, the default cores
      (streams on CPUs 1 0 2 3, the parallel stage on every core), as the
      third baseline:

      bench/baseline.sh --detach --fresh

  `--detach` starts at once and checks neither the load nor the timers;
  with `--arm --fresh` instead it waits for the slot (20:00 to 07:00,
  `--slot=HH:MM-HH:MM`) and an idle machine, and is stopped at the
  slot's end. Where the desktop stays in use, `--cores=4,5,2,3
  --cpus=2-15` keeps CPUs 0 and 1 free; RESULTS.md then says so, and
  the times compare with the fourth baseline's, not the third's.
- [ ] `systemctl --user status deduxit-baseline` active; `journalctl
      --user -fu deduxit-baseline` shows `started into
      bench/results/DAY` and the cargo build.
- [ ] Expected length: about three hours (the third baseline 2 h 58 min;
      stage 1 about fifty minutes, the two ILTP passes a quarter of an
      hour each in its two shorter streams; stage 3 about an hour and a
      half).

## The morning

- [ ] `journalctl --user -u deduxit-baseline | tail` says `finished`,
      and `bench/results/DAY/RESULTS.md` exists, copied to
      `bench/RESULTS.md`. Without `RESULTS.md` it stopped early:
      `bench/baseline.sh --detach` (no `--fresh`) resumes it.
- [ ] `starts.txt`: one line per start, the commit expected, "forced"
      only if the slot made it start on a busy machine.
- [ ] RESULTS.md's header: the load at the start, the timers that fired
      (`none` expected), the throttling.
- [ ] RESULTS.md's tables (or `deduxit-bench summary
      bench/results/DAY/*.csv`): no `MISMATCH`, `error` or `crash`
      beyond the known ones (the 28 LLTP files whose headers contradict
      their problems, the crashes on large nets, `.claude/rules/bench.md`);
      none at all in the ILTP tables.
      The count of `†` rows (a CPU waited for) small.
- [ ] The ILTP tables against the runs that chose the defaults
      (`bench/defaults/sat-default.csv`, `sat-classical.csv`, at 2 s):
      decided of 274, the SAT solver and the default 252 intuitionistic
      and 253 minimal, LJT 200 in each, the embedding 188 intuitionistic
      and 192 minimal; classical 254 by the SAT
      solver and the default, the embedding 157. The table decides only
      the problems of at most ten atoms; the tableau is new.
- [ ] The timers stopped by hand started again; the system slices given
      back if they were moved.
- [ ] Commit the results in the planning session's checkout: one change
      "Benchmark baseline of DAY" with `bench/results/DAY/` and
      `bench/RESULTS.md`, then the push as usual (`plan/push-main.sh`).
- [ ] Sessions resumed ("Resume").
