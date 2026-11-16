// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Tests of the review of written tableaux.

use super::{Standing, TableauReview, WrittenTableau};
use crate::linear::search::generate::Rng;
use crate::ordinary::{Logic, Method, Options, Search, Sequent, Verdict, decide};
use crate::rules::{Calculus, TABLEAU};
use crate::{Error, Limits, Refusal};

/// Returns the rule file of the tableau, read.
fn calculus() -> Calculus {
    let mut json = serde_json::Deserializer::from_str(TABLEAU);
    crate::wire::upgrade::<Calculus, _>(&mut json, &Limits::default()).unwrap()
}

/// Returns the review of a tableau's text within the default limits.
fn review(text: &str) -> TableauReview {
    let tableau = WrittenTableau::parse_within(text, &Limits::default()).unwrap();
    tableau
        .review(&calculus(), None, &Limits::default(), |_| false)
        .unwrap()
}

/// Returns the text of the tableau the method `tableau` makes for a
/// sequent, and whether the sequent is valid.
fn method(sequent: &Sequent) -> (String, bool) {
    let options = Options::default()
        .with_logic(Logic::Classical)
        .with_method(Some(Method::Tableau));
    let outcome = decide(sequent, &options).unwrap();
    let Search::Tableau(tableau) = &outcome.search else {
        unreachable!("the method tableau makes a tableau")
    };
    (
        tableau.to_string(),
        matches!(outcome.verdict, Verdict::Valid(_)),
    )
}

/// Returns the sequents the tests read: some by hand, with `↔`, and
/// generated ones.
fn sequents() -> Vec<Sequent> {
    let mut all: Vec<Sequent> = [
        "p \\/ q |- q \\/ p",
        "a -> b |- b -> a",
        "p <-> q |- q <-> p",
        "|- (p <-> q) \\/ (p <-> ~q)",
        "p /\\ (q \\/ r) |- (p /\\ q) \\/ (p /\\ r)",
        "|- ((a -> b) -> a) -> a",
        "~(a /\\ b) |- ~a \\/ ~b",
        "a <-> b, b <-> c |- a <-> c",
        "true, ~false |- a",
        "a /\\ a |- a",
    ]
    .into_iter()
    .map(|text| text.parse().unwrap())
    .collect();
    let mut rng = Rng::new(68);
    // A sequent of no formula has a tableau of no line, which no one
    // writes.
    all.extend(
        (0..150)
            .map(|_| super::super::tests::generated_sequent(&mut rng))
            .filter(|s| !(s.left().is_empty() && s.right().is_empty())),
    );
    all
}

#[test]
fn the_method_s_tableaux_read_back_with_their_verdicts() {
    for sequent in sequents() {
        let (text, valid) = method(&sequent);
        // As drawn, in ASCII, and as a list.
        let ascii = text.replace("├─", "|-").replace("└─", "`-");
        let list = text
            .replace("├─", "- ")
            .replace("└─", "- ")
            .replace('│', " ");
        for written in [text, ascii, list] {
            let review = review(&written);
            let wanted = if valid {
                Standing::Closed
            } else {
                Standing::Open
            };
            assert_eq!(review.standing, wanted, "{sequent}\n{written}\n{review:?}");
            assert_eq!(review.countermodel.is_some(), !valid, "{sequent}");
            assert_eq!(review.sequent.to_string(), sequent.to_string());
        }
    }
}

#[test]
fn a_list_reads_as_the_drawn_tree() {
    let text = "\
1 T p ∨ q
2 F q ∨ p
3 F q        F∨ 2
4 F p        F∨ 2
- 5 T p      T∨ 1
  × 4, 5
- 6 T q      ∨L 1
  ×
";
    assert_eq!(review(text).standing, Standing::Closed);
    // The branches in the other order, and the equivalence by its sides'
    // values, as the textbooks expand it.
    let swapped = "\
1 T p <-> q
2 F q <-> p
- 3 F p 1
  4 F q 1
  - 5 T q 2
    6 F p 2
    open
  - 7 F q 2
    8 T p 2
    × 3, 8
- 9 T p 1
  10 T q 1
  - 11 F q 2
    12 T p 2
    × 10, 11
  - 13 T q 2
    14 F p 2
    × 9, 14
";
    let review = review(swapped);
    assert_eq!(review.standing, Standing::Wrong, "{review:?}");
    assert_eq!(
        review.message,
        "the branch ending at line 6 is marked open, and it closes on line 5 (T q) and line 4 \
         (F q)"
    );
    assert_eq!(review.text_line, Some(7));
    assert_eq!(
        self::review(&swapped.replace("    open", "    × 4, 5")).standing,
        Standing::Closed
    );
}

