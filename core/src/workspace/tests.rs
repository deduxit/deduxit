// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The workspace's tests: a random walk of edits, runs, undos and redos
//! against a reference that runs every current node afresh, the JSON form,
//! and each bound's refusal.

use super::{Cause, NodeId, State, What, Workspace, settings_of};
use crate::limits::WorkspaceLimits;
use crate::ordinary::Logic;
use crate::protocol::{self, Status};
use crate::{Error, Limits, Settings, wire};

/// Returns a classical statement of `text`.
fn classical(text: &str) -> String {
    protocol::ordinary_statement(&text.parse().unwrap(), Logic::Classical).unwrap()
}

/// What a caller sees of every node: its number, kind, value, recipe,
/// state and the check of the run it shows.
type Seen = Vec<(
    u32,
    String,
    Option<String>,
    Option<String>,
    Vec<u32>,
    Option<String>,
    State,
    Option<Status>,
)>;

/// Returns what a caller sees of the workspace.
fn seen(workspace: &Workspace) -> Seen {
    workspace
        .nodes()
        .map(|node| {
            (
                node.id.index(),
                node.kind.to_owned(),
                node.value.map(str::to_owned),
                node.operation.map(str::to_owned),
                node.from.iter().map(|id| id.index()).collect(),
                node.parameters.map(str::to_owned),
                node.state,
                node.check.map(|check| check.status),
            )
        })
        .collect()
}

/// Checks the reference's claim: every current node shows what its recipe
/// gives on its inputs' values run afresh on one thread.
fn current_is_fresh(workspace: &Workspace) {
    for node in workspace.nodes() {
        if node.state != State::Current {
            continue;
        }
        let Some(operation) = node.operation else {
            continue;
        };
        let inputs: Vec<String> = node
            .from
            .iter()
            .map(|&id| workspace.value(id).unwrap().to_owned())
            .collect();
        let inputs: Vec<&str> = inputs.iter().map(String::as_str).collect();
        let settings = settings_of(node.settings.unwrap()).unwrap();
        let fresh = protocol::run(
            operation,
            &inputs,
            node.parameters.unwrap(),
            &settings,
            |_| false,
        );
        match fresh {
            Ok(fresh) => assert_eq!(node.value, Some(fresh.value.as_str()), "node {}", node.id),
            Err(_) => assert!(
                node.value.is_none(),
                "node {} shows an output its run refuses",
                node.id
            ),
        }
    }
}

/// A small generator of numbers, so that a walk is the same every run.
struct Walk(u64);

impl Walk {
    /// Returns a number below `bound`.
    fn below(&mut self, bound: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        usize::try_from(self.0 % u64::try_from(bound).unwrap()).unwrap()
    }
}

/// The classical sequents a walk gives and changes nodes to.
const SEQUENTS: [&str; 5] = [
    "|- a \\/ ~a",
    "a |- a",
    "a \\/ b |- a",
    "|- (a -> b) -> a -> b",
    "a -> b, b -> c |- a -> c",
];

