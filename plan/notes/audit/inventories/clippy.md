sites: 3184, lints that fire: 53

| lint | sites | linear::search | linear::proofs | linear::nets | linear (rest) | ordinary | modal | lambda | rules | export | protocol | workspace | serialize | core (root) | core tests | cli | bench | web* |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `clippy::cast_possible_truncation` | 965 | 176 | 115 | 84 | 45 | 239 | 38 | 37 | 88 | 49 | 1 |  | 37 | 8 | 10 | 4 | 1 | 33 |
| `clippy::must_use_candidate` | 556 | 27 | 76 | 36 | 128 | 141 | 22 | 25 | 46 | 11 | 4 | 8 |  | 26 |  |  |  | 6 |
| `unused_qualifications` | 219 | 10 | 28 | 9 | 6 | 58 | 6 | 7 | 1 | 3 | 4 | 1 | 7 |  | 20 | 50 | 1 | 8 |
| `clippy::too_many_lines` | 216 | 28 | 28 | 7 | 5 | 50 | 5 | 9 | 11 | 7 | 3 | 3 | 5 | 4 | 16 | 25 | 8 | 2 |
| `clippy::too_long_first_doc_paragraph` | 144 | 3 | 2 | 1 | 5 | 30 | 2 | 1 | 8 | 23 | 2 | 1 |  | 6 |  |  |  | 60 |
| `clippy::doc_markdown` | 101 | 2 |  |  | 2 | 2 | 1 | 2 |  | 22 |  |  |  | 1 | 6 | 57 | 6 |  |
| `clippy::similar_names` | 94 | 9 | 23 | 5 |  | 13 | 7 | 5 | 2 | 15 |  |  |  |  | 1 | 6 | 1 | 7 |
| `clippy::many_single_char_names` | 88 | 19 | 21 | 7 | 3 | 26 | 1 | 4 |  | 3 |  |  | 2 | 1 | 1 |  |  |  |
| `elided_lifetimes_in_paths` | 75 | 7 | 15 |  | 8 |  |  |  |  | 12 |  |  |  |  | 3 | 3 | 27 |  |
| `clippy::enum_glob_use` | 69 | 5 | 29 | 2 | 14 | 7 |  | 1 | 3 | 1 |  |  | 6 | 1 |  |  |  |  |
| `clippy::single_match_else` | 67 | 14 | 5 | 3 | 5 | 13 | 2 | 3 | 5 | 2 | 2 |  | 1 | 1 |  | 7 | 3 | 1 |
| `clippy::or_fun_call` | 52 | 4 | 17 | 1 | 2 | 10 |  |  |  | 4 | 1 |  |  |  |  | 5 | 7 | 1 |
| `clippy::cast_precision_loss` | 43 | 5 |  |  |  |  |  |  |  | 23 |  |  |  |  | 6 | 1 | 8 |  |
| `clippy::redundant_clone` | 39 | 7 | 5 | 1 | 3 | 4 |  | 1 |  |  |  | 1 |  |  | 10 | 4 |  | 3 |
| `clippy::trivially_copy_pass_by_ref` | 38 | 1 | 9 |  |  | 4 |  | 2 |  |  |  |  |  |  |  | 22 |  |  |
| `clippy::match_same_arms` | 36 | 5 | 7 |  | 3 | 10 | 1 | 1 | 4 | 1 |  |  |  |  |  | 3 | 1 |  |
| `clippy::items_after_statements` | 32 | 1 | 3 | 5 | 7 | 7 | 1 | 1 | 2 |  |  |  | 2 |  | 3 |  |  |  |
| `clippy::needless_pass_by_value` | 28 | 6 | 6 | 1 |  | 3 |  | 2 |  | 1 |  | 2 |  |  |  | 2 | 2 | 3 |
| `clippy::match_bool` | 28 |  |  |  |  | 1 |  |  |  | 27 |  |  |  |  |  |  |  |  |
| `clippy::format_push_string` | 27 |  |  |  |  | 2 |  | 2 |  |  |  |  |  |  | 1 | 14 | 8 |  |
| `clippy::inline_always` | 26 | 15 | 7 |  | 4 |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::cast_possible_wrap` | 25 | 12 |  |  |  | 1 |  |  |  | 12 |  |  |  |  |  |  |  |  |
| `clippy::redundant_closure_for_method_calls` | 25 | 6 | 1 | 1 | 2 | 5 |  |  |  |  | 1 |  |  |  | 1 | 7 | 1 |  |
| `clippy::default_trait_access` | 19 |  | 8 | 1 | 5 | 1 |  | 4 |  |  |  |  |  |  |  |  |  |  |
| `clippy::semicolon_if_nothing_returned` | 16 | 2 |  |  | 2 | 3 |  | 3 |  | 2 | 4 |  |  |  |  |  |  |  |
| `clippy::match_wildcard_for_single_variants` | 12 | 11 |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::unnested_or_patterns` | 11 | 1 |  |  | 3 | 4 | 2 |  |  |  |  | 1 |  |  |  |  |  |  |
| `clippy::cast_sign_loss` | 11 | 5 |  |  |  | 3 |  |  |  | 3 |  |  |  |  |  |  |  |  |
| `clippy::struct_excessive_bools` | 11 | 4 |  |  | 2 |  | 1 |  |  |  |  |  |  |  |  | 3 | 1 |  |
| `clippy::unnecessary_wraps` | 11 | 1 |  |  | 1 | 1 |  |  |  |  | 2 |  |  |  |  | 1 | 5 |  |
| `clippy::needless_raw_string_hashes` | 10 |  | 1 |  |  |  |  |  |  |  |  |  |  |  | 3 | 3 | 3 |  |
| `clippy::nonminimal_bool` | 10 | 7 | 1 |  | 2 |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::manual_midpoint` | 8 |  | 1 |  |  | 1 |  |  |  | 6 |  |  |  |  |  |  |  |  |
| `clippy::unused_self` | 7 | 1 | 2 |  |  | 2 |  | 1 |  |  |  | 1 |  |  |  |  |  |  |
| `clippy::format_collect` | 7 | 2 |  |  |  | 4 |  |  |  |  |  |  |  |  |  |  | 1 |  |
| `clippy::explicit_iter_loop` | 6 | 1 |  |  |  | 1 |  | 4 |  |  |  |  |  |  |  |  |  |  |
| `single_use_lifetimes` | 5 | 5 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::allow_attributes` | 5 | 4 |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  |
| `clippy::allow_attributes_without_reason` | 5 | 4 |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  |
| `clippy::if_not_else` | 5 | 1 |  |  | 1 | 2 |  |  |  |  |  |  |  |  |  | 1 |  |  |
| `clippy::assigning_clones` | 5 | 1 | 1 |  | 1 |  |  |  |  |  |  |  |  |  |  | 1 | 1 |  |
| `clippy::maybe_infinite_iter` | 4 |  |  |  | 1 | 1 |  | 2 |  |  |  |  |  |  |  |  |  |  |
| `clippy::branches_sharing_code` | 3 |  |  |  |  | 1 | 1 | 1 |  |  |  |  |  |  |  |  |  |  |
| `clippy::ref_option` | 3 |  | 1 |  |  |  |  |  |  |  |  |  | 2 |  |  |  |  |  |
| `clippy::elidable_lifetime_names` | 3 | 3 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::inconsistent_struct_constructor` | 3 | 1 |  |  |  | 1 |  |  | 1 |  |  |  |  |  |  |  |  |  |
| `clippy::naive_bytecount` | 3 | 2 |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::needless_pass_by_ref_mut` | 2 |  |  | 1 |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::case_sensitive_file_extension_comparisons` | 2 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 2 |  |  |
| `clippy::debug_assert_with_mut_call` | 1 |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::checked_conversions` | 1 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::unnecessary_semicolon` | 1 |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::manual_string_new` | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |

