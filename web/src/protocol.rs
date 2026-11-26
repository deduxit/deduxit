// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The protocol every suite's calls speak, written as data: [`describe`].
//!
//! A suite offers the stage's calls by one set of names (`read`, `start`,
//! `view`, `rules`, `candidates`, `apply`, `undo`, `close`, `hint`, `draw`,
//! `export_as`, `certificate`), each exported under the suite's prefix with the
//! arguments its row lists, or `null` where the suite has no such call.
//! Every call is stateless JSON: the documents it takes and gives carry
//! their wire level (`version`), and a session's `view` says in `layout`
//! which host the stage draws it into.

use serde::Serialize;

/// The protocol's calls, in the order every suite lists them.
pub const CALLS: [&str; 12] = [
    "read",
    "start",
    "view",
    "rules",
    "candidates",
    "apply",
    "undo",
    "close",
    "hint",
    "draw",
    "export_as",
    "certificate",
];

/// One of the protocol's calls as a suite offers it.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Offered {
    /// The protocol's name.
    pub call: &'static str,
    /// The module's export, `None` where the suite has no such call.
    pub export: Option<&'static str>,
    /// Its arguments, in order: `session`, `goal`, `settings`, ….
    pub takes: &'static [&'static str],
}

/// A call no suite's row names: one a suite has beyond the protocol's.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Extra {
    /// The module's export.
    pub export: &'static str,
    /// Its arguments, in order.
    pub takes: &'static [&'static str],
    /// What it does.
    pub does: &'static str,
}

/// A suite's calls, layouts and methods.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Speaks {
    /// The suite, as `about` names it.
    pub suite: &'static str,
    /// The protocol's calls, in the order of [`CALLS`].
    pub calls: [Offered; 12],
    /// The suite's calls beyond the protocol's.
    pub extra: &'static [Extra],
    /// The hosts its views draw into: `tree` (a derivation), `list` (a
    /// Fitch proof), `graph` and `text`.
    pub layouts: &'static [&'static str],
    /// How a proof of the suite is built, each method with the suite whose
    /// calls it runs where that is another (natural deduction for ordinary
    /// logic runs a calculus given as data). `about`'s `methods` are the
    /// ordinary suite's ways of deciding, which this key leaves as they are.
    pub builds: &'static [Method],
}

/// A way of building a proof.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Method {
    /// Its name: `sequent`, `natural`.
    pub method: &'static str,
    /// The suite whose calls build it, where it is another suite's.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs: Option<&'static str>,
    /// For `runs` `rules`: the built-in calculus each logic is proved in.
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub calculi: &'static [(&'static str, &'static str)],
    /// The layouts its views draw into.
    pub layouts: &'static [&'static str],
}

/// Returns the protocol's call `call` as `export` with its arguments.
const fn offered(
    call: &'static str,
    export: &'static str,
    takes: &'static [&'static str],
) -> Offered {
    Offered {
        call,
        export: Some(export),
        takes,
    }
}

/// Returns the protocol's call `call` as one the suite does not have.
const fn lacks(call: &'static str) -> Offered {
    Offered {
        call,
        export: None,
        takes: &[],
    }
}

/// The arguments of a call on a session alone.
const SESSION: &[&str] = &["session", "settings"];

/// The arguments of a call on a session's goal.
const GOAL: &[&str] = &["session", "goal", "settings"];

/// The arguments of a step.
const STEP: &[&str] = &["session", "goal", "step", "settings"];

/// The arguments of an export.
const FORMAT: &[&str] = &["session", "format", "settings"];

/// The sequent calculus, the method of the linear and the ordinary
/// suite.
const SEQUENT: Method = Method {
    method: "sequent",
    runs: None,
    calculi: &[],
    layouts: &["tree"],
};

/// The labelled sequent calculus, the method of the modal suite: each
/// formula at a world, decided by a search, with a countermodel drawn as
/// a graph.
const LABELLED: Method = Method {
    method: "labelled",
    runs: None,
    calculi: &[],
    layouts: &["tree", "graph"],
};

/// The typing derivation, the method of the lambda suite: a checked
/// term's judgements drawn as a tree, which nobody builds step by step.
const TYPING: Method = Method {
    method: "typing",
    runs: None,
    calculi: &[],
    layouts: &["tree"],
};

