---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261031T030035Z-planner-architect-rjns
timestamp: 2026-10-31T03:00:35Z
model: claude
immutable: true
references-to:
  - 20261031T024603Z-planner-architect-jner.md
  - 20261031T022614Z-governor-reviewer-qpjq.md
topic: pages-g4-g5-candidate-ci-defect-fixed-new-head
supersedes:
  - 20261031T024603Z-planner-architect-jner
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Route the reviews listed in jner against the new head, after the new pipeline result is read.
  must_read:
    - docs/coordination/20261031T024603Z-planner-architect-jner.md
  do_not_touch:
    - agf/rules/
    - agf/docs/site/reference/figures.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# G4 and G5 candidate: CI defect fixed, new exact head

This record replaces the head in jner. The reviews and limits listed in jner still apply. Any change to the head
invalidates this readiness claim.

| Item | MR | Exact head | Branch |
|---|---|---|---|
| G4 and G5 source alignment (draft) | !3460 | `73823036` (full SHA in the MR; short form read from pipeline 3011006873) | `docs/pages-g4-g5-source-alignment` |

## What went wrong

Pipeline 3010991496 at `48ee105a` failed in `site-html-build` (job 17626241394). This was a defect in my change, not
infrastructure. The `figure_accessibility` build hook requires the exact sentence "AI-generated figure. Publication
remains the responsibility of the human maintainer." within five lines of the figure embed. My caption rewrite put other
words inside that sentence, so the hook rejected it. The hook did its job. I had run the palette and stale-label hooks
locally but not this one, and I wrongly listed the build as "CI will run it" without checking which hooks it contains.

## Fix

The caption is restored byte-for-byte. The default-model context moves to its own sentence after the caption:
"The figure illustrates the default operating model; its "human-only" labels describe that default." The alt text still
names the default model. I confirmed the hook's disclosure pattern matches in the five lines after the embed. Nothing
else changed. The old job is not retried. Pipeline 3011006873 is the next already-required run, and I will report it.

## Reviewer note

The rbkp item 5 asked for the caption to identify the default model. The fixed layout does that with an adjacent
sentence instead of inside the disclosure line. If the reviewers read that as weaker context, tell me and I will propose
a different placement that keeps the hook's pattern intact. I will not change the hook.
