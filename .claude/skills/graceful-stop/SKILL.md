---
name: graceful-stop
description: >-
  Pause all running work so that it resumes later with as little loss as
  possible: this session's own runs, every session it supervises, their
  sub-agents, workflows and systemd units. Use when the author says
  "stop" (or "pause", or that they need the machine now, or by a time),
  or when a supervising session sends "Pause". With a time `HH:MM`,
  work goes on as long as it can and everything has stopped by then.
  With the argument `resume`, picks the paused work up again, which is
  what the author's "go" or a supervisor's "Resume" asks for.
argument-hint: "[HH:MM | resume]"
---

# Graceful stop

The machine is shared with the author's other work, and they may need it
at any moment. "Stop" means: free the machine now, keep everything that
was done, and leave each piece of work where it can be picked up.
Nothing is abandoned and nothing is deleted.

`$ARGUMENTS` is empty for a stop now, a time `HH:MM` (24-hour, local)
for a stop by then, and `resume` for a resume.

## A stop by a time (`HH:MM`)

The author wants the machine free at `HH:MM` and the work going until
shortly before. The pause itself takes about ten minutes: the worker's
message read (at its next step, or only when its turn ends: a message to
a busy session in the app may wait for that), its units stopped, its
commit and checklist, then its turn ended. So:

1. **Read the deadline** `D` as today's `HH:MM`. If `D` is less than
   fifteen minutes away, or already past, stop now (the sections below)
   and tell the author why it did not wait.
2. **Tell each worker now** (`SendMessage`, no reply needed): the
   machine must be free at `D`; at `D − 12 min` it gets "Pause"; by
   `D − 4 min` every unit of its own but the warm
   loop must be stopped, or the supervisor stops it by name; until then
   it carries on, but starts nothing (a batch, a pass, a build) that
   cannot end or reach a resumable point by `D − 8 min`.
3. **Schedule the stop** with two one-shot crons (`CronCreate`,
   `recurring: false`, the minute and hour pinned to today's date):
   - at `D − 12 min`: the supervisor's steps 2 to 4 below (send
     "Pause", pause your own work and delete the recurring crons, noting
     them for the resume);
   - at `D − 4 min`: steps 3 and 5 at once (interrupt each worker still
     in a turn, stop every leftover unit by name, tell the worker which,
     check the load), then step 6 (tell the author).

   A cron fires only while this session is idle, so keep turns short
   from `D − 15 min` on, and run nothing long in the foreground.
4. **Tell the author** in two lines: what runs until when, and what the
   stop will cost (the batch or pass that will be cut).
5. **If the pause completes early** (every worker reports, no unit
   left), delete the `D − 4 min` cron and tell the author at once.

## Rules for every stop

- **A short task in progress may finish** if that saves work: a commit,
  a batch with a minute left, a test run nearly done. Anything longer
  stops at its nearest resumable point, and its cut part reruns later.
- **Record before you stop**: what each run has done, what it has not,
  and the exact command that resumes it. A stop that leaves no record
  loses the work as surely as a kill.
- **Stop by unit name or PID**, never `pkill -f` or `killall`: a
  pattern matches the asking shell too.
- **Keep the gpg warm loop** (the machine's unit `keys-warm`): it takes no
  CPU, and a resume needs signing.
- **Delete nothing**: scratch directories, workspaces, partial outputs
  and logs stay.

## As a supervisor (you started other sessions)

Step sessions are sessions in the Claude app that you started
(`start_session`). `list_sessions` with `linked: true` lists them with
their state, and `list_events` reads a session's transcript.

1. **Take stock**:
   - the sessions you started (`list_sessions` with `linked: true`,
     `ListAgents`);
   - your sub-agents and workflows;
   - your background tasks and monitors;
   - your crons;
   - the units: `systemctl --user list-units 'step*' 'deduxit-*'
     'linlog-*' --no-pager`.

   Note each worker's name, model, effort and session id (`local_…`):
   the resume messages it by that id.
2. **Pause each worker by message**: `SendMessage` "Pause" to its id,
   with `notify_when_idle` (a session you started with `wake_when_done`
   brings its note anyway). A worker whose brief does not mention this skill
   gets "Pause: run the graceful-stop skill as a worker". It finishes or
   cuts its task, stops its units, commits and writes its checklist,
   then ends its turn.
3. **Interrupt a worker that is still in a turn** at the deadline:
   `stop_session`. An idle session in the app takes no turn until a
   message comes, so a worker that ended its turn needs nothing more; the
   app keeps it with its conversation. A worker interrupted this way has
   a checklist that says less.
4. **Pause your own work**:
   - tell your sub-agents to stop at a resumable point, then `TaskStop`
     them;
   - stop the background tasks that run work, and your units by name;
   - delete your crons (`CronDelete`), since a supervision check would
     nudge a paused worker back to work. Note each cron's schedule and
     prompt so the resume recreates it.
5. **Check the units and the load** (`uptime`). Stop a worker's leftover
   unit by its name, and tell the worker which, for its resume.
6. **Tell the author**, shortly:
   - the machine is free (the load, no unit left but the warm loop);
   - what was paused and where;
   - what each piece loses (the batch it was in);
   - that "go" resumes it.

## As a worker (you got "Pause", or the author said "stop" to you)

0. **Told of a deadline** `D` ahead of time: carry on, but start
   nothing that cannot end or reach a resumable point by `D − 8 min`;
   with no supervisor, pause yourself at `D − 12 min` (`ScheduleWakeup`
   or a one-shot cron) and ask the author to type `/goal clear` if you
   have a goal.
1. **Finish or cut** the current task, by the rule above.
2. **Stop your sub-agents and workflows** at a resumable point (tell
   them, then `TaskStop`), and **your units** by name. A run that skips
   its finished parts on a rerun (a mutation batch whose list exists, a
   baseline that resumes) needs only a note of which part was cut.
3. **Commit what is finished** (`jj st` first). jj writes commits
   unsigned and a global hook signs them after the command; with the
   key locked it says they wait, and the first jj command after the
   unlock, or the push, signs them. Unfinished edits stay in `@`: say in
   the checklist what they are.
4. **Write the checklist**: each run's state, the command that resumes
   it, what comes next.
5. **Report and end the turn**: one line to the supervisor with
   `SendMessage` (the machine is free, the checklist is written), or to
   the author directly if there is no supervisor. No background task
   waits on anything while paused.
6. **Stay paused**: a turn started by a goal's check before the
   supervisor clears the goal does no work and ends at once. If you
   have a goal and no supervisor, ask the author to type `/goal clear`.

## Resume (`resume`, "go", "Resume")

- **A supervisor**:
  1. Test signing with a signature (`echo x | gpg --batch --pinentry-mode
     error --local-user flgrubm@grubmueller.dev --sign -o /dev/null`).
     If it fails, ask the author to unlock through SSH (`ssh -T
     git@github.com`, which unlocks the signing key too), and restart
     the warm loop (`keys-warm`).
  2. Restart your own units and recreate your crons.
  3. Resume each paused worker: `SendMessage` "Resume" to its `local_…`
     id. An idle session in the app takes it as a new turn, with its
     conversation and in its workspace. A session the app archived in
     the meantime (after 7 idle days) is unarchived first
     (`unarchive_session`).
- **After a reboot**, the author reopens the app, and you with it. The
  sessions are still there: resume each as above.
- **A worker**: read your checklist, rerun the commands it records, and
  carry on from there.
