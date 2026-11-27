# Unit 5b notes

## Read so far: mod.rs, terms.rs, calculus.rs, derivation.rs, check.rs, print.rs, text.rs, draw.rs, fitch.rs

- calculus.rs:437 `if sort.is_empty() || sort_ids.insert(...).is_some()` -> message "the sort \"\" is declared twice" for an empty sort (wrong message). Same pattern ok at :538 ("empty or used twice").
- calculus.rs:383 `from_form` pub(crate): Calculus is made only by wire::upgrade from JSON (F-arch-16). No `FromStr`, no builder. Calculus has no Serialize? check serialize/rules.rs.
- check.rs:221 `check_some` generic over `impl FnMut` (150 lines) + check/check_open/check_node generic; Meter holds &mut dyn. Body compiled per caller -> crate rule (core.md:327) says shim. List under handover "every unit".
- check.rs:228/346 Error::InvalidDerivation{calculus, reason:String}: node index only inside text "node {index} (...)". A front end can't point at the node w/o parsing. Check web/cli for parsing "node ".
- check.rs:544-548 per written member per node: `"the conclusion".to_owned()` / format! allocated for an error message never shown unless fails (P11/P2). Also :539 partition into two Vecs per zone per sequent per node; :512 Vec of sequents per node; mapped() clones (:781); sets() unmapped(&[m]) allocs Vec per member (:912, :937). Candidate; no journey runs rules.
- check.rs:326 and :341 Fault->Error map twice (tiny).
- text.rs:232 occurrences bound checked per term_of slice; parse_sequent/parse_list call per member, so total tokens across members not bounded as docs (:522-524 "for a text of more tokens than limits.occurrences") claim.
- text.rs:193-213 `reading` duplicates print.rs:72-98 `form` defaults (fixity by arity, precedence 0 infix / MAX else, assoc None). Round-trip "what show writes reads back" rests on two copies. (print: arity 0 -> Function via `_`; text: (_,0) => Function explicit.)
- text.rs:434 sort_name: linear scan of HashMap to find name by index; form.sorts has the order (index i -> sorts[i-1]).
- text.rs:460 UnknownName{known: &[]} for a sort: no list of the calculus's sorts.
- text.rs:106 Lexer built per parse (HashMap of all symbols); tokens(): longest match scans all symbols per char.
- draw.rs:87-103 / fitch.rs:184-203 duplicated preamble (duplicates.md 118 tokens).
- draw.rs:285 `markup` ignores Labels::Table (user labels) -> answer to handover (F-1b-04).
- rules names linear: InfId, Labels, Drawn, LITERAL, RAISED, superscript (draw.rs:10-12), Notation (print.rs:10).
- fitch.rs:187/481 `Fitch::new(options: &FitchOptions)` then `options.clone()` (C-CALLER-CONTROL).
- fitch.rs:184 Fitch::new ~300 lines (ordering, line expansion, citations: three seams).

## ordinary.rs, tableau.rs, search.rs, natural.rs
- Connective-name table for ordinary formulas written 3x: rules/ordinary.rs:24-47 `terms_of` (ordinary Formulas -> rules Terms), natural.rs:858-881 (same loop, inline, in `Translator::derivation`), natural.rs:1084-1097 `ordinary_of` (inverse). natural.rs:858 could call terms_of.
- natural.rs:1012-1018 and search.rs:300-308: same NotApplicable "is not one of the natural deduction files deduxit ships" (Profile::of then error) twice.
- natural.rs:326 `self.trees[at].clone()` clones Tree (with Vec) per visit in replace; replace allocs memo HashMap per substitute; candidate (no journey).
- natural.rs:981 Derivation::natural is a shim to natural_dyn: OK per rule.
- search.rs:84 close: generic but short, delegates to found (dyn). search.rs:126 hint: 125-line body generic over `impl FnMut` stop -> per-caller copies (handover "every unit").
- search.rs:135, :203, :220: hint clones the whole session (self.clone(), state.clone() twice) per call (cf F-4e-09). Candidate.
- search.rs:84/126 close/hint take `&linear::search::Options` (search = crate::linear::search) for a ND session; Closed.outcome is ordinary::Outcome (D4, F-arch-02).
- search.rs:141 each found_at gets full limits: hint may run up to path.len() searches each with full limits.work (bound per search, not per call). Note only.
- search.rs:47 Hint.steps: Vec<Hinted> holds 0 or 1 element (Option would say it); check ordinary::Hint shape for consistency.
- tableau.rs:60 calculus name "tableau" vs TABLEAU file name - check.

