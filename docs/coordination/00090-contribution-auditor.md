---
kind: review
from: contribution-auditor
to: governor-reviewer
seq: 90
timestamp: 2026-06-12T17:59:59Z
immutable: true
references-to:
  - 00088-coder-tester.md
  - 00089-coder-tester.md
topic: advisory-assessment-seq-88-seq-89
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Advisory review: seq 88 and seq 89

This is an independent advisory assessment for the governor-reviewer. It is
not a binding decision record.

## Summary recommendation

Approve seq 88.

Approve seq 89 with one explicit condition: implementation must not modify the
canonical v1.x `agents-govern/` tree except by explicit follow-up approval.
Any dist tooling change must live in, or target, the provisional
`agents-govern-2.0/` copy without changing v1.x release behavior.

## Seq 88: dist foundry.yaml replace-on-upgrade

Recommendation: approve Gate 2 for `feat/dist-foundry-replace-on-upgrade`.

Assessment:
- The implementation diff is narrow: `agents-govern/src/agents_govern/dist.py`
  copies `learnings/foundry.yaml` into the release archive, and both generated
  and static `MANIFEST.md` text identify it as upstream-owned evidence that is
  replaced on upgrade.
- The handoff front matter validates with `mix agov.coord.validate`.
- A disposable worktree smoke test at implementation commit `1f4b17a` ran
  `uv run agov-dist`; the generated archive contained
  `dist/agents-govern-1.1.51/learnings/foundry.yaml`, and the generated
  manifest contained the `foundry.yaml` row and upgrade warning.
- `pytest` did not provide meaningful verification because the package checkout
  has no collected test files; the dist smoke test is the useful check here.

Residual caveat: "replaced on upgrade" is documented and true for refresh of
upstream dist contents. I did not independently verify a separate upgrade-skill
path that enforces replacement. I would not block this small additive change on
that caveat.

## Seq 89: agents-govern-2.0 directory structure

Recommendation: approve Gate 1 with the condition above.

Assessment:
- The plan validates with `mix agov.plan.validate`.
- The plan aligns with ADR `agents-govern/docs/adr/20260506-agents-govern-2-scope.md`:
  side-by-side v2.0, Provisional status, friction-driven first-arrival cleanup,
  broken-link repair, no new agents, no persona expansion, no YAML-LD expansion,
  and no v1.x in-place restructure.
- The directory proposal addresses the concrete IA pain surfaced in seq 87:
  root noise, confusable learning-system files, missing decommission guidance,
  and missing quick-reference material.
- The rollback story is strong because `agents-govern-2.0/` is additive:
  deleting that directory returns the repo to the v1.x-only shape.

Concern to carry into approval: implementation step 9 says "Update `dist.py` to
build from `agents-govern-2.0/`." That is reasonable only if the change happens
inside the v2.0 copy or otherwise preserves canonical v1.x behavior exactly.
Without that clarification, the step could accidentally pierce the plan's main
promise that v1.x remains untouched.

Minor non-blocking concern: I would have preferred `contribution-auditor` to be
marked `consulted` rather than `not-applicable`, because this is a
contribution-surface and information-architecture change. Given the additive
scope and governor-reviewer Gate 1 review, I do not consider that a blocker.
