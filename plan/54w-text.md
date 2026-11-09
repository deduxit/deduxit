# Step 54w: every text a person reads, rewritten

You are working in deduxit (`deduxit/deduxit`, the checkout
`~/Projects/own/deduxit`), and for some sessions in the web client
(`../linlog-prover.github.io`, `deduxit/deduxit.github.io`) or the Rocq
library (`../rocq-linlog`, `deduxit/rocq-deduxit`). CLAUDE.md applies
throughout:
- jj only, never git;
- thematic commits;
- the verification table;
- never push (deduxit-orchestrator pushes after your messages).

Written on 2026-11-09 by the planning session, from the author's request
of that day.

## Goal

The author, 2026-11-09: "rework all the human-facing text, i.e.
comments, docs, readme, teaching". The rules are
`.claude/rules/writing.md`, the author's standing instruction:
- concise, plain language;
- self-contained;
- written for someone using the library, the command or the page;
- free of the patterns that make text hard to read.

Read that file first. It names the education plugin's skills to use as
references for teaching text, and the patterns from Wikipedia's "Signs
of AI writing" to avoid.

## The sessions

Each session on Opus 5.5 at `high`, with its own brief and goal from the
planning session, its own jj workspaces, and the `fresh-reviewer` agent
once before its final message.

1. **54w-r, the usability review** (read-only, first). It answers, for
   the people who use deduxit, what they want to do and how easily they
   can. It writes `plan/notes/usability.md`.
   - **The users:**
     - a Rust programmer using the library;
     - someone at a terminal or in a script using the command;
     - a student or teacher using the web page;
     - a logician checking a proof or exporting it.
   - **For each:**
     - their most common tasks;
     - the shortest way to do each today, run for real;
     - where it is harder than it need be (a missing default, a name
       that misleads, three calls where one would do, an error that
       doesn't say what to do);
     - for each friction, the change that would remove it, with its
       cost to other uses.
   - **The proposals** that change the API, the command's flags or the
     page are questions for the author, each with a recommendation of
     the most idiomatic current practice, collected for the planning
     session. Nothing is changed in this session.
2. **54w-a, the web client** (`../linlog-prover.github.io`), beside step
   54. It rewrites:
   - the page's labels, messages and errors;
   - the Learn and Practice material, using the education skills the
     rules file names: worked examples first, one new idea at a time,
     feedback that says what to do next;
   - About and the README;
   - the client's own comments;
   - **a hint for every rule label** (the author, 2026-11-09: "can you
     give hover-hints for each of the applicable rules? It's a bit hard
     to know which exact rule is meant for each abbreviation"). The hint
     covers:
     - where a label shows: the rule popover's list of applicable rules,
       each inference's label in the derivation tree, and Learn's steps;
     - what it shows: the rule's full name ("⊗R: tensor, right"), its
       form (the premises over the conclusion), and one plain sentence
       on what it does;
     - every suite: the linear rules and LK's and LJ's.

     It follows the page's keyword-and-sentence pattern
     (`src/lib/words.ts`): it opens on hover, keyboard focus and tap,
     and is announced to screen readers, not a `title` alone. It is a
     feature the author asked for, so it is in scope although it
     changes the page. It needs no change to the library, and comes
     with e2e tests.

   Then it runs the e2e tests and `nix flake check`. Behaviour does not
   change: an e2e test that asserts an old text changes with it.
3. **54w-b, the Rocq library** (`../rocq-linlog`), beside step 54. It
   rewrites the README, the docs pages and the comments, keeping the
   theorems and their names. It runs its own `nix flake check`.
4. **54w-c, README, the manual, the changelog's Unreleased section and
   the guide for AI models**, after 54b.
   - Rewrite them in the tutorial, how-to, reference and explanation
     split the rules file asks for. A newcomer finds the shortest path
     to each common task, from 54w-r's notes.
   - The console examples stay real: `cli/tests/readme.rs` runs them.
   - Keep the guide's summary within its budget.
5. **54w-d, the command's help, messages and errors** (`cli/`), after
   54b. The help is the doc comments of `cli/src/argument_parsing.rs`,
   so `--help` reads as plain how-to text. Each error says what went
   wrong and what to do. The behaviour lock's changed lines are a commit
   of their own.
6. **54w-e and on, the crate's doc comments and comments** (`core/`,
   `web/`, `bench/`, `compat/`), after 54b, one session per group of
   modules so that no two edit one file:
   - `linear::sequents`, `parse`, the forest;
   - `linear::proofs` and the checker;
   - `linear::search` and its engines;
   - `linear::nets`;
   - `export`;
   - `ordinary` and `lambda`;
   - the root, `web/`, `bench/`, `compat/`.

   Doc comments say what an item is or does for a caller, with an
   example where a caller would want one. Comments say why. Nothing
   refers to how the code came to be. The rustdoc builds with
   `-D warnings`, and the doc tests run.

## Rules for every session

- **Text only.** A change to behaviour, the API, a flag or the page is
  out of scope. If one would help users, it goes to 54w-r's list, to be
  asked of the author.
- **Facts stay true.** Rewriting must not change what a sentence claims.
  Where a claim is wrong, fix the claim and say so in the report.
- **What stays for history:** the changelog's released sections, the
  `plan/` files and the reports keep their wording.
- **The checks:** the conventions check (no comment naming the plan),
  typos and treefmt, the manual and its links where docs change, and the
  gate where code files change.

## Deliverables

The commits, `plan/notes/usability.md`, the author's questions with
recommendations, and `plan/reports/54w-text.md`, one section per
session.
