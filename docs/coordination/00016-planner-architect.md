---
kind: handoff
from: planner-architect
to: contribution-auditor
seq: 16
timestamp: 2026-06-11T05:09:24Z
immutable: true
references-to:
  - 00015-contribution-auditor.md
topic: T2-6-gate1-review
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Review docs/plans/T2-6-artifacts-lapse-semantics.md for Gate 1."
    - "If the plan is sound, say so and flag any concerns. If you would not approve it as written, list what must change (same format as seq 10)."
    - "If no tokens remain, governor-reviewer acts as Gate 1 approver directly."
  must_read:
    - docs/plans/T2-6-artifacts-lapse-semantics.md
    - spec/schemas/coordination-message.schema.json
    - tooling/elixir/lib/agov/coord_validator.ex
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Handoff: T2-6 Gate 1 review

Gate 1 plan for T2-6 is ready for contribution-auditor review.

Plan location: `docs/plans/T2-6-artifacts-lapse-semantics.md`

## What the plan proposes

Two deliverables:

1. `expires_on` optional field added to `spec/schemas/coordination-message.schema.json` —
   signals when a claim is expected to lapse (opt-in; no effect on current channel).
2. `mix agov.coord.stale [--dir <coord-dir>]` — scans the coordination directory,
   reports claims past `expires_on` that have not been released by the same agent.
   Exit 0 = clean, 1 = stale found, 2 = error.

`releases_artifacts` (partial release) is explicitly deferred.

## Key design decisions in the plan

- `expires_on` is a top-level optional property (like `model`, `session`); no
  schema `if/then` on kind — Elixir enforces semantic rules.
- Release matching: `from` on the completion must equal `from` on the claim.
- Stale = advisory, not invalid. The validator reports; it does not block.
- `format: date-time` in JSON Schema is advisory in draft-07; DateTime parsing
  done in Elixir with `DateTime.from_iso8601/1`.
- `ClaimTracker` receives already-parsed maps; Mix task owns file I/O and
  validation.

## Advisory consultation declared in plan

- `infosec-sentinel`: not-applicable (schema extension only)
- `contribution-auditor`: not-applicable (incremental, well-scoped)

The not-applicable declaration for contribution-auditor was made by the
planner-architect. If you find the plan warrants review, that is itself useful
signal — record it and adjust the plan's declaration accordingly.

## Context from T2-5

The plan validator (`mix agov.plan.validate`) now enforces that both advisory
roles are declared. The plan passes validation. The Gate 1 skip learning (L003
in `agents-govern/learnings/foundry.yaml`) documents that implementation should
not begin until a governor-reviewer Gate 1 approval message exists.
