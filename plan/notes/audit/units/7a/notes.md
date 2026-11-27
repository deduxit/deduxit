# Unit 7a: notes

## Inventories
- duplicates: check.rs:872/1017 (197 tok); argument_parsing.rs:1492/1738 (149); settings.rs:329/413/445 (137), 329/497 (125); batch.rs:1062/1135 (135); argument_parsing.rs:1320/2350 (105)
- clippy too_many_lines: prove.rs:296 (382 lines), check.rs:36 (226), batch.rs:682 (217), settings.rs:813 (146), seq.rs:30 (118), check.rs:1085 (115), batch.rs:1422 (114), check.rs:721 (105)
- cli lint debt: doc_markdown 57, unused_qualifications 50, too_many_lines 25, trivially_copy_pass_by_ref 22, format_push_string 14
- help.md: 80 flags; 16 flags whose short/value/heading differs between commands (--to FORM vs --to LOGIC; --method; --steps; --node-limit; --depth; --format/--output/--file heading Options vs Output/Input; --memory-limit Options in seq print/json/fragment)

## lib.rs
- lib.rs:4-7 crate doc names only prove, check, interact, seq; the command has 11 subcommands (grade, eliminate, interpolate, resolve, herbrand, term, workspace missing). T3/text.
- lib.rs:83-95 `Status::worse` ranks by closure; a derived `Ord` with variants in rank order and `max` would do (E2 small).
- lib.rs:300 CLI matches the library hint text by prefix "intuitionistic linear logic has no" (also hint.rs "-> is ordinary logic's arrow"): stringly coupling; recorded in cli.md:63-70 as known. Hint could be a kind.
- lib.rs:344 `eprintln!("error: {e:#}")`: anyhow chain on one line.
- lib.rs:111 ERROR const 2; Status::Error doc says "An entry of a batch was an error".

## main.rs
- fine.

## argument_parsing.rs
- :17-35 SYNTAX is linear logic's alone; :53-67 ORDINARY = linear syntax then "With --logic the input is ordinary logic" (D4: linear first, ordinary as an add-on; no modal syntax anywhere in after_help although prove/check/grade take --system). Top-level after_help = ORDINARY (:101).
- :111-116 prove's about: "The command finds the sequent's fragment (MLL, MALL, …) and picks the engine" linear-only description of a command that decides 3 suites.
- :171 grade: "The problem is the sequent, in linear logic by default, or in the logic --logic, --system or --calculus names" (D1 witness).
- :534-549 StatementArgs (workspace new/add): `--logic`, `--system`, `--mode MODE` as a `String` default "classical" conflicts_with logic/system: D1 default linear; mode is a free String (C-CUSTOM-TYPE); third spelling of the linear mode beside -i/-a/--mix flags and the batch's MODE column / --entry-mode String.
- :677-698 ProveArgs: mode: ModeArgs (-i -a --mix --cyclic --lambek --empty-antecedents), logic: LogicArgs (--logic, --translation), system --system conflicts_with logic. No flag -> Mode::CLASSICAL linear (ModeArgs::mode :2629-2649). --system doesn't conflict with -i/-a/--mix? only conflicts_with="logic". check later what happens with `prove --system S4 -i`.
- :1222 interact --calculus conflicts with logic,-i,-a,--mix; interact has no --system (modal) at all.
- :1345 check --logic only "With a Kripke model"; :1359 --system same. check of a proof reads mode from file.
- :1476-1485 grade --logic (no conflicts with -i), --system, --calculus.
- :1478 grade --system doc "K, D, T, B, S4, S5, or K or D followed by axiom letters, such as KB4" vs prove :690-692 list of 15 systems: two descriptions of one flag.
- :1423 check --calculus heading INPUT; grade/interact --calculus heading LOGIC (help.md).
- --settings long help copied 12x (WorkspaceArgs :569, RunArgs :590, ProveArgs :843, InteractArgs :1265, CheckArgs :1442, GradeArgs :1508, seq print :1608, TransformArgs :1756, TermCheck :2066, TermReduce :2178, TermInfer :2211) with variants; a test pins the keys (:3435). E6 text duplication.
- :1569-1620 seq print: no help_heading on any flag (Options), -i/--cyclic/--lambek but no -a/--mix/--system; seq json :1681 linear only (no --logic); seq fragment linear only.
- :1725 TransformArgs doc "What `seq translate` and `interpolate` share" but resolve, herbrand, normalize, term inhabit/enumerate flatten it too (stale).
- :1784 seq translate `--by TRANSLATION` (negative translation) vs prove `--translation` (linear embedding) vs workspace `--set translation=glivenko` (cli.md:441); :1791 `--to LOGIC` in translate vs :1802 `--to FORM` in normalize (L8, same flag two meanings).
- :2057, :2152, :2202: term check default calculus simple, reduce and infer default untyped.
- CalculusArg kebab `system-f` while JSON writes "calculus":"system_f" and lambda::Calculus::NAMES says "as the JSON form and the command write them" (core/src/lambda/mod.rs:345). Ran: `term check --calculus system_f` -> "invalid value 'system_f'". Same for StrategyArg normal-order vs library NAMES normal_order (core/src/lambda/reduce.rs:119); FragmentArg mll-units vs Fragment NAMES "MLL with units".
- 28 ValueEnum enums, 19 `impl From<XArg> for library::X` mirrors (Schedule, Form, Calculus, Strategy, Translation(neg), IllSyntax, Logic, Method, Translation, Layout, ClassicalSides, Notation, Structural, Compact, MathFont, Kernel, Fragment, Bias, Format<-SequentFormat). Engine alone goes through library NAMES (:3349 engine_name). Library has NAMES for Bias, Notation, Strategy, Calculus, Fragment.
- :2469 LogicArgs --logic help "a formula or a sequent of propositional logic"; LogicArg :2496 "Classical propositional logic"; InputFormat::Tptp :2427 "ordinary propositional logic in TPTP syntax" — but first-order is decided (connection, grounding, models; iltp-first-order). Stale text.
- :2671-2673 --output doc "The file is made only when it holds a derivation or a net, and always in JSON." — check what it means.
- --memo-limit "Zero switches the memo off", --classical-work "or 0 for none", others `none`: mixed conventions for "no bound" (u32 default_value_t vs Count).
- :3131/:3383 hand-written parse_duration; Time Display uses `{t:?}` (Debug of Duration, may print µs which parse_duration refuses). humantime could replace (B8) — minor.
- :3401-3468 tests ok; settings-help test.

