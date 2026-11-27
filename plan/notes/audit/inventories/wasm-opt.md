# The web module's bytes, by where they come from

6540 items, 9547603 bytes raw (after wasm-opt -O3 with names kept; stripped and at gzip -9 it is 3 028 524 bytes).

## By module of the library

| module | bytes | share |
|---|---:|---:|
| `deduxit::linear` | 1837524 | 19.2 % |
| `deduxit::ordinary` | 1287356 | 13.5 % |
| `deduxit::serialize` | 1130081 | 11.8 % |
| `deduxit::protocol` | 719751 | 7.5 % |
| `deduxit::export` | 534766 | 5.6 % |
| `deduxit::lambda` | 527187 | 5.5 % |
| `deduxit::rules` | 274294 | 2.9 % |
| `deduxit::workspace` | 168488 | 1.8 % |
| `deduxit::first_order` | 85836 | 0.9 % |
| `deduxit::errors` | 77448 | 0.8 % |
| `deduxit::modal` | 68278 | 0.7 % |
| `deduxit::limits` | 53555 | 0.6 % |
| `deduxit::settings` | 25798 | 0.3 % |
| `deduxit::wire` | 13651 | 0.1 % |
| `deduxit::kripke` | 12001 | 0.1 % |
| `deduxit::unify` | 6983 | 0.1 % |
| `deduxit::name` | 6365 | 0.1 % |
| all of deduxit | 6829362 | 71.5 % |

## Inside `deduxit::linear`

| module | bytes | share |
|---|---:|---:|
| `deduxit::linear::search` | 675769 | 7.1 % |
| `deduxit::linear::proofs` | 664138 | 7.0 % |
| `deduxit::linear::sequents` | 173036 | 1.8 % |
| `deduxit::linear::occurrences` | 164563 | 1.7 % |
| `deduxit::linear::nets` | 121520 | 1.3 % |
| `deduxit::linear::parse` | 28064 | 0.3 % |
| `deduxit::linear::fragment` | 10434 | 0.1 % |

## Inside `deduxit::ordinary`

| module | bytes | share |
|---|---:|---:|
| `deduxit::ordinary::connection` | 187243 | 2.0 % |
| `deduxit::ordinary` | 120801 | 1.3 % |
| `deduxit::ordinary::sat` | 105799 | 1.1 % |
| `deduxit::ordinary::normal` | 93262 | 1.0 % |
| `deduxit::ordinary::interactive` | 84346 | 0.9 % |
| `deduxit::ordinary::resolution` | 76135 | 0.8 % |
| `deduxit::ordinary::herbrand` | 67765 | 0.7 % |
| `deduxit::ordinary::models` | 58836 | 0.6 % |
| `deduxit::ordinary::rocq` | 56658 | 0.6 % |
| `deduxit::ordinary::grounding` | 54210 | 0.6 % |
| `deduxit::ordinary::derivation` | 51791 | 0.5 % |
| `deduxit::ordinary::ljt` | 43917 | 0.5 % |
| `deduxit::ordinary::kripke_structure` | 33264 | 0.3 % |
| `deduxit::ordinary::kripke_search` | 31858 | 0.3 % |
| `deduxit::ordinary::translate` | 30405 | 0.3 % |
| `deduxit::ordinary::interpolant` | 30091 | 0.3 % |
| `deduxit::ordinary::decide` | 27301 | 0.3 % |
| `deduxit::ordinary::rocq_models` | 20173 | 0.2 % |
| `deduxit::ordinary::structure` | 19730 | 0.2 % |
| `deduxit::ordinary::written` | 19398 | 0.2 % |
| `deduxit::ordinary::parse` | 18764 | 0.2 % |
| `deduxit::ordinary::equality` | 12388 | 0.1 % |
| `deduxit::ordinary::transfer` | 11721 | 0.1 % |
| `deduxit::ordinary::instance` | 10246 | 0.1 % |
| `deduxit::ordinary::tableau` | 9738 | 0.1 % |
| `deduxit::ordinary::negative` | 9098 | 0.1 % |
| `deduxit::ordinary::model` | 2301 | 0.0 % |
| `deduxit::ordinary::table` | 103 | 0.0 % |
| `deduxit::ordinary::grounds` | 14 | 0.0 % |

## Everything else

