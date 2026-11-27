// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use super::search::Why;
use super::sequents::{Atoms, Extension, FoExpression, Tables};
use crate::Error;
use crate::first_order::FoTermId;
use crate::limits::{Limits, Refusal, Space};
use crate::linear::proofs::InfId;
use crate::ordinary::{
    Derivation, Evidence, Formulas, Inference, Logic, Node, NodeId, Outcome, Rule, Search, Sequent,
    Side, Translation, Verdict, bound, ljt,
};
#[cfg(feature = "interactive")]
use crate::ordinary::{Interactive, Parts};
use crate::wire::{self, Readable};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for Rule {
    /// Serializes the rule as its name, such as `"→L"`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.name())
    }
}

impl<'a> Deserialize<'a> for Rule {
    /// Deserializes a rule from its name, refusing one no rule has.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let name = <std::borrow::Cow<'a, str>>::deserialize(deserializer)?;
        Rule::ALL
            .into_iter()
            .find(|rule| rule.name() == name)
            .ok_or_else(|| serde::de::Error::custom(format!("no rule of LK or LJ is named {name}")))
    }
}

/// The serialized form of a formula of ordinary or of modal logic: its
/// connective, with its operands as indices of formulas before it, or, as
/// a reader reads it, as the ids of the formulas they were read as. Its
/// tags are part of the interchange format. Each suite refuses the tags of
/// the other: `∀` and `∃` are ordinary logic's alone, `□` and `◇` modal
/// logic's.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub(super) enum FormulaForm<I = u32> {
    /// An atom, by its index among the names.
    #[serde(rename = "V")]
    Atom(u32),
    /// `⊤`
    #[serde(rename = "⊤")]
    True,
    /// `⊥`
    #[serde(rename = "⊥")]
    False,
    /// `¬A`
    #[serde(rename = "¬")]
    Not(I),
    /// `A ∧ B`
    #[serde(rename = "∧")]
    And(I, I),
    /// `A ∨ B`
    #[serde(rename = "∨")]
    Or(I, I),
    /// `A → B`
    #[serde(rename = "→")]
    Implies(I, I),
    /// `A ↔ B`
    #[serde(rename = "↔")]
    Iff(I, I),
    /// `∀x. A`
    #[serde(rename = "∀")]
    Forall(I),
    /// `∃x. A`
    #[serde(rename = "∃")]
    Exists(I),
    /// `□A`
    #[serde(rename = "□")]
    Box(I),
    /// `◇A`
    #[serde(rename = "◇")]
    Diamond(I),
}

impl FormulaForm {
    /// Returns the form with each operand `k` read as `at(k)`.
    fn map<I>(self, at: impl Fn(u32) -> Result<I, Error>) -> Result<FormulaForm<I>, Error> {
        Ok(match self {
            Self::Atom(a) => FormulaForm::Atom(a),
            Self::True => FormulaForm::True,
            Self::False => FormulaForm::False,
            Self::Not(a) => FormulaForm::Not(at(a)?),
            Self::And(a, b) => FormulaForm::And(at(a)?, at(b)?),
            Self::Or(a, b) => FormulaForm::Or(at(a)?, at(b)?),
            Self::Implies(a, b) => FormulaForm::Implies(at(a)?, at(b)?),
            Self::Iff(a, b) => FormulaForm::Iff(at(a)?, at(b)?),
            Self::Forall(a) => FormulaForm::Forall(at(a)?),
            Self::Exists(a) => FormulaForm::Exists(at(a)?),
            Self::Box(a) => FormulaForm::Box(at(a)?),
            Self::Diamond(a) => FormulaForm::Diamond(at(a)?),
        })
    }
}

/// Fails where a document holds more formulas than `limits.occurrences`,
/// before any is read.
pub(super) fn forms_within(forms: &[FormulaForm], limits: &Limits) -> Result<(), Error> {
    match limits.occurrences {
        Some(limit) if forms.len() as u64 > limit => Err(Error::Refused(Refusal::Occurrences {
            occurrences: forms.len() as u64,
            limit,
        })),
        _ => Ok(()),
    }
}

/// Returns the id of each of `forms`, made by `make` from the form with
/// its operands' ids; an operand must be the index of a form before it.
pub(super) fn read_forms<I: Copy>(
    forms: &[FormulaForm],
    mut make: impl FnMut(FormulaForm<I>) -> Result<I, Error>,
) -> Result<Vec<I>, Error> {
    let mut ids: Vec<I> = Vec::with_capacity(forms.len());
    for (n, form) in forms.iter().enumerate() {
        let form = form.map(|k| {
            ids.get(k as usize).copied().ok_or(Error::IndexOutOfBounds {
                space: Space::Formula,
                index: k as usize,
                len: n,
            })
        })?;
        ids.push(make(form)?);
    }
    Ok(ids)
}

/// Returns the ids of the indices given, each of a form of `ids`.
pub(super) fn ids_at<I: Copy>(ids: &[I], indices: &[u32]) -> Result<Vec<I>, Error> {
    indices
        .iter()
        .map(|&k| {
            ids.get(k as usize).copied().ok_or(Error::IndexOutOfBounds {
                space: Space::Formula,
                index: k as usize,
                len: ids.len(),
            })
        })
        .collect()
}

impl From<Node> for FormulaForm {
    /// Writes a node as its form. The arena's ids are the form's indices.
    fn from(node: Node) -> Self {
        let i = |k: NodeId| k.index() as u32;
        match node {
            Node::Atom(a) => Self::Atom(a),
            Node::True => Self::True,
            Node::False => Self::False,
            Node::Not(a) => Self::Not(i(a)),
            Node::And(a, b) => Self::And(i(a), i(b)),
            Node::Or(a, b) => Self::Or(i(a), i(b)),
            Node::Implies(a, b) => Self::Implies(i(a), i(b)),
            Node::Iff(a, b) => Self::Iff(i(a), i(b)),
            Node::Forall(a) => Self::Forall(i(a)),
            Node::Exists(a) => Self::Exists(i(a)),
        }
    }
}

/// The serialized form of an ordinary sequent: the formulas, the atom
/// names and the two sides, and for a first-order arena its symbols, its
/// terms, every atom as its predicate applied to terms, and the names of
/// its binders' variables, the linear sequent's keys.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct SequentForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The formulas, every operand before the formula built from it.
    formulas: Vec<FormulaForm>,
    /// The atoms, by name or, in a first-order arena, as predicates.
    atoms: Atoms,
    /// The symbols, each a name and its arity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    symbols: Vec<(String, u32)>,
    /// The terms, each after those it is built from.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    fo: Vec<FoExpression>,
    /// The name of every binder's variable, in the order of the binders
    /// among the formulas.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    binders: Vec<String>,
    /// The hypotheses, as indices of formulas, in the order written.
    left: Vec<u32>,
    /// The formulas right of `⊢`, as indices, in the order written.
    right: Vec<u32>,
}

impl SequentForm {
    /// Returns the form of the sequent `left ⊢ right` over the formulas.
    fn of(formulas: &Formulas, left: &[NodeId], right: &[NodeId]) -> Self {
        let ids = |list: &[NodeId]| list.iter().map(|k| k.index() as u32).collect();
        let tables = Tables::of_tables(formulas.first_order(), formulas.atom_names());
        // Every binder has its name in the tables, in the order of nodes.
        let binders = formulas
            .first_order()
            .binders
            .iter()
            .map(|(_, name)| name.clone())
            .collect();
        Self {
            version: None,
            formulas: formulas
                .nodes()
                .iter()
                .map(|&n| FormulaForm::from(n))
                .collect(),
            atoms: tables.atoms,
            symbols: tables.symbols,
            fo: tables.fo,
            binders,
            left: ids(left),
            right: ids(right),
        }
    }

