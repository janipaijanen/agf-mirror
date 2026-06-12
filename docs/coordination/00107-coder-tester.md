---
kind: note
from: coder-tester
to: all
seq: 107
timestamp: 2026-06-12T20:23:22Z
immutable: true
references-to:
  - 00103-governor-reviewer.md
  - 00106-coder-tester.md
topic: agents-govern-2.0-rescope-slice-cadence
requires-ack: false
artifacts_owned:
  - feat/agents-govern-2.0-structure
model: claude-fable-5
---

# Note: v2.0 site-pages slice complete; re-scope to slice cadence per governor

## Slice completed

The 16-page docs/site learning path on `feat/agents-govern-2.0-structure`
is written and committed:

- `a836c68` — content-plan, landing, value, use-cases, docs index, should-you-adopt
- `4935063` — first-governed-change, install-and-layouts, **downstream-impact**,
  **exit-and-decommission** (the two early pages required by seq 103)
- `cffaf49` — roles-and-gates, evidence-and-learnings, day-to-day,
  review-and-release, contracts-and-validators, customizing-the-framework,
  evolution (Hypothesis-labelled)

## Re-scope (human governor directive, 2026-06-12)

Effective immediately for this branch and onward:

1. **One active implementation thread at a time.**
2. **Slice-based delivery** — each slice is a small committed artifact
   plus a coordination note; no all-at-once construction.
3. **Deferred until structure + CI docs pipeline are stable:**
   design system / palette / figure sources, `docs/paper/` academic PDF,
   marketing polish passes.
4. **Throttle is the coordination channel** (`handoff.next`), not agent
   judgment.
5. The current 2.0 branch finishes **structural / docs-pipeline work only**.

## Remaining slices on this branch (in order)

1. `docs/build/` MkDocs config + `.gitlab-ci.yml` (site pipeline only —
   no figures/paper jobs).
2. v2.0 README + CHANGELOG + cross-link repair + validator run.
3. Gate 2 handoff for the structural scope (explicitly including the
   codex session, per seq 88 retrospective).

Each slice gets its own coordination note. Deferred items return as
separate Gate-1-scoped threads after the structural merge.
