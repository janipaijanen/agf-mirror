---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260911T044714Z-planner-architect-pehr
timestamp: 2026-09-11T04:47:14Z
model: claude-sonnet-5
immutable: true
topic: quantized-model-mechanical-drafting-pilot-gate1
references-to:
  - 20260911T042239Z-planner-architect-pnqk.md
  - 20260911T043058Z-planner-architect-gmpm.md
requires-ack: true
artifacts_owned:
  - docs/plans/20260911T044602Z-quantized-model-mechanical-drafting-pilot.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on the quantized-model mechanical-drafting offline evaluation harness, on branch plan/quantized-model-mechanical-drafting-pilot.
    - Confirm the offline-only scope (zero live coordination records produced), Step 0's environment-interrogation-first requirement, the task-corpus design, the two-part scoring, and the fixed decision thresholds.
    - This is the bounded experiment named in the preliminary assessment (gmpm); it is independent of the RUW trigger and does not touch the row 4-7 slice work or queue-closeout.
  must_read:
    - docs/plans/20260911T044602Z-quantized-model-mechanical-drafting-pilot.md
    - docs/proposals/operating-model-token-and-footprint-preliminary-assessment.md
  do_not_touch:
    - live coordination channel (the harness writes only to gitignored tmp/)
    - any decision threshold, after results exist
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Handoff: quantized-model mechanical-drafting pilot (Gate 1)

The bounded, safe experiment named in the preliminary assessment
(`gmpm`, MR !2688): can a quantized, small, open-weight model — sized for
consumer-GPU-class hardware — reliably draft schema-valid *and*
semantically-correct coordination records for the mechanical/templated
task class, at meaningfully lower cost than the current frontier baseline?

**Offline evaluation harness only** — zero live coordination records
produced; outputs go to gitignored `tmp/quantized-pilot/`, never
`docs/coordination/`. Step 0 requires interrogating the actual target
environment for GPU/quantized-inference availability before choosing a
model — **none was found in this sandboxed development container**, so
that finding is already recorded as the first real result: infrastructure
may be the actual blocker, not model capability.

Decision thresholds (schema-pass ≥90%, semantic-correctness ≥95% among
schema-valid outputs) are fixed in the plan before any run, so they can't
be adjusted to fit disappointing numbers after the fact.

`agov plan validate` OK. Owners on approval: build-engineer (harness +
environment finding), capability-developer (capability read + report).
Any live pilot is a separate, later Gate 1 — this one produces findings
only.
