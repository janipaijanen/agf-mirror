---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260917T184959Z-contribution-auditor-hq7s
timestamp: 2026-09-17T18:49:59Z
model: gpt-6
immutable: true
references-to:
  - 20260917T040118Z-contribution-auditor-bhmj.md
topic: mr2758-correction-followup
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !2758 correction review: substantive findings resolved, one schema fix remains

Scope: MR !2758 at exact head
`b8048e5676e14973dc432fe8235ca6ccbfed69e0`.

Coverage: COMPLETE

Recommendation: BLOCK on one mechanical correction and acceptance refresh.

Blocking findings: 1

Advisory findings: 0

Open findings with accepted risk: 0

Authority: recommendation only; the governor-reviewer decides Gate 2.

The corrected learning resolves all three substantive findings from `bhmj`:
it now states the !2757 chronology accurately, distinguishes absence from
`main` from total invisibility, and supplies durable severity, confidence,
and evidence fields.

Check: learning-schema-confidence-level

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: CA-2758-4

Impact: minor

Enforcement: blocking

Basis: `agov.validate.learnings` and the governed learning schema.

Finding state: open

Risk disposition: none

Resolution condition: replace invalid `confidence.level: single-session`
with schema-valid `single-adopter`, rebase the branch onto current `main`,
and obtain a fresh exact-head MR pipeline. The stale pipeline's separate
`plan-collision-block` deletion failure came from its 509-commit-old target
base and should disappear after rebase. Route the new head for final
contribution-auditor verification; no repeat review of the resolved
substantive findings is needed.
