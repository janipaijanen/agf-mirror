---
kind: handoff
from: coder-tester
to: coder-tester
id: 20260906T203532Z-coder-tester-5bjb
timestamp: 2026-09-06T20:35:32Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260906T195149Z-governor-reviewer-56bh.md
  - 20260906T195537Z-contribution-auditor-mmmp.md
  - 20260906T195800Z-planner-architect-4mk8.md
topic: universal-polling-claude-docs-slice-ready-for-integration
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/operate/poll-and-coordinate.md
  - agf/docs/site/docs/operate/runbook.md
  - agf/skills/poll-and-coordinate/SKILL.md
  - agf/rules/workflow/branch-and-worktree-rule.md
  - docs/reference/agent-cold-start-habits.md
  - docs/reference/agf-development-starter.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - >-
      Integrate the Claude documentation slice for the Gate-1-approved
      universal-polling plan. Branch feat/universal-polling-docs-slice,
      exact head 78c46edbe1653e940959c3276f6b55c6c4e16b26, off main
      d854ec30. No AGENTS.md edit; branch-and-worktree-rule.md changed only
      in its multi-agent branch-watch subsection.
    - >-
      The slice forward-references
      agf/rules/always/universal-polling-and-autonomous-continuation-rule.md
      by bare path in 5 of the 6 files (a real link in
      branch-and-worktree-rule.md, resolving ../always/). That file is
      created by the Codex slice; the references resolve after integration.
    - >-
      Integrate without semantic rewriting per 56bh/4mk8. Any cross-slice
      conflict stops for joint resolution.
  must_read:
    - Branch feat/universal-polling-docs-slice @ 78c46edb
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claude documentation slice ready for integration

Filed by this Claude session in its `56bh`/`mmmp` temporary
coder-tester documentation-slice role.

## Exact head and evidence

- Branch: `feat/universal-polling-docs-slice`
- Head: `78c46edbe1653e940959c3276f6b55c6c4e16b26`
- Base: `main` `d854ec304d22e02908fe9711b1ece98291ae912d`
- `agov validate` (this head): `adopter_docs` OK, `assistant_loader` OK,
  `assistant_drift` OK. `git diff --check` clean. The `assistant_drift`
  `.claude/rules` WARN is pre-existing and unrelated to this slice.
- 6 files, 136 insertions / 17 deletions. **No `AGENTS.md` edit.**

## What changed

| File | Change |
|---|---|
| `poll-and-coordinate.md` | audience → every active agent/session (not "responsible for advancing", not prompt-triggered); observation covers the whole surface regardless of author; mandatory cold-start/daily full reconciliation; step 8 uses the canonical priority order; cursor advances only after the action is complete/running/blocked; new "Collision-safe selection, role consensus, and stopping" section; non-reentrant cadence paragraph. Points to the canonical rule as owner. |
| `runbook.md` | the "Asked to poll or coordinate" situation row is replaced by an always-on "Active session — cold start, resume, or a configured wake (no prompt needed)" row plus a "At a safe wait boundary" spare-capacity row. |
| `SKILL.md` | canonical-rule pointer in the intro; non-reentrant coalescing added to Step 0; Step 3 opens with "primary-watcher is not an observation exemption"; Step 5 requires re-checking claims/overlap before first mutation and remote revalidation before claiming a decomposed child. |
| `branch-and-worktree-rule.md` | the "Multi-agent branch-watch ownership" subsection now states the primary-watcher designation is a scheduled-notification role, not an observation exemption; non-primary watchers keep a complete scan. Links `../always/universal-polling-and-autonomous-continuation-rule.md`. Only this subsection changed. |
| `agent-cold-start-habits.md` | canonical-rule pointer in the intro; non-reentrant coalescing note; collision re-check before first mutation in the "Treat Timers As Intake" action-loop paragraph. |
| `agf-development-starter.md` | states complete polling is a standing obligation of every active session (not prompt-triggered) with the canonical-rule pointer; non-reentrant note in the poll-loop paragraph. |

## Carry to Gate 2

Per `56bh`, the accepted mandatory refinements ride the integrated head, not
this slice: `6mk1` AV1-AV4 and infosec `p74g` SI-1. This slice adds no new
findings.

## Role boundary

Per `56bh`/`4mk8`, because this session authored an implementation slice it
does **not** provide the independent whole-change contribution audit; that
goes to a reviewer independent of both slices before Gate 2. This session's
temporary role ends at the integrated-head Gate 2 handoff.
