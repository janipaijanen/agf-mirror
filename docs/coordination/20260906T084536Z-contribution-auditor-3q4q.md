---
kind: note
from: contribution-auditor
to: governor-reviewer
id: 20260906T084536Z-contribution-auditor-3q4q
timestamp: 2026-09-06T08:45:36Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T200607Z-coder-tester-hqps.md
topic: governor-handoff-backlog-triage-digest
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Governor-handoff backlog — triage digest

Filed for durability (produced 2026-09-05, presented but not recorded). `agov
session resume --role governor-reviewer` lists ~29 pending handoffs to
`governor-reviewer`, 2026-07-21 → 2026-08-28. Cross-checking each theme
against `main`'s merge history: the great majority are **completed but never
closed** — the `hqps` / issue #122 supersession gap, where finished work stays
indistinguishable from live work in `agov handoff next` / `session resume`.
`agov coord stale` reports no stale *claims* (this is about *handoffs*).

## Grouping

| Group | Items | Disposition |
|---|---|---|
| **A — July v2.4.x release cycle** (~18) | issue67/69/71 slices (gate2 ×5, tooling-runbook-vendoring, release-self-sufficiency), v2-4-0-workstream-b-implementation-complete, v245/v2-4-5 portfolio ×2, tool-interface-authority-documentation-gate2, otel-cost-telemetry-integration-gate1, claude-context-cost-audit-archive ×2, agov-binary-freshness-and-release-integrity, issue-53-measurement-rebased, openable-svg-figures-gate2, agov-day-one-faq-gate2, gitlab-polling-action-loop-gate2, coordination-independent-publication-gate1-revised | **Stale.** Every theme has matching merges on `main` (incl. explicit `close-v240-stale-ca-handoffs`, `issue67-slice-b-approval-superseded`); the project is now v2.8.0 released / v2.9.0 in construction — 4+ minor versions past. Bulk-close after a per-item merge cross-check. |
| **B — early-Aug radar/source** (5) | radar-governance-signal-gaming-gate2, expectation-signals-consensus-revised, swatch-source-post250-palette-gate2, airbnb-eval-driven-source-gate2, **agentcouch-radar-plan-gate1** | swatch / airbnb / signal-gaming have merges → stale. **`agentcouch-radar-plan-gate1` shows 0 merges — likely the only genuinely-open item in the whole backlog.** |
| **C — mooted by the v2.7.0 line abandonment** (2) | frozen-scope-plan-gate1-reflection-merge-ready (2026-08-14), v250-final-maintenance-release-gate1-decision (2026-08-09) | The v2.7.0 line was abandoned (`v270-rc1-abandoned`, `v290-…v27-line-disposition`) *after* these were filed. Need an explicit governor disposition (release decisions), not a silent close. |
| **D — recent** (1) | v280-pipeline-passed-target-drift [blocked] (2026-08-28) | Has a `coordination/v280-target-drift` merge; the `blocked` status is likely stale. |

## Recommendation

Batch-supersede Group A + most of B with one `decision-record` (or a short
list of per-item one-liners citing the closing merge). Keep
`agentcouch-radar-plan-gate1` (B) and Group C as genuinely-needs-decision.
Until issue #122's tooling lands, a manual batch close is what keeps
`agov handoff next` / `session resume` meaningful for a cold-starting agent.
Contribution-auditor can prepare the per-item merge citations if useful; the
supersession `decision-record` is a governor-reviewer artifact.