    /// Returns the formulas the form holds, the id among them of every
    /// index of the form, and the two sides. Both the form's formulas and
    /// the sequent's unfolding must be within `limits.occurrences`.
    fn within(
        self,
        limits: &Limits,
        extension: Extension,
    ) -> Result<(Read, Vec<NodeId>, Vec<NodeId>), Error> {
        forms_within(&self.formulas, limits)?;
        let first_order = !self.symbols.is_empty()
            || !self.fo.is_empty()
            || matches!(self.atoms, Atoms::Mixed(_));
        let (mut formulas, indices, names) = if first_order {
            let tables = Tables {
                symbols: self.symbols,
                fo: self.fo,
                atoms: self.atoms,
            };
            let (tables, names) = tables.read::<NodeId>(limits, extension)?;
            match Formulas::of_tables(&tables)? {
                Some((formulas, indices, terms)) => (formulas, Some((indices, terms)), Vec::new()),
                None => (Formulas::default(), None, names),
            }
        } else {
            let Atoms::Names(names) = self.atoms else {
                unreachable!("a document of named atoms alone is propositional");
            };
            (Formulas::default(), None, names)
        };
        let mut binders = self.binders.iter().map(String::as_str);
        let ids = read_forms(&self.formulas, |form| {
            let node = match form {
                FormulaForm::Atom(a) => {
                    let len = indices.as_ref().map_or(names.len(), |(i, _)| i.len());
                    let outside = Error::IndexOutOfBounds {
                        space: Space::Atom,
                        index: a as usize,
                        len,
                    };
                    return match &indices {
                        Some((indices, _)) => {
                            let index = indices.get(a as usize).ok_or(outside)?;
                            formulas.add(Node::Atom(*index))
                        }
                        None => formulas.atom(names.get(a as usize).ok_or(outside)?),
                    };
                }
                FormulaForm::Forall(body) => {
                    return formulas.binder(true, body, binders.next().unwrap_or("x"));
                }
                FormulaForm::Exists(body) => {
                    return formulas.binder(false, body, binders.next().unwrap_or("x"));
                }
                FormulaForm::True => Node::True,
                FormulaForm::False => Node::False,
                FormulaForm::Not(a) => Node::Not(a),
                FormulaForm::And(a, b) => Node::And(a, b),
                FormulaForm::Or(a, b) => Node::Or(a, b),
                FormulaForm::Implies(a, b) => Node::Implies(a, b),
                FormulaForm::Iff(a, b) => Node::Iff(a, b),
                FormulaForm::Box(_) | FormulaForm::Diamond(_) => {
                    return Err(Error::Json {
                        form: "ordinary sequent",
                        message: "`□` and `◇` are modal logic's, which an ordinary sequent \
                                  does not hold"
                            .to_owned(),
                    });
                }
            };
            formulas.add(node)
        })?;
        let terms = indices.map(|(_, terms)| terms).unwrap_or_default();
        let read = Read {
            formulas,
            ids,
            terms,
        };
        let left = read.ids(&self.left)?;
        let right = read.ids(&self.right)?;
        bound(&read.formulas, &left, &right, limits)?;
        Ok((read, left, right))
    }
}

/// The formulas a form held, and the id of each of its indices.
struct Read {
    /// The formulas.
    formulas: Formulas,
    /// The id of each index of the form.
    ids: Vec<NodeId>,
    /// The id of each index of the form's terms.
    terms: Vec<FoTermId>,
}

impl Read {
    /// Returns the id of the term at `index` of the form's terms.
    fn term(&self, index: u32) -> Result<FoTermId, Error> {
        self.terms
            .get(index as usize)
            .copied()
            .ok_or(Error::IndexOutOfBounds {
                space: Space::Term,
                index: index as usize,
                len: self.terms.len(),
            })
    }

    /// Returns the ids of the indices given.
    fn ids(&self, indices: &[u32]) -> Result<Vec<NodeId>, Error> {
        ids_at(&self.ids, indices)
    }
}

impl Serialize for Sequent {
    /// Serializes the sequent as a document at level 8, the level of
    /// ordinary logic, or at level 25 where its arena is first-order.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = SequentForm::of(self.formulas(), self.left(), self.right());
        form.version = Some(terms(self.formulas(), wire::ORDINARY));
        form.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Sequent {
    /// Deserializes a sequent within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for Sequent {}

impl Readable for Sequent {
    const FORM: &'static str = "ordinary sequent";

    /// Reads a sequent, its formulas and sides within the limits.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let form = SequentForm::deserialize(deserializer)?;
        sequent(form, limits).map_err(wire::fail)
    }
}

/// Returns the sequent a form holds, within the limits.
fn sequent(form: SequentForm, limits: &Limits) -> Result<Sequent, Error> {
    let (read, left, right) = form.within(limits, Extension::Sequent)?;
    Sequent::new(read.formulas, left, right)
}

/// The serialized form of an inference: its two sides as indices of
/// formulas, its rule, the side and position of its principal formula,
/// and its premises. An open goal of a proof in progress is its sides
/// alone.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct InferenceForm {
    /// The hypotheses.
    left: Vec<u32>,
    /// The formulas right of `⊢`.
    right: Vec<u32>,
    /// The rule, absent for an open goal of a proof in progress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rule: Option<Rule>,
    /// The side and position of the principal formula, absent for an
    /// axiom and an open goal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    principal: Option<(Side, usize)>,
    /// The premises, as indices of inferences; absent for a leaf.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    premises: Vec<u32>,
    /// The term of a quantifier rule, as an index of the sequent's terms;
    /// absent for every other rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    term: Option<u32>,
}

impl InferenceForm {
    /// Returns the form of an inference. Where `open` is set, an open goal
    /// is written without its rule.
    fn of(inference: &Inference, open: bool) -> Self {
        let ids = |list: &[NodeId]| list.iter().map(|k| k.index() as u32).collect();
        Self {
            left: ids(&inference.left),
            right: ids(&inference.right),
            rule: (!(open && inference.rule == Rule::Open)).then_some(inference.rule),
            principal: inference.principal,
            premises: inference.premises.iter().map(|p| p.get()).collect(),
            term: inference.term.map(FoTermId::get),
        }
    }

    /// Returns the inference the form holds, its formulas by the ids that
    /// `read` gives. Refuses a premise at `len` or past it, and a
    /// principal position past its side.
    fn read(self, read: &Read, len: usize) -> Result<Inference, Error> {
        let premises = self
            .premises
            .iter()
            .map(|&p| match p as usize {
                p if p < len => Ok(InfId::new(p as u32)),
                index => Err(Error::IndexOutOfBounds {
                    space: Space::Inference,
                    index,
                    len,
                }),
            })
            .collect::<Result<_, _>>()?;
        let inference = Inference {
            term: self.term.map(|t| read.term(t)).transpose()?,
            left: read.ids(&self.left)?,
            right: read.ids(&self.right)?,
            rule: self.rule.unwrap_or(Rule::Open),
            principal: self.principal,
            premises,
        };
        if let Some((side, at)) = inference.principal
            && at >= inference.side(side).len()
        {
            return Err(Error::IndexOutOfBounds {
                space: Space::Formula,
                index: at,
                len: inference.side(side).len(),
            });
        }
        Ok(inference)
    }
}

/// The serialized form of a derivation of LK or LJ: the logic, the
/// sequent with every formula the inferences name, and the inferences.
/// Premises come before their conclusions, and the root is last.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DerivationForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The logic.
    logic: Logic,
    /// The sequent derived.
    sequent: SequentForm,
    /// The inferences.
    inferences: Vec<InferenceForm>,
}

impl From<&Derivation> for DerivationForm {
    /// Writes the derivation's form, every rule named.
    fn from(derivation: &Derivation) -> Self {
        let (left, right) = derivation.sequent();
        Self {
            version: None,
            logic: derivation.logic(),
            sequent: SequentForm::of(derivation.formulas(), left, right),
            inferences: derivation
                .inferences()
                .iter()
                .map(|i| InferenceForm::of(i, false))
                .collect(),
        }
    }
}

