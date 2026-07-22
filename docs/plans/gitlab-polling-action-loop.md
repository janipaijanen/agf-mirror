---
id: gitlab-polling-action-loop
status: approved
topic: gitlab-polling-action-loop
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-07-22T20:02:44Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: Instruction and learning updates only; no credentials, access
      controls, schemas, executable code, or sensitive data are introduced.
  - role: contribution-auditor
    status: consulted
    reason: The contribution-auditor reconstructed the missed-message and
      duplicate-pipeline evidence, identified the fragmented guidance, and
      bounded autonomous actions by existing role and gate authority.
---

# Plan: Git/GitLab polling as an action loop

## Trigger and authority

The governor-reviewer directly instructed the assistant on 2026-07-22 to
write the operational learning and update AGF developers' AI documentation on
what Git/GitLab monitoring covers and which actions assistants take without
waiting for another governor prompt. Decision record
`20260722T200244Z-governor-reviewer-mkhe` supplies Gate 1 authority.

## Problem

Existing guidance is split across `AGENTS.md`, a cold-start reference page,
Claude-specific rules, and historical learnings. It says polling should drive
action, but does not give one complete minimum scan covering fetched Git refs,
cross-branch coordination visibility, every open merge request and its notes,
issues, or exact-SHA pipelines and jobs. It also leaves enough ambiguity for an
assistant to treat `poll` as report-only and wait for the human to redispatch a
visible obligation.

On 2026-07-22 that ambiguity became measurable waste: a local-only pre-push
check missed two published coordination records and their MR-note bridges. A
release-blocking branch was pushed without the available cost-containment
instruction, creating a redundant branch pipeline alongside the required MR
pipeline. Six redundant jobs accumulated about 1,007.6 runner-seconds before
cancellation.

## Changes

1. Append one foundry learning containing the incident, causal boundary,
   business impact, and reusable recommendation.
2. Expand root `AGENTS.md` so the developer contract defines the complete poll
   surface, cursor/evidence expectations, autonomous action loop, escalation
   boundary, and CI-cost response.
3. Expand `docs/reference/agent-cold-start-habits.md` with the same operational
   contract in executable detail.
4. Correct `.claude/rules/always/coordination-message-rule.md` so Claude's
   developer instruction no longer permits report-only polling after actionable
   work is visible.

No adopter-facing product rule, schema, role authority, CI configuration,
runtime code, or release content changes in this task.

## Autonomous-action boundary

The documentation will authorize assistants to act without another human
prompt only when the action is already within the active role, an approved
plan or existing handoff, and the current gate. Examples include fetching and
reading state, acknowledging or claiming work, running diagnostics and tests,
responding to findings within approved scope, updating owned MR metadata,
publishing required coordination, batching and pushing an owned correction,
and cancelling a demonstrably redundant paid pipeline while preserving the
canonical evidence path.

It will preserve explicit human/governor authority for Gate 1 approval, Gate 2
approval and merge, releases/tags/deployments, scope expansion, destructive or
irreversible action, new authority, and materially ambiguous choices.

## Validation

- `agov plan validate docs/plans/gitlab-polling-action-loop.md`
- `agov coord validate` on the decision and Gate 2 handoff records
- `agov validate learnings`
- `agov validate infosec`
- `agov validate assistant-loader`
- `git diff --check`
- focused review that the three instruction surfaces agree and do not grant
  authority beyond existing gates or roles

## Risks and rollback

The main risk is over-broad autonomy language. Mitigation is an explicit
existing-authority test and a closed escalation list. The changes are additive
documentation and one append-only learning; instruction changes can be reverted
in one commit, while the learning remains historical and can be superseded by a
later record rather than rewritten after publication.
