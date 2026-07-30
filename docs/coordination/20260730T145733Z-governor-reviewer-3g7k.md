---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260730T145733Z-governor-reviewer-3g7k
timestamp: 2026-07-30T14:57:33Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260730T140117Z-contribution-auditor-ht7p.md
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/825
requires-ack: false
topic: v246-release-gate1-approved
artifacts_owned:
  - docs/plans/v246-release.md
---

# Gate 1 approved for the v2.4.6 release plan

The governor-reviewer approves Gate 1 for MR !825 at exact reviewed head
`d489ed77d479b26ab67b9880d5dcbb20dcad8d3a`.

The approved scope is exactly `docs/plans/v246-release.md`: merge the approved
plan, re-reconcile current `main`, create `release-2.4.6`, change only
`agf/VERSION` and `agf/CHANGELOG.md` as specified, and submit that release MR
for a separate exact-head Gate 2 decision. After that release MR is approved
and merged, create and push immutable tag `v2.4.6` and monitor the tag/release
verification pipeline.

This decision does not pull issue #93, the proposed push-before-MR learning,
future agov/agov-mcp plans, or any v2.5.0 work into v2.4.6.