#[test]
fn every_line_changed_is_found_at_or_before_it() {
    for sequent in sequents().into_iter().take(40) {
        let (text, _) = method(&sequent);
        let rows: Vec<&str> = text.lines().collect();
        for (row, line) in rows.iter().enumerate() {
            // A line that expands another, and the line it expands.
            let Some(cited) = line
                .split_whitespace()
                .last()
                .and_then(|word| word.parse::<u32>().ok())
            else {
                continue;
            };
            if line
                .trim_start_matches(['│', '├', '└', '─', ' '])
                .starts_with('×')
            {
                continue;
            }
            // Its formula changed to an atom no line has, which expands
            // nothing: the step is wrong whatever else the branch holds.
            let words: Vec<&str> = line.split_whitespace().collect();
            let Some(number) = words.iter().position(|w| w.parse::<u32>().is_ok()) else {
                continue;
            };
            let layout = &line[..line.find(words[number]).unwrap()];
            let flipped = format!(
                "{layout}{} {} zz  {} {cited}",
                words[number],
                words[number + 1],
                words[words.len() - 2]
            );
            let mut changed = rows.clone();
            changed[row] = &flipped;
            let review = self::review(&changed.join("\n"));
            assert_eq!(review.standing, Standing::Wrong, "{flipped}\n{review:?}");
            let at = review.text_line.unwrap() as usize;
            let cited_row = rows
                .iter()
                .position(|l| {
                    l.trim_start_matches(['│', '├', '└', '─', ' '])
                        .starts_with(&format!("{cited} "))
                })
                .unwrap();
            assert!(
                cited_row < at && at <= row + 1,
                "{}\n{review:?}",
                changed.join("\n")
            );
        }
    }
}

#[test]
fn each_fault_is_said_with_its_line() {
    let cases = [
        (
            "1 T p ∧ q\n2 F p\n3 T p 1\n4 T r 1\n",
            "line 4: T∧ on line 1 (T p ∧ q) adds T p and T q, not T r",
        ),
        (
            "1 F p → q\n2 T q 1\n",
            "line 2: F→ on line 1 (F p → q) adds T p and F q, not T q",
        ),
        (
            "1 T p ∨ q\n2 T p 1\n",
            "line 2: T∨ on line 1 (T p ∨ q) splits the branch in two, one that starts with T p \
             and one with T q",
        ),
        (
            "1 T p ∧ q\n- 2 T p 1\n- 3 T q 1\n",
            "line 2: T∧ on line 1 (T p ∧ q) adds T p and T q to the branch and does not split it",
        ),
        (
            "1 T p ∨ q\n2 F p\n- 3 T p 1\n  × 2, 3\n- 4 T p 1\n",
            "line 4: T∨ on line 1 (T p ∨ q) splits the branch into one that starts with T p and \
             one with T q; this branch starts with T p",
        ),
        (
            "1 T p\n2 F q\n3 T p 1\n",
            "line 3 expands line 1 (T p), which is an atom, which no rule expands",
        ),
        (
            "1 F ~p\n2 T p 3\n3 T p 1\n",
            "line 2 expands line 3, which comes after it",
        ),
        (
            "1 F ~p\n2 T p 9\n",
            "line 2 expands line 9, which the tableau does not have",
        ),
        (
            "1 F ~p\n2 T p T¬ 1\n",
            "line 2 names the rule T¬, but line 1 (F ¬p) is expanded by F¬",
        ),
        (
            "1 T p\n2 F q\n×\n",
            "the branch ending at line 2 is marked ×, and nothing closes it: no formula on it is \
             both true and false, and neither ⊥ is true nor ⊤ false",
        ),
        (
            "1 T p\n2 F q\n× 1, 2\n",
            "the × under line 2 names line 1 (T p) and line 2 (F q), which do not close the \
             branch: a branch closes on one formula true and false (two lines), on ⊥ true or on \
             ⊤ false (one line)",
        ),
        (
            "1 T p ∨ q\n2 F p\nopen\n",
            "the branch ending at line 2 is marked open, and line 1 (T p ∨ q) is not expanded \
             on it",
        ),
        (
            "1 F ~p\n2 T p 1\n3 T q\n",
            "line 3 names no line it expands: only the first lines, the sequent's, stand \
             without one",
        ),
    ];
    for (text, message) in cases {
        let review = review(text);
        assert_eq!(review.standing, Standing::Wrong, "{text}");
        assert_eq!(review.message, message, "{text}");
    }
}

#[test]
fn a_right_tableau_reads_as_written() {
    // A negated atom that reads like a rule's name ends a formula; one
    // result of `F→` alone, and its two results apart, as Smullyan's
    // rule A allows; `x` with a space of another width.
    for text in [
        "1 T P /\\ (Q /\\ ~R)\n2 F Q\n3 T P 1\n4 T Q /\\ ~R 1\n5 T Q 4\n6 T ~R 4\nx 5, 2\n",
        "1 F a -> (b -> a)\n2 T a 1\n3 F b -> a 1\n4 F a 3\nx 2, 4\n",
        "1 F a -> (c -> b)\n2 T a 1\n3 F c -> b 1\n4 T c 3\n5 F b\u{3000}F→ 3\n",
        "1 T p\n2 F p\n×\u{3000}1,\u{3000}2\n",
    ] {
        let review = review(text);
        assert_ne!(review.standing, Standing::Wrong, "{text}\n{review:?}");
    }
}

