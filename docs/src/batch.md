# Decide many sequents at once

Write one sequent per line into a file, as `NAME: SEQUENT` or just
`SEQUENT`, and give the file to `prove` with `--input-format lines`:

```console
$ cat > sequents.txt
> # a comment, then four sequents
> identity: A |- A
> modus ponens: A, A -o B |- B
> A |- B
> broken: A |- (
$ deduxit prove --input-format lines --file sequents.txt
identity: provable (MLL, classical, focus engine)
modus ponens: provable (MLL, classical, focus engine)
sequents.txt:4: unprovable (MLL, classical, focus engine): A on the left has no A on the right to meet in an axiom
broken: error: cannot parse the sequent
   A |- (
         ^ unexpected end of input, expected a formula
```

Every sequent gets one line, in the order of the input, as soon as it
is decided. A line is named by its name, or by its file and line
number where it has none. Blank lines and everything from `#` on are
skipped. A sequent that cannot be read gets an error line, and the
batch goes on.

## Other inputs

`prove` decides a batch whenever it is given more than one sequent:

- `--file` more than once, or a directory;
- `--files-from LIST`, a file of paths, one per line, or separated by
  NUL with `--null` as `find -print0` writes them; `-` reads the list
  from standard input;
- `--input-format lines`, as above;
- `--input-format jsonl`: one JSON value per line, a sequent as
  `seq json` writes it, or a record with a `name`, a `mode` and the
  `sequent`;
- `--input-format problems`: the benchmark harness's problem files.

Paths are taken literally, relative to the current directory, with no
glob or `~`. A directory is walked in sorted order, following links,
for the files of the input format's extension (`.p` and `.json` by
default). A file in the batch is named by its path as given.

A file's kind is its extension's unless `--input-format` says
otherwise, for a single `prove` as well:

- `.p` is a problem of the LLTP library;
- `.spec` is a coverability problem in the format of the Mist tool;
- `.json` is a sequent in JSON;
- anything else is the text syntax.

An LLTP problem does not say whether it is intuitionistic (the library
keeps those under `ILL/`), so the flags say it:

```console
$ cat > problem.p
> % Status (intuit.) : Theorem
> fof(rule, axiom, !(a -o b)).
> fof(fact, axiom, a).
> fof(goal, conjecture, b * 1).
$ deduxit prove -i -q --file problem.p
provable (IMELL, intuitionistic, horn engine)
```

A `.spec` problem is a Petri net or a vector addition system, as the
coverability suites of software verification write it. It has
counters, rules that test and change them, an initial marking
(`x >= k` for at least `k` tokens), and the markings to cover, one line
each. deduxit reads it as a Horn program: the rules under `!` and the
initial tokens left of `⊢`. Covering a marking is a question of affine
mode, so a `.spec` file is decided in affine mode whatever the flags
say (`-i` and `--mix` keep their meaning). Here: can two processes be
in the critical section at once, for any number of processes and one
lock?

```console
$ cat > mutex.spec
> vars idle critical lock
> rules
>   idle >= 1, lock >= 1 -> idle' = idle - 1, lock' = lock - 1, critical' = critical + 1;
>   critical >= 1 -> critical' = critical - 1, idle' = idle + 1, lock' = lock + 1;
> init idle >= 1, lock = 1
> target critical >= 2
$ deduxit prove -i --affine -q --file mutex.spec
unprovable (IMELL, intuitionistic affine, horn engine): the state equation of the Petri net has no solution, so no firing of its clauses yields the goal's atoms: weighting each lock by 1 and critical by 1, no clause that can fire raises the weighted count of the atoms, and the goal asks it raised
```

The mode of each sequent is the flags' (else a settings file's
`batch.mode`), a problem file's column, or a JSON Lines record's.

On standard input, a format of many sequents is a stream: each line is
answered as soon as it is decided, so a program can ask, wait for the
answer and ask again.

## Write each proof to a file

