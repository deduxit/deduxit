// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! `workspace`: sequents and what is made from them, kept in a file.

use std::fmt::Write as _;
use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result, bail};
use deduxit::workspace::{Cause, NodeId, State, What, Workspace};
use deduxit::{ErrorKind, Settings, protocol, wire};

use crate::argument_parsing::{AnswerFormat, StatementArgs, WorkspaceCommand};
use crate::limit::Deadline;
use crate::prove::on_large_stack;
use crate::settings::{Given, time_limit};
use crate::{Status, catch_interrupt, interrupted, io};

/// Runs a workspace's command.
///
/// # Errors
///
/// A file that cannot be read or written, and an edit or a run the
/// library refuses.
pub(crate) fn run(command: &WorkspaceCommand, given: &Given) -> Result<Status> {
    let defaults = Settings::default();
    match command {
        WorkspaceCommand::New {
            file,
            sequent,
            statement,
        } => {
            if file.exists() {
                bail!(
                    "{} exists already; a new workspace is not written over a file",
                    file.display()
                );
            }
            let mut workspace = Workspace::new();
            let mut touched = Vec::new();
            if let Some(sequent) = sequent {
                let value = statement_of(statement, sequent, &defaults)?;
                touched.push(workspace.edit("sequent", &defaults.limits, |edit| {
                    edit.add(kind_of(statement), &value)
                })?);
            }
            save(file, &workspace)?;
            list(&workspace, &touched)
        }
        WorkspaceCommand::Add {
            file,
            sequent,
            statement,
        } => {
            let mut workspace = load(file, &defaults)?;
            let value = statement_of(statement, sequent, &defaults)?;
            let node = workspace.edit("sequent", &defaults.limits, |edit| {
                edit.add(kind_of(statement), &value)
            })?;
            save(file, &workspace)?;
            list(&workspace, &[node])
        }
        WorkspaceCommand::Apply {
            file,
            words,
            set,
            no_run,
            common,
        } => {
            let settings = common.settings(given)?;
            let mut workspace = load(file, &settings)?;
            let Some((operation, nodes)) = words.split_last() else {
                bail!("name the input nodes, then the operation");
            };
            let nodes = nodes
                .iter()
                .map(|word| node_of(word))
                .collect::<Result<Vec<_>>>()?;
            let parameters = parameters_of(set)?;
            let node = workspace.edit(operation, &settings.limits, |edit| {
                edit.connect(operation, &nodes, &parameters, &settings)
            })?;
            let status = if *no_run {
                Status::Yes
            } else {
                running(&mut workspace, &[node], What::Check, &settings)?
            };
            save(file, &workspace)?;
            list(&workspace, &[node])?;
            Ok(status)
        }
        WorkspaceCommand::Set {
            file,
            node,
            sequent,
            set,
            common,
        } => {
            let settings = common.settings(given)?;
            let mut workspace = load(file, &settings)?;
            let node = NodeId::new(*node);
            let view = workspace
                .nodes()
                .find(|view| view.id == node)
                .with_context(|| format!("the workspace has no node {node}"))?;
            let value = match sequent {
                Some(text) => Some(retyped(view.kind, view.value, text, &settings)?),
                None => None,
            };
            let parameters = if set.is_empty() {
                None
            } else {
                Some(parameters_of(set)?)
            };
            let limits_given = ["memory_limit", "work_limit"]
                .iter()
                .any(|flag| given.has(flag))
                || common.settings.is_some();
            let name = if value.is_some() {
                "sequent change"
            } else if parameters.is_some() {
                "parameters change"
            } else {
                "limits change"
            };
            workspace.edit(name, &settings.limits, |edit| {
                if let Some(value) = &value {
                    edit.set(node, value)?;
                }
                if let Some(parameters) = &parameters {
                    edit.set_parameters(node, parameters)?;
                }
                if limits_given {
                    edit.set_settings(node, &settings)?;
                }
                Ok(())
            })?;
            save(file, &workspace)?;
            let after: Vec<NodeId> = workspace
                .nodes()
                .filter(|view| view.id == node || matches!(view.state, State::OutOfDate(_)))
                .map(|view| view.id)
                .collect();
            list(&workspace, &after)
        }
        WorkspaceCommand::Run {
            file,
            nodes,
            checks,
            common,
        } => {
            let settings = common.settings(given)?;
            let mut workspace = load(file, &settings)?;
            let targets: Vec<NodeId> = if nodes.is_empty() {
                workspace.nodes().map(|view| view.id).collect()
            } else {
                nodes.iter().copied().map(NodeId::new).collect()
            };
            let what = if *checks { What::Check } else { What::Output };
            let planned = workspace.plan(&targets);
            let status = running(&mut workspace, &targets, what, &settings)?;
            save(file, &workspace)?;
            list(
                &workspace,
                if planned.is_empty() && !*checks {
                    &[]
                } else {
                    &targets
                },
            )?;
            Ok(status)
        }
        WorkspaceCommand::Remove { file, node } => {
            let mut workspace = load(file, &defaults)?;
            let gone = workspace.edit("removal", &defaults.limits, |edit| {
                edit.remove(NodeId::new(*node))
            })?;
            save(file, &workspace)?;
            let gone: Vec<String> = gone.iter().map(ToString::to_string).collect();
            io::write(None, &format!("removed: {}", gone.join(", ")))?;
            Ok(Status::Yes)
        }
        WorkspaceCommand::Undo { file } | WorkspaceCommand::Redo { file } => {
            let mut workspace = load(file, &defaults)?;
            let undo = matches!(command, WorkspaceCommand::Undo { .. });
            let done = if undo {
                workspace.undo()
            } else {
                workspace.redo()
            };
            let Some(name) = done else {
                io::write(
                    None,
                    if undo {
                        "nothing to undo"
                    } else {
                        "nothing to redo"
                    },
                )?;
                return Ok(Status::Yes);
            };
            save(file, &workspace)?;
            let stale = out_of_date(&workspace);
            let mut line = format!("{}: {name}.", if undo { "undone" } else { "redone" });
            if stale.is_empty() {
                line.push_str(" Nothing is out of date.");
            } else {
                write!(line, " Out of date: {}.", stale.join(", "))?;
            }
            io::write(None, &line)?;
            Ok(Status::Yes)
        }
        WorkspaceCommand::Log { file } => {
            let workspace = load(file, &defaults)?;
            let mut text = String::new();
            match workspace.undoable() {
                Some(name) => write!(text, "undo takes back: {name}")?,
                None => text.push_str("nothing to undo"),
            }
            match workspace.redoable() {
                Some(name) => write!(text, "\nredo brings back: {name}")?,
                None => text.push_str("\nnothing to redo"),
            }
            io::write(None, &text)?;
            Ok(Status::Yes)
        }
        WorkspaceCommand::Show {
            file,
            node,
            format,
            output,
        } => {
            let workspace = load(file, &defaults)?;
            match (node, format) {
                (None, AnswerFormat::Json) => {
                    io::write(output.as_deref(), &serde_json::to_string(&workspace)?)?;
                    Ok(Status::Yes)
                }
                (None, AnswerFormat::Text) => {
                    let all: Vec<NodeId> = workspace.nodes().map(|view| view.id).collect();
                    shown(&workspace, &all, output.as_deref())
                }
                (Some(node), AnswerFormat::Json) => {
                    let value = workspace
                        .value(NodeId::new(*node))
                        .with_context(|| format!("node {node} has no value"))?;
                    io::write(output.as_deref(), value)?;
                    Ok(Status::Yes)
                }
                (Some(node), AnswerFormat::Text) => {
                    shown(&workspace, &[NodeId::new(*node)], output.as_deref())
                }
            }
        }
        WorkspaceCommand::Check { file, common } => {
            let settings = common.settings(given)?;
            let mut workspace = load(file, &settings)?;
            let all: Vec<NodeId> = workspace.nodes().map(|view| view.id).collect();
            let mut status = running(&mut workspace, &all, What::Check, &settings)?;
            for view in workspace.nodes() {
                status = status.worse(match view.check.map(|check| check.status) {
                    Some(protocol::Status::Failed) => Status::No,
                    Some(protocol::Status::Open) => Status::Unknown,
                    _ => Status::Yes,
                });
            }
            list(&workspace, &all)?;
            Ok(status)
        }
        WorkspaceCommand::Operations { kind } => {
            let mut text = String::new();
            for suite in protocol::suites() {
                for operation in suite.operations() {
                    let takes = operation
                        .from
                        .iter()
                        .any(|kinds| kind.as_deref().is_none_or(|k| kinds.contains(&k)));
                    if takes {
                        let from: Vec<String> = operation
                            .from
                            .iter()
                            .map(|kinds| kinds.join(" or "))
                            .collect();
                        writeln!(
                            text,
                            "{}  {} -> {}: {}",
                            operation.name,
                            from.join(", "),
                            operation.to,
                            operation.label
                        )?;
                    }
                }
            }
            io::write(None, text.trim_end())?;
            Ok(Status::Yes)
        }
    }
}

