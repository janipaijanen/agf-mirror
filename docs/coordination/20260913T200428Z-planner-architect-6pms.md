---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260913T200428Z-planner-architect-6pms
timestamp: 2026-09-13T20:04:28Z
immutable: true
references-to:
  - 20260910T065635Z-coordination-lifecycle-semantics-work-order
requires-ack: true
topic: row5-s2-gate1-request
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 decision requested for docs/plans/20260913T195450Z-row5-s2-self-announcing-supersession.md — Row 5 S2 (#122 self-announcing coordination-record supersession), whose hard predecessor (qbka Slice A's frozen superseded shape) is now merged."
  must_read:
    - docs/plans/20260913T195450Z-row5-s2-self-announcing-supersession.md
    - docs/reviews/rule-projection/20260913T195450Z-row5-s2-self-announcing-supersession.yaml
  do_not_touch: []
---

# Gate 1 requested: Row 5 S2 — self-announcing supersession

Following up on "proceed independently" (survey while Codex is dormant):
qbka Slice A + Slice B are merged, unblocking Row 5 S2 per the approved
work order's own dependency graph. Drafted the plan adding a
`supersedes`/`superseded_by` coordination-message field pair (matching
qbka's frozen vocabulary, per the work order's own instruction) plus the
reader-side display/filtering change in `agov coord pending` (drops a
record whose named successor is present on the reader's ref) and
`agov coord list --since` (annotates, does not drop — it's an audit log).

`agov plan validate` and `agov plan projection-check` both pass. No open
MR exists to collide with (confirmed zero open MRs before drafting).

This is a new plan, not covered by the earlier blanket Gate 1
pre-approval (`8h7s`), which was scoped to exactly Row 5 S4 and qbka
Slice B. Requesting a fresh Gate 1 decision.
