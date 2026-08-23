---
artifacts_owned: []
from: governor-reviewer
id: 20260823T090334Z-governor-reviewer-g4h3
immutable: true
kind: decision-record
model: human-governor
references-to:
- 20260823T084601Z-contribution-auditor-qshk.md
- 20260822T202031Z-coder-tester-rbqm.md
requires-ack: false
timestamp: 2026-08-23T09:03:34Z
to: coder-tester
topic: issue110-recovery-batch-gate1-approved
---

# Gate 1 approved: issue #110 coordination-recovery and closure batch

This record transcribes the human governor's binding instruction in the
active session ("Gate 1 approved. coordinate work with codex"),
responding to contribution-auditor's CLEAR review `qshk` of
`docs/plans/20260823T082450Z-issue110-coordination-recovery-batch.md`
(MR !1782).

## Binding decision

Gate 1 is approved for the exact plan head reviewed by `qshk`:
MR !1782, `20545c37262dff9646868f053634bf992b60ab3f`. Coder-tester is
authorized to execute the plan's Steps 1–4:

1. Recompute every discovery predicate fresh against current `origin/main`
   immediately before construction (do not trust the plan's or `rbqm`'s
   cited numbers). Drop, do not repair, any MR that no longer matches.
2. Construct the recovery branch, copy the 61 surviving files byte-for-
   byte, `agov coord validate` the aggregate, publish with the
   `coordination:` tip-commit prefix.
3. Satisfy `qshk`'s advisory CA-HY2-A1 in the implementation evidence: an
   exhaustive deterministic manifest covering every surviving
   `MR -> path -> source commit/blob -> landed main blob` mapping, not a
   sample, with all rows required to match before any closure API call.
   This does not require a plan revision per `qshk`'s own resolution
   condition.
4. After the recovery merge is verified green on `main`, close the
   surviving source MRs with the standardized note, per the plan's Step 4
   fail-closed conditions (skip and record why for any MR whose state
   changed since Step 1's recomputation).

Request Gate 2 for the recovery branch's exact head plus the conditional
close operation as one combined decision, per the plan's own "minimal
resource use" framing.

## Boundaries unchanged

This decision approves Gate 1 for this exact plan only. It does not
authorize editing any recovered record's text, deleting any of the 51
source branches, closing any MR outside the exact reverified set, or
touching the remaining open MRs that are not coordination-only.
