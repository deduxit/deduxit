// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The checker: whether a derivation is one of a calculus, by matching
//! each node against its rule.
//!
//! Nothing is searched. A node says where each member stands that its
//! rule writes out and that cannot be found otherwise, so matching a
//! pattern is first-order and what a context holds follows from the
//! zone's discipline. Given those positions, a node passes exactly when
//! it is an instance of its rule.

use super::calculus::{Calculus, Discipline, Entry, Find, Item, Pattern, Rule};
use super::derivation::{Derivation, Kind, NodeRef};
use super::terms::{Head, TermId};
use crate::Error;
use crate::limits::{Limits, Phase, Progress, Refusal};

/// How many units of work pass between two asks of the stop inside one
/// node.
const POLL: u64 = 1 << 12;

/// The most characters of a node's sequent an error writes before the
/// `…` that marks a cut.
const SHOWN: usize = 200;

/// The work done and the stop, which every loop of the check goes
/// through.
struct Meter<'a> {
    /// The units done. It saturates: each addition is at most the size of
    /// a vector the derivation holds.
    done: u64,
    /// The units done since the stop was last asked.
    since: u64,
    /// The bound, [`Limits::work`].
    limit: Option<u64>,
    /// The caller's stop.
    stop: &'a mut dyn FnMut(Progress) -> bool,
}

impl Meter<'_> {
    /// Counts `units` of work, and asks the stop once [`POLL`] units
    /// have passed since it was last asked.
    fn tick(&mut self, units: usize) -> Result<(), Error> {
        self.done = self.done.saturating_add(units as u64);
        self.since = self.since.saturating_add(units as u64);
        if let Some(limit) = self.limit
            && self.done > limit
        {
            return Err(Error::Refused(Refusal::Work { limit }));
        }
        if self.since >= POLL {
            self.ask()?;
        }
        Ok(())
    }

    /// Asks the stop.
    fn ask(&mut self) -> Result<(), Error> {
        let progress = Progress::new(Phase::Check, self.since, self.done);
        self.since = 0;
        if (self.stop)(progress) {
            return Err(Error::Refused(Refusal::Stopped {
                phase: Phase::Check,
            }));
        }
        Ok(())
    }
}

/// Which nodes a check checks.
#[derive(Clone, Copy, Debug)]
enum Which {
    /// Every node.
    All,
    /// Every node but the open goals.
    #[cfg_attr(
        not(all(feature = "interactive", feature = "parse")),
        expect(dead_code, reason = "a session read from JSON is checked so")
    )]
    Open,
    /// The node at this index alone.
    #[cfg_attr(
        not(all(feature = "interactive", feature = "parse")),
        expect(dead_code, reason = "a session checks each step so")
    )]
    Node(usize),
}

/// Why a node is no instance of its rule. A refusal is told apart from
/// it, so that a bound never reads as a fault.
enum Fault {
    /// The node breaks the calculus, for this reason.
    Invalid(String),
    /// A bound or the stop ended the check.
    Refused(Error),
}

impl From<Error> for Fault {
    fn from(error: Error) -> Self {
        Self::Refused(error)
    }
}

impl From<&str> for Fault {
    fn from(reason: &str) -> Self {
        Self::Invalid(reason.to_owned())
    }
}

impl From<String> for Fault {
    fn from(reason: String) -> Self {
        Self::Invalid(reason)
    }
}

/// One check of a derivation against a calculus: what was resolved once,
/// and the vectors every node's check reuses.
struct Checker<'a> {
    /// The derivation.
    derivation: &'a Derivation,
    /// The calculus.
    calculus: &'a Calculus,
    /// The work and the stop.
    meter: Meter<'a>,
    /// Per connective of the derivation's terms, the calculus's.
    global: Vec<u32>,
    /// Per connective of the calculus, the derivation's, if a term uses
    /// it.
    local: Vec<Option<u32>>,
    /// Per term, its sort in the calculus.
    sorts: Vec<u32>,
    /// The hypothesis nodes, sorted by scope.
    hypotheses: Vec<(u32, u32)>,
    /// Per schematic variable, the term it is bound to.
    bindings: Vec<Option<TermId>>,
    /// Per context variable, its members: sorted in a multiset or set,
    /// in order in a sequence.
    contexts: Vec<Vec<TermId>>,
    /// Per context variable of a set, whether a zone has bounded it yet.
    bounded: Vec<bool>,
    /// Per sequent of the node's rule and zone, which members a written
    /// member took, and where the written members stand.
    taken: Vec<Vec<bool>>,
    /// Per sequent and zone, the places of its written members in the
    /// order written.
    places: Vec<Vec<u32>>,
    /// Per term, the walk that last visited it.
    seen: Vec<u32>,
    /// The number of the current walk.
    walk: u32,
    /// A stack for the walks over terms and patterns.
    stack: Vec<(u32, TermId)>,
}

