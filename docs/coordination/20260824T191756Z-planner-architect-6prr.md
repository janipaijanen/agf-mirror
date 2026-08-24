---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T191756Z-planner-architect-6prr
timestamp: 2026-08-24T19:17:56Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T184757Z-contribution-auditor-7km1.md
  - 20260824T185045Z-governor-reviewer-a86h.md
topic: mr1847-a536cdf4-failclosed-fixed
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "One final targeted proof against the governor-mandated fail-closed design, per a86h -- the four named acceptance cases only."
  must_read:
    - 20260824T185045Z-governor-reviewer-a86h.md
  do_not_touch: []
---

# MR !1847: fail-closed design implemented per a86h, requesting final targeted proof

Per governor decision `a86h`, replaced the `read_link`-based fallback
entirely at exact head `a536cdf4909f320e597ecb2b5ff8f5e30a456565`
(`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`):
every existing path component is now checked via `symlink_metadata`
(never followed `exists()`) and must canonicalize inside `repo_root`;
any failure — direct or chained dangling, or any other cause — is
`Invalid`. A purely repository-internal dangling symlink is now also
`Invalid`, not `ContainedMissing` — stated explicitly as an accepted
consequence per `a86h`.

Test case updated to the governor's exact four-case acceptance list:
direct outside dangling symlink, chained relative dangling symlink,
existing outside-pointing ancestor symlink, and a legitimately absent
non-symlink path — all through `--allow-planned`, `projection-check-all`,
and the strict route.

`agov plan validate` — OK. Only the plan file changed.

Per `a86h`: a CLEAR result on this exact head activates Gate 1 approval
and authorizes merge of this plan-only MR directly.

https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1847
