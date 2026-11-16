// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A workspace of linked values: a sequent, its translation, the
//! translation's proof, a countermodel, a drawing, each a node, and how
//! each was made from the others. Change a value upstream, and what was
//! made from it shows as out of date until it is made again; undo and redo
//! take whole edits back. Needs the cargo feature `protocol` (on by
//! default): the kinds of values and the operations between them are the
//! suites' ([`protocol`]).
//!
//! # The model
//!
//! - A **value** is an artifact as its kind's JSON. Values never change,
//!   and equal values are one.
//! - A **node** holds a value. It is *given* (its value is the user's), or
//!   it has a **recipe**: an operation, its input nodes, its parameters
//!   and the settings it runs under, which the recipe keeps (a change of
//!   the caller's settings changes no recipe).
//! - A **record** is one run of a recipe on values: what went in, what came
//!   out, how the run ended and the check of what the operation promised.
//!   The records are the cache: a recipe on the same inputs, parameters and
//!   settings is the same record.
//! - A node's [`State`] is computed from its recipe, its inputs' values and
//!   the records; nothing stores it.
//! - The **log** holds the edits ([`Workspace::edit`]), each one action of
//!   the user's, which [`undo`](Workspace::undo) and
//!   [`redo`](Workspace::redo) take back and bring back whole. Running is
//!   no edit, and neither is a step of a proof in a node's session
//!   ([`work`](Workspace::work)).
//!
//! The workspace reads no clock and starts no thread; every run takes the
//! caller's stop and the recipe's settings on one thread, so a run's
//! output is a function of its record's key. Its JSON form is a document
//! at wire level 19.
//!
//! # Examples
//!
//! ```
//! use deduxit::{Limits, Settings, workspace::{State, What, Workspace}};
//!
//! let sequent = r#"{"version": 8, "formulas": [{"V": 0}, {"¬": 0}, {"∨": [0, 1]}],
//!     "atoms": ["a"], "left": [], "right": [2], "logic": "classical"}"#;
//! let settings = Settings::default();
//! let limits = Limits::default();
//! let mut workspace = Workspace::new();
//! let (given, translated) = workspace.edit("translation", &limits, |edit| {
//!     let given = edit.add("ordinary.sequent", sequent)?;
//!     let translated = edit.connect(
//!         "ordinary.translate", &[given], r#"{"translation": "glivenko"}"#, &settings,
//!     )?;
//!     Ok((given, translated))
//! })?;
//! assert_eq!(workspace.state(translated), State::NotRun);
//! workspace.run(&[translated], What::Output, &limits, |_| false)?;
//! assert_eq!(workspace.state(translated), State::Current);
//! assert_eq!(workspace.undoable(), Some("translation"));
//! # let _ = given;
//! # Ok::<(), deduxit::Error>(())
//! ```

mod form;
#[cfg(test)]
mod tests;

use serde::Serialize;

use crate::hash::HashMap;
use crate::limits::{Held, Refusal};
use crate::protocol::{self, Check, Class, Kind, Limit, Operation, Produced, Status};
use crate::{Error, ErrorKind, Limits, Progress, Settings};

/// A node's number in its workspace, never used twice there.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct NodeId(u32);

impl NodeId {
    /// Wraps a raw node number.
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw node number.
    pub const fn index(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for NodeId {
    /// Writes the node's number.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A value's place in the workspace's table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ValueId(u32);

/// A record's place in the workspace's table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RecordId(u32);

/// A value: its kind and its JSON.
#[derive(Clone, Debug)]
struct Value {
    /// Its kind.
    kind: &'static Kind,
    /// Its JSON, as the writer of its kind wrote it.
    text: Box<str>,
}

/// How a node is made.
#[derive(Clone, Debug)]
struct Recipe {
    /// The operation.
    operation: &'static Operation,
    /// Its inputs, nodes made before this one.
    from: Vec<NodeId>,
    /// Its parameters, a JSON object with sorted keys; empty for every
    /// default.
    parameters: Box<str>,
    /// The settings it runs under: the keys the operation reads, as a JSON
    /// object with sorted keys.
    settings: Box<str>,
}

impl PartialEq for Recipe {
    /// Compares two recipes, their operations by name.
    fn eq(&self, other: &Self) -> bool {
        self.operation.name == other.operation.name
            && self.from == other.from
            && self.parameters == other.parameters
            && self.settings == other.settings
    }
}

impl Eq for Recipe {}

/// What a node holds.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Body {
    /// A value of the user's.
    Given(ValueId),
    /// The output of its recipe's record, where it has one.
    Derived {
        /// The recipe.
        recipe: Recipe,
        /// The record it shows.
        record: Option<RecordId>,
    },
    /// A session the recipe started, and the user's steps since.
    Work {
        /// The recipe that started it.
        recipe: Recipe,
        /// The session.
        value: ValueId,
        /// The inputs' values it was started from.
        started: Vec<ValueId>,
    },
    /// A value its recipe made once and the user owns since.
    Copy {
        /// The recipe that made it.
        recipe: Recipe,
        /// The value.
        value: ValueId,
    },
}

/// A node.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    /// Its number.
    id: NodeId,
    /// What it holds.
    body: Body,
}

impl Node {
    /// Returns its recipe, where it has one.
    const fn recipe(&self) -> Option<&Recipe> {
        match &self.body {
            Body::Given(_) => None,
            Body::Derived { recipe, .. }
            | Body::Work { recipe, .. }
            | Body::Copy { recipe, .. } => Some(recipe),
        }
    }
}

/// What a record is the run of: the cache's key.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Key {
    /// The operation's name.
    operation: &'static str,
    /// The parameters, as the recipe keeps them.
    parameters: Box<str>,
    /// The settings read, as the recipe keeps them.
    settings: Box<str>,
    /// The inputs' values.
    inputs: Vec<ValueId>,
}

/// How a run ended.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum End {
    /// It ran to its end, whatever the verdict.
    Done,
    /// A bound ended it before it decided: its output says "unknown".
    Limit(Limit),
    /// The operation refused: the library's error, as its JSON writes it.
    Refused(Box<str>),
}

/// One run of a recipe.
#[derive(Clone, Debug)]
struct Record {
    /// What it is the run of.
    key: Key,
    /// How it ended.
    end: End,
    /// Its output, where it made one.
    output: Option<ValueId>,
    /// The check of what the operation promised, where it was made.
    check: Option<Check>,
    /// Whether another build of deduxit made it.
    foreign: bool,
    /// When it was made, by the workspace's count of changes: the oldest
    /// unpinned record gives way first.
    made: u64,
}

/// A node before and after an edit; `None` where it did not exist.
#[derive(Clone, Debug)]
struct Swap {
    /// The node.
    id: NodeId,
    /// Before the edit.
    before: Option<Node>,
    /// After it.
    after: Option<Node>,
}

