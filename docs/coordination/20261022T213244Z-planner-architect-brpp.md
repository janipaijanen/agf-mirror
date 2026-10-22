---
kind: note
from: planner-architect
to: governor-reviewer,coder-tester
id: 20261022T213244Z-planner-architect-brpp
timestamp: 2026-10-22T21:32:44Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261022T212922Z-planner-architect-rhqs
  - 20261022T212401Z-coder-tester-epbp
topic: bridge-live-gate1-approval-and-merge-3118
requires-ack: true
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3114
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3118
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Bridge record: live Gate 1 approval for the instruction-context plan and merge of !3118

This is a bridge record. It transcribes decisions the governor gave in a live
session. It is provisional and is not binding authority on its own until a
`governor-reviewer` record (`ack` or `decision-record`) references it.

## What the governor said (live session, 2026-10-22)

"yes merge !3118. approved gate1."

The message answered planner-architect's question whether to merge !3118 (bridge
note `rhqs` for the advisory-row waiver) and whether the governor wanted to
decide Gate 1 on MR !3114. It came from the interactive session user, whom this
project identifies as the governor-reviewer.

## Candidate for Gate 1

Plan `docs/plans/20261022T205113Z-instruction-context-reduction.md` on branch
`plan/instruction-context-reduction`, MR !3114, issue #138. Head named by this
record: `3bfa9603ac30edc779263438ef6e0ca091cd1188`.
Scope as in the plan: Task 0 (done), foundry-local Phase 0 (`claudeMdExcludes`
plus an INDEX) and Phase 1 (slim `AGENTS.md`, with Codex trigger lines). No
shipped `agf/rules/**` change and no rule-text compression. Measured target: at
most 140.0k startup-report chars. Advisory rows: both waived (note `rhqs`).
Codex review `epbp` blocked the earlier text; Codex re-review `1mkq` gave CLEAR
at head `1a7a98c1`. Head `3bfa9603ac30edc779263438ef6e0ca091cd1188` differs from that head only in the two waived
advisory rows (Codex confirmation was requested in the room).

Codex advisory carried into implementation: the Phase 0 `InstructionsLoaded` log
is a fail-closed checkpoint. Phase 1 must not start unless the exclusions are
proven on the installed Claude Code version.

## Actions

1. Planner-architect merges !3118 (this note and `rhqs`) with the exact-SHA
   guard once its pipeline passes. This record is published inside !3118, so it is
   visible before that merge executes.
2. The plan `status`, `gate1_approved_by` and `gate1_approved_at` fields stay as
   they are (`pending-gate1`). They change only after a `governor-reviewer`
   record exists that references this note. No implementation starts before then.

## Requested follow-up

A `governor-reviewer` `decision-record` that references this note and records
Gate 1 approval of the plan at head `3bfa9603ac30edc779263438ef6e0ca091cd1188`. Per the standing instruction, Codex may
file it on the governor's behalf. The plan is red-listed under delegation
artifact `qpda`, so no delegated joint decision may stand in for it.
