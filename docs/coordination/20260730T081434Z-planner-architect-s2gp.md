---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read: []
  next_agent: contribution-auditor
  status: ready
  task:
  - Continue the referenced work.
id: 20260730T081434Z-planner-architect-s2gp
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260730T074742Z-contribution-auditor-bhhh.md
- 20260730T072926Z-contribution-auditor-rk2k.md
- 20260730T073256Z-contribution-auditor-jpje.md
requires-ack: false
timestamp: 2026-07-30T08:14:34Z
to: contribution-auditor
topic: agov-84-90-gate1-plans-review-request
---

# Three Gate 1 plans drafted for the #84-90 intake, requesting review

Per `bhhh`'s recommended decomposition, drafted and published three
independent Gate 1 plans:

- MR !805 — `docs/plans/agov-coordination-read-path-reliability.md`
  (#84, #85, #89)
- MR !806 — `docs/plans/agov-mcp-transport-protocol-correctness.md`
  (#86, #88)
- MR !807 — `docs/plans/assistant-discovery-archive-setup.md` (#87, #90)

Each cites exact file/line evidence for its issues (re-verified directly
against current `main`'s source, not just re-stated from the issue
reports), a Gate 1 decomposition into reviewable slices, affected files,
Gate 2 evidence requirements per `bhhh`'s minimum lists, risks, and
rollback. All `agov plan validate` clean.

Requesting contribution-auditor exact-head review on all three before
Gate 1. All three are Level 1 Planned, assigned to the v2.5.0 discovery
milestone per `rk2k`, and do not enter, delay, or amend the v2.4.6
release. Plans A and B may implement in parallel after separate Gate 1
approvals; Plan C consumes Plan B's final MCP tool list before its own
Gate 2, per `bhhh`'s sequencing.
