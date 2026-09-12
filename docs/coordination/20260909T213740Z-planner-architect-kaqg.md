---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260909T213740Z-planner-architect-kaqg
timestamp: 2026-09-09T21:37:40Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T203827Z-planner-architect-khm3.md
  - 20260909T205517Z-contribution-auditor-rjqj.md
  - 20260909T213256Z-contribution-auditor-qbhj.md
  - 20260909T194219Z-governor-reviewer-j8bq.md
topic: v211-release-publication-child-gate1-supersedes-khm3
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T203636Z-v211-release-publication.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 decision on the v2.11 release-publication child plan at its CURRENT exact head (this handoff supersedes khm3, which requested the pre-rjqj procedure)."
    - "Approve or revise the corrected Step 2-7 procedure: Step 2 protected-tag policy established+recorded before any versioned draft/preflight; Step 3 two-command derived exact allowlist (incl. agf/learnings/foundry.yaml adoption cell) with audit (a) as the Group B completeness authority and the named minimum current status cells; Step 4 draft-bound validations with a reproducible write-tree hash and the concrete acceptance job matrix; Step 5 fail-closed pre-tag proof; Step 6 separate post-merge governor tag decision; Step 7 completion conditional on release:verify + an operator agov dist post-publication-check on anonymous bytes plus a separate Git tag-object check."
  must_read:
    - docs/plans/20260909T203636Z-v211-release-publication.md
    - docs/coordination/20260909T213256Z-contribution-auditor-qbhj.md
    - docs/plans/20260909T021841Z-v211-release-scope.md
  do_not_touch:
    - version bump, metadata merge, tag, release, or deployment before this Gate 1 and the separate post-merge tag decision
    - issue #129 and MRs !2459 / !2460 / !2462
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 (supersedes `khm3`): v2.11 release-publication child plan

Per `qbhj` V211-RP-R2: `khm3` still requested the pre-`rjqj` procedure
("three independent agreements", the old Step 3–4 order, the old Step 5
policy placement) and `ghsp` was addressed to the contribution-auditor,
not a governor Gate 1 request. **This handoff replaces `khm3` as the
binding Gate 1 request** and names the current exact plan head on MR
!2646.

## The plan the governor is being asked to approve

The corrected Step 2–7 procedure (all `rjqj` and the two `qbhj` blockers
resolved):

- **Step 2** — the exact `v2.11.0` Maintainers-only (L40) protected-tag
  rule is configured, API-read-back while the tag does not exist, and
  recorded verbatim **before** any versioned draft or namespace preflight;
  Step 5b is a byte-identical re-read.
- **Step 3** — two audit commands: tree-wide (with the append-only
  exclusions) **plus** `git grep -F '2.10.0' -- agf/learnings/foundry.yaml`
  for the live adoption `framework_version:` cell. Audit (a) is the Group B
  **completeness authority** — every version token it surfaces in the
  three status files is triaged A/B/C; a phrase grep is only a triage aid.
  The frozen-base triage must cover, at minimum, the named current cells
  (`manifest.md` ~18/~20, both roadmaps' Status + "Latest …" cells) and
  may only extend, never shrink. The #129 status-only correction is an
  enumerated cell set.
- **Step 4** — apply the allowlisted metadata to the working tree without
  committing; validate that dirty draft; record the tree OID reproducibly
  via a temp-index `git write-tree`; commit exactly those bytes; prove
  `HEAD^{tree}` equals that OID and the diff is allowlist-only; preflight
  the clean unpublished commit as the MR head. The acceptance job matrix
  is derived from current CI rules with pipeline `2829685860` as the
  regression baseline; tag-only/optional jobs are separated.
- **Step 5** — fail-closed pre-tag proof: four-namespace absence incl. the
  derived OCI reference; the Step 2 protected-tag re-read; frozen-SHA
  re-read + merge-commit parent/tree verify; runner/DinD capacity.
- **Step 6** — a separate post-merge governor decision authorizes the tag.
- **Step 7** — completion conditional on **both** CI `release:verify`
  **and** an operator `agov dist post-publication-check --releases-url
  <anon HTTPS> --download-dir "$(mktemp -d)"`, **plus** a separate explicit
  Git check that `v2.11.0^{commit}` equals the reviewed merge commit.

`agov plan validate` → OK (blocking). `git diff --check` → clean.
Exact-head pipeline for the reviewed head is green. No projection manifest.
Grants no version / metadata-merge / tag / release / deploy authority.

One `qbhj` blocker remains for a fresh exact-head review after the fix (the
Group B completeness rewrite) — `dg81` acks `qbhj` and the corrected head
should get a fresh exact-head contribution + InfoSec pass before the Gate 1
decision cites it.
