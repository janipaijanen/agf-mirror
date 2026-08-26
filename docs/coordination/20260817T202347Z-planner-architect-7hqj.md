---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260817T202347Z-planner-architect-7hqj
timestamp: 2026-08-17T20:23:47Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: operational-debt-slo-collaborative-response
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Independently review the attached DD finding (R8/F7) and the
      companion learning record (id 20260817T202241Z-ookv, this MR),
      then co-propose a bounded response plan covering closure SLOs,
      bounded branch lifetime, a recurring drift-closure cadence, and a
      lightweight dashboard/ownership mechanism.
  must_read:
    - agf/learnings/foundry.yaml (entry 20260817T202241Z-ookv, this MR)
    - docs/plans/20260816T114725Z-open-mr-hygiene-disposition.md
    - tmp/dd-20260817/11-ChatGPT-Answer.md (sections F7, Prioritized Risk
      Register R8)
  do_not_touch: []
---

# Proposal: joint response plan for the operational-debt backlog finding (DD R8/F7)

## Why this is separate from issue #110

Issue #110 and its Gate 1 plan
(`docs/plans/20260816T114725Z-open-mr-hygiene-disposition.md`, MR
!1617) already exist and are working correctly on their own terms:
bounded, per-item, evidence-verified MR disposition, deliberately
narrow to avoid destroying real unpublished backlog. That plan's own
first round closed exactly one MR (!921) after a corrected
verification methodology. I am not proposing to change or replace it.

What issue #110 does not attempt, and nothing else in this project
currently attempts either, is bounding the *rate* of the queue itself.
An external due-diligence simulation run today
(`tmp/dd-20260817/11-ChatGPT-Answer.md`, finding F7 / risk R8) flagged
this directly: 81 pending handoffs, 8 unresolved required
acknowledgements past cutoff, 51 rule-copy drift findings, and 547
branches, with required treatment stated as "closure SLOs for
acknowledgements, bounded branch lifetime, drift-closure process, and
dashboards/ownership." I've filed this as learning record
`20260817T202241Z-ookv` in `agf/learnings/foundry.yaml` (same MR as
this handoff) rather than acting on it unilaterally — a queue-health
mechanism changes how both of us poll and prioritize, so it needs your
independent take before it becomes a plan, not just mine.

## What I'm asking for

Not a plan yet — a second, independent read on whether these four
pieces are the right shape, before either of us writes a Gate 1 plan
for it:

1. **Closure SLO for pending handoffs/required acknowledgements.** A
   maximum age (e.g. N hours/days) before an unresolved
   `requires-ack: true` handoff or a pending acknowledgement is
   surfaced for explicit governor attention, rather than aging
   silently into the kind of large queue the DD flagged.
2. **Bounded branch lifetime.** A maximum age after which a branch
   with no corresponding open, active MR is flagged (not
   auto-deleted — deletion stays governor-authorized per issue #110's
   own boundary) for an explicit abandon/keep call.
3. **Recurring drift-closure cadence.** The 51 rule-copy drift
   findings need a repeatable check, not a one-off remediation pass —
   what cadence, and does an existing validator (`agov validate`
   umbrella, or a dedicated drift check) already have the primitive
   this needs, or does one need to be built?
4. **Dashboard/ownership.** The lightest mechanism that gives a
   reader (human or agent, at cold-start) a queue-count-over-time
   signal without adding another thing that itself needs polling and
   maintenance. Candidates worth weighing rather than assuming:
   a periodic coordination-record snapshot (fits existing patterns,
   zero new tooling) vs. an `agov coord status`-style computed report
   (needs tooling work, but avoids manual snapshot drift).

## What I am explicitly not asking for

Not asking you to execute anything yet. Not asking for a Gate 1
submission from either of us in this handoff — that's the next step,
after we've reconciled independent views, same discipline as issue
#110 used (rmn4/jtm6 review rounds before Gate 1 activation). If you
think this whole framing is wrong — e.g. that a formal SLO mechanism
is itself over-engineering for a single-governor project per
`workflow/simplicity-rule.md` — say so directly; that's exactly the
kind of pushback this handoff wants, not a delay.

Reply with your independent read (agree with this shape / propose a
different shape / argue no new mechanism is needed at all), and I'll
fold it into whichever of us drafts the actual Gate 1 plan.
