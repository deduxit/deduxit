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
    let json = serde_json::to_string(&workspace).unwrap();
    let read = |json: &str| {
        let mut document = serde_json::Deserializer::from_str(json);
        wire::upgrade::<Workspace, _>(&mut document, &limits).unwrap()
    };
    assert_eq!(read(&json).state(translated), State::Current);
    // Point the record's output at the other given sequent's value.
    let forged = edited(&json, r#""output":2"#, r#""output":1"#);
    assert_eq!(read(&forged).state(translated), State::NotRun);
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
    // A node the workspace lacks is named with the nodes it has.
    let error = workspace
        .edit("missing", &limits, |edit| {
            edit.set(NodeId(99), &classical("|- d"))
        })
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "index 99 names no node: there are 2 nodes"
    );
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

#[test]
fn a_link_holds_recipes_and_sessions_but_no_result() {
    let limits = Limits::default();
    let settings = Settings::default();
    let mut workspace = Workspace::new();
    let (given, translated, session, decided) = workspace
        .edit("translation", &limits, |edit| {
            let given = edit.add("ordinary.sequent", &classical("|- a \\/ ~a"))?;
            let translated = edit.connect(
                "ordinary.translate",
                &[given],
                r#"{"translation": "glivenko"}"#,
                &settings,
            )?;
            let decided = edit.connect("ordinary.decide", &[translated], "", &settings)?;
            let session = edit.connect("ordinary.start", &[given], "", &settings)?;
            Ok((given, translated, session, decided))
        })
        .unwrap();
    workspace
        .run(&[decided], What::Check, &limits, |_| false)
        .unwrap();
    let packed = workspace.pack(&[translated]);
    let ids: Vec<NodeId> = packed.nodes().map(|node| node.id).collect();
    assert_eq!(ids, vec![given, translated]);
    assert_eq!(packed.state(translated), State::NotRun);
    assert_eq!(packed.undoable(), None);
    let packed = workspace.pack(&[session]);
    assert_eq!(
        packed.state(session),
        State::Current,
        "a session is in the link"
    );
}

/// Returns a workspace of two classical sequents, each decided, and the
/// numbers of the first sequent and of the two decisions.
fn two_decided(limits: &Limits) -> (Workspace, NodeId, NodeId, NodeId) {
    let settings = Settings::default();
    let mut workspace = Workspace::new();
    let (first, decided, other) = workspace
        .edit("two", limits, |edit| {
            let first = edit.add("ordinary.sequent", &classical("a |- a"))?;
            let decided = edit.connect("ordinary.decide", &[first], "", &settings)?;
            let second = edit.add("ordinary.sequent", &classical("a |- b"))?;
            let other = edit.connect("ordinary.decide", &[second], "", &settings)?;
            Ok((first, decided, other))
        })
        .unwrap();
    workspace
        .run(&[decided, other], What::Output, limits, |_| false)
        .unwrap();
    (workspace, first, decided, other)
}

/// Returns the document with its first `from` replaced by `to`: an edit of
/// the text, since a round trip through `serde_json::Value` reorders a
/// value's keys, which makes it another value.
fn edited(json: &str, from: &str, to: &str) -> String {
    assert!(json.contains(from), "{from} is not in {json}");
    json.replacen(from, to, 1)
}

/// Reads a document within `limits`.
fn read(json: &str, limits: &Limits) -> Result<Workspace, Error> {
    let mut document = serde_json::Deserializer::from_str(json);
    wire::upgrade(&mut document, limits)
}

#[test]
fn a_run_the_caller_stopped_is_refused_and_kept_nowhere() {
    let limits = Limits::default();
    let settings = Settings::default();
    let statement = protocol::statement(
        "linear",
        "!(A -o A * A), !(A -o B), !(B * B -o C), A |- C * ?D",
        "classical",
        &limits,
    )
    .unwrap();
    let mut workspace = Workspace::new();
    let proved = workspace
        .edit("prove", &limits, |edit| {
            let given = edit.add("linear.sequent", &statement)?;
            edit.connect("linear.prove", &[given], "", &settings)
        })
        .unwrap();
    let error = workspace
        .run(&[proved], What::Output, &limits, |_| true)
        .unwrap_err();
    assert_eq!(error.code(), "stopped");
    assert_eq!(workspace.state(proved), State::NotRun);
    assert!(workspace.value(proved).is_none());
}

