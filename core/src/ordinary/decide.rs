// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Deciding an ordinary sequent in one call, by the method the options
//! choose: through the embedding (the translation, the search on the
//! image, and for a valid sequent the derivation of LK or LJ read back and
//! checked), by LJT or by the SAT solver.

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
/// `statistics` are [`ljt::Statistics`]. A sequent valid by a clausal
/// proof (`Evidence::Clausal`, the SAT solver in classical logic) has
/// `"clausal_proof": {…}` in place of `derivation`, at level 10.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Outcome {
    /// The verdict, with the evidence of a valid sequent.
    pub verdict: Verdict,
    /// The logic decided.
    pub logic: Logic,
    /// How the sequent was searched, and what the search found and cost.
    pub search: Search,
    /// The Kripke model that refutes a sequent that is not valid, checked:
    /// the SAT solver's, or in classical logic the assignment of the
    /// image's refutation as a model of one world. `None` for every other
    /// verdict, for a "not valid" of LJT, which gives no model, and for a
    /// model past [`MODEL_CELLS`](super::MODEL_CELLS), too large to show
    /// (the SAT solver's statistics count its worlds).
    pub countermodel: Option<super::Countermodel>,
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
    /// By the SAT solver, and what its search cost. Needs the cargo
    /// feature `sat`.
    #[cfg(feature = "sat")]
    Sat(super::sat::Statistics),
}

impl Outcome {
    /// Returns the method the sequent was decided by.
    pub const fn method(&self) -> Method {
        match self.search {
            Search::Embedding { .. } => Method::Embedding,
            Search::Ljt(_) => Method::Ljt,
            #[cfg(feature = "sat")]
            Search::Sat(_) => Method::Sat,
        }
    }
}

#[cfg(feature = "sat")]
impl From<super::sat::Outcome> for Outcome {
    /// The outcome of the SAT solver as an outcome of deciding an ordinary
    /// sequent, its model the countermodel.
    fn from(outcome: super::sat::Outcome) -> Self {
        Self {
            verdict: outcome.verdict,
            logic: outcome.logic,
            search: Search::Sat(outcome.statistics),
            countermodel: outcome.model,
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
            countermodel: None,
        }
    }
}

/// Whether an ordinary sequent is valid in its logic.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Verdict {
    /// The sequent is valid, and here is the checked evidence.
    Valid(Evidence),
    /// The image is unprovable, so the sequent is not valid; the linear
    /// outcome's disproof says why.
    NotValid,
    /// The search stopped before it could decide, for the reason given.
    Unknown(Reason),
}

/// What shows a valid sequent valid, checked before a search gives the
/// verdict. Evidence read from JSON is unchecked until its checker runs
/// ([`Derivation::check`], `Proof::check`).
///
/// Every method but the SAT solver in classical logic gives a derivation.
/// The SAT solver gives a clausal proof there, which has no derivation to
/// draw, export or read as a lambda-term; [`Method::Embedding`] gives the
/// derivation of LK, and so does the default wherever the embedding
/// gives it within [`EVIDENCE_WORK`].
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Evidence {
    /// A derivation of LK or LJ that [`Derivation::check`] accepted.
    Derivation(Box<Derivation>),
    /// A clausal proof that the clauses of the sequent's negation have no
    /// model, which this crate's checker accepted. Needs the cargo feature
    /// `sat`.
    #[cfg(feature = "sat")]
    Clausal(super::sat::ClausalProof),
    /// The proof of the image, which the outcome's [`Search::Embedding`]
    /// holds, where its derivation of LK or LJ was not built: a bound or
    /// the stop refused it. From [`Image::outcome`](super::Image::outcome)
    /// the linear checker accepted the proof; [`decide`] never gives it.
    Image,
}

impl Evidence {
    /// Returns the derivation, for a call that needs one: an export, a
    /// drawing, a lambda-term.
    ///
    /// # Errors
    ///
    /// [`Error::NoDerivation`] for a clausal proof, [`Error::NotBuilt`]
    /// for the image's proof alone.
    pub fn derivation(&self) -> Result<&Derivation, Error> {
        match self {
            Self::Derivation(derivation) => Ok(derivation),
            #[cfg(feature = "sat")]
            Self::Clausal(_) => Err(Error::NoDerivation),
            Self::Image => Err(Error::NotBuilt),
        }
    }
}

