// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A derivation in a calculus given as data: terms, nodes that name
//! their rules, and scopes.

use super::terms::{TermId, Terms};
use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Refusal, Space};

/// A node of a [`Derivation`], by its index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(u32);

impl NodeId {
    /// Returns the node's index in its derivation.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// A scope of a [`Derivation`], by its index. The root scope is `0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScopeId(u32);

impl ScopeId {
    /// The root scope, whose hypotheses are the derivation's assumptions.
    pub const ROOT: Self = Self(0);

    /// Returns the scope's index in its derivation.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// What justifies a node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Step<'a> {
    /// The rule of this name.
    Rule(&'a str),
    /// Nothing: the node is a hypothesis of its scope, until a rule
    /// discharges it.
    Hypothesis,
    /// Nothing yet: the node is an open goal of a proof in progress. No
    /// derivation the checker accepts has one.
    Open,
}

/// A node as [`Derivation::push`] takes it.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Node<'a> {
    /// The members of each zone of its sequent, in the calculus's order
    /// of zones.
    pub sequent: &'a [Vec<TermId>],
    /// What justifies it.
    pub step: Step<'a>,
    /// Its premises, as indices of earlier nodes, in the rule's order.
    pub premises: &'a [u32],
    /// The positions of the members its rule finds by position
    /// ([`Calculus::positions`](super::Calculus::positions)).
    pub positions: &'a [u32],
    /// The index of its scope.
    pub scope: u32,
}

impl<'a> Node<'a> {
    /// Returns a node of the root scope with this sequent and
    /// justification, without premises or positions.
    pub const fn new(sequent: &'a [Vec<TermId>], step: Step<'a>) -> Self {
        Self {
            sequent,
            step,
            premises: &[],
            positions: &[],
            scope: 0,
        }
    }

    /// Returns the node with these premises.
    #[must_use]
    pub const fn premises(mut self, premises: &'a [u32]) -> Self {
        self.premises = premises;
        self
    }

    /// Returns the node with these positions.
    #[must_use]
    pub const fn positions(mut self, positions: &'a [u32]) -> Self {
        self.positions = positions;
        self
    }

    /// Returns the node in this scope.
    #[must_use]
    pub const fn scope(mut self, scope: ScopeId) -> Self {
        self.scope = scope.0;
        self
    }
}

/// What justifies a stored node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    /// The rule at this index among the derivation's rule names.
    Rule(u32),
    /// A hypothesis.
    Hypothesis,
    /// An open goal.
    Open,
}

/// A stored node. Its members, premises and positions lie in the
/// derivation's vectors, each node's after the node's before it.
#[derive(Clone, Copy, Debug)]
pub(super) struct Entry {
    /// What justifies it.
    pub(super) kind: Kind,
    /// Its scope.
    pub(super) scope: u32,
    /// Where its premises end among the derivation's.
    premises_end: u32,
    /// Where its positions end among the derivation's.
    positions_end: u32,
}

/// A derivation in a calculus given as data ([`Calculus`]): terms, and
/// nodes that each hold a sequent, the rule that concludes it and its
/// premises, which come before it. The last node is the root.
///
/// A node may also be a hypothesis. Hypotheses and the nodes that depend
/// on them lie in scopes, which form a tree: a rule that discharges
/// hypotheses takes a premise from a scope inside its own, and every
/// hypothesis of that scope is one the rule discharges. The hypotheses
/// of the root scope are the derivation's assumptions. This serves
/// natural deduction, in Gentzen's trees and in Fitch's nested
/// subproofs, beside sequent calculi and tableaux, which use the root
/// scope alone.
///
/// A derivation is only data. [`check`](Self::check) says whether it is
/// one of a calculus, and its verdict is about that calculus alone:
/// nothing says the calculus's rules are sound.
///
/// # JSON
///
/// ```json
/// {"version": 15, "calculus": "LK",
///  "terms": ["a", "b", ["∧", 0, 1]],
///  "nodes": [{"sequent": [[0], [0]], "rule": "ax"},
///            {"sequent": [[2], [0]], "rule": "∧L₁", "at": [0], "premises": [0]}]}
/// ```
///
/// - A term is an atom's name, `{"atom": name, "sort": sort}` for an
///   atom that is no formula, or an array of a connective's name and the
///   indices of its arguments, which come before it.
/// - A node's `sequent` lists the members of each zone as indices of
///   terms. `rule` names its rule, `"hypothesis": true` makes it a
///   hypothesis, and a node with neither is an open goal. `premises`
///   are indices of earlier nodes, `at` the positions its rule needs,
///   `scope` its scope where that is not the root scope.
/// - `scopes` lists the parent of every scope after the root scope,
///   which is scope `0`.
///
/// Reading one checks its indices, not its rules.
///
/// [`Calculus`]: super::Calculus
#[derive(Clone, Debug)]
pub struct Derivation {
    /// The name of the calculus.
    calculus: String,
    /// How many zones every sequent has.
    zones: usize,
    /// The terms.
    terms: Terms,
    /// The names of the rules the nodes use, in the order first used.
    pub(super) rules: Vec<String>,
    /// The index of each rule's name.
    rule_ids: HashMap<String, u32>,
    /// The parent of each scope; the root scope is its own.
    pub(super) scopes: Vec<u32>,
    /// The members of every zone of every node.
    members: Vec<TermId>,
    /// Per node and zone, where the zone ends among the members.
    ends: Vec<u32>,
    /// The premises of every node.
    premises: Vec<NodeId>,
    /// The positions of every node.
    positions: Vec<u32>,
    /// The nodes.
    pub(super) nodes: Vec<Entry>,
}