#[test]
fn another_build_s_record_stays_out_of_date_until_run_again() {
    let limits = Limits::default();
    let (workspace, _, decided, _) = two_decided(&limits);
    let json = edited(
        &serde_json::to_string(&workspace).unwrap(),
        concat!(r#""deduxit":""#, env!("CARGO_PKG_VERSION")),
        r#""deduxit":"0.0.1"#,
    );
    let mut read_back = read(&json, &limits).unwrap();
    assert_eq!(read_back.state(decided), State::OutOfDate(Cause::Version));
    // Written again by this build, the record is still another build's.
    read_back
        .edit("add", &limits, |edit| {
            edit.add("ordinary.sequent", &classical("b |- b"))
        })
        .unwrap();
    let again = read(&serde_json::to_string(&read_back).unwrap(), &limits).unwrap();
    assert_eq!(again.state(decided), State::OutOfDate(Cause::Version));
    // A run replaces it.
    read_back
        .run(&[decided], What::Output, &limits, |_| false)
        .unwrap();
    assert_eq!(read_back.state(decided), State::Current);
    current_is_fresh(&read_back);
}

#[test]
fn undo_shows_again_what_a_stale_node_showed() {
    let limits = Limits::default();
    let (mut workspace, first, _, _) = two_decided(&limits);
    workspace
        .edit("sequent change", &limits, |edit| {
            edit.set(first, &classical("|- c \\/ d"))
        })
        .unwrap();
    let before = seen(&workspace);
    // The new value has a record kept: the other decision's.
    workspace
        .edit("sequent change", &limits, |edit| {
            edit.set(first, &classical("a |- b"))
        })
        .unwrap();
    workspace.undo().unwrap();
    assert_eq!(seen(&workspace), before);
}

#[test]
fn a_check_answers_only_from_this_build_s_runs() {
    let limits = Limits::default();
    let settings = Settings::default();
    let (mut workspace, _, decided, _) = two_decided(&limits);
    let input = workspace
        .nodes()
        .find(|node| node.id == decided)
        .map(|node| node.from[0])
        .unwrap();
    let text = workspace.value(input).unwrap().to_owned();
    assert!(
        workspace
            .asked("ordinary.decide", &[&text], "", &settings)
            .is_some()
    );
    for record in workspace.records.iter_mut().flatten() {
        record.trusted = false;
    }
    assert!(
        workspace
            .asked("ordinary.decide", &[&text], "", &settings)
            .is_none()
    );
}

#[test]
fn a_recorded_check_is_made_again_and_a_failed_one_stays() {
    let limits = Limits::default();
    let settings = Settings::default();
    let mut workspace = Workspace::new();
    let translated = workspace
        .edit("translation", &limits, |edit| {
            let given = edit.add("ordinary.sequent", &classical("|- a \\/ ~a"))?;
            edit.connect(
                "ordinary.translate",
                &[given],
                r#"{"translation": "glivenko"}"#,
                &settings,
            )
        })
        .unwrap();
    workspace
        .run(&[translated], What::Check, &limits, |_| false)
        .unwrap();
    let json = serde_json::to_string(&workspace).unwrap();
    let mut read_back = read(&json, &limits).unwrap();
    let status = |workspace: &Workspace| {
        workspace
            .nodes()
            .find(|node| node.id == translated)
            .and_then(|node| node.check.map(|check| check.status))
    };
    assert_eq!(status(&read_back), Some(Status::Recorded));
    read_back
        .run(&[translated], What::Check, &limits, |_| false)
        .unwrap();
    assert_eq!(status(&read_back), Some(Status::Held));
    let failed = edited(
        &json,
        r#""promise":"same_verdict","status":"held""#,
        r#""promise":"same_verdict","status":"failed""#,
    );
    let mut read_back = read(&failed, &limits).unwrap();
    assert_eq!(status(&read_back), Some(Status::Failed));
    // A failed check a file claims is made again too.
    read_back
        .run(&[translated], What::Check, &limits, |_| false)
        .unwrap();
    assert_eq!(status(&read_back), Some(Status::Held));
}

#[test]
fn a_log_that_does_not_fit_its_nodes_is_refused_on_reading() {
    let limits = Limits::default();
    let (workspace, _, _, _) = two_decided(&limits);
    let json = serde_json::to_value(&workspace).unwrap();
    // A change that names another node than its own.
    let mut other = json.clone();
    other["log"]["entries"][0]["swaps"][0]["id"] = serde_json::json!(3);
    assert_eq!(
        read(&other.to_string(), &limits).unwrap_err().code(),
        "json"
    );
    // A change whose node is not the one standing there.
    let mut moved = json.clone();
    moved["log"]["entries"][0]["swaps"][0]["after"]["value"] = serde_json::json!(1);
    assert_eq!(
        read(&moved.to_string(), &limits).unwrap_err().code(),
        "json"
    );
    // A log whose undo would pass the bound on nodes.
    let given =
        |id: u32| serde_json::json!({"id": id, "before": {"id": id, "value": 0}, "after": null});
    let many = serde_json::json!({
        "version": 19, "deduxit": env!("CARGO_PKG_VERSION"), "next": 3,
        "values": [{"kind": "ordinary.sequent", "value": serde_json::from_str::<serde_json::Value>(&classical("a |- a")).unwrap()}],
        "nodes": [],
        "log": {"at": 1, "entries": [{"name": "removal", "swaps": [given(0), given(1), given(2)]}]},
    });
    let two = Limits::default().with_workspace(WorkspaceLimits::UNBOUNDED.with_nodes(Some(2)));
    assert_eq!(
        read(&many.to_string(), &two).unwrap_err().code(),
        "workspace_limit"
    );
    assert!(read(&many.to_string(), &limits).is_ok());
    // A `next` far past the nodes costs nothing.
    let mut far = many;
    far["next"] = serde_json::json!(u32::MAX);
    assert!(read(&far.to_string(), &limits).is_ok());
}

#[test]
fn rebound_reaches_the_recipes_undo_brings_back() {
    let limits = Limits::default();
    let (mut workspace, _, decided, _) = two_decided(&limits);
    workspace
        .edit("limits change", &limits, |edit| {
            edit.set_settings(
                decided,
                &Settings::default().with_limits(Limits::default().with_work(Some(200))),
            )
        })
        .unwrap();
    workspace.rebound(&Limits::default().with_work(Some(7)));
    workspace.undo().unwrap();
    let settings = workspace
        .nodes()
        .find(|node| node.id == decided)
        .and_then(|node| node.settings.map(str::to_owned))
        .unwrap();
    assert!(settings.contains(r#""limits.work":7"#), "{settings}");
}

#[test]
fn a_proof_of_a_translation_is_made_in_one_action() {
    let limits = Limits::default();
    let settings = Settings::default();
    let mut workspace = Workspace::new();
    let (given, translated, session) = workspace
        .edit("translation", &limits, |edit| {
            let given = edit.add("ordinary.sequent", &classical("|- a \\/ ~a"))?;
            let translated = edit.connect(
                "ordinary.translate",
                &[given],
                r#"{"translation": "glivenko", "target": "intuitionistic"}"#,
                &settings,
            )?;
            let session = edit.connect("ordinary.start", &[translated], "", &settings)?;
            Ok((given, translated, session))
        })
        .unwrap();
    assert_eq!(workspace.state(translated), State::Current);
    assert_eq!(workspace.state(session), State::Current);
    current_is_fresh(&workspace);
    // A changed sequent: the restart makes the translation it needs.
    workspace
        .edit("sequent change", &limits, |edit| {
            edit.set(given, &classical("|- a -> b \\/ ~b"))
        })
        .unwrap();
    let before = seen(&workspace);
    workspace
        .edit("restart", &limits, |edit| edit.restart(session))
        .unwrap();
    assert_eq!(workspace.state(session), State::Current);
    assert_eq!(workspace.state(translated), State::Current);
    current_is_fresh(&workspace);
    workspace.undo().unwrap();
    assert_eq!(seen(&workspace), before);
    workspace.undo().unwrap();
    workspace.undo().unwrap();
    assert_eq!(workspace.nodes().count(), 0);
}

#[test]
fn the_program_s_own_document_keeps_its_checks_and_a_claim_does_not() {
    let limits = Limits::default();
    let settings = Settings::default();
    let mut workspace = Workspace::new();
    let translated = workspace
        .edit("translation", &limits, |edit| {
            let given = edit.add("ordinary.sequent", &classical("|- a \\/ ~a"))?;
            edit.connect(
                "ordinary.translate",
                &[given],
                r#"{"translation": "glivenko"}"#,
                &settings,
            )
        })
        .unwrap();
    workspace
        .run(&[translated], What::Check, &limits, |_| false)
        .unwrap();
    let json = serde_json::to_string(&workspace).unwrap();
    let status = |workspace: &Workspace| {
        workspace
            .nodes()
            .find(|node| node.id == translated)
            .and_then(|node| node.check.map(|check| check.status))
    };
    let own = Workspace::read_trusted(&json, &limits).unwrap();
    assert_eq!(status(&own), Some(Status::Held));
    assert_eq!(seen(&own), seen(&workspace));
    assert!(own.records.iter().flatten().all(|record| record.trusted));
    let claim = read(&json, &limits).unwrap();
    assert_eq!(status(&claim), Some(Status::Recorded));
    // The bounds hold for the program's own document too.
    let one = Limits::default().with_workspace(WorkspaceLimits::UNBOUNDED.with_nodes(Some(1)));
    assert_eq!(
        Workspace::read_trusted(&json, &one).unwrap_err().code(),
        "workspace_limit"
    );
}

/// A run ended by any bound keeps the setting that lifts it: every key a
/// reason or a refusal names is one the workspace knows. Its own list
/// lacked ordinary logic's three (`search.path_length`,
/// `search.model_size`, `search.model_worlds`), so such a record lost
/// its setting.
#[test]
fn every_bound_keeps_its_setting() {
    use crate::limits::{Bound, Held, Phase, Refusal, Space};
    use crate::linear::search::Reason;
    // A new variant fails to compile here until it is listed below.
    let listed = |reason: &Reason| match reason {
        Reason::Stopped
        | Reason::RecursionLimit { .. }
        | Reason::CopyBound { .. }
        | Reason::MemoryLimit { .. }
        | Reason::IndexLimit
        | Reason::WorkLimit { .. }
        | Reason::Unchecked { .. }
        | Reason::WitnessDepth { .. }
        | Reason::Instances { .. }
        | Reason::Incomplete
        | Reason::PathLength { .. }
        | Reason::ModelSize { .. }
        | Reason::KripkeSize { .. } => (),
    };
    let reasons = [
        Reason::Stopped,
        Reason::RecursionLimit { depth: 1 },
        Reason::CopyBound { copies: 1 },
        Reason::MemoryLimit { limit_bytes: 1 },
        Reason::IndexLimit,
        Reason::WorkLimit { limit: 1 },
        Reason::Unchecked { limit_bytes: 1 },
        Reason::WitnessDepth { depth: 1 },
        Reason::Instances { instances: 1 },
        Reason::Incomplete,
        Reason::PathLength { length: 1 },
        Reason::ModelSize { size: 1 },
        Reason::KripkeSize { worlds: 1, size: 1 },
    ];
    let refused = |refusal: &Refusal| match refusal {
        Refusal::Stopped { .. }
        | Refusal::Memory { .. }
        | Refusal::Occurrences { .. }
        | Refusal::Terms { .. }
        | Refusal::Output { .. }
        | Refusal::Work { .. }
        | Refusal::Pixels { .. }
        | Refusal::Elimination { .. }
        | Refusal::Atoms { .. }
        | Refusal::Workspace { .. }
        | Refusal::Index { .. }
        | Refusal::Form { .. } => (),
    };
    let mut refusals = vec![
        Refusal::stopped(Phase::Search),
        Refusal::Memory {
            phase: Phase::Search,
            limit_bytes: 1,
            needed_bytes: None,
        },
        Refusal::Occurrences {
            occurrences: 2,
            limit: 1,
        },
        Refusal::Terms { terms: 2, limit: 1 },
        Refusal::Output {
            what: "derivation",
            estimate_bytes: 2,
            limit_bytes: 1,
            least_bytes: None,
        },
        Refusal::Work { limit: 1 },
        Refusal::Pixels {
            pixels: 2,
            limit: 1,
        },
        Refusal::Atoms { atoms: 2, limit: 1 },
        Refusal::Index {
            what: Space::Term,
            count: 2,
            most: 1,
        },
        Refusal::Form {
            occurrences: 2,
            limit: 1,
        },
    ];
    for what in [Bound::Steps, Bound::Nodes, Bound::Vertices] {
        refusals.push(Refusal::Elimination { what, limit: 1 });
    }
    for what in [Held::Nodes, Held::Records, Held::Log, Held::Bytes] {
        refusals.push(Refusal::Workspace {
            what,
            count: 2,
            limit: 1,
        });
    }
    reasons.iter().for_each(listed);
    refusals.iter().for_each(refused);
    let settings = reasons
        .iter()
        .filter_map(Reason::setting)
        .chain(refusals.iter().filter_map(Refusal::setting));
    for setting in settings {
        assert_eq!(
            super::limit_named("work_limit", Some(setting)).setting,
            Some(setting),
            "{setting}"
        );
    }
    let ended = super::limit_named("model_size", Some("search.model_size"));
    assert_eq!(
        (ended.reason, ended.setting),
        ("model_size", Some("search.model_size"))
    );
}
