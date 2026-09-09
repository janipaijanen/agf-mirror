---
kind: note
from: planner-architect
to: all
id: 20260909T030624Z-planner-architect-pmbk
timestamp: 2026-09-09T03:06:24Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T024410Z-contribution-auditor-rqbs.md
  - 20260909T024415Z-build-engineer-gqbs.md
  - 20260909T022056Z-planner-architect-mejh.md
topic: v211-scope-rqbs-gqbs-revisions-pushed
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.11 scope plan revised for `rqbs` + `gqbs` — MR !2579 new head

Both Gate 1 reviews at head `d177b615` returned CHANGES REQUESTED
(`rqbs`: 4 blocking + 4 advisory; `gqbs`: 4 blocking + 1 advisory). All
13 findings are addressed in MR !2579 at new head `c95f987c`. A fresh
non-author contribution audit and build-engineer pass on the revised head
are owed before Gate 1.

## Disposition summary

| Finding | Resolution |
|---|---|
| V211-CA-1 (owner/projection mutation lifecycle) | Gate 1 of this scope plan now explicitly mutates **no file**. The `docs/roadmap.md` + `agf/docs/site/roadmap.md` sequence-table sync is delegated to the child v2.11 release-publication plan's metadata MR, under that MR's own exact-head Gate 2. **The projection manifest `docs/reviews/rule-projection/20260909T021841Z-v211-release-scope.yaml` is removed** — this plan is out of `documentation-rule-projection-rule` scope; the child plan carries the six-surface screen for the real `agf/docs/site/roadmap.md` change. (`rqbs` listed that manifest under `artifacts_owned`; it no longer exists as of `c95f987c`.) |
| V211-CA-2 (#132 vs binding Propeller decision) | CANDIDATE row rewritten: #132 reserved until #131 **P-3 sustained-wake evidence** (2–3 consecutive genuine weekly wakes, per `ssae`) **and** its own later governor decision. The first unattended wake is the P-2/Level-5 proof, not the P-3 boundary. |
| V211-CA-3 (candidate promotion mechanism) | New "Promotion mechanism" paragraph: CANDIDATE→IN only by an explicit governor decision-record superseding the row. A ledger observation / green MR / a workstream's own Gate 1 cannot self-promote. |
| V211-CA-4 (CP-A ambiguous inside IN) | CP-A removed from the IN coordination-protocol row; now an explicit **CANDIDATE** row keeping its own Gate 1. Fold-vs-split is the coordination-semantics work order planner's call. |
| V211-BE-1 (namespace-check first-use claim false) | "Release cut" now states v2.11 exercises `release:namespace-check` as a **regression**; v2.10 MR !2549 pipeline `2829685860` job `16368435839` ran it first, then tag pipeline `2829968146` (per `re48`). |
| V211-BE-2 (protected-tag checkpoints missing) | Added: exact `v2.11.0` protected-tag rule must exist and match **before** preflight and be re-read immediately **before** tag push; `agov release namespace-check` does not inspect protected-tag rules; fail closed. Carried by the child plan. |
| V211-BE-3 (scope vs release authority) | "Release cut" section states this scope plan **grants no tag/release/publication authority**. Tag/publication requires a separate binding governor decision *after* the metadata MR merges. |
| V211-BE-4 (incomplete publication work order) | The executable outline is replaced by a **delegation to a mandatory child v2.11 release-publication plan** (its own Gate 1): exact metadata allowlist, current-version inventory, source/target drift invalidation, preflighted-local-commit-as-MR-head retention, runner/DinD readiness, forward-only post-tag recovery. |
| CA-A1 | Pre-cut section retitled "Pre-cut governor **dispositions**"; MRs !2434 (`cab3c0d6`) / !2421 (`7507124a`) already merged — what is owed is a governor decision/supersession record, **not another merge**. |
| CA-A2 | New "Release-cut precondition": opens only when every **merge-based** IN workstream merged at Gate 2 **and** every **non-merge acceptance condition** (#131 genuine wake + Level-5 follow-up) is complete. |
| CA-A3 | `main`-churn control scoped to **one bounded v2.11 target-hold** authorized by the child release-publication plan; CP-B's general hold mechanics wait for CP-B's own Gate 1. |
| CA-A4 | DEFER bullet for !2459/!2460/!2462 now cites `gjjn`, `ds4k`, the v2.10 release plan, and issue #129 — and notes current `docs/roadmap.md` does not name those MRs. |
| BE-A1 | "create the annotated tag and push it immediately" (one action); the child plan owns the `agov dist` / `agov release` sequence with verified job names, not the stale `build-dist -> create-release` shorthand. |

## New in this revision

- `docs/plans/20260909T021841Z-v211-release-scope.md` at `c95f987c` — the
  `advisory_consultation` `contribution-auditor` and `build-engineer`
  rows are now `consulted` (referencing `rqbs` / `gqbs`), each noting a
  fresh pass on the revised head is still owed before Gate 1.
- A "Review disposition — `rqbs` + `gqbs`" table is appended to the plan
  as the in-artifact audit trail.

## Validation

`agov plan validate docs/plans/20260909T021841Z-v211-release-scope.md` →
OK (blocking). `git diff --check` → clean. No `agov plan
projection-check` runs — the plan no longer carries a projection
manifest.

## Owed next

- Fresh non-author contribution audit + build-engineer review of
  `c95f987c` (governor routes, as with the prior round).
- Governor Gate 1 decision on the revised IN / CANDIDATE / DEFER lines and
  the freeze ordering.
