// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The read-backs against a type checker of the tests' own: the type it
//! infers for a term, under the variables the term gives its hypotheses,
//! is the proved sequent's goal, the hypotheses' types are the sequent's
//! formulas as their own printers write them, and every variable is used
//! as the calculus says. The checker recurses over the term unfolded,
//! which the small terms here allow.

use super::*;
use crate::Error;
use crate::limits::{Limits, Progress};
use crate::linear::occurrences::Member;
use crate::linear::proofs::Proof;
use crate::linear::{Mode, Options as SearchOptions, Sequent, ViewOptions, prove};
use crate::ordinary::Logic;
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
        F::Forall(_) | F::Exists(_) => unreachable!("the samples are propositional"),
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
    let view = ViewOptions::default().with_sides(crate::linear::proofs::Sides::Two);
    let derivation = proof
        .derivation_within(&view, &Limits::default(), never)
        .unwrap();
    derivation.term(&Limits::default(), never).unwrap()
}

/// Reads a proof of intuitionistic linear logic back, checks its term
/// against the sequent as the reading writes it, and returns the term.
fn round_trip(proof: &Proof) -> Term {
    let view = ViewOptions::default().with_sides(crate::linear::proofs::Sides::Two);
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
        let ty = if reading.position(o) == crate::linear::occurrences::Side::Input {
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
    agrees(&term);
    term
}

/// Checks a term the tests' checker accepted with the crate's own, and
/// that it reads back from what the printer writes with types shown, in
/// both notations, and from its JSON form.
fn agrees(term: &Term) {
    let limits = Limits::default();
    let typing = term.check(&limits, never).unwrap();
    assert_eq!(typing.ty(term.root), term.goal);
    for notation in [Notation::Unicode, Notation::Ascii] {
        let options = Options::default().with_notation(notation).with_types(true);
        let mut written = String::new();
        term.write_judgement(&mut written, &options, &limits, never)
            .unwrap();
        let read = Term::parse(&written, term.calculus, &limits)
            .unwrap_or_else(|error| panic!("{written}: {error}"));
        read.check(&limits, never)
            .unwrap_or_else(|error| panic!("{written}: {error}"));
        let mut again = String::new();
        read.write_judgement(&mut again, &options, &limits, never)
            .unwrap();
        assert_eq!(again, written);
    }
    #[cfg(feature = "serialize")]
    {
        let json = serde_json::to_string(term).unwrap();
        let mut document = serde_json::Deserializer::from_str(&json);
        let read: Term = crate::wire::upgrade(&mut document, &limits).unwrap();
        assert_eq!(&read, term, "{json}");
    }
}

/// Returns the proof the search finds of a sequent in a mode.
fn proved(text: &str, mode: Mode) -> Proof {
    let sequent: Sequent = text.parse().unwrap();
    let outcome = prove(&sequent, mode, &SearchOptions::default()).unwrap();
    let crate::linear::Verdict::Proved(proof) = outcome.verdict else {
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
    use crate::linear::proofs::{CutFormula, Interactive};
    let mode = Mode::INTUITIONISTIC;
    let limits = Limits::default();
    let sequent: Sequent = text.parse().unwrap();
    let mut session = Interactive::new(&sequent, mode).unwrap();
    let goal = session.goals().next().unwrap();
    let formula = CutFormula::parse_within(
        formula,
        mode,
        &crate::linear::InputOptions::default(),
        &limits,
    )
    .unwrap();
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
    use crate::linear::proofs::cut::{Options as CutOptions, eliminate};
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
            None,
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
#[cfg(feature = "interactive")]
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

#[cfg(feature = "interactive")]
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
        Evidence, Options as OrdinaryOptions, Sequent as OrdinarySequent, Verdict, decide_within,
    };
    let sequent: OrdinarySequent = text.parse().unwrap();
    let options = OrdinaryOptions::default().with_logic(logic);
    let outcome = decide_within(
        &sequent,
        &options,
        &SearchOptions::default(),
        &Limits::default(),
        never,
    )
    .unwrap();
    let Verdict::Valid(Evidence::Derivation(derivation)) = outcome.verdict else {
        panic!("{text} is valid in {logic:?}");
    };
    let term = derivation.term(&Limits::default(), never).unwrap();
    assert_eq!(term.calculus, Calculus::Simple);
    let f = derivation.formulas();
    let (left, right) = derivation.sequent();
    let context: Vec<T> = left.iter().map(|&h| of_ordinary(f, h)).collect();
    let goal = right.first().map_or(T::Zero, |&c| of_ordinary(f, c));
    Checker { term: &term }.check(&context, &goal);
    agrees(&term);
    let shown = derivation.shown(crate::linear::proofs::Structural::Admissible);
    let admissible = shown.term(&Limits::default(), never).unwrap();
    Checker { term: &admissible }.check(&context, &goal);
    agrees(&admissible);
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
    let lambek = Sequent::parse_ordered(
        "a, a \\ b |- b",
        crate::linear::Order::Cyclic,
        &Limits::default(),
    )
    .unwrap();
    let outcome = prove(&lambek, Mode::LAMBEK, &SearchOptions::default()).unwrap();
    let crate::linear::Verdict::Proved(lambek) = outcome.verdict else {
        panic!("provable");
    };
    assert_eq!(reason(&lambek), NoTerm::Ordered);
    let quantified = proved("forall x. p(x) |- p(c)", Mode::INTUITIONISTIC);
    assert_eq!(reason(&quantified), NoTerm::Quantified);
    use crate::ordinary::{
        Evidence, Options as OrdinaryOptions, Sequent as OrdinarySequent, Verdict,
    };
    let sequent: OrdinarySequent = "|- a \\/ ~a".parse().unwrap();
    let options = OrdinaryOptions::default().with_logic(Logic::Classical);
    let outcome = crate::ordinary::decide_within(
        &sequent,
        &options,
        &SearchOptions::default(),
        &Limits::default(),
        never,
    )
    .unwrap();
    let Verdict::Valid(Evidence::Derivation(lk)) = outcome.verdict else {
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
    use crate::linear::proofs::Node::{Bot, Copy, One, Quest};
    use crate::linear::proofs::NodeId;
    use crate::linear::sequents::{Term as Formula, TermId as FormulaId};
    const ROUNDS: u32 = 40_000;
    let o = |i| Member::from(crate::linear::OccId::new(i));
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
        let proof = Proof::new(crate::linear::Forest::new(&sequent).unwrap(), nodes, root)
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

/// `Display` writes the term under the default options, and a term past
/// the default bound as the refusal's message, without failing (which
/// would make `to_string` panic).
#[test]
fn a_term_displays_as_written() {
    let term = round_trip(&proved("a -o b, a |- b", Mode::INTUITIONISTIC));
    assert_eq!(term.to_string(), text(&term, false));

    // `x ⊗ x` nested 64 deep: 2⁶⁴ copies of `x` in 129 nodes.
    let mut builder = Builder::new();
    let atom = builder.atom("a");
    let a = builder.ty(Type::Atom { atom });
    let (x, mut root) = builder.variable(a);
    for _ in 0..64 {
        root = builder.node(Node::Tensor {
            left: root,
            right: root,
        });
    }
    let long = builder.finish(Calculus::Linear, vec![x], a, root);
    let shown = long.to_string();
    assert!(shown.starts_with("<the term is estimated at "), "{shown}");
}

/// Returns the error of reading and then checking a text in a calculus.
fn refused(text: &str, calculus: Calculus) -> Error {
    let limits = Limits::default();
    match Term::parse(text, calculus, &limits) {
        Ok(term) => term
            .check(&limits, never)
            .expect_err("the term is ill typed"),
        Err(error) => error,
    }
}

/// A term whose types do not fit is refused with the reason, where it
/// is read with the place and where it is checked with the node.
#[test]
fn an_ill_typed_term_is_refused_with_its_reason() {
    use Calculus::{Affine, Linear, Simple};
    for (text, calculus, reason) in [
        (
            r"\x:a. x x",
            Simple,
            "the function of an application has type a, which is not a function type",
        ),
        (
            r"\f:a -> b. \x:c. f x",
            Simple,
            "the function takes an argument of type a, and its argument has type c",
        ),
        (r"x : a |- x : b", Simple, "the term has type a, not b"),
        (
            r"\x:a. inl_(b \/ a)(x)",
            Simple,
            "an injection into b ∨ a takes a term of type b here, and its term has type a",
        ),
        (
            r"\x:a \/ b. case x of inl(y) -> y | inr(z) -> z",
            Simple,
            "the branches of a `case` have the types a and b, which differ",
        ),
        (
            r"\x:a. x * x",
            Simple,
            "a pair `M ⊗ N` is no part of the simply typed lambda calculus",
        ),
        (
            r"\x:a. x * x",
            Linear,
            "the variable x is used more than once, and the linear lambda calculus uses every variable exactly once",
        ),
        (
            r"\x:a. \y:b. x",
            Linear,
            "the variable y is not used, and the linear lambda calculus uses every variable exactly once",
        ),
        (
            r"\x:a. x * x",
            Affine,
            "the variable x is used more than once, and the affine lambda calculus uses every variable at most once",
        ),
        (
            r"x : a, y : b |- <x, y>",
            Linear,
            "the variable x is used in one component of a pair",
        ),
        (
            r"y : !a, z : b |- promote y for x in x * z",
            Linear,
            "the body of a `promote` uses the variable z, which is none of its own",
        ),
        (
            r"x : a |- let x be y * z in y",
            Linear,
            "the term a `let` takes apart has type a, which is not a pair type A ⊗ B",
        ),
    ] {
        let Error::IllTyped {
            reason: found,
            node,
            span,
        } = refused(text, calculus)
        else {
            panic!("{text} is ill typed");
        };
        assert!(found.starts_with(reason), "{text}: {found}");
        assert!(
            node.is_some() != span.is_some(),
            "{text}: a node or a place"
        );
    }
    // The types fit and the affine calculus allows what the linear one
    // does not.
    let limits = Limits::default();
    for text in [r"\x:a. \y:b. x", r"x : a, y : b |- <x, y>"] {
        let term = Term::parse(text, Affine, &limits).unwrap();
        term.check(&limits, never).unwrap();
    }
    // A place in the text: the application.
    let Error::IllTyped { span, .. } = refused(r"\x:a. x x", Simple) else {
        panic!("ill typed");
    };
    assert_eq!(span, Some(6..9));
}

/// Text that is no term is refused at its place, with what to write
/// where the calculus needs a type.
#[test]
fn text_that_is_no_term_is_refused_at_its_place() {
    for (text, at, expected) in [
        (r"\x. x", 2, "`:` and the variable's type"),
        (
            r"\x:a. y",
            6,
            "a variable that a binder or the context declares",
        ),
        (r"\x:a. inl(x)", 9, "`_` and the type the term has"),
        (r"\x:a. (x", 8, "`)`"),
        (r"\let:a. x", 1, "a variable"),
        (r"\x:a -> . x", 8, "a type"),
        (r"\x:a. x )", 8, "the end"),
    ] {
        let Error::Parse(error) = refused(text, Calculus::Simple) else {
            panic!("{text} is no term");
        };
        assert_eq!(error.span.start, at, "{text}: {error}");
        assert_eq!(error.expected, [expected], "{text}");
    }
}

/// The syntax reads as documented: application to the left, a binder's
/// body to the right, the types' connectives by their strength, a name's
/// innermost binder, and an index as subscripts or digits.
#[test]
fn the_syntax_reads_as_documented() {
    let limits = Limits::default();
    let options = Options::default().with_types(true);
    for (text, calculus, written) in [
        (
            "λf:a → b → c. λx:a. λy:b. f x y",
            Calculus::Simple,
            "⊢ λf:(a → (b → c)). λx:a. λy:b. f x y : (a → (b → c)) → (a → (b → c))",
        ),
        (
            r"\x:a /\ b \/ ~c. x",
            Calculus::Simple,
            "⊢ λx:((a ∧ b) ∨ (c → ⊥)). x : ((a ∧ b) ∨ (c → ⊥)) → ((a ∧ b) ∨ (c → ⊥))",
        ),
        (
            r"\x:a <-> b. fst(x)",
            Calculus::Simple,
            "⊢ λx:((a → b) ∧ (b → a)). fst(x) : ((a → b) ∧ (b → a)) → (a → b)",
        ),
        (
            "λx:a. λx:b. x",
            Calculus::Simple,
            "⊢ λx:a. λx:b. x : a → (b → b)",
        ),
        (
            "x₁ : a, x2 : b ⊢ ⟨x1, x₂⟩",
            Calculus::Simple,
            "x₁ : a, x₂ : b ⊢ ⟨x₁, x₂⟩ : a ∧ b",
        ),
        (
            "z : ⊥ ⊢ abort_(a → b)(z)",
            Calculus::Simple,
            "z : ⊥ ⊢ abort_(a → b)(z) : a → b",
        ),
        (
            r"\x:!a * b -o c & 1. x",
            Calculus::Linear,
            "⊢ λx:((!a ⊗ b) ⊸ (c & 1)). x : ((!a ⊗ b) ⊸ (c & 1)) ⊸ ((!a ⊗ b) ⊸ (c & 1))",
        ),
        (
            "x : a, y : b, z : c ⊢ x ⊗ y ⊗ z",
            Calculus::Linear,
            "x : a, y : b, z : c ⊢ (x ⊗ y) ⊗ z : (a ⊗ b) ⊗ c",
        ),
        (
            "z : 0, x : a ⊢ false_!b(x; z)",
            Calculus::Linear,
            "z : 0, x : a ⊢ false_!b(x; z) : !b",
        ),
    ] {
        let term = Term::parse(text, calculus, &limits).unwrap_or_else(|e| panic!("{text}: {e}"));
        term.check(&limits, never)
            .unwrap_or_else(|e| panic!("{text}: {e}"));
        let mut judgement = String::new();
        term.write_judgement(&mut judgement, &options, &limits, never)
            .unwrap();
        assert_eq!(judgement, written, "{text}");
    }
}

/// A document whose parts are no term is refused as it is read, and a
/// term whose names would print as another term when it is checked.
#[cfg(feature = "serialize")]
#[test]
fn a_document_that_is_no_term_is_refused() {
    let limits = Limits::default();
    let read = |json: &str| {
        let mut document = serde_json::Deserializer::from_str(json);
        crate::wire::upgrade::<Term, _>(&mut document, &limits)
    };
    let document = |variables: &str, nodes: &str, root: u32| {
        format!(
            r#"{{"version":23,"calculus":"simple","atoms":["a","b"],"types":[{{"kind":"atom","atom":0}},{{"kind":"atom","atom":1}},{{"kind":"lollipop","from":1,"to":0}},{{"kind":"lollipop","from":0,"to":2}}],"variables":{variables},"context":[],"goal":3,"nodes":{nodes},"root":{root}}}"#
        )
    };
    let variables = r#"[{"type":0,"name":"x"},{"type":1,"name":"y"}]"#;
    let nodes = r#"[{"kind":"var","variable":0},{"kind":"lambda","variable":1,"body":0},{"kind":"lambda","variable":0,"body":1}]"#;
    // `λx:a. λy:b. x`, and the same with a type written twice.
    let term = read(&document(variables, nodes, 2)).unwrap();
    assert_eq!(text(&term, true), "⊢ λx. λy. x : a → (b → a)");
    term.check(&limits, never).unwrap();
    // The inner binder's variable has the outer one's name: the text
    // `λx:a. λx:b. x` would name the inner.
    let twins = r#"[{"type":0,"name":"x"},{"type":1,"name":"x"}]"#;
    let term = read(&document(twins, nodes, 2)).unwrap();
    let Err(Error::IllTyped { reason, .. }) = term.check(&limits, never) else {
        panic!("the names capture");
    };
    assert!(
        reason.starts_with("the variable x is used under a binder of another variable named x"),
        "{reason}"
    );
    for (json, message) in [
        (
            document(variables, nodes, 3),
            "`root` names a node past the list",
        ),
        (
            document(
                variables,
                r#"[{"kind":"lambda","variable":0,"body":1},{"kind":"var","variable":0}]"#,
                0,
            ),
            "node 0 names a node that does not come before it",
        ),
        (
            document(r#"[{"type":9,"name":"x"}]"#, nodes, 2),
            "a variable names a type past the list",
        ),
        (
            document(
                r#"[{"type":0,"name":"in"},{"type":1,"name":"y"}]"#,
                nodes,
                2,
            ),
            "variable 0 is named \"in\", which is a keyword or no identifier",
        ),
    ] {
        let Err(Error::Json {
            form,
            message: found,
        }) = read(&json)
        else {
            panic!("{json} is no term");
        };
        assert_eq!(form, "lambda-term");
        assert!(found.starts_with(message), "{found}");
    }
    let above = crate::wire::LEVEL + 1;
    let later =
        document(variables, nodes, 2).replace(r#""version":23"#, &format!(r#""version":{above}"#));
    assert!(matches!(
        read(&later),
        Err(Error::Version { found, .. }) if found == u64::from(above)
    ));
    let small = Limits::default().with_occurrences(Some(5));
    let json = document(variables, nodes, 2);
    let mut reader = serde_json::Deserializer::from_str(&json);
    assert!(matches!(
        crate::wire::upgrade::<Term, _>(&mut reader, &small),
        Err(Error::Refused(Refusal::Occurrences { .. }))
    ));
}

/// A document whose printed text would name another variable is refused
/// by the check, whatever binds the variables, and names spelt two ways
/// are one name.
#[cfg(feature = "serialize")]
#[test]
fn a_capture_is_refused_whatever_binds() {
    let limits = Limits::default();
    let read = |json: &str| {
        let mut document = serde_json::Deserializer::from_str(json);
        crate::wire::upgrade::<Term, _>(&mut document, &limits).unwrap()
    };
    let reason = |term: &Term| {
        let Err(Error::IllTyped { reason, .. }) = term.check(&limits, never) else {
            panic!("{term} is refused");
        };
        reason
    };
    // `z : a ⊗ b ⊢ let z be x ⊗ x in x`, the first `x` meant: the text
    // names the second.
    let pattern = |left: u32, right: u32| {
        format!(
            r#"{{"version":23,"calculus":"affine","atoms":["a","b"],"types":[{{"kind":"atom","atom":0}},{{"kind":"atom","atom":1}},{{"kind":"tensor","left":0,"right":1}},{{"kind":"tensor","left":0,"right":0}}],"variables":[{{"type":2,"name":"z"}},{{"type":0,"name":"x"}},{{"type":1,"name":"x"}},{{"type":3,"name":"w"}}],"context":[{context}],"goal":0,"nodes":[{{"kind":"var","variable":{context}}},{{"kind":"var","variable":1}},{{"kind":"let_tensor","term":0,"left":{left},"right":{right},"body":1}}],"root":2}}"#,
            context = if left == right { 3 } else { 0 }
        )
    };
    assert!(
        reason(&read(&pattern(1, 2)))
            .starts_with("the variable x is used under a binder of another variable named x")
    );
    assert_eq!(
        reason(&read(&pattern(1, 1))),
        "a pattern names the variable x for two of its parts"
    );
    // `λé:a. λé:b. é`, the outer one meant, the inner one's name written
    // with a combining accent.
    let spelt = r#"{"version":23,"calculus":"simple","atoms":["a","b"],"types":[{"kind":"atom","atom":0},{"kind":"atom","atom":1},{"kind":"lollipop","from":1,"to":0},{"kind":"lollipop","from":0,"to":2}],"variables":[{"type":0,"name":"\u00e9"},{"type":1,"name":"e\u0301"}],"context":[],"goal":3,"nodes":[{"kind":"var","variable":0},{"kind":"lambda","variable":1,"body":0},{"kind":"lambda","variable":0,"body":1}],"root":2}"#;
    assert!(reason(&read(spelt)).starts_with("the variable é is used under a binder"));
    // An atom written twice is one atom: `f : a → a, x : a ⊢ f x`.
    let twice = r#"{"version":23,"calculus":"simple","atoms":["a","a"],"types":[{"kind":"atom","atom":0},{"kind":"atom","atom":1},{"kind":"lollipop","from":0,"to":1}],"variables":[{"type":2,"name":"f"},{"type":1,"name":"x"}],"context":[0,1],"goal":0,"nodes":[{"kind":"var","variable":0},{"kind":"var","variable":1},{"kind":"apply","function":0,"argument":1}],"root":2}"#;
    let term = read(twice);
    term.check(&limits, never).unwrap();
    assert_eq!(text(&term, true), "f : a → a, x : a ⊢ f x : a");
}

/// The bounds refuse: a text past the bound on occurrences, a check past
/// its work, and a stop.
#[test]
fn the_reader_and_the_checker_keep_their_bounds() {
    let text = r"\f:a -> a. \x:a. f (f (f (f x)))";
    let small = Limits::default().with_occurrences(Some(8));
    assert!(matches!(
        Term::parse(text, Calculus::Simple, &small),
        Err(Error::Refused(Refusal::Occurrences { limit: 8, .. }))
    ));
    let term = Term::parse(text, Calculus::Simple, &Limits::default()).unwrap();
    let work = Limits::default().with_work(Some(3));
    assert!(matches!(
        term.check(&work, never),
        Err(Error::Refused(Refusal::Work { limit: 3 }))
    ));
    let memory = Limits::default().with_memory_bytes(Some(64));
    assert!(matches!(
        term.check(&memory, never),
        Err(Error::Refused(Refusal::Memory { .. }))
    ));
    // Every variable counts, used or not.
    let many = format!("{}x", r"\x:a. ".repeat(1000));
    let term = Term::parse(&many, Calculus::Simple, &Limits::default()).unwrap();
    let Err(Error::Refused(Refusal::Memory {
        needed_bytes: Some(needed),
        ..
    })) = term.check(&memory, never)
    else {
        panic!("past the bound");
    };
    assert!(needed >= 1000 * 64, "{needed}");
    // A term past a thousand units of work asks the stop.
    let long = format!(r"\x:a. {}x", r"(\y:a. y) (".repeat(400)) + &")".repeat(400);
    let term = Term::parse(&long, Calculus::Simple, &Limits::default()).unwrap();
    assert!(matches!(
        term.check(&Limits::default(), |_| true),
        Err(Error::Refused(Refusal::Stopped {
            phase: crate::limits::Phase::Check
        }))
    ));
}

/// A text nested any depth is read, checked and written on a small
/// stack.
#[test]
fn any_depth_is_read_and_checked_on_a_small_stack() {
    let run = || {
        let limits = Limits::default();
        let depth = 50_000;
        // Binders, brackets, a type nested to the right, and arguments
        // nested to the right.
        let ty = format!("{}a{}", "(a -> ".repeat(depth), ")".repeat(depth));
        let text = format!(
            r"{}\f:a -> a. \z:{ty}. {}f x{}",
            r"\x:a. ".repeat(depth),
            "f (".repeat(depth),
            ")".repeat(depth)
        );
        let term = Term::parse(&text, Calculus::Simple, &limits).unwrap();
        term.check(&limits, never).unwrap();
        let mut written = String::new();
        term.write(&mut written, &Options::default(), &limits, never)
            .unwrap();
        assert!(written.contains("f (f (f x)))"));
    };
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}

/// A derivation of LJ that breaks what the read-back rests on is refused
/// as a defect, naming the inference, never a panic: the checker refuses
/// each of these, and the crate makes none.
#[test]
fn a_derivation_the_read_back_cannot_read_is_refused() {
    use crate::linear::proofs::InfId;
    use crate::ordinary::{Derivation, Inference, Rule, Sequent as OrdinarySequent, Side};
    let sequent: OrdinarySequent = "a /\\ b, a <-> b |- a".parse().unwrap();
    let formulas = sequent.formulas().clone();
    let (and, iff, a) = (sequent.left()[0], sequent.left()[1], sequent.right()[0]);
    let inference = |left: Vec<_>, right: Vec<_>, rule, principal, premises: Vec<u32>| Inference {
        left,
        right,
        rule,
        principal,
        premises: premises.into_iter().map(InfId::new).collect(),
        term: None,
    };
    let refused = |root: (Vec<_>, Vec<_>), inferences| {
        let derivation =
            Derivation::from_parts(Logic::Intuitionistic, formulas.clone(), root, inferences);
        match derivation.term(&Limits::default(), never) {
            Err(crate::Error::ReadBack { reason, .. }) => reason,
            other => panic!("a defect, not {other:?}"),
        }
    };
    // An axiom on a formula no hypothesis gives.
    let reason = refused(
        (vec![], vec![a]),
        vec![inference(vec![], vec![a], Rule::Axiom, None, vec![])],
    );
    assert!(
        reason.contains("a hypothesis no rule below gives"),
        "{reason}"
    );
    // `∧R` with one premise.
    let axiom = inference(vec![a], vec![a], Rule::Axiom, None, vec![]);
    let reason = refused(
        (vec![a], vec![and]),
        vec![
            axiom.clone(),
            inference(
                vec![a],
                vec![and],
                Rule::AndRight,
                Some((Side::Right, 0)),
                vec![0],
            ),
        ],
    );
    assert!(reason.contains("fewer premises than its rule"), "{reason}");
    // A rule of LK alone.
    let reason = refused(
        (vec![a], vec![a]),
        vec![
            axiom.clone(),
            inference(
                vec![a],
                vec![a],
                Rule::ContractRight,
                Some((Side::Right, 0)),
                vec![0],
            ),
        ],
    );
    assert!(reason.contains("no rule of LJ"), "{reason}");
    // `∧L` that names no principal formula.
    let reason = refused(
        (vec![and], vec![a]),
        vec![
            inference(vec![and, a], vec![a], Rule::Axiom, None, vec![]),
            inference(vec![and], vec![a], Rule::AndLeft, None, vec![0]),
        ],
    );
    assert!(reason.contains("first operand"), "{reason}");
    // `↔L` over an arena without the equivalence's implications.
    let reason = refused(
        (vec![iff], vec![a]),
        vec![
            inference(vec![iff, a], vec![a], Rule::Axiom, None, vec![]),
            inference(
                vec![iff],
                vec![a],
                Rule::IffLeft,
                Some((Side::Left, 0)),
                vec![0],
            ),
        ],
    );
    assert!(reason.contains("lacks an implication"), "{reason}");
    // A premise past the inference, which the walk would index.
    let reason = refused(
        (vec![a], vec![a]),
        vec![inference(
            vec![a],
            vec![a],
            Rule::WeakenLeft,
            Some((Side::Left, 0)),
            vec![5],
        )],
    );
    assert!(reason.contains("does not come before it"), "{reason}");
    // A principal position past the sequent.
    let reason = refused(
        (vec![and], vec![a]),
        vec![
            inference(vec![and, a], vec![a], Rule::Axiom, None, vec![]),
            inference(
                vec![and],
                vec![a],
                Rule::AndLeft,
                Some((Side::Left, 5)),
                vec![0],
            ),
        ],
    );
    assert!(reason.contains("past its sequent"), "{reason}");
    // An axiom with nothing right of `⊢`.
    let reason = refused(
        (vec![a], vec![]),
        vec![inference(vec![a], vec![], Rule::Axiom, None, vec![])],
    );
    assert!(reason.contains("no formula right"), "{reason}");
    // `∨R` naming no principal formula.
    let reason = refused(
        (vec![a], vec![a]),
        vec![
            axiom.clone(),
            inference(vec![a], vec![a], Rule::OrRight1, None, vec![0]),
        ],
    );
    assert!(reason.contains("∨R names no principal formula"), "{reason}");
    // `¬L` over an arena without `⊥`.
    let reason = refused(
        (vec![and], vec![a]),
        vec![
            inference(vec![and], vec![a], Rule::Axiom, None, vec![]),
            inference(vec![and], vec![a], Rule::Axiom, None, vec![]),
            inference(
                vec![and],
                vec![a],
                Rule::NotLeft,
                Some((Side::Left, 0)),
                vec![0, 1],
            ),
        ],
    );
    assert!(reason.contains("lacks the ⊥"), "{reason}");
    // A cut whose first premise has nothing right of `⊢`.
    let reason = refused(
        (vec![a], vec![a]),
        vec![
            inference(vec![a], vec![], Rule::WeakenRight, None, vec![]),
            axiom.clone(),
            inference(vec![a], vec![a], Rule::Cut, None, vec![0, 1]),
        ],
    );
    assert!(reason.contains("cut formula is not right"), "{reason}");
}
