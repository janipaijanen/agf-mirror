---
kind: handoff
from: planner-architect
to: planner-architect
id: 20260905T215754Z-planner-architect-gprk
timestamp: 2026-09-05T21:57:54Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260905T205921Z-planner-architect-pm6m.md
  - 20260905T213622Z-planner-architect-gph5.md
topic: universal-polling-spare-capacity-rule-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the spare-capacity and visible-decomposition addition at MR !2405 exact head 4904bdc6.
    - Challenge whether its WIP, issue-board, coordination, CI-cost, collision, and atomic-change boundaries are sufficient and implementable.
  must_read:
    - docs/plans/20260905T210255Z-universal-agent-autonomous-polling.md
  do_not_touch:
    - plan/universal-agent-autonomous-polling
---

# Review request: recurring spare-capacity loop for every development agent

The governor requires every active AGF development agent to keep asking, at
safe wait boundaries, what useful work it can advance without blocking its
current pipeline or duplicating another actor. MR !2405 head `4904bdc6`
incorporates this as a normative rule design rather than an optional prompt.

The proposed loop requires the agent to:

1. ask for the highest-priority authorized small item that will not delay,
   invalidate, or collide with work already in flight;
2. claim at most one spare lane alongside one primary lane;
3. keep release-critical defect removal and approved roadmap work above
   unrelated research or polish;
4. when a valuable item is too large, collision-scan first, then create/update
   a parent plus linked bounded child issues/tasks on the issue board and
   publish a coordination note;
5. give each child an outcome, artifact scope, dependency/current gate,
   acceptance evidence, and ownership state;
6. on the next feasible poll, re-read remote state and claim a child only if it
   remains open, authorized, unclaimed, and unanswered/unworked by another AI
   or human; and
7. reject decomposition that would split an atomic safety or release change
   into invalid intermediate states.

The cadence remains non-reentrant: scheduled polls defer/coalesce during active
work and one runs at the next safe boundary. The spare-capacity loop is invoked
from that completed poll or another safe wait boundary; it must never interrupt
the primary execution or amend an exact head whose acceptance pipeline is
running.

Please review the exact plan delta independently. In particular, try to break
the proposed default limit of one primary plus one spare lane, the definition
of a safe wait boundary, the “no other actor responded or worked it” remote
revalidation condition, issue-board decomposition quality, and the protection
against duplicate/pipeline-spam work. Publish a durable CLEAR or numbered
findings. This is design review only; Codex retains the plan branch and neither
session has implementation authority before Gate 1.

