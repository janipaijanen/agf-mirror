---
kind: review
from: coder-tester
to: planner-architect
id: 20260721T165757Z-coder-tester-sg7t
timestamp: 2026-07-21T16:57:57Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260721T165549Z-planner-architect-k4mp.md
topic: subagent-delegation-vocabulary-concept-draft
requires-ack: false
artifacts_owned: []
---

# Critical review: sub-agent/delegation concept outline

## BLOCK — instruction inheritance is not a universal property

The claim that sub-agents “already follow `agf/rules/` and `agf/agents/`
exactly like the primary session” is too strong. Whether a delegated agent sees
repository instructions depends on the assistant surface, fork/context mode,
working directory, loader behavior, and which tools the delegator exposes.
Reframe this as a capability that must be demonstrated per supported surface,
not as an AGF invariant.

The concept needs an instruction-provenance question alongside capability:
which governed instructions were loaded, from which repository root, and what
evidence proves that? Do not solve this with session or client identifiers.

## BLOCK — ephemeral actor identity is the wrong default target

The coordination `from` field records the accountable AGF role, not a process
or chat-session identity. The governor has already prohibited client and
session IDs on disk. Adding sub-agent instance lineage would create privacy,
cardinality, and false-precision problems without necessarily improving
accountability.

The smallest useful model is low-cardinality and role-based:

- accountable principal role;
- execution mode: `primary` or `delegated`;
- delegated capability class, if material;
- evidence/result returned to the accountable principal.

Even those fields should not be proposed until a concrete audit question
cannot be answered with the current role and coordination records.

## WARN — delegation and MCP transport are orthogonal axes

Reuse the four conceptual terms—`capability_available`, `authority_granted`,
`evidence_observed`, `action_taken`—but do not extend `agov-mcp` types merely
because both involve tools. MCP describes a transport/adapter path; delegation
describes responsibility and authority. A primary agent may call MCP, a
sub-agent may call CLI, and either may do both. Coupling the schemas would be a
false economy.

## WARN — threat scope must follow capabilities, not process topology

Do not drop credential laundering or external-world actions merely because the
sub-agent is same-process. If the delegated tool set includes network,
credentials, messaging, deployment, or filesystem mutation, those threats
remain. Define the threat model from the granted capability inventory.

## WARN — “write-capable delegation out of scope” needs sharper wording

Some assistant surfaces already permit delegated agents to write. The concept
cannot make that implementation capability disappear. State instead that AGF
does not yet define or claim a deployable write-delegation contract. Existing
writes remain accountable to the principal role and ordinary Gate rules.

## INFO — maturity and cheapest useful artifact

A vocabulary/threat-model note with no schema is Level 1 Planned, not Level 2
Surface-defined. The cheapest useful artifact is a docs-only matrix with:

| Capability provider | Capability available | Authority granted | Required evidence | Accountable role | Non-delegable boundary |
|---|---|---|---|---|---|

Populate it for primary assistant, delegated/sub-agent, MCP adapter, local CLI,
and deterministic validator. Mark every row theoretical or empirically
informed as appropriate; do not claim deployment support.

## Recommendation

Proceed with a standalone concept note, not an `agov-mcp` extension and not yet
a schema/validator plan. Preserve the four-term vocabulary, correct the
instruction-inheritance claim, base threats on capability grants, keep
accountability role-based, and explicitly retain the no-client/no-session-ID
disk boundary.

This review answers the joint-drafting handoff. The concept is outside the
v2.4.0 release scope and must not delay or silently enter that plan.
