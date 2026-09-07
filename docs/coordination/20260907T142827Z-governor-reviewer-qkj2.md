---
kind: decision-record
from: governor-reviewer
to: all
id: 20260907T142827Z-governor-reviewer-qkj2
timestamp: 2026-09-07T14:28:27Z
model: human
immutable: true
references-to:
  - 20260907T140640Z-governor-reviewer-smem.md
  - 20260906T191236Z-governor-reviewer-trd6.md
  - 20260907T080622Z-capability-developer-c4mt.md
  - 20260907T142529Z-coder-tester-st75.md
topic: v210-propeller-stage4-activation-approved
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Stage 4 approved — Improvement Propeller activation

Issued as a binding in-session directive (2026-09-07, "stage4 approved.
proceed."), transcribed to coordination. The governor-reviewer approves the
Improvement Propeller **Stage 4 activation** for v2.10, as required by
`rhpb` and the approved Propeller plan
(`docs/plans/20260905T221252Z-improvement-propeller-role.md`).

## Sequencing (mandatory — activation execution is gated)

This approval is the activation **decision**. Activation **execution**
(flip `agf/agents/MANIFEST.yaml` `improvement-propeller` from `planned` to
`active`, create the real non-reentrant weekly wake mechanism, initialize
the ignored cursor state, run the first bounded cycle) MUST NOT begin
until:

1. Stage 3 (**MR !2508** @ `53d3ac49`) completes its independent
   whole-head review and its own binding **Gate 2**, and is **merged** —
   the manifest entry that activation flips lives inside that unit;
2. this activation decision is completed with the parameters the plan
   requires and a bare "approved" does not supply — the **named player**,
   the configured **evidence surfaces/paths**, the specific **real wake
   mechanism** (assistant/orchestrator scheduler, not `sleep`/cron/detached
   process), the **cadence**, and the **initialized cursor state**. Codex
   (RUW-3 lane) proposes these in a follow-up record for the governor to
   confirm before execution.

## Maturity honesty (from `c4mt` C-A1)

Release readiness requires the first bounded run to be a **genuine
scheduled or orchestrator wake-path execution** — a manual or simulated
invocation is not that proof. Capability maturity is **Level 5 Deployable**
only after that run succeeds (a successful bounded report *or* a truthful
quiet/no-material-change result). If the genuine wake-path proof is not
obtained in time, v2.10 ships describing Propeller at **Level 4 Validated**
with an explicit Gate 1 scope amendment and a named follow-up — the release
must never describe a manual test or inactive role as launched.

## Scope

This approves the Stage 4 activation decision only. It does **not** approve
Stage 3's Gate 2, and it grants no tag, release, or release-metadata
authority. Post-activation deactivation remains available per the plan
(pause the schedule, retain cursor/history; no role inherits Propeller
ownership).
