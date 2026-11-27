# Each journey's allocations (DHAT)

| journey | bytes allocated | blocks | instructions | instructions per block |
|---|---:|---:|---:|---:|
| batch-families | 16576774 | 9154 | 57324736 | 6262 |
| check-qbf-20-2 | 39259838 | 129111 | 649423964 | 5029 |
| check-wide-m1-2048 | 33620248 | 72324 | 15982901198 | 220990 |
| derivation-chain-64 | 2716519 | 2873 | 146338907 | 50935 |
| ordinary-pigeons | 5956104 | 60124 | 27026459 | 449 |
| read-json | 4793827 | 18951 | 36822479 | 1943 |
| read-lltp | 4854631 | 10465 | 31905319 | 3048 |
| read-spec | 4790489 | 12440 | 23971291 | 1926 |
| read-text | 5805958 | 14919 | 34470849 | 2310 |
| read-tptp | 3355368 | 18418 | 35217015 | 1912 |
| render-latex | 2323842 | 3042 | 143910372 | 47307 |
| render-svg | 12803657 | 52499 | 224268567 | 4271 |
| render-typst | 2369054 | 3046 | 142978857 | 46939 |
| search-additive-14 | 104927998 | 167253 | 681391692 | 4074 |
| search-chain-128 | 1887513 | 3257 | 1065805765 | 327235 |
| search-chain-64-intuitionistic | 1482145 | 1797 | 135482724 | 75393 |
| search-partition-no-5 | 1309622 | 833 | 8569998 | 10288 |
| search-qbf-20-2 | 24434593 | 1749 | 516774067 | 295468 |
| search-spec-chain | 8165203 | 52685 | 42983743 | 815 |
| search-wide-m2-256 | 1635840 | 6439 | 253150932 | 39315 |

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
| 64896 | 50 % | 3618816 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, deduxit::first_order::FoTermId)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, deduxit::first_order::FoTermId, foldhash::fast::FixedSt` |
| 62463 | 48 % | 8994672 | `new<deduxit::linear::proofs::check::zones::State> (boxed.rs:302)` |
| 254 | 0 % | 5996 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 236 | 0 % | 10956 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 120 | 0 % | 4706 | `{closure#0}<&[(u32, bool); 3], alloc::string::String, (), deduxit::linear::families::qbf::{closure_env#4}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#` |
| 103 | 0 % | 7232 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |
| 102 | 0 % | 2650 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 92 | 0 % | 5888 | `{closure#0}<deduxit::linear::search::focus::zone::Ground> (scratch.rs:60)` |

## check-wide-m1-2048: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 32768 | 45 % | 22467928 | `new_uninit<[deduxit::linear::occurrences::OccId; 1]> (boxed.rs:334)` |
| 12291 | 16 % | 349692 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 6150 | 8 % | 253948 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 4117 | 5 % | 360676 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, deduxit::first_order::FoTermId)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, deduxit::first_order::FoTermId, foldhash::fast::FixedSt` |
| 4107 | 5 % | 214772 | `deduxit::linear::parse::read (mod.rs:209)` |
| 4096 | 5 % | 589824 | `new<deduxit::linear::proofs::check::zones::State> (boxed.rs:302)` |
| 3096 | 4 % | 58016 | `{closure#0}<u32, alloc::string::String, (), deduxit::linear::families::wide::{closure_env#0}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#22}::extend_t` |
| 3096 | 4 % | 58016 | `{closure#0}<u32, alloc::string::String, (), deduxit::linear::families::wide::{closure_env#1}, core::iter::traits::iterator::Iterator::for_each::call::{closure_env#0}<alloc::string::String, alloc::vec::{impl#22}::extend_t` |

## derivation-chain-64: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 555 | 19 % | 47060 | `with_capacity_in<deduxit::linear::occurrences::Member, alloc::alloc::Global> (mod.rs:187)` |
| 500 | 17 % | 49600 | `push_mut<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:1000)` |
| 197 | 6 % | 5160 | `with_capacity_in<deduxit::linear::sequents::term::TermId, alloc::alloc::Global> (mod.rs:187)` |
| 134 | 4 % | 3544 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 130 | 4 % | 652 | `with_capacity_in<deduxit::linear::proofs::derivation::view::InfId, alloc::alloc::Global> (mod.rs:187)` |
| 103 | 3 % | 1896 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |
| 90 | 3 % | 12888 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, deduxit::first_order::FoTermId)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, deduxit::first_order::FoTermId, foldhash::fast::FixedSt` |
| 89 | 3 % | 8544 | `<alloc::raw_vec::RawVec<alloc::vec::Vec<deduxit::linear::search::focus::split::Frame<deduxit::linear::search::focus::zone::Ground>>>>::grow_one (mod.rs:198)` |

## ordinary-pigeons: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 27910 | 46 % | 360936 | `with_capacity_in<deduxit::ordinary::NodeId, alloc::alloc::Global> (mod.rs:187)` |
| 9769 | 16 % | 348080 | `with_capacity_in<deduxit::linear::occurrences::Member, alloc::alloc::Global> (mod.rs:187)` |
| 8271 | 13 % | 55756 | `with_capacity_in<deduxit::linear::proofs::derivation::view::InfId, alloc::alloc::Global> (mod.rs:187)` |
| 4647 | 7 % | 218656 | `with_capacity_in<(deduxit::ordinary::NodeId, deduxit::ordinary::derivation::Side), alloc::alloc::Global> (mod.rs:187)` |
| 4135 | 6 % | 37168 | `with_capacity_in<&deduxit::ordinary::derivation::Inference, alloc::alloc::Global> (mod.rs:187)` |
| 3076 | 5 % | 307480 | `<hashbrown::raw::RawTable<(deduxit::first_order::Eigen, deduxit::first_order::FoTermId)>>::reserve_rehash::<hashbrown::map::make_hasher<deduxit::first_order::Eigen, deduxit::first_order::FoTermId, foldhash::fast::FixedSt` |
| 1024 | 1 % | 147456 | `new<deduxit::linear::proofs::check::zones::State> (boxed.rs:302)` |
| 512 | 0 % | 32768 | `with_capacity_in<(usize, alloc::vec::Vec<(deduxit::ordinary::NodeId, deduxit::ordinary::derivation::Side), alloc::alloc::Global>), alloc::alloc::Global> (mod.rs:187)` |

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
| 20 | 0 % | 196416 | `<alloc::raw_vec::RawVec<alloc::vec::Vec<deduxit::linear::search::focus::split::Frame<deduxit::linear::search::focus::zone::Ground>>>>::grow_one (mod.rs:198)` |
| 14 | 0 % | 311277 | `deduxit_bench::journeys::read_lltp (journeys.rs:296)` |

## read-spec: the frames of deduxit that allocate most blocks

| blocks | share | bytes | frame |
|---:|---:|---:|---|
| 8026 | 64 % | 2674624 | `<alloc::raw_vec::RawVec<alloc::boxed::Box<[deduxit::ordinary::NodeId]>>>::grow_one (mod.rs:198)` |
| 2011 | 16 % | 107103 | `<deduxit::linear::mist::Arena>::atom (mist.rs:347)` |
| 2001 | 16 % | 32016 | `deduxit::linear::mist::merged (mist.rs:421)` |
| 76 | 0 % | 8378 | `<deduxit_bench::RunArgs as clap_builder::derive::Args>::augment_args (main.rs:176)` |
| 62 | 0 % | 25904 | `<deduxit_bench::Command as clap_builder::derive::Subcommand>::augment_subcommands (main.rs:61)` |
| 52 | 0 % | 7985 | `<deduxit_bench::Cli as clap_builder::derive::Parser>::parse (src/derive.rs:32)` |
| 28 | 0 % | 1688 | `<deduxit_bench::OneArgs as clap_builder::derive::Args>::augment_args (main.rs:341)` |
| 16 | 0 % | 327675 | `deduxit_bench::journeys::spec (journeys.rs:326)` |

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
| 4012 | 21 % | 214297 | `<deduxit::ordinary::Formulas>::atom (mod.rs:936)` |
| 4001 | 21 % | 192032 | `<alloc::raw_vec::RawVec<[deduxit::linear::sequents::term::TermId; 2]>>::grow_one (mod.rs:198)` |
| 4001 | 21 % | 17785 | `<deduxit::ordinary::Formulas>::atom (mod.rs:937)` |
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
| 165 | 5 % | 15840 | `<alloc::raw_vec::RawVec<alloc::vec::Vec<deduxit::linear::search::focus::split::Frame<deduxit::linear::search::focus::zone::Ground>>>>::grow_one (mod.rs:198)` |
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
| 165 | 5 % | 15840 | `<alloc::raw_vec::RawVec<alloc::vec::Vec<deduxit::linear::search::focus::split::Frame<deduxit::linear::search::focus::zone::Ground>>>>::grow_one (mod.rs:198)` |
| 164 | 5 % | 164 | `{closure#0}<deduxit::linear::proofs::derivation::view::Derivation, alloc::string::String, deduxit::limits::counting::{closure_env#0}<deduxit_bench::journeys::render::{closure#0}::{closure#1}::{closure_env#0}>> (typst.rs:` |
| 134 | 4 % | 3544 | `<deduxit::linear::sequents::Sequent as core::clone::Clone>::clone (mod.rs:194)` |
| 130 | 4 % | 652 | `with_capacity_in<deduxit::linear::proofs::derivation::view::InfId, alloc::alloc::Global> (mod.rs:187)` |
| 103 | 3 % | 1896 | `reserve<deduxit::linear::occurrences::OccId, alloc::alloc::Global> (mod.rs:358)` |

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
| 8026 | 15 % | 2674624 | `<alloc::raw_vec::RawVec<alloc::boxed::Box<[deduxit::ordinary::NodeId]>>>::grow_one (mod.rs:198)` |
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
