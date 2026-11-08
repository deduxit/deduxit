# The web page in teaching: what teachers and students need of it

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research
sub-agent; sources read that day. "(inference)" marks a conclusion of my own,
"(unsourced)" a claim I could not source. A neighbouring note,
`scope-education.md`, re-checks the nine old candidates of
`plan/notes/research/scope/education.md` one by one and proposes
education-10 and education-11; this note covers the use of the page (what a
teacher or a student needs), and where the two meet it says so and names the
candidate it adds to. `web-front-end.md` covers the unified stage's
architecture; the last candidate here is only what Learn and Practice need of it.

## 1. What linlog has today

From the client (`/home/tux/Projects/own/linlog-prover.github.io`: `README.md`,
`src/content/`, `src/islands/`, `src/lib/practice.ts`), `plan/web/design.md`
and `plan/reports/43-learn-lessons.md`:

- **Learn** (D6, D47, D56, D60): eleven lessons in `src/content/lessons/`
  (reading a sequent, multiplicatives, additives, units, exponentials, two
  sides, when there is no proof, proof nets, cut, quantifiers, ordinary logic
  through the translations). Each plays a worked proof one inference at a time
  with the next rule predicted before each step and a reason beside it, then
  the same proof with its last steps left to the student (backward fading), a
  self-explanation prompt with a model answer at the densest step, practice
  from the store, and a hinge question whose wrong answers each name a
  misconception. Lesson 7 is the erroneous example: a recorded attempt
  (`7-no-proof.attempt.json`) in which the student finds the step that went
  wrong, then three reasons for unprovability and an honest "unknown". Learn is
  off by default so that the expert's page is not slowed (expertise reversal,
  D6, D47).
- **Practice** (step 43a): one store of 452 problems in `src/content/problems/`
  (407 with a session, so provable; counted in the files on 2026-11-08, the
  client's README still says 425 and 392), levels from Easy to Very hard by a
  measured score (84, 138, 143, 87 by level), categories and fragment recorded,
  every verdict and session checked by a flake check that runs linlog's command.
  Filters, a random pick within them, the problem opened on a new tab, the
  problems proved marked "also on the next visit" (the key `linlog-practice`).
  The levels order effort and first-attempt success strictly, not plain success
  between Easy and Hard (43a, "What cannot be shown"); no people were tested.
- **Feedback** (D11, D12): a refusal is one plain sentence at the goal, a legal
  dead end gets no comment unless asked, verdicts are three-valued, hints are a
  four-rung ladder on request, never on a timer.
- **Sharing and persistence**: the link holds the sequent, the mode and, under
  way, the session in the fragment (D20); autosave and a restore bar (D29);
  several proofs at once (D50); import of a session or a sequent (D51).
- **Privacy** (D36, `/privacy/`): no cookies, no tracking, nothing sent to a
  server; the page lists every key the browser keeps and cites the Swiss
  revFADP and the GDPR (client `src/pages/privacy.astro`).
- **Offline and installable** (D49): a service worker and a manifest.
- **Accessibility**: keyboard for every formula (D17), WCAG 2.2 targets and no
  drag-only action (D18), 44-pixel targets on a phone, colour never alone,
  reduced motion (`prefers-reduced-motion` in `tokens.css` and `Stage.tsx`),
  live regions (`role="status"`, `aria-live` in `Stage.tsx`).
- **Exports for a hand-in or a handout**: SVG, PNG, PDF, LaTeX, Typst, text,
  the session as JSON, the Rocq certificate (client README). `linlog check`
  decides in the command whether a session proves its sequent (CLAUDE.md,
  `docs/src/status.md`).
- **Content licence**: CC BY 4.0 for every written text, problem and lesson, so
  a teacher may copy and adapt it (client README).

What is not there, read in the client's source (grep on 2026-11-08): no record
of lessons done or hinge answers across visits (the only progress key is
`linlog-practice`; `Learn.tsx` has no storage call); no teacher-facing file
format; no embed; no user-interface language but English (`<html lang="en">` in
`src/layouts/Base.astro`, no `i18n` or `locale` handling in `src/`); no print
stylesheet (`@media print` occurs nowhere in `src/`); no structural twin of the
derivation or the net for a screen reader (`graphics-document`, `speech`,
`spoken` occur nowhere); no automated accessibility audit in `package.json`
(no axe dependency); the tab's "Ordinary" choice is greyed. Of the old
candidates (`scope/education.md`, written 2026-10-09): education-3's hints are
built, its auto-reverse is not; education-5's generator was built offline in
Python (`gen/gen.py`, 43a) and not on the page; education-1 (a light on every
open goal), -2 (net exercises), -4 (course packs), -6 (interactive LK and LJ),
-7's twin, -8's embed and -9 (resource puzzles) stand; education-8's link is
built. `scope-education.md` gives the fuller account.

## 2. The field

### 2.1 What the teaching tools offer a teacher

- **Carnap** (https://carnap.io; Haskell and
  GHCJS). The primary source is Leach-Krouse's experience report [1]: "over
  45,000 students" and "over four million problems" graded by 2026. Instructors
  work "mostly through their instructor dashboard": assigning partial credit,
  offering extensions, creating assignments. An assignment is a Markdown
  document with a special syntax for the interactive exercises, one web page
  that "might be closer to a textbook chapter with interactive examples, or
  closer to an exam". Proof checking runs in the student's browser; "the server
  records the student's submission and the client's verdict, but does not
  reverify the proof" [1], which is why one small server scaled to a lecture of
  several hundred, though an in-class exam of that size needed a larger machine.
  Over fifty formal systems are supported so that homework matches the
  teacher's textbook; the free Forall x family of textbooks, once supported,
  "quickly became one of the dominant modes of usage" [1]. An instructor first
  creates an account and then asks the developers to be registered as an
  instructor [2]. LTI integration exists, made possible by a group at the
  University of British Columbia [1]. Two lessons bear on linlog: "engagement
  requires trust" (a bug or a confusing message makes students build a model of
  the software instead of the logic) and that a design in which a new proof
  system needs code and the whole toolchain was a misfeature [1]. Carnap's own
  textbook is a CC BY 4.0 online book of 12 chapters with a practice appendix
  and an index of rules [3]. Carnap was first presented to this field at ThEdu'17
  [4].