/// Returns the kind of a sequent typed with these flags.
const fn kind_of(statement: &StatementArgs) -> &'static str {
    if statement.logic.is_some() {
        "ordinary.sequent"
    } else {
        "linear.sequent"
    }
}

/// Returns the value of a sequent typed with these flags.
fn statement_of(statement: &StatementArgs, text: &str, settings: &Settings) -> Result<String> {
    Ok(match statement.logic {
        Some(logic) => {
            let logic: deduxit::ordinary::Logic = logic.into();
            protocol::statement("ordinary", text, logic.name(), &settings.limits)?
        }
        None => protocol::statement("linear", text, &statement.mode, &settings.limits)?,
    })
}

/// Returns the value a given node takes for a new sequent, in its node's
/// calculus.
fn retyped(kind: &str, old: Option<&str>, text: &str, settings: &Settings) -> Result<String> {
    let old: serde_json::Value = serde_json::from_str(old.unwrap_or("{}"))?;
    let (suite, parameter) = match kind {
        "ordinary.sequent" => ("ordinary", old["logic"].as_str().unwrap_or("classical")),
        "linear.sequent" => ("linear", old["mode"].as_str().unwrap_or("classical")),
        other => bail!("a node of kind {other} takes no sequent"),
    };
    Ok(protocol::statement(
        suite,
        text,
        parameter,
        &settings.limits,
    )?)
}

