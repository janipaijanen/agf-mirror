---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260908T185809Z-planner-architect-pe1k
timestamp: 2026-09-08T18:58:09Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260908T185629Z-governor-reviewer-j2gs.md
  - 20260908T183242Z-planner-architect-ds4k.md
  - 20260903T221643Z-coder-tester-qbka.md
  - 20260903T215909Z-coder-tester-67kq.md
topic: qbka-lifecycle-work-order-planner-allocation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Allocate the qbka plan-status-lifecycle Gate 1 work order to a planner session other than the one that authored ds4k (fresh Claude planner, or Codex as planner-architect under explicit temporary allocation). ds4k Finding-1 seam rule and Finding-4 #129 sequence are binding inputs."
  must_read:
    - docs/coordination/20260903T221643Z-coder-tester-qbka.md
    - docs/coordination/20260903T215909Z-coder-tester-67kq.md
    - docs/coordination/20260908T183242Z-planner-architect-ds4k.md
  do_not_touch:
    - "agf/schemas/gate1-plan.schema.json, the 12 invalid legacy plan files, agf/rules/, .gitlab-ci.yml — not authorized until this work order has its own binding Gate 1"
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# qbka lifecycle work order — needs its own planner and its own Gate 1

`j2gs` confirmed `ds4k`, including the recommendation that the qbka work
order go to a **separate planner session**. This handoff records that
routing request; the ds4k-author session does not carry it.

## Scope the allocated planner must deliver (one Gate 1 work order, two atomic slices)

Per `67kq`, independently re-challenged:

- **Slice A** — a provenance-bound `superseded` plan lifecycle state:
  `agf/schemas/gate1-plan.schema.json` enum + required provenance fields
  (replacement identifier, authorizing governor decision, UTC disposition
  timestamp), the Rust `agov plan validate` fixtures + negative tests
  rejecting missing/empty provenance, defined transition semantics among
  `pending-gate1 / approved / active / complete / superseded`, and
  queue-consumer behaviour (both `complete` and `superseded` terminal;
  `superseded` shows the replacement link).
- **Slice B** — the exact twelve-file migration (`67kq` enumerates them:
  status anomalies plus the four frontmatter-less historical files whose
  full envelope must be reconstructed from binding coordination/commit
  evidence, not body-text labels) plus an all-corpus `agov plan validate`
  test.

Land atomically or ordered so that no supported revision accepts a new
state without its provenance contract and none claims corpus validity
before all twelve files pass.

## Binding inputs from `ds4k`

1. **Seam with queue-convergence** — when a queue-convergence ledger row
   dispositions a *plan file* as `close-superseded`, it must cite the same
   replacement identifier + governor decision this Slice A contract
   requires. The work order defines that as an explicit consumer contract;
   queue-convergence text does not change.
2. **#129 gate** — issue #129's linked-data / evidence-model assessment
   must complete and be reviewed **before** Slice A's provenance fields
   are frozen, so any evidence-model conclusion is designed in.
3. **#122 alignment** — issue #122 (coordination-record supersession)
   should adopt this Slice A provenance shape; the two "supersession
   announces itself" mechanisms stay one design, applied to two artifact
   types.

## Gate class

Schema + validator + gate change. `gate-self-test-rule` negative tests
mandatory. Not eligible for research-maintenance or trivial-data fast
paths. Full Gate 1, then per-slice Gate 2.
