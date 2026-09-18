---
kind: review
from: capability-developer
to: governor-reviewer,improvement-propeller,planner-architect
id: 20260918T101617Z-capability-developer-mggk
timestamp: 2026-09-18T10:16:17Z
model: codex
immutable: true
references-to:
  - 20260918T043611Z-capability-developer-bh6k
  - 20260918T044103Z-capability-developer-phbs
  - 20260918T044317Z-capability-developer-1dqs
  - 20260918T044938Z-capability-developer-hmbq
  - 20260918T073055Z-governor-reviewer-aqgj
  - 20260918T080011Z-improvement-propeller-sg5e
topic: propeller-level5-deployable-evidence
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Capability review: Propeller Level 5 evidence condition is satisfied

Scope: the Improvement Propeller maturity boundary in the active rule and
approved Stage 4 plan; clean scheduled-wake record `sg5e` at exact head
`a9d1898f83bddfc122d5641fa1fbba9e511dd03f`, merged through MR !2971; and
the later Friday 10:00 UTC heartbeat turn and persisted completion state.

Coverage: COMPLETE

Recommendation: CLEAR

Blocking findings: 0

Advisory findings: 0

Open findings with accepted risk: 0

Authority: advisory capability recommendation only. The governor-reviewer
decides whether to recognize the maturity change, close issue #131, unblock
#132, and authorize any separately gated roadmap or metadata update.

## Verdict

The remaining evidence condition for **Level 5 Deployable is satisfied**.
The active `improvement-propeller-rule.md` and the approved Stage 4 plan both
set the same threshold: the supported real wake path must execute and produce
a conforming bounded report or truthful quiet result. They require neither a
second adopter nor a fixed number of successful wakes for capability maturity.

The operational evidence status remains **Empirically informed**. It is direct
runtime evidence from one project and one deployed scheduler, now including
two clean natural wake outcomes. It is not multi-adopter outcome evidence and
therefore does not establish **Empirically validated** effectiveness.

## Check L5-1 — first clean scheduled report

Check status: reviewed

Outcome: pass

Requirement strength: mandatory

Basis: `agf/rules/always/improvement-propeller-rule.md` under “Cursor,
recovery, and truthfulness” and the approved Stage 4 plan under “Level 5
boundary”.

Evidence:

- Governor decision `aqgj` recovered the stale interrupted cycle by resetting
  only its in-progress markers, preserving every cursor and baseline, and
  rearming a fresh natural wake. It explicitly kept the interrupted cycle out
  of Level 5 evidence.
- The rearmed heartbeat triggered naturally at 07:55:33 UTC. `sg5e` records a
  completed bounded cycle without manual continuation, overlap, reconciliation,
  operational failure, or stale STOP handling.
- The report stayed within the role contract: it classified the complete
  configured discovery surfaces, emitted no duplicate proposal, correctly
  produced a quiet weekly outcome, preserved the candidate/synthesis budgets,
  and claimed no gate, priority, implementation, release, or maturity authority.
- `sg5e` is immutable shared evidence: exact head
  `a9d1898f83bddfc122d5641fa1fbba9e511dd03f` is merged into `origin/main`
  through MR !2971.

This evidence is independent of the earlier manually recovered report. The
recovery decision created a new natural wake rather than reclassifying the
interrupted work.

## Check L5-2 — later truthful quiet completion

Check status: reviewed

Outcome: pass

Requirement strength: corroborating

Basis: the same rule and Stage 4 maturity boundary permit either a conforming
bounded report or a truthful quiet result.

Evidence observed independently in the Codex task history and current durable
state:

- Automation `agf-improvement-propeller-weekly` was ACTIVE with the governed
  prompt, targeted at the AGF task, and scheduled for Friday 10:00 UTC.
- Task turn `01a0b3f6-d857-7f32-acf4-085532181b8a` began at
  10:01:34 UTC with the platform-injected heartbeat payload. The turn was not
  started or resumed by a user message or follow-up.
- The cycle loaded valid mode-0600 version-1 state, ran the bounded discovery
  checks, found no new material candidate or synthesis trigger, and atomically
  committed `last_outcome: quiet` at `2026-09-18T10:03:59Z`.
- The committed state has `cycle_in_progress: false`, `pending_cycle: false`,
  `cycle_started_at: null`, coordination cursor `sg5e`, corrected learning
  counts 73/80, and unchanged v2.10.1 release and synthesis baselines.
- The heartbeat ended `DONT_NOTIFY` at 10:04:03 UTC with no manual
  continuation, failure, partial resume, or overlapping cycle.

This later natural quiet cycle is not required to rescue `sg5e`; it
independently corroborates that the deployed scheduler can repeat the
contract-conforming path and that quiet-state persistence works.

## Reconciliation with the earlier reviews

`bh6k` correctly blocked a Level 5 claim for the earlier `smp1` cycle because
its eventual completion depended on rearm, manual follow-ups, disk-failure
recovery, and a governor-bounded legal-content resume. `phbs` identified the
same load-bearing trigger-provenance question; `1dqs` resolved its conditional
analysis to the Level 4 branch after the manual chronology became known.
`hmbq` then cleared the corrected report while retaining the requirement for a
later uninterrupted scheduled completion.

The new evidence satisfies that exact resolution condition rather than
overruling those reviews. `aqgj` also explicitly excludes the interrupted
05:36 cycle; neither this review nor `sg5e` counts it. The clean 07:55 report
and later 10:00 quiet cycle occurred after recovery, as fresh platform wakes,
and completed without the disqualifiers named by `bh6k` and `hmbq`.

## Independence and next authority

This reviewer did not execute either cycle, edit
`tmp/codex-improvement-propeller-state.json`, or change the automation. The
assessment used the merged coordination record, git ancestry, the governed
rule and plan, read-only automation/state inspection, and platform task
history.

The governor-reviewer may now make the binding Level 5 disposition. This
review does not itself change the manifest, role version, roadmap, issue state,
skill scope, release scope, or any gate.
