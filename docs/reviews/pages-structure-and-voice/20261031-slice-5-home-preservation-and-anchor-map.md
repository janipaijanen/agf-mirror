<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->

# Slice 5 ("What is AGF?"): Home preservation report and anchor map

- **Status:** advisory review evidence. Not a complete semantic inventory and not a blocking check.
- **Plan and allocation:** `docs/plans/20261030T163253Z-pages-structure-and-voice.md`, work allocation item 5. Author allocation: governor-reviewer decision `20261031T202008Z-governor-reviewer-164q`; acknowledgement `20261031T202522Z-planner-architect-mm41`; claim `20261031T202525Z-planner-architect-qakq`.
- **Immutable base:** `5b4c3f110347a1e5fafd15647becee07dafbe652` (main after the operation slice and A2 merged).
- **Content authority:** the exact reviewed proposal in `docs/coordination/20261031T201749Z-coder-tester-mn2h.md` (on `coord/source-parity-feedback`). The author reconstructed the fenced body byte for byte. The new `agf/docs/site/index.md` has SHA-256 `f876009a5e3b2573b82a9b5bfc7ea1632cc66505a33f6caebf1b40a8699425f8`, equal to the proposal. The compatibility map hash `0bed90d9e7164effd0859be160c6c5e1db2a40dae1015ee0d68b9a5dc23fd0b1` and the review text hash `76cd2437c740b9cc7808127a6c21db475651bb248873529a223673eb6806e53b` were also checked equal. Nothing was repaired or reworded.
- **Author limit:** the author transcribed the proposal and wrote no new Home copy. The author is not independent of the four destination pages and cannot review this slice.
- **Reproduce:** `python3 -I docs/reviews/pages-structure-and-voice/slice-report-extractor.py 5b4c3f110347a1e5fafd15647becee07dafbe652 agf/docs/site/index.md`

## Extractor output

```
Base ref: 5b4c3f110347a1e5fafd15647becee07dafbe652 (5b4c3f110347a1e5fafd15647becee07dafbe652)

## index.md

- Candidate sentences in base: 5; present verbatim in final: 0; not found: 5
  - NOT FOUND: "For non-trivial changes, a human approves the plan before work starts and decides whether the tested result may merge.</p> <div class="agf-hero__actions"> <a class="agf-hero__action agf-hero__action--primary" href="docs/start/should-you-adopt/" aria-label="Decide whether AGF fits your project">Decide whether AGF fits</a> <a class="agf-hero__action agf-hero__action--secondary" href="docs/start/commission/" aria-label="Set up AGF in your repository">Set up AGF</a> </div> <div class="agf-hero__foot">Files in <code>your-repo/agf/</code> · No AGF-hosted service or agent runtime · 
  - NOT FOUND: "[See the one-page orientation flyer](reference/figures/agf-nutshell-flyer.png) if you need a compact overview first. </article> <article class="agf-route-card" markdown> <span id="adopt"></span> Commission the framework files and optional `agov` tooling, verify the assistant loader, and review the prepared diff. </article> <article class="agf-route-card" markdown> Take one small real change through plan approval, implementation, review, and merge."
  - NOT FOUND: "**Accountability structure for AI agents.** Each agent operates under a written definition: what it owns, what it may change, what it must escalate."
  - NOT FOUND: "Those definitions do not authorize an implementing agent to approve its own work, or a reviewing agent to implement the change it approves."
  - NOT FOUND: "**Decisions you can reconstruct.** Every plan approval, review finding, and merge decision is a file in the coordination channel — the append-only decision log."
- Headings before: 6; after: 6
- Heading ids before: decide, set-up, first-governed-change, operate, what-you-get, evidence-maturity-and-licensing
- Heading ids after:  decide, set-up, first-governed-change, operate, what-you-get, evidence-maturity-and-licensing
- Removed ids: none; added ids: none

## Cross-file anchor links in the final files

- index.md -> docs/start/should-you-adopt.md#cost-and-accountability: ok
- index.md -> docs/start/should-you-adopt.md#understand-the-basics: ok
- index.md -> docs/start/should-you-adopt.md#cost-and-accountability: ok
- index.md -> docs/start/should-you-adopt.md#technical-fit: ok
- index.md -> docs/start/should-you-adopt.md#assistant-workflows: ok
- index.md -> docs/concepts/roles-and-gates.md#approval-authority-and-optional-deputies: ok
```

The extractor reads the Markdown headings only. The hero ids (`agf-hero-title`, `agf-routes-title`) and the two span anchors are raw HTML, checked separately below.

## Counted ids, anchors and markup (checked in the source)

| Item | Before | After |
|---|---|---|
| Counted range ids on Home (`__top__`, `agf-hero-title`, `agf-routes-title`, `decide`, `set-up`, `first-governed-change`, `operate`) | present | present, same ids |
| Hero section markup (`aria-labelledby="agf-hero-title"`, class `agf-hero`) | present | present |
| CTA classes and hrefs (`agf-hero__action--primary` to `docs/start/should-you-adopt/`, `--secondary` to `docs/start/commission/`) | present | present, byte-identical anchors |
| Span anchors `adopt` and `using-agf-with-other-tools` | present | present |
| Four Decide anchors used (`#understand-the-basics`, `#cost-and-accountability`, `#technical-fit`, `#assistant-workflows`) and `roles-and-gates.md#approval-authority-and-optional-deputies` | not used | each resolves (extractor "ok") |
| Figure bytes (`reference/figures/governed-change-flow.svg`) | unchanged | unchanged |

