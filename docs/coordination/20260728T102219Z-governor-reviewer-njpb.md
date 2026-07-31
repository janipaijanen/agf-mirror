---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260728T102219Z-governor-reviewer-njpb
timestamp: 2026-07-28T10:22:19Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260727T154845Z-coder-tester-bsg4
topic: cross-repo-scientific-promotional-intake-design
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "Draft a Gate 1 plan for how separate scientific/research and promotional/communications repositories feed findings and content proposals into AGF-NG without becoming alternate governance authorities."
    - "Evaluate repository topology explicitly: one combined external-content repository, separate scientific and promotional repositories, or repository-neutral intake contracts. Recommend a default and state when separation is justified."
    - "Design a transparent multi-axis issue taxonomy that keeps lifecycle status separate from source, finding/content type, destination audience, evidence status, and resolution. Reuse or extend issue #75 rather than conflating these axes into workflow labels."
    - "Specify provenance, deduplication, licensing/privacy, cross-linking, review routing, and promotion criteria for inconsistencies, empirical findings, scientific claims, promotional claims, and proposed Pages content for human or AI adopters."
    - "Define the canonical-authority boundary: external repositories may discover and propose; an AGF issue/coordination record and normal Gate 1/Gate 2 process remain required before AGF artifacts change."
    - "Coordinate with contribution-auditor and academic-peer-reviewer in the plan; include fact-checker routing for factual/promotional claims and infosec status as required by the plan schema."
  must_read:
    - docs/plans/issue75-issue-board-workflow.md
    - agf/rules/always/downstream-impact-rule.md
    - agf/rules/always/agent-improvement-proposal-rule.md
    - agf/agents/academic-peer-reviewer.yaml
    - agf/docs/site/docs/reference/maturity-vocabulary.md
  do_not_touch:
    - GitLab labels or board lists before Gate 1 approval
    - any external repository before topology and authority boundaries are approved
    - agf/VERSION
    - any tag or release
---

# Design governed cross-repository evidence and content intake

The governor is considering separate scientific/research and promotional/
communications repositories. Work in either repository may reveal AGF
inconsistencies or propose new Pages content for human or AI adopters. Design
the AGF-side intake and promotion process before those repositories acquire
workflow authority or ad hoc label systems.

Issue #75's current plan is published in MR !745. Treat it as an in-flight
dependency: coordinate any taxonomy refinements with that plan rather than
silently designing a second lifecycle model.

## Starting principles to test

- External repositories are sources of evidence, critique, drafts, and
  proposals. AGF-NG remains the canonical authority for AGF issues, plans,
  coordination, reviews, Pages content, and releases.
- Lifecycle is one axis only. Do not encode source, evidence strength,
  audience, and work state in one label or board column.
- Scientific claims need academic-method and evidence-status review;
  promotional claims need traceability to current capabilities and factual
  review. Neither lane may promote claims merely because prose is polished.
- A cross-repository finding should preserve an immutable source permalink
  (repository, commit, path/issue, author, observation date), a concise claim
  or proposal, reproduction/evidence, license/privacy disposition, intended
  AGF destination, and deduplication relationship.

## Taxonomy questions the plan must resolve

Keep these dimensions orthogonal and define a minimal label vocabulary:

1. **Lifecycle:** align with issue #75's governed workflow proposal.
2. **Origin:** AGF internal, research repository, promotional repository,
   adopter/external review, or other named source.
3. **Intake type:** inconsistency/defect, evidence/observation, research
   question, content proposal, claim review, or communication request.
4. **Destination/audience:** framework/rule/tooling, human-adopter Pages,
   AI-adopter instructions, academic material, or promotional material.
5. **Evidence status:** Theoretical, Empirically informed, or Empirically
   validated, kept distinct from capability maturity.
6. **Resolution:** accepted, duplicate, transferred, incorporated, rejected
   with rationale, superseded, or retained as radar; resolution must not be
   confused with deployment.

The plan should define which dimensions deserve GitLab labels, which belong
in issue fields/templates, and which are derived rather than manually set.
Avoid an unbounded label explosion.

## Expected deliverables

- Gate 1 plan with acceptance criteria and migration/rollout stages.
- Recommended repository topology and authority diagram.
- Intake issue template or schema proposal with a worked example from each
  scientific and promotional lane.
- Routing matrix from intake type to AGF role/review path.
- Deduplication and bidirectional-link rules between repositories.
- Promotion criteria from external observation/draft to AGF issue, plan,
  Pages change, evidence-status change, and release note.
- Interaction with issue #75's board/lifecycle plan, including whether this
  work amends that plan or becomes a dependent follow-up.

Planning only. Do not create labels, board lists, external repositories,
templates, automation, or Pages changes before Gate 1 approval.
