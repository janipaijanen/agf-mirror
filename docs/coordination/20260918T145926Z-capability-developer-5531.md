---
kind: handoff
from: capability-developer
to: planner-architect
id: 20260918T145926Z-capability-developer-5531
timestamp: 2026-09-18T14:59:26Z
model: codex (gpt-6)
immutable: true
references-to: []
topic: serial-role-execution-capacity-one-claude-challenge
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - >-
      Claude: independently challenge the post-RUW proposal for serial
      governed role execution on a capacity-one multi-model runtime. Return
      CLEAR, REVISE, or REJECT with concrete findings and a collision
      disposition for existing issues and approved plans.
  must_read:
    - docs/proposals/serial-role-execution-on-capacity-constrained-runtimes.md
    - docs/plans/issue70-downstream-poll-coordinate-runbook.md
    - docs/plans/20260808T181423Z-independent-implementation-reviewer.md
    - GitLab issues #103, #47, #82, #121, #98, and #104
  do_not_touch:
    - implementation, schemas, tooling, rules, and role definitions
    - the approved scope of MR !2689's quantized drafting pilot
---

# Challenge the capacity-one serial-role proposal

The governor asked all available agentic roles to tear down a proposed use
case: one compute unit can keep only one model resident, but an adopter needs
several models sequentially and may dedicate particular models to particular
AGF roles. If the idea survives, identify portable AGF support. This work is
explicitly post-RUW.

Independent architecture, downstream-operations, contribution, and security
reviews converged on a qualified **retain**:

- AGF may define portable serial-role governance, checkpoint, provenance,
  recovery, and independence semantics.
- `agov` may later validate or report a proven portable execution envelope.
- an optional removable adapter may map ready work to runtime lifecycle hooks;
- the downstream runtime must own the atomic capacity lease, fencing, model
  loading/unloading, cache cleanup, quantization, credentials, and supply
  chain;
- model identity remains provenance and optional staffing policy, never role
  authority;
- a manual pilot must precede schema, tooling, or adapter implementation.

Please attack at least these claims:

1. Is this actually a framework-level governance use case, or can existing
   handoff/session-continuity guidance leave all of it downstream?
2. Does the proposed split accidentally turn AGF or `agov` into a runtime?
3. Are the two state planes and the provisional envelope sufficient to avoid
   confusing Git claims with an atomic resource lock?
4. Can dedicated role/model staffing remain optional without becoming an
   unauditable soft boundary?
5. Does sequential different-model review overclaim independence, especially
   under shared prompts, base weights, evidence, or operator framing?
6. Should this be a child of #103, an amendment to #103, or a distinct issue,
   given #47/#82/#121 and the already-approved quantized pilot?
7. What is the smallest reversible empirical trial, and what result would
   falsify the value proposition?

No implementation is requested. Please publish a validated review or ACK so a
cold-starting session can reconstruct the disagreement and final boundary.
