// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The witnesses of a session's `∃` steps: read from text, built into its
//! table, and held to the eigenvariables in the goal's scope.

use super::{GoalId, Interactive, StepError, TermNode, Witness};
use crate::WitnessFault;
use crate::linear::occurrences::Member;
use crate::linear::occurrences::instances::eigen_names;
use crate::linear::proofs::Rule;
use crate::linear::proofs::derivation::InfId;
use crate::linear::proofs::multiset::Multiset;
use crate::linear::sequents::{Eigen, FoTerm, FoTermId, Symbol};

impl Interactive {
    /// Reads a witness from text, such as `f(a, g(b))`, for a [`Step`]'s
    /// [`with_witness`](super::Step::with_witness).
    ///
    /// The text is a closed term of the sequent's function symbols and
    /// constants, and of the session's eigenvariables by the letters the
    /// session writes them with. Whether its eigenvariables are in the
    /// goal's scope is for [`apply`](Self::apply) to decide.
    ///
    /// [`Step`]: super::Step
    ///
    /// # Errors
    ///
    /// [`StepError::Witness`] for text that is not such a term: an unknown
    /// name, a predicate, a symbol given the wrong number of arguments, or
    /// a stray bracket or comma.
    pub fn witness(&self, text: &str) -> Result<Witness, StepError> {
        let fault = |fault: WitnessFault| StepError::Witness {
            fault: Box::new(fault),
        };
        let fo = &self.forest.sequent().fo;
        let eigens = self
            .fo
            .as_ref()
            .map(|q| eigen_names(fo, &q.shown.eigenvariables()))
            .unwrap_or_default();
        let predicates = super::predicates(fo);
        // The tokens: names, brackets and commas.
        let mut tokens = Vec::new();
        let mut name = String::new();
        for c in text.chars().chain([' ']) {
            if c.is_whitespace() || matches!(c, '(' | ')' | ',') {
                if !name.is_empty() {
                    tokens.push(std::mem::take(&mut name));
                }
                if !c.is_whitespace() {
                    tokens.push(c.to_string());
                }
            } else {
                name.push(c);
            }
        }
        // A name is a node, applied to the terms in the brackets after it.
        // The open applications wait on a stack with their arguments.
        let mut nodes: Vec<TermNode> = Vec::new();
        let mut open: Vec<(Symbol, Vec<u32>)> = Vec::new();
        let mut done: Option<u32> = None;
        let mut tokens = tokens.into_iter().peekable();
        while let Some(token) = tokens.next() {
            match token.as_str() {
                "(" | ")" | "," if done.is_none() && token != "(" => {
                    return Err(fault(WitnessFault::Missing { before: token }));
                }
                "(" => return Err(fault(WitnessFault::Bracket)),
                "," | ")" => {
                    let (Some((_, args)), Some(arg)) = (open.last_mut(), done.take()) else {
                        return Err(fault(WitnessFault::Unopened { token }));
                    };
                    args.push(arg);
                    if token == ")"
                        && let Some((f, args)) = open.pop()
                    {
                        let arity = fo.symbols[f.index()].1 as usize;
                        if args.len() != arity {
                            return Err(fault(WitnessFault::Arity {
                                name: fo.symbols[f.index()].0.clone(),
                                arity: fo.symbols[f.index()].1,
                                given: Some(args.len()),
                            }));
                        }
                        done = Some(push(&mut nodes, TermNode::App(f, args)));
                    }
                }
                _ if done.is_some() => {
                    return Err(fault(WitnessFault::Comma { token }));
                }
                name => {
                    let applied = tokens.peek().is_some_and(|t| t == "(");
                    if let Some((&a, _)) = eigens.iter().find(|(_, n)| n.as_str() == name) {
                        if applied {
                            return Err(fault(WitnessFault::Applied { name: name.into() }));
                        }
                        done = Some(push(&mut nodes, TermNode::Eigen(a)));
                        continue;
                    }
                    let Some(f) = fo.symbols.iter().position(|(s, _)| s == name) else {
                        return Err(fault(WitnessFault::UnknownSymbol { name: name.into() }));
                    };
                    // The cast is exact: a sequent has fewer symbols than a
                    // `u32` counts.
                    let f = Symbol::new(f as u32);
                    if predicates.contains(&f) {
                        return Err(fault(WitnessFault::Predicate { name: name.into() }));
                    }
                    let arity = fo.symbols[f.index()].1;
                    match (applied, arity) {
                        (true, _) => {
                            tokens.next();
                            open.push((f, Vec::new()));
                        }
                        (false, 0) => done = Some(push(&mut nodes, TermNode::App(f, Vec::new()))),
                        (false, _) => {
                            return Err(fault(WitnessFault::Arity {
                                name: name.into(),
                                arity,
                                given: None,
                            }));
                        }
                    }
                }
            }
        }
        if !open.is_empty() {
            return Err(fault(WitnessFault::Unclosed));
        }
        if done.is_none() {
            return Err(fault(WitnessFault::Empty));
        }
        Ok(Witness::Term(nodes))
    }

