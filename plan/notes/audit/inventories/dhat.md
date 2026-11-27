# Each journey's allocations (DHAT)

| journey | bytes allocated | blocks | instructions | instructions per block |
|---|---:|---:|---:|---:|
| batch-families | 16576774 | 9154 | 57426209 | 6273 |
| check-qbf-20-2 | 39259838 | 129111 | 649523554 | 5030 |
| check-saturated | 3977261 | 82267 | 59257201 | 720 |
| check-wide-m1-2048 | 33620248 | 72324 | 15983008107 | 220991 |
| cut-elimination | 3453426 | 17784 | 12782987 | 718 |
| derivation-chain-64 | 2716519 | 2873 | 146411063 | 50961 |
| lambda-check | 5309326 | 20406 | 37739945 | 1849 |
| lambda-inhabit | 4446330 | 1634 | 1487759 | 910 |
| lambda-reduce | 7905478 | 60863 | 29280960 | 481 |
| modal-tableau | 2094721 | 27807 | 7593591 | 273 |
| ordinary-connection-classical | 1031129 | 13682 | 9965786 | 728 |
| ordinary-connection-intuitionistic | 2020646 | 16794 | 8369051 | 498 |
| ordinary-kripke | 12803890 | 1238 | 1409610 | 1138 |
| ordinary-ljt | 5232284 | 43980 | 30096144 | 684 |
| ordinary-models | 12840302 | 1800 | 1594012 | 885 |
| ordinary-pigeons | 5956104 | 60124 | 27132599 | 451 |
| ordinary-resolution-first-order | 812836 | 17192 | 6729317 | 391 |
| ordinary-resolution | 902159 | 12389 | 9237382 | 745 |
| ordinary-sat-classical | 4920598 | 6890 | 22016138 | 3195 |
| ordinary-sat-intuitionistic | 8584339 | 124685 | 29112507 | 233 |
| read-json | 4793827 | 18951 | 37194661 | 1962 |
| read-lltp | 4854631 | 10465 | 32033704 | 3061 |
| read-spec | 4790489 | 12440 | 24061243 | 1934 |
| read-text | 5805958 | 14919 | 34574936 | 2317 |
| read-tptp | 3355368 | 18418 | 35454205 | 1924 |
| render-latex | 2323842 | 3042 | 144088207 | 47366 |
| render-svg | 12803657 | 52499 | 225079599 | 4287 |
| render-typst | 2369054 | 3046 | 143223702 | 47020 |
| rules-check | 8630188 | 144458 | 60678395 | 420 |
| search-additive-14 | 104927998 | 167253 | 681359300 | 4073 |
| search-chain-128 | 1887513 | 3257 | 1065902167 | 327265 |
| search-chain-64-intuitionistic | 1482145 | 1797 | 135580397 | 75448 |
| search-chain-fo-32 | 2824616 | 19369 | 25575821 | 1320 |
| search-inverse | 641266 | 5440 | 6515967 | 1197 |
| search-partition-no-5 | 1309622 | 833 | 8668291 | 10406 |
| search-qbf-20-2 | 24434593 | 1749 | 516873622 | 295525 |
| search-spec-chain | 8165203 | 52685 | 43053599 | 817 |
| search-wide-m2-256 | 1635840 | 6439 | 253251523 | 39330 |
| session-linear | 17589038 | 4715 | 3426899 | 726 |
| session-ordinary | 7401745 | 77414 | 32836497 | 424 |
| session-rules | 16238705 | 142414 | 28946024 | 203 |
| workspace-run | 4796215 | 5146 | 4106831 | 798 |

## batch-families: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 1086 | 11 % | 24928 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 781 | 8 % | 20236 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 700 | 7 % | 50714 | `deduxit::linear::families::additive_tree (families.rs:962)` |
| 502 | 5 % | 2480 | `deduxit::linear::families::additive_tree (families.rs:957)` |
| 445 | 4 % | 21268 | `deduxit::linear::parse::read (mod.rs:209)` |
| 376 | 4 % | 12284 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |
| 261 | 2 % | 6100 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 216 | 2 % | 3592 | `{closure#0}<deduxit::linear::search::focus::zone::Ground> (scratch.rs:60)` |

## check-qbf-20-2: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 64896 | 50 % | 3618816 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, deduxit::first_order::Eigen)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, deduxit::first_order::Eigen, foldhash::fast::FixedState>::` |
| 62463 | 48 % | 8994672 | `new<deduxit::linear::proofs::check::zones::State> (boxed.rs:302)` |
| 254 | 0 % | 5996 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 236 | 0 % | 10956 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 120 | 0 % | 4706 | `{closure#0}<&[(u32, bool); 3], alloc::string::String, (), deduxit::linear::families::qbf::{closure_env#4}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#` |
| 103 | 0 % | 7232 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |
| 102 | 0 % | 2650 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 92 | 0 % | 5888 | `{closure#0}<deduxit::linear::search::focus::zone::Ground> (scratch.rs:60)` |

## check-saturated: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 39242 | 47 % | 917804 | `with_capacity_in<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:187)` |
| 8298 | 10 % | 82956 | `new_uninit<[deduxit::linear::occurrences::OccId; 1]> (boxed.rs:334)` |
| 8154 | 9 % | 130464 | `insert_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:2273)` |
| 5192 | 6 % | 122992 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |
| 4061 | 4 % | 909664 | `<alloc::raw_vec::RawVec<deduxit::kripke::World>>::grow_one (mod.rs:198)` |
| 4041 | 4 % | 64656 | `<deduxit::linear::search::inverse::index::Trie>::walk::<<deduxit::linear::search::inverse::index::Trie>::ends_within<<deduxit::linear::search::inverse::Saturation>::subsumed::{closure#0}>::{closure#0}> (index.rs:77)` |
| 2304 | 2 % | 36864 | `walk<deduxit::linear::search::inverse::index::{impl#0}::within::{closure_env#0}<deduxit::linear::search::inverse::{impl#2}::subsumed::{closure_env#0}>> (index.rs:77)` |
| 1728 | 2 % | 36864 | `from_iter<u32, core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::flatten::Flatten<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<deduxit::linear::occurrenc` |

