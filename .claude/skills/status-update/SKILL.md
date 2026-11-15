---
name: status-update
description: >-
  Give the author a status update of deduxit's plan: every step and
  session in a window around now, in time order (done, running,
  planned), each with what it depends on, what it does, its model,
  effort and agents, and its time (taken, so far, left), then anything
  the author should do now. Use when the author asks for the status,
  the state of the work, ETAs or "what is going on", in the planning
  session.
argument-hint: "[hours of history, default 36] [planned steps, default 12]"
---

# Status update

The author reads this to know, at a glance, what is done, what runs,
what comes next and whether anything waits on them. It is a report, not
a log: short lines, plain words (`.claude/rules/writing.md`), the times
in the author's local time.

## 1. Bring the record up to date

`plan/notes/step-log.tsv` is the planning session's record: a row per
session (step or sub-step, kind, model, effort, agents, session id,
started, finished, a few words), and a row of kind `step` when a whole
step is complete. The planning session adds a row when it starts a
session and fills `finished` when it reviews it; check that now:
- `list_sessions {"linked": true}`: a session the log shows running
  that is idle and reported done gets its `finished` (its report
  commit's time, `jj log -r '::main@origin' -T 'author.timestamp()…'`),
  and is reviewed by the standing rule before the update goes out.
- A session running that the log lacks gets its row.
- The kinds are those of `plan/notes/step-durations.md`'s rules of
  thumb (text, small, client, panel, large, design, release, run).

## 2. The overview

Run `python3 -I plan/status.py --hours H --planned N` (the arguments,
or 36 and 12). It prints three tables: done in the window, running, and
planned in the table's order, from `plan/README.md`'s step table and the
log, with the estimates of the rules of thumb.

Then correct what the script cannot know:
- **Running**: read each session's last messages (`list_events`, a few)
  and say where it is (writing, in its reviewer, in its flake check,
  waiting on the machine lock); move its "left" by that (the reviewer
  starts about halfway; after it 10 to 30 minutes; a flake check 5 to 11
  minutes, more behind the lock).
- **Planned**: give each a rough start, from its dependencies and the
  free sessions (three or four run at once, each on its own cores), and
  say which can run side by side. A step whose prompt is to be finished
  at a review says so.
- Steps that depend on the author (a release's tags, a design choice)
  say so in their row.

## 3. Settings

Each row names its model and effort and its agents: the fresh reviewer
(never the writer's model), the panel (counterexamples on Opus,
argument on Fable, integers on Sonnet), writers, research sub-agents,
design panels (63g's, with the education and frontend-design plugins).
Take them from the log for done and running rows, and from the table's
model and effort columns for planned ones.

## 4. What the author should do

Check, and list only what applies, most urgent first:
- a session waiting on a prompt
  (`$XDG_RUNTIME_DIR/deduxit-sessions/notifications.log`, newest lines)
  or a card the author must click;
- a release waiting for the author (`gh api
  repos/deduxit/deduxit/actions/runs?status=waiting`, pending
  deployments), or a tag the plan holds for them;
- CI failing on `main` in any of the three repositories (`gh run list
  --repo deduxit/deduxit --limit 5`, and `rocq-deduxit`,
  `deduxit.github.io`): say whether it is the code or GitHub's runner;
- the keys (`keys-warm --check`): a locked signing or SSH key stops
  pushes;
- the baseline night, which waits for the author's word
  (`plan/notes/baseline-night.md`);
- a question a session's report holds for the author, if the planning
  session is not unattended;
- anything advisable (a long idle machine, a slot the author could give).

If nothing is needed, say so in one line.

## 5. The answer

Lead with one or two sentences: the time, what changed since the last
update, the next thing that will finish. Then the three tables (trim
long "what" cells to the part that tells the author what the step does),
then "What you could do" (or "Nothing needs you now"). End with the
next ETAs that matter (the release, the end of the current step).

Offer the overview as an artifact or a file only if the author asks;
in the conversation it is Markdown.
