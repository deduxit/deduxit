# Unit 7b notes

## interact/mod.rs, ordinary.rs, rules.rs
- mod.rs:440-930 vs ordinary.rs:177-578: Session copied: undo arm (mod 594-614 / ord 255-275), invertible arm (615-629/276-290), close loop (630-650/291-304), show arm (651-670/305-324), save/load (712-754/360-400), invert_all/stepped/goals/opened (829-888/415-557). CPD: 268 tok mod:593/ord:254, 145 mod:851/ord:424, 107 mod:871/ord:540.
- rules.rs session: `show` only lines (275), `proof [FILE]` JSON only (434-436) — `proof x.tex` writes JSON; linear/ordinary `proof FILE` pick format by extension. Library has rules::Drawing/Fitch (check --calculus --format latex uses them).
- Message wording differs across the 3 sessions: goal parse "is not a goal: a goal is a number, and `goals` lists them" (mod 473, ord 185) vs "names no goal: give its number, as `goals` shows it" (rules 294); "no rule acts on it" vs "no rule applies to the goal" (rules 308); unknown command "`help` lists them" vs "`help` lists the commands" (rules 469); load of wrong kind: three wordings (mod 725, ord 373, rules 455).
- proof verdict: linear "valid proof ({mode})" (mod 677), ordinary "valid proof in LK (classical logic)" (ord 333), rules "valid in the calculus given (NAME): SEQ" (rules 406).
- interact exit: Status::No (1) when unfinished (mod 76-80) -> exit 1 = "unprovable" by D3? check docs.
- show arm: derivation_within(|_| false), latex::write(|_| false): no stop, Ctrl-C ignored during show (mod 653-668; ord 307-321).
- ordinary.rs:672 binders: stack walk no seen set (handover, exponential on DAG); only on unknown rule error path.
- ordinary.rs:625 ASCII and 656 ordinary_rules: copies of core tables (F-4a-07); mod.rs:935 LINEAR_ASCII, 968 linear_rules.
- rules.rs:92 interact --calculus ignores --file (only argument); no no_standard_input call.
- from_stdin (mod 205): on a terminal prompt "> " but nothing printed at start (no goals, no "help"/"quit" hint) unless invertible on.

## show.rs
- show.rs:1513 `#[allow(clippy::too_many_arguments)]` no reason; :768 uses #[expect(.., reason)] (convention: expect at site).
- show.rs:628 first_order_model_drawn / :702 model_drawn / :1533 net_into: same tail (Stopped→LeftOut why, error→LeftOut, svg write or render match) x3 (CPD 673/735).
- show.rs:522/546/595 three certificate_into fns with same `match written {Ok|WriteFailed→Written, Err→LeftOut("no certificate")}`.
- show.rs:441 too_large / :461 too_large_to_draw / :495 proof_unread / :505 over_memory: advice names `--format json`, `prove --format json` even when called from a session's `proof` (Show::session) or check: context-wrong advice? verify.
- show.rs:1127 seq print refusal says settings file's limits.derivation_bytes (no flag on seq print).
- render :1267 unreachable! for non-binary Format.

## ordinary.rs (cli)
- ordinary.rs:344-575 decide_natively/by_connection: the command's own copy of the library's ordinary default (core/src/ordinary/decide.rs:230 by_grounding, :412 connected_by_default, :461 with_evidence, :906 models after embedding). Grounding arm ord.rs:358-391 copies by_grounding:238-280 line for line ("as the library's by_grounding does", cli.md:1408). Policies differ: CLI time shares (FIRST_ORDER_SHARE 90, CONNECTION_SHARE 70, MODEL_GRACE 5) + thread race; library CONNECTION_WORK bound, sequential, models after embedding. cli.md:1447-1493 records the shares (library reads no clock). cli.md:1545 "never re-implements what core computes".
- ord.rs:325 by(): `_ => "LJT"` catch-all names Method::Embedding as LJT.
- ord.rs:946-971 three near-identical `if let (NotValid, Some(x), Text, false) ... show.after(model.to_string())` blocks.
- ord.rs:1127 native_statistics: per-search counter tables hand-written (linear side uses Engine::counters from library; cli.md:1380). Ordinary has no counters() in the library -> CLI keeps labels.
- write_native: verdict `_ => Status::Unknown`.
- session test: `proof` past --derivation-limit in session prints "--format json writes the proof itself" (show.rs:450): not a session word (session uses `proof --json`). Verified by run.
- rules session `proof c.tex` writes JSON into c.tex; linear/ordinary `proof p.tex` writes LaTeX (verified).

## modal.rs, calculus.rs
- calculus.rs:28 BUILT_IN (13) = web/src/rules.rs:38 BUILT_IN (13, "the command's"), and core/src/protocol/rules.rs:207 built_in only LK/LJ/LM; CLI also G3<system> via modal() calculus.rs:50 (web: no G3). Library has the rule-file consts but no lookup by name.
- calculus.rs:143 check_rules+:315 drawing vs :393 shown: two writers of a rules derivation; check_rules refuses png/pdf (verified: `check d.json --format png` exit 2) while prove --system --format png renders a G3 derivation via shown (verified). Fitch only in check_rules. write closure duplicated (:335 vs :401), list estimate duplicated (:266-278 vs :413-424) with different advice "--format json writes the verdict alone" vs "--format json writes it".
- modal.rs:533 counted == calculus.rs:605 counted.
- modal.rs:266-297 countermodel_shown: 4th copy of draw-then-render tail (show.rs 628, 702, 1533). Stopped -> "it was stopped" (modal 293, calculus 470) vs show.rs uses why() naming time limit.
- modal.rs:137 refusal names three flags without error.setting() (calculus.rs:191 uses setting()).
- modal.rs:308 worlds_text own printer (F-4a-12).

