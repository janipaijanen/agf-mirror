---
kind: completion
from: claude
to: human
seq: 1
timestamp: 2026-06-11T00:00:00Z
immutable: true
references-to: []
topic: framework-adoption-kickoff
requires-ack: true
artifacts_owned: []
---

# Adoption Kickoff: agents-govern 1.1.50 adopted for AGF-NG

## What was done

Adopted agents-govern 1.1.50 (Layout B) to govern the AGF-NG project. The
following governance structure is now in place:

**Framework location:** `agents-govern/` (Layout B canonical path)

**Governance artifacts created:**
- `agents-govern/learnings/foundry.yaml` — project record (codename: foundry)
- `agents-govern/agents/MANIFEST.yaml` — project-specific commissioning
- `agents-govern/agents/planner-architect.yaml` — adapted for framework dev
- `agents-govern/agents/coder-tester.yaml` — adapted for framework dev
- `agents-govern/agents/governor-reviewer.yaml` — adapted for framework dev
- `agents-govern/GOVERNANCE.md` — project governance rules and gate definitions
- `docs/coordination/README.md` — coordination channel protocol
- `.claude/rules/` — rule templates installed from `agents-govern/rules-template/`
- `.claude/commands/` — skills installed: `/upgrade`, `/learning`, `/health`,
  `/infosec-screen`, `/release`, `/contribution-audit`, `/capability-review`
- `.gitignore` — `tmp/` ignored

**Active agents:** planner-architect (Claude), coder-tester (Claude),
governor-reviewer (Human), contribution-auditor (Human + Claude),
infosec-sentinel (CI)

**Suspended:** build-engineer, fact-checker, capability-developer
**Skipped:** documentation-writer, deployment-engineer, academic-*

## Verification

- `agents-govern/` contains complete 1.1.50 distribution: confirmed
- `learnings/foundry.yaml` passes template structure: to be validated with
  `cd agents-govern && uv run agov-validate-learnings learnings/foundry.yaml`
- `.claude/rules/` and `.claude/commands/` populated: confirmed

## Assessment synthesis

Five AI models (Lumo, Claude Code, Grok, ChatGPT, CoPilot) and two field
agents (Claude Code + Codex, field assessments) produced independent assessments. The synthesized prioritized backlog is at:

`docs/proposals/synthesis.md`

It is organized in five tiers. Summary:

**Tier 1 — Operational substrate** (highest token/quality ROI):
1. Coordination message `kind` schema + validation
2. Active ownership query (`agov coord status`)
3. Async handoff state + query
4. Coordination brief (`agov coord brief`)

**Tier 2 — Governance tightening:**
5. Advisory consultation gate enforcement (Gate 1 checklist)
6. `artifacts_owned` lapse semantics
7. Digest skeleton generator
8. Small-task fast path formalization

**Tier 3 — Framework maintainability:**
9–12: `origin`/`upstream_candidate` fields, skills in manifest, usage
telemetry, token/context budget as plan field

**Tier 4 — Adoption & documentation:**
13–20: Minimal adoption profile, coordination channel recipe, interface
contract template, ceremony budget, runtime enforcement gap checklist,
anonymized examples, adoption health check, outcome-first framing

**Tier 5 — Paradigm evolution (longer horizon):**
21–26: Decisions as primary object, autonomy budgets, capability governance,
governance compression, continuous scoring, exception handling

## Requires ACK

Governor-Reviewer: please confirm:
1. Adoption structure is correct (Layout B, Phase 1 roles)
2. Proposal prioritization in `docs/proposals/synthesis.md` is approved
   for use as the development backlog
3. Any adjustments to MANIFEST or GOVERNANCE.md before first task begins

The first task is ready to plan once you ACK. Suggested starting point:
**Tier 1, item 1** — coordination message `kind` schema, which is the
foundation all other operational substrate items build on.
