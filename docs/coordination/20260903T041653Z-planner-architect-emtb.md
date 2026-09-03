---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260903T041653Z-planner-architect-emtb
timestamp: 2026-09-03T04:16:57Z
model: claude-opus-5
immutable: true
references-to:
  - 20260902T064458Z-planner-architect-a7jr.md
  - 20260903T040339Z-coder-tester-tk2n.md
topic: plan-status-spelling-assessment
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Assess the plan-status spelling divergence in docs/plans/ against agf/schemas/gate1-plan.schema.json.
    - Decide whether the six non-enum spellings are data errors, or whether the enum is missing a legitimate lifecycle state.
    - Propose a correction and say whether it needs its own Gate 1; do not apply it from this handoff.
  must_read:
    - agf/schemas/gate1-plan.schema.json
    - docs/coordination/20260902T064458Z-planner-architect-a7jr.md
  do_not_touch:
    - .gitlab-ci.yml
    - release branches, tags, or published artifacts
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan-status spelling: assessment and correction proposal requested

Routed at governor direction. I raised this in `a7jr` and explicitly did not
take it; I am not working on it now.

## Correcting my own framing first

`a7jr` said "187 plan files … three different spellings of approval" and called
any status-derived queue unreliable. **That overstated it.** Measured again
today across 189 files:

| Value | Count | Schema-valid? |
|---|---|---|
| `approved` | 127 | yes |
| `pending-gate1` | 41 | yes |
| `complete` | 7 | yes |
| `active` | 4 | yes |
| *(absent)* | 4 | **no — `status` is required** |
| `gate1-approved` | 3 | **no** |
| `superseded` | 1 | **no** |
| `gate-1-pending` | 1 | **no** |
| `gate-1-approved` | 1 | **no** |

Schema enum: `pending-gate1`, `approved`, `active`, `complete`.

**Ten files, not 187.** 179 of 189 are schema-valid. A status-derived queue is
substantially more reliable than I implied — the defect is a bounded tail, and
saying otherwise discouraged using a signal that mostly works.

Confirmed the tail actually fails, rather than inferring it from the enum:

```
agf-v2-layout-s1.md                        FAILS
plan-validator-advisory-role-allowlist.md  FAILS
rc1-sequence.md                            FAILS
```

## The part I think is a real question, not cleanup

Six files carry non-enum values. Five look like plain spelling drift
(`gate1-approved`, `gate-1-approved`, `gate-1-pending` for `approved` /
`pending-gate1`).

**`superseded` is different.** The enum has `pending-gate1`, `approved`,
`active`, `complete` — and **no terminal state for a plan replaced by another**.
`plan-validator-advisory-role-allowlist.md` needed to express exactly that and
invented a value. That reads to me less like an author error than a missing
state: this project supersedes plans routinely, and today's `tk2n` explicitly
contemplates superseding !2269.

So the assessment splits:

1. **Data** — normalise five drifted spellings; supply `status` for four files
   missing a required field.
2. **Schema** — decide whether `superseded` should be added. If yes, that is a
   schema change with its own downstream-impact classification, and the one
   file stops being an error.

Deciding (2) first matters, because it changes whether that file is normalised
or validated.

## Why this is yours rather than mine

I found it and I have a view, which is exactly why the assessment should not
also be mine — and my count was wrong the first time, which is the second
reason. Independent measurement before any correction is worth more here than
speed.

## Bounds

Assessment and proposal only. Do not apply the correction from this handoff.
Please state whether it warrants its own Gate 1 — my expectation is yes for the
schema half, and plausibly a fast path for the data half if the schema half
lands first, but that is your call to make and the governor's to approve.
