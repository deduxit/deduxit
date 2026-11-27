
## core/src, top level

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `errors/` | 4 | 2885 | 0 | `errors/mod.rs` (1820) |
| `export/` | 17 | 13665 | 0 | `export/svg/mod.rs` (2438) |
| `first_order.rs` | 1 | 695 | 0 | `first_order.rs` (695) |
| `goal_tree.rs` | 1 | 173 | 0 | `goal_tree.rs` (173) |
| `hash.rs` | 1 | 23 | 0 | `hash.rs` (23) |
| `kripke.rs` | 1 | 125 | 0 | `kripke.rs` (125) |
| `lambda/` | 15 | 13705 | 2669 | `lambda/tests.rs` (1653) |
| `lib.rs` | 1 | 437 | 0 | `lib.rs` (437) |
| `limits.rs` | 1 | 889 | 0 | `limits.rs` (889) |
| `linear/` | 107 | 90122 | 10469 | `linear/search/mod.rs` (5552) |
| `modal/` | 9 | 6427 | 635 | `modal/search.rs` (1959) |
| `name.rs` | 1 | 150 | 0 | `name.rs` (150) |
| `ordinary/` | 52 | 56540 | 1430 | `ordinary/interactive.rs` (5900) |
| `protocol/` | 8 | 6419 | 945 | `protocol/ordinary.rs` (1939) |
| `rules/` | 14 | 9824 | 0 | `rules/interactive.rs` (2355) |
| `serialize/` | 14 | 7963 | 0 | `serialize/ordinary.rs` (2065) |
| `settings.rs` | 1 | 359 | 0 | `settings.rs` (359) |
| `unify.rs` | 1 | 322 | 0 | `unify.rs` (322) |
| `wire.rs` | 1 | 630 | 0 | `wire.rs` (630) |
| `workspace/` | 3 | 3562 | 861 | `workspace/mod.rs` (1927) |
| **total** | 253 | 214915 | 17009 | |

## core/src/linear

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `families.rs` | 1 | 1292 | 0 | `families.rs` (1292) |
| `fragment.rs` | 1 | 877 | 0 | `fragment.rs` (877) |
| `lltp.rs` | 1 | 566 | 0 | `lltp.rs` (566) |
| `mist.rs` | 1 | 806 | 0 | `mist.rs` (806) |
| `mod.rs` | 1 | 47 | 0 | `mod.rs` (47) |
| `nets/` | 13 | 9170 | 1456 | `nets/mod.rs` (2177) |
| `occurrences/` | 6 | 5084 | 0 | `occurrences/mod.rs` (1101) |
| `parse/` | 1 | 1265 | 0 | `parse/mod.rs` (1265) |
| `proofs/` | 30 | 25316 | 1700 | `proofs/interactive/mod.rs` (2117) |
| `search/` | 47 | 42690 | 7313 | `search/mod.rs` (5552) |
| `sequents/` | 5 | 3009 | 0 | `sequents/notation.rs` (1250) |
| **total** | 107 | 90122 | 10469 | |

## core/src/linear/search

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `additive.rs` | 1 | 514 | 0 | `additive.rs` (514) |
| `basis.rs` | 1 | 595 | 0 | `basis.rs` (595) |
| `batch.rs` | 1 | 987 | 0 | `batch.rs` (987) |
| `classical.rs` | 1 | 302 | 0 | `classical.rs` (302) |
| `essential.rs` | 1 | 945 | 0 | `essential.rs` (945) |
| `focus/` | 19 | 15803 | 2237 | `focus/tests.rs` (2237) |
| `generate.rs` | 1 | 1242 | 1242 | `generate.rs` (1242) |
| `horn/` | 6 | 3949 | 812 | `horn/mod.rs` (866) |
| `inverse/` | 5 | 2736 | 259 | `inverse/mod.rs` (1007) |
| `memory.rs` | 1 | 309 | 0 | `memory.rs` (309) |
| `mod.rs` | 1 | 5552 | 0 | `mod.rs` (5552) |
| `net.rs` | 1 | 1779 | 0 | `net.rs` (1779) |
| `parallel.rs` | 1 | 538 | 0 | `parallel.rs` (538) |
| `planar.rs` | 1 | 626 | 0 | `planar.rs` (626) |
| `reference.rs` | 1 | 1180 | 1180 | `reference.rs` (1180) |
| `reference_fo.rs` | 1 | 1583 | 1583 | `reference_fo.rs` (1583) |
| `refutation.rs` | 1 | 2072 | 0 | `refutation.rs` (2072) |
| `saturated.rs` | 1 | 1342 | 0 | `saturated.rs` (1342) |
| `weighted.rs` | 1 | 501 | 0 | `weighted.rs` (501) |
| `work.rs` | 1 | 135 | 0 | `work.rs` (135) |
| **total** | 47 | 42690 | 7313 | |

