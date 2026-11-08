# What users say: the public signals of linlog two weeks after 0.2.0

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

The author's rule (2026-10-23) for this area: the repositories' issues and discussions, citations of the Zenodo DOI, crates.io's download counts, all read through the public web and GitHub's public API, nothing else, since the page tracks no one. Two api.github.com calls were used (the release list and the repository); two crates.io API calls.

## 1. What is public

What linlog already offers for hearing users, from the repository: the web page tracks no one (`plan/web/design.md`, the page's privacy decisions); the repository has issues and pull requests enabled and no Discussions tab (sources 1, 3, 4 below); the release 0.2.0 is on crates.io, GitHub releases and Zenodo (`docs/src/status.md`, README). Nothing below re-proposes a tracker: every candidate is self-report or a public channel.

## 2. The numbers

All read 2026-11-08, one week after 0.2.0 (2026-11-02) and three weeks after 0.1.0 (2026-10-18).

### GitHub

| repository | stars | forks | watchers | issues | pull requests | commits |
|---|---|---|---|---|---|---|
| `linlog-prover/linlog` | 0 | 0 | 0 | 0 open, 0 closed | 0 | 1,681 |
| `linlog-prover/rocq-linlog` | 0 | 0 | 0 | not shown (see Not verified) | not shown | 12 |
| `linlog-prover/linlog-prover.github.io` | 0 | 0 | 0 | not shown | not shown | 113 |

- The `linlog` repository was created 2026-09-28 and last pushed 2026-11-08 (API, source 2); `open_issues_count` 0, `has_discussions` false, topics linear-logic, proof-nets, proof-search, rocq, rust, sequent-calculus, theorem-prover (source 2). The issue search `is:issue` returns 0 results and `is:pr` returns 0 results (sources 3, 4). The `/discussions` URL answers 404 (source 5).
- The repository's HTML page shows an empty "Releases" section although the API lists two releases, so the HTML page was cut short for the page-reading tool; the API is the authority (inference).
- Release asset downloads (API, source 1). v0.2.0, published 2026-11-02T23:08:59Z: 25 downloads across the 15 assets in total, 18 of them of the binaries and packages, by asset: x86_64 musl tarball 3, SHA256SUMS 5, aarch64 macOS 2, x86_64 Windows zip 2, x86_64 .deb 2, x86_64 Arch package 2, the two `.crate` files 1 each, every other binary 1 each (aarch64 and x86_64 .rpm, aarch64 Arch, aarch64 musl, x86_64 macOS, aarch64 Windows, arm64 .deb). v0.1.0 (2026-10-18): the two `.crate` files, 1 download each, no binaries. The list was cut at 480 characters, so a third older release cannot be excluded, but the two shown are the only tags known.
  - The SHA256SUMS file is the one asset someone fetches to verify a download: 5 downloads beside 18 of binaries and packages suggest that some downloads are by scripts or mirrors, not people (inference, weak).
- Stars of the repositories could be zero because the author has not asked anyone to star; the page has no star button or link (inference; not checked in the client).

### crates.io (API, sources 6, 7)

| crate | total | 0.2.0 | 0.1.0 | created |
|---|---|---|---|---|
| `linlog` | 67 | 26 | 41 | 2026-10-18 |
| `linlog-cli` | 43 | 21 | 22 | 2026-10-18 |

- The crates.io counts include CI, mirrors, `cargo install` by the author's own machines and by the registry mirrors and security scanners that fetch every new crate (inference: crates.io's own documentation of the counting was not read, so the share cannot be stated). Recent downloads equal the total (the crate is younger than the 90-day window crates.io uses).
- docs.rs built 0.2.0 (average successful build 23 s, 100 percent of 1705 items documented, 32 of 600 with examples; source 8). lib.rs answered 403, so its listing could not be read.

### Zenodo (sources 9, 10)

- Record 23448588 ("linlog", v0.2.0, published 2026-11-03): 9 views (6 for the version), 2 downloads (1 for the version); the version's own DOI is 10.5281/zenodo.23899551; indexed in OpenAIRE; "Is supplement to" the GitHub tag v0.2.0. The citation section of the page is empty.
- The DOI 10.5281/zenodo.23899109 resolved to the record page of `rocq-linlog` (record 23899110, published 2026-11-02, v0.2.0): 1 view, 1 download of 125.2 kB. The prompt names 23899109 "the library's", and OpenAlex also lists "rocq-linlog" under 10.5281/zenodo.23899109 (source 11), so the DOI is the Rocq library's concept DOI and the record id differs by one (the DOI and the record id are not equal numbers; inference from the pages, not from Zenodo's documentation).
- Both records are the repository's own deposits; views and downloads at this size are the author and the indexers (inference).

### Citations and mentions

