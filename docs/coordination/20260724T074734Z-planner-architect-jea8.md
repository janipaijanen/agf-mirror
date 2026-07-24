---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260724T074734Z-planner-architect-jea8
timestamp: 2026-07-24T07:47:34Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260724T074630Z-governor-reviewer-1ege.md
  - 20260724T074333Z-planner-architect-2r6q.md
topic: review-finding-contract-v1-implementation
requires-ack: true
artifacts_owned:
  - agf/docs/site/reference/review-findings.md
  - agf/docs/site/docs/operate/review-and-release.md
  - agf/docs/site/docs/concepts/coordination-protocol.md
  - agf/docs/build/mkdocs.yml
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/coord_brief.rs
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement exactly the approved advisory review-finding convention, documentation links/nav, and non-enforcing agov review skeleton.
    - Add the closed-vocabulary, mixed incomplete-plus-BLOCK, bounded accepted-risk, authority-boundary, migration, and Goodhart examples/tests from the plan.
    - Run focused and full Rust validation plus strict docs build, then hand the exact implementation to contribution-auditor for Gate 2.
  must_read:
    - docs/plans/review-finding-contract-v1.md
    - docs/coordination/20260724T074630Z-governor-reviewer-1ege.md
    - docs/coordination/20260724T074302Z-contribution-auditor-gjgk.md
    - agf/rules/always/tool-authority-boundary-rule.md
  do_not_touch:
    - agf/schemas/
    - agf/rules/
    - agf/agents/
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Implementation handoff: review finding contract v1

Gate 1 approval is scoped to the plan at its audited content hash and the files
listed above. This slice is advisory. Do not add body validation, aggregation,
blocking behavior, mandatory adoption language, or an external standards wire
format.
