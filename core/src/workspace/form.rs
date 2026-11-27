// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A workspace's JSON form, wire level 19.
//!
//! `{"version": 19, "deduxit", "next", "values": [{"kind", "value"}],
//! "nodes": [...], "records": [...], "log": {"at", "entries"}}`. A value is
//! its kind's document as its writer wrote it, byte for byte. A node is
//! `id` with `value` (given), or `operation`, `from`, `parameters` and
//! `settings` and then `record` (made by its recipe), `value` and
//! `started` (a session), or `value` (a copy). A record is its key
//! (`operation`, `parameters`, `settings`, `inputs`), `end`, `output` and
//! `check`. Reading checks every value with its kind's reader, the numbers
//! and the order, the bounds, and runs every record of an instant operation
//! again (one that differs is dropped) and every instant check; a check
//! that needs a search is read as `recorded`.

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use serde_json::value::RawValue;

use super::{
    Body, End, Entry, Key, Node, NodeId, Recipe, Record, RecordId, Swap, ValueId, Workspace,
};
use crate::limits::Held;
use crate::protocol::{self, Check, Class, Cost, Status};
use crate::{Error, Limits, wire};

/// The document.
#[derive(serde::Serialize, serde::Deserialize)]
struct Form {
    /// The wire level.
    #[serde(deserialize_with = "wire::version")]
    version: Option<u32>,
    /// The build that wrote it.
    #[serde(default)]
    deduxit: Option<String>,
    /// The next node's number.
    next: u32,
    /// The values.
    values: Vec<ValueForm>,
    /// The nodes alive.
    nodes: Vec<NodeForm>,
    /// The records.
    #[serde(default)]
    records: Vec<RecordForm>,
    /// The log.
    #[serde(default)]
    log: LogForm,
}

/// A value.
#[derive(serde::Serialize, serde::Deserialize)]
struct ValueForm {
    /// Its kind.
    kind: String,
    /// Its JSON.
    value: Box<RawValue>,
}

/// A node.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct NodeForm {
    /// Its number.
    id: u32,
    /// Its value, for a given node, a session or a copy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value: Option<u32>,
    /// Its recipe's operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    operation: Option<String>,
    /// Its recipe's inputs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    from: Vec<u32>,
    /// Its recipe's parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parameters: Option<Box<RawValue>>,
    /// Its recipe's settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    settings: Option<Box<RawValue>>,
    /// The record it shows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    record: Option<u32>,
    /// The inputs' values a session was started from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    started: Option<Vec<u32>>,
}

/// A record.
#[derive(serde::Serialize, serde::Deserialize)]
struct RecordForm {
    /// The operation.
    operation: String,
    /// The parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parameters: Option<Box<RawValue>>,
    /// The settings read.
    settings: Box<RawValue>,
    /// The inputs' values.
    inputs: Vec<u32>,
    /// How the run ended.
    end: EndForm,
    /// Its output.
    #[serde(default)]
    output: Option<u32>,
    /// The check.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    check: Option<Check>,
    /// Whether another build of deduxit made it, where the document's own
    /// build did not: a file written again keeps another build's records
    /// out of date.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    foreign: bool,
}

/// How a run ended.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum EndForm {
    /// To its end.
    Done,
    /// At a bound.
    Limit {
        /// The library's name for it.
        reason: String,
        /// The settings key that lifts it.
        #[serde(default)]
        setting: Option<String>,
    },
    /// Refused, with the library's error.
    Refused(Box<RawValue>),
}

/// The log.
#[derive(Default, serde::Serialize, serde::Deserialize)]
struct LogForm {
    /// How many entries are applied.
    at: usize,
    /// The entries.
    entries: Vec<EntryForm>,
}

/// An entry of the log.
#[derive(serde::Serialize, serde::Deserialize)]
struct EntryForm {
    /// Its name.
    name: String,
    /// What it changed.
    swaps: Vec<SwapForm>,
    /// The records it pins.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pins: Vec<u32>,
}

/// A node before and after an edit.
#[derive(serde::Serialize, serde::Deserialize)]
struct SwapForm {
    /// The node.
    id: u32,
    /// Before.
    before: Option<NodeForm>,
    /// After.
    after: Option<NodeForm>,
}