impl DerivationForm {
    /// Returns the derivation the form holds, its sequent within the
    /// limits. Refuses a form without the shape every derivation has:
    /// - at least one inference;
    /// - every premise before its conclusion, and the premise of exactly
    ///   one inference;
    /// - the root last.
    ///
    /// Whether it is a derivation of its logic is for
    /// [`Derivation::check`] to say.
    fn within(self, limits: &Limits) -> Result<Derivation, Error> {
        let (read, left, right) = self.sequent.within(limits, Extension::Derivation)?;
        let len = self.inferences.len();
        let mut parents = vec![0u8; len];
        let mut inferences = Vec::with_capacity(len);
        for (n, form) in self.inferences.into_iter().enumerate() {
            let inference = form.read(&read, n)?;
            // An open goal is a leaf with no principal formula, as the
            // drawings draw it.
            if inference.rule == Rule::Open
                && (inference.principal.is_some() || !inference.premises.is_empty())
            {
                return Err(Error::Json {
                    form: "ordinary derivation",
                    message: format!("inference {n} is an open goal with a principal or premises"),
                });
            }
            for p in &inference.premises {
                parents[p.index()] = parents[p.index()].saturating_add(1);
            }
            inferences.push(inference);
        }
        let shaped = parents
            .split_last()
            .is_some_and(|(&root, rest)| root == 0 && rest.iter().all(|&n| n == 1));
        if !shaped {
            return Err(Error::Json {
                form: "ordinary derivation",
                message: "the inferences are no tree with its root last".to_owned(),
            });
        }
        // Every formula of every inference is closed, as a sequent's are.
        if read.formulas.is_first_order() {
            crate::ordinary::closed(
                &read.formulas,
                inferences
                    .iter()
                    .flat_map(|i| i.left.iter().chain(&i.right))
                    .chain(left.iter().chain(&right)),
            )?;
        }
        Ok(
            Derivation::from_parts(self.logic, read.formulas, (left, right), inferences)
                .untrusted(),
        )
    }
}

impl Serialize for Derivation {
    /// Serializes the derivation as a document: at level 25 where its
    /// arena is first-order, else at level 9 where an inference is a cut,
    /// else at level 8.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = DerivationForm::from(self);
        form.version = Some(terms(self.formulas(), level(self.inferences())));
        form.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Derivation {
    /// Deserializes a derivation within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for Derivation {}

impl Readable for Derivation {
    const FORM: &'static str = "ordinary derivation";

    /// Reads a derivation that has the shape every derivation has, its
    /// sequent within the limits.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let form = DerivationForm::deserialize(deserializer)?;
        form.within(limits).map_err(wire::fail)
    }
}

/// The serialized form of the outcome of deciding an ordinary sequent:
/// the verdict, the logic, the search that decided it, and the evidence
/// of a valid sequent.
#[derive(Serialize)]
struct OutcomeForm<'a> {
    /// The wire level, written only at the top of a document.
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<u32>,
    /// The crate's version, which wrote the outcome, written only at the
    /// top of a document.
    #[serde(skip_serializing_if = "Option::is_none")]
    deduxit: Option<&'static str>,
    /// `valid`, `not_valid` or `unknown`.
    verdict: &'static str,
    /// Why the search could not decide.
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<Why>,
    /// The logic decided.
    logic: Logic,
    /// How the sequent was searched, tagged by its method.
    search: SearchForm<'a>,
    /// The derivation, for `valid` by a derivation.
    #[serde(skip_serializing_if = "Option::is_none")]
    derivation: Option<DerivationForm>,
    /// The clausal proof, for `valid` by one.
    #[cfg(feature = "sat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    clausal_proof: Option<&'a crate::ordinary::sat::ClausalProof>,
    /// The Kripke model that refutes the sequent, for `not_valid` by one.
    #[serde(skip_serializing_if = "Option::is_none")]
    countermodel: Option<CountermodelForm>,
    /// The finite structure that refutes a first-order sequent, for
    /// `not_valid` by grounding.
    #[serde(skip_serializing_if = "Option::is_none")]
    structure: Option<StructureForm>,
    /// The Kripke model with growing domains that refutes a first-order
    /// sequent, for `not_valid` by the model search.
    #[serde(skip_serializing_if = "Option::is_none")]
    kripke_structure: Option<KripkeStructureForm>,
}

/// The serialized form of a Kripke countermodel with growing domains: the
/// sequent refuted and the model.
#[derive(Serialize, Deserialize)]
pub(super) struct KripkeStructureForm {
    /// The sequent refuted.
    sequent: SequentForm,
    /// The model.
    model: crate::ordinary::KripkeStructure,
}

impl KripkeStructureForm {
    /// Returns the written form of a Kripke countermodel with growing
    /// domains.
    fn of(c: &crate::ordinary::KripkeCountermodel) -> Self {
        Self {
            sequent: SequentForm::of(c.sequent.formulas(), c.sequent.left(), c.sequent.right()),
            model: c.model.clone(),
        }
    }
}

/// The serialized form of a countermodel: the sequent refuted and the
/// model.
#[derive(Serialize, Deserialize)]
pub(super) struct CountermodelForm {
    /// The sequent refuted.
    sequent: SequentForm,
    /// The model.
    model: super::kripke::ModelForm,
}

impl CountermodelForm {
    /// Returns the written form of a countermodel.
    pub(super) fn of(c: &crate::ordinary::Countermodel) -> Self {
        Self {
            sequent: SequentForm::of(c.sequent.formulas(), c.sequent.left(), c.sequent.right()),
            model: super::kripke::ModelForm::from(&c.model),
        }
    }
}

/// The serialized form of a finite countermodel: the sequent refuted and
/// the structure.
#[derive(Serialize, Deserialize)]
pub(super) struct StructureForm {
    /// The sequent refuted.
    sequent: SequentForm,
    /// The structure.
    structure: crate::ordinary::Structure,
}

impl StructureForm {
    /// Returns the written form of a finite countermodel.
    fn of(c: &crate::ordinary::FiniteCountermodel) -> Self {
        Self {
            sequent: SequentForm::of(c.sequent.formulas(), c.sequent.left(), c.sequent.right()),
            structure: c.structure.clone(),
        }
    }
}

/// How an ordinary sequent was searched, as an outcome writes it: an
/// object tagged by `method`, the method's name.
#[derive(Serialize)]
#[serde(tag = "method", rename_all = "snake_case")]
enum SearchForm<'a> {
    /// Through the embedding: the translation and the image's outcome.
    Embedding {
        /// The translation the image was made by.
        translation: Translation,
        /// The outcome of the image's search, with its proof or disproof.
        image: super::search::Nested<'a>,
    },
    /// By LJT, and what its search cost.
    Ljt {
        /// LJT's statistics.
        statistics: &'a ljt::Statistics,
    },
    /// By the SAT solver, and what its search cost.
    #[cfg(feature = "sat")]
    Sat {
        /// The SAT solver's statistics.
        statistics: &'a crate::ordinary::sat::Statistics,
    },
    /// By the truth table.
    Table {
        /// The table.
        table: TableForm,
    },
    /// By a tableau.
    #[cfg(feature = "interactive")]
    Tableau {
        /// The tableau.
        tableau: TableauForm,
    },
    /// By the connection prover, and what its search cost.
    Connection {
        /// The prover's statistics.
        statistics: &'a crate::ordinary::connection::Statistics,
    },
    /// By grounding, and what it cost.
    #[cfg(feature = "sat")]
    Grounding {
        /// Grounding's statistics.
        statistics: &'a crate::ordinary::grounding::Statistics,
    },
    /// By the finite model search, and what it cost.
    #[cfg(feature = "sat")]
    Models {
        /// The search's statistics.
        statistics: &'a crate::ordinary::models::Statistics,
    },
}

impl Serialize for Outcome {
    /// Serializes the outcome as a document at level 12: its verdict, with
    /// the reason of an unknown one and the evidence of a valid sequent,
    /// and the search that decided it, the image's outcome nested under
    /// the embedding.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        outcome_form(self).serialize(serializer)
    }
}

/// An ordinary outcome inside another document: its keys without
/// `version` and `deduxit`.
struct NestedOutcome<'a>(&'a Outcome);

impl Serialize for NestedOutcome<'_> {
    /// Serializes the outcome's keys but `version` and `deduxit`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = outcome_form(self.0);
        form.version = None;
        form.deduxit = None;
        form.serialize(serializer)
    }
}