    /// Builds a witness given flat into the table for a step on `goal`, and
    /// returns its term. The witness may use only the sequent's function
    /// symbols, with their arities, and the eigenvariables in the goal's
    /// scope.
    pub(super) fn witness_term(
        &mut self,
        goal: GoalId,
        nodes: &[TermNode],
    ) -> Result<FoTermId, StepError> {
        let fault = |fault: WitnessFault| StepError::Witness {
            fault: Box::new(fault),
        };
        if nodes.is_empty() {
            return Err(fault(WitnessFault::Empty));
        }
        let scope = self.scope(goal.inf());
        let fo = &self.forest.sequent().fo;
        let q = self.fo.as_mut().expect("a first-order session");
        let names = eigen_names(fo, &q.shown.eigenvariables());
        let predicates = super::predicates(fo);
        let mut built: Vec<FoTermId> = Vec::with_capacity(nodes.len());
        for (i, node) in nodes.iter().enumerate() {
            let term = match node {
                TermNode::Eigen(a) => {
                    if !scope.contains(a) {
                        return Err(StepError::Eigenvariable {
                            name: names
                                .get(a)
                                .cloned()
                                .unwrap_or_else(|| format!("#{}", a.get())),
                        });
                    }
                    q.eigen_term(*a)
                }
                TermNode::App(f, args) => {
                    let Some(&(ref name, arity)) = fo.symbols.get(f.index()) else {
                        return Err(fault(WitnessFault::SymbolIndex { symbol: f.get() }));
                    };
                    if predicates.contains(f) {
                        return Err(fault(WitnessFault::Predicate { name: name.clone() }));
                    }
                    if args.len() != arity as usize {
                        return Err(fault(WitnessFault::Arity {
                            name: name.clone(),
                            arity,
                            given: Some(args.len()),
                        }));
                    }
                    let mut items = Vec::with_capacity(args.len());
                    for &arg in args {
                        let Some(&t) = built.get(arg as usize).filter(|_| (arg as usize) < i)
                        else {
                            return Err(fault(WitnessFault::NodeOrder {
                                node: i,
                                names: arg,
                            }));
                        };
                        items.push(t);
                    }
                    q.apply(*f, &items)
                }
            };
            built.push(term);
        }
        Ok(*built.last().expect("not empty"))
    }

    /// Returns the eigenvariables in a goal's scope: those the `∀` steps
    /// below it introduced, and the constants.
    pub(super) fn scope(&self, goal: InfId) -> Vec<Eigen> {
        let mut parent = vec![None; self.inferences.len()];
        for (i, inference) in self.inferences.iter().enumerate() {
            for p in &inference.premises {
                parent[p.index()] = Some(InfId::new(i as u32));
            }
        }
        let mut scope = self
            .fo
            .as_ref()
            .map(|q| q.constants.clone())
            .unwrap_or_default();
        let mut at = parent[goal.index()];
        while let Some(id) = at {
            if self.inferences[id.index()].rule.rule == Rule::Forall
                && let Some(a) = self.introduced(id)
            {
                scope.push(a);
            }
            at = parent[id.index()];
        }
        scope
    }

