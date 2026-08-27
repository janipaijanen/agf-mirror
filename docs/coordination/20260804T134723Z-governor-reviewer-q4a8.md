---
kind: decision-record
from: governor-reviewer
to: all
id: 20260804T134723Z-governor-reviewer-q4a8
timestamp: 2026-08-04T13:47:23Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260804T130826Z-contribution-auditor-3skh
  - 20260804T124015Z-planner-architect-tsgk
topic: v260-805-746-scheduling-approved
requires-ack: false
artifacts_owned: []
---

# Governor decision: schedule !805 and !746 for v2.6.0 reconciliation

Codex-scribed transcription of the human governor's direct instruction:
"proceed as proposed. collaborate and coordinate."

## Binding decision

Approve the scheduling intent recommended in `3skh`:

- MR !805 remains the authoritative correctness and CLI/default-ref semantics
  plan for issues #84, #85, and #89.
- MR !746 remains the performance plan for bounded issue #76 work and must be
  narrowed so it does not redefine !805's default branch, effective-base, or
  remote-discovery contract.
- Planner-architect may refresh and reconcile both plans now, then request one
  coupled contribution-auditor review packet.
- After both plans receive separate binding Gate 1 approvals, implementation
  should use one coordinated v2.6.0 branch/MR with issue-separated commits and
  an acceptance map covering both plans. This is intended to avoid same-source
  conflicts and redundant acceptance pipelines without collapsing the two
  plan decisions.

## Explicit non-approval

This decision is **scheduling authority only**. It is not Gate 1 approval for
either current exact head and does not authorize implementation. Before Gate 1:

1. !805 must be refreshed against current `main`, add BREAKING impact and
   migration guidance, and name !746 as its performance dependency.
2. !746 must be refreshed against current `main`, remove the conflicting
   default/ref-selection design, and retain deterministic batching,
   invocation-count, benchmark, telemetry, and documentation evidence.
3. Contribution-auditor must review both revised exact heads together.
4. Governor-reviewer must then decide Gate 1 separately for each plan.

MR !1151 and held MR !1082 are separate tracks and are not authorized or
otherwise changed by this decision.
