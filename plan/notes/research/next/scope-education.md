# Education: the nine old candidates re-checked, and what is new

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research
sub-agent; sources read that day.

This note updates `plan/notes/research/scope/education.md` (written
2026-10-09, referred to below as "the scope note"; its candidates education-1
to education-9 keep their names and numbers, new ones continue at
education-10). Another note covers how teachers and students use such pages
in general (`web-teaching`); this one covers the old candidates' fate and the
linear-logic teaching material. `plan/notes/research/next/users.md` does not
exist yet, so teaching uses of the author's other tools are left to the
users agent; no public use of any of them is known to me.
"(inference)" marks my own conclusions. "(unsourced)" marks a claim I could
not source.

## 1. What linlog has today

From `docs/src/status.md`, `plan/web/design.md` (D1 to D60), `plan/web/README.md`,
`plan/reports/43-learn-lessons.md` and the client (`/home/tux/Projects/own/linlog-prover.github.io`,
read only):

- The page is a stage with tabs and a derivation or net view (D3, D4), one goal at
  a time (D5), a click on a formula lists the rules the library gives for it (D9),
  the search's inferences marked apart from the student's (D10), refusals explained
  in a plain sentence and verdicts three-valued with an honest "unknown" (D11), a
  four-rung hint ladder from the search, never on a timer (D12), undo per branch
  (D13), close one goal or all within a work budget (D14), every export (D16), keyboard
  for every formula (D17), WCAG 2.2 AA with status messages in live regions and the clickable
  formulas as buttons beside a drawing that stays a picture (D33), offline and installable (D49),
  several proofs at once (D50), import of a session or sequent (D51).
- The link: the URL fragment holds the sequent, the mode and, when under
  way, the session document (deflate-raw, base64url), past about 2000
  characters the sequent alone (D20). Autosave in the browser's storage with a
  restore bar (D29). No tracking (D36).
- Learn (D6, D56, D60): eleven lessons, each a worked proof played one
  inference at a time with a predict-the-next-rule prompt, a self-explanation prompt, a faded
  proof, practice from the store, and a hinge question whose wrong answers each name a
  misconception. Lessons 1 to 5 are the connectives, 6 two-sided, 7 when there is no
  proof (an erroneous example, three reasons for unprovability), 8 proof nets, 9 cut and its
  elimination, 10 quantifiers, 11 ordinary logic through the translations
  (`plan/reports/43-learn-lessons.md`, sessions 43b to 43d). A lesson's progress is not
  remembered across visits (43b to 43d, "What is left open").
- Practice (43a): a store of 452 problems (407 provable) in
  `src/content/problems/`, each decided by linlog's command in a flake check, with a level
  (easy to very hard, by a measured metric), categories, fragment and, if provable, a session
  `proof` accepts. 311 of the first 392 came from generators written for a target difficulty, 49
  cite a work read in itself. Filters by connectives, logic, source and level; a random pick
  within the filters; proved problems marked on the next visit.
