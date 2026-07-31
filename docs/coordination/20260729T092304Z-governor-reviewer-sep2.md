---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260729T092304Z-governor-reviewer-sep2
timestamp: 2026-07-29T09:23:04Z
model: human
immutable: true
references-to:
  - 20260729T092303Z-governor-reviewer-bn8g
  - 20260729T091518Z-contribution-auditor-mq4r
topic: v246-recovery-child-plan
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - On MR !755's existing branch, change only the approved SoW plan status from pending-gate1 to approved and refresh the live MR description for the new exact head.
    - On a separate fresh-main planning branch, draft the v2.4.6 recovery child plan as the first release-train deliverable.
    - Center the child plan on issue #81's clean-archive failure; include only verified patch-compatible #67/#78 corrections whose adopter impact and reproduction are evidenced.
    - Define tar and zip clean-extraction acceptance, execution of every shipped test without checkout-only fixtures, patch-SemVer boundaries, exact-artifact review, and explicit stop conditions.
    - Argue the scope with Codex/contribution-auditor and route factual claims to fact-checker before requesting the child's Gate 1 decision.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/755
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/81
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/67
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/78
    - docs/coordination/20260729T092303Z-governor-reviewer-bn8g.md
  do_not_touch:
    - agf/VERSION
    - agf/CHANGELOG.md
    - any release branch or tag
    - issue-board or coordination-archive implementation
    - MR !772 rule formalization
    - product implementation before the child plan receives Gate 1
---

# First child-planning handoff: v2.4.6 recovery

Proceed with the release train in external-value order. The immediate planning
question is whether the smallest patch-safe v2.4.6 package repairs the released
archive contract honestly and reproducibly. Internal portfolio hygiene may run
later or in parallel; it is not a reason to delay the issue #81 correction.

Claude/planner-architect owns the first draft and must respond to Codex's
challenge with AGREE, PARTIAL, or DISSENT rather than silently absorbing it.
Codex/contribution-auditor owns adversarial scope review. The fact-checker owns
verification of reproduced defects and claims. The governor-reviewer retains
the child Gate 1 decision.

No implementation or release action begins from this handoff alone.
