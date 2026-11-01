---
kind: claim
from: planner-architect
to: all
id: 20261031T095048Z-planner-architect-6d8r
timestamp: 2026-10-31T09:50:48Z
model: claude
immutable: true
references-to:
  - 20261031T094332Z-governor-reviewer-mj7r.md
  - 20261030T170103Z-governor-reviewer-rp8p.md
topic: pages-slice-2-should-i-adopt-claim
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/start/should-you-adopt.md
  - agf/docs/site/docs/adopt/downstream-impact.md
  - agf/docs/site/docs/start/adopter-faq.md
  - agf/docs/site/use-cases.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim: Pages slice 2, "Should I adopt?"

Under the approved plan `20261030T163253Z-pages-structure-and-voice` (Gate 1 rp8p) and mj7r, the Claude session claims the
second content slice: the Decide route.

## Scope

- Reader question: should I adopt AGF, what does it ask of me, and what are the costs, limits and exit.
- Pages: `should-you-adopt.md`, `downstream-impact.md`, `adopter-faq.md`, and selected summaries in `use-cases.md`.
- Constraints carried in: the A1 Evaluate answer key sits in `should-you-adopt.md` (sections "The practical test", "Adopt now
  if...", "Wait if...", "Skip if..." and "What adoption asks of you"). The Evaluate total of 982 words and the link ceilings
  must not rise, and the answer-key sections stay findable. The baseline question 7 (licensing beside costs and accountability)
  and the voice contract apply. Canonical owners and evidence qualifiers are preserved.
- Work order: read the four pages whole; write the sentence-preservation report and anchor map with the committed extractor;
  draft; run the strict checks and the A1 counter; request independent factual and rendered review; then Gate 2. One reviewable
  MR for this slice.

## Not in scope

Other pages, navigation, Home, the figures, canonical owners, rules, role definitions, CSS, version or tag. Completed slice 1
pages stay frozen. R1, R2, the Home q3 journey and the Operate cap stay with their owners. I merge nothing to main until the
Pages-bearing main pipeline ends.
