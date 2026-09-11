---
kind: decision-record
from: planner-architect
to: all
id: 20260911T042239Z-planner-architect-pnqk
timestamp: 2026-09-11T04:22:39Z
model: claude-sonnet-5
immutable: true
topic: post-ruw-post-queue-closeout-operating-model-efficiency-assessment
references-to:
  - 20260909T195116Z-post-v211-queue-closeout-work-order
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

<!--
Bridge for a governor directive delivered off-channel (2026-09-11, to
Codex; relayed to this session). Per the #122 lesson (off-channel
instructions must be captured on-channel or they are invisible to
`agov coord list`/`agov coord pending`), this record is that capture. It
establishes a future-triggered obligation; it authorizes no work now.
-->

# Governor directive: post-RUW, post-queue-closeout operating-model efficiency assessment

## Decision (governor, 2026-09-11)

> "when RUW is out AND MRs are closed, then assess the operating model,
> its details and implementation on how to minimize it to a) minimize
> token use on services like claude, codex and friends, and b) minimize
> overall footprint so much that it could be run on consumer GPUs and
> efficient, perhaps quantitized, models."

## Trigger (both conditions required)

1. **v2.11 (RUW) published and verified** — the tag pushed, `agov dist
   post-publication-check` passing, per the release-publication plan's
   Step 7 completion criteria.
2. **MRs closed** — the queue-closeout work order
   (`docs/plans/20260909T195116Z-post-v211-queue-closeout-work-order.md`)
   has executed its QC-4 batches to the governor's satisfaction (not
   necessarily zero open MRs — "closed" per that work order's own
   terminal-disposition sense, at the governor's call when the trigger
   fires).

Neither condition is met yet. This record does not authorize any
assessment work now; it exists so the directive is not lost between now
and then.

## Scope (as directed, to be detailed at trigger time)

An assessment of AGF-NG's own **operating model** — the governance
process (Gate 1/2, the nine-kind coordination protocol, the polling and
autonomous-continuation loop, multi-round independent review) **and its
implementation** (`agov` tooling, the coordination/plan/learning schemas,
the coordination-channel file volume) — against two efficiency axes:

- **(a) Token economy** — how much of the current model's LLM-token
  consumption (Claude, Codex, and similar services) is structural
  overhead (coordination-record ceremony, multi-round review, cold-start
  context) versus load-bearing governance value, and what could shrink
  without weakening the guarantees (independence, audit trail,
  irreversibility gates).
- **(b) Footprint minimization** — whether the roles and workflow could
  run on materially smaller, potentially quantized, models on consumer
  GPU hardware instead of large cloud-hosted models, and what capability
  gap (context window, instruction-following reliability, tool-use
  fidelity) that would expose.

## Routing (at trigger time, not now)

Best-fit owner: **capability-developer** ("evidence-triggered agent
evolution, maturity, phase, and boundary proposals" — this is exactly
that mandate, applied to the framework's own operating cost rather than a
single role). Advisory only, per that role's standing charter: it
proposes, the governor disposes. `documentation-estate-steward` may
contribute an estate-wide view of coordination-record volume/redundancy
as one input. This record does not activate either role for this task —
that activation is the governor's call when the trigger fires.

## Nothing implied

This is a **future-triggered obligation**, not a v2.11 scope item, not a
precondition, and not a redirection of any current workstream. It does
not affect the row 4-7 slice plans, the queue-closeout execution, or any
in-progress Codex work.