#[test]
fn faults_on_odd_input_are_said_not_panicked() {
    // A number past 2³² after a wide space, a first line that cites one
    // with a sequent given, a formula cut short before its line's end.
    let refused =
        WrittenTableau::parse_within("1 T p\n2 F p\n×\u{3000}99999999999\n", &Limits::default());
    assert!(matches!(refused, Err(Error::Parse(_))));
    let tableau: WrittenTableau = "1 T p 1\n".parse().unwrap();
    let sequent: Sequent = "p |- p".parse().unwrap();
    let cited = tableau
        .review(&calculus(), Some(&sequent), &Limits::default(), |_| false)
        .unwrap();
    assert_eq!(cited.standing, Standing::Wrong);
    let Err(Error::Parse(error)) = WrittenTableau::parse_within(
        "1 T p /\\ q\n2 F p\n3 T p /\\ 1\n4 T q 1\n",
        &Limits::default(),
    ) else {
        panic!("read")
    };
    assert_eq!(error.line, 3);
    // The rule as a whole, whatever the arena holds of its parts.
    let whole = review("1 T p <-> q\n2 F p -> q\n3 T q 1\n");
    assert_eq!(
        whole.message,
        "line 3: T↔ on line 1 (T p ↔ q) splits the branch in two, one that starts with T p and \
         T q and one with F p and F q"
    );
}

#[test]
fn a_branch_left_unmarked_is_unfinished() {
    let cases = [
        (
            "1 T p ∨ q\n2 F p\n",
            "every step so far follows the rules; the branch ending at line 2 is not finished: \
             line 1 (T p ∨ q) is not expanded on it",
        ),
        (
            "1 T p\n2 F p\n",
            "every step so far follows the rules; the branch ending at line 2 is not marked: it \
             closes on line 1 (T p) and line 2 (F p), so mark it ×",
        ),
        (
            "1 T p\n2 F q\n",
            "every step so far follows the rules; the branch ending at line 2 is finished and \
             nothing closes it, so mark it open",
        ),
    ];
    for (text, message) in cases {
        let review = review(text);
        assert_eq!(review.standing, Standing::Unfinished, "{text}");
        assert_eq!(review.message, message);
        assert_eq!(review.text_line, Some(2));
    }
}

#[test]
fn an_open_branch_gives_a_checked_countermodel() {
    let review =
        review("1 T a → b\n2 F b → a\n3 T b 2\n4 F a 2\n- 5 F a 1\n  open\n- 6 T b 1\n  open\n");
    assert_eq!(review.standing, Standing::Open);
    assert_eq!(review.text_line, Some(6));
    let countermodel = review.countermodel.unwrap();
    assert_eq!(countermodel.model.worlds[0].atoms, ["b"]);
}

#[test]
fn the_sequent_given_is_the_first_lines() {
    let tableau: WrittenTableau = "1 T p\n2 F p\n×\n".parse().unwrap();
    let limits = Limits::default();
    let right: Sequent = "p |- p".parse().unwrap();
    let other: Sequent = "p |- q".parse().unwrap();
    let reviewed = |sequent: &Sequent| {
        tableau
            .review(&calculus(), Some(sequent), &limits, |_| false)
            .unwrap()
    };
    assert_eq!(reviewed(&right).standing, Standing::Closed);
    let wrong = reviewed(&other);
    assert_eq!(wrong.standing, Standing::Wrong);
    assert_eq!(
        wrong.message,
        "the first lines give the sequent p ⊢ p, and the tableau is to be of p ⊢ q"
    );
}

#[test]
fn a_layout_without_a_branch_is_refused_where_it_fails() {
    for (text, line, column) in [
        ("1 T p ∨ q\n   2 F p\n", 2, 4),
        ("1 T p\n- ×\n", 2, 3),
        ("- 1 T p\n", 1, 3),
        ("1 T p\n×\n2 F q\n", 3, 1),
        ("1 T p\n1 F q\n", 2, 1),
        ("1 p\n", 1, 3),
        ("1 T p ⊢ q\n", 1, 7),
    ] {
        let Err(Error::Parse(error)) = WrittenTableau::parse_within(text, &Limits::default())
        else {
            panic!("{text} was read")
        };
        assert_eq!(
            (error.line, error.column),
            (line, column),
            "{text}: {error}"
        );
    }
}

#[test]
fn a_tableau_past_the_memory_bound_is_refused() {
    let (text, _) = method(
        &"p /\\ (q \\/ r) |- (p /\\ q) \\/ (p /\\ r)"
            .parse()
            .unwrap(),
    );
    let tableau: WrittenTableau = text.parse().unwrap();
    let limits = Limits::default().with_memory_bytes(Some(256));
    let refused = tableau.review(&calculus(), None, &limits, |_| false);
    assert!(
        matches!(refused, Err(Error::Refused(Refusal::Memory { .. }))),
        "{refused:?}"
    );
}
