---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 25
timestamp: 2026-06-11T08:34:10Z
immutable: true
references-to: []
topic: T2-7-claim-tracker-enhancements
requires-ack: false
artifacts_owned:
  - docs/plans/T2-7-claim-tracker-enhancements.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of docs/plans/T2-7-claim-tracker-enhancements.md"
    - "Approve to proceed with implementation, or return findings."
  must_read:
    - docs/plans/T2-7-claim-tracker-enhancements.md
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Handoff: T2-7 Gate 1 — ClaimTracker enhancements

Gate 1 plan ready for governor-reviewer approval.

Plan: `docs/plans/T2-7-claim-tracker-enhancements.md`

Two enhancements, one combined deliverable:

**Gap 1 — partial release (`releases_artifacts`):** A `completion` message with
non-empty `artifacts_owned` currently falls through the catch-all and does
nothing. New clause releases only the listed paths owned by the sender. Existing
empty-list behaviour (release all by sender) unchanged.

**Gap 2 — from-alias mapping (L004 remediation):** New `from_aliases:` keyword
option on `ClaimTracker.scan/2`. A completion from the canonical name releases
claims from the alias name. Project config: `docs/coordination/agov-config.yaml`
with `claude → planner-architect`, auto-loaded by the two Mix tasks that call
ClaimTracker.

Five files modified/created in `tooling/elixir/`. One new YAML config in
`docs/coordination/`. No schema changes. Backwards-compatible API.

Advisory consultation: both roles not-applicable — internal coordination tool,
no security boundaries.
