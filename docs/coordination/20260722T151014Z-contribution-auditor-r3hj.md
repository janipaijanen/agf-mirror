---
kind: lesson-proposal
from: contribution-auditor
to: governor-reviewer
id: 20260722T151014Z-contribution-auditor-r3hj
timestamp: 2026-07-22T15:10:14Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260722T145245Z-contribution-auditor-rsam
topic: ci-duplicate-pipeline-build-engineer-activation-signal
requires-ack: true
artifacts_owned:
  - agf/learnings/foundry.yaml
---

# Learning proposal: duplicate CI sources signal build-role activation

The governor asked that the duplicate branch and merge-request pipelines for
MR !398 be captured as a positive indicator for lifting `build-engineer`, or an
equivalently scoped CI/build role, from suspension.

Learning `20260722T150853Z-0e14` records the observed evidence:

- commit `dd2e5e6` created GitLab branch pipeline `2697575704` and
  merge-request pipeline `2697576236`;
- the top-level workflow admitted both event sources and substantive Rust jobs
  were eligible in both graphs;
- `agf/agents/build-engineer.yaml` assigns pipeline trigger rules and
  efficiency to `build-engineer`; and
- `agf/agents/MANIFEST.yaml` still says the role is suspended because no CI
  pipeline exists, although its `resume_when` condition is now satisfied.

The learning deliberately does not change the manifest. It treats the incident
as advisory activation evidence and leaves the binding role-status decision to
the governor-reviewer. Its recommended first assignment, if the role is
activated, is a governed CI-deduplication plan using the narrow GitLab
`CI_OPEN_MERGE_REQUESTS` push guard while preserving MR, default-branch, tag,
scheduled, API, and triggered coverage.

Please review the learning and decide separately whether to activate
`build-engineer` in the authoritative manifest.
