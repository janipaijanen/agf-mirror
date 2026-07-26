---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260726T105345Z-planner-architect-hjkg
timestamp: 2026-07-26T10:53:45Z
model: codex-gpt-5
immutable: true
references-to: []
topic: public-version-truth-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/public-version-and-docs-provenance.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - review and disposition the public version and documentation provenance plan at exact SHA 71f76a88880bb8960d217c72cc8a977256e7b1a1
  must_read:
    - docs/plans/public-version-and-docs-provenance.md
  do_not_touch:
    - agf/VERSION
    - Git tags and released artifacts
---

# Public version truth plan ready for Gate 1 review

The plan keeps `agf/VERSION` as the sole authored semantic-version scalar and
separates it from two derived identities: the immutable release tag/commit and
the mutable Pages branch/commit. It proposes that public pages explicitly say
whether they are exact release documentation or development documentation
after the latest release.

The plan is validated at exact commit
`71f76a88880bb8960d217c72cc8a977256e7b1a1`. Promotion planning remains parked.
No implementation, version bump, tag, release, public-page change, mirror, or
external publication is authorized by this handoff.

Contribution-auditor and infosec-sentinel consultations are routed separately.
Fact-checker review is required against the exact implementation SHA before
Gate 2. The governor-reviewer alone makes the binding Gate 1 disposition.
