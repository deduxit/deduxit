# Handoff: the planning session

You are the planning session of deduxit's repository (the local
checkout `linlog`), the orchestrator
of its stepped plan. The author (Fabian, GitHub `flgrubm`, the
repositories under `deduxit`, before 2026-11-08 `linlog-prover`) runs every step as Claude Code
sessions started from prompt files under `plan/`. You write and finish
those prompts, start each step's sessions and supervise them, review
each session before the next starts, stack and check their commits,
push `main`, keep `plan/README.md` true, and report to the author in
plain words. This file was rewritten on 2026-11-03 by the planning
session of the first account, which ran 42e to 45f from 2026-10-30 to
2026-11-03, and brought up to date on 2026-11-13 ("Read first", "Where
things stand", the models). It is for a planner on **either** account, `claude` or
`claude2`. Neither account shares the other's memory notes, so this
file is self-contained: where a memory note of yours says otherwise,
this file holds.

## Starting

- **The two accounts**:
  - `claude`: the first account, config `~/.claude`, Max 20x. Its memory
    notes are at
    `/home/tux/.claude/projects/-home-tux-Projects-own-linlog/memory/`
    (`MEMORY.md` the index).
  - `claude2`: the second account, a wrapper that sets
    `CLAUDE_CONFIG_DIR=/home/tux/.claude-2` and runs the same Claude Code,
    Max 5x. Its notes are at
    `/home/tux/.claude-2/projects/-home-tux-Projects-own-linlog/memory/`,
    last written for this project on 2026-10-29.
  - The sessions you start with `plan/start-session.sh` run on your own
    account: it keeps `CLAUDE_CONFIG_DIR` (unset for `claude`). Every
    session on one account shares its five-hour window, the planner's
    included, and the author's other projects fill it too.
- **How the author starts you**, in `/home/tux/Projects/own/deduxit` (`linlog` until 2026-11-09), from
  their shell, which is nushell (put `claude2` for `claude` on the second
  account):
  ```nu
  claude --name deduxit-orchestrator --model claude-opus-5-5 --effort high --permission-mode auto (open --raw plan/handoff.md)
  ```
  The name matters: every brief and goal names `deduxit-orchestrator`
  (`linlog-orchestrator` until 2026-11-09, when the author renamed the
  session with `/rename`) as the recipient of its messages, and the other project's session
  (below) writes to that name. Check with `ListAgents` that no other
  session of that name is alive: the first account's planner ended at
  this handoff. Auto mode matters too, since messages between sessions
  of different permission classes wait for approval.
- **Models and efforts**: the role table of 2026-11-13 in
  `plan/README.md` ("Why these models and efforts") holds for every
  session and agent you start, and for yourself.
  - You and the step sessions run on Opus 5.5 at `high`.
  - Fable 5.2 at `high` runs the design sessions the rows name (the
    first of 60, 65, 67 and 71), 66's Rocq proof, and the panel's
    argument.
  - The `fresh-reviewer` agent runs on Fable 5.2 at `medium`; it is
    called with `model: "opus"`, `effort: "high"` for a session on Fable.
  - Every session start, `Agent` call and workflow stage names its model
    and its effort.
  - Never `max`, `ultracode`, fast mode or the advisor.
  - Fable may use half of the weekly limit, and nothing bills to usage
    credits. When Fable's half is spent, its roles run on Opus 5.5 at
    `high`, noted in the Status log.
  - On the second account (Max 5x), the same choices with one step
    session at a time, and watch the window (below, "The usage limit").
- **Read first**, in this order:
  1. `CLAUDE.md`.
  2. `plan/README.md` up to its Status log: the step table, "Why these
     models and efforts" (its table of 2026-11-13 holds), "How the
     prompts are written", "The commands (nushell)", "Review protocol"
     and the decisions. Then the Status log from 2026-11-08 on, in
     full; read an earlier entry when a prompt or report points at it.
  3. `plan/conduct.md`, which every step prompt ends with.
  4. The steps under way: `plan/54-ljt-engine.md`,
     `plan/54s-sat-engines.md`, `plan/54u-usability.md` and
     `plan/54w-text.md`; the reports `plan/reports/54-ljt-engine.md` and
     `plan/reports/54w-text.md`; and `plan/notes/usability.md` §6, the
     changes the author accepted.
  5. `plan/55-kripke-countermodels.md`, the step after them.
  6. `plan/later.md`'s section heads.
- **Then find out where things stand** before you act:
  - `jj log -r 'main@origin..'` and `jj workspace list`, in linlog and in
    `../rocq-linlog`;
  - `ListAgents`, and `claude agents --json` for step sessions running or
    stopped;
  - `systemctl --user list-units 'step*' 'deduxit-*' 'linlog-*'`;
  - the notifications log, `$XDG_RUNTIME_DIR/deduxit-sessions/notifications.log`;
  - `plan/keys-warm.sh --check`.
  `gh` asks the author's permission (a settings rule), so read CI's state
  only when the author is there, or from the repositories' pages, the
  crates.io API and Zenodo's records. GitHub's unauthenticated API
  allows 60 calls an hour, so never poll it in a loop.

## Where things stand (2026-11-13)

- **0.3.0 is released** (2026-11-09):
  - the tags `v0.3.0` of deduxit (8e2bbb18) and rocq-deduxit (66905bcc),
    signed with the author's key;
  - `deduxit`, `deduxit-cli`, `linlog` and `linlog-cli` 0.3.0 on
    crates.io, with Trusted Publishing set;
  - the GitHub release, docs.rs, and Zenodo's records under both concept
    DOIs.

  The opam submission waits until there are more features (the author).
  It is prepared in `~/Projects/own/rocq-opam` (`mmsqlyzk`) and is redone
  for the release current then.
- **Done**: steps 47 to 53, and these sessions of step 54, each reviewed
  and pushed:
  - 54a and 54b: LJT, now the default for intuitionistic and minimal
    logic where it wins;
  - 54w-r: the usability review, `plan/notes/usability.md`;
  - 54w-a: the page's text;
  - 54w-b: the Rocq library's text;
  - 54w-e1 and 54w-e2: the comments.
- **Paused since 2026-11-09, 17:20**, at the author's word. No session
  starts before the author's go. Then, by the Status log of 2026-11-09
  (17:45 and 18:10):
  - On go: 54s-a, 54u-a, 54u-b and 54u-c, at most four step sessions
    at once. 54u-e starts when a slot frees, and 54w-e6 may start too.
  - 54w-c follows 54u-a, 54u-b and 54u-e. 54w-d follows 54u-b, and
    54w-e3 to e5 follow 54u-a.
  - After 54s, step 55 opens with its new first session (Q5, Q12).
  - Ask at the go whether the author will be away (unattended) or
    present.
- **The models** are the table of 2026-11-13 in `plan/README.md`. The
  prompts of 54s to 73 name it, and every brief you write follows it.
- **The heads**, all pushed. Only the default workspaces exist in the
  three repositories.
  - deduxit's `main`: the commit with this file.
  - The web client's `main` (`../linlog-prover.github.io`, the
    repository `deduxit/deduxit.github.io`): `qyxnytmo` (223ba9ef).
  - rocq-deduxit's `main` (`../rocq-linlog`): `qlxkqlrx` (df4b15ec).