impl Derivation {
    /// Checks that the derivation is one of `calculus`: every node an
    /// instance of the rule it names, every hypothesis discharged from
    /// the scope it lies in, the root in the root scope, no goal left
    /// open. `Ok` reads "valid in the
    /// calculus given": it says nothing of what the calculus's rules
    /// mean, and nothing vouches for them.
    ///
    /// The hypotheses of the root scope stay:
    /// [`assumptions`](Self::assumptions) lists them, and the derivation
    /// shows its root's sequent from them.
    ///
    /// A node, a member of a sequent it handles, and a term or scope it
    /// walks through are each a unit of `limits.work`. `stop` is asked at
    /// every node and every few thousand units within one.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidDerivation`] naming the first node that fails and
    /// why. [`Refusal::Memory`] where the check would hold more than
    /// `limits.memory_bytes`, [`Refusal::Work`] past the bound and
    /// [`Refusal::Stopped`] when `stop` fired; none of the three is a
    /// verdict.
    pub fn check(
        &self,
        calculus: &Calculus,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        self.check_some(calculus, limits, stop, Which::All)
    }

    /// Checks the derivation as [`check`](Self::check) does, but for its
    /// open goals, which it takes as they are: a proof in progress whose
    /// every step keeps to the calculus.
    #[cfg_attr(
        not(all(feature = "interactive", feature = "parse")),
        expect(dead_code, reason = "a session read from JSON is checked so")
    )]
    pub(crate) fn check_open(
        &self,
        calculus: &Calculus,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        self.check_some(calculus, limits, stop, Which::Open)
    }

    /// Checks the one node at `node` against its rule, or as a
    /// hypothesis, the rest of the derivation taken as it is.
    #[cfg_attr(
        not(all(feature = "interactive", feature = "parse")),
        expect(dead_code, reason = "a session checks each step so")
    )]
    pub(crate) fn check_node(
        &self,
        calculus: &Calculus,
        node: usize,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        self.check_some(calculus, limits, stop, Which::Node(node))
    }

    /// Checks the nodes `which` names.
    fn check_some(
        &self,
        calculus: &Calculus,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
        which: Which,
    ) -> Result<(), Error> {
        let invalid = |reason: String| Error::InvalidDerivation {
            calculus: calculus.name().into(),
            reason,
        };
        if self.calculus() != calculus.name() {
            return Err(invalid(format!(
                "the derivation names the calculus {:?}",
                self.calculus()
            )));
        }
        if self.zones() != calculus.zones.len() {
            return Err(invalid(format!(
                "its sequents have {} zones, the calculus's {}",
                self.zones(),
                calculus.zones.len()
            )));
        }
        // What the check holds, in bytes, each sum saturating:
        // - per term its sort, the walk that saw it and a place on the
        //   freshness walk's stack, which pushes a term once;
        // - per connective, variable and context of the calculus a slot;
        // - per node its place among the hypotheses;
        // - for the node being checked, per sequent of its rule and zone
        //   the marks and places of its members (two vectors), and copies
        //   of a zone's members, each at most the widest sequent long;
        // - every context's members, each at most the widest sequent;
        // - per rule its patterns' stacks and the hypotheses it discharges.
        let widest = self
            .nodes()
            .map(|node| {
                (0..self.zones())
                    .map(|zone| node.zone(zone).len())
                    .sum::<usize>()
            })
            .max()
            .unwrap_or(0) as u64;
        let (sequents, rule_bytes) = calculus.rules.iter().fold((1u64, 0u64), |(s, r), rule| {
            let discharges = rule
                .premises
                .iter()
                .map(|p| p.discharges.len() as u64)
                .sum::<u64>();
            let bytes = (rule.patterns.len() as u64)
                .saturating_mul(16)
                .saturating_add(discharges.saturating_mul(calculus.zones.len() as u64 * 24));
            (s.max(rule.premises.len() as u64 + 1), r.max(bytes))
        });
        let zones = calculus.zones.len() as u64;
        // Per term: its sort, its walk mark, its connective's or sort's
        // index, and a place on the walk's stack, whose vector may hold
        // twice its length.
        let needed = (self.terms().len() as u64)
            .saturating_mul(32)
            .saturating_add((self.len() as u64).saturating_mul(8))
            // Per connective, variable and context of the calculus, its
            // slot in the checker's tables.
            .saturating_add((calculus.connectives.len() as u64).saturating_mul(8))
            .saturating_add(u64::from(calculus.variables).saturating_mul(8))
            .saturating_add((calculus.contexts.len() as u64).saturating_mul(32))
            .saturating_add(widest.saturating_mul(sequents.saturating_mul(24)))
            .saturating_add(sequents.saturating_mul(zones).saturating_mul(48))
            .saturating_add(
                (calculus.contexts.len() as u64)
                    .saturating_mul(widest)
                    .saturating_mul(4),
            )
            .saturating_add(rule_bytes);
        if let Some(limit_bytes) = limits.memory_bytes
            && needed > limit_bytes
        {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Check,
                limit_bytes,
                needed_bytes: Some(needed),
            }));
        }
        let mut checker = Checker {
            derivation: self,
            calculus,
            meter: Meter {
                done: 0,
                since: 0,
                limit: limits.work,
                stop: &mut stop,
            },
            global: Vec::new(),
            local: vec![None; calculus.connectives.len()],
            sorts: Vec::with_capacity(self.terms().len()),
            hypotheses: Vec::new(),
            bindings: vec![None; calculus.variables as usize],
            contexts: vec![Vec::new(); calculus.contexts.len()],
            bounded: vec![false; calculus.contexts.len()],
            taken: Vec::new(),
            places: Vec::new(),
            seen: Vec::new(),
            walk: 0,
            stack: Vec::new(),
        };
        checker.terms().map_err(|fault| match fault {
            Fault::Invalid(reason) => invalid(reason),
            Fault::Refused(error) => error,
        })?;
        for node in self.nodes() {
            let index = node.id().index();
            let skipped = match which {
                Which::All => false,
                Which::Open => self.nodes[index].kind == Kind::Open,
                Which::Node(only) => index != only,
            };
            if skipped {
                continue;
            }
            checker.meter.ask()?;
            checker.node(node).map_err(|fault| match fault {
                Fault::Refused(error) => error,
                Fault::Invalid(reason) => {
                    let index = node.id().index();
                    // The zones are the calculus's, checked above.
                    let sequent = calculus.show_sequent(self, node.id()).limit(SHOWN);
                    invalid(match self.nodes[index].kind {
                        Kind::Rule(rule) => format!(
                            "node {index} ({}, concluding {sequent}): {reason}",
                            self.rules[rule as usize]
                        ),
                        Kind::Hypothesis | Kind::Open => {
                            format!("node {index} ({sequent}): {reason}")
                        }
                    })
                }
            })?;
        }
        // A root inside a scope would rest on hypotheses that no rule
        // discharges and that are no assumptions either.
        match self.nodes.last() {
            None => Err(invalid("it has no node".to_owned())),
            Some(_) if matches!(which, Which::Node(_)) => Ok(()),
            Some(root) if root.scope != 0 => Err(invalid(format!(
                "node {}: the root lies in a scope that no rule closes",
                self.len() - 1
            ))),
            Some(_) => Ok(()),
        }
    }
}