/// Returns JSON text as a raw value.
fn raw(text: &str) -> Result<Box<RawValue>, serde_json::Error> {
    RawValue::from_string(text.to_owned())
}

impl Workspace {
    /// Returns a node's form, its records by their numbers in `records`.
    fn node_form(
        &self,
        node: &Node,
        records: &[Option<u32>],
    ) -> Result<NodeForm, serde_json::Error> {
        let mut form = NodeForm {
            id: node.id.0,
            value: None,
            operation: None,
            from: Vec::new(),
            parameters: None,
            settings: None,
            record: None,
            started: None,
        };
        if let Some(recipe) = node.recipe() {
            form.operation = Some(recipe.operation.name.to_owned());
            form.from = recipe.from.iter().map(|id| id.0).collect();
            if !recipe.parameters.is_empty() {
                form.parameters = Some(raw(&recipe.parameters)?);
            }
            form.settings = Some(raw(&recipe.settings)?);
        }
        match &node.body {
            Body::Given(value) | Body::Copy { value, .. } => form.value = Some(value.0),
            Body::Work { value, started, .. } => {
                form.value = Some(value.0);
                form.started = Some(started.iter().map(|v| v.0).collect());
            }
            Body::Derived { record, .. } => {
                form.record = record.and_then(|r| records.get(r.0 as usize).copied().flatten());
            }
        }
        Ok(form)
    }
}

impl Serialize for Workspace {
    /// Writes the workspace's document at wire level 19.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let fail = |error: serde_json::Error| <S::Error as serde::ser::Error>::custom(error);
        // The records alive, numbered again in order.
        let mut numbers = vec![None; self.records.len()];
        let mut kept = Vec::new();
        for (at, record) in self.records.iter().enumerate() {
            if let Some(record) = record {
                numbers[at] = Some(
                    u32::try_from(kept.len()).map_err(<S::Error as serde::ser::Error>::custom)?,
                );
                kept.push(record);
            }
        }
        let values = self
            .values
            .iter()
            .map(|value| {
                Ok(ValueForm {
                    kind: value.kind.name.to_owned(),
                    value: raw(&value.text)?,
                })
            })
            .collect::<Result<Vec<_>, serde_json::Error>>()
            .map_err(fail)?;
        let nodes = self
            .nodes
            .iter()
            .map(|node| self.node_form(node, &numbers))
            .collect::<Result<Vec<_>, _>>()
            .map_err(fail)?;
        let records = kept
            .iter()
            .map(|record| {
                Ok(RecordForm {
                    operation: record.key.operation.to_owned(),
                    parameters: if record.key.parameters.is_empty() {
                        None
                    } else {
                        Some(raw(&record.key.parameters)?)
                    },
                    settings: raw(&record.key.settings)?,
                    inputs: record.key.inputs.iter().map(|v| v.0).collect(),
                    end: match &record.end {
                        End::Done => EndForm::Done,
                        End::Limit(limit) => EndForm::Limit {
                            reason: limit.reason.to_owned(),
                            setting: limit.setting.map(str::to_owned),
                        },
                        End::Refused(error) => EndForm::Refused(raw(error)?),
                    },
                    output: record.output.map(|v| v.0),
                    check: record.check.clone(),
                    foreign: record.foreign,
                })
            })
            .collect::<Result<Vec<_>, serde_json::Error>>()
            .map_err(fail)?;
        let entries = self
            .log
            .iter()
            .map(|entry| {
                Ok(EntryForm {
                    name: entry.name.to_string(),
                    swaps: entry
                        .swaps
                        .iter()
                        .map(|swap| {
                            Ok(SwapForm {
                                id: swap.id.0,
                                before: swap
                                    .before
                                    .as_ref()
                                    .map(|n| self.node_form(n, &numbers))
                                    .transpose()?,
                                after: swap
                                    .after
                                    .as_ref()
                                    .map(|n| self.node_form(n, &numbers))
                                    .transpose()?,
                            })
                        })
                        .collect::<Result<Vec<_>, serde_json::Error>>()?,
                    pins: entry
                        .pins
                        .iter()
                        .filter_map(|pin| numbers.get(pin.0 as usize).copied().flatten())
                        .collect(),
                })
            })
            .collect::<Result<Vec<_>, serde_json::Error>>()
            .map_err(fail)?;
        Form {
            version: Some(wire::WORKSPACE),
            deduxit: Some(env!("CARGO_PKG_VERSION").to_owned()),
            next: self.next,
            values,
            nodes,
            records,
            log: LogForm {
                at: self.at,
                entries,
            },
        }
        .serialize(serializer)
    }
}

