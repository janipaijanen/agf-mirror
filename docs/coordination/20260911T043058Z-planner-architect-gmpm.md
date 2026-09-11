---
kind: note
from: planner-architect
to: all
id: 20260911T043058Z-planner-architect-gmpm
timestamp: 2026-09-11T04:30:58Z
model: claude-sonnet-5
immutable: true
topic: post-ruw-efficiency-assessment-early-start-preliminary-pass
references-to:
  - 20260911T042239Z-planner-architect-pnqk.md
requires-ack: true
artifacts_owned:
  - docs/proposals/operating-model-token-and-footprint-preliminary-assessment.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Preliminary pass started (governor: "you could progress it now")

`pnqk` recorded the post-RUW/post-queue-closeout operating-model
efficiency directive as future-triggered, no work authorized yet. The
governor then authorized starting during the current pause (both AI
sessions were token-constrained; nothing else was moving).

This is a **preliminary pass only** — evidence-gathering and reasoning
from what's already observable, not the full assessment `pnqk` describes
(which still needs v2.11 published/verified and the queue-closeout
executed, so a *finished* workflow is what gets measured, not a
mid-flight one).

Report: `docs/proposals/operating-model-token-and-footprint-preliminary-assessment.md`.

## Summary

- **Token economy**: concrete evidence this session — cold-start fixed
  cost (`AGENTS.md` 581 lines + ~10,700 lines of rules), 3,789
  coordination files and growing, a 5-round independent-review case study
  (the QC-1 ledger), and a ~10-record bridge-record overhead case study
  (the `POLL-G1` pattern). `agov`'s deterministic validators already carry
  the purely mechanical checks; remaining ceremony is largely in the
  coordination/review *protocol*, not individual tasks.
- **Footprint**: three capability tiers observed (mechanical/deterministic
  — already zero-LLM; templated/mechanically-verifiable drafting — a
  plausible small/quantized-model target since `agov coord validate` is
  an existing zero-cost safety net; judgment-heavy cross-cutting review —
  no evidence yet a smaller model holds up, an open question not a
  settled negative).
- **One bounded, safe experiment proposed**, runnable independent of the
  RUW trigger: a quantized/small-model pilot restricted to mechanical
  coordination-record drafting, measured against `agov` pass/fail and
  token cost versus a frontier baseline.
- **Nothing concluded** about removing/weakening any role, rule, or gate.
  Any real change stays a full Gate 1 decision under `gate-self-test-rule`.

This does not touch or redirect the row 4-7 slice work, queue-closeout,
or any Codex in-progress branch.
