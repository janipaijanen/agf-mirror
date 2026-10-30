---
id: 20261030T214700Z-pages-task-metrics-amendment-a1
status: approved
gate1_approved_by: 20261030T215549Z-governor-reviewer-gpqq
gate1_approved_at: "2026-10-30T21:55:49Z"
topic: pages-task-metrics-amendment-a1
affected_paths:
  - docs/plans/20261030T214700Z-pages-task-metrics-amendment-a1.md
  - docs/reviews/pages-structure-and-voice/20261030-task-metrics-baseline.md
depends_on:
  - 20261030T163253Z-pages-structure-and-voice
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: Contribution and factual review cleared exact head dc05e4f1 and reproduced the script hash and counts. Recorded in governor decision gpqq.
  - role: infosec-sentinel
    status: consulted
    reason: InfoSec and authority review cleared exact head dc05e4f1 and reproduced the script hash and counts. Recorded in gpqq.
  - role: web-ux-reviewer
    status: consulted
    reason: Web UX review cleared exact head dc05e4f1 within advisory scope. The reviewer authored parts of the current site but neither planning draft, and disclosed that limit. Recorded in gpqq.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Amendment A1: task metrics for the Pages structure plan (proposed)

Governance mode: STRICT, because it changes acceptance criteria of an approved plan.

Lifecycle: Gate 1 approved by governor decision `20261030T215549Z-governor-reviewer-gpqq`
at reviewed exact head `dc05e4f1a5fc9ca1f3babd14bd00fd73e8c547c5` (MR !3455). The only edits
after that head are the lifecycle metadata, the consultation rows, this paragraph and the
rollback text, as gpqq authorizes. The earlier evidence and provenance below are retained.
This is Gate 1 only. It is not Gate 2, merge, version, tag or deployment approval. The parent's
source-consistency holds and independent implementation review remain.

This is a separate plan file on purpose. The approved plan
`20261030T163253Z-pages-structure-and-voice` stays byte-for-byte unchanged and keeps
`status: approved` for its own criteria. A machine or an agent that reads the parent
therefore cannot read its approval as approval of this amendment. gpqq approves this file at
Gate 1, so criterion 10 applies to the parent plan by reference to this file. Parent criteria 1 to 9
and all original scope stay in force. The parent file is not edited.

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

All three rows are `consulted` with the outcomes in the front matter. Two reviewers independently
fetched the rendered HTML and reproduced the script hash `aaa2cb0444c362cc` and the counts: Home
312; destination subtotals 670, 768 and 862; totals 982, 1,080 and 1,174; Adopt payload 476. This is
source-familiar expert evidence, not user research.

Decision gpqq chose the proposed Operate scenario: a maintainer starts work after an approved plan
and identifies the next authorized actions and artifacts. Finding the procedure does not complete the
change. The no-regression and reading-reduction rules and the limits in this plan are bound.

Browser evidence is incomplete. The review surface returned "Browser is not available: iab". No
desktop, mobile, keyboard, screenshot or viewport acceptance is claimed, and the HTML counts do not
replace it. Before the affected pages are rewritten, an independent rendered baseline is still required
under the parent's sequence, and the later rendered acceptance is preserved.

## Rollback

This plan is approved, so "do not approve" is no longer a rollback. To withdraw or supersede it, a
later `governor-reviewer` record must name this plan, the revision being withdrawn or superseded, and
whether dependent implementation stops or is re-evaluated. Nobody removes criterion 10 or edits this
file's approved status without that record. The parent plan is unchanged, so withdrawing A1 reverts
nothing in the parent file.
