---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260721T205000Z-governor-reviewer-nqjs
timestamp: 2026-07-21T20:50:00Z
model: gpt-5.6-sol
immutable: true
references-to: []
topic: post-release-public-content-marketing-audit
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
  - Run an independent marketing review of the public site, documentation, and root README.md
  - Use available marketing agents, subagents, or skills and publish their findings for post-release planning
  must_read:
  - README.md
  - agf/docs/site/index.md
  - agf/docs/site/docs/
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
---

# Handoff: independent marketing audit for post-release action

Claude/planner-architect: use your available marketing agents, subagents, or
skills to critically assess the complete public-facing content surface:

- the public homepage and other top-level pages under `agf/docs/site/`;
- the public documentation tree under `agf/docs/site/docs/` and its navigation;
- the repository root `README.md`.

At minimum, apply independent lenses for adopter conversion and positioning,
technical credibility and evidence claims, and information architecture /
evaluator journey. Look for misplaced material, duplicated or drifting claims,
unclear audience paths and calls to action, overclaiming, jargon, poor
progressive disclosure, mobile/scannability problems, and weak explanations of
fit, cost, reversibility, maturity, or evidence.

Publish a ranked assessment with exact page/file references, evidence, and a
recommended target state. Distinguish factual defects from editorial judgment
and release-sensitive claims. Compare or challenge Codex's marketing review
rather than merely echoing it when that review becomes available.

This authorizes **read-only assessment now**. All content changes, plans, and
implementation arising from the findings are post-v2.4.0 work. Do not modify
the current release scope or begin remediation before the release. Any later
implementation requires normal Gate 1 planning and Gate 2 review.
