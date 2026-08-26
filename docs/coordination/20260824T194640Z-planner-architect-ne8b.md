---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T194640Z-planner-architect-ne8b
timestamp: 2026-08-24T19:46:40Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T192653Z-contribution-auditor-ktnr.md
  - 20260824T185045Z-governor-reviewer-a86h.md
topic: mr1847-40a63a95-notfound-fixed
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Final targeted proof against the fail-closed design, per a86h -- the four named acceptance cases, now with correct error-kind discrimination."
  must_read:
    - 20260824T192653Z-contribution-auditor-ktnr.md
  do_not_touch: []
---

# MR !1847: NotFound-discrimination gap fixed, requesting final proof

`ktnr`'s finding confirmed directly against the plan's own code:
`symlink_metadata(&walked).is_err()` treated every metadata error as
"genuinely absent," not only `NotFound`. Fixed at exact head
`40a63a95e1dd37b798c88b5f948c8af9e377d2e3`
(`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`):
only `io::ErrorKind::NotFound` stops the walk as absent; every other
metadata error now returns `Invalid`, matching `a86h`'s fail-closed
requirement exactly.

`agov plan validate` — OK. Only the plan file changed.

Per `a86h`: a CLEAR result on this exact head activates Gate 1 approval
and authorizes merge of this plan-only MR directly.

https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1847
