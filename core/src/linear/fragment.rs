// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use crate::Error;
use crate::linear::sequents::{Kind, Sequent};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{BitAnd, BitOr, BitOrAssign};
use std::str::FromStr;

/// A fragment of linear logic: a set of connective classes.
///
/// There are six classes: the multiplicatives `⊗ ⅋`, their units `1 ⊥`,
/// the additives `& ⊕`, their units `⊤ 0`, the exponentials `! ?`, and
/// the quantifiers `∀ ∃`. A fragment is any set of them.
///
/// - [`Sequent::fragment`] returns the classes a sequent uses.
/// - [`union`](Self::union) joins two fragments, and
///   [`contains`](Self::contains) tells whether one is part of another.
/// - The constants [`MLL`](Self::MLL) to [`LL1`](Self::LL1) are the usual
///   named fragments.
/// - A fragment prints as the name of the smallest named fragment that
///   contains it, and `FromStr` reads such a name.
///
/// # Examples
///
/// ```
/// use deduxit::linear::Fragment;
///
/// let fragment = Fragment::MULTIPLICATIVES | Fragment::EXPONENTIALS;
/// assert!(Fragment::MELL.contains(fragment));
/// assert!(!Fragment::MALL.contains(fragment));
/// assert_eq!(fragment.to_string(), "MELL");
/// assert_eq!("IMALL".parse::<Fragment>()?, Fragment::MALL);
/// # Ok::<(), deduxit::Error>(())
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Fragment(u8);

impl Fragment {
    /// No connective at all: atoms only.
    pub const EMPTY: Self = Self(0);
    /// `⊗` and `⅋`.
    pub const MULTIPLICATIVES: Self = Self(1);
    /// `1` and `⊥`.
    pub const MULTIPLICATIVE_UNITS: Self = Self(2);
    /// `&` and `⊕`.
    pub const ADDITIVES: Self = Self(4);
    /// `⊤` and `0`.
    pub const ADDITIVE_UNITS: Self = Self(8);
    /// `!` and `?`.
    pub const EXPONENTIALS: Self = Self(16);
    /// `∀` and `∃`: the quantifiers of first-order linear logic.
    ///
    /// A first-order fragment is named as the propositional one with a `1`
    /// after the acronym (`MLL1`, `MALL1`, `LL1`). A ground atom such as
    /// `p(a)` sets no class: only a quantifier puts a sequent in a
    /// first-order fragment.
    pub const QUANTIFIERS: Self = Self(32);

    /// Multiplicative linear logic without units: `⊗ ⅋`.
    pub const MLL: Self = Self::MULTIPLICATIVES;
    /// Multiplicative linear logic with units: `⊗ ⅋ 1 ⊥`.
    pub const MLL_WITH_UNITS: Self = Self::MLL.union(Self::MULTIPLICATIVE_UNITS);
    /// Additive linear logic, named `ALL`: `& ⊕ ⊤ 0`, and nothing
    /// multiplicative. The fragment with every connective is
    /// [`LL`](Self::LL).
    pub const ADDITIVE: Self = Self::ADDITIVES.union(Self::ADDITIVE_UNITS);
    /// Multiplicative-additive linear logic: `⊗ ⅋ 1 ⊥ & ⊕ ⊤ 0`.
    pub const MALL: Self = Self::MLL_WITH_UNITS.union(Self::ADDITIVE);
    /// Multiplicative-exponential linear logic: `⊗ ⅋ 1 ⊥ ! ?`.
    pub const MELL: Self = Self::MLL_WITH_UNITS.union(Self::EXPONENTIALS);
    /// Full propositional linear logic: every propositional connective.
    pub const LL: Self = Self::MALL.union(Self::EXPONENTIALS);
    /// First-order multiplicative linear logic without units: `⊗ ⅋ ∀ ∃`.
    pub const MLL1: Self = Self::MLL.union(Self::QUANTIFIERS);
    /// First-order multiplicative linear logic with units.
    pub const MLL1_WITH_UNITS: Self = Self::MLL_WITH_UNITS.union(Self::QUANTIFIERS);
    /// First-order additive linear logic, named `ALL1`.
    pub const ADDITIVE1: Self = Self::ADDITIVE.union(Self::QUANTIFIERS);
    /// First-order multiplicative-additive linear logic.
    pub const MALL1: Self = Self::MALL.union(Self::QUANTIFIERS);
    /// First-order multiplicative-exponential linear logic.
    pub const MELL1: Self = Self::MELL.union(Self::QUANTIFIERS);
    /// Full first-order linear logic: every connective and both quantifiers.
    pub const LL1: Self = Self::LL.union(Self::QUANTIFIERS);

