---
kind: decision-record
from: governor-reviewer
to: all
id: 20260909T194219Z-governor-reviewer-j8bq
timestamp: 2026-09-09T19:42:19Z
model: human-via-codex
immutable: true
references-to:
  - 20260909T163409Z-governor-reviewer-sd23
  - 20260909T184019Z-governor-reviewer-st83
  - 20260909T193710Z-planner-architect-8255
topic: v211-release-first-parent-sequence-amendment
requires-ack: false
artifacts_owned: []
---

# v2.11 release-first sequence supersedes the earlier B2–B8 cut condition

The governor-reviewer has directed: “proceed. After RUW is ready, then focus
on closing those open MRs.” This continues the standing mandate to collaborate
until RUW v2.11 is released and makes the intended order binding.

The v2.11 parent scope's earlier requirement that every B2–B8 source receive a
verified live terminal disposition before the metadata merge request opens is
superseded. Merged, validated B2/B3/B8 ledgers and the B4–B7 analysis remain
release-readiness evidence, but their bulk forge close/park/reconstruct
execution is not a v2.11 cut precondition. After v2.11 is published and
verified, the active priority becomes evidence-based terminal disposition of
the remaining open merge requests and corresponding issues.

This sequence amendment does not authorize indiscriminate closure. Each
post-release disposition must retain durable accepted scope and evidence,
respect `awaiting-governor`, `ship-in`, `merge-now`, and exclusion states, use
the applicable gate or governor batch decision, and record partial failures and
ledger reconciliation. Issue #129 and MRs !2459, !2460, and !2462 remain
excluded from semantic-data analysis, reconstruction, carrier work, or closure
without a fresh governor decision.

Planner-architect is authorized to draft the complete v2.11
release-publication child plan against this amended order and a separate
post-v2.11 queue-closeout work order. This record grants no Gate 1 approval for
those plans and no metadata merge, tag, release, deployment, or forge-close
authority.

This also acknowledges the planner's `8255` disposition: the withdrawn combined
plan is replaced by the two independently gated plans, and the CP-B-1 copy gap
must be resolved through its proposed narrow Gate 1 clarification before that
implementation can advance.
