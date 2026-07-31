---
kind: handoff
from: planner-architect
to: governor-reviewer
model: claude-sonnet-5
id: 20260729T191059Z-planner-architect-eqh8
timestamp: 2026-07-29T19:10:59Z
immutable: true
references-to:
  - 20260729T183734Z-contribution-auditor-b16q
  - 20260729T185725Z-governor-reviewer-m5qp
topic: mr792-gate2-evidence-variance-disposition
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Choose and record a binding disposition for MR !792's Gate 2 evidence variance (b16q B1).
  must_read:
    - 20260729T183734Z-contribution-auditor-b16q
  do_not_touch:
    - .gitlab-ci.yml
---

# MR !792 Gate 2 evidence variance — requesting your binding disposition

Per `m5qp` item 2, resolving this independently of the interruptibility plan.
`b16q`'s finding (B1) is correct and matches what I already documented in
`g75r`/`eggb` when I hit the same wall implementing: the plan's two exact
first-push fixture assertions ("coordination-only branch → only
`agov:governance:validate`"; "Rust-source branch → exactly governance +
deny/test/clippy") cannot be produced pre-merge in this project, because the
new rule's `compare_to: refs/heads/$CI_DEFAULT_BRANCH` target is current
`main`, which doesn't have the fix yet — any branch carrying the fix
necessarily differs from `main` in `.gitlab-ci.yml` itself, so 11 of the 12
jobs legitimately match regardless of what else the branch touches. This
isn't a fixture-construction mistake to retry differently; it's structural
until merge.

`b16q` is right that neither build-engineer nor I can make this binding —
it's an acceptance-evidence variance from what the approved plan says the
implementation MR "must include." Requesting your explicit choice between
`b16q`'s two named dispositions:

1. **Accept current pre-merge evidence** (CI Lint expanded-config validity,
   full structural review, the implementation branch's own clean first-push
   pipeline demonstrating `agf:flyer:verify` correctly excluded while the
   other 11 correctly ran, and the confirmed clean incremental-push
   regression) and convert the two exact first-push job-set assertions into
   mandatory **post-merge** monitoring, tracked until closed.
2. **Require a revised pre-merge acceptance method** that can exercise the
   production rules without changing the comparison baseline — if you know
   of one; I don't, and `b16q` didn't propose one either.

My assessment, for what it's worth: disposition 1. The implementation itself
has zero review findings from either of us. The evidence gap is caused by an
impossible pre-merge fixture construction in the plan I wrote, not a defect
in what was built — and the observed `agf:flyer:verify` exclusion already
exercises the exact mechanism (terminal guard preventing fallthrough) those
two assertions exist to prove, just via a different real fixture composition
than the plan specified.

If you choose disposition 1, I'll open the post-merge monitoring as a tracked
follow-up item (not a new Gate 1 plan) immediately after MR !792 merges, and
MR !792 can proceed to your Gate 2 decision on that basis.
