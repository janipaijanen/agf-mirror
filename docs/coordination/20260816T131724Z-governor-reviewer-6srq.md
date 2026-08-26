---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260816T131724Z-governor-reviewer-6srq
timestamp: 2026-08-16T13:17:24Z
model: human-governor
immutable: true
references-to:
  - 20260816T130941Z-governor-reviewer-p4hk
topic: contribution-auditor-independence-fallback
requires-ack: false
artifacts_owned: []
---

# Superseding contribution-auditor independence policy

This decision supersedes `p4hk` where that record requires a different LLM
without an availability fallback. The binding policy for the pending B2 plan
is:

1. No AI agent or LLM may audit a contribution within the session that
   authored or materially co-authored it.
2. The preferred auditor uses a genuinely different underlying LLM or model
   family. Codex, Claude, Cursor, Windsurf, and other current or future agent
   surfaces are non-exhaustive examples, not a normative allowlist; product
   names alone do not prove that different underlying models were used.
3. When no different LLM is available, a fresh isolated session may perform
   the audit. The audit record must disclose that the fallback used the same
   LLM or model family and identify the authoring and auditing sessions
   sufficiently to demonstrate session separation.
4. A role-label change inside the authoring session is not separation. The
   fallback requires a genuinely fresh context, without the authoring
   session's hidden scratch state or commitments.
5. Material or high-risk findings must be escalated to governor-reviewer. The
   governor may require another LLM, human review, or additional evidence
   before accepting the audit for a governed gate.

The audit may inspect the contribution, its cited evidence, and published
coordination. Isolation does not mean withholding the review target; it means
the auditor does not continue the authoring session as the supposed reviewer.

## Scope and activation boundary

This remains a narrow contribution-auditor policy for contribution, plan,
proposal, and guidance review. It does not authorize implementation review or
commission the suspended independent-implementation-reviewer.

Planner-architect is authorized to incorporate this superseding policy into
the fresh B2 Gate 1 plan and candidate manifest/rule changes. It does not
activate a new player by itself. Any added player gains authority only after
the exact governed change completes required consultation, Gate 1,
implementation and independent review, Gate 2, merge, and publication on the
authoritative branch.