/// Returns a node named by its number.
fn node_of(word: &str) -> Result<NodeId> {
    word.parse().map(NodeId::new).with_context(|| {
        format!("{word:?} is no node's number; name the nodes first, then the operation")
    })
}

/// Returns the parameters `--set` gives, as a JSON object.
fn parameters_of(set: &[String]) -> Result<String> {
    if set.is_empty() {
        return Ok(String::new());
    }
    let mut object = serde_json::Map::new();
    for pair in set {
        let (key, value) = pair
            .split_once('=')
            .with_context(|| format!("--set {pair:?} is no KEY=VALUE"))?;
        let value = serde_json::from_str(value)
            .unwrap_or_else(|_| serde_json::Value::String(value.to_owned()));
        object.insert(key.to_owned(), value);
    }
    Ok(serde_json::Value::Object(object).to_string())
}

/// Reads a workspace file within the settings' limits.
fn load(file: &Path, settings: &Settings) -> Result<Workspace> {
    let text = io::read_configuration(file, "workspace file")?;
    let mut document = serde_json::Deserializer::from_str(&text);
    let workspace = wire::upgrade(&mut document, &settings.limits)?;
    document
        .end()
        .with_context(|| format!("{} holds more than one workspace", file.display()))?;
    Ok(workspace)
}

/// Writes a workspace file whole, beside and then renamed.
fn save(file: &Path, workspace: &Workspace) -> Result<()> {
    io::write(Some(file), &serde_json::to_string(workspace)?)
}

/// Runs the nodes `targets` need on the large stack, under the time limit
/// and Ctrl-C, and returns the worst verdict of the outcomes run.
fn running(
    workspace: &mut Workspace,
    targets: &[NodeId],
    what: What,
    settings: &Settings,
) -> Result<Status> {
    catch_interrupt();
    let deadline = Deadline::start(time_limit(settings), Instant::now())?;
    let limits = settings.limits;
    let ran = on_large_stack(limits.stack_bytes(), || {
        workspace.run(targets, what, &limits, |_| {
            interrupted() || deadline.passed()
        })
    })?;
    let ran = match ran {
        Ok(ran) => ran,
        Err(error) if error.kind() == ErrorKind::Stopped => {
            eprintln!(
                "unknown: the time limit or Ctrl-C stopped a run; the nodes run before it stay"
            );
            return Ok(Status::Unknown);
        }
        Err(error) => return Err(error.into()),
    };
    let mut status = Status::Yes;
    for ran in ran {
        let view = workspace.nodes().find(|view| view.id == ran.node);
        if let Some(view) = view {
            status = status.worse(verdict_status(view.value, view.end, view.check));
        }
    }
    Ok(status)
}

