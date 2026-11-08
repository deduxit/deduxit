// linlog © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The read-backs against a type checker of the tests' own: the type it
//! infers for a term, under the variables the term gives its hypotheses,
//! is the proved sequent's goal, the hypotheses' types are the sequent's
//! formulas as their own printers write them, and every variable is used
//! as the calculus says. The checker recurses over the term unfolded,
//! which the small terms here allow.

use super::*;
use crate::limits::{Limits, Progress};
use crate::occurrences::Member;
use crate::ordinary::Logic;
use crate::proofs::Proof;
use crate::{Mode, Options as SearchOptions, Sequent, ViewOptions, prove};
use std::collections::BTreeMap;

/// A type of the checker, built from the term's types or from a sequent's
/// formulas apart.
#[derive(Clone, Debug, PartialEq, Eq)]
enum T {
    /// An atom by name.
    Atom(String),
    /// `1`
    One,
    /// `⊤`
    Top,
    /// `0`
    Zero,
    /// `⊗`
    Tensor(Box<T>, Box<T>),
    /// `&`, `∧`
    With(Box<T>, Box<T>),
    /// `⊕`, `∨`
    Plus(Box<T>, Box<T>),
    /// `⊸`, `→`
    Arrow(Box<T>, Box<T>),
    /// `!`
    Bang(Box<T>),
}

impl T {
    /// Writes the type as the reading's `Display` writes a formula: every
    /// binary operand in brackets.
    fn show(&self, top: bool) -> String {
        let binary = |l: &T, op: &str, r: &T| {
            let s = format!("{} {op} {}", l.show(false), r.show(false));
            if top { s } else { format!("({s})") }
        };
        match self {
            T::Atom(a) => a.clone(),
            T::One => "1".into(),
            T::Top => "⊤".into(),
            T::Zero => "0".into(),
            T::Tensor(l, r) => binary(l, "⊗", r),
            T::With(l, r) => binary(l, "&", r),
            T::Plus(l, r) => binary(l, "⊕", r),
            T::Arrow(l, r) => binary(l, "⊸", r),
            T::Bang(a) => format!("!{}", a.show(false)),
        }
    }
}

/// Returns a type of the term as the checker's.
fn of_type(term: &Term, id: TypeId) -> T {
    let of = |id| Box::new(of_type(term, id));
    match term.types[id.index()] {
        Type::Atom { atom } => T::Atom(term.atoms[atom as usize].to_string()),
        Type::One => T::One,
        Type::Top => T::Top,
        Type::Zero => T::Zero,
        Type::Tensor { left, right } => T::Tensor(of(left), of(right)),
        Type::With { left, right } => T::With(of(left), of(right)),
        Type::Plus { left, right } => T::Plus(of(left), of(right)),
        Type::Lollipop { from, to } => T::Arrow(of(from), of(to)),
        Type::Bang { of: a } => T::Bang(of(a)),
    }
}

/// Returns the type of an ordinary formula, `¬A` as `A → ⊥` and `A ↔ B`
/// as both implications.
fn of_ordinary(formulas: &crate::ordinary::Formulas, id: crate::ordinary::NodeId) -> T {
    use crate::ordinary::Node as F;
    let of = |id| Box::new(of_ordinary(formulas, id));
    match formulas.node(id) {
        F::Atom(a) => T::Atom(formulas.atom_name(a).to_owned()),
        F::True => T::Top,
        F::False => T::Zero,
        F::Not(a) => T::Arrow(of(a), Box::new(T::Zero)),
        F::And(a, b) => T::With(of(a), of(b)),
        F::Or(a, b) => T::Plus(of(a), of(b)),
        F::Implies(a, b) => T::Arrow(of(a), of(b)),
        F::Iff(a, b) => T::With(
            Box::new(T::Arrow(of(a), of(b))),
            Box::new(T::Arrow(of(b), of(a))),
        ),
    }
}

/// The uses of every free variable of a subterm.
type Uses = BTreeMap<VarId, usize>;

/// The checker: infers types and counts uses, checking the calculus's
/// discipline at every binder.
struct Checker<'t> {
    /// The term.
    term: &'t Term,
}

/// Returns the uses of both of two subterms.
fn sum(mut a: Uses, b: Uses) -> Uses {
    for (v, n) in b {
        *a.entry(v).or_default() += n;
    }
    a
}

