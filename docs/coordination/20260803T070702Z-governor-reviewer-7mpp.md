---
kind: handoff
from: governor-reviewer
to: infosec-sentinel
id: 20260803T070702Z-governor-reviewer-7mpp
timestamp: 2026-08-03T07:07:02Z
immutable: true
references-to:
  - 20260803T065103Z-governor-reviewer-se21.md
  - 20260802T233331Z-contribution-auditor-rbdm.md
topic: infosec-boundary-check-mr-967-ai2-ai5
requires-ack: true
artifacts_owned: []
model: codex-gpt-5
handoff:
  next_agent: infosec-sentinel
  status: ready
  task:
    - "Run the configured infosec validation authorized by current AGF rules against MR !967 exact head a78dd266b7c68c73eee1aad4fac48978c5d702e0, focusing mechanically available checks on AI-2 and AI-5 and their referenced records."
    - "Report exactly what the configured checks can and cannot establish. Do not treat a green credential/cross-record-leak scan as semantic validation of CRA vulnerability policy, GDPR erasure compatibility, legal applicability, or security effectiveness."
    - "Route any semantic/security-design ambiguity to governor-reviewer; the tool supplies evidence and may block only where a named rule grants it authority."
  must_read:
    - docs/plans/ai-regulation-requirements-gap-analysis.md
    - agf/rules/always/tool-authority-boundary-rule.md
  do_not_touch:
    - .claude/settings.local.json
---

# Infosec-sentinel dispatch with explicit authority boundary

This handoff satisfies the plan's request for an infosec lane without asking
the CI sentinel to impersonate a legal, privacy, or human security reviewer.
The governor will decide how any unsupported semantic remainder is dispositioned
before Gate 1.
