---
kind: decision-record
from: governor-reviewer
to: all
id: 20260721T165544Z-governor-reviewer-3h53
timestamp: 2026-07-21T16:55:44Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260720T193644Z-coder-tester-kh7j.md
  - 20260720T191940Z-coder-tester-e366.md
  - 20260720T160702Z-coder-tester-h378.md
  - 20260720T151740Z-coder-tester-j1d2.md
  - 20260720T144632Z-coder-tester-rhnh.md
  - 20260720T115855Z-coder-tester-gnss.md
  - 20260720T112857Z-coder-tester-34k5.md
  - 20260720T105755Z-coder-tester-tnp3.md
  - 20260626T062329Z-coder-tester-bhj2.md
  - 20260626T055951Z-coder-tester-76jm.md
  - 20260625T192603Z-coder-tester-57ss.md
  - 01KVK7J9N2AW8DJZQAG2-planner-architect.md
  - 01KVK7TFN1ADA2WBZDWKFRPBXR-planner-architect.md
  - 01KVK7DYM21GGQ7ZZ5RR-planner-architect.md
  - 01KVK7TFMY6DQNPRH7SAS5BSHP-planner-architect.md
topic: stale-handoff-reconciliation-before-v2-4-0
requires-ack: false
artifacts_owned: []
---

# Decision: reconcile stale handoffs before v2.4.0 planning continues

The governor requested coordination cleanup before further release planning.
The handoff tracker still reported obligations whose requested outcome had
already merged, shipped, or become obsolete. This record closes those stale
edges without editing or deleting immutable history.

## Shipped or merged outcomes

- The nutshell flyer implementation merged in !250, was embedded on the public
  site in !254, and shipped in the v2.3 line. The old Gate 2 handoff is closed.
- The agent-improvement proposal rule merged in !238; duplicate MR !240 was
  closed. The old Gate 2 handoff is closed.
- Documentation audience work merged through !228 (install/layout split), !229
  (taxonomy/map), !230 (per-step markers), and !231 (AI-operator relabelling).
  The two reported Gate 2 handoffs are closed.
- Coordination CLI write primitives merged in !220, `agov-mcp` in !221, and the
  OCI release path in !222; all were subsequently included in the v2.3 release
  line. Their implementation handoffs are closed. Later path-contract,
  telemetry, provenance, and release plans are distinct obligations.
- External-review documentation hardening merged in !205. The framework map is
  present on main from commit `9974bdb`. Their old handoffs are closed.
- The v2.2.6 release MR !206 merged but was not published as a tag; its contents
  and unpublished disposition are explicitly incorporated into the v2.3.0
  changelog/release trail. The v2.2.6 handoff is closed and must not trigger a
  retroactive tag.
- Readable coordination IDs were implemented through !37, later made the
  default in !155, and their stale Gate 1 routing was already addressed by
  prior closure records. Both duplicate implementation/routing handoffs are
  closed here for tracker normalization.

## Superseded review request

The TODO/Sol High synthesis critique asked whether release-truth enforcement
should block v2.3.0 and whether a then-current assessment needed review before
commitment. The synthesis merged in !252, v2.3.0 and its v2.3.1 recovery are
complete, and the release-truth failures produced newer plans and learning
records. That time-bound handoff is superseded. Any reuse of the synthesis must
start with a fresh review against current main rather than pretending the old
pre-v2.3 priority question remains actionable.

## Boundary

This is audit-trail reconciliation, not deletion, retroactive approval, or a
claim that every historical recommendation was implemented. It does not close
newer explicit follow-ups such as provenance, MCP telemetry, asset naming,
figure redesign, or the v2.4.0 release plan. It does not decide the separate
GitLab closure of MR !117; the current review of that MR is recorded in
`20260721T165543Z-planner-architect-gtbg`.
