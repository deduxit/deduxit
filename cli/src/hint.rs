// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use crate::Explained;
use crate::argument_parsing::{Cli, InputFormat};
use clap::builder::StyledStr;
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{ArgMatches, CommandFactory};
use deduxit::Error;

/// Returns clap's error with a tip of what to write where the command
/// line is a sequent that names no command, or a sequent that the shell
/// split into words because it was not quoted. `args` is the command line,
/// the program's name first.
pub(crate) fn command_line(mut error: clap::Error, args: &[String]) -> clap::Error {
    let tip = match error.kind() {
        ErrorKind::InvalidSubcommand if error.get(ContextKind::SuggestedSubcommand).is_none() => {
            no_command(&error, args)
        }
        ErrorKind::UnknownArgument => split(&error, args),
        _ => None,
    };
    if let Some(tip) = tip {
        error.insert(
            ContextKind::Suggested,
            ContextValue::StyledStrs(vec![StyledStr::from(tip)]),
        );
    }
    error
}

/// Returns the tip for a command line whose first word is no command:
/// that `prove` decides a sequent.
fn no_command(error: &clap::Error, args: &[String]) -> Option<String> {
    let Some(ContextValue::String(word)) = error.get(ContextKind::InvalidSubcommand) else {
        return None;
    };
    let at = args.iter().skip(1).position(|arg| arg == word)? + 1;
    let words: Vec<String> = args[at..].iter().map(|w| shell_word(w)).collect();
    Some(format!(
        "to decide a sequent, run deduxit prove {}",
        words.join(" ")
    ))
}

/// Returns the tip for an argument a command does not take, where it is a
/// word of a sequent the shell split: the sequent quoted. The command line
/// is parsed again without each word refused, at most once per word, until
/// the command reads a sequent; the sequent is then the words of the
/// command line from the first of those up to a flag.
fn split(error: &clap::Error, args: &[String]) -> Option<String> {
    let refused = |error: &clap::Error| match error.get(ContextKind::InvalidArg) {
        Some(ContextValue::String(word)) if !word.starts_with('-') => Some(word.clone()),
        _ => None,
    };
    let mut word = refused(error)?;
    // The words left, each with its place in `args`.
    let mut kept: Vec<(usize, String)> = args.iter().cloned().enumerate().collect();
    let mut first = usize::MAX;
    for _ in 0..args.len() {
        // A word of the sequent's equal to the refused one may come first;
        // the sequent's words are taken from the first place either way.
        let at = kept.iter().skip(1).position(|(_, arg)| *arg == word)? + 1;
        first = first.min(kept.remove(at).0);
        let words: Vec<String> = kept.iter().map(|(_, arg)| arg.clone()).collect();
        match Cli::command().try_get_matches_from(&words) {
            Ok(matches) => {
                let (path, matches) = innermost(&matches);
                let sequent = matches.try_get_one::<String>("sequent").ok().flatten()?;
                let ordinary = matches.try_contains_id("logic").unwrap_or(false);
                let from = first.min(kept[sequent_at(&words, sequent)?].0);
                return Some(quote(&path, &words_from(args, from).join(" "), ordinary));
            }
            Err(error) if error.kind() == ErrorKind::UnknownArgument => word = refused(&error)?,
            Err(_) => return None,
        }
    }
    None
}

/// The word that stands for the sequent while a command line is parsed
/// again to find where the sequent is.
const MARK: &str = "\u{1}sequent";

/// Returns the place in `args` of the word the command reads as its
/// sequent: the word equal to `sequent` that, replaced by a mark, makes
/// the mark the sequent.
fn sequent_at(args: &[String], sequent: &str) -> Option<usize> {
    (1..args.len()).filter(|&i| args[i] == sequent).find(|&i| {
        let mut marked = args.to_vec();
        marked[i] = MARK.to_owned();
        Cli::command()
            .try_get_matches_from(&marked)
            .is_ok_and(|matches| {
                innermost(&matches)
                    .1
                    .try_get_one::<String>("sequent")
                    .ok()
                    .flatten()
                    .is_some_and(|read| read == MARK)
            })
    })
}

/// Returns the words of a sequent the shell split, from the one at
/// `from` up to a flag; `-o` is the implication.
fn words_from(args: &[String], from: usize) -> Vec<&str> {
    let rest = args[from + 1..]
        .iter()
        .map(String::as_str)
        .take_while(|word| !word.starts_with('-') || *word == "-o");
    std::iter::once(args[from].as_str()).chain(rest).collect()
}

