---
name: capability-review
origin: upstream
description: >
  Trigger a capability-developer analysis round. Examines accumulated
  evidence (learnings, escalations, agent versions, communication
  topology) and proposes agent evolution: phase splits, new agents,
  maturity bumps, boundary adjustments.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Capability review

Perform a structured capability-developer analysis round. This is the
meta-analysis that drives agent evolution — run it periodically or
when evidence accumulates.

## When to run

- After accumulating 5+ new learnings since last review
- After a release that introduced or modified agents
- When recurring escalation patterns suggest agent gaps
- Quarterly (minimum cadence for active projects)

## Step 1 — Gather evidence

Collect inputs for the analysis:

1. **Learning records** — read all `learnings/*.yaml`, count by category
   (gap, validation, adaptation, tension, overkill)
2. **Agent versions** — run `uv run agov-validate-agents` and note which
   agents are still at v0.1.x vs which have matured
3. **Communication topology** — run `uv run agov-validate-agents --graph
   --output docs/` and review the graph for isolated nodes, asymmetries,
   and missing channels
4. **CHANGELOG.md** — review recent entries for patterns (recurring areas
   of change, repeated fixes to same agents)
5. **Escalation patterns** — if logged, review what gets escalated and
   whether escalation routes are effective

## Step 2 — Analyze patterns

For each evidence source, ask:

### Phase split signals
- Is any combined agent (e.g., coder-tester, planner-architect)
  producing conflicts with itself?
- Is an agent's jurisdiction too broad (owns too many unrelated things)?
- Are forbidden_actions being violated because the agent needs to do
  the forbidden thing to fulfill its purpose?

### New agent candidates
- Are there artifacts with no owner (gaps in jurisdiction coverage)?
- Are there recurring learnings about the same ungovern category?
- Do 3+ independent projects report the same gap?

### Merger signals
- Are two agents' jurisdictions converging (overlap increasing)?
- Is one agent consistently idle (skipped in MANIFEST across projects)?
- Would merging reduce communication overhead without losing governance?

### Maturity assessment
- Which agents have been exercised by multiple projects?
- Which agents have stable definitions (no changes for 2+ releases)?
- Are there agents ready to bump from v0.1.x to v0.2.x?

## Step 3 — Draft proposals

For each finding, produce a structured proposal:

```
## Proposal: <title>

**Type:** phase-split | new-agent | merger | maturity-bump | boundary-adjustment
**Evidence:** <which learnings, projects, or patterns>
**Corroboration:** <how many independent sources>
**Affected agents:** <list>
**Change:** <specific modification>
**Migration:** <how existing adopters adapt>
**Risk:** <what could go wrong>
```

### GATE — Human approves proposals

> **Wait for explicit human approval before any implementation.**
> Proposals are recommendations, not directives. The capability-developer
> agent does not modify agent definitions — it proposes changes that
> the Governor-Reviewer and human approve.

## Step 4 — Implement approved proposals

For each approved proposal:

1. Modify the relevant agent YAML files
2. Update framework.md if phase distribution changed
3. Update ADOPTING.md if adoption steps changed
4. Run `uv run agov-validate-agents --strict` to verify
5. Run `uv run agov-health` for full validation

## Step 5 — Record the review

Document the review outcomes:

- Date of review
- Evidence analyzed (counts)
- Proposals made (accepted/rejected/deferred)
- Agent changes implemented
- Next review trigger (what evidence threshold)

<!-- CUSTOMIZE: adjust evidence thresholds to your project's scale.
Small projects (1-5 agents) may review after every 3 learnings.
Large projects (20+ agents) may review quarterly with higher thresholds. -->
