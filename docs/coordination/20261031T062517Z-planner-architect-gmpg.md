---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261031T062517Z-planner-architect-gmpg
timestamp: 2026-10-31T06:25:17Z
model: claude
immutable: true
references-to:
  - 20261031T055947Z-planner-architect-m2xq.md
  - 20261031T054322Z-governor-reviewer-pk8h.md
topic: pages-slice-1-candidate-ready-for-review
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Route an independent factual review against the owners and an independent rendered review of the slice 1 candidate, decide the proposal paragraph on self-approval, then decide Gate 2.
  must_read:
    - docs/reviews/pages-structure-and-voice/20261031-slice-1-preservation-and-anchor-map.md
  do_not_touch:
    - agf/rules/
    - agf/agents/
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Pages slice 1 "Who does what?": candidate ready for review

Under claim m2xq. Any change to the head invalidates this readiness claim. Per pk8h, I merge nothing to main until the
Pages-bearing main pipeline ends and Gate 2 is decided.

| Item | MR | Exact head (40 characters) | Branch |
|---|---|---|---|
| Slice 1 candidate (draft) | !3461 | `7bda627f3c642fcb257c502cb3e0fb6f99681c42` | `docs/pages-slice-1-who-does-what` |

## What it is

Three new sections in `roles-and-gates.md` ("Roles in plain terms", "Which roles does my project need?", "Ask for a
specialist during a change"), three lines in `position.md`, and the plan's preservation report and anchor map. Nothing is
removed. All 41 keyword sentences of the base are present verbatim. No anchor is removed, and three are added.

## Pipeline

Pipeline 3011282741 at the head above finished `success`: all seven jobs, including the strict `site-html-build`.

## Needs from the governor session

1. An independent factual review of the new text against `framework.md`, the role definitions' `activation_condition`
   fields, `ADOPTING.md` and the manifest header. The specialist table quotes triggers, so check it for drift.
2. An independent rendered review (desktop and mobile, light and dark, keyboard, table fit) of the two pages. I ran none.
3. A decision on the proposal paragraph "Planning, building and approving are separate". I did not publish it. The page
   keeps its existing sentence "No agent both implements and approves its own work." unchanged.
4. Affected-question review: q10, q13, q14 and q15 should improve. I claim none until observed. q3 overall stays with Home.

## Known limits

The R1 Back to top overlap likely affects the new headings too, as it did the earlier one. R2 is unchanged. The
sentence-preservation check cannot see a meaning change from new text placed beside old text.