impl Checker<'_> {
    /// Returns a variable's type.
    fn var(&self, v: VarId) -> T {
        of_type(self.term, self.term.variables[v.index()].ty)
    }

    /// Checks that a bound variable is used as the calculus allows and
    /// takes it out of the uses.
    fn bind(&self, uses: &mut Uses, v: VarId) {
        let n = uses.remove(&v).unwrap_or(0);
        match self.term.calculus {
            Calculus::Linear => assert_eq!(n, 1, "{v:?} used {n} times"),
            Calculus::Affine => assert!(n <= 1, "{v:?} used {n} times"),
            Calculus::Simple => {}
        }
    }

    /// Returns the uses of two branches that share their context: equal
    /// in the linear calculus, the larger of each otherwise.
    fn share(&self, a: Uses, b: Uses) -> Uses {
        if self.term.calculus == Calculus::Linear {
            assert_eq!(a, b, "the branches of an additive rule share their context");
            return a;
        }
        let mut max = a;
        for (v, n) in b {
            let e = max.entry(v).or_default();
            *e = (*e).max(n);
        }
        max
    }

    /// Returns the type of a subterm and the uses of its free variables.
    #[expect(clippy::too_many_lines, reason = "one arm per node of the calculus")]
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "a type that is no function is a fault, whichever it is"
    )]
    fn infer(&self, id: TermId) -> (T, Uses) {
        let arrow = |t: T| match t {
            T::Arrow(a, b) => (*a, *b),
            other => panic!("not a function: {other:?}"),
        };
        let bang = |t: &T| assert!(matches!(t, T::Bang(_)), "not a `!`: {t:?}");
        let node = &self.term.nodes[id.index()];
        match node {
            Node::Var { variable } => (self.var(*variable), Uses::from([(*variable, 1)])),
            Node::Lambda { variable, body } => {
                let (b, mut uses) = self.infer(*body);
                self.bind(&mut uses, *variable);
                (T::Arrow(Box::new(self.var(*variable)), Box::new(b)), uses)
            }
            Node::Apply { function, argument } => {
                let (f, u) = self.infer(*function);
                let (a, v) = self.infer(*argument);
                let (from, to) = arrow(f);
                assert_eq!(from, a, "an argument of the function's type");
                (to, sum(u, v))
            }
            Node::Tensor { left, right } => {
                let (l, u) = self.infer(*left);
                let (r, v) = self.infer(*right);
                (T::Tensor(Box::new(l), Box::new(r)), sum(u, v))
            }
            Node::LetTensor {
                term,
                left,
                right,
                body,
            } => {
                let (t, u) = self.infer(*term);
                let pair = T::Tensor(Box::new(self.var(*left)), Box::new(self.var(*right)));
                assert_eq!(t, pair);
                let (b, mut v) = self.infer(*body);
                self.bind(&mut v, *left);
                self.bind(&mut v, *right);
                (b, sum(u, v))
            }
            Node::Unit => (T::One, Uses::new()),
            Node::LetUnit { term, body } => {
                let (t, u) = self.infer(*term);
                assert_eq!(t, T::One);
                let (b, v) = self.infer(*body);
                (b, sum(u, v))
            }
            Node::Pair { left, right } => {
                let (l, u) = self.infer(*left);
                let (r, v) = self.infer(*right);
                (T::With(Box::new(l), Box::new(r)), self.share(u, v))
            }
            Node::First { term } | Node::Second { term } => {
                let (t, u) = self.infer(*term);
                let T::With(l, r) = t else {
                    panic!("a projection of a product")
                };
                (
                    if matches!(node, Node::First { .. }) {
                        *l
                    } else {
                        *r
                    },
                    u,
                )
            }
            Node::Left { term, sum: s } | Node::Right { term, sum: s } => {
                let (t, u) = self.infer(*term);
                let whole = of_type(self.term, *s);
                let T::Plus(l, r) = &whole else {
                    panic!("an injection into a sum")
                };
                let left = matches!(node, Node::Left { .. });
                assert_eq!(&t, if left { &**l } else { &**r });
                (whole, u)
            }
            Node::Case {
                term,
                left,
                left_body,
                right,
                right_body,
            } => {
                let (t, u) = self.infer(*term);
                let sum_type = T::Plus(Box::new(self.var(*left)), Box::new(self.var(*right)));
                assert_eq!(t, sum_type);
                let (a, mut v) = self.infer(*left_body);
                let (b, mut w) = self.infer(*right_body);
                assert_eq!(a, b, "both branches of one type");
                self.bind(&mut v, *left);
                self.bind(&mut w, *right);
                (a, sum(u, self.share(v, w)))
            }
            Node::True { context } => {
                let uses = context
                    .iter()
                    .fold(Uses::new(), |u, &c| sum(u, self.infer(c).1));
                (T::Top, uses)
            }
            Node::False {
                context,
                term,
                goal,
            } => {
                let (t, u) = self.infer(*term);
                assert_eq!(t, T::Zero);
                let uses = context.iter().fold(u, |u, &c| sum(u, self.infer(c).1));
                (of_type(self.term, *goal), uses)
            }
            Node::Promote {
                terms,
                variables,
                body,
            } => {
                assert_eq!(terms.len(), variables.len());
                let mut uses = Uses::new();
                for (&t, &v) in terms.iter().zip(variables.iter()) {
                    let (ty, u) = self.infer(t);
                    assert_eq!(ty, self.var(v));
                    bang(&ty);
                    uses = sum(uses, u);
                }
                let (b, mut v) = self.infer(*body);
                for &x in variables.iter() {
                    self.bind(&mut v, x);
                }
                assert!(v.is_empty(), "a promoted body uses only promoted variables");
                (T::Bang(Box::new(b)), uses)
            }
            Node::Derelict { term } => {
                let (t, u) = self.infer(*term);
                let T::Bang(a) = t else {
                    panic!("a dereliction of a `!`")
                };
                (*a, u)
            }
            Node::Discard { term, body } => {
                let (t, u) = self.infer(*term);
                bang(&t);
                let (b, v) = self.infer(*body);
                (b, sum(u, v))
            }
            Node::Copy {
                term,
                left,
                right,
                body,
            } => {
                let (t, u) = self.infer(*term);
                bang(&t);
                assert_eq!(t, self.var(*left));
                assert_eq!(t, self.var(*right));
                let (b, mut v) = self.infer(*body);
                self.bind(&mut v, *left);
                self.bind(&mut v, *right);
                (b, sum(u, v))
            }
        }
    }

    /// Checks the whole term: of the goal's type, its free variables those
    /// of the context, each used as the calculus says.
    fn check(&self, context: &[T], goal: &T) {
        let term = self.term;
        assert_eq!(
            term.context.len(),
            context.len(),
            "a variable per hypothesis"
        );
        for (&v, t) in term.context.iter().zip(context) {
            assert_eq!(&self.var(v), t, "the hypothesis's type");
        }
        assert_eq!(&of_type(term, term.goal), goal, "the goal's type");
        let (ty, mut uses) = self.infer(term.root);
        assert_eq!(&ty, goal, "the term's type is the goal");
        for &v in &term.context {
            self.bind(&mut uses, v);
        }
        assert!(
            uses.is_empty(),
            "no free variable but the context's: {uses:?}"
        );
    }
}