impl<'a> Checker<'a> {
    /// Resolves the terms' names against the calculus and finds every
    /// term's sort, and lists the hypotheses by scope.
    fn terms(&mut self) -> Result<(), Fault> {
        let (terms, calculus) = (self.derivation.terms(), self.calculus);
        self.meter
            .tick(terms.connectives().len() + terms.sorts().len())?;
        for name in terms.connectives() {
            let Some(&index) = calculus.connective_ids.get(name) else {
                return Err(format!("the calculus has no connective {name:?}").into());
            };
            self.local[index as usize] = Some(self.global.len() as u32);
            self.global.push(index);
        }
        let mut atom_sorts = Vec::with_capacity(terms.sorts().len());
        for name in terms.sorts() {
            let Some(&sort) = calculus.sort_ids.get(name) else {
                return Err(format!("the calculus has no sort {name:?}").into());
            };
            atom_sorts.push(sort);
        }
        for index in 0..terms.len() {
            let (head, arguments) = terms.parts(TermId::new(index as u32));
            self.meter.tick(1 + arguments.len())?;
            let sort = match head {
                Head::Atom(atom) => atom_sorts[terms.sort_of(atom) as usize],
                Head::Connective(connective) => {
                    let signature =
                        &calculus.connectives[self.global[connective as usize] as usize];
                    let fits = signature.arguments.len() == arguments.len()
                        && arguments
                            .iter()
                            .zip(&signature.arguments)
                            .all(|(argument, &sort)| self.sorts[argument.index()] == sort);
                    if !fits {
                        return Err(format!(
                            "term {index}: {:?} is applied to another number or sort of \
                             arguments than it takes",
                            terms.connectives()[connective as usize]
                        )
                        .into());
                    }
                    signature.result
                }
            };
            self.sorts.push(sort);
        }
        self.seen = vec![0; terms.len()];
        for (index, entry) in self.derivation.nodes.iter().enumerate() {
            if entry.kind == Kind::Hypothesis {
                self.hypotheses.push((entry.scope, index as u32));
            }
        }
        self.meter.tick(self.derivation.len())?;
        self.hypotheses.sort_unstable();
        Ok(())
    }

