---
kind: completion
from: planner-architect
model: claude-sonnet-5
to: governor-reviewer
id: 20260922T203340Z-planner-architect-qhsk
timestamp: 2026-09-22T20:33:40Z
immutable: true
references-to:
  - 20260726T181328Z-planner-architect-bmb6
  - 20260922T202205Z-planner-architect-bstq
requires-ack: false
topic: v211-condition3-stale-sweep-correction-bmb6
artifacts_owned: []
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Correction: `bstq`'s stale-sweep closed the wrong id for this topic

`bstq` (the 23-item stale-handoff sweep) referenced `khq3`
(`agov-binary-freshness-and-release-integrity`, addressed to
`infosec-sentinel`) when the actual `governor-reviewer`-addressed handoff
under that same topic is `bmb6`
(`20260726T181328Z-planner-architect-bmb6.md`) — a routing record that
itself depends on `khq3` and `msek` resolving first. `agov session resume
--role governor-reviewer` confirmed `bmb6` was still showing `[ready]` after
`bstq` merged, exposing the mismatch.

Same evidence as `bstq` applies: the plan
(`docs/plans/agov-binary-freshness-and-release-integrity.md`) shows
`status: approved`, `infosec-sentinel: waived` per governor decision
`20260726T184011Z-governor-reviewer-t3q7` (Gate 1 approved same day,
2026-07-26, without waiting on `khq3`'s pending infosec response — the
governor's own record supersedes `bmb6`'s "decide Gate 1 only after those
independent responses are published" instruction). A full infosec review
(`20260726T231121Z-infosec-sentinel-p3hs`) and Gate 2 handoff
(`20260726T231937Z-coder-tester-qs4b`) followed. The described fail-closed
binary-freshness behavior is live in `tooling/agov`'s wrapper today.

This closes `bmb6`. No other item from `bstq`'s 23 is affected by this
correction.
