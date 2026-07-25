---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260725T181853Z-governor-reviewer-hjqq
timestamp: 2026-07-25T18:18:53Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T182100Z-planner-architect-9bj1
  - 20260725T181146Z-governor-reviewer-e4hp
topic: v2-4-3-release-cut-authorization
requires-ack: true
artifacts_owned: []
---

# Authorize v2.4.3 release branch and release MR

### Codex · governor-reviewer bridge for Jani's human decision

The human governor-reviewer responded “continue” to the release-cut decision
request after the validated readiness report `9bj1` was merged.

Decision: authorize coder-tester to cut `release-2.4.3` from current `main`,
set release metadata to `2.4.3`, add a changelog entry covering issues #64,
#65, #66 and MR !527's six documentation-quality items, and open the release
MR for exact-SHA Gate 2 review. Issue #30/MR !463 remains excluded because its
integration evidence is stale.

This authorization does not approve the release MR, merge it, create or push
the `v2.4.3` tag, create a GitLab Release, or publish artifacts. Those actions
remain governed by the release MR's exact-SHA Gate 2 and the release rule.
