---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20261025T212338Z-governor-reviewer-tqtr
timestamp: 2026-10-25T21:23:38Z
model: gpt-5
immutable: true
references-to:
  - 20261025T181659Z-governor-reviewer-q88m.md
topic: claude-usage-self-management-investigation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Investigate which Claude usage controls can be enforced autonomously and explain why the release rule still appears in the reported per-request cost.
  must_read:
    - docs/plans/20261022T205113Z-instruction-context-reduction.md
    - docs/plans/20261023T080431Z-instruction-context-followup.md
    - .claude/settings.json
    - .claude/rules/INDEX.md
  do_not_touch:
    - .claude/settings.local.json
    - agf/rules/workflow/release-via-merge-request-rule.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Investigate Claude usage self-management

The Human Governor supplied Claude's seven-day usage contributors for the
AGF/ACS work: task switches without `/clear` (89%), sessions left running for
eight hours or more (68%), the release rule at about 10k tokens per request
(39%), up to 140 concurrent sessions (20%), and simple subagents using an
expensive model (14%). Percentages overlap.

Determine, with current-runtime evidence, which controls Claude can apply by
itself and which require a human, client setting, scheduler, or product change:

1. clear or replace context at a task/role boundary;
2. stop completed and idle sessions, including a practical maximum lifetime;
3. cap concurrent sessions and avoid ownerless work;
4. select a cheaper model for mechanically bounded subagents; and
5. reduce instruction cost without weakening governed obligations.

Start by resolving the apparent contradiction in the evidence. Current
`origin/main` already lists
`.claude/rules/workflow/release-via-merge-request-rule.md` in
`claudeMdExcludes` and exposes it through one INDEX trigger. The approved
follow-up plan records 27 loaded files and about 136.2k on-disk characters.
Identify the exact checkout/ref and settings layers behind the new statistic,
and capture `/context` or `InstructionsLoaded` evidence showing whether the
40.3k canonical rule was loaded at startup, opened because a release trigger
fired, retained by a stale long session, or loaded because symlink exclusion
did not match on that Claude Code version.

Report a compact matrix: control, autonomously enforceable (yes/no/partial),
mechanism, evidence, failure mode, recommended default, and measured expected
benefit. Include a proposed session lifecycle policy only if the runtime can
actually execute it; do not claim `/clear`, session termination, concurrency
limits, or model routing are autonomous without a demonstrated control.

Do not edit the shipped canonical release rule under this handoff. Shrinking it
would change adopter-facing governance, requires a separate Gate 1 plan and
contribution audit, and will not reduce startup context if the existing
exclusion works. If evidence still supports canonical slimming, return a
scoped plan proposal preserving every modal, negation, authority boundary,
failure condition, exact identifier, and test obligation, with a measured size
target and projection manifest.

This handoff authorizes investigation and proposal only, not implementation.