/// Never stops.
fn never(_: Progress) -> bool {
    false
}

/// Returns a term printed, alone or as its judgement, in Unicode.
fn text(term: &Term, judgement: bool) -> String {
    let mut out = String::new();
    let options = Options::default();
    let limits = Limits::default();
    if judgement {
        term.write_judgement(&mut out, &options, &limits, never)
            .unwrap();
    } else {
        term.write(&mut out, &options, &limits, never).unwrap();
    }
    out
}

/// Reads a proof back without the checker.
fn read(proof: &Proof) -> Term {
    let view = ViewOptions::default().with_sides(crate::proofs::Sides::Two);
    let derivation = proof
        .derivation_within(&view, &Limits::default(), never)
        .unwrap();
    derivation.term(&Limits::default(), never).unwrap()
}

/// Reads a proof of intuitionistic linear logic back, checks its term
/// against the sequent as the reading writes it, and returns the term.
fn round_trip(proof: &Proof) -> Term {
    let view = ViewOptions::default().with_sides(crate::proofs::Sides::Two);
    let derivation = proof
        .derivation_within(&view, &Limits::default(), never)
        .unwrap();
    let term = derivation.term(&Limits::default(), never).unwrap();
    let reading = derivation.reading().unwrap();
    let root = derivation.inference(derivation.root());
    let mut context = Vec::new();
    let mut goal = None;
    for &m in root.sequent() {
        let o = m.occ();
        let ty = if reading.position(o) == crate::occurrences::Side::Input {
            let v = term.context[context.len()];
            let ty = of_type(&term, term.variables[v.index()].ty);
            context.push(ty.clone());
            ty
        } else {
            let ty = of_type(&term, term.goal);
            goal = Some(ty.clone());
            ty
        };
        let shown = reading.formula(o).to_string();
        assert_eq!(
            ty.show(true),
            shown,
            "a formula's type as the reading writes it"
        );
    }
    Checker { term: &term }.check(&context, &goal.unwrap());
    term
}

/// Returns the proof the search finds of a sequent in a mode.
fn proved(text: &str, mode: Mode) -> Proof {
    let sequent: Sequent = text.parse().unwrap();
    let outcome = prove(&sequent, mode, &SearchOptions::default()).unwrap();
    let crate::Verdict::Proved(proof) = outcome.verdict else {
        panic!("{text} is provable in {mode}");
    };
    *proof
}

