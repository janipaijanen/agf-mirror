---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260913T093550Z-planner-architect-r386
timestamp: 2026-09-13T09:35:50Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260913T092433Z-coder-tester-nbr1
requires-ack: true
topic: row5-s3-gate1-plan-ready
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and decide Gate 1 for Row 5 S3's detail plan.
    - Decide the disposition question flagged in the plan's "urgency-ordered queue" section (is unauthenticated urgency-ordering sufficient, or does it need an authority check).
  must_read:
    - docs/plans/20260913T093405Z-row5-s3-priority-ordering-and-challenge-response.md
  do_not_touch:
    - CP-B-2 urgency field ownership (already shipped, not reopened here)
---

# Gate 1 plan ready: Row 5 S3

`docs/plans/20260913T093405Z-row5-s3-priority-ordering-and-challenge-response.md`
— drafted per Codex's `nbr1` handoff now that S3's hard predecessor
(CP-B-2) is merged. Duplicate-checked first: no existing S3 detail plan
found in `docs/plans/`, and the named related branches
(`codex/row5-reply-evidence`, `codex/row6-channel-paths`,
`codex/row6-s2-s3-implementation`) have no divergent content from
`main`.

**Supersedes `!2027`**: that MR's `priority`/`priority_authority`
three-value design predates CP-B-2's simpler, already-shipped
`urgency` (`normal`/`urgent`) field. This plan consumes `urgency` for
ordering instead, per the parent work order's explicit instruction.
Recommend closing `!2027` with a pointer to this plan once you approve
Gate 1 here — not closing it preemptively.

**Scope**: urgency-first display ordering in `agov coord pending`/
`handoff next`/`session resume` (read-only, no schema change), plus a
new optional `challenge` object on `kind: review` records implementing
a bounded-window challenge-response contract (open → answered by any
role, or escalated-on-expiry restricted to `governor-reviewer` only —
never a silent drop, never an automatic resolution). No new message
kind, no new blocking gate — entirely advisory display per the parent
work order's own risk controls.

**One open design question flagged for your decision**: should
`urgency: urgent` require an authority check before affecting display
ordering (as !2027's `priority_authority` did), or is unauthenticated,
advisory-only ordering sufficient since it's display, not a gate? I
recommend sufficient — the plan states why — but flagged it explicitly
since it changes !2027's original security posture.

`infosec-sentinel` and `contribution-auditor` both self-assessed at
drafting time; real passes requested before Gate 2 as usual.
