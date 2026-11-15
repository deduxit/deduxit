// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A calculus read from a rule file: its signature, the shape of its
//! sequents and its rules, each compiled into the plan the checker
//! follows.

use super::FORMULA;
use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Limits, Refusal};
use crate::serialize::rules::{CalculusForm, DISCHARGES, PatternForm, RuleForm, SequentForm};

/// The most arguments a connective takes. A rule file says the number,
/// and the calculus holds a sort for each.
const ARITY: u32 = 255;

/// What the members of a zone are.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Discipline {
    /// A set: neither how often a member occurs nor where counts.
    Set,
    /// A multiset: how often a member occurs counts, where does not.
    Multiset,
    /// A sequence: both count.
    Sequence,
}

/// Where a connective is printed among its arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Fixity {
    /// Before its one argument: `¬A`.
    Prefix,
    /// Between its two arguments: `A ∧ B`.
    Infix,
    /// After its one argument.
    Postfix,
    /// Before its arguments in brackets: `f(A, B)`.
    Function,
}

/// How a chain of an infix connective is read without brackets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Associativity {
    /// `A ∘ B ∘ C` is `(A ∘ B) ∘ C`.
    Left,
    /// `A ∘ B ∘ C` is `A ∘ (B ∘ C)`.
    Right,
    /// A chain needs its brackets.
    None,
}

/// A connective of the signature.
#[derive(Clone, Debug)]
pub(crate) struct Connective {
    /// The sorts of its arguments.
    pub(crate) arguments: Vec<u32>,
    /// The sort of the term it makes.
    pub(crate) result: u32,
}

/// A zone of the calculus's sequents.
#[derive(Clone, Debug)]
pub(crate) struct Zone {
    /// Its name.
    pub(crate) name: String,
    /// The sort of its members.
    pub(crate) sort: u32,
    /// Its discipline.
    pub(crate) discipline: Discipline,
    /// The fewest members it holds.
    pub(crate) least: u32,
    /// The most members it holds, if bounded.
    pub(crate) most: Option<u32>,
}

/// A node of a rule's patterns: a variable, or a connective applied to
/// patterns before it.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Pattern {
    /// The schematic variable at this index.
    Variable(u32),
    /// A connective and where its arguments lie among the rule's
    /// arguments.
    Apply {
        /// The connective.
        connective: u32,
        /// The index of its first argument.
        start: u32,
        /// How many arguments it has.
        len: u32,
    },
}

/// How the checker finds a member a rule writes out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Find {
    /// At this place from the zone's start.
    Start(u32),
    /// At this place from the zone's end, the last member `0`.
    End(u32),
    /// By its value: every variable of it is bound by then.
    Value,
    /// At the next position the node gives.
    Position,
}

/// A member a rule writes out: the range of its pattern's nodes, the last
/// its root, and how it is found.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Item {
    /// The pattern's first node.
    pub(crate) first: u32,
    /// The pattern's root.
    pub(crate) root: u32,
    /// How the member is found.
    pub(crate) find: Find,
}

/// An entry of a zone of a schematic sequent.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Entry {
    /// A member written out.
    Item(Item),
    /// A context, under a connective or as it is.
    Context {
        /// The context variable.
        variable: u32,
        /// The unary connective applied to each of its members.
        under: Option<u32>,
    },
}

/// A schematic sequent: the entries of each zone of the calculus.
#[derive(Clone, Debug, Default)]
pub(crate) struct Shape {
    /// Per zone, its entries in the order written.
    pub(crate) zones: Vec<Vec<Entry>>,
}

impl Shape {
    /// Returns the contexts of the segment `segment` of `zone`: all of a
    /// zone that is no sequence, and in a sequence those between two
    /// members written out.
    pub(crate) fn segment(
        &self,
        zone: usize,
        segment: u32,
        sequence: bool,
    ) -> impl Iterator<Item = (u32, Option<u32>)> + '_ {
        let mut at = 0;
        self.zones[zone]
            .iter()
            .filter_map(move |entry| match *entry {
                Entry::Item(_) => {
                    at += u32::from(sequence);
                    None
                }
                Entry::Context { variable, under } => (at == segment).then_some((variable, under)),
            })
    }
}

