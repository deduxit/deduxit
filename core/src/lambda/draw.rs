// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A well-typed term's typing derivation, laid out as a tree for the
//! exports: one inference per subterm, its judgement `Γ ⊢ M : A`.

use super::print::{Framing, Spelling, UNICODE, lengths, write_judgement_at};
use super::{Calculus, Node, Options, Term, TermId, TypeId, VarId};
use crate::linear::InfId;
use crate::linear::proofs::Labels;
use crate::linear::proofs::style::Drawn;
use crate::{Error, Limits, Progress};
use std::fmt::Write;

/// What the estimate counts per inference, in bytes: its line, its label
/// and the layout's numbers.
const INFERENCE: u64 = 256;

/// What the estimate counts per byte of a judgement's Unicode text: the
/// widest target writes at most this many bytes for one, Typst's
/// ` space ` for a space.
const PER_BYTE: u64 = 8;

/// The rules, in the order of [`Rule`]'s variants: the label of each in
/// the linear calculi, then in the simple one, in the label markup.
const LABELS: [[&str; 2]; Rule::COUNT] = [
    ["ax", "ax"],
    ["⊸I", "→I"],
    ["⊸E", "→E"],
    ["⊗I", "⊗I"],
    ["⊗E", "⊗E"],
    ["1I", "1I"],
    ["1E", "1E"],
    ["&I", "∧I"],
    ["&E_1", "∧E_1"],
    ["&E_2", "∧E_2"],
    ["⊕I_1", "∨I_1"],
    ["⊕I_2", "∨I_2"],
    ["⊕E", "∨E"],
    ["⊤I", "⊤I"],
    ["0E", "⊥E"],
    ["!I", "!I"],
    ["!E", "!E"],
    ["!w", "!w"],
    ["!c", "!c"],
];

/// The rules' names in plain text, as [`LABELS`] has them.
const NAMES: [[&str; 2]; Rule::COUNT] = [
    ["ax", "ax"],
    ["⊸I", "→I"],
    ["⊸E", "→E"],
    ["⊗I", "⊗I"],
    ["⊗E", "⊗E"],
    ["1I", "1I"],
    ["1E", "1E"],
    ["&I", "∧I"],
    ["&E₁", "∧E₁"],
    ["&E₂", "∧E₂"],
    ["⊕I₁", "∨I₁"],
    ["⊕I₂", "∨I₂"],
    ["⊕E", "∨E"],
    ["⊤I", "⊤I"],
    ["0E", "⊥E"],
    ["!I", "!I"],
    ["!E", "!E"],
    ["!w", "!w"],
    ["!c", "!c"],
];

/// The typing rule a node's inference is an instance of: one per kind of
/// node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rule {
    /// A variable.
    Ax,
    /// `λx:A. M`
    LollipopI,
    /// `M N`
    LollipopE,
    /// `M ⊗ N`
    TensorI,
    /// `let M be x ⊗ y in N`
    TensorE,
    /// `∗`
    OneI,
    /// `let M be ∗ in N`
    OneE,
    /// `⟨M, N⟩`
    WithI,
    /// `fst(M)`
    WithE1,
    /// `snd(M)`
    WithE2,
    /// `inl(M)`
    PlusI1,
    /// `inr(M)`
    PlusI2,
    /// `case M of …`
    PlusE,
    /// `true(M⃗)`, `⟨⟩`
    TopI,
    /// `false(M⃗; N)`, `abort(N)`
    ZeroE,
    /// `promote M⃗ for x⃗ in N`
    BangI,
    /// `derelict(M)`
    BangE,
    /// `discard M in N`
    Weaken,
    /// `copy M as x, y in N`
    Contract,
}

impl Rule {
    /// How many rules there are.
    const COUNT: usize = 19;

    /// Returns the rule of a node.
    const fn of(node: &Node) -> Self {
        match node {
            Node::Var { .. } => Self::Ax,
            Node::Lambda { .. } => Self::LollipopI,
            Node::Apply { .. } => Self::LollipopE,
            Node::Tensor { .. } => Self::TensorI,
            Node::LetTensor { .. } => Self::TensorE,
            Node::Unit => Self::OneI,
            Node::LetUnit { .. } => Self::OneE,
            Node::Pair { .. } => Self::WithI,
            Node::First { .. } => Self::WithE1,
            Node::Second { .. } => Self::WithE2,
            Node::Left { .. } => Self::PlusI1,
            Node::Right { .. } => Self::PlusI2,
            Node::Case { .. } => Self::PlusE,
            Node::True { .. } => Self::TopI,
            Node::False { .. } => Self::ZeroE,
            Node::Promote { .. } => Self::BangI,
            Node::Derelict { .. } => Self::BangE,
            Node::Discard { .. } => Self::Weaken,
            Node::Copy { .. } => Self::Contract,
        }
    }
}

