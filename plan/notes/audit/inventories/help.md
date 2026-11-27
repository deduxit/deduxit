# The command's help

| command | flags | lines of `-h` |
|---|---:|---:|
| `(top level)` | 2 | 42 |
| `prove` | 47 | 88 |
| `check` | 40 | 58 |
| `grade` | 19 | 37 |
| `eliminate` | 44 | 62 |
| `interact` | 31 | 65 |
| `interpolate` | 11 | 50 |
| `resolve` | 15 | 52 |
| `herbrand` | 10 | 45 |
| `seq` | 1 | 14 |
| `term` | 1 | 14 |
| `workspace` | 1 | 21 |
| `seq print` | 17 | 52 |
| `seq translate` | 11 | 48 |
| `seq normalize` | 11 | 48 |
| `seq json` | 9 | 33 |
| `seq fragment` | 6 | 30 |
| `term check` | 6 | 18 |
| `term reduce` | 9 | 23 |
| `term infer` | 6 | 18 |
| `term inhabit` | 9 | 44 |
| `term enumerate` | 11 | 48 |
| `workspace new` | 4 | 13 |
| `workspace add` | 4 | 13 |
| `workspace apply` | 7 | 18 |
| `workspace set` | 6 | 18 |
| `workspace run` | 4 | 15 |
| `workspace remove` | 1 | 10 |
| `workspace undo` | 1 | 9 |
| `workspace redo` | 1 | 9 |
| `workspace log` | 1 | 9 |
| `workspace show` | 3 | 12 |
| `workspace check` | 3 | 13 |
| `workspace operations` | 1 | 9 |

## The flags: 80