/// A premise of a rule: its sequent and the hypotheses it discharges.
#[derive(Clone, Debug)]
pub(crate) struct Premise {
    /// The sequent.
    pub(crate) shape: Shape,
    /// Each hypothesis discharged: per zone, the patterns of its
    /// members, each a range of nodes.
    pub(crate) discharges: Vec<Vec<Vec<(u32, u32)>>>,
}

/// One equation between what a zone holds beyond its written members and
/// its contexts, in the order the checker solves them.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Equation {
    /// The sequent: `0` the conclusion, then the premises.
    pub(crate) sequent: u32,
    /// The zone.
    pub(crate) zone: u32,
    /// The segment of the zone.
    pub(crate) segment: u32,
    /// The place in the segment of the context the equation defines;
    /// `None` where all are bound and the equation is a comparison.
    pub(crate) define: Option<u32>,
}

/// A rule, compiled.
#[derive(Clone, Debug)]
pub(crate) struct Rule {
    /// The nodes of its patterns, arguments before the node they belong
    /// to, each written member's nodes together.
    pub(crate) patterns: Vec<Pattern>,
    /// The arguments of its patterns' nodes, as indices of nodes.
    pub(crate) arguments: Vec<u32>,
    /// The conclusion.
    pub(crate) conclusion: Shape,
    /// The premises.
    pub(crate) premises: Vec<Premise>,
    /// The variables matched by atoms new to the conclusion.
    pub(crate) fresh: Vec<u32>,
    /// The variables matched by atoms.
    pub(crate) atomic: Vec<u32>,
    /// The equations of its multiset and sequence zones, in the order
    /// that defines every context before it is used.
    pub(crate) equations: Vec<Equation>,
    /// How many positions a node of the rule gives.
    pub(crate) positions: u32,
}

impl Rule {
    /// Returns the sequent at `index`: `0` the conclusion, then the
    /// premises.
    pub(crate) fn shape(&self, index: usize) -> &Shape {
        match index.checked_sub(1) {
            None => &self.conclusion,
            Some(premise) => &self.premises[premise].shape,
        }
    }

    /// Returns the arguments of a pattern's node.
    pub(crate) fn arguments(&self, start: u32, len: u32) -> &[u32] {
        &self.arguments[start as usize..][..len as usize]
    }
}

/// A calculus: sorts, connectives, the shape of a sequent, and rules as
/// premises and a conclusion over schematic formulas and contexts.
///
/// A calculus is read from a rule file, a JSON document
/// ([`wire`](crate::wire), level 15), and checked as it is read: every
/// name it uses is declared, every pattern is well sorted, and every rule
/// can be matched without a search. [`LK`](super::LK), [`LJ`](super::LJ)
/// and [`LM`](super::LM) are rule files of this crate.
///
/// # JSON
///
/// ```json
/// {"version": 15, "name": "LK",
///  "connectives": [{"name": "∧", "arity": 2}],
///  "sequent": [{"name": "left", "discipline": "multiset"},
///              {"name": "right", "discipline": "multiset", "mark": "⊢"}],
///  "variables": {"formula": ["A", "B"]},
///  "contexts": ["Γ", "Δ"],
///  "rules": [{"name": "∧R",
///             "premises": [{"left": ["Γ"], "right": ["A", "Δ"]},
///                          {"left": ["Γ"], "right": ["B", "Δ"]}],
///             "conclusion": {"left": ["Γ"], "right": [["∧", "A", "B"], "Δ"]}}]}
/// ```
///
/// - `sorts` names the sorts beside `formula`. A connective's `sorts`
///   are those of its arguments and then of the term it makes; without
///   the key all are `formula`. `symbol`, `fixity`, `precedence` and
///   `associativity` say how it is printed.
/// - `sequent` lists the zones in the order written. A zone has a
///   `discipline` (`set`, `multiset` or `sequence`), the `sort` of its
///   members, a `mark` written before it, and bounds `least` and `most`
///   on its size.
/// - `variables` declares the schematic variables of each sort,
///   `contexts` those that stand for the rest of a zone.
/// - A rule's sequents are objects keyed by zone; an absent zone is
///   empty. A pattern is a variable, a connective without arguments, or
///   an array of a connective's name and its arguments. `["!", "Γ"]` in a
///   zone is the context `Γ` with `!` on every member.
/// - A premise's `discharges` lists the hypotheses it discharges, each a
///   sequent. A rule's `fresh` names variables matched by atoms that
///   occur neither in the conclusion nor in a hypothesis in force, and
///   `atomic` variables matched by atoms.
///
/// An unknown key is refused: a rule file is written by hand, and a
/// misspelt `premises` would read as a rule without premises.
#[derive(Clone, Debug)]
pub struct Calculus {
    /// The rule file.
    form: CalculusForm,
    /// The names of the sorts, `formula` first.
    pub(crate) sorts: Vec<String>,
    /// The connectives, in the file's order.
    pub(crate) connectives: Vec<Connective>,
    /// The index of each connective's name.
    pub(crate) connective_ids: HashMap<String, u32>,
    /// The zones.
    pub(crate) zones: Vec<Zone>,
    /// How many schematic variables are declared.
    pub(crate) variables: u32,
    /// The discipline of each context variable; `None` for one no rule
    /// uses.
    pub(crate) contexts: Vec<Option<Discipline>>,
    /// The rules, in the file's order.
    pub(crate) rules: Vec<Rule>,
    /// The index of each rule's name.
    pub(crate) rule_ids: HashMap<String, u32>,
}