- **Nothing runs.** `claude agents --json` lists no session of
  deduxit's, and there is no unit, not even the warm loop
  `step-gpg-warm`. Both keys were unlocked on 2026-11-13: start the loop
  with `plan/keys-warm.sh`.
- **Left beside the checkout** by earlier sessions: directories that are
  no longer jj workspaces.
  - They are `../linlog-step54a`, `../rocq-linlog-53b`, `-53n`, `-53r`,
    `-53s` and `-54w-b`, `../linlog-step41b-*`, and the `*-screenshots`
    directories.
  - Before deleting one, look for untracked results that a report
    cites.
- **Off `main`, never meant for it**; leave them:
  - `ulktsrrq` (2026-10-23), an earlier planner's review probe of MELL
    nets;
  - `zyqyunxy` (2026-10-30), a review probe of 42f's cut elimination;
  - `uworquvp` (2026-11-08), a change to 53's report.
- **The briefs and goals** of every session are in
  `/tmp/linlog-handoff/`.
  - `goal-54a.txt`, `goal-54b.txt` and `goal-54w-*.txt` are the recent
    models for goals.
  - 54's `start-*.md` briefs were not kept: write them by "How a step
    runs", item 2.
  - `/tmp` ages files out after about ten days.

## The steps left

The table in `plan/README.md` is the authority; its steps from 47 are
the roadmap step 46 set on 2026-11-08 from the author's decisions (D24
to D32), grouped by release. In brief:

- **0.3.0, ordinary logic on the page**: steps 47 to 53, done and
  released on 2026-11-09.
