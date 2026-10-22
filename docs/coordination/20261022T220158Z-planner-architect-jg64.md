---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261022T220158Z-planner-architect-jg64
timestamp: 2026-10-22T22:01:58Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261022T213843Z-governor-reviewer-jk7s
  - 20261022T215322Z-coder-tester-nnmm
  - 20261022T215633Z-coder-tester-hqr3
  - 20261022T215915Z-coder-tester-pde7
topic: gate2-instruction-context-phase0-mr3120
requires-ack: true
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3120
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review MR !3120 at exact head 6bf5cef6f06d71725c78d299e308572a98868a2d and issue a separate Gate 2 decision.
    - If approved, authorize the merge to main explicitly; this handoff does not authorize merge.
    - Decide whether to accept the Phase 0 evidence so Phase 1 (AGENTS.md slimming) may start.
  must_read:
    - docs/plans/20261022T205113Z-instruction-context-reduction.md
    - docs/coordination/20261022T213843Z-governor-reviewer-jk7s.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3120
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3121
  do_not_touch:
    - refs/heads/main before binding Gate 2 approval and explicit merge authorization
    - chore/instruction-context-phase0
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !3120 is ready for Gate 2 review (Phase 0 of the instruction-context plan)

The work is red-listed under delegation artifact `qpda` (it changes `.claude/**`).
Only the human governor decides Gate 2 and the merge. No delegated decision was
used; the count remains 0 of 5.

## Exact candidate

- MR: <https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3120>
- Source head: `6bf5cef6f06d71725c78d299e308572a98868a2d`, target `main`.
- Exact-head MR pipeline `2978548530`: success.
- Plan: `20261022T205113Z-instruction-context-reduction` (issue #138), Gate 1
  approved by `jk7s` at plan head `3bfa9603`; plan metadata head `7144cbd4`.

## What changed (3 new files, no rule text, no symlink, no `agf/` change)

- `.claude/settings.json`: `claudeMdExcludes` with 25 globs.
- `.claude/rules/INDEX.md`: 25 "when X, read Y" rows, each restating the
  canonical rule's own Scope.
- `docs/reviews/rule-projection/20261022T205113Z-instruction-context-reduction.yaml`:
  six-surface manifest and coverage matrix.

## Evidence

- Fail-closed checkpoint required by `jk7s` (Claude Code 2.1.308,
  `InstructionsLoaded` hook via `claude -p`): before 57 files, 323,871 chars;
  after 33 files, 169,674 chars. Exactly the 25 intended files are absent,
  `INDEX.md` is added, every other file still loads (including
  `external-anonymization`, `branch-and-worktree`, `systematic-code-review`).
  The pattern `**/.claude/rules/<dir>/<file>.md` works on this version.
- Expected startup report after Phase 0: about 162k chars (ratio 0.953 from the
  308.8k report). This misses the 140.0k target; Phase 1 is required. Phase 0
  alone is not a claim of reaching the target.
- `agov plan projection-check --base-ref origin/main` on the manifest: OK.
  25 settings = 25 INDEX rows = 25 manifest owner paths (script-checked).
- `agov validate`: `assistant_drift` advisory lines 7 to 8; the new line is
  `INDEX.md` having no canonical `agf/rules/` counterpart (expected, advisory;
  `readme_stats` WARNs pre-exist).
- Behavior replay (`claude -p`, no tools): coordination record answered NONE
  (correct, rules already loaded); release bump answered
  `release-via-merge-request-rule.md` (correct); a plan changing runbook MUST
  text first MISSED the projection rule, the INDEX row was tightened, the re-run
  was correct.
- Independent exact-head review: Codex rounds `nnmm` (BLOCK at 14261a59),
  `hqr3` (round two, BLOCK at 4bea1c9d), and final `pde7` (CLEAR at 6bf5cef6),
  all in MR !3121. Codex states the pipeline remained an independent Gate 2
  condition; it has since passed.

## Impact and docs parity

- Foundry: every Claude Code session loads about 154k fewer chars. Codex is
  unaffected in Phase 0 (it never reads `.claude/**`).
- Direct adopters and downstream: none. `agf/` and shipped templates unchanged.
- Migration: none. Rollback: revert the MR (removes the exclusions and INDEX).
- Adopter docs parity: not applicable (project-local loader configuration).

## Decisions requested

1. Gate 2 on MR !3120 at the head above, and explicit merge authorization.
2. Acceptance of the Phase 0 evidence, which releases Phase 1 (slim
   `AGENTS.md`, keep Codex trigger lines) per the fail-closed condition in `jk7s`.
