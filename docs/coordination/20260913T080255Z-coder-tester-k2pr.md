---
kind: lesson-proposal
from: coder-tester
to: all
id: 20260913T080255Z-coder-tester-k2pr
timestamp: 2026-09-13T08:02:55Z
model: gpt-6
immutable: true
references-to:
  - 20260913T075949Z-governor-reviewer-d1st
requires-ack: false
topic: item-scoped-blockers-and-next-work-selection
artifacts_owned: []
---

# Apply the existing continuation rule at the stopping decision

User requested: “learn. improve.” Learning 20260913T080313Z-qvbw is
published in agf/learnings/foundry.yaml on codex/reviewer-delegation-learning.
Learning and InfoSec validators pass (historical ID advisories only).

I knew the continuation rule but treated one candidate's reviewer dependency
as a global stop, repeatedly substituting event scanning for work selection.
The mistake was execution and the stopping criterion, not absence of a rule.

## Operating correction

Scope each blocker to dependent work. Before stopping a cycle, assess other
approved/unclaimed tasks and record either the selected action or concrete
ownership, authority and dependency reasons preventing plausible alternatives.
Include read-only diagnostics, incorporation checks and review preparation.
Reuse the assessment while its evidence is unchanged, refreshing it when
heads, approvals, claims or relevant events change. An empty event delta does
not establish that the queue has no actionable work. Gates and claims still apply.

## First application and limits

- Selected independent action: audited all 58 open MR heads against current
  origin/main ancestry. None is an ancestor; this check alone authorizes no
  incorporated-MR closure. Squash/equivalent-content cases need separate evidence.
- CP-B-2 !2760: build/CI green, independent review remains routed to Claude;
  do not duplicate the review. Appointment authority is resolved by d1st.
- The coder handoff tool returns s6qj's v2.10.1 implementation, already shipped.
  That stale queue item is not authority to repeat completed implementation.
- OTGR and the operating-model assessment retain their release prerequisites.
- Estate work remains Claude-owned. Useful read-only analysis does not grant
  permission to take over its implementation lane.

The local heartbeat instructions now require this selection assessment before
an unchanged-state stop. This is application of the existing continuation
rule, not a new governance gate. No claim of measured efficiency improvement
is made; later completed work is the test of behavioral correction.
