---
kind: handoff
from: planner-architect
to: planner-architect
id: 20260905T205921Z-planner-architect-pm6m
timestamp: 2026-09-05T20:59:21Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260905T193056Z-planner-architect-sgg6
  - 20260905T194621Z-planner-architect-htar
  - 20260905T200607Z-coder-tester-hqps
topic: universal-agent-autonomous-polling-design-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review and materially challenge the proposed universal polling rule before its Gate 1 plan is finalized.
    - Confirm collision-safe ownership, wake-up semantics, stop conditions, and the complete shipped projection set.
    - Return a durable review or revised design addressed to this Codex planner session.
  must_read:
    - AGENTS.md
    - agf/docs/site/docs/operate/poll-and-coordinate.md
    - docs/reference/agent-cold-start-habits.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2369
  do_not_touch:
    - .claude/settings.local.json
---

# Claude collaboration request: universal autonomous polling

This handoff is specifically for the Claude planner-architect session that
owns the current v2.9 release-chain coordination (`sgg6`/`htar`), not a
self-ack request to the authoring Codex session. The governor has made this a
high-priority requirement: every active AGF development agent must poll the
complete shared work surface, continue authorized work autonomously, and stop
only at genuine gates or an equivalent authority boundary or concrete external
blocker.

## Proposed normative invariant

Create one canonical rule under `agf/rules/` that applies to every active AGF
development agent/session, regardless of implementation role. Each agent must:

1. run a complete remote-aware poll before selecting work, at each configured
   wake, and whenever resumed after inactivity;
2. scan fetched coordination, all open forge MRs and issues (not only its own
   or a subjective "relevant" subset), their notes/discussions, exact heads,
   approvals, mergeability, and exact-head pipelines/jobs, plus releases,
   tags, runners, branches, or other exact external state it started or awaits;
3. select and immediately advance the highest-priority visible obligation that
   is within its role, approved plan, current gate, and explicit ownership;
4. monitor work it starts or depends on until it becomes reviewable, completes,
   or reaches a concrete blocker, without asking the governor to redispatch or
   say "proceed"; and
5. leave collision-safe claims, exact cursors, and durable coordination so
   universal polling does not become universal duplicate execution.

Polling must not enlarge authority. Binding Gate 1/Gate 2, merge, tag, release,
scope expansion, destructive action, new authority, secrets/security ambiguity,
or materially conflicting instructions remain stop points when their named
authority is absent. A concrete external blocker is also a legitimate pause,
but only after safe in-scope diagnostics and alternatives are exhausted and
the blocker has an owner and evidence. Ordinary uncertainty, waiting CI, or
the absence of another chat prompt is not a stop condition.

An inactive session cannot poll. A session with a real scheduler uses it; one
without a wake mechanism polls on activation/resume and states that limitation
instead of claiming a shell timer will wake it.

## Proposed projections and dependency

The plan should cover the new rule, `agf/rules/README.md`, adopter
`agf/templates/AGENTS.md`, the released Pages runbook and
`poll-and-coordinate.md`, foundry `AGENTS.md`, internal cold-start guidance,
and the starter guide in MR !2369. Because !2369 owns that new file and is
already Gate-2-ready, implementation should depend on !2369 merging and then
amend it from fresh `main`; it must not edit or commandeer !2369's branch.
A six-surface projection manifest and tabletop/negative examples should prevent
semantic drift. The initial proposal is reviewer-enforced normative guidance,
not a new CI gate; any mechanical enforcement proposal must name its explicit
blocking authority and negative tests.

Please return specific objections, missing surfaces, and revised wording. In
particular, test whether "every agent polls all items" can coexist with
role-scoped claiming and whether the rule distinguishes a genuine authority
stop from discretionary idling. The v2.9 release lane remains monitored while
this Gate 1 design is reviewed.
