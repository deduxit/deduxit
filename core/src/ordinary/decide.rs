// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Deciding an ordinary sequent in one call, by the method the options
//! choose: through the embedding (the translation, the search on the
//! image, and for a valid sequent the derivation of LK or LJ read back and
//! checked) or by LJT.

use super::{Derivation, Logic, Method, Options, Sequent, Translation, ljt, translate};
use crate::limits::{Limits, Progress};
use crate::linear::search::{self, Reason};
use crate::{Error, Refusal};

/// What deciding an ordinary sequent found, with what the method's
/// search found and cost.
///
/// # JSON
///
/// With the feature `serialize` an outcome is written (never read) as
/// `{"version": 8, "deduxit": "0.3.0", "verdict": "valid", "logic":
/// "classical", "translation": "affine", "engine": "focus", "statistics":
/// {…}, "derivation": {…}}` ([`wire`](crate::wire)), `verdict` one of
/// `valid`, `not_valid` and `unknown`, the last with its `reason`. An
/// outcome of LJT has no `translation`, its `engine` is `ljt`, and its
/// `statistics` are [`ljt::Statistics`].
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Outcome {
    /// The verdict, with the derivation of a valid sequent.
    pub verdict: Verdict,
    /// The logic decided.
    pub logic: Logic,
    /// How the sequent was searched, and what the search found and cost.
    pub search: Search,
}

/// How an ordinary sequent was searched, by its [`Method`].
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Search {
    /// Through the embedding: the translation the image was made by, and
    /// the linear search's outcome on the image.
    Embedding {
        /// The translation the image was made by.
        translation: Translation,
        /// The search's outcome on the image.
        linear: Box<search::Outcome>,
    },
    /// By LJT, and what its search cost.
    Ljt(ljt::Statistics),
}

impl Outcome {
    /// Returns the method the sequent was decided by.
    pub const fn method(&self) -> Method {
        match self.search {
            Search::Embedding { .. } => Method::Embedding,
            Search::Ljt(_) => Method::Ljt,
        }
    }
}

impl From<ljt::Outcome> for Outcome {
    /// The outcome of LJT as an outcome of deciding an ordinary sequent.
    fn from(outcome: ljt::Outcome) -> Self {
        Self {
            verdict: outcome.verdict,
            logic: outcome.logic,
            search: Search::Ljt(outcome.statistics),
        }
    }
}

/// Whether an ordinary sequent is valid in its logic.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Verdict {
    /// The sequent is valid, and here is its derivation of LK or LJ, read
    /// back from the proof of its image and checked.
    Valid(Box<Derivation>),
    /// The image is unprovable, so the sequent is not valid; the linear
    /// outcome's disproof says why.
    NotValid,
    /// The search stopped before it could decide, for the reason given.
    Unknown(Reason),
}

/// Returns whether LJT's answer leaves the sequent to the embedding when
/// the method is the logic's default: LJT stopped at a bound other than
/// the caller's stop (the memory, the work), or a bound refused the
/// derivation of the proof it found (its size, the memory), so that it
/// gave no verdict where the embedding may give one.
pub fn gives_way(decided: &Result<ljt::Outcome, Error>) -> bool {
    match decided {
        Ok(outcome) => {
            matches!(outcome.verdict, Verdict::Unknown(reason) if reason != Reason::Stopped)
        }
        Err(Error::Refused(Refusal::Stopped { .. })) => false,
        Err(error) => error.is_refusal(),
    }
}