## core/src/linear/proofs

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `check/` | 6 | 4401 | 0 | `check/mod.rs` (1425) |
| `cut/` | 3 | 3073 | 869 | `cut/mod.rs` (1525) |
| `derivation/` | 4 | 4121 | 0 | `derivation/classical.rs` (1486) |
| `first_order.rs` | 1 | 1151 | 0 | `first_order.rs` (1151) |
| `fmt.rs` | 1 | 522 | 0 | `fmt.rs` (522) |
| `interactive/` | 9 | 7640 | 0 | `interactive/mod.rs` (2117) |
| `mod.rs` | 1 | 1091 | 0 | `mod.rs` (1091) |
| `multiset.rs` | 1 | 172 | 0 | `multiset.rs` (172) |
| `oracle.rs` | 1 | 831 | 831 | `oracle.rs` (831) |
| `rule.rs` | 1 | 532 | 0 | `rule.rs` (532) |
| `size.rs` | 1 | 1329 | 0 | `size.rs` (1329) |
| `style.rs` | 1 | 453 | 0 | `style.rs` (453) |
| **total** | 30 | 25316 | 1700 | |

## core/src/ordinary

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `bruijn.rs` | 1 | 458 | 0 | `bruijn.rs` (458) |
| `connection/` | 12 | 8206 | 695 | `connection/intuitionistic/read_back.rs` (1476) |
| `decide.rs` | 1 | 1307 | 0 | `decide.rs` (1307) |
| `derivation.rs` | 1 | 2919 | 0 | `derivation.rs` (2919) |
| `equality.rs` | 1 | 505 | 0 | `equality.rs` (505) |
| `grounding.rs` | 1 | 1739 | 0 | `grounding.rs` (1739) |
| `herbrand.rs` | 1 | 2211 | 0 | `herbrand.rs` (2211) |
| `instance.rs` | 1 | 893 | 0 | `instance.rs` (893) |
| `interactive.rs` | 1 | 5900 | 0 | `interactive.rs` (5900) |
| `interpolant.rs` | 1 | 1344 | 0 | `interpolant.rs` (1344) |
| `invert.rs` | 1 | 305 | 0 | `invert.rs` (305) |
| `kripke_search.rs` | 1 | 1068 | 0 | `kripke_search.rs` (1068) |
| `kripke_structure.rs` | 1 | 1274 | 0 | `kripke_structure.rs` (1274) |
| `ljt.rs` | 1 | 1643 | 0 | `ljt.rs` (1643) |
| `mod.rs` | 1 | 2374 | 0 | `mod.rs` (2374) |
| `model.rs` | 1 | 594 | 0 | `model.rs` (594) |
| `models.rs` | 1 | 1429 | 0 | `models.rs` (1429) |
| `negative.rs` | 1 | 733 | 0 | `negative.rs` (733) |
| `normal.rs` | 1 | 3688 | 0 | `normal.rs` (3688) |
| `parse.rs` | 1 | 1411 | 0 | `parse.rs` (1411) |
| `resolution/` | 7 | 5063 | 328 | `resolution/first_order.rs` (2563) |
| `rocq.rs` | 1 | 971 | 0 | `rocq.rs` (971) |
| `rocq_models.rs` | 1 | 552 | 0 | `rocq_models.rs` (552) |
| `sat/` | 5 | 4302 | 0 | `sat/mod.rs` (1477) |
| `structure.rs` | 1 | 845 | 0 | `structure.rs` (845) |
| `table.rs` | 1 | 382 | 0 | `table.rs` (382) |
| `tableau.rs` | 1 | 1109 | 0 | `tableau.rs` (1109) |
| `transfer.rs` | 1 | 326 | 0 | `transfer.rs` (326) |
| `translate.rs` | 1 | 742 | 0 | `translate.rs` (742) |
| `written/` | 2 | 2247 | 407 | `written/mod.rs` (1840) |
| **total** | 52 | 56540 | 1430 | |

