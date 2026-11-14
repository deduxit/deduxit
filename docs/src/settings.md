# The settings file

`--settings FILE` reads settings from a JSON file. `prove`, `check`,
`interact`, `eliminate` and `seq print` take it. The file holds the
same settings the flags set, in the form a front end keeps (the
library's `Settings`).

## What the file holds

The file is a JSON object with any of these keys:

| key | holds |
|---|---|
| `clock` | the time limits, and when a pool of threads joins the search |
| `limits` | the resources a call may use, such as memory and the sequent's size |
| `search` | the search's options |
| `view` | how a derivation is shown |
| `styles` | the options of every output format, as `--style-file` holds them |
| `batch` | how a batch shares the machine, and its entries' mode |
| `ordinary` | how ordinary logic is decided |
| `interactive` | how a session takes steps of its own |
| `input` | how the input is read |
| `cut` | how cut elimination proceeds and when it stops |
| `lambda` | how a lambda-term is printed |

Each key holds the fields the flags set. A flag's `--help` names its
field in parentheses: `--copies` sets `search.copies`, `--timeout` sets
`clock.time_limit_ms`, and `--memory-limit` sets `limits.memory_bytes`.
`null` lifts a bound, as `none` does on the command line:
`{"limits": {"memory_bytes": null}}`. A key or field deduxit does not
know is an error.

The file is read whole, up to 16 MiB, and never from standard input,
which carries the sequent.

## How the file and the flags combine

- A field the file leaves out keeps the command's default.
- A flag given on the command line overrides the file. A flag that is
  not given leaves the file's value as it is.
- A flag that only switches something on or off one way (`--no-check`,
  the mode flags) cannot undo the file's value: change the file
  instead.
- `batch.mode` is the mode of a batch's entries that name none, as the
  mode flags set it. A mode flag given on the command line overrides it.
- `ordinary` applies only under `--logic`, which names the logic. The
  file's `ordinary.logic` is not read: the settings a front end keeps
  always hold one, which would make every input ordinary logic. The
  file's `ordinary.translation` is the translation unless
  `--translation` names one.

## Example

```console
$ echo '{"search": {"copies": 0, "engine": "focus"}}' > settings.json
$ deduxit prove --settings settings.json "|- ?(~a * ~a), a * a * a * a"
unknown (MELL, classical, focus engine): the copy bound of 0 was reached after 300.39µs; raise it with --copies N, or lift it with --copies none to deepen it while the time limit lasts
$ deduxit prove --settings settings.json --copies 3 "|- ?(~a * ~a), a * a * a * a"
unprovable (MELL, classical, focus engine): the search was exhaustive
```