/// The examples the documentation gives read back as it says.
#[test]
fn the_examples_read_back_as_documented() {
    let term = round_trip(&proved("|- a -o b -o a * b", Mode::INTUITIONISTIC));
    assert_eq!(text(&term, false), "λx. λy. x ⊗ y");
    assert_eq!(text(&term, true), "⊢ λx. λy. x ⊗ y : a ⊸ (b ⊸ (a ⊗ b))");
    let term = round_trip(&proved("a -o b, a |- b", Mode::INTUITIONISTIC));
    assert_eq!(text(&term, true), "f : a ⊸ b, x : a ⊢ f x : b");
    let mut ascii = String::new();
    let options = Options::default()
        .with_notation(Notation::Ascii)
        .with_types(true);
    term.write_judgement(&mut ascii, &options, &Limits::default(), never)
        .unwrap();
    assert_eq!(ascii, "f : a -o b, x : a |- f x : b");
    let composition = simple("|- (a -> b) -> (b -> c) -> a -> c", Logic::Intuitionistic);
    assert_eq!(text(&composition, false), "λf. λg. λx. g (f x)");
}

/// Every rule of the calculus reads back to a term whose type is the
/// proved sequent, in intuitionistic linear logic and its affine variant.
#[test]
fn every_rule_reads_back_to_a_term_of_the_sequent() {
    let linear = [
        "a, a -o b |- b",
        "|- a -o b -o a * b",
        "a * b |- b * a",
        "a & b |- b & a",
        "a + b |- b + a",
        "a |- a + b",
        "|- 1",
        "1, a |- a",
        "a, b |- top",
        "0, a |- b",
        "a, b -o 0 |- (b -o c) & top",
        "!a |- a * a",
        "!a |- !a",
        "!a, b |- b",
        "!(a -o b), !a |- !b",
        "!a -o b, !a |- b * a",
        "(a -o c) & (b -o c), a + b |- c",
        "!(a & b) |- a * b",
    ];
    let mut seen = std::collections::HashSet::new();
    for text in linear {
        let term = round_trip(&proved(text, Mode::INTUITIONISTIC));
        assert_eq!(term.calculus, Calculus::Linear, "{text}");
        for node in &term.nodes {
            seen.insert(std::mem::discriminant(node));
        }
    }
    let term = round_trip(&proved("a, b |- a", Mode::INTUITIONISTIC.with_affine()));
    assert_eq!(term.calculus, Calculus::Affine);
    assert_eq!(text(&term, true), "x : a, y : b ⊢ x : a");
    // Every kind of node appears among the terms above.
    assert_eq!(seen.len(), 19, "{} kinds of node met", seen.len());
}

/// A proof with cuts reads back as a term with a redex per cut, of the
/// sequent's type.
#[cfg(feature = "interactive")]
#[test]
fn a_cut_is_a_redex() {
    let proof = cut_proof("a, a -o b |- b", "b", &[0, 1]);
    let term = round_trip(&proof);
    assert_eq!(text(&term, true), "x : a, f : a ⊸ b ⊢ (λy. y) (f x) : b");
    for (text, formula, left) in [
        ("!a |- a * a", "!a", &[0][..]),
        ("!a |- !a", "!a", &[0]),
        ("!a, b |- b", "!a", &[0]),
        ("!a, !a -o b |- b * a", "!a", &[0]),
    ] {
        round_trip(&cut_proof(text, formula, left));
    }
}

/// Returns a proof of an intuitionistic sequent with a cut on `formula`,
/// whose first premise takes the members at `left`; both premises closed
/// by the search.
#[cfg(feature = "interactive")]
fn cut_proof(text: &str, formula: &str, left: &[usize]) -> Proof {
    use crate::proofs::{CutFormula, Interactive};
    let mode = Mode::INTUITIONISTIC;
    let limits = Limits::default();
    let sequent: Sequent = text.parse().unwrap();
    let mut session = Interactive::new(&sequent, mode).unwrap();
    let goal = session.goals().next().unwrap();
    let formula =
        CutFormula::parse_within(formula, mode, &crate::InputOptions::default(), &limits).unwrap();
    session.cut(goal, &formula, left, &limits).unwrap();
    let closed = session.close_all(
        &SearchOptions::default(),
        &ViewOptions::default(),
        &limits,
        never,
    );
    assert!(
        closed
            .iter()
            .all(|(_, c)| c.as_ref().is_ok_and(|c| c.grafted.is_ok())),
        "{text} with a cut on {formula:?}"
    );
    session.proof(&limits, never).unwrap()
}

