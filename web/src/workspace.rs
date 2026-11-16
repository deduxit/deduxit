// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The workspace of linked tabs, over JSON: a page holds the workspace's
//! document and sends it with every call, as it does a session. Exported
//! with the prefix `workspace_`.
//!
//! An edit is an action of the page's, `{"name", "edits": [...]}`, one
//! entry of the workspace's log, which `undo` takes back whole. Each edit
//! is an object of one key:
//! - `{"add": {"kind", "value"}}`, a given value;
//! - `{"set": {"node", "value"}}`, a given node's new value;
//! - `{"connect": {"operation", "from", "parameters"}}`, a node made from
//!   others, under the call's settings;
//! - `{"parameters": {"node", "parameters"}}`, `{"settings": {"node"}}`
//!   (the call's settings), `{"restart": {"node"}}`,
//!   `{"work": {"node", "session"}}`, `{"detach": {"node"}}` and
//!   `{"remove": {"node"}}`.
//!
//! A node is its number, or `{"made": K}`, the node the action's edit K
//! made. `work` maps a proof's node to its session as the page holds it
//! now, laid in before the call without an entry of the log: a step of a
//! proof is the session's own undo. A node the workspace no longer holds
//! (an undo took it away) is left out of `work`.

use crate::failure::Failure;
use crate::tab;
use deduxit::workspace::{Cause, End, NodeId, State, What, Workspace};
use deduxit::{Limits, Settings, protocol, wire};
use serde_json::{Value, json};

/// Reads the page's own document within the settings' limits, its runs
/// and checks as written: the page holds what these calls wrote, and a
/// document from elsewhere comes in through [`open`]. An empty text is an
/// empty workspace.
fn read(document: &str, limits: &Limits) -> Result<Workspace, Failure> {
    if document.trim().is_empty() {
        return Ok(Workspace::new());
    }
    Ok(Workspace::read_own(document, limits)?)
}

/// Writes a workspace's document.
fn written(workspace: &Workspace) -> Result<String, Failure> {
    serde_json::to_string(workspace).map_err(Failure::json)
}

/// Reads a document and lays in the sessions `work` gives.
fn opened(document: &str, work: &str, limits: &Limits) -> Result<Workspace, Failure> {
    let mut workspace = read(document, limits)?;
    if !work.trim().is_empty() {
        let sessions: serde_json::Map<String, Value> =
            serde_json::from_str(work).map_err(Failure::json)?;
        for (node, session) in sessions {
            let node = NodeId::new(node.parse::<u32>().map_err(|_| {
                Failure::refused("json", "malformed", &format!("{node} is no node's number"))
            })?);
            // A node an undo took away is left out: the page learns it from the answer.
            if workspace.nodes().all(|view| view.id != node) {
                continue;
            }
            workspace.work(node, &session.to_string(), limits)?;
        }
    }
    Ok(workspace)
}

/// A node an edit names: a number, or the node an earlier edit of the same
/// action made.
#[derive(Clone, Copy)]
enum Named {
    /// A node of the workspace.
    Node(NodeId),
    /// The node the action's edit of this place made.
    Made(usize),
}

/// One edit of an action, read before any is made.
enum Change {
    /// A given value.
    Add {
        /// Its kind.
        kind: String,
        /// Its document.
        value: String,
    },
    /// A given node's new value.
    Set {
        /// The node.
        node: Named,
        /// Its new document.
        value: String,
    },
    /// A node made from others.
    Connect {
        /// The operation.
        operation: String,
        /// Its inputs.
        from: Vec<Named>,
        /// Its parameters, a JSON object or empty.
        parameters: String,
    },
    /// A node's new parameters.
    Parameters {
        /// The node.
        node: Named,
        /// The parameters, a JSON object or empty.
        parameters: String,
    },
    /// A node's new settings, the call's.
    Settings {
        /// The node.
        node: Named,
    },
    /// A proof started again.
    Restart {
        /// The proof's node.
        node: Named,
    },
    /// A proof's session within the action.
    Work {
        /// The proof's node.
        node: Named,
        /// Its session.
        session: String,
    },
    /// A node cut from its inputs.
    Detach {
        /// The node.
        node: Named,
    },
    /// A node removed, with what is made from it.
    Remove {
        /// The node.
        node: Named,
    },
}