## check-wide-m1-2048: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 32768 | 45 % | 22467928 | `new_uninit<[deduxit::linear::occurrences::OccId; 1]> (boxed.rs:334)` |
| 12291 | 16 % | 349692 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 6150 | 8 % | 253948 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 4117 | 5 % | 360676 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, deduxit::first_order::Eigen)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, deduxit::first_order::Eigen, foldhash::fast::FixedState>::` |
| 4107 | 5 % | 214772 | `deduxit::linear::parse::read (mod.rs:209)` |
| 4096 | 5 % | 589824 | `new<deduxit::linear::proofs::check::zones::State> (boxed.rs:302)` |
| 3096 | 4 % | 58016 | `{closure#0}<u32, alloc::string::String, (), deduxit::linear::families::wide::{closure_env#0}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#22}::extend_t` |
| 3096 | 4 % | 58016 | `{closure#0}<u32, alloc::string::String, (), deduxit::linear::families::wide::{closure_env#1}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#22}::extend_t` |

## cut-elimination: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 7144 | 40 % | 368852 | `with_capacity_in<deduxit::linear::occurrences::Member, alloc::alloc::Global> (mod.rs:187)` |
| 2128 | 11 % | 120160 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, deduxit::first_order::Eigen)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, deduxit::first_order::Eigen, foldhash::fast::FixedState>::` |
| 1949 | 10 % | 280656 | `new<deduxit::linear::proofs::check::zones::State> (boxed.rs:302)` |
| 828 | 4 % | 119232 | `try_new_uninit_in<deduxit::linear::proofs::check::zones::State, alloc::alloc::Global> (boxed.rs:552)` |
| 828 | 4 % | 43056 | `new_uninitialized<(deduxit::linear::occurrences::Member, u32), alloc::alloc::Global> (src/raw.rs:637)` |
| 805 | 4 % | 28980 | `new_uninitialized<(deduxit::linear::occurrences::Member, ()), alloc::alloc::Global> (src/raw.rs:637)` |
| 447 | 2 % | 27436 | `with_capacity_in<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:187)` |
| 417 | 2 % | 508008 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, alloc::string::String)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, alloc::string::String, foldhash::fast::FixedState>::{closure#0}>` |

