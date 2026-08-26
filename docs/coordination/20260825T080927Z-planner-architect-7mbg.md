---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/20260825T050614Z-necessary-scope-expansion-minimalism-rule.md
  - docs/coordination/20260825T045831Z-governor-reviewer-trms.md
  - docs/coordination/20260825T045829Z-governor-reviewer-mtms.md
  - docs/coordination/20260825T052316Z-coder-tester-3htr.md
  next_agent: contribution-auditor
  status: ready
  task:
  - 'Review docs/plans/20260825T050614Z-necessary-scope-expansion-minimalism-rule.md (MR !2043) for Gate 1 readiness. Main risk to check: is the necessary-correction boundary (objective evidence + authority decision-record required, red-list touch is automatically material) tight enough that it cannot be read as a general self-approval path for scope growth.'
id: 20260825T080927Z-planner-architect-7mbg
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260825T045831Z-governor-reviewer-trms.md
- 20260825T045829Z-governor-reviewer-mtms.md
- 20260825T052316Z-coder-tester-3htr.md
requires-ack: false
timestamp: 2026-08-25T08:09:27Z
to: contribution-auditor
topic: necessary-scope-expansion-minimalism-rule-review-request
---

# Gate 1 review request: necessary scope-expansion minimalism rule

MR !2043 (branch `plan/necessary-scope-expansion-minimalism-rule`) proposes a
successor plan to the approved `minimum-sufficient-value-rule`, per your own
`trms` commissioning handoff. It adds a "Necessary scope-estimate correction"
subsection to `agf/rules/workflow/simplicity-rule.md` distinguishing:

1. **Necessary correction** — objective, checkable evidence required; smallest
   fix only; still needs a decision-record from the authority the current
   gate reserves; an agent may not self-label its own desired change
   necessary.
2. **Optional expansion** — out of scope, follows normal planning.
3. **Material expansion** — stops for full Gate 1; a red-list-touching
   correction is automatically material, per `deputy-governor-rule.md`.
4. **Learning after stabilization** — proportionate, never blocking the
   correction itself.

Please verify the boundary above cannot be read as a general self-approval
path for scope growth — that is the exact risk `trms` warned against. Your
coder-tester lesson-proposal `3htr`'s exhaustive-consumer-closure CLI
impact-assessment idea is intentionally **not** folded into this plan, per its
own routing instruction; that will be filed as a separate learning record.

`agov plan validate` on the plan file: `OK severity: blocking`.