## core/src/export

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `latex.rs` | 1 | 867 | 0 | `latex.rs` (867) |
| `mod.rs` | 1 | 697 | 0 | `mod.rs` (697) |
| `models.rs` | 1 | 306 | 0 | `models.rs` (306) |
| `notation.rs` | 1 | 71 | 0 | `notation.rs` (71) |
| `pdf.rs` | 1 | 344 | 0 | `pdf.rs` (344) |
| `png.rs` | 1 | 225 | 0 | `png.rs` (225) |
| `rocq/` | 2 | 2939 | 0 | `rocq/mod.rs` (1636) |
| `styles.rs` | 1 | 203 | 0 | `styles.rs` (203) |
| `svg/` | 7 | 6755 | 0 | `svg/mod.rs` (2438) |
| `typst.rs` | 1 | 1258 | 0 | `typst.rs` (1258) |
| **total** | 17 | 13665 | 0 | |

## core/tests

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `depth.rs` | 1 | 301 | 0 | `depth.rs` (301) |
| `export.rs` | 1 | 3469 | 0 | `export.rs` (3469) |
| `first_order.rs` | 1 | 1763 | 0 | `first_order.rs` (1763) |
| `iltp.rs` | 1 | 265 | 0 | `iltp.rs` (265) |
| `limits.rs` | 1 | 238 | 0 | `limits.rs` (238) |
| `lock.rs` | 1 | 186 | 0 | `lock.rs` (186) |
| `natural.rs` | 1 | 1376 | 0 | `natural.rs` (1376) |
| `parse.rs` | 1 | 501 | 0 | `parse.rs` (501) |
| `rules.rs` | 1 | 1677 | 0 | `rules.rs` (1677) |
| `serialize.rs` | 1 | 1251 | 0 | `serialize.rs` (1251) |
| `stability.rs` | 1 | 592 | 0 | `stability.rs` (592) |
| **total** | 11 | 11619 | 0 | |

## cli

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `src/argument_parsing.rs` | 1 | 3469 | 0 | `src/argument_parsing.rs` (3469) |
| `src/batch.rs` | 1 | 1618 | 0 | `src/batch.rs` (1618) |
| `src/calculus.rs` | 1 | 607 | 0 | `src/calculus.rs` (607) |
| `src/check.rs` | 1 | 1442 | 0 | `src/check.rs` (1442) |
| `src/eliminate.rs` | 1 | 832 | 0 | `src/eliminate.rs` (832) |
| `src/grade.rs` | 1 | 843 | 0 | `src/grade.rs` (843) |
| `src/hint.rs` | 1 | 235 | 0 | `src/hint.rs` (235) |
| `src/interact/` | 3 | 2494 | 0 | `src/interact/mod.rs` (1290) |
| `src/io.rs` | 1 | 992 | 0 | `src/io.rs` (992) |
| `src/lib.rs` | 1 | 388 | 0 | `src/lib.rs` (388) |
| `src/limit.rs` | 1 | 232 | 0 | `src/limit.rs` (232) |
| `src/main.rs` | 1 | 16 | 0 | `src/main.rs` (16) |
| `src/modal.rs` | 1 | 535 | 0 | `src/modal.rs` (535) |
| `src/ordinary.rs` | 1 | 1420 | 0 | `src/ordinary.rs` (1420) |
| `src/prove.rs` | 1 | 963 | 0 | `src/prove.rs` (963) |
| `src/resolve.rs` | 1 | 611 | 0 | `src/resolve.rs` (611) |
| `src/seq.rs` | 1 | 151 | 0 | `src/seq.rs` (151) |
| `src/settings.rs` | 1 | 1177 | 0 | `src/settings.rs` (1177) |
| `src/show.rs` | 1 | 1733 | 0 | `src/show.rs` (1733) |
| `src/style.rs` | 1 | 113 | 0 | `src/style.rs` (113) |
| `src/tableau.rs` | 1 | 144 | 0 | `src/tableau.rs` (144) |
| `src/term.rs` | 1 | 577 | 0 | `src/term.rs` (577) |
| `src/transform.rs` | 1 | 533 | 0 | `src/transform.rs` (533) |
| `src/workspace.rs` | 1 | 558 | 0 | `src/workspace.rs` (558) |
| `tests/cli.rs` | 1 | 3887 | 3887 | `tests/cli.rs` (3887) |
| `tests/common/` | 1 | 71 | 71 | `tests/common/mod.rs` (71) |
| `tests/lock.rs` | 1 | 1527 | 1527 | `tests/lock.rs` (1527) |
| `tests/readme.rs` | 1 | 974 | 974 | `tests/readme.rs` (974) |
| **total** | 30 | 28142 | 6459 | |

