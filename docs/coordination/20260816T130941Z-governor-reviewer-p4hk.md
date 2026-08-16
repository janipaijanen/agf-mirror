---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260816T130941Z-governor-reviewer-p4hk
timestamp: 2026-08-16T13:09:41Z
model: human-governor
immutable: true
references-to: []
topic: contribution-auditor-cross-llm-policy
requires-ack: false
artifacts_owned: []
---

# Binding cross-LLM contribution-auditor policy

The governor-reviewer approves the following policy for the pending governed
proposal to add Codex as a contribution-auditor player:

1. Codex may act as contribution-auditor only for a contribution authored by
   another LLM.
2. Codex must not act as contribution-auditor for any contribution it authored
   or materially co-authored. A separate session does not cure that conflict.
3. When Codex authored or materially co-authored the contribution, a genuinely
   different LLM must perform the contribution-auditor review.
4. Mixed-authorship work that includes Codex is treated as Codex-authored for
   this independence rule.
5. The review evidence must identify the contribution's authoring LLM and the
   auditing LLM sufficiently to demonstrate that the required separation
   actually held. Role labels alone are not evidence of LLM independence.

This decision preserves the proposed narrow contribution-auditor scope:
contribution, plan, proposal, and guidance review. It does not authorize Codex
to perform implementation review or to stand in for the suspended
independent-implementation-reviewer.

## Activation boundary

This record fixes the required policy for the pending B2 plan; it does not by
itself activate Codex as contribution-auditor. Planner-architect is authorized
to incorporate these exact constraints into the fresh Gate 1 plan and candidate
manifest/rule changes. Codex gains contribution-auditor authority only after
that exact governed change completes required consultation, Gate 1,
implementation and independent review, Gate 2, merge, and publication on the
authoritative branch.

Until then, the current manifest remains authoritative and Codex continues to
act only under its currently active roles.