impl crate::sealed::Readable for Workspace {}

impl wire::Readable for Workspace {
    const FORM: &'static str = "workspace";

    /// Reads a workspace within `limits`: every value by its kind's
    /// reader, the bounds of `limits.workspace`, and each record of an
    /// instant operation and each instant check made again.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let form = Form::deserialize(deserializer)?;
        built(form, limits, Trust::Claim).map_err(wire::fail)
    }
}

impl<'de> Deserialize<'de> for Workspace {
    /// Reads a workspace within the default limits.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <Self as wire::Readable>::read(deserializer, &Limits::default())
    }
}

/// Returns the error of a document whose numbers do not fit.
fn malformed(message: &str) -> Error {
    Error::Json {
        form: "workspace",
        message: message.to_owned(),
    }
}

/// Refuses a count past a bound.
fn bounded(what: Held, count: usize, bound: Option<u64>) -> Result<(), Error> {
    match super::past(count, bound) {
        Some(limit) => Err(super::refused(what, count, limit)),
        None => Ok(()),
    }
}

/// How far a document's records are believed.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Trust {
    /// A document from elsewhere: each instant run and check is made again,
    /// and a search's check is `recorded`.
    Claim,
    /// A document this program wrote and kept itself: its records stand as
    /// written.
    Own,
}

impl Workspace {
    /// Reads a document this program wrote and kept itself, such as a
    /// page's between two calls, trusting its records: the numbers, the
    /// order, the log and the bounds are checked and every value is read
    /// by its kind's reader, as [`wire::upgrade`] does, but the runs and
    /// the checks stand as written (another build's records are still
    /// another build's).
    ///
    /// # Security
    ///
    /// A record read here answers a check's own runs, so a document that
    /// claims a check held makes it hold. A document from elsewhere, a
    /// link's, a file's or another program's, goes through
    /// [`wire::upgrade`], which makes each instant run again and reads a
    /// search's check as `recorded`.
    ///
    /// # Errors
    ///
    /// A document that is no workspace's, and what [`wire::upgrade`]
    /// refuses but the runs.
    pub fn read_trusted(document: &str, limits: &Limits) -> Result<Self, Error> {
        let form: Form = serde_json::from_str(document).map_err(|error| Error::Json {
            form: "workspace",
            message: error.to_string(),
        })?;
        built(form, limits, Trust::Own)
    }

    /// [`read_trusted`](Self::read_trusted), under its earlier name.
    ///
    /// # Errors
    ///
    /// Those of [`read_trusted`](Self::read_trusted).
    #[deprecated(
        since = "0.9.0",
        note = "named `read_trusted`, which says that it trusts the document's records"
    )]
    pub fn read_own(document: &str, limits: &Limits) -> Result<Self, Error> {
        Self::read_trusted(document, limits)
    }
}