- OpenAlex lists "linlog" (DOI 10.5281/zenodo.23448588, work W7222868258, by Fabian Lukas Grubmüller) and "rocq-linlog" (10.5281/zenodo.23899109); no work in the first page of results cites them, and the records list no referenced works (source 11, first 100,000 of 152,000 characters read; the rest not read).
- Crossref's search for "linlog linear logic prover" returns eight works, none of them linlog (source 12): the Zenodo DOIs are DataCite DOIs, which Crossref does not index, so Crossref cannot show a citation of them (inference). DataCite's own citation API was not queried.
- Web search for "linlog" with "linear logic", the organisation's name, the page's address and the author's name found no blog, Reddit, Hacker News, Mastodon or Zulip mention of linlog-prover.github.io (sources 13, 14, 16). It found the author's Codeberg project of the same name (https://codeberg.org/flgrubm/linlog, source 16, the earlier home of the project; its page was not opened) and an unrelated Rust linear logic workbench, Lolli, by another author (MIT, 1 star at the snapshot, source 14). The search engine is US-only and does not index Reddit, Zulip or Mastodon threads reliably, so absence there is weak evidence (inference).

### Comparable projects, for scale

| project | stars | forks | watchers | open issues | open PRs | licence | source |
|---|---|---|---|---|---|---|---|
| Click & coLLecT (`ComputerAidedLL/click-and-collect`) | 27 | 2 | 4 | 19 | 3 | LGPL-2.1 | 17 |
| Carnap (`Carnap/Carnap`) | 349 | 34 | 18 | 103 | 5 | GPL-3.0 | 18 |
| Lolli (`ibrahimcesar/lolli`, a Rust linear logic workbench) | 1 | 0 | not shown | not shown | not shown | MIT | 14 |

- The Incredible Proof Machine and Logitext could not be read: the repository paths tried (`nomeata/incredible-proof-machine`, `logic-ex/incredible-proof-machine`, `goodlyrottenapple/logitext`) answered 404, and a `logitext/logitext` page read showed 0 stars and 26 commits, which is probably a different repository and is not used. Their numbers are not given (unsourced).
- Read against these (inference): the closest teaching tool for linear logic, Click & coLLecT, has 27 stars and 19 open issues after years; the stars of a niche logic tool are tens, not hundreds. Zero stars and zero issues three weeks after a first release, with no announcement found, is the number to expect of an unannounced project, and it says nothing of quality.

## 3. What is said

- **Issues:** none, open or closed (sources 2 to 4). **Pull requests:** none. **Discussions:** not enabled. **Comments, mentions, citations:** none found. No user-facing request or bug exists in public, so no quote can be given. The finding is that there is no public evidence of any user besides the author.
- What the numbers can say (inference): about 18 downloads of the 0.2.0 binaries and packages and about 26 and 21 downloads of the two crates in a week are the floor of curiosity, mixed with mirrors and the author's own checks, and cannot separate one real user from none. What they cannot say: whether anyone uses the web page (the page tracks no one by design), whether an AI model reads the guide `skills/linlog/` (served as `llms.txt` on the rustdoc site; no signal is public), or whether anyone teaches with it.
- What the project's setting makes slow (inference): the release was announced nowhere that this search could find, so any user would have come by search or by the topics on GitHub.

## 4. Candidates

All three use only public, self-reported signals and track no one.

### users-1: Ask for the signal where the user already is

- *Brings and to whom:* turns silence into evidence. The author, deciding what to build after 0.2.0, has no public statement of use at all; a cheap invitation gets one. Evidence of the need: sections 2 and 3, zero issues, zero discussions.
- *Offers in linlog's terms:* not a logic feature. Switch on GitHub Discussions for `linlog` (the repository has none, source 5) with a pinned "Who uses linlog and for what" thread; add to README and to the page's footer a one-line request "if you used linlog in a course or paper, say so here"; ask users who publish to cite the Zenodo DOI and give a `CITATION.cff` (whether the repository has one was not checked; `.claude/rules` and the repository were not searched for it). A "used in courses" list on the manual by self-report belongs here: a Markdown page with entries added by the author after an issue or mail.
- *Cost:* under half a session (text, one settings change by the author; enabling Discussions is a `gh` administrative task that needs the author's yes). Trusted base: none.
- *Risk:* an empty thread looks worse than none; a self-report list is biased to the keen; asking for contact invites mail the author must answer.
- *Prerequisites:* none. *Sources:* 2, 5.

### users-2: A once-a-month reading of the public counters, kept in the repository

- *Brings and to whom:* the author and the planner, to read growth against Click & coLLecT (27 stars) and to see whether a step moved anything. The numbers of this note are a baseline.
- *Offers:* a small table in `plan/notes/` (not code) with the date, stars, forks, release asset downloads and crates.io downloads for each version, read by hand or by a script that calls the public APIs once a month (a handful of calls, far under the 60 an hour of the machine's limit; GitHub's rule for unauthenticated calls was read only through the task's statement, not the documentation).
- *Cost:* a fraction of a session for a script, nothing recurring if by hand. Trusted base: none.
- *Risk:* download counts include mirrors and scanners; reading them as users overstates; the table must carry that caveat.
- *Prerequisites:* none. *Sources:* 1, 2, 6, 7, 9.

### users-3: Reach where users of this kind are, once, with something to try

- *Brings and to whom:* teaching and research users of linear logic who have no listing to find linlog by. The search for the project found only unrelated linear logic tools (sources 13, 14), so linlog is not discoverable by name or topic.
- *Offers:* listing in the places that gather such tools (the topic page `github.com/topics/linear-logic` is reached through the repository's topics, which are already set; the "other provers" lists of existing tools; a post to a logic or proof-theory mailing list or a Zulip stream, which need accounts and which the author decides). The web page's Learn lessons are the thing to point at: they need no install.
- *Cost:* about a quarter of a session to prepare text; posting is the author's act. Trusted base: none.
- *Risk:* one post that draws no one; reputation cost if the claims are over-stated (every claim in a post must be one the manual's `status.md` makes).
- *Prerequisites:* the page and the manual public (they are). *Sources:* 13, 14, 16.

## 5. Considered and rejected

- **Analytics or telemetry of any kind** (page visit counters, CLI "phone home"): the author's rule is that the page tracks no one; a counter would be the only direct measure of use and is excluded for that reason.
- **A survey form on the page:** collects data from visitors and needs a service; self-report by a thread (users-1) gives the same without storage.
- **Paying attention to GitHub stars as a goal:** the comparable numbers (27 for the nearest teaching tool) show stars are not a measure of use for this kind of tool, and chasing them invites the wrong work.
- **Querying DataCite or Scholar for citations now:** nothing in the records suggests any exists after one week; a monthly look (users-2) is enough.

## 6. Sources

1. https://api.github.com/repos/linlog-prover/linlog/releases (2026-11-08)
2. https://api.github.com/repos/linlog-prover/linlog (2026-11-08)
3. https://github.com/linlog-prover/linlog (2026-11-08); the issue search https://github.com/linlog-prover/linlog/issues?q=is%3Aissue
4. https://github.com/linlog-prover/linlog/pulls?q=is%3Apr (2026-11-08)
5. https://github.com/linlog-prover/linlog/discussions (404, 2026-11-08)
6. https://crates.io/api/v1/crates/linlog (2026-11-08)
7. https://crates.io/api/v1/crates/linlog-cli (2026-11-08)
8. https://docs.rs/crate/linlog/0.2.0 (2026-11-08)
9. https://zenodo.org/records/23448588 (2026-11-08)
10. https://zenodo.org/records/23899109 (2026-11-08; showed record 23899110)
11. https://api.openalex.org/works?search=linlog%20linear%20logic&per-page=10 (2026-11-08)
12. https://api.crossref.org/works?query=linlog+linear+logic+prover&rows=8 (2026-11-08)
13. WebSearch "linlog" linear logic prover linlog-prover.github.io (2026-11-08)
14. WebSearch linlog-prover github linear logic Rust prover reddit or hacker news or mastodon or zulip (2026-11-08); the Lolli entry https://awesome.ecosyste.ms/projects/github.com%2Fibrahimcesar%2Flolli
15. https://github.com/linlog-prover/rocq-linlog and https://github.com/linlog-prover/linlog-prover.github.io (2026-11-08)
16. WebSearch "linlog" "linear logic" Rust prover Grubmüller (2026-11-08); https://codeberg.org/flgrubm/linlog (listed in results, not opened)
17. https://github.com/ComputerAidedLL/click-and-collect (2026-11-08)
18. https://github.com/Carnap/Carnap (2026-11-08)

## 7. Not verified

- Issue and pull request counts of `rocq-linlog` and `linlog-prover.github.io`: the page-reading tool showed tabs without counts; the API was not called for them (limit). Stars, forks and watchers are from the HTML pages and read 0 for both.
- Last-commit dates of the comparable repositories (the pages gave none), the Incredible Proof Machine's and Logitext's numbers (404), lib.rs (403), Google Scholar, DataCite and Semantic Scholar citation counts (not queried), the rest of OpenAlex's 152,000-character result page, and whether a `CITATION.cff` exists in the repository.
- The v0.2.0 release asset total of 25 is my sum of the per-asset counts the tool reported; the tool's reading of a 100 KB JSON page may have missed an asset.
- How crates.io counts downloads and whether mirrors are included: not read.
- Reddit, Hacker News, Mastodon, Zulip and mailing-list mentions: a US-only search engine found none, which does not exclude them.
- Nothing needed running; no command beyond reading was used.
