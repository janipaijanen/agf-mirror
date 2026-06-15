<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Governance — AGF-NG (foundry)

Framework version adopted: **1.1.50** | Layout: **B** | Codename: **foundry**

This file governs the AGF-NG project — the next version of agents-govern.
It replaces the upstream GOVERNANCE.md for this project's purposes.

---

## Role Assignments

| Agent | Played by | Notes |
|-------|-----------|-------|
| **Planner-Architect** | Claude (claude-sonnet-4-6) | Plans proposals, authors ADRs |
| **Coder-Tester** | Claude (claude-sonnet-4-6) | Implements framework artifact changes |
| **Governor-Reviewer** | Human (Jani) | Approves all gates; final authority |
| **Contribution-Auditor** | Human + Claude | Reviews incoming proposals for adversarial patterns |
| **Infosec-Sentinel** | CI (`mix agov.validate.infosec`) | Automated on commit |

Claude plays both Planner-Architect and Coder-Tester in Phase 1.
Split signals are in each agent's YAML `phase_notes`.

---

## Pipeline Mapping

| Pipeline Step | Maps To |
|---------------|---------|
| Requirement | Proposal in `docs/proposals/synthesis.md` or issue |
| Plan | Coordination claim + plan doc in `docs/plans/` |
| Gate 1 | Governor-Reviewer approval in coordination channel |
| Implement | Coder-Tester edits framework artifacts |
| Verify | `mix agov.validate` passes; Infosec-Sentinel clean |
| Gate 2 | Governor-Reviewer reviews in coordination channel |
| Commit | Merge to main |

---

## Gate Rules

### Gate 1 (Plan Approval)

- [ ] Plan cites at least one corroborating learning ID or AI assessment finding
- [ ] Scope is bounded (specific files and sections named)
- [ ] Backward compatibility impact stated (breaking / non-breaking / N/A)
- [ ] If plan touches advisory roles: consultation noted or explicitly waived
- [ ] Governor-Reviewer has approved

**Fast track** (Gate 1 waived): ≤2 files, no new concepts, no
backward-compatibility risk. Waiver recorded in coordination channel.

### Gate 2 (Merge Approval)

- [ ] `mix agov.validate` passes
- [ ] `mix agov.validate.infosec` clean
- [ ] Implements exactly what the approved plan specified
- [ ] Evidence citation present in changed docs (where applicable)
- [ ] CHANGELOG.md updated (for user-visible changes)
- [ ] Governor-Reviewer has approved

---

## Artifact Ownership

| Artifact | Owner | Consumers |
|----------|-------|-----------|
| `docs/proposals/synthesis.md` | Planner-Architect | Governor-Reviewer, Coder-Tester |
| `docs/plans/*.md` | Planner-Architect | Coder-Tester, Governor-Reviewer |
| `agf/docs/adr/*.md` | Planner-Architect | All |
| `agf/framework.md` | Coder-Tester | All |
| `agf/agents/*.yaml` | Coder-Tester | All |
| `agf/rules/**` | Coder-Tester | All |
| `agf/skills/**` | Coder-Tester | All |
| `agf/schemas/**` | Coder-Tester | All |
| `agf/learnings/foundry.yaml` | Coder-Tester (entries) + Governor-Reviewer (approval) | All |
| `agf/CHANGELOG.md` | Coder-Tester | All |
| `docs/coordination/*.md` | Originating agent (immutable) | All |
| `agf/GOVERNANCE.md` | Governor-Reviewer | All |
| `agf/agents/MANIFEST.yaml` | Governor-Reviewer | All |
| `.claude/rules/**` | Coder-Tester | Claude |
| `.claude/commands/**` | Coder-Tester | Claude |

---

## Coordination Channel

Append-only channel at `docs/coordination/`. See `docs/coordination/README.md`.

Key rules:
- Messages are immutable. Corrections are new messages.
- Every message has YAML front matter with `kind`, `from`, `to`, `seq`,
  `artifacts_owned`, and `requires-ack`.
- File ownership lapses when the claiming agent posts a completion with
  empty `artifacts_owned`.
- Digests at `docs/coordination/digests/`; raw files remain authoritative.

---

## Self-Governing Principle

This project uses the framework it is building. Every friction point is
recorded in `foundry.yaml` and becomes evidence for `docs/proposals/synthesis.md`.

Additional Governor-Reviewer responsibility: **evidence arbiter** —
reject plans citing no corroborating evidence. This enforces the framework's
own evidence discipline while building the next version.

---

## Split Signals (Phase 2 triggers)

| Signal | Action |
|--------|--------|
| Plans conflict with Coder-Tester's own designs | Split Planner + Architect |
| Coder-Tester misses bugs in own changes | Split Coder + Tester |
| Review burden slows governance | Split Governor + Reviewer |

---

*Last updated: 2026-06-11 | Framework 1.1.50 | Layout B*
