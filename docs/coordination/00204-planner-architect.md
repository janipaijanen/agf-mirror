---
kind: decision-record
from: planner-architect
to: all
seq: 204
timestamp: 2026-06-13T19:50:00Z
immutable: true
references-to: []
topic: agf-v2-single-directory-layout
requires-ack: false
artifacts_owned: []
---

# Decision: AGF v2.0+ Uses a Single Landing Directory for Downstream Adopters

**Governor decision, received 2026-06-13.**

## Decision

AGF v2.0 and later will ship as a **single directory** that downstream projects
drop in to adopt the framework. The current two-layout model (Layout A: governance
at repo root; Layout B: governance in a subdirectory) is retired for v2.0.

## What this means

- Adopters receive one canonical directory structure, not a layout choice.
- Distribution artifacts are scoped to that single directory.
- `ADOPTING.md`, `framework.md`, and rules-template documentation must be
  updated to remove Layout A / Layout B references and describe the single
  canonical layout instead.
- The docs restructure work (currently deferred) must account for this decision
  when the deferred lanes are resumed.
- Any tooling that generates or validates adopter layouts must target the single
  layout.

## What is not yet decided

- The exact name and contents of the single landing directory (to be specified
  in the docs restructure Gate 1 plan when resumed).
- Whether existing Layout B adopters get a migration path or whether v2.0 is a
  clean break.

## Impact on current work

- `feat/session-resume` and `feat/coord-visibility-scanner` are not affected —
  they are foundry-internal tooling, not adopter-facing distribution artifacts.
- The docs restructure deferred lanes (plan branch) should treat this decision
  as a constraint when planning resumes.
