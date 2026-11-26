// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Untyped terms, β-reduction and principal types, against references
//! of the tests' own: a normal-order reducer over de Bruijn indices, and
//! the simply typed checker run on every typing of small terms.

use super::{Calculus, End, Node, Notation, Options, ReduceOptions, Strategy, Term, Type, TypeId};
use crate::hash::HashMap;
use crate::{Error, Limits, Progress};

/// The stop that never fires.
const fn never(_: Progress) -> bool {
    false
}

/// Reads a term, or panics.
fn read(text: &str, calculus: Calculus) -> Term {
    Term::parse(text, calculus, &Limits::default()).unwrap_or_else(|e| panic!("{text}: {e}"))
}

#[test]
fn an_untyped_term_reads_back_as_written() {
    let limits = Limits::default();
    for text in [
        r"\x. x",
        r"(\x. x x) (\x. x x)",
        r"\f. \x. f (f x)",
        r"x (\y. y x) z",
        r"x, z |- \y. x y",
    ] {
        let term = read(text, Calculus::Untyped);
        for notation in [Notation::Unicode, Notation::Ascii] {
            let options = Options::default().with_notation(notation);
            let mut written = String::new();
            term.write_judgement(&mut written, &options, &limits, never)
                .unwrap();
            let again = read(&written, Calculus::Untyped);
            assert_eq!(again.to_string(), term.to_string(), "{written}");
            assert_eq!(again.context().len(), term.context().len(), "{written}");
        }
        #[cfg(feature = "serialize")]
        {
            let json = serde_json::to_string(&term).unwrap();
            assert!(json.starts_with(r#"{"version":30,"calculus":"untyped","variables""#));
            let again: Term =
                crate::wire::upgrade(&mut serde_json::Deserializer::from_str(&json), &limits)
                    .unwrap();
            assert_eq!(again, term);
        }
    }
    // The free variables are the context, in the order they first stand.
    let term = read("y (x y)", Calculus::Untyped);
    let names: Vec<&str> = term
        .context()
        .iter()
        .map(|v| term.variables()[v.index()].name())
        .collect();
    assert_eq!(names, ["y", "x"]);
    // A context written declares every free variable.
    assert!(Term::parse("x |- x y", Calculus::Untyped, &limits).is_err());
    // An untyped `λ` takes no type, and a typed one needs its type.
    assert!(Term::parse(r"\x:a. x", Calculus::Untyped, &limits).is_err());
    assert!(Term::parse(r"\x. x", Calculus::Simple, &limits).is_err());
}

#[test]
fn the_untyped_calculi_check_the_uses_of_variables() {
    let limits = Limits::default();
    let verdict = |text: &str, calculus| read(text, calculus).check(&limits, never).map(drop);
    assert!(verdict(r"\x. x x", Calculus::Untyped).is_ok());
    let Err(Error::IllTyped { reason, .. }) = verdict(r"\x. x x", Calculus::UntypedLinear) else {
        panic!("x is used twice");
    };
    assert!(reason.contains("used more than once"), "{reason}");
    assert!(verdict(r"\x. \y. x", Calculus::UntypedLinear).is_err());
    assert!(verdict(r"\x. \y. x", Calculus::UntypedAffine).is_ok());
    assert!(verdict(r"\x. x x", Calculus::UntypedAffine).is_err());
    // A free variable is the context's, and counts as bound there.
    assert!(verdict("x x", Calculus::UntypedLinear).is_err());
    assert!(verdict(r"x (\y. y)", Calculus::UntypedLinear).is_ok());
}

/// A term of the tests' own, with names: a variable, `λ` or an
/// application.
#[derive(Clone, Debug)]
enum Named {
    /// A variable by its name.
    Var(&'static str),
    /// `λx. M`
    Lam(&'static str, Box<Named>),
    /// `M N`
    App(Box<Named>, Box<Named>),
}

/// The names the generator takes, few so that binders shadow and
/// substitutions would capture.
const NAMES: [&str; 3] = ["x", "y", "z"];

/// A generator of terms, a linear congruential one so that the runs are
/// repeatable.
struct Generator(u64);

impl Generator {
    /// Returns a number below `n`.
    fn below(&mut self, n: u64) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) % n
    }

    /// Returns a term of at most `size` nodes.
    fn term(&mut self, size: u64) -> Named {
        let name = NAMES[self.below(3) as usize];
        match if size < 3 { 0 } else { self.below(3) } {
            0 => Named::Var(name),
            1 => Named::Lam(name, Box::new(self.term(size - 1))),
            _ => {
                let left = 1 + self.below(size - 2);
                // Half of the functions are `λ`s, so that redexes abound.
                let function = if left > 2 && self.below(2) == 0 {
                    Named::Lam(name, Box::new(self.term(left - 1)))
                } else {
                    self.term(left)
                };
                Named::App(Box::new(function), Box::new(self.term(size - 1 - left)))
            }
        }
    }
}

impl Named {
    /// Writes the term, every `λ` with the type `types` gives its
    /// variable's place in the order the binders are written, or none.
    fn text(&self, types: &mut dyn Iterator<Item = &'static str>, out: &mut String) {
        match self {
            Self::Var(x) => out.push_str(x),
            Self::Lam(x, body) => {
                out.push_str("(\\");
                out.push_str(x);
                if let Some(ty) = types.next() {
                    out.push(':');
                    out.push_str(ty);
                }
                out.push_str(". ");
                body.text(types, out);
                out.push(')');
            }
            Self::App(f, a) => {
                out.push('(');
                f.text(types, out);
                out.push(' ');
                a.text(types, out);
                out.push(')');
            }
        }
    }

    /// Returns how many binders the term has.
    fn binders(&self) -> usize {
        match self {
            Self::Var(_) => 0,
            Self::Lam(_, body) => 1 + body.binders(),
            Self::App(f, a) => f.binders() + a.binders(),
        }
    }

    /// Adds the free names to `free`, in the order they first stand.
    fn free(&self, bound: &mut Vec<&'static str>, free: &mut Vec<&'static str>) {
        match self {
            Self::Var(x) if !bound.contains(x) && !free.contains(x) => free.push(x),
            Self::Var(_) => {}
            Self::Lam(x, body) => {
                bound.push(x);
                body.free(bound, free);
                bound.pop();
            }
            Self::App(f, a) => {
                f.free(bound, free);
                a.free(bound, free);
            }
        }
    }
}

/// A term with de Bruijn indices, a free variable an index past its
/// binders.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Db {
    /// A variable.
    Var(usize),
    /// `λ. M`
    Lam(Box<Db>),
    /// `M N`
    App(Box<Db>, Box<Db>),
}

impl Db {
    /// Returns the term with every index of a variable free at depth
    /// `cut` or deeper raised by `by` (lowered for `by` negative).
    fn shift(&self, by: isize, cut: usize) -> Self {
        match self {
            Self::Var(i) if *i >= cut => Self::Var(i.checked_add_signed(by).unwrap()),
            Self::Var(i) => Self::Var(*i),
            Self::Lam(body) => Self::Lam(Box::new(body.shift(by, cut + 1))),
            Self::App(f, a) => Self::App(Box::new(f.shift(by, cut)), Box::new(a.shift(by, cut))),
        }
    }

    /// Returns the term with `with` for the variable `index`.
    fn subst(&self, index: usize, with: &Self) -> Self {
        match self {
            Self::Var(i) if *i == index => with.clone(),
            Self::Var(i) => Self::Var(*i),
            Self::Lam(body) => Self::Lam(Box::new(body.subst(index + 1, &with.shift(1, 0)))),
            Self::App(f, a) => Self::App(
                Box::new(f.subst(index, with)),
                Box::new(a.subst(index, with)),
            ),
        }
    }

    /// Returns the term after one step of normal order, or `None` in
    /// normal form.
    fn step(&self) -> Option<Self> {
        match self {
            Self::App(f, a) => {
                if let Self::Lam(body) = &**f {
                    return Some(body.subst(0, &a.shift(1, 0)).shift(-1, 0));
                }
                if let Some(f) = f.step() {
                    return Some(Self::App(Box::new(f), a.clone()));
                }
                a.step().map(|a| Self::App(f.clone(), Box::new(a)))
            }
            Self::Lam(body) => body.step().map(|body| Self::Lam(Box::new(body))),
            Self::Var(_) => None,
        }
    }

    /// Returns a term's de Bruijn form, its context's variables the free
    /// indices, the last declared the nearest.
    fn of(term: &Term) -> Self {
        let mut scope: Vec<super::VarId> = term.context().to_vec();
        Self::at(term, term.root(), &mut scope)
    }

    /// Returns the de Bruijn form of the subterm at `id` under `scope`.
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "an untyped term has no other node"
    )]
    fn at(term: &Term, id: super::TermId, scope: &mut Vec<super::VarId>) -> Self {
        match term.node(id) {
            Node::Var { variable } => {
                Self::Var(scope.iter().rev().position(|v| v == variable).unwrap())
            }
            Node::Lambda { variable, body } => {
                scope.push(*variable);
                let body = Self::at(term, *body, scope);
                scope.pop();
                Self::Lam(Box::new(body))
            }
            Node::Apply { function, argument } => Self::App(
                Box::new(Self::at(term, *function, scope)),
                Box::new(Self::at(term, *argument, scope)),
            ),
            other => panic!("no untyped node: {other:?}"),
        }
    }
}

