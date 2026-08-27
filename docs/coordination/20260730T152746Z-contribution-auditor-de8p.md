---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260730T152746Z-contribution-auditor-de8p
timestamp: 2026-07-30T15:27:46Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260730T150606Z-contribution-auditor-tsgk
topic: v246-release-roadmap-version-surface
requires-ack: false
artifacts_owned:
  - docs/plans/v246-release.md
---

# v2.4.6 release amendment: distributed roadmap is a sixth truth surface

# Review evidence envelope

Scope: `docs/plans/v246-release.md` on MR !834, reviewed at prior head
`6cdb3c1031b9c94278bb3f8177467789e33a1079` and corrected by this record's
commit
Coverage: COMPLETE
Recommendation: BLOCK until the roadmap surface is included; request fresh
Gate 1 approval only at the corrected exact head
Blocking findings: 1, resolved by the accompanying plan correction
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1

Check: release-version-surface-sweep
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: R1
Impact: material
Enforcement: blocking
Basis: `agf/dist/release-manifest.yaml` includes `agf/docs/site/**`, and
`agf/docs/site/roadmap.md` statically calls v2.4.5 the latest release
Finding state: resolved
Risk disposition: none
Resolution condition: the release plan and future release MR include a narrow
roadmap baseline update to v2.4.6 and reclassify issue #81's v2.4.5 archive
failure as historical/fixed

The tracked Claude example loaders contain no hard-coded AGF release scalar,
so they require no mechanical version bump. Root `AGENTS.md` likewise contains
no pinned release scalar. Historical changelog/review evidence and the
self-sufficiency script's synthetic fixture names are not current-release
claims and remain unchanged.
