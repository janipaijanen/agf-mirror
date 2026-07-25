---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260725T214402Z-planner-architect-3krp
timestamp: 2026-07-25T21:44:02Z
model: codex-gpt-5
immutable: true
references-to: []
topic: public-feature-summary-pedagogy-marketing-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Designate explicit Pedagogist and Marketing advisory reviewers for the proposed public feature-summary work."
    - "Confirm whether these are temporary advisory perspectives played through existing roles or proposed new governed roles requiring a separate role-status change."
  must_read:
    - agf/agents/MANIFEST.yaml
    - agf/rules/docs/documentation-audience-rule.md
    - agf/rules/docs/marketing-tone-rule.md
    - agf/rules/docs/implementation-honesty-rule.md
  do_not_touch:
    - README.md
    - agf/docs/site/
    - agf/agents/MANIFEST.yaml
---

# Coordinate Pedagogist and Marketing review of a public feature summary

### Codex · planner-architect

The human governor proposed a tight bullet list of AGF features on the root
README and at the start of public-facing pages, explicitly including gates,
the learning loop, and improvement. The governor requires both Pedagogist and
Marketing roles to have their say before this is planned or written.

The current foundry manifest has no roles literally named `pedagogist` or
`marketing`. `documentation-writer` is the nearest pedagogical role but is
currently skipped. `fact-checker` is active for factual review of marketing
and evaluation copy, while `marketing-tone-rule` governs tone and claim
integrity; neither constitutes a named Marketing role. This handoff therefore
does not invent either role or activate a skipped role.

## Proposed consultation mandate

The governor-reviewer should designate named players for two independent,
advisory first passes:

1. **Pedagogist perspective** — assess cognitive load, ordering, terminology,
   novice comprehension, progressive disclosure, and whether repeating the
   list on many pages helps orientation or creates noise. Recommend the
   smallest useful placement pattern.
2. **Marketing perspective** — assess value clarity, audience fit,
   differentiation, credibility, calls to action, and overclaim risk. Every
   feature statement must remain implementation-honest and use AGF maturity
   and evidence vocabulary where needed.

The two first passes should be written independently before either reviewer
sees the other's conclusions. A planner synthesis should then reconcile them,
and the active fact-checker should verify the resulting claims before Gate 1.
These advisory perspectives do not approve Gate 1 or Gate 2.

## Candidate content envelope for review

The proposed compact summary should consider, without presuming final wording:

- human-approved Gate 1 planning and Gate 2 merge decisions;
- durable coordination and exact-revision handoffs;
- structured roles with explicit authority boundaries;
- evidence and maturity labels that limit overclaiming;
- the learning loop from operational evidence to governed change;
- the improvement-proposal loop, clearly distinguished from automatic scope
  expansion or self-modification;
- deterministic validation and auditability;
- adoption, upgrade, and removal paths; and
- optional tooling that supports rather than replaces human authority.

Reviewers must also decide the surface strategy. A single canonical list with
short links from other entry pages reduces drift; repeating a full list on
every public page may weaken page-specific orientation. “First page of public
pages” must be enumerated precisely before implementation, likely separating
the root README, the documentation landing page, and any release/evaluation
landing surfaces.

## Requested decision

Record one of the following before planning begins:

- designate existing governed roles/players to provide the two named advisory
  perspectives for this task; or
- authorize a separate Gate 1 proposal for new temporary or permanent role
  definitions/status changes.

After designation, planner-architect will issue two explicit consultation
handoffs, collect the independent responses, and produce a bounded Gate 1 plan.
