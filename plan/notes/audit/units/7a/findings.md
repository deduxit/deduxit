# Unit 7a: findings

Read in full: 11 files (`cli/src/argument_parsing.rs`, `lib.rs`, `main.rs`,
`settings.rs`, `io.rs`, `limit.rs`, `style.rs`, `seq.rs`, `prove.rs`,
`check.rs`, `batch.rs`), and beside them `target/audit/help.md`,
`help/top.txt`, the first 120 lines of `help/prove.txt` (the rest is the
same doc comments read in `argument_parsing.rs`), the headings of
`help/check.txt`, `grade.txt`, `interact.txt`, `seq-print.txt`, the manual's
`commands.md`, `batch.md`, `check.md`, `options.md`, `settings.md`,
`SUMMARY.md` and README's sections up to "Where to go next". Not read in
full: none of the unit's source files. `.claude/rules/cli.md` (1 559 lines)
was read in the parts on these files (Layout, Extension points, Decisions).

Runs (capped, cores 13-15) used as witnesses are quoted with their output.

## F-7a-01

- dimension: api
- criterion: D1
- place: cli/src/argument_parsing.rs:2629
- kind: API break
- cost: large
- candidate: no

**Claim.** Every command that reads a sequent picks linear logic when no
flag names a logic, through five different mechanisms; D1 asks for one
required selector.

**Witness.** How each command chooses its logic today:

| command | no flag | linear | ordinary | modal | rules |
|---|---|---|---|---|---|
| `prove` | classical linear logic (`ModeArgs::mode`, argument_parsing.rs:2629-2649; prove.rs:315) | `-i -a --mix --cyclic --lambek --empty-antecedents` | `--logic L` (:2484) | `--system S` (:697) | none |
| `interact` | classical linear | the mode flags (:1229) | `--logic` (:1233) | none | `--calculus` (:1222) |
| `grade` | classical linear (help :171 "in linear logic by default") | the mode flags (:1474) | `--logic` (:1476) | `--system` (:1480) | `--calculus` (:1484) |
| `check`, `eliminate` | the file's `mode`; a proof without one takes the flags', classical linear by default (check.rs:276-285) | mode flags | `--logic`, only for a Kripke model (:1345) | `--system`, only for a model (:1359) | `--calculus` (:1423) |
| `seq print` | linear | `-i --cyclic --lambek` (:1576-1586) | `--logic`, which prints the linear image (seq.rs:60-65) | none | none |
| `seq json`, `seq fragment` | linear only (:1681-1722) | `-i`, `--cyclic` | none | none | none |
| `interpolate` | classical ordinary logic (`default_value_t = LogicArg::Classical`, :2289) | | `--logic` | | |
| `workspace new/add` | linear, `--mode` a `String` defaulting to `"classical"` (:541-549) | `--mode MODE` | `--logic` | `--system` | |
| batch | the flags', else the settings' `batch.mode`, else a record's or a problem line's `mode` (batch.rs:693) | | `--logic` for every entry | refused (prove.rs:300-302) | |
| settings file | `batch.mode` (linear); `ordinary.logic` is never read (settings.md:53-57) | | | | |

```rust
    pub(crate) fn mode(&self) -> Mode {
        let mut mode = if self.intuitionistic {
            Mode::INTUITIONISTIC
        } else if self.cyclic {
            Mode::CYCLIC
        } else if self.lambek {
            Mode::LAMBEK
        } else {
            Mode::CLASSICAL
        };
```

README.md:78: "Without `--logic`, `prove` reads linear logic". The
linear mode is spelt three ways: boolean flags, `workspace --mode MODE`
(argument_parsing.rs:543-549, a free `String`), and the batch's mode names
(`affine-mix`, `lambek-star`, :2431-2435; `--entry-mode`, :1159).

**Change.** A required `ArgGroup` over the present selectors, the form
clap gives for "exactly one of these names the logic":

