---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 242
timestamp: 2026-06-13T23:10:00Z
immutable: true
references-to:
  - 00236-coder-tester.md
  - 00238-coder-tester.md
  - 00241-planner-architect.md
topic: tier1-provenance-labels-gate2-corrected
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
    - Confirm the three provenance values used are correct for this slice.
    - Merge feat/tier1-provenance-labels via --no-ff to master if approved.
  must_read:
    - docs/coordination/00233-planner-architect.md
    - docs/coordination/00235-planner-architect.md
    - docs/coordination/00238-coder-tester.md
    - docs/coordination/00241-planner-architect.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

Corrected Gate 2 handoff superseding seq 236. Two schema violations in seq 236
fixed: `handoff.status` is now `ready`; extra `branch` and `base_branch` fields
removed from the `handoff` object (additionalProperties: false).

Branch: `feat/tier1-provenance-labels`  
Base branch: `master`  
Branch head as of this handoff: seq 241 (planner-architect positions)

## Product scope

Rules (41 files):
- 39 old-format rules: prepended `---\nprovenance: inherited-v1x-unverified\n---`
- 2 YAML-format rules (`active-branch-preflight-rule`, `answer-only-rule`):
  `provenance: adapted-from-v1x` added to existing frontmatter

Agents (12 files):
- All agent definitions: `provenance: adapted-from-v1x` added after
  `framework_version: "2.0.0"`

Public docs (6 files not updated in docs-polish):
- `agf/docs/site/docs/advanced/customizing-the-framework.md`
- `agf/docs/site/docs/concepts/evidence-and-learnings.md`
- `agf/docs/site/docs/concepts/roles-and-gates.md`
- `agf/docs/site/docs/operate/day-to-day.md`
- `agf/docs/site/docs/operate/review-and-release.md`
- `agf/docs/site/docs/start/first-governed-change.md`

Each received a `!!! warning "Inherited from agents-govern v1.x"` callout after
the Status paragraph.

## On seq 237–241 riding the merge

Seq 237–240 are advisory consultation records filed by the governor on this
branch during Gate 2 review. Seq 241 is the planner-architect response. All
should ride the merge — they are part of the audit trail for this provenance
decision.

## Consultation disposition

- Seq 237 (provenance policy): consulted — PA response in seq 241.
- Seq 238 (Gate 2 review/blocking): consulted — fixes applied in this handoff.
- Seq 239 (gate consultation disposition): consulted — PA position in seq 241.
- Seq 240 (deliberation/bulletin-board): consulted — PA position in seq 241.

## Fast-follow items (not blockers)

- Provenance enum consolidation in rule and agent schemas (fast-track eligible).
- `agf/framework.md` and `agf/schemas/*.json` provenance pass.
- Gate consultation disposition slice (Gate 1 plan required).
- `mix agov.validate-provenance` CI check.
