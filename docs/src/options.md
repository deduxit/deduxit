# Limits, defaults and exit statuses

`deduxit <command> -h` lists a command's common options under the
headings Logic, Input, Output, Search and Limits, and ends with
"--help shows every option."; `deduxit <command> --help` lists every
option with its default. A flag that sets a key of the settings file
names the key in parentheses, such as `(search.copies)`
([The settings file](settings.md)).

## Exit statuses

| status | `prove` | `check` | `interact` |
|---|---|---|---|
| 0 | provable (valid under `--logic`) | valid | the session ends with a finished proof that checks |
| 1 | unprovable (not valid) | invalid | the session ends otherwise |
| 2 | an error | an error | an error |
| 3 | unknown | unknown: a limit stopped the check, or a refutation has no check | |
| 130 | a second Ctrl-C | | |

The first Ctrl-C stops the search, and the verdict is unknown. A
reader that closes the pipe early, as `deduxit prove … | head -1` does,
does not change the status. A batch's status is the worst of its
entries' ([Decide many sequents at once](batch.md)).

## The limits

| flag | default | what it bounds |
|---|---|---|
| `--timeout DURATION` | `2s` | the whole command, reading the input included (`500ms`, `10s`, `2m`, `1h`, `none`) |
| `--copies N` | `none` | how often a `?` formula may be copied on one branch; with `none` the bound deepens while the time limit lasts |
| `--forward-copies N` | `30` | the forward search's copy bound on a Horn program, under `--copies` |
| `--memory-limit SIZE` | `1GiB` | the search's memory (`512MiB`, `4GiB`, `none`) |
| `--work-limit N` | `none` | the search's units of work, which are the same on every machine |
| `--recursion-limit N` | `2048` | the nesting of rules on one branch |
| `--memo-limit N` | `1048576` | the decided sequents remembered at once |
| `--classical-work N` | `67108864` | the classical reading after the search (`0` turns it off) |
| `--witness-depth N` | `64` | the deepest term a first-order search fills a witness in with |
| `--instances N` | `65536` | the instances of quantified formulas a first-order search holds at once |
| `--derivation-limit SIZE` | `64MiB` | the derivation built from a proof; above it the verdict stands without it |
| `--occurrence-limit N` | `50000000` | the sequent's subformula occurrences |
| `--term-limit N` | `50000000` | the first-order terms the sequent's atoms unfold to, and a proof's instance table |

A search that reaches a limit answers unknown, exit status 3, and its
line names the flag that raises the limit. For the same verdict on
every machine, bound the search with `--work-limit N` or `--copies N`
and lift the time limit with `--timeout none`. `--deterministic` also
makes the proof and `--stats` the same on every run.

### The time limit

`--timeout` counts from the start of the command, so it covers reading
and parsing the sequent as well as the search. The command answers
within a fraction of a second of it, on one thread and on several. The
classical reading after the search runs past it within its own bound of
work, which this example turns off:

<!-- readme-check: machine -->
```console
$ deduxit prove -q --engine focus --timeout 1s --classical-work 0 "!(A -o A * A), !(A -o B), !(B * B -o C), A |- C * ?D"
unknown (MELL, classical, focus engine): the time limit of 1s was reached at a copy bound of 82; --timeout DURATION gives the search longer
```

The largest problem of the LLTP library is a file of 86 MB that takes
twelve seconds to parse. The limit ends its reading too, with exit
status 3 a second after the start:

<!-- readme-check: skip, the file is the LLTP library's -->
```console
$ deduxit prove -q -i --timeout 1s --file SYJ212+1.020.p
unknown: the time limit of 1s was reached while the sequent was read
```

The time limit and Ctrl-C also stop building and writing a derivation.

### Memory

`--memory-limit` counts the sequent as the search holds it (about 25
bytes for each subformula occurrence) and what grows with the search:
what it remembers, the proofs it keeps, and what each level of its
recursion takes. What the search remembers is emptied first when it no
longer fits, which costs time and never an answer. When that is not
enough, the verdict is unknown:

```console
$ deduxit prove --memory-limit 100 "|- (a & b) + (a & c), ~a par (~b & ~c)"
unknown (MALL, classical, focus engine): the memory limit of 100 B was reached after 32.28µs; raise it with --memory-limit SIZE
$ deduxit prove --memory-limit 100 --format json "|- (a & b) + (a & c), ~a par (~b & ~c)"
{"version":1,"deduxit":"…","verdict":"unknown","reason":{"kind":"memory_limit","limit_bytes":100},"fragment":"MALL","mode":"classical","engine":"focus","statistics":{"nodes":0,"memo_hits":0,"memo_entries":0,"splits":0,"links":0,"tests":0,"copies":0,"forward_copies":0,"work":0}}
```

### The size of the input

`--occurrence-limit` bounds the sequent on every command that reads
one. A sequent in JSON can share subformulas, so a file of 438 bytes
that doubles one atom 25 times stands for 67 million occurrences. It is
refused before anything unfolds it:

```console
$ deduxit seq fragment --file shared.json
error: the sequent unfolds to 67108863 subformula occurrences, more than the limit of 50000000; raise it with --occurrence-limit
```

`--term-limit` bounds a first-order sequent's terms: the terms its
atoms' arguments unfold to, which shared terms in JSON can make many
from a short file, and the entries of a proof's instance table that
`check` reads:

<!-- readme-check: machine -->
```console
$ deduxit seq fragment --term-limit 2 "|- p(f(f(a)))"
error: the first-order terms unfold to at least 3 terms, more than the limit of 2; raise it with --term-limit
```

No input ends the command other than by a verdict or an error. A
formula nested a hundred thousand deep is parsed, printed and searched
without recursion on it (the search itself gives up at
`--recursion-limit`), and a parse error far into a long line is shown
with the part of the line around it.

## Threads

| flag | default | what it sets |
|---|---|---|
| `-j N`, `--jobs N` | `auto` | the threads a search may use; `auto` is every thread the machine runs at once |
| `--pool-after DURATION` | `100ms` without `--jobs`, `0` with it | how long one thread searches before the others join it |
| `--deterministic` | off | one thread throughout, so the proof and `--stats` are a function of the input |

[How deduxit searches](engines.md#threads) explains how the threads
share the work.

## The JSON outcome

`prove --format json` writes one object. Its keys:

| key | value |
|---|---|
| `version` | the wire level of the document |
| `deduxit` | the version of deduxit that wrote it |
| `verdict` | `proved`, `unprovable` or `unknown` |
| `checked` | whether the proof passed the checker (with a proof) |
| `refutation` | why the sequent is unprovable (with `unprovable`) |
| `reason` | which limit stopped the search (with `unknown`) |
| `fragment`, `mode`, `engine` | as on the verdict line |
| `statistics` | the engine's counters, as `--stats` prints them |
| `sequent`, `nodes` | the sequent and the proof, which `check` reads |

Under `--logic`, the outcome is that of ordinary logic
([Decide ordinary logic](ordinary-logic.md)). The guide for AI models
([`skills/deduxit/reference.md`](https://github.com/deduxit/deduxit/blob/main/skills/deduxit/reference.md),
section 8) has every JSON form.
