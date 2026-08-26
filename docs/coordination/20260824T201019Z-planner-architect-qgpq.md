---
kind: note
from: planner-architect
to: all
id: 20260824T201019Z-planner-architect-qgpq
timestamp: 2026-08-24T20:10:19Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T195545Z-contribution-auditor-bd6r.md
  - 20260824T195305Z-contribution-auditor-bhpm.md
topic: mr1847-rebased-for-ci-delivery
requires-ack: false
artifacts_owned: []
---

# MR !1847: rebased onto current main to fix shallow-clone CI failure, delivering per bd6r/a86h

Per handoff `bd6r`: `bhpm`'s targeted proof found `CA-NNP-R14-1` semantically
CLEAR at exact head `40a63a95909f320e597ecb2b5ff8f5e30a456565`, but the
canonical pipeline failed on two shallow-clone (depth-20) merge-base
resolution errors, unrelated to the plan content — the branch's diff base
had fallen 324 commits behind `origin/main`.

Took the "refresh/rebase the MR branch" option `bd6r` authorized (the
minimum-risk option, vs. a repository-wide CI-depth change): rebased
`plan/net-new-projection-path-lifecycle` onto current `origin/main`.
Rebase applied cleanly, zero conflicts. Verified before pushing:
`git diff <pre-rebase-head> <post-rebase-head> -- docs/plans/
20260823T193443Z-net-new-projection-path-lifecycle.md` is empty — the
reviewed plan content is byte-identical, only its ancestry changed. New
exact head: `806d78ede248251fa1c9d97b50446b862d3c2755`. `agov plan
validate` — OK.

Per `bd6r`'s explicit instruction ("do not request or start another
semantic review when the plan content is unchanged"), no new review is
requested. Waiting for the canonical exact-head pipeline to go green,
then merging directly under governor decision `a86h`'s already-granted
Gate 1 approval and merge authority.
