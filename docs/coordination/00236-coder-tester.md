---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 236
timestamp: 2026-06-13T22:53:00Z
immutable: true
references-to:
  - 00235-planner-architect.md
topic: tier1-provenance-labels-gate2
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
  branch: feat/tier1-provenance-labels
  base_branch: master
  status: ready-for-review
  task:
    - Review provenance labeling across agf/ artifacts.
    - Verify the 3 provenance values used are correct.
    - Approve or redirect the warning callout text for unverified docs pages.
    - If approved, merge via --no-ff to master.
  must_read:
    - docs/coordination/00233-planner-architect.md
    - docs/coordination/00235-planner-architect.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

Tier 1 provenance labels applied to all agf/ rules, agents, and the 6
public docs pages that were not updated in docs-polish.

## What was done

### Rules (41 files, commit bbba1c2)

39 old-format rules (HTML comment SPDX + `# Title` format) received a
prepended minimal YAML frontmatter block:
```
---
provenance: inherited-v1x-unverified
---
```

2 rules with existing YAML frontmatter received `provenance: adapted-from-v1x`:
- `agf/rules/always/answer-only-rule.md` (evidence: foundry-2026-06-10)
- `agf/rules/always/active-branch-preflight-rule.md` (evidence: L007)

### Agents (12 files)

All 12 agent definition YAML files received:
```yaml
  provenance: adapted-from-v1x
```
Added after the existing `framework_version: "2.0.0"` field. All agents carry
`framework_version: "2.0.0"` but content has not been fully verified against
AGF v2 operating model.

### Public docs (6 pages)

Pages not updated in docs-polish received a warning callout after the
`*Status: ...*` paragraph:
```
!!! warning "Inherited from agents-govern v1.x"
    Not yet verified under the AGF v2 layout and tooling. Treat as
    background context only until updated. Adopters may verify and
    report evidence upstream.
```

Pages updated in docs-polish (adopt/, advanced/contracts-and-validators,
index.md, start/should-you-adopt) received no banner.

## Provenance values used

| Value | Count | Meaning |
|---|---|---|
| `inherited-v1x-unverified` | 39 rules + 6 docs pages | Carried from v1.x, not reviewed |
| `adapted-from-v1x` | 2 rules + 12 agents | Touched during AGF v2 development |
| `verified-agf-v2` | 0 | No artifact has had an explicit review pass yet |

## Not yet done (follow-up slices)

- `agf/framework.md` provenance pass
- `agf/schemas/*.json` provenance pass
- `mix agov.validate-provenance` CI check
- Full `verified-agf-v2` review pass for Tier 1 rules/agents
- Workitem manifests rule (Stage 2, Gate 1 plan)
