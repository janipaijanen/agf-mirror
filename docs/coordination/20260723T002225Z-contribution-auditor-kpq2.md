---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260723T002225Z-contribution-auditor-kpq2
timestamp: 2026-07-23T00:22:25Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T001504Z-coder-tester-rrrk.md
  - 20260723T001812Z-contribution-auditor-27kr.md
  - 20260723T001947Z-contribution-auditor-hhmm.md
topic: cross-assistant-operating-efficiency-gate1-plan
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft a Gate 1 plan for cross-assistant operating-efficiency guidance using MR !436 and the Codex/AGF runbook on MR !438 as inputs.
    - Resolve evidence provenance, product-specific scope, shared principles, documentation placement, and acceptance tests before proposing implementation.
    - Route the draft plan back to contribution-auditor for critique before governor-reviewer submission.
  must_read:
    - docs/coordination/20260723T001812Z-contribution-auditor-27kr.md
    - docs/coordination/20260723T001947Z-contribution-auditor-hhmm.md
    - docs/reference/claude-operating-efficiency.md
    - AGENTS.md
  do_not_touch:
    - docs/reference/claude-operating-efficiency.md
    - AGENTS.md
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1 planning handoff: efficient Claude and Codex operation in AGF

MR !436 contains useful Claude Code retrospective guidance, and MR !438 now
contains a current-official-docs-based Codex counterpart plus an AGF-specific
runbook. Review `27kr` blocks direct merge because the new reference page has
no durable Gate 1 authority and its quantitative usage claims lack governed
evidence or qualification.

Planner-architect should turn these drafts into one governed documentation
change rather than treating either assistant's prose as pre-approved policy.

## Required plan decisions

1. **Audience and placement:** decide whether this becomes one shared
   operating-efficiency page with Claude/Codex subsections or a shared page
   plus a narrowly scoped Claude appendix. Avoid two duplicative long pages.
2. **Authority:** distinguish advisory operator habits, repository-required
   behavior from `AGENTS.md`, and product-specific commands. The page must not
   silently create new blocking rules.
3. **Evidence:** either add privacy-safe provenance for the 91%, 8+ hour, 95%
   above 150k, and 14% MCP observations or qualify them as an operator-observed
   UI snapshot with no independently reproducible causal inference.
4. **Portability:** keep shared principles once—clean task boundaries, bounded
   debugging, batched verification, evidence-aware polling, and procedural
   pre-action controls. Put `/compact`, `/clear`, scheduling, `/plan`, `/fast`,
   `/review`, and subagent controls only in the supported product subsection.
5. **AGF runbook:** preserve `hhmm`'s cold start, role/gate check, in-repo
   worktree, `agov`-first orientation, exact pipeline prediction, exact-SHA
   review, coordination-before-bridge, and action-loop polling sequence.
6. **Acceptance tests:** include link checking, factual-source verification,
   consistency with `AGENTS.md`, non-native-English readability, explicit
   advisory labels, no unsupported wake-up claims, and confirmation that the
   page does not override governed role or gate authority.

## Sequencing

This remains post-release documentation work and must not displace MR !429's
Class 4 governor decision or Workstream D Class 5. Return the proposed plan to
contribution-auditor for challenge before Gate 1 submission. Do not revise MR
!436's product file until the governor approves the plan.
