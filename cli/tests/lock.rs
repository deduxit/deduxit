// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The behaviour lock: what the command prints and the status it exits
//! with, on a fixed corpus of calls (every command, every output format,
//! every fragment and mode, ordinary logic, the problem files, the batch
//! and the errors), pinned in `tests/lock/out/NAME.txt`. A change of what
//! the command does shows here as a changed file; such a change is a
//! commit of its own that says why. `BLESS=1 cargo test --test lock`
//! rewrites the files.
//!
//! The calls run in `tests/lock/inputs`, which holds the files they read.
//! A call whose output carries the search's counters names
//! `--deterministic`, since a pool's counters add every thread's; the
//! others decide in microseconds, long before the pool would start. PNG
//! and PDF output is pinned by its length and an FNV-1a hash.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

mod common;
use common::timeless;

/// A call of the command: the name of its file, its arguments and its
/// standard input.
struct Case {
    /// The file's name, without `.txt`.
    name: &'static str,
    /// The arguments.
    args: &'static [&'static str],
    /// The standard input.
    stdin: &'static str,
}

/// A call with no standard input.
const fn call(name: &'static str, args: &'static [&'static str]) -> Case {
    Case {
        name,
        args,
        stdin: "",
    }
}

/// A call that reads standard input.
const fn piped(name: &'static str, args: &'static [&'static str], stdin: &'static str) -> Case {
    Case { name, args, stdin }
}

/// A closed tableau written as a list.
const TABLEAU_CLOSED: &str =
    "1 T p \\/ q\n2 F q \\/ p\n3 F q 2\n4 F p 2\n- 5 T p 1\n  × 4, 5\n- 6 T q 1\n  × 3, 6\n";
/// An open tableau, drawn as `prove --method tableau` prints one.
const TABLEAU_OPEN: &str = "1  T a → b\n2  F b → a\n3  T b      →R 2\n4  F a      →R 2\n├─ 5  F a   →L 1\n│  open\n└─ 6  T b   →L 1\n   open\n";
/// A proof of `A * B |- B * A`, as `prove --format json` writes it.
const PROOF: &str = r#"{"version":1,"linlog":"0.1.0","verdict":"proved","checked":true,"fragment":"MLL","mode":"classical","engine":"net","statistics":{"nodes":2,"memo_hits":0,"memo_entries":0,"splits":0,"links":2,"tests":2,"copies":0},"sequent":{"terms":[{"D":0},{"D":1},{"⅋":[0,1]},{"V":1},{"V":0},{"⊗":[3,4]}],"roots":[2,5],"atoms":["A","B"]},"nodes":[{"ax":[2,4]},{"ax":[1,5]},{"⊗":[3,0,1]},{"⅋":[0,2]}]}"#;

/// The same proof with its axioms crossed: no proof of the sequent.
const WRONG_PROOF: &str = r#"{"version":1,"sequent":{"terms":[{"D":0},{"D":1},{"⅋":[0,1]},{"V":1},{"V":0},{"⊗":[3,4]}],"roots":[2,5],"atoms":["A","B"]},"nodes":[{"ax":[1,4]},{"ax":[2,5]},{"⊗":[3,0,1]},{"⅋":[0,2]}]}"#;

/// An outcome of ordinary logic that `prove --logic intuitionistic
/// --method ljt --format json` writes: a derivation of LJ.
/// A derivation of `a ∧ b ⊢ b ∧ a` in the built-in calculus LK, given as
/// data.
const RULES: &str = r#"{"version": 15, "calculus": "LK", "terms": ["a", "b", ["∧", 0, 1], ["∧", 1, 0]], "nodes": [{"sequent": [[1], [1]], "rule": "ax"}, {"sequent": [[2], [1]], "rule": "∧L₂", "at": [0], "premises": [0]}, {"sequent": [[0], [0]], "rule": "ax"}, {"sequent": [[2], [0]], "rule": "∧L₁", "at": [0], "premises": [2]}, {"sequent": [[2], [3]], "rule": "∧R", "at": [0], "premises": [1, 3]}]}"#;

/// A proof of `a` from `¬¬a` in natural deduction, by `RAA` in the
/// built-in calculus NK.
const NATURAL: &str = r#"{"version": 15, "calculus": "NK", "terms": ["a", ["¬", 0], ["¬", 1], ["⊥"]], "scopes": [0], "nodes": [{"sequent": [[2]], "hypothesis": true}, {"sequent": [[1]], "hypothesis": true, "scope": 1}, {"sequent": [[3]], "rule": "¬E", "premises": [0, 1], "scope": 1}, {"sequent": [[0]], "rule": "RAA", "premises": [2]}]}"#;

/// The same proof named as one of NJ, which has no `RAA`.
const NATURAL_NJ: &str = r#"{"version": 15, "calculus": "NJ", "terms": ["a", ["¬", 0], ["¬", 1], ["⊥"]], "scopes": [0], "nodes": [{"sequent": [[2]], "hypothesis": true}, {"sequent": [[1]], "hypothesis": true, "scope": 1}, {"sequent": [[3]], "rule": "¬E", "premises": [0, 1], "scope": 1}, {"sequent": [[0]], "rule": "RAA", "premises": [2]}]}"#;

/// The same derivation with the wrong rule at its fourth node.
const WRONG_RULES: &str = r#"{"version": 15, "calculus": "LK", "terms": ["a", "b", ["∧", 0, 1], ["∧", 1, 0]], "nodes": [{"sequent": [[1], [1]], "rule": "ax"}, {"sequent": [[2], [1]], "rule": "∧L₂", "at": [0], "premises": [0]}, {"sequent": [[0], [0]], "rule": "ax"}, {"sequent": [[2], [0]], "rule": "∧L₂", "at": [0], "premises": [2]}, {"sequent": [[2], [3]], "rule": "∧R", "at": [0], "premises": [1, 3]}]}"#;

const ORDINARY_OUTCOME: &str = r#"{"version":12,"deduxit":"0.3.0","verdict":"valid","logic":"intuitionistic","search":{"method":"ljt","statistics":{"sequents":1,"memo_hits":0,"memo_entries":1,"subsumed":0}},"derivation":{"logic":"intuitionistic","sequent":{"formulas":[{"V":0},"⊥"],"atoms":["a"],"left":[0],"right":[0]},"inferences":[{"left":[0],"right":[0],"rule":"ax"}]}}"#;

/// A Kripke model of two worlds, the second above the first and forcing
/// `a`: the countermodel of the excluded middle.
const MODEL: &str = r#"{"worlds":[{"atoms":[],"arrows":[1]},{"atoms":["a"]}]}"#;

/// A modal outcome at level 20: `□p ⊢ □□p` not valid in K, with its
/// countermodel.
const MODAL_OUTCOME: &str = r#"{"version":20,"deduxit":"0.7.0","verdict":"not_valid","system":"K","statistics":{"worlds":3,"splits":0,"blocked":0,"work":15,"inferences":0},"sequent":{"formulas":[{"V":0},{"□":0},{"□":1}],"atoms":["p"],"left":[1],"right":[2]},"countermodel":{"model":{"worlds":[{"atoms":[],"arrows":[1]},{"atoms":["p"],"arrows":[2]},{"atoms":[]}],"world":0}}}"#;

/// A Kripke model of three worlds in a row, `p` at the middle one: it
/// refutes `□p ⊢ □□p` at world 0, and no world sees itself.
const MODAL_MODEL: &str =
    r#"{"worlds":[{"atoms":[],"arrows":[1]},{"atoms":["p"],"arrows":[2]},{"atoms":[]}]}"#;

/// Every call the lock pins.
const CASES: &[Case] = &[
    // Fragments and modes, as text.
    call("prove-mll", &["prove", "A * B |- B * A"]),
    call("prove-mll-units", &["prove", "1, bot |- 1 * bot"]),
    call("prove-mall", &["prove", "A & B |- A + B"]),
    call("prove-mall-units", &["prove", "|- T, 0 + 1"]),
    call("prove-mell", &["prove", "!A |- A * !A"]),
    call("prove-ll", &["prove", "!(A & B) |- !A * ?B"]),
    call("prove-intuitionistic", &["prove", "-i", "A, A -o B |- B"]),
    call(
        "prove-intuitionistic-unprovable",
        &["prove", "-i", "A -o B |- B -o A"],
    ),
    call("prove-affine", &["prove", "-a", "A, B |- A"]),
    call("prove-iall", &["prove", "-i", "A & B |- B & A"]),
    call("prove-imell", &["prove", "-i", "!A |- A * A"]),
    call("prove-ill", &["prove", "-i", "A & !B |- !B + A"]),
    call(
        "prove-intuitionistic-affine",
        &["prove", "-i", "-a", "A, B |- A"],
    ),
    call("prove-mix", &["prove", "--mix", "A, B |- A, B"]),
    call("prove-horn", &["prove", "!(a -o a * b), a |- a * b * b"]),
    // Every refutation, as text and as JSON.
    call("refute-unbalanced", &["prove", "A |- A * A"]),
    call("refute-equation", &["prove", "|- A * ~A"]),
    call("refute-exhausted", &["prove", "A & B |- A * B"]),
    call(
        "refute-state-equation",
        &["prove", "!(A -o A * A), !(B * B -o C), A, B |- C"],
    ),
    call("refute-one-sided", &["prove", "!A, A |- 1"]),
    call("refute-classical", &["prove", "A + B |- A"]),
    call(
        "refute-classical-unknown",
        &["prove", "--copies", "1", "!(A -o A * A), A |- ?B"],
    ),
    call(
        "refute-classical-off",
        &[
            "prove",
            "--copies",
            "1",
            "--classical-work",
            "0",
            "!(A -o A * A), A |- ?B",
        ],
    ),
    call(
        "json-classical",
        &["prove", "--deterministic", "--format", "json", "A + B |- A"],
    ),
    call(
        "json-unbalanced",
        &["prove", "--deterministic", "--format", "json", "A |- A * A"],
    ),
    call(
        "json-equation",
        &["prove", "--deterministic", "--format", "json", "|- A * ~A"],
    ),
    call(
        "json-exhausted",
        &[
            "prove",
            "--deterministic",
            "--format",
            "json",
            "A & B |- A * B",
        ],
    ),
    call(
        "json-state-equation",
        &[
            "prove",
            "--deterministic",
            "--format",
            "json",
            "!(A -o A * A), !(B * B -o C), A, B |- C",
        ],
    ),
    call(
        "json-one-sided",
        &["prove", "--deterministic", "--format", "json", "!A, A |- 1"],
    ),
    // Every output format of a proof.
    call(
        "format-json",
        &[
            "prove",
            "--deterministic",
            "--format",
            "json",
            "!A |- A * !A",
        ],
    ),
    call(
        "format-json-intuitionistic",
        &[
            "prove",
            "--deterministic",
            "-i",
            "--format",
            "json",
            "A, A -o B |- B",
        ],
    ),
    call(
        "format-latex",
        &["prove", "--format", "latex", "A & B |- A + B"],
    ),
    call(
        "format-latex-standalone",
        &[
            "prove",
            "-i",
            "--format",
            "latex",
            "--standalone",
            "A, A -o B |- B",
        ],
    ),
    call(
        "format-typst",
        &["prove", "--format", "typst", "A & B |- A + B"],
    ),
    call(
        "format-svg",
        &["prove", "--format", "svg", "A * B |- B * A"],
    ),
    call(
        "format-png",
        &["prove", "--format", "png", "A * B |- B * A"],
    ),
    call(
        "format-pdf",
        &["prove", "--format", "pdf", "A * B |- B * A"],
    ),
    call(
        "format-rocq",
        &["prove", "--format", "rocq", "A * B |- B * A"],
    ),
    call(
        "format-rocq-standalone",
        &[
            "prove",
            "--format",
            "rocq",
            "--standalone",
            "--lemma",
            "swap",
            "!A |- A * !A",
        ],
    ),
    call(
        "format-rocq-refuted",
        &["prove", "--format", "rocq", "A |- B"],
    ),
    call(
        "format-rocq-refuted-ill",
        &["prove", "-i", "--format", "rocq", "A + B |- A"],
    ),
    call(
        "format-rocq-uncertified",
        &["prove", "--format", "rocq", "A & B |- A * B"],
    ),
    call(
        "format-rocq-not-valid",
        &[
            "prove",
            "--logic",
            "classical",
            "--format",
            "rocq",
            "a -> b, b |- a",
        ],
    ),
    call(
        "format-rocq-countermodel",
        &[
            "prove",
            "--logic",
            "intuitionistic",
            "--format",
            "rocq",
            "|- a \\/ ~a",
        ],
    ),
    call(
        "format-rocq-countermodel-minimal",
        &[
            "prove",
            "--logic",
            "minimal",
            "--format",
            "rocq",
            "a, ~a |- b",
        ],
    ),
    call(
        "format-svg-countermodel",
        &[
            "prove",
            "--logic",
            "intuitionistic",
            "--format",
            "svg",
            "|- a \\/ ~a",
        ],
    ),
    call(
        "format-latex-countermodel",
        &[
            "prove",
            "--logic",
            "intuitionistic",
            "--format",
            "latex",
            "|- a \\/ ~a",
        ],
    ),
    call("format-net", &["prove", "--net", "A * B |- B * A"]),
    call(
        "format-net-svg",
        &["prove", "--net", "--format", "svg", "A * B |- B * A"],
    ),
    call(
        "format-stats",
        &["prove", "--deterministic", "--stats", "!(A & B) |- !A * ?B"],
    ),
    call("format-quiet", &["prove", "--quiet", "A |- B"]),
    call("format-no-verdict", &["prove", "--no-verdict", "A |- A"]),
    call("format-tree-never", &["prove", "--tree", "never", "A |- A"]),
    call(
        "format-compact",
        &["prove", "--compact", "always", "A, B, C |- A * (B * C)"],
    ),
    call(
        "format-style",
        &[
            "prove",
            "--format",
            "latex",
            "--style",
            "labels=subscript",
            "--style",
            "align=false",
            "A, A -o B |- B",
        ],
    ),
    call(
        "format-derivation-limit",
        &["prove", "--derivation-limit", "100", "A * B |- B * A"],
    ),
    // Ordinary logic.
    call(
        "ordinary-classical",
        &["prove", "--logic", "classical", "a \\/ ~a"],
    ),
    call(
        "ordinary-intuitionistic",
        &["prove", "--logic", "intuitionistic", "a \\/ ~a"],
    ),
    call(
        "ordinary-minimal",
        &["prove", "--logic", "minimal", "a, a -> b |- b"],
    ),
    call(
        "ordinary-translation",
        &[
            "prove",
            "--logic",
            "intuitionistic",
            "--translation",
            "cbv",
            "(a /\\ b) -> (b /\\ a)",
        ],
    ),
    call("translate", &["seq", "translate", "a \\/ ~a"]),
    call(
        "translate-not-valid",
        &["seq", "translate", "--by", "godel-gentzen", "a |- b"],
    ),
    call("interpolate", &["interpolate", "a /\\ b |- b \\/ c"]),
    call("resolve", &["resolve", "a -> b, b -> c |- a -> c"]),
    call(
        "resolve-dpll",
        &["resolve", "--method", "dpll", "a \\/ b, ~a |- b"],
    ),
    call(
        "resolve-satisfiable",
        &["resolve", "--clauses", "tseitin", "a \\/ b |- a"],
    ),
    call("resolve-dimacs", &["resolve", "--dimacs", "four.cnf"]),
    call(
        "resolve-json",
        &[
            "resolve", "--method", "dpll", "--dimacs", "four.cnf", "--format", "json",
        ],
    ),
    call(
        "resolve-limit",
        &["resolve", "--step-limit", "1", "a -> b, b -> c |- a -> c"],
    ),
    call("term-check", &["term", "check", "\\f:a -> b. \\x:a. f x"]),
    call(
        "term-ill-typed",
        &["term", "check", "--calculus", "linear", "\\x:a. x * x"],
    ),
    call(
        "term-check-json",
        &[
            "term",
            "check",
            "--format",
            "json",
            "x : a |- \\y:b. x : b -> a",
        ],
    ),
    call("term-inhabit", &["term", "inhabit", "a /\\ b -> b /\\ a"]),
    call(
        "term-inhabit-empty",
        &["term", "inhabit", "((a -> b) -> a) -> a"],
    ),
    call(
        "term-enumerate",
        &["term", "enumerate", "(a -> a -> b) -> a -> a -> b"],
    ),
    call(
        "term-enumerate-infinite",
        &["term", "enumerate", "--depth", "3", "(a -> a) -> a -> a"],
    ),
    call(
        "term-enumerate-json",
        &[
            "term",
            "enumerate",
            "--format",
            "json",
            "--max-terms",
            "1",
            "a -> a -> a",
        ],
    ),
    call("term-enumerate-refused", &["term", "enumerate", "a -> ~~a"]),
    call(
        "check-drat",
        &["check", "--dimacs", "four.cnf", "four.drat"],
    ),
    call(
        "check-lrat",
        &["check", "--dimacs", "four.cnf", "four.lrat"],
    ),
    call(
        "check-drat-wrong",
        &["check", "--dimacs", "four.cnf", "wrong.drat"],
    ),
    call("check-refutation", &["check", "refutation.json"]),
    call("normalize", &["seq", "normalize", "~(a -> b /\\ c)"]),
    call(
        "normalize-dnf",
        &["seq", "normalize", "--to", "dnf", "(a \\/ b) /\\ ~c"],
    ),
    call(
        "normalize-plaisted-greenbaum",
        &[
            "seq",
            "normalize",
            "--to",
            "plaisted-greenbaum",
            "a -> b |- ~(b /\\ c)",
        ],
    ),
    call(
        "normalize-json",
        &[
            "seq",
            "normalize",
            "--to",
            "tseitin",
            "--format",
            "json",
            "a |- a",
        ],
    ),
    call(
        "normalize-limit",
        &[
            "seq",
            "normalize",
            "--form-limit",
            "8",
            "a /\\ b \\/ c /\\ d",
        ],
    ),
    // A workspace written by the command, read back.
    call("workspace-show", &["workspace", "show", "workspace.json"]),
    call("workspace-log", &["workspace", "log", "workspace.json"]),
    call(
        "workspace-node",
        &[
            "workspace",
            "show",
            "workspace.json",
            "1",
            "--format",
            "json",
        ],
    ),
    call(
        "workspace-operations",
        &["workspace", "operations", "ordinary.sequent"],
    ),
    call(
        "interpolate-first",
        &[
            "interpolate",
            "--logic",
            "intuitionistic",
            "--first",
            "L1",
            "a, a -> b |- b",
        ],
    ),
    call(
        "ordinary-linear",
        &["prove", "--logic", "intuitionistic", "--linear", "a -> a"],
    ),
    call(
        "ordinary-json",
        &[
            "prove",
            "--deterministic",
            "--logic",
            "classical",
            "--format",
            "json",
            "~~a -> a",
        ],
    ),
    call(
        "ordinary-rocq",
        &[
            "prove",
            "--logic",
            "classical",
            "--format",
            "rocq",
            "a \\/ ~a",
        ],
    ),
    call(
        "ordinary-embedding",
        &[
            "prove",
            "--logic",
            "intuitionistic",
            "--method",
            "embedding",
            "a \\/ ~a",
        ],
    ),
    call(
        "ordinary-ljt-json",
        &[
            "prove",
            "--method",
            "ljt",
            "--logic",
            "intuitionistic",
            "--format",
            "json",
            "(a /\\ b) -> c |- a -> b -> c",
        ],
    ),
    call(
        "ordinary-ljt-stats",
        &[
            "prove",
            "--method",
            "ljt",
            "--logic",
            "minimal",
            "--stats",
            "(a /\\ b) -> c |- a -> b -> c",
        ],
    ),
    call(
        "ordinary-ljt-rocq",
        &[
            "prove",
            "--method",
            "ljt",
            "--logic",
            "intuitionistic",
            "--format",
            "rocq",
            "(a /\\ b) -> c |- a -> b -> c",
        ],
    ),
    call(
        "ordinary-sat-clausal",
        &[
            "prove",
            "--logic",
            "classical",
            "--method",
            "sat",
            "--stats",
            "((a -> b) -> a) -> a",
        ],
    ),
    call(
        "ordinary-sat-json",
        &[
            "prove",
            "--logic",
            "classical",
            "--method",
            "sat",
            "--format",
            "json",
            "a -> b, b |- a",
        ],
    ),
    call(
        "ordinary-default-stats",
        &[
            "prove",
            "--logic",
            "intuitionistic",
            "--stats",
            "a /\\ b -> b /\\ a",
        ],
    ),
    // Four pigeons in three holes, past the embedding's bound for its
    // evidence: the classical default answers with the solver's proof.
    call(
        "ordinary-classical-default-clausal",
        &[
            "prove",
            "--logic",
            "classical",
            "--stats",
            "--deterministic",
            "p00 \\/ p01 \\/ p02, p10 \\/ p11 \\/ p12, p20 \\/ p21 \\/ p22, p30 \\/ p31 \\/ p32 |- \
             (p00 /\\ p10) \\/ (p00 /\\ p20) \\/ (p00 /\\ p30) \\/ (p10 /\\ p20) \\/ (p10 /\\ p30) \
             \\/ (p20 /\\ p30) \\/ (p01 /\\ p11) \\/ (p01 /\\ p21) \\/ (p01 /\\ p31) \\/ (p11 /\\ \
             p21) \\/ (p11 /\\ p31) \\/ (p21 /\\ p31) \\/ (p02 /\\ p12) \\/ (p02 /\\ p22) \\/ (p02 \
             /\\ p32) \\/ (p12 /\\ p22) \\/ (p12 /\\ p32) \\/ (p22 /\\ p32)",
        ],
    ),
    call(
        "ordinary-sat-intuitionistic",
        &[
            "prove",
            "--logic",
            "intuitionistic",
            "--method",
            "sat",
            "--stats",
            "~~a -> a",
        ],
    ),
    // The truth table and the tableau, valid and not, and their JSON.
    call(
        "ordinary-table",
        &[
            "prove",
            "--logic",
            "classical",
            "--method",
            "table",
            "a -> b |- b -> a",
        ],
    ),
    call(
        "ordinary-table-json",
        &[
            "prove",
            "--logic",
            "classical",
            "--method",
            "table",
            "--format",
            "json",
            "|- a \\/ ~a",
        ],
    ),
    call(
        "ordinary-table-atoms",
        &[
            "prove",
            "--logic",
            "classical",
            "--method",
            "table",
            "--table-atoms",
            "1",
            "a |- b",
        ],
    ),
    call(
        "ordinary-tableau",
        &[
            "prove",
            "--logic",
            "classical",
            "--method",
            "tableau",
            "p \\/ q, p -> r, q -> r |- r",
        ],
    ),
    call(
        "ordinary-tableau-open",
        &[
            "prove",
            "--logic",
            "classical",
            "--method",
            "tableau",
            "--stats",
            "a -> b |- b -> a",
        ],
    ),
    call(
        "ordinary-tableau-json",
        &[
            "prove",
            "--logic",
            "classical",
            "--method",
            "tableau",
            "--format",
            "json",
            "a /\\ b |- b",
        ],
    ),
    // Modal logic.
    call("modal-valid", &["prove", "--system", "S4", "[]p -> [][]p"]),
    call(
        "modal-rocq",
        &[
            "prove",
            "--system",
            "S4",
            "--format",
            "rocq",
            "[]p -> [][]p",
        ],
    ),
    call(
        "modal-rocq-countermodel",
        &[
            "prove",
            "--system",
            "S4",
            "--format",
            "rocq",
            "<>[]p |- []<>p",
        ],
    ),
    call(
        "modal-not-valid",
        &["prove", "--system", "K", "[]p -> [][]p"],
    ),
    call(
        "modal-json",
        &["prove", "--system", "K", "--format", "json", "[]p -> [][]p"],
    ),
    call(
        "modal-stats",
        &["prove", "--system", "KD45", "--stats", "-q", "[]p -> <>p"],
    ),
    call(
        "modal-latex",
        &[
            "prove",
            "--system",
            "S4",
            "--format",
            "latex",
            "[]p, <>q |- <>([]p /\\ q)",
        ],
    ),
    call(
        "modal-svg-countermodel",
        &["prove", "--system", "K", "--format", "svg", "[]p -> [][]p"],
    ),
    call(
        "modal-unknown",
        &[
            "prove",
            "--system",
            "S4",
            "--work-limit",
            "1",
            "[](p -> []p) -> (p -> []p)",
        ],
    ),
    call(
        "modal-alias",
        &["prove", "--system", "kt4", "-q", "<>[]p -> []<>p"],
    ),
    call("modal-unknown-system", &["prove", "--system", "S6", "[]p"]),
    call("modal-mode-flag", &["prove", "--system", "S4", "-i", "[]p"]),
    call(
        "modal-batch",
        &[
            "prove",
            "--system",
            "K",
            "--input-format",
            "lines",
            "--file",
            "many.txt",
        ],
    ),
    // Problem files.
    call("file-lltp", &["prove", "--file", "double.p"]),
    call(
        "file-tptp",
        &["prove", "--logic", "classical", "--file", "peirce.p"],
    ),
    call(
        "file-tptp-intuitionistic",
        &["prove", "--logic", "intuitionistic", "--file", "peirce.p"],
    ),
    call("file-spec", &["prove", "--file", "mutex.spec"]),
    call("file-spec-affine", &["prove", "-a", "--file", "mutex.spec"]),
    piped(
        "input-json",
        &["prove", "--input-format", "json"],
        r#"{"version":1,"terms":[{"D":0},{"V":0}],"roots":[0,1],"atoms":["A"]}"#,
    ),
    // The batch.
    call(
        "batch-lines",
        &["prove", "--input-format", "lines", "--file", "many.txt"],
    ),
    piped(
        "batch-jsonl",
        &["prove", "--input-format", "jsonl", "--file", "-"],
        "{\"version\":1,\"terms\":[{\"D\":0},{\"V\":0}],\"roots\":[0,1],\"atoms\":[\"A\"]}\n",
    ),
    call(
        "batch-json",
        &[
            "prove",
            "--input-format",
            "lines",
            "--format",
            "json",
            "--deterministic",
            "--file",
            "many.txt",
        ],
    ),
    piped(
        "batch-json-error",
        &["prove", "--input-format", "jsonl", "--format", "json"],
        "{\"name\": \"bad\", \"sequent\": \"A |-|- B\"}\n",
    ),
    // check.
    piped("check-valid", &["check"], PROOF),
    piped("check-invalid", &["check"], WRONG_PROOF),
    piped("check-not-json", &["check"], "A |- A"),
    piped("check-ordinary", &["check"], ORDINARY_OUTCOME),
    piped(
        "check-model-fails",
        &[
            "check",
            "--logic",
            "intuitionistic",
            "--sequent",
            "a \\/ ~a",
        ],
        MODEL,
    ),
    piped(
        "check-model-holds",
        &[
            "check",
            "--logic",
            "intuitionistic",
            "--sequent",
            "~~a -> a",
            "--world",
            "1",
        ],
        MODEL,
    ),
    piped("check-modal", &["check"], MODAL_OUTCOME),
    piped(
        "check-modal-json",
        &["check", "--format", "json"],
        MODAL_OUTCOME,
    ),
    piped(
        "check-modal-system",
        &["check", "--system", "K"],
        MODAL_OUTCOME,
    ),
    piped(
        "check-modal-model",
        &["check", "--system", "K", "--sequent", "[]p |- [][]p"],
        MODAL_MODEL,
    ),
    piped(
        "check-modal-frame",
        &["check", "--system", "T", "--sequent", "[]p |- [][]p"],
        MODAL_MODEL,
    ),
    piped("check-format-latex", &["check", "--format", "latex"], PROOF),
    piped("check-net", &["check", "--net"], PROOF),
    piped("check-json-valid", &["check", "--format", "json"], PROOF),
    piped(
        "check-json-invalid",
        &["check", "--format", "json"],
        WRONG_PROOF,
    ),
    piped(
        "check-json-unknown",
        &["check", "--format", "json", "--work-limit", "1"],
        PROOF,
    ),
    piped("check-rules-valid", &["check"], RULES),
    piped("check-rules-invalid", &["check"], WRONG_RULES),
    piped("check-rules-json", &["check", "--format", "json"], RULES),
    piped("check-rules-other", &["check", "--calculus", "lj"], RULES),
    piped("check-rules-latex", &["check", "--format", "latex"], RULES),
    piped("check-natural-nk", &["check"], NATURAL),
    piped("check-natural-nj", &["check"], NATURAL_NJ),
    piped(
        "interact-calculus-forward",
        &[
            "interact",
            "--calculus",
            "NJ",
            "--assume",
            "a /\\ b",
            "b /\\ a",
        ],
        "derive 0 ∧E₂ 1\nderive 0 ∧E₁ 1\nderive 0 ∧I 2 3\nderive 0 →I 1\nderive 0 ∧E₁ 9\nassume 0 2\nassume 0 4\nproof\n",
    ),
    piped(
        "interact-calculus-labelled",
        &["interact", "--calculus", "G3S4", "w0 : []p |- w0 : [][]p"],
        "derive 9 L∧ 0\napply 0 R□ 1\napply 1 R□ 3\nderive 2 Trans 1 2\nderive 3 L□ 0 3\nderive 4 R∧ 0\n\
         derive 4 L□ 0 3\nderive 4 Ref\nderive 4 Ref x=w0\napply 5 ax 4 8\nproof\n",
    ),
    piped(
        "check-natural-fitch",
        &["check", "--layout", "fitch"],
        NATURAL,
    ),
    piped(
        "check-natural-fitch-style",
        &[
            "check",
            "--layout",
            "fitch",
            "--style",
            "fitch.hypothesis=Hyp",
        ],
        NATURAL,
    ),
    piped(
        "check-natural-fitch-typst",
        &["check", "--layout", "fitch", "--format", "typst"],
        NATURAL,
    ),
    piped("check-natural-svg", &["check", "--format", "svg"], NATURAL),
    piped(
        "check-tableau-closed",
        &["check", "--tableau"],
        TABLEAU_CLOSED,
    ),
    piped(
        "check-tableau-open",
        &["check", "--tableau", "--sequent", "a -> b |- b -> a"],
        TABLEAU_OPEN,
    ),
    piped(
        "check-tableau-json",
        &["check", "--tableau", "--format", "json"],
        TABLEAU_OPEN,
    ),
    piped(
        "check-tableau-wrong",
        &["check", "--tableau"],
        "1 T p /\\ q\n2 F p\n3 T p 1\n4 T r 1\n",
    ),
    piped(
        "check-tableau-unfinished",
        &["check", "--tableau"],
        "1 T p \\/ q\n2 F p\n",
    ),
    piped(
        "check-tableau-layout",
        &["check", "--tableau"],
        "1 T p\n   2 F q\n",
    ),
    piped(
        "interact-calculus",
        &["interact", "--calculus", "LK", "a /\\ b |- b /\\ a"],
        "rules 0\napply 0 ∧R 1\napply 1 ∧L₂ 0\napply 2 ∧L₁ 0\napply 3 ax\napply 4 wrong\nundo\napply 3 ax\napply 4 ax\nproof\n",
    ),
    // interact.
    piped(
        "interact-session",
        &["interact", "A * B |- B * A"],
        "goals\nrules 0 0\napply 0 0 par\nundo\napply 0 0 par\ngoals\nrules 1 2\nclose\nshow\nproof --json\nquit\n",
    ),
    piped(
        "interact-errors",
        &["interact", "A |- A"],
        "apply 9 0 ax\nrules 0 7\nfrobnicate\nproof\nquit\n",
    ),
    piped("interact-help", &["interact", "A |- A"], "help\n"),
    // grade.
    piped(
        "grade-script",
        &["grade", "A, B |- A * B"],
        "# a split, then the axioms\napply 0 2 * 0\napply 1 1 ax\napply 2 1 ax\nproof\n",
    ),
    piped(
        "grade-script-unfinished",
        &["grade", "A, B |- A * B"],
        "apply 0 2 * 0\napply 1 0 par\nclose\n",
    ),
    piped("grade-proof", &["grade", "A * B |- B * A"], PROOF),
    piped("grade-proof-other", &["grade", "A * B |- A * B"], PROOF),
    piped(
        "grade-tableau",
        &["grade", "--logic", "classical", "p \\/ q |- q \\/ p"],
        TABLEAU_CLOSED,
    ),
    piped(
        "grade-json",
        &["grade", "--format", "json", "A |- A"],
        "not an answer\n",
    ),
    // seq.
    call("seq-print", &["seq", "print", "A -o B, !C |- ?D & 1"]),
    call(
        "seq-print-latex",
        &[
            "seq",
            "print",
            "-i",
            "--format",
            "latex",
            "A * B -o C |- A -o B -o C",
        ],
    ),
    call("seq-json", &["seq", "json", "A, A -o B |- B"]),
    call(
        "seq-print-svg",
        &["seq", "print", "--format", "svg", "A -o B |- B"],
    ),
    call("seq-fragment", &["seq", "fragment", "!A & B |- ?C"]),
    call(
        "seq-fragment-intuitionistic",
        &["seq", "fragment", "-i", "A & B |- A"],
    ),
    // Errors.
    call("error-parse", &["prove", "A * |- B"]),
    call(
        "error-parse-ordinary",
        &["prove", "--logic", "classical", "a /\\"],
    ),
    call("error-not-intuitionistic", &["prove", "-i", "|- ?A, !~A"]),
    call("error-engine", &["prove", "--engine", "net", "A & B |- A"]),
    call(
        "error-occurrence-limit",
        &["prove", "--occurrence-limit", "2", "A * B |- B * A"],
    ),
    call(
        "unknown-memory-limit",
        &["prove", "--memory-limit", "100", "!(A & B) |- !A * ?B"],
    ),
    call("error-missing-file", &["prove", "--file", "missing.p"]),
    call("error-unknown-flag", &["prove", "--frobnicate", "A |- A"]),
    call("error-no-command", &[]),
    // Help.
    call("help-prove", &["prove", "--help"]),
    call("help-check", &["check", "--help"]),
    call("help-grade", &["grade", "--help"]),
    call("help-interact", &["interact", "--help"]),
    call("help-seq-print", &["seq", "print", "--help"]),
];

/// Runs one call in the inputs directory and returns what the lock pins:
/// the command line, the exit status, standard output and standard error.
fn run(case: &Case, inputs: &Path) -> String {
    let out = common::output(
        Command::new(env!("CARGO_BIN_EXE_deduxit"))
            .args(case.args)
            .current_dir(inputs)
            .env_remove("NO_COLOR"),
        case.stdin,
    );
    let mut text = String::from("$ deduxit");
    for arg in case.args {
        if arg.is_empty()
            || arg.contains(|c: char| c.is_whitespace() || "\"'\\|*&!?~()<>;$".contains(c))
        {
            write!(text, " '{}'", arg.replace('\'', "'\\''")).unwrap();
        } else {
            write!(text, " {arg}").unwrap();
        }
    }
    if !case.stdin.is_empty() {
        text.push_str("\n--- stdin\n");
        text.push_str(case.stdin);
        if !case.stdin.ends_with('\n') {
            text.push('\n');
        }
    }
    writeln!(text, "\n--- status {}", out.status.code().unwrap_or(-1)).unwrap();
    text.push_str("--- stdout\n");
    match String::from_utf8(out.stdout) {
        Ok(stdout) => text.push_str(&timeless(&stdout)),
        Err(bytes) => {
            let bytes = bytes.into_bytes();
            writeln!(text, "{} bytes, FNV-1a {:016x}", bytes.len(), fnv(&bytes)).unwrap();
        }
    }
    text.push_str("--- stderr\n");
    text.push_str(&timeless(&String::from_utf8(out.stderr).unwrap()));
    text
}

/// The 64-bit FNV-1a hash of the bytes, which pins binary output without a
/// dependency.
fn fnv(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &b| {
        (hash ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// Every call of the corpus prints what its file pins, and every file in
/// `out/` is a call of the corpus.
#[test]
fn behaviour_lock() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/lock");
    let inputs = dir.join("inputs");
    let out = dir.join("out");
    let bless = std::env::var_os("BLESS").is_some();
    let mut changed = Vec::new();
    for case in CASES {
        let got = run(case, &inputs);
        let path = out.join(format!("{}.txt", case.name));
        let pinned = std::fs::read_to_string(&path).unwrap_or_default();
        if got != pinned {
            if bless {
                std::fs::write(&path, &got).unwrap();
            } else {
                let line = got
                    .lines()
                    .zip(pinned.lines())
                    .position(|(a, b)| a != b)
                    .unwrap_or_else(|| got.lines().count().min(pinned.lines().count()));
                changed.push(format!("{} (from line {})", case.name, line + 1));
            }
        }
    }
    for entry in std::fs::read_dir(&out).unwrap() {
        let name = entry.unwrap().file_name().into_string().unwrap();
        let stem = name.trim_end_matches(".txt");
        assert!(
            CASES.iter().any(|case| case.name == stem),
            "{name} in tests/lock/out is no call of the corpus"
        );
    }
    assert!(
        changed.is_empty(),
        "the command's behaviour changed (BLESS=1 rewrites the files): {}",
        changed.join(", ")
    );
}