| flag | short | value | heading | commands |
|---|---|---|---|---|
| `--abbreviate` | `` | `<CHARS>` | Output | prove, check, eliminate |
| `--affine` | `-a` | `` | Logic | prove, check, grade, eliminate, interact |
| `--answer` | `` | `<PATH>` | Input | grade |
| `--assume` | `` | `<SEQUENT>` | Logic | grade, interact |
| `--at` | `` | `<NODE>` | Options | eliminate |
| `--by` | `` | `<TRANSLATION>` | Logic | seq translate |
| `--calculus` | `` | `<CALCULUS>` | Input | check, eliminate, term check, term reduce, term infer |
| `--calculus` | `` | `<CALCULUS>` | Logic | grade, interact |
| `--checks` | `` | `` | Options | workspace run |
| `--classical-sides` | `` | `<SIDES>` | Output | prove, check, eliminate |
| `--clause-limit` | `` | `<N>` | Limits | resolve |
| `--clauses` | `` | `<FORM>` | Input | resolve |
| `--compact` | `` | `<WHEN>` | Output | prove, check, eliminate |
| `--copies` | `` | `<N>` | Search | prove, interact |
| `--cyclic` | `` | `` | Logic | prove, check, grade, eliminate, interact |
| `--cyclic` | `` | `` | Options | seq print, seq json |
| `--depth` | `` | `<N>` | Limits | herbrand |
| `--depth` | `` | `<N>` | Search | term enumerate |
| `--deterministic` | `` | `` | Search | prove, interact |
| `--dimacs` | `` | `<PATH>` | Input | check, eliminate, resolve |
| `--drawings-limit` | `` | `<SIZE>` | Options | eliminate |
| `--empty-antecedents` | `` | `` | Logic | prove, check, grade, eliminate, interact |
| `--engine` | `` | `<ENGINE>` | Search | prove |
| `--equality` | `` | `<WAY>` | Search | resolve |
| `--file` | `-f` | `<PATH>` | Input | prove, grade, interact, interpolate, resolve, herbrand, seq print, seq translate, seq normalize, seq json, seq fragment, term check, term reduce, term infer, term inhabit, term enumerate |
| `--file` | `-f` | `<PATH>` | Options | check, eliminate |
| `--first` | `` | `<POSITIONS>` | Input | interpolate |
| `--font` | `` | `<FONT>` | Output | prove, check, eliminate, interact, seq print |
| `--form-limit` | `` | `<N>` | Limits | seq normalize |
| `--format` | `` | `<FORMAT>` | Options | seq print, workspace show |
| `--format` | `` | `<FORMAT>` | Output | prove, check, grade, eliminate, interpolate, resolve, herbrand, seq translate, seq normalize, term check, term reduce, term infer, term inhabit, term enumerate |
| `--fragment` | `` | `<FRAGMENT>` | Search | prove |
| `--help` | `-h` | `` | Options | (top level), prove, check, grade, eliminate, interact, interpolate, resolve, herbrand, seq, term, workspace, seq print, seq translate, seq normalize, seq json, seq fragment, term check, term reduce, term infer, term inhabit, term enumerate, workspace new, workspace add, workspace apply, workspace set, workspace run, workspace remove, workspace undo, workspace redo, workspace log, workspace show, workspace check, workspace operations |
| `--ill-syntax` | `` | `<SYNTAX>` | Input | prove, grade, interact, interpolate, resolve, herbrand, seq print, seq translate, seq normalize, seq json, seq fragment, term inhabit, term enumerate |
| `--input-format` | `` | `<FORMAT>` | Input | prove, grade, interact, interpolate, resolve, herbrand, seq print, seq translate, seq normalize, seq json, seq fragment, term inhabit, term enumerate |
| `--intuitionistic` | `-i` | `` | Logic | prove, check, grade, eliminate, interact |
| `--intuitionistic` | `-i` | `` | Options | seq print, seq json, seq fragment |
| `--invertible` | `` | `` | Options | interact |
| `--jobs` | `-j` | `<N>` | Search | prove, interact |
| `--kernel` | `` | `<KERNEL>` | Output | prove, check, eliminate, interact |
| `--lambda` | `` | `` | Output | prove, check, eliminate |
| `--lambda-notation` | `` | `<NOTATION>` | Output | prove, check, eliminate, interact |
| `--lambda-types` | `` | `` | Output | prove, check, eliminate, interact |
| `--lambek` | `` | `` | Logic | prove, check, grade, eliminate, interact |
| `--lambek` | `` | `` | Options | seq print |
| `--layout` | `` | `<LAYOUT>` | Output | check, eliminate |
| `--lemma` | `` | `<NAME>` | Output | prove, check, eliminate, interact |
| `--linear` | `` | `` | Output | prove |
| `--logic` | `` | `<LOGIC>` | Logic | prove, check, grade, eliminate, interact, interpolate, seq print |
| `--logic` | `` | `<LOGIC>` | Options | workspace new, workspace add |
| `--max-terms` | `` | `<N>` | Search | term enumerate |
| `--memory-limit` | `` | `<SIZE>` | Limits | prove, check, grade, eliminate, interact, interpolate, resolve, herbrand, seq translate, seq normalize, term inhabit, term enumerate, workspace apply, workspace set |
| `--memory-limit` | `` | `<SIZE>` | Options | seq print, seq json, seq fragment |
| `--method` | `` | `<METHOD>` | Logic | prove |
| `--method` | `` | `<METHOD>` | Search | resolve |
| `--mix` | `` | `` | Logic | prove, check, grade, eliminate, interact |
| `--mode` | `` | `<MODE>` | Options | workspace new, workspace add |
| `--net` | `` | `` | Output | prove, check, eliminate |
| `--no-check` | `` | `` | Search | prove |
| `--no-run` | `` | `` | Options | workspace apply |
| `--no-verdict` | `` | `` | Output | prove, check, eliminate |
| `--node-limit` | `` | `<N>` | Options | eliminate |
| `--node-limit` | `` | `<N>` | Search | term reduce |
| `--optimize` | `` | `` | Options | seq json |
| `--output` | `-o` | `<PATH>` | Options | seq print, seq json, workspace show |
| `--output` | `-o` | `<PATH>` | Output | prove, check, eliminate, interpolate, resolve, herbrand, seq translate, seq normalize, term check, term reduce, term infer, term inhabit, term enumerate |
| `--prelude` | `` | `<TEXT>` | Output | prove, check, eliminate, interact |
| `--proof-format` | `` | `<FORMAT>` | Input | check, eliminate |
| `--quiet` | `-q` | `` | Output | prove, check, eliminate |
| `--screens` | `` | `<N>` | Output | prove, check, eliminate |
| `--sequent` | `` | `<SEQUENT>` | Input | check, eliminate |
| `--set` | `` | `<KEY=VALUE>` | Options | workspace apply, workspace set |
| `--settings` | `` | `<PATH>` | Options | prove, check, grade, eliminate, interact, interpolate, resolve, herbrand, seq print, seq translate, seq normalize, term check, term reduce, term infer, term inhabit, term enumerate, workspace apply, workspace set, workspace run, workspace check |
| `--standalone` | `` | `` | Options | seq print |
| `--standalone` | `` | `` | Output | prove, check, eliminate |
| `--state` | `` | `<PATH>` | Input | interact |
| `--stats` | `` | `` | Output | prove |
| `--step-limit` | `` | `<N>` | Limits | resolve |
| `--steps` | `` | `<N>` | Options | eliminate |
| `--steps` | `` | `<N>` | Search | term reduce |
| `--strategy` | `` | `<STRATEGY>` | Search | term reduce |
| `--structural` | `` | `<RULES>` | Output | prove, interact |
| `--style` | `` | `<KEY=VALUE>` | Output | prove, check, eliminate, interact, seq print |
| `--style-file` | `` | `<PATH>` | Output | prove, check, eliminate, interact, seq print |
| `--system` | `` | `<SYSTEM>` | Logic | prove, check, grade, eliminate |
| `--system` | `` | `<SYSTEM>` | Options | workspace new, workspace add |
| `--szs` | `` | `` | Output | prove |
| `--tableau` | `` | `` | Input | check, eliminate |
| `--timeout` | `` | `<DURATION>` | Limits | prove, check, grade, eliminate, interact, interpolate, resolve, herbrand, seq translate, seq normalize, term inhabit, term enumerate, workspace apply, workspace set, workspace run, workspace check |
| `--to` | `` | `<FORM>` | Logic | seq normalize |
| `--to` | `` | `<LOGIC>` | Logic | seq translate |
| `--translation` | `` | `<TRANSLATION>` | Logic | prove, interact, seq print |
| `--tree` | `` | `<WHEN>` | Output | prove, check, eliminate |
| `--version` | `-V` | `` | Options | (top level) |
| `--work-limit` | `` | `<N>` | Limits | workspace apply, workspace set |
| `--world` | `` | `<N>` | Input | check, eliminate |

## Flags whose short form, value or heading differs between commands: 16

- `--calculus`
- `--cyclic`
- `--depth`
- `--file`
- `--format`
- `--intuitionistic`
- `--lambek`
- `--logic`
- `--memory-limit`
- `--method`
- `--node-limit`
- `--output`
- `--standalone`
- `--steps`
- `--system`
- `--to`

## One-letter forms that stand for more than one flag

