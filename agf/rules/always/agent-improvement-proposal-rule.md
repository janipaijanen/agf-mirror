---
id: agent-improvement-proposal-rule
scope: always
applies_to: [llm-agent, human-agent]
evidence: 20260720T152331Z-coder-tester-prjn
upstream_candidate: false
strength: SHOULD
status: active
see_also: [tool-authority-boundary-rule, downstream-impact-rule, uncertainty-and-scope-rule]
provenance: foundry-native
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Agent Improvement-Proposal Norm
#
# Framework principle: agents surface friction; humans decide priority
# See: framework.md — Principle 5 (Escalation over Improvisation)
# Evidence: governor directive, 20260720T152331Z-coder-tester-prjn.md
# (single-session evidence — Provisional until a second session or
# adopter corroborates)

**Scope:** Every agent doing assigned AGF work — planner-architect,
coder-tester, governor-reviewer, contribution-auditor, fact-checker, or
any project-local role.

## The norm

Agents SHOULD surface oddness, friction, duplication, confusing workflow,
avoidable token spend, unclear authority boundaries, brittle CI or
tooling, or documentation gaps they notice while doing assigned work —
for AGF-scope reasons or any other reason.

Agents SHOULD keep the active task narrow and route the improvement idea
separately instead of quietly expanding the current MR:

| Impact | Route as |
|---|---|
| Trivial, no behavior change (typo, wording) | `note` |
| Worth a second opinion before acting | `review` |
| A recurring pattern worth recording for future sessions | `lesson-proposal` |
| A real rule, schema, or architecture change | Gate 1 `plan` |

A proposal SHOULD name: the observed issue, the likely benefit, the
affected artifact(s), whether the idea is AGF-scope or project-local, and
any cheap evidence (a grep result, a token count, a repro step).

Token-saving ideas SHOULD prefer deterministic tooling, compact summaries,
and existing `agov` surfaces over adding ceremony.

Both AGF-scope and project-local observations are in scope for proposing.
A project-local idea does not become an AGF framework rule without
governor review and the normal evidence path — see "The boundary" below.

## The boundary

Agents MUST NOT treat an improvement idea as approval to change rules,
gates, roles, or merge policy. Proposing is not deciding. Governor-reviewer
approval remains binding — this rule grants permission to speak, not
authority to act (see `tool-authority-boundary-rule.md`: evidence informs
decisions, it does not replace them).

An improvement idea that touches a trigger path in `downstream-impact-
rule.md` still requires that rule's impact classification and, if
BREAKING, a migration note — this rule does not create a shortcut around
that gate.

## Why this exists

An agent with assigned, narrow work has no natural place to put an
observation that falls outside that work's scope. Without a named
permission, the incentive is silence (the friction persists) or silent
scope creep (the observation becomes an unrequested edit). Naming both
the permission and its boundary in one place removes the ambiguity in
both directions.

## Related rules

- `tool-authority-boundary-rule.md` — a proposal is advisory evidence,
  never a blocking authority.
- `downstream-impact-rule.md` — a proposal that would touch a trigger
  path still needs that rule's impact classification.
- `uncertainty-and-scope-rule.md` — the general escalation discipline this
  rule extends to friction/improvement observations specifically.
