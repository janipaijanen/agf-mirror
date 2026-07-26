---
artifacts_owned:
- docs/plans/public-version-and-docs-provenance.md
from: coder-tester
handoff:
  do_not_touch:
  - agf/VERSION
  - Git tags and released artifacts
  - GitLab protected-tag or project permission settings
  - promotion and mirror artifacts
  must_read:
  - docs/plans/public-version-and-docs-provenance.md
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/594
  next_agent: governor-reviewer
  status: ready
  task:
  - independent fact-check of exact-SHA and public-claim evidence in MR !594
  - Gate 2 review and disposition
id: 20260726T124701Z-coder-tester-kdsm
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260726T114427Z-planner-architect-g4ak.md
- 20260726T122556Z-planner-architect-smh3.md
- 20260726T123745Z-coder-tester-sagq.md
requires-ack: true
timestamp: 2026-07-26T12:47:01Z
to: governor-reviewer
topic: public-version-truth-implementation-gate2
---

# Public-version-truth implementation: ready for fact-check and Gate 2

Implemented the approved plan on `feat/public-version-docs-provenance`
(MR !594, commit `88e2997868fbd3501384018bd11c56022ef72b4b`, base
`dc06ad87e17e34f153f9a2d9c2502f376f1b07b1`). Full evidence is in the MR
description: exact-SHA hook tests (11/11 green, covering the three required
acceptance cases), the new stale-version-label scanner's self-test negative
cases, strict `mkdocs build`, rendered `release-info.md` text, dossier
plain-text extraction, and header `aria-label` parity. All stated
do-not-touch boundaries from `g4ak` preserved. Not merged.

Requesting fact-checker verification of the exact release tag/commit claims
and public wording against Git refs, then governor Gate 2 disposition.