/// Returns the form of an outcome as a document of its own.
fn outcome_form(outcome: &Outcome) -> OutcomeForm<'_> {
    let (verdict, reason, derivation) = match &outcome.verdict {
        Verdict::Valid(Evidence::Derivation(d)) => {
            ("valid", None, Some(DerivationForm::from(&**d)))
        }
        #[cfg(feature = "sat")]
        Verdict::Valid(Evidence::Clausal(_)) => ("valid", None, None),
        Verdict::Valid(Evidence::Image | Evidence::Table) => ("valid", None, None),
        Verdict::NotValid => ("not_valid", None, None),
        Verdict::Unknown(r) => ("unknown", Some(Why::from(*r)), None),
    };
    let search = match &outcome.search {
        Search::Embedding {
            translation,
            linear,
        } => SearchForm::Embedding {
            translation: *translation,
            image: super::search::Nested(linear),
        },
        Search::Ljt(statistics) => SearchForm::Ljt { statistics },
        #[cfg(feature = "sat")]
        Search::Sat(statistics) => SearchForm::Sat { statistics },
        Search::Table(table) => SearchForm::Table {
            table: TableForm::from(&**table),
        },
        #[cfg(feature = "interactive")]
        Search::Tableau(tableau) => SearchForm::Tableau {
            tableau: TableauForm::from(&**tableau),
        },
        Search::Connection(statistics) => SearchForm::Connection { statistics },
        #[cfg(feature = "sat")]
        Search::Grounding(statistics) => SearchForm::Grounding { statistics },
        #[cfg(feature = "sat")]
        Search::Models(statistics) => SearchForm::Models { statistics },
    };
    // A reader of level 12 does not know the methods `table` and
    // `tableau`, so an outcome of either is at level 14; the method
    // `connection` is first read at level 25.
    let version = match &outcome.search {
        Search::Table(_) => wire::TABLES,
        #[cfg(feature = "interactive")]
        Search::Tableau(_) => wire::TABLES,
        Search::Connection(_) => wire::ORDINARY_TERMS,
        #[cfg(feature = "sat")]
        Search::Grounding(_) => wire::ORDINARY_TERMS,
        #[cfg(feature = "sat")]
        Search::Models(_) => wire::MODELS,
        Search::Embedding { .. } | Search::Ljt(_) => wire::SEARCH,
        #[cfg(feature = "sat")]
        Search::Sat(_) => wire::SEARCH,
    };
    // A first-order derivation, and the unknown a first-order sequent's
    // unprovable image is, are first read at level 25.
    let first_order = match &outcome.verdict {
        Verdict::Valid(Evidence::Derivation(d)) => d.formulas().is_first_order(),
        Verdict::Unknown(reason) => matches!(
            reason,
            crate::linear::search::Reason::Incomplete
                | crate::linear::search::Reason::PathLength { .. }
        ),
        #[cfg(feature = "sat")]
        Verdict::Valid(Evidence::Clausal(_)) => false,
        Verdict::Valid(Evidence::Image | Evidence::Table) | Verdict::NotValid => false,
    };
    let version = if first_order {
        version.max(wire::ORDINARY_TERMS)
    } else {
        version
    };
    // A structure with function tables is read from level 26 on: a reader
    // of 25 refuses the key.
    let version = match &outcome.structure {
        Some(found) if !found.structure.functions.is_empty() => version.max(wire::MODELS),
        Some(_) | None => version,
    };
    // So is the model search's unknown.
    let version = if matches!(
        outcome.verdict,
        Verdict::Unknown(crate::linear::search::Reason::ModelSize { .. })
    ) {
        version.max(wire::MODELS)
    } else {
        version
    };
    // A Kripke model with growing domains, the bound on its worlds, and
    // the statistics of a search that tried one are read from level 27 on.
    #[cfg(feature = "sat")]
    let tried_kripke =
        matches!(&outcome.search, Search::Models(statistics) if statistics.worlds > 0);
    #[cfg(not(feature = "sat"))]
    let tried_kripke = false;
    let version = if outcome.kripke.is_some()
        || tried_kripke
        || matches!(
            outcome.verdict,
            Verdict::Unknown(crate::linear::search::Reason::KripkeSize { .. })
        ) {
        version.max(wire::KRIPKE_STRUCTURES)
    } else {
        version
    };
    let countermodel = outcome.countermodel.as_ref().map(CountermodelForm::of);
    // A reader of level 12 may ignore the countermodel without
    // misreading the verdict, so it raises no level.
    OutcomeForm {
        version: Some(version),
        deduxit: Some(env!("CARGO_PKG_VERSION")),
        verdict,
        reason,
        logic: outcome.logic,
        search,
        derivation,
        #[cfg(feature = "sat")]
        clausal_proof: match &outcome.verdict {
            Verdict::Valid(Evidence::Clausal(proof)) => Some(proof),
            Verdict::Valid(Evidence::Derivation(_) | Evidence::Image | Evidence::Table)
            | Verdict::NotValid
            | Verdict::Unknown(_) => None,
        },
        countermodel,
        structure: outcome.structure.as_deref().map(StructureForm::of),
        kripke_structure: outcome.kripke.as_deref().map(KripkeStructureForm::of),
    }
}

/// The verdict of an outcome of ordinary or modal logic, by its word.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum VerdictWord {
    /// `valid`.
    Valid,
    /// `not_valid`.
    NotValid,
    /// `unknown`.
    Unknown,
}

/// How an ordinary sequent was searched, as an outcome is read.
#[derive(Deserialize)]
#[serde(tag = "method", rename_all = "snake_case")]
enum SearchRead {
    /// Through the embedding.
    Embedding {
        /// The translation the image was made by.
        translation: Translation,
        /// The image's outcome.
        image: Box<super::search::OutcomeForm>,
    },
    /// By LJT.
    Ljt {
        /// LJT's statistics.
        statistics: ljt::Statistics,
    },
    /// By the SAT solver.
    #[cfg(feature = "sat")]
    Sat {
        /// The SAT solver's statistics.
        statistics: crate::ordinary::sat::Statistics,
    },
    /// By the truth table.
    Table {
        /// The table.
        table: TableForm,
    },
    /// By a tableau.
    #[cfg(feature = "interactive")]
    Tableau {
        /// The tableau.
        tableau: TableauForm,
    },
    /// By the connection prover.
    Connection {
        /// The prover's statistics.
        statistics: crate::ordinary::connection::Statistics,
    },
    /// By grounding.
    #[cfg(feature = "sat")]
    Grounding {
        /// Grounding's statistics.
        statistics: crate::ordinary::grounding::Statistics,
    },
    /// By the finite model search.
    #[cfg(feature = "sat")]
    Models {
        /// The search's statistics.
        statistics: crate::ordinary::models::Statistics,
    },
}

/// The keys an ordinary outcome is read from. `deduxit`, the writer's
/// version, is not read.
#[derive(Deserialize)]
struct OutcomeRead {
    /// The wire level, read to refuse one above this build's.
    #[serde(default, deserialize_with = "wire::version")]
    version: Option<u32>,
    /// `valid`, `not_valid` or `unknown`.
    verdict: VerdictWord,
    /// Why the search could not decide, for `unknown`.
    #[serde(default)]
    reason: Option<Why>,
    /// The logic decided.
    logic: Logic,
    /// How the sequent was searched; absent below level 12.
    #[serde(default)]
    search: Option<SearchRead>,
    /// The derivation, for `valid` by a derivation.
    #[serde(default)]
    derivation: Option<DerivationForm>,
    /// The clausal proof's counts, for `valid` by one.
    #[serde(default)]
    clausal_proof: Option<serde::de::IgnoredAny>,
    /// The Kripke model that refutes the sequent, for `not_valid`.
    #[serde(default)]
    countermodel: Option<CountermodelForm>,
    /// The finite structure that refutes a first-order sequent, for
    /// `not_valid`.
    #[serde(default)]
    structure: Option<StructureForm>,
    /// The Kripke model with growing domains that refutes a first-order
    /// sequent, for `not_valid`.
    #[serde(default)]
    kripke_structure: Option<KripkeStructureForm>,
}