/// One edit: everything one action of the user's changed.
#[derive(Clone, Debug)]
struct Entry {
    /// Its name, for "Undo translation".
    name: Box<str>,
    /// What it changed, in order.
    swaps: Vec<Swap>,
    /// The records the nodes showed before it, kept while the entry is,
    /// so that an undo shows them again without a run.
    pins: Vec<RecordId>,
}

/// Whether a node shows what its recipe gives now.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// A value of the user's: given, or a copy made once.
    Given,
    /// It shows what its recipe gives on its inputs' values now.
    Current,
    /// It shows what its recipe gave on other values, settings or
    /// parameters, or a build of deduxit other than this one made it.
    OutOfDate(Cause),
    /// Its recipe has not run on any values yet.
    NotRun,
}

/// Why a node is out of date.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cause {
    /// This input holds another value than the run had.
    Input(NodeId),
    /// The node's own parameters or settings changed.
    Recipe,
    /// Another build of deduxit made what it shows.
    Version,
    /// This input is not current, or its check failed.
    Waits(NodeId),
}

/// What to compute for a node.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum What {
    /// Its recipe's output.
    Output,
    /// Its output, and the check of what its operation promised.
    Check,
}

/// A node as a caller reads it.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct NodeView<'a> {
    /// Its number.
    pub id: NodeId,
    /// Its value's kind.
    pub kind: &'static str,
    /// Its value, where it has one.
    pub value: Option<&'a str>,
    /// Its recipe's operation, where it has one.
    pub operation: Option<&'static str>,
    /// Its recipe's inputs.
    pub from: &'a [NodeId],
    /// Its recipe's parameters, a JSON object or empty.
    pub parameters: Option<&'a str>,
    /// The settings its recipe runs under, a JSON object.
    pub settings: Option<&'a str>,
    /// Its state.
    pub state: State,
    /// How the run it shows ended.
    pub end: Option<&'a End>,
    /// The check of the run it shows, where it was made.
    pub check: Option<&'a Check>,
}

/// The output or the check of a node, computed apart from the workspace,
/// which [`Workspace::accept`] puts in if the node still asks for it.
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, serde::Deserialize)]
pub struct Computed {
    /// The node.
    node: u32,
    /// The operation.
    operation: String,
    /// The parameters.
    parameters: String,
    /// The settings read.
    settings: String,
    /// The inputs' values.
    inputs: Vec<String>,
    /// The output, or the error's JSON.
    output: Result<Output, String>,
    /// The check, where it was asked for.
    check: Option<Check>,
}

/// A run's output as [`Computed`] carries it.
#[derive(Clone, Debug, Serialize, serde::Deserialize)]
struct Output {
    /// The value.
    value: String,
    /// The bound that ended it, by the library's name and the settings key.
    limit: Option<(String, Option<String>)>,
}

/// What a run did to a node.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ran {
    /// The node.
    pub node: NodeId,
    /// Its state after the run.
    pub state: State,
}

/// A workspace: nodes over immutable values, the records and the edit log.
#[derive(Clone, Debug, Default)]
pub struct Workspace {
    /// The values.
    values: Vec<Value>,
    /// Each value's place, by its kind and its JSON.
    interned: HashMap<(&'static str, Box<str>), ValueId>,
    /// The nodes alive, by their numbers in order.
    nodes: Vec<Node>,
    /// The next node's number.
    next: u32,
    /// The records; `None` where one gave way.
    records: Vec<Option<Record>>,
    /// Each record by its key.
    index: HashMap<Key, RecordId>,
    /// The edits.
    log: Vec<Entry>,
    /// How many edits are applied: undo takes back the one before.
    at: usize,
    /// The count of changes made, which orders the records.
    made: u64,
}

/// The settings every run starts from, before its recipe's keys: one
/// thread, so that a run is a function of its key.
fn base_settings() -> Settings {
    Settings::default().deterministic()
}

/// Returns the value at a dotted settings key in a JSON object.
fn at_key<'v>(value: &'v serde_json::Value, key: &str) -> Option<&'v serde_json::Value> {
    key.split('.')
        .try_fold(value, |value, part| value.get(part))
}

/// Returns the keys `reads` lists of `settings` taken on one thread, as a
/// JSON object with sorted keys.
fn project(settings: &Settings, reads: &[&str]) -> Result<Box<str>, Error> {
    let whole = serde_json::to_value(settings.clone().deterministic()).map_err(json_error)?;
    let mut picked = serde_json::Map::new();
    for key in reads {
        if let Some(value) = at_key(&whole, key) {
            picked.insert((*key).to_owned(), value.clone());
        }
    }
    Ok(serde_json::Value::Object(picked).to_string().into())
}

/// Returns the settings a recipe runs under: the base with its keys laid
/// over.
fn settings_of(projection: &str) -> Result<Settings, Error> {
    let mut whole = serde_json::to_value(base_settings()).map_err(json_error)?;
    let keys: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(projection).map_err(json_error)?;
    for (key, value) in keys {
        let mut at = &mut whole;
        let mut parts = key.split('.').peekable();
        while let Some(part) = parts.next() {
            let object = at.as_object_mut().ok_or_else(|| Error::InvalidOption {
                key: "settings",
                message: format!("{key} names no setting"),
            })?;
            if parts.peek().is_none() {
                object.insert(part.to_owned(), value);
                break;
            }
            at = object.get_mut(part).ok_or_else(|| Error::InvalidOption {
                key: "settings",
                message: format!("{key} names no setting"),
            })?;
        }
    }
    let settings: Settings =
        serde_json::from_value(whole).map_err(|error| Error::InvalidOption {
            key: "settings",
            message: error.to_string(),
        })?;
    Ok(settings.deterministic())
}

/// Returns parameters as the recipe keeps them: a JSON object with sorted
/// keys, or empty for every default.
fn canonical(parameters: &str) -> Result<Box<str>, Error> {
    if parameters.trim().is_empty() {
        return Ok("".into());
    }
    let value: serde_json::Value =
        serde_json::from_str(parameters).map_err(|error| Error::InvalidOption {
            key: "parameters",
            message: error.to_string(),
        })?;
    if !value.is_object() {
        return Err(Error::InvalidOption {
            key: "parameters",
            message: "parameters are a JSON object".to_owned(),
        });
    }
    Ok(value.to_string().into())
}

/// Returns the error of JSON the workspace wrote and cannot read back.
fn json_error(error: serde_json::Error) -> Error {
    Error::Json {
        form: "workspace",
        message: error.to_string(),
    }
}

/// Returns the refusal of a bound passed.
fn refused(what: Held, count: usize, limit: u64) -> Error {
    Error::Refused(Refusal::Workspace {
        what,
        count: u64::try_from(count).unwrap_or(u64::MAX),
        limit,
    })
}