## bench

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `src/compare.rs` | 1 | 548 | 0 | `src/compare.rs` (548) |
| `src/growth.rs` | 1 | 160 | 0 | `src/growth.rs` (160) |
| `src/journeys.rs` | 1 | 513 | 0 | `src/journeys.rs` (513) |
| `src/main.rs` | 1 | 528 | 0 | `src/main.rs` (528) |
| `src/ordinary.rs` | 1 | 564 | 0 | `src/ordinary.rs` (564) |
| `src/problems.rs` | 1 | 492 | 0 | `src/problems.rs` (492) |
| `src/ranking.rs` | 1 | 564 | 0 | `src/ranking.rs` (564) |
| `src/ratchet.rs` | 1 | 440 | 0 | `src/ratchet.rs` (440) |
| `src/run.rs` | 1 | 1259 | 0 | `src/run.rs` (1259) |
| `src/summary.rs` | 1 | 753 | 0 | `src/summary.rs` (753) |
| `src/tools.rs` | 1 | 1480 | 0 | `src/tools.rs` (1480) |
| `src/translate.rs` | 1 | 682 | 0 | `src/translate.rs` (682) |
| `tests/compare.rs` | 1 | 199 | 199 | `tests/compare.rs` (199) |
| `tests/manual.rs` | 1 | 229 | 229 | `tests/manual.rs` (229) |
| **total** | 14 | 8411 | 428 | |

## web

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `build.rs` | 1 | 17 | 0 | `build.rs` (17) |
| `src/calls.rs` | 1 | 2609 | 0 | `src/calls.rs` (2609) |
| `src/lambda.rs` | 1 | 270 | 0 | `src/lambda.rs` (270) |
| `src/lib.rs` | 1 | 847 | 0 | `src/lib.rs` (847) |
| `src/modal.rs` | 1 | 205 | 0 | `src/modal.rs` (205) |
| `src/ordinary.rs` | 1 | 1345 | 0 | `src/ordinary.rs` (1345) |
| `src/protocol.rs` | 1 | 504 | 0 | `src/protocol.rs` (504) |
| `src/rules.rs` | 1 | 831 | 0 | `src/rules.rs` (831) |
| `src/tab.rs` | 1 | 22 | 0 | `src/tab.rs` (22) |
| `src/workspace.rs` | 1 | 610 | 0 | `src/workspace.rs` (610) |
| `tests/calls.rs` | 1 | 1478 | 1478 | `tests/calls.rs` (1478) |
| `tests/lambda.rs` | 1 | 125 | 125 | `tests/lambda.rs` (125) |
| `tests/modal.rs` | 1 | 78 | 78 | `tests/modal.rs` (78) |
| `tests/ordinary.rs` | 1 | 712 | 712 | `tests/ordinary.rs` (712) |
| `tests/protocol.rs` | 1 | 128 | 128 | `tests/protocol.rs` (128) |
| `tests/rules.rs` | 1 | 417 | 417 | `tests/rules.rs` (417) |
| `tests/workspace.rs` | 1 | 164 | 164 | `tests/workspace.rs` (164) |
| **total** | 17 | 10362 | 3102 | |