- **0.4.0, intuitionistic logic decided**: 54 the LJT engine (54a and
  54b done), 54s the SAT-based engines, 54u the usability review's
  accepted changes, 54w every text rewritten to `.claude/rules/writing.md`
  (under way, "Where things stand"), 55 Kripke
  countermodels with their Rocq lemma, 56 tableaux and truth tables, 57
  a checked basis for non-coverability, 58 maintenance with Kani, 59 the
  release.
- **0.5.0, custom rules and natural deduction**: 60 rules as data and
  the generic derivation (its first session on Fable 5.2), 61 building
  with custom rules, 62 natural deduction, 63 the suites' protocol and
  Learn and Practice by suite, 64 the release.
- **0.6.0, modal logics**: 65 K to S5 decided with countermodels (first
  session on Fable), 66 the labelled calculus and its Rocq soundness
  (the proof on Fable), 67 the kernel crate if the copies show it (first
  session on Fable), 68 tools for teachers, 69 the release.
- **0.7.0, resolution and the typed lambda calculus**: 70 resolution, 71
  the simply typed lambda calculus (first session on Fable), 72 a
  dataset for models, 73 the release.
- **After 73**: the research items (D30), the deferred verification
  (D31) and every candidate `later.md` keeps, for a step the author
  starts.
- The prompts from 54 on say what is fixed and are finished at the
  review their row names; `later.md`'s "Where each candidate went
  (2026-11-08)" says which `later.md` entries each step takes.
- **Proposed, not decided**: a desktop app (Tauri 2, about three
  sessions; asked by the author on 2026-10-25). Step 46 did not place
  it; it waits for the author.

## Open with the author

- **The move is done**: step 48 moved everything to the organisation
  `sequentium` on 2026-11-08; the author renamed it `deduxit` on
  2026-11-09 (the repositories `deduxit/deduxit`,
  `deduxit/rocq-linlog`, `deduxit/deduxit.github.io`; the library
  renamed `rocq-deduxit` the same day, 53r), and session 53n
  moved every address and the placeholder's redirects
  (`plan/reports/53-release-0.3.0.md`). Never create a repository
  named `linlog` or `rocq-linlog` under `linlog-prover`, nor take the
  name `sequentium` back for an organisation: either ends GitHub's
  redirects.
- **A custom domain** for the site: open; without one the addresses are
  the `github.io` ones.
- **The opam pull request**, after the library's `v0.3.0` tag: the
  change `mmsqlyzk` "Add rocq-deduxit.0.3.0" in `~/Projects/own/rocq-opam`
  (the chain for `rocq-linlog.0.2.0` is superseded and never sent). The
  planning session first fills in its checksum and `date:` tag by the
  commands of 53b's report; then the author:
  1. forks `rocq-prover/opam` on GitHub;
  2. in `~/Projects/own/rocq-opam`, runs `jj git remote add fork
     git@github.com:flgrubm/opam.git`;
  3. pushes it (`jj git push --remote fork --named
     "rocq-deduxit.0.3.0=mmsqlyzk"`);
  4. opens the pull request against `master` with the text in 53b's
     report.
  coqbot runs the repository's CI, and a maintainer merges.
- **Where step 66's Rocq proof lives** (a module of `rocq-deduxit` or a
  new library of the collection, whose repository the author would
  create): at the review of step 65.
- **A night for the quiet rerun of the fourth baseline's parallel stage**:
  its four files ran 16 threads beside other work and are marked "not
  comparable" in the manual. A rerun needs every core, the desktop's
  too, for about 1.5 hours (stage 3 of `bench/baseline.sh`, which resumes
  files it finds, so move the four files aside rather than deleting
  them). Then the four rows replace theirs in `bench/COMPARISON.md`
  and on the manual's page of the 2026-11-02 baseline.
- **The messages to the other provers' authors**: the drafts in
  `plan/notes/comparison-messages.md` now carry the overflow finding with
  its smallest example. Sending them is the author's.
- **Not now, by decision 9 (D32)**: GitHub Discussions and an
  announcement; ask again when the author raises them.
- Code signing for the installers; the WiX EULA for a Windows `.msi`;
  install channels that need an account (a Homebrew tap, the AUR,
  winget, Scoop, COPR, Chocolatey); rocq-deduxit's `CACHIX_AUTH_TOKEN`
  secret (checked again after its transfer, step 48); the LLTP header
  report (`plan/notes/lltp-headers.md`), theirs to send; the MCP server
  ("not now", kept in `later.md` until a client without a shell asks);
  the Zenodo community; the desktop app above; the web client's choices
  kept as built and the Safari pinch to check by hand (`later.md`,
  "Kept as built").