/// Returns whether a method's answer leaves the sequent to the embedding
/// when the method is the logic's default: the method stopped at a bound
/// other than the caller's stop (the memory, the work), or a bound refused
/// the derivation of the proof it found (its size, the memory), so that
/// it gave no verdict where the embedding may give one.
pub fn gives_way(decided: &Result<Outcome, Error>) -> bool {
    match decided {
        Ok(outcome) => {
            matches!(outcome.verdict, Verdict::Unknown(reason) if reason != Reason::Stopped)
        }
        Err(error) => matches!(error, Error::Refused(r) if !matches!(r, Refusal::Stopped { .. })),
    }
}

/// The most units of work the embedding's search may do, by default in
/// classical logic, for the evidence of a sequent the SAT solver decided:
/// a derivation of LK, or a refutation of the image. The 124 problems of
/// ILTP whose evidence the embedding gives within 2 s (123 derivations and
/// one refutation) need at most 177 136.
pub const EVIDENCE_WORK: u64 = 1 << 20;

/// Returns the outcome of a classical sequent the SAT solver decided,
/// with the embedding's evidence where the embedding gives it: its
/// search within [`EVIDENCE_WORK`] units of work (or the caller's lower
/// bound), and for a proof the derivation of LK read back and checked
/// within the caller's limits. Where the embedding ends without a
/// verdict, a bound refuses its derivation or `stop` fires, the solver's
/// outcome stands.
///
/// # Errors
///
/// [`Error::ReadBack`] where the embedding contradicts the solver, or
/// where its derivation fails the check, a defect.
#[cfg(feature = "sat")]
fn with_evidence(
    solved: Outcome,
    sequent: &Sequent,
    translation: Translation,
    search: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    let logic = solved.logic;
    let work = limits.work.map_or(EVIDENCE_WORK, |w| w.min(EVIDENCE_WORK));
    let Ok(image) = translate(sequent, logic, translation, limits) else {
        return Ok(solved);
    };
    let bounded = limits.with_work(Some(work));
    let Ok(linear) =
        search::prove_within(image.sequent(), image.mode(), search, &bounded, &mut stop)
    else {
        return Ok(solved);
    };
    let contradiction = || Error::ReadBack {
        calculus: logic.calculus(),
        reason: "the embedding contradicts the SAT solver".to_owned(),
    };
    let verdict = match (&linear.verdict, &solved.verdict) {
        (search::Verdict::Proved(proof), Verdict::Valid(_)) => {
            let derivation = match image.read_back(proof, limits, &mut stop) {
                Ok(derivation) => derivation,
                Err(error @ Error::ReadBack { .. }) => return Err(error),
                Err(_) => return Ok(solved),
            };
            match derivation.check(limits, &mut stop) {
                Ok(()) => Verdict::Valid(Evidence::Derivation(Box::new(derivation))),
                Err(error @ Error::ReadBack { .. }) => return Err(error),
                Err(_) => return Ok(solved),
            }
        }
        (search::Verdict::Unprovable(_), Verdict::NotValid) => Verdict::NotValid,
        (search::Verdict::Proved(_), _) | (search::Verdict::Unprovable(_), _) => {
            return Err(contradiction());
        }
        (search::Verdict::Unknown(_), _) => return Ok(solved),
    };
    let countermodel = image.countermodel(&linear)?;
    Ok(Outcome {
        verdict,
        logic,
        search: Search::Embedding {
            translation,
            linear: Box::new(linear),
        },
        countermodel,
    })
}

/// Decides an ordinary sequent as `options` say, within the default
/// [`Limits`] and with the linear search's default options, which bound
/// the copies of the image's search. [`decide_within`] takes bounds, a
/// search's options and a stop.
///
/// # Errors
///
/// As [`decide_within`].
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::ordinary::{self, Logic, Options, Verdict};
///
/// let sequent: ordinary::Sequent = "|- ((a -> b) -> a) -> a".parse()?;
/// let outcome = ordinary::decide(&sequent, &Options::default().with_logic(Logic::Classical))?;
/// assert!(matches!(outcome.verdict, Verdict::Valid(_)));
/// # Ok::<(), deduxit::Error>(())
/// ```
pub fn decide(sequent: &Sequent, options: &Options) -> Result<Outcome, Error> {
    decide_within(
        sequent,
        options,
        &search::Options::default(),
        &Limits::default(),
        |_| false,
    )
}

