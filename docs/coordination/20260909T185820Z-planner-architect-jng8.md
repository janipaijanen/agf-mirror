---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260909T185820Z-planner-architect-jng8
timestamp: 2026-09-09T18:58:20Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T184019Z-governor-reviewer-st83.md
  - 20260909T021841Z-planner-architect-mejh.md
topic: v211-release-and-queue-closeout-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T185609Z-v211-release-and-queue-closeout.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 decision on the v2.11 release-publication + post-release queue-closeout mechanism plan (draft, per the 2026-09-09 governor direction and st83)."
    - "Approve or revise: Part A as the required child v2.11 release-publication plan; the A1 precondition checklist; Part B's one-vocab/two-ledger/per-disposition rules; the B4 batch consent-agenda mechanism and B5 safety rails; the Part A -> Part B sequencing; and the RC-A..RC-B4 decomposition."
  must_read:
    - docs/plans/20260909T185609Z-v211-release-and-queue-closeout.md
    - docs/plans/20260909T021841Z-v211-release-scope.md
    - docs/portfolio/post-v248-issue-disposition.md
    - docs/portfolio/post-v210-mr-disposition.md
  do_not_touch:
    - version bump, tag, release, package, or OCI state
    - any MR or issue close before its slice's batch approval
    - semantic/linked-data scope (st83 standing direction)
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1: v2.11 release publication + queue closeout mechanism

Per the governor direction "plan draft release content and mechanism to
close vast majority of open MRs and Issues" and `st83`.

## Shape

- **Part A** is the child v2.11 release-publication plan the parent scope
  plan (`sd23`) requires: A1 the hard precondition checklist (restated
  against today's state — v2.11 is weeks out; CP-B impl, CP-B-2, `qbka`
  A/B, the coordination-semantics work order, #123/CP-C, B4-B7, and the
  Propeller genuine wake + Level-5 are all still open), A2 release content
  (metadata surfaces, CHANGELOG structure, `Closes #N` from the B3
  `ship-in v2.11.0` set), A3 the standard release mechanism
  (namespace-check + protected-tag checkpoints), A4 tag/publish stays a
  separate post-merge governor decision.
- **Part B** closes the vast majority of the 54 open MRs and 64 open issues
  via one §3b vocab over the two existing portfolio ledgers, per-disposition
  execution rules (`close-superseded` cites the canonical artifact;
  `reconstruct` creates a carrier branch first; `Archive -> close` uses the
  proven B4 content-hash archive; `merge-now` never batches), a batch
  consent-agenda mechanism (one coordination record per class cohort ->
  non-author review -> governor batch approval -> execute -> completion
  record with exact IIDs) so the closes do not need one MR each, and B5
  safety rails against indiscriminate closure.
- **Sequencing:** Part A before Part B (`st83`), with two pre-release
  exceptions: B4-B7 *dispositioning* (analysis) and the stale-Gate-1-plan-MR
  cohort (its execution is what brings the B8 intake bound under its <=3
  limit).

## Evidence

`agov plan validate` -> OK (blocking). `git diff --check` -> clean. No
projection manifest (introduces no normative rule text — it is a work order
over existing ledgers and the existing release rule). Grants no version,
tag, release, MR-close, or issue-close authority; every slice and batch
returns for its own decision.

`infosec-sentinel` and `contribution-auditor` are `waived` pending a routed
Gate 1 review; the specific things each must confirm are named in the plan
front matter.