    /// The named fragments, in the order of [`NAMES`](Self::NAMES). Each is
    /// the largest fragment of its name.
    pub const NAMED: &'static [Self] = &[
        Self::MLL,
        Self::MLL_WITH_UNITS,
        Self::ADDITIVE,
        Self::MALL,
        Self::MELL,
        Self::LL,
        Self::MLL1,
        Self::MLL1_WITH_UNITS,
        Self::ADDITIVE1,
        Self::MALL1,
        Self::MELL1,
        Self::LL1,
    ];

    /// The names of the [`NAMED`](Self::NAMED) fragments, as
    /// [`name`](Self::name) writes them and [`FromStr`] reads them.
    pub const NAMES: &'static [&'static str] = &[
        "MLL",
        "MLL with units",
        "ALL",
        "MALL",
        "MELL",
        "LL",
        "MLL1",
        "MLL1 with units",
        "ALL1",
        "MALL1",
        "MELL1",
        "LL1",
    ];

    /// Returns the fragment with the connective classes of both.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns the connective classes the two fragments share.
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Returns whether every connective class of `other` is in this
    /// fragment: whether a sequent of `other` is also a sequent of this
    /// fragment.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns whether no connective class is present: atoms only.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns whether `⊗` or `⅋` occurs.
    pub const fn has_multiplicatives(self) -> bool {
        self.contains(Self::MULTIPLICATIVES)
    }

    /// Returns whether `1` or `⊥` occurs.
    pub const fn has_multiplicative_units(self) -> bool {
        self.contains(Self::MULTIPLICATIVE_UNITS)
    }

    /// Returns whether `&` or `⊕` occurs.
    pub const fn has_additives(self) -> bool {
        self.contains(Self::ADDITIVES)
    }

    /// Returns whether `⊤` or `0` occurs.
    pub const fn has_additive_units(self) -> bool {
        self.contains(Self::ADDITIVE_UNITS)
    }

    /// Returns whether `!` or `?` occurs.
    pub const fn has_exponentials(self) -> bool {
        self.contains(Self::EXPONENTIALS)
    }

    /// Returns whether `∀` or `∃` occurs, which makes the fragment
    /// first-order.
    pub const fn has_quantifiers(self) -> bool {
        self.contains(Self::QUANTIFIERS)
    }

    /// Returns whether a sequent of this fragment has proof structures
    /// ([`ProofStructure`](crate::linear::ProofStructure)): whether the
    /// fragment is unit-free MLL.
    pub const fn has_nets(self) -> bool {
        Self::MLL.contains(self)
    }

    /// Returns the name of the smallest named fragment that contains this
    /// one: `MLL`, `MLL with units`, `ALL`, `MALL`, `MELL` or `LL`.
    ///
    /// A fragment with quantifiers has a `1` after the acronym (`MLL1`,
    /// `MLL1 with units`, `LL1`). The empty fragment is named `MLL`. Units
    /// change only the name of a purely multiplicative fragment, where
    /// they decide which engine applies.
    pub const fn name(self) -> &'static str {
        let additive = self.has_additives() || self.has_additive_units();
        let multiplicative = self.has_multiplicatives() || self.has_multiplicative_units();
        let units = self.has_multiplicative_units();
        match (
            self.has_quantifiers(),
            self.has_exponentials(),
            additive,
            multiplicative,
        ) {
            (false, true, true, _) => "LL",
            (false, true, false, _) => "MELL",
            (false, false, true, true) => "MALL",
            (false, false, true, false) => "ALL",
            (false, false, false, _) if units => "MLL with units",
            (false, false, false, _) => "MLL",
            (true, true, true, _) => "LL1",
            (true, true, false, _) => "MELL1",
            (true, false, true, true) => "MALL1",
            (true, false, true, false) => "ALL1",
            (true, false, false, _) if units => "MLL1 with units",
            (true, false, false, _) => "MLL1",
        }
    }

    /// Returns the fragment's name in `mode`: the classical
    /// [`name`](Self::name), or in an intuitionistic mode the
    /// intuitionistic one: `IMLL`, `IMLL with units`, `IALL`, `IMALL`,
    /// `IMELL` or `ILL`, with the `1` of a first-order fragment (`IMLL1`,
    /// …).
    pub const fn name_in(self, mode: Mode) -> &'static str {
        if !mode.is_intuitionistic() {
            return self.name();
        }
        let additive = self.has_additives() || self.has_additive_units();
        let multiplicative = self.has_multiplicatives() || self.has_multiplicative_units();
        let units = self.has_multiplicative_units();
        match (
            self.has_quantifiers(),
            self.has_exponentials(),
            additive,
            multiplicative,
        ) {
            (false, true, true, _) => "ILL",
            (false, true, false, _) => "IMELL",
            (false, false, true, true) => "IMALL",
            (false, false, true, false) => "IALL",
            (false, false, false, _) if units => "IMLL with units",
            (false, false, false, _) => "IMLL",
            (true, true, true, _) => "ILL1",
            (true, true, false, _) => "IMELL1",
            (true, false, true, true) => "IMALL1",
            (true, false, true, false) => "IALL1",
            (true, false, false, _) if units => "IMLL1 with units",
            (true, false, false, _) => "IMLL1",
        }
    }
}

