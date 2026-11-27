// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The names deduxit's Rocq library gives what a certificate of its
//! kernel writes, one table per module of the library, and the helpers
//! the certificate writers share.
//!
//! The library pins the same names in its `tests/Interface/`, at the
//! types a certificate needs: a rename is made there and here together,
//! and nowhere else in the crate names a constructor of the library.

use crate::limits::{Phase, Refusal};
use crate::linear::proofs::Rule;
use crate::linear::sequents::Term;
use crate::ordinary::Node;
use crate::{Error, Limits};

/// The constructor of the one-sided formula `t` in `Deduxit.Linear.Formulas`
/// (and `FirstOrderFormulas`, which adds the quantifiers).
pub(crate) const fn formula(t: Term) -> &'static str {
    match t {
        Term::Atom(_) => "var",
        Term::DualAtom(_) => "covar",
        Term::One => "one",
        Term::Bot => "bot",
        Term::Top => "top",
        Term::Zero => "zero",
        Term::Tensor(..) => "tens",
        Term::Par(..) => "parr",
        Term::With(..) => "awith",
        Term::Plus(..) => "aplus",
        Term::Bang(_) => "oc",
        Term::Quest(_) => "wn",
        Term::Forall(_) => "fall",
        Term::Exists(_) => "fex",
    }
}

/// The constructor of the intuitionistic formula the one-sided `t` stands
/// for in output position (`output`) or input position, in
/// `Deduxit.Linear.Intuitionistic` (and `FirstOrderIntuitionistic`).
pub(crate) const fn iformula(t: Term, output: bool) -> &'static str {
    match (t, output) {
        (Term::Atom(_) | Term::DualAtom(_), _) => "ivar",
        (Term::One | Term::Bot, _) => "ione",
        (Term::Top, true) | (Term::Zero, false) => "itop",
        (Term::Zero, true) | (Term::Top, false) => "izero",
        (Term::Tensor(..), true) | (Term::Par(..), false) => "itens",
        (Term::Par(..), true) | (Term::Tensor(..), false) => "ilmap",
        (Term::With(..), true) | (Term::Plus(..), false) => "iwith",
        (Term::Plus(..), true) | (Term::With(..), false) => "iplus",
        (Term::Bang(_) | Term::Quest(_), _) => "ioc",
        (Term::Forall(_), true) | (Term::Exists(_), false) => "iall",
        (Term::Exists(_), true) | (Term::Forall(_), false) => "iex",
    }
}

/// A bound variable, a de Bruijn index, in `Deduxit.Linear.Terms`.
pub(crate) const BOUND: &str = "bound";
/// An eigenvariable, by its number, in `Deduxit.Linear.Terms`.
pub(crate) const EIGEN: &str = "eigen";
/// A function applied to its arguments, in `Deduxit.Linear.Terms`.
pub(crate) const TAPP: &str = "tapp";

/// The constructor of a one-sided proof term's node for `rule`, in
/// `Deduxit.Linear.Proofs` (and `FirstOrderProofs`), or `None` for a rule
/// the term leaves implicit (contraction and weakening of `?` formulas)
/// or has no node for.
pub(crate) const fn proof(rule: Rule) -> Option<&'static str> {
    Some(match rule {
        Rule::Ax => "Ax",
        Rule::Tensor => "Tensor",
        Rule::Par => "Par",
        Rule::One => "One",
        Rule::Bot => "Bot",
        Rule::With => "With",
        Rule::PlusLeft | Rule::PlusRight => "Plus",
        Rule::Top => "Top",
        Rule::Promotion => "Bang",
        Rule::Dereliction => "Copy",
        Rule::Mix => "Mix",
        Rule::AffineWeakening => "Weaken",
        Rule::Cut => "Cut",
        Rule::Forall => "Forall",
        Rule::Exists => "Exists",
        Rule::Contraction | Rule::Weakening | Rule::Open => return None,
    })
}

/// The node that moves a `?` formula to the unrestricted zone, in
/// `Deduxit.Linear.Proofs`.
pub(crate) const QUEST: &str = "Quest";
/// The branch of a `⊕` node that keeps its left operand.
pub(crate) const LEFT: &str = "Left";
/// The branch of a `⊕` node that keeps its right operand.
pub(crate) const RIGHT: &str = "Right";

/// The constructor of a two-sided proof term's node for `rule` with its
/// principal formula on the right (`output`) or the left, in
/// `Deduxit.Linear.IntuitionisticCheck` (and
/// `FirstOrderIntuitionisticCheck`), or `None` for a rule with no
/// two-sided node.
pub(crate) const fn iproof(rule: Rule, output: bool) -> Option<&'static str> {
    Some(match (rule, output) {
        (Rule::Ax, _) => "IAx",
        (Rule::One, _) => "IOneR",
        (Rule::Bot, _) => "IOneL",
        (Rule::Tensor, true) => "ITensR",
        (Rule::Tensor, false) => "ILmapL",
        (Rule::Par, true) => "ILmapR",
        (Rule::Par, false) => "ITensL",
        (Rule::Top, true) => "ITopR",
        (Rule::Top, false) => "IZeroL",
        (Rule::With, true) => "IWithR",
        (Rule::With, false) => "IPlusL",
        (Rule::PlusLeft, true) => "IPlusR1",
        (Rule::PlusLeft, false) => "IWithL1",
        (Rule::PlusRight, true) => "IPlusR2",
        (Rule::PlusRight, false) => "IWithL2",
        (Rule::Promotion, _) => "IOcR",
        (Rule::Dereliction, _) => "IDe",
        (Rule::Contraction, _) => "ICo",
        (Rule::Weakening, _) => "IWk",
        (Rule::AffineWeakening, _) => "IWeaken",
        (Rule::Cut, _) => "ICut",
        (Rule::Forall, true) => "IAllR",
        (Rule::Forall, false) => "IExL",
        (Rule::Exists, true) => "IExR",
        (Rule::Exists, false) => "IAllL",
        (Rule::Mix | Rule::Open, _) => return None,
    })
}