- Not on the page: the net editor (D4; `plan/web/README.md` says `ProofStructure::link` and
  `unlink` exist in the library, the bindings do not offer them), interactive ordinary logic
  (the tab's "Ordinary" choice is greyed with the tip "comes later", `src/islands/TopBand.tsx`
  line 343; lesson 11 proves the translations' images instead), the two-sided presentation of
  classical derivations at the time of D58's writing.
- Library: the checker, `Interactive` with a JSON session at a promised level, proof
  nets with a witness for a failing criterion (`NetError::SwitchingCycle`, the disconnection,
  the box), ordinary logic read back to LK and LJ (`core/src/ordinary/`).

## 2. The field

### 2.1 The tools the scope note cites, checked again

- **Click & coLLecT** (Callies and Laurent, TLLA 2021). Repository
  https://github.com/ComputerAidedLL/click-and-collect (read 2026-11-08): LGPL-2.1, OCaml backend,
  479 commits, no release, 27 stars, 19 open issues; auto-reverse, cut mode, proof
  transformation, LaTeX and Coq (NanoYalla) export, automatic proof search and provability check,
  options to disable interaction or provability checks or export buttons through URL parameters,
  and sharing by a compressed proof in the `p` parameter (LZMA). The page shows no last-commit date,
  no proof nets, no exercises and no course use. The site
  https://click-and-collect.linear-logic.org/ (read 2026-11-08) offers a tutorial, example sequents
  and the turnstile colours: red known unprovable, orange time-out of the prover, black no claim.
  The URL parameters that switch interaction, provability and export off are a way to embed a
  fixed sequent (inference; the page does not say that is their purpose). Result: nothing in the
  scope note's description changed, and nothing newer found.
- **Sequent Calculus Trainer** (Ehle, Hundeshagen, Lange, ThEdu'17). The paper
  (https://arxiv.org/pdf/1803.01467, text read 2026-11-08) says the interactive mode "uses a
  traffic-light-like system to provide students with feedback", that a valid sequent turning invalid
  means there is no hope of completing the proof, that a counter-model comes from the SMT solver and
  that the hint to the next rule uses the proof found. The abstract page (https://arxiv.org/abs/1803.01467)
  names the syntactic and the semantic level but not the light. The same paper also reports statistics
  of the first version's use (line "In [12] we presented some statistical findings"; I did not read
  the numbers).
- **Incredible Proof Machine** (Breitner). https://github.com/nomeata/incredible (read 2026-11-08):
  MIT, 708 commits, 26 open issues, CI on every push; the page shows no date, so whether it is still
  maintained is (unsourced). Its sessions with a subset of rules are the model for the scope note's
  education-4; I did not re-read the ITP 2016 paper today.
- **Carnap**. Leach-Krouse, "Carnap Ten Years Later: Lessons Learned and Next Steps",
  https://arxiv.org/abs/2607.07722 (submitted 2026-07-04, accepted at TEAL, CC BY 4.0; read
  2026-11-08, abstract only): over 45,000 students in a decade; the paper announces a bottom-up
  redesign, a verifier kernel (mm0-zig, targeting Metamath Zero) and a proof compiler (abc) for
  "richly interactive proof-authoring on the web". The abstract does not list the lessons; the
  scope note's quote about time-outs shown as "no" came from the paper's full text on 2026-10-09 and
  I did not re-read it.
- **Iltis**. https://iltis.rub.de/ (read 2026-11-08): task types for propositional, modal,
  first-order logic and CTL, formal languages, reductions, and general tasks (drag-and-drop proof
  blocks, buckets); atomic tasks combined into multi-step exercises, feedback tunable per instructor,
  LTI integration for Moodle; "well on its way to becoming open source"; the page does not mention
  sequent calculus or linear logic.

### 2.2 New since the scope note

- **LogicProof** (arXiv 2607.04321, https://arxiv.org/abs/2607.04321, read 2026-11-08; listed for
  TEAL 2026 by a search result): a browser tool for natural deduction and sequent calculus,
  propositional and first-order, classical and constructive, real-time feedback per rule, proof trees
  shown interactively; a study with 35 students reported faster iteration and more confidence. The
  abstract names no hints, export, sharing, licence or linear logic.
- **MiniCalc** (Villadsen, DTU; EPTCS paper arXiv 2505.05988, https://arxiv.org/abs/2505.05988, read
  2026-11-08): a web app for first-order logic over a minimal sequent calculus, proofs optionally
  verified in Isabelle; a search result gives "more than 100 students 2021 to 2024" and a 2026
  patch at https://proof.compute.dtu.dk/ (the figures come from the search summary, which I did not
  open). Its checker option is the same shape as linlog's `--kernel` certificates (inference).
- **Linear-logic course material with public pages** (all read through a search summary or a
  fetched page on 2026-11-08):
  - The LL Handbook, public draft of 6 March 2026, https://ll-handbook.pages.math.cnrs.fr/ and
    https://ll-handbook.pages.math.cnrs.fr/book/ll-handbook-draft.pdf: a project of the Linear
    Logic IRN for master's and doctoral students and researchers; "very early stage", licence not
    stated. The client's lessons 1 to 5 already cite its tables for the classics (report 43a).
  - Pfenning, 15-836 Substructural Logics, Fall 2023,
    https://www.cs.cmu.edu/~fp/courses/15836-f23/index.html: linear, ordered, affine, bunched and
    separation logics, cut elimination, focusing, session types; weekly homework (60%), midterm,
    miniproject; no textbook, a combined set of lecture notes of 248 pages posted publicly; tools
    named are Piazza, Gradescope and Canvas, with no proof tool named. 15-816 in Spring 2012
    (https://www.cs.cmu.edu/~fp/courses/15816-s12/) is the older linear-logic course the scope note
    cited; the page links nothing after 2012. I found no 2025 or 2026 offering by Pfenning
    (search summary).
  - Ankush Das, CS 599 Logic and Programming Languages, UIC, Spring 2026,
    https://ankushdas.github.io/courses/CS599-S26.html: follows Pfenning's constructive logic
    course; linear inference and linear logic lectures on 14 and 16 April 2026; code is OCaml and
    Prolog files (`inv_calc.ml`, `seq_calc.pl`, `inv_calc.pl`), homework as LaTeX PDFs through
    Gradescope; no web proof tool named.
  - Kesner, "Lambda calculi through the lens of linear logic", Oregon Programming Languages Summer
    School 2025,
    https://www.cs.uoregon.edu/research/summerschool/summer25/_lectures/Kesner-notes-1.pdf: a day of
    proof nets and a day of a calculus inspired by them (search summary; I did not open the notes).
  - Older, still in use: Strassburger, "Proof nets for beginners" (ESSLLI 2006, arXiv cs/0610123),
    Mazza, "Introduction to linear logic" (Torino, HAL cel-01144229), IIT Kanpur CS 662A
    (https://cse.iitk.ac.in/pages/CS662A.html), Di Cosmo's Linear Logic Primer
    (https://dicosmo.org/CourseNotes/LinLog/), Moot's proof-net lecture notes
    (https://www.labri.fr/perso/moot/moot02proofnets.pdf). All from a search summary only.
  - A search for 2025/26 ESSLLI courses on linear logic or proof nets found none, and a search for
    a proof-net web tool for exercises found none; two searches, not a survey.
  - Laurent's own lecture notes: a search was not possible (the search budget of the session was
    used up); (unsourced).
- **Accessibility**: the search found no screen-reader-usable sequent or net tool. The forall x
  accessible proof tables are confirmed (https://forallx.openlogicproject.org/html/A4.html, read
  2026-11-08: four columns, line, subproof level, formula, justification; screen readers announce when a
  subproof opens and closes; simple text-to-speech may not read the tables at all).

What the course pages show (inference): the courses that teach linear logic to computer scientists
(Pfenning's line, Das) do homework as PDFs and small programs, not in a web trainer. The one trainer
for linear logic stays Click & coLLecT. A student of such a course has no checker for a net or a
sequent proof unless they use that tool or linlog. This is the gap the page already fills for
sequents and does not fill for net exercises.

## 3. Candidates

For each old candidate: its fate on the page, what the page's design makes cheap or
impossible, cost, then the evidence re-checked. Marks: built, partly, untouched.

### education-1: a provability light on every open goal. Status: partly

- *Built:* the verdicts on request (D11, D14: "close" per goal or all within a budget, three-valued,
  with the reason and a checked refutation; Learn lesson 7 shows "Unprovable, checked: atom
  balance", a classical countermodel, an exhausted search, and "Unknown: gave up" turned into a
  proof by the retry's doublings). *Untouched:* a light that is on every open goal after every step.
- *The design makes it a choice, not a gap:* D11 says a legal step that leads nowhere gets no
  comment unless the student asks for a hint, from the expertise-reversal argument, and D12 never
  offers help on a timer. The Trainer's light is on by default for beginners [1]. Under D6 a light
  fits behind the Learn toggle (inference): it is the help that costs an expert attention and
  helps a novice.
- *What the work-budget design makes cheap:* the budget is work, not time (D14), so a light per
  goal is deterministic; the bindings run in a worker (D28), so the main thread is free (D35).
- *Cost:* small, about one session in the client with a per-goal cache and a budget share; no new
  trusted base, because the verdicts are the library's checked ones. The `Stop::poll` and per-goal
  status points of the scope note still stand.
- *Risk:* an "unknown" light is most of the picture on hard problems; Carnap's lesson (D11) is that
  it must be shown as unknown. A red light on a step teaches the student that the step lost the proof
  [1], which the lesson 7 erroneous example does once by hand.
- *Evidence:* [1] confirmed today in the paper's text. Click & coLLecT's turnstile colours [2]
  confirmed on its site.

### education-2: proof-net exercises, a linking drawn and checked. Status: untouched (lesson 8 shows nets, nothing is exercised)

- *Built:* the net view of a finished proof (D4), lesson 8's worked proof turned into its net, "More
  nets" with an essential net, a box in a box and an intuitionistic net, and the criterion's witness
  in the library. *Not built:* the net editor. D4 names it a later step "which has no precedent";
  `plan/web/README.md` says the bindings do not offer `link` and `unlink`.
- *What the design makes cheap:* the library side exists (`ProofStructure::link`, the witness with
  ids), the SVG carries ids, and the stage already draws a net; D33's rule (formulas as buttons
  beside a drawing) gives the click targets. *What makes it hard:* nothing in D1 to D60; the
  net view is greyed until a proof exists, and its layout is for finished nets, so a half-linked
  structure with a switching cycle drawn needs a new drawing state (inference).
- *Cost:* moderate, two sessions: bindings for `link`, `unlink`, the criterion's witness as ids; the
  editor island (click two dual literals, colour the cycle, offer sequentialization); MELL boxes
  after. No new trusted base: a net's sequentialization is a checked proof.
- *Risk:* net exercises need level metrics the problem store does not have (43a's metric is for
  derivations); a random linking is a poor exercise, a good one is a net with one bad link.
- *Evidence of a gap:* the scope note's "no web tool does it" still holds after the searches of
  2026-11-08 (two searches, no web tool, no 2025 exercise set found). Matsuoka's calculator
  (https://staff.aist.go.jp/s-matsuoka/PNCalculator/index.html) was not re-read; (unsourced for today).
  The LL Handbook's chapter on proof structures [3] and Strassburger's and Mazza's notes are the
  texts such an exercise would accompany.
- *Linear-specific:* yes.

### education-3: hints and auto-reverse from the engines. Status: hints built, auto-reverse untouched

- *Built:* D12's four rungs from the search on a copy of the active goal (goal and kind of
  connective, invertible rules first; the formula; the rule and the split; bottom-out where the page
  applies the step and asks why). The library's hints are the steps `interact` applies for the
  store's sessions (43a).
- *Untouched:* an option that applies the invertible rules automatically (Click & coLLecT's
  auto-reverse [2]). The invertible-first order appears only in hint rung one (`src/lib/steps.ts`).
- *The design:* D9 forbids applying any rule silently on a click and the page never acts on its own,
  so auto-reverse must be an explicit toggle or a button "apply the invertible steps" (inference).
  With D10 the page can mark them as the machine's.
- *Cost:* small, under one session; the library knows invertibility per rule (the hint's first rung
  uses it).
- *Evidence:* Click & coLLecT's auto-reverse is confirmed on its site [2]: "applies reversible rules
  to generated sequents".

### education-4: course packs. Status: untouched; the pieces exist

- *Built:* a store of problems as Markdown with a frontmatter and a session, with a flake check that
  runs linlog's command over all of them (43a); Practice's filters; import of a session or a
  sequent (D51); a proof in a link (D20). *Not built:* a teacher's file format (a list with the
  rules allowed, whether search may close goals, the expected verdict), a rule restriction in
  `Interactive`, a grader, or a solution sheet.
- *What the design makes cheap:* a teacher can send a link (D20); the student opens it and the page
  runs the library's checker, so a handed-in session is checkable by anyone with no server. The
  store's format is already one file per problem under a CC BY 4.0 licence (README of the client).
  *What it makes impossible:* D36 (no tracking) and "nothing syncs anywhere" (D50) rule out a grade
  book, a roster and per-student progress on a server; a teacher sees a result only if the student
  sends a link or a file. Autosave (D29) and the proved-marks are in the student's own browser.
  Iltis's route to a learning platform is LTI [4]; it needs a tool provider on a server, which D36
  and the hosting (D31, GitHub Pages) do not have (inference).
- *Cost:* moderate, one to two sessions: a versioned pack format (a pack is a Markdown or JSON file
  the page imports), a rule subset that the popover filters by, and a read-only "check this link"
  view. The checker is already the trusted base, so nothing is added to it.
- *Risk:* no accounts means no way to stop copying; the pack's answers are checkable by anyone.
- *Evidence:* Iltis lets instructors tune feedback per task [4] (read today, but the "much feedback
  to almost none" detail is from the scope note's reading of the paper); Carnap's decade of use [5].
- *Carries over to every suite:* yes, if the pack names a calculus.

### education-5: generated exercises with certified properties. Status: partly, offline

- *Built:* `gen/gen.py` and `gen/pick.py` in the client (43a): a plan for a target level, a proof grown
  from its conclusion, a disguise; 10,000 candidates decided and measured by linlog, each kept only
  if its measured level is the aimed one; every kept problem has a session the flake check runs
  through `proof`. That is the scope note's "decided by the engines, kept only with a checked proof"
  and Ahmed et al.'s search from proofs to problems [6], done once, in Python, not on the page.
  The unprovable problems of Learn are a hand grade (43a, "What is left open").
- *Not built:* a seeded generator in the library or the page, a fresh sequent per student or exam,
  a certified minimal height, and a required rule as a property. The level metric (43a) is measured
  by running linlog, so a page-side generator would need the same run in the worker (inference).
- *The design:* no server and the worker's work budget (D14, D28) make a page generator possible
  (a bounded batch in the worker); the store's size (74 KB for 392 problems, report 43a) shows why
  the store is fetched, not bundled. Per-student variants would be a seed in a link (D20).
- *Cost:* moderate, two sessions to bring the generator into the library's `families.rs` style and
  expose it through the bindings; no addition to the trusted base, since every kept problem is
  decided by the engines and checked.
- *Risk:* a "level" measured on agents, not on students (43a says so: "students would test the levels
  for people"); generated problems read as arbitrary (43a keeps no titles for them).
- *Carries over to every suite:* the pattern does (generate, decide, keep the certified ones); the
  generators and the level metric are linear-specific.

### education-6: interactive LK and LJ through the embeddings. Status: untouched (the library reads back LK and LJ; the page has none)

- *Built:* lesson 11 proves the images of ordinary sequents under linlog's translations; the library
  decides and reads back `core/src/ordinary/`. *Not built:* the ordinary tab (greyed, D57) and an
  interactive state over an ordinary derivation; `plan/web/README.md` lists the missing JSON forms of
  `ordinary::Outcome`, the sequent and its derivation.
- *The design:* D57 already places the choice on the tab and D2 keeps linear logic the default;
  nothing in the page rules it out. A page that proves LK and LJ steps directly is Logitext's
  territory and, now, LogicProof's and MiniCalc's [7][8]: three web tools for classical and
  constructive sequent calculus exist, none with linear logic (inference from §2.2). The distinctive
  offer is the one the scope note named: the same goal in LJ and in ILL side by side.
- *Cost:* real, two to three sessions; the other note on natural deduction and the collection of
  suites covers the generic kernel it would sit on.
- *Teaching value (inference):* high where a course teaches LK or LJ first (Das's course does, in the
  same semester as linear logic [9]), lower for a pure linear-logic course.
- *Carries over:* yes, it is the first suite beyond linear.

### education-7: accessible derivations and nets. Status: partly

- *Built:* keyboard for every formula (D17), WCAG 2.2 AA with the drawing as a picture and buttons
  beside it (D33), live regions for every verdict and refusal (D33; `aria-live` or `role="status"`
  in five files of the client's `src/`), reduced motion (D34), colour told apart under simulated
  deficiencies, 44-pixel targets on a phone. *Untouched:* a derivation as a nested outline or a net as
  a list of links as the twin of the drawing, spoken forms of formulas, and the `graphics-document`
  role (no occurrence of `graphics-document`, `speech` or `spoken` in the client's `src/`).
- *The design:* D33's rule that an interactive SVG must not sit inside `role="img"` makes the
  structural twin the missing piece. The forall x model [10] shows the form: a table with the subproof
  level in a column and announced openings and closings. The search of 2026-11-08 found no
  screen-reader-usable sequent or net tool.
- *Cost:* small in the library (a printer over `sequents::fmt::Walk`), one session in the client for
  the outline and the tab order; no trusted base.
- *Risk:* no blind student's test is available to me or to the project; the claim "usable" needs a
  screen-reader user's review (unsourced for any tool).
- *Carries over:* yes, wholly: the outline printer is generic once a derivation is a tree of rules.

### education-8: shareable and embeddable proofs. Status: share built, embed untouched

- *Built:* D20's link with the session in the fragment, past 2000 characters the sequent alone and an
  export of the session file, D50 and D51 for several proofs and import, D29 autosave. This is Click
  & coLLecT's compressed proof in a URL [2][11] with the version mark and a refusal path.
- *Untouched:* an embed (an iframe or a web component with a read-only or live proof). No `embed`
  occurrence in the client's `src/`. The page is a single-page app with a CSP set by a `<meta>` element
  (D42, README), and an iframe needs the framing rules of GitHub Pages' host, which sets no header
  (inference; the client's README says Pages sets no header); this is the design's one real obstacle.
  Click & coLLecT's URL switches (interaction, provability, export off) are the precedent [11].
- *Cost:* small to moderate, one session: a stripped route of the page with a URL option; the checks
  of D42 for a framed page.
- *Risk:* a link is untrusted input (the scope note's `from_json_within` bound; D20's fallback covers a
  refused document).
- *Carries over:* yes.

### education-9: resource puzzles over the Horn engine. Status: untouched

- *Built:* the Horn engine (status.md: reachability, backward search, coverability, the state
  equation's refutation); no page view of a net's tokens. The scope note's open question (D12 of the page asks for a layout the object suggests, and a Petri net has none fixed)
  is still open.
- *The design:* nothing excludes it; Learn's progression (D60) has no lesson on the resource reading,
  and lesson 5 (the exponentials) is where clauses under `!` appear.
- *Cost:* moderate, two sessions (a step API, a Petri-net drawing, level files).
- *Evidence of use:* Ceptre ties play to proof search [12] (not re-read today); no teaching use
  found (inference: the idea is a design, not a documented practice).
- *Linear-specific:* yes, the most so of the nine.
- *Teaching value (inference):* a motivation for a first lesson, low for a course that already starts from
  sequents.

### education-10: a lesson-and-problem format as a library for other suites. Status: new

- *What it brings:* the page's teaching form (a worked proof played with a reason and a prompt, a faded
  proof, practice, a hinge question) lives in the client's Markdown and its islands, tied to linear
  rules. If the collection of suites grows (the author's direction in
  `plan/46-research-next.md`), a second suite repeats the content work, which was the largest cost
  of step 43 (four sessions for the lessons, one for the store). A format that names the calculus,
  the rules of a lesson and the reasons per inference as data would make a lesson for another suite
  a content task (inference).
- *Cost:* small to moderate; most of it is refactoring what step 43 wrote.
- *Prerequisite:* the generic kernel of the collection note; the format follows education-4's pack.
- *Risk:* too early before a second suite exists (inference).
- *Carries over:* it is the carrying-over itself.

### education-11: a learner's own record, kept locally. Status: new

- *What it brings:* progress across lessons ("a lesson's progress is not remembered", 43b to 43d) and
  Practice's proved marks exist already; a visible record of lessons done, hinge answers, and which
  rung of the hint ladder a problem needed, kept in the browser only (D29, D36), exportable as a file
  the student gives a teacher. This is what a grade book is without a server (inference).
- *Cost:* small, one session; it adds nothing to the trusted base, though a self-reported file is
  not proof of anything the teacher can verify except sessions the checker accepts.
- *Risk:* the temptation to count; D11 forbids praise, and a record of hint use may breed the
  gaming that the hint ladder wants to avoid (inference).
- *Carries over:* yes.

## 4. Considered and rejected

- **A server with accounts, a grade book and LTI** (Carnap, Iltis [4][5]): still excluded by D36, D50
  and the hosting; the pack and the link give a local equivalent (education-4, education-11).
- **User-defined logics for courses** (the Incredible Proof Machine's files): stays with the custom-rules
  note; a rule subset of a fixed calculus is enough for a progression.
- **A language-model tutor**: its hints could not be checked (the scope note's reasoning, unchanged).
  Hints from the search are checked by the checker.
- **Graphical natural deduction** and **a Lean or Rocq game**: as in the scope note, no representation or
  checker of linlog reads them; the natural deduction note covers the former.
- **Finite phase countermodels shown to students**: unchanged; a classical falsifying assignment
  already stands in the verdict (lesson 7).
- **MALL nets as an exercise**: unchanged (`plan/later.md`: "a display feature without a use").

## 5. Re-rank for teaching (inference)

The audience is a teacher or a student of linear logic, then of other logics (the author's direction).
The ranking weighs benefit to a learner, whether the evidence supports it, cost, and whether it carries to
other suites.

1. **education-4 (course packs, with the local check of a link)** and **education-8's embed**: the lowest
   cost for the largest change to who can use the page in a course: a teacher can set a list, restrict
   rules, and read a handed-in link, with no server. The evidence is that courses of linear logic use
   PDFs and small programs (§2.2), so a checkable handed-in link is the missing piece. Carries over.
2. **education-2 (net exercises)**: the one place where linlog could offer what no tool found offers, in
   an area the courses and the Handbook treat in depth. Linear-specific. Moderate cost.
3. **education-1 (the light) and education-3's auto-reverse**, behind the Learn toggle: small, and the
   evidence (the Trainer [1]) is on feedback at the semantic level. Carries over.
4. **education-7 (accessibility twin)**: small, no competitor found; the case is equity and WCAG, not
   course demand. Carries over. Rank it first if the author takes accessibility as a promise.
5. **education-5 (generated exercises on the page)**: the store exists; per-student variants matter for
   an exam. Carries over in pattern only.
6. **education-6 (interactive LK and LJ)**: high value for a course that teaches both, but three tools
   already serve LK and LJ (§2.2) and the cost is real. It belongs in the collection's order, not here.
7. **education-10 and education-11**: wait for a second suite and for evidence from use.
8. **education-9 (resource puzzles)**: lowest. Linear-specific, no teaching use found.

Linear-specific: education-2 and education-9, and the generators and metric of education-5. Carry over
to every suite: education-1, 3 (hints), 4, 5 (pattern), 7, 8, 10 and 11.

## 6. What the structure must allow

- Reusable (judged from the client's design, not from `.claude/rules/core*.md`, which I did not
  re-read for this note): the session document and its versioned JSON; the checker; the explained
  refusal; the budgeted search with three-valued verdicts; the store's format and its flake check; the
  link; the lesson format of D56 once it is data.
- Linear-specific: the rule schemas of the reference, the level metric (counts and bits of choice for
  splits and copies), the net editor, the Horn puzzles.
- A second suite needs: a per-calculus rule set that the popover and the pack can filter, a "session"
  document with its own level, and a verdict type that can be unknown.

## 7. Sources

1. Ehle, Hundeshagen, Lange, Sequent Calculus Trainer, https://arxiv.org/abs/1803.01467 and
   https://arxiv.org/pdf/1803.01467 (2026-11-08)
2. Click & coLLecT site, https://click-and-collect.linear-logic.org/ (2026-11-08)
3. LL Handbook, https://ll-handbook.pages.math.cnrs.fr/ and
   https://ll-handbook.pages.math.cnrs.fr/book/ll-handbook-draft.pdf (2026-11-08; the PDF through a search
   summary)
4. Iltis, https://iltis.rub.de/ (2026-11-08)
5. Leach-Krouse, Carnap Ten Years Later, https://arxiv.org/abs/2607.07722 (2026-11-08)
6. Ahmed, Gulwani, Karkare, IJCAI 2013, https://www.ijcai.org/Proceedings/13/Papers/291.pdf (not re-read)
7. LogicProof, https://arxiv.org/abs/2607.04321 (2026-11-08)
8. MiniCalc, https://arxiv.org/abs/2505.05988, https://proof.compute.dtu.dk/ (2026-11-08; the latter through a
   search summary)
9. Das, CS 599, https://ankushdas.github.io/courses/CS599-S26.html (2026-11-08)
10. forall x: Calgary, https://forallx.openlogicproject.org/html/A4.html (2026-11-08)
11. Click & coLLecT repository, https://github.com/ComputerAidedLL/click-and-collect (2026-11-08)
12. Martens, Ceptre, https://ojs.aaai.org/index.php/AIIDE/article/view/12784 (not re-read)
13. Incredible Proof Machine, https://github.com/nomeata/incredible (2026-11-08)
14. Pfenning, 15-836 (Fall 2023), https://www.cs.cmu.edu/~fp/courses/15836-f23/index.html; 15-816 (Spring 2012),
    https://www.cs.cmu.edu/~fp/courses/15816-s12/ (2026-11-08)
15. Search summaries of 2026-11-08 (not opened pages): Kesner's notes
    (https://www.cs.uoregon.edu/research/summerschool/summer25/_lectures/Kesner-notes-1.pdf), Strassburger
    (https://arxiv.org/abs/cs/0610123), Mazza (https://hal.archives-ouvertes.fr/cel-01144229), IIT Kanpur
    (https://cse.iitk.ac.in/pages/CS662A.html), Di Cosmo (https://dicosmo.org/CourseNotes/LinLog/), Moot
    (https://www.labri.fr/perso/moot/moot02proofnets.pdf)
16. Repository files read: `docs/src/status.md`, `plan/web/design.md`, `plan/web/README.md`,
    `plan/reports/43-learn-lessons.md`, `plan/notes/research/scope/education.md`, the client's `README.md`,
    `src/content/`, `src/islands/TopBand.tsx`

## 8. Not verified

- Click & coLLecT's, the Incredible Proof Machine's and MiniCalc's last activity (no date on the pages read).
- The statistics of the Trainer's first version, Carnap's full lessons, Iltis's feedback levels, the
  ITP 2016 paper, Ceptre and Matsuoka's calculator: carried from the scope note's reading of 2026-10-09,
  not re-read today.
- Everything in the course list marked as a search summary; Laurent's lecture notes (no search possible, the
  session's budget was used up).
- That no proof-net exercise tool exists: two searches, not a survey.
- The client's absence of an outline twin, auto-reverse toggle, embed and spoken forms: a grep of `src/`
  for `graphics-document`, `embed`, `speech`, `spoken` and for the invertible-first hint rung, not a
  use of the page.
- The brief spoke of an "ordinary tab" as built; in the client I read, the choice is greyed ("comes later").
- `.claude/rules/core*.md` were not re-read for section 6.