/// The term of a proof with cuts and the term of the proof cut
/// elimination makes of it have one normal form: elimination is
/// reduction. The proofs keep to `⊸`, `&` and `⊤`, whose reductions are
/// β, the projections of a pair and `true` absorbing its arguments, so
/// that no commuting conversion stands between the two normal forms.
#[cfg(feature = "interactive")]
#[test]
fn cut_elimination_keeps_the_normal_form() {
    use crate::proofs::cut::{Options as CutOptions, eliminate};
    for (text, formula, left) in [
        ("a, a -o b |- b", "b", &[0, 1][..]),
        ("a, a -o b |- b", "a", &[0]),
        ("a, a -o b |- b", "a -o b", &[1]),
        ("a -o b, b -o c |- a -o c", "a -o c", &[0, 1]),
        ("a -o b, b -o c, a |- c", "b", &[0, 2]),
        ("a & b |- b & a", "a & b", &[0]),
        ("a & b |- b & a", "b & a", &[0]),
        ("a, b |- top", "a -o top", &[]),
        ("a -o b -o c, a, b |- c", "b -o c", &[0, 1]),
    ] {
        let proof = cut_proof(text, formula, left);
        assert!(!proof.is_cut_free());
        let with = round_trip(&proof);
        let eliminated = eliminate(
            &proof,
            Mode::INTUITIONISTIC,
            &CutOptions::default(),
            &Limits::default(),
            never,
            |_, _| {},
        )
        .unwrap();
        assert!(eliminated.proof.is_cut_free());
        let without = round_trip(&eliminated.proof);
        assert!(
            Normal::of(&without).is_normal(),
            "{text}: a cut-free proof reads back in normal form"
        );
        assert_eq!(
            Normal::of(&with).normal().canonical(),
            Normal::of(&without).canonical(),
            "{text}: cut on {formula}"
        );
    }
}

/// A term of `⊸`, `&` and `⊤` as a tree, for the normal forms.
#[derive(Clone, Debug)]
enum Normal {
    /// A variable.
    Var(u32),
    /// `λx. M`
    Lam(u32, Box<Normal>),
    /// `M N`
    App(Box<Normal>, Box<Normal>),
    /// `⟨M, N⟩`
    Pair(Box<Normal>, Box<Normal>),
    /// `fst(M)`
    Fst(Box<Normal>),
    /// `snd(M)`
    Snd(Box<Normal>),
    /// `true(M⃗)`
    True(Vec<Normal>),
}

impl Normal {
    /// Unfolds a term of the fragment.
    fn of(term: &Term) -> Self {
        Self::at(term, term.root)
    }

