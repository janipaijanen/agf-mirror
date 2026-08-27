---
kind: decision-record
from: governor-reviewer
to: all
id: 20260824T195700Z-governor-reviewer-redm
timestamp: 2026-08-24T19:57:00Z
model: human-via-codex
immutable: true
references-to:
  - 20260824T192313Z-governor-reviewer-ejmm.md
topic: codex-ejmm-planner-authority
requires-ack: false
artifacts_owned: []
---

# Codex is authorized to take the `ejmm` planning task

The governor approves the required wording: Codex is temporarily authorized
to act as `planner-architect` for the single bounded task handed off in
`20260824T192313Z-governor-reviewer-ejmm`. Codex must take it as the next
planning obligation after MR !1847's currently executing atomic publication
and exact-head delivery path is safely completed or actively waiting on CI.

This decision overrides the player assignment only for producing and routing
that Gate 1 plan. It does not change the standing role manifest, authorize
implementation, approve Gate 1, approve Gate 2, or authorize merge, release,
tag, or deployment. The Codex session that authors the plan must not provide
its contribution-auditor consultation; that consultation must come from a
separate eligible session.

This explicit assignment is the dispatch mechanism because coordination
records currently have no schema-level priority field and `agov handoff next`
does not rank free-text priority declarations mechanically.