#[test]
fn a_random_walk_keeps_current_nodes_fresh_and_undo_exact() {
    let limits = Limits::default();
    let settings = Settings::default();
    for seed in 1..=12_u64 {
        let mut walk = Walk(0x9e37_79b9_7f4a_7c15 ^ seed);
        let mut workspace = Workspace::new();
        for step in 0..40 {
            let ids: Vec<NodeId> = workspace.nodes().map(|n| n.id).collect();
            let sequents: Vec<NodeId> = workspace
                .nodes()
                .filter(|n| n.kind == "ordinary.sequent" && n.value.is_some())
                .map(|n| n.id)
                .collect();
            let given: Vec<NodeId> = workspace
                .nodes()
                .filter(|n| n.operation.is_none() && n.kind == "ordinary.sequent")
                .map(|n| n.id)
                .collect();
            let before = seen(&workspace);
            let pick = |walk: &mut Walk, from: &[NodeId]| from[walk.below(from.len())];
            let edited: Option<Result<(), Error>> = match walk.below(9) {
                0 | 1 => {
                    let value = classical(SEQUENTS[walk.below(5)]);
                    Some(
                        workspace
                            .edit("add", &limits, |edit| edit.add("ordinary.sequent", &value))
                            .map(drop),
                    )
                }
                2 if !given.is_empty() => {
                    let node = pick(&mut walk, &given);
                    let value = classical(SEQUENTS[walk.below(5)]);
                    Some(workspace.edit("sequent change", &limits, |edit| edit.set(node, &value)))
                }
                3 if !given.is_empty() => {
                    let node = pick(&mut walk, &given);
                    let translation =
                        ["kolmogorov", "glivenko", "kuroda", "godel-gentzen"][walk.below(4)];
                    let parameters = format!(r#"{{"translation": "{translation}"}}"#);
                    Some(
                        workspace
                            .edit("translation", &limits, |edit| {
                                edit.connect("ordinary.translate", &[node], &parameters, &settings)
                            })
                            .map(drop),
                    )
                }
                4 if !sequents.is_empty() => {
                    let node = pick(&mut walk, &sequents);
                    let operation =
                        ["ordinary.decide", "ordinary.embed", "ordinary.start"][walk.below(3)];
                    Some(
                        workspace
                            .edit("make", &limits, |edit| {
                                edit.connect(operation, &[node], "", &settings)
                            })
                            .map(drop),
                    )
                }
                5 if !ids.is_empty() => {
                    let node = pick(&mut walk, &ids);
                    Some(
                        workspace
                            .edit("removal", &limits, |edit| edit.remove(node))
                            .map(drop),
                    )
                }
                6 => {
                    workspace.undo();
                    None
                }
                7 => {
                    workspace.redo();
                    None
                }
                _ => {
                    let what = if walk.below(2) == 0 {
                        What::Output
                    } else {
                        What::Check
                    };
                    workspace.run(&ids, what, &limits, |_| false).unwrap();
                    None
                }
            };
            // An edit that applied: undo restores what was seen before it,
            // and redo what was seen after it.
            if let Some(Ok(())) = edited {
                let after = seen(&workspace);
                let mut probe = workspace.clone();
                probe.undo().unwrap();
                assert_eq!(
                    seen(&probe),
                    before,
                    "seed {seed}, step {step}: undo restores"
                );
                probe.redo().unwrap();
                assert_eq!(
                    seen(&probe),
                    after,
                    "seed {seed}, step {step}: redo restores"
                );
            }
            current_is_fresh(&workspace);
        }
    }
}

#[test]
fn an_upstream_change_puts_what_follows_out_of_date_and_undo_back() {
    let limits = Limits::default();
    let settings = Settings::default();
    let mut workspace = Workspace::new();
    let (given, translated, decided) = workspace
        .edit("translation", &limits, |edit| {
            let given = edit.add("ordinary.sequent", &classical("|- a \\/ ~a"))?;
            let translated = edit.connect(
                "ordinary.translate",
                &[given],
                r#"{"translation": "glivenko"}"#,
                &settings,
            )?;
            let decided = edit.connect("ordinary.decide", &[translated], "", &settings)?;
            Ok((given, translated, decided))
        })
        .unwrap();
    workspace
        .run(&[decided], What::Check, &limits, |_| false)
        .unwrap();
    assert_eq!(workspace.state(decided), State::Current);
    let shown = workspace.value(decided).unwrap().to_owned();
    workspace
        .edit("sequent change", &limits, |edit| {
            edit.set(given, &classical("|- a -> b \\/ ~b"))
        })
        .unwrap();
    assert_eq!(
        workspace.state(translated),
        State::OutOfDate(Cause::Input(given))
    );
    assert_eq!(
        workspace.state(decided),
        State::OutOfDate(Cause::Waits(translated))
    );
    // The last value stays shown while out of date.
    assert_eq!(workspace.value(decided), Some(shown.as_str()));
    workspace
        .run(&[decided], What::Output, &limits, |_| false)
        .unwrap();
    assert_eq!(workspace.state(decided), State::Current);
    assert_eq!(workspace.undo().as_deref(), Some("sequent change"));
    // The records before the change are shown again, without a run.
    assert_eq!(workspace.state(decided), State::Current);
    assert_eq!(workspace.value(decided), Some(shown.as_str()));
}

#[test]
fn a_session_follows_its_sequent_and_is_never_run_again() {
    let limits = Limits::default();
    let settings = Settings::default();
    let mut workspace = Workspace::new();
    let (given, session) = workspace
        .edit("start", &limits, |edit| {
            let given = edit.add("ordinary.sequent", &classical("a |- a"))?;
            Ok((
                given,
                edit.connect("ordinary.start", &[given], "", &settings)?,
            ))
        })
        .unwrap();
    assert_eq!(workspace.state(session), State::Current);
    let started = workspace.value(session).unwrap().to_owned();
    workspace
        .edit("sequent change", &limits, |edit| {
            edit.set(given, &classical("b |- b"))
        })
        .unwrap();
    assert_eq!(
        workspace.state(session),
        State::OutOfDate(Cause::Input(given))
    );
    assert!(workspace.plan(&[session]).is_empty(), "a session is no run");
    assert_eq!(workspace.value(session), Some(started.as_str()));
    workspace
        .edit("restart", &limits, |edit| edit.restart(session))
        .unwrap();
    assert_eq!(workspace.state(session), State::Current);
    assert_ne!(workspace.value(session), Some(started.as_str()));
    workspace.undo();
    assert_eq!(workspace.value(session), Some(started.as_str()));
}

#[test]
fn the_json_form_reads_back_as_written() {
    let limits = Limits::default();
    let settings = Settings::default();
    let mut workspace = Workspace::new();
    let decided = workspace
        .edit("translation", &limits, |edit| {
            let given = edit.add("ordinary.sequent", &classical("|- a \\/ ~a"))?;
            let translated = edit.connect(
                "ordinary.translate",
                &[given],
                r#"{"translation": "kuroda"}"#,
                &settings,
            )?;
            edit.connect("ordinary.decide", &[translated], "", &settings)
        })
        .unwrap();
    workspace
        .run(&[decided], What::Check, &limits, |_| false)
        .unwrap();
    let json = serde_json::to_string(&workspace).unwrap();
    assert!(json.starts_with(r#"{"version":19,"#));
    let mut document = serde_json::Deserializer::from_str(&json);
    let read: Workspace = wire::upgrade(&mut document, &limits).unwrap();
    assert_eq!(seen(&read), seen(&workspace));
    assert_eq!(serde_json::to_string(&read).unwrap(), json);
    assert_eq!(read.undoable(), Some("translation"));
}

#[test]
fn a_forged_record_is_dropped_on_reading() {
    let limits = Limits::default();
    let settings = Settings::default();
    let mut workspace = Workspace::new();
    let translated = workspace
        .edit("translation", &limits, |edit| {
            let given = edit.add("ordinary.sequent", &classical("|- a"))?;
            edit.add("ordinary.sequent", &classical("|- b"))?;
            edit.connect(
                "ordinary.translate",
                &[given],
                r#"{"translation": "glivenko"}"#,
                &settings,
            )
        })
        .unwrap();
    workspace
        .run(&[translated], What::Output, &limits, |_| false)
        .unwrap();
    let mut json: serde_json::Value = serde_json::to_value(&workspace).unwrap();
    // Point the record's output at the other given sequent's value.
    json["records"][0]["output"] = serde_json::json!(1);
    let read: Workspace = serde_json::from_value(json).unwrap();
    assert_eq!(read.state(translated), State::NotRun);
}

#[test]
fn every_bound_is_a_refusal() {
    let settings = Settings::default();
    let tight = |bounds: WorkspaceLimits| Limits::default().with_workspace(bounds);
    let unbounded = WorkspaceLimits::UNBOUNDED;
    let code = |error: Error| error.code();
    // Nodes.
    let mut workspace = Workspace::new();
    let limits = tight(unbounded.with_nodes(Some(1)));
    workspace
        .edit("add", &limits, |edit| {
            edit.add("ordinary.sequent", &classical("|- a"))
        })
        .unwrap();
    let error = workspace
        .edit("add", &limits, |edit| {
            edit.add("ordinary.sequent", &classical("|- b"))
        })
        .unwrap_err();
    assert_eq!(code(error), "workspace_limit");
    // Bytes.
    let limits = tight(unbounded.with_bytes(Some(10)));
    let error = Workspace::new()
        .edit("add", &limits, |edit| {
            edit.add("ordinary.sequent", &classical("|- a"))
        })
        .unwrap_err();
    assert_eq!(code(error), "workspace_limit");
    // Records: the oldest that nothing shows gives way; with every record
    // shown, the run is refused.
    let limits = tight(unbounded.with_records(Some(1)));
    let mut workspace = Workspace::new();
    let (first, second) = workspace
        .edit("two", &limits, |edit| {
            let a = edit.add("ordinary.sequent", &classical("|- a \\/ ~a"))?;
            let first = edit.connect("ordinary.decide", &[a], "", &settings)?;
            let second = edit.connect("ordinary.translate", &[a], "", &settings)?;
            Ok((first, second))
        })
        .unwrap();
    workspace
        .run(&[first], What::Output, &limits, |_| false)
        .unwrap();
    let error = workspace
        .run(&[second], What::Output, &limits, |_| false)
        .unwrap_err();
    assert_eq!(code(error), "workspace_limit");
    // The log forgets its oldest edit past its bound.
    let limits = tight(unbounded.with_log(Some(1)));
    let mut workspace = Workspace::new();
    workspace
        .edit("first", &limits, |edit| {
            edit.add("ordinary.sequent", &classical("|- a"))
        })
        .unwrap();
    workspace
        .edit("second", &limits, |edit| {
            edit.add("ordinary.sequent", &classical("|- b"))
        })
        .unwrap();
    assert_eq!(workspace.undo().as_deref(), Some("second"));
    assert_eq!(workspace.undo(), None);
    // A document past a bound is refused on reading.
    let mut workspace = Workspace::new();
    let limits = Limits::default();
    for text in ["|- a", "|- b"] {
        workspace
            .edit("add", &limits, |edit| {
                edit.add("ordinary.sequent", &classical(text))
            })
            .unwrap();
    }
    let json = serde_json::to_string(&workspace).unwrap();
    let mut document = serde_json::Deserializer::from_str(&json);
    let error = wire::upgrade::<Workspace, _>(&mut document, &tight(unbounded.with_nodes(Some(1))))
        .unwrap_err();
    assert_eq!(code(error), "workspace_limit");
    let mut document = serde_json::Deserializer::from_str(&json);
    let error = wire::upgrade::<Workspace, _>(&mut document, &tight(unbounded.with_log(Some(1))))
        .unwrap_err();
    assert_eq!(code(error), "workspace_limit");
}

#[test]
fn edits_that_do_not_apply_are_refused_and_leave_nothing() {
    let limits = Limits::default();
    let settings = Settings::default();
    let mut workspace = Workspace::new();
    let (given, decided) = workspace
        .edit("decision", &limits, |edit| {
            let given = edit.add("ordinary.sequent", &classical("|- a"))?;
            Ok((
                given,
                edit.connect("ordinary.decide", &[given], "", &settings)?,
            ))
        })
        .unwrap();
    let before = seen(&workspace);
    let error = workspace
        .edit("set", &limits, |edit| edit.set(decided, &classical("|- b")))
        .unwrap_err();
    assert_eq!(error.code(), "not_given");
    let error = workspace
        .edit("wrong", &limits, |edit| {
            edit.add("ordinary.sequent", &classical("|- c"))?;
            edit.connect("ordinary.half", &[given], "", &settings)
        })
        .unwrap_err();
    assert_eq!(error.code(), "not_applicable");
    let error = workspace
        .edit("detach", &limits, |edit| edit.detach(decided))
        .unwrap_err();
    assert_eq!(error.code(), "not_applicable");
    assert_eq!(seen(&workspace), before, "a refused action keeps nothing");
    assert_eq!(workspace.undoable(), Some("decision"));
}

#[test]
fn rebound_gives_every_recipe_the_caller_s_bounds() {
    let limits = Limits::default();
    let settings = Settings::default();
    let mut workspace = Workspace::new();
    let decided = workspace
        .edit("decision", &limits, |edit| {
            let given = edit.add("ordinary.sequent", &classical("|- a \\/ ~a"))?;
            edit.connect("ordinary.decide", &[given], "", &settings)
        })
        .unwrap();
    workspace
        .run(&[decided], What::Output, &limits, |_| false)
        .unwrap();
    let small = Limits::default().with_work(Some(5));
    assert_eq!(workspace.rebound(&small), vec![decided]);
    assert_eq!(workspace.state(decided), State::OutOfDate(Cause::Recipe));
    let settings = settings_of(workspace.nodes().last().unwrap().settings.unwrap()).unwrap();
    assert_eq!(settings.limits.work, Some(5));
    // Reading a document changes no recipe.
    let json = serde_json::to_string(&workspace).unwrap();
    let read: Workspace = serde_json::from_str(&json).unwrap();
    assert_eq!(
        read.nodes().last().unwrap().settings,
        workspace.nodes().last().unwrap().settings
    );
}
