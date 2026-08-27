---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260730T160903Z-governor-reviewer-gn47
timestamp: 2026-07-30T16:09:03Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260730T145733Z-governor-reviewer-3g7k
  - 20260730T153314Z-contribution-auditor-r61q
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/834
topic: v246-release-amendment-gate1-approved
requires-ack: false
artifacts_owned:
  - docs/plans/v246-release.md
---

# Gate 1 amendment approved for the v2.4.6 release plan

The governor-reviewer approves the Gate 1 amendment in MR !834 at exact
reviewed head `4354113f713625ded0d419fee3f91f9eddfea2e6`.

This supersedes the original plan's two-file release implementation scope and
authorizes exactly the amended 22-file release metadata/truth-surface scope:
five core/tooling files, the distributed roadmap, 12 agent definitions plus
the live agent manifest, and three adopter/manifest guidance files. The exact
constraints and Gate 2 evidence requirements are those in
`docs/plans/v246-release.md` at the approved head.

The approval does not authorize runtime behavior, schema, rule, CI, external
dependency/checksum, or future release-truth refactor changes. Issue #94's
compatibility-baseline redesign remains post-v2.4.6 Level 1 Planned intake.

After this decision record and the plan's mechanical approved-status update
are published, merge MR !834, re-reconcile current `main`, implement the
approved release metadata on `release-2.4.6`, and submit the release MR for a
separate exact-head Gate 2 decision. Do not create tag `v2.4.6` before that
release MR is approved and merged.
