---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260910T070211Z-planner-architect-p2rk
timestamp: 2026-09-10T07:02:11Z
model: claude-sonnet-5
immutable: true
topic: correction-urgency-field-owned-by-cpb2-not-row5-work-order
references-to:
  - 20260910T065817Z-planner-architect-g1g5.md
  - 20260909T133651Z-cpb-coordination-publication-efficiency.md
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Correction: the `urgency` field is owned by CP-B-2, not the Row 5 work order

## What was wrong

`g1g5` and the first-pushed draft of the coordination-lifecycle-semantics
work order (`20260910T065635Z`, MR !2658) stated the `urgency` field is
"defined once here (S3), consumed by CP-B-2". That has the ownership
reversed.

## Correct allocation

The **Gate-1-approved** CP-B work order
`docs/plans/20260909T133651Z-cpb-coordination-publication-efficiency.md`
decomposition already assigns the `urgency` enum field on
`agf/schemas/coordination-message.schema.json` (enum, optional, default
`normal`) to **CP-B-2**. That allocation stands.

The Row 5 work order's **S3** is corrected to:
- **not** define the `urgency` field;
- **consume** CP-B-2's `urgency` field for priority-ordering of
  `agov coord pending` / `agov handoff next`;
- add only the `challenge-response` obligation type on the schema;
- carry **CP-B-2 as a hard predecessor** (alongside `S2 → qbka Slice A`).

MR !2658's plan file and description are updated to this effect; no member
is demoted and no other part of the work order changes.

## Note

This reversal is itself a #123-class plan collision — two plans claiming
the same schema field — caught only by a manual read during drafting. It
is added to the row-6 (#123 + CP-C) evidence base.
