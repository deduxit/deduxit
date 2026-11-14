# How long the steps take

Wall-clock times of the step sessions, from their Claude Code
transcripts, for the planning session's ETAs. Refresh with
`python3 -I plan/durations.py OUT.tsv` (one row per transcript, local
time); a step split across transcripts (a compaction, a pause) has one
row each, summed below. First written 2026-11-14 from 113 transcripts
(2026-10-16 to 2026-11-14).

## Rules of thumb (Opus 5.5 at `high`, since 2026-11-09)

| kind of session | examples | hours |
|---|---|---|
| text rewrite of one area, with writers and the reviewer | 54w-c, 54w-d, 54w-e2 to 54w-e6 | 0.65 to 1.0 |
| a small feature or a few items, with the reviewer | 54u-a, 54u-b, 54u-e, 54s-c1 | 0.45 to 1.1 |
| a client session (unit and e2e suites, the client's flake check) | 54u-c, 54u-d, 43b, 43c | 0.9 to 1.2 |
| soundness-critical, with the panel and the reviewer | 54b, 54s-a, 54s-b | 2.0 to 2.4 |
| a larger engine or a first session that sets a design | 54a, 37a, 39c, 40b | 4 to 6.5 |
| design on Fable, with the author's rounds | 32, 53m | 7 to 10 |
| a timed run or a baseline | 45e, 53n | the run's own length, plus about an hour |

Within a session:
- the first commit comes 5 to 40 minutes in;
- the fresh reviewer starts about halfway (median 48 % of the session);
- after it, 10 to 30 minutes for its fixes and the last checks;
- the gate takes 6 minutes at the median, 10 at the 90th percentile
  (69 runs);
- `nix flake check` with something to build takes 5 minutes at the
  median, 11 at the 90th (72 runs), and a few seconds when cached; a
  wait for the machine lock comes on top when four sessions share it.

The planning session's review of a finished session: 5 to 15 minutes
for a text or small session, 20 to 40 for a stack of several chains
with a combined gate and flake check.

The October steps (30 to 45) ran 2 to 6 hours each: they were larger,
ran from the CLI, and waited on prompts; they are a poor guide to the
sessions of today.

## Each step

| step | model | started | hours | transcripts | panel | gate runs (min) | flake runs over 0 (min) |
|---|---|---|---|---|---|---|---|
| 30b | opus-5-5 | 2026-10-17 07:40 | 2.97 | 1 |  |  | 3 3 |
| 30c | opus-5-5 | 2026-10-17 18:29 | 5.08 | 1 |  | 0 | 10 10 1 |
| 31a | fable-5-1 | 2026-10-18 05:44 | 2.12 | 1 |  | 1 | 2 |
| 35a | opus-5-5 | 2026-10-18 05:44 | 4.67 | 1 |  |  | 3 8 |
| 31b | opus-5-5 | 2026-10-18 06:39 | 2.72 | 1 |  |  |  |
| 31c | opus-5-5 | 2026-10-18 09:33 | 1.27 | 1 |  | 0 | 2 |
| 35b | opus-5-5 | 2026-10-18 09:33 | 4.63 | 1 |  | 0 4 |  |
| 31d | opus-5-5 | 2026-10-18 09:56 | 4.26 | 1 |  | 0 0 0 0 0 3 0 | 1 |
| 32 | fable-5-1 | 2026-10-18 16:01 | 6.75 | 1 |  |  |  |
| 39a | opus-5-5 | 2026-10-18 22:52 | 2.67 | 2 |  | 0 0 0 | 3 |
| 32a | opus-5-5 | 2026-10-18 22:52 | 1.19 | 1 |  | 0 0 0 0 0 |  |
| 36a | opus-5-5 | 2026-10-18 23:21 | 1.89 | 1 |  | 0 0 0 |  |
| 32b | opus-5-5 | 2026-10-19 06:53 | 1.10 | 1 |  |  | 2 4 |
| 36b | opus-5-5 | 2026-10-19 07:12 | 2.73 | 1 |  | 0 0 0 0 |  |
| 32c | opus-5-5 | 2026-10-19 08:10 | 0.84 | 1 |  |  |  |
| 40a | opus-5-5 | 2026-10-19 09:30 | 2.12 | 2 |  | 0 0 0 0 0 8 0 0 7 5 |  |
| 37a | opus-5-5 | 2026-10-19 09:54 | 5.75 | 1 |  | 10 4 5 |  |
| 40b | opus-5-5 | 2026-10-19 10:11 | 5.97 | 2 |  | 0 |  |
| 37b | opus-5-5 | 2026-10-19 15:06 | 4.19 | 1 |  | 0 0 0 0 0 1 0 0 0 4 3 3 |  |
| 38e | opus-5-5 | 2026-10-23 11:41 | 1.09 | 1 |  |  | 2 |
| 38f | fable-5-1 | 2026-10-23 12:46 | 0.39 | 1 |  |  |  |
| 38g | opus-5-5 | 2026-10-23 13:20 | 0.54 | 1 |  | 10 | 5 5 |
| 39b | opus-5-5 | 2026-10-23 14:05 | 1.49 | 1 |  | 0 6 5 |  |
| 43a | opus-5-5 | 2026-10-23 14:14 | 2.15 | 1 |  |  |  |
| 38h | opus-5-5 | 2026-10-23 16:31 | 0.77 | 1 |  | 0 0 0 0 |  |
| 38i | opus-5-5 | 2026-10-23 17:19 | 2.20 | 1 |  | 5 5 | 5 |
| 39c | opus-5-5 | 2026-10-23 19:28 | 6.39 | 1 |  | 0 0 |  |
| 41a | opus-5-5 | 2026-10-24 09:01 | 2.57 | 1 |  | 6 8 7 6 10 0 1 | 5 |
| 44 | opus-5-5 | 2026-10-24 09:01 | 1.34 | 1 |  | 0 0 0 |  |
| 43b | opus-5-5 | 2026-10-24 09:01 | 1.23 | 1 |  |  | 1 |
| 43c | opus-5-5 | 2026-10-24 10:15 | 1.03 | 1 |  |  |  |
| 42e | opus-5-5 | 2026-10-30 07:26 | 2.69 | 1 |  |  |  |
| 42f | opus-5-5 | 2026-10-30 10:08 | 1.34 | 1 |  | 9 |  |
| 42g | opus-5-5 | 2026-10-30 11:31 | 2.89 | 1 |  | 4 5 | 10 7 10 10 2 |
| 42h | opus-5-5 | 2026-10-30 14:27 | 2.24 | 1 |  | 0 0 2 0 |  |
| 42i | opus-5-5 | 2026-10-30 16:53 | 2.99 | 1 |  | 0 0 0 0 0 0 0 0 |  |
| 42j | opus-5-5 | 2026-10-30 21:16 | 3.67 | 1 |  |  |  |
| 42k | opus-5-5 | 2026-10-31 00:57 | 2.03 | 1 |  | 0 0 |  |
| 43d | opus-5-5 | 2026-10-31 03:00 | 0.95 | 1 |  |  | 3 |
| 44b | opus-5-5 | 2026-10-31 03:58 | 0.76 | 1 |  | 0 |  |
| 45a | opus-5-5 | 2026-11-02 10:46 | 0.66 | 1 |  |  | 22 1 |
| 45d1 | opus-5-5 | 2026-11-02 11:27 | 2.45 | 1 |  | 4 | 6 4 6 |
| 45d2 | opus-5-5 | 2026-11-02 17:32 | 4.69 | 1 |  |  | 4 |
| 45e | opus-5-5 | 2026-11-03 00:15 | 10.08 | 1 |  |  |  |
| 45f | opus-5-5 | 2026-11-03 09:52 | 0.90 | 1 |  |  | 26 3 |
| 46a | fable-5-1 | 2026-11-08 13:42 | 1.04 | 1 |  |  |  |
| 46b | opus-5-5 | 2026-11-08 15:04 | 0.41 | 1 |  |  |  |
| 47 | opus-5-5 | 2026-11-08 15:35 | 0.39 | 1 |  |  |  |
| 48a | opus-5-5 | 2026-11-08 15:35 | 0.50 | 1 |  |  |  |
| 48b | opus-5-5 | 2026-11-08 16:44 | 0.35 | 1 |  |  |  |
| 49a | opus-5-5 | 2026-11-08 17:48 | 1.55 | 1 |  | 7 4 5 |  |
| 49b | opus-5-5 | 2026-11-08 19:23 | 1.04 | 1 |  |  | 8 |
| 49c | opus-5-5 | 2026-11-08 20:27 | 0.99 | 1 |  |  | 1 |
| 50a | opus-5-5 | 2026-11-08 21:28 | 1.39 | 1 |  | 7 5 | 6 9 |
| 51a | opus-5-5 | 2026-11-08 22:05 | 0.78 | 1 |  |  | 8 |
| 53m | fable-5-1 | 2026-11-08 22:05 | 9.99 | 1 |  |  |  |
| 52 | opus-5-5 | 2026-11-08 22:53 | 3.89 | 1 |  | 2 9 5 9 8 | 10 |
| 51b | opus-5-5 | 2026-11-08 22:53 | 1.78 | 1 |  | 2 10 | 9 9 |
| 50b | opus-5-5 | 2026-11-08 22:53 | 0.62 | 1 |  | 10 | 2 1 |
| 53n | opus-5-5 | 2026-11-09 01:04 | 8.85 | 1 |  |  |  |
| 53r | opus-5-5 | 2026-11-09 02:39 | 4.85 | 1 |  |  |  |
| 53p | opus-5-5 | 2026-11-09 02:39 | 7.28 | 1 |  |  | 9 |
| 53a | opus-5-5 | 2026-11-09 02:47 | 2.09 | 1 |  | 10 4 4 | 2 8 |
| 53s | opus-5-5 | 2026-11-09 07:34 | 1.77 | 1 |  | 10 | 13 10 21 1 4 |
| 53b | opus-5-5 | 2026-11-09 09:31 | 1.01 | 1 |  |  |  |
| 54a | opus-5-5 | 2026-11-09 10:34 | 4.89 | 2 | yes | 9 9 8 4 |  |
| 54w-r | opus-5-5 | 2026-11-09 12:39 | 0.37 | 1 |  |  |  |
| 54w-b | opus-5-5 | 2026-11-09 12:39 | 0.34 | 1 |  |  |  |
| 54w-a | opus-5-5 | 2026-11-09 13:02 | 0.51 | 1 |  |  | 4 |
| 54b | opus-5-5 | 2026-11-09 14:45 | 2.40 | 1 | yes | 10 8 7 8 | 10 |
| 54w-e1 | opus-5-5 | 2026-11-09 14:45 | 1.82 | 2 |  | 13 5 8 | 10 4 |
| 54w-e2 | opus-5-5 | 2026-11-09 14:45 | 1.04 | 1 |  |  | 11 |
| 54s-a | opus-5-5 | 2026-11-14 00:07 | 1.98 | 1 | yes | 10 0 5 | 10 8 |
| 54u-b | opus-5-5 | 2026-11-14 00:07 | 1.11 | 1 |  | 2 0 8 5 | 1 11 |
| 54u-c | opus-5-5 | 2026-11-14 00:07 | 1.08 | 1 |  |  | 1 |
| 54u-a | opus-5-5 | 2026-11-14 00:07 | 0.81 | 1 |  |  |  |
| 54s-b | opus-5-5 | 2026-11-14 12:26 | 2.24 | 1 | yes | 6 5 |  |
| 54u-e | opus-5-5 | 2026-11-14 12:26 | 0.44 | 1 |  |  | 4 |
| 54u-d | opus-5-5 | 2026-11-14 12:26 | 0.94 | 1 |  |  | 1 9 1 |
| 54w-d | opus-5-5 | 2026-11-14 12:27 | 0.88 | 1 |  | 9 | 11 |
| 54w-c | opus-5-5 | 2026-11-14 14:03 | 0.83 | 1 |  |  | 3 20 |
| 54w-e3 | opus-5-5 | 2026-11-14 14:03 | 0.78 | 1 |  | 13 | 18 |
| 54w-e5 | opus-5-5 | 2026-11-14 14:04 | 1.00 | 1 |  | 16 | 9 |
| 54w-e6 | opus-5-5 | 2026-11-14 15:46 | 0.65 | 1 |  | 11 | 10 |
| 54s-b2 | opus-5-5 | 2026-11-14 16:07 | 0.68 | 1 | yes |  |  |
| 54s-c1 | opus-5-5 | 2026-11-14 16:07 | 0.45 | 1 |  | 14 0 | 3 |
| 54x | opus-5-5 | 2026-11-14 16:35 | 0.22 | 1 |  |  |  |
| 58a | opus-5-5 | 2026-11-14 16:37 | 0.19 | 1 |  |  |  |
| 58b | opus-5-5 | 2026-11-14 16:37 | 0.15 | 1 |  |  |  |