/// Builds a workspace from its form.
fn built(form: Form, limits: &Limits, trust: Trust) -> Result<Workspace, Error> {
    let bounds = &limits.workspace;
    bounded(Held::Nodes, form.nodes.len(), bounds.nodes)?;
    bounded(Held::Records, form.records.len(), bounds.records)?;
    bounded(Held::Log, form.log.entries.len(), bounds.log)?;
    let bytes: usize = form.values.iter().map(|v| v.value.get().len()).sum();
    bounded(Held::Bytes, bytes, bounds.bytes)?;
    let foreign = form.deduxit.as_deref() != Some(env!("CARGO_PKG_VERSION"));
    let mut workspace = Workspace::new();
    let unbounded = Limits {
        workspace: crate::limits::WorkspaceLimits::UNBOUNDED,
        ..*limits
    };
    // The values, each read by its kind's reader; equal ones become one.
    let mut values = Vec::with_capacity(form.values.len());
    for value in &form.values {
        let (_, kind) = protocol::kind(&value.kind)?;
        let text = value.value.get();
        protocol::read(kind.name, text, limits)?;
        values.push(workspace.intern(kind, text, &unbounded)?);
    }
    let value = |index: u32| -> Result<ValueId, Error> {
        values
            .get(index as usize)
            .copied()
            .ok_or_else(|| malformed("a value's number is past the values"))
    };
    // The records, their keys read against the operations.
    let mut records = Vec::with_capacity(form.records.len());
    for record in &form.records {
        let (_, operation) = protocol::operation(&record.operation)?;
        let inputs = record
            .inputs
            .iter()
            .map(|&i| value(i))
            .collect::<Result<Vec<_>, _>>()?;
        if inputs.len() != operation.from.len() {
            return Err(malformed(
                "a record has another number of inputs than its operation",
            ));
        }
        let end = match &record.end {
            EndForm::Done => End::Done,
            EndForm::Limit { reason, setting } => {
                End::Limit(super::limit_named(reason, setting.as_deref()))
            }
            EndForm::Refused(error) => End::Refused(error.get().into()),
        };
        let output = record.output.map(value).transpose()?;
        if output.is_none() && !matches!(end, End::Refused(_)) {
            return Err(malformed("a record that was not refused has no output"));
        }
        records.push(Some(Record {
            key: Key {
                operation: operation.name,
                parameters: super::canonical(record.parameters.as_ref().map_or("", |p| p.get()))?,
                settings: super::canonical(record.settings.get())?,
                inputs,
            },
            end,
            output,
            check: record.check.clone(),
            foreign: foreign || record.foreign,
            trusted: trust == Trust::Own && !(foreign || record.foreign),
            made: 0,
        }));
    }
    workspace.records = records;
    let record = |index: u32| -> Result<RecordId, Error> {
        if (index as usize) < form.records.len() {
            Ok(RecordId(index))
        } else {
            Err(malformed("a record's number is past the records"))
        }
    };
    // A node of the log is checked against the nodes of its own time when
    // the log is replayed, below; here only its inputs' numbers.
    let node = |form: &NodeForm, alive: Option<&[NodeId]>| -> Result<Node, Error> {
        let id = NodeId(form.id);
        let body = match &form.operation {
            None => Body::Given(value(
                form.value
                    .ok_or_else(|| malformed("a given node has no value"))?,
            )?),
            Some(name) => {
                let (_, operation) = protocol::operation(name)?;
                let from: Vec<NodeId> = form.from.iter().map(|&i| NodeId(i)).collect();
                if from.len() != operation.from.len()
                    || from.iter().any(|input| {
                        *input >= id
                            || alive.is_some_and(|alive| alive.binary_search(input).is_err())
                    })
                {
                    return Err(malformed("a recipe names a node that is not before it"));
                }
                let recipe = Recipe {
                    operation,
                    from,
                    parameters: super::canonical(form.parameters.as_ref().map_or("", |p| p.get()))?,
                    settings: super::canonical(form.settings.as_ref().map_or("{}", |s| s.get()))?,
                };
                match (operation.class, form.value, &form.started) {
                    (Class::Derived, None, None) => Body::Derived {
                        recipe,
                        record: form.record.map(record).transpose()?,
                    },
                    (Class::Work, Some(v), Some(started)) => Body::Work {
                        recipe,
                        value: value(v)?,
                        started: started
                            .iter()
                            .map(|&i| value(i))
                            .collect::<Result<_, _>>()?,
                    },
                    (Class::Copy, Some(v), None) => Body::Copy {
                        recipe,
                        value: value(v)?,
                    },
                    _ => return Err(malformed("a node's keys do not fit its operation")),
                }
            }
        };
        Ok(Node { id, body })
    };
    // The nodes alive, in order, each made from nodes before it.
    let mut alive: Vec<NodeId> = Vec::with_capacity(form.nodes.len());
    for found in &form.nodes {
        let id = NodeId(found.id);
        if alive.last().is_some_and(|&last| last >= id) || found.id >= form.next {
            return Err(malformed(
                "the nodes are not in order, or a number is past `next`",
            ));
        }
        let built = node(found, Some(&alive))?;
        workspace.nodes.push(built);
        alive.push(id);
    }
    fits(&workspace, limits)?;
    workspace.next = form.next;
    // The log: its nodes are read whole; a removed node's inputs need not
    // stand any more.
    if form.log.at > form.log.entries.len() {
        return Err(malformed("the log applies more entries than it has"));
    }
    for entry in &form.log.entries {
        let mut swaps = Vec::with_capacity(entry.swaps.len());
        for swap in &entry.swaps {
            let before = swap.before.as_ref().map(|n| node(n, None)).transpose()?;
            let after = swap.after.as_ref().map(|n| node(n, None)).transpose()?;
            if swap.id >= form.next
                || [&before, &after]
                    .into_iter()
                    .flatten()
                    .any(|n| n.id.0 != swap.id)
            {
                return Err(malformed(
                    "a change of the log names another node than its own, or one past `next`",
                ));
            }
            swaps.push(Swap {
                id: NodeId(swap.id),
                before,
                after,
            });
        }
        workspace.log.push(Entry {
            name: entry.name.as_str().into(),
            swaps,
            pins: entry
                .pins
                .iter()
                .map(|&p| record(p))
                .collect::<Result<_, _>>()?,
        });
    }
    workspace.at = form.log.at;
    replayed(&mut workspace, limits)?;
    workspace.index = workspace
        .records
        .iter()
        .enumerate()
        .filter_map(|(at, r)| {
            r.as_ref().map(|r| {
                (
                    r.key.clone(),
                    RecordId(u32::try_from(at).unwrap_or(u32::MAX)),
                )
            })
        })
        .collect();
    if trust == Trust::Claim {
        verify(&mut workspace)?;
    }
    workspace.settle();
    Ok(workspace)
}

