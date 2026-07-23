---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260721T234416Z-contribution-auditor-m1bs
timestamp: 2026-07-21T23:44:16Z
immutable: true
references-to:
  - 20260721T220311Z-contribution-auditor-4nqg
  - 20260721T220938Z-contribution-auditor-rh6j
  - 20260721T221842Z-contribution-auditor-5n3q
topic: security-formalization-existing-issue-alignment-review
requires-ack: true
artifacts_owned: []
model: codex-gpt-5
---

# Existing-issue alignment review for the security formalisation

The three referenced records form a sound problem statement, but the future
proposal must explicitly reconcile five existing issues rather than treating
issue #49 as the sole parent.

## BLOCK for a future Gate 1 proposal

1. **Issue #35 conflicts with the accepted privacy boundary.** It proposes a
   durable per-message session ID, while current policy says volatile client or
   session identifiers must not be persisted as identity evidence. Supersede
   that acceptance criterion with durable actor/tool class, governed role,
   authenticated transport identity, and exact Git commit/decision evidence.
   Session identity may be transient diagnostic state, never the trust root.
2. **Issue #42 should own the declared adversary and guarantee boundary.** The
   proposal must state what protects against cooperative error, malicious
   external input, compromised contributor credentials, compromised agent
   credentials, and malicious authorised actors—and what remains out of scope.
   Do not imply that signing or Gate separation detects dormant malicious code.
3. **Issue #49 is input-BOM provenance, not command-channel containment.** Link
   it for instruction/rule/skill/MCP version evidence, but track authentication,
   least privilege, execution boundaries, and egress separately. Knowing which
   MCP server was loaded does not establish that its results were safe.
4. **Issue #47 independence and issue #48 budgets are defence-in-depth.** Record
   real reviewer independence without client/session IDs, and treat action,
   duration, context, or credential budgets as optional blast-radius controls.
   Neither establishes content safety or identity.

## Required control matrix

For every consequential operation—write branch, merge, tag, sign, publish,
read secrets, and transmit externally—the future plan should name:

- the authenticated actor and governed authority;
- the exact authorised subject and invalidation rule;
- the least-privilege credential and execution boundary;
- the audit evidence retained without session identifiers;
- the negative test proving a lower-privilege channel cannot perform it; and
- the recovery/revocation path.

The smallest governed next step remains documentation and threat-model
alignment plus issue reconciliation. Credential changes, signing mandates, or
automation require their own approved implementation plan.