/// What a name of a rule file stands for.
#[derive(Clone, Copy, Debug)]
enum Name {
    /// The schematic variable at this index, of this sort.
    Variable(u32, u32),
    /// The context variable at this index.
    Context(u32),
    /// The connective at this index.
    Connective(u32),
}

/// Declares `name` as `what`, refusing an empty name and one declared
/// before.
fn declare<'a>(names: &mut HashMap<&'a str, Name>, name: &'a str, what: Name) -> Result<(), Error> {
    if name.is_empty() {
        return Err(invalid("a name is empty".to_owned()));
    }
    match names.insert(name, what) {
        None => Ok(()),
        Some(_) => Err(invalid(format!("the name {name:?} is declared twice"))),
    }
}

/// Returns the error for a rule file that is no calculus.
fn invalid(reason: String) -> Error {
    Error::InvalidCalculus { reason }
}

impl Calculus {
    /// Returns the calculus's name.
    pub fn name(&self) -> &str {
        &self.form.name
    }

    /// Returns what the rule file says the calculus is.
    pub fn description(&self) -> &str {
        &self.form.description
    }

    /// Returns the names of the zones of a sequent, in the order written.
    pub fn zones(&self) -> impl ExactSizeIterator<Item = &str> {
        self.zones.iter().map(|zone| &*zone.name)
    }

    /// Returns the names of the rules, in the file's order.
    pub fn rules(&self) -> impl ExactSizeIterator<Item = &str> {
        self.form.rules.iter().map(|rule| &*rule.name)
    }

    /// Returns how many positions a node of the rule named `rule` gives:
    /// one for each member the rule writes out that the checker cannot
    /// find otherwise, the conclusion's first, then each premise's.
    /// `None` for a name that is no rule's.
    pub fn positions(&self, rule: &str) -> Option<usize> {
        let index = *self.rule_ids.get(rule)?;
        Some(self.rules[index as usize].positions as usize)
    }

    /// Returns the rule file the calculus was read from.
    pub(crate) const fn form(&self) -> &CalculusForm {
        &self.form
    }