## settings.rs
- :246-251 ProveArgs::settings: batch.mode set only when one of ["intuitionistic","affine","mix"] is given; `--cyclic`, `--lambek`, `--empty-antecedents` are not asked, so with --settings FILE holding batch.mode, `prove --cyclic` batch keeps the file's mode (help :852 says "batch.mode is a batch's mode unless a mode flag is given"). Test table :846-848 covers -a, --mix, -i -a only. VERIFY with run.
- :336-474 the five limit blocks (memory/timeout/work/occurrence/term) repeated in CheckArgs, TransformArgs, GradeArgs, WorkspaceArgs (CPD 329/413/445/497). Paired with argument_parsing having --timeout in 6 Args structs, --memory-limit in 7, --work-limit in 5, each with its own doc. A flattened `LimitArgs` with one `apply` would remove both.
- :377-411 three identical `settings()` impls for TermCheck/Reduce/Infer (just read()).
- :413 TransformArgs::settings doc "`seq translate` and `interpolate`" stale (resolve, herbrand, grade? no grade has own; term inhabit/enumerate).
- :587 of_print takes 5 positional params incl. Option<u64> x2 (C-CUSTOM-TYPE minor).
- :636 note "--jobs N is more than this machine runs" from Threads::of – names a flag from inside settings (fine).
- :844 settings JSON "MALL" vs flag `mall`: the fragment's spelling differs between --fragment and search.fragment (file "MLL with units" vs flag mll-units).
- tests good: each_flag_overrides_its_key_in_a_file pins keys.
- No env var configuration at all (no DEDUXIT_* ), no user config file: L12 only partly applies.

## io.rs
- Well built: partial files renamed, BrokenPipe taken as done, stdin terminal refused with message (:208-215), capped reads.
- :906-921 Output::open(Some("-")) creates a file named `-`; `--file -` reads stdin. RAN: `prove -o - 'A |- A'` -> exit 0, nothing on stdout, file `./-` holds the verdict and tree. L6 (`-` for stdout where a file is written).
- argument_parsing.rs:2671-2673 `--output` help: "The file is made only when it holds a derivation or a net, and always in JSON. Otherwise the verdict goes to standard error." RAN: `-o out.tex 'A |- A'` writes LaTeX; `-o out.txt 'A |- B'` writes no file and verdict to stderr. "always in JSON" is wrong/misleading (means: a JSON output is always made). T1.
- :667-675 a `.p` file is LLTP unless --logic, and the CLI parses it a second time as TPTP to print a note; D1 makes this moot.
- :149-166/175-184 two identical with_context closures ("cannot read {path}" / "cannot read standard input"), minor.
- :399-408 is_ill_json parses the JSON once as a probe and again for real (not hot).

