# The web module's bytes, by where they come from

8398 items, 11216136 bytes raw (before wasm-bindgen and wasm-opt).

## By module of the library

| module | bytes | share |
|---|---:|---:|
| `deduxit::linear` | 2188457 | 19.5 % |
| `deduxit::ordinary` | 1660209 | 14.8 % |
| `deduxit::serialize` | 1242864 | 11.1 % |
| `deduxit::export` | 787119 | 7.0 % |
| `deduxit::lambda` | 671609 | 6.0 % |
| `deduxit::protocol` | 500191 | 4.5 % |
| `deduxit::rules` | 328757 | 2.9 % |
| `deduxit::workspace` | 222482 | 2.0 % |
| `deduxit::modal` | 128298 | 1.1 % |
| `deduxit::first_order` | 115598 | 1.0 % |
| `deduxit::errors` | 79134 | 0.7 % |
| `deduxit::limits` | 64510 | 0.6 % |
| `deduxit::settings` | 20743 | 0.2 % |
| `deduxit::wire` | 14086 | 0.1 % |
| `deduxit::kripke` | 13395 | 0.1 % |
| `deduxit::unify` | 9696 | 0.1 % |
| `deduxit::name` | 7235 | 0.1 % |
| `deduxit::goal_tree` | 2598 | 0.0 % |
| all of deduxit | 8056981 | 71.8 % |

## Inside `deduxit::linear`

| module | bytes | share |
|---|---:|---:|
| `deduxit::linear::proofs` | 837255 | 7.5 % |
| `deduxit::linear::search` | 751255 | 6.7 % |
| `deduxit::linear::occurrences` | 209066 | 1.9 % |
| `deduxit::linear::sequents` | 198244 | 1.8 % |
| `deduxit::linear::nets` | 148537 | 1.3 % |
| `deduxit::linear::parse` | 32110 | 0.3 % |
| `deduxit::linear::fragment` | 11990 | 0.1 % |

## Inside `deduxit::ordinary`

| module | bytes | share |
|---|---:|---:|
| `deduxit::ordinary::connection` | 199018 | 1.8 % |
| `deduxit::ordinary` | 194779 | 1.7 % |
| `deduxit::ordinary::interactive` | 159358 | 1.4 % |
| `deduxit::ordinary::resolution` | 145879 | 1.3 % |
| `deduxit::ordinary::sat` | 119229 | 1.1 % |
| `deduxit::ordinary::normal` | 104775 | 0.9 % |
| `deduxit::ordinary::herbrand` | 72166 | 0.6 % |
| `deduxit::ordinary::interpolant` | 69177 | 0.6 % |
| `deduxit::ordinary::derivation` | 65703 | 0.6 % |
| `deduxit::ordinary::models` | 64757 | 0.6 % |
| `deduxit::ordinary::grounding` | 58523 | 0.5 % |
| `deduxit::ordinary::written` | 57169 | 0.5 % |
| `deduxit::ordinary::ljt` | 48945 | 0.4 % |
| `deduxit::ordinary::rocq` | 40772 | 0.4 % |
| `deduxit::ordinary::kripke_structure` | 38492 | 0.3 % |
| `deduxit::ordinary::kripke_search` | 33245 | 0.3 % |
| `deduxit::ordinary::translate` | 27132 | 0.2 % |
| `deduxit::ordinary::decide` | 25951 | 0.2 % |
| `deduxit::ordinary::structure` | 23787 | 0.2 % |
| `deduxit::ordinary::rocq_models` | 21554 | 0.2 % |
| `deduxit::ordinary::parse` | 20384 | 0.2 % |
| `deduxit::ordinary::equality` | 13643 | 0.1 % |
| `deduxit::ordinary::negative` | 13607 | 0.1 % |
| `deduxit::ordinary::transfer` | 12544 | 0.1 % |
| `deduxit::ordinary::tableau` | 11973 | 0.1 % |
| `deduxit::ordinary::instance` | 11369 | 0.1 % |
| `deduxit::ordinary::model` | 3552 | 0.0 % |
| `deduxit::ordinary::closed` | 1948 | 0.0 % |
| `deduxit::ordinary::table` | 764 | 0.0 % |
| `deduxit::ordinary::grounds` | 14 | 0.0 % |

