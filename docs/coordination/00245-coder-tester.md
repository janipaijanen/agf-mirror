---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 245
timestamp: 2026-06-13T23:16:00Z
immutable: true
references-to:
  - 00236-coder-tester.md
  - 00238-coder-tester.md
  - 00242-coder-tester.md
  - 00243-coder-tester.md
  - 00244-planner-architect.md
topic: tier1-provenance-labels-gate2-final
requires-ack: true
artifacts_owned:
  - agf/rules/**/*.md
  - agf/agents/*.yaml
  - agf/docs/site/docs/advanced/customizing-the-framework.md
  - agf/docs/site/docs/concepts/evidence-and-learnings.md
  - agf/docs/site/docs/concepts/roles-and-gates.md
  - agf/docs/site/docs/operate/day-to-day.md
  - agf/docs/site/docs/operate/review-and-release.md
  - agf/docs/site/docs/start/first-governed-change.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review Tier 1 provenance labels across agf/ rules, agents, and unverified doc pages.
    - Confirm three provenance values used are correct for this slice.
    - Merge feat/tier1-provenance-labels via --no-ff to master if approved.
  must_read:
    - docs/coordination/00233-planner-architect.md
    - docs/coordination/00235-planner-architect.md
    - docs/coordination/00238-coder-tester.md
    - docs/coordination/00244-planner-architect.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

Final Gate 2 handoff for `feat/tier1-provenance-labels`. Supersedes seq 236
(schema violations) and seq 242 (referenced colliding seq 241-PA).

Branch: `feat/tier1-provenance-labels`  
Base branch: `master`

## Collision history

- Seq 236: schema violations (`handoff.status: ready-for-review`; extra
  `branch`/`base_branch` fields in strict `handoff` object). Superseded by seq 242.
- Seq 241-PA collision with seq 241-CT: branch state issue; governor had
  committed `00241-coder-tester.md` before I filed `00241-planner-architect.md`.
  Superseded by seq 244 (PA positions) at this record.
- Seq 242: referenced the colliding `00241-planner-architect.md`. Superseded
  by this record.

## Product scope (unchanged)

Rules (41 files, commit bbba1c2):
- 39 old-format: `---\nprovenance: inherited-v1x-unverified\n---` prepended
- 2 YAML-format: `provenance: adapted-from-v1x` in existing frontmatter

Agents (12 files, commit bbba1c2):
- `provenance: adapted-from-v1x` after `framework_version`

Public docs (6 files, commit bbba1c2):
- `!!! warning "Inherited from agents-govern v1.x"` callout on 6 unverified pages

## Consultation disposition

- Seq 237 (provenance policy): consulted — PA positions in seq 244.
- Seq 238 (Gate 2 review, blocking): consulted — schema fixed in seq 242/245.
- Seq 239 (gate consultation disposition): consulted — PA position in seq 244.
- Seq 240 (deliberation/bulletin-board): consulted — PA position in seq 244.
- Seq 241-CT (deliberation must stay on-record): consulted — acknowledged and
  agreed in seq 244.
- Seq 243 (seq 241 collision review): consulted — fixed by seq 244 + this record.

## Fast-follow items (not blockers)

- Provenance enum consolidation (fast-track eligible).
- `agf/framework.md` and `agf/schemas/*.json` provenance pass.
- Gate consultation disposition slice (Gate 1 plan).
- `mix agov.validate-provenance` CI check.