/// Returns the refusal of a malformed action.
fn malformed(message: &str) -> Failure {
    Failure::refused("json", "malformed", message)
}

/// Reads a node an edit names; `{"made": K}` must name an earlier edit
/// that makes a node.
fn named(value: Option<&Value>, makes: &[bool]) -> Result<Named, Failure> {
    let value = value.ok_or_else(|| malformed("the edit names no node"))?;
    let refused = || malformed(&format!("{value} names no node"));
    if let Some(number) = value.as_u64() {
        return Ok(Named::Node(NodeId::new(
            u32::try_from(number).map_err(|_| refused())?,
        )));
    }
    let earlier = value
        .get("made")
        .and_then(Value::as_u64)
        .and_then(|at| usize::try_from(at).ok())
        .filter(|&at| makes.get(at).copied().unwrap_or(false))
        .ok_or_else(refused)?;
    Ok(Named::Made(earlier))
}

/// Returns a member of an edit as text: a string as it is, any other JSON
/// as written, nothing as empty.
fn text_of(edit: &Value, key: &str) -> String {
    match edit.get(key) {
        Some(Value::String(text)) => text.clone(),
        Some(Value::Null) | None => String::new(),
        Some(value) => value.to_string(),
    }
}

/// Reads an action's edits, each checked before any is made.
fn changes(action: &Value) -> Result<Vec<Change>, Failure> {
    let edits = action
        .get("edits")
        .and_then(Value::as_array)
        .ok_or_else(|| malformed("an action lists its `edits`"))?;
    let mut makes = Vec::with_capacity(edits.len());
    let mut read = Vec::with_capacity(edits.len());
    for one in edits {
        let (key, body) = one
            .as_object()
            .filter(|object| object.len() == 1)
            .and_then(|object| object.iter().next())
            .ok_or_else(|| malformed("an edit is an object of one key"))?;
        let node = || named(body.get("node"), &makes);
        let change = match key.as_str() {
            "add" => Change::Add {
                kind: text_of(body, "kind"),
                value: text_of(body, "value"),
            },
            "set" => Change::Set {
                node: node()?,
                value: text_of(body, "value"),
            },
            "connect" => Change::Connect {
                operation: text_of(body, "operation"),
                from: body
                    .get("from")
                    .and_then(Value::as_array)
                    .ok_or_else(|| malformed("a connection lists its `from`"))?
                    .iter()
                    .map(|input| named(Some(input), &makes))
                    .collect::<Result<_, _>>()?,
                parameters: text_of(body, "parameters"),
            },
            "parameters" => Change::Parameters {
                node: node()?,
                parameters: text_of(body, "parameters"),
            },
            "settings" => Change::Settings { node: node()? },
            "restart" => Change::Restart { node: node()? },
            "work" => Change::Work {
                node: node()?,
                session: text_of(body, "session"),
            },
            "detach" => Change::Detach { node: node()? },
            "remove" => Change::Remove { node: node()? },
            other => return Err(malformed(&format!("no edit is called `{other}`"))),
        };
        makes.push(matches!(
            change,
            Change::Add { .. } | Change::Connect { .. }
        ));
        read.push(change);
    }
    Ok(read)
}

