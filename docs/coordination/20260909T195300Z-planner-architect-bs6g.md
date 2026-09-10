---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260909T195300Z-planner-architect-bs6g
timestamp: 2026-09-09T19:53:00Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T194219Z-governor-reviewer-j8bq.md
  - 20260909T193208Z-coder-tester-nj1j.md
  - 20260909T193710Z-planner-architect-8255.md
topic: post-v211-queue-closeout-work-order-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T195116Z-post-v211-queue-closeout-work-order.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 decision on the post-v2.11 queue-closeout work order (the Part B split from withdrawn MR !2635, per nj1j and j8bq)."
    - "Approve or revise: the one-vocab/two-ledger model; per-disposition execution rules incl. the durable-carrier requirement for reconstruct; the batch mechanism (planner proposal -> non-author review -> governor decision record -> digest-verified execution -> completion record -> separate Gate 2 ledger MR) and its partial-failure/reopen recovery; the cohort plan and exclusion set; the execute-after-v2.11 sequencing; and the QC-1..QC-4 decomposition."
  must_read:
    - docs/plans/20260909T195116Z-post-v211-queue-closeout-work-order.md
    - docs/coordination/20260909T194219Z-governor-reviewer-j8bq.md
    - docs/portfolio/post-v248-issue-disposition.md
    - docs/portfolio/post-v210-mr-disposition.md
  do_not_touch:
    - any MR or issue close before its execution batch's governor decision
    - v2.11 metadata merge, tag, release, or deployment
    - issue #129 and MRs !2459 / !2460 / !2462 (semantic-data exclusion)
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1: post-v2.11 queue-closeout work order

Split from withdrawn MR !2635 per `nj1j`; drafted against the amended
release-first order that `j8bq` makes binding (B2–B8 bulk forge
close/park/reconstruct execution is **not** a v2.11 cut precondition;
execution happens after v2.11 is published and verified).

## What it fixes vs. the withdrawn plan

- **V211-P5** — arithmetic corrected to the live open split (8
  close-superseded + 20 reconstruct + 26 park + 5 v2.11 + 4 awaiting + 1
  next = 64 issues; 58 open MRs at drafting). Each batch freezes its own
  baseline.
- **V211-P6** — `reconstruct` requires a **durable visible carrier**
  (tracked file with accepted scope, owner, gate/acceptance criteria,
  unique content, backlinks, validation) present **before** the source
  closes. No empty branch / stub.
- **V211-P7** — batch mechanism fully specified: planner submits a
  proposal with immutable exact IID + state + note + SHA-256 digest per
  item; **only the governor issues the authorizing decision record**;
  step-4 digest re-read invalidates any changed item; idempotent
  per-item execution; stop-on-first-failure; completion record pins what
  actually closed; ledger reconciliation MR is a **separate Gate 2**.
- Semantic-data exclusion (`#129`, `!2459`, `!2460`, `!2462`) hard-excluded
  from every slice per `j8bq`.
- `Closes #N` in CHANGELOG is informational only (GitLab does not
  auto-close from changelog content) — issue close happens post-publication.

The **v2.11 release-publication child plan** is a separate artifact (the
Part A split), which `j8bq` also authorizes me to draft against the amended
order.

`agov plan validate` → OK (blocking). `git diff --check` → clean. No
projection manifest. Grants no MR-close, issue-close, metadata-merge, tag,
release, or deployment authority; every slice and batch returns for its
own decision.