- nixpkgs: skipped at the author's word ("any nix user can already get
  it quite easily anyway", 2026-11-03).

## How a step runs

1. **Finish its prompt** at the review its row names: a section "What
   step N left you" (the facts a session cannot see: renamed items,
   numbers, the oracles, what the release promised), "How the step
   runs" (its sessions, each with its model and effort), "The machine"
   if another step runs beside it, and "Deliverables". Commit it as
   "Plan: finish the prompt of step N".
2. **Write each session's brief and goal** in `/tmp/linlog-handoff/`.
   - `start-NNx.md` says where the session starts and what it reads,
     its review, the machine's cores, "nothing that asks for the
     author's permission", how it ends, "Pause" and signing. It also
     names its workspaces: `jj workspace add --name stepNNx
     ../linlog-stepNNx -r REV`, and for the client `jj workspace add
     --name stepNNx ../linlog-prover.github.io-NNx -r CREV`.
   - `goal-NNx.txt` is one `/goal` line of numbered items, each proved
     "by a command's own output" (the judge sees only the conversation).
     It ends with: "Nothing is pushed. A failing check is fixed at its
     cause; never weaken, delete or loosen a test, a check or a
     tolerance to make it pass. If a requirement turns out impossible,
     say why in the report and in the message to deduxit-orchestrator."
   - A session that touches both repositories sends "linlog ready" after
     its linlog part; you review and push that part, then answer with
     linlog's commit and the client's revision to base on.
   - Briefs forbid entering the client's dev shell from linlog's
     directory: it once left `node_modules` and `.cache` symlinks in
     linlog's checkout.
3. **Start it**: `taskset -c CORES plan/start-session.sh deduxit-step-NNx
   MODEL EFFORT /tmp/linlog-handoff/goal-NNx.txt`.
   - It starts a background job (`claude --bg`) and prints the job's id
     and the session's id: note both, since the session id resumes it.
   - The `taskset` keeps the session's own process, and what it starts,
     off the cores a timed run needs.
   - The goal file is the launch prompt, so the goal is set at once.
     `claude logs deduxit-step-NNx` prints its recent screen: check that
     it shows `◎ /goal active`.
   - `SendMessage` to it with `notify_when_idle: true` and no message.
   - Keep two crons:
     - a supervision check (`17 */2 * * *`): its commits, its checklist,
       its screen (`claude logs`) and the notifications log (below) for a
       waiting prompt or a heads-up box, the units, the load, and
       `claude agents --json` for a job that stopped. Nudge only an idle
       session with work open, twice at most, and say nothing when all
       is well;
     - while unattended, a wake-up every 20 minutes (below).
   - Re-subscribe after each idle notice, but not to a session already
     idle (it fires at once).
4. **Review it** when it reports (its last message to
   `deduxit-orchestrator`):
   - read its report section and the diff (`jj diff -r 'BASE..HEAD'
     --stat`, every soundness-relevant file in full: the checker's arm,
     the Rocq trusted definitions);
   - run checks yourself, not only read them. Probe the command on what
     the tests do not pin: a `nix build -o SCRATCH/deduxit-main` binary for
     `main`, the session's `target/debug/deduxit` for its tree, on cores 4
     to 11. Compare target sets with their oracles (`deduxit-bench summary
     --against …`), recompute a checksum, rebuild a key check under the
     lock. You are the same model as the session, so your independence
     lies in what you run;
   - hold every "(departs from the design)" mark for the author;
   - fix a small defect yourself as a commit of its own; send a larger
     one back to the session.
5. **Stack and check**: rebase its chain onto `main` (`jj rebase -s
   FIRST -d main`), resolve conflicts by hand, and check the combined
   head. When code from two chains meets, that is the gate and `nix
   flake check` (they once found semantic conflicts each chain's checks
   had passed); when only plan files came between, the text checks
   suffice (`nix build .#checks.x86_64-linux.typos
   .#checks.x86_64-linux.treefmt .#checks.x86_64-linux.conventions`).
6. **Record it**: a dated Status entry in `plan/README.md` (what was
   delivered, what you checked and how, what the author must decide),
   the follow-ups into `plan/later.md`, the next prompts amended. Commit
   as "Plan: review session NNx", on top of the session's commits.