With a drawn format, `--output DIR` writes each proved sequent's
derivation into the directory, named after the sequent with the
format's extension:

```console
$ cat > sequents.txt
> identity: A |- A
> modus ponens: A, A -o B |- B
$ deduxit prove --input-format lines --file sequents.txt --format svg --output drawings
identity: provable (MLL, classical, focus engine)
modus ponens: provable (MLL, classical, focus engine)
```

This writes `drawings/identity.svg` and `drawings/modus ponens.svg`. A
path keeps its directories: `ILL/01/X.p` becomes `DIR/ILL/01/X.p.svg`,
so the LLTP library's translations of one problem do not overwrite
each other. Rocq takes a file's name as its module's name, so a
certificate's name is made a Rocq identifier: every character but a
letter, digit or `_` is written `_`. `modus ponens` becomes
`DIR/modus_ponens.v`, and `ILL/01/X.p` becomes `DIR/ILL/entry_01/X_p.v`.

## JSON Lines

With `--format json` every answer is a JSON Lines record: the wire
level and the name first, then what `--format json` writes for one
sequent. With `--stats` it has `seconds` too, the time the sequent took,
which a single sequent's JSON leaves out to stay reproducible:

```console
$ echo '{"name": "pair", "mode": "intuitionistic", "sequent": "A, B |- A * B"}' | deduxit prove --input-format jsonl --format json
{"version":1,"name":"pair","deduxit":"…","verdict":"proved","checked":true,"fragment":"IMLL","mode":"intuitionistic","engine":"two-sided","statistics":{"nodes":1,"memo_hits":0,"memo_entries":1,"splits":1,"links":0,"tests":0,"copies":0,"forward_copies":0,"work":16},"sequent":{"terms":[{"D":0},{"D":1},{"V":0},{"V":1},{"⊗":[2,3]}],"roots":[0,1,4],"atoms":["A","B"],"antecedents":2},"nodes":[{"ax":[3,0]},{"ax":[4,1]},{"⊗":[2,0,1]}]}
```

- A sequent that could not be read or searched has
  `"verdict": "error"` with a `message`. Where the error is the
  library's, such as a parse error or a bound that refused the input,
  the record also has the library's `error` object, with its `code`,
  `kind` and `details`.
- A sequent the batch stopped before deciding it has
  `"verdict": "unknown"` with a `message`.

## The exit status of a batch

The exit status is the worst verdict: an error (2) before unknown (3)
before unprovable (1) before proved (0). The first batch above exits
with 2. A batch that holds no sequent is an error. Ctrl-C stops the
searches and ends the input, and the status is then unknown at best.

## Use the cores

By default the cores go across the sequents: each worker decides one
sequent on one thread, as `--deterministic` does. A batch with fewer
sequents than workers, or a stream on standard input, gives each
sequent every thread instead, as a single call does. `--cores across`
or `--cores within` chooses, and `--workers N` caps the sequents
decided at once.

Across the cores a batch answers the most sequents in a given time, and
each answer depends on its input alone. A sequent that only the pool
of a single call decides within its time limit stays unknown there. A
second pass over the unknown sequents with `--cores within` costs
little, since they are few:

```sh
deduxit prove -i --file problems/ > results.txt
grep ': unknown' results.txt | cut -d: -f1 > unknown.txt
deduxit prove -i --files-from unknown.txt --cores within > second.txt
```

## Memory and time limits

- `--timeout` is each sequent's time limit, its reading included.
  `--batch-timeout` is the whole batch's: when it passes, the running
  searches stop and the sequents not begun are unknown.
- Each search holds at most `--memory-limit`. As many searches run at
  once as `--batch-memory` holds, so a batch never holds more than that
  for its searches. By default it is half the memory the process may
  use: the machine's, or its control group's limit where one is set.
- `--isolate` decides each sequent in a child process of its own, so
  that one that exhausts the memory or the stack ends only itself.
