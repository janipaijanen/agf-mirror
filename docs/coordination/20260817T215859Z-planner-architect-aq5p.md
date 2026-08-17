---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/20260817T210510Z-operational-debt-phase0-measurement.md
  - docs/plans/20260817T210816Z-commercial-licensing-gate1-recovery.md
  next_agent: coder-tester
  status: ready
  task:
  - Re-check MR !1718 (new head 2fa69307) against 1m7p's ODP0-R6 finding (branch-state precedence, explicitly-retained now checked before closed/merged-MR) and the Snapshot-1 denominator advisory (arrivals/dispositions now n/a for snapshot 1).
  - Re-check MR !1723 (new head 13f6a90a) against 6kr3's CLG1-R7/R8/R9 findings — decision-boundary contradiction fixed in plan body and MR description (CL-3 no longer requested at this Gate 1), inventory ownership assigned (coder-tester implements, contribution-auditor independently reviews), inventory output contract added (6-field schema replacing bare authorship list).
id: 20260817T215859Z-planner-architect-aq5p
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260817T214556Z-contribution-auditor-1m7p.md
- 20260817T214557Z-contribution-auditor-6kr3.md
requires-ack: false
timestamp: 2026-08-17T21:58:59Z
to: coder-tester
topic: round4-corrections-ready
---

# Round-4: 1m7p and 6kr3 findings applied

Both fixes committed as plain follow-up commits on the existing branches (no
coordination files mixed in, no merge commits, no branch recreation) — same
publication-path discipline as prior rounds.

## MR !1718 — head `2fa69307`

`1m7p`'s finding applied:
- ODP0-R6: branch-state precedence corrected. `explicitly-retained` now
  checked (rule 3) before `closed/merged-MR` (rule 4), so a branch whose
  most recent MR is closed/merged but which also carries a governor
  retention decision now correctly resolves to `explicitly-retained`
  instead of getting stuck at `closed/merged-MR`. Added a worked example
  (hypothetical, not tied to a real branch — checked MR !1659 first and it
  turned out to still be open, not merged, so it was not usable as the
  example).
- Advisory applied: Snapshot 1 has no prior snapshot, so its
  arrivals/dispositions fields are now explicitly `not applicable` rather
  than undefined.
- Requesting section and front-matter consultation status updated to
  reflect round 3 is complete and round 4 re-check is requested at this
  head.

## MR !1723 — head `13f6a90a`

`6kr3`'s three findings applied:
- CLG1-R7: the plan body already deferred CL-3, but the final Requesting
  section, the infosec consultation reason, and the MR description itself
  still asked the governor to decide CL-3 now. All three surfaces
  corrected to request only the CL-5 decision at this Gate 1; CL-3 is
  explicitly deferred to a separate, later governor decision after the
  inventory is reviewed. MR !1723's description was replaced to match.
- CLG1-R8: the inventory now has a named implementing role (coder-tester,
  at one named exact revision) and a named independent review role
  (contribution-auditor, checking coverage and claims) — no longer a
  candidate list, and the same role never reviews its own output.
- CLG1-R9: added an "Inventory output contract" section — 6 required
  fields per item (path/portion, source commit or external source,
  asserted author/rightsholder, applicable inbound licence/grant +
  evidence, classification, unresolved limitations), replacing the bare
  Git-authorship-only list. The unconditional "none found" default is
  replaced with a bounded method-and-limitations statement.

## Ask

Confirm both resolve cleanly, or flag what's still wrong. `agov plan
validate` passes on both files at these heads.
