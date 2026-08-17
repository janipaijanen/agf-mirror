---
kind: decision-record
from: governor-reviewer
to: all
id: 20260817T224145Z-governor-reviewer-qt32
timestamp: 2026-08-17T22:41:45Z
model: human-governor
immutable: true
references-to:
  - 20260817T222215Z-coder-tester-845r.md
  - 20260817T223214Z-contribution-auditor-gmgs.md
topic: gate1-operational-debt-and-commercial-licensing
requires-ack: false
artifacts_owned: []
---

# Gate 1 approved for operational-debt Phase 0 and commercial-licensing recovery

This binding decision is transcribed by Codex at the governor-reviewer's
explicit instruction in the active conversation: “both gate 1s are approved.
please proceed. (You write the record of approval and continue)”. The decision
and authority are the human governor-reviewer's; Codex is only recording the
instruction in the durable coordination channel.

## Approved exact heads

1. **Operational-debt Phase 0** — MR !1718, plan
   `docs/plans/20260817T210510Z-operational-debt-phase0-measurement.md`, exact
   head `ff3458266ad186051500a7d1a93a6eb2035de7b9`. Gate 1 is approved.
   Coder-tester review `845r` is CLEAR and exact-head pipeline `2767346525`
   passed.
2. **Commercial-licensing recovery** — MR !1723, plan
   `docs/plans/20260817T210816Z-commercial-licensing-gate1-recovery.md`, exact
   head `0faf002b325ba5032bcb85e8103ded68c620908f`. Gate 1 is approved.
   Independent contribution-auditor review `gmgs` is CLEAR and exact-head
   pipeline `2767379168` passed.

## Authorized work

- Coder-tester may begin the operational-debt plan's two-snapshot Phase 0
  measurement exactly as approved. This authorizes Snapshot 1 now and
  Snapshot 2 no earlier than 24 hours later, using stable item identities and
  read-only classification. It does not authorize new `agov` tooling,
  enforcement, automatic MR closure, or branch deletion.
- Coder-tester may begin the commercial-licensing plan's bounded technical
  rights/provenance inventory at one named exact revision. The output must use
  the approved evidence contract and candidate-set framing. Contribution-
  auditor review of that inventory must satisfy `6srq` actor/session
  independence.

## Decisions still reserved

- **CL-5 naming remains undecided in this instruction.** No exact legal
  Licensor name or canonical product name/abbreviation was supplied. Work
  dependent on CL-5, including the corresponding mechanical corrections to
  MR !1712, remains paused until the governor records those values.
- **CL-3 remains deferred** until the rights/provenance inventory is complete,
  independently reviewed, and routed to the governor/counsel.
- Gate 2, merging, any licence offer, and any agreement execution remain
  separately governed and are not approved by this record.
