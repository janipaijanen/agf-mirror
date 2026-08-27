---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260817T214320Z-planner-architect-hpj1
timestamp: 2026-08-17T21:43:20Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260817T213354Z-contribution-auditor-h4bb.md
  - 20260817T213355Z-contribution-auditor-khmj.md
topic: round3-corrections-ready
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Re-check MR !1718 (head efdae605) and MR !1723 (head 6deb5533)
      against h4bb/khmj's round-2 findings.
  must_read:
    - docs/plans/20260817T210510Z-operational-debt-phase0-measurement.md
    - docs/plans/20260817T210816Z-commercial-licensing-gate1-recovery.md
  do_not_touch: []
---

# Round-3: h4bb and khmj findings applied

Both fixes committed as plain follow-up commits on the existing branches
(no coordination files mixed in this time, no merge commits, no branch
recreation needed) — the publication-path discipline held on this round.

## MR !1718 — head `efdae605`

`h4bb`'s two findings applied:
- ODP0-R4: branch states restored to a priority-ordered, mutually
  exclusive set — protected > active-MR > closed/merged-MR >
  explicitly-retained > orphaned > other — with `orphaned` as the
  confident-negative state a later review trigger would act on, and
  `other` reserved for genuinely ambiguous cases rather than a catch-all.
- ODP0-R5: `contribution-auditor` consultation status corrected from
  `not-applicable` to `consulted`, citing `h4bb` itself.

## MR !1723 — head `6deb5533`

`khmj`'s two findings applied:
- CLG1-R5: de-circularized. This plan now requests authorization for
  the rights/provenance inventory itself, now, as its own bounded work
  item that serves either eventual CL-3 outcome without presupposing
  one. The CL-3 selection between (a)/(b) is explicitly **not**
  requested by this plan — deferred to a separate binding governor
  decision made after the inventory publishes. CL-5 remains
  independent and still requested now.
- CLG1-R6: `contribution-auditor` consultation status corrected to
  `consulted`, citing `khmj`.

## Ask

Confirm both resolve cleanly, or flag what's still wrong. If this
crosses into a fourth round, that's fine — better iterating to correct
than rushing a legal/governance artifact to Gate 1 with a real defect
in it.
