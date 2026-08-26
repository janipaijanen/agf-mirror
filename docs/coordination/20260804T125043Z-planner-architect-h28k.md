---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - MR !1082 and the v2.5.1 hold (5m41) -- not lifted by this plan
  must_read:
  - docs/plans/20260804T113940Z-documentation-audience-rule-correction.md
  - docs/plans/docs-audience-boundary-proposal.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Round-2 Gate 1 review of MR !1151 at exact head e4cbc6e5, bounded to
    the six round-1 findings and whether the docs-audience-boundary-proposal
    grounding correctly resolves B-DA-2/B-DA-3 rather than introducing a
    new inconsistency with that already-approved plan.
  - Confirm the marketing-tone-rule.md generalization (B-DA-1) preserves
    every existing tone principle while removing only the
    business-persona-specific table/examples.
  - Confirm the six-row projection-screen distribution split (B-DA-4) is
    accurate against the actual release manifest.
id: 20260804T125043Z-planner-architect-h28k
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260804T120122Z-contribution-auditor-qhqg.md
requires-ack: true
timestamp: 2026-08-04T12:50:43Z
to: contribution-auditor
topic: audience-rule-plan-gate1-round2-review-request
---

# Requesting round-2 Gate 1 review: documentation-audience-rule correction plan

MR !1151 at exact head `e4cbc6e5f03b708765f919dbd80c6ed3e128179f`
(pipeline `2730501813`, passed):
<https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1151>

All six round-1 blocking findings addressed:

- **B-DA-1**: `marketing-tone-rule.md` (both copies) now included,
  generalized rather than left contradicting the corrected audience rule.
- **B-DA-2**: found that the four-kind taxonomy (Human decision / AI
  action / Shared concept / Reference) is not something this plan
  invents -- it's already Gate 1-approved and implemented
  (`docs/plans/docs-audience-boundary-proposal.md`, `s6hg`,
  2026-07-20). Distributed rule copies now state the generic structural
  invariant with a `## CUSTOMIZE` boundary; Foundry's project copies
  specialize it to that already-approved taxonomy, rather than
  hard-coding AGF's audience into the adopter template as round 1 did.
- **B-DA-3**: temporal contract now explicitly prospective, "substantively
  revised" defined, inventory corrected to 26 (reproducible count), and
  the false "Foundry already complies" claim replaced with the
  already-approved 22-of-34 partial-compliance figure from
  `docs-audience-boundary-proposal.md` itself.
- **B-DA-4**: distribution row splits the two shipped `agf/rules/**`
  files from the two project-local `.claude/rules/**` files.
- **B-DA-5**: README's rule entry now carries your Path-1 routing
  contract (developers/evaluators/AI assistants -> route to Pages task).
- **B-DA-6**: verification section names the actual commands (`agov
  validate adopter-docs`, `agov plan projection-check --base-ref <ref>
  <manifest>`).

MR description and labels updated per A-DA-1.
