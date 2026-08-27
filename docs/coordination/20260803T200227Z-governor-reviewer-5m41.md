---
kind: decision-record
from: governor-reviewer
to: all
id: 20260803T200227Z-governor-reviewer-5m41
timestamp: 2026-08-03T20:02:27Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T195539Z-contribution-auditor-aj5q
  - 20260803T194954Z-contribution-auditor-q4rh
topic: v250-rc1-observation-hold
requires-ack: true
artifacts_owned: []
---

# Hold at published v2.5.0-rc.1 for observation

Codex-scribed governor-reviewer clarification, transcribing the human
governor's live instruction after the post-rc.1 version-sequence consultation
was published.

## Binding decision

Remain on the published and verified `v2.5.0-rc.1` candidate for an
observation period and gather real learnings, if any. Do not begin a v2.5.0
final, v2.5.0-rc.2, v2.5.1-rc.1, or v2.5.1-rc.2 plan or release process yet.

The earlier `2.5.1-rc.1` → `2.5.1-rc.2` wording was not a release
authorization. Consultation `20260803T195539Z-contribution-auditor-aj5q`
therefore closes without creating a release-planning obligation.

## Observation boundary

Collect only evidence that actually appears, including:

- installation, upgrade, or archive-overlay failures;
- packaged CLI/MCP behavior mismatches;
- documentation misunderstandings on the five shipped surfaces;
- provenance, checksum, OCI, Release, or generic-package defects;
- adopter or independent-review feedback; and
- regressions attributable to the immutable rc.1 artifact rather than to
  unrelated `main` development.

Record a learning only when supported by concrete evidence and the learning
schema; do not manufacture a lesson merely because an observation period was
requested. Route an actual correction through its own governed plan and use
rc.2 only if the existing rc.2 trigger is genuinely met. Stable v2.5.0 remains
separately Gate 1-gated and begins only on a later explicit governor decision.

No fixed elapsed-time threshold is implied by “for a moment.” Poll and report
new evidence when active or resumed; the governor decides when the observation
period is sufficient.