/// Checks the nodes alive: within the bound on nodes, and each recipe's
/// inputs alive, before it and of kinds its operation takes.
fn fits(workspace: &Workspace, limits: &Limits) -> Result<(), Error> {
    bounded(Held::Nodes, workspace.nodes.len(), limits.workspace.nodes)?;
    for found in &workspace.nodes {
        if let Some(recipe) = found.recipe() {
            for (input, kinds) in recipe.from.iter().zip(recipe.operation.from) {
                let input = workspace
                    .node(*input)
                    .filter(|input| input.id < found.id)
                    .ok_or_else(|| no_input(found.id))?;
                if !kinds.contains(&workspace.kind_of(input)) {
                    return Err(no_input(found.id));
                }
            }
        }
    }
    Ok(())
}

/// Returns a node as the log compares it: without the record it shows,
/// which runs change, and without a session's state, which its steps
/// change; the log holds neither.
fn placed(node: Option<&Node>) -> Option<Node> {
    node.cloned().map(|mut node| {
        match &mut node.body {
            Body::Derived { record, .. } => *record = None,
            Body::Work { value, .. } => *value = ValueId(u32::MAX),
            Body::Given(_) | Body::Copy { .. } => {}
        }
        node
    })
}

/// Puts one change of the log, `to` in place of `from`, after checking that
/// `from` is what stands there now.
fn swapped(
    workspace: &mut Workspace,
    id: NodeId,
    from: Option<&Node>,
    to: Option<Node>,
) -> Result<(), Error> {
    if placed(workspace.node(id)) != placed(from) {
        return Err(malformed(
            "the log does not lead to the nodes it stands beside",
        ));
    }
    workspace.put(id, to);
    Ok(())
}