7. **Push** with `plan/push-main.sh REV` for linlog, `--client REV` for
   the client and `--rocq REV` for the Rocq library, the linlog part
   first, chained after its check (`check && push`).
   - The script fails within a minute when the SSH key is locked instead
     of waiting on a prompt, and checks `main@origin` after.
   - Never push any other way: any `jj git push …`, even with `--help`,
     matches the settings' `ask` rule, and an unattended run must not
     ask.
8. **Clean up**:
   - close the session: `claude stop JOB` (its conversation is kept;
     `claude rm JOB` deletes it, for probes only);
   - check `jj st` clean in its workspaces, and that no result it cites
     lives only there (untracked lists, CSVs, logs);
   - `jj workspace forget stepNNx` (by its jj name, in each repository),
     then `rm -rf` the directories, a session's helper workspaces too;
   - `systemctl --user reset-failed 'linlog-stepNNx-*' 'stepNNx-*'`.
9. **Report** to the author: the outcome first, then what you checked,
   then what needs them, in short plain sentences. When you give the
   next step, say what it does and every model and effort it starts.

## Mechanisms, and what each is for

- **Names** (the author, 2026-10-29, with the project's name since
  2026-11-09): every session and agent the planning session starts is
  named with the prefix `deduxit`: step sessions `deduxit-step-NNx`,
  sub-agents `deduxit-…`. The briefs ask step sessions to name their own
  sub-agents so too. (Before 2026-11-09 the prefix was `linlog`, and some
  of that night's sessions were plain `step-NNx`.)
- **A step session**: a background Claude Code session (`claude --bg`,
  started by `plan/start-session.sh`), one writer per coupled change,
  its goal in its launch prompt. Not agent teams, which are experimental
  and make teammates children of one lead. Not the Workflow tool for
  supervision either: it is for fan-out within a session, such as a
  step's panel, which the author chose a workflow for (step 26).
  - **Background jobs are the documented interface**
    (code.claude.com/docs/en/agent-view, checked 2026-11-09):
    - `claude agents --json` lists each job's id, name, session id and
      state;
    - `claude logs NAME` prints its recent screen;
    - `claude stop JOB` stops it (by job id, not name), keeping its
      conversation;
    - `claude --bg --resume SESSION_ID …` brings it back, through the
      launcher's `--resume`, under a new session id.
  - **What stops a job:** a machine restart stops every job, and an idle
    job that no terminal is attached to is stopped after about an hour.
    Resume it.
  - **Until 2026-11-09** sessions ran in detached zellij panes, and the
    planning session typed into them, an interface no document covers.
    Nothing needs that now.
- **`/goal`**: its judge sees only the conversation and has no turn cap,
  and the goal's last sentence lets it answer "impossible".
  - A goal cannot be paused, and a message cannot set or clear one: a
    command inside a message does not run.
  - So a pause is a "Pause" message, then `claude stop JOB` once the
    session is idle. The goal stays with the conversation and comes back
    when `--resume` restores it, its turn count and timer afresh.
- **Messages wake an idle session.** Code.claude.com/docs/en/
  cross-session-messaging says an idle receiver takes a message as a
  new turn, and a background probe confirmed it on 2026-11-09; a busy
  one reads it between tool calls.
  - A message can be held instead: across permission-mode classes
    (bypass against the rest), or when a session is set to hold or
    refuse. All of ours run in auto mode, one class, so none is held.
  - While unattended, keep a cron every 20 minutes (`*/20 * * * *`) for
    what sends no message: a CI run, a unit, a job that stopped. Crons
    fire only while you are idle, so keep turns short, and rewrite the
    cron's prompt when the work moves on.
- **A session waiting on a prompt**: a session in auto mode still stops
  at a command a settings `ask` rule matches; on 2026-11-03, 45e stopped
  at `jj git push --help`.
  - The `Notification` hook (`.claude/hooks/notify-supervisor.sh`)
    appends each such wait (a permission prompt, a question, a session
    needing input) to `$XDG_RUNTIME_DIR/deduxit-sessions/notifications.log`
    with its `cwd`. Read the log in every check.
  - Never approve on the author's behalf, and tell the session another
    way (here, the forms `plan/push-main.sh` uses). Answering the prompt
    itself needs `claude attach`, which is the author's.
- **A session that uses `EnterWorktree`** to move into its jj workspace
  waits on a prompt (moving outside `.claude/worktrees/` always asks).
  Briefs say "absolute paths and `cd` in Bash, never EnterWorktree". If
  one is stuck there anyway: `claude stop JOB`, then resume it with that
  instruction.
- **Your own turn waiting on a prompt** blocks everything you supervise:
  on 2026-11-09 a raw `jj git push` held it for two hours.
  - Run nothing in your own turn that an `ask` rule matches while the
    author is away.
  - A `gh` call the author asked for goes to a background sub-agent: its
    prompt shows in your session and waits for the author, while your
    turn goes on.
  - The auto-mode classifier refuses to make public surfaces on its own,
    such as release tags or a public repository. Prepare them (signed
    tag objects, a script), and let the author run them.
- **Heads-up boxes in a session's screen** (Claude Code's built-in
  plugin `cc-plugin-you-should-know`: "Heads up · …"). Each carries a
  finding about that session's work, often a real one.
  - Read its text in `claude logs NAME`, and act on it, usually with a
    short message to the session.
  - A box doesn't block the session, so it needs no dismissing.
  - A note that reaches you as "a note offered by a side agent" can be
    stale: check it against what the session reported since.
