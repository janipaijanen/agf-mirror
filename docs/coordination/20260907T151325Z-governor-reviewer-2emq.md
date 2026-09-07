---
kind: decision-record
from: governor-reviewer
to: all
id: 20260907T151325Z-governor-reviewer-2emq
timestamp: 2026-09-07T15:13:25Z
model: human
immutable: true
references-to:
  - 20260907T144250Z-coder-tester-8dkp.md
  - 20260907T144254Z-coder-tester-gakp.md
  - 20260907T144721Z-coder-tester-pgm4.md
  - 20260907T142827Z-governor-reviewer-qkj2.md
  - 20260907T143711Z-contribution-auditor-dt4s.md
  - 20260907T145518Z-contribution-auditor-grd1.md
topic: v210-ruw3-a1-scope-stage4-params-ruw4-dispositions
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Dispositions — RUW3-A1 scope, Stage 4 parameters, RUW-4

Issued as a binding in-session directive (2026-09-07, "approved. proceed as
proposed"), transcribed to coordination. Dispositions the three Codex
handoffs `8dkp`, `gakp`, `pgm4`.

## 1. `8dkp` — RUW3-A1 scope: authorized

MR !2508 (RUW-3) **may add `README.md` as a 14th path**, restricted to
changing the single line "50 governance rules, 15 agent definitions" to
"51 governance rules, 16 agent definitions". **No other `README.md` edit.**
The `applicability-catalog.md` stays unchanged — `grd1` confirmed it
correctly counts 15 role definitions (51 rules + 15 roles = its stated 66)
while `readme_stats` separately counts `MANIFEST.example.yaml` as a
distribution artifact (hence 16).

The new !2508 head takes the complete local acceptance set, a fresh
canonical exact-head pipeline, and a **focused** independent rereview
before Gate 2 — the rereview confirms only the two README numerals and
that nothing else regressed; it does not re-audit the Stage 3 unit.

## 2. `gakp` — Stage 4 parameters confirmed; v2.10 ships Propeller at Level 4; genuine wake is a post-RUW follow-up

### 2a. Scope amendment (Gate 1) — Stage 4 *launch* is no longer mandatory-for-v2.10

Amending `rhpb` and the v2.10 plan's RUW-3 acceptance: **v2.10 ships the
Improvement Propeller Stage 3 unit at Level 4 Validated.** The genuine
scheduled/orchestrator first-wake execution that proves **Level 5
Deployable** is an explicit **named post-RUW (v2.11) follow-up**, not a
v2.10 release gate. This is the `qkj2` Level-4 path, chosen now
deliberately rather than as a timing fallback, so Stage 4 *execution*
leaves the v2.10 critical path entirely.

Requirements of this amendment:

- The v2.10 release changelog states Propeller as **Level 4 Validated**
  with the follow-up named — it must not describe an inactive role as
  launched or a manual run as the Level 5 proof.
- A tracked follow-up item (issue and/or plan) is filed for the post-tag
  Stage 4 activation + first-wake proof + maturity bump to Level 5.
- RUW-1, RUW-2 (done), RUW-4 (#120 child), the reversible preflight, the
  release-metadata MR, and the `v2.10.0` tag are unaffected — the release
  cut now gates only on those.

### 2b. Stage 4 parameters — confirmed and held ready for post-RUW execution

The proposed player (Codex playing only `improvement-propeller` on its
scheduled wake), evidence surfaces, weekly Monday 09:00 UTC cadence with
monthly/post-release friction synthesis, metadata-only
`tmp/codex-improvement-propeller-state.json`, and the baselines (62 valid
learnings through `20260906T104942Z-ev5m`; coordination cursor
`20260907T143711Z-contribution-auditor-dt4s`; release baseline `v2.9.0`
peeled `ee27978b218de47b062ab4ac2afee090b2c48ff8`) are **confirmed**,
subject to:

1. **Execution occurs post-tag**, and not before MR !2508 Gate 2 and
   merge. No manifest flip, heartbeat, or state initialization until then.
2. **The Codex-app heartbeat must be a real, observable wake mechanism**
   that actually fires on the stated cadence. The Level 5 maturity bump is
   recorded only after a genuine scheduled wake produces a conforming
   bounded report or truthful quiet result — a manual invocation is never
   that proof.
3. **Propeller proposals receive independent disposition** through
   planner-architect / capability-developer / contribution-auditor per the
   rule's routing. The Codex runtime that plays `coder-tester` does not
   implement a Propeller proposal outside the normal planner/gate path.

## 3. `pgm4` — RUW-4 #120 child Gate 1 plan: proceed

The handoff to planner-architect stands. Planner-architect claims and
authors the issue #120 child Gate 1 plan resolving the six enumerated
points; it takes the full Gate 1 path — `documentation-rule-projection-rule`
six-surface screen and a `gate-self-test-rule` negative test in the same
MR, plus independent contribution-auditor, infosec-sentinel, and
build-engineer reviews — then governor Gate 1. Codex retains the
post-Gate-1 implementation lane. RUW-4 is the release-cut critical path;
its pace, not RUW-3 or Stage 4, drives the v2.10 schedule.

## Scope

No gate is approved here. RUW-3 Stage 3 Gate 2, the #120 child Gate 1 and
its implementation Gate 2, Stage 4 execution, the release-metadata MR Gate
2, and the `v2.10.0` tag each still require their own binding governor
decision at the exact head.
