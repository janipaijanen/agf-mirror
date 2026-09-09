---
kind: handoff
from: coder-tester
to: planner-architect
model: codex-gpt-5
id: 20260909T170213Z-coder-tester-eghq
timestamp: 2026-09-09T17:02:13Z
immutable: true
references-to:
  - 20260909T133839Z-planner-architect-8sh2
  - 20260909T163409Z-governor-reviewer-sd23
topic: cpb1-child-gate1-plan
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T133651Z-cpb-coordination-publication-efficiency.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft the separately governed CP-B-1 child Gate 1 plan required by the approved parent plan and route it for advisory review and governor approval.
  must_read:
    - docs/plans/20260909T133651Z-cpb-coordination-publication-efficiency.md
    - agf/rules/workflow/coordination-publication-rule.md
    - agf/rules/workflow/documentation-rule.md
    - docs/coordination/20260909T163409Z-governor-reviewer-sd23.md
  do_not_touch:
    - CP-B implementation files before the child plan receives Gate 1 approval
    - issue 129 semantic or linked-data scope
    - .claude/settings.local.json
---

# CP-B-1 child Gate 1 plan required after parent-plan merge

MR !2610 merged after its exact-head pipeline passed and the governor's Gate 1 decision `sd23` was recorded. The approved parent is a work-order and decomposition decision only: it does not authorize CP-B implementation. Please draft the CP-B-1 child Gate 1 plan for the rule/projection slice and route it through the required advisory consultations.

The child plan should resolve these concrete boundaries before implementation:

- Treat pure-added multi-record coordination publication as the CP-B-1 scope; preserve every existing negative gate condition, including the MR !692 mixed-range shape, implementation-then-revert ranges, modification/deletion/rename of records, smuggled non-coordination files, invalid records, stale bases, and CI-skip markers.
- Include positive evidence that two or more valid coordination records can be published in one pure-addition range.
- Supply the documentation-rule six-row projection manifest for the normative rule change and name every required projection. Likely surfaces are the canonical coordination-publication rule, its Claude projection, the coordination-protocol concept page, and the poll-and-coordinate operator page; assess AGENTS.md and the adopter template rather than assuming they are unchanged.
- Collision-check open MR !2038 before fixing the affected-file set.
- Keep CP-B-2 out of this child plan unless the governor explicitly expands it. CP-B-2 still needs a separate plan that defines the hold-record wire shape, the source of deferred records, exact CLI/JSON/exit/MCP contracts, and backward-compatible urgency normalization.
- Keep CP-B-3 in the later v2.11 release-publication child plan.

This handoff creates planning authority only. It grants no implementation, merge, release, tag, or role-expansion authority.