## web-render

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `build.rs` | 1 | 16 | 0 | `build.rs` (16) |
| `src/lib.rs` | 1 | 63 | 0 | `src/lib.rs` (63) |
| `tests/render.rs` | 1 | 82 | 82 | `tests/render.rs` (82) |
| **total** | 3 | 161 | 82 | |

## web-tab

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `src/failure.rs` | 1 | 67 | 0 | `src/failure.rs` (67) |
| `src/lib.rs` | 1 | 207 | 0 | `src/lib.rs` (207) |
| **total** | 2 | 274 | 0 | |

## compat

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `linlog-cli/src/` | 1 | 13 | 0 | `linlog-cli/src/main.rs` (13) |
| `linlog/src/` | 1 | 24 | 0 | `linlog/src/lib.rs` (24) |
| **total** | 2 | 37 | 0 | |

## The 40 largest Rust files

| lines | file |
|---:|---|
| 5900 | `core/src/ordinary/interactive.rs` |
| 5552 | `core/src/linear/search/mod.rs` |
| 3887 | `cli/tests/cli.rs` |
| 3688 | `core/src/ordinary/normal.rs` |
| 3469 | `core/tests/export.rs` |
| 3469 | `cli/src/argument_parsing.rs` |
| 2919 | `core/src/ordinary/derivation.rs` |
| 2609 | `web/src/calls.rs` |
| 2563 | `core/src/ordinary/resolution/first_order.rs` |
| 2438 | `core/src/export/svg/mod.rs` |
| 2374 | `core/src/ordinary/mod.rs` |
| 2355 | `core/src/rules/interactive.rs` |
| 2237 | `core/src/linear/search/focus/tests.rs` |
| 2226 | `core/src/linear/search/focus/mod.rs` |
| 2211 | `core/src/ordinary/herbrand.rs` |
| 2177 | `core/src/linear/nets/mod.rs` |
| 2119 | `core/src/linear/search/focus/framed.rs` |
| 2117 | `core/src/linear/proofs/interactive/mod.rs` |
| 2072 | `core/src/linear/search/refutation.rs` |
| 2065 | `core/src/serialize/ordinary.rs` |
| 1959 | `core/src/modal/search.rs` |
| 1939 | `core/src/protocol/ordinary.rs` |
| 1927 | `core/src/workspace/mod.rs` |
| 1840 | `core/src/ordinary/written/mod.rs` |
| 1820 | `core/src/errors/mod.rs` |
| 1779 | `core/src/linear/search/net.rs` |
| 1777 | `core/src/export/svg/mell.rs` |
| 1763 | `core/tests/first_order.rs` |
| 1739 | `core/src/ordinary/grounding.rs` |
| 1733 | `cli/src/show.rs` |
| 1677 | `core/tests/rules.rs` |
| 1653 | `core/src/lambda/tests.rs` |
| 1643 | `core/src/ordinary/ljt.rs` |
| 1636 | `core/src/export/rocq/mod.rs` |
| 1618 | `cli/src/batch.rs` |
| 1583 | `core/src/linear/search/reference_fo.rs` |
| 1544 | `core/src/lambda/parse.rs` |
| 1527 | `cli/tests/lock.rs` |
| 1525 | `core/src/linear/proofs/cut/mod.rs` |
| 1486 | `core/src/linear/proofs/derivation/classical.rs` |

Files: 332; over 1 000 lines: 95; over 3 000: 6; over 5 000: 2.

## rocq-deduxit theories

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `Intuitionistic/` | 9 | 1965 | 0 | `Intuitionistic/FirstOrderCheck.v` (489) |
| `Linear/` | 20 | 5682 | 0 | `Linear/FirstOrderIntuitionisticCheck.v` (770) |
| `Modal/` | 7 | 1318 | 0 | `Modal/Soundness.v` (371) |
| **total** | 36 | 8965 | 0 | |