impl super::Image {
    /// Returns the outcome of deciding the ordinary sequent through this
    /// image, from `linear`, the outcome of a search of the image that the
    /// caller ran: for a proof the derivation of LK or LJ read back and
    /// checked within `limits` and until `stop` fires, or, where a bound
    /// or the stop refuses it, [`Evidence::Image`], since the image's
    /// proof stands; for a refutation not valid; else unknown. A front end
    /// that runs its own search (a race, a pool) writes this outcome.
    ///
    /// # Errors
    ///
    /// [`Error::ReadBack`] where the derivation read back fails its check,
    /// a defect.
    pub fn outcome(
        &self,
        linear: search::Outcome,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Outcome, Error> {
        let verdict = match &linear.verdict {
            search::Verdict::Proved(proof) => {
                let checked = self
                    .read_back(proof, limits, &mut stop)
                    .and_then(|d| d.check(limits, &mut stop).map(|()| d));
                match checked {
                    Ok(derivation) => Verdict::Valid(Evidence::Derivation(Box::new(derivation))),
                    Err(error) if error.is_refusal() => Verdict::Valid(Evidence::Image),
                    Err(error) => return Err(error),
                }
            }
            search::Verdict::Unprovable(_) => Verdict::NotValid,
            search::Verdict::Unknown(reason) => Verdict::Unknown(*reason),
        };
        let countermodel = self.countermodel(&linear)?;
        Ok(Outcome {
            verdict,
            logic: self.logic(),
            search: Search::Embedding {
                translation: self.translation(),
                linear: Box::new(linear),
            },
            countermodel,
        })
    }

    /// Returns the countermodel the image's refutation gives in classical
    /// logic: its assignment as a model of one world, which must refute
    /// the ordinary sequent; `None` in the other logics, for every other
    /// refutation and every other verdict.
    ///
    /// # Errors
    ///
    /// [`Error::Countermodel`] where the assignment does not refute the
    /// ordinary sequent, a defect.
    fn countermodel(&self, linear: &search::Outcome) -> Result<Option<super::Countermodel>, Error> {
        let search::Verdict::Unprovable(disproof) = &linear.verdict else {
            return Ok(None);
        };
        let crate::linear::Refutation::Classical(classical) = disproof.refutation() else {
            return Ok(None);
        };
        if self.logic() != Logic::Classical {
            return Ok(None);
        }
        let names = disproof.sequent().atom_names();
        let atoms = names
            .iter()
            .zip(&classical.assignment)
            .filter(|&(_, &value)| value)
            .map(|(name, _)| name.clone())
            .collect();
        let world = crate::kripke::World::new(atoms, false, Vec::new());
        let model = crate::kripke::Model::new(vec![world], 0);
        let countermodel = super::Countermodel::new(self.ordinary().clone(), model);
        countermodel.check(Logic::Classical, |_| false)?;
        Ok(Some(countermodel))
    }
}

/// Decides an ordinary sequent as `options` say, within `limits` and
/// until `stop` fires, the image searched under `search`: [`decide`] with
/// the bounds and options a caller chooses.
///
/// By the embedding it translates the sequent, searches the image in its
/// mode, and for a proof reads the derivation of LK or LJ back and checks
/// it; by LJT it is [`ljt::decide`] and by the SAT solver `sat::decide`,
/// which read no `search` option. The logic's default (no method and no
/// translation given) is the SAT solver: in intuitionistic and minimal
/// logic it takes LJT's derivation where LJT gives one
/// (`sat::decide_preferring_ljt`), in classical logic the embedding's
/// evidence where the embedding gives it within [`EVIDENCE_WORK`], and
/// where it [`gives_way`] the embedding decides in its place.
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
/// use deduxit::ordinary::{self, Evidence, Logic, Verdict};
///
/// let sequent: ordinary::Sequent = "|- ((a -> b) -> a) -> a".parse()?;
/// let options = ordinary::Options::default().with_logic(Logic::Classical);
/// let search = deduxit::linear::Options::default();
/// let outcome = ordinary::decide_within(&sequent, &options, &search, &Limits::default(), |_| false)?;
/// let Verdict::Valid(Evidence::Derivation(derivation)) = outcome.verdict else {
///     panic!("Peirce's law holds classically");
/// };
/// assert_eq!(derivation.logic(), Logic::Classical);
/// let options = options.with_logic(Logic::Intuitionistic);
/// let outcome = ordinary::decide_within(&sequent, &options, &search, &Limits::default(), |_| false)?;
/// assert!(matches!(outcome.verdict, Verdict::NotValid));
/// # Ok::<(), deduxit::Error>(())
/// ```
pub fn decide_within(
    sequent: &Sequent,
    options: &Options,
    search: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    let (logic, translation) = (options.logic, options.translation());
    let decided = match options.method() {
        Method::Embedding => None,
        Method::Ljt => Some(ljt::decide(sequent, logic, limits, &mut stop).map(Outcome::from)),
        // The default of classical logic keeps the embedding's evidence
        // where the embedding gives it within its bound.
        #[cfg(feature = "sat")]
        Method::Sat if options.method.is_none() && logic == Logic::Classical => {
            let decided = super::sat::decide(sequent, logic, limits, &mut stop).map(Outcome::from);
            match decided {
                Ok(
                    solved @ Outcome {
                        verdict: Verdict::Valid(_) | Verdict::NotValid,
                        ..
                    },
                ) => {
                    return with_evidence(solved, sequent, translation, search, limits, stop);
                }
                decided => Some(decided),
            }
        }
        // The default prefers LJT's derivation, the method named the one
        // read off the search.
        #[cfg(feature = "sat")]
        Method::Sat if options.method.is_none() => Some(
            super::sat::decide_preferring_ljt(sequent, logic, limits, &mut stop).map(Outcome::from),
        ),
        #[cfg(feature = "sat")]
        Method::Sat => {
            Some(super::sat::decide(sequent, logic, limits, &mut stop).map(Outcome::from))
        }
    };
    if let Some(decided) = decided
        && (options.method.is_some() || !gives_way(&decided))
    {
        return decided;
    }
    let image = translate(sequent, logic, translation, limits)?;
    let linear = search::prove_within(image.sequent(), image.mode(), search, limits, &mut stop)?;
    let verdict = match &linear.verdict {
        search::Verdict::Proved(proof) => {
            let derivation = image.read_back(proof, limits, &mut stop)?;
            derivation.check(limits, &mut stop)?;
            Verdict::Valid(Evidence::Derivation(Box::new(derivation)))
        }
        search::Verdict::Unprovable(_) => Verdict::NotValid,
        search::Verdict::Unknown(reason) => Verdict::Unknown(*reason),
    };
    let countermodel = image.countermodel(&linear)?;
    Ok(Outcome {
        verdict,
        logic,
        search: Search::Embedding {
            translation,
            linear: Box::new(linear),
        },
        countermodel,
    })
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::{Method, Options, Search, Verdict, decide_within};
    use crate::Limits;
    use crate::linear::search;
    use crate::ordinary::{Logic, Sequent, Translation};

    /// A translation given names the embedding unless a method is given,
    /// and LJT's outcome says so: written with the engine `ljt` and no
    /// translation; by default LJT gives way where a bound stops it.
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
        let outcome = decide_within(&sequent, &ljt, &search, &limits, |_| false).unwrap();
        assert!(matches!(outcome.verdict, Verdict::Valid(_)));
        assert!(matches!(outcome.search, Search::Ljt(_)));
        #[cfg(feature = "serialize")]
        {
            let json = serde_json::to_value(&outcome).unwrap();
            assert_eq!(json["search"]["method"], "ljt");
            assert!(json["search"].get("translation").is_none());
            assert!(json["search"]["statistics"]["sequents"].as_u64().unwrap() > 0);
        }
        let embedded = decide_within(&sequent, &translated, &search, &limits, |_| false).unwrap();
        assert_eq!(embedded.method(), Method::Embedding);
        let classical = ljt.with_logic(Logic::Classical);
        assert!(decide_within(&sequent, &classical, &search, &limits, |_| false).is_err());

        // By default LJT gives way to the embedding where a bound refuses
        // its derivation; named, it reports the bound.
        let small = limits.with_derivation_bytes(Some(64));
        let refused = super::ljt::decide(&sequent, Logic::Intuitionistic, &small, |_| false);
        assert!(
            super::gives_way(&refused.map(Into::into)),
            "the derivation's bound"
        );
        assert!(decide_within(&sequent, &ljt, &search, &small, |_| false).is_err());
        let stopped = super::ljt::decide(&sequent, Logic::Intuitionistic, &limits, |_| true);
        assert!(!super::gives_way(&stopped.map(Into::into)), "the stop");
        let classical = super::ljt::decide(&sequent, Logic::Classical, &limits, |_| false);
        assert!(!super::gives_way(&classical.map(Into::into)), "the logic");
    }