impl FromStr for Fragment {
    type Err = Error;

    /// Reads a fragment's name, classical or with the intuitionistic `I`
    /// before it (`MALL`, `IMALL`). Returns the largest fragment of that
    /// name.
    fn from_str(name: &str) -> Result<Self, Error> {
        let classical = name.strip_prefix('I').unwrap_or(name);
        Self::NAMES
            .iter()
            .position(|n| *n == classical)
            .map(|i| Self::NAMED[i])
            .ok_or_else(|| Error::UnknownName {
                what: "fragment",
                name: name.into(),
                known: Self::NAMES,
            })
    }
}

impl BitOr for Fragment {
    type Output = Self;

    /// The union of the connective classes.
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}

impl BitOrAssign for Fragment {
    /// Adds the connective classes of `other`.
    fn bitor_assign(&mut self, other: Self) {
        *self = self.union(other);
    }
}

impl BitAnd for Fragment {
    type Output = Self;

    /// The connective classes both fragments have.
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}

impl Display for Fragment {
    /// Writes the fragment's [`name`](Self::name).
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

impl Kind {
    /// Returns the connective class of this kind, or the empty fragment
    /// for a literal.
    pub const fn fragment(self) -> Fragment {
        use Kind::*;
        match self {
            Atom | DualAtom => Fragment::EMPTY,
            Tensor | Par => Fragment::MULTIPLICATIVES,
            One | Bot => Fragment::MULTIPLICATIVE_UNITS,
            With | Plus => Fragment::ADDITIVES,
            Top | Zero => Fragment::ADDITIVE_UNITS,
            Bang | Quest => Fragment::EXPONENTIALS,
            Forall | Exists => Fragment::QUANTIFIERS,
        }
    }
}

impl Sequent {
    /// Returns the smallest fragment the sequent lives in: the connective
    /// classes of the terms that its root and cut formulas reach.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "parse", doc = "```")]
    #[cfg_attr(not(feature = "parse"), doc = "```ignore")]
    /// use deduxit::linear::{Fragment, Sequent};
    ///
    /// let mll: Sequent = "A, A -o B |- B".parse()?;
    /// assert_eq!(mll.fragment(), Fragment::MLL);
    /// assert_eq!(mll.fragment().to_string(), "MLL");
    ///
    /// let ll: Sequent = "|- (A & B) * !C".parse()?;
    /// assert!(ll.fragment().has_additives());
    /// assert!(Fragment::LL.contains(ll.fragment()));
    /// assert!(!Fragment::MALL.contains(ll.fragment()));
    /// assert_eq!(ll.fragment().to_string(), "LL");
    /// # Ok::<(), deduxit::Error>(())
    /// ```
    pub fn fragment(&self) -> Fragment {
        let mut fragment = Fragment::EMPTY;
        // A subterm precedes its parent, so one pass from the top visits
        // every reachable term after the term that reaches it.
        let mut reachable = vec![false; self.terms.len()];
        for root in self.roots.iter().chain(self.cuts.iter().flatten()) {
            reachable[root.index()] = true;
        }
        for (n, term) in self.terms.iter().enumerate().rev() {
            if !reachable[n] {
                continue;
            }
            fragment |= term.kind().fragment();
            for k in term.subterms() {
                reachable[k.index()] = true;
            }
        }
        fragment
    }
}

/// The logic a sequent is proved in: classical or intuitionistic,
/// ordered or not, and which extra rules hold.
///
/// Pass a mode to [`prove`](crate::linear::prove) and the other entry
/// points of the search. Start from a constant, add rules with the
/// builders, or read a mode from its name:
///
/// | name | value | what it means |
/// |---|---|---|
/// | `classical` | [`CLASSICAL`](Self::CLASSICAL), the default | classical linear logic: a sequent `⊢ Γ`, every formula used exactly once (a `?A` may be dropped or copied) |
/// | `affine` | `CLASSICAL.with_affine()` | adds weakening: a formula may go unused, so `A, B ⊢ A` holds |
/// | `mix` | `CLASSICAL.with_mix()` | adds the Mix rule: proofs of `⊢ Γ` and `⊢ Δ` side by side prove `⊢ Γ, Δ` |
/// | `affine-mix` | `CLASSICAL.with_affine().with_mix()` | adds both |
/// | `intuitionistic` | [`INTUITIONISTIC`](Self::INTUITIONISTIC) | intuitionistic linear logic: a sequent `Γ ⊢ A` with one formula, the goal, right of `⊢` |
/// | `intuitionistic-affine` | `INTUITIONISTIC.with_affine()` | adds weakening: a hypothesis may go unused |
/// | `cyclic` | [`CYCLIC`](Self::CYCLIC) | cyclic linear logic: classical, and the order of the formulas counts, up to rotation |
/// | `lambek` | [`LAMBEK`](Self::LAMBEK) | the Lambek calculus `L`: intuitionistic, the hypotheses in their written order, at least one of them |
/// | `lambek-star` | `LAMBEK.with_empty_antecedents()` | the Lambek calculus `L*`: as `L`, and a sequent may have no hypothesis |
///
/// [`check`](Self::check) refuses the combinations that no calculus has:
/// intuitionistic with Mix, and an ordered mode with weakening or Mix.
///
/// # Examples
///
/// ```
/// use deduxit::linear::Mode;
///
/// // Intuitionistic linear logic in which a hypothesis may go unused.
/// let mode = Mode::INTUITIONISTIC.with_affine();
/// assert_eq!(mode.name(), "intuitionistic-affine");
/// assert_eq!("intuitionistic-affine".parse::<Mode>()?, mode);
/// assert!(mode.check().is_ok());
///
/// // No calculus is intuitionistic with Mix.
/// assert!(Mode::INTUITIONISTIC.with_mix().check().is_err());
/// # Ok::<(), deduxit::Error>(())
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Mode {
    /// Intuitionistic linear logic, with one formula right of `⊢`. Else
    /// classical, with one-sided sequents.
    pub(crate) intuitionistic: bool,
    /// Affine logic: weakening is allowed, so a formula may go unused.
    pub(crate) affine: bool,
    /// The Mix rule is allowed: `⊢ Γ` and `⊢ Δ` give `⊢ Γ, Δ`.
    pub(crate) mix: bool,
    /// Whether the order of a sequent's formulas counts.
    pub(crate) order: Order,
    /// In an ordered intuitionistic mode, whether a sequent may have no
    /// formula left of `⊢`: true for the Lambek calculus `L*`, false for
    /// `L`. False in every other mode, which has no such restriction to
    /// lift.
    pub(crate) empty_antecedents: bool,
}

/// Whether a calculus keeps the order of a sequent's formulas.
///
/// In a commutative calculus a sequent is a multiset: its formulas may
/// be exchanged freely. In the cyclic calculus a one-sided sequent is a
/// cyclic sequence of formulas. It may be rotated, and allows no other
/// exchange:
/// - `⊗` joins the two arcs `⊢ Γ, A` and `⊢ B, Δ` into `⊢ Γ, A ⊗ B, Δ`;
/// - `⅋` joins two neighbours `A, B` into `A ⅋ B`;
/// - the dual of a product reverses it: `(A ⊗ B)⊥ = B⊥ ⅋ A⊥`.
///
/// A sequent's order is that of its literals as written. The occurrence
/// forest numbers them in that order, a formula's left operand before
/// its right one. So the formulas of a cyclic sequent, and of every
/// subproof, stand in ascending occurrence order, up to rotation.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Order {
    /// A sequent is a multiset of formulas.
    #[default]
    Commutative,
    /// A sequent is a cyclic sequence of formulas: cyclic linear logic.
    Cyclic,
}