## derivation-chain-64: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 555 | 19 % | 47060 | `with_capacity_in<deduxit::linear::occurrences::Member, alloc::alloc::Global> (mod.rs:187)` |
| 500 | 17 % | 49600 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 197 | 6 % | 5160 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 134 | 4 % | 3544 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 130 | 4 % | 652 | `with_capacity_in<deduxit::linear::proofs::derivation::view::InfId, alloc::alloc::Global> (mod.rs:187)` |
| 103 | 3 % | 1896 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |
| 90 | 3 % | 12888 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, deduxit::first_order::Eigen)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, deduxit::first_order::Eigen, foldhash::fast::FixedState>::` |
| 89 | 3 % | 8544 | `<alloc::raw_vec::RawVec<alloc::vec::Vec<alloc::vec::Vec<deduxit::rules::terms::TermId>>>>::grow_one (mod.rs:198)` |

## lambda-check: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 8002 | 39 % | 8002 | `<deduxit::lambda::parse::Parser>::name (parse.rs:329)` |
| 4001 | 19 % | 4001 | `<deduxit::lambda::Builder>::named_variable (mod.rs:1247)` |
| 4001 | 19 % | 4001 | `<deduxit::lambda::parse::Parser>::bind (parse.rs:353)` |
| 4001 | 19 % | 336084 | `<hashbrown::raw::RawTable<(deduxit::first_order::FoTermId, u64)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::FoTermId, u64, foldhash::fast::FixedState>::{closure#0}> (src/raw.rs:962)` |
| 76 | 0 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 62 | 0 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |
| 52 | 0 % | 7994 | `<deduxit_bench::Cli as clap_builder::derive::Parser>::parse (src/derive.rs:32)` |
| 28 | 0 % | 1688 | `<deduxit_bench::OneArgs as clap_builder::derive::Args>::augment_args (main.rs:341)` |

## lambda-inhabit: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 388 | 23 % | 2420 | `with_capacity_in<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:187)` |
| 192 | 11 % | 3440 | `<deduxit::lambda::inhabit::Search>::terms (inhabit.rs:1213)` |
| 76 | 4 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 64 | 3 % | 80 | `<deduxit::lambda::Builder>::named_variable (mod.rs:1247)` |
| 62 | 3 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |
| 59 | 3 % | 460 | `new_uninit<[deduxit::ordinary::NodeId; 1]> (boxed.rs:334)` |
| 52 | 3 % | 8000 | `<deduxit_bench::Cli as clap_builder::derive::Parser>::parse (src/derive.rs:32)` |
| 38 | 2 % | 368 | `with_capacity_in<&deduxit::ordinary::derivation::Inference, alloc::alloc::Global> (mod.rs:187)` |

## lambda-reduce: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 10635 | 17 % | 3394720 | `<alloc::raw_vec::RawVec<core::option::Option<std::collections::hash::map::HashMap<deduxit::lambda::VarId, u64, foldhash::fast::FixedState>>>>::grow_one (mod.rs:198)` |
| 10198 | 16 % | 40792 | `new_uninit<[deduxit::lambda::VarId; 1]> (boxed.rs:334)` |
| 10001 | 16 % | 61008 | `with_capacity_in<deduxit::lambda::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 5344 | 8 % | 13478 | `deduxit::lambda::reduce::copied (reduce.rs:653)` |
| 5151 | 8 % | 432684 | `<hashbrown::raw::RawTable<(deduxit::first_order::FoTermId, u64)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::FoTermId, u64, foldhash::fast::FixedState>::{closure#0}> (src/raw.rs:962)` |
| 5050 | 8 % | 5050 | `<deduxit::lambda::reduce::Copier>::fresh (reduce.rs:406)` |
| 5050 | 8 % | 242400 | `push_mut<(deduxit::lambda::VarId, core::option::Option<deduxit::lambda::VarId>), alloc::alloc::Global> (mod.rs:1000)` |
| 5047 | 8 % | 181692 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, ())>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, (), foldhash::fast::FixedState>::{closure#0}> (src/raw.rs:962)` |

## modal-tableau: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 14082 | 50 % | 163688 | `to_vec<deduxit::modal::search::World, alloc::alloc::Global> (slice.rs:432)` |
| 7540 | 27 % | 90480 | `new_uninit<[deduxit::modal::search::Member; 1]> (boxed.rs:334)` |
| 1069 | 3 % | 16128 | `with_capacity_in<deduxit::modal::search::Member, alloc::alloc::Global> (mod.rs:187)` |
| 925 | 3 % | 61824 | `with_capacity_in<(u32, deduxit::modal::NodeId, bool), alloc::alloc::Global> (mod.rs:187)` |
| 786 | 2 % | 75456 | `reserve<alloc::vec::Vec<deduxit::modal::search::Member, alloc::alloc::Global>, alloc::alloc::Global> (mod.rs:358)` |
| 600 | 2 % | 14400 | `new_uninit<[deduxit::modal::search::Member; 2]> (boxed.rs:334)` |
| 447 | 1 % | 807392 | `with_capacity_in<deduxit::modal::search::World, alloc::alloc::Global> (mod.rs:187)` |
| 433 | 1 % | 20784 | `new_uninit<[alloc::vec::Vec<deduxit::modal::search::Member, alloc::alloc::Global>; 2]> (boxed.rs:334)` |

## ordinary-connection-classical: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 4596 | 33 % | 151428 | `with_capacity_in<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:187)` |
| 2600 | 19 % | 41600 | `from_iter<((u32, u32), (u32, u32)), core::iter::adapters::map::Map<core::ops::range::Range<u32>, deduxit::ordinary::connection::terms::{impl#2}::equal::{closure_env#0}>> (spec_from_iter_nested.rs:52)` |
| 649 | 4 % | 5384 | `with_capacity_in<&deduxit::ordinary::derivation::Inference, alloc::alloc::Global> (mod.rs:187)` |
| 600 | 4 % | 9600 | `from_iter<((u32, u32), (u32, u32)), core::iter::adapters::map::Map<core::ops::range::Range<u32>, deduxit::ordinary::connection::terms::{impl#2}::unify::{closure_env#0}>> (spec_from_iter_nested.rs:52)` |
| 600 | 4 % | 2400 | `new_uninit<[deduxit::linear::proofs::derivation::view::InfId; 1]> (boxed.rs:334)` |
| 519 | 3 % | 51872 | `<alloc::raw_vec::RawVec<[deduxit::linear::sequents::term::TermId; 2]>>::grow_one (mod.rs:198)` |
| 463 | 3 % | 5852 | `with_capacity_in<deduxit::first_order::FoTermId, alloc::alloc::Global> (mod.rs:187)` |
| 344 | 2 % | 10480 | `write_atom<deduxit::ordinary::NodeId, alloc::string::String> (first_order.rs:341)` |

## ordinary-connection-intuitionistic: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 9904 | 58 % | 810480 | `with_capacity_in<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:187)` |
| 2093 | 12 % | 338824 | `deduxit::ordinary::connection::intuitionistic::prefix::successors::{closure#0} (prefix.rs:700)` |
| 1224 | 7 % | 9984 | `with_capacity_in<&deduxit::ordinary::derivation::Inference, alloc::alloc::Global> (mod.rs:187)` |
| 1200 | 7 % | 4800 | `new_uninit<[deduxit::linear::proofs::derivation::view::InfId; 1]> (boxed.rs:334)` |
| 179 | 1 % | 7536 | `<deduxit::ordinary::connection::intuitionistic::prefix::Prefixes>::chain (prefix.rs:208)` |
| 174 | 1 % | 5912 | `<deduxit::ordinary::connection::intuitionistic::prove::Search>::prefixes_unify (prove.rs:979)` |
| 149 | 0 % | 66752 | `<alloc::raw_vec::RawVec<deduxit::rules::interactive::Goal>>::grow_one (mod.rs:198)` |
| 123 | 0 % | 5784 | `<deduxit::ordinary::connection::intuitionistic::prefix::State>::resolve (prefix.rs:567)` |

## ordinary-kripke: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 101 | 8 % | 13560 | `new_var<deduxit::ordinary::sat::solver::Bridge> (src/core.rs:256)` |
| 83 | 6 % | 712 | `from_iter<batsat::clause::Lit, core::iter::adapters::map::Map<core::slice::iter::Iter<deduxit::ordinary::sat::solver::Lit>, deduxit::ordinary::sat::solver::{impl#4}::add_clause::{closure_env#0}>> (spec_from_iter_nested.r` |
| 81 | 6 % | 2592 | `add_clause_reuse<deduxit::ordinary::sat::solver::Bridge> (src/core.rs:281)` |
| 76 | 6 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 73 | 5 % | 10368 | `<alloc::raw_vec::RawVec<alloc::vec::Vec<alloc::vec::Vec<deduxit::rules::terms::TermId>>>>::grow_one (mod.rs:198)` |
| 62 | 5 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |
| 52 | 4 % | 8003 | `<deduxit_bench::Cli as clap_builder::derive::Parser>::parse (src/derive.rs:32)` |
| 43 | 3 % | 688 | `new_uninit<[deduxit::ordinary::sat::solver::Lit; 2]> (boxed.rs:334)` |

## ordinary-ljt: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 25569 | 58 % | 3850300 | `with_capacity_in<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:187)` |
| 5334 | 12 % | 21336 | `new_uninit<[deduxit::ordinary::NodeId; 1]> (boxed.rs:334)` |
| 4863 | 11 % | 175068 | `<hashbrown::raw::RawTable<(deduxit::ordinary::NodeId, ())>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::ordinary::NodeId, (), foldhash::fast::FixedState>::{closure#0}> (src/raw.rs:962)` |
| 3164 | 7 % | 26064 | `with_capacity_in<&deduxit::ordinary::derivation::Inference, alloc::alloc::Global> (mod.rs:187)` |
| 3070 | 6 % | 12280 | `new_uninit<[deduxit::linear::proofs::derivation::view::InfId; 1]> (boxed.rs:334)` |
| 623 | 1 % | 10144 | `push_mut<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:1000)` |
| 128 | 0 % | 3358 | `<deduxit::ordinary::Formulas as core::clone::Clone>::clone (mod.rs:777)` |
| 128 | 0 % | 8814 | `<deduxit::ordinary::Formulas as core::clone::Clone>::clone (mod.rs:779)` |

## ordinary-models: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 162 | 9 % | 5184 | `add_clause_reuse<deduxit::ordinary::sat::solver::Bridge> (src/core.rs:281)` |
| 142 | 7 % | 1220 | `from_iter<batsat::clause::Lit, core::iter::adapters::map::Map<core::slice::iter::Iter<deduxit::ordinary::sat::solver::Lit>, deduxit::ordinary::sat::solver::{impl#4}::add_clause::{closure_env#0}>> (spec_from_iter_nested.r` |
| 124 | 6 % | 26552 | `new_var<deduxit::ordinary::sat::solver::Bridge> (src/core.rs:256)` |
| 104 | 5 % | 936 | `with_capacity_in<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:187)` |
| 101 | 5 % | 17856 | `<alloc::raw_vec::RawVec<alloc::vec::Vec<alloc::vec::Vec<deduxit::rules::terms::TermId>>>>::grow_one (mod.rs:198)` |
| 81 | 4 % | 1296 | `new_uninit<[deduxit::ordinary::sat::solver::Lit; 2]> (boxed.rs:334)` |
| 76 | 4 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 62 | 3 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |

## ordinary-pigeons: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 27910 | 46 % | 360936 | `with_capacity_in<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:187)` |
| 9769 | 16 % | 348080 | `with_capacity_in<deduxit::linear::occurrences::Member, alloc::alloc::Global> (mod.rs:187)` |
| 8271 | 13 % | 55756 | `with_capacity_in<deduxit::linear::proofs::derivation::view::InfId, alloc::alloc::Global> (mod.rs:187)` |
| 4647 | 7 % | 218656 | `with_capacity_in<(deduxit::ordinary::NodeId, deduxit::ordinary::derivation::Side), alloc::alloc::Global> (mod.rs:187)` |
| 4135 | 6 % | 37168 | `with_capacity_in<&deduxit::ordinary::derivation::Inference, alloc::alloc::Global> (mod.rs:187)` |
| 3076 | 5 % | 307480 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, deduxit::first_order::Eigen)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, deduxit::first_order::Eigen, foldhash::fast::FixedState>::` |
| 1024 | 1 % | 147456 | `new<deduxit::linear::proofs::check::zones::State> (boxed.rs:302)` |
| 512 | 0 % | 32768 | `with_capacity_in<(usize, alloc::vec::Vec<(deduxit::ordinary::NodeId, deduxit::ordinary::derivation::Side), alloc::alloc::Global>), alloc::alloc::Global> (mod.rs:187)` |

## ordinary-resolution-first-order: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 3782 | 21 % | 19344 | `with_capacity_in<deduxit::first_order::FoTermId, alloc::alloc::Global> (mod.rs:187)` |
| 3742 | 21 % | 45552 | `new_uninit<[(deduxit::ordinary::NodeId, u32, bool); 1]> (boxed.rs:334)` |
| 2185 | 12 % | 54112 | `with_capacity_in<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:187)` |
| 724 | 4 % | 24608 | `new_uninit<[(deduxit::first_order::FoTermId, bool); 1]> (boxed.rs:334)` |
| 718 | 4 % | 16856 | `write_atom<deduxit::ordinary::NodeId, alloc::string::String> (first_order.rs:341)` |
| 570 | 3 % | 35392 | `<alloc::raw_vec::RawVec<[deduxit::linear::sequents::term::TermId; 2]>>::grow_one (mod.rs:198)` |
| 557 | 3 % | 46820 | `<hashbrown::raw::RawTable<(deduxit::linear::search::focus::framed::TRef, ())>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::linear::search::focus::framed::TRef, (), foldhash::fast::FixedState>::{closure#0}> (sr` |
| 408 | 2 % | 3264 | `with_capacity_in<(deduxit::first_order::FoTermId, deduxit::first_order::FoTermId), alloc::alloc::Global> (mod.rs:187)` |

## ordinary-resolution: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 11795 | 95 % | 163864 | `with_capacity_in<deduxit::ordinary::resolution::Literal, alloc::alloc::Global> (mod.rs:187)` |
| 76 | 0 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 66 | 0 % | 149 | `<deduxit::ordinary::resolution::refutation::Refutation>::resolve (refutation.rs:154)` |
| 62 | 0 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |
| 52 | 0 % | 8015 | `<deduxit_bench::Cli as clap_builder::derive::Parser>::parse (src/derive.rs:32)` |
| 44 | 0 % | 544 | `push_mut<deduxit::ordinary::resolution::Literal, alloc::alloc::Global> (mod.rs:1000)` |
| 28 | 0 % | 1688 | `<deduxit_bench::OneArgs as clap_builder::derive::Args>::augment_args (main.rs:341)` |
| 26 | 0 % | 630 | `<deduxit::ordinary::resolution::Clauses as core::clone::Clone>::clone (mod.rs:174)` |

## ordinary-sat-classical: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 1723 | 25 % | 123368 | `<deduxit::ordinary::sat::drup::Checker>::keep (drup.rs:342)` |
| 857 | 12 % | 11996 | `from_iter<u32, core::iter::adapters::map::Map<core::slice::iter::Iter<deduxit::ordinary::sat::solver::Lit>, deduxit::ordinary::sat::drup::{impl#4}::normal::{closure_env#1}>> (spec_from_iter_nested.rs:52)` |
| 630 | 9 % | 36800 | `add_clause_reuse<deduxit::ordinary::sat::solver::Bridge> (src/core.rs:281)` |
| 510 | 7 % | 11592 | `deduxit_bench::journeys::ordinary_logic::pigeonhole (ordinary_logic.rs:32)` |
| 438 | 6 % | 16064 | `with_capacity_in<deduxit::ordinary::sat::solver::Lit, alloc::alloc::Global> (mod.rs:187)` |
| 419 | 6 % | 3964 | `from_iter<batsat::clause::Lit, core::iter::adapters::map::Map<core::slice::iter::Iter<deduxit::ordinary::sat::solver::Lit>, deduxit::ordinary::sat::solver::{impl#4}::add_clause::{closure_env#0}>> (spec_from_iter_nested.r` |
| 365 | 5 % | 22496 | `<deduxit::ordinary::sat::drup::Checker>::keep (drup.rs:338)` |
| 311 | 4 % | 125472 | `<alloc::raw_vec::RawVec<alloc::vec::Vec<alloc::vec::Vec<deduxit::rules::terms::TermId>>>>::grow_one (mod.rs:198)` |

## ordinary-sat-intuitionistic: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 77581 | 62 % | 405608 | `with_capacity_in<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:187)` |
| 22310 | 17 % | 89240 | `new_uninit<[deduxit::ordinary::NodeId; 1]> (boxed.rs:334)` |
| 9382 | 7 % | 112280 | `with_capacity_in<&deduxit::ordinary::derivation::Inference, alloc::alloc::Global> (mod.rs:187)` |
| 4729 | 3 % | 18916 | `new_uninit<[deduxit::linear::proofs::derivation::view::InfId; 1]> (boxed.rs:334)` |
| 4653 | 3 % | 37224 | `new_uninit<[deduxit::linear::proofs::derivation::view::InfId; 2]> (boxed.rs:334)` |
| 4606 | 3 % | 75040 | `push_mut<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:1000)` |
| 186 | 0 % | 6624 | `add_clause_reuse<deduxit::ordinary::sat::solver::Bridge> (src/core.rs:281)` |
| 165 | 0 % | 4736 | `deduxit_bench::journeys::ordinary_logic::pigeonhole (ordinary_logic.rs:32)` |

## read-json: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 4107 | 21 % | 214772 | `deduxit::linear::parse::read (mod.rs:209)` |
| 4107 | 21 % | 214772 | `deserialize_any<serde_json::read::StrRead, deduxit::serialize::sequents::{impl#2}::deserialize::Push> (src/de.rs:0)` |
| 4097 | 21 % | 116564 | `of_tables<deduxit::linear::sequents::term::TermId> (sequents.rs:225)` |
| 3096 | 16 % | 58016 | `{closure#0}<u32, alloc::string::String, (), deduxit::linear::families::wide::{closure_env#0}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#22}::extend_t` |
| 3096 | 16 % | 58016 | `{closure#0}<u32, alloc::string::String, (), deduxit::linear::families::wide::{closure_env#1}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#22}::extend_t` |
| 76 | 0 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 62 | 0 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |
| 52 | 0 % | 7985 | `<deduxit_bench::Cli as clap_builder::derive::Parser>::parse (src/derive.rs:32)` |

## read-lltp: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 6007 | 57 % | 949893 | `deduxit::linear::lltp::read (lltp.rs:120)` |
| 4012 | 38 % | 214297 | `deduxit::linear::parse::read (mod.rs:209)` |
| 76 | 0 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 62 | 0 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |
| 52 | 0 % | 7985 | `<deduxit_bench::Cli as clap_builder::derive::Parser>::parse (src/derive.rs:32)` |
| 28 | 0 % | 1688 | `<deduxit_bench::OneArgs as clap_builder::derive::Args>::augment_args (main.rs:341)` |
| 20 | 0 % | 196416 | `<alloc::raw_vec::RawVec<alloc::vec::Vec<alloc::vec::Vec<deduxit::rules::terms::TermId>>>>::grow_one (mod.rs:198)` |
| 14 | 0 % | 311277 | `deduxit_bench::journeys::read_lltp (mod.rs:421)` |

## read-spec: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 8026 | 64 % | 2674624 | `<alloc::raw_vec::RawVec<alloc::boxed::Box<[deduxit::lambda::TypeVarId]>>>::grow_one (mod.rs:198)` |
| 2011 | 16 % | 107103 | `<deduxit::linear::mist::Arena>::atom (mist.rs:347)` |
| 2001 | 16 % | 32016 | `deduxit::linear::mist::merged (mist.rs:421)` |
| 76 | 0 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 62 | 0 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |
| 52 | 0 % | 7985 | `<deduxit_bench::Cli as clap_builder::derive::Parser>::parse (src/derive.rs:32)` |
| 28 | 0 % | 1688 | `<deduxit_bench::OneArgs as clap_builder::derive::Args>::augment_args (main.rs:341)` |
| 16 | 0 % | 327675 | `deduxit_bench::journeys::spec (mod.rs:451)` |

## read-text: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 8214 | 55 % | 429544 | `deduxit::linear::parse::read (mod.rs:209)` |
| 3096 | 20 % | 58016 | `{closure#0}<u32, alloc::string::String, (), deduxit::linear::families::wide::{closure_env#0}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#22}::extend_t` |
| 3096 | 20 % | 58016 | `{closure#0}<u32, alloc::string::String, (), deduxit::linear::families::wide::{closure_env#1}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#22}::extend_t` |
| 76 | 0 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 62 | 0 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |
| 52 | 0 % | 7985 | `<deduxit_bench::Cli as clap_builder::derive::Parser>::parse (src/derive.rs:32)` |
| 28 | 0 % | 1688 | `<deduxit_bench::OneArgs as clap_builder::derive::Args>::augment_args (main.rs:341)` |
| 26 | 0 % | 786336 | `push_mut<deduxit::linear::sequents::term::Term, alloc::alloc::Global> (mod.rs:1000)` |

## read-tptp: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 6004 | 32 % | 850817 | `deduxit::ordinary::parse::read_tptp (parse.rs:942)` |
| 4013 | 21 % | 558517 | `<deduxit::ordinary::Formulas>::atom (mod.rs:937)` |
| 4012 | 21 % | 214297 | `<deduxit::ordinary::Formulas>::atom (mod.rs:936)` |
| 4001 | 21 % | 192032 | `<alloc::raw_vec::RawVec<[deduxit::linear::sequents::term::TermId; 2]>>::grow_one (mod.rs:198)` |
| 76 | 0 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 62 | 0 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |
| 52 | 0 % | 7985 | `<deduxit_bench::Cli as clap_builder::derive::Parser>::parse (src/derive.rs:32)` |
| 28 | 0 % | 1688 | `<deduxit_bench::OneArgs as clap_builder::derive::Args>::augment_args (main.rs:341)` |

## render-latex: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 555 | 18 % | 47060 | `with_capacity_in<deduxit::linear::occurrences::Member, alloc::alloc::Global> (mod.rs:187)` |
| 500 | 16 % | 49600 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 197 | 6 % | 5160 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 165 | 5 % | 15840 | `<alloc::raw_vec::RawVec<alloc::vec::Vec<alloc::vec::Vec<deduxit::rules::terms::TermId>>>>::grow_one (mod.rs:198)` |
| 164 | 5 % | 164 | `{closure#0}<deduxit::linear::proofs::derivation::view::Derivation, alloc::string::String, deduxit::limits::counting::{closure_env#0}<deduxit_bench::journeys::render::{closure#0}::{closure#0}::{closure_env#0}>> (latex.rs:` |
| 134 | 4 % | 3544 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 130 | 4 % | 652 | `with_capacity_in<deduxit::linear::proofs::derivation::view::InfId, alloc::alloc::Global> (mod.rs:187)` |
| 103 | 3 % | 1896 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |

## render-svg: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 46165 | 87 % | 627528 | `deduxit::export::svg::escape (mod.rs:1045)` |
| 1637 | 3 % | 5698880 | `<alloc::raw_vec::RawVec<deduxit::export::svg::Piece>>::grow_one (mod.rs:198)` |
| 1473 | 2 % | 107086 | `<deduxit::export::svg::Escaping<alloc::string::String> as core::fmt::Write>::write_str (mod.rs:1267)` |
| 555 | 1 % | 47060 | `with_capacity_in<deduxit::linear::occurrences::Member, alloc::alloc::Global> (mod.rs:187)` |
| 500 | 0 % | 49600 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 197 | 0 % | 5160 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 164 | 0 % | 2296 | `{closure#11}<deduxit::linear::proofs::derivation::view::Derivation, alloc::string::String, deduxit::limits::counting::{closure_env#0}<&mut dyn core::ops::function::FnMut<(deduxit::limits::Progress), Output=bool>>> (tree.` |
| 134 | 0 % | 3544 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |

## render-typst: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 555 | 18 % | 47060 | `with_capacity_in<deduxit::linear::occurrences::Member, alloc::alloc::Global> (mod.rs:187)` |
| 500 | 16 % | 49600 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 197 | 6 % | 5160 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 165 | 5 % | 15840 | `<alloc::raw_vec::RawVec<alloc::vec::Vec<alloc::vec::Vec<deduxit::rules::terms::TermId>>>>::grow_one (mod.rs:198)` |
| 164 | 5 % | 164 | `{closure#0}<deduxit::linear::proofs::derivation::view::Derivation, alloc::string::String, deduxit::limits::counting::{closure_env#0}<deduxit_bench::journeys::render::{closure#0}::{closure#1}::{closure_env#0}>> (typst.rs:` |
| 134 | 4 % | 3544 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 130 | 4 % | 652 | `with_capacity_in<deduxit::linear::proofs::derivation::view::InfId, alloc::alloc::Global> (mod.rs:187)` |
| 103 | 3 % | 1896 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |

## rules-check: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 48025 | 33 % | 669304 | `with_capacity_in<deduxit::rules::terms::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 27911 | 19 % | 360864 | `with_capacity_in<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:187)` |
| 9769 | 6 % | 348080 | `with_capacity_in<deduxit::linear::occurrences::Member, alloc::alloc::Global> (mod.rs:187)` |
| 8333 | 5 % | 110754 | `<deduxit::rules::check::Checker>::node (check.rs:487)` |
| 8271 | 5 % | 55756 | `with_capacity_in<deduxit::linear::proofs::derivation::view::InfId, alloc::alloc::Global> (mod.rs:187)` |
| 8270 | 5 % | 74336 | `with_capacity_in<&deduxit::ordinary::derivation::Inference, alloc::alloc::Global> (mod.rs:187)` |
| 6406 | 4 % | 444288 | `<alloc::raw_vec::RawVec<core::cmp::Reverse<(bool, u32, deduxit::rules::fitch::Unit)>>>::grow_one (mod.rs:198)` |
| 4647 | 3 % | 218656 | `with_capacity_in<(deduxit::ordinary::NodeId, deduxit::ordinary::derivation::Side), alloc::alloc::Global> (mod.rs:187)` |

## search-additive-14: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 52090 | 31 % | 7762218 | `deduxit::linear::families::additive_tree (families.rs:962)` |
| 32771 | 19 % | 786424 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 32770 | 19 % | 960820 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 32758 | 19 % | 163760 | `deduxit::linear::families::additive_tree (families.rs:957)` |
| 16397 | 9 % | 873530 | `deduxit::linear::parse::read (mod.rs:209)` |
| 76 | 0 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 62 | 0 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |
| 52 | 0 % | 8012 | `<deduxit_bench::Cli as clap_builder::derive::Parser>::parse (src/derive.rs:32)` |

## search-chain-128: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 1176 | 36 % | 197568 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 389 | 11 % | 10280 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 262 | 8 % | 7060 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 199 | 6 % | 3688 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |
| 137 | 4 % | 12602 | `deduxit::linear::parse::read (mod.rs:209)` |
| 130 | 3 % | 13520 | `{closure#0}<deduxit::linear::search::focus::zone::Ground> (scratch.rs:60)` |
| 128 | 3 % | 3328 | `{closure#0}<u32, alloc::string::String, (), deduxit::linear::families::FAMILIES::{closure#15}::{closure_env#0}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{` |
| 76 | 2 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |

## search-chain-64-intuitionistic: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 500 | 27 % | 49600 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 197 | 10 % | 5160 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 134 | 7 % | 3544 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 103 | 5 % | 1896 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |
| 76 | 4 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 72 | 4 % | 6236 | `deduxit::linear::parse::read (mod.rs:209)` |
| 66 | 3 % | 3696 | `{closure#0}<deduxit::linear::search::focus::zone::Ground> (scratch.rs:60)` |
| 64 | 3 % | 1664 | `{closure#0}<u32, alloc::string::String, (), deduxit::linear::families::FAMILIES::{closure#15}::{closure_env#0}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{` |

## search-chain-fo-32: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 3855 | 19 % | 388524 | `<hashbrown::raw::RawTable<(deduxit::linear::search::focus::framed::TRef, ())>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::linear::search::focus::framed::TRef, (), foldhash::fast::FixedState>::{closure#0}> (sr` |
| 2083 | 10 % | 72192 | `<alloc::raw_vec::RawVec<[deduxit::linear::sequents::term::TermId; 2]>>::grow_one (mod.rs:198)` |
| 2049 | 10 % | 73764 | `<hashbrown::raw::RawTable<(deduxit::first_order::Symbol, ())>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Symbol, (), foldhash::fast::FixedState>::{closure#0}> (src/raw.rs:962)` |
| 2049 | 10 % | 32784 | `<deduxit::linear::search::focus::framed::Framed as deduxit::linear::search::focus::zone::Zone>::open_answers (framed.rs:1792)` |
| 1830 | 9 % | 165888 | `new_uninit<[(deduxit::linear::search::focus::framed::TRef, bool); 1]> (boxed.rs:334)` |
| 1670 | 8 % | 56240 | `<deduxit::linear::search::focus::framed::Framed as deduxit::linear::search::focus::zone::Zone>::record (framed.rs:1823)` |
| 1371 | 7 % | 224216 | `<hashbrown::raw::RawTable<(deduxit::linear::search::focus::framed::TRef, deduxit::first_order::FoTermId)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::linear::search::focus::framed::TRef, deduxit::first_order:` |
| 1286 | 6 % | 224352 | `<alloc::raw_vec::RawVec<core::cmp::Reverse<(bool, u32, deduxit::rules::fitch::Unit)>>>::grow_one (mod.rs:198)` |

## search-inverse: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 1722 | 31 % | 82076 | `with_capacity_in<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:187)` |
| 900 | 16 % | 14400 | `insert_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:2273)` |
| 431 | 7 % | 15444 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |
| 425 | 7 % | 6800 | `walk<deduxit::linear::search::inverse::index::{impl#0}::within::{closure_env#0}<deduxit::linear::search::inverse::{impl#2}::subsumed::{closure_env#0}>> (index.rs:77)` |
| 289 | 5 % | 4624 | `<deduxit::linear::search::inverse::index::Trie>::insert (index.rs:68)` |
| 250 | 4 % | 6976 | `from_iter<u32, core::iter::adapters::copied::Copied<core::iter::adapters::chain::Chain<core::iter::adapters::flatten::Flatten<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<deduxit::linear::occurrenc` |
| 136 | 2 % | 2176 | `<deduxit::linear::search::inverse::index::Trie>::walk::<<deduxit::linear::search::inverse::index::Trie>::ends_within<<deduxit::linear::search::inverse::Saturation>::subsumed::{closure#0}>::{closure#0}> (index.rs:77)` |
| 113 | 2 % | 12528 | `deduxit::linear::search::inverse::search (mod.rs:170)` |

## search-partition-no-5: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 101 | 12 % | 1816 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 76 | 9 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 62 | 7 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |
| 52 | 6 % | 8021 | `<deduxit_bench::Cli as clap_builder::derive::Parser>::parse (src/derive.rs:32)` |
| 52 | 6 % | 1744 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 40 | 4 % | 1270 | `deduxit::linear::families::partition (families.rs:754)` |
| 28 | 3 % | 1688 | `<deduxit_bench::OneArgs as clap_builder::derive::Args>::augment_args (main.rs:341)` |
| 20 | 2 % | 720 | `<deduxit::linear::search::focus::counts::Tally>::add (counts.rs:790)` |

## search-qbf-20-2: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 254 | 14 % | 5996 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 236 | 13 % | 10956 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 120 | 6 % | 4706 | `{closure#0}<&[(u32, bool); 3], alloc::string::String, (), deduxit::linear::families::qbf::{closure_env#4}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#` |
| 103 | 5 % | 7232 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |
| 102 | 5 % | 2650 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 92 | 5 % | 5888 | `{closure#0}<deduxit::linear::search::focus::zone::Ground> (scratch.rs:60)` |
| 90 | 5 % | 1440 | `{closure#0}<alloc::string::String, (u32, bool), deduxit::linear::families::qbf::{closure_env#3}> (try_trait.rs:415)` |
| 76 | 4 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |

## search-spec-chain: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 16003 | 30 % | 160012 | `new_uninit<[deduxit::linear::occurrences::OccId; 1]> (boxed.rs:334)` |
| 8026 | 15 % | 2674624 | `<alloc::raw_vec::RawVec<alloc::boxed::Box<[deduxit::lambda::TypeVarId]>>>::grow_one (mod.rs:198)` |
| 4015 | 7 % | 522920 | `{closure#0}<deduxit::linear::occurrences::Sign, deduxit::linear::search::horn::Program, deduxit::linear::search::horn::{impl#4}::read::{closure_env#0}> (iterator.rs:3010)` |
| 4005 | 7 % | 112040 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 4004 | 7 % | 113838 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 4002 | 7 % | 128064 | `<alloc::raw_vec::RawVec<[deduxit::linear::sequents::term::TermId; 2]>>::grow_one (mod.rs:198)` |
| 2012 | 3 % | 48400 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 2011 | 3 % | 107103 | `<deduxit::linear::mist::Arena>::atom (mist.rs:347)` |

## search-wide-m2-256: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 3328 | 51 % | 361816 | `new_uninit<[deduxit::linear::occurrences::OccId; 1]> (boxed.rs:334)` |
| 774 | 12 % | 31740 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 771 | 11 % | 20844 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 263 | 4 % | 12996 | `deduxit::linear::parse::read (mod.rs:209)` |
| 256 | 3 % | 3584 | `{closure#0}<u32, alloc::string::String, (), deduxit::linear::families::wide::{closure_env#0}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#22}::extend_t` |
| 256 | 3 % | 3584 | `{closure#0}<u32, alloc::string::String, (), deduxit::linear::families::wide::{closure_env#1}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#22}::extend_t` |
| 256 | 3 % | 4096 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 76 | 1 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |

## session-linear: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 892 | 18 % | 27940 | `with_capacity_in<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:187)` |
| 561 | 11 % | 14382 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 288 | 6 % | 3200 | `with_capacity_in<deduxit::linear::occurrences::Member, alloc::alloc::Global> (mod.rs:187)` |
| 170 | 3 % | 2720 | `insert_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:2273)` |
| 144 | 3 % | 278400 | `push_mut<deduxit::linear::search::focus::counts::RowEntry, alloc::alloc::Global> (mod.rs:1000)` |
| 144 | 3 % | 7488 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, deduxit::first_order::Eigen)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, deduxit::first_order::Eigen, foldhash::fast::FixedState>::` |
| 111 | 2 % | 111 | `new_uninit<[deduxit::linear::proofs::rule::Rule; 1]> (boxed.rs:334)` |
| 111 | 2 % | 444 | `with_capacity_in<deduxit::linear::proofs::interactive::Applicable, alloc::alloc::Global> (mod.rs:187)` |

## session-ordinary: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 37850 | 48 % | 480712 | `with_capacity_in<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:187)` |
| 12418 | 16 % | 74364 | `with_capacity_in<deduxit::linear::proofs::derivation::view::InfId, alloc::alloc::Global> (mod.rs:187)` |
| 9717 | 12 % | 347684 | `with_capacity_in<deduxit::linear::occurrences::Member, alloc::alloc::Global> (mod.rs:187)` |
| 4617 | 5 % | 461700 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, deduxit::first_order::Eigen)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, deduxit::first_order::Eigen, foldhash::fast::FixedState>::` |
| 4607 | 5 % | 217016 | `with_capacity_in<(deduxit::ordinary::NodeId, deduxit::ordinary::derivation::Side), alloc::alloc::Global> (mod.rs:187)` |
| 4227 | 5 % | 38176 | `with_capacity_in<&deduxit::ordinary::derivation::Inference, alloc::alloc::Global> (mod.rs:187)` |
| 1536 | 1 % | 221184 | `new<deduxit::linear::proofs::check::zones::State> (boxed.rs:302)` |
| 512 | 0 % | 32768 | `with_capacity_in<(usize, alloc::vec::Vec<(deduxit::ordinary::NodeId, deduxit::ordinary::derivation::Side), alloc::alloc::Global>), alloc::alloc::Global> (mod.rs:187)` |

