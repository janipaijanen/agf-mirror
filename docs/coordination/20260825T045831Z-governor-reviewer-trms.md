---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260825T045831Z-governor-reviewer-trms
timestamp: 2026-08-25T04:58:31Z
model: human-via-codex
immutable: true
references-to:
  - 20260825T045829Z-governor-reviewer-mtms
  - 20260825T044258Z-coder-tester-5djr
topic: necessary-scope-expansion-minimalism-rule
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Prepare a narrow Gate 1 amendment or successor plan that makes necessary scope-estimate correction explicit in the simplicity/minimum-sufficient-value rule.
    - Define proportional post-correction learning that improves future impact assessment without blocking the necessary correction.
  must_read:
    - agf/rules/workflow/simplicity-rule.md
    - agf/rules/always/uncertainty-and-scope-rule.md
    - docs/plans/20260824T205737Z-minimum-sufficient-value-rule.md
    - docs/coordination/20260825T045829Z-governor-reviewer-mtms.md
  do_not_touch:
    - Current review-convergence implementation branch
    - .claude/settings.local.json
---

# Plan a bounded necessary-expansion rule

Improve the minimalism rule around this principle:

> When an approved outcome proves impossible to complete correctly within the
> estimated file list, the agent states its intent and evidence, communicates
> the smallest necessary expansion, obtains any authority the governing gate
> still reserves, and makes that correction without waiting for the
> retrospective. After the correction is stable and validated, the agent
> records why the impact assessment missed the dependency and how future
> assessments should improve.

This is the useful part of intent-based leadership: distribute thinking and
initiative while preserving alignment. David Marquet's description distinguishes
intent from permission by making action the default absent a veto, but AGF must
retain explicit stops where authority, risk, or the approved outcome changes:
https://davidmarquet.com/

The planned rule should distinguish all of the following:

1. **Necessary correction:** an existing invariant, mandatory test, or required
   integration surface proves that the approved outcome cannot be complete
   without the additional path. Permit only the minimum evidenced change after
   its intent and scope are visible and the applicable authority is satisfied.
2. **Optional expansion:** cleanup, redesign, convenience refactoring, or added
   capability remains out of scope and follows normal planning.
3. **Material expansion:** changes to outcome, interfaces, architecture,
   dependencies, authority, security/privacy, cost, release, or other governed
   commitments stop for renewed Gate 1 authority.
4. **Learning after stabilization:** capture the missed dependency, why the
   original impact method missed it, where it was detected, and one concrete
   assessment or tooling improvement. Keep the record proportionate; promote a
   reusable or recurring miss into the governed learning channel.

The plan should state that exact file lists are impact estimates and control
boundaries, not incentives to ship an internally inconsistent change. It should
also retain a default-deny boundary: an agent may not label a desirable change
"necessary" without objective evidence and may not use this rule to approve its
own material scope expansion.

Do not silently append this policy to the already approved minimum-sufficient-
value implementation. Amend that plan with renewed approval or create a small
successor plan so the policy change remains reviewable.