/// Makes one action of the page's: every edit of `action` in one entry of
/// the log. Answers `{"workspace", "made"}`, `made` holding per edit the
/// node it made (`add`, `connect`), or for `remove` the nodes removed.
///
/// # Errors
///
/// An action that is no such JSON, and the library's refusal of an edit,
/// after which nothing of the action is kept.
pub fn edit(document: &str, action: &str, work: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let limits = settings.limits;
    let mut workspace = opened(document, work, &limits)?;
    let action: Value = serde_json::from_str(action).map_err(Failure::json)?;
    let name = action
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("change")
        .to_owned();
    let changes = changes(&action)?;
    let made = workspace.edit(&name, &limits, |edit| {
        let mut made: Vec<Option<NodeId>> = Vec::with_capacity(changes.len());
        let mut answers: Vec<Value> = Vec::with_capacity(changes.len());
        for change in &changes {
            // `changes` let a node name only an earlier edit that made one.
            let at = |named: &Named| match *named {
                Named::Node(node) => node,
                Named::Made(earlier) => made[earlier].unwrap_or(NodeId::new(u32::MAX)),
            };
            let mut new = None;
            let mut answer = Value::Null;
            match change {
                Change::Add { kind, value } => new = Some(edit.add(kind, value)?),
                Change::Set { node, value } => edit.set(at(node), value)?,
                Change::Connect {
                    operation,
                    from,
                    parameters,
                } => {
                    let from: Vec<NodeId> = from.iter().map(at).collect();
                    new = Some(edit.connect(operation, &from, parameters, &settings)?);
                }
                Change::Parameters { node, parameters } => {
                    edit.set_parameters(at(node), parameters)?;
                }
                Change::Settings { node } => edit.set_settings(at(node), &settings)?,
                Change::Restart { node } => edit.restart(at(node))?,
                Change::Work { node, session } => edit.work(at(node), session)?,
                Change::Detach { node } => edit.detach(at(node))?,
                Change::Remove { node } => {
                    let gone = edit.remove(at(node))?;
                    answer = json!(gone.iter().map(|id| id.index()).collect::<Vec<_>>());
                }
            }
            if let Some(new) = new {
                answer = json!(new.index());
            }
            made.push(new);
            answers.push(answer);
        }
        Ok(answers)
    })?;
    Ok(answer(&[
        ("workspace", json!(written(&workspace)?).to_string()),
        ("made", json!(made).to_string()),
    ]))
}

/// Returns a JSON object of members whose values are JSON text, written as
/// they are. An answer's document is a JSON string of its text, which the
/// page hands back as it is: read as an object and written again (by
/// `serde_json::Value`, or by JavaScript, which puts a key that looks like
/// a number first), a value's keys could come out in another order, which
/// is another value to the workspace.
fn answer(members: &[(&str, String)]) -> String {
    let mut text = String::from("{");
    for (at, (key, value)) in members.iter().enumerate() {
        if at > 0 {
            text.push(',');
        }
        text.push_str(&json!(key).to_string());
        text.push(':');
        text.push_str(value);
    }
    text.push('}');
    text
}

/// Returns JSON text as a value to nest, read once.
fn raw(text: &str) -> Result<Value, Failure> {
    serde_json::from_str(text).map_err(Failure::json)
}

/// Takes back the last action, or with `forward` brings back the one
/// taken back last. Answers `{"workspace", "name", "sessions"}`: the
/// action's name (`null` with nothing to do) and each proof's session
/// that changed, by node, for the page to show.
///
/// # Errors
///
/// A document or a session the reader refuses.
pub fn undo(document: &str, work: &str, settings: &str, forward: bool) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut workspace = opened(document, work, &settings.limits)?;
    let before = sessions(&workspace);
    let name = if forward {
        workspace.redo()
    } else {
        workspace.undo()
    };
    let after = sessions(&workspace);
    let changed: Vec<(String, String)> = after
        .into_iter()
        .filter(|(node, session)| before.get(node) != Some(session))
        .collect();
    let changed: Vec<(&str, String)> = changed
        .iter()
        .map(|(node, session)| (node.as_str(), session.clone()))
        .collect();
    Ok(answer(&[
        ("workspace", json!(written(&workspace)?).to_string()),
        ("name", json!(name).to_string()),
        ("sessions", answer(&changed)),
    ]))
}

/// Returns each proof's session by its node's number.
fn sessions(workspace: &Workspace) -> std::collections::BTreeMap<String, String> {
    workspace
        .nodes()
        .filter(|node| {
            node.operation.is_some_and(|name| {
                protocol::operation(name)
                    .is_ok_and(|(_, operation)| operation.class == protocol::Class::Work)
            })
        })
        .filter_map(|node| Some((node.id.to_string(), node.value?.to_owned())))
        .collect()
}