    /// Every kind of ordinary outcome reads back through the wire and
    /// writes the same bytes: the embedding's three verdicts, its proof
    /// without a derivation, LJT and the SAT solver; an outcome below
    /// level 12 and one valid by a clausal proof are refused.
    #[cfg(feature = "serialize")]
    #[test]
    fn outcomes_read_back() {
        use super::super::Image;
        let (search, limits) = (search::Options::default(), Limits::default());
        let round = |outcome: &super::Outcome| {
            let json = serde_json::to_string(outcome).unwrap();
            let mut document = serde_json::Deserializer::from_str(&json);
            let read: super::Outcome = crate::wire::upgrade(&mut document, &limits)
                .unwrap_or_else(|e| panic!("{json}: {e}"));
            assert_eq!(serde_json::to_string(&read).unwrap(), json);
        };
        let classical = Options::default()
            .with_logic(Logic::Classical)
            .with_method(Some(Method::Embedding));
        for text in ["|- a \\/ ~a", "a |- b"] {
            let sequent: Sequent = text.parse().unwrap();
            round(&decide_within(&sequent, &classical, &search, &limits, |_| false).unwrap());
        }
        let sequent: Sequent = "~~a -> a".parse().unwrap();
        for method in [Method::Ljt, Method::Embedding] {
            let options = Options::default()
                .with_logic(Logic::Intuitionistic)
                .with_method(Some(method));
            round(&decide_within(&sequent, &options, &search, &limits, |_| false).unwrap());
        }
        let stopped = Options::default().with_logic(Logic::Intuitionistic);
        let options = stopped.with_method(Some(Method::Embedding));
        let unknown = decide_within(&sequent, &options, &search, &limits, |_| true).unwrap();
        assert!(matches!(unknown.verdict, Verdict::Unknown(_)));
        round(&unknown);
        #[cfg(feature = "sat")]
        round(&decide_within(&sequent, &stopped, &search, &limits, |_| false).unwrap());

        // The image's proof stands where its derivation is refused.
        let peirce: Sequent = "((a -> b) -> a) -> a".parse().unwrap();
        let image: Image =
            super::super::translate(&peirce, Logic::Classical, Translation::Affine, &limits)
                .unwrap();
        let linear = search::prove(image.sequent(), image.mode(), &search).unwrap();
        let small = limits.with_derivation_bytes(Some(64));
        let outcome = image.outcome(linear, &small, |_| false).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Valid(super::Evidence::Image)
        ));
        round(&outcome);

        let refused = |json: &str| {
            let mut document = serde_json::Deserializer::from_str(json);
            crate::wire::upgrade::<super::Outcome, _>(&mut document, &limits).unwrap_err()
        };
        let old = r#"{"version":8,"verdict":"not_valid","logic":"classical","translation":"affine","engine":"focus","statistics":{}}"#;
        assert!(refused(old).to_string().contains("below level 12"));
        #[cfg(feature = "sat")]
        {
            let options = Options::default()
                .with_logic(Logic::Classical)
                .with_method(Some(Method::Sat));
            let outcome = decide_within(&peirce, &options, &search, &limits, |_| false).unwrap();
            let json = serde_json::to_string(&outcome).unwrap();
            assert!(refused(&json).to_string().contains("clausal proof"));
        }
    }

    /// By default the SAT solver decides a classical sequent and the
    /// embedding gives its evidence; where the embedding ends without it,
    /// here by the stop right after the solver's answer, the solver's
    /// clausal proof stands.
    #[cfg(feature = "sat")]
    #[test]
    fn the_classical_default_keeps_the_embedding_s_evidence() {
        use super::Evidence;
        let options = Options::default().with_logic(Logic::Classical);
        let (search, limits) = (search::Options::default(), Limits::default());
        let sequent: Sequent = "|- ((a -> b) -> a) -> a".parse().unwrap();
        let outcome = decide_within(&sequent, &options, &search, &limits, |_| false).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Valid(Evidence::Derivation(_))
        ));
        assert_eq!(outcome.method(), Method::Embedding);
        let refuted: Sequent = "a |- b".parse().unwrap();
        let outcome = decide_within(&refuted, &options, &search, &limits, |_| false).unwrap();
        assert!(matches!(outcome.verdict, Verdict::NotValid));
        assert_eq!(outcome.method(), Method::Embedding);

        let mut polls = 0_usize;
        let solved = super::super::sat::decide(&sequent, Logic::Classical, &limits, |_| {
            polls += 1;
            false
        });
        assert!(solved.is_ok());
        let mut asked = 0_usize;
        let outcome = decide_within(&sequent, &options, &search, &limits, |_| {
            asked += 1;
            asked > polls
        })
        .unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Valid(Evidence::Clausal(_))
        ));
        assert_eq!(outcome.method(), Method::Sat);
    }
}
