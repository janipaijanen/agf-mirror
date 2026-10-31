<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->

# Slice 4 ("How do I work?"): sentence-preservation report and anchor map

- **Status:** advisory review evidence. Not a complete semantic inventory and not a blocking check.
- **Plan:** `docs/plans/20261030T163253Z-pages-structure-and-voice.md`, work allocation item 4 (this is the first, bounded part: restore the Operate cap and put ordinary work first).
- **Immutable base:** `2c4ff4319cbb072a756a76f405932977bf483d65` (main after slice 3 merged). Earlier versions of this report used `origin/main` at older commits.
- **Reproduce:** `python3 -I docs/reviews/pages-structure-and-voice/slice-report-extractor.py 2c4ff4319cbb072a756a76f405932977bf483d65 agf/docs/site/docs/operate/day-to-day.md agf/docs/site/docs/operate/runbook.md agf/docs/site/docs/adopt/upgrade-from-v1.md agf/docs/site/docs/adopt/exit-and-decommission.md`

## Extractor output

```
Base ref: 2c4ff4319cbb072a756a76f405932977bf483d65 (2c4ff4319cbb072a756a76f405932977bf483d65)

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

## upgrade-from-v1.md

- Candidate sentences in base: 2; present verbatim in final: 1; not found: 1
  - NOT FOUND: "After this page you have a step-by-step checklist that covers every instruction surface that must be updated, not just the file copy.* !!! note "What changed structurally" v1 placed framework files in `agents-govern/` (or a local copy) with numeric coordination sequence files and v1 commands. v2 places them in `agf/` with tooling-generated readable ID coordination files and the Rust `agov` binary."
- Headings before: 15; after: 15
- Heading ids before: upgrading-from-v1-agents-govern-to-agf-v2, before-you-start, migration-checklist, 1-retire-old-layout-paths, 2-place-project-roles-under-agfagents, 3-move-assistant-commands-and-skill-sources, 4-update-every-instruction-surface-for-cold-start-and-tooling, 5-replace-coordination-numbering-instructions, 6-update-tooling-references-for-colleagues, 7-validate-the-full-layout, 8-validate-new-coordination-entries, 9-scan-active-docs-for-v1-references, 10-review-adopter-learning-records, after-the-checklist, see-also
- Heading ids after:  upgrading-from-v1-agents-govern-to-agf-v2, before-you-start, migration-checklist, 1-retire-old-layout-paths, 2-place-project-roles-under-agfagents, 3-move-assistant-commands-and-skill-sources, 4-update-every-instruction-surface-for-cold-start-and-tooling, 5-replace-coordination-numbering-instructions, 6-update-tooling-references-for-colleagues, 7-validate-the-full-layout, 8-validate-new-coordination-entries, 9-scan-active-docs-for-v1-references, 10-review-adopter-learning-records, after-the-checklist, see-also
- Removed ids: none; added ids: none

## exit-and-decommission.md

- Candidate sentences in base: 5; present verbatim in final: 3; not found: 2
  - NOT FOUND: "Reading the branch point first, before the table below defines each mode in detail, shows that all three start from the same decision ("stop using some or all of AGF") and differ only in how much of the governed history and tooling stays behind afterward."
  - NOT FOUND: "The point to retain: exit is a branch on the same timeline as adoption, evaluated with the same governed-decision discipline — not an emergency procedure reached only after something has gone wrong. ![Adoption timeline branching into freeze, replacement, and decommission exit paths](../../reference/figures/adoption-exit-flow.svg) *AI-generated figure."
- Headings before: 5; after: 5
- Heading ids before: exit-and-decommission, three-exit-modes, what-you-keep-after-full-removal, what-removal-involves, why-this-page-exists
- Heading ids after:  exit-and-decommission, three-exit-modes, what-you-keep-after-full-removal, what-removal-involves, why-this-page-exists
- Removed ids: none; added ids: none

## Cross-file anchor links in the final files

- day-to-day.md -> ../concepts/coordination-protocol.md#how-agents-communicate: ok
- runbook.md -> session-continuity.md#start-or-resume: ok
- runbook.md -> session-continuity.md#graceful-checkpoint: ok
- runbook.md -> session-continuity.md#abrupt-loss-recovery: ok
- runbook.md -> poll-and-coordinate.md#collision-safe-selection-role-consensus-and-stopping: ok
- runbook.md -> day-to-day.md#optional-improvement-propeller-cycle: ok
- upgrade-from-v1.md -> (same file)#before-you-start: ok
- exit-and-decommission.md -> (same file)#three-exit-modes: ok
```

## Changes

| Page | Change | Disposition |
|---|---|---|
| `day-to-day.md`, "The daily shape" | The paragraph after Figure 1 and the paragraph before Figure 2 are shortened. | Both said the same things twice: the figure's caption-like explanation and a "point to retain". Kept: modes are sorted by action, not by trust; planning and approving are different actions and, per review mpq4, the assigned approver decides and an AI assistant that prepared or implemented the work cannot approve its own (the earlier shorthand "even when one agent does both" is removed because it invited a self-approval reading); Table 1 applies the split; the six steps form a cycle; the completion or learning record feeds step 1 and skipping it breaks the cycle for the next reader. Figures, captions, tables and the deputy pointer are unchanged. |
| `runbook.md` | The "Beginning ordinary governed work" row moves to the top of the table. A one-sentence pointer leads the page. | Order and pointer only. All 12 situations and their links are kept. |
| `upgrade-from-v1.md` | A three-sentence lead before the structural note: use this page if the project runs v1 and is moving to v2; the human who approves changes decides whether and when and approves the result; the assistant carries out the checklist; begin at "Before you start". | Additive. The checklist, the note and every caveat are unchanged. The extractor block reported as not found is a glue artefact of the unchanged note. |
| `exit-and-decommission.md` | A short lead names the human governor as the person accountable for approval decisions, links the three exit modes, and says removal means the framework-owned integration and that every mode keeps Git history. The paragraph around Figure 1 is shortened (review 1169). | Kept: the three modes branch from one decision; they differ in how much active governance and tooling stays in the working tree (not in history); same discipline as adopting; exit is not an emergency procedure. Table 1, "What you keep", "What removal involves" (learning export, optional project evidence, code and tests, no history rewrite, unmeasured duration) and all caveats are unchanged. |

## A1 Operate route

Source-level: "The daily shape" has 99 fewer words by the crude section count, and "Choose the situation" is unchanged in length. The original Operate cap is 1,174 (runbook "Choose the situation" 305 + day-to-day "The daily shape" 557 + Home 312 at most). The rendered counter runs on the strict-build artifact of the published head. No baseline is reset.

## Not done

No change to any rule, role, figure, navigation or other page. No heading id changes.
