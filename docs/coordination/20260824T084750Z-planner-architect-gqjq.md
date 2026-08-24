---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T084750Z-planner-architect-gqjq
timestamp: 2026-08-24T08:47:50Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260823T221311Z-governor-reviewer-nrpt.md
topic: pattern6-standing-delegation-reconciliation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Fresh COMPLETE independent audit of MR !1926 (issue #118 carrier) before Gate 1."
  must_read:
    - 20260823T221311Z-governor-reviewer-nrpt.md
    - 20260823T211206Z-contribution-auditor-g4rb.md
  do_not_touch: []
---

# New plan: reconcile framework.md Pattern 6 with standing-delegation model (issue #118)

Picked up GitLab issue #118 from an unplanned-backlog scan — it is the
governor-authored corrective-work carrier for two bounded documentation
risks (`CA-RMFP-C2`, `CA-RMFP-C-A1`) accepted while approving Gate 2 for
MR !1878 (the sources/radar research-maintenance fast path), with an
explicit revisit condition before any release containing that feature.
No prior plan or MR existed for this issue.

## Exact head

MR !1926 (`pattern6-standing-delegation-reconciliation`) at
`25fe9b57803a1e2e1f7b84d6cb09c98495d2517a`.

## What the plan does

Adds one new Pattern 6 subsection to `agf/framework.md` distinguishing
the existing per-instance Governor-Reviewer waiver from the shipped
standing, pre-approved delegation model (citing the sources/radar fast
path as the concrete example), plus a negative example clarifying that
using the fast path to advance a radar entry's lifecycle does not by
itself roadmap, assign, adopt, or authorize implementation of anything
— resolving `CA-RMFP-C-A1`.

Verified directly (not assumed) that every other surface the governor's
disposition named — `simplicity-rule.md`, its `.claude/rules/` mirror,
`development-workflow-rule.md`, and two `agf/docs/site/docs/operate/`
pages — already correctly describes the standing-delegation model, so
this plan's only actual edit is `agf/framework.md` itself.

Validated (`agov plan validate`: OK) and against the full local
umbrella (`agov validate`: no new findings beyond pre-existing,
unrelated warnings) before push.

Requesting a fresh COMPLETE audit before Gate 1 is sought.
