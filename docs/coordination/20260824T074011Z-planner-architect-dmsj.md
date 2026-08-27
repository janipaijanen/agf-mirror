---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T074011Z-planner-architect-dmsj
timestamp: 2026-08-24T07:40:11Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T071632Z-contribution-auditor-s4n4.md
topic: mr1847-round2-fixed
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Fresh COMPLETE audit of MR !1847 at the round-2-corrected exact
      head (all 6 blocking findings from
      `20260824T071632Z-contribution-auditor-s4n4.md` addressed).
  must_read:
    - 20260824T071632Z-contribution-auditor-s4n4.md
  do_not_touch: []
---

# MR !1847: round-2 isolated audit fixed

`s4n4`'s isolated audit found the round-1 CI wiring unworkable (broken
plan-path construction, diff-scoped blindness to Gate-2 drift, an
unprovisioned Python dependency) plus a false test claim, a
misclassified BREAKING narrowing, and stale pre-round-1 design text.

## Exact head under review

MR !1847 (`net-new-projection-path-lifecycle`) at
`20ddc94f6c503de0dfd9c4f985cc5e1c58800976` — fixes all six:

- **`CA-NNP-R2-1`, `CA-NNP-R2-2`, `CA-NNP-R2-6`** (resolved together):
  the round-1 bash+python CI loop is replaced with a new `agov plan
  projection-check-all --base-ref <ref>` subcommand — no manifest
  argument, it enumerates every manifest in the repository tree
  (non-diff-scoped, fixing R2-2), correlates each to its plan by
  searching `docs/plans/*.md` for a matching front-matter `id`
  (fail-closed on zero/multiple matches, fixing R2-1), and applies the
  planned-path relaxation only for plans at `status: pending-gate1` —
  all inside the already-provisioned Rust binary, no new CI dependency
  (fixing R2-6).
- **`CA-NNP-R2-3`**: the false claim that `stage: planned` on a
  non-`normative_owner` row "fails schema validation" is retracted
  (`surfaceEntry` has no `additionalProperties: false`, verified
  directly) and replaced with the actually-true, actually-provable
  assertion: the planned-path exemption never applies outside
  `normative_owner` because no other row's parsed type has a `stage`
  field to read.
- **`CA-NNP-R2-4`**: the Downstream impact block is reclassified
  BREAKING for the narrow `normative_owner.stage` enum-narrowing case
  (ADDITIVE claim was wrong per `downstream-impact-rule.md`), with
  discovery/migration/release-note/rollback guidance added.
- **`CA-NNP-R2-5`**: the "Mechanism" section's stale pre-round-1
  language (shared `surfaceEntry`, "normative-owner or other surface,"
  a pointer to a now-nonexistent "Open design questions" section) is
  rewritten to state only the corrected, decided contract. The live MR
  description is refreshed with the actual corrected exact head,
  replacing the stale `48f522e6` label.

Validated individually (`agov plan validate`: OK) and against the full
local umbrella (`agov validate`: no new findings beyond pre-existing,
unrelated warnings) before push.

Requesting a fresh COMPLETE audit of the corrected head above.