    /// Returns the body member a closed `∀` or `∃` inference opened: the
    /// member its premise has that its conclusion, without the principal
    /// formula, lacks.
    pub(super) fn body(&self, id: InfId) -> Option<Member> {
        let inference = &self.inferences[id.index()];
        let premise = &self.inferences[inference.premises.first()?.index()].sequent;
        let mut rest = Multiset::of(inference.sequent.iter().copied());
        rest.remove(inference.sequent[inference.principal?]);
        Multiset::of(premise.iter().copied())
            .difference(&rest)
            .as_slice()
            .first()
            .copied()
    }

    /// Returns the term a closed `∀` or `∃` step on `goal` opened its
    /// formula with, as text, or `None` for a goal no quantifier step
    /// closed.
    ///
    /// The term is written as the step took it: a `∀` step's eigenvariable
    /// by its letter, an `∃` step's given witness, or `$1`, `$2`, … for a
    /// witness left open, whatever an axiom or the search filled it with
    /// later. [`witness`](Self::witness) reads a given witness back, to
    /// take the step again.
    pub fn opened_term(&self, goal: GoalId) -> Option<String> {
        let id = goal.inf();
        if !matches!(
            self.inferences.get(id.index())?.rule.rule,
            Rule::Forall | Rule::Exists
        ) {
            return None;
        }
        let q = self.fo.as_ref()?;
        let term = self.opened_with(id)?;
        let framing = crate::linear::occurrences::instances::Framing::new(
            &self.forest.sequent().fo,
            &q.table,
        );
        let mut text = String::new();
        framing
            .write_term(&mut text, term, &mut |out: &mut String, name: &str| {
                out.push_str(name);
                Ok(())
            })
            .ok()?;
        Some(text)
    }

    /// Returns the term a closed `∀` or `∃` inference put for its bound
    /// variable, as the table holds it: the innermost term of its body
    /// member's frame.
    pub(super) fn opened_with(&self, id: InfId) -> Option<FoTermId> {
        let q = self.fo.as_ref()?;
        let (_, frame) = q.instance(self.body(id)?);
        Some(q.table.frame_parts(frame?).1)
    }

    /// Returns the term a closed `∃` inference put for its bound variable,
    /// as the shown table numbers it, with its bindings followed.
    pub(super) fn shown_witness(&self, id: InfId) -> Option<FoTermId> {
        let q = self.fo.as_ref()?;
        let (_, frame) = q.instance(self.body(id)?);
        Some(q.shown.frame_parts(frame?).1)
    }

    /// Returns the member of the subformula `c` of the formula `m` stands
    /// for, under `m`'s frame.
    ///
    /// # Panics
    ///
    /// For a subformula whose instance the session never made. A step on
    /// `m` makes the instances its premises hold, so this never happens for
    /// a subformula in a premise.
    pub(super) fn child(&self, m: Member, c: crate::linear::occurrences::OccId) -> Member {
        match &self.fo {
            Some(q) => q
                .find(c, q.instance(m).1)
                .expect("a step made its premises' instances"),
            None => Member::from(c),
        }
    }

    /// Returns the eigenvariable a closed `∀` inference introduced.
    pub(super) fn introduced(&self, id: InfId) -> Option<Eigen> {
        let q = self.fo.as_ref()?;
        match q.terms(&self.forest).term(self.opened_with(id)?) {
            FoTerm::Eigen(a) => Some(a),
            FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => None,
        }
    }
}

/// Appends a node and returns its index. The index fits a `u32`: a witness
/// of more nodes would be more text than any reader holds.
fn push(nodes: &mut Vec<TermNode>, node: TermNode) -> u32 {
    nodes.push(node);
    u32::try_from(nodes.len() - 1).expect("fewer nodes than a u32 counts")
}
