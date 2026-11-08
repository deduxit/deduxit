# Step 47: the measurement before the collection grows

You are working in the linlog repository. CLAUDE.md applies throughout: jj
only (never git), thematic commits as soon as a unit is done, no pushing.
The step writes no code beyond what a measurement needs to be repeated
(a script under `bench/`, if any). It may run beside step 48, which
touches other files. This prompt was written on 2026-11-08 by step 46's
second session, from the author's decisions of that day. It is complete.

Read `plan/reports/46-research-next.md` (session 46a: "The structure of
a collection of suites", "The unified web front end" and "What the next
sessions must know", whose second point is this step),
`plan/notes/research/next/structure.md` (section 5 and the
recommendation) and `web-front-end.md` (sections 1 and 3, the module's
size), `.claude/rules/bench.md`, `.claude/rules/web.md`, `bench/TARGETS.md`
and `plan/README.md`'s D17, D19 and D24 to D32.

## Why this step comes first

linlog grows into a collection of suites (D24), in one repository and
one workspace, with a kernel crate extracted only once a second
non-embedded suite has shown what is shared (D25), and one web module
until a suite from another repository arrives (D29). Each of those
choices will be weighed later against numbers nobody has taken: step
46's research ran nothing, and the module's size after step 42 (6.9 MB
uncompressed) is the last figure on record. This step takes the numbers
on the released code, so that every later step compares with a
measurement rather than a memory.

## Goal

A report of the costs a growing workspace will move, measured on
`main` at the step's start (0.2.0 and the plan commits after it, no
code changed since the release), each with the command that repeats it:

1. **The web module**: `linlog_web_bg.wasm` from `nix build
   .#linlog-web`, uncompressed, gzip at level 9 and brotli at level 11,
   and its share by crate or function where `twiggy` or `wasm-opt
   --metrics` can say it cheaply (a devshell tool added through the
   `new-tool` skill only if one is needed and worth keeping); and the
   web client's critical path as its own build reports it, against the
   budget of the client's design (D35 there: about 170 KB compressed
   before the module, which loads after first paint), read in the
   client's repository (`../linlog-prover.github.io`, its own CLAUDE.md)
   without changing it.
2. **The build**: `cargo build --timings` of the workspace, once from
   clean and once after touching one file of `core/src/ordinary/`, in
   the dev and the release profile, with the crates' share of the time
   (the HTML report's table, summarised as a text table: the author
   reads profiles as text, not as graphs);
   and the size of `target/` after each.
3. **The ratchet**: `ratchet` on the head, every journey's count against
   `bench/ceilings.csv`, as the gate runs it.
4. **The target sets**: `bench/targets.sh 47` and `bench/net-targets.sh
   47`, compared with the last labels on record (`bench/targets/45d.csv`,
   `net-45d.csv`) by `linlog-bench summary --against … --counters`;
   every decided row's counters equal, or the difference explained. The
   two files become the oracles the next steps compare with (D17), named
   in `bench/TARGETS.md`.

## Constraints

- **Timed runs**: the target sets and the pinned measurements need no
  yes from the author but are coordinated with the other sessions
  through the planning session (the author, 2026-10-30): before a timed
  run, ask `linlog-orchestrator` whether another session's load would
  disturb it, and keep to the cores it names (cores 2 to 15 only, never
  0 or 1). `bench/targets.sh` takes two cores for about twenty minutes,
  detached; `bench/net-targets.sh` one core for about five.
- `cargo build --timings` runs in a capped scope on cores 4 to 9 with
  `CARGO_BUILD_JOBS=6`, as `conduct.md` says, and its wall time is read
  as a rough figure (the cores differ in speed, `bench.md`); the
  crates' shares are what the later steps need.
- No change to the engines, the bindings, the harness's verdicts or the
  client. A script that repeats a measurement goes under `bench/` with
  its licence header and a line in `bench.md`; a tool added for it goes
  through `new-tool`.

## Deliverables

- `bench/targets/47.csv`, `bench/targets/net-47.csv` and their lines in
  `bench/TARGETS.md`, a commit of their own.
- `plan/reports/47-measurement.md`: each number with its command, the
  machine's state when it was taken, and a section "What the next
  sessions must know": the figures step 49 (the client's suite registry),
  step 63 (the suite protocol, the module question) and step 67 (the
  kernel crate) compare with.

## How the step runs

One session on Opus 5.5 at `high` (`plan/README.md`, "Why these models
and efforts": a step session), by day, its timed runs coordinated with
`linlog-orchestrator`. No reviewer: nothing it builds is checked by
another, and the numbers carry their commands.
