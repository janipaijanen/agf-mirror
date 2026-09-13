---
id: agents-govern-2.0-session-continuity-s1
status: complete
timestamp: 2026-06-13T12:40:16Z
references:
  - docs/proposals/session-continuity-and-memory.md
  - docs/proposals/session-continuity-assessment-planner-architect.md
  - docs/coordination/00156-coder-tester.md
  - docs/coordination/00157-planner-architect.md
  - docs/coordination/00167-coder-tester.md
  - docs/coordination/00168-coder-tester.md
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: Historical plan predating this project's per-plan advisory-consultation tracking convention; reconstructed retroactively per qbka Slice B, no fabricated consultation record created.
  - role: contribution-auditor
    status: not-applicable
    reason: Historical plan predating this project's per-plan advisory-consultation tracking convention; reconstructed retroactively per qbka Slice B, no fabricated consultation record created.
---

**Correction (2026-09-13, qbka Slice B):** this file had no `id` or
`advisory_consultation` (schema-invalid) and `status: pending-gate1`, which
misrepresented the work as unimplemented. `agov session resume` exists
today and is this project's documented cold-start command
(`docs/reference/session-resume.md`, AGENTS.md) — the feature this plan
proposed (originally `mix agov.session.resume`, later ported to Rust).
Reconstructed per `docs/plans/20260913T171837Z-qbka-slice-b-legacy-plan-
migration.md`.

# Plan: Session Continuity S1 — `mix agov.session.resume`

## Intent

Add a read-only Mix task that assembles a role-specific boot context from the
existing coordination channel. The task shows pending handoffs, recent
decisions, open claims, hidden branch-local coordination files, and dirty
worktree state.

This reduces cold-start token cost (from ~3,000–5,000 tokens of manual channel
reconstruction to ~300–500 tokens for structured resume output) and prevents
L005/L007-class false context at session boundaries.

## Background and evidence

- **L005**: validated governance artifacts left uncommitted; agent had false
  "done" state.
- **L007**: commits landed on wrong branch; branch treated as stable session
  context rather than volatile state.
- **seq 156–168**: assessment and debate converging on S1 = one command only.
  Codex and planner-architect agree: `session.resume` is immediately valuable;
  `session.guard`, `session.closeout`, and `handoff.snapshot` are deferred.

## Scope: what S1 includes

**One command:** `mix agov.session.resume [--role ROLE]`

Output:

```
Role: <role>
Branch: <current branch>
Dirty tree: yes/no [list if yes]
Pending handoffs addressed to you:
  - seq NNN from <agent> — <topic> — <status>
  [...]
Open claims owned by you:
  - <artifact> (seq NNN, claimed <date>)
  [...]
Stale claims (expired):
  - <artifact> (expired <date>)
  [...]
Recent decisions (last 3):
  - seq NNN <topic>
  [...]
Must read:
  - <file>
  [...]
Warnings:
  - [hidden branch-local coordination files not visible on master]
  - [dirty tree: unrelated staged changes present]
  - [before durable writes: run git branch --show-current + git status --short --branch]
```

**No writes.** Read-only. No CI gate. Optional use.

**Scope explicitly excluded from S1:**
- `mix agov.session.guard` — deferred; calibration risk
- `mix agov.session.closeout` — deferred; rule covers the obligation
- `mix agov.handoff.snapshot` — deferred; revisit on 2+ recurrences
- Pre-commit hooks — deferred; `session.resume` is not an enforcement gate

## Affected files

| File | Action | Purpose |
|---|---|---|
| `tooling/elixir/lib/agov/session_resume.ex` | Create | Core logic: assembles role-specific boot context from existing readers |
| `tooling/elixir/lib/mix/tasks/agov.session.resume.ex` | Create | Mix task wrapper; CLI arg parsing, output formatting |
| `tooling/elixir/test/agov/session_resume_test.exs` | Create | Unit tests on five fixture scenarios |
| `tooling/elixir/test/fixtures/session_resume/` | Create | Fixture coordination channel directory |
| `tooling/elixir/test/fixtures/session_resume/basic/` | Create | Fixture: ready handoff + resolved handoff + stale claim |
| `tooling/elixir/test/fixtures/session_resume/branch_local/` | Create | Fixture: hidden branch-local coordination file scenario |

No changes to existing modules. No schema changes. No coordination message
schema changes (closeout uses ordinary `note` kind — no new kind required).

