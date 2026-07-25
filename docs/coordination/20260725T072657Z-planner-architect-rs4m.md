---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260725T072657Z-planner-architect-rs4m
timestamp: 2026-07-25T07:26:57Z
model: codex-gpt-5
immutable: true
references-to: []
topic: split-person-gates-adopter-faq-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/split-person-gates-adopter-faq.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review and decide Gate 1 for the split-person gates FAQ plan at exact plan commit f6644a387aa0dc571b0975ee7e24fd271e3bb0c4"
  must_read:
    - docs/plans/split-person-gates-adopter-faq.md
    - "GitLab issue #66"
  do_not_touch:
    - release-2.4.2
    - agf/docs/site/docs/start/adopter-faq.md
---

# Split-person gates FAQ clarification ready for Gate 1

Issue #66 records the adopter question: may Gate 1 and Gate 2 be approved by
different people?

The planned answer is yes when authority is explicit and sequential. Person A
may own Gate 1 and Person B Gate 2, but the Gate 2 reviewer must read the exact
approved plan and conditions, scope changes return to Gate 1, and one named
primary governor/project authority resolves disagreement. Neither approver may
approve implementation they personally performed as coder-tester.

The clarification preserves the current evidence boundary: AGF v2's validated
minimum is one governor at a time. It does not create simultaneous governors,
quorum approval, a new role, schema/tooling enforcement, or a claim that two
people are universally better. Exact future prose requires fact-checker review
before Gate 2.

Plan validation and `git diff --check` pass at exact commit
`f6644a387aa0dc571b0975ee7e24fd271e3bb0c4`.

Governor-reviewer: record APPROVE or BLOCK for Gate 1. No FAQ implementation
may begin while the plan remains `pending-gate1`.