impl OutcomeRead {
    /// Returns the outcome the form holds, its derivation and the image's
    /// proof or disproof read within `limits`, neither checked. Refuses an
    /// outcome below level 12, which has no `search`; one valid by a
    /// clausal proof, whose document holds the proof's counts alone; and
    /// one whose verdict the image's outcome does not bear out.
    fn within(self, limits: &Limits) -> Result<Outcome, Error> {
        let refuse = |message: &str| Error::Json {
            form: "ordinary outcome",
            message: message.to_owned(),
        };
        let search = match self.search {
            Some(search) => search,
            None if self.version.unwrap_or(wire::BASE) < wire::SEARCH => {
                return Err(refuse(
                    "an ordinary outcome below level 12 has no `search` and is not read back",
                ));
            }
            None => return Err(refuse("missing field `search`")),
        };
        let search = match search {
            SearchRead::Embedding { translation, image } => Search::Embedding {
                translation,
                linear: Box::new(image.within(limits)?),
            },
            SearchRead::Ljt { statistics } => Search::Ljt(statistics),
            #[cfg(feature = "sat")]
            SearchRead::Sat { statistics } => Search::Sat(statistics),
            SearchRead::Table { table } => Search::Table(Box::new(table.within(limits)?)),
            #[cfg(feature = "interactive")]
            SearchRead::Tableau { tableau } => Search::Tableau(Box::new(tableau.within(limits)?)),
            SearchRead::Connection { statistics } => Search::Connection(statistics),
            #[cfg(feature = "sat")]
            SearchRead::Grounding { statistics } => Search::Grounding(statistics),
            #[cfg(feature = "sat")]
            SearchRead::Models { statistics } => Search::Models(statistics),
        };
        let image = match &search {
            Search::Embedding { linear, .. } => Some(&linear.verdict),
            #[cfg(feature = "sat")]
            Search::Sat(_) => None,
            Search::Ljt(_) | Search::Table(_) | Search::Connection(_) => None,
            #[cfg(feature = "sat")]
            Search::Grounding(_) | Search::Models(_) => None,
            #[cfg(feature = "interactive")]
            Search::Tableau(_) => None,
        };
        use crate::linear::search::Verdict as Linear;
        // An unprovable image of a first-order sequent leaves it unknown.
        let incomplete = matches!(self.reason, Some(Why::Incomplete));
        let borne = match (&self.verdict, image) {
            (_, None)
            | (VerdictWord::Valid, Some(Linear::Proved(_)))
            | (VerdictWord::NotValid, Some(Linear::Unprovable(_)))
            | (VerdictWord::Unknown, Some(Linear::Unknown(_))) => true,
            (VerdictWord::Unknown, Some(Linear::Unprovable(_))) => incomplete,
            (_, Some(_)) => false,
        };
        if !borne {
            return Err(refuse("the verdict is not the image's outcome's"));
        }
        // A table or a tableau bears its verdict out itself.
        let shown = match (&self.verdict, &search) {
            (VerdictWord::Valid, Search::Table(table)) => table.falsifying().is_empty(),
            (VerdictWord::NotValid, Search::Table(table)) => !table.falsifying().is_empty(),
            #[cfg(feature = "interactive")]
            (VerdictWord::Valid, Search::Tableau(tableau)) => tableau.is_closed(),
            #[cfg(feature = "interactive")]
            (VerdictWord::NotValid, Search::Tableau(tableau)) => !tableau.is_closed(),
            _ => true,
        };
        if !shown {
            return Err(refuse("the verdict is not its table's or its tableau's"));
        }
        let verdict = match self.verdict {
            VerdictWord::Valid if self.clausal_proof.is_some() => {
                return Err(refuse(
                    "a clausal proof is written as its counts, which hold no proof to read back",
                ));
            }
            VerdictWord::Valid => match self.derivation {
                Some(form) => Verdict::Valid(Evidence::Derivation(Box::new(form.within(limits)?))),
                None if image.is_some() => Verdict::Valid(Evidence::Image),
                None if matches!(search, Search::Table(_)) => Verdict::Valid(Evidence::Table),
                None => return Err(refuse("missing field `derivation`")),
            },
            VerdictWord::NotValid => Verdict::NotValid,
            VerdictWord::Unknown => match self.reason {
                Some(reason) => Verdict::Unknown(reason.into()),
                None => return Err(refuse("missing field `reason`")),
            },
        };
        let countermodel = match (self.countermodel, &verdict) {
            (None, _) => None,
            (Some(form), Verdict::NotValid) => Some(crate::ordinary::Countermodel::new(
                sequent(form.sequent, limits)?,
                form.model.within(limits)?,
            )),
            (Some(_), _) => {
                return Err(refuse(
                    "a countermodel beside a verdict other than not_valid",
                ));
            }
        };
        let structure = match (self.structure, &verdict) {
            (None, _) => None,
            (Some(form), Verdict::NotValid) => {
                Some(Box::new(crate::ordinary::FiniteCountermodel::new(
                    sequent(form.sequent, limits)?,
                    form.structure,
                )))
            }
            (Some(_), _) => {
                return Err(refuse("a structure beside a verdict other than not_valid"));
            }
        };
        let kripke = match (self.kripke_structure, &verdict) {
            (None, _) => None,
            (Some(form), Verdict::NotValid) => {
                Some(Box::new(crate::ordinary::KripkeCountermodel::new(
                    sequent(form.sequent, limits)?,
                    form.model,
                )))
            }
            (Some(_), _) => {
                return Err(refuse(
                    "a Kripke structure beside a verdict other than not_valid",
                ));
            }
        };
        Ok(Outcome {
            verdict,
            logic: self.logic,
            search,
            countermodel,
            structure,
            kripke,
        })
    }
}

impl<'a> Deserialize<'a> for Outcome {
    /// Deserializes an ordinary outcome within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for Outcome {}

impl Readable for Outcome {
    const FORM: &'static str = "ordinary outcome";

    /// Reads an ordinary outcome: its derivation and the image's proof or
    /// disproof as their own readers read them, neither checked.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        OutcomeRead::deserialize(deserializer)?
            .within(limits)
            .map_err(wire::fail)
    }
}

/// The serialized form of a proof in progress of an ordinary sequent: the
/// logic, the sequent with every formula the inferences name, the
/// inferences with the root first, and the inferences the steps closed.
#[cfg(feature = "interactive")]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct SessionForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The logic.
    logic: Logic,
    /// The sequent being proved.
    sequent: SequentForm,
    /// The inferences.
    inferences: Vec<InferenceForm>,
    /// The steps, each the inference it closed.
    history: Vec<u32>,
    /// The bindings of open witnesses in force, in the order made: the
    /// witness's number, its term and the step whose axiom bound it.
    /// Written only where there is one.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    bindings: Vec<[u32; 3]>,
}

#[cfg(feature = "interactive")]
impl Serialize for Interactive {
    /// Serializes the proof in progress as a document: at level 25 where
    /// its arena is first-order, else at level 9 where an inference is a
    /// cut, else at level 8.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (logic, formulas, inferences, history, bindings) = self.parts();
        let (left, right) = self.sequent();
        SessionForm {
            version: Some(terms(formulas, level(&inferences))),
            logic,
            sequent: SequentForm::of(formulas, left, right),
            inferences: inferences
                .iter()
                .map(|i| InferenceForm::of(i, true))
                .collect(),
            history: history.iter().map(|h| h.get()).collect(),
            bindings: bindings
                .into_iter()
                .map(|(k, t, step)| [k, t.get(), step])
                .collect(),
        }
        .serialize(serializer)
    }
}

#[cfg(feature = "interactive")]
impl<'a> Deserialize<'a> for Interactive {
    /// Deserializes a proof in progress within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

#[cfg(feature = "interactive")]
impl crate::sealed::Readable for Interactive {}

#[cfg(feature = "interactive")]
impl Readable for Interactive {
    const FORM: &'static str = "ordinary session";

    /// Reads a proof in progress, its sequent within the limits, and
    /// checks it as one built step by step is checked.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let form = SessionForm::deserialize(deserializer)?;
        session(form, limits).map_err(wire::fail)
    }
}

/// Returns the proof in progress a form holds, within the limits.
#[cfg(feature = "interactive")]
fn session(form: SessionForm, limits: &Limits) -> Result<Interactive, Error> {
    // A session's terms are a derivation's and its open witnesses.
    let (read, left, right) = form.sequent.within(limits, Extension::Session)?;
    let len = form.inferences.len();
    let inferences = form
        .inferences
        .into_iter()
        .map(|i| i.read(&read, len))
        .collect::<Result<Vec<_>, _>>()?;
    // Every formula of every inference is closed, as a sequent's are.
    if read.formulas.is_first_order() {
        crate::ordinary::closed(
            &read.formulas,
            inferences
                .iter()
                .flat_map(|i| i.left.iter().chain(&i.right)),
        )?;
    }
    if inferences
        .first()
        .is_some_and(|root| root.left != left || root.right != right)
    {
        return Err(Error::InconsistentSession {
            reason: "the root does not conclude the sequent",
        });
    }
    let history = form
        .history
        .iter()
        .map(|&h| match h as usize {
            h if h < len => Ok(InfId::new(h as u32)),
            index => Err(Error::IndexOutOfBounds {
                space: Space::Inference,
                index,
                len,
            }),
        })
        .collect::<Result<_, _>>()?;
    let bindings = form
        .bindings
        .iter()
        .map(|&[k, t, step]| Ok((k, read.term(t)?, step)))
        .collect::<Result<_, Error>>()?;
    Interactive::from_parts(
        Parts {
            logic: form.logic,
            formulas: read.formulas,
            inferences,
            history,
            bindings,
        },
        limits,
    )
}