#[test]
fn normal_order_agrees_with_a_reference() {
    let limits = Limits::default();
    let options = ReduceOptions::default().with_steps(Some(40));
    let mut generator = Generator(84);
    let (mut finished, mut renamed) = (0, 0);
    for _ in 0..400 {
        let named = generator.term(16);
        let mut text = String::new();
        named.text(&mut std::iter::empty(), &mut text);
        let term = read(&text, Calculus::Untyped);
        let mut steps = 0;
        let done = term
            .reduce(&options, &limits, never, |before, step| {
                // Every term a step is taken on reads back as itself,
                // with the free variables a step dropped.
                let mut judgement = String::new();
                before
                    .write_judgement(&mut judgement, &Options::default(), &limits, never)
                    .unwrap();
                let again = read(&judgement, Calculus::Untyped);
                assert_eq!(Db::of(&again), Db::of(before), "{judgement}");
                renamed += usize::from(before.variables().iter().any(|v| v.name().len() > 1));
                assert!(matches!(before.node(step.redex), Node::Apply { .. }));
                steps += 1;
            })
            .unwrap();
        let mut reference = Db::of(&term);
        let mut reference_steps = 0;
        while reference_steps < 40 {
            let Some(next) = reference.step() else { break };
            reference = next;
            reference_steps += 1;
        }
        assert_eq!(done.steps.len(), steps);
        if done.end == End::NormalForm {
            finished += 1;
            assert_eq!(Db::of(&done.term), reference, "{text}");
            assert_eq!(steps, reference_steps, "{text}");
        } else {
            assert!(matches!(done.end, End::Steps { limit: 40, .. }), "{text}");
            assert!(reference.step().is_some(), "{text}");
        }
    }
    // The run reaches normal forms, and renames binders on the way.
    assert!(finished > 300 && renamed > 0, "{finished} {renamed}");
}

