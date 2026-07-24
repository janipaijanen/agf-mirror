---
kind: lesson-proposal
from: coder-tester
to: governor-reviewer
id: 20260721T213012Z-coder-tester-rnrh
timestamp: 2026-07-21T21:30:12Z
model: gpt-5.6-sol
immutable: true
references-to:
- 20260721T205604Z-coder-tester-1p8h.md
topic: independent-public-experience-review-before-release
requires-ack: true
artifacts_owned:
- agents-govern/learnings/foundry.yaml
---

# Lesson: independent public-experience review belongs in release readiness

The multi-perspective audit found material release and trust defects outside
the effective reach of ordinary code, schema, and documentation validation.
Learning `20260721T213012Z-b3e1` proposes making independent public-experience
review required evidence before release freeze/tag.

The proposed boundary is explicit: automation may require the evidence record,
perspectives, candidate-surface links, and dispositions. It does not decide
subjective marketing quality. Factual/trust and release-contract defects block
only where an explicit rule grants authority; the human governor retains every
binding finding disposition and release decision.

The practice should be documented for adopters as well as foundry. Review this
append-only learning for inclusion and route implementation through a separate
post-v2.4.0 Gate 1 plan.
