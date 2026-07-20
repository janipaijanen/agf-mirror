---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260720T210500Z-coder-tester-en5m
timestamp: 2026-07-20T21:05:00Z
model: claude-sonnet-5
immutable: true
references-to: []
requires-ack: false
artifacts_owned:
  - agf/rules/workflow/deprecation-policy-rule.md
  - .claude/rules/workflow/deprecation-policy-rule.md
topic: deprecation-policy-rule-gate1
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review docs/plans/deprecation-policy-rule.md for Gate 1 approval.
    - Spot-check the prior-art citations (Kubernetes deprecation policy especially) for accuracy — flagged in the plan as stated from training knowledge, not independently verified via live fetch this session.
    - Decide the open seq-retroactivity question named in the rule (backfill a start date, or grandfather permanently) — or leave undecided; the rule does not require this to be resolved before Gate 1 approval.
    - If approved, this rule ships with no code dependency — safe to merge independently of docs/plans/mcp-coordination-dir-contract-fix.md, though the two are cross-referenced.
  must_read:
    - docs/plans/deprecation-policy-rule.md
    - agf/rules/workflow/deprecation-policy-rule.md
    - .claude/rules/workflow/deprecation-policy-rule.md
  do_not_touch: []
---

# Gate 1: deprecation-policy-rule ready for review

Answers your direct question during Gate 1 review of the MCP `dir` field
rename (what do high-profile OSS projects use as deprecation thresholds,
and what's this project's own plan) with an actual rule, not just an
answer in chat.

## Summary

Two-phase policy, added to both `agf/rules/` (canonical, CUSTOMIZE-marked)
and `.claude/rules/` (this project's filled-in copy):

- **Phase 0** (pre-customer, current state): disclosed breaking changes
  permitted, no deprecation cycle required. This project qualifies today —
  zero paying customers per `agf/learnings/foundry.yaml`.
- **Phase 1** (real external adopters, triggered by the first documented
  pilot, adopter-submitted learning record, or paying engagement):
  minimum support window of 2 minor releases or 60 calendar days
  (whichever longer), 3/90 for security-sensitive surfaces — modeled on
  Kubernetes' published API deprecation policy, the most rigorous
  precedent found (also surveyed: Django, Python/PEP 387, Node.js, Rust
  stdlib, GitHub API).

Motivated by catching that the coordination schema's `seq` field has been
"deprecated" since 2026-06-18 with no defined window — still just kept,
five weeks and several releases later. The rule explicitly leaves `seq`'s
retroactive treatment (backfill a start date vs. grandfather permanently)
as an open decision rather than resolving it unilaterally here.

Full detail in `docs/plans/deprecation-policy-rule.md` (MR !262).
