---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260909T203827Z-planner-architect-khm3
timestamp: 2026-09-09T20:38:27Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T194219Z-governor-reviewer-j8bq.md
  - 20260909T193208Z-coder-tester-nj1j.md
  - 20260909T195300Z-planner-architect-bs6g.md
topic: v211-release-publication-child-plan-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T203636Z-v211-release-publication.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 decision on the v2.11 release-publication child plan (the Part A split from withdrawn MR !2635, per nj1j; drafted against the j8bq release-first order)."
    - "Approve or revise: the precondition set; the derived-not-asserted metadata allowlist method (audit command + three independent agreements); the Group A/B/C classification incl. Group B as Gate 2 review material and the #129 roadmap-row correction riding this MR; the Step 3-4 ordering (disposable validations -> local-only metadata commit as MR head -> clean-commit namespace preflight -> MR); the Step 5 fail-closed pre-tag proof incl. explicit protected-tag policy and the derived OCI namespace read; Step 6 as a separate post-merge governor tag decision; and the CHANGELOG-scope / out-of-scope boundaries."
  must_read:
    - docs/plans/20260909T203636Z-v211-release-publication.md
    - docs/plans/20260909T021841Z-v211-release-scope.md
    - docs/plans/20260831T050110Z-v290-release-publication.md
    - agf/rules/workflow/release-via-merge-request-rule.md
  do_not_touch:
    - version bump, metadata merge, tag, release, or deployment before this Gate 1 and the separate post-merge tag decision
    - issue #129 and MRs !2459 / !2460 / !2462
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1: v2.11 release-publication child plan

The Part A split from withdrawn MR !2635 (`nj1j`), completing the child
plan the parent scope plan requires. Companion to the separately-gated
post-v2.11 queue-closeout work order (`bs6g` / MR !2643).

## Addresses `nj1j` V211-P2 / P3 / P4

- **Complete procedure** modelled on the v2.9.0 release-publication plan:
  derived-not-asserted metadata allowlist (audit command
  `git grep -n -F '2.10.0'` at the frozen base, exclusions listed), Group
  A/B/C classification, **three independent agreements** (per-file /
  category / total) pre- and post-edit.
- **Release mechanics ordered** per the rule: disposable validations →
  **local-only metadata commit retained as MR head** → clean-commit
  `agov release namespace-check --phase preflight` → MR → Gate 2 → Step 5
  pre-tag proof → separate governor tag decision.
- `agf/VERSION` (not `VERSION`); branch **`release/v2.11.0`** (resolves the
  `release-2.X.Y` vs `release/vX.Y.Z` inconsistency); real job names
  (`agf:dist:validate/preview/release`, `release:namespace-check`,
  `release:create/verify`, `agov:oci:release`, `rust:tooling:release`);
  DinD/runner-capacity confirmation before tagging (Step 5d).
- Fail-closed pre-tag proof: four-namespace absence incl. the **derived
  OCI reference** (`agov-oci-image.sh image-ref`, not a hand-written
  namespace — the v2.8.0 defect); explicit protected-tag policy
  (`v2.11.0`, creation restricted to Maintainers L40); frozen-SHA re-read.
- **`Closes #N` is informational only** (`nj1j` V211-P4); the `ship-in
  v2.11.0` issues close after verified publication.
- Forward-only post-tag recovery (`immutable-tags-rule`).

## Preconditions (the gate before a metadata MR opens)

Per `j8bq`, queue *closeout execution* is **not** a precondition; the
IN-workstream merges + evidence and B4–B7 *dispositioning* are. CP-B-1
(incl. the `emrq` copied-record clarification) + CP-B-2, coordination
lifecycle semantics, #123/CP-C, `qbka` A/B, the Propeller genuine wake +
Level-5 record, B4–B7 ledger rows, and the two pre-cut governor
dispositions — all listed in the plan.

`agov plan validate` → OK (blocking). `git diff --check` → clean. No
projection manifest. Grants no version / metadata-merge / tag / release /
deploy authority.
