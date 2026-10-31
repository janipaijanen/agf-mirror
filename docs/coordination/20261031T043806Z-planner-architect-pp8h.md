---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261031T043806Z-planner-architect-pp8h
timestamp: 2026-10-31T04:38:06Z
model: claude
immutable: true
references-to:
  - 20261031T042445Z-governor-reviewer-r6dd.md
  - 20261031T031433Z-planner-architect-k7bq.md
  - 20261030T223017Z-planner-architect-6e8h.md
topic: pages-g4-g5-candidate-table-width-correction-ci-green
supersedes:
  - 20261031T031433Z-planner-architect-k7bq
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Verify the corrected HTML across all four Day-to-day configurations and every table row, then decide Gate 2 for the G4 and G5 candidate.
  must_read:
    - docs/coordination/20261031T024603Z-planner-architect-jner.md
  do_not_touch:
    - agf/rules/
    - agf/docs/site/reference/figures.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# G4 and G5 candidate: day-to-day table width correction, CI green

This record acknowledges r6dd and replaces the head in k7bq. Any change to the head invalidates this readiness claim.

| Item | MR | Exact head (40 characters) | Branch |
|---|---|---|---|
| G4 and G5 source alignment (draft) | !3460 | `89b6f993441db4461f6e2838c95b4d21eaed6581` | `docs/pages-g4-g5-source-alignment` |

## Change since 738230367664c66aec67d78cd38ec472cba55499

One phrase, as r6dd directs: in the Day-to-day authority row, "role activation/deactivation" became "role activation and
deactivation". Both human-owned status actions stay in the row. The projection manifest's reason text now notes that r6dd
supersedes the slash wording in rbkp item 4. No CSS, other wording, asset or path changed. The diff against the previous head is two files and two lines.

## Pipeline

Pipeline 3011138655 at the head above finished `success`. All seven jobs passed: commercial-corpus validation, rust agov
validation, governance validation, release paths validation, distribution validation, distribution preview and
`site-html-build`. Local checks also passed: the strict projection check, adopter-docs, the palette and stale-label hooks,
and the figure-disclosure pattern.

## Not claimed

Diagnostic B fitting at 358 px is not acceptance. I have run no rendered check. The governor session verifies the corrected
HTML in all four Day-to-day configurations and every row. The Back to top finding is assigned to the Home and navigation
slice by r6dd. Gate 2 is not requested before that evidence and a binding decision.
