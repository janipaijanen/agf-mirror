---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260725T072128Z-planner-architect-3kke
timestamp: 2026-07-25T07:21:28Z
model: codex-gpt-5
immutable: true
references-to: []
topic: agov-day-one-adopter-faq-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/agov-day-one-adopter-faq.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review and decide Gate 1 for the agov day-one Adopter FAQ plan at exact plan commit ea9db82468a14322f5b7046577024d573e0fc98c"
  must_read:
    - docs/plans/agov-day-one-adopter-faq.md
    - "GitLab issue #65"
  do_not_touch:
    - release-2.4.2
    - agf/docs/site/docs/start/adopter-faq.md
---

# Day-one agov FAQ correction ready for Gate 1

Issue #65 records direct adopter feedback that the FAQ's current “Usually yes,
but not on day one” tooling recommendation is backwards for AI-assisted
evaluation and commissioning.

The plan changes one FAQ answer and reconciles only its first-week checklist.
It recommends using the supported Rust `agov` binary from the first session
when available, because manual evaluation, cold starts, and repeated governance
reads can be slow and token-intensive. It preserves the boundaries that source
files and human decisions remain authoritative and that token cost varies—no
fixed or universal savings claim is permitted.

Fact-checker review of the exact future prose is mandatory before Gate 2. The
plan does not change tooling, CI authority, telemetry, or the frozen v2.4.2
release.

Plan validation and `git diff --check` pass at exact commit
`ea9db82468a14322f5b7046577024d573e0fc98c`.

Governor-reviewer: record APPROVE or BLOCK for Gate 1. No FAQ implementation
may begin while the plan remains `pending-gate1`.
