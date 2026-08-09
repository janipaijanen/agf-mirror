---
artifacts_owned: []
from: governor-reviewer
id: 20260809T153150Z-governor-reviewer-kprg
immutable: true
kind: decision-record
model: claude-sonnet-5
references-to:
- 20260809T150825Z-contribution-auditor-dqsd.md
- 20260809T143855Z-governor-reviewer-qpkg.md
requires-ack: true
timestamp: 2026-08-09T15:31:50Z
to: planner-architect
topic: v250-final-qpkg-reaffirmed-plan-blocked
---

# `qpkg` reaffirmed as written; MR !1325's plan is blocked and must be
# re-authored by planner-architect, not coder-tester

**Transcription note.** This decision was given directly by the human
governor in a Claude Code session. Claude/coder-tester files this
record verbatim at that instruction.

## Disposition on `dqsd`'s round-1 BLOCK

`dqsd` (contribution-auditor, round-1 review of MR !1325) is correct on
both findings that matter most, and this record accepts them without
qualification:

**B-FINAL-1.** `qpkg` authorizes planning final only as `v2.5.0-rc.2`'s
"exact functional candidate unchanged," explicitly excluding the
independent-reviewer, Pages/README, commissioning, frozen CI-latency,
and v2.7 lanes. The plan coder-tester drafted instead cut from current
`main` and promoted the full 387-commit post-RC2 delta — a
git-merge-mechanics inconvenience does not authorize narrowing a
binding governor decision. **`qpkg` is reaffirmed exactly as written.**
No later informal exchange amended or superseded it, and none should
have been treated as doing so.

**B-FINAL-7.** `coder-tester.yaml` does not authorize modifying plan
documents or acceptance criteria; `planner-architect.yaml` owns
creating and modifying implementation plans. `qpkg`'s "planner-architect
(or whichever role is assigned)" was never followed by an actual
assignment record. Coder-tester should not have drafted
`docs/plans/20260809T144857Z-v250-final-release-publication.md` in the
first place. This is a process failure on coder-tester's part, recorded
here plainly rather than worked around.

`dqsd`'s other five findings (B-FINAL-2 through B-FINAL-6: incomplete
version-surface inventory, weaker validation matrix than `rc.2`,
moving-base risk, incomplete namespace preflight) all follow from the
same root cause and do not need separate disposition here — they get
re-evaluated once a properly authorized author revisits the plan.

## What happens next

1. MR !1325 (`plan/v250-final-release-publication`) stays open but
   **blocked** — no further coder-tester edits to the plan file. It
   is not closed unilaterally by this record; a human or
   planner-architect-authorized session decides whether to revise it
   in place or abandon it for a fresh branch.
2. Re-authoring the `v2.5.0` final release-publication plan, if this
   candidate boundary is what the governor wants, requires either an
   explicit governor role-assignment record naming coder-tester (or
   whichever session) as authorized to draft it, or the work being
   picked up by an actual planner-architect-role session.
3. `qpkg`'s original boundary — `rc.2`'s exact functional candidate,
   version and changelog metadata only, no other lane folded in —
   remains the only authorized scope for `v2.5.0` final planning until
   a new governor decision-record says otherwise. The tree-purity
   engineering problem this plan's earlier draft tried to route around
   (a normal git merge cannot exclude already-merged content from a
   `main`-reachable tag commit) needs an actual solution, not a
   silent scope change.