```rust
#[derive(Args)]
#[group(id = "suite", required = true, multiple = false)]
struct Suite {
    /// Ordinary logic: classical, intuitionistic or minimal
    #[arg(long, value_enum)] logic: Option<LogicArg>,
    /// Linear logic in this mode: classical, affine, mix, affine-mix,
    /// intuitionistic, intuitionistic-affine, cyclic, lambek, lambek-star
    #[arg(long, value_parser = mode_name)] linear: Option<Mode>,
    /// Modal logic in this system, such as S4
    #[arg(long, value_parser = parse_system)] system: Option<System>,
    /// A calculus given as a rule file, or a built-in one
    #[arg(long)] calculus: Option<PathBuf>,
}
```

`--linear MODE` reads the library's `Mode` names (`Mode::from_str`, which
the batch already uses, batch.rs:324), so the flag, `workspace --mode`, a
JSON Lines record and a problem line spell a mode one way; `-i`, `-a`,
`--mix`, `--cyclic`, `--lambek` and `--empty-antecedents` go (or stay one
release as shorthands that are members of the group). `prove`'s present
`--linear` (show the image's proof, :757) is renamed `--image`.
`interpolate --logic` loses its default. `check` and `eliminate` read the
suite from the file and refuse a proof without `mode` unless `--linear`
names it. `seq print`, `seq json` and `seq fragment` take the group
(`seq fragment` only `--linear`). Commands of one logic (`resolve`,
`herbrand`, `seq normalize`, `seq translate`, `term inhabit`, `term
enumerate`) name it by their own name; the leader decides whether they
take `--logic classical` too. Without a selector clap answers with its
required-group error, and `hint.rs` adds the tip:

```text
error: the following required arguments were not provided:
  <--logic <LOGIC>|--linear <MODE>|--system <SYSTEM>|--calculus <CALCULUS>>

Usage: deduxit prove <--logic <LOGIC>|--linear <MODE>|--system <SYSTEM>|--calculus <CALCULUS>> [SEQUENT]

  tip: name the logic: --logic classical for `a -> b |- b`, --linear classical for `A -o B |- B`, --system S4 for `[]p -> p`
```

**Value.** A user of the command never gets a verdict of linear logic for
a sequent they meant in ordinary logic; one spelling of a mode across
flags, files and records; the first parse error of `a -> b |- b` today
(`-> is ordinary logic's arrow: write -o, or add --logic classical`)
becomes a missing-flag error before any parsing.

**Net.** README's and the manual's examples and the guide's
(`cli/tests/readme.rs`), the behaviour lock (`cli/tests/lock.rs`), the
settings tests; no checker, search or certificate is touched. The
changelog's "Changed" and a migration line per command (B3).

## F-7a-02

- dimension: idiom
- criterion: L9
- place: cli/src/prove.rs:748
- kind: refactor
- cost: small
- candidate: no

**Claim.** The linear mode flags are listed by hand in five places, and
three lists miss `--cyclic` and `--lambek`, so `prove` and `interact`
run with a flag silently dropped and a settings file beats a flag given.

**Witness.**

```rust
    if args.logic.logic.is_some() && (mode.intuitionistic || mode.affine || mode.mix) {
        bail!("the logic's translation decides the mode: leave out -i, -a and --mix");
```

(prove.rs:748-749; the same in `interact/ordinary.rs:85-86`), and
settings.rs:246-251:

```rust
        if ["intuitionistic", "affine", "mix"]
            .map(|id| apply.set(id))
            .contains(&true)
        {
            settings.batch.mode = self.mode.mode();
        }
```

`ModeArgs::given` (argument_parsing.rs:2652) and `takes_logic` (:650)
list all of them; `grade.rs:100-105` uses `given()`. Runs:

```text
$ deduxit prove --logic classical --cyclic 'a |- a'          # exit 0, --cyclic ignored
valid (classical logic by the affine translation into affine MALL, focus engine)
$ deduxit prove --logic intuitionistic --lambek -q 'a |- a'   # exit 0, ignored
$ deduxit grade --logic classical --cyclic 'a |- a' --answer /dev/null
error: --logic, --system and --calculus name the problem's logic: leave out the mode flags
$ cat s.json; cat l.txt
{"batch":{"mode":"affine"}}
x: A, B |- A
$ deduxit prove --settings s.json --cyclic --input-format lines -f l.txt   # exit 0
x: provable (MLL, classical affine, focus engine)
$ deduxit prove --cyclic --input-format lines -f l.txt                     # exit 1
x: unprovable (MLL, cyclic, net engine): B on the left has no B on the right to meet in an axiom
```

The help (:852) and settings.md:51-52 promise "A mode flag given on the
command line overrides it". cli.md:121-122 records the refusal of `-i`,
`-a` and `--mix` because "the image's mode is the translation's", a
reason that covers `--cyclic` and `--lambek` as well.

**Change.** Use `mode.given()` in `ordinary_mode` and in
`interact/ordinary.rs`, and ask every mode id in settings.rs (or a
`ModeArgs::IDS` const that all five read); F-7a-01 removes the lists
altogether.

**Value.** A user of the command gets an error, not a verdict in another
logic than the one asked for; a flag beats the settings file as the help
says.

**Net.** `each_flag_overrides_its_key_in_a_file` gains `--cyclic` and
`--lambek` rows; the behaviour lock. No checker or search.

## F-7a-03

- dimension: api
- criterion: L8
- place: cli/src/argument_parsing.rs:2079
- kind: API break
- cost: medium
- candidate: no

**Claim.** Flag values are a second spelling of the library's names, held
in 18 hand-made mirror enums, so a name the command writes in JSON or a
settings file is refused as a flag value.

**Witness.** `lambda::Calculus::NAMES` (core/src/lambda/mod.rs:345-355):

```rust
    /// The names, as the JSON form and the command write them.
    pub const NAMES: &'static [&'static str] = &[
        "linear",
        …
        "system_f",
```

```text
$ deduxit term check --calculus system-f '/\X. \x:X. x' --format json
{"version":31,"calculus":"system_f",…}
$ deduxit term check --calculus system_f '/\X. \x:X. x'
error: invalid value 'system_f' for '--calculus <CALCULUS>'
  [possible values: simple, linear, affine, untyped, untyped-linear, untyped-affine, system-f, system-f-linear, system-f-affine]
```

The same split: `--strategy normal-order` against `Strategy::NAMES`
`normal_order` (core/src/lambda/reduce.rs:119); `--fragment mll-units`
against the settings file's `search.fragment` `"MLL with units"` (the
test table at settings.rs:844 has `--fragment mall` beside `"MALL"`).
argument_parsing.rs has 28 `ValueEnum`s and 18 `impl From<XArg> for
deduxit::…` mirrors (`ScheduleArg`, `FormArg`, `CalculusArg`,
`StrategyArg`, `NegativeArg`, `IllSyntaxArg`, `LogicArg`, `MethodArg`,
`TranslationArg`, `LayoutArg`, `ClassicalSidesArg`, `NotationArg`,
`StructuralArg`, `CompactArg`, `FontArg`, `KernelArg`, `FragmentArg`,
`BiasArg`); `--engine` alone reads the library's names (`engine_name`,
:3349), as cli.md's "never re-implements what core computes" asks.