| group | bytes | share |
|---|---:|---:|
| sections of names and producers | 807416 | 8.5 % |
| data segments | 533974 | 5.6 % |
| other | 523084 | 5.5 % |
| the bindings (deduxit_web) | 286069 | 3.0 % |
| crate core | 205904 | 2.2 % |
| crate serde_json | 127798 | 1.3 % |
| the bindings (deduxit_web_tab) | 69915 | 0.7 % |
| crate alloc | 56308 | 0.6 % |
| crate hashbrown | 30651 | 0.3 % |
| tables, types, imports and exports | 16533 | 0.2 % |
| crate batsat | 14948 | 0.2 % |
| crate serde | 14782 | 0.2 % |
| crate dlmalloc | 7431 | 0.1 % |
| crate serde_core | 5478 | 0.1 % |
| crate compiler_builtins | 3489 | 0.0 % |
| crate std | 2872 | 0.0 % |
| crate foldhash | 2484 | 0.0 % |
| crate unicode_normalization | 2022 | 0.0 % |
| crate zmij | 1762 | 0.0 % |
| crate trim_matches | 1326 | 0.0 % |
| crate __rustc | 1112 | 0.0 % |
| crate replace | 1020 | 0.0 % |
| crate tinyvec | 676 | 0.0 % |
| crate itoa | 412 | 0.0 % |
| crate trim_start_matches | 366 | 0.0 % |

## Generic functions compiled more than once

1959200 bytes (20.5 %) are the second and later instantiations of a function: what one shared body would save at most.

| bytes beyond the largest copy | copies | function |
|---:|---:|---|
| 217123 | 36 | `core::slice::sort::unstable::quicksort::quicksort` |
| 140540 | 40 | `<serde::private::de::content::ContentDeserializer<serde_json::error::Error> as serde_core::de::Deserializer>::deserialize_any` |
| 95732 | 30 | `core::slice::sort::stable::quicksort::quicksort` |
| 93355 | 415 | `core::ptr::drop_glue` |
| 78931 | 2 | `<core::marker::PhantomData<alloc::vec::Vec<deduxit::lambda::Node>> as serde_core::de::DeserializeSeed>::deserialize` |
| 70303 | 95 | `<serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry` |
| 69188 | 13 | `deduxit::protocol::document` |
| 52951 | 76 | `<serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeStruct>::serialize_field` |
| 46011 | 30 | `core::slice::sort::stable::drift::sort` |
| 45823 | 2 | `deduxit::export::svg::drawn` |
| 35407 | 2 | `<core::marker::PhantomData<alloc::vec::Vec<deduxit::lambda::Type>> as serde_core::de::DeserializeSeed>::deserialize` |
| 31791 | 2 | `<core::marker::PhantomData<core::option::Option<deduxit::serialize::ordinary::SearchRead>> as serde_core::de::DeserializeSeed>::deserialize` |
| 25838 | 10 | `<deduxit::linear::sequents::notation::Atoms>::write` |
| 25469 | 2 | `deduxit::serialize::errors::envelope` |
| 22616 | 4 | `deduxit::linear::search::prove_goal` |
| 17751 | 7 | `deduxit::protocol::object` |
| 16978 | 4 | `<deduxit::linear::proofs::check::error::CheckError as serde_core::ser::Serialize>::serialize` |
| 16144 | 4 | `<deduxit::linear::nets::NetError as serde_core::ser::Serialize>::serialize` |
| 15409 | 6 | `<&mut serde_json::de::Deserializer<serde_json::read::StrRead> as serde_core::de::Deserializer>::deserialize_any` |
| 13511 | 2 | `<serde_json::value::Value as serde_core::de::Deserializer>::deserialize_struct` |
| 13327 | 2 | `<deduxit::serialize::ordinary::EndForm as serde_core::de::Deserialize>::deserialize` |
| 13024 | 51 | `core::slice::sort::shared::pivot::median3_rec` |
| 11633 | 2 | `<<deduxit::serialize::proofs::Step as serde_core::de::Deserialize>::deserialize::__Visitor as serde_core::de::Visitor>::visit_enum` |
| 11416 | 4 | `<core::marker::PhantomData<deduxit::serialize::sequents::Atoms> as serde_core::de::DeserializeSeed>::deserialize` |
| 11335 | 29 | `<serde_json::value::ser::SerializeMap as serde_core::ser::SerializeStruct>::serialize_field` |
| 11274 | 2 | `<deduxit::serialize::ordinary::OutcomeForm as serde_core::ser::Serialize>::serialize` |
| 10953 | 6 | `<deduxit::linear::sequents::notation::Notation>::ill` |
| 10943 | 4 | `<deduxit::limits::Refusal as serde_core::ser::Serialize>::serialize` |
| 10879 | 12 | `core::slice::sort::shared::smallsort::sort8_stable` |
| 10709 | 2 | `<core::marker::PhantomData<alloc::boxed::Box<deduxit::serialize::search::OutcomeForm>> as serde_core::de::DeserializeSeed>::deserialize` |