- **Iltis** (Ruhr-Universität Bochum): teachers can add tasks, combine them into
  exercises and cascade feedback mechanisms; "more than 300 students" in some
  courses using the material; no formal evaluation in the paper [5]. Its site
  says it works with learning platforms such as Moodle through LTI, offers
  German and English courses, and lets instructors specify "how much and which
  feedback" students receive [6].
- **The Incredible Proof Machine** (Breitner, ITP 2016): the logics and tasks
  are files an educator edits, tasks are grouped into sessions that may show a
  subset of a logic's rules "for an educational progression"; the state of all
  proofs, custom blocks and tasks is kept in the browser's Web Storage; once
  loaded it needs no network, which "was useful when the workshop WiFi turned
  out to be unreliable"; it follows the browser's language, English and German;
  attempted tasks are yellow, completed green [7]. In a four-day workshop of 13
  students aged 13 to 20 the class covered the propositional content in 14
  hours; a standardised questionnaire (UEQ, not the SUS the scope note says)
  placed it in the top quartile for attractiveness, dependability and novelty
  against 163 evaluations, and some students found it hard [7]. The author
  unlocked each next session with a password to keep the class together [7]. The
  repository is MIT licensed, 708 commits, deployed from `master` to
  incredible.pm [8].
- **Click & coLLecT** (Callies, Laurent; TLLA 2021): shares a proof as a URL
  (LZMA-compressed in the `p` parameter), exports Coq, LaTeX, PDF, PNG and text;
  its site and README name no teacher feature, no course and no language option
  [9][10]. LGPL-2.1, 479 commits, 27 stars (repository page, 2026-11-08; no
  commit date shown). No source shows its use in a course (unsourced).
- **Logitext, client-side reimplementation**: the URL updates as one works and
  can be handed on; LK without cut, LJ separate; its page mentions no teaching
  use [11].