/// Returns whether `count` passes `bound`.
fn past(count: usize, bound: Option<u64>) -> Option<u64> {
    bound.filter(|&bound| u64::try_from(count).unwrap_or(u64::MAX) > bound)
}

impl Workspace {
    /// Returns an empty workspace.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the node `id`.
    fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes
            .binary_search_by_key(&id, |node| node.id)
            .ok()
            .map(|at| &self.nodes[at])
    }

    /// Returns the record `id`.
    fn record(&self, id: RecordId) -> Option<&Record> {
        self.records.get(id.0 as usize).and_then(Option::as_ref)
    }

    /// Returns a value's text.
    fn text(&self, id: ValueId) -> &str {
        &self.values[id.0 as usize].text
    }

    /// Returns the value a node shows, where it has one.
    fn shown(&self, node: &Node) -> Option<ValueId> {
        match &node.body {
            Body::Given(value) | Body::Work { value, .. } | Body::Copy { value, .. } => {
                Some(*value)
            }
            Body::Derived { record, .. } => record
                .and_then(|record| self.record(record))
                .and_then(|r| r.output),
        }
    }

    /// Returns a node's kind.
    fn kind_of(&self, node: &Node) -> &'static str {
        match &node.body {
            Body::Given(value) => self.values[value.0 as usize].kind.name,
            Body::Derived { recipe, .. }
            | Body::Work { recipe, .. }
            | Body::Copy { recipe, .. } => recipe.operation.to,
        }
    }

    /// Returns the place of a value, interning it; refuses past the
    /// bound on bytes.
    fn intern(
        &mut self,
        kind: &'static Kind,
        text: &str,
        limits: &Limits,
    ) -> Result<ValueId, Error> {
        if let Some(&id) = self.interned.get(&(kind.name, Box::from(text))) {
            return Ok(id);
        }
        let bytes = self.bytes() + text.len();
        if let Some(limit) = past(bytes, limits.workspace.bytes) {
            return Err(refused(Held::Bytes, bytes, limit));
        }
        let id = ValueId(u32::try_from(self.values.len()).map_err(|_| {
            Error::Refused(Refusal::Index {
                what: crate::limits::Space::Node,
                count: u64::MAX,
                most: u64::from(u32::MAX),
            })
        })?);
        self.values.push(Value {
            kind,
            text: text.into(),
        });
        self.interned.insert((kind.name, text.into()), id);
        Ok(id)
    }

    /// Returns the bytes of the values held.
    fn bytes(&self) -> usize {
        self.values.iter().map(|value| value.text.len()).sum()
    }

    /// Returns the key a node's recipe asks for on its inputs' values now,
    /// or the first input without a value.
    fn key_of(&self, recipe: &Recipe) -> Result<Key, NodeId> {
        let mut inputs = Vec::with_capacity(recipe.from.len());
        for &input in &recipe.from {
            let value = self.node(input).and_then(|node| self.shown(node));
            inputs.push(value.ok_or(input)?);
        }
        Ok(Key {
            operation: recipe.operation.name,
            parameters: recipe.parameters.clone(),
            settings: recipe.settings.clone(),
            inputs,
        })
    }

    /// Returns every node's state, by the nodes' order.
    fn states(&self) -> Vec<State> {
        let mut states: Vec<State> = Vec::with_capacity(self.nodes.len());
        let state_of = |states: &Vec<State>, id: NodeId| {
            self.nodes
                .binary_search_by_key(&id, |node| node.id)
                .ok()
                .map(|at| states[at])
        };
        for node in &self.nodes {
            let state = match &node.body {
                Body::Given(_) | Body::Copy { .. } => State::Given,
                Body::Derived { recipe, record } => {
                    let waits = recipe.from.iter().copied().find(|&input| {
                        !matches!(
                            state_of(&states, input),
                            Some(State::Given | State::Current)
                        ) || self.failed(input)
                    });
                    let shown = record.and_then(|record| self.record(record));
                    match (waits, shown) {
                        (Some(_), None) | (None, None) => State::NotRun,
                        (Some(input), Some(_)) => State::OutOfDate(Cause::Waits(input)),
                        (None, Some(shown)) => match self.key_of(recipe) {
                            Ok(key) if key == shown.key && shown.foreign => {
                                State::OutOfDate(Cause::Version)
                            }
                            Ok(key) if key == shown.key => State::Current,
                            Ok(key) => State::OutOfDate(
                                key.inputs
                                    .iter()
                                    .zip(&shown.key.inputs)
                                    .zip(&recipe.from)
                                    .find(|((now, then), _)| now != then)
                                    .map_or(Cause::Recipe, |(_, &input)| Cause::Input(input)),
                            ),
                            Err(input) => State::OutOfDate(Cause::Waits(input)),
                        },
                    }
                }
                Body::Work {
                    recipe, started, ..
                } => {
                    let waits = recipe.from.iter().copied().find(|&input| {
                        !matches!(
                            state_of(&states, input),
                            Some(State::Given | State::Current)
                        )
                    });
                    match (waits, self.key_of(recipe)) {
                        (Some(input), _) | (None, Err(input)) => {
                            State::OutOfDate(Cause::Waits(input))
                        }
                        (None, Ok(key)) => key
                            .inputs
                            .iter()
                            .zip(started)
                            .zip(&recipe.from)
                            .find(|((now, then), _)| now != then)
                            .map_or(State::Current, |(_, &input)| {
                                State::OutOfDate(Cause::Input(input))
                            }),
                    }
                }
            };
            states.push(state);
        }
        states
    }

    /// Returns whether a node shows a run whose check failed.
    fn failed(&self, id: NodeId) -> bool {
        self.node(id)
            .and_then(|node| match &node.body {
                Body::Derived {
                    record: Some(record),
                    ..
                } => self.record(*record),
                _ => None,
            })
            .and_then(|record| record.check.as_ref())
            .is_some_and(|check| check.status == Status::Failed)
    }

    /// Returns a node's state.
    pub fn state(&self, node: NodeId) -> State {
        self.nodes
            .binary_search_by_key(&node, |n| n.id)
            .map_or(State::NotRun, |at| self.states()[at])
    }

    /// Returns the nodes, by their numbers in order.
    pub fn nodes(&self) -> impl Iterator<Item = NodeView<'_>> + '_ {
        let states = self.states();
        self.nodes.iter().zip(states).map(|(node, state)| {
            let recipe = node.recipe();
            let shown = match &node.body {
                Body::Derived {
                    record: Some(record),
                    ..
                } => self.record(*record),
                _ => None,
            };
            NodeView {
                id: node.id,
                kind: self.kind_of(node),
                value: self.shown(node).map(|value| self.text(value)),
                operation: recipe.map(|recipe| recipe.operation.name),
                from: recipe.map_or(&[], |recipe| &recipe.from),
                parameters: recipe.map(|recipe| &*recipe.parameters),
                settings: recipe.map(|recipe| &*recipe.settings),
                state,
                end: shown.map(|record| &record.end),
                check: shown.and_then(|record| record.check.as_ref()),
            }
        })
    }

    /// Returns the value a node shows, where it has one.
    pub fn value(&self, node: NodeId) -> Option<&str> {
        self.node(node)
            .and_then(|node| self.shown(node))
            .map(|value| self.text(value))
    }

    /// Returns each node's column for a drawing: its longest path from a
    /// node without a recipe, so that every input stands left of its node.
    pub fn layers(&self) -> Vec<(NodeId, u32)> {
        let mut columns: Vec<(NodeId, u32)> = Vec::with_capacity(self.nodes.len());
        for node in &self.nodes {
            let column = node.recipe().map_or(0, |recipe| {
                recipe
                    .from
                    .iter()
                    .filter_map(|input| {
                        columns
                            .binary_search_by_key(input, |&(id, _)| id)
                            .ok()
                            .map(|at| columns[at].1 + 1)
                    })
                    .max()
                    .unwrap_or(0)
            });
            columns.push((node.id, column));
        }
        columns
    }

    /// Returns the name of the edit that undo takes back.
    pub fn undoable(&self) -> Option<&str> {
        self.at.checked_sub(1).map(|at| &*self.log[at].name)
    }

    /// Returns the name of the edit that redo brings back.
    pub fn redoable(&self) -> Option<&str> {
        self.log.get(self.at).map(|entry| &*entry.name)
    }

    /// Puts `node` at its number, or removes the node there for `None`, and
    /// returns what was there.
    fn put(&mut self, id: NodeId, node: Option<Node>) -> Option<Node> {
        match (self.nodes.binary_search_by_key(&id, |n| n.id), node) {
            (Ok(at), Some(node)) => Some(std::mem::replace(&mut self.nodes[at], node)),
            (Ok(at), None) => Some(self.nodes.remove(at)),
            (Err(at), Some(node)) => {
                self.nodes.insert(at, node);
                None
            }
            (Err(_), None) => None,
        }
    }

    /// Makes one action of the user's: every edit `f` makes is one entry of
    /// the log, named `name`, and none is kept if `f` fails. The entries
    /// past the one applied last are dropped, so redo has nothing after a
    /// new edit; past `limits.workspace.log` the oldest is forgotten. Each
    /// node then shows a record that fits it, where one is kept.
    ///
    /// # Errors
    ///
    /// The error of `f`, after its edits are taken back.
    pub fn edit<T>(
        &mut self,
        name: &str,
        limits: &Limits,
        f: impl FnOnce(&mut Edit<'_>) -> Result<T, Error>,
    ) -> Result<T, Error> {
        self.collect(limits);
        let pins = self.pinned_now();
        let mut edit = Edit {
            workspace: self,
            limits,
            swaps: Vec::new(),
        };
        let done = f(&mut edit);
        let swaps = std::mem::take(&mut edit.swaps);
        match done {
            Ok(done) => {
                if !swaps.is_empty() {
                    self.log.truncate(self.at);
                    self.log.push(Entry {
                        name: name.into(),
                        swaps,
                        pins,
                    });
                    if let Some(bound) = limits.workspace.log {
                        let most = usize::try_from(bound).unwrap_or(usize::MAX);
                        let over = self.log.len().saturating_sub(most);
                        self.log.drain(..over);
                    }
                    self.at = self.log.len();
                    self.made += 1;
                }
                self.settle();
                Ok(done)
            }
            Err(error) => {
                for swap in swaps.into_iter().rev() {
                    self.put(swap.id, swap.before);
                }
                Err(error)
            }
        }
    }

    /// Returns the records the nodes show now.
    fn pinned_now(&self) -> Vec<RecordId> {
        self.nodes
            .iter()
            .filter_map(|node| match &node.body {
                Body::Derived { record, .. } => *record,
                _ => None,
            })
            .collect()
    }

    /// Takes back the last edit, and returns its name; `None` with nothing
    /// to take back.
    pub fn undo(&mut self) -> Option<String> {
        let at = self.at.checked_sub(1)?;
        let swaps: Vec<Swap> = self.log[at].swaps.clone();
        for swap in swaps.into_iter().rev() {
            self.put(swap.id, swap.before);
        }
        self.at = at;
        self.settle();
        Some(self.log[at].name.to_string())
    }

    /// Brings back the edit undo took back last, and returns its name;
    /// `None` with nothing to bring back.
    pub fn redo(&mut self) -> Option<String> {
        let swaps: Vec<Swap> = self.log.get(self.at)?.swaps.clone();
        for swap in swaps {
            self.put(swap.id, swap.after);
        }
        self.at += 1;
        self.settle();
        Some(self.log[self.at - 1].name.to_string())
    }

    /// Points each node that is not current at a record of its recipe on
    /// its inputs' values now, where one is kept. Runs nothing; returns
    /// the nodes it pointed elsewhere.
    pub fn settle(&mut self) -> Vec<NodeId> {
        let mut moved = Vec::new();
        for at in 0..self.nodes.len() {
            let Body::Derived { recipe, record } = &self.nodes[at].body else {
                continue;
            };
            let Ok(key) = self.key_of(recipe) else {
                continue;
            };
            let fits = |record: &Option<RecordId>| {
                record
                    .and_then(|record| self.record(record))
                    .is_some_and(|record| record.key == key && !record.foreign)
            };
            if fits(record) {
                continue;
            }
            if let Some(&found) = self.index.get(&key)
                && self.record(found).is_some_and(|record| !record.foreign)
            {
                let id = self.nodes[at].id;
                if let Body::Derived { record, .. } = &mut self.nodes[at].body {
                    *record = Some(found);
                }
                moved.push(id);
            }
        }
        moved
    }

    /// Replaces the session of a `work` node by its next state, as a step of
    /// the proof makes it; no entry of the log, since the session keeps its
    /// own undo.
    ///
    /// # Errors
    ///
    /// [`Error::NotApplicable`] for a node that holds no session, and the
    /// session's reader's error.
    pub fn work(&mut self, node: NodeId, session: &str, limits: &Limits) -> Result<(), Error> {
        let Some(at) = self.nodes.binary_search_by_key(&node, |n| n.id).ok() else {
            return Err(no_node(node));
        };
        let Body::Work { recipe, .. } = &self.nodes[at].body else {
            return Err(Error::NotApplicable {
                operation: "work".into(),
                reason: format!("node {node} holds no proof in progress"),
            });
        };
        let (_, kind) = protocol::kind(recipe.operation.to)?;
        protocol::read(kind.name, session, limits)?;
        let id = self.intern(kind, session, limits)?;
        if let Body::Work { value, .. } = &mut self.nodes[at].body {
            *value = id;
        }
        self.made += 1;
        Ok(())
    }

    /// Returns the nodes a run of `targets` runs, in order: each that is
    /// not current and has a recipe whose class is `derived`, among the
    /// targets and every node they are made from.
    pub fn plan(&self, targets: &[NodeId]) -> Vec<NodeId> {
        let mut wanted: Vec<NodeId> = targets.to_vec();
        let mut at = 0;
        while at < wanted.len() {
            if let Some(recipe) = self.node(wanted[at]).and_then(Node::recipe) {
                for &input in &recipe.from {
                    if !wanted.contains(&input) {
                        wanted.push(input);
                    }
                }
            }
            at += 1;
        }
        wanted.sort_unstable();
        let states = self.states();
        self.nodes
            .iter()
            .zip(states)
            .filter(|(node, state)| {
                wanted.binary_search(&node.id).is_ok()
                    && matches!(node.body, Body::Derived { .. })
                    && !matches!(state, State::Current)
            })
            .map(|(node, _)| node.id)
            .collect()
    }

    /// Computes a node's output, or with [`What::Check`] its output and its
    /// check, and leaves the workspace as it is. A node already current is
    /// not run again for its check: its output is the record's.
    ///
    /// # Errors
    ///
    /// [`Error::NotApplicable`] for a node with no recipe of class
    /// `derived` or an input with no value yet, and the stop's
    /// [`Refusal::Stopped`]; an operation's own refusal is the computed
    /// output, since a cache keeps it.
    pub fn compute(
        &self,
        node: NodeId,
        what: What,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Computed, Error> {
        let found = self.node(node).ok_or_else(|| no_node(node))?;
        let Body::Derived { recipe, record } = &found.body else {
            return Err(Error::NotApplicable {
                operation: "run".into(),
                reason: format!("node {node} is no output of a recipe that runs again"),
            });
        };
        let key = self.key_of(recipe).map_err(|input| Error::NotApplicable {
            operation: recipe.operation.name.into(),
            reason: format!("input {input} has no value yet"),
        })?;
        let inputs: Vec<String> = key
            .inputs
            .iter()
            .map(|&v| self.text(v).to_owned())
            .collect();
        let texts: Vec<&str> = inputs.iter().map(String::as_str).collect();
        let settings = settings_of(&recipe.settings)?;
        let current = record
            .and_then(|record| self.record(record))
            .filter(|record| record.key == key && !record.foreign);
        let output = match current {
            Some(record) => match (&record.end, record.output) {
                (End::Refused(error), _) => Err(error.to_string()),
                (end, Some(output)) => Ok(Output {
                    value: self.text(output).to_owned(),
                    limit: match end {
                        End::Limit(limit) => {
                            Some((limit.reason.to_owned(), limit.setting.map(str::to_owned)))
                        }
                        _ => None,
                    },
                }),
                (_, None) => Err(String::new()),
            },
            None => {
                match protocol::run(
                    recipe.operation.name,
                    &texts,
                    &recipe.parameters,
                    &settings,
                    &mut stop,
                ) {
                    Ok(produced) => Ok(output_of(produced)),
                    Err(error) if error.kind() == ErrorKind::Stopped => return Err(error),
                    Err(error) => Err(serde_json::to_string(&error).map_err(json_error)?),
                }
            }
        };
        let check = match (&output, what) {
            (Ok(output), What::Check) => {
                let (suite, operation) = protocol::operation(recipe.operation.name)?;
                let call = protocol::Call::new(operation, &texts, &recipe.parameters, &settings);
                let stop = std::cell::RefCell::new(&mut stop);
                let mut ask = |name: &str, inputs: &[&str], parameters: &str| {
                    self.asked(name, inputs, parameters, &settings).map_or_else(
                        || {
                            protocol::run(name, inputs, parameters, &settings, |progress| {
                                (stop.borrow_mut())(progress)
                            })
                            .map(|produced| produced.value)
                        },
                        Ok,
                    )
                };
                Some(
                    suite.check(&call, &output.value, &mut ask, &mut |progress| {
                        (stop.borrow_mut())(progress)
                    })?,
                )
            }
            _ => None,
        };
        Ok(Computed {
            node: node.0,
            operation: recipe.operation.name.to_owned(),
            parameters: recipe.parameters.to_string(),
            settings: recipe.settings.to_string(),
            inputs,
            output,
            check,
        })
    }

    /// Returns the output of a record kept for the run a check asks for.
    fn asked(
        &self,
        name: &str,
        inputs: &[&str],
        parameters: &str,
        settings: &Settings,
    ) -> Option<String> {
        let (_, operation) = protocol::operation(name).ok()?;
        let mut values = Vec::with_capacity(inputs.len());
        for (text, kinds) in inputs.iter().zip(operation.from) {
            let value = kinds
                .iter()
                .find_map(|kind| self.interned.get(&(*kind, Box::from(*text))))?;
            values.push(*value);
        }
        let key = Key {
            operation: operation.name,
            parameters: canonical(parameters).ok()?,
            settings: project(settings, operation.reads).ok()?,
            inputs: values,
        };
        let record = self.record(*self.index.get(&key)?)?;
        record.output.map(|output| self.text(output).to_owned())
    }

    /// Puts a computed output or check in, if the node's recipe on its
    /// inputs' values still asks for exactly that run; returns whether it
    /// did. The output is read by its kind's reader first, since a computed
    /// value may come from outside.
    ///
    /// # Errors
    ///
    /// The output's reader's error, and [`Refusal::Workspace`] past the
    /// bound on records or on bytes.
    pub fn accept(&mut self, computed: Computed, limits: &Limits) -> Result<bool, Error> {
        let node = NodeId(computed.node);
        let Some(Body::Derived { recipe, .. }) = self.node(node).map(|n| &n.body) else {
            return Ok(false);
        };
        let recipe = recipe.clone();
        let Ok(key) = self.key_of(&recipe) else {
            return Ok(false);
        };
        let same = computed.operation == recipe.operation.name
            && *computed.parameters == *recipe.parameters
            && *computed.settings == *recipe.settings
            && computed.inputs.len() == key.inputs.len()
            && computed
                .inputs
                .iter()
                .zip(&key.inputs)
                .all(|(text, &value)| text == self.text(value));
        if !same {
            return Ok(false);
        }
        let (end, output) = match computed.output {
            Ok(output) => {
                let (_, kind) = protocol::kind(recipe.operation.to)?;
                protocol::read(kind.name, &output.value, limits)?;
                let value = self.intern(kind, &output.value, limits)?;
                let end = match output.limit {
                    Some((reason, setting)) => End::Limit(limit_named(&reason, setting.as_deref())),
                    None => End::Done,
                };
                (end, Some(value))
            }
            Err(error) => (End::Refused(error.into()), None),
        };
        let found = match self.index.get(&key) {
            Some(&found) if self.record(found).is_some() => found,
            _ => self.insert(
                Record {
                    key: key.clone(),
                    end,
                    output,
                    check: None,
                    foreign: false,
                    made: self.made,
                },
                limits,
            )?,
        };
        if let Some(check) = computed.check
            && let Some(Some(record)) = self.records.get_mut(found.0 as usize)
        {
            record.check = Some(check);
        }
        if let Ok(at) = self.nodes.binary_search_by_key(&node, |n| n.id)
            && let Body::Derived { record, .. } = &mut self.nodes[at].body
        {
            *record = Some(found);
        }
        self.made += 1;
        self.settle();
        Ok(true)
    }

    /// Adds a record, making room past the bound by dropping the oldest
    /// that no node shows and no edit pins.
    fn insert(&mut self, record: Record, limits: &Limits) -> Result<RecordId, Error> {
        if let Some(bound) = limits.workspace.records {
            let most = usize::try_from(bound).unwrap_or(usize::MAX);
            let mut held = self.records.iter().flatten().count();
            if held >= most {
                let pinned = self.pinned();
                let mut free: Vec<(u64, usize)> = self
                    .records
                    .iter()
                    .enumerate()
                    .filter_map(|(at, record)| record.as_ref().map(|r| (r.made, at)))
                    .filter(|&(_, at)| {
                        !pinned.contains(&RecordId(u32::try_from(at).unwrap_or(u32::MAX)))
                    })
                    .collect();
                free.sort_unstable();
                for (_, at) in free {
                    if held < most {
                        break;
                    }
                    if let Some(gone) = self.records[at].take() {
                        self.index.remove(&gone.key);
                    }
                    held -= 1;
                }
                if held >= most {
                    return Err(refused(Held::Records, held + 1, bound));
                }
            }
        }
        let id = RecordId(
            u32::try_from(self.records.len())
                .map_err(|_| refused(Held::Records, self.records.len(), u64::from(u32::MAX)))?,
        );
        self.index.insert(record.key.clone(), id);
        self.records.push(Some(record));
        Ok(id)
    }

    /// Returns the records no record may give way of: those the nodes
    /// show, and those the log's entries pin or their nodes show.
    fn pinned(&self) -> std::collections::BTreeSet<RecordId> {
        let mut pinned: std::collections::BTreeSet<RecordId> =
            self.pinned_now().into_iter().collect();
        for entry in &self.log {
            pinned.extend(entry.pins.iter().copied());
            for swap in &entry.swaps {
                for node in [&swap.before, &swap.after].into_iter().flatten() {
                    if let Body::Derived {
                        record: Some(record),
                        ..
                    } = node.body
                    {
                        pinned.insert(record);
                    }
                }
            }
        }
        pinned
    }

    /// Runs the nodes [`plan`](Self::plan) lists for `targets`, each
    /// computed and accepted in order, and with [`What::Check`] makes the
    /// checks the targets' records lack. Returns each node run with its
    /// state after.
    ///
    /// # Errors
    ///
    /// As [`compute`](Self::compute) and [`accept`](Self::accept); the runs
    /// before an error stay.
    pub fn run(
        &mut self,
        targets: &[NodeId],
        what: What,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Vec<Ran>, Error> {
        self.collect(limits);
        let mut ran = Vec::new();
        for node in self.plan(targets) {
            if self.state(node) == State::Current {
                continue;
            }
            let wants = if targets.contains(&node) {
                what
            } else {
                What::Output
            };
            let computed = self.compute(node, wants, &mut stop)?;
            self.accept(computed, limits)?;
            ran.push(Ran {
                node,
                state: self.state(node),
            });
        }
        if what == What::Check {
            for &node in targets {
                let lacks = self.node(node).is_some_and(|n| match &n.body {
                    Body::Derived {
                        record: Some(record),
                        ..
                    } => self
                        .record(*record)
                        .is_some_and(|r| r.check.is_none() && r.output.is_some()),
                    _ => false,
                });
                if lacks && self.state(node) == State::Current {
                    let computed = self.compute(node, What::Check, &mut stop)?;
                    self.accept(computed, limits)?;
                    ran.push(Ran {
                        node,
                        state: self.state(node),
                    });
                }
            }
        }
        Ok(ran)
    }

    /// Gives every recipe the `limits.*` values of `limits` for the keys it
    /// reads, so that a document from elsewhere runs within the caller's
    /// bounds; the records that then fit no node stay in the cache. Returns
    /// the nodes whose recipes changed.
    pub fn rebound(&mut self, limits: &Limits) -> Vec<NodeId> {
        let Ok(bounds) = serde_json::to_value(limits) else {
            return Vec::new();
        };
        let mut changed = Vec::new();
        for node in &mut self.nodes {
            let id = node.id;
            let recipe = match &mut node.body {
                Body::Given(_) => continue,
                Body::Derived { recipe, .. }
                | Body::Work { recipe, .. }
                | Body::Copy { recipe, .. } => recipe,
            };
            let Ok(mut keys) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
                &recipe.settings,
            ) else {
                continue;
            };
            let mut touched = false;
            for (key, value) in &mut keys {
                if let Some(limit) = key.strip_prefix("limits.").and_then(|k| at_key(&bounds, k))
                    && value != limit
                {
                    *value = limit.clone();
                    touched = true;
                }
            }
            if touched {
                recipe.settings = serde_json::Value::Object(keys).to_string().into();
                changed.push(id);
            }
        }
        self.settle();
        changed
    }

    /// Drops what nothing reaches (values no node, record or edit holds,
    /// records past the bounds that no node shows and no edit pins) and
    /// numbers the rest again.
    fn collect(&mut self, limits: &Limits) {
        let over = past(self.bytes(), limits.workspace.bytes).is_some();
        let pinned = self.pinned();
        if over {
            for (at, record) in self.records.iter_mut().enumerate() {
                if !pinned.contains(&RecordId(u32::try_from(at).unwrap_or(u32::MAX)))
                    && let Some(gone) = record.take()
                {
                    self.index.remove(&gone.key);
                }
            }
        }
        // The values reached.
        let mut reached = vec![false; self.values.len()];
        let mark = |node: &Node, reached: &mut Vec<bool>| match &node.body {
            Body::Given(value) | Body::Copy { value, .. } => reached[value.0 as usize] = true,
            Body::Work { value, started, .. } => {
                reached[value.0 as usize] = true;
                for value in started {
                    reached[value.0 as usize] = true;
                }
            }
            Body::Derived { .. } => {}
        };
        for node in &self.nodes {
            mark(node, &mut reached);
        }
        for entry in &self.log {
            for swap in &entry.swaps {
                for node in [&swap.before, &swap.after].into_iter().flatten() {
                    mark(node, &mut reached);
                }
            }
        }
        for record in self.records.iter().flatten() {
            for value in record.key.inputs.iter().chain(&record.output) {
                reached[value.0 as usize] = true;
            }
        }
        if reached.iter().all(|&r| r) && !over {
            return;
        }
        let mut renumbered = vec![ValueId(u32::MAX); self.values.len()];
        let mut values = Vec::new();
        for (at, value) in std::mem::take(&mut self.values).into_iter().enumerate() {
            if reached[at] {
                renumbered[at] = ValueId(u32::try_from(values.len()).unwrap_or(u32::MAX));
                values.push(value);
            }
        }
        self.values = values;
        self.interned = self
            .values
            .iter()
            .enumerate()
            .map(|(at, value)| {
                (
                    (value.kind.name, value.text.clone()),
                    ValueId(u32::try_from(at).unwrap_or(u32::MAX)),
                )
            })
            .collect();
        let map = |value: &mut ValueId| *value = renumbered[value.0 as usize];
        let remap = |node: &mut Node| match &mut node.body {
            Body::Given(value) | Body::Copy { value, .. } => map(value),
            Body::Work { value, started, .. } => {
                map(value);
                started.iter_mut().for_each(map);
            }
            Body::Derived { .. } => {}
        };
        self.nodes.iter_mut().for_each(remap);
        for entry in &mut self.log {
            for swap in &mut entry.swaps {
                swap.before
                    .iter_mut()
                    .chain(swap.after.iter_mut())
                    .for_each(remap);
            }
        }
        self.index.clear();
        for (at, record) in self.records.iter_mut().enumerate() {
            if let Some(record) = record {
                record.key.inputs.iter_mut().for_each(map);
                record.output.iter_mut().for_each(map);
                self.index.insert(
                    record.key.clone(),
                    RecordId(u32::try_from(at).unwrap_or(u32::MAX)),
                );
            }
        }
    }
}

/// Returns a computed output of a run.
fn output_of(produced: Produced) -> Output {
    Output {
        value: produced.value,
        limit: produced
            .limit
            .map(|limit| (limit.reason.to_owned(), limit.setting.map(str::to_owned))),
    }
}

/// Returns the bound a computed output names, by the library's names: a
/// name it does not know is kept as `unknown`.
fn limit_named(reason: &str, setting: Option<&str>) -> Limit {
    let known = crate::linear::search::Reason::NAMES
        .iter()
        .chain(crate::Error::CODES)
        .find(|&&name| name == reason)
        .copied()
        .unwrap_or("unknown");
    let setting = setting.and_then(|setting| {
        [
            "limits.memory_bytes",
            "limits.occurrences",
            "limits.terms",
            "limits.derivation_bytes",
            "limits.work",
            "limits.table_atoms",
            "limits.recursion_depth",
            "search.copies",
            "search.witness_depth",
            "search.instances",
        ]
        .into_iter()
        .find(|&key| key == setting)
    });
    Limit {
        reason: known,
        setting,
    }
}

/// Returns the error of a node the workspace lacks.
fn no_node(node: NodeId) -> Error {
    Error::IndexOutOfBounds {
        space: crate::limits::Space::Node,
        index: node.0 as usize,
        len: 0,
    }
}

/// The edits of one action, which [`Workspace::edit`] makes one entry of
/// the log.
#[derive(Debug)]
pub struct Edit<'a> {
    /// The workspace.
    workspace: &'a mut Workspace,
    /// The caller's limits: the values' readers and the bounds.
    limits: &'a Limits,
    /// What the action changed so far.
    swaps: Vec<Swap>,
}

