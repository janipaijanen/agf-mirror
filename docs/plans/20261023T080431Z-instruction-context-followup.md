---
id: 20261023T080431Z-instruction-context-followup
status: approved
gate1_approved_by: 20261023T112836Z-governor-reviewer-dhdh
gate1_approved_at: 2026-10-23T11:28:36Z
topic: instruction-context-followup-below-140k-chars
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >
      `agov validate infosec` passes; Codex's screen found no credentials or
      cross-project linkage. Summarized in contribution-auditor record
      20261023T112100Z-contribution-auditor-8mh1.
  - role: contribution-auditor
    status: consulted
    reason: >
      Audit by the AI side of the role (the plan's own author), record
      20261023T112100Z-contribution-auditor-8mh1: no blocking finding, two INFO.
      The governor may pull or redo it.
affected_paths:
  - AGENTS.md
  - .claude/settings.json
  - .claude/rules/INDEX.md
  - .claude/rules/workflow/simplicity-rule.md
  - docs/reference/foundry-agents-md-moved-text.md
  - docs/reviews/rule-projection/20261023T080431Z-instruction-context-followup.yaml
depends_on:
  - 20261022T205113Z-instruction-context-reduction
  - 20261023T045214Z-remove-elixir-references
  - 20260913T085123Z-work-item-steward-gate1
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20260921T040743Z-assistant-rule-drift-resolution
  - 20260920T113336Z-governance-mode-classification
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: second pass to bring always-loaded context under 140k chars

Follow-up to issue #138 and plan `20261022T205113Z-instruction-context-reduction`,
whose Phase 1 landed (MR !3126) with the 140.0k target unmet and the governor's
explicit acceptance of about 150.0k plus separate follow-up work (record
`20261023T050150Z-governor-reviewer-qk4s`). Authored by planner-architect.
Gate 1 approved in governor-reviewer decision record
`20261023T112836Z-governor-reviewer-dhdh` (scope unchanged). Gate 2 and merge of
the implementation stay with the Human Governor.

## Decision requested

Gate 1 approval for a foundry-local, no-shipped-text change that moves six more
situational rules off the always-load path (same `claudeMdExcludes` mechanism as
Phase 0) and trims one foundry-local projection. `AGENTS.md` and `.claude/**` are
on the delegation red list, so Gate 1 and Gate 2 are the Human Governor's alone;
no `qpda` decision applies.

## Measured baseline (2026-10-23, main @ eb70adb8)

`InstructionsLoaded` run via `claude -p`: 33 files, 156,887 chars on disk, about
149.5k startup-report chars at the Phase 0 ratio 0.953 (about 156.2k at Codex's
0.9955). Target: at most 140.0k measured report chars. The ratio is uncertain,
so acceptance is the measured report, not the estimate, and the plan must clear
140.0k under both ratios.

Largest loaded files (chars): `coordination-publication-rule` 22,790;
`AGENTS.md` 21,093; `.claude/rules/workflow/simplicity-rule.md` (foundry-local
projection) 11,922; `branch-and-worktree-rule` 10,260; `gate-self-test-rule`
8,753; `systematic-code-review-rule` 7,260; `external-anonymization-rule` 7,252.

## Constraints

1. No obligation is dropped, weakened, or re-owned. One owner per statement
   (`documentation-rule-projection-rule`); every other surface points to it.
2. A rule moved off the always-load path stays discoverable: one trigger line in
   `AGENTS.md` (Codex loads only that file, Codex review `a6041191` F1) and one row
   in `.claude/rules/INDEX.md` (Claude), each restating the canonical Scope.
3. No `agf/rules/**` text changes; no compression; files and symlinks stay on disk
   so every path reference and the advisory `assistant_drift` hash check hold.
4. Keep Tier A as decided by Codex: `external-anonymization`, `branch-and-worktree`,
   `systematic-code-review`, the safety rules, and the coordination message and
   sequence-guard rules.

## Design

### Package B: demote six situational rules (about 17.9k chars)

Same mechanism as Phase 0: add `claudeMdExcludes` globs, INDEX row, and an
`AGENTS.md` trigger line for each.

| Rule (canonical Scope) | Chars | Trigger |
|---|---|---|
| `always/agent-improvement-proposal-rule` | 3,515 | noticing friction, duplication, confusing workflow, avoidable token spend or a doc gap while doing assigned AGF work, or proposing an improvement (route as note, review, lesson-proposal or plan) |
| `always/governance-file-layout-rule` | 3,414 | creating or placing a learning record, agent definition, ADR, framework rule, working plan, coordination record or `GOVERNANCE.md`, or deciding where a governance artifact lives |
| `workflow/coordination-challenge-response-rule` | 2,961 | raising or answering a coordination `challenge` (`kind: review`) |
| `workflow/debugging-rule` | 2,859 | diagnosing a bug, failing test, stack trace or screenshot |
| `workflow/feature-regression-guard-rule` | 2,606 | modifying a route, service, template conditional, permission check or feature flag, or deleting or weakening a test |
| `workflow/coordination-id-rule` | 2,501 | creating any coordination message, or fixing an ID or filename (the always-loaded `AGENTS.md` instruction to use `agov coord brief` stays; this trigger adds the filename and validator rules) |

Added back: about 150 chars per trigger line in `AGENTS.md` and a similar INDEX
row, about 1.8k total. Net about -16.1k.

### Package A: trim the foundry-local simplicity projection (about 4.8k chars)

`.claude/rules/workflow/simplicity-rule.md` is a foundry-local projection (11,922
chars), not a symlink. Its "Sources/radar research-maintenance fast path" and
"Required branch, evidence, and review" sections (about 5.1k) are situational and
owned by `agf/rules/workflow/simplicity-rule.md`. Replace them with a three-line
pointer; `AGENTS.md` already carries the pointer line for that exception. The
projection's own header already says the canonical file resolves conflicts, and
no text is lost from the repository. Net about -4.8k.

### Expected result

About -20.9k chars on disk: 156,887 to about 136.0k, which is about 129.6k report
chars at 0.953 and about 135.4k at 0.9955. If measured output is above 140.0k,
the plan does not merge; it stops and returns to the governor.

## Phases

1. Add six `claudeMdExcludes` globs; INDEX rows; `AGENTS.md` trigger lines;
   rewrite the simplicity projection with the pointer.
2. Projection manifest `docs/reviews/rule-projection/20261023T080431Z-instruction-context-followup.yaml`
   (one row per demoted rule, six in total, each disposing the six projection
   surfaces); `agov plan projection-check` OK.
3. Measure: `InstructionsLoaded` log and report; before/after table.
4. Behavior replays, one per demoted rule: a coordination challenge, a debugging
   task, a new ADR, an improvement note, a feature-flag change, a coordination
   record with an ID. Record which rule the agent opens. Replay success is an
   acceptance condition for that rule's demotion: a rule whose replay does not open
   it stays in the always-loaded set and the plan re-measures.

## Test strategy

1. Exclusion proof: the `InstructionsLoaded` log lists exactly the six rules as
   absent and nothing else changed.
2. Negative test (`gate-self-test-rule`, where a check can block): a manifest row
   pointing at a missing owner makes `agov plan projection-check` exit non-zero.
3. Coverage script (not committed): every excluded rule has both an INDEX row and
   an `AGENTS.md` trigger containing its path.
4. Verbatim: `git diff` shows only removal of the projection sections and added
   pointer text; nothing under `agf/rules/**` changes.

## Classification and impact

STRICT: AI loader (`AGENTS.md`, `.claude/**`), downstream-impact trigger path.
```
Impact:
  Foundry:          every Claude session reads about 21k fewer chars; Codex gains six trigger lines.
  Direct adopters:  none; agf/ and shipped templates unchanged.
  Downstream:       none.
  Migration:        not required.
  Evidence:         issue #138; Phase 1 record qk4s; 2026-10-23 measurement above.
```
Adopter docs parity: not applicable (foundry-local loader files).

## Risks and rollback

- R1: a demoted rule is not opened when needed. Mitigation: trigger lines in both
  loaders restating canonical Scope; behavior replays; any rule that a replay
  misses stays Tier A and the plan re-measures.
- R2: the 0.953 ratio is wrong. Mitigation: acceptance uses the measured report.
- R3: trimming the projection hides the sources/radar exception. Mitigation: the
  pointer and the existing `AGENTS.md` line keep it discoverable; canonical file
  unchanged.
Rollback: revert the MR. No tag, release or immutable record is involved.
