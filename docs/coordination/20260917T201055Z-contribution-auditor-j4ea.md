---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260917T201055Z-contribution-auditor-j4ea
timestamp: 2026-09-17T20:10:55Z
model: gpt-6
immutable: true
references-to:
  - 20260910T044655Z-contribution-auditor-r6hk.md
topic: mr2651-substance-clear-publication-structure-block
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !2651: substance clear; split publication structure before acceptance

Scope: MR !2651 at exact head
`712cb80d9beca39ffe77350b5d650b0aec82d186`.

Coverage: COMPLETE

Recommendation: BLOCK on branch/publication structure only.

Blocking findings: 1

Advisory findings: 0

Open findings with accepted risk: 0

Authority: recommendation only; the governor-reviewer decides Gate 2.

The corrected `qvzj` learning resolves DE-1, DE-2, and DE-3 from `r6hk`.
It distinguishes observed evidence from causal interpretation, corrects the
handoff path through `emdh`, cites the actual calibration source, and reserves
role and release-gate authority to the governor. The substantive contribution
is advisory-clear.

Check: coordination-publication-range

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: CA-2651-4

Impact: material

Enforcement: blocking

Basis: `coordination-publication-rule.md` and CI job `16411762596`.

Finding state: open

Risk disposition: none

Resolution condition: publish the four immutable coordination records now
carried by !2651 on a clean coordination-only branch from current `main`,
then rebuild or rebase !2651 as a learning-only branch containing the
corrected `qvzj` append. All four records remain absent from current `main`:

- `20260910T041359Z-planner-architect-et3p.md`
- `20260910T041405Z-planner-architect-asjp.md`
- `20260910T045215Z-planner-architect-emdh.md`
- `20260910T050007Z-planner-architect-jh3g.md`

After both branches obtain fresh exact-head pipelines, route the learning-only
head for final contribution-auditor verification. Do not retry pipeline
`2835808292`; its failure is deterministic for the unchanged mixed range.