- **Reviewers and panels are named agents** in `.claude/agents/`:
  - `fresh-reviewer`, Fable 5.2 at `medium`, the once-per-session
    reviewer of `plan/conduct.md`. A session on Fable calls it with
    `model: "opus"`, `effort: "high"`, so that it is never the writer's
    model;
  - `panel-counterexamples` (Opus 5.5), `panel-argument` (Fable 5.2) and
    `panel-integers` (Sonnet 5.5), all at `high`: the three members of
    step 26's panel.

  Each carries its model, effort, tools and the rules for anything it
  runs. A brief names the agent and gives it the commits, the brief's
  path and the criteria.
- **Sub-agents** (`Agent`): each call names its model and effort.
  - Reading and searching run on Sonnet 5.5 at `medium`, and research
    that a decision rests on at `high`.
  - A mechanical extraction runs on Haiku 5.5 at `medium`.
  - One that names no model runs Sonnet 5.5
    (`CLAUDE_CODE_SUBAGENT_MODEL` in `.claude/settings.json`). The
    exceptions are the built-in Explore and Plan agents, which keep their
    parent's model (Explore under Fable runs Opus 5.5).
- **The machine's lock**: `/tmp/linlog-machine.lock` (`flock -o`, so a
  build's own children don't inherit it), held by every timed run and
  every `nix build` or `nix flake check`, since the daemon builds on any
  core. Several checks queue on it: a `flock` waiting is normal.

## The machine

- **Its cores**: an Intel Core Ultra X9 388H, 16 cores without SMT. CPUs
  0–3 are performance cores (5.1 GHz), 4–11 efficiency cores (4.0 GHz),
  12–15 low-power cores (3.7 GHz).
  - Cores 0 and 1 are the desktop's: runs keep to 2–15, and queue rather
    than take 0 or 1.
  - A linlog step's builds and tests go on a few cores (2 to 7, or 4 to 7
    beside a timed run), every one in a memory-capped scope
    (`systemd-run --user --scope -p MemoryMax=8G -p MemorySwapMax=0
    taskset -c …`). A bare test run once froze the machine.
  - Mutation runs (6 to 11) and fuzzing (12 to 15) only where a review
    needs them.
  - Kill by unit name or PID, never by pattern: `pkill -f` once matched
    its own shell.