- **Sequent Calculus Trainer** (Ehle, Hundeshagen, Lange; ThEdu'17): compiler-like
  feedback on rule applications, a green, red or yellow light on every sequent
  from an automated prover, a red light meaning the last rule lost the proof, a
  hint mode naming the next rule for valid sequents. The paper reports no
  numbers for the light or the hints and says the extension was not yet tested
  in a major course; an earlier version that checked syntax only was followed by
  fewer syntactic mistakes in exams, with no figures given [12].
- **Proof Tree Builder** (Korkut, ThEdu'22): students of Princeton's automated
  reasoning course used it and found it "intuitive"; that is the evaluation [13].
- **ProofBuddy** (DTU): built to collect fine-grained interaction data, with a
  preliminary usability study; the abstract says the efficacy of proof assistants
  in teaching is still unclear [14].
- **Logax** (Lodder, Heeren, Jeuring): hints and feedback that still work when the
  student strays from the generated proof; compared with expert and student
  solutions; no course use stated on its page [15].
- **Lean Game Server** (lean4game, GPL-3.0): hosts the Natural Number Game and
  others, proof checking on a server; Carnap's author cites its developers'
  estimate of "a ceiling of 70 simultaneous users" [1][16]. Its README says
  nothing on teaching or progress storage [16].
- **Jape** (GPL-2.0): a "configurable proof calculator" whose logics are written
  by the user in a documented way ("Roll your own Jape logic"); its repository
  records the death of Richard Bornat and a successor under way (page read
  2026-11-08) [17]. Its course use is (unsourced) here.
- **ProofWeb**: its server did not answer on 2026-11-08 (connection refused);
  nothing is claimed.

A pattern (inference): tools for teachers grow on two things, a way for the
teacher to pick the logic and the rules shown (Carnap's fifty systems, Iltis's
tasks, the Incredible Proof Machine's sessions) and a way to see what students
did (Carnap's dashboard, ProofBuddy's records). The first is a file; the second
is what a no-server page cannot have.

### 2.2 Standards a university will ask about

- **LTI 1.3** (1EdTech): a platform (the LMS) launches a tool by an OpenID Connect
  flow in which the launch message is a signed `id_token`; grade return and other
  services are server-to-server calls with OAuth 2 tokens [18]. A tool provider
  therefore has a server, keys and a registered deployment (inference from [18]).
  The page has none and is on GitHub Pages (D31).
- **WCAG 2.2** is a W3C Recommendation of 12 December 2024; 2.5.7 (dragging) and
  2.5.8 (target size) are new at AA; 2.1.1 Keyboard is level A, 1.4.1 Use of
  Colour level A, 1.4.11 Non-text Contrast AA [19]. The page's decisions D17,
  D18 and D33 follow these. No audit by a screen-reader user is on record
  (unsourced).
- **GDPR and the Swiss revFADP**: the client's privacy page already rests the
  logs GitHub keeps on a legitimate interest (Art. 6(1)(f)) and gives the Art. 13
  information (client `privacy.astro`). A server holding accounts and grades
  would be a controller of students' data (inference); the page's design avoids
  this.

### 2.3 What education research says, and what I read

Only what I read this session is stated as read; the rest is named and marked.

- **Feedback**: Shute's review (Review of Educational Research, 2008) describes
  desirable feedback as nonevaluative and specific and treats its timing and
  learner factors [20]. D11 cites it; the page follows it (no praise, a specific
  sentence). I read the abstract only.
- **Worked examples, fading and self-explanation prompts**: Atkinson, Renkl and
  Merrill (J. Educ. Psychol. 2003) is titled "Transitioning from studying
  examples to solving problems: effects of self-explanation prompts and fading
  worked-out steps" [21]; Chi et al. (Cognitive Science 1989) found that stronger
  students explained under what conditions a step applies and monitored their
  understanding better [22]. Lessons 1 to 11 are built on this (D21, D56). I read
  metadata of [21] and the abstract of [22], not the findings of [21].
- **Expertise reversal**: Kalyuga, Ayres, Chandler and Sweller 2003 [23] (metadata
  only read); the basis of D6 and D12.
- **Erroneous examples**: Adams et al. 2014 used them in a web tutor for
  mathematics [24] (metadata only read); lesson 7 is the page's use.
- **Retrieval and spacing**: Roediger and Karpicke 2006: after a delay, prior
  testing gave much better retention than restudying, though restudying raised
  confidence [25]; Cepeda et al. 2006 is a quantitative synthesis of distributed
  practice in verbal recall [26] (metadata only read). Both concern recall of
  text or words; their transfer to proof skill is an assumption (inference).
- **Tutoring**: VanLehn 2011 compares human tutors, intelligent tutoring systems
  and other tutoring systems [27] (title only read; the finding is not claimed).

What the page does against this: worked and faded examples (D21, D56),
self-explanation prompts at the dense step (D60), erroneous example (lesson 7),
elaborated, specific feedback and hints on request (D11, D12), interleaved random
Practice (43a). What it does not: remembering the student between visits, any
spaced or retrieval scheduling, self-explanation after a solved Practice problem,
a light on open goals (education-1), a confidence rating.

### 2.4 Evidence of use in courses, and what I could not find

Found and read: Carnap's institutions, "dozens of colleges and universities", the
page naming Arizona State, McGill, UC San Diego and Northwestern, "the list is
incomplete" [2], and 45,000 students [1]; Iltis courses over 300 students [5];
Breitner's workshop [7]; Korkut's Princeton course [13]. The venue is the ThEdu
series (Theorem Proving Components for Educational Software); ThEdu'17 was held
on 6 August 2017 in Gothenburg with CADE-26 and its proceedings, EPTCS 267, hold
the Trainer and Carnap papers [12][28]. I found no syllabus
citing the Incredible Proof Machine, Click & coLLecT or Logitext: the web search
budget of the session was exhausted before I could search for syllabi, so this
is not evidence of absence (unsourced).

## 3. Candidates

Costs are in sessions as the brief defines them. All are client work unless
stated; none adds to the trusted base, because a proof is still accepted by the
library's checker and a verdict is still the library's.

### web-teaching-1: a learner's own record, kept in the browser, exportable

- *Brings and to whom.* Students: lessons and hinge questions done, which hint
  rung a Practice problem needed, so that a returning student sees where they
  are. Teachers: the student can export the record as a file and hand it in.
  The Incredible Proof Machine keeps all state in Web Storage and marks tasks
  attempted and completed [7]; Practice already remembers proved problems. This
  is `scope-education.md`'s education-11, which I support; the missing piece in
  its account is the lesson state, absent today (Section 1).
- *In linlog's terms.* Nothing is decided or proved here; the record is a small
  JSON document with a version mark in one new key. A hand-in is only as good as
  what the checker can verify: sessions are checkable by `linlog check` or the
  page's import, the rest is self-report (inference).
- *Cost.* One session. Adds a key to the privacy page's table (the legal ground
  is the article it already cites, as for `linlog-practice`, 43a).
- *Risk.* D11 forbids praise, and a visible count invites gaming the hint ladder
  (inference); keep it a record, not a score. Local storage can be cleared
  (the export is the answer).
- *Prerequisites.* A decision that the page may keep this, as D29 and D36 allow for
  autosave and proved marks; no server.
- *Sources.* [7], client `src/lib/practice.ts`, D29, D36.

### web-teaching-2: the hand-in: an assignment link, a pack file and a check view

- *Brings and to whom.* Teachers: a list of problems (a pack: a Markdown or JSON
  file naming sequents, calculus, optionally the rules shown and whether Close is
  offered), opened from one link; students: a link back. Carnap's assignments are
  Markdown documents with exercises interleaved [1]; the Incredible Proof Machine
  restricts a session to a subset of rules for progression [7]; Iltis lets the
  instructor tune the feedback [6]. This is education-4 with education-8's link;
  it also covers "exercise authoring as files" and "grading".
- *In linlog's terms.* Decide and check are the whole product: a pack is graded
  by the library's checker on a session file or link; the teacher runs
  `linlog check` or a "check this proof" view of the page on the handed-in link
  (D20's fragment holds the session; past about 2000 characters the link falls
  back to the sequent, so a long proof is handed in as the session file, D51). A
  rule restriction needs a rule subset the popover filters by (the library change
  that education-4 names). Solutions can be exported to LaTeX or Typst.
- *Cost.* One to two sessions: the pack format with a version, the restricted
  rule set in the client (and a library filter if wanted in the command), the
  check view. Adds a format to maintain; nothing to the trusted base.
- *Risk.* Without accounts nothing stops copying and a teacher sees only what
  students send (inference from D36, D50). Carnap's own cost lesson applies to
  the format: keep a new calculus's rules out of code the teacher cannot reach
  [1].
- *Prerequisites.* None; the multi-logic stage wants the pack to name a suite.
- *Sources.* [1][6][7], D20, D51, `scope-education.md` education-4.

### web-teaching-3: a provability light behind the Learn toggle

- *Brings and to whom.* Students in Learn: after each step a bounded search tells
  whether the new goals can still close (green, red with the reason, yellow
  unknown), the Trainer's traffic light [12]. This is education-1.
- *In linlog's terms.* The verdicts are the library's checked ones, three-valued;
  a classical red can carry the counter-assignment.
- *Cost.* One session (a per-goal cache and a budget share in the worker).
- *Risk.* The Trainer paper reports no outcome figures for the light [12], so the
  benefit is a claim of design, not of evidence; D11 and D12 chose silence for
  experts, so the light must sit behind Learn (D6) and be off otherwise; on hard
  goals "unknown" is most of the picture, and Carnap's author says a time-out
  shown as "no" costs credibility (D11).
- *Prerequisites.* None.
- *Sources.* [12], D6, D11, D12, `scope-education.md` education-1.

### web-teaching-4: a screen-reader path: a structural twin, spoken formulas, a real review

- *Brings and to whom.* Blind and low-vision students, and any university
  obliged to offer an accessible tool. The page has keyboard buttons and live
  regions but its drawing is a picture with no twin (Section 1). The
  *forall x: Calgary* accessible edition codes proofs as tables with a column for
  the subproof level so that a screen reader announces a closed subproof
  (scope note E23, read 2026-10-09, not re-read today).
- *In linlog's terms.* A printer over the formula walk produces a derivation as
  a nested outline (conclusion, rule, premises) and a net as a list of links, each
  formula with a spoken form; the stage shows it as an alternative view. No new
  logic.
- *Cost.* One to two sessions, plus a review by a screen-reader user, which no
  session can supply (needs the author to find one). An automated audit (an
  axe-style check in the flake) can be a small first piece (inference: none is
  in `package.json`).
- *Risk.* Without a user's review "accessible" is a claim, not a finding
  (unsourced for any tool); interactive SVG roles are fragile across screen
  readers.
- *Prerequisites.* None.
- *Sources.* [19], D17, D18, D33, `scope-education.md` education-7.

### web-teaching-5: Learn and Practice keyed by suite, and a course across logics

- *Brings and to whom.* Teachers and students of every suite added to a unified
  stage (`web-front-end-7`, same idea: a `suite` field and a folder per suite
  under `src/content/`). A course across logics is the thing Carnap does by
  being book-like: assignments are single pages with text between exercises, an
  index of rules and a book of 12 chapters, and an instructor picks the system
  that matches the textbook [1][3]. For linlog (inference): a course page per
  suite, with the same form (worked proof, faded proof, practice, hinge
  question), and one cross-suite lesson per bridge (lesson 11's translations
  already is one).
- *In linlog's terms.* Each suite's problems decided and its sessions checked by
  its own command in a flake check, as the store is today. The lesson form and
  the level metric are linear-specific; the pattern, the Markdown and the
  checks are generic.
- *Cost.* One session for the mechanism (`education-10`'s format as data), then
  content per suite: step 43 took four sessions for eleven lessons and one for
  the store (43a to 43d), so a first suite is about five sessions of content
  (inference from that measure; `web-front-end-7` estimates five to eight
  lessons and 60 to 100 problems).
- *Risk.* A suite without lessons is a tool, not a course, and the content is
  the larger cost; levels measured for linear logic do not transfer.
- *Prerequisites.* The suite registry of `web-front-end-1`.
- *Sources.* [1][3], client `src/content.config.ts`, D56, D60,
  `scope-education.md` education-10, `web-front-end.md`.

### web-teaching-6: spaced review in Practice

- *Brings and to whom.* Students: "due" problems come back after growing
  intervals, drawn from the proved ones. Retrieval after a delay beats restudy
  for retention [25]; spacing is reviewed in [26]; both are for recall of verbal
  material, so for proof skill the benefit is a hypothesis (inference).
- *In linlog's terms.* Practice already draws a random problem within filters;
  a due-ness needs the record of web-teaching-1 (a proved timestamp). The proof
  must be redone, so the checker still decides.
- *Cost.* One session after web-teaching-1.
- *Risk.* Nags: D12's "never on a timer" applies; a review is a button, not a
  notification. An Easy problem solved a second time may teach nothing.
- *Prerequisites.* web-teaching-1.
- *Sources.* [25][26].

### web-teaching-7: embed one exercise or example in a course page

- *Brings and to whom.* Teachers who write course notes in Markdown or a book (the
  Forall x model); Click & coLLecT's URL parameters (for instance
  `exportButtons=0`) are the precedent for a stripped view [10]. This is the
  embed half of education-8.
- *In linlog's terms.* A read-only or live proof by a route of the page with a URL
  option; verdicts and sessions are the library's.
- *Cost.* One session.
- *Risk.* The page's Content-Security-Policy is a `<meta>` element since Pages
  sets no header (client README), so framing rules are limited; the client's
  design and a framed page's limits need a check (inference).
- *Prerequisites.* None.
- *Sources.* [10], client README, `scope-education.md` education-8.

### web-teaching-8: a classroom view for a projector

- *Brings and to whom.* Lecturers: D18 says large screens are the main use; a view
  with larger drawing and text, the chrome hidden, Watch advanced by one key, a
  lesson's worked proof as a slide sequence. Breitner paced a class by unlocking
  sessions with a password [7]; no source I read describes a projector mode in
  any tool (unsourced), so the need is an inference from D18.
- *Cost.* Under one session (a stage setting and a key).
- *Risk.* Small; may duplicate browser zoom and the camera.
- *Prerequisites.* None.
- *Sources.* [7], D18.

### web-teaching-9: a classroom study with people, not agents

- *Brings and to whom.* Research and the author: 43a says the difficulty levels
  were validated on agents and that "students instead of agents would test plain
  success for people". The ThEdu workshops and Breitner's classroom study with a
  questionnaire [7] show the venue and the method. A teacher runs a class, asks
  students to export web-teaching-1's record, and the author reads the files; no
  tracking is added.
- *Cost.* Little code (the export of web-teaching-1), the rest is a teacher and
  the ethics of a study, which are not sessions of the plan's kind.
- *Risk.* Needs a willing course; sample size; an opt-in only, D36.
- *Prerequisites.* web-teaching-1; a teacher.
- *Sources.* [7][12][28], `plan/reports/43-learn-lessons.md`.

### web-teaching-10: a second interface language

- *Brings and to whom.* Students whose first language is not English. The
  Incredible Proof Machine follows the browser's language, English and German
  [7]; Iltis's courses are in German and English, the English ones with slightly
  fewer exercises [6]. The page is English only (Section 1).
- *Cost.* Two or more sessions: strings of the islands and the content (eleven
  lessons, 452 problems with their explanations); the problems' sequents need no
  translation. A language is a continuing cost (inference).
- *Risk.* The largest ongoing cost of the list for a small gain unless a course
  asks for it; Iltis shows the content lags the original [6].
- *Prerequisites.* A reason: a teacher who will use it.
- *Sources.* [6][7].

## 4. Considered and rejected

- **LTI 1.3 as a tool provider.** It needs a server with keys and a registered
  deployment [18] (inference), against D31 and D36. A teacher can still place the
  page's link, a pack's link or a lesson in an LMS as a plain URL; Carnap's LTI
  came from outside contributors [1].
- **A grade book, a roster or accounts.** Carnap's dashboard and ProofBuddy's
  monitoring are what they buy [1][14]; D36, D50 and the GDPR burden of a
  controller (inference) rule them out. The record and the pack of
  web-teaching-1 and -2 are the local equivalent.
- **A language-model tutor.** Its hints could not be checked by the checker
  (the scope note's reason).
- **User-defined logics for teachers as the Incredible Proof Machine and Jape
  allow** [7][17]. The checker is a fixed calculus (D6 of the plan); the pack's
  rule subset gives the progression. Custom rules belong to `custom-rules.md`.
- **Gamification: scores, streaks, badges.** D11 forbids praise; no source read
  shows it improves learning here (unsourced).

## 5. What the structure must allow

- A pack, a record and a lesson are data that name a suite and a calculus, so the
  same file shapes serve every suite; keep their version marks as the session
  JSON's (D20).
- Generic: the lesson form (Markdown plus a session), the problem store's
  frontmatter, the flake check that runs the command over the store, the link, the
  record, the export of a session, the check. Linear-specific: the lessons'
  content, the level metric (it counts splits and copies), the net exercises.
- Each suite's command must be able to check a session file alone, because the
  hand-in is a file or a link and the teacher's check is `linlog check` or its
  page equivalent.
- A structural twin of a proof (outline) wants a derivation to be a tree of named
  rules in every suite.

## 6. Sources (all read 2026-11-08 unless noted)

1. G. Leach-Krouse, "Carnap Ten Years Later: Lessons Learned and Next Steps",
   arXiv:2607.07722 (submitted 4 July 2026, CC BY 4.0), https://arxiv.org/abs/2607.07722
   (abstract) and the PDF text.
2. Carnap, About, https://carnap.io/about
3. Carnap Book, https://carnap.io/book
4. ThEdu'17 contents (Carnap paper at p. 70), https://cgi.cse.unsw.edu.au/~eptcs/content.cgi?ThEdu17
5. Geck et al., "Iltis: Teaching Logic in the Web", https://arxiv.org/abs/2105.05763 (abstract)
6. Iltis, https://iltis.rub.de/
7. J. Breitner, "Visual Theorem Proving with the Incredible Proof Machine", ITP 2016,
   https://www.joachim-breitner.de/publications/Incredible_ITP2016_preprint.pdf (full text)
8. https://github.com/nomeata/incredible (MIT, 708 commits; no date shown)
9. Click & coLLecT, https://click-and-collect.linear-logic.org/
10. https://github.com/ComputerAidedLL/click-and-collect (LGPL-2.1; no date shown)
11. https://www.hedonisticlearning.com/logic/description.html
12. Ehle, Hundeshagen, Lange, https://arxiv.org/abs/1803.01467 and the HTML text
    https://arxiv.org/html/1803.01467v1
13. J. Korkut, https://arxiv.org/abs/2303.05865 (abstract)
14. Karsten et al., "ProofBuddy", https://arxiv.org/abs/2308.06970 (abstract)
15. Logax page, https://cs.ou.nl/members/bastiaan/AxiomaticProofs.html
16. https://github.com/leanprover-community/lean4game (GPL-3.0; no date shown)
17. https://github.com/RBornat/jape (GPL-2.0; a note dated 15 October 2026)
18. 1EdTech, LTI 1.3 core, https://www.imsglobal.org/spec/lti/v1p3/ (first 100000
    characters read; Assignment and Grade Services and Deep Linking not read)
19. W3C, WCAG 2.2, https://www.w3.org/TR/WCAG22/ (Recommendation of 12 December 2024)
20. Shute 2008, doi:10.3102/0034654307313795, via https://api.crossref.org/works/10.3102/0034654307313795
21. Atkinson, Renkl, Merrill 2003, doi:10.1037/0022-0663.95.4.774 (Crossref metadata)
22. Chi et al. 1989, doi:10.1207/s15516709cog1302_1 (Crossref)
23. Kalyuga et al. 2003, doi:10.1207/S15326985EP3801_4 (Crossref metadata)
24. Adams et al. 2014, doi:10.1016/j.chb.2014.03.053 (Crossref metadata)
25. Roediger, Karpicke 2006, doi:10.1111/j.1467-9280.2006.01693.x (Crossref)
26. Cepeda et al. 2006, doi:10.1037/0033-2909.132.3.354 (Crossref metadata)
27. VanLehn 2011, doi:10.1080/00461520.2011.611369 (Crossref metadata)
28. ThEdu'17 as in 4.

Repository files read: the client's README, `src/content/lessons/*`, `src/lib/practice.ts`,
`src/pages/privacy.astro`, `src/layouts/Base.astro`; linlog's `docs/src/status.md`,
`plan/46-research-next.md`, `plan/web/design.md`, `plan/reports/43-learn-lessons.md`,
`plan/later.md` (web client sections), `plan/notes/research/scope/education.md`, the
sibling notes `scope-education.md` and `web-front-end.md`.

## 7. Not verified

- Syllabi citing Carnap, the Incredible Proof Machine, Click & coLLecT or Logitext:
  not searched (the session's search budget ran out); only the institutions
  Carnap's own page lists were read.
- Education research: only the metadata or abstract of [20] to [27] were read; the
  effect sizes and the conditions (Renkl 2014 and Sweller 1988 were not read at all,
  the Crossref call returned 429) are not claimed. The publishers' pages answered 403.
- LTI's Assignment and Grade Services and Deep Linking, and whether an LMS can launch
  a static page without a server: not read.
- ProofWeb: unreachable. Jape's and Lean Game Server's use in courses, Click &
  coLLecT's use in courses, last-commit dates of GitHub repositories (the pages
  show none): not found.
- That the page has no screen-reader review, no print stylesheet and no lesson
  progress is from a grep of the client's source on 2026-11-08, not from running it.
- Licence of Carnap's code: not read.
- Whether Pages-hosted pages can be framed by a course site: not read.
- Nothing was run; had I been allowed, I would have run the client's page under a
  screen reader (NVDA or Orca) on a lesson and a Practice problem, to test
  web-teaching-4's premise.