| lines | file |
|---:|---|
| 770 | `theories/Linear/FirstOrderIntuitionisticCheck.v` |
| 745 | `theories/Linear/FirstOrderSoundness.v` |
| 697 | `theories/Linear/Weights.v` |
| 534 | `theories/Linear/Refutations.v` |
| 505 | `theories/Linear/IntuitionisticCheck.v` |
| 501 | `theories/Linear/FirstOrderWeights.v` |
| 489 | `theories/Intuitionistic/FirstOrderCheck.v` |
| 422 | `theories/Intuitionistic/FirstOrderKripke.v` |
| 371 | `theories/Modal/Soundness.v` |
| 352 | `theories/Intuitionistic/FirstOrderClassical.v` |
| 342 | `theories/Linear/Soundness.v` |
| 327 | `theories/Linear/FirstOrderCheck.v` |
| 287 | `theories/Modal/Countermodel.v` |
| 272 | `theories/Modal/Check.v` |
| 227 | `theories/Intuitionistic/KripkeCheck.v` |
| 200 | `theories/Linear/Check.v` |
| 176 | `theories/Modal/Kripke.v` |
| 157 | `theories/Intuitionistic/Kripke.v` |
| 149 | `theories/Linear/FirstOrderIntuitionistic.v` |
| 120 | `theories/Linear/Intuitionistic.v` |
| 120 | `theories/Intuitionistic/FirstOrderCalculus.v` |
| 118 | `theories/Linear/TermsCheck.v` |
| 117 | `theories/Modal/Calculus.v` |
| 115 | `theories/Linear/Formulas.v` |
| 100 | `theories/Linear/FirstOrderFormulas.v` |
| 99 | `theories/Linear/Mix.v` |
| 82 | `theories/Linear/FirstOrderCalculus.v` |
| 81 | `theories/Intuitionistic/FirstOrderFormulas.v` |
| 78 | `theories/Intuitionistic/Calculus.v` |
| 77 | `theories/Linear/Calculus.v` |
| 73 | `theories/Linear/FirstOrderProofs.v` |
| 65 | `theories/Linear/Proofs.v` |
| 63 | `theories/Linear/Terms.v` |
| 62 | `theories/Modal/Labelled.v` |
| 39 | `theories/Intuitionistic/Formulas.v` |
| 33 | `theories/Modal/Formulas.v` |

## rocq-deduxit tests

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `Assumptions.v` | 1 | 71 | 0 | `Assumptions.v` (71) |
| `Certificates.v` | 1 | 114 | 0 | `Certificates.v` (114) |
| `Cuts.v` | 1 | 92 | 0 | `Cuts.v` (92) |
| `FirstOrder.v` | 1 | 141 | 0 | `FirstOrder.v` (141) |
| `FirstOrderTwoSided.v` | 1 | 77 | 0 | `FirstOrderTwoSided.v` (77) |
| `FirstOrderWeights.v` | 1 | 67 | 0 | `FirstOrderWeights.v` (67) |
| `FromCommand.v` | 1 | 75 | 0 | `FromCommand.v` (75) |
| `Intuitionistic/` | 2 | 374 | 0 | `Intuitionistic/FirstOrder.v` (231) |
| `Modal/` | 2 | 317 | 0 | `Modal/Certificates.v` (201) |
| `Modes.v` | 1 | 57 | 0 | `Modes.v` (57) |
| `Refutations.v` | 1 | 112 | 0 | `Refutations.v` (112) |
| `TwoSided.v` | 1 | 137 | 0 | `TwoSided.v` (137) |
| `Weights.v` | 1 | 160 | 0 | `Weights.v` (160) |
| **total** | 15 | 1794 | 0 | |

## rocq-deduxit bridges

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `Assumptions.v` | 1 | 13 | 0 | `Assumptions.v` (13) |
| `NanoIllBridge.v` | 1 | 169 | 0 | `NanoIllBridge.v` (169) |
| `NanoYallaBridge.v` | 1 | 154 | 0 | `NanoYallaBridge.v` (154) |
| **total** | 3 | 336 | 0 | |