- **Confine by affinity**: the user manager has no cpuset controller, so
  a unit's `AllowedCPUs` does nothing. `taskset` (or `systemd-run -p
  CPUAffinity=…`) is the one mechanism that works, and `bench/baseline.sh
  --cpus` uses it since 45d2.
- **Another project shares the machine**: the session
  `learning-website-orchestrator`, for the author's site `website-learn`.
  linlog has priority.
  - It keeps its heavy jobs on cores 8 to 15, and wraps its `nix flake
    check` and `nix build` in `flock -o /tmp/linlog-machine.lock` with
    `--max-jobs 2 --cores 6`, as everyone does since 2026-11-02 (the
    daemon's builds otherwise take every core, the desktop's too).
  - It runs at most four unit writers, never a server under the lock (a
    preview server once held the lock for half an hour), and no e2e
    group or full Vitest run while anyone's full check holds the lock.
  - Before each timed run of linlog's, send it "pause heavy work" with
    the run's length and cores, and "go ahead" after. On 2026-11-03 it
    moved its whole process to cores 10–11 (`taskset -acp`) for the
    run's length.
  - Other projects' sessions come and go (`effect-optics-orchestrator`
    on 2026-11-02): look at `ListAgents` before each timed run and tell
    each.
- **After a reboot**:
  - every step session's background job has stopped:
    `plan/start-session.sh --resume SESSION_ID NAME MODEL EFFORT
    "Resume"` brings each back with its context and its goal;
  - signing and SSH are locked: ask the author to unlock through SSH
    (`ssh -T git@github.com`, which unlocks the signing key too; signing
    alone leaves SSH locked), then run `plan/keys-warm.sh`;
  - `nix-optimise` catches up its missed night run some minutes after
    boot (`systemctl list-timers`). Start no timed run before it has
    finished; it took a minute on 2026-11-03.
- **Timers that would disturb a night run**: `nix-gc` at midnight,
  `nix-optimise` at 03:45, `borgbackup-job-eth`, and `obsidian-snapshot`
  (a user timer) at 23:00. System ones need the author's `sudo
  systemctl stop …` for the night.
- **Night jobs** (the baseline) start detached when ready
  (`bench/baseline.sh --detach --fresh`), on the night the author gives.
- **"Stop" or "Pause" from the author**: the `graceful-stop` skill
  (`HH:MM` for a stop by then, `resume` for the resume). It records each
  session's goal file for the resume.

## What the author has asked for, standing

- **jj only, never git**, not even to read. Every commit is signed as
  `flgrubm@grubmueller.dev`, with no other identity.
- **The keys**:
  - Keep both warm with `plan/keys-warm.sh` (the unit `step-gpg-warm`,
    signing and SSH every four minutes; `--check` says which is
    unlocked).
  - Signing lapses 12 hours after the author entered the passphrase,
    however warm the loop. Then the author types:
    ```
    ! gpgconf --reload gpg-agent
    ! echo x | gpg --local-user flgrubm@grubmueller.dev --sign -o /dev/null
    ```
    plus `! ssh -T git@github.com` if SSH lapsed too, and you run
    `plan/keys-warm.sh` again.
  - Never touch `~/.gnupg`. If a jj command fails on a stale keyring
    lock, run `/home/tux/.claude/hooks/unwedge-gpg-lock.sh`.
  - Since 2026-11-08 jj writes commits unsigned (the author's flake,
    `modules/jujutsu.nix`): the global hook
    `~/.claude/hooks/jj-sign-commits.py` signs a session's new commits
    right after the jj command that made them and reports their new
    ids, and `git.sign-on-push` signs what is left before a push, so a
    lapsed key stops no jj command and no commit. It does stop the push
    (the signing before it fails), and `push-main.sh` fails then: tell
    the author. Never pass `--config signing.behavior=…`; auto mode
    refuses it anyway.
- **Pushing** `main` of all three repositories without asking, after
  review, as above.
  - Tags, releases and every administrative `gh` task (settings,
    rulesets, secrets, Pages, environments, a workflow run, a new
    repository) need the author's yes for that one task.
  - Deleting and transferring are theirs alone.
- **Unattended, deciding** (the author, 2026-10-30, 22:07: "consider
  yourself unattended from now. If any decisions come up, please decide
  them in the way that rational thinking makes you believe is best").
  - Decide what would have gone to the author, each decision with its
    reason in the Status log.
  - The tags, publication, the night baseline, `gh` administration and
    anything outward stay theirs.
  - The author has been present since 2026-11-02 and gives the go for
    each step: ask at the next go whether it is unattended again.
- **Timed runs need no yes**, only coordination (the author, 2026-10-30,
  and again on 2026-11-03: "feel free to do some benchmarks now.
  Coordinate with the other Claude code sessions on resource usage").
  - A session tells the planning session before each timed run, and the
    planning session pauses the other sessions' heavy work, its own
    sessions' included ("pause heavy work", then "go ahead").
  - The night baseline still waits for the night the author gives.
- **Benchmarks are protected from other load** (the author, 2026-10-30).
  - Whenever a timed run is going, look for any other session or unit
    that would disturb it: its cores, the shared caches, memory
    bandwidth, a `nix` build in the daemon on any core, a system timer.
  - Coordinate with it: ask that session to move off the timed run's
    cores, hold its heavy work until the run ends, or pause the run and
    resume it, whichever loses less.
  - A timed run's report says what else ran beside it.
- **Nothing that asks for permission while unattended**: no `gh`, no
  `jj git push`, nothing the settings mark `ask`.
- **Idiomatic and current** (2026-10-25): everything written the way its
  ecosystem writes it today, from that ecosystem's current documents
  and recent examples, read, not remembered (`plan/conduct.md` says so
  to every session); hold your own prompts to it.
- **Design departures**: a session marks one "(departs from the
  design)" and goes on; you relay each to the author.
- **The web client**: Practice holds only provable problems; Learn and
  Practice draw from one store of problems, whose flake check decides
  every one by deduxit's command; every lesson problem is decided well
  inside the page's budget of work, except lesson 7's deliberate
  small-budget example. The mark is dēdūxit in Gentium ExtraBold, its
  long marks lapis (#2456c9 light, #7aa7ff dark) as `--color-logo`,
  CC BY 4.0, held to the `.logo` rule by the colour test. The client's content is CC BY 4.0, its code EUPL-1.2.
- **The guide for AI models** (`skills/deduxit/`): a step that changes a
  feature it describes changes it in the same commit (conduct.md);
  remind sessions that touch the command, the JSON forms or the
  library's entry points.
- **Prompts** say what is wanted and why and leave the design to the
  session; each names its model and effort; a step that changes a
  search, a checker or a criterion has the panel (the argument on
  Fable 5.2, counterexamples on Opus 5.5, integers on Sonnet 5.5).
- **Commands for the author** are nushell: `;` not `&&`, `(open --raw
  f)` not `$(cat f)`; a command they run in this session is typed with
  `! ` in front.
- **Testing** only as necessary; never weaken a test to make it pass.
- **Reports**: plain words, the outcome first; profiles as text and
  tables, never flame graphs.

## The usage limit

- **Both accounts stop at their five-hour limit**, the planner too. A
  step session shows "Goal paused · usage limit reached" and does not go
  on by itself; its running sub-agents die with an API error.
- Claude Code continues a session by itself at the reset, but only for a
  turn that a person's prompt or a peer's message started; a turn that a
  cron, an idle notice or a task notification started does not.
- So keep a background wall-clock wait ending a few minutes after the
  reset the pane names (`while [ "$(date +%s)" -lt TARGET ]; do sleep 30;
  done`, `run_in_background`), and at the wake message the session to go
  on and restart any sub-agent that died. A plain `sleep` counts no
  suspended time, and the laptop suspends on lid close.
- On the second account (Max 5x) this comes often; on the first (Max
  20x) rarely.

## Things that bit earlier sessions

- **Stacked chains conflict semantically**: check the combined head.
- **A report can say more than was done**: check that what it cites
  exists and that the commit it names is the head (sessions amend their
  report commit after their message).
- **`jj rebase -r X -d Y`** moves only X: its children (an empty `@`
  among them) drop onto X's old parent; use `-s` for a chain, and `jj
  new TOP` to put `@` back on top.
- **Shell edits skip the formatter**: `nix fmt FILES` before
  committing. `jj squash` into a described commit needs
  `--use-destination-message` or `-m`; never `jj new -m`.
- **nix sees only tracked files**: `jj st` after creating a file,
  before a `nix` command that must see it.
- **A snapshot is a signed commit**: read another workspace with `jj
  --ignore-working-copy`; a background run writing into the repository
  lands in `@` (write timed runs' output to a scratch directory and copy
  it in).
- **The Bash tool is zsh**: an unquoted `====` is an error; quote
  separators; a foreground `sleep` is refused, so wait with
  `run_in_background` or Monitor.
- **An uncapped build**: a `cargo run` typed in a hurry runs on every
  core outside any scope; even a probe goes through the capped scope.
- **Delete nothing unread**: look at a workspace's untracked files before
  removing it (a reviewer's uncommitted scratch was once lost).
- **A timed run measures giving up as easily as deciding**: a row that
  ends at a limit (the recursion limit, the copy bound) times the limit,
  not the search; read the reason column before comparing times.
- **Found and parked** (`plan/later.md`): without `--deterministic` the
  command answers unknown on a sequent the inverse method saturates at
  once; deduxit's
  recursion limit on `lambek-fo` from 512 on (LinearOne solves them);
  the memory account that never reaches its limit on `wide-m1/8192`.

## When asked to re-evaluate models and efforts

Read the model docs and Claude Code's of the day (the page names are in
"Why these models and efforts"), argue from what the steps so far showed
and from the account's quota (the first Max 20x, the second Max 5x),
and add a dated paragraph there.