/// Every suite of the module, as the protocol has it.
pub const SUITES: [Speaks; 5] = [
    Speaks {
        suite: "linear",
        calls: [
            offered("read", "read_as", &["input", "mode", "settings"]),
            offered("start", "start", &["sequent", "mode", "settings"]),
            offered("view", "view", SESSION),
            offered(
                "rules",
                "rules",
                &["session", "goal", "position", "settings"],
            ),
            offered(
                "candidates",
                "candidates",
                &["session", "goal", "position", "settings"],
            ),
            offered("apply", "apply", STEP),
            offered("undo", "undo", SESSION),
            offered("close", "close", GOAL),
            offered("hint", "hint", GOAL),
            offered("draw", "draw", SESSION),
            offered("export_as", "export_as", FORMAT),
            offered("certificate", "certificate", &["document", "settings"]),
        ],
        extra: &[
            Extra {
                export: "close_all",
                takes: SESSION,
                does: "closes every open goal by the search, within one budget",
            },
            Extra {
                export: "cut",
                takes: &["session", "goal", "formula", "left", "settings"],
                does: "cuts a goal on a formula typed in the mode's syntax",
            },
            Extra {
                export: "invert_all",
                takes: SESSION,
                does: "applies every invertible rule to every open goal",
            },
            Extra {
                export: "fill",
                takes: &["session", "goal", "source", "settings"],
                does: "fills a goal with another tab's finished proof",
            },
            Extra {
                export: "draw_net",
                takes: SESSION,
                does: "draws a finished proof's net",
            },
        ],
        layouts: &["tree"],
        builds: &[SEQUENT],
    },
    Speaks {
        suite: "ordinary",
        calls: [
            offered("read", "ordinary_read", &["input", "logic", "settings"]),
            offered("start", "ordinary_start", &["sequent", "logic", "settings"]),
            offered("view", "ordinary_view", SESSION),
            offered(
                "rules",
                "ordinary_rules",
                &["session", "goal", "side", "position", "settings"],
            ),
            offered(
                "candidates",
                "ordinary_candidates",
                &["session", "goal", "side", "position", "settings"],
            ),
            offered("apply", "ordinary_apply", STEP),
            offered("undo", "ordinary_undo", SESSION),
            offered("close", "ordinary_close", GOAL),
            offered("hint", "ordinary_hint", GOAL),
            offered("draw", "ordinary_draw", SESSION),
            offered("export_as", "ordinary_export", FORMAT),
            offered(
                "certificate",
                "ordinary_certificate",
                &["outcome", "settings"],
            ),
        ],
        extra: &[
            Extra {
                export: "ordinary_table",
                takes: &["sequent", "settings"],
                does: "decides a classical sequent by its truth table",
            },
            Extra {
                export: "ordinary_tableau",
                takes: &["sequent", "settings"],
                does: "decides a classical sequent by a signed tableau",
            },
            Extra {
                export: "ordinary_invert_all",
                takes: SESSION,
                does: "applies every invertible rule of LK or LJ to every open goal",
            },
            Extra {
                export: "ordinary_tableau_review",
                takes: &["text", "sequent", "settings"],
                does: "reviews a classical tableau written by hand, step by step",
            },
            Extra {
                export: "ordinary_translate",
                takes: &["sequent", "options", "settings"],
                does: "translates a classical sequent negatively and compares the verdicts",
            },
            Extra {
                export: "ordinary_interpolate",
                takes: &["sequent", "logic", "partition", "settings"],
                does: "finds a Craig interpolant and checks it",
            },
            Extra {
                export: "ordinary_normalize",
                takes: &["sequent", "options", "settings"],
                does: "rewrites a classical sequent into a normal form step by step and checks it",
            },
            Extra {
                export: "ordinary_resolve",
                takes: &["sequent", "options", "settings"],
                does: "refutes the clauses of a classical sequent's negation by resolution or DPLL",
            },
            Extra {
                export: "ordinary_clauses",
                takes: &["sequent", "form", "settings"],
                does: "starts a refutation by hand from a classical sequent's clauses",
            },
            Extra {
                export: "ordinary_resolve_step",
                takes: &["refutation", "first", "second", "atom", "settings"],
                does: "adds a resolvent to a refutation by hand",
            },
            Extra {
                export: "ordinary_refutation_check",
                takes: &["refutation", "settings"],
                does: "checks a refutation by hand, finished or not",
            },
            Extra {
                export: "ordinary_model",
                takes: &["model", "sequent", "logic", "world", "settings"],
                does: "checks a Kripke model the page gives",
            },
            Extra {
                export: "ordinary_structure",
                takes: &["structure", "sequent", "logic", "settings"],
                does: "checks a finite structure the page gives, formula by formula",
            },
            Extra {
                export: "ordinary_model_drawing",
                takes: &["outcome", "settings"],
                does: "draws an outcome's countermodel",
            },
        ],
        layouts: &["tree", "list"],
        builds: &[
            SEQUENT,
            Method {
                method: "natural",
                runs: Some("rules"),
                calculi: &[
                    ("classical", "NK"),
                    ("intuitionistic", "NJ"),
                    ("minimal", "NM"),
                ],
                layouts: &["tree", "list"],
            },
        ],
    },
    Speaks {
        suite: "rules",
        calls: [
            offered("read", "rules_read", &["calculus", "settings"]),
            offered(
                "start",
                "rules_start",
                &["calculus", "goal", "assumptions", "settings"],
            ),
            offered("view", "rules_view", SESSION),
            offered("rules", "rules_rules", GOAL),
            offered("candidates", "rules_candidates", GOAL),
            offered("apply", "rules_apply", STEP),
            offered("undo", "rules_undo", SESSION),
            lacks("close"),
            lacks("hint"),
            offered("draw", "rules_draw", SESSION),
            offered("export_as", "rules_export", FORMAT),
            lacks("certificate"),
        ],
        extra: &[
            Extra {
                export: "rules_calculus",
                takes: &["name"],
                does: "gives a built-in calculus's rule file",
            },
            Extra {
                export: "rules_derive",
                takes: STEP,
                does: "takes a forward step from lines in force",
            },
            Extra {
                export: "rules_cite",
                takes: &["session", "goal", "line", "settings"],
                does: "closes a goal by a line equal to it",
            },
            Extra {
                export: "rules_assume",
                takes: GOAL,
                does: "closes a goal by a hypothesis in force",
            },
            Extra {
                export: "rules_check",
                takes: SESSION,
                does: "checks a finished derivation whole",
            },
        ],
        layouts: &["tree", "list"],
        builds: &[SEQUENT],
    },
    Speaks {
        suite: "modal",
        calls: [
            offered("read", "modal_read", &["input", "system", "settings"]),
            lacks("start"),
            lacks("view"),
            lacks("rules"),
            lacks("candidates"),
            lacks("apply"),
            lacks("undo"),
            lacks("close"),
            lacks("hint"),
            offered("draw", "modal_draw", &["outcome", "settings"]),
            offered(
                "export_as",
                "modal_export",
                &["outcome", "format", "settings"],
            ),
            lacks("certificate"),
        ],
        extra: &[
            Extra {
                export: "modal_decide",
                takes: &["statement", "settings"],
                does: "decides the statement in its system: a derivation or a countermodel",
            },
            Extra {
                export: "modal_countermodel",
                takes: &["outcome", "settings"],
                does: "gives a not-valid outcome's Kripke model",
            },
            Extra {
                export: "modal_evaluate",
                takes: &["statement", "model", "world", "settings"],
                does: "checks a Kripke model against the statement, at a world or every world",
            },
            Extra {
                export: "modal_model_drawing",
                takes: &["statement", "model", "world", "settings"],
                does: "draws a model that refutes the statement at a world",
            },
            Extra {
                export: "modal_calculus",
                takes: &["statement", "settings"],
                does: "gives the system's labelled calculus as a rule file",
            },
        ],
        layouts: &["tree", "graph"],
        builds: &[LABELLED],
    },
    Speaks {
        suite: "lambda",
        calls: [
            offered("read", "lambda_read", &["input", "calculus", "settings"]),
            lacks("start"),
            lacks("view"),
            lacks("rules"),
            lacks("candidates"),
            lacks("apply"),
            lacks("undo"),
            lacks("close"),
            lacks("hint"),
            offered("draw", "lambda_draw", &["typing", "settings"]),
            offered(
                "export_as",
                "lambda_export",
                &["typing", "format", "settings"],
            ),
            lacks("certificate"),
        ],
        extra: &[
            Extra {
                export: "lambda_check",
                takes: &["term", "settings"],
                does: "checks a term: the type of every subterm, or why it is ill typed",
            },
            Extra {
                export: "lambda_reduce",
                takes: &["term", "options", "settings"],
                does: "reduces a term by β step by step, and marks the next redex",
            },
            Extra {
                export: "lambda_infer",
                takes: &["term", "settings"],
                does: "types an untyped term at its principal simple type, or says it has none",
            },
            Extra {
                export: "lambda_read_type",
                takes: &["input", "settings"],
                does: "reads a type, a formula of intuitionistic logic",
            },
            Extra {
                export: "lambda_inhabit",
                takes: &["type", "settings"],
                does: "finds a term of a type, or shows it has none",
            },
            Extra {
                export: "lambda_enumerate",
                takes: &["type", "options", "settings"],
                does: "lists and counts a type's terms",
            },
        ],
        layouts: &["tree"],
        builds: &[TYPING],
    },
];

/// Returns the protocol as JSON: `about`'s suites, each with its `calls`,
/// `extra`, `layouts` and `builds`, and under `catalogue` the library's
/// list of each suite's kinds and operations
/// ([`deduxit::protocol::catalogue`]), which the workspace runs.
pub fn describe() -> String {
    let mut about: serde_json::Value =
        serde_json::from_str(&crate::calls::about()).unwrap_or_default();
    if let Some(suites) = about["suites"].as_array_mut() {
        for row in suites {
            let Some(speaks) = SUITES.iter().find(|speaks| row["suite"] == speaks.suite) else {
                continue;
            };
            if let (Some(row), Ok(serde_json::Value::Object(speaks))) =
                (row.as_object_mut(), serde_json::to_value(speaks))
            {
                for (key, value) in speaks {
                    if key != "suite" {
                        row.insert(key, value);
                    }
                }
            }
        }
    }
    about["calls"] = serde_json::json!(CALLS);
    about["catalogue"] = serde_json::to_value(deduxit::protocol::catalogue()).unwrap_or_default();
    about.to_string()
}
