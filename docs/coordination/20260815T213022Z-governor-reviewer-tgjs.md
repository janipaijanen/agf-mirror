---
kind: decision-record
from: governor-reviewer
to: all
id: 20260815T213022Z-governor-reviewer-tgjs
timestamp: 2026-08-15T21:30:22Z
model: human
immutable: true
references-to:
  - 20260815T173111Z-governor-reviewer-kpkk.md
topic: backlog-absence-consensus-gate1-mandate
requires-ack: false
artifacts_owned: []
---

# Binding absence mandate for bounded backlog advancement

This record transcribes Jani's explicit in-session direction:

> I need to leave for quite some time. When this current work is finished, but
> waiting for gate, advance future work from backlog, until you stop to gate 1
> - when you face gate 1 you have mandate with claude to approve to in
> concensus, and proceed until gate 2. Ok?

## Decision

The governor-reviewer authorizes the coder-tester and planner-architect to
continue the current v2.6.1 corrective-release work through its already
governed sequence. When that work is waiting on a human-only gate or has
otherwise reached its authorized stopping point, they may select the next
highest-priority, already-recorded AGF-NG backlog item and advance it without
waiting for Jani to redispatch it.

For each selected backlog item, Gate 1 is conditionally approved only when all
of the following conditions are documented for the exact plan head:

1. The work is traceable to an existing GitLab Issue or published backlog
   record and does not expand that recorded scope.
2. The planner-architect (Claude) and coder-tester (Codex) independently state
   a clear consensus that the plan is ready and within the selected backlog
   scope. Plan authorship alone is not a consensus vote.
3. `agov plan validate` succeeds, the exact-head pipeline succeeds, all
   material review findings are resolved, and the plan's required
   contribution-auditor and infosec-sentinel consultation evidence is present.
   This mandate is not a standing consultation waiver.
4. A self-contained coordination record names the plan path, exact plan SHA,
   backlog source, review evidence, and the fact that these conditions were
   satisfied before implementation begins.

Once those conditions are met, implementation of that exact plan is
authorized through validation and presentation for Gate 2. The agents may
open and update implementation MRs, correct findings within the approved
scope, publish coordination, and collaborate through GitLab and the immutable
coordination channel. Each backlog item stops at Gate 2.

## Boundaries

This is a human-originated conditional Gate 1 decision. It does not make any
AI a governor-reviewer, permit an agent to approve its own plan, or allow
authority to be inferred from a merge, pipeline, model identity, or platform
credential.

It does not authorize Gate 2 approval or merge, direct pushes to protected
branches, tags, releases, package or OCI publication, deployment, issue
closure, destructive or irreversible actions, secrets/security exceptions,
role-status changes, scope expansion, or materially conflicting work. Those
remain human decisions. OpenClaw-governor issue #104 remains explicitly parked
and outside this mandate unless Jani separately promotes it.

If consensus is absent, evidence conflicts, a required consultation cannot be
obtained, or the proposed work crosses any boundary above, work stops at the
relevant gate for a binding human decision.
