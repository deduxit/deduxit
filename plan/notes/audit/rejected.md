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