/// A premise of a node's rule: its subterm, the group of premises that
/// share their part of the context (a pair's components, a case's
/// branches), and the variables the node binds in it.
struct Premise {
    /// The subterm.
    term: TermId,
    /// The group: premises of one group get the same part of the
    /// context, different groups different parts.
    group: u32,
    /// The variables bound in the subterm, in the order written.
    binds: Vec<VarId>,
    /// Whether the premise's context is its bound variables alone: a
    /// `promote`'s body.
    closed: bool,
}

/// Returns the premises of a node, in the order the term writes them.
fn premises(node: &Node) -> Vec<Premise> {
    let one = |term: TermId, group: u32, binds: Vec<VarId>| Premise {
        term,
        group,
        binds,
        closed: false,
    };
    match node {
        Node::Var { .. } | Node::Unit => Vec::new(),
        Node::Lambda { variable, body } => vec![one(*body, 0, vec![*variable])],
        Node::Apply {
            function: a,
            argument: b,
        }
        | Node::Tensor { left: a, right: b }
        | Node::LetUnit { term: a, body: b }
        | Node::Discard { term: a, body: b } => {
            vec![one(*a, 0, Vec::new()), one(*b, 1, Vec::new())]
        }
        Node::LetTensor {
            term,
            left,
            right,
            body,
        }
        | Node::Copy {
            term,
            left,
            right,
            body,
        } => vec![
            one(*term, 0, Vec::new()),
            one(*body, 1, vec![*left, *right]),
        ],
        Node::Pair { left, right } => vec![one(*left, 0, Vec::new()), one(*right, 0, Vec::new())],
        Node::First { term }
        | Node::Second { term }
        | Node::Left { term, .. }
        | Node::Right { term, .. }
        | Node::Derelict { term } => vec![one(*term, 0, Vec::new())],
        Node::Case {
            term,
            left,
            left_body,
            right,
            right_body,
        } => vec![
            one(*term, 0, Vec::new()),
            one(*left_body, 1, vec![*left]),
            one(*right_body, 1, vec![*right]),
        ],
        Node::True { context } => (0u32..)
            .zip(context.iter())
            .map(|(g, &t)| one(t, g, Vec::new()))
            .collect(),
        Node::False { context, term, .. } => (0u32..)
            .zip(context.iter().chain(std::iter::once(term)))
            .map(|(g, &t)| one(t, g, Vec::new()))
            .collect(),
        Node::Promote {
            terms,
            variables,
            body,
        } => (0u32..)
            .zip(terms.iter())
            .map(|(g, &t)| one(t, g, Vec::new()))
            .chain(std::iter::once(Premise {
                term: *body,
                group: u32::MAX,
                binds: variables.to_vec(),
                closed: true,
            }))
            .collect(),
    }
}

/// Returns the free variables of every node, each list sorted: a node's
/// are its premises' less what it binds in each.
fn free_variables(term: &Term) -> Vec<Vec<VarId>> {
    let mut free: Vec<Vec<VarId>> = Vec::with_capacity(term.nodes.len());
    for node in &term.nodes {
        let mut own = if let Node::Var { variable } = node {
            vec![*variable]
        } else {
            Vec::new()
        };
        for premise in premises(node) {
            own.extend(
                free[premise.term.index()]
                    .iter()
                    .filter(|v| !premise.binds.contains(v)),
            );
        }
        own.sort_unstable();
        own.dedup();
        free.push(own);
    }
    free
}