## session-rules: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 19698 | 13 % | 29106 | `to_vec<deduxit::serialize::rules::PatternForm, alloc::alloc::Global> (slice.rs:432)` |
| 16464 | 11 % | 945504 | `with_capacity_in<deduxit::serialize::rules::PatternForm, alloc::alloc::Global> (mod.rs:187)` |
| 12685 | 8 % | 6900640 | `try_new_uninit_in<alloc::collections::btree::node::LeafNode<alloc::string::String, alloc::vec::Vec<deduxit::serialize::rules::PatternForm, alloc::alloc::Global>>, alloc::alloc::Global> (boxed.rs:552)` |
| 12642 | 8 % | 88494 | `<alloc::collections::btree::map::BTreeMap<_, _, _> as core::clone::Clone>::clone::clone_subtree::<alloc::string::String, alloc::vec::Vec<deduxit::serialize::rules::PatternForm>, alloc::alloc::Global> (map.rs:258)` |
| 10939 | 7 % | 262536 | `with_capacity_in<alloc::vec::Vec<deduxit::rules::calculus::Entry, alloc::alloc::Global>, alloc::alloc::Global> (mod.rs:187)` |
| 10933 | 7 % | 174928 | `with_capacity_in<deduxit::rules::calculus::Entry, alloc::alloc::Global> (mod.rs:187)` |
| 5295 | 3 % | 98840 | `to_vec<deduxit::rules::calculus::Premise, alloc::alloc::Global> (slice.rs:432)` |
| 4704 | 3 % | 337512 | `<deduxit::rules::calculus::Calculus as core::clone::Clone>::clone (calculus.rs:297)` |

