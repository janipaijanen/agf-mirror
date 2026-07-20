---
id: agent-improvement-proposal-norm
status: approved
topic: agent-improvement-proposal-norm
gate1_approved_by: "governor-reviewer; Claude-scribed decision in 20260720T160045Z-governor-reviewer-5h8j.md"
gate1_approved_at: "2026-07-20T16:00:45Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: documentation/rule-only norm; no secrets, auth, or security-boundary change
  - role: contribution-auditor
    status: consulted
    reason: a SHOULD-level self-authorization norm for agents needs a scope-creep
      check — does it read as license to widen an active MR under cover of
      "improvement"? see self-review below
  - role: fact-checker
    status: not-applicable
    reason: no measured counts asserted; the plan quotes the requesting
      governor directive verbatim (via the originating handoff) and proposes
      wording, not verifiable facts
---

# Plan: Agent improvement-proposal norm
#
# Timestamp: 2026-07-20T15:40:00Z
# Status: pending-gate1
# References: docs/coordination/20260720T152331Z-coder-tester-prjn.md (requesting handoff)
# Non-goal: this plan does not touch the active MR-2 audience-relabeling
# branch, and does not create a new coordination message kind.

## Problem statement

During the docs audience-boundary implementation, the governor-reviewer
stated an operating norm in session, quoted verbatim in the requesting
handoff (`20260720T152331Z-coder-tester-prjn.md`):

> if anything is odd you and any other agent can propose changes to make
> things more smooth, transparent, logical, token saving, etc (what are in
> our agf scope at least), but also for any other reason. this should be
> marked as a rule-ish?

Right now this is a spoken norm that exists only inside one coordination
handoff. Nothing in `agf/rules/` or the documentation site tells an agent
(or a new adopter reading the docs) that surfacing friction is expected
behavior, or how to route it without derailing the task actually assigned.
Two failure modes this closes:

1. **Under-reporting.** An agent notices something odd (a stale doc, a
   token-wasteful pattern, a confusing gate boundary) but has no named
   permission to say so, so it says nothing and the friction persists.
2. **Over-reaching.** Absent a stated boundary, "I noticed something" could
   slide into "so I fixed it" inside an unrelated MR — silent scope creep
   that the branch-and-worktree sizing test and systematic-code-review
   checklist are specifically designed to catch and would flag.

## Proposal

A new **SHOULD-strength** (non-blocking) rule, `agf/rules/always/
agent-improvement-proposal-rule.md`, plus one short paragraph in
`agf/docs/site/docs/operate/day-to-day.md`. No new coordination message
kind — the existing `note`, `review`, `lesson-proposal`, and Gate 1 `plan`
kinds already cover the routing shapes needed (confirmed against
`agf/schemas/coordination-message.schema.json`, which already declares
both `fast-track-waiver` and `lesson-proposal`).

### Why SHOULD, not MUST

Codex's handoff explicitly recommended SHOULD over MUST, and this plan
agrees: the norm authorizes agents to *speak up*, and authorization to
speak is naturally non-blocking. A MUST would imply an agent that stays
silent about a minor stylistic quibble has violated a rule — that overreaches
what the governor actually asked for ("if anything is odd... can propose").
This also follows the framework's own evidence-graduation pattern (see
`gate-self-test-rule.md`, `destructive-op-preflight-rule.md`): a rule born
from one session's spoken guidance ships **Provisional**, not MUST, until a
second adopter or session corroborates it.

### Draft rule text (exhibit — final wording is Gate 2 implementation work)

```markdown
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
```

### Draft day-to-day.md addition (exhibit)

One short paragraph after the existing "The daily shape" section's
authority-mode table, before "A governed working session, in order:":

> Agents SHOULD say so when something looks off — a stale doc, wasted
> tokens, a confusing gate boundary — even outside the assigned task. Route
> it as a `note`, `review`, `lesson-proposal`, or Gate 1 plan depending on
> impact; never fold it into the active MR. See
> [agent-improvement-proposal-rule](../../../rules/always/agent-improvement-proposal-rule.md)
> for the exact routing table and the boundary against self-authorization.

## Contribution-auditor self-review

Performed in-session (same constraint as the docs-audience-boundary plan:
a same-session self-review is advisory, not a substitute for an
independent Gate 2 reviewer if the governor wants one).

- **Scope-creep risk**: the draft rule's "Boundary" section explicitly
  forecloses the failure mode this check exists to catch (idea ≠
  approval; downstream-impact-rule still applies). No blocking finding.
- **Duplication risk**: checked against `uncertainty-and-scope-rule.md`
  ("if you are uncertain whether a behaviour is intentional...") and
  `agent-proactive-analysis-rule.md` ("Proactively Propose Better
  Options") — both are about *how an agent behaves inside its own task*.
  This norm is about *reporting something outside the task's scope*. No
  overlap that would make one rule redundant.
- **Naming**: `agent-improvement-proposal-rule` was chosen over
  "continuous-improvement" (too generic; reads as a corporate-process
  buzzword the marketing-tone-rule would flag if it ever reached
  external docs) or "friction-reporting" (undersells the "any other
  reason" clause the governor explicitly included).

No blocking findings.

## Downstream-impact preview (for the Gate 2 implementation MR)

Not yet triggered — this plan only proposes the change; no file under a
`downstream-impact-rule.md` trigger path is modified by this plan itself.
When implemented, the new rule file is **ADDITIVE** (new permission, no
existing contract narrowed) and the `day-to-day.md` paragraph is
documentation-only. The implementer should still write the three-circle
impact statement at commit time per the rule's own requirement.

## Recommended agent work split

- **MR-A** (either agent): create `agf/rules/always/
  agent-improvement-proposal-rule.md` from the exhibit above, add the
  `day-to-day.md` paragraph, run `mkdocs build --strict`, file Gate 2.
- No MR-B needed — this is intentionally one small MR, per the requesting
  handoff's "small Gate 1 proposal" framing.

## Risks

- **Provisional-evidence risk**: single-session evidence (one governor
  quote). Mitigated by shipping as SHOULD + Provisional rather than MUST,
  consistent with `gate-self-test-rule.md`'s and
  `destructive-op-preflight-rule.md`'s precedent for norms born from one
  incident or one session.
- **Vagueness risk**: "any other reason" is intentionally broad per the
  governor's own wording; the routing table and the boundary section keep
  it from becoming a blank check.

## Rollback

Revert the rule file and the day-to-day.md paragraph; no schema or
tooling change to unwind.

## Questions for Gate 1

1. Is SHOULD (not MUST) the right strength, given this is one governor's
   spoken guidance in one session?
2. Does `agf/rules/always/` match the intended landing directory, or
   should this live under `agf/rules/workflow/` since it's about routing
   behavior rather than a constant constraint?
3. Should the rule name reference "AGF-scope" explicitly in its scope
   line, given the governor's parenthetical ("what are in our agf scope
   at least, but also for any other reason") — i.e., should project-local
   observations outside AGF's own scope be explicitly in-scope for this
   rule, or left to a project's own local rule set?
