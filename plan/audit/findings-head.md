# The audit's findings (step 86, phase B)

What an audit of deduxit's Rust libraries and its Rocq library found
before 0.9.0, deduplicated into themes and ranked. This file is
generated: `plan/audit/themes.py` writes the themes from
`plan/audit/themes.json` and the units' files, between this head and
the list of rejected findings; change those, not this file.

## What was read, and how

The tree at main after step 85 (`vqwqlqkl`, 2026-11-27) and
`rocq-deduxit` at its main: 239 000 lines of source in 21 units, each
read whole by one finder (Opus 5.5 at `high`) for five dimensions
(idiom, architecture and duplication, performance, the public API, text)
against the criteria of `criteria.md`; and the core crate as one design
by an architecture reader (Fable 5.2 at `high`). Every finding then went
to a verifier on the other model, told to refute it: it opened the place,
checked the witness, read the criterion and the rules files for a
recorded choice, and looked for callers the change would break
(`method.md`). Tools made the inventories the agents worked from
(`baselines.md`, `inventories/`).

| | findings | confirmed | candidates to measure | rejected |
|---|---:|---:|---:|---:|
| the architecture reader | 27 | 25 | 2 | 0 |
| 21 units | 352 | 300 | 44 | 8 |
| **all** | **379** | **325** | **46** | **8** |

A finding's full text (its witness, the change, the value, the net) is
`units/UNIT/findings.md`, and its verifier's reasons and corrections
`units/UNIT/verdicts.md`: read both before acting on one, since most
verdicts of "confirmed" correct the place, the cost, the kind or the
change (216 of 256 in stages 3 to 5).
`findings-index.md` lists all 379 in one table. `rejected.md` holds the
eight a verifier refuted, `standing.md` the recorded decisions a
proposal must answer, `handovers.md` what one unit asked of another.

No file of either repository was changed by the audit. Two defects it
found are being fixed beside it (theme A1).

## The ranking

The 371 findings that stand are in 45 themes and nine groups. The groups
are in the order the refactor takes them; inside a group the themes are
in the order of their value against their cost and risk: first what
many users gain from at small cost with an instrument that shows
behaviour is kept, last what is large, gains tidiness alone, or touches
a checker.

| group | themes | findings | what it is | when |
|---|---:|---:|---|---|
| A | 4 | 35 | defects and bounds that do not hold | first, as fixes |
| B | 10 | 96 | the public API for 0.9.0 | after the author signs `api-proposal.md` |
| C | 12 | 90 | duplication | beside and after B, unit by unit |
| D | 5 | 57 | performance | journeys first, then by measurement |
| E | 3 | 26 | files and modules to divide or move | after B's moves |
| F | 4 | 38 | docs, messages and help | one pass per area, after B |
| G | 1 | 4 | the command, beside its selector | with B's command step |
| H | 4 | 18 | the Rocq library | in its own repository |
| I | 2 | 7 | features and the web module's size | with B, by measurement |

What stands out:

- **The checkers are sound but for one case**, now being fixed: the
  first-order Kripke countermodel's check accepted `=` unread (A1). No
  other finding shows a wrong "proved" or "valid". Eleven calls do not
  keep the crate's own rule on limits and stops (A2), and the front
  ends answer wrongly in a few places because each chooses by hand what
  the library should choose once (A3).
- **The linear suite is still the crate's second shared layer.** About
  forty findings, from every unit that is not linear, are one fact:
  what all suites use lives under `linear` because it came first (B2,
  B3). Moving it is the largest single gain for a user of another
  suite, and it changes no behaviour.
- **An outside engine cannot yet be written** (B4): the consumer test's
  thirteen points are confirmed from inside the code, three are
  corrected (`Progress` can be built; `ordinary::Options` has
  `with_logic`; `Proof`'s docs do say that `check` is the boundary).
- **Duplication is the most frequent kind of finding** (90, and half
  of group B's 96 are the same thing seen from a user's side): two arenas, two
  connection provers, two text exporters, two net kernels, three
  sessions and their drivers in two front ends, and many small tables.
  The copies have already drifted apart, and several of group A's
  defects are that drift: the protocol's choice of a net, the lists of
  mode flags, the workspace's list of settings keys.
- **Performance has few measured facts and many candidates**, because
  the instruments run the linear suite and the exports and little else
  (D1). Where DHAT runs, it points at a handful of allocation sites
  that make most of a journey's blocks (D2).
- **The text is in good state by the patterns** (seven matches of the
  writing rules' list in 68 000 lines of prose) and behind in its
  content: front pages that lead with one suite, lists that fell behind
  the code, messages that do not name the fault (F).

