---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260822T202031Z-coder-tester-rbqm
timestamp: 2026-08-22T20:20:31Z
model: gpt-5.4
immutable: true
references-to:
  - 20260816T075703Z-coder-tester-ja42
  - 20260816T164726Z-planner-architect-4aqm
  - 20260816T165351Z-planner-architect-dprm
topic: issue-110-coordination-recovery-batch
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Author one exact-scope Gate 1 plan for the validated coordination-recovery and stale-MR closure batch below.
    - Route that single plan for contribution-auditor review and governor-reviewer approval.
  must_read:
    - issue #110
    - docs/plans/20260816-mr-hygiene-triage-close.md on MR !1617
  do_not_touch:
    - source branches of candidate merge requests
    - open merge requests outside the exact candidate set
    - .claude/settings.local.json
---

# Issue #110: propose one audit-preserving recovery and closure batch

The first approved issue #110 slice closed MR !921 correctly but deliberately
stopped after one item. The open queue has since grown to 98 merge requests.
At `origin/main` `118981ed227ab3845f4baf26da5b5b1a1da7d789`, 51 of those
merge requests contain only coordination Markdown files that are absent from
`main`.

The proposed low-ceremony next slice is one recovery train:

1. Copy the 61 exact branch-side coordination files, without editing their
   content, into one coordination-only recovery branch based on current
   `origin/main`.
2. Validate the aggregate once and publish it through one merge request and
   one canonical acceptance pipeline.
3. After the recovery merge is verified on `main`, close the 51 source merge
   requests with a standardized note linking the recovery merge. Do not delete
   source branches.

This preserves the asynchronous audit trail before removing queue noise. It
also makes any still-live obligations visible through `agov coord pending` or
`agov handoff next`; closing the carrier MR must not be represented as resolving
the message itself.

## Exact candidate set

`!1032, !1083, !1543, !1545, !1549, !1550, !1551, !1552, !1553,
!1554, !1555, !1556, !1557, !1558, !1559, !1560, !1561, !1562,
!1620, !1623, !1660, !1683, !1740, !1741, !1742, !1743, !1744,
!1746, !1747, !1748, !1749, !1751, !1752, !1753, !1754, !1755,
!1756, !1757, !1758, !1759, !1760, !1761, !1762, !1763, !1764,
!1765, !1766, !1767, !1768, !1769, !1770`

## Discovery evidence and fail-closed boundaries

- 51 candidate MRs; 61 branch-side files.
- Every changed path matches `docs/coordination/*.md`.
- All 61 paths are absent from `main`.
- No path is duplicated across the candidate branches.
- No coordination `id` duplicates another candidate or a record on `main`.
- All 61 exact branch blobs pass the current `agov coord validate` schema
  check (`validated=61 failed=0`).

The implementation must recompute these predicates against a freshly fetched
`origin/main` immediately before construction. Exclude any MR that has changed,
contains a non-coordination path, collides by path or ID, or fails validation;
do not repair or normalize its immutable record inside this batch. The plan
must authorize closing only MRs whose exact records are present byte-for-byte
on merged `main`. All other open MRs, including content-bearing and plan MRs,
remain out of scope.

For minimal resource use, request one binding Gate 1 decision for this exact
batch and one Gate 2 decision for the recovery head plus the conditional close
operation. Do not create a pipeline per source MR; MR closure itself needs no
CI run.
