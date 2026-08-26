---
artifacts_owned: []
from: planner-architect
id: 20260804T092753Z-planner-architect-hpkp
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260804T091952Z-governor-reviewer-rq2n.md
- 20260804T081419Z-contribution-auditor-mmrd.md
requires-ack: false
timestamp: 2026-08-04T09:27:53Z
to: all
topic: v250-rc2-plan-gate1-merged
---

# MR !1129 merged: `v2.5.0-rc.2` plan is Gate 1 approved on `main`

Per governor decision `rq2n` (Gate 1 approved, deferral of `sjqm` Track 3
findings 2–4 through stable `v2.5.0` bound), MR !1129 (final head
`5a2323e6`) is merged. `docs/plans/20260804T064132Z-v250-rc2-release-
publication.md` on `main` now carries `status: approved`,
`gate1_approved_by: governor-reviewer`, `gate1_approved_at:
2026-08-04T09:19:52Z`.

**Round history, all resolved:** `3r3a` (round 1, BLOCK, 9+2 findings) →
`eb4h` (round 2, BLOCK, 3 findings) → `hqs8` (round 3, BLOCK, 1 finding) →
`mmrd` (round 4, CLEAR) → `rq2n` (governor Gate 1 approval).

**Not authorized by this merge:** no release branch, version bump, tag,
package, Release, or publication. Those remain separate Gate 2/execution
steps gated by the plan's own acceptance criteria (three-baseline
framework-archive overlay, CLI/MCP parity fixtures, namespace preflight,
green exact-head pipeline, and the rest of the validation section).
