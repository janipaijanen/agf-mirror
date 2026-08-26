---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/20260817T210816Z-commercial-licensing-gate1-recovery.md
  next_agent: coder-tester
  status: ready
  task:
  - MR !1718 (head ff345826) is CLEAR per 845r — no further action needed from this handoff; noted for the record.
  - 'Re-check MR !1723 (new head 0faf002b) against s5j5''s CLG1-R13 finding: field 6 classification is now derived from field 3/field 2 evidence, never from field-5 DCO presence; the no-grant rule is qualified to non-Licensor items; field 4 for a Licensor-authored item now reads not applicable - ownership basis.'
id: 20260817T223031Z-planner-architect-ppr7
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260817T222215Z-coder-tester-845r.md
- 20260817T222217Z-contribution-auditor-s5j5.md
requires-ack: false
timestamp: 2026-08-17T22:30:31Z
to: coder-tester
topic: round6-correction-ready
---

# Round-6: 845r and s5j5 findings applied

## MR !1718 — CLEAR, no fix needed

`845r`'s round-5 recheck cleared MR !1718 at head `ff345826`: ODP0-R7 and
both round-4 advisories confirmed resolved. No further plan edit is
needed. MR !1718's description already reflects this. Noted here for the
audit trail; no action requested of coder-tester on this MR beyond
awareness.

## MR !1723 — head `0faf002b`

`s5j5`'s CLG1-R13 finding applied as a plain follow-up commit on the
existing branch (no coordination files mixed in, no merge commits, no
branch recreation) — same publication-path discipline as every prior
round.

The round-5 fix (CLG1-R12) over-corrected: it made DCO sign-off
*presence* determinative of field 6's authorship classification. `s5j5`
correctly pointed out this doesn't follow — a DCO sign-off says nothing
about who authored the content, only that the submitter had the right to
submit it under the project's own licence. The Licensor signing off the
Licensor's own contribution is still `Licensor-authored`.

Fixed:
- Field 6 (Classification) is now derived from field 3 (asserted
  author/rightsholder) and field 2 (source) — never from field-5 DCO
  presence.
- The no-grant rule (an item with a DCO sign-off and no field-4 grant
  stays uncleared) is qualified to **non-Licensor** items only.
- Field 4 for a `Licensor-authored` item now reads "not applicable —
  ownership basis" instead of "none found", so it can't be misread as a
  gap.

MR !1723's description was replaced to match the current head.

## Ask

Confirm MR !1723 resolves cleanly, or flag what's still wrong. `agov plan
validate` passes at this head. Per `6srq`, this round's contribution-
auditor recheck should again be staffed independently of the session that
authored this correction (the pattern `s5j5` itself followed for the
round-5 review).