    /// Returns the calculus a rule file holds, its patterns within
    /// `limits.occurrences` nodes.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidCalculus`] for a file that is no calculus, naming
    /// what is wrong; [`Refusal::Occurrences`] past the bound.
    pub(crate) fn from_form(form: CalculusForm, limits: &Limits) -> Result<Self, Error> {
        let mut names: HashMap<&str, Name> = HashMap::default();
        if form.name.is_empty() {
            return Err(invalid("the calculus has no name".to_owned()));
        }
        // Sorts.
        let mut sorts = vec![FORMULA.to_owned()];
        for sort in &form.sorts {
            if sorts.contains(sort) || sort.is_empty() {
                return Err(invalid(format!("the sort {sort:?} is declared twice")));
            }
            sorts.push(sort.clone());
        }
        let sort_of = |name: &str| {
            sorts
                .iter()
                .position(|sort| sort == name)
                .map(|index| index as u32)
                .ok_or_else(|| invalid(format!("no sort is named {name:?}")))
        };
        // Connectives.
        let mut connectives = Vec::with_capacity(form.connectives.len());
        for (index, connective) in form.connectives.iter().enumerate() {
            if connective.arity > ARITY {
                return Err(invalid(format!(
                    "the connective {:?} takes {} arguments; a connective takes {ARITY} at most",
                    connective.name, connective.arity
                )));
            }
            let arity = connective.arity as usize;
            let (arguments, result) = match &connective.sorts {
                None => (vec![0; arity], 0),
                Some(list) if list.len() == arity + 1 => {
                    let mut list = list
                        .iter()
                        .map(|name| sort_of(name))
                        .collect::<Result<Vec<_>, _>>()?;
                    let result = list.pop().unwrap_or(0);
                    (list, result)
                }
                Some(list) => {
                    return Err(invalid(format!(
                        "the connective {:?} takes {arity} arguments and names {} sorts: \
                         it needs one for each argument and one for the term it makes",
                        connective.name,
                        list.len()
                    )));
                }
            };
            declare(&mut names, &connective.name, Name::Connective(index as u32))?;
            connectives.push(Connective { arguments, result });
        }
        // Zones.
        let mut zones = Vec::with_capacity(form.sequent.len());
        for zone in &form.sequent {
            if zone.name == DISCHARGES || zones.iter().any(|z: &Zone| z.name == zone.name) {
                return Err(invalid(format!(
                    "the zone name {:?} is taken: by another zone, or as a premise's key",
                    zone.name
                )));
            }
            let least = zone.least.unwrap_or(0);
            if zone.most.is_some_and(|most| most < least) {
                return Err(invalid(format!(
                    "the zone {:?} holds at least {least} members and at most fewer",
                    zone.name
                )));
            }
            zones.push(Zone {
                name: zone.name.clone(),
                sort: sort_of(zone.sort.as_deref().unwrap_or(FORMULA))?,
                discipline: zone.discipline,
                least,
                most: zone.most,
            });
        }
        if zones.is_empty() {
            return Err(invalid("a sequent has at least one zone".to_owned()));
        }
        // Variables and contexts.
        let mut variables = 0;
        for (sort, list) in &form.variables {
            let sort = sort_of(sort)?;
            for name in list {
                declare(&mut names, name, Name::Variable(variables, sort))?;
                variables += 1;
            }
        }
        for (index, name) in form.contexts.iter().enumerate() {
            declare(&mut names, name, Name::Context(index as u32))?;
        }
        // Rules.
        let mut compiler = Compiler {
            names: &names,
            connectives: &connectives,
            zones: &zones,
            contexts: vec![None; form.contexts.len()],
            budget: limits.occurrences,
            nodes: 0,
        };
        let mut rules = Vec::with_capacity(form.rules.len());
        let mut rule_ids = HashMap::default();
        for (index, rule) in form.rules.iter().enumerate() {
            if rule.name.is_empty() || rule_ids.insert(rule.name.clone(), index as u32).is_some() {
                return Err(invalid(format!(
                    "the rule name {:?} is empty or used twice",
                    rule.name
                )));
            }
            let compiled = compiler
                .rule(rule, variables)
                .map_err(|error| match error {
                    Error::InvalidCalculus { reason } => {
                        invalid(format!("rule {:?}: {reason}", rule.name))
                    }
                    other => other,
                })?;
            rules.push(compiled);
        }
        let contexts = compiler
            .contexts
            .iter()
            .map(|c| c.map(|(d, _)| d))
            .collect();
        let connective_ids = form
            .connectives
            .iter()
            .enumerate()
            .map(|(index, connective)| (connective.name.clone(), index as u32))
            .collect();
        Ok(Self {
            sorts,
            connectives,
            connective_ids,
            zones,
            variables,
            contexts,
            rules,
            rule_ids,
            form,
        })
    }
}

