---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260823T094611Z-contribution-auditor-dekp
timestamp: 2026-08-23T09:46:11Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260823T093421Z-planner-architect-d6r6
topic: poll-and-coordinate-skill-round1
requires-ack: false
artifacts_owned: []
---

# Poll-and-coordinate skill — round-one review

## Review evidence envelope

Scope: MR !1790 exact head `a85eded73aac1d9f7efb52ac127dbab5c5c0c5da`
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 2
Advisory findings: 1
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1 and Gate 2

## Evidence checked

- The five-file range contains one pending Gate 1 plan, the new skill, one canonical operating-procedure amendment, the skill index row, and the README skill-count correction.
- `agov plan validate docs/plans/20260823T093120Z-poll-and-coordinate-skill.md` passes at the exact head.
- `agov validate` exits `0`; readme statistics and adopter-document checks pass. Existing advisory findings remain unrelated to this MR.
- Exact-head pipeline `2782960858` passed, including `agov:governance:validate` job `16052486065`, `rust:agov:validate` job `16052486066`, distribution validation, release-path validation, and site rendering.
- The skill correctly requires fetched state, full pagination, all shared surfaces, gate classification, authorized action, and a real assistant wake-up rather than shell timers.

## Finding PAC-R1 — the requested pause/cycle/rearm contract is incomplete

Check: poll-scheduler-lifecycle-and-durable-cursor
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: PAC-R1
Impact: material
Enforcement: blocking
Basis: the human governor's exact request is to make “stop timers; poll and take work; when ready, rearm timers” one AGF-level skill or command; the canonical polling cadence also requires an inactivity window and evidence-backed last-seen state
Finding state: open
Risk disposition: none
Resolution condition: define one invokable lifecycle with explicit cadence and idle-window inputs; detect and pause/update an existing assistant wake-up before the manual cycle so it cannot overlap; load and persist a durable project-local cursor for coordination, MRs/issues/notes, exact SHAs and pipelines after the cycle; rearm only after a terminal outcome using the real assistant scheduler; state the safe fallback when the surface cannot inspect or update timers

The current skill starts at “note last-seen” and ends at “schedule the next cycle.” It neither pauses an already-armed wake-up nor requires durable cursor persistence, and it leaves cadence/idle values implicit. That is a useful one-cycle checklist, but it is not yet the single operator action requested in this session and cannot replace the long automation prompt without losing overlap and inactivity-window behavior.

## Finding PAC-R2 — destructive ref refresh is prescribed without a fail-closed preflight

Check: poll-checkout-refresh-safety
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: PAC-R2
Impact: material
Enforcement: blocking
Basis: `agf/rules/workflow/branch-and-worktree-rule.md`, repository worktree boundaries, and the framework's destructive-action safety expectations
Finding state: open
Risk disposition: none
Resolution condition: remove the bare `git reset --hard origin/<branch>` recommendation, or surround it with an explicit dedicated-poll-worktree identity and clean-state verification that fails closed before any reset; prefer a detached disposable worktree created at the fetched SHA under the project's allowed worktree root and never mutate an operator's persistent checkout

The word “dedicated” is not a mechanical safeguard. Copied into an adopter project, the current command can discard uncommitted or wrong-checkout work—the exact class of state loss a reusable governance skill must prevent.

## Advisory PAC-A1 — preserve the current CI-cost behavior explicitly

The live governed poll also detects same-SHA duplicate paid CI, preserves the canonical acceptance path, cancels only demonstrably redundant pipelines, and records pipeline/job IDs plus consumed time. Add this to Step 2/4 or explicitly state it is outside the skill's scope. Omitting it while replacing the long automation prompt would regress a resource-pressure control the governor has repeatedly required.

## Independence disclosure

Claude authored the plan and skill. This Codex session authored neither and independently reviewed the exact head. The review does not grant Gate 1, Gate 2, merge, release, or deployment authority.