    /// Unfolds a subterm.
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "a node outside the fragment is a fault, whichever it is"
    )]
    fn at(term: &Term, id: TermId) -> Self {
        let of = |id| Box::new(Self::at(term, id));
        match &term.nodes[id.index()] {
            Node::Var { variable } => Self::Var(variable.0),
            Node::Lambda { variable, body } => Self::Lam(variable.0, of(*body)),
            Node::Apply { function, argument } => Self::App(of(*function), of(*argument)),
            Node::Pair { left, right } => Self::Pair(of(*left), of(*right)),
            Node::First { term: t } => Self::Fst(of(*t)),
            Node::Second { term: t } => Self::Snd(of(*t)),
            Node::True { context } => {
                Self::True(context.iter().map(|&c| Self::at(term, c)).collect())
            }
            other => panic!("outside the fragment: {other:?}"),
        }
    }

    /// Adds the free variables to `out`.
    fn free(&self, out: &mut Vec<u32>) {
        match self {
            Self::Var(v) => out.push(*v),
            Self::Lam(x, b) => {
                let mut inner = Vec::new();
                b.free(&mut inner);
                out.extend(inner.into_iter().filter(|v| v != x));
            }
            Self::App(a, b) | Self::Pair(a, b) => {
                a.free(out);
                b.free(out);
            }
            Self::Fst(a) | Self::Snd(a) => a.free(out),
            Self::True(c) => c.iter().for_each(|c| c.free(out)),
        }
    }

    /// Returns the term with `by` for `x`: the read-back's variables are
    /// distinct, and every copy of `by` has its binders renamed apart.
    fn substitute(self, x: u32, by: &Self, fresh: &mut u32) -> Self {
        let mut go = |n: Self| n.substitute(x, by, fresh);
        match self {
            Self::Var(v) if v == x => by.clone().renamed(fresh),
            Self::Var(v) => Self::Var(v),
            Self::Lam(y, b) if y == x => Self::Lam(y, b),
            Self::Lam(y, b) => Self::Lam(y, Box::new(go(*b))),
            Self::App(a, b) => Self::App(Box::new(go(*a)), Box::new(go(*b))),
            Self::Pair(a, b) => Self::Pair(Box::new(go(*a)), Box::new(go(*b))),
            Self::Fst(a) => Self::Fst(Box::new(go(*a))),
            Self::Snd(a) => Self::Snd(Box::new(go(*a))),
            Self::True(c) => Self::True(c.into_iter().map(go).collect()),
        }
    }

    /// Returns the term with every binder renamed to a fresh variable.
    fn renamed(self, fresh: &mut u32) -> Self {
        match self {
            Self::Lam(y, b) => {
                *fresh += 1;
                let z = *fresh;
                let b = b.substitute(y, &Self::Var(z), &mut 0).renamed(fresh);
                Self::Lam(z, Box::new(b))
            }
            Self::Var(v) => Self::Var(v),
            Self::App(a, b) => Self::App(Box::new(a.renamed(fresh)), Box::new(b.renamed(fresh))),
            Self::Pair(a, b) => Self::Pair(Box::new(a.renamed(fresh)), Box::new(b.renamed(fresh))),
            Self::Fst(a) => Self::Fst(Box::new(a.renamed(fresh))),
            Self::Snd(a) => Self::Snd(Box::new(a.renamed(fresh))),
            Self::True(c) => Self::True(c.into_iter().map(|c| c.renamed(fresh)).collect()),
        }
    }

    /// Returns the term after one step of reduction, leftmost outermost,
    /// or the term itself where it has no redex: β, the projections of a
    /// pair, and `true` absorbing its arguments into their free variables.
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "every term that is no redex reduces inside alike"
    )]
    fn step(self, fresh: &mut u32) -> Result<Self, Self> {
        /// Reduces in one subterm of two, the first first.
        fn either(
            a: Normal,
            b: Normal,
            fresh: &mut u32,
            make: fn(Box<Normal>, Box<Normal>) -> Normal,
        ) -> Result<Normal, Normal> {
            match a.step(fresh) {
                Ok(a) => Ok(make(Box::new(a), Box::new(b))),
                Err(a) => match b.step(fresh) {
                    Ok(b) => Ok(make(Box::new(a), Box::new(b))),
                    Err(b) => Err(make(Box::new(a), Box::new(b))),
                },
            }
        }
        match self {
            Self::App(f, a) => match *f {
                Self::Lam(x, b) => Ok(b.substitute(x, &a, fresh)),
                f => either(f, *a, fresh, Self::App),
            },
            Self::Fst(p) => match *p {
                Self::Pair(a, _) => Ok(*a),
                p => p
                    .step(fresh)
                    .map(|p| Self::Fst(Box::new(p)))
                    .map_err(|p| Self::Fst(Box::new(p))),
            },
            Self::Snd(p) => match *p {
                Self::Pair(_, b) => Ok(*b),
                p => p
                    .step(fresh)
                    .map(|p| Self::Snd(Box::new(p)))
                    .map_err(|p| Self::Snd(Box::new(p))),
            },
            Self::Lam(x, b) => b
                .step(fresh)
                .map(|b| Self::Lam(x, Box::new(b)))
                .map_err(|b| Self::Lam(x, Box::new(b))),
            Self::Pair(a, b) => either(*a, *b, fresh, Self::Pair),
            Self::True(c) if c.iter().any(|c| !matches!(c, Self::Var(_))) => {
                let mut free = Vec::new();
                for c in &c {
                    c.free(&mut free);
                }
                Ok(Self::True(free.into_iter().map(Self::Var).collect()))
            }
            other @ (Self::Var(_) | Self::True(_)) => Err(other),
        }
    }

    /// Returns whether the term has no redex.
    fn is_normal(&self) -> bool {
        self.clone().step(&mut 1_000_000).is_err()
    }

    /// Returns the normal form.
    fn normal(self) -> Self {
        let mut fresh = 1_000_000;
        let mut term = self;
        for _ in 0..10_000 {
            match term.step(&mut fresh) {
                Ok(next) => term = next,
                Err(normal) => return normal,
            }
        }
        panic!("no normal form within 10 000 steps");
    }

    /// Returns the term with its binders numbered by depth and `true`'s
    /// arguments sorted, which equal terms up to the names of their bound
    /// variables share.
    fn canonical(&self) -> String {
        /// Writes a subterm under the binders given.
        fn go(n: &Normal, bound: &mut Vec<u32>) -> String {
            match n {
                Normal::Var(v) => match bound.iter().rposition(|b| b == v) {
                    Some(i) => format!("b{i}"),
                    None => format!("f{v}"),
                },
                Normal::Lam(x, b) => {
                    bound.push(*x);
                    let s = format!("λ.{}", go(b, bound));
                    bound.pop();
                    s
                }
                Normal::App(a, b) => format!("({} {})", go(a, bound), go(b, bound)),
                Normal::Pair(a, b) => format!("⟨{}, {}⟩", go(a, bound), go(b, bound)),
                Normal::Fst(a) => format!("fst({})", go(a, bound)),
                Normal::Snd(a) => format!("snd({})", go(a, bound)),
                Normal::True(c) => {
                    let mut names: Vec<String> = c.iter().map(|c| go(c, bound)).collect();
                    names.sort();
                    format!("true({})", names.join(","))
                }
            }
        }
        go(self, &mut Vec::new())
    }
}

