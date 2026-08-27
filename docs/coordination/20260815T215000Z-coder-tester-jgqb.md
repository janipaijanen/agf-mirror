---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260815T215000Z-coder-tester-jgqb
timestamp: 2026-08-15T21:50:00Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260815T213022Z-governor-reviewer-tgjs.md
  - 20260813T044747Z-governor-reviewer-8hgm.md
  - 20260809T074441Z-contribution-auditor-pghm.md
topic: v270-mcp-assistant-integration-gate1-planning
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - >-
      Resume the governor-selected v2.7.0 MCP and assistant-integration lane
      for issues #86, #87, #88, and #90 while the v2.6.1 corrective plan is
      waiting on its human-only infosec disposition.
    - >-
      Reconcile and revise existing plan MRs !806 and !807 against the fresh
      exact-head BLOCK findings in pghm and current main; do not start a
      duplicate plan or implementation.
    - >-
      Route revised exact heads for required contribution-auditor, infosec,
      build-engineer, and coder-tester review under tgjs's conditional Gate 1
      mechanism.
  must_read:
    - >-
      MR !806 head 3e86215a50faa7bbf08255864fc9abf6ffa54d26,
      docs/plans/agov-mcp-transport-protocol-correctness.md
    - >-
      MR !807 head 63222f267e9bf187c5af63387b1cc822ff4130a1,
      docs/plans/assistant-discovery-archive-setup.md
    - docs/coordination/20260809T074441Z-contribution-auditor-pghm.md
    - docs/coordination/20260813T044747Z-governor-reviewer-8hgm.md
    - docs/coordination/20260815T213022Z-governor-reviewer-tgjs.md
  do_not_touch:
    - .claude/settings.local.json
    - issue #104 OpenClaw-governor scope
---

# Resume the approved next backlog lane: v2.7.0 MCP and assistant integration

The v2.6.1 corrective plan's round-four mechanical design is now clear, and
its non-author contribution audit is complete. It is waiting on a human-only
infosec consultation/waiver disposition, recorded in coder review `hegm`.
That satisfies `tgjs`'s condition for advancing the next already-recorded
backlog lane without pretending the blocked work is complete.

Governor decision `8hgm` selected issues **#86, #87, #88, and #90** as the
v2.7.0 MCP/assistant-integration lane. Existing plan MRs already cover the
lane:

- MR !806: #86/#88, `docs/plans/agov-mcp-transport-protocol-correctness.md`,
  exact current head `3e86215a50faa7bbf08255864fc9abf6ffa54d26`;
- MR !807: #87/#90, `docs/plans/assistant-discovery-archive-setup.md`, exact
  current head `63222f267e9bf187c5af63387b1cc822ff4130a1`.

Both plans once had older CLEAR evidence, but fresh current-context review
`pghm` superseded that readiness and BLOCKED the exact heads on 2026-08-09.
Do not infer approval from the older reviews or create replacement plans.
Revise the existing plans against every open `pghm` finding, rebase their
facts and tool inventory against current `main`, and preserve their stated
dependency: the assistant-discovery/archive plan consumes the transport
plan's final supported MCP tool list before its own Gate 2.

## Gate boundary

This handoff authorizes planning and review only. `tgjs` conditionally
activates Gate 1 only after Claude/planner-architect and Codex/coder-tester
independently reach exact-head CLEAR consensus, Rust plan validation and CI
pass, all material findings are resolved, and every required advisory
consultation is evidenced. Publish the self-contained activation record
before implementation. Implementation may then proceed only through Gate 2
presentation; no merge, tag, release, deployment, issue closure, scope
expansion, consultation waiver, or OpenClaw #104 work is authorized.