#[test]
fn a_reduction_ends_by_its_bounds() {
    let limits = Limits::default();
    // Ω reduces to itself: the reduction is not finished at the bound.
    let omega = read(r"(\x. x x) (\x. x x)", Calculus::Untyped);
    let options = ReduceOptions::default().with_steps(Some(5));
    let done = omega.reduce(&options, &limits, never, |_, _| {}).unwrap();
    assert_eq!(done.end, End::Steps { limit: 5 });
    assert_eq!(done.term.to_string(), omega.to_string());
    // Applicative order reduces the argument first, which never ends;
    // normal order drops it.
    let dropped = read(r"(\x. y) ((\x. x x) (\x. x x))", Calculus::Untyped);
    let applicative = options.with_strategy(Strategy::ApplicativeOrder);
    let done = dropped
        .reduce(&applicative, &limits, never, |_, _| {})
        .unwrap();
    assert!(matches!(done.end, End::Steps { .. }));
    let done = dropped.reduce(&options, &limits, never, |_, _| {}).unwrap();
    assert_eq!(
        (done.end, done.term.to_string()),
        (End::NormalForm, "y".to_owned())
    );
    // A term that grows at every step meets the bound on nodes.
    let growing = read(r"(\x. x x x) (\x. x x x)", Calculus::Untyped);
    let small = ReduceOptions::default().with_nodes(Some(60));
    let done = growing.reduce(&small, &limits, never, |_, _| {}).unwrap();
    assert_eq!(done.end, End::Nodes { limit: 60 });
}

#[test]
fn a_typed_term_keeps_its_type_and_a_binder_is_renamed() {
    let limits = Limits::default();
    let term = read(
        r"y : a |- (\f:a -> a. \y:a. f y) (\x:a. x) y",
        Calculus::Simple,
    );
    let done = term
        .reduce(&ReduceOptions::default(), &limits, never, |_, _| {})
        .unwrap();
    assert_eq!(done.end, End::NormalForm);
    assert_eq!(done.term.to_string(), "y");
    assert_eq!(
        done.term.goal().map(|t| done.term.types()[t.index()]),
        Some(Type::Atom { atom: 0 })
    );
    // The binder `y` would take in the free `y`: it is renamed.
    let capture = read(r"(\x. \y. x) y", Calculus::Untyped);
    let mut seen = Vec::new();
    let done = capture
        .reduce(&ReduceOptions::default(), &limits, never, |before, step| {
            let mut text = String::new();
            let span = before
                .write_marked(&mut text, step.redex, &Options::default(), &limits, never)
                .unwrap();
            seen.push(text[span].to_owned());
        })
        .unwrap();
    assert_eq!(done.term.to_string(), "λy₁. y");
    assert_eq!(seen, ["(λx. λy. x) y"]);
}