/// Returns the term of the LJ derivation `decide` finds of a sequent,
/// checked against the sequent's formulas typed apart; the view that
/// leaves the structural rules out reads back to the same term.
fn simple(text: &str, logic: Logic) -> Term {
    use crate::ordinary::{
        Options as OrdinaryOptions, Sequent as OrdinarySequent, Verdict, decide,
    };
    let sequent: OrdinarySequent = text.parse().unwrap();
    let options = OrdinaryOptions::default().with_logic(logic);
    let outcome = decide(
        &sequent,
        &options,
        &SearchOptions::default(),
        &Limits::default(),
        never,
    )
    .unwrap();
    let Verdict::Valid(derivation) = outcome.verdict else {
        panic!("{text} is valid in {logic:?}");
    };
    let term = derivation.term(&Limits::default(), never).unwrap();
    assert_eq!(term.calculus, Calculus::Simple);
    let f = derivation.formulas();
    let (left, right) = derivation.sequent();
    let context: Vec<T> = left.iter().map(|&h| of_ordinary(f, h)).collect();
    let goal = right.first().map_or(T::Zero, |&c| of_ordinary(f, c));
    Checker { term: &term }.check(&context, &goal);
    let shown = derivation.shown(crate::proofs::Structural::Admissible);
    let admissible = shown.term(&Limits::default(), never).unwrap();
    Checker { term: &admissible }.check(&context, &goal);
    term
}

/// Every rule of LJ reads back to a term of the simply typed calculus
/// whose type is the sequent, in intuitionistic and minimal logic.
#[test]
fn every_rule_of_lj_reads_back_to_a_term_of_the_sequent() {
    for text in [
        "|- (a -> b) -> (b -> c) -> a -> c",
        "a /\\ b |- b /\\ a",
        "a \\/ b |- b \\/ a",
        "|- ~~(a \\/ ~a)",
        "|- (a <-> b) -> (b <-> a)",
        "false |- a",
        "|- a -> true",
        "a, ~a |-",
        "|- (a -> b) -> ~b -> ~a",
        "a -> a -> b, a |- b",
    ] {
        simple(text, Logic::Intuitionistic);
    }
    simple("|- a -> ~~a", Logic::Minimal);
    let term = simple("a, ~a |-", Logic::Intuitionistic);
    assert_eq!(text(&term, true), "x : a, f : a → ⊥ ⊢ f x : ⊥");
}

/// What has no term is refused with its reason.
#[test]
fn what_has_no_term_is_refused() {
    let reason = |proof: &Proof| {
        let derivation = proof.derivation().unwrap();
        match derivation.term(&Limits::default(), never) {
            Err(crate::Error::NoTerm { reason }) => reason,
            other => panic!("refused, not {other:?}"),
        }
    };
    assert_eq!(
        reason(&proved("|- a, ~a", Mode::CLASSICAL)),
        NoTerm::Classical
    );
    let mix = Mode::CLASSICAL.with_mix();
    assert_eq!(reason(&proved("|- 1, 1", mix)), NoTerm::Mix);
    assert_eq!(
        reason(&proved("|- a * b, ~b, ~a", Mode::CYCLIC)),
        NoTerm::Ordered
    );
    let lambek =
        Sequent::parse_ordered("a, a \\ b |- b", crate::Order::Cyclic, &Limits::default()).unwrap();
    let outcome = prove(&lambek, Mode::LAMBEK, &SearchOptions::default()).unwrap();
    let crate::Verdict::Proved(lambek) = outcome.verdict else {
        panic!("provable");
    };
    assert_eq!(reason(&lambek), NoTerm::Ordered);
    let quantified = proved("forall x. p(x) |- p(c)", Mode::INTUITIONISTIC);
    assert_eq!(reason(&quantified), NoTerm::Quantified);
    use crate::ordinary::{Options as OrdinaryOptions, Sequent as OrdinarySequent, Verdict};
    let sequent: OrdinarySequent = "|- a \\/ ~a".parse().unwrap();
    let options = OrdinaryOptions::default().with_logic(Logic::Classical);
    let outcome = crate::ordinary::decide(
        &sequent,
        &options,
        &SearchOptions::default(),
        &Limits::default(),
        never,
    )
    .unwrap();
    let Verdict::Valid(lk) = outcome.verdict else {
        panic!("valid");
    };
    assert!(matches!(
        lk.term(&Limits::default(), never),
        Err(crate::Error::NoTerm {
            reason: NoTerm::Classical
        })
    ));
}

