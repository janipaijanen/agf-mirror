---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 169
timestamp: 2026-06-13T12:40:16Z
immutable: true
references-to:
  - 00167-coder-tester.md
  - 00168-coder-tester.md
topic: session-continuity-s1-gate1-plan
requires-ack: true
artifacts_owned:
  - docs/plans/agents-govern-2.0-session-continuity-s1.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and approve the S1 Gate 1 plan for mix agov.session.resume.
    - Answer the three Gate 1 questions in the plan.
  must_read:
    - docs/plans/agents-govern-2.0-session-continuity-s1.md
    - docs/proposals/session-continuity-assessment-planner-architect.md
    - docs/coordination/00167-coder-tester.md
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
    - agents-govern/
---

# Gate 1: Session Continuity S1 — `mix agov.session.resume`

Planner-architect and coder-tester have converged on a single S1 scope.

## What is being proposed

One new Elixir Mix task: `mix agov.session.resume`.

Read-only. Assembles a role-specific boot context from existing readers
(ChannelReader, HandoffTracker, ClaimTracker, HandoffReader). Output: pending
handoffs, recent decisions, open/stale claims, hidden branch-local warnings,
dirty tree warnings.

## Why: evidence from this project

- **L005**: agents left validated governance artifacts uncommitted; another
  agent had to recover them.
- **L007**: agents committed to wrong branches; branch treated as stable context.
- **This session**: planner-architect buried seq 157 on a plan branch, invisible
  to Codex — exactly the failure `session.resume` is designed to prevent on
  cold-start.

## What is explicitly deferred

- `session.guard` — calibration risk; defer until resume data shows it needed
- `session.closeout` — rule covers the obligation; closeout shape is a `note`
  convention, not a new command
- `handoff.snapshot` — manual visibility bridge pattern works; automate on 2+
  recurrences
- Pre-commit hooks

## Three Gate 1 questions requiring governor decision

**Q1 — Role resolution:** Infer `--role` from `agov-config.yaml` with
`--role` override? Or require explicit flag? Recommendation: infer from config.

**Q2 — Hidden branch-local file detection:** Use `--base-branch main` flag
(defaulting to skip if not accessible) for hidden coordination file detection?
Or require remote fetch? Recommendation: `--base-branch` flag, skip if
unavailable.

**Q3 — Dirty tree adapter in tests:** Use `Agov.GitStatus` behaviour with a
stub for tests? Or shell out to `git status --porcelain` in both environments?
Recommendation: thin behaviour adapter.

## Token ROI

- Current cold-start cost (manual channel reconstruction): ~3,000–5,000 tokens
- With `session.resume`: ~300–500 tokens
- Net saving: ~2,500–4,500 tokens/boot
- Payback: first day of use

## Implementation

Seven micro-tasks defined in the plan. TDD from T1 (five RED tests) through
T6 (gate-self-test negative). Two new files; no existing module changes; no
schema changes.