/// Returns the names of the commands the matches go down through, and
/// the innermost command's matches.
fn innermost(matches: &ArgMatches) -> (Vec<&str>, &ArgMatches) {
    let mut path = Vec::new();
    let mut matches = matches;
    while let Some((name, sub)) = matches.subcommand() {
        path.push(name);
        matches = sub;
    }
    (path, matches)
}

/// Returns the hint to quote a sequent: the command with the sequent as
/// one word, behind a turnstile where it has none and is linear logic, so
/// that the command runs.
fn quote(path: &[&str], sequent: &str, ordinary: bool) -> String {
    let sequent = if ordinary || sequent.contains("|-") || sequent.contains('⊢') {
        sequent.to_owned()
    } else {
        format!("|- {sequent}")
    };
    format!(
        "quote the sequent: deduxit {} {}",
        path.join(" "),
        quoted(&sequent)
    )
}

/// Returns the error a command ended with, with a hint where it is a parse
/// error that says what to write instead: a sequent given as an argument
/// whose text ends early while the command line has more words (the shell
/// split it, and `-o` became `--output`), a file or standard input of
/// `NAME: SEQUENT` lines read by `prove` as one sequent, and after the
/// library's hint on `->` the flag that reads ordinary logic, where the
/// command takes it.
pub(crate) fn parse(error: anyhow::Error, cli: &Cli, args: &[String]) -> anyhow::Error {
    let Some((path, input, ordinary)) = cli.command.sequent_input() else {
        return error;
    };
    let Some(Error::Parse(parsed)) = Explained::of(&error) else {
        return error;
    };
    // The `->` hint names the flag where the command takes it in this mode.
    let suffix = match &input.sequent {
        _ if parsed
            .hint
            .is_some_and(|hint| hint.starts_with("-> is ordinary logic's arrow")) =>
        {
            if !cli.command.takes_logic() {
                return error;
            }
            ", or add --logic classical".to_owned()
        }
        Some(sequent) if parsed.found.is_none() => {
            let Some(at) = sequent_at(args, sequent) else {
                return error;
            };
            let words = words_from(args, at);
            if words.len() == 1 {
                return error;
            }
            format!("\n  {}", quote(path, &words.join(" "), ordinary))
        }
        // An unexpected `:` is a line's name, and a word at the start of
        // a later line a sequent of its own.
        None if path == ["prove"]
            && !ordinary
            && matches!(input.format(false), InputFormat::Auto | InputFormat::Text)
            && (parsed.found.as_deref() == Some(":")
                || parsed.line > 1 && parsed.column == 1 && parsed.found.is_some()) =>
        {
            "\n  to read one sequent per line, add --input-format lines".to_owned()
        }
        _ => return error,
    };
    match error.downcast::<Explained>() {
        Ok(explained) => Explained::error(format!("{}{suffix}", explained.line), explained.error),
        Err(error) => error,
    }
}

/// Returns a word of a command line as a shell reads it back: as it is
/// where it holds nothing a shell reads specially, else quoted.
fn shell_word(word: &str) -> String {
    let plain = |c: char| c.is_ascii_alphanumeric() || "_-./=,:@+%^".contains(c);
    if !word.is_empty() && word.chars().all(plain) {
        word.to_owned()
    } else {
        quoted(word)
    }
}

/// Returns text quoted for a POSIX shell: in double quotes where nothing
/// in it is special there, else in single quotes (`!A` would be history
/// expansion in double quotes).
fn quoted(text: &str) -> String {
    if !text.contains(['"', '\\', '$', '`', '!']) {
        format!("\"{text}\"")
    } else {
        format!("'{}'", text.replace('\'', r"'\''"))
    }
}

#[cfg(test)]
mod tests {
    use super::{quoted, shell_word};

    /// A sequent is quoted so that a shell reads it back as one word:
    /// double quotes by default, single quotes around `!` and the other
    /// characters special in double quotes, a `'` within those escaped.
    #[test]
    fn quoting_keeps_a_sequent_one_word() {
        assert_eq!(quoted("A |- A"), "\"A |- A\"");
        assert_eq!(quoted("!A |- A"), "'!A |- A'");
        assert_eq!(quoted("a /\\ b"), "'a /\\ b'");
        assert_eq!(quoted("it's !A"), r"'it'\''s !A'");
        assert_eq!(shell_word("-o"), "-o");
        assert_eq!(shell_word("A,"), "A,");
        assert_eq!(shell_word("A |- A"), "\"A |- A\"");
    }
}
