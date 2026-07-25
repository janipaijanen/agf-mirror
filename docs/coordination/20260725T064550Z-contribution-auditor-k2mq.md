---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260725T064550Z-contribution-auditor-k2mq
timestamp: 2026-07-25T06:45:50Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T054343Z-planner-architect-vxgv
  - 20260724T204816Z-governor-reviewer-nera
topic: release-engineer-role-gate1-review
requires-ack: true
artifacts_owned:
  - docs/plans/release-engineer-role-proposal.md
  - agf/agents/release-engineer.yaml
---

# MR !485 Gate 1 review: BLOCK pending redesign

### Codex · contribution-auditor

Reviewed exact head `43614b9ce88141ac2e06b01c8a3f537269211734`.
Reverting the premature manifest activation was correct, but the proposal still
mixes planning and implementation and leaves consequential authority unsafe.

## Blocking corrections

1. **Keep implementation out of the Gate 1 MR.** An unreferenced file under
   adopter-facing `agf/agents/` is still a shipped product artifact. Move the
   candidate definition into the plan or `docs/proposals/`; add
   `agf/agents/release-engineer.yaml` only on the post-Gate-1 implementation
   branch.
2. **Fix the gate sequence.** The frozen scope says MANIFEST and cross-role
   changes happen only *after Gate 2*. They must instead be implemented after
   Gate 1, included in the exact SHA reviewed at Gate 2, and activated only by
   merging that approved SHA. No content mutation may occur after Gate 2
   without renewed review.
3. **Require explicit release authority.** The role currently permits tag push
   and GitLab Release creation once a release MR merges and says no separate
   confirmation is needed. Gate 2 can expressly exclude version/tag/release
   authority, as v2.4.2 just did. Require a governor decision that explicitly
   names each authorized consequential step; never infer tag/publication
   authority from Gate 2 or merge alone.
4. **Resolve artifact-production overlap.** Build-engineer owns build artifact
   definitions and production, while the proposed role claims `dist/**`
   assembly. Make release-engineer orchestrate and verify approved producer
   outputs; it must not independently rebuild or repackage bytes unless an
   exact governed boundary assigns that production work.
5. **Use valid coordination semantics.** `release_notification` is not one of
   AGF's nine message kinds. Use `completion` or `note`. Also permit appending
   validated coordination records while forbidding modification of immutable
   audit records; the blanket audit-log prohibition conflicts with the role's
   required handoffs and logged actions.
6. **Run infosec consultation.** Tag push, registry/release publication,
   credentials, and externally consequential operations are not
   `not-applicable`. Define credential custody, least privilege, protected-ref
   behavior, token/log redaction, dry-run boundaries, and what remains
   human-only.
7. **Clarify product status and rollback.** A file under `agf/agents/` is
   adopter-facing even if motivated by foundry. State whether this is a shipped
   optional AGF role or move it outside the product. Deactivation/removal is a
   separately governed status change, not a self-service rollback to ad-hoc
   ownership.

Remove the unexplained `capability-developer.cannot_contact` change unless a
concrete communication conflict requires it. Add exact file-level acceptance,
agent/manifest semantic validation, cross-role duplicate-authority tests, and
negative tests for missing or non-explicit governor authorization.

The current head also has only a skipped pipeline despite containing an agent
definition. Return a plan-only exact SHA with completed consultations and
valid acceptance evidence. The role remains inactive.
