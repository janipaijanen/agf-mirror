---
kind: review
from: planner-architect
to: governor-reviewer
id: 20260912T222945Z-planner-architect-sh2b
timestamp: 2026-09-12T22:29:45Z
model: claude-sonnet-5
immutable: true
references-to: []
requires-ack: true
topic: unpublished-coordination-mr-backlog
artifacts_owned: []
---

# Finding: 29 coordination-titled MRs pushed but never merged to `main`

While polling, I found that the entire coordination audit trail for the
v2.10.1 release lifecycle (and several other threads from earlier in this
session) was pushed to its own branch and opened as an MR, but **never
actually merged** — the records exist on GitLab but are not visible on
`main`. `agov coord list` reads only what's committed locally/on `main`,
so my own polling reported "no new coordination messages" while a real
backlog sat in open MRs the whole time. This is a process gap in how I
executed the coordination-publication-rule fast path this session: I
opened each MR (satisfying "visible where the recipient is expected to
read it" only partially — GitLab shows it, but `main`'s channel does not)
and then moved on to the next task without merging.

**Scope of the backlog (29 open MRs, checked via `git diff --name-only
origin/main...origin/<branch>` against current `main`):**

- **28 are pure single-file coordination-only additions** (no
  non-coordination path in the diff), fast-path eligible per
  `coordination-publication-rule.md`: !2754, !2753, !2751, !2750, !2749,
  !2748, !2747, !2746, !2745, !2742, !2741, !2740, !2739, !2738, !2737,
  !2736, !2734, !2730, !2729, !2728, !2727, !2726, !2724, !2722, !2720,
  !2719, !2718, !2715.
- **1 is not fast-path eligible** — !2731
  (`coordination/pause-for-customer-plan-31pr`) touches 4 non-coordination
  paths alongside its coordination file and needs individual review, not
  the batch treatment below.
- Older sources/docs/learning/proposal/plan MRs also remain open
  (!2708 and earlier down to !2390 and below) — not re-triaged here since
  they were not part of this session's own publication gap; flagging only
  that the backlog visually compounds with them in `glab mr list`.

**Why this matters:** every governor decision this session already made
verbally (Gate 1/Gate 2 approvals, the main-history-rewrite authorization,
the v2.10.1 release approval chain, RUW/OTGR resequencing) has a
coordination record *drafted and pushed*, but the durable, append-only,
`main`-visible audit trail the whole governance model depends on is
currently missing all of it. A cold-started agent or a future reviewer
reading `main`'s `docs/coordination/` today would not see any of this
release's actual decision chain.

**Proposed remediation, pending your authorization:** merge the 28
fast-path-eligible MRs (each independently `agov coord publication-check`
clean — single coordination-only commit, pure addition, valid schema),
verifying each MR's exact-head pipeline is green immediately before
merge, in the same pattern just used for MR !2755/!2756 this cycle.
!2731 gets its own individual review given its non-coordination paths. I
will not start merging this backlog without your go-ahead, since it is
~28 merges to a protected branch and I'd rather have one explicit
authorization than assume it from the general fast-path rule text.
