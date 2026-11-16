// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The protocol every suite's calls speak, written as data: [`describe`].
//!
//! A suite offers the stage's calls by one set of names (`read`, `start`,
//! `view`, `rules`, `apply`, `undo`, `close`, `hint`, `draw`, `export_as`,
//! `certificate`), each exported under the suite's prefix with the
//! arguments its row lists, or `null` where the suite has no such call.
//! Every call is stateless JSON: the documents it takes and gives carry
//! their wire level (`version`), and a session's `view` says in `layout`
//! which host the stage draws it into.

use serde::Serialize;

/// The protocol's calls, in the order every suite lists them.
pub const CALLS: [&str; 11] = [
    "read",
    "start",
    "view",
    "rules",
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
    pub calls: [Offered; 11],
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

/// Every suite of the module, as the protocol has it.
pub const SUITES: [Speaks; 3] = [
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
                export: "ordinary_model",
                takes: &["model", "sequent", "logic", "world", "settings"],
                does: "checks a Kripke model the page gives",
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