/// Reads `what`: `output` or `check`.
fn what_of(what: &str) -> Result<What, Failure> {
    match what {
        "output" => Ok(What::Output),
        "check" => Ok(What::Check),
        other => Err(Failure::refused(
            "unknown_name",
            "malformed",
            &format!("{other} is neither `output` nor `check`"),
        )),
    }
}

/// Computes one node's output, or with `check` its check too, and leaves
/// the workspace as it was: the run a worker makes, whose answer the page
/// hands to [`accept`]. Answers the computed run.
///
/// # Errors
///
/// A node that does not run, an input with no value yet, and a document
/// the reader refuses. An operation's own refusal is the computed output.
pub fn compute(
    document: &str,
    node: u32,
    what: &str,
    work: &str,
    settings: &str,
) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let workspace = opened(document, work, &settings.limits)?;
    let computed = workspace.compute(NodeId::new(node), what_of(what)?, |_| false)?;
    serde_json::to_string(&computed).map_err(Failure::json)
}

/// Puts a computed run in, where the node still asks for exactly that
/// run. Answers `{"workspace", "accepted"}`.
///
/// # Errors
///
/// A computed run that is no such JSON, its output refused by its kind's
/// reader, and a bound of the workspace passed.
pub fn accept(
    document: &str,
    computed: &str,
    work: &str,
    settings: &str,
) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut workspace = opened(document, work, &settings.limits)?;
    let computed = serde_json::from_str(computed).map_err(Failure::json)?;
    let accepted = workspace.accept(computed, &settings.limits)?;
    Ok(answer(&[
        ("workspace", json!(written(&workspace)?).to_string()),
        ("accepted", json!(accepted).to_string()),
    ]))
}

/// Reads a list of node numbers.
fn nodes_of(nodes: &str) -> Result<Vec<NodeId>, Failure> {
    let numbers: Vec<u32> = serde_json::from_str(nodes).map_err(Failure::json)?;
    Ok(numbers.into_iter().map(NodeId::new).collect())
}

/// Runs what the nodes `nodes` need, in order, and with `check` their
/// checks. Answers `{"workspace", "ran"}`, `ran` each node run.
///
/// # Errors
///
/// As [`compute`] and [`accept`]; the runs before an error are lost with
/// the call, so a page that wants each kept runs them one at a time.
pub fn run(
    document: &str,
    nodes: &str,
    what: &str,
    work: &str,
    settings: &str,
) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut workspace = opened(document, work, &settings.limits)?;
    let ran = workspace.run(&nodes_of(nodes)?, what_of(what)?, &settings.limits, |_| {
        false
    })?;
    let ran: Vec<u32> = ran.iter().map(|ran| ran.node.index()).collect();
    Ok(answer(&[
        ("workspace", json!(written(&workspace)?).to_string()),
        ("ran", json!(ran).to_string()),
    ]))
}

/// Returns a state as the page reads it: `{"state"}`, with `cause` and
/// `node` where it is out of date.
fn state_of(state: State) -> Value {
    match state {
        State::Given => json!({"state": "given"}),
        State::Current => json!({"state": "current"}),
        State::NotRun => json!({"state": "not_run"}),
        State::OutOfDate(cause) => {
            let (cause, node) = match cause {
                Cause::Input(node) => ("input", Some(node.index())),
                Cause::Recipe => ("recipe", None),
                Cause::Version => ("version", None),
                Cause::Waits(node) => ("waits", Some(node.index())),
                _ => ("other", None),
            };
            json!({"state": "out_of_date", "cause": cause, "node": node})
        }
        _ => json!({"state": "other"}),
    }
}

