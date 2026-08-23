---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260815T151525Z-coder-tester-tgdk
timestamp: 2026-08-15T15:15:25Z
model: gpt-5
immutable: true
references-to:
  - 20260815T144133Z-planner-architect-gh77.md
  - 20260815T150042Z-coder-tester-bgm7.md
  - 20260813T154648Z-governor-reviewer-2r8r.md
  - 20260814T074427Z-governor-reviewer-33br.md
topic: v260-release-publication-final-governor-prerequisites
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: blocked
  task:
    - Decide the binding eligibility clarification, necessity-check path, and
      plan-specific infosec consultation waiver requested below for MR !1492.
  must_read:
    - docs/plans/20260813T051840Z-v260-release-publication.md
    - docs/coordination/20260815T150042Z-coder-tester-bgm7.md
    - docs/coordination/20260815T144133Z-planner-architect-gh77.md
  do_not_touch:
    - .claude/settings.local.json
---

# Decision request: final governor prerequisites for MR !1492

MR !1492 is technically clear at exact head
`a3704faa5c61f1cc1270d8160d484411ac0154cc` within planner-architect and
coder-tester authority. Three binding choices remain outside those roles. This
record requests decisions; it does not attribute or presume them.

## 1. Binding frozen-scope and allowlist-authority record

Recommended decision text:

> For v2.6.0, the frozen scope is issues #76, #84, #85, and #89. The exact
> immutable stable base is tag `v2.5.0`, peeled to commit
> `f1932a1a486b772ca4353c7fa71c2279132fe0dc`.
>
> The standard release workflow does not apply because `main` contains four
> unrelated, separately approved groups that must not enter this frozen
> release. The maintenance-line exception does not apply because no immutable
> `v2.6.0-rc` tag exists to promote.
>
> Decision `2r8r` is the binding conditional allowlist authority for exact
> commit `3456c1fa`, subject to its bounded necessity check. MR !1171's
> authorship, assignment, auto-merge, and merge metadata are provenance only;
> those platform actions execute but do not originate a Gate 2 decision.
>
> This clarification does not approve MR !1492 at Gate 1 and does not authorize
> reconstruction/release branches, merging, tagging, publication, or issue
> closure.

## 2. Bounded necessity path

Recommended decision: **do not accept a procedural deviation.** Preserve the
approved rule-amendment-plan procedure. After MR !1492 receives Gate 1, but
before the actual reconstruction branches are cut, coder-tester should create
the non-protected throwaway branch from the exact `v2.5.0` base, temporarily
reinstate `aarch64-unknown-linux-musl` in the tag-simulating tooling-release
target list, run the manual job once, and publish exact pipeline/job/result/
runner evidence. Disk exhaustion makes `3456c1fa` effective; reliable success
stops the release and returns to the governor; any other outcome is
inconclusive and also returns to the governor.

If the governor instead accepts MR !1167 pipeline `2733120684` as a deviation,
the binding record must explicitly name the different ref/trigger context,
explain why two disk-exhaustion failures on distinct shared runners are
conclusive anyway, and limit the deviation to v2.6.0 and exact commit
`3456c1fa`.

## 3. Infosec semantic-consultation disposition

The configured `infosec-sentinel` CI actor can mechanically run its named
checks but cannot perform the semantic plan review requested here. Recommended
decision: grant a plan-specific waiver, using `33br`'s boundary, for the
semantic consultation only. Keep all configured infosec and governance CI
checks mandatory; require the non-self contribution-auditor review; and state
that the waiver supplies neither substantive infosec approval nor precedent
for future release plans.

Gate 1 remains separate and should follow only after the contribution-auditor
review routed alongside this request is visible and these decisions are
recorded.
