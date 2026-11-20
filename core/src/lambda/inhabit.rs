// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Inhabitation of simple types: whether a type has a closed term, and
//! its terms counted and listed.
//!
//! A simple type is an ordinary formula, and a term of the type is a
//! proof of the formula in intuitionistic logic (the Curry-Howard
//! correspondence; *Proofs and Types*, chapter 3). So [`inhabit`] asks
//! the ordinary engines whether `Γ ⊢ A` is valid, reads the derivation
//! back as a term and checks it.
//!
//! [`enumerate`] lists the terms of a type up to βη-equality, as the
//! long normal forms of Ben-Yelles's algorithm (C.-B. Ben-Yelles, *Type
//! Assignment in the Lambda-Calculus: Syntax & Semantics*, PhD thesis,
//! Swansea, 1979; J. R. Hindley, *Basic Simple Type Theory*, Cambridge,
//! 1997, chapter 8, "Counting a type's inhabitants"): a
//! long normal form of a function type is a `λ`, of a pair type a pair,
//! of `⊤` the unit `⟨⟩`, and of an atom a variable taken apart down to
//! that atom, `y M₁ … Mₙ` with projections between. Each long normal
//! form is one term up to βη, and two different ones are two terms.
//!
//! Deciding inhabitation is PSPACE-complete (R. Statman,
//! "Intuitionistic propositional logic is polynomial-space complete",
//! TCS 9, 1979), so every answer is found within bounds. The count is
//! one of three:
//! - **exactly n**: the search met every long normal form;
//! - **infinitely many**: a goal came back inside itself, with the same
//!   types in scope, on a way that every term passes; putting a term of
//!   the outer goal in place of the inner goal's makes a longer term, as
//!   often as wanted;
//! - **at least n**: a bound cut the search, so more terms may exist.
//!
//! The search never enters a goal without a term: each goal's types in
//! scope and the goal are first decided by the engines, so the search
//! tree is finite exactly when the type has finitely many terms. Every
//! term listed is built and checked by [`Term::check`], so the listing
//! rests on the checker, and the count on the engines' checked verdicts.
//!
//! The count is up to βη-equality only for the types without `∨` and
//! `⊥` (and so without `¬`). With them, βη-equality identifies terms
//! that long normal forms tell apart (G. Scherer and D. Rémy, "Which
//! simple types have a unique inhabitant?", ICFP 2015), and
//! [`enumerate`] refuses them; [`inhabit`] takes every type.

use super::{Builder, Calculus, Node, Term, TermId, Type, TypeId, VarId};
use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::search::Reason;
use crate::ordinary::{self, Formulas, Logic, Method, NodeId, Sequent, Verdict};

/// How far [`enumerate`] searches: how deep a term nests and how many
/// terms it lists.
///
/// # JSON
///
/// With the feature `serialize`, `{"depth": 8, "max_terms": 32}`; a key
/// left out keeps its default, and an unknown one is refused.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct InhabitOptions {
    /// The most variables a term applies inside each other, counted along
    /// the arguments: `f (f x)` nests three deep. [`DEFAULT_DEPTH`](Self::DEFAULT_DEPTH)
    /// by default.
    pub depth: u32,
    /// The most terms listed; the count goes on past it.
    /// [`DEFAULT_MAX_TERMS`](Self::DEFAULT_MAX_TERMS) by default.
    pub max_terms: u32,
}

impl InhabitOptions {
    /// The default depth.
    pub const DEFAULT_DEPTH: u32 = 8;
    /// The default number of terms listed.
    pub const DEFAULT_MAX_TERMS: u32 = 32;

    /// Returns these options with the depth given.
    #[must_use]
    pub const fn with_depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }

    /// Returns these options with the number of terms listed given.
    #[must_use]
    pub const fn with_max_terms(mut self, max_terms: u32) -> Self {
        self.max_terms = max_terms;
        self
    }
}

impl Default for InhabitOptions {
    fn default() -> Self {
        Self {
            depth: Self::DEFAULT_DEPTH,
            max_terms: Self::DEFAULT_MAX_TERMS,
        }
    }
}

/// Whether a type has a term, as [`inhabit`] decides it.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Inhabitation {
    /// The type has this term, which [`Term::check`] accepted.
    Inhabited(Box<Term>),
    /// The type has no term: its formula is not valid in intuitionistic
    /// logic.
    #[non_exhaustive]
    Empty {
        /// A Kripke model in which the formula is false, checked, where
        /// the engine gave one.
        countermodel: Option<ordinary::Countermodel>,
    },
    /// The engines did not decide within the bounds, for the reason
    /// given.
    Unknown(Reason),
}

/// How many terms a type has, as [`enumerate`] counts them.
///
/// # JSON
///
/// With the feature `serialize`, tagged by `kind`: `{"kind": "exactly",
/// "count": 2}`, `{"kind": "infinite"}`, `{"kind": "at_least", "count":
/// 5}`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "kind", rename_all = "snake_case"))]
pub enum Count {
    /// Exactly this many: the search met every term.
    #[non_exhaustive]
    Exactly {
        /// How many.
        count: u64,
    },
    /// Infinitely many.
    Infinite,
    /// At least this many: a bound cut the search, so there may be more.
    #[non_exhaustive]
    AtLeast {
        /// How many were met.
        count: u64,
    },
}

