// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! System F: terms read, checked and written, type substitution that
//! captures nothing, the side condition of `∀` introduction, and type β.

use super::{
    Calculus, End, Node, Notation, Options, ReduceOptions, Term, TermId, Type, TypeId, TypeVarId,
    VarId, Variable,
};
use crate::{Error, Limits, Progress};

/// The stop that never fires.
const fn never(_: Progress) -> bool {
    false
}

/// Reads a term, or panics.
fn read(text: &str, calculus: Calculus) -> Term {
    Term::parse(text, calculus, &Limits::default()).unwrap_or_else(|e| panic!("{text}: {e}"))
}

/// Returns a term's judgement with the types of its `λ`s, in a notation.
fn judgement(term: &Term, notation: Notation) -> String {
    let options = Options::default().with_notation(notation).with_types(true);
    let mut text = String::new();
    term.write_judgement(&mut text, &options, &Limits::default(), never)
        .unwrap();
    text
}

/// Returns the reason of a term's check, which must fail.
fn refused(term: &Term) -> String {
    match term.check(&Limits::default(), never) {
        Err(Error::IllTyped { reason, .. }) => reason,
        other => panic!("{term}: {other:?}"),
    }
}

#[test]
fn a_term_of_system_f_reads_back_as_written() {
    for (text, calculus) in [
        (r"/\X. \x:X. x : forall X. X -> X", Calculus::SystemF),
        (
            r"\f:forall X. X -> X. f [forall Y. Y -> Y] f",
            Calculus::SystemF,
        ),
        (r"/\X. /\Y. \x:X. \y:Y. <x, y>", Calculus::SystemF),
        (r"/\X. \x:X. x", Calculus::SystemFLinear),
        (
            r"/\X. \x:!X. promote x for y in derelict(y)",
            Calculus::SystemFAffine,
        ),
    ] {
        let term = read(text, calculus);
        term.check(&Limits::default(), never)
            .unwrap_or_else(|e| panic!("{text}: {e}"));
        for notation in [Notation::Unicode, Notation::Ascii] {
            let written = judgement(&term, notation);
            let again = read(&written, calculus);
            assert_eq!(judgement(&again, notation), written);
        }
        #[cfg(feature = "serialize")]
        {
            let limits = Limits::default();
            let json = serde_json::to_string(&term).unwrap();
            assert!(json.starts_with(r#"{"version":31,"#), "{json}");
            let again: Term =
                crate::wire::upgrade(&mut serde_json::Deserializer::from_str(&json), &limits)
                    .unwrap();
            assert_eq!(again, term);
        }
    }
    let term = read(r"/\X. \x:X. x", Calculus::SystemF);
    assert_eq!(
        judgement(&term, Notation::Unicode),
        "⊢ ΛX. λx:X. x : ∀X. X → X"
    );
    // Type abstractions and applications are shown with the types off.
    let term = read(r"\f:forall X. X -> X. f [a]", Calculus::SystemF);
    assert_eq!(term.to_string(), "λf. f [a]");
}

#[test]
fn a_type_application_substitutes_without_capture() {
    // `Y` is an atom here, and the `∀Y` of `f`'s type must not take it in.
    let term = read(r"\f:forall X. forall Y. X -> Y. f [Y]", Calculus::SystemF);
    let typing = term.check(&Limits::default(), never).unwrap();
    let Some(Node::Lambda { body, .. }) = term.nodes().last() else {
        panic!("a λ");
    };
    let applied = typing.ty(*body).unwrap();
    let typed = Term {
        types: typing.types().to_vec(),
        goal: Some(applied),
        ..term.clone()
    };
    let mut text = String::new();
    super::print::write_type(
        &mut text,
        &typed.types,
        &typed.atoms,
        &typed.type_variables,
        &super::print::UNICODE,
        super::print::UNICODE.connectives(Calculus::SystemF),
        applied,
        false,
    )
    .unwrap();
    assert_eq!(text, "∀Y₁. Y → Y₁");
    // The same with the type variable of a `Λ` for `Y`, and the type
    // written back reads as the type the rule made.
    let term = read(
        r"\f:forall X. forall Y. X -> Y. /\Y. f [Y -> Y]",
        Calculus::SystemF,
    );
    let written = judgement(&term, Notation::Unicode);
    assert_eq!(
        written,
        "⊢ λf:(∀X. ∀Y. X → Y). ΛY. f [Y → Y] : (∀X. ∀Y. X → Y) → (∀Y. ∀Y₁. (Y → Y) → Y₁)"
    );
    let again = read(&written, Calculus::SystemF);
    again.check(&Limits::default(), never).unwrap();
    // Opening `∀X. ∀Y. X → Y` at `∀Z. Z` keeps every index in place.
    let term = read(
        r"\f:forall X. forall Y. X -> Y. f [forall Z. Z] : (forall X. forall Y. X -> Y) -> forall Y. (forall Z. Z) -> Y",
        Calculus::SystemF,
    );
    term.check(&Limits::default(), never).unwrap();
}

#[test]
fn equal_types_are_one_whatever_their_bound_names() {
    // A type is printed with the names of the first equal one made.
    let term = read(
        r"\f:forall X. X -> X. f : (forall X. X -> X) -> forall Y. Y -> Y",
        Calculus::SystemF,
    );
    term.check(&Limits::default(), never).unwrap();
    assert_eq!(term.to_string(), "λf. f");
    // A wrong argument is refused with both types, as it is read.
    let text = r"\f:forall X. X -> X. \x:a. f [b] x";
    let Err(Error::IllTyped { reason, .. }) =
        Term::parse(text, Calculus::SystemF, &Limits::default())
    else {
        panic!("{text} is ill typed");
    };
    assert!(
        reason.contains("type b, and its argument has type a"),
        "{reason}"
    );
}

/// Returns a term of System F made by hand: the context `x` of type `X`
/// (a type variable), and `ΛX. body` with `x` used in the body.
fn outside_its_abstraction() -> Term {
    Term {
        calculus: Calculus::SystemF,
        atoms: Vec::new(),
        types: vec![
            Type::Variable {
                variable: TypeVarId(0),
            },
            Type::Bound { index: 0 },
            Type::Forall {
                variable: TypeVarId(0),
                body: TypeId(1),
            },
        ],
        variables: vec![Variable {
            ty: Some(TypeId(0)),
            name: "x".into(),
        }],
        type_variables: vec!["X".into()],
        context: vec![VarId(0)],
        goal: Some(TypeId(2)),
        nodes: vec![
            Node::Var { variable: VarId(0) },
            Node::TypeLambda {
                variable: TypeVarId(0),
                body: TermId(0),
            },
        ],
        root: TermId(1),
    }
}

#[test]
fn a_type_variable_free_in_a_hypothesis_is_not_generalized() {
    // `x : X ⊢ ΛX. x : ∀X. X` would prove `X → ∀X. X`.
    let reason = refused(&outside_its_abstraction());
    assert!(
        reason.starts_with("the type variable X is named outside"),
        "{reason}"
    );
}

#[test]
fn a_type_variable_bound_twice_is_refused() {
    // `ΛX. λx:X. ΛX. x`, one type variable for both abstractions: the
    // inner would generalize `x`, bound outside it, to `∀X. X`.
    let variable = |n| Type::Variable {
        variable: TypeVarId(n),
    };
    let term = Term {
        calculus: Calculus::SystemF,
        atoms: Vec::new(),
        types: vec![
            variable(0),
            Type::Bound { index: 0 },
            Type::Forall {
                variable: TypeVarId(0),
                body: TypeId(1),
            },
            Type::Lollipop {
                from: TypeId(0),
                to: TypeId(2),
            },
            Type::Lollipop {
                from: TypeId(1),
                to: TypeId(2),
            },
            Type::Forall {
                variable: TypeVarId(0),
                body: TypeId(4),
            },
        ],
        variables: vec![Variable {
            ty: Some(TypeId(0)),
            name: "x".into(),
        }],
        type_variables: vec!["X".into()],
        context: Vec::new(),
        goal: Some(TypeId(5)),
        nodes: vec![
            Node::Var { variable: VarId(0) },
            Node::TypeLambda {
                variable: TypeVarId(0),
                body: TermId(0),
            },
            Node::Lambda {
                variable: VarId(0),
                body: TermId(1),
            },
            Node::TypeLambda {
                variable: TypeVarId(0),
                body: TermId(2),
            },
        ],
        root: TermId(3),
    };
    let reason = refused(&term);
    assert!(reason.starts_with("two type abstractions bind"), "{reason}");
}

#[test]
fn a_type_variable_of_a_name_another_binds_is_refused() {
    // `ΛX. ΛX. λy:X. y`, the outer `X` meant: its text names the inner.
    let mut term = read(r"/\X. /\Z. \y:X. y", Calculus::SystemF);
    term.type_variables[1] = "X".into();
    let reason = refused(&term);
    assert!(
        reason.contains("under a type abstraction of another"),
        "{reason}"
    );
    // An atom's name for a `Λ`'s variable.
    let term = read(r"\a:X. /\X. a", Calculus::SystemF);
    let reason = refused(&term);
    assert!(reason.contains("has the name of an atom"), "{reason}");
}

#[test]
fn a_term_without_its_types_is_refused_in_system_f() {
    for calculus in [Calculus::SystemF, Calculus::SystemFLinear] {
        let error = Term::parse(r"\x. x", calculus, &Limits::default()).unwrap_err();
        let Error::Parse(error) = error else {
            panic!("{error}");
        };
        assert!(
            error.hint.unwrap_or("").contains("infers no types"),
            "{error}"
        );
    }
    // The simple calculus has no `Λ`, and a linear System F term uses each
    // variable once.
    assert!(Term::parse(r"/\X. \x:X. x", Calculus::Simple, &Limits::default()).is_err());
    let twice = read(r"/\X. \x:X. \y:X. x", Calculus::SystemFLinear);
    assert!(refused(&twice).contains("not used"));
}

#[test]
fn type_beta_steps_reach_the_normal_form() {
    let limits = Limits::default();
    let term = read(r"y:a |- (/\X. \x:X. x) [a] y", Calculus::SystemF);
    let mut seen = Vec::new();
    let done = term
        .reduce(&ReduceOptions::default(), &limits, never, |before, _| {
            seen.push(before.to_string());
        })
        .unwrap();
    assert_eq!(done.end, End::NormalForm);
    assert_eq!(done.term.to_string(), "y");
    assert_eq!(seen, ["(ΛX. λx. x) [a] y", "(λx. x) y"]);
    // A type put under an abstraction of its name's variable renames it.
    let term = read(r"/\X. (/\Y. /\X. \z:Y. \w:X. z) [X]", Calculus::SystemF);
    let done = term
        .reduce(&ReduceOptions::default(), &limits, never, |_, _| {})
        .unwrap();
    assert_eq!(done.end, End::NormalForm);
    assert_eq!(
        judgement(&done.term, Notation::Unicode),
        "⊢ ΛX. ΛX₁. λz:X. λw:X₁. z : ∀Y. ∀X. Y → (X → Y)"
    );
}

#[cfg(feature = "serialize")]
#[test]
fn a_document_with_a_loose_bound_index_is_refused() {
    let json = r#"{"version": 31, "calculus": "system_f", "atoms": [], "types":
        [{"kind": "bound", "index": 0}], "variables": [{"type": 0, "name": "x"}],
        "type_variables": [], "context": [0], "goal": 0, "nodes": [{"kind": "var",
        "variable": 0}], "root": 0}"#;
    let read: Result<Term, _> = crate::wire::upgrade(
        &mut serde_json::Deserializer::from_str(json),
        &Limits::default(),
    );
    assert!(read.unwrap_err().to_string().contains("`bound` index"));
}
