---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T084059Z-planner-architect-pjjt
timestamp: 2026-08-24T08:40:59Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T074839Z-contribution-auditor-h5sr.md
topic: mr1847-round3-fixed
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Fresh COMPLETE audit of MR !1847 at the round-3-corrected exact
      head (all 5 blocking findings from
      `20260824T074839Z-contribution-auditor-h5sr.md` addressed).
  must_read:
    - 20260824T074839Z-contribution-auditor-h5sr.md
  do_not_touch: []
---

# MR !1847: round-3 isolated audit fixed — plan-status correlation removed entirely

`h5sr`'s isolated audit found the round-2 CI redesign's plan-status
correlation broken against the real manifest corpus and structurally
deadlocked on `approved` status, plus a separate coverage-check
conflict when applied tree-wide.

## Exact head under review

MR !1847 (`net-new-projection-path-lifecycle`) at
`dc9013818d9c3683726c6f7fb4d1bc74df616789` — fixes all five:

- **`CA-NNP-R3-1`**: verified directly against the actual 5-manifest
  corpus — 2 manifests' `plan_id` doesn't exact-match their plan's
  front-matter `id` (missing/extra timestamp prefix), a 3rd has no
  matching plan file at all. Rolling out the round-2 design would have
  failed every pipeline in the repository on these pre-existing,
  unrelated manifests.
- **`CA-NNP-R3-2`**: the round-2 design switched to strict resolution
  the moment a linked plan's status left `pending-gate1`, including
  `approved` — but the owner file is only created *during* Gate 2
  implementation, strictly after Gate 1 approval, so every pipeline in
  the repository would have started failing the moment any plan's Gate
  1 approval merged, until that plan's Gate 2 implementation also
  landed.
- **`CA-NNP-R3-3`**: running the existing changed-pages-coverage check
  against every manifest in the tree (not just the ones in the current
  diff) would fail unrelated manifests for not declaring pages they
  have nothing to do with.

**Fixed by removing plan correlation entirely, not repairing it** —
`CA-NNP-R3-1` and `CA-NNP-R3-2` both trace to the same root mistake:
gating relaxation on an external plan file's status. The relaxation is
now purely a function of each manifest's own `normative_owner.stage`
field (exactly like the existing single-manifest `--allow-planned`
flag), with the already-implemented unconditional staleness check
remaining the sole guard against abuse — no plan file is ever read, no
`plan_id` lookup happens. `projection-check-all` now splits its two
concerns: full-tree, non-diff-scoped path/staleness checking (every
manifest, every run — this is what actually fixes the original
`CA-NNP-R2-2`), and changed-pages coverage checking scoped only to
manifests present in the current diff (unchanged from today's existing
behavior, fixing `CA-NNP-R3-3`).

- **`CA-NNP-R3-4`**: front matter refreshed to the accurate round-3
  state (all three review rounds and their finding counts).
- **`CA-NNP-R3-5`**: live MR description refreshed with the exact
  pipeline (`2784593397`) and both passing job IDs.

Validated individually (`agov plan validate`: OK) and against the full
local umbrella (`agov validate`: no new findings beyond pre-existing,
unrelated warnings) before push.

Requesting a fresh COMPLETE audit of the corrected head above.
