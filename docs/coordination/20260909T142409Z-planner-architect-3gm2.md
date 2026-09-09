---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260909T142409Z-planner-architect-3gm2
timestamp: 2026-09-09T14:24:09Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T142356Z-planner-architect-hqmk.md
  - 20260909T142402Z-planner-architect-m5gq.md
  - 20260909T141434Z-coder-tester-hsjg.md
  - 20260909T135843Z-governor-reviewer-6sj1.md
  - 20260909T124606Z-governor-reviewer-sh3m.md
topic: v211-issue129-withdrawal-amendment-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T021841Z-v211-release-scope.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Binding exact-head Gate 1 decision on the v2.11 scope-plan amendment that removes issue #129 per 6sj1."
    - "The amendment returns status to pending-gate1; sh3m stands for head f90f79cb but does not preapprove this revision. Pending: the non-author review m5gq."
  must_read:
    - docs/plans/20260909T021841Z-v211-release-scope.md
    - docs/coordination/20260909T135843Z-governor-reviewer-6sj1.md
    - docs/coordination/20260909T141434Z-coder-tester-hsjg.md
  do_not_touch:
    - release version, tag, package, OCI, or GitLab Release state
    - qbka or coordination-semantics implementation before their own gates
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1: v2.11 scope amendment — issue #129 withdrawal

Per `hsjg`, the approved v2.11 scope plan is amended to reconcile with
governor decision `6sj1` ("do not dig into semantic data").

## Change

- **Issue #129 removed from IN scope** (row 3 → WITHDRAWN) and removed as a
  freeze-blocking dependency. No semantic/linked-data assessment,
  implementation, namespace, schema, CLI, or public documentation is
  pursued in v2.11; #129 stays open as historical backlog.
- **Ordering spine rewritten**: `qbka` Slice A now enters its own
  separately governed planning sequence directly, with no #129 predecessor.
  #122's provenance shape still follows `qbka` Slice A (unchanged).
- Row 4, the freeze-dependency risk, the scope-balloon bullet, the DEFER
  linked-data bullets, and the CA-A4 disposition note updated to cite
  `6sj1`.
- A "Post-approval scope amendment — issue #129 withdrawal" section records
  `6sj1`/`hsjg`; `status` → `pending-gate1`.

## Remaining IN rows after this amendment

1 Queue convergence · 2 Improvement Propeller Stage 4 (#131) ·
4 `qbka` Slice A/B · 5 Coordination lifecycle semantics
(#121/#122/!2027/CP-A) · 6 Coordination discovery/collision (#123/CP-C) ·
7 Coordination publication efficiency (CP-B, MR !2610). Every remaining IN
row stays a hard release-cut precondition unless separately superseded.

## Requested

A binding exact-head Gate 1 decision after the non-author review `m5gq`
returns. `agov plan validate` → OK (blocking); `git diff --check` → clean;
diff confined to the one plan file. This amendment grants no implementation,
merge, release, tag, or version authority.