## workspace-run: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 1844 | 35 % | 78028 | `with_capacity_in<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:187)` |
| 614 | 11 % | 23296 | `deserialize_struct<serde_json::read::StrRead, deduxit::serialize::ordinary::_::{impl#0}::deserialize::__Visitor> (src/de.rs:1851)` |
| 147 | 2 % | 2080 | `new_uninit<[deduxit::ordinary::NodeId; 1]> (boxed.rs:334)` |
| 130 | 2 % | 3176 | `from_iter<u32, core::iter::adapters::map::Map<core::slice::iter::Iter<deduxit::ordinary::NodeId>, deduxit::serialize::ordinary::{impl#9}::of::{closure#0}::{closure_env#0}>> (spec_from_iter_nested.rs:52)` |
| 130 | 2 % | 324 | `visit_some<deduxit::ordinary::derivation::Rule, &mut serde_json::de::Deserializer<serde_json::read::StrRead>> (src/de/impls.rs:905)` |
| 112 | 2 % | 1280 | `with_capacity_in<&deduxit::ordinary::derivation::Inference, alloc::alloc::Global> (mod.rs:187)` |
| 112 | 2 % | 1600 | `with_capacity_in<deduxit::linear::proofs::derivation::view::InfId, alloc::alloc::Global> (mod.rs:187)` |
| 105 | 2 % | 7405 | `<deduxit::ordinary::Formulas>::atom (mod.rs:936)` |