/// A term whose text would pass the output bound is refused before
/// anything is written, and one that `stop` ends is stopped: a pair
/// gives a left rule's term to both components, so the text doubles with
/// every `&` while the derivation grows by a few inferences.
#[test]
fn a_term_past_the_bound_is_refused() {
    let term = round_trip(&proved(
        "a -o b, a |- (b & b) & (b & b)",
        Mode::INTUITIONISTIC,
    ));
    let full = text(&term, false);
    assert_eq!(full.matches("f x").count(), 4, "{full}");
    let limits = Limits::default().with_derivation_bytes(Some(10));
    let mut out = String::new();
    let refused = term.write(&mut out, &Options::default(), &limits, never);
    assert!(
        matches!(
            refused,
            Err(crate::Error::Refused(Refusal::Output { what: "term", .. }))
        ),
        "{refused:?}"
    );
    assert!(out.is_empty(), "nothing written");
    let stopped = term.write(&mut out, &Options::default(), &Limits::default(), |_| true);
    assert!(matches!(
        stopped,
        Err(crate::Error::Refused(Refusal::Stopped { .. }))
    ));
}

/// A type whose text unfolds its shared operands past the bound is
/// refused from the arena's lengths alone: an equivalence nested 64 deep
/// writes its innermost atom 2⁶⁴ times, which building the text to
/// measure it would never finish.
#[test]
fn a_type_past_the_bound_is_refused_unwritten() {
    let mut builder = Builder::new();
    let atom = builder.atom("a");
    let a = builder.ty(Type::Atom { atom });
    let mut nested = a;
    for _ in 0..64 {
        let there = builder.ty(Type::Lollipop {
            from: nested,
            to: a,
        });
        let back = builder.ty(Type::Lollipop {
            from: a,
            to: nested,
        });
        nested = builder.ty(Type::With {
            left: there,
            right: back,
        });
    }
    let (x, root) = builder.variable(nested);
    let term = builder.finish(Calculus::Simple, vec![x], nested, root);
    for options in [Options::default(), Options::default().with_types(true)] {
        let mut out = String::new();
        let refused = term.write_judgement(&mut out, &options, &Limits::default(), never);
        assert!(
            matches!(
                refused,
                Err(crate::Error::Refused(Refusal::Output { what: "term", .. }))
            ),
            "{refused:?}"
        );
        assert!(out.is_empty(), "nothing written");
    }
}

/// A read-back that `stop` ends says so in its own phase.
#[test]
fn a_stopped_read_back_names_its_phase() {
    let proof = proved("a, a -o b |- b", Mode::INTUITIONISTIC);
    let derivation = proof.derivation().unwrap();
    let stopped = derivation.term(&Limits::default(), |_| true);
    assert!(
        matches!(
            stopped,
            Err(crate::Error::Refused(Refusal::Stopped {
                phase: crate::limits::Phase::ReadBack
            }))
        ),
        "{stopped:?}"
    );
}

/// A derivation of any height reads back and prints on a small stack:
/// 120 000 inferences, one above the other, on a thread with 256 KiB.
#[test]
fn any_height_reads_back_on_a_small_stack() {
    use crate::proofs::Node::{Bot, Copy, One, Quest};
    use crate::proofs::NodeId;
    use crate::sequents::{Term as Formula, TermId as FormulaId};
    const ROUNDS: u32 = 40_000;
    let o = |i| Member::from(crate::OccId::new(i));
    let n = NodeId::new;
    // `⊢ ?⊥, 1`, which reads `!1 ⊢ 1`: every round copies the `!1` out,
    // derelicts it and lets its `∗` go.
    let sequent = Sequent {
        terms: vec![
            Formula::Bot,
            Formula::Quest(FormulaId::new(0)),
            Formula::One,
        ],
        roots: vec![FormulaId::new(1), FormulaId::new(2)],
        atoms: vec![],
        antecedents: None,
        cuts: vec![],
        fo: Default::default(),
    };
    let mut nodes = vec![One(o(2))];
    for _ in 0..ROUNDS {
        nodes.push(Bot(o(1), n(nodes.len() as u32 - 1)));
        nodes.push(Copy(o(1), n(nodes.len() as u32 - 1)));
    }
    nodes.push(Quest(o(0), n(nodes.len() as u32 - 1)));
    let root = n(nodes.len() as u32 - 1);
    let build = move || {
        let proof = Proof::new(crate::Forest::new(&sequent).unwrap(), nodes, root)
            .unwrap()
            .with_mode(Mode::INTUITIONISTIC);
        let term = read(&proof);
        let mut out = String::new();
        let limits = Limits::default().with_derivation_bytes(None);
        term.write(&mut out, &Options::default(), &limits, never)
            .unwrap();
        assert_eq!(out.matches("derelict").count(), ROUNDS as usize);
    };
    std::thread::Builder::new()
        .stack_size(256 << 10)
        .spawn(build)
        .unwrap()
        .join()
        .unwrap();
}
