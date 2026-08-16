---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260816T113957Z-coder-tester-beth
timestamp: 2026-08-16T11:39:57Z
model: codex-gpt-5
immutable: true
references-to: []
topic: external-handoff-and-reconciliation-roadmap-review
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the AIHero handoff prior art and the bounded portable session-transit candidate added to the operator-interface roadmap.
    - Review AetherGrid's renewable-lease and reconciliation mechanics as a radar question for AGF claim ownership without runtime lock-in.
    - Decide during roadmap planning whether to propose a Gate 1 implementation plan, retain it as documentation guidance, or reject it with rationale.
  must_read:
    - docs/sources.md
    - docs/TODO.md
    - https://www.aihero.dev/skills-handoff
    - https://github.com/mattpocock/skills/blob/main/skills/productivity/handoff/SKILL.md
    - docs/radar.md
    - https://github.com/wycliffRotich-dev/aethergrid
  do_not_touch:
    - Existing authoritative coordination handoff semantics without a separately approved Gate 1 plan.
    - Current v2.6.1 release-publication work.
    - OpenClaw issue 104.
---

# Review portable handoff and reconciliation prior art for the AGF roadmap

The governor asked that AIHero's `/handoff` pattern be recorded and considered
by Claude. The source and its upstream skill are now classified in
`docs/sources.md`; the existing first-class skills and commands TODO now carries
a bounded assessment item rather than an implementation commitment.

The promising part is cross-harness portability: a small transit document can
move live session context from Claude to Codex, to another repository or
colleague, or into a side-task fork while referencing durable artifacts rather
than duplicating them. The important AGF boundary is that this document cannot
become a second authority channel. A task, review, approval, or other downstream
obligation still requires validated and published AGF coordination.

Planner-architect should compare the candidate with `compact-agf`, `agov
session resume`, and coordination `kind: handoff`. If implementation is
warranted, propose it through a separate exact-scope Gate 1 plan with the normal
consultations. This record does not activate implementation.

AetherGrid is also recorded as prior art and as a radar question, not as an AGF
competitor or implementation dependency. Its renewable work leases, heartbeats,
explicit execution-start transition, immutable lifecycle events, and failure
reconciliation offer a useful stress test for AGF claims. Planner-architect
should assess whether any invariant belongs in repository-native claim
semantics without importing a continuously available scheduler or allowing
lease ownership to imply gate authority.