impl Mode {
    /// Classical linear logic, the default: no weakening, no Mix.
    pub const CLASSICAL: Self = Self {
        intuitionistic: false,
        affine: false,
        mix: false,
        order: Order::Commutative,
        empty_antecedents: false,
    };

    /// Intuitionistic linear logic: one formula right of `⊢`, no
    /// weakening.
    pub const INTUITIONISTIC: Self = Self {
        intuitionistic: true,
        affine: false,
        mix: false,
        order: Order::Commutative,
        empty_antecedents: false,
    };

    /// Cyclic linear logic without Mix: classical, one-sided, with the
    /// order of the formulas kept up to rotation ([`Order::Cyclic`]).
    pub const CYCLIC: Self = Self {
        intuitionistic: false,
        affine: false,
        mix: false,
        order: Order::Cyclic,
        empty_antecedents: false,
    };

    /// The Lambek calculus `L`: intuitionistic, in the cyclic order.
    ///
    /// A sequent is `A₁, …, Aₙ ⊢ B`, with its hypotheses in the order
    /// written and at least one of them. The connectives are the two
    /// divisions `A \ B` and `B / A` and the product `A ⊗ B`. `L` is the
    /// fragment of cyclic MLL whose sequents, and every subproof's, have
    /// one output and at least one input.
    /// [`with_empty_antecedents`](Self::with_empty_antecedents) drops the
    /// condition of at least one input (`L*`).
    pub const LAMBEK: Self = Self {
        intuitionistic: true,
        affine: false,
        mix: false,
        order: Order::Cyclic,
        empty_antecedents: false,
    };

