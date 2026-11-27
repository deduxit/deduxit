# Step 87c9: the command's repeats, part one (report)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the
workspace `../deduxit-wt-step-84-dc9a99`, from `plan/87-refactor.md`
(87c9, wave C, after 87b3) and the planning session's briefs
`start-87-common.md` and `start-87c9.md`, on cores 4 to 6. Based on
87b3's head (`rvykkpsr`). No change of behaviour: every output is
byte for byte as it was (the behaviour lock and the manual's examples
pass unchanged).

## The step divides (departs from the plan)

The step is ten findings, two of them large and tied to each other:
F-7a-12 (one decision of a sequent, for `prove`, the batch and a
session's `close`, with an answer value that `grade` reads) and F-7b-05
(the library's default in place of the command's own first-order
decision, which lives inside that decision's embedding fallback, the
batch and their writers). F-7a-13 (the common flags declared once) is a
restructuring of every command's arguments, whose docs differ by
command, and F-7b-09 (one mapping from a refusal's setting to its flag)
needs those flag groups, since a flag exists on some commands only; it
also changes refusal lines that the manual and the lock show. The plan
lets a step that proves too large for one session divide along its
themes, and this one does:

- **Here, the output code's repeats**: F-7b-21, F-7b-17, F-7b-16 and
  F-7a-14, each a refactor that keeps every output, and the baseline
  F-7b-05 needs (below).
- **87c9b, the decision, the flags and the sessions**: F-7a-12 with
  `grade` reading the answer value, F-7b-05 measured against the
  baseline here, F-7a-13 with F-7b-09 on its flag groups, F-7b-08 (one
  session driver over the linear and the ordinary session) and F-7b-07
  (one writer of a rules derivation, PNG and PDF for `check`, which is
  additive).

## What landed

| finding | change | commit |
|---|---|---|
| F-7b-21 | `show::counted` is the one count-with-noun helper (three copies, `resolve`'s `plural` and three inline forms gone); generic over the integer, so `u64` counts take it too | `mklmyskz` |
| F-7b-17 | `eliminate` calls `sequent_of` where a closure copied it, builds the line and the JSON record of a failed check once (`check_failed`, `check_record`), and dispatches on the net once (`run`, which returns the library's record for JSON where no step callback is given) | `mklmyskz` |
| F-7b-16 | `show::drawn_into` writes a drawing as text or renders it and says why one is left out, where five copies did; each caller keeps its words (the modal model's and the rules derivation's "it was stopped") | `mklmyskz` |
| F-7a-14 | `Checked::new(word)`, `Checked::of(status)` and one `with_…` setter per field (a value or an `Option`) replace fifteen ten-field literals; two sites keep their own word for a status (`check`'s proof, where an error is `unknown`, and the Kripke model's, where unknown is `invalid`) | `vrslvkmq` |

The tagged suite field of F-7a-14 is left out, as its verdict says: it
would change a wire form, which the proposal for all wire forms decides.

## F-7b-05's baseline

`bench/defaults/command-first-order-before.csv` holds the command at
this step's base (`rvykkpsr`, a release build from a workspace of its
own) on ILTP's 2 670 first-order problems in classical and
intuitionistic logic: `prove --input-format tptp --logic L --jobs 1
--timeout 2s --quiet --file P`, one row per run (problem, logic, exit
status, first line, seconds), three runs at a time pinned to cores 4,
5 and 6, no build beside them. The script is
`bench/command-first-order.py` (`tytxwxlr`). 87c9b's after run is the same
command with its own release build:

```sh
bench/command-first-order.py BIN bench/iltp-first-order/Problems AFTER.csv 4,5,6
```

and compares the exit statuses row by row: a row that ends at the time
limit on one side is no verdict change, a decided verdict that flips is.
`--jobs 1` is the sequential path the verdict names, so the race on a
second thread is not in it; a run with the default jobs goes beside it
there.

The baseline (5 340 rows, no duplicates; exit statuses 0 valid, 1 not
valid, 3 unknown, 2 error):

| logic | 0 | 1 | 3 | 2 |
|---|---|---|---|---|
| classical | 994 | 296 | 1 376 | 4 |
| intuitionistic | 471 | 388 | 1 807 | 4 |

The eight errors are four problems the TPTP reader refuses
(`GEJ056+1`, `GEJ056+2`, `GEJ057+1`, `GEJ057+2`: "expected a bound
variable or a constant" at character 80 of a long line), a gap of the
reader that predates this step, noted for a fixup. The run was paused
once at 2 019 rows and finished from its CSV, with nothing built beside
it.

## The net

- **No target set**: nothing under `core/` changed.
- **The behaviour lock and the manual's examples** pass unchanged: no
  output moved, so no help page and no lock file was reblessed.
- **The gate** (cores 4 to 6): passed at `tytxwxlr`, and again at the
  final head (`ptypmmzu`). The ratchet's journeys are within their
  ceilings. The misses note names `search-chain-128` and
  `check-qbf-20-2` as every gate since step 84 does, and now
  `search-chain-fo-32` too: a journey of the linear search, which this
  step does not touch (its base carries other steps' core changes).
- **`breaks 0.8.0`**: nothing of this step, which changes no library
  code; its three failures are main's (`StepError::Witness::reason` and
  the related fields, System F's variants, `Workspace::read_own`
  deprecated).
- **`nix flake check`** under the machine lock at the final head: all
  checks passed (this line still a placeholder there).

## The review

`fresh-reviewer` (Opus 5.5, high, by the planning session's rule since
Fable passed half its weekly share) read the chain. It confirmed every
replaced form writes what it wrote: each `counted` site, `eliminate`'s
check report, record and net dispatch, each of the five render tails,
and each of the fifteen `Checked` literals field by field. It also
confirmed the CSV complete and the script as documented. Two findings,
both fixed in `mklmyskz`:
- **`show.rs` had two blank lines** where rustfmt keeps one, which the
  `treefmt` check would have failed.
- **A failed write printed another message on three paths**: the modal
  model's and the rules derivation's drawings and `eliminate --net
  --format json` wrote through `io::Write` on the output's stream, whose
  error names the device ("No space left on device"); through the
  output's `fmt::Write` the error is a bare formatting error.
  `drawn_into` now takes the caller's writing as a closure, so each path
  writes as it did, and `eliminate`'s record goes to the stream again.

## Commits

Oldest first, change ids, on 87b3's head (`rvykkpsr`):
- `mklmyskz` Write a count with its noun once, eliminate's repeats
  once, and a drawing's render once
- `vrslvkmq` Build check's JSON record by a constructor and setters
- `tytxwxlr` Bench: the command's first-order verdicts before the
  library's default
- `ptypmmzu` Plan: 87c9's report

## Paused

The step paused once for the author's CPU, the before run cut at 2 019
rows, and resumed the next day from its checklist.
