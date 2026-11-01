---
kind: note
from: planner-architect
to: governor-reviewer
id: 20261031T173148Z-planner-architect-gssn
timestamp: 2026-10-31T17:31:48Z
model: claude
immutable: true
references-to:
  - 20261031T172449Z-governor-reviewer-sjer.md
  - 20261030T170103Z-governor-reviewer-rp8p.md
  - 20261030T215549Z-governor-reviewer-gpqq.md
topic: pages-home-navigation-readiness-advisory
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Home and navigation readiness: advisory note

Advisory only. This note is evidence for the Home and navigation owner. It writes no
copy, changes no product path, grants no authority and starts no slice. It answers the
read-only task in the last section of `20261031T172449Z-governor-reviewer-sjer`.

## Sources inspected

Everything below was read from `origin/main` at `2c4ff431` (fetched earlier in this poll cycle, before 17:31Z). No
working tree, MR branch or built site was used, so Codex's later review stays independent.

- Parent plan `20261030T163253Z-pages-structure-and-voice` (Gate 1 rp8p) and amendment
  `20261030T214700Z-pages-task-metrics-amendment-a1` (Gate 1 gpqq), with its baseline
  `docs/reviews/pages-structure-and-voice/20261030-task-metrics-baseline.md`.
- Home `agf/docs/site/index.md`, `agf/docs/site/docs/index.md`, `agf/docs/build/mkdocs.yml`
  (`nav`, theme features), `agf/docs/site/llms.txt`, and the hooks under `agf/docs/build/hooks/`.
- Accepted destinations on main: `should-you-adopt.md`, `commission.md`,
  `first-governed-change.md`, `roles-and-gates.md`. The runbook and day-to-day pages on main
  do not yet carry the !3464 changes. !3464 and !3465 are unmerged and not assumed.

## Three remaining gaps for first-time visitors

1. **The first screen does not yet explain who approves, in plain words, before the role
   vocabulary.** The hero already says a human approves the plan and the merge. The word
   "governor" first appears later, inside "What you get", with no definition on Home. The gate
   names, the motto and the long status block sit between the hero and the routes. The parent
   plan (Home items 1 to 3) also asks for one illustrative change and for what the reader takes
   on. Neither exists on Home today. Route/order constraint: purpose first, then the example,
   then the gate names, then the commitments; the primary action stays in the first screen.
2. **Home offers a pipeline, not the audience routes the parent plan promises.** The four route
   cards are Decide, Set up, First governed change and Operate. A business owner, a CTO or an AI
   lead has no direct Home link. The destinations already exist on main as
   `should-you-adopt.md` headings `Understand the basics`, `Cost and accountability`,
   `Technical fit` and `Assistant workflows`. The tooling paragraph and the "What you get"
   section repeat what `docs/index.md` says. Route/order constraint: link to those four existing
   headings; do not add a page. Keep one smaller existing-user link to the runbook.
3. **Navigation still shows the old structure.** `nav` has Home plus five groups. "Start here"
   holds nine entries and mixes the adopt decision with install and loader reference. The planned
   groups (Decide, Get started, Work with AGF, Reference, About) do not exist. `docs/about.md`
   does not exist. `navigation.indexes` is not enabled in the theme `features`; the pin is
   `mkdocs-material>=9.5,<10`, which supports it. Route/order constraint: regroup without moving
   files, switch last, after the accepted destinations (parent plan, order 6).

## Constraints from A1 that bind the Home rewrite

- **Home is counted by heading ids.** The 312-word Home component is the sum of ids `__top__`,
  `agf-hero-title`, `agf-routes-title`, `decide`, `set-up`, `first-governed-change` and `operate`.
  A renamed or removed heading drops out of the count silently. The same script then reports a
  lower number without any error.