    /// The names of the modes that [`check`](Self::check) accepts, as
    /// [`name`](Self::name) writes them and [`FromStr`] reads them.
    pub const NAMES: &'static [&'static str] = &[
        "classical",
        "affine",
        "mix",
        "affine-mix",
        "intuitionistic",
        "intuitionistic-affine",
        "cyclic",
        "lambek",
        "lambek-star",
    ];

    /// Returns whether sequents are intuitionistic: two-sided, with one
    /// formula right of `⊢`. Otherwise they are classical and one-sided.
    pub const fn is_intuitionistic(self) -> bool {
        self.intuitionistic
    }

    /// Returns whether weakening is allowed: whether a formula may go
    /// unused.
    pub const fn is_affine(self) -> bool {
        self.affine
    }

    /// Returns whether the Mix rule is allowed.
    pub const fn has_mix(self) -> bool {
        self.mix
    }

    /// Returns which order of a sequent's formulas the calculus keeps.
    pub const fn order(self) -> Order {
        self.order
    }

    /// Returns whether the order of a sequent's formulas counts: whether
    /// the order is other than [`Order::Commutative`].
    pub const fn is_ordered(self) -> bool {
        !matches!(self.order, Order::Commutative)
    }

    /// Returns the mode with weakening allowed.
    #[must_use]
    pub const fn with_affine(self) -> Self {
        Self {
            affine: true,
            ..self
        }
    }

    /// Returns the mode with the Mix rule allowed.
    #[must_use]
    pub const fn with_mix(self) -> Self {
        Self { mix: true, ..self }
    }

    /// Returns the mode that also allows sequents with no formula left of
    /// `⊢`: from [`LAMBEK`](Self::LAMBEK), the Lambek calculus `L*`. Every
    /// other mode allows them already and comes back unchanged.
    #[must_use]
    pub const fn with_empty_antecedents(self) -> Self {
        Self {
            empty_antecedents: self.intuitionistic && self.is_ordered(),
            ..self
        }
    }

    /// Returns whether a sequent may have no formula left of `⊢`. True in
    /// every mode but the Lambek calculus `L`, where every sequent of a
    /// proof keeps a hypothesis.
    pub const fn allows_empty_antecedents(self) -> bool {
        !(self.intuitionistic && self.is_ordered()) || self.empty_antecedents
    }

    /// Returns the mode's name: one of [`NAMES`](Self::NAMES). A mode that
    /// [`check`](Self::check) refuses is named by the words of what it
    /// combines (`intuitionistic-mix`, `cyclic-affine`, `lambek-mix`, …).
    pub const fn name(self) -> &'static str {
        let Self {
            intuitionistic,
            affine,
            mix,
            order,
            empty_antecedents: star,
        } = self;
        match (order, intuitionistic, affine, mix) {
            (Order::Commutative, false, false, false) => "classical",
            (Order::Commutative, false, true, false) => "affine",
            (Order::Commutative, false, false, true) => "mix",
            (Order::Commutative, false, true, true) => "affine-mix",
            (Order::Commutative, true, false, false) => "intuitionistic",
            (Order::Commutative, true, true, false) => "intuitionistic-affine",
            (Order::Commutative, true, false, true) => "intuitionistic-mix",
            (Order::Commutative, true, true, true) => "intuitionistic-affine-mix",
            (Order::Cyclic, false, false, false) => "cyclic",
            (Order::Cyclic, false, true, false) => "cyclic-affine",
            (Order::Cyclic, false, false, true) => "cyclic-mix",
            (Order::Cyclic, false, true, true) => "cyclic-affine-mix",
            (Order::Cyclic, true, false, false) if star => "lambek-star",
            (Order::Cyclic, true, false, false) => "lambek",
            (Order::Cyclic, true, true, false) if star => "lambek-star-affine",
            (Order::Cyclic, true, true, false) => "lambek-affine",
            (Order::Cyclic, true, false, true) if star => "lambek-star-mix",
            (Order::Cyclic, true, false, true) => "lambek-mix",
            (Order::Cyclic, true, true, true) if star => "lambek-star-affine-mix",
            (Order::Cyclic, true, true, true) => "lambek-affine-mix",
        }
    }

    /// Refuses a combination that no calculus has: intuitionistic with
    /// Mix, or an ordered mode with weakening or Mix.
    ///
    /// # Errors
    ///
    /// [`Error::IntuitionisticMix`] for an intuitionistic mode with Mix,
    /// [`Error::OrderedMode`] for an ordered mode with weakening or Mix.
    pub const fn check(self) -> Result<(), Error> {
        if self.is_ordered() && (self.affine || self.mix) {
            Err(Error::OrderedMode { mode: self })
        } else if self.intuitionistic && self.mix {
            Err(Error::IntuitionisticMix)
        } else {
            Ok(())
        }
    }
}

