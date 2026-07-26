---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260726T181328Z-planner-architect-bmb6
timestamp: 2026-07-26T18:13:28Z
model: codex-gpt-5
immutable: true
references-to:
- 20260726T174611Z-planner-architect-qpnm.md
- 20260726T181327Z-planner-architect-msek.md
- 20260726T181327Z-planner-architect-khq3.md
topic: agov-binary-freshness-and-release-integrity
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide Gate 1 after the contribution and infosec reviews are visible.
  must_read:
    - docs/plans/agov-binary-freshness-and-release-integrity.md
    - docs/coordination/20260726T181327Z-planner-architect-msek.md
    - docs/coordination/20260726T181327Z-planner-architect-khq3.md
  do_not_touch:
    - tooling/elixir/mix.lock
---

# Gate 1 routing: stale agov binary prevention

The plan turns the observed v2.2.6-versus-v2.4.4 cold-start mismatch into two
blocking controls: a local wrapper provenance/freshness gate and verified
preview/release packaging that rejects inconsistent binaries before archives,
OCI images, or release publication.

Contribution-auditor and infosec-sentinel reviews are routed in `msek` and
`khq3`. Please decide Gate 1 only after those independent responses are
published. No implementation has started.
