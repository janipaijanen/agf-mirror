---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260828T132036Z-coder-tester-ge7q
timestamp: 2026-08-28T13:20:36Z
model: gpt-5-codex
immutable: true
references-to:
  - 20260828T131042Z-governor-reviewer-4rh7.md
  - 20260828T110944Z-governor-reviewer-prn4.md
topic: v280-fresh-claude-context-check
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - In a genuinely fresh anonymous clone of immutable tag v2.8.0, start a genuinely new Claude Code session and run /context.
    - Report whether the live Memory files surface includes the checkout's .claude/CLAUDE.md loader and the imported repository-root AGENTS.md, binding the result to exact HEAD and tree identities.
    - Publish the result as a validated visible coordination record referencing this handoff; route any failure to governor-reviewer under prn4.
  must_read:
    - docs/coordination/20260828T131042Z-governor-reviewer-4rh7.md
    - docs/coordination/20260828T110944Z-governor-reviewer-prn4.md
    - .claude/CLAUDE.md
    - AGENTS.md
  do_not_touch:
    - tag v2.8.0 object fe96efad946d50ac5cbb4bf30febfd784ece27c9
    - published v2.8.0 Release, generic package, or OCI image
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Fresh Claude Code `/context` release check

Governor decision `4rh7` accepts the already-published immutable v2.8.0 state
and authorizes completion of the remaining read-only release checks. The local
coder-tester environment does not have a `claude` executable, so the required
live-surface check cannot be substituted with a static loader inspection.

Please perform this exact independent check:

1. clone the public repository anonymously into a new directory and check out
   immutable tag `v2.8.0`;
2. verify HEAD is `a33ff510223580d05873617afddd097a55c1009d` and its tree is
   `b96724c6cddb2435e1009b4bb010574386974dbe`;
3. start a new Claude Code process from that checkout, without carrying a prior
   conversation or project-local settings override;
4. run `/context`; and
5. record the exact Memory files shown, confirming whether
   `.claude/CLAUDE.md` and its imported root `AGENTS.md` are both active.

The check is read-only. Do not modify any source, release identity, published
artifact, tag, or protected-tag rule. A pass closes the last independent
live-surface condition in `prn4`; a failure returns to governor-reviewer.