impl FromStr for Mode {
    type Err = Error;

    /// Reads a mode's [`name`](Self::name), one of
    /// [`NAMES`](Self::NAMES).
    fn from_str(name: &str) -> Result<Self, Error> {
        let mode = match name {
            "classical" => Self::CLASSICAL,
            "affine" => Self::CLASSICAL.with_affine(),
            "mix" => Self::CLASSICAL.with_mix(),
            "affine-mix" => Self::CLASSICAL.with_affine().with_mix(),
            "intuitionistic" => Self::INTUITIONISTIC,
            "intuitionistic-affine" => Self::INTUITIONISTIC.with_affine(),
            "cyclic" => Self::CYCLIC,
            "lambek" => Self::LAMBEK,
            "lambek-star" => Self::LAMBEK.with_empty_antecedents(),
            _ => {
                return Err(Error::UnknownName {
                    what: "mode",
                    name: name.into(),
                    known: Self::NAMES,
                });
            }
        };
        Ok(mode)
    }
}

impl Display for Mode {
    /// Writes the mode in words: `classical`, `intuitionistic`, or where
    /// the order counts `cyclic` or `Lambek`. Then `affine` and
    /// `with Mix` where they apply, and for `L*` `with empty antecedents`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(match (self.is_ordered(), self.intuitionistic) {
            (false, false) => "classical",
            (false, true) => "intuitionistic",
            (true, false) => "cyclic",
            (true, true) => "Lambek",
        })?;
        if self.affine {
            f.write_str(" affine")?;
        }
        if self.mix {
            f.write_str(" with Mix")?;
        }
        if self.empty_antecedents {
            f.write_str(" with empty antecedents")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The named fragments nest as the names say.
    #[test]
    fn named_fragments_nest() {
        use Fragment as F;
        assert!(F::MLL_WITH_UNITS.contains(F::MLL));
        assert!(F::MALL.contains(F::MLL_WITH_UNITS));
        assert!(F::MALL.contains(F::ADDITIVE));
        assert!(F::MELL.contains(F::MLL_WITH_UNITS));
        assert!(F::LL.contains(F::MALL));
        assert!(F::LL.contains(F::MELL));
        assert!(!F::MALL.contains(F::MELL));
        assert!(!F::MELL.contains(F::ADDITIVE));
        assert!(!F::MLL.contains(F::MULTIPLICATIVE_UNITS));
        assert!(F::EMPTY.is_empty());
        assert!(!F::MLL.is_empty());
        assert_eq!(F::MALL & F::MELL, F::MLL_WITH_UNITS);
        assert_eq!(F::ADDITIVE | F::EXPONENTIALS | F::MLL_WITH_UNITS, F::LL);
    }

    /// Every combination of connective classes gets the name of the smallest
    /// named fragment containing it.
    #[test]
    fn names() {
        use Fragment as F;
        for (fragment, name) in [
            (F::EMPTY, "MLL"),
            (F::MLL, "MLL"),
            (F::MULTIPLICATIVE_UNITS, "MLL with units"),
            (F::MLL_WITH_UNITS, "MLL with units"),
            (F::ADDITIVES, "ALL"),
            (F::ADDITIVE_UNITS, "ALL"),
            (F::ADDITIVE, "ALL"),
            (F::MLL | F::ADDITIVES, "MALL"),
            (F::MULTIPLICATIVE_UNITS | F::ADDITIVE_UNITS, "MALL"),
            (F::MALL, "MALL"),
            (F::EXPONENTIALS, "MELL"),
            (F::MLL | F::EXPONENTIALS, "MELL"),
            (F::MELL, "MELL"),
            (F::ADDITIVES | F::EXPONENTIALS, "LL"),
            (F::MELL | F::ADDITIVE_UNITS, "LL"),
            (F::LL, "LL"),
        ] {
            assert_eq!(fragment.to_string(), name, "{fragment:?}");
        }
    }

    /// The intuitionistic names put an `I` in front of the classical ones.
    #[test]
    fn intuitionistic_names() {
        use Fragment as F;
        for (fragment, name) in [
            (F::EMPTY, "IMLL"),
            (F::MLL, "IMLL"),
            (F::MLL_WITH_UNITS, "IMLL with units"),
            (F::ADDITIVE, "IALL"),
            (F::MALL, "IMALL"),
            (F::MELL, "IMELL"),
            (F::LL, "ILL"),
        ] {
            assert_eq!(fragment.name_in(Mode::INTUITIONISTIC), name, "{fragment:?}");
            assert_eq!(
                fragment.name_in(Mode::INTUITIONISTIC.with_affine()),
                name,
                "{fragment:?}"
            );
            assert_eq!(
                fragment.name_in(Mode::CLASSICAL),
                fragment.name(),
                "{fragment:?}"
            );
        }
    }

    /// Every mode a calculus has, and every named fragment, reads back
    /// from its name; an unknown name is refused naming the known ones.
    #[test]
    fn names_read_back() {
        let modes = [
            Mode::CLASSICAL,
            Mode::CLASSICAL.with_affine(),
            Mode::CLASSICAL.with_mix(),
            Mode::CLASSICAL.with_affine().with_mix(),
            Mode::INTUITIONISTIC,
            Mode::INTUITIONISTIC.with_affine(),
            Mode::CYCLIC,
            Mode::LAMBEK,
            Mode::LAMBEK.with_empty_antecedents(),
        ];
        assert_eq!(modes.map(Mode::name), Mode::NAMES);
        for mode in modes {
            assert_eq!(mode.name().parse::<Mode>().unwrap(), mode);
            assert!(mode.check().is_ok(), "{mode}");
        }
        assert!(Mode::INTUITIONISTIC.with_mix().check().is_err());
        for refused in [Mode::CYCLIC.with_affine(), Mode::CYCLIC.with_mix()] {
            assert!(matches!(refused.check(), Err(Error::OrderedMode { mode }) if mode == refused));
        }
        assert!(matches!(
            "planar".parse::<Mode>(),
            Err(Error::UnknownName {
                what: "mode",
                known: Mode::NAMES,
                ..
            })
        ));
        for (&fragment, name) in Fragment::NAMED.iter().zip(Fragment::NAMES) {
            assert_eq!(fragment.name(), *name);
            assert_eq!(name.parse::<Fragment>().unwrap(), fragment);
            assert_eq!(format!("I{name}").parse::<Fragment>().unwrap(), fragment);
        }
        assert!("MLLL".parse::<Fragment>().is_err());
    }

    /// Detection sees exactly the connectives of a sequent, dualised
    /// hypotheses included.
    #[cfg(feature = "parse")]
    #[test]
    fn detection() {
        use Fragment as F;
        for (input, fragment) in [
            ("|-", F::EMPTY),
            ("A |- A", F::EMPTY),
            ("A, A -o B |- B", F::MLL),
            ("A * B |- A par B", F::MLL),
            ("|- 1", F::MULTIPLICATIVE_UNITS),
            ("1 |-", F::MULTIPLICATIVE_UNITS),
            ("A * 1 |- A", F::MLL_WITH_UNITS),
            ("A & B |- A", F::ADDITIVES),
            ("|- A & B", F::ADDITIVES),
            ("|- top", F::ADDITIVE_UNITS),
            ("0 |- A", F::ADDITIVE_UNITS),
            ("A & B |- A + 0", F::ADDITIVE),
            ("A & B |- A * B", F::MLL | F::ADDITIVES),
            ("A * B, C par D |- A & B, C + D, 0, top, 1, bot", F::MALL),
            ("!A |- A", F::EXPONENTIALS),
            ("|- !A", F::EXPONENTIALS),
            ("!A, !(A -o B) |- !B", F::MLL | F::EXPONENTIALS),
            ("!A, !(A -o B) |- !B, 1", F::MELL),
            ("!(A & B) |- ?(A + B)", F::ADDITIVES | F::EXPONENTIALS),
            ("!(A & B) * 1 |- ?(A + B), top", F::LL),
        ] {
            let s: Sequent = input.parse().unwrap();
            assert_eq!(s.fragment(), fragment, "{input:?}");
        }
    }

    /// Terms no root reaches do not count.
    #[test]
    fn detection_ignores_unreachable_terms() {
        use crate::linear::sequents::{Term, TermId};
        let s = Sequent {
            terms: vec![Term::One, Term::Bang(TermId::new(0)), Term::Top],
            roots: vec![TermId::new(1)],
            atoms: vec![],
            antecedents: None,
            cuts: vec![],
            fo: Default::default(),
        };
        assert_eq!(
            s.fragment(),
            Fragment::MULTIPLICATIVE_UNITS | Fragment::EXPONENTIALS
        );
    }

    /// The modes print in words.
    #[test]
    fn mode_names() {
        assert_eq!(Mode::default(), Mode::CLASSICAL);
        assert_eq!(Mode::CLASSICAL.to_string(), "classical");
        assert_eq!(
            Mode::INTUITIONISTIC.with_affine().to_string(),
            "intuitionistic affine"
        );
        assert_eq!(
            Mode::CLASSICAL.with_affine().with_mix().to_string(),
            "classical affine with Mix"
        );
        assert_eq!(
            Mode::INTUITIONISTIC.with_mix().to_string(),
            "intuitionistic with Mix"
        );
        assert_eq!(Mode::CYCLIC.to_string(), "cyclic");
        assert_eq!(Mode::LAMBEK.to_string(), "Lambek");
        assert_eq!(
            Mode::LAMBEK.with_empty_antecedents().to_string(),
            "Lambek with empty antecedents"
        );
    }

    /// Only the Lambek calculus `L` forbids empty antecedents, and lifting
    /// the restriction changes no other mode.
    #[test]
    fn empty_antecedents() {
        assert!(!Mode::LAMBEK.allows_empty_antecedents());
        let star = Mode::LAMBEK.with_empty_antecedents();
        assert!(star.allows_empty_antecedents() && star != Mode::LAMBEK);
        for mode in [Mode::CLASSICAL, Mode::INTUITIONISTIC, Mode::CYCLIC] {
            assert!(mode.allows_empty_antecedents());
            assert_eq!(mode.with_empty_antecedents(), mode);
        }
        assert_eq!(Mode::LAMBEK.check(), Ok(()));
        assert!(matches!(
            Mode::LAMBEK.with_affine().check(),
            Err(Error::OrderedMode { .. })
        ));
    }
}