**Change.** Read every value through the library's `FromStr` and `NAMES`
(a `PossibleValuesParser` as `engine_name` does), keeping the kebab
spelling as a clap alias for one release; delete the mirrors. Where the
library itself mixes `snake_case` and kebab names, the core unit decides
the one spelling and the command follows it.

**Value.** A user of the command can paste a value from the JSON, a
settings file or the library's docs into a flag; a new library variant
needs no CLI edit (a reader of the code, about 400 lines fewer).

**Net.** The settings tests, README's and the manual's examples, the
guide's name check in `readme.rs`. No checker or search.

## F-7a-04

- dimension: api
- criterion: D4
- place: cli/src/seq.rs:60
- kind: API break
- cost: medium
- candidate: no

**Claim.** The `seq` commands and the batch serve linear logic only:
`seq print --logic` prints the linear image instead of the sequent, `seq
json` and `seq fragment` take no other suite, nothing prints a modal
sequent, and a batch refuses `--system`.

**Witness.**

```rust
            let (sequent, mode) = match settings::ordinary(&settings, logic, false) {
                Some(options) => {
                    let image = input.image(&options, &settings.limits)?.1;
                    let mode = image.mode();
                    (image.sequent().clone(), mode)
```

```text
$ deduxit seq print --logic classical 'a -> b |- b'
⊢ a & ~b, b
$ deduxit seq print 'a -> b |- b'
error: cannot parse the sequent … -> is ordinary logic's arrow: write -o, or add --logic classical
$ deduxit seq print '[]p -> p'
error: cannot parse the sequent
  []p -> p
  ^ unexpected "[", expected a formula or `|-`
$ deduxit seq json --input-format text 'a /\ b'
error: cannot parse the sequent …
```

