# Avoid the common mistakes

These are the mistakes people make first, with what the command says
and what to write instead. Each error exits with status 2.

## Forgetting `prove`

The first word after `deduxit` is a command. To decide a sequent, the
command is `prove`:

```console
$ deduxit "A |- A"
error: unrecognized subcommand 'A |- A'

  tip: to decide a sequent, run deduxit prove "A |- A"

Usage: deduxit <COMMAND>

For more information, try '--help'.
```

## Not quoting the sequent

The shell splits an unquoted sequent into words. `-o` then reads as the
flag `--output`, and a comma ends the sequent. Put the sequent in
quotes; the tip shows the command quoted:

```console
$ deduxit prove A -o B
error: cannot parse the sequent
  A
   ^ unexpected end of input, expected a connective, `,` or `|-`
  quote the sequent: deduxit prove "|- A -o B"
$ deduxit prove A, B
error: unexpected argument 'B' found

  tip: quote the sequent: deduxit prove "|- A, B"

Usage: deduxit prove [OPTIONS] [SEQUENT]

For more information, try '--help'.
```

Use single quotes around a sequent with `!`: in bash and zsh, `!` in
double quotes recalls a command from the history. The tip does so:

```console
$ deduxit prove '!A,' B
error: unexpected argument 'B' found

  tip: quote the sequent: deduxit prove '|- !A, B'

Usage: deduxit prove [OPTIONS] [SEQUENT]

For more information, try '--help'.
```

## Writing `->` in linear logic

Linear implication is `-o` (or `⊸`). `->` is the implication of
ordinary logic, which `--logic` reads:

```console
$ deduxit prove "A -> B |- A"
error: cannot parse the sequent
  A -> B |- A
     ^ unexpected ">", expected the `o` of `-o`
  -> is ordinary logic's arrow: write -o, or add --logic classical
```

[Decide ordinary logic](ordinary-logic.md) has the syntax of `--logic`.

## Giving a file of named sequents as one sequent

A file of `NAME: SEQUENT` lines is a batch. Without `--input-format
lines`, `prove` reads the file as one sequent:

```console
$ printf 'id: A |- A\nmp: A, A -o B |- B\n' > s.txt
$ deduxit prove --file s.txt
error: cannot parse the sequent
  id: A |- A
    ^ unexpected ":" at line 1, character 3, expected a connective, `,` or `|-`
  to read one sequent per line, add --input-format lines
$ deduxit prove --input-format lines --file s.txt
id: provable (MLL, classical, focus engine)
mp: provable (MLL, classical, focus engine)
```

[Decide many sequents at once](batch.md) has the other batch formats.

## Using classical symbols under `-i`

Intuitionistic linear logic has no negation, no `par`, no `?` and no
`bot`. Write the formula on the other side of `|-`, and `~A par B` as `A
-o B`; or give `--ill-syntax one-sided` to read the classical syntax:

```console
$ deduxit prove -i "|- ~A par B"
error: cannot parse the sequent
  |- ~A par B
     ^ unexpected "~"
  intuitionistic linear logic has no negation: write a negated formula on the other side of |- (A |- B, not |- ~A, B), and ~A par B as A -o B; --ill-syntax one-sided reads the classical syntax
```

An intuitionistic sequent also has exactly one formula right of `|-`:

```console
$ deduxit prove -i "A |- B, C"
error: not an intuitionistic sequent: an intuitionistic sequent has exactly one formula right of ⊢, not 2: write the hypotheses left of ⊢
```

## Naming a format by its file extension

`--format` takes `latex`, `typst`, `svg`, `png`, `pdf`, `rocq`, `json`
and `text`; `tex` is read as `latex`. With `--output FILE`, the
extension of `FILE` picks the format, so `--format` is not needed.

## Expecting a hypothesis to be optional

In linear logic every hypothesis is used exactly once, so `A, B |- A` is
unprovable. Affine mode, `-a`, lets a hypothesis go unused; the
refutation names it where that is the problem
([Getting started](introduction.md#your-first-unprovable-sequent)).
`!A` lets `A` be used any number of times.