impl Edit<'_> {
    /// Puts a node, keeping what was there for the log.
    fn put(&mut self, id: NodeId, node: Option<Node>) {
        let after = node.clone();
        let before = self.workspace.put(id, node);
        self.swaps.push(Swap { id, before, after });
    }

    /// Returns a new node's number, refusing past the bound on nodes.
    fn number(&mut self) -> Result<NodeId, Error> {
        let alive = self.workspace.nodes.len() + 1;
        if let Some(limit) = past(alive, self.limits.workspace.nodes) {
            return Err(refused(Held::Nodes, alive, limit));
        }
        let id = NodeId(self.workspace.next);
        self.workspace.next = self
            .workspace
            .next
            .checked_add(1)
            .ok_or_else(|| refused(Held::Nodes, usize::MAX, u64::from(u32::MAX)))?;
        Ok(id)
    }

    /// Returns a node, or the error that names it missing.
    fn existing(&self, id: NodeId) -> Result<Node, Error> {
        self.workspace.node(id).cloned().ok_or_else(|| no_node(id))
    }

    /// Adds a node that holds a value of the kind named `kind`, and returns
    /// its number.
    ///
    /// # Errors
    ///
    /// An unknown kind, the kind's reader's error, and the bounds' refusals.
    pub fn add(&mut self, kind: &str, value: &str) -> Result<NodeId, Error> {
        let (_, kind) = protocol::kind(kind)?;
        protocol::read(kind.name, value, self.limits)?;
        let value = self.workspace.intern(kind, value, self.limits)?;
        let id = self.number()?;
        self.put(
            id,
            Some(Node {
                id,
                body: Body::Given(value),
            }),
        );
        Ok(id)
    }

    /// Gives a node a value of its kind: a given node or a copy.
    ///
    /// # Errors
    ///
    /// [`Error::NotGiven`] for a node a recipe makes, and the kind's
    /// reader's error.
    pub fn set(&mut self, node: NodeId, value: &str) -> Result<(), Error> {
        let mut changed = self.existing(node)?;
        if matches!(changed.body, Body::Derived { .. } | Body::Work { .. }) {
            return Err(Error::NotGiven { node: node.0 });
        }
        let kind = self.workspace.kind_of(&changed);
        let (_, kind) = protocol::kind(kind)?;
        protocol::read(kind.name, value, self.limits)?;
        let id = self.workspace.intern(kind, value, self.limits)?;
        match &mut changed.body {
            Body::Given(value) | Body::Copy { value, .. } => *value = id,
            Body::Derived { .. } | Body::Work { .. } => {
                return Err(Error::NotGiven { node: node.0 });
            }
        }
        self.put(node, Some(changed));
        Ok(())
    }

    /// Adds a node made by `operation` from the nodes `from`, with its
    /// parameters (a JSON object, `""` for every default), running under
    /// the keys of `settings` the operation reads, and returns its number.
    /// A recipe of class `work` or `copy` runs at once, since its node
    /// holds what it made; one of class `derived` runs when asked.
    ///
    /// # Errors
    ///
    /// An unknown operation, [`Error::NotApplicable`] for inputs of other
    /// kinds or without a value, bad parameters, a run's refusal, and the
    /// bounds' refusals.
    pub fn connect(
        &mut self,
        operation: &str,
        from: &[NodeId],
        parameters: &str,
        settings: &Settings,
    ) -> Result<NodeId, Error> {
        let (_, operation) = protocol::operation(operation)?;
        if from.len() != operation.from.len() {
            return Err(Error::NotApplicable {
                operation: operation.name.into(),
                reason: format!(
                    "it takes {} inputs, not {}",
                    operation.from.len(),
                    from.len()
                ),
            });
        }
        for (&input, kinds) in from.iter().zip(operation.from) {
            let node = self.existing(input)?;
            let kind = self.workspace.kind_of(&node);
            if !kinds.contains(&kind) {
                return Err(Error::NotApplicable {
                    operation: operation.name.into(),
                    reason: format!(
                        "node {input} is {kind}, and it takes {}",
                        kinds.join(" or ")
                    ),
                });
            }
        }
        let recipe = Recipe {
            operation,
            from: from.to_vec(),
            parameters: canonical(parameters)?,
            settings: project(settings, operation.reads)?,
        };
        let body = match operation.class {
            Class::Derived => Body::Derived {
                recipe,
                record: None,
            },
            Class::Work | Class::Copy => {
                let (value, started) = self.made_now(&recipe)?;
                if operation.class == Class::Work {
                    Body::Work {
                        recipe,
                        value,
                        started,
                    }
                } else {
                    Body::Copy { recipe, value }
                }
            }
        };
        let id = self.number()?;
        self.put(id, Some(Node { id, body }));
        Ok(id)
    }

    /// Runs a recipe at once on its inputs' values, for a node that holds
    /// what it made; returns the value and the inputs' values.
    fn made_now(&mut self, recipe: &Recipe) -> Result<(ValueId, Vec<ValueId>), Error> {
        let key = self
            .workspace
            .key_of(recipe)
            .map_err(|input| Error::NotApplicable {
                operation: recipe.operation.name.into(),
                reason: format!("input {input} has no value yet"),
            })?;
        let texts: Vec<&str> = key.inputs.iter().map(|&v| self.workspace.text(v)).collect();
        let settings = settings_of(&recipe.settings)?;
        let produced = protocol::run(
            recipe.operation.name,
            &texts,
            &recipe.parameters,
            &settings,
            |_| false,
        )?;
        let (_, kind) = protocol::kind(recipe.operation.to)?;
        let value = self.workspace.intern(kind, &produced.value, self.limits)?;
        Ok((value, key.inputs))
    }

    /// Changes a node's recipe's parameters.
    ///
    /// # Errors
    ///
    /// [`Error::NotApplicable`] for a node without a recipe, and bad
    /// parameters.
    pub fn set_parameters(&mut self, node: NodeId, parameters: &str) -> Result<(), Error> {
        let parameters = canonical(parameters)?;
        self.change_recipe(node, |recipe| recipe.parameters = parameters)
    }

    /// Changes the settings a node's recipe runs under to the keys of
    /// `settings` its operation reads.
    ///
    /// # Errors
    ///
    /// [`Error::NotApplicable`] for a node without a recipe.
    pub fn set_settings(&mut self, node: NodeId, settings: &Settings) -> Result<(), Error> {
        let reads = self
            .existing(node)?
            .recipe()
            .map_or(&[][..], |recipe| recipe.operation.reads);
        let settings = project(settings, reads)?;
        self.change_recipe(node, |recipe| recipe.settings = settings)
    }

    /// Changes a node's recipe by `change`.
    fn change_recipe(
        &mut self,
        node: NodeId,
        change: impl FnOnce(&mut Recipe),
    ) -> Result<(), Error> {
        let mut changed = self.existing(node)?;
        match &mut changed.body {
            Body::Given(_) => {
                return Err(Error::NotApplicable {
                    operation: "recipe".into(),
                    reason: format!("node {node} has no recipe"),
                });
            }
            Body::Derived { recipe, .. }
            | Body::Work { recipe, .. }
            | Body::Copy { recipe, .. } => {
                change(recipe);
            }
        }
        self.put(node, Some(changed));
        Ok(())
    }

    /// Starts a `work` node's session again from its inputs' values now.
    ///
    /// # Errors
    ///
    /// [`Error::NotApplicable`] for a node that holds no session, and the
    /// start's refusal.
    pub fn restart(&mut self, node: NodeId) -> Result<(), Error> {
        let mut changed = self.existing(node)?;
        let Body::Work {
            recipe,
            value,
            started,
        } = &mut changed.body
        else {
            return Err(Error::NotApplicable {
                operation: "restart".into(),
                reason: format!("node {node} holds no proof in progress"),
            });
        };
        let (made, inputs) = self.made_now(recipe)?;
        *value = made;
        *started = inputs;
        self.put(node, Some(changed));
        Ok(())
    }

    /// Replaces a `work` node's session within the action.
    ///
    /// # Errors
    ///
    /// As [`Workspace::work`].
    pub fn work(&mut self, node: NodeId, session: &str) -> Result<(), Error> {
        let mut changed = self.existing(node)?;
        let Body::Work { recipe, value, .. } = &mut changed.body else {
            return Err(Error::NotApplicable {
                operation: "work".into(),
                reason: format!("node {node} holds no proof in progress"),
            });
        };
        let (_, kind) = protocol::kind(recipe.operation.to)?;
        protocol::read(kind.name, session, self.limits)?;
        *value = self.workspace.intern(kind, session, self.limits)?;
        self.put(node, Some(changed));
        Ok(())
    }

    /// Makes a node given: it keeps the value it shows and loses its
    /// recipe.
    ///
    /// # Errors
    ///
    /// [`Error::NotApplicable`] for a node that shows no value.
    pub fn detach(&mut self, node: NodeId) -> Result<(), Error> {
        let found = self.existing(node)?;
        let value = self
            .workspace
            .shown(&found)
            .ok_or_else(|| Error::NotApplicable {
                operation: "detach".into(),
                reason: format!("node {node} shows no value yet"),
            })?;
        self.put(
            node,
            Some(Node {
                id: node,
                body: Body::Given(value),
            }),
        );
        Ok(())
    }

    /// Removes a node and every node made from it, and returns them.
    ///
    /// # Errors
    ///
    /// The error of a node the workspace lacks.
    pub fn remove(&mut self, node: NodeId) -> Result<Vec<NodeId>, Error> {
        self.existing(node)?;
        let mut gone = vec![node];
        for found in &self.workspace.nodes {
            if found
                .recipe()
                .is_some_and(|recipe| recipe.from.iter().any(|input| gone.contains(input)))
            {
                gone.push(found.id);
            }
        }
        for &id in gone.iter().rev() {
            self.put(id, None);
        }
        Ok(gone)
    }
}