/// The typing derivation of a well-typed lambda-term, as the exports
/// draw it: a tree with an inference per subterm, each concluding the
/// subterm's typing judgement `Γ ⊢ M : A`.
///
/// Its rules are those of natural deduction, which the typing rules are
/// with a term written in each conclusion: `ax` for a variable, `→I` for
/// `λ`, `→E` for an application, and so on, with `⊸`, `⊗`, `&` and `⊕`
/// and the rules of `!` in the linear calculi.
///
/// The context `Γ` of a conclusion holds the variables the subterm may
/// use. In the simple calculus a premise has its conclusion's context
/// and the variables the rule binds. In the linear and the affine
/// calculus the context is split, as the rules of linear logic split
/// it: each variable goes to the premise that uses it, the two
/// components of a pair and the two branches of a `case` share theirs,
/// and the body of a `promote` has its own variables alone. In the
/// affine calculus a variable no premise uses goes to the last, so that
/// only a leaf weakens.
///
/// Every export takes it as it takes a proof's derivation:
///
#[cfg_attr(all(feature = "parse", feature = "latex"), doc = "```")]
#[cfg_attr(not(all(feature = "parse", feature = "latex")), doc = "```ignore")]
/// use deduxit::Limits;
/// use deduxit::export::latex;
/// use deduxit::lambda::{Calculus, Drawing, Options, Term};
///
/// let limits = Limits::default();
/// let term = Term::parse(r"\f:a -> b. \x:a. f x", Calculus::Simple, &limits)?;
/// let drawing = Drawing::new(&term, &Options::default(), &limits, |_| false)?;
/// let tex = latex::to_string(&drawing, &latex::Options::default())?;
/// assert!(tex.contains(r"\infer2[$\to\mathrm{E}$]"));
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// A subterm that several nodes name is drawn once for each place.
#[derive(Clone, Debug)]
pub struct Drawing {
    /// The term, its types those of its typing: the term's own, then
    /// those the rules made.
    term: Term,
    /// The type of every node.
    of: Vec<TypeId>,
    /// How the judgements are written: the notation of the text tree,
    /// and whether `λ` shows its variable's type.
    options: Options,
    /// The node each inference draws, premises before the inferences
    /// they are premises of.
    nodes: Vec<TermId>,
    /// Per inference, where its premises end in `premises`.
    ends: Vec<u32>,
    /// The premises of every inference.
    premises: Vec<InfId>,
    /// The contexts of every inference, one after the other.
    contexts: Vec<VarId>,
    /// Per inference, where its context ends in `contexts`.
    context_ends: Vec<u32>,
}

/// A step of the walk that unfolds the term into the tree.
enum Visit {
    /// A node met on the way up, with where its context lies in the
    /// walk's contexts.
    Enter(TermId, std::ops::Range<usize>),
    /// A node whose premises' inferences are made, with its context.
    Exit(TermId, std::ops::Range<usize>),
}