## tableau.rs, hint.rs, grade.rs
- setting()->flag advice match written ~16 times: eliminate.rs:757, term.rs:333 (says "--settings (limits.memory_bytes)"), term.rs:467, tableau.rs:135, interact/mod.rs:397-402, resolve.rs:95,166, transform.rs:127,349, calculus.rs:191,296, check.rs:432,761,1398; spellings differ ("--memory-limit SIZE", "raise --work-limit", "raise it with").
- counted() x3: calculus.rs:605, eliminate.rs:689 (pub(crate), used by grade.rs:647), modal.rs:533.
- grade.rs reads check's verdict from the first line of its text output (first_line, strip_prefix "unknown: "): couples grade to message wording. kept() held output.
- grade.rs:634 modal has no session: script refused for modal.
- hint.rs: fine; `-> hint` for seq print F-7a-04 (7a).

## eliminate.rs
- eliminate.rs:442 `println!("{line}")` bypasses io::Stdout (closed-pipe handling io.rs:747): `eliminate --net --format svg -o net.svg cutp.json | true` panics "failed printing to stdout: Broken pipe", exit 101 (VERIFIED, script pipe.sh). D3/L1.
- eliminate.rs:183-196 closure `sequent` == fn sequent_of :283-296 (CPD 112).
- eliminate.rs:199-244 duplicates unchecked() :626-686 (invalid/unknown message + Checked JSON).
- eliminate.rs:492-515 and :569-590: Net::Mll/Mell dispatch of nets::cut::eliminate twice.
- advice :756 another setting->flag table.

## resolve.rs, term.rs
- term reduce/check/infer: no --timeout/--memory-limit/--work-limit (help/term-reduce.txt: only --settings, "the others are read and not used"); reduce stop is |_| interrupted() (term.rs:257), clock setting ignored. VERIFIED: `term reduce --steps none '(\x. x x) (\x. x x)'` runs until killed (timeout 15 -> 124); `--timeout 1s` refused by clap. inhabit/enumerate take TransformArgs (--timeout etc.). term.rs:334 advice "--settings (limits.memory_bytes)".
- term check/infer: no catch_interrupt, |_| false stops (term.rs:53, :375, :378).
- resolve.rs:233-243 and :356-366: Bound -> why match duplicated; :95 and :166 refusal flag duplicated.
- resolve.rs:72 / :163 Form refusal: "raise limits.form_occurrences in a --settings file" while seq normalize has --form-limit for that key (cli.md:538).
- plural() resolve.rs:424 vs counted() x3.

## transform.rs, workspace.rs
- workspace.rs:431 verdict_status reads value["verdict"] string: "not_valid"|"unprovable" -> 1, "unknown" -> 3, else 0. VERIFIED: `workspace apply w.json 0 ordinary.resolve` on `a |- b` exits 0 (verdict "satisfiable"), `deduxit resolve 'a |- b'` exits 1; `ordinary.interpolate` node (no top-level verdict) exits 0, `deduxit interpolate 'a |- b'` exits 1. docs/src/workspace.md:52-54 "worst verdict run: 0 valid, 1 not valid".
- transform.rs:346 herbrand Form refusal "--settings and limits.form_occurrences"; resolve.rs:72 "raise limits.form_occurrences in a --settings file"; seq normalize has --form-limit (transform.rs:120).
- transform.rs:124-133 / :346-355 duplicate (CPD 100).
- transform.rs:500 verdict() vs prove's verdict lines: third wording of an ordinary verdict.
- interact at a terminal: only "> " printed, no goals, no help/quit hint (VERIFIED with script(1)).
- binders DAG: occurrence limit (unfolded, 50M) refuses dag.json at load; exponential not reachable by default (handover claim mitigated).

## help, manual, cross-checks
- eliminate flattens CheckArgs: help/eliminate.txt lists --sequent :102, --world :107, --tableau :110, --dimacs :115, --proof-format :120, --calculus :127, --logic :155, --system :165, --layout :171, --net, --lambda. VERIFIED: `eliminate --tableau|--dimacs x.cnf|--system S4|--lambda cutp.json` exit 0, flag ignored; --logic/--sequent/--calculus refused (eliminate.rs:44-51).
- docs/src/interactive.md:266-269 "same commands but `cut` and `invertible`": false, ordinary session has invertible (ordinary.rs:276; ordinary-logic.md:1136). interactive.md never names `interact --calculus`. interact --help (help/interact.txt) describes the linear session's commands and rules only.
- net choice x3: cli show.rs:1461-1505 (with Placement::Cut fallback), web/src/calls.rs:2298, core/src/protocol/linear.rs:553-563 (no fallback).
- statistics labels: linear in library (Engine::counters core/src/linear/search/mod.rs:2469); ordinary (cli ordinary.rs:1127-1184) and modal (modal.rs:166-171) labels only in CLI; web none.
