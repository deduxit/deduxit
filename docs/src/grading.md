# Grade an answer

`deduxit grade` checks an answer to a problem: a proof or a refutation
that someone else wrote, a student or a language model. You give the
problem's sequent and its logic, as to `prove`, and the answer on
standard input or with `--answer FILE`. The exit status is the verdict:
0 correct, 1 incorrect, 3 unknown (a limit stopped the check), 2 an
error in the problem or the flags.

## A proof step by step

The most natural answer is a script of `interact`'s commands
([Prove step by step](interactive.md)). It is correct when it ends with
the proof finished:

```console
$ printf 'apply 0 2 * 0\napply 1 1 ax\napply 2 1 ax\n' | deduxit grade "A, B |- A * B"
correct: the script's 3 commands finish the proof, which checks
```

Every step must be the answer's own, so `close`, which lets the search
finish a goal, is refused, and so are `save` and `load`. An incorrect
script names the first line that failed:

```console
$ printf 'apply 0 2 * 0\nclose\n' | deduxit grade "A, B |- A * B"; echo "exit $?"
incorrect: the script's 1 command leaves the proof unfinished; the first that failed: `close` is not part of an answer: every step must be the answer's own, and an answer reads and writes no file (line 2)
exit 1
```

Blank lines, `#` comments and the fences of a Markdown code block are
passed over, and so are the commands that only show the session
(`goals`, `rules`, `show`, `proof`). With `--logic` the script is a
session of LK or LJ, and with `--calculus` one in that calculus.

## A proof or a refutation as JSON

What `prove --format json` writes is an answer too, and so is a
session's `proof FILE`. It must be of the problem's sequent, in its
logic:

```console
$ deduxit prove --format json "A |- B" > answer.json
$ deduxit grade "A |- B" --answer answer.json
correct: valid refutation of A ⊢ B (classical): A on the left has no A on the right to meet in an axiom
$ deduxit grade "A |- A" --answer answer.json; echo "exit $?"
incorrect: the refutation is of ⊢ ~A, B in classical, and the problem is ⊢ ~A, A in classical
exit 1
```

In ordinary and modal logic a Kripke model answers that a sequent is
not valid: the model must refute it. `prove` writes one under
`countermodel` (for modal logic, under `countermodel.model`):

```console
$ echo '{"worlds": [{"atoms": [], "arrows": [1]}, {"atoms": ["a"]}], "world": 0}' | deduxit grade --logic intuitionistic "|- a \/ ~a"
correct: the model does not satisfy ⊢ a ∨ ¬a at every world: world 0 forces each formula left of ⊢ and not the one right of it
```

## A tableau

In classical logic a signed tableau written by hand, as
[Check a proof or a refutation](check.md) describes it, is an answer.
Its first line starts with a line number:

```console
$ printf '1 T p\n2 F q\nopen\n' | deduxit grade --logic classical "p |- q"
correct: open tableau of p ⊢ q: every step follows the rules, every branch is finished, and the branch ending at line 2 is open, so the sequent is not valid in classical logic: a checked countermodel
```

## The verdict as JSON

`--format json` writes the verdict as one object: `verdict` (`correct`,
`incorrect` or `unknown`), `answer` (the kind read: `script`, `proof`,
`refutation`, `outcome`, `countermodel`, `derivation`, `tableau`, or
`none` for one that does not read), `message`, and for a script `line`,
the first line that failed.

```console
$ echo 'not an answer' | deduxit grade --format json "A |- A"
{"version":1,"verdict":"incorrect","answer":"script","message":"the script's 1 command leaves the proof unfinished; the first that failed: unknown command \"not\"; `help` lists them","line":1}
```
