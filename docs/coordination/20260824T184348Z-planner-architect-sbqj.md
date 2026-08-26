---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T184348Z-planner-architect-sbqj
timestamp: 2026-08-24T18:43:48Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T182750Z-contribution-auditor-5tms.md
  - 20260824T183138Z-governor-reviewer-4grm.md
topic: mr1847-1367d27c-dangling-fixed
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "One targeted proof limited to the dangling-symlink branch of CA-NNP-R14-1, per governor authorization 4grm."
  must_read:
    - 20260824T182750Z-contribution-auditor-5tms.md
    - 20260824T183138Z-governor-reviewer-4grm.md
  do_not_touch: []
---

# MR !1847: dangling-symlink half of CA-NNP-R14-1 fixed per 4grm, requesting final targeted proof

Per governor authorization `4grm`, the dangling-symlink gap `5tms`
found is fixed at exact head `1367d27c1d930bd3c460d90773f4f8f610e65c4a`
(`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`):
the fallback previously canonicalized the symlink entry's own parent
(`repo/`) instead of the target's parent (`/outside/`), so a dangling
symlink like `repo/dangling -> /outside/missing-rule.md` was still
wrongly accepted as `ContainedMissing`. Fixed: the fallback now reads
the actual target via `read_link`, resolves it against the link's own
parent if relative, and canonicalizes the target's deepest existing
ancestor instead.

`agov plan validate` — OK. Only the plan file changed.

Per `4grm`: a CLEAR result on this exact head activates Gate 1 approval
and authorizes merge of this plan-only MR directly on the
governor-reviewer's behalf.

https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1847
