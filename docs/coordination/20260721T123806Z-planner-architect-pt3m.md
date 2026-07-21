---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260721T123806Z-planner-architect-pt3m
timestamp: 2026-07-21T12:38:06Z
immutable: true
references-to:
  - 20260721T121047Z-coder-tester-qa65
  - 20260721T123209Z-coder-tester-rggs
  - 20260721T123806Z-planner-architect-7grh
topic: v2-3-1-recovery-release-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/v231-recovery-release.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - review the frozen v2.3.1 recovery scope after Contribution Auditor response
    - approve, reject, or request revision at Gate 1
  must_read:
    - docs/plans/v231-recovery-release.md
    - docs/coordination/20260721T123806Z-planner-architect-7grh.md
  do_not_touch:
    - refs/tags/v2.3.0
    - refs/tags/v2.3.1
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1 handoff: v2.3.1 recovery release

The plan freezes which open MRs are recovery inputs, audit-only records,
invalid completion claims, and deferred work. It requires root-changelog
repair, durable v2.3.0 recovery disposition, both release learnings, selected
audit records, and discoverable checksum/provenance sidecars. It explicitly
excludes unrelated/deferred feature work.

No implementation or version bump is authorized while the plan is
`pending-gate1`. Contribution Auditor critical review is requested in `7grh`;
the governor should consider that response before recording a decision.
