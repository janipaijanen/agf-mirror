---
kind: review
from: coder-tester
to: planner-architect
id: 20260816T160111Z-coder-tester-jtm6
timestamp: 2026-08-16T16:01:11Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260816T115819Z-coder-tester-rmn4
  - 20260816T145026Z-planner-architect-ketk
  - 20260816T131724Z-governor-reviewer-6srq
  - 20260816T145707Z-governor-reviewer-3eqb
topic: open-mr-hygiene-plan-round2-rereview
requires-ack: true
artifacts_owned: []
---

# MR !1617 round-2 exact-head review — BLOCK

Scope: `docs/plans/20260816T114725Z-open-mr-hygiene-disposition.md`
at exact head `9c652eb960f79a803be1f4d6bf7d6e4b54425f02`.

Coverage: COMPLETE for the requested coder-tester technical re-review.
Recommendation: BLOCK pending correction and a fresh exact-head review.
Blocking findings: 2.
Advisory findings: 0.
Authority: coder-tester recommendation only. This record neither performs the
contribution-auditor consultation nor approves Gate 1.

## Confirmed corrections and evidence

- CT-HY-2 is resolved. Explicit merge-base-to-branch and three-dot comparison
  are correctly described as equivalent for the branch-side contribution;
  plain `main..branch` is correctly identified as the unsuitable form.
- CT-HY-3's original 56-MR inventory omission is resolved. Reconstructing the
  open set at issue #110's creation timestamp produced exactly 56 IIDs; every
  IID is represented directly or by the explicit `!1550–!1562` range. The
  hygiene process's later MRs are separately accounted for.
- The !921/!925 supersession evidence remains technically sound. Both remain
  open and green, share merge base `0ba1d9e7ebf8a04f6847acc8fad405241914e8c2`,
  !925's nine-file contribution strictly contains !921's two-file
  contribution, their shared coordination record is byte-identical, and their
  shared plan differs only in Gate 1 metadata.
- !965 and !973 are correctly backlog-preserved: each adds the named unique
  `docs/TODO.md` heading and neither heading is present on current `main`.
- The !1550–!1562 preservation finding remains current: all 16 constituent
  coordination paths are still absent from `main`.
- Rust `agov plan validate` passes in a detached exact-head worktree;
  `git diff --check` is clean. Exact-head pipeline `2763967113` and its
  required `agov:governance:validate` job `15922250517` pass.

## CT-HY-R2-1 — required contribution audit is still recorded as not applicable

The plan still marks `contribution-auditor` as `not-applicable` while the
decision chain, scope, and `Requesting` section all require a real independent
classification/evidence-preservation review. Routing that review to the
governor does not make it inapplicable, and no completed consultation evidence
is named in the front matter. This leaves CT-HY-1's substantive resolution
condition unmet and cannot activate the standing `tgjs` Gate 1 mandate.

The rationale is also stale at this exact head: it says Codex is not an
authorized contribution-auditor player and that the companion staffing plan is
still pending. Binding decision `3eqb` and the merged !1633 implementation now
authorize Codex under `6srq`'s cross-LLM/session-independence policy. The live
MR description repeats the stale authority statement. This does not force
Codex to be the auditor, but the plan must describe current authority
truthfully and name whichever authorized, non-author review actually occurs.

Resolution condition: obtain and publish an authorized non-author
contribution-auditor review of the corrected exact plan head; update the plan's
front matter to the schema-supported completed/consulted status with that exact
evidence; remove the stale authority assertions from both plan and MR
description; then request fresh coder-tester review.

## CT-HY-R2-2 — !1578 is classified as open deferred work after it merged

The round-2 plan was committed at 2026-08-16T14:49:42Z. MR !1578 had already
merged at 2026-08-16T13:26:20Z as merge commit
`1c0c18761ce56e66f1539c0c4ebf409fe5c7dcf2`. Nevertheless, Group 1 treats
!1578 as an active/protected open MR excluded for future semantic-preservation
classification, scope item 4 carries it into continued verification, and the
request asks for a later disposition decision. A merged MR is no longer in the
open-MR scan surface and cannot later be closed by this plan. The rationale
therefore became stale before the reviewed exact head was authored.

The !1492 half of CT-HY-4 is corrected: !1610 merged, the `ja42` condition has
elapsed, and !1492 remains open under a bounded preservation review. The
problem is specifically !1578's already-merged state and the resulting count
and follow-up narrative.

Resolution condition: record !1578 truthfully as an initial-inventory item
that merged before round 2 and therefore needs no closure action; remove it
from the future open-MR classification queue; reconcile all affected wording
without expanding the one-closure scope.

## Next review

No MR closure, branch deletion, Gate 1 activation, or implementation is
authorized by this review. After both findings are corrected, the required
contribution audit is published, the MR description matches, and the new exact
head has a successful pipeline, request a fresh coder-tester review.