/// A node of a derivation, as [`Derivation::node`] shows it.
#[derive(Clone, Copy, Debug)]
pub struct NodeRef<'a> {
    /// The derivation.
    derivation: &'a Derivation,
    /// The node's index.
    index: usize,
}

impl<'a> NodeRef<'a> {
    /// Returns the node's id.
    pub const fn id(self) -> NodeId {
        NodeId(self.index as u32)
    }

    /// Returns the members of the zone at `zone` of the node's sequent.
    ///
    /// # Panics
    ///
    /// For a zone the derivation's sequents do not have.
    pub fn zone(self, zone: usize) -> &'a [TermId] {
        let derivation = self.derivation;
        assert!(zone < derivation.zones, "no such zone");
        let at = self.index * derivation.zones + zone;
        let start = at
            .checked_sub(1)
            .map_or(0, |before| derivation.ends[before]);
        &derivation.members[start as usize..derivation.ends[at] as usize]
    }

    /// Returns what justifies the node.
    pub fn step(self) -> Step<'a> {
        match self.derivation.nodes[self.index].kind {
            Kind::Rule(rule) => Step::Rule(&self.derivation.rules[rule as usize]),
            Kind::Hypothesis => Step::Hypothesis,
            Kind::Open => Step::Open,
        }
    }

    /// Returns the node's premises, in its rule's order.
    pub fn premises(self) -> &'a [NodeId] {
        let nodes = &self.derivation.nodes;
        let start = self
            .index
            .checked_sub(1)
            .map_or(0, |before| nodes[before].premises_end);
        &self.derivation.premises[start as usize..nodes[self.index].premises_end as usize]
    }

    /// Returns the positions the node gives its rule.
    pub fn positions(self) -> &'a [u32] {
        let nodes = &self.derivation.nodes;
        let start = self
            .index
            .checked_sub(1)
            .map_or(0, |before| nodes[before].positions_end);
        &self.derivation.positions[start as usize..nodes[self.index].positions_end as usize]
    }

    /// Returns the node's scope.
    pub fn scope(self) -> ScopeId {
        ScopeId(self.derivation.nodes[self.index].scope)
    }
}

impl Derivation {
    /// Returns a derivation without nodes in the calculus named
    /// `calculus`, whose sequents have `zones` zones.
    pub fn new(calculus: &str, zones: usize) -> Self {
        Self {
            calculus: calculus.to_owned(),
            zones,
            terms: Terms::default(),
            rules: Vec::new(),
            rule_ids: HashMap::default(),
            scopes: vec![0],
            members: Vec::new(),
            ends: Vec::new(),
            premises: Vec::new(),
            positions: Vec::new(),
            nodes: Vec::new(),
        }
    }

    /// Returns the name of the calculus the derivation is in.
    pub fn calculus(&self) -> &str {
        &self.calculus
    }

    /// Returns how many zones every sequent of the derivation has.
    pub const fn zones(&self) -> usize {
        self.zones
    }

    /// Returns the terms the nodes' sequents are made of.
    pub const fn terms(&self) -> &Terms {
        &self.terms
    }

    /// Returns the terms, to add to them. A term added changes no node.
    pub const fn terms_mut(&mut self) -> &mut Terms {
        &mut self.terms
    }

