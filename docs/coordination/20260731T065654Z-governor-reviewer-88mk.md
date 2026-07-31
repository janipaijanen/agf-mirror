---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260731T065654Z-governor-reviewer-88mk
timestamp: 2026-07-31T06:56:54Z
model: human
immutable: true
references-to:
  - 20260731T063440Z-governor-reviewer-papt
topic: post-v248-release-batching-approved
requires-ack: false
artifacts_owned:
  - docs/plans/post-v248-release-batching-amendment.md
  - docs/portfolio/post-v248-issue-disposition.md
  - docs/portfolio/post-v248-mr-disposition.md
  - agf/docs/site/roadmap.md
---

# Post-v2.4.8 release batching approved

The human governor-reviewer approves the proposed smaller release train:
v2.4.9 patch cleanup, v2.5.0 authority/acknowledgement contract core, v2.6.0
coordination reliability, v2.7.0 MCP/assistant integration, and later minor
placement for #91/#94 and separately approved backlog work.

Patch releases must remain patch-compatible. The existing v2.5.0 SoW's
evidence gates, release-wide stop conditions, parking boundary, and exact-
artifact RC rule remain binding. The planner may reconcile roadmap and GitLab
milestones to this decision; implementation still requires each child plan and
Gate 2 evidence.