/// Replays the log from the nodes alive: every edit taken back to the
/// first, brought back to the last and taken back again to where the
/// document stands, each change checked against the node it replaces and
/// each state with [`fits`]. Undo and redo then reach no other states.
fn replayed(workspace: &mut Workspace, limits: &Limits) -> Result<(), Error> {
    let log = std::mem::take(&mut workspace.log);
    // The log's nodes hold no record shown; the nodes as read come back.
    let alive = workspace.nodes.clone();
    let back = |workspace: &mut Workspace, entry: &Entry| -> Result<(), Error> {
        for swap in entry.swaps.iter().rev() {
            swapped(workspace, swap.id, swap.after.as_ref(), swap.before.clone())?;
        }
        fits(workspace, limits)
    };
    let forth = |workspace: &mut Workspace, entry: &Entry| -> Result<(), Error> {
        for swap in &entry.swaps {
            swapped(workspace, swap.id, swap.before.as_ref(), swap.after.clone())?;
        }
        fits(workspace, limits)
    };
    let done = (|| {
        for entry in log[..workspace.at].iter().rev() {
            back(workspace, entry)?;
        }
        for entry in &log {
            forth(workspace, entry)?;
        }
        for entry in log[workspace.at..].iter().rev() {
            back(workspace, entry)?;
        }
        Ok(())
    })();
    workspace.log = log;
    workspace.nodes = alive;
    done
}

/// Returns the error of a recipe whose input is of a kind it does not take.
fn no_input(node: NodeId) -> Error {
    Error::NotApplicable {
        operation: "read".into(),
        reason: format!("node {node} is made from a node of a kind its operation does not take"),
    }
}

/// Makes each record of an instant operation again and drops one whose
/// output differs, makes each instant check again, and reads a check that
/// needs a search as `recorded`: a document from elsewhere is a claim.
fn verify(workspace: &mut Workspace) -> Result<(), Error> {
    for at in 0..workspace.records.len() {
        let Some(record) = &workspace.records[at] else {
            continue;
        };
        let foreign = record.foreign;
        let (suite, operation) = protocol::operation(record.key.operation)?;
        let texts: Vec<String> = record
            .key
            .inputs
            .iter()
            .map(|&v| workspace.text(v).to_owned())
            .collect();
        let texts: Vec<&str> = texts.iter().map(String::as_str).collect();
        let settings = super::settings_of(&record.key.settings)?;
        let output = record.output.map(|v| workspace.text(v).to_owned());
        let mut keep = true;
        if operation.cost == Cost::Instant && !foreign {
            let made = protocol::run(
                operation.name,
                &texts,
                &record.key.parameters,
                &settings,
                |_| false,
            );
            keep = match (&made, &output) {
                (Ok(made), Some(output)) => made.value == *output,
                (Err(_), None) => true,
                _ => false,
            };
        }
        let check = match (&record.check, &output) {
            (Some(stored), Some(output)) if keep => Some(match operation.promise.check {
                Some(Cost::Instant) => {
                    let call =
                        protocol::Call::new(operation, &texts, &record.key.parameters, &settings);
                    let mut ask = |name: &str, inputs: &[&str], parameters: &str| {
                        protocol::run(name, inputs, parameters, &settings, |_| false)
                            .map(|p| p.value)
                    };
                    suite.check(&call, output, &mut ask, &mut |_| false)?
                }
                // A held check is a claim until made again; a failed one
                // keeps what follows from it held back.
                Some(Cost::Search) => Check {
                    status: if stored.status == Status::Held {
                        Status::Recorded
                    } else {
                        stored.status
                    },
                    ..stored.clone()
                },
                _ => stored.clone(),
            }),
            _ => None,
        };
        if keep {
            let made_again = operation.cost == Cost::Instant && !foreign;
            if let Some(Some(record)) = workspace.records.get_mut(at) {
                record.check = check;
                record.trusted = made_again;
            }
        } else {
            workspace.records[at] = None;
            for node in &mut workspace.nodes {
                if let Body::Derived { record, .. } = &mut node.body
                    && *record == Some(RecordId(u32::try_from(at).unwrap_or(u32::MAX)))
                {
                    *record = None;
                }
            }
        }
    }
    workspace.index = workspace
        .records
        .iter()
        .enumerate()
        .filter_map(|(at, r)| {
            r.as_ref().map(|r| {
                (
                    r.key.clone(),
                    RecordId(u32::try_from(at).unwrap_or(u32::MAX)),
                )
            })
        })
        .collect();
    Ok(())
}