## limit.rs
- Clean: Deadline as AtomicBool raised by timer thread; within() leaves the load thread behind; Notice writes a line on a terminal stderr and clears it with "\r\x1b[2K" (L9 good). Notice ignores NO_COLOR/TERM=dumb for the escape (TERM=dumb terminals may not take \x1b[2K) — minor L4/L12.

## style.rs
- :17-19 FORMATS has `fitch`, but the error at :53-56 lists "(text, latex, typst, svg, png, pdf or rocq)" without fitch; fine-ish.
- :65 a value that is no JSON is a string (documented).
- Clean otherwise.

## seq.rs
- :55-79 `seq print --logic L` prints the linear IMAGE of the ordinary sequent (RAN: `seq print --logic classical 'a -> b |- b'` -> `⊢ a & ~b, b`); there is no command that prints an ordinary or modal sequent itself (latex/typst/svg) nor converts one to JSON: `seq json` and `seq fragment` read linear logic only (no --logic, no --system). RAN `seq print '[]p -> p'` -> parse error without a hint at modal logic. `seq print 'a -> b |- b'` hint says "add --logic classical", which then prints the image, not the sequent. D4/D1.
- :117, :143 seq json/fragment build limits ad hoc (io::bound + memory) and take no --settings: inconsistent with seq print (L8 minor).
- :142-146 seq fragment reads in Mode::CLASSICAL then names in -i mode: --ill-syntax strict not applied (fine).
- seq.rs:30 run is 118 lines (clippy).

## prove.rs
- :296-721 `proved` is 382 lines: batch check, ordinary_mode, the --system branch (to crate::modal::prove), then ordinary logic (native methods, model race, model_first, embedding) threaded as `Option<(options, sequent)>`, `solved`, `gave_way`, `race` through the linear search and both output paths (JSON :573-615, text :617-718). The suite is chosen implicitly by which Options are Some. E5 seam: choose the suite once (an enum from the D1 selector), one function per suite (modal already is one).
- :746-752 ordinary_mode refuses only -i, -a, --mix beside --logic. RAN: `prove --logic classical --cyclic 'a |- a'` -> exit 0 valid (flag ignored); `prove --logic intuitionistic --lambek` -> exit 0 (ignored); `interact --logic classical --cyclic` starts (ignored); `grade --logic classical --cyclic` -> exit 2 "leave out the mode flags"; `seq print --logic --cyclic` -> exit 2 (seq.rs:55); `prove --system S4 -i/--lambek` -> exit 2 (modal). takes_logic (argument_parsing.rs:648-651) knows cyclic and lambek. L9/L8.
- :239-251 --szs: computes settings twice (minor), status line last (recorded departure).
- :128, :161, :172, :884 Duration printed with `{:?}` Debug (e.g. "10s", "1.5s", "100µs"): developer formatting in user text; `--timeout` reads "ms/s/m/h" only (L4 minor).
- :941-963 statistics reads counters by serializing Statistics to JSON and looking keys up.
- :875-935 advice names the flag for each Reason: good L5.

## check.rs
- :36-268 check(): document sniffed by keys (read_document :319-425), then 9 arms each `io::Output::open(...)` + `(status, keep)` + `if keep { out.finish()? }` (repeated ~9x).
- :342-424 five copies of `Deserializer::from_str(text)` + `wire::upgrade::<T,_>(&mut document, limits).map_err(|error| limit_advice(error).unwrap_or_else(...))` + `document.end().context("not a X in JSON: more follows it")` (calculus, resolvents, system, worlds, logic/verdict, disproof; plus read_proof :1278). One generic helper `whole::<T>(text, limits, what)`.
- `Checked { version: CHECKED_LEVEL, verdict, mode, logic, calculus, system, error, message, lambda, failing }` literal 16x (check.rs 10, eliminate.rs 3, modal.rs 2, calculus.rs 1), the Status->"valid"/"invalid"/"unknown" match 7x. Four optional fields mode/logic/calculus/system name the suite: one per suite (D4/L8: JSON says "mode" for linear, "logic" for ordinary, "system" modal, "calculus" rules).
- CPD check.rs:872 (check_countermodel) / :1017 (write_refuted) same verdict-line/JSON/draw block.
- :761-771 and :1398-1408 the same refusal advice match (memory/work/timeout).
- :1182-1184 two comment lines, first a stale fragment: "// A failing world refutes the sequent: its certificate." then "// A failing world refutes the sequent: the model about it is a countermodel...".
- :1311-1313 good error naming the command to run.
- check of a proof without `mode` in file takes ModeArgs default linear classical (:276-285): D1 witness for check.
- check --logic/--system serve only Kripke models (help), --calculus only rules derivations; the logic of a proof comes from the file.

## batch.rs
- RAN (settings.rs:246 bug confirmed): s.json `{"batch":{"mode":"affine"}}`, l.txt `x: A, B |- A`; `prove --settings s.json --cyclic --input-format lines -f l.txt` -> "x: provable (MLL, classical affine, focus engine)" exit 0 (the flag lost to the file); `-i` instead -> intuitionistic used; without file `--cyclic` -> cyclic, unprovable. L12 (flags win over the file).
- :682-915 Shared::decide repeats prove.rs proved's ordinary/linear pipeline (by_native -> models_beside/model_first -> translated within bounded -> search -> evidence -> verdict_line/ordinary::status -> JSON or text). Two copies of the per-sequent decision (E6), kept in step by hand (cli.md:1490-1493 says so).
- :1054-1115 write_native vs :1120-1205 write: same open/create_dir/JSON/finish/Shown match (CPD 1062/1135).
- :565-581 record_json edits JSON as text (strip '{', parse version digits), :885-888 and :1030-1033 `body.pop(); body.push_str(",\"seconds\":…}")` – string surgery on serde output; a serde struct with #[serde(flatten)] would do (E2/idiom).
- :300-305 (prove.rs) a batch refuses --system: modal logic cannot be batched; records carry `mode` only (linear), no `logic`/`system` per record (batch.rs:400-433). D4.
- :331-369 Problems format: `NAME; MODE; EXPECTED; COPIES; SEQUENT`, EXPECTED and COPIES not read — the benchmark harness's file format as a user-facing input format.
- :1367-1410 machine_memory reads /proc/meminfo and cgroups by hand: Linux-only (fine, falls back).
- Batch writes one line per entry, flushes each (P9 ok). JSON Lines records (L4 ok).

## help/ and manual
- help/*.txt: no example in any --help (grep "example" none), no URL to the manual (grep https none); prove --help 538 lines; --method long help one 400-word paragraph (prove.txt:76).
- Heading order differs: prove Options/Logic/Input…, check Options/Limits/Input/Logic/Output (Limits first, since CheckArgs declares memory_limit first), grade Options/Input/Logic/Output/Limits; seq print has no heading at all (every flag under Options); --file under Options in check/eliminate, Input elsewhere.
- SequentInput's shared doc says "(but not by `interact`)" in every command's help (prove.txt:15, seq print).
- No man page, no completions (no clap_mangen/clap_complete; nothing in docs/installation.md).
- docs/src/commands.md:1 "Decide a sequent" is linear logic only; :140 "By default a sequent is read in classical linear logic."; README.md:78 "Without `--logic`, `prove` reads linear logic".
- docs/src/settings.md:3-5 lists prove, check, interact, eliminate, seq print, workspace as the commands that take --settings; grade, interpolate, resolve, herbrand, seq translate, seq normalize and the five term commands take it too (help.md:116). settings.md:51-52 "A mode flag given on the command line overrides it" (false for --cyclic/--lambek with a file).
- docs/src/options.md:83, 89 and settings.md:64 show "54.08µs", "32.28µs", "300.39µs": Debug of Duration.
- interact/ordinary.rs:85-86 has a 4th copy of the -i/-a/--mix refusal (lacks --cyclic/--lambek) — 7b's file; grade.rs:100-105 uses mode.given() (full).
- cli.md:121-122 records ordinary_mode refusing -i, -a, --mix "the image's mode is the translation's": the reason covers --cyclic/--lambek too, so the omission is a gap.