/// Compiles the rules of a file against its declarations.
struct Compiler<'a> {
    /// What each declared name stands for.
    names: &'a HashMap<&'a str, Name>,
    /// The connectives.
    connectives: &'a [Connective],
    /// The zones.
    zones: &'a [Zone],
    /// Per context variable, the discipline and the sort of its members,
    /// once a rule has used it.
    contexts: Vec<Option<(Discipline, u32)>>,
    /// The most pattern nodes the file may hold.
    budget: Option<u64>,
    /// The pattern nodes made so far.
    nodes: u64,
}

/// A rule's patterns as they are built.
#[derive(Default)]
struct Patterns {
    /// The nodes.
    nodes: Vec<Pattern>,
    /// The nodes' arguments.
    arguments: Vec<u32>,
}

impl Compiler<'_> {
    /// Returns what `name` stands for.
    fn name(&self, name: &str) -> Result<Name, Error> {
        self.names.get(name).copied().ok_or_else(|| {
            invalid(format!(
                "{name:?} is neither a declared variable or context nor a connective"
            ))
        })
    }

    /// Adds a term pattern to `patterns` and returns its root and its
    /// sort. A pattern is nested no deeper than the reader allows, so the
    /// recursion is bounded.
    fn pattern(
        &mut self,
        form: &PatternForm,
        patterns: &mut Patterns,
    ) -> Result<(u32, u32), Error> {
        self.nodes += 1;
        if let Some(limit) = self.budget
            && self.nodes > limit
        {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: self.nodes,
                limit,
            }));
        }
        let (name, arguments) = match form {
            PatternForm::Name(name) => (name, &[][..]),
            PatternForm::Apply(name, arguments) => (name, &arguments[..]),
        };
        let (node, sort) = match self.name(name)? {
            Name::Variable(variable, sort) if arguments.is_empty() => {
                (Pattern::Variable(variable), sort)
            }
            Name::Variable(..) => {
                return Err(invalid(format!("the variable {name:?} takes no arguments")));
            }
            Name::Context(_) => {
                return Err(invalid(format!(
                    "the context {name:?} stands where a term does: a context is an entry \
                     of a zone, alone or under one unary connective"
                )));
            }
            Name::Connective(connective) => {
                let signature = &self.connectives[connective as usize];
                if signature.arguments.len() != arguments.len() {
                    return Err(invalid(format!(
                        "{name:?} takes {} arguments, not {}",
                        signature.arguments.len(),
                        arguments.len()
                    )));
                }
                let mut roots = Vec::with_capacity(arguments.len());
                for (argument, &expected) in arguments.iter().zip(&signature.arguments) {
                    let (root, sort) = self.pattern(argument, patterns)?;
                    if sort != expected {
                        return Err(invalid(format!(
                            "an argument of {name:?} has another sort than the connective takes"
                        )));
                    }
                    roots.push(root);
                }
                let start = patterns.arguments.len() as u32;
                patterns.arguments.extend(roots);
                let node = Pattern::Apply {
                    connective,
                    start,
                    len: arguments.len() as u32,
                };
                (node, signature.result)
            }
        };
        patterns.nodes.push(node);
        Ok((patterns.nodes.len() as u32 - 1, sort))
    }

    /// Returns the zone named `name` and its index.
    fn zone(&self, name: &str) -> Result<usize, Error> {
        self.zones
            .iter()
            .position(|zone| zone.name == name)
            .ok_or_else(|| invalid(format!("no zone is named {name:?}")))
    }

    /// Compiles a schematic sequent. Its written members get
    /// [`Find::Position`]; [`Self::rule`] decides how each is found.
    fn shape(
        &mut self,
        form: &SequentForm,
        patterns: &mut Patterns,
        contexts: bool,
    ) -> Result<Shape, Error> {
        let mut shape = Shape {
            zones: vec![Vec::new(); self.zones.len()],
        };
        for (name, entries) in form {
            let zone = self.zone(name)?;
            let (sort, discipline) = (self.zones[zone].sort, self.zones[zone].discipline);
            for entry in entries {
                // A context, alone or under a unary connective.
                let context = match entry {
                    PatternForm::Name(name) => match self.name(name)? {
                        Name::Context(variable) => Some((variable, None, sort)),
                        Name::Variable(..) | Name::Connective(_) => None,
                    },
                    PatternForm::Apply(head, arguments) => match arguments.as_slice() {
                        [PatternForm::Name(inner)] => match (self.name(head)?, self.name(inner)?) {
                            (Name::Connective(connective), Name::Context(variable)) => {
                                let signature = &self.connectives[connective as usize];
                                if signature.result != sort {
                                    return Err(invalid(format!(
                                        "{head:?} over the context {inner:?} makes no member \
                                         of the zone {name:?}"
                                    )));
                                }
                                Some((variable, Some(connective), signature.arguments[0]))
                            }
                            _ => None,
                        },
                        _ => None,
                    },
                };
                let Some((variable, under, members)) = context else {
                    let first = patterns.nodes.len() as u32;
                    let (root, item) = self.pattern(entry, patterns)?;
                    if item != sort {
                        return Err(invalid(format!(
                            "an entry of the zone {name:?} has another sort than the zone"
                        )));
                    }
                    shape.zones[zone].push(Entry::Item(Item {
                        first,
                        root,
                        find: Find::Position,
                    }));
                    continue;
                };
                if !contexts {
                    return Err(invalid(
                        "a discharged hypothesis writes out its members: it has no context"
                            .to_owned(),
                    ));
                }
                let twice = shape.zones[zone]
                    .iter()
                    .any(|e| matches!(e, Entry::Context { variable: v, .. } if *v == variable));
                if twice {
                    return Err(invalid(format!(
                        "a context stands twice in the zone {name:?}"
                    )));
                }
                match self.contexts[variable as usize] {
                    None => self.contexts[variable as usize] = Some((discipline, members)),
                    Some(known) if known == (discipline, members) => {}
                    Some(_) => {
                        return Err(invalid(format!(
                            "a context of the zone {name:?} is used elsewhere with another \
                             discipline or sort"
                        )));
                    }
                }
                shape.zones[zone].push(Entry::Context { variable, under });
            }
        }
        Ok(shape)
    }

    /// Compiles a rule: its patterns, how each written member is found,
    /// and the order in which its contexts are solved.
    fn rule(&mut self, form: &RuleForm, variables: u32) -> Result<Rule, Error> {
        let mut patterns = Patterns::default();
        let conclusion = self.shape(&form.conclusion, &mut patterns, true)?;
        let mut premises = Vec::with_capacity(form.premises.len());
        for premise in &form.premises {
            let shape = self.shape(&premise.sequent, &mut patterns, true)?;
            let mut discharges = Vec::with_capacity(premise.discharges.len());
            for hypothesis in &premise.discharges {
                let shape = self.shape(hypothesis, &mut patterns, false)?;
                let ranges = |zone: &Vec<Entry>| {
                    zone.iter()
                        .filter_map(|entry| match entry {
                            Entry::Item(item) => Some((item.first, item.root)),
                            Entry::Context { .. } => None,
                        })
                        .collect()
                };
                discharges.push(shape.zones.iter().map(ranges).collect());
            }
            premises.push(Premise { shape, discharges });
        }
        let mut rule = Rule {
            patterns: patterns.nodes,
            arguments: patterns.arguments,
            conclusion,
            premises,
            fresh: Vec::new(),
            atomic: Vec::new(),
            equations: Vec::new(),
            positions: 0,
        };
        // How each written member is found, in the order the checker
        // meets them.
        let mut bound = vec![false; variables as usize];
        for index in 0..=rule.premises.len() {
            for zone in 0..self.zones.len() {
                let sequence = self.zones[zone].discipline == Discipline::Sequence;
                let entries = rule.shape(index).zones[zone].clone();
                let contexts = entries
                    .iter()
                    .filter(|e| matches!(e, Entry::Context { .. }))
                    .count();
                let mut found = Vec::with_capacity(entries.len());
                let mut seen_context = false;
                let mut place = 0;
                for (at, entry) in entries.iter().enumerate() {
                    let Entry::Item(item) = entry else {
                        seen_context = true;
                        continue;
                    };
                    let range = item.first as usize..=item.root as usize;
                    let unbound = rule.patterns[range.clone()]
                        .iter()
                        .any(|node| matches!(node, Pattern::Variable(v) if !bound[*v as usize]));
                    let find = if contexts <= 1 && (sequence || entries.len() == 1) {
                        if seen_context {
                            Find::End((entries.len() - 1 - at) as u32)
                        } else {
                            Find::Start(place)
                        }
                    } else if sequence || unbound || index == 0 {
                        // A node says which members of its conclusion
                        // its rule acts on, even where their value
                        // would find them.
                        rule.positions += 1;
                        Find::Position
                    } else {
                        Find::Value
                    };
                    place += 1;
                    for node in &rule.patterns[range] {
                        if let Pattern::Variable(variable) = node {
                            bound[*variable as usize] = true;
                        }
                    }
                    found.push((at, find));
                }
                let shape = match index.checked_sub(1) {
                    None => &mut rule.conclusion,
                    Some(premise) => &mut rule.premises[premise].shape,
                };
                for (at, find) in found {
                    if let Entry::Item(item) = &mut shape.zones[zone][at] {
                        item.find = find;
                    }
                }
            }
        }
        // The conditions and the discharged hypotheses name bound
        // variables only.
        let unbound = |what: &str| {
            invalid(format!(
                "a variable of {what} occurs in no sequent of the rule"
            ))
        };
        for (list, into) in [(&form.fresh, 0), (&form.atomic, 1)] {
            for name in list {
                let Name::Variable(variable, _) = self.name(name)? else {
                    return Err(invalid(format!("{name:?} is no schematic variable")));
                };
                if !bound[variable as usize] {
                    return Err(unbound("a condition"));
                }
                if into == 0 {
                    rule.fresh.push(variable);
                } else {
                    rule.atomic.push(variable);
                }
            }
        }
        for premise in &rule.premises {
            for &(first, root) in premise.discharges.iter().flatten().flatten() {
                let open = rule.patterns[first as usize..=root as usize]
                    .iter()
                    .any(|node| matches!(node, Pattern::Variable(v) if !bound[*v as usize]));
                if open {
                    return Err(unbound("a discharged hypothesis"));
                }
            }
        }
        rule.equations = self.equations(&rule)?;
        Ok(rule)
    }

    /// Returns the equations of a rule's multiset and sequence zones in an
    /// order in which each has at most one context not yet defined.
    fn equations(&self, rule: &Rule) -> Result<Vec<Equation>, Error> {
        let mut open = Vec::new();
        for sequent in 0..=rule.premises.len() {
            for (zone, entries) in rule.shape(sequent).zones.iter().enumerate() {
                let sequence = match self.zones[zone].discipline {
                    Discipline::Set => continue,
                    Discipline::Multiset => false,
                    Discipline::Sequence => true,
                };
                let items = entries
                    .iter()
                    .filter(|e| matches!(e, Entry::Item(_)))
                    .count();
                let segments = if sequence { items as u32 + 1 } else { 1 };
                open.extend((0..segments).map(|segment| (sequent as u32, zone as u32, segment)));
            }
        }
        let mut defined = vec![false; self.contexts.len()];
        let mut order = Vec::with_capacity(open.len());
        while !open.is_empty() {
            let before = open.len();
            open.retain(|&(sequent, zone, segment)| {
                let sequence = self.zones[zone as usize].discipline == Discipline::Sequence;
                let mut unknown = rule
                    .shape(sequent as usize)
                    .segment(zone as usize, segment, sequence)
                    .enumerate()
                    .filter(|(_, (variable, _))| !defined[*variable as usize]);
                let define = match (unknown.next(), unknown.next()) {
                    (None, _) => None,
                    (Some((place, (variable, _))), None) => {
                        defined[variable as usize] = true;
                        Some(place as u32)
                    }
                    (Some(_), Some(_)) => return true,
                };
                order.push(Equation {
                    sequent,
                    zone,
                    segment,
                    define,
                });
                false
            });
            if open.len() == before {
                return Err(invalid(
                    "its contexts cannot be told apart: every context must stand alone in \
                     some zone, or beside contexts that do"
                        .to_owned(),
                ));
            }
        }
        Ok(order)
    }
}
