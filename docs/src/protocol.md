# What each suite offers

deduxit's suites (linear logic, ordinary logic, calculi given as rule
files) each make values and turn values into others: a sequent becomes a
decision, a translation, a proof step by step, a drawing. The library
lists these as data, so that a program, a web page or the workspace of
linked tabs can offer every one of them without code of its own per
suite. This chapter is the reference of that list and of the values it
passes.

## In the library

The module `deduxit::protocol` (the crate's feature `protocol`, on by
default) holds the list and runs it. Every value is JSON text. Here a
classical sequent is translated into intuitionistic logic, and the
translation's promise, that both sequents are valid together or not, is
checked:

```rust
use deduxit::{Settings, protocol};

let settings = Settings::default().deterministic();
let sequent = r#"{"version": 8, "formulas": [{"V": 0}, {"¬": 0}, {"∨": [0, 1]}],
    "atoms": ["a"], "left": [], "right": [2], "logic": "classical"}"#;
let glivenko = r#"{"translation": "glivenko"}"#;
let translated =
    protocol::run("ordinary.translate", &[sequent], glivenko, &settings, |_| false)?;
let text = protocol::summary("ordinary.sequent", &translated.value, &settings.limits)?;
assert_eq!(text, "⊢ ¬¬(a ∨ ¬a)");
let check = protocol::check(
    "ordinary.translate", &[sequent], glivenko, &translated.value, &settings, |_| false,
)?;
assert_eq!(check.status, protocol::Status::Held);
```

`protocol::catalogue()` writes the whole list as JSON. Each suite is a
value of the trait `protocol::Suite`, so a suite from another crate can
join it.

## Kinds of values

A kind says what a value is and how it is written. A statement is a
sequent with the calculus it is claimed in; a work kind holds the steps
someone took; evidence is what a run found; a rendering is a drawing or a
text, quick to make again.

| kind | what it is | written as |
|---|---|---|
| `linear.sequent` | a sequent of linear logic and its mode | the sequent's JSON with `mode` |
| `linear.session` | a proof in progress | the session's JSON |
| `linear.outcome` | a sequent decided | the outcome's JSON |
| `linear.proof` | a proof | the proof's JSON |
| `linear.net` | a proof net | the proof structure's JSON |
| `ordinary.sequent` | a sequent of ordinary logic and its logic | the sequent's JSON with `logic` |
| `ordinary.session` | a proof in progress in LK or LJ | the session's JSON |
| `ordinary.outcome` | a sequent decided, with a derivation or a countermodel | the outcome's JSON |
| `ordinary.derivation` | a derivation of LK or LJ | the derivation's JSON |
| `ordinary.interpolant` | a Craig interpolant and its two halves | `{"logic", "partition", "outcome", "interpolant"}` |
| `rules.calculus` | a calculus | the rule file |
| `rules.sequent` | a sequent to derive in a calculus | `{"goal", "assumptions"}`, text the calculus reads |
| `rules.session` | a derivation in progress | the session's JSON |
| `rules.derivation` | a derivation | the derivation's JSON |
| `drawing` | a drawing | `{"svg"}` |
| `text` | a text in a format | `{"format", "text"}` |
| `certificate` | a certificate for Rocq | `{"text"}` |
| `term` | a proof read as a lambda-term | `{"text", "judgement"}` |

A sequent's JSON with `mode` or `logic` beside its keys reads as the
sequent wherever a sequent is read: the key is ignored there. `text`,
where present, is the sequent as it was typed.

## Operations

An operation takes values of the kinds it lists, in order, and makes one
value. Its parameters are a JSON object, `""` for every default. It
promises something of its output and says what checking that costs.

