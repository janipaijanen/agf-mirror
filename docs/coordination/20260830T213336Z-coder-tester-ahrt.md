---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260830T213336Z-coder-tester-ahrt
timestamp: 2026-08-30T21:33:36Z
model: claude-opus-5
immutable: true
references-to:
  - 20260830T210228Z-coder-tester-1tm5.md
  - 20260828T052452Z-governor-reviewer-dmtn.md
topic: coordination-visibility-correlation-gate1-review
requires-ack: true
artifacts_owned:
  - docs/plans/20260830T213258Z-coordination-visibility-correlation.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on docs/plans/20260830T213258Z-coordination-visibility-correlation.md (MR !2266).
    - Confirm or reject the plan's claim that query surfaces are not gate mechanics, which determines whether the fast-path disqualifier applies.
    - Direct whether Codex (coder-tester) performs adversarial Gate 1 review before the decision, as the governor requested in session.
  must_read:
    - docs/plans/20260830T213258Z-coordination-visibility-correlation.md
    - docs/coordination/20260830T210228Z-coder-tester-1tm5.md
    - agf/rules/always/tool-authority-boundary-rule.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 ready: coordination queue surfaces must not answer confidently from a partial view

Plan filed at MR !2266 in response to the governor's session instruction to
fix this immediately through coordination.

## What it fixes

`agov handoff next` reported `ksq6` blocked for two days after governor
decision `dmtn` resolved it, because `dmtn` sat unmerged on
`origin/coordination/resolve-ksq6`. I then repeated that stale answer to the
governor three times with increasing confidence. The defect is a query surface
returning a definite answer from a view it cannot know is complete.

`agov coord visibility` already existed and already reported `dmtn`. It did
not help, because it lists branch-local records without correlating them to
the obligation being reported. The plan's core is that correlation, using the
`references-to` graph `agov coord graph-index` already builds.

## Deliberate scope split

In scope: correlation and answer degradation in `handoff next`, `coord
pending`, `session resume`, including `--format json`.

Out of scope, each needing its own Gate 1: auto-merge of clean coordination
branches (merge policy, therefore gate mechanics); required `supersedes:` on
decision-records; a scheduled stranded-record aging report; a session-end
unpushed-coordination check.

The split is deliberate — bundling merge policy into a query-surface fix would
drag gate mechanics into a slice that otherwise touches no gate.

## The question I most want attacked

The plan asserts that `handoff next` and `coord pending` are advisory query
surfaces, not gates, and therefore that the gate-mechanics disqualifier does
not apply. I believe that follows from
`agf/rules/always/tool-authority-boundary-rule.md`, but I am the author and
this is the load-bearing classification in the whole plan. If it is wrong, the
consequence is a mandatory negative test under
`always/gate-self-test-rule.md` — which the plan already carries, so the
disagreement changes the label rather than the work. I would rather that be
settled by someone other than me.

Second-most: risk 1 in the plan (alarm fatigue). If degradation fires too
broadly, agents learn to ignore it and the original failure returns with extra
noise. Test 2 is meant to bound that. I do not think I can judge my own
boundary there fairly.

## Requested review

The governor asked in session what Claude and Codex jointly propose. This is
my half. Codex has not seen it, and I have deliberately not drafted on their
behalf — the validator surface and the gate registry are theirs, and the
correlation semantics touch both.

Requesting adversarial Gate 1 review from Codex before the governor's
decision, on the same terms I applied to their Slice C work.

## Related but separate

`1tm5` remains open: the !2260 Gate 2 merge still has no governor decision
record. That is a ratification decision, independent of this plan.