impl Drawing {
    /// Checks a term and returns its typing derivation laid out as a tree
    /// from the whole term, its judgements written as `options` says.
    ///
    /// # Errors
    ///
    /// - [`Error::IllTyped`] for a term that is not well typed, as
    ///   [`Term::check`] says.
    /// - [`Refusal::Output`](crate::Refusal::Output) when the tree, a
    ///   subterm that several nodes name drawn once for each place, is
    ///   estimated past `limits.derivation_bytes`. Each judgement writes
    ///   its whole subterm, so the text grows with the term's size times
    ///   its depth.
    /// - The refusals of [`Term::check`] past the limits, and
    ///   [`Refusal::Stopped`](crate::Refusal::Stopped) when `stop` fired.
    pub fn new(
        term: &Term,
        options: &Options,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
        let typing = term.check(limits, &mut stop)?;
        let typed = Term {
            types: typing.types().to_vec(),
            ..term.clone()
        };
        let of: Vec<TypeId> = (0..typed.nodes.len())
            // A term's nodes are numbered by `u32`.
            .map(|i| typing.ty(TermId(i as u32)))
            .collect();
        let (node_lengths, type_lengths) = lengths(&typed, &UNICODE, options.types);
        let refused = |estimate: u64, limit: u64| {
            Error::Refused(crate::limits::Refusal::Output {
                what: "typing derivation",
                estimate_bytes: estimate,
                limit_bytes: limit,
                least_bytes: None,
            })
        };
        let over = |estimate: u64| match limits.derivation_bytes {
            Some(limit) if estimate > limit => Err(refused(estimate, limit)),
            _ => Ok(()),
        };
        // Every node is drawn once at least, so its text bounds what its
        // free variables take before they are made.
        let mut reached = vec![false; typed.nodes.len()];
        reached[typed.root.index()] = true;
        let mut least = 0u64;
        for i in (0..typed.nodes.len()).rev() {
            if reached[i] {
                least = least.saturating_add(node_lengths[i].saturating_mul(PER_BYTE));
                super::operands(&typed.nodes[i], |t| reached[t.index()] = true);
            }
        }
        over(least)?;
        let free = match typed.calculus {
            Calculus::Simple => None,
            Calculus::Linear | Calculus::Affine => Some(free_variables(&typed)),
        };
        let mut stop = crate::limits::counting(stop, crate::limits::Phase::View);
        let declared = |v: VarId| -> u64 {
            let variable = &typed.variables[v.index()];
            // A name, `" : "`, its type and `", "`.
            (variable.name.len() as u64)
                .saturating_add(5)
                .saturating_add(type_lengths[variable.ty.index()])
        };
        let (mut nodes, mut ends, mut premises_of) = (Vec::new(), Vec::new(), Vec::new());
        let (mut contexts, mut context_ends) = (Vec::new(), Vec::new());
        // The contexts of the visits waiting on the stack.
        let mut waiting: Vec<VarId> = typed.context.clone();
        let mut made: Vec<InfId> = Vec::new();
        let mut stack = vec![Visit::Enter(typed.root, 0..waiting.len())];
        let mut estimate = 0u64;
        let too_many = |estimate| refused(estimate, u64::from(u32::MAX));
        while let Some(visit) = stack.pop() {
            match visit {
                Visit::Enter(node, context) => {
                    if stop() {
                        return Err(Error::Refused(crate::limits::Refusal::Stopped {
                            phase: crate::limits::Phase::View,
                        }));
                    }
                    let gamma: Vec<VarId> = waiting[context.clone()].to_vec();
                    let text = gamma
                        .iter()
                        .fold(node_lengths[node.index()], |sum, &v| {
                            sum.saturating_add(declared(v))
                        })
                        .saturating_add(type_lengths[of[node.index()].index()])
                        .saturating_add(8);
                    estimate = estimate
                        .saturating_add(INFERENCE)
                        .saturating_add(text.saturating_mul(PER_BYTE));
                    over(estimate)?;
                    stack.push(Visit::Exit(node, context));
                    let premises = premises(&typed.nodes[node.index()]);
                    let parts = split(&gamma, &premises, free.as_deref());
                    let mut entered = Vec::with_capacity(premises.len());
                    for (premise, part) in premises.iter().zip(parts) {
                        let start = waiting.len();
                        waiting.extend(part);
                        waiting.extend(&premise.binds);
                        entered.push(Visit::Enter(premise.term, start..waiting.len()));
                    }
                    stack.extend(entered.into_iter().rev());
                }
                Visit::Exit(node, context) => {
                    let count = premises(&typed.nodes[node.index()]).len();
                    premises_of.extend(made.drain(made.len() - count..));
                    contexts.extend_from_slice(&waiting[context]);
                    let id =
                        InfId::new(u32::try_from(nodes.len()).map_err(|_| too_many(estimate))?);
                    nodes.push(node);
                    ends.push(u32::try_from(premises_of.len()).map_err(|_| too_many(estimate))?);
                    context_ends
                        .push(u32::try_from(contexts.len()).map_err(|_| too_many(estimate))?);
                    made.push(id);
                }
            }
        }
        Ok(Self {
            term: typed,
            of,
            options: *options,
            nodes,
            ends,
            premises: premises_of,
            contexts,
            context_ends,
        })
    }

    /// Returns the subterm each inference concludes the judgement of, in
    /// the inferences' order: the SVG's `i<n>` draws the `n`th. A subterm
    /// that several nodes name is drawn once for each place, so it may
    /// stand more than once.
    pub fn nodes(&self) -> impl ExactSizeIterator<Item = TermId> + '_ {
        self.nodes.iter().copied()
    }

    /// Returns the variables of an inference's context, in the order
    /// written.
    fn context(&self, id: InfId) -> &[VarId] {
        let start = match id.index() {
            0 => 0,
            index => self.context_ends[index - 1] as usize,
        };
        &self.contexts[start..self.context_ends[id.index()] as usize]
    }

    /// Returns the column of the calculus in the rules' tables.
    const fn column(&self) -> usize {
        match self.term.calculus {
            Calculus::Linear | Calculus::Affine => 0,
            Calculus::Simple => 1,
        }
    }

    /// Writes an inference's judgement in a spelling.
    fn judgement(
        &self,
        out: &mut impl Write,
        spelling: &Spelling,
        id: InfId,
        framing: Framing,
    ) -> std::fmt::Result {
        let node = self.nodes[id.index()];
        write_judgement_at(
            out,
            &self.term,
            spelling,
            self.context(id),
            node,
            self.of[node.index()],
            framing,
        )
    }
}

