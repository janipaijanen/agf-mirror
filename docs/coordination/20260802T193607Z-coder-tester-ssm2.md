---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260802T193607Z-coder-tester-ssm2
timestamp: 2026-08-02T19:36:07Z
immutable: true
references-to:
  - 20260730T131404Z-contribution-auditor-rg8h.md
  - 20260728T161124Z-governor-reviewer-38ge.md
  - 20260728T154327Z-governor-reviewer-gkj5.md
topic: "Gate 1 decision requested: MR !805/!806/!807 (issues #84-90, CLEAR since 2026-07-30, unchanged); MR !745/!746 amended per outstanding ADAPT requirements"
requires-ack: false
artifacts_owned: []
model: claude-sonnet-5
---

# Gate 1 backlog: three CLEAR plans awaiting decision, two ADAPT plans amended

While confirming there was no ready-to-implement work to run in parallel
with the WP1/WP2/CI-fix Gate 2 review already in flight (MR !947-!951), I
found five open Gate 1 plan MRs that have been sitting since 2026-07-28/30
with no governor decision yet. Two had exact, already-specified required
adaptations that were never applied; I applied them. Three are simply
waiting.

## Ready for a decision now: MR !805, !806, !807 (issues #84-90)

Contribution-auditor's final review (`20260730T131404Z-contribution-auditor-rg8h.md`)
gave all three a CLEAR with zero open blocking or advisory findings:

- MR !805 (issues #84, #85, #89 — coordination read-path reliability) at
  `295ea1c40a2f83c91d0b1221569514b18f1a8e70`, pipeline `2718579451` passed.
- MR !806 (issues #86, #88 — agov-mcp transport protocol correctness) at
  `3e86215a50faa7bbf08255864fc9abf6ffa54d26`, pipeline `2718091294` passed.
- MR !807 (issues #87, #90 — assistant discovery and archive setup) at
  `63222f267e9bf187c5af63387b1cc822ff4130a1`, pipeline `2718105392` passed.

I re-verified today (2026-08-02T19:1{9,9,9}Z): all three MRs' current
`sha` and `head_pipeline` still exactly match what was reviewed — nothing
has changed since the CLEAR. These are "future v2.5.0 discovery plans"
per the review's own words, not named in the narrower v2.5.0-rc.1 scope
(`docs/plans/v250-rc1-contract-core.md`, WP1/WP2 only) — I am not
assuming they belong in the current release train. Flagging that a
binding Gate 1 decision is the only thing outstanding, whenever you want
to schedule this slice (now, alongside rc.1, or deferred to a later
minor) is your call, not mine to make by proceeding to implement.

## Amended and returned: MR !745 (issue #75), MR !746 (issue #76)

Both plans had a governor-specified adaptation requirement recorded on
2026-07-28 that was never applied (confirmed via `git log` — no commit
touched either plan file since drafting, three-plus days before this
note).

- **MR !745** (`6abef4991934dc09285422764bbb202415928e35`, was
  `3a7755001a271459dcaf75f69200c35067351980`): applied the exact one
  sentence specified in `20260728T161124Z-governor-reviewer-38ge.md`'s
  "Required adaptation" section, verbatim, after the Resolution semantics
  vocabulary list. No other change. `agov plan validate` passes.
  Returning this exact amended head for Gate 1 approval per that review's
  closing instruction.
- **MR !746** (`672294fcdd7770a99b8ce7e384a567ecdf9abcde`, was
  `b3eb9556c1c9fc93e26ffefb5ddd4be908a723e1`): routed the issue #78
  finding-4 branch-visibility adopter evidence into the plan as an
  acceptance/use-case input, per `20260728T154327Z-governor-reviewer-gkj5.md`'s
  task list, scoped explicitly to what the plan already covers (not a
  scope expansion). Also resolved the portfolio adjudication's
  "Conditional v2.5 RC work after #80/ref correctness" placement: #80 is
  now implemented (WP2, MR !948), and its ref-scoping model does not
  conflict with this plan. No change to Planned changes, Required tests,
  or Acceptance criteria. `agov plan validate` passes.

Both MRs' pipelines are running at the time of this note; I have not yet
confirmed green (will follow up separately or you can check directly —
MR !745 and MR !746 on GitLab).

## What I am asking for

Purely a scheduling/priority call, not a technical one — the plan content
itself was either already CLEAR or has had its named-blocking adaptation
applied. No implementation on any of these five has started, and none
will start without a Gate 1 decision-record.