## Everything else

| group | bytes | share |
|---|---:|---:|
| sections of names and producers | 1362214 | 12.1 % |
| data segments | 541339 | 4.8 % |
| the bindings (deduxit_web) | 526892 | 4.7 % |
| crate core | 243999 | 2.2 % |
| crate serde_json | 193149 | 1.7 % |
| crate alloc | 75652 | 0.7 % |
| crate hashbrown | 47851 | 0.4 % |
| other | 47042 | 0.4 % |
| the bindings (deduxit_web_tab) | 25229 | 0.2 % |
| crate batsat | 20842 | 0.2 % |
| crate serde | 19966 | 0.2 % |
| tables, types, imports and exports | 16682 | 0.1 % |
| crate dlmalloc | 7991 | 0.1 % |
| crate serde_core | 7972 | 0.1 % |
| crate std | 3986 | 0.0 % |
| crate compiler_builtins | 3748 | 0.0 % |
| crate foldhash | 3040 | 0.0 % |
| crate unicode_normalization | 2169 | 0.0 % |
| crate zmij | 1872 | 0.0 % |
| crate trim_matches | 1423 | 0.0 % |
| crate __rustc | 1392 | 0.0 % |
| crate replace | 1163 | 0.0 % |
| crate trim_start_matches | 778 | 0.0 % |
| crate tinyvec | 745 | 0.0 % |
| crate trim_end_matches | 696 | 0.0 % |

## Generic functions compiled more than once

2907378 bytes (25.9 %) are the second and later instantiations of a function: what one shared body would save at most.

| bytes beyond the largest copy | copies | function |
|---:|---:|---|
| 203920 | 200 | `<serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeStruct>::serialize_field` |
| 184910 | 36 | `core::slice::sort::unstable::quicksort::quicksort` |
| 154249 | 157 | `<serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry` |
| 145175 | 621 | `core::ptr::drop_glue` |
| 131108 | 40 | `<serde::private::de::content::ContentDeserializer<serde_json::error::Error> as serde_core::de::Deserializer>::deserialize_any` |
| 103092 | 30 | `core::slice::sort::stable::quicksort::quicksort` |
| 96309 | 7 | `deduxit::linear::proofs::check::pass::examine_with` |
| 73806 | 2 | `<deduxit::lambda::Node as serde_core::de::Deserialize>::deserialize` |
| 73781 | 13 | `deduxit::protocol::document` |
| 54752 | 76 | `<serde_json::value::ser::SerializeMap as serde_core::ser::SerializeStruct>::serialize_field` |
| 48848 | 30 | `core::slice::sort::stable::drift::sort` |
| 43547 | 2 | `deduxit::export::svg::drawn` |
| 40422 | 7 | `<deduxit::ordinary::Formulas>::write_first_order` |
| 39070 | 4 | `<<deduxit::serialize::proofs::Step as serde_core::de::Deserialize>::deserialize::__Visitor as serde_core::de::Visitor>::visit_enum` |
| 32492 | 2 | `<deduxit::lambda::Type as serde_core::de::Deserialize>::deserialize` |
| 29414 | 2 | `<deduxit::serialize::ordinary::SearchRead as serde_core::de::Deserialize>::deserialize` |
| 29344 | 6 | `deduxit::export::typst::write` |
| 28057 | 3 | `core::slice::sort::shared::smallsort::small_sort_network` |
| 27756 | 10 | `<deduxit::linear::sequents::notation::Atoms>::write` |
| 24769 | 4 | `deduxit::linear::search::prove_goal` |
| 22110 | 8 | `deduxit::protocol::object` |
| 22046 | 4 | `deduxit::export::rocq::write` |
| 20794 | 36 | `core::slice::sort::unstable::ipnsort` |
| 19586 | 4 | `<deduxit::linear::proofs::check::error::CheckError as serde_core::ser::Serialize>::serialize` |
| 18332 | 16 | `deduxit::protocol::written` |
| 18093 | 38 | `<serde_json::value::ser::SerializeMap as serde_core::ser::SerializeMap>::serialize_entry` |
| 18035 | 4 | `<deduxit::errors::step::StepFault as serde_core::ser::Serialize>::serialize` |
| 17180 | 52 | `core::slice::sort::shared::smallsort::insertion_sort_shift_left` |
| 16394 | 6 | `<&mut serde_json::de::Deserializer<serde_json::read::StrRead> as serde_core::de::Deserializer>::deserialize_any` |
| 15642 | 10 | `deduxit::export::notation::walk` |

