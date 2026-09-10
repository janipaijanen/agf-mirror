---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260910T120412Z-planner-architect-hsqs
timestamp: 2026-09-10T12:04:12Z
model: claude-sonnet-5
immutable: true
topic: v211-scope-amendment-pmm-role-and-estate-assessment-precondition-gate1
references-to:
  - 20260910T050007Z-planner-architect-jh3g.md
  - 20260910T120152Z-planner-architect-2skr.md
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T021841Z-v211-release-scope.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on the v2.11 parent-scope amendment adding the documentation-estate-steward role as IN row 8 and its first estate-wide assessment as release-cut condition 4 (branch plan/v211-scope-pmm-precondition).
    - This is R-2 of the steward routing; it is a separate governor Gate 1 from the role instantiation (R-1, MR !2661), same shape as the #129-withdrawal and open-MR-queue-hygiene amendments.
    - On approval, R-3 folds "estate assessment precedes freezing the metadata allowlist + CHANGELOG scope" into the v2.11 release-publication child plan (its own metadata edit + your confirmation).
  must_read:
    - docs/plans/20260909T021841Z-v211-release-scope.md
    - docs/coordination/20260910T050007Z-planner-architect-jh3g.md
    - docs/plans/20260910T115843Z-documentation-estate-steward-role.md
  do_not_touch:
    - the other three post-approval amendments in the same plan (er78, #129, open-MR queue hygiene) - unchanged
    - the role definition itself (that is R-1 / MR !2661)
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Handoff: v2.11 scope amendment — PMM role IN + estate-assessment cut precondition (Gate 1)

## What this is

**R-2** of the steward routing in `jh3g` / `2skr`. Amends
`docs/plans/20260909T021841Z-v211-release-scope.md` on branch
`plan/v211-scope-pmm-precondition`:

1. **New subsection** "Post-approval scope amendment —
   documentation-estate-steward (PMM) role": states the governor decision,
   the `qvzj` evidence, the IN row 8 (`ship-in` the role), and the
   condition-4 precondition.
2. **Release-cut precondition — new condition 4**: the steward's first
   estate-wide assessment is complete and every finding is dispositioned
   (fixed / accepted-as-current / deferred-with-reason /
   referred-and-governor-dispositioned), run at the
   `release-content-freeze` cadence **before** the metadata allowlist and
   CHANGELOG scope are frozen. A finding with no recorded disposition
   blocks the cut.
3. **Child-plan constraint bullet**: three-part → four-part condition,
   adding (d).
4. `status: approved → pending-gate1` (same pattern as the two prior
   amendments; this is its own governor Gate 1).

## Disposition authority (your 2026-09-10 answer, encoded)

Steward dispositions in-scope findings directly; escalates
normative / cross-role / structural to you; a referred finding is
dispositioned by a governor record. The precondition is satisfied when
**every** finding has a recorded disposition (steward or governor) — not
when every finding is fixed.

## Sequence after approval

R-3: fold "estate assessment precedes freezing the metadata allowlist +
CHANGELOG scope" into `docs/plans/20260909T203636Z-v211-release-publication.md`
(a metadata edit + your confirmation, same review cycle). Then the
steward's first sweep (R-4) runs as part of the v2.11 drive.

## Decision requested

Gate 1 approval of R-2. On approval: `status → approved`,
`gate1_approved_by` citing your decision record; R-3 follows.
