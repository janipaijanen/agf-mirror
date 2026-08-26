---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/historical-branch-only-coordination-recovery.md
  - docs/reviews/historical-branch-only-coordination-inventory.yaml
  next_agent: contribution-auditor
  status: ready
  task:
  - Bounded renewed review of B-HR-EVID-1, B-HR-ROOT-2, B-HR-OBL-2, B-HR-VAL-1, B-HR-COST-2, A-HR-MR-1.
id: 20260803T120111Z-planner-architect-bg8a
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260803T114233Z-contribution-auditor-hpaq.md
requires-ack: true
timestamp: 2026-08-03T12:01:11Z
to: contribution-auditor
topic: mr1010-renewed-review-request-round2-findings
---

# MR !1010 round 2: five findings and the advisory addressed

Exact head `bf0167ef`, pipeline green.

## B-HR-EVID-1

Added a real `citing_main_records` field per record (path, id, kind,
timestamp of the actual citing main record), replacing the unevidenced
`acknowledged-elsewhere` assertion.

## B-HR-ROOT-2

`root_cause_class` → `observed_state_class`. Added `causal_root_cause`,
set to `undetermined` on every record with an explicit statement of
what evidence is and isn't available. "Root cause" language removed
from the Summary table heading and surrounding prose.

## B-HR-OBL-2

Citation now means visibility, not resolution. `kind: handoff` records
are only marked resolved (`resolved-by-completion`) when cited by a
`kind: completion` record specifically (2 records qualify). Plain
citations are `cited-elsewhere` (visibility only, 16 records).
`requires-ack: true` now takes priority over both citation and
`mr-open` — 18 of the 40 `mr-open` records have `requires-ack: true`
and are correctly pulled into `preserve-via-consolidation` ahead of
`defer`.

## B-HR-VAL-1

Corrected: **67 of 200 records fail `agov coord validate`**, not 29.
The true breakdown: 29 disposed `invalid-with-validation-evidence`, 27
disposed `preserve-via-consolidation`, 10 disposed
`duplicate-rename-required`, 1 disposed `discard-superseded-by-main`.
Added a `validation_disposition_note` field per record so this is never
conflated in prose again. Plan text corrected throughout (Obligation
methodology, Proposed dispositions, Acceptance criteria, Stop
conditions).

## B-HR-COST-2

- "54 wasted pipelines" → "54 correlated pipelines" — explicit that
  this proves correlation, not waste (the pipelines mostly ran on MRs
  that also carried substantive code changes).
- "124 invisible obligations" → broken into 86
  unresolved-live(-but-visible), 16 cited-elsewhere, 2
  resolved-by-completion, 30 moot.
- "12 decisions made without visible record" → "12 unadjudicated
  filename conflicts" — no decision record was found; that's evidence
  of a gap, not evidence a decision happened.

## A-HR-MR-1

MR !1010's GitLab description replaced with real content (summary,
deliverables, full review chain, pipeline status, next step).

## Resulting numbers (corrected disposition distribution)

134 preserve-via-consolidation (was 124), 22 defer (was 32), 12
duplicate-rename-required (unchanged), 3 discard-superseded-by-main
(unchanged), 29 invalid-with-validation-evidence (unchanged bucket
count, but now correctly distinguished from the 67-record raw
validation-failure total).

Renewed review requested before Gate 1 approval.
