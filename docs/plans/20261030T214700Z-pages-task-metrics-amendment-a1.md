---
id: 20261030T214700Z-pages-task-metrics-amendment-a1
status: pending-gate1
topic: pages-task-metrics-amendment-a1
affected_paths:
  - docs/plans/20261030T214700Z-pages-task-metrics-amendment-a1.md
  - docs/reviews/pages-structure-and-voice/20261030-task-metrics-baseline.md
depends_on:
  - 20261030T163253Z-pages-structure-and-voice
advisory_consultation:
  - role: contribution-auditor
    status: pending
    reason: Re-review requested after the revision answering decision record pg2s.
  - role: infosec-sentinel
    status: pending
    reason: Re-review requested. The amendment adds no data collection, and unfamiliar-participant research is excluded.
  - role: web-ux-reviewer
    status: pending
    reason: Re-review requested for the answer keys, the counting rules, and the rendered desktop and mobile check.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Amendment A1: task metrics for the Pages structure plan (proposed)

Governance mode: STRICT, because it would change acceptance criteria of an approved
plan. Status is `pending-gate1`. Nothing here is approved.

This is a separate plan file on purpose. The approved plan
`20261030T163253Z-pages-structure-and-voice` stays byte-for-byte unchanged and keeps
`status: approved` for its own criteria. A machine or an agent that reads the parent
therefore cannot read its approval as approval of this amendment. The parent gains
criterion 10 only when a `governor-reviewer` record approves this file at Gate 1 and
names it. Until then, parent criteria 1 to 9 are the only acceptance criteria.

Claim: `20261030T210529Z-planner-architect-p214`. Handoffs: `3ghb`, `kmhn`. Findings
answered: `20261030T212253Z-governor-reviewer-pg2s` (section "Combined planning review").
Baseline evidence: `docs/reviews/pages-structure-and-voice/20261030-task-metrics-baseline.md`.

## What changes if approved

Add one criterion to the parent plan's "Acceptance evidence":

> 10. Task metrics. For the three rows in Amendment A1, record before and after values with
> the A1 counting rules and the A1 non-regression rule. Record the independent reviewer's
> check of each answer key.

Section labels, the six slices, URL compatibility, the two-selection landing ceiling
(parent criterion 2) and the independent-review limits are unchanged. Reader jobs map to
the approved sections: Evaluate is Decide, Adopt is Get started, Operate is Work with AGF.

## Counting rules

The baseline artifact defines extraction, normalization and section ranges, and ships the
counting script with its checksum. Anyone can recount from the rendered HTML.

- Link transitions: followed links between pages from Home to the endpoint page. Menu
  expansion, search and mobile menu friction are recorded as notes and are not counted.
- Required reading: whitespace-separated words of the named ranges in the rendered article,
  split into the Home component, the destination subtotal and the total. Text in `<pre>`
  (assistant-directed payload or code) is reported separately and is not required human
  reading.
- Non-selected options: the options offered at a decision point that the scenario does not
  select. Many are legitimate routes for other audiences or situations. The count is
  recorded, and removing an option is not by itself an improvement.

## Task rows (before values from the baseline, rendered build `4076884d`)

| Job | Scenario | Start | Checkable end (answer key) | Link transitions | Required reading, words (Home + destination = total) | Non-selected options |
|---|---|---|---|---|---|---|
| Evaluate | A technical lead on a small team with one assistant decides adopt, wait or skip. | Home | The reader gives one outcome with its reason from the "Adopt now if", "Wait if" and "Skip if" lists, and names the governor's per-change and ongoing obligations from "What adoption asks of you". Answer key: those sections of the Decide page. | 1 | 312 + 670 = **982** | 3 on Home |
| Adopt | A solo maintainer with one assistant and an existing repository wants the minimum setup. | Home | The reader says: choose "existing project"; download the pinned `agf-<version>` archive from the GitLab Releases page and verify it; the `agov` archive is optional; paste the one commissioning prompt into the assistant; review the report. Answer key: Overview and Steps 1 to 3 of the Set up page. | 1 | 312 + 768 = **1,080**, plus 476 payload words | 3 on Home |
| Operate | A maintainer's Gate 1 plan for a non-trivial change was approved yesterday. They start work today. | Home | In order, the reader names: read the channel, file a claim before working, no new plan is needed because Gate 1 is approved, implement on a branch with self-review, Gate 2 before merge, and the completion record that closes the loop. Answer key: the numbered steps in "The daily shape" on the Day-to-day page, reached through the runbook row "Beginning ordinary governed work". | 2 | 312 + 862 = **1,174** | 3 on Home + 11 in the runbook table |

These are answerability endpoints. They do not show a completed installation or an approved
and merged change. The Adopt row names a file and an artifact because that page requires
them in this scenario. Optional CLI and MCP tooling stay optional and are not an endpoint.

## Targets and non-regression rule

Targets are rules, not invented numbers.

1. Link transitions: after is at most before (1, 1, 2). The approved two-selection ceiling
   stays. A threshold of three links never authorizes a regression from these baselines.
2. Required reading: after total is at most before total in every row (982, 1,080, 1,174).
   Improvement goal: the total falls in at least two of the three rows. The goal is met only
   if the independent reviewer confirms that every answer-key item is still present and
   findable. Words moved out of the named ranges do not count as saved unless the reader no
   longer needs them to reach the endpoint.
3. Non-selected options: recorded for information. There is no reduction target. A route
   removed from Home or the runbook must still have a reachable home, and the plan records
   where. Genuine governance gates are never counted as choices to remove.
4. If a row misses a rule, the implementer reports the miss and the reason. The reviewer or
   the governor decides. Tools do not approve.

## Not claimed and not included

- The five-minute figure stays an unvalidated future usability target. It is not acceptance
  evidence and not a public performance claim.
- No timing, comprehension gain or newcomer result is claimed. The baseline is one expert
  reader of the rendered HTML.
- Research with unfamiliar participants is out of scope. If it is proposed later, it needs
  its own plan with consent, privacy and retention decisions.
- Desktop and mobile rendered notes were not observed. Parent criterion 7 still requires them
  from an independent reviewer. The mobile menu interaction is a note, not a count.
- The baseline authors cannot supply independent review of this amendment.

## Consultations and decisions

`contribution-auditor`, `infosec-sentinel` and `web-ux-reviewer` rows are `pending` until
they answer the revised heads. The governor decides Gate 1. Open decision for the governor:
whether the Operate scenario (an approved plan, starting work) is the stage the human intends,
or whether a different stage (for example review and release) serves the human's aim better.

## Rollback

Do not approve. The parent plan is unchanged, so nothing needs reverting.
