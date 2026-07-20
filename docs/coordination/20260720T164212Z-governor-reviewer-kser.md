---
kind: decision-record
from: governor-reviewer
to: all
id: 20260720T164212Z-governor-reviewer-kser
timestamp: 2026-07-20T16:42:12Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260720T163751Z-planner-architect-hkrr.md
  - 20260720T162959Z-coder-tester-hqhp.md
  - 20260720T162444Z-planner-architect-4584.md
  - 20260720T160959Z-planner-architect-m8k3.md
requires-ack: false
artifacts_owned: []
topic: mr-description-mechanism-decision
---

# Decision: untrack MR_DESCRIPTION.md, move staging to tmp/

Claude-scribed governor decision, given in chat directly by the human
governor ("approved") in response to the converged recommendation in
`20260720T163751Z-planner-architect-hkrr.md`.

Decision: approved as recommended.

1. `MR_DESCRIPTION.md` is untracked from the repository and added to
   `.gitignore`'s existing `tmp/` scope by relocating the staging file to
   `tmp/MR_DESCRIPTION.md`.
2. The GitLab MR description remains the sole authoritative surface. The
   local file is never read back as evidence of current state.
3. Every edit to the staging file must be followed immediately by
   `glab mr update <n> --description "$(cat tmp/MR_DESCRIPTION.md)"` (or
   `glab mr create --description ...` for a new MR).
4. No pre-push hook for now. If tooling is wanted later, an explicit
   `agov mr description sync`-style command is preferred over an implicit
   auto-syncing hook (agreed by both Claude and Codex).

Implementation follows as a small fast-track change (single-purpose,
no shared interface/security boundary): update `.gitignore`, remove the
tracked `MR_DESCRIPTION.md`, relocate to `tmp/MR_DESCRIPTION.md`, and add
the convention to `branch-and-worktree-rule.md`.
