---
kind: decision-record
from: governor-reviewer
to: all
id: 20260826T042010Z-governor-reviewer-4ssr
timestamp: 2026-08-26T04:20:10Z
model: human
immutable: true
references-to:
  - 20260815T213022Z-governor-reviewer-tgjs.md
  - 20260825T200220Z-planner-architect-4dgq.md
  - 20260825T214621Z-coder-tester-s7rg.md
  - 20260826T035053Z-planner-architect-k157.md
  - 20260826T040121Z-coder-tester-jhdr.md
topic: mr806-rebased-plan-ratification-and-ci-correction-commission
requires-ack: false
artifacts_owned: []
---

# Binding MR !806 continuity disposition and CI-correction commission

This record transcribes Jani's explicit in-session governor-reviewer decision
after receiving the focused continuity evidence and proposed disposition:

> approved as proposed.. proceed.

## Rebased-plan continuity decision

The Gate 1 activation in `4dgq` did not automatically transfer from named
plan commit `77ccdb1b9505c5d33b43fda32f1a4d8a0f83516c` across the rebase to
plan commit `38c80c3b12e5985e8480807f68568d4037952bfd`. Exact-SHA authority must
not be inferred merely from similar content or from a later implementation.

The governor-reviewer nevertheless ratifies rebased exact plan head
`38c80c3b12e5985e8480807f68568d4037952bfd` and accepts the already-performed
MR !806 implementation sequence as a narrow corrective exception. The
decision rests on all of the following verified facts:

1. `git range-diff` maps every one of the eight original plan commits to its
   rebased counterpart with `=`;
2. the original and rebased aggregate plan deltas have the same stable patch
   ID, `3c22dffdc401d97436ec00cc171aa15ad4eb22ca`;
3. both exact plan heads contain the same final plan blob,
   `7663baf2c80fb03fcab009ade6d19350b7b7eeee`;
4. the rebase occurred before implementation, the plan still validates at
   implementation head `e321fa704a4163cbeb5f68b52a6c9652172bf9c2`, and
   implementation did not modify the plan.

This resolves coder-tester finding B-MR806-G2-4. It does not create a general
rule that approvals survive rebases. Future exact-head rewrites require a new
binding disposition before dependent implementation unless a governed rule
expressly supplies another continuity mechanism.

## Shared shallow-history CI correction

The governor-reviewer also commissions a separately scoped correction for
CI-HISTORY-1, the standard `rust:tooling:test` failure shared by MRs !2041 and
!806. Planning should prefer making complete history available only to the
job that runs the real pinned-history acceptance test, while comparing that
option against explicit pinned-revision fetches and a self-contained fixture.
The plan must preserve the historical-evidence semantics, address fork/MR
behavior and runner/network cost, and produce fresh corrected exact-head
acceptance runs for affected work.

This commission authorizes Gate 1 planning and review, not implementation
before plan approval. It is not a waiver of the failed job and does not
authorize retrying an unchanged paid job merely to seek green status.

## Boundaries

This decision does not approve Gate 2, merge, tag, release, deployment, scope
outside the two decisions above, or a CI failure waiver. MR !806 remains
blocked from Gate 2 presentation until the CI-history correction is governed
and fresh required exact-head acceptance evidence is available.