/// Returns whether the type `p` of `general` becomes the type `c` of
/// `special` by a substitution of `general`'s atoms, extending the
/// substitution `atoms` holds.
fn instance(
    general: &Term,
    p: TypeId,
    special: &Term,
    c: TypeId,
    atoms: &mut HashMap<u32, TypeId>,
) -> bool {
    match (general.types()[p.index()], special.types()[c.index()]) {
        (Type::Atom { atom }, _) => *atoms.entry(atom).or_insert(c) == c,
        (Type::Lollipop { from, to }, Type::Lollipop { from: f, to: t }) => {
            instance(general, from, special, f, atoms) && instance(general, to, special, t, atoms)
        }
        _ => false,
    }
}

#[test]
fn every_typing_the_checker_accepts_is_an_instance_of_the_principal_type() {
    const TYPES: [&str; 6] = [
        "p",
        "q",
        "(p -> p)",
        "(p -> q)",
        "((p -> p) -> p)",
        "(p -> p -> p)",
    ];
    let limits = Limits::default();
    let mut generator = Generator(85);
    let (mut typed, mut untypable, mut accepted) = (0, 0, 0);
    for _ in 0..120 {
        let named = generator.term(8);
        let mut free = Vec::new();
        named.free(&mut Vec::new(), &mut free);
        let places = named.binders() + free.len();
        if places > 3 {
            continue;
        }
        let mut text = String::new();
        named.text(&mut std::iter::empty(), &mut text);
        let principal = read(&text, Calculus::Untyped).principal_type(&limits, never);
        match &principal {
            Ok(_) => typed += 1,
            Err(Error::IllTyped { .. }) => untypable += 1,
            Err(other) => panic!("{text}: {other}"),
        }
        for choice in 0..TYPES.len().pow(places as u32) {
            let mut picked =
                (0..places).map(|i| TYPES[choice / TYPES.len().pow(i as u32) % TYPES.len()]);
            let mut text = String::new();
            for (i, x) in free.iter().enumerate() {
                text.push_str(if i == 0 { "" } else { ", " });
                text.push_str(&format!("{x} : {}", picked.next().unwrap()));
            }
            text.push_str(" |- ");
            named.text(&mut picked, &mut text);
            let Ok(candidate) = Term::parse(&text, Calculus::Simple, &limits) else {
                continue;
            };
            if candidate.check(&limits, never).is_err() {
                continue;
            }
            accepted += 1;
            let Ok(principal) = &principal else {
                panic!("{text} is typed, though the term has no principal type");
            };
            let mut atoms = HashMap::default();
            let goal = instance(
                principal,
                principal.goal().unwrap(),
                &candidate,
                candidate.goal().unwrap(),
                &mut atoms,
            );
            let context = principal
                .context()
                .iter()
                .zip(candidate.context())
                .all(|(&p, &c)| {
                    let (p, c) = (
                        principal.variables()[p.index()].ty(),
                        candidate.variables()[c.index()].ty(),
                    );
                    instance(principal, p.unwrap(), &candidate, c.unwrap(), &mut atoms)
                });
            assert!(goal && context, "{text} is no instance of {principal}");
        }
    }
    assert!(
        typed > 20 && untypable > 5 && accepted > 100,
        "{typed} {untypable} {accepted}"
    );
}

#[test]
fn principal_types_are_the_textbook_ones() {
    let limits = Limits::default();
    let judgement = |text: &str| {
        let typed = read(text, Calculus::Untyped)
            .principal_type(&limits, never)
            .unwrap();
        let mut out = String::new();
        typed
            .write_judgement(&mut out, &Options::default(), &limits, never)
            .unwrap();
        out
    };
    assert_eq!(
        judgement(r"\x. \y. \z. x z (y z)"),
        "⊢ λx. λy. λz. x z (y z) : (a → (b → c)) → ((a → b) → (a → c))"
    );
    assert_eq!(judgement("f x"), "f : a → b, x : a ⊢ f x : b");
    let Err(Error::IllTyped { reason, .. }) =
        read(r"\x. x x", Calculus::Untyped).principal_type(&limits, never)
    else {
        panic!("x x has no simple type");
    };
    assert!(
        reason.contains("in `x x`") && reason.contains("a = a → b"),
        "{reason}"
    );
}
