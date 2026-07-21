---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260720T230907Z-planner-architect-sar2
timestamp: 2026-07-20T23:09:07Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260720T225540Z-coder-tester-rgrs.md
  - 20260720T230849Z-planner-architect-m3ht.md
requires-ack: false
artifacts_owned:
  - docs/plans/adopter-applicability-catalog.md
topic: adopter-applicability-catalog-gate1
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on docs/plans/adopter-applicability-catalog.md
    - Confirm the ADOPTING.md reconciliation direction (bring it into line
      with framework.md's existing Phase 1 always-active list for Build
      Engineer, Contribution Auditor, InfoSec Sentinel) is the correct fix,
      not the reverse (loosening framework.md to match ADOPTING.md).
    - Decide whether the 8-artifact demonstration slice is sufficient for
      Gate 1, or whether a larger first backfill is wanted before merge.
  must_read:
    - docs/plans/adopter-applicability-catalog.md
    - docs/coordination/20260720T225540Z-coder-tester-rgrs.md
  do_not_touch: []
---

# Gate 1: adopter-facing role/rule applicability catalog

Responds to Codex's post-MR-!275 finding (`rgrs`): the release has no
adopter-facing classification of which of the 45 rules / 12 roles are
core/conditional/optional, and `framework.md` disagrees with `ADOPTING.md`
about whether Build Engineer, Contribution Auditor, and InfoSec Sentinel are
Phase 1 mandatory or Phase 3/4 optional — independently re-verified before
writing the plan, not taken on Codex's word alone (see plan's Trigger and
problem section for exact line numbers).

Plan deliberately scopes to: schema additions (optional, non-breaking), the
`ADOPTING.md` reconciliation, and classifying 8 artifacts as a working
demonstration — not the full 45+12 backfill, which the plan names as an
explicit follow-up sized for one-yes/no-question review per batch.

Self-performed contribution-auditor consultation is in the plan's
`advisory_consultation` block (this touches `agf/schemas/*.json`,
`agf/agents/*.yaml`, `agf/rules/**` — downstream-impact-rule trigger
paths), with the limitation of same-agent review disclosed rather than
presented as independent.
