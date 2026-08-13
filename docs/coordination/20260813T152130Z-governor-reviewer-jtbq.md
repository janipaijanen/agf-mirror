---
artifacts_owned: []
from: governor-reviewer
id: 20260813T152130Z-governor-reviewer-jtbq
immutable: true
kind: decision-record
model: human
references-to:
- 20260813T152050Z-contribution-auditor-mm4k.md
- 20260813T143900Z-planner-architect-aghp.md
- 20260813T142155Z-coder-tester-4gnn.md
requires-ack: false
timestamp: 2026-08-13T15:21:30Z
to: all
topic: v260-release-publication-gate1-blocked
---

# Decision: Gate 1 for v2.6.0 release-publication plan remains BLOCKED

Direct human decision in chat, transcribed by Claude (planner-architect
session).

Governor's stated decision, verbatim intent:

> Gate 1 remains blocked because the plan still requires decisions on the
> `3456c1fa` prerequisite exception and the currently unauthorized
> non-main release lineage.

## Scope

Applies to MR !1492 (`docs/plans/20260813T051840Z-v260-release-
publication.md`), current exact head `95c06a4f`. Two prior coder-tester
review rounds (`tqqh`, `4gnn`) each found and the plan author fixed a
real construction defect; a third review (`mm4k`, contribution-auditor
self-review capacity per MANIFEST.yaml) independently re-verified the
current cherry-pick construction as technically CLEAR (compiles, the
relevant 20-test `coord_visibility` suite passes on the reconstructed
tree, groups 1–3 confirmed absent). None of that resolves this decision's
two named blockers — they are governance/process questions, not
construction-correctness questions, and remain open:

1. Whether `3456c1fa` (the `aarch64-unknown-linux-musl` CI-target drop)
   qualifies for `8hgm`'s stated "prerequisite strictly necessary to
   deliver an already-approved item" exception.
2. Whether a release lineage constructed independently of `main` (built
   from the `v2.5.0` stable tag forward, never merged back into `main`)
   is an authorized release-construction mechanism, or whether it
   requires a new exception/amendment to
   `release-via-merge-request-rule.md` before it can be used.

## What happens next

No release branch, version bump, tag, or issue closure is authorized.
The plan, its two review rounds, and this decision stay as the record of
where the blocker sits. Gate 1 proceeds once this decision is
superseded by a new record resolving both questions.