/// The terms of a type, as [`enumerate`] lists them, and their count.
///
/// # JSON
///
/// With the feature `serialize`, `{"version": 24, "count": {"kind":
/// "exactly", "count": 2}, "terms": […]}`, each term a term's document
/// ([`Term`]). It is written only.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Inhabitants {
    /// The terms listed, each checked, the shortest first: all of them
    /// where the count is exact and no more than
    /// [`InhabitOptions::max_terms`].
    pub terms: Vec<Term>,
    /// How many terms the type has.
    pub count: Count,
}

impl Inhabitants {
    /// Returns whether the type has exactly one term: `None` where the
    /// count is "at least" one or none.
    pub const fn unique(&self) -> Option<bool> {
        match self.count {
            Count::Exactly { count } => Some(count == 1),
            Count::Infinite => Some(false),
            Count::AtLeast { count } => {
                if count >= 2 {
                    Some(false)
                } else {
                    None
                }
            }
        }
    }
}

/// Decides whether `sequent`'s formula right of `⊢` has a term in which
/// the formulas left of it are free variables, and returns that term,
/// checked.
///
/// The sequent is decided in intuitionistic logic by
/// [`ordinary::decide_within`] with its default method, and by LJT where
/// that method gives no derivation; the derivation is read back
/// ([`ordinary::Derivation::term`]) and checked ([`Term::check`]). `¬A`
/// is the type `A → ⊥` and `A ↔ B` the pair of both implications. A
/// sequent with nothing right of `⊢` asks for a term of `⊥`.
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::{Limits, lambda::{self, Inhabitation}, ordinary};
///
/// let sequent: ordinary::Sequent = "|- a -> b -> a".parse()?;
/// let Inhabitation::Inhabited(term) = lambda::inhabit(&sequent, &Limits::default(), |_| false)? else {
///     panic!("a → b → a has a term");
/// };
/// assert_eq!(term.to_string(), "λx. λy. x");
///
/// let peirce: ordinary::Sequent = "|- ((a -> b) -> a) -> a".parse()?;
/// let answer = lambda::inhabit(&peirce, &Limits::default(), |_| false)?;
/// assert!(matches!(answer, Inhabitation::Empty { .. }));
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// # Errors
///
/// [`Error::Succedents`] for more than one formula right of `⊢`; the
/// engines' errors and refusals ([`Refusal`]), the read-back's and the
/// checker's.
pub fn inhabit(
    sequent: &Sequent,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Inhabitation, Error> {
    inhabit_dyn(sequent, limits, &mut stop)
}

/// [`inhabit`] with the stop as a trait object.
fn inhabit_dyn(
    sequent: &Sequent,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Inhabitation, Error> {
    succedents(sequent)?;
    let search = crate::linear::Options::default();
    let options = ordinary::Options::default().with_logic(Logic::Intuitionistic);
    let mut outcome = ordinary::decide_within(sequent, &options, &search, limits, &mut *stop)?;
    if matches!(&outcome.verdict, Verdict::Valid(evidence) if evidence.derivation().is_err()) {
        let ljt = options.with_method(Some(Method::Ljt));
        outcome = ordinary::decide_within(sequent, &ljt, &search, limits, &mut *stop)?;
    }
    match outcome.verdict {
        Verdict::Valid(evidence) => {
            let term = evidence.derivation()?.term(limits, &mut *stop)?;
            term.check(limits, &mut *stop)?;
            Ok(Inhabitation::Inhabited(Box::new(term)))
        }
        Verdict::NotValid => Ok(Inhabitation::Empty {
            countermodel: outcome.countermodel,
        }),
        Verdict::Unknown(reason) => Ok(Inhabitation::Unknown(reason)),
    }
}

/// Refuses a sequent with more than one formula right of `⊢`.
fn succedents(sequent: &Sequent) -> Result<(), Error> {
    match sequent.right().len() {
        0 | 1 => Ok(()),
        count => Err(Error::Succedents { count }),
    }
}

/// Lists the terms of `sequent`'s formula right of `⊢`, in which the
/// formulas left of it are free variables, up to βη-equality, and counts
/// them: exactly, infinitely many, or at least as many as were met
/// within `options` and `limits`.
///
/// The types are built from atoms, `→`, `∧`, `⊤` and `↔`: with `∨`, `⊥`
/// or `¬`, βη-equality identifies terms that long normal forms tell
/// apart, and the count would be wrong. Every term listed is checked by
/// [`Term::check`].
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::{Limits, lambda::{self, Count, InhabitOptions}, ordinary};
///
/// let options = InhabitOptions::default();
/// let limits = Limits::default();
/// let pick: ordinary::Sequent = "|- a -> a -> a".parse()?;
/// let found = lambda::enumerate(&pick, &options, &limits, |_| false)?;
/// assert!(matches!(found.count, Count::Exactly { count: 2, .. }));
/// let texts: Vec<String> = found.terms.iter().map(ToString::to_string).collect();
/// assert_eq!(texts, ["λx. λy. x", "λx. λy. y"]);
///
/// // The Church numerals: infinitely many.
/// let numerals: ordinary::Sequent = "|- (a -> a) -> a -> a".parse()?;
/// let found = lambda::enumerate(&numerals, &options, &limits, |_| false)?;
/// assert_eq!(found.count, Count::Infinite);
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// # Errors
///
/// [`Error::NotEnumerable`] for a type with `∨`, `⊥` or `¬`;
/// [`Error::Succedents`] for more than one formula right of `⊢`;
/// [`Refusal::Work`] and [`Refusal::Memory`] past the limits, and
/// [`Refusal::Stopped`] when `stop` fired; the engines' and the checker's
/// errors.
pub fn enumerate(
    sequent: &Sequent,
    options: &InhabitOptions,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Inhabitants, Error> {
    enumerate_dyn(sequent, options, limits, &mut stop)
}

/// [`enumerate`] with the stop as a trait object.
fn enumerate_dyn(
    sequent: &Sequent,
    options: &InhabitOptions,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Inhabitants, Error> {
    succedents(sequent)?;
    let mut search = Search::new(sequent, options, limits, stop)?;
    let root_types: Vec<TypeId> = search
        .context
        .iter()
        .map(|&v| search.vars[v as usize].ty)
        .collect();
    let decided = search.decided(root_types, search.goal)?;
    let found = match decided {
        Decided::Empty => Found::default(),
        Decided::Unknown => {
            search.undecided = true;
            Found::default()
        }
        Decided::Inhabited => search.run()?,
    };
    let count = if search.infinite {
        Count::Infinite
    } else if search.cut || search.undecided || found.overflow {
        Count::AtLeast { count: found.count }
    } else {
        Count::Exactly { count: found.count }
    };
    let terms = search.terms(&found.terms)?;
    Ok(Inhabitants { terms, count })
}

/// Returns whether `term` is a term of `sequent`'s formula right of `⊢`
/// (`⊥` where there is none) whose free variables are the formulas left
/// of it, in order: each formula's type is made in the term's own arena,
/// where equal types are one id, and compared by id.
#[cfg(feature = "protocol")]
pub(crate) fn has_type(term: &Term, sequent: &Sequent) -> bool {
    let mut builder = Builder::new();
    for atom in term.atoms() {
        builder.atom(atom);
    }
    builder.types = super::Types::of(term.types(), sequent.formulas().len());
    let known = builder.types.list.len();
    let mut cache = HashMap::default();
    let given = sequent.formulas();
    let left: Vec<TypeId> = sequent
        .left()
        .iter()
        .map(|&f| super::simple::type_of(given, &mut builder, &mut cache, f))
        .collect();
    let goal = match sequent.right().first() {
        Some(&f) => super::simple::type_of(given, &mut builder, &mut cache, f),
        None => builder.ty(Type::Zero),
    };
    let context: Vec<TypeId> = term
        .context()
        .iter()
        .map(|&v| term.variables()[v.index()].ty())
        .collect();
    // A type made anew is no type of the term's.
    goal.index() < known && goal == term.goal() && left == context
}

/// A long normal form, as the search builds it: its parts by index.
#[derive(Clone, Debug)]
enum Lnf {
    /// `λx:A. M`: the variable and the body.
    Lambda {
        /// `x`, an index of [`Search::vars`].
        var: u32,
        /// `M`
        body: u32,
    },
    /// `⟨M, N⟩`
    Pair(u32, u32),
    /// `⟨⟩`
    Unit,
    /// `y` taken apart: the variable, then its eliminations, a range of
    /// [`Search::elims`].
    Spine {
        /// `y`, an index of [`Search::vars`].
        head: u32,
        /// The first elimination.
        from: u32,
        /// One past the last.
        to: u32,
    },
}

/// One elimination of a spine.
#[derive(Clone, Copy, Debug)]
enum Elim {
    /// An application to this long normal form.
    Apply(u32),
    /// `fst`
    First,
    /// `snd`
    Second,
}

/// An elimination of a candidate head, before its arguments are found.
#[derive(Clone, Copy, Debug)]
enum Way {
    /// An application to a term of this type.
    Apply(TypeId),
    /// `fst`
    First,
    /// `snd`
    Second,
}

/// A variable of the search: its type and its name.
#[derive(Clone, Debug)]
struct Var {
    /// The type.
    ty: TypeId,
    /// The name.
    name: Box<str>,
}

/// A context: the variables in scope, as a list linked to its parent.
#[derive(Clone, Copy, Debug)]
struct Ctx {
    /// The context it extends, `None` for the sequent's.
    parent: Option<u32>,
    /// The variable it adds, `None` for the sequent's.
    var: Option<u32>,
    /// How many variables of function type and of other types are in
    /// scope, for the next name.
    named: [u32; 2],
}

/// The terms found for a goal: some listed, all counted.
#[derive(Clone, Debug, Default)]
struct Found {
    /// The terms listed, at most `max_terms`.
    terms: Vec<u32>,
    /// How many there are, saturating.
    count: u64,
    /// Whether the count saturated.
    overflow: bool,
}

/// What the search has yet to do.
#[derive(Debug)]
enum Task {
    /// Find the terms of a type in a context, at a depth left.
    Solve {
        /// The context.
        ctx: u32,
        /// The type.
        ty: TypeId,
        /// How many atoms deep the search may still go.
        depth: u32,
    },
    /// `λ` around each term found last.
    Lambda(u32),
    /// Pair the two lists found last.
    Pair,
    /// Apply a head to the lists found last, one per argument.
    Spine {
        /// The head.
        head: u32,
        /// Its eliminations.
        ways: Vec<Way>,
    },
    /// Join the lists of this many candidates found last.
    Join(usize),
    /// Leave a goal: the shape at this index is no longer on the way.
    Leave(u32),
}

/// Whether the engines found a type inhabited in a context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Decided {
    /// It has a term.
    Inhabited,
    /// It has none.
    Empty,
    /// The engines did not decide.
    Unknown,
}

/// The bytes a long normal form and an elimination take, for the memory
/// bound.
const NODE_BYTES: u64 = 16;

/// How often the stop is asked, in units of work.
const POLL: u64 = 1024;

/// The state of one enumeration.
struct Search<'a> {
    /// The options.
    options: &'a InhabitOptions,
    /// The limits.
    limits: &'a Limits,
    /// The caller's stop.
    stop: &'a mut dyn FnMut(Progress) -> bool,
    /// The atoms and the types, every type of the sequent's.
    base: Builder,
    /// The type of every formula as an ordinary formula, for the engines.
    formulas: Formulas,
    /// The ordinary formula of every type, by index.
    formula_of: Vec<NodeId>,
    /// The variables: the sequent's hypotheses first.
    vars: Vec<Var>,
    /// The sequent's hypotheses.
    context: Vec<u32>,
    /// The goal.
    goal: TypeId,
    /// The contexts.
    ctxs: Vec<Ctx>,
    /// The long normal forms made.
    lnf: Vec<Lnf>,
    /// The eliminations of their spines.
    elims: Vec<Elim>,
    /// What the engines decided, by the sorted types in scope and the
    /// goal.
    decisions: HashMap<(Vec<TypeId>, TypeId), Decided>,
    /// Every atomic goal's shape, by its sorted types in scope and atom.
    shapes: HashMap<(Vec<TypeId>, TypeId), u32>,
    /// How often each shape is on the way from the root.
    on_way: Vec<u32>,
    /// Whether a shape came back inside itself.
    infinite: bool,
    /// Whether the depth cut the search.
    cut: bool,
    /// Whether the engines left a goal undecided.
    undecided: bool,
    /// The work done.
    work: u64,
    /// The work at which the stop is asked next.
    next: u64,
}

impl<'a> Search<'a> {
    /// Returns the search for `sequent`'s goal, its types made and
    /// checked to be in the fragment.
    fn new(
        sequent: &Sequent,
        options: &'a InhabitOptions,
        limits: &'a Limits,
        stop: &'a mut dyn FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
        let mut base = Builder::new();
        let mut cache = HashMap::default();
        let given = sequent.formulas();
        let left: Vec<TypeId> = sequent
            .left()
            .iter()
            .map(|&f| super::simple::type_of(given, &mut base, &mut cache, f))
            .collect();
        let goal = match sequent.right().first() {
            Some(&f) => super::simple::type_of(given, &mut base, &mut cache, f),
            None => base.ty(Type::Zero),
        };
        let mut formulas = Formulas::default();
        let mut formula_of = Vec::with_capacity(base.types.list.len());
        for i in 0..base.types.list.len() {
            let of = |t: TypeId| formula_of[t.index()];
            let node = match base.types.list[i] {
                Type::Atom { atom } => formulas.atom(&base.atoms[atom as usize])?,
                Type::Top => formulas.add(ordinary::Node::True)?,
                Type::With { left, right } => {
                    formulas.add(ordinary::Node::And(of(left), of(right)))?
                }
                Type::Lollipop { from, to } => {
                    formulas.add(ordinary::Node::Implies(of(from), of(to)))?
                }
                Type::Zero => return Err(Error::NotEnumerable { connective: "⊥" }),
                Type::Plus { .. } => return Err(Error::NotEnumerable { connective: "∨" }),
                // An ordinary formula's type is none of these.
                Type::One | Type::Tensor { .. } | Type::Bang { .. } => {
                    return Err(Error::NotEnumerable {
                        connective: "a linear connective",
                    });
                }
            };
            formula_of.push(node);
        }
        let mut search = Self {
            options,
            limits,
            stop,
            base,
            formulas,
            formula_of,
            vars: Vec::new(),
            context: Vec::new(),
            goal,
            ctxs: Vec::new(),
            lnf: Vec::new(),
            elims: Vec::new(),
            decisions: HashMap::default(),
            shapes: HashMap::default(),
            on_way: Vec::new(),
            infinite: false,
            cut: false,
            undecided: false,
            work: 0,
            next: POLL,
        };
        let mut ctx = None;
        for ty in left {
            let (var, extended) = search.extend(ctx, ty);
            search.context.push(var);
            ctx = Some(extended);
        }
        if ctx.is_none() {
            search.ctxs.push(Ctx {
                parent: None,
                var: None,
                named: [0; 2],
            });
        }
        Ok(search)
    }

    /// Adds a variable of type `ty` to the context `ctx` (`None` for the
    /// empty one), named by the next letter for its kind of type, and
    /// returns it and the context made.
    fn extend(&mut self, ctx: Option<u32>, ty: TypeId) -> (u32, u32) {
        let named = ctx.map_or([0; 2], |c| self.ctxs[c as usize].named);
        let class = usize::from(!matches!(self.base.types.get(ty), Type::Lollipop { .. }));
        let letters = super::LETTERS[class];
        let n = named[class];
        let letters_len = letters.len() as u32;
        let letter = letters[(n % letters_len) as usize];
        let name = match n / letters_len {
            0 => letter.to_string(),
            index => format!("{letter}{index}"),
        };
        let mut named = named;
        named[class] = n.saturating_add(1);
        // The variables are fewer than the work units the search counts,
        // which its limit keeps far below 2³².
        let var = self.vars.len() as u32;
        self.vars.push(Var {
            ty,
            name: name.into(),
        });
        let made = self.ctxs.len() as u32;
        self.ctxs.push(Ctx {
            parent: ctx,
            var: Some(var),
            named,
        });
        (var, made)
    }

    /// Returns the variables in scope in `ctx`, the outermost first.
    fn scope(&self, ctx: u32) -> Vec<u32> {
        let mut vars = Vec::new();
        let mut at = Some(ctx);
        while let Some(c) = at {
            let node = self.ctxs[c as usize];
            vars.extend(node.var);
            at = node.parent;
        }
        vars.reverse();
        vars
    }

    /// Counts `units` of work, and refuses past the limit, past the
    /// memory bound or when the stop fires.
    fn tick(&mut self, units: usize) -> Result<(), Error> {
        self.work = self.work.saturating_add(units as u64);
        if let Some(limit) = self.limits.work.filter(|&most| self.work > most) {
            return Err(Error::Refused(Refusal::Work { limit }));
        }
        let held = ((self.lnf.len() + self.elims.len()) as u64).saturating_mul(NODE_BYTES);
        if let Some(limit) = self.limits.memory_bytes.filter(|&limit| held > limit) {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Search,
                limit_bytes: limit,
                needed_bytes: None,
            }));
        }
        if self.work >= self.next {
            self.next = self.work.saturating_add(POLL);
            if (self.stop)(Progress::new(Phase::Search, POLL, self.work)) {
                return Err(Error::Refused(Refusal::Stopped {
                    phase: Phase::Search,
                }));
            }
        }
        Ok(())
    }

    /// Returns whether the engines find `goal` inhabited with variables
    /// of `types` in scope, asking them once per set of types and goal.
    fn decided(&mut self, mut types: Vec<TypeId>, goal: TypeId) -> Result<Decided, Error> {
        types.sort_unstable();
        types.dedup();
        let key = (types, goal);
        if let Some(&decided) = self.decisions.get(&key) {
            return Ok(decided);
        }
        self.tick(key.0.len().saturating_add(1))?;
        let left = key.0.iter().map(|t| self.formula_of[t.index()]).collect();
        let right = vec![self.formula_of[goal.index()]];
        let sequent = Sequent::new(self.formulas.clone(), left, right)?;
        let options = ordinary::Options::default().with_logic(Logic::Intuitionistic);
        let search = crate::linear::Options::default();
        let outcome =
            ordinary::decide_within(&sequent, &options, &search, self.limits, &mut *self.stop)?;
        let decided = match outcome.verdict {
            Verdict::Valid(_) => Decided::Inhabited,
            Verdict::NotValid => Decided::Empty,
            Verdict::Unknown(Reason::Stopped) => {
                return Err(Error::Refused(Refusal::Stopped {
                    phase: Phase::Search,
                }));
            }
            Verdict::Unknown(_) => Decided::Unknown,
        };
        self.decisions.insert(key, decided);
        Ok(decided)
    }

    /// Returns the shape of an atomic goal: an index for its sorted types
    /// in scope and its atom.
    fn shape(&mut self, ctx: u32, atom: TypeId) -> u32 {
        let mut types: Vec<TypeId> = self
            .scope(ctx)
            .iter()
            .map(|&v| self.vars[v as usize].ty)
            .collect();
        types.sort_unstable();
        types.dedup();
        let next = self.shapes.len() as u32;
        let id = *self.shapes.entry((types, atom)).or_insert(next);
        if id as usize == self.on_way.len() {
            self.on_way.push(0);
        }
        id
    }

    /// Returns the ways to take a variable of type `ty` apart down to
    /// `atom`, each a list of eliminations, `fst` before `snd`.
    fn ways(&mut self, ty: TypeId, atom: TypeId) -> Result<Vec<Vec<Way>>, Error> {
        let mut found = Vec::new();
        let mut stack = vec![(ty, Vec::new())];
        while let Some((at, ways)) = stack.pop() {
            self.tick(1)?;
            match self.base.types.get(at) {
                Type::Lollipop { from, to } => {
                    let mut ways = ways;
                    ways.push(Way::Apply(from));
                    stack.push((to, ways));
                }
                Type::With { left, right } => {
                    let mut second = ways.clone();
                    second.push(Way::Second);
                    stack.push((right, second));
                    let mut first = ways;
                    first.push(Way::First);
                    stack.push((left, first));
                }
                Type::Atom { .. } => {
                    if at == atom {
                        found.push(ways);
                    }
                }
                // The fragment was checked: no other type is reached.
                Type::Top
                | Type::Zero
                | Type::One
                | Type::Plus { .. }
                | Type::Tensor { .. }
                | Type::Bang { .. } => {}
            }
        }
        Ok(found)
    }

    /// Adds a long normal form and returns its index.
    fn add(&mut self, lnf: Lnf) -> Result<u32, Error> {
        self.tick(1)?;
        let id = self.lnf.len() as u32;
        self.lnf.push(lnf);
        Ok(id)
    }

    /// Finds the terms of the goal in the sequent's context.
    fn run(&mut self) -> Result<Found, Error> {
        let root = (self.ctxs.len() - 1) as u32;
        let mut tasks = vec![Task::Solve {
            ctx: root,
            ty: self.goal,
            depth: self.options.depth,
        }];
        let mut results: Vec<Found> = Vec::new();
        while let Some(task) = tasks.pop() {
            self.tick(1)?;
            match task {
                Task::Solve { ctx, ty, depth } => {
                    self.solve(ctx, ty, depth, &mut tasks, &mut results)?
                }
                Task::Lambda(var) => {
                    let body = results.pop().unwrap_or_default();
                    let mut terms = Vec::with_capacity(body.terms.len());
                    for &t in &body.terms {
                        terms.push(self.add(Lnf::Lambda { var, body: t })?);
                    }
                    results.push(Found { terms, ..body });
                }
                Task::Pair => {
                    let right = results.pop().unwrap_or_default();
                    let left = results.pop().unwrap_or_default();
                    let lists = [left, right];
                    let found = self.product(&lists, |search, picked| {
                        search.add(Lnf::Pair(picked[0], picked[1]))
                    })?;
                    results.push(found);
                }
                Task::Spine { head, ways } => {
                    let args = ways.iter().filter(|w| matches!(w, Way::Apply(_))).count();
                    let lists = results.split_off(results.len().saturating_sub(args));
                    let found = self.product(&lists, |search, picked| {
                        let from = search.elims.len() as u32;
                        let mut next = picked.iter();
                        for way in &ways {
                            let elim = match way {
                                Way::Apply(_) => Elim::Apply(*next.next().unwrap_or(&0)),
                                Way::First => Elim::First,
                                Way::Second => Elim::Second,
                            };
                            search.elims.push(elim);
                        }
                        let to = search.elims.len() as u32;
                        search.add(Lnf::Spine { head, from, to })
                    })?;
                    results.push(found);
                }
                Task::Join(n) => {
                    let parts = results.split_off(results.len().saturating_sub(n));
                    let mut joined = Found::default();
                    let most = self.options.max_terms as usize;
                    for part in parts {
                        let room = most.saturating_sub(joined.terms.len());
                        joined.terms.extend(part.terms.into_iter().take(room));
                        let (count, overflow) = joined.count.overflowing_add(part.count);
                        joined.count = if overflow { u64::MAX } else { count };
                        joined.overflow |= overflow || part.overflow;
                    }
                    results.push(joined);
                }
                Task::Leave(shape) => {
                    let on = &mut self.on_way[shape as usize];
                    *on = on.saturating_sub(1);
                }
            }
        }
        Ok(results.pop().unwrap_or_default())
    }

    /// Plans the search of one goal: a `λ` for a function type, a pair
    /// for a pair type, the unit for `⊤`, and for an atom every variable
    /// in scope that can be taken apart down to it, with arguments that
    /// all have terms.
    fn solve(
        &mut self,
        ctx: u32,
        ty: TypeId,
        depth: u32,
        tasks: &mut Vec<Task>,
        results: &mut Vec<Found>,
    ) -> Result<(), Error> {
        match self.base.types.get(ty) {
            Type::Lollipop { from, to } => {
                let (var, inner) = self.extend(Some(ctx), from);
                tasks.push(Task::Lambda(var));
                tasks.push(Task::Solve {
                    ctx: inner,
                    ty: to,
                    depth,
                });
            }
            Type::With { left, right } => {
                tasks.push(Task::Pair);
                tasks.push(Task::Solve {
                    ctx,
                    ty: right,
                    depth,
                });
                tasks.push(Task::Solve {
                    ctx,
                    ty: left,
                    depth,
                });
            }
            Type::Top => {
                let unit = self.add(Lnf::Unit)?;
                results.push(Found {
                    terms: vec![unit],
                    count: 1,
                    overflow: false,
                });
            }
            Type::Atom { .. } => self.atomic(ctx, ty, depth, tasks, results)?,
            // The fragment was checked: no other type is reached.
            Type::Zero
            | Type::One
            | Type::Plus { .. }
            | Type::Tensor { .. }
            | Type::Bang { .. } => {
                results.push(Found::default());
            }
        }
        Ok(())
    }

    /// Plans the search of an atomic goal, every one of whose candidates'
    /// arguments the engines found inhabited.
    fn atomic(
        &mut self,
        ctx: u32,
        atom: TypeId,
        depth: u32,
        tasks: &mut Vec<Task>,
        results: &mut Vec<Found>,
    ) -> Result<(), Error> {
        let shape = self.shape(ctx, atom);
        if self.on_way[shape as usize] > 0 {
            // The same types in scope and the same atom, inside itself:
            // every goal on the way has a term, so a term of the outer goal
            // can stand for the inner one, as often as wanted.
            self.infinite = true;
        }
        if depth == 0 {
            // The goal has a term (its candidate's arguments were
            // decided), which this depth does not reach.
            self.cut = true;
            results.push(Found::default());
            return Ok(());
        }
        let scope = self.scope(ctx);
        let types: Vec<TypeId> = scope.iter().map(|&v| self.vars[v as usize].ty).collect();
        let mut candidates = Vec::new();
        for &head in &scope {
            let head_ty = self.vars[head as usize].ty;
            for ways in self.ways(head_ty, atom)? {
                let mut all = true;
                for way in &ways {
                    if let Way::Apply(arg) = *way {
                        match self.decided(types.clone(), arg)? {
                            Decided::Inhabited => {}
                            Decided::Empty => all = false,
                            Decided::Unknown => {
                                self.undecided = true;
                                all = false;
                            }
                        }
                    }
                    if !all {
                        break;
                    }
                }
                if all {
                    candidates.push((head, ways));
                }
            }
        }
        self.on_way[shape as usize] = self.on_way[shape as usize].saturating_add(1);
        tasks.push(Task::Leave(shape));
        tasks.push(Task::Join(candidates.len()));
        for (head, ways) in candidates.into_iter().rev() {
            let args: Vec<TypeId> = ways
                .iter()
                .filter_map(|w| match w {
                    Way::Apply(t) => Some(*t),
                    Way::First | Way::Second => None,
                })
                .collect();
            tasks.push(Task::Spine { head, ways });
            for &arg in args.iter().rev() {
                tasks.push(Task::Solve {
                    ctx,
                    ty: arg,
                    depth: depth - 1,
                });
            }
        }
        Ok(())
    }

    /// Returns every pick of one term from each list, made by `make`, at
    /// most `max_terms` of them, and their count: the product of the
    /// lists' counts.
    fn product(
        &mut self,
        lists: &[Found],
        mut make: impl FnMut(&mut Self, &[u32]) -> Result<u32, Error>,
    ) -> Result<Found, Error> {
        let mut count = 1u64;
        let mut overflow = false;
        for list in lists {
            let (product, over) = count.overflowing_mul(list.count);
            count = if over { u64::MAX } else { product };
            overflow |= over || list.overflow;
        }
        if lists.iter().any(|l| l.count == 0) {
            return Ok(Found::default());
        }
        let most = self.options.max_terms as usize;
        let mut terms = Vec::new();
        let mut at = vec![0usize; lists.len()];
        let mut picked = vec![0u32; lists.len()];
        // An odometer over the listed terms, the last list turning fastest.
        'picks: while terms.len() < most {
            if lists.iter().zip(&at).any(|(l, &i)| i >= l.terms.len()) {
                break;
            }
            for (k, list) in lists.iter().enumerate() {
                picked[k] = list.terms[at[k]];
            }
            terms.push(make(self, &picked)?);
            for k in (0..lists.len()).rev() {
                at[k] += 1;
                if at[k] < lists[k].terms.len() {
                    continue 'picks;
                }
                at[k] = 0;
            }
            break;
        }
        Ok(Found {
            terms,
            count,
            overflow,
        })
    }

    /// Returns the terms of the long normal forms given, each built,
    /// checked, and the shortest first.
    fn terms(&mut self, roots: &[u32]) -> Result<Vec<Term>, Error> {
        let mut sized: Vec<(u64, Term)> = Vec::with_capacity(roots.len());
        for &root in roots {
            let (size, term) = self.term(root)?;
            term.check(self.limits, &mut *self.stop)?;
            sized.push((size, term));
        }
        sized.sort_by_key(|(size, _)| *size);
        Ok(sized.into_iter().map(|(_, term)| term).collect())
    }

    /// Builds the term of one long normal form, and returns its size in
    /// nodes as a tree.
    fn term(&mut self, root: u32) -> Result<(u64, Term), Error> {
        let mut builder = self.base.clone();
        let mut var_of: HashMap<u32, (VarId, TermId)> = HashMap::default();
        let mut context = Vec::with_capacity(self.context.len());
        for &v in &self.context {
            let var = &self.vars[v as usize];
            let made = builder.named_variable(var.ty, &var.name);
            var_of.insert(v, made);
            context.push(made.0);
        }
        let mut made: HashMap<u32, (TermId, u64)> = HashMap::default();
        let mut stack = vec![(root, false)];
        while let Some((at, ready)) = stack.pop() {
            if made.contains_key(&at) {
                continue;
            }
            self.tick(1)?;
            let lnf = self.lnf[at as usize].clone();
            let parts: Vec<u32> = match &lnf {
                Lnf::Lambda { body, .. } => vec![*body],
                Lnf::Pair(a, b) => vec![*a, *b],
                Lnf::Unit => Vec::new(),
                Lnf::Spine { from, to, .. } => self.elims[*from as usize..*to as usize]
                    .iter()
                    .filter_map(|e| match e {
                        Elim::Apply(a) => Some(*a),
                        Elim::First | Elim::Second => None,
                    })
                    .collect(),
            };
            if !ready {
                stack.push((at, true));
                stack.extend(parts.iter().rev().map(|&p| (p, false)));
                continue;
            }
            let size_of = |p: u32, made: &HashMap<u32, (TermId, u64)>| made[&p].1;
            let mut var = |v: u32, builder: &mut Builder| {
                *var_of.entry(v).or_insert_with(|| {
                    let var = &self.vars[v as usize];
                    builder.named_variable(var.ty, &var.name)
                })
            };
            let (id, size) = match lnf {
                Lnf::Lambda { var: v, body } => {
                    let (variable, _) = var(v, &mut builder);
                    let id = builder.node(Node::Lambda {
                        variable,
                        body: made[&body].0,
                    });
                    (id, size_of(body, &made).saturating_add(1))
                }
                Lnf::Pair(a, b) => {
                    let id = builder.node(Node::Pair {
                        left: made[&a].0,
                        right: made[&b].0,
                    });
                    let size = size_of(a, &made).saturating_add(size_of(b, &made));
                    (id, size.saturating_add(1))
                }
                Lnf::Unit => (
                    builder.node(Node::True {
                        context: Box::new([]),
                    }),
                    1,
                ),
                Lnf::Spine { head, from, to } => {
                    let (_, mut id) = var(head, &mut builder);
                    let mut size = 1u64;
                    for k in from..to {
                        let node = match self.elims[k as usize] {
                            Elim::Apply(a) => {
                                size = size.saturating_add(size_of(a, &made));
                                Node::Apply {
                                    function: id,
                                    argument: made[&a].0,
                                }
                            }
                            Elim::First => Node::First { term: id },
                            Elim::Second => Node::Second { term: id },
                        };
                        id = builder.node(node);
                        size = size.saturating_add(1);
                    }
                    (id, size)
                }
            };
            made.insert(at, (id, size));
        }
        let (root_id, size) = made[&root];
        let term = builder.finish(Calculus::Simple, context, self.goal, root_id);
        Ok((size, term))
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::{Count, InhabitOptions, Inhabitation, enumerate, inhabit};
    use crate::{Error, Limits, ordinary};

    /// Returns the count and the terms' texts of a sequent's goal.
    fn listed(text: &str, options: &InhabitOptions) -> (Count, Vec<String>) {
        let sequent: ordinary::Sequent = text.parse().expect("a sequent");
        let found =
            enumerate(&sequent, options, &Limits::default(), |_| false).expect("enumerated");
        (
            found.count,
            found.terms.iter().map(ToString::to_string).collect(),
        )
    }

    /// Returns the count of a sequent's goal under the default options.
    fn count(text: &str) -> Count {
        listed(text, &InhabitOptions::default()).0
    }

    #[test]
    fn counts_are_the_known_ones() {
        let exactly = |count| Count::Exactly { count };
        assert_eq!(count("|- a -> a"), exactly(1));
        assert_eq!(count("|- a -> b -> a"), exactly(1));
        assert_eq!(count("|- a -> a -> a"), exactly(2));
        assert_eq!(count("a, a |- a"), exactly(2));
        assert_eq!(count("|- a /\\ b -> b /\\ a"), exactly(1));
        assert_eq!(count("|- (a -> b) -> (b -> c) -> a -> c"), exactly(1));
        assert_eq!(count("|- true"), exactly(1));
        assert_eq!(count("|- a -> b"), exactly(0));
        assert_eq!(count("|- ((a -> b) -> a) -> a"), exactly(0));
        // A function of two arguments of type a, applied in every way:
        // f x x, f x y, f y x, f y y.
        assert_eq!(count("|- (a -> a -> b) -> a -> a -> b"), exactly(4));
        // snd p y, snd p (fst p (snd p y)), …: back and forth along a ↔ b.
        assert_eq!(count("|- (a <-> b) -> b -> a"), Count::Infinite);
        assert_eq!(count("|- (a -> a) -> a -> a"), Count::Infinite);
        assert_eq!(count("|- ((a -> a) -> a) -> a"), Count::Infinite);
    }

    #[test]
    fn terms_come_shortest_first_and_the_count_goes_past_the_list() {
        let (count, terms) = listed("|- a -> a -> a", &InhabitOptions::default());
        assert_eq!(count, Count::Exactly { count: 2 });
        assert_eq!(terms, ["λx. λy. x", "λx. λy. y"]);
        let one = InhabitOptions::default().with_max_terms(1);
        let (count, terms) = listed("|- (a -> a -> b) -> a -> a -> b", &one);
        assert_eq!(count, Count::Exactly { count: 4 });
        assert_eq!(terms.len(), 1);
        let (count, terms) = listed(
            "|- (a -> a) -> a -> a",
            &InhabitOptions::default().with_depth(3),
        );
        assert_eq!(count, Count::Infinite);
        assert_eq!(terms, ["λf. λx. x", "λf. λx. f x", "λf. λx. f (f x)"]);
    }

    #[test]
    fn a_shallow_depth_says_at_least() {
        let shallow = InhabitOptions::default().with_depth(1);
        let (count, _) = listed("|- (a -> b) -> (b -> c) -> a -> c", &shallow);
        assert_eq!(count, Count::AtLeast { count: 0 });
    }

    #[test]
    fn sums_and_false_are_refused() {
        let sequent: ordinary::Sequent = "|- a -> ~~a".parse().expect("a sequent");
        let error = enumerate(
            &sequent,
            &InhabitOptions::default(),
            &Limits::default(),
            |_| false,
        );
        assert!(matches!(
            error,
            Err(Error::NotEnumerable { connective: "⊥" })
        ));
    }

    #[test]
    fn inhabitation_is_decided_with_a_checked_term() {
        let decide = |text: &str| {
            let sequent: ordinary::Sequent = text.parse().expect("a sequent");
            inhabit(&sequent, &Limits::default(), |_| false).expect("decided")
        };
        let Inhabitation::Inhabited(term) = decide("|- a \\/ b -> b \\/ a") else {
            panic!("a ∨ b → b ∨ a has a term");
        };
        assert_eq!(
            term.goal(),
            term.check(&Limits::default(), |_| false)
                .expect("checked")
                .ty(term.root())
        );
        assert!(matches!(
            decide("|- ((a -> b) -> a) -> a"),
            Inhabitation::Empty { .. }
        ));
    }
}