- **Slack is almost nil.** Destination subtotals after slices 2 to 4 are Evaluate 670, Adopt 768
  and Operate 846 (305 + 541, !3464). Against the caps 982, 1,080 and 1,174, the Home component
  may be at most 312, 312 and 328 words. Operate has 16 words of room. Anything new in the counted
  range (for example the illustrative change) breaks rule 2 unless it sits outside the range or
  the reviewer and the governor re-map the range. A1 rule 2 says words moved out of the range count
  as saved only when the reader no longer needs them. That decision belongs to the reviewer and the
  governor, not to the author of Home.
- **Link transitions must not rise.** Evaluate and Adopt stay at 1 (Home to the landing page).
  Operate stays at 2 (Home to runbook to day-to-day). A direct Home link to the runbook is
  therefore load-bearing.
- **Non-selected options are informational.** A removed Home route still needs a reachable home.

## Other constraints

- **Pinned markup and links.** `check_commissioning_links.py` pins the hero section
  (`aria-labelledby="agf-hero-title"`, class `agf-hero`) and the CTA classes
  `agf-hero__action--primary` and `--secondary`, and reads their hrefs
  (today `docs/start/should-you-adopt/` and `docs/start/commission/`). The parent plan keeps
  the exact CTA destinations and classes. I did not check which hrefs the hook asserts.
  `ai_evaluation_dossier.py` lists `index.md` and `docs/index.md` in its page inventory.
  `mkdocs.yml` has `strict: true`.
- **Anchors to keep (no redirect plugin).** Home: `adopt`, `using-agf-with-other-tools`.
  `docs/index.md`: `how-to-read-this-site` (linked from `first-governed-change.md`),
  `1-start-decide`, `2-concepts-the-mental-model`, `3-adopt-install-with-eyes-open`,
  `4-operate-the-daily-loop`, `5-advanced-integration-and-customization`, `6-contribute`,
  `7-evolution-explicitly-non-binding`.
- **Solo, default and deputy.** Slice 3 put the default human approval path in
  `first-governed-change.md` and `commission.md`, and the canonical text is
  `roles-and-gates.md#approval-authority-and-optional-deputies`. Home should point to that
  default and to the Get started route for a solo maintainer. The deputy route is optional,
  default-off and provisional. It does not belong on the first screen, and Home should not
  restate or soften it.
- **Tooling claims.** The third-party coexistence sentence carries "Level 2 — Surface-defined,
  Theoretical evidence". If the sentence moves, the labels move with it
  (`implementation-honesty-rule`, `verifiable-claims-rule`). "No AGF-hosted service or agent
  runtime" is a claim the destinations support. Keep it tied to them.
- **`llms.txt`.** Its "`/docs/` — documentation learning path" line and reading paths go stale
  when the groups change. It is in slice 6 scope and should change with navigation, not before.
- **Order.** Home links into the runbook and day-to-day; those pages change in !3464, so
  Home anchors into them should be confirmed after that merge. Navigation switches last.

## Readiness limits

- Source reading of `origin/main` only. No rendered page, no screenshot and no built artifact
  was inspected for this note, so nothing here is visual, keyboard or mobile evidence.
- One reader, not user research. The three gaps are design findings, not measured results.
- The word counts are quoted from the A1 baseline and my earlier counter runs on !3463 and
  !3464, not recounted for this note.
- I authored slices 2 to 4 and !3465. I am not independent of those destinations.
- No claim of Gate 2, merge or deployment readiness for any Home, navigation or A3 work.

## Browser and image-viewing capability on this host

Checked read-only; nothing installed or started.

- No Chromium, Chrome, Firefox, Playwright or Puppeteer binary, and no Python `playwright`
  module. No `rsvg-convert`, ImageMagick or Graphviz `dot`. Node 24 is present with no browser
  package. The session has no browser tool.
- I can view a local PNG or JPG (and PDF pages) as an image with the file reader. I cannot render
  HTML or SVG to an image here.
- So a distinct-model rendered review by me needs images captured elsewhere, stored at a path
  this account can read, with their SHA-256 hashes recorded. I can also read built HTML text
  from a pipeline artifact. That checks wording, anchors and word counts, not layout.