    /// Returns how many nodes the derivation has.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns whether the derivation has no node.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the node at `id`, which must be one of this derivation.
    ///
    /// # Panics
    ///
    /// For an id of another derivation, past this one's end.
    pub fn node(&self, id: NodeId) -> NodeRef<'_> {
        assert!(id.index() < self.nodes.len(), "no such node");
        NodeRef {
            derivation: self,
            index: id.index(),
        }
    }

    /// Returns the node at `index`, which is one of this derivation's.
    pub(super) fn at(&self, index: usize) -> NodeRef<'_> {
        debug_assert!(index < self.nodes.len(), "no such node");
        NodeRef {
            derivation: self,
            index,
        }
    }

    /// Returns every node, premises before conclusions.
    pub fn nodes(&self) -> impl ExactSizeIterator<Item = NodeRef<'_>> {
        (0..self.nodes.len()).map(|index| NodeRef {
            derivation: self,
            index,
        })
    }

    /// Returns the root, the last node, if the derivation has a node.
    pub fn root(&self) -> Option<NodeRef<'_>> {
        self.len().checked_sub(1).map(|index| self.at(index))
    }

    /// Returns the parent of `scope`, or `None` for the root scope.
    ///
    /// # Panics
    ///
    /// For a scope the derivation does not have.
    pub fn parent(&self, scope: ScopeId) -> Option<ScopeId> {
        (scope != ScopeId::ROOT).then(|| ScopeId(self.scopes[scope.index()]))
    }

    /// Returns the hypotheses of `scope`: the nodes a rule discharges
    /// when it takes a premise from that scope.
    pub fn hypotheses(&self, scope: ScopeId) -> impl Iterator<Item = NodeRef<'_>> {
        self.nodes()
            .filter(move |node| node.step() == Step::Hypothesis && node.scope() == scope)
    }

    /// Returns the derivation's assumptions: the hypotheses of the root
    /// scope, which no rule discharges.
    pub fn assumptions(&self) -> impl Iterator<Item = NodeRef<'_>> {
        self.hypotheses(ScopeId::ROOT)
    }

    /// Returns the open goals.
    pub fn open(&self) -> impl Iterator<Item = NodeRef<'_>> {
        self.nodes().filter(|node| node.step() == Step::Open)
    }

    /// Returns the parent of each scope, the root scope its own.
    pub(crate) fn scope_parents(&self) -> &[u32] {
        &self.scopes
    }

    /// Adds a scope inside the scope at `parent` and returns it.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for a parent the derivation does not
    /// have, and [`Refusal::Index`] past the most scopes.
    pub fn scope(&mut self, parent: u32) -> Result<ScopeId, Error> {
        if parent as usize >= self.scopes.len() {
            return Err(Error::IndexOutOfBounds {
                space: Space::Scope,
                index: parent as usize,
                len: self.scopes.len(),
            });
        }
        let id = fits(self.scopes.len(), Space::Scope)?;
        self.scopes.push(parent);
        Ok(ScopeId(id))
    }

    /// Adds a node and returns it. Its terms, premises and scope must
    /// exist already, so that a premise precedes its conclusion.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for a member that is no term, a
    /// premise that is no earlier node, or a scope the derivation does
    /// not have; [`Error::Zones`] for a sequent with another number of
    /// zones than the derivation's; [`Refusal::Index`] past what the
    /// derivation's indices can name.
    pub fn push(&mut self, node: Node<'_>) -> Result<NodeId, Error> {
        if node.sequent.len() != self.zones {
            return Err(Error::Zones {
                found: node.sequent.len(),
                zones: self.zones,
            });
        }
        let outside = |space, index: usize, len| Error::IndexOutOfBounds { space, index, len };
        for member in node.sequent.iter().flatten() {
            if member.index() >= self.terms.len() {
                return Err(outside(Space::Term, member.index(), self.terms.len()));
            }
        }
        if let Some(&premise) = node.premises.iter().find(|&&p| p as usize >= self.len()) {
            return Err(outside(Space::Node, premise as usize, self.len()));
        }
        if node.scope as usize >= self.scopes.len() {
            return Err(outside(
                Space::Scope,
                node.scope as usize,
                self.scopes.len(),
            ));
        }
        // Every vector stays within a `u32`, so no stored end wraps.
        let id = fits(self.nodes.len(), Space::Node)?;
        let members: usize = node.sequent.iter().map(Vec::len).sum();
        fits(self.members.len().saturating_add(members), Space::Member)?;
        let premises_end = fits(self.premises.len() + node.premises.len(), Space::Node)?;
        let positions_end = fits(self.positions.len() + node.positions.len(), Space::Member)?;
        let kind = match node.step {
            Step::Hypothesis => Kind::Hypothesis,
            Step::Open => Kind::Open,
            Step::Rule(name) => Kind::Rule(match self.rule_ids.get(name) {
                Some(&index) => index,
                None => {
                    // No more names than nodes.
                    let index = self.rules.len() as u32;
                    self.rules.push(name.to_owned());
                    self.rule_ids.insert(name.to_owned(), index);
                    index
                }
            }),
        };
        for zone in node.sequent {
            self.members.extend_from_slice(zone);
            self.ends.push(self.members.len() as u32);
        }
        self.premises
            .extend(node.premises.iter().map(|&p| NodeId(p)));
        self.positions.extend_from_slice(node.positions);
        self.nodes.push(Entry {
            kind,
            scope: node.scope,
            premises_end,
            positions_end,
        });
        Ok(NodeId(id))
    }
}

/// Returns `count` as an index, or the refusal for a count past what a
/// `u32` names.
fn fits(count: usize, what: Space) -> Result<u32, Error> {
    u32::try_from(count).map_err(|_| {
        Error::Refused(Refusal::Index {
            what,
            count: count as u64,
            most: u64::from(u32::MAX),
        })
    })
}
