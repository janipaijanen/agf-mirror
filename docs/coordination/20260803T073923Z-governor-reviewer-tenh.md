---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260803T073923Z-governor-reviewer-tenh
timestamp: 2026-08-03T07:39:23Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T071815Z-governor-reviewer-4d5t.md
topic: historical-branch-only-coordination-recovery-prevention
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "After MR !1002 merges, assess the pre-2026-08-01 branch-only coordination inventory and produce one evidence-backed root cause and governed disposition per record."
    - "Draft a Gate 1 prevention plan covering remote visibility, exact-head pipeline gaps, MR closeout, bridge notes, and branch/worktree cleanup; preserve tool-authority boundaries."
    - "Coordinate with coder-tester, build-engineer, and contribution-auditor before implementation; this handoff does not authorize implementation or historical-record publication."
  must_read:
    - docs/TODO.md
    - docs/coordination/20260803T071815Z-governor-reviewer-4d5t.md
  do_not_touch:
    - .claude/settings.local.json
---

# Recover and prevent historical branch-only coordination

The human governor requested that the older branch-only records discovered by
the broader visibility scan be handled after the bounded August 1--3
consolidation, and that the project determine why they were created but left
unpublished and how to prevent recurrence. MR !1002 records the full assessment
and prevention scope in `docs/TODO.md`.

The forensic pass must not bulk-merge old branches or infer that age, a merged
implementation, a bridge note, or a closed MR disposes of a unique immutable
record. Each record needs evidence-backed provenance and a governor-approved
disposition before recovery or exclusion.

## Publication-race evidence from MR !1002

The TODO's own push/MR transition created two pipelines for exact SHA
`c9f36f6f179c7ada7e4a5d82859f5ea97d7086fe`:

- source-branch pipeline 2726359644 started first; its governance job had used
  approximately 49 seconds when cancellation was requested;
- MR-head pipeline 2726360527 is the canonical acceptance path and was
  preserved;
- the redundant source pipeline was promptly moved to canceling state.

This is concrete evidence for the prevention assessment: publication and MR
creation order can create duplicate CI for one SHA, while absent/skipped
pipelines can create the opposite failure mode. The design should treat both
as lifecycle-state problems rather than relying on commit titles or operator
memory.