## The 40 largest items

| bytes | item |
|---:|---|
| 761466 | `"function names" subsection` |
| 144892 | `<deduxit::protocol::ordinary::Ordinary as deduxit::protocol::Suite>::run` |
| 118405 | `deduxit::linear::proofs::derivation::build::unfold` |
| 81773 | `<deduxit::protocol::modal::Modal as deduxit::protocol::Suite>::run` |
| 79011 | `<core::marker::PhantomData<alloc::vec::Vec<deduxit::lambda::Node>> as serde_core::de::DeserializeSeed>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::StrRead>>` |
| 78931 | `<core::marker::PhantomData<alloc::vec::Vec<deduxit::lambda::Node>> as serde_core::de::DeserializeSeed>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::StrRead>>` |
| 73585 | `deduxit::export::svg::mell::draw` |
| 70491 | `deduxit::ordinary::connection::run` |
| 63893 | `<deduxit::protocol::linear::Linear as deduxit::protocol::Suite>::run` |
| 58805 | `deduxit::export::svg::drawn::<alloc::string::String>` |
| 57527 | `data segment ".rodata.384"` |
| 53532 | `deduxit_web::calls::export` |
| 45823 | `deduxit::export::svg::drawn::<alloc::string::String>` |
| 40880 | `deduxit::ordinary::herbrand::deskolemized` |
| 40570 | `deduxit::protocol::statement` |
| 39903 | `data segment ".rodata.1"` |
| 39067 | `data segment ".rodata.1065"` |
| 38293 | `deduxit::ordinary::sat::run` |
| 35435 | `<core::marker::PhantomData<alloc::vec::Vec<deduxit::lambda::Type>> as serde_core::de::DeserializeSeed>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::StrRead>>` |
| 35407 | `<core::marker::PhantomData<alloc::vec::Vec<deduxit::lambda::Type>> as serde_core::de::DeserializeSeed>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::StrRead>>` |
| 35399 | `ordinary_tableau_review_5337a3da5bc63d93` |
| 33617 | `deduxit_web_tab::failure::written` |
| 32496 | `data segment ".rodata.1166"` |
| 32032 | `<deduxit::protocol::lambda::Lambda as deduxit::protocol::Suite>::run` |
| 31884 | `<deduxit::protocol::ordinary::Ordinary as deduxit::protocol::Suite>::check` |
| 31845 | `<core::marker::PhantomData<core::option::Option<deduxit::serialize::ordinary::SearchRead>> as serde_core::de::DeserializeSeed>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::StrRea` |
| 31791 | `<core::marker::PhantomData<core::option::Option<deduxit::serialize::ordinary::SearchRead>> as serde_core::de::DeserializeSeed>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::StrRea` |
| 31721 | `deduxit_web_tab::read` |
| 31698 | `deduxit::serialize::ordinary::session` |
| 30845 | `<deduxit::errors::Error as serde_core::ser::Serialize>::serialize::<&mut serde_json::ser::Serializer<&mut alloc::vec::Vec<u8>>>` |
| 30083 | `<deduxit::rules::interactive::Interactive>::found_at` |
| 29121 | `<deduxit::linear::proofs::cut::Record>::of` |
| 28845 | `custom section '__wasm_bindgen_unstable'` |
| 28640 | `<deduxit::serialize::interactive::Interactive>::within` |
| 28635 | `<deduxit::linear::proofs::interactive::Interactive>::close::<deduxit_web::calls::never>` |
| 28378 | `deduxit::serialize::errors::envelope::<&mut serde_json::ser::Serializer<&mut alloc::vec::Vec<u8>>>` |
| 28292 | `deduxit::ordinary::models::run` |
| 26983 | `data segment ".rodata.901"` |
| 25469 | `deduxit::serialize::errors::envelope::<serde_json::value::ser::Serializer>` |
| 24498 | `cut_5337a3da5bc63d93` |