/// Returns how a run ended, as the page reads it.
fn end_of(end: Option<&End>) -> Value {
    match end {
        None => Value::Null,
        Some(End::Done) => json!({"end": "done"}),
        Some(End::Limit(limit)) => {
            json!({"end": "limit", "reason": limit.reason, "setting": limit.setting})
        }
        Some(End::Refused(error)) => {
            json!({"end": "refused", "error": raw(error).unwrap_or(Value::Null)})
        }
        Some(_) => json!({"end": "other"}),
    }
}

/// Returns what the page draws of a workspace: per node its number, kind,
/// value, summary, operation with its label, cost (`instant` or `search`)
/// and class (`derived`, `work` or `copy`), inputs, parameters, state, how
/// its run ended, check and column; and the names undo and redo take.
///
/// # Errors
///
/// A document or a session the reader refuses.
pub fn view(document: &str, work: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let limits = settings.limits;
    let workspace = opened(document, work, &limits)?;
    let layers: std::collections::BTreeMap<u32, u32> = workspace
        .layers()
        .into_iter()
        .map(|(node, column)| (node.index(), column))
        .collect();
    let nodes: Vec<Value> = workspace
        .nodes()
        .map(|node| {
            let operation = node
                .operation
                .and_then(|name| protocol::operation(name).ok())
                .map(|(_, operation)| operation);
            let label = operation.map(|operation| operation.label);
            let cost = operation.map(|operation| match operation.cost {
                protocol::Cost::Instant => "instant",
                protocol::Cost::Search => "search",
                _ => "other",
            });
            let class = operation.map(|operation| match operation.class {
                protocol::Class::Derived => "derived",
                protocol::Class::Work => "work",
                protocol::Class::Copy => "copy",
                _ => "other",
            });
            let summary = node
                .value
                .and_then(|value| protocol::summary(node.kind, value, &limits).ok());
            json!({
                "node": node.id.index(),
                "kind": node.kind,
                "value": node.value.map(raw).transpose().ok().flatten(),
                "summary": summary,
                "operation": node.operation,
                "label": label,
                "cost": cost,
                "class": class,
                "from": node.from.iter().map(|id| id.index()).collect::<Vec<_>>(),
                "parameters": node.parameters.filter(|p| !p.is_empty()).map(raw).transpose().ok().flatten(),
                "state": state_of(node.state),
                "end": end_of(node.end),
                "check": node.check,
                "column": layers.get(&node.id.index()),
            })
        })
        .collect();
    Ok(json!({
        "nodes": nodes,
        "undo": workspace.undoable(),
        "redo": workspace.redoable(),
    })
    .to_string())
}

/// Returns the value a sequent typed in a tab is in the workspace: the
/// sequent's document with its calculus, `suite` `ordinary` with a logic
/// or `linear` with a mode.
///
/// # Errors
///
/// An unknown suite, logic or mode, the parser's error, and a sequent past
/// the settings' bounds.
pub fn statement(
    suite: &str,
    text: &str,
    parameter: &str,
    settings: &str,
) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    Ok(protocol::statement(
        suite,
        text,
        parameter,
        &settings.limits,
    )?)
}

/// Reads a document from elsewhere, a link's or a file's, as a claim (each
/// run that needs no search made again, a search's check `recorded`), and
/// gives its recipes the tab's bounds. Answers the document as the page
/// holds it, which every other call reads as the page's own.
///
/// # Errors
///
/// A document the reader refuses within the tab's limits and bounds.
pub fn open(document: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut reader = serde_json::Deserializer::from_str(document);
    let mut workspace: Workspace = wire::upgrade(&mut reader, &settings.limits)?;
    workspace.rebound(&settings.limits);
    written(&workspace)
}

/// Returns the part of a workspace a link holds: the nodes `nodes` and
/// what they are made from, with their recipes and sessions, and no runs
/// and no log.
///
/// # Errors
///
/// A document, a session or a list the reader refuses.
pub fn pack(document: &str, nodes: &str, work: &str, settings: &str) -> Result<String, Failure> {
    let settings: Settings = tab::read(settings)?;
    let workspace = opened(document, work, &settings.limits)?;
    written(&workspace.pack(&nodes_of(nodes)?))
}