| operation | from | to | promise |
|---|---|---|---|
| `linear.prove` | `linear.sequent` | `linear.outcome` | decides |
| `linear.start` | `linear.sequent` | `linear.session` | exact |
| `linear.goal` | `linear.session` | `linear.sequent` | exact |
| `linear.proof` | `linear.session` | `linear.proof` | proves |
| `linear.net` | `linear.proof` | `linear.net` | proves |
| `linear.eliminate` | `linear.proof` | `linear.proof` | proves |
| `linear.term` | a session or a proof | `term` | renders |
| `linear.draw` | a session, a proof or a net | `drawing` | renders |
| `linear.export` | a session or a proof | `text` | renders |
| `linear.certificate` | a proof or an outcome | `certificate` | certifies |
| `linear.duplicate` | `linear.sequent` | `linear.sequent` | exact |
| `ordinary.decide` | `ordinary.sequent` | `ordinary.outcome` | decides |
| `ordinary.start` | `ordinary.sequent` | `ordinary.session` | exact |
| `ordinary.goal` | `ordinary.session` | `ordinary.sequent` | exact |
| `ordinary.derivation` | a session or an outcome | `ordinary.derivation` | proves |
| `ordinary.translate` | `ordinary.sequent` | `ordinary.sequent` | same verdict |
| `ordinary.interpolate` | `ordinary.sequent` | `ordinary.interpolant` | interpolates |
| `ordinary.half` | `ordinary.interpolant` | `ordinary.sequent` | exact |
| `ordinary.embed` | `ordinary.sequent` | `linear.sequent` | same verdict |
| `ordinary.term` | a session or a derivation | `term` | renders |
| `ordinary.draw` | a session, a derivation or an outcome | `drawing` | renders |
| `ordinary.export` | a session or a derivation | `text` | renders |
| `ordinary.certificate` | a derivation or an outcome | `certificate` | certifies |
| `ordinary.duplicate` | `ordinary.sequent` | `ordinary.sequent` | exact |
| `rules.start` | `rules.calculus`, `rules.sequent` | `rules.session` | exact |
| `rules.derivation` | `rules.session` | `rules.derivation` | proves |
| `rules.from_ordinary` | `ordinary.derivation` | `rules.derivation` | proves |
| `rules.draw` | `rules.session` | `drawing` | renders |
| `rules.export` | `rules.session` | `text` | renders |
| `rules.duplicate` | `rules.sequent` | `rules.sequent` | exact |

The catalogue gives each operation's parameters with their values and
defaults, its plain name, how its output relates to its inputs once made
(`derived`, `work` or `copy`), and whether a run is `instant` or a
`search` that may end at a bound.

## Promises and their checks

| promise | what it claims | what checking it costs |
|---|---|---|
| exact | the output is read off the input | nothing to check |
| decides | the verdict has evidence deduxit's checker accepts | instant |
| proves | the proof or derivation proves its sequent | instant |
| same verdict | input and output are valid together or not | two searches |
| interpolates | the interpolant's atoms occur in both parts, and both halves are valid | two searches |
| renders | the drawing or text shows its input | nothing to check |
| certifies | Rocq accepts the certificate | Rocq checks it |

A check ends `held`, `failed` (a fault of deduxit), `open` (a part was
not decided within the bounds), `recorded` (it held when the output was
made, and the output keeps nothing to check it by again, as a clausal
proof's counts), `outside` (another program checks it) or `none`.

## What a run depends on

Each operation lists the settings keys its output depends on, besides its
inputs and parameters: `limits.work`, `search`, `view`, `styles.svg` and
so on, from [the settings file](settings.md). On one thread
(`--deterministic` in the command, `Settings::deterministic` in the
library) the same inputs, parameters and settings give the same output,
so a program may keep it. A run that a bound ended says which: its
output says "unknown", and the run names the reason and the settings
key that lifts it, `limits.work` for the budget of work.

## The schema of the JSON forms

[`wire.schema.json`](wire.schema.json) is a JSON Schema of every
document deduxit writes and reads, with the level each form appeared at,
and of the values above. A rule file has a schema of its own,
`deduxit::rules::SCHEMA` ([Check a derivation in your own
rules](rules.md)).