/// Returns `level`, or the level of a first-order form where `formulas`
/// is first-order.
fn terms(formulas: &Formulas, level: u32) -> u32 {
    if formulas.is_first_order() {
        level.max(wire::ORDINARY_TERMS)
    } else {
        level
    }
}

/// Returns the level of a form of ordinary logic with these inferences:
/// 9 where one is a cut, else 8, the level of ordinary logic.
fn level(inferences: &[Inference]) -> u32 {
    if inferences.iter().any(|i| i.rule() == Rule::Cut) {
        wire::ORDINARY_CUT
    } else {
        wire::ORDINARY
    }
}

/// The serialized form of a truth table: the sequent, a row per
/// assignment as a `T` or `F` per formula of the sequent's form, and the
/// rows that falsify the sequent.
#[derive(Serialize, Deserialize)]
struct TableForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The sequent.
    sequent: SequentForm,
    /// The rows, from every atom true to every atom false.
    rows: Vec<String>,
    /// The rows that falsify the sequent.
    falsifying: Vec<u64>,
}

impl From<&crate::ordinary::TruthTable> for TableForm {
    /// Writes the table's form, nested: without its level.
    fn from(table: &crate::ordinary::TruthTable) -> Self {
        let sequent = table.sequent();
        Self {
            version: None,
            sequent: SequentForm::of(sequent.formulas(), sequent.left(), sequent.right()),
            rows: (0..table.rows())
                .map(|row| {
                    table
                        .row(row)
                        .map(|value| if value { 'T' } else { 'F' })
                        .collect()
                })
                .collect(),
            falsifying: table.falsifying().to_vec(),
        }
    }
}

impl TableForm {
    /// Returns the table of the form's sequent, computed again within
    /// `limits`; refuses rows or falsifying rows that are not that
    /// table's.
    fn within(self, limits: &Limits) -> Result<crate::ordinary::TruthTable, Error> {
        let (read, left, right) = self.sequent.within(limits, Extension::Sequent)?;
        let ids = read.ids.clone();
        let sequent = Sequent::new(read.formulas, left, right)?;
        let table = crate::ordinary::truth_table(&sequent, limits, |_| false)?;
        let matches = self.rows.len() as u64 == table.rows()
            && self.rows.iter().enumerate().all(|(row, text)| {
                text.chars().count() == ids.len()
                    && text.chars().zip(&ids).all(|(c, &id)| {
                        (c == 'T') == table.value(row as u64, id) && "TF".contains(c)
                    })
            })
            && self.falsifying == table.falsifying();
        if !matches {
            return Err(Error::Json {
                form: "truth table",
                message: "the rows are not the sequent's truth table".to_owned(),
            });
        }
        Ok(table)
    }
}

impl Serialize for crate::ordinary::TruthTable {
    /// Serializes the table as a document at level 14.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = TableForm::from(self);
        form.version = Some(wire::TABLES);
        form.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for crate::ordinary::TruthTable {
    /// Deserializes a table within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for crate::ordinary::TruthTable {}

impl Readable for crate::ordinary::TruthTable {
    const FORM: &'static str = "truth table";

    /// Reads a truth table, computed again from its sequent.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        TableForm::deserialize(deserializer)?
            .within(limits)
            .map_err(wire::fail)
    }
}

/// A sign as a tableau's form writes it.
#[cfg(feature = "interactive")]
#[derive(Clone, Copy, Serialize, Deserialize)]
enum SignForm {
    /// True.
    T,
    /// False.
    F,
}

/// How a branch ends, as a tableau's form writes it, tagged by `kind`.
#[cfg(feature = "interactive")]
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum EndForm {
    /// Closed by a formula true at one line and false at another.
    Closed {
        /// The line where it is true.
        true_line: u32,
        /// The line where it is false.
        false_line: u32,
    },
    /// Closed by `⊥` true.
    Falsity {
        /// The line.
        line: u32,
    },
    /// Closed by `⊤` false.
    Truth {
        /// The line.
        line: u32,
    },
    /// Open and finished.
    Open,
    /// Open and not expanded.
    Unexpanded,
}

/// The serialized form of a tableau's node.
#[cfg(feature = "interactive")]
#[derive(Serialize, Deserialize)]
struct TableauNodeForm {
    /// The lines, each a sign and a formula's index.
    lines: Vec<(SignForm, u32)>,
    /// The line the children expand and the rule, absent for a leaf.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expands: Option<(u32, Rule)>,
    /// The children, by index; absent for a leaf.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    children: Vec<u32>,
    /// How the branch ends, for a leaf.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    end: Option<EndForm>,
}

/// The serialized form of a tableau: the sequent with every formula a
/// line names, and the nodes, each before its children.
#[cfg(feature = "interactive")]
#[derive(Serialize, Deserialize)]
struct TableauForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The sequent, its form holding every formula a line names.
    sequent: SequentForm,
    /// The nodes.
    nodes: Vec<TableauNodeForm>,
}

#[cfg(feature = "interactive")]
impl From<&crate::ordinary::Tableau> for TableauForm {
    /// Writes the tableau's form, nested: without its level.
    fn from(tableau: &crate::ordinary::Tableau) -> Self {
        use crate::ordinary::{End, Sign};
        let (left, right) = tableau.sequent();
        Self {
            version: None,
            sequent: SequentForm::of(tableau.formulas(), left, right),
            nodes: tableau
                .nodes()
                .iter()
                .map(|node| TableauNodeForm {
                    lines: node
                        .lines()
                        .iter()
                        .map(|line| {
                            let sign = match line.sign {
                                Sign::True => SignForm::T,
                                Sign::False => SignForm::F,
                            };
                            (sign, line.formula.index() as u32)
                        })
                        .collect(),
                    expands: node.expands(),
                    children: node.children().to_vec(),
                    end: node.end().map(|end| match end {
                        End::Closed {
                            true_line,
                            false_line,
                        } => EndForm::Closed {
                            true_line,
                            false_line,
                        },
                        End::Falsity { line } => EndForm::Falsity { line },
                        End::Truth { line } => EndForm::Truth { line },
                        End::Open => EndForm::Open,
                        End::Unexpanded => EndForm::Unexpanded,
                    }),
                })
                .collect(),
        }
    }
}

#[cfg(feature = "interactive")]
impl TableauForm {
    /// Returns the tableau the form holds, its sequent within the limits.
    /// Refuses a form without the shape every tableau has: at least one
    /// node; every node but the first the child of exactly one node before
    /// it; a node with children expanding a line and without an end, a
    /// leaf with an end and expanding nothing; every line named in range.
    /// Whether its expansions follow the rules is not checked.
    fn within(self, limits: &Limits) -> Result<crate::ordinary::Tableau, Error> {
        use crate::ordinary::{End, Sign, Signed, Tableau, TableauNode};
        let refuse = |message: String| Error::Json {
            form: "tableau",
            message,
        };
        let (read, left, right) = self.sequent.within(limits, Extension::Sequent)?;
        read.formulas.propositional("a tableau")?;
        let total: u64 = self.nodes.iter().map(|n| n.lines.len() as u64).sum();
        let total =
            u32::try_from(total).map_err(|_| refuse("more lines than a u32 numbers".to_owned()))?;
        let line = |n: u32| {
            if n < total {
                Ok(n)
            } else {
                Err(refuse(format!(
                    "line {n} is past the tableau's {total} lines"
                )))
            }
        };
        let len = self.nodes.len();
        let mut parents = vec![0u8; len];
        let mut nodes = Vec::with_capacity(len);
        let mut first = 0u32;
        for (n, form) in self.nodes.into_iter().enumerate() {
            let lines = form
                .lines
                .iter()
                .map(|&(sign, k)| {
                    let sign = match sign {
                        SignForm::T => Sign::True,
                        SignForm::F => Sign::False,
                    };
                    Ok(Signed::new(sign, read.ids(&[k])?[0]))
                })
                .collect::<Result<Vec<_>, Error>>()?;
            for &child in &form.children {
                if (child as usize) <= n || child as usize >= len {
                    return Err(refuse(format!("node {n} names child {child}")));
                }
                parents[child as usize] = parents[child as usize].saturating_add(1);
            }
            let expands = form
                .expands
                .map(|(l, rule)| line(l).map(|l| (l, rule)))
                .transpose()?;
            let end = form
                .end
                .map(|end| {
                    Ok::<_, Error>(match end {
                        EndForm::Closed {
                            true_line,
                            false_line,
                        } => End::Closed {
                            true_line: line(true_line)?,
                            false_line: line(false_line)?,
                        },
                        EndForm::Falsity { line: l } => End::Falsity { line: line(l)? },
                        EndForm::Truth { line: l } => End::Truth { line: line(l)? },
                        EndForm::Open => End::Open,
                        EndForm::Unexpanded => End::Unexpanded,
                    })
                })
                .transpose()?;
            let leaf = form.children.is_empty();
            if leaf != end.is_some() || leaf == expands.is_some() {
                return Err(refuse(format!(
                    "node {n} must have children and expand a line, or end its branch"
                )));
            }
            let count = lines.len() as u32;
            nodes.push(TableauNode::from_parts(
                first,
                lines,
                (expands, form.children, end),
            ));
            first += count;
        }
        let shaped = parents
            .split_first()
            .is_some_and(|(&root, rest)| root == 0 && rest.iter().all(|&p| p == 1));
        if !shaped {
            return Err(refuse(
                "the nodes are no tree with its root first".to_owned(),
            ));
        }
        Ok(Tableau::from_parts(read.formulas, (left, right), nodes))
    }
}