/// Returns the exit status of a node's run: its verdict where it is an
/// outcome, an error where it was refused or its check failed.
fn verdict_status(
    value: Option<&str>,
    end: Option<&deduxit::workspace::End>,
    check: Option<&protocol::Check>,
) -> Status {
    if check.is_some_and(|check| check.status == protocol::Status::Failed)
        || matches!(end, Some(deduxit::workspace::End::Refused(_)))
    {
        return Status::Error;
    }
    let verdict = value
        .and_then(|value| serde_json::from_str::<serde_json::Value>(value).ok())
        .and_then(|value| value["verdict"].as_str().map(str::to_owned));
    match verdict.as_deref() {
        Some("not_valid" | "unprovable") => Status::No,
        Some("unknown") => Status::Unknown,
        _ => Status::Yes,
    }
}

/// Returns the nodes out of date, by their numbers.
fn out_of_date(workspace: &Workspace) -> Vec<String> {
    workspace
        .nodes()
        .filter(|view| matches!(view.state, State::OutOfDate(_)))
        .map(|view| view.id.to_string())
        .collect()
}

/// Writes the lines of the nodes `nodes` on standard output.
fn list(workspace: &Workspace, nodes: &[NodeId]) -> Result<Status> {
    shown(workspace, nodes, None)
}

/// Writes the lines of the nodes `nodes`: the number, the kind and the
/// value in a line, then how it was made and the check, and the state
/// where the node is not current.
fn shown(workspace: &Workspace, nodes: &[NodeId], output: Option<&Path>) -> Result<Status> {
    let limits = Settings::default().limits;
    let mut text = String::new();
    for view in workspace.nodes().filter(|view| nodes.contains(&view.id)) {
        let summary = match view.value {
            Some(value) => {
                let mut line = protocol::summary(view.kind, value, &limits)
                    .unwrap_or_else(|_| "a value".to_owned());
                let read: serde_json::Value = serde_json::from_str(value).unwrap_or_default();
                if let Some(calculus) = read["logic"].as_str().or_else(|| read["mode"].as_str())
                    && view.kind.ends_with(".sequent")
                {
                    write!(line, " ({calculus})")?;
                }
                line
            }
            None => "no value yet".to_owned(),
        };
        writeln!(text, "{}  {}  {summary}", view.id, view.kind)?;
        if let Some(operation) = view.operation {
            let from: Vec<String> = view.from.iter().map(ToString::to_string).collect();
            write!(text, "   from {} by {operation}", from.join(", "))?;
            if let Some(parameters) = view.parameters.filter(|p| !p.is_empty())
                && let Ok(serde_json::Value::Object(map)) = serde_json::from_str(parameters)
            {
                for (key, value) in map {
                    let value = value
                        .as_str()
                        .map_or_else(|| value.to_string(), str::to_owned);
                    write!(text, " {key}={value}")?;
                }
            }
            if let Some(check) = view.check {
                write!(text, "; check: {}", status_word(check.status))?;
            }
            text.push('\n');
        }
        let state = match view.state {
            State::Given | State::Current => None,
            State::NotRun => Some("not run".to_owned()),
            State::OutOfDate(Cause::Input(node)) => {
                Some(format!("out of date: node {node} changed"))
            }
            State::OutOfDate(Cause::Recipe) => {
                Some("out of date: its parameters or limits changed".to_owned())
            }
            State::OutOfDate(Cause::Version) => {
                Some("out of date: another build of deduxit made it".to_owned())
            }
            State::OutOfDate(Cause::Waits(node)) => {
                Some(format!("out of date: it waits for node {node}"))
            }
            State::OutOfDate(_) | _ => Some("out of date".to_owned()),
        };
        if let Some(state) = state {
            writeln!(text, "   {state}")?;
        }
    }
    io::write(output, text.trim_end())?;
    Ok(Status::Yes)
}

/// Returns a check's status in words.
const fn status_word(status: protocol::Status) -> &'static str {
    match status {
        protocol::Status::Held => "held",
        protocol::Status::Failed => "failed",
        protocol::Status::Open => "open",
        protocol::Status::Recorded => "recorded",
        protocol::Status::Outside => "made by Rocq",
        _ => "none",
    }
}
