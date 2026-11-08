# Step 72: problems with verified answers for evaluating models

You are working in the linlog repository (`sequentium/sequentium`).
CLAUDE.md applies throughout: jj only (never git), thematic commits as
soon as a unit is done, doc comments on every item, the verification
table, no pushing. This prompt was written on 2026-11-08 by step 46's
second session, from the author's decisions of that day, and is finished
at the review of step 69: it says what is fixed.

Read `plan/notes/research/next/llm-agents.md` (candidates 2 and 6),
`scope-engines.md` (engines-7), the guide for AI models
(`skills/linlog/`), the report of step 44, and `bench/RESULTS.md`'s
sources and licences.

## Goal

What models need of the collection beyond the guide: a dataset of
problems in every suite with answers linlog has verified (each proof
checked, each refutation checked or certified, each "unknown" left
out), in a form current evaluation harnesses read, and a one-call
grading mode over `interact`'s session script that checks a model's
proof or refutation and answers with the exit statuses.

## Fixed now

- **Only sources whose licences allow it**: the generated families and
  ILTP's problems; LLTP (GPL-3.0) stays out.
- The dataset's format and its card follow current practice, read by a
  research sub-agent on Sonnet 5.5 at `medium` with links; nothing is
  uploaded to a hub without the author's act.

## How the step runs

One to two sessions on Opus 5.5 at `high`. Deliverables: the commits,
the guide's lines, `plan/reports/72-model-dataset.md`.
