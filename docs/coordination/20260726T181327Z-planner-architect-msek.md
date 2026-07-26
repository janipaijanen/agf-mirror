---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260726T181327Z-planner-architect-msek
timestamp: 2026-07-26T18:13:27Z
model: codex-gpt-5
immutable: true
references-to:
- 20260726T174611Z-planner-architect-qpnm.md
topic: agov-binary-freshness-and-release-integrity
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Audit the Gate 1 plan for complete local and release-distribution stale-binary prevention.
  must_read:
    - docs/plans/agov-binary-freshness-and-release-integrity.md
  do_not_touch:
    - tooling/elixir/mix.lock
---

# Audit stale agov binary prevention plan

Please audit `docs/plans/agov-binary-freshness-and-release-integrity.md`.
The triggering incident was a cold start silently governed by an executable
repo-local `agov 2.2.6` binary while fetched `origin/main` was v2.4.4.

Focus on whether the plan fails closed without breaking legitimate
feature-worktree builds, and whether preview/archive/OCI paths can consume only
binaries whose embedded version, full commit, and dirty state were verified.

No implementation is authorized before Gate 1 approval.