/// The constructor of the formula `node`, in
/// `Deduxit.Intuitionistic.Formulas` (and `FirstOrderFormulas`, which adds
/// the quantifiers): `top`, `neg` and `equiv` are the library's
/// definitions, the others its constructors.
pub(crate) const fn ordinary(node: Node) -> &'static str {
    match node {
        Node::Atom(_) => "var",
        Node::True => "top",
        Node::False => "bot",
        Node::Not(_) => "neg",
        Node::And(..) => "wedge",
        Node::Or(..) => "vee",
        Node::Implies(..) => "imp",
        Node::Iff(..) => "equiv",
        Node::Forall(_) => "fall",
        Node::Exists(_) => "fex",
    }
}

/// The constructor of the modal formula `node` in `Deduxit.Modal.Formulas`.
#[cfg(feature = "serialize")]
pub(crate) const fn modal_node(node: crate::modal::Node) -> &'static str {
    use crate::modal::Node;
    match node {
        Node::Atom(_) => "var",
        Node::True => "top",
        Node::False => "bot",
        Node::Not(_) => "neg",
        Node::And(..) => "wedge",
        Node::Or(..) => "vee",
        Node::Implies(..) => "imp",
        Node::Iff(..) => "equiv",
        Node::Box(_) => "box",
        Node::Diamond(_) => "dia",
    }
}

/// A labelled formula `x : A`, in `Deduxit.Modal.Labelled`.
#[cfg(feature = "serialize")]
pub(crate) const LAB: &str = "lab";
/// An edge `x R y` of the accessibility relation, in
/// `Deduxit.Modal.Labelled`.
#[cfg(feature = "serialize")]
pub(crate) const REL: &str = "rel";

/// The constructor of `Deduxit.Modal.Formulas` for a connective of the
/// labelled calculus, as [`modal_node`] names it, or `None` for a symbol
/// that is none.
#[cfg(feature = "serialize")]
pub(crate) fn modal(connective: &str) -> Option<&'static str> {
    Some(match connective {
        "⊤" => "top",
        "⊥" => "bot",
        "¬" => "neg",
        "∧" => "wedge",
        "∨" => "vee",
        "→" => "imp",
        "↔" => "equiv",
        "□" => "box",
        "◇" => "dia",
        _ => return None,
    })
}

/// The constructor of a labelled proof term's node for a rule of the
/// labelled calculus, in `Deduxit.Modal.Check`, or `None` for a rule it
/// has none for.
#[cfg(feature = "serialize")]
pub(crate) fn modal_proof(rule: &str) -> Option<&'static str> {
    Some(match rule {
        "ax" => "Ax",
        "L⊥" => "LBot",
        "R⊤" => "RTop",
        "L¬" => "LNeg",
        "R¬" => "RNeg",
        "L∧" => "LAnd",
        "R∧" => "RAnd",
        "L∨" => "LOr",
        "R∨" => "ROr",
        "L→" => "LImp",
        "R→" => "RImp",
        "L↔" => "LIff",
        "R↔" => "RIff",
        "L□" => "LBox",
        "R□" => "RBox",
        "L◇" => "LDia",
        "R◇" => "RDia",
        "Ref" => "Ref",
        "Ser" => "Ser",
        "Sym" => "Sym",
        "Trans" => "Trans",
        "Eucl" => "Eucl",
        _ => return None,
    })
}

/// Returns a name as a Rocq string, `"` doubled.
pub(crate) fn string(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

/// Returns the stop's refusal while a certificate is written.
pub(crate) fn stopped() -> Error {
    Refusal::stopped(Phase::Write).into()
}

/// Text that refuses to grow past `limits.derivation_bytes`, so that a
/// certificate is never held or written past the bound.
pub(crate) struct Bounded {
    /// The text so far.
    pub(crate) text: String,
    /// The bound, if any.
    limit: Option<u64>,
}

impl Bounded {
    /// Returns empty text under `limits.derivation_bytes`.
    pub(crate) const fn new(limits: &Limits) -> Self {
        Self {
            text: String::new(),
            limit: limits.derivation_bytes,
        }
    }

    /// Appends `piece`, or refuses where the text would pass the bound.
    pub(crate) fn push(&mut self, piece: &str) -> Result<(), Error> {
        self.push_all(&[piece])
    }

    /// Appends `pieces` as one piece, or refuses where the text would pass
    /// the bound: the bound is compared once, after the last, so that a
    /// refusal names the same length whether a name and its space are
    /// pushed together or apart.
    pub(crate) fn push_all(&mut self, pieces: &[&str]) -> Result<(), Error> {
        for piece in pieces {
            self.text.push_str(piece);
        }
        match self.limit {
            Some(limit) if self.text.len() as u64 > limit => Err(Refusal::Output {
                what: "certificate",
                estimate_bytes: self.text.len() as u64,
                limit_bytes: limit,
                least_bytes: None,
            }
            .into()),
            _ => Ok(()),
        }
    }
}
