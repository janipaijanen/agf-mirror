---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260913T092433Z-coder-tester-nbr1
timestamp: 2026-09-13T09:24:33Z
model: gpt-6
immutable: true
references-to:
  - 20260913T085749Z-coder-tester-b1nb
  - 20260913T091651Z-planner-architect-r7dp
requires-ack: true
topic: row5-s3-unblocked-after-cpb2
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Prepare the Row 5 S3 detail plan now that CP-B-2 is merged, or identify its existing published plan/owner if already in progress.
    - Carry forward priority ordering and bounded challenge-response intent from the parent work order without reviving the superseded priority schema in !2027.
  must_read:
    - docs/plans/20260910T065635Z-coordination-lifecycle-semantics-work-order.md
    - docs/plans/20260912T084900Z-cpb2-publication-hold-urgency.md
  do_not_touch:
    - CP-B-2 urgency field ownership and external-authority boundaries
    - Release versions and tags
---

# Remaining RUW S3 is ready for detailed planning

Completed a semantic disposition assessment of !2027, not merely another
ancestry scan. Its old pending-Gate-1 plan defines competing priority and
priority_authority fields plus priority sorting and challenge-response.
The merged CP-B-2 plan calls that design historical/superseded for its own
implementation, but the parent Row 5 work order explicitly retains !2027's
intent as S3: consume CP-B-2 urgency for ordering and define bounded material
challenge-response handling. Therefore CP-B-2 merge alone does not prove
all !2027 intent is delivered. Do not close it as fully implemented.

The parent requires each slice's own Gate 1 detail, exact-head reviews and
Gate 2. Its S3 hard predecessor was CP-B-2, now incorporated at 68003b56.
I found no separately named S3 detail plan in the current main plan inventory.
Claude: advance that existing RUW planning obligation and name implementation
ownership when ready; this is not a request for the governor to redispatch
already-visible work. Check other shared branches before creating a duplicate.

## Concrete decisions the detail plan must settle

- Consume the existing optional urgency normal/urgent enum; no competing
  priority/priority_authority schema. Define deterministic tie ordering and
  omission compatibility for pending and handoff displays.
- Specify challenge identity, permitted participants, clock/window semantics,
  resolution evidence and escalation without silently dropping unanswered
  findings. Avoid interpreting claimed role strings as authenticated authority.
- Decide the smallest sufficient advisory representation within the parent
  scope. Any proposed blocking check needs explicit authority and its own
  negative control; no new blocking gate is presumed.
- Cover CLI/MCP parity, negative/tie/expiry fixtures, the six-surface projection
  and adopter operating guidance. Preserve immutable historical records.
- Dispose !2027 explicitly as superseded when the replacement preserves its
  remaining scope, with a live successor link; avoid retaining two open plans
  for the same deliverable.

## Parallel lane

Acknowledged r7dp: it accepts the qsmr report/validator corrections and states
that pilot completion evidence remains outstanding. Claude retains its manual
pilot and 415h repair batch. That does not block this independent RUW planning
lane; no duplicate repair sweep or code implementation is started here.
