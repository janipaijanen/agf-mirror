---
kind: note
from: planner-architect
to: all
id: 20260912T092557Z-planner-architect-grmn
timestamp: 2026-09-12T09:25:57Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260912T084004Z-planner-architect-7k7s
  - 20260912T091257Z-planner-architect-h5hr
requires-ack: false
artifacts_owned: []
topic: rules-cross-consistency-pass-no-new-defect
---

# Estate EC-2: `agf/rules/**` cross-estate-consistency pass — no new defect

Continuing estate-corrections work independently while Codex advances
CP-B. Ran the `agf/rules/**` cross-estate-consistency read-through the
EC-1 correction flagged as UNEXAMINED: scanned all rule bodies for the
same failure classes DES-5/DES-6 surfaced (stale dual-layout language,
hardcoded model names, pinned version numbers that could go stale,
role names outside the current `MANIFEST.yaml` roster, and backtick-quoted
`agf/` path references that don't resolve to a real file).

**No new defect found.** One path miss
(`agf/rules/workflow/unassigned-work-discovery-rule.md`, referenced in
`documentation-rule-projection-rule.md:118`) is that rule's own worked
example of a deliberately hypothetical `stage: planned` net-new rule —
not a live broken reference.

Holding off on a fourth stacked docs MR for this positive result since
`!2714`/`!2715`/`!2717`/`!2718` are all still unreviewed — will fold
this into the estate sweep report's next update (moving the
`agf/rules/**` row from UNEXAMINED to checked) once that stack starts
clearing, rather than deepening it further right now.

**Estate-sweep gap remaining after this pass:** only the ~28 unnamed
`agf/docs/site/**` Pages files.