## workspace/mod.rs, form.rs
- mod.rs:700 `state(node)` computes `states()` for every node (O(n) per call, key_of clones parameters/settings Box<str> per node :601-612); run() calls state per planned node twice (:1313,:1325) -> O(n^2) allocs. Candidate; no journey.
- mod.rs:572 intern lookup allocs Box::from(text) (copy of value text, maybe MBs) per lookup; :590 stores text twice (values + interned key) while bytes() (:595) counts once -> workspace.bytes bound covers half of what is held; bytes() sums all values per intern (quadratic). Also asked() :1127 Box::from per input.
- mod.rs:1003 compute (~110 lines) and :1303 run generic over impl FnMut stop (crate rule: shim).
- mod.rs:1534-1557 limit_named: hand-written list of 11 settings keys; names linear::search::Reason::NAMES (D4 handover).
- mod.rs:1565 no_node -> IndexOutOfBounds{space: Node, len: 0}: says the workspace has 0 nodes. mod.rs:579 Refusal::Index{what: Space::Node, count: u64::MAX} for the value table.
- mod.rs:1059 `(_, None) => Err(String::new())` an empty error JSON.
- mod.rs:1743 made_now runs protocol::run with |_| false (no stop) in Edit::connect/restart.
- index rebuild written 3x: mod.rs:1508-1518, form.rs:568-580, form.rs:760-772.
- form.rs:381 `read_own` pub: trusts every record and check of any document (Trust::Own, :454) -> forged `held` check & search output answer `asked`. core-workspace.md "Two readers" records it; C-VALIDATE: an unchecked path is named `_unchecked`.
- form.rs:687 verify runs instant ops with |_| false.

## Cross-checks
- limit_named (workspace/mod.rs:1541-1557) lacks search.path_length, search.model_size, search.model_worlds which Reason::setting names (linear/search/mod.rs:3659-3661): DRIFT, a read-back record loses its setting.
- built-in calculi table x4: cli/src/calculus.rs:28 (13 + G3 lookup :50), web/src/rules.rs:38 BUILT_IN + :55 NAMES, core/src/protocol/rules.rs:207 (LK/LJ/LM only), natural.rs:84 FILES (8 ND).
- Derivation API raw u32: derivation.rs:66 premises &[u32], :71 scope u32, :395 scope(parent: u32) -> ScopeId; crate converts back: natural.rs:906,927,941,948; rules/ordinary.rs:203-207; tableau.rs:90.
- InvalidDerivation{calculus, reason} (errors/mod.rs:711) variant non_exhaustive -> node field additive. check.rs:347-355 node index only in text.
- positions: compile rule calculus.rs:846-860 undocumented publicly; Calculus::positions doc :362-365 "cannot find otherwise"; manual rules.md:49-51.
- read_own used by web/src/workspace.rs:40.
- no_node -> out_of_bounds Space::Node "a proof refers to node {index}, but it has 0 nodes" (errors/mod.rs:1196-1198).
- No SipHash maps in unit (crate::hash everywhere; BinaryHeap/BTreeSet only).
- Generic stop bodies: check.rs:221 (callers: interactive x4, modal decide x2, modal rocq, protocol rules, natural dyn, cli, web), search.rs:126 hint, workspace/mod.rs:1003 compute, :1303 run.

## Chosen findings (19), in order
01 limit_named drift; 02 built-in calculi table x4; 03 positions undocumented; 04 raw u32 in Derivation API; 05 InvalidDerivation node in text only; 06 rule-file errors omit names; 07 read_own unchecked naming; 08 connective table x3; 09 print/text defaults x2; 10 no_node message; 11 parse_sequent bound per member; 12 generic stop bodies (cand); 13 checker diagnostics allocs (cand); 14 Workspace::state recomputes all (cand); 15 intern double storage (cand); 16 hint clones session (cand); 17 index rebuild x3; 18 Drawing/Fitch preamble; 19 Fitch::new clones options.