#[cfg(feature = "interactive")]
impl Serialize for crate::ordinary::Tableau {
    /// Serializes the tableau as a document at level 14.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = TableauForm::from(self);
        form.version = Some(wire::TABLES);
        form.serialize(serializer)
    }
}

#[cfg(feature = "interactive")]
impl<'a> Deserialize<'a> for crate::ordinary::Tableau {
    /// Deserializes a tableau within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

#[cfg(feature = "interactive")]
impl crate::sealed::Readable for crate::ordinary::Tableau {}

#[cfg(feature = "interactive")]
impl Readable for crate::ordinary::Tableau {
    const FORM: &'static str = "tableau";

    /// Reads a tableau: its shape and indices checked, not its
    /// expansions.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        TableauForm::deserialize(deserializer)?
            .within(limits)
            .map_err(wire::fail)
    }
}

/// The serialized form of a negative translation compared: the
/// translation, the target logic, the agreement, both sequents and both
/// outcomes.
#[derive(Serialize)]
struct ComparisonForm<'a> {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// The translation's name.
    translation: crate::ordinary::negative::Translation,
    /// The logic translated into.
    target: Logic,
    /// `valid`, `not_valid` or `unknown`.
    agreement: crate::ordinary::negative::Agreement,
    /// The classical sequent.
    sequent: SequentForm,
    /// Its translation.
    translated: SequentForm,
    /// The classical sequent decided.
    classical: NestedOutcome<'a>,
    /// The translation decided in the target logic.
    decided: NestedOutcome<'a>,
}

/// The written form of a review of a tableau written by hand.
#[cfg(all(feature = "interactive", feature = "parse"))]
#[derive(Serialize)]
struct ReviewForm<'a> {
    /// The wire level.
    version: u32,
    /// How the tableau stands.
    standing: &'static str,
    /// The sequent its first lines give.
    sequent: SequentForm,
    /// The text line the message is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    text_line: Option<u32>,
    /// The tableau's line the message is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u32>,
    /// What the review found.
    message: &'a str,
    /// The model an open branch gives, checked.
    #[serde(skip_serializing_if = "Option::is_none")]
    countermodel: Option<CountermodelForm>,
}

#[cfg(all(feature = "interactive", feature = "parse"))]
impl Serialize for crate::ordinary::TableauReview {
    /// Serializes the review as a document at level 14, written only.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let sequent = &self.sequent;
        ReviewForm {
            version: wire::TABLES,
            standing: self.standing.word(),
            sequent: SequentForm::of(sequent.formulas(), sequent.left(), sequent.right()),
            text_line: self.text_line,
            line: self.line,
            message: &self.message,
            countermodel: self.countermodel.as_ref().map(|c| CountermodelForm {
                sequent: SequentForm::of(c.sequent.formulas(), c.sequent.left(), c.sequent.right()),
                model: super::kripke::ModelForm::from(&c.model),
            }),
        }
        .serialize(serializer)
    }
}

impl Serialize for crate::ordinary::negative::Comparison {
    /// Serializes the comparison as a document at level 16.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let form = |s: &Sequent| SequentForm::of(s.formulas(), s.left(), s.right());
        ComparisonForm {
            version: wire::TRANSFORMS,
            deduxit: env!("CARGO_PKG_VERSION"),
            translation: self.translation,
            target: self.decided.logic,
            agreement: self.agreement,
            sequent: form(&self.sequent),
            translated: form(&self.translated),
            classical: NestedOutcome(&self.classical),
            decided: NestedOutcome(&self.decided),
        }
        .serialize(serializer)
    }
}

/// The serialized form of an interpolation: the logic, the partition, the
/// sequent's outcome and the interpolant.
#[derive(Serialize)]
struct InterpolationForm<'a> {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// The logic.
    logic: Logic,
    /// The positions of the first half.
    partition: &'a crate::ordinary::interpolant::Partition,
    /// The sequent decided.
    outcome: NestedOutcome<'a>,
    /// The interpolant, where the sequent is valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    interpolant: Option<InterpolantForm<'a>>,
}

/// The serialized form of an interpolant: its text and both halves.
#[derive(Serialize)]
struct InterpolantForm<'a> {
    /// The interpolant as text.
    text: String,
    /// The first half with the interpolant right of `⊢`, decided.
    first: HalfForm<'a>,
    /// The second half with the interpolant as its first hypothesis,
    /// decided.
    second: HalfForm<'a>,
}

/// The serialized form of a half of an interpolation: its sequent and
/// its outcome.
#[derive(Serialize)]
struct HalfForm<'a> {
    /// The half's sequent.
    sequent: SequentForm,
    /// The half decided.
    outcome: NestedOutcome<'a>,
}

impl Serialize for crate::ordinary::interpolant::Interpolation {
    /// Serializes the interpolation as a document at level 16.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let half = |s: &'_ Sequent, outcome| HalfForm {
            sequent: SequentForm::of(s.formulas(), s.left(), s.right()),
            outcome: NestedOutcome(outcome),
        };
        InterpolationForm {
            version: wire::TRANSFORMS,
            deduxit: env!("CARGO_PKG_VERSION"),
            logic: self.outcome.logic,
            partition: &self.partition,
            outcome: NestedOutcome(&self.outcome),
            interpolant: self.interpolant.as_ref().map(|i| InterpolantForm {
                text: i.formula().to_string(),
                first: half(&i.first, &i.first_outcome),
                second: half(&i.second, &i.second_outcome),
            }),
        }
        .serialize(serializer)
    }
}

/// The written form of a normal form: the form, its promise, the input,
/// the result, the steps and the checks.
#[derive(Serialize)]
struct NormalFormForm<'a> {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// The form's name.
    form: crate::ordinary::normal::Form,
    /// What it keeps of the input.
    promise: crate::ordinary::normal::Promise,
    /// Whether every claim of the promise was decided valid.
    checked: bool,
    /// The input.
    sequent: SequentForm,
    /// The normal form, whose formulas the steps index.
    result: SequentForm,
    /// The steps.
    steps: Vec<StepForm>,
    /// The claims decided.
    checks: Vec<CheckedForm<'a>>,
}

/// The written form of a step: its rule and law, and indices of the
/// result's formulas.
#[derive(Serialize)]
struct StepForm {
    /// The rule.
    rule: crate::ordinary::normal::Rule,
    /// The law the rule applies, as text.
    law: &'static str,
    /// The subformula rewritten.
    before: u32,
    /// What replaces it.
    after: u32,
    /// The input's formulas left of `⊢` after the step.
    left: Vec<u32>,
    /// The input's formulas right of `⊢` after the step.
    right: Vec<u32>,
    /// The clauses the step adds as hypotheses.
    added: Vec<u32>,
    /// The term a Skolem or Herbrand step puts in, an index of the result's
    /// terms (`fo.terms`); left out for every other step.
    #[serde(skip_serializing_if = "Option::is_none")]
    term: Option<u32>,
}