/// Returns the part of a conclusion's context `gamma` each premise gets,
/// before the variables the rule binds in it: the whole in the simple
/// calculus (`free` is `None`); in the linear calculi each variable in
/// the group of the first premise that uses it, or, used by none, in the
/// last group, the premises of one group sharing their part, and a
/// `promote`'s body nothing.
fn split(gamma: &[VarId], premises: &[Premise], free: Option<&[Vec<VarId>]>) -> Vec<Vec<VarId>> {
    let Some(free) = free else {
        return premises
            .iter()
            .map(|p| if p.closed { Vec::new() } else { gamma.to_vec() })
            .collect();
    };
    let last = premises.iter().rev().find(|p| !p.closed).map(|p| p.group);
    let mut parts = vec![Vec::new(); premises.len()];
    for &v in gamma {
        let group = premises
            .iter()
            .find(|p| !p.closed && free[p.term.index()].binary_search(&v).is_ok())
            .map(|p| p.group)
            .or(last);
        let Some(group) = group else { continue };
        for (part, premise) in parts.iter_mut().zip(premises) {
            if !premise.closed && premise.group == group {
                part.push(v);
            }
        }
    }
    parts
}

/// Returns the spelling of a lambda-term that goes with a notation of
/// the exports, told by how it writes atoms.
#[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
fn spelling(notation: &crate::linear::sequents::notation::Notation) -> &'static Spelling {
    use crate::linear::sequents::notation::Atoms;
    match notation.atom {
        Atoms::Plain => &UNICODE,
        #[cfg(feature = "latex")]
        Atoms::Latex => &super::print::LATEX,
        #[cfg(feature = "typst")]
        Atoms::Typst => &super::print::TYPST,
        #[cfg(feature = "svg")]
        Atoms::Italic => &super::print::SVG,
    }
}

impl Drawn for Drawing {
    fn rules(&self) -> usize {
        Rule::COUNT
    }

    fn open(&self) -> Option<usize> {
        None
    }

    fn markup<'a>(&'a self, rule: usize, labels: &'a Labels) -> Option<&'a str> {
        match labels {
            Labels::Off => None,
            Labels::Upright | Labels::Subscript | Labels::Table(_) => {
                LABELS.get(rule).map(|row| row[self.column()])
            }
        }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn root(&self) -> InfId {
        // `new` makes the root's inference last, and at least one.
        InfId::new(u32::try_from(self.nodes.len() - 1).unwrap_or(u32::MAX))
    }

    fn premises(&self, id: InfId) -> &[InfId] {
        let start = match id.index() {
            0 => 0,
            index => self.ends[index - 1] as usize,
        };
        &self.premises[start..self.ends[id.index()] as usize]
    }

    fn rule(&self, id: InfId) -> usize {
        Rule::of(&self.term.nodes[self.nodes[id.index()].index()]) as usize
    }

    fn name(&self, id: InfId) -> &str {
        NAMES[self.rule(id)][self.column()]
    }

    fn times(&self, _: InfId) -> u32 {
        1
    }

    fn write_sequent(&self, out: &mut impl Write, id: InfId) -> std::fmt::Result {
        let framing = Framing {
            types: self.options.types,
            marks: ("", ""),
            align: "",
        };
        self.judgement(out, self.options.notation.spelling(), id, framing)
    }

    #[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
    fn sequent(
        &self,
        out: &mut String,
        notation: &crate::linear::sequents::notation::Notation,
        id: InfId,
        aligned: bool,
        marks: bool,
    ) {
        let framing = Framing {
            types: self.options.types,
            marks: if marks { ("\u{2}", "\u{3}") } else { ("", "") },
            align: if aligned { notation.align } else { "" },
        };
        // Writing into a `String` does not fail.
        let _ = self.judgement(out, spelling(notation), id, framing);
    }

    #[cfg(feature = "svg")]
    fn positions(&self, id: InfId) -> Vec<usize> {
        (0..=self.context(id).len()).collect()
    }
}
