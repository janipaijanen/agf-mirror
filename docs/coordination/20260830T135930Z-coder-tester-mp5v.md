---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260830T135930Z-coder-tester-mp5v
timestamp: 2026-08-30T13:59:30Z
model: claude-opus-5
immutable: true
references-to:
  - 20260830T135126Z-coder-tester-3rh6.md
  - 20260830T085834Z-governor-reviewer-dq5a.md
  - 20260830T101713Z-coder-tester-ssjg.md
topic: gate-change-discipline-ab-gate2-decision-requested
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide Gate 2 for MR !2237 at exact head a727bb96f7b236b07a35fc0e7aaca4d86ecb6e25.
    - Six independent review rounds; the sixth recommends CLEAR with zero findings.
    - MR !2242 (learning record) is held pending this decision and merges after it.
  must_read:
    - docs/coordination/20260830T135126Z-coder-tester-3rh6.md
    - docs/plans/20260829T211252Z-gate-change-discipline.md
  do_not_touch: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 decision requested: gate-change discipline A+B

MR !2237, exact head `a727bb96f7b236b07a35fc0e7aaca4d86ecb6e25`.
Gate 1 approved by your decision `dq5a`. Independent review `3rh6`
recommends **CLEAR**: zero blocking, zero advisory, zero accepted risks.
Canonical pipeline `2803335087` passed; required self-test job
`16190764052` reports 29 passed, 0 failed.

## What this ships

**A — the guardrail you asked for.** A gate could previously be weakened by a
one-line diff, called a bug fix, waived past Gate 1, and merged with no record
that a gate changed. `gate-self-test-rule.md` now fires on introducing **or
modifying** a gate, and `simplicity-rule.md` makes gate mechanics
fast-track-ineligible regardless of diff size. Mirrored into
`.claude/rules/`, and both adopter-facing surfaces that stated the
eligibility list in their own words are corrected so they no longer
contradict the rules they explain.

**B — the live defect that exposed the gap.** `agov:governance:validate`
could report success without validating anything, two independent ways: a
failed `git diff` masked by `xargs -0r`, and an empty base degenerating the
range to `HEAD...HEAD`. Both closed by a fail-closed contract. Deleted plan
paths no longer reach `plan validate` — the block on MR !2206. Coordination
records are protected against deletion **and rename** by an intentional check
naming the append-only rule. All four shipped adopter CI sketches carried
both defects and are corrected.

## Review history — six rounds, five defects

Every round attacked the previous round's fix and found a real defect in it.

| Round | Finding | Where |
|---|---|---|
| 1 (`13rh`) | MODIFY test was an ADD over its own range | the assertion |
| 1 (`13rh`) | rename-and-rewrite bypassed the append-only guard | the gate |
| 2 (`2shr`) | self-test lost schema context; validators never ran | the validator |
| 3 (`hna4`) | the control written to fix that accepted a feeder failure | the control |
| 4 (`gme3`) | the meta-test proving that control fires accepted a missing command | the meta-test |
| 5 (`3rh6`) | **nothing** | — |

All five were one class: **an assertion reaching its intended verdict for an
unintended reason.** Each walked one level up.

I flagged the pattern myself in the first handoff and asked reviewers to
assume more instances existed — and then repeated it three more times,
including once in a control I had *named* as the design's weakest point in
the same message that shipped it. Self-disclosure did not work as a
mitigation. Independent review did, every round.

**The reason it terminated is not mine.** `3rh6` supplied it: each mutation
assertion is paired with a positive invocation of the same helper in the same
run, so a broken helper makes the mutation assertion appear to pass while
failing its paired positive one and raising the failure count. No state of a
single helper satisfies both. That is a structural argument, not another
layer, and it is recorded in the learning record with attribution.

## Residual you accepted at Gate 1, restated

Until slice C lands (MR !2222, separate Gate 1), gate-change discipline is
enforced by rule and human review, not by mechanism. Nothing here changes
that.

## Sequencing

Acceptance criterion 12 — MR !2206 re-runs green and the governor-ordered
revert merges — is deliberately post-merge and cannot be demonstrated before
this lands. MR !2242 (learning `tw3s`, five instances) is held unmerged so a
record claiming `implemented` does not reach `main` ahead of the
implementation it describes; it merges immediately after your decision.

Gate 2 is yours. I have no further work on this MR.
