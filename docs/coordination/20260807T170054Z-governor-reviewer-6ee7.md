---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260807T170054Z-governor-reviewer-6ee7
timestamp: 2026-08-07T17:00:54Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260807T170036Z-governor-reviewer-qbhq.md
  - 20260806T173435Z-planner-architect-jprr.md
topic: get-started-commissioning-ux-implementation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement the approved Get Started commissioning UX plan in its prescribed ordered, independently reviewable slices.
    - Begin with the first authorized slice on a new feature branch and linked worktree based on current main.
    - Produce plan-prescribed deterministic checks and Class B AI-execution evidence, then publish an exact-head Gate 2 handoff.
  must_read:
    - docs/plans/20260806T122121Z-get-started-commissioning-ux-v1.md
    - docs/coordination/20260807T170036Z-governor-reviewer-qbhq.md
    - docs/coordination/20260806T162915Z-governor-reviewer-bqdk.md
  do_not_touch:
    - .claude/settings.local.json
    - deferred slice-3 navigation restructure beyond the single additive commissioning-page entry authorized by the plan
---

# Implementation handoff: Get Started commissioning UX

Gate 1 is approved in `qbhq`. Claude has returned at full capacity and should
resume the `coder-tester` role for implementation.

Execute only the approved plan scope. Preserve the plan's canonical-prompt
ownership, new-versus-existing-install safety semantics, checksum and
provenance requirements, and separation between deterministic Class A checks
and one-time Class B assistant-execution evidence. Route the resulting exact
head through independent review and Gate 2; do not infer Gate 2 approval from
the governor's plural wording before an implementation artifact exists.
