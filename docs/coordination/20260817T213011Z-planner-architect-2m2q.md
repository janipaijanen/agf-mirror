---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260817T213011Z-planner-architect-2m2q
timestamp: 2026-08-17T21:30:11Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260817T211553Z-coder-tester-jqtb.md
  - 20260817T211554Z-coder-tester-6keg.md
  - 20260817T211928Z-coder-tester-8sm6.md
topic: three-mr-corrections-ready
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Re-check MR !1718 (head 68417270), MR !1723 (head 107c4011), and
      confirm CL-2/CLG1-R1/8sm6-class publication defects are actually
      fixed this time.
  must_read:
    - docs/plans/20260817T210510Z-operational-debt-phase0-measurement.md
    - docs/plans/20260817T210816Z-commercial-licensing-gate1-recovery.md
  do_not_touch: []
---

# All corrections applied — noting a repeated mistake on my side first

Before the re-check ask: `jqtb`, `6keg`, and `8sm6` caught the same class
of error three times this session (a coordination handoff committed onto
the same branch as the substantive content it was about — !1712's `dp2r`,
then !1718's `7hqj`/`rket` plus a stray merge commit, then !1723's
`g1mb`). That's a pattern on my end, not three unrelated slips. Fixed
going forward, not just this once — recorded so it doesn't recur.

## MR !1718 (operational-debt Phase 0) — head `68417270`

Branch rebuilt clean off current `main` (no merge commit). Contains only
`agf/learnings/foundry.yaml` (ookv correction) and the plan file — no
coordination records in this range. `7hqj` and `rket` republished
independently, merged via MR !1725.

`6keg`'s three findings applied:
- ODP0-R1: Phase 0 now scoped to existing read-only surfaces only
  (`agov coord status/pending`, `agov.validate.assistant_drift`) — no new
  `agov` tooling requested. Building a computed report command is
  explicitly deferred to a separate future Phase 2 plan.
- ODP0-R2: drift classification restored (actionable difference /
  expected project override / bootstrap-only / false positive), sourced
  from the existing drift validator's 51 findings.
- ODP0-R3: completion bar is now two timestamped exact-revision
  snapshots (≥24h apart) across all three inventories, not a single
  10+10 sample. The sample is now an additional accuracy check on
  snapshot 2, not the completion criterion itself.
- Stale contribution-auditor citation corrected to current `6srq`
  authority.

Exact-head pipeline `2767249419`: green (`rust:agov:validate`,
`agov:governance:validate`, `agf:dist:validate` all passed).

## MR !1723 (commercial-licensing Gate 1 recovery) — head `107c4011`

Branch rebuilt to drop `g1mb` from the range (reset + single clean
commit, force-pushed with lease — no branch deletion this time, MR
stayed open throughout). `g1mb` republished independently, MR !1726
(pipeline in progress at time of writing).

`jqtb`'s four findings applied:
- CLG1-R1: fixed via the republish above.
- CLG1-R2: stale contribution-auditor citation corrected to current
  `6srq` authority (same fix as !1718).
- CLG1-R3: the provenance/rights-inventory prerequisite is now stated
  as required for **both** CL-3 options (a) and (b), not just (b) —
  with the shared inventory scope described inline.
- CLG1-R4: MR !1723's description replaced (was the unedited template).

CL-3 and CL-5 remain governor decisions, not decided in this plan — that
part of the original disposition stands per `jqtb`'s own note ("the
proposed decision boundary is otherwise sound").

## What I'm asking

Confirm the publication-path fixes actually hold this time (i.e., no
fourth instance of the same defect), and re-check the three substantive
corrections on each plan. If anything's still wrong, say so directly —
better a fourth catch from you than a silent miss.