/// The written form of a claim decided.
#[derive(Serialize)]
struct CheckedForm<'a> {
    /// The claim.
    claim: crate::ordinary::normal::Claim,
    /// The step whose law the claim is, by its index; left out for the
    /// other claims.
    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<usize>,
    /// The sequent whose validity is the claim.
    sequent: SequentForm,
    /// The sequent decided.
    outcome: NestedOutcome<'a>,
}

/// Returns the written form of a normal form's step.
fn step_form(step: &crate::ordinary::normal::Step) -> StepForm {
    let ids = |list: &[NodeId]| list.iter().map(|k| k.index() as u32).collect();
    StepForm {
        rule: step.rule,
        law: step.rule.law(),
        before: step.before.index() as u32,
        after: step.after.index() as u32,
        left: ids(&step.left),
        right: ids(&step.right),
        added: ids(&step.added),
        term: step.term.map(|t| t.index() as u32),
    }
}

/// The written form of Herbrand's theorem applied.
#[cfg(feature = "sat")]
#[derive(Serialize)]
struct HerbrandForm<'a> {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// `valid` or `unknown`.
    verdict: &'static str,
    /// Why the search could not decide, for `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<Why>,
    /// The sequent decided.
    sequent: SequentForm,
    /// Its clausal form and the steps that made it.
    clausal: ClausalForm,
    /// The depth of the last terms tried.
    depth: u32,
    /// The instances as a sequent.
    ground: SequentForm,
    /// Each instance's clause, terms and formula.
    instances: Vec<InstanceForm>,
    /// The SAT solver's outcome on the instances.
    #[serde(skip_serializing_if = "Option::is_none")]
    refutation: Option<NestedOutcome<'a>>,
    /// The derivation built from the instances, for `valid` by one.
    #[serde(skip_serializing_if = "Option::is_none")]
    derivation: Option<DerivationForm>,
    /// What the search cost.
    statistics: crate::ordinary::herbrand::Statistics,
}

/// The written form of a clausal form within Herbrand's theorem: the
/// result and the steps.
#[cfg(feature = "sat")]
#[derive(Serialize)]
struct ClausalForm {
    /// The clausal form, whose formulas the steps index.
    result: SequentForm,
    /// The steps.
    steps: Vec<StepForm>,
}

/// The written form of a ground instance.
#[cfg(feature = "sat")]
#[derive(Serialize)]
struct InstanceForm {
    /// The clause, by its position among the clausal form's hypotheses.
    clause: usize,
    /// The terms put for its variables, the outermost's first, indices of
    /// the ground sequent's terms.
    terms: Vec<u32>,
    /// The instance, an index of the ground sequent's formulas.
    formula: u32,
}

#[cfg(feature = "sat")]
impl Serialize for crate::ordinary::herbrand::Herbrand {
    /// Serializes Herbrand's theorem applied as a document at level 28.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let form = |s: &Sequent| SequentForm::of(s.formulas(), s.left(), s.right());
        let (verdict, reason, derivation) = match &self.verdict {
            Verdict::Valid(Evidence::Derivation(d)) => {
                ("valid", None, Some(DerivationForm::from(&**d)))
            }
            Verdict::Valid(_) => ("valid", None, None),
            Verdict::NotValid => ("not_valid", None, None),
            Verdict::Unknown(r) => ("unknown", Some(Why::from(*r)), None),
        };
        HerbrandForm {
            version: wire::HERBRAND,
            deduxit: env!("CARGO_PKG_VERSION"),
            verdict,
            reason,
            sequent: form(&self.sequent),
            clausal: ClausalForm {
                result: form(&self.clausal.result),
                steps: self.clausal.steps.iter().map(step_form).collect(),
            },
            depth: self.depth,
            ground: form(&self.ground),
            instances: self
                .instances
                .iter()
                .map(|i| InstanceForm {
                    clause: i.clause,
                    terms: i.terms.iter().map(|t| t.index() as u32).collect(),
                    formula: i.formula.index() as u32,
                })
                .collect(),
            refutation: self.refutation.as_ref().map(NestedOutcome),
            derivation,
            statistics: self.statistics,
        }
        .serialize(serializer)
    }
}

/// The written form of first-order resolution.
#[cfg(feature = "sat")]
#[derive(Serialize)]
struct FirstOrderForm {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// The search, saturation.
    method: crate::ordinary::resolution::Method,
    /// `unsatisfiable`, `saturated` or `unknown`.
    verdict: &'static str,
    /// The bound that ended the search, for `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    bound: Option<crate::ordinary::resolution::Bound>,
    /// The sequent decided.
    sequent: SequentForm,
    /// The clausal form of its negation and its steps.
    clausal: ClausalForm,
    /// The clauses the search started from, as text.
    clauses: Vec<String>,
    /// The refutation's inferences, for `unsatisfiable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    inferences: Option<Vec<InferenceStepForm>>,
    /// The derivation of LK read back, where it was.
    #[serde(skip_serializing_if = "Option::is_none")]
    derivation: Option<DerivationForm>,
    /// Why the read-back did not reach, where it did not.
    #[serde(skip_serializing_if = "Option::is_none")]
    not_read_back: Option<String>,
    /// What the search cost.
    statistics: crate::ordinary::resolution::first_order::Statistics,
}

/// The written form of a first-order inference: its step, the unifier and
/// the clause it made, as text.
#[cfg(feature = "sat")]
#[derive(Serialize)]
struct InferenceStepForm {
    /// The step: `rule` and its positions.
    #[serde(flatten)]
    step: crate::ordinary::resolution::first_order::Step,
    /// The unifier, a binding per line of text.
    unifier: Vec<String>,
    /// The clause made.
    clause: String,
}

#[cfg(feature = "sat")]
impl Serialize for crate::ordinary::resolution::first_order::FirstOrder {
    /// Serializes the decided clauses as a document at level 29.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use crate::ordinary::resolution::first_order::Answer;
        let form = |s: &Sequent| SequentForm::of(s.formulas(), s.left(), s.right());
        let (verdict, bound, inferences, derivation, not_read_back) = match &self.answer {
            Answer::Refuted {
                refutation,
                derivation,
                not_read_back,
            } => {
                let set = refutation.clauses();
                let steps = refutation
                    .inferences()
                    .iter()
                    .map(|i| InferenceStepForm {
                        step: i.step,
                        unifier: set.bindings_text(i),
                        clause: set.text(i.clause),
                    })
                    .collect();
                (
                    "unsatisfiable",
                    None,
                    Some(steps),
                    derivation.as_deref().map(DerivationForm::from),
                    not_read_back.clone(),
                )
            }
            Answer::Saturated => ("saturated", None, None, None, None),
            Answer::Unknown(bound) => ("unknown", Some(*bound), None, None, None),
        };
        FirstOrderForm {
            version: wire::FIRST_ORDER_CLAUSES,
            deduxit: env!("CARGO_PKG_VERSION"),
            method: crate::ordinary::resolution::Method::Saturation,
            verdict,
            bound,
            sequent: form(&self.sequent),
            clausal: ClausalForm {
                result: form(&self.clausal.result),
                steps: self.clausal.steps.iter().map(step_form).collect(),
            },
            clauses: (0..self.clauses.clauses().len())
                .map(|k| self.clauses.text(k))
                .collect(),
            inferences,
            derivation,
            not_read_back,
            statistics: self.statistics,
        }
        .serialize(serializer)
    }
}

impl Serialize for crate::ordinary::normal::NormalForm {
    /// Serializes the normal form as a document at level 21, at 25 where
    /// its sequents are first-order.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let form = |s: &Sequent| SequentForm::of(s.formulas(), s.left(), s.right());
        let version = if self.result.formulas().is_first_order() {
            wire::FORMS.max(wire::ORDINARY_TERMS)
        } else {
            wire::FORMS
        };
        NormalFormForm {
            version,
            deduxit: env!("CARGO_PKG_VERSION"),
            form: self.form,
            promise: self.form.promise(),
            checked: self.is_checked(),
            sequent: form(&self.sequent),
            result: form(&self.result),
            steps: self.steps.iter().map(step_form).collect(),
            checks: self
                .checks
                .iter()
                .map(|c| CheckedForm {
                    claim: c.claim,
                    step: c.step,
                    sequent: form(&c.sequent),
                    outcome: NestedOutcome(&c.outcome),
                })
                .collect(),
        }
        .serialize(serializer)
    }
}