## The 40 largest items

| bytes | item |
|---:|---|
| 1333009 | `"function names" subsection` |
| 540746 | `data segment ".rodata"` |
| 75429 | `deduxit::export::svg::mell::draw` |
| 73806 | `<deduxit::lambda::Node as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::StrRead>>` |
| 73806 | `<deduxit::lambda::Node as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::StrRead>>` |
| 55530 | `deduxit::export::svg::drawn::<alloc::string::String>` |
| 43547 | `deduxit::export::svg::drawn::<alloc::string::String>` |
| 36869 | `<deduxit::protocol::ordinary::Ordinary as deduxit::protocol::Suite>::run` |
| 32492 | `<deduxit::lambda::Type as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::StrRead>>` |
| 32492 | `<deduxit::lambda::Type as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::StrRead>>` |
| 31516 | `<deduxit::protocol::modal::Modal as deduxit::protocol::Suite>::run` |
| 29414 | `<deduxit::serialize::ordinary::SearchRead as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::StrRead>>` |
| 29414 | `<deduxit::serialize::ordinary::SearchRead as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::StrRead>>` |
| 28845 | `custom section '__wasm_bindgen_unstable'` |
| 28536 | `deduxit::ordinary::connection::run` |
| 27252 | `deduxit::lambda::reduce::copied` |
| 25780 | `deduxit::linear::search::inverse::search` |
| 24725 | `deduxit_web::calls::about` |
| 24510 | `<deduxit::ordinary::herbrand::Build>::step` |
| 23594 | `<deduxit::protocol::linear::Linear as deduxit::protocol::Suite>::run` |
| 22907 | `<deduxit::protocol::ordinary::Ordinary as deduxit::protocol::Suite>::check` |
| 22676 | `<deduxit::ordinary::connection::matrix::Matrix>::of` |
| 22060 | `deduxit::modal::calculus::form` |
| 21852 | `deduxit::ordinary::sat::run` |
| 21758 | `deduxit::ordinary::kripke_search::force` |
| 21566 | `deduxit::linear::proofs::cut::reduce` |
| 20895 | `<deduxit::lambda::parse::Parser>::term` |
| 20517 | `deduxit::ordinary::models::kripke` |
| 20272 | `deduxit::export::typst::write::<deduxit::export::Drawable, alloc::string::String, deduxit_web::calls::never>` |
| 19680 | `<deduxit::rules::natural::Translator>::translate` |
| 19220 | `deduxit::linear::proofs::check::pass::examine_with::<deduxit::linear::proofs::derivation::build::Record<deduxit::linear::proofs::check::frames::Ground>, deduxit::linear::proofs::check::frames::Framed,` |
| 19220 | `deduxit::linear::proofs::check::pass::examine_with::<deduxit::linear::proofs::derivation::build::Record<deduxit::linear::proofs::check::frames::Framed>, deduxit::linear::proofs::check::frames::Framed,` |
| 19139 | `<deduxit::ordinary::interactive::Replay>::run` |
| 18960 | `<deduxit::linear::proofs::interactive::Interactive>::written` |
| 18921 | `deduxit::linear::proofs::check::pass::examine_with::<deduxit::linear::proofs::cut::Record, deduxit::linear::proofs::check::frames::Framed, deduxit::linear::proofs::check::pass::examine<deduxit::linear` |
| 18869 | `deduxit::ordinary::decide::dispatch` |
| 18657 | `<deduxit::serialize::ordinary::SequentForm>::within` |
| 18287 | `deduxit::export::svg::net::draw` |
| 18260 | `<deduxit::linear::search::horn::Horn as deduxit::linear::search::Decide>::decide` |
| 18058 | `deduxit::export::svg::modal_countermodel::<&mut dyn core::ops::function::FnMut<(deduxit::limits::Progress,), Output = bool>>` |
