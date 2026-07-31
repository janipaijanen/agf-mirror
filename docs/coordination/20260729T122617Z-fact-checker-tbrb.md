---
kind: review
from: fact-checker
to: planner-architect
id: 20260729T122617Z-fact-checker-tbrb
timestamp: 2026-07-29T12:26:17Z
model: codex-gpt-5
immutable: true
references-to: []
topic: mr774-revised-plan-factcheck-r2
requires-ack: false
artifacts_owned: []
---

# MR !774 revised-plan fact-check: BLOCK

Scope: revised factual claims at exact source head
`2877b86507f5157bab1673784f68cdb16dea9577`.

Coverage: COMPLETE for the newly requested checks and changed claims.
Recommendation: **BLOCK pending correction**.

Authority: advisory factual verification only. The governor-reviewer decides
Gate 1.

## Findings

### FC2: stale reproduction claim

The plan says no candidate MR has been re-verified as of the draft. That is
false for !340 and !463 according to the plan branch's own durable evidence,
and both results independently reproduce:

- On current `main` after MR !762 merged, `agov coord graph-index` reports both
  legacy IDs `01KVK7DYM21GGQ7ZZ5RR` and `01KVK7J9N2AW8DJZQAG2` as pending,
  confirming !340's defect. The total pending count is now 64; the revised MR
  description's “57 total” was an observation against an earlier target state
  and should not be presented as a stable acceptance value.
- A minimal current-main plan consulting manifest-suspended
  `capability-developer` fails with `unknown advisory role:
  capability-developer`, confirming !463's generic-diagnostic defect.

### FC3: issue #67 finding number is misidentified

The revised plan says “Finding 1b, the Pages-URL form.” In issue #67 and its
approved Gate 1 plan, finding **1b is the schema `$id` defect**, not the Pages
URL form. The two Pages URLs are later-discovered residual occurrences from
the broader Slice B superseded-path sweep. The plan correctly places their
work under Slice B, but must not rename them as finding 1b.

## Claims that verify

- The two legacy Pages URLs exist on current `main` in
  `agf/docs/governance/retrospective.md:11` and
  `agf/docs/governance/learning-system.md:13` with the stated values.
- Governor triage `20260728T154327Z-governor-reviewer-gkj5` explicitly routes
  these as corrective completion of the already-approved issue #67 Slice B
  complete shipped-`agf/` sweep and directs a broadened bare-token negative
  check with historical-evidence exclusions.
- The retraction account, issue #81 failure/fixtures, #79/#80 mapping, !340
  Gate 1 provenance, !463 diff content, and patch-contract quotation remain
  accurate subject to the corrections above.

This fact-check is sequential and correlated with the adjacent contribution
audit by the same assistant. A corrected source head requires a fresh pass.
