<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->

# Slice 4 ("How do I work?"): sentence-preservation report and anchor map

- **Status:** advisory review evidence. Not a complete semantic inventory and not a blocking check.
- **Plan:** `docs/plans/20261030T163253Z-pages-structure-and-voice.md`, work allocation item 4 (this is the first, bounded part: restore the Operate cap and put ordinary work first).
- **Reproduce:** `python3 -I docs/reviews/pages-structure-and-voice/slice-report-extractor.py origin/main agf/docs/site/docs/operate/day-to-day.md agf/docs/site/docs/operate/runbook.md`

## Extractor output

```
Base ref: origin/main (6bd6cfb02b594ecc5a2ce94eb96f56374cb330cd)

## day-to-day.md

- Candidate sentences in base: 16; present verbatim in final: 15; not found: 1
  - NOT FOUND: "The point to retain before reading Table 1: the figure's three columns are the answer key for "who does this," and the table below is that same answer key applied to specific artifacts. ![Default model: human decision authority, AI-executed, and mixed responsibility modes separated by authority](../../reference/figures/operating-responsibility-map.svg) *AI-generated figure."
- Headings before: 13; after: 13
- Heading ids before: day-to-day-operation, the-daily-shape, daily-checklist, optional-improvement-propeller-cycle, when-the-gate-change-check-fails, proportional-source-and-radar-maintenance, coordination-hygiene, show-the-assistant-and-role-on-gitlab, context-hygiene, multi-agent-sessions, the-governors-day, failure-modes-to-watch, next
- Heading ids after:  day-to-day-operation, the-daily-shape, daily-checklist, optional-improvement-propeller-cycle, when-the-gate-change-check-fails, proportional-source-and-radar-maintenance, coordination-hygiene, show-the-assistant-and-role-on-gitlab, context-hygiene, multi-agent-sessions, the-governors-day, failure-modes-to-watch, next
- Removed ids: none; added ids: none

## runbook.md

- Candidate sentences in base: 9; present verbatim in final: 9; not found: 0
- Headings before: 7; after: 7
- Heading ids before: operations-runbook, choose-the-situation, authority-stays-with-roles, durable-state-rule, adapter-boundaries, gitlab-adapter, claude-adapter
- Heading ids after:  operations-runbook, choose-the-situation, authority-stays-with-roles, durable-state-rule, adapter-boundaries, gitlab-adapter, claude-adapter
- Removed ids: none; added ids: none

## Cross-file anchor links in the final files

- runbook.md -> session-continuity.md#start-or-resume: ok
- runbook.md -> session-continuity.md#graceful-checkpoint: ok
- runbook.md -> session-continuity.md#abrupt-loss-recovery: ok
- runbook.md -> poll-and-coordinate.md#collision-safe-selection-role-consensus-and-stopping: ok
- runbook.md -> day-to-day.md#optional-improvement-propeller-cycle: ok
```

## Changes

| Page | Change | Disposition |
|---|---|---|
| `day-to-day.md`, "The daily shape" | The paragraph after Figure 1 and the paragraph before Figure 2 are shortened. | Both said the same things twice: the figure's caption-like explanation and a "point to retain". Kept: modes are sorted by action, not by trust; planning and approving differ even when one agent does both; Table 1 applies the split; the six steps form a cycle; the completion or learning record feeds step 1 and skipping it breaks the cycle for the next reader. Figures, captions, tables and the deputy pointer are unchanged. |
| `runbook.md` | The "Beginning ordinary governed work" row moves to the top of the table. A one-sentence pointer leads the page. | Order and pointer only. All 12 situations and their links are kept. |

## A1 Operate route

Source-level: "The daily shape" has 99 fewer words by the crude section count, and "Choose the situation" is unchanged in length. The original Operate cap is 1,174 (runbook "Choose the situation" 305 + day-to-day "The daily shape" 557 + Home 312 at most). The rendered counter runs on the strict-build artifact of the published head. No baseline is reset.

## Not done

No change to any rule, role, figure, navigation or other page. No heading id changes.
