---
kind: decision-record
from: governor-reviewer
to: all
id: 20260815T091535Z-governor-reviewer-dkq5
timestamp: 2026-08-15T09:15:35Z
model: human
immutable: true
references-to:
  - 20260815T083951Z-coder-tester-es3r.md
  - 20260815T082712Z-planner-architect-pggm.md
  - 20260814T074427Z-governor-reviewer-33br.md
topic: mr1544-bounded-branch-rule-scope-ratified
requires-ack: false
artifacts_owned:
  - agf/rules/workflow/branch-and-worktree-rule.md
  - .claude/rules/workflow/branch-and-worktree-rule.md
---

# Binding decision: ratify MR !1544's bounded branch-rule scope addition

This record transcribes Jani's explicit in-session response to the proposed
binding confirmation: “proposed binding is approved. please proceed.”

## Decision

The governor-reviewer ratifies retaining the corrected two-file
`branch-and-worktree-rule.md` addition in MR !1544 at exact head
`5148934508cdeaf7528a8ffea442fa200e44a55e`.

The authorization is limited to the wording already independently reviewed in
`es3r`:

- release-version metadata is constructed on the dedicated reconstruction
  source `release-vX.Y.Z`;
- that source is merged into protected reconstruction target
  `reconstruct/vX.Y`, which is never a direct-push location; and
- the immutable tag is created only from the reviewed target-side merge
  commit.

The only authorized additional surfaces are:

- `agf/rules/workflow/branch-and-worktree-rule.md`; and
- `.claude/rules/workflow/branch-and-worktree-rule.md`.

No other scope expansion is authorized. This ratification closes the durable
authority condition identified as `B-FSR-IMPL-R2-1` for the named exact head.

## Boundaries

This decision does not itself approve MR !1544 at Gate 2, authorize its merge,
approve or merge coordination MRs !1546/!1547, approve MR !1492, create a
release branch, publish v2.6.0, change the frozen release sequence, resume
OpenClaw #104, or authorize any broader exception to Gate discipline. Those
decisions remain separate.
