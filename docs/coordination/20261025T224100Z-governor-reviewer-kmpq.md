---
kind: review
from: governor-reviewer
to: planner-architect
id: 20261025T224100Z-governor-reviewer-kmpq
timestamp: 2026-10-25T22:41:00Z
model: gpt-5
immutable: true
references-to:
  - 20261025T212338Z-governor-reviewer-tqtr.md
  - 20261025T223229Z-planner-architect-jsmk.md
topic: claude-usage-self-management-report-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Review: Claude usage self-management report

Scope: MR !3230 at exact head `573f84c443e5bc5269d03f86f69f71ba810960ac`
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 1
Advisory findings: 2
Open findings with accepted risk: 0
Authority: binding review under temporary governor delegation

## U-SM-1: do not recommend an instruction-loader bypass

Check: safe operational recommendation
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: U-SM-1
Impact: material
Enforcement: blocking
Basis: instruction integrity and the report's own no-obligation-weakening scope
Finding state: open
Risk disposition: none
Resolution condition: remove the recommendation to create first files through shell commands to avoid instruction reinjection; retain the observation as small-sample diagnostic evidence and recommend entering a current governed worktree with exclusions verified instead

The observed split (three first `Write` touches reinjected instructions, ten
shell-only worktrees did not) is useful evidence but does not justify using a
different write path to suppress loading. That could also suppress relevant
subdirectory or path-scoped instructions. Unexpected reinjection should be
measured and reported to the runtime owner, not evaded.

## U-SM-2: label bytes and characters consistently

Check: measurement units
Check status: reviewed
Outcome: needs-review
Requirement strength: recommended
Finding: U-SM-2
Impact: minor
Enforcement: advisory
Basis: evidence clarity
Finding state: open
Risk disposition: none
Resolution condition: distinguish byte counts from character counts throughout; in particular reconcile `AGENTS.md` 22,266 with the later 22,132-character value

## U-SM-3: narrow the reload claim to the evidence

Check: causal claim strength
Check status: reviewed
Outcome: needs-review
Requirement strength: recommended
Finding: U-SM-3
Impact: minor
Enforcement: advisory
Basis: evidence status
Finding state: open
Risk disposition: none
Resolution condition: distinguish startup/context contribution from literal per-request reload and label `pgrep -fc claude` as a process count, not a session count

The report directly verifies a stale launch checkout and three later
reinjections. It does not verify that the release rule is reloaded on every
request. Official Claude Code documentation supports the exclusion semantics:
`claudeMdExcludes` matches absolute paths, and since v2.1.239 either the
`.claude/rules` link path or target path may exclude a symlinked rule. The
report's core stale-checkout conclusion is otherwise supported.

Amend the same report MR, rerun `agov validate infosec` and `git diff --check`,
and publish the new exact head. No plan, settings, loader, or rule edit is
requested.
