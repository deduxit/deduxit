# Findings the verifiers rejected (step 86, phase B)

Kept so that no finder raises one twice: a finding listed here comes
back only with a witness the rejection lacked. One entry per finding:
its id, its place, the claim, and the verifier's reason. The full text
is in `target/audit/units/UNIT/` (`findings.md`, `verdicts.md`) while
the audit runs, and under `units/` beside this file once a stage is
copied.

## Stage 2

- **F-6a-14**, `core/src/errors/mod.rs:460`. Claim: the message of
  `GlivenkoFirstOrder` is a nine-line essay with a countermodel and
  Rocq lemma names instead of the cause and the remedy. Rejected:
  `core-ordinary.md:2219-2226` records the message's content on purpose
  (no search of deduxit can show that countermodel, so the message
  carries it) and ties it to the manual and the guide; the finding
  neither names the choice nor says why its reason no longer holds.
- **F-7a-18**, `cli/src/batch.rs:565`. Claim: the batch edits
  serde_json's output as text where a `Serialize` record struct with
  `#[serde(flatten)]` would do. Rejected: an outcome writes its own
  `version`, so a flattened record writes it twice; `preserve_order`
  would change the key order of every `Value` in the workspace and add
  a package; `seconds` written by `{}` differs from serde_json's float
  form. `cli.md` records why the record is assembled as text.
- **F-7a-19**, `cli/src/prove.rs:876`. Claim: times in verdict lines are
  `Debug` of `Duration`, so users read `µs`, a unit `--timeout` does not
  take. Rejected: a limit echoed in a message reads back (`1.5s`,
  `1500ms`); `µs` appears only in elapsed times, which no flag takes,
  and users understand it. A finding on relying on `Debug`'s unpromised
  format would need another criterion and a shown harm (27 lines of the
  manual show such times).

Unit 9 was verified twice: its first verifier started before the
findings' full text was on disk and judged from the one-line claims
(`units/9/verdicts-first.md`), rejecting F-9-01; the second, with the
full text, confirmed all 18 (16 confirmed, 2 candidates), F-9-01 with
its witness corrected (the library's tests do use the exporter's names;
what is missing is the list that marks them as the contract). The
second verification stands.

## Stage 3

- **F-3-09**, `core/src/linear/nets/mod.rs:275`. Claim:
  `NetError::NoVertex` names its vertex as a bare `u32` where every
  other variant has a `VertexId`. Rejected: the number names no vertex
  of the structure (passing it to `partner` or `occurrence` answers
  `None`, `core-nets.md:36-39`), and the bare number says so.
- **F-3-12**, `core/src/linear/nets/mod.rs:1278`. Claim: the nets module
  holds 36 `must_use_candidate` sites, predicates among them. Rejected:
  `must_use_candidate` is one of the ten lints set aside on purpose
  (`Cargo.toml:95-100`); E7 admits a finding on one only if its reason
  no longer holds, and marking 36 of 556 sites would not turn the lint
  on. The same holds for the other nine: `many_single_char_names`,
  `similar_names`, `too_many_lines`, `enum_glob_use`,
  `single_match_else`, `match_bool`, `if_not_else`,
  `items_after_statements`, `naive_bytecount`.
- **F-4a-10**, `core/src/ordinary/decide.rs:885`. Claim: two places
  decide between "unknown" and "defect" by comparing an error's message
  with a sentinel string. Rejected: both sides use the one private
  constant `OPEN_WITNESS`, so rewording cannot turn one into the other;
  and the criterion cited (an AIP) applies to the JSON wire forms only.
- **F-4a-20**, `core/src/ordinary/mod.rs:720`. Claim: seven comments
  use "refers to" and "stands as". Rejected: at those places the verbs
  carry their own meaning (a node refers to its subformulas by index;
  terms "stand as they are", that is, stay unchanged). `text-read.md`
  lists the seven places in the crate where "stands as" does replace
  "is"; a text finding of this kind names one of those or shows the
  sentence.

Stage 4 rejected none of its 80 findings.

## Stage 5

- **F-5b-19**, `core/src/rules/fitch.rs:187`. Claim: `Fitch::new`
  borrows its options and then clones them, where taking them by value
  would spare a caller the copy. Rejected: every caller outside the
  tests borrows the options from styles it keeps, so by value each
  would clone at the call site instead, and borrowing them for the
  drawing's lifetime breaks the documented call with
  `&FitchOptions::default()`.