## Disposition of the old Home statements (manual, for the reviewer)

"Kept" means the meaning survives on Home. "Moved" means it is now only on a linked page. "Dropped" means it is not on Home and not restated there. These are the author's reading. The reviewer decides.

| Old statement or element | Disposition on the new Home |
|---|---|
| "For non-trivial changes, a human approves the plan before work starts and decides whether the tested result may merge." | Kept in meaning: the illustrative paragraph and the hero sentence name the human governor, Gate 1 and Gate 2. The words "for non-trivial changes" are dropped. The example is a CSV export. **Reviewer to confirm that this scope qualifier is not needed on Home.** |
| Hero foot: "Files in `your-repo/agf/` · No AGF-hosted service or agent runtime · Optional local tooling" | "No hosted service or agent runtime" moved into "What you get". The `your-repo/agf/` path and the word "local" are dropped. "Tooling is optional" is kept. |
| Eyebrow "Two gates, one repo" | Replaced by "Plans and decisions in your repository". The gate names now appear after their purpose. |
| Status blockquote (build, source, citing guidance, release archives) | Reduced to one link to Release information, as the parent plan says. The "when citing AGF use the framework content version" sentence is dropped from Home and remains on Release information. The Releases link stays in the licensing paragraph. |
| "Follow Decide, Set up, First governed change, Operate" and the four-card prose | The four cards remain. Decide now lists four questions with anchors. The adopt, wait and skip wording is not on Home. Costs, terms, evidence limits and exit are linked in "What you take on". |
| Set up card: "Commission the framework files and optional `agov` tooling, verify the assistant loader, and review the prepared diff." | Moved: "Follow the existing-project setup path; review the prepared changes." The loader check and `agov` mention live on the commissioning page. |
| First governed change card | Kept in meaning: the tutorial follows setup; installation alone does not show governed work. |
| "Each agent operates under a written definition: what it owns, what it may change, what it must escalate." | Kept: written role definitions describe responsibilities, permissions and escalation. |
| "Those definitions do not authorize an implementing agent to approve its own work, or a reviewing agent to implement the change it approves." | Partly kept: "Assigning an assistant execution or advisory responsibilities does not grant approval authority" and "An assistant cannot approve its own work." **The clause "a reviewing agent may not implement the change it approves" is not on Home.** It is stated on the roles and gates page. |
| "The audit trail makes violations reviewable; whether a check is structurally blocked or relies on reviewer discipline is stated by the applicable rule." | Partly kept: the applicable rules state which checks block work and which rely on review. The "audit trail makes violations reviewable" sentence is dropped. |
| "Every plan approval, review finding, and merge decision is a file in the coordination channel — the append-only decision log." and the "six months later" sentence | Kept and corrected: coordination records are an append-only decision log; corrections add linked records; mutable plans are not append-only records. The "six months later" sentence is dropped. |
| "...works whether one AI assistant fills all roles across sessions or several assistants collaborate" | Changed on purpose: one or several assistants can perform non-approval execution or advisory responsibilities; approval needs separate valid authority. "Fills all roles" is gone. |
| "Quality gates that bind assistants too": the rule library, test-first, migration hygiene, immutable tags, merge-request workflow; "CI validators supplying evidence; the governor makes the binding decision" | Partly kept: "Checks supply evidence, not approval." The rule-library description and the named disciplines are dropped from Home. |
| "Learning that compounds": incidents, patterns, rules, "rules ship to every adopter", anonymization | Partly kept: incidents can become learning records and inform framework changes; follow anonymization and submission requirements before sharing upstream; single-adopter rules are marked Provisional. "Rules ship to every adopter" is dropped. |
| Evidence paragraph: learning records under codenames; Level 1 to Level 5 and the three evidence statuses by name | Moved: one sentence separates capability maturity from evidence status and links the maturity vocabulary and the evolution page. The level and status names are only on the vocabulary page. |
| Release information sentence (version, source commit, copyright, license) | Reduced: "Build version, source and release/development status" links to Release information. The copyright mention is dropped from Home. |
| License sentence (CC BY-SA 4.0, commercial terms, MIT tooling) | Kept, with links to the licensing terms and the Releases page. |
| Third-party tooling sentence with "Level 2 — Surface-defined", "Theoretical" evidence | Kept verbatim. |
| One-page orientation flyer link | Kept, as the last link before the motto. |
| Motto | Kept at the end, as the parent plan moves it off the first screen. |
| New on Home | The illustrative change (CSV export); the named person or rotation and the solo case; the "What you take on" paragraph; the default-off deputy route with its limits; the figure alt text and caption. |

## Reading budget (source only)

The proposal estimates 265 words in the primary counted range and 300 with the deputy and runtime detail. These are source estimates, not rendered A1 results. The original counter on the CI artifact decides, against the caps 312 (Evaluate), 312 (Adopt) and 328 (Operate) for the counted Home range. Required reading is not moved outside the counted headings. The rendered result, the two-row improvement rule and the answer keys remain open.

## Limits

Source only. No build, browser or rendered check was run for this report. The author did not review the destination pages' fit with the new Home. The figure caption and alt text come from the proposal and were not compared with the figure by the author. R1, R2, full-paint figure readability, BODY and font behavior, and the final A1 targets stay open parent obligations.