## Functions over Clippy's line limit: 216

| lines | at |
|---:|---|
| 392 | `core/src/linear/proofs/first_order.rs:137` |
| 389 | `core/src/export/svg/net.rs:374` |
| 382 | `cli/src/prove.rs:296` |
| 328 | `core/src/protocol/ordinary.rs:953` |
| 319 | `core/src/errors/mod.rs:1311` |
| 288 | `cli/src/interact/mod.rs:452` |
| 284 | `core/src/ordinary/grounding.rs:1140` |
| 278 | `cli/src/workspace.rs:26` |
| 274 | `cli/tests/cli.rs:2004` |
| 270 | `core/src/rules/fitch.rs:184` |
| 250 | `core/src/export/svg/kripke.rs:130` |
| 248 | `core/src/rules/interactive.rs:1515` |
| 240 | `core/src/rules/interactive.rs:2067` |
| 237 | `core/src/ordinary/connection/intuitionistic/read_back.rs:784` |
| 233 | `core/src/ordinary/tableau.rs:264` |
| 232 | `core/src/linear/search/saturated.rs:944` |
| 226 | `core/src/ordinary/translate.rs:362` |
| 226 | `cli/src/check.rs:36` |
| 225 | `core/src/ordinary/derivation.rs:1383` |
| 220 | `cli/src/interact/ordinary.rs:178` |
| 219 | `core/src/export/svg/tree.rs:45` |
| 218 | `bench/src/run.rs:313` |
| 217 | `cli/src/batch.rs:682` |
| 216 | `core/src/ordinary/rocq.rs:142` |
| 216 | `cli/tests/cli.rs:1145` |
| 212 | `core/src/lambda/print.rs:775` |
| 207 | `core/src/modal/calculus.rs:252` |
| 206 | `core/src/linear/search/focus/counts.rs:356` |
| 205 | `core/src/ordinary/derivation.rs:2430` |
| 205 | `cli/src/grade.rs:284` |
| 203 | `core/src/ordinary/derivation.rs:837` |
| 202 | `core/src/lambda/read.rs:139` |
| 199 | `bench/src/compare.rs:326` |
| 197 | `bench/src/summary.rs:311` |
| 196 | `core/src/rules/text.rs:220` |
| 194 | `core/src/linear/proofs/size.rs:757` |
| 193 | `core/src/ordinary/model.rs:222` |
| 191 | `core/src/linear/search/refutation.rs:403` |
| 191 | `core/src/export/rocq/deduxit.rs:1009` |
| 188 | `core/src/linear/proofs/oracle.rs:523` |
| 188 | `core/src/linear/proofs/check/pass.rs:883` |
| 187 | `core/src/workspace/form.rs:391` |
| 187 | `core/src/rules/calculus.rs:383` |
| 185 | `core/src/modal/search.rs:1749` |
| 182 | `core/src/ordinary/grounding.rs:729` |
| 178 | `core/src/linear/proofs/interactive/cut.rs:246` |
| 175 | `core/src/linear/proofs/check/mod.rs:418` |
| 174 | `core/src/lambda/simple.rs:285` |
| 173 | `core/src/protocol/linear.rs:516` |
| 170 | `cli/src/interact/rules.rs:299` |
| 165 | `core/tests/first_order.rs:1287` |
| 162 | `core/src/linear/proofs/interactive/handover.rs:290` |
| 161 | `core/src/linear/search/focus/mod.rs:1250` |
| 161 | `core/src/linear/proofs/interactive/mod.rs:1228` |
| 160 | `cli/tests/cli.rs:2874` |
| 158 | `core/src/linear/proofs/interactive/expand.rs:558` |
| 158 | `core/src/linear/nets/mell/sequentialize.rs:256` |
| 156 | `core/src/ordinary/resolution/dpll.rs:105` |
| 156 | `core/src/ordinary/kripke_search.rs:762` |
| 154 | `core/src/ordinary/written/mod.rs:278` |
| 154 | `core/src/linear/nets/cut/mell.rs:209` |
| 151 | `core/tests/export.rs:1092` |
| 150 | `core/src/ordinary/structure.rs:120` |
| 150 | `core/src/linear/search/inverse/eval.rs:245` |
| 148 | `core/src/ordinary/sat/mod.rs:273` |
| 148 | `core/src/linear/search/refutation.rs:1845` |
| 148 | `core/src/linear/proofs/interactive/replay.rs:37` |
| 147 | `core/tests/export.rs:2016` |
| 147 | `core/src/ordinary/ljt.rs:1165` |
| 146 | `core/src/modal/search.rs:1453` |
| 146 | `cli/src/settings.rs:813` |
| 144 | `core/src/ordinary/rocq.rs:393` |
| 143 | `core/src/linear/search/refutation.rs:1688` |
| 142 | `core/tests/serialize.rs:595` |
| 142 | `core/src/linear/proofs/check/mod.rs:241` |
| 142 | `core/src/lambda/tests.rs:1499` |
| 142 | `cli/src/ordinary.rs:824` |
| 140 | `core/src/ordinary/rocq.rs:685` |
| 140 | `core/src/ordinary/interactive.rs:3643` |
| 140 | `core/src/linear/search/reference_fo.rs:418` |
| 139 | `core/tests/export.rs:3276` |
| 139 | `core/tests/export.rs:2181` |
| 138 | `bench/src/ordinary.rs:158` |
| 137 | `core/src/modal/decide.rs:415` |
| 136 | `core/src/ordinary/resolution/first_order.rs:1887` |
| 136 | `core/src/ordinary/normal.rs:862` |
| 136 | `core/src/ordinary/interpolant.rs:784` |
| 136 | `core/src/linear/nets/mod.rs:530` |
| 136 | `core/src/linear/mist.rs:139` |
| 135 | `core/src/serialize/errors.rs:59` |
| 135 | `cli/src/calculus.rs:143` |
| 134 | `core/src/ordinary/interactive.rs:3472` |
| 133 | `core/src/ordinary/written/mod.rs:1407` |
| 133 | `core/src/ordinary/translate.rs:105` |
| 133 | `core/src/ordinary/normal.rs:2727` |
| 133 | `core/src/linear/search/mod.rs:3949` |
| 133 | `core/src/linear/proofs/interactive/invert.rs:393` |
| 132 | `core/src/ordinary/kripke_search.rs:521` |
| 132 | `core/src/linear/search/generate.rs:318` |
| 130 | `core/src/ordinary/rocq_models.rs:156` |
| 130 | `core/src/linear/proofs/interactive/handover.rs:103` |
| 130 | `bench/src/tools.rs:841` |
| 129 | `core/src/serialize/ordinary.rs:940` |
| 129 | `core/src/rules/draw.rs:85` |
| 128 | `core/src/linear/proofs/interactive/quantified.rs:564` |
| 128 | `core/src/linear/lltp.rs:270` |
| 128 | `bench/src/ordinary.rs:433` |
| 127 | `core/src/linear/search/mod.rs:760` |
| 127 | `core/src/linear/proofs/interactive/terms.rs:98` |
| 127 | `core/src/linear/occurrences/instances.rs:551` |
| 127 | `core/src/linear/nets/mell/graph.rs:188` |
| 127 | `core/src/lambda/parse.rs:1092` |
| 126 | `core/tests/rules.rs:1541` |
| 125 | `core/src/linear/search/basis.rs:275` |
| 124 | `core/src/rules/check.rs:221` |
| 124 | `core/src/ordinary/normal.rs:1624` |
| 124 | `core/src/linear/search/generate.rs:677` |
| 123 | `core/tests/export.rs:3140` |
| 123 | `core/src/ordinary/resolution/first_order.rs:1444` |
| 123 | `core/src/linear/search/net.rs:418` |
| 123 | `core/src/linear/search/mod.rs:5362` |
| 122 | `core/src/ordinary/transfer.rs:74` |
| 122 | `core/src/ordinary/connection/read_back.rs:713` |
| 122 | `core/src/ordinary/connection/matrix.rs:767` |
| 122 | `core/src/linear/proofs/derivation/classical.rs:1254` |
| 122 | `bench/src/run.rs:615` |
| 121 | `core/src/linear/search/saturated.rs:354` |
| 121 | `core/src/linear/search/mod.rs:2469` |
| 121 | `core/src/first_order.rs:419` |
| 120 | `core/src/ordinary/models.rs:165` |
| 119 | `cli/src/eliminate.rs:308` |
| 118 | `core/src/rules/calculus.rs:792` |
| 118 | `core/src/ordinary/normal.rs:1854` |
| 118 | `core/src/ordinary/interactive.rs:4868` |
| 118 | `core/src/linear/search/essential.rs:285` |
| 118 | `core/src/linear/nets/essential.rs:131` |
| 118 | `core/src/export/svg/fitch.rs:20` |
| 118 | `cli/src/seq.rs:30` |
| 117 | `core/tests/lock.rs:35` |
| 117 | `core/tests/first_order.rs:1472` |
| 117 | `core/src/linear/proofs/derivation/mod.rs:453` |
| 117 | `core/src/lambda/infer.rs:324` |
| 117 | `bench/src/run.rs:878` |
| 116 | `core/tests/export.rs:1370` |
| 116 | `core/src/ordinary/kripke_search.rs:944` |
| 116 | `cli/src/ordinary.rs:445` |
| 115 | `core/tests/rules.rs:1349` |
| 115 | `core/src/linear/search/planar.rs:505` |
| 115 | `core/src/linear/proofs/interactive/mod.rs:1998` |
| 115 | `cli/src/check.rs:1085` |
| 114 | `web/tests/calls.rs:737` |
| 114 | `core/src/linear/search/reference_fo.rs:1179` |
| 114 | `core/src/linear/proofs/derivation/build.rs:621` |
| 114 | `cli/tests/cli.rs:548` |
| 114 | `cli/src/batch.rs:1422` |
| 113 | `core/src/rules/search.rs:126` |
| 113 | `core/src/ordinary/resolution/first_order.rs:1645` |
| 113 | `core/src/linear/search/reference.rs:474` |
| 113 | `cli/tests/readme.rs:651` |
| 112 | `core/src/ordinary/structure.rs:340` |
| 112 | `core/src/modal/search.rs:923` |
| 112 | `core/src/linear/search/refutation.rs:720` |
| 112 | `core/src/limits.rs:759` |
| 112 | `core/src/lambda/draw.rs:369` |
| 112 | `cli/tests/cli.rs:58` |
| 111 | `core/src/ordinary/sat/mod.rs:654` |
| 111 | `core/src/ordinary/decide.rs:1149` |
| 111 | `core/src/ordinary/connection/matrix.rs:428` |
| 111 | `core/src/linear/search/reference_fo.rs:713` |
| 110 | `core/tests/natural.rs:606` |
| 110 | `core/src/serialize/ordinary.rs:709` |
| 110 | `core/src/ordinary/sat/read_back.rs:345` |
| 110 | `core/src/ordinary/resolution/first_order.rs:265` |
| 110 | `cli/tests/cli.rs:673` |
| 110 | `cli/src/eliminate.rs:156` |
| 109 | `core/src/rules/natural.rs:626` |
| 109 | `core/src/ordinary/sat/classical.rs:112` |
| 109 | `core/src/linear/sequents/notation.rs:839` |
| 109 | `core/src/linear/search/inverse/rules.rs:407` |
| 109 | `core/src/linear/search/focus/waiting.rs:363` |
| 109 | `core/src/linear/proofs/cut/mod.rs:1271` |
| 109 | `core/src/linear/proofs/check/error.rs:391` |
| 109 | `core/src/linear/nets/sequentialize.rs:229` |
| 109 | `core/src/export/svg/mod.rs:1945` |
| 108 | `core/src/linear/proofs/interactive/mod.rs:1496` |
| 108 | `core/src/lambda/reduce.rs:442` |
| 106 | `core/src/serialize/sequents.rs:264` |
| 106 | `core/src/ordinary/normal.rs:1981` |
| 106 | `core/src/errors/mod.rs:1636` |
| 105 | `core/tests/export.rs:2345` |
| 105 | `core/src/serialize/search.rs:439` |
| 105 | `core/src/ordinary/herbrand.rs:1360` |
| 105 | `core/src/linear/search/saturated.rs:606` |
| 105 | `core/src/linear/proofs/interactive/mod.rs:1112` |
| 105 | `core/src/linear/proofs/check/mod.rs:965` |
| 105 | `cli/src/check.rs:721` |
| 104 | `core/src/linear/search/weighted.rs:151` |
| 104 | `core/src/linear/search/batch.rs:520` |
| 104 | `core/src/linear/proofs/interactive/replay.rs:214` |
| 104 | `core/src/linear/proofs/interactive/quantified.rs:183` |
| 104 | `core/src/linear/nets/mell/tests.rs:452` |
| 103 | `core/tests/export.rs:1563` |
| 103 | `core/src/workspace/tests.rs:109` |
| 103 | `core/src/ordinary/decide.rs:807` |
| 102 | `core/tests/depth.rs:72` |
| 102 | `core/src/workspace/form.rs:212` |
| 102 | `core/src/rules/check.rs:493` |
| 102 | `core/src/ordinary/sat/classical.rs:235` |
| 102 | `core/src/ordinary/models.rs:911` |
| 102 | `core/src/linear/proofs/interactive/quantified.rs:718` |
| 102 | `core/src/linear/occurrences/ill.rs:690` |
| 101 | `web/tests/calls.rs:1106` |
| 101 | `core/src/protocol/modal.rs:467` |
| 101 | `core/src/linear/proofs/derivation/mod.rs:284` |
| 101 | `core/src/lambda/check.rs:1069` |
| 101 | `core/src/export/rocq/deduxit.rs:742` |