## Dependencies on existing modules

`Agov.SessionResume` composes without modifying:

- `Agov.ChannelReader.scan/1` — channel scan
- `Agov.HandoffTracker.scan/1` — pending vs. resolved handoff state
- `Agov.ClaimTracker.scan/2` — active, stale, released claims
- `Agov.HandoffReader.find_next/2` — role-filtered pending handoff

New: shell call to `git branch --show-current` and `git status --porcelain`
for branch and dirty-tree detection. These are shelled out (not Elixir-native)
to avoid a git library dependency.

## Migration needed

No. Read-only command, no schema changes, no persistent state.

## Test strategy

Five fixture scenarios per seq 167 Q3 (Codex):

1. **Ready handoff:** channel has one unresolved handoff addressed to the
   current role. Assert resume surfaces it in pending list.

2. **Resolved handoff:** channel has a handoff followed by a decision-record
   referencing it. Assert resume does NOT surface the resolved handoff.
   (Negative assertion — critical for L007 prevention.)

3. **Hidden branch-local file:** fixture represents a coordination file present
   on a plan branch but absent from master. Assert resume emits a warning.
   Implementation note: this fixture must simulate the git branch state or be
   driven by a configurable base-branch comparator. See Gate 1 Q3.

4. **Stale claim:** channel has a claim with `expires_on` in the past relative
   to a test-injected clock. Assert resume surfaces it in stale claims section.

5. **Dirty tree:** injectable worktree status shows modified files. Assert
   resume emits dirty-tree warning and before-durable-write reminder.

TDD: write all five tests RED before implementing `Agov.SessionResume`.

## Risks

| Risk | Mitigation |
|---|---|
| Branch-local file detection requires a git subprocess | Accept; document that `mix agov.session.resume` requires git on PATH |
| ClaimTracker `expires_on` handling needs injectable clock for tests | Pass `now:` option via Keyword (ClaimTracker already supports this) |
| Hidden branch-local detection is incomplete if the base branch name varies | Accept a `--base-branch` flag defaulting to `main`/`master`; document |
| Output is too verbose for large channels | Cap "recent decisions" at 3; cap pending handoffs at 10; add `--verbose` for more |

## Rollback

Remove `tooling/elixir/lib/agov/session_resume.ex` and
`tooling/elixir/lib/mix/tasks/agov.session.resume.ex`. No schema changes to
revert, no persistent state to clean up, no CI gate to disable.

## Gate 1 questions for governor

**Q1 — Role resolution:** Should `--role` be a required CLI argument, or should
`mix agov.session.resume` infer the role from `agov-config.yaml`? Config
inference is cleaner for single-role sessions; an explicit flag is more
flexible for a human switching between roles. Recommendation: infer from config,
accept `--role` override.

**Q2 — Base-branch comparator for hidden files:** Should hidden branch-local
file detection use `git diff --name-only HEAD..origin/main` (accurate, requires
remote fetch) or `git show origin/main:docs/coordination/ 2>/dev/null | diff`
(fragile) or an explicit `--base-branch` flag that defaults to skipping if
unavailable (safest for air-gapped environments)? Recommendation: `--base-branch
main` flag defaulting to skip the check if the branch is not accessible.

**Q3 — Dirty tree injection in tests:** Should dirty-tree detection be injected
via a behaviour/adapter (pure Elixir, no subprocess in tests) or shelled out to
`git status --porcelain` in both production and tests (simpler, but requires git
in the test environment)? Recommendation: thin adapter pattern — `Agov.GitStatus`
behaviour with a real implementation and a stub for tests.

## Micro-task breakdown (after Gate 1 approval)

| # | File | Done when |
|---|---|---|
| T1 | `test/agov/session_resume_test.exs` | Five RED tests |
| T2 | `test/fixtures/session_resume/basic/` | Basic fixture files created |
| T3 | `test/fixtures/session_resume/branch_local/` | Branch-local fixture files created |
| T4 | `lib/agov/session_resume.ex` | Five tests GREEN |
| T5 | `lib/mix/tasks/agov.session.resume.ex` | `mix agov.session.resume --role coder-tester` prints correct output against fixture |
| T6 | Gate-self-test | Negative test: resume against bad-state fixture, confirm warnings surface |
| T7 | Docs | Add `session.resume` to tooling reference |