`Json` and `Fragment` (argument_parsing.rs:1679-1722) have no `--logic` and
`after_help = SYNTAX` (linear logic's alone); prove.rs:300-302: `--system
decides one sequent: give it as the argument, or one file with --file`.
`seq json` is also a format as a subcommand where every other command
writes JSON with `--format json` (L8, "no catch-all subcommand").

**Change.** `seq print` prints the sequent of whichever suite the selector
of F-7a-01 names (text, LaTeX, Typst, SVG, PNG, PDF and `--format json`),
and the image only under a flag of its own (`--image`); `seq json` becomes
`seq print --format json` for every suite; `seq fragment` stays linear
logic's. The batch takes `--system`, and a JSON Lines record may name its
suite.

**Value.** A user of ordinary or modal logic can print, typeset and
convert a sequent; a front end gets every suite's JSON from one command.

**Net.** README's and the manual's examples, the behaviour lock. No
checker or search.

## F-7a-05

- dimension: api
- criterion: L8
- place: cli/src/argument_parsing.rs:1791
- kind: API break
- cost: small
- candidate: no

**Claim.** One flag name means two things under different subcommands,
and one thing has three names.

**Witness.**
- `--to`: `seq translate --to <LOGIC>` (:1790-1791) against `seq
  normalize --to <FORM>` (:1801-1802).
- `--calculus`: `check`, `grade`, `interact` take a rule file or a
  built-in calculus such as `LK` or `NJ` (:1423, :1484, :1222);
  `term check|reduce|infer` take a lambda calculus such as `system-f`
  (:2057, :2152, :2202).
- A translation is `prove --translation cbn` (:2489, the embedding),
  `seq translate --by kolmogorov` (:1784, a negative translation) and
  `workspace apply … --set translation=glivenko` (:441).

**Change.** `seq normalize --form FORM`; `term … --lambda-calculus` (or
`--types simple|linear|…`); `seq translate --translation`, which matches
the workspace's key.

**Value.** A user of the command learns one meaning per flag (clig's
"the same flag means the same thing under every subcommand").

**Net.** README's and the manual's examples, the guide's name check.

## F-7a-06

- dimension: text
- criterion: L2
- place: cli/src/argument_parsing.rs:699
- kind: text
- cost: medium
- candidate: no

**Claim.** No `--help` has an example or a link to the manual, and the
long help of the most used flags is a wall: `prove --help` is 538 lines,
`--method`'s help one paragraph of about 400 words.

**Witness.** `grep -il example target/audit/help/*.txt` and `grep -l
https target/audit/help/*.txt` find nothing. `help/prove.txt:76` is
`--method`'s help, a single line of 3 400 characters that describes every
method again although each `MethodArg` variant has its own line
(:2517-2546). The `SequentInput` doc shared by every command says "(but
not by `interact`)" in `prove`'s and `seq print`'s help
(`help/prove.txt:15`).

**Change.** `after_long_help` on each command with two or three runnable
examples (the README test can run them) and the manual's URL of the
command's chapter; `--method`'s long help cut to when to pick which, the
rest in the variants' lines and the manual.

**Value.** A user at a terminal finds the shortest working command and
where to read more (clig: "lead with examples").

**Net.** `readme.rs` checks the guide's flags against `--help`;
`short_help_keeps_to_the_common_flags`.

## F-7a-07

- dimension: api
- criterion: L6
- place: cli/src/io.rs:906
- kind: API break
- cost: small
- candidate: no

**Claim.** `--output -` writes a file named `-`, while `--file -` reads
standard input.

**Witness.**

```rust
        let (sink, rename): (Box<dyn Write>, _) = match path {
            None => (Box::new(Stdout::lock()), None),
```

```text
$ deduxit prove -o - 'A |- A'; echo $?; ls
0
-
$ cat ./-
provable (MLL, classical, focus engine)
─────── ax
⊢ ~A, A
```

**Change.** Take `Some("-")` as standard output in `Output::open` (and in
`output_can_be_made`), and say so in `--output`'s help.

**Value.** A user of the command can write `-o -` in a script that
builds its arguments, as clig asks ("`-` for standard input and
output").

**Net.** A case in `cli/tests/cli.rs`; nothing else changes.

## F-7a-08

- dimension: text
- criterion: T1
- place: cli/src/argument_parsing.rs:2672
- kind: text
- cost: small
- candidate: no

**Claim.** `--output`'s help says the file is "always in JSON", which is
false.

**Witness.**

```rust
    /// The file is made only when it holds a derivation or a net, and
    /// always in JSON. Otherwise the verdict goes to standard error.
```

```text
$ deduxit prove -o out.tex 'A |- A' && head -1 out.tex
% provable (MLL, classical, focus engine)
```

**Change.** "The file is made when it holds a derivation or a net, and
with `--format json` always; otherwise the verdict goes to standard
error."

**Value.** A user of the command reads what the flag does.

**Net.** `readme.rs` (help text is not pinned elsewhere).

## F-7a-09

- dimension: text
- criterion: T1
- place: cli/src/argument_parsing.rs:2469
- kind: text
- cost: small
- candidate: no

**Claim.** The help of `--logic`, of its values and of `--input-format
tptp` says "propositional" although `--logic` decides first-order
sequents and TPTP problems with quantifiers.

**Witness.**

```rust
    /// The input is a formula or a sequent of propositional logic, in the
```

(:2469); `/// Classical propositional logic, derivations in LK` and
`/// Intuitionistic propositional logic, derivations in LJ` (:2496,
:2498); `/// A problem of ordinary propositional logic in TPTP syntax`
(:2427). `--method`'s help on the same page describes grounding and the
connection prover for first-order sequents.

**Change.** "a formula or a sequent of ordinary logic, propositional or
first-order"; "Classical logic, derivations in LK"; "A problem of
ordinary logic in TPTP syntax".

**Value.** A user of first-order logic learns from `--help` that the
command takes their problem.

**Net.** None needed beyond `readme.rs`.

## F-7a-10

- dimension: text
- criterion: D4
- place: cli/src/argument_parsing.rs:79
- kind: text
- cost: small
- candidate: no

**Claim.** The command's help and the manual's first how-to give linear
logic the place of honour: the syntax shown under every command is
linear logic's, ordinary logic's follows as "With --logic", modal logic's
is nowhere, and `prove`'s description speaks of fragments and engines
alone.

**Witness.** `const SYNTAX: &str = linear_syntax!();` (:79) and
`ORDINARY` = linear syntax then "With --logic the input is ordinary
logic" (:53-66), shown under the top-level help (:101); `prove`: "The
command finds the sequent's fragment (MLL, MALL, …) and picks the engine
for it" (:113-114); `grade`: "in linear logic by default" (:171).
docs/src/commands.md is titled "Decide a sequent" and is linear logic's
alone (:140 "By default a sequent is read in classical linear logic"),
while ordinary and modal logic have "Decide ordinary logic" and "Decide
modal logic" further down `SUMMARY.md` (:8, :22, :23).

**Change.** A syntax block per suite in the page's order (ordinary,
linear, modal), each under the selector that reads it; `prove`'s
description "Decide a sequent in the logic the flags name, and print its
proof or why there is none"; `commands.md` becomes "Decide a sequent of
linear logic" beside the others, or a short chapter that leads to all
three. This follows F-7a-01's change.

**Value.** A user who opens `--help` or the manual sees every suite as a
peer, as the author's rule asks.

**Net.** `readme.rs`.

## F-7a-11

- dimension: api
- criterion: L3
- place: cli/Cargo.toml:46
- kind: build
- cost: medium
- candidate: no

**Claim.** The command has eleven subcommands and 80 flags but ships no
man page and no shell completions.

**Witness.** `[dependencies]` of `cli/Cargo.toml` has neither
`clap_mangen` nor `clap_complete`; no file under `docs/src` or
`.claude/rules/cli.md` mentions completions or a man page;
`target/audit/help.md` counts 80 flags.

**Change.** Generate `deduxit.1` (and one page per subcommand) with
`clap_mangen` and completions for bash, zsh, fish and PowerShell with
`clap_complete`, from a build step or a hidden `deduxit completions
SHELL`; put them in the release archive and the `.deb`/`.rpm` packages.

**Value.** A user of the command gets `man deduxit` and tab completion of
the flags and their values.

**Net.** The `package` and `deny` flake checks; a smoke test that the
generator runs. Two new dependencies (MIT/Apache-2.0) to weigh under B8.

## F-7a-12

- dimension: architecture
- criterion: E6
- place: cli/src/prove.rs:296
- kind: refactor
- cost: large
- candidate: no

**Claim.** The decision of one sequent is written twice, in `proved`
(382 lines) and in the batch's `Shared::decide`, and both interleave the
suites through `Option`s rather than choosing the suite once.

**Witness.** The same steps in the same order:

| step | prove.rs | batch.rs |
|---|---|---|
| a native method before the embedding | :336-342 `options.method_for(&sequent) != Method::Embedding` | :743-748 |
| the model search first on one thread | :360-388 `models_beside` / `model_first` | :752-761 |
| the image within a bound when solved | :404-414 `ordinary::bounded`, `translated` | :766-779 |
| the search | :498-505 `searched` | :806-823 |
| the embedding's evidence | :548-555 `ordinary::evidence` | :834-843 |
| status and verdict line | :560-564, :622-640 | :847-867 |

The batch's two writers repeat each other too (`write_native` :1054 and
`write` :1120, CPD 135 tokens). cli.md:1490-1493 says the batch is kept
answering "as `prove --deterministic` does" by hand.

**Change.** One function per suite that decides one sequent and returns
an answer value (verdict, evidence, line, status), called by `prove`,
the batch and a session's `close`; the writers take that value. The
suite is the selector of F-7a-01, matched once.

**Value.** A reader of the code; every change of a method's order is
made once, and F-7a-01 and F-7a-04 touch one place.

**Net.** The behaviour lock, the ratchet's journeys that run `prove`,
the batch's verdicts (`deduxit-bench run --all-families`). The search is
not touched; the order of calls must stay.

## F-7a-13

- dimension: architecture
- criterion: E6
- place: cli/src/settings.rs:336
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The common flags are declared and applied per command: the time
limit 6 times, the memory limit 8 times, the work limit 5 times and
`--settings` 11 times, each with its own doc and its own `apply` block.

**Witness.** `--timeout` at argument_parsing.rs:559, :588, :912, :1306,
:1498, :1745; `--memory-limit` at :563, :1007, :1297, :1503, :1606,
:1704, :1720, :1750; `--work-limit` at :567, :1024, :1312, :1506, :1754;
the `--settings` long help at :569, :590, :843, :1265, :1442, :1508,
:1608, :1756, :2066, :2178, :2211. The apply blocks:

```rust
        if apply.set("memory_limit") {
            settings.limits.memory_bytes = self.memory_limit.0;
        }
        if apply.set("timeout") {
            settings.clock.time_limit_ms = self.timeout.0.map(millis);
        }
        if apply.set("work_limit") {
            settings.limits.work = self.work_limit.0;
        }
```

repeated at settings.rs:342-350, :426-434, :458-466, :510-518 (CPD
329/413/445/497), and three identical `settings()` bodies for the term
commands (:384-410).

**Change.** A `#[derive(Args)] struct Limits { timeout, memory_limit,
work_limit }` and a `SettingsFile { settings }` flattened where they
apply, each with one `apply`; a per-command difference in a doc goes in
the command's `long_about`.

**Value.** A reader of the code; a user sees one wording of each flag on
every command.

**Net.** `each_flag_overrides_its_key_in_a_file`,
`every_id_asked_is_an_argument`, `the_settings_help_names_every_key`.

## F-7a-14

- dimension: idiom
- criterion: E6
- place: cli/src/check.rs:1236
- kind: refactor
- cost: medium
- candidate: no

**Claim.** `check`'s JSON verdict is built by a ten-field struct literal
16 times, with the status-to-word match 7 times, and it names its suite
by which of four optional fields is set.

**Witness.** `Checked { version: CHECKED_LEVEL, verdict: …, mode: …,
logic: …, calculus: None, system: None, error: …, message: …, lambda:
None, failing: None }` at check.rs:70, :494, :663, :693, :789, :881,
:1026, :1155, :1373, eliminate.rs:70, :222, :661, modal.rs:381, :481,
calculus.rs:212;

```rust
            verdict: match status {
                Status::Yes => "valid",
                Status::Unknown => "unknown",
                _ => "invalid",
            },
```

`mode` (linear), `logic` (ordinary), `system` (modal) and `calculus`
(rules) are four `Option`s of which one is set (:1241-1254).

**Change.** `Checked::new(status, suite)` with `#[derive(Default)]` and
`with_…` setters, the word from `Status`; at the next wire level one
tagged suite field in place of the four (a JSON break, written into the
changelog).

**Value.** A reader of the code; a front end reads one field for the
suite.

**Net.** The behaviour lock, the `wire-schema` check, README's examples
of `check --format json`.

## F-7a-15

- dimension: text
- criterion: T3
- place: docs/src/settings.md:3
- kind: text
- cost: small
- candidate: no

**Claim.** Lists of commands in the crate docs, two doc comments, the
manual's settings page and a comment have fallen behind the command.

**Witness.**
- docs/src/settings.md:3-5: "`prove`, `check`, `interact`, `eliminate`,
  `seq print` and the `workspace` commands that run take it." `grade`,
  `interpolate`, `resolve`, `herbrand`, `seq translate`, `seq normalize`
  and the five `term` commands take `--settings` too (help.md:116).
- cli/src/lib.rs:4-6: "The deduxit command line interface: `deduxit
  prove`, `deduxit check`, `deduxit interact` and `deduxit seq`" (11
  commands exist).
- argument_parsing.rs:1725 "What `seq translate` and `interpolate`
  share" and settings.rs:414 "the settings `seq translate` and
  `interpolate` run under": `resolve`, `herbrand`, `seq normalize`,
  `term inhabit` and `term enumerate` flatten `TransformArgs` too.
- check.rs:1182-1184: "// A failing world refutes the sequent: its
  certificate." above "// A failing world refutes the sequent: the model
  about it is a".

**Change.** Write the settings page as "every command that takes
`--settings` (`deduxit <command> --help` shows it)"; name no commands in
the crate doc beyond "the `deduxit` command"; "What the commands of one
ordinary sequent share"; drop the stale comment line.

**Value.** A user of the manual knows `--settings` works on the command
at hand; a reader of the code.

**Net.** `manual` and `doc-private` checks.

## F-7a-16

- dimension: text
- criterion: L8
- place: target/audit/help/check.txt:17
- kind: text
- cost: small
- candidate: no

**Claim.** The help headings come in a different order on each command,
and `seq print` has none.

**Witness.** `prove`: Options, Logic, Input, … (prove.txt:17, :26, :88);
`check`: Options, Limits, Input, Logic, Output (check.txt:17, :29, :65,
:106, :144) because `CheckArgs` declares `memory_limit` first
(argument_parsing.rs:1297); `grade`: Options, Input, Logic, Output,
Limits. Every flag of `seq print` (:1569-1620), `seq json` and `seq
fragment` is under "Options"; `--file` is under "Options" in `check`
and `eliminate`, under "Input" elsewhere (help.md:68-69). cli.md:20-28
fixes the order for `prove` only.

**Change.** Declare the fields of every `Args` struct in one order
(Logic, Input, Search, Output, Limits) and give every flag a heading;
extend `short_help_keeps_to_the_common_flags` to check the order.

**Value.** A user who knows one command's help finds a flag in another's.

**Net.** `cli.rs`'s help test; `readme.rs`.

## F-7a-17

- dimension: idiom
- criterion: E6
- place: cli/src/check.rs:342
- kind: refactor
- cost: small
- candidate: no

**Claim.** `read_document` reads a whole document of a given type six
times by the same five lines.

**Witness.**

```rust
        let mut document = serde_json::Deserializer::from_str(text);
        let derivation =
            deduxit::wire::upgrade::<deduxit::rules::Derivation, _>(&mut document, limits)
                .map_err(|error| limit_advice(error).unwrap_or_else(|error| error.into()))?;
        document
            .end()
            .context("not a derivation in JSON: more follows it")?;
```

at check.rs:343-349, :353-361, :365-370, :374-379, :395-407, :418-423,
and in `read_proof` :1278-1284.

**Change.** `fn whole<T: …>(text, limits, what) -> Result<T>` holding the
deserializer, the upgrade with `limit_advice` and the `end` check.

**Value.** A reader of the code; a new document kind is one line.

**Net.** The behaviour lock's `check` cases.

## F-7a-18

- dimension: idiom
- criterion: B8
- place: cli/src/batch.rs:565
- kind: refactor
- cost: small
- candidate: no

**Claim.** The batch edits serde's JSON output as text to put the name
first and append `seconds`.

**Witness.**

```rust
fn record_json(name: &str, body: &str) -> String {
    let name = serde_json::to_string(name).expect("a string is JSON");
    let rest = body.strip_prefix('{').expect("the body is an object");
    let (level, rest) = match rest.strip_prefix("\"version\":") {
```

(:565-581), and `body.pop(); body.push_str(&format!(",\"seconds\":{}}}",
…))` at :885-888 and :1030-1033.

**Change.** A `#[derive(Serialize)] struct Record<'a, T> { version,
name, #[serde(flatten)] body: &'a T, #[serde(skip_serializing_if)]
seconds }`, or `serde_json::Map` edits (with `preserve_order`, or the
version and name written first by the struct).

**Value.** A reader of the code; no `expect` on the shape of another
type's output.

**Net.** The batch's JSON Lines cases in the behaviour lock and
`batch.md`'s example.

## F-7a-19

- dimension: text
- criterion: L4
- place: cli/src/prove.rs:876
- kind: text
- cost: small
- candidate: no

**Claim.** Times in the command's lines are Rust's `Debug` of a
`Duration`, a developer's format the command's own `--timeout` cannot
read back.

**Witness.** `let after = format!("after {:.2?}", ended.elapsed);`
(:876), `{t:?}` at :128, :161, :172, :884 and in check.rs:65, :484.
The manual shows "after 54.08µs" (commands.md:83), "after 32.28µs"
(options.md:89); `parse_duration` (argument_parsing.rs:3388-3393) takes
`ms`, `s`, `m`, `h`, not `µs`.

**Change.** One `duration_text` that writes `54 µs`, `1.2 s`, `2 s` in
the units `--timeout` reads (or `ms` at least), used by every line.

**Value.** A user reads and reuses the times the command prints.

**Net.** README's and the manual's examples (marked `machine`), the
behaviour lock where it pins a time.