/// Decides an ordinary sequent as `options` say, within `limits` and
/// until `stop` fires. By the embedding it translates the sequent,
/// searches the image in its mode under `search`, and for a proof reads
/// the derivation of LK or LJ back and checks it, under the same limits
/// and stop; by LJT it is [`ljt::decide`], which reads no `search`
/// option. When the method is the logic's default (no method and no
/// translation given) and LJT [`gives_way`], the embedding decides the
/// sequent in its place.
///
/// # Errors
///
/// As [`translate()`], [`search::prove_within`], [`Image::read_back`] and
/// [`Derivation::check`], or as [`ljt::decide`]: a proof whose derivation
/// a bound or the stop refused is that refusal, not a verdict, and a
/// derivation that fails the check is [`Error::ReadBack`], a defect.
///
/// [`Image::read_back`]: super::Image::read_back
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::Limits;
/// use deduxit::ordinary::{self, Logic, Verdict};
///
/// let sequent: ordinary::Sequent = "|- ((a -> b) -> a) -> a".parse()?;
/// let options = ordinary::Options::default().with_logic(Logic::Classical);
/// let search = deduxit::linear::Options::default();
/// let outcome = ordinary::decide(&sequent, &options, &search, &Limits::default(), |_| false)?;
/// let Verdict::Valid(derivation) = outcome.verdict else {
///     panic!("Peirce's law holds classically");
/// };
/// assert_eq!(derivation.logic(), Logic::Classical);
/// let options = options.with_logic(Logic::Intuitionistic);
/// let outcome = ordinary::decide(&sequent, &options, &search, &Limits::default(), |_| false)?;
/// assert!(matches!(outcome.verdict, Verdict::NotValid));
/// # Ok::<(), deduxit::Error>(())
/// ```
pub fn decide(
    sequent: &Sequent,
    options: &Options,
    search: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    let (logic, translation) = (options.logic, options.translation());
    if options.method() == Method::Ljt {
        let decided = ljt::decide(sequent, logic, limits, &mut stop);
        if options.method.is_some() || !gives_way(&decided) {
            return decided.map(Outcome::from);
        }
    }
    let image = translate(sequent, logic, translation, limits)?;
    let linear = search::prove_within(image.sequent(), image.mode(), search, limits, &mut stop)?;
    let verdict = match &linear.verdict {
        search::Verdict::Proved(proof) => {
            let derivation = image.read_back(proof, limits, &mut stop)?;
            derivation.check(limits, &mut stop)?;
            Verdict::Valid(Box::new(derivation))
        }
        search::Verdict::Unprovable(_) => Verdict::NotValid,
        search::Verdict::Unknown(reason) => Verdict::Unknown(*reason),
    };
    Ok(Outcome {
        verdict,
        logic,
        search: Search::Embedding {
            translation,
            linear: Box::new(linear),
        },
    })
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::{Method, Options, Search, Verdict, decide};
    use crate::Limits;
    use crate::linear::search;
    use crate::ordinary::{Logic, Sequent, Translation};

    /// A translation given names the embedding unless a method is given,
    /// and LJT's outcome says so: written with the engine `ljt` and no
    /// translation.
    #[test]
    fn the_options_choose_the_method() {
        let options = Options::default().with_logic(Logic::Intuitionistic);
        assert_eq!(options.method(), Method::default_for(Logic::Intuitionistic));
        let translated = options.with_translation(Some(Translation::CallByValue));
        assert_eq!(translated.method(), Method::Embedding);
        let ljt = translated.with_method(Some(Method::Ljt));
        assert_eq!(ljt.method(), Method::Ljt);

        let sequent: Sequent = "~~(a \\/ ~a)".parse().unwrap();
        let (search, limits) = (search::Options::default(), Limits::default());
        let outcome = decide(&sequent, &ljt, &search, &limits, |_| false).unwrap();
        assert!(matches!(outcome.verdict, Verdict::Valid(_)));
        assert!(matches!(outcome.search, Search::Ljt(_)));
        #[cfg(feature = "serialize")]
        {
            let json = serde_json::to_value(&outcome).unwrap();
            assert_eq!(json["engine"], "ljt");
            assert!(json.get("translation").is_none());
            assert!(json["statistics"]["sequents"].as_u64().unwrap() > 0);
        }
        let embedded = decide(&sequent, &translated, &search, &limits, |_| false).unwrap();
        assert_eq!(embedded.method(), Method::Embedding);
        let classical = ljt.with_logic(Logic::Classical);
        assert!(decide(&sequent, &classical, &search, &limits, |_| false).is_err());
    }
}