    /// Returns where the hypothesis nodes of `scope` lie among the
    /// hypotheses.
    fn hypothesis_range(&self, scope: u32) -> std::ops::Range<usize> {
        let start = self.hypotheses.partition_point(|&(s, _)| s < scope);
        let end = self.hypotheses.partition_point(|&(s, _)| s <= scope);
        start..end
    }

    /// Returns the node at `index` of the derivation.
    fn at(&self, index: u32) -> NodeRef<'a> {
        self.derivation.at(index as usize)
    }

    /// Checks one node: its sequent's shape, then what justifies it.
    fn node(&mut self, node: NodeRef<'a>) -> Result<(), Fault> {
        let calculus = self.calculus;
        for (index, zone) in calculus.zones.iter().enumerate() {
            let members = node.zone(index);
            self.meter.tick(1 + members.len())?;
            let len = members.len() as u64;
            if len < u64::from(zone.least) || zone.most.is_some_and(|most| len > u64::from(most)) {
                return Err(format!(
                    "the zone {:?} holds {len} members, outside what the calculus allows",
                    zone.name
                )
                .into());
            }
            if members.iter().any(|m| self.sorts[m.index()] != zone.sort) {
                return Err(format!(
                    "a member of the zone {:?} has another sort than the zone",
                    zone.name
                )
                .into());
            }
            if zone.discipline == Discipline::Set {
                let mut sorted = members.to_vec();
                sorted.sort_unstable();
                if sorted.windows(2).any(|pair| pair[0] == pair[1]) {
                    return Err(format!(
                        "the zone {:?} is a set and holds a member twice",
                        zone.name
                    )
                    .into());
                }
            }
        }
        let entry = self.derivation.nodes[node.id().index()];
        match entry.kind {
            Kind::Open => Err("an open goal".into()),
            Kind::Hypothesis if node.premises().is_empty() && node.positions().is_empty() => Ok(()),
            Kind::Hypothesis => Err("a hypothesis has neither premises nor positions".into()),
            Kind::Rule(rule) => {
                let name = &self.derivation.rules[rule as usize];
                let Some(&index) = calculus.rule_ids.get(name) else {
                    return Err("the calculus has no rule of this name".into());
                };
                self.rule(node, &calculus.rules[index as usize])
            }
        }
    }

    /// Checks a node against its rule.
    fn rule(&mut self, node: NodeRef<'a>, rule: &'a Rule) -> Result<(), Fault> {
        let premises = node.premises();
        if premises.len() != rule.premises.len() {
            return Err(format!(
                "{} premises, where the rule has {}",
                premises.len(),
                rule.premises.len()
            )
            .into());
        }
        if node.positions().len() != rule.positions as usize {
            return Err(format!(
                "{} positions, where the rule needs {}",
                node.positions().len(),
                rule.positions
            )
            .into());
        }
        let zones = self.calculus.zones.len();
        let sequents: Vec<NodeRef<'a>> = std::iter::once(node)
            .chain(premises.iter().map(|p| self.at(p.index() as u32)))
            .collect();
        self.bindings.fill(None);
        self.taken.resize_with(sequents.len() * zones, Vec::new);
        self.places.resize_with(sequents.len() * zones, Vec::new);
        // The written members.
        let mut positions = node.positions().iter();
        for (s, sequent) in sequents.iter().enumerate() {
            for zone in 0..zones {
                self.meter.tick(1)?;
                let members = sequent.zone(zone);
                let discipline = self.calculus.zones[zone].discipline;
                let slot = s * zones + zone;
                self.taken[slot].clear();
                self.taken[slot].resize(members.len(), false);
                self.places[slot].clear();
                // The members found by value last: they bind nothing, and
                // the first free one of equal value must not take the
                // place a position names. A sequence has none of them, so
                // its places keep the order written.
                let items = rule.shape(s).zones[zone]
                    .iter()
                    .filter_map(|entry| match entry {
                        Entry::Item(item) => Some(item),
                        Entry::Context { .. } => None,
                    });
                let (by_value, placed): (Vec<Item>, Vec<Item>) =
                    items.partition(|item| item.find == Find::Value);
                for item in placed.into_iter().chain(by_value) {
                    self.meter.tick(1 + (item.root - item.first) as usize)?;
                    // Where the member is looked for, for the message.
                    let sequent = match s {
                        0 => "the conclusion".to_owned(),
                        _ if sequents.len() == 2 => "the premise".to_owned(),
                        _ => format!("premise {s}"),
                    };
                    let zone_name = &self.calculus.zones[zone].name;
                    let missing = || -> Fault {
                        format!(
                            "{sequent} lacks a formula the rule writes out in its zone {zone_name:?}"
                        )
                        .into()
                    };
                    let place = match item.find {
                        Find::Start(place) => Some(place as usize),
                        Find::End(place) => members.len().checked_sub(1 + place as usize),
                        Find::Position => positions.next().map(|&place| place as usize),
                        Find::Value => {
                            let wanted = self
                                .instance(rule, item.first, item.root)
                                .ok_or_else(missing)?;
                            self.meter.tick(members.len())?;
                            let taken = &self.taken[slot];
                            members.iter().enumerate().position(|(at, &member)| {
                                member == wanted && (discipline == Discipline::Set || !taken[at])
                            })
                        }
                    };
                    let place = match place.filter(|&place| place < members.len()) {
                        Some(place) => place,
                        None if item.find == Find::Value => return Err(missing()),
                        None => {
                            return Err(format!(
                                "{sequent} has no member where the rule writes one out in its \
                                 zone {zone_name:?}"
                            )
                            .into());
                        }
                    };
                    if discipline != Discipline::Set && self.taken[slot][place] {
                        return Err("two members the rule writes out stand at one place".into());
                    }
                    if item.find != Find::Value && !self.matches(rule, item.root, members[place])? {
                        return Err("a member is no instance of what the rule writes out".into());
                    }
                    self.taken[slot][place] = true;
                    self.places[slot].push(place as u32);
                }
                if discipline == Discipline::Sequence
                    && self.places[slot].windows(2).any(|pair| pair[0] >= pair[1])
                {
                    return Err("the members the rule writes out stand in another order".into());
                }
            }
        }
        self.conditions(node, rule)?;
        self.equations(&sequents, rule)?;
        self.sets(&sequents, rule)?;
        self.scopes(node, rule)
    }

    /// Returns whether `term` is an instance of the pattern at `root`,
    /// binding the variables it meets first here.
    fn matches(&mut self, rule: &Rule, root: u32, term: TermId) -> Result<bool, Fault> {
        let terms = self.derivation.terms();
        self.stack.clear();
        self.stack.push((root, term));
        while let Some((pattern, term)) = self.stack.pop() {
            self.meter.tick(1)?;
            match rule.patterns[pattern as usize] {
                Pattern::Variable(variable) => match self.bindings[variable as usize] {
                    None => self.bindings[variable as usize] = Some(term),
                    Some(bound) if bound == term => {}
                    Some(_) => return Ok(false),
                },
                Pattern::Apply {
                    connective,
                    start,
                    len,
                } => {
                    let (head, arguments) = terms.parts(term);
                    let same = matches!(head, Head::Connective(local)
                        if self.global[local as usize] == connective);
                    // A term's arguments are as many as its connective
                    // takes: the pass over the terms checked it.
                    if !same {
                        return Ok(false);
                    }
                    let patterns = rule.arguments(start, len);
                    self.stack
                        .extend(patterns.iter().copied().zip(arguments.iter().copied()));
                }
            }
        }
        Ok(true)
    }

    /// Returns the term that the pattern with nodes `first` to `root` is
    /// under the bindings, if the derivation's terms hold it. Nothing is
    /// built: a term the arena lacks is in no sequent.
    fn instance(&self, rule: &Rule, first: u32, root: u32) -> Option<TermId> {
        let terms = self.derivation.terms();
        let mut built: Vec<TermId> = Vec::with_capacity((root - first) as usize + 1);
        let mut arguments = Vec::new();
        for node in first..=root {
            let term = match rule.patterns[node as usize] {
                Pattern::Variable(variable) => self.bindings[variable as usize]?,
                Pattern::Apply {
                    connective,
                    start,
                    len,
                } => {
                    arguments.clear();
                    arguments.extend(
                        rule.arguments(start, len)
                            .iter()
                            .map(|&argument| built[(argument - first) as usize]),
                    );
                    terms.find(
                        Head::Connective(self.local[connective as usize]?),
                        &arguments,
                    )?
                }
            };
            built.push(term);
        }
        built.last().copied()
    }

    /// Checks the rule's conditions on its variables.
    fn conditions(&mut self, node: NodeRef<'a>, rule: &Rule) -> Result<(), Fault> {
        let terms = self.derivation.terms();
        let atom = |checker: &Self, variable: u32| {
            checker.bindings[variable as usize]
                .filter(|&term| matches!(terms.parts(term).0, Head::Atom(_)))
        };
        self.meter.tick(rule.atomic.len() + rule.fresh.len())?;
        for &variable in &rule.atomic {
            atom(self, variable).ok_or("a formula the rule asks to be an atom is none")?;
        }
        for (index, &variable) in rule.fresh.iter().enumerate() {
            let name = atom(self, variable).ok_or("a name the rule asks to be new is no atom")?;
            let again = rule.fresh[..index]
                .iter()
                .any(|&other| self.bindings[other as usize] == Some(name));
            if again {
                return Err("two names the rule asks to be new are one".into());
            }
            // A new walk; the marks of a walk long ago are cleared
            // before its number comes round again.
            if self.walk == u32::MAX {
                self.seen.fill(0);
                self.walk = 0;
            }
            self.walk += 1;
            for zone in 0..self.calculus.zones.len() {
                if self.occurs(name, node.zone(zone))? {
                    return Err("a name the rule asks to be new occurs in the conclusion".into());
                }
            }
            let mut scope = node.scope().index() as u32;
            loop {
                self.meter.tick(1)?;
                for at in self.hypothesis_range(scope) {
                    let hypothesis = self.at(self.hypotheses[at].1);
                    for zone in 0..self.calculus.zones.len() {
                        if self.occurs(name, hypothesis.zone(zone))? {
                            return Err("a name the rule asks to be new occurs in a hypothesis \
                                        in force"
                                .into());
                        }
                    }
                }
                if scope == 0 {
                    break;
                }
                scope = self.derivation.scopes[scope as usize];
            }
        }
        Ok(())
    }

    /// Returns whether the atom `name` occurs in one of `members`,
    /// visiting each term once per walk.
    fn occurs(&mut self, name: TermId, members: &[TermId]) -> Result<bool, Fault> {
        let terms = self.derivation.terms();
        self.meter.tick(1 + members.len())?;
        self.stack.clear();
        // A term is marked as it is pushed, so the stack holds each term
        // once at most. A term's arguments lie before it, so a term
        // before the atom does not hold it.
        let walk = self.walk;
        let push = |stack: &mut Vec<(u32, TermId)>, seen: &mut [u32], term: TermId| {
            if term < name || seen[term.index()] == walk {
                return false;
            }
            seen[term.index()] = walk;
            stack.push((0, term));
            term == name
        };
        for &member in members {
            if push(&mut self.stack, &mut self.seen, member) {
                return Ok(true);
            }
        }
        while let Some((_, term)) = self.stack.pop() {
            let (_, arguments) = terms.parts(term);
            self.meter.tick(1 + arguments.len())?;
            for &argument in arguments {
                if push(&mut self.stack, &mut self.seen, argument) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Returns the members a context stands for in a zone: its own, or
    /// each under the connective. `None` where the derivation's terms
    /// lack one, so that no sequent holds it.
    fn mapped(&self, variable: u32, under: Option<u32>) -> Option<Vec<TermId>> {
        let members = &self.contexts[variable as usize];
        let Some(connective) = under else {
            return Some(members.clone());
        };
        // An empty context stands for no member, whether or not a term
        // is built with the connective.
        if members.is_empty() {
            return Some(Vec::new());
        }
        let local = self.local[connective as usize]?;
        let terms = self.derivation.terms();
        members
            .iter()
            .map(|&member| terms.find(Head::Connective(local), &[member]))
            .collect()
    }

    /// Returns what stands under the connective in each of `members`, or
    /// `None` where one is not built with it.
    fn unmapped(&self, members: &[TermId], under: Option<u32>) -> Option<Vec<TermId>> {
        let Some(connective) = under else {
            return Some(members.to_vec());
        };
        let terms = self.derivation.terms();
        members
            .iter()
            .map(|&member| match terms.parts(member) {
                (Head::Connective(local), &[body]) if self.global[local as usize] == connective => {
                    Some(body)
                }
                _ => None,
            })
            .collect()
    }

    /// Solves the equations of the rule's multiset and sequence zones:
    /// what each zone holds beyond its written members is its contexts.
    fn equations(&mut self, sequents: &[NodeRef<'a>], rule: &Rule) -> Result<(), Fault> {
        let zones = self.calculus.zones.len();
        let differs = "the contexts are not what the rule makes of them";
        for equation in &rule.equations {
            let (s, zone) = (equation.sequent as usize, equation.zone as usize);
            let slot = s * zones + zone;
            let members = sequents[s].zone(zone);
            self.meter.tick(1 + members.len())?;
            let sequence = self.calculus.zones[zone].discipline == Discipline::Sequence;
            let segment = &equation.contexts;
            if sequence {
                // The members between two written ones, in order.
                let places = &self.places[slot];
                let from = match equation.segment.checked_sub(1) {
                    None => 0,
                    Some(before) => places[before as usize] as usize + 1,
                };
                let to = places
                    .get(equation.segment as usize)
                    .map_or(members.len(), |&place| place as usize);
                let mut rest = &members[from..to];
                let define = equation.define.map(|place| place as usize);
                // The contexts before the one defined, from the start.
                for &(variable, under) in &segment[..define.unwrap_or(segment.len())] {
                    let known = self.mapped(variable, under).ok_or(differs)?;
                    self.meter.tick(known.len())?;
                    rest = rest.strip_prefix(&known[..]).ok_or(differs)?;
                }
                // Those after it, from the end.
                if let Some(place) = define {
                    for &(variable, under) in segment[place + 1..].iter().rev() {
                        let known = self.mapped(variable, under).ok_or(differs)?;
                        self.meter.tick(known.len())?;
                        rest = rest.strip_suffix(&known[..]).ok_or(differs)?;
                    }
                    let (variable, under) = segment[place];
                    self.contexts[variable as usize] = self.unmapped(rest, under).ok_or(differs)?;
                } else if !rest.is_empty() {
                    return Err(differs.into());
                }
                continue;
            }
            let taken = &self.taken[slot];
            let mut rest: Vec<TermId> = members
                .iter()
                .zip(taken)
                .filter_map(|(&member, &taken)| (!taken).then_some(member))
                .collect();
            rest.sort_unstable();
            for (place, &(variable, under)) in segment.iter().enumerate() {
                if equation.define == Some(place as u32) {
                    continue;
                }
                let mut known = self.mapped(variable, under).ok_or(differs)?;
                self.meter.tick(known.len())?;
                known.sort_unstable();
                rest = difference(&rest, &known).ok_or(differs)?;
            }
            match equation.define {
                Some(place) => {
                    let (variable, under) = segment[place as usize];
                    let mut value = self.unmapped(&rest, under).ok_or(differs)?;
                    value.sort_unstable();
                    self.contexts[variable as usize] = value;
                }
                None if rest.is_empty() => {}
                None => return Err(differs.into()),
            }
        }
        Ok(())
    }

    /// Checks the rule's set zones. A context is at most each zone it
    /// stands in, so the largest it can be is what those zones share;
    /// the node is an instance exactly if, with every context that
    /// large, each zone is covered by its written members and contexts.
    fn sets(&mut self, sequents: &[NodeRef<'a>], rule: &Rule) -> Result<(), Fault> {
        let zones = self.calculus.zones.len();
        let set = |zone: usize| self.calculus.zones[zone].discipline == Discipline::Set;
        if !(0..zones).any(set) {
            return Ok(());
        }
        self.bounded.fill(false);
        for (s, sequent) in sequents.iter().enumerate() {
            for zone in (0..zones).filter(|&zone| set(zone)) {
                let members = sequent.zone(zone);
                for entry in &rule.shape(s).zones[zone] {
                    let Entry::Context { variable, under } = *entry else {
                        continue;
                    };
                    self.meter.tick(1 + members.len())?;
                    // What the context may hold of this zone.
                    let mut bound: Vec<TermId> = match under {
                        None => members.to_vec(),
                        Some(_) => members
                            .iter()
                            .filter_map(|&m| self.unmapped(&[m], under).map(|body| body[0]))
                            .collect(),
                    };
                    bound.sort_unstable();
                    let variable = variable as usize;
                    if std::mem::replace(&mut self.bounded[variable], true) {
                        self.contexts[variable].retain(|m| bound.binary_search(m).is_ok());
                    } else {
                        self.contexts[variable] = bound;
                    }
                }
            }
        }
        for (s, sequent) in sequents.iter().enumerate() {
            for zone in (0..zones).filter(|&zone| set(zone)) {
                let members = sequent.zone(zone);
                let entries = rule.shape(s).zones[zone].len();
                self.meter
                    .tick(members.len().saturating_mul(1 + entries).saturating_add(1))?;
                let taken = &self.taken[s * zones + zone];
                let covered = members.iter().zip(taken).all(|(&member, &taken)| {
                    taken
                        || rule.shape(s).zones[zone].iter().any(|entry| match *entry {
                            Entry::Item(_) => false,
                            Entry::Context { variable, under } => {
                                self.unmapped(&[member], under).is_some_and(|body| {
                                    self.contexts[variable as usize]
                                        .binary_search(&body[0])
                                        .is_ok()
                                })
                            }
                        })
                });
                if !covered {
                    return Err(
                        "a member is neither written out by the rule nor in a context \
                                all its sequents share"
                            .into(),
                    );
                }
            }
        }
        Ok(())
    }

    /// Returns whether the scope `ancestor` is `scope` or lies around it.
    fn around(&mut self, ancestor: u32, mut scope: u32) -> Result<bool, Fault> {
        // A scope's parent has a smaller index, so the walk ends.
        while scope > ancestor {
            self.meter.tick(1)?;
            scope = self.derivation.scopes[scope as usize];
        }
        Ok(scope == ancestor)
    }

    /// Checks where the premises lie: in the node's scope or one around
    /// it, or, for a premise whose rule discharges hypotheses, in a scope
    /// directly inside one of those, every hypothesis of which the rule
    /// discharges.
    fn scopes(&mut self, node: NodeRef<'a>, rule: &Rule) -> Result<(), Fault> {
        let zones = self.calculus.zones.len();
        let own = node.scope().index() as u32;
        for (premise, shape) in node.premises().iter().zip(&rule.premises) {
            let scope = self.derivation.nodes[premise.index()].scope;
            if shape.discharges.is_empty() {
                if !self.around(scope, own)? {
                    return Err("a premise lies in a scope that is closed here".into());
                }
                continue;
            }
            // A premise in force at the node discharges nothing.
            if self.around(scope, own)? {
                continue;
            }
            let parent = self.derivation.scopes[scope as usize];
            if !self.around(parent, own)? {
                return Err(
                    "a premise that discharges hypotheses lies in no scope directly \
                     inside the node's or one around it"
                        .into(),
                );
            }
            // The hypotheses the premise may discharge, each zone's
            // members sorted where their order does not count.
            let mut allowed: Vec<Vec<Vec<TermId>>> = Vec::with_capacity(shape.discharges.len());
            for hypothesis in &shape.discharges {
                let nodes = hypothesis
                    .iter()
                    .flatten()
                    .map(|&(first, root)| (root - first) as usize + 1)
                    .fold(hypothesis.len(), usize::saturating_add);
                self.meter.tick(nodes)?;
                let instance: Option<Vec<Vec<TermId>>> = hypothesis
                    .iter()
                    .enumerate()
                    .map(|(zone, items)| {
                        let mut members = items
                            .iter()
                            .map(|&(first, root)| self.instance(rule, first, root))
                            .collect::<Option<Vec<_>>>()?;
                        sort_by(&mut members, self.calculus.zones[zone].discipline);
                        Some(members)
                    })
                    .collect();
                allowed.extend(instance);
            }
            for at in self.hypothesis_range(scope) {
                let hypothesis = self.at(self.hypotheses[at].1);
                // Its zones, sorted where their order does not count, once
                // for every pattern it is compared with.
                let mut members: Vec<Vec<TermId>> = Vec::with_capacity(zones);
                let mut size = 0usize;
                for (zone, of) in self.calculus.zones.iter().enumerate() {
                    let mut list = hypothesis.zone(zone).to_vec();
                    sort_by(&mut list, of.discipline);
                    size = size.saturating_add(list.len());
                    members.push(list);
                }
                self.meter
                    .tick(size.saturating_add(zones).saturating_mul(1 + allowed.len()))?;
                let discharged = allowed.contains(&members);
                if !discharged {
                    return Err(format!(
                        "the hypothesis at node {} lies in the scope of a premise and is none \
                         the rule discharges",
                        hypothesis.id().index()
                    )
                    .into());
                }
            }
        }
        Ok(())
    }
}

/// Puts `members` in the form two zones of `discipline` are compared in:
/// sorted, and once each in a set; as written in a sequence.
fn sort_by(members: &mut Vec<TermId>, discipline: Discipline) {
    match discipline {
        Discipline::Sequence => {}
        Discipline::Multiset => members.sort_unstable(),
        Discipline::Set => {
            members.sort_unstable();
            members.dedup();
        }
    }
}

/// Returns the sorted multiset `whole` without the sorted multiset
/// `part`, or `None` where `part` holds a member more often.
fn difference(whole: &[TermId], part: &[TermId]) -> Option<Vec<TermId>> {
    let mut rest = Vec::with_capacity(whole.len().saturating_sub(part.len()));
    let mut part = part.iter().peekable();
    for &member in whole {
        match part.peek() {
            Some(&&next) if next == member => {
                part.next();
            }
            Some(&&next) if next < member => return None,
            _ => rest.push(member),
        }
    }
    part.next().is_none().then_some(rest)
}
