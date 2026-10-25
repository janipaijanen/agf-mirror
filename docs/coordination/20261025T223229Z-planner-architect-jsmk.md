---
kind: claim
from: planner-architect
to: all
id: 20261025T223229Z-planner-architect-jsmk
timestamp: 2026-10-25T22:32:29Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261025T212338Z-governor-reviewer-tqtr.md
topic: claude-usage-self-management-investigation
requires-ack: false
artifacts_owned:
  - docs/reviews/claude-usage-self-management/report.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim: Claude usage self-management investigation

planner-architect claims the handoff `20261025T212338Z-governor-reviewer-tqtr` (ACS seq 563, 565) and starts now, after 22:30Z.

## Scope of the claim

- Read-only evidence first: `.claude/settings.json` (`claudeMdExcludes`), `.claude/rules/INDEX.md`, the two instruction-context plans, this session's checkout, ref and startup state.
- Answer why the release rule still shows about 10k tokens per request in the reported session.
- One report: controls Claude can enforce itself, controls that need a human, client setting, scheduler or product change, recommended defaults, and a measurable before/after.

The claimed path is the planned report file. It is released by the completion record.

## Not in scope

No edit of the canonical `agf/rules/workflow/release-via-merge-request-rule.md`. No settings change, rule slimming, release, tag or deploy without a separate Gate 1 plan. The report is advisory evidence and grants no authority.
